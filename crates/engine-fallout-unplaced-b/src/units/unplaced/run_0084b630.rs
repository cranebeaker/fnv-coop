//! `(unplaced)/run_0084b630` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0042f5d0` (`cdecl`, `(block, size)`; engine-map unit `extradatalist.cpp`): reallocates a
/// block to that size through the `MemoryManager` (`00aa4150` on `00401020`'s singleton) and
/// answers the new block.
const FREE_SIZED: u32 = 0x0042_f5d0;

// Translated from 0084b630 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, `this` not read): an allocator's
/// reallocation of `block` to `count` pointers, `0042f5d0(block, count * 4)`, whose answer
/// (`EAX`, untouched after the call) is the new block.
pub fn fn_0084b630(e: &mut Engine, _this: Ptr, block: u32, count: u32) -> u32 {
    e.call(FREE_SIZED, &args![block, count << 2]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_b630, fn_0084b630(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reallocates_to_four_bytes_per_entry_and_answers_the_block() {
        let mut e = Engine::new();
        e.register(FREE_SIZED, |_, _| 0x7000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0084_b630, &args![0x5000u32, 0x6000u32, 3u32])
                .u32(),
            0x7000
        );
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FREE_SIZED)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![0x6000, 12]]);
    }
}

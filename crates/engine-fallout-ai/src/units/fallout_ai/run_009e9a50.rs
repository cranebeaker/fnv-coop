//! `fallout/ai/run_009e9a50` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// The float `009e9af0` stores (`-FLT_MAX`, `0xff7fffff`).
const LOWEST_FLOAT: u32 = 0x0101_5f5c;

// Translated from 009e9a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e9a50 - the C++ exception frame (FS:[0]) is not translated
/// No map name: constructs `count` 8-byte elements at `elements`
/// (placement `new`, then [`fn_009e9af0`] on each non-null result). `this`
/// is kept but not read.
pub fn fn_009e9a50(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(
                PLACEMENT_NEW,
                &args![8u32, elements.wrapping_add(index.wrapping_mul(8))],
            )
            .u32();
        if element != 0 {
            fn_009e9af0(e, Ptr::new(element));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009e9af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: an 8-byte element's constructor: the word at `+0` is 0,
/// the float at `+4` the one at `01015f5c` (through the FPU). Returns
/// `this`.
pub fn fn_009e9af0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
    e.mem.set_u32(this.addr() + 4, lowest);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_9a50, fn_009e9a50(Ptr, u32, u32)),
        entry!(0x009e_9af0, fn_009e9af0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_each_element() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1000);
        e.mem.set_u32(LOWEST_FLOAT, 0xff7f_ffff);
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        let elements = e.mem.alloc(24);
        e.mem.write(elements, &[0xaa; 24]);
        e.call(0x009e_9a50, &args![0u32, elements, 2u32]);
        for i in 0..2 {
            assert_eq!(e.mem.u32(elements + 8 * i), 0);
            assert_eq!(e.mem.u32(elements + 8 * i + 4), 0xff7f_ffff);
        }
        assert_eq!(e.mem.u32(elements + 16), 0xaaaa_aaaa);
        // A null placement result constructs nothing.
        e.register(PLACEMENT_NEW, |_, _| 0u32.into_ret());
        e.mem.set_u32(elements, 5);
        e.call(0x009e_9a50, &args![0u32, elements, 1u32]);
        assert_eq!(e.mem.u32(elements), 5);
        assert_eq!(e.call(0x009e_9af0, &args![elements]).u32(), elements);
        assert_eq!(e.mem.u32(elements), 0);
    }
}

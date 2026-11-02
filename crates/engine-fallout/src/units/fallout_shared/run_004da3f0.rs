//! `fallout shared/run_004da3f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004da3f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004da3f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Loads a counted array from the save (`ECX` is the object, no stack words): the count byte
/// at +0x0 through the load buffer (`008579e0` on the object at `011de45c`, one byte), then a
/// block of that many 0x1C-byte entries (`operator new`, a size of `0xFFFFFFFF` when the
/// multiplication overflows; each entry built by `0055a400` through the vector constructor
/// `00401050`, or null when the allocation fails) into +0x4, and the entries' bytes into it.
pub fn fn_004da3f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let buffer = e.global::<u32>(0x011d_e45c);
    e.call(0x0085_79e0, &args![buffer, base, 1u32]);
    let count = u32::from(e.mem.u8(base));
    let size = count.checked_mul(0x1c).unwrap_or(u32::MAX);
    let block = e.call(0x0040_1000, &args![size]).u32();
    let entries = if block == 0 {
        0
    } else {
        e.call(0x0040_1050, &args![block, 0x1cu32, count, 0x0055_a400u32]);
        block
    };
    e.mem.set_u32(base + 0x4, entries);
    let bytes = u32::from(e.mem.u8(base)).wrapping_mul(0x1c);
    let entries = e.mem.u32(base + 0x4);
    let buffer = e.global::<u32>(0x011d_e45c);
    e.call(0x0085_79e0, &args![buffer, entries, bytes]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004d_a3f0, fn_004da3f0(Ptr))]
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_004da3f0_loads_the_count_then_the_entries() {
        let mut e = Engine::new();
        e.map(0x011d_e45c, 4);
        e.set_global::<u32>(0x011d_e45c, 0x5000);
        // The load writes 3 into the count byte.
        e.register(0x0085_79e0, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 3);
            }
            Ret::default()
        });
        e.register(0x0040_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0040_1050, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x004d_a3f0, &args![this]);
        assert_eq!(calls_to(&e, 0x0040_1000), vec![vec![0x54]]);
        assert_eq!(
            calls_to(&e, 0x0040_1050),
            vec![vec![0x7000, 0x1c, 3, 0x0055_a400]]
        );
        assert_eq!(e.mem.u32(this + 4), 0x7000);
        assert_eq!(
            calls_to(&e, 0x0085_79e0),
            vec![vec![0x5000, this, 1], vec![0x5000, 0x7000, 0x54]]
        );
        // A failed allocation leaves a null array, still loaded from.
        e.register(0x0040_1000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004d_a3f0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&e, 0x0040_1050).is_empty());
        assert_eq!(calls_to(&e, 0x0085_79e0)[1], vec![0x5000, 0, 0x54]);
    }
}

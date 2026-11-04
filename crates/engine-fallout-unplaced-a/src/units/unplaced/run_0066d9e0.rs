//! `(unplaced)/run_0066d9e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's capacity (`0062a100`; thiscall: the word at `+8` masked with `0x3FFFFFFF`),
/// the growth helper `00c90940` (cdecl: the first argument, the array, the element size
/// 0x10; in `hkarrayutil.cpp` by the engine map) and the element-range constructor
/// `0066db30` (cdecl: first element, count, value, a tag byte).
const ARRAY_CAPACITY: u32 = 0x0062_a100;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const CONSTRUCT_ELEMENTS: u32 = 0x0066_db30;

// Translated from 0066d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; called from `0066d9b0`): appends one
/// 16-byte element to the Havok-style array at `this` (data `+0`, size `+4`): when the size
/// equals the capacity (`0062a100`) it grows the array first (`00c90940(allocator, this,
/// 0x10)`), then constructs the element at `data + size * 16` from `value`
/// (`0066db30(element, 1, value, 0)`) and increments the size.
pub fn fn_0066d9e0(e: &mut Engine, this: Ptr, allocator: u32, value: u32) {
    let base = this.addr();
    let capacity = e.call(ARRAY_CAPACITY, &args![this]).u32();
    if e.mem.u32(base + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, this, 0x10u32]);
    }
    let element = (e.mem.u32(base + 4) << 4).wrapping_add(e.mem.u32(base));
    e.call(CONSTRUCT_ELEMENTS, &args![element, 1u32, value, 0u32]);
    let size = e.mem.u32(base + 4);
    e.mem.set_u32(base + 4, size.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_d9e0, fn_0066d9e0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn appends_and_grows_only_when_full() {
        let mut e = Engine::new();
        e.register(ARRAY_CAPACITY, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x3fff_ffff).into_ret()
        });
        e.register(ARRAY_RESERVE_MORE, |_, _| Ret::default());
        e.register(CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x4000);
        e.mem.set_u32(array + 4, 2);
        e.mem.set_u32(array + 8, 0x8000_0003);
        e.call_log = Some(vec![]);
        e.call(0x0066_d9e0, &args![array, 0x77u32, 0x99u32]);
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
        assert_eq!(
            calls_to(&e, CONSTRUCT_ELEMENTS),
            vec![vec![0x4020, 1, 0x99, 0]]
        );
        e.call(0x0066_d9e0, &args![array, 0x77u32, 0x98u32]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![0x77, array, 0x10]]
        );
        assert_eq!(e.mem.u32(array + 4), 4);
    }
}

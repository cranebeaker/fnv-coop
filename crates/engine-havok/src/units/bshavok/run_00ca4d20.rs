//! `BSHavok/run_00ca4d20` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTLargeArray<hkNiTriStripsData>::RemoveAll` (Xbox PDB, `00ca4b20`, `this` only).
const ARRAY_REMOVE_ALL: u32 = 0x00ca_4b20;
/// `NiTLargeArray<hkNiTriStripsData>::SetSize` (Xbox PDB, `00ca47f0`, `this` plus the new
/// capacity).
const ARRAY_SET_SIZE: u32 = 0x00ca_47f0;
/// `NiTLargeArray<hkNiTriStripsData>::SetAt` (Xbox PDB, `00ca49a0`, `this` plus the index and
/// the element).
const ARRAY_SET_AT: u32 = 0x00ca_49a0;
/// Copies the vector at the argument into the block it is run on (`004b4db0`, `this` plus one
/// argument).
const COPY_VECTOR_IN: u32 = 0x004b_4db0;

// Translated from 00ca4d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca4d20` (unnamed in the Xbox PDB): takes an output block `out`. Stores the float at
/// `+0x90` (through the x87 stack) at `out + 4`, empties the `NiTLargeArray` at `out + 8`
/// (`RemoveAll`), runs `004b4db0` on `out + 0x20` with the address `this + 0x80`, then walks
/// the 8-byte entries of the array at `+0x98` (the count at `+0xA0`, a signed compare): for
/// each, when the target array's size (`+0xC`) has reached its capacity (`+0x8`, unsigned
/// compare) it grows the array to the step (`+0x14`) plus the size (`SetSize`), then stores
/// the entry's address at the index equal to the size (`SetAt`).
pub fn fn_00ca4d20(e: &mut Engine, this: u32, out: u32) {
    let array = out + 8;
    let length = x87_quiet_bits(e.mem.u32(this + 0x90));
    e.mem.set_u32(out + 4, length);
    e.call(ARRAY_REMOVE_ALL, &args![array]);
    e.call(COPY_VECTOR_IN, &args![out + 0x20, this + 0x80]);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0xa0) {
        let entry = e.mem.u32(this + 0x98).wrapping_add(index.wrapping_mul(8));
        let size = e.mem.u32(array + 0xc);
        if size >= e.mem.u32(array + 8) {
            let step = e.mem.u32(array + 0x14);
            e.call(ARRAY_SET_SIZE, &args![array, step.wrapping_add(size)]);
        }
        e.call(ARRAY_SET_AT, &args![array, size, entry]);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca4d20, fn_00ca4d20(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Doubles for the array routines: `SetSize` raises the capacity by the requested amount
    /// and `SetAt` counts the element in.
    fn setup(count: i32, capacity: u32, step: u32) -> (Engine, u32, u32) {
        let mut e = logging_engine();
        e.register(ARRAY_REMOVE_ALL, nothing);
        e.register(COPY_VECTOR_IN, nothing);
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1] + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x98, 0x5000);
        e.mem.set_i32(this + 0xa0, count);
        let out = e.mem.alloc(0x40);
        e.mem.set_u32(out + 8 + 8, capacity);
        e.mem.set_u32(out + 8 + 0x14, step);
        (e, this, out)
    }

    #[test]
    fn copies_the_float_and_clears_the_array_even_without_entries() {
        let (mut e, this, out) = setup(0, 4, 2);
        e.mem.set_u32(this + 0x90, 0x7f80_0001);
        e.call(0x00ca_4d20, &args![this, out]);
        assert_eq!(e.mem.u32(out + 4), 0x7fc0_0001);
        assert_eq!(calls_to(&e, ARRAY_REMOVE_ALL), [[out + 8]]);
        assert_eq!(calls_to(&e, COPY_VECTOR_IN), [[out + 0x20, this + 0x80]]);
        assert!(calls_to(&e, ARRAY_SET_AT).is_empty());
    }

    #[test]
    fn stores_each_entry_and_grows_the_array_when_full() {
        let (mut e, this, out) = setup(3, 2, 5);
        e.call(0x00ca_4d20, &args![this, out]);
        assert_eq!(
            calls_to(&e, ARRAY_SET_AT),
            [
                [out + 8, 0, 0x5000],
                [out + 8, 1, 0x5008],
                [out + 8, 2, 0x5010]
            ]
        );
        // Full at the third element only: 5 + 2.
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), [[out + 8, 7]]);
    }

    #[test]
    fn a_negative_count_does_nothing() {
        let (mut e, this, out) = setup(-1, 0, 0);
        e.call(0x00ca_4d20, &args![this, out]);
        assert!(calls_to(&e, ARRAY_SET_AT).is_empty());
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
    }
}

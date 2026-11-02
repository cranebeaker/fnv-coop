//! `NiMain/run_00a99800` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two functions of an extra-data class with a counted array of words (the count at +0xc,
//! the array at +0x10), just before `NiIntegersExtraData::IsEqual` in the exe; the class is
//! not confirmed here.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFree` (`00aa10f0`, `cdecl`) and the array allocator (`00aa1070`, `cdecl`, the size).
const NI_FREE: u32 = 0x00aa_10f0;
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;
/// The `NiExtraData` member copy (`00a7b330`, unit `niextradata.cpp`, unnamed in the engine
/// map; `thiscall`, clone and cloning process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const COUNT: u32 = 0xc;
const ARRAY: u32 = 0x10;

// Translated from 00a99800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a count and the address of that many words;
/// `RET 8`): frees the array (+0x10, `NiFree`); with no words or a zero count clears the
/// count and the array, otherwise stores the count and copies the words into a new array
/// (`count * 4` bytes from `00aa1070`).
pub fn fn_00a99800(e: &mut Engine, this: Ptr, count: u32, values: Ptr) {
    let base = this.addr();
    let old = e.mem.u32(base + ARRAY);
    e.call(NI_FREE, &args![old]);
    if values.is_null() || count == 0 {
        e.mem.set_u32(base + COUNT, 0);
        e.mem.set_u32(base + ARRAY, 0);
        return;
    }
    e.mem.set_u32(base + COUNT, count);
    let array = e.call(ALLOCATE_ARRAY, &args![count.wrapping_mul(4)]).u32();
    e.mem.set_u32(base + ARRAY, array);
    for i in 0..count {
        let value = e.mem.u32(values.addr().wrapping_add(4 * i));
        let array = e.mem.u32(base + ARRAY);
        e.mem.set_u32(array.wrapping_add(4 * i), value);
    }
}

// Translated from 00a99870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the clone and the cloning process; `RET 8`): the
/// member copy: the `NiExtraData` one (`00a7b330`), then, when this one has an array and a
/// count, the clone gets the count and a new array (`count * 4` bytes from `00aa1070`) with
/// the words copied; otherwise its count and array are cleared.
pub fn fn_00a99870(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    let base = this.addr();
    let target = clone.addr();
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    if e.mem.u32(base + ARRAY) == 0 || e.mem.u32(base + COUNT) == 0 {
        e.mem.set_u32(target + ARRAY, 0);
        e.mem.set_u32(target + COUNT, 0);
        return;
    }
    let count = e.mem.u32(base + COUNT);
    e.mem.set_u32(target + COUNT, count);
    let size = e.mem.u32(base + COUNT).wrapping_mul(4);
    let array = e.call(ALLOCATE_ARRAY, &args![size]).u32();
    e.mem.set_u32(target + ARRAY, array);
    let mut i = 0u32;
    while i < e.mem.u32(base + COUNT) {
        let value = e.mem.u32(e.mem.u32(base + ARRAY).wrapping_add(4 * i));
        let array = e.mem.u32(target + ARRAY);
        e.mem.set_u32(array.wrapping_add(4 * i), value);
        i += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_9800, fn_00a99800(Ptr, u32, Ptr)),
        entry!(0x00a9_9870, fn_00a99870(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn set_array_copies_or_clears() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + ARRAY, 0x4444);
        let values = e.mem.alloc(8);
        e.mem.set_u32(values, 7);
        e.mem.set_u32(values + 4, 8);
        e.call(0x00a9_9800, &args![this, 2u32, values]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_FREE, vec![0x4444])));
        assert!(log.contains(&(ALLOCATE_ARRAY, vec![8])));
        let array = e.mem.u32(this + ARRAY);
        assert_eq!(
            (
                e.mem.u32(this + COUNT),
                e.mem.u32(array),
                e.mem.u32(array + 4)
            ),
            (2, 7, 8)
        );
        e.call(0x00a9_9800, &args![this, 0u32, values]);
        assert_eq!((e.mem.u32(this + COUNT), e.mem.u32(this + ARRAY)), (0, 0));
        e.mem.set_u32(this + COUNT, 5);
        e.call(0x00a9_9800, &args![this, 3u32, 0u32]);
        assert_eq!((e.mem.u32(this + COUNT), e.mem.u32(this + ARRAY)), (0, 0));
    }

    #[test]
    fn copy_members_duplicates_the_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let clone = e.mem.alloc(0x14);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 5);
        e.mem.set_u32(array + 4, 6);
        e.mem.set_u32(this + COUNT, 2);
        e.mem.set_u32(this + ARRAY, array);
        e.call(0x00a9_9870, &args![this, clone, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(EXTRA_DATA_COPY_MEMBERS, vec![this, clone, 0x77])));
        let copy = e.mem.u32(clone + ARRAY);
        assert_ne!(copy, array);
        assert_eq!(
            (
                e.mem.u32(clone + COUNT),
                e.mem.u32(copy),
                e.mem.u32(copy + 4)
            ),
            (2, 5, 6)
        );
        e.mem.set_u32(this + COUNT, 0);
        e.call(0x00a9_9870, &args![this, clone, 0x77u32]);
        assert_eq!((e.mem.u32(clone + COUNT), e.mem.u32(clone + ARRAY)), (0, 0));
    }
}

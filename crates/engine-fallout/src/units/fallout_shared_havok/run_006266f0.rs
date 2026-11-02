//! `fallout shared/havok/run_006266f0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__thiscall` capacity of the array (`0062a100`, no arguments).
const GET_CAPACITY: u32 = 0x0062_a100;
/// `__cdecl` grow (`00c90940`, `hkarrayutil.cpp`: allocator, array, element size).
const GROW_ARRAY: u32 = 0x00c9_0940;
/// `__cdecl` element construction (`00626800`: address, count, value, flag).
const CONSTRUCT_ELEMENTS: u32 = 0x0062_6800;

// Translated from 006266f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall` push-back on an array of 8-byte elements
/// (data pointer at +0, size at +4): when the size equals the capacity
/// (`0062a100`) the array grows first (`00c90940` with `allocator`, `this`
/// and 8); then one element is constructed at the end from `value`
/// (`00626800`, count 1, flag 0) and the size is incremented.
pub fn fn_006266f0(e: &mut Engine, this: Ptr, allocator: u32, value: u32) {
    let capacity = e.call(GET_CAPACITY, &args![this]).u32();
    if e.mem.u32(this.addr() + 4) == capacity {
        e.call(GROW_ARRAY, &args![allocator, this, 8u32]);
    }
    let data = e.mem.u32(this.addr());
    let size = e.mem.u32(this.addr() + 4);
    e.call(
        CONSTRUCT_ELEMENTS,
        &args![data.wrapping_add(size.wrapping_mul(8)), 1u32, value, 0u32],
    );
    let size = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr() + 4, size.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006266f0, fn_006266f0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(e: &mut Engine, size: u32, capacity: u32) -> u32 {
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(this + 4, size);
        e.register_double(GET_CAPACITY, move |_, _| Ret {
            eax: capacity,
            ..Ret::default()
        });
        // The grow replaces the data pointer.
        e.register(GROW_ARRAY, |e, a| {
            e.mem.set_u32(a[1], 0x2000);
            Ret::default()
        });
        e.register(CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn appends_without_growing_when_there_is_room() {
        let mut e = Engine::new();
        let this = setup(&mut e, 2, 4);
        e.call(0x0062_66f0, &args![Ptr::<()>::new(this), 0x77u32, 0x88u32]);
        let log = e.call_log.clone().unwrap();
        assert!(log.iter().all(|c| c.0 != GROW_ARRAY));
        assert!(log.contains(&(CONSTRUCT_ELEMENTS, vec![0x1010, 1, 0x88, 0])));
        assert_eq!(e.mem.u32(this + 4), 3);
    }

    #[test]
    fn grows_first_when_full() {
        let mut e = Engine::new();
        let this = setup(&mut e, 4, 4);
        e.call(0x0062_66f0, &args![Ptr::<()>::new(this), 0x77u32, 0x88u32]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(GROW_ARRAY, vec![0x77, this, 8])));
        assert!(log.contains(&(CONSTRUCT_ELEMENTS, vec![0x2020, 1, 0x88, 0])));
        assert_eq!(e.mem.u32(this + 4), 5);
    }
}

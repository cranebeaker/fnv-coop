//! `fallout/interface/run_007021b0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, `cdecl`: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00702220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the block the word at `+0` points to (unnamed in the map; `ECX`,
/// no stack words): `operator delete` (`00401030`) on it, the pointer
/// itself left as it is. The change record's buffer teardown (`008456c0`)
/// calls it on the record's `+4`.
pub fn fn_00702220(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![block]);
}

/// `` `eh vector destructor iterator' `` (`00ec5fce`, stdcall: the array, the element
/// size, the count and the destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;

// Translated from 007021b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `` `vector deleting destructor' `` of the 8-byte holders `00702220` destroys
/// (`thiscall`, `RET 4`). With bit 1 of `flags`: destroys the array's elements (their
/// count is the word before it) and frees the block from that count word when bit 0 is
/// set, answering that address; otherwise `00702220` on the one object, freed with bit 0,
/// answering `this`.
pub fn fn_007021b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(base.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![base, 8u32, count, 0x0070_2220u32],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![base.wrapping_sub(4)]);
        }
        return Ptr::new(base.wrapping_sub(4));
    }
    fn_00702220(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00702220, fn_00702220(Ptr)),
        entry!(0x0070_21b0, fn_007021b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deletes_the_block_at_plus_0() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x0b00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0070_2220, &args![Ptr::<()>::new(holder)]);
        let log = e.call_log.unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == OPERATOR_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![0x0b00_0000]);
        assert_eq!(e.mem.u32(holder), 0x0b00_0000);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn vector_deleting_destructor_handles_arrays_and_single_objects() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_21b0, &args![array, 3u32]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter()
                .any(|(a, w)| *a == VECTOR_DESTRUCTOR_ITERATOR
                    && *w == vec![array, 8, 2, 0x0070_2220])
        );
        assert!(log
            .iter()
            .any(|(a, w)| *a == OPERATOR_DELETE && *w == vec![block]));
        e.mem.set_u32(array, 0x0b00_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_21b0, &args![array, 0u32]).u32(), array);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|(a, _)| *a == OPERATOR_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![0x0b00_0000]);
    }
}

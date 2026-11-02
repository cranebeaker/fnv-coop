//! `BSSystem/run_00afcde0` (the engine map's run of unnamed functions from `00afcde0`),
//! subsystem `BSSystem`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's allocation, its own virtual function at vtable slot +4 (`thiscall`, one word:
/// the element count; answers the block).
const SLOT_ALLOCATE: u32 = 4;

// Translated from 00afcde0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The constructor of a growable array of words (no label; the debug text's array, `thiscall`,
/// two words: the capacity and the count, `RET 8`): its data (+4), count (+8) and capacity
/// (+0xc) cleared; the capacity raised to the count when that is larger (unsigned); a nonzero
/// capacity is allocated through the object's virtual function at slot +4 and kept. With a
/// nonzero count the loop over the elements (`data + i * 4` for each index below the count,
/// unsigned) computes each element's address and does nothing with it (the elements' empty
/// constructor), then the count is stored.
pub fn array_construct(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let capacity = if count > capacity { count } else { capacity };
    if capacity != 0 {
        let data = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
        e.mem.set_u32(base + 4, data);
        e.mem.set_u32(base + 0xc, capacity);
    }
    if count != 0 {
        let mut index: u32 = 0;
        while index < count {
            // MOV ECX,[this+4]; LEA EAX,[ECX+EDX*4]: the element's address, not used further.
            let _element = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(4));
            index = index.wrapping_add(1);
        }
        e.mem.set_u32(base + 8, count);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_cde0, array_construct(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const OBJECT: u32 = 0x0800_0000;
    const VTABLE: u32 = 0x0900_0000;
    const ALLOCATE: u32 = 0x0a00_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(OBJECT, 0x10);
        e.put_vtable(VTABLE, &[0, ALLOCATE]);
        e.mem.set_u32(OBJECT, VTABLE);
        e.register(ALLOCATE, |_, a| (0x0b00_0000 + a[1]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn allocations(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ALLOCATE)
            .map(|c| c.1.clone())
            .collect()
    }

    fn fields(e: &Engine) -> (u32, u32, u32) {
        (
            e.mem.u32(OBJECT + 4),
            e.mem.u32(OBJECT + 8),
            e.mem.u32(OBJECT + 0xc),
        )
    }

    #[test]
    fn construct_allocates_the_larger_of_capacity_and_count() {
        let mut e = engine();
        array_construct(&mut e, Ptr::new(OBJECT), 8, 3);
        assert_eq!(fields(&e), (0x0b00_0008, 3, 8));
        array_construct(&mut e, Ptr::new(OBJECT), 2, 5);
        assert_eq!(fields(&e), (0x0b00_0005, 5, 5));
        assert_eq!(allocations(&e), vec![vec![OBJECT, 8], vec![OBJECT, 5]]);
    }

    #[test]
    fn construct_with_nothing_allocates_nothing() {
        let mut e = engine();
        e.mem.write(OBJECT + 4, &[0xaa; 12]);
        array_construct(&mut e, Ptr::new(OBJECT), 0, 0);
        assert_eq!(fields(&e), (0, 0, 0));
        assert!(allocations(&e).is_empty());
        // A capacity without a count: allocated, the count left 0.
        array_construct(&mut e, Ptr::new(OBJECT), 4, 0);
        assert_eq!(fields(&e), (0x0b00_0004, 0, 4));
        e.call(0x00af_cde0, &args![OBJECT, 1u32, 1u32]);
        assert_eq!(fields(&e), (0x0b00_0001, 1, 1));
    }
}

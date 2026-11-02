//! `BSMain/run_00c5bda0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// The counted-pointer list's `RemovePos` (`00c5bfb0`, `thiscall`, `RET 8`: address of the
/// result, address of the position; answers the result's address).
const REMOVE_POSITION: u32 = 0x00c5_bfb0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c5bda0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5bda0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fc29, state variable) is not translated
/// A counted-pointer list's `Remove` (unnamed in the engine map; `thiscall`, `RET 8`:
/// address of the result, address of the counted pointer to remove). Finds the first node
/// holding the object (the compiler set the cursor to 0 and then tested it before loading
/// the head). Found: `RemovePos` (`00c5bfb0`) moves the node's object into a stack local and
/// its answer is used; otherwise the argument itself. The result gets that object with one
/// more reference (`InterlockedIncrement` of its count at +4); then the local, when it was
/// used and holds an object, drops its reference (at zero the object's virtual `+4` runs).
/// Answers the result's address.
pub fn fn_00c5bda0(e: &mut Engine, this: Ptr<NiTPointerList>, result: u32, object: u32) -> u32 {
    let mut node = std::hint::black_box(0u32);
    if node == 0 {
        node = e.get(this, NiTPointerList::m_pkHead);
    }
    let mut found = 0u32;
    while node != 0 {
        let current = node;
        let item = Ptr::<NiTListItem>::new(node);
        node = e.get(item, NiTListItem::m_pkNext);
        if e.mem.u32(object) == e.get(item, NiTListItem::m_element) {
            found = current;
            break;
        }
    }
    e.with_stack(8, |e, locals| {
        let removed = locals.addr();
        let position = removed + 4;
        let source = if found != 0 {
            e.mem.set_u32(position, found);
            e.call(REMOVE_POSITION, &args![this, removed, position])
                .u32()
        } else {
            object
        };
        let value = e.mem.u32(source);
        e.mem.set_u32(result, value);
        let stored = e.mem.u32(result);
        if stored != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![stored.wrapping_add(4)]);
        }
        if found != 0 {
            let held = e.mem.u32(removed);
            if held != 0
                && e.call(INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                    .u32()
                    == 0
            {
                e.vcall(held, 4, &args![]);
            }
        }
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c5_bda0,
        fn_00c5bda0(Ptr<NiTPointerList>, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e
    }

    #[test]
    fn a_held_object_is_removed_and_handed_over() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        let list = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, object);
        e.mem.set_u32(list, node);
        e.register(REMOVE_POSITION, |e, a| {
            // The node's reference moves into the result local.
            let node = e.mem.u32(a[2]);
            let held = e.mem.u32(node + 8);
            e.mem.set_u32(a[1], held);
            a[1].into_ret()
        });
        let argument = e.mem.alloc(4);
        e.mem.set_u32(argument, object);
        let result = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00c5_bda0, &args![list, result, argument]).u32(),
            result
        );
        assert_eq!(e.mem.u32(result), object);
        // One reference added for the result, the local's dropped: still 1.
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn a_missing_object_comes_back_with_a_reference() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        let list = e.mem.alloc(0xc);
        let argument = e.mem.alloc(4);
        e.mem.set_u32(argument, object);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00c5_bda0, &args![list, result, argument]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == REMOVE_POSITION));
        assert_eq!(e.mem.u32(result), object);
        assert_eq!(e.mem.u32(object + 4), 2);
    }
}

//! `BSMain/run_00c5dcb0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a5a310` (`thiscall`, `RET 4`; unit `NiMain/niavobject.cpp`): adds the object held in
/// the word given to the list.
const LIST_ADD: u32 = 0x00a5_a310;

/// Drops one reference of a counted object (count at +4; at zero its vtable slot +4 runs).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00c5dcd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5dcd0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fdee, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`): adds `object` to the list at +0xac
/// (nodes: next at +0, item at +8) unless it is already in it. The search holds a counted
/// reference to the object (`InterlockedIncrement` / `InterlockedDecrement` of its count at
/// +4; at zero its vtable slot +4 runs); when nothing matched, `00a5a310` on the list gets
/// the address of a local holding a counted reference, which is released afterwards (as the
/// local then holds it).
pub fn fn_00c5dcd0(e: &mut Engine, this: Ptr, object: u32) {
    let list = this.addr() + 0xac;
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
    // The compiler set the cursor to 0 and then tested it before loading the head.
    let mut node = std::hint::black_box(0u32);
    if node == 0 {
        node = e.mem.u32(list);
    }
    let mut found = 0u32;
    while node != 0 {
        let current = node;
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if object == item {
            found = current;
            break;
        }
    }
    if object != 0 {
        release(e, object);
    }
    if found != 0 {
        return;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
        e.call(LIST_ADD, &args![list, local]);
        let held = e.mem.u32(local.addr());
        if held != 0 {
            release(e, held);
        }
    });
}

/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiPSysModifier>>::RemoveAll`
/// (`00e74d40`, engine map; `thiscall`), run on the list at +0xAC.
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;

// Translated from 00c5dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the low 16 bits of its word at
/// +0xBA.
pub fn fn_00c5dcb0(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 0xba, value);
}

// Translated from 00c5de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): clears the 16-bit word at +0xB8 and the word at
/// +0xC0, then empties the list at +0xAC (`00e74d40`).
pub fn fn_00c5de30(e: &mut Engine, this: Ptr) {
    e.mem.set_u16(this.addr() + 0xb8, 0);
    e.mem.set_u32(this.addr() + 0xc0, 0);
    e.call(LIST_REMOVE_ALL, &args![this.addr() + 0xac]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_dcb0, fn_00c5dcb0(Ptr, u16)),
        entry!(0x00c5_dcd0, fn_00c5dcd0(Ptr, u32)),
        entry!(0x00c5_de30, fn_00c5de30(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Engine, u32) {
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
        e.register(LIST_ADD, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(object + 4, 1);
        (e, object)
    }

    #[test]
    fn a_new_object_is_added_with_balanced_references() {
        let (mut e, object) = fixture();
        let this = e.mem.alloc(0xb0);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0x9999);
        e.mem.set_u32(this + 0xac, node);
        e.call_log = Some(vec![]);
        e.call(0x00c5_dcd0, &args![this, object]);
        let log = e.call_log.take().unwrap();
        let adds: Vec<_> = log.iter().filter(|c| c.0 == LIST_ADD).collect();
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0].1[0], this + 0xac);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert!(log.iter().all(|c| c.0 != 0x0150_1000));
    }

    #[test]
    fn an_object_already_listed_is_not_added() {
        let (mut e, object) = fixture();
        let this = e.mem.alloc(0xb0);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x9999);
        e.mem.set_u32(second + 8, object);
        e.mem.set_u32(this + 0xac, first);
        e.call_log = Some(vec![]);
        e.call(0x00c5_dcd0, &args![this, object]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != LIST_ADD));
        assert_eq!(e.mem.u32(object + 4), 1);
    }

    #[test]
    fn a_null_object_is_added_when_no_node_holds_null() {
        let (mut e, _) = fixture();
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_dcd0, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1].0, LIST_ADD);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_word_store_and_the_reset() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        e.call(0x00c5_dcb0, &args![this, 0x1_2345u32]);
        assert_eq!(e.mem.u16(this + 0xba), 0x2345);
        assert_eq!(e.mem.u16(this + 0xbc), 0);
        e.mem.set_u16(this + 0xb8, 7);
        e.mem.set_u32(this + 0xc0, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c5_de30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(LIST_REMOVE_ALL, vec![this + 0xac])));
        assert_eq!(e.mem.u16(this + 0xb8), 0);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(e.mem.u16(this + 0xba), 0x2345);
    }
}

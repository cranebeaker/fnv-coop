//! `(unplaced)/run_00a5f860` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` and `InterlockedDecrement` import slots (stdcall: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::m_uiRefCount` (Xbox PDB) +0x4, and the `DeleteThis` slot (+0x4) of its
/// virtual table.
const REF_COUNT: u32 = 0x4;
const DELETE_THIS_SLOT: u32 = 0x4;

// Translated from 00a5f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on a pointer slot, one word; the shape of `NiPointer<T>::operator=`,
/// shared by many callers): nothing when the slot already holds `object`. Otherwise the old
/// object's count goes down (`InterlockedDecrement` on +4) and at zero it deletes itself
/// (virtual +4), the slot takes `object`, and a non-null `object`'s count goes up. Answers
/// the slot.
pub fn fn_00a5f860(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    let old = e.mem.u32(this.addr());
    if old == object.addr() {
        return this;
    }
    if old != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![old + REF_COUNT]).u32();
        if count == 0 {
            e.vcall(old, DELETE_THIS_SLOT, &args![]);
        }
    }
    e.mem.set_u32(this.addr(), object.addr());
    if !object.is_null() {
        e.call(INTERLOCKED_INCREMENT, &args![object.addr() + REF_COUNT]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5f860, fn_00a5f860(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE_DOUBLE: u32 = 0x0072_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(object + REF_COUNT, count);
        object
    }

    #[test]
    fn fn_00a5f860_moves_the_references() {
        let mut e = Engine::new();
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        // The imports, as the platform's stand-ins do them (one thread).
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        let slot = e.mem.alloc(4);
        let a = object(&mut e, 1);
        let b = object(&mut e, 2);
        e.call_log = Some(vec![]);
        // Empty slot to `a`.
        assert_eq!(e.call(0x00a5f860, &args![slot, a]).u32(), slot);
        assert_eq!((e.mem.u32(slot), e.mem.u32(a + 4)), (a, 2));
        // The same object again: nothing.
        e.call(0x00a5f860, &args![slot, a]);
        assert_eq!(e.mem.u32(a + 4), 2);
        // To `b`: `a` keeps one reference.
        e.call(0x00a5f860, &args![slot, b]);
        assert_eq!(
            (e.mem.u32(slot), e.mem.u32(a + 4), e.mem.u32(b + 4)),
            (b, 1, 3)
        );
        assert!(calls_to(&e, DELETE_DOUBLE).is_empty());
        // To null: `b` goes down; then a last reference deletes.
        e.mem.set_u32(b + 4, 1);
        e.call(0x00a5f860, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![b]]);
    }
}

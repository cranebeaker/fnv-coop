//! `(unplaced)/run_006ec1c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ec1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106d698` (RTTI `.?AV?$BSTCommonMessageQueue@VPathingMessage@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006ec1e0`. One word, plain `RET`, returns nothing.
pub fn fn_006ec1c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d698);
    e.call(0x006e_c1e0, &args![this]);
}

// Translated from 006ec1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor that only sets the
/// vtable `0106d6b8`.
pub fn fn_006ec1e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d6b8);
}

/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 006ec200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BSTCommonMessageQueue<PathingMessage>` (slot +0x00 of
/// its vtable `0106d698`; `thiscall`, `RET 4`): the destructor [`fn_006ec1c0`], then
/// `operator delete(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006ec200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec1c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_c1c0, fn_006ec1c0(Ptr)),
        entry!(0x006e_c1e0, fn_006ec1e0(Ptr)),
        entry!(0x006e_c200, fn_006ec200(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006ec1c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006e_c1e0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006e_c1c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006e_c1e0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_d698);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_d698,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_006ec1e0_sets_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.call(0x006e_c1e0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0106_d6b8);
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    #[test]
    fn deleting_destructor_deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(0x006e_c1e0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_c200, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0106_d698);
        e.call(0x006e_c200, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == OPERATOR_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![this]);
    }
}

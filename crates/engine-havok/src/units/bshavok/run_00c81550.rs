//! `BSHavok/run_00c81550` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the function stores (`010c53dc`) and the static object count it decrements
/// (`01267e60`), and the destructor body of `bhkNiCollisionObject` it ends in (`00c6c290`).
const VTABLE: u32 = 0x010c_53dc;
const OBJECT_COUNT: u32 = 0x0126_7e60;
const COLLISION_OBJECT_DESTRUCTOR_BODY: u32 = 0x00c6_c290;

// Translated from 00c81550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c81550` (the engine map has no name; a destructor body of a class derived from
/// `bhkNiCollisionObject`, unconfirmed which): stores the vtable `010c53dc`, decrements the
/// class's object count at `01267e60` and jumps to the `bhkNiCollisionObject` destructor body
/// (`00c6c290`) with `this`.
pub fn fn_00c81550(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(COLLISION_OBJECT_DESTRUCTOR_BODY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c81550, fn_00c81550(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_counts_down_and_runs_the_collision_object_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(OBJECT_COUNT, 4);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.register(COLLISION_OBJECT_DESTRUCTOR_BODY, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call(0x00c81550, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 2);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == COLLISION_OBJECT_DESTRUCTOR_BODY)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, args![this]);
    }
}

//! `fallout shared/havok/run_00624150` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0061f580` (unnamed in the engine map): `thiscall`, plain `RET`; the second base's
/// constructor.
const SECOND_BASE_CONSTRUCT: u32 = 0x0061_f580;
/// `00624230` (unnamed in the engine map): `thiscall`, plain `RET`; the destructor `00624190`
/// runs.
const DESTRUCT: u32 = 0x0062_4230;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete of a deleting
/// destructor.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable `006241c0` stores.
const FIRST_BASE_VTABLE: u32 = 0x0104_de10;
/// The vtable `00624150` stores at `+0`.
const VTABLE: u32 = 0x0104_de00;
/// The vtable `00624150` stores at `+4`.
const SECOND_VTABLE: u32 = 0x0104_dde4;

// Translated from 006241c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: stores the vtable `0104de10`. Answers `this`.
pub fn fn_006241c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FIRST_BASE_VTABLE);
    this
}

// Translated from 00624150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: a constructor with two bases, `006241c0` on `this` and
/// `0061f580` on `this + 4`, then the vtables `0104de00` at `+0` and `0104dde4` at `+4`.
/// Answers `this`.
pub fn fn_00624150(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006241c0(e, this);
    e.call(SECOND_BASE_CONSTRUCT, &args![this.addr().wrapping_add(4)]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(4), SECOND_VTABLE);
    this
}

// Translated from 00624190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. A deleting destructor: `00624230`, then with
/// bit 0 of `flags` the delete `00401030`. Answers `this`.
pub fn fn_00624190(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_4150, fn_00624150(Ptr) -> Ptr),
        entry!(0x0062_4190, fn_00624190(Ptr, u32) -> Ptr),
        entry!(0x0062_41c0, fn_006241c0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00624150_constructs_both_bases_then_stores_the_vtables() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(SECOND_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0062_4150, &args![this]).u32(), this);
        assert_eq!(calls(&e, SECOND_BASE_CONSTRUCT), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 4), SECOND_VTABLE);
        assert_eq!(e.call(0x0062_41c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FIRST_BASE_VTABLE);
    }

    #[test]
    fn at_00624190_deletes_only_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_4190, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0062_4190, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
    }
}

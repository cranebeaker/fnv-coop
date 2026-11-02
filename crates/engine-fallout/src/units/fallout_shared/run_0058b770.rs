//! `fallout shared/run_0058b770` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A constructor/destructor pair of an unnamed class whose base part is
//! built by `0058b3f0` (vtable `010326fc`) and torn down by `0058b450`, with
//! an embedded member at +0x18 (built by `0058bc30`, destroyed through
//! `0058b830` → `0058bc00`). The engine map names none of them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The base part's constructor (`0058b3f0`, `thiscall`).
const BASE_CONSTRUCT: u32 = 0x0058_b3f0;
/// The base part's destructor body (`0058b450`, `thiscall`).
const BASE_DESTRUCT: u32 = 0x0058_b450;
/// The member's constructor (`0058bc30`, `thiscall(a, b)`).
const MEMBER_CONSTRUCT: u32 = 0x0058_bc30;
/// The member's destructor body (`0058bc00`, `thiscall`).
const MEMBER_DESTRUCT: u32 = 0x0058_bc00;
/// Offset of the embedded member.
const MEMBER: u32 = 0x18;

// Translated from 0058b770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058b770 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor: builds the base part (`0058b3f0`), then the member at +0x18
/// (`0058bc30(0, 1)`). Returns `this`.
pub fn fn_0058b770(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.call(MEMBER_CONSTRUCT, &args![this.addr() + MEMBER, 0u32, 1u32]);
    this
}

// Translated from 0058b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058b7d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body: destroys the member at +0x18 (`0058b830`), then the
/// base part (`0058b450`).
pub fn fn_0058b7d0(e: &mut Engine, this: Ptr) {
    fn_0058b830(e, Ptr::new(this.addr() + MEMBER));
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 0058b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `this` to `0058bc00` (the member's destructor body).
pub fn fn_0058b830(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_b770, fn_0058b770(Ptr) -> Ptr),
        entry!(0x0058_b7d0, fn_0058b7d0(Ptr)),
        entry!(0x0058_b830, fn_0058b830(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| {
                [
                    BASE_CONSTRUCT,
                    BASE_DESTRUCT,
                    MEMBER_CONSTRUCT,
                    MEMBER_DESTRUCT,
                ]
                .contains(a)
            })
            .cloned()
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            BASE_CONSTRUCT,
            BASE_DESTRUCT,
            MEMBER_CONSTRUCT,
            MEMBER_DESTRUCT,
        ] {
            e.register(addr, |_, _| 0x0bad_0000u32.into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_builds_base_then_member_and_returns_this() {
        let mut e = engine();
        let back = e.call(0x0058_b770, &args![0x5000u32]).u32();
        assert_eq!(back, 0x5000);
        assert_eq!(
            calls(&e),
            vec![
                (BASE_CONSTRUCT, vec![0x5000]),
                (MEMBER_CONSTRUCT, vec![0x5018, 0, 1]),
            ]
        );
    }

    #[test]
    fn destructor_destroys_member_then_base() {
        let mut e = engine();
        e.call(0x0058_b7d0, &args![0x5000u32]);
        assert_eq!(
            calls(&e),
            vec![
                (MEMBER_DESTRUCT, vec![0x5018]),
                (BASE_DESTRUCT, vec![0x5000])
            ]
        );
    }

    #[test]
    fn member_destructor_forwards_this() {
        let mut e = engine();
        e.call(0x0058_b830, &args![0x6000u32]);
        assert_eq!(calls(&e), vec![(MEMBER_DESTRUCT, vec![0x6000])]);
    }
}

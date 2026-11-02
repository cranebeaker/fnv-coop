//! `fallout/interface/run_00754b40` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `CompanionWheelMenu`'s vtable (`01071d0c`, RTTI
/// `.?AVCompanionWheelMenu@@`), the constructor of the object at +0x7c (`0041a250`),
/// a local's constructor (`0057bd10`, `RET 4`), the assignment to +0x7c (`00418900`,
/// `RET 4`) and the local's destructor (`00483710`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const COMPANION_WHEEL_MENU_VTABLE: u32 = 0x0107_1d0c;
const FIELD_CONSTRUCT_0041A250: u32 = 0x0041_a250;
const LOCAL_CONSTRUCT_0057BD10: u32 = 0x0057_bd10;
const FIELD_ASSIGN_00418900: u32 = 0x0041_8900;
const LOCAL_DESTRUCT_00483710: u32 = 0x0048_3710;

// Translated from 00754b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00754b40 - the C++ exception-unwinding frame (FS:[0] around the base, the field and the local) is not translated.
/// The `CompanionWheelMenu` constructor (unnamed in the engine map; `thiscall`): the
/// `Menu` constructor, the vtable, the object at +0x7c constructed (`0041a250`), the
/// byte at +0x74 cleared and +0x70 set to -1; a 12-byte local made with 0 (`0057bd10`)
/// is assigned to +0x7c (`00418900`) and destroyed. Answers `this`.
pub fn fn_00754b40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, COMPANION_WHEEL_MENU_VTABLE);
    e.call(FIELD_CONSTRUCT_0041A250, &args![base + 0x7c]);
    e.mem.set_u8(base + 0x74, 0);
    e.mem.set_u32(base + 0x70, 0xffff_ffff);
    // The local at `EBP - 0x18`.
    e.with_stack(0xc, |e, local| {
        e.call(LOCAL_CONSTRUCT_0057BD10, &args![local, 0u32]);
        e.call(FIELD_ASSIGN_00418900, &args![base + 0x7c, local]);
        e.call(LOCAL_DESTRUCT_00483710, &args![local]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0075_4b40, fn_00754b40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn companion_wheel_menu_constructor() {
        let mut e = Engine::new();
        for address in [
            MENU_CONSTRUCT,
            FIELD_CONSTRUCT_0041A250,
            LOCAL_CONSTRUCT_0057BD10,
            FIELD_ASSIGN_00418900,
            LOCAL_DESTRUCT_00483710,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x88);
        e.mem.set_u8(this + 0x74, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_4b40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), COMPANION_WHEEL_MENU_VTABLE);
        assert_eq!(e.mem.u8(this + 0x74), 0);
        assert_eq!(e.mem.u32(this + 0x70), 0xffff_ffff);
        let log = e.call_log.unwrap();
        let local = log
            .iter()
            .find(|(a, _)| *a == LOCAL_CONSTRUCT_0057BD10)
            .unwrap()
            .1[0];
        assert!(log.contains(&(FIELD_CONSTRUCT_0041A250, vec![this + 0x7c])));
        assert!(log.contains(&(LOCAL_CONSTRUCT_0057BD10, vec![local, 0])));
        assert!(log.contains(&(FIELD_ASSIGN_00418900, vec![this + 0x7c, local])));
        assert!(log.contains(&(LOCAL_DESTRUCT_00483710, vec![local])));
    }
}

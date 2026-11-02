//! `fallout shared/run_0047b470` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSPickupPutdownSounds@@`).
const VTABLE: u32 = 0x0101_abd8;
/// The `BGSPickupPutdownSounds::InitializeDataComponent` body the constructor calls (Xbox PDB name of `0047b540`).
const INITIALIZE_DATA: u32 = 0x0047_b540;

// Translated from 0047b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSPickupPutdownSounds` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSPickupPutdownSounds@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0047b4f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// `00483710` (thiscall): the base destructor (an empty body).
const BASE_COMPONENT_DESTRUCT: u32 = 0x0048_3710;

// Translated from 0047b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether any of the bits `mask` is set in the
/// 16-bit flags at +0xC.
pub fn fn_0047b4d0(e: &mut Engine, this: Ptr, mask: u16) -> bool {
    e.mem.u16(this.addr().wrapping_add(0xc)) & mask != 0
}

// Translated from 0047b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether flag 0x40 of the 16-bit flags at
/// +0xC is set (`0047b4d0(0x40)`).
pub fn fn_0047b4b0(e: &mut Engine, this: Ptr) -> bool {
    fn_0047b4d0(e, this, 0x40)
}

// Translated from 0047b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: true when flag 0x20 of the 16-bit flags at
/// +0xC is set and flag 0x40 (`0047b4b0`) is not.
pub fn fn_0047b470(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u16(this.addr().wrapping_add(0xc)) & 0x20 != 0 && !fn_0047b4b0(e, this)
}

// Translated from 0047b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSPickupPutdownSounds` destructor (the engine map gives it no
/// name): stores the class vtable (`0101abd8`) and runs the base
/// destructor (`00483710`).
pub fn fn_0047b520(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(BASE_COMPONENT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047b4f0, fn_0047b4f0(Ptr) -> Ptr),
        entry!(0x0047_b4d0, fn_0047b4d0(Ptr, u16) -> bool),
        entry!(0x0047_b4b0, fn_0047b4b0(Ptr) -> bool),
        entry!(0x0047_b470, fn_0047b470(Ptr) -> bool),
        entry!(0x0047_b520, fn_0047b520(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0047b4f0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0047b4f0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0047_b540, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_abd8)],
        });
    }

    use super::*;

    #[test]
    fn fn_0047b4d0_masks_the_flags_at_c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x0001_0005);
        assert!(e.call(0x0047_b4d0, &args![this, 4u32]).bool());
        assert!(!e.call(0x0047_b4d0, &args![this, 0x0002u32]).bool());
        assert_eq!(e.call(0x0047_b4d0, &args![this, 1u32]).u32(), 1);
    }

    #[test]
    fn fn_0047b4b0_tests_flag_40() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x0047_b4b0, &args![this]).bool());
        e.mem.set_u16(this + 0xc, 0x40);
        assert!(e.call(0x0047_b4b0, &args![this]).bool());
    }

    #[test]
    fn fn_0047b470_needs_20_without_40() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for (flags, expected) in [(0u16, false), (0x20, true), (0x60, false), (0x40, false)] {
            e.mem.set_u16(this + 0xc, flags);
            assert_eq!(
                e.call(0x0047_b470, &args![this]).bool(),
                expected,
                "{flags:#x}"
            );
        }
    }

    #[test]
    fn fn_0047b520_restores_the_vtable_then_runs_the_base_destructor() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0047_b520, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BASE_COMPONENT_DESTRUCT, vec![this])
        );
    }
}

//! `fallout shared/run_0048ea40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESWeightForm@@`).
const VTABLE: u32 = 0x0101_d874;
/// The `TESWeightForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `00910b10`).
const INITIALIZE_DATA: u32 = 0x0091_0b10;

// Translated from 0048eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESWeightForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESWeightForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0048eac0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

// Translated from 0048eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101d874` (RTTI `.?AVTESWeightForm@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_0048eaf0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_d874);
    e.call(0x0048_3710, &args![this]);
}

/// `00428110` (`thiscall`, `RET 4`; unit `extradatalist.cpp` by the engine
/// map): the buffer's changed-form data, written to the given out word;
/// answers the object `004280f0` is asked on.
const CHANGE_DATA: u32 = 0x0042_8110;
/// `004280f0` (`thiscall`, `RET 4`): whether the change flags hold the
/// given bits (`AL`).
const HAS_CHANGE_FLAGS: u32 = 0x0042_80f0;
/// The save buffer's write (`00865e50`, `thiscall`: data, size, flag;
/// `bgssavegamebuffer.cpp` by the engine map).
const SAVE_WRITE: u32 = 0x0086_5e50;
/// The load buffer's read (`00864980`, `thiscall`: destination, size;
/// `bgsloadgamebuffer.cpp` by the engine map).
const LOAD_READ: u32 = 0x0086_4980;

/// Whether the buffer's change data has flag 2 (the stack out word the
/// game passes is a fresh block here).
fn weight_changed(e: &mut Engine, buffer: u32) -> bool {
    e.with_stack(4, |e, out| {
        let data = e.call(CHANGE_DATA, &args![buffer, out]).u32();
        e.call(HAS_CHANGE_FLAGS, &args![data, 2u32]).bool()
    })
}

// Translated from 0048ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm`'s save-game step (`thiscall`, `RET 4`, the save buffer;
/// the class of this unit's vtable `0101d874`; the engine map gives it no
/// name): when the change data (`00428110`) has flag 2 (`004280f0`), the
/// four bytes at +4 are written (`00865e50` with 4 and 0).
pub fn fn_0048ea40(e: &mut Engine, this: Ptr, buffer: u32) {
    if weight_changed(e, buffer) {
        e.call(SAVE_WRITE, &args![buffer, this.addr() + 4, 4u32, 0u32]);
    }
}

// Translated from 0048ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm`'s load-game step (`thiscall`, `RET 4`, the load buffer;
/// the engine map gives it no name): when the change data (`00428110`) has
/// flag 2 (`004280f0`), four bytes are read into +4 (`00864980`).
pub fn fn_0048ea80(e: &mut Engine, this: Ptr, buffer: u32) {
    if weight_changed(e, buffer) {
        e.call(LOAD_READ, &args![buffer, this.addr() + 4, 4u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_ea40, fn_0048ea40(Ptr, u32)),
        entry!(0x0048_ea80, fn_0048ea80(Ptr, u32)),
        entry!(0x0048_eaf0, fn_0048eaf0(Ptr)),
        entry!(0x0048eac0, fn_0048eac0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048eac0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048eac0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0091_0b10, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_d874)],
        });
    }

    #[test]
    fn fn_0048eaf0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_eaf0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_d874);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_d874,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn save_and_load_run_only_when_flagged() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 4);
        e.register(CHANGE_DATA, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register(HAS_CHANGE_FLAGS, |e, _| Ret {
            eax: e.mem.u32(0x0f00_0000),
            ..Ret::default()
        });
        e.register(SAVE_WRITE, |_, _| Ret::default());
        e.register(LOAD_READ, |_, _| Ret::default());
        for (addr, callee) in [(0x0048_ea40, SAVE_WRITE), (0x0048_ea80, LOAD_READ)] {
            e.mem.set_u32(0x0f00_0000, 0);
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x4000u32, 0x5000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1].0, CHANGE_DATA);
            assert_eq!(log[1].1[0], 0x5000);
            assert_eq!(log[2], (HAS_CHANGE_FLAGS, vec![0x6000, 2]));
            assert_eq!(log.len(), 3);
            e.mem.set_u32(0x0f00_0000, 1);
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x4000u32, 0x5000u32]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![0x5000, 0x4004, 4];
            if callee == SAVE_WRITE {
                expected.push(0);
            }
            assert_eq!(log[3], (callee, expected));
        }
    }
}

//! `fallout shared/run_004750b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSClipRoundsForm@@`).
const VTABLE: u32 = 0x0101_a644;
/// The `BGSClipRoundsForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `00475100`).
const INITIALIZE_DATA: u32 = 0x0047_5100;

// Translated from 004750b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSClipRoundsForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSClipRoundsForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_004750b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

// Translated from 004750e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101a644` (RTTI `.?AVBGSClipRoundsForm@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_004750e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_a644);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_50e0, fn_004750e0(Ptr)),
        entry!(0x004750b0, fn_004750b0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004750b0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004750b0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0047_5100, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_a644)],
        });
    }

    #[test]
    fn fn_004750e0_stores_the_vtable_then_forwards_this() {
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
        e.call(0x0047_50e0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_a644);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_a644,
            "the vtable was stored before the call"
        );
    }
}

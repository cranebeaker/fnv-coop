//! `fallout shared/run_0048b800` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESRaceForm@@`).
const VTABLE: u32 = 0x0101_d398;
/// The `InitializeDataComponent` body the constructor calls (`008c6060`: a shared empty body; the engine map names it ActorPackage::NullInstanceData because the linker folded it).
const INITIALIZE_DATA: u32 = 0x008c_6060;

// Translated from 0048b800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESRaceForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESRaceForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0048b800(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// `0084e3a0` (`thiscall`, no stack word): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `00485910` (`cdecl`, chunk type and a word): adds a four-byte chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;

// Translated from 0048b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRaceForm`'s save (the class of `0048b800`'s vtable; the engine map
/// gives it no name; no stack word): when the race at +4 is set, its id
/// (`0084e3a0`) as chunk `RNAM` (`00485910`).
pub fn fn_0048b830(e: &mut Engine, this: Ptr) {
    let race = e.mem.u32(this.addr() + 4);
    if race != 0 {
        let id = e.call(FORM_ID, &args![race]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x4d41_4e52u32, id]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_b830, fn_0048b830(Ptr)),
        entry!(0x0048b800, fn_0048b800(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048b800_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048b800,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x008c_6060, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_d398)],
        });
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0048b830_saves_the_race_id_when_set() {
        let mut e = Engine::new();
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0048_b830, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 4, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0048_b830, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (ADD_CHUNK_WORD, vec![0x4d41_4e52, 0x4001]));
    }
}

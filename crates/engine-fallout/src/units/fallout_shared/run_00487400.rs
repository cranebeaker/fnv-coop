//! `fallout shared/run_00487400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00487400 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01017f88 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00487400(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0101_7f88
}

// Translated from 00487410 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0101cbb0 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00487410(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0101_cbb0
}

// Translated from 00487420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESImageSpaceModifiableForm` constructor (no Xbox PDB name; the
/// class from the vtable's RTTI): the `BaseFormComponent` constructor
/// (`00402d30`), the vtable `0101cbd8`, then `008c6060` (the engine map's
/// `ActorPackage::NullInstanceData` is folded code). Returns `this`.
pub fn fn_00487420(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_cbd8);
    e.call(0x008c_6060, &args![this]);
    this
}

/// `0084e3a0` (`thiscall`, no stack word): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `00485910` (`cdecl`, chunk type and a word): adds a four-byte chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;

// Translated from 00487450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableForm`'s save (the class from `00487420`; the
/// engine map gives it no name; no stack word): when the form at +4 is set,
/// its id (`0084e3a0`) as chunk `MNAM` (`00485910`).
pub fn fn_00487450(e: &mut Engine, this: Ptr) {
    let form = e.mem.u32(this.addr() + 4);
    if form != 0 {
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x4d41_4e4du32, id]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_7450, fn_00487450(Ptr)),
        entry!(0x0048_7400, fn_00487400(Ptr) -> u32),
        entry!(0x0048_7410, fn_00487410(Ptr) -> u32),
        entry!(0x00487420, fn_00487420(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    #[test]
    fn constructor_runs_the_base_then_sets_its_vtable() {
        let mut e = Engine::new();
        e.register(0x0040_2d30, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(0x008c_6060, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x0101_cbd8);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_7420, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0101_cbd8);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![0x0048_7420, 0x0040_2d30, 0x008c_6060]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00487400_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0048_7400, &args![0x1000u32]).u32(), 0x0101_7f88);
        assert_eq!(e.call(0x0048_7400, &args![0u32]).u32(), 0x0101_7f88);
    }

    #[test]
    fn fn_00487410_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0048_7410, &args![0x1000u32]).u32(), 0x0101_cbb0);
        assert_eq!(e.call(0x0048_7410, &args![0u32]).u32(), 0x0101_cbb0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00487450_saves_the_form_id_when_set() {
        let mut e = Engine::new();
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0048_7450, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 4, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0048_7450, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FORM_ID, vec![0x4000]));
        assert_eq!(log[2], (ADD_CHUNK_WORD, vec![0x4d41_4e4d, 0x4001]));
    }
}

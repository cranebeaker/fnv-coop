//! `fallout shared/run_00474cb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global that keeps [`fn_00474cb0`] from entering itself again
/// while the virtual call it makes runs.
const NAME_LENGTH_GUARD: u32 = 0x011c_40c8;
/// The `strlen` wrapper (cdecl, `0044a670`).
const STRLEN_WRAPPER: u32 = 0x0044_a670;
/// The `BaseFormComponent` constructor (`00402d30`) and its empty
/// destructor (`00483710`).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
const BASE_COMPONENT_DESTRUCT: u32 = 0x0048_3710;
/// The vtable `0101a4c4`: its RTTI locator names `BGSBipedModelList`
/// (`.?AVBGSBipedModelList@@`).
const BIPED_MODEL_LIST_VTABLE: u32 = 0x0101_a4c4;
/// The function in that vtable's first slot (`008c6060`), which the
/// constructor calls directly. The engine map's name for it,
/// `ActorPackage::NullInstanceData`, belongs to folded code.
const BIPED_MODEL_LIST_FIRST_SLOT: u32 = 0x008c_6060;

// Translated from 00474cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of the name `this` answers through its virtual method at
/// slot `+0x130`, or 0 when that answers null. While the call runs the
/// byte at `011c40c8` is set, and a call made meanwhile answers 0 without
/// calling anything.
pub fn fn_00474cb0(e: &mut Engine, this: Ptr) -> u32 {
    if e.global::<u8>(NAME_LENGTH_GUARD) != 0 {
        return 0;
    }
    e.set_global::<u8>(NAME_LENGTH_GUARD, 1);
    let text = e.vcall(this.addr(), 0x130, &[]).u32();
    e.set_global::<u8>(NAME_LENGTH_GUARD, 0);
    if text == 0 {
        0
    } else {
        e.call(STRLEN_WRAPPER, &args![text]).u32()
    }
}

// Translated from 00474d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BGSBipedModelList` (the class its vtable's RTTI
/// names): the `BaseFormComponent` base, the class vtable `0101a4c4`, then
/// that vtable's first function (`008c6060`) called directly on `this`.
/// Returns `this`.
pub fn fn_00474d10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BIPED_MODEL_LIST_VTABLE);
    e.call(BIPED_MODEL_LIST_FIRST_SLOT, &args![this]);
    this
}

// Translated from 00474d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `BGSBipedModelList`: puts back the class vtable
/// `0101a4c4`, then runs the `BaseFormComponent` destructor (`00483710`).
pub fn fn_00474d40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BIPED_MODEL_LIST_VTABLE);
    e.call(BASE_COMPONENT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00474cb0, fn_00474cb0(Ptr) -> u32),
        entry!(0x00474d10, fn_00474d10(Ptr) -> Ptr),
        entry!(0x00474d40, fn_00474d40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn name_length_measures_the_virtual_name_under_the_guard() {
        let mut e = Engine::new();
        e.map(0x011c_4000, 0x1000);
        let object = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x134);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 0x130, 0x0090_0000);
        // The name call sees the guard set; it answers the text at 0x5000
        // only while the word at object + 4 is set.
        e.register(0x0090_0000, |e, a| {
            assert_eq!(e.global::<u8>(NAME_LENGTH_GUARD), 1);
            eax(e.mem.u32(a[0] + 4))
        });
        e.register(STRLEN_WRAPPER, |_, _| eax(7));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_4cb0, &[object]).u32(), 0);
        assert!(calls_to(&e, STRLEN_WRAPPER).is_empty());
        assert_eq!(e.global::<u8>(NAME_LENGTH_GUARD), 0);
        e.mem.set_u32(object + 4, 0x5000);
        assert_eq!(e.call(0x0047_4cb0, &[object]).u32(), 7);
        assert_eq!(calls_to(&e, STRLEN_WRAPPER), [[0x5000]]);
        // With the guard already set nothing is called.
        e.set_global::<u8>(NAME_LENGTH_GUARD, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_4cb0, &[object]).u32(), 0);
        assert!(calls_to(&e, 0x0090_0000).is_empty());
        assert_eq!(e.global::<u8>(NAME_LENGTH_GUARD), 1);
    }

    #[test]
    fn biped_model_list_constructor_and_destructor() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.register(BASE_COMPONENT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_1d68);
            eax(a[0])
        });
        e.register(BIPED_MODEL_LIST_FIRST_SLOT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), BIPED_MODEL_LIST_VTABLE);
            Ret::default()
        });
        e.register(BASE_COMPONENT_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_4d10, &[object]).u32(), object);
        assert_eq!(calls_to(&e, BASE_COMPONENT_CONSTRUCT), [[object]]);
        assert_eq!(calls_to(&e, BIPED_MODEL_LIST_FIRST_SLOT), [[object]]);
        e.mem.set_u32(object, 0);
        e.call(0x0047_4d40, &[object]);
        assert_eq!(e.mem.u32(object), BIPED_MODEL_LIST_VTABLE);
        assert_eq!(calls_to(&e, BASE_COMPONENT_DESTRUCT), [[object]]);
    }
}

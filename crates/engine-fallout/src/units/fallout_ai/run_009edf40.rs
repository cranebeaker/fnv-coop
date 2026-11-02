//! `fallout/ai/run_009edf40` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `DialoguePackage` (RTTI `.?AVDialoguePackage@@`).
const DIALOGUE_PACKAGE_VTABLE: u32 = 0x0109_2f6c;
/// The object pointer the destructor checks (`011dea3c`).
const OWNER_SINGLETON: u32 = 0x011d_ea3c;
/// The package pointer it clears in that object (+0x208).
const OWNER_PACKAGE: u32 = 0x208;
/// `009ee020` and `005c8a30` (engine map `MiddleHighProcess::GetBSBound`
/// and `SetBSBound`, names possibly folded): a getter and a setter of a
/// pointer of the object at `011dea3c`.
const OWNER_GET_POINTER: u32 = 0x009e_e020;
const OWNER_SET_POINTER: u32 = 0x005c_8a30;
/// The pointer at +0xa4 and its call `00798450(pointer, 1)`
/// (`mapmenu.cpp`), whose result is not used.
const HELD_OBJECT: u32 = 0xa4;
const HELD_OBJECT_RELEASE: u32 = 0x0079_8450;
/// The member at +0x80 and its destructor (`00483710`, no stack words).
const MEMBER_80: u32 = 0x80;
const MEMBER_80_DESTRUCT: u32 = 0x0048_3710;
/// The base class destructor (`006708b0`).
const BASE_DESTRUCT: u32 = 0x0067_08b0;

// Translated from 009edf40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009edf40 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// `DialoguePackage`'s destructor body (by the vtable it stores; unnamed in
/// the engine map): the vtable; when the object at `011dea3c` holds this
/// package at +0x208 that pointer is cleared, and when its pointer getter
/// (`009ee020`) gives this package its setter (`005c8a30`) is given null.
/// A non-null object at +0xa4 gets `00798450(object, 1)`; then the member
/// at +0x80 is destroyed (`00483710`) and the base destructor (`006708b0`)
/// runs.
pub fn fn_009edf40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, DIALOGUE_PACKAGE_VTABLE);
    let owner = e.global::<u32>(OWNER_SINGLETON);
    if base == e.mem.u32(owner.wrapping_add(OWNER_PACKAGE)) {
        let owner = e.global::<u32>(OWNER_SINGLETON);
        e.mem.set_u32(owner.wrapping_add(OWNER_PACKAGE), 0);
    }
    let owner = e.global::<u32>(OWNER_SINGLETON);
    if base == e.call(OWNER_GET_POINTER, &args![owner]).u32() {
        let owner = e.global::<u32>(OWNER_SINGLETON);
        e.call(OWNER_SET_POINTER, &args![owner, 0u32]);
    }
    let held = e.mem.u32(base + HELD_OBJECT);
    if held != 0 {
        e.call(HELD_OBJECT_RELEASE, &args![held, 1u32]);
    }
    e.call(MEMBER_80_DESTRUCT, &args![base + MEMBER_80]);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009e_df40, fn_009edf40(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(e: &mut Engine, getter_gives: u32) -> u32 {
        let owner = e.mem.alloc(0x210);
        e.map(OWNER_SINGLETON, 4);
        e.mem.set_u32(OWNER_SINGLETON, owner);
        e.register_double(OWNER_GET_POINTER, move |_, _| Ret {
            eax: getter_gives,
            ..Ret::default()
        });
        for callee in [
            OWNER_SET_POINTER,
            HELD_OBJECT_RELEASE,
            MEMBER_80_DESTRUCT,
            BASE_DESTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        owner
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a != 0x009e_df40)
            .cloned()
            .collect()
    }

    #[test]
    fn destructor_detaches_from_the_owner_and_releases_its_parts() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa8);
        let owner = setup(&mut e, this);
        e.mem.set_u32(owner + OWNER_PACKAGE, this);
        e.mem.set_u32(this + HELD_OBJECT, 0x0555_0000);
        e.call_log = Some(vec![]);
        e.call(0x009e_df40, &args![this]);
        assert_eq!(e.mem.u32(this), DIALOGUE_PACKAGE_VTABLE);
        assert_eq!(e.mem.u32(owner + OWNER_PACKAGE), 0);
        assert_eq!(
            calls(&e),
            vec![
                (OWNER_GET_POINTER, vec![owner]),
                (OWNER_SET_POINTER, vec![owner, 0]),
                (HELD_OBJECT_RELEASE, vec![0x0555_0000, 1]),
                (MEMBER_80_DESTRUCT, vec![this + MEMBER_80]),
                (BASE_DESTRUCT, vec![this]),
            ]
        );
    }

    #[test]
    fn destructor_leaves_another_packages_owner_alone() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa8);
        let owner = setup(&mut e, 0x0999_0000);
        e.mem.set_u32(owner + OWNER_PACKAGE, 0x0999_0000);
        e.call_log = Some(vec![]);
        e.call(0x009e_df40, &args![this]);
        assert_eq!(e.mem.u32(owner + OWNER_PACKAGE), 0x0999_0000);
        assert_eq!(
            calls(&e),
            vec![
                (OWNER_GET_POINTER, vec![owner]),
                (MEMBER_80_DESTRUCT, vec![this + MEMBER_80]),
                (BASE_DESTRUCT, vec![this]),
            ]
        );
    }
}

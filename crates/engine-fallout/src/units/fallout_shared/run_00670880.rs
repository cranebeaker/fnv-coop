//! `fallout shared/run_00670880` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESPackage`'s vtable (`0106847c`; RTTI `.?AVTESPackage@@`).
const VTABLE_TES_PACKAGE: u32 = 0x0106_847c;
/// The object `004226e0` asks (a pointer global) and the one `0042ce10`
/// asks.
const GLOBAL_011C3F2C: u32 = 0x011c_3f2c;
const GLOBAL_011DDF38: u32 = 0x011d_df38;
/// `004226e0`, `0042ce10`: `bool` checks on those objects (`thiscall`).
const CHECK_004226E0: u32 = 0x0042_26e0;
const CHECK_0042CE10: u32 = 0x0042_ce10;
/// `0041ca90`: the package's type (`thiscall`).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `00970810`: `thiscall` on the object at `011e0e80`, the package.
const RELEASE_00970810: u32 = 0x0097_0810;
const OBJECT_011E0E80: u32 = 0x011e_0e80;
/// `TESPackage::SetIsCreated` (engine map), `thiscall`, one word.
const SET_IS_CREATED: u32 = 0x0067_4d70;
/// `00670a50`: `thiscall` on the package.
const PACKAGE_00670A50: u32 = 0x0067_0a50;
/// `0067da80`: a member destructor (`thiscall`), run on `+0x6c`, `+0x5c`,
/// `+0x4c`.
const MEMBER_DESTRUCTOR_0067DA80: u32 = 0x0067_da80;
/// `006808b0`: the destructor of the member at `+0x40`.
const MEMBER_DESTRUCTOR_006808B0: u32 = 0x0068_08b0;
/// `00483710`: the destructor of the member at `+0x38`.
const MEMBER_DESTRUCTOR_00483710: u32 = 0x0048_3710;
/// `TESForm::~TESForm` (engine map).
const TES_FORM_DESTRUCTOR: u32 = 0x0048_3630;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00670880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage`'s scalar deleting destructor: [`fn_006708b0`], then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00670880(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006708b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006708b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006708b0 - the C++ exception frame (FS:[0]) is not translated
/// The `TESPackage` destructor (by the vtable it sets): unless `004226e0`
/// of the object at `011c3f2c` holds, with the word at `+0x7c` raised by
/// one meanwhile, hands the package to `00970810` (on `011e0e80`) when
/// `0042ce10` of the object at `011ddf38` holds or its type is `0x15` or
/// `0x17`; then `TESPackage::SetIsCreated(0)`, `00670a50`, the member
/// destructors (`+0x6c`, `+0x5c`, `+0x4c`, `+0x40`, `+0x38`) and
/// `~TESForm`.
pub fn fn_006708b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_TES_PACKAGE);
    let first = e.global::<u32>(GLOBAL_011C3F2C);
    if !e.call(CHECK_004226E0, &args![first]).bool() {
        let count = e.mem.u32(base + 0x7c);
        e.mem.set_u32(base + 0x7c, count.wrapping_add(1));
        let second = e.global::<u32>(GLOBAL_011DDF38);
        if e.call(CHECK_0042CE10, &args![second]).bool()
            || e.call(PACKAGE_TYPE, &args![base]).u32() == 0x15
            || e.call(PACKAGE_TYPE, &args![base]).u32() == 0x17
        {
            e.call(RELEASE_00970810, &args![OBJECT_011E0E80, base]);
        }
        let count = e.mem.u32(base + 0x7c);
        e.mem.set_u32(base + 0x7c, count.wrapping_sub(1));
    }
    e.call(SET_IS_CREATED, &args![base, 0u32]);
    e.call(PACKAGE_00670A50, &args![base]);
    e.call(MEMBER_DESTRUCTOR_0067DA80, &args![base + 0x6c]);
    e.call(MEMBER_DESTRUCTOR_0067DA80, &args![base + 0x5c]);
    e.call(MEMBER_DESTRUCTOR_0067DA80, &args![base + 0x4c]);
    e.call(MEMBER_DESTRUCTOR_006808B0, &args![base + 0x40]);
    e.call(MEMBER_DESTRUCTOR_00483710, &args![base + 0x38]);
    e.call(TES_FORM_DESTRUCTOR, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_0880, fn_00670880(Ptr, u32) -> Ptr),
        entry!(0x0067_08b0, fn_006708b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_engine(e: &mut Engine, busy: bool, special: bool, package_type: u32) -> u32 {
        e.map(GLOBAL_011C3F2C, 4);
        e.set_global(GLOBAL_011C3F2C, 0x1111u32);
        e.map(GLOBAL_011DDF38, 4);
        e.set_global(GLOBAL_011DDF38, 0x2222u32);
        e.register_double(CHECK_004226E0, move |_, _| u32::from(busy).into_ret());
        e.register_double(CHECK_0042CE10, move |_, _| u32::from(special).into_ret());
        e.register_double(PACKAGE_TYPE, move |_, _| package_type.into_ret());
        e.register(RELEASE_00970810, |e, a| {
            // The count is raised while the package is handed over.
            assert_eq!(e.mem.u32(a[1] + 0x7c), 6);
            Ret::default()
        });
        for addr in [
            SET_IS_CREATED,
            PACKAGE_00670A50,
            MEMBER_DESTRUCTOR_0067DA80,
            MEMBER_DESTRUCTOR_006808B0,
            MEMBER_DESTRUCTOR_00483710,
            TES_FORM_DESTRUCTOR,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x7c, 5);
        this
    }

    #[test]
    fn package_destructor_hands_over_special_packages() {
        for (busy, special, package_type, expected) in [
            (false, true, 0, 1),
            (false, false, 0x15, 1),
            (false, false, 0x17, 1),
            (false, false, 0x16, 0),
            (true, true, 0x15, 0),
        ] {
            let mut e = Engine::new();
            let this = package_engine(&mut e, busy, special, package_type);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0067_0880, &args![this, 1u32]).u32(), this);
            let log = e.call_log.take().unwrap();
            let released = log.iter().filter(|c| c.0 == RELEASE_00970810).count();
            assert_eq!(released, expected);
            assert_eq!(e.mem.u32(this + 0x7c), 5);
            assert_eq!(e.mem.u32(this), VTABLE_TES_PACKAGE);
            let tail: Vec<_> = log.iter().rev().take(9).rev().cloned().collect();
            assert_eq!(
                tail,
                vec![
                    (SET_IS_CREATED, vec![this, 0]),
                    (PACKAGE_00670A50, vec![this]),
                    (MEMBER_DESTRUCTOR_0067DA80, vec![this + 0x6c]),
                    (MEMBER_DESTRUCTOR_0067DA80, vec![this + 0x5c]),
                    (MEMBER_DESTRUCTOR_0067DA80, vec![this + 0x4c]),
                    (MEMBER_DESTRUCTOR_006808B0, vec![this + 0x40]),
                    (MEMBER_DESTRUCTOR_00483710, vec![this + 0x38]),
                    (TES_FORM_DESTRUCTOR, vec![this]),
                    (OPERATOR_DELETE, vec![this]),
                ]
            );
        }
    }
}

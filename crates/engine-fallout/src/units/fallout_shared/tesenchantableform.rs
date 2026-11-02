//! `fallout shared/tesenchantableform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00483210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::InitItem` (Xbox PDB): turns the enchantment form id stored at `+0x4` of
/// `this` into the form. `owner` is the form that owns the enchantable part (it may be null).
///
/// The id gets the owner file's compile index (`TESForm::GetFile` `00484e60` with -1, then
/// `TESForm::AddCompileIndex` `00485d50`), is looked up (`004839c0`) and cast with
/// `__RTDynamicCast` from the type at `0x01183028` to the type at `0x01183594`. When nothing is
/// found, one of three `MASTERFILE: Unable to find enchanting ...` messages goes to `005b5e40`
/// (no owner; owner for which `00474cb0` answers non-zero, which prints the owner's name through
/// virtual slot `+0x130`; any other owner, which prints `0084e3a0`'s answer). When an enchantment
/// was found for an owner but `004094f0` (on the enchantment, with the owner) answers false, the
/// `has invalid effect trigger type or effect` message is printed and the field is cleared.
pub fn tesenchantableform_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        return;
    }
    let slot = e.mem.alloc(4);
    let id_in = e.mem.u32(base + 4);
    e.mem.set_u32(slot, id_in);
    if owner.addr() != 0 {
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
    }
    let id = e.mem.u32(slot);
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let found = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_3594u32, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 4, found);
    if found == 0 {
        if owner.addr() == 0 {
            e.call(0x005b_5e40, &args![0x0101_c1c4u32, id]);
        } else if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0101_c250u32, id, name]);
        } else {
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            e.call(0x005b_5e40, &args![0x0101_c208u32, id, owner_id]);
        }
    }
    let enchantment = e.mem.u32(base + 4);
    if enchantment != 0
        && owner.addr() != 0
        && !e.call(0x0040_94f0, &args![enchantment, owner]).bool()
    {
        let owner_name = e.vcall(owner.addr(), 0x130, &args![]).u32();
        let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
        let enchantment = e.mem.u32(base + 4);
        let enchantment_name = e.vcall(enchantment, 0x130, &args![]).u32();
        let enchantment = e.mem.u32(base + 4);
        let enchantment_id = e.call(0x0084_e3a0, &args![enchantment]).u32();
        e.call(
            0x005b_5e40,
            &args![
                0x0101_af00u32,
                enchantment_id,
                enchantment_name,
                owner_id,
                owner_name
            ],
        );
        e.mem.set_u32(base + 4, 0);
    }
    e.mem.free(slot);
}

// Translated from 004830d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the word at +4 (the enchantment) null, the 16-bit word at
/// +8 0 and the word at +0xC 4.
pub fn tes_enchantable_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 4);
}

// Translated from 00483170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the 16-bit word at +8 (no Xbox PDB name; `thiscall`, `RET 4`).
pub fn fn_00483170(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 8, value);
}

/// `TESForm::AddChunk` with a word (`00485910`, `cdecl (chunk, value)`)
/// and with a 16-bit word (`TESForm::AddChunk_ov2`, `00485950`).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_SHORT: u32 = 0x0048_5950;
/// The getter of the form id at +0xC (`0084e3a0`, `thiscall`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `__RTDynamicCast` (`00ec43fb`) from `BaseFormComponent` (`01183040`) to
/// `TESEnchantableForm` (`011839b4`), the type descriptors' names read at +8.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_ENCHANTABLE_FORM: u32 = 0x0118_39b4;
/// The getters `CopyComponent` and `CompareComponent` call: the word at +4
/// (`00726070`) and the 16-bit word (`004a8ae0`, answers `AX`); the setter of
/// the word at +4 (`006ecd40`, one word) and `0041fd00` (one word, given the
/// other component's virtual slot `+0x10`).
const GET_ENCHANTMENT: u32 = 0x0072_6070;
const GET_AMOUNT: u32 = 0x004a_8ae0;
const SET_ENCHANTMENT: u32 = 0x006e_cd40;
const SET_SLOT_10_VALUE: u32 = 0x0041_fd00;

// Translated from 0047bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::Save` (Xbox PDB): with an enchantment (+4), adds
/// `EITM` holding its form id (`0084e3a0`); with a non-zero 16-bit word at
/// +8, adds `EAMT` holding it.
pub fn tes_enchantable_form_save(e: &mut Engine, this: Ptr) {
    let enchantment = e.mem.u32(this.addr() + 4);
    if enchantment != 0 {
        let id = e.call(FORM_ID, &args![enchantment]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x4d54_4945u32, id]);
    }
    let amount = e.mem.u16(this.addr() + 8);
    if amount != 0 {
        e.call(ADD_CHUNK_SHORT, &args![0x544d_4145u32, u32::from(amount)]);
    }
}

// Translated from 00483100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::CopyComponent` (Xbox PDB): when `other` casts to
/// `TESEnchantableForm`, takes its word at +4 (`00726070` into `006ecd40`),
/// its 16-bit word (`004a8ae0` into `fn_00483170`) and hands its virtual
/// slot `+0x10`'s answer to `0041fd00`.
pub fn tes_enchantable_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ENCHANTABLE_FORM,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let enchantment = e.call(GET_ENCHANTMENT, &args![source]).u32();
    e.call(SET_ENCHANTMENT, &args![this, enchantment]);
    let amount = e.call(GET_AMOUNT, &args![source]).u16();
    fn_00483170(e, this, amount);
    let value = e.vcall(source, 0x10, &args![]).u32();
    e.call(SET_SLOT_10_VALUE, &args![this, value]);
}

// Translated from 00483190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::CompareComponent` (Xbox PDB): true (different) when
/// `other` is no `TESEnchantableForm`, when the words at +4 (`00726070`)
/// differ or when the 16-bit words (`004a8ae0`) differ.
pub fn tes_enchantable_form_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ENCHANTABLE_FORM,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    let own = e.call(GET_ENCHANTMENT, &args![this]).u32();
    if own != e.call(GET_ENCHANTMENT, &args![source]).u32() {
        return true;
    }
    let own = e.call(GET_AMOUNT, &args![this]).u16();
    own != e.call(GET_AMOUNT, &args![source]).u16()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_3210, tesenchantableform_init_item(Ptr, Ptr)),
        entry!(
            0x004830d0,
            tes_enchantable_form_initialize_data_component(Ptr)
        ),
        entry!(0x00483170, fn_00483170(Ptr, u16)),
        entry!(0x0047_bd70, tes_enchantable_form_save(Ptr)),
        entry!(0x0048_3100, tes_enchantable_form_copy_component(Ptr, Ptr)),
        entry!(0x0048_3190, tes_enchantable_form_compare_component(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    #[test]
    fn start_values_and_the_16_bit_store() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xff; 0x10]);
        e.call(0x0048_30d0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0xffff);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        e.call(0x0048_3170, &args![this, 0x1_2345u32]);
        assert_eq!(e.mem.u16(this + 8), 0x2345);
        assert_eq!(e.mem.u16(this + 0xa), 0xffff);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{calls_to, run, stub};

    const OWNER: u32 = 0x0800_0000;
    const ENCHANTMENT: u32 = 0x0800_1000;
    const FILE: u32 = 0x0a11_e000;
    const OWNER_NAME: u32 = 0xaaaa_0001;
    const ENCHANTMENT_NAME: u32 = 0xbbbb_0002;

    /// An engine where the lookup (`004839c0`) answers `ENCHANTMENT` when `found`, the owner's
    /// `00474cb0` answers `named`, and `004094f0` answers `valid`. Returns the engine and `this`.
    fn setup(id: u32, found: bool, named: u32, valid: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, id);
        // Both forms have a virtual slot `+0x130` that answers their name.
        for (object, vtable, target, name) in [
            (OWNER, 0x0900_0000, 0x0090_0300, OWNER_NAME),
            (ENCHANTMENT, 0x0900_1000, 0x0090_0400, ENCHANTMENT_NAME),
        ] {
            e.map(object, 0x40);
            e.map(vtable, 0x200);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(vtable + 0x130, target);
            e.register_double(target, move |_, _| Ret {
                eax: name,
                ..Ret::default()
            });
        }
        stub(&mut e, &[0x005b_5e40]);
        e.register(0x0048_4e60, |_, _| Ret {
            eax: FILE,
            ..Ret::default()
        });
        // The compile index goes into the top byte of the id.
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] ^ 0x1000_0000,
            ..Ret::default()
        });
        e.register_double(0x00ec_43fb, move |_, _| Ret {
            eax: if found { ENCHANTMENT } else { 0 },
            ..Ret::default()
        });
        e.register_double(0x0047_4cb0, move |_, _| Ret {
            eax: named,
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 0x77,
            ..Ret::default()
        });
        e.register_double(0x0040_94f0, move |_, _| Ret {
            eax: u32::from(valid),
            ..Ret::default()
        });
        (e, this)
    }

    #[test]
    fn nothing_happens_without_an_enchantment_id() {
        let (mut e, this) = setup(0, true, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert!(calls.is_empty());
    }

    #[test]
    fn a_found_enchantment_is_stored_after_the_compile_index() {
        let (mut e, this) = setup(0x0000_1234, true, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(e.mem.u32(this + 4), ENCHANTMENT);
        assert_eq!(calls_to(&calls, 0x0048_4e60), [vec![OWNER, 0xffff_ffff]]);
        assert_eq!(calls_to(&calls, 0x0048_39c0), [vec![0x0100_1234]]);
        // The cast takes the lookup's answer.
        assert_eq!(
            calls_to(&calls, 0x00ec_43fb),
            [vec![0x1100_1234, 0, 0x0118_3028, 0x0118_3594, 0]]
        );
        assert_eq!(calls_to(&calls, 0x0040_94f0), [vec![ENCHANTMENT, OWNER]]);
        assert!(calls_to(&calls, 0x005b_5e40).is_empty());
    }

    #[test]
    fn a_missing_enchantment_reports_the_owner_it_has() {
        // No owner: no compile index, the short message.
        let (mut e, this) = setup(0x1234, false, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, 0]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&calls, 0x0048_4e60).is_empty());
        assert_eq!(calls_to(&calls, 0x005b_5e40), [vec![0x0101_c1c4, 0x1234]]);
        assert!(calls_to(&calls, 0x0040_94f0).is_empty());

        // An owner `00474cb0` answers for: its name.
        let (mut e, this) = setup(0x1234, false, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_c250, 0x0100_1234, OWNER_NAME]]
        );

        // Any other owner: its `0084e3a0` answer.
        let (mut e, this) = setup(0x1234, false, 0, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_c208, 0x0100_1234, OWNER + 0x77]]
        );
    }

    #[test]
    fn an_enchantment_the_owner_rejects_is_dropped_with_a_message() {
        let (mut e, this) = setup(0x1234, true, 1, false);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![
                0x0101_af00,
                ENCHANTMENT + 0x77,
                ENCHANTMENT_NAME,
                OWNER + 0x77,
                OWNER_NAME
            ]]
        );
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const SLOT_10: u32 = 0x7100_0010;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        e.register(ADD_CHUNK_SHORT, |_, _| Ret::default());
        e.register(FORM_ID, |_, a| (a[0] + 0x10).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_ENCHANTMENT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_AMOUNT, |e, a| {
            (0xabcd_0000 | u32::from(e.mem.u16(a[0] + 8))).into_ret()
        });
        e.register(SET_ENCHANTMENT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(SET_SLOT_10_VALUE, |_, _| Ret::default());
        e.register(SLOT_10, |_, a| (a[0] + 0x30).into_ret());
        e
    }

    fn component(e: &mut Engine, enchantment: u32, amount: u16) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, SLOT_10);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, enchantment);
        e.mem.set_u16(object + 8, amount);
        object
    }

    #[test]
    fn save_adds_the_chunks_it_has() {
        let mut e = engine();
        let this = component(&mut e, 0x5000, 7);
        e.call_log = Some(vec![]);
        e.call(0x0047_bd70, &args![this]);
        assert_eq!(calls(&e, ADD_CHUNK_WORD), vec![vec![0x4d54_4945, 0x5010]]);
        assert_eq!(calls(&e, ADD_CHUNK_SHORT), vec![vec![0x544d_4145, 7]]);
        let empty = component(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_bd70, &args![empty]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn copy_component_takes_the_three_values() {
        let mut e = engine();
        let this = component(&mut e, 0, 0);
        let other = component(&mut e, 0x5000, 9);
        e.call_log = Some(vec![]);
        e.call(0x0048_3100, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ENCHANTABLE_FORM,
                0
            ]]
        );
        assert_eq!(e.mem.u32(this + 4), 0x5000);
        assert_eq!(e.mem.u16(this + 8), 9);
        assert_eq!(calls(&e, SET_SLOT_10_VALUE), vec![vec![this, other + 0x30]]);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0048_3100, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn compare_component_branches() {
        let mut e = engine();
        let this = component(&mut e, 0x5000, 9);
        let same = component(&mut e, 0x5000, 9);
        let other_enchantment = component(&mut e, 0x6000, 9);
        let other_amount = component(&mut e, 0x5000, 8);
        assert!(!e.call(0x0048_3190, &args![this, same]).bool());
        assert!(e.call(0x0048_3190, &args![this, other_enchantment]).bool());
        assert!(e.call(0x0048_3190, &args![this, other_amount]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0048_3190, &args![this, same]).bool());
    }
}

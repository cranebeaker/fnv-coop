//! `fallout shared/bgsammoform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, the index).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; `cdecl`: address
/// of a form id, file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `00401170` (`ECX` = form): the byte at +4 (`TESForm::cFormType`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `00474cb0` (`ECX` = form): the length of the name from the form's
/// virtual slot `0x130`, or 0.
const FORM_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;
/// The form's name virtual slot, as `00474cb0` reads it.
const NAME_SLOT: u32 = 0x130;

/// The two form types the ammo may have (form type bytes 0x29 and 0x55).
const FORM_TYPE_FIRST: u32 = 0x29;
const FORM_TYPE_SECOND: u32 = 0x55;

/// `"MASTERFILE: Unable to find ammo (%08X) on unknown owner."`
const MESSAGE_UNKNOWN_OWNER: u32 = 0x0101_a3c8;
/// `"MASTERFILE: Unable to find ammo (%08X) on owner object (%08X)."`
const MESSAGE_OWNER_ID: u32 = 0x0101_a404;
/// `"MASTERFILE: Unable to find ammo (%08X) on owner object \"%s\"."`
const MESSAGE_OWNER_NAME: u32 = 0x0101_a444;

layout! {
    /// `BGSAmmoForm` (Xbox PDB), 8 bytes: a `BaseFormComponent` and the ammo.
    pub struct BGSAmmoForm: 0x8 {
        /// `pFormAmmo` (Xbox PDB): the ammo form; before `InitItem` it holds
        /// the form id read from the file.
        0x04 pFormAmmo: u32,
    }
}

// Translated from 00474bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAmmoForm::InitItem` (Xbox PDB; `RET 4`, `owner` the form being
/// loaded): turns the form id in `pFormAmmo` into the form (the id adjusted
/// by the owner's file with `TESForm::AddCompileIndex`, then looked up). A
/// form whose type byte is neither 0x29 nor 0x55 is dropped. When nothing is
/// left, a `MASTERFILE` message names the owner: unknown (null owner), by
/// name (when `00474cb0` finds one) or by form id. `GetFile` is called on
/// `owner` even when it is null.
pub fn bgs_ammo_form_init_item(e: &mut Engine, this: Ptr<BGSAmmoForm>, owner: Ptr) {
    let loaded = e.get(this, BGSAmmoForm::pFormAmmo);
    if loaded == 0 {
        return;
    }
    let adjusted = e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), loaded);
        let file = e
            .call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
        e.mem.u32(id.addr())
    });
    let form = e.call(FORM_BY_ID, &args![adjusted]).u32();
    e.set(this, BGSAmmoForm::pFormAmmo, form);
    if form != 0
        && e.call(FORM_TYPE, &args![form]).u32() != FORM_TYPE_FIRST
        && e.call(FORM_TYPE, &args![form]).u32() != FORM_TYPE_SECOND
    {
        e.set(this, BGSAmmoForm::pFormAmmo, 0);
    }
    if e.get(this, BGSAmmoForm::pFormAmmo) == 0 {
        if owner.is_null() {
            e.call(PRINT, &args![MESSAGE_UNKNOWN_OWNER, adjusted]);
        } else if e.call(FORM_NAME_LENGTH, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), NAME_SLOT, &[]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_NAME, adjusted, name]);
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_ID, adjusted, owner_id]);
        }
    }
}

/// `00474a40` (`thiscall`, no stack word): the ammo form (the word at +4 when its form type
/// is 0x55), or null.
const GET_AMMO_FORM: u32 = 0x0047_4a40;
/// `00500940` (`thiscall`, no stack word; unit `fallout shared/bgsterminal.cpp`).
const AMMO_ENTRY_LIST: u32 = 0x0050_0940;
/// `006815c0` (`thiscall`, no stack word): the address of the list's first entry word.
const FIRST_ENTRY: u32 = 0x0068_15c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of the cast: from `TESForm` (`01183028`) to `TESAmmo` (`011840c4`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_AMMO_TYPE: u32 = 0x0118_40c4;

// Translated from 00474a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAmmoForm::IsRockItLauncher` (Xbox PDB; `thiscall`, no stack word): true when the
/// ammo form (`00474a40`) is set and the first entry of its list (`00500940`, then
/// `006815c0`) is set and is not a `TESAmmo` (`__RTDynamicCast` from `TESForm` gives null);
/// false otherwise.
pub fn bgs_ammo_form_is_rock_it_launcher(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_AMMO_FORM, &args![this]).u32() == 0 {
        return false;
    }
    let ammo = e.call(GET_AMMO_FORM, &args![this]).u32();
    let list = e.call(AMMO_ENTRY_LIST, &args![ammo]).u32();
    let holder = e.call(FIRST_ENTRY, &args![list]).u32();
    let entry = e.mem.u32(holder);
    if entry == 0 {
        return false;
    }
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![entry, 0u32, TES_FORM_TYPE, TES_AMMO_TYPE, 0u32],
        )
        .u32();
    cast == 0
}

/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The chunk signature `NAM0` (`0x304d414e`).
const CHUNK_NAM0: u32 = 0x304d_414e;
/// The cast's type descriptors: from `BaseFormComponent` (`01183040`) to `BGSAmmoForm`
/// (`01186890`).
const BASE_FORM_COMPONENT_TYPE: u32 = 0x0118_3040;
const BGS_AMMO_FORM_TYPE: u32 = 0x0118_6890;

/// `__RTDynamicCast` of `other` from `BaseFormComponent` to `BGSAmmoForm`.
fn cast_to_ammo_form(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            BASE_FORM_COMPONENT_TYPE,
            BGS_AMMO_FORM_TYPE,
            0u32
        ],
    )
    .u32()
}

// Translated from 00474af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAmmoForm::Save` (Xbox PDB; `thiscall`, no stack word): when an ammo form is set,
/// saves its form id (`0084e3a0`) in a `NAM0` chunk (`00485910`).
pub fn bgs_ammo_form_save(e: &mut Engine, this: Ptr<BGSAmmoForm>) {
    let ammo = e.get(this, BGSAmmoForm::pFormAmmo);
    if ammo == 0 {
        return;
    }
    let id = e.call(FORM_ID, &args![ammo]).u32();
    e.call(ADD_CHUNK_WORD, &args![CHUNK_NAM0, id]);
}

// Translated from 00474b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAmmoForm::CopyComponent` (Xbox PDB; `RET 4`): casts `other` to a `BGSAmmoForm`
/// (`__RTDynamicCast`) and, when it is one, copies its ammo form.
pub fn bgs_ammo_form_copy_component(e: &mut Engine, this: Ptr<BGSAmmoForm>, other: Ptr) {
    let cast = cast_to_ammo_form(e, other);
    if cast == 0 {
        return;
    }
    let ammo = e.mem.u32(cast + 4);
    e.set(this, BGSAmmoForm::pFormAmmo, ammo);
}

// Translated from 00474b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAmmoForm::CompareComponent` (Xbox PDB; `RET 4`, result in `AL`): true ("differs")
/// when `other` is not a `BGSAmmoForm` or holds another ammo form, false when both hold
/// the same one.
pub fn bgs_ammo_form_compare_component(e: &mut Engine, this: Ptr<BGSAmmoForm>, other: Ptr) -> bool {
    let cast = cast_to_ammo_form(e, other);
    if cast == 0 {
        return true;
    }
    e.get(this, BGSAmmoForm::pFormAmmo) != e.mem.u32(cast + 4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00474bc0, bgs_ammo_form_init_item(Ptr<BGSAmmoForm>, Ptr)),
        entry!(0x0047_4a80, bgs_ammo_form_is_rock_it_launcher(Ptr) -> bool),
        entry!(0x0047_4af0, bgs_ammo_form_save(Ptr<BGSAmmoForm>)),
        entry!(
            0x0047_4b30,
            bgs_ammo_form_copy_component(Ptr<BGSAmmoForm>, Ptr)
        ),
        entry!(
            0x0047_4b70,
            bgs_ammo_form_compare_component(Ptr<BGSAmmoForm>, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine where the looked-up form is `found` (0 for none) and
    /// has the type byte `form_type`; the owner has a name of
    /// `name_length` characters.
    fn engine(found: u32, form_type: u8, name_length: u32) -> Engine {
        let mut e = Engine::new();
        e.register(TES_FORM_GET_FILE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register_double(FORM_BY_ID, move |e, _| {
            if found != 0 {
                e.mem.map(found, 0x10);
                e.mem.set_u8(found + 4, form_type);
            }
            Ret {
                eax: found,
                ..Ret::default()
            }
        });
        e.register(FORM_TYPE, |e, a| Ret {
            eax: u32::from(e.mem.u8(a[0] + 4)),
            ..Ret::default()
        });
        e.register_double(FORM_NAME_LENGTH, move |_, _| Ret {
            eax: name_length,
            ..Ret::default()
        });
        e.register(FORM_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(PRINT, |_, _| Ret::default());
        e.register(0x0050_0000, |_, _| Ret {
            eax: 0x0400_0000,
            ..Ret::default()
        });
        e
    }

    fn component(e: &mut Engine, id: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, id);
        object
    }

    #[test]
    fn nothing_is_done_without_a_loaded_id() {
        let mut e = engine(0x9000, 0x29, 0);
        let object = component(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00474bc0, &args![object, 0x8000u32]);
        let log: CallLog = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn an_ammo_of_either_form_type_is_kept() {
        for (found, form_type, type_calls) in [(0x9000, 0x29, 1), (0xa000, 0x55, 2)] {
            let mut e = engine(found, form_type, 0);
            let object = component(&mut e, 0x42);
            let owner = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            e.call(0x00474bc0, &args![object, owner]);
            assert_eq!(e.mem.u32(object + 4), found);
            assert_eq!(
                calls_to(&e, TES_FORM_GET_FILE),
                vec![vec![owner, 0xffff_ffff]]
            );
            assert_eq!(calls_to(&e, FORM_BY_ID), vec![vec![0x0100_0042]]);
            assert_eq!(calls_to(&e, FORM_TYPE).len(), type_calls);
            assert!(calls_to(&e, PRINT).is_empty());
        }
    }

    #[test]
    fn a_form_of_another_type_is_dropped_and_reported_by_owner_id() {
        let mut e = engine(0x9000, 0x30, 0);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00474bc0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(calls_to(&e, FORM_TYPE).len(), 2);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_ID, 0x0100_0042, 0x1234]]
        );
    }

    #[test]
    fn a_missing_form_is_reported_for_an_unknown_owner() {
        let mut e = engine(0, 0, 0);
        let object = component(&mut e, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x00474bc0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(calls_to(&e, TES_FORM_GET_FILE), vec![vec![0, 0xffff_ffff]]);
        assert!(calls_to(&e, FORM_TYPE).is_empty());
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_UNKNOWN_OWNER, 0x0100_0042]]
        );
    }

    #[test]
    fn a_missing_form_is_reported_by_the_owner_name_when_it_has_one() {
        let mut e = engine(0, 0, 5);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.mem.set_u32(owner, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x00474bc0, &args![object, owner]);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_NAME, 0x0100_0042, 0x0400_0000]]
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn run(form: u32, entry: u32, cast: u32) -> (bool, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, entry);
        e.register_double(GET_AMMO_FORM, move |_, _| form.into_ret());
        e.register(AMMO_ENTRY_LIST, |_, _| 0x6100u32.into_ret());
        e.register_double(FIRST_ENTRY, move |_, _| holder.into_ret());
        e.register_double(DYNAMIC_CAST, move |_, _| cast.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0047_4a80, &args![0x7000u32]).bool();
        (result, e.call_log.take().unwrap())
    }

    #[test]
    fn is_rock_it_launcher_when_the_entry_is_not_ammo() {
        let (result, log) = run(0x6000, 0x6200, 0);
        assert!(result);
        assert!(log.contains(&(AMMO_ENTRY_LIST, vec![0x6000])));
        assert!(log.contains(&(
            DYNAMIC_CAST,
            vec![0x6200, 0, TES_FORM_TYPE, TES_AMMO_TYPE, 0]
        )));
        assert!(!run(0x6000, 0x6200, 0x6200).0);
        assert!(!run(0x6000, 0, 0).0);
        let (result, log) = run(0, 0x6200, 0);
        assert!(!result);
        assert_eq!(log.len(), 2);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_writes_the_ammo_form_id_only_when_set() {
        let mut e = Engine::new();
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        let ammo = e.mem.alloc(0x18);
        e.mem.set_u32(ammo + 0xc, 0x0012_3456);
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0047_4af0, &args![this]);
        assert!(calls(&e, ADD_CHUNK_WORD).is_empty());
        e.mem.set_u32(this + 4, ammo);
        e.call_log = Some(vec![]);
        e.call(0x0047_4af0, &args![this]);
        assert_eq!(calls(&e, FORM_ID), vec![vec![ammo]]);
        assert_eq!(
            calls(&e, ADD_CHUNK_WORD),
            vec![vec![CHUNK_NAM0, 0x0012_3456]]
        );
    }

    #[test]
    fn copy_component_copies_the_ammo_of_an_ammo_form() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x11);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 0x22);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_4b30, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x11);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                BASE_FORM_COMPONENT_TYPE,
                BGS_AMMO_FORM_TYPE,
                0
            ]]
        );
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call(0x0047_4b30, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x22);
    }

    #[test]
    fn compare_component_differs_unless_the_same_ammo() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x44);
        e.mem.set_u32(other + 4, 0x44);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        assert!(!e.call(0x0047_4b70, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0x45);
        assert!(e.call(0x0047_4b70, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0047_4b70, &args![this, other]).bool());
    }
}

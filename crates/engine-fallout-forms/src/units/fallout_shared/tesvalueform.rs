//! `fallout shared/tesvalueform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable offset, source
/// type, target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESValueForm@@` and `.?AVTESForm@@`.
const TYPE_TES_VALUE_FORM: u32 = 0x0118_6b6c;
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The form's virtual at `+0x48`, called with 2.
const SLOT_FORM_0X48: u32 = 0x48;

// Translated from 0048e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the value of a `TESValueForm` component (unnamed in the map; `ECX`,
/// one stack word, `RET 4`; `TESForm::LoadData` calls it with the value
/// it read): stores the word at `+4`, casts the component to its `TESForm`
/// (`__RTDynamicCast` from `TESValueForm`) and, when there is one, calls the
/// form's virtual at `+0x48` with 2 (the slot `TESForm` calls with 1 when
/// the form changed).
pub fn fn_0048e960(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 4, value);
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, TYPE_TES_VALUE_FORM, TYPE_TES_FORM, 0u32],
        )
        .u32();
    if form != 0 {
        e.vcall(form, SLOT_FORM_0X48, &args![2u32]);
    }
}

/// RTTI type descriptors `.?AVMagicItem@@` and `.?AVAlchemyItem@@`.
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
/// `00726070`: the value of a `TESValueForm` component (`thiscall`, a word).
const VALUE_FORM_VALUE: u32 = 0x0072_6070;
/// `_ftol2` (`00ec62c0`): truncates ST0 to an integer (a leading `f64`
/// argument in the uniform form).
const FTOL: u32 = 0x00ec_62c0;

// Translated from 0048e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::GetFormValue` (engine map), `cdecl`: a form's value. A
/// `TESValueForm` gives its own (`00726070`); a `MagicItem` the float of
/// the virtual at slot 8 of its component at `+0x0c` (argument 0), an
/// `AlchemyItem` that of its component at `+0x3c`, each truncated by
/// `_ftol2`; anything else -1.
pub fn tes_value_form_get_form_value(e: &mut Engine, form: Ptr) -> i32 {
    let value_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_VALUE_FORM, 0u32],
        )
        .u32();
    if value_form != 0 {
        return e.call(VALUE_FORM_VALUE, &args![value_form]).i32();
    }
    for (target, component) in [(TYPE_MAGIC_ITEM, 0x0cu32), (TYPE_ALCHEMY_ITEM, 0x3c)] {
        let item = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
            )
            .u32();
        if item != 0 {
            let value = e.vcall(item + component, 8, &args![0u32]).f64();
            return e.call(FTOL, &args![value]).i32();
        }
    }
    -1
}

/// `BaseFormComponent`'s RTTI type descriptor (`01183040`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// The save/load game object the old save functions use (the pointer at
/// `011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008579b0` (thiscall on the save/load game object, `RET 8`: data, size):
/// writes bytes.
const WRITE_BYTES: u32 = 0x0085_79b0;
/// `008579e0` (thiscall on the save/load game object, `RET 8`: data, size):
/// reads bytes.
const READ_BYTES: u32 = 0x0085_79e0;

/// The other component cast to `TESValueForm` (`__RTDynamicCast` from
/// `BaseFormComponent`), or 0.
fn as_value_form(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_VALUE_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 0048e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::CopyComponent` (Xbox PDB, `RET 4`: the other component):
/// when it is a `TESValueForm`, sets this value (`0048e960`) to the other's
/// (`00726070`, its word at +4).
pub fn tes_value_form_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_value_form(e, other);
    if source == 0 {
        return;
    }
    let value = e.call(VALUE_FORM_VALUE, &args![source]).u32();
    fn_0048e960(e, this, value);
}

// Translated from 0048e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::CompareComponent` (Xbox PDB, `RET 4`: the other
/// component): true ("different") unless the other component is a
/// `TESValueForm` with the same value (`00726070` on each, this first).
pub fn tes_value_form_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_value_form(e, other);
    if source == 0 {
        return true;
    }
    let mine = e.call(VALUE_FORM_VALUE, &args![this]).u32();
    mine != e.call(VALUE_FORM_VALUE, &args![source]).u32()
}

// Translated from 0048e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::GetSaveSize` (Xbox PDB, `RET 4`: the save flags): 4 when
/// flag 2 is set, else 0.
pub fn tes_value_form_get_save_size(_e: &mut Engine, _this: Ptr, flags: u32) -> u16 {
    let mut size = 0u16;
    if flags & 2 != 0 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 0048e9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::SaveGame` (Xbox PDB, `RET 4`: the save flags): with flag
/// 2, writes the value (the 4 bytes at +4) to the old save (`008579b0`).
pub fn tes_value_form_save_game(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 2 != 0 {
        let slg = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(WRITE_BYTES, &args![slg, this.addr().wrapping_add(4), 4u32]);
    }
}

// Translated from 0048ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESValueForm::LoadGame` (Xbox PDB, `RET 8`: the save flags and a word it
/// does not read): with flag 2, reads the value (the 4 bytes at +4) back
/// from the old save (`008579e0`).
pub fn tes_value_form_load_game(e: &mut Engine, this: Ptr, flags: u32, _unused_2: u32) {
    if flags & 2 != 0 {
        let slg = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(READ_BYTES, &args![slg, this.addr().wrapping_add(4), 4u32]);
    }
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048e960, fn_0048e960(Ptr, u32)),
        entry!(0x0048_e8a0, tes_value_form_get_form_value(Ptr) -> i32),
        entry!(0x0048_e7f0, tes_value_form_copy_component(Ptr, u32)),
        entry!(0x0048_e840, tes_value_form_compare_component(Ptr, u32) -> bool),
        entry!(0x0048_e9b0, tes_value_form_get_save_size(Ptr, u32) -> u16),
        entry!(0x0048_e9e0, tes_value_form_save_game(Ptr, u32)),
        entry!(0x0048_ea10, tes_value_form_load_game(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0600_0000;
    const HOOK: u32 = 0x0060_0048;

    /// An engine whose cast answers `form`, with a form vtable whose slot
    /// at `+0x48` is [`HOOK`].
    fn engine(form: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(RT_DYNAMIC_CAST, move |_, _| Ret {
            eax: form,
            ..Ret::default()
        });
        e.register(HOOK, |_, _| Ret::default());
        let mut slots = vec![0u32; (SLOT_FORM_0X48 / 4) as usize];
        slots.push(HOOK);
        e.put_vtable(VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn stores_the_value_and_tells_the_form() {
        let form = 0x0700_0000;
        let mut e = engine(form);
        e.map(form, 0x1000);
        e.mem.set_u32(form, VTABLE);
        let component = form + 0x18;
        e.call(0x0048_e960, &args![Ptr::<()>::new(component), 0x1234u32]);
        assert_eq!(e.mem.u32(component + 4), 0x1234);
        let log = e.call_log.unwrap();
        let casts: Vec<_> = log.iter().filter(|c| c.0 == RT_DYNAMIC_CAST).collect();
        assert_eq!(
            casts[0].1,
            vec![component, 0, TYPE_TES_VALUE_FORM, TYPE_TES_FORM, 0]
        );
        let hooks: Vec<_> = log.iter().filter(|c| c.0 == HOOK).collect();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].1, vec![form, 2]);
    }

    #[test]
    fn stores_the_value_without_a_form() {
        let mut e = engine(0);
        let component = e.mem.alloc(8);
        e.call(0x0048_e960, &args![Ptr::<()>::new(component), 77u32]);
        assert_eq!(e.mem.u32(component + 4), 77);
        assert!(e.call_log.unwrap().iter().all(|c| c.0 != HOOK));
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    /// A cast double that answers `answer` for the target type `hit`.
    fn get_value_engine(hit: u32, answer: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == hit { answer } else { 0 }.into_ret()
        });
        e.register(VALUE_FORM_VALUE, |_, _| 42u32.into_ret());
        e.register(0x7000_0008, |_, _| Ret {
            st0: 7.75,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            (f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn get_form_value_by_kind() {
        let mut e = get_value_engine(TYPE_TES_VALUE_FORM, 0x5000);
        assert_eq!(e.call(0x0048_e8a0, &args![0x4000u32]).i32(), 42);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(VALUE_FORM_VALUE, vec![0x5000])));
        assert!(log.contains(&(
            RT_DYNAMIC_CAST,
            vec![0x4000, 0, TYPE_TES_FORM, TYPE_TES_VALUE_FORM, 0]
        )));

        for (kind, component) in [(TYPE_MAGIC_ITEM, 0x0cu32), (TYPE_ALCHEMY_ITEM, 0x3c)] {
            let item = 0x0900_0000;
            let mut e = get_value_engine(kind, item);
            e.map(item, 0x100);
            e.put_vtable(0x0800_0000, &[0, 0, 0x7000_0008]);
            e.mem.set_u32(item + component, 0x0800_0000);
            assert_eq!(e.call(0x0048_e8a0, &args![0x4000u32]).i32(), 7);
            let log = e.call_log.take().unwrap();
            assert!(log.contains(&(0x7000_0008, vec![item + component, 0])));
        }

        let mut e = get_value_engine(0, 0);
        assert_eq!(e.call(0x0048_e8a0, &args![0x4000u32]).i32(), -1);
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        // Casts to `TESValueForm` succeed for nonzero objects; casts to
        // `TESForm` fail.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == TYPE_TES_VALUE_FORM {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(VALUE_FORM_VALUE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e
    }

    #[test]
    fn copy_and_compare_use_the_others_value() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 250);
        assert!(e.call(0x0048_e840, &[this, other]).bool());
        assert!(e.call(0x0048_e840, &[this, 0]).bool());
        e.call(0x0048_e7f0, &[this, 0]);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x0048_e7f0, &[this, other]);
        assert_eq!(e.mem.u32(this + 4), 250);
        assert!(!e.call(0x0048_e840, &[this, other]).bool());
    }

    #[test]
    fn save_size_save_and_load_follow_flag_2() {
        let mut e = engine();
        assert_eq!(e.call(0x0048_e9b0, &[0x5000, 1]).u16(), 0);
        assert_eq!(e.call(0x0048_e9b0, &[0x5000, 2]).u16(), 4);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x7000);
        e.register(WRITE_BYTES, |_, _| Ret::default());
        e.register(READ_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_e9e0, &[0x5000, 0xfd]);
        e.call(0x0048_ea10, &[0x5000, 0xfd, 9]);
        e.call(0x0048_e9e0, &[0x5000, 2]);
        e.call(0x0048_ea10, &[0x5000, 2, 9]);
        let log = e.call_log.take().unwrap();
        let io: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| *a == WRITE_BYTES || *a == READ_BYTES)
            .collect();
        assert_eq!(
            io,
            vec![
                (WRITE_BYTES, vec![0x7000, 0x5004, 4]),
                (READ_BYTES, vec![0x7000, 0x5004, 4])
            ]
        );
    }
}

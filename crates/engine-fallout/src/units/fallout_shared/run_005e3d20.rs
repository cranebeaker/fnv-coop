//! `fallout shared/run_005e3d20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (`00484e60`, engine map): `ECX` = the form, one stack
/// word (the index, -1 here).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map): cdecl, the address
/// of a form id and the file.
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`tesform.cpp`): cdecl, one word (the form id); its result is
/// the pointer the dynamic cast takes.
const FORM_LOOKUP_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`): pointer, vftable delta, source type,
/// target type, is-reference (cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `BGSExplosion`'s RTTI type descriptor (`.?AVBGSExplosion@@`).
const TYPE_BGS_EXPLOSION: u32 = 0x0118_620c;
/// `BGSDebris`'s RTTI type descriptor (`.?AVBGSDebris@@`).
const TYPE_BGS_DEBRIS: u32 = 0x0118_61f4;
/// `BGSImpactDataSet`'s RTTI type descriptor (`.?AVBGSImpactDataSet@@`).
const TYPE_BGS_IMPACT_DATA_SET: u32 = 0x0118_62dc;

// Translated from 005e3d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls vtable slot 0 of the member objects at `this + 0x2C` and
/// `this + 0x44` (each its own `this`, no stack words).
pub fn fn_005e3d20(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr() + 0x2c, 0x0, &args![]);
    e.vcall(this.addr() + 0x44, 0x0, &args![]);
}

// Translated from 005e3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls vtable slot 1 (+0x4) of the member objects at `this + 0x2C` and
/// `this + 0x44` (each its own `this`, no stack words).
pub fn fn_005e3d50(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr() + 0x2c, 0x4, &args![]);
    e.vcall(this.addr() + 0x44, 0x4, &args![]);
}

/// One block of `fn_005e3d80`: the form id in `form_id` gets the file's
/// compile index (`TESForm::GetFile(-1)` on `file`, then
/// `TESForm::AddCompileIndex` on a stack copy), is looked up through
/// `004839c0` and cast from `TESForm` to `target` (`__RTDynamicCast`).
fn resolve_form_id(e: &mut Engine, file: Ptr, form_id: u32, target: u32) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), form_id);
        let compile_file = e
            .call(TES_FORM_GET_FILE, &args![file, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![local, compile_file]);
        let id = e.mem.u32(local.addr());
        let found = e.call(FORM_LOOKUP_004839C0, &args![id]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![found, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32()
    })
}

// Translated from 005e3d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the form ids this object holds after loading, against `file`
/// (one stack word). Nothing happens when `file` is null or `004013e0` on it
/// returns true. Otherwise each id that is not 0 goes through
/// [`resolve_form_id`]: the getter `005e3fa0` (+0x6C) as a `BGSExplosion`
/// into +0x6C, `008d8520` (+0x68 by its body; the map's name
/// `MiddleHighProcess::GetSavedAcquireObject` is folded code) as a
/// `BGSDebris` into +0x68, `004fb070` as a `BGSExplosion` into +0x80,
/// `00407840` as a `BGSDebris` into +0x7C, `004fd400` as a
/// `BGSImpactDataSet` passed to `00500a20`, and `005e3fc0` (+0xA4) as a
/// `BGSImpactDataSet` passed to `00994ef0` (both `ECX` = `this`).
pub fn fn_005e3d80(e: &mut Engine, this: Ptr, file: Ptr) {
    if file.is_null() || e.call(0x0040_13e0, &args![file]).bool() {
        return;
    }
    let id = fn_005e3fa0(e, this);
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_EXPLOSION);
        e.mem.set_u32(this.addr() + 0x6c, form);
    }
    let id = e.call(0x008d_8520, &args![this]).u32();
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_DEBRIS);
        e.mem.set_u32(this.addr() + 0x68, form);
    }
    let id = e.call(0x004f_b070, &args![this]).u32();
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_EXPLOSION);
        e.mem.set_u32(this.addr() + 0x80, form);
    }
    let id = e.call(0x0040_7840, &args![this]).u32();
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_DEBRIS);
        e.mem.set_u32(this.addr() + 0x7c, form);
    }
    let id = e.call(0x004f_d400, &args![this]).u32();
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_IMPACT_DATA_SET);
        e.call(0x0050_0a20, &args![this, form]);
    }
    let id = fn_005e3fc0(e, this);
    if id != 0 {
        let form = resolve_form_id(e, file, id, TYPE_BGS_IMPACT_DATA_SET);
        e.call(0x0099_4ef0, &args![this, form]);
    }
}

// Translated from 005e3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x6C` (`ECX` only).
pub fn fn_005e3fa0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6c)
}

// Translated from 005e3fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0xA4` (`ECX` only).
pub fn fn_005e3fc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xa4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_3d20, fn_005e3d20(Ptr)),
        entry!(0x005e_3d50, fn_005e3d50(Ptr)),
        entry!(0x005e_3d80, fn_005e3d80(Ptr, Ptr)),
        entry!(0x005e_3fa0, fn_005e3fa0(Ptr) -> u32),
        entry!(0x005e_3fc0, fn_005e3fc0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_005e3d20_and_fn_005e3d50_call_slots_0_and_1_of_both_members() {
        let mut e = Engine::new();
        e.put_vtable(0x0200_0000, &[0x0090_0000, 0x0090_0010]);
        e.put_vtable(0x0200_0100, &[0x0090_0020, 0x0090_0030]);
        for target in [0x0090_0000, 0x0090_0010, 0x0090_0020, 0x0090_0030] {
            e.register(target, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x2c, 0x0200_0000);
        e.mem.set_u32(this + 0x44, 0x0200_0100);
        e.call_log = Some(vec![]);
        e.call(0x005e_3d20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0090_0000, vec![this + 0x2c]),
                (0x0090_0020, vec![this + 0x44])
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x005e_3d50, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0090_0010, vec![this + 0x2c]),
                (0x0090_0030, vec![this + 0x44])
            ]
        );
    }

    #[test]
    fn the_getters_read_0x6c_and_0xa4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x6c, 11);
        e.mem.set_u32(this + 0xa4, 22);
        assert_eq!(e.call(0x005e_3fa0, &args![this]).u32(), 11);
        assert_eq!(e.call(0x005e_3fc0, &args![this]).u32(), 22);
    }

    fn resolve_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(TES_FORM_GET_FILE, |_, _| Ret {
            eax: 0x0700_0000,
            ..Ret::default()
        });
        // The compile index goes into the top byte.
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORM_LOOKUP_004839C0, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0] + 0x1000 * (a[3] & 0xfff),
            ..Ret::default()
        });
        // The getters: +0x68, the explosion and debris ids, the impact set.
        e.register(0x008d_8520, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(0x004f_b070, |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        e.register(0x0040_7840, |e, a| e.mem.u32(a[0] + 0x7c).into_ret());
        e.register(0x004f_d400, |e, a| e.mem.u32(a[0] + 0x90).into_ret());
        e.register(0x0050_0a20, |_, _| Ret::default());
        e.register(0x0099_4ef0, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_005e3d80_resolves_every_present_id() {
        let mut e = resolve_engine();
        let this = e.mem.alloc(0xb0);
        let file = e.mem.alloc(4);
        e.mem.set_u32(this + 0x6c, 0x1);
        e.mem.set_u32(this + 0x68, 0x2);
        e.mem.set_u32(this + 0x80, 0x3);
        e.mem.set_u32(this + 0x7c, 0x4);
        e.mem.set_u32(this + 0x90, 0x5);
        e.mem.set_u32(this + 0xa4, 0x6);
        e.call_log = Some(vec![]);
        e.call(0x005e_3d80, &args![this, file]);
        let cast = |id: u32, target: u32| 0x0100_0000 + id + 0x10 + 0x1000 * (target & 0xfff);
        assert_eq!(e.mem.u32(this + 0x6c), cast(1, TYPE_BGS_EXPLOSION));
        assert_eq!(e.mem.u32(this + 0x68), cast(2, TYPE_BGS_DEBRIS));
        assert_eq!(e.mem.u32(this + 0x80), cast(3, TYPE_BGS_EXPLOSION));
        assert_eq!(e.mem.u32(this + 0x7c), cast(4, TYPE_BGS_DEBRIS));
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(
            of(0x0050_0a20),
            vec![vec![this, cast(5, TYPE_BGS_IMPACT_DATA_SET)]]
        );
        assert_eq!(
            of(0x0099_4ef0),
            vec![vec![this, cast(6, TYPE_BGS_IMPACT_DATA_SET)]]
        );
        let casts = of(RT_DYNAMIC_CAST);
        assert_eq!(casts.len(), 6);
        assert_eq!(
            casts[0],
            vec![0x0100_0011, 0, TYPE_TES_FORM, TYPE_BGS_EXPLOSION, 0]
        );
        assert_eq!(of(TES_FORM_GET_FILE)[0], vec![file, 0xffff_ffff]);
        assert_eq!(of(TES_FORM_ADD_COMPILE_INDEX)[0][1], 0x0700_0000);
    }

    #[test]
    fn fn_005e3d80_skips_zero_ids_a_null_file_and_a_true_check() {
        let mut e = resolve_engine();
        let this = e.mem.alloc(0xb0);
        let file = e.mem.alloc(4);
        e.mem.set_u32(this + 0x68, 0x2);
        e.call_log = Some(vec![]);
        e.call(0x005e_3d80, &args![this, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == RT_DYNAMIC_CAST).count(), 1);
        assert_eq!(log.iter().filter(|c| c.0 == 0x0050_0a20).count(), 0);

        e.call_log = Some(vec![]);
        e.call(0x005e_3d80, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        e.register(0x0040_13e0, |_, _| Ret {
            eax: 0x1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005e_3d80, &args![this, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (0x0040_13e0, vec![file]));
    }
}

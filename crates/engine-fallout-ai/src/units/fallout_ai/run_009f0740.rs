//! `fallout/ai/run_009f0740` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESPackage::InitLoadGame` (`00679a90`, engine map): `thiscall`, one word (`RET 4`).
const PACKAGE_INIT_LOAD_GAME: u32 = 0x0067_9a90;
/// `004839c0` (unnamed, `tesform.cpp`): `cdecl`, one word (a form id); the form.
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, engine map): `cdecl` (object, 0, source type, target
/// type, 0).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `TESForm` (`0x01183028`), `MobileObject` (`0x01184920`)
/// and `TESObjectREFR` (`0x011841cc`), by their decorated names.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_MOBILE_OBJECT: u32 = 0x0118_4920;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `0083c420` (unnamed, `conversation.cpp`): `thiscall`, one word (`RET 4`).
const CONVERSATION_INIT_LOAD: u32 = 0x0083_c420;
/// `0067d920` (unnamed, `packagedatauseweapon.cpp`): the base constructor (`ECX` only).
const BASE_CONSTRUCT_0067D920: u32 = 0x0067_d920;
/// The vtable `01093188` (RTTI `.?AVEscortActorPackageData@@`).
const ESCORT_DATA_VTABLE: u32 = 0x0109_3188;
/// `0096a2d0` (unnamed): constructs the member at `+0x10` (`ECX` only).
const MEMBER_CONSTRUCT_0096A2D0: u32 = 0x0096_a2d0;

/// A saved form id at `field` replaced by the form it names, cast to `target`
/// (`004839c0`, `__RTDynamicCast` from `TESForm`); 0 stays 0.
fn resolve(e: &mut Engine, field: u32, target: u32) {
    let id = e.mem.u32(field);
    let resolved = if id != 0 {
        let form = e.call(FORM_FOR_ID, &args![id]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(field, resolved);
}

// Translated from 009f0740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a package's load fix-up):
/// `TESPackage::InitLoadGame` with `buffer`, then the saved ids at `+0x94`, `+0xb0`,
/// `+0xb4` and `+0xb8` become `MobileObject`s and the one at `+0x9c` a `TESObjectREFR`
/// (`resolve`); the object at `+0xa4`, when set, gets `0083c420` with `buffer`.
pub fn fn_009f0740(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(PACKAGE_INIT_LOAD_GAME, &args![this, buffer]);
    for offset in [0x94u32, 0xb0, 0xb4, 0xb8] {
        resolve(e, p + offset, TYPE_MOBILE_OBJECT);
    }
    resolve(e, p + 0x9c, TYPE_TES_OBJECT_REFR);
    let conversation = e.mem.u32(p + 0xa4);
    if conversation != 0 {
        e.call(CONVERSATION_INIT_LOAD, &args![conversation, buffer]);
    }
}

// Translated from 009f0900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f0900 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): the constructor of the object with the vtable
/// `01093188` (RTTI `.?AVEscortActorPackageData@@`): the base `0067d920`, the vtable, the
/// member at `+0x10` (`0096a2d0`) and the words at `+4`, `+8` and `+0xc` cleared. Returns
/// `this`.
pub fn fn_009f0900(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_0067D920, &args![this]);
    e.mem.set_u32(p, ESCORT_DATA_VTABLE);
    e.call(MEMBER_CONSTRUCT_0096A2D0, &args![p + 0x10]);
    e.mem.set_u32(p + 0x4, 0);
    e.mem.set_u32(p + 0x8, 0);
    e.mem.set_u32(p + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_0740, fn_009f0740(Ptr, u32)),
        entry!(0x009f_0900, fn_009f0900(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_009f0740_resolves_the_saved_ids() {
        let mut e = Engine::new();
        e.register(PACKAGE_INIT_LOAD_GAME, |_, _| Ret::default());
        e.register(FORM_FOR_ID, |_, a| ret(a[0] + 0x1000));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[3] == TYPE_TES_OBJECT_REFR {
                a[0] + 1
            } else {
                a[0] + 2
            })
        });
        e.register(CONVERSATION_INIT_LOAD, |_, _| Ret::default());
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x94, 0x10);
        e.mem.set_u32(this + 0xb4, 0x20);
        e.mem.set_u32(this + 0x9c, 0x30);
        e.call_log = Some(vec![]);
        e.call(0x009f_0740, &args![this, 0x6000u32]);
        assert_eq!(calls(&e, PACKAGE_INIT_LOAD_GAME), vec![vec![this, 0x6000]]);
        assert_eq!(e.mem.u32(this + 0x94), 0x1012);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0x1022);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.u32(this + 0x9c), 0x1031);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x1010, 0, TYPE_TES_FORM, TYPE_MOBILE_OBJECT, 0]
        );
        assert!(calls(&e, CONVERSATION_INIT_LOAD).is_empty());
        e.mem.set_u32(this + 0xa4, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x009f_0740, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&e, CONVERSATION_INIT_LOAD),
            vec![vec![0x7700, 0x6000]]
        );
    }

    #[test]
    fn fn_009f0900_constructs_the_data() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_0067D920, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCT_0096A2D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcd; 0x20]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_0900, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCT_0067D920), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMBER_CONSTRUCT_0096A2D0),
            vec![vec![this + 0x10]]
        );
        assert_eq!(e.mem.u32(this), ESCORT_DATA_VTABLE);
        let words: Vec<u32> = (1..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0x10), 0xcdcd_cdcd);
    }
}

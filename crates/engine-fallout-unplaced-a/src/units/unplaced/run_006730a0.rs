//! `(unplaced)/run_006730a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0067fcb0`: constructor of the 8-byte value at `TESPackage + 0x38`.
const VALUE_CONSTRUCT: u32 = 0x0067_fcb0;
/// `0067fd40`: fills that value from a source object.
const VALUE_FILL: u32 = 0x0067_fd40;
/// `00483710` (`tesform.cpp`): the destructor run on the stack value.
const VALUE_DESTRUCT: u32 = 0x0048_3710;

// Translated from 00673180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00673180 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Fills the value at `this + 0x38` from `source` (one stack word) through
/// `0067fd40`. When `source` is null it constructs an 8-byte value on the
/// stack (`0067fcb0`), fills from that one's address instead, and destroys
/// it (`00483710`).
pub fn fn_00673180(e: &mut Engine, this: Ptr, source: Ptr) {
    let member = this.byte_add(0x38);
    if !source.is_null() {
        e.call(VALUE_FILL, &args![member, source]);
        return;
    }
    e.with_stack(8, |e, temporary| {
        e.call(VALUE_CONSTRUCT, &args![temporary]);
        e.call(VALUE_FILL, &args![member, temporary]);
        e.call(VALUE_DESTRUCT, &args![temporary]);
    });
}

/// `BGSIdleCollection`'s scalar deleting destructor (`00670b60`, thiscall, flags) and
/// constructor (`00479790`, thiscall; it puts `BGSIdleCollection`'s component vtable),
/// and `operator new` (`00401000`, cdecl size).
const IDLE_COLLECTION_DELETE: u32 = 0x0067_0b60;
const IDLE_COLLECTION_CONSTRUCT: u32 = 0x0047_9790;
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BaseFormComponent::CopyComponent` (Xbox PDB slot +0x08).
const SLOT_COPY_COMPONENT: u32 = 0x08;

// Translated from 006730a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006730a0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`; called from [`tes_package_copy`]):
/// sets the package's idle collection (`TESPackage::pIdleCollection`, Xbox PDB +0x44, PC
/// +0x34) from `source`. A null `source` deletes the current one (`00670b60(1)`) and
/// clears the field; otherwise a missing one is made first (0x10 bytes, `00479790`; null
/// when the allocation fails) and `source` is copied into it (its slot +0x08,
/// `CopyComponent`; through the field unchecked).
pub fn fn_006730a0(e: &mut Engine, this: Ptr, source: u32) {
    let field = this.addr() + 0x34;
    if source == 0 {
        let current = e.mem.u32(field);
        if current != 0 {
            e.call(IDLE_COLLECTION_DELETE, &args![current, 1u32]);
        }
        e.mem.set_u32(field, 0);
        return;
    }
    if e.mem.u32(field) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let made = if block == 0 {
            0
        } else {
            e.call(IDLE_COLLECTION_CONSTRUCT, &args![block]).u32()
        };
        e.mem.set_u32(field, made);
    }
    let collection = e.mem.u32(field);
    e.vcall(collection, SLOT_COPY_COMPONENT, &args![source]);
}

/// `TESCondition`'s copy (`00680a70`, thiscall: source, a flag), constructor (`00680890`)
/// and destructor (`006808b0`).
const CONDITION_COPY: u32 = 0x0068_0a70;
const CONDITION_CONSTRUCT: u32 = 0x0068_0890;
const CONDITION_DESTRUCT: u32 = 0x0068_08b0;

// Translated from 00673200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00673200 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`; called from [`tes_package_copy`]):
/// copies `source` into the package's conditions (`TESPackage::packConditions`, Xbox PDB
/// +0x50, PC +0x40) with `00680a70(source, 0)`; a null `source` copies an empty
/// condition made on the stack instead (`00680890` .. `006808b0`).
pub fn fn_00673200(e: &mut Engine, this: Ptr, source: u32) {
    let conditions = this.addr() + 0x40;
    if source != 0 {
        e.call(CONDITION_COPY, &args![conditions, source, 0u32]);
        return;
    }
    e.with_stack(8, |e, empty| {
        e.call(CONDITION_CONSTRUCT, &args![empty]);
        e.call(CONDITION_COPY, &args![conditions, empty, 0u32]);
        e.call(CONDITION_DESTRUCT, &args![empty]);
    });
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl five words) and the type descriptors of `TESForm`
/// (`01183028`) and `TESPackage` (`011846a0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_PACKAGE_TYPE: u32 = 0x0118_46a0;
/// The callees of [`tes_package_copy`] (thiscall): `TESForm::CopyAllComponents`
/// (`004851b0`), the location getter `0055b980` and `TESPackage::SetPackageLocation`
/// (`00671d30`), the target getter `00671d10` and `TESPackage::SetPackageTarget`
/// (`00672fc0`), the schedule getter `0041d8a0`, the getter `00441110` and setter
/// `0050f9a0` (the map's names, `PathingLocation::GetWorldspace`, are a folded body), the
/// conditions getter `0087eaa0`, the idle collection getter `005f36f0` (folded name
/// `ActorMover::GetPreferredMoveMode`), the getter `006733e0` and setter `00673400` (folded
/// names `LowProcess::Get/SetSecondGenericLocation`), the getter `0041ca90` and
/// `TESPackage::SetPackType` (`00670fc0`), the event-action copy `0067dad0` and the
/// getter `009611e0`.
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const GETTER_0055B980: u32 = 0x0055_b980;
const SET_PACKAGE_LOCATION: u32 = 0x0067_1d30;
const GETTER_00671D10: u32 = 0x0067_1d10;
const SET_PACKAGE_TARGET: u32 = 0x0067_2fc0;
const GETTER_0041D8A0: u32 = 0x0041_d8a0;
const GETTER_00441110: u32 = 0x0044_1110;
const SETTER_0050F9A0: u32 = 0x0050_f9a0;
const GETTER_0087EAA0: u32 = 0x0087_eaa0;
const GETTER_005F36F0: u32 = 0x005f_36f0;
const GETTER_006733E0: u32 = 0x0067_33e0;
const SETTER_00673400: u32 = 0x0067_3400;
const GETTER_0041CA90: u32 = 0x0041_ca90;
const SET_PACK_TYPE: u32 = 0x0067_0fc0;
const EVENT_ACTION_COPY: u32 = 0x0067_dad0;
const GETTER_009611E0: u32 = 0x0096_11e0;

/// Calls the setter `set` on `this` with what the getter `get` answers on `from`.
fn copy_through(e: &mut Engine, this: Ptr, from: u32, get: u32, set: u32) {
    let value = e.call(get, &args![from]).u32();
    e.call(set, &args![this, value]);
}

// Translated from 00673290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::Copy` (Xbox PDB slot name `TESForm::Copy`, vtable +0x108; at +0x108 of
/// `TESPackage`'s PC vtable `0106847c`; `thiscall`, `RET 4`). Unless `form` is not a
/// `TESPackage` (`__RTDynamicCast`), copies its components (`004851b0`), location,
/// target, schedule ([`fn_00673180`]), the word pair at `+0x22`/`+0x24` (16 bits each, in
/// the exe's order among the calls), conditions ([`fn_00673200`]), idle collection
/// ([`fn_006730a0`]) and the other fields through their getters and setters, the package
/// data (its slot +0x04 with the other's `+0x28`, when this package has one), the three
/// event actions at `+0x4C`, `+0x5C` and `+0x6C` (`0067dad0`, from `form`'s own
/// addresses) and the word at `+0x18` (`009611e0`).
pub fn tes_package_copy(e: &mut Engine, this: Ptr, form: u32) {
    let package = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, TES_PACKAGE_TYPE, 0u32],
        )
        .u32();
    if package == 0 {
        return;
    }
    let base = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, package]);
    copy_through(e, this, package, GETTER_0055B980, SET_PACKAGE_LOCATION);
    copy_through(e, this, package, GETTER_00671D10, SET_PACKAGE_TARGET);
    let schedule = e.call(GETTER_0041D8A0, &args![package]).u32();
    fn_00673180(e, this, Ptr::new(schedule));
    copy_through(e, this, package, GETTER_00441110, SETTER_0050F9A0);
    let word = e.mem.u16(package.wrapping_add(0x22));
    e.mem.set_u16(base + 0x22, word);
    let conditions = e.call(GETTER_0087EAA0, &args![package]).u32();
    fn_00673200(e, this, conditions);
    let collection = e.call(GETTER_005F36F0, &args![package]).u32();
    fn_006730a0(e, this, collection);
    copy_through(e, this, package, GETTER_006733E0, SETTER_00673400);
    copy_through(e, this, package, GETTER_0041CA90, SET_PACK_TYPE);
    let word = e.mem.u16(package.wrapping_add(0x24));
    e.mem.set_u16(base + 0x24, word);
    let data = e.mem.u32(base + 0x28);
    if data != 0 {
        let other = e.mem.u32(package.wrapping_add(0x28));
        e.vcall(data, 4, &args![other]);
    }
    for offset in [0x4cu32, 0x5c, 0x6c] {
        e.call(
            EVENT_ACTION_COPY,
            &args![base + offset, form.wrapping_add(offset)],
        );
    }
    let value = e.call(GETTER_009611E0, &args![package]).u32();
    e.mem.set_u32(base + 0x18, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_3180, fn_00673180(Ptr, Ptr)),
        entry!(0x0067_30a0, fn_006730a0(Ptr, u32)),
        entry!(0x0067_3200, fn_00673200(Ptr, u32)),
        entry!(0x0067_3290, tes_package_copy(Ptr, u32)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00673180_fills_from_the_source_or_a_temporary() {
        let mut e = Engine::new();
        for callee in [VALUE_CONSTRUCT, VALUE_FILL, VALUE_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0067_3180, &args![0x4000u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(VALUE_FILL, vec![0x4038, 0x7000])]);

        e.call_log = Some(vec![]);
        e.call(0x0067_3180, &args![0x4000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 4);
        let temporary = log[1].1[0];
        assert_eq!(log[1], (VALUE_CONSTRUCT, vec![temporary]));
        assert_eq!(log[2], (VALUE_FILL, vec![0x4038, temporary]));
        assert_eq!(log[3], (VALUE_DESTRUCT, vec![temporary]));
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A collection object whose vtable slot +0x08 is the recorded double `0x0151_0008`.
    fn collection(e: &mut Engine) -> u32 {
        e.put_vtable(0x0151_0000, &[0, 0, 0x0151_0008]);
        e.register(0x0151_0008, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0151_0000);
        object
    }

    #[test]
    fn idle_collection_is_made_copied_or_deleted() {
        let mut e = Engine::new();
        let made = collection(&mut e);
        e.register(IDLE_COLLECTION_DELETE, |_, _| Ret::default());
        e.register_double(OPERATOR_NEW, move |_, _| made.into_ret());
        e.register(IDLE_COLLECTION_CONSTRUCT, |_, a| a[0].into_ret());
        let package = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x0067_30a0, &args![package, 0x7000u32]);
        assert_eq!(e.mem.u32(package + 0x34), made);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, 0x0151_0008), vec![vec![made, 0x7000]]);
        // An existing one is reused.
        e.call(0x0067_30a0, &args![package, 0x7100u32]);
        assert_eq!(calls_to(&e, OPERATOR_NEW).len(), 1);
        // Null deletes it.
        e.call(0x0067_30a0, &args![package, 0u32]);
        assert_eq!(calls_to(&e, IDLE_COLLECTION_DELETE), vec![vec![made, 1]]);
        assert_eq!(e.mem.u32(package + 0x34), 0);
        e.call(0x0067_30a0, &args![package, 0u32]);
        assert_eq!(calls_to(&e, IDLE_COLLECTION_DELETE).len(), 1);
    }

    #[test]
    fn conditions_copy_the_source_or_an_empty_one() {
        let mut e = Engine::new();
        for callee in [CONDITION_COPY, CONDITION_CONSTRUCT, CONDITION_DESTRUCT] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0067_3200, &args![0x4000u32, 0x7000u32]);
        assert_eq!(calls_to(&e, CONDITION_COPY), vec![vec![0x4040, 0x7000, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x0067_3200, &args![0x4000u32, 0u32]);
        let empty = calls_to(&e, CONDITION_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, CONDITION_COPY), vec![vec![0x4040, empty, 0]]);
        assert_eq!(calls_to(&e, CONDITION_DESTRUCT), vec![vec![empty]]);
    }

    #[test]
    fn package_copy_copies_every_part_of_a_package() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let other = e.mem.alloc(0x80);
        e.mem.set_u16(other + 0x22, 0x1234);
        e.mem.set_u16(other + 0x24, 0x5678);
        e.mem.set_u32(other + 0x28, 0x9999);
        // The package data object (slot +0x04 recorded).
        e.put_vtable(0x0152_0000, &[0, 0x0152_0004]);
        e.register(0x0152_0004, |_, _| Ret::default());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0152_0000);
        e.mem.set_u32(this + 0x28, data);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            if a[0] == other {
                other.into_ret()
            } else {
                Ret::default()
            }
        });
        for getter in [
            GETTER_0055B980,
            GETTER_00671D10,
            GETTER_0041D8A0,
            GETTER_00441110,
            GETTER_0087EAA0,
            GETTER_005F36F0,
            GETTER_006733E0,
            GETTER_0041CA90,
            GETTER_009611E0,
        ] {
            e.register(getter, |_, a| (a[0] + 1).into_ret());
        }
        for callee in [
            COPY_ALL_COMPONENTS,
            SET_PACKAGE_LOCATION,
            SET_PACKAGE_TARGET,
            SETTER_0050F9A0,
            SETTER_00673400,
            SET_PACK_TYPE,
            EVENT_ACTION_COPY,
            0x0067_fd40,
            CONDITION_COPY,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // The idle collection exists already.
        let idle = collection(&mut e);
        e.mem.set_u32(this + 0x34, idle);
        e.call_log = Some(vec![]);
        e.call(0x0067_3290, &args![this, 0x5555u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.call(0x0067_3290, &args![this, other]);
        assert_eq!(calls_to(&e, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            calls_to(&e, SET_PACKAGE_TARGET),
            vec![vec![this, other + 1]]
        );
        assert_eq!(
            calls_to(&e, 0x0067_fd40),
            vec![vec![this + 0x38, other + 1]]
        );
        assert_eq!(
            calls_to(&e, CONDITION_COPY),
            vec![vec![this + 0x40, other + 1, 0]]
        );
        assert_eq!(calls_to(&e, 0x0151_0008), vec![vec![idle, other + 1]]);
        assert_eq!(calls_to(&e, SET_PACK_TYPE), vec![vec![this, other + 1]]);
        assert_eq!(e.mem.u16(this + 0x22), 0x1234);
        assert_eq!(e.mem.u16(this + 0x24), 0x5678);
        assert_eq!(calls_to(&e, 0x0152_0004), vec![vec![data, 0x9999]]);
        assert_eq!(
            calls_to(&e, EVENT_ACTION_COPY),
            vec![
                vec![this + 0x4c, other + 0x4c],
                vec![this + 0x5c, other + 0x5c],
                vec![this + 0x6c, other + 0x6c],
            ]
        );
        assert_eq!(e.mem.u32(this + 0x18), other + 1);
    }
}

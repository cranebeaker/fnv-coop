//! `fallout/ai/run_009f25f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActorPackageData`'s constructor (`0067d920`: stores the base vtable).
const ACTOR_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_d920;
/// The `GuardActorPackageData` vtable (its RTTI locator names
/// `.?AVGuardActorPackageData@@`).
const GUARD_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_33c8;

// Translated from 009f27a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GuardActorPackageData` constructor: the `ActorPackageData` base,
/// the class's vtable, then `pGuardWarnedActor` (+8), `iWarningCount`
/// (+0xC), `fGuardTimer` (+0x10, 0.0) and `eState` (+4) cleared (Xbox PDB
/// field names). Returns `this`.
pub fn fn_009f27a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ACTOR_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, GUARD_ACTOR_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_f32(base + 0x10, 0.0);
    e.mem.set_u32(base + 4, 0);
    this
}

/// `TESPackage::InitLoadGame` (`00679a90`, engine map): `thiscall`, one word (`RET 4`).
const PACKAGE_INIT_LOAD_GAME: u32 = 0x0067_9a90;
/// `004839c0` (unnamed, `tesform.cpp`): `cdecl`, one word (a form id); the form.
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, engine map): `cdecl` (object, 0, source type, target
/// type, 0); the RTTI type descriptors of `TESForm` (`0x01183028`) and `TESObjectREFR`
/// (`0x011841cc`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// The reference list helpers: `008256d0` (whether the node is empty), `006815c0` (the
/// node's item slot), `00726c60` (sets the item, `RET 4`), `00726070` (the next node),
/// `00905330` (removes the node after `this`, given the item slot, `RET 4`) and
/// `0063f7b0` (removes the first node's item).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_SET_ITEM: u32 = 0x0072_6c60;
const NODE_NEXT: u32 = 0x0072_6070;
const REMOVE_AFTER: u32 = 0x0090_5330;
const REMOVE_FIRST: u32 = 0x0063_f7b0;

/// A saved form id replaced by its `TESObjectREFR` (`004839c0`, `__RTDynamicCast` from
/// `TESForm`); 0 stays 0.
fn resolve_reference(e: &mut Engine, id: u32) -> u32 {
    if id != 0 {
        let form = e.call(FORM_FOR_ID, &args![id]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
        )
        .u32()
    } else {
        0
    }
}

// Translated from 009f25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a package's load fix-up):
/// `TESPackage::InitLoadGame` with `buffer`, the saved ids at `+0xa0` and `+0xa4` resolved
/// to references, and each id of the list at `+0x98` too; an item that is 0 or does not
/// resolve is removed (after the previous node with `00905330`, or as the first node with
/// `0063f7b0`, which leaves the walk on the same node).
pub fn fn_009f25f0(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(PACKAGE_INIT_LOAD_GAME, &args![this, buffer]);
    for offset in [0xa0u32, 0xa4] {
        let id = e.mem.u32(p + offset);
        let reference = resolve_reference(e, id);
        e.mem.set_u32(p + offset, reference);
    }
    let mut node = p + 0x98;
    let mut previous = 0u32;
    while node != 0 && e.call(NODE_IS_EMPTY, &args![node]).u8() == 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let id = e.mem.u32(slot);
        let mut remove = false;
        if id != 0 {
            let reference = resolve_reference(e, id);
            if reference != 0 {
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), reference);
                    e.call(NODE_SET_ITEM, &args![node, local]);
                });
            } else {
                remove = true;
            }
        } else {
            remove = true;
        }
        if remove {
            if previous != 0 {
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                e.call(REMOVE_AFTER, &args![previous, slot]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            } else {
                e.call(REMOVE_FIRST, &args![node]);
            }
        } else {
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f27a0, fn_009f27a0(Ptr) -> Ptr),
        entry!(0x009f_25f0, fn_009f25f0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_data_constructor_clears_its_fields() {
        let mut e = Engine::new();
        e.register(ACTOR_PACKAGE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xaa; 0x14]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_27a0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTOR_PACKAGE_DATA_CONSTRUCT, vec![this]));
        assert_eq!(e.mem.u32(this), GUARD_ACTOR_PACKAGE_DATA_VTABLE);
        for offset in [4, 8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// List doubles over nodes `[item, next]`; ids 0x66 do not resolve.
    fn doubles(e: &mut Engine) {
        e.register(PACKAGE_INIT_LOAD_GAME, |_, _| Ret::default());
        e.register(FORM_FOR_ID, |_, a| ret(a[0] + 0x1000));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x1066 { 0 } else { a[0] + 1 })
        });
        e.register(NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(REMOVE_AFTER, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(REMOVE_FIRST, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
    }

    /// `this` with the list `items` at `+0x98` (the head node inline).
    fn package(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0xa8);
        let mut next = 0;
        for item in items[1..].iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        e.mem.set_u32(this + 0x98, items[0]);
        e.mem.set_u32(this + 0x9c, next);
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let mut out = vec![e.mem.u32(this + 0x98)];
        let mut node = e.mem.u32(this + 0x9c);
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn fn_009f25f0_resolves_and_prunes_the_references() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = package(&mut e, &[0x11, 0x66, 0, 0x22]);
        e.mem.set_u32(this + 0xa0, 0x33);
        e.mem.set_u32(this + 0xa4, 0x66);
        e.call(0x009f_25f0, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(this + 0xa0), 0x1034);
        assert_eq!(e.mem.u32(this + 0xa4), 0);
        assert_eq!(items(&e, this), vec![0x1012, 0x1023]);
        // An unresolvable first item is dropped from the head.
        let this = package(&mut e, &[0x66, 0x22]);
        e.call(0x009f_25f0, &args![this, 0x6000u32]);
        assert_eq!(items(&e, this), vec![0x1023]);
    }
}

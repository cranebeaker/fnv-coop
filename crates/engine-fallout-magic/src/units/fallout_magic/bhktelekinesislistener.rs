//! `fallout/magic/bhktelekinesislistener.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `bhkTelekinesisListener` (Xbox PDB), 0x1c bytes: three listener bases (vtables at
    /// +0, +4, +8), then the fields below.
    pub struct BhkTelekinesisListener: 0x1c {
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x0c pRef: u32,
        /// `pBody` (Xbox PDB): `bhkRigidBody*`.
        0x10 pBody: u32,
        /// `DefaultFilter` (Xbox PDB): `CFilter`, one word.
        0x14 DefaultFilter: u32,
        /// `bCanFinish` (Xbox PDB).
        0x18 bCanFinish: u8,
    }
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `NiAVObject`'s RTTI type descriptor (`.?AVNiAVObject@@`).
const NI_AV_OBJECT_TYPE: u32 = 0x0118_6044;
/// `BSFadeNode`'s RTTI type descriptor (`.?AVBSFadeNode@@`).
const BS_FADE_NODE_TYPE: u32 = 0x0119_ba28;
/// `hkpEntityActivationListener`'s RTTI type descriptor (`.?AVhkpEntityActivationListener@@`).
const HKP_ENTITY_ACTIVATION_LISTENER_TYPE: u32 = 0x0119_d900;
/// `bhkTelekinesisListener`'s RTTI type descriptor (`.?AVbhkTelekinesisListener@@`).
const BHK_TELEKINESIS_LISTENER_TYPE: u32 = 0x011a_0eac;
/// `BSFadeNode::SetLODMultType` (`00b4dec0`, thiscall: one word).
const BS_FADE_NODE_SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
/// `00626cb0` (thiscall on the body: the filter word; unnamed).
const BODY_SET_FILTER_00626CB0: u32 = 0x0062_6cb0;
/// `00807cb0` (cdecl: the listener, one word; unnamed).
const FINISH_HELPER_00807CB0: u32 = 0x0080_7cb0;
/// `004ae750` (thiscall, returns a pointer; the map's name `bhkCharacterProxy::operatorP`
/// may come from folded code).
const OWNER_004AE750: u32 = 0x004a_e750;
/// `0096a350` (thiscall on the listener array, returns its count; unnamed).
const ARRAY_COUNT_0096A350: u32 = 0x0096_a350;
/// `00629ab0` (thiscall on the listener array: index; returns the address of the element).
const ARRAY_ELEMENT_00629AB0: u32 = 0x0062_9ab0;
/// `00807bf0` (cdecl: size; an allocator).
const ALLOCATE_00807BF0: u32 = 0x0080_7bf0;
/// `00807c50` (thiscall on a new 0x10-byte block; unnamed constructor).
const CONSTRUCTOR_00807C50: u32 = 0x0080_7c50;

// Translated from 00807a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTelekinesisListener::Finish` (Xbox PDB): with a reference, its slot +0x1d0 object,
/// when it is a `BSFadeNode` (`__RTDynamicCast` from `NiAVObject`), gets LOD mult type 2;
/// the body gets the default filter back (`00626cb0`); `00807cb0` runs with `this` and
/// `argument`; then the listener destroys itself (slot +0xc with 1; skipped for null).
pub fn bhk_telekinesis_listener_finish(
    e: &mut Engine,
    this: Ptr<BhkTelekinesisListener>,
    argument: u32,
) {
    let reference = e.get(this, BhkTelekinesisListener::pRef);
    if reference != 0 {
        let reference = e.get(this, BhkTelekinesisListener::pRef);
        let object = e.vcall(reference, 0x1d0, &[]).u32();
        let node = e
            .call(
                RT_DYNAMIC_CAST,
                &args![object, 0u32, NI_AV_OBJECT_TYPE, BS_FADE_NODE_TYPE, 0u32],
            )
            .u32();
        if node != 0 {
            e.call(BS_FADE_NODE_SET_LOD_MULT_TYPE, &args![node, 2u32]);
        }
    }
    let filter = e.get(this, BhkTelekinesisListener::DefaultFilter);
    let body = e.get(this, BhkTelekinesisListener::pBody);
    e.call(BODY_SET_FILTER_00626CB0, &args![body, filter]);
    e.call(FINISH_HELPER_00807CB0, &args![this, argument]);
    if this.addr() != 0 {
        e.vcall(this.addr(), 0xc, &args![1u32]);
    }
}

// Translated from 00807ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTelekinesisListener::RemoveTelekinesisListeners` (Xbox PDB, cdecl: one word): walks
/// the listener array of the object `004ae750` gives for `argument` (`fn_00807b50`; its
/// count `0096a350` read again each step). Each element that is a `bhkTelekinesisListener`
/// (`__RTDynamicCast` from `hkpEntityActivationListener`) is finished (`Finish` with that
/// object); the index then goes back to 0 and the step makes it 1.
pub fn bhk_telekinesis_listener_remove_telekinesis_listeners(e: &mut Engine, argument: u32) {
    let owner = e.call(OWNER_004AE750, &args![argument]).u32();
    let mut index = 0i32;
    loop {
        let array = fn_00807b50(e, owner);
        if index >= e.call(ARRAY_COUNT_0096A350, &args![array]).i32() {
            break;
        }
        let array = fn_00807b50(e, owner);
        let slot = e.call(ARRAY_ELEMENT_00629AB0, &args![array, index]).u32();
        let element = e.mem.u32(slot);
        let listener = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    element,
                    0u32,
                    HKP_ENTITY_ACTIVATION_LISTENER_TYPE,
                    BHK_TELEKINESIS_LISTENER_TYPE,
                    0u32
                ],
            )
            .u32();
        if listener != 0 {
            bhk_telekinesis_listener_finish(e, Ptr::new(listener), owner);
            index = 0;
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00807b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00807b50 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`bhktelekinesislistener.cpp`): the object at +0x214 of `this`, made on first
/// use (`00807bf0` with 0x10, then `00807c50`; null when the allocation fails).
pub fn fn_00807b50(e: &mut Engine, this: u32) -> u32 {
    if e.mem.u32(this + 0x214) == 0 {
        let block = e.call(ALLOCATE_00807BF0, &args![0x10u32]).u32();
        let object = if block == 0 {
            0
        } else {
            e.call(CONSTRUCTOR_00807C50, &args![block]).u32()
        };
        e.mem.set_u32(this + 0x214, object);
    }
    e.mem.u32(this + 0x214)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_7a10,
            bhk_telekinesis_listener_finish(Ptr<BhkTelekinesisListener>, u32)
        ),
        entry!(
            0x0080_7ac0,
            bhk_telekinesis_listener_remove_telekinesis_listeners(u32)
        ),
        entry!(0x0080_7b50, fn_00807b50(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// A listener whose slot +0xc is a double at `0x0300_000c`, with reference, body and
    /// filter set.
    fn listener(e: &mut Engine, reference: u32) -> Ptr<BhkTelekinesisListener> {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0300_000c);
        answer(e, 0x0300_000c, 0);
        let this: Ptr<BhkTelekinesisListener> = e.new_object();
        e.mem.set_u32(this.addr(), vtable);
        e.set(this, BhkTelekinesisListener::pRef, reference);
        e.set(this, BhkTelekinesisListener::pBody, 0x6000);
        e.set(this, BhkTelekinesisListener::DefaultFilter, 0x0f0f);
        this
    }

    #[test]
    fn finish_restores_the_filter_and_the_fade_node() {
        for fade_node in [0u32, 0x7100] {
            let mut e = Engine::new();
            let reference = {
                let vtable = e.mem.alloc(0x200);
                e.mem.set_u32(vtable + 0x1d0, 0x0300_01d0);
                let reference = e.mem.alloc(8);
                e.mem.set_u32(reference, vtable);
                reference
            };
            answer(&mut e, 0x0300_01d0, 0x7000);
            answer(&mut e, RT_DYNAMIC_CAST, fade_node);
            for addr in [
                BS_FADE_NODE_SET_LOD_MULT_TYPE,
                BODY_SET_FILTER_00626CB0,
                FINISH_HELPER_00807CB0,
            ] {
                answer(&mut e, addr, 0);
            }
            let this = listener(&mut e, reference);
            e.call_log = Some(vec![]);
            e.call(0x0080_7a10, &args![this, 0x77u32]);
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![0x7000, 0, NI_AV_OBJECT_TYPE, BS_FADE_NODE_TYPE, 0]]
            );
            let lod = calls(&e, BS_FADE_NODE_SET_LOD_MULT_TYPE);
            if fade_node == 0 {
                assert!(lod.is_empty());
            } else {
                assert_eq!(lod, vec![vec![0x7100, 2]]);
            }
            assert_eq!(
                calls(&e, BODY_SET_FILTER_00626CB0),
                vec![vec![0x6000, 0x0f0f]]
            );
            assert_eq!(
                calls(&e, FINISH_HELPER_00807CB0),
                vec![vec![this.addr(), 0x77]]
            );
            assert_eq!(calls(&e, 0x0300_000c), vec![vec![this.addr(), 1]]);
        }
    }

    #[test]
    fn remove_finishes_each_listener_and_restarts_at_one() {
        let mut e = Engine::new();
        let owner = e.mem.alloc(0x220);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 0x214, array);
        answer(&mut e, OWNER_004AE750, owner);
        // Three elements; the second is a listener.
        let elements = e.mem.alloc(12);
        e.mem.set_u32(elements, 0x10);
        e.mem.set_u32(elements + 4, 0x20);
        e.mem.set_u32(elements + 8, 0x30);
        answer(&mut e, ARRAY_COUNT_0096A350, 3);
        e.register_double(ARRAY_ELEMENT_00629AB0, move |_, a| {
            (elements + 4 * a[1]).into_ret()
        });
        let first = listener(&mut e, 0);
        let first_addr = first.addr();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[0] == 0x20 { first_addr } else { 0 }).into_ret()
        });
        answer(&mut e, BODY_SET_FILTER_00626CB0, 0);
        // Finishing takes the listener out of the array.
        e.register_double(FINISH_HELPER_00807CB0, move |e, _| {
            e.mem.set_u32(elements + 4, 0x30);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0080_7ac0, &args![0x99u32]);
        assert_eq!(calls(&e, OWNER_004AE750), vec![vec![0x99]]);
        // Index 0, 1 (a listener: back to 0, then 1), 1 again, 2, then the end.
        let indices: Vec<u32> = calls(&e, ARRAY_ELEMENT_00629AB0)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(indices, vec![0, 1, 1, 2]);
        assert_eq!(
            calls(&e, FINISH_HELPER_00807CB0),
            vec![vec![first_addr, owner]]
        );
    }

    #[test]
    fn fn_00807b50_makes_the_array_once() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x220);
        answer(&mut e, ALLOCATE_00807BF0, 0x7000);
        answer(&mut e, CONSTRUCTOR_00807C50, 0x7004);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_7b50, &args![this]).u32(), 0x7004);
        assert_eq!(e.call(0x0080_7b50, &args![this]).u32(), 0x7004);
        assert_eq!(calls(&e, ALLOCATE_00807BF0), vec![vec![0x10]]);
        assert_eq!(calls(&e, CONSTRUCTOR_00807C50), vec![vec![0x7000]]);
        // A failed allocation leaves null (and tries again next time).
        let other = e.mem.alloc(0x220);
        answer(&mut e, ALLOCATE_00807BF0, 0);
        assert_eq!(e.call(0x0080_7b50, &args![other]).u32(), 0);
        assert_eq!(calls(&e, CONSTRUCTOR_00807C50).len(), 1);
    }
}

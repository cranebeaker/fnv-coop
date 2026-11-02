//! `BSMain/run_00c4b220` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObjectNET::GetController` (`00a5c570`, `thiscall (object, rtti)`)
/// and the `NiRTTI` record it is given (`011f36ac`).
const GET_CONTROLLER: u32 = 0x00a5_c570;
const CONTROLLER_RTTI: u32 = 0x011f_36ac;
/// Makes a pooled `NiFixedString` handle from a C string (`00a5b690`,
/// `cdecl (text)`).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// `InterlockedDecrement` (import slot, one word: the address).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;

/// Releases the name handle: `InterlockedDecrement` of its count unless it
/// is the empty string.
fn release_name(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

// Translated from 00c4b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4b310 - its C++ exception frame (FS:[0], the name handle's unwind) is not translated
/// Finds something by name under `object` (the engine map has no name;
/// `cdecl (object, name, also_object)`): null for a null object or name.
/// The name becomes a pooled string handle (`00a5b690`) held in a stack
/// local. With a controller of the class at `011f36ac` (`GetController`),
/// the object at the controller's +0x78 is asked first (vtable +0x8C with
/// the handle's address); a non-null answer is returned. Otherwise, when
/// `also_object` is set, `object` itself is asked (vtable +0x9C with the
/// handle's address) and its answer returned; else null. The handle is
/// released before each return.
pub fn fn_00c4b310(e: &mut Engine, object: Ptr, name: Ptr, also_object: bool) -> u32 {
    if object.is_null() || name.is_null() {
        return 0;
    }
    let controller = e
        .call(GET_CONTROLLER, &args![object, CONTROLLER_RTTI])
        .u32();
    e.with_stack(4, |e, handle_slot| {
        // `name` is non-null here; the exe still tests it again.
        let handle = if name.is_null() {
            e.global::<u32>(FIXED_STRING_EMPTY)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
        };
        e.mem.set_u32(handle_slot.addr(), handle);
        if controller != 0 {
            let owner = e.mem.u32(controller.wrapping_add(0x78));
            let found = e.vcall(owner, 0x8c, &args![handle_slot]).u32();
            if found != 0 {
                let handle = e.mem.u32(handle_slot.addr());
                release_name(e, handle);
                return found;
            }
        }
        let found = if also_object {
            e.vcall(object.addr(), 0x9c, &args![handle_slot]).u32()
        } else {
            0
        };
        let handle = e.mem.u32(handle_slot.addr());
        release_name(e, handle);
        found
    })
}

/// `00546780` (unit `tesobjectcell.cpp`; `thiscall`, `RET 4`), called with 1 on each object.
const OBJECT_FLAG_CALL: u32 = 0x0054_6780;
/// `NiNode::ms_RTTI` (`011f4428`).
const NI_NODE_RTTI: u32 = 0x011f_4428;
/// `NiAVObject::m_uFlags` (Xbox PDB) +0x30, and the flag set on every object (`0x2000`).
const AV_OBJECT_FLAGS: u32 = 0x30;
const SET_FLAG: u32 = 0x2000;
/// `NiNode`'s children (`m_kChildren`, Xbox PDB): elements at +0xA0, size at +0xA6.
const CHILDREN: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;

// Translated from 00c4b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks a scene tree (unnamed in the engine map; `cdecl`, one word): a non-null object gets
/// `00546780` with 1 and flag `0x2000` set in its flags (+0x30; the compiler kept a constant
/// test, `MOV EAX,1; TEST EAX,EAX`, so the branch that would clear the flag is folded away
/// and never runs). When the object is an `NiNode` (its RTTI chain, virtual `+0x08` then
/// `+4` links, reaches `011f4428`), every child slot below the children's size is visited the
/// same way (a null child does nothing).
pub fn fn_00c4b220(e: &mut Engine, object: Ptr) {
    let object = object.addr();
    if object != 0 {
        e.call(OBJECT_FLAG_CALL, &args![object, 1u32]);
        let flags = e.mem.u32(object.wrapping_add(AV_OBJECT_FLAGS)) | SET_FLAG;
        e.mem.set_u32(object.wrapping_add(AV_OBJECT_FLAGS), flags);
    }
    let node = if object == 0 {
        0
    } else {
        let mut rtti = e.vcall(object, 0x8, &args![]).u32();
        let mut is_node = false;
        while rtti != 0 {
            if rtti == NI_NODE_RTTI {
                is_node = true;
                break;
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
        if is_node {
            object
        } else {
            0
        }
    };
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
        let children = e.mem.u32(node.wrapping_add(CHILDREN));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        fn_00c4b220(e, Ptr::new(child));
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_b220, fn_00c4b220(Ptr)),
        entry!(0x00c4_b310, fn_00c4b310(Ptr, Ptr, bool) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn by_name_engine(controller: u32, owner_answer: u32) -> (Engine, u32, u32) {
        const OWNER_FIND: u32 = 0x0300_0000;
        const OBJECT_FIND: u32 = 0x0300_0010;
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 2);
        e.register_double(FIXED_STRING_FROM_TEXT, move |_, _| handle.into_ret());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register_double(GET_CONTROLLER, move |_, _| controller.into_ret());
        e.register_double(OWNER_FIND, move |_, _| owner_answer.into_ret());
        e.register(OBJECT_FIND, |_, _| 0x6666u32.into_ret());
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(vtable + 0x8c, OWNER_FIND);
        e.mem.set_u32(vtable + 0x9c, OBJECT_FIND);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        if controller != 0 {
            e.map(controller, 0x80);
            e.mem.set_u32(controller + 0x78, object);
        }
        e.call_log = Some(vec![]);
        (e, object, handle)
    }

    #[test]
    fn find_by_name_asks_the_controller_owner_then_the_object() {
        let name = 0x0700_0000u32;
        // The controller's owner answers.
        let (mut e, object, handle) = by_name_engine(0x0400_0000, 0x5555);
        e.map(name, 4);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x5555
        );
        assert_eq!(e.mem.u32(handle - 8), 1);
        let log = e.call_log.take().unwrap();
        let get: Vec<_> = log.iter().filter(|call| call.0 == GET_CONTROLLER).collect();
        assert_eq!(get[0].1, vec![object, CONTROLLER_RTTI]);
        // No answer from the owner: the object is asked when allowed.
        let (mut e, object, handle) = by_name_engine(0x0400_0000, 0);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x6666
        );
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(e.call(0x00c4_b310, &args![object, name, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(handle - 8), 0);
        // No controller.
        let (mut e, object, _) = by_name_engine(0, 0);
        assert_eq!(
            e.call(0x00c4_b310, &args![object, name, 1u32]).u32(),
            0x6666
        );
        // A null object or name answers null at once.
        assert_eq!(e.call(0x00c4_b310, &args![0u32, name, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00c4_b310, &args![object, 0u32, 1u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_flag_reaches_every_object_under_nodes() {
        let mut e = Engine::new();
        e.register(OBJECT_FLAG_CALL, |_, _| Ret::default());
        // Node RTTI: a derived record whose base is NiNode's; leaves answer another chain.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, NI_NODE_RTTI);
        let leaf_rtti = e.mem.alloc(8);
        e.register_double(0x00ee_a000, move |_, _| derived.into_ret());
        e.register_double(0x00ee_a010, move |_, _| leaf_rtti.into_ret());
        e.put_vtable(0x0300_6000, &[0, 0, 0x00ee_a000]);
        e.put_vtable(0x0300_7000, &[0, 0, 0x00ee_a010]);
        let object = |e: &mut Engine, vtable: u32, children: &[u32]| {
            let o = e.mem.alloc(0xb0);
            e.mem.set_u32(o, vtable);
            let list = e.mem.alloc(4 * children.len() as u32 + 4);
            for (i, child) in children.iter().enumerate() {
                e.mem.set_u32(list + 4 * i as u32, *child);
            }
            e.mem.set_u32(o + CHILDREN, list);
            e.mem.set_u16(o + CHILDREN_SIZE, children.len() as u16);
            o
        };
        let hidden = object(&mut e, 0x0300_6000, &[]);
        // A leaf's own children are not visited.
        let leaf = object(&mut e, 0x0300_7000, &[hidden]);
        let inner = object(&mut e, 0x0300_6000, &[leaf]);
        let root = object(&mut e, 0x0300_6000, &[inner, 0]);
        e.mem.set_u32(leaf + AV_OBJECT_FLAGS, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c4_b220, &args![root]);
        let log = e.call_log.take().unwrap();
        let marked: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == OBJECT_FLAG_CALL)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(marked, vec![root, inner, leaf]);
        assert_eq!(e.mem.u32(leaf + AV_OBJECT_FLAGS), 0x2001);
        assert_eq!(e.mem.u32(root + AV_OBJECT_FLAGS), 0x2000);
        assert_eq!(e.mem.u32(hidden + AV_OBJECT_FLAGS), 0);
        e.call(0x00c4_b220, &args![0u32]);
    }
}

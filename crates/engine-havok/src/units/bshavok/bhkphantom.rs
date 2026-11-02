//! `BSHavok/bhkphantom.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkPhantom` is a `bhkWorldObject` (`+8` is the Havok phantom) with a flag byte at `+0x14`.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, push_viewer_string, NI_GET_VIEWER_STRING, SERIALIZABLE_KILL,
    WORLD_OBJECT_DESTRUCT, WORLD_OBJECT_GET_VIEWER_STRINGS,
};

/// `bhkWorld::Remove` variant for phantoms (`00c68cb0`, one stack argument) and the
/// call `UpdateCFilter` makes on the object slot `0x98` answers (`00c944a0`,
/// `(object, phantom, 1)`, RET 8).
const WORLD_REMOVE_PHANTOM: u32 = 0x00c6_8cb0;
const WORLD_UPDATE_PHANTOM_FILTER: u32 = 0x00c9_44a0;
/// Counter of live `bhkPhantom` objects, the `bhkPhantom` vtable the destructor installs
/// and the `NiRTTI` global of the class.
const PHANTOM_COUNT: u32 = 0x0126_8238;
const PHANTOM_VTABLE: u32 = 0x0103_078c;
const PHANTOM_RTTI: u32 = 0x0126_823c;

layout! {
    /// `bhkPhantom` (Xbox PDB), 0x18 bytes; the fields this unit uses.
    pub struct BhkPhantom: 0x18 {
        /// `phkObject` (Xbox PDB): the Havok phantom.
        0x08 phkObject: Ptr,
    }
}

// Translated from 00c9f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f3b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkPhantom::~bhkPhantom` (Xbox PDB): installs the `bhkPhantom` vtable, runs
/// `bhkSerializable::Kill`, decrements the live phantom counter and runs the
/// `bhkWorldObject` destructor body.
pub fn bhk_phantom_destruct(e: &mut Engine, this: Ptr<BhkPhantom>) {
    e.mem.set_u32(this.addr(), PHANTOM_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(PHANTOM_COUNT);
    e.mem.set_u32(PHANTOM_COUNT, count.wrapping_sub(1));
    e.call(WORLD_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00c9f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPhantom::Remove` (Xbox PDB): asks virtual slot `0x94` for an object and, when it
/// exists and its word at `+0x350` is set, calls `00c68cb0` on that word with the Havok
/// phantom (`+8`) and answers its result; otherwise answers `false`.
pub fn bhk_phantom_remove(e: &mut Engine, this: Ptr<BhkPhantom>) -> bool {
    let world_object = e.vcall(this.addr(), 0x94, &args![]).u32();
    let world = if world_object != 0 {
        e.mem.u32(world_object + 0x350)
    } else {
        0
    };
    if world != 0 {
        let phantom = e.get(this, BhkPhantom::phkObject);
        e.call(WORLD_REMOVE_PHANTOM, &args![world, phantom]).bool()
    } else {
        false
    }
}

// Translated from 00c9f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPhantom::UpdateCFilter` (Xbox PDB): asks virtual slot `0x98` for an object and, when
/// there is one, calls `00c944a0` on it with the Havok phantom (`+8`) and `1`.
pub fn bhk_phantom_update_c_filter(e: &mut Engine, this: Ptr<BhkPhantom>) {
    let world = e.vcall(this.addr(), 0x98, &args![]).u32();
    if world != 0 {
        let phantom = e.get(this, BhkPhantom::phkObject);
        e.call(WORLD_UPDATE_PHANTOM_FILTER, &args![world, phantom, 1u32]);
    }
}

// Translated from 00c9f480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f480 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkPhantom::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10, runs
/// the `bhkWorldObject` strings, then appends the class name (the `NiRTTI` at `0126823c`
/// through `NiGetViewerString`) to the viewer-string array.
pub fn bhk_phantom_get_viewer_strings(e: &mut Engine, this: Ptr<BhkPhantom>, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(WORLD_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(PHANTOM_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_f3b0, bhk_phantom_destruct(Ptr<BhkPhantom>)),
        entry!(0x00c9_f420, bhk_phantom_remove(Ptr<BhkPhantom>) -> bool),
        entry!(0x00c9_f460, bhk_phantom_update_c_filter(Ptr<BhkPhantom>)),
        entry!(
            0x00c9_f480,
            bhk_phantom_get_viewer_strings(Ptr<BhkPhantom>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{SCOPE_WORD, STRINGS_SET_AT};
    use super::*;

    #[test]
    fn destructor_counts_down_and_runs_the_base_destructor() {
        let mut e = logging_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(WORLD_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.map(PHANTOM_COUNT, 4);
        e.mem.set_u32(PHANTOM_COUNT, 2);
        let this = e.mem.alloc(0x18);
        e.call(0x00c9_f3b0, &args![this]);
        assert_eq!(e.mem.u32(this), PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(PHANTOM_COUNT), 1);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_DESTRUCT), vec![vec![this]]);
    }

    /// An object whose slots `0x94` and `0x98` answer `world_object` / `world`.
    fn object_with_worlds(e: &mut Engine, world_object: u32, world: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, 0x5555);
        e.map(0x0900_2000, 8);
        e.mem.set_u32(0x0900_2000, world_object);
        e.mem.set_u32(0x0900_2004, world);
        e.register(0x0900_0094, |e, _| Ret {
            eax: e.mem.u32(0x0900_2000),
            ..Ret::default()
        });
        e.register(0x0900_0098, |e, _| Ret {
            eax: e.mem.u32(0x0900_2004),
            ..Ret::default()
        });
        e.mem.set_u32(vtable + 0x94, 0x0900_0094);
        e.mem.set_u32(vtable + 0x98, 0x0900_0098);
        this
    }

    #[test]
    fn remove_asks_the_object_the_slot_answers() {
        let mut e = logging_engine();
        e.register(WORLD_REMOVE_PHANTOM, |_, _| Ret {
            eax: 0x101,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x360);
        e.mem.set_u32(holder + 0x350, 0x7777);
        let this = object_with_worlds(&mut e, holder, 0);
        assert!(e.call(0x00c9_f420, &args![this]).bool());
        assert_eq!(
            calls_to(&e, WORLD_REMOVE_PHANTOM),
            vec![vec![0x7777, 0x5555]]
        );
        e.mem.set_u32(holder + 0x350, 0);
        assert!(!e.call(0x00c9_f420, &args![this]).bool());
        e.mem.set_u32(0x0900_2000, 0);
        assert!(!e.call(0x00c9_f420, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE_PHANTOM).len(), 1);
    }

    #[test]
    fn update_c_filter_passes_the_phantom_and_one() {
        let mut e = logging_engine();
        e.register(WORLD_UPDATE_PHANTOM_FILTER, |_, _| Ret::default());
        let this = object_with_worlds(&mut e, 0, 0x8888);
        e.call(0x00c9_f460, &args![this]);
        assert_eq!(
            calls_to(&e, WORLD_UPDATE_PHANTOM_FILTER),
            vec![vec![0x8888, 0x5555, 1]]
        );
        e.mem.set_u32(0x0900_2004, 0);
        e.call(0x00c9_f460, &args![this]);
        assert_eq!(calls_to(&e, WORLD_UPDATE_PHANTOM_FILTER).len(), 1);
    }

    #[test]
    fn viewer_strings_append_the_class_name() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        e.register(WORLD_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.map(PHANTOM_RTTI, 4);
        e.mem.set_u32(PHANTOM_RTTI, 0x300);
        let this = e.mem.alloc(0x18);
        let strings = strings_header(&mut e, 1, 4, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 9);
        e.call(0x00c9_f480, &args![this, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 9);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let stores = calls_to(&e, STRINGS_SET_AT);
        assert_eq!(stores.len(), 1);
        assert_eq!(stores[0][..2], [strings, 1]);
    }
}

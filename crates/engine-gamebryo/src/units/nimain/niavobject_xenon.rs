//! `NiMain/niavobject_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiAVObject;

/// The collision object's virtual +0x90 (`__thiscall`, one word), which updates the world
/// data in place of the object's own update.
const COLLISION_UPDATE_WORLD_DATA_SLOT: u32 = 0x90;
/// `00a68bf0` (`__thiscall`, no words; `NiMain/run_00a68bf0`): the object's own world data
/// update.
const UPDATE_WORLD_DATA_BODY: u32 = 0x00a6_8bf0;

// Translated from 00a68c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateWorldData` (Xbox PDB; `__thiscall`, one word): with a collision object
/// (`m_spCollisionObject`, +0x1c) the work goes to its virtual +0x90 with the same word (a
/// tail jump); otherwise `00a68bf0` runs on the object (it takes no words).
pub fn ni_av_object_update_world_data(e: &mut Engine, this: Ptr<NiAVObject>, update: u32) {
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        e.vcall(collision, COLLISION_UPDATE_WORLD_DATA_SLOT, &args![update]);
        return;
    }
    e.call(UPDATE_WORLD_DATA_BODY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a68c60,
        ni_av_object_update_world_data(Ptr<NiAVObject>, u32)
    )]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const COLLISION_DOUBLE: u32 = 0x0070_2000;

    #[test]
    fn ni_av_object_update_world_data_prefers_the_collision_object() {
        let mut e = Engine::new();
        e.register(UPDATE_WORLD_DATA_BODY, |_, _| Ret::default());
        e.register(COLLISION_DOUBLE, |_, _| Ret::default());
        let object = e.mem.alloc(0x9c);
        e.call_log = Some(vec![]);
        e.call(0x00a68c60, &args![object, 7u32]);
        assert_eq!(calls_to(&e, UPDATE_WORLD_DATA_BODY), vec![vec![object]]);
        let collision = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(collision, vtable);
        e.mem
            .set_u32(vtable + COLLISION_UPDATE_WORLD_DATA_SLOT, COLLISION_DOUBLE);
        e.mem.set_u32(object + 0x1c, collision);
        e.call(0x00a68c60, &args![object, 7u32]);
        assert_eq!(calls_to(&e, COLLISION_DOUBLE), vec![vec![collision, 7]]);
        assert_eq!(calls_to(&e, UPDATE_WORLD_DATA_BODY).len(), 1);
    }
}

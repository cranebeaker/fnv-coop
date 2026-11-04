//! `fallout/ai/highactorculler.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The callees of `HighActorCuller::Operate`, all `thiscall` unless noted:
/// the object's node (`0043fcd0`), the actor test (`00552470`, `bool` in
/// `AL`), `NiAVObject::GetWorldBound` (`0043d450`, answers the 4-word
/// bound), the bound's centre (`006815c0`, answers `this`) and radius
/// (`0084d030`, the `float` at +0xC in ST0), the pointer getter
/// (`00559450`), the sphere's radius setter (`00819a50`, one `float`), the
/// game's interior cell (`005f36f0` on the global at `011dea10`) and its
/// list (`009d9f20`), the worldspace (`004fd3e0`) and its list
/// (`006698d0`), the list's first node (`00413f40`), the iterator step
/// (`0057cbe0`, `thiscall (list, &node)`, answers the address of the
/// node's item and moves to the next node), the item's word at +0x40
/// (`008d6f30`), the culling member's camera (`0084e3a0`) and its position
/// (`0045bb80`, answers 3 words), the occlusion test
/// (`BSMultiBoundNode::VisibleAgainstActiveOccluders`, `00c47150`, five
/// words: the position by value, the culling member, the item; `bool` in
/// `AL`) and the cull (`008d6f10`, with 1).
const OBJECT_NODE: u32 = 0x0043_fcd0;
const IS_SKIPPED: u32 = 0x0055_2470;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_CENTER: u32 = 0x0068_15c0;
const BOUND_RADIUS: u32 = 0x0084_d030;
const POINTER_GET: u32 = 0x0055_9450;
const SET_SPHERE_RADIUS: u32 = 0x0081_9a50;
const GAME: u32 = 0x011d_ea10;
const INTERIOR_CELL: u32 = 0x005f_36f0;
const CELL_LIST: u32 = 0x009d_9f20;
const WORLDSPACE: u32 = 0x004f_d3e0;
const WORLDSPACE_LIST: u32 = 0x0066_98d0;
const LIST_FIRST: u32 = 0x0041_3f40;
const ITERATOR_STEP: u32 = 0x0057_cbe0;
const ITEM_KIND: u32 = 0x008d_6f30;
const CULLING_CAMERA: u32 = 0x0084_e3a0;
const CAMERA_POSITION: u32 = 0x0045_bb80;
const VISIBLE_AGAINST_OCCLUDERS: u32 = 0x00c4_7150;
const CULL: u32 = 0x008d_6f10;

// Translated from 008d6d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighActorCuller::Operate` (Xbox PDB; `thiscall (object)`, `RET 4`):
/// with the object's node, the culling member at +8 is told of it (its
/// vtable +0x44); the node's vtable +0x10 answers the actor, which must
/// pass `00552470` as false. The sphere at +0xA0 is moved to the actor's
/// world bound (centre through its vtable +0xB8, radius through
/// `00819a50`). The list of the interior cell, or else of the worldspace,
/// is walked: every item whose word at +0x40 is 1 and against which the
/// node at +0x98 is not visible from the culling member's camera position
/// gets the actor culled (`008d6f10` with 1).
pub fn high_actor_culler_operate(e: &mut Engine, this: Ptr, object: Ptr) {
    let culler = this.addr();
    let node = e.call(OBJECT_NODE, &args![object]).u32();
    if node == 0 {
        return;
    }
    e.vcall(culler + 8, 0x44, &args![node]);
    let actor = e.vcall(node, 0x10, &args![]).u32();
    if actor == 0 || e.call(IS_SKIPPED, &args![actor]).bool() {
        return;
    }
    e.with_stack(0x10, |e, bound| {
        let bound = bound.addr();
        let world = e.call(GET_WORLD_BOUND, &args![actor]).u32();
        for i in 0..4 {
            let value = e.mem.u32(world + 4 * i);
            e.mem.set_u32(bound + 4 * i, value);
        }
        let sphere = e.call(POINTER_GET, &args![culler + 0xa0]).u32();
        let center = e.call(BOUND_CENTER, &args![bound]).u32();
        e.vcall(sphere, 0xb8, &args![center]);
        let radius = x87_store32(e.call(BOUND_RADIUS, &args![bound]).f64());
        let sphere = e.call(POINTER_GET, &args![culler + 0xa0]).u32();
        e.call(SET_SPHERE_RADIUS, &args![sphere, radius]);
    });
    let game = e.global::<u32>(GAME);
    let list = if e.call(INTERIOR_CELL, &args![game]).u32() != 0 {
        let game = e.global::<u32>(GAME);
        let cell = e.call(INTERIOR_CELL, &args![game]).u32();
        e.call(CELL_LIST, &args![cell]).u32()
    } else {
        let game = e.global::<u32>(GAME);
        if e.call(WORLDSPACE, &args![game]).u32() != 0 {
            let game = e.global::<u32>(GAME);
            let worldspace = e.call(WORLDSPACE, &args![game]).u32();
            e.call(WORLDSPACE_LIST, &args![worldspace]).u32()
        } else {
            0
        }
    };
    if list == 0 {
        return;
    }
    let first = e.call(LIST_FIRST, &args![list]).u32();
    e.with_stack(4, |e, cursor| {
        let cursor = cursor.addr();
        let start = e.call(POINTER_GET, &args![first]).u32();
        e.mem.set_u32(cursor, start);
        while e.mem.u32(cursor) != 0 {
            let slot = e.call(ITERATOR_STEP, &args![first, cursor]).u32();
            let item = e.mem.u32(slot);
            if item == 0 || e.call(ITEM_KIND, &args![item]).u32() != 1 {
                continue;
            }
            let camera = e.call(CULLING_CAMERA, &args![culler + 8]).u32();
            let position = e.call(CAMERA_POSITION, &args![camera]).u32();
            let point = [
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            ];
            let bound_node = e.call(POINTER_GET, &args![culler + 0x98]).u32();
            let visible = e
                .call(
                    VISIBLE_AGAINST_OCCLUDERS,
                    &args![bound_node, point[0], point[1], point[2], culler + 8, item],
                )
                .bool();
            if !visible {
                e.call(CULL, &args![actor, 1u32]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_6d40, high_actor_culler_operate(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A culler whose sub-object at +8, node, actor and sphere have
    /// vtables with the slots `Operate` uses; the list holds the items
    /// `items` (each item's kind is its own +0x40 word).
    fn culler_engine(interior: bool, items: &'static [u32]) -> (Engine, u32, u32) {
        const MEMBER_NOTIFY: u32 = 0x0300_0000;
        const NODE_ACTOR: u32 = 0x0300_0010;
        const SPHERE_CENTER: u32 = 0x0300_0020;
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0xc0);
        e.mem.set_u32(vtable + 0x44, MEMBER_NOTIFY);
        e.mem.set_u32(vtable + 0x10, NODE_ACTOR);
        e.mem.set_u32(vtable + 0xb8, SPHERE_CENTER);
        let culler = e.mem.alloc(0xa4);
        e.mem.set_u32(culler + 8, vtable);
        let node = e.mem.alloc(4);
        e.mem.set_u32(node, vtable);
        let sphere = e.mem.alloc(4);
        e.mem.set_u32(sphere, vtable);
        e.mem.set_u32(culler + 0xa0, sphere);
        e.mem.set_u32(culler + 0x98, 0x6600);
        let actor = 0x7700u32;
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 64.0);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 1.0);
        e.map(GAME, 4);
        e.set_global(GAME, 0x8800u32);
        // The list: a chain of nodes (next, item).
        let mut next = 0u32;
        for item in items.iter().rev() {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry, next);
            e.mem.set_u32(entry + 8, *item);
            next = entry;
        }
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, next);
        for (addr, answer) in [
            (MEMBER_NOTIFY, 0),
            (SPHERE_CENTER, 0),
            (SET_SPHERE_RADIUS, 0),
            (CULL, 0),
            (NODE_ACTOR, actor),
            (OBJECT_NODE, node),
            (GET_WORLD_BOUND, bound),
            (CAMERA_POSITION, position),
            (CULLING_CAMERA, 0x9900),
            (LIST_FIRST, list),
            (CELL_LIST, 0x4400),
            (WORLDSPACE_LIST, 0x4500),
            (INTERIOR_CELL, if interior { 0x4100 } else { 0 }),
            (WORLDSPACE, 0x4200),
        ] {
            e.register_double(addr, move |_, _| answer.into_ret());
        }
        e.register(IS_SKIPPED, |_, _| Ret::default());
        e.register(BOUND_CENTER, |_, a| a[0].into_ret());
        e.register(BOUND_RADIUS, |e, a| {
            f64::from(e.mem.f32(a[0] + 0xc)).into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ITERATOR_STEP, |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], e.mem.u32(node));
            (node + 8).into_ret()
        });
        // Items 0x10 and 0x30 are of kind 1; the node is hidden behind 0x30 only.
        e.register(ITEM_KIND, |_, a| u32::from(a[0] != 0x20).into_ret());
        e.register(VISIBLE_AGAINST_OCCLUDERS, |_, a| {
            u32::from(a[5] != 0x30).into_ret()
        });
        e.call_log = Some(vec![]);
        (e, culler, actor)
    }

    fn culler_calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn operate_culls_the_actor_behind_an_occluder() {
        let (mut e, culler, actor) = culler_engine(true, &[0x10, 0x20, 0x30]);
        e.call(0x008d_6d40, &args![culler, 0x5000u32]);
        assert_eq!(culler_calls(&e, CELL_LIST), vec![vec![0x4100]]);
        assert_eq!(culler_calls(&e, SET_SPHERE_RADIUS)[0][1], 64.0f32.to_bits());
        let tests = culler_calls(&e, VISIBLE_AGAINST_OCCLUDERS);
        assert_eq!(tests.len(), 2);
        assert_eq!(
            tests[0],
            vec![0x6600, 1.0f32.to_bits(), 0, 0, culler + 8, 0x10]
        );
        assert_eq!(culler_calls(&e, CULL), vec![vec![actor, 1]]);
    }

    #[test]
    fn operate_uses_the_worldspace_outside_and_stops_without_a_list() {
        let (mut e, culler, _) = culler_engine(false, &[0x30]);
        e.call(0x008d_6d40, &args![culler, 0x5000u32]);
        assert_eq!(culler_calls(&e, WORLDSPACE_LIST), vec![vec![0x4200]]);
        assert_eq!(culler_calls(&e, CULL).len(), 1);
        let (mut e, culler, _) = culler_engine(false, &[0x30]);
        e.register(WORLDSPACE, |_, _| Ret::default());
        e.call(0x008d_6d40, &args![culler, 0x5000u32]);
        assert!(culler_calls(&e, LIST_FIRST).is_empty());
    }

    #[test]
    fn operate_stops_without_a_node_or_for_a_skipped_actor() {
        let (mut e, culler, _) = culler_engine(true, &[0x30]);
        e.register(IS_SKIPPED, |_, _| 1u32.into_ret());
        e.call(0x008d_6d40, &args![culler, 0x5000u32]);
        assert!(culler_calls(&e, GET_WORLD_BOUND).is_empty());
        e.register(OBJECT_NODE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_6d40, &args![culler, 0x5000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

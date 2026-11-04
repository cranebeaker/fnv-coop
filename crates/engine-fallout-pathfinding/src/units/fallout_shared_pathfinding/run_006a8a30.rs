//! `fallout shared/pathfinding/run_006a8a30` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a8b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b6fc` (RTTI `.?AVNavMeshSearchClosePoint@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a7bc0`. One word, plain `RET`, returns nothing.
pub fn fn_006a8b40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b6fc);
    e.call(0x006a_7bc0, &args![this]);
}

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `NavMeshSearchClosePoint`'s vtable (`0106b6fc`).
const VTABLE_CLOSE_POINT: u32 = 0x0106_b6fc;
/// `006a7b10`: the base (`NavMeshSearch` part) constructor (`thiscall`).
const BASE_CONSTRUCT: u32 = 0x006a_7b10;
/// `006815c0`: returns `this` (the trivial constructor of the three
/// members at `+0x2074`, `+0x2080`, `+0x208c`).
const SELF: u32 = 0x0068_15c0;
/// `006d2c40`, `006d2c20`: two floats of the actor (`thiscall`, ST0).
const ACTOR_VALUE_006D2C40: u32 = 0x006d_2c40;
const ACTOR_VALUE_006D2C20: u32 = 0x006d_2c20;
/// `RandomFloat` (engine map), `cdecl`, two floats, ST0.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `0048d150`: `thiscall` on the actor, an object.
const ACTOR_OBJECT_0048D150: u32 = 0x0048_d150;
/// `005c3420`: `thiscall` on that object, one word (a point to fill).
const OBJECT_POINT_005C3420: u32 = 0x005c_3420;
/// The zero point copied to `+0x208c` (`011f426c`, three words).
const POINT_011F426C: u32 = 0x011f_426c;
/// `0069e390`: `thiscall` on the actor, an object or 0.
const ACTOR_OBJECT_0069E390: u32 = 0x0069_e390;
/// `006dc9f0`: `thiscall` on that object, two words (two points), ST0.
const OBJECT_VALUE_006DC9F0: u32 = 0x006d_c9f0;
/// `NavMesh::GetCenter` (engine map), `thiscall`: out point, triangle.
const NAV_MESH_GET_CENTER: u32 = 0x0055_8220;
/// `006a6eb0`: `thiscall` on the start node, one word (a point to fill);
/// returns a point.
const NODE_POINT_006A6EB0: u32 = 0x006a_6eb0;
/// `00439ef0`: `this - other` into `out` (`thiscall`: out, other).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990`: a point's length (`thiscall`, ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// `005330e0`: clamps the float at the address between two floats
/// (`cdecl`: address, low, high).
const CLAMP_FLOAT: u32 = 0x0053_30e0;
/// `006a94f0`: the search node for a nav mesh and triangle (`thiscall`).
const SEARCH_NODE: u32 = 0x006a_94f0;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::AddStartNode` and
/// `::Search` (engine map), `thiscall`, three words each.
const ADD_START_NODE: u32 = 0x006a_7c90;
const SEARCH: u32 = 0x006a_7d60;
/// `0076b610`: a `bool` of the search's result object (`thiscall`).
const RESULT_CHECK_0076B610: u32 = 0x0076_b610;

// Translated from 006a8a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: "constructs" `count` 0x14-byte elements at `elements`
/// (placement `new` of size 0x14 on each; the type has no constructor).
/// `this` is kept but not read.
pub fn fn_006a8a30(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        e.call(
            PLACEMENT_NEW,
            &args![0x14u32, index.wrapping_mul(0x14).wrapping_add(elements)],
        );
        index = index.wrapping_add(1);
    }
}

// Translated from 006a8a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a8a80 - the C++ exception frame (FS:[0]) is not translated
/// The `NavMeshSearchClosePoint` constructor (by the vtable it sets): the
/// base constructor, the vtable, the three members at `+0x2074`, `+0x2080`
/// and `+0x208c` (`006815c0`), and the word at `+0x2058` cleared. Returns
/// `this`.
pub fn fn_006a8a80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_CLOSE_POINT);
    e.call(SELF, &args![base + 0x2074]);
    e.call(SELF, &args![base + 0x2080]);
    e.call(SELF, &args![base + 0x208c]);
    e.mem.set_u32(base + 0x2058, 0);
    this
}

// Translated from 006a8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: starts a close-point search for `actor` from the nav mesh
/// location `start` (five words: nav mesh, triangle, ...): stores both, a
/// random float between the actor's `006d2c20` and `006d2c40` values at
/// `+0x2070`, the zero point at `+0x208c` and, when the actor has a
/// `0069e390` object, that object's `006dc9f0` value (else 0) at `+0x2098`;
/// the start node costs that value less the distance from the start
/// triangle's centre to `+0x208c` (`006a6eb0`), clamped to `0..value`.
/// Then searches (`(results, limit, 1)`) and returns 1 when `0076b610` of
/// `results` is false, else 0.
pub fn fn_006a8b60(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    start: Ptr,
    results: u32,
    limit: u32,
) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base + 0x2058, actor);
    for i in 0..5 {
        let word = e.mem.u32(start.addr() + 4 * i);
        e.mem.set_u32(base + 0x205c + 4 * i, word);
    }
    let high = e.call(ACTOR_VALUE_006D2C40, &args![actor]).f64();
    let high = x87_store32(high);
    let low = e.call(ACTOR_VALUE_006D2C20, &args![actor]).f64();
    let low = x87_store32(low);
    let random = e.call(RANDOM_FLOAT, &args![low, high]).f64();
    e.mem.set_u32(base + 0x2070, x87_store32(random));
    e.with_stack(0x48, |e, frame| {
        let frame = frame.addr();
        let node_point = frame;
        let actor_point = frame + 0xc;
        let location = frame + 0x18;
        let offset = frame + 0x2c;
        let cost = frame + 0x38;
        let center = frame + 0x3c;

        let object = e.call(ACTOR_OBJECT_0048D150, &args![actor]).u32();
        e.call(OBJECT_POINT_005C3420, &args![object, actor_point]);
        for i in 0..3 {
            let word = e.global::<u32>(POINT_011F426C + 4 * i);
            e.mem.set_u32(base + 0x208c + 4 * i, word);
        }
        e.mem.set_u32(base + 0x2098, 0);
        let owner = e.mem.u32(base + 0x2058);
        if e.call(ACTOR_OBJECT_0069E390, &args![owner]).u32() != 0 {
            let owner = e.mem.u32(base + 0x2058);
            let object = e.call(ACTOR_OBJECT_0069E390, &args![owner]).u32();
            let value = e
                .call(
                    OBJECT_VALUE_006DC9F0,
                    &args![object, actor_point, base + 0x208c],
                )
                .f64();
            e.mem.set_u32(base + 0x2098, x87_store32(value));
        }
        let mesh = e.mem.u32(start.addr());
        e.mem.set_u32(location, mesh);
        let triangle = e.mem.u16(start.addr() + 4);
        e.mem.set_u16(location + 4, triangle);
        e.call(
            NAV_MESH_GET_CENTER,
            &args![mesh, center, u32::from(triangle)],
        );
        for i in 0..3 {
            let word = x87_quiet_bits(e.mem.u32(center + 4 * i));
            e.mem.set_u32(location + 8 + 4 * i, word);
        }
        let point = e
            .call(NODE_POINT_006A6EB0, &args![location, node_point])
            .u32();
        e.call(POINT_SUBTRACT, &args![point, offset, base + 0x208c]);
        let distance = e.call(POINT_LENGTH, &args![offset]).f64();
        let remaining = x87_sub(x87_m32(e.mem.u32(base + 0x2098)), distance);
        e.mem.set_u32(cost, x87_store32(remaining));
        let limit_value = x87_quiet_bits(e.mem.u32(base + 0x2098));
        e.call(CLAMP_FLOAT, &args![cost, 0u32, limit_value]);
        let start_cost = x87_quiet_bits(e.mem.u32(cost));
        let mesh = e.mem.u32(location);
        let triangle = u32::from(e.mem.u16(location + 4));
        let node = e.call(SEARCH_NODE, &args![base, mesh, triangle]).u32();
        e.call(ADD_START_NODE, &args![base, node, location, start_cost]);
    });
    e.call(SEARCH, &args![base, results, limit, 1u32]);
    let found = e.call(RESULT_CHECK_0076B610, &args![results]).bool();
    u32::from(!found)
}

/// `00401030` (cdecl: block): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;

// Translated from 006a8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `NavMeshSearchClosePoint` (`thiscall`;
/// flags; `RET 4`): the destructor `006a8b40`, then the block is freed
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006a8b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006a8b40(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_8b40, fn_006a8b40(Ptr)),
        entry!(0x006a_8a30, fn_006a8a30(Ptr, u32, u32)),
        entry!(0x006a_8a80, fn_006a8a80(Ptr) -> Ptr),
        entry!(0x006a_8b60, fn_006a8b60(Ptr, u32, Ptr, u32, u32) -> u32),
        entry!(0x006a_8b10, fn_006a8b10(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006a8b40_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006a_7bc0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a_8b40, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_7bc0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b6fc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b6fc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn element_placement_and_constructor() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006a_8a30, &args![0u32, 0x5000u32, 2u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x14, 0x5000], vec![0x14, 0x5014]]
        );

        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(SELF, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x2100);
        e.mem.set_u32(this + 0x2058, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_8a80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CLOSE_POINT);
        assert_eq!(e.mem.u32(this + 0x2058), 0);
        assert_eq!(
            calls_to(&e, SELF),
            vec![
                vec![this + 0x2074],
                vec![this + 0x2080],
                vec![this + 0x208c]
            ]
        );
    }

    fn search_engine(e: &mut Engine, has_object: bool, found: bool) {
        e.register(ACTOR_VALUE_006D2C40, |_, _| st0(8.0));
        e.register(ACTOR_VALUE_006D2C20, |_, _| st0(2.0));
        e.register(RANDOM_FLOAT, |_, _| st0(5.0));
        e.register(ACTOR_OBJECT_0048D150, |_, _| 0x7000u32.into_ret());
        e.register(OBJECT_POINT_005C3420, |_, _| Ret::default());
        e.map(0x011f_4000, 0x1000);
        e.register_double(ACTOR_OBJECT_0069E390, move |_, _| {
            u32::from(has_object).wrapping_mul(0x7100).into_ret()
        });
        e.register(OBJECT_VALUE_006DC9F0, |_, _| st0(10.0));
        e.register(NAV_MESH_GET_CENTER, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            Ret::default()
        });
        e.register(NODE_POINT_006A6EB0, |_, a| (a[0] + 8).into_ret());
        e.register(POINT_SUBTRACT, |_, _| Ret::default());
        e.register(POINT_LENGTH, |_, _| st0(4.0));
        e.register(CLAMP_FLOAT, |e, a| {
            let value = e
                .mem
                .f32(a[0])
                .clamp(f32::from_bits(a[1]), f32::from_bits(a[2]));
            e.mem.set_f32(a[0], value);
            Ret::default()
        });
        e.register(SEARCH_NODE, |_, _| 0x7200u32.into_ret());
        e.register(ADD_START_NODE, |e, a| {
            // The start location: mesh, triangle, centre.
            assert_eq!(e.mem.u32(a[2]), 0x6000);
            assert_eq!(e.mem.u16(a[2] + 4), 9);
            assert_eq!(e.mem.f32(a[2] + 8), 1.0);
            assert_eq!(e.mem.f32(a[2] + 0x10), 3.0);
            Ret::default()
        });
        e.register(SEARCH, |_, _| Ret::default());
        e.register_double(RESULT_CHECK_0076B610, move |_, _| {
            u32::from(found).into_ret()
        });
    }

    #[test]
    fn search_from_the_start_triangle() {
        let mut e = Engine::new();
        search_engine(&mut e, true, false);
        let this = e.mem.alloc(0x2100);
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x6000);
        e.mem.set_u32(start + 4, 0x1234_0009);
        e.mem.set_u32(start + 0x10, 0x55);
        e.call_log = Some(vec![]);
        let r = e.call(0x006a_8b60, &args![this, 0x8000u32, start, 0x9000u32, 3u32]);
        assert_eq!(r.u32(), 1);
        assert_eq!(e.mem.u32(this + 0x2058), 0x8000);
        assert_eq!(e.mem.u32(this + 0x2060), 0x1234_0009);
        assert_eq!(e.mem.u32(this + 0x206c), 0x55);
        assert_eq!(e.mem.f32(this + 0x2070), 5.0);
        assert_eq!(
            calls_to(&e, RANDOM_FLOAT),
            vec![vec![2.0f32.to_bits(), 8.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(this + 0x2098), 10.0);
        let object = calls_to(&e, OBJECT_VALUE_006DC9F0);
        assert_eq!(object[0][0], 0x7100);
        assert_eq!(object[0][2], this + 0x208c);
        assert_eq!(calls_to(&e, OBJECT_POINT_005C3420)[0][1], object[0][1]);
        // 10 - 4 clamped to 0..10.
        let start_node = calls_to(&e, ADD_START_NODE);
        assert_eq!(start_node[0][1], 0x7200);
        assert_eq!(start_node[0][3], 6.0f32.to_bits());
        assert_eq!(calls_to(&e, SEARCH), vec![vec![this, 0x9000, 3, 1]]);
        assert_eq!(calls_to(&e, NAV_MESH_GET_CENTER)[0][2], 9);
    }

    #[test]
    fn search_without_an_object_clamps_to_zero() {
        let mut e = Engine::new();
        search_engine(&mut e, false, true);
        let this = e.mem.alloc(0x2100);
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x6000);
        e.mem.set_u32(start + 4, 9);
        e.call_log = Some(vec![]);
        let r = e.call(0x006a_8b60, &args![this, 0x8000u32, start, 0x9000u32, 3u32]);
        assert_eq!(r.u32(), 0);
        assert_eq!(e.mem.f32(this + 0x2098), 0.0);
        assert!(calls_to(&e, OBJECT_VALUE_006DC9F0).is_empty());
        assert_eq!(calls_to(&e, ADD_START_NODE)[0][3], 0);
    }

    #[test]
    fn at_006a8b10_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(0x006a_7bc0, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_8b10, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CLOSE_POINT);
        assert!(calls_to(&e, FREE_BLOCK).is_empty());
        assert_eq!(calls_to(&e, 0x006a_7bc0), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x006a_8b10, &args![this, 3u32]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![this]]);
    }
}

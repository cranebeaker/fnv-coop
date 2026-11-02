//! `BSHavok/bhkcharcontrollershape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor of the 0x10-byte serializable part (`00c87c30`).
const OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Constructor of the Havok convex vertices shape (`00d59bd0`, thiscall on the new block, three
/// stack words: the strided vertices record, the plane equations array, the radius).
const CONVEX_VERTICES_SHAPE_CONSTRUCT: u32 = 0x00d5_9bd0;
/// Reference count decrement of a Havok object (`00c905b0`; the engine map does not name it).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Vtables of the chain of base classes the constructor steps through and the object counters
/// it raises; the vtable of the shape and the vtable of its Havok shape.
const BASE_VTABLE_A: u32 = 0x0103_0bf4;
const BASE_VTABLE_B: u32 = 0x0103_0b0c;
const BASE_VTABLE_C: u32 = 0x0103_0a24;
const CONVEX_VERTICES_VTABLE: u32 = 0x010c_a924;
const CHAR_CONTROLLER_SHAPE_VTABLE: u32 = 0x010c_b4ac;
const CHAR_CONTROLLER_HAVOK_VTABLE: u32 = 0x010c_b45c;
const BASE_COUNT_A: u32 = 0x0126_80f8;
const BASE_COUNT_B: u32 = 0x0126_8274;
const BASE_COUNT_C: u32 = 0x0126_8250;
const CONVEX_VERTICES_COUNT: u32 = 0x0126_869c;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkCharControllerShape` (a `bhkConvexVerticesShape`, Xbox PDB, 0x14 bytes).
    pub struct BhkCharControllerShape: 0x14 {
        /// `phkObject` (Xbox PDB): the Havok shape.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB).
        0x0C pInfo: Ptr,
    }
}

layout! {
    /// `hkConvexVerticesShapeCinfo` (Xbox PDB, 0x20 bytes).
    pub struct HkConvexVerticesShapeCinfo: 0x20 {
        /// `m_userData` (Xbox PDB).
        0x00 m_userData: i32,
        /// `m_radius` (Xbox PDB).
        0x04 m_radius: f32,
        /// `m_vertices.m_data` (Xbox PDB).
        0x08 m_vertices: Ptr,
        /// `m_vertices.m_size` (Xbox PDB).
        0x0C m_numVertices: i32,
        /// `m_planeEquations.m_data` (Xbox PDB).
        0x14 m_planeEquations: Ptr,
    }
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

// Translated from 00cd7d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCharControllerShape::getAabb` (map name, taken from a vtable).
/// Thiscall, three words (`RET 0xc`), SSE (`f32` per lane). With the
/// transform column `+0x30` of `other` and of `this` summed into the centre
/// and `radius` added to the half extents at `this + 0x20`, writes
/// `centre - extents` to `out[0..4]` and `extents + centre` to `out[4..8]`.
pub fn hk_char_controller_shape_get_aabb(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    radius: f32,
    out: Ptr,
) {
    for lane in 0..4u32 {
        let centre =
            e.mem.f32(other.addr() + 0x30 + 4 * lane) + e.mem.f32(this.addr() + 0x30 + 4 * lane);
        let extent = radius + e.mem.f32(this.addr() + 0x20 + 4 * lane);
        e.mem.set_f32(out.addr() + 4 * lane, centre - extent);
        e.mem.set_f32(out.addr() + 0x10 + 4 * lane, extent + centre);
    }
}

// Translated from 00cd7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharControllerShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_86e8.
pub fn bhk_char_controller_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_86e8
}

// Translated from 00cd7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `this` and two stack words, reads none of them and answers 0x70 (the size of the
/// Havok shape [`bhk_char_controller_shape_init`] allocates). The engine map names it
/// `hkpConvexVerticesShape::calcSizeForSpu`, a name taken from a vtable; unconfirmed.
pub fn fn_00cd7d10(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> u32 {
    0x70
}

// Translated from 00cd7d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd7d60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCharControllerShape::Init` (engine map name, from a vtable), `(cinfo)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, allocates the 0x70-byte Havok shape from the memory router's
/// allocator (virtual slot 4) with its 16-bit size stored at `+4` and constructs it
/// (`00d59bd0`, the convex vertices shape constructor) on a local strided-vertices record
/// (the Cinfo's `m_vertices` data and size, stride 0x10), the Cinfo's `m_planeEquations` (at
/// `+0x14`) and the radius (`+4`). The shape gets the vtable `010cb45c`, goes to `SethkObject`
/// (virtual `+0x8c`), loses the construction's reference (`00c905b0`), and `PostInit` (virtual
/// `+0xbc`) runs with the Cinfo.
pub fn bhk_char_controller_shape_init(
    e: &mut Engine,
    this: Ptr<BhkCharControllerShape>,
    cinfo: Ptr<HkConvexVerticesShapeCinfo>,
) {
    let scope = enter_scope(e);
    let vertices = e.get(cinfo, HkConvexVerticesShapeCinfo::m_vertices).addr();
    let vertex_count = e.get(cinfo, HkConvexVerticesShapeCinfo::m_numVertices);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x70u32]).u32();
    e.mem.set_u16(block + 4, 0x70);
    let radius = x87_quiet_bits(e.mem.u32(cinfo.addr() + 4));
    e.with_stack(0xc, |e, strided| {
        e.mem.set_u32(strided.addr(), vertices);
        e.mem.set_u32(strided.addr() + 4, vertex_count as u32);
        e.mem.set_u32(strided.addr() + 8, 0x10);
        e.call(
            CONVEX_VERTICES_SHAPE_CONSTRUCT,
            &args![block, strided, cinfo.addr() + 0x14, radius],
        );
    });
    e.mem.set_u32(block, CHAR_CONTROLLER_HAVOK_VTABLE);
    e.vcall(this.addr(), 0x8c, &args![block]);
    e.call(REMOVE_REFERENCE, &args![block]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, scope);
}

// Translated from 00cd7e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd7e80 - the FS:[0] exception-unwinding frame is not modeled.
/// Constructs the shape at `this` from a Cinfo (unconfirmed name; the Xbox PDB has a
/// `bhkCharControllerShape(hkConvexVerticesShapeCinfo const&)` constructor): the base
/// constructor (`00c87c30`), then each base class's vtable stored and its object counter
/// raised (`012680f8`, `01268274`, `01268250`, `0126869c`), `+0xc` and `+0x10` cleared, the
/// final vtable `010cb4ac`, [`bhk_char_controller_shape_init`] on the Cinfo and the counter
/// `0126869c` raised once more. Answers `this`.
pub fn fn_00cd7e80(
    e: &mut Engine,
    this: Ptr<BhkCharControllerShape>,
    cinfo: Ptr<HkConvexVerticesShapeCinfo>,
) -> Ptr {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in [
        (BASE_VTABLE_A, BASE_COUNT_A),
        (BASE_VTABLE_B, BASE_COUNT_B),
        (BASE_VTABLE_C, BASE_COUNT_C),
        (CONVEX_VERTICES_VTABLE, CONVEX_VERTICES_COUNT),
    ] {
        e.mem.set_u32(this.addr(), vtable);
        let count = e.mem.u32(counter);
        e.mem.set_u32(counter, count.wrapping_add(1));
    }
    e.mem.set_u32(this.addr(), CHAR_CONTROLLER_SHAPE_VTABLE);
    bhk_char_controller_shape_init(e, this, cinfo);
    let count = e.mem.u32(CONVEX_VERTICES_COUNT);
    e.mem.set_u32(CONVEX_VERTICES_COUNT, count.wrapping_add(1));
    this.cast()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd7d10, fn_00cd7d10(Ptr, u32, u32) -> u32),
        entry!(
            0x00cd7d60,
            bhk_char_controller_shape_init(
                Ptr<BhkCharControllerShape>,
                Ptr<HkConvexVerticesShapeCinfo>,
            )
        ),
        entry!(
            0x00cd7e80,
            fn_00cd7e80(Ptr<BhkCharControllerShape>, Ptr<HkConvexVerticesShapeCinfo>) -> Ptr
        ),
        entry!(0x00cd7f20, bhk_char_controller_shape_get_r_t_t_i() -> u32),
        entry!(
            0x00cd7d20,
            hk_char_controller_shape_get_aabb(Ptr, Ptr, f32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOC_SLOT: u32 = 0x7000_0004;
    const SET_HK_OBJECT_SLOT: u32 = 0x7000_008c;
    const POST_INIT_SLOT: u32 = 0x7000_00bc;
    const SHAPE_VTABLE: u32 = 0x7001_1000;
    const ALLOCATOR_VTABLE: u32 = 0x7001_2000;
    const ROUTER_OBJECT: u32 = 0x7100_0000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// A vtable with the slots `SethkObject` (`+0x8c`) and `PostInit` (`+0xbc`).
    fn put_shape_vtable(e: &mut Engine, address: u32) {
        let mut table = vec![0u32; 0xc0 / 4];
        table[0x8c / 4] = SET_HK_OBJECT_SLOT;
        table[0xbc / 4] = POST_INIT_SLOT;
        e.put_vtable(address, &table);
    }

    /// A shape object with that vtable, and the memory router whose allocator (virtual slot
    /// 4) hands out zeroed blocks.
    fn shape_with_allocator(e: &mut Engine) -> u32 {
        e.put_vtable(ALLOCATOR_VTABLE, &[0, ALLOC_SLOT]);
        put_shape_vtable(e, SHAPE_VTABLE);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, ALLOCATOR_VTABLE);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        e.register(POST_INIT_SLOT, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, SHAPE_VTABLE);
        this
    }

    #[test]
    fn fn_00cd7d10_answers_0x70() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cd_7d10, &args![1u32, 2u32, 3u32]).u32(), 0x70);
    }

    #[test]
    fn bhk_char_controller_shape_init_constructs_the_havok_shape_from_the_cinfo() {
        let mut e = logged_engine();
        let this = shape_with_allocator(&mut e);
        // The constructor double keeps the strided record's three words in the block.
        e.register(CONVEX_VERTICES_SHAPE_CONSTRUCT, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 0x10 + 4 * word, value);
            }
            Ret::default()
        });
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, 0x7f80_0001);
        e.mem.set_u32(cinfo + 8, 0x6000);
        e.mem.set_u32(cinfo + 0xc, 5);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cd_7d60, &args![this, cinfo]);
        let constructs = calls_to(&e, CONVEX_VERTICES_SHAPE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(constructs[0][2], cinfo + 0x14);
        // The radius is passed as the quieted bit pattern.
        assert_eq!(constructs[0][3], 0x7fc0_0001);
        assert_eq!(e.mem.u16(block + 4), 0x70);
        assert_eq!(
            calls_to(&e, ALLOC_SLOT),
            vec![vec![ROUTER_OBJECT + 0x20, 0x70]]
        );
        let strided: Vec<u32> = (0..3)
            .map(|word| e.mem.u32(block + 0x10 + 4 * word))
            .collect();
        assert_eq!(strided, vec![0x6000, 5, 0x10]);
        assert_eq!(e.mem.u32(block), CHAR_CONTROLLER_HAVOK_VTABLE);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, cinfo]]);
        let order = vec![
            CONVEX_VERTICES_SHAPE_CONSTRUCT,
            SET_HK_OBJECT_SLOT,
            REMOVE_REFERENCE,
            POST_INIT_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn fn_00cd7e80_constructs_through_the_base_classes_and_initialises() {
        let mut e = logged_engine();
        let this = shape_with_allocator(&mut e);
        e.mem.set_u32(this, 0);
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(CONVEX_VERTICES_SHAPE_CONSTRUCT, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        for (address, value) in [
            (BASE_COUNT_A, 1),
            (BASE_COUNT_B, 2),
            (BASE_COUNT_C, 3),
            (CONVEX_VERTICES_COUNT, 4),
        ] {
            e.mem.set_u32(address, value);
        }
        // The initialisation calls the virtual slots through the vtable the constructor
        // stored last.
        put_shape_vtable(&mut e, CHAR_CONTROLLER_SHAPE_VTABLE);
        let cinfo = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cd_7e80, &args![this, cinfo]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), CHAR_CONTROLLER_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(BASE_COUNT_A), 2);
        assert_eq!(e.mem.u32(BASE_COUNT_B), 3);
        assert_eq!(e.mem.u32(BASE_COUNT_C), 4);
        // Raised after the convex vertices vtable and again after the initialisation.
        assert_eq!(e.mem.u32(CONVEX_VERTICES_COUNT), 6);
        assert_eq!(calls_to(&e, CONVEX_VERTICES_SHAPE_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, cinfo]]);
        let order = vec![
            OBJECT_CONSTRUCT,
            CONVEX_VERTICES_SHAPE_CONSTRUCT,
            SET_HK_OBJECT_SLOT,
            POST_INIT_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
    }

    #[test]
    fn hk_char_controller_shape_get_aabb_writes_min_then_max() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        for lane in 0..4u32 {
            e.mem
                .set_f32(this + 0x30 + 4 * lane, 10.0 * (lane + 1) as f32);
            e.mem.set_f32(other + 0x30 + 4 * lane, 1.0);
            e.mem.set_f32(this + 0x20 + 4 * lane, lane as f32);
        }
        e.call(
            0x00cd_7d20,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(other),
                0.5f32,
                Ptr::<()>::new(out)
            ],
        );
        // Centres 11, 21, 31, 41; extents 0.5, 1.5, 2.5, 3.5.
        let read = |e: &Engine, offset: u32| e.mem.f32(out + offset);
        assert_eq!([0, 4, 8, 12].map(|o| read(&e, o)), [10.5, 19.5, 28.5, 37.5]);
        assert_eq!(
            [16, 20, 24, 28].map(|o| read(&e, o)),
            [11.5, 22.5, 33.5, 44.5]
        );
    }

    #[test]
    fn bhk_char_controller_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cd_7f20, &args![]).u32(), 0x0126_86e8);
    }
}

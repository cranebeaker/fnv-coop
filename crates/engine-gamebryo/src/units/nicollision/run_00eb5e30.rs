//! `NiCollision/run_00eb5e30` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiIntersector` constructor (`00eae720`, thiscall: maximum time, derivative time step,
/// pseudodistance threshold, iteration limit).
const NI_INTERSECTOR_CONSTRUCTOR: u32 = 0x00ea_e720;
/// `NiSphereTriIntersector`'s vtable (`010f1b2c`, by its RTTI).
const NI_SPHERE_TRI_INTERSECTOR_VTABLE: u32 = 0x010f_1b2c;
/// The `float` at `010f1b24` (`FLT_MAX` in the executable) both parameters start at.
const INITIAL_PARAMETER_010F1B24: u32 = 0x010f_1b24;
/// `NiOBBRoot`'s vtable (`010f1b44`, by its RTTI).
const NI_OBB_ROOT_VTABLE: u32 = 0x010f_1b44;
/// The memory manager instance and its `Allocate` (`00aa3e40`, thiscall: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The `NiOBBNode` constructor (`00eba1b0`, thiscall: the root, the depth word and four more
/// words) and the `NiOBBLeaf` constructor (`00eb6b90`, thiscall: the root and four words).
const NI_OBB_NODE_CONSTRUCTOR: u32 = 0x00eb_a1b0;
const NI_OBB_LEAF_CONSTRUCTOR: u32 = 0x00eb_6b90;
/// `NiTransform::MakeIdentity` (`00a86bf0`, thiscall).
const NI_TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
/// The three words at `011f426c` (zero in the image): the starting velocity.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;

// Translated from 00eb5e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiSphereTriIntersector` constructor (vtable `010f1b2c` by its
/// RTTI, `RET 0x20`): the base constructor with the four numbers (the floats through
/// `FLD`/`FSTP`), the sphere volume (+0x38), the triangle `m_kTri` (+0x3c: the first vertex, then
/// the edges `v1 - v0` and `v2 - v0` stored as floats), `m_fInvRadiusSqr` (+0x60) `1 / r^2` of the
/// sphere's radius (+0x10) and both parameters (+0x64, +0x68) the float at `010f1b24`. Answers
/// `this`.
pub fn fn_00eb5e30(
    e: &mut Engine,
    this: Ptr,
    sphere: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    max_time: f32,
    derivative_time_step: f32,
    pseudodistance_threshold: f32,
    max_iterations: i32,
) -> Ptr {
    let base = this.addr();
    let quiet = |value: f32| x87_quiet_bits(value.to_bits());
    e.call(
        NI_INTERSECTOR_CONSTRUCTOR,
        &args![
            this,
            quiet(max_time),
            quiet(derivative_time_step),
            quiet(pseudodistance_threshold),
            max_iterations
        ],
    );
    e.mem.set_u32(base, NI_SPHERE_TRI_INTERSECTOR_VTABLE);
    e.mem.set_u32(base + 0x38, sphere.addr());
    let origin = vertex0.addr();
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(origin.wrapping_add(offset));
        e.mem.set_u32(base + 0x3c + offset, word);
    }
    for (edge, vertex) in [(0x48u32, vertex1.addr()), (0x54, vertex2.addr())] {
        for offset in [0u32, 4, 8] {
            let word = x87_store32(x87_sub(
                x87_load32(e.mem.u32(vertex.wrapping_add(offset))),
                x87_m32(e.mem.u32(origin.wrapping_add(offset))),
            ));
            e.mem.set_u32(base + edge + offset, word);
        }
    }
    let radius = x87_load32(e.mem.u32(sphere.addr().wrapping_add(0x10)));
    e.mem.set_u32(
        base + 0x60,
        x87_store32(x87_div(1.0, x87_mul(radius, radius))),
    );
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER_010F1B24));
    e.mem.set_u32(base + 0x64, parameter);
    e.mem.set_u32(base + 0x68, parameter);
    this
}

// Translated from 00eb5f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiOBBRoot` constructor (vtable `010f1b44` by its RTTI,
/// `RET 0x14`). For a depth (the low 16 bits of `depth`) above 1 the tree `m_pTree` (+8) is a new
/// 0x8c-byte `NiOBBNode` (`00eba1b0` with this root, the whole depth word and the four words);
/// otherwise a new 0x98-byte `NiOBBLeaf` (`00eb6b90` with this root, the first three words and 0).
/// A failed allocation leaves the tree null. Then `m_fDeltaTime` (+4) is 0.0, `m_velocity`
/// (+0x10) the three words at `011f426c`, the transform (+0x1c) the identity
/// (`NiTransform::MakeIdentity`), `m_bTransformChanged` (+0xc) false and `m_uiChangeStamp` (+0x50)
/// 0. Answers `this`.
pub fn fn_00eb5f10(
    e: &mut Engine,
    this: Ptr,
    depth: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_OBB_ROOT_VTABLE);
    let tree = if depth as u16 > 1 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x8cu32])
            .u32();
        if block == 0 {
            0
        } else {
            e.call(
                NI_OBB_NODE_CONSTRUCTOR,
                &args![block, this, depth, first, second, third, fourth],
            )
            .u32()
        }
    } else {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x98u32])
            .u32();
        if block == 0 {
            0
        } else {
            e.call(
                NI_OBB_LEAF_CONSTRUCTOR,
                &args![block, this, first, second, third, 0u32],
            )
            .u32()
        }
    };
    e.mem.set_u32(base + 8, tree);
    e.mem.set_u32(base + 4, 0);
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(ZERO_POINT_011F426C + offset);
        e.mem.set_u32(base + 0x10 + offset, word);
    }
    e.call(NI_TRANSFORM_MAKE_IDENTITY, &args![base + 0x1c]);
    e.mem.set_u8(base + 0xc, 0);
    e.mem.set_u32(base + 0x50, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb_5e30,
            fn_00eb5e30(Ptr, Ptr, Ptr, Ptr, Ptr, f32, f32, f32, i32) -> Ptr
        ),
        entry!(
            0x00eb_5f10,
            fn_00eb5f10(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn point(e: &mut Engine, v: [f32; 3]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *c);
        }
        p
    }

    #[test]
    fn sphere_tri_constructor_keeps_the_triangle() {
        let mut e = Engine::new();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER_010F1B24, 0x7f7f_ffff);
        e.register(NI_INTERSECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let sphere = e.mem.alloc(0x14);
        e.mem.set_f32(sphere + 0x10, 0.5);
        let v0 = point(&mut e, [0.0, 0.0, 1.0]);
        let v1 = point(&mut e, [1.0, 0.0, 1.0]);
        let v2 = point(&mut e, [0.0, 2.0, 1.0]);
        let this = e.mem.alloc(0x6c);
        let r = e.call(
            0x00eb_5e30,
            &args![this, sphere, v0, v1, v2, 1.0f32, 0.001f32, 0.0001f32, 5i32],
        );
        assert_eq!(r.u32(), this);
        let words: Vec<f32> = (0..9).map(|i| e.mem.f32(this + 0x3c + 4 * i)).collect();
        assert_eq!(words, vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0]);
        assert_eq!(e.mem.u32(this), NI_SPHERE_TRI_INTERSECTOR_VTABLE);
        assert_eq!(e.mem.f32(this + 0x60), 4.0);
        assert_eq!(e.mem.u32(this + 0x68), 0x7f7f_ffff);
    }

    #[test]
    fn obb_root_builds_a_node_or_a_leaf() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let blocks = [e.mem.alloc(0x8c), e.mem.alloc(0x98)];
        let mut next = 0usize;
        e.register_double(MEMORY_ALLOCATE, move |_, _| {
            next += 1;
            blocks[(next - 1) % 2].into_ret()
        });
        e.register(NI_OBB_NODE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(NI_OBB_LEAF_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(NI_TRANSFORM_MAKE_IDENTITY, |_, _| Ret::default());
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x50, 7);
        e.mem.set_u8(this + 0xc, 1);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00eb_5f10,
            &args![this, 0x0001_0003u32, 1u32, 2u32, 3u32, 4u32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x8c]]
        );
        assert_eq!(
            calls_to(&e, NI_OBB_NODE_CONSTRUCTOR),
            vec![vec![blocks[0], this, 0x0001_0003, 1, 2, 3, 4]]
        );
        assert_eq!(e.mem.u32(this), NI_OBB_ROOT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), blocks[0]);
        assert_eq!(e.mem.u8(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(
            calls_to(&e, NI_TRANSFORM_MAKE_IDENTITY),
            vec![vec![this + 0x1c]]
        );
        // Depth 1 (the high bits ignored): a leaf with the first three words and 0.
        e.call_log = Some(vec![]);
        e.call(
            0x00eb_5f10,
            &args![this, 0x0005_0001u32, 1u32, 2u32, 3u32, 4u32],
        );
        assert_eq!(
            calls_to(&e, NI_OBB_LEAF_CONSTRUCTOR),
            vec![vec![blocks[1], this, 1, 2, 3, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), blocks[1]);
        // A failed allocation leaves no tree.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00eb_5f10, &args![this, 2u32, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}

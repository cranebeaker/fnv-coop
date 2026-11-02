//! `BSMain/bsportal.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c4f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalSharedNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c4f550` and frees 0xc4 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_portal_shared_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c4_f550, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xc4u32]);
    }
    this
}

// Translated from 00c4f8d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012030b8 without reading the receiver. The map names this body `BSPortalSharedNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c4f8d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_30b8
}

/// `00a5ecb0` (`thiscall`, one word; unit `ninode.cpp`, the `NiNode` constructor shape).
const NODE_CONSTRUCT_00A5ECB0: u32 = 0x00a5_ecb0;
/// `BSPortalSharedNode`'s vtable (RTTI `.?AVBSPortalSharedNode@@`).
const PORTAL_SHARED_NODE_VTABLE: u32 = 0x010c_21fc;
/// `BSSimpleArray<ShadowSceneLight*,1024>::_InitialSetup` (engine map, `00c3a220`;
/// `thiscall`, no stack words).
const LIGHT_ARRAY_CONSTRUCT: u32 = 0x00c3_a220;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map, `00c47660`; `thiscall`, no stack
/// words) and the frustum's size.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
const COMPOUND_FRUSTUM_SIZE: u32 = 0xa4;
/// `BSPortalSharedNode` (Xbox PDB) on PC, 0x24 lower than the PDB (as `m_kChildren`, PDB
/// +0xC0, is at +0x9C): `pCompoundFrustum` +0xAC, `kLights` +0xB0, `pPortal` +0xC0;
/// `NiAVObject::m_uFlags` at +0x30.
const COMPOUND_FRUSTUM: u32 = 0xac;
const LIGHTS: u32 = 0xb0;
const PORTAL: u32 = 0xc0;
const AV_OBJECT_FLAGS: u32 = 0x30;

// Translated from 00c4f460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4f460 - the compiler's exception frame (FS:[0] chain, handler 00f2eb9e, states 0 to 2 that free the frustum block when its constructor throws) is not translated
/// `BSPortalSharedNode::BSPortalSharedNode` (engine map; `thiscall`, one word: the portal;
/// `RET 4`): the node base (`00a5ecb0` with 0), the vtable, the light array (`00c3a220`), a
/// new 0xA4-byte `BSCompoundFrustum` from the memory manager (null when the allocation
/// fails), flag `0x800` of `m_uFlags` set (the compiler kept a test of the constant 1; its
/// clearing branch never runs) and `pPortal`. Returns `this`.
pub fn bs_portal_shared_node_bs_portal_shared_node(e: &mut Engine, this: Ptr, portal: u32) -> Ptr {
    let base = this.addr();
    e.call(NODE_CONSTRUCT_00A5ECB0, &args![this, 0u32]);
    e.mem.set_u32(base, PORTAL_SHARED_NODE_VTABLE);
    e.call(LIGHT_ARRAY_CONSTRUCT, &args![base + LIGHTS]);
    let block = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, COMPOUND_FRUSTUM_SIZE],
        )
        .u32();
    let frustum = if block != 0 {
        e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + COMPOUND_FRUSTUM, frustum);
    // `MOV ECX,1; TEST ECX,ECX; JZ`: the set branch always runs; the clear branch is the
    // exe's unreachable one (`black_box` keeps it as written).
    let flags = e.mem.u32(base + AV_OBJECT_FLAGS);
    if std::hint::black_box(1u32) != 0 {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, flags | 0x800);
    } else {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, !0x800 & flags);
    }
    e.mem.set_u32(base + PORTAL, portal);
    this
}

/// `00aa13e0` (`cdecl`, one word): allocates; the heap free (`00aa4060`, `thiscall` on the
/// memory manager, one word).
const ALLOCATE: u32 = 0x00aa_13e0;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSPortal`'s size and vtable (`0101e82c`, RTTI `.?AVBSPortal@@`), the
/// `BSOcclusionPlane` constructor (`00c335d0`, `thiscall`, no stack words) and the
/// `NiPointer` assignment `0066b0d0` (`thiscall`, the new pointer); `BSPortal`'s words at
/// +0xfc and +0x100 (the two rooms it joins, by `00c4f7b0`'s use) and its `NiPointer` at
/// +0x104.
const PORTAL_SIZE: u32 = 0x108;
const PORTAL_VTABLE: u32 = 0x0101_e82c;
const OCCLUSION_PLANE_CONSTRUCT: u32 = 0x00c3_35d0;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const PORTAL_SIDE_A: u32 = 0xfc;
const PORTAL_SIDE_B: u32 = 0x100;
const PORTAL_POINTER: u32 = 0x104;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The shared node's size, `BSCompoundFrustum::~BSCompoundFrustum` (`00c47770`), `00b5af80`
/// (`thiscall` on the light array, one word), the light array's destructor body (`00c3a620`,
/// `bsmultiboundroom.rs`) and the `NiNode` destructor body (`00a5e930`); all `thiscall`.
const SHARED_NODE_SIZE: u32 = 0xc4;
const COMPOUND_FRUSTUM_DESTRUCT: u32 = 0x00c4_7770;
const LIGHT_ARRAY_00B5AF80: u32 = 0x00b5_af80;
const LIGHT_ARRAY_DESTRUCTOR: u32 = 0x00c3_a620;
const NI_NODE_DESTRUCTOR: u32 = 0x00a5_e930;
/// `BSCompoundFrustum::PrethreadOpList` (`00c49850`, `thiscall`) and `NiAVObject::Cull`
/// (`00a59e00`, `thiscall`, one word); the node's children (`m_kChildren` at +0x9c: the
/// elements at +0xa0, the size at +0xa6, the used count at +0xa8) and the culler's frustum
/// word (+0xc0).
const COMPOUND_FRUSTUM_PRETHREAD: u32 = 0x00c4_9850;
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;
const CHILDREN: u32 = 0x9c;
const CHILDREN_USED: u32 = 0xa8;
const CULLER_FRUSTUM: u32 = 0xc0;
/// `BSMultiBoundNode::spMultiBound` (+0xac on PC).
const MULTI_BOUND: u32 = 0xac;

// Translation incomplete: 00c4f390 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the block if the constructor throws, with its /GS cookie word) is not translated.
// Translated from 00c4f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortal::CreateClone` (Xbox PDB; `thiscall`, one word: the cloning process): 0x108
/// bytes from `00aa13e0` constructed by `BSPortal::BSPortal` (null when the allocation
/// fails), then `CopyMembers(clone, cloning)`. Answers the clone.
pub fn bs_portal_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![PORTAL_SIZE]).u32();
    let clone = if block != 0 {
        bs_portal_bs_portal(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    bs_portal_copy_members(e, this, clone, cloning);
    clone
}

// Translated from 00c4f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortal::CopyMembers` (Xbox PDB; `thiscall`, two words, the cloning process not read):
/// copies the words at +0xfc and +0x100 into the clone.
pub fn bs_portal_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, _unused_2: Ptr) {
    let a = e.mem.u32(this.addr() + PORTAL_SIDE_A);
    e.mem.set_u32(clone.addr().wrapping_add(PORTAL_SIDE_A), a);
    let b = e.mem.u32(this.addr() + PORTAL_SIDE_B);
    e.mem.set_u32(clone.addr().wrapping_add(PORTAL_SIDE_B), b);
}

// Translation incomplete: 00c4f550 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1, with its /GS cookie word) is not translated.
// Translated from 00c4f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSPortalSharedNode` destructor body (`thiscall`, no stack
/// words). The vtable (`010c21fc`); a set `pCompoundFrustum` (+0xac) destroyed
/// (`00c47770`) and given back to the memory manager (the compiler kept a test of the
/// constant 1 around the free; it always passes); `00b5af80(1)` on the light array (+0xb0);
/// `pPortal` (+0xc0) cleared; the light array's destructor body (`00c3a620`); the `NiNode`
/// destructor body (`00a5e930`).
pub fn fn_00c4f550(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PORTAL_SHARED_NODE_VTABLE);
    let frustum = e.mem.u32(base + COMPOUND_FRUSTUM);
    if frustum != 0 {
        e.call(COMPOUND_FRUSTUM_DESTRUCT, &args![frustum]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, frustum]);
    }
    e.call(LIGHT_ARRAY_00B5AF80, &args![base + LIGHTS, 1u32]);
    e.mem.set_u32(base + PORTAL, 0);
    e.call(LIGHT_ARRAY_DESTRUCTOR, &args![base + LIGHTS]);
    e.call(NI_NODE_DESTRUCTOR, &args![this]);
}

// Translation incomplete: 00c4f620 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the block if the constructor throws, with its /GS cookie word) is not translated.
// Translated from 00c4f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalSharedNode::CreateClone` (Xbox PDB; `thiscall`, one word: the cloning process):
/// 0xc4 bytes from `00aa13e0` constructed by `BSPortalSharedNode::BSPortalSharedNode` with
/// this node's `pPortal` (null when the allocation fails), then `00c4f6c0(clone, cloning)`.
/// Answers the clone.
pub fn bs_portal_shared_node_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![SHARED_NODE_SIZE]).u32();
    let clone = if block != 0 {
        let portal = e.mem.u32(this.addr() + PORTAL);
        bs_portal_shared_node_bs_portal_shared_node(e, Ptr::new(block), portal)
    } else {
        Ptr::NULL
    };
    fn_00c4f6c0(e, this, clone, cloning);
    clone
}

// Translated from 00c4f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shared node's `CopyMembers` call target (`thiscall`, two words, neither read;
/// the engine map's name, a `std::vector` member, comes from identical-code folding): does
/// nothing.
pub fn fn_00c4f6c0(_e: &mut Engine, _this: Ptr, _unused_1: Ptr, _unused_2: Ptr) {}

// Translated from 00c4f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalSharedNode::OnVisible` (Xbox PDB; `thiscall`, one word: the culling process):
/// nothing without used children (+0xa8). Otherwise the culler's frustum word (+0xc0) is
/// kept and replaced by `pCompoundFrustum` (+0xac; `PrethreadOpList` run on it when set),
/// every child (`m_kChildren`, the size read once) is culled (`NiAVObject::Cull`), and the
/// frustum word is restored.
pub fn bs_portal_shared_node_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let node = this.addr();
    let process = culler.addr();
    if e.mem.u16(node + CHILDREN_USED) == 0 {
        return;
    }
    let saved = e.mem.u32(process + CULLER_FRUSTUM);
    let frustum = e.mem.u32(node + COMPOUND_FRUSTUM);
    e.mem.set_u32(process + CULLER_FRUSTUM, frustum);
    if e.mem.u32(node + COMPOUND_FRUSTUM) != 0 {
        let frustum = e.mem.u32(node + COMPOUND_FRUSTUM);
        e.call(COMPOUND_FRUSTUM_PRETHREAD, &args![frustum]);
    }
    let children = node + CHILDREN;
    let count = u32::from(e.mem.u16(children + 0xa));
    for index in 0..count {
        let child = e.mem.u32(e.mem.u32(children + 4) + 4 * index);
        if child != 0 {
            e.call(NI_AV_OBJECT_CULL, &args![child, culler]);
        }
    }
    e.mem.set_u32(process + CULLER_FRUSTUM, saved);
}

// Translated from 00c4f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsportal.obj`; `thiscall`, no stack words): 1 when
/// either of the portal's words at +0xfc and +0x100 is set and `00c4f860` answers 1 for it,
/// else 0.
pub fn fn_00c4f7b0(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = 0;
    let a = e.mem.u32(this.addr() + PORTAL_SIDE_A);
    if a != 0 {
        result = u32::from(fn_00c4f860(e, Ptr::new(a)) == 1);
    }
    let b = e.mem.u32(this.addr() + PORTAL_SIDE_B);
    if b != 0 {
        result |= u32::from(fn_00c4f860(e, Ptr::new(b)) == 1);
    }
    result
}

// Translated from 00c4f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsportal.obj`; `thiscall`, no stack words): the state
/// (+8) of the multi-bound's shape (`spMultiBound` +0xac, then +0xc), or 0 when either is
/// null.
pub fn fn_00c4f860(e: &mut Engine, this: Ptr) -> u32 {
    let multi_bound = e.mem.u32(this.addr() + MULTI_BOUND);
    if multi_bound == 0 {
        return 0;
    }
    let shape = e.mem.u32(e.mem.u32(this.addr() + MULTI_BOUND) + 0xc);
    if shape == 0 {
        return 0;
    }
    let shape = e.mem.u32(e.mem.u32(this.addr() + MULTI_BOUND) + 0xc);
    e.mem.u32(shape + 8)
}

// Translation incomplete: 00c4f8e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1, with its /GS cookie word) is not translated.
// Translated from 00c4f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortal::BSPortal` (Xbox PDB; `thiscall`, no stack words): the `BSOcclusionPlane` base
/// (`00c335d0`), the vtable (`0101e82c`), the `NiPointer` at +0x104 constructed as null (the
/// inlined constructor's increment of a non-null pointer reads the word just written, so it
/// never runs) and assigned null (`0066b0d0`). Answers `this`.
pub fn bs_portal_bs_portal(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(OCCLUSION_PLANE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, PORTAL_VTABLE);
    let pointer = base.wrapping_add(PORTAL_POINTER);
    e.mem.set_u32(pointer, 0);
    let held = e.mem.u32(pointer);
    if held != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    }
    e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_f390, bs_portal_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c4_f420, bs_portal_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c4_f550, fn_00c4f550(Ptr)),
        entry!(0x00c4_f620, bs_portal_shared_node_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c4_f6c0, fn_00c4f6c0(Ptr, Ptr, Ptr)),
        entry!(0x00c4_f6d0, bs_portal_shared_node_on_visible(Ptr, Ptr)),
        entry!(0x00c4_f7b0, fn_00c4f7b0(Ptr) -> u32),
        entry!(0x00c4_f860, fn_00c4f860(Ptr) -> u32),
        entry!(0x00c4_f8e0, bs_portal_bs_portal(Ptr) -> Ptr),
        entry!(0x00c4_f460, bs_portal_shared_node_bs_portal_shared_node(Ptr, u32) -> Ptr),
        entry!(0x00c4_f8d0, fn_00c4f8d0(Ptr) -> u32),
        entry!(0x00c4_f820, bs_portal_shared_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_portal_shared_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c4_f820, 0x00c4_f550, 0xc4);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c4f8d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_f8d0, &args![0x1000u32]).u32(), 0x0120_30b8);
        assert_eq!(e.call(0x00c4_f8d0, &args![0u32]).u32(), 0x0120_30b8);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn rig(block: u32) -> Engine {
        let mut e = Engine::new();
        e.register(NODE_CONSTRUCT_00A5ECB0, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x10);
            Ret::default()
        });
        e.register(LIGHT_ARRAY_CONSTRUCT, |_, _| Ret::default());
        e.register_double(MEMORY_ALLOCATE, move |_, a| {
            assert_eq!(a, [MEMORY_MANAGER, COMPOUND_FRUSTUM_SIZE]);
            block.into_ret()
        });
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn the_shared_node_constructor_builds_its_frustum() {
        let mut e = rig(0x0d00_0000);
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_f460, &args![this, 0x7000u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), PORTAL_SHARED_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0x0d00_0000);
        assert_eq!(e.mem.u32(this + 0x30), 0x810);
        assert_eq!(e.mem.u32(this + PORTAL), 0x7000);
        let log = e.call_log.take().unwrap();
        let node = log
            .iter()
            .find(|(a, _)| *a == NODE_CONSTRUCT_00A5ECB0)
            .unwrap();
        assert_eq!(node.1, vec![this, 0]);
        let lights = log
            .iter()
            .find(|(a, _)| *a == LIGHT_ARRAY_CONSTRUCT)
            .unwrap();
        assert_eq!(lights.1, vec![this + LIGHTS]);
    }

    #[test]
    fn a_failed_allocation_leaves_no_frustum() {
        let mut e = rig(0);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this + COMPOUND_FRUSTUM, 0xdddd_dddd);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f460, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != COMPOUND_FRUSTUM_CONSTRUCT));
    }
}

#[cfg(test)]
mod n082_tests {
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

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    #[test]
    fn portal_constructor_and_clone() {
        let mut e = Engine::new();
        quiet(&mut e, &[OCCLUSION_PLANE_CONSTRUCT, NI_POINTER_ASSIGN]);
        let block = e.mem.alloc(PORTAL_SIZE);
        e.mem.set_u32(block + PORTAL_POINTER, 0xdead_beef);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        let portal = e.mem.alloc(PORTAL_SIZE);
        e.mem.set_u32(portal + PORTAL_SIDE_A, 0x11);
        e.mem.set_u32(portal + PORTAL_SIDE_B, 0x22);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_f390, &args![portal, 0x4000u32]).u32(), block);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![PORTAL_SIZE]]);
        assert_eq!(calls_to(&e, OCCLUSION_PLANE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), PORTAL_VTABLE);
        assert_eq!(e.mem.u32(block + PORTAL_POINTER), 0);
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![block + PORTAL_POINTER, 0]]
        );
        assert_eq!(e.mem.u32(block + PORTAL_SIDE_A), 0x11);
        assert_eq!(e.mem.u32(block + PORTAL_SIDE_B), 0x22);
        // The constructor on its own answers `this`.
        assert_eq!(e.call(0x00c4_f8e0, &args![portal]).u32(), portal);
        // A failed allocation copies into a null clone, as the exe does.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.map(0, 0x1000);
        assert_eq!(e.call(0x00c4_f390, &args![portal, 0x4000u32]).u32(), 0);
        assert_eq!(e.mem.u32(PORTAL_SIDE_B), 0x22);
    }

    #[test]
    fn shared_node_destructor_body_releases_its_members() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                COMPOUND_FRUSTUM_DESTRUCT,
                MEMORY_DEALLOCATE,
                LIGHT_ARRAY_00B5AF80,
                LIGHT_ARRAY_DESTRUCTOR,
                NI_NODE_DESTRUCTOR,
            ],
        );
        let node = e.mem.alloc(SHARED_NODE_SIZE);
        e.mem.set_u32(node + COMPOUND_FRUSTUM, 0x9000);
        e.mem.set_u32(node + PORTAL, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f550, &args![node]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap()[1..].to_vec();
        assert_eq!(
            order,
            vec![
                (COMPOUND_FRUSTUM_DESTRUCT, vec![0x9000]),
                (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x9000]),
                (LIGHT_ARRAY_00B5AF80, vec![node + LIGHTS, 1]),
                (LIGHT_ARRAY_DESTRUCTOR, vec![node + LIGHTS]),
                (NI_NODE_DESTRUCTOR, vec![node]),
            ]
        );
        assert_eq!(e.mem.u32(node), PORTAL_SHARED_NODE_VTABLE);
        assert_eq!(e.mem.u32(node + PORTAL), 0);
        // Without a frustum nothing is destroyed or freed.
        e.mem.set_u32(node + COMPOUND_FRUSTUM, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f550, &args![node]);
        assert!(calls_to(&e, COMPOUND_FRUSTUM_DESTRUCT).is_empty());
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
    }

    #[test]
    fn shared_node_clone_takes_the_portal() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                NODE_CONSTRUCT_00A5ECB0,
                LIGHT_ARRAY_CONSTRUCT,
                COMPOUND_FRUSTUM_CONSTRUCT,
            ],
        );
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let block = e.mem.alloc(SHARED_NODE_SIZE);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        let node = e.mem.alloc(SHARED_NODE_SIZE);
        e.mem.set_u32(node + PORTAL, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_f620, &args![node, 0x4000u32]).u32(), block);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![SHARED_NODE_SIZE]]);
        assert_eq!(e.mem.u32(block), PORTAL_SHARED_NODE_VTABLE);
        assert_eq!(e.mem.u32(block + PORTAL), 0x77);
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c4_f620, &args![node, 0x4000u32]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f6c0, &args![1u32, 2u32, 3u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn shared_node_on_visible_culls_with_its_frustum() {
        let mut e = Engine::new();
        let culler = e.mem.alloc(0x100);
        let frames = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = frames.clone();
        e.register_double(NI_AV_OBJECT_CULL, move |e, a| {
            seen.borrow_mut()
                .push((a[0], a[1], e.mem.u32(culler + CULLER_FRUSTUM)));
            Ret::default()
        });
        e.register(COMPOUND_FRUSTUM_PRETHREAD, |_, _| Ret::default());
        let node = e.mem.alloc(SHARED_NODE_SIZE);
        let child = e.mem.alloc(0x40);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, child);
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, 2);
        e.mem.set_u32(culler + CULLER_FRUSTUM, 0x5555);
        // No used children: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c4_f6d0, &args![node, culler]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u16(node + CHILDREN_USED, 1);
        e.mem.set_u32(node + COMPOUND_FRUSTUM, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f6d0, &args![node, culler]);
        assert_eq!(*frames.borrow(), vec![(child, culler, 0x9000)]);
        assert_eq!(calls_to(&e, COMPOUND_FRUSTUM_PRETHREAD), vec![vec![0x9000]]);
        assert_eq!(e.mem.u32(culler + CULLER_FRUSTUM), 0x5555);
        // Without a frustum the culler culls with none.
        frames.borrow_mut().clear();
        e.mem.set_u32(node + COMPOUND_FRUSTUM, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f6d0, &args![node, culler]);
        assert_eq!(*frames.borrow(), vec![(child, culler, 0)]);
        assert!(calls_to(&e, COMPOUND_FRUSTUM_PRETHREAD).is_empty());
    }

    #[test]
    fn portal_side_states() {
        let mut e = Engine::new();
        let side = |e: &mut Engine, state: Option<u32>| {
            let node = e.mem.alloc(0xc4);
            if let Some(state) = state {
                let shape = e.mem.alloc(0x10);
                e.mem.set_u32(shape + 8, state);
                let multi_bound = e.mem.alloc(0x10);
                e.mem.set_u32(multi_bound + 0xc, shape);
                e.mem.set_u32(node + MULTI_BOUND, multi_bound);
            }
            node
        };
        let visible = side(&mut e, Some(1));
        let hidden = side(&mut e, Some(2));
        let bare = side(&mut e, None);
        let shapeless = e.mem.alloc(0xc4);
        let empty_bound = e.mem.alloc(0x10);
        e.mem.set_u32(shapeless + MULTI_BOUND, empty_bound);
        assert_eq!(e.call(0x00c4_f860, &args![visible]).u32(), 1);
        assert_eq!(e.call(0x00c4_f860, &args![hidden]).u32(), 2);
        assert_eq!(e.call(0x00c4_f860, &args![bare]).u32(), 0);
        assert_eq!(e.call(0x00c4_f860, &args![shapeless]).u32(), 0);
        let portal = e.mem.alloc(PORTAL_SIZE);
        assert_eq!(e.call(0x00c4_f7b0, &args![portal]).u32(), 0);
        e.mem.set_u32(portal + PORTAL_SIDE_A, hidden);
        e.mem.set_u32(portal + PORTAL_SIDE_B, visible);
        assert_eq!(e.call(0x00c4_f7b0, &args![portal]).u32(), 1);
        e.mem.set_u32(portal + PORTAL_SIDE_A, visible);
        e.mem.set_u32(portal + PORTAL_SIDE_B, hidden);
        assert_eq!(e.call(0x00c4_f7b0, &args![portal]).u32(), 1);
        e.mem.set_u32(portal + PORTAL_SIDE_A, hidden);
        e.mem.set_u32(portal + PORTAL_SIDE_B, bare);
        assert_eq!(e.call(0x00c4_f7b0, &args![portal]).u32(), 0);
    }
}

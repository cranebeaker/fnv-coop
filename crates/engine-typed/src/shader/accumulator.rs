//! The shader accumulator's side (`BSShaderAccumulator`, Xbox PDB): registering a visible geometry's
//! passes with the thread's batch renderer, drawing the sorted geometry, and the per-geometry constant
//! set-ups the shaders call while drawing (the world matrix, the light colour, the fog colour, the
//! pass textures, the alpha and blend states).

use super::batch::{self, pool_give, POOL_LOCK, RENDERING_PASS, RENDERING_PASS_TYPE, SKIN_RTTI};
use super::layout::{
    BSBatchRenderer, BSShaderAccumulator, BSShaderProperty, NiDX9Renderer, NiGeometry, NiProperty,
    NiSkinInstance, RenderPass, ShaderLight, ShadowSceneNode,
};
use super::state;
use super::{elem_addr, low8, FAMILY};
use crate::culling::layout::{NiMatrix3, NiPoint3, NiTransform};
use crate::culling::{fld, fstp, m32};
use engine_core::ptr::Ptr;
use engine_core::x87::{
    x87_add, x87_load32, x87_load64, x87_m32, x87_mul, x87_quiet_bits, x87_store32, x87_sub,
};
use engine_core::{args, typed_call, Engine};

/// The thread block's word with the thread's index (`+0x2bc`).
const TLS_THREAD_INDEX: u32 = 0x2bc;
/// The render modes' register functions (`011f9f80`, entry 0 for a mode without one); the generic one
/// is [`process_commands`].
const REGISTER_FUNCTIONS: u32 = 0x011f_9f80;
const PROCESS_COMMANDS: u32 = 0x00b6_4020;
/// The shader flags handed to `GetRenderPasses` (`011f91d8`).
const SHADER_FLAGS: u32 = 0x011f_91d8;
/// `NiAVObject`'s empty bound (`011f4288`).
const NULL_BOUND: u32 = 0x011f_4288;
/// `BSShaderManager::GetShader` (`cdecl(index)`), a shader's pass set-up (`cdecl(type, 0)`) and the
/// accumulator's immediate pass render (`00b63320`, `cdecl(pass, type)`).
const GET_SHADER: u32 = 0x00b5_5560;
const SHADER_PASS_SETUP: u32 = 0x00b7_a870;
const RENDER_PASS_NOW: u32 = 0x00b6_3320;
/// The property slots: depth pass (0xac), render passes (0xa0), the first-person passes (0xa8); the
/// renderer's `AddPass` (0x8c); a shader's own (0x24); the accumulator's sort (0xa4); a skin's type
/// (8).
const SLOT_DEPTH_PASS: u32 = 0xac;
const SLOT_RENDER_PASSES: u32 = 0xa0;
const SLOT_LATE_PASSES: u32 = 0xa8;
const SLOT_ADD_PASS: u32 = 0x8c;
const SLOT_SHADER_OWN: u32 = 0x24;
const SLOT_SORT: u32 = 0xa4;
const SLOT_SKIN_TYPE: u32 = 8;
/// `BSShaderProperty` flag bits the register function sets: the batch renderer is grouping (`0x200`
/// of `ulFlags`), first person (`0x40` of `ulFlags2`).
const FLAG_GROUPING: u32 = 0x200;
const FLAG_FIRST_PERSON: u32 = 0x40;
/// The node pool's lock (`0040fbf0`, `thiscall(lock, 0)`).
const LOCK_ENTER: u32 = 0x0040_fbf0;

/// The thread's batch renderer, by the thread block's index with no modulo.
#[inline(always)]
fn thread_renderer(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) -> Ptr<BSBatchRenderer> {
    let tls = e.tls();
    let thread = e.global::<u32>(tls.wrapping_add(TLS_THREAD_INDEX));
    renderer_at(e, accumulator, thread)
}

#[inline(always)]
fn renderer_at(
    e: &Engine,
    accumulator: Ptr<BSShaderAccumulator>,
    index: u32,
) -> Ptr<BSBatchRenderer> {
    Ptr::new(e.global::<u32>(elem_addr(
        accumulator,
        BSShaderAccumulator::pBatchRenderer,
        index,
        4,
    )))
}

/// When `bit` of a flags word isn't what `on` says: clears `iLastRenderPassState` (+0x38) and
/// the render pass list's word +0x10 (that one only for `ulFlags`), then sets or clears the
/// bit. When it already is, nothing is written, as in the exe (under real threads a write-back
/// could lose another thread's change to the word).
#[inline(always)]
fn follow_bit(
    e: &mut Engine,
    property: Ptr<BSShaderProperty>,
    word: engine_core::ptr::Field<BSShaderProperty, u32>,
    bit: u32,
    on: bool,
    clears_list: bool,
) {
    let set = e.get(property, word) & bit != 0;
    if on != set {
        e.set(property, BSShaderProperty::iLastRenderPassState, 0);
        if clears_list {
            let list = e.get(property, BSShaderProperty::pRenderPassList);
            if list != 0 {
                e.set_global(list.wrapping_add(0x10), 0u32);
            }
        }
        let flags = e.get(property, word);
        e.set(property, word, if on { flags | bit } else { flags & !bit });
    }
}

/// Registers a pass with the thread's batch renderer in `group`.
#[inline(always)]
fn register(
    e: &mut Engine,
    accumulator: Ptr<BSShaderAccumulator>,
    pass: Ptr<RenderPass>,
    group: u32,
    prepend: u32,
) {
    let renderer = thread_renderer(e, accumulator);
    typed_call!(
        FAMILY,
        0x00b9_b3b0,
        batch::register_pass(e, renderer, pass, group, prepend)
    );
}

// Typed from 00b64020
/// The render modes' generic register function (Ghidra made no function here; 00b64020 to the `RET`
/// at 00b643f8, typed from the translation and its notes), `cdecl(accumulator, geometry, state, property,
/// shader)`, always true. A shader of type 0xd answers at once while +0x164 is set. With depth passes
/// on, the property's depth pass goes to group 0xc in first person, else 0xb. The property's grouping
/// bit follows whether the thread's batch renderer groups alphas, and its first-person bit the
/// accumulator's. Each render pass with a type is filed by its register byte: 1 the alpha holder (flagged
/// when the radius test is on and its bound's radius is below the limit), 2, 3, 6, 7, 10, 14 groups
/// 0xa, 1, 6, 8, 0, 9; 4 and 5 (only with decals) groups 2 (prepended) and 3; 8, 9, 11 to 13 nothing;
/// any other byte the renderer's `AddPass`. In first person while accumulating, the property's late
/// passes go to the renderer, or are rendered now when not accumulating.
pub fn process_commands(
    e: &mut Engine,
    accumulator: Ptr<BSShaderAccumulator>,
    geometry: Ptr<NiGeometry>,
    _state: u32,
    property: Ptr<BSShaderProperty>,
    shader: u32,
) -> bool {
    type A = BSShaderAccumulator;
    if e.get(accumulator, A::ucSkipType13) != 0 && e.global::<u32>(shader.wrapping_add(0x68)) == 0xd
    {
        return true;
    }
    if e.get(accumulator, A::bDoDepthPasses) != 0 {
        let depth = e
            .vcall(property.addr(), SLOT_DEPTH_PASS, &args![geometry])
            .u32();
        if depth != 0 {
            let group = if e.get(accumulator, A::b1stPerson) != 0 {
                0xcu32
            } else {
                0xb
            };
            register(e, accumulator, Ptr::new(depth), group, 0);
        }
    }
    let tls = e.tls();
    let thread = e.global::<u32>(tls.wrapping_add(TLS_THREAD_INDEX));
    let count = e.get(accumulator, A::uiBatchRendererCount);
    let current = renderer_at(e, accumulator, thread % count);
    let active = e.get(current, BSBatchRenderer::iGroupingAlphas) > 0
        && e.get(current, BSBatchRenderer::ucGroupingActive) != 0;
    follow_bit(
        e,
        property,
        BSShaderProperty::ulFlags,
        FLAG_GROUPING,
        active,
        true,
    );
    let mode = e.get(accumulator, A::eRenderMode);
    let flags = e.global::<u32>(SHADER_FLAGS);
    let list = e.with_stack(4, |e, local| {
        e.set_global(local.addr(), 0u32);
        e.vcall(
            property.addr(),
            SLOT_RENDER_PASSES,
            &args![geometry, flags, local, mode, accumulator, 1u32],
        )
        .u32()
    });
    let first_person = e.get(accumulator, A::b1stPerson) != 0;
    follow_bit(
        e,
        property,
        BSShaderProperty::ulFlags2,
        FLAG_FIRST_PERSON,
        first_person,
        false,
    );
    if list == 0 || e.global::<u32>(list.wrapping_add(0x10)) == 0 {
        return true;
    }
    let mut index = 0u32;
    while index < e.global::<u32>(list.wrapping_add(0x10)) {
        let base = e.global::<u32>(list.wrapping_add(4));
        let pass =
            Ptr::<RenderPass>::new(e.global::<u32>(base.wrapping_add(index.wrapping_mul(4))));
        let pass_enum = e.get(pass, RenderPass::usPassEnum);
        if pass_enum != 0 {
            register_by_kind(e, accumulator, pass, pass_enum);
        }
        index = index.wrapping_add(1);
    }
    if e.get(accumulator, A::b1stPerson) == 0 || e.get(accumulator, A::bAccumulate) == 0 {
        return true;
    }
    let late = e.vcall(property.addr(), SLOT_LATE_PASSES, &[]).u32();
    if late == 0 || e.global::<u32>(late.wrapping_add(0x10)) == 0 {
        return true;
    }
    let mut index = 0u32;
    loop {
        let accumulate = e.get(accumulator, A::bAccumulate) != 0;
        let base = e.global::<u32>(late.wrapping_add(4));
        let pass =
            Ptr::<RenderPass>::new(e.global::<u32>(base.wrapping_add(index.wrapping_mul(4))));
        if accumulate {
            let renderer = thread_renderer(e, accumulator);
            e.vcall(renderer.addr(), SLOT_ADD_PASS, &args![pass, 0u32]);
        } else {
            let shader = e.call(GET_SHADER, &args![1u32]).u32();
            let pass_enum = u32::from(e.get(pass, RenderPass::usPassEnum));
            e.call(SHADER_PASS_SETUP, &args![pass_enum, 0u32]);
            if shader != 0 {
                let first = e.global::<u32>(shader.wrapping_add(0x34));
                e.vcall(first, SLOT_SHADER_OWN, &[]);
                let second = e.global::<u32>(shader.wrapping_add(0x30));
                e.vcall(second, SLOT_SHADER_OWN, &[]);
            }
            let pass_enum = u32::from(e.get(pass, RenderPass::usPassEnum));
            e.call(RENDER_PASS_NOW, &args![pass, pass_enum]);
        }
        index = index.wrapping_add(1);
        if index >= e.global::<u32>(late.wrapping_add(0x10)) {
            return true;
        }
    }
}

/// The register byte's jump table of [`process_commands`].
fn register_by_kind(
    e: &mut Engine,
    accumulator: Ptr<BSShaderAccumulator>,
    pass: Ptr<RenderPass>,
    pass_enum: u16,
) {
    type A = BSShaderAccumulator;
    let decals = |e: &Engine| e.get(accumulator, A::bRenderDecals) != 0;
    match e.get(pass, RenderPass::ucRegisterKind) {
        1 => {
            let mut flag = 0u32;
            if e.get(accumulator, A::ucRadiusTest) != 0 {
                let geometry = e.get(pass, RenderPass::pGeometry);
                let mut bound = e.get(geometry, NiGeometry::m_kWorldBound);
                if bound == 0 {
                    bound = NULL_BOUND;
                }
                // `FILD` of the limit against the bound's word +8 as a float: above and ordered.
                let value = x87_load32(e.global::<u32>(bound.wrapping_add(8)));
                let limit = f64::from(e.get(accumulator, A::iRadiusLimit));
                if limit > value {
                    flag = 1;
                }
            }
            let renderer = thread_renderer(e, accumulator);
            typed_call!(
                FAMILY,
                0x00b9_b3d0,
                batch::register_alpha(e, renderer, pass, flag)
            );
        }
        2 => register(e, accumulator, pass, 0xa, 0),
        3 => register(e, accumulator, pass, 1, 0),
        4 => {
            if decals(e) {
                register(e, accumulator, pass, 2, 1);
            }
        }
        5 => {
            if decals(e) {
                register(e, accumulator, pass, 3, 0);
            }
        }
        6 => register(e, accumulator, pass, 6, 0),
        7 => register(e, accumulator, pass, 8, 0),
        10 => register(e, accumulator, pass, 0, 0),
        14 => register(e, accumulator, pass, 9, 0),
        8 | 9 | 11..=13 => {}
        _ => {
            let renderer = thread_renderer(e, accumulator);
            e.vcall(
                renderer.addr(),
                SLOT_ADD_PASS,
                &args![pass, u32::from(pass_enum)],
            );
        }
    }
}

// Typed from 00b63f10
/// The accumulator's `RegisterObject`, virtual slot 0x98 (Ghidra made no function here; typed from the
/// translation), `thiscall(accumulator, geometry)`: true at once for a dismember skin that isn't
/// renderable, or a geometry without shader or property; else the render mode's register function (or
/// entry 0) with the accumulator, the geometry, its property state, property and shader, and its answer.
pub fn register_object(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    geometry: Ptr<NiGeometry>,
) -> bool {
    let skin = e.get(geometry, NiGeometry::m_spSkinInstance);
    if !skin.is_null()
        && e.vcall(skin.addr(), SLOT_SKIN_TYPE, &[]).u32() == SKIN_RTTI
        && e.get(skin, NiSkinInstance::bIsRenderable) == 0
    {
        return true;
    }
    let shader = e.get(geometry, NiGeometry::m_spShader);
    let property = e.get(geometry, NiGeometry::pShaderProperty);
    if shader == 0 || property.is_null() {
        return true;
    }
    let mode = e.get(this, BSShaderAccumulator::eRenderMode);
    let mut function = e.global::<u32>(REGISTER_FUNCTIONS.wrapping_add(mode.wrapping_mul(4)));
    if function == 0 {
        function = e.global::<u32>(REGISTER_FUNCTIONS);
    }
    let state = geometry
        .addr()
        .wrapping_add(NiGeometry::m_PropertyState.off);
    if function == PROCESS_COMMANDS {
        typed_call!(
            FAMILY,
            0x00b6_4020,
            process_commands(e, this, geometry, state, property, shader)
        )
    } else {
        e.call(function, &args![this, geometry, state, property, shader])
            .bool()
    }
}

// Typed from 00b65e80
/// `cdecl(accumulator)`: with a camera, the sorted list is reset (+0x18), the accumulator sorts (slot
/// 0xa4), the sorted geometry is drawn ([`draw_sorted`]) and alpha blending goes off.
pub fn finish(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    if e.get(accumulator, BSShaderAccumulator::m_pkCamera) == 0 {
        return;
    }
    e.set(accumulator, BSShaderAccumulator::pGeometryList, 0);
    e.vcall(accumulator.addr(), SLOT_SORT, &[]);
    typed_call!(FAMILY, 0x00b6_4f90, draw_sorted(e, accumulator));
    typed_call!(
        FAMILY,
        0x00b9_7fa0,
        state::set_alpha_blend_enable(e, 0u8, 0u32)
    );
}

/// A node of a geometry list back to the node pool (the item cleared, under the pool's lock).
#[inline(always)]
fn give_node(e: &mut Engine, node: u32) {
    e.set_global(node.wrapping_add(8), 0u32);
    e.call(LOCK_ENTER, &args![POOL_LOCK, 0u32]);
    pool_give(e, node);
}

// Typed from 00b64f90
/// `thiscall(accumulator)`: ends the batch renderer's pass, draws the sorted geometry last to first
/// through the accumulator's own pass (type from +0x1a8, all three flags), stopping at a null one,
/// then empties the geometry list (the one at +0x18, else the one inline at +0xc) into the node pool,
/// and ends the pass again.
pub fn draw_sorted(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    type A = BSShaderAccumulator;
    typed_call!(FAMILY, 0x00b9_88e0, batch::end_pass(e));
    let pass = this.at(A::kSortedPass);
    let mut index = e.get(this, A::uiSortedCount);
    e.set(this, A::uiSortedCurrent, index);
    if index != 0 {
        index -= 1;
        e.set(this, A::uiSortedCurrent, index);
        let mut item = e.global::<u32>(e.get(this, A::ppSorted).wrapping_add(index * 4));
        while item != 0 {
            let pass_type = u32::from(e.get(pass, RenderPass::usPassEnum));
            e.set(pass, RenderPass::pGeometry, Ptr::new(item));
            typed_call!(
                FAMILY,
                0x00b9_94f0,
                batch::render_pass(e, pass, pass_type, 1u32, 1u32, 1u32)
            );
            let index = e.get(this, A::uiSortedCurrent);
            if index == 0 {
                break;
            }
            e.set(this, A::uiSortedCurrent, index - 1);
            item = e.global::<u32>(e.get(this, A::ppSorted).wrapping_add((index - 1) * 4));
        }
    }
    let list = e.get(this, A::pGeometryList);
    if list != 0 {
        if e.global::<u32>(list.wrapping_add(8)) != 0 {
            loop {
                let list = e.get(this, A::pGeometryList);
                let node = e.global::<u32>(list);
                let next = e.global::<u32>(node);
                e.set_global(list, next);
                if next != 0 {
                    e.set_global(next.wrapping_add(4), 0u32);
                } else {
                    e.set_global(list.wrapping_add(4), 0u32);
                }
                give_node(e, node);
                let count = e.global::<u32>(list.wrapping_add(8)).wrapping_sub(1);
                e.set_global(list.wrapping_add(8), count);
                if e.global::<u32>(e.get(this, A::pGeometryList).wrapping_add(8)) == 0 {
                    break;
                }
            }
        }
    } else if e.get(this, A::kGeometry_count) != 0 {
        loop {
            let node = e.get(this, A::kGeometry_head);
            let next = e.global::<u32>(node);
            e.set(this, A::kGeometry_head, next);
            if next != 0 {
                e.set_global(next.wrapping_add(4), 0u32);
            } else {
                e.set(this, A::kGeometry_tail, 0);
            }
            give_node(e, node);
            let count = e.get(this, A::kGeometry_count).wrapping_sub(1);
            e.set(this, A::kGeometry_count, count);
            if count == 0 {
                break;
            }
        }
    }
    typed_call!(FAMILY, 0x00b9_88e0, batch::end_pass(e));
}

// Typed from 00be1ff0
/// `thiscall(shader, block)`: alpha blending for the geometry: off when the property (+0xc of the
/// block; one whose +0x1c is -1 counts as none, and none reads address 0) has both alphas at least 1.0
/// (ordered) and the alpha property (+0) doesn't blend; else on, with the alpha property's source and
/// destination factors (flags bits 1-4 and 5-8) or 6 and 7.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // NaN-aware as the exe: not above, including unordered.
pub fn set_blend_of(e: &mut Engine, _this: u32, block: u32) {
    let alpha = Ptr::<NiProperty>::new(e.global::<u32>(block));
    let mut object = Ptr::<BSShaderProperty>::new(e.global::<u32>(block.wrapping_add(0xc)));
    if !object.is_null() && e.get(object, BSShaderProperty::unknown_1c) == u32::MAX {
        object = Ptr::new(0);
    }
    let blends = !alpha.is_null() && e.get(alpha, NiProperty::m_usFlags) & 1 != 0;
    let opaque = !(1.0 > m32(e, object, BSShaderProperty::fAlpha))
        && !(1.0 > m32(e, object, BSShaderProperty::fFadeAlpha))
        && !blends;
    if opaque {
        typed_call!(
            FAMILY,
            0x00b9_7fa0,
            state::set_alpha_blend_enable(e, 0u8, 0u32)
        );
        return;
    }
    typed_call!(
        FAMILY,
        0x00b9_7fa0,
        state::set_alpha_blend_enable(e, 1u8, 0u32)
    );
    let (source, destination) = if blends {
        let flags = u32::from(e.get(alpha, NiProperty::m_usFlags));
        ((flags >> 1) & 0xf, (flags >> 5) & 0xf)
    } else {
        (6, 7)
    };
    typed_call!(
        FAMILY,
        0x00b9_7ff0,
        state::set_blend(e, source, destination, 0u32)
    );
}

/// The world origin offset the matrices are made relative to (`011f474c`).
const ORIGIN: u32 = 0x011f_474c;

// Typed from 00b71a40
/// `cdecl(out, rotation, point, scale)`: a Direct3D matrix (row vectors) from a rotation scaled, the
/// point less the world origin as the translation, 0 0 0 1 as the last column.
pub fn make_matrix(
    e: &mut Engine,
    out: u32,
    rotation: Ptr<NiMatrix3>,
    point: Ptr<NiPoint3>,
    scale: f32,
) {
    let scale = x87_load32(scale.to_bits());
    let r = |e: &Engine, i: u32| {
        x87_load32(e.global::<u32>(elem_addr(rotation, NiMatrix3::m_pEntry, i, 4)))
    };
    let put = |e: &mut Engine, i: u32, value: f32| e.set_global(out.wrapping_add(i * 4), value);
    // Columns of the rotation become rows: out[0..3] = r0 r3 r6, out[4..7] = r1 r4 r7, out[8..11] =
    // r2 r5 r8, each times the scale; the first and last products with the scale as the source.
    let v = fstp(x87_mul(r(e, 0), scale));
    put(e, 0, v);
    let v = fstp(x87_mul(r(e, 3), scale));
    put(e, 1, v);
    let v = fstp(x87_mul(r(e, 6), scale));
    put(e, 2, v);
    put(e, 3, 0.0);
    let v = fstp(x87_mul(r(e, 1), scale));
    put(e, 4, v);
    let v = fstp(x87_mul(r(e, 4), scale));
    put(e, 5, v);
    let v = fstp(x87_mul(r(e, 7), scale));
    put(e, 6, v);
    put(e, 7, 0.0);
    let v = fstp(x87_mul(r(e, 2), scale));
    put(e, 8, v);
    let v = fstp(x87_mul(r(e, 5), scale));
    put(e, 9, v);
    let v = fstp(x87_mul(scale, r(e, 8)));
    put(e, 10, v);
    put(e, 11, 0.0);
    for (i, field) in (0u32..).zip([NiPoint3::x, NiPoint3::y, NiPoint3::z]) {
        let origin = x87_m32(e.global::<u32>(ORIGIN.wrapping_add(4 * i)));
        let v = fstp(x87_sub(fld(e, point, field), origin));
        put(e, 12 + i, v);
    }
    put(e, 15, 1.0);
}

/// `IDirect3DDevice9::SetTransform`'s slot and `D3DTS_WORLD`; the render state's world-transform
/// notice (slot 0x30).
const DEVICE_SET_TRANSFORM: u32 = 0xb0;
const WORLD: u32 = 0x100;
const SLOT_STATE_TRANSFORM: u32 = 0x30;

// Typed from 00e6fbb0
/// `thiscall(renderer, transform, set world)`: the world matrix from the transform ([`make_matrix`]),
/// sent to the device when asked; the render state is told of the transform; the rotation scaled turns
/// the renderer's two directions into model space (+0x64c and +0x658), each sum in the exe's order.
pub fn set_world_transform(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    transform: Ptr<NiTransform>,
    set_world: u8,
) {
    type R = NiDX9Renderer;
    let t = transform.addr();
    let world = this.addr().wrapping_add(R::m_kWorld.off);
    let scale = x87_quiet_bits(e.get(transform, NiTransform::m_fScale).to_bits());
    let rotation = transform.at(NiTransform::m_Rotate);
    let point = transform.at(NiTransform::m_Translate);
    typed_call!(
        FAMILY,
        0x00b7_1a40,
        make_matrix(e, world, rotation, point, f32::from_bits(scale))
    );
    if set_world != 0 {
        let device = e.get(this, R::m_pkD3DDevice9);
        e.vcall(device, DEVICE_SET_TRANSFORM, &args![WORLD, world]);
    }
    let state = e.get(this, R::m_pkRenderState);
    // `NiDX9RenderState`'s slot 0x30 is `00e879e0` ([`state::set_alpha_test_of`]).
    if crate::slot_target(e, state.addr(), SLOT_STATE_TRANSFORM) == 0x00e8_79e0 {
        typed_call!(FAMILY, 0x00e8_79e0, state::set_alpha_test_of(e, state, t));
    } else {
        e.vcall(state.addr(), SLOT_STATE_TRANSFORM, &args![t]);
    }
    let s = x87_load32(x87_quiet_bits(
        e.get(transform, NiTransform::m_fScale).to_bits(),
    ));
    let entry = |e: &Engine, i: u32| x87_load32(e.global::<u32>(t.wrapping_add(i * 4)));
    let mut a = [0u32; 9];
    for (i, word) in (0u32..).zip(a.iter_mut().take(8)) {
        *word = x87_store32(x87_mul(entry(e, i), s));
    }
    a[8] = x87_store32(x87_mul(s, x87_m32(e.global::<u32>(t.wrapping_add(0x20)))));
    let ld = |i: usize| x87_load32(a[i]);
    let m = |i: usize| x87_m32(a[i]);
    let v = |e: &Engine, i: u32| x87_load32(e.global::<u32>(elem_addr(this, R::kDirectionA, i, 4)));
    let w = |e: &Engine, i: u32| x87_load32(e.global::<u32>(elem_addr(this, R::kDirectionB, i, 4)));
    let wm = |e: &Engine, i: u32| x87_m32(e.global::<u32>(elem_addr(this, R::kDirectionB, i, 4)));
    let sum = x87_add(x87_mul(v(e, 1), ld(3)), x87_mul(v(e, 0), ld(0)));
    let out0 = x87_store32(x87_add(x87_mul(v(e, 2), ld(6)), sum));
    let sum = x87_add(x87_mul(v(e, 1), ld(4)), x87_mul(v(e, 0), ld(1)));
    let out1 = x87_store32(x87_add(sum, x87_mul(v(e, 2), m(7))));
    let sum = x87_add(x87_mul(v(e, 0), m(2)), x87_mul(v(e, 1), m(5)));
    let out2 = x87_store32(x87_add(sum, x87_mul(v(e, 2), m(8))));
    for (i, word) in (0u32..).zip([out0, out1, out2]) {
        e.set_global(elem_addr(this, R::kModelDirectionA, i, 4), word);
    }
    let sum = x87_add(x87_mul(ld(0), w(e, 0)), x87_mul(ld(3), w(e, 1)));
    let out3 = x87_store32(x87_add(x87_mul(ld(6), w(e, 2)), sum));
    let sum = x87_add(x87_mul(ld(1), w(e, 0)), x87_mul(ld(4), wm(e, 1)));
    let out4 = x87_store32(x87_add(sum, x87_mul(w(e, 2), m(7))));
    let sum = x87_add(x87_mul(w(e, 0), m(2)), x87_mul(w(e, 1), m(5)));
    let out5 = x87_store32(x87_add(sum, x87_mul(w(e, 2), m(8))));
    for (i, word) in (0u32..).zip([out3, out4, out5]) {
        e.set_global(elem_addr(this, R::kModelDirectionB, i, 4), word);
    }
}

/// The current pass (`0126f74c`): its vertex shader (+0x5c) holds the device (+0x20) whose slot 0x178
/// (`SetVertexShaderConstantF`) takes constants; its stages are at +0x24.
const CURRENT_PASS: u32 = 0x0126_f74c;
const DEVICE_SET_VERTEX_CONSTANTS: u32 = 0x178;
/// `D3DXMatrixMultiplyTranspose`, `D3DXVec3TransformNormal`, `D3DXVec3Normalize`.
const MATRIX_MULTIPLY_TRANSPOSE: u32 = 0x00ee_6dfe;
const VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const VEC3_NORMALIZE: u32 = 0x00ee_6dec;
/// `-1.0` (`double`, `0101a6b0`).
const MINUS_ONE: u32 = 0x0101_a6b0;

#[inline(always)]
fn set_vertex_constants(e: &mut Engine, register: u32, data: u32, count: u32) {
    let pass = e.global::<u32>(CURRENT_PASS);
    let shader = e.global::<u32>(pass.wrapping_add(0x5c));
    let device = e.global::<u32>(shader.wrapping_add(0x20));
    e.vcall(
        device,
        DEVICE_SET_VERTEX_CONSTANTS,
        &args![register, data, count],
    );
}

// Typed from 00b7c750
/// `thiscall(shader, unused, transform)`: the renderer (+0x18 of the shader) takes the transform
/// ([`set_world_transform`], not sent); its world times view, transposed, goes to vertex constants 0
/// to 3.
pub fn set_world_view_constants(e: &mut Engine, this: u32, _unused: u32, transform: u32) {
    let renderer = Ptr::<NiDX9Renderer>::new(e.global::<u32>(this.wrapping_add(0x18)));
    typed_call!(
        FAMILY,
        0x00e6_fbb0,
        set_world_transform(e, renderer, Ptr::new(transform), 0u8)
    );
    let renderer = e.global::<u32>(this.wrapping_add(0x18));
    e.with_stack(0x40, |e, matrix| {
        e.call(
            MATRIX_MULTIPLY_TRANSPOSE,
            &args![
                matrix,
                renderer.wrapping_add(NiDX9Renderer::m_kWorld.off),
                renderer.wrapping_add(NiDX9Renderer::m_kView.off)
            ],
        );
        set_vertex_constants(e, 0, matrix.addr(), 4);
    });
}

// Typed from 00b7c7b0
/// `stdcall(matrix)`: the drawn object's direction (+0xf0 of its +0xf8) negated, through the matrix as
/// a normal, normalized, goes to vertex constant 0x19. The frame is the translation's: 0x24 bytes, the
/// constant's fourth float its last word (in the exe the word above its 0x20-byte frame).
pub fn set_light_direction(e: &mut Engine, matrix: u32) {
    let rendering = e.global::<u32>(RENDERING_PASS);
    let holder = e.global::<u32>(rendering.wrapping_add(0xc));
    let object = e.global::<u32>(holder);
    let vector = e.global::<u32>(object.wrapping_add(0xf8));
    let minus_one = x87_load64(e.global::<u64>(MINUS_ONE));
    let y = x87_store32(x87_mul(
        x87_load32(e.global::<u32>(vector.wrapping_add(0xf4))),
        minus_one,
    ));
    let z = x87_store32(x87_mul(
        x87_load32(e.global::<u32>(vector.wrapping_add(0xf8))),
        minus_one,
    ));
    let x = x87_store32(x87_mul(
        minus_one,
        x87_m32(e.global::<u32>(vector.wrapping_add(0xf0))),
    ));
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        e.set_global(frame, y);
        e.set_global(frame + 4, z);
        e.set_global(frame + 8, x);
        e.set_global(frame + 0xc, x87_quiet_bits(y));
        e.set_global(frame + 0x10, x87_quiet_bits(z));
        e.call(
            VEC3_TRANSFORM_NORMAL,
            &args![frame + 0x14, frame + 8, matrix],
        );
        e.call(VEC3_NORMALIZE, &args![frame + 8, frame + 0x14]);
        set_vertex_constants(e, 0x19, frame + 0x14, 1);
    });
}

/// The two default textures (`011f9518` for a property with bit 7 of its flags, else `011f9510`).
const DEFAULT_TEXTURE_FLAGGED: u32 = 0x011f_9518;
const DEFAULT_TEXTURE: u32 = 0x011f_9510;

// Typed from 00c03230
/// `stdcall(property, index)`: the property's texture `index` (its array at +0xb0), or a default one.
pub fn texture_by_index(e: &mut Engine, property: Ptr<BSShaderProperty>, index: u32) -> u32 {
    let textures = e.global::<u32>(property.addr().wrapping_add(0xb0));
    let texture = e.global::<u32>(textures.wrapping_add(index.wrapping_mul(4)));
    if texture != 0 {
        return texture;
    }
    if e.get(property, BSShaderProperty::ulFlags) & 0x80 != 0 {
        e.global::<u32>(DEFAULT_TEXTURE_FLAGGED)
    } else {
        e.global::<u32>(DEFAULT_TEXTURE)
    }
}

/// The neutral colour (`011f4998`), the HDR byte (`011f941e`), the image space manager (`011f91ac`),
/// the ambient sum (`011fa0c0`, four floats).
const NEUTRAL: u32 = 0x011f_4998;
const HDR_ENABLED: u32 = 0x011f_941e;
const IMAGE_SPACE_MANAGER: u32 = 0x011f_91ac;
const AMBIENT: u32 = 0x011f_a0c0;

// Typed from 00c03010
/// `stdcall(light)`: adds the light's colour (unless it is the neutral one) to the ambient sum: scaled
/// by the HDR scale times the fade, or by the fade when it is below 1; without HDR each sum is held to
/// at most 1.0 (a NaN becomes 1.0).
#[allow(clippy::neg_cmp_op_on_partial_ord)] // NaN-aware as the exe: not above, including unordered.
pub fn accumulate_light_color(e: &mut Engine, light: Ptr<ShaderLight>) {
    let mut color = e.get(light, ShaderLight::pColor);
    if color == 0 {
        color = light.addr().wrapping_add(ShaderLight::kColor.off);
    }
    let channel = |e: &Engine, i: u32| x87_load32(e.global::<u32>(color.wrapping_add(4 * i)));
    let (red, green, blue) = (channel(e, 0), channel(e, 1), channel(e, 2));
    let neutral = |e: &Engine, i: u32| x87_load32(e.global::<u32>(NEUTRAL + 4 * i));
    if neutral(e, 0) == red && neutral(e, 1) == green && neutral(e, 2) == blue {
        return;
    }
    let hdr = e.global::<u8>(HDR_ENABLED) != 0;
    let scale = if hdr {
        let manager = e.global::<u32>(IMAGE_SPACE_MANAGER);
        let effect = e.global::<u32>(e.global::<u32>(manager.wrapping_add(8)).wrapping_add(4));
        let data = e.global::<u32>(effect.wrapping_add(0x6c));
        Some(x87_mul(
            x87_load32(e.global::<u32>(data.wrapping_add(0xc))),
            m32(e, light, ShaderLight::fFade),
        ))
    } else if 1.0 > m32(e, light, ShaderLight::fFade) {
        Some(fld(e, light, ShaderLight::fFade))
    } else {
        None
    };
    let (red, green, blue) = match scale {
        Some(scale) => {
            let scale = x87_load32(x87_store32(scale));
            let red = x87_store32(x87_mul(red, scale));
            let green = x87_store32(x87_mul(green, scale));
            let blue = x87_store32(x87_mul(blue, scale));
            (x87_load32(red), x87_load32(green), x87_load32(blue))
        }
        None => (red, green, blue),
    };
    let alpha = e.global::<u32>(AMBIENT + 0xc);
    let mut sums = [0u32; 3];
    for ((i, value), sum) in (0u32..).zip([red, green, blue]).zip(sums.iter_mut()) {
        let ambient = e.global::<u32>(AMBIENT + 4 * i);
        *sum = x87_store32(x87_add(value, x87_m32(ambient)));
    }
    if !hdr {
        for sum in &mut sums {
            if !(1.0 > x87_m32(*sum)) {
                *sum = 0x3f80_0000;
            }
        }
    }
    for (i, sum) in (0u32..).zip(sums) {
        e.set_global(AMBIENT + 4 * i, sum);
    }
    e.set_global(AMBIENT + 0xc, alpha);
}

/// The default colour (`011a9bd0`, four floats), the default blue (`01018f5c`) and the colour set
/// (`011fa2d0`).
const DEFAULT_COLOR: u32 = 0x011a_9bd0;
const DEFAULT_BLUE: u32 = 0x0101_8f5c;
const COLOR: u32 = 0x011f_a2d0;

// Typed from 00c02d90
/// `thiscall(shader, geometry, green, source)`: the colour constant: red 1.0 when the geometry's
/// property has bit 5 of its second flags, green 1.0 when asked (else the default's), blue the source's
/// float +0x38 (or the default blue), alpha the default's.
pub fn set_tint(e: &mut Engine, _this: u32, owner: Ptr<NiGeometry>, green: u32, source: u32) {
    let default_green = e.global::<u32>(DEFAULT_COLOR + 4);
    let alpha = e.global::<u32>(DEFAULT_COLOR + 0xc);
    let property = e.get(owner, NiGeometry::pShaderProperty);
    let flags = e.get(property, BSShaderProperty::ulFlags2);
    let red = if flags & 0x20 != 0 { 0x3f80_0000 } else { 0 };
    let green = if green != 0 {
        0x3f80_0000
    } else {
        default_green
    };
    let blue = if source != 0 {
        e.global::<u32>(source.wrapping_add(0x38))
    } else {
        e.global::<u32>(DEFAULT_BLUE)
    };
    let blue = x87_quiet_bits(blue);
    e.set_global(COLOR + 4, green);
    e.set_global(COLOR, red);
    e.set_global(COLOR + 8, blue);
    e.set_global(COLOR + 0xc, alpha);
}

/// The render pass array: the memory manager's allocation and its constructor (`thiscall(array,
/// size)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const PASS_ARRAY_CONSTRUCT: u32 = 0x00ba_94b0;

// Typed from 00ba95e0
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map), `thiscall(property, size)`: makes the
/// render pass list (size 5 for -1) when there is none.
pub fn check_create_pass_array(e: &mut Engine, this: Ptr<BSShaderProperty>, size: i32) {
    if e.get(this, BSShaderProperty::pRenderPassList) != 0 {
        return;
    }
    let size = if size == -1 { 5 } else { size };
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    let array = if block == 0 {
        0
    } else {
        e.call(PASS_ARRAY_CONSTRUCT, &args![block, size]).u32()
    };
    e.set(this, BSShaderProperty::pRenderPassList, array);
}

// Typed from 00ba8ab0
/// `BSShaderProperty::SetAlpha` (engine map), `thiscall(property, alpha)`: the alpha (quieted by the
/// load); crossing 1.0 either way (both ordered) clears `iLastRenderPassState`.
pub fn set_alpha(e: &mut Engine, this: Ptr<BSShaderProperty>, alpha: f32) {
    let old = m32(e, this, BSShaderProperty::fAlpha);
    let new = x87_load32(alpha.to_bits());
    let crosses = (old >= 1.0 && new < 1.0) || (old < 1.0 && new >= 1.0);
    if crosses {
        e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    }
    e.set(this, BSShaderProperty::fAlpha, fstp(new));
}

// Typed from 00b5ac20
/// `thiscall(node, index)`: the light at `index` of the node's light list, or 0 past its count.
pub fn light_at(e: &mut Engine, this: Ptr<ShadowSceneNode>, index: u32) -> u32 {
    if index >= e.get(this, ShadowSceneNode::kLights_count) {
        return 0;
    }
    let head = e.get(this, ShadowSceneNode::kLights_head);
    let mut next = e.global::<u32>(head);
    let mut item = e.global::<u32>(head.wrapping_add(8));
    for _ in 0..index {
        item = e.global::<u32>(next.wrapping_add(8));
        next = e.global::<u32>(next);
    }
    item
}

/// The pair of textures `00b7cfb0` puts into two stages (`011f95e4`, `011f95e8`).
const PAIR_TEXTURES: [u32; 2] = [0x011f_95e4, 0x011f_95e8];

// Typed from 00b7cfb0
/// `thiscall(shader, type)`: for the pass types that sample the pair, its two textures go into the
/// current pass's stages `first` and `first + 1` (stage +8): 2 for 0x94 and 0x1f3, 6, 5 and 4 for their
/// type groups, 14 for every fourth type from 0x1f8 to 0x22c.
pub fn set_pair_textures(e: &mut Engine, _this: u32, pass_type: u32) {
    let first = match pass_type {
        0x94 | 0x1f3 => 2u32,
        0xa0..=0xa2
        | 0xa6..=0xa8
        | 0xb6..=0xb8
        | 0xbc..=0xbe
        | 0x10a..=0x10c
        | 0x110..=0x112
        | 0x115
        | 0x121
        | 0x122
        | 0x126..=0x128
        | 0x12b => 6,
        0xe0 | 0xe1 | 0xe5 | 0xe6 | 0xf2 | 0xf3 | 0xf7 | 0xf8 => 5,
        0x160 | 0x161 | 0x163 | 0x164 => 4,
        0x1f8..=0x22c if (pass_type - 0x1f8) & 3 == 0 => 14,
        _ => return,
    };
    for (i, texture) in (0u32..).zip(PAIR_TEXTURES) {
        let texture = e.global::<u32>(texture);
        let pass = e.global::<u32>(CURRENT_PASS);
        let stages = e.global::<u32>(pass.wrapping_add(0x24));
        let stage = e.global::<u32>(stages.wrapping_add((first + i).wrapping_mul(4)));
        e.set_global(stage.wrapping_add(8), texture);
    }
}

/// `00b98070` (`cdecl(enable, lock)`, not in this family) and the two counters `00b7c320` releases.
const RESTORE_BLEND_STATE: u32 = 0x00b9_8070;
const LOCK_COUNTER_DEPTH_WRITE: u32 = 0x011f_f9dc;
const LOCK_COUNTER_COLOR_MASK: u32 = 0x011f_fa18;

// Typed from 00b7c320
/// The shader's state restore after a geometry (Ghidra made no function here; typed from the
/// translation), `thiscall(shader, block)`: types 0x250 and 0x251 put the depth function back (3);
/// a blending alpha property (+0x10) puts `00b98070` back; types 6 and 8 with bit 18 of the property's
/// flags release the two counters and put depth writes and the colour mask (7) back.
pub fn restore_states(e: &mut Engine, _this: u32, block: u32) {
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE);
    if pass_type == 0x250 || pass_type == 0x251 {
        typed_call!(FAMILY, 0x00b9_7e80, state::set_z_function(e, 3u32, 0u32));
    }
    let alpha = Ptr::<NiProperty>::new(e.global::<u32>(block.wrapping_add(0x10)));
    if !alpha.is_null() && low8(u32::from(e.get(alpha, NiProperty::m_usFlags))) & 1 != 0 {
        e.call(RESTORE_BLEND_STATE, &args![0u32, 0u32]);
    }
    let property = Ptr::<BSShaderProperty>::new(e.global::<u32>(block.wrapping_add(0xc)));
    if (pass_type == 6 || pass_type == 8)
        && e.get(property, BSShaderProperty::ulFlags) & 0x40000 != 0
    {
        let count = e.global::<u32>(LOCK_COUNTER_DEPTH_WRITE).wrapping_sub(1);
        e.set_global(LOCK_COUNTER_DEPTH_WRITE, count);
        typed_call!(FAMILY, 0x00b9_7e30, state::set_z_write_enable(e, 1u8, 0u32));
        let count = e.global::<u32>(LOCK_COUNTER_COLOR_MASK).wrapping_sub(1);
        e.set_global(LOCK_COUNTER_COLOR_MASK, count);
        typed_call!(FAMILY, 0x00b9_8380, state::set_color_write(e, 7u32, 0u32));
    }
}

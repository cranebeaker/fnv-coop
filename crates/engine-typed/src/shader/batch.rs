//! The batch renderer (`BSBatchRenderer`, Xbox PDB): passes are filed by pass type into groups of five
//! lists, the groups are drawn in the order of the active pass type list, and each pass is drawn by
//! beginning the shader's pass when the type or shader changes, then the geometry (plain or skinned,
//! partition by partition). The alpha lists are sorted back to front by the camera's direction.

use super::layout::{
    ActivePass, BSBatchRenderer, NiGeometry, NiGeometryData, NiProperty, NiSkinInstance,
    NiSkinPartition, PassGroup, PassHolder, PassList, PassNode, RenderPass,
};
use super::state::{self, DEVICE_SET_SAMPLER_STATE};
use super::{elem_addr, low8, FAMILY};
use crate::culling::layout::NiBound;
use crate::culling::{fstp, m32};
use engine_core::abi::OneWord;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_load32, x87_m32, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The pass and type being drawn (`011f91e0`, `011f91e4`).
pub(crate) const RENDERING_PASS: u32 = 0x011f_91e0;
pub(crate) const RENDERING_PASS_TYPE: u32 = 0x011f_91e4;
/// The pass type and shader whose pass is begun (`011ffe30`, `011ffe2c`), and the byte that says the
/// shader's per-geometry restore (slot 0x110) is still owed (`011ad8ec`).
const CURRENT_PASS_TYPE: u32 = 0x011f_fe30;
const CURRENT_SHADER: u32 = 0x011f_fe2c;
const RESTORE_OWED: u32 = 0x011a_d8ec;
/// The renderer as the batch renderer holds it (`011f4748`).
const RENDER_HOLDER: u32 = 0x011f_4748;
/// Whether the card takes alpha to coverage (`011f9421`).
const COVERAGE_AVAILABLE: u32 = 0x011f_9421;
/// The two lock counters the group drawing releases (`011ffa24`, `011ff9e8`).
const GROUP_COUNTERS: [u32; 2] = [0x011f_fa24, 0x011f_f9e8];
/// The marker word `00b99390` resets (`011ae63c`) and the sixteen words of stage textures (`0126f680`).
const PASS_MARKER: u32 = 0x011a_e63c;
const STAGE_TEXTURES: u32 = 0x0126_f680;
/// The alpha sort's settings (`011ffe35`: subtract the radius; `011ffe38`: the camera).
const SORT_SUBTRACT_RADIUS: u32 = 0x011f_fe35;
const SORT_CAMERA: u32 = 0x011f_fe38;
/// `NiAVObject`'s empty bound (`011f4288`).
const NULL_BOUND: u32 = 0x011f_4288;
/// `BSSkinInstance`'s type (`011f49d8`), what the skin's slot 8 answers for a dismember skin.
pub(crate) const SKIN_RTTI: u32 = 0x011f_49d8;

/// Callees by address: the map's `GetAt` (`thiscall(map, key, out)`) and `SetAt`, the node pool's
/// allocation (`thiscall(list + 0x10)`), the active list's clear, the memory manager and `operator
/// new`, the batch renderer's constructor, the shadow-pass test, the shaders' own pass set-ups, the
/// renderer's skinned test, the geometry holder's draw, the shaders' pass tails, the node pool lock.
const MAP_GET_AT: u32 = 0x0085_3130;
const MAP_SET_AT: u32 = 0x0084_4700;
const LIST_NODE_ALLOCATE: u32 = 0x0043_a010;
const ACTIVE_LIST_CLEAR: u32 = 0x00b6_4ec0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const RENDERER_CONSTRUCT: u32 = 0x00b9_a5b0;
const ACTIVE_INSERT_FIRST: u32 = 0x00b9_8e20;
const IS_SHADOW_PASS: u32 = 0x00b6_30f0;
const SETUP_TYPES_FC_FD: u32 = 0x00b7_dde0;
const SETUP_TYPE_CA: u32 = 0x00b7_ded0;
const SETUP_TYPE_D1: u32 = 0x00b7_dfe0;
const SETUP_TYPE_102: u32 = 0x00b7_e150;
const BEGIN_TYPES_FC_102: u32 = 0x00b7_dd50;
const BEGIN_TYPES_CA_D1: u32 = 0x00b7_deb0;
const RENDERER_IS_SKINNED: u32 = 0x00e7_2c20;
const HOLDER_DRAW: u32 = 0x00e6_d310;
const TAIL_KINDS_1_TO_5: u32 = 0x00b7_ac20;
const TAIL_KIND_11: u32 = 0x00bb_96d0;
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// The global node pool: its lock (owner `011c5f80`, count `011c5f84`) and free list (`011c5f58`).
pub(crate) const POOL_LOCK: u32 = 0x011c_5f80;
const POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const POOL_FREE: u32 = 0x011c_5f58;
/// The pass-type comparison the active list is kept in order with (`00b988a0`).
const PASS_TYPE_COMPARE: u32 = 0x00b9_88a0;
/// The alpha sort's comparison (this file's [`compare_by_depth`]).
const COMPARE_BY_DEPTH: u32 = 0x00b9_8b20;

/// The shader slots the drawing calls: begin a pass (0xf0, 0xfc, 0xf8, 0x100, 0xf4), end it (0x104,
/// 0x114), per geometry (0x78, 0x7c, 0x80, 0x84, 0x88, 0x6c, 0x8c, 0x110, 0x64).
mod slot {
    pub const BEGIN: u32 = 0xf0;
    pub const BEGIN_STATES: u32 = 0xfc;
    pub const BEGIN_CONSTANTS: u32 = 0xf8;
    pub const BEGIN_TEXTURES: u32 = 0x100;
    pub const BEGIN_DONE: u32 = 0xf4;
    pub const END: u32 = 0x104;
    pub const END_RESTORE: u32 = 0x114;
    pub const GEOMETRY_A: u32 = 0x78;
    pub const GEOMETRY_B: u32 = 0x7c;
    pub const SECOND: u32 = 0x80;
    pub const FIRST: u32 = 0x84;
    pub const THIRD: u32 = 0x88;
    pub const DRAW: u32 = 0x6c;
    pub const GEOMETRY_DONE: u32 = 0x8c;
    pub const RESTORE: u32 = 0x110;
    pub const PARTITION: u32 = 0x64;
    /// The geometry's own: render (0xdc), draws itself (0x30, 0x34), set the holder (0xe0).
    pub const OWN_RENDER: u32 = 0xdc;
    pub const OWN_A: u32 = 0x30;
    pub const OWN_B: u32 = 0x34;
    pub const SET_HOLDER: u32 = 0xe0;
    /// A batch renderer's `AddPass` ([`super::add_to_group`]) and a map's hash.
    pub const ADD_PASS: u32 = 0x8c;
    pub const MAP_HASH: u32 = 4;
    /// A skin instance's type.
    pub const SKIN_TYPE: u32 = 8;
}

/// List `index` of group `group`.
#[inline(always)]
fn group_list(e: &Engine, this: Ptr<BSBatchRenderer>, group: u32, index: u32) -> Ptr<PassList> {
    let groups = e.get(this, BSBatchRenderer::pRenderPassA);
    Ptr::new(
        group
            .wrapping_mul(0x68)
            .wrapping_add(index.wrapping_mul(0x14))
            .wrapping_add(groups),
    )
}

/// The map's value for `key` (`GetAt` into a stack word).
#[inline(always)]
fn map_get(e: &mut Engine, this: Ptr<BSBatchRenderer>, key: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(MAP_GET_AT, &args![this.addr().wrapping_add(0xc), key, slot]);
        e.global::<u32>(slot.addr())
    })
}

// Typed from 00b98ae0
/// `thiscall(renderer, group, out)`: the first of the group's five lists with a head; its index goes
/// to `out`. False when none has one.
pub fn first_list(e: &mut Engine, this: Ptr<BSBatchRenderer>, group: u32, out: u32) -> bool {
    let groups = e.get(this, BSBatchRenderer::pRenderPassA);
    for index in 0..5u32 {
        let list = group
            .wrapping_mul(0x68)
            .wrapping_add(index * 0x14)
            .wrapping_add(groups);
        if e.global::<u32>(list.wrapping_add(4)) != 0 {
            e.set_global(out, index);
            return true;
        }
    }
    false
}

// Typed from 00b99730
/// `thiscall(active list)`: drops the head entry: the second entry's type and link move into the head
/// (or the head's type clears when there is none) and the second is freed.
pub fn pop_active(e: &mut Engine, this: Ptr<ActivePass>) {
    let second = Ptr::<ActivePass>::new(e.get(this, ActivePass::pNext));
    if second.is_null() {
        e.set(this, ActivePass::usItem, 0);
        return;
    }
    let next = e.get(second, ActivePass::pNext);
    e.set(this, ActivePass::pNext, next);
    let item = e.get(second, ActivePass::usItem);
    e.set(this, ActivePass::usItem, item);
    e.set(second, ActivePass::pNext, 0);
    e.call(ACTIVE_LIST_CLEAR, &args![second]);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, second]);
}

// Typed from 00b99900
/// `thiscall(renderer, type out, list out)`: takes the next active pass type at or above the first
/// one drawn (+0x20) into `pass_type`; false when there is none or it is above the last (+0x24).
/// Otherwise the entry is dropped and the answer is whether its group has a list with passes.
pub fn next_active(e: &mut Engine, this: Ptr<BSBatchRenderer>, pass_type: u32, out: u32) -> bool {
    let head = this.at(BSBatchRenderer::kActivePassIndexList);
    let mut node = head;
    if e.get(node, ActivePass::pNext) == 0 && e.get(node, ActivePass::usItem) == 0 {
        return false;
    }
    let mut value = u32::from(e.get(node, ActivePass::usItem));
    e.set_global(pass_type, value);
    while value < e.get(this, BSBatchRenderer::uiFirstPass) {
        node = Ptr::new(e.get(node, ActivePass::pNext));
        if node.is_null() {
            return false;
        }
        value = u32::from(e.get(node, ActivePass::usItem));
        e.set_global(pass_type, value);
    }
    if value > e.get(this, BSBatchRenderer::uiLastPass) {
        return false;
    }
    typed_call!(FAMILY, 0x00b9_9730, pop_active(e, head));
    let key = e.global::<u32>(pass_type);
    let group = map_get(e, this, key);
    typed_call!(FAMILY, 0x00b9_8ae0, first_list(e, this, group, out))
}

// Typed from 00b9a090
/// `thiscall(renderer, type, list out, flag out)`: makes sure a list is ready to draw: the current
/// type's group when it still has a list with passes, else the next active type's. The sort byte of
/// the list's first pass goes to `flag`. False when nothing is left.
pub fn next_group(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_type: u32,
    element: u32,
    flag: u32,
) -> bool {
    e.with_stack(4, |e, slot| {
        let mut answer = true;
        let key = e.global::<u32>(pass_type);
        let mut ready = false;
        if key != 0 {
            e.call(MAP_GET_AT, &args![this.addr().wrapping_add(0xc), key, slot]);
            let group = e.global::<u32>(slot.addr());
            ready = typed_call!(FAMILY, 0x00b9_8ae0, first_list(e, this, group, element));
        }
        if !ready {
            answer = typed_call!(
                FAMILY,
                0x00b9_9900,
                next_active(e, this, pass_type, element)
            );
            if !answer {
                return false;
            }
        }
        let key = e.global::<u32>(pass_type);
        e.call(MAP_GET_AT, &args![this.addr().wrapping_add(0xc), key, slot]);
        let index = e.global::<u32>(element);
        let group = e.global::<u32>(slot.addr());
        let list = group_list(e, this, group, index);
        let node = Ptr::<PassNode>::new(e.get(list, PassList::pHead));
        let first = Ptr::<RenderPass>::new(e.get(node, PassNode::pItem));
        let byte = e.get(first, RenderPass::ucSortFlag);
        e.set_global(flag, byte);
        answer
    })
}

// Typed from 00b9a120
/// `thiscall(renderer, type, list, flag out, second)`: draws the run of passes at the head of the
/// current list that share the first one's sort byte, removing each from the list (its node to the
/// list's spare chain). The list index picks the cull mode, the alpha test and alpha to coverage for
/// the run (lists 2 to 4 blend). A pass with another sort byte stops the run with its byte in `flag`
/// (true); else the next group is made ready ([`next_group`]).
pub fn render_group(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_type: u32,
    element: u32,
    flag: u32,
    second: u32,
) -> bool {
    let value = map_get(e, this, e.global::<u32>(pass_type));
    let index = e.global::<u32>(element);
    let list = group_list(e, this, value, index);
    let head = Ptr::<PassNode>::new(e.get(list, PassList::pHead));
    let first = Ptr::<RenderPass>::new(e.get(head, PassNode::pItem));
    let first_flag = e.get(first, RenderPass::ucSortFlag);
    let mut blend = 0u8;
    // Lists 0 and 1: no alpha test, no coverage.
    for (element_index, mode) in [(0u32, 1u32), (1, 3)] {
        if e.global::<u32>(element) == element_index {
            typed_call!(
                FAMILY,
                0x00b9_84d0,
                state::set_cull_mode_of_renderer(e, mode, element_index)
            );
            typed_call!(
                FAMILY,
                0x00b9_7ed0,
                state::set_alpha_test_enable(e, 0u8, 1u32)
            );
            typed_call!(
                FAMILY,
                0x00b9_8540,
                state::set_alpha_to_coverage(e, 0u8, 1u32)
            );
        }
    }
    for (element_index, mode, coverage) in [(2u32, 3u32, 1u8), (3, 1, 1), (4, 1, 0)] {
        if e.global::<u32>(element) == element_index {
            typed_call!(
                FAMILY,
                0x00b9_84d0,
                state::set_cull_mode_of_renderer(e, mode, 1u32)
            );
            typed_call!(
                FAMILY,
                0x00b9_7ed0,
                state::set_alpha_test_enable(e, 1u8, 1u32)
            );
            blend = 1;
            if e.global::<u8>(COVERAGE_AVAILABLE) != 0 {
                typed_call!(
                    FAMILY,
                    0x00b9_8540,
                    state::set_alpha_to_coverage(e, coverage, 1u32)
                );
            }
        }
    }
    // The exe keeps the blend byte in the low byte of the map value's stack word and passes the word.
    let blend_word = (value & 0xffff_ff00) | u32::from(blend);
    let group = || value.wrapping_mul(0x68);
    let mut stopped = false;
    let list = group_list(e, this, value, e.global::<u32>(element));
    let mut pass = Ptr::<RenderPass>::new(e.get(
        Ptr::<PassNode>::new(e.get(list, PassList::pHead)),
        PassNode::pItem,
    ));
    if !pass.is_null() {
        loop {
            if e.get(pass, RenderPass::ucSortFlag) != first_flag {
                let byte = e.get(pass, RenderPass::ucSortFlag);
                e.set_global(flag, byte);
                stopped = true;
                break;
            }
            let key = e.global::<u32>(pass_type);
            typed_call!(
                FAMILY,
                0x00b9_94f0,
                render_pass(e, pass, key, blend_word, second, 0u32)
            );
            let index = e.global::<u32>(element);
            let group_at = e
                .get(this, BSBatchRenderer::pRenderPassA)
                .wrapping_add(group());
            let list = Ptr::<PassList>::new(group_at.wrapping_add(index.wrapping_mul(0x14)));
            let node = Ptr::<PassNode>::new(e.get(list, PassList::pHead));
            if !node.is_null() {
                if node.addr() == e.get(list, PassList::pTail) {
                    e.set(list, PassList::pTail, 0);
                }
                let mut next = 0;
                if e.get(node, PassNode::pNext) != 0 {
                    next = e.get(node, PassNode::pNext);
                    e.set(Ptr::<PassNode>::new(next), PassNode::pPrev, 0);
                }
                let spare = e.get(list, PassList::pSpare);
                if spare != 0 {
                    let head = e.get(list, PassList::pHead);
                    e.set(Ptr::<PassNode>::new(spare), PassNode::pPrev, head);
                }
                let head = Ptr::<PassNode>::new(e.get(list, PassList::pHead));
                let spare = e.get(list, PassList::pSpare);
                e.set(head, PassNode::pNext, spare);
                let head = e.get(list, PassList::pHead);
                e.set(list, PassList::pSpare, head);
                let count = e.get(list, PassList::uiCount).wrapping_sub(1);
                e.set(list, PassList::uiCount, count);
                e.set(list, PassList::pHead, next);
            }
            let group_ptr = Ptr::<PassGroup>::new(group_at);
            let total = e.get(group_ptr, PassGroup::uiCount).wrapping_sub(1);
            e.set(group_ptr, PassGroup::uiCount, total);
            let list = group_list(e, this, value, e.global::<u32>(element));
            let head = Ptr::<PassNode>::new(e.get(list, PassList::pHead));
            if head.is_null() {
                break;
            }
            pass = Ptr::new(e.get(head, PassNode::pItem));
            if pass.is_null() {
                break;
            }
        }
    }
    for counter in GROUP_COUNTERS {
        let count = e.global::<u32>(counter);
        if count != 0 {
            e.set_global(counter, count - 1);
        }
    }
    if e.global::<u8>(COVERAGE_AVAILABLE) != 0 {
        let count = e.global::<u32>(state::COVERAGE_COUNTER).wrapping_sub(1);
        e.set_global(state::COVERAGE_COUNTER, count);
        typed_call!(
            FAMILY,
            0x00b9_8540,
            state::set_alpha_to_coverage(e, 0u8, 0u32)
        );
    }
    typed_call!(
        FAMILY,
        0x00b9_8540,
        state::set_alpha_to_coverage(e, 0u8, 0u32)
    );
    if stopped {
        return true;
    }
    typed_call!(
        FAMILY,
        0x00b9_a090,
        next_group(e, this, pass_type, element, flag)
    )
}

// Typed from 00b994f0
/// `BSBatchRenderer::RenderPassImmediately` (engine map), `cdecl(pass, type, first, second, third)`:
/// only for a pass of that type. Marks it the one being drawn, begins the shader's pass when the type
/// or shader changed (or for type 0x2f6), sets alpha to coverage from its alpha property where the card
/// has it, then draws it: skinned, by its own render (when it draws itself), or plain. A pass begun here
/// ends with the shader's tail (types 1 to 5 and 0x11).
pub fn render_pass(
    e: &mut Engine,
    pass: Ptr<RenderPass>,
    pass_type: u32,
    first: u32,
    second: u32,
    third: u32,
) {
    if u32::from(e.get(pass, RenderPass::usPassEnum)) != pass_type {
        return;
    }
    e.set_global(RENDERING_PASS_TYPE, pass_type);
    e.set_global(RENDERING_PASS, pass.addr());
    let geometry = e.get(pass, RenderPass::pGeometry);
    let shader = e.get(geometry, NiGeometry::m_spShader);
    let mut begun = false;
    if e.global::<u32>(CURRENT_PASS_TYPE) != pass_type
        || pass_type == 0x2f6
        || shader != e.global::<u32>(CURRENT_SHADER)
    {
        begun = true;
        typed_call!(FAMILY, 0x00b9_9390, begin_pass(e, pass_type, shader));
    }
    if e.global::<u8>(COVERAGE_AVAILABLE) != 0 {
        let geometry = e.get(pass, RenderPass::pGeometry);
        let alpha = Ptr::<NiProperty>::new(e.global::<u32>(elem_addr(
            geometry,
            NiGeometry::m_PropertyState,
            0,
            4,
        )));
        let on = !alpha.is_null() && (e.get(alpha, NiProperty::m_usFlags) >> 9) & 1 != 0;
        typed_call!(
            FAMILY,
            0x00b9_8540,
            state::set_alpha_to_coverage(e, u8::from(on), 0u32)
        );
    }
    let renderer = e.global::<u32>(state::RENDERER);
    let geometry = e.get(pass, RenderPass::pGeometry);
    if e.call(RENDERER_IS_SKINNED, &args![renderer, geometry, 0u32])
        .u8()
        != 0
    {
        typed_call!(
            FAMILY,
            0x00b9_9110,
            render_skinned(e, pass, first, second, third)
        );
    } else {
        let geometry = e.get(pass, RenderPass::pGeometry);
        let own = geometry_word(e, geometry.addr(), slot::OWN_A) != 0 || {
            let geometry = e.get(pass, RenderPass::pGeometry);
            geometry_word(e, geometry.addr(), slot::OWN_B) != 0
        };
        if own {
            let geometry = e.get(pass, RenderPass::pGeometry);
            let holder = e.global::<u32>(RENDER_HOLDER);
            e.vcall(geometry.addr(), slot::OWN_RENDER, &args![holder]);
        } else {
            typed_call!(
                FAMILY,
                0x00b9_8e80,
                render_standard(e, pass, first, second, third)
            );
        }
    }
    if begun {
        let shader = e.global::<u32>(CURRENT_SHADER);
        if shader != 0 {
            #[allow(clippy::cast_possible_wrap)]
            let kind = e.global::<u32>(shader.wrapping_add(0x68)) as i32;
            if (1..=5).contains(&kind) {
                e.call(TAIL_KINDS_1_TO_5, &args![shader, pass_type]);
            } else if kind == 0x11 {
                e.call(TAIL_KIND_11, &args![shader, pass_type]);
            }
        }
    }
}

/// `00e88dc0` with the renderer's group manager and group, after the renderer's current property
/// state (+0xc) is set to the geometry's and its effects (+0x10) cleared. Answers the renderer.
fn add_to_geometry_group(
    e: &mut Engine,
    geometry: Ptr<NiGeometry>,
    data: Ptr<NiGeometryData>,
    skin: Ptr<NiSkinInstance>,
    partitions: u8,
) -> u32 {
    use super::layout::NiDX9Renderer as R;
    let holder = Ptr::<R>::new(e.global::<u32>(RENDER_HOLDER));
    let group = e.get(holder, R::pGeometryGroup);
    let manager = e.get(holder, R::pGeometryGroupManager);
    e.set(
        holder,
        R::m_pkCurrProp,
        geometry
            .addr()
            .wrapping_add(NiGeometry::m_PropertyState.off),
    );
    e.set(holder, R::m_pkCurrEffects, 0);
    typed_call!(
        FAMILY,
        0x00e8_8dc0,
        state::add_object_to_group(e, manager, group, data, skin, partitions, 0u32, 0u32)
    );
    holder.addr()
}

// Typed from 00b98e80
/// `cdecl(pass, first, second, third)`: draws a plain geometry: into the geometry group, then the
/// shader's per-geometry set-up (four types have their own set-up functions; the others the slots
/// 0x78 and 0x7c, the first and second slots unless the pass is a shadow pass, and the owed restore),
/// the draw (slot 0x6c), the holder to the geometry, and the shader's geometry end (slot 0x8c). The
/// flags are the low bytes of their words.
pub fn render_standard(e: &mut Engine, pass: Ptr<RenderPass>, first: u32, second: u32, third: u32) {
    let (first, second, third) = (low8(first) != 0, low8(second) != 0, low8(third) != 0);
    let pass_type = u32::from(e.get(pass, RenderPass::usPassEnum));
    let geometry = e.get(pass, RenderPass::pGeometry);
    let data = e.get(geometry, NiGeometry::m_spModelData);
    let shader = e.get(geometry, NiGeometry::m_spShader);
    let state = geometry
        .addr()
        .wrapping_add(NiGeometry::m_PropertyState.off);
    let holder = add_to_geometry_group(e, geometry, data, Ptr::new(0), 0);
    let buffer = e.get(data, NiGeometryData::m_pkBuffData);
    let call_third = |e: &mut Engine| {
        if third {
            let flag = u32::from(e.get(pass, RenderPass::ucSortFlag));
            e.vcall(shader, slot::THIRD, &args![state, flag]);
        }
    };
    let first_then_second = |e: &mut Engine| {
        if first {
            e.vcall(shader, slot::FIRST, &args![state]);
        }
        if second {
            e.vcall(shader, slot::SECOND, &args![state]);
        }
    };
    let own = match pass_type {
        0xfc | 0xfd => Some((SETUP_TYPES_FC_FD, false)),
        0x102 => Some((SETUP_TYPE_102, false)),
        0xca => Some((SETUP_TYPE_CA, true)),
        0xd1 => Some((SETUP_TYPE_D1, true)),
        _ => None,
    };
    if let Some((setup, then_third)) = own {
        if setup == SETUP_TYPES_FC_FD {
            typed_call!(
                crate::render::FAMILY,
                0x00b7_dde0,
                crate::render::pass::geometry_pass(e, Ptr::new(shader), Ptr::new(state))
            );
        } else {
            e.call(setup, &args![shader, state]);
        }
        first_then_second(e);
        if then_third {
            call_third(e);
        }
    } else {
        e.vcall(shader, slot::GEOMETRY_A, &args![state]);
        e.vcall(shader, slot::GEOMETRY_B, &args![state]);
        if e.get(pass, RenderPass::ucSortFlag) != 0 {
            if second && e.call(IS_SHADOW_PASS, &args![pass_type]).u8() == 0 {
                e.vcall(shader, slot::SECOND, &args![state]);
            }
            if first
                && !(4..=5).contains(&pass_type)
                && !(0xe..=0xf).contains(&pass_type)
                && pass_type != 0x23a
            {
                e.vcall(shader, slot::FIRST, &args![state]);
            }
        } else if e.global::<u8>(RESTORE_OWED) != 0
            && e.call(IS_SHADOW_PASS, &args![pass_type]).u8() == 0
        {
            e.vcall(shader, slot::RESTORE, &args![state]);
            e.set_global(RESTORE_OWED, 0u8);
        }
        call_third(e);
    }
    shader_draw(e, shader, geometry, 0u32, buffer, state);
    e.vcall(geometry.addr(), slot::SET_HOLDER, &args![holder]);
    e.vcall(shader, slot::GEOMETRY_DONE, &args![state]);
}

// Typed from 00b99110
/// `cdecl(pass, first, second, third)`: draws a skinned geometry partition by partition (only the
/// partitions a dismember skin marks drawn): each starts with the shader's partition set-up (slot 0x64,
/// with the world bound), the first one also with the per-geometry set-up; each is drawn (slot 0x6c)
/// and handed to the holder (`00e6d310`), and a run of drawn partitions that don't start a run is
/// drawn with it. A draw that answers 0 inside a run ends the whole call. The model data's dirty flags
/// clear after, and the shader's geometry end runs.
pub fn render_skinned(e: &mut Engine, pass: Ptr<RenderPass>, first: u32, second: u32, third: u32) {
    let (first, second, third) = (low8(first) != 0, low8(second) != 0, low8(third) != 0);
    let pass_type = u32::from(e.get(pass, RenderPass::usPassEnum));
    let geometry = e.get(pass, RenderPass::pGeometry);
    let skin = e.get(geometry, NiGeometry::m_spSkinInstance);
    let data = e.get(geometry, NiGeometry::m_spModelData);
    let partition_data = e.get(skin, NiSkinInstance::m_spSkinPartition);
    let shader = e.get(geometry, NiGeometry::m_spShader);
    let state = geometry
        .addr()
        .wrapping_add(NiGeometry::m_PropertyState.off);
    let partitions = e.get(partition_data, NiSkinPartition::m_pkPartitions);
    let holder = add_to_geometry_group(e, geometry, data, skin, 1);
    let count = e.get(partition_data, NiSkinPartition::m_uiPartitions);
    let mut flags = 0u32;
    if e.vcall(skin.addr(), slot::SKIN_TYPE, &[]).u32() == SKIN_RTTI {
        flags = e.get(skin, NiSkinInstance::pPartitionFlags);
    }
    let mut first_partition = true;
    let mut index = 0u32;
    while index < count {
        if flags == 0 || e.global::<u8>(flags.wrapping_add(index * 4)) != 0 {
            let mut partition = partitions.wrapping_add(index.wrapping_mul(0x2c));
            let word = e.global::<u32>(partition.wrapping_add(0x28));
            let bound = typed_call!(leaf::FAMILY, 0x0043_d450, leaf::world_bound(e, geometry));
            e.vcall(
                shader,
                slot::PARTITION,
                &args![
                    geometry,
                    skin,
                    partition,
                    word,
                    state,
                    0u32,
                    geometry.addr().wrapping_add(NiGeometry::m_kWorld.off),
                    bound
                ],
            );
            if first_partition {
                e.vcall(shader, slot::GEOMETRY_A, &args![state]);
                e.vcall(shader, slot::GEOMETRY_B, &args![state]);
                if e.get(pass, RenderPass::ucSortFlag) != 0
                    && e.call(IS_SHADOW_PASS, &args![pass_type]).u8() == 0
                {
                    if second {
                        e.vcall(shader, slot::SECOND, &args![state]);
                    }
                    if first && !(4..=5).contains(&pass_type) && !(0xe..=0xf).contains(&pass_type) {
                        e.vcall(shader, slot::FIRST, &args![state]);
                    }
                } else if e.global::<u8>(RESTORE_OWED) != 0 {
                    e.vcall(shader, slot::RESTORE, &args![state]);
                    e.set_global(RESTORE_OWED, 0u8);
                }
                if third {
                    let flag = u32::from(e.get(pass, RenderPass::ucSortFlag));
                    e.vcall(shader, slot::THIRD, &args![state, flag]);
                }
                first_partition = false;
            }
            let drawn = shader_draw(e, shader, geometry, partition, word, state);
            e.call(HOLDER_DRAW, &args![holder, drawn, partition, data]);
            if flags != 0 {
                let mut next = index + 1;
                while next < count {
                    if e.global::<u8>(flags.wrapping_add(index * 4 + 5)) != 0 {
                        break;
                    }
                    index += 1;
                    next += 1;
                    partition = partition.wrapping_add(0x2c);
                    if e.global::<u8>(flags.wrapping_add(index * 4)) != 0 {
                        let word = e.global::<u32>(partition.wrapping_add(0x28));
                        let drawn = shader_draw(e, shader, geometry, partition, word, state);
                        if drawn == 0 {
                            return;
                        }
                        e.call(HOLDER_DRAW, &args![holder, drawn, partition, data]);
                    }
                }
            }
        }
        index += 1;
    }
    let data = e.get(geometry, NiGeometry::m_spModelData);
    let bits = e.get(data, NiGeometryData::m_usDirtyFlags) & 0xf000;
    e.set(data, NiGeometryData::m_usDirtyFlags, bits);
    e.vcall(shader, slot::GEOMETRY_DONE, &args![state]);
}

// Typed from 00b99390
/// `cdecl(type, shader)`: begins a shader's pass: the pass before ends, the marker resets, the shader
/// begins (slot 0xf0), the stage textures clear, then the shader's own begin for four type groups (types
/// 0xfc and 0xfd also switch the depth test on) or its states, constants and textures (slots 0xfc, 0xf8,
/// 0x100), and the begin's end (0xf4). The type and shader become the current ones; a restore is owed.
pub fn begin_pass(e: &mut Engine, pass_type: u32, shader: u32) {
    typed_call!(FAMILY, 0x00b9_88e0, end_pass(e));
    e.set_global(PASS_MARKER, 0xffffu16);
    e.vcall(shader, slot::BEGIN, &args![pass_type]);
    for i in 0..16 {
        e.set_global(STAGE_TEXTURES + i * 4, 0u32);
    }
    match pass_type {
        0xca | 0xd1 => {
            e.call(BEGIN_TYPES_CA_D1, &args![shader]);
        }
        0xfc | 0xfd => {
            e.call(BEGIN_TYPES_FC_102, &args![shader]);
            typed_call!(FAMILY, 0x00b9_7de0, state::set_z_enable(e, 1u8, 0u32));
        }
        0x102 => {
            e.call(BEGIN_TYPES_FC_102, &args![shader]);
        }
        _ => {
            e.vcall(shader, slot::BEGIN_STATES, &args![pass_type]);
            e.vcall(shader, slot::BEGIN_CONSTANTS, &args![pass_type]);
            e.vcall(shader, slot::BEGIN_TEXTURES, &args![pass_type]);
        }
    }
    e.vcall(shader, slot::BEGIN_DONE, &args![pass_type]);
    e.set_global(CURRENT_PASS_TYPE, pass_type);
    e.set_global(CURRENT_SHADER, shader);
    e.set_global(RESTORE_OWED, 1u8);
}

// Typed from 00b988e0
/// `BSBatchRenderer::EndPass` (engine map), no arguments: with a current shader, its pass ends (slot
/// 0x104 with the type) with the restore (slot 0x114) unless one is still owed; types 0xfe and 0x10b
/// set sampler 7's, and 0x230 sampler 2's, address mode back (state 8 to 0) on the device. The current
/// type and shader clear.
pub fn end_pass(e: &mut Engine) {
    let shader = e.global::<u32>(CURRENT_SHADER);
    if shader != 0 {
        let pass_type = e.global::<u32>(CURRENT_PASS_TYPE);
        e.vcall(shader, slot::END, &args![pass_type]);
        if e.global::<u8>(RESTORE_OWED) == 0 {
            let shader = e.global::<u32>(CURRENT_SHADER);
            e.vcall(shader, slot::END_RESTORE, &[]);
        }
        let sampler = match e.global::<u32>(CURRENT_PASS_TYPE) {
            0xfe | 0x10b => Some(7u32),
            0x230 => Some(2),
            _ => None,
        };
        if let Some(sampler) = sampler {
            use super::layout::NiDX9Renderer as R;
            let holder = Ptr::<R>::new(e.global::<u32>(RENDER_HOLDER));
            let device = e.get(holder, R::m_pkD3DDevice9);
            e.vcall(
                device,
                DEVICE_SET_SAMPLER_STATE,
                &args![sampler, 8u32, 0u32],
            );
        }
    }
    e.set_global(CURRENT_PASS_TYPE, 0u32);
    e.set_global(CURRENT_SHADER, 0u32);
}

/// The distance of a pass's world bound along the camera's direction (as the exe sums it: y, x, z),
/// stored to a float; minus the bound's radius (stored again) when asked. `second` is the order of
/// the comparison's second operand, whose last product takes its z as a memory operand.
#[inline(always)]
fn depth(e: &Engine, item: u32, d: [f64; 3], subtract: bool, second: bool) -> f32 {
    let pass = Ptr::<RenderPass>::new(e.global::<u32>(item));
    let geometry = e.get(pass, RenderPass::pGeometry);
    let mut bound = Ptr::<NiBound>::new(e.get(geometry, NiGeometry::m_kWorldBound));
    if bound.is_null() {
        bound = Ptr::new(NULL_BOUND);
    }
    let centre = bound.addr();
    let x = e.global::<u32>(centre);
    let y = e.global::<u32>(centre.wrapping_add(4));
    let z = e.global::<u32>(centre.wrapping_add(8));
    let sum = if second {
        let q1 = x87_mul(d[1], x87_load32(y));
        let q0 = x87_mul(d[0], x87_load32(x));
        let s = x87_add(q0, q1);
        x87_add(s, x87_mul(d[2], x87_m32(z)))
    } else {
        let p1 = x87_mul(x87_load32(y), d[1]);
        let p0 = x87_mul(x87_load32(x), d[0]);
        let s = x87_add(p0, p1);
        x87_add(x87_mul(x87_load32(z), d[2]), s)
    };
    let stored = fstp(sum);
    if subtract {
        let radius = e.get(bound, NiBound::m_fRadius).to_bits();
        let value = x87_sub(fld_bits(stored), x87_m32(radius));
        fstp(value)
    } else {
        stored
    }
}

#[inline(always)]
fn fld_bits(value: f32) -> f64 {
    x87_load32(value.to_bits())
}

// Typed from 00b98b20
/// `cdecl(first, second)`, the alpha sort's comparison of two list items (pointers to passes): each
/// pass's depth along the camera's direction (+0x68, +0x74, +0x80), less its radius when asked
/// (`011ffe35`). 0 when equal, 1 when the second is deeper, -1 otherwise (a NaN included).
pub fn compare_by_depth(e: &mut Engine, first: u32, second: u32) -> i32 {
    let subtract = e.global::<u8>(SORT_SUBTRACT_RADIUS) != 0;
    let camera = e.global::<u32>(SORT_CAMERA);
    let direction = |off: u32| x87_load32(e.global::<u32>(camera.wrapping_add(off)));
    let d = [direction(0x68), direction(0x74), direction(0x80)];
    let a = fld_bits(depth(e, first, d, subtract, false));
    let b = fld_bits(depth(e, second, d, subtract, true));
    if b == a {
        0
    } else if b > a {
        1
    } else {
        -1
    }
}

// Typed from 00b99a10
/// `thiscall(renderer, camera, subtract)`: sorts the two alpha holders' lists (+0x70, +0x74) back to
/// front by [`compare_by_depth`] for the camera.
pub fn sort_alphas(e: &mut Engine, this: Ptr<BSBatchRenderer>, camera: u32, subtract: u8) {
    e.set_global(SORT_SUBTRACT_RADIUS, subtract);
    e.set_global(SORT_CAMERA, camera);
    for index in 0..2 {
        let holder = e.global::<u32>(elem_addr(this, BSBatchRenderer::pAlphaGroups, index, 4));
        let list = Ptr::<PassHolder>::new(holder).at(PassHolder::kList);
        typed_call!(FAMILY, 0x00b9_9630, sort_list(e, list, COMPARE_BY_DEPTH));
    }
}

/// The shader's draw (slot 0x6c, `thiscall(shader, geometry, partition, buffer data, state)`):
/// `00e812f0` (the render family's `prepare_streams`) directly when the slot holds it, as every
/// `NiD3DShader` class has; another target through the vtable.
#[inline(always)]
fn shader_draw(
    e: &mut Engine,
    shader: u32,
    geometry: impl OneWord,
    partition: impl OneWord,
    buffer: impl OneWord,
    state: impl OneWord,
) -> u32 {
    let (g, p, b, s) = (
        geometry.one_word(),
        partition.one_word(),
        buffer.one_word(),
        state.one_word(),
    );
    if crate::slot_target(e, shader, slot::DRAW) == 0x00e8_12f0 {
        typed_call!(
            crate::render::FAMILY,
            0x00e8_12f0,
            crate::render::stream::prepare_streams(e, Ptr::new(shader), g, p, b, s)
        )
    } else {
        e.vcall(shader, slot::DRAW, &args![g, p, b, s]).u32()
    }
}

/// A geometry's word-answering slot with no arguments (0x30, 0x34): the leaf `00acbb70` (0) that
/// every geometry class holds there, directly; another target through the vtable.
#[inline(always)]
fn geometry_word(e: &mut Engine, geometry: u32, slot: u32) -> u32 {
    if crate::slot_target(e, geometry, slot) == 0x00ac_bb70 {
        typed_call!(leaf::FAMILY, 0x00ac_bb70, leaf::zero_of_this(e, geometry))
    } else {
        e.vcall(geometry, slot, &[]).u32()
    }
}

/// The comparison of two nodes' items: the alpha sort's by a Rust call when it is the one asked for.
#[inline(always)]
fn compare_items(e: &mut Engine, compare: u32, p: u32, q: u32) -> i32 {
    let (a, b) = (p.wrapping_add(8), q.wrapping_add(8));
    if compare == COMPARE_BY_DEPTH {
        typed_call!(FAMILY, 0x00b9_8b20, compare_by_depth(e, a, b))
    } else {
        e.call(compare, &args![a, b]).i32()
    }
}

// Typed from 00b99630
/// `thiscall(list, compare)`: a merge sort of the list in place (runs doubling), the item with the
/// comparison above 0 for the second run going first; the spare chain is released first.
pub fn sort_list(e: &mut Engine, this: Ptr<PassList>, compare: u32) {
    typed_call!(FAMILY, 0x00b6_3e90, release_spare(e, this));
    if e.get(this, PassList::pHead) == 0 {
        return;
    }
    let mut run = 1u32;
    let mut tail;
    loop {
        let mut p = e.get(this, PassList::pHead);
        e.set(this, PassList::pHead, 0);
        tail = 0u32;
        let mut merges = 0u32;
        while p != 0 {
            merges += 1;
            let mut p_size = 0i32;
            let mut q = p;
            let mut i = 0u32;
            if run != 0 {
                loop {
                    q = e.global::<u32>(q);
                    p_size += 1;
                    if q == 0 {
                        break;
                    }
                    i += 1;
                    if i >= run {
                        break;
                    }
                }
            }
            #[allow(clippy::cast_possible_wrap)]
            let mut q_size = run as i32;
            loop {
                let second_first = |e: &mut Engine, p: u32, q: u32, q_size: i32| {
                    q_size != 0 && q != 0 && compare_items(e, compare, p, q) > 0
                };
                let from_second = if p_size > 0 {
                    second_first(e, p, q, q_size)
                } else {
                    if q_size <= 0 || q == 0 {
                        break;
                    }
                    p_size == 0 || second_first(e, p, q, q_size)
                };
                let next;
                if from_second {
                    next = q;
                    q = e.global::<u32>(q);
                    q_size -= 1;
                } else {
                    next = p;
                    p = e.global::<u32>(p);
                    p_size -= 1;
                }
                if tail != 0 {
                    e.set_global(tail, next);
                } else {
                    e.set(this, PassList::pHead, next);
                }
                e.set_global(next.wrapping_add(4), tail);
                tail = next;
            }
            p = q;
        }
        run = run.wrapping_add(run);
        e.set_global(tail, 0u32);
        if merges == 1 {
            break;
        }
    }
    e.set(this, PassList::pTail, tail);
}

/// A node for `list`: the spare chain's first, or a new one from the pool allocator.
#[inline(always)]
fn take_node(e: &mut Engine, list: Ptr<PassList>) -> Ptr<PassNode> {
    let spare = e.get(list, PassList::pSpare);
    if spare == 0 {
        Ptr::new(
            e.call(
                LIST_NODE_ALLOCATE,
                &args![list.addr().wrapping_add(PassList::uiCount.off)],
            )
            .u32(),
        )
    } else {
        let next = e.global::<u32>(spare);
        e.set(list, PassList::pSpare, next);
        Ptr::new(spare)
    }
}

// Typed from 00b99840
/// `thiscall(list, item pointer)`: appends the item.
pub fn list_append(e: &mut Engine, this: Ptr<PassList>, item: u32) {
    let node = take_node(e, this);
    let value = e.global::<u32>(item);
    e.set(node, PassNode::pItem, value);
    e.set(node, PassNode::pNext, 0);
    let tail = e.get(this, PassList::pTail);
    e.set(node, PassNode::pPrev, tail);
    let tail = e.get(this, PassList::pTail);
    let count = e.get(this, PassList::uiCount).wrapping_add(1);
    if tail != 0 {
        e.set(Ptr::<PassNode>::new(tail), PassNode::pNext, node.addr());
        e.set(this, PassList::uiCount, count);
    } else {
        e.set(this, PassList::uiCount, count);
        e.set(this, PassList::pHead, node.addr());
    }
    e.set(this, PassList::pTail, node.addr());
}

// Typed from 00b99890
/// `thiscall(list, item pointer)`: prepends the item.
pub fn list_prepend(e: &mut Engine, this: Ptr<PassList>, item: u32) {
    let node = take_node(e, this);
    let value = e.global::<u32>(item);
    e.set(node, PassNode::pItem, value);
    let head = e.get(this, PassList::pHead);
    e.set(node, PassNode::pNext, head);
    e.set(node, PassNode::pPrev, 0);
    let head = e.get(this, PassList::pHead);
    let count = e.get(this, PassList::uiCount).wrapping_add(1);
    if head != 0 {
        e.set(Ptr::<PassNode>::new(head), PassNode::pPrev, node.addr());
        e.set(this, PassList::uiCount, count);
    } else {
        e.set(this, PassList::uiCount, count);
        e.set(this, PassList::pTail, node.addr());
    }
    e.set(this, PassList::pHead, node.addr());
}

// Typed from 00b9abc0
/// `thiscall(holder, pass, prepend)`: a holder with bit 0 keeps the pass in its list (first or last);
/// another hands it to its batch renderer's `AddPass` (made with the holder's size on first use). The
/// holder's count goes up either way.
pub fn add_pass(e: &mut Engine, this: Ptr<PassHolder>, pass: Ptr<RenderPass>, prepend: u32) {
    if e.get(this, PassHolder::ucFlags) & 1 != 0 {
        e.with_stack(4, |e, slot| {
            e.set_global(slot.addr(), pass.addr());
            let list = this.at(PassHolder::kList);
            if low8(prepend) != 0 {
                typed_call!(FAMILY, 0x00b9_9890, list_prepend(e, list, slot.addr()));
            } else {
                typed_call!(FAMILY, 0x00b9_9840, list_append(e, list, slot.addr()));
            }
        });
    } else {
        if e.get(this, PassHolder::pRenderer).is_null() {
            let block = e.call(OPERATOR_NEW, &args![0x98u32]).u32();
            let renderer = if block != 0 {
                let size = e.get(this, PassHolder::uiRendererSize);
                e.call(RENDERER_CONSTRUCT, &args![block, size, 100u32])
                    .u32()
            } else {
                0
            };
            e.set(this, PassHolder::pRenderer, Ptr::new(renderer));
        }
        let pass_type = u32::from(e.get(pass, RenderPass::usPassEnum));
        let renderer = e.get(this, PassHolder::pRenderer);
        e.vcall(renderer.addr(), slot::ADD_PASS, &args![pass, pass_type]);
    }
    let count = e.get(this, PassHolder::usCount).wrapping_add(1);
    e.set(this, PassHolder::usCount, count);
}

// Typed from 00b9b3b0
/// `thiscall(renderer, pass, group, prepend)`: [`add_pass`] to the holder of register group `group`.
pub fn register_pass(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass: Ptr<RenderPass>,
    group: u32,
    prepend: u32,
) -> u32 {
    let holder = Ptr::<PassHolder>::new(e.global::<u32>(elem_addr(
        this,
        BSBatchRenderer::pGroups,
        group,
        4,
    )));
    typed_call!(FAMILY, 0x00b9_abc0, add_pass(e, holder, pass, prepend));
    0
}

// Typed from 00b9b3d0
/// `thiscall(renderer, pass, flag)`: [`add_pass`] (appended) to the alpha holder of `flag`, the
/// grouped one while alphas are grouped (+0x34 above 0).
pub fn register_alpha(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass: Ptr<RenderPass>,
    flag: u32,
) -> u32 {
    let field = if e.get(this, BSBatchRenderer::iGroupingAlphas) > 0 {
        BSBatchRenderer::pGroupedAlphas.off
    } else {
        BSBatchRenderer::pAlphaGroups.off
    };
    let holder = Ptr::<PassHolder>::new(
        e.global::<u32>(
            this.addr()
                .wrapping_add(flag.wrapping_mul(4))
                .wrapping_add(field),
        ),
    );
    typed_call!(FAMILY, 0x00b9_abc0, add_pass(e, holder, pass, 0u32));
    0
}

/// Empties a list into its spare chain: the spare chain released to the pool, the nodes in use kept
/// as the new spare chain.
#[inline(always)]
fn reset_list(e: &mut Engine, list: Ptr<PassList>) {
    typed_call!(FAMILY, 0x00b6_3e90, release_spare(e, list));
    let head = e.get(list, PassList::pHead);
    e.set(list, PassList::pSpare, head);
    e.set(list, PassList::pHead, 0);
    e.set(list, PassList::pTail, 0);
    e.set(list, PassList::uiCount, 0);
}

// Typed from 00b9a490
/// `thiscall(holder)`: empties the holder's list and its batch renderer's groups; the count clears.
pub fn clear_holder(e: &mut Engine, this: Ptr<PassHolder>) {
    reset_list(e, this.at(PassHolder::kList));
    let renderer = e.get(this, PassHolder::pRenderer);
    if !renderer.is_null() {
        typed_call!(FAMILY, 0x00b9_9bc0, clear_renderer(e, renderer, 0u32));
    }
    e.set(this, PassHolder::usCount, 0);
}

/// The first item of the map's buckets from `start` on.
#[inline(always)]
fn map_item_from(e: &Engine, this: Ptr<BSBatchRenderer>, start: u32) -> u32 {
    let count = e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize);
    let buckets = e.get(this, BSBatchRenderer::RenderPassMap_m_ppkHashTable);
    (start..count)
        .map(|i| e.global::<u32>(buckets.wrapping_add(i * 4)))
        .find(|item| *item != 0)
        .unwrap_or(0)
}

// Typed from 00b99a50
/// `thiscall(group, unused)`: empties each of the group's five lists that has passes; the total clears.
pub fn clear_group(e: &mut Engine, this: Ptr<PassGroup>, _unused: u32) {
    for index in 0..5u32 {
        let list = Ptr::<PassList>::new(this.addr().wrapping_add(index * 0x14));
        if e.get(list, PassList::uiCount) != 0 {
            reset_list(e, list);
        }
    }
    e.set(this, PassGroup::uiCount, 0);
}

// Typed from 00b99bc0
/// `thiscall(renderer, unused)`: empties the group of every pass type in the map, then the active
/// list.
pub fn clear_renderer(e: &mut Engine, this: Ptr<BSBatchRenderer>, _unused: u32) {
    let mut item = if e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize) != 0 {
        map_item_from(e, this, 0)
    } else {
        0
    };
    while item != 0 {
        let link = e.global::<u32>(item);
        let group = e.global::<u32>(item.wrapping_add(8));
        let next = if link != 0 {
            link
        } else {
            let key = e.global::<u32>(item.wrapping_add(4));
            let map = this
                .addr()
                .wrapping_add(BSBatchRenderer::RenderPassMap_vtable.off);
            let index = e
                .vcall(map, slot::MAP_HASH, &args![key])
                .u32()
                .wrapping_add(1);
            if index < e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize) {
                map_item_from(e, this, index)
            } else {
                0
            }
        };
        let groups = e.get(this, BSBatchRenderer::pRenderPassA);
        let group = Ptr::<PassGroup>::new(group.wrapping_mul(0x68).wrapping_add(groups));
        for index in 0..5u32 {
            let list = Ptr::<PassList>::new(group.addr() + index * 0x14);
            if e.get(list, PassList::uiCount) != 0 {
                reset_list(e, list);
            }
        }
        e.set(group, PassGroup::uiCount, 0);
        item = next;
    }
    let active = this.at(BSBatchRenderer::kActivePassIndexList);
    e.call(ACTIVE_LIST_CLEAR, &args![active]);
}

// Typed from 00b63e90
/// `thiscall(list)`: returns the list's spare chain to the global node pool under the pool's lock
/// (taken once around the walk and once per node), and cuts the list after its tail. Answers the tail.
pub fn release_spare(e: &mut Engine, this: Ptr<PassList>) -> u32 {
    let mut node = e.get(this, PassList::pSpare);
    if node != 0 {
        e.call(LOCK_ENTER, &args![POOL_LOCK, 0u32]);
        loop {
            let current = node;
            node = e.global::<u32>(node);
            e.call(LOCK_ENTER, &args![POOL_LOCK, 0u32]);
            pool_give(e, current);
            if node == 0 {
                break;
            }
        }
        pool_unlock(e);
    }
    let tail = e.get(this, PassList::pTail);
    e.set(this, PassList::pSpare, 0);
    if tail != 0 {
        e.set_global(tail, 0u32);
    }
    tail
}

/// A node back to the pool's free list, the lock held once less.
#[inline(always)]
pub(crate) fn pool_give(e: &mut Engine, node: u32) {
    e.set_global(node.wrapping_add(4), 0u32);
    let free = e.global::<u32>(POOL_FREE);
    e.set_global(node, free);
    let count = e.global::<u32>(POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(POOL_LOCK_COUNT, count);
    e.set_global(POOL_FREE, node);
    if count == 0 {
        e.set_global(POOL_LOCK, 0u32);
    }
}

/// The pool's lock held once less.
#[inline(always)]
fn pool_unlock(e: &mut Engine) {
    let count = e.global::<u32>(POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(POOL_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(POOL_LOCK, 0u32);
    }
}

// Typed from 00b99c90
/// The `AddPass` of `BSBatchRenderer`, virtual slot 0x8c, `thiscall(renderer, pass, type)`: files the
/// pass under `pass_type` (or its own type when 0). The list: 4 when the shader property has bit 22 of
/// its second flags; else an alpha pass goes to 2 or 3 and another to 1 or 0, by whether the shade
/// property has both bits 10 and 11. A pass counts as alpha when its alpha property blends, its sort
/// byte is set and it isn't held back (1.0 above either alpha float, no bit 0, type not 2), and always
/// for the types 4, 5, 0xe, 0xf and 0x23a. A new type gets the next group index, a cleared group and
/// a place in the active list; the pass is prepended when its sort byte is set and the type isn't
/// 0x238 to 0x23e, else appended.
pub fn add_to_group(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass: Ptr<RenderPass>,
    pass_type: u32,
) {
    if pass.is_null() {
        return;
    }
    let geometry = e.get(pass, RenderPass::pGeometry);
    if geometry.is_null() {
        return;
    }
    let properties = e.get(geometry, NiGeometry::pShaderProperty);
    let shade = e.get(geometry, NiGeometry::pShadeProperty);
    let paired = !shade.is_null() && e.get(shade, NiProperty::m_usFlags) & 0xc00 == 0xc00;
    let alpha_property = Ptr::<NiProperty>::new(e.global::<u32>(elem_addr(
        geometry,
        NiGeometry::m_PropertyState,
        0,
        4,
    )));
    let mut alpha = false;
    if !alpha_property.is_null()
        && (e.get(alpha_property, NiProperty::m_usFlags) >> 9) & 1 != 0
        && e.get(pass, RenderPass::ucSortFlag) != 0
    {
        use super::layout::BSShaderProperty as P;
        alpha = true;
        let blocked =
            1.0 > m32(e, properties, P::fAlpha) || 1.0 > m32(e, properties, P::fFadeAlpha);
        if blocked && e.get(alpha_property, NiProperty::m_usFlags) & 1 == 0 && pass_type != 2 {
            alpha = false;
        }
    }
    if matches!(pass_type, 0xe | 0xf | 4 | 5 | 0x23a) {
        alpha = true;
    }
    let list = if e.get(properties, super::layout::BSShaderProperty::ulFlags2) & 0x40_0000 != 0 {
        4
    } else if alpha {
        u32::from(!paired) + 2
    } else {
        u32::from(paired)
    };
    let mut key = u32::from(e.get(pass, RenderPass::usPassEnum));
    if pass_type != 0 {
        key = pass_type;
    }
    let map = this
        .addr()
        .wrapping_add(BSBatchRenderer::RenderPassMap_vtable.off);
    // The out slot is the stack word of the `pass_type` argument.
    let (found, value) = e.with_stack(4, |e, slot| {
        e.set_global(slot.addr(), pass_type);
        let found = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (found, e.global::<u32>(slot.addr()))
    });
    let index = if found {
        value
    } else {
        let index = e.get(this, BSBatchRenderer::RenderPassMap_m_uiCount);
        e.call(MAP_SET_AT, &args![map, key, index]);
        let group = group_at(e, this, index);
        typed_call!(FAMILY, 0x00b9_9a50, clear_group(e, group, 0u32));
        index
    };
    let group = group_at(e, this, index);
    if e.get(group, PassGroup::uiCount) == 0 {
        let active = this.at(BSBatchRenderer::kActivePassIndexList);
        typed_call!(
            FAMILY,
            0x00b9_9770,
            insert_active(e, active, key, PASS_TYPE_COMPARE)
        );
    }
    let group = group_at(e, this, index);
    if e.get(pass, RenderPass::ucSortFlag) != 0 && !(0x238..=0x23e).contains(&key) {
        typed_call!(FAMILY, 0x00b9_9a90, group_prepend(e, group, list, pass));
    } else {
        typed_call!(FAMILY, 0x00b9_9b00, group_append(e, group, list, pass));
    }
}

#[inline(always)]
fn group_at(e: &Engine, this: Ptr<BSBatchRenderer>, index: u32) -> Ptr<PassGroup> {
    let groups = e.get(this, BSBatchRenderer::pRenderPassA);
    Ptr::new(index.wrapping_mul(0x68).wrapping_add(groups))
}

// Typed from 00b99770
/// `thiscall(active list, type, compare)`: puts a pass type into the active list in the comparison's
/// order: into an empty head, after the last entry it compares above, or before the first it doesn't
/// (at the head by `00b98e20`). A type of 0 does nothing.
pub fn insert_active(e: &mut Engine, this: Ptr<ActivePass>, item: u32, compare: u32) {
    let value = item & 0xffff;
    if value == 0 {
        return;
    }
    let mut previous = 0u32;
    let mut node = this.addr();
    while node != 0 {
        let entry = Ptr::<ActivePass>::new(node);
        let current = e.get(entry, ActivePass::usItem);
        if current == 0 {
            e.set(entry, ActivePass::usItem, low16(value));
            break;
        }
        if e.call(compare, &args![item, u32::from(current)]).i32() > 0 {
            if e.get(entry, ActivePass::pNext) == 0 {
                let added = new_entry(e, value);
                e.set(entry, ActivePass::pNext, added);
                break;
            }
        } else {
            if previous == 0 {
                e.with_stack(4, |e, slot| {
                    e.set_global(slot.addr(), item);
                    e.call(ACTIVE_INSERT_FIRST, &args![this, slot]);
                });
            } else {
                let added = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
                let before = Ptr::<ActivePass>::new(previous);
                if added != 0 {
                    let added = Ptr::<ActivePass>::new(added);
                    e.set(added, ActivePass::usItem, low16(value));
                    e.set(added, ActivePass::pNext, 0);
                    e.set(before, ActivePass::pNext, added.addr());
                    e.set(added, ActivePass::pNext, node);
                } else {
                    // The exe links through the null block: `[0 + 4]` takes the node.
                    e.set(before, ActivePass::pNext, 0);
                    e.set_global(4u32, node);
                }
            }
            break;
        }
        previous = node;
        node = e.get(entry, ActivePass::pNext);
    }
}

/// A new active entry (eight bytes from the memory manager) with `value`, or 0.
#[inline(always)]
fn new_entry(e: &mut Engine, value: u32) -> u32 {
    let added = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    if added != 0 {
        let entry = Ptr::<ActivePass>::new(added);
        e.set(entry, ActivePass::usItem, low16(value));
        e.set(entry, ActivePass::pNext, 0);
    }
    added
}

#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
fn low16(word: u32) -> u16 {
    word as u16
}

// Typed from 00b99a90
/// `thiscall(group, list, pass)`: prepends the pass to the group's list; both counts go up.
pub fn group_prepend(e: &mut Engine, this: Ptr<PassGroup>, list: u32, pass: Ptr<RenderPass>) {
    let list = Ptr::<PassList>::new(this.addr().wrapping_add(list.wrapping_mul(0x14)));
    let node = take_node(e, list);
    e.set(node, PassNode::pItem, pass.addr());
    let head = e.get(list, PassList::pHead);
    e.set(node, PassNode::pNext, head);
    e.set(node, PassNode::pPrev, 0);
    let head = e.get(list, PassList::pHead);
    if head != 0 {
        e.set(Ptr::<PassNode>::new(head), PassNode::pPrev, node.addr());
    } else {
        e.set(list, PassList::pTail, node.addr());
    }
    e.set(list, PassList::pHead, node.addr());
    count_up(e, this, list);
}

// Typed from 00b99b00
/// `thiscall(group, list, pass)`: appends the pass to the group's list; both counts go up.
pub fn group_append(e: &mut Engine, this: Ptr<PassGroup>, list: u32, pass: Ptr<RenderPass>) {
    let list = Ptr::<PassList>::new(this.addr().wrapping_add(list.wrapping_mul(0x14)));
    let node = take_node(e, list);
    e.set(node, PassNode::pItem, pass.addr());
    e.set(node, PassNode::pNext, 0);
    let tail = e.get(list, PassList::pTail);
    e.set(node, PassNode::pPrev, tail);
    let tail = e.get(list, PassList::pTail);
    if tail != 0 {
        e.set(Ptr::<PassNode>::new(tail), PassNode::pNext, node.addr());
    } else {
        e.set(list, PassList::pHead, node.addr());
    }
    e.set(list, PassList::pTail, node.addr());
    count_up(e, this, list);
}

#[inline(always)]
fn count_up(e: &mut Engine, group: Ptr<PassGroup>, list: Ptr<PassList>) {
    let count = e.get(list, PassList::uiCount).wrapping_add(1);
    e.set(list, PassList::uiCount, count);
    let total = e.get(group, PassGroup::uiCount).wrapping_add(1);
    e.set(group, PassGroup::uiCount, total);
}

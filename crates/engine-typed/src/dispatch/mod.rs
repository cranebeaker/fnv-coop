//! The dispatch family (speed batch D1 of the migration, stage 1): the functions the town calls by
//! address most often, picked by calls by address a frame in a profiled Goodsprings run, each with
//! the callers whose call sites make those calls, so that the calls become Rust calls. A typed
//! callee reached by address from a translation still pays the call by address; a typed caller
//! calling it with `typed_call!` doesn't. The list with each function's calls a frame is
//! `measure/typed/hand/batchD1.tsv` (private).
//!
//! - [`math`]: the transforms' and matrices' arithmetic (compose, invert, transpose, matrix times
//!   matrix and vector), points scaled and normalised, the multi-bound box's corners, the 4x4
//!   inverse for the shaders, the Havok vector scale, float closeness;
//! - [`scene`]: the scene graph's walks and getters: an object's controllers, children and their
//!   updates, the collision object and its RTTI cast, a reference's 3D, the frustum test of a bound,
//!   the geometry's immediate render;
//! - [`refs`]: the counted references, the recursive spin lock, lists, arrays and the small getters
//!   the AI, the inventory, the HUD's compass and the process levels call on every actor.
//!
//! Stage 1: the objects stay in guest memory where the game made them. Each function stays
//! registered at its exe address; a call inside the family, or into another family, is a
//! `typed_call!`.
//!
//! **Floats.** These run on the main thread and on others ([`Threads::Both`]), so the x87 code keeps
//! the exact helpers ([`engine_core::x87`]): every load, operation and store is the exe's, in its
//! order, with a memory operand kept a memory operand.

use engine_core::ptr::Ptr;
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn};

use crate::culling::layout::{NiAVObject, NiMatrix3, NiNode, NiPoint3, NiTransform};
pub(crate) use crate::culling::{fld, fld_local, fstp, m32_local};

pub mod layout;
pub mod math;
pub mod refs;
pub mod scene;

use layout::{LightLock, RefObject};

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(9);

/// `NV_TYPED=dispatch`. Past its speed gate (2026-11-08, together with the other five families of this crate: real
/// threads, the fixes preset, the same exe off and on, BAAB): Goodsprings 55.0 to 60.1 fps (+9.3%), the crowded
/// street at 2560x1440 8x AA 52.3 to 57.7 fps (+10.3%), the sky above it 117 to 134 fps (+14%). On unless
/// `NV_TYPED` says otherwise (`NV_TYPED=off` or `dispatch:off`).
pub static DISPATCH: Family = Family {
    name: "dispatch",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// Copies `count` words from `from` to `to` (`MOVSD`/`REP MOVSD`, never through the x87 stack).
#[inline(always)]
pub(crate) fn copy_words(e: &mut engine_core::Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The family's functions, by exe address.
#[rustfmt::skip]
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        // The arithmetic.
        entry!(0x0062_c250, math::compose(Ptr<NiTransform>, Ptr<NiTransform>, Ptr<NiTransform>) -> Ptr<NiTransform>),
        entry!(0x004b_4880, math::invert_non_uniform(Ptr<NiTransform>, Ptr<NiTransform>)),
        entry!(0x004b_4500, math::rotate(Ptr<NiMatrix3>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>),
        entry!(0x0043_f8d0, math::multiply(Ptr<NiMatrix3>, Ptr<NiMatrix3>, Ptr<NiMatrix3>) -> Ptr<NiMatrix3>),
        entry!(0x0047_68c0, math::transpose(Ptr<NiMatrix3>, Ptr<NiMatrix3>) -> Ptr<NiMatrix3>),
        entry!(0x0047_6930, math::row(Ptr<NiMatrix3>, u32, Ptr<NiPoint3>)),
        entry!(0x0047_6980, math::set_columns(Ptr<NiMatrix3>, Ptr<NiPoint3>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiMatrix3>),
        entry!(0x0047_69c0, math::set_column(Ptr<NiMatrix3>, u32, Ptr<NiPoint3>)),
        entry!(0x0047_6a80, math::transform_new(Ptr<NiTransform>) -> Ptr<NiTransform>),
        entry!(0x0045_bb20, math::times(Ptr<NiPoint3>, Ptr<NiPoint3>, f32) -> Ptr<NiPoint3>),
        entry!(0x004a_0c10, math::normalize(Ptr<NiPoint3>)),
        entry!(0x00c3_9270, math::box_corners(Ptr, Ptr)),
        entry!(0x00c4_c2d0, math::inverse_4x4(Ptr<NiTransform>, u32, Ptr)),
        entry!(0x004a_3e00, math::havok_vector(Ptr, Ptr<NiPoint3>) -> Ptr),
        entry!(0x004a_3e90, math::havok_scale(f32) -> f32),
        entry!(0x0056_0d30, math::lane(Ptr, u32) -> Ptr),
        entry!(0x004a_3c90, math::copy_16(Ptr, Ptr) -> Ptr),
        entry!(0x0045_2dc0, math::point2_new(Ptr, f32, f32) -> Ptr),
        entry!(0x0049_e2f0, math::points_close(Ptr<NiPoint3>, Ptr<NiPoint3>, f32) -> bool),
        entry!(0x0049_e390, math::floats_close(f32, f32, f32) -> bool),
        // The scene graph.
        entry!(0x0043_b230, scene::first_controller(Ptr<NiAVObject>) -> Ptr),
        entry!(0x004a_8a90, scene::next_controller(Ptr) -> u32),
        entry!(0x004e_ff50, scene::update_object_controllers(Ptr<NiAVObject>, Ptr, u8)),
        entry!(0x004f_0000, scene::has_property_controllers(Ptr<NiAVObject>) -> bool),
        entry!(0x0043_b480, scene::child_count(Ptr<NiNode>) -> u16),
        entry!(0x0043_b4a0, scene::child(Ptr<NiNode>, u32) -> Ptr),
        entry!(0x0065_8930, scene::array_size(Ptr) -> u16),
        entry!(0x00a5_dd70, scene::update_downward(Ptr<NiNode>, u32, u32)),
        entry!(0x00a5_ded0, scene::update_selected_downward(Ptr<NiNode>, u32, u32)),
        entry!(0x00a5_e0b0, scene::update_transform_and_bounds(Ptr<NiNode>, u32)),
        entry!(0x00a8_0610, scene::geometry_render_immediate(Ptr, Ptr)),
        entry!(0x00a7_1d70, scene::tri_strips_render_immediate(Ptr, Ptr)),
        entry!(0x0043_fcd0, scene::reference_3d(Ptr) -> Ptr),
        entry!(0x0043_b4f0, scene::loaded_3d(Ptr, Ptr) -> Ptr),
        entry!(0x0043_b540, scene::handle_3d(Ptr) -> u32),
        entry!(0x0043_b560, scene::loaded_handle(Ptr) -> u32),
        entry!(0x0043_b5b0, scene::lock_loaded(Ptr)),
        entry!(0x0043_b5d0, scene::loaded_lock(Ptr) -> Ptr),
        entry!(0x0043_b5f0, scene::unlock_loaded(Ptr)),
        entry!(0x0043_b610, scene::collision_object(Ptr<NiAVObject>) -> Ptr),
        entry!(0x0068_38b0, scene::collision_word(Ptr) -> u32),
        entry!(0x0065_3270, scene::cast_if(u32, u32) -> u32),
        entry!(0x0065_3290, scene::cast(Ptr, u32) -> Ptr),
        entry!(0x0065_32c0, scene::is_kind_of(Ptr, u32) -> bool),
        entry!(0x0045_3470, scene::esize_of_children(Ptr) -> u32),
        entry!(0x0099_38b0, scene::esize(Ptr) -> u16),
        entry!(0x004b_5fc0, scene::bound_in_frustum(Ptr<NiAVObject>, Ptr) -> bool),
        entry!(0x004b_5ff0, scene::in_frustum(Ptr, Ptr) -> bool),
        entry!(0x004b_4cf0, scene::plane(Ptr, u32) -> Ptr),
        entry!(0x0062_0b80, scene::word_8(u32) -> u32),
        entry!(0x004a_e750, scene::character_proxy(Ptr) -> u32),
        entry!(0x0056_0e10, scene::body_vector(Ptr) -> Ptr),
        entry!(0x0056_0e50, scene::body_vector_of(Ptr) -> Ptr),
        entry!(0x009d_9f40, scene::part_e0(Ptr) -> Ptr),
        // References, locks, lists and getters.
        entry!(0x0040_b460, refs::increment(Ptr) -> i32),
        entry!(0x0040_19a0, refs::decrement(Ptr) -> i32),
        entry!(0x0040_f6e0, refs::add_reference(Ptr<RefObject>)),
        entry!(0x0040_1970, refs::release(Ptr<RefObject>)),
        entry!(0x0045_cec0, refs::release_held(Ptr)),
        entry!(0x0049_68b0, refs::ref_object_new(Ptr<RefObject>) -> Ptr<RefObject>),
        entry!(0x0049_6910, refs::ref_object_drop(Ptr<RefObject>)),
        entry!(0x0040_fc90, refs::current_thread() -> u32),
        entry!(0x0043_b460, refs::compare_exchange(Ptr, i32, i32) -> i32),
        entry!(0x0040_fbf0, refs::lock(Ptr<LightLock>, u32)),
        entry!(0x0040_fba0, refs::unlock(Ptr<LightLock>)),
        entry!(0x0045_34f0, refs::nothing_2(u32, u32)),
        entry!(0x0048_3710, refs::nothing_1(u32)),
        entry!(0x0046_0140, refs::at_10(Ptr) -> Ptr),
        entry!(0x008c_71b0, refs::store_word(Ptr, u32) -> Ptr),
        entry!(0x0048_c1b0, refs::reaction(Ptr, u32) -> u32),
        entry!(0x008f_6650, refs::detection_state(Ptr, u32, i32) -> Ptr),
        entry!(0x0096_8670, refs::array_item(Ptr, u32) -> u32),
        entry!(0x004c_8f30, refs::item_count(Ptr, u32) -> i32),
        entry!(0x004b_ffb0, refs::container_of(Ptr) -> u32),
        entry!(0x004b_fba0, refs::entry_for(Ptr, u32, u32, u32) -> u32),
        entry!(0x008a_e4c0, refs::location_on_reference(Ptr) -> u32),
        entry!(0x0043_6aa0, refs::position(Ptr) -> Ptr),
        entry!(0x005a_2030, refs::byte_14d(Ptr) -> u8),
        entry!(0x0088_44f0, refs::in_state(Ptr, bool) -> bool),
        entry!(0x004f_8960, refs::word_108(Ptr) -> u32),
        entry!(0x0090_7650, refs::compute_last_time_processed(Ptr)),
        entry!(0x0069_3d50, refs::set_hour(Ptr, f32)),
        entry!(0x0070_37c0, refs::set_date(Ptr, u32)),
        entry!(0x0077_b300, refs::compass_order(Ptr<u32>, Ptr<u32>) -> i32),
        entry!(0x0086_a050, refs::object_factor(u32) -> f64),
        entry!(0x009a_d610, refs::word_cc(Ptr) -> Ptr),
        entry!(0x0084_54f0, refs::array_clear(Ptr, u8)),
        entry!(0x0069_aee0, refs::order_array_new(Ptr) -> Ptr),
        entry!(0x0069_af10, refs::info_array_drop(Ptr) -> Ptr),
        entry!(0x004f_1540, refs::extra_kind(Ptr) -> u8),
    ]
}

//! The shader family (F7 of the migration plan, stage 1): the shader set-up and batch renderer layer
//! the main thread spends the most time in once threads run. A visible geometry is registered with
//! the accumulator (`RegisterObject`, the render mode's register function), which files each of its
//! render passes with the thread's batch renderer by pass type ([`batch`]); the batch renderer then
//! draws the groups type by type, beginning each shader pass and drawing each geometry, while the
//! render state setters keep the device's states behind lock counters and a cache ([`state`]); the
//! shaders' per-geometry set-ups (world matrix, light colour, blend states) are in [`accumulator`].
//!
//! Stage 1: the objects stay in guest memory where the game made them and are read through their
//! [`layout`]. Each function stays registered at its exe address; a call inside the family is a
//! `typed_call!`, and a function pointer that names one of them (the register function, the alpha
//! sort's comparison) is called directly when it does.
//!
//! **Floats.** The per-thread census (`t3/threads.tsv`) has this layer on the main thread, a few of
//! its functions also on another game thread (about 1 call in 2,000 for the stage caches and the
//! transform). So the family is [`Threads::Both`] and keeps the culling family's exact x87 helpers:
//! every load, operation and store is the exe's, in its order, under either control word.

use engine_core::ptr::{Field, Ptr};
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn};

pub mod accumulator;
pub mod batch;
pub mod layout;
pub mod state;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(17);

/// `NV_TYPED=shader`. Past its speed gate (2026-11-08, together with the other five families of this crate: real
/// threads, the fixes preset, the same exe off and on, BAAB): Goodsprings 55.0 to 60.1 fps (+9.3%), the crowded
/// street at 2560x1440 8x AA 52.3 to 57.7 fps (+10.3%), the sky above it 117 to 134 fps (+14%). On unless
/// `NV_TYPED` says otherwise (`NV_TYPED=off` or `shader:off`).
pub static SHADER: Family = Family {
    name: "shader",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// The guest address of element `i` of an array field, `stride` bytes apart.
#[inline(always)]
pub(crate) fn elem_addr<S, A>(p: Ptr<S>, f: Field<S, A>, i: u32, stride: u32) -> u32 {
    p.addr()
        .wrapping_add(f.off)
        .wrapping_add(i.wrapping_mul(stride))
}

/// The low byte of a word (a `bool` or `char` argument the exe pushes as a word).
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn low8(word: u32) -> u8 {
    word as u8
}

/// The family's functions, by exe address.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        // The render state setters.
        entry!(0x00b9_7de0, state::set_z_enable(u8, u32)),
        entry!(0x00b9_7e30, state::set_z_write_enable(u8, u32)),
        entry!(0x00b9_7ed0, state::set_alpha_test_enable(u8, u32)),
        entry!(0x00b9_7fa0, state::set_alpha_blend_enable(u8, u32)),
        entry!(0x00b9_8380, state::set_color_write(u32, u32)),
        entry!(0x00b9_7e80, state::set_z_function(u32, u32)),
        entry!(0x00b9_7f20, state::set_alpha_function(u32, u32, u32)),
        entry!(0x00b9_7ff0, state::set_blend(u32, u32, u32)),
        entry!(0x00b9_7d90, state::set_state(u32, u32, u32)),
        entry!(0x00b9_8540, state::set_alpha_to_coverage(u8, u32)),
        entry!(0x00b9_84d0, state::set_cull_mode_of_renderer(u32, u32)),
        // The renderer's state cache.
        entry!(0x00e8_79b0, state::set_cull_mode(Ptr<_>, u32, u32)),
        entry!(
            0x00e8_8780,
            state::set_render_state(Ptr<_>, u32, u32, u32, u8)
        ),
        entry!(
            0x00e8_8930,
            state::set_stage_state(Ptr<_>, u32, u32, u32, u32)
        ),
        entry!(
            0x00e9_10a0,
            state::set_sampler_state(Ptr<_>, u32, u32, u32, u32)
        ),
        entry!(0x00e8_8a20, state::set_texture(Ptr<_>, u32, u32)),
        entry!(0x00e9_0e00, state::set_vertex_shader(Ptr<_>, u32, u8)),
        entry!(0x00e8_8870, state::set_pixel_shader(Ptr<_>, u32, u8)),
        entry!(0x00e9_0fc0, state::set_declaration(Ptr<_>, u32, u8)),
        entry!(0x00e8_79e0, state::set_alpha_test_of(Ptr<_>, u32)),
        entry!(0x00e8_8730, state::set_range(Ptr<_>, f32, f32)),
        entry!(0x00e8_8750, state::cull_side_set(Ptr<_>) -> bool),
        // The texture stage caches.
        entry!(
            0x00e8_9060,
            state::cached_stage_state(Ptr<_>, u32, u32, u32) -> bool
        ),
        entry!(0x00e8_9250, state::apply_stage_states(Ptr<_>, u32) -> u32),
        entry!(0x00e8_92d0, state::apply_sampler_states(Ptr<_>, u32) -> u32),
        entry!(0x00e8_9410, state::apply_states(Ptr<_>, u32) -> u32),
        entry!(0x00e7_de90, state::set_address_modes(Ptr<_>, u32)),
        entry!(0x00e7_def0, state::set_filter_mode(Ptr<_>, u32)),
        // Geometry buffers and groups, constant maps.
        entry!(0x00e8_eeb0, state::has_streams(Ptr<_>) -> bool),
        entry!(
            0x00e8_8dc0,
            state::add_object_to_group(u32, u32, Ptr<_>, Ptr<_>, u8, u32, u32) -> bool
        ),
        entry!(0x00e8_6f40, state::array_set_at(Ptr<_>, u32, u32)),
        entry!(0x00e8_70c0, state::sort_entries_by_enabled(u32, Ptr<_>)),
        entry!(0x00e8_7200, state::sort_by_enabled(Ptr<_>)),
        // The batch renderer.
        entry!(0x00b9_8ae0, batch::first_list(Ptr<_>, u32, u32) -> bool),
        entry!(0x00b9_9730, batch::pop_active(Ptr<_>)),
        entry!(0x00b9_9900, batch::next_active(Ptr<_>, u32, u32) -> bool),
        entry!(
            0x00b9_a090,
            batch::next_group(Ptr<_>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00b9_a120,
            batch::render_group(Ptr<_>, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00b9_94f0, batch::render_pass(Ptr<_>, u32, u32, u32, u32)),
        entry!(0x00b9_8e80, batch::render_standard(Ptr<_>, u32, u32, u32)),
        entry!(0x00b9_9110, batch::render_skinned(Ptr<_>, u32, u32, u32)),
        entry!(0x00b9_9390, batch::begin_pass(u32, u32)),
        entry!(0x00b9_88e0, batch::end_pass()),
        entry!(0x00b9_8b20, batch::compare_by_depth(u32, u32) -> i32),
        entry!(0x00b9_9a10, batch::sort_alphas(Ptr<_>, u32, u8)),
        entry!(0x00b9_9630, batch::sort_list(Ptr<_>, u32)),
        entry!(0x00b9_9840, batch::list_append(Ptr<_>, u32)),
        entry!(0x00b9_9890, batch::list_prepend(Ptr<_>, u32)),
        entry!(0x00b9_abc0, batch::add_pass(Ptr<_>, Ptr<_>, u32)),
        entry!(
            0x00b9_b3b0,
            batch::register_pass(Ptr<_>, Ptr<_>, u32, u32) -> u32
        ),
        entry!(
            0x00b9_b3d0,
            batch::register_alpha(Ptr<_>, Ptr<_>, u32) -> u32
        ),
        entry!(0x00b9_a490, batch::clear_holder(Ptr<_>)),
        entry!(0x00b9_9a50, batch::clear_group(Ptr<_>, u32)),
        entry!(0x00b9_9bc0, batch::clear_renderer(Ptr<_>, u32)),
        entry!(0x00b6_3e90, batch::release_spare(Ptr<_>) -> u32),
        entry!(0x00b9_9c90, batch::add_to_group(Ptr<_>, Ptr<_>, u32)),
        entry!(0x00b9_9770, batch::insert_active(Ptr<_>, u32, u32)),
        entry!(0x00b9_9a90, batch::group_prepend(Ptr<_>, u32, Ptr<_>)),
        entry!(0x00b9_9b00, batch::group_append(Ptr<_>, u32, Ptr<_>)),
        // The accumulator and the shaders' per-geometry set-ups.
        entry!(
            0x00b6_4020,
            accumulator::process_commands(Ptr<_>, Ptr<_>, u32, Ptr<_>, u32) -> bool
        ),
        entry!(
            0x00b6_3f10,
            accumulator::register_object(Ptr<_>, Ptr<_>) -> bool
        ),
        entry!(0x00b6_5e80, accumulator::finish(Ptr<_>)),
        entry!(0x00b6_4f90, accumulator::draw_sorted(Ptr<_>)),
        entry!(0x00be_1ff0, accumulator::set_blend_of(u32, u32)),
        entry!(
            0x00b7_1a40,
            accumulator::make_matrix(u32, Ptr<_>, Ptr<_>, f32)
        ),
        entry!(
            0x00e6_fbb0,
            accumulator::set_world_transform(Ptr<_>, Ptr<_>, u8)
        ),
        entry!(
            0x00b7_c750,
            accumulator::set_world_view_constants(u32, u32, u32)
        ),
        entry!(0x00b7_c7b0, accumulator::set_light_direction(u32)),
        entry!(
            0x00c0_3230,
            accumulator::texture_by_index(Ptr<_>, u32) -> u32
        ),
        entry!(0x00c0_3010, accumulator::accumulate_light_color(Ptr<_>)),
        entry!(0x00c0_2d90, accumulator::set_tint(u32, Ptr<_>, u32, u32)),
        entry!(
            0x00ba_95e0,
            accumulator::check_create_pass_array(Ptr<_>, i32)
        ),
        entry!(0x00ba_8ab0, accumulator::set_alpha(Ptr<_>, f32)),
        entry!(0x00b5_ac20, accumulator::light_at(Ptr<_>, u32) -> u32),
        entry!(0x00b7_cfb0, accumulator::set_pair_textures(u32, u32)),
        entry!(0x00b7_c320, accumulator::restore_states(u32, u32)),
    ]
}

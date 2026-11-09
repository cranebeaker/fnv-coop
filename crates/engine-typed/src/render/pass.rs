//! The pass's side of the stage binding: the current pass binds each of its enabled stages
//! ([`bind_pass_stages`]), and the shadow light shader's plain geometry pass sets the two texture
//! entries of the pass, binds them and sets its world-view constants and light direction
//! ([`geometry_pass`]).

use super::stage;
use super::FAMILY;
use crate::shader::{accumulator, state};
use engine_core::ptr::Ptr;
use engine_core::{typed_call, Engine};

/// The current shader pass (`0126f74c`): its stage count (+0x18) and stages (+0x24).
const CURRENT_PASS: u32 = 0x0126_f74c;
/// The holder of the object whose matrix (+0x68) the geometry pass inverts (`011f91e0`).
const VIEW_HOLDER: u32 = 0x011f_91e0;
/// The shader property's texture clamp mode (slot 0xe0, [`clamp_mode`] in the lighting property).
const SLOT_CLAMP_MODE: u32 = 0xe0;
const CLAMP_MODE: u32 = 0x00b6_71c0;

// Typed from 00be2170
/// `cdecl`, no arguments: each stage of the current pass, up to the first null one, whose byte +0x64
/// and texture are set is bound ([`stage::bind`], not forced). The count is read once.
pub fn bind_pass_stages(e: &mut Engine) {
    let pass = e.global::<u32>(CURRENT_PASS);
    let count = e.mem.u32(pass.wrapping_add(0x18));
    let mut index = 0u32;
    while index < count {
        let stages = e.mem.u32(pass.wrapping_add(0x24));
        let entry = e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)));
        if entry == 0 {
            return;
        }
        if e.mem.u8(entry.wrapping_add(0x64)) != 0 && e.mem.u32(entry.wrapping_add(8)) != 0 {
            let _: bool = typed_call!(FAMILY, 0x00e7_eb00, stage::bind(e, Ptr::new(entry), 0u8));
        }
        index += 1;
    }
}

/// The current pass's stage `index` (read afresh each time, as the exe does).
#[inline(always)]
fn pass_stage(e: &Engine, index: u32) -> u32 {
    let pass = e.global::<u32>(CURRENT_PASS);
    let stages = e.mem.u32(pass.wrapping_add(0x24));
    e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)))
}

/// The property's clamp mode (slot 0xe0): [`clamp_mode`] directly when the slot holds it.
#[inline(always)]
fn property_clamp_mode(e: &mut Engine, property: u32) -> u32 {
    if crate::slot_target(e, property, SLOT_CLAMP_MODE) == CLAMP_MODE {
        typed_call!(FAMILY, 0x00b6_71c0, clamp_mode(e, property))
    } else {
        e.vcall(property, SLOT_CLAMP_MODE, &[]).u32()
    }
}

// Typed from 00b7dde0
/// `thiscall(shader, pass)`, the property `[pass + 0xc]`: stage 0 of the current pass gets the word at
/// the property's `[+0xac]` as its texture, stage 1 the property's texture 0 (`00c03230`), each then
/// the address modes of the property's clamp mode (`00e7de90`); both stages are bound (not forced);
/// the view object's matrix (`[[011f91e0]] + 0x68`) is inverted into a 64-byte local (`00c4c2d0`), the
/// shader's world-view constants are set from it (`00b7c750`) and the light direction from the local
/// (`00b7c7b0`).
pub fn geometry_pass(e: &mut Engine, this: Ptr, pass: Ptr) {
    let this = this.addr();
    let property = e.mem.u32(pass.addr().wrapping_add(0xc));
    // The two texture entries.
    let source = e.mem.u32(property.wrapping_add(0xac));
    let stage0 = pass_stage(e, 0);
    let texture = e.mem.u32(source);
    e.mem.set_u32(stage0.wrapping_add(8), texture);
    let mode = property_clamp_mode(e, property);
    typed_call!(
        crate::shader::FAMILY,
        0x00e7_de90,
        state::set_address_modes(e, Ptr::new(stage0), mode)
    );
    let stage1 = pass_stage(e, 1);
    let texture: u32 = typed_call!(
        crate::shader::FAMILY,
        0x00c0_3230,
        accumulator::texture_by_index(e, Ptr::new(property), 0u32)
    );
    e.mem.set_u32(stage1.wrapping_add(8), texture);
    let mode = property_clamp_mode(e, property);
    typed_call!(
        crate::shader::FAMILY,
        0x00e7_de90,
        state::set_address_modes(e, Ptr::new(stage1), mode)
    );
    // Both stages bound.
    for index in [0u32, 1] {
        let entry = pass_stage(e, index);
        let _: bool = typed_call!(FAMILY, 0x00e7_eb00, stage::bind(e, Ptr::new(entry), 0u8));
    }
    e.with_stack(0x40, |e, frame| {
        let local = frame.addr();
        let holder = e.global::<u32>(VIEW_HOLDER);
        let object = e.mem.u32(holder);
        let matrix = object.wrapping_add(0x68);
        typed_call!(
            crate::dispatch::FAMILY,
            0x00c4_c2d0,
            crate::dispatch::math::inverse_4x4(e, Ptr::new(matrix), 0u32, Ptr::new(local))
        );
        typed_call!(
            crate::shader::FAMILY,
            0x00b7_c750,
            accumulator::set_world_view_constants(e, this, object, matrix)
        );
        typed_call!(
            crate::shader::FAMILY,
            0x00b7_c7b0,
            accumulator::set_light_direction(e, local)
        );
    });
}

// Typed from 00b671c0
/// The word +0xd8 of a lighting shader property (its slot 0xe0): the texture clamp mode.
pub fn clamp_mode(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this.wrapping_add(0xd8))
}

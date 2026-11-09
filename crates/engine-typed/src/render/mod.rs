//! The render family (speed batch, family 10, stage 1): the render path's functions the crowded street
//! calls by address most often that the shader family left out, picked by calls through dispatch a
//! frame in a profiled street run (the frame-breakdown agent's crowded view, every family on), each
//! with the callers and callees whose calls then become Rust calls:
//!
//! - [`stage`]: binding a texture stage (`NiD3DTextureStage`'s apply, its texture, transform and
//!   sampler set-up through the render state cache) and the texture manager's query of a texture
//!   with the texture data getters it reaches through vtables;
//! - [`pass`]: the current pass's stage binding and the shadow light shader's plain geometry pass.
//!
//! A call the exe makes through a vtable or a function pointer stays one, except where the typed
//! caller finds the slot holding a typed function ([`crate::slot_target`]): then it is a
//! `typed_call!`, a Rust call while that family is on, the same call by address otherwise.
//!
//! Stage 1: the objects stay in guest memory where the game made them and are read through their
//! [`layout`]. Each function stays registered at its exe address.
//!
//! **Threads.** The census (`t3/threads.tsv`) has these on the main thread and, about one call in
//! 4,000, the loading thread: [`Threads::Both`]. No float is computed here.

use engine_core::ptr::Ptr;
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn};

pub mod layout;
pub mod pass;
pub mod stage;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(10);

/// `NV_TYPED=render`. Not past its gate.
pub static RENDER: Family = Family {
    name: "render",
    id: FAMILY,
    past_gate: false,
    threads: Threads::Both,
    funcs,
};

/// The family's functions, by exe address.
#[rustfmt::skip]
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        // The texture stage.
        entry!(0x00e7_eb00, stage::bind(Ptr<_>, u8) -> bool),
        entry!(0x00e7_ea00, stage::apply(Ptr<_>)),
        entry!(0x00e7_e940, stage::send_transform(Ptr<_>)),
        entry!(0x00e7_dc90, stage::send_sampler_commands(Ptr, u32)),
        // The texture manager's query and the texture data getters.
        entry!(0x00e9_0b10, stage::texture_data(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e6_8800, stage::d3d_texture(u32) -> u32),
        entry!(0x00e6_8810, stage::this_word(u32) -> u32),
        entry!(0x00e6_87f0, stage::levels(u32) -> u32),
        // The pass.
        entry!(0x00be_2170, pass::bind_pass_stages()),
        entry!(0x00b7_dde0, pass::geometry_pass(Ptr, Ptr)),
        entry!(0x00b6_71c0, pass::clamp_mode(u32) -> u32),
    ]
}

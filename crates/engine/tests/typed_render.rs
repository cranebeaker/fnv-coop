//! The differential test of every typed function of the render family (`engine_typed::render`) against
//! its translation, and of the vtable call sites the shader family makes direct through the slot's
//! target (`engine_typed::slot_target`): each with the slot holding the typed function and holding a
//! stand-in, so both branches compare.
//!
//! Two worlds. In the first the shader family is on and real: the render state cache's vtable holds
//! the shader family's setters, so the stage's states go through them, directly on the direct run. In
//! the second the shader family is off and its functions the render family calls are stand-ins
//! answering from a table, so the tests can choose what they answer; their `typed_call!`s then go by
//! address on every run, as they would in play with that family off.
//!
//! What stands in: every virtual function the code calls on the device, the renderer, the texture
//! manager, the texture data, the property and the shader (test functions past `.text` answering a
//! word per slot from a table), the critical section imports, the texture's source and the renderer
//! data maker. The texture matrix builder (`00e7df90`) is the real translation on both sides.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Diff, Input};
use engine_core::typed::leaf;
use engine_core::{entry, Engine};
use engine_typed::render::{self, pass, passes, stage, stream};
use engine_typed::{crt, culling, dispatch, pathing, shader};

/// The stand-in virtual functions, one per slot, and the table of what each answers.
const VT_FN: u32 = 0x00ff_2000;
const RETS: u32 = 0x0300_0000;
const MAX_CALLS: usize = 5_000;

/// What the stand-in of the renderer data maker (`00e68ef0`) answers, and the stand-ins of the shader
/// family's functions.
const MADE_ANSWER: u32 = RETS + 0x404;
const APPLY_STATES_ANSWER: u32 = RETS + 0x408;
const CACHED_ANSWER: u32 = RETS + 0x40c;
const TEXTURE_ANSWER: u32 = RETS + 0x410;

/// The shader family's functions the render family calls (stand-ins in the second world).
const SHADER_CALLEES: &[u32] = &[
    0x00e8_9410,
    0x00e8_9060,
    0x00e7_de90,
    0x00c0_3230,
    0x00b7_c750,
    0x00b7_c7b0,
    0x00e8_8780,
    0x00e8_8930,
    0x00e9_10a0,
    0x00e8_8a20,
];

/// The globals.
const RENDERER: u32 = 0x0126_f6c4;
const DEVICE: u32 = 0x0126_f6c0;
const RENDER_STATE: u32 = 0x0126_f6c8;
const SHADOWED: u32 = 0x0126_f680;
const STAGE_COUNT: u32 = 0x011b_f558;
const CURRENT_PASS: u32 = 0x0126_f74c;
const VIEW_HOLDER: u32 = 0x011f_91e0;
const STATE_TARGET: u32 = 0x0126_f99c;
const TEXTURE_RENDERER: u32 = 0x011f_4748;

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

fn slot_answer<const S: u32>(e: &mut Engine, _: &[u32]) -> Ret {
    ret(e.mem.u32(RETS + S))
}

macro_rules! slots {
    ($e:ident; $($s:literal),* $(,)?) => { $( $e.register(VT_FN + $s, slot_answer::<$s>); )* };
}

fn set(e: &mut Engine, at: u32, value: u32) {
    e.mem.set_u32(at, value);
}

/// A vtable of 0x200 bytes of stand-ins, with `real` slots pointing at real functions.
fn vtable(e: &mut Engine, real: &[(u32, u32)]) -> u32 {
    let vt = e.mem.alloc(0x200);
    for slot in (0..0x200).step_by(4) {
        let target = real
            .iter()
            .find(|(s, _)| *s == slot)
            .map_or(VT_FN + slot, |(_, a)| *a);
        set(e, vt + slot, target);
    }
    vt
}

/// An object of `size` bytes with vtable `vt`.
fn obj(e: &mut Engine, size: u32, vt: u32) -> u32 {
    let o = e.mem.alloc(size);
    set(e, o, vt);
    o
}

/// The world's objects.
#[derive(Clone, Copy)]
struct W {
    renderer: u32,
    manager: u32,
    stand_in_manager: u32,
    cache: u32,
    stage_cache: u32,
    stage: u32,
    texture: u32,
    data: u32,
    rendered: u32,
    stand_in_data: u32,
    property: u32,
    stand_in_property: u32,
    pass_object: u32,
    stages: [u32; 3],
}

/// The callees outside the families, and the shader family's functions when `real_shader` is false.
fn stand_ins(e: &mut Engine, real_shader: bool) {
    e.map(RETS, 0x1000);
    slots!(e; 0x04, 0x08, 0x0c, 0x30, 0x68, 0x98, 0x9c, 0xa0, 0xa4, 0xa8, 0xac, 0xb0, 0xb4, 0xc0,
        0xcc, 0xdc, 0xe0, 0xe4, 0x104, 0x10c, 0x114, 0x15c, 0x170, 0x1ac, 0x1c, 0x8c, 0x90, 0x118, 0x190, 0x1a0);
    e.register(0x00fd_f05c, |_, _| ret(0));
    e.register(0x00fd_f100, |_, _| ret(0));
    e.register(0x00e6_8ef0, |e, _| ret(e.mem.u32(MADE_ANSWER)));
    if !real_shader {
        // The stage cache's apply answers from the table; its cached value writes the out words.
        e.register(0x00e8_9410, |e, _| ret(e.mem.u32(APPLY_STATES_ANSWER)));
        e.register(0x00e8_9060, |e, a| {
            let answer = e.mem.u32(CACHED_ANSWER);
            e.mem.set_u32(a[2], 0x55);
            e.mem.set_u8(a[3], 1);
            ret(answer)
        });
        e.register(0x00c0_3230, |e, a| ret(e.mem.u32(TEXTURE_ANSWER) ^ a[1]));
        for addr in [
            0x00e7_de90u32,
            0x00b7_c750,
            0x00b7_c7b0,
            0x00e8_8780,
            0x00e8_8930,
            0x00e9_10a0,
            0x00e8_8a20,
        ] {
            e.register(addr, |e, a| {
                // A mark of the call in memory, so a call moved shows in what it left.
                let n = e.mem.u32(RETS + 0x500);
                e.mem.set_u32(
                    RETS + 0x500,
                    n.wrapping_mul(31).wrapping_add(a.len() as u32),
                );
                ret(0)
            });
        }
    }
    for (addr, len) in [
        (0x0101_0000, 0x10000),
        (0x011a_9000, 0x1000),
        (0x011b_f000, 0x1000),
        (0x011f_4000, 0xc000),
        (0x0126_f000, 0x1000),
    ] {
        e.map(addr, len);
    }
}

/// The world: with `real_shader` the render state cache's vtable holds the shader family's setters.
fn world(e: &mut Engine, real_shader: bool) -> W {
    stand_ins(e, real_shader);
    let vt = vtable(e, &[]);
    let device = obj(e, 0x10, vt);
    set(e, DEVICE, device);
    let cache_vt = if real_shader {
        vtable(
            e,
            &[
                (0x68, 0x00e8_8780),
                (0xc0, 0x00e8_8930),
                (0xcc, 0x00e9_10a0),
                (0xdc, 0x00e8_8a20),
            ],
        )
    } else {
        vt
    };
    let cache = obj(e, 0x1114, cache_vt);
    set(e, cache + 0x10f8, device);
    set(e, RENDER_STATE, cache);
    set(e, STATE_TARGET, cache);
    let renderer = obj(e, 0x900, vt);
    set(e, RENDERER, renderer);
    // The texture manager: the real lookup in its slot 0xc, or a stand-in.
    let manager_vt = vtable(e, &[(0xc, 0x00e9_0b10)]);
    let manager = obj(e, 0x10, manager_vt);
    set(e, manager + 0xc, renderer);
    let stand_in_manager = obj(e, 0x10, vt);
    set(e, renderer + 0x8c4, manager);
    let lock = e.mem.alloc(0x200);
    set(e, TEXTURE_RENDERER, lock);
    // Texture data of the source class, the rendered class and one with stand-ins only.
    let source_vt = vtable(
        e,
        &[
            (0x98, 0x00e6_87f0),
            (0x9c, 0x00e6_8800),
            (0xa4, 0x00e7_cd40),
            (0xa8, 0x00e6_8810),
            (0xac, 0x00e7_cd40),
        ],
    );
    let rendered_vt = vtable(
        e,
        &[
            (0x98, 0x00e6_87f0),
            (0x9c, 0x00e6_8800),
            (0xa4, 0x00e6_8810),
            (0xa8, 0x00e7_cd40),
            (0xac, 0x00e7_cd40),
        ],
    );
    let data = obj(e, 0x70, source_vt);
    let rendered = obj(e, 0x70, rendered_vt);
    let stand_in_data = obj(e, 0x70, vt);
    for d in [data, rendered, stand_in_data] {
        set(e, d + 0xc, 0x100);
        set(e, d + 0x10, 0x40);
        set(e, d + 0x64, 0x7e00_1000);
        set(e, d + 0x68, 3);
    }
    let texture = obj(e, 0x50, vt);
    set(e, texture + 0x24, data);
    // The stage's state cache, the stage and the pass.
    let stage_cache = e.mem.alloc(0xb8);
    let stage = e.mem.alloc(0x68);
    set(e, stage + 4, 1);
    set(e, stage + 8, texture);
    set(e, stage + 0x10, stage_cache);
    set(e, STAGE_COUNT, 4);
    let property_vt = vtable(e, &[(0xe0, 0x00b6_71c0)]);
    let property = obj(e, 0x100, property_vt);
    let stand_in_property = obj(e, 0x100, vt);
    for p in [property, stand_in_property] {
        let source = e.mem.alloc(4);
        set(e, source, texture);
        set(e, p + 0xac, source);
        set(e, p + 0xd8, 2);
    }
    let mut stages = [0u32; 3];
    for (i, s) in stages.iter_mut().enumerate() {
        *s = e.mem.alloc(0x68);
        set(e, *s + 4, i as u32);
        set(e, *s + 8, texture);
        set(e, *s + 0x10, stage_cache);
        e.mem.set_u8(*s + 0x64, 1);
    }
    let pass_object = e.mem.alloc(0x40);
    let array = e.mem.alloc(0x10);
    for (i, s) in stages.iter().enumerate() {
        set(e, array + 4 * i as u32, *s);
    }
    set(e, pass_object + 0x18, 3);
    set(e, pass_object + 0x24, array);
    set(e, CURRENT_PASS, pass_object);
    // The view object whose matrix the geometry pass inverts.
    let view = e.mem.alloc(0xb0);
    for (i, w) in [0x3f80_0000u32, 0, 0, 0, 0x3f80_0000, 0, 0, 0, 0x3f80_0000]
        .iter()
        .enumerate()
    {
        set(e, view + 0x68 + 4 * i as u32, *w);
    }
    set(e, view + 0x68 + 0x30, 0x3f80_0000);
    let holder = e.mem.alloc(4);
    set(e, holder, view);
    set(e, VIEW_HOLDER, holder);
    W {
        renderer,
        manager,
        stand_in_manager,
        cache,
        stage_cache,
        stage,
        texture,
        data,
        rendered,
        stand_in_data,
        property,
        stand_in_property,
        pass_object,
        stages,
    }
}

trait Common {
    /// The render family with the shader family real (`real_shader`) or standing in.
    fn common(self, real_shader: bool) -> Diff;
}

impl Common for Diff {
    fn common(self, real_shader: bool) -> Diff {
        let d = self
            .family(&render::RENDER)
            .family(&leaf::LEAVES)
            .family(&dispatch::DISPATCH)
            // What dispatch calls in turn (the 4x4 inverse's helpers are pathing's).
            .family(&pathing::PATHING)
            .family(&culling::CULLING)
            .family(&crt::CRT)
            .max_calls(MAX_CALLS)
            .max_cases(200);
        if real_shader {
            d.family(&shader::SHADER)
        } else {
            d.goes_by_address(SHADER_CALLEES)
        }
    }
}

const TWO_WAYS: &[u32] = &[0, 1];

#[test]
fn bind() {
    Diff::new(entry!(0x00e7_eb00, stage::bind(Ptr<_>, u8) -> bool))
        .common(false)
        .inputs(&[
            Input::Of(&[0, 1, 0x100]),
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
            Input::Of(&[0, 6]),
        ])
        .translation_calls_plainly(&[0x00e7_ea00, 0x00e7_e940, 0x00e7_df90])
        .setup(|e, c| {
            let w = world(e, false);
            // The texture last bound to the stage: this one or another.
            let shadow = if c.word(1) != 0 { w.texture } else { 0x1234 };
            set(e, SHADOWED + 4, shadow);
            set(e, APPLY_STATES_ANSWER, c.word(2));
            set(e, w.stage, c.word(3));
            vec![w.stage, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn apply() {
    for real_shader in [true, false] {
        Diff::new(entry!(0x00e7_ea00, stage::apply(Ptr<_>)))
            .common(real_shader)
            .inputs(&[
                Input::Of(&[0, 3, 4, 9]),
                Input::Of(TWO_WAYS),
                Input::Of(&[0, 1, 2]),
                Input::Of(&[0, 6]),
                Input::Of(&[0, 1, 0x101]),
            ])
            .translation_calls_plainly(&[0x00e7_e940, 0x00e7_df90])
            .setup(move |e, c| {
                let w = world(e, real_shader);
                set(e, w.stage + 4, c.word(0));
                // A cached texture coordinate index in bank B or none.
                e.mem.set_u8(w.stage_cache + 0x5c, c.word(1) as u8);
                set(e, w.stage_cache + 0x3c, 0x77);
                set(e, CACHED_ANSWER, c.word(1));
                // No texture, the real lookup, or a stand-in lookup.
                match c.word(2) {
                    0 => set(e, w.stage + 8, 0),
                    1 => {}
                    _ => set(e, w.renderer + 0x8c4, w.stand_in_manager),
                }
                set(e, RETS + 0xc, c.word(4).wrapping_mul(0x40));
                set(e, w.stage, c.word(3));
                set(e, RETS + 0xa0, c.word(4) * 4);
                vec![w.stage]
            })
            .run()
            .assert_same();
    }
}

#[test]
fn send_transform() {
    Diff::new(entry!(0x00e7_e940, stage::send_transform(Ptr<_>)))
        .common(true)
        .inputs(&[
            Input::Of(&[1, 4]),
            Input::Of(&[0, 1, 0x100]),
            Input::Of(&[0, 0x1000_0000, 0x2000_0000, 0x3000_0000]),
            Input::Of(&[0, 1, 2]),
        ])
        .translation_calls_plainly(&[0x00e7_df90])
        .setup(|e, c| {
            let w = world(e, true);
            set(e, w.stage + 4, c.word(0));
            e.mem.set_u8(w.stage + 0x5e, c.word(1) as u8);
            set(e, w.stage + 0x14, c.word(2) | c.word(3));
            let source = e.mem.alloc(0x20);
            let matrix = e.mem.alloc(0x40);
            for i in 0..16 {
                set(e, matrix + 4 * i, 0x3f80_0000 + i);
            }
            set(e, source + 0x14, matrix);
            set(e, w.stage + 0x18, source);
            vec![w.stage]
        })
        .run()
        .assert_same();
}

#[test]
fn send_sampler_commands() {
    for real_shader in [true, false] {
        Diff::new(entry!(0x00e7_dc90, stage::send_sampler_commands(Ptr, u32)))
            .common(real_shader)
            .inputs(&[Input::Of(&[0, 1, 0x100, 0x101]), Input::Of(&[0, 4, 8, 12])])
            .setup(move |e, c| {
                let w = world(e, real_shader);
                set(e, RETS + 0xa0, c.word(1));
                vec![w.stage, c.word(0)]
            })
            .run()
            .assert_same();
    }
}

#[test]
fn texture_data() {
    Diff::new(entry!(
        0x00e9_0b10,
        stage::texture_data(Ptr, u32, u32, u32, u32) -> u32
    ))
    .common(true)
    .inputs(&[
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 1, 2]),
        Input::Of(TWO_WAYS),
        Input::Of(&[0, 1, 2]),
    ])
    .setup(|e, c| {
        let w = world(e, true);
        // The texture's data: the source class, the rendered class, stand-ins, or none.
        let data = [w.data, w.rendered, w.stand_in_data, 0][c.word(0) as usize];
        set(e, w.texture + 0x24, data);
        // Without data: no source (no RTTI from the texture's slot 8), or a source of the class
        // `011f444c` the maker turns into texture data.
        set(e, RETS + 0x08, if c.word(1) == 0 { 0 } else { 0x011f_444c });
        set(
            e,
            MADE_ANSWER,
            if c.word(1) == 2 { w.rendered } else { w.data },
        );
        for d in [w.data, w.rendered, w.stand_in_data] {
            // Width and height: both powers of two, the width not, the height not.
            let (width, height) = [(0x100, 0x40), (0x30, 0x40), (0x100, 0x30)][c.word(2) as usize];
            set(e, d + 0xc, width);
            set(e, d + 0x10, height);
        }
        e.mem.set_u8(w.texture + 0x40, c.word(3) as u8);
        // What the stand-in slots answer.
        set(e, RETS + 0xa4, [0, w.rendered, 0][c.word(4) as usize]);
        set(e, RETS + 0xa8, [0, 0, w.data][c.word(4) as usize]);
        set(e, RETS + 0x9c, c.word(4));
        set(e, RETS + 0x98, c.word(4));
        let out = e.mem.alloc(4);
        let texture = if c.word(0) == 3 && c.word(1) == 0 {
            0
        } else {
            w.texture
        };
        vec![w.manager, texture, out, out + 1, out + 2]
    })
    .run()
    .assert_same();
}

/// A getter's setup: an object with the case's word at `at`.
fn word_at(at: u32) -> impl Fn(&mut Engine, &engine_core::typed::diff::Case) -> Vec<u32> {
    move |e, c| {
        let o = e.mem.alloc(0x100);
        set(e, o + at, c.word(0));
        vec![o]
    }
}

#[test]
fn d3d_texture() {
    Diff::new(entry!(0x00e6_8800, stage::d3d_texture(u32) -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(word_at(0x64))
        .run()
        .assert_same();
}

#[test]
fn levels() {
    Diff::new(entry!(0x00e6_87f0, stage::levels(u32) -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(word_at(0x68))
        .run()
        .assert_same();
}

#[test]
fn this_word() {
    Diff::new(entry!(0x00e6_8810, stage::this_word(u32) -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(word_at(0))
        .run()
        .assert_same();
}

#[test]
fn clamp_mode() {
    Diff::new(entry!(0x00b6_71c0, pass::clamp_mode(u32) -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(word_at(0xd8))
        .run()
        .assert_same();
}

#[test]
fn bind_pass_stages() {
    Diff::new(entry!(0x00be_2170, pass::bind_pass_stages()))
        .common(false)
        .inputs(&[
            Input::Of(&[0, 1, 3]),
            Input::Of(&[0, 1, 2, 3]),
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[0x00e7_ea00, 0x00e7_e940, 0x00e7_df90])
        .setup(|e, c| {
            let w = world(e, false);
            set(e, w.pass_object + 0x18, c.word(0));
            // A null stage ends the walk; one stage disabled, or without a texture.
            let array = e.mem.u32(w.pass_object + 0x24);
            if c.word(1) < 3 {
                set(e, array + 4 * c.word(1), 0);
            }
            e.mem.set_u8(w.stages[0] + 0x64, c.word(2) as u8);
            if c.word(3) != 0 {
                set(e, w.stages[1] + 8, 0);
            }
            vec![]
        })
        .run()
        .assert_same();
}

#[test]
fn geometry_pass() {
    Diff::new(entry!(0x00b7_dde0, pass::geometry_pass(Ptr, Ptr)))
        .common(false)
        .inputs(&[
            Input::Of(TWO_WAYS),
            Input::Of(&[0, 0x40]),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[
            0x00e7_ea00,
            0x00e7_e940,
            0x00e7_df90,
            // The 4x4 inverse's translation reaches the matrix row and column helpers plainly.
            0x0047_6930,
            0x0047_6980,
            0x0047_69c0,
        ])
        .setup(|e, c| {
            let w = world(e, false);
            // The lighting property (its clamp mode typed) or one with stand-ins.
            let property = if c.word(0) != 0 {
                w.stand_in_property
            } else {
                w.property
            };
            set(e, RETS + 0xe0, 3);
            // Stage 1's texture: none, or the world's.
            set(
                e,
                TEXTURE_ANSWER,
                if c.word(1) != 0 { w.texture } else { 0 },
            );
            set(e, APPLY_STATES_ANSWER, c.word(2));
            let shader = e.mem.alloc(0x40);
            let pass = e.mem.alloc(0x10);
            set(e, pass + 0xc, property);
            vec![shader, pass]
        })
        .run()
        .assert_same();
}

/// The shader family's call sites this step made direct through the render state cache's slots: the
/// stage and sampler states of a stage cache (slots 0xc0 and 0xcc), and the cull mode (slot 0x68,
/// whose lock word is a byte for the direct branch).
#[test]
fn shader_sites_through_the_cache() {
    // (set bytes and values of the first bank, of the second).
    for (f, banks) in [
        (
            entry!(
                0x00e8_9250,
                shader::state::apply_stage_states(Ptr<_>, u32) -> u32
            ),
            [(0x2cu32, 0x0cu32), (0x5c, 0x3c)],
        ),
        (
            entry!(
                0x00e8_92d0,
                shader::state::apply_sampler_states(Ptr<_>, u32) -> u32
            ),
            [(0x80, 0x6c), (0xa4, 0x90)],
        ),
    ] {
        for real_shader in [true, false] {
            Diff::new(f)
                .common(real_shader)
                .inputs(&[Input::Of(&[0, 1, 0x21, 0x3ff]), Input::Of(&[0, 1, 3])])
                .setup(move |e, c| {
                    let w = world(e, real_shader);
                    for (k, (set_at, values)) in banks.iter().enumerate() {
                        for i in 0..5 {
                            let bit = (c.word(0) >> (i + 5 * k as u32)) & 1;
                            e.mem.set_u8(w.stage_cache + set_at + i, bit as u8);
                            set(e, w.stage_cache + values + 4 * i, 0x10 + i + 8 * k as u32);
                        }
                    }
                    vec![w.stage_cache, c.word(1)]
                })
                .run()
                .assert_same();
        }
    }
    for real_shader in [true, false] {
        Diff::new(entry!(
            0x00e8_79b0,
            shader::state::set_cull_mode(Ptr<_>, u32, u32)
        ))
        .common(real_shader)
        .inputs(&[Input::Of(&[0, 1]), Input::Of(&[0, 1, 0x100, u32::MAX])])
        .setup(move |e, c| {
            let w = world(e, real_shader);
            vec![w.cache, c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
    }
}

/// The vector constructor iterator the dispatch family's transpose calls (`00401050`), with the
/// matrices' constructor (the leaf `006815c0`, direct) or another (a stand-in, by address).
#[test]
fn construct_array() {
    Diff::new(entry!(
        0x0040_1050,
        dispatch::math::construct_array(u32, u32, i32, u32)
    ))
    .common(true)
    .inputs(&[
        Input::Of(&[0, 1, 3, u32::MAX, 0xffff_fffe]),
        Input::Of(&[4, 12]),
        Input::Of(TWO_WAYS),
    ])
    .setup(|e, c| {
        let _ = world(e, true);
        let array = e.mem.alloc(0x40);
        let constructor = if c.word(2) != 0 {
            0x0068_15c0
        } else {
            VT_FN + 0x04
        };
        vec![array, c.word(1), c.word(0), constructor]
    })
    .run()
    .assert_same();
}

/// The untyped callees of `GetRenderPasses` and of the geometry packing: stand-ins answering from the
/// table (`RETS + 0x600` on), each leaving a mark so a call moved shows in memory.
fn pass_stand_ins(e: &mut Engine) {
    for addr in [
        0x00ba_9660u32,
        0x00bd_f650,
        0x00ba_9740,
        0x00bd_a060,
        0x00ba_96d0,
        0x00bd_e170,
        0x00ba_9820,
        0x00bd_f6c0,
        0x00b7_0390,
        0x00e6_d760,
        0x00e6_d780,
        0x00e8_bf90,
        0x00e8_ef10,
        0x00e8_f200,
        0x00e8_eee0,
    ] {
        e.register(addr, |e, a| {
            let n = e.mem.u32(RETS + 0x500);
            e.mem.set_u32(
                RETS + 0x500,
                n.wrapping_mul(31).wrapping_add(a.len() as u32),
            );
            ret(e.mem.u32(RETS + 0x600))
        });
    }
    e.register(0x00a5_9d30, |e, _| ret(e.mem.u32(RETS + 0x604)));
    e.register(0x005b_f7b0, |e, _| ret(e.mem.u32(RETS + 0x608)));
    e.register(0x0043_b300, |e, _| ret(e.mem.u32(RETS + 0x60c)));
    e.register(0x00a7_b850, |e, _| ret(e.mem.u32(RETS + 0x610)));
    e.register(0x00a7_5f10, |e, _| ret(e.mem.u32(RETS + 0x614)));
    e.register(0x00e8_c1e0, |e, _| ret(e.mem.u32(RETS + 0x618)));
    e.register(0x004b_3ab0, |_, _| Ret {
        st0: 0.25,
        ..Ret::default()
    });
}

/// A lighting property with its pass arrays, a geometry, an accumulator with a camera: (property,
/// geometry, accumulator).
fn property_world(e: &mut Engine) -> (u32, u32, u32) {
    let w = world(e, true);
    pass_stand_ins(e);
    let vt = e.mem.u32(w.stand_in_property);
    let property = obj(e, 0xc0, vt);
    for at in [0x3c, 0x40, 0x44, 0x48, 0x54] {
        let array = e.mem.alloc(0x14);
        let list = e.mem.alloc(0x10);
        for i in 0..3u32 {
            let pass = e.mem.alloc(0x10);
            e.mem.set_u16(pass + 4, [0xfc, 0x10, 0x102][i as usize]);
            let target = e.mem.alloc(4);
            set(e, pass + 0xc, target);
            set(e, list + 4 * i, pass);
        }
        set(e, array + 4, list);
        set(e, array + 0x10, 3);
        set(e, property + at, array);
    }
    set(e, property + 0x28, 0x3f80_0000);
    set(e, property + 0x2c, 0x3f40_0000);
    set(e, property + 0x34, 0x4120_0000);
    let geometry = e.mem.alloc(0xc0);
    let bound = e.mem.alloc(0x10);
    for (i, word) in [0x4120_0000u32, 0, 0, 0x3f80_0000].iter().enumerate() {
        set(e, bound + 4 * i as u32, *word);
    }
    set(e, geometry + 0x20, bound);
    let geometry_property = e.mem.alloc(0x20);
    e.mem.set_u16(geometry_property + 0x18, 0x201);
    set(e, geometry + 0x9c, geometry_property);
    let accumulator = e.mem.alloc(0x1a0);
    let camera = e.mem.alloc(0x200);
    set(e, camera + 0x1f0, 0x3f80_0000);
    set(e, accumulator + 0x194, camera);
    let shader_accumulator = e.mem.alloc(0x10);
    set(e, 0x011f_95ec, shader_accumulator);
    let shader = e.mem.alloc(0x120);
    set(e, shader + 0x110, 0x4000_0000);
    set(e, shader_accumulator + 8, shader);
    for (at, value) in [
        (0x011f_9458u32, 0x4120_0000u32),
        (0x011f_9460, 0x4100_0000),
        (0x011f_9468, 0x4110_0000),
        (0x011f_946c, 0x40a0_0000),
        (0x011f_9470, 0x4130_0000),
    ] {
        set(e, at, value);
    }
    (property, geometry, accumulator)
}

/// `GetRenderPasses`: each special pass type and the main rebuild, with and without fades (the camera
/// distance measured), a material, the shader level, the state matching the key, depth passes; the
/// special arrays as made, with their first pass's byte +9 set, or empty (their pass added); the bound
/// centre and the shader's scale with low mantissa bits.
#[test]
fn render_passes() {
    Diff::new(entry!(
        0x00b6_7be0,
        passes::render_passes(u32, u32, u32, u32, u32, u32, u32) -> u32
    ))
    .common(true)
    .inputs(&[
        Input::Of(&[0x10, 7, 8, 9, 0xf]),
        Input::Of(&[0, 1, 0x101]),
        Input::Of(&[0, 1, 0x80, 0x0402_0000, 0x0001_8000]),
        Input::Of(&[0, 1, 2]),
        Input::Of(TWO_WAYS),
        Input::Of(TWO_WAYS),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0x4120_0000, 0x4123_d70b, 0x3fb5_04f3]),
    ])
    .max_cases(600)
    // The point length's root: the translation of `004019b0` takes it plainly.
    .translation_calls_plainly(&[0x00b6_6760, 0x0040_19d0])
    .setup(|e, c| {
        let (property, geometry, accumulator) = property_world(e);
        set(e, property + 0x20, c.word(2));
        set(e, 0x011f_91c0, c.word(3));
        if c.word(4) != 0 {
            let material = e.mem.alloc(0x40);
            set(e, material + 0x3c, 0x3f00_0000);
            set(e, geometry + 0xa4, material);
        }
        set(e, RETS + 0x608, c.word(5));
        // The state already at the key, once.
        if c.word(5) != 0 {
            set(e, property + 0x38, (c.word(0) << 8) | 3);
        }
        // The special arrays (depth map, constant alpha, silhouette): first pass's +9 set, or empty.
        for at in [0x40, 0x44, 0x54] {
            let array = e.mem.u32(property + at);
            match c.word(6) {
                1 => {
                    let first = e.mem.u32(e.mem.u32(array + 4));
                    e.mem.set_u8(first + 9, 1);
                }
                2 => set(e, array + 0x10, 0),
                _ => {}
            }
        }
        // The renderer's `+8` object's word `+0x1a0`, what a refreshed first pass gets.
        let inner = e.mem.alloc(0x1a4);
        set(e, inner + 0x1a0, 0x5151_0001);
        let renderer = e.mem.alloc(0x10);
        set(e, renderer + 8, inner);
        set(e, 0x011f_4748, renderer);
        // The bound centre's x and the shader's scale.
        let bound = e.mem.u32(geometry + 0x20);
        set(e, bound, c.word(7));
        let shader = e.mem.u32(e.mem.u32(0x011f_95ec) + 8);
        set(e, shader + 0x110, c.word(7));
        let count = e.mem.alloc(4);
        vec![
            property,
            geometry,
            3,
            count,
            c.word(0),
            accumulator,
            c.word(1),
        ]
    })
    .run()
    .assert_same();
}

#[test]
fn shader_accumulator() {
    Diff::new(entry!(0x00b4_f5c0, passes::shader_accumulator() -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(|e, c| {
            let _ = world(e, true);
            set(e, 0x011f_95ec, c.word(0));
            vec![]
        })
        .run()
        .assert_same();
}

#[test]
fn has_flag() {
    Diff::new(entry!(0x004a_2020, passes::has_flag(u32, u32) -> bool))
        .common(true)
        .inputs(&[Input::Of(&[0, 5, 0x1b, 0x23, 63]), Input::Word])
        .setup(|e, c| {
            let o = e.mem.alloc(0x40);
            set(e, o + 0x20, c.word(1));
            set(e, o + 0x24, !c.word(1));
            vec![o, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn camera_position() {
    Diff::new(entry!(0x005b_b9b0, passes::camera_position(u32, u32) -> u32))
        .common(true)
        .inputs(&[Input::Word])
        .setup(|e, c| {
            let o = e.mem.alloc(0x200);
            for i in 0..3 {
                set(e, o + 0x1f0 + 4 * i, c.word(0) ^ i);
            }
            let out = e.mem.alloc(0x10);
            vec![o, out]
        })
        .run()
        .assert_same();
}

#[test]
fn lowered_pass_number() {
    Diff::new(entry!(0x00b6_6760, passes::lowered_pass_number(u32) -> u32))
        .common(true)
        .inputs(&[Input::Of(&[0xfc, 0x102, 0xfd, 0, 0x1fc])])
        .setup(|_, c| vec![c.word(0)])
        .run()
        .assert_same();
}

/// A shader, its renderer, a geometry with data and buffer data, for the streams and the packer:
/// (shader, geometry, buffer data, data, renderer).
fn stream_world(e: &mut Engine, real_declaration: bool) -> (u32, u32, u32, u32, u32) {
    let w = world(e, true);
    pass_stand_ins(e);
    let vt = e.mem.u32(w.stand_in_property);
    let shader = obj(e, 0x40, vt);
    let device = obj(e, 0x10, vt);
    set(e, shader + 0x14, device);
    let renderer = obj(e, 0x900, vt);
    let group = obj(e, 0x10, vt);
    set(e, renderer + 0x8bc, group);
    let manager = obj(e, 0x10, vt);
    set(e, renderer + 0x8c0, manager);
    set(e, shader + 0x18, renderer);
    let target = if real_declaration {
        w.cache
    } else {
        obj(e, 0x10, vt)
    };
    set(e, shader + 0x1c, target);
    let geometry = e.mem.alloc(0xc0);
    let data = obj(e, 0x50, vt);
    set(e, geometry + 0xb8, data);
    let buffer = e.mem.alloc(0x60);
    set(e, data + 0x34, buffer);
    let owner = obj(e, 0x10, vt);
    set(e, buffer + 8, owner);
    let strides = e.mem.alloc(8);
    set(e, strides, 0x20);
    let chips = e.mem.alloc(8);
    let chip = e.mem.alloc(0x10);
    set(e, chip + 8, 0x0400_1000);
    set(e, chips, chip);
    set(e, buffer + 0x24, strides);
    set(e, buffer + 0x28, chips);
    set(e, buffer + 0x34, 0x0400_2000);
    (shader, geometry, buffer, data, renderer)
}

/// `00e812f0`: a geometry or none, the word +0x38, the data's declaration (its slot 0x8c set or not)
/// or the shader's own, the cache's `SetVertexDeclaration` typed in its slot or a stand-in, a skin
/// partition or none, the stream count.
#[test]
fn prepare_streams() {
    Diff::new(entry!(
        0x00e8_12f0,
        stream::prepare_streams(Ptr, u32, u32, u32, u32) -> u32
    ))
    .common(true)
    .inputs(&[
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 1, 2]),
        Input::Of(TWO_WAYS),
        Input::Of(TWO_WAYS),
        Input::Of(&[0, 1]),
    ])
    .setup(|e, c| {
        let (shader, geometry, buffer, data, _) = stream_world(e, c.word(2) != 0);
        if c.word(0) == 2 {
            set(e, shader + 0x38, 1);
        }
        let vt = e.mem.u32(shader);
        match c.word(1) {
            1 => {
                let declaration = obj(e, 0x30, vt);
                set(e, declaration + 0x2c, 0x77);
                set(e, data + 0x30, declaration);
            }
            2 => {
                let own = obj(e, 0x10, vt);
                set(e, shader + 0x28, own);
            }
            _ => {}
        }
        set(e, RETS + 0x8c, c.word(3));
        set(e, RETS + 0xb4, 0x55);
        set(e, buffer + 0x20, c.word(4));
        let partition = if c.word(3) != 0 && c.word(1) == 0 {
            let p = e.mem.alloc(0x30);
            set(e, p + 0x28, buffer);
            p
        } else {
            0
        };
        let g = if c.word(0) == 0 { 0 } else { geometry };
        vec![shader, g, partition, buffer, 0]
    })
    .run()
    .assert_same();
}

/// `PackGeometryBuffer`: packed or not, forced or not, the data's kind (triangle list, strips, other),
/// the dirty bits, a source or none, the index buffer made or not.
#[test]
fn pack_geometry_buffer() {
    Diff::new(entry!(
        0x00e7_1fe0,
        stream::pack_geometry_buffer(Ptr, u32, u32, u32, u32, u8) -> bool
    ))
    .common(true)
    .inputs(&[
        Input::Of(&[0, 1, 2]),
        Input::Of(TWO_WAYS),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 0x4000, 0x0030, 0x4fff]),
        Input::Of(TWO_WAYS),
        Input::Of(&[0, 0x0500_0000]),
    ])
    .max_cases(300)
    .setup(|e, c| {
        let (_, _, buffer, data, renderer) = stream_world(e, true);
        // Packed: the buffer data has streams (the shader family's `has_streams`).
        set(e, buffer + 0x20, c.word(0));
        e.mem.set_u16(data + 0xe, c.word(3) as u16);
        e.mem.set_u16(data + 0xc, 0x1001);
        set(e, data + 0x24, 1);
        // The base count and the strip count (a strip list's index count adds two a strip).
        e.mem.set_u16(data + 0x40, 9);
        e.mem.set_u16(data + 0x44, 3);
        // The data's kind: its RTTI chain (slot 8) holds the triangle list data, or the strip test.
        let rtti = e.mem.alloc(8);
        let next = if c.word(2) == 0 { 0x011f_4a88 } else { 0 };
        set(e, rtti + 4, next);
        set(e, RETS + 0x08, rtti);
        set(e, RETS + 0x60c, u32::from(c.word(2) == 1));
        set(e, RETS + 0x90, 12);
        set(e, RETS + 0xa8, 4);
        set(e, RETS + 0x610, c.word(5));
        set(e, RETS + 0x614, c.word(5));
        set(e, RETS + 0x600, c.word(5));
        set(e, RETS + 0x618, 1);
        let vt = e.mem.u32(renderer);
        let source = if c.word(4) != 0 {
            let s = obj(e, 0x30, vt);
            set(e, s + 0x20, 1);
            s
        } else {
            0
        };
        let object = e.mem.alloc(0x20);
        set(e, object + 0x18, 0x99);
        vec![renderer, buffer, data, object, source, c.word(1)]
    })
    .run()
    .assert_same();
}

//! `BSShader/bscubemapcamera.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// The vtable of `BSCubeMapCamera` (`010bc470`, no RTTI).
const VTABLE: u32 = 0x010b_c470;
/// `NiCamera`'s constructor (`00a712f0`) and destructor (`00a6fae0`),
/// `thiscall`, no stack words.
const CAMERA_CONSTRUCTOR: u32 = 0x00a7_12f0;
const CAMERA_DESTRUCTOR: u32 = 0x00a6_fae0;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, five words:
/// array, element size, count, constructor, destructor) and `eh vector
/// destructor iterator` (`00ec5fce`, four words), both cleaned by the
/// callee, with the `NiPointer` element constructor `00b54620` and
/// destructor `0045cec0`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const POINTER_CONSTRUCT: u32 = 0x00b5_4620;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiFrustum::NiFrustum` (`00a71b70`, `thiscall`, one word: orthographic).
const FRUSTUM_CONSTRUCTOR: u32 = 0x00a7_1b70;
/// The CRT's `tan` (`00eca5d0`, the angle in ST0, here a leading `f64`).
const TAN: u32 = 0x00ec_a5d0;
/// The `double` angle (`0103b100`, a quarter turn's half: pi/4 as a
/// `float`) and the `float` far distance (`01018bfc`, 2048.0) of the view.
const HALF_FIELD_OF_VIEW: u32 = 0x0103_b100;
const FAR_DISTANCE: u32 = 0x0101_8bfc;
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, `thiscall`,
/// three words).
const ACCUMULATOR_CONSTRUCTOR: u32 = 0x00b6_60d0;

/// The camera's fields on PC: the six face `NiPointer`s (+0x118), the words
/// +0x114 (the constructor's argument), the `NiPointer`s +0x130, +0x138 and
/// the accumulator +0x13C; `NiCamera::m_kViewFrustum` (Xbox PDB) +0xDC.
const FACES: u32 = 0x118;
const OWNER: u32 = 0x114;
const POINTER_130: u32 = 0x130;
const POINTER_138: u32 = 0x138;
const ACCUMULATOR: u32 = 0x13c;
const VIEW_FRUSTUM: u32 = 0xdc;

/// An `NiPointer`'s release: the count at +4 decremented, the object
/// deleted through its vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` member's release and clear.
fn release_and_clear(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(member, 0);
    }
}

/// An `NiPointer` member's release without clearing.
fn release_member(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
    }
}

// Translated from 00bd74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`BSCubeMapCamera`'s destructor by its vtable and fields): the
/// vtable, the six faces released and cleared, +0x130 and +0x13C released
/// and cleared, then the member destructors of +0x13C, +0x138 and +0x130,
/// the faces' vector destructor and `NiCamera`'s destructor.
// Translation incomplete: 00bd74a0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2d00a, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bd74a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for face in 0..6 {
        release_and_clear(e, base + FACES + 4 * face);
    }
    release_and_clear(e, base + POINTER_130);
    release_and_clear(e, base + ACCUMULATOR);
    release_member(e, base + ACCUMULATOR);
    release_member(e, base + POINTER_138);
    release_member(e, base + POINTER_130);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + FACES, 4u32, 6u32, POINTER_DESTRUCT],
    );
    e.call(CAMERA_DESTRUCTOR, &args![this]);
}

// Translated from 00bd8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCubeMapCamera::BSCubeMapCamera` (Xbox PDB): `NiCamera`'s constructor,
/// the six face pointers constructed, the vtable, the pointers +0x130,
/// +0x138 and +0x13C cleared, `owner` kept at +0x114, the faces and +0x130,
/// +0x138 released and cleared (null here); the view frustum is a default
/// `NiFrustum` with left and bottom `-tan(pi/4)`, right and top `tan(pi/4)`
/// (the `float`s through the x87 stack and the argument's slot), near 1 and
/// far 2048; then a new `BSShaderAccumulator` (0x280 bytes; `(0x63, 1,
/// 0x2F7)`; null when the allocation fails) becomes the accumulator +0x13C.
/// Returns `this`.
// Translation incomplete: 00bd8290 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2d13c, state variable) and the security-cookie bookkeeping are not translated.
pub fn bs_cube_map_camera_bs_cube_map_camera(e: &mut Engine, this: Ptr, owner: u32) -> Ptr {
    let base = this.addr();
    e.call(CAMERA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + FACES,
            4u32,
            6u32,
            POINTER_CONSTRUCT,
            POINTER_DESTRUCT
        ],
    );
    e.mem.set_u32(base + POINTER_130, 0);
    e.mem.set_u32(base + POINTER_138, 0);
    e.mem.set_u32(base + ACCUMULATOR, 0);
    e.mem.set_u32(base + OWNER, owner);
    for face in 0..6 {
        release_and_clear(e, base + FACES + 4 * face);
    }
    release_and_clear(e, base + POINTER_130);
    release_and_clear(e, base + POINTER_138);
    e.with_stack(0x1c, |e, frustum| {
        e.call(FRUSTUM_CONSTRUCTOR, &args![frustum, 0u32]);
        let angle = f64::from_bits(e.global::<u64>(HALF_FIELD_OF_VIEW));
        let tangent = e.call(TAN, &args![angle]).f64();
        let tangent = x87_store32(tangent);
        let tangent = x87_store32(x87_load32(tangent));
        let negative = x87_store32(-x87_load32(tangent));
        let negative = x87_quiet_bits(negative);
        let f = frustum.addr();
        e.mem.set_u32(f, negative);
        e.mem.set_u32(f + 0xc, negative);
        e.mem.set_u32(f + 4, x87_quiet_bits(tangent));
        e.mem.set_u32(f + 8, x87_quiet_bits(tangent));
        e.mem.set_u32(f + 0x10, 1.0f32.to_bits());
        e.mem
            .set_u32(f + 0x14, x87_quiet_bits(e.global::<u32>(FAR_DISTANCE)));
        let bytes = e.mem.bytes(f, 0x1c);
        e.mem.write(base + VIEW_FRUSTUM, &bytes);
    });
    let block = e.call(NI_NEW, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            ACCUMULATOR_CONSTRUCTOR,
            &args![block, 0x63u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    let old = e.mem.u32(base + ACCUMULATOR);
    if old != accumulator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(base + ACCUMULATOR, accumulator);
        if accumulator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![accumulator + 4]);
        }
    }
    this
}

/// `NiCamera::LookAtWorldPoint` (engine map, `00a701b0`; `thiscall`: the point, the up
/// vector) and the three axis vectors of the cube faces (`011a946c`, `011a9478`, `011a9484`;
/// three `float`s each).
const LOOK_AT_WORLD_POINT_00A701B0: u32 = 0x00a7_01b0;
const AXIS_011A946C: u32 = 0x011a_946c;
const AXIS_011A9478: u32 = 0x011a_9478;
const AXIS_011A9484: u32 = 0x011a_9484;

// Translated from 00bd7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCubeMapCamera::SetupTransformation` (Xbox PDB), `thiscall`, one word (the face 0..5):
/// looks at the camera position (+0x8c) moved along an axis (the compiler's jump table as
/// `match` arms; A, B, C the axes at `011a946c`, `011a9478`, `011a9484`): face 0 at
/// `pos - A` with up `-B`, 1 at `A + pos` with up `-B`, 2 at `pos - B` with up `C` (passed
/// by address), 3 at `pos + B` with up `-C`, 4 at `pos - C` and 5 at `pos + C` with up `-B`
/// (`NiCamera::LookAtWorldPoint`; x87 through `float`s). Other faces do nothing.
pub fn bs_cube_map_camera_setup_transformation(e: &mut Engine, this: Ptr, face: u32) {
    let base = this.addr();
    let ld = |e: &Engine, addr: u32| x87_load32(e.mem.u32(addr));
    let m32 = |e: &Engine, addr: u32| x87_m32(e.mem.u32(addr));
    let negated =
        |e: &Engine, axis: u32| [0u32, 4, 8].map(|offset| x87_store32(-ld(e, axis + offset)));
    let position = base + 0x8c;
    let moved = |e: &Engine, axis: u32, subtract: bool| {
        [0u32, 4, 8].map(|offset| {
            let value = if subtract {
                x87_sub(ld(e, position + offset), m32(e, axis + offset))
            } else {
                x87_add(ld(e, position + offset), m32(e, axis + offset))
            };
            x87_store32(value)
        })
    };
    let (point, up) = match face {
        0 => (
            moved(e, AXIS_011A946C, true),
            Some(negated(e, AXIS_011A9478)),
        ),
        1 => {
            let up = negated(e, AXIS_011A9478);
            let x = x87_add(ld(e, AXIS_011A946C), m32(e, position));
            let y = x87_add(ld(e, position + 4), m32(e, AXIS_011A946C + 4));
            let z = x87_add(ld(e, position + 8), m32(e, AXIS_011A946C + 8));
            ([x, y, z].map(x87_store32), Some(up))
        }
        2 => (moved(e, AXIS_011A9478, true), None),
        3 => (
            moved(e, AXIS_011A9478, false),
            Some(negated(e, AXIS_011A9484)),
        ),
        4 => (
            moved(e, AXIS_011A9484, true),
            Some(negated(e, AXIS_011A9478)),
        ),
        5 => (
            moved(e, AXIS_011A9484, false),
            Some(negated(e, AXIS_011A9478)),
        ),
        _ => return,
    };
    e.with_stack(0x18, |e, local| {
        let point_at = local.addr();
        for (index, word) in point.iter().enumerate() {
            e.mem.set_u32(point_at + 4 * index as u32, *word);
        }
        let up_at = match up {
            Some(up) => {
                for (index, word) in up.iter().enumerate() {
                    e.mem.set_u32(point_at + 0xc + 4 * index as u32, *word);
                }
                point_at + 0xc
            }
            None => AXIS_011A9484,
        };
        e.call(LOOK_AT_WORLD_POINT_00A701B0, &args![base, point_at, up_at]);
    });
}

/// What `00bd7610` answers: the address `012023e8` (no RTTI locator for the vtable; the word
/// is the class's `NiRTTI` by its place in the vtable, unconfirmed).
const RECORD_012023E8: u32 = 0x0120_23e8;

// Translated from 00bd7610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 2 of the vtable `010bc470`, `thiscall`
/// without reading `this`: answers the address `012023e8`.
pub fn fn_00bd7610(_e: &mut Engine, _this: Ptr) -> u32 {
    RECORD_012023E8
}

/// The `NiPointer` at `slot` set to the word at `source` when they differ (old one released,
/// the source read again, new one held).
fn assign_from(e: &mut Engine, slot: u32, source: u32) {
    if e.mem.u32(slot) == e.mem.u32(source) {
        return;
    }
    release_member(e, slot);
    let new = e.mem.u32(source);
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

/// The renderer (`011f9508`), the culling helpers (`00a69400` constructor, one word;
/// `00a694a0`, one word; `00a693e0` destructor; unnamed, `thiscall`), the node update
/// `00a59c60` (`thiscall`, the address of an update record) and `00b9d020` (unnamed;
/// `thiscall` on the accumulating object, one word: the culler).
const RENDERER_011F9508: u32 = 0x011f_9508;
const CULLER_CONSTRUCT_00A69400: u32 = 0x00a6_9400;
const CULLER_SET_FRUSTUM_00A694A0: u32 = 0x00a6_94a0;
const CULLER_DESTRUCT_00A693E0: u32 = 0x00a6_93e0;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const ACCUMULATE_00B9D020: u32 = 0x00b9_d020;

/// The node update record: `float` time 0.0 and five clear bytes, given to `00a59c60`.
fn update_with_zero_record(e: &mut Engine, node: u32) {
    e.with_stack(0xc, |e, record| {
        e.mem.set_u32(record.addr(), x87_store32(0.0));
        for offset in 4..9u32 {
            e.mem.set_u8(record.addr() + offset, 0);
        }
        e.call(UPDATE_00A59C60, &args![node, record]);
    });
}

// Translated from 00bd7620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd7620 - the C++ exception frame (FS:[0] chain, unwind state 0 for the culler) and the security-cookie bookkeeping are not translated.
/// Unnamed in the engine map (unit `bscubemapcamera.cpp`), `thiscall`, one word (an object to
/// accumulate into). With it: the camera's rotation columns (+0x68.., +0x6c.., +0x70.., each
/// three `float`s through the FPU) go to the renderer's (`011f9508`) slot +0x18c with the
/// position +0x8c, the frustum +0xdc and +0x100 when its mode (+0x200) is 1 or 2 and its byte
/// +0x208 is 1; the accumulator (+0x13c) gets its slot +0x8c with the camera and its byte
/// +0x84 set; a culler on the stack (`00a69400` with 0) is set up with the camera (+0xc) and
/// the frustum (`00a694a0`) after the camera's update (`00a59c60`, a zero record), the object
/// runs `00b9d020` with it, the accumulator's slot +0x90 runs and its count (+4) is raised
/// when below 2; then the culler is destroyed (`00a693e0`).
pub fn fn_00bd7620(e: &mut Engine, this: Ptr, object: u32) {
    if object == 0 {
        return;
    }
    let base = this.addr();
    let column = |e: &Engine, first: u32| {
        [0u32, 0xc, 0x18].map(|step| x87_quiet_bits(e.mem.u32(base + first + step)))
    };
    let third = column(e, 0x70);
    let second = column(e, 0x6c);
    let first = column(e, 0x68);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let mode = e.mem.u32(renderer + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(renderer + 0x208) == 1 {
        e.with_stack(0x24, |e, local| {
            let at = local.addr();
            for (index, word) in first.iter().chain(&second).chain(&third).enumerate() {
                e.mem.set_u32(at + 4 * index as u32, *word);
            }
            e.vcall(
                renderer,
                0x18c,
                &args![
                    base + 0x8c,
                    at,
                    at + 0xc,
                    at + 0x18,
                    base + 0xdc,
                    base + 0x100
                ],
            );
        });
    }
    let accumulator = e.mem.u32(base + ACCUMULATOR);
    e.vcall(accumulator, 0x8c, &args![this]);
    let accumulator = e.mem.u32(base + ACCUMULATOR);
    e.mem.set_u8(accumulator + 0x84, 1);
    e.with_stack(0x90, |e, culler| {
        let culler = culler.addr();
        e.call(CULLER_CONSTRUCT_00A69400, &args![culler, 0u32]);
        update_with_zero_record(e, base);
        e.mem.set_u32(culler + 0xc, base);
        e.call(
            CULLER_SET_FRUSTUM_00A694A0,
            &args![culler, base + VIEW_FRUSTUM],
        );
        e.call(ACCUMULATE_00B9D020, &args![object, culler]);
        let accumulator = e.mem.u32(base + ACCUMULATOR);
        e.vcall(accumulator, 0x90, &args![]);
        let accumulator = e.mem.u32(base + ACCUMULATOR);
        if e.mem.u32(accumulator + 4) < 2 {
            e.call(INTERLOCKED_INCREMENT, &args![accumulator + 4]);
        }
        e.call(CULLER_DESTRUCT_00A693E0, &args![culler]);
    });
}

/// What the face renderers call: `BSRenderedTexture::StopOffscreen` (`00b6b260`; `thiscall`),
/// `00b6b8d0` and `00b6b790` (unit `run_00b6b330`; `cdecl`), `00b71360` (unnamed; `cdecl`,
/// three words), `BSShaderUtil::AccumulateScene` (`00b6bee0`; `cdecl`, three words),
/// `00b6c0d0` (unnamed; `cdecl`, three words), `BSCullingProcess::~BSCullingProcess`
/// (`004a0f60`) and the culling process's vtable `0101e2ec`, the background colour words
/// (`011a9bd0`), the `float` `01017a3c` and the mode word `011f91b8`.
const STOP_OFFSCREEN_00B6B260: u32 = 0x00b6_b260;
const BEGIN_TARGETS_00B6B8D0: u32 = 0x00b6_b8d0;
const END_TARGETS_00B6B790: u32 = 0x00b6_b790;
const FN_00B71360: u32 = 0x00b7_1360;
const ACCUMULATE_SCENE_00B6BEE0: u32 = 0x00b6_bee0;
const FN_00B6C0D0: u32 = 0x00b6_c0d0;
const CULLING_DESTRUCT_004A0F60: u32 = 0x004a_0f60;
const CULLING_VTABLE_0101E2EC: u32 = 0x0101_e2ec;
const BACKGROUND_011A9BD0: u32 = 0x011a_9bd0;
const GREY_01017A3C: u32 = 0x0101_7a3c;
const MODE_011F91B8: u32 = 0x011f_91b8;

/// The face's bit: `1 << face` for faces 0..5 (the compiler's jump table as `match` arms),
/// else 0.
fn face_bit(face: u32) -> u32 {
    match face {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        4 => 0x10,
        5 => 0x20,
        _ => 0,
    }
}

/// The background colour words saved (`011a9bd0`, then the renderer's slot +0xb4 fills them);
/// answers the saved words.
fn save_background(e: &mut Engine) -> [u32; 4] {
    let saved = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(BACKGROUND_011A9BD0 + offset));
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.with_stack(0x10, |e, local| {
        for (index, word) in saved.iter().enumerate() {
            e.mem.set_u32(local.addr() + 4 * index as u32, *word);
        }
        e.vcall(renderer, 0xb4, &args![local]);
        [0u32, 4, 8, 0xc].map(|offset| e.mem.u32(local.addr() + offset))
    })
}

/// The grey background (`01017a3c` three times, 1.0) set through the renderer's slot +0xac.
fn set_grey(e: &mut Engine) {
    let value = x87_quiet_bits(e.global::<u32>(GREY_01017A3C));
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.with_stack(0x10, |e, local| {
        for (index, word) in [value, value, value, x87_store32(1.0)].iter().enumerate() {
            e.mem.set_u32(local.addr() + 4 * index as u32, *word);
        }
        e.vcall(renderer, 0xac, &args![local]);
    });
}

/// The saved background colour given back (the renderer's slot +0xac).
fn restore_background(e: &mut Engine, saved: [u32; 4]) {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.with_stack(0x10, |e, local| {
        for (index, word) in saved.iter().enumerate() {
            e.mem.set_u32(local.addr() + 4 * index as u32, *word);
        }
        e.vcall(renderer, 0xac, &args![local]);
    });
}

/// A face's set-up: the transformation, the rendered texture's (+0x130, its +0x30; null
/// without one) face (+0x48) and face texture (`NiPointer` +0x30 from +0x4c + 4 face), then the
/// targets begun (`00b6b8d0(7, StopOffscreen())`).
fn begin_face(e: &mut Engine, this: Ptr, face: u32) {
    let base = this.addr();
    bs_cube_map_camera_setup_transformation(e, this, face);
    let target = e.mem.u32(base + POINTER_130);
    let rendered = if target != 0 {
        e.mem.u32(target + 0x30)
    } else {
        0
    };
    e.mem.set_u32(rendered + 0x48, face);
    assign_from(e, rendered + 0x30, rendered + 4 * face + 0x4c);
    let target = e.mem.u32(base + POINTER_130);
    let group = e.call(STOP_OFFSCREEN_00B6B260, &args![target]).u32();
    e.call(BEGIN_TARGETS_00B6B8D0, &args![7u32, group]);
}

/// The face rendered in mode 7: with a list (+0x134), `00b71360(0, list, 1.0)`, the rendered
/// texture (+0x130) taken from the list's +0x10c and `00bd7620` with the list.
fn render_face_mode_7(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x134);
    if list == 0 {
        return;
    }
    e.call(FN_00B71360, &args![0u32, list, x87_store32(1.0)]);
    let list = e.mem.u32(base + 0x134);
    assign_from(e, base + POINTER_130, list + 0x10c);
    let list = e.mem.u32(base + 0x134);
    fn_00bd7620(e, this, list);
}

/// The culling process on the stack (0xc8 bytes) made for a face: `00a69400` with 0, the vtable
/// `0101e2ec`, +0x90 and +0xbc to +0xc4 cleared, the camera at +0xc and the frustum
/// (`00a694a0`).
fn make_culler(e: &mut Engine, culler: u32, camera: u32) {
    e.call(CULLER_CONSTRUCT_00A69400, &args![culler, 0u32]);
    e.mem.set_u32(culler, CULLING_VTABLE_0101E2EC);
    for offset in [0x90u32, 0xbc, 0xc0, 0xc4] {
        e.mem.set_u32(culler + offset, 0);
    }
    e.mem.set_u32(culler + 0xc, camera);
    e.call(
        CULLER_SET_FRUSTUM_00A694A0,
        &args![culler, camera + VIEW_FRUSTUM],
    );
}

// Translated from 00bd77c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd77c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the culling process) and the security-cookie bookkeeping are not translated.
/// Unnamed in the engine map (unit `bscubemapcamera.cpp`), `thiscall`, no stack words: renders
/// the six faces. The background colour is saved (renderer slot +0xb4) and, in mode 7
/// (`011f91b8`), set to grey with the face mask from the list's (+0x134) u16 +0x110. For each
/// face: its set-up (transformation, rendered texture, targets begun); unless its bit is in
/// the mask: the camera's update (zero record), then in mode 7 the list path, else with a
/// scene (+0x138) the accumulator (+0x13c, its count raised when 1) gets slot +0x8c and byte
/// +0x84, a culling process (`0101e2ec`) holds it (`NiPointer` +0xc4) while the scene is
/// accumulated (`AccumulateScene(this, scene, culler)`, `00b6c0d0(this, accumulator, 0)`) and
/// is destroyed; then the targets end (`00b6b790`). Last the colour is restored.
pub fn fn_00bd77c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let saved = save_background(e);
    let mode_7 = e.global::<u32>(MODE_011F91B8) == 7;
    let mut mask = 0;
    if mode_7 {
        mask = u32::from(e.mem.u16(e.mem.u32(base + 0x134) + 0x110));
        set_grey(e);
    }
    for face in 0..6u32 {
        begin_face(e, this, face);
        if face_bit(face) & mask & 0xffff == 0 {
            update_with_zero_record(e, base);
            if e.global::<u32>(MODE_011F91B8) == 7 {
                render_face_mode_7(e, this);
            } else if e.mem.u32(base + POINTER_138) != 0 {
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                if e.mem.u32(accumulator + 4) == 1 {
                    e.call(INTERLOCKED_INCREMENT, &args![accumulator + 4]);
                }
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                e.vcall(accumulator, 0x8c, &args![this]);
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                e.mem.set_u8(accumulator + 0x84, 1);
                e.with_stack(0xc8, |e, culler| {
                    let culler = culler.addr();
                    make_culler(e, culler, base);
                    assign_from(e, culler + 0xc4, base + ACCUMULATOR);
                    let scene = e.mem.u32(base + POINTER_138);
                    e.call(ACCUMULATE_SCENE_00B6BEE0, &args![this, scene, culler]);
                    let accumulator = e.mem.u32(base + ACCUMULATOR);
                    e.call(FN_00B6C0D0, &args![this, accumulator, 0u32]);
                    if e.mem.u32(culler + 0xc4) != 0 {
                        release_member(e, culler + 0xc4);
                        e.mem.set_u32(culler + 0xc4, 0);
                    }
                    e.call(CULLING_DESTRUCT_004A0F60, &args![culler]);
                });
            }
        }
        e.call(END_TARGETS_00B6B790, &args![]);
    }
    restore_background(e, saved);
}

/// What `00bd7bb0` adds: the render-state setters `00b97fa0` (two words) and `00b97ff0` (three
/// words) (unit `bsrenderstate_xenon.cpp`, `cdecl`), the culler's accumulator setter `004a0fd0`
/// (unnamed; `thiscall`, one word), `ImageSpaceShaderParam`'s constructor (`00b8a9e0`),
/// `InitConstantMap` (`00b8ab10`, two words), its vertex constant setter `00b8a760` (unnamed;
/// `thiscall`: index, four `float`s) and destructor `00b8a8e0` (unnamed), the image-space
/// manager's render `00b975f0` (unnamed; `thiscall` on `011f91ac`, six words), the words
/// `011f9520`, the counter `011ff9f0`, the CRT `sin` (`00eca0a0`) and `cos` (`00ec9f70`) and
/// the face angles (`double`s 3 pi / 2 at `01030f30`, pi / 2 at `01030f38`, pi at
/// `0101ff40`).
const RENDER_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const CULLER_SET_ACCUMULATOR_004A0FD0: u32 = 0x004a_0fd0;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_VERTEX_CONSTANT_00B8A760: u32 = 0x00b8_a760;
const PARAM_DESTRUCT_00B8A8E0: u32 = 0x00b8_a8e0;
const IMAGE_SPACE_RENDER_00B975F0: u32 = 0x00b9_75f0;
const IMAGE_SPACE_MANAGER_011F91AC: u32 = 0x011f_91ac;
const SOURCE_011F9520: u32 = 0x011f_9520;
const COUNTER_011FF9F0: u32 = 0x011f_f9f0;
const SIN_00ECA0A0: u32 = 0x00ec_a0a0;
const COS_00EC9F70: u32 = 0x00ec_9f70;
const ANGLE_01030F30: u32 = 0x0103_0f30;
const ANGLE_01030F38: u32 = 0x0103_0f38;
const ANGLE_0101FF40: u32 = 0x0101_ff40;

// Translated from 00bd7bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd7bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the culling process and the shader parameter) and the /GS stack cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (unit `bscubemapcamera.cpp`), `thiscall`, one word (a face, -1 for
/// all): renders the faces as `00bd77c0` does (mode 7 from the accumulator's +0x19c for the
/// grey colour and mask, from `011f91b8` for the list path; the culling process holds the
/// accumulator through `004a0fd0`, whose count is raised when 1 and dropped again after), then
/// per face: render states `00b97fa0(1, 1)` and `00b97ff0(6, 7, 0)`, a shader parameter on the
/// stack (`InitConstantMap(2, 0)`) whose vertex constants 0 and 1 are `(cos a, sin a, 0, 0)` and
/// `(-sin a, cos a, 0, 0)` for the face's angle (face 0: 3 pi / 2, 1: pi / 2, 2: 0, 3: pi; face
/// 4: no constants; the compiler's jump table as `match` arms) and the image-space render
/// `00b975f0(0x2f, renderer, [011f9520], rendered texture, parameter, 0)` (not for face 5),
/// `011ff9f0` counted down, `00b97fa0(0, 0)`, the targets ended and the parameter destroyed.
pub fn fn_00bd7bb0(e: &mut Engine, this: Ptr, selected: u32) {
    let base = this.addr();
    let saved = save_background(e);
    let accumulator = e.mem.u32(base + ACCUMULATOR);
    let mut mask = 0;
    if e.mem.u32(accumulator + 0x19c) == 7 {
        mask = u32::from(e.mem.u16(e.mem.u32(base + 0x134) + 0x110));
        set_grey(e);
    }
    for face in 0..6u32 {
        if selected != u32::MAX && face != selected {
            continue;
        }
        begin_face(e, this, face);
        if face_bit(face) & mask & 0xffff == 0 {
            update_with_zero_record(e, base);
            if e.global::<u32>(MODE_011F91B8) == 7 {
                render_face_mode_7(e, this);
            } else if e.mem.u32(base + POINTER_138) != 0 {
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                let raised = e.mem.u32(accumulator + 4) == 1;
                if raised {
                    e.call(INTERLOCKED_INCREMENT, &args![accumulator + 4]);
                }
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                e.vcall(accumulator, 0x8c, &args![this]);
                let accumulator = e.mem.u32(base + ACCUMULATOR);
                e.mem.set_u8(accumulator + 0x84, 1);
                e.with_stack(0xc8, |e, culler| {
                    let culler = culler.addr();
                    make_culler(e, culler, base);
                    let accumulator = e.mem.u32(base + ACCUMULATOR);
                    e.call(CULLER_SET_ACCUMULATOR_004A0FD0, &args![culler, accumulator]);
                    let scene = e.mem.u32(base + POINTER_138);
                    e.call(ACCUMULATE_SCENE_00B6BEE0, &args![this, scene, culler]);
                    let accumulator = e.mem.u32(base + ACCUMULATOR);
                    e.call(FN_00B6C0D0, &args![this, accumulator, 0u32]);
                    if e.mem.u32(culler + 0xc4) != 0 {
                        release_member(e, culler + 0xc4);
                        e.mem.set_u32(culler + 0xc4, 0);
                    }
                    if raised {
                        let accumulator = e.mem.u32(base + ACCUMULATOR);
                        release(e, accumulator);
                    }
                    e.call(CULLING_DESTRUCT_004A0F60, &args![culler]);
                });
            }
        }
        e.call(RENDER_STATE_00B97FA0, &args![1u32, 1u32]);
        e.call(RENDER_STATE_00B97FF0, &args![6u32, 7u32, 0u32]);
        let word = e.global::<u32>(SOURCE_011F9520);
        e.with_stack(0x90, |e, param| {
            let param = param.addr();
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![param]);
            let manager = e.global::<u32>(IMAGE_SPACE_MANAGER_011F91AC);
            let renderer = e.global::<u32>(RENDERER_011F9508);
            e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 2u32, 0u32]);
            let angle = match face {
                0 => Some(x87_load64(e.global::<u64>(ANGLE_01030F30))),
                1 => Some(x87_load64(e.global::<u64>(ANGLE_01030F38))),
                2 => Some(0.0),
                3 => Some(x87_load64(e.global::<u64>(ANGLE_0101FF40))),
                _ => None,
            };
            if let Some(angle) = angle {
                let sine = e.call(SIN_00ECA0A0, &args![angle]).f64();
                let sine = x87_quiet_bits(x87_store32(sine));
                let cosine = e.call(COS_00EC9F70, &args![angle]).f64();
                let cosine = x87_quiet_bits(x87_store32(cosine));
                e.call(
                    PARAM_SET_VERTEX_CONSTANT_00B8A760,
                    &args![param, 0u32, cosine, sine, 0u32, 0u32],
                );
                let minus_sine = x87_store32(-x87_load32(sine));
                e.call(
                    PARAM_SET_VERTEX_CONSTANT_00B8A760,
                    &args![param, 1u32, minus_sine, cosine, 0u32, 0u32],
                );
            }
            if face <= 4 {
                let target = e.mem.u32(base + POINTER_130);
                e.call(
                    IMAGE_SPACE_RENDER_00B975F0,
                    &args![manager, 0x2fu32, renderer, word, target, param, 0u32],
                );
            }
            let count: u32 = e.global(COUNTER_011FF9F0);
            e.set_global(COUNTER_011FF9F0, count.wrapping_sub(1));
            e.call(RENDER_STATE_00B97FA0, &args![0u32, 0u32]);
            e.call(END_TARGETS_00B6B790, &args![]);
            e.call(PARAM_DESTRUCT_00B8A8E0, &args![param]);
        });
    }
    restore_background(e, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bd_7200,
            bs_cube_map_camera_setup_transformation(Ptr, u32)
        ),
        entry!(0x00bd_7610, fn_00bd7610(Ptr) -> u32),
        entry!(0x00bd_7620, fn_00bd7620(Ptr, u32)),
        entry!(0x00bd_77c0, fn_00bd77c0(Ptr)),
        entry!(0x00bd_7bb0, fn_00bd7bb0(Ptr, u32)),
        entry!(0x00bd_74a0, fn_00bd74a0(Ptr)),
        entry!(
            0x00bd_8290,
            bs_cube_map_camera_bs_cube_map_camera(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        for quiet in [
            CAMERA_CONSTRUCTOR,
            CAMERA_DESTRUCTOR,
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            FRUSTUM_CONSTRUCTOR,
        ] {
            e.register(quiet, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_9000, &[0, 0x0040_9004]);
        e.register(0x0040_9004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_sets_the_frustum_and_the_accumulator() {
        let mut e = rig();
        e.map(0x0103_b000, 0x1000);
        e.map(0x0101_8000, 0x1000);
        e.set_global(HALF_FIELD_OF_VIEW, std::f64::consts::FRAC_PI_4);
        e.set_global(FAR_DISTANCE, 2048.0f32);
        e.register(TAN, |_, a| Ret {
            st0: f64::take(a, &mut 0).tan(),
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ACCUMULATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0090_9000);
            a[0].into_ret()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x140));
        assert_eq!(
            e.call(0x00bd_8290, &args![this, 0x7777u32]).u32(),
            this.addr()
        );
        let base = this.addr();
        assert_eq!(e.mem.u32(base), VTABLE);
        assert_eq!(e.mem.u32(base + OWNER), 0x7777);
        let tangent = std::f64::consts::FRAC_PI_4.tan() as f32;
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM), -tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 4), tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0xc), -tangent);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0x10), 1.0);
        assert_eq!(e.mem.f32(base + VIEW_FRUSTUM + 0x14), 2048.0);
        let accumulator = e.mem.u32(base + ACCUMULATOR);
        assert_ne!(accumulator, 0);
        assert_eq!(e.mem.u32(accumulator + 4), 1);
        assert_eq!(
            calls_to(&e, ACCUMULATOR_CONSTRUCTOR),
            vec![vec![accumulator, 0x63, 1, 0x2f7]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![
                base + FACES,
                4,
                6,
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT
            ]]
        );
    }

    #[test]
    fn destructor_releases_faces_and_pointers() {
        let mut e = rig();
        let this: Ptr = Ptr::new(e.mem.alloc(0x140));
        let base = this.addr();
        let face = e.mem.alloc(8);
        e.mem.set_u32(face, 0x0090_9000);
        e.mem.set_u32(face + 4, 1);
        e.mem.set_u32(base + FACES + 8, face);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x0090_9000);
        e.mem.set_u32(other + 4, 2);
        e.mem.set_u32(base + POINTER_138, other);
        e.call(0x00bd_74a0, &args![this]);
        assert_eq!(e.mem.u32(base + FACES + 8), 0);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![face]]);
        // +0x138 only gets its member destructor: not cleared.
        assert_eq!(e.mem.u32(base + POINTER_138), other);
        assert_eq!(e.mem.u32(other + 4), 1);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![base + FACES, 4, 6, POINTER_DESTRUCT]]
        );
        assert_eq!(calls_to(&e, CAMERA_DESTRUCTOR), vec![vec![base]]);
    }
}

#[cfg(test)]
mod n070_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SLOTS: u32 = 0x0f30_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x1a0);
        for offset in (0..0x1a0u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Records the point and up vector `LookAtWorldPoint` gets at `0x011a_9f00`.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011a_9000u32,
            0x011f_9000,
            0x011f_f000,
            0x0101_7000,
            0x0101_f000,
            0x0103_0000,
        ] {
            e.map(page, 0x1000);
        }
        for offset in (0..0x1a0u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.register(LOOK_AT_WORLD_POINT_00A701B0, |e, a| {
            for i in 0..3 {
                let p = e.mem.u32(a[1] + 4 * i);
                let u = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(0x011a_9f00 + 4 * i, p);
                e.mem.set_u32(0x011a_9f10 + 4 * i, u);
            }
            Ret::default()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        for (i, v) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.set_global(AXIS_011A946C + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn looked(e: &Engine) -> (Vec<f32>, Vec<f32>) {
        let p = (0..3)
            .map(|i| e.global::<f32>(0x011a_9f00 + 4 * i))
            .collect();
        let u = (0..3)
            .map(|i| e.global::<f32>(0x011a_9f10 + 4 * i))
            .collect();
        (p, u)
    }

    #[test]
    fn setup_transformation_looks_along_each_axis() {
        let mut e = rig();
        let this = e.mem.alloc(0x140);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(this + 0x8c + 4 * i as u32, *v);
        }
        let cases = [
            (0u32, [9.0f32, 20.0, 30.0], [0.0f32, -1.0, 0.0]),
            (1, [11.0, 20.0, 30.0], [0.0, -1.0, 0.0]),
            (2, [10.0, 19.0, 30.0], [0.0, 0.0, 1.0]),
            (3, [10.0, 21.0, 30.0], [0.0, 0.0, -1.0]),
            (4, [10.0, 20.0, 29.0], [0.0, -1.0, 0.0]),
            (5, [10.0, 20.0, 31.0], [0.0, -1.0, 0.0]),
        ];
        for (face, point, up) in cases {
            e.call(0x00bd_7200, &args![this, face]);
            assert_eq!(looked(&e), (point.to_vec(), up.to_vec()), "face {face}");
        }
        assert_eq!(calls(&e, LOOK_AT_WORLD_POINT_00A701B0)[2][2], AXIS_011A9484);
        e.call(0x00bd_7200, &args![this, 6u32]);
        assert_eq!(calls(&e, LOOK_AT_WORLD_POINT_00A701B0).len(), 6);
        assert_eq!(e.call(0x00bd_7610, &args![this]).u32(), 0x0120_23e8);
    }

    /// A camera with a renderer (mode 1, byte set), an accumulator and the culler helpers.
    fn camera_rig(e: &mut Engine) -> (u32, u32, u32) {
        let renderer = object(e, 0x210);
        e.mem.set_u32(renderer + 0x200, 1);
        e.mem.set_u8(renderer + 0x208, 1);
        e.set_global(RENDERER_011F9508, renderer);
        let accumulator = object(e, 0x1a0);
        let this = object(e, 0x140);
        e.mem.set_u32(this + ACCUMULATOR, accumulator);
        for target in [
            CULLER_CONSTRUCT_00A69400,
            CULLER_SET_FRUSTUM_00A694A0,
            CULLER_DESTRUCT_00A693E0,
            UPDATE_00A59C60,
            ACCUMULATE_00B9D020,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        (this, renderer, accumulator)
    }

    #[test]
    fn fn_00bd7620_accumulates_into_the_object() {
        let mut e = rig();
        let (this, renderer, accumulator) = camera_rig(&mut e);
        e.mem.set_f32(this + 0x68, 7.0);
        e.register(SLOTS + 0x18c, |e, a| {
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(0x011a_9f20, v);
            Ret::default()
        });
        e.call(0x00bd_7620, &args![this, 0x5150u32]);
        let rows = &calls(&e, SLOTS + 0x18c)[0];
        assert_eq!(rows[0], renderer);
        assert_eq!(rows[1], this + 0x8c);
        assert_eq!(rows[5..7], [this + 0xdc, this + 0x100]);
        assert_eq!(e.global::<f32>(0x011a_9f20), 7.0);
        assert_eq!(calls(&e, SLOTS + 0x8c), vec![vec![accumulator, this]]);
        assert_eq!(e.mem.u8(accumulator + 0x84), 1);
        let culler = calls(&e, CULLER_CONSTRUCT_00A69400)[0][0];
        assert_eq!(calls(&e, ACCUMULATE_00B9D020), vec![vec![0x5150, culler]]);
        assert_eq!(calls(&e, SLOTS + 0x90), vec![vec![accumulator]]);
        assert_eq!(e.mem.u32(accumulator + 4), 1);
        assert_eq!(calls(&e, CULLER_DESTRUCT_00A693E0), vec![vec![culler]]);
        // Without the object: nothing.
        e.call(0x00bd_7620, &args![this, 0u32]);
        assert_eq!(calls(&e, SLOTS + 0x8c).len(), 1);
    }

    /// The face renderers' rig: the rendered texture with six face textures, the scene, the
    /// helpers doubled.
    fn faces_rig(e: &mut Engine) -> (u32, u32) {
        let (this, _, accumulator) = camera_rig(e);
        let rendered = e.mem.alloc(0x64);
        for face in 0..6u32 {
            let texture = object(e, 8);
            e.mem.set_u32(rendered + 0x4c + 4 * face, texture);
        }
        let target = e.mem.alloc(0x34);
        e.mem.set_u32(target + 0x30, rendered);
        e.mem.set_u32(this + POINTER_130, target);
        e.mem.set_u32(this + POINTER_138, 0x1380);
        for target in [
            STOP_OFFSCREEN_00B6B260,
            BEGIN_TARGETS_00B6B8D0,
            END_TARGETS_00B6B790,
            ACCUMULATE_SCENE_00B6BEE0,
            FN_00B6C0D0,
            CULLING_DESTRUCT_004A0F60,
            CULLER_SET_ACCUMULATOR_004A0FD0,
            RENDER_STATE_00B97FA0,
            RENDER_STATE_00B97FF0,
            PARAM_CONSTRUCT_00B8A9E0,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_VERTEX_CONSTANT_00B8A760,
            PARAM_DESTRUCT_00B8A8E0,
            IMAGE_SPACE_RENDER_00B975F0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SIN_00ECA0A0, |_, a| {
            f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
                .sin()
                .into_ret()
        });
        e.register(COS_00EC9F70, |_, a| {
            f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
                .cos()
                .into_ret()
        });
        (this, accumulator)
    }

    #[test]
    fn fn_00bd77c0_renders_six_faces() {
        let mut e = rig();
        let (this, accumulator) = faces_rig(&mut e);
        e.mem.set_u32(accumulator + 4, 1);
        e.call(0x00bd_77c0, &args![this]);
        assert_eq!(calls(&e, BEGIN_TARGETS_00B6B8D0).len(), 6);
        assert_eq!(calls(&e, END_TARGETS_00B6B790).len(), 6);
        let scenes = calls(&e, ACCUMULATE_SCENE_00B6BEE0);
        assert_eq!(scenes.len(), 6);
        assert!(scenes.iter().all(|a| a[0] == this && a[1] == 0x1380));
        let rendered = e.mem.u32(e.mem.u32(this + POINTER_130) + 0x30);
        assert_eq!(e.mem.u32(rendered + 0x48), 5);
        assert_eq!(e.mem.u32(rendered + 0x30), e.mem.u32(rendered + 0x4c + 20));
        // Raised once on the first face (count 1 -> 2), then held twice per face and dropped.
        assert_eq!(e.mem.u32(accumulator + 4), 2);
        // The background colour saved and restored.
        assert_eq!(calls(&e, SLOTS + 0xb4).len(), 1);
        assert_eq!(calls(&e, SLOTS + 0xac).len(), 1);
    }

    #[test]
    fn fn_00bd77c0_mode_7_skips_masked_faces() {
        let mut e = rig();
        let (this, _) = faces_rig(&mut e);
        e.set_global(MODE_011F91B8, 7u32);
        let list = e.mem.alloc(0x114);
        e.mem.set_u16(list + 0x110, 0b11_1110);
        let target = e.mem.u32(this + POINTER_130);
        e.mem.set_u32(list + 0x10c, target);
        e.mem.set_u32(this + 0x134, list);
        e.register(FN_00B71360, |_, _| Ret::default());
        e.call(0x00bd_77c0, &args![this]);
        // Only face 0 renders, through the list.
        assert_eq!(calls(&e, FN_00B71360).len(), 1);
        assert_eq!(calls(&e, SLOTS + 0xac).len(), 2);
        assert!(calls(&e, ACCUMULATE_SCENE_00B6BEE0).is_empty());
    }

    #[test]
    fn fn_00bd7bb0_renders_one_face_with_constants() {
        let mut e = rig();
        let (this, accumulator) = faces_rig(&mut e);
        e.mem.set_u32(accumulator + 4, 1);
        e.set_global(ANGLE_0101FF40, f64::from(std::f32::consts::PI));
        e.call(0x00bd_7bb0, &args![this, 3u32]);
        assert_eq!(calls(&e, BEGIN_TARGETS_00B6B8D0).len(), 1);
        assert_eq!(calls(&e, CULLER_SET_ACCUMULATOR_004A0FD0).len(), 1);
        // Raised and dropped again.
        assert_eq!(e.mem.u32(accumulator + 4), 1);
        let constants = calls(&e, PARAM_SET_VERTEX_CONSTANT_00B8A760);
        let pi = f64::from(std::f32::consts::PI);
        let (s, c) = (pi.sin() as f32, pi.cos() as f32);
        assert_eq!(constants[0][1..4], [0, c.to_bits(), s.to_bits()]);
        assert_eq!(constants[1][1..4], [1, (-s).to_bits(), c.to_bits()]);
        let render = &calls(&e, IMAGE_SPACE_RENDER_00B975F0)[0];
        assert_eq!(render[1], 0x2f);
        assert_eq!(e.global::<u32>(COUNTER_011FF9F0), u32::MAX);
        // Face 5 alone: no constants and no image-space render; all faces: six.
        e.call(0x00bd_7bb0, &args![this, 5u32]);
        assert_eq!(calls(&e, IMAGE_SPACE_RENDER_00B975F0).len(), 1);
        e.call(0x00bd_7bb0, &args![this, u32::MAX]);
        assert_eq!(calls(&e, IMAGE_SPACE_RENDER_00B975F0).len(), 6);
        assert_eq!(calls(&e, PARAM_SET_VERTEX_CONSTANT_00B8A760).len(), 2 + 8);
    }
}

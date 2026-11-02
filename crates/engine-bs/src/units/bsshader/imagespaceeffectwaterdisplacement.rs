//! `BSShader/imagespaceeffectwaterdisplacement.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ImageSpaceTexture` calls (`thiscall` on a 0x10-byte texture record):
/// `BorrowRenderedTexture` (`00ba3840`, one word), `GetRenderedTexture` (`00ba3770`),
/// `SetTexture` (`00ba37a0`, one word), `operator=` (`00ba3940`, the address of another
/// record), `GetTexture` (`00ba3780`), `ReturnRenderedTexture` (`00ba39a0`) and the destructor
/// (`00ba3a00`).
const TEXTURE_BORROW: u32 = 0x00ba_3840;
const TEXTURE_GET_RENDERED: u32 = 0x00ba_3770;
const TEXTURE_SET: u32 = 0x00ba_37a0;
const TEXTURE_ASSIGN: u32 = 0x00ba_3940;
const TEXTURE_GET: u32 = 0x00ba_3780;
const TEXTURE_RETURN_RENDERED: u32 = 0x00ba_39a0;
const TEXTURE_DESTRUCTOR: u32 = 0x00ba_3a00;
/// `ImageSpaceEffect::SetTexture` (`00ba3cb0`, `thiscall`: index, texture, a third word), the
/// shader parameter setters (`00b8a790` `SetPixelConstant` and `00b8a760`, `thiscall`: a
/// register and four `float`s), the allocation (`00aa13e0`), the render target of a texture
/// (`00b6b260`), `BSRenderedTexture::GetTexture` (`004bc320`, one word), the texture copy
/// `00b6b380` (`thiscall`, three words), the 2D vector call `004e7560` (`thiscall`, ST0) and the
/// image-space manager's effect render (`00b97550`, six words).
const EFFECT_SET_TEXTURE: u32 = 0x00ba_3cb0;
const SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
const SET_CONSTANT_00B8A760: u32 = 0x00b8_a760;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const RENDER_TARGET_OF: u32 = 0x00b6_b260;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
const TEXTURE_COPY_00B6B380: u32 = 0x00b6_b380;
const VECTOR2_004E7560: u32 = 0x004e_7560;
const IMAGE_SPACE_RENDER_EFFECT: u32 = 0x00b9_7550;
/// The screen geometry calls (as `ImageSpaceManager::CreatePartialScreenGeometry` makes it):
/// `NiScreenElementsData` (`00a881e0`), `NiScreenElements` (`00a87810`), the `NiPointer`
/// assignment `0066b0d0` (`thiscall` on the slot, the object), `Insert` (`00a87ae0`),
/// `SetRectangle` (`00a88020`), `UpdateBound` (`00a87900`), `SetTextures` (`00a88130`),
/// `UpdateProperties` (`00a5a040`), the update `00a59c60`, the name maker `00a5b690`, the
/// name setter `00a5b950`, the empty name word (`0109b220`) and `InterlockedIncrement` /
/// `InterlockedDecrement` (import slots).
const SCREEN_ELEMENTS_DATA_CONSTRUCT: u32 = 0x00a8_81e0;
const SCREEN_ELEMENTS_CONSTRUCT: u32 = 0x00a8_7810;
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;
const SCREEN_ELEMENTS_DATA_INSERT: u32 = 0x00a8_7ae0;
const SCREEN_ELEMENTS_DATA_SET_RECTANGLE: u32 = 0x00a8_8020;
const SCREEN_ELEMENTS_DATA_UPDATE_BOUND: u32 = 0x00a8_7900;
const SCREEN_ELEMENTS_DATA_SET_TEXTURES: u32 = 0x00a8_8130;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const MAKE_FIXED_STRING: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const NULL_NAME_0109B220: u32 = 0x0109_b220;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The globals the render reads and writes: the time step (`010313d4`), the time added each
/// frame (`011f9410`), the two run flags (`011fffec` the first-frame copy, `011fffed` the
/// displacement), the displacement count written (`011ffffc`), the flag byte set while
/// passes render (`01200000`), the shader constants (`01200004` to `01200048`, `011ffff4`),
/// the two point lists (`011ada00` and `011ada10`: allocator objects with the array at `+4`,
/// the count at `+8` and a word at `+0xc`), the rectangle `float`s (`010542a4`, `01013ea4`),
/// the geometry name (`"Partial Screen Trishape"`, `010b7ee8`), the renderer (`011f9508`) and
/// the image-space manager (`011f91ac`).
const TIME_STEP_010313D4: u32 = 0x0103_13d4;
const TIME_ADD_011F9410: u32 = 0x011f_9410;
const FIRST_FRAME_011FFFEC: u32 = 0x011f_ffec;
const DISPLACE_011FFFED: u32 = 0x011f_ffed;
const DISPLACEMENTS_011FFFFC: u32 = 0x011f_fffc;
const RENDERING_01200000: u32 = 0x0120_0000;
const CONSTANT_011FFFF4: u32 = 0x011f_fff4;
const POINTS_011ADA00: u32 = 0x011a_da00;
const OLD_POINTS_011ADA10: u32 = 0x011a_da10;
const RECT_HEIGHT_010542A4: u32 = 0x0105_42a4;
const RECT_Y_01013EA4: u32 = 0x0101_3ea4;
const GEOMETRY_NAME_010B7EE8: u32 = 0x010b_7ee8;
const RENDERER_011F9508: u32 = 0x011f_9508;
const IMAGE_SPACE_MANAGER_011F91AC: u32 = 0x011f_91ac;

/// An `ImageSpaceTexture` record on the stack: three zero bytes and three zero words.
fn texture_record(e: &mut Engine, at: u32) {
    for k in 0..3u32 {
        e.mem.set_u8(at + k, 0);
    }
    for k in 1..4u32 {
        e.mem.set_u32(at + 4 * k, 0);
    }
}

/// The effect of `pEffectList` slot `index` (`+0xc`'s buffer) as the code takes it: the
/// pointer when it is neither null nor 0x6c (the shader part 0x6c below it null), else null.
fn effect_at(e: &Engine, this: u32, index: u32) -> u32 {
    let effect = e.mem.u32(e.mem.u32(this + 0xc).wrapping_add(4 * index));
    if effect != 0 && effect.wrapping_sub(0x6c) != 0 {
        effect
    } else {
        0
    }
}

/// The effect parameter of `pEffectParamList` slot `index` (`+0x1c`'s buffer).
fn param_at(e: &Engine, this: u32, index: u32) -> u32 {
    e.mem.u32(e.mem.u32(this + 0x1c).wrapping_add(4 * index))
}

/// A `SetPixelConstant` call with four `float` words.
fn constant(e: &mut Engine, param: u32, register: u32, values: [u32; 4]) {
    e.call(
        SET_PIXEL_CONSTANT,
        &args![param, register, values[0], values[1], values[2], values[3]],
    );
}

/// The five pixel constants the displacement passes set: register 0 (the time `+0x5c`), 1
/// (`011ffff4`), 2 (`01200024`, `01200028`), 4 (`01200004` to `01200010`) and 5 (`01200014`
/// to `01200020`), each `float` through `FLD`/`FSTP`, zeros as `FLDZ`.
fn displacement_constants(e: &mut Engine, this: u32, param: u32) {
    let q = |e: &Engine, at: u32| x87_quiet_bits(e.mem.u32(at));
    let zero = x87_store32(0.0);
    let time = q(e, this + 0x5c);
    constant(e, param, 0, [time, zero, zero, zero]);
    let v = q(e, CONSTANT_011FFFF4);
    constant(e, param, 1, [v, zero, zero, zero]);
    let (a, b) = (q(e, 0x0120_0024), q(e, 0x0120_0028));
    constant(e, param, 2, [a, b, zero, zero]);
    let v = [
        q(e, 0x0120_0004),
        q(e, 0x0120_0008),
        q(e, 0x0120_000c),
        q(e, 0x0120_0010),
    ];
    constant(e, param, 4, v);
    let v = [
        q(e, 0x0120_0014),
        q(e, 0x0120_0018),
        q(e, 0x0120_001c),
        q(e, 0x0120_0020),
    ];
    constant(e, param, 5, v);
}

/// Copies the `ImageSpaceTexture` records `from` into `to` (`operator=`).
fn assign(e: &mut Engine, to: u32, from: u32) {
    e.call(TEXTURE_ASSIGN, &args![to, from]);
}

/// The partial screen geometry `Render` makes into the `NiPointer` at `+0x58`: as
/// `CreatePartialScreenGeometry` builds it, with the rectangle (0, 0.0, the `float` at
/// `01013ea4`, 1.0, the one at `010542a4`) and the textures (0.0, 0.0, 1.0, 1.0), named
/// `"Partial Screen Trishape"`; the local data pointer released at the end.
fn make_geometry(e: &mut Engine, this: u32) {
    let slot = this + 0x58;
    let block = e.call(NI_ALLOCATE, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            SCREEN_ELEMENTS_DATA_CONSTRUCT,
            &args![block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    if data != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let elements = if block != 0 {
        e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![block, data]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN_0066B0D0, &args![slot, elements]);
    let elements_data = |e: &Engine| e.mem.u32(e.mem.u32(slot) + 0xb8);
    let target = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_INSERT,
        &args![target, 4u32, 0u32, 0u32],
    );
    let height = x87_quiet_bits(e.mem.u32(RECT_HEIGHT_010542A4));
    let target = elements_data(e);
    let y = x87_quiet_bits(e.mem.u32(RECT_Y_01013EA4));
    e.call(
        SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
        &args![target, 0u32, x87_store32(0.0), y, x87_store32(1.0), height],
    );
    let target = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_UPDATE_BOUND, &args![target]);
    let target = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_SET_TEXTURES,
        &args![
            target,
            0u32,
            0u32,
            x87_store32(0.0),
            x87_store32(0.0),
            x87_store32(1.0),
            x87_store32(1.0)
        ],
    );
    let elements = e.mem.u32(slot);
    e.call(UPDATE_PROPERTIES, &args![elements]);
    e.with_stack(0x10, |e, locals| {
        let update = locals.addr();
        let name_slot = update + 0xc;
        e.mem.set_u32(update, x87_store32(0.0));
        for k in 4..9u32 {
            e.mem.set_u8(update + k, 0);
        }
        let elements = e.mem.u32(slot);
        e.call(UPDATE_00A59C60, &args![elements, update]);
        let name = e
            .call(MAKE_FIXED_STRING, &args![GEOMETRY_NAME_010B7EE8])
            .u32();
        e.mem.set_u32(name_slot, name);
        let elements = e.mem.u32(slot);
        e.call(SET_NAME, &args![elements, name_slot]);
        let name = e.mem.u32(name_slot);
        if name != e.mem.u32(NULL_NAME_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
    if data != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![data + 4]).u32();
        if left == 0 {
            e.vcall(data, 4, &args![]);
        }
    }
}

/// The point lists' release: when the array (`+4`) exists, the allocator object's slot +8
/// frees it and the array, count and `+0xc` words are cleared.
fn release_points(e: &mut Engine, list: u32) {
    let array = e.mem.u32(list + 4);
    if array == 0 {
        return;
    }
    e.vcall(list, 8, &args![array]);
    e.mem.set_u32(list + 4, 0);
    e.mem.set_u32(list + 0xc, 0);
    e.mem.set_u32(list + 8, 0);
}

// Translated from 00ba0900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba0900 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `ImageSpaceEffectWaterDisplacement::Render` (engine map; the body matches): `thiscall`,
/// `RET 0x10` (a geometry and a renderer word handed to the effects, two unread words).
/// Advances the time (`+0x5c` by the `float` at `011f9410`, accumulated into `+0x68`; the step
/// `+0x6c` from `010313d4`), borrows a rendered texture (`00ba3840` with 8) into one local
/// `ImageSpaceTexture` and makes another from a copy of texture 1's render target (`00b6b380`,
/// 0x40 bytes). On the first frame (`011fffec`) effect 0 copies texture 1 into the chain; with
/// the displacement flag (`011fffed`) effect 1 renders with its pixel constants, effect 2 gets
/// the chain texture and, when the point list at `011ada00` has points, each point's
/// displacement (against the list at `011ada10`) is written to the constants at `0120002c`
/// to `01200048` and rendered on the partial screen geometry (`+0x58`, made on first use), both
/// lists released afterwards. Then the time steps (`+0x60` toward `+0x5c` by `+0x6c`) each run
/// effect 4 when the displacement flag is set, swapping the chain textures; after an even
/// number of those runs effect 0x22 of the image-space manager copies the result. The borrowed
/// texture is returned and the four locals destroyed.
pub fn image_space_effect_water_displacement_render(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    renderer: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let base = this.addr();
    let step = x87_quiet_bits(e.mem.u32(TIME_STEP_010313D4));
    e.mem.set_u32(base + 0x6c, step);
    let time = x87_store32(x87_add(
        x87_m32(e.mem.u32(base + 0x5c)),
        x87_m32(e.mem.u32(TIME_ADD_011F9410)),
    ));
    e.mem.set_u32(base + 0x5c, x87_quiet_bits(time));
    let total = x87_store32(x87_add(x87_load32(time), x87_m32(e.mem.u32(base + 0x68))));
    e.mem.set_u32(base + 0x68, total);
    e.with_stack(0x40, |e, locals| {
        let ping = locals.addr();
        let borrowed = ping + 0x10;
        let pong = ping + 0x20;
        let spare = ping + 0x30;
        texture_record(e, ping);
        texture_record(e, borrowed);
        texture_record(e, pong);
        e.call(TEXTURE_BORROW, &args![borrowed, 8u32]);
        let block = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
        let copy = if block != 0 {
            let textures = e.mem.u32(base + 0x2c);
            let first = e.mem.u32(textures + 4);
            let rendered = e.call(TEXTURE_GET_RENDERED, &args![first]).u32();
            let target = e.call(RENDER_TARGET_OF, &args![rendered]).u32();
            let format = e.vcall(target, 0xb4, &args![]).u32();
            let mine = e.call(TEXTURE_GET_RENDERED, &args![borrowed]).u32();
            let texture = e
                .call(RENDERED_TEXTURE_GET_TEXTURE, &args![mine, 0u32])
                .u32();
            e.call(TEXTURE_COPY_00B6B380, &args![block, texture, 1u32, format])
                .u32()
        } else {
            0
        };
        e.call(TEXTURE_SET, &args![pong, copy]);
        texture_record(e, spare);
        if e.mem.u8(FIRST_FRAME_011FFFEC) != 0 {
            let effect = effect_at(e, base, 0);
            let param = param_at(e, base, 0);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, pong, 0u32]);
            let first = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 1u32, first, 1u32]);
            e.vcall(effect, 4, &args![geometry, renderer, param, 1u32]);
            let first = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
            assign(e, spare, first);
            assign(e, ping, pong);
            assign(e, pong, spare);
            e.mem.set_u8(FIRST_FRAME_011FFFEC, 0);
        }
        if e.mem.u8(DISPLACE_011FFFED) != 0 {
            let effect = effect_at(e, base, 1);
            let param = param_at(e, base, 1);
            displacement_constants(e, base, param);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, pong, 0u32]);
            let first = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 1u32, first, 0u32]);
            e.vcall(effect, 4, &args![geometry, renderer, param, 1u32]);
            if e.mem.u8(DISPLACE_011FFFED) != 0 {
                e.mem.set_u32(DISPLACEMENTS_011FFFFC, 0);
                let point_effect = effect_at(e, base, 2);
                let point_param = param_at(e, base, 2);
                e.call(EFFECT_SET_TEXTURE, &args![point_effect, 0u32, pong, 0u32]);
                if e.mem.u32(POINTS_011ADA00 + 8) != 0 {
                    if e.mem.u32(base + 0x58) == 0 {
                        make_geometry(e, base);
                    }
                    let mut index = 0u32;
                    while (index as i32) < e.mem.i32(POINTS_011ADA00 + 8) {
                        displace_point(e, base, index, point_effect, point_param, renderer);
                        let n = e.mem.u32(DISPLACEMENTS_011FFFFC);
                        e.mem.set_u32(DISPLACEMENTS_011FFFFC, n.wrapping_add(1));
                        index += 1;
                    }
                    release_points(e, POINTS_011ADA00);
                    release_points(e, OLD_POINTS_011ADA10);
                }
            }
        }
        e.mem.set_u8(RENDERING_01200000, 0);
        let first = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
        assign(e, spare, first);
        assign(e, ping, pong);
        assign(e, pong, spare);
        let mut runs = 0u32;
        let behind = |e: &Engine| {
            let lag = x87_sub(
                x87_m32(e.mem.u32(base + 0x5c)),
                x87_m32(e.mem.u32(base + 0x60)),
            );
            x87_m32(e.mem.u32(base + 0x6c)) < lag
        };
        while behind(e) {
            let caught = x87_store32(x87_add(
                x87_m32(e.mem.u32(base + 0x60)),
                x87_m32(e.mem.u32(base + 0x6c)),
            ));
            e.mem.set_u32(base + 0x60, caught);
            if e.mem.u8(DISPLACE_011FFFED) == 0 {
                continue;
            }
            runs = runs.wrapping_add(1);
            let effect = effect_at(e, base, 4);
            let param = param_at(e, base, 4);
            displacement_constants(e, base, param);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, pong, 0u32]);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 1u32, ping, 1u32]);
            e.vcall(effect, 4, &args![geometry, renderer, param, 1u32]);
            e.mem.set_u8(RENDERING_01200000, 1);
            assign(e, spare, ping);
            assign(e, ping, pong);
            assign(e, pong, spare);
        }
        if (runs as i32) % 2 == 0 {
            let renderer_word = e.mem.u32(RENDERER_011F9508);
            let manager = e.mem.u32(IMAGE_SPACE_MANAGER_011F91AC);
            let second = e.call(TEXTURE_GET_RENDERED, &args![pong]).u32();
            let first = e.call(TEXTURE_GET_RENDERED, &args![ping]).u32();
            e.call(
                IMAGE_SPACE_RENDER_EFFECT,
                &args![manager, 0x22u32, renderer_word, first, second, 0u32, 1u32],
            );
        }
        if e.call(TEXTURE_GET, &args![borrowed]).u32() != 0 {
            e.call(TEXTURE_RETURN_RENDERED, &args![borrowed]);
        }
        e.call(TEXTURE_DESTRUCTOR, &args![spare]);
        e.call(TEXTURE_DESTRUCTOR, &args![pong]);
        e.call(TEXTURE_DESTRUCTOR, &args![borrowed]);
        e.call(TEXTURE_DESTRUCTOR, &args![ping]);
    });
}

/// One point of the displacement list: the offset of point `index` (`011ada04`, 8-byte
/// points) from the matching old point (`011ada14`) into the constants (`0120002c` x,
/// `0120003c` y, `01200034` the old x), the vector (-y, x) through `004e7560` (its answer
/// dropped) into `01200030` and `01200040`, the old y into `01200044`; the two constant
/// registers set (`00b8a760` with 0 and 1) and the effect rendered on the partial screen
/// geometry.
fn displace_point(e: &mut Engine, this: u32, index: u32, effect: u32, param: u32, renderer: u32) {
    let points = e.mem.u32(POINTS_011ADA00 + 4);
    let old = e.mem.u32(OLD_POINTS_011ADA10 + 4);
    let at = |list: u32, k: u32| list.wrapping_add(index.wrapping_mul(8)).wrapping_add(k);
    let x = x87_store32(x87_sub(
        x87_m32(e.mem.u32(at(points, 0))),
        x87_m32(e.mem.u32(at(old, 0))),
    ));
    e.mem.set_u32(0x0120_002c, x);
    let y = x87_store32(x87_sub(
        x87_m32(e.mem.u32(at(points, 4))),
        x87_m32(e.mem.u32(at(old, 4))),
    ));
    e.mem.set_u32(0x0120_003c, y);
    let old_x = x87_quiet_bits(e.mem.u32(at(old, 0)));
    e.mem.set_u32(0x0120_0034, old_x);
    e.with_stack(8, |e, vector| {
        let v = vector.addr();
        let minus_y = x87_store32(-x87_load32(e.mem.u32(0x0120_003c)));
        e.mem.set_u32(v, minus_y);
        let x = x87_quiet_bits(e.mem.u32(0x0120_002c));
        e.mem.set_u32(v + 4, x);
        e.call(VECTOR2_004E7560, &args![v]);
        let first = x87_quiet_bits(e.mem.u32(v));
        e.mem.set_u32(0x0120_0030, first);
        let second = x87_quiet_bits(e.mem.u32(v + 4));
        e.mem.set_u32(0x0120_0040, second);
        let old_y = x87_quiet_bits(e.mem.u32(at(old, 4)));
        e.mem.set_u32(0x0120_0044, old_y);
        let q = |e: &Engine, a: u32| x87_quiet_bits(e.mem.u32(a));
        let (c2c, c34, c38) = (q(e, 0x0120_002c), q(e, 0x0120_0034), q(e, 0x0120_0038));
        e.call(
            SET_CONSTANT_00B8A760,
            &args![param, 0u32, c2c, first, c34, c38],
        );
        let (c3c, c40, c44, c48) = (
            q(e, 0x0120_003c),
            q(e, 0x0120_0040),
            q(e, 0x0120_0044),
            q(e, 0x0120_0048),
        );
        e.call(
            SET_CONSTANT_00B8A760,
            &args![param, 1u32, c3c, c40, c44, c48],
        );
    });
    let screen = e.mem.u32(this + 0x58);
    e.vcall(effect, 4, &args![screen, renderer, param, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ba_0900,
        image_space_effect_water_displacement_render(Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod n012_tests {
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

    /// An effect with five effect slots (each an object 0x6c into its shader, slot +4 its
    /// render) and five parameters, texture 1, and doubles for every outside call.
    fn rig() -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        for page in [
            0x0103_1000u32,
            0x011f_9000,
            0x011f_f000,
            0x0120_0000,
            0x011a_d000,
            0x0105_4000,
            0x0101_3000,
            0x0109_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(TIME_STEP_010313D4, 0.25);
        e.mem.set_f32(TIME_ADD_011F9410, 0.5);
        for addr in [
            TEXTURE_BORROW,
            TEXTURE_SET,
            TEXTURE_ASSIGN,
            TEXTURE_RETURN_RENDERED,
            TEXTURE_DESTRUCTOR,
            EFFECT_SET_TEXTURE,
            SET_PIXEL_CONSTANT,
            SET_CONSTANT_00B8A760,
            TEXTURE_COPY_00B6B380,
            IMAGE_SPACE_RENDER_EFFECT,
            RENDERED_TEXTURE_GET_TEXTURE,
            VECTOR2_004E7560,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TEXTURE_GET_RENDERED, |_, a| (a[0] | 1).into_ret());
        e.register(TEXTURE_GET, |_, _| 1u32.into_ret());
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.put_vtable(0x0f00_8000, &[0, 0x0f00_0004]);
        let this = e.mem.alloc(0x70);
        let effects = e.mem.alloc(0x14);
        let params = e.mem.alloc(0x14);
        let mut objects = vec![];
        for k in 0..5u32 {
            let effect = e.mem.alloc(0x80);
            e.mem.set_u32(effect, 0x0f00_8000);
            e.mem.set_u32(effects + 4 * k, effect);
            e.mem.set_u32(params + 4 * k, 0x9000 + k);
            objects.push(effect);
        }
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures + 4, 0x7100);
        e.mem.set_u32(this + 0xc, effects);
        e.mem.set_u32(this + 0x1c, params);
        e.mem.set_u32(this + 0x2c, textures);
        e.call_log = Some(vec![]);
        (e, this, objects)
    }

    #[test]
    fn render_without_flags_catches_up_the_time_and_copies() {
        let (mut e, this, _) = rig();
        e.mem.set_f32(this + 0x5c, 1.0);
        e.mem.set_f32(this + 0x68, 2.0);
        e.mem.set_f32(this + 0x60, 0.5);
        e.call(0x00ba_0900, &args![this, 0x60u32, 0x61u32, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0x6c), 0.25);
        assert_eq!(e.mem.f32(this + 0x5c), 1.5);
        assert_eq!(e.mem.f32(this + 0x68), 3.5);
        // 0.5 catches up to 1.5 in steps of 0.25 while the step is below the lag.
        assert_eq!(e.mem.f32(this + 0x60), 1.25);
        assert!(calls(&e, 0x0f00_0004).is_empty());
        let copied = calls(&e, IMAGE_SPACE_RENDER_EFFECT);
        assert_eq!(copied.len(), 1);
        assert_eq!(copied[0][1], 0x22);
        assert_eq!(calls(&e, TEXTURE_BORROW)[0][1], 8);
        assert_eq!(calls(&e, TEXTURE_DESTRUCTOR).len(), 4);
        assert_eq!(calls(&e, TEXTURE_RETURN_RENDERED).len(), 1);
    }

    #[test]
    fn render_with_both_flags_runs_each_effect() {
        let (mut e, this, effects) = rig();
        e.mem.set_u8(FIRST_FRAME_011FFFEC, 1);
        e.mem.set_u8(DISPLACE_011FFFED, 1);
        e.mem.set_f32(this + 0x60, 1.0);
        e.mem.set_u32(this + 0x58, 0x5858);
        // One point displaced by (1, 2) from its old place.
        let points = e.mem.alloc(8);
        e.mem.set_f32(points, 3.0);
        e.mem.set_f32(points + 4, 5.0);
        let old = e.mem.alloc(8);
        e.mem.set_f32(old, 2.0);
        e.mem.set_f32(old + 4, 3.0);
        e.mem.set_u32(POINTS_011ADA00, 0x0f00_9000);
        e.mem.set_u32(POINTS_011ADA00 + 4, points);
        e.mem.set_u32(POINTS_011ADA00 + 8, 1);
        e.mem.set_u32(OLD_POINTS_011ADA10, 0x0f00_9000);
        e.mem.set_u32(OLD_POINTS_011ADA10 + 4, old);
        e.put_vtable(0x0f00_9000, &[0, 0, 0x0f00_9008]);
        e.register(0x0f00_9008, |_, _| Ret::default());
        e.mem.set_f32(this + 0x5c, 0.5);
        e.call(0x00ba_0900, &args![this, 0x60u32, 0x61u32, 0u32, 0u32]);
        let renders = calls(&e, 0x0f00_0004);
        // Effect 0 (first frame), effect 1, then effect 2 per point on the screen geometry,
        // then effect 4 for each catch-up step.
        assert_eq!(renders[0], vec![effects[0], 0x60, 0x61, 0x9000, 1]);
        assert_eq!(renders[1], vec![effects[1], 0x60, 0x61, 0x9001, 1]);
        assert_eq!(renders[2], vec![effects[2], 0x5858, 0x61, 0x9002, 1]);
        assert!(renders[3..].iter().all(|w| w[0] == effects[4]));
        assert_eq!(e.mem.f32(0x0120_002c), 1.0);
        assert_eq!(e.mem.f32(0x0120_003c), 2.0);
        assert_eq!(e.mem.f32(0x0120_0030), -2.0);
        assert_eq!(e.mem.u32(DISPLACEMENTS_011FFFFC), 1);
        assert_eq!(e.mem.u8(FIRST_FRAME_011FFFEC), 0);
        assert_eq!(e.mem.u32(POINTS_011ADA00 + 4), 0);
        assert_eq!(
            calls(&e, 0x0f00_9008),
            vec![
                vec![POINTS_011ADA00, points],
                vec![OLD_POINTS_011ADA10, old]
            ]
        );
        let steps = renders.len() - 3;
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT).len(),
            usize::from(steps % 2 == 0)
        );
        assert_eq!(calls(&e, SET_CONSTANT_00B8A760).len(), 2);
    }
}

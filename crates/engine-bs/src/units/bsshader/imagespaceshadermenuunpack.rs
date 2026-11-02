//! `BSShader/imagespaceshadermenuunpack.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The number of unpack rectangles (`011ae5e2`, 16 bits), the rebuild flag (`011ae5d4`) and
/// the array of rectangle records (`011ae5dc`: each six floats, the texture's left and top,
/// width and height, then the screen's left and top).
const RECTANGLE_COUNT_011AE5E2: u32 = 0x011a_e5e2;
const REBUILD_FLAG_011AE5D4: u32 = 0x011a_e5d4;
const RECTANGLES_011AE5DC: u32 = 0x011a_e5dc;
/// The default for a null third argument (`01202410`).
const DEFAULT_PARAMETER_01202410: u32 = 0x0120_2410;
/// The object at `011f91ac`, whose `+0x200` holds the screen elements (an `NiPointer`), and
/// the renderer at `011f4748`.
const HOLDER_011F91AC: u32 = 0x011f_91ac;
const RENDERER_011F4748: u32 = 0x011f_4748;
/// The two screen offsets added to the rectangles (`012007d0`, `012007d4`, `float`s).
const OFFSET_X_012007D0: u32 = 0x0120_07d0;
const OFFSET_Y_012007D4: u32 = 0x0120_07d4;
/// The `double`s the code divides and multiplies by: 4/3 as a `float` (`01021790`), 1280
/// (`0106e960`), 960 (`0106e7f8`) and 2 (`01011590`); the `float` 2^32 (`01099378`).
const ASPECT_01021790: u32 = 0x0102_1790;
const WIDTH_0106E960: u32 = 0x0106_e960;
const HEIGHT_0106E7F8: u32 = 0x0106_e7f8;
const TWO_01011590: u32 = 0x0101_1590;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
/// `"Interface: HUD Unpack Geometry"` (`010b9d04`) and the null name handle word (`0109b220`).
const GEOMETRY_NAME_010B9D04: u32 = 0x010b_9d04;
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The allocation `00aa13e0` (`cdecl`, one word).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// The callees (engine map names; `thiscall` unless said): `ImageSpaceTexture::
/// GetRenderedTexture` (`00ba3770`), `NiScreenElementsData::NiScreenElementsData`
/// (`00a881e0`, nine words), `NiScreenElements::NiScreenElements` (`00a87810`, one word),
/// `NiScreenElementsData::Insert` (`00a87ae0`, three words), `::SetRectangle` (`00a88020`,
/// five words), `::UpdateBound` (`00a87900`), `::SetTextures` (`00a88130`, six words),
/// `NiGeometryData::SetConsistency` (`00a67050`, one word), the fixed-string maker
/// `00a5b690` (`cdecl`, one word), the name setter `00a5b950` (unit `niobjectnet.cpp`, one
/// word: the address of the handle), `NiAVObject::UpdateProperties` (`00a5a040`), `00a59c60`
/// (one word: the address of a float and five bytes), `ImageSpaceShader::SetVSConstants`
/// (`00c03c80`) and `NiScreenElements::Draw` (`00a87790`, one word: the renderer).
const GET_RENDERED_TEXTURE: u32 = 0x00ba_3770;
const SCREEN_ELEMENTS_DATA_CONSTRUCT: u32 = 0x00a8_81e0;
const SCREEN_ELEMENTS_CONSTRUCT: u32 = 0x00a8_7810;
const DATA_INSERT: u32 = 0x00a8_7ae0;
const DATA_SET_RECTANGLE: u32 = 0x00a8_8020;
const DATA_UPDATE_BOUND: u32 = 0x00a8_7900;
const DATA_SET_TEXTURES: u32 = 0x00a8_8130;
const DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
const MAKE_FIXED_STRING: u32 = 0x00a5_b690;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const SET_VS_CONSTANTS: u32 = 0x00c0_3c80;
const SCREEN_ELEMENTS_DRAW: u32 = 0x00a8_7790;

/// Releases an `NiPointer`'s object (`InterlockedDecrement` on `+4`, its slot +4 at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// `FILD` of an unsigned word: loaded signed, 2^32 (`01099378`) added when negative.
fn load_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        loaded
    }
}

/// The word at `offset` of the size object of `source`'s slot +0xc8 object (its slot +0xb0
/// with 0), or 0 when either is null.
fn target_size(e: &mut Engine, source: u32, offset: u32) -> u32 {
    let target = e.vcall(source, 0xc8, &args![]).u32();
    if target == 0 {
        return 0;
    }
    let size = e.vcall(target, 0xb0, &args![0u32]).u32();
    if size == 0 {
        return 0;
    }
    e.mem.u32(size + offset)
}

/// The screen elements `Render` makes: 0xc4 bytes from `00aa13e0` (null when none), on 0x70
/// bytes of `NiScreenElementsData` (`00a881e0` with 0, 0, 1, 1, 1, 4, 1, 2, 1) or on none when
/// that allocation fails.
fn new_screen_elements(e: &mut Engine) -> u32 {
    let elements = e.call(ALLOCATE_00AA13E0, &args![0xc4u32]).u32();
    if elements == 0 {
        return 0;
    }
    let block = e.call(ALLOCATE_00AA13E0, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            SCREEN_ELEMENTS_DATA_CONSTRUCT,
            &args![block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![elements, data])
        .u32()
}

/// One rectangle of the rebuild: the data gets a quad (`Insert` with 4, 0, 0); with
/// `aspect = (width / height) / (4/3)` (the source's size, stored as `float`s) and `fw = aspect
/// * 1280`, the rectangle `x' = (left / fw) * 2 - 1 + offX`, `y' = 1 - (top / 960) * 2 - offY`,
/// `w' = (width / fw) * 2`, `h' = 2 * -(height / 960)` and the texture coordinates `u0 / fw`,
/// `v0 / 960`, `u0 / fw + w / fw`, `v0 / 960 + h / 960` (each `float` as the code stores it, in
/// its x87 order); then the bound, the consistency (0) and the name.
fn build_rectangle(e: &mut Engine, elements: u32, source: u32, index: u32) {
    let width = target_size(e, source, 8);
    let height = target_size(e, source, 0xc);
    let ratio = x87_store32(x87_div(load_unsigned(e, width), load_unsigned(e, height)));
    let data = e.mem.u32(elements + 0xb8);
    let aspect = x87_store32(x87_div(x87_load32(ratio), e.mem.f64(ASPECT_01021790)));
    e.call(DATA_INSERT, &args![data, 4u32, 0u32, 0u32]);
    let fw = x87_mul(x87_load32(aspect), e.mem.f64(WIDTH_0106E960));
    let rectangles = e.mem.u32(RECTANGLES_011AE5DC);
    let record = e.mem.u32(rectangles.wrapping_add(index.wrapping_mul(4)));
    let field = |e: &Engine, offset: u32| e.mem.u32(record.wrapping_add(offset));
    let screen_height: f64 = e.mem.f64(HEIGHT_0106E7F8);
    let a = x87_store32(x87_div(x87_load32(field(e, 0x10)), fw));
    let b = x87_store32(x87_div(x87_load32(field(e, 0x14)), screen_height));
    let c = x87_store32(x87_div(x87_load32(field(e, 8)), fw));
    let d = x87_store32(x87_div(x87_load32(field(e, 0xc)), screen_height));
    let u0 = x87_store32(x87_div(x87_load32(field(e, 0)), fw));
    let v0 = x87_store32(x87_div(x87_m32(field(e, 4)), screen_height));
    let two: f64 = e.mem.f64(TWO_01011590);
    let x = x87_sub(x87_mul(x87_load32(a), two), 1.0);
    let x = x87_store32(x87_add(x, x87_load32(e.mem.u32(OFFSET_X_012007D0))));
    let y = x87_sub(1.0, x87_mul(x87_load32(b), two));
    let y = x87_store32(x87_sub(y, x87_load32(e.mem.u32(OFFSET_Y_012007D4))));
    let c_loaded = x87_load32(c);
    let w = x87_store32(x87_mul(c_loaded, two));
    let d_loaded = x87_load32(d);
    let h = x87_store32(x87_mul(two, -d_loaded));
    let u1 = x87_store32(x87_add(c_loaded, x87_m32(u0)));
    let v1 = x87_store32(x87_add(d_loaded, x87_m32(v0)));
    let q = x87_quiet_bits;
    e.call(
        DATA_SET_RECTANGLE,
        &args![data, index, q(x), q(y), q(w), q(h)],
    );
    let data = e.mem.u32(elements + 0xb8);
    e.call(DATA_UPDATE_BOUND, &args![data]);
    let data = e.mem.u32(elements + 0xb8);
    e.call(
        DATA_SET_TEXTURES,
        &args![data, index, 0u32, q(u0), q(v0), q(u1), q(v1)],
    );
    let data = e.mem.u32(elements + 0xb8);
    e.call(DATA_SET_CONSISTENCY, &args![data, 0u32]);
    e.with_stack(4, |e, handle| {
        let name = e
            .call(MAKE_FIXED_STRING, &args![GEOMETRY_NAME_010B9D04])
            .u32();
        e.mem.set_u32(handle.addr(), name);
        e.call(SET_NAME_00A5B950, &args![elements, handle]);
        let name = e.mem.u32(handle.addr());
        if name != e.mem.u32(NULL_NAME_0109B220) {
            e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
}

// Translated from 00bb7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderMenuUnpack::Render` (Xbox PDB; `thiscall` on the shader's sub-object at
/// `+0x6c`, four words: an unread word, a source whose slot +0xc8 gives the target the sizes
/// come from, a parameter (`01202410` when null) and an unread word). Nothing without unpack
/// rectangles (`011ae5e2`). When the rebuild flag (`011ae5d4`) is set: the rendered texture of
/// the image-space texture at `this + 0x2c`'s first word is held (counted); the old screen
/// elements at `011f91ac`'s `+0x200` are released and cleared; new screen elements are made
/// and get one quad per rectangle (see `build_rectangle`); their properties are updated
/// (`00a5a040`) and `00a59c60` runs with a zero float and five zero bytes; their `+0xc0` gets
/// the shader (`this - 0x6c`); they go into `011f91ac`'s `+0x200` (an `NiPointer` assignment,
/// counted without a null test); the held texture is released. Then, always: the shader's slot
/// +0x50 with the screen elements, its slot +0x148 with the parameter, `SetVSConstants`; when
/// the renderer's word `+0x200` is 1 or 2 and its byte `+0x208` is 1, its slot +0x190 with four
/// floats (0, 1, 1, 0); `NiScreenElements::Draw` with the renderer; the rebuild flag cleared.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7530 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn image_space_shader_menu_unpack_render(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    source: u32,
    parameter: u32,
    _unused_4: u32,
) {
    let base = this.addr();
    if e.mem.u16(RECTANGLE_COUNT_011AE5E2) == 0 {
        return;
    }
    let parameter = if parameter != 0 {
        parameter
    } else {
        DEFAULT_PARAMETER_01202410
    };
    if e.mem.u8(REBUILD_FLAG_011AE5D4) != 0 {
        let texture_holder = e.mem.u32(e.mem.u32(base + 0x2c));
        let held = e.call(GET_RENDERED_TEXTURE, &args![texture_holder]).u32();
        if held != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![held + 4]);
        }
        let slot = e.mem.u32(HOLDER_011F91AC).wrapping_add(0x200);
        let old = e.mem.u32(slot);
        if old != 0 {
            release(e, old);
            e.mem.set_u32(slot, 0);
        }
        let elements = new_screen_elements(e);
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(RECTANGLE_COUNT_011AE5E2)) {
            build_rectangle(e, elements, source, index);
            index += 1;
        }
        e.call(UPDATE_PROPERTIES, &args![elements]);
        e.with_stack(0xc, |e, update| {
            e.mem.set_u32(update.addr(), x87_store32(0.0));
            e.call(UPDATE_00A59C60, &args![elements, update]);
        });
        e.mem
            .set_u32(elements.wrapping_add(0xc0), base.wrapping_sub(0x6c));
        let slot = e.mem.u32(HOLDER_011F91AC).wrapping_add(0x200);
        let old = e.mem.u32(slot);
        if old != elements {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(slot, elements);
            e.call(INTERLOCKED_INCREMENT, &args![elements.wrapping_add(4)]);
        }
        if held != 0 {
            release(e, held);
        }
    }
    let elements = e.mem.u32(e.mem.u32(HOLDER_011F91AC).wrapping_add(0x200));
    let shader = base.wrapping_sub(0x6c);
    e.vcall(shader, 0x50, &args![elements]);
    e.vcall(shader, 0x148, &args![parameter]);
    e.call(SET_VS_CONSTANTS, &args![shader]);
    let renderer = e.mem.u32(RENDERER_011F4748);
    let mode = e.mem.u32(renderer + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(renderer + 0x208) == 1 {
        e.with_stack(0x10, |e, floats| {
            for (i, value) in [0.0f64, 1.0, 1.0, 0.0].into_iter().enumerate() {
                e.mem
                    .set_u32(floats.addr() + 4 * i as u32, x87_store32(value));
            }
            e.vcall(renderer, 0x190, &args![floats]);
        });
    }
    let renderer = e.mem.u32(RENDERER_011F4748);
    e.call(SCREEN_ELEMENTS_DRAW, &args![elements, renderer]);
    e.mem.set_u8(REBUILD_FLAG_011AE5D4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bb_7530,
        image_space_shader_menu_unpack_render(Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUB: AbiFn = |_, _| Ret::default();
    const SHADER_VTABLE: u32 = 0x0f00_1000;
    const SLOT_50: u32 = 0x0f00_0050;
    const SLOT_148: u32 = 0x0f00_0148;
    const SOURCE_VTABLE: u32 = 0x0f00_2000;
    const SOURCE_C8: u32 = 0x0f00_00c8;
    const TARGET_B0: u32 = 0x0f00_00b0;
    const RENDERER_VTABLE: u32 = 0x0f00_3000;
    const RENDERER_190: u32 = 0x0f00_0190;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    struct World {
        e: Engine,
        this: u32,
        source: u32,
        holder: u32,
        renderer: u32,
    }

    fn world() -> World {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0102_1000,
            0x0106_e000,
            0x0109_9000,
            0x0109_b000,
            0x011a_e000,
            0x011f_4000,
            0x011f_9000,
            0x0120_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(ASPECT_01021790, f64::from(4.0f32 / 3.0));
        e.mem.set_f64(WIDTH_0106E960, 1280.0);
        e.mem.set_f64(HEIGHT_0106E7F8, 960.0);
        e.mem.set_f64(TWO_01011590, 2.0);
        e.mem.set_f32(TWO_TO_32_01099378, 4_294_967_296.0);
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
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SCREEN_ELEMENTS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0f00_4000);
            e.mem.set_u32(a[0] + 0xb8, a[1]);
            a[0].into_ret()
        });
        e.put_vtable(0x0f00_4000, &[0, 0x0f00_0004]);
        for quiet in [
            0x0f00_0004,
            DATA_INSERT,
            DATA_SET_RECTANGLE,
            DATA_UPDATE_BOUND,
            DATA_SET_TEXTURES,
            DATA_SET_CONSISTENCY,
            SET_NAME_00A5B950,
            UPDATE_PROPERTIES,
            UPDATE_00A59C60,
            SET_VS_CONSTANTS,
            SCREEN_ELEMENTS_DRAW,
            SLOT_50,
            SLOT_148,
            RENDERER_190,
        ] {
            e.register(quiet, STUB);
        }
        e.register(MAKE_FIXED_STRING, |_, _| 0x0d00_0008u32.into_ret());
        e.map(0x0d00_0000, 0x10);
        let rendered = e.mem.alloc(8);
        e.mem.set_u32(rendered, 0x0f00_4000);
        e.mem.set_u32(rendered + 4, 1);
        e.register_double(GET_RENDERED_TEXTURE, move |_, _| rendered.into_ret());
        // The shader: its vtable at -0x6c of `this`.
        let mut slots = vec![0u32; 0x53];
        slots[0x14] = SLOT_50;
        slots[0x52] = SLOT_148;
        e.put_vtable(SHADER_VTABLE, &slots);
        let shader = e.mem.alloc(0x100);
        e.mem.set_u32(shader, SHADER_VTABLE);
        let this = shader + 0x6c;
        let image_texture = e.mem.alloc(4);
        let image_texture_holder = e.mem.alloc(4);
        e.mem.set_u32(image_texture_holder, image_texture);
        e.mem.set_u32(this + 0x2c, image_texture_holder);
        // The source and its target, sized 1600 x 900.
        let mut slots = vec![0u32; 0x33];
        slots[0x32] = SOURCE_C8;
        slots[0x2c] = TARGET_B0;
        e.put_vtable(SOURCE_VTABLE, &slots);
        let size = e.mem.alloc(0x10);
        e.mem.set_u32(size + 8, 1600);
        e.mem.set_u32(size + 0xc, 900);
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, SOURCE_VTABLE);
        e.register_double(SOURCE_C8, move |_, _| target.into_ret());
        e.register_double(TARGET_B0, move |_, _| size.into_ret());
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, SOURCE_VTABLE);
        let holder = e.mem.alloc(0x210);
        e.set_global(HOLDER_011F91AC, holder);
        let mut slots = vec![0u32; 0x65];
        slots[0x64] = RENDERER_190;
        e.put_vtable(RENDERER_VTABLE, &slots);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        e.set_global(RENDERER_011F4748, renderer);
        // One rectangle.
        let record = e.mem.alloc(0x18);
        for (i, v) in [100.0f32, 50.0, 200.0, 96.0, 400.0, 480.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(record + 4 * i as u32, *v);
        }
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, record);
        e.set_global(RECTANGLES_011AE5DC, array);
        e.set_global(RECTANGLE_COUNT_011AE5E2, 1u16);
        e.call_log = Some(vec![]);
        World {
            e,
            this,
            source,
            holder,
            renderer,
        }
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    #[test]
    fn rebuilds_the_quads_then_draws() {
        let mut w = world();
        let e = &mut w.e;
        e.set_global(REBUILD_FLAG_011AE5D4, 1u8);
        e.call(0x00bb_7530, &args![w.this, 0u32, w.source, 0u32, 0u32]);
        let elements = e.mem.u32(w.holder + 0x200);
        assert_ne!(elements, 0);
        assert_eq!(e.mem.u32(elements + 4), 1);
        assert_eq!(e.mem.u32(elements + 0xc0), w.this - 0x6c);
        // 1600 / 900 / (4/3) * 1280 = 1706.67 wide.
        let rect = &calls(e, DATA_SET_RECTANGLE)[0];
        assert_eq!(rect[1], 0);
        let fw = f32::from_bits(x87_store32(x87_div(1600.0, 900.0)));
        let aspect = (f64::from(fw) / f64::from(4.0f32 / 3.0)) as f32;
        let fw = f64::from(aspect) * 1280.0;
        let a = (400.0 / fw) as f32;
        assert_eq!(f(rect[2]), (f64::from(a) * 2.0 - 1.0) as f32);
        assert_eq!(f(rect[3]), 0.0);
        assert_eq!(f(rect[5]), -0.2);
        let tex = &calls(e, DATA_SET_TEXTURES)[0];
        assert_eq!(&tex[1..3], &[0, 0]);
        assert_eq!(f(tex[4]), (50.0f64 / 960.0) as f32);
        // The second coordinate pair adds the size: 96 / 960 more down.
        assert_eq!(
            f(tex[6]),
            ((50.0f64 / 960.0) as f32 + (96.0f64 / 960.0) as f32)
        );
        assert_eq!(
            calls(e, DATA_INSERT),
            vec![vec![e.mem.u32(elements + 0xb8), 4, 0, 0]]
        );
        // The held texture was counted and released again.
        assert_eq!(calls(e, GET_RENDERED_TEXTURE).len(), 1);
        // Then the draw with the default parameter, without the renderer's slot.
        let shader = w.this - 0x6c;
        assert_eq!(calls(e, SLOT_50), vec![vec![shader, elements]]);
        assert_eq!(
            calls(e, SLOT_148),
            vec![vec![shader, DEFAULT_PARAMETER_01202410]]
        );
        assert!(calls(e, RENDERER_190).is_empty());
        assert_eq!(
            calls(e, SCREEN_ELEMENTS_DRAW),
            vec![vec![elements, w.renderer]]
        );
        assert_eq!(e.mem.u8(REBUILD_FLAG_011AE5D4), 0);
    }

    #[test]
    fn draws_without_rebuilding_and_sets_the_renderer_floats() {
        let mut w = world();
        let e = &mut w.e;
        e.mem.set_u32(w.holder + 0x200, 0x7000);
        e.mem.set_u32(w.renderer + 0x200, 2);
        e.mem.set_u8(w.renderer + 0x208, 1);
        e.call(0x00bb_7530, &args![w.this, 0u32, w.source, 0x99u32, 0u32]);
        assert!(calls(e, SCREEN_ELEMENTS_CONSTRUCT).is_empty());
        assert_eq!(calls(e, SLOT_148)[0][1], 0x99);
        assert_eq!(calls(e, RENDERER_190).len(), 1);
        assert_eq!(
            calls(e, SCREEN_ELEMENTS_DRAW),
            vec![vec![0x7000, w.renderer]]
        );
        // No rectangles: nothing at all.
        e.set_global(RECTANGLE_COUNT_011AE5E2, 0u16);
        e.call_log = Some(vec![]);
        e.call(0x00bb_7530, &args![w.this, 0u32, w.source, 0x99u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

//! `fallout/misc/run_00878860` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Reads the word at its `this` (`00559450`, thiscall, no stack words).
const READ_POINTER: u32 = 0x0055_9450;
/// Reads the word at +0x0c of its `this` (`0084e3a0`, thiscall; the form id
/// when `this` is a form).
const FORM_ID: u32 = 0x0084_e3a0;
/// The pointer holder the function reads (+0x10 of `this`).
const HOLDER: u32 = 0x10;

// Translated from 00878f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form id of the object held at +0x10 (unnamed in the engine map): the
/// pointer is read through `00559450(this + 0x10)`; null gives 0, otherwise
/// the pointer is read again and `0084e3a0` gives the word at its +0x0c.
pub fn fn_00878f20(e: &mut Engine, this: Ptr) -> u32 {
    let holder = this.addr() + HOLDER;
    if e.call(READ_POINTER, &args![holder]).u32() == 0 {
        return 0;
    }
    let form = e.call(READ_POINTER, &args![holder]).u32();
    e.call(FORM_ID, &args![form]).u32()
}

/// `004bc3f0` (no arguments: the renderer) and `004dc020` (`thiscall` on it: its Direct3D
/// device, `renderer.cpp`).
const RENDERER_GETTER: u32 = 0x004b_c3f0;
const RENDERER_DEVICE: u32 = 0x004d_c020;
/// `NiPixelFormat::NiPixelFormat` (engine map, `00a7cca0`; `thiscall`) and the format's
/// size (0x11 words, copied with `REP MOVSD`).
const PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
const PIXEL_FORMAT_WORDS: u32 = 0x11;
/// `00e7bd20` (`cdecl`, one word: the Direct3D format of a pixel format).
const D3D_FORMAT_OF: u32 = 0x00e7_bd20;
/// The Direct3D 9 COM slots the screenshot uses: the device's `GetBackBuffer` (+0x48), the
/// surface's `GetDesc` (+0x30), `LockRect` (+0x34), `UnlockRect` (+0x38) and `Release`
/// (+0x8).
const GET_BACK_BUFFER: u32 = 0x48;
const SURFACE_GET_DESC: u32 = 0x30;
const SURFACE_LOCK_RECT: u32 = 0x34;
const SURFACE_UNLOCK_RECT: u32 = 0x38;
const SURFACE_RELEASE: u32 = 0x8;
/// `D3DLOCK_READONLY`.
const LOCK_READ_ONLY: u32 = 0x800;
/// The debug log (`005b5e40`, variadic `cdecl`) and its messages.
const LOG: u32 = 0x005b_5e40;
const NO_BACK_BUFFER_MESSAGE: u32 = 0x0108_3e24;
const NO_DESCRIPTION_MESSAGE: u32 = 0x0108_3de4;
const MULTISAMPLED_MESSAGE: u32 = 0x0108_3d88;
const NOT_ENABLED_MESSAGE: u32 = 0x0108_3d18;
const CANNOT_CREATE_MESSAGE: u32 = 0x0108_3cb4;
/// `004dc310` (no arguments: whether the multisample type is 2 or more).
const MULTISAMPLED: u32 = 0x004d_c310;
/// `00aa13e0` (`cdecl`, one word: allocates) and `NiPixelData::NiPixelData_ov2` (engine map,
/// `00a7c190`; `thiscall`, five words: width, height, format, two counts) on 0x74 bytes.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
const PIXEL_DATA_SIZE: u32 = 0x74;
/// The render width and height (`004dc1f0`, `004dc200`; no arguments).
const RENDER_WIDTH: u32 = 0x004d_c1f0;
const RENDER_HEIGHT: u32 = 0x004d_c200;
/// `0064da80` (`thiscall` on the pixel data, two words: its width), `0070ec90` (`thiscall`:
/// its bytes per pixel; the map's `DetailedActorPathHandler::GetCurrentNodeIndex`, a shared
/// body) and `0064dac0` (`thiscall`, two words: its pixels).
const PIXEL_DATA_WIDTH: u32 = 0x0064_da80;
const PIXEL_DATA_PIXEL_SIZE: u32 = 0x0070_ec90;
const PIXEL_DATA_PIXELS: u32 = 0x0064_dac0;
/// `00401460` (`cdecl`: destination, source, count; a `memcpy`).
const COPY_BYTES: u32 = 0x0040_1460;
/// `strcpy_s` and `sprintf_s` wrappers (`00406d30`, `00406d00`; `cdecl`), `sprintf`
/// (`00ec623a`), the name format `"%s%d.bmp"` and `"ScreenShot: File '%s' created."`.
const STRING_COPY: u32 = 0x0040_6d30;
const FORMAT_SIZED: u32 = 0x0040_6d00;
const FORMAT: u32 = 0x00ec_623a;
const NAME_FORMAT: u32 = 0x0108_3d08;
const CREATED_MESSAGE: u32 = 0x0108_3ce8;
/// The settings `iScreenShotIndex:Display` (`011df124`, its value's address through
/// `0043d4d0`) and `sScreenShotBaseName:Display` (`011df0f8`, its text through `00403df0`).
const SCREEN_SHOT_INDEX: u32 = 0x011d_f124;
const SCREEN_SHOT_BASE_NAME: u32 = 0x011d_f0f8;
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
const SETTING_STRING: u32 = 0x0040_3df0;
/// `BSSystemFile`'s constructor (`00b00900`, `thiscall`: name, two modes, 0), destructor
/// (`00b00950`) and `BSSystemFile::DoWrite` (engine map, `00856350`; `thiscall`: data,
/// size, 0, the written count's address).
const SYSTEM_FILE_CONSTRUCT: u32 = 0x00b0_0900;
const SYSTEM_FILE_DESTRUCT: u32 = 0x00b0_0950;
const SYSTEM_FILE_WRITE: u32 = 0x0085_6350;
/// `operator new` and `operator delete` (`00401000`, `00401030`; `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The interface message (`007052f0`, `cdecl`, six words) and its `float` (2.0,
/// `010162c0`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const MESSAGE_TIME_010162C0: u32 = 0x0101_62c0;
/// `004de490` (no arguments) and `008798e0` (`thiscall` on its answer): run after a
/// screenshot by number.
const SETTINGS_004DE490: u32 = 0x004d_e490;
const AFTER_SCREEN_SHOT_008798E0: u32 = 0x0087_98e0;
/// The size of the BMP file and information headers.
const BMP_HEADER_SIZE: u32 = 0x36;

/// The function's frame (`EBP-0x340` up to `EBP`): `at(offset)` is `EBP - offset`.
const FRAME_SIZE: u32 = 0x340;

// Translated from 00878860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00878860 - the compiler's exception frame (FS:[0] chain, handler 00f1453d, states 0 and 1 for the pixel data block and the file) and the /GS stack cookie (011c16bc, checked by 00ec408c) are not translated
/// Unnamed in the engine map (`cdecl`, one word: a file name or null): the screenshot.
/// Copies the renderer's back buffer (`GetBackBuffer`, `GetDesc`, a read-only `LockRect`)
/// row by row into new `NiPixelData` of the render size in the renderer's pixel format
/// (`00878f20` of its vtable +0xD4 object), each failure logged (`"... Unable to get back
/// buffer."`, `"... Unable to acquire BackBuffer Description"`, and for the lock
/// `"... Can't lock the backbuffer when multisampling is enabled."` or `"... Add
/// 'bAllowScreenShot = 1' ..."`) and the surface released. The file is `name`, or
/// `sScreenShotBaseName` followed by `iScreenShotIndex` and `.bmp`; it is opened as a
/// `BSSystemFile` and, unless that fails (`"... Unable to create file '%s'."`), gets a
/// 24-bit BMP: the 0x36-byte header, then the pixels' first three bytes each, bottom row
/// first. A numbered screenshot then shows `"ScreenShot: File '%s' created."`
/// (`007052f0`), counts the index up and runs `008798e0` on `004de490()`. Last, the pixel
/// data is deleted (vtable slot 0 with 1) and the file closed.
pub fn fn_00878860(e: &mut Engine, name: u32) {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let at = |offset: u32| frame.addr() + FRAME_SIZE - offset;
        let format = at(0x84);
        let surface_slot = at(0xac);
        let description = at(0x34);
        let locked = at(0x3c);
        let file_name = at(0x1bc);
        let file = at(0xa4);
        let header = at(0x20c);
        let written = at(0x1cc);
        let message = at(0x334);

        let renderer = e.call(RENDERER_GETTER, &args![]).u32();
        e.call(PIXEL_FORMAT_CONSTRUCT, &args![format]);
        let device = e.call(RENDERER_DEVICE, &args![renderer]).u32();
        let holder = e.vcall(renderer, 0xd4, &args![]).u32();
        let source = fn_00878f20(e, Ptr::new(holder));
        for word in 0..PIXEL_FORMAT_WORDS {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(format + 4 * word, value);
        }
        e.call(D3D_FORMAT_OF, &args![format]);
        if e.vcall(
            device,
            GET_BACK_BUFFER,
            &args![0u32, 0u32, 0u32, surface_slot],
        )
        .u32()
            != 0
        {
            e.call(LOG, &args![NO_BACK_BUFFER_MESSAGE]);
            return;
        }
        let surface = e.mem.u32(surface_slot);
        if e.vcall(surface, SURFACE_GET_DESC, &args![description])
            .u32()
            != 0
        {
            e.call(LOG, &args![NO_DESCRIPTION_MESSAGE]);
            let surface = e.mem.u32(surface_slot);
            e.vcall(surface, SURFACE_RELEASE, &args![]);
            return;
        }
        let surface = e.mem.u32(surface_slot);
        if e.vcall(
            surface,
            SURFACE_LOCK_RECT,
            &args![locked, 0u32, LOCK_READ_ONLY],
        )
        .u32()
            != 0
        {
            let text = if e.call(MULTISAMPLED, &args![]).bool() {
                MULTISAMPLED_MESSAGE
            } else {
                NOT_ENABLED_MESSAGE
            };
            e.call(LOG, &args![text]);
            let surface = e.mem.u32(surface_slot);
            e.vcall(surface, SURFACE_RELEASE, &args![]);
            return;
        }
        let block = e.call(NI_ALLOCATE, &args![PIXEL_DATA_SIZE]).u32();
        let pixel_data = if block != 0 {
            let height = e.call(RENDER_HEIGHT, &args![]).u32();
            let width = e.call(RENDER_WIDTH, &args![]).u32();
            e.call(
                PIXEL_DATA_CONSTRUCT,
                &args![block, width, height, format, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let width = e
            .call(PIXEL_DATA_WIDTH, &args![pixel_data, 0u32, 0u32])
            .u32();
        let row_bytes = width.wrapping_mul(e.call(PIXEL_DATA_PIXEL_SIZE, &args![pixel_data]).u32());
        let mut destination = e
            .call(PIXEL_DATA_PIXELS, &args![pixel_data, 0u32, 0u32])
            .u32();
        let mut source = e.mem.u32(locked + 4);
        let mut row = 0i32;
        while row < e.call(RENDER_HEIGHT, &args![]).i32() {
            e.call(COPY_BYTES, &args![destination, source, row_bytes]);
            destination = destination.wrapping_add(row_bytes);
            source = source.wrapping_add(e.mem.u32(locked));
            row += 1;
        }
        let surface = e.mem.u32(surface_slot);
        e.vcall(surface, SURFACE_UNLOCK_RECT, &args![]);
        let surface = e.mem.u32(surface_slot);
        e.vcall(surface, SURFACE_RELEASE, &args![]);
        if name != 0 {
            e.call(STRING_COPY, &args![file_name, 0x104u32, name]);
        } else {
            let index_at = e.call(SETTING_INT_POINTER, &args![SCREEN_SHOT_INDEX]).u32();
            let index = e.mem.u32(index_at);
            let base_name = e.call(SETTING_STRING, &args![SCREEN_SHOT_BASE_NAME]).u32();
            e.call(
                FORMAT_SIZED,
                &args![file_name, 0x104u32, NAME_FORMAT, base_name, index],
            );
        }
        e.call(
            SYSTEM_FILE_CONSTRUCT,
            &args![file, file_name, 2u32, 2u32, 0u32],
        );
        if e.call(READ_POINTER, &args![file]).u32() != 0 {
            e.call(LOG, &args![CANNOT_CREATE_MESSAGE, file_name]);
        } else {
            write_bitmap(e, pixel_data, file, header, written);
            if name == 0 {
                e.call(FORMAT, &args![message, CREATED_MESSAGE, file_name]);
                let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME_010162C0));
                e.call(SHOW_MESSAGE, &args![message, 0u32, 0u32, 0u32, time, 0u32]);
                let index_at = e.call(SETTING_INT_POINTER, &args![SCREEN_SHOT_INDEX]).u32();
                let index = e.mem.u32(index_at).wrapping_add(1);
                let index_at = e.call(SETTING_INT_POINTER, &args![SCREEN_SHOT_INDEX]).u32();
                e.mem.set_u32(index_at, index);
                let settings = e.call(SETTINGS_004DE490, &args![]).u32();
                e.call(AFTER_SCREEN_SHOT_008798E0, &args![settings]);
            }
        }
        if pixel_data != 0 {
            e.vcall(pixel_data, 0, &args![1u32]);
        }
        e.call(SYSTEM_FILE_DESTRUCT, &args![file]);
    });
}

/// The BMP part of [`fn_00878860`]: a buffer of `width * height * 3` bytes (`operator
/// new`) gets the first three bytes of each pixel, rows from the last to the first
/// (signed), columns from the first (unsigned); then the 0x36-byte header (`BM`, the file
/// size, offset 0x36, a 0x28-byte information header with width, height, one plane, 24
/// bits, no compression and the image size) and the buffer are written, and the buffer
/// deleted.
fn write_bitmap(e: &mut Engine, pixel_data: u32, file: u32, header: u32, written: u32) {
    let width = e
        .call(PIXEL_DATA_WIDTH, &args![pixel_data, 0u32, 0u32])
        .u32();
    let height = e.call(RENDER_HEIGHT, &args![]).u32();
    let mut pixel_size = e.call(PIXEL_DATA_PIXEL_SIZE, &args![pixel_data]).u32();
    let pixels = e
        .call(PIXEL_DATA_PIXELS, &args![pixel_data, 0u32, 0u32])
        .u32();
    let buffer = e
        .call(
            OPERATOR_NEW,
            &args![width.wrapping_mul(height).wrapping_mul(3)],
        )
        .u32();
    let mut out = 0u32;
    let mut row = height.wrapping_sub(1) as i32;
    while row >= 0 {
        let mut column = 0u32;
        while column < width {
            let offset = width
                .wrapping_mul(pixel_size)
                .wrapping_mul(row as u32)
                .wrapping_add(column.wrapping_mul(pixel_size));
            for byte in 0..3u32 {
                let value = e.mem.u8(pixels.wrapping_add(offset).wrapping_add(byte));
                e.mem
                    .set_u8(buffer.wrapping_add(out).wrapping_add(byte), value);
            }
            out = out.wrapping_add(3);
            column += 1;
        }
        row -= 1;
    }
    pixel_size = 3;
    let image_size = width.wrapping_mul(height).wrapping_mul(pixel_size);
    e.mem.set_u16(header, 0x4d42);
    e.mem
        .set_u32(header + 2, image_size.wrapping_add(BMP_HEADER_SIZE));
    e.mem.set_u16(header + 6, 0);
    e.mem.set_u16(header + 8, 0);
    e.mem.set_u32(header + 0xa, BMP_HEADER_SIZE);
    e.mem.set_u32(header + 0xe, 0x28);
    e.mem.set_u32(header + 0x12, width);
    e.mem.set_u32(header + 0x16, height);
    e.mem.set_u16(header + 0x1a, 1);
    e.mem.set_u16(header + 0x1c, (pixel_size << 3) as u16);
    e.mem.set_u32(header + 0x1e, 0);
    e.mem.set_u32(header + 0x22, image_size);
    for word in 0..4u32 {
        e.mem.set_u32(header + 0x26 + 4 * word, 0);
    }
    e.call(
        SYSTEM_FILE_WRITE,
        &args![file, header, BMP_HEADER_SIZE, 0u32, written],
    );
    e.call(
        SYSTEM_FILE_WRITE,
        &args![file, buffer, image_size, 0u32, written],
    );
    e.call(OPERATOR_DELETE, &args![buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_8f20, fn_00878f20(Ptr) -> u32),
        entry!(0x0087_8860, fn_00878860(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_the_held_forms_id_or_zero() {
        let mut e = Engine::new();
        e.register(READ_POINTER, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FORM_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x0c),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8f20, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == READ_POINTER).count(), 1);
        assert!(!log.iter().any(|(a, _)| *a == FORM_ID));

        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0x0c, 0x0001_2345);
        e.mem.set_u32(this + HOLDER, form);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_8f20, &args![this]).u32(), 0x0001_2345);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|(a, _)| *a == READ_POINTER).collect();
        assert_eq!(reads.len(), 2);
        assert!(reads.iter().all(|(_, w)| *w == vec![this + HOLDER]));
        let ids: Vec<_> = log.iter().filter(|(a, _)| *a == FORM_ID).collect();
        assert_eq!(ids[0].1, vec![form]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Writes = Rc<RefCell<Vec<Vec<u8>>>>;

    /// The COM slots as the rig places them.
    const BACK_BUFFER_SLOT: u32 = 0x0090_0048;
    const DESC_SLOT: u32 = 0x0090_1030;
    const LOCK_SLOT: u32 = 0x0090_1034;
    const UNLOCK_SLOT: u32 = 0x0090_1038;
    const RELEASE_SLOT: u32 = 0x0090_1008;
    const HOLDER_SLOT: u32 = 0x0090_20d4;
    const PIXEL_DATA_DELETE: u32 = 0x0090_3000;

    struct Rig {
        e: Engine,
        writes: Writes,
        pixels: u32,
        surface_bits: u32,
    }

    /// A 2x2 screen whose back buffer rows (pitch 16, 4 bytes per pixel) hold the bytes
    /// `row * 0x10 + column * 4 + byte`; `failing` is the COM slot that answers an error.
    fn rig(failing: u32) -> Rig {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x011d_f000] {
            e.map(page, 0x1000);
        }
        e.set_global(MESSAGE_TIME_010162C0, 2.0f32);
        let renderer = e.mem.alloc(4);
        e.put_vtable(0x0090_2000, &[0; 0x36]);
        e.mem.set_u32(0x0090_2000 + 0xd4, HOLDER_SLOT);
        e.mem.set_u32(renderer, 0x0090_2000);
        e.register_double(RENDERER_GETTER, move |_, _| renderer.into_ret());
        let device = e.mem.alloc(4);
        e.put_vtable(0x0090_0000, &[0; 0x13]);
        e.mem.set_u32(0x0090_0000 + 0x48, BACK_BUFFER_SLOT);
        e.mem.set_u32(device, 0x0090_0000);
        e.register_double(RENDERER_DEVICE, move |_, _| device.into_ret());
        // The holder's +0x10 word holds an object whose +0xC word is the format.
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format, 0xf0f0_f0f0);
        let held = e.mem.alloc(0x10);
        e.mem.set_u32(held + 0xc, format);
        let holder = e.mem.alloc(0x14);
        e.mem.set_u32(holder + 0x10, held);
        e.register_double(HOLDER_SLOT, move |_, _| holder.into_ret());
        e.register(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(PIXEL_FORMAT_CONSTRUCT, |_, _| Ret::default());
        e.register(D3D_FORMAT_OF, |_, _| 0x16u32.into_ret());
        let surface = e.mem.alloc(4);
        e.put_vtable(0x0090_1000, &[0; 0x0f]);
        for (slot, target) in [
            (0x30, DESC_SLOT),
            (0x34, LOCK_SLOT),
            (0x38, UNLOCK_SLOT),
            (0x8, RELEASE_SLOT),
        ] {
            e.mem.set_u32(0x0090_1000 + slot, target);
        }
        e.mem.set_u32(surface, 0x0090_1000);
        let surface_bits = e.mem.alloc(0x20);
        for row in 0..2u32 {
            for column in 0..4u32 {
                e.mem.set_u8(
                    surface_bits + row * 0x10 + column,
                    (row * 0x10 + column) as u8,
                );
            }
        }
        let answer = move |slot: u32| u32::from(slot == failing) * 0x8876_0000;
        e.register_double(BACK_BUFFER_SLOT, move |e, a| {
            e.mem.set_u32(a[4], surface);
            answer(BACK_BUFFER_SLOT).into_ret()
        });
        e.register_double(DESC_SLOT, move |_, _| answer(DESC_SLOT).into_ret());
        e.register_double(LOCK_SLOT, move |e, a| {
            e.mem.set_u32(a[1], 0x10);
            e.mem.set_u32(a[1] + 4, surface_bits);
            answer(LOCK_SLOT).into_ret()
        });
        for slot in [UNLOCK_SLOT, RELEASE_SLOT, PIXEL_DATA_DELETE] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(MULTISAMPLED, |_, _| 1u32.into_ret());
        e.register(LOG, |_, _| Ret::default());
        // The pixel data: 2x2, 3 bytes per pixel, its pixels in a block of its own.
        let pixel_data = e.mem.alloc(PIXEL_DATA_SIZE);
        let pixels = e.mem.alloc(12);
        e.put_vtable(0x0090_4000, &[PIXEL_DATA_DELETE]);
        e.mem.set_u32(pixel_data, 0x0090_4000);
        e.register_double(NI_ALLOCATE, move |_, _| pixel_data.into_ret());
        e.register(PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(RENDER_WIDTH, |_, _| 2u32.into_ret());
        e.register(RENDER_HEIGHT, |_, _| 2u32.into_ret());
        e.register(PIXEL_DATA_WIDTH, |_, _| 2u32.into_ret());
        e.register(PIXEL_DATA_PIXEL_SIZE, |_, _| 3u32.into_ret());
        e.register_double(PIXEL_DATA_PIXELS, move |_, _| pixels.into_ret());
        e.register(COPY_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRING_COPY, |_, _| Ret::default());
        e.register(FORMAT_SIZED, |_, _| Ret::default());
        e.register(FORMAT, |_, _| Ret::default());
        e.register(SETTING_STRING, |_, _| 0x0300_0000u32.into_ret());
        let index = e.mem.alloc(4);
        e.mem.set_u32(index, 7);
        e.register_double(SETTING_INT_POINTER, move |_, a| {
            assert_eq!(a[0], SCREEN_SHOT_INDEX);
            index.into_ret()
        });
        e.register(SYSTEM_FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(SYSTEM_FILE_DESTRUCT, |_, _| Ret::default());
        let writes: Writes = Rc::new(RefCell::new(vec![]));
        let record = writes.clone();
        e.register_double(SYSTEM_FILE_WRITE, move |e, a| {
            record.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        e.register(SETTINGS_004DE490, |_, _| 0x0c00_0000u32.into_ret());
        e.register(AFTER_SCREEN_SHOT_008798E0, |_, _| Ret::default());
        Rig {
            e,
            writes,
            pixels,
            surface_bits,
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn a_numbered_screenshot_writes_a_bottom_up_bmp() {
        let mut r = rig(0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x0087_8860, &args![0u32]);
        // The rows were copied with the pixel data's row size (2 * 3 bytes).
        let copies = calls(&r.e, COPY_BYTES);
        assert_eq!(
            copies,
            vec![
                vec![r.pixels, r.surface_bits, 6],
                vec![r.pixels + 6, r.surface_bits + 0x10, 6]
            ]
        );
        let writes = r.writes.borrow();
        assert_eq!(writes.len(), 2);
        let header = &writes[0];
        assert_eq!(header.len(), 0x36);
        assert_eq!(&header[0..2], b"BM");
        assert_eq!(
            u32::from_le_bytes(header[2..6].try_into().unwrap()),
            0x36 + 12
        );
        assert_eq!(
            u32::from_le_bytes(header[0xa..0xe].try_into().unwrap()),
            0x36
        );
        assert_eq!(
            u32::from_le_bytes(header[0x12..0x16].try_into().unwrap()),
            2
        );
        assert_eq!(
            u16::from_le_bytes(header[0x1c..0x1e].try_into().unwrap()),
            24
        );
        assert_eq!(
            u32::from_le_bytes(header[0x22..0x26].try_into().unwrap()),
            12
        );
        // Bottom row first: row 1 (0x10..) then row 0, three bytes per pixel.
        assert_eq!(
            writes[1],
            vec![0x10, 0x11, 0x12, 0x13, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x00, 0x00]
        );
        drop(writes);
        let index = calls(&r.e, SETTING_INT_POINTER);
        assert_eq!(index.len(), 3);
        let at =
            r.e.call(SETTING_INT_POINTER, &args![SCREEN_SHOT_INDEX])
                .u32();
        assert_eq!(r.e.mem.u32(at), 8);
        let format = calls(&r.e, FORMAT_SIZED);
        assert_eq!(format[0][1..], [0x104, NAME_FORMAT, 0x0300_0000, 7]);
        let shown = calls(&r.e, SHOW_MESSAGE);
        assert_eq!(shown[0][1..], [0, 0, 0, 2.0f32.to_bits(), 0]);
        assert_eq!(
            calls(&r.e, AFTER_SCREEN_SHOT_008798E0),
            vec![vec![0x0c00_0000]]
        );
        assert_eq!(calls(&r.e, PIXEL_DATA_DELETE).len(), 1);
        assert_eq!(calls(&r.e, SYSTEM_FILE_DESTRUCT).len(), 1);
        assert_eq!(calls(&r.e, RELEASE_SLOT).len(), 1);
    }

    #[test]
    fn a_named_screenshot_shows_no_message() {
        let mut r = rig(0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x0087_8860, &args![0x0310_0000u32]);
        let copy = calls(&r.e, STRING_COPY);
        assert_eq!(copy[0][1..], [0x104, 0x0310_0000]);
        assert_eq!(r.writes.borrow().len(), 2);
        assert!(calls(&r.e, SHOW_MESSAGE).is_empty());
        assert!(calls(&r.e, SETTING_INT_POINTER).is_empty());
    }

    #[test]
    fn a_file_that_cannot_be_created_is_logged() {
        let mut r = rig(0);
        r.e.register(SYSTEM_FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 1);
            Ret::default()
        });
        r.e.call_log = Some(vec![]);
        r.e.call(0x0087_8860, &args![0x0310_0000u32]);
        let logged = calls(&r.e, LOG);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][0], CANNOT_CREATE_MESSAGE);
        assert!(r.writes.borrow().is_empty());
        assert_eq!(calls(&r.e, PIXEL_DATA_DELETE).len(), 1);
        assert_eq!(calls(&r.e, SYSTEM_FILE_DESTRUCT).len(), 1);
    }

    #[test]
    fn back_buffer_failures_are_logged() {
        for (failing, message, released) in [
            (BACK_BUFFER_SLOT, NO_BACK_BUFFER_MESSAGE, 0usize),
            (DESC_SLOT, NO_DESCRIPTION_MESSAGE, 1),
            (LOCK_SLOT, MULTISAMPLED_MESSAGE, 1),
        ] {
            let mut r = rig(failing);
            r.e.call_log = Some(vec![]);
            r.e.call(0x0087_8860, &args![0u32]);
            assert_eq!(calls(&r.e, LOG), vec![vec![message]]);
            assert_eq!(calls(&r.e, RELEASE_SLOT).len(), released);
            assert!(calls(&r.e, NI_ALLOCATE).is_empty());
        }
        // Without multisampling the lock failure asks for `bAllowScreenShot`.
        let mut r = rig(LOCK_SLOT);
        r.e.register(MULTISAMPLED, |_, _| 0u32.into_ret());
        r.e.call_log = Some(vec![]);
        r.e.call(0x0087_8860, &args![0u32]);
        assert_eq!(calls(&r.e, LOG), vec![vec![NOT_ENABLED_MESSAGE]]);
    }
}

//! `fallout/misc/screenshot.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ScreenShot::TakeSaveGameScreenShot` (`00878f60`) renders the scene into
//! a texture, copies it into a 24-bit image object and, when it is given a
//! file path, writes it there as a BMP file. Two small setters of four
//! `float`s on one object (`00879860` sets them, `008798a0` resets them)
//! follow it in the exe.
//!
//! The renderer's objects are called through their virtual tables by slot
//! offset; the slot numbers (`0x44`, `0x48`, `0x80`, `0x90`, `0x34`, `0x38`)
//! match the Direct3D 9 device and surface interfaces, but the code does not
//! name them, so they are described by position here.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00404eb0` / `00404ee0`: construct and destroy a scope object kept on the
/// stack (thiscall; the constructor takes `0x11`, `1`, the address of a text
/// and a line number).
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTRUCT: u32 = 0x0040_4ee0;
/// The text and the line the scope is given.
const SCOPE_TEXT: u32 = 0x0108_3f0c;
const SCOPE_LINE: u32 = 0x139;
/// `004bc3f0` (returns the renderer object) and `004dc020` (on it: returns
/// the device object whose slots `0x80` and `0x90` are used).
const GET_RENDERER: u32 = 0x004b_c3f0;
const GET_DEVICE: u32 = 0x004d_c020;
/// `004ea970` returns the rendering-system object `00ba2f30`, `00ba30f0`,
/// `00ba3130` and `00ba2fa0` are called on.
const GET_RENDERING_SYSTEM: u32 = 0x004e_a970;
const RENDERING_SYSTEM_BEGIN: u32 = 0x00ba_2f30;
const RENDERING_SYSTEM_STAGE_A: u32 = 0x00ba_30f0;
const RENDERING_SYSTEM_STAGE_B: u32 = 0x00ba_3130;
const RENDERING_SYSTEM_END: u32 = 0x00ba_2fa0;
/// The renderer's virtual slot whose result `00878f20` turns into a
/// pointer to a 0x44-byte description that is copied to a local (unused
/// afterwards).
const RENDERER_SLOT_DESCRIPTION: u32 = 0xd4;
const DESCRIPTION_GETTER: u32 = 0x0087_8f20;
const DESCRIPTION_SIZE: u32 = 0x44;
/// `004f31d0`: constructs a three-word record (thiscall).
const RECORD_CONSTRUCT: u32 = 0x004f_31d0;
/// `00438170`: constructs a fixed string from a text (thiscall, one stack
/// word); `004381b0` destroys it.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// The text the fixed string is made from.
const RENDER_TARGET_NAME: u32 = 0x0108_3efc;
/// `00b6b610`: cdecl, nine words (the fixed string, width, height, the
/// record, 0, 1, 0, 0, 0); returns the object `NiPointer` `00633c90` keeps.
const CREATE_RENDER_TARGET: u32 = 0x00b6_b610;
/// `NiPointer` constructor from a raw pointer (`00633c90`, thiscall, one
/// word), the pointer getter (`00559450`), the assignment (`0066b0d0`, one
/// word) and the destructor (`0045cec0`).
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const SMART_POINTER_GET: u32 = 0x0055_9450;
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `005b5e40`: writes a line to the log (cdecl: format text, arguments).
const LOG: u32 = 0x005b_5e40;
/// The messages: no render target, the surface creation failed, the file
/// could not be opened (takes the path).
const MESSAGE_NO_RENDER_TARGET: u32 = 0x0108_3ec4;
const MESSAGE_SURFACE_FAILED: u32 = 0x0108_3e8c;
const MESSAGE_FILE_FAILED: u32 = 0x0108_3e58;
/// `004dc200` / `004dc1f0`: two integers whose ratio scales the width.
const SCREEN_NUMERATOR: u32 = 0x004d_c200;
const SCREEN_DENOMINATOR: u32 = 0x004d_c1f0;
/// `_ftol2_sse` (`00ec62c0`): the `double` argument as two words.
const FTOL: u32 = 0x00ec_62c0;
/// The global pointer to the player character; `0094ae40` is its
/// `PlayerCharacter::UpdateCamera` (Xbox PDB, two words) and `009466d0`
/// another method of it (a `float` and a flag).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
const PLAYER_UPDATE_CAMERA: u32 = 0x0094_ae40;
const PLAYER_METHOD_9466D0: u32 = 0x0094_66d0;
/// The global pointer to the object `008706b0` (three words) and `005bb4d0`
/// are called on.
const MAIN_OBJECT_POINTER: u32 = 0x011d_ea0c;
const MAIN_OBJECT_METHOD_8706B0: u32 = 0x0087_06b0;
const MAIN_OBJECT_TEST_5BB4D0: u32 = 0x005b_b4d0;
/// `004e3270` returns the object whose four `float`s `00879860` and
/// `008798a0` set.
const GET_FLOAT_OWNER: u32 = 0x004e_3270;
const SET_FLOATS: u32 = 0x0087_9860;
const RESET_FLOATS: u32 = 0x0087_98a0;
/// `004bc320` (on the raw render target, one word): the texture object;
/// `0059bb30` (on it): the object whose slot `0xa4` returns the object
/// whose slot `0x9c` returns the object the slots `0x44` and `0x48` are
/// called on.
const RENDER_TARGET_TEXTURE: u32 = 0x004b_c320;
const TEXTURE_OWNER: u32 = 0x0059_bb30;
const OWNER_SLOT_A4: u32 = 0xa4;
const OWNER_SLOT_9C: u32 = 0x9c;
/// The slots of the object `0x9c` returned: each writes a pointer or a
/// value through its second word.
const DEVICE_SLOT_44: u32 = 0x44;
const DEVICE_SLOT_48: u32 = 0x48;
/// The renderer's device slots: creating a surface (`0x90`: height, width,
/// the value `0x44` wrote, 2, the out pointer, 0) and copying one into
/// another (`0x80`).
const DEVICE_CREATE_SURFACE: u32 = 0x90;
const DEVICE_COPY_SURFACE: u32 = 0x80;
/// The surfaces' slots: lock (`0x34`: a record, 0, `0x800`), unlock
/// (`0x38`) and release (`8`).
const SURFACE_LOCK: u32 = 0x34;
const SURFACE_UNLOCK: u32 = 0x38;
const RELEASE: u32 = 8;
/// The flag word of the lock (`0x800`).
const LOCK_FLAGS: u32 = 0x800;
/// The 0x44-byte image description at `011aa2e8` and the image object:
/// `00aa13e0` allocates 0x74 bytes, `00a7c190` constructs it (thiscall;
/// width, height, description, 1, 1); `0064dac0` gives its pixels
/// (`(object, 0, 0)`), `0070ec90` its bytes per pixel.
const IMAGE_DESCRIPTION: u32 = 0x011a_a2e8;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const IMAGE_SIZE: u32 = 0x74;
const IMAGE_CONSTRUCT: u32 = 0x00a7_c190;
const IMAGE_PIXELS: u32 = 0x0064_dac0;
const IMAGE_BYTES_PER_PIXEL: u32 = 0x0070_ec90;
/// `0044a670` (cdecl, the path): non-zero when the file is to be written;
/// `00aff0b0` (cdecl, the path) prepares for it.
const PATH_CHECK: u32 = 0x0044_a670;
const PATH_PREPARE: u32 = 0x00af_f0b0;
/// `00401000` / `00401030`: allocate / free (cdecl).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The size of the file object (`0x158`) and its constructor `00b00260`
/// (`BSFile::BSFile`, Xbox PDB: path, 1, `0x4000`, 0); slot `0x20` of it
/// (two words, a byte result) says whether it opened; `00473180` writes
/// (`(data, size)`).
const FILE_SIZE: u32 = 0x158;
const FILE_CONSTRUCT: u32 = 0x00b0_0260;
const FILE_BUFFER_SIZE: u32 = 0x4000;
const FILE_SLOT_IS_OPEN: u32 = 0x20;
const FILE_WRITE: u32 = 0x0047_3180;
/// The size of a BMP file header plus its info header.
const BMP_HEADER_SIZE: u32 = 0x36;

/// Offsets in the frame holding the function's stack locals.
const FRAME_SIZE: u32 = 0x200;
const FRAME_SCOPE: u32 = 0x000;
const FRAME_DESCRIPTION: u32 = 0x040;
const FRAME_RECORD: u32 = 0x090;
const FRAME_NAME: u32 = 0x0a0;
const FRAME_POINTER: u32 = 0x0b0;
const FRAME_FORMAT_VALUE: u32 = 0x0c0;
const FRAME_RENDER_TARGET_SURFACE: u32 = 0x0c4;
const FRAME_STAGING_SURFACE: u32 = 0x0c8;
const FRAME_LOCKED_RECT: u32 = 0x0d0;
const FRAME_BMP_HEADER: u32 = 0x100;

/// The failure exit: logs `message`, destroys the smart pointer and the
/// scope, and returns 0.
fn fail(e: &mut Engine, message: u32, smart_pointer: u32, scope: u32) -> u32 {
    e.call(LOG, &args![message]);
    e.call(SMART_POINTER_DESTRUCT, &args![smart_pointer]);
    e.call(SCOPE_DESTRUCT, &args![scope]);
    0
}

/// Copies the 32-bit pixels at `source` (blue, green, red, alpha) into
/// `destination` as red, green, blue (three bytes per pixel), for
/// `width * height` pixels (the count is a signed 32-bit product).
fn convert_to_rgb(e: &mut Engine, source: u32, destination: u32, width: u32, height: u32) {
    let limit = (width as i32).wrapping_mul(height as i32).wrapping_mul(4);
    let mut offset = 0i32;
    let mut written = 0u32;
    while offset < limit {
        let pixel = source.wrapping_add(offset as u32);
        for channel in [2u32, 1, 0] {
            let byte = e.mem.u8(pixel.wrapping_add(channel));
            e.mem.set_u8(destination.wrapping_add(written), byte);
            written = written.wrapping_add(1);
        }
        offset = offset.wrapping_add(4);
    }
}

/// Writes the pixels of the image as the rows of a BMP (last row first,
/// red and blue swapped back) into `buffer`.
fn flip_rows(e: &mut Engine, pixels: u32, buffer: u32, width: u32, height: u32, stride: u32) {
    let mut written = 0u32;
    let mut row = (height as i32).wrapping_sub(1);
    while row >= 0 {
        let mut column = 0u32;
        while column < width {
            let index = width
                .wrapping_mul(stride)
                .wrapping_mul(row as u32)
                .wrapping_add(column.wrapping_mul(stride));
            for channel in [2u32, 1, 0] {
                let byte = e.mem.u8(pixels.wrapping_add(index).wrapping_add(channel));
                e.mem.set_u8(buffer.wrapping_add(written), byte);
                written = written.wrapping_add(1);
            }
            column = column.wrapping_add(1);
        }
        row = row.wrapping_sub(1);
    }
}

/// Writes the 54-byte BMP headers at `header` and the image to the open
/// file object `file`: `BM`, the file size, the offset to the pixels
/// (`0x36`), the 40-byte info header, the size, one plane, 24 bits, no
/// compression and the byte count; the headers first, then the pixel rows.
fn write_bmp(e: &mut Engine, file: u32, image: u32, header: u32, width: u32, height: u32) {
    let stride = e.call(IMAGE_BYTES_PER_PIXEL, &args![image]).u32();
    let pixels = e.call(IMAGE_PIXELS, &args![image, 0u32, 0u32]).u32();
    let byte_count = width.wrapping_mul(height).wrapping_mul(3);
    let buffer = e.call(ALLOCATE, &args![byte_count]).u32();
    flip_rows(e, pixels, buffer, width, height, stride);
    // The code overwrites the stride with the 3 bytes per pixel it writes.
    let stride = 3u32;
    e.mem.set_u16(header, 0x4d42);
    e.mem.set_u32(
        header + 2,
        width
            .wrapping_mul(height)
            .wrapping_mul(stride)
            .wrapping_add(BMP_HEADER_SIZE),
    );
    e.mem.set_u16(header + 6, 0);
    e.mem.set_u16(header + 8, 0);
    e.mem.set_u32(header + 10, BMP_HEADER_SIZE);
    e.mem.set_u32(header + 14, 0x28);
    e.mem.set_u32(header + 18, width);
    e.mem.set_u32(header + 22, height);
    e.mem.set_u16(header + 26, 1);
    e.mem.set_u16(header + 28, (stride << 3) as u16);
    e.mem.set_u32(header + 30, 0);
    e.mem
        .set_u32(header + 34, width.wrapping_mul(height).wrapping_mul(stride));
    for offset in [38, 42, 46, 50] {
        e.mem.set_u32(header + offset, 0);
    }
    e.call(FILE_WRITE, &args![file, header, BMP_HEADER_SIZE]);
    let size = width.wrapping_mul(height).wrapping_mul(stride);
    e.call(FILE_WRITE, &args![file, buffer, size]);
    e.call(FREE, &args![buffer]);
}

// Translated from 00878f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScreenShot::TakeSaveGameScreenShot` (Xbox PDB): cdecl with pointers to
/// the width and the height and a file path (0 for none); returns the image
/// object (`00a7c190`) or 0 when no render target or surface could be made.
///
/// A render target of the given size is made (`00b6b610`); the width is
/// scaled by `004dc200 / 004dc1f0` (a `float`, truncated by `_ftol2`) and
/// written back to `*width`; the settings objects are told (`0094ae40`,
/// `00879860` with `{0.0, 1.0, 1.0, 1 - ratio}`, `008706b0`, `008798a0`, the
/// rendering system's stages); a surface of height by width is created
/// (slot `0x90`), the render target is copied into it (slot `0x80`), locked
/// (slot `0x34`) and its 32-bit pixels converted into the image's 24-bit
/// ones; the surface is unlocked and released. When `path` is not 0 and
/// `0044a670` accepts it, the image is written there as a BMP through a
/// file object (`00b00260`), or the message at `01083e58` is logged when it
/// does not open.
// Translation incomplete: 00878f60 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn screenshot_take_save_game_screen_shot(
    e: &mut Engine,
    width: Ptr,
    height: Ptr,
    path: u32,
) -> u32 {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let base = frame.addr();
        let scope = base + FRAME_SCOPE;
        e.call(
            SCOPE_CONSTRUCT,
            &args![scope, 0x11u32, 1u32, SCOPE_TEXT, SCOPE_LINE],
        );

        let renderer = e.call(GET_RENDERER, &[]).u32();
        let rendering_system = e.call(GET_RENDERING_SYSTEM, &[]).u32();
        e.call(RENDERING_SYSTEM_BEGIN, &args![rendering_system]);
        let device = e.call(GET_DEVICE, &args![renderer]).u32();
        let description_owner = e.vcall(renderer, RENDERER_SLOT_DESCRIPTION, &[]).u32();
        let description = e.call(DESCRIPTION_GETTER, &args![description_owner]).u32();
        let bytes = e.mem.bytes(description, DESCRIPTION_SIZE);
        e.mem.write(base + FRAME_DESCRIPTION, &bytes);

        let record = base + FRAME_RECORD;
        e.call(RECORD_CONSTRUCT, &args![record]);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, 0);
        let name = base + FRAME_NAME;
        e.call(FIXED_STRING_CONSTRUCT, &args![name, RENDER_TARGET_NAME]);
        let requested_width = e.mem.u32(width.addr());
        let requested_height = e.mem.u32(height.addr());
        let target = e
            .call(
                CREATE_RENDER_TARGET,
                &args![
                    name,
                    requested_width,
                    requested_height,
                    record,
                    0u32,
                    1u32,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        let smart_pointer = base + FRAME_POINTER;
        e.call(SMART_POINTER_CONSTRUCT, &args![smart_pointer, target]);
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        if e.call(SMART_POINTER_GET, &args![smart_pointer]).u32() == 0 {
            return fail(e, MESSAGE_NO_RENDER_TARGET, smart_pointer, scope);
        }

        let numerator = e.call(SCREEN_NUMERATOR, &[]).i32();
        let denominator = e.call(SCREEN_DENOMINATOR, &[]).i32();
        let ratio = (numerator as f64 / denominator as f64) as f32;
        let width_now = e.mem.i32(width.addr());
        let scaled_width = e.call(FTOL, &args![width_now as f64 * ratio as f64]).u32();
        let player = e.global::<u32>(PLAYER_POINTER);
        e.call(PLAYER_UPDATE_CAMERA, &args![player, 0u32, 0u32]);
        let remaining = (1.0 - ratio as f64) as f32;
        let float_owner = e.call(GET_FLOAT_OWNER, &[]).u32();
        e.call(
            SET_FLOATS,
            &args![float_owner, 0.0f32, 1.0f32, 1.0f32, remaining],
        );
        let main_object = e.global::<u32>(MAIN_OBJECT_POINTER);
        let raw_target = e.call(SMART_POINTER_GET, &args![smart_pointer]).u32();
        e.call(
            MAIN_OBJECT_METHOD_8706B0,
            &args![main_object, raw_target, 0u32, 0u32],
        );
        let float_owner = e.call(GET_FLOAT_OWNER, &[]).u32();
        e.call(RESET_FLOATS, &args![float_owner]);
        if e.call(MAIN_OBJECT_TEST_5BB4D0, &args![main_object]).u8() != 0 {
            e.call(PLAYER_METHOD_9466D0, &args![player, 0.0f32, 1u32]);
        }
        let rendering_system = e.call(GET_RENDERING_SYSTEM, &[]).u32();
        e.call(
            RENDERING_SYSTEM_STAGE_A,
            &args![rendering_system, 0u32, 0x17u32],
        );
        let rendering_system = e.call(GET_RENDERING_SYSTEM, &[]).u32();
        e.call(
            RENDERING_SYSTEM_STAGE_B,
            &args![rendering_system, 1u32, 0x17u32],
        );
        let rendering_system = e.call(GET_RENDERING_SYSTEM, &[]).u32();
        e.call(RENDERING_SYSTEM_END, &args![rendering_system]);

        let raw_target = e.call(SMART_POINTER_GET, &args![smart_pointer]).u32();
        let texture = e
            .call(RENDER_TARGET_TEXTURE, &args![raw_target, 0u32])
            .u32();
        let owner = e.call(TEXTURE_OWNER, &args![texture]).u32();
        let inner = e.vcall(owner, OWNER_SLOT_A4, &[]).u32();
        let surface_device = e.vcall(inner, OWNER_SLOT_9C, &[]).u32();
        e.vcall(
            surface_device,
            DEVICE_SLOT_44,
            &args![0u32, base + FRAME_FORMAT_VALUE],
        );
        e.vcall(
            surface_device,
            DEVICE_SLOT_48,
            &args![0u32, base + FRAME_RENDER_TARGET_SURFACE],
        );
        let format_value = e.mem.u32(base + FRAME_FORMAT_VALUE);
        let current_width = e.mem.u32(width.addr());
        let current_height = e.mem.u32(height.addr());
        let failed = e
            .vcall(
                device,
                DEVICE_CREATE_SURFACE,
                &args![
                    current_height,
                    current_width,
                    format_value,
                    2u32,
                    base + FRAME_STAGING_SURFACE,
                    0u32
                ],
            )
            .u32();
        if failed != 0 {
            return fail(e, MESSAGE_SURFACE_FAILED, smart_pointer, scope);
        }
        let render_target_surface = e.mem.u32(base + FRAME_RENDER_TARGET_SURFACE);
        let staging_surface = e.mem.u32(base + FRAME_STAGING_SURFACE);
        e.vcall(
            device,
            DEVICE_COPY_SURFACE,
            &args![render_target_surface, staging_surface],
        );
        e.vcall(render_target_surface, RELEASE, &[]);
        let locked = base + FRAME_LOCKED_RECT;
        e.vcall(
            staging_surface,
            SURFACE_LOCK,
            &args![locked, 0u32, LOCK_FLAGS],
        );
        let bits = e.mem.u32(locked + 4);

        e.mem.set_u32(width.addr(), scaled_width);
        let block = e.call(OPERATOR_NEW, &args![IMAGE_SIZE]).u32();
        let image = if block != 0 {
            let image_width = e.mem.u32(width.addr());
            let image_height = e.mem.u32(height.addr());
            e.call(
                IMAGE_CONSTRUCT,
                &args![
                    block,
                    image_width,
                    image_height,
                    IMAGE_DESCRIPTION,
                    1u32,
                    1u32
                ],
            )
            .u32()
        } else {
            0
        };
        let destination = e.call(IMAGE_PIXELS, &args![image, 0u32, 0u32]).u32();
        let image_width = e.mem.u32(width.addr());
        let image_height = e.mem.u32(height.addr());
        convert_to_rgb(e, bits, destination, image_width, image_height);
        e.vcall(staging_surface, SURFACE_UNLOCK, &[]);
        e.vcall(staging_surface, RELEASE, &[]);
        e.call(SMART_POINTER_ASSIGN, &args![smart_pointer, 0u32]);

        if path != 0 && e.call(PATH_CHECK, &args![path]).u32() != 0 {
            e.call(PATH_PREPARE, &args![path]);
            let block = e.call(ALLOCATE, &args![FILE_SIZE]).u32();
            let file = if block != 0 {
                e.call(
                    FILE_CONSTRUCT,
                    &args![block, path, 1u32, FILE_BUFFER_SIZE, 0u32],
                )
                .u32()
            } else {
                0
            };
            if e.vcall(file, FILE_SLOT_IS_OPEN, &args![0u32, 0u32]).u8() != 0 {
                let file_width = e.mem.u32(width.addr());
                let file_height = e.mem.u32(height.addr());
                write_bmp(
                    e,
                    file,
                    image,
                    base + FRAME_BMP_HEADER,
                    file_width,
                    file_height,
                );
            } else {
                e.call(LOG, &args![MESSAGE_FILE_FAILED, path]);
            }
            if file != 0 {
                // The file object's deleting destructor (vtable slot 0).
                e.vcall(file, 0, &args![1u32]);
            }
        }
        e.call(SMART_POINTER_DESTRUCT, &args![smart_pointer]);
        e.call(SCOPE_DESTRUCT, &args![scope]);
        image
    })
}

// Translated from 00879860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the four `float`s at `+0x308`, `+0x30c`, `+0x310` and `+0x314` of
/// `this` (copied through the x87 stack, so a signalling NaN is stored
/// quiet).
pub fn fn_00879860(e: &mut Engine, this: Ptr, first: f32, second: f32, third: f32, fourth: f32) {
    for (offset, value) in [
        (0x308u32, first),
        (0x30c, second),
        (0x310, third),
        (0x314, fourth),
    ] {
        e.mem.set_u32(
            this.addr().wrapping_add(offset),
            x87_quiet_bits(value.to_bits()),
        );
    }
}

// Translated from 008798a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the four `float`s at `+0x308` to `+0x314` of `this` to 0.0, 1.0,
/// 1.0 and 0.0.
pub fn fn_008798a0(e: &mut Engine, this: Ptr) {
    fn_00879860(e, this, 0.0, 1.0, 1.0, 0.0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00878f60, screenshot_take_save_game_screen_shot(Ptr, Ptr, u32) -> u32),
        entry!(0x00879860, fn_00879860(Ptr, f32, f32, f32, f32)),
        entry!(0x008798a0, fn_008798a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    // Targets of the fake virtual tables.
    const SLOT_DESCRIPTION: u32 = 0x0d00_0001;
    const SLOT_CREATE_SURFACE: u32 = 0x0d00_0002;
    const SLOT_COPY_SURFACE: u32 = 0x0d00_0003;
    const SLOT_OWNER_A4: u32 = 0x0d00_0004;
    const SLOT_INNER_9C: u32 = 0x0d00_0005;
    const SLOT_44: u32 = 0x0d00_0006;
    const SLOT_48: u32 = 0x0d00_0007;
    const SLOT_RELEASE: u32 = 0x0d00_0008;
    const SLOT_LOCK: u32 = 0x0d00_0009;
    const SLOT_UNLOCK: u32 = 0x0d00_000a;
    const SLOT_FILE_DESTRUCT: u32 = 0x0d00_000b;
    const SLOT_FILE_OPEN: u32 = 0x0d00_000c;
    // The fake objects.
    const RENDERER: u32 = 0x0b00_0000;
    const DEVICE: u32 = 0x0b20_0000;
    const OWNER: u32 = 0x0b60_0000;
    const INNER: u32 = 0x0b61_0000;
    const SURFACE_DEVICE: u32 = 0x0b62_0000;
    const IMAGE: u32 = 0x0b70_0000;
    const FLOAT_OWNER: u32 = 0x0b40_0000;
    const RENDER_SURFACE: u32 = 0x0b80_0000;
    const STAGING_SURFACE: u32 = 0x0b81_0000;
    const FILE_OBJECT: u32 = 0x0b90_0000;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A double that returns `value` and records its arguments in `log`.
    fn recording(e: &mut Engine, log: &Log, addr: u32, value: u32) {
        let log = log.clone();
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            word_ret(value)
        });
    }

    /// An object with a vtable of `slots` (offset, target) at `address`.
    fn object(e: &mut Engine, address: u32, vtable: u32, slots: &[(u32, u32)]) {
        e.put_vtable(vtable, &[0; 0x40]);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        e.mem.map(address, 8);
        e.mem.set_u32(address, vtable);
    }

    struct Options {
        no_target: bool,
        surface_fails: bool,
        ratio: (u32, u32),
        path_accepted: bool,
        file_opens: bool,
        camera_test: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                no_target: false,
                surface_fails: false,
                ratio: (2, 1),
                path_accepted: true,
                file_opens: true,
                camera_test: false,
            }
        }
    }

    struct Scene {
        log: Log,
        width: u32,
        height: u32,
        pixels: u32,
        writes: Rc<RefCell<Vec<Vec<u8>>>>,
    }

    /// Every callee of `TakeSaveGameScreenShot` as a recording double.
    /// The locked surface holds a counting pattern, the image's pixel
    /// buffer is `pixels`, and `writes` collects the bytes written to
    /// the file.
    fn scene(options: Options) -> (Engine, Scene) {
        let mut e = Engine::new();
        for (addr, size) in [
            (PLAYER_POINTER, 4),
            (MAIN_OBJECT_POINTER, 4),
            (IMAGE_DESCRIPTION, DESCRIPTION_SIZE),
        ] {
            e.map(addr, size);
        }
        e.set_global(PLAYER_POINTER, 0x0a00_0000u32);
        e.set_global(MAIN_OBJECT_POINTER, 0x0a10_0000u32);
        let log: Log = Rc::default();
        for addr in [
            SCOPE_CONSTRUCT,
            SCOPE_DESTRUCT,
            RENDERING_SYSTEM_BEGIN,
            RENDERING_SYSTEM_STAGE_A,
            RENDERING_SYSTEM_STAGE_B,
            RENDERING_SYSTEM_END,
            RECORD_CONSTRUCT,
            FIXED_STRING_CONSTRUCT,
            FIXED_STRING_DESTRUCT,
            SMART_POINTER_CONSTRUCT,
            SMART_POINTER_ASSIGN,
            SMART_POINTER_DESTRUCT,
            LOG,
            PLAYER_UPDATE_CAMERA,
            PLAYER_METHOD_9466D0,
            MAIN_OBJECT_METHOD_8706B0,
            SET_FLOATS,
            RESET_FLOATS,
            PATH_PREPARE,
            FREE,
        ] {
            recording(&mut e, &log, addr, 0);
        }
        recording(&mut e, &log, GET_RENDERER, RENDERER);
        recording(&mut e, &log, GET_RENDERING_SYSTEM, 0x0b10_0000);
        recording(&mut e, &log, GET_DEVICE, DEVICE);
        recording(&mut e, &log, DESCRIPTION_GETTER, IMAGE_DESCRIPTION);
        recording(&mut e, &log, CREATE_RENDER_TARGET, 0x0b30_0000);
        let target = if options.no_target { 0 } else { 0x0b30_0000 };
        recording(&mut e, &log, SMART_POINTER_GET, target);
        recording(&mut e, &log, SCREEN_NUMERATOR, options.ratio.0);
        recording(&mut e, &log, SCREEN_DENOMINATOR, options.ratio.1);
        recording(&mut e, &log, GET_FLOAT_OWNER, FLOAT_OWNER);
        recording(
            &mut e,
            &log,
            MAIN_OBJECT_TEST_5BB4D0,
            u32::from(options.camera_test),
        );
        recording(&mut e, &log, RENDER_TARGET_TEXTURE, 0x0b50_0000);
        recording(&mut e, &log, TEXTURE_OWNER, OWNER);
        recording(&mut e, &log, OPERATOR_NEW, IMAGE);
        recording(&mut e, &log, IMAGE_CONSTRUCT, IMAGE);
        recording(&mut e, &log, IMAGE_BYTES_PER_PIXEL, 3);
        recording(&mut e, &log, PATH_CHECK, u32::from(options.path_accepted));
        recording(&mut e, &log, FILE_CONSTRUCT, FILE_OBJECT);
        let sink = log.clone();
        e.register_double(FTOL, move |_, a| {
            sink.borrow_mut().push((FTOL, a.to_vec()));
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word_ret(value as i32 as u32)
        });
        let sink = log.clone();
        e.register_double(ALLOCATE, move |e, a| {
            sink.borrow_mut().push((ALLOCATE, a.to_vec()));
            word_ret(e.mem.alloc(a[0].max(8)))
        });
        let writes: Rc<RefCell<Vec<Vec<u8>>>> = Rc::default();
        let sink = log.clone();
        let data = writes.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            sink.borrow_mut().push((FILE_WRITE, a.to_vec()));
            data.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });

        // The renderer: slot 0xd4 gives the description owner.
        object(&mut e, RENDERER, 0x0c00_0000, &[(0xd4, SLOT_DESCRIPTION)]);
        recording(&mut e, &log, SLOT_DESCRIPTION, 0x0b05_0000);
        // The device creates the staging surface (slot 0x90) and copies (0x80).
        object(
            &mut e,
            DEVICE,
            0x0c10_0000,
            &[(0x90, SLOT_CREATE_SURFACE), (0x80, SLOT_COPY_SURFACE)],
        );
        let fails = options.surface_fails;
        let sink = log.clone();
        e.register_double(SLOT_CREATE_SURFACE, move |e, a| {
            sink.borrow_mut().push((SLOT_CREATE_SURFACE, a.to_vec()));
            if !fails {
                e.mem.set_u32(a[5], STAGING_SURFACE);
            }
            word_ret(u32::from(fails))
        });
        recording(&mut e, &log, SLOT_COPY_SURFACE, 0);
        // The chain from the texture to the object with slots 0x44 and 0x48.
        object(&mut e, OWNER, 0x0c20_0000, &[(0xa4, SLOT_OWNER_A4)]);
        recording(&mut e, &log, SLOT_OWNER_A4, INNER);
        object(&mut e, INNER, 0x0c21_0000, &[(0x9c, SLOT_INNER_9C)]);
        recording(&mut e, &log, SLOT_INNER_9C, SURFACE_DEVICE);
        object(
            &mut e,
            SURFACE_DEVICE,
            0x0c22_0000,
            &[(0x44, SLOT_44), (0x48, SLOT_48)],
        );
        let sink = log.clone();
        e.register_double(SLOT_44, move |e, a| {
            sink.borrow_mut().push((SLOT_44, a.to_vec()));
            e.mem.set_u32(a[2], 0x16);
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(SLOT_48, move |e, a| {
            sink.borrow_mut().push((SLOT_48, a.to_vec()));
            e.mem.set_u32(a[2], RENDER_SURFACE);
            Ret::default()
        });
        // Both surfaces share one vtable: release, lock and unlock.
        let slots = [
            (8, SLOT_RELEASE),
            (SURFACE_LOCK, SLOT_LOCK),
            (SURFACE_UNLOCK, SLOT_UNLOCK),
        ];
        object(&mut e, RENDER_SURFACE, 0x0c30_0000, &slots);
        e.mem.map(STAGING_SURFACE, 8);
        e.mem.set_u32(STAGING_SURFACE, 0x0c30_0000);
        recording(&mut e, &log, SLOT_RELEASE, 0);
        recording(&mut e, &log, SLOT_UNLOCK, 0);
        let bits = e.mem.alloc(256);
        for i in 0..256u32 {
            e.mem.set_u8(bits + i, i as u8);
        }
        let sink = log.clone();
        e.register_double(SLOT_LOCK, move |e, a| {
            sink.borrow_mut().push((SLOT_LOCK, a.to_vec()));
            e.mem.set_u32(a[1] + 4, bits);
            Ret::default()
        });
        // The file object: slot 0 is the deleting destructor, 0x20 says it opened.
        object(
            &mut e,
            FILE_OBJECT,
            0x0c40_0000,
            &[(0, SLOT_FILE_DESTRUCT), (FILE_SLOT_IS_OPEN, SLOT_FILE_OPEN)],
        );
        recording(&mut e, &log, SLOT_FILE_DESTRUCT, 0);
        recording(&mut e, &log, SLOT_FILE_OPEN, u32::from(options.file_opens));
        // The image's pixel buffer, and the pointers to the width and height.
        let pixels = e.mem.alloc(256);
        recording(&mut e, &log, IMAGE_PIXELS, pixels);
        let width = e.mem.alloc(8);
        e.mem.set_u32(width, 4);
        e.mem.set_u32(width + 4, 2);
        (
            e,
            Scene {
                log,
                width,
                height: width + 4,
                pixels,
                writes,
            },
        )
    }

    fn addresses(scene: &Scene) -> Vec<u32> {
        scene.log.borrow().iter().map(|call| call.0).collect()
    }

    fn position(scene: &Scene, address: u32) -> usize {
        addresses(scene)
            .iter()
            .position(|a| *a == address)
            .unwrap_or_else(|| panic!("{address:#x} was not called"))
    }

    fn arguments(scene: &Scene, address: u32) -> Vec<Vec<u32>> {
        scene
            .log
            .borrow()
            .iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn floats_are_set_and_reset() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x320);
        e.call(
            0x0087_9860,
            &args![object, 1.5f32, -2.0f32, 3.0f32, 4.25f32],
        );
        assert_eq!(e.mem.f32(object + 0x308), 1.5);
        assert_eq!(e.mem.f32(object + 0x30c), -2.0);
        assert_eq!(e.mem.f32(object + 0x310), 3.0);
        assert_eq!(e.mem.f32(object + 0x314), 4.25);
        e.call(0x0087_98a0, &[object]);
        assert_eq!(e.mem.f32(object + 0x308), 0.0);
        assert_eq!(e.mem.f32(object + 0x30c), 1.0);
        assert_eq!(e.mem.f32(object + 0x310), 1.0);
        assert_eq!(e.mem.f32(object + 0x314), 0.0);
    }

    #[test]
    fn signalling_nans_are_stored_quiet() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x320);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x0087_9860,
            &args![object, signalling, 0.0f32, 0.0f32, 0.0f32],
        );
        assert_eq!(e.mem.u32(object + 0x308), 0x7fc0_0001);
    }

    #[test]
    fn screenshot_scales_the_width_and_converts_the_pixels() {
        let (mut e, s) = scene(Options::default());
        let image = e.call(0x0087_8f60, &[s.width, s.height, 0]).u32();
        assert_eq!(image, IMAGE);
        // 4 * 2.0 = 8: written back to the width.
        assert_eq!(e.mem.u32(s.width), 8);
        assert_eq!(e.mem.u32(s.height), 2);
        let ftol = arguments(&s, FTOL);
        assert_eq!(ftol.len(), 1);
        let value = f64::from_bits(u64::from(ftol[0][0]) | u64::from(ftol[0][1]) << 32);
        assert_eq!(value, 8.0);
        // The render target gets the requested size, not the scaled one.
        let target = &arguments(&s, CREATE_RENDER_TARGET)[0];
        assert_eq!(target[1], 4);
        assert_eq!(target[2], 2);
        assert_eq!(target[4..], [0, 1, 0, 0, 0]);
        // The four floats: 0.0, 1.0, 1.0 and 1 - ratio.
        assert_eq!(
            arguments(&s, SET_FLOATS)[0],
            vec![
                FLOAT_OWNER,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                (-1.0f32).to_bits()
            ]
        );
        assert_eq!(arguments(&s, RESET_FLOATS)[0], vec![FLOAT_OWNER]);
        // The surface is made of the height by the (not yet scaled) width and the
        // format value.
        let create = &arguments(&s, SLOT_CREATE_SURFACE)[0];
        assert_eq!(create[..5], [DEVICE, 2, 4, 0x16, 2]);
        // The image is made with the scaled width.
        assert_eq!(
            arguments(&s, IMAGE_CONSTRUCT)[0],
            vec![IMAGE, 8, 2, IMAGE_DESCRIPTION, 1, 1]
        );
        // Blue, green, red, alpha become red, green, blue.
        assert_eq!(e.mem.bytes(s.pixels, 6), vec![2, 1, 0, 6, 5, 4]);
        assert_eq!(e.mem.u8(s.pixels + 47), 60);
        // The order of the main steps.
        let order = [
            SLOT_CREATE_SURFACE,
            SLOT_COPY_SURFACE,
            SLOT_LOCK,
            IMAGE_CONSTRUCT,
            SLOT_UNLOCK,
        ];
        let positions: Vec<usize> = order.iter().map(|a| position(&s, *a)).collect();
        assert!(positions.windows(2).all(|w| w[0] < w[1]), "{positions:?}");
        assert_eq!(addresses(&s).last(), Some(&SCOPE_DESTRUCT));
        // No path: no file.
        assert!(!addresses(&s).contains(&FILE_CONSTRUCT));
    }

    #[test]
    fn camera_method_runs_only_when_the_main_object_asks() {
        let (mut e, s) = scene(Options::default());
        e.call(0x0087_8f60, &[s.width, s.height, 0]);
        assert!(arguments(&s, PLAYER_METHOD_9466D0).is_empty());
        assert_eq!(
            arguments(&s, PLAYER_UPDATE_CAMERA)[0],
            vec![0x0a00_0000, 0, 0]
        );
        assert_eq!(
            arguments(&s, MAIN_OBJECT_METHOD_8706B0)[0],
            vec![0x0a10_0000, 0x0b30_0000, 0, 0]
        );
        let (mut e, s) = scene(Options {
            camera_test: true,
            ..Options::default()
        });
        e.call(0x0087_8f60, &[s.width, s.height, 0]);
        assert_eq!(
            arguments(&s, PLAYER_METHOD_9466D0)[0],
            vec![0x0a00_0000, 0.0f32.to_bits(), 1]
        );
    }

    #[test]
    fn missing_render_target_logs_and_returns_null() {
        let (mut e, s) = scene(Options {
            no_target: true,
            ..Options::default()
        });
        assert_eq!(e.call(0x0087_8f60, &[s.width, s.height, 0]).u32(), 0);
        let calls = s.log.borrow();
        let tail: Vec<u32> = calls[calls.len() - 3..].iter().map(|c| c.0).collect();
        assert_eq!(tail, vec![LOG, SMART_POINTER_DESTRUCT, SCOPE_DESTRUCT]);
        assert_eq!(calls[calls.len() - 3].1, vec![MESSAGE_NO_RENDER_TARGET]);
        drop(calls);
        assert!(!addresses(&s).contains(&FTOL));
        assert_eq!(e.mem.u32(s.width), 4);
    }

    #[test]
    fn failing_surface_logs_and_returns_null_before_copying() {
        let (mut e, s) = scene(Options {
            surface_fails: true,
            ..Options::default()
        });
        assert_eq!(e.call(0x0087_8f60, &[s.width, s.height, 0]).u32(), 0);
        let calls = s.log.borrow();
        let tail: Vec<u32> = calls[calls.len() - 3..].iter().map(|c| c.0).collect();
        assert_eq!(tail, vec![LOG, SMART_POINTER_DESTRUCT, SCOPE_DESTRUCT]);
        assert_eq!(calls[calls.len() - 3].1, vec![MESSAGE_SURFACE_FAILED]);
        drop(calls);
        assert!(!addresses(&s).contains(&SLOT_COPY_SURFACE));
        // The width is only written back after the surface exists.
        assert_eq!(e.mem.u32(s.width), 4);
    }

    #[test]
    fn a_path_writes_a_bmp_with_the_last_row_first() {
        let (mut e, s) = scene(Options::default());
        let path = e.mem.alloc(8);
        assert_eq!(e.call(0x0087_8f60, &[s.width, s.height, path]).u32(), IMAGE);
        let constructed = arguments(&s, FILE_CONSTRUCT);
        assert_eq!(constructed[0][1..], [path, 1, 0x4000, 0]);
        assert_eq!(arguments(&s, PATH_PREPARE)[0], vec![path]);
        let writes = s.writes.borrow();
        assert_eq!(writes.len(), 2);
        // 8 x 2 pixels, 3 bytes each.
        let header = &writes[0];
        assert_eq!(header.len(), 0x36);
        assert_eq!(&header[0..2], b"BM");
        let word = |at: usize| u32::from_le_bytes(header[at..at + 4].try_into().unwrap());
        assert_eq!(word(2), 48 + 0x36);
        assert_eq!(word(10), 0x36);
        assert_eq!(word(14), 0x28);
        assert_eq!(word(18), 8);
        assert_eq!(word(22), 2);
        assert_eq!(u16::from_le_bytes(header[26..28].try_into().unwrap()), 1);
        assert_eq!(u16::from_le_bytes(header[28..30].try_into().unwrap()), 24);
        assert_eq!(word(34), 48);
        assert!(header[38..].iter().all(|b| *b == 0));
        // The first pixel written is the first of the second row, with red
        // and blue swapped back.
        let body = &writes[1];
        assert_eq!(body.len(), 48);
        let second_row = 8 * 3;
        let pixels = e.mem.bytes(s.pixels, 48);
        assert_eq!(body[0], pixels[second_row + 2]);
        assert_eq!(body[1], pixels[second_row + 1]);
        assert_eq!(body[2], pixels[second_row]);
        assert_eq!(body[24], pixels[2]);
        assert_eq!(arguments(&s, SLOT_FILE_DESTRUCT)[0], vec![FILE_OBJECT, 1]);
        assert_eq!(arguments(&s, FREE).len(), 1);
    }

    #[test]
    fn an_unopened_file_is_logged_and_still_destroyed() {
        let (mut e, s) = scene(Options {
            file_opens: false,
            ..Options::default()
        });
        let path = e.mem.alloc(8);
        e.call(0x0087_8f60, &[s.width, s.height, path]);
        assert!(s.writes.borrow().is_empty());
        assert_eq!(arguments(&s, LOG)[0], vec![MESSAGE_FILE_FAILED, path]);
        assert_eq!(arguments(&s, SLOT_FILE_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_refused_path_makes_no_file() {
        let (mut e, s) = scene(Options {
            path_accepted: false,
            ..Options::default()
        });
        let path = e.mem.alloc(8);
        assert_eq!(e.call(0x0087_8f60, &[s.width, s.height, path]).u32(), IMAGE);
        assert!(!addresses(&s).contains(&PATH_PREPARE));
        assert!(!addresses(&s).contains(&FILE_CONSTRUCT));
    }

    #[test]
    fn pixel_conversion_follows_the_signed_pixel_count() {
        let mut e = Engine::new();
        let source = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        for i in 0..16u32 {
            e.mem.set_u8(source + i, 0x10 + i as u8);
        }
        // Negative counts convert nothing.
        convert_to_rgb(&mut e, source, target, u32::MAX, 3);
        assert_eq!(e.mem.bytes(target, 3), vec![0, 0, 0]);
        convert_to_rgb(&mut e, source, target, 2, 1);
        assert_eq!(
            e.mem.bytes(target, 6),
            vec![0x12, 0x11, 0x10, 0x16, 0x15, 0x14]
        );
    }
}

//! `NiMain/niimageconverter.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_increment, ALLOCATE};

/// `011f4a7c`: the pointer `00a76ab0` stores the object it builds in.
const IMAGE_CONVERTER_SLOT: u32 = 0x011f_4a7c;
/// `011f4400`: a counted word `00a76ab0` increments for the object it builds.
const IMAGE_CONVERTER_COUNT: u32 = 0x011f_4400;
/// `00a8ae50` (`thiscall`, engine map unit `ninifimagereader.cpp`): constructs the part of
/// the object at `+0x80`.
const PART_CONSTRUCTOR: u32 = 0x00a8_ae50;

// Translated from 00a76ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niimageconverter.cpp`; run by the start-up `00a89cc0`,
/// no arguments): allocates 0x780 bytes (`00aa13e0`), sets the vtable `0101dce4` and the
/// word at `+4` to 0, counts `011f4400`, sets the vtable `0109d9a4`, constructs the part
/// at `+0x80` with `00a8ae50` and stores the object, counted, in `011f4a7c`, releasing the
/// object held there before. A failed allocation stores null.
pub fn fn_00a76ab0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x780u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, 0x0101_dce4);
        e.mem.set_u32(block + 4, 0);
        interlocked_increment(e, IMAGE_CONVERTER_COUNT);
        e.mem.set_u32(block, 0x0109_d9a4);
        e.call(PART_CONSTRUCTOR, &args![block + 0x80]);
        block
    };
    assign_ref_counted(e, IMAGE_CONVERTER_SLOT, object);
}

use crate::units::nimain::nimainsdm::{
    interlocked_decrement, EMPTY_FIXED_STRING, GET_FIXED_STRING,
};

/// The platform subdirectory string (`011f4a78`, a heap C string or null) the file-name
/// conversions put into names.
const PLATFORM_DIRECTORY: u32 = 0x011f_4a78;
/// `NiRefObject`'s and `NiImageConverter`'s virtual tables (RTTI `.?AVNiRefObject@@`,
/// `.?AVNiImageConverter@@`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_IMAGE_CONVERTER_VTABLE: u32 = 0x0109_d9a4;
/// The `NiNIFImageReader` part of the converter (+0x80) and its constructor (`00a8ae50`) and
/// destructor (`00a8ada0`) bodies (`ninifimagereader.cpp`, `__thiscall`).
const READER_PART: u32 = 0x80;
const READER_DESTRUCTOR: u32 = 0x00a8_ada0;
/// `NiNIFImageReader` sized as the converter's deleting destructor frees it.
const CONVERTER_SIZE: u32 = 0x780;
/// `NiFilename` (a stack object of 0x404 bytes): its directory part at +0x303 (0x100 bytes)
/// that the platform subdirectory is copied into.
const FILENAME_SIZE: u32 = 0x404;
const FILENAME_PLATFORM_PART: u32 = 0x303;
/// `NiPixelData` fields (Xbox PDB): `m_kPixelFormat` +0x08, `m_spPalette` +0x4c, `m_pucPixels`
/// +0x50, `m_puiWidth` +0x54, `m_puiHeight` +0x58, `m_puiOffsetInBytes` +0x5c,
/// `m_uiMipmapLevels` +0x60.
const PIXEL_DATA_FORMAT: u32 = 0x08;
const PIXEL_DATA_PALETTE: u32 = 0x4c;
const PIXEL_DATA_PIXELS: u32 = 0x50;
const PIXEL_DATA_WIDTHS: u32 = 0x54;
const PIXEL_DATA_HEIGHTS: u32 = 0x58;
const PIXEL_DATA_OFFSETS: u32 = 0x5c;
const PIXEL_DATA_LEVELS: u32 = 0x60;

// Callees outside this unit.
/// `_strcpy_s` (`00ec65a6`, cdecl: buffer, size, source) and `_strncpy_s` (`00ec8d5f`,
/// cdecl: buffer, size, source, count).
const STRCPY_S: u32 = 0x00ec_65a6;
const STRNCPY_S: u32 = 0x00ec_8d5f;
/// `NiFilename::NiFilename` (`00aa27d0`, `__thiscall`, one word: the path) and its full-path
/// writer `00aa29d0` (`__thiscall`, two words: buffer, size; a thunk).
const FILENAME_CONSTRUCTOR: u32 = 0x00aa_27d0;
const FILENAME_FULL_PATH: u32 = 0x00aa_29d0;
/// `00aa1070` (cdecl allocate), `00aa10f0` (cdecl free), `00aa13e0` (cdecl object
/// allocation), `00aa1460` (cdecl sized delete).
const ALLOCATE_BYTES: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiPixelFormat::operator==` (`00a76540`) and the other comparison `00a765c0` (map name
/// `operator__ov2`; it answers true when the formats differ, as `ConvertPixelData` uses it),
/// `__thiscall`, one word: the other format.
const FORMAT_EQUAL: u32 = 0x00a7_6540;
const FORMAT_DIFFERENT: u32 = 0x00a7_65c0;
/// `NiPixelData::NiPixelData` (`00a7c030`, the copy constructor, `__thiscall`, one word),
/// `NiPalette::NiPalette` (`00a8b080`, copy, one word), `NiPalette::operator=` (`00a8b190`,
/// one word) and the pixel data's palette setter `00a76a60` (`__thiscall`, one word).
const PIXEL_DATA_COPY: u32 = 0x00a7_c030;
const PALETTE_COPY: u32 = 0x00a8_b080;
const PALETTE_ASSIGN: u32 = 0x00a8_b190;
const SET_PALETTE: u32 = 0x00a7_6a60;
/// `_memcpy` (`00ec44d0`, cdecl, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// `InterlockedIncrement`/`InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT_SLOT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT_SLOT: u32 = 0x00fd_f1c4;

/// A fixed string's reference given up (nothing for the empty handle).
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT_SLOT, &args![handle.wrapping_sub(8)]);
    }
}

/// `*slot = fixed string of buffer` as the code inlines it: nothing when the handle is the
/// buffer's address itself.
fn assign_from_buffer(e: &mut Engine, slot: u32, buffer: u32) {
    let old = e.mem.u32(slot);
    if old == buffer {
        return;
    }
    let handle = e.call(GET_FIXED_STRING, &args![buffer]).u32();
    e.mem.set_u32(slot, handle);
    release_fixed_string(e, old);
}

// Translated from 00a76640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiImageConverter::GetImageConverter` (Xbox PDB): the converter in `011f4a7c`.
pub fn ni_image_converter_get_image_converter(e: &mut Engine) -> Ptr {
    Ptr::new(e.mem.u32(IMAGE_CONVERTER_SLOT))
}

// Translated from 00a76650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiImageConverter` constructor (unnamed in the map): `NiRefObject`'s table and a count
/// of 0, the object count (`011f4400`) up, the class's table and its `NiNIFImageReader` part
/// (`00a8ae50` on +0x80). Answers `this`.
pub fn fn_00a76650(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(INTERLOCKED_INCREMENT_SLOT, &args![IMAGE_CONVERTER_COUNT]);
    e.mem.set_u32(base, NI_IMAGE_CONVERTER_VTABLE);
    e.call(PART_CONSTRUCTOR, &args![base + READER_PART]);
    this
}

/// The destructor body as both destructors have it.
fn destroy(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, NI_IMAGE_CONVERTER_VTABLE);
    let directory = e.mem.u32(PLATFORM_DIRECTORY);
    if directory != 0 {
        e.call(FREE, &args![directory]);
    }
    e.mem.set_u32(PLATFORM_DIRECTORY, 0);
    e.call(READER_DESTRUCTOR, &args![base + READER_PART]);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    interlocked_decrement(e, IMAGE_CONVERTER_COUNT);
}

// Translated from 00a76680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiImageConverter` destructor body (unnamed in the map): the class's table, the
/// platform subdirectory freed (`00aa10f0`) and cleared, the reader part destroyed
/// (`00a8ada0`), then `NiRefObject`'s table and the object count down.
pub fn fn_00a76680(e: &mut Engine, this: Ptr) {
    destroy(e, this.addr());
}

// Translated from 00a76890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiImageConverter::CanConvertPixelData` (Xbox PDB; `__thiscall`, two words): whether the
/// two pixel formats are equal (`NiPixelFormat::operator==` on the first with the second; the
/// converter itself is not read).
pub fn ni_image_converter_can_convert_pixel_data(
    e: &mut Engine,
    _this: Ptr,
    source: Ptr,
    destination: Ptr,
) -> bool {
    e.call(FORMAT_EQUAL, &args![source, destination]).bool()
}

// Translated from 00a768b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a768b0 - the /GS stack cookie (stored at frame +0x404 and checked through 00ec408c) is not translated
/// `NiImageConverter::ConvertFilenameToPlatformSpecific` (Xbox PDB; cdecl, one word: a C
/// string): a new 0x104-byte buffer (`00aa1070`). With a platform subdirectory the name is
/// split (`NiFilename` on the stack), the subdirectory copied into it (`_strcpy_s`, 0x100
/// bytes at +0x303) and the full path written to the buffer (`00aa29d0`); otherwise the name
/// is copied (`_strncpy_s` with its length). Answers the buffer.
pub fn ni_image_converter_convert_filename_to_platform_specific(e: &mut Engine, name: Ptr) -> Ptr {
    let buffer = e.call(ALLOCATE_BYTES, &args![0x104u32]).u32();
    let directory = e.mem.u32(PLATFORM_DIRECTORY);
    if directory != 0 {
        e.with_stack(0x408, |e, filename| {
            let filename = filename.addr();
            e.call(FILENAME_CONSTRUCTOR, &args![filename, name]);
            let directory = e.mem.u32(PLATFORM_DIRECTORY);
            e.call(
                STRCPY_S,
                &args![filename + FILENAME_PLATFORM_PART, 0x100u32, directory],
            );
            e.call(FILENAME_FULL_PATH, &args![filename, buffer, 0x104u32]);
        });
    } else {
        let mut length = 0u32;
        while e.mem.u8(name.addr().wrapping_add(length)) != 0 {
            length += 1;
        }
        e.call(STRNCPY_S, &args![buffer, 0x104u32, name, length]);
    }
    Ptr::new(buffer)
}

// Translated from 00a76960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a76960 - the /GS stack cookie (stored at frame +0x508 and checked through 00ec408c) is not translated
/// `NiImageConverter::ConvertFilenameToPlatformSpecific` (Xbox PDB `_ov2`; cdecl, two words:
/// source and destination `NiFixedString`s). With a platform subdirectory, as the C-string
/// form with the full path in a 0x104-byte stack buffer that then becomes the destination
/// (a new handle, the old one's reference given up). Without one the destination takes the
/// source handle (a reference on it, the old one's given up).
pub fn ni_image_converter_convert_filename_to_platform_specific_ov2(
    e: &mut Engine,
    source: Ptr,
    destination: Ptr,
) {
    let target = destination.addr();
    if e.mem.u32(PLATFORM_DIRECTORY) != 0 {
        e.with_stack(0x50c, |e, frame| {
            let filename = frame.addr();
            let buffer = filename + FILENAME_SIZE;
            let name = e.mem.u32(source.addr());
            e.call(FILENAME_CONSTRUCTOR, &args![filename, name]);
            let directory = e.mem.u32(PLATFORM_DIRECTORY);
            e.call(
                STRCPY_S,
                &args![filename + FILENAME_PLATFORM_PART, 0x100u32, directory],
            );
            e.call(FILENAME_FULL_PATH, &args![filename, buffer, 0x104u32]);
            assign_from_buffer(e, target, buffer);
        });
        return;
    }
    let handle = e.mem.u32(source.addr());
    if e.mem.u32(target) == handle {
        return;
    }
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT_SLOT, &args![handle.wrapping_sub(8)]);
    }
    let old = e.mem.u32(target);
    release_fixed_string(e, old);
    e.mem.set_u32(target, handle);
}

// Translated from 00a76b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): gives up the converter in `011f4a7c` (deleted at its last reference)
/// and clears it.
pub fn fn_00a76b30(e: &mut Engine) {
    let converter = e.mem.u32(IMAGE_CONVERTER_SLOT);
    if converter == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT_SLOT, &args![converter + 4])
        .u32()
        == 0
    {
        e.vcall(converter, 4, &args![]);
    }
    e.mem.set_u32(IMAGE_CONVERTER_SLOT, 0);
}

// Translated from 00a76b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word; the shape of `NiImageConverter::SetImageConverter`): the
/// converter in `011f4a7c` takes the new one as an `NiPointer` does.
pub fn fn_00a76b60(e: &mut Engine, converter: Ptr) {
    let old = e.mem.u32(IMAGE_CONVERTER_SLOT);
    if old == converter.addr() {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT_SLOT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &args![]);
    }
    e.mem.set_u32(IMAGE_CONVERTER_SLOT, converter.addr());
    if !converter.is_null() {
        e.call(INTERLOCKED_INCREMENT_SLOT, &args![converter.addr() + 4]);
    }
}

// Translated from 00a76bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiImageConverter::_scalar_deleting_destructor_` (Xbox PDB): the destructor body as
/// [`fn_00a76680`] inlines it, then the sized delete of 0x780 bytes when bit 0 of `flags` is
/// set. Answers `this`.
pub fn ni_image_converter_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    destroy(e, this.addr());
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, CONVERTER_SIZE]);
    }
    this
}

// Translated from 00a76c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiImageConverter::ConvertPixelData` (Xbox PDB; `__thiscall`, four words: source pixel
/// data, destination format, destination pixel data or null, mipmap flag (not read)): null
/// when the source format differs from the destination format (`00a765c0`). A destination
/// that is the source is answered as it is. A destination of that format, the same level-0
/// sides and no more levels than the source gets every level copied (`_memcpy` between the
/// two buffers by their offsets) and the source's palette (assigned to its palette, or a copy
/// made with `NiPalette::NiPalette` and set through `00a76a60`; a failed allocation sets
/// null). Otherwise a copy of the source (`NiPixelData::NiPixelData`) is made (null when its
/// allocation fails). Answers the destination.
pub fn ni_image_converter_convert_pixel_data(
    e: &mut Engine,
    _this: Ptr,
    source: Ptr,
    format: Ptr,
    destination: Ptr,
    _unused_4: u32,
) -> Ptr {
    let source = source.addr();
    let destination = destination.addr();
    if e.call(FORMAT_DIFFERENT, &args![source + PIXEL_DATA_FORMAT, format])
        .bool()
    {
        return Ptr::new(0);
    }
    let reuse = destination != 0
        && (destination == source
            || (e
                .call(
                    FORMAT_EQUAL,
                    &args![destination + PIXEL_DATA_FORMAT, format],
                )
                .bool()
                && e.mem.u32(e.mem.u32(destination + PIXEL_DATA_WIDTHS))
                    == e.mem.u32(e.mem.u32(source + PIXEL_DATA_WIDTHS))
                && e.mem.u32(e.mem.u32(destination + PIXEL_DATA_HEIGHTS))
                    == e.mem.u32(e.mem.u32(source + PIXEL_DATA_HEIGHTS))
                && e.mem.u32(destination + PIXEL_DATA_LEVELS)
                    <= e.mem.u32(source + PIXEL_DATA_LEVELS)));
    if !reuse {
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        if memory == 0 {
            return Ptr::new(0);
        }
        return e.call(PIXEL_DATA_COPY, &args![memory, source]).ptr();
    }
    if destination == source {
        return Ptr::new(destination);
    }
    let levels = e.mem.u32(destination + PIXEL_DATA_LEVELS);
    for level in 0..levels {
        let source_offsets = e.mem.u32(source + PIXEL_DATA_OFFSETS);
        let bytes = e
            .mem
            .u32(source_offsets + 4 * level + 4)
            .wrapping_sub(e.mem.u32(source_offsets + 4 * level));
        let from = e
            .mem
            .u32(source + PIXEL_DATA_PIXELS)
            .wrapping_add(e.mem.u32(source_offsets + 4 * level));
        let destination_offsets = e.mem.u32(destination + PIXEL_DATA_OFFSETS);
        let to = e
            .mem
            .u32(destination_offsets + 4 * level)
            .wrapping_add(e.mem.u32(destination + PIXEL_DATA_PIXELS));
        e.call(MEMCPY, &args![to, from, bytes]);
    }
    let palette = e.mem.u32(source + PIXEL_DATA_PALETTE);
    if palette == 0 {
        return Ptr::new(destination);
    }
    let own = e.mem.u32(destination + PIXEL_DATA_PALETTE);
    if own != 0 {
        e.call(PALETTE_ASSIGN, &args![own, palette]);
        return Ptr::new(destination);
    }
    let memory = e.call(NEW_OBJECT, &args![0x24u32]).u32();
    let copy = if memory != 0 {
        let palette = e.mem.u32(source + PIXEL_DATA_PALETTE);
        e.call(PALETTE_COPY, &args![memory, palette]).u32()
    } else {
        0
    };
    e.call(SET_PALETTE, &args![destination, copy]);
    Ptr::new(destination)
}

/// `NiFile::GetFile` (`00aa1a20`, `cdecl`: path, mode, buffer size), answering the opened
/// file or null.
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;

/// Runs `body` with a stack `NiFilename` (0x404 bytes) built from `path`, then asks the
/// reader part (+0x80, its virtual slot +4) whether it reads the name's extension
/// (`NiFilename` +0x103); `body` runs only on a yes, with the reader.
fn with_reader<T>(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    no: T,
    body: impl FnOnce(&mut Engine, u32) -> T,
) -> T {
    e.with_stack(FILENAME_SIZE, |e, filename| {
        e.call(FILENAME_CONSTRUCTOR, &args![filename, path]);
        let reader = this.addr() + READER_PART;
        if !e.vcall(reader, 4, &args![filename.addr() + 0x103]).bool() {
            return no;
        }
        body(e, reader)
    })
}

// Translated from 00a766d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a766d0 - the /GS cookie check (00ec408c) guarding the stack NiFilename is not modeled
/// `NiImageConverter::ReadImageFileInfo` (Xbox PDB; `thiscall`, six words: the path, then
/// the addresses the format, mipmap flag, width, height and face count go to): false when
/// the reader part (+0x80) does not take the name's extension, when `NiFile::GetFile`
/// (mode 0, 0x8000 bytes) gives no file, or when the file's slot +4 says it is not good;
/// otherwise the reader's slot +0xc reads the header (file, width, height, format,
/// mipmap, faces). The file is deleted (its slot 0 with 1) once opened. Answers what the
/// reader answered.
#[allow(clippy::too_many_arguments)]
pub fn ni_image_converter_read_image_file_info(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    format: Ptr,
    mipmap: Ptr,
    width: Ptr,
    height: Ptr,
    faces: Ptr,
) -> bool {
    with_reader(e, this, path, false, |e, reader| {
        let file = e
            .call(NI_FILE_GET_FILE, &args![path, 0u32, 0x8000u32])
            .u32();
        if file == 0 {
            return false;
        }
        if !e.vcall(file, 4, &args![]).bool() {
            e.vcall(file, 0, &args![1u32]);
            return false;
        }
        let read = e
            .vcall(
                reader,
                0xc,
                &args![file, width, height, format, mipmap, faces],
            )
            .bool();
        e.vcall(file, 0, &args![1u32]);
        read
    })
}

// Translated from 00a767c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a767c0 - the /GS cookie check (00ec408c) guarding the stack NiFilename is not modeled
/// `NiImageConverter::ReadImageFile` (Xbox PDB; `thiscall`, three words: the path, the
/// pixel data to read into, a file or null): when the reader part (+0x80) does not take the
/// name's extension, a given file is deleted (slot 0 with 1) and null answered. Otherwise
/// without a file one is opened (`NiFile::GetFile`, mode 0, 0x8000 bytes; null answers
/// null); a file whose slot +4 says it is not good is deleted and null answered; else the
/// reader's slot +8 reads the pixel data (file, the pixel data), the file is deleted and
/// the reader's answer returned (null stays null).
pub fn ni_image_converter_read_image_file(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    pixel_data: Ptr,
    file: Ptr,
) -> Ptr {
    let given = file.addr();
    let read = with_reader(e, this, path, None, |e, reader| {
        let mut file = given;
        if file == 0 {
            file = e
                .call(NI_FILE_GET_FILE, &args![path, 0u32, 0x8000u32])
                .u32();
            if file == 0 {
                return Some(0);
            }
        }
        if !e.vcall(file, 4, &args![]).bool() {
            e.vcall(file, 0, &args![1u32]);
            return Some(0);
        }
        let data = e.vcall(reader, 8, &args![file, pixel_data]).u32();
        e.vcall(file, 0, &args![1u32]);
        Some(data)
    });
    match read {
        Some(data) => Ptr::new(data),
        None => {
            if given != 0 {
                e.vcall(given, 0, &args![1u32]);
            }
            Ptr::new(0)
        }
    }
}

// Translated from 00a76a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niimageconverter.cpp`; `thiscall`, one word); by its
/// body the pixel data's palette setter: the counted assignment of the palette to
/// `m_spPalette` (+0x4c; nothing when it is already there, the old one released and freed
/// through its slot +4 at a count of 0, the new one counted).
pub fn fn_00a76a60(e: &mut Engine, this: Ptr, palette: Ptr) {
    assign_ref_counted(e, this.addr() + PIXEL_DATA_PALETTE, palette.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_6ab0, fn_00a76ab0()),
        entry!(0x00a76640, ni_image_converter_get_image_converter() -> Ptr),
        entry!(0x00a76650, fn_00a76650(Ptr) -> Ptr),
        entry!(0x00a76680, fn_00a76680(Ptr)),
        entry!(0x00a76890, ni_image_converter_can_convert_pixel_data(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00a768b0, ni_image_converter_convert_filename_to_platform_specific(Ptr) -> Ptr),
        entry!(
            0x00a76960,
            ni_image_converter_convert_filename_to_platform_specific_ov2(Ptr, Ptr)
        ),
        entry!(0x00a76b30, fn_00a76b30()),
        entry!(0x00a76b60, fn_00a76b60(Ptr)),
        entry!(0x00a76bb0, ni_image_converter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a76c10, ni_image_converter_convert_pixel_data(Ptr, Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00a7_66d0,
            ni_image_converter_read_image_file_info(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a7_67c0,
            ni_image_converter_read_image_file(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a7_6a60, fn_00a76a60(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_counts_the_global_and_constructs_the_part() {
        let run = run_default_instance(0x00a7_6ab0, IMAGE_CONVERTER_SLOT, |e| {
            e.mem.set_u32(IMAGE_CONVERTER_COUNT, 7);
            e.register(PART_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0], 0x5a5a);
                Ret::default()
            });
        });
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0109_d9a4);
        assert_eq!(e.mem.u32(run.object + 0x80), 0x5a5a);
        assert_eq!(e.mem.u32(IMAGE_CONVERTER_COUNT), 8);
        // The object's own count is at +4: stored 0, then counted once by the global.
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null_and_counts_nothing() {
        let run = run_default_instance(0x00a7_6ab0, IMAGE_CONVERTER_SLOT, |e| {
            e.mem.set_u32(IMAGE_CONVERTER_COUNT, 7);
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(run.engine.mem.u32(IMAGE_CONVERTER_COUNT), 7);
    }
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const DELETE_DOUBLE: u32 = 0x0070_8000;

    fn doubles(e: &mut Engine) {
        e.map(IMAGE_CONVERTER_SLOT, 4);
        e.map(PLATFORM_DIRECTORY, 4);
        e.map(IMAGE_CONVERTER_COUNT, 4);
        e.map(EMPTY_FIXED_STRING, 4);
        for addr in [
            PART_CONSTRUCTOR,
            READER_DESTRUCTOR,
            FREE,
            SIZED_DELETE,
            STRCPY_S,
            STRNCPY_S,
            FILENAME_CONSTRUCTOR,
            FILENAME_FULL_PATH,
            MEMCPY,
            DELETE_DOUBLE,
            PALETTE_ASSIGN,
            SET_PALETTE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_INCREMENT_SLOT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(INTERLOCKED_DECREMENT_SLOT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 4, DELETE_DOUBLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn converter_global_life() {
        let mut e = Engine::new();
        doubles(&mut e);
        assert_eq!(e.call(0x00a76640, &args![]).u32(), 0);
        let a = counted(&mut e, 0);
        let b = counted(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a76b60, &args![a]);
        assert_eq!((e.mem.u32(IMAGE_CONVERTER_SLOT), e.mem.u32(a + 4)), (a, 1));
        assert_eq!(e.call(0x00a76640, &args![]).u32(), a);
        e.call(0x00a76b60, &args![a]);
        assert_eq!(e.mem.u32(a + 4), 1);
        e.call(0x00a76b60, &args![b]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![a]]);
        e.call(0x00a76b30, &args![]);
        assert_eq!(e.mem.u32(IMAGE_CONVERTER_SLOT), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![a], vec![b]]);
        e.call(0x00a76b30, &args![]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 2);
    }

    #[test]
    fn construct_and_destroy() {
        let mut e = Engine::new();
        doubles(&mut e);
        let c = e.mem.alloc(0x780);
        e.mem.set_u32(c + 4, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a76650, &args![c]).u32(), c);
        assert_eq!(
            (e.mem.u32(c), e.mem.u32(c + 4)),
            (NI_IMAGE_CONVERTER_VTABLE, 0)
        );
        assert_eq!(e.mem.u32(IMAGE_CONVERTER_COUNT), 1);
        assert_eq!(calls_to(&e, PART_CONSTRUCTOR), vec![vec![c + 0x80]]);
        e.mem.set_u32(PLATFORM_DIRECTORY, 0x4567);
        e.call(0x00a76680, &args![c]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x4567]]);
        assert_eq!(e.mem.u32(PLATFORM_DIRECTORY), 0);
        assert_eq!(calls_to(&e, READER_DESTRUCTOR), vec![vec![c + 0x80]]);
        assert_eq!(e.mem.u32(c), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(IMAGE_CONVERTER_COUNT), 0);
        assert_eq!(e.call(0x00a76bb0, &args![c, 1u32]).u32(), c);
        assert_eq!(calls_to(&e, FREE).len(), 1);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![c, 0x780]]);
        e.call(0x00a76bb0, &args![c, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn file_names_for_the_platform() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(ALLOCATE_BYTES, |_, _| word(0x6000));
        let name = e.mem.alloc(8);
        for (i, b) in b"a.dds".iter().enumerate() {
            e.mem.set_u8(name + i as u32, *b);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a768b0, &args![name]).u32(), 0x6000);
        assert_eq!(calls_to(&e, STRNCPY_S), vec![vec![0x6000, 0x104, name, 5]]);
        e.mem.set_u32(PLATFORM_DIRECTORY, 0x4567);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a768b0, &args![name]).u32(), 0x6000);
        let filename = calls_to(&e, FILENAME_CONSTRUCTOR)[0][0];
        assert_eq!(
            calls_to(&e, FILENAME_CONSTRUCTOR),
            vec![vec![filename, name]]
        );
        assert_eq!(
            calls_to(&e, STRCPY_S),
            vec![vec![filename + 0x303, 0x100, 0x4567]]
        );
        assert_eq!(
            calls_to(&e, FILENAME_FULL_PATH),
            vec![vec![filename, 0x6000, 0x104]]
        );
        assert!(calls_to(&e, STRNCPY_S).is_empty());
    }

    #[test]
    fn fixed_string_file_names_for_the_platform() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(GET_FIXED_STRING, |_, _| word(0x7708));
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 1);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, block + 8);
        let destination = e.mem.alloc(4);
        // No platform directory: the destination shares the source handle.
        e.call(0x00a76960, &args![source, destination]);
        assert_eq!((e.mem.u32(destination), e.mem.u32(block)), (block + 8, 2));
        e.call(0x00a76960, &args![source, destination]);
        assert_eq!(e.mem.u32(block), 2);
        // With one: the full path becomes a new handle; the old one's reference goes.
        e.mem.set_u32(PLATFORM_DIRECTORY, 0x4567);
        e.call_log = Some(vec![]);
        e.call(0x00a76960, &args![source, destination]);
        let filename = calls_to(&e, FILENAME_CONSTRUCTOR)[0][0];
        assert_eq!(
            calls_to(&e, FILENAME_FULL_PATH),
            vec![vec![filename, filename + 0x404, 0x104]]
        );
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![filename + 0x404]]);
        assert_eq!((e.mem.u32(destination), e.mem.u32(block)), (0x7708, 1));
    }

    #[test]
    fn can_convert_compares_the_formats() {
        let mut e = Engine::new();
        e.register(FORMAT_EQUAL, |_, a| word(u32::from(a[0] == a[1]) | 0x100));
        assert!(e.call(0x00a76890, &args![0u32, 5u32, 5u32]).bool());
        assert!(!e.call(0x00a76890, &args![0u32, 5u32, 6u32]).bool());
    }

    /// Pixel data with level sizes, offsets from 0 and its pixels at `pixels`.
    fn pixel_data(e: &mut Engine, sides: (u32, u32), offsets: &[u32], pixels: u32) -> u32 {
        let pd = e.mem.alloc(0x74);
        let w = e.mem.alloc(4);
        e.mem.set_u32(w, sides.0);
        let h = e.mem.alloc(4);
        e.mem.set_u32(h, sides.1);
        let table = e.mem.alloc(4 * offsets.len() as u32);
        for (i, o) in offsets.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *o);
        }
        e.mem.set_u32(pd + PIXEL_DATA_WIDTHS, w);
        e.mem.set_u32(pd + PIXEL_DATA_HEIGHTS, h);
        e.mem.set_u32(pd + PIXEL_DATA_OFFSETS, table);
        e.mem
            .set_u32(pd + PIXEL_DATA_LEVELS, offsets.len() as u32 - 1);
        e.mem.set_u32(pd + PIXEL_DATA_PIXELS, pixels);
        pd
    }

    #[test]
    fn convert_pixel_data_copies_or_reuses() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(FORMAT_DIFFERENT, |_, a| word(u32::from(a[1] == 0xbad)));
        e.register(FORMAT_EQUAL, |_, a| word(u32::from(a[1] != 0x0dd)));
        e.register(PIXEL_DATA_COPY, |_, a| word(a[0]));
        let source = pixel_data(&mut e, (8, 8), &[0, 64, 80], 0x1000);
        // A format the source does not have: null.
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 0xbadu32, 0u32, 0u32])
                .u32(),
            0
        );
        // No destination: a copy.
        let fresh = e.mem.alloc(0x74);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, 0u32, 0u32])
                .u32(),
            fresh
        );
        assert_eq!(calls_to(&e, PIXEL_DATA_COPY), vec![vec![fresh, source]]);
        // The source itself.
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, source, 0u32])
                .u32(),
            source
        );
        // A matching destination with one level: one copy, then the palette.
        let destination = pixel_data(&mut e, (8, 8), &[0, 64], 0x2000);
        let palette = e.mem.alloc(0x24);
        e.mem.set_u32(source + PIXEL_DATA_PALETTE, palette);
        let copied_palette = e.mem.alloc(0x24);
        e.register_double(NEW_OBJECT, move |_, _| word(copied_palette));
        e.register(PALETTE_COPY, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, destination, 0u32])
                .u32(),
            destination
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x2000, 0x1000, 64]]);
        assert_eq!(
            calls_to(&e, PALETTE_COPY),
            vec![vec![copied_palette, palette]]
        );
        assert_eq!(
            calls_to(&e, SET_PALETTE),
            vec![vec![destination, copied_palette]]
        );
        // With its own palette: assigned.
        let own = e.mem.alloc(0x24);
        e.mem.set_u32(destination + PIXEL_DATA_PALETTE, own);
        e.call_log = Some(vec![]);
        e.call(0x00a76c10, &args![0u32, source, 1u32, destination, 0u32]);
        assert_eq!(calls_to(&e, PALETTE_ASSIGN), vec![vec![own, palette]]);
        // A destination of another format, other sides or more levels: a copy instead.
        let fresh = e.mem.alloc(0x74);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        assert_eq!(
            e.call(
                0x00a76c10,
                &args![0u32, source, 0x0ddu32, destination, 0u32]
            )
            .u32(),
            fresh
        );
        let wide = pixel_data(&mut e, (16, 8), &[0, 64], 0x2000);
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, wide, 0u32])
                .u32(),
            fresh
        );
        let deep = pixel_data(&mut e, (8, 8), &[0, 64, 80, 84], 0x2000);
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, deep, 0u32])
                .u32(),
            fresh
        );
        // A failed allocation: null.
        e.register(NEW_OBJECT, |_, _| word(0));
        assert_eq!(
            e.call(0x00a76c10, &args![0u32, source, 1u32, 0u32, 0u32])
                .u32(),
            0
        );
        // A failed palette allocation sets null.
        e.mem.set_u32(destination + PIXEL_DATA_PALETTE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a76c10, &args![0u32, source, 1u32, destination, 0u32]);
        assert_eq!(calls_to(&e, SET_PALETTE), vec![vec![destination, 0]]);
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// Slots of the reader (`0x0e10_0000`) and file (`0x0e20_0000`) vtables.
    const READER_TAKES: u32 = 0x0e00_1004;
    const READER_READ: u32 = 0x0e00_1008;
    const READER_HEADER: u32 = 0x0e00_100c;
    const FILE_DELETE: u32 = 0x0e00_2000;
    const FILE_GOOD: u32 = 0x0e00_2004;

    /// A converter whose reader takes every extension, a file that is good, and a header
    /// read that answers true and a pixel read that answers `0x0e30_0000`.
    fn rig() -> (Engine, u32, u32) {
        let mut e = engine_with_counting();
        e.put_vtable(0x0e10_0000, &[0, READER_TAKES, READER_READ, READER_HEADER]);
        e.put_vtable(0x0e20_0000, &[FILE_DELETE, FILE_GOOD]);
        e.register(FILENAME_CONSTRUCTOR, |_, _| Ret::default());
        e.register(READER_TAKES, |_, _| 1u32.into_ret());
        e.register(READER_READ, |_, _| 0x0e30_0000u32.into_ret());
        e.register(READER_HEADER, |_, _| 1u32.into_ret());
        e.register(FILE_DELETE, |_, _| Ret::default());
        e.register(FILE_GOOD, |_, _| 1u32.into_ret());
        let file = e.mem.alloc(4);
        e.mem.set_u32(file, 0x0e20_0000);
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        let this = e.mem.alloc(0x780);
        e.mem.set_u32(this + READER_PART, 0x0e10_0000);
        e.call_log = Some(vec![]);
        (e, this, file)
    }

    #[test]
    fn read_image_file_info_reads_the_header_through_the_reader() {
        let (mut e, this, file) = rig();
        let args = args![this, 0x0e40_0000u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        assert!(e.call(0x00a7_66d0, &args).bool());
        let filename = calls_to(&e, FILENAME_CONSTRUCTOR)[0][0];
        assert_eq!(calls_to(&e, FILENAME_CONSTRUCTOR)[0][1], 0x0e40_0000);
        assert_eq!(
            calls_to(&e, READER_TAKES),
            vec![vec![this + READER_PART, filename + 0x103]]
        );
        assert_eq!(
            calls_to(&e, NI_FILE_GET_FILE),
            vec![vec![0x0e40_0000, 0, 0x8000]]
        );
        assert_eq!(
            calls_to(&e, READER_HEADER),
            vec![vec![this + READER_PART, file, 3, 4, 1, 2, 5]]
        );
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![file, 1]]);
        // A bad file is deleted; no file or no reader answers false.
        e.register(FILE_GOOD, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_66d0, &args).bool());
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 2);
        assert_eq!(calls_to(&e, READER_HEADER).len(), 1);
        e.register(NI_FILE_GET_FILE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_66d0, &args).bool());
        e.register(READER_TAKES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_66d0, &args).bool());
        assert_eq!(calls_to(&e, NI_FILE_GET_FILE).len(), 3);
        // The reader's false comes back after the delete.
        e.register(READER_TAKES, |_, _| 1u32.into_ret());
        e.register(FILE_GOOD, |_, _| 1u32.into_ret());
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        e.register(READER_HEADER, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_66d0, &args).bool());
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 3);
    }

    #[test]
    fn read_image_file_opens_or_takes_the_file() {
        let (mut e, this, file) = rig();
        let read = e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, 0u32],
        );
        assert_eq!(read.u32(), 0x0e30_0000);
        assert_eq!(
            calls_to(&e, READER_READ),
            vec![vec![this + READER_PART, file, 0x0e50_0000]]
        );
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![file, 1]]);
        // A given file is used (no open) and deleted too.
        let given = e.mem.alloc(4);
        e.mem.set_u32(given, 0x0e20_0000);
        e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, given],
        );
        assert_eq!(calls_to(&e, NI_FILE_GET_FILE).len(), 1);
        assert_eq!(calls_to(&e, FILE_DELETE)[1], vec![given, 1]);
        // The reader refusing the name deletes a given file and answers null.
        e.register(READER_TAKES, |_, _| 0u32.into_ret());
        let read = e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, given],
        );
        assert_eq!(read.u32(), 0);
        assert_eq!(calls_to(&e, FILE_DELETE)[2], vec![given, 1]);
        e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, 0u32],
        );
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 3);
        // A bad file is deleted and null answered; no file answers null.
        e.register(READER_TAKES, |_, _| 1u32.into_ret());
        e.register(FILE_GOOD, |_, _| 0u32.into_ret());
        let read = e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, 0u32],
        );
        assert_eq!(read.u32(), 0);
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 4);
        e.register(NI_FILE_GET_FILE, |_, _| 0u32.into_ret());
        let read = e.call(
            0x00a7_67c0,
            &args![this, 0x0e40_0000u32, 0x0e50_0000u32, 0u32],
        );
        assert_eq!(read.u32(), 0);
        assert_eq!(calls_to(&e, READER_READ).len(), 2);
    }

    #[test]
    fn palette_setter_counts_the_new_palette() {
        let mut e = engine_with_counting();
        e.put_vtable(0x0e20_0000, &[0, FILE_DELETE]);
        e.register(FILE_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let data = e.mem.alloc(0x64);
        let old = e.mem.alloc(8);
        let new = e.mem.alloc(8);
        for (palette, count) in [(old, 1u32), (new, 3)] {
            e.mem.set_u32(palette, 0x0e20_0000);
            e.mem.set_u32(palette + 4, count);
        }
        e.mem.set_u32(data + PIXEL_DATA_PALETTE, old);
        e.call(0x00a7_6a60, &args![data, new]);
        assert_eq!(e.mem.u32(data + PIXEL_DATA_PALETTE), new);
        assert_eq!(e.mem.u32(new + 4), 4);
        assert_eq!(calls_to(&e, FILE_DELETE), vec![vec![old]]);
        e.call(0x00a7_6a60, &args![data, new]);
        assert_eq!(e.mem.u32(new + 4), 4);
        e.call(0x00a7_6a60, &args![data, 0u32]);
        assert_eq!(e.mem.u32(data + PIXEL_DATA_PALETTE), 0);
        assert_eq!(e.mem.u32(new + 4), 3);
    }
}

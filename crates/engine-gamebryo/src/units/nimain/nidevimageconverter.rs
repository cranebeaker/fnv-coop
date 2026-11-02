//! `NiMain/nidevimageconverter.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

// Translated from 00a7b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7aff0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xa00 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_dev_image_converter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_aff0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xa00u32]);
    }
    this
}

/// `NiImageConverter`'s constructor (`00a76650`, engine map: `niimageconverter.cpp`;
/// `thiscall`, no stack words).
const IMAGE_CONVERTER_CONSTRUCT: u32 = 0x00a7_6650;
/// `NiDevImageConverter`'s virtual table (`0109d9c4`; its RTTI names `NiDevImageConverter`).
const NI_DEV_IMAGE_CONVERTER_VTABLE: u32 = 0x0109_d9c4;
/// `00a8bd20` (unnamed; `thiscall`, no stack words), run on the member at +0x780.
const MEMBER_780_CONSTRUCT: u32 = 0x00a8_bd20;
const MEMBER_780: u32 = 0x780;
/// The reader list (`NiTPointerList` at +0x990: head, tail, count).
const READERS: u32 = 0x990;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// Bytes of each reader the constructor makes.
const READER_SIZE: u32 = 0x180;
/// The three reader constructors, in the exe's order (`00a8e410` and `00a8d750` unnamed,
/// `00a8c190` in `niddsreader.cpp`; `thiscall`, no stack words).
const READER_CONSTRUCTORS: [u32; 3] = [0x00a8_e410, 0x00a8_d750, 0x00a8_c190];
/// `0043a010` (engine map: `NiTPointerListBase<..>::AddHead`; its body takes a cleared list
/// item from the item pool and answers it), `thiscall` on the list's count word.
const LIST_ITEM_NEW: u32 = 0x0043_a010;

// Translated from 00a7b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::NiDevImageConverter` (Xbox PDB), `thiscall`, no stack words: the
/// `NiImageConverter` constructor, the class's virtual table, the member at +0x780
/// (`00a8bd20`), an empty reader list at +0x990, then three readers of 0x180 bytes (from the
/// memory manager; a failed allocation puts a null reader) made by `00a8e410`, `00a8d750` and
/// `00a8c190` and each put at the head of the list (`0043a010` gives the item). Answers `this`.
pub fn ni_dev_image_converter_ni_dev_image_converter(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(IMAGE_CONVERTER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_DEV_IMAGE_CONVERTER_VTABLE);
    e.call(
        MEMBER_780_CONSTRUCT,
        &args![this.addr().wrapping_add(MEMBER_780)],
    );
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr().wrapping_add(READERS));
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    for constructor in READER_CONSTRUCTORS {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, READER_SIZE])
            .u32();
        let reader = if block != 0 {
            e.call(constructor, &args![block]).u32()
        } else {
            0
        };
        let count_word = list.addr().wrapping_add(8);
        let item: Ptr<NiTListItem> = e.call(LIST_ITEM_NEW, &args![count_word]).ptr();
        e.set(item, NiTListItem::m_element, reader);
        e.set(item, NiTListItem::m_pkPrev, 0);
        let head = e.get(list, NiTPointerList::m_pkHead);
        e.set(item, NiTListItem::m_pkNext, head);
        let head = e.get(list, NiTPointerList::m_pkHead);
        if head != 0 {
            e.set(
                Ptr::<NiTListItem>::new(head),
                NiTListItem::m_pkPrev,
                item.addr(),
            );
        } else {
            e.set(list, NiTPointerList::m_pkTail, item.addr());
        }
        let count = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(list, NiTPointerList::m_uiCount, count);
        e.set(list, NiTPointerList::m_pkHead, item.addr());
    }
    this
}

/// `NiDevImageConverter::PixelBits` (Xbox PDB name of the 0x18-byte record
/// `00a779b0` fills): the four component masks (+0..+0xc), the four left
/// shifts (+0x10..+0x13) and the four right shifts `8 - bits` (+0x14..+0x17).
const PIXEL_BITS_SIZE: u32 = 0x18;

/// `00a62190` (`thiscall` on an `NiPixelFormat`, `RET 4`: a component index):
/// the component's mask; `00a62230` (same shape): its shift byte.
const FORMAT_COMPONENT_MASK: u32 = 0x00a6_2190;
const FORMAT_COMPONENT_SHIFT: u32 = 0x00a6_2230;

/// The bits byte of the first of the format's four component records
/// (`format+0x14`, 0xc bytes each: type word, .., bits byte at `+8`) whose
/// type is `component`; 0 when none is.
fn component_bits(e: &Engine, format: u32, component: u32) -> u8 {
    for index in 0..4u32 {
        if e.mem.u32(format + 0x14 + index * 0xc) == component {
            return e.mem.u8(format + 0x1c + index * 0xc);
        }
    }
    0
}

/// `NiPixelFormat` (Xbox PDB) as `00a77980` reads it: `m_akComponents` from +0x14, four
/// entries of 0xc bytes, each the component kind word at +0 and its bit count byte at +8.
const PIXEL_FORMAT_COMPONENTS: u32 = 0x14;
const PIXEL_FORMAT_COMPONENT_SIZE: u32 = 0xc;
const PIXEL_FORMAT_COMPONENT_BITS: u32 = 0x8;

// Translated from 00a77980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidevimageconverter.cpp` range), `thiscall` on an
/// `NiPixelFormat` with one word (a component kind; the shape of `NiPixelFormat::GetBits`,
/// unconfirmed): the bit count byte of the first of the four components whose kind is the word,
/// 0 when none is (only `AL` is set; the rest of `EAX` is not part of the answer).
pub fn fn_00a77980(e: &mut Engine, this: Ptr, kind: u32) -> u8 {
    let format = this.addr();
    for index in 0..4u32 {
        let entry = format
            .wrapping_add(PIXEL_FORMAT_COMPONENTS)
            .wrapping_add(index * PIXEL_FORMAT_COMPONENT_SIZE);
        if e.mem.u32(entry) == kind {
            return e.mem.u8(entry.wrapping_add(PIXEL_FORMAT_COMPONENT_BITS));
        }
    }
    0
}

// Translated from 00a779b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::PixelBits::PixelBits` (engine map name), `thiscall`,
/// `RET 4` (the pixel format): for each component 0..3 in turn its mask
/// (`00a62190`) at `+4c`, its shift byte (`00a62230`) at `+0x10+c` and
/// `8 - bits` of its component record at `+0x14+c`. Answers `this`.
pub fn ni_dev_image_converter_pixel_bits(e: &mut Engine, this: Ptr, format: Ptr) -> Ptr {
    let t = this.addr();
    for component in 0..4u32 {
        let mask = e
            .call(FORMAT_COMPONENT_MASK, &args![format, component])
            .u32();
        e.mem.set_u32(t + 4 * component, mask);
        let shift = e
            .call(FORMAT_COMPONENT_SHIFT, &args![format, component])
            .u32() as u8;
        e.mem.set_u8(t + 0x10 + component, shift);
        let bits = component_bits(e, format.addr(), component);
        e.mem.set_u8(t + 0x14 + component, 8u8.wrapping_sub(bits));
    }
    this
}

/// One channel of a pixel: the byte moved right by the record's `8 - bits`
/// byte, left by its shift byte, and masked (the x86 shift counts use their
/// low five bits).
fn channel(e: &Engine, bits: u32, index: u32, value: u32) -> u32 {
    let right = u32::from(e.mem.u8(bits + 0x14 + index));
    let left = u32::from(e.mem.u8(bits + 0x10 + index));
    value.wrapping_shr(right).wrapping_shl(left) & e.mem.u32(bits + 4 * index)
}

// Translated from 00a76f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: one of `ConvertPixelDataFormat`'s row
/// converters (`cdecl`, eight words: width, height, the level's size, the
/// destination, the palette, the destination's `PixelBits`, the source and
/// the source's `PixelBits`; the size, the palette and the source's bits are
/// not read). For `height` rows of `width` pixels: three source bytes become
/// one 32-bit word (each byte placed through the destination bits' shifts and
/// masks of components 0, 1 and 2, and 0xff for component 3).
#[allow(clippy::too_many_arguments)] // the converters' eight stack words
pub fn fn_00a76f80(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut out = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from + 1));
            let third = u32::from(e.mem.u8(from + 2));
            from = from.wrapping_add(3);
            let word = channel(e, bits, 0, first)
                | channel(e, bits, 1, second)
                | channel(e, bits, 2, third)
                | channel(e, bits, 3, 0xff);
            e.mem.set_u32(out, word);
            out = out.wrapping_add(4);
        }
    }
}

// Translated from 00a771d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its
/// bytes): one of `ConvertPixelDataFormat`'s row converters, the same eight
/// `cdecl` words as `00a76f80`. For `height` rows of `width` pixels: four
/// source bytes become one 32-bit word, each byte placed through the destination
/// bits' shifts and masks of components 0..3.
#[allow(clippy::too_many_arguments)] // the converters' eight stack words
pub fn fn_00a771d0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut out = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from + 1));
            let third = u32::from(e.mem.u8(from + 2));
            let fourth = u32::from(e.mem.u8(from + 3));
            from = from.wrapping_add(4);
            let word = channel(e, bits, 0, first)
                | channel(e, bits, 2, third)
                | channel(e, bits, 1, second)
                | channel(e, bits, 3, fourth);
            e.mem.set_u32(out, word);
            out = out.wrapping_add(4);
        }
    }
}

/// `NiPixelFormat::operator==` (`00a76540`, `thiscall`, `RET 4`: the other
/// format) and the other comparison `00a765c0` (engine map
/// `NiPixelFormat::operator__ov2`, same shape).
const FORMAT_EQUALS: u32 = 0x00a7_6540;
const FORMAT_COMPARE_00A765C0: u32 = 0x00a7_65c0;
/// The static pixel formats the converter compares with (`NiPixelFormat`
/// globals of 0x44 bytes in `.data`).
const FORMAT_011AA180: u32 = 0x011a_a180;
const FORMAT_011AA1C8: u32 = 0x011a_a1c8;
const FORMAT_011AA210: u32 = 0x011a_a210;
const FORMAT_011AA258: u32 = 0x011a_a258;
const FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const FORMAT_011AA2E8: u32 = 0x011a_a2e8;
const FORMAT_011AA330: u32 = 0x011a_a330;
const FORMAT_011AA3C0: u32 = 0x011a_a3c0;
const FORMAT_011AA408: u32 = 0x011a_a408;
const FORMAT_011AA450: u32 = 0x011a_a450;

/// `format == other` through `00a76540`.
fn format_is(e: &mut Engine, format: u32, other: u32) -> bool {
    e.call(FORMAT_EQUALS, &args![format, other]).bool()
}

// Translated from 00a78010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::IsAnAlphaSupportedMipmapGenerationFormat` (engine
/// map name; a virtual method, `thiscall`, `RET 4`: the format): whether the
/// format equals `011aa1c8`, `011aa258` or `011aa2a0` (tested in that order).
pub fn ni_dev_image_converter_is_an_alpha_supported_mipmap_generation_format(
    e: &mut Engine,
    _this: Ptr,
    format: u32,
) -> bool {
    format_is(e, format, FORMAT_011AA1C8)
        || format_is(e, format, FORMAT_011AA258)
        || format_is(e, format, FORMAT_011AA2A0)
}

// Translated from 00a78050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::IsANonAlphaSupportedMipmapGenerationFormat` (engine
/// map name; a virtual method, `thiscall`, `RET 4`: the format): whether the
/// format equals `011aa180`, `011aa210` or `011aa2e8` (in that order).
pub fn ni_dev_image_converter_is_a_non_alpha_supported_mipmap_generation_format(
    e: &mut Engine,
    _this: Ptr,
    format: u32,
) -> bool {
    format_is(e, format, FORMAT_011AA180)
        || format_is(e, format, FORMAT_011AA210)
        || format_is(e, format, FORMAT_011AA2E8)
}

/// `00a77960` (`thiscall` on a format, no stack words): whether its format
/// word (+4) is 4..6.
const FORMAT_IS_COMPRESSED_00A77960: u32 = 0x00a7_7960;

// Translated from 00a77e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::CanConvertPixelData` (engine map name; virtual,
/// `thiscall`, `RET 8`: the source and the destination formats). The
/// destination's `PixelBits` are made first. Equal formats: true. Otherwise
/// both need bit 0 of their first byte, the destination's format word (+4)
/// must not be 4..6, and a 24-bit destination (+1) needs the masks 0xff00 for
/// component 1 and 0xff or 0xff0000 for component 0. Then by the source:
/// `011aa180`/`011aa1c8` (a destination of format 0 or 1 needs 24 or 32 bits;
/// any other must compare 0 with `00a765c0` against `011aa180`, else against
/// `011aa1c8`), `011aa210`/`011aa258` (format 0 or 1: 16, 24 or 32 bits; other:
/// the same two-step test against `011aa210` and `011aa258`),
/// `011aa2e8`/`011aa2a0`/a compressed source (`00a77960`) (format 0, 1, 8 or
/// 9: 16, 24 or 32 bits), `011aa408`/`011aa450` (format 8 or 9: 16, 24 or 32
/// bits), `011aa330`/`011aa3c0` (16 bits); anything else is false.
pub fn ni_dev_image_converter_can_convert_pixel_data(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
    destination: u32,
) -> bool {
    let kind = e.mem.u32(destination + 4);
    let depth = u32::from(e.mem.u8(destination + 1));
    let (mask_0, mask_1) = e.with_stack(PIXEL_BITS_SIZE, |e, bits| {
        ni_dev_image_converter_pixel_bits(e, bits, Ptr::new(destination));
        (e.mem.u32(bits.addr()), e.mem.u32(bits.addr() + 4))
    });
    if format_is(e, source, destination) {
        return true;
    }
    if e.mem.u8(source) & 1 == 0 || e.mem.u8(destination) & 1 == 0 {
        return false;
    }
    let format_word = e.mem.u32(destination + 4) as i32;
    if (4..=6).contains(&format_word) {
        return false;
    }
    if depth == 0x18 && (mask_1 != 0xff00 || (mask_0 != 0xff && mask_0 != 0x00ff_0000)) {
        return false;
    }
    let any_depth = |depth: u32| depth == 0x10 || depth == 0x18 || depth == 0x20;
    let palette_kind = kind == 8 || kind == 9;
    let second_test = |e: &mut Engine, first: u32, second: u32| {
        e.call(FORMAT_COMPARE_00A765C0, &args![destination, first])
            .u32() as u8
            == 0
            || e.call(FORMAT_COMPARE_00A765C0, &args![destination, second])
                .u32() as u8
                == 0
    };
    if format_is(e, source, FORMAT_011AA180) || format_is(e, source, FORMAT_011AA1C8) {
        if kind == 0 || kind == 1 {
            return depth == 0x18 || depth == 0x20;
        }
        return second_test(e, FORMAT_011AA180, FORMAT_011AA1C8);
    }
    if format_is(e, source, FORMAT_011AA210) || format_is(e, source, FORMAT_011AA258) {
        if kind == 0 || kind == 1 {
            return any_depth(depth);
        }
        return second_test(e, FORMAT_011AA210, FORMAT_011AA258);
    }
    if format_is(e, source, FORMAT_011AA2E8)
        || format_is(e, source, FORMAT_011AA2A0)
        || e.call(FORMAT_IS_COMPRESSED_00A77960, &args![source]).u32() as u8 != 0
    {
        if kind == 0 || kind == 1 {
            return any_depth(depth);
        }
        return palette_kind && any_depth(depth);
    }
    if format_is(e, source, FORMAT_011AA408) || format_is(e, source, FORMAT_011AA450) {
        return palette_kind && any_depth(depth);
    }
    if format_is(e, source, FORMAT_011AA330) || format_is(e, source, FORMAT_011AA3C0) {
        return depth == 0x10;
    }
    false
}

/// CRT `_strcpy_s` (`00ec65a6`, `cdecl`), `NiPath::Standardize` (`00aa2a00`,
/// `cdecl`), `NiFilename::NiFilename` (`00aa27d0`, `thiscall`, the path) and
/// `NiFile::GetFile` (`00aa1a20`, `cdecl`: path, mode, buffer size).
const STRCPY_S: u32 = 0x00ec_65a6;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const NI_FILENAME_CONSTRUCT: u32 = 0x00aa_27d0;
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;

// Translated from 00a77d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a77d10 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiDevImageConverter::ReadImageFile` (engine map name; virtual,
/// `thiscall`, `RET 0xc`: the file name, a word passed to the reader, and an
/// open file or 0). The name is copied into a 0x104-byte buffer, standardized
/// and split (`NiFilename`). Each reader of the list at `+0x990` whose slot
/// `+4` accepts the extension gets the file (the given one, or a new one from
/// `NiFile::GetFile(path, 0, 0x8000)`; none answers 0): a file whose slot `+4`
/// answers false is deleted (slot 0 with 1, even a given one) and 0 answered;
/// otherwise the reader's slot `+8` reads it with the word. A result deletes
/// the file (even a given one) and is answered; no result deletes a file made
/// here and goes on with the next reader. After the last reader a given file
/// is deleted and 0 answered.
pub fn ni_dev_image_converter_read_image_file(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    word: u32,
    given: u32,
) -> u32 {
    let mut item = e.mem.u32(this.addr() + READERS);
    e.with_stack(0x50c, |e, frame| {
        // The frame: the word (+0), the split name (+4, 0x403 bytes), the
        // path (+0x408, 0x104 bytes).
        let base = frame.addr();
        let filename = base + 4;
        let path = base + 0x408;
        e.mem.set_u32(base, word);
        e.call(STRCPY_S, &args![path, 0x104u32, name]);
        e.call(PATH_STANDARDIZE, &args![path]);
        e.call(NI_FILENAME_CONSTRUCT, &args![filename, path]);
        while item != 0 {
            let reader = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if !e.vcall(reader, 4, &args![filename + 0x103]).bool() {
                continue;
            }
            let file = if given != 0 {
                given
            } else {
                e.call(NI_FILE_GET_FILE, &args![path, 0u32, 0x8000u32])
                    .u32()
            };
            if file == 0 {
                return 0;
            }
            if !e.vcall(file, 4, &args![]).bool() {
                e.vcall(file, 0, &args![1u32]);
                return 0;
            }
            let word = e.mem.u32(base);
            let data = e.vcall(reader, 8, &args![file, word]).u32();
            if data != 0 {
                e.vcall(file, 0, &args![1u32]);
                return data;
            }
            if given == 0 {
                e.vcall(file, 0, &args![1u32]);
            }
        }
        if given != 0 {
            e.vcall(given, 0, &args![1u32]);
        }
        0
    })
}

/// `NiTextureCodec::ExpandCompressedData` (`00a8be70`, `thiscall` on the codec
/// at `+0x780`, `RET 4`: the pixel data), the `NiPointer` assignment
/// `00a5f860` (`thiscall` on the pointer, `RET 4`: the object), the
/// `NiPointer` release `00b66f20` (`thiscall` on the pointer), the
/// power-of-two test `00a77920` (`cdecl`, a word; a byte answer), the Gamebryo
/// allocation `00aa13e0` (`cdecl`: the size) and
/// `NiPixelData::NiPixelData_ov2` (`00a7c190`, `thiscall`, `RET 0x14`: width,
/// height, format address, mipmap levels, faces).
const CODEC_EXPAND_COMPRESSED_DATA: u32 = 0x00a8_be70;
const NI_POINTER_ASSIGN: u32 = 0x00a5_f860;
const NI_POINTER_RELEASE: u32 = 0x00b6_6f20;
const IS_POWER_OF_TWO_00A77920: u32 = 0x00a7_7920;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiPixelData` words the converter reads: the pixels (+0x50), the widths
/// (+0x54), the heights (+0x58), the level offsets (+0x5c), the level count
/// (+0x60) and the face count (+0x6c).
const DATA_PIXELS: u32 = 0x50;
const DATA_WIDTHS: u32 = 0x54;
const DATA_HEIGHTS: u32 = 0x58;
const DATA_OFFSETS: u32 = 0x5c;
const DATA_LEVELS: u32 = 0x60;
const DATA_FACES: u32 = 0x6c;

/// The inline `NiPointer` release of `object` (when not null).
fn release_object(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a7a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::ConvertPixelData` (engine map name; virtual,
/// `thiscall`, `RET 0x10`: the source data, the destination format, a
/// destination data or 0, and the mipmap byte). Both need bit 0 of their
/// format's first byte (source `+8`). A compressed source (format word `+0xc`
/// 4..6) of another format than the destination is expanded first (the codec
/// at `+0x780`). With the mipmap byte, a one-level source of power-of-two
/// width (and height, `00a77920`) and a destination format 0, 1, 8 or 9 gets
/// its levels (`GenerateMipmapLevels` with the destination data). A given
/// destination of the destination format (and more than one level when
/// mipmapping) that is the source or takes it (slot `+0x30` with -1) is
/// answered after the two pointers are released (`00b66f20`). Otherwise a
/// palette destination format (8 or 9) gets a palettized copy (slot `+0x34`);
/// a new `NiPixelData` of the source's size, the destination format, the
/// source's levels and faces is filled through slot `+0x30` (-1) and
/// answered, or 0 when that fails (the new data is not freed); the
/// intermediate pointers are released either way.
pub fn ni_dev_image_converter_convert_pixel_data(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    format: u32,
    destination: u32,
    mipmap: u32,
) -> u32 {
    let t = this.addr();
    if e.mem.u8(source + 8) & 1 == 0 || e.mem.u8(format) & 1 == 0 {
        return 0;
    }
    e.with_stack(12, |e, frame| {
        // The frame: the expanded data (+0), the mipmapped data (+4), the
        // palettized data (+8); each an `NiPointer`.
        let expanded = frame.addr();
        let mipmapped = expanded + 4;
        let palettized = expanded + 8;
        let mut current = source;
        let source_kind = e.mem.u32(source + 0xc) as i32;
        if (4..=6).contains(&source_kind) && e.mem.u32(format + 4) != e.mem.u32(source + 0xc) {
            let data = e
                .call(CODEC_EXPAND_COMPRESSED_DATA, &args![t + MEMBER_780, source])
                .u32();
            e.call(NI_POINTER_ASSIGN, &args![expanded, data]);
            current = e.mem.u32(expanded);
        }
        if mipmap as u8 != 0 && e.mem.u32(current + DATA_LEVELS) <= 1 {
            let width = e.mem.u32(e.mem.u32(current + DATA_WIDTHS));
            if width != 0 && width & width.wrapping_sub(1) == 0 {
                let height = e.mem.u32(e.mem.u32(current + DATA_HEIGHTS));
                let square = e.call(IS_POWER_OF_TWO_00A77920, &args![height]).u32() as u8 != 0;
                let kind = e.mem.u32(format + 4);
                if square && (kind == 0 || kind == 1 || kind == 8 || kind == 9) {
                    let data = ni_dev_image_converter_generate_mipmap_levels(
                        e,
                        this,
                        current,
                        destination,
                    );
                    e.call(NI_POINTER_ASSIGN, &args![mipmapped, data]);
                    let made = e.mem.u32(mipmapped);
                    if made != 0 {
                        current = made;
                    }
                }
            }
        }
        if destination != 0
            && format_is(e, destination + 8, format)
            && (mipmap as u8 == 0 || e.mem.u32(destination + DATA_LEVELS) > 1)
            && (destination == current
                || e.vcall(t, 0x30, &args![destination, current, 0xffff_ffffu32])
                    .bool())
        {
            e.call(NI_POINTER_RELEASE, &args![mipmapped]);
            e.call(NI_POINTER_RELEASE, &args![expanded]);
            return destination;
        }
        let mut palette_data = 0;
        let kind = e.mem.u32(format + 4);
        if kind == 8 || kind == 9 {
            let data = e.vcall(t, 0x34, &args![current, format]).u32();
            e.call(NI_POINTER_ASSIGN, &args![palettized, data]);
            palette_data = e.mem.u32(palettized);
            if palette_data != 0 {
                current = palette_data;
            }
        }
        let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
        let made = if block != 0 {
            let width = e.mem.u32(e.mem.u32(current + DATA_WIDTHS));
            let height = e.mem.u32(e.mem.u32(current + DATA_HEIGHTS));
            let levels = e.mem.u32(current + DATA_LEVELS);
            let faces = e.mem.u32(current + DATA_FACES);
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![block, width, height, format, levels, faces],
            )
            .u32()
        } else {
            0
        };
        let filled = e
            .vcall(t, 0x30, &args![made, current, 0xffff_ffffu32])
            .bool();
        release_object(e, palette_data);
        let mipmapped_data = e.mem.u32(mipmapped);
        release_object(e, mipmapped_data);
        let expanded_data = e.mem.u32(expanded);
        release_object(e, expanded_data);
        if filled {
            made
        } else {
            0
        }
    })
}

/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `thiscall`, no stack words).
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;

/// One box-filtered level: `size` bytes per pixel (3 or 4) from the level
/// above at `from` into `to`, `width` x `height` pixels, the level above
/// `above_width` x `above_height`. A level above one pixel wide (or else one
/// pixel high) averages pixel pairs; otherwise each pixel averages a 2x2
/// block of two rows.
#[allow(clippy::too_many_arguments)] // the loop's registers
fn filter_level(
    e: &mut Engine,
    size: u32,
    from: u32,
    to: u32,
    width: u32,
    height: u32,
    above_width: u32,
    above_height: u32,
) {
    let mut from = from;
    let mut to = to;
    let row = above_width.wrapping_mul(size);
    let pair = |e: &mut Engine, from: u32, to: u32| {
        for byte in 0..size {
            let sum = u32::from(e.mem.u8(from + byte)) + u32::from(e.mem.u8(from + size + byte));
            e.mem.set_u8(to + byte, (sum >> 1) as u8);
        }
    };
    if above_width == 1 {
        for _ in 0..height {
            pair(e, from, to);
            to = to.wrapping_add(size);
            from = from.wrapping_add(2 * size);
        }
        return;
    }
    if above_height == 1 {
        for _ in 0..width {
            pair(e, from, to);
            to = to.wrapping_add(size);
            from = from.wrapping_add(2 * size);
        }
        return;
    }
    for _ in 0..height {
        if width != 0 {
            let mut next = from.wrapping_add(row);
            for _ in 0..width {
                for byte in 0..size {
                    let sum = u32::from(e.mem.u8(from + byte))
                        + u32::from(e.mem.u8(from + size + byte))
                        + u32::from(e.mem.u8(next + byte))
                        + u32::from(e.mem.u8(next + size + byte));
                    e.mem.set_u8(to + byte, (sum >> 2) as u8);
                }
                to = to.wrapping_add(size);
                from = from.wrapping_add(2 * size);
                next = next.wrapping_add(2 * size);
            }
        }
        from = from.wrapping_add(row);
    }
}

// Translated from 00a78090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::GenerateMipmapLevels` (engine map name; virtual,
/// `thiscall`, `RET 8`: the source data and a destination data or 0). The
/// working format is `011aa2e8` when the source format passes slot `+0x24`
/// (non-alpha mipmap formats), else `011aa2a0` when it passes slot `+0x20`,
/// else 0 is answered (a constructed `NiPixelFormat` local is overwritten by a
/// copy of 0x11 words). The destination is used when it has that format, more
/// than one level and the source's face count; otherwise a new `NiPixelData`
/// of the source's size with that format, all levels (0) and the source's
/// faces. A destination other than the source is filled from it (slot
/// `+0x30` with face 0; false answers 0). Then for each face every level from
/// 1 is box-filtered from the one above it: 3 bytes per pixel for `011aa2e8`,
/// 4 for `011aa2a0`. Answers the destination.
pub fn ni_dev_image_converter_generate_mipmap_levels(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    destination: u32,
) -> u32 {
    let t = this.addr();
    e.with_stack(0x44, |e, working| {
        let working = working.addr();
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![working]);
        let template = if e.vcall(t, 0x24, &args![source + 8]).bool() {
            FORMAT_011AA2E8
        } else if e.vcall(t, 0x20, &args![source + 8]).bool() {
            FORMAT_011AA2A0
        } else {
            return 0;
        };
        for word in 0..0x11u32 {
            let value = e.mem.u32(template + 4 * word);
            e.mem.set_u32(working + 4 * word, value);
        }
        let reuse = destination != 0
            && format_is(e, destination + 8, working)
            && e.mem.u32(destination + DATA_LEVELS) > 1
            && e.mem.u32(destination + DATA_FACES) == e.mem.u32(source + DATA_FACES);
        let target = if reuse {
            destination
        } else {
            let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
            if block != 0 {
                let faces = e.mem.u32(source + DATA_FACES);
                let height = e.mem.u32(e.mem.u32(source + DATA_HEIGHTS));
                let width = e.mem.u32(e.mem.u32(source + DATA_WIDTHS));
                e.call(
                    NI_PIXEL_DATA_CONSTRUCT,
                    &args![block, width, height, working, 0u32, faces],
                )
                .u32()
            } else {
                0
            }
        };
        let levels = e.mem.u32(target + DATA_LEVELS);
        if source != target && !e.vcall(t, 0x30, &args![target, source, 0u32]).bool() {
            return 0;
        }
        let mut face = 0;
        while face < e.mem.u32(source + DATA_FACES) {
            let size = if format_is(e, working, FORMAT_011AA2E8) {
                3
            } else if format_is(e, working, FORMAT_011AA2A0) {
                4
            } else {
                0
            };
            if size != 0 {
                let mut level = 1;
                while level < levels {
                    let offsets = e.mem.u32(target + DATA_OFFSETS);
                    let all = e.mem.u32(target + DATA_LEVELS);
                    let face_offset = e.mem.u32(offsets + 4 * all).wrapping_mul(face);
                    let pixels = e.mem.u32(target + DATA_PIXELS);
                    let from = e
                        .mem
                        .u32(offsets + 4 * (level - 1))
                        .wrapping_add(pixels)
                        .wrapping_add(face_offset);
                    let to = pixels
                        .wrapping_add(face_offset)
                        .wrapping_add(e.mem.u32(offsets + 4 * level));
                    let widths = e.mem.u32(target + DATA_WIDTHS);
                    let width = e.mem.u32(widths + 4 * level);
                    let above_width = e.mem.u32(widths + 4 * (level - 1));
                    let heights = e.mem.u32(target + DATA_HEIGHTS);
                    let height = e.mem.u32(heights + 4 * level);
                    let above_height = if above_width == 1 {
                        0
                    } else {
                        e.mem.u32(heights + 4 * (level - 1))
                    };
                    filter_level(e, size, from, to, width, height, above_width, above_height);
                    level += 1;
                }
            }
            face += 1;
        }
        target
    })
}

/// The shared list-node pool's lock (`011c5f80`, count at `011c5f84`, taken
/// with `0040fbf0`) and free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE: u32 = 0x011c_5f58;
const LOCK_TAKE: u32 = 0x0040_fbf0;
/// `NiTPointerListBase<..>::RemoveAll` (`00b71300`), the member at `+0x780`'s
/// destructor (`00a29680`) and `NiImageConverter`'s destructor body
/// (`00a76680`), all `thiscall` with no stack words.
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const MEMBER_780_DESTRUCT: u32 = 0x00a2_9680;
const IMAGE_CONVERTER_DESTRUCT: u32 = 0x00a7_6680;

// Translated from 00a7aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiDevImageConverter`'s destructor body
/// (`thiscall`, no stack words). Stores the class's vtable; while the reader
/// list (`+0x990`) has items: the head is unlinked, its reader kept and
/// cleared, the item goes back to the shared pool (under the pool lock), the
/// count goes down and the reader (when not null) is deleted (slot 0 with 1).
/// Then `RemoveAll` on the list, the member at `+0x780`'s destructor and
/// `NiImageConverter`'s destructor body.
pub fn fn_00a7aff0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, NI_DEV_IMAGE_CONVERTER_VTABLE);
    let list: Ptr<NiTPointerList> = Ptr::new(t + READERS);
    while e.get(list, NiTPointerList::m_uiCount) != 0 {
        let item = e.get(list, NiTPointerList::m_pkHead);
        let next = e.mem.u32(item);
        e.set(list, NiTPointerList::m_pkHead, next);
        if next != 0 {
            e.mem.set_u32(next + 4, 0);
        } else {
            e.set(list, NiTPointerList::m_pkTail, 0);
        }
        let reader = e.mem.u32(item + 8);
        e.mem.set_u32(item + 8, 0);
        e.call(LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
        e.mem.set_u32(item + 4, 0);
        let free = e.global::<u32>(NODE_POOL_FREE);
        e.mem.set_u32(item, free);
        let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
        e.set_global::<u32>(NODE_POOL_FREE, item);
        if count == 0 {
            e.set_global::<u32>(NODE_POOL_LOCK, 0);
        }
        let remaining = e.get(list, NiTPointerList::m_uiCount).wrapping_sub(1);
        e.set(list, NiTPointerList::m_uiCount, remaining);
        if reader != 0 {
            e.vcall(reader, 0, &args![1u32]);
        }
    }
    e.call(POINTER_LIST_REMOVE_ALL, &args![list]);
    e.call(MEMBER_780_DESTRUCT, &args![t + MEMBER_780]);
    e.call(IMAGE_CONVERTER_DESTRUCT, &args![this]);
}

/// `NiPalette`'s copy constructor (`00a8b080`, `thiscall`, `RET 4`: the
/// other palette), its assignment (`00a8b190`, `thiscall`, `RET 4`), its
/// constructor from entries (`00a8b110`, `thiscall`, `RET 0xc`: 0, the entry
/// count, the entries) and the pixel data's palette setter (`00a76a60`,
/// `thiscall`, `RET 4`).
const PALETTE_COPY: u32 = 0x00a8_b080;
const PALETTE_ASSIGN: u32 = 0x00a8_b190;
const PALETTE_FROM_ENTRIES: u32 = 0x00a8_b110;
const DATA_SET_PALETTE: u32 = 0x00a7_6a60;
/// `NiPixelData`'s palette (`+0x4c`).
const DATA_PALETTE: u32 = 0x4c;
/// The CRT `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;

/// The levels `first..=last` of every face copied straight from the source
/// (each level's size from the source's offsets) with `memcpy`.
fn copy_levels(e: &mut Engine, destination: u32, source: u32, faces: u32, first: u32, last: u32) {
    let mut face = 0;
    while face < faces {
        let mut level = first;
        while level <= last {
            let offsets = e.mem.u32(source + DATA_OFFSETS);
            let size = e
                .mem
                .u32(offsets + 4 * (level + 1))
                .wrapping_sub(e.mem.u32(offsets + 4 * level));
            let all = e.mem.u32(source + DATA_LEVELS);
            let from = e
                .mem
                .u32(offsets + 4 * all)
                .wrapping_mul(face)
                .wrapping_add(e.mem.u32(offsets + 4 * level))
                .wrapping_add(e.mem.u32(source + DATA_PIXELS));
            let offsets = e.mem.u32(destination + DATA_OFFSETS);
            let all = e.mem.u32(destination + DATA_LEVELS);
            let to = e
                .mem
                .u32(offsets + 4 * all)
                .wrapping_mul(face)
                .wrapping_add(e.mem.u32(offsets + 4 * level))
                .wrapping_add(e.mem.u32(destination + DATA_PIXELS));
            e.call(MEMCPY_00EC44D0, &args![to, from, size]);
            level += 1;
        }
        face += 1;
    }
}

/// The destination gets a copy of the source's palette: assigned into its
/// own when it has one, else a new copy (0x24 bytes; a failed allocation sets
/// a null palette).
fn copy_palette(e: &mut Engine, destination: u32, source_palette: u32) {
    let own = e.mem.u32(destination + DATA_PALETTE);
    if own != 0 {
        e.call(PALETTE_ASSIGN, &args![own, source_palette]);
        return;
    }
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    let palette = if block != 0 {
        e.call(PALETTE_COPY, &args![block, source_palette]).u32()
    } else {
        0
    };
    e.call(DATA_SET_PALETTE, &args![destination, palette]);
}

/// A new palette of `count` entries made from the source palette's entries
/// (`+0x14`) for the destination (a failed allocation sets a null palette).
fn palette_from_entries(e: &mut Engine, destination: u32, source_palette: u32, count: u32) {
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    if block == 0 {
        e.call(DATA_SET_PALETTE, &args![destination, 0u32]);
        return;
    }
    let entries = e.mem.u32(source_palette + 0x14);
    let palette = e
        .call(PALETTE_FROM_ENTRIES, &args![block, 0u32, count, entries])
        .u32();
    e.call(DATA_SET_PALETTE, &args![destination, palette]);
}

/// The second palette step of the palettized copies: the source must have a
/// palette (else false); it is copied, and when the destination's palette
/// has a word at `+0xc` the fourth byte of its first entry becomes 0xff.
fn copy_palette_opaque(e: &mut Engine, destination: u32, source: u32) -> bool {
    let source_palette = e.mem.u32(source + DATA_PALETTE);
    if source_palette == 0 {
        return false;
    }
    copy_palette(e, destination, source_palette);
    let palette = e.mem.u32(destination + DATA_PALETTE);
    if e.mem.u32(palette + 0xc) != 0 {
        let entries = e.mem.u32(palette + 0x14);
        e.mem.set_u8(entries + 3, 0xff);
    }
    true
}

/// What `ConvertPixelDataFormat` does after its format tests.
enum Conversion {
    /// Refused: false.
    Refuse,
    /// Copied straight, then `then`.
    Copy(AfterCopy),
    /// Through the row converter at this address.
    Convert(u32),
}

/// The palette step after a straight palettized copy.
enum AfterCopy {
    /// `copy_palette_opaque`.
    Opaque,
    /// A new palette of this many entries (the source must have a palette).
    Entries(u32),
}

/// The converter for a source matched to a pair of static formats: by
/// destination bits per pixel among `(depth, converter)`.
fn by_depth(depth: u32, table: &[(u32, u32)]) -> Conversion {
    table
        .iter()
        .find(|(d, _)| *d == depth)
        .map_or(Conversion::Refuse, |(_, c)| Conversion::Convert(*c))
}

// Translated from 00a7a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::ConvertPixelDataFormat` (engine map name; virtual,
/// `thiscall`, `RET 0xc`: the destination data, the source data and a level,
/// -1 for all). The level-0 sizes and the face counts must match, and the
/// level(s) exist in both (all: the destination's levels, which the source
/// must have). Equal formats copy the levels straight (`memcpy`) and the
/// palette (assigned, or a new copy). Otherwise (both `PixelBits` made; a
/// compressed destination, 4..6, refuses; a 24-bit destination needs the
/// green mask 0xff00 and a red one of 0xff or 0xff0000) the source format
/// picks the way: the palettized `011aa180`/`011aa1c8`/`011aa210`/`011aa258`
/// (a palette required) convert to a format-0/1 destination by its bits per
/// pixel, or copy straight to their partner format (then the palette:
/// opaque copy, or a new 16- or 256-entry one); `011aa2e8`, `011aa2a0` (format
/// 0/1), `011aa408`, `011aa450` (format 8/9) convert by bits per pixel;
/// `011aa330`/`011aa3c0` convert into each other; any other format-0/1
/// source converts into `011aa2a0` (16 and 32 bits by whether the source has
/// an alpha mask) or `011aa2e8`. The row converter runs for each face and
/// level with the width, height and size (of the destination), both level
/// addresses, the source palette and both `PixelBits`. Answers whether it
/// converted.
pub fn ni_dev_image_converter_convert_pixel_data_format(
    e: &mut Engine,
    _this: Ptr,
    destination: u32,
    source: u32,
    level: u32,
) -> bool {
    let first_width = e.mem.u32(e.mem.u32(source + DATA_WIDTHS));
    if first_width != e.mem.u32(e.mem.u32(destination + DATA_WIDTHS)) {
        return false;
    }
    if e.mem.u32(e.mem.u32(source + DATA_HEIGHTS))
        != e.mem.u32(e.mem.u32(destination + DATA_HEIGHTS))
    {
        return false;
    }
    let faces = e.mem.u32(source + DATA_FACES);
    if faces != e.mem.u32(destination + DATA_FACES) {
        return false;
    }
    let (first, last) = if level == u32::MAX {
        let levels = e.mem.u32(destination + DATA_LEVELS);
        if e.mem.u32(source + DATA_LEVELS) < levels {
            return false;
        }
        (0, levels.wrapping_sub(1))
    } else {
        if level >= e.mem.u32(source + DATA_LEVELS) || level >= e.mem.u32(destination + DATA_LEVELS)
        {
            return false;
        }
        (level, level)
    };
    if format_is(e, source + 8, destination + 8) {
        copy_levels(e, destination, source, faces, first, last);
        let source_palette = e.mem.u32(source + DATA_PALETTE);
        if source_palette != 0 {
            copy_palette(e, destination, source_palette);
        }
        return true;
    }
    e.with_stack(0x38, |e, frame| {
        // The frame: the source's `PixelBits` (+8) and the destination's
        // (+0x20).
        let source_bits = frame.addr() + 8;
        let destination_bits = frame.addr() + 0x20;
        let source_kind = e.mem.u32(source + 0xc);
        let depth = u32::from(e.mem.u8(destination + 9));
        let kind = e.mem.u32(destination + 0xc);
        let source_depth = u32::from(e.mem.u8(source + 9));
        ni_dev_image_converter_pixel_bits(e, Ptr::new(destination_bits), Ptr::new(destination + 8));
        ni_dev_image_converter_pixel_bits(e, Ptr::new(source_bits), Ptr::new(source + 8));
        if (4..=6).contains(&kind) {
            return false;
        }
        if depth == 0x18 {
            if e.mem.u32(destination_bits + 4) != 0xff00 {
                return false;
            }
            let red = e.mem.u32(destination_bits);
            if red != 0xff && red != 0x00ff_0000 {
                return false;
            }
        }
        let plain_kind = kind == 0 || kind == 1;
        let palette_kind = kind == 8 || kind == 9;
        let has_palette = |e: &Engine| e.mem.u32(source + DATA_PALETTE) != 0;
        let conversion = if format_is(e, source + 8, FORMAT_011AA180) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(depth, &[(0x18, 0x00a7_8f70), (0x20, 0x00a7_90a0)])
            } else if format_is(e, destination + 8, FORMAT_011AA1C8) {
                Conversion::Copy(AfterCopy::Opaque)
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA1C8) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(depth, &[(0x18, 0x00a7_8f70), (0x20, 0x00a7_9310)])
            } else if format_is(e, destination + 8, FORMAT_011AA180) {
                Conversion::Copy(AfterCopy::Entries(0x10))
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA210) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_95d0),
                        (0x18, 0x00a7_9e20),
                        (0x20, 0x00a7_98c0),
                    ],
                )
            } else if format_is(e, destination + 8, FORMAT_011AA258) {
                Conversion::Copy(AfterCopy::Opaque)
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA258) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_9ae0),
                        (0x18, 0x00a7_9e20),
                        (0x20, 0x00a7_9ef0),
                    ],
                )
            } else if format_is(e, destination + 8, FORMAT_011AA210) {
                Conversion::Copy(AfterCopy::Entries(0x100))
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA2E8) {
            if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_6ea0),
                        (0x18, 0x00a7_a160),
                        (0x20, 0x00a7_6f80),
                    ],
                )
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA2A0) {
            if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_7040),
                        (0x18, 0x00a7_7110),
                        (0x20, 0x00a7_71d0),
                    ],
                )
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA408) {
            if palette_kind {
                by_depth(depth, &[(0x10, 0x00a7_7880), (0x20, 0x00a7_7800)])
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA450) {
            if palette_kind {
                by_depth(depth, &[(0x10, 0x00a7_7740), (0x20, 0x00a7_76b0)])
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA330)
            && format_is(e, destination + 8, FORMAT_011AA3C0)
        {
            Conversion::Convert(0x00a7_6d70)
        } else if format_is(e, source + 8, FORMAT_011AA3C0)
            && format_is(e, destination + 8, FORMAT_011AA330)
        {
            Conversion::Convert(0x00a7_6e90)
        } else if source_kind != 0 && source_kind != 1 {
            Conversion::Refuse
        } else if format_is(e, destination + 8, FORMAT_011AA2A0) {
            let alpha = e.mem.u32(source_bits + 0xc) != 0;
            match source_depth {
                0x10 => Conversion::Convert(if alpha { 0x00a7_7330 } else { 0x00a7_73d0 }),
                0x18 => Conversion::Convert(0x00a7_7460),
                0x20 => Conversion::Convert(if alpha { 0x00a7_7590 } else { 0x00a7_7630 }),
                _ => Conversion::Refuse,
            }
        } else if format_is(e, destination + 8, FORMAT_011AA2E8) {
            by_depth(
                source_depth,
                &[
                    (0x10, 0x00a7_72b0),
                    (0x18, 0x00a7_a1f0),
                    (0x20, 0x00a7_7510),
                ],
            )
        } else {
            Conversion::Refuse
        };
        match conversion {
            Conversion::Refuse => false,
            Conversion::Copy(after) => {
                copy_levels(e, destination, source, faces, first, last);
                match after {
                    AfterCopy::Opaque => copy_palette_opaque(e, destination, source),
                    AfterCopy::Entries(count) => {
                        let source_palette = e.mem.u32(source + DATA_PALETTE);
                        if source_palette == 0 {
                            return false;
                        }
                        palette_from_entries(e, destination, source_palette, count);
                        true
                    }
                }
            }
            Conversion::Convert(converter) => {
                let mut face = 0;
                while face < faces {
                    let mut level = first;
                    while level <= last {
                        let offsets = e.mem.u32(source + DATA_OFFSETS);
                        let all = e.mem.u32(source + DATA_LEVELS);
                        let from = e
                            .mem
                            .u32(offsets + 4 * all)
                            .wrapping_mul(face)
                            .wrapping_add(e.mem.u32(offsets + 4 * level))
                            .wrapping_add(e.mem.u32(source + DATA_PIXELS));
                        let palette = e.mem.u32(source + DATA_PALETTE);
                        let offsets = e.mem.u32(destination + DATA_OFFSETS);
                        let all = e.mem.u32(destination + DATA_LEVELS);
                        let to = e
                            .mem
                            .u32(offsets + 4 * all)
                            .wrapping_mul(face)
                            .wrapping_add(e.mem.u32(offsets + 4 * level))
                            .wrapping_add(e.mem.u32(destination + DATA_PIXELS));
                        let size = e
                            .mem
                            .u32(offsets + 4 * (level + 1))
                            .wrapping_sub(e.mem.u32(offsets + 4 * level));
                        let height = e.mem.u32(e.mem.u32(destination + DATA_HEIGHTS) + 4 * level);
                        let width = e.mem.u32(e.mem.u32(destination + DATA_WIDTHS) + 4 * level);
                        e.call(
                            converter,
                            &args![
                                width,
                                height,
                                size,
                                to,
                                palette,
                                destination_bits,
                                from,
                                source_bits
                            ],
                        );
                        level += 1;
                    }
                    face += 1;
                }
                true
            }
        }
    })
}

// The row converters below take the same eight `cdecl` words as `00a76f80` (width, height,
// the level's size, the destination, the source palette, the destination's `PixelBits`, the
// source and the source's `PixelBits`) and walk `height` rows of `width` pixels.

/// `PixelBits` component `index`: its mask (+4 index), its shift byte (+0x10 + index) and its
/// `8 - bits` byte (+0x14 + index).
fn bits_mask(e: &Engine, bits: u32, index: u32) -> u32 {
    e.mem.u32(bits + 4 * index)
}

fn bits_shift(e: &Engine, bits: u32, index: u32) -> u8 {
    e.mem.u8(bits + 0x10 + index)
}

fn bits_right(e: &Engine, bits: u32, index: u32) -> u8 {
    e.mem.u8(bits + 0x14 + index)
}

/// 8-bit `SHL`/`SHR` and 16-bit `SHL`: x86 masks the count to five bits, so a count past the
/// operand's width shifts everything out.
fn shl8(value: u8, count: u8) -> u8 {
    (u32::from(value) << (count & 0x1f)) as u8
}

fn shr8(value: u8, count: u8) -> u8 {
    (u32::from(value) >> (count & 0x1f)) as u8
}

fn shl16(value: u16, count: u8) -> u16 {
    (u32::from(value) << (count & 0x1f)) as u16
}

/// Component `index` of a source pixel as 8 bits: masked, moved down by its shift (32-bit
/// `SHR`), then up by its `8 - bits` (8-bit `SHL`).
fn expand(e: &Engine, bits: u32, index: u32, pixel: u32) -> u8 {
    let low =
        (pixel & bits_mask(e, bits, index)).wrapping_shr(u32::from(bits_shift(e, bits, index)));
    shl8(low as u8, bits_right(e, bits, index))
}

/// An 8-bit value as component `index` of a 16-bit pixel: down by its `8 - bits` (8-bit
/// `SHR`), up by its shift (16-bit `SHL`), masked by the low half of its mask.
fn place16(e: &Engine, bits: u32, index: u32, value: u8) -> u16 {
    let narrowed = shr8(value, bits_right(e, bits, index));
    shl16(u16::from(narrowed), bits_shift(e, bits, index)) & bits_mask(e, bits, index) as u16
}

// Translated from 00a76d70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): the row
/// converter `ConvertPixelDataFormat` uses from `011aa330` to `011aa3c0`. Each 16-bit source
/// pixel's four components are expanded to 8 bits through the source's `PixelBits` and placed
/// into a 16-bit pixel through the destination's (components 3, 2, 0, 1, OR-ed).
pub fn fn_00a76d70(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let pixel = u32::from(e.mem.u16(from));
            let mut word = place16(e, destination_bits, 3, expand(e, source_bits, 3, pixel));
            word |= place16(e, destination_bits, 2, expand(e, source_bits, 2, pixel));
            from = from.wrapping_add(2);
            to = to.wrapping_add(2);
            word |= place16(e, destination_bits, 0, expand(e, source_bits, 0, pixel));
            word |= place16(e, destination_bits, 1, expand(e, source_bits, 1, pixel));
            e.mem.set_u16(to.wrapping_sub(2), word);
        }
    }
}

// Translated from 00a76e90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): a jump to [`fn_00a76d70`] with the same
/// eight words, the converter `ConvertPixelDataFormat` uses from `011aa3c0` to `011aa330`.
pub fn fn_00a76e90(
    e: &mut Engine,
    width: u32,
    height: u32,
    size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    fn_00a76d70(
        e,
        width,
        height,
        size,
        destination,
        palette,
        destination_bits,
        source,
        source_bits,
    );
}

// Translated from 00a76ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from `011aa2e8` to a 16-bit destination. Three
/// source bytes become components 0, 1 and 2 of a 16-bit pixel (each through the destination's
/// `PixelBits`, as `place16`), component 3 is `0xff` moved by the destination's 32-bit shifts
/// and masked by its 16-bit mask.
pub fn fn_00a76ea0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = e.mem.u8(from);
            let second = e.mem.u8(from.wrapping_add(1));
            let third = e.mem.u8(from.wrapping_add(2));
            from = from.wrapping_add(3);
            let mut word = channel(e, destination_bits, 3, 0xff) as u16;
            word |= place16(e, destination_bits, 0, first);
            to = to.wrapping_add(2);
            word |= place16(e, destination_bits, 2, third);
            word |= place16(e, destination_bits, 1, second);
            e.mem.set_u16(to.wrapping_sub(2), word);
        }
    }
}

// Translated from 00a77040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from `011aa2a0` to a 16-bit destination. Four
/// source bytes become components 0 to 3 of a 16-bit pixel through the destination's
/// `PixelBits` (as `place16`).
pub fn fn_00a77040(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = e.mem.u8(from);
            let second = e.mem.u8(from.wrapping_add(1));
            let third = e.mem.u8(from.wrapping_add(2));
            let fourth = e.mem.u8(from.wrapping_add(3));
            from = from.wrapping_add(4);
            let mut word = place16(e, destination_bits, 1, second);
            word |= place16(e, destination_bits, 0, first);
            to = to.wrapping_add(2);
            word |= place16(e, destination_bits, 3, fourth);
            word |= place16(e, destination_bits, 2, third);
            e.mem.set_u16(to.wrapping_sub(2), word);
        }
    }
}

/// Copies three bytes of each 4-byte (`stride` 4) or 3-byte source pixel into a 3-byte
/// destination pixel, in order or reversed, then the given fourth byte when `fill` is set.
fn copy_bytes(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    source: u32,
    stride: u32,
    reversed: bool,
    fill: Option<u8>,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let order = if reversed { [2, 1, 0] } else { [0, 1, 2] };
            for (index, byte) in order.into_iter().enumerate() {
                let value = e.mem.u8(from.wrapping_add(byte));
                e.mem.set_u8(to.wrapping_add(index as u32), value);
            }
            to = to.wrapping_add(3);
            if let Some(value) = fill {
                e.mem.set_u8(to, value);
                to = to.wrapping_add(1);
            }
            from = from.wrapping_add(stride);
        }
    }
}

// Translated from 00a77110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa2a0` to a
/// 24-bit destination. Only for a destination mask 1 of `0xff00`: with mask 0 `0xff0000` each
/// 4-byte source pixel's bytes 2, 1, 0 are copied, with mask 0 `0xff` bytes 0, 1, 2; any other
/// mask copies nothing.
pub fn fn_00a77110(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    if bits_mask(e, destination_bits, 1) != 0xff00 {
        return;
    }
    match bits_mask(e, destination_bits, 0) {
        0x00ff_0000 => copy_bytes(e, width, height, destination, source, 4, true, None),
        0xff => copy_bytes(e, width, height, destination, source, 4, false, None),
        _ => {}
    }
}

/// Each source pixel (`pixel_size` bytes, read as a 16- or 32-bit word) expanded to bytes:
/// components 0, 1, 2 (and 3 when `four` is set) through the source's `PixelBits`, then `fill`
/// as a fourth byte when given.
fn expand_pixels(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    source: u32,
    source_bits: u32,
    pixel_size: u32,
    four: bool,
    fill: Option<u8>,
) {
    let mut to = destination;
    let mut from = source;
    let read = |e: &Engine, from: u32| {
        if pixel_size == 2 {
            u32::from(e.mem.u16(from))
        } else {
            e.mem.u32(from)
        }
    };
    for _ in 0..height {
        for _ in 0..width {
            let count = if four { 4 } else { 3 };
            for index in 0..count {
                let pixel = read(e, from);
                let value = expand(e, source_bits, index, pixel);
                e.mem.set_u8(to.wrapping_add(index), value);
            }
            if let Some(value) = fill {
                e.mem.set_u8(to.wrapping_add(3), value);
            }
            to = to.wrapping_add(if four || fill.is_some() { 4 } else { 3 });
            from = from.wrapping_add(pixel_size);
        }
    }
}

// Translated from 00a772b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 16-bit source to
/// `011aa2e8`: components 0, 1 and 2 of each 16-bit pixel expanded to three bytes through the
/// source's `PixelBits`.
pub fn fn_00a772b0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        2,
        false,
        None,
    );
}

// Translated from 00a77330 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 16-bit source
/// with alpha to `011aa2a0`: components 0 to 3 of each 16-bit pixel expanded to four bytes.
pub fn fn_00a77330(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        2,
        true,
        None,
    );
}

// Translated from 00a773d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 16-bit source
/// without alpha to `011aa2a0`: components 0, 1 and 2 expanded to three bytes, the fourth
/// `0xff`.
pub fn fn_00a773d0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        2,
        false,
        Some(0xff),
    );
}

// Translated from 00a77460 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 24-bit source to
/// `011aa2a0`. Only for a source mask 1 of `0xff00`: with source mask 0 `0xff0000` each 3-byte
/// pixel's bytes 2, 1, 0 are copied, with any other mask 0 bytes 0, 1, 2; the fourth byte is
/// `0xff`.
pub fn fn_00a77460(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    if bits_mask(e, source_bits, 1) != 0xff00 {
        return;
    }
    let reversed = bits_mask(e, source_bits, 0) == 0x00ff_0000;
    copy_bytes(
        e,
        width,
        height,
        destination,
        source,
        3,
        reversed,
        Some(0xff),
    );
}

// Translated from 00a77510 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 32-bit source to
/// `011aa2e8`: components 0, 1 and 2 of each 32-bit pixel expanded to three bytes.
pub fn fn_00a77510(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        4,
        false,
        None,
    );
}

// Translated from 00a77590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 32-bit source
/// with alpha to `011aa2a0`: components 0 to 3 of each 32-bit pixel expanded to four bytes.
pub fn fn_00a77590(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        4,
        true,
        None,
    );
}

// Translated from 00a77630 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 32-bit source
/// without alpha to `011aa2a0`: components 0, 1 and 2 expanded to three bytes, the fourth
/// `0xff`.
pub fn fn_00a77630(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    expand_pixels(
        e,
        width,
        height,
        destination,
        source,
        source_bits,
        4,
        false,
        Some(0xff),
    );
}

// Translated from 00a776b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa450` to a
/// 32-bit destination. Of each 4-byte source pixel, byte 0 becomes component 0, byte 2
/// component 3 and byte 1 component 1 of a 32-bit pixel (each through the destination's 32-bit
/// shifts and mask, as `channel`); byte 3 is not read.
pub fn fn_00a776b0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from.wrapping_add(1)));
            let third = u32::from(e.mem.u8(from.wrapping_add(2)));
            from = from.wrapping_add(4);
            let word = channel(e, destination_bits, 0, first)
                | channel(e, destination_bits, 3, third)
                | channel(e, destination_bits, 1, second);
            to = to.wrapping_add(4);
            e.mem.set_u32(to.wrapping_sub(4), word);
        }
    }
}

// Translated from 00a77740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from `011aa450` to a 16-bit destination. Of
/// each 4-byte source pixel, byte 2 becomes component 3, byte 1 component 1 and byte 0
/// component 0 of a 16-bit pixel (as `place16`); byte 3 is not read.
pub fn fn_00a77740(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = e.mem.u8(from);
            let second = e.mem.u8(from.wrapping_add(1));
            let third = e.mem.u8(from.wrapping_add(2));
            let mut word = place16(e, destination_bits, 3, third);
            word |= place16(e, destination_bits, 1, second);
            let low = place16(e, destination_bits, 0, first);
            from = from.wrapping_add(4);
            to = to.wrapping_add(2);
            word |= low;
            e.mem.set_u16(to.wrapping_sub(2), word);
        }
    }
}

// Translated from 00a77800 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa408` to a
/// 32-bit destination. Of each 2-byte source pixel, byte 0 becomes component 0 and byte 1
/// component 1 (as `channel`); component 3 is `0xffffffff` moved by the destination's shifts
/// and masked.
pub fn fn_00a77800(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from.wrapping_add(1)));
            from = from.wrapping_add(2);
            let word = channel(e, destination_bits, 0, first)
                | channel(e, destination_bits, 1, second)
                | channel(e, destination_bits, 3, 0xffff_ffff);
            to = to.wrapping_add(4);
            e.mem.set_u32(to.wrapping_sub(4), word);
        }
    }
}

// Translated from 00a77880 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa408` to a
/// 16-bit destination. Of each 2-byte source pixel, byte 0 becomes component 0 and byte 1
/// component 1 (as `place16`); component 3 is `0xffffffff` moved by the destination's 32-bit
/// shifts and masked by its 16-bit mask.
pub fn fn_00a77880(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = e.mem.u8(from);
            let second = e.mem.u8(from.wrapping_add(1));
            let mut word = place16(e, destination_bits, 0, first);
            word |= place16(e, destination_bits, 1, second);
            from = from.wrapping_add(2);
            word |= channel(e, destination_bits, 3, 0xffff_ffff) as u16;
            to = to.wrapping_add(2);
            e.mem.set_u16(to.wrapping_sub(2), word);
        }
    }
}

// Translated from 00a7a160 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa2e8` to a
/// 24-bit destination. Only for a destination mask 1 of `0xff00`: with mask 0 `0xff0000` each
/// 3-byte pixel's bytes are copied reversed; with mask 0 `0xff` the whole level
/// (`width * height * 3` bytes) is copied by `memcpy`; any other mask copies nothing.
pub fn fn_00a7a160(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    copy_or_reverse_24(e, width, height, destination, source, destination_bits);
}

// Translated from 00a7a1f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from a 24-bit source to
/// `011aa2e8`: as [`fn_00a7a160`], but the masks tested are the source's.
pub fn fn_00a7a1f0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    _unused_destination_bits: u32,
    source: u32,
    source_bits: u32,
) {
    copy_or_reverse_24(e, width, height, destination, source, source_bits);
}

/// The body of `00a7a160` and `00a7a1f0` with the `PixelBits` whose masks they test.
fn copy_or_reverse_24(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    source: u32,
    bits: u32,
) {
    if bits_mask(e, bits, 1) != 0xff00 {
        return;
    }
    match bits_mask(e, bits, 0) {
        0x00ff_0000 => copy_bytes(e, width, height, destination, source, 3, true, None),
        0xff => {
            let size = width.wrapping_mul(height).wrapping_mul(3);
            e.call(MEMCPY_00EC44D0, &args![destination, source, size]);
        }
        _ => {}
    }
}

/// `00aa1070` and `00aa10f0` (`cdecl`, one word): the byte buffer allocation and its free.
const BUFFER_ALLOCATE: u32 = 0x00aa_1070;
const BUFFER_FREE: u32 = 0x00aa_10f0;
/// `NiPalette`'s entries (+0x14): four bytes each, components 0 to 3.
const PALETTE_ENTRIES: u32 = 0x14;

/// The 4-bit source of a 16-entry palette unpacked: a buffer of `width * height` bytes from
/// `00aa1070`, of which the first `(width >> 1) * height` source bytes fill two each (high
/// nibble first). For an odd width the last bytes stay as the allocation left them.
fn unpack_nibbles(e: &mut Engine, width: u32, height: u32, source: u32) -> u32 {
    let buffer = e
        .call(BUFFER_ALLOCATE, &args![width.wrapping_mul(height)])
        .u32();
    let count = (width >> 1).wrapping_mul(height);
    let mut to = buffer;
    for index in 0..count {
        let byte = e.mem.u8(source.wrapping_add(index));
        e.mem.set_u8(to, byte >> 4);
        let byte = e.mem.u8(source.wrapping_add(index));
        e.mem.set_u8(to.wrapping_add(1), byte & 0xf);
        to = to.wrapping_add(2);
    }
    buffer
}

/// Palette entry `index` as a 32-bit pixel: components 0, 1 and 2 through the destination's
/// `channel`, component 3 from the entry's fourth byte or, without `palette_alpha`, from `0xff`.
fn palette_pixel32(e: &Engine, entries: u32, index: u32, bits: u32, palette_alpha: bool) -> u32 {
    let entry = entries.wrapping_add(4 * index);
    let alpha = if palette_alpha {
        u32::from(e.mem.u8(entry.wrapping_add(3)))
    } else {
        0xff
    };
    channel(e, bits, 0, u32::from(e.mem.u8(entry)))
        | channel(e, bits, 2, u32::from(e.mem.u8(entry.wrapping_add(2))))
        | channel(e, bits, 1, u32::from(e.mem.u8(entry.wrapping_add(1))))
        | channel(e, bits, 3, alpha)
}

/// Palette entry `index` as a 16-bit pixel: components 0, 1 and 2 as `place16`, component 3
/// from the entry's fourth byte (`place16`) or, without `palette_alpha`, `0xff` moved by the
/// 32-bit shifts and masked by the 16-bit mask.
fn palette_pixel16(e: &Engine, entries: u32, index: u32, bits: u32, palette_alpha: bool) -> u16 {
    let entry = entries.wrapping_add(4 * index);
    let alpha = if palette_alpha {
        place16(e, bits, 3, e.mem.u8(entry.wrapping_add(3)))
    } else {
        channel(e, bits, 3, 0xff) as u16
    };
    place16(e, bits, 2, e.mem.u8(entry.wrapping_add(2)))
        | place16(e, bits, 0, e.mem.u8(entry))
        | place16(e, bits, 1, e.mem.u8(entry.wrapping_add(1)))
        | alpha
}

/// The 32-bit pixels of a 4-bit palettized level through a 16-entry table built on the stack:
/// each buffer byte picks its table word (a byte past 15, left by an odd width, reads past the
/// table as the exe reads past it on its stack). The buffer is freed after.
fn nibble_table_pixels(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    palette: u32,
    bits: u32,
    source: u32,
    palette_alpha: bool,
) {
    let entries = e.mem.u32(palette.wrapping_add(PALETTE_ENTRIES));
    let buffer = unpack_nibbles(e, width, height, source);
    e.with_stack(0x40, |e, table| {
        for index in 0..0x10 {
            let word = palette_pixel32(e, entries, index, bits, palette_alpha);
            e.mem.set_u32(table.addr() + 4 * index, word);
        }
        let mut to = destination;
        let mut from = buffer;
        for _ in 0..height {
            for _ in 0..width {
                let index = u32::from(e.mem.u8(from));
                let word = e.mem.u32(table.addr().wrapping_add(4 * index));
                e.mem.set_u32(to, word);
                to = to.wrapping_add(4);
                from = from.wrapping_add(1);
            }
        }
    });
    e.call(BUFFER_FREE, &args![buffer]);
}

// Translated from 00a78f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from the 4-bit palettized `011aa180`/`011aa1c8`
/// to a 24-bit destination. Only for a destination mask 1 of `0xff00`: the source is unpacked
/// to one byte per pixel (`00aa1070`), then with destination mask 0 `0xff` each pixel gets its
/// palette entry's bytes 0, 1, 2, with mask 0 `0xff0000` bytes 2, 1, 0 (any other mask writes
/// nothing); the buffer is freed (`00aa10f0`). The palette's entries (+0x14) are read first.
pub fn fn_00a78f70(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let entries = e.mem.u32(palette.wrapping_add(PALETTE_ENTRIES));
    if bits_mask(e, destination_bits, 1) != 0xff00 {
        return;
    }
    let buffer = unpack_nibbles(e, width, height, source);
    let order = match bits_mask(e, destination_bits, 0) {
        0xff => Some([0u32, 1, 2]),
        0x00ff_0000 => Some([2, 1, 0]),
        _ => None,
    };
    if let Some(order) = order {
        palette_bytes(e, width, height, destination, entries, buffer, order);
    }
    e.call(BUFFER_FREE, &args![buffer]);
}

/// Each index byte at `indices` gives three bytes of its palette entry, in `order`.
fn palette_bytes(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    entries: u32,
    indices: u32,
    order: [u32; 3],
) {
    let mut to = destination;
    let mut from = indices;
    for _ in 0..height {
        for _ in 0..width {
            for (offset, byte) in order.into_iter().enumerate() {
                let index = u32::from(e.mem.u8(from));
                let value = e.mem.u8(entries.wrapping_add(4 * index).wrapping_add(byte));
                e.mem.set_u8(to.wrapping_add(offset as u32), value);
            }
            to = to.wrapping_add(3);
            from = from.wrapping_add(1);
        }
    }
}

// Translated from 00a790a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from `011aa180` to a 32-bit destination: the
/// 4-bit source unpacked (`00aa1070`), a 16-word table of the palette's entries as destination
/// pixels (components 0 to 2 from the entry, component 3 from `0xff`), each pixel its table
/// word; the buffer freed (`00aa10f0`).
pub fn fn_00a790a0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    nibble_table_pixels(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        false,
    );
}

// Translated from 00a79310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the row converter from `011aa1c8` to a 32-bit destination: as
/// [`fn_00a790a0`], with component 3 from each palette entry's fourth byte.
pub fn fn_00a79310(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    nibble_table_pixels(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        true,
    );
}

/// The 16-bit pixels of an 8-bit palettized level through a 256-entry table of the palette's
/// entries (`palette_pixel16`).
fn byte_table_pixels16(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    palette: u32,
    bits: u32,
    source: u32,
    palette_alpha: bool,
) {
    let entries = e.mem.u32(palette.wrapping_add(PALETTE_ENTRIES));
    let table: Vec<u16> = (0..0x100)
        .map(|index| palette_pixel16(e, entries, index, bits, palette_alpha))
        .collect();
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let index = usize::from(e.mem.u8(from));
            e.mem.set_u16(to, table[index]);
            to = to.wrapping_add(2);
            from = from.wrapping_add(1);
        }
    }
}

/// The 32-bit pixels of an 8-bit palettized level through a 256-entry table of the palette's
/// entries (`palette_pixel32`).
fn byte_table_pixels32(
    e: &mut Engine,
    width: u32,
    height: u32,
    destination: u32,
    palette: u32,
    bits: u32,
    source: u32,
    palette_alpha: bool,
) {
    let entries = e.mem.u32(palette.wrapping_add(PALETTE_ENTRIES));
    let table: Vec<u32> = (0..0x100)
        .map(|index| palette_pixel32(e, entries, index, bits, palette_alpha))
        .collect();
    let mut to = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let index = usize::from(e.mem.u8(from));
            e.mem.set_u32(to, table[index]);
            to = to.wrapping_add(4);
            from = from.wrapping_add(1);
        }
    }
}

// Translated from 00a795d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a795d0 - the /GS cookie check (011c16bc, 00ec408c) guarding the 0x200-byte table is not modelled
/// No function in the engine map (Ghidra made none): the row converter from the 8-bit
/// palettized `011aa210` to a 16-bit destination: a 256-entry table of the palette's entries
/// as 16-bit pixels (components 0 to 2 from the entry, component 3 from `0xff`), each pixel
/// its table word.
pub fn fn_00a795d0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    byte_table_pixels16(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        false,
    );
}

// Translated from 00a798c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa210` to a
/// 32-bit destination: a 256-entry table of the palette's entries as 32-bit pixels (component 3
/// from `0xff`), each pixel its table word.
pub fn fn_00a798c0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    byte_table_pixels32(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        false,
    );
}

// Translated from 00a79ae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a79ae0 - the /GS cookie check (011c16bc, 00ec408c) guarding the 0x200-byte table is not modelled
/// No function in the engine map (Ghidra made none): the row converter from `011aa258` to a
/// 16-bit destination: as [`fn_00a795d0`], with component 3 from each entry's fourth byte.
pub fn fn_00a79ae0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    byte_table_pixels16(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        true,
    );
}

// Translated from 00a79e20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa210` or
/// `011aa258` to a 24-bit destination. The palette's entries (+0x14) are read first; only for
/// a destination mask 1 of `0xff00`: with mask 0 `0xff` each index byte gives its entry's
/// bytes 0, 1, 2, with `0xff0000` bytes 2, 1, 0; any other mask writes nothing.
pub fn fn_00a79e20(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let entries = e.mem.u32(palette.wrapping_add(PALETTE_ENTRIES));
    if bits_mask(e, destination_bits, 1) != 0xff00 {
        return;
    }
    match bits_mask(e, destination_bits, 0) {
        0xff => palette_bytes(e, width, height, destination, entries, source, [0, 1, 2]),
        0x00ff_0000 => palette_bytes(e, width, height, destination, entries, source, [2, 1, 0]),
        _ => {}
    }
}

// Translated from 00a79ef0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none): the row converter from `011aa258` to a
/// 32-bit destination: as [`fn_00a798c0`], with component 3 from each entry's fourth byte.
pub fn fn_00a79ef0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    palette: u32,
    destination_bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    byte_table_pixels32(
        e,
        width,
        height,
        destination,
        palette,
        destination_bits,
        source,
        true,
    );
}

// Translated from 00a76d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::IsASupportedMipmapGenerationFormat` (Xbox PDB): whether the format
/// passes the converter's alpha test (its slot +0x20) or, failing that, its non-alpha test
/// (slot +0x24).
pub fn ni_dev_image_converter_is_a_supported_mipmap_generation_format(
    e: &mut Engine,
    this: Ptr,
    format: u32,
) -> bool {
    e.vcall(this.addr(), 0x20, &args![format]).bool()
        || e.vcall(this.addr(), 0x24, &args![format]).bool()
}

// Translated from 00a77920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): whether the value is a power of two (0 is
/// not; only `AL` is set for 0).
pub fn fn_00a77920(_e: &mut Engine, value: u32) -> bool {
    if value == 0 {
        return false;
    }
    value.wrapping_sub(1) & value == 0
}

// Translated from 00a77940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPixelFormat`, no stack words): 1 when its
/// format word (+4) is 2 or 3, else 0.
pub fn fn_00a77940(e: &mut Engine, this: Ptr) -> u32 {
    let format = e.mem.u32(this.addr() + 4);
    u32::from(format == 2 || format == 3)
}

// Translated from 00a77960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPixelFormat`, no stack words): 1 when its
/// format word (+4, signed) is 4 to 6, else 0.
pub fn fn_00a77960(e: &mut Engine, this: Ptr) -> u32 {
    let format = e.mem.u32(this.addr() + 4) as i32;
    u32::from((4..=6).contains(&format))
}

// Translated from 00a77ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a77ae0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiDevImageConverter::CanReadImageFile` (Xbox PDB): the name copied into a 0x104-byte
/// buffer (`_strcpy_s`), standardized and split (`NiFilename`); the first reader of the list
/// at +0x990 whose slot +4 accepts the extension decides: the file from
/// `NiFile::GetFile(path, 0, 0x8000)` (none: false) answers through its slot +4, and is
/// deleted (slot 0 with 1) either way. No reader: false.
pub fn ni_dev_image_converter_can_read_image_file(e: &mut Engine, this: Ptr, name: u32) -> bool {
    let mut item = e.mem.u32(this.addr() + READERS);
    e.with_stack(0x508, |e, frame| {
        // The frame: the split name (+0, 0x404 bytes), the path (+0x404, 0x104 bytes).
        let filename = frame.addr();
        let path = frame.addr() + 0x404;
        e.call(STRCPY_S, &args![path, 0x104u32, name]);
        e.call(PATH_STANDARDIZE, &args![path]);
        e.call(NI_FILENAME_CONSTRUCT, &args![filename, path]);
        while item != 0 {
            let reader = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if !e.vcall(reader, 4, &args![filename + 0x103]).bool() {
                continue;
            }
            let file = e
                .call(NI_FILE_GET_FILE, &args![path, 0u32, 0x8000u32])
                .u32();
            if file == 0 {
                return false;
            }
            let good = e.vcall(file, 4, &args![]).bool();
            e.vcall(file, 0, &args![1u32]);
            return good;
        }
        false
    })
}

// Translated from 00a77bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a77bd0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiDevImageConverter::ReadImageFileInfo` (Xbox PDB): the name standardized and split as in
/// `CanReadImageFile`; each reader of the list at +0x990 whose slot +4 accepts the extension
/// gets a file from `NiFile::GetFile(path, 0, 0x8000)` (none: false); a file whose slot +4
/// answers false is deleted and false answered; otherwise the reader's slot +0xc reads the
/// information with the file and the five words in the order `(file, word_3, word_4, word_1,
/// word_2, word_5)` (the pointers the information is written to; their meaning is not
/// confirmed here). A true answer deletes the file and answers true; false deletes it and goes
/// on. After the last reader: false.
pub fn ni_dev_image_converter_read_image_file_info(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) -> bool {
    let mut item = e.mem.u32(this.addr() + READERS);
    e.with_stack(0x518, |e, frame| {
        // The frame: the words 4, 3, 2 and 5 (+0..+0xc), the split name (+0x10), the path
        // (+0x414).
        let base = frame.addr();
        e.mem.set_u32(base + 8, word_2);
        e.mem.set_u32(base + 4, word_3);
        e.mem.set_u32(base, word_4);
        e.mem.set_u32(base + 0xc, word_5);
        let filename = base + 0x10;
        let path = base + 0x414;
        e.call(STRCPY_S, &args![path, 0x104u32, name]);
        e.call(PATH_STANDARDIZE, &args![path]);
        e.call(NI_FILENAME_CONSTRUCT, &args![filename, path]);
        while item != 0 {
            let reader = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if !e.vcall(reader, 4, &args![filename + 0x103]).bool() {
                continue;
            }
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
            let fifth = e.mem.u32(base + 0xc);
            let second = e.mem.u32(base + 8);
            let fourth = e.mem.u32(base);
            let third = e.mem.u32(base + 4);
            let read = e
                .vcall(
                    reader,
                    0xc,
                    &args![file, third, fourth, word_1, second, fifth],
                )
                .bool();
            e.vcall(file, 0, &args![1u32]);
            if read {
                return true;
            }
        }
        false
    })
}

// Translated from 00a78570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::ConvertForPlatform` (Xbox PDB): the closest format for the platform
/// (`FindClosestPixelFormat`, slot +0x3c, with the two words, the data's format at +8 and the
/// last word). The data itself when that is its own format (by address or `operator==`); null
/// when there is none or the conversion is impossible (`CanConvertPixelData`, slot +0x10, data
/// format to the found one); otherwise a new 0x74-byte `NiPixelData` (`00aa13e0`, constructed by
/// `00a7c190` with the level-0 width and height, the found format, the data's level and face
/// counts; null when the allocation fails) filled by `ConvertPixelDataFormat` (slot +0x30, all
/// levels): the new data when that converted, else null.
pub fn ni_dev_image_converter_convert_for_platform(
    e: &mut Engine,
    this: Ptr,
    platform: u32,
    word: u32,
    data: Ptr,
    last: u32,
) -> Ptr {
    let format = data.addr() + 8;
    let found = e
        .vcall(this.addr(), 0x3c, &args![platform, word, format, last])
        .u32();
    if found == format {
        return data;
    }
    if found == 0 {
        return Ptr::new(0);
    }
    if e.call(FORMAT_EQUALS, &args![found, format]).bool() {
        return data;
    }
    if !e.vcall(this.addr(), 0x10, &args![format, found]).bool() {
        return Ptr::new(0);
    }
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    let converted = if block != 0 {
        let faces = e.mem.u32(data.addr() + DATA_FACES);
        let levels = e.mem.u32(data.addr() + DATA_LEVELS);
        let height = e.mem.u32(e.mem.u32(data.addr() + DATA_HEIGHTS));
        let width = e.mem.u32(e.mem.u32(data.addr() + DATA_WIDTHS));
        e.call(
            NI_PIXEL_DATA_CONSTRUCT,
            &args![block, width, height, found, levels, faces],
        )
        .u32()
    } else {
        0
    };
    let done = e
        .vcall(this.addr(), 0x30, &args![converted, data, u32::MAX])
        .bool();
    Ptr::new(if done { converted } else { 0 })
}

// Translated from 00a78630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::FindClosestPixelFormat` (Xbox PDB): by the platform word (the
/// compiler's jump table as the `match`): 0 answers `011aa2a0` for a format (`format`) of
/// format word 2 or 3 or 16 bits per pixel when the request's first word is 4, and otherwise
/// `format` itself; 4, 5 and 6 pass the three other words to the Xenon (slot +0x4c), PS3
/// (+0x50) and DX9 (+0x48) searches; 1 to 3 and anything else answer 0.
pub fn ni_dev_image_converter_find_closest_pixel_format(
    e: &mut Engine,
    this: Ptr,
    platform: u32,
    request: u32,
    word: u32,
    format: u32,
) -> u32 {
    match platform {
        0 => {
            let kind = e.mem.u32(format + 4);
            if (kind == 2 || kind == 3 || e.mem.u8(format + 1) == 0x10) && e.mem.u32(request) == 4 {
                return FORMAT_011AA2A0;
            }
            format
        }
        4 => e
            .vcall(this.addr(), 0x4c, &args![request, word, format])
            .u32(),
        5 => e
            .vcall(this.addr(), 0x50, &args![request, word, format])
            .u32(),
        6 => e
            .vcall(this.addr(), 0x48, &args![request, word, format])
            .u32(),
        _ => 0,
    }
}

/// The static pixel formats the platform searches answer (`NiPixelFormat` globals in `.data`).
const FORMAT_011AA498: u32 = 0x011a_a498;
const FORMAT_011AA4E0: u32 = 0x011a_a4e0;
const FORMAT_011AA528: u32 = 0x011a_a528;
const FORMAT_011AA570: u32 = 0x011a_a570;
const FORMAT_011AA5B8: u32 = 0x011a_a5b8;
const FORMAT_011AA600: u32 = 0x011a_a600;
const FORMAT_011AA648: u32 = 0x011a_a648;
const FORMAT_011AA690: u32 = 0x011a_a690;
const FORMAT_011AA6D8: u32 = 0x011a_a6d8;
const FORMAT_011AA768: u32 = 0x011a_a768;
const FORMAT_011AA8D0: u32 = 0x011a_a8d0;
const FORMAT_011AA960: u32 = 0x011a_a960;
const FORMAT_011AA9F0: u32 = 0x011a_a9f0;
const FORMAT_011AAAC8: u32 = 0x011a_aac8;
const FORMAT_011AAB10: u32 = 0x011a_ab10;
const FORMAT_011AAB58: u32 = 0x011a_ab58;
const FORMAT_011AAD98: u32 = 0x011a_ad98;

/// The searches' answer by a format's word (+4) 4, 5 or 6; any other word goes by the
/// request's second word (+4): 2 gives `011aa528`, anything else `011aa498`.
fn by_format_word(e: &Engine, request: u32, format: u32) -> u32 {
    match e.mem.u32(format + 4) {
        4 => FORMAT_011AA498,
        5 => FORMAT_011AA4E0,
        6 => FORMAT_011AA528,
        _ => by_request_word(e, request),
    }
}

/// `011aa528` when the request's second word (+4) is 2, else `011aa498`.
fn by_request_word(e: &Engine, request: u32) -> u32 {
    if e.mem.u32(request + 4) == 2 {
        FORMAT_011AA528
    } else {
        FORMAT_011AA498
    }
}

/// The 16-bit alpha test of the DX9 and PS3 searches: for a format of word 2 or 3 whether the
/// word is 3, otherwise whether it has bits of component 3 ([`fn_00a77980`]); a compressed
/// format ([`fn_00a77960`]) always has.
fn has_alpha_by_calls(e: &mut Engine, format: u32) -> bool {
    let mut alpha = if fn_00a77940(e, Ptr::new(format)) as u8 != 0 {
        e.mem.u32(format + 4) == 3
    } else {
        fn_00a77980(e, Ptr::new(format), 3) != 0
    };
    if fn_00a77960(e, Ptr::new(format)) as u8 != 0 {
        alpha = true;
    }
    alpha
}

/// The inline alpha test of the Xenon and PS3 searches on `format`: for a format word (+4) of
/// 2 or 3 whether it is 3, otherwise whether the first component of kind 3 has bits (0 when
/// none has); a word of 4 to 6 (signed) always has.
fn has_alpha_inline(e: &Engine, format: u32) -> bool {
    let word = e.mem.u32(format + 4);
    let mut alpha = if word == 2 || word == 3 {
        word == 3
    } else {
        component_bits(e, format, 3) != 0
    };
    if (4..=6).contains(&(word as i32)) {
        alpha = true;
    }
    alpha
}

// Translated from 00a786e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::FindClosestPixelFormatDX9` (Xbox PDB; `thiscall`, `this` not read):
/// the static format for the request's first word (with its second word, +4, choosing among
/// variants), the format's own word and bits per pixel deciding for words 3 and 6; a word 6
/// request without a match answers `format` itself.
pub fn ni_dev_image_converter_find_closest_pixel_format_dx9(
    e: &mut Engine,
    _this: Ptr,
    request: u32,
    _unused_word: u32,
    format: u32,
) -> u32 {
    let kind = e.mem.u32(request);
    let variant = |e: &Engine| e.mem.u32(request + 4);
    match kind {
        7 => {
            if variant(e) == 0 {
                FORMAT_011AAD98
            } else {
                FORMAT_011AA768
            }
        }
        8 => FORMAT_011AA6D8,
        9 => FORMAT_011AA600,
        0xa => FORMAT_011AA648,
        0xb => FORMAT_011AA690,
        0xe => FORMAT_011AA570,
        0xd | 0xc => FORMAT_011AA5B8,
        2 => {
            if variant(e) == 0 {
                FORMAT_011AAB58
            } else {
                FORMAT_011AAAC8
            }
        }
        1 => match variant(e) {
            1 => FORMAT_011AA960,
            2 => FORMAT_011AA9F0,
            _ => FORMAT_011AA8D0,
        },
        0 | 5 => by_request_word(e, request),
        3 => by_format_word(e, request, format),
        4 => FORMAT_011AA450,
        6 => {
            if fn_00a77960(e, Ptr::new(format)) as u8 != 0
                || fn_00a77940(e, Ptr::new(format)) as u8 != 0
            {
                return by_format_word(e, request, format);
            }
            match e.mem.u8(format + 1) {
                0x10 => {
                    let alpha = has_alpha_by_calls(e, format);
                    match variant(e) {
                        1 => FORMAT_011AA960,
                        2 => FORMAT_011AA9F0,
                        0 => FORMAT_011AA8D0,
                        _ if alpha => FORMAT_011AA9F0,
                        _ => FORMAT_011AA8D0,
                    }
                }
                0x20 => match variant(e) {
                    0 => FORMAT_011AAB58,
                    1 | 2 => FORMAT_011AAAC8,
                    _ => match e.mem.u32(format + 4) {
                        0 => FORMAT_011AAB58,
                        1 => FORMAT_011AAAC8,
                        _ => format,
                    },
                },
                _ => format,
            }
        }
        _ => format,
    }
}

// Translated from 00a78910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::FindClosestPixelFormatXenon` (Xbox PDB; `thiscall`, `this` not read):
/// the alpha test of the second word's format first (`has_alpha_inline`), then by the
/// request's first word: 0xe `011aa570`, 0xd `011aa5b8`, 2 `011aaac8`, 1 `011aa9f0` with alpha
/// or `011aa8d0`, 0 and 5 `011aa528` with alpha or `011aa498`, 3 by the third word's format
/// word (4, 5, 6) or else by the alpha, 4 `011aa450`, 6 by the third word's format (formats of
/// word 2 to 6 as for 3; 16 bits as for 1, 64 bits `011aa5b8`, 128 bits `011aa570`), anything
/// else `011aaac8`.
pub fn ni_dev_image_converter_find_closest_pixel_format_xenon(
    e: &mut Engine,
    _this: Ptr,
    request: u32,
    alpha_format: u32,
    format: u32,
) -> u32 {
    let alpha = has_alpha_inline(e, alpha_format);
    let with_alpha = |alpha: bool, yes: u32, no: u32| if alpha { yes } else { no };
    let by_word = |word: u32| match word {
        4 => FORMAT_011AA498,
        5 => FORMAT_011AA4E0,
        6 => FORMAT_011AA528,
        _ => with_alpha(alpha, FORMAT_011AA528, FORMAT_011AA498),
    };
    match e.mem.u32(request) {
        0xe => FORMAT_011AA570,
        0xd => FORMAT_011AA5B8,
        2 => FORMAT_011AAAC8,
        1 => with_alpha(alpha, FORMAT_011AA9F0, FORMAT_011AA8D0),
        0 | 5 => with_alpha(alpha, FORMAT_011AA528, FORMAT_011AA498),
        3 => by_word(e.mem.u32(format + 4)),
        4 => FORMAT_011AA450,
        6 => {
            if fn_00a77940(e, Ptr::new(format)) as u8 != 0
                || fn_00a77960(e, Ptr::new(format)) as u8 != 0
            {
                return by_word(e.mem.u32(format + 4));
            }
            match e.mem.u8(format + 1) {
                0x10 => with_alpha(alpha, FORMAT_011AA9F0, FORMAT_011AA8D0),
                0x40 => FORMAT_011AA5B8,
                0x80 => FORMAT_011AA570,
                _ => FORMAT_011AAAC8,
            }
        }
        _ => FORMAT_011AAAC8,
    }
}

// Translated from 00a78a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::FindClosestPixelFormatPS3` (Xbox PDB; `thiscall`, `this` not read):
/// the alpha test of the second word's format (`has_alpha_inline`, used for request word 1
/// only), then by the request's first word: 0xe `011aa570`, 0xd and 0xc `011aa5b8`, 1
/// `011aa9f0` with alpha or `011aa8d0`, 0 and 5 by the request's second word, 3 by the third
/// word's format word (4, 5, 6) or else the request's second word, 6 by the third word's
/// format (formats of word 2 to 6 as for 3; 16 bits by the request's second word and the
/// format's own alpha test; others `011aab10`), anything else `011aab10`.
pub fn ni_dev_image_converter_find_closest_pixel_format_ps3(
    e: &mut Engine,
    _this: Ptr,
    request: u32,
    alpha_format: u32,
    format: u32,
) -> u32 {
    let alpha = has_alpha_inline(e, alpha_format);
    match e.mem.u32(request) {
        0xe => FORMAT_011AA570,
        0xd | 0xc => FORMAT_011AA5B8,
        1 => {
            if alpha {
                FORMAT_011AA9F0
            } else {
                FORMAT_011AA8D0
            }
        }
        0 | 5 => by_request_word(e, request),
        3 => by_format_word(e, request, format),
        6 => {
            if fn_00a77960(e, Ptr::new(format)) as u8 != 0
                || fn_00a77940(e, Ptr::new(format)) as u8 != 0
            {
                return by_format_word(e, request, format);
            }
            if e.mem.u8(format + 1) != 0x10 {
                return FORMAT_011AAB10;
            }
            let format_alpha = has_alpha_by_calls(e, format);
            match e.mem.u32(request + 4) {
                1 | 2 => FORMAT_011AA9F0,
                0 => FORMAT_011AA8D0,
                _ if format_alpha => FORMAT_011AA9F0,
                _ => FORMAT_011AA8D0,
            }
        }
        _ => FORMAT_011AAB10,
    }
}

/// A new 0x74-byte `NiPixelData` (`00aa13e0`, constructed by `00a7c190` with the width,
/// height, format, level count and one face); null when the allocation fails.
fn new_pixel_data(
    e: &mut Engine,
    width: u32,
    height: u32,
    format: u32,
    levels: u32,
    faces: u32,
) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        NI_PIXEL_DATA_CONSTRUCT,
        &args![block, width, height, format, levels, faces],
    )
    .u32()
}

/// Level `level`'s first pixel of a pixel data: its offset (`+0x5c`) plus the pixels (`+0x50`).
fn level_pixels(e: &Engine, data: u32, level: u32) -> u32 {
    let offset = e.mem.u32(
        e.mem
            .u32(data.wrapping_add(DATA_OFFSETS))
            .wrapping_add(4 * level),
    );
    offset.wrapping_add(e.mem.u32(data.wrapping_add(DATA_PIXELS)))
}

/// Level `level`'s width (`+0x54`) and height (`+0x58`).
fn level_size(e: &Engine, data: u32, level: u32) -> (u32, u32) {
    let width = e
        .mem
        .u32(e.mem.u32(data + DATA_WIDTHS).wrapping_add(4 * level));
    let height = e
        .mem
        .u32(e.mem.u32(data + DATA_HEIGHTS).wrapping_add(4 * level));
    (width, height)
}

// Translated from 00a78be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::GenerateBumpMap` (Xbox PDB; `thiscall`, `this` not read, `RET 8`: the
/// source data and a format). A source of format `011aa2a0` gives a new `011aa450` data whose
/// pixels take bytes 3 and 0 of each 4-byte source pixel into bytes 2 and 3; a source of
/// `011aa2e8` with the format `011aa450` gives one whose pixels get `0xff` and byte 0 of each
/// 3-byte pixel; a source of `011aa2e8` otherwise a new `011aa408` data (2 bytes a pixel); any
/// other source: null. (The new data has the source's level-0 size and level count, one face;
/// null when the allocation fails.) Then, for each level, bytes 0 and 1 of each pixel are the
/// differences of the source's first byte to the right and below (the last column takes the
/// difference to the left, 0 for a one-pixel row; the last row the difference to the row
/// above, 0 for a one-row level).
pub fn ni_dev_image_converter_generate_bump_map(
    e: &mut Engine,
    _this: Ptr,
    source: Ptr,
    format: u32,
) -> Ptr {
    let s = source.addr();
    let height0 = e.mem.u32(e.mem.u32(s + DATA_HEIGHTS));
    let levels = e.mem.u32(s + DATA_LEVELS);
    let width0 = e.mem.u32(e.mem.u32(s + DATA_WIDTHS));
    let bump;
    let stride: u32;
    let pixel: u32;
    if format_is(e, s + 8, FORMAT_011AA2A0) {
        bump = new_pixel_data(e, width0, height0, FORMAT_011AA450, levels, 1);
        for level in 0..levels {
            let (width, height) = level_size(e, s, level);
            let mut to = level_pixels(e, bump, level);
            let mut from = level_pixels(e, s, level);
            for _ in 0..height {
                for _ in 0..width {
                    let value = e.mem.u8(from.wrapping_add(3));
                    e.mem.set_u8(to.wrapping_add(2), value);
                    let value = e.mem.u8(from);
                    e.mem.set_u8(to.wrapping_add(3), value);
                    from = from.wrapping_add(4);
                    to = to.wrapping_add(4);
                }
            }
        }
        stride = 4;
        pixel = 4;
    } else if format_is(e, s + 8, FORMAT_011AA2E8) && format_is(e, format, FORMAT_011AA450) {
        bump = new_pixel_data(e, width0, height0, FORMAT_011AA450, levels, 1);
        for level in 0..levels {
            let (width, height) = level_size(e, s, level);
            let mut to = level_pixels(e, bump, level);
            let mut from = level_pixels(e, s, level);
            for _ in 0..height {
                for _ in 0..width {
                    e.mem.set_u8(to.wrapping_add(2), 0xff);
                    let value = e.mem.u8(from);
                    e.mem.set_u8(to.wrapping_add(3), value);
                    from = from.wrapping_add(3);
                    to = to.wrapping_add(4);
                }
            }
        }
        stride = 4;
        pixel = 3;
    } else if format_is(e, s + 8, FORMAT_011AA2E8) {
        bump = new_pixel_data(e, width0, height0, FORMAT_011AA408, levels, 1);
        stride = 2;
        pixel = 3;
    } else {
        return Ptr::new(0);
    }
    let byte = |e: &Engine, at: u32| e.mem.u8(at);
    for level in 0..levels {
        let mut out = level_pixels(e, bump, level);
        let width = e
            .mem
            .u32(e.mem.u32(s + DATA_WIDTHS).wrapping_add(4 * level));
        let mut here = level_pixels(e, s, level);
        let mut below = here.wrapping_add(pixel.wrapping_mul(width));
        let last_column = width.wrapping_sub(1);
        let rows = e
            .mem
            .u32(e.mem.u32(s + DATA_HEIGHTS).wrapping_add(4 * level))
            .wrapping_sub(1);
        let mut right = here.wrapping_add(pixel);
        let mut more_rows = 0;
        if rows != 0 {
            more_rows = rows;
            for _ in 0..rows {
                for _ in 0..last_column {
                    let centre = byte(e, here);
                    let down = byte(e, below).wrapping_sub(centre);
                    let across = byte(e, right).wrapping_sub(centre);
                    e.mem.set_u8(out, across);
                    e.mem.set_u8(out.wrapping_add(1), down);
                    out = out.wrapping_add(stride);
                    here = here.wrapping_add(pixel);
                    right = right.wrapping_add(pixel);
                    below = below.wrapping_add(pixel);
                }
                if last_column != 0 {
                    let left = byte(e, here).wrapping_sub(byte(e, here.wrapping_sub(pixel)));
                    e.mem.set_u8(out, left);
                } else {
                    e.mem.set_u8(out, 0);
                }
                let down = byte(e, below).wrapping_sub(byte(e, here));
                here = here.wrapping_add(pixel);
                e.mem.set_u8(out.wrapping_add(1), down);
                out = out.wrapping_add(stride);
                right = right.wrapping_add(pixel);
                below = below.wrapping_add(pixel);
            }
        }
        let mut above = here.wrapping_sub(last_column.wrapping_add(1).wrapping_mul(pixel));
        for _ in 0..last_column {
            let centre = byte(e, here);
            let up = centre.wrapping_sub(byte(e, above));
            let across = byte(e, right).wrapping_sub(centre);
            e.mem.set_u8(out, across);
            e.mem.set_u8(out.wrapping_add(1), up);
            out = out.wrapping_add(stride);
            above = above.wrapping_add(pixel);
            here = here.wrapping_add(pixel);
            right = right.wrapping_add(pixel);
        }
        if last_column != 0 {
            let left = byte(e, here).wrapping_sub(byte(e, here.wrapping_sub(pixel)));
            e.mem.set_u8(out, left);
        } else {
            e.mem.set_u8(out, 0);
        }
        if more_rows != 0 {
            let up = byte(e, here).wrapping_sub(byte(e, above));
            e.mem.set_u8(out.wrapping_add(1), up);
        } else {
            e.mem.set_u8(out.wrapping_add(1), 0);
        }
    }
    Ptr::new(bump)
}

// Translated from 00a7ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::GenerateMipmapPyramidFromFiles` (Xbox PDB; `thiscall`, `RET 0xc`: the
/// array of file names, the level count, an image for level 0 or null). Null when the first
/// name is null. The first file's information (`ReadImageFileInfo`, slot 8, into a local
/// `NiPixelFormat` from `00a7cca0` and locals for the width, height, face count and a byte)
/// must read with one face; then a new `NiPixelData` of that size and format with the given
/// level count (null when the allocation fails). Each level's image (the given one for level 0,
/// else `ReadImageFile`, slot 0xc, with the name, 0, 0) is taken into a counted local
/// (`00a5f860`); a missing name or image deletes the new data (slot 0 with 1) and answers null,
/// as does an image whose level-0 size differs from the level's (released, `00b66f20`).
/// Otherwise its level 0 is copied in (`memcpy`), for level 0 of a format of word 2 or 3
/// (`00a77940`) with the image's palette (+0x4c, `00a76a60`), and the image released.
pub fn ni_dev_image_converter_generate_mipmap_pyramid_from_files(
    e: &mut Engine,
    this: Ptr,
    names: u32,
    count: u32,
    given: u32,
) -> Ptr {
    if e.mem.u32(names) == 0 {
        return Ptr::new(0);
    }
    e.with_stack(0x5c, |e, frame| {
        // The frame: a byte (+3), the counted image (+4), the face count (+8), the height
        // (+0xc), the width (+0x10), `this` (+0x14) and the format (+0x18, 0x44 bytes).
        let f = frame.addr();
        let image_slot = f + 4;
        let format = f + 0x18;
        e.mem.set_u32(f + 0x14, this.addr());
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
        let first = e.mem.u32(names);
        let read = e
            .vcall(
                this.addr(),
                8,
                &args![first, format, f + 3, f + 0x10, f + 0xc, f + 8],
            )
            .bool();
        if !read || e.mem.u32(f + 8) != 1 {
            return Ptr::new(0);
        }
        let width = e.mem.u32(f + 0x10);
        let height = e.mem.u32(f + 0xc);
        let data = new_pixel_data(e, width, height, format, count, 1);
        if data == 0 {
            return Ptr::new(0);
        }
        for level in 0..count {
            let name = e.mem.u32(names.wrapping_add(4 * level));
            if name == 0 {
                e.vcall(data, 0, &args![1u32]);
                return Ptr::new(0);
            }
            e.mem.set_u32(image_slot, 0);
            let image = if level == 0 && given != 0 {
                given
            } else {
                let converter = e.mem.u32(f + 0x14);
                e.vcall(converter, 0xc, &args![name, 0u32, 0u32]).u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![image_slot, image]);
            let image = e.mem.u32(image_slot);
            if image == 0 {
                e.vcall(data, 0, &args![1u32]);
                return Ptr::new(0);
            }
            let offsets = e.mem.u32(data + DATA_OFFSETS);
            let size = e
                .mem
                .u32(offsets.wrapping_add(4 * level + 4))
                .wrapping_sub(e.mem.u32(offsets.wrapping_add(4 * level)));
            let image_offsets = e.mem.u32(image + DATA_OFFSETS);
            let image_size = e
                .mem
                .u32(image_offsets + 4)
                .wrapping_sub(e.mem.u32(image_offsets));
            if size != image_size {
                e.vcall(data, 0, &args![1u32]);
                e.call(NI_POINTER_RELEASE, &args![image_slot]);
                return Ptr::new(0);
            }
            let from = e
                .mem
                .u32(image + DATA_PIXELS)
                .wrapping_add(e.mem.u32(image_offsets));
            let to = level_pixels(e, data, level);
            e.call(MEMCPY_00EC44D0, &args![to, from, size]);
            if level == 0 && fn_00a77940(e, Ptr::new(format)) as u8 != 0 {
                let palette = e.mem.u32(image + DATA_PALETTE);
                e.call(DATA_SET_PALETTE, &args![data, palette]);
            }
            e.call(NI_POINTER_RELEASE, &args![image_slot]);
        }
        Ptr::new(data)
    })
}

// Translated from 00a7b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::GetImageReaderList` (Xbox PDB): the reader list `m_kReaders` (+0x990).
pub fn ni_dev_image_converter_get_image_reader_list(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + READERS)
}

// Translated from 00a7b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::AddReader` (Xbox PDB): puts the reader at the head of `m_kReaders`
/// (+0x990): a list item from `0043a010` (on the list's count word) with the reader, no
/// previous item and the old head as next; the old head's previous (or, for an empty list,
/// the tail) becomes the item, the count grows by one and the item is the new head.
pub fn ni_dev_image_converter_add_reader(e: &mut Engine, this: Ptr, reader: u32) {
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr() + READERS);
    let count_word = list.addr() + 8;
    let item: Ptr<NiTListItem> = e.call(LIST_ITEM_NEW, &args![count_word]).ptr();
    e.set(item, NiTListItem::m_element, reader);
    e.set(item, NiTListItem::m_pkPrev, 0);
    let head = e.get(list, NiTPointerList::m_pkHead);
    e.set(item, NiTListItem::m_pkNext, head);
    let head = e.get(list, NiTPointerList::m_pkHead);
    if head != 0 {
        e.set(
            Ptr::<NiTListItem>::new(head),
            NiTListItem::m_pkPrev,
            item.addr(),
        );
    } else {
        e.set(list, NiTPointerList::m_pkTail, item.addr());
    }
    let count = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(list, NiTPointerList::m_uiCount, count);
    e.set(list, NiTPointerList::m_pkHead, item.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7b260, ni_dev_image_converter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_b110,
            ni_dev_image_converter_ni_dev_image_converter(Ptr) -> Ptr
        ),
        entry!(0x00a7_7980, fn_00a77980(Ptr, u32) -> u8),
        entry!(0x00a7_79b0, ni_dev_image_converter_pixel_bits(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a7_6f80,
            fn_00a76f80(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_71d0,
            fn_00a771d0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00a7_8010, ni_dev_image_converter_is_an_alpha_supported_mipmap_generation_format(Ptr, u32) -> bool),
        entry!(0x00a7_8050, ni_dev_image_converter_is_a_non_alpha_supported_mipmap_generation_format(Ptr, u32) -> bool),
        entry!(0x00a7_7e40, ni_dev_image_converter_can_convert_pixel_data(Ptr, u32, u32) -> bool),
        entry!(0x00a7_7d10, ni_dev_image_converter_read_image_file(Ptr, u32, u32, u32) -> u32),
        entry!(0x00a7_a280, ni_dev_image_converter_convert_pixel_data(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00a7_8090, ni_dev_image_converter_generate_mipmap_levels(Ptr, u32, u32) -> u32),
        entry!(0x00a7_aff0, fn_00a7aff0(Ptr)),
        entry!(0x00a7_a4f0, ni_dev_image_converter_convert_pixel_data_format(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x00a7_6d70,
            fn_00a76d70(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_6e90,
            fn_00a76e90(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_6ea0,
            fn_00a76ea0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7040,
            fn_00a77040(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7110,
            fn_00a77110(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_72b0,
            fn_00a772b0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7330,
            fn_00a77330(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_73d0,
            fn_00a773d0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7460,
            fn_00a77460(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7510,
            fn_00a77510(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7590,
            fn_00a77590(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7630,
            fn_00a77630(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_76b0,
            fn_00a776b0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7740,
            fn_00a77740(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7800,
            fn_00a77800(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_7880,
            fn_00a77880(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_8f70,
            fn_00a78f70(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_90a0,
            fn_00a790a0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_9310,
            fn_00a79310(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_95d0,
            fn_00a795d0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_98c0,
            fn_00a798c0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_9ae0,
            fn_00a79ae0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_9e20,
            fn_00a79e20(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_9ef0,
            fn_00a79ef0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_a160,
            fn_00a7a160(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_a1f0,
            fn_00a7a1f0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_6d40,
            ni_dev_image_converter_is_a_supported_mipmap_generation_format(Ptr, u32) -> bool
        ),
        entry!(0x00a7_7920, fn_00a77920(u32) -> bool),
        entry!(0x00a7_7940, fn_00a77940(Ptr) -> u32),
        entry!(0x00a7_7960, fn_00a77960(Ptr) -> u32),
        entry!(0x00a7_7ae0, ni_dev_image_converter_can_read_image_file(Ptr, u32) -> bool),
        entry!(
            0x00a7_7bd0,
            ni_dev_image_converter_read_image_file_info(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00a7_8570,
            ni_dev_image_converter_convert_for_platform(Ptr, u32, u32, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a7_8630,
            ni_dev_image_converter_find_closest_pixel_format(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a7_86e0,
            ni_dev_image_converter_find_closest_pixel_format_dx9(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a7_8910,
            ni_dev_image_converter_find_closest_pixel_format_xenon(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a7_8a50,
            ni_dev_image_converter_find_closest_pixel_format_ps3(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x00a7_8be0, ni_dev_image_converter_generate_bump_map(Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00a7_ae60,
            ni_dev_image_converter_generate_mipmap_pyramid_from_files(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a7_b0a0, ni_dev_image_converter_get_image_reader_list(Ptr) -> Ptr),
        entry!(0x00a7_b0b0, ni_dev_image_converter_add_reader(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dev_image_converter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_aff0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a7b260, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7b260, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xa00]]);
        assert_eq!(calls_to(&e, 0x00a7_aff0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000s_tests {
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

    fn rig(failing_allocation: u32) -> Engine {
        let mut e = Engine::new();
        e.register(IMAGE_CONVERTER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMBER_780_CONSTRUCT, |_, a| a[0].into_ret());
        let mut allocations = 0;
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            allocations += 1;
            if allocations == failing_allocation {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[1]).into_ret()
            }
        });
        for constructor in READER_CONSTRUCTORS {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(LIST_ITEM_NEW, |e, _| {
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item, 0xdead);
            item.into_ret()
        });
        e
    }

    #[test]
    fn ni_dev_image_converter_puts_its_three_readers_at_the_head() {
        let mut e = rig(0);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this + READERS, 0x1111);
        assert_eq!(e.call(0x00a7_b110, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DEV_IMAGE_CONVERTER_VTABLE);
        assert_eq!(calls(&e, MEMBER_780_CONSTRUCT), vec![vec![this + 0x780]]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, READER_SIZE]; 3]
        );
        assert_eq!(calls(&e, LIST_ITEM_NEW), vec![vec![this + READERS + 8]; 3]);
        let readers: Vec<u32> = READER_CONSTRUCTORS
            .iter()
            .map(|c| calls(&e, *c)[0][0])
            .collect();
        // Head to tail: the last reader made first.
        let head = e.mem.u32(this + READERS);
        let tail = e.mem.u32(this + READERS + 4);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
        let mut item = head;
        let mut walked = vec![];
        let mut previous = 0;
        while item != 0 {
            assert_eq!(e.mem.u32(item + 4), previous);
            walked.push(e.mem.u32(item + 8));
            previous = item;
            item = e.mem.u32(item);
        }
        assert_eq!(previous, tail);
        assert_eq!(walked, vec![readers[2], readers[1], readers[0]]);
    }

    #[test]
    fn ni_dev_image_converter_keeps_a_null_reader_when_allocation_fails() {
        let mut e = rig(2);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.call(0x00a7_b110, &args![this]);
        assert!(calls(&e, READER_CONSTRUCTORS[1]).is_empty());
        let head = e.mem.u32(this + READERS);
        let middle = e.mem.u32(head);
        assert_eq!(e.mem.u32(middle + 8), 0);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    /// An object whose vtable slots `0..0x40` are recorders `base + slot`.
    fn object(e: &mut Engine, base: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        for slot in (0..0x40).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        let o = e.mem.alloc(0x80);
        e.mem.set_u32(o, table);
        o
    }

    /// A pixel format whose component records are `(type, bits)`.
    fn format(e: &mut Engine, components: [(u32, u8); 4]) -> u32 {
        let f = e.mem.alloc(0x44);
        for (i, (kind, bits)) in components.iter().enumerate() {
            e.mem.set_u32(f + 0x14 + 0xc * i as u32, *kind);
            e.mem.set_u8(f + 0x1c + 0xc * i as u32, *bits);
        }
        f
    }

    #[test]
    fn fn_00a77980_finds_the_component_bits() {
        let mut e = Engine::new();
        let f = format(&mut e, [(3, 1), (1, 6), (1, 7), (0, 5)]);
        assert_eq!(e.call(0x00a7_7980, &args![f, 1u32]).u32() & 0xff, 6);
        assert_eq!(e.call(0x00a7_7980, &args![f, 0u32]).u32() & 0xff, 5);
        assert_eq!(e.call(0x00a7_7980, &args![f, 2u32]).u32() & 0xff, 0);
    }

    #[test]
    fn ni_dev_image_converter_pixel_bits_fills_the_record() {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, a| (0x100 + a[1]).into_ret());
        e.register(FORMAT_COMPONENT_SHIFT, |_, a| (0x320 + a[1]).into_ret());
        let f = format(&mut e, [(0, 5), (1, 6), (2, 5), (7, 1)]);
        let bits = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00a7_79b0, &args![bits, f]).u32(), bits);
        for c in 0..4 {
            assert_eq!(e.mem.u32(bits + 4 * c), 0x100 + c);
            assert_eq!(e.mem.u8(bits + 0x10 + c), 0x20 + c as u8);
        }
        assert_eq!(
            (0..4)
                .map(|c| e.mem.u8(bits + 0x14 + c))
                .collect::<Vec<_>>(),
            vec![3, 2, 3, 8]
        );
    }

    /// `PixelBits` for 8-bit channels placed at bytes 2, 1, 0, 3 (BGRA words).
    fn bgra_bits(e: &mut Engine) -> u32 {
        let bits = e.mem.alloc(0x18);
        for (c, (mask, left)) in [
            (0x00ff_0000u32, 16u8),
            (0xff00, 8),
            (0xff, 0),
            (0xff00_0000, 24),
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_u32(bits + 4 * c as u32, *mask);
            e.mem.set_u8(bits + 0x10 + c as u32, *left);
            e.mem.set_u8(bits + 0x14 + c as u32, 0);
        }
        bits
    }

    #[test]
    fn fn_00a76f80_packs_three_bytes_with_opaque_alpha() {
        let mut e = Engine::new();
        let bits = bgra_bits(&mut e);
        let source = e.mem.alloc(12);
        e.mem
            .write(source, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let out = e.mem.alloc(16);
        e.call(
            0x00a7_6f80,
            &args![2u32, 2u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0xff01_0203);
        assert_eq!(e.mem.u32(out + 12), 0xff0a_0b0c);
    }

    #[test]
    fn fn_00a771d0_packs_four_bytes() {
        let mut e = Engine::new();
        let bits = bgra_bits(&mut e);
        e.mem.set_u8(bits + 0x14, 4);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0xf0, 2, 3, 4, 5, 6, 7, 8]);
        let out = e.mem.alloc(8);
        e.call(
            0x00a7_71d0,
            &args![1u32, 2u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0x040f_0203);
        assert_eq!(e.mem.u32(out + 4), 0x0800_0607);
        // No rows: nothing written.
        e.mem.set_u32(out, 0x55);
        e.call(
            0x00a7_71d0,
            &args![1u32, 0u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0x55);
    }

    #[test]
    fn mipmap_format_tests_compare_three_formats() {
        let mut e = Engine::new();
        e.register(FORMAT_EQUALS, |_, a| {
            u32::from(a[1] == FORMAT_011AA2A0 || a[1] == FORMAT_011AA2E8).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_8010, &args![0u32, 0x10u32]).bool());
        let compared: Vec<u32> = calls(&mut e).into_iter().map(|c| c.1[1]).collect();
        assert_eq!(
            compared,
            vec![FORMAT_011AA1C8, FORMAT_011AA258, FORMAT_011AA2A0]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_8050, &args![0u32, 0x10u32]).bool());
        let compared: Vec<u32> = calls(&mut e).into_iter().map(|c| c.1[1]).collect();
        assert_eq!(
            compared,
            vec![FORMAT_011AA180, FORMAT_011AA210, FORMAT_011AA2E8]
        );
        e.register(FORMAT_EQUALS, |_, _| Ret::default());
        assert!(!e.call(0x00a7_8010, &args![0u32, 0x10u32]).bool());
        assert!(!e.call(0x00a7_8050, &args![0u32, 0x10u32]).bool());
    }

    /// `CanConvertPixelData` with a source equal to `source_as` (one of the
    /// static formats, or 0 for none), a destination of `kind` and `depth`.
    fn can_convert(source_as: u32, kind: u32, depth: u8, compare: u32) -> bool {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, a| {
            [0xffu32, 0xff00, 0xff_0000, 0][a[1] as usize].into_ret()
        });
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        e.register(FORMAT_IS_COMPRESSED_00A77960, |_, _| Ret::default());
        e.register_double(FORMAT_COMPARE_00A765C0, move |_, _| compare.into_ret());
        let source = e.mem.alloc(0x44);
        e.mem.set_u8(source, 1);
        let destination = e.mem.alloc(0x44);
        e.mem.set_u8(destination, 1);
        e.mem.set_u8(destination + 1, depth);
        e.mem.set_u32(destination + 4, kind);
        e.register_double(FORMAT_EQUALS, move |_, a| {
            u32::from(a[0] == source && a[1] == source_as).into_ret()
        });
        e.call(0x00a7_7e40, &args![0u32, source, destination])
            .bool()
    }

    #[test]
    fn ni_dev_image_converter_can_convert_pixel_data_by_source() {
        assert!(can_convert(FORMAT_011AA180, 0, 0x20, 1));
        assert!(!can_convert(FORMAT_011AA180, 1, 0x10, 1));
        assert!(can_convert(FORMAT_011AA1C8, 2, 0x10, 0));
        assert!(!can_convert(FORMAT_011AA1C8, 2, 0x10, 1));
        assert!(can_convert(FORMAT_011AA258, 0, 0x10, 1));
        assert!(can_convert(FORMAT_011AA2A0, 8, 0x18, 1));
        assert!(!can_convert(FORMAT_011AA2A0, 3, 0x18, 1));
        assert!(can_convert(FORMAT_011AA450, 9, 0x20, 1));
        assert!(!can_convert(FORMAT_011AA450, 0, 0x20, 1));
        assert!(can_convert(FORMAT_011AA3C0, 3, 0x10, 1));
        assert!(!can_convert(FORMAT_011AA3C0, 3, 0x20, 1));
        assert!(!can_convert(0, 0, 0x20, 1));
        // A compressed destination is refused.
        assert!(!can_convert(FORMAT_011AA180, 5, 0x20, 1));
    }

    #[test]
    fn ni_dev_image_converter_can_convert_pixel_data_equal_and_flags() {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, _| Ret::default());
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        e.register(FORMAT_EQUALS, |_, _| 1u32.into_ret());
        let f = e.mem.alloc(0x44);
        assert!(e.call(0x00a7_7e40, &args![0u32, f, f]).bool());
        e.register(FORMAT_EQUALS, |_, _| Ret::default());
        let g = e.mem.alloc(0x44);
        assert!(!e.call(0x00a7_7e40, &args![0u32, f, g]).bool());
        // A 24-bit destination needs the 0xff00 green mask.
        e.mem.set_u8(f, 1);
        e.mem.set_u8(g, 1);
        e.mem.set_u8(g + 1, 0x18);
        assert!(!e.call(0x00a7_7e40, &args![0u32, f, g]).bool());
    }

    const READ_SLOTS: u32 = 0x0131_0000;
    const FILE_SLOTS: u32 = 0x0132_0000;

    fn reader_engine(accepts: u32, data: u32, file_ok: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        for f in [STRCPY_S, PATH_STANDARDIZE, NI_FILENAME_CONSTRUCT] {
            e.register(f, |_, _| Ret::default());
        }
        let reader = object(&mut e, READ_SLOTS);
        e.register_double(READ_SLOTS + 4, move |_, _| accepts.into_ret());
        e.register_double(READ_SLOTS + 8, move |_, _| data.into_ret());
        let file = object(&mut e, FILE_SLOTS);
        e.register_double(FILE_SLOTS + 4, move |_, _| file_ok.into_ret());
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 8, reader);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this + READERS, item);
        e.call_log = Some(vec![]);
        (e, this, file)
    }

    #[test]
    fn ni_dev_image_converter_read_image_file_reads_with_a_new_file() {
        let (mut e, this, file) = reader_engine(1, 0x7000, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0x7000
        );
        let log = calls(&mut e);
        let read: Vec<_> = log.iter().filter(|c| c.0 == READ_SLOTS + 8).collect();
        assert_eq!(read[0].1[1..].to_vec(), vec![file, 9]);
        assert!(log.contains(&(FILE_SLOTS, vec![file, 1])));
        assert_eq!(log[0].1[1..].to_vec(), vec![0x104, 0x500]);
    }

    #[test]
    fn ni_dev_image_converter_read_image_file_failures() {
        // No result: the new file is deleted and 0 answered.
        let (mut e, this, file) = reader_engine(1, 0, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&mut e).contains(&(FILE_SLOTS, vec![file, 1])));
        // A file that is not ready is deleted.
        let (mut e, this, file) = reader_engine(1, 0x7000, 0);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&mut e).contains(&(FILE_SLOTS, vec![file, 1])));
        // No reader accepts: a given file is deleted at the end.
        let (mut e, this, file) = reader_engine(0, 0x7000, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, file])
                .u32(),
            0
        );
        let log = calls(&mut e);
        assert!(log.contains(&(FILE_SLOTS, vec![file, 1])));
        assert!(log.iter().all(|c| c.0 != NI_FILE_GET_FILE));
    }

    const CONVERTER_SLOTS: u32 = 0x0133_0000;

    /// Pixel data with one level of `width` x `height`, the format word at
    /// +0xc and bit 0 of its format byte set.
    fn pixel_data(e: &mut Engine, width: u32, height: u32, kind: u32, levels: u32) -> u32 {
        let d = e.mem.alloc(0x74);
        e.mem.set_u8(d + 8, 1);
        e.mem.set_u32(d + 0xc, kind);
        let w = e.mem.alloc(4);
        e.mem.set_u32(w, width);
        let h = e.mem.alloc(4);
        e.mem.set_u32(h, height);
        e.mem.set_u32(d + DATA_WIDTHS, w);
        e.mem.set_u32(d + DATA_HEIGHTS, h);
        e.mem.set_u32(d + DATA_LEVELS, levels);
        e.mem.set_u32(d + DATA_FACES, 1);
        d
    }

    #[test]
    fn ni_dev_image_converter_convert_pixel_data_makes_new_data() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        let block = e.mem.alloc(0x74);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        let source = pixel_data(&mut e, 4, 4, 0, 1);
        let format = e.mem.alloc(0x44);
        e.mem.set_u8(format, 1);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
            .u32();
        assert_eq!(made, block);
        let log = calls(&mut e);
        assert!(log.contains(&(NI_PIXEL_DATA_CONSTRUCT, vec![block, 4, 4, format, 1, 1])));
        assert!(log.contains(&(
            CONVERTER_SLOTS + 0x30,
            vec![this, block, source, 0xffff_ffff]
        )));
        // The fill failing answers 0.
        e.register(CONVERTER_SLOTS + 0x30, |_, _| Ret::default());
        let made = e
            .call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
            .u32();
        assert_eq!(made, 0);
        // A format without bit 0 answers 0 at once.
        e.mem.set_u8(format, 0);
        assert_eq!(
            e.call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn ni_dev_image_converter_convert_pixel_data_reuses_the_destination() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        e.register(FORMAT_EQUALS, |_, _| 1u32.into_ret());
        e.register(NI_POINTER_RELEASE, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(CODEC_EXPAND_COMPRESSED_DATA, |_, _| Ret::default());
        let source = pixel_data(&mut e, 4, 4, 5, 1);
        let destination = pixel_data(&mut e, 4, 4, 0, 1);
        let format = e.mem.alloc(0x44);
        e.mem.set_u8(format, 1);
        e.mem.set_u32(format + 4, 5);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a7_a280, &args![this, source, format, destination, 0u32])
            .u32();
        assert_eq!(made, destination);
        let log = calls(&mut e);
        assert!(log.iter().all(|c| c.0 != CODEC_EXPAND_COMPRESSED_DATA));
        assert_eq!(log.iter().filter(|c| c.0 == NI_POINTER_RELEASE).count(), 2);
        // A compressed source of another format is expanded first.
        e.mem.set_u32(format + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_a280, &args![this, source, format, destination, 0u32]);
        assert!(calls(&mut e).contains(&(
            CODEC_EXPAND_COMPRESSED_DATA,
            vec![this + MEMBER_780, source]
        )));
    }

    #[test]
    fn ni_dev_image_converter_generate_mipmap_levels_box_filters_32_bit() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, _| Ret::default());
        e.register(CONVERTER_SLOTS + 0x24, |_, _| Ret::default());
        e.register(CONVERTER_SLOTS + 0x20, |_, _| 1u32.into_ret());
        e.map(FORMAT_011AA2A0, 0x44);
        e.mem.set_u32(FORMAT_011AA2A0, 0x2a0);
        e.map(FORMAT_011AA2E8, 0x44);
        // The working copy is compared by its first word.
        e.register(FORMAT_EQUALS, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0x2a0 && a[1] == FORMAT_011AA2A0).into_ret()
        });
        // A 2x2 source and a 3-level destination (2x2, 1x1 ... ) built by hand.
        let target = e.mem.alloc(0x74);
        let pixels = e.mem.alloc(0x20);
        e.mem.write(
            pixels,
            &[
                10, 20, 30, 40, 12, 22, 32, 42, 14, 24, 34, 44, 16, 26, 36, 46,
            ],
        );
        let offsets = e.mem.alloc(16);
        for (i, v) in [0u32, 16, 20, 20].iter().enumerate() {
            e.mem.set_u32(offsets + 4 * i as u32, *v);
        }
        let widths = e.mem.alloc(8);
        e.mem.set_u32(widths, 2);
        e.mem.set_u32(widths + 4, 1);
        let heights = e.mem.alloc(8);
        e.mem.set_u32(heights, 2);
        e.mem.set_u32(heights + 4, 1);
        e.mem.set_u32(target + DATA_PIXELS, pixels);
        e.mem.set_u32(target + DATA_OFFSETS, offsets);
        e.mem.set_u32(target + DATA_WIDTHS, widths);
        e.mem.set_u32(target + DATA_HEIGHTS, heights);
        e.mem.set_u32(target + DATA_LEVELS, 2);
        e.mem.set_u32(target + DATA_FACES, 1);
        e.register_double(NI_ALLOCATE, move |_, _| target.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        let source = e.mem.alloc(0x74);
        e.mem.set_u32(source + DATA_FACES, 1);
        e.mem.set_u32(source + DATA_WIDTHS, widths);
        e.mem.set_u32(source + DATA_HEIGHTS, heights);
        let made = e.call(0x00a7_8090, &args![this, source, 0u32]).u32();
        assert_eq!(made, target);
        let level: Vec<u8> = (0..4).map(|i| e.mem.u8(pixels + 16 + i)).collect();
        assert_eq!(level, vec![13, 23, 33, 43]);
        // Neither mipmap format: 0.
        e.register(CONVERTER_SLOTS + 0x20, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_8090, &args![this, source, 0u32]).u32(), 0);
    }

    #[test]
    fn filter_level_pairs_rows_and_columns() {
        let mut e = Engine::new();
        let from = e.mem.alloc(12);
        e.mem.write(from, &[0, 10, 20, 2, 12, 22]);
        let to = e.mem.alloc(4);
        // A level above one pixel wide: pairs of pixels.
        filter_level(&mut e, 3, from, to, 1, 1, 1, 2);
        assert_eq!(
            (0..3).map(|i| e.mem.u8(to + i)).collect::<Vec<_>>(),
            vec![1, 11, 21]
        );
    }

    #[test]
    fn fn_00a7aff0_takes_the_readers_down() {
        let mut e = Engine::new();
        e.map(NODE_POOL_FREE, 4);
        e.map(NODE_POOL_LOCK, 8);
        e.register(LOCK_TAKE, |e, _| {
            let count = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        for f in [
            POINTER_LIST_REMOVE_ALL,
            MEMBER_780_DESTRUCT,
            IMAGE_CONVERTER_DESTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let reader = object(&mut e, READ_SLOTS);
        let this = e.mem.alloc(0xa00);
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        e.mem.set_u32(a, b);
        e.mem.set_u32(a + 8, reader);
        e.mem.set_u32(b + 4, a);
        e.mem.set_u32(this + READERS, a);
        e.mem.set_u32(this + READERS + 4, b);
        e.mem.set_u32(this + READERS + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a7_aff0, &args![this]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(this), NI_DEV_IMAGE_CONVERTER_VTABLE);
        assert_eq!(e.mem.u32(this + READERS + 8), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE), b);
        assert!(log.contains(&(READ_SLOTS, vec![reader, 1])));
        let tail: Vec<u32> = log.iter().rev().take(3).map(|c| c.0).collect();
        assert_eq!(
            tail,
            vec![
                IMAGE_CONVERTER_DESTRUCT,
                MEMBER_780_DESTRUCT,
                POINTER_LIST_REMOVE_ALL
            ]
        );
    }
}

#[cfg(test)]
mod b008_format_tests {
    use super::*;

    /// One-face pixel data of `levels` levels (4x4, 2x2, ...) with 4 bytes
    /// per pixel.
    fn data(e: &mut Engine, levels: u32, depth: u8, kind: u32) -> u32 {
        let d = e.mem.alloc(0x74);
        e.mem.set_u8(d + 9, depth);
        e.mem.set_u32(d + 0xc, kind);
        let widths = e.mem.alloc(4 * levels);
        let heights = e.mem.alloc(4 * levels);
        let offsets = e.mem.alloc(4 * levels + 4);
        let mut offset = 0;
        for l in 0..levels {
            let side = 4 >> l;
            e.mem.set_u32(widths + 4 * l, side);
            e.mem.set_u32(heights + 4 * l, side);
            e.mem.set_u32(offsets + 4 * l, offset);
            offset += side * side * 4;
        }
        e.mem.set_u32(offsets + 4 * levels, offset);
        let pixels = e.mem.alloc(offset);
        e.mem.set_u32(d + DATA_WIDTHS, widths);
        e.mem.set_u32(d + DATA_HEIGHTS, heights);
        e.mem.set_u32(d + DATA_OFFSETS, offsets);
        e.mem.set_u32(d + DATA_PIXELS, pixels);
        e.mem.set_u32(d + DATA_LEVELS, levels);
        e.mem.set_u32(d + DATA_FACES, 1);
        d
    }

    const CONVERTER: u32 = 0x00a7_90a0;

    fn engine(equal: fn(&Engine, u32, u32) -> bool) -> Engine {
        let mut e = Engine::new();
        e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
        e.register(FORMAT_COMPONENT_MASK, |_, a| {
            [0xffu32, 0xff00, 0xff_0000, 0][a[1] as usize].into_ret()
        });
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        for f in [PALETTE_ASSIGN, DATA_SET_PALETTE, CONVERTER] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(PALETTE_COPY, |_, a| a[0].into_ret());
        e.register(PALETTE_FROM_ENTRIES, |_, a| a[0].into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // Formats compare equal by the rule given.
        e.register_double(FORMAT_EQUALS, move |e, a| {
            u32::from(equal(e, a[0], a[1])).into_ret()
        });
        e
    }

    #[test]
    fn same_formats_copy_every_level_and_the_palette() {
        let mut e = engine(|e, a, b| e.mem.u32(a + 4) == e.mem.u32(b + 4));
        let source = data(&mut e, 3, 32, 0);
        let destination = data(&mut e, 3, 32, 0);
        e.mem.set_u32(source + DATA_PALETTE, 0x7000);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a7_a4f0,
                &args![0u32, destination, source, 0xffff_ffffu32]
            )
            .bool());
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMCPY_00EC44D0)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(copies, vec![64, 16, 4]);
        assert!(log.iter().any(|c| c.0 == PALETTE_COPY));
        // A missing level refuses.
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 3u32])
            .bool());
        // Different sizes refuse.
        let small = data(&mut e, 1, 32, 0);
        e.mem.set_u32(e.mem.u32(small + DATA_WIDTHS), 2);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, small, source, 0u32])
            .bool());
    }

    #[test]
    fn a_palettized_source_converts_by_depth() {
        // The source format's word at +0x3c (data +0x44) is 0x180 to match 011aa180.
        fn rule(e: &Engine, a: u32, b: u32) -> bool {
            b == FORMAT_011AA180 && e.mem.u32(a + 0x3c) == 0x180
        }
        let mut e = engine(rule);
        let source = data(&mut e, 2, 8, 8);
        e.mem.set_u32(source + 0x44, 0x180);
        e.mem.set_u32(source + DATA_PALETTE, 0x7000);
        let destination = data(&mut e, 2, 32, 0);
        e.mem.set_u32(destination + 0x44, 0x999);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 1u32])
            .bool());
        let calls: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == CONVERTER)
            .map(|c| c.1)
            .collect();
        assert_eq!(calls.len(), 1);
        let pixels = e.mem.u32(destination + DATA_PIXELS);
        assert_eq!(calls[0][..5].to_vec(), vec![2, 2, 16, pixels + 64, 0x7000]);
        // Without a palette: refused.
        e.mem.set_u32(source + DATA_PALETTE, 0);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 1u32])
            .bool());
    }

    #[test]
    fn unknown_formats_and_compressed_destinations_refuse() {
        let mut e = engine(|_, _, _| false);
        let source = data(&mut e, 1, 32, 3);
        let destination = data(&mut e, 1, 32, 0);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
        e.mem.set_u32(destination + 0xc, 5);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
    }

    #[test]
    fn palettized_copies_set_up_the_palette() {
        fn rule(e: &Engine, a: u32, b: u32) -> bool {
            (b == FORMAT_011AA258 && e.mem.u32(a + 0x3c) == 0x258)
                || (b == FORMAT_011AA210 && e.mem.u32(a + 0x3c) == 0x210)
        }
        let mut e = engine(rule);
        let source = data(&mut e, 1, 8, 8);
        e.mem.set_u32(source + 0x44, 0x258);
        let palette = e.mem.alloc(0x24);
        e.mem.set_u32(palette + 0x14, 0x7700);
        e.mem.set_u32(source + DATA_PALETTE, palette);
        let destination = data(&mut e, 1, 8, 8);
        e.mem.set_u32(destination + 0x44, 0x210);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let made: Vec<_> = log.iter().filter(|c| c.0 == PALETTE_FROM_ENTRIES).collect();
        assert_eq!(made[0].1[1..].to_vec(), vec![0, 0x100, 0x7700]);
        assert!(log.iter().any(|c| c.0 == MEMCPY_00EC44D0));
    }
}

#[cfg(test)]
mod b000y_tests {
    use super::*;

    #[test]
    fn fn_00a77980_answers_the_first_matching_components_bits() {
        let mut e = Engine::new();
        let format = e.mem.alloc(0x44);
        for (index, (kind, bits)) in [(0x11u32, 5u8), (0x12, 6), (0x12, 9), (0x13, 0)]
            .iter()
            .enumerate()
        {
            let entry = format + 0x14 + 0xc * index as u32;
            e.mem.set_u32(entry, *kind);
            e.mem.set_u8(entry + 8, *bits);
        }
        let bits = |e: &mut Engine, kind: u32| e.call(0x00a7_7980, &args![format, kind]).u8();
        assert_eq!(bits(&mut e, 0x11), 5);
        assert_eq!(bits(&mut e, 0x12), 6);
        assert_eq!(bits(&mut e, 0x13), 0);
        assert_eq!(bits(&mut e, 0x14), 0);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    /// A `PixelBits` record with the given masks, shifts and `8 - bits` bytes.
    fn pixel_bits(e: &mut Engine, masks: [u32; 4], shifts: [u8; 4], rights: [u8; 4]) -> u32 {
        let bits = e.mem.alloc(0x18);
        for index in 0..4 {
            e.mem.set_u32(bits + 4 * index as u32, masks[index]);
            e.mem.set_u8(bits + 0x10 + index as u32, shifts[index]);
            e.mem.set_u8(bits + 0x14 + index as u32, rights[index]);
        }
        bits
    }

    /// Four 4-bit components, component `i` at bits `4i`.
    fn bits_4444(e: &mut Engine) -> u32 {
        pixel_bits(e, [0xf, 0xf0, 0xf00, 0xf000], [0, 4, 8, 12], [4; 4])
    }

    /// Four 8-bit components, component `i` at bits `8i`.
    fn bits_8888(e: &mut Engine) -> u32 {
        pixel_bits(
            e,
            [0xff, 0xff00, 0xff_0000, 0xff00_0000],
            [0, 8, 16, 24],
            [0; 4],
        )
    }

    /// Runs the row converter at `address` on `width` x `height` pixels.
    #[allow(clippy::too_many_arguments)]
    fn convert(
        e: &mut Engine,
        address: u32,
        width: u32,
        height: u32,
        destination: u32,
        palette: u32,
        destination_bits: u32,
        source: u32,
        source_bits: u32,
    ) {
        e.call(
            address,
            &args![
                width,
                height,
                0u32,
                destination,
                palette,
                destination_bits,
                source,
                source_bits
            ],
        );
    }

    #[test]
    fn sixteen_bit_to_sixteen_bit_moves_each_component() {
        let mut e = Engine::new();
        let source_bits = bits_4444(&mut e);
        // The destination keeps component 0 at bits 12, 1 at 4, 2 at 8, 3 at 0.
        let destination_bits =
            pixel_bits(&mut e, [0xf000, 0xf0, 0xf00, 0xf], [12, 4, 8, 0], [4; 4]);
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 0x1234);
        e.mem.set_u16(source + 2, 0xabcd);
        e.mem.set_u16(source + 4, 0x0f0f);
        e.mem.set_u16(source + 6, 0x1111);
        let to = e.mem.alloc(8);
        convert(
            &mut e,
            0x00a7_6d70,
            2,
            2,
            to,
            0,
            destination_bits,
            source,
            source_bits,
        );
        assert_eq!(
            (0..4).map(|i| e.mem.u16(to + 2 * i)).collect::<Vec<_>>(),
            vec![0x4231, 0xdbca, 0xff00, 0x1111]
        );
        // The jump at 00a76e90 runs the same body.
        let again = e.mem.alloc(8);
        convert(
            &mut e,
            0x00a7_6e90,
            1,
            1,
            again,
            0,
            destination_bits,
            source,
            source_bits,
        );
        assert_eq!(e.mem.u16(again), 0x4231);
        // No rows, no columns: nothing written.
        let none = e.mem.alloc(2);
        convert(
            &mut e,
            0x00a7_6d70,
            0,
            3,
            none,
            0,
            destination_bits,
            source,
            source_bits,
        );
        convert(
            &mut e,
            0x00a7_6d70,
            3,
            0,
            none,
            0,
            destination_bits,
            source,
            source_bits,
        );
        assert_eq!(e.mem.u16(none), 0);
    }

    #[test]
    fn bytes_to_sixteen_bit() {
        let mut e = Engine::new();
        let bits = bits_4444(&mut e);
        let source = e.mem.alloc(8);
        e.mem
            .write(source, &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
        let to = e.mem.alloc(4);
        // Three bytes and alpha 0xff.
        convert(&mut e, 0x00a7_6ea0, 2, 1, to, 0, bits, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0xf531, 0xfb97));
        // Four bytes.
        convert(&mut e, 0x00a7_7040, 2, 1, to, 0, bits, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0x7531, 0xfdb9));
        // Bytes 0, 1 and 2 (as component 3) of four.
        convert(&mut e, 0x00a7_7740, 2, 1, to, 0, bits, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0x5031, 0xd0b9));
        // Two bytes, component 3 from all ones.
        convert(&mut e, 0x00a7_7880, 2, 1, to, 0, bits, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0xf031, 0xf075));
        // A shift past 15 clears the component.
        let wide = pixel_bits(&mut e, [0xffff, 0, 0, 0], [16, 0, 0, 0], [0, 8, 8, 8]);
        convert(&mut e, 0x00a7_7040, 1, 1, to, 0, wide, source, 0);
        assert_eq!(e.mem.u16(to), 0);
    }

    #[test]
    fn bytes_to_thirty_two_bit() {
        let mut e = Engine::new();
        let bits = bits_8888(&mut e);
        let source = e.mem.alloc(8);
        e.mem
            .write(source, &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
        let to = e.mem.alloc(8);
        convert(&mut e, 0x00a7_76b0, 2, 1, to, 0, bits, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0x5600_3412, 0xde00_bc9a)
        );
        convert(&mut e, 0x00a7_7800, 2, 1, to, 0, bits, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0xff00_3412, 0xff00_7856)
        );
    }

    #[test]
    fn twenty_four_bit_copies_follow_the_masks() {
        let mut e = Engine::new();
        let red_high = pixel_bits(&mut e, [0xff_0000, 0xff00, 0xff, 0], [16, 8, 0, 0], [0; 4]);
        let red_low = bits_8888(&mut e);
        let other = pixel_bits(&mut e, [0xf, 0xff00, 0, 0], [0; 4], [0; 4]);
        let no_green = pixel_bits(&mut e, [0xff, 0xf0, 0, 0], [0; 4], [0; 4]);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let to = e.mem.alloc(8);
        // 32 to 24 bits by the destination's masks.
        convert(&mut e, 0x00a7_7110, 2, 1, to, 0, red_high, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![3, 2, 1, 7, 6, 5]);
        convert(&mut e, 0x00a7_7110, 2, 1, to, 0, red_low, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![1, 2, 3, 5, 6, 7]);
        e.mem.write(to, &[0; 8]);
        convert(&mut e, 0x00a7_7110, 2, 1, to, 0, other, source, 0);
        convert(&mut e, 0x00a7_7110, 2, 1, to, 0, no_green, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![0; 6]);
        // 24 to 32 bits by the source's masks; any red mask but 0xff0000 copies straight.
        convert(&mut e, 0x00a7_7460, 2, 1, to, 0, 0, source, red_high);
        assert_eq!(e.mem.bytes(to, 8), vec![3, 2, 1, 0xff, 6, 5, 4, 0xff]);
        convert(&mut e, 0x00a7_7460, 2, 1, to, 0, 0, source, other);
        assert_eq!(e.mem.bytes(to, 8), vec![1, 2, 3, 0xff, 4, 5, 6, 0xff]);
        e.mem.write(to, &[0; 8]);
        convert(&mut e, 0x00a7_7460, 2, 1, to, 0, 0, source, no_green);
        assert_eq!(e.mem.bytes(to, 8), vec![0; 8]);
    }

    #[test]
    fn twenty_four_bit_to_twenty_four_bit_reverses_or_copies() {
        let mut e = Engine::new();
        e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
        let red_high = pixel_bits(&mut e, [0xff_0000, 0xff00, 0xff, 0], [16, 8, 0, 0], [0; 4]);
        let red_low = bits_8888(&mut e);
        let other = pixel_bits(&mut e, [0xf, 0xff00, 0, 0], [0; 4], [0; 4]);
        let source = e.mem.alloc(6);
        e.mem.write(source, &[1, 2, 3, 4, 5, 6]);
        let to = e.mem.alloc(6);
        convert(&mut e, 0x00a7_a160, 2, 1, to, 0, red_high, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![3, 2, 1, 6, 5, 4]);
        e.call_log = Some(vec![]);
        convert(&mut e, 0x00a7_a160, 2, 3, to, 0, red_low, source, 0);
        assert_eq!(calls_to(&e, MEMCPY_00EC44D0), vec![vec![to, source, 18]]);
        e.call_log = Some(vec![]);
        convert(&mut e, 0x00a7_a160, 2, 3, to, 0, other, source, 0);
        assert!(calls_to(&e, MEMCPY_00EC44D0).is_empty());
        // 00a7a1f0 tests the source's masks.
        e.mem.write(to, &[0; 6]);
        convert(&mut e, 0x00a7_a1f0, 2, 1, to, 0, red_low, source, red_high);
        assert_eq!(e.mem.bytes(to, 6), vec![3, 2, 1, 6, 5, 4]);
        e.call_log = Some(vec![]);
        convert(&mut e, 0x00a7_a1f0, 1, 1, to, 0, red_high, source, red_low);
        assert_eq!(calls_to(&e, MEMCPY_00EC44D0), vec![vec![to, source, 3]]);
    }

    #[test]
    fn sixteen_and_thirty_two_bit_sources_expand_to_bytes() {
        let mut e = Engine::new();
        let bits16 = bits_4444(&mut e);
        let bits32 = bits_8888(&mut e);
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 0x1234);
        e.mem.set_u16(source + 2, 0xabcd);
        let to = e.mem.alloc(8);
        convert(&mut e, 0x00a7_72b0, 2, 1, to, 0, 0, source, bits16);
        assert_eq!(e.mem.bytes(to, 6), vec![0x40, 0x30, 0x20, 0xd0, 0xc0, 0xb0]);
        convert(&mut e, 0x00a7_7330, 2, 1, to, 0, 0, source, bits16);
        assert_eq!(
            e.mem.bytes(to, 8),
            vec![0x40, 0x30, 0x20, 0x10, 0xd0, 0xc0, 0xb0, 0xa0]
        );
        convert(&mut e, 0x00a7_73d0, 2, 1, to, 0, 0, source, bits16);
        assert_eq!(
            e.mem.bytes(to, 8),
            vec![0x40, 0x30, 0x20, 0xff, 0xd0, 0xc0, 0xb0, 0xff]
        );
        e.mem.set_u32(source, 0x4433_2211);
        e.mem.set_u32(source + 4, 0x8877_6655);
        convert(&mut e, 0x00a7_7510, 2, 1, to, 0, 0, source, bits32);
        assert_eq!(e.mem.bytes(to, 6), vec![0x11, 0x22, 0x33, 0x55, 0x66, 0x77]);
        convert(&mut e, 0x00a7_7590, 2, 1, to, 0, 0, source, bits32);
        assert_eq!(
            e.mem.bytes(to, 8),
            vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]
        );
        convert(&mut e, 0x00a7_7630, 2, 1, to, 0, 0, source, bits32);
        assert_eq!(
            e.mem.bytes(to, 8),
            vec![0x11, 0x22, 0x33, 0xff, 0x55, 0x66, 0x77, 0xff]
        );
    }

    /// A palette whose entry `i` is `[i, i + 1, i + 2, i + 3]`.
    fn palette(e: &mut Engine) -> u32 {
        let palette = e.mem.alloc(0x18);
        let entries = e.mem.alloc(0x400);
        for index in 0..0x100u32 {
            for byte in 0..4 {
                e.mem
                    .set_u8(entries + 4 * index + byte, (index + byte) as u8);
            }
        }
        e.mem.set_u32(palette + PALETTE_ENTRIES, entries);
        palette
    }

    fn buffer_engine() -> Engine {
        let mut e = Engine::new();
        e.register(BUFFER_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BUFFER_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn four_bit_palettes_unpack_and_look_up() {
        let mut e = buffer_engine();
        let palette = palette(&mut e);
        let bits = bits_8888(&mut e);
        let red_high = pixel_bits(&mut e, [0xff_0000, 0xff00, 0xff, 0], [16, 8, 0, 0], [0; 4]);
        let source = e.mem.alloc(2);
        e.mem.write(source, &[0x5a, 0x31]);
        let to = e.mem.alloc(16);
        convert(&mut e, 0x00a7_8f70, 2, 2, to, palette, bits, source, 0);
        assert_eq!(
            e.mem.bytes(to, 12),
            vec![5, 6, 7, 10, 11, 12, 3, 4, 5, 1, 2, 3]
        );
        assert_eq!(calls_to(&e, BUFFER_ALLOCATE), vec![vec![4]]);
        let buffer = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.0 == BUFFER_FREE)
            .unwrap()
            .1[0];
        assert_eq!(e.mem.bytes(buffer, 4), vec![5, 0xa, 3, 1]);
        convert(&mut e, 0x00a7_8f70, 2, 1, to, palette, red_high, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![7, 6, 5, 12, 11, 10]);
        // A destination without the 0xff00 mask: nothing allocated.
        e.call_log = Some(vec![]);
        let other = pixel_bits(&mut e, [0xff, 0xf0, 0, 0], [0; 4], [0; 4]);
        convert(&mut e, 0x00a7_8f70, 2, 1, to, palette, other, source, 0);
        assert!(calls_to(&e, BUFFER_ALLOCATE).is_empty());
        // Any other red mask unpacks and frees but writes nothing.
        let odd = pixel_bits(&mut e, [0xf, 0xff00, 0, 0], [0; 4], [0; 4]);
        e.mem.write(to, &[0; 16]);
        convert(&mut e, 0x00a7_8f70, 2, 1, to, palette, odd, source, 0);
        assert_eq!(calls_to(&e, BUFFER_FREE).len(), 1);
        assert_eq!(e.mem.bytes(to, 6), vec![0; 6]);

        convert(&mut e, 0x00a7_90a0, 2, 1, to, palette, bits, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0xff07_0605, 0xff0c_0b0a)
        );
        convert(&mut e, 0x00a7_9310, 2, 1, to, palette, bits, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0x0807_0605, 0x0d0c_0b0a)
        );
    }

    #[test]
    fn eight_bit_palettes_look_up() {
        let mut e = Engine::new();
        let palette = palette(&mut e);
        let bits32 = bits_8888(&mut e);
        let bits16 = bits_4444(&mut e);
        let red_high = pixel_bits(&mut e, [0xff_0000, 0xff00, 0xff, 0], [16, 8, 0, 0], [0; 4]);
        let source = e.mem.alloc(2);
        e.mem.write(source, &[0x10, 0xfe]);
        let to = e.mem.alloc(8);
        convert(&mut e, 0x00a7_98c0, 2, 1, to, palette, bits32, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0xff12_1110, 0xff00_fffe)
        );
        convert(&mut e, 0x00a7_9ef0, 2, 1, to, palette, bits32, source, 0);
        assert_eq!(
            (e.mem.u32(to), e.mem.u32(to + 4)),
            (0x1312_1110, 0x0100_fffe)
        );
        convert(&mut e, 0x00a7_95d0, 2, 1, to, palette, bits16, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0xf111, 0xf0ff));
        convert(&mut e, 0x00a7_9ae0, 2, 1, to, palette, bits16, source, 0);
        assert_eq!((e.mem.u16(to), e.mem.u16(to + 2)), (0x1111, 0x00ff));
        convert(&mut e, 0x00a7_9e20, 2, 1, to, palette, bits32, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![0x10, 0x11, 0x12, 0xfe, 0xff, 0]);
        convert(&mut e, 0x00a7_9e20, 2, 1, to, palette, red_high, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![0x12, 0x11, 0x10, 0, 0xff, 0xfe]);
        let other = pixel_bits(&mut e, [0xff, 0xf0, 0, 0], [0; 4], [0; 4]);
        e.mem.write(to, &[0; 8]);
        convert(&mut e, 0x00a7_9e20, 2, 1, to, palette, other, source, 0);
        assert_eq!(e.mem.bytes(to, 6), vec![0; 6]);
    }

    #[test]
    fn supported_mipmap_format_asks_both_tests() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, 0x0300_0020);
        e.mem.set_u32(vtable + 0x24, 0x0300_0024);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.register(0x0300_0020, |_, a| u32::from(a[1] == 1).into_ret());
        e.register(0x0300_0024, |_, a| u32::from(a[1] == 2).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_6d40, &args![this, 1u32]).bool());
        assert!(calls_to(&e, 0x0300_0024).is_empty());
        assert!(e.call(0x00a7_6d40, &args![this, 2u32]).bool());
        assert!(!e.call(0x00a7_6d40, &args![this, 3u32]).bool());
        assert_eq!(
            calls_to(&e, 0x0300_0024),
            vec![vec![this, 2], vec![this, 3]]
        );
    }

    #[test]
    fn small_format_tests() {
        let mut e = Engine::new();
        for (value, power) in [
            (0u32, false),
            (1, true),
            (2, true),
            (6, false),
            (0x8000_0000, true),
        ] {
            assert_eq!(
                e.call(0x00a7_7920, &args![value]).bool(),
                power,
                "{value:x}"
            );
        }
        let format = e.mem.alloc(0x44);
        for (word, two_or_three, compressed) in [
            (1u32, 0u32, 0u32),
            (2, 1, 0),
            (3, 1, 0),
            (4, 0, 1),
            (6, 0, 1),
            (7, 0, 0),
            (u32::MAX, 0, 0),
        ] {
            e.mem.set_u32(format + 4, word);
            assert_eq!(e.call(0x00a7_7940, &args![format]).u32(), two_or_three);
            assert_eq!(e.call(0x00a7_7960, &args![format]).u32(), compressed);
        }
    }

    const READER_VTABLE: u32 = 0x0300_1000;
    const READER_ACCEPTS: u32 = 0x0300_2004;
    const READER_INFO: u32 = 0x0300_200c;
    const FILE_VTABLE: u32 = 0x0300_3000;
    const FILE_DELETE: u32 = 0x0300_4000;
    const FILE_GOOD: u32 = 0x0300_4004;

    /// A converter whose reader list holds readers that accept when their word +4 is set and
    /// read the information when their word +8 is set; `GetFile` answers a file that is good
    /// when the global `0300_5000` is set (none when it is 2).
    fn reader_engine(readers: &[(u32, u32)]) -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        for double in [
            STRCPY_S,
            PATH_STANDARDIZE,
            NI_FILENAME_CONSTRUCT,
            FILE_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.put_vtable(READER_VTABLE, &[0, READER_ACCEPTS, 0, READER_INFO]);
        e.register(READER_ACCEPTS, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(READER_INFO, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.put_vtable(FILE_VTABLE, &[FILE_DELETE, FILE_GOOD]);
        e.map(0x0300_5000, 4);
        e.register(FILE_GOOD, |e, _| (e.mem.u32(0x0300_5000) & 1).into_ret());
        e.register(NI_FILE_GET_FILE, |e, _| {
            if e.mem.u32(0x0300_5000) == 2 {
                return 0u32.into_ret();
            }
            let file = e.mem.alloc(4);
            e.mem.set_u32(file, FILE_VTABLE);
            file.into_ret()
        });
        let this = e.mem.alloc(0xa00);
        let mut items = vec![];
        let mut next = 0;
        let mut made = vec![];
        for (accepts, reads) in readers.iter().rev() {
            let reader = e.mem.alloc(0xc);
            e.mem.set_u32(reader, READER_VTABLE);
            e.mem.set_u32(reader + 4, *accepts);
            e.mem.set_u32(reader + 8, *reads);
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, reader);
            next = item;
            items.push(item);
            made.push(reader);
        }
        made.reverse();
        e.mem.set_u32(this + READERS, next);
        e.call_log = Some(vec![]);
        (e, this, made)
    }

    #[test]
    fn can_read_image_file_asks_the_first_accepting_reader() {
        let (mut e, this, _) = reader_engine(&[(0, 0), (1, 0), (1, 0)]);
        e.mem.set_u32(0x0300_5000, 1);
        assert!(e.call(0x00a7_7ae0, &args![this, 0x1234u32]).bool());
        let copy = calls_to(&e, STRCPY_S);
        assert_eq!((copy[0][1], copy[0][2]), (0x104, 0x1234));
        assert_eq!(calls_to(&e, NI_FILE_GET_FILE).len(), 1);
        assert_eq!(calls_to(&e, NI_FILE_GET_FILE)[0][1..], [0, 0x8000]);
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 1);
        // A bad file is deleted too; no file or no reader: false.
        e.mem.set_u32(0x0300_5000, 0);
        assert!(!e.call(0x00a7_7ae0, &args![this, 0x1234u32]).bool());
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 2);
        e.mem.set_u32(0x0300_5000, 2);
        assert!(!e.call(0x00a7_7ae0, &args![this, 0x1234u32]).bool());
        let (mut e, this, _) = reader_engine(&[(0, 0)]);
        assert!(!e.call(0x00a7_7ae0, &args![this, 0x1234u32]).bool());
        assert!(calls_to(&e, NI_FILE_GET_FILE).is_empty());
    }

    #[test]
    fn read_image_file_info_passes_the_words_reordered() {
        let (mut e, this, readers) = reader_engine(&[(1, 0), (0, 1), (1, 1)]);
        e.mem.set_u32(0x0300_5000, 1);
        let words = args![this, 0x1234u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        assert!(e.call(0x00a7_7bd0, &words).bool());
        let info = calls_to(&e, READER_INFO);
        assert_eq!(info.len(), 2);
        assert_eq!(info[0][0], readers[0]);
        assert_eq!(info[1][0], readers[2]);
        assert_eq!(info[1][2..], [3, 4, 1, 2, 5]);
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 2);
        // A bad file stops with false.
        e.mem.set_u32(0x0300_5000, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a7_7bd0, &words).bool());
        assert!(calls_to(&e, READER_INFO).is_empty());
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 1);
        // No reader reads: false after all of them.
        let (mut e, this, _) = reader_engine(&[(1, 0)]);
        e.mem.set_u32(0x0300_5000, 1);
        assert!(!e
            .call(
                0x00a7_7bd0,
                &args![this, 0x1234u32, 1u32, 2u32, 3u32, 4u32, 5u32]
            )
            .bool());
    }

    const CONVERTER_VTABLE: u32 = 0x0300_6000;
    const CAN_CONVERT: u32 = 0x0300_7010;
    const CONVERT_FORMAT: u32 = 0x0300_7030;
    const FIND_CLOSEST: u32 = 0x0300_703c;
    const DX9: u32 = 0x0300_7048;
    const XENON: u32 = 0x0300_704c;
    const PS3: u32 = 0x0300_7050;

    fn converter(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x15];
        slots[4] = CAN_CONVERT;
        slots[0xc] = CONVERT_FORMAT;
        slots[0xf] = FIND_CLOSEST;
        slots[0x12] = DX9;
        slots[0x13] = XENON;
        slots[0x14] = PS3;
        e.put_vtable(CONVERTER_VTABLE, &slots);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this, CONVERTER_VTABLE);
        this
    }

    #[test]
    fn convert_for_platform_converts_into_new_data() {
        let mut e = Engine::new();
        let this = converter(&mut e);
        e.map(0x0300_8000, 0x100);
        e.register(FIND_CLOSEST, |e, _| e.mem.u32(0x0300_8000).into_ret());
        e.register(FORMAT_EQUALS, |e, _| e.mem.u32(0x0300_8004).into_ret());
        e.register(CAN_CONVERT, |e, _| e.mem.u32(0x0300_8008).into_ret());
        e.register(CONVERT_FORMAT, |e, _| e.mem.u32(0x0300_800c).into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let data = e.mem.alloc(0x74);
        let widths = e.mem.alloc(4);
        let heights = e.mem.alloc(4);
        e.mem.set_u32(widths, 64);
        e.mem.set_u32(heights, 32);
        e.mem.set_u32(data + DATA_WIDTHS, widths);
        e.mem.set_u32(data + DATA_HEIGHTS, heights);
        e.mem.set_u32(data + DATA_LEVELS, 7);
        e.mem.set_u32(data + DATA_FACES, 1);
        let run = |e: &mut Engine| {
            e.call(0x00a7_8570, &args![this, 5u32, 6u32, data, 8u32])
                .u32()
        };
        e.call_log = Some(vec![]);
        // Its own format by address.
        e.mem.set_u32(0x0300_8000, data + 8);
        assert_eq!(run(&mut e), data);
        assert_eq!(
            calls_to(&e, FIND_CLOSEST),
            vec![vec![this, 5, 6, data + 8, 8]]
        );
        // None.
        e.mem.set_u32(0x0300_8000, 0);
        assert_eq!(run(&mut e), 0);
        // An equal format.
        e.mem.set_u32(0x0300_8000, 0x0400_0000);
        e.mem.set_u32(0x0300_8004, 1);
        assert_eq!(run(&mut e), data);
        // Cannot convert.
        e.mem.set_u32(0x0300_8004, 0);
        assert_eq!(run(&mut e), 0);
        assert_eq!(
            calls_to(&e, CAN_CONVERT).last().unwrap(),
            &vec![this, data + 8, 0x0400_0000]
        );
        // Converts: the new data; a failed conversion: null.
        e.mem.set_u32(0x0300_8008, 1);
        e.mem.set_u32(0x0300_800c, 1);
        let made = run(&mut e);
        assert_ne!(made, 0);
        assert_eq!(
            calls_to(&e, NI_PIXEL_DATA_CONSTRUCT),
            vec![vec![made, 64, 32, 0x0400_0000, 7, 1]]
        );
        assert_eq!(
            calls_to(&e, CONVERT_FORMAT),
            vec![vec![this, made, data, u32::MAX]]
        );
        e.mem.set_u32(0x0300_800c, 0);
        assert_eq!(run(&mut e), 0);
    }

    #[test]
    fn find_closest_pixel_format_dispatches_by_platform() {
        let mut e = Engine::new();
        let this = converter(&mut e);
        for slot in [DX9, XENON, PS3] {
            e.register(slot, |_, a| (a[1] + 0x100).into_ret());
        }
        let request = e.mem.alloc(8);
        let format = e.mem.alloc(0x44);
        let run = |e: &mut Engine, platform: u32| {
            e.call(0x00a7_8630, &args![this, platform, request, 7u32, format])
                .u32()
        };
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 4), request + 0x100);
        assert_eq!(calls_to(&e, XENON), vec![vec![this, request, 7, format]]);
        run(&mut e, 5);
        assert_eq!(calls_to(&e, PS3).len(), 1);
        run(&mut e, 6);
        assert_eq!(calls_to(&e, DX9).len(), 1);
        for platform in [1, 2, 3, 7] {
            assert_eq!(run(&mut e, platform), 0);
        }
        // Platform 0: 011aa2a0 for a request word of 4 and a 2/3 or 16-bit format.
        e.mem.set_u32(request, 4);
        e.mem.set_u32(format + 4, 3);
        assert_eq!(run(&mut e, 0), FORMAT_011AA2A0);
        e.mem.set_u32(format + 4, 1);
        assert_eq!(run(&mut e, 0), format);
        e.mem.set_u8(format + 1, 0x10);
        assert_eq!(run(&mut e, 0), FORMAT_011AA2A0);
        e.mem.set_u32(request, 3);
        assert_eq!(run(&mut e, 0), format);
    }

    /// A request `{kind, variant}` and a format of word `word`, `depth` bits, component 3 of
    /// `alpha_bits`.
    fn request_and_format(
        e: &mut Engine,
        kind: u32,
        variant: u32,
        word: u32,
        depth: u8,
        alpha_bits: u8,
    ) -> (u32, u32) {
        let request = e.mem.alloc(8);
        e.mem.set_u32(request, kind);
        e.mem.set_u32(request + 4, variant);
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format + 4, word);
        e.mem.set_u8(format + 1, depth);
        for index in 0..4 {
            e.mem.set_u32(format + 0x14 + 0xc * index, 9);
        }
        e.mem.set_u32(format + 0x14 + 0xc * 2, 3);
        e.mem.set_u8(format + 0x1c + 0xc * 2, alpha_bits);
        (request, format)
    }

    type Case = (u32, u32, u32, u8, u8, u32);

    #[test]
    fn dx9_search_picks_the_static_format() {
        let mut e = Engine::new();
        let cases: [Case; 22] = [
            (7, 0, 0, 0, 0, FORMAT_011AAD98),
            (7, 1, 0, 0, 0, FORMAT_011AA768),
            (8, 0, 0, 0, 0, FORMAT_011AA6D8),
            (9, 0, 0, 0, 0, FORMAT_011AA600),
            (0xa, 0, 0, 0, 0, FORMAT_011AA648),
            (0xb, 0, 0, 0, 0, FORMAT_011AA690),
            (0xe, 0, 0, 0, 0, FORMAT_011AA570),
            (0xc, 0, 0, 0, 0, FORMAT_011AA5B8),
            (2, 0, 0, 0, 0, FORMAT_011AAB58),
            (2, 3, 0, 0, 0, FORMAT_011AAAC8),
            (1, 1, 0, 0, 0, FORMAT_011AA960),
            (1, 2, 0, 0, 0, FORMAT_011AA9F0),
            (1, 0, 0, 0, 0, FORMAT_011AA8D0),
            (5, 2, 0, 0, 0, FORMAT_011AA528),
            (0, 1, 0, 0, 0, FORMAT_011AA498),
            (3, 0, 5, 0, 0, FORMAT_011AA4E0),
            (3, 2, 9, 0, 0, FORMAT_011AA528),
            (4, 0, 0, 0, 0, FORMAT_011AA450),
            (6, 2, 4, 0, 0, FORMAT_011AA498),
            (6, 3, 0, 0x10, 4, FORMAT_011AA9F0),
            (6, 3, 0, 0x10, 0, FORMAT_011AA8D0),
            (6, 3, 1, 0x20, 0, FORMAT_011AAAC8),
        ];
        for (kind, variant, word, depth, alpha, expected) in cases {
            let (request, format) = request_and_format(&mut e, kind, variant, word, depth, alpha);
            let found = e
                .call(0x00a7_86e0, &args![0u32, request, 0u32, format])
                .u32();
            assert_eq!(found, expected, "{kind} {variant} {word} {depth}");
        }
        // Unmatched: the format itself.
        for (kind, variant, word, depth) in
            [(6u32, 3u32, 7u32, 0x20u8), (6, 0, 0, 8), (0xf, 0, 0, 0)]
        {
            let (request, format) = request_and_format(&mut e, kind, variant, word, depth, 0);
            assert_eq!(
                e.call(0x00a7_86e0, &args![0u32, request, 0u32, format])
                    .u32(),
                format
            );
        }
    }

    #[test]
    fn xenon_and_ps3_searches_pick_the_static_format() {
        let mut e = Engine::new();
        // (kind, variant, format word, depth, alpha bits of the second word's format, answer)
        let xenon: [Case; 13] = [
            (0xe, 0, 0, 0, 0, FORMAT_011AA570),
            (0xd, 0, 0, 0, 0, FORMAT_011AA5B8),
            (2, 0, 0, 0, 0, FORMAT_011AAAC8),
            (1, 0, 0, 0, 4, FORMAT_011AA9F0),
            (1, 0, 0, 0, 0, FORMAT_011AA8D0),
            (5, 0, 0, 0, 4, FORMAT_011AA528),
            (0, 0, 0, 0, 0, FORMAT_011AA498),
            (3, 0, 5, 0, 0, FORMAT_011AA4E0),
            (3, 0, 1, 0, 4, FORMAT_011AA528),
            (4, 0, 0, 0, 0, FORMAT_011AA450),
            (6, 0, 0, 0x40, 0, FORMAT_011AA5B8),
            (6, 0, 0, 0x80, 0, FORMAT_011AA570),
            (0xc, 0, 0, 0, 0, FORMAT_011AAAC8),
        ];
        for (kind, variant, word, depth, alpha, expected) in xenon {
            let (request, format) = request_and_format(&mut e, kind, variant, word, depth, 0);
            let (_, alpha_format) = request_and_format(&mut e, 0, 0, 1, 0, alpha);
            let found = e
                .call(0x00a7_8910, &args![0u32, request, alpha_format, format])
                .u32();
            assert_eq!(found, expected, "xenon {kind} {word} {depth}");
        }
        let ps3: [Case; 11] = [
            (0xe, 0, 0, 0, 0, FORMAT_011AA570),
            (0xc, 0, 0, 0, 0, FORMAT_011AA5B8),
            (1, 0, 0, 0, 4, FORMAT_011AA9F0),
            (1, 0, 0, 0, 0, FORMAT_011AA8D0),
            (0, 2, 0, 0, 0, FORMAT_011AA528),
            (3, 0, 4, 0, 0, FORMAT_011AA498),
            (2, 0, 0, 0, 0, FORMAT_011AAB10),
            (6, 0, 0, 0x20, 0, FORMAT_011AAB10),
            (6, 2, 0, 0x10, 0, FORMAT_011AA9F0),
            (6, 3, 0, 0x10, 0, FORMAT_011AA8D0),
            (6, 0, 6, 0, 0, FORMAT_011AA528),
        ];
        for (kind, variant, word, depth, alpha, expected) in ps3 {
            let (request, format) = request_and_format(&mut e, kind, variant, word, depth, 0);
            let (_, alpha_format) = request_and_format(&mut e, 0, 0, 1, 0, alpha);
            let found = e
                .call(0x00a7_8a50, &args![0u32, request, alpha_format, format])
                .u32();
            assert_eq!(found, expected, "ps3 {kind} {variant} {word} {depth}");
        }
        // The PS3 16-bit path's own alpha test on the format.
        let (request, format) = request_and_format(&mut e, 6, 3, 0, 0x10, 4);
        let (_, alpha_format) = request_and_format(&mut e, 0, 0, 1, 0, 0);
        assert_eq!(
            e.call(0x00a7_8a50, &args![0u32, request, alpha_format, format])
                .u32(),
            FORMAT_011AA9F0
        );
    }

    /// A one-level pixel data of `width` x `height` pixels of `pixel` bytes, its bytes from
    /// `bytes`, with the format comparison answering by the format address at +8.
    fn pixel_data(e: &mut Engine, width: u32, height: u32, pixel: u32, bytes: &[u8]) -> u32 {
        let data = e.mem.alloc(0x74);
        let widths = e.mem.alloc(4);
        let heights = e.mem.alloc(4);
        let offsets = e.mem.alloc(8);
        let pixels = e.mem.alloc(width * height * pixel + 4);
        e.mem.set_u32(widths, width);
        e.mem.set_u32(heights, height);
        e.mem.set_u32(offsets + 4, width * height * pixel);
        e.mem.write(pixels, bytes);
        e.mem.set_u32(data + DATA_PIXELS, pixels);
        e.mem.set_u32(data + DATA_WIDTHS, widths);
        e.mem.set_u32(data + DATA_HEIGHTS, heights);
        e.mem.set_u32(data + DATA_OFFSETS, offsets);
        e.mem.set_u32(data + DATA_LEVELS, 1);
        data
    }

    fn bump_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0300_9000, 0x10);
        // Format equality: the format's first word against the compared address.
        e.register(FORMAT_EQUALS, |e, a| {
            u32::from(e.mem.u32(a[0]) == a[1]).into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |e, a| {
            let pixel = if a[3] == FORMAT_011AA408 { 2 } else { 4 };
            let made = pixel_data(e, a[1], a[2], pixel, &[]);
            e.mem.set_u32(made + 8, a[3]);
            made.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn generate_bump_map_takes_differences() {
        let mut e = bump_engine();
        // A 3 x 2 source of 011aa2e8 (3 bytes a pixel), first bytes 10 20 40 / 11 25 50.
        let bytes = [10, 0, 0, 20, 0, 0, 40, 0, 0, 11, 0, 0, 25, 0, 0, 50, 0, 0];
        let source = pixel_data(&mut e, 3, 2, 3, &bytes);
        e.mem.set_u32(source + 8, FORMAT_011AA2E8);
        let plain = e.mem.alloc(4);
        let bump = e.call(0x00a7_8be0, &args![0u32, source, plain]).u32();
        assert_eq!(e.mem.u32(bump + 8), FORMAT_011AA408);
        let out = level_pixels(&e, bump, 0);
        assert_eq!(
            e.mem.bytes(out, 12),
            vec![10, 1, 20, 5, 20, 10, 14, 1, 25, 5, 25, 10]
        );
        // With the format 011aa450: 4-byte pixels, bytes 2 and 3 filled from the source.
        let format = e.mem.alloc(4);
        e.mem.set_u32(format, FORMAT_011AA450);
        let bump = e.call(0x00a7_8be0, &args![0u32, source, format]).u32();
        let out = level_pixels(&e, bump, 0);
        assert_eq!(e.mem.bytes(out, 8), vec![10, 1, 0xff, 10, 20, 5, 0xff, 20]);
        // A one-row, one-pixel source of 011aa2a0.
        let single = pixel_data(&mut e, 1, 1, 4, &[7, 0, 0, 9]);
        e.mem.set_u32(single + 8, FORMAT_011AA2A0);
        let bump = e.call(0x00a7_8be0, &args![0u32, single, 0u32]).u32();
        assert_eq!(e.mem.u32(bump + 8), FORMAT_011AA450);
        let out = level_pixels(&e, bump, 0);
        assert_eq!(e.mem.bytes(out, 4), vec![0, 0, 9, 7]);
        // Any other source: null.
        e.mem.set_u32(single + 8, FORMAT_011AA180);
        assert_eq!(e.call(0x00a7_8be0, &args![0u32, single, 0u32]).u32(), 0);
    }

    #[test]
    fn mipmap_pyramid_copies_each_file_level() {
        let mut e = Engine::new();
        let this = converter(&mut e);
        e.put_vtable(0x0300_a000, &[0x0300_a100]);
        e.register(0x0300_a100, |_, _| Ret::default());
        e.mem.set_u32(CONVERTER_VTABLE + 8, 0x0300_b008);
        e.mem.set_u32(CONVERTER_VTABLE + 0xc, 0x0300_b00c);
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 2);
            Ret::default()
        });
        e.register(0x0300_b008, |e, a| {
            e.mem.set_u32(a[4], 2);
            e.mem.set_u32(a[5], 2);
            e.mem.set_u32(a[6], 1);
            1u32.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |e, a| {
            let data = pixel_data(e, a[1], a[2], 1, &[]);
            let offsets = e.mem.alloc(12);
            e.mem.set_u32(offsets + 4, 4);
            e.mem.set_u32(offsets + 8, 5);
            e.mem.set_u32(data + DATA_OFFSETS, offsets);
            e.mem.set_u32(data, 0x0300_a000);
            data.into_ret()
        });
        e.register(0x0300_b00c, |e, a| {
            let size = if a[1] == 2 { 1 } else { 9 };
            let image = pixel_data(e, size, 1, 1, &[0x77]);
            e.mem.set_u32(image + DATA_PALETTE, 0x5555);
            image.into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_RELEASE, |_, _| Ret::default());
        e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
        e.register(DATA_SET_PALETTE, |_, _| Ret::default());
        let given = pixel_data(&mut e, 2, 2, 1, &[1, 2, 3, 4]);
        let names = e.mem.alloc(8);
        e.mem.set_u32(names, 1);
        e.mem.set_u32(names + 4, 2);
        e.call_log = Some(vec![]);
        let data = e.call(0x00a7_ae60, &args![this, names, 2u32, given]).u32();
        assert_ne!(data, 0);
        let copies = calls_to(&e, MEMCPY_00EC44D0);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][2], 4);
        assert_eq!(copies[1][2], 1);
        assert_eq!(calls_to(&e, 0x0300_b00c).len(), 1);
        assert_eq!(calls_to(&e, DATA_SET_PALETTE), vec![vec![data, 0]]);
        assert_eq!(calls_to(&e, NI_POINTER_RELEASE).len(), 2);
        // A level of the wrong size deletes the new data.
        e.mem.set_u32(names + 4, 3);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a7_ae60, &args![this, names, 2u32, given]).u32(),
            0
        );
        assert_eq!(calls_to(&e, 0x0300_a100).len(), 1);
        // A missing name, or no first name.
        e.mem.set_u32(names + 4, 0);
        assert_eq!(
            e.call(0x00a7_ae60, &args![this, names, 2u32, given]).u32(),
            0
        );
        e.mem.set_u32(names, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a7_ae60, &args![this, names, 2u32, given]).u32(),
            0
        );
        assert!(calls_to(&e, NI_PIXEL_FORMAT_CONSTRUCT).is_empty());
    }

    #[test]
    fn reader_list_and_add_reader() {
        let mut e = Engine::new();
        e.register(LIST_ITEM_NEW, |e, _| e.mem.alloc(0xc).into_ret());
        let this = e.mem.alloc(0xa00);
        assert_eq!(e.call(0x00a7_b0a0, &args![this]).u32(), this + 0x990);
        e.call(0x00a7_b0b0, &args![this, 0x1111u32]);
        let first = e.mem.u32(this + 0x990);
        assert_eq!(e.mem.u32(this + 0x994), first);
        assert_eq!(e.mem.u32(this + 0x998), 1);
        assert_eq!(
            (e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(first + 8)),
            (0, 0, 0x1111)
        );
        e.call(0x00a7_b0b0, &args![this, 0x2222u32]);
        let second = e.mem.u32(this + 0x990);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first + 4), second);
        assert_eq!(e.mem.u32(this + 0x994), first);
        assert_eq!(e.mem.u32(this + 0x998), 2);
    }
}

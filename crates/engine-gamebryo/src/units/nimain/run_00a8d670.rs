//! `NiMain/run_00a8d670` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual tables the constructors store (RTTI `NiImageReader`, `NiTGAReader`,
/// `NiBMPReader`).
const NI_IMAGE_READER_VTABLE: u32 = 0x0109_f3d0;
const NI_TGA_READER_VTABLE: u32 = 0x0109_f4b0;
const NI_BMP_READER_VTABLE: u32 = 0x0109_f4c4;
/// `InitializeCriticalSection` (import slot) and `NiPixelFormat::NiPixelFormat` (engine map,
/// `00a7cca0`; `thiscall`, no stack words).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
/// The pixel format both readers copy over their own (0x11 words at `011aa2e8`).
const PIXEL_FORMAT_SOURCE_011AA2E8: u32 = 0x011a_a2e8;
const PIXEL_FORMAT_WORDS: u32 = 0x11;
/// Where each reader keeps its pixel format.
const TGA_PIXEL_FORMAT: u32 = 0x11c;
const BMP_PIXEL_FORMAT: u32 = 0x108;

/// The start both readers share: the `NiImageReader` virtual table, its critical section at
/// +0x80, the reader's own virtual table, and its pixel format constructed. Answers the pixel
/// format's address.
fn reader_start(e: &mut Engine, base: u32, vtable: u32, pixel_format: u32) -> u32 {
    e.mem.set_u32(base, NI_IMAGE_READER_VTABLE);
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base, vtable);
    let format = base.wrapping_add(pixel_format);
    e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
    format
}

/// The `REP MOVSD` of the 0x11 words at `011aa2e8` over the pixel format.
fn copy_pixel_format(e: &mut Engine, format: u32) {
    for word in 0..PIXEL_FORMAT_WORDS {
        let value = e.mem.u32(PIXEL_FORMAT_SOURCE_011AA2E8 + 4 * word);
        e.mem.set_u32(format.wrapping_add(4 * word), value);
    }
}

// Translated from 00a8d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8d670`): the `NiTGAReader` constructor (the RTTI of the
/// virtual table it stores), `thiscall`, no stack words. The shared reader start (pixel format
/// at +0x11c), the 16-bit fields +0x104, +0x10a, +0x10e, +0x106, +0x10c and +0x110 and the
/// bytes +0x100 to +0x102, +0x108 and +0x112 to +0x118 cleared, the pixel format at `011aa2e8`
/// copied over its own, then the words +0x160 to +0x174 and the bytes +0x178 to +0x17c
/// cleared. Answers `this`.
pub fn fn_00a8d750(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let format = reader_start(e, base, NI_TGA_READER_VTABLE, TGA_PIXEL_FORMAT);
    for offset in [0x104u32, 0x10a, 0x10e] {
        e.mem.set_u16(base.wrapping_add(offset), 0);
    }
    for offset in [
        0x100u32, 0x101, 0x102, 0x108, 0x112, 0x113, 0x114, 0x115, 0x116, 0x117, 0x118,
    ] {
        e.mem.set_u8(base.wrapping_add(offset), 0);
    }
    for offset in [0x106u32, 0x10c, 0x110] {
        e.mem.set_u16(base.wrapping_add(offset), 0);
    }
    copy_pixel_format(e, format);
    for offset in (0x160u32..=0x174).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in 0x178u32..=0x17c {
        e.mem.set_u8(base.wrapping_add(offset), 0);
    }
    this
}

// Translated from 00a8e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a8d670`): the `NiBMPReader` constructor (the RTTI of the
/// virtual table it stores), `thiscall`, no stack words. The shared reader start (pixel format
/// at +0x108), +0x100 and +0x104 cleared, the pixel format at `011aa2e8` copied over its own,
/// then +0x150 and +0x154, the bytes +0x158 and +0x159 and the 16-bit +0x14c cleared. Answers
/// `this`.
pub fn fn_00a8e410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let format = reader_start(e, base, NI_BMP_READER_VTABLE, BMP_PIXEL_FORMAT);
    e.mem.set_u32(base.wrapping_add(0x100), 0);
    e.mem.set_u32(base.wrapping_add(0x104), 0);
    copy_pixel_format(e, format);
    e.mem.set_u32(base.wrapping_add(0x150), 0);
    e.mem.set_u32(base.wrapping_add(0x154), 0);
    e.mem.set_u8(base.wrapping_add(0x158), 0);
    e.mem.set_u8(base.wrapping_add(0x159), 0);
    e.mem.set_u16(base.wrapping_add(0x14c), 0);
    this
}

/// The `DeleteCriticalSection` import slot.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The memory manager (`011f6238`) and its `Deallocate` (`00aa4060`,
/// `thiscall`, `RET 4`), and the Gamebryo free (`00aa10f0`, `cdecl`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a8e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor of an image reader
/// with nothing of its own to free (`thiscall`, `RET 4`: the flags): stores
/// `NiImageReader`'s vtable, deletes the critical section at `+0x80` and, with
/// bit 0 of the flags, gives the object back to the memory manager. Answers
/// `this`.
pub fn fn_00a8e480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, NI_IMAGE_READER_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![t + 0x80]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, t]);
    }
    this
}

// Translated from 00a8d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiTGAReader`'s scalar deleting destructor
/// (`thiscall`, `RET 4`: the flags): stores its vtable, frees the block at
/// `+0x164` (`00aa10f0`) and gives the one at `+0x16c` back to the memory
/// manager, stores `NiImageReader`'s vtable, deletes the critical section at
/// `+0x80` and, with bit 0 of the flags, gives the object back. Answers
/// `this`.
pub fn fn_00a8d850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    let block = e.mem.u32(t + 0x164);
    e.mem.set_u32(t, NI_TGA_READER_VTABLE);
    e.call(NI_FREE, &args![block]);
    let other = e.mem.u32(t + 0x16c);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, other]);
    e.mem.set_u32(t, NI_IMAGE_READER_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![t + 0x80]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, t]);
    }
    this
}

/// `NiTGAReader` fields as the PC code uses them (the header fields follow the TGA file
/// header, read field by field into +0x100 to +0x113): the ID length (+0x100, u8), the
/// colour-map type (+0x101, u8), the image type (+0x102, u8), the first colour-map entry
/// (+0x104, u16), the colour-map length (+0x106, u16), the colour-map entry size in bits
/// (+0x108, u8), the width (+0x10e, u16), the height (+0x110, u16), the pixel depth
/// (+0x112, u8), the image descriptor (+0x113, u8); then the reader's own state: bytes per
/// pixel (+0x114), 32-bit colour (+0x115), colour-mapped (+0x116), bottom-up (+0x117),
/// run-length encoded (+0x118), the pixel format (+0x11c, 0x44 bytes), the row buffer's
/// size and address (+0x160, +0x164), the palette's capacity and address (+0x168, +0x16c,
/// 4 bytes per entry), the row unpacker (+0x170), the pixels left in the current run
/// (+0x174), whether the run repeats one pixel (+0x178) and that pixel (+0x179). The field
/// meanings are the TGA format's, as the code uses them.
const TGA_ID_LENGTH: u32 = 0x100;
const TGA_COLOUR_MAP_TYPE: u32 = 0x101;
const TGA_IMAGE_TYPE: u32 = 0x102;
const TGA_FIRST_ENTRY: u32 = 0x104;
const TGA_MAP_LENGTH: u32 = 0x106;
const TGA_ENTRY_BITS: u32 = 0x108;
const TGA_WIDTH: u32 = 0x10e;
const TGA_HEIGHT: u32 = 0x110;
const TGA_PIXEL_DEPTH: u32 = 0x112;
const TGA_DESCRIPTOR: u32 = 0x113;
const TGA_BYTES_PER_PIXEL: u32 = 0x114;
const TGA_THIRTY_TWO_BIT: u32 = 0x115;
const TGA_MAPPED: u32 = 0x116;
const TGA_BOTTOM_UP: u32 = 0x117;
const TGA_RUN_LENGTH: u32 = 0x118;
const TGA_ROW_CAPACITY: u32 = 0x160;
const TGA_ROW_BUFFER: u32 = 0x164;
const TGA_PALETTE_CAPACITY: u32 = 0x168;
const TGA_PALETTE: u32 = 0x16c;
const TGA_UNPACK: u32 = 0x170;
const TGA_RUN_LEFT: u32 = 0x174;
const TGA_RUN_REPEATS: u32 = 0x178;
const TGA_RUN_PIXEL: u32 = 0x179;

/// `NiBMPReader` fields as the PC code uses them: the width (+0x100), the height (+0x104),
/// the bit count (+0x14c, u16), the colours used (+0x150), the pixel data offset (+0x154)
/// and whether the rows are stored bottom-up (+0x158); the pixel format is at
/// [`BMP_PIXEL_FORMAT`].
const BMP_WIDTH: u32 = 0x100;
const BMP_HEIGHT: u32 = 0x104;
const BMP_BIT_COUNT: u32 = 0x14c;
const BMP_COLOURS: u32 = 0x150;
const BMP_DATA_OFFSET: u32 = 0x154;
const BMP_BOTTOM_UP: u32 = 0x158;

/// `EnterCriticalSection` and `LeaveCriticalSection` (import slots), `InterlockedIncrement`
/// and `InterlockedDecrement` (import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStricmp` (`00aa1360`, `cdecl`, two strings).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `_memcpy` (`00ec44d0`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// `00aa1070` (`cdecl`, the size): the byte allocator the row buffers come from;
/// `00aa13e0` (`cdecl`, the size): `NiNew`'s allocator.
const ALLOCATE_BYTES: u32 = 0x00aa_1070;
const NI_NEW: u32 = 0x00aa_13e0;
/// The memory manager's allocate (`00aa3e40`, unit `BSCore/memorymanager.cpp`; `thiscall`
/// on the singleton, the size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `NiPixelFormat::operator==` (`00a76540`, engine map; `thiscall`, the other format).
const PIXEL_FORMAT_EQUALS: u32 = 0x00a7_6540;
/// `NiPixelData::NiPixelData_ov2` (`00a7c190`, `thiscall`: width, height, the pixel format,
/// two more words).
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// `00a8b110` (unit `nipalette.cpp`, `thiscall`: a flag, the entry count and the entries;
/// answers the object) and `00a76a60` (unit `niimageconverter.cpp`, `thiscall` on the pixel
/// data, the palette).
const PALETTE_CONSTRUCT: u32 = 0x00a8_b110;
const PIXEL_DATA_SET_PALETTE: u32 = 0x00a7_6a60;
/// The word passed as the second argument of the stream's virtual `+0x14` (`010a2484`),
/// read at run time.
const SEEK_ORIGIN: u32 = 0x010a_2484;
/// The static pixel formats the readers choose from (`.data`).
const PIXEL_FORMAT_011AA180: u32 = 0x011a_a180;
const PIXEL_FORMAT_011AA1C8: u32 = 0x011a_a1c8;
const PIXEL_FORMAT_011AA210: u32 = 0x011a_a210;
const PIXEL_FORMAT_011AA258: u32 = 0x011a_a258;
const PIXEL_FORMAT_011AA2A0: u32 = 0x011a_a2a0;
/// `".tga"`, `".targa"` and `".bmp"`.
const EXTENSION_TGA: u32 = 0x0109_f4a4;
const EXTENSION_TARGA: u32 = 0x0109_f49c;
const EXTENSION_BMP: u32 = 0x0107_f794;

/// One read through the binary stream's read function (`stream` +8, `cdecl` (stream,
/// buffer, size, &component size, component count 1)): the word at `count` is set to
/// `component` first and the answer is added to the stream's position (+4).
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, count: u32, component: u32) {
    let function = e.mem.u32(stream + 8);
    e.mem.set_u32(count, component);
    let done = e
        .call(function, &args![stream, buffer, size, count, 1u32])
        .u32();
    let position = e.mem.u32(stream + 4).wrapping_add(done);
    e.mem.set_u32(stream + 4, position);
}

/// Copies the 0x11 words at `source` to `destination` (`REP MOVSD`).
fn copy_words(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..PIXEL_FORMAT_WORDS {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

/// The image width (`NiTGAReader` +0x10e, re-read on each pass as the exe does).
fn tga_width(e: &Engine, this: u32) -> u32 {
    u32::from(e.mem.u16(this + TGA_WIDTH))
}

/// The three bytes of a 2-byte (`lo`, `hi`) colour: `(hi & 0xfc) << 1`, `((lo >> 2) &
/// 0x38) + (hi << 6)` and `lo << 3`, in 8-bit arithmetic, each source byte read where the
/// exe reads it.
fn unpack_two_bytes(e: &mut Engine, pixel: u32, out: u32) {
    let high = e.mem.u8(pixel.wrapping_add(1)) & 0xfc;
    e.mem.set_u8(out, high.wrapping_add(high));
    let low = e.mem.u8(pixel);
    let high = e.mem.u8(pixel.wrapping_add(1));
    e.mem.set_u8(
        out.wrapping_add(1),
        ((low >> 2) & 0x38).wrapping_add(high << 6),
    );
    let low = e.mem.u8(pixel);
    e.mem.set_u8(out.wrapping_add(2), low << 3);
}

/// Copies the bytes `from` of `source` to the bytes `to` of `out`, one by one in order.
fn copy_bytes(e: &mut Engine, source: u32, out: u32, pairs: &[(u32, u32)]) {
    for (to, from) in pairs {
        let value = e.mem.u8(source.wrapping_add(*from));
        e.mem.set_u8(out.wrapping_add(*to), value);
    }
}

// Translated from 00a8d460 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall` on an
/// `NiTGAReader`, the source and destination rows; `RET 8`; the unpacker [`fn_00a8dce0`]
/// installs for 4-bit colour-mapped images): when the width has bits above bit 0, `width /
/// 2` bytes, each the source byte minus the first colour-map entry's low byte (+0x104).
pub fn fn_00a8d460(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    if tga_width(e, this) & 0xffff_fffe == 0 {
        return;
    }
    let mut i = 0u32;
    loop {
        let value = e
            .mem
            .u8(source.addr().wrapping_add(i))
            .wrapping_sub(e.mem.u8(this + TGA_FIRST_ENTRY));
        e.mem.set_u8(destination.addr().wrapping_add(i), value);
        i += 1;
        if i >= tga_width(e, this) >> 1 {
            break;
        }
    }
}

// Translated from 00a8d4a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, source and
/// destination rows; `RET 8`; the unpacker for 8-bit colour-mapped images): `width` bytes,
/// each the source byte minus the first colour-map entry's low byte (+0x104).
pub fn fn_00a8d4a0(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let value = e
            .mem
            .u8(source.addr().wrapping_add(i))
            .wrapping_sub(e.mem.u8(this + TGA_FIRST_ENTRY));
        e.mem.set_u8(destination.addr().wrapping_add(i), value);
        i += 1;
    }
}

// Translated from 00a8d4e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, source and
/// destination rows; `RET 8`; the unpacker for 8-bit grey images): each source byte becomes
/// three equal destination bytes (written third, second, first).
pub fn fn_00a8d4e0(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let value = e.mem.u8(source.addr().wrapping_add(i));
        let out = destination.addr().wrapping_add(3 * i);
        for offset in [2u32, 1, 0] {
            e.mem.set_u8(out.wrapping_add(offset), value);
        }
        i += 1;
    }
}

// Translated from 00a8d520 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, source and
/// destination rows; `RET 8`; the unpacker for 16-bit true-colour images): each 2-byte
/// source pixel becomes three bytes ([`unpack_two_bytes`]).
pub fn fn_00a8d520(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let pixel = source.addr().wrapping_add(2 * i);
        let out = destination.addr().wrapping_add(3 * i);
        unpack_two_bytes(e, pixel, out);
        i += 1;
    }
}

/// The palette entry of the 2-byte index at `pixel`: `(hi << 8) - first entry (+0x104) +
/// lo`, 4 bytes per entry from the palette at +0x16c (wrapping, as the exe's address
/// arithmetic).
fn palette_entry(e: &Engine, this: u32, pixel: u32) -> u32 {
    let index = (u32::from(e.mem.u8(pixel.wrapping_add(1))) << 8)
        .wrapping_sub(u32::from(e.mem.u16(this + TGA_FIRST_ENTRY)))
        .wrapping_add(u32::from(e.mem.u8(pixel)));
    e.mem
        .u32(this + TGA_PALETTE)
        .wrapping_add(index.wrapping_mul(4))
}

// Translated from 00a8d590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, source and
/// destination rows; `RET 8`; the unpacker for 16-bit colour-mapped images without 32-bit
/// colour): each 2-byte index becomes the first three bytes of its palette entry.
pub fn fn_00a8d590(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let entry = palette_entry(e, this, source.addr().wrapping_add(2 * i));
        let out = destination.addr().wrapping_add(3 * i);
        copy_bytes(e, entry, out, &[(0, 0), (1, 1), (2, 2)]);
        i += 1;
    }
}

// Translated from 00a8d600 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, source and
/// destination rows; `RET 8`; the unpacker for 16-bit colour-mapped images with 32-bit
/// colour): each 2-byte index becomes the four bytes of its palette entry.
pub fn fn_00a8d600(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let entry = palette_entry(e, this, source.addr().wrapping_add(2 * i));
        let out = destination.addr().wrapping_add(4 * i);
        copy_bytes(e, entry, out, &[(0, 0), (1, 1), (2, 2), (3, 3)]);
        i += 1;
    }
}

// Translated from 00a8d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, source and destination rows; `RET 8`; the
/// unpacker for 24-bit true-colour images): each 3-byte pixel is copied with its first and
/// third bytes swapped.
pub fn fn_00a8d670(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let pixel = source.addr().wrapping_add(3 * i);
        let out = destination.addr().wrapping_add(3 * i);
        copy_bytes(e, pixel, out, &[(0, 2), (1, 1), (2, 0)]);
        i += 1;
    }
}

// Translated from 00a8d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, source and destination rows; `RET 8`; the
/// unpacker for 32-bit true-colour images): each 4-byte pixel is copied with its first and
/// third bytes swapped.
pub fn fn_00a8d6c0(e: &mut Engine, this: Ptr, source: Ptr, destination: Ptr) {
    let this = this.addr();
    let mut i = 0u32;
    while i < tga_width(e, this) {
        let pixel = source.addr().wrapping_add(4 * i);
        let out = destination.addr().wrapping_add(4 * i);
        copy_bytes(e, pixel, out, &[(0, 2), (1, 1), (2, 0), (3, 3)]);
        i += 1;
    }
}

// Translated from 00a8d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, an extension; `RET 4`; the TGA reader's
/// extension test): `NiStricmp` against `".tga"`, then `".targa"`. The low byte of the
/// answer is 1 on a match, 0 otherwise; the upper bytes are those of the last `NiStricmp`
/// answer.
pub fn fn_00a8d710(e: &mut Engine, _this: Ptr, extension: Ptr) -> u32 {
    let first = e.call(NI_STRICMP, &args![extension, EXTENSION_TGA]).u32();
    if first == 0 {
        return 1;
    }
    let second = e.call(NI_STRICMP, &args![extension, EXTENSION_TARGA]).u32();
    if second == 0 {
        return 1;
    }
    second & 0xffff_ff00
}

// Translated from 00a8e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, an extension; `RET 4`; the BMP reader's extension
/// test): `NiStricmp` against `".bmp"`; the low byte of the answer is 1 when it matches
/// (`SETZ`), the upper bytes are `NiStricmp`'s.
pub fn fn_00a8e3f0(e: &mut Engine, _this: Ptr, extension: Ptr) -> u32 {
    let answer = e.call(NI_STRICMP, &args![extension, EXTENSION_BMP]).u32();
    (answer & 0xffff_ff00) | u32::from(answer == 0)
}

// Translated from 00a8d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: a binary stream, a buffer and a size): reads `size`
/// bytes into the buffer through the stream's read function (+8) with a component size of
/// 1 (a local word), adding the answer to the stream's position (+4).
pub fn fn_00a8d8b0(e: &mut Engine, stream: Ptr, buffer: Ptr, size: u32) {
    e.with_stack(4, |e, count| {
        stream_read(e, stream.addr(), buffer.addr(), size, count.addr(), 1);
    });
}

// Translated from 00a8d8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTGAReader`: the binary stream, the
/// destination row and a word it does not read; `RET 0xc`): reads one run-length encoded
/// row of `width` pixels of `bytes per pixel` (+0x114) bytes. While pixels are left: when
/// the current run is used up (+0x174 is 0) a 1-byte header is read (into the low byte of
/// the stream argument's word, cleared first): bit 7 set starts a repeat run of `header -
/// 0x7f` pixels whose pixel is read into +0x179, clear starts a raw run of `header + 1`
/// pixels; +0x178 holds which. Then `min(run left, pixels left in the row)` pixels are
/// either copied from +0x179 (`_memcpy`, one pixel at a time) or read straight into the row,
/// and the run and the row advance by that many. The component-size words are the stream
/// and row argument words, and the repeat loop counts down in the stream argument's word.
pub fn fn_00a8d8e0(e: &mut Engine, this: Ptr, stream: Ptr, row: Ptr, _unused_3: u32) {
    let this = this.addr();
    let stream = stream.addr();
    let mut out = row.addr();
    if tga_width(e, this) == 0 {
        return;
    }
    // Frame: the stream and row argument words.
    e.with_stack(8, |e, frame| {
        let stream_word = frame.addr();
        let row_word = frame.addr() + 4;
        e.mem.set_u32(stream_word, stream);
        e.mem.set_u32(row_word, row.addr());
        let mut done = 0u32;
        loop {
            if e.mem.u32(this + TGA_RUN_LEFT) == 0 {
                e.mem.set_u8(stream_word, 0);
                stream_read(e, stream, stream_word, 1, row_word, 1);
                let header = e.mem.u8(stream_word);
                e.mem.set_u8(this + TGA_RUN_REPEATS, header >> 7);
                if header & 0x80 != 0 {
                    let size = u32::from(e.mem.u8(this + TGA_BYTES_PER_PIXEL));
                    e.mem
                        .set_u32(this + TGA_RUN_LEFT, u32::from(header).wrapping_sub(0x7f));
                    e.mem.set_u8(this + TGA_RUN_REPEATS, 1);
                    stream_read(e, stream, this + TGA_RUN_PIXEL, size, row_word, 1);
                } else {
                    e.mem.set_u8(this + TGA_RUN_REPEATS, 0);
                    e.mem.set_u32(this + TGA_RUN_LEFT, u32::from(header) + 1);
                }
            }
            let left_in_row = tga_width(e, this).wrapping_sub(done);
            let run_left = e.mem.u32(this + TGA_RUN_LEFT);
            let count = if run_left < left_in_row {
                run_left
            } else {
                left_in_row
            };
            if e.mem.u8(this + TGA_RUN_REPEATS) != 0 {
                if count != 0 {
                    e.mem.set_u32(stream_word, count);
                    loop {
                        let size = u32::from(e.mem.u8(this + TGA_BYTES_PER_PIXEL));
                        e.call(MEMCPY, &args![out, this + TGA_RUN_PIXEL, size]);
                        let size = u32::from(e.mem.u8(this + TGA_BYTES_PER_PIXEL));
                        out = out.wrapping_add(size);
                        let left = e.mem.u32(stream_word).wrapping_sub(1);
                        e.mem.set_u32(stream_word, left);
                        if left == 0 {
                            break;
                        }
                    }
                }
            } else {
                let size = u32::from(e.mem.u8(this + TGA_BYTES_PER_PIXEL)).wrapping_mul(count);
                stream_read(e, stream, out, size, stream_word, 1);
                let size = u32::from(e.mem.u8(this + TGA_BYTES_PER_PIXEL)).wrapping_mul(count);
                out = out.wrapping_add(size);
            }
            let run_left = e.mem.u32(this + TGA_RUN_LEFT).wrapping_sub(count);
            e.mem.set_u32(this + TGA_RUN_LEFT, run_left);
            done = done.wrapping_add(count);
            if done >= tga_width(e, this) {
                break;
            }
        }
    });
}

// Translated from 00a8da50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTGAReader`, the binary stream; `RET 4`):
/// reads the colour map into the palette (+0x16c, 4 bytes per entry). The palette grows to
/// the map length (+0x106) when its capacity (+0x168) is smaller (the old block given back
/// to the memory manager, `length * 4` bytes allocated, 0xffffffff when that overflows),
/// and the row buffer (+0x164) to `length * 4` bytes when its size (+0x160) is smaller
/// (`NiFree`, `00aa1070`). Then by the entry size (+0x108; the exe's byte and jump tables on
/// `size - 8`): 8 bits are grey bytes made three equal bytes and alpha 0xff, 15 and 16 bits
/// are 2-byte entries unpacked as [`fn_00a8d520`] does with alpha 0xff, 24 bits have their
/// first and third bytes swapped and alpha 0xff, 32 bits their first and third bytes
/// swapped; other sizes read nothing. The entries are read whole into the row buffer first
/// ([`fn_00a8d8b0`]).
pub fn fn_00a8da50(e: &mut Engine, this: Ptr, stream: Ptr) {
    let this = this.addr();
    let length = u32::from(e.mem.u16(this + TGA_MAP_LENGTH));
    if e.mem.u32(this + TGA_PALETTE_CAPACITY) < length {
        e.mem.set_u32(this + TGA_PALETTE_CAPACITY, length);
        let old = e.mem.u32(this + TGA_PALETTE);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
        let capacity = e.mem.u32(this + TGA_PALETTE_CAPACITY);
        let size = capacity.checked_mul(4).unwrap_or(0xffff_ffff);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.mem.set_u32(this + TGA_PALETTE, block);
    }
    let bytes = u32::from(e.mem.u16(this + TGA_MAP_LENGTH)).wrapping_mul(4);
    if e.mem.u32(this + TGA_ROW_CAPACITY) < bytes {
        e.mem.set_u32(this + TGA_ROW_CAPACITY, bytes);
        let old = e.mem.u32(this + TGA_ROW_BUFFER);
        e.call(NI_FREE, &args![old]);
        let size = e.mem.u32(this + TGA_ROW_CAPACITY);
        let block = e.call(ALLOCATE_BYTES, &args![size]).u32();
        e.mem.set_u32(this + TGA_ROW_BUFFER, block);
    }
    let mut out = e.mem.u32(this + TGA_PALETTE);
    let entry_size = match e.mem.u8(this + TGA_ENTRY_BITS) {
        8 => 1u32,
        0xf | 0x10 => 2,
        0x18 => 3,
        0x20 => 4,
        _ => return,
    };
    let length = u32::from(e.mem.u16(this + TGA_MAP_LENGTH));
    let mut source = e.mem.u32(this + TGA_ROW_BUFFER);
    fn_00a8d8b0(e, stream, Ptr::new(source), length.wrapping_mul(entry_size));
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(this + TGA_MAP_LENGTH)) {
        match entry_size {
            1 => {
                let value = e.mem.u8(source);
                for offset in [2u32, 1, 0] {
                    e.mem.set_u8(out.wrapping_add(offset), value);
                }
                e.mem.set_u8(out.wrapping_add(3), 0xff);
            }
            2 => {
                unpack_two_bytes(e, source, out);
                e.mem.set_u8(out.wrapping_add(3), 0xff);
            }
            3 => {
                copy_bytes(e, source, out, &[(2, 0), (1, 1), (0, 2)]);
                e.mem.set_u8(out.wrapping_add(3), 0xff);
            }
            _ => copy_bytes(e, source, out, &[(2, 0), (1, 1), (0, 2), (3, 3)]),
        }
        i += 1;
        out = out.wrapping_add(4);
        source = source.wrapping_add(entry_size);
    }
}

// Translated from 00a8dce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTGAReader`, `RET 0x18`: the binary
/// stream, then where to put the width, the height, the pixel format (0x44 bytes), a byte
/// and a word; the TGA reader's header read, its virtual `+0xc` as [`fn_00a8e120`] calls
/// it). Seeks the stream to 0 (its virtual `+0x10`), enters the reader's critical section
/// (+0x80), stores 1 in the last output, reads the header field by field (+0x100 to +0x113;
/// the component-size word is the last argument's own word), skips the image ID (the
/// stream's virtual `+0x14` with the ID length and the word at `010a2484`), sets bottom-up
/// (+0x117) from bit 5 of the descriptor (clear means bottom-up), and by the image type (the
/// exe's jump table on `type - 1`): 1 and 9 set colour-mapped, 2, 3, 10 and 11 clear it,
/// with run-length (+0x118) set for 9 to 11; any other type fails. 32-bit colour (+0x115)
/// is whether the entry size (mapped) or the pixel depth is 32; the pixel format (+0x11c)
/// is the static one for the case (mapped with a 16-entry map `011aa1c8`/`011aa180`, other
/// mapped `011aa258`/`011aa210`, true colour `011aa2a0`/`011aa2e8`, the 32-bit one first).
/// The pixel depth (the exe's byte table on `depth - 4`) sets bytes per pixel (+0x114) and
/// the row unpacker (+0x170): 4 bits (mapped only, else it fails) [`fn_00a8d460`], 8 bits
/// [`fn_00a8d4a0`] or [`fn_00a8d4e0`], 15 and 16 bits [`fn_00a8d600`], [`fn_00a8d590`] or
/// [`fn_00a8d520`], 24 bits [`fn_00a8d670`], 32 bits [`fn_00a8d6c0`]; other depths set
/// neither. On success the run state is cleared, the width, height and pixel format written
/// out, the byte output cleared, and it answers true; it leaves the critical section either
/// way.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a8dce0(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    width_out: Ptr,
    height_out: Ptr,
    format_out: Ptr,
    flag_out: Ptr,
    last_out: Ptr,
) -> bool {
    let this = this.addr();
    let stream = stream.addr();
    e.vcall(stream, 0x10, &args![0u32]);
    let section = this + 0x80;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    // Frame: the last argument's word (the component-size word).
    e.with_stack(4, |e, count| {
        let count = count.addr();
        e.mem.set_u32(count, last_out.addr());
        e.mem.set_u32(last_out.addr(), 1);
        for (field, size) in [
            (TGA_ID_LENGTH, 1u32),
            (TGA_COLOUR_MAP_TYPE, 1),
            (TGA_IMAGE_TYPE, 1),
            (TGA_FIRST_ENTRY, 2),
            (TGA_MAP_LENGTH, 2),
            (TGA_ENTRY_BITS, 1),
            (0x10a, 2),
            (0x10c, 2),
            (TGA_WIDTH, 2),
            (TGA_HEIGHT, 2),
            (TGA_PIXEL_DEPTH, 1),
            (TGA_DESCRIPTOR, 1),
        ] {
            stream_read(e, stream, this + field, size, count, size);
        }
    });
    let id_length = e.mem.u8(this + TGA_ID_LENGTH);
    if id_length != 0 {
        let origin = e.global::<u32>(SEEK_ORIGIN);
        e.vcall(stream, 0x14, &args![u32::from(id_length), origin]);
    }
    let descriptor = e.mem.u8(this + TGA_DESCRIPTOR);
    e.mem.set_u8(this + TGA_BOTTOM_UP, !(descriptor >> 5) & 1);
    let (mapped, run_length) = match e.mem.u8(this + TGA_IMAGE_TYPE) {
        1 => (1u8, 0u8),
        2 | 3 => (0, 0),
        9 => (1, 1),
        10 | 11 => (0, 1),
        _ => {
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            return false;
        }
    };
    e.mem.set_u8(this + TGA_MAPPED, mapped);
    e.mem.set_u8(this + TGA_RUN_LENGTH, run_length);
    let mapped = e.mem.u8(this + TGA_MAPPED) != 0;
    let bits = if mapped {
        e.mem.u8(this + TGA_ENTRY_BITS)
    } else {
        e.mem.u8(this + TGA_PIXEL_DEPTH)
    };
    let thirty_two = bits == 0x20;
    e.mem
        .set_u8(this + TGA_THIRTY_TWO_BIT, u8::from(thirty_two));
    let preset = match (mapped, thirty_two) {
        (false, true) => PIXEL_FORMAT_011AA2A0,
        (false, false) => PIXEL_FORMAT_SOURCE_011AA2E8,
        (true, _) if e.mem.u16(this + TGA_MAP_LENGTH) == 0x10 => {
            if thirty_two {
                PIXEL_FORMAT_011AA1C8
            } else {
                PIXEL_FORMAT_011AA180
            }
        }
        (true, true) => PIXEL_FORMAT_011AA258,
        (true, false) => PIXEL_FORMAT_011AA210,
    };
    copy_words(e, this + TGA_PIXEL_FORMAT, preset);
    let unpack = match e.mem.u8(this + TGA_PIXEL_DEPTH) {
        4 => {
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 0);
            if !mapped {
                e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                return false;
            }
            Some(0x00a8_d460u32)
        }
        8 => {
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 1);
            Some(if mapped { 0x00a8_d4a0 } else { 0x00a8_d4e0 })
        }
        0xf | 0x10 => {
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 2);
            Some(match (mapped, thirty_two) {
                (false, _) => 0x00a8_d520,
                (true, true) => 0x00a8_d600,
                (true, false) => 0x00a8_d590,
            })
        }
        0x18 => {
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 3);
            Some(0x00a8_d670)
        }
        0x20 => {
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 4);
            Some(0x00a8_d6c0)
        }
        _ => None,
    };
    if let Some(unpack) = unpack {
        e.mem.set_u32(this + TGA_UNPACK, unpack);
    }
    let width = u32::from(e.mem.u16(this + TGA_WIDTH));
    e.mem.set_u8(this + TGA_RUN_REPEATS, 0);
    e.mem.set_u32(this + TGA_RUN_LEFT, 0);
    e.mem.set_u32(width_out.addr(), width);
    let height = u32::from(e.mem.u16(this + TGA_HEIGHT));
    e.mem.set_u32(height_out.addr(), height);
    copy_words(e, format_out.addr(), this + TGA_PIXEL_FORMAT);
    e.mem.set_u8(flag_out.addr(), 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    true
}

/// The pixel data the readers fill: reused when given and its first level's width and
/// height (the words its +0x54 and +0x58 point at) and its pixel format (+8,
/// `NiPixelFormat::operator==`) match the reader's; otherwise a new one (`00aa13e0` 0x74
/// bytes, `00a7c190` with the width, height, the reader's format, 1 and 1; null when the
/// allocation fails).
fn pixel_data_for(e: &mut Engine, reuse: Ptr, width: u32, height: u32, format: u32) -> u32 {
    let reused = !reuse.is_null() && {
        let data = reuse.addr();
        e.mem.u32(e.mem.u32(data + 0x54)) == width
            && e.mem.u32(e.mem.u32(data + 0x58)) == height
            && e.call(PIXEL_FORMAT_EQUALS, &args![data + 8, format]).u8() != 0
    };
    if reused {
        return reuse.addr();
    }
    let block = e.call(NI_NEW, &args![0x74u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        NI_PIXEL_DATA_CONSTRUCT,
        &args![block, width, height, format, 1u32, 1u32],
    )
    .u32()
}

/// The first level's first row of a pixel data: the word its +0x5c points at plus its
/// pixels (+0x50).
fn first_row(e: &Engine, data: u32) -> u32 {
    e.mem
        .u32(e.mem.u32(data + 0x5c))
        .wrapping_add(e.mem.u32(data + 0x50))
}

// Translated from 00a8e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTGAReader`: the binary stream and a pixel
/// data to reuse; `RET 8`; the TGA reader's file read). Constructs a local pixel format
/// (`00a7cca0`), enters the critical section, reads the header (virtual `+0xc` with frame
/// locals for its outputs; a false answer gives null). The row size is `bytes per pixel *
/// width`, or `width / 2` for 4-bit images (any other zero-byte depth fails); images with a
/// colour map fail when `map length + first entry + 1` reaches 0x4000 and read their map
/// ([`fn_00a8da50`]); a set colour-mapped flag without a map fails. The pixel data is reused
/// or made ([`pixel_data_for`] with the width and height); colour-mapped images give it a
/// new palette (`00aa13e0` 0x24 bytes, `00a8b110` with the 32-bit flag, the map length and
/// the palette; null on failure) through `00a76a60`. The row buffer grows to the row size
/// when smaller. Then every row is read (run-length rows through [`fn_00a8d8e0`], others in
/// one read with a component size of 1) and unpacked by the function at +0x170 into the
/// first level, from the top or, when bottom-up, from the last row up (the row stride is
/// the first level's width times the word at +0x64, or half the width when that is 0).
/// Leaves the critical section and answers the pixel data.
pub fn fn_00a8e120(e: &mut Engine, this: Ptr, stream: Ptr, reuse: Ptr) -> Ptr {
    let this = this.addr();
    let stream_word = stream.addr();
    // Frame (the exe's locals): +0xf the header's byte output, +0x10 the row size, +0x14 the
    // component-size word, +0x18 the critical section, +0x1c the header's last output,
    // +0x20 the height, +0x24 the width, +0x28 the pixel format.
    e.with_stack(0x6c, |e, frame| {
        let frame = frame.addr();
        let format = frame + 0x28;
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
        let section = this + 0x80;
        e.mem.set_u32(frame + 0x18, section);
        e.call(ENTER_CRITICAL_SECTION, &args![section]);
        let read = e
            .vcall(
                this,
                0xc,
                &args![
                    stream_word,
                    frame + 0x24,
                    frame + 0x20,
                    format,
                    frame + 0xf,
                    frame + 0x1c
                ],
            )
            .u8();
        let fail = |e: &mut Engine| {
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            Ptr::new(0)
        };
        if read == 0 {
            return fail(e);
        }
        let bytes_per_pixel = e.mem.u8(this + TGA_BYTES_PER_PIXEL);
        let width = u32::from(e.mem.u16(this + TGA_WIDTH));
        let mut row_size = u32::from(bytes_per_pixel).wrapping_mul(width);
        e.mem.set_u32(frame + 0x10, row_size);
        if bytes_per_pixel == 0 {
            if e.mem.u8(this + TGA_PIXEL_DEPTH) != 4 {
                return fail(e);
            }
            row_size = width >> 1;
            e.mem.set_u32(frame + 0x10, row_size);
        }
        if e.mem.u8(this + TGA_COLOUR_MAP_TYPE) != 0 {
            let entries = u32::from(e.mem.u16(this + TGA_MAP_LENGTH))
                + u32::from(e.mem.u16(this + TGA_FIRST_ENTRY))
                + 1;
            if entries >= 0x4000 {
                return fail(e);
            }
            fn_00a8da50(e, Ptr::new(this), stream);
        } else if e.mem.u8(this + TGA_MAPPED) != 0 {
            return fail(e);
        }
        let width = u32::from(e.mem.u16(this + TGA_WIDTH));
        let height = u32::from(e.mem.u16(this + TGA_HEIGHT));
        let data = pixel_data_for(e, reuse, width, height, this + TGA_PIXEL_FORMAT);
        if e.mem.u8(this + TGA_MAPPED) != 0 {
            let block = e.call(NI_NEW, &args![0x24u32]).u32();
            let palette = if block == 0 {
                0
            } else {
                let entries = e.mem.u32(this + TGA_PALETTE);
                let length = u32::from(e.mem.u16(this + TGA_MAP_LENGTH));
                let alpha = u32::from(e.mem.u8(this + TGA_THIRTY_TWO_BIT));
                e.call(PALETTE_CONSTRUCT, &args![block, alpha, length, entries])
                    .u32()
            };
            e.call(PIXEL_DATA_SET_PALETTE, &args![data, palette]);
        }
        if e.mem.u32(this + TGA_ROW_CAPACITY) < row_size {
            let old = e.mem.u32(this + TGA_ROW_BUFFER);
            e.mem.set_u32(this + TGA_ROW_CAPACITY, row_size);
            e.call(NI_FREE, &args![old]);
            let size = e.mem.u32(this + TGA_ROW_CAPACITY);
            let block = e.call(ALLOCATE_BYTES, &args![size]).u32();
            e.mem.set_u32(this + TGA_ROW_BUFFER, block);
        }
        let mut row = first_row(e, data);
        let level_width = e.mem.u32(e.mem.u32(data + 0x54));
        let mut stride = e.mem.u32(data + 0x64).wrapping_mul(level_width);
        if stride == 0 {
            stride = level_width >> 1;
        }
        if e.mem.u8(this + TGA_BOTTOM_UP) != 0 {
            let last = u32::from(e.mem.u16(this + TGA_HEIGHT)).wrapping_sub(1);
            row = row.wrapping_add(last.wrapping_mul(stride));
            stride = stride.wrapping_neg();
        }
        let run_length = e.mem.u8(this + TGA_RUN_LENGTH) != 0;
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(this + TGA_HEIGHT)) {
            if run_length {
                let buffer = e.mem.u32(this + TGA_ROW_BUFFER);
                let capacity = e.mem.u32(this + TGA_ROW_CAPACITY);
                fn_00a8d8e0(e, Ptr::new(this), stream, Ptr::new(buffer), capacity);
            } else {
                let size = e.mem.u32(frame + 0x10);
                let buffer = e.mem.u32(this + TGA_ROW_BUFFER);
                stream_read(e, stream_word, buffer, size, frame + 0x14, 1);
            }
            let buffer = e.mem.u32(this + TGA_ROW_BUFFER);
            let unpack = e.mem.u32(this + TGA_UNPACK);
            e.call(unpack, &args![this, buffer, row]);
            i += 1;
            row = row.wrapping_add(stride);
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        Ptr::new(data)
    })
}

// Translated from 00a8e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiBMPReader`, `RET 0x18`: the binary
/// stream, then where to put the width, the height, the pixel format (0x44 bytes), a byte
/// and a word; the BMP reader's header read). Seeks the stream to 0 (virtual `+0x10`),
/// stores 1 in the last output and reads the 2-byte signature (into the stream argument's
/// word; the component-size word is the last argument's word): anything but `BM` answers
/// false at once. Otherwise enters the critical section (+0x80) and reads the file and info
/// headers field by field into frame words and the reader (the data offset +0x154, the bit
/// count +0x14c, the colours used +0x150; the others into frame words). It fails when the
/// info header's size word is 4, when the bit count is not 4, 8, 24 or 32 (pixel formats
/// `011aa210` for 4 and 8, `011aa2e8`, `011aa2a0`; the exe's byte table on `bits - 4`) or
/// when the compression is 1 or 2 (after the pixel format is copied over +0x108). On
/// success: the width (+0x100), the bottom-up flag (+0x158: the height is not negative) and
/// the height's magnitude (+0x104; a height that is not positive is negated), a zero colour
/// count becomes `1 << bits` for depths other than 24 and 32; the width, height and pixel
/// format are written out, the byte output set to 1, and it answers true. It leaves the
/// critical section either way.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a8e4b0(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    width_out: Ptr,
    height_out: Ptr,
    format_out: Ptr,
    flag_out: Ptr,
    last_out: Ptr,
) -> bool {
    let this = this.addr();
    let stream_address = stream.addr();
    e.vcall(stream_address, 0x10, &args![0u32]);
    // Frame: W0..W6 at +0..+0x18 (the exe's locals), the return address word +0x1c, the
    // stream argument word +0x20, the last argument word +0x34.
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        let w = |i: u32| frame + 4 * i;
        let stream_word = frame + 0x20;
        let last_word = frame + 0x34;
        e.mem.set_u32(stream_word, stream_address);
        e.mem.set_u32(last_word, last_out.addr());
        e.mem.set_u32(last_out.addr(), 1);
        stream_read(e, stream_address, stream_word, 2, last_word, 2);
        if e.mem.u16(stream_word) != 0x4d42 {
            return false;
        }
        let section = this + 0x80;
        e.mem.set_u32(last_word, section);
        e.call(ENTER_CRITICAL_SECTION, &args![section]);
        let count = w(1);
        for (buffer, size) in [
            (w(0), 4u32),
            (stream_word, 2),
            (stream_word, 2),
            (this + BMP_DATA_OFFSET, 4),
            (w(3), 4),
            (w(5), 4),
            (w(6), 4),
            (stream_word, 2),
            (this + BMP_BIT_COUNT, 2),
            (w(4), 4),
            (w(0), 4),
            (w(2), 4),
            (w(2), 4),
            (this + BMP_COLOURS, 4),
            (w(0), 4),
        ] {
            stream_read(e, stream_address, buffer, size, count, size);
        }
        let fail = |e: &mut Engine| {
            let section = e.mem.u32(last_word);
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            false
        };
        if e.mem.u32(w(3)) == 4 {
            return fail(e);
        }
        let bits = e.mem.u16(this + BMP_BIT_COUNT);
        e.mem.set_u32(w(1), u32::from(bits));
        let preset = match bits {
            4 | 8 => PIXEL_FORMAT_011AA210,
            0x18 => PIXEL_FORMAT_SOURCE_011AA2E8,
            0x20 => PIXEL_FORMAT_011AA2A0,
            _ => return fail(e),
        };
        copy_words(e, this + BMP_PIXEL_FORMAT, preset);
        let compression = e.mem.u32(w(4));
        if compression == 2 || compression == 1 {
            return fail(e);
        }
        let height = e.mem.u32(w(6)) as i32;
        let width = e.mem.u32(w(5));
        e.mem.set_u32(this + BMP_WIDTH, width);
        e.mem.set_u8(this + BMP_BOTTOM_UP, u8::from(height >= 0));
        let magnitude = if height > 0 {
            height
        } else {
            height.wrapping_neg()
        };
        e.mem.set_u32(this + BMP_HEIGHT, magnitude as u32);
        if e.mem.u32(this + BMP_COLOURS) == 0 && bits != 0x18 && bits != 0x20 {
            let shift = e.mem.u32(w(1)) & 0x1f;
            e.mem.set_u32(this + BMP_COLOURS, 1u32 << shift);
        }
        e.mem.set_u32(width_out.addr(), width);
        let height = e.mem.u32(this + BMP_HEIGHT);
        e.mem.set_u32(height_out.addr(), height);
        copy_words(e, format_out.addr(), this + BMP_PIXEL_FORMAT);
        e.mem.set_u8(flag_out.addr(), 1);
        let section = e.mem.u32(last_word);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        true
    })
}

// Translated from 00a8e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiBMPReader`: the binary stream and a pixel
/// data to reuse; `RET 8`; the BMP reader's file read). Constructs a local pixel format
/// (`00a7cca0`), enters the critical section, reads the header (virtual `+0xc`; a false
/// answer gives null). The pixel data is reused or made ([`pixel_data_for`] with the width
/// and height words). For the 8-bit palette format (`011aa210`) the `colours * 4` palette
/// bytes are read into a 0x400-byte frame buffer, each entry's first and third bytes
/// swapped and its fourth set to 0xff, and a new palette (`00a8b110` with 0, 0x100 and the
/// buffer; null on failure) replaces the pixel data's (+0x4c, counted: the old one released
/// through its slot `+4` when its count reaches 0); otherwise the palette bytes are skipped
/// (virtual `+0x14` with the word at `010a2484`). The bytes between the palette and the data
/// offset (`offset - colours * 4 - 0x36`, when positive) are skipped too. Then the rows:
/// 4-bit images read each padded row (`(width + 1) / 2` bytes rounded up to 4) into a
/// buffer (`00aa1070`, freed with `NiFree`) and split each byte into two pixels (high
/// nibble first); other depths read each row of `first level width * bytes per pixel`
/// bytes straight into the pixel data, followed by its padding to 4 bytes (into a frame
/// word), or, top-down with no padding, the whole image (`((bits * width + 31) >> 3 &
/// 0x1ffffffc) * height` bytes) in one read; bottom-up images fill from the last row up.
/// Finally 24- and 32-bit formats (`011aa2e8`, `011aa2a0`) have the first and third bytes of
/// every pixel swapped. Leaves the critical section and answers the pixel data.
pub fn fn_00a8e810(e: &mut Engine, this: Ptr, stream: Ptr, reuse: Ptr) -> Ptr {
    let this = this.addr();
    let stream_address = stream.addr();
    // Frame (the exe's locals): +4 a component-size word, +0xc the old palette and another
    // component-size word, +0x10 the padding size, +0x17 the header's byte output, +0x18 a
    // component-size word and the padding bytes, +0x20 the critical section, +0x24 the
    // header's last output, +0x28 the width, +0x2c the height, +0x30 the pixel format, +0x74
    // the palette buffer (0x400 bytes).
    e.with_stack(0x474, |e, frame| {
        let frame = frame.addr();
        let format = frame + 0x30;
        let palette_buffer = frame + 0x74;
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
        let section = this + 0x80;
        e.mem.set_u32(frame + 0x20, section);
        e.call(ENTER_CRITICAL_SECTION, &args![section]);
        let read = e
            .vcall(
                this,
                0xc,
                &args![
                    stream_address,
                    frame + 0x28,
                    frame + 0x2c,
                    format,
                    frame + 0x17,
                    frame + 0x24
                ],
            )
            .u8();
        if read == 0 {
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            return Ptr::new(0);
        }
        let width = e.mem.u32(this + BMP_WIDTH);
        let height = e.mem.u32(this + BMP_HEIGHT);
        let data = pixel_data_for(e, reuse, width, height, this + BMP_PIXEL_FORMAT);
        let palettized = e
            .call(
                PIXEL_FORMAT_EQUALS,
                &args![this + BMP_PIXEL_FORMAT, PIXEL_FORMAT_011AA210],
            )
            .u8()
            != 0;
        if palettized {
            let size = e.mem.u32(this + BMP_COLOURS).wrapping_mul(4);
            stream_read(e, stream_address, palette_buffer, size, frame + 0xc, 1);
            let colours = e.mem.u32(this + BMP_COLOURS);
            let mut entry = palette_buffer;
            for _ in 0..colours {
                let first = e.mem.u8(entry);
                let third = e.mem.u8(entry.wrapping_add(2));
                e.mem.set_u8(entry, third);
                e.mem.set_u8(entry.wrapping_add(2), first);
                e.mem.set_u8(entry.wrapping_add(3), 0xff);
                entry = entry.wrapping_add(4);
            }
            let block = e.call(NI_NEW, &args![0x24u32]).u32();
            let palette = if block == 0 {
                0
            } else {
                e.call(
                    PALETTE_CONSTRUCT,
                    &args![block, 0u32, 0x100u32, palette_buffer],
                )
                .u32()
            };
            let old = e.mem.u32(data + 0x4c);
            e.mem.set_u32(frame + 0xc, old);
            if old != palette {
                if old != 0
                    && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                        .u32()
                        == 0
                {
                    let old = e.mem.u32(frame + 0xc);
                    e.vcall(old, 4, &args![]);
                }
                e.mem.set_u32(data + 0x4c, palette);
                if palette != 0 {
                    e.call(INTERLOCKED_INCREMENT, &args![palette.wrapping_add(4)]);
                }
            }
        } else {
            let size = e.mem.u32(this + BMP_COLOURS).wrapping_mul(4);
            let origin = e.global::<u32>(SEEK_ORIGIN);
            e.vcall(stream_address, 0x14, &args![size, origin]);
        }
        let gap = e
            .mem
            .u32(this + BMP_DATA_OFFSET)
            .wrapping_sub(e.mem.u32(this + BMP_COLOURS).wrapping_mul(4))
            .wrapping_sub(0x36) as i32;
        if gap > 0 {
            let origin = e.global::<u32>(SEEK_ORIGIN);
            e.vcall(stream_address, 0x14, &args![gap as u32, origin]);
        }
        let bits = e.mem.u16(this + BMP_BIT_COUNT);
        let height = e.mem.u32(this + BMP_HEIGHT);
        let mut row_bytes = e
            .mem
            .u32(e.mem.u32(data + 0x54))
            .wrapping_mul(e.mem.u32(data + 0x64));
        let total = ((u32::from(bits)
            .wrapping_mul(e.mem.u32(this + BMP_WIDTH))
            .wrapping_add(0x1f)
            >> 3)
            & 0x1fff_fffc)
            .wrapping_mul(height);
        let bottom_up = e.mem.u8(this + BMP_BOTTOM_UP) != 0;
        if bits == 4 {
            let width = e.mem.u32(this + BMP_WIDTH);
            let half = (width >> 1).wrapping_add(width & 1);
            let pad = if half & 3 == 0 { 0 } else { 4 - (half & 3) };
            let size = half.wrapping_add(pad);
            let buffer = e.call(ALLOCATE_BYTES, &args![size]).u32();
            let mut row = if bottom_up {
                let level_height = e.mem.u32(e.mem.u32(data + 0x58));
                let row = level_height
                    .wrapping_sub(1)
                    .wrapping_mul(row_bytes)
                    .wrapping_add(first_row(e, data));
                row_bytes = row_bytes.wrapping_neg();
                row
            } else {
                first_row(e, data)
            };
            let mut i = 0u32;
            while i < e.mem.u32(this + BMP_HEIGHT) {
                stream_read(e, stream_address, buffer, size, frame + 0x18, 1);
                let mut out = row;
                for k in 0..half {
                    let byte = e.mem.u8(buffer.wrapping_add(k));
                    e.mem.set_u8(out, byte >> 4);
                    let byte = e.mem.u8(buffer.wrapping_add(k));
                    e.mem.set_u8(out.wrapping_add(1), byte & 0xf);
                    out = out.wrapping_add(2);
                }
                row = row.wrapping_add(row_bytes);
                i += 1;
            }
            e.call(NI_FREE, &args![buffer]);
        } else if bottom_up {
            let mut row = height
                .wrapping_sub(1)
                .wrapping_mul(row_bytes)
                .wrapping_add(first_row(e, data));
            let pad = row_bytes & 3;
            if pad != 0 {
                e.mem.set_u32(frame + 0x10, 4 - pad);
            }
            let mut i = 0u32;
            while i < e.mem.u32(this + BMP_HEIGHT) {
                stream_read(e, stream_address, row, row_bytes, frame + 4, 1);
                if pad != 0 {
                    let pad = e.mem.u32(frame + 0x10);
                    stream_read(e, stream_address, frame + 0x18, pad, frame + 4, 1);
                }
                i += 1;
                row = row.wrapping_sub(row_bytes);
            }
        } else if row_bytes & 3 != 0 {
            e.mem.set_u32(frame + 0x10, 4 - (row_bytes & 3));
            let mut row = first_row(e, data);
            let mut i = 0u32;
            while i < e.mem.u32(this + BMP_HEIGHT) {
                stream_read(e, stream_address, row, row_bytes, frame + 4, 1);
                let pad = e.mem.u32(frame + 0x10);
                stream_read(e, stream_address, frame + 0x18, pad, frame + 4, 1);
                i += 1;
                row = row.wrapping_add(row_bytes);
            }
        } else {
            let start = first_row(e, data);
            stream_read(e, stream_address, start, total, frame + 4, 1);
        }
        let swap = e
            .call(
                PIXEL_FORMAT_EQUALS,
                &args![this + BMP_PIXEL_FORMAT, PIXEL_FORMAT_SOURCE_011AA2E8],
            )
            .u8()
            != 0
            || e.call(
                PIXEL_FORMAT_EQUALS,
                &args![this + BMP_PIXEL_FORMAT, PIXEL_FORMAT_011AA2A0],
            )
            .u8()
                != 0;
        if swap {
            let pixels = e
                .mem
                .u32(this + BMP_HEIGHT)
                .wrapping_mul(e.mem.u32(this + BMP_WIDTH));
            let mut pixel = first_row(e, data);
            let stride = e.mem.u32(data + 0x64);
            for _ in 0..pixels {
                let first = e.mem.u8(pixel);
                let third = e.mem.u8(pixel.wrapping_add(2));
                e.mem.set_u8(pixel, third);
                e.mem.set_u8(pixel.wrapping_add(2), first);
                pixel = pixel.wrapping_add(stride);
            }
        }
        let section = e.mem.u32(frame + 0x20);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        Ptr::new(data)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_d750, fn_00a8d750(Ptr) -> Ptr),
        entry!(0x00a8_e410, fn_00a8e410(Ptr) -> Ptr),
        entry!(0x00a8_e480, fn_00a8e480(Ptr, u32) -> Ptr),
        entry!(0x00a8_d850, fn_00a8d850(Ptr, u32) -> Ptr),
        entry!(0x00a8_d460, fn_00a8d460(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d4a0, fn_00a8d4a0(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d4e0, fn_00a8d4e0(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d520, fn_00a8d520(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d590, fn_00a8d590(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d600, fn_00a8d600(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d670, fn_00a8d670(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d6c0, fn_00a8d6c0(Ptr, Ptr, Ptr)),
        entry!(0x00a8_d710, fn_00a8d710(Ptr, Ptr) -> u32),
        entry!(0x00a8_e3f0, fn_00a8e3f0(Ptr, Ptr) -> u32),
        entry!(0x00a8_d8b0, fn_00a8d8b0(Ptr, Ptr, u32)),
        entry!(0x00a8_d8e0, fn_00a8d8e0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00a8_da50, fn_00a8da50(Ptr, Ptr)),
        entry!(
            0x00a8_dce0,
            fn_00a8dce0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a8_e120, fn_00a8e120(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00a8_e4b0,
            fn_00a8e4b0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a8_e810, fn_00a8e810(Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(PIXEL_FORMAT_SOURCE_011AA2E8, 4 * PIXEL_FORMAT_WORDS);
        for word in 0..PIXEL_FORMAT_WORDS {
            e.mem
                .set_u32(PIXEL_FORMAT_SOURCE_011AA2E8 + 4 * word, 0x200 + word);
        }
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A block of 0x180 bytes all 0xff.
    fn filled(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x180);
        for i in 0..0x60 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        this
    }

    #[test]
    fn fn_00a8d750_builds_the_tga_reader() {
        let mut e = rig();
        let this = filled(&mut e);
        assert_eq!(e.call(0x00a8_d750, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TGA_READER_VTABLE);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(NI_PIXEL_FORMAT_CONSTRUCT, vec![this + TGA_PIXEL_FORMAT])));
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        // +0x100 to +0x118 cleared except the bytes +0x103 and +0x109, which it leaves.
        for offset in 0x100..0x119 {
            let expected = if offset == 0x103 || offset == 0x109 {
                0xff
            } else {
                0
            };
            assert_eq!(e.mem.u8(this + offset), expected, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u16(this + 0x11a), 0xffff);
        assert_eq!(e.mem.u32(this + TGA_PIXEL_FORMAT), 0x200);
        assert_eq!(e.mem.u32(this + TGA_PIXEL_FORMAT + 0x40), 0x210);
        for offset in 0x160..0x17d {
            assert_eq!(e.mem.u8(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x17d), 0xff);
    }

    #[test]
    fn fn_00a8e410_builds_the_bmp_reader() {
        let mut e = rig();
        let this = filled(&mut e);
        assert_eq!(e.call(0x00a8_e410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BMP_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        assert_eq!(e.mem.u32(this + 0x104), 0);
        assert_eq!(e.mem.u32(this + BMP_PIXEL_FORMAT), 0x200);
        assert_eq!(e.mem.u32(this + BMP_PIXEL_FORMAT + 0x40), 0x210);
        assert_eq!(e.mem.u16(this + 0x14c), 0);
        assert_eq!(e.mem.u16(this + 0x14e), 0xffff);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        assert_eq!(e.mem.u32(this + 0x154), 0);
        assert_eq!(e.mem.u16(this + 0x158), 0);
        assert_eq!(e.mem.u8(this + 0x15a), 0xff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for f in [
            IMPORT_INITIALIZE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            NI_PIXEL_FORMAT_CONSTRUCT,
            DEALLOCATE,
            NI_FREE,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.map(PIXEL_FORMAT_SOURCE_011AA2E8, 0x44);
        e.mem.set_u32(PIXEL_FORMAT_SOURCE_011AA2E8, 0x2e8);
        e
    }

    #[test]
    fn fn_00a8e410_builds_the_bmp_reader() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x150, 9);
        e.mem.set_u16(this + 0x14c, 9);
        assert_eq!(e.call(0x00a8_e410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BMP_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x108), 0x2e8);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        assert_eq!(e.mem.u16(this + 0x14c), 0);
    }

    #[test]
    fn fn_00a8d750_builds_the_tga_reader() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        for at in (0x100..0x180).step_by(4) {
            e.mem.set_u32(this + at, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_d750, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TGA_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x11c), 0x2e8);
        assert_eq!(e.mem.u32(this + 0x100), 0xff00_0000);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x17c) & 0xff, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (NI_PIXEL_FORMAT_CONSTRUCT, vec![this + 0x11c]));
    }

    #[test]
    fn fn_00a8e480_frees_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_e480, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (DEALLOCATE, vec![MEMORY_MANAGER, this]),
            ]
        );
        assert_eq!(e.mem.u32(this), NI_IMAGE_READER_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a8_e480, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00a8d850_frees_the_buffers() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x164, 0x700);
        e.mem.set_u32(this + 0x16c, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x00a8_d850, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (NI_FREE, vec![0x700]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x800]),
                (DELETE_CRITICAL_SECTION, vec![this + 0x80]),
                (DEALLOCATE, vec![MEMORY_MANAGER, this]),
            ]
        );
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;

    const READ: u32 = 0x0300_0000;
    const SEEK: u32 = 0x0300_0010;
    const SKIP: u32 = 0x0300_0014;
    const STREAM_VTABLE: u32 = 0x0300_1000;
    const HEADER_READ: u32 = 0x0300_2000;

    /// A binary stream over `bytes`: the position at +4, the read function at +8 (copies
    /// from the data at +0xc and answers the size; the caller adds it to the position), its
    /// vtable's `+0x10` seeks to the word given and `+0x14` skips the count given.
    fn stream(e: &mut Engine, bytes: &[u8]) -> u32 {
        let data = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(data, bytes);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, STREAM_VTABLE);
        e.mem.set_u32(stream + 8, READ);
        e.mem.set_u32(stream + 0xc, data);
        e.put_vtable(STREAM_VTABLE, &[0, 0, 0, 0, SEEK, SKIP]);
        e.register(READ, |e, a| {
            assert_eq!(a[4], 1);
            let at = e.mem.u32(a[0] + 0xc) + e.mem.u32(a[0] + 4);
            let bytes = e.mem.bytes(at, a[2]);
            e.mem.write(a[1], &bytes);
            a[2].into_ret()
        });
        e.register(SEEK, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(SKIP, |e, a| {
            let position = e.mem.u32(a[0] + 4) + a[1];
            e.mem.set_u32(a[0] + 4, position);
            Ret::default()
        });
        stream
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.map(SEEK_ORIGIN, 4);
        e.mem.set_u32(SEEK_ORIGIN, 1);
        for preset in [
            PIXEL_FORMAT_011AA180,
            PIXEL_FORMAT_011AA1C8,
            PIXEL_FORMAT_011AA210,
            PIXEL_FORMAT_011AA258,
            PIXEL_FORMAT_011AA2A0,
            PIXEL_FORMAT_SOURCE_011AA2E8,
        ] {
            e.mem.set_u32(preset, preset);
        }
        for f in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            NI_PIXEL_FORMAT_CONSTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(ALLOCATE_BYTES, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn reader(e: &mut Engine, width: u16) -> u32 {
        let this = e.mem.alloc(0x180);
        e.mem.set_u16(this + TGA_WIDTH, width);
        this
    }

    /// A reader whose virtual `+0xc` (the header read) answers `answer`.
    fn reader_with_header(e: &mut Engine, answer: u32) -> u32 {
        let this = reader(e, 0);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, HEADER_READ);
        e.mem.set_u32(this, vtable);
        e.register_double(HEADER_READ, move |_, _| answer.into_ret());
        this
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn row_unpackers() {
        let mut e = engine();
        let this = reader(&mut e, 3);
        e.mem.set_u16(this + TGA_FIRST_ENTRY, 0x0102);
        let source = e.mem.alloc(0x10);
        e.mem
            .write(source, &[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d4a0, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 4), vec![3, 4, 5, 0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d460, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 2), vec![3, 0]);
        e.mem.set_u16(this + TGA_WIDTH, 1);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d460, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 1), vec![0]);
        e.mem.set_u16(this + TGA_WIDTH, 2);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d4e0, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 7), vec![5, 5, 5, 6, 6, 6, 0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d670, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 6), vec![7, 6, 5, 10, 9, 8]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d6c0, &args![this, source, out]);
        assert_eq!(e.mem.bytes(out, 8), vec![7, 6, 5, 8, 11, 10, 9, 12]);
        // 16 bits: 0x7fff (lo ff, hi 7f) gives f8 f8 f8; 0x1234 (lo 34, hi 12) 20 88 a0.
        let pixels = e.mem.alloc(4);
        e.mem.write(pixels, &[0xff, 0x7f, 0x34, 0x12]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d520, &args![this, pixels, out]);
        assert_eq!(
            e.mem.bytes(out, 6),
            vec![0xf8, 0xf8, 0xf8, 0x20, 0x88, 0xa0]
        );
        // Palette indices: (hi << 8) - first entry + lo.
        let palette = e.mem.alloc(0x40);
        for i in 0..0x40u32 {
            e.mem.set_u8(palette + i, i as u8);
        }
        e.mem.set_u32(this + TGA_PALETTE, palette);
        e.mem.set_u16(this + TGA_FIRST_ENTRY, 0x0100);
        e.mem.write(pixels, &[2, 1, 3, 1]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d590, &args![this, pixels, out]);
        assert_eq!(e.mem.bytes(out, 7), vec![8, 9, 10, 12, 13, 14, 0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a8_d600, &args![this, pixels, out]);
        assert_eq!(e.mem.bytes(out, 8), vec![8, 9, 10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn extension_tests() {
        let mut e = engine();
        e.register(NI_STRICMP, |_, a| {
            if a[1] == EXTENSION_TARGA {
                0u32.into_ret()
            } else {
                0x1234_5601u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00a8_d710, &args![0u32, 0x55u32]).u32(), 1);
        assert_eq!(
            calls(&e, NI_STRICMP),
            vec![vec![0x55, EXTENSION_TGA], vec![0x55, EXTENSION_TARGA]]
        );
        assert_eq!(
            e.call(0x00a8_e3f0, &args![0u32, 0x55u32]).u32(),
            0x1234_5600
        );
        e.register(NI_STRICMP, |_, _| 0x1234_5601u32.into_ret());
        assert_eq!(
            e.call(0x00a8_d710, &args![0u32, 0x55u32]).u32(),
            0x1234_5600
        );
        e.register(NI_STRICMP, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_e3f0, &args![0u32, 0x55u32]).u32(), 1);
        assert_eq!(e.call(0x00a8_d710, &args![0u32, 0x55u32]).u32(), 1);
    }

    #[test]
    fn plain_read_uses_component_size_one() {
        let mut e = engine();
        let stream = stream(&mut e, &[1, 2, 3, 4]);
        let buffer = e.mem.alloc(4);
        e.call(0x00a8_d8b0, &args![stream, buffer, 3u32]);
        assert_eq!(e.mem.bytes(buffer, 4), vec![1, 2, 3, 0]);
        assert_eq!(e.mem.u32(stream + 4), 3);
    }

    #[test]
    fn run_length_rows_carry_runs_across_rows() {
        let mut e = engine();
        let this = reader(&mut e, 3);
        e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 1);
        // A repeat run of 4 (0x83) of 9, then a raw run of 2 (0x01): 7, 8.
        let stream = stream(&mut e, &[0x83, 9, 0x01, 7, 8]);
        let row = e.mem.alloc(4);
        e.call(0x00a8_d8e0, &args![this, stream, row, 0u32]);
        assert_eq!(e.mem.bytes(row, 3), vec![9, 9, 9]);
        assert_eq!(e.mem.u32(this + TGA_RUN_LEFT), 1);
        assert_eq!(e.mem.u8(this + TGA_RUN_REPEATS), 1);
        let row = e.mem.alloc(4);
        e.call(0x00a8_d8e0, &args![this, stream, row, 0u32]);
        assert_eq!(e.mem.bytes(row, 3), vec![9, 7, 8]);
        assert_eq!(e.mem.u32(this + TGA_RUN_LEFT), 0);
        assert_eq!(e.mem.u8(this + TGA_RUN_REPEATS), 0);
        assert_eq!(e.mem.u32(stream + 4), 5);
        assert_eq!(calls(&e, MEMCPY).len(), 4);
        // No width: nothing is read.
        e.mem.set_u16(this + TGA_WIDTH, 0);
        e.call(0x00a8_d8e0, &args![this, stream, row, 0u32]);
        assert_eq!(e.mem.u32(stream + 4), 5);
    }

    #[test]
    fn colour_maps_grow_and_unpack() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = reader(&mut e, 1);
        e.mem.set_u16(this + TGA_MAP_LENGTH, 2);
        e.mem.set_u8(this + TGA_ENTRY_BITS, 0x18);
        let first = stream(&mut e, &[1, 2, 3, 4, 5, 6]);
        e.call(0x00a8_da50, &args![this, first]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 8]]);
        assert_eq!(e.mem.u32(this + TGA_PALETTE_CAPACITY), 2);
        assert_eq!(e.mem.u32(this + TGA_ROW_CAPACITY), 8);
        let palette = e.mem.u32(this + TGA_PALETTE);
        assert_eq!(e.mem.bytes(palette, 8), vec![3, 2, 1, 0xff, 6, 5, 4, 0xff]);
        // A grey map of the same size reuses both blocks.
        e.call_log = Some(vec![]);
        e.mem.set_u8(this + TGA_ENTRY_BITS, 8);
        let grey = stream(&mut e, &[0x40, 0x80]);
        e.call(0x00a8_da50, &args![this, grey]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(
            e.mem.bytes(palette, 8),
            vec![0x40, 0x40, 0x40, 0xff, 0x80, 0x80, 0x80, 0xff]
        );
        // 32-bit entries swap the first and third bytes and keep the fourth.
        e.mem.set_u8(this + TGA_ENTRY_BITS, 0x20);
        let wide = stream(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.call(0x00a8_da50, &args![this, wide]);
        assert_eq!(e.mem.bytes(palette, 8), vec![3, 2, 1, 4, 7, 6, 5, 8]);
        // Unknown entry sizes read nothing.
        e.mem.set_u8(this + TGA_ENTRY_BITS, 9);
        let empty = stream(&mut e, &[]);
        e.call(0x00a8_da50, &args![this, empty]);
        assert_eq!(e.mem.u32(empty + 4), 0);
    }

    /// An 18-byte TGA header: map type, image type, map length, entry bits, width, height,
    /// depth and descriptor; ID length 0.
    fn tga_header(fields: (u8, u8, u16, u8, u16, u16, u8, u8)) -> Vec<u8> {
        let (map_type, image_type, map_length, entry_bits, width, height, depth, descriptor) =
            fields;
        let mut bytes = vec![0, map_type, image_type, 0, 0];
        bytes.extend(map_length.to_le_bytes());
        bytes.push(entry_bits);
        bytes.extend([0, 0, 0, 0]);
        bytes.extend(width.to_le_bytes());
        bytes.extend(height.to_le_bytes());
        bytes.push(depth);
        bytes.push(descriptor);
        bytes
    }

    #[test]
    fn tga_header_chooses_format_and_unpacker() {
        for (image_type, map_length, entry_bits, depth, descriptor, preset, unpack, bytes) in [
            (
                2u8,
                0u16,
                0u8,
                24u8,
                0x20u8,
                PIXEL_FORMAT_SOURCE_011AA2E8,
                0x00a8_d670u32,
                3u8,
            ),
            (10, 0, 0, 32, 0, PIXEL_FORMAT_011AA2A0, 0x00a8_d6c0, 4),
            (3, 0, 0, 8, 0, PIXEL_FORMAT_SOURCE_011AA2E8, 0x00a8_d4e0, 1),
            (1, 0x10, 24, 8, 0, PIXEL_FORMAT_011AA180, 0x00a8_d4a0, 1),
            (9, 0x100, 32, 16, 0, PIXEL_FORMAT_011AA258, 0x00a8_d600, 2),
            (1, 0x100, 24, 16, 0, PIXEL_FORMAT_011AA210, 0x00a8_d590, 2),
            (1, 0x10, 32, 4, 0, PIXEL_FORMAT_011AA1C8, 0x00a8_d460, 0),
            (2, 0, 0, 16, 0, PIXEL_FORMAT_SOURCE_011AA2E8, 0x00a8_d520, 2),
        ] {
            let mut e = engine();
            let this = reader(&mut e, 0);
            let map_type = u8::from(map_length != 0);
            let header = tga_header((
                map_type, image_type, map_length, entry_bits, 5, 6, depth, descriptor,
            ));
            let stream = stream(&mut e, &header);
            let outs = e.mem.alloc(0x60);
            let words = args![
                this,
                stream,
                outs,
                outs + 4,
                outs + 8,
                outs + 0x4c,
                outs + 0x50
            ];
            assert!(e.call(0x00a8_dce0, &words).bool(), "{image_type} {depth}");
            assert_eq!(
                e.mem.u32(this + TGA_PIXEL_FORMAT),
                preset,
                "{image_type} {depth}"
            );
            assert_eq!(e.mem.u32(this + TGA_UNPACK), unpack);
            assert_eq!(e.mem.u8(this + TGA_BYTES_PER_PIXEL), bytes);
            assert_eq!(e.mem.u8(this + TGA_RUN_LENGTH), u8::from(image_type >= 9));
            assert_eq!(
                e.mem.u8(this + TGA_BOTTOM_UP),
                u8::from(descriptor & 0x20 == 0)
            );
            assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (5, 6));
            assert_eq!(e.mem.u32(outs + 8), preset);
            assert_eq!(e.mem.u32(outs + 0x50), 1);
            assert_eq!(e.mem.u32(stream + 4), 18);
            assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![this + 0x80]]);
        }
    }

    #[test]
    fn tga_header_failures_and_id_skip() {
        let mut e = engine();
        let this = reader(&mut e, 0);
        let outs = e.mem.alloc(0x60);
        let mut header = tga_header((0, 2, 0, 0, 1, 1, 4, 0));
        // 4 bits without a colour map.
        let first = stream(&mut e, &header);
        let words = args![
            this,
            first,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        assert!(!e.call(0x00a8_dce0, &words).bool());
        // An unknown image type.
        header[2] = 5;
        let second = stream(&mut e, &header);
        let words = args![
            this,
            second,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        assert!(!e.call(0x00a8_dce0, &words).bool());
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
        // An image ID is skipped through the stream's virtual +0x14.
        header[0] = 3;
        header[2] = 2;
        header[16] = 24;
        let third = stream(&mut e, &header);
        let words = args![
            this,
            third,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        assert!(e.call(0x00a8_dce0, &words).bool());
        assert_eq!(calls(&e, SKIP), vec![vec![third, 3, 1]]);
    }

    /// A pixel data with one level of `width` by `height` pixels of `stride` bytes.
    fn pixel_data(e: &mut Engine, width: u32, height: u32, stride: u32) -> u32 {
        let data = e.mem.alloc(0x74);
        let widths = e.mem.alloc(4);
        let heights = e.mem.alloc(4);
        let offsets = e.mem.alloc(4);
        e.mem.set_u32(widths, width);
        e.mem.set_u32(heights, height);
        let pixels = e.mem.alloc((width * height * stride).max(1) + 8);
        e.mem.set_u32(data + 0x50, pixels);
        e.mem.set_u32(data + 0x54, widths);
        e.mem.set_u32(data + 0x58, heights);
        e.mem.set_u32(data + 0x5c, offsets);
        e.mem.set_u32(data + 0x64, stride);
        data
    }

    #[test]
    fn tga_file_rows_go_top_down_or_bottom_up() {
        for bottom_up in [false, true] {
            let mut e = engine();
            let this = reader_with_header(&mut e, 1);
            e.mem.set_u16(this + TGA_WIDTH, 2);
            e.mem.set_u16(this + TGA_HEIGHT, 2);
            e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 3);
            e.mem.set_u8(this + TGA_BOTTOM_UP, u8::from(bottom_up));
            e.mem.set_u32(this + TGA_UNPACK, 0x00a8_d670);
            let data = pixel_data(&mut e, 2, 2, 3);
            e.register(NI_NEW, |_, _| 0x0400_0000u32.into_ret());
            e.register_double(NI_PIXEL_DATA_CONSTRUCT, move |_, _| data.into_ret());
            let stream = stream(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
            let answer = e.call(0x00a8_e120, &args![this, stream, 0u32]).u32();
            assert_eq!(answer, data);
            let pixels = e.mem.u32(data + 0x50);
            let expected = if bottom_up {
                vec![9, 8, 7, 12, 11, 10, 3, 2, 1, 6, 5, 4]
            } else {
                vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 12, 11, 10]
            };
            assert_eq!(e.mem.bytes(pixels, 12), expected);
            assert_eq!(
                calls(&e, NI_PIXEL_DATA_CONSTRUCT),
                vec![vec![0x0400_0000, 2, 2, this + TGA_PIXEL_FORMAT, 1, 1]]
            );
            assert_eq!(e.mem.u32(this + TGA_ROW_CAPACITY), 6);
        }
    }

    #[test]
    fn tga_file_failures_and_reuse() {
        let mut e = engine();
        let this = reader_with_header(&mut e, 0x100);
        e.mem.set_u16(this + TGA_WIDTH, 2);
        e.mem.set_u16(this + TGA_HEIGHT, 1);
        let stream = stream(&mut e, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, 0u32]).u32(), 0);
        e.register(HEADER_READ, |_, _| 1u32.into_ret());
        // Zero bytes per pixel at a depth other than 4.
        e.mem.set_u8(this + TGA_PIXEL_DEPTH, 8);
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, 0u32]).u32(), 0);
        // Colour-mapped without a map.
        e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 3);
        e.mem.set_u8(this + TGA_MAPPED, 1);
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, 0u32]).u32(), 0);
        // A map too large.
        e.mem.set_u8(this + TGA_COLOUR_MAP_TYPE, 1);
        e.mem.set_u16(this + TGA_MAP_LENGTH, 0x3fff);
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, 0u32]).u32(), 0);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 4);
        // A matching pixel data is reused.
        e.mem.set_u8(this + TGA_COLOUR_MAP_TYPE, 0);
        e.mem.set_u8(this + TGA_MAPPED, 0);
        e.mem.set_u32(this + TGA_UNPACK, 0x00a8_d670);
        let data = pixel_data(&mut e, 2, 1, 3);
        e.register(PIXEL_FORMAT_EQUALS, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, data]).u32(), data);
        assert_eq!(
            calls(&e, PIXEL_FORMAT_EQUALS),
            vec![vec![data + 8, this + TGA_PIXEL_FORMAT]]
        );
        let pixels = e.mem.u32(data + 0x50);
        assert_eq!(e.mem.bytes(pixels, 6), vec![3, 2, 1, 6, 5, 4]);
    }

    #[test]
    fn tga_file_colour_mapped_gets_a_palette() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = reader_with_header(&mut e, 1);
        e.mem.set_u16(this + TGA_WIDTH, 2);
        e.mem.set_u16(this + TGA_HEIGHT, 1);
        e.mem.set_u8(this + TGA_BYTES_PER_PIXEL, 1);
        e.mem.set_u8(this + TGA_COLOUR_MAP_TYPE, 1);
        e.mem.set_u8(this + TGA_MAPPED, 1);
        e.mem.set_u8(this + TGA_THIRTY_TWO_BIT, 1);
        e.mem.set_u16(this + TGA_MAP_LENGTH, 1);
        e.mem.set_u8(this + TGA_ENTRY_BITS, 8);
        e.mem.set_u32(this + TGA_UNPACK, 0x00a8_d4a0);
        let data = pixel_data(&mut e, 2, 1, 1);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(NI_PIXEL_DATA_CONSTRUCT, move |_, _| data.into_ret());
        e.register(PALETTE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PIXEL_DATA_SET_PALETTE, |_, _| Ret::default());
        let stream = stream(&mut e, &[0x77, 4, 5]);
        assert_eq!(e.call(0x00a8_e120, &args![this, stream, 0u32]).u32(), data);
        let palette = e.mem.u32(this + TGA_PALETTE);
        let made = calls(&e, PALETTE_CONSTRUCT);
        assert_eq!(&made[0][1..], &[1, 1, palette]);
        assert_eq!(
            calls(&e, PIXEL_DATA_SET_PALETTE),
            vec![vec![data, made[0][0]]]
        );
        assert_eq!(e.mem.bytes(e.mem.u32(data + 0x50), 2), vec![4, 5]);
    }

    /// A 54-byte BMP file and info header: info header size, width, height, bits,
    /// compression, colours used and data offset.
    fn bmp_header(fields: (u32, i32, i32, u16, u32, u32, u32)) -> Vec<u8> {
        let (header_size, width, height, bits, compression, colours, offset) = fields;
        let mut bytes = b"BM".to_vec();
        bytes.extend(0u32.to_le_bytes());
        bytes.extend([0, 0, 0, 0]);
        bytes.extend(offset.to_le_bytes());
        bytes.extend(header_size.to_le_bytes());
        bytes.extend(width.to_le_bytes());
        bytes.extend(height.to_le_bytes());
        bytes.extend(1u16.to_le_bytes());
        bytes.extend(bits.to_le_bytes());
        bytes.extend(compression.to_le_bytes());
        bytes.extend([0; 12]);
        bytes.extend(colours.to_le_bytes());
        bytes.extend(0u32.to_le_bytes());
        bytes
    }

    #[test]
    fn bmp_header_reads_the_fields() {
        let mut e = engine();
        let this = reader(&mut e, 0);
        let outs = e.mem.alloc(0x60);
        let first = stream(&mut e, &bmp_header((40, 3, -2, 8, 0, 0, 0x436)));
        let words = args![
            this,
            first,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        assert!(e.call(0x00a8_e4b0, &words).bool());
        assert_eq!(e.mem.u32(first + 4), 54);
        assert_eq!(e.mem.u32(this + BMP_WIDTH), 3);
        assert_eq!(e.mem.u32(this + BMP_HEIGHT), 2);
        assert_eq!(e.mem.u8(this + BMP_BOTTOM_UP), 0);
        assert_eq!(e.mem.u32(this + BMP_COLOURS), 0x100);
        assert_eq!(e.mem.u32(this + BMP_DATA_OFFSET), 0x436);
        assert_eq!(e.mem.u32(this + BMP_PIXEL_FORMAT), PIXEL_FORMAT_011AA210);
        assert_eq!(
            (e.mem.u32(outs), e.mem.u32(outs + 4), e.mem.u32(outs + 8)),
            (3, 2, PIXEL_FORMAT_011AA210)
        );
        assert_eq!(e.mem.u8(outs + 0x4c), 1);
        assert_eq!(e.mem.u32(outs + 0x50), 1);
        // 24 bits keep a zero colour count; a positive height is bottom-up.
        let second = stream(&mut e, &bmp_header((40, 3, 2, 24, 0, 0, 54)));
        let words = args![
            this,
            second,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        e.mem.set_u32(this + BMP_COLOURS, 0);
        assert!(e.call(0x00a8_e4b0, &words).bool());
        assert_eq!(e.mem.u32(this + BMP_COLOURS), 0);
        assert_eq!(e.mem.u8(this + BMP_BOTTOM_UP), 1);
        assert_eq!(
            e.mem.u32(this + BMP_PIXEL_FORMAT),
            PIXEL_FORMAT_SOURCE_011AA2E8
        );
    }

    #[test]
    fn bmp_header_failures() {
        let mut e = engine();
        let this = reader(&mut e, 0);
        let outs = e.mem.alloc(0x60);
        for fields in [
            (4u32, 1i32, 1i32, 24u16, 0u32, 0u32, 54u32),
            (40, 1, 1, 16, 0, 0, 54),
            (40, 1, 1, 8, 1, 0, 54),
            (40, 1, 1, 8, 2, 0, 54),
        ] {
            e.call_log = Some(vec![]);
            let stream = stream(&mut e, &bmp_header(fields));
            let words = args![
                this,
                stream,
                outs,
                outs + 4,
                outs + 8,
                outs + 0x4c,
                outs + 0x50
            ];
            assert!(!e.call(0x00a8_e4b0, &words).bool());
            assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 1);
            assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        }
        // A bad signature answers before the critical section.
        e.call_log = Some(vec![]);
        let mut header = bmp_header((40, 1, 1, 24, 0, 0, 54));
        header[0] = b'X';
        let stream = stream(&mut e, &header);
        let words = args![
            this,
            stream,
            outs,
            outs + 4,
            outs + 8,
            outs + 0x4c,
            outs + 0x50
        ];
        assert!(!e.call(0x00a8_e4b0, &words).bool());
        assert!(calls(&e, ENTER_CRITICAL_SECTION).is_empty());
        assert_eq!(e.mem.u32(stream + 4), 2);
    }

    #[test]
    fn bmp_file_reads_padded_rows_and_swaps_colours() {
        let mut e = engine();
        let this = reader_with_header(&mut e, 1);
        e.mem.set_u32(this + BMP_WIDTH, 1);
        e.mem.set_u32(this + BMP_HEIGHT, 2);
        e.mem.set_u16(this + BMP_BIT_COUNT, 24);
        e.mem.set_u32(this + BMP_DATA_OFFSET, 54);
        e.mem.set_u8(this + BMP_BOTTOM_UP, 1);
        let data = pixel_data(&mut e, 1, 2, 3);
        e.register(NI_NEW, |_, _| 0x0400_0000u32.into_ret());
        e.register_double(NI_PIXEL_DATA_CONSTRUCT, move |_, _| data.into_ret());
        e.register(PIXEL_FORMAT_EQUALS, |_, a| {
            (a[1] == PIXEL_FORMAT_SOURCE_011AA2E8).into_ret()
        });
        let stream = stream(&mut e, &[1, 2, 3, 0xee, 4, 5, 6, 0xee]);
        assert_eq!(e.call(0x00a8_e810, &args![this, stream, 0u32]).u32(), data);
        // No palette bytes to skip; the rows fill from the bottom; the colours swap.
        assert_eq!(calls(&e, SKIP), vec![vec![stream, 0, 1]]);
        let pixels = e.mem.u32(data + 0x50);
        assert_eq!(e.mem.bytes(pixels, 6), vec![6, 5, 4, 3, 2, 1]);
        assert_eq!(e.mem.u32(stream + 4), 8);
    }

    #[test]
    fn bmp_file_reads_unpadded_images_at_once() {
        let mut e = engine();
        let this = reader_with_header(&mut e, 1);
        e.mem.set_u32(this + BMP_WIDTH, 4);
        e.mem.set_u32(this + BMP_HEIGHT, 2);
        e.mem.set_u16(this + BMP_BIT_COUNT, 8);
        e.mem.set_u32(this + BMP_DATA_OFFSET, 54);
        let data = pixel_data(&mut e, 4, 2, 1);
        e.register(PIXEL_FORMAT_EQUALS, |_, a| (a[0] == a[1]).into_ret());
        e.register(NI_NEW, |_, _| 0x0400_0000u32.into_ret());
        e.register_double(NI_PIXEL_DATA_CONSTRUCT, move |_, _| data.into_ret());
        let stream = stream(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(e.call(0x00a8_e810, &args![this, stream, 0u32]).u32(), data);
        let reads: Vec<_> = calls(&e, READ).iter().map(|w| w[2]).collect();
        assert_eq!(reads, vec![8]);
        let pixels = e.mem.u32(data + 0x50);
        assert_eq!(e.mem.bytes(pixels, 8), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn bmp_file_four_bit_and_palette() {
        let mut e = engine();
        let this = reader_with_header(&mut e, 1);
        e.mem.set_u32(this + BMP_WIDTH, 3);
        e.mem.set_u32(this + BMP_HEIGHT, 1);
        e.mem.set_u16(this + BMP_BIT_COUNT, 4);
        e.mem.set_u32(this + BMP_COLOURS, 2);
        e.mem.set_u32(this + BMP_DATA_OFFSET, 54 + 8 + 2);
        let data = pixel_data(&mut e, 3, 1, 1);
        // The old palette is released (count 1) and the new one counted.
        let old = e.mem.alloc(8);
        let old_vtable = e.mem.alloc(8);
        e.mem.set_u32(old, old_vtable);
        e.mem.set_u32(old_vtable + 4, 0x0300_3004);
        e.register(0x0300_3004, |_, _| Ret::default());
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(data + 0x4c, old);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PALETTE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |_, _| 1u32.into_ret());
        let format = this + BMP_PIXEL_FORMAT;
        e.register_double(PIXEL_FORMAT_EQUALS, move |_, a| {
            (a[1] == PIXEL_FORMAT_011AA210 || a[0] != format).into_ret()
        });
        let stream = stream(
            &mut e,
            &[1, 2, 3, 0, 4, 5, 6, 0, 0x11, 0x22, 0xab, 0xc0, 0, 0],
        );
        let answer = e.call(0x00a8_e810, &args![this, stream, data]).u32();
        assert_eq!(answer, data);
        let palette = calls(&e, PALETTE_CONSTRUCT);
        assert_eq!(&palette[0][1..3], &[0, 0x100]);
        let entries = palette[0][3];
        assert_eq!(e.mem.bytes(entries, 8), vec![3, 2, 1, 0xff, 6, 5, 4, 0xff]);
        assert_eq!(e.mem.u32(data + 0x4c), palette[0][0]);
        assert_eq!(calls(&e, 0x0300_3004), vec![vec![old]]);
        assert_eq!(calls(&e, SKIP), vec![vec![stream, 2, 1]]);
        // (3 + 1) / 2 = 2 bytes, padded to 4: 0xab 0xc0 give a b c 0.
        let pixels = e.mem.u32(data + 0x50);
        assert_eq!(e.mem.bytes(pixels, 4), vec![0xa, 0xb, 0xc, 0]);
    }
}

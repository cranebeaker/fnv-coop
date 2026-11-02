//! `fallout/misc/saveload/bgssaveloadheader.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The header of a save file: a magic string, a header size (`0x30`), a
//! 0x40-byte string, the
//! screenshot's width and height, the player's name and title, the level,
//! the location name, the play time, and the screenshot's pixels
//! (`SaveHeader`); and the readers of it (`LoadHeader`, the fill of a list
//! entry `0084dab0`, `LoadHeaderScreenshot`).
//!
//! The frames are the game's stack locals: the writer is a
//! `BGSSaveGameBuffer` (0x18 bytes, `00865bd0`), the reader a
//! `BGSLoadGameBuffer` (0x10 bytes, `bgsloadgamebuffer.rs`). Every function
//! with an exception frame or a `/GS` cookie check is marked as incomplete:
//! the unwinding and the cookie check are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::bgsloadgamebuffer::BGSLoadGameBuffer;
use crate::units::fallout_misc::bgssaveloadfile::BGSSaveLoadFile;

/// The global pointer to the magic string every save file starts with.
const HEADER_MAGIC: u32 = 0x011a_2224;
/// The global pointer to the save/load manager object (`011de134`) the
/// header functions call methods of (`0066d730`, `00424940`, `0044ddc0`,
/// `00851b40`, `00851bf0`).
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// The global pointer to the player character.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `_strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `00408b20`: compares two strings (cdecl: both strings; 0 when equal).
const STRCMP: u32 = 0x0040_8b20;
/// `00403d30` and `00ec61c0`: fill a buffer with a byte (cdecl: buffer,
/// byte, count).
const MEMSET_SMALL: u32 = 0x0040_3d30;
const MEMSET: u32 = 0x00ec_61c0;
/// `00406d30`: copies a string into a sized buffer (cdecl: destination,
/// size, source). `00406d50`: appends (same arguments).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
/// `00403df0`: the string value of a setting (`ECX`): the word at `+4`.
const SETTING_STRING: u32 = 0x0040_3df0;
/// `00408d60`: the address of a setting's value (`ECX` is the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_8d60;
/// `_ftol2_sse` (`00ec62c0`): truncates a `double` (two words here) to an
/// integer.
const FTOL: u32 = 0x00ec_62c0;
/// `BGSSaveGameBuffer::BGSSaveGameBuffer` (Xbox PDB), 0x18 bytes.
const WRITER_CONSTRUCT: u32 = 0x0086_5bd0;
/// `BGSSaveGameBuffer::~BGSSaveGameBuffer` (Xbox PDB).
const WRITER_DESTRUCT: u32 = 0x0086_5c10;
/// `BGSSaveGameBuffer::Save` (Xbox PDB): `(writer, file)`.
const WRITER_SAVE: u32 = 0x0086_5c40;
/// The writer's data and string savers: `(writer, pointer, size, flag)`
/// (`00865e50`) and `BGSSaveGameBuffer::SaveString` (Xbox PDB):
/// `(writer, text, flag)`.
const WRITER_SAVE_DATA: u32 = 0x0086_5e50;
const WRITER_SAVE_STRING: u32 = 0x0086_5e70;
/// `BGSSaveLoadFile::Write`/`Read`/`GetFilePosition`/`SeekSet`/`SeekCurrent`.
const FILE_WRITE: u32 = 0x0084_6330;
const FILE_READ: u32 = 0x0084_6300;
const FILE_POSITION: u32 = 0x0084_6400;
const FILE_SEEK_SET: u32 = 0x0084_6440;
const FILE_SEEK_CURRENT: u32 = 0x0084_6490;
/// `BGSLoadGameBuffer::BGSLoadGameBuffer` / `Load` (attach a file) / the
/// raw reader `(reader, destination, size)` / `LoadString(reader, flag)`
/// (returns the text) / the destructor.
const READER_CONSTRUCT: u32 = 0x0086_46b0;
const READER_ATTACH: u32 = 0x0086_4740;
const READER_LOAD_RAW: u32 = 0x0086_4980;
const READER_LOAD_STRING: u32 = 0x0086_49a0;
const READER_DESTRUCT: u32 = 0x0086_46f0;
/// The header size `SaveHeader` writes (`0066d730` returns it).
const HEADER_SIZE_GETTER: u32 = 0x0066_d730;
/// The header size `SaveHeader` writes; the readers treat it specially.
const FULL_HEADER_SIZE: u32 = 0x30;
/// `00424940` (byte at `+0xc` of the manager): set when no real screenshot
/// is taken.
const NO_SCREENSHOT_FLAG: u32 = 0x0042_4940;
/// `0044ddc0` (on the manager): a word that is saved.
const MANAGER_WORD: u32 = 0x0044_ddc0;
/// `004dc200` / `004dc1f0`: two integers whose ratio decides the size of
/// the screenshot.
const SCREEN_NUMERATOR: u32 = 0x004d_c200;
const SCREEN_DENOMINATOR: u32 = 0x004d_c1f0;
/// The `double` the ratio is compared with (`0101de30`).
const RATIO_LIMIT: u32 = 0x0101_de30;
/// `00851b40` (`BGSSaveLoadManager::GetPlayerLocationName`, Xbox PDB) and
/// `00851bf0` (`GetPlayTimeString`): fill a text buffer.
const GET_LOCATION_NAME: u32 = 0x0085_1b40;
const GET_PLAY_TIME: u32 = 0x0085_1bf0;
/// `0055d520` (the player's name) and `0047e0e0` (`TESActorBaseData::
/// GetKarmicTitle`, Xbox PDB, cdecl with the player), and `0087f9f0` (a
/// 16-bit value of the actor, saved as a word).
const PLAYER_NAME: u32 = 0x0055_d520;
const PLAYER_TITLE: u32 = 0x0047_e0e0;
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// The settings whose strings start the 0x40-byte block and the path of
/// the screenshot, and the setting object tested for a path.
const GAME_NAME_SETTING: u32 = 0x011c_3cb4;
const SCREENSHOT_DIRECTORY_SETTING: u32 = 0x011c_3f74;
const SCREENSHOT_ENABLED_SETTING: u32 = 0x011d_e2fc;
/// The string appended after the file name in the screenshot path.
const SCREENSHOT_EXTENSION: u32 = 0x0107_f794;
/// `006815c0` (on the file): the file's name.
const FILE_NAME: u32 = 0x0068_15c0;
/// `00878f60` (`TakeSaveGameScreenShot`, in `screenshot.rs`): `(width
/// pointer, height pointer, path)`; returns the pixel data object.
const TAKE_SCREENSHOT: u32 = 0x0087_8f60;
/// `0064dac0`: the address of the pixels of a pixel data object
/// (`(object, 0, 0)`).
const PIXEL_DATA_POINTER: u32 = 0x0064_dac0;
/// A string constant (`01018348`) the entry gets in place of the 0x40-byte
/// block when the text at the reader's position starts with a bar.
const DEFAULT_NAME: u32 = 0x0101_8348;
/// `operator new` (`00aa13e0`, cdecl: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;

/// The size of a screen-size screenshot is 0x200 by 0x200 when the ratio is
/// below the limit, else 0x1b0 by 0x1b0.
const LARGE_SIZE: u32 = 0x200;
const SMALL_SIZE: u32 = 0x1b0;

// Offsets in the frame `SaveHeader` keeps its locals in.
const FRAME_WRITER: u32 = 0x000;
const FRAME_VERSION_WORD: u32 = 0x020;
const FRAME_WIDTH: u32 = 0x024;
const FRAME_HEIGHT: u32 = 0x028;
const FRAME_MANAGER_WORD: u32 = 0x02c;
const FRAME_LEVEL_WORD: u32 = 0x030;
const FRAME_NAME_BLOCK: u32 = 0x040;
const FRAME_PATH: u32 = 0x080;
const FRAME_LOCATION: u32 = 0x200;
const FRAME_PLAY_TIME: u32 = 0x320;
const FRAME_SIZE: u32 = 0x440;

/// The text capacity of the path, location and play-time buffers.
const TEXT_SIZE: u32 = 0x104;

// Translated from 0084d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHeader::SaveHeader` (Xbox PDB): writes the header of the save
/// file `file`.
///
/// It writes the magic string to the file, then through a
/// `BGSSaveGameBuffer` the header size (`0066d730`, 0x30), the 0x40-byte
/// string (`GAME_NAME_SETTING`'s text), the screenshot's width and height
/// (0x200 square when `004dc200 / 004dc1f0` is below the `double` at
/// `0101de30`, else 0x1b0), the manager's word, the player's name and
/// title, the player's 16-bit value, the location name and the play time;
/// the buffer is then saved to the file, followed by the screenshot's
/// pixels (`width * height * 3` bytes; a null buffer when the manager's
/// byte `00424940` is set, in which case no screenshot is taken and the
/// height is scaled by the ratio).
// Translation incomplete: 0084d4b0 - the compiler's exception-unwind frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
pub fn bgssaveloadheader_save_header(e: &mut Engine, file: Ptr<BGSSaveLoadFile>) {
    let magic = e.global::<u32>(HEADER_MAGIC);
    let length = e.call(STRLEN, &args![magic]).u32();
    e.call(FILE_WRITE, &args![file, magic, length]);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let base = frame.addr();
        let writer = base + FRAME_WRITER;
        e.call(WRITER_CONSTRUCT, &args![writer]);
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        let save_data = |e: &mut Engine, address: u32, size: u32| {
            e.call(WRITER_SAVE_DATA, &args![writer, address, size, 0u32]);
        };

        let version = e.call(HEADER_SIZE_GETTER, &args![manager]).u32();
        e.mem.set_u32(base + FRAME_VERSION_WORD, version);
        save_data(e, base + FRAME_VERSION_WORD, 4);

        let name_block = base + FRAME_NAME_BLOCK;
        e.call(MEMSET_SMALL, &args![name_block, 0u32, 0x40u32]);
        let game_name = e.call(SETTING_STRING, &args![GAME_NAME_SETTING]).u32();
        e.call(STRING_APPEND, &args![name_block, 0x40u32, game_name]);
        save_data(e, name_block, 0x40);

        let numerator = e.call(SCREEN_NUMERATOR, &[]).i32();
        let denominator = e.call(SCREEN_DENOMINATOR, &[]).i32();
        let ratio = numerator as f64 / denominator as f64;
        let limit: f64 = e.global(RATIO_LIMIT);
        let size = if ratio < limit {
            LARGE_SIZE
        } else {
            SMALL_SIZE
        };
        e.mem.set_u32(base + FRAME_WIDTH, size);
        e.mem.set_u32(base + FRAME_HEIGHT, size);

        let path = base + FRAME_PATH;
        e.mem.set_u8(path, 0);
        e.call(MEMSET, &args![path + 1, 0u32, 0x103u32]);
        let enabled = e
            .call(SETTING_VALUE_ADDRESS, &args![SCREENSHOT_ENABLED_SETTING])
            .u32();
        if e.mem.u8(enabled) != 0 {
            let directory = e
                .call(SETTING_STRING, &args![SCREENSHOT_DIRECTORY_SETTING])
                .u32();
            e.call(STRING_COPY, &args![path, TEXT_SIZE, directory]);
            let name = e.call(FILE_NAME, &args![file]).u32();
            e.call(STRING_APPEND, &args![path, TEXT_SIZE, name]);
            e.call(STRING_APPEND, &args![path, TEXT_SIZE, SCREENSHOT_EXTENSION]);
        }

        let mut pixel_data = 0u32;
        if e.call(NO_SCREENSHOT_FLAG, &args![manager]).u8() == 0 {
            pixel_data = e
                .call(
                    TAKE_SCREENSHOT,
                    &args![base + FRAME_WIDTH, base + FRAME_HEIGHT, path],
                )
                .u32();
        } else {
            let numerator = e.call(SCREEN_NUMERATOR, &[]).i32();
            let denominator = e.call(SCREEN_DENOMINATOR, &[]).i32();
            let ratio = (numerator as f64 / denominator as f64) as f32;
            let height = e.mem.i32(base + FRAME_HEIGHT);
            let scaled = e.call(FTOL, &args![height as f64 * ratio as f64]).u32();
            e.mem.set_u32(base + FRAME_HEIGHT, scaled);
        }
        save_data(e, base + FRAME_WIDTH, 4);
        save_data(e, base + FRAME_HEIGHT, 4);

        let manager_word = e.call(MANAGER_WORD, &args![manager]).u32();
        e.mem.set_u32(base + FRAME_MANAGER_WORD, manager_word);
        save_data(e, base + FRAME_MANAGER_WORD, 4);

        let player = e.global::<u32>(PLAYER_POINTER);
        let player_name = e.call(PLAYER_NAME, &args![player]).u32();
        e.call(WRITER_SAVE_STRING, &args![writer, player_name, 0u32]);
        let title = e.call(PLAYER_TITLE, &args![player]).u32();
        e.call(WRITER_SAVE_STRING, &args![writer, title, 0u32]);
        let level = e.call(PLAYER_LEVEL, &args![player]).u16();
        e.mem.set_u32(base + FRAME_LEVEL_WORD, u32::from(level));
        save_data(e, base + FRAME_LEVEL_WORD, 4);

        let location = base + FRAME_LOCATION;
        e.call(GET_LOCATION_NAME, &args![manager, location]);
        e.call(WRITER_SAVE_STRING, &args![writer, location, 0u32]);
        let play_time = base + FRAME_PLAY_TIME;
        e.call(GET_PLAY_TIME, &args![manager, play_time, 1u32]);
        e.call(WRITER_SAVE_STRING, &args![writer, play_time, 0u32]);

        e.call(WRITER_SAVE, &args![writer, file]);

        let width = e.mem.u32(base + FRAME_WIDTH);
        let height = e.mem.u32(base + FRAME_HEIGHT);
        let byte_count = width.wrapping_mul(height).wrapping_mul(3);
        if e.call(NO_SCREENSHOT_FLAG, &args![manager]).u8() == 0 {
            let pixels = e
                .call(PIXEL_DATA_POINTER, &args![pixel_data, 0u32, 0u32])
                .u32();
            e.call(FILE_WRITE, &args![file, pixels, byte_count]);
        } else {
            e.call(FILE_WRITE, &args![file, 0u32, byte_count]);
        }
        if pixel_data != 0 {
            // The pixel data object's deleting destructor (vtable slot 0).
            e.vcall(pixel_data, 0, &args![1u32]);
        }
        e.call(WRITER_DESTRUCT, &args![writer]);
    });
}

// Translated from 0084d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHeader::LoadHeader` (Xbox PDB): reads the header of the save
/// file `file`. Returns 0 when the file gives no bytes for the magic string,
/// -1 when the magic string differs, else the header size read (then
/// `file` has been moved past the screenshot's pixels).
///
/// A header of size 0x30 has its 0x40-byte block read only when the text
/// right after the reader's current position (+4) is not `'|'`; any other
/// size always reads the block. The width and height are read, the pixels
/// skipped (`width * height * 3` bytes, `SeekCurrent`).
// Translation incomplete: 0084d8c0 - the compiler's exception-unwind frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
pub fn bgssaveloadheader_load_header(e: &mut Engine, file: Ptr<BGSSaveLoadFile>) -> u32 {
    let magic = e.global::<u32>(HEADER_MAGIC);
    e.with_stack(0x110, |e, buffer| {
        e.mem.set_u8(buffer.addr(), 0);
        e.call(MEMSET, &args![buffer.addr() + 1, 0u32, 0x103u32]);
        let length = e.call(STRLEN, &args![magic]).u32();
        let read = e.call(FILE_READ, &args![file, buffer, length]).u32();
        if read == 0 {
            return 0;
        }
        if e.call(STRCMP, &args![buffer, magic]).u32() != 0 {
            return u32::MAX;
        }
        e.with_stack(0x10, |e, reader| {
            e.call(READER_CONSTRUCT, &args![reader]);
            e.call(READER_ATTACH, &args![reader, file]);
            e.with_stack(0x80, |e, scratch| {
                let header_size = scratch.addr();
                let name_block = scratch.addr() + 0x10;
                let width = scratch.addr() + 0x8;
                let height = scratch.addr() + 0xc;
                e.call(READER_LOAD_RAW, &args![reader, header_size, 4u32]);
                if e.mem.u32(header_size) == FULL_HEADER_SIZE {
                    let current = fn_0084da90(e, reader.cast::<BGSLoadGameBuffer>());
                    if e.mem.i8(current.wrapping_add(4)) != 0x7c {
                        e.call(READER_LOAD_RAW, &args![reader, name_block, 0x40u32]);
                    }
                } else {
                    e.call(READER_LOAD_RAW, &args![reader, name_block, 0x40u32]);
                }
                e.call(READER_LOAD_RAW, &args![reader, width, 4u32]);
                e.call(READER_LOAD_RAW, &args![reader, height, 4u32]);
                let pixel_bytes = e
                    .mem
                    .u32(width)
                    .wrapping_mul(e.mem.u32(height))
                    .wrapping_mul(3);
                e.call(FILE_SEEK_CURRENT, &args![file, pixel_bytes]);
                let size = e.mem.u32(header_size);
                e.call(READER_DESTRUCT, &args![reader]);
                size
            })
        })
    })
}

// Translated from 0084da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reader's current position as an address: the buffer pointer
/// (`+4`) plus the position (`+0xc`).
pub fn fn_0084da90(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>) -> u32 {
    let buffer = e.get(this, BGSLoadGameBuffer::pBuffer);
    let position = e.get(this, BGSLoadGameBuffer::iBufferPosition);
    buffer.addr().wrapping_add(position)
}

// Translated from 0084dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the list entry `entry` from the header of the save file `file`
/// (cdecl; `bgssaveloadmanager.rs` calls it for each save in the list): the
/// same reads as `LoadHeader` (the file starts at its beginning, the magic
/// string is checked and nothing happens if it differs), each field handed
/// to one of the entry's setters (`0040e500` the header size, `0084dd70` the
/// 0x40-byte block's text, `00403550`, `0041fd00`, `00437730`, and, for a
/// header size above 0x2f as a signed number, `0050f9a0`, then `00600ab0`,
/// `00984f60`, `0050f9c0` the width, `007037c0` the height, `0061cc40` the
/// position of the file after the header). The names of those setters are
/// not known.
// Translation incomplete: 0084dab0 - the compiler's exception-unwind frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_0084dab0(e: &mut Engine, file: Ptr<BGSSaveLoadFile>, entry: Ptr) {
    e.call(FILE_SEEK_SET, &args![file, 0u32]);
    let magic = e.global::<u32>(HEADER_MAGIC);
    e.with_stack(0x110, |e, buffer| {
        e.mem.set_u8(buffer.addr(), 0);
        let length = e.call(STRLEN, &args![magic]).u32();
        e.call(FILE_READ, &args![file, buffer, length]);
        if e.call(STRCMP, &args![buffer, magic]).u32() != 0 {
            return;
        }
        e.with_stack(0x10, |e, reader| {
            e.call(READER_CONSTRUCT, &args![reader]);
            e.call(READER_ATTACH, &args![reader, file]);
            e.with_stack(0x80, |e, scratch| {
                let header_size = scratch.addr();
                let width = scratch.addr() + 0x4;
                let height = scratch.addr() + 0x8;
                let third_word = scratch.addr() + 0xc;
                let fourth_word = scratch.addr() + 0x10;
                let name_block = scratch.addr() + 0x20;
                e.call(READER_LOAD_RAW, &args![reader, header_size, 4u32]);
                let size = e.mem.u32(header_size);
                e.call(0x0040_e500, &args![entry, size]);
                if size == FULL_HEADER_SIZE {
                    let current = fn_0084da90(e, reader.cast::<BGSLoadGameBuffer>());
                    if e.mem.i8(current.wrapping_add(4)) == 0x7c {
                        entry_set_name_block(e, entry, DEFAULT_NAME);
                    } else {
                        e.call(READER_LOAD_RAW, &args![reader, name_block, 0x40u32]);
                        entry_set_name_block(e, entry, name_block);
                    }
                } else {
                    e.call(READER_LOAD_RAW, &args![reader, name_block, 0x40u32]);
                    entry_set_name_block(e, entry, name_block);
                }
                e.call(READER_LOAD_RAW, &args![reader, width, 4u32]);
                e.call(READER_LOAD_RAW, &args![reader, height, 4u32]);
                e.mem.set_u32(third_word, 0);
                e.call(READER_LOAD_RAW, &args![reader, third_word, 4u32]);
                let third = e.mem.u32(third_word);
                e.call(0x0040_3550, &args![entry, third]);
                let text = e.call(READER_LOAD_STRING, &args![reader, 0u32]).u32();
                e.call(0x0041_fd00, &args![entry, text]);
                let text = e.call(READER_LOAD_STRING, &args![reader, 0u32]).u32();
                e.call(0x0043_7730, &args![entry, text]);
                // 0084dca8: CMP [size],0x2f / JLE, a signed test of the word
                // as it is in memory.
                if e.mem.i32(header_size) > 0x2f {
                    e.mem.set_u32(fourth_word, 0);
                    e.call(READER_LOAD_RAW, &args![reader, fourth_word, 4u32]);
                    let fourth = e.mem.u32(fourth_word);
                    e.call(0x0050_f9a0, &args![entry, fourth]);
                }
                let text = e.call(READER_LOAD_STRING, &args![reader, 0u32]).u32();
                e.call(0x0060_0ab0, &args![entry, text]);
                let text = e.call(READER_LOAD_STRING, &args![reader, 0u32]).u32();
                e.call(0x0098_4f60, &args![entry, text]);
                let width_value = e.mem.u32(width);
                e.call(0x0050_f9c0, &args![entry, width_value]);
                let height_value = e.mem.u32(height);
                e.call(0x0070_37c0, &args![entry, height_value]);
                let position = e.call(FILE_POSITION, &args![file]).u32();
                e.call(0x0061_cc40, &args![entry, position]);
                e.call(READER_DESTRUCT, &args![reader]);
            });
        });
    });
}

// Translated from 0084dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the text `source` into the entry's 0x40-byte block at `+0x3c`
/// (`00406d30`: destination, size, source).
pub fn entry_set_name_block(e: &mut Engine, this: Ptr, source: u32) {
    e.call(
        STRING_COPY,
        &args![this.addr().wrapping_add(0x3c), 0x40u32, source],
    );
}

/// The 0x44 bytes `LoadHeaderScreenshot` copies for the image format
/// description (`011aa2e8`).
const IMAGE_FORMAT: u32 = 0x011a_a2e8;
/// The size of that description in bytes.
const IMAGE_FORMAT_SIZE: u32 = 0x44;
/// The size of the image object `LoadHeaderScreenshot` allocates.
const IMAGE_OBJECT_SIZE: u32 = 0x74;
/// `00a7c190`: constructs the image object in place (thiscall; five stack
/// words: width, height, the format description, 1, 1).
const IMAGE_CONSTRUCT: u32 = 0x00a7_c190;
/// `004f31d0`: constructs a three-word record (thiscall).
const RECORD_CONSTRUCT: u32 = 0x004f_31d0;
/// `00438170`: constructs a fixed string from a text (thiscall, one
/// stack word); `0043b1b0`: its text; `004381b0`: its destructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// The text handed to the fixed string (`0107f79c`).
const IMAGE_NAME: u32 = 0x0107_f79c;
/// `00a5ff10`: cdecl `(image object, name text, record)`; returns the
/// result of `LoadHeaderScreenshot`.
const IMAGE_FINISH: u32 = 0x00a5_ff10;

// Translated from 0084dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHeader::LoadHeaderScreenshot` (Xbox PDB), cdecl with four
/// stack words: moves `file` to `position`, builds an image object of
/// `width` by `height` (`00a7c190`), reads `width * height * 3` bytes of
/// pixels into it and finishes it (`00a5ff10`) with a record `{2, 0, 0}`
/// and the fixed string made from the text at `0107f79c`; returns what that
/// returns.
// Translation incomplete: 0084dda0 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn load_header_screenshot(
    e: &mut Engine,
    file: Ptr<BGSSaveLoadFile>,
    position: u32,
    width: u32,
    height: u32,
) -> u32 {
    e.call(FILE_SEEK_SET, &args![file, position]);
    e.with_stack(0x80, |e, frame| {
        let format = frame.addr();
        let record = frame.addr() + 0x50;
        let name = frame.addr() + 0x60;
        let description = e.mem.bytes(IMAGE_FORMAT, IMAGE_FORMAT_SIZE);
        e.mem.write(format, &description);
        let block = e.call(OPERATOR_NEW, &args![IMAGE_OBJECT_SIZE]).u32();
        let image = if block != 0 {
            e.call(
                IMAGE_CONSTRUCT,
                &args![block, width, height, format, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let pixels = e.call(PIXEL_DATA_POINTER, &args![image, 0u32, 0u32]).u32();
        let byte_count = width.wrapping_mul(height).wrapping_mul(3);
        e.call(FILE_READ, &args![file, pixels, byte_count]);
        e.call(RECORD_CONSTRUCT, &args![record]);
        e.mem.set_u32(record, 2);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, 0);
        e.call(FIXED_STRING_CONSTRUCT, &args![name, IMAGE_NAME]);
        let text = e.call(FIXED_STRING_TEXT, &args![name]).u32();
        let result = e.call(IMAGE_FINISH, &args![image, text, record]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0084d4b0,
            bgssaveloadheader_save_header(Ptr<BGSSaveLoadFile>)
        ),
        entry!(
            0x0084d8c0,
            bgssaveloadheader_load_header(Ptr<BGSSaveLoadFile>) -> u32
        ),
        entry!(0x0084da90, fn_0084da90(Ptr<BGSLoadGameBuffer>) -> u32),
        entry!(0x0084dab0, fn_0084dab0(Ptr<BGSSaveLoadFile>, Ptr)),
        entry!(0x0084dd70, entry_set_name_block(Ptr, u32)),
        entry!(
            0x0084dda0,
            load_header_screenshot(Ptr<BGSSaveLoadFile>, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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

    #[test]
    fn current_position_is_the_buffer_plus_the_position() {
        let mut e = Engine::new();
        let reader = e.new_object::<BGSLoadGameBuffer>();
        e.set(reader, BGSLoadGameBuffer::pBuffer, Ptr::new(0x1000));
        e.set(reader, BGSLoadGameBuffer::iBufferPosition, 0x24);
        assert_eq!(e.call(0x0084_da90, &[reader.addr()]).u32(), 0x1024);
    }

    #[test]
    fn name_block_is_copied_with_its_size() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        recording(&mut e, &log, STRING_COPY, 0);
        e.call(0x0084_dd70, &[0x2000, 0x3000]);
        assert_eq!(
            *log.borrow(),
            vec![(STRING_COPY, vec![0x203c, 0x40, 0x3000])]
        );
    }

    /// Doubles for everything the loaders call on the reader; the raw
    /// reads give the words and blocks of `words` in order.
    fn reader_engine(header_size: u32, text_at_current: u8) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(HEADER_MAGIC, 4);
        let magic = e.mem.alloc(16);
        e.mem.set_cstr(magic, b"MAGIC");
        e.set_global(HEADER_MAGIC, magic);
        let log: Log = Rc::default();
        e.register(STRLEN, |_, _| word_ret(5));
        e.register(MEMSET, |_, _| Ret::default());
        recording(&mut e, &log, FILE_READ, 5);
        e.register(STRCMP, |_, _| word_ret(0));
        recording(&mut e, &log, READER_CONSTRUCT, 0);
        recording(&mut e, &log, READER_ATTACH, 0);
        recording(&mut e, &log, READER_DESTRUCT, 0);
        recording(&mut e, &log, FILE_SEEK_CURRENT, 0);
        recording(&mut e, &log, FILE_SEEK_SET, 0);
        // The reader's buffer holds one byte at +4 from its current position.
        let data = e.mem.alloc(16);
        e.mem.set_u8(data + 4, text_at_current);
        let values = Rc::new(RefCell::new(vec![header_size, 3, 4].into_iter()));
        let sink = log.clone();
        e.register_double(READER_LOAD_RAW, move |e, a| {
            sink.borrow_mut().push((READER_LOAD_RAW, a.to_vec()));
            if a[2] == 4 {
                // Header size, then width and height.
                if let Some(value) = values.borrow_mut().next() {
                    e.mem.set_u32(a[1], value);
                }
            }
            // Point the reader at the byte under test.
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u32(a[0] + 0xc, 0);
            Ret::default()
        });
        (e, log, magic)
    }

    #[test]
    fn load_header_reads_the_block_unless_it_starts_with_a_bar() {
        // Size 0x30 with the text '|': no 0x40 block; width 3, height 4 -> skip 36.
        let (mut e, log, _) = reader_engine(0x30, b'|');
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_d8c0, &[file]).u32(), 0x30);
        let calls = log.borrow();
        assert!(!calls
            .iter()
            .any(|c| c.0 == READER_LOAD_RAW && c.1[2] == 0x40));
        assert!(calls.contains(&(FILE_SEEK_CURRENT, vec![file, 36])));
        assert_eq!(calls.last().unwrap().0, READER_DESTRUCT);
        drop(calls);
        // Size 0x30 with another character: the block is read.
        let (mut e, log, _) = reader_engine(0x30, b'x');
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_d8c0, &[file]).u32(), 0x30);
        assert_eq!(
            log.borrow()
                .iter()
                .filter(|c| c.0 == READER_LOAD_RAW && c.1[2] == 0x40)
                .count(),
            1
        );
        // Another size: the block is read whatever the byte.
        let (mut e, log, _) = reader_engine(0x20, b'|');
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_d8c0, &[file]).u32(), 0x20);
        assert_eq!(
            log.borrow()
                .iter()
                .filter(|c| c.0 == READER_LOAD_RAW && c.1[2] == 0x40)
                .count(),
            1
        );
    }

    #[test]
    fn load_header_returns_zero_without_bytes_and_minus_one_for_another_magic() {
        let (mut e, log, _) = reader_engine(0x30, b'|');
        recording(&mut e, &log, FILE_READ, 0);
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_d8c0, &[file]).u32(), 0);
        assert!(!log.borrow().iter().any(|c| c.0 == READER_CONSTRUCT));
        let (mut e, log, _) = reader_engine(0x30, b'|');
        e.register(STRCMP, |_, _| word_ret(1));
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_d8c0, &[file]).u32(), u32::MAX);
        assert!(!log.borrow().iter().any(|c| c.0 == READER_CONSTRUCT));
    }

    #[test]
    fn entry_fill_hands_every_field_to_the_entry() {
        let (mut e, log, _) = reader_engine(0x30, b'x');
        for addr in [
            0x0040_e500,
            0x0040_3550,
            0x0041_fd00,
            0x0043_7730,
            0x0050_f9a0,
            0x0060_0ab0,
            0x0098_4f60,
            0x0050_f9c0,
            0x0070_37c0,
            0x0061_cc40,
            STRING_COPY,
        ] {
            recording(&mut e, &log, addr, 0);
        }
        recording(&mut e, &log, READER_LOAD_STRING, 0x5000);
        recording(&mut e, &log, FILE_POSITION, 0x77);
        let file = e.mem.alloc(8);
        let entry = e.mem.alloc(0x80);
        e.call(0x0084_dab0, &[file, entry]);
        let calls = log.borrow();
        let order: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(order[0], FILE_SEEK_SET);
        assert_eq!(calls[0].1, vec![file, 0]);
        let setters: Vec<(u32, Vec<u32>)> = calls
            .iter()
            .filter(|c| {
                [
                    0x0040_e500u32,
                    0x0040_3550,
                    0x0041_fd00,
                    0x0043_7730,
                    0x0050_f9a0,
                    0x0060_0ab0,
                    0x0098_4f60,
                    0x0050_f9c0,
                    0x0070_37c0,
                    0x0061_cc40,
                ]
                .contains(&c.0)
            })
            .cloned()
            .collect();
        let addrs: Vec<u32> = setters.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                0x0040_e500,
                0x0040_3550,
                0x0041_fd00,
                0x0043_7730,
                0x0050_f9a0,
                0x0060_0ab0,
                0x0098_4f60,
                0x0050_f9c0,
                0x0070_37c0,
                0x0061_cc40,
            ]
        );
        assert_eq!(setters[0].1, vec![entry, 0x30]);
        // The width and height values come from the raw reads (3 and 4).
        assert_eq!(setters[7].1, vec![entry, 3]);
        assert_eq!(setters[8].1, vec![entry, 4]);
        assert_eq!(setters[9].1, vec![entry, 0x77]);
        assert_eq!(setters[2].1, vec![entry, 0x5000]);
        assert_eq!(calls.last().unwrap().0, READER_DESTRUCT);
    }

    #[test]
    fn entry_fill_uses_the_default_name_for_a_bar_and_skips_the_fourth_word_when_small() {
        let (mut e, log, _) = reader_engine(0x30, b'|');
        for addr in [
            0x0040_e500,
            0x0040_3550,
            0x0041_fd00,
            0x0043_7730,
            0x0050_f9a0,
            0x0060_0ab0,
            0x0098_4f60,
            0x0050_f9c0,
            0x0070_37c0,
            0x0061_cc40,
            STRING_COPY,
        ] {
            recording(&mut e, &log, addr, 0);
        }
        recording(&mut e, &log, READER_LOAD_STRING, 0);
        recording(&mut e, &log, FILE_POSITION, 0);
        let file = e.mem.alloc(8);
        let entry = e.mem.alloc(0x80);
        e.call(0x0084_dab0, &[file, entry]);
        assert!(log
            .borrow()
            .contains(&(STRING_COPY, vec![entry + 0x3c, 0x40, DEFAULT_NAME])));
        // A smaller header has no fourth word.
        let (mut e, log, _) = reader_engine(0x2c, b'x');
        for addr in [
            0x0040_e500,
            0x0040_3550,
            0x0041_fd00,
            0x0043_7730,
            0x0050_f9a0,
            0x0060_0ab0,
            0x0098_4f60,
            0x0050_f9c0,
            0x0070_37c0,
            0x0061_cc40,
            STRING_COPY,
        ] {
            recording(&mut e, &log, addr, 0);
        }
        recording(&mut e, &log, READER_LOAD_STRING, 0);
        recording(&mut e, &log, FILE_POSITION, 0);
        let file = e.mem.alloc(8);
        let entry = e.mem.alloc(0x80);
        e.call(0x0084_dab0, &[file, entry]);
        assert!(!log.borrow().iter().any(|c| c.0 == 0x0050_f9a0));
    }

    #[test]
    fn entry_fill_tests_the_header_size_as_signed_for_the_fourth_word() {
        // 0084dca8: CMP/JLE is signed, so a size with the top bit set is
        // not above 0x2f: no fourth word is read and 0050f9a0 is not called.
        let (mut e, log, _) = reader_engine(0x8000_0030, b'x');
        for addr in [
            0x0040_e500,
            0x0040_3550,
            0x0041_fd00,
            0x0043_7730,
            0x0050_f9a0,
            0x0060_0ab0,
            0x0098_4f60,
            0x0050_f9c0,
            0x0070_37c0,
            0x0061_cc40,
            STRING_COPY,
        ] {
            recording(&mut e, &log, addr, 0);
        }
        recording(&mut e, &log, READER_LOAD_STRING, 0);
        recording(&mut e, &log, FILE_POSITION, 0);
        let file = e.mem.alloc(8);
        let entry = e.mem.alloc(0x80);
        e.call(0x0084_dab0, &[file, entry]);
        let calls = log.borrow();
        assert!(calls.contains(&(0x0040_e500, vec![entry, 0x8000_0030])));
        assert!(!calls.iter().any(|c| c.0 == 0x0050_f9a0));
        // Raw reads: the size, the 0x40 block, the width, the height and
        // the third word; no fourth word.
        assert_eq!(calls.iter().filter(|c| c.0 == READER_LOAD_RAW).count(), 5);
    }

    #[test]
    fn entry_fill_stops_when_the_magic_differs() {
        let (mut e, log, _) = reader_engine(0x30, b'x');
        e.register(STRCMP, |_, _| word_ret(1));
        let file = e.mem.alloc(8);
        let entry = e.mem.alloc(0x80);
        e.call(0x0084_dab0, &[file, entry]);
        assert!(!log.borrow().iter().any(|c| c.0 == READER_CONSTRUCT));
    }

    /// The doubles `SaveHeader` needs; `no_screenshot` is the manager's
    /// byte, `ratio` the two integers' ratio numerator over denominator 100.
    fn save_engine(no_screenshot: bool, numerator: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        for (addr, size) in [
            (HEADER_MAGIC, 4),
            (SAVE_LOAD_MANAGER, 4),
            (PLAYER_POINTER, 4),
            (RATIO_LIMIT, 8),
        ] {
            e.map(addr, size);
        }
        e.mem.set_f64(RATIO_LIMIT, 1.5);
        let magic = e.mem.alloc(16);
        e.set_global(HEADER_MAGIC, magic);
        e.set_global(SAVE_LOAD_MANAGER, 0x0a00_0000u32);
        e.set_global(PLAYER_POINTER, 0x0a10_0000u32);
        let log: Log = Rc::default();
        recording(&mut e, &log, STRLEN, 11);
        recording(&mut e, &log, FILE_WRITE, 0);
        recording(&mut e, &log, WRITER_CONSTRUCT, 0);
        recording(&mut e, &log, WRITER_DESTRUCT, 0);
        recording(&mut e, &log, WRITER_SAVE, 0);
        recording(&mut e, &log, WRITER_SAVE_STRING, 0);
        recording(&mut e, &log, MEMSET_SMALL, 0);
        recording(&mut e, &log, MEMSET, 0);
        recording(&mut e, &log, STRING_APPEND, 0);
        recording(&mut e, &log, STRING_COPY, 0);
        recording(&mut e, &log, GET_LOCATION_NAME, 0);
        recording(&mut e, &log, GET_PLAY_TIME, 0);
        recording(&mut e, &log, PLAYER_NAME, 0x6000);
        recording(&mut e, &log, PLAYER_TITLE, 0x6100);
        recording(&mut e, &log, PLAYER_LEVEL, 0x1_0007);
        recording(&mut e, &log, MANAGER_WORD, 0x1234);
        recording(&mut e, &log, FILE_NAME, 0x6200);
        recording(&mut e, &log, SETTING_STRING, 0x6300);
        recording(&mut e, &log, HEADER_SIZE_GETTER, 0x30);
        recording(&mut e, &log, NO_SCREENSHOT_FLAG, u32::from(no_screenshot));
        recording(&mut e, &log, SCREEN_NUMERATOR, numerator);
        recording(&mut e, &log, SCREEN_DENOMINATOR, 100);
        recording(&mut e, &log, PIXEL_DATA_POINTER, 0x7000);
        // The screenshot-enabled setting's value byte.
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        recording(&mut e, &log, SETTING_VALUE_ADDRESS, flag);
        // Saved data is logged with the value read at the time of the call.
        let sink_log = log.clone();
        e.register_double(WRITER_SAVE_DATA, move |e, a| {
            let mut entry = a.to_vec();
            entry.push(e.mem.u32(a[1]));
            sink_log.borrow_mut().push((WRITER_SAVE_DATA, entry));
            Ret::default()
        });
        // The screenshot routine logs the width and height it is given.
        let sink_log = log.clone();
        e.register_double(TAKE_SCREENSHOT, move |e, a| {
            let sizes = vec![e.mem.u32(a[0]), e.mem.u32(a[1]), a[2]];
            sink_log.borrow_mut().push((TAKE_SCREENSHOT, sizes));
            word_ret(0x0b00_0000)
        });
        // The pixel data object's deleting destructor.
        e.put_vtable(0x0c00_0000, &[0x0c00_0100]);
        e.mem.map(0x0b00_0000, 8);
        e.mem.set_u32(0x0b00_0000, 0x0c00_0000);
        recording(&mut e, &log, 0x0c00_0100, 0);
        (e, log)
    }

    #[test]
    fn save_header_with_a_real_screenshot() {
        let (mut e, log) = save_engine(false, 100);
        let file = e.mem.alloc(8);
        e.call(0x0084_d4b0, &[file]);
        let calls = log.borrow();
        // The magic string goes first.
        assert_eq!(calls[0].0, STRLEN);
        assert_eq!(
            calls[1],
            (FILE_WRITE, vec![file, e.mem.u32(HEADER_MAGIC), 11])
        );
        // 100/100 = 1.0 < 1.5: the large size,.
        let take = calls.iter().find(|c| c.0 == TAKE_SCREENSHOT).unwrap();
        assert_eq!(take.1[..2], [0x200, 0x200]);
        // The pixels are written after the buffer is saved to the file.
        let save = calls.iter().position(|c| c.0 == WRITER_SAVE).unwrap();
        assert_eq!(
            calls[save + 1..].iter().map(|c| c.0).collect::<Vec<_>>(),
            vec![
                NO_SCREENSHOT_FLAG,
                PIXEL_DATA_POINTER,
                FILE_WRITE,
                0x0c00_0100,
                WRITER_DESTRUCT
            ]
        );
        assert_eq!(calls[save + 3].1, vec![file, 0x7000, 0x200 * 0x200 * 3]);
        assert_eq!(calls[save + 4].1, vec![0x0b00_0000, 1]);
        // The path is made of the directory, the file name and the extension.
        let appends: Vec<&(u32, Vec<u32>)> =
            calls.iter().filter(|c| c.0 == STRING_APPEND).collect();
        assert_eq!(appends.len(), 3);
        assert_eq!(appends[1].1[2], 0x6200);
        assert_eq!(appends[2].1[2], SCREENSHOT_EXTENSION);
        // The level is the 16-bit part of the actor value.
        let saves: Vec<&(u32, Vec<u32>)> =
            calls.iter().filter(|c| c.0 == WRITER_SAVE_DATA).collect();
        assert_eq!(saves.len(), 6);
        assert!(saves.iter().all(|c| c.1[2] == 4 || c.1[2] == 0x40));
        // The values: the header size, the width and height, the manager's word and the level.
        assert_eq!(saves[0].1[4], 0x30);
        assert_eq!(saves[2].1[4], 0x200);
        assert_eq!(saves[3].1[4], 0x200);
        assert_eq!(saves[4].1[4], 0x1234);
        assert_eq!(saves[5].1[4], 7);
    }

    #[test]
    fn save_header_without_a_screenshot_scales_the_height_and_writes_a_null_buffer() {
        // 30/100 = 0.3 < 1.5: size 0x200; the height becomes trunc(512 * 0.3f32).
        let (mut e, log) = save_engine(true, 30);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word_ret(value as i32 as u32)
        });
        let file = e.mem.alloc(8);
        e.call(0x0084_d4b0, &[file]);
        let calls = log.borrow();
        assert!(!calls.iter().any(|c| c.0 == TAKE_SCREENSHOT));
        let expected_height = (512.0f64 * (0.3f64 as f32) as f64) as i32 as u32;
        let writes: Vec<&(u32, Vec<u32>)> = calls.iter().filter(|c| c.0 == FILE_WRITE).collect();
        assert_eq!(
            writes.last().unwrap().1,
            vec![file, 0, 0x200 * expected_height * 3]
        );
        // No pixel data object: nothing is destroyed through a vtable.
        assert!(!calls.iter().any(|c| c.0 == 0x0c00_0100));
        assert!(!calls.iter().any(|c| c.0 == PIXEL_DATA_POINTER));
    }

    #[test]
    fn save_header_uses_the_small_size_at_or_above_the_limit() {
        let (mut e, log) = save_engine(false, 150);
        let file = e.mem.alloc(8);
        e.call(0x0084_d4b0, &[file]);
        let calls = log.borrow();
        let take = calls.iter().find(|c| c.0 == TAKE_SCREENSHOT).unwrap();
        assert_eq!(take.1[..2], [0x1b0, 0x1b0]);
    }

    #[test]
    fn save_header_skips_the_path_when_the_setting_is_off() {
        let (mut e, log) = save_engine(false, 100);
        let off = e.mem.alloc(4);
        recording(&mut e, &log, SETTING_VALUE_ADDRESS, off);
        let file = e.mem.alloc(8);
        e.call(0x0084_d4b0, &[file]);
        let calls = log.borrow();
        assert!(!calls.iter().any(|c| c.0 == STRING_COPY));
        assert_eq!(calls.iter().filter(|c| c.0 == STRING_APPEND).count(), 1);
    }

    #[test]
    fn screenshot_loader_builds_the_image_reads_the_pixels_and_finishes_it() {
        let mut e = Engine::new();
        e.map(IMAGE_FORMAT, 0x44);
        e.mem.set_u32(IMAGE_FORMAT, 0xdead_beef);
        let log: Log = Rc::default();
        recording(&mut e, &log, FILE_SEEK_SET, 0);
        recording(&mut e, &log, OPERATOR_NEW, 0x0d00_0000);
        recording(&mut e, &log, IMAGE_CONSTRUCT, 0x0d00_0000);
        recording(&mut e, &log, PIXEL_DATA_POINTER, 0x0d10_0000);
        recording(&mut e, &log, FILE_READ, 0);
        recording(&mut e, &log, RECORD_CONSTRUCT, 0);
        recording(&mut e, &log, FIXED_STRING_CONSTRUCT, 0);
        recording(&mut e, &log, FIXED_STRING_TEXT, 0x0d20_0000);
        recording(&mut e, &log, IMAGE_FINISH, 0x99);
        recording(&mut e, &log, FIXED_STRING_DESTRUCT, 0);
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_dda0, &[file, 0x500, 4, 3]).u32(), 0x99);
        let calls = log.borrow();
        assert_eq!(calls[0], (FILE_SEEK_SET, vec![file, 0x500]));
        assert_eq!(calls[1], (OPERATOR_NEW, vec![0x74]));
        // The construction gets the width, the height, a copy of the format and two ones.
        let construct = &calls[2];
        assert_eq!(construct.0, IMAGE_CONSTRUCT);
        assert_eq!(&construct.1[..3], &[0x0d00_0000, 4, 3]);
        assert_eq!(&construct.1[4..], &[1, 1]);
        assert_eq!(calls[3], (PIXEL_DATA_POINTER, vec![0x0d00_0000, 0, 0]));
        assert_eq!(calls[4], (FILE_READ, vec![file, 0x0d10_0000, 36]));
        let finish = calls.iter().find(|c| c.0 == IMAGE_FINISH).unwrap();
        assert_eq!(finish.1[..2], [0x0d00_0000, 0x0d20_0000]);
        let record = finish.1[2];
        assert_eq!(
            (
                e.mem.u32(record),
                e.mem.u32(record + 4),
                e.mem.u32(record + 8)
            ),
            (2, 0, 0)
        );
        assert_eq!(calls.last().unwrap().0, FIXED_STRING_DESTRUCT);
    }

    #[test]
    fn screenshot_loader_skips_the_construction_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.map(IMAGE_FORMAT, 0x44);
        let log: Log = Rc::default();
        recording(&mut e, &log, FILE_SEEK_SET, 0);
        recording(&mut e, &log, OPERATOR_NEW, 0);
        recording(&mut e, &log, PIXEL_DATA_POINTER, 0);
        recording(&mut e, &log, FILE_READ, 0);
        recording(&mut e, &log, RECORD_CONSTRUCT, 0);
        recording(&mut e, &log, FIXED_STRING_CONSTRUCT, 0);
        recording(&mut e, &log, FIXED_STRING_TEXT, 0);
        recording(&mut e, &log, IMAGE_FINISH, 7);
        recording(&mut e, &log, FIXED_STRING_DESTRUCT, 0);
        let file = e.mem.alloc(8);
        assert_eq!(e.call(0x0084_dda0, &[file, 0, 2, 2]).u32(), 7);
        assert!(!log.borrow().iter().any(|c| c.0 == IMAGE_CONSTRUCT));
    }
}

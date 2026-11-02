//! `fallout shared/lipsynchanim.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A lip-sync animation (`.lip` file): per frame 16 phoneme weights and 17
//! modifier values, read from a file whose data may be run-length packed
//! (a 0 byte followed by a 16-bit count of zero bytes).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `LipSynchAnim` (Xbox PDB), 0x10 bytes (the PC code allocates 0x10 and
    /// uses these offsets).
    pub struct LipSynchAnim: 0x10 {
        /// `iNumFrames` (Xbox PDB).
        0x00 iNumFrames: u32,
        /// `iStartingFrame` (Xbox PDB).
        0x04 iStartingFrame: i32,
        /// `ppPhonemes` (Xbox PDB): `float **`, one row of 16 per frame.
        0x08 ppPhonemes: u32,
        /// `ppModifiers` (Xbox PDB): `float **`, one row of 17 per frame.
        0x0C ppModifiers: u32,
    }
}

/// The game's allocator (`00401000`, `cdecl`, size).
const ALLOCATE: u32 = 0x0040_1000;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset` (`00403d30`, `cdecl`: destination, value, count).
const MEMSET: u32 = 0x0040_3d30;
/// `FileFinder::Exist` (engine map), `cdecl`, four words.
const FILE_FINDER_EXIST: u32 = 0x0045_6a20;
/// `FileFinder::GetFile_ov2` (engine map), `cdecl`, four words.
const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;
/// `00462d80`: reads bytes from a file (`thiscall`: buffer, count); the
/// count read.
const FILE_READ: u32 = 0x0046_2d80;
/// The word passed with 1 to the file's slot `0x14` (a seek) when the data
/// is unpacked a second time.
const SEEK_ARGUMENT_010A2480: u32 = 0x010a_2480;
/// `00403e20`: the address of a setting's value (`thiscall`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The three settings that scale modifiers 14, 15 and 16.
const SETTING_011C6528: u32 = 0x011c_6528;
const SETTING_011C6534: u32 = 0x011c_6534;
const SETTING_011C651C: u32 = 0x011c_651c;
/// The most frames a file may have.
const MAX_FRAMES: u32 = 0xf_4240;

/// `MUL` by `factor` with the compiler's overflow guard (`SETO; NEG; OR`):
/// `0xffffffff` when the product does not fit in a word.
fn checked_size(count: u32, factor: u32) -> u32 {
    let product = u64::from(count) * u64::from(factor);
    if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    }
}

// Translated from 004d4f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the [`LipSynchAnim`] constructor for `frames` frames from
/// `start`: allocates the two row tables and one zeroed block of
/// `frames * 0x440` bytes, giving frame `i` phoneme row `block + 0x84 * i`
/// and modifier row `block + 0x84 * i + 0x40`. Returns `this`.
pub fn fn_004d4f70(
    e: &mut Engine,
    this: Ptr<LipSynchAnim>,
    frames: u32,
    start: i32,
) -> Ptr<LipSynchAnim> {
    e.set(this, LipSynchAnim::iNumFrames, frames);
    e.set(this, LipSynchAnim::iStartingFrame, start);
    let count = e.get(this, LipSynchAnim::iNumFrames);
    let phonemes = e.call(ALLOCATE, &args![checked_size(count, 4)]).u32();
    e.set(this, LipSynchAnim::ppPhonemes, phonemes);
    let count = e.get(this, LipSynchAnim::iNumFrames);
    let modifiers = e.call(ALLOCATE, &args![checked_size(count, 4)]).u32();
    e.set(this, LipSynchAnim::ppModifiers, modifiers);
    let count = e.get(this, LipSynchAnim::iNumFrames);
    let mut block = e
        .call(ALLOCATE, &args![checked_size(count.wrapping_mul(0x110), 4)])
        .u32();
    let count = e.get(this, LipSynchAnim::iNumFrames);
    e.call(MEMSET, &args![block, 0u32, count.wrapping_mul(0x440)]);
    let mut index = 0u32;
    while index < e.get(this, LipSynchAnim::iNumFrames) {
        let phonemes = e.get(this, LipSynchAnim::ppPhonemes);
        e.mem.set_u32(phonemes.wrapping_add(index * 4), block);
        block = block.wrapping_add(0x40);
        let modifiers = e.get(this, LipSynchAnim::ppModifiers);
        e.mem.set_u32(modifiers.wrapping_add(index * 4), block);
        block = block.wrapping_add(0x44);
        index += 1;
    }
    this
}

// Translated from 004d5080 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the [`LipSynchAnim`] destructor body: frees the rows'
/// block (the first phoneme row), then both row tables.
pub fn fn_004d5080(e: &mut Engine, this: Ptr<LipSynchAnim>) {
    let phonemes = e.get(this, LipSynchAnim::ppPhonemes);
    let block = e.mem.u32(phonemes);
    e.call(OPERATOR_DELETE, &args![block]);
    let phonemes = e.get(this, LipSynchAnim::ppPhonemes);
    e.call(OPERATOR_DELETE, &args![phonemes]);
    let modifiers = e.get(this, LipSynchAnim::ppModifiers);
    e.call(OPERATOR_DELETE, &args![modifiers]);
}

// Translated from 004d52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipSynchAnim::LoadFromFile` (engine map), `cdecl`: when `path` is set
/// and `FileFinder::Exist(path, 0, 0, -1)`, the animation read from
/// `FileFinder::GetFile_ov2(path, 0, 0x4000, 0x10)` ([`lip_synch_anim_load_from_file_ov2`]);
/// otherwise 0.
pub fn lip_synch_anim_load_from_file(e: &mut Engine, path: u32) -> u32 {
    if path == 0 {
        return 0;
    }
    if e.call(FILE_FINDER_EXIST, &args![path, 0u32, 0u32, 0xffff_ffffu32])
        .u32()
        == 0
    {
        return 0;
    }
    let file = e
        .call(FILE_FINDER_GET_FILE, &args![path, 0u32, 0x4000u32, 0x10u32])
        .u32();
    lip_synch_anim_load_from_file_ov2(e, file)
}

// Translated from 004d5350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipSynchAnim::LoadFromFile_ov2` (engine map), `cdecl`: reads an
/// animation from an open file and deletes the file (its slot 0 with 1).
/// 0 when there is no file or it does not open (slot `0x20(1, 0)`).
///
/// The file starts with a 12-byte header (format, data size, packed flag).
/// A header whose first word is not 1 is an old file: the rest of the file
/// (its size, slot `0x28`, less one) is read once to count the unpacked
/// size, the file is sought back (slot `0x14(1, [010a2480])`), and the
/// data is read packed; otherwise the size is less the header. The data
/// (in a block 0x10 bytes larger) is unpacked ([`fn_004d5880`]) or read as
/// is, then parsed ([`fn_004d5570`]).
pub fn lip_synch_anim_load_from_file_ov2(e: &mut Engine, file: u32) -> u32 {
    if file == 0 || !e.vcall(file, 0x20, &args![1u32, 0u32]).bool() {
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
        return 0;
    }
    e.with_stack(0xc, |e, header| {
        let header = header.addr();
        let mut remaining = e.vcall(file, 0x28, &args![]).u32();
        e.call(FILE_READ, &args![file, header, 0xcu32]);
        if e.mem.u32(header) == 1 {
            remaining = remaining.wrapping_sub(0xc);
        } else {
            remaining = remaining.wrapping_sub(1);
            let seek = e.global::<u32>(SEEK_ARGUMENT_010A2480);
            e.vcall(file, 0x14, &args![1u32, seek]);
            let packed = e.call(ALLOCATE, &args![remaining]).u32();
            e.call(FILE_READ, &args![file, packed, remaining]);
            let end = packed.wrapping_add(remaining);
            let mut size = 0u32;
            let mut offset = 0u32;
            while packed.wrapping_add(offset) < end {
                if e.mem.u8(packed.wrapping_add(offset)) != 0 {
                    size = size.wrapping_add(1);
                    offset = offset.wrapping_add(1);
                } else {
                    offset = offset.wrapping_add(1);
                    let zeros = e.mem.u16(packed.wrapping_add(offset));
                    size = size.wrapping_add(u32::from(zeros));
                    offset = offset.wrapping_add(2);
                }
            }
            e.mem.set_u32(header + 4, size);
            e.mem.set_u32(header, 1);
            e.mem.set_u32(header + 8, 1);
            let seek = e.global::<u32>(SEEK_ARGUMENT_010A2480);
            e.vcall(file, 0x14, &args![1u32, seek]);
            e.call(OPERATOR_DELETE, &args![packed]);
        }
        let size = e.mem.u32(header + 4);
        let data = e.call(ALLOCATE, &args![size.wrapping_add(0x10)]).u32();
        if e.mem.u32(header + 8) & 1 != 0 {
            fn_004d5880(e, file, data, remaining, size);
        } else {
            e.call(FILE_READ, &args![file, data, size]);
        }
        let anim = fn_004d5570(e, data, size);
        e.vcall(file, 0, &args![1u32]);
        e.call(OPERATOR_DELETE, &args![data]);
        anim
    })
}

// Translated from 004d5570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d5570 - the C++ exception frame (FS:[0]) around the allocation is not translated
/// No map name, `cdecl`: parses `size` bytes of animation data: frame
/// count (at most 1000000), starting frame, then per frame 16 phoneme and
/// 17 modifier floats (copied through the FPU); modifiers 14, 15 and 16 are
/// scaled by the settings at `011c6528`, `011c6534` and `011c651c`. 0 for
/// no data, too many frames, or data that ends early (the animation built
/// so far is deleted).
pub fn fn_004d5570(e: &mut Engine, data: u32, size: u32) -> u32 {
    if data == 0 || size == 0 {
        return 0;
    }
    let mut offset = 0u32;
    let mut anim = 0u32;
    let fail = |e: &mut Engine, anim: u32| {
        if anim != 0 {
            lip_synch_anim_scalar_deleting_destructor(e, Ptr::new(anim), 1);
        }
        0
    };
    if offset.wrapping_add(4) > size {
        return fail(e, anim);
    }
    let frames = e.mem.u32(data.wrapping_add(offset));
    offset = offset.wrapping_add(4);
    if frames > MAX_FRAMES {
        return 0;
    }
    if offset.wrapping_add(4) > size {
        return fail(e, anim);
    }
    let start = e.mem.i32(data.wrapping_add(offset));
    offset = offset.wrapping_add(4);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    anim = if block != 0 {
        fn_004d4f70(e, Ptr::new(block), frames, start).addr()
    } else {
        0
    };
    let this = Ptr::<LipSynchAnim>::new(anim);
    for frame in 0..frames {
        for index in 0..0x10u32 {
            if offset.wrapping_add(4) > size {
                return fail(e, anim);
            }
            let row = e.mem.u32(e.get(this, LipSynchAnim::ppPhonemes) + frame * 4);
            let value = x87_quiet_bits(e.mem.u32(data.wrapping_add(offset)));
            e.mem.set_u32(row.wrapping_add(index * 4), value);
            offset = offset.wrapping_add(4);
        }
        for index in 0..0x11u32 {
            if offset.wrapping_add(4) > size {
                return fail(e, anim);
            }
            let row = e
                .mem
                .u32(e.get(this, LipSynchAnim::ppModifiers) + frame * 4);
            let value = x87_quiet_bits(e.mem.u32(data.wrapping_add(offset)));
            e.mem.set_u32(row.wrapping_add(index * 4), value);
            offset = offset.wrapping_add(4);
        }
        for (field, setting) in [
            (0x38u32, SETTING_011C6528),
            (0x3c, SETTING_011C6534),
            (0x40, SETTING_011C651C),
        ] {
            let source = e
                .mem
                .u32(e.get(this, LipSynchAnim::ppModifiers) + frame * 4);
            let factor = e.call(SETTING_VALUE, &args![setting]).u32();
            let product = x87_mul(
                x87_load32(e.mem.u32(source + field)),
                x87_m32(e.mem.u32(factor)),
            );
            let target = e
                .mem
                .u32(e.get(this, LipSynchAnim::ppModifiers) + frame * 4);
            e.mem.set_u32(target + field, x87_store32(product));
        }
    }
    anim
}

// Translated from 004d5850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipSynchAnim::~LipSynchAnim` (engine map; the scalar deleting
/// destructor): [`fn_004d5080`], then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn lip_synch_anim_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LipSynchAnim>,
    flags: u32,
) -> Ptr<LipSynchAnim> {
    fn_004d5080(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004d5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: unpacks `remaining` bytes of the file into `out`:
/// a non-zero byte is copied, a 0 byte is followed by a 16-bit count of
/// zero bytes to write (`memset`) and counts as three. False when a read of
/// the next byte returns 0 (the rest is left). The fourth word is passed
/// but not read.
pub fn fn_004d5880(e: &mut Engine, file: u32, out: u32, remaining: u32, _unused_3: u32) -> bool {
    let mut out = out;
    let mut left = remaining;
    e.with_stack(0x10, |e, frame| {
        let count = frame.addr();
        let byte = frame.addr() + 7;
        while left != 0 {
            e.mem.set_u8(byte, 0);
            if e.call(FILE_READ, &args![file, byte, 1u32]).u32() == 0 {
                return false;
            }
            let value = e.mem.u8(byte);
            if value != 0 {
                e.mem.set_u8(out, value);
                out = out.wrapping_add(1);
                left = left.wrapping_sub(1);
            } else {
                e.mem.set_u16(count, 0);
                e.call(FILE_READ, &args![file, count, 2u32]);
                let zeros = u32::from(e.mem.u16(count));
                e.call(MEMSET, &args![out, 0u32, zeros]);
                out = out.wrapping_add(zeros);
                left = left.wrapping_sub(3);
            }
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004d_4f70,
            fn_004d4f70(Ptr<LipSynchAnim>, u32, i32) -> Ptr<LipSynchAnim>
        ),
        entry!(0x004d_5080, fn_004d5080(Ptr<LipSynchAnim>)),
        entry!(0x004d_52f0, lip_synch_anim_load_from_file(u32) -> u32),
        entry!(0x004d_5350, lip_synch_anim_load_from_file_ov2(u32) -> u32),
        entry!(0x004d_5570, fn_004d5570(u32, u32) -> u32),
        entry!(
            0x004d_5850,
            lip_synch_anim_scalar_deleting_destructor(Ptr<LipSynchAnim>, u32) -> Ptr<LipSynchAnim>
        ),
        entry!(0x004d_5880, fn_004d5880(u32, u32, u32, u32) -> bool),
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

    fn heap_doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn constructor_lays_out_rows() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        let this = e.new_object::<LipSynchAnim>();
        e.call_log = Some(vec![]);
        e.call(0x004d_4f70, &args![this, 3u32, -2i32]);
        assert_eq!(e.get(this, LipSynchAnim::iNumFrames), 3);
        assert_eq!(e.get(this, LipSynchAnim::iStartingFrame), -2);
        let sizes: Vec<u32> = calls_to(&e, ALLOCATE).iter().map(|c| c[0]).collect();
        assert_eq!(sizes, vec![12, 12, 3 * 0x440]);
        let phonemes = e.get(this, LipSynchAnim::ppPhonemes);
        let modifiers = e.get(this, LipSynchAnim::ppModifiers);
        let block = e.mem.u32(phonemes);
        for i in 0..3 {
            assert_eq!(e.mem.u32(phonemes + 4 * i), block + 0x84 * i);
            assert_eq!(e.mem.u32(modifiers + 4 * i), block + 0x84 * i + 0x40);
        }
        assert_eq!(checked_size(0x4000_0000, 4), u32::MAX);
        assert_eq!(checked_size(0x3fff_ffff, 4), 0xffff_fffc);

        e.call_log = Some(vec![]);
        let r = e.call(0x004d_5850, &args![this, 1u32]).u32();
        assert_eq!(r, this.addr());
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![
                vec![block],
                vec![phonemes],
                vec![modifiers],
                vec![this.addr()]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x004d_5850, &args![this, 0u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 3);
    }

    fn settings(e: &mut Engine) {
        for (setting, value) in [
            (SETTING_011C6528, 2.0f32),
            (SETTING_011C6534, 0.5),
            (SETTING_011C651C, 4.0),
        ] {
            let cell = 0x7600_0000 + (setting & 0xfff);
            e.map(cell & !0xfff, 0x1000);
            e.mem.set_f32(cell, value);
        }
        e.register(SETTING_VALUE, |_, a| {
            (0x7600_0000 + (a[0] & 0xfff)).into_ret()
        });
    }

    fn anim_data(e: &mut Engine, frames: u32, floats: u32) -> (u32, u32) {
        let size = 8 + 4 * floats;
        let data = e.mem.alloc(size);
        e.mem.set_u32(data, frames);
        e.mem.set_i32(data + 4, -1);
        for i in 0..floats {
            e.mem.set_f32(data + 8 + 4 * i, i as f32);
        }
        (data, size)
    }

    #[test]
    fn parse_fills_rows_and_scales_modifiers() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        settings(&mut e);
        let (data, size) = anim_data(&mut e, 2, 66);
        e.mem.set_u32(data + 8 + 4 * 3, 0x7f80_0001);
        let anim = e.call(0x004d_5570, &args![data, size]).u32();
        let this = Ptr::<LipSynchAnim>::new(anim);
        assert_eq!(e.get(this, LipSynchAnim::iNumFrames), 2);
        assert_eq!(e.get(this, LipSynchAnim::iStartingFrame), -1);
        let phonemes = e.get(this, LipSynchAnim::ppPhonemes);
        let modifiers = e.get(this, LipSynchAnim::ppModifiers);
        let row0 = e.mem.u32(phonemes);
        assert_eq!(e.mem.f32(row0 + 4), 1.0);
        assert_eq!(e.mem.u32(row0 + 12), 0x7fc0_0001);
        let mods1 = e.mem.u32(modifiers + 4);
        // Frame 1's modifiers are floats 49..65; 14, 15, 16 are scaled.
        assert_eq!(e.mem.f32(mods1), 49.0);
        assert_eq!(e.mem.f32(mods1 + 0x38), 63.0 * 2.0);
        assert_eq!(e.mem.f32(mods1 + 0x3c), 64.0 * 0.5);
        assert_eq!(e.mem.f32(mods1 + 0x40), 65.0 * 4.0);
    }

    #[test]
    fn parse_refuses_short_or_oversized_data() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        settings(&mut e);
        assert_eq!(e.call(0x004d_5570, &args![0u32, 8u32]).u32(), 0);
        let (data, _) = anim_data(&mut e, 2, 66);
        assert_eq!(e.call(0x004d_5570, &args![data, 3u32]).u32(), 0);
        assert_eq!(e.call(0x004d_5570, &args![data, 7u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5570, &args![data, 8 + 4 * 20u32]).u32(), 0);
        // The partly built animation is deleted (rows, tables, itself).
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 4);
        e.mem.set_u32(data, MAX_FRAMES + 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5570, &args![data, 0x1000u32]).u32(), 0);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    /// A file of `bytes`: reads move a cursor; slot 0x20 opens it, 0x28 is
    /// its size, 0x14 seeks back to 12, slot 0 deletes it.
    fn file(e: &mut Engine, bytes: &[u8]) -> u32 {
        let file = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x30);
        e.mem.set_u32(file, table);
        let content = e.mem.alloc(bytes.len() as u32);
        e.mem.write(content, bytes);
        e.mem.set_u32(file + 4, content);
        e.mem.set_u32(file + 8, bytes.len() as u32);
        e.mem.set_u32(file + 0xc, 0);
        for slot in [0u32, 0x14, 0x20, 0x28] {
            e.mem.set_u32(table + slot, 0x7700_0000 + slot);
        }
        e.register(0x7700_0000, |_, _| Ret::default());
        e.register(0x7700_0014, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 12);
            Ret::default()
        });
        e.register(0x7700_0020, |_, _| 1u32.into_ret());
        e.register(0x7700_0028, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FILE_READ, |e, a| {
            let (file, buffer, count) = (a[0], a[1], a[2]);
            let cursor = e.mem.u32(file + 0xc);
            let length = e.mem.u32(file + 8);
            let take = count.min(length - cursor);
            let bytes = e.mem.bytes(e.mem.u32(file + 4) + cursor, take);
            e.mem.write(buffer, &bytes);
            e.mem.set_u32(file + 0xc, cursor + take);
            take.into_ret()
        });
        e.map(SEEK_ARGUMENT_010A2480, 4);
        file
    }

    #[test]
    fn unpack_copies_and_expands_zero_runs() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        let f = file(&mut e, &[5, 0, 3, 0, 7]);
        let out = e.mem.alloc(16);
        e.mem.write(out, &[0xff; 16]);
        assert!(e.call(0x004d_5880, &args![f, out, 5u32, 99u32]).bool());
        assert_eq!(e.mem.bytes(out, 6), vec![5, 0, 0, 0, 7, 0xff]);
        // The file ends early: false.
        let f = file(&mut e, &[5]);
        assert!(!e.call(0x004d_5880, &args![f, out, 4u32, 0u32]).bool());
    }

    fn le(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn load_reads_a_current_file() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        settings(&mut e);
        // Header (format 1, size, not packed), then one frame.
        let mut body = le(&[1, 0]);
        body.extend(le(&[0; 33]));
        let mut bytes = le(&[1, body.len() as u32, 0]);
        bytes.extend(&body);
        let f = file(&mut e, &bytes);
        e.call_log = Some(vec![]);
        let anim = e.call(0x004d_5350, &args![f]).u32();
        assert_ne!(anim, 0);
        assert_eq!(e.mem.u32(anim), 1);
        assert_eq!(calls_to(&e, 0x7700_0000), vec![vec![f, 1]]);
        assert!(calls_to(&e, 0x7700_0014).is_empty());
        assert_eq!(calls_to(&e, ALLOCATE)[0], vec![body.len() as u32 + 0x10]);
    }

    #[test]
    fn load_counts_and_unpacks_an_old_file() {
        let mut e = Engine::new();
        heap_doubles(&mut e);
        settings(&mut e);
        // Old header (format 0); the packed data: the byte 1, then a run of
        // 139 zero bytes (frames 1, start 0, 33 zero floats: 140 bytes).
        let mut bytes = le(&[0, 0, 0]);
        bytes.extend([1, 0, 139, 0]);
        let f = file(&mut e, &bytes);
        e.call_log = Some(vec![]);
        let anim = e.call(0x004d_5350, &args![f]).u32();
        assert_ne!(anim, 0);
        assert_eq!(e.mem.u32(anim), 1);
        assert_eq!(calls_to(&e, 0x7700_0014).len(), 2);
        // The rest of the file less one byte is read packed (15 bytes, the
        // missing ones zero), counted to 140 and unpacked.
        let sizes: Vec<u32> = calls_to(&e, ALLOCATE).iter().map(|c| c[0]).collect();
        assert_eq!(sizes[..2], [15, 140 + 0x10]);
    }

    #[test]
    fn load_without_a_file_or_an_open_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004d_5350, &args![0u32]).u32(), 0);
        let f = file(&mut e, &[]);
        e.register(0x7700_0020, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5350, &args![f]).u32(), 0);
        assert_eq!(calls_to(&e, 0x7700_0000), vec![vec![f, 1]]);

        e.register(FILE_FINDER_EXIST, |_, a| (a[0] == 0x1234).into_ret());
        e.register(FILE_FINDER_GET_FILE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_52f0, &args![0x1234u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, FILE_FINDER_GET_FILE),
            vec![vec![0x1234, 0, 0x4000, 0x10]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_52f0, &args![0x9999u32]).u32(), 0);
        assert!(calls_to(&e, FILE_FINDER_GET_FILE).is_empty());
        assert_eq!(e.call(0x004d_52f0, &args![0u32]).u32(), 0);
    }
}

//! `fallout/misc/saveload/bgssavegamebuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSSaveGameBuffer` (Xbox PDB), PC fields verified in 00865bd0.
    pub struct BGSSaveGameBuffer: 0x14 {
        0x04 pBuffer: Ptr<u8>,
        0x08 iBufferSize: u32,
        0x0c iBufferPosition: u32,
        0x10 iDebugOverhead: u32,
    }
}

// Translated from 00865bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::BGSSaveGameBuffer` (Xbox PDB): installs the PC
/// vtable and initializes all four data words to zero; returns this.
pub fn construct(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>) -> Ptr<BGSSaveGameBuffer> {
    e.mem.set_u32(this.addr(), 0x01082324);
    for offset in [4, 8, 12, 16] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    this
}

// Translated from 00865ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes at an explicit position, optionally appending the native '|'
/// separator. Capacity/position comparisons are unsigned with wrapping
/// arithmetic. Mode and buffer fields are reloaded across callbacks.
pub fn write_at(
    e: &mut Engine,
    this: Ptr<BGSSaveGameBuffer>,
    source: Ptr,
    size: u32,
    position: u32,
    no_separator: u8,
) {
    let end = position
        .wrapping_add(size)
        .wrapping_add(u32::from(no_separator == 0));
    while end > e.get(this, BGSSaveGameBuffer::iBufferSize) {
        e.call(0x008660b0, &args![this]);
    }
    let state = e.mem.u32(0x011de134);
    if e.call(0x00424940, &args![state]).u8() == 0 {
        let destination = e
            .get(this, BGSSaveGameBuffer::pBuffer)
            .addr()
            .wrapping_add(position);
        e.call(0x00401460, &args![destination, source, size]);
    }
    if no_separator == 0 {
        let state = e.mem.u32(0x011de134);
        if e.call(0x00424940, &args![state]).u8() == 0 {
            let buffer = e.get(this, BGSSaveGameBuffer::pBuffer).addr();
            e.mem
                .set_u8(buffer.wrapping_add(position).wrapping_add(size), b'|');
        }
        if position == e.get(this, BGSSaveGameBuffer::iBufferPosition) {
            let overhead = e
                .get(this, BGSSaveGameBuffer::iDebugOverhead)
                .wrapping_add(1);
            e.set(this, BGSSaveGameBuffer::iDebugOverhead, overhead);
        }
    }
    if e.get(this, BGSSaveGameBuffer::iBufferPosition) < end {
        e.set(this, BGSSaveGameBuffer::iBufferPosition, end);
    }
}

// Translated from 00865e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves with a separator. PC consumes three stack words (RET 0xc),
/// ignores the third, and always forwards zero to the append wrapper.
pub fn save_bytes(
    e: &mut Engine,
    this: Ptr<BGSSaveGameBuffer>,
    source: Ptr,
    size: u32,
    _unused: u32,
) {
    e.call(0x00865570, &args![this, source, size, 0u8]);
}

// Translated from 00865f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::StartVariableSizedValue` (Xbox PDB): writes '#|'
/// and returns the position captured before writing.
pub fn start_variable_value(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>) -> u32 {
    let start = e.get(this, BGSSaveGameBuffer::iBufferPosition);
    e.with_stack(4, |e, marker| {
        e.mem.set_u8(marker.addr(), b'#');
        let position = e.get(this, BGSSaveGameBuffer::iBufferPosition);
        write_at(e, this, marker, 1, position, 0);
    });
    start
}

// Translated from 00866140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts space before existing bytes. The PC uses signed comparisons here,
/// unlike write_at, and retains the original end across callbacks.
pub fn insert_space(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, position: u32, extra: u32) {
    let end = e
        .get(this, BGSSaveGameBuffer::iBufferPosition)
        .wrapping_add(extra);
    if end as i32 <= 0 {
        return;
    }
    while (end as i32) > (e.get(this, BGSSaveGameBuffer::iBufferSize) as i32) {
        e.call(0x008660b0, &args![this]);
    }
    let state = e.mem.u32(0x011de134);
    if e.call(0x00424940, &args![state]).u8() == 0 {
        let buffer = e.get(this, BGSSaveGameBuffer::pBuffer).addr();
        let source = buffer.wrapping_add(position);
        let destination = source.wrapping_add(extra);
        let capacity = e
            .get(this, BGSSaveGameBuffer::iBufferSize)
            .wrapping_sub(position.wrapping_add(extra));
        let size = e
            .get(this, BGSSaveGameBuffer::iBufferPosition)
            .wrapping_sub(position);
        e.call(0x008661e0, &args![destination, capacity, source, size]);
    }
    e.set(this, BGSSaveGameBuffer::iBufferPosition, end);
}

// Translated from 00865ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (Xbox PDB): patches the
/// placeholder using one, two or four tagged bytes followed by '|'.
pub fn finish_variable_value(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, value: u32, start: u32) {
    e.with_stack(4, |e, encoded| {
        if value < 0x40 {
            e.mem.set_u8(encoded.addr(), (value << 2) as u8);
            write_at(e, this, encoded, 1, start, 0);
        } else if value < 0x4000 {
            insert_space(e, this, start, 1);
            e.mem.set_u16(encoded.addr(), ((value << 2) | 1) as u16);
            write_at(e, this, encoded, 2, start, 0);
        } else if value < 0x40000000 {
            insert_space(e, this, start, 3);
            e.mem.set_u32(encoded.addr(), (value << 2) | 2);
            write_at(e, this, encoded, 4, start, 0);
        } else {
            e.call(0x00845cc0, &args![0x0107ee00u32, value]);
        }
    });
}

// Translated from 00865db0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB): encode the numeric form ID
/// and append its three bytes plus separator. The second stack word is unused.
pub fn save_numeric_form_id(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, id: u32, _unused: u32) {
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        e.call(0x00853570, &args![index, id]);
        e.call(0x00865570, &args![this, index, 3u32, 0u8]);
    });
}
// Translated from 00865df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB): writes a three-byte
/// numeric index for a form (zero for null). The second stack word is
/// consumed but unused by the PC body.
pub fn save_form_id(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, form: Ptr, _unused: u32) {
    let id = if form.is_null() {
        0
    } else {
        e.call(0x0084e3a0, &args![form]).u32()
    };
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        e.call(0x00853570, &args![index, id]);
        e.call(0x00865570, &args![this, index, 3u32, 0u8]);
    });
}

/// The vtable `BGSSaveGameBuffer`'s constructor and destructor store.
const SAVE_GAME_BUFFER_VTABLE: u32 = 0x0108_2324;
/// The game's `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// The append wrapper (`00865570`, thiscall: source, size, a byte flag).
const APPEND: u32 = 0x0086_5570;
/// The save-load mode object the code asks (`011de134`) and its query
/// (`00424940`, thiscall, a `bool` in AL; non-zero while only sizing).
const SAVE_LOAD_MODE: u32 = 0x011d_e134;
const MODE_SIZING_ONLY: u32 = 0x0042_4940;
/// `SaveGameDataToFileFunction` (`00845d00`, cdecl: file, source, size).
const SAVE_DATA_TO_FILE: u32 = 0x0084_5d00;
/// `BGSSaveLoadFile::Write` (`00846330`, thiscall on the file: source,
/// size).
const SAVE_LOAD_FILE_WRITE: u32 = 0x0084_6330;
/// `strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `SaveGameWarning` (`00845cc0`, cdecl, a format and its words).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"SAVELOAD: Variable-sized value %08X is too large to be stored."`
const VALUE_TOO_LARGE_FORMAT: u32 = 0x0107_ee00;

// Translated from 00865c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::~BGSSaveGameBuffer` (Xbox PDB): the class's vtable,
/// then frees the buffer (`00401030(pBuffer)`, also when null).
pub fn destruct(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>) {
    e.mem.set_u32(this.addr(), SAVE_GAME_BUFFER_VTABLE);
    let buffer = e.get(this, BGSSaveGameBuffer::pBuffer);
    e.call(FREE, &args![buffer]);
}

// Translated from 00865c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::Save` (Xbox PDB): writes the buffer's position
/// (the 4 bytes of `iBufferPosition`) to `file` through
/// `SaveGameDataToFileFunction`, then the buffer's first `iBufferPosition`
/// bytes (`Save_ov2`).
pub fn save(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, file: Ptr) {
    e.call(SAVE_DATA_TO_FILE, &args![file, this.addr() + 0x0c, 4u32]);
    let size = e.get(this, BGSSaveGameBuffer::iBufferPosition);
    save_ov2(e, this, file, size);
}

// Translated from 00865c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::Save_ov2` (Xbox PDB): writes `size` bytes of the
/// buffer to `file` (`BGSSaveLoadFile::Write`). While the save-load mode
/// says sizing only (`00424940` on `011de134`) the source is null and only
/// a non-zero size is written; otherwise both the buffer and the size must
/// be non-zero.
pub fn save_ov2(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, file: Ptr, size: u32) {
    let mode = e.global::<u32>(SAVE_LOAD_MODE);
    if e.call(MODE_SIZING_ONLY, &args![mode]).u32() & 0xff != 0 {
        if size != 0 {
            e.call(SAVE_LOAD_FILE_WRITE, &args![file, 0u32, size]);
        }
    } else {
        let buffer = e.get(this, BGSSaveGameBuffer::pBuffer);
        if !buffer.is_null() && size != 0 {
            e.call(SAVE_LOAD_FILE_WRITE, &args![file, buffer, size]);
        }
    }
}

// Translated from 00865e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveString` (Xbox PDB; `RET 8`, the second word
/// unused): the string's length (`strlen`, 0 for null) as a 16-bit count,
/// then, when that count is non-zero, that many bytes of the string, each
/// through the append wrapper (`00865570`, flag 0).
pub fn save_string(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, text: Ptr, _unused_2: u32) {
    let length = if text.is_null() {
        0
    } else {
        e.call(STRLEN, &args![text]).u32()
    };
    let count = length as u16;
    e.with_stack(4, |e, local| {
        e.mem.set_u16(local.addr(), count);
        e.call(APPEND, &args![this, local, 2u32, 0u8]);
    });
    if count != 0 {
        e.call(APPEND, &args![this, text, u32::from(count), 0u8]);
    }
}

// Translated from 00865f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (Xbox PDB): appends `value`
/// tagged in its two low bits: one byte (`value << 2`) below 0x40, two
/// bytes (`value << 2 | 1`) below 0x4000, four bytes (`value << 2 | 2`)
/// below 0x40000000; larger values only give the save-game warning.
pub fn save_variable_sized_value(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, value: u32) {
    if value < 0x40 {
        e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), (value << 2) as u8);
            e.call(APPEND, &args![this, local, 1u32, 0u8]);
        });
    } else if value < 0x4000 {
        e.with_stack(4, |e, local| {
            e.mem.set_u16(local.addr(), ((value << 2) | 1) as u16);
            e.call(APPEND, &args![this, local, 2u32, 0u8]);
        });
    } else if value < 0x4000_0000 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), (value << 2) | 2);
            e.call(APPEND, &args![this, local, 4u32, 0u8]);
        });
    } else {
        e.call(SAVE_GAME_WARNING, &args![VALUE_TOO_LARGE_FORMAT, value]);
    }
}

/// `006815c0` (`thiscall`): the quaternion local's (empty) constructor.
const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// `NiQuaternion::FromRotation` (`00a6df40`, `thiscall`, `RET 4`: the
/// matrix).
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;

// Translated from 00865ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveData` (Xbox PDB) for a rotation (`thiscall`,
/// `RET 0xc`; only the first word is read): the matrix `rotation` as a
/// 16-byte quaternion (`NiQuaternion::FromRotation` on a local), saved
/// with `00865e50` (16 bytes, 0).
pub fn bgs_save_game_buffer_save_data(
    e: &mut Engine,
    this: Ptr<BGSSaveGameBuffer>,
    rotation: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    e.with_stack(0x10, |e, quaternion| {
        e.call(TRIVIAL_CONSTRUCT, &args![quaternion]);
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, rotation]);
        save_bytes(e, this, quaternion, 0x10, 0);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0086_5ee0,
            bgs_save_game_buffer_save_data(Ptr<BGSSaveGameBuffer>, u32, u32, u32)
        ),
        entry!(0x0086_5c10, destruct(Ptr<BGSSaveGameBuffer>)),
        entry!(0x0086_5c40, save(Ptr<BGSSaveGameBuffer>, Ptr)),
        entry!(0x0086_5c80, save_ov2(Ptr<BGSSaveGameBuffer>, Ptr, u32)),
        entry!(0x0086_5e70, save_string(Ptr<BGSSaveGameBuffer>, Ptr, u32)),
        entry!(
            0x0086_5f60,
            save_variable_sized_value(Ptr<BGSSaveGameBuffer>, u32)
        ),
        entry!(
            0x00865db0,
            save_numeric_form_id(Ptr<BGSSaveGameBuffer>, u32, u32)
        ),
        entry!(0x00865df0, save_form_id(Ptr<BGSSaveGameBuffer>, Ptr, u32)),
        entry!(0x00866140, insert_space(Ptr<BGSSaveGameBuffer>, u32, u32)),
        entry!(
            0x00865ff0,
            finish_variable_value(Ptr<BGSSaveGameBuffer>, u32, u32)
        ),
        entry!(
            0x00865bd0,
            construct(Ptr<BGSSaveGameBuffer>) -> Ptr<BGSSaveGameBuffer>
        ),
        entry!(
            0x00865ce0,
            write_at(Ptr<BGSSaveGameBuffer>, Ptr, u32, u32, u8)
        ),
        entry!(
            0x00865e50,
            save_bytes(Ptr<BGSSaveGameBuffer>, Ptr, u32, u32)
        ),
        entry!(
            0x00865f20,
            start_variable_value(Ptr<BGSSaveGameBuffer>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_form_writer_matches_native_created_id_vectors() {
        for (id, encoded) in [
            (0xff00_0000, [0x80, 0, 0]),
            (0xff12_3456, [0x92, 0x34, 0x56]),
            (0xff7f_ffff, [0xff, 0xff, 0xff]),
        ] {
            for unused in [0, 0xdead_beef] {
                let mut e = Engine::new();
                for address in [0x011de134, 0x011ddf38, 0x011c3f2c] {
                    e.map(address, 4);
                }
                let base = e.mem.alloc(256);
                e.mem.set_u32(0x011de134, base);
                e.mem.set_u32(0x011ddf38, base);
                e.mem.set_u32(0x011c3f2c, 0);
                let mut bytes = vec![0xa5; 256];
                bytes[12] = 0;
                for (offset, word) in [
                    (8, 0),
                    (0x24, base + 0x60),
                    (0x28, 64),
                    (0x2c, 5),
                    (0x30, 7),
                ] {
                    bytes[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
                }
                e.mem.write(base, &bytes);
                bytes[0x65..0x68].copy_from_slice(&encoded);
                bytes[0x68] = b'|';
                bytes[0x2c..0x30].copy_from_slice(&9u32.to_le_bytes());
                bytes[0x30..0x34].copy_from_slice(&8u32.to_le_bytes());
                e.call(0x00865db0, &[base + 0x20, id, unused]);
                assert_eq!(e.mem.bytes(base, 256), bytes);
            }
        }
    }

    #[test]
    fn numeric_form_writer_initializes_then_encodes_and_appends_exact_three_bytes() {
        for id in [0, 0x1234_5678] {
            let mut e = Engine::new();
            let buffer = e.new_object::<BGSSaveGameBuffer>();
            e.register_double(0x0084eaf0, |e, a| {
                assert_eq!(a.len(), 1);
                e.mem.write(a[0], &[0, 0, 0, 0xa5]);
                Ret::default()
            });
            e.register_double(0x00853570, move |e, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[1], id);
                assert_eq!(e.mem.bytes(a[0], 4), [0, 0, 0, 0xa5]);
                e.mem.write(a[0], &[0x12, 0x34, 0x56]);
                Ret::default()
            });
            e.register_double(0x00865570, move |e, a| {
                assert_eq!(a.len(), 4);
                assert_eq!(a[0], buffer.addr());
                assert_eq!(&a[2..], &[3, 0]);
                assert_eq!(e.mem.bytes(a[1], 4), [0x12, 0x34, 0x56, 0xa5]);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            e.call(0x00865db0, &args![buffer, id, u32::MAX]);
            let calls = e
                .call_log
                .unwrap()
                .into_iter()
                .map(|(address, _)| address)
                .collect::<Vec<_>>();
            assert_eq!(calls, [0x00865db0, 0x0084eaf0, 0x00853570, 0x00865570]);
        }
    }
    fn fixture() -> (Engine, Ptr<BGSSaveGameBuffer>, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011de134, 4);
        let this = e.new_object::<BGSSaveGameBuffer>();
        construct(&mut e, this);
        let bytes = e.mem.alloc(64);
        e.mem.write(bytes, &[0xcc; 64]);
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(bytes));
        e.set(this, BGSSaveGameBuffer::iBufferSize, 64);
        let mode = e.mem.alloc(16);
        e.mem.set_u8(mode + 12, 0);
        e.set_global(0x011de134, mode);
        // Generated copy dependency; these tests do not claim full save-game
        // startup coverage. Checked movement uses the real translated wrapper.
        e.register_double(0x00401460, |e, a| {
            assert_eq!(a.len(), 3);
            let data = e.mem.bytes(a[1], a[2]).to_vec();
            e.mem.write(a[0], &data);
            a[0].into_ret()
        });
        (e, this, bytes, mode)
    }

    #[test]
    fn save_form_id_reads_form_once_and_writes_three_encoded_bytes_for_null_too() {
        let (mut e, this, bytes, _) = fixture();
        let form = e.mem.alloc(16);
        e.mem.set_u32(form + 12, 0x12345678);
        e.register_double(0x00853570, move |e, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(e.mem.bytes(a[0], 3), [0, 0, 0]);
            if a[1] == 0x12345678 {
                e.mem.set_u32(form + 12, 0xaabbccdd);
                e.mem.write(a[0], &[0x31, 0x42, 0x53]);
            } else {
                assert_eq!(a[1], 0);
                e.mem.write(a[0], &[0, 0, 7]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00865df0, &args![this, form, u32::MAX]);
        e.call(0x00865df0, &args![this, 0u32, 0x1234u32]);
        assert_eq!(
            e.mem.bytes(bytes, 8),
            [0x31, 0x42, 0x53, b'|', 0, 0, 7, b'|']
        );
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 8);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 2);
        let reads: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0084e3a0)
            .map(|(_, arguments)| arguments.clone())
            .collect();
        assert_eq!(reads, [vec![form]]);
    }

    #[test]
    fn append_wrappers_and_positioned_overwrite_preserve_cursor_and_overhead() {
        let (mut e, this, bytes, _) = fixture();
        let source = e.mem.alloc(3);
        e.mem.write(source, b"abc");
        e.call(0x00865e50, &args![this, source, 2u32, 0xffffffffu32]);
        assert_eq!(e.mem.bytes(bytes, 4), b"ab|\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        e.call(0x00865ce0, &args![this, source + 2, 1u32, 0u32, 0u8]);
        assert_eq!(e.mem.bytes(bytes, 4), b"c||\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        e.call(0x00865570, &args![this, source, 1u32, 0x101u32]);
        assert_eq!(e.mem.bytes(bytes, 5), b"c||a\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 4);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
    }

    #[test]
    fn positioned_write_reloads_mode_buffer_cursor_and_overhead_after_copy() {
        let (mut e, this, bytes, first_mode) = fixture();
        let moved = e.mem.alloc(64);
        e.mem.write(moved, &[0xdd; 64]);
        let second_mode = e.mem.alloc(16);
        let source = e.mem.alloc(2);
        e.mem.write(source, b"xy");
        e.register_double(0x00424940, move |_, a| {
            assert!(a == [first_mode] || a == [second_mode]);
            0u32.into_ret()
        });
        e.register_double(0x00401460, move |e, a| {
            assert_eq!(a, &[bytes, source, 2]);
            e.mem.write(bytes, b"xy");
            e.set_global(0x011de134, second_mode);
            e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(moved));
            e.set(this, BGSSaveGameBuffer::iBufferPosition, 0);
            e.set(this, BGSSaveGameBuffer::iDebugOverhead, u32::MAX);
            Ret::default()
        });
        e.set(this, BGSSaveGameBuffer::iBufferPosition, 55);
        e.call_log = Some(vec![]);
        e.call(0x00865ce0, &args![this, source, 2u32, 0u32, 0u8]);
        assert_eq!(e.mem.bytes(bytes, 3), b"xy\xcc");
        assert_eq!(e.mem.bytes(moved, 3), b"\xdd\xdd|");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 0);
        let calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00424940)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(calls, vec![vec![first_mode], vec![second_mode]]);
    }

    #[test]
    fn sizing_mode_grows_without_touching_source_or_buffer_and_wraps_end() {
        let (mut e, this, bytes, mode) = fixture();
        e.mem.set_u8(mode + 12, 1);
        e.set(this, BGSSaveGameBuffer::iBufferSize, 0);
        e.call(
            0x00865ce0,
            &args![this, 0xdead0000u32, 0x10000u32, 0u32, 0u8],
        );
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferSize), 0x20000);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 0x10001);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        assert_eq!(e.mem.bytes(bytes, 64), [0xcc; 64]);
        e.call(0x00865ce0, &args![this, 0u32, 2u32, u32::MAX, 1u8]);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 0x10001);
    }

    #[test]
    fn variable_value_boundaries_shift_existing_payload_and_preserve_overhead() {
        for (value, encoded) in [
            (0u32, vec![0]),
            (0x3f, vec![0xfc]),
            (0x40, vec![1, 1]),
            (0x3fff, vec![0xfd, 0xff]),
            (0x4000, vec![2, 0, 1, 0]),
            (0x3fffffff, vec![0xfe, 0xff, 0xff, 0xff]),
        ] {
            let (mut e, this, bytes, _) = fixture();
            let start = e.call(0x00865f20, &args![this]).u32();
            assert_eq!(start, 0);
            assert_eq!(e.mem.bytes(bytes, 2), b"#|");
            let source = e.mem.alloc(2);
            e.mem.write(source, b"ab");
            e.call(0x00865570, &args![this, source, 2u32, 1u8]);
            e.call(0x00865ff0, &args![this, value, start]);
            let mut expected = encoded;
            expected.extend_from_slice(b"|ab");
            assert_eq!(e.mem.bytes(bytes, expected.len() as u32), expected);
            assert_eq!(
                e.get(this, BGSSaveGameBuffer::iBufferPosition),
                expected.len() as u32
            );
            assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        }
    }

    #[test]
    fn invalid_variable_value_reports_exact_format_and_value_without_writing() {
        for value in [0x40000000u32, u32::MAX] {
            let (mut e, this, bytes, _) = fixture();
            e.register_double(0x00845cc0, move |_, a| {
                assert_eq!(a, &[0x0107ee00, value]);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00865ff0, &args![this, value, 0u32]);
            assert_eq!(e.mem.bytes(bytes, 64), [0xcc; 64]);
            assert_eq!(
                e.call_log
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|(address, _)| *address == 0x00845cc0)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn insertion_rejects_nonpositive_signed_end_before_callbacks() {
        for (position, extra) in [(0u32, 0u32), (0x7fffffff, 1), (u32::MAX, 1)] {
            let (mut e, this, _, _) = fixture();
            e.set(this, BGSSaveGameBuffer::iBufferPosition, position);
            e.register_double(0x00424940, |_, _| panic!("early exit must not query mode"));
            e.call(0x00866140, &args![this, 0u32, extra]);
            assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), position);
        }
    }

    #[test]
    fn insertion_uses_fresh_fields_but_commits_end_captured_before_mode_callback() {
        let (mut e, this, _, _) = fixture();
        let moved = e.mem.alloc(20);
        e.mem.write(moved, b"abcdefghijklmnopqrst");
        e.set(this, BGSSaveGameBuffer::iBufferPosition, 6);
        e.register_double(0x00424940, move |e, _| {
            e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(moved));
            e.set(this, BGSSaveGameBuffer::iBufferSize, 20);
            e.set(this, BGSSaveGameBuffer::iBufferPosition, 10);
            0u32.into_ret()
        });
        e.register_double(0x00ec7ce3, move |_, a| {
            assert_eq!(a, &[moved + 5, 15, moved + 2, 8]);
            0u32.into_ret()
        });
        e.call(0x00866140, &args![this, 2u32, 3u32]);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 9);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Where the append double records what it is given: a length word,
    /// then the appended bytes.
    const APPENDED: u32 = 0x0310_0000;

    fn record_appends(e: &mut Engine) {
        e.map(APPENDED, 0x100);
        e.register(APPEND, |e, a| {
            assert_eq!(a[3], 0);
            let used = e.mem.u32(APPENDED);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(APPENDED + 4 + used, &bytes);
            e.mem.set_u32(APPENDED, used + a[2]);
            Ret::default()
        });
    }

    fn appended(e: &Engine) -> Vec<u8> {
        e.mem.bytes(APPENDED + 4, e.mem.u32(APPENDED))
    }

    #[test]
    fn destructor_sets_the_vtable_and_frees_the_buffer() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(0x0abc_0000));
        e.call_log = Some(vec![]);
        e.call(0x0086_5c10, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), SAVE_GAME_BUFFER_VTABLE);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x0abc_0000]]);
    }

    fn mode(e: &mut Engine, sizing_only: bool) {
        e.map(SAVE_LOAD_MODE, 4);
        e.mem.set_u32(SAVE_LOAD_MODE, 0x0123_0000);
        if sizing_only {
            e.register(MODE_SIZING_ONLY, |_, a| {
                assert_eq!(a[0], 0x0123_0000);
                Ret {
                    eax: 0xffff_ff01,
                    ..Ret::default()
                }
            });
        } else {
            e.register(MODE_SIZING_ONLY, |_, _| Ret {
                eax: 0xffff_ff00,
                ..Ret::default()
            });
        }
        e.register(SAVE_LOAD_FILE_WRITE, |_, _| Ret::default());
        e.register(SAVE_DATA_TO_FILE, |_, _| Ret::default());
    }

    #[test]
    fn save_writes_the_position_then_that_many_buffer_bytes() {
        let mut e = Engine::new();
        mode(&mut e, false);
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(0x0abc_0000));
        e.set(this, BGSSaveGameBuffer::iBufferPosition, 0x30);
        e.call_log = Some(vec![]);
        e.call(0x0086_5c40, &args![this, 0x0777_0000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA_TO_FILE),
            vec![vec![0x0777_0000, this.addr() + 0x0c, 4]]
        );
        assert_eq!(
            calls_to(&e, SAVE_LOAD_FILE_WRITE),
            vec![vec![0x0777_0000, 0x0abc_0000, 0x30]]
        );
    }

    #[test]
    fn save_ov2_writes_only_with_a_buffer_and_a_size_or_null_while_sizing() {
        let mut e = Engine::new();
        mode(&mut e, false);
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.call_log = Some(vec![]);
        // No buffer: nothing written.
        e.call(0x0086_5c80, &args![this, 0x0777_0000u32, 8u32]);
        // A buffer but no size: nothing written.
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(0x0abc_0000));
        e.call(0x0086_5c80, &args![this, 0x0777_0000u32, 0u32]);
        assert!(calls_to(&e, SAVE_LOAD_FILE_WRITE).is_empty());
        e.call(0x0086_5c80, &args![this, 0x0777_0000u32, 8u32]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_FILE_WRITE),
            vec![vec![0x0777_0000, 0x0abc_0000, 8]]
        );

        // Sizing only: a null source, a zero size writes nothing.
        let mut e = Engine::new();
        mode(&mut e, true);
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(0x0abc_0000));
        e.call_log = Some(vec![]);
        e.call(0x0086_5c80, &args![this, 0x0777_0000u32, 0u32]);
        assert!(calls_to(&e, SAVE_LOAD_FILE_WRITE).is_empty());
        e.call(0x0086_5c80, &args![this, 0x0777_0000u32, 8u32]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_FILE_WRITE),
            vec![vec![0x0777_0000, 0, 8]]
        );
    }

    #[test]
    fn save_string_appends_the_16_bit_length_then_the_text() {
        let mut e = Engine::new();
        record_appends(&mut e);
        e.register(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        let this = e.new_object::<BGSSaveGameBuffer>();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abc");
        e.call(0x0086_5e70, &args![this, text, 0xdead_beefu32]);
        assert_eq!(appended(&e), b"\x03\x00abc");

        // Null and empty strings give the zero count only.
        let mut e = Engine::new();
        record_appends(&mut e);
        e.register(STRLEN, |_, _| Ret::default());
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.call_log = Some(vec![]);
        e.call(0x0086_5e70, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, STRLEN).is_empty());
        let empty = e.mem.alloc(4);
        e.call(0x0086_5e70, &args![this, empty, 0u32]);
        assert_eq!(appended(&e), [0, 0, 0, 0]);
        assert_eq!(calls_to(&e, APPEND).len(), 2);

        // A length of 0x10000 truncates to a zero count: no text follows.
        let mut e = Engine::new();
        record_appends(&mut e);
        e.register(STRLEN, |_, _| Ret {
            eax: 0x0001_0000,
            ..Ret::default()
        });
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.call(0x0086_5e70, &args![this, text, 0u32]);
        assert_eq!(appended(&e), [0, 0]);
    }

    #[test]
    fn variable_sized_value_uses_one_two_or_four_tagged_bytes() {
        for (value, bytes) in [
            (0u32, vec![0u8]),
            (0x3f, vec![0xfc]),
            (0x40, vec![0x01, 0x01]),
            (0x3fff, vec![0xfd, 0xff]),
            (0x4000, vec![0x02, 0x00, 0x01, 0x00]),
            (0x3fff_ffff, vec![0xfe, 0xff, 0xff, 0xff]),
        ] {
            let mut e = Engine::new();
            record_appends(&mut e);
            let this = e.new_object::<BGSSaveGameBuffer>();
            e.call(0x0086_5f60, &args![this, value]);
            assert_eq!(appended(&e), bytes, "{value:#x}");
        }
        let mut e = Engine::new();
        record_appends(&mut e);
        e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.call_log = Some(vec![]);
        e.call(0x0086_5f60, &args![this, 0x4000_0000u32]);
        assert!(appended(&e).is_empty());
        assert_eq!(
            calls_to(&e, SAVE_GAME_WARNING),
            vec![vec![VALUE_TOO_LARGE_FORMAT, 0x4000_0000]]
        );
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn save_data_saves_the_rotation_as_a_quaternion() {
        let mut e = Engine::new();
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for i in 0..0x10 {
                e.mem.set_u8(a[0] + i, (a[1] as u8).wrapping_add(i as u8));
            }
            Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(0x0086_5570, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2]), a[3]));
            Ret::default()
        });
        let this = e.new_object::<BGSSaveGameBuffer>();
        e.call(0x0086_5ee0, &args![this, 0x10u32, 7u32, 9u32]);
        let expected: Vec<u8> = (0x10..0x20).collect();
        assert_eq!(*seen.borrow(), vec![(this.addr(), expected, 0)]);
    }
}

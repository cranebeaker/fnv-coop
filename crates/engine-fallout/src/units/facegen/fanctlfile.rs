//! `FaceGen/fanctlfile.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Reading FaceGen's control file (`.ctl`, id `"FRCTL001"`) and the two binary-file
//! destructors it inlines: a header of six words (two counts and a 2x2 table of counts), the
//! 2x2 groups of linear controls (`00aaac10`) and the control groups
//! (`FanGenCtlsGrpC::readFromFile`, `00aaaf10`).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    string_chars, BSFaceGenBinaryFile, StringArg, INVALID_PARAMETER, MEMORY_MANAGER_FREE,
    MEMORY_MANAGER_OBJECT, STRING_LENGTH, STRING_NPOS, STRING_TIDY, VECTOR_FIRST, VECTOR_LAST,
};

/// `FutBinaryFileC`'s and `BSFaceGenBinaryFile`'s vtables (RTTI).
const FUT_BINARY_FILE_VTABLE: u32 = 0x010a_2c20;
const BS_FACE_GEN_BINARY_FILE_VTABLE: u32 = 0x010a_2c38;
/// `FutBinaryFileC::close` (`00ac5430`) and the game file's close (`00ac5c00`), both
/// `thiscall` (unit `binaryfile`).
const FUT_BINARY_FILE_CLOSE: u32 = 0x00ac_5430;
const CLOSE_GAME_FILE: u32 = 0x00ac_5c00;
/// `BSFaceGenBinaryFile`'s constructor (`00ac5490`, the id by value), `openForRead`
/// (`00ac5540`, the name by value and a byte) and `readHeader` (`00ac5e00`: buffer, size,
/// count), unit `binaryfile`.
const BINARY_FILE_CONSTRUCT: u32 = 0x00ac_5490;
const BINARY_FILE_OPEN_FOR_READ: u32 = 0x00ac_5540;
const BINARY_FILE_READ_HEADER: u32 = 0x00ac_5e00;
/// The string from a C string (`0044a4c0`, `thiscall`: the characters) and the string's
/// assign from part of another (`0044a580`, `thiscall`: source, offset, count).
const STRING_FROM_C: u32 = 0x0044_a4c0;
const STRING_ASSIGN: u32 = 0x0044_a580;
/// The control file's id, `"FRCTL001"`.
const CONTROL_FILE_ID: u32 = 0x010a_2c10;
/// The control groups' reader (`FanGenCtlsGrpC::readFromFile`, `00aaaf10`, `thiscall`: the
/// file and the 2x2 count table), unit `fancontrols`.
const CONTROL_GROUPS_READ: u32 = 0x00aa_af10;
/// The binary file's `read` (vtable slot +0x10: buffer, element size, count).
const FILE_READ: u32 = 0x10;
/// The control vector's resize (`00aadde0`, `thiscall`, the count), the float vector's
/// resize-with-value (`0060b410`, `thiscall`: count, value) and its element-pointer getter
/// (`0060b370`, `thiscall`: an out word and the index; answers the out word's address), the
/// string's erase (`0044a690`, `thiscall`: offset, count) and its append of copies
/// (`00aadeb0`, `thiscall`: count, character).
const CONTROL_VECTOR_RESIZE: u32 = 0x00aa_dde0;
const FLOAT_VECTOR_RESIZE: u32 = 0x0060_b410;
const FLOAT_VECTOR_POINTER: u32 = 0x0060_b370;
const STRING_ERASE: u32 = 0x0044_a690;
const STRING_APPEND_COPIES: u32 = 0x00aa_deb0;

/// A linear control (0x3C bytes): a `FR2MatrixVTC<float>` (its row and column counts at
/// +0x18, +0x1C) and its label string at +0x20 (characters at +0x24, capacity at +0x38).
const CONTROL_SIZE: u32 = 0x3c;
const CONTROL_ROWS: u32 = 0x18;
const CONTROL_COLUMNS: u32 = 0x1c;
const CONTROL_LABEL: u32 = 0x20;

// Translated from 00aaa720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an empty `thiscall` (the control file reader calls it on its
/// header).
pub fn fn_00aaa720(_e: &mut Engine, _this: Ptr) {}

/// The inlined `BSFaceGenBinaryFile` destructor at the end of `fanReadCtlFile`: its vtable,
/// the game file's close (`00ac5c00`), then `FutBinaryFileC`'s destructor (`00aaaa80`).
fn destroy_binary_file(e: &mut Engine, file: Ptr<BSFaceGenBinaryFile>) {
    e.mem.set_u32(file.addr(), BS_FACE_GEN_BINARY_FILE_VTABLE);
    e.call(CLOSE_GAME_FILE, &args![file]);
    fn_00aaaa80(e, file);
}

// Translated from 00aaa7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaa7d0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// `fanReadCtlFile` (Xbox PDB; `cdecl`, six words: the file name string, two out words, the
/// 2x2 out table, the linear control groups and the control groups). Makes a
/// `BSFaceGenBinaryFile` with the id `"FRCTL001"` and opens the name (`openForRead`, the byte
/// 1); reads the 0x18-byte header (`readHeader(header, 1, 0x18)`), passes it to `00aaa720`, and
/// stores its first two words in the out words and the 2x2 table after them in `table`. Then
/// the linear controls (`00aaac10` on `linear`) and the control groups
/// (`FanGenCtlsGrpC::readFromFile` on `groups`) are read with the file and `table`. Every way
/// out destroys the file; the answer is whether all of it was read.
pub fn fan_read_ctl_file(
    e: &mut Engine,
    name: u32,
    first: u32,
    second: u32,
    table: u32,
    linear: u32,
    groups: u32,
) -> bool {
    e.with_stack(0x48 + 0x18, |e, frame| {
        let file: Ptr<BSFaceGenBinaryFile> = Ptr::new(frame.addr());
        let header = frame.addr() + 0x48;
        let id = e.with_stack(0x1c, |e, id| {
            e.call(STRING_FROM_C, &args![id, CONTROL_FILE_ID]);
            StringArg::load(e, id.addr())
        });
        e.call(BINARY_FILE_CONSTRUCT, &args![file, id]);
        let copy = e.with_stack(0x1c, |e, copy| {
            let copy = copy.addr();
            e.call(STRING_TIDY, &args![copy, 0u32, 0u32]);
            let npos: u32 = e.global(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![copy, name, 0u32, npos]);
            StringArg::load(e, copy)
        });
        let opened = e
            .call(BINARY_FILE_OPEN_FOR_READ, &args![file, copy, 1u32])
            .u8();
        if opened == 0 {
            destroy_binary_file(e, file);
            return false;
        }
        if e.call(BINARY_FILE_READ_HEADER, &args![file, header, 1u32, 0x18u32])
            .u8()
            == 0
        {
            destroy_binary_file(e, file);
            return false;
        }
        fn_00aaa720(e, Ptr::new(header));
        let word = e.mem.u32(header);
        e.mem.set_u32(first, word);
        let word = e.mem.u32(header + 4);
        e.mem.set_u32(second, word);
        for row in 0..2u32 {
            for column in 0..2u32 {
                let value = e.mem.u32(header + 8 + row * 8 + column * 4);
                e.mem.set_u32(table + row * 8 + column * 4, value);
            }
        }
        if !fn_00aaac10(e, Ptr::new(linear), file, table) {
            destroy_binary_file(e, file);
            return false;
        }
        let read = e
            .call(CONTROL_GROUPS_READ, &args![groups, file, table])
            .u8()
            != 0;
        destroy_binary_file(e, file);
        read
    })
}

// Translated from 00aaaa80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaaa80 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FutBinaryFileC`'s destructor: sets its vtable, closes the file
/// (`FutBinaryFileC::close`) and destroys the file id (+0x20) and the name (+0x04).
pub fn fn_00aaaa80(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>) {
    e.mem.set_u32(this.addr(), FUT_BINARY_FILE_VTABLE);
    e.call(FUT_BINARY_FILE_CLOSE, &args![this]);
    e.call(STRING_TIDY, &args![this.addr() + 0x20, 1u32, 0u32]);
    e.call(STRING_TIDY, &args![this.addr() + 4, 1u32, 0u32]);
}

// Translated from 00aaab30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaab30 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `BSFaceGenBinaryFile`'s destructor: sets its vtable, closes the
/// game file (`00ac5c00`) and runs `FutBinaryFileC`'s destructor (`00aaaa80`).
pub fn fn_00aaab30(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>) {
    e.mem.set_u32(this.addr(), BS_FACE_GEN_BINARY_FILE_VTABLE);
    e.call(CLOSE_GAME_FILE, &args![this]);
    fn_00aaaa80(e, this);
}

// Translated from 00aaab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::` scalar deleting destructor (Xbox PDB; `thiscall`, the flags):
/// `FutBinaryFileC`'s destructor (`00aaaa80`), then, with flag bit 0, the memory manager's free
/// of the object. Answers `this`.
pub fn fut_binary_file_c_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    flags: u32,
) -> Ptr<BSFaceGenBinaryFile> {
    fn_00aaaa80(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00aaab90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaab90 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `BSFaceGenBinaryFile::` scalar deleting destructor (Xbox PDB; `thiscall`, the flags):
/// sets `BSFaceGenBinaryFile`'s vtable, closes the game file (`00ac5c00`), runs
/// `FutBinaryFileC`'s destructor (`00aaaa80`), then, with flag bit 0, the memory manager's free
/// of the object. Answers `this`.
pub fn bs_face_gen_binary_file_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    flags: u32,
) -> Ptr<BSFaceGenBinaryFile> {
    e.mem.set_u32(this.addr(), BS_FACE_GEN_BINARY_FILE_VTABLE);
    e.call(CLOSE_GAME_FILE, &args![this]);
    fn_00aaaa80(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

/// The `index`-th control of the vector at `vector`, after the code's bounds check
/// (`index < (last - first) / 0x3C`, a signed division compared unsigned).
fn checked_control(e: &mut Engine, vector: u32, index: u32) -> u32 {
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let span = e.mem.u32(vector + VECTOR_LAST).wrapping_sub(first) as i32;
    if index >= (span / CONTROL_SIZE as i32) as u32 {
        e.call(INVALID_PARAMETER, &[]);
    }
    index
        .wrapping_mul(CONTROL_SIZE)
        .wrapping_add(e.mem.u32(vector + VECTOR_FIRST))
}

// Translated from 00aaac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`: the file and the 2x2 count table): reads the 2x2
/// groups of linear controls. `this` holds 2x2 vectors of 0x3C-byte controls (row `i` at
/// `i * 0x30`, column `j` at `j * 0x18`). For each group: a word count is read and the vector
/// resized to it (`00aadde0`); each control becomes a 1 x `table[i][j]` matrix (counts at
/// +0x18, +0x1C) whose floats are resized to that many zeros (`0060b410`) and read
/// (`table[i][j]` floats), then its label's length is read, the label cut (`0044a690`) or
/// padded with zero characters (`00aadeb0`) to it and its characters read. Every access to a
/// control is bounds-checked (`_invalid_parameter_noinfo` on failure). Any failed read answers
/// false.
// The label's "length >= 0" check is an unsigned compare against 0: it never fails, so its
// `_invalid_parameter_noinfo` call is never reached.
#[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
pub fn fn_00aaac10(e: &mut Engine, this: Ptr, file: Ptr<BSFaceGenBinaryFile>, table: u32) -> bool {
    // The out local `0060b370` fills is its 8-byte element iterator (`[EBP-0x1C]`, two words
    // stored by `004b0680`), so the frame holds 0x10 bytes.
    e.with_stack(0x10, |e, locals| {
        let count_slot = locals.addr();
        let length_slot = locals.addr() + 4;
        let out_slot = locals.addr() + 8;
        let read = |e: &mut Engine, buffer: u32, size: u32, count: u32| {
            e.vcall(file.addr(), FILE_READ, &args![buffer, size, count])
                .u8()
                != 0
        };
        for row in 0..2u32 {
            for column in 0..2u32 {
                let vector = this.addr() + row * 0x30 + column * 0x18;
                let cell = table + row * 8 + column * 4;
                if !read(e, count_slot, 4, 1) {
                    return false;
                }
                let count = e.mem.u32(count_slot);
                e.call(CONTROL_VECTOR_RESIZE, &args![vector, count]);
                let mut index = 0u32;
                while index < e.mem.u32(count_slot) {
                    let control = checked_control(e, vector, index);
                    let columns = e.mem.u32(cell);
                    e.mem.set_u32(control + CONTROL_ROWS, 1);
                    e.mem.set_u32(control + CONTROL_COLUMNS, columns);
                    e.call(
                        FLOAT_VECTOR_RESIZE,
                        &args![control, columns, x87_store32(0.0)],
                    );
                    let control = checked_control(e, vector, index);
                    let pointer = e
                        .call(FLOAT_VECTOR_POINTER, &args![control, out_slot, 0u32])
                        .u32();
                    let floats = e.mem.u32(pointer);
                    let columns = e.mem.u32(cell);
                    if !read(e, floats, 4, columns) {
                        return false;
                    }
                    if !read(e, length_slot, 4, 1) {
                        return false;
                    }
                    let control = checked_control(e, vector, index);
                    let label = control + CONTROL_LABEL;
                    let length = e.mem.u32(length_slot);
                    if length <= e.mem.u32(label + STRING_LENGTH) {
                        let npos: u32 = e.global(STRING_NPOS);
                        e.call(STRING_ERASE, &args![label, length, npos]);
                    } else {
                        let missing = length.wrapping_sub(e.mem.u32(label + STRING_LENGTH));
                        e.call(STRING_APPEND_COPIES, &args![label, missing, 0u32]);
                    }
                    let control = checked_control(e, vector, index);
                    let label = control + CONTROL_LABEL;
                    if e.mem.u32(label + STRING_LENGTH) < 0 {
                        e.call(INVALID_PARAMETER, &[]);
                    }
                    let chars = string_chars(e, label);
                    let length = e.mem.u32(length_slot);
                    if !read(e, chars, 1, length) {
                        return false;
                    }
                    index = index.wrapping_add(1);
                }
            }
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_a720, fn_00aaa720(Ptr)),
        entry!(
            0x00aa_a7d0,
            fan_read_ctl_file(u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00aa_aa80, fn_00aaaa80(Ptr<BSFaceGenBinaryFile>)),
        entry!(
            0x00aa_ab00,
            fut_binary_file_c_scalar_deleting_destructor(
                Ptr<BSFaceGenBinaryFile>,
                u32,
            ) -> Ptr<BSFaceGenBinaryFile>
        ),
        entry!(0x00aa_ab30, fn_00aaab30(Ptr<BSFaceGenBinaryFile>)),
        entry!(
            0x00aa_ab90,
            bs_face_gen_binary_file_scalar_deleting_destructor(
                Ptr<BSFaceGenBinaryFile>,
                u32,
            )
                -> Ptr<BSFaceGenBinaryFile>
        ),
        entry!(
            0x00aa_ac10,
            fn_00aaac10(Ptr, Ptr<BSFaceGenBinaryFile>, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn empty_call_does_nothing() {
        let mut e = Engine::new();
        let header = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00aa_a720, &args![header]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn destructors_close_and_destroy_strings() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[FUT_BINARY_FILE_CLOSE, CLOSE_GAME_FILE, STRING_TIDY],
        );
        let file: Ptr<BSFaceGenBinaryFile> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00aa_aa80, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(file.addr()), FUT_BINARY_FILE_VTABLE);
        assert_eq!(
            calls_to(&log, FUT_BINARY_FILE_CLOSE),
            vec![vec![file.addr()]]
        );
        assert_eq!(
            calls_to(&log, STRING_TIDY),
            vec![vec![file.addr() + 0x20, 1, 0], vec![file.addr() + 4, 1, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa_ab30, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE), vec![vec![file.addr()]]);
        assert_eq!(calls_to(&log, FUT_BINARY_FILE_CLOSE).len(), 1);
        assert_eq!(e.mem.u32(file.addr()), FUT_BINARY_FILE_VTABLE);
    }

    #[test]
    fn scalar_deleting_destructors_free_only_with_flag_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FUT_BINARY_FILE_CLOSE,
                CLOSE_GAME_FILE,
                STRING_TIDY,
                MEMORY_MANAGER_FREE,
            ],
        );
        let file: Ptr<BSFaceGenBinaryFile> = e.new_object();
        for (addr, closes_game_file) in [(0x00aa_ab00u32, 0usize), (0x00aa_ab90, 1)] {
            for flags in [0u32, 1, 2, 3] {
                e.call_log = Some(vec![]);
                let back = e.call(addr, &args![file, flags]).u32();
                let log = e.call_log.take().unwrap();
                assert_eq!(back, file.addr());
                assert_eq!(e.mem.u32(file.addr()), FUT_BINARY_FILE_VTABLE);
                assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), closes_game_file);
                assert_eq!(calls_to(&log, FUT_BINARY_FILE_CLOSE).len(), 1);
                let frees = calls_to(&log, MEMORY_MANAGER_FREE);
                if flags & 1 != 0 {
                    assert_eq!(frees, vec![vec![MEMORY_MANAGER_OBJECT, file.addr()]]);
                } else {
                    assert!(frees.is_empty());
                }
            }
        }
    }

    /// An engine whose file `read` (slot +0x10) serves the words of `data` in order (each read
    /// takes `size * count` bytes).
    fn reading_engine(data: Vec<u8>) -> (Engine, Ptr<BSFaceGenBinaryFile>) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        let mut position = 0usize;
        e.register_double(0x0040_7000, move |e, a| {
            let bytes = (a[2] * a[3]) as usize;
            if position + bytes > data.len() {
                return Ret::default();
            }
            e.mem.write(a[1], &data[position..position + bytes]);
            position += bytes;
            1u32.into_ret()
        });
        e.put_vtable(0x0090_9000, &[0, 0, 0, 0, 0x0040_7000]);
        let file: Ptr<BSFaceGenBinaryFile> = e.new_object();
        e.mem.set_u32(file.addr(), 0x0090_9000);
        quiet(
            &mut e,
            &[INVALID_PARAMETER, STRING_ERASE, STRING_APPEND_COPIES],
        );
        // The control vector resize: count controls of 0x3C bytes, each with a float block.
        e.register(CONTROL_VECTOR_RESIZE, |e, a| {
            let block = e.mem.alloc((a[1] * CONTROL_SIZE).max(4));
            for i in 0..a[1] {
                let label = block + i * CONTROL_SIZE + CONTROL_LABEL;
                e.mem.set_u32(label + STRING_LENGTH, 2);
                e.mem.set_u32(label + 0x18, 15);
            }
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem
                .set_u32(a[0] + VECTOR_LAST, block + a[1] * CONTROL_SIZE);
            Ret::default()
        });
        e.register(FLOAT_VECTOR_RESIZE, |e, a| {
            let block = e.mem.alloc((a[1] * 4).max(4));
            e.mem.set_u32(a[0] + 0xc, block);
            Ret::default()
        });
        e.register(FLOAT_VECTOR_POINTER, |e, a| {
            let block = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], block + 4 * a[2]);
            a[1].into_ret()
        });
        (e, file)
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    #[test]
    fn linear_controls_read_counts_floats_and_labels() {
        // Groups (0,0): one control of 2 floats and label "ab"; (0,1), (1,0), (1,1): none.
        let mut data = words(&[1]);
        data.extend(words(&[1.5f32.to_bits(), 2.5f32.to_bits()]));
        data.extend(words(&[3]));
        data.extend(b"abc");
        data.extend(words(&[0, 0, 0]));
        let (mut e, file) = reading_engine(data);
        let this: Ptr = Ptr::new(e.mem.alloc(0x60));
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_ac10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let control = e.mem.u32(this.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(control + CONTROL_ROWS), 1);
        assert_eq!(e.mem.u32(control + CONTROL_COLUMNS), 2);
        assert_eq!(
            calls_to(&log, FLOAT_VECTOR_RESIZE),
            vec![vec![control, 2, 0]]
        );
        let floats = e.mem.u32(control + 0xc);
        assert_eq!(e.mem.f32(floats + 4), 2.5);
        // The label had 2 characters: one zero is appended, then 3 read.
        assert_eq!(
            calls_to(&log, STRING_APPEND_COPIES),
            vec![vec![control + CONTROL_LABEL, 1, 0]]
        );
        assert_eq!(e.mem.bytes(control + 0x24, 3), b"abc".to_vec());
        assert_eq!(calls_to(&log, CONTROL_VECTOR_RESIZE).len(), 4);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn linear_controls_cut_long_labels_and_fail_on_short_reads() {
        let mut data = words(&[1]);
        data.extend(words(&[7]));
        data.extend(words(&[1]));
        data.extend(b"z");
        let (mut e, file) = reading_engine(data);
        let this: Ptr = Ptr::new(e.mem.alloc(0x60));
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 1);
        e.call_log = Some(vec![]);
        // The data stops before the second group's count.
        assert!(!e.call(0x00aa_ac10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let control = e.mem.u32(this.addr() + VECTOR_FIRST);
        assert_eq!(
            calls_to(&log, STRING_ERASE),
            vec![vec![control + CONTROL_LABEL, 1, u32::MAX]]
        );
        assert_eq!(calls_to(&log, CONTROL_VECTOR_RESIZE).len(), 1);
    }

    fn reader_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        quiet(
            &mut e,
            &[
                STRING_FROM_C,
                STRING_TIDY,
                STRING_ASSIGN,
                BINARY_FILE_CONSTRUCT,
                CLOSE_GAME_FILE,
                FUT_BINARY_FILE_CLOSE,
            ],
        );
        e.register(BINARY_FILE_OPEN_FOR_READ, |_, _| 1u32.into_ret());
        e.register(BINARY_FILE_READ_HEADER, |e, a| {
            for i in 0..6 {
                e.mem.set_u32(a[1] + 4 * i, 10 + i);
            }
            1u32.into_ret()
        });
        e.register(CONTROL_GROUPS_READ, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn control_file_reader_reads_the_header_and_both_parts() {
        let mut e = reader_engine();
        let out = e.mem.alloc(0x20);
        let name = e.mem.alloc(0x1c);
        let linear = e.mem.alloc(0x60);
        let groups = e.mem.alloc(0x40);
        // No linear controls in any group.
        let mut data = Vec::new();
        for _ in 0..4 {
            data.extend(0u32.to_le_bytes());
        }
        let mut position = 0usize;
        e.register_double(0x0040_7100, move |e, a| {
            let bytes = (a[2] * a[3]) as usize;
            e.mem.write(a[1], &data[position..position + bytes]);
            position += bytes;
            1u32.into_ret()
        });
        e.register(CONTROL_VECTOR_RESIZE, |_, _| Ret::default());
        e.register_double(BINARY_FILE_CONSTRUCT, |e, a| {
            e.put_vtable(0x0090_a000, &[0, 0, 0, 0, 0x0040_7100]);
            e.mem.set_u32(a[0], 0x0090_a000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let ok = e
            .call(
                0x00aa_a7d0,
                &args![name, out, out + 4, out + 8, linear, groups],
            )
            .bool();
        let log = e.call_log.take().unwrap();
        assert!(ok);
        assert_eq!(calls_to(&log, STRING_FROM_C)[0][1], CONTROL_FILE_ID);
        let open = calls_to(&log, BINARY_FILE_OPEN_FOR_READ);
        assert_eq!(open[0].len(), 9);
        assert_eq!(open[0][8], 1);
        let file = open[0][0];
        assert_eq!(calls_to(&log, BINARY_FILE_READ_HEADER)[0][2..], [1, 0x18]);
        assert_eq!(e.mem.u32(out), 10);
        assert_eq!(e.mem.u32(out + 4), 11);
        assert_eq!(e.mem.u32(out + 8), 12);
        assert_eq!(e.mem.u32(out + 0x14), 15);
        assert_eq!(
            calls_to(&log, CONTROL_GROUPS_READ),
            vec![vec![groups, file, out + 8]]
        );
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE), vec![vec![file]]);
        assert_eq!(calls_to(&log, FUT_BINARY_FILE_CLOSE), vec![vec![file]]);
    }

    #[test]
    fn control_file_reader_stops_at_each_failure() {
        let mut e = reader_engine();
        let out = e.mem.alloc(0x20);
        e.register(BINARY_FILE_OPEN_FOR_READ, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00aa_a7d0, &args![0u32, out, out + 4, out + 8, 0u32, 0u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BINARY_FILE_READ_HEADER).is_empty());
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);

        e.register(BINARY_FILE_OPEN_FOR_READ, |_, _| 1u32.into_ret());
        e.register(BINARY_FILE_READ_HEADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00aa_a7d0, &args![0u32, out, out + 4, out + 8, 0u32, 0u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
        assert!(calls_to(&log, CONTROL_GROUPS_READ).is_empty());
    }
}

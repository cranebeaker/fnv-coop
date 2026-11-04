//! `FaceGen/egtfileio_sdk.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Reading FaceGen's texture morph file (`.egt`, id `"FREGT003"`): a 0x38-byte header (the row
//! count, the row width in bytes, the symmetric and asymmetric morph counts and a word handed
//! back), then for every morph its scale word and three planes of `rows` rows of `width`
//! bytes, stored bottom row first into rows padded to a multiple of 8 bytes.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    BSFaceGenBinaryFile, StringArg, STRING_ASSIGN, STRING_NPOS, STRING_TIDY, VECTOR_FIRST,
    VECTOR_LAST,
};
use crate::units::facegen::run_00ab3370::{fn_00ab3370, fn_00ab3a40};
use crate::units::facegen::run_00aba3c0::{
    advance_by_address, check, erase_by_address, resize_with_value, with_value_slot, ElementSize,
    ResizeShape, WordsArg,
};

/// `BSFaceGenBinaryFile`'s vtable (RTTI), set again by its inlined destructor.
const BS_FACE_GEN_BINARY_FILE_VTABLE: u32 = 0x010a_2c38;
/// The game file's close (`00ac5c00`, `thiscall`), `FutBinaryFileC`'s destructor
/// (`00aaaa80`, `thiscall`) and `BSFaceGenBinaryFile`'s destructor (`00aaab30`, `thiscall`).
const CLOSE_GAME_FILE: u32 = 0x00ac_5c00;
const FUT_BINARY_FILE_DESTROY: u32 = 0x00aa_aa80;
const BINARY_FILE_DESTROY: u32 = 0x00aa_ab30;
/// `BSFaceGenBinaryFile`'s constructor (`00ac5490`, the id by value), `openForRead`
/// (`00ac5540`, the name by value and a byte), `readHeader` (`00ac5e00`: buffer, 1, size) and
/// `read` (`00ac5cf0`: buffer, element size, count), all `thiscall`, unit `binaryfile`.
const BINARY_FILE_CONSTRUCT: u32 = 0x00ac_5490;
const BINARY_FILE_OPEN_FOR_READ: u32 = 0x00ac_5540;
const BINARY_FILE_READ_HEADER: u32 = 0x00ac_5e00;
const BINARY_FILE_READ: u32 = 0x00ac_5cf0;
/// The string from a C string (`0044a4c0`, `thiscall`: the characters).
const STRING_FROM_C: u32 = 0x0044_a4c0;
/// The texture morph file's id, `"FREGT003"`.
const TEXTURE_MORPH_FILE_ID: u32 = 0x010a_2d3c;
/// The texture morph vector's clear (`0065cef0`, `thiscall`) and the byte vector's
/// checked element address (`0065cc70`, `thiscall`: the index).
const MORPH_VECTOR_CLEAR: u32 = 0x0065_cef0;
const BYTE_ADDRESS: u32 = 0x0065_cc70;
/// The CRT's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element size,
/// count, constructor, destructor) and `eh vector destructor iterator` (`00ec5fce`, `stdcall`:
/// array, element size, count, destructor), with the byte vector destructor `0065f060`.
const EH_VECTOR_CONSTRUCTOR: u32 = 0x00ec_782f;
const EH_VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
const BYTE_VECTOR_DESTRUCT: u32 = 0x0065_f060;
/// The byte vector's iterator advance (`00ab43d0`) and erase (`0065f700`), and the texture
/// morph vector's (`00ab4310`, `0065d8e0`), all `thiscall`.
const ADVANCE_BYTES: u32 = 0x00ab_43d0;
const ERASE_BYTES: u32 = 0x0065_f700;
const ADVANCE_58: u32 = 0x00ab_4310;
const ERASE_58: u32 = 0x0065_d8e0;

/// A texture morph (0x58 bytes): its scale word, the row width, the row count and the row
/// padding at +0, +4, +8, +0xC, then three byte vectors (0x18 bytes each) from +0x10.
const MORPH_SIZE: u32 = 0x58;
const MORPH_PLANES: u32 = 0x10;
const PLANE_SIZE: u32 = 0x18;

// Translated from 00ab2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an empty `thiscall` (the texture morph file reader calls it on
/// its header).
pub fn fn_00ab2630(_e: &mut Engine, _this: Ptr) {}

/// The inlined `BSFaceGenBinaryFile` destructor at most ways out of `fffReadEgtFile`: its
/// vtable, the game file's close (`00ac5c00`), then `FutBinaryFileC`'s destructor
/// (`00aaaa80`).
fn destroy_binary_file(e: &mut Engine, file: u32) {
    e.mem.set_u32(file, BS_FACE_GEN_BINARY_FILE_VTABLE);
    e.call(CLOSE_GAME_FILE, &args![file]);
    e.call(FUT_BINARY_FILE_DESTROY, &args![file]);
}

/// The address of morph `index` of the morph vector at `vector`, after the debug check that
/// `index` is below its size (signed division by 0x58). Each check is followed by a jump back
/// that is never taken (`XOR`, `JNZ`: a `do ... while (0)`).
fn checked_morph(e: &mut Engine, vector: u32, index: u32) -> u32 {
    let span = e
        .mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST));
    let ok = index < ElementSize::Divide(MORPH_SIZE).count(span);
    check(e, ok);
    index
        .wrapping_mul(MORPH_SIZE)
        .wrapping_add(e.mem.u32(vector + VECTOR_FIRST))
}

/// The header's fields as the reader uses them.
struct Header {
    address: u32,
}

impl Header {
    fn rows(&self, e: &Engine) -> u32 {
        e.mem.u32(self.address)
    }
    fn width(&self, e: &Engine) -> u32 {
        e.mem.u32(self.address + 4)
    }
}

/// The sizes the reader derives from the header: the padding that rounds a row up to a
/// multiple of 8 bytes, the padded row and the plane.
struct Layout {
    padding: u32,
    stride: u32,
    plane: u32,
}

/// Where a morph's planes are read: whether the plane's start is found inline (the symmetric
/// morphs) or through `0065cc70` (the asymmetric ones), and whether a failed row read
/// destroys the file inline or through `00aaab30`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum MorphSet {
    Symmetric,
    Asymmetric,
}

/// Reads the morphs of one set (`count` at `count_at` in the header): every morph's scale
/// word, its width, row count and padding, and its three planes (each resized to the plane
/// size with zeros by `00ab2f50`, then read row by row from the last padded row back).
/// Answers false when a read failed (the file is then destroyed).
fn read_morphs(
    e: &mut Engine,
    file: u32,
    header: &Header,
    count_at: u32,
    vector: u32,
    layout: &Layout,
    set: MorphSet,
) -> bool {
    let file_ptr: Ptr<BSFaceGenBinaryFile> = Ptr::new(file);
    let one = 1u32;
    let mut index = 0u32;
    while index < e.mem.u32(count_at) {
        let morph = checked_morph(e, vector, index);
        if e.call(BINARY_FILE_READ, &args![file_ptr, morph, 4u32, 1u32])
            .u8()
            == 0
        {
            destroy_binary_file(e, file);
            return false;
        }
        let morph = checked_morph(e, vector, index);
        let width = header.width(e);
        e.mem.set_u32(morph + 4, width);
        let morph = checked_morph(e, vector, index);
        let rows = header.rows(e);
        e.mem.set_u32(morph + 8, rows);
        let morph = checked_morph(e, vector, index);
        e.mem.set_u32(morph + 0xc, layout.padding);
        for plane in 0..3u32 {
            let morph = checked_morph(e, vector, index);
            let bytes = morph + MORPH_PLANES + plane * PLANE_SIZE;
            fn_00ab2f50(e, Ptr::new(bytes), layout.plane, 0);
            let morph = checked_morph(e, vector, index);
            let offset = layout.plane.wrapping_sub(layout.stride);
            let bytes = morph + MORPH_PLANES + plane * PLANE_SIZE;
            let mut row = if set == MorphSet::Symmetric {
                let size = e
                    .mem
                    .u32(bytes + VECTOR_LAST)
                    .wrapping_sub(e.mem.u32(bytes + VECTOR_FIRST));
                // A failed check is followed by the same never-taken jump back.
                check(e, offset < size);
                e.mem.u32(bytes + VECTOR_FIRST).wrapping_add(offset)
            } else {
                e.call(BYTE_ADDRESS, &args![bytes, offset]).u32()
            };
            let mut line = 0u32;
            while line < header.rows(e) {
                let width = header.width(e);
                if e.call(BINARY_FILE_READ, &args![file_ptr, row, one, width])
                    .u8()
                    == 0
                {
                    if set == MorphSet::Symmetric {
                        destroy_binary_file(e, file);
                    } else {
                        e.call(BINARY_FILE_DESTROY, &args![file_ptr]);
                    }
                    return false;
                }
                let step = header.width(e).wrapping_add(layout.padding);
                row = row.wrapping_sub(step);
                line = line.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00ab2640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fffReadEgtFile` (Xbox PDB; `cdecl`, four words: the file name string, an out word, the
/// symmetric and the asymmetric texture morph vectors). Makes a `BSFaceGenBinaryFile` with the
/// id `"FREGT003"` and opens the name (`openForRead`, the byte 0); reads the 0x38-byte header
/// (`readHeader(header, 1, 0x38)`) and passes it to `00ab2630`; stores its word at +0x10 in the
/// out word and clears both vectors (`0065cef0`). The padding is `((width * 1 + 7) & ~7) / 1 -
/// width` (unsigned), the padded row `width + padding` and the plane `padded row * rows`. Each
/// vector is resized to its count (+8, +0xC) with a default morph built in the outgoing
/// argument area (`00ab2e40`, then `00ab3130`), then its morphs are read: the scale word
/// (`read(morph, 4, 1)`), the width, row count and padding stored, and each of the three planes
/// sized and read row by row (`read(row, 1, width)`), from offset `plane - padded row` down by a
/// padded row each time. The symmetric morphs check the plane offset inline, the asymmetric
/// ones get it from `0065cc70`; a failed row read of an asymmetric morph and the successful end
/// run `BSFaceGenBinaryFile`'s destructor (`00aaab30`), every other way out its inlined copy.
/// The answer is whether all of it was read.
// Translation incomplete: 00ab2640 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f890, state variable) and the /GS cookie check (00ec408c) are not translated.
pub fn fff_read_egt_file(
    e: &mut Engine,
    name: u32,
    out: u32,
    symmetric: Ptr,
    asymmetric: Ptr,
) -> bool {
    e.with_stack(0x48 + 0x38, |e, frame| {
        let file = frame.addr();
        let header = Header {
            address: file + 0x48,
        };
        let id = e.with_stack(0x1c, |e, id| {
            e.call(STRING_FROM_C, &args![id, TEXTURE_MORPH_FILE_ID]);
            StringArg::load(e, id.addr())
        });
        let file_ptr: Ptr<BSFaceGenBinaryFile> = Ptr::new(file);
        e.call(BINARY_FILE_CONSTRUCT, &args![file_ptr, id]);
        let copy = e.with_stack(0x1c, |e, copy| {
            let copy = copy.addr();
            e.call(STRING_TIDY, &args![copy, 0u32, 0u32]);
            let npos: u32 = e.global(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![copy, name, 0u32, npos]);
            StringArg::load(e, copy)
        });
        let opened = e
            .call(BINARY_FILE_OPEN_FOR_READ, &args![file_ptr, copy, 0u32])
            .u8();
        if opened == 0 {
            destroy_binary_file(e, file);
            return false;
        }
        if e.call(
            BINARY_FILE_READ_HEADER,
            &args![file_ptr, header.address, 1u32, 0x38u32],
        )
        .u8()
            == 0
        {
            destroy_binary_file(e, file);
            return false;
        }
        fn_00ab2630(e, Ptr::new(header.address));
        let word = e.mem.u32(header.address + 0x10);
        e.mem.set_u32(out, word);
        e.call(MORPH_VECTOR_CLEAR, &args![symmetric]);
        e.call(MORPH_VECTOR_CLEAR, &args![asymmetric]);
        let one = 1u32;
        let width = header.width(e);
        let padding = (width.wrapping_mul(one).wrapping_add(7) & 0xffff_fff8) / one;
        let padding = padding.wrapping_sub(header.width(e));
        let stride = header.width(e).wrapping_add(padding);
        let layout = Layout {
            padding,
            stride,
            plane: stride.wrapping_mul(header.rows(e)),
        };
        let sets = [
            (header.address + 8, symmetric, MorphSet::Symmetric),
            (header.address + 0xc, asymmetric, MorphSet::Asymmetric),
        ];
        for (count_at, vector, set) in sets {
            let count = e.mem.u32(count_at);
            e.with_stack(MORPH_SIZE, |e, slot| {
                let slot = slot.addr();
                fn_00ab2e40(e, Ptr::new(slot));
                resize_from_slot(e, vector, count, slot);
            });
            if !read_morphs(e, file, &header, count_at, vector.addr(), &layout, set) {
                return false;
            }
        }
        e.call(BINARY_FILE_DESTROY, &args![file_ptr]);
        true
    })
}

// Translated from 00ab2e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a texture morph's constructor (`thiscall`, no words): its three
/// byte vectors (+0x10) through the CRT's `eh vector constructor iterator` (`00ec782f`, with
/// the constructor `00ab2e70` and the destructor `0065f060`); the first 0x10 bytes are left as
/// they are. Returns `this`.
pub fn fn_00ab2e40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        EH_VECTOR_CONSTRUCTOR,
        &args![
            this.addr() + MORPH_PLANES,
            PLANE_SIZE,
            3u32,
            0x00ab_2e70u32,
            BYTE_VECTOR_DESTRUCT
        ],
    );
    this
}

// Translated from 00ab2f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a byte vector's `resize(count, value)` (`thiscall`, two words:
/// the count and the value byte in a word): grows with `00ab3370`, or shrinks with the advance
/// `00ab43d0` and the erase `0065f700`.
pub fn fn_00ab2f50(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let shape = ResizeShape {
        size: ElementSize::Shift(0),
        insert: fn_00ab3370,
        advance: |e, iterator, count| advance_by_address(ADVANCE_BYTES, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(ERASE_BYTES, e, this, result, a, b, c, d);
        },
    };
    with_value_slot(e, &[value], |e, slot| {
        resize_with_value(e, this, count, slot, &shape);
    });
}

// Translated from 00ab3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a texture morph vector's `resize(count, value)` (`thiscall`, 23
/// words: the count and the 0x58-byte value): grows with `00ab3a40`, or shrinks with the
/// advance `00ab4310` and the erase `0065d8e0`; then the value's byte vectors are destroyed
/// (`00ec5fce` with `0065f060`).
// Translation incomplete: 00ab3130 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f908, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab3130(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<22>) {
    with_value_slot(e, &value.0, |e, slot| {
        resize_from_slot(e, this, count, slot)
    });
}

/// `00ab3130`'s body over the value's argument slot at `slot`.
fn resize_from_slot(e: &mut Engine, this: Ptr, count: u32, slot: u32) {
    let shape = ResizeShape {
        size: ElementSize::Divide(MORPH_SIZE),
        insert: fn_00ab3a40,
        advance: |e, iterator, count| advance_by_address(ADVANCE_58, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(ERASE_58, e, this, result, a, b, c, d);
        },
    };
    resize_with_value(e, this, count, slot, &shape);
    e.call(
        EH_VECTOR_DESTRUCTOR,
        &args![slot + MORPH_PLANES, PLANE_SIZE, 3u32, BYTE_VECTOR_DESTRUCT],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_2630, fn_00ab2630(Ptr)),
        entry!(0x00ab_2640, fff_read_egt_file(u32, u32, Ptr, Ptr) -> bool),
        entry!(0x00ab_2e40, fn_00ab2e40(Ptr) -> Ptr),
        entry!(0x00ab_2f50, fn_00ab2f50(Ptr, u32, u32)),
        entry!(0x00ab_3130, fn_00ab3130(Ptr, u32, WordsArg<22>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::facegen::binaryfile::{INVALID_PARAMETER, VECTOR_PROXY};

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A file of 2 rows of 3 bytes (padding 5, padded row 8, plane 16), one morph of each
    /// set and the word 0x66; every read of `n` bytes writes `n` copies of a running counter
    /// (scale reads included). The read number `fail_at` (from 1) fails.
    fn engine(fail_at: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        for addr in [
            STRING_FROM_C,
            BINARY_FILE_CONSTRUCT,
            STRING_TIDY,
            STRING_ASSIGN,
            CLOSE_GAME_FILE,
            FUT_BINARY_FILE_DESTROY,
            BINARY_FILE_DESTROY,
            MORPH_VECTOR_CLEAR,
            INVALID_PARAMETER,
            EH_VECTOR_CONSTRUCTOR,
            EH_VECTOR_DESTRUCTOR,
            ADVANCE_BYTES,
            ERASE_BYTES,
            ADVANCE_58,
            ERASE_58,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(BINARY_FILE_OPEN_FOR_READ, |_, _| 1u32.into_ret());
        e.register(BINARY_FILE_READ_HEADER, |e, a| {
            e.mem.set_u32(a[1], 2);
            e.mem.set_u32(a[1] + 4, 3);
            e.mem.set_u32(a[1] + 8, 1);
            e.mem.set_u32(a[1] + 0xc, 1);
            e.mem.set_u32(a[1] + 0x10, 0x66);
            1u32.into_ret()
        });
        e.register(BYTE_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + VECTOR_FIRST) + a[1]).into_ret()
        });
        let mut reads = 0u32;
        e.register_double(BINARY_FILE_READ, move |e, a| {
            reads += 1;
            if reads == fail_at {
                return Ret::default();
            }
            for i in 0..a[2] * a[3] {
                e.mem.set_u8(a[1] + i, reads as u8);
            }
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A texture morph vector of one morph whose planes already hold 16 bytes.
    fn morphs(e: &mut Engine) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let morph = e.mem.alloc(MORPH_SIZE);
        e.mem.set_u32(vector + VECTOR_FIRST, morph);
        e.mem.set_u32(vector + VECTOR_LAST, morph + MORPH_SIZE);
        e.mem.set_u32(vector + 0x14, morph + MORPH_SIZE);
        for plane in 0..3 {
            let bytes = morph + MORPH_PLANES + plane * PLANE_SIZE;
            let block = e.mem.alloc(16);
            let proxy = e.mem.alloc(8);
            e.mem.set_u32(bytes + VECTOR_PROXY, proxy);
            e.mem.set_u32(bytes + VECTOR_FIRST, block);
            e.mem.set_u32(bytes + VECTOR_LAST, block + 16);
            e.mem.set_u32(bytes + 0x14, block + 16);
        }
        Ptr::new(vector)
    }

    fn plane(e: &Engine, vector: Ptr, plane: u32) -> Vec<u8> {
        let morph = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let first = e
            .mem
            .u32(morph + MORPH_PLANES + plane * PLANE_SIZE + VECTOR_FIRST);
        e.mem.bytes(first, 16)
    }

    #[test]
    fn egt_file_reads_planes_bottom_row_first() {
        let mut e = engine(0);
        let name = e.mem.alloc(0x1c);
        let out = e.mem.alloc(4);
        let symmetric = morphs(&mut e);
        let asymmetric = morphs(&mut e);
        assert!(fff_read_egt_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(out), 0x66);
        let morph = e.mem.u32(symmetric.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(morph), 0x0101_0101);
        assert_eq!(e.mem.u32(morph + 4), 3);
        assert_eq!(e.mem.u32(morph + 8), 2);
        assert_eq!(e.mem.u32(morph + 0xc), 5);
        // Plane 0: read 2 goes to offset 8 (the last row), read 3 to offset 0.
        assert_eq!(
            plane(&e, symmetric, 0),
            vec![3, 3, 3, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0]
        );
        assert_eq!(plane(&e, symmetric, 2)[8..11], [6, 6, 6]);
        // The asymmetric morph: scale read 8, its planes from 0065cc70.
        let morph = e.mem.u32(asymmetric.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(morph), 0x0808_0808);
        assert_eq!(plane(&e, asymmetric, 0)[..3], [10, 10, 10]);
        let addresses = calls_to(&log, BYTE_ADDRESS);
        assert_eq!(addresses.len(), 3);
        assert_eq!(addresses[0][1], 8);
        let reads = calls_to(&log, BINARY_FILE_READ);
        assert_eq!(reads.len(), 14);
        assert_eq!(&reads[1][2..], &[1, 3]);
        assert_eq!(calls_to(&log, BINARY_FILE_DESTROY).len(), 1);
        assert!(calls_to(&log, CLOSE_GAME_FILE).is_empty());
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        let open = calls_to(&log, BINARY_FILE_OPEN_FOR_READ);
        assert_eq!(open[0][8], 0);
        // Each set: a default morph built and destroyed around the resize.
        assert_eq!(calls_to(&log, EH_VECTOR_CONSTRUCTOR).len(), 2);
        assert_eq!(calls_to(&log, EH_VECTOR_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn egt_file_failures_destroy_the_file_inline_or_by_call() {
        // A symmetric row read fails: the inlined destructor.
        let mut e = engine(2);
        let name = e.mem.alloc(0x1c);
        let out = e.mem.alloc(4);
        let symmetric = morphs(&mut e);
        let asymmetric = morphs(&mut e);
        assert!(!fff_read_egt_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
        assert!(calls_to(&log, BINARY_FILE_DESTROY).is_empty());
        // An asymmetric row read fails: `00aaab30`.
        let mut e = engine(9);
        let symmetric = morphs(&mut e);
        let asymmetric = morphs(&mut e);
        let name = e.mem.alloc(0x1c);
        assert!(!fff_read_egt_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CLOSE_GAME_FILE).is_empty());
        assert_eq!(calls_to(&log, BINARY_FILE_DESTROY).len(), 1);
        // The asymmetric scale read fails: inlined again.
        let mut e = engine(8);
        let symmetric = morphs(&mut e);
        let asymmetric = morphs(&mut e);
        let name = e.mem.alloc(0x1c);
        assert!(!fff_read_egt_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
    }

    #[test]
    fn empty_header_hook_does_nothing() {
        let mut e = Engine::new();
        let header: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.mem.set_u32(header.addr(), 9);
        fn_00ab2630(&mut e, header);
        assert_eq!(e.mem.u32(header.addr()), 9);
    }

    #[test]
    fn texture_morph_constructor_builds_three_byte_vectors() {
        let mut e = engine(0);
        let this: Ptr = Ptr::new(e.mem.alloc(MORPH_SIZE));
        assert_eq!(fn_00ab2e40(&mut e, this), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EH_VECTOR_CONSTRUCTOR),
            vec![vec![
                this.addr() + 0x10,
                0x18,
                3,
                0x00ab_2e70,
                BYTE_VECTOR_DESTRUCT
            ]]
        );
    }

    #[test]
    fn byte_resize_grows_with_the_value_byte_or_erases() {
        let mut e = engine(0);
        e.register(0x0065_fa30, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(0x0065_fea0, |e, a| {
            if a[1] > a[0] {
                let bytes = e.mem.bytes(a[0], a[1] - a[0]);
                e.mem.write(a[2], &bytes);
            }
            (a[2] + a[1] - a[0]).into_ret()
        });
        e.register(0x00ac_3300, |e, a| {
            let byte = e.mem.u8(a[2]);
            for i in 0..a[1] {
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register(0x00aa_4060, |_, _| Ret::default());
        let vector: Ptr = Ptr::new(e.mem.alloc(0x18));
        let block = e.mem.alloc(4);
        e.mem.set_u8(block, 7);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(vector.addr() + VECTOR_PROXY, proxy);
        e.mem.set_u32(vector.addr() + VECTOR_FIRST, block);
        e.mem.set_u32(vector.addr() + VECTOR_LAST, block + 1);
        e.mem.set_u32(vector.addr() + 0x14, block + 1);
        fn_00ab2f50(&mut e, vector, 3, 0x5a);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.bytes(first, 3), vec![7, 0x5a, 0x5a]);
        e.call_log = Some(vec![]);
        fn_00ab2f50(&mut e, vector, 1, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADVANCE_BYTES).len(), 1);
        assert_eq!(calls_to(&log, ERASE_BYTES).len(), 1);
    }

    #[test]
    fn texture_morph_resize_destroys_the_value_planes() {
        let mut e = engine(0);
        let vector = morphs(&mut e);
        fn_00ab3130(&mut e, vector, 1, WordsArg([0; 22]));
        let log = e.call_log.take().unwrap();
        let destroyed = calls_to(&log, EH_VECTOR_DESTRUCTOR);
        assert_eq!(destroyed.len(), 1);
        assert_eq!(&destroyed[0][1..], &[0x18, 3, BYTE_VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        fn_00ab3130(&mut e, vector, 0, WordsArg([0; 22]));
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADVANCE_58).len(), 1);
        assert_eq!(calls_to(&log, ERASE_58).len(), 1);
    }
}

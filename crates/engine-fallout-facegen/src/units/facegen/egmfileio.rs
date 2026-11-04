//! `FaceGen/egmfileio.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Reading FaceGen's morph file (`.egm`, id `"FREGM002"`): a 0x38-byte header (the vertex
//! count, the symmetric and asymmetric morph counts and a word handed back), then for every
//! morph its float scale and `vertex count * 3` 16-bit values, stored as the morph's 6-byte
//! elements.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    BSFaceGenBinaryFile, StringArg, MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT, STRING_ASSIGN,
    STRING_NPOS, STRING_TIDY, VECTOR_FIRST, VECTOR_LAST,
};
use crate::units::facegen::run_00ab0a80::fn_00ab0a80;
use crate::units::facegen::run_00ab0d50::{fn_00ab0fd0, resize_from_slot};
use crate::units::facegen::run_00aba3c0::{
    advance_by_address, check, erase_by_address, resize_with_value, with_value_slot, ElementSize,
    ResizeShape,
};

/// `BSFaceGenBinaryFile`'s vtable (RTTI), set again by its inlined destructor.
const BS_FACE_GEN_BINARY_FILE_VTABLE: u32 = 0x010a_2c38;
/// The game file's close (`00ac5c00`, `thiscall`) and `FutBinaryFileC`'s destructor
/// (`00aaaa80`, `thiscall`), unit `binaryfile` and `fanctlfile`.
const CLOSE_GAME_FILE: u32 = 0x00ac_5c00;
const FUT_BINARY_FILE_DESTROY: u32 = 0x00aa_aa80;
/// `BSFaceGenBinaryFile`'s constructor (`00ac5490`, the id by value), `openForRead`
/// (`00ac5540`, the name by value and a byte), `readHeader` (`00ac5e00`: buffer, 1, size),
/// `read` (`00ac5cf0`: buffer, element size, count) and the transfer of `count` 16-bit values
/// into a buffer (`00ac5c70`: buffer, count), all `thiscall`, unit `binaryfile`.
const BINARY_FILE_CONSTRUCT: u32 = 0x00ac_5490;
const BINARY_FILE_OPEN_FOR_READ: u32 = 0x00ac_5540;
const BINARY_FILE_READ_HEADER: u32 = 0x00ac_5e00;
const BINARY_FILE_READ: u32 = 0x00ac_5cf0;
const BINARY_FILE_READ_SHORTS: u32 = 0x00ac_5c70;
/// The string from a C string (`0044a4c0`, `thiscall`: the characters).
const STRING_FROM_C: u32 = 0x0044_a4c0;
/// The morph file's id, `"FREGM002"`.
const MORPH_FILE_ID: u32 = 0x010a_2d24;
/// The morph vector's clear (`0065cd70`, `thiscall`).
const MORPH_VECTOR_CLEAR: u32 = 0x0065_cd70;
/// The memory manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, the size).
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The 6-byte vector's iterator advance (`00ab2150`, `thiscall`: the count) and erase
/// (`0065f5a0`, `thiscall`: result, first and last iterators).
const ADVANCE_6: u32 = 0x00ab_2150;
const ERASE_6: u32 = 0x0065_f5a0;

/// A morph (0x1C bytes): its float scale at +0 and its vector of 6-byte elements at +4.
const MORPH_SIZE: u32 = 0x1c;
const MORPH_ELEMENTS: u32 = 0x04;

// Translated from 00aafbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an empty `thiscall` (the morph file reader calls it on its
/// header).
pub fn fn_00aafbd0(_e: &mut Engine, _this: Ptr) {}

/// The inlined `BSFaceGenBinaryFile` destructor at every way out of `fffReadEgmFile`: its
/// vtable, the game file's close (`00ac5c00`), then `FutBinaryFileC`'s destructor
/// (`00aaaa80`).
fn destroy_binary_file(e: &mut Engine, file: u32) {
    e.mem.set_u32(file, BS_FACE_GEN_BINARY_FILE_VTABLE);
    e.call(CLOSE_GAME_FILE, &args![file]);
    e.call(FUT_BINARY_FILE_DESTROY, &args![file]);
}

/// The number of morphs in the morph vector at `vector` (signed division by 0x1C).
fn morph_count(e: &Engine, vector: u32) -> u32 {
    let span = e
        .mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST));
    ElementSize::Divide(MORPH_SIZE).count(span)
}

/// The address of element `index` of the 6-byte vector at `vector`, after the debug check
/// that `index` is below its size.
fn checked_six(e: &mut Engine, vector: u32, index: u32) -> u32 {
    let span = e
        .mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST));
    let ok = index < ElementSize::Divide(6).count(span);
    check(e, ok);
    index
        .wrapping_mul(6)
        .wrapping_add(e.mem.u32(vector + VECTOR_FIRST))
}

/// The copy of the read 16-bit values (three per vertex) into the morph's 6-byte elements:
/// four vertices at a time (the four element addresses checked first), then one value at a
/// time with a check before each.
fn store_vertices(e: &mut Engine, morph: u32, header: u32, buffer: u32) {
    let vector = morph + MORPH_ELEMENTS;
    let mut vertex = 0u32;
    while vertex.wrapping_add(3) < e.mem.u32(header) {
        let mut elements = [0u32; 4];
        for (i, element) in elements.iter_mut().enumerate() {
            *element = checked_six(e, vector, vertex + i as u32);
        }
        for (i, element) in elements.iter().enumerate() {
            let source = buffer.wrapping_add((vertex + i as u32).wrapping_mul(6));
            for half in 0..3u32 {
                let value = e.mem.u16(source + 2 * half);
                e.mem.set_u16(element + 2 * half, value);
            }
        }
        vertex = vertex.wrapping_add(4);
    }
    while vertex < e.mem.u32(header) {
        for half in 0..3u32 {
            let element = checked_six(e, vector, vertex);
            let source = buffer.wrapping_add(vertex.wrapping_mul(6));
            let value = e.mem.u16(source + 2 * half);
            e.mem.set_u16(element + 2 * half, value);
        }
        vertex = vertex.wrapping_add(1);
    }
}

// Translated from 00aafc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fffReadEgmFile` (Xbox PDB; `cdecl`, four words: the file name string, an out word, the
/// symmetric and the asymmetric morph vectors). Makes a `BSFaceGenBinaryFile` with the id
/// `"FREGM002"` and opens the name (`openForRead`, the byte 0); reads the 0x38-byte header
/// (`readHeader(header, 1, 0x38)`) and passes it to `00aafbd0`; stores its word at +0xC in
/// the out word; clears both vectors (`0065cd70`) and sizes them to the counts at +4 and +8
/// (`00ab0a30`). A buffer of `vertex count * 6` bytes (the count at +0) comes from the memory
/// manager. Every morph's element vector is resized to the vertex count (`00ab0b60`, the value
/// three zero halves), then its float scale is read (`read(scale, 4, 1)`) and stored, then
/// `vertex count * 3` 16-bit values (`00ac5c70`) are copied into its elements. The symmetric
/// morphs check their index against the vector three times, the asymmetric ones once. The
/// buffer is freed at the end only (a failed read leaves it); every way out destroys the
/// file; the answer is whether all of it was read.
// Translation incomplete: 00aafc80 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f730, state variable) and the /GS cookie check (00ec408c) are not translated.
pub fn fff_read_egm_file(
    e: &mut Engine,
    name: u32,
    out: u32,
    symmetric: Ptr,
    asymmetric: Ptr,
) -> bool {
    e.with_stack(0x48 + 0x38 + 4, |e, frame| {
        let file = frame.addr();
        let header = file + 0x48;
        let scale = header + 0x38;
        let id = e.with_stack(0x1c, |e, id| {
            e.call(STRING_FROM_C, &args![id, MORPH_FILE_ID]);
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
            &args![file_ptr, header, 1u32, 0x38u32],
        )
        .u8()
            == 0
        {
            destroy_binary_file(e, file);
            return false;
        }
        fn_00aafbd0(e, Ptr::new(header));
        let word = e.mem.u32(header + 0xc);
        e.mem.set_u32(out, word);
        e.call(MORPH_VECTOR_CLEAR, &args![symmetric]);
        e.call(MORPH_VECTOR_CLEAR, &args![asymmetric]);
        let count = e.mem.u32(header + 4);
        fn_00ab0a30(e, symmetric, count);
        let count = e.mem.u32(header + 8);
        fn_00ab0a30(e, asymmetric, count);
        let bytes = e.mem.u32(header).wrapping_mul(6);
        let buffer = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER_OBJECT, bytes],
            )
            .u32();
        // The upper half of the value's second word is not written (stack); the insert reads
        // only the six bytes of the element.
        let mut index = 0u32;
        while index < e.mem.u32(header + 4) {
            let vector = symmetric.addr();
            let ok = index < morph_count(e, vector);
            check(e, ok);
            let morph = index
                .wrapping_mul(MORPH_SIZE)
                .wrapping_add(e.mem.u32(vector + VECTOR_FIRST));
            let ok = index < morph_count(e, vector);
            check(e, ok);
            let vertices = e.mem.u32(header);
            fn_00ab0b60(e, Ptr::new(morph + MORPH_ELEMENTS), vertices, 0, 0);
            if e.call(BINARY_FILE_READ, &args![file_ptr, scale, 4u32, 1u32])
                .u8()
                == 0
            {
                destroy_binary_file(e, file);
                return false;
            }
            let ok = index < morph_count(e, vector);
            check(e, ok);
            let target = index
                .wrapping_mul(MORPH_SIZE)
                .wrapping_add(e.mem.u32(vector + VECTOR_FIRST));
            let value = x87_quiet_bits(e.mem.u32(scale));
            e.mem.set_u32(target, value);
            let shorts = e.mem.u32(header).wrapping_mul(3);
            if e.call(BINARY_FILE_READ_SHORTS, &args![file_ptr, buffer, shorts])
                .u8()
                == 0
            {
                destroy_binary_file(e, file);
                return false;
            }
            store_vertices(e, morph, header, buffer);
            index = index.wrapping_add(1);
        }
        let mut index = 0u32;
        while index < e.mem.u32(header + 8) {
            let vector = asymmetric.addr();
            let ok = index < morph_count(e, vector);
            check(e, ok);
            let morph = index
                .wrapping_mul(MORPH_SIZE)
                .wrapping_add(e.mem.u32(vector + VECTOR_FIRST));
            let vertices = e.mem.u32(header);
            fn_00ab0b60(e, Ptr::new(morph + MORPH_ELEMENTS), vertices, 0, 0);
            if e.call(BINARY_FILE_READ, &args![file_ptr, scale, 4u32, 1u32])
                .u8()
                == 0
            {
                destroy_binary_file(e, file);
                return false;
            }
            let value = x87_quiet_bits(e.mem.u32(scale));
            e.mem.set_u32(morph, value);
            let shorts = e.mem.u32(header).wrapping_mul(3);
            if e.call(BINARY_FILE_READ_SHORTS, &args![file_ptr, buffer, shorts])
                .u8()
                == 0
            {
                destroy_binary_file(e, file);
                return false;
            }
            store_vertices(e, morph, header, buffer);
            index = index.wrapping_add(1);
        }
        if buffer != 0 {
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, buffer]);
        }
        destroy_binary_file(e, file);
        true
    })
}

// Translated from 00ab0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a morph vector's `resize(count)` (`thiscall`, one word): builds
/// the default morph in its outgoing argument area (the float scale word left as the stack
/// had it, the element vector constructed by `00ab0a80`) and passes it by value to the
/// resize with a value (`00ab0d50`).
pub fn fn_00ab0a30(e: &mut Engine, this: Ptr, count: u32) {
    e.with_stack(MORPH_SIZE, |e, slot| {
        let slot = slot.addr();
        fn_00ab0a80(e, Ptr::new(slot + MORPH_ELEMENTS));
        resize_from_slot(e, this, count, slot);
    });
}

// Translated from 00ab0b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 6-byte-element vector's `resize(count, value)` (`thiscall`,
/// three words: the count and the value's two words): grows with `00ab0fd0`, or shrinks with
/// the advance `00ab2150` and the erase `0065f5a0`.
pub fn fn_00ab0b60(e: &mut Engine, this: Ptr, count: u32, value_0: u32, value_1: u32) {
    let shape = ResizeShape {
        size: ElementSize::Divide(6),
        insert: fn_00ab0fd0,
        advance: |e, iterator, count| advance_by_address(ADVANCE_6, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(ERASE_6, e, this, result, a, b, c, d);
        },
    };
    with_value_slot(e, &[value_0, value_1], |e, slot| {
        resize_with_value(e, this, count, slot, &shape);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_fbd0, fn_00aafbd0(Ptr)),
        entry!(0x00aa_fc80, fff_read_egm_file(u32, u32, Ptr, Ptr) -> bool),
        entry!(0x00ab_0a30, fn_00ab0a30(Ptr, u32)),
        entry!(0x00ab_0b60, fn_00ab0b60(Ptr, u32, u32, u32)),
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

    const ALLOCATE_PROXY: u32 = 0x0044_d670;
    const FLOATS_BUY: u32 = 0x0065_f380;
    const FLOATS_TIDY: u32 = 0x0065_f430;

    /// A file whose header gives `vertices`, one symmetric and one asymmetric morph and the
    /// word 0x77; the scale read gives 0.5 (or fails when `scale_fails`); the 16-bit values
    /// read gives 1, 2, 3, ...
    fn engine(vertices: u32, opens: bool, scale_fails: bool) -> Engine {
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
            MORPH_VECTOR_CLEAR,
            MEMORY_MANAGER_FREE,
            INVALID_PARAMETER,
            FLOATS_BUY,
            FLOATS_TIDY,
            ADVANCE_6,
            ERASE_6,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE_PROXY, |e, _| e.mem.alloc(8).into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(4)).into_ret()
        });
        e.register_double(BINARY_FILE_OPEN_FOR_READ, move |_, _| {
            (opens as u32).into_ret()
        });
        e.register_double(BINARY_FILE_READ_HEADER, move |e, a| {
            e.mem.set_u32(a[1], vertices);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 1);
            e.mem.set_u32(a[1] + 0xc, 0x77);
            1u32.into_ret()
        });
        e.register_double(BINARY_FILE_READ, move |e, a| {
            if scale_fails {
                return Ret::default();
            }
            e.mem.set_u32(a[1], 0.5f32.to_bits());
            1u32.into_ret()
        });
        e.register(BINARY_FILE_READ_SHORTS, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u16(a[1] + 2 * i, 1 + i as u16);
            }
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A morph vector of one morph whose element vector already has `vertices` elements.
    fn morphs(e: &mut Engine, vertices: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let morph = e.mem.alloc(MORPH_SIZE);
        e.mem.set_u32(vector + VECTOR_FIRST, morph);
        e.mem.set_u32(vector + VECTOR_LAST, morph + MORPH_SIZE);
        e.mem.set_u32(vector + 0x14, morph + MORPH_SIZE);
        let elements = morph + MORPH_ELEMENTS;
        let block = e.mem.alloc(6 * vertices);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(elements + VECTOR_PROXY, proxy);
        e.mem.set_u32(elements + VECTOR_FIRST, block);
        e.mem.set_u32(elements + VECTOR_LAST, block + 6 * vertices);
        e.mem.set_u32(elements + 0x14, block + 6 * vertices);
        Ptr::new(vector)
    }

    fn halves(e: &Engine, vector: Ptr) -> Vec<u16> {
        let morph = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let first = e.mem.u32(morph + MORPH_ELEMENTS + VECTOR_FIRST);
        let last = e.mem.u32(morph + MORPH_ELEMENTS + VECTOR_LAST);
        (first..last).step_by(2).map(|a| e.mem.u16(a)).collect()
    }

    #[test]
    fn egm_file_reads_scales_and_vertices_of_both_morph_sets() {
        let mut e = engine(5, true, false);
        let name = e.mem.alloc(0x1c);
        let out = e.mem.alloc(4);
        let symmetric = morphs(&mut e, 5);
        let asymmetric = morphs(&mut e, 5);
        assert!(fff_read_egm_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(out), 0x77);
        let expected: Vec<u16> = (1..=15).collect();
        assert_eq!(halves(&e, symmetric), expected);
        assert_eq!(halves(&e, asymmetric), expected);
        for vector in [symmetric, asymmetric] {
            let morph = e.mem.u32(vector.addr() + VECTOR_FIRST);
            assert_eq!(e.mem.f32(morph), 0.5);
        }
        let allocation = calls_to(&log, MEMORY_MANAGER_ALLOCATE);
        assert_eq!(allocation, vec![vec![MEMORY_MANAGER_OBJECT, 30]]);
        let shorts = calls_to(&log, BINARY_FILE_READ_SHORTS);
        assert_eq!(shorts.len(), 2);
        assert_eq!(shorts[0][2], 15);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        assert_eq!(calls_to(&log, MORPH_VECTOR_CLEAR).len(), 2);
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
        assert_eq!(calls_to(&log, FUT_BINARY_FILE_DESTROY).len(), 1);
        // The buffer freed: the memory manager's free of the allocated block.
        let frees = calls_to(&log, MEMORY_MANAGER_FREE);
        assert!(frees.iter().any(|w| w[1] != 0 && shorts[0][1] == w[1]));
        let open = calls_to(&log, BINARY_FILE_OPEN_FOR_READ);
        assert_eq!(open[0].len(), 9);
        assert_eq!(open[0][8], 0);
    }

    #[test]
    fn egm_file_failures_destroy_the_file() {
        let mut e = engine(5, false, false);
        let name = e.mem.alloc(0x1c);
        let out = e.mem.alloc(4);
        let symmetric = morphs(&mut e, 5);
        let asymmetric = morphs(&mut e, 5);
        assert!(!fff_read_egm_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
        assert!(calls_to(&log, MEMORY_MANAGER_ALLOCATE).is_empty());
        // A failed scale read leaves the buffer allocated.
        let mut e = engine(5, true, true);
        let symmetric = morphs(&mut e, 5);
        let asymmetric = morphs(&mut e, 5);
        let name = e.mem.alloc(0x1c);
        assert!(!fff_read_egm_file(&mut e, name, out, symmetric, asymmetric));
        let log = e.call_log.take().unwrap();
        let buffer = calls_to(&log, MEMORY_MANAGER_ALLOCATE).len();
        assert_eq!(buffer, 1);
        assert_eq!(calls_to(&log, CLOSE_GAME_FILE).len(), 1);
        assert_eq!(calls_to(&log, BINARY_FILE_READ).len(), 1);
    }

    #[test]
    fn empty_header_hook_does_nothing() {
        let mut e = Engine::new();
        let header: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.mem.set_u32(header.addr(), 9);
        fn_00aafbd0(&mut e, header);
        assert_eq!(e.mem.u32(header.addr()), 9);
    }

    #[test]
    fn morph_resize_builds_a_default_morph_in_its_argument_area() {
        let mut e = engine(0, true, false);
        let vector = morphs(&mut e, 0);
        // Already one morph: the resize to 1 only builds and destroys the value.
        fn_00ab0a30(&mut e, vector, 1);
        let log = e.call_log.take().unwrap();
        let buy = calls_to(&log, FLOATS_BUY);
        assert_eq!(buy.len(), 1);
        assert_eq!(calls_to(&log, FLOATS_TIDY), vec![vec![buy[0][0]]]);
        assert_eq!(
            e.mem.u32(vector.addr() + VECTOR_LAST) - e.mem.u32(vector.addr() + VECTOR_FIRST),
            MORPH_SIZE
        );
    }

    #[test]
    fn six_byte_resize_grows_with_the_value_or_erases() {
        let mut e = engine(0, true, false);
        e.register(0x0065_f9e0, |e, a| e.mem.alloc(6 * a[0]).into_ret());
        e.register(0x0065_fde0, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2], &bytes);
            (a[2] + a[1] - a[0]).into_ret()
        });
        let vector: Ptr = Ptr::new(e.mem.alloc(0x18));
        let block = e.mem.alloc(6);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(vector.addr() + VECTOR_PROXY, proxy);
        e.mem.set_u32(vector.addr() + VECTOR_FIRST, block);
        e.mem.set_u32(vector.addr() + VECTOR_LAST, block + 6);
        e.mem.set_u32(vector.addr() + 0x14, block + 6);
        fn_00ab0b60(&mut e, vector, 3, 0x2222_1111, 0x4444_3333);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(vector.addr() + VECTOR_LAST), first + 18);
        assert_eq!(e.mem.u32(first + 6), 0x2222_1111);
        assert_eq!(e.mem.u16(first + 10), 0x3333);
        assert_eq!(e.mem.u16(first + 16), 0x3333);
        e.call_log = Some(vec![]);
        fn_00ab0b60(&mut e, vector, 1, 0, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADVANCE_6).len(), 1);
        assert_eq!(calls_to(&log, ERASE_6).len(), 1);
    }
}

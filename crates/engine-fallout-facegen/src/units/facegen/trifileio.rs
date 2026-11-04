//! `FaceGen/trifileio.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The FaceGen SDK's `.tri` reader, compiled with the debug `std::string`
//! of the SDK's runtime (size at +0x14, capacity at +0x18, the text inline
//! at +4 below 16 characters).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::run_00aba3c0::{fn_00abb700, resize_38_in_area};

/// `std::string::erase` (`0044a690`: `this`; position, count) and the
/// append of `count` copies of a character (`00aadeb0`: `this`; count,
/// character).
const STRING_ERASE: u32 = 0x0044_a690;
const STRING_APPEND_CHARS: u32 = 0x00aa_deb0;
/// `std::string::npos` (the exe's data).
const NPOS: u32 = 0x0101_73f0;

// Translated from 00ab46e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that does nothing (`this` only).
pub fn fn_00ab46e0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00aba370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::resize` (`this`; size, character): grows by appending
/// `size - length` copies of `fill`, or erases from `size` to the end.
pub fn fn_00aba370(e: &mut Engine, this: Ptr, size: u32, fill: u8) {
    let length = e.mem.u32(this.addr().wrapping_add(0x14));
    if size <= length {
        let npos = e.global::<u32>(NPOS);
        e.call(STRING_ERASE, &args![this, size, npos]);
    } else {
        let length = e.mem.u32(this.addr().wrapping_add(0x14));
        e.call(
            STRING_APPEND_CHARS,
            &args![this, size.wrapping_sub(length), u32::from(fill)],
        );
    }
}

/// `_memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// `std::string` construction from text (`0044a4c0`: `this`; text), the
/// empty construction / release (`0044a730`: `this`; two words), assign
/// (`0044a580`: `this`; other, position, count) and copy construction
/// (`0044a500`: `this`; other).
const STRING_FROM_TEXT: u32 = 0x0044_a4c0;
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_COPY: u32 = 0x0044_a500;
/// `"FRTRI003"`, the file's magic, and `""`.
const TEXT_MAGIC: u32 = 0x010a_2d54;
const TEXT_EMPTY: u32 = 0x0101_1584;
/// `BSFaceGenBinaryFile` (Xbox PDB names): constructor (`00ac5490`:
/// `this`; magic string by value), `openForRead` (`00ac5540`: `this`; name
/// string by value, 0), `readHeader` (`00ac5e00`: `this`; buffer, 1,
/// size), `read` (`00ac5cf0`: `this`; buffer, size, count), the close and
/// destructor bodies of the failure paths (`00ac5c00`, `00aaaa80`, after
/// the vtable `010a2c38` is put back) and the destructor of the later
/// paths (`00aaab30`).
const FILE_CONSTRUCT: u32 = 0x00ac_5490;
const FILE_OPEN_FOR_READ: u32 = 0x00ac_5540;
const FILE_READ_HEADER: u32 = 0x00ac_5e00;
const FILE_READ: u32 = 0x00ac_5cf0;
const FILE_CLOSE: u32 = 0x00ac_5c00;
const FILE_DESTRUCT_BASE: u32 = 0x00aa_aa80;
const FILE_DESTRUCT: u32 = 0x00aa_ab30;
const FILE_VTABLE: u32 = 0x010a_2c38;
/// The mesh's vector resizes: `NiPoint3`s (`00aba7b0`: vector; count,
/// three words), 16-byte rows (`00abaaa0`: vector; count, four words),
/// 32-byte named entries (`00ab9eb0`: vector; count), 0x2C-byte entries
/// (`00abb3c0`: vector; count, eleven words, from `00aba220`), two-float
/// rows (`00abad80`: vector; count, two words) and words (`00aba5d0`:
/// vector; count, value); the clears of +0x60 and +0x78 (`00ab9cd0`,
/// `00ab9dc0`). The 0x34-byte entries (`00abb700`, thirteen words, from
/// `00aba270`) and the 0x38-byte entries (`00abbae0`, fourteen words, from
/// `00aba2f0`) take their value in place, called directly with the
/// argument area the entry is built in: [`fn_00abb700`],
/// [`resize_38_in_area`].
const RESIZE_POINTS: u32 = 0x00ab_a7b0;
const RESIZE_ROWS: u32 = 0x00ab_aaa0;
const RESIZE_NAMED: u32 = 0x00ab_9eb0;
const RESIZE_2C: u32 = 0x00ab_b3c0;
const RESIZE_PAIRS: u32 = 0x00ab_ad80;
const RESIZE_WORDS: u32 = 0x00ab_a5d0;
const MAKE_2C: u32 = 0x00ab_a220;
const MAKE_34: u32 = 0x00ab_a270;
const MAKE_38: u32 = 0x00ab_a2f0;
const CLEAR_60: u32 = 0x00ab_9cd0;
const CLEAR_78: u32 = 0x00ab_9dc0;
/// A word vector's element address (`006578b0`: vector; index).
const WORD_AT: u32 = 0x0065_78b0;
/// The name buffer, a `std::vector<char>`: constructor (`00aba0d0`:
/// `this`; size) and its two destructor steps (`00abc0a0`, `00abc120`).
const BUFFER_CONSTRUCT: u32 = 0x00ab_a0d0;
const BUFFER_RELEASE: u32 = 0x00ab_c0a0;
const BUFFER_DESTRUCT: u32 = 0x00ab_c120;
/// The CRT's invalid-index report (`00ec7c56`).
const INVALID_INDEX: u32 = 0x00ec_7c56;
/// `futSetLastError` (Xbox PDB name of `00ac6430`, cdecl: code, string
/// by value).
const SET_LAST_ERROR: u32 = 0x00ac_6430;

/// The by-value words of the object of `size` bytes at `at`.
fn words_of(e: &Engine, at: Ptr, size: u32) -> Vec<u32> {
    (0..size / 4)
        .map(|i| e.mem.u32(at.addr() + i * 4))
        .collect()
}

/// `(last - first) / size` of a vector (first at +0xC, last at +0x10),
/// signed division (`CDQ; IDIV`).
fn count_div(e: &Engine, vector: u32, size: u32) -> u32 {
    let first = e.mem.u32(vector.wrapping_add(0xc));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    ((last.wrapping_sub(first) as i32).wrapping_div(size as i32)) as u32
}

/// `(last - first) >> shift` of a vector (`SAR`).
fn count_sar(e: &Engine, vector: u32, shift: u32) -> u32 {
    let first = e.mem.u32(vector.wrapping_add(0xc));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    ((last.wrapping_sub(first) as i32) >> shift) as u32
}

/// The element `index` of a vector, after the exe's bounds check
/// (`index < count`, reported to the CRT otherwise).
fn element(e: &mut Engine, vector: u32, index: u32, count: u32, size: u32) -> u32 {
    if index >= count {
        e.call(INVALID_INDEX, &[]);
    }
    index
        .wrapping_mul(size)
        .wrapping_add(e.mem.u32(vector.wrapping_add(0xc)))
}

/// The first element of a vector, after the exe's non-empty check.
fn first_element(e: &mut Engine, vector: u32, count: u32) -> u32 {
    if count == 0 {
        e.call(INVALID_INDEX, &[]);
    }
    e.mem.u32(vector.wrapping_add(0xc))
}

fn read(e: &mut Engine, file: Ptr, buffer: u32, size: u32, count: u32) -> bool {
    e.call(FILE_READ, &args![file, buffer, size, count]).bool()
}

/// The early failure path: the file's vtable put back, closed and
/// destroyed; false.
fn give_up(e: &mut Engine, file: Ptr) -> bool {
    e.mem.set_u32(file.addr(), FILE_VTABLE);
    e.call(FILE_CLOSE, &args![file]);
    e.call(FILE_DESTRUCT_BASE, &args![file]);
    false
}

fn release_buffer(e: &mut Engine, buffer: Ptr) {
    e.call(BUFFER_RELEASE, &args![buffer]);
    e.call(BUFFER_DESTRUCT, &args![buffer]);
}

/// Where a name goes: the string of entry `index` of the vector at
/// `mesh + vector`, `string` bytes into the entry, the entry's size and
/// how the exe computes the count.
#[derive(Clone, Copy)]
struct NameSlot {
    vector: u32,
    size: u32,
    shift: Option<u32>,
    string: u32,
}

impl NameSlot {
    fn address(self, e: &mut Engine, mesh: u32, index: u32) -> u32 {
        let vector = mesh.wrapping_add(self.vector);
        let count = match self.shift {
            Some(shift) => count_sar(e, vector, shift),
            None => count_div(e, vector, self.size),
        };
        element(e, vector, index, count, self.size).wrapping_add(self.string)
    }
}

/// Reads a name of `length` bytes into the slot's string: through the
/// buffer `buffer`, resized to `length - 1`; `copy_each` copies every
/// character (the morph loops), otherwise only the one at `length - 1`
/// after an empty counting loop (the loops of named entries and modifiers,
/// as compiled). A length of 1 assigns `""` through `temp`. Returns false
/// after the failure path when the read fails.
#[allow(clippy::too_many_arguments)]
fn read_name(
    e: &mut Engine,
    file: Ptr,
    mesh: u32,
    slot: NameSlot,
    index: u32,
    length: u32,
    buffer: Ptr,
    temp: Ptr,
    copy_each: bool,
) -> bool {
    e.call(BUFFER_CONSTRUCT, &args![buffer, length]);
    let buffer_first = |e: &Engine| e.mem.u32(buffer.addr() + 0xc);
    let buffer_size = |e: &Engine| {
        e.mem
            .u32(buffer.addr() + 0x10)
            .wrapping_sub(e.mem.u32(buffer.addr() + 0xc))
    };
    if buffer_size(e) == 0 {
        e.call(INVALID_INDEX, &[]);
    }
    let data = buffer_first(e);
    if !read(e, file, data, 1, length) {
        release_buffer(e, buffer);
        return give_up(e, file);
    }
    if length > 1 {
        let string = slot.address(e, mesh, index);
        fn_00aba370(e, Ptr::new(string), length - 1, 0);
        let copy = |e: &mut Engine, k: u32| {
            if k >= buffer_size(e) {
                e.call(INVALID_INDEX, &[]);
            }
            let source = buffer_first(e).wrapping_add(k);
            let string = slot.address(e, mesh, index);
            if k > e.mem.u32(string.wrapping_add(0x14)) {
                e.call(INVALID_INDEX, &[]);
            }
            let text = if e.mem.u32(string.wrapping_add(0x18)) >= 0x10 {
                e.mem.u32(string.wrapping_add(4))
            } else {
                string.wrapping_add(4)
            };
            let c = e.mem.u8(source);
            e.mem.set_u8(text.wrapping_add(k), c);
        };
        let mut k = 0u32;
        if copy_each {
            while k < length - 1 {
                copy(e, k);
                k += 1;
            }
        } else {
            while k < length - 1 {
                k += 1;
            }
            copy(e, k);
        }
    } else {
        let empty = e.call(STRING_FROM_TEXT, &args![temp, TEXT_EMPTY]).u32();
        let string = slot.address(e, mesh, index);
        let npos = e.global::<u32>(NPOS);
        e.call(STRING_ASSIGN, &args![string, empty, 0u32, npos]);
        e.call(STRING_TIDY, &args![temp, 1u32, 0u32]);
    }
    release_buffer(e, buffer);
    true
}

// Translated from 00ab5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fffReadTriFile` (Xbox PDB, cdecl: file name `std::string *`, mesh):
/// reads a FaceGen `.tri` file (`"FRTRI003"`) into the mesh: the header
/// (0x38 bytes: vertex, triangle, quad, named-entry, modifier, extra-UV
/// and flag counts...), then the vertices (+0x0, also room for the
/// modifier vertices), triangles (+0x18), quads (+0x30), the named
/// entries (+0x90: a word and a name each), the modifier entries (+0xA8:
/// a word, a point and a name), the UV data when header flag 1 is set
/// (+0x48, +0x60, +0x78), the differential morphs (+0xC0: name, scale and
/// `short` offsets scaled into points) and the statistical morphs (+0xD8:
/// name, first vertex index, vertex indices). Every failed read closes
/// the file and returns false; a vertex count that does not add up sets
/// the last error (`futSetLastError(1, name)`) and returns false.
// Translation incomplete: 00ab5ec0 - SEH exception frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not modelled.
pub fn fff_read_tri_file(e: &mut Engine, name: Ptr, mesh: u32) -> bool {
    e.with_stack(0x4d0, |e, frame| {
        let ebp = frame.addr() + 0x4d0;
        let local = |offset: u32| Ptr::<()>::new(ebp - offset);
        let header = local(0x48);
        let file = local(0x94);
        let h = |e: &Engine, offset: u32| e.mem.u32(header.addr() + offset);
        e.call(MEMSET, &args![header, 0u32, 0x38u32]);
        let magic = e.with_stack(0x1c, |e, text| {
            e.call(STRING_FROM_TEXT, &args![text, TEXT_MAGIC]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(magic);
        e.call(FILE_CONSTRUCT, &words);
        let path = e.with_stack(0x1c, |e, text| {
            e.call(STRING_TIDY, &args![text, 0u32, 0u32]);
            let npos = e.global::<u32>(NPOS);
            e.call(STRING_ASSIGN, &args![text, name, 0u32, npos]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(path);
        words.push(0);
        if !e.call(FILE_OPEN_FOR_READ, &words).bool() {
            return give_up(e, file);
        }
        if !e
            .call(FILE_READ_HEADER, &args![file, header, 1u32, 0x38u32])
            .bool()
        {
            return give_up(e, file);
        }
        fn_00ab46e0(e, header);
        let vertices = h(e, 0).wrapping_add(h(e, 0x24));
        e.call(
            RESIZE_POINTS,
            &args![
                mesh,
                vertices,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits()
            ],
        );
        e.call(
            RESIZE_POINTS,
            &args![mesh + 0x18, h(e, 4), 0u32, 0u32, 0u32],
        );
        e.call(
            RESIZE_ROWS,
            &args![mesh + 0x30, h(e, 8), 0u32, 0u32, 0u32, 0u32],
        );
        e.call(RESIZE_NAMED, &args![mesh + 0x90, h(e, 0xc)]);
        let n = h(e, 0x10);
        let value = e.with_stack(0x2c, |e, value| {
            e.call(MAKE_2C, &args![value]);
            words_of(e, value, 0x2c)
        });
        let mut words = vec![mesh + 0xa8, n];
        words.extend(value);
        e.call(RESIZE_2C, &words);
        // 00ab61a4: `SUB ESP,0x34; MOV ECX,ESP`: the default 0x34-byte entry is built in the
        // outgoing argument area, which stays in place as `00abb700`'s own `[EBP+0xC]` (it
        // destroys the entry there; its `RET 0x38` frees the area). The entry's vector proxy
        // names the area.
        let n = h(e, 0x1c);
        e.with_stack(0x34, |e, value| {
            e.call(MAKE_34, &args![value]); // 00ab61af
            fn_00abb700(e, Ptr::new(mesh + 0xc0), n, value); // 00ab61ca: the area itself
        });
        // 00ab61d8: `SUB ESP,0x38; MOV ECX,ESP`: the default 0x38-byte entry is built in the
        // outgoing argument area, which stays in place as `00abbae0`'s own `[EBP+0xC]`: its
        // vector's proxy names that area, and `00abbae0` destroys the entry there.
        let n = h(e, 0x20); // 00ab61cf
        e.with_stack(0x38, |e, outgoing| {
            e.call(MAKE_38, &args![outgoing]); // 00ab61e3
            resize_38_in_area(e, Ptr::new(mesh + 0xd8), n, outgoing.addr()); // 00ab61fe: the area
        });
        if h(e, 0).wrapping_add(h(e, 0x24)) != 0 {
            let count = count_div(e, mesh, 0xc);
            let first = first_element(e, mesh, count);
            let n = h(e, 0).wrapping_add(h(e, 0x24));
            if !read(e, file, first, 0xc, n) {
                return give_up(e, file);
            }
        }
        if h(e, 4) != 0 {
            let vector = mesh + 0x18;
            let count = count_div(e, vector, 0xc);
            let first = first_element(e, vector, count);
            if !read(e, file, first, 0xc, h(e, 4)) {
                return give_up(e, file);
            }
        }
        if h(e, 8) != 0 {
            let vector = mesh + 0x30;
            let count = count_sar(e, vector, 4);
            let first = first_element(e, vector, count);
            if !read(e, file, first, 0x10, h(e, 8)) {
                return give_up(e, file);
            }
        }
        // The named entries (+0x90, 32 bytes: a word, then the name).
        let named = NameSlot {
            vector: 0x90,
            size: 0x20,
            shift: Some(5),
            string: 4,
        };
        let mut i = 0u32;
        while i < h(e, 0xc) {
            let entry = named.address(e, mesh, i).wrapping_sub(4);
            if !read(e, file, entry, 4, 1) {
                return give_up(e, file);
            }
            let length = local(0xa0);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    named,
                    i,
                    length,
                    local(0xb8),
                    local(0x158),
                    false,
                )
            {
                return false;
            }
            i += 1;
        }
        // The modifier entries (+0xA8, 0x2C bytes: a word, a point, the name).
        let modifiers = NameSlot {
            vector: 0xa8,
            size: 0x2c,
            shift: None,
            string: 0x10,
        };
        let mut i = 0u32;
        while i < h(e, 0x10) {
            let entry = modifiers.address(e, mesh, i).wrapping_sub(0x10);
            if !read(e, file, entry, 4, 1) {
                return give_up(e, file);
            }
            let entry = modifiers.address(e, mesh, i).wrapping_sub(0x10);
            if !read(e, file, entry.wrapping_add(4), 0xc, 1) {
                return give_up(e, file);
            }
            let length = local(0xc0);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    modifiers,
                    i,
                    length,
                    local(0xd8),
                    local(0x178),
                    false,
                )
            {
                return false;
            }
            i += 1;
        }
        if h(e, 0x18) & 1 != 0 {
            let zero = 0.0f32.to_bits();
            if h(e, 0x14) == 0 {
                e.call(RESIZE_PAIRS, &args![mesh + 0x48, h(e, 0), zero, zero]);
                if h(e, 0) != 0 {
                    let vector = mesh + 0x48;
                    let count = count_sar(e, vector, 3);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 8, h(e, 0)) {
                        return give_up(e, file);
                    }
                }
                e.call(CLEAR_60, &args![mesh + 0x60]);
                e.call(CLEAR_78, &args![mesh + 0x78]);
            } else {
                e.call(RESIZE_PAIRS, &args![mesh + 0x48, h(e, 0x14), zero, zero]);
                if h(e, 0x14) != 0 {
                    let vector = mesh + 0x48;
                    let count = count_sar(e, vector, 3);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 8, h(e, 0x14)) {
                        return give_up(e, file);
                    }
                }
                e.call(
                    RESIZE_POINTS,
                    &args![mesh + 0x60, h(e, 4), 0u32, 0u32, 0u32],
                );
                if h(e, 4) != 0 {
                    let vector = mesh + 0x60;
                    let count = count_div(e, vector, 0xc);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 0xc, h(e, 4)) {
                        return give_up(e, file);
                    }
                }
                e.call(
                    RESIZE_ROWS,
                    &args![mesh + 0x78, h(e, 8), 0u32, 0u32, 0u32, 0u32],
                );
                if h(e, 8) != 0 {
                    let vector = mesh + 0x78;
                    let count = count_sar(e, vector, 4);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 0x10, h(e, 8)) {
                        return give_up(e, file);
                    }
                }
            }
        }
        // The differential morphs (+0xC0, 0x34 bytes: name, then points at +0x1C).
        let differential = NameSlot {
            vector: 0xc0,
            size: 0x34,
            shift: None,
            string: 0,
        };
        let mut i = 0u32;
        while i < h(e, 0x1c) {
            let length = local(0xe4);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    differential,
                    i,
                    length,
                    local(0xfc),
                    local(0x19c),
                    true,
                )
            {
                return false;
            }
            let scale = local(0xe0);
            if !read(e, file, scale.addr(), 4, 1) {
                return give_up(e, file);
            }
            let morph = differential.address(e, mesh, i);
            let zero = 0.0f32.to_bits();
            e.call(
                RESIZE_POINTS,
                &args![morph + 0x1c, h(e, 0), zero, zero, zero],
            );
            let offsets = local(0x10c);
            let mut j = 0u32;
            while j < h(e, 0) {
                if !read(e, file, offsets.addr(), 2, 3) {
                    return give_up(e, file);
                }
                let scale_bits = e.mem.u32(scale.addr());
                let scaled = |e: &Engine, at: u32| {
                    let value = e.mem.i16(offsets.addr() + at) as i32;
                    x87_store32(x87_mul(x87_m32(scale_bits), f64::from(value)))
                };
                let z = scaled(e, 4);
                let y = scaled(e, 2);
                let x = scaled(e, 0);
                let (x, y, z) = (x87_quiet_bits(x), x87_quiet_bits(y), x87_quiet_bits(z));
                let morph = differential.address(e, mesh, i);
                let points = morph.wrapping_add(0x1c);
                let count = count_div(e, points, 0xc);
                let point = element(e, points, j, count, 0xc);
                e.mem.set_u32(point, x);
                e.mem.set_u32(point + 4, y);
                e.mem.set_u32(point + 8, z);
                j += 1;
            }
            i += 1;
        }
        // The statistical morphs (+0xD8, 0x38 bytes: name, first vertex at
        // +0x1C, vertex indices at +0x20).
        let statistical = NameSlot {
            vector: 0xd8,
            size: 0x38,
            shift: None,
            string: 0,
        };
        let mut offset = h(e, 0);
        let mut i = 0u32;
        while i < h(e, 0x20) {
            let count_slot = local(0x110);
            if !read(e, file, count_slot.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(count_slot.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    statistical,
                    i,
                    length,
                    local(0x128),
                    local(0x1cc),
                    true,
                )
            {
                return false;
            }
            let morph = statistical.address(e, mesh, i);
            e.mem.set_u32(morph.wrapping_add(0x1c), offset);
            if !read(e, file, count_slot.addr(), 4, 1) {
                return give_up(e, file);
            }
            let morph = statistical.address(e, mesh, i);
            let n = e.mem.u32(count_slot.addr());
            e.call(RESIZE_WORDS, &args![morph + 0x20, n, 0u32]);
            if e.mem.u32(count_slot.addr()) != 0 {
                let morph = statistical.address(e, mesh, i);
                let indices = e
                    .call(WORD_AT, &args![morph.wrapping_add(0x20), 0u32])
                    .u32();
                let n = e.mem.u32(count_slot.addr());
                if !read(e, file, indices, 4, n) {
                    e.call(FILE_DESTRUCT, &args![file]);
                    return false;
                }
            }
            offset = offset.wrapping_add(e.mem.u32(count_slot.addr()));
            i += 1;
        }
        if offset != h(e, 0).wrapping_add(h(e, 0x24)) {
            let copy = e.with_stack(0x1c, |e, text| {
                e.call(STRING_COPY, &args![text, name]);
                words_of(e, text, 0x1c)
            });
            let mut words = vec![1u32];
            words.extend(copy);
            e.call(SET_LAST_ERROR, &words);
            e.call(FILE_DESTRUCT, &args![file]);
            return false;
        }
        e.call(FILE_DESTRUCT, &args![file]);
        true
    })
}

/// `_invalid_parameter_noinfo` (`00ec7c56`): the library's debug checks.
fn check(e: &mut Engine, ok: bool) {
    if !ok {
        e.call(INVALID_INDEX, &[]);
    }
}

/// The checked iterator's proxy for `pointer` in the vector at `vector` (first +0xC, last
/// +0x10): `pointer` must lie within it.
fn checked_proxy(e: &mut Engine, vector: u32, pointer: u32) -> u32 {
    let ok = vector != 0
        && e.mem.u32(vector.wrapping_add(0xc)) <= pointer
        && pointer <= e.mem.u32(vector.wrapping_add(0x10));
    check(e, ok);
    e.mem.u32(vector)
}

/// The 16-byte-element vector's `erase(first, last)` (`00abac80`, `thiscall`: the result and
/// the two iterators' words).
const ERASE_ROWS: u32 = 0x00ab_ac80;

// Translated from 00ab9dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 16-byte-element vector's `clear()` (`thiscall`): erases
/// (`00abac80`) from the checked begin iterator to the checked end iterator.
pub fn fn_00ab9dc0(e: &mut Engine, this: Ptr) {
    let vector = this.addr();
    let last = e.mem.u32(vector.wrapping_add(0x10));
    let end_proxy = checked_proxy(e, vector, last);
    let first = e.mem.u32(vector.wrapping_add(0xc));
    let begin_proxy = checked_proxy(e, vector, first);
    e.with_stack(8, |e, result| {
        e.call(
            ERASE_ROWS,
            &args![this, result, begin_proxy, first, end_proxy, last],
        );
    });
}

/// `FutBinaryFileC`'s constructor (`00ac3df0`, `thiscall`: the file id string by value) and
/// `FutBinaryFileC::openForOverwrite` (Xbox PDB, `00ac4910`, `thiscall`: the name string by
/// value; answers `AL`).
const FUT_FILE_CONSTRUCT: u32 = 0x00ac_3df0;
const FUT_FILE_OPEN_FOR_OVERWRITE: u32 = 0x00ac_4910;

// Translated from 00ab7e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ab7e30 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// `fffWriteTriFile` (Xbox PDB, cdecl: file name `std::string *`, mesh): a `FutBinaryFileC`
/// on the stack with the id `"FRTRI003"` (`00ac3df0`, given a string made from the text),
/// opened for overwriting (`openForOverwrite`, given a copy of the name); when that works, the
/// mesh is written (`fffWriteTriFile_ov2`, `00ab7f40`). The file is destroyed (`00aaaa80`)
/// either way. Returns whether it was opened and written.
pub fn fff_write_tri_file(e: &mut Engine, name: Ptr, mesh: u32) -> bool {
    e.with_stack(0x48, |e, file| {
        let magic = e.with_stack(0x1c, |e, text| {
            e.call(STRING_FROM_TEXT, &args![text, TEXT_MAGIC]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(magic);
        e.call(FUT_FILE_CONSTRUCT, &words);
        let path = e.with_stack(0x1c, |e, text| {
            e.call(STRING_TIDY, &args![text, 0u32, 0u32]);
            let npos = e.global::<u32>(NPOS);
            e.call(STRING_ASSIGN, &args![text, name, 0u32, npos]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(path);
        if e.call(FUT_FILE_OPEN_FOR_OVERWRITE, &words).u8() == 0 {
            e.call(FILE_DESTRUCT_BASE, &args![file]);
            return false;
        }
        let written = fff_write_tri_file_ov2(e, file, mesh);
        e.call(FILE_DESTRUCT_BASE, &args![file]);
        written
    })
}

/// `FffTriFileDataS::numModelVtx` (Xbox PDB, `0065a5a0`, `thiscall` on the mesh),
/// `FutBinaryFileC::writeHeader` (Xbox PDB, `00ac5220`, `thiscall`: buffer, size, count;
/// answers `AL`) and the file's write (its slot +0xc, `thiscall`: buffer, size, count; answers
/// `AL`).
const NUM_MODEL_VERTICES: u32 = 0x0065_a5a0;
const FILE_WRITE_HEADER: u32 = 0x00ac_5220;
const SLOT_WRITE: u32 = 0xc;
/// `_fabs` (`00ec6cde`, cdecl: a `double`) and `00ec6940` (cdecl: a `double`; its library match
/// is unclear), both answering in `ST0`, and `_ftol2` (`00ec62c0`, the value in `ST0`).
const FABS: u32 = 0x00ec_6cde;
const ROUND_00EC6940: u32 = 0x00ec_6940;
const FTOL2: u32 = 0x00ec_62c0;
/// `32767.0` and `0.5` (`double`s): the offsets' scale and rounding.
const OFFSET_RANGE: u32 = 0x0102_9790;
const HALF: u32 = 0x0101_1588;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`: the block).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `"Invalid morph data during TRI file save."`.
const TEXT_INVALID_MORPHS: u32 = 0x010a_2d60;

/// The file's write (slot +0xc); whether it worked.
fn write(e: &mut Engine, file: Ptr, buffer: u32, size: u32, count: u32) -> bool {
    e.vcall(file.addr(), SLOT_WRITE, &args![buffer, size, count])
        .u8()
        != 0
}

/// A `double` as its two argument words, low first.
fn double_words(value: f64) -> [u32; 2] {
    let bits = value.to_bits();
    [bits as u32, (bits >> 32) as u32]
}

/// Writes a name: its characters (from the string `string` bytes into the entry `element`
/// gives, the entry's check made again for each character) into the byte vector `buffer`
/// constructed with `length` bytes (`00aba0d0`), a 0 at `length - 1`, then the bytes
/// (`write`, size 1, count `length`). Answers whether the write worked; the vector is left
/// for the caller.
fn write_name(
    e: &mut Engine,
    file: Ptr,
    buffer: Ptr,
    length: u32,
    string: u32,
    element: &dyn Fn(&mut Engine) -> u32,
) -> bool {
    e.call(BUFFER_CONSTRUCT, &args![buffer, length]);
    let size = |e: &Engine| {
        e.mem
            .u32(buffer.addr() + 0x10)
            .wrapping_sub(e.mem.u32(buffer.addr() + 0xc))
    };
    let mut k = 0u32;
    while k < length.wrapping_sub(1) {
        let text = element(e).wrapping_add(string);
        if k > e.mem.u32(text.wrapping_add(0x14)) {
            e.call(INVALID_INDEX, &[]);
        }
        let chars = if e.mem.u32(text.wrapping_add(0x18)) < 0x10 {
            text.wrapping_add(4)
        } else {
            e.mem.u32(text.wrapping_add(4))
        };
        if k >= size(e) {
            e.call(INVALID_INDEX, &[]);
        }
        let c = e.mem.u8(chars.wrapping_add(k));
        let data = e.mem.u32(buffer.addr() + 0xc);
        e.mem.set_u8(data.wrapping_add(k), c);
        k += 1;
    }
    let last = length.wrapping_sub(1);
    if last >= size(e) {
        e.call(INVALID_INDEX, &[]);
    }
    let data = e.mem.u32(buffer.addr() + 0xc);
    e.mem.set_u8(data.wrapping_add(last), 0);
    if size(e) == 0 {
        e.call(INVALID_INDEX, &[]);
    }
    let data = e.mem.u32(buffer.addr() + 0xc);
    write(e, file, data, 1, length)
}

/// The byte vector's release as the morph loops do it: its tidy (`00abc0a0`), then the
/// memory manager's free of its proxy (inline).
fn release_inline(e: &mut Engine, buffer: Ptr) {
    e.call(BUFFER_RELEASE, &args![buffer]);
    let proxy = e.mem.u32(buffer.addr());
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, proxy]);
}

/// Writes the first `count` elements of `size` bytes of the vector at `vector` (after the
/// non-empty check with the count `elements` gives); whether it worked.
fn write_elements(
    e: &mut Engine,
    file: Ptr,
    vector: u32,
    elements: u32,
    size: u32,
    count: u32,
) -> bool {
    let first = first_element(e, vector, elements);
    write(e, file, first, size, count)
}

// Translated from 00ab7f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ab7f40 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// `fffWriteTriFile_ov2` (Xbox PDB, cdecl: the open `FutBinaryFileC`, the mesh): writes a
/// FaceGen `.tri` file. The header (0x38 bytes, `writeHeader`): the model vertex count
/// (`numModelVtx`), the triangle, quad, named-entry, modifier, differential and statistical
/// morph counts, the UV count and flag 1 (set with no UV data, or with UV pairs for the model
/// vertices, or with all three UV vectors matching the vertex, triangle and quad counts, the
/// UV count then the pairs'), and the extra vertex count. Then the vertices, triangles,
/// quads, the named entries (a word, the name), the modifiers (a word, a point, the name),
/// the UV data, each differential morph (its name, the largest coordinate magnitude over the
/// model vertices / 32767 as the scale, then each coordinate `* (1 / scale) + 0.5` through
/// `00ec6940` and `_ftol2` as three `short`s) and each statistical morph (its name, its index
/// count and indices). The statistical index counts must add up to the extra vertex count,
/// else `futSetLastError(0xb, "Invalid morph data during TRI file save.")`. Every write
/// (slot +0xc) that fails answers false; the name vectors are freed on every path.
pub fn fff_write_tri_file_ov2(e: &mut Engine, file: Ptr, mesh: u32) -> bool {
    e.with_stack(0x100, |e, frame| {
        let header = frame.addr();
        let length = frame.addr() + 0x38;
        let shorts = frame.addr() + 0x3c;
        let scale_slot = frame.addr() + 0x44;
        let buffer = Ptr::<()>::new(frame.addr() + 0x48);
        let h = |e: &Engine, offset: u32| e.mem.u32(header + offset);
        e.call(MEMSET, &args![header, 0u32, 0x38u32]);
        let model = e.call(NUM_MODEL_VERTICES, &args![mesh]).u32();
        e.mem.set_u32(header, model);
        let counts = [
            (4u32, count_div(e, mesh + 0x18, 0xc)),
            (8, count_sar(e, mesh + 0x30, 4)),
            (0xc, count_sar(e, mesh + 0x90, 5)),
            (0x10, count_div(e, mesh + 0xa8, 0x2c)),
            (0x1c, count_div(e, mesh + 0xc0, 0x34)),
            (0x20, count_div(e, mesh + 0xd8, 0x38)),
        ];
        for (offset, count) in counts {
            e.mem.set_u32(header + offset, count);
        }
        let vertices = count_div(e, mesh, 0xc);
        let model = e.call(NUM_MODEL_VERTICES, &args![mesh]).u32();
        e.mem.set_u32(header + 0x24, vertices.wrapping_sub(model));
        let flags = h(e, 0x18);
        if count_sar(e, mesh + 0x48, 3) == h(e, 0)
            && count_div(e, mesh + 0x60, 0xc) == 0
            && count_sar(e, mesh + 0x78, 4) == 0
        {
            e.mem.set_u32(header + 0x18, flags | 1);
            e.mem.set_u32(header + 0x14, 0);
        } else if count_sar(e, mesh + 0x48, 3) != 0
            && count_div(e, mesh + 0x60, 0xc) == h(e, 4)
            && count_sar(e, mesh + 0x78, 4) == h(e, 8)
        {
            e.mem.set_u32(header + 0x18, flags | 1);
            let pairs = count_sar(e, mesh + 0x48, 3);
            e.mem.set_u32(header + 0x14, pairs);
        } else {
            e.mem.set_u32(header + 0x18, flags & 0xffff_fffe);
            e.mem.set_u32(header + 0x14, 0);
        }
        if e.call(FILE_WRITE_HEADER, &args![file, header, 0x38u32, 1u32])
            .u8()
            == 0
        {
            return false;
        }
        let all = h(e, 0).wrapping_add(h(e, 0x24));
        if all != 0 {
            let count = count_div(e, mesh, 0xc);
            if !write_elements(e, file, mesh, count, 0xc, all) {
                return false;
            }
        }
        if h(e, 4) != 0 {
            let count = count_div(e, mesh + 0x18, 0xc);
            if !write_elements(e, file, mesh + 0x18, count, 0xc, h(e, 4)) {
                return false;
            }
        }
        if h(e, 8) != 0 {
            let count = count_sar(e, mesh + 0x30, 4);
            if !write_elements(e, file, mesh + 0x30, count, 0x10, h(e, 8)) {
                return false;
            }
        }
        // The named entries (+0x90, 0x20 bytes: a word, then the name at +4).
        let mut i = 0u32;
        while i < h(e, 0xc) {
            let named = move |e: &mut Engine| {
                let count = count_sar(e, mesh + 0x90, 5);
                element(e, mesh + 0x90, i, count, 0x20)
            };
            let entry = named(e);
            if !write(e, file, entry, 4, 1) {
                return false;
            }
            let entry = named(e);
            let n = e.mem.u32(entry + 0x18).wrapping_add(1);
            e.mem.set_u32(length, n);
            if !write(e, file, length, 4, 1) {
                return false;
            }
            let n = e.mem.u32(length);
            let written = write_name(e, file, buffer, n, 4, &named);
            release_buffer(e, buffer);
            if !written {
                return false;
            }
            i += 1;
        }
        // The modifiers (+0xA8, 0x2C bytes: a word, a point, then the name at +0x10).
        let mut i = 0u32;
        while i < h(e, 0x10) {
            let modifier = move |e: &mut Engine| {
                let count = count_div(e, mesh + 0xa8, 0x2c);
                element(e, mesh + 0xa8, i, count, 0x2c)
            };
            let entry = modifier(e);
            if !write(e, file, entry, 4, 1) {
                return false;
            }
            let entry = modifier(e);
            if !write(e, file, entry + 4, 0xc, 1) {
                return false;
            }
            let entry = modifier(e);
            let n = e.mem.u32(entry + 0x24).wrapping_add(1);
            e.mem.set_u32(length, n);
            if !write(e, file, length, 4, 1) {
                return false;
            }
            let n = e.mem.u32(length);
            let written = write_name(e, file, buffer, n, 0x10, &modifier);
            release_buffer(e, buffer);
            if !written {
                return false;
            }
            i += 1;
        }
        if h(e, 0x18) & 1 != 0 {
            if h(e, 0x14) == 0 {
                let count = count_sar(e, mesh + 0x48, 3);
                if !write_elements(e, file, mesh + 0x48, count, 8, h(e, 0)) {
                    return false;
                }
            } else {
                let count = count_sar(e, mesh + 0x48, 3);
                if !write_elements(e, file, mesh + 0x48, count, 8, h(e, 0x14)) {
                    return false;
                }
                let count = count_div(e, mesh + 0x60, 0xc);
                if !write_elements(e, file, mesh + 0x60, count, 0xc, h(e, 4)) {
                    return false;
                }
                let count = count_sar(e, mesh + 0x78, 4);
                if !write_elements(e, file, mesh + 0x78, count, 0x10, h(e, 8)) {
                    return false;
                }
            }
        }
        // The differential morphs (+0xC0, 0x34 bytes: the name, then the points at +0x1C).
        let range: f64 = e.global(OFFSET_RANGE);
        let half: f64 = e.global(HALF);
        let mut i = 0u32;
        while i < h(e, 0x1c) {
            let morph = move |e: &mut Engine| {
                let count = count_div(e, mesh + 0xc0, 0x34);
                element(e, mesh + 0xc0, i, count, 0x34)
            };
            let entry = morph(e);
            let n = e.mem.u32(entry + 0x14).wrapping_add(1);
            e.mem.set_u32(length, n);
            if !write(e, file, length, 4, 1) {
                return false;
            }
            let n = e.mem.u32(length);
            if !write_name(e, file, buffer, n, 0, &morph) {
                release_buffer(e, buffer);
                return false;
            }
            let point = |e: &mut Engine, v: u32| {
                let points = morph(e) + 0x1c;
                let count = count_div(e, points, 0xc);
                element(e, points, v, count, 0xc)
            };
            let magnitude = |e: &mut Engine, v: u32, at: u32| {
                let p = point(e, v);
                let value = x87_load32(e.mem.u32(p + at));
                let absolute = e.call(FABS, &double_words(value)).f64();
                x87_quiet_bits(x87_store32(absolute))
            };
            let mut scale = x87_store32(0.0);
            let mut v = 0u32;
            while v < h(e, 0) {
                for at in [0u32, 4, 8] {
                    let candidate = magnitude(e, v, at);
                    // FLD scale; FLD candidate; FCOMPP; TEST AH,0x41; JNZ: only a larger one.
                    if x87_load32(candidate) > x87_load32(scale) {
                        scale = magnitude(e, v, at);
                    }
                }
                v += 1;
            }
            scale = x87_store32(x87_div(x87_load32(scale), range));
            e.mem.set_u32(scale_slot, scale);
            if !write(e, file, scale_slot, 4, 1) {
                release_inline(e, Ptr::new(buffer.addr()));
                return false;
            }
            let scale = x87_store32(x87_div(1.0, x87_load32(e.mem.u32(scale_slot))));
            e.mem.set_u32(scale_slot, scale);
            let mut v = 0u32;
            while v < h(e, 0) {
                for (k, at) in [0u32, 4, 8].into_iter().enumerate() {
                    let p = point(e, v);
                    let value = x87_add(
                        x87_mul(
                            x87_load32(e.mem.u32(p + at)),
                            x87_m32(e.mem.u32(scale_slot)),
                        ),
                        half,
                    );
                    let rounded = e.call(ROUND_00EC6940, &double_words(value)).f64();
                    let whole = e.call(FTOL2, &double_words(rounded)).u32();
                    e.mem.set_u16(shorts + 2 * k as u32, whole as u16);
                }
                if !write(e, file, shorts, 2, 3) {
                    release_inline(e, Ptr::new(buffer.addr()));
                    return false;
                }
                v += 1;
            }
            release_inline(e, Ptr::new(buffer.addr()));
            i += 1;
        }
        // The statistical morphs (+0xD8, 0x38 bytes: the name, then the indices at +0x20).
        let mut total = 0u32;
        let mut i = 0u32;
        while i < h(e, 0x20) {
            let morph = move |e: &mut Engine| {
                let count = count_div(e, mesh + 0xd8, 0x38);
                element(e, mesh + 0xd8, i, count, 0x38)
            };
            let entry = morph(e);
            let n = e.mem.u32(entry + 0x14).wrapping_add(1);
            e.mem.set_u32(length, n);
            if !write(e, file, length, 4, 1) {
                return false;
            }
            let n = e.mem.u32(length);
            if !write_name(e, file, buffer, n, 0, &morph) {
                release_buffer(e, buffer);
                return false;
            }
            let indices = morph(e) + 0x20;
            let count = count_sar(e, indices, 2);
            e.mem.set_u32(length, count);
            if !write(e, file, length, 4, 1) {
                release_inline(e, Ptr::new(buffer.addr()));
                return false;
            }
            let count = e.mem.u32(length);
            if count != 0 {
                let indices = morph(e) + 0x20;
                let elements = count_sar(e, indices, 2);
                if !write_elements(e, file, indices, elements, 4, count) {
                    release_inline(e, Ptr::new(buffer.addr()));
                    return false;
                }
            }
            total = total.wrapping_add(e.mem.u32(length));
            release_inline(e, Ptr::new(buffer.addr()));
            i += 1;
        }
        if total != h(e, 0x24) {
            let text = e.with_stack(0x1c, |e, text| {
                e.call(STRING_FROM_TEXT, &args![text, TEXT_INVALID_MORPHS]);
                words_of(e, text, 0x1c)
            });
            let mut words = vec![0xbu32];
            words.extend(text);
            e.call(SET_LAST_ERROR, &words);
            return false;
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_46e0, fn_00ab46e0(Ptr)),
        entry!(0x00ab_a370, fn_00aba370(Ptr, u32, u8)),
        entry!(0x00ab_5ec0, fff_read_tri_file(Ptr, u32) -> bool),
        entry!(0x00ab_9dc0, fn_00ab9dc0(Ptr)),
        entry!(0x00ab_7e30, fff_write_tri_file(Ptr, u32) -> bool),
        entry!(0x00ab_7f40, fff_write_tri_file_ov2(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn nothing_happens() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00ab_46e0, &args![0x1234u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn resize_grows_or_erases() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(NPOS, 0xffff_ffffu32);
        e.register(STRING_ERASE, |_, _| Ret::default());
        e.register(STRING_APPEND_CHARS, |_, _| Ret::default());
        let s = e.mem.alloc(0x1c);
        e.mem.set_u32(s + 0x14, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ab_a370, &args![s, 8u32, 0x120u32]);
        e.call(0x00ab_a370, &args![s, 5u32, 0x20u32]);
        e.call(0x00ab_a370, &args![s, 2u32, 0x20u32]);
        assert_eq!(calls_to(&e, STRING_APPEND_CHARS), vec![vec![s, 3, 0x20]]);
        assert_eq!(
            calls_to(&e, STRING_ERASE),
            vec![vec![s, 5, 0xffff_ffff], vec![s, 2, 0xffff_ffff]]
        );
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The 0x34-byte entry's destructor (`0065ed70`, `thiscall`), which
    /// `00abb700` (run for real) runs on its value.
    const DESTROY_34: u32 = 0x0065_ed70;

    /// `00abbae0`'s insert of `count` copies (`00abfa90`) and the 0x38-byte
    /// entry's destructor (`0065edd0`).
    const INSERT_38: u32 = 0x00ab_fa90;
    const DESTRUCT_38: u32 = 0x0065_edd0;

    /// Makes `vector` (first +0xC, last +0x10) hold `count` zeroed elements
    /// of `size` bytes.
    fn grow(e: &mut Engine, vector: u32, count: u32, size: u32) {
        let first = e.mem.alloc(count * size + 4);
        e.mem.set_u32(vector + 0xc, first);
        e.mem.set_u32(vector + 0x10, first + count * size);
    }

    /// Doubles for the file (reading `stream` in order), the vectors and
    /// strings; the header is `header` (0x38 bytes).
    fn tri_engine(header: [u32; 14], stream: Vec<u8>, opens: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(NPOS, 0xffff_ffffu32);
        let stream = Rc::new(RefCell::new((stream, 0usize)));
        e.register_double(FILE_READ, move |e, a| {
            let (bytes, at) = &mut *stream.borrow_mut();
            let n = (a[2] * a[3]) as usize;
            let chunk = bytes[*at..*at + n].to_vec();
            *at += n;
            e.mem.write(a[1], &chunk);
            ret(1)
        });
        e.register_double(FILE_READ_HEADER, move |e, a| {
            for (i, word) in header.iter().enumerate() {
                e.mem.set_u32(a[1] + i as u32 * 4, *word);
            }
            ret(1)
        });
        e.register_double(FILE_OPEN_FOR_READ, move |_, _| ret(u32::from(opens)));
        for f in [
            MEMSET,
            STRING_FROM_TEXT,
            STRING_TIDY,
            STRING_ASSIGN,
            STRING_COPY,
            FILE_CONSTRUCT,
            FILE_CLOSE,
            FILE_DESTRUCT_BASE,
            FILE_DESTRUCT,
            MAKE_2C,
            MAKE_34,
            MAKE_38,
            DESTROY_34,
            CLEAR_60,
            CLEAR_78,
            BUFFER_RELEASE,
            BUFFER_DESTRUCT,
            INVALID_INDEX,
            SET_LAST_ERROR,
            STRING_ERASE,
            RESIZE_NAMED,
            RESIZE_2C,
            RESIZE_ROWS,
            RESIZE_PAIRS,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(RESIZE_POINTS, |e, a| {
            grow(e, a[0], a[1], 0xc);
            Ret::default()
        });
        // The 0x38-byte resize (00abbae0) runs on its value in place: its insert (this, proxy,
        // where, count, value) grows the vector and the value's destructor is quiet.
        e.register(INSERT_38, |e, a| {
            grow(e, a[0], a[3], 0x38);
            Ret::default()
        });
        e.register(DESTRUCT_38, |_, _| Ret::default());
        e.register(RESIZE_WORDS, |e, a| {
            grow(e, a[0], a[1], 4);
            Ret::default()
        });
        e.register(WORD_AT, |e, a| ret(e.mem.u32(a[0] + 0xc) + a[1] * 4));
        e.register(BUFFER_CONSTRUCT, |e, a| {
            grow(e, a[0], a[1], 1);
            Ret::default()
        });
        // The string append sets the size (the text stays zeroed).
        e.register(STRING_APPEND_CHARS, |e, a| {
            let size = e.mem.u32(a[0] + 0x14) + a[1];
            e.mem.set_u32(a[0] + 0x14, size);
            Ret::default()
        });
        e
    }

    fn mesh(e: &mut Engine) -> u32 {
        e.mem.alloc(0xf0)
    }

    #[test]
    fn tri_file_that_does_not_open() {
        let mut e = tri_engine([0; 14], vec![], false);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ab_5ec0, &args![0x500u32, mesh]).bool());
        assert_eq!(calls_to(&e, FILE_CLOSE).len(), 1);
        assert_eq!(calls_to(&e, FILE_DESTRUCT_BASE).len(), 1);
        assert!(calls_to(&e, FILE_READ_HEADER).is_empty());
        let open = calls_to(&e, FILE_OPEN_FOR_READ);
        assert_eq!(open[0].len(), 9);
        assert_eq!(open[0][8], 0);
    }

    #[test]
    fn tri_file_with_vertices_and_a_differential_morph() {
        let mut header = [0u32; 14];
        header[0] = 2;
        header[7] = 1;
        let mut stream = vec![];
        for v in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            stream.extend(v.to_le_bytes());
        }
        stream.extend(3u32.to_le_bytes());
        stream.extend(b"ab\0");
        stream.extend(0.5f32.to_le_bytes());
        for s in [2i16, 4, 6, -2, 0, 8] {
            stream.extend(s.to_le_bytes());
        }
        let mut e = tri_engine(header, stream, true);
        let mesh = mesh(&mut e);
        // The mesh already holds the one 0x34-byte entry, so the real
        // `00abb700` keeps it and only destroys its value.
        grow(&mut e, mesh + 0xc0, 1, 0x34);
        e.call_log = Some(vec![]);
        assert!(fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let vertices = e.mem.u32(mesh + 0xc);
        let read: Vec<f32> = (0..6).map(|i| e.mem.f32(vertices + i * 4)).collect();
        assert_eq!(read, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let morph = e.mem.u32(mesh + 0xc0 + 0xc);
        assert_eq!(e.mem.u32(morph + 0x14), 2);
        assert_eq!(e.mem.bytes(morph + 4, 2), b"ab".to_vec());
        let points = e.mem.u32(morph + 0x1c + 0xc);
        let scaled: Vec<f32> = (0..6).map(|i| e.mem.f32(points + i * 4)).collect();
        assert_eq!(scaled, vec![1.0, 2.0, 3.0, -1.0, 0.0, 4.0]);
        assert_eq!(calls_to(&e, FILE_DESTRUCT).len(), 1);
        assert!(calls_to(&e, SET_LAST_ERROR).is_empty());
        assert!(calls_to(&e, INVALID_INDEX).is_empty());
    }

    #[test]
    fn tri_file_whose_counts_do_not_add_up() {
        let mut header = [0u32; 14];
        header[8] = 1;
        let mut stream = vec![];
        stream.extend(0u32.to_le_bytes());
        stream.extend(1u32.to_le_bytes());
        stream.extend(7u32.to_le_bytes());
        let mut e = tri_engine(header, stream, true);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let morph = e.mem.u32(mesh + 0xd8 + 0xc);
        assert_eq!(e.mem.u32(morph + 0x1c), 0);
        let indices = e.mem.u32(morph + 0x20 + 0xc);
        assert_eq!(e.mem.u32(indices), 7);
        let error = calls_to(&e, SET_LAST_ERROR);
        assert_eq!(error.len(), 1);
        assert_eq!(error[0][0], 1);
        assert_eq!(error[0].len(), 8);
        assert_eq!(calls_to(&e, FILE_DESTRUCT).len(), 1);
    }

    /// 00ab61a4..00ab61ca: the default 0x34-byte entry (`00aba270`) is
    /// built in the argument area of `00abb700`, which resizes with it and
    /// destroys it there (`0065ed70`), not in a copy of its words. The
    /// freed check is on because under it stack locals are separate
    /// blocks: a copy of the words would be destroyed somewhere else.
    /// Without it the copy's area reuses the ended local's address, so the
    /// copy would be destroyed at the built address by chance.
    #[test]
    fn tri_file_destroys_the_default_entry_where_it_was_built() {
        let mut header = [0u32; 14];
        header[8] = 1;
        let mut stream = vec![];
        for word in [0u32, 1, 7] {
            stream.extend(word.to_le_bytes());
        }
        let mut e = tri_engine(header, stream, true);
        e.mem.enable_freed_check(false);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let built = calls_to(&e, MAKE_34);
        assert_eq!(built.len(), 1);
        assert_eq!(calls_to(&e, DESTROY_34), built);
    }

    /// 00ab61d8..00ab61fe: the default 0x38-byte entry (`00aba2f0`) is
    /// built in `00abbae0`'s argument area, so the proxy of its 4-byte
    /// vector (+0x20, `00aba4f0`: pointing back at that vector) names the
    /// vector of the value the insert (`00abfa90`) copies from, and
    /// `00abbae0` destroys the entry there (`0065edd0`). Built in a local
    /// whose words were passed after it ended, the value the resize used was
    /// a copy elsewhere. The freed check is on because under it stack
    /// locals are separate blocks: a copy of the words would be read and
    /// destroyed somewhere else. Without it the copy's area reuses the ended
    /// local's address, so the stale proxy would name the copy by chance.
    #[test]
    fn tri_file_grows_the_0x38_entries_from_the_entry_its_proxy_names() {
        // One statistical morph (header +0x20) whose vertex count does not add up (as in the
        // test above): the read stops after it.
        let mut header = [0u32; 14];
        header[8] = 1;
        let mut stream = vec![];
        for word in [0u32, 1, 7] {
            stream.extend(word.to_le_bytes());
        }
        let mut e = tri_engine(header, stream, true);
        e.mem.enable_freed_check(false);
        e.register(MAKE_38, |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0] + 0x20);
            e.mem.set_u32(a[0] + 0x20, proxy);
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(INSERT_38, move |e, a| {
            let proxy = e.mem.u32(a[4] + 0x20);
            sink.borrow_mut().push((a[0], a[3], a[4], e.mem.u32(proxy)));
            grow(e, a[0], a[3], 0x38);
            Ret::default()
        });
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let built = calls_to(&e, MAKE_38)[0][0];
        assert_eq!(*seen.borrow(), vec![(mesh + 0xd8, 1, built, built + 0x20)]);
        assert_eq!(calls_to(&e, DESTRUCT_38), vec![vec![built]]);
    }
}

#[cfg(test)]
mod n062_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Writes = Rc<RefCell<Vec<(u32, u32, Vec<u8>)>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn clear_erases_everything() {
        let mut e = Engine::new();
        quiet(&mut e, &[ERASE_ROWS, INVALID_INDEX]);
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector, 0x77);
        e.mem.set_u32(vector + 0xc, 0x1000);
        e.mem.set_u32(vector + 0x10, 0x1020);
        e.call_log = Some(vec![]);
        e.call(0x00ab_9dc0, &args![vector]);
        let erase = calls(&e, ERASE_ROWS);
        assert_eq!(erase.len(), 1);
        assert_eq!(erase[0][0], vector);
        assert_eq!(erase[0][2..], [0x77, 0x1000, 0x77, 0x1020]);
        assert!(calls(&e, INVALID_INDEX).is_empty());
    }

    /// A vector (first +0xC, last +0x10) at `at` over `words`.
    fn fill_vector(e: &mut Engine, at: u32, words: &[u32]) {
        let block = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *w);
        }
        e.mem.set_u32(at + 0xc, block);
        e.mem.set_u32(at + 0x10, block + 4 * words.len() as u32);
    }

    /// An inline string at `at`.
    fn set_string(e: &mut Engine, at: u32, text: &[u8]) {
        e.mem.write(at + 4, text);
        e.mem.set_u32(at + 0x14, text.len() as u32);
        e.mem.set_u32(at + 0x18, 0xf);
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// A mesh of 2 model vertices and 1 extra, a triangle, a named entry `"ab"`, UV pairs for
    /// the model vertices, a differential morph `"m"` and a statistical morph `"s"` of one
    /// index; a file whose writes are recorded (failing from write `fail_at` on).
    fn writer(fail_at: usize) -> (Engine, u32, u32, Writes) {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_9000, 0x1000);
        e.mem.set_f64(OFFSET_RANGE, 32767.0);
        e.mem.set_f64(HALF, 0.5);
        let mesh = e.mem.alloc(0xf0);
        fill_vector(
            &mut e,
            mesh,
            &bits(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
        );
        fill_vector(&mut e, mesh + 0x18, &[0, 1, 2]);
        let named = e.mem.alloc(0x20);
        e.mem.set_u32(named, 0x55);
        set_string(&mut e, named + 4, b"ab");
        e.mem.set_u32(mesh + 0x90 + 0xc, named);
        e.mem.set_u32(mesh + 0x90 + 0x10, named + 0x20);
        fill_vector(&mut e, mesh + 0x48, &[1, 2, 3, 4]);
        let diff = e.mem.alloc(0x34);
        set_string(&mut e, diff, b"m");
        fill_vector(
            &mut e,
            diff + 0x1c,
            &bits(&[1.0, -2.0, 0.5, 0.25, 0.0, 0.0]),
        );
        e.mem.set_u32(mesh + 0xc0 + 0xc, diff);
        e.mem.set_u32(mesh + 0xc0 + 0x10, diff + 0x34);
        let stat = e.mem.alloc(0x38);
        set_string(&mut e, stat, b"s");
        fill_vector(&mut e, stat + 0x20, &[7]);
        e.mem.set_u32(mesh + 0xd8 + 0xc, stat);
        e.mem.set_u32(mesh + 0xd8 + 0x10, stat + 0x38);
        e.register(NUM_MODEL_VERTICES, |_, _| 2u32.into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![0u8; a[2] as usize]);
            Ret::default()
        });
        e.register(FILE_WRITE_HEADER, |_, _| 1u32.into_ret());
        e.register(BUFFER_CONSTRUCT, |e, a| {
            let block = e.mem.alloc(a[1].max(1));
            e.mem.set_u32(a[0] + 0xc, block);
            e.mem.set_u32(a[0] + 0x10, block + a[1]);
            Ret::default()
        });
        e.register(FABS, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).abs(),
            ..Ret::default()
        });
        e.register(ROUND_00EC6940, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).floor(),
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32 as u32).into_ret()
        });
        quiet(
            &mut e,
            &[
                BUFFER_RELEASE,
                BUFFER_DESTRUCT,
                MEMORY_MANAGER_FREE,
                INVALID_INDEX,
                SET_LAST_ERROR,
                STRING_FROM_TEXT,
            ],
        );
        let file = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(file, vtable);
        e.mem.set_u32(vtable + SLOT_WRITE, 0x0350_0000);
        let writes: Writes = Rc::new(RefCell::new(vec![]));
        let log = writes.clone();
        e.register_double(0x0350_0000, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2] * a[3]);
            let mut log = log.borrow_mut();
            log.push((a[2], a[3], bytes));
            u32::from(log.len() < fail_at).into_ret()
        });
        e.call_log = Some(vec![]);
        (e, file, mesh, writes)
    }

    #[test]
    fn writer_writes_every_part() {
        let (mut e, file, mesh, writes) = writer(usize::MAX);
        let header = Rc::new(RefCell::new(vec![]));
        let seen = header.clone();
        e.register_double(FILE_WRITE_HEADER, move |e, a| {
            *seen.borrow_mut() = (0..14).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            1u32.into_ret()
        });
        assert!(e.call(0x00ab_7f40, &args![file, mesh]).bool());
        assert_eq!(
            *header.borrow(),
            vec![2, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]
        );
        let writes = writes.borrow();
        let shapes: Vec<(u32, u32)> = writes.iter().map(|w| (w.0, w.1)).collect();
        assert_eq!(
            shapes,
            vec![
                (0xc, 3),
                (0xc, 1),
                (4, 1),
                (4, 1),
                (1, 3),
                (8, 2),
                (4, 1),
                (1, 2),
                (4, 1),
                (2, 3),
                (2, 3),
                (4, 1),
                (1, 2),
                (4, 1),
                (4, 1),
            ]
        );
        assert_eq!(writes[4].2, b"ab\0".to_vec());
        assert_eq!(writes[7].2, b"m\0".to_vec());
        // The scale: the largest magnitude over 32767.
        let scale = (2.0f64 / 32767.0) as f32;
        assert_eq!(writes[8].2, scale.to_le_bytes().to_vec());
        let inverse = (1.0f64 / f64::from(scale)) as f32;
        let offset = |v: f32| (f64::from(v) * f64::from(inverse) + 0.5).floor() as i32 as i16;
        let first: Vec<u8> = [offset(1.0), offset(-2.0), offset(0.5)]
            .iter()
            .flat_map(|s| s.to_le_bytes())
            .collect();
        assert_eq!(writes[9].2, first);
        assert_eq!(writes[13].2, 1u32.to_le_bytes().to_vec());
        assert_eq!(writes[14].2, 7u32.to_le_bytes().to_vec());
        assert!(calls(&e, SET_LAST_ERROR).is_empty());
        assert!(calls(&e, INVALID_INDEX).is_empty());
        // The named entry's vector freed through the destructor, the morphs' inline.
        assert_eq!(calls(&e, BUFFER_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 2);
    }

    #[test]
    fn writer_stops_on_a_failed_write_or_bad_morphs() {
        // The named entry's name write fails: its vector is freed, false.
        let (mut e, file, mesh, writes) = writer(5);
        assert!(!e.call(0x00ab_7f40, &args![file, mesh]).bool());
        assert_eq!(writes.borrow().len(), 5);
        assert_eq!(calls(&e, BUFFER_RELEASE).len(), 1);
        assert_eq!(calls(&e, BUFFER_DESTRUCT).len(), 1);
        // The morph's scale write fails: freed inline.
        let (mut e, file, mesh, _) = writer(9);
        assert!(!e.call(0x00ab_7f40, &args![file, mesh]).bool());
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 1);
        // The header fails: nothing written.
        let (mut e, file, mesh, writes) = writer(usize::MAX);
        e.register(FILE_WRITE_HEADER, |_, _| Ret::default());
        assert!(!e.call(0x00ab_7f40, &args![file, mesh]).bool());
        assert!(writes.borrow().is_empty());
        // Statistical indices not adding up to the extra vertices: error 0xb.
        let (mut e, file, mesh, _) = writer(usize::MAX);
        e.register(NUM_MODEL_VERTICES, |_, _| 1u32.into_ret());
        let diff = e.mem.u32(mesh + 0xc0 + 0xc);
        e.mem.set_u32(mesh + 0xc0 + 0x10, diff);
        assert!(!e.call(0x00ab_7f40, &args![file, mesh]).bool());
        let error = calls(&e, SET_LAST_ERROR);
        assert_eq!(error.len(), 1);
        assert_eq!(error[0][0], 0xb);
        assert_eq!(calls(&e, STRING_FROM_TEXT)[0][1], TEXT_INVALID_MORPHS);
    }

    #[test]
    fn write_tri_file_opens_and_destroys_the_file() {
        let (mut e, _, mesh, _) = writer(usize::MAX);
        e.map(0x0101_7000, 0x1000);
        e.set_global(NPOS, u32::MAX);
        quiet(
            &mut e,
            &[
                FUT_FILE_CONSTRUCT,
                FILE_DESTRUCT_BASE,
                STRING_TIDY,
                STRING_ASSIGN,
            ],
        );
        e.register(FILE_WRITE_HEADER, |_, _| Ret::default());
        e.register(FUT_FILE_OPEN_FOR_OVERWRITE, |_, _| Ret::default());
        let name = e.mem.alloc(0x1c);
        assert!(!e.call(0x00ab_7e30, &args![name, mesh]).bool());
        let file = calls(&e, FUT_FILE_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, STRING_FROM_TEXT)[0][1], TEXT_MAGIC);
        assert_eq!(calls(&e, STRING_ASSIGN)[0][1..3], [name, 0]);
        assert_eq!(calls(&e, FILE_DESTRUCT_BASE), vec![vec![file]]);
        assert!(calls(&e, FILE_WRITE_HEADER).is_empty());
        // Opened: the mesh is written (the header fails here), the file destroyed.
        e.register(FUT_FILE_OPEN_FOR_OVERWRITE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ab_7e30, &args![name, mesh]).bool());
        assert_eq!(calls(&e, FILE_WRITE_HEADER).len(), 1);
        assert_eq!(calls(&e, FILE_DESTRUCT_BASE).len(), 1);
    }
}

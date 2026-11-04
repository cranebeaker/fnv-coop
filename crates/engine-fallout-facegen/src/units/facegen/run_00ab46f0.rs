//! `FaceGen/run_00ab46f0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Members of FaceGen's triangle mesh (the `"FRTRI003"` file's contents): ten library vectors
//! of 0x18 bytes each (first and last pointers at +0x0C, +0x10): +0x00 vertices (12 bytes),
//! +0x18 triangles (three indices), +0x30 quads (four indices), +0x48 8-byte elements, +0x60
//! 12-byte and +0x78 16-byte elements, then four vectors of named elements (+0x90 0x20 bytes,
//! +0xA8 0x2C bytes whose first word is a face index and whose next three floats are weights,
//! +0xC0 0x34 bytes, +0xD8 0x38 bytes).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    BSFaceGenBinaryFile, StringArg, STRING_ASSIGN, STRING_NPOS, VECTOR_FIRST, VECTOR_LAST,
};
use crate::units::facegen::run_00aba3c0::{
    check, fn_00abb700, resize_38_in_area, vector_size, ElementSize,
};

/// The resizes with a value (unit `run_00aba3c0`, all `thiscall`: the count and the value's
/// words): 12-byte elements (`00aba7b0`, three words), 16-byte (`00abaaa0`, four), 8-byte
/// (`00abad80`, two), 4-byte (`00aba5d0`, one) and 0x2C-byte (`00abb3c0`, eleven); and the
/// 0x20-byte element vector's `resize(count)` (`00ab9eb0`, unit `run_00ab9eb0`). The 0x34-byte
/// one (`00abb700`, thirteen words) and the 0x38-byte one (`00abbae0`, fourteen) take their
/// value in place, in the argument area it is built in: [`fn_00abb700`],
/// [`resize_38_in_area`].
const RESIZE_12: u32 = 0x00ab_a7b0;
const RESIZE_16: u32 = 0x00ab_aaa0;
const RESIZE_8: u32 = 0x00ab_ad80;
const RESIZE_4: u32 = 0x00ab_a5d0;
const RESIZE_2C: u32 = 0x00ab_b3c0;
const RESIZE_20: u32 = 0x00ab_9eb0;
/// The default constructors of the 0x2C-, 0x34- and 0x38-byte elements (`00aba220`,
/// `00aba270`, `00aba2f0`, `thiscall`; unit `run_00ab9eb0`), which build the resize value in
/// the argument area.
const DEFAULT_2C: u32 = 0x00ab_a220;
const DEFAULT_34: u32 = 0x00ab_a270;
const DEFAULT_38: u32 = 0x00ab_a2f0;
/// The element iterators: made from a pointer and its container (`00abbeb0`, `thiscall` on the
/// iterator: pointer, container), compared for equality (`00abbf00`, `thiscall`: the other;
/// answers a byte) and dereferenced (`00abbf50`, `thiscall`; answers the element's address);
/// the `++` of the 0x20-, 0x2C-, 0x34- and 0x38-byte element iterators (`00ab9f10`,
/// `00ab9f80`, `00ab9ff0`, `00aba060`, unit `run_00ab9eb0`).
const MAKE_ITERATOR: u32 = 0x00ab_beb0;
const ITERATORS_EQUAL: u32 = 0x00ab_bf00;
const DEREFERENCE: u32 = 0x00ab_bf50;
const INCREMENT_20: u32 = 0x00ab_9f10;
const INCREMENT_2C: u32 = 0x00ab_9f80;
const INCREMENT_34: u32 = 0x00ab_9ff0;
const INCREMENT_38: u32 = 0x00ab_a060;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The 12-byte element vector's `clear` (`00ab9cd0`) and `push_back` (`00ab9bc0`, the value's
/// address), both `thiscall`, unit `run_00ab9bc0`.
const CLEAR_12: u32 = 0x00ab_9cd0;
const PUSH_BACK_12: u32 = 0x00ab_9bc0;
/// The vector's product with a float (`00ac1120`, `thiscall`: result, float) and sum
/// (`00ac10c0`, `thiscall`: result, other); each answers the result.
const VECTOR_SCALE: u32 = 0x00ac_1120;
const VECTOR_ADD: u32 = 0x00ac_10c0;
/// The string from a C string (`0044a4c0`, `thiscall`: the characters), the TRI file id
/// `"FRTRI003"` (`010a2d54`), `FutBinaryFileC`'s constructor (`00ac3df0`, `thiscall`: the id
/// string by value), its check of a named file (`00ac3fe0`, `thiscall`: the address of a name
/// string; answers a byte) and its destructor (`00aaaa80`, `thiscall`, unit `fanctlfile`).
const STRING_FROM_C: u32 = 0x0044_a4c0;
const TRI_FILE_ID: u32 = 0x010a_2d54;
const FUT_BINARY_FILE_CONSTRUCT: u32 = 0x00ac_3df0;
const FUT_BINARY_FILE_CHECK: u32 = 0x00ac_3fe0;
const FUT_BINARY_FILE_DESTRUCT: u32 = 0x00aa_aa80;

/// The mesh's vectors (offsets in the mesh).
const VERTICES: u32 = 0x00;
const TRIANGLES: u32 = 0x18;
const QUADS: u32 = 0x30;
const NAMED_20: u32 = 0x90;
const WEIGHTED_POINTS: u32 = 0xa8;
const NAMED_34: u32 = 0xc0;
const NAMED_38: u32 = 0xd8;

/// The vector's first element after the check that it has one (`&v[0]`).
fn first_checked(e: &mut Engine, vector: u32, size: ElementSize) -> u32 {
    check(e, vector_size(e, vector, size) != 0);
    e.mem.u32(vector + VECTOR_FIRST)
}

/// One vector of plain elements copied as the code copies it: this mesh's vector is resized to
/// the source's count (`resize` with a value of `zeros` zero words), then the source's elements
/// are copied over (`memcpy` of this vector's new count, each start checked).
fn copy_plain(
    e: &mut Engine,
    vector: u32,
    source: u32,
    size: ElementSize,
    resize: u32,
    zeros: usize,
) {
    let count = vector_size(e, source, size);
    let mut words = vec![vector, count];
    words.resize(2 + zeros, 0);
    e.call(resize, &words);
    let count = vector_size(e, vector, size);
    let from = first_checked(e, source, size);
    let to = first_checked(e, vector, size);
    e.call(MEMCPY, &args![to, from, count.wrapping_mul(size.bytes())]);
}

/// Resizes `vector` to `count` with a value made by `construct` in the argument area (`words`
/// words), as the code passes it to `resize`.
fn resize_with_default(
    e: &mut Engine,
    vector: u32,
    count: u32,
    construct: u32,
    resize: u32,
    words: u32,
) {
    e.with_stack(4 * words, |e, value| {
        e.call(construct, &args![value]);
        let mut call = vec![vector, count];
        for i in 0..words {
            call.push(e.mem.u32(value.addr() + 4 * i));
        }
        e.call(resize, &call);
    });
}

/// Walks this vector and the source's together from their starts until this one's iterator
/// equals its end (`00abbeb0`, `00abbf00`), calling `body` with the two iterators and then
/// advancing both (`increment`).
fn for_each_pair(
    e: &mut Engine,
    vector: u32,
    source: u32,
    increment: u32,
    mut body: impl FnMut(&mut Engine, u32, u32),
) {
    e.with_stack(0x18, |e, iterators| {
        let target = iterators.addr();
        let from = iterators.addr() + 8;
        let end = iterators.addr() + 0x10;
        let first = e.mem.u32(vector + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![target, first, vector]);
        let first = e.mem.u32(source + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![from, first, source]);
        loop {
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(MAKE_ITERATOR, &args![end, last, vector]);
            if e.call(ITERATORS_EQUAL, &args![target, end]).u8() != 0 {
                break;
            }
            body(e, target, from);
            e.call(increment, &args![target]);
            e.call(increment, &args![from]);
        }
    });
}

fn element(e: &mut Engine, iterator: u32) -> u32 {
    e.call(DEREFERENCE, &args![iterator]).u32()
}

fn assign_string(e: &mut Engine, target: u32, source: u32) {
    let npos: u32 = e.global(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![target, source, 0u32, npos]);
}

/// The element vector at `offset` inside the elements of a `for_each_pair` walk: this one
/// resized to the source's count (`resize`, `zeros` zero words), then the source's elements
/// copied over.
#[allow(clippy::too_many_arguments)]
fn copy_inner(
    e: &mut Engine,
    target: u32,
    from: u32,
    offset: u32,
    size: ElementSize,
    resize: u32,
    zeros: usize,
) {
    let source = element(e, from).wrapping_add(offset);
    let count = vector_size(e, source, size);
    let vector = element(e, target).wrapping_add(offset);
    let mut words = vec![vector, count];
    words.resize(2 + zeros, 0);
    e.call(resize, &words);
    let vector = element(e, target).wrapping_add(offset);
    let count = vector_size(e, vector, size);
    let source = element(e, from).wrapping_add(offset);
    let source_first = first_checked(e, source, size);
    let vector = element(e, target).wrapping_add(offset);
    let target_first = first_checked(e, vector, size);
    e.call(
        MEMCPY,
        &args![target_first, source_first, count.wrapping_mul(size.bytes())],
    );
}

// Translated from 00ab46f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the mesh's assignment from another (`thiscall`, the source mesh).
/// The six plain vectors are resized to the source's counts (zero values) and their elements
/// copied (`memcpy`). The four vectors of named elements are resized to the source's counts
/// (`00ab9eb0`; the others with a default element built by `00aba220` / `00aba270` /
/// `00aba2f0`) and walked together with the source's: the 0x20-byte elements get the word and
/// the name (+4); the 0x2C-byte ones four words and the name (+0x10); the 0x34-byte ones the
/// name and their 12-byte vector at +0x1C (resized and copied); the 0x38-byte ones the name,
/// the word at +0x1C and their 4-byte vector at +0x20 (resized and copied).
pub fn fn_00ab46f0(e: &mut Engine, this: Ptr, source: Ptr) {
    let mesh = this.addr();
    let other = source.addr();
    let twelve = ElementSize::Divide(12);
    for (offset, size, resize, zeros) in [
        (VERTICES, twelve, RESIZE_12, 3),
        (TRIANGLES, twelve, RESIZE_12, 3),
        (QUADS, ElementSize::Shift(4), RESIZE_16, 4),
        (0x48, ElementSize::Shift(3), RESIZE_8, 2),
        (0x60, twelve, RESIZE_12, 3),
        (0x78, ElementSize::Shift(4), RESIZE_16, 4),
    ] {
        copy_plain(e, mesh + offset, other + offset, size, resize, zeros);
    }

    let vector = mesh + NAMED_20;
    let from = other + NAMED_20;
    let count = vector_size(e, from, ElementSize::Shift(5));
    e.call(RESIZE_20, &args![vector, count]);
    for_each_pair(e, vector, from, INCREMENT_20, |e, target, from| {
        let target = element(e, target);
        let source = element(e, from);
        let word = e.mem.u32(source);
        e.mem.set_u32(target, word);
        assign_string(e, target + 4, source + 4);
    });

    let vector = mesh + WEIGHTED_POINTS;
    let from = other + WEIGHTED_POINTS;
    let count = vector_size(e, from, ElementSize::Divide(0x2c));
    resize_with_default(e, vector, count, DEFAULT_2C, RESIZE_2C, 11);
    for_each_pair(e, vector, from, INCREMENT_2C, |e, target, from| {
        let target = element(e, target);
        let source = element(e, from);
        for i in 0..4u32 {
            let word = e.mem.u32(source + 4 * i);
            e.mem.set_u32(target + 4 * i, word);
        }
        assign_string(e, target + 0x10, source + 0x10);
    });

    let vector = mesh + NAMED_34;
    let from = other + NAMED_34;
    let count = vector_size(e, from, ElementSize::Divide(0x34));
    // 00ab5165: `SUB ESP,0x34; MOV ECX,ESP`: the default element is built in the outgoing
    // argument area, which stays in place as `00abb700`'s own `[EBP+0xC]` (it destroys the
    // element there; its `RET 0x38` frees the area). The element's vector proxy names the area.
    e.with_stack(0x34, |e, value| {
        e.call(DEFAULT_34, &args![value]); // 00ab5170
        fn_00abb700(e, Ptr::new(vector), count, value); // 00ab518e: the area itself, not a copy
    });
    for_each_pair(e, vector, from, INCREMENT_34, |e, target, from| {
        let source = element(e, from);
        let name = element(e, target);
        assign_string(e, name, source);
        copy_inner(e, target, from, 0x1c, twelve, RESIZE_12, 3);
    });

    let vector = mesh + NAMED_38;
    let from = other + NAMED_38;
    let count = vector_size(e, from, ElementSize::Divide(0x38));
    // 00ab5467: `SUB ESP,0x38; MOV ECX,ESP`: the default element is built in the outgoing
    // argument area, which stays in place as `00abbae0`'s own `[EBP+0xC]`: its vector's proxy
    // names that area, and `00abbae0` destroys the element there (its `RET 0x3C` frees it).
    e.with_stack(0x38, |e, outgoing| {
        e.call(DEFAULT_38, &args![outgoing]); // 00ab5472
        resize_38_in_area(e, Ptr::new(vector), count, outgoing.addr()); // 00ab5490: the area
    });
    for_each_pair(e, vector, from, INCREMENT_38, |e, target, from| {
        let source = element(e, from);
        let name = element(e, target);
        assign_string(e, name, source);
        let source = element(e, from);
        let word_target = element(e, target);
        let word = e.mem.u32(source + 0x1c);
        e.mem.set_u32(word_target + 0x1c, word);
        copy_inner(e, target, from, 0x20, ElementSize::Shift(2), RESIZE_4, 1);
    });
}

// Translated from 00ab5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the mesh's weighted points (`thiscall`, a vector of 12-byte
/// points to fill). The vector is cleared (`00ab9cd0`); for each 0x2C-byte element of +0xA8
/// whose face index is below `triangles + 2 * quads`, the face's three vertex indices are found
/// (a triangle; for a quad `k = face - triangles`, quad `k >> 1`'s indices 0, 1, 2 when `k` is
/// even and 2, 3, 0 when odd, a quad index past the quads skipping the element) and the point
/// `v[a] * w0 + v[b] * w1 + v[c] * w2` (`00ac1120`, `00ac10c0`; the element's three floats) is
/// appended (`00ab9bc0`). Every index is checked against its vector.
pub fn fn_00ab5720(e: &mut Engine, this: Ptr, points: u32) {
    let mesh = this.addr();
    let twelve = ElementSize::Divide(12);
    let sixteen = ElementSize::Shift(4);
    let weighted = ElementSize::Divide(0x2c);
    e.call(CLEAR_12, &args![points]);
    let triangles = vector_size(e, mesh + TRIANGLES, twelve);
    let quads = vector_size(e, mesh + QUADS, sixteen);
    let faces = triangles.wrapping_add(quads.wrapping_mul(2));
    let quad_word = |e: &mut Engine, quad: u32, word: u32| {
        check(e, quad < vector_size(e, mesh + QUADS, sixteen));
        let at = (quad << 4).wrapping_add(e.mem.u32(mesh + QUADS + VECTOR_FIRST));
        e.mem.u32(at.wrapping_add(4 * word))
    };
    e.with_stack(0x3c, |e, temporaries| {
        let scaled_c = temporaries.addr();
        let scaled_b = temporaries.addr() + 0xc;
        let scaled_a = temporaries.addr() + 0x18;
        let partial = temporaries.addr() + 0x24;
        let total = temporaries.addr() + 0x30;
        let mut index = 0u32;
        while index < vector_size(e, mesh + WEIGHTED_POINTS, weighted) {
            check(e, index < vector_size(e, mesh + WEIGHTED_POINTS, weighted));
            let list = e.mem.u32(mesh + WEIGHTED_POINTS + VECTOR_FIRST);
            let face = e.mem.u32(index.wrapping_mul(0x2c).wrapping_add(list));
            if face >= faces {
                index = index.wrapping_add(1);
                continue;
            }
            let corners = if face < triangles {
                check(e, face < vector_size(e, mesh + TRIANGLES, twelve));
                let first = e.mem.u32(mesh + TRIANGLES + VECTOR_FIRST);
                let at = face.wrapping_mul(12).wrapping_add(first);
                [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
            } else {
                let k = face.wrapping_sub(triangles);
                let quad = ((k as i32) >> 1) as u32;
                if (quad as i32) >= (quads as i32) {
                    index = index.wrapping_add(1);
                    continue;
                }
                if k & 1 != 0 {
                    let a = quad_word(e, quad, 0);
                    let b = quad_word(e, quad, 3);
                    let c = quad_word(e, quad, 2);
                    [c, b, a]
                } else {
                    let a = quad_word(e, quad, 2);
                    let b = quad_word(e, quad, 1);
                    let c = quad_word(e, quad, 0);
                    [c, b, a]
                }
            };
            check(e, index < vector_size(e, mesh + WEIGHTED_POINTS, weighted));
            let list = e.mem.u32(mesh + WEIGHTED_POINTS + VECTOR_FIRST);
            let weights = list.wrapping_add(index.wrapping_mul(0x2c)).wrapping_add(4);
            let vertex = |e: &mut Engine, corner: u32| {
                check(e, corner < vector_size(e, mesh + VERTICES, twelve));
                corner
                    .wrapping_mul(12)
                    .wrapping_add(e.mem.u32(mesh + VERTICES + VECTOR_FIRST))
            };
            let third = vertex(e, corners[2]);
            let second = vertex(e, corners[1]);
            let first = vertex(e, corners[0]);
            let weight = |e: &Engine, i: u32| x87_quiet_bits(e.mem.u32(weights + 4 * i));
            let w2 = weight(e, 2);
            let c = e.call(VECTOR_SCALE, &args![third, scaled_c, w2]).u32();
            let w1 = weight(e, 1);
            let b = e.call(VECTOR_SCALE, &args![second, scaled_b, w1]).u32();
            let w0 = weight(e, 0);
            let a = e.call(VECTOR_SCALE, &args![first, scaled_a, w0]).u32();
            let ab = e.call(VECTOR_ADD, &args![a, partial, b]).u32();
            let abc = e.call(VECTOR_ADD, &args![ab, total, c]).u32();
            e.call(PUSH_BACK_12, &args![points, abc]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00ab5ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the mesh's named points (`thiscall`, a vector of 12-byte points to
/// fill). The vector is cleared (`00ab9cd0`); for each 0x20-byte element of +0x90 whose first
/// word is a vertex index below the vertex count, that vertex is appended (`00ab9bc0`). Every
/// index is checked against its vector.
pub fn fn_00ab5ce0(e: &mut Engine, this: Ptr, points: u32) {
    let mesh = this.addr();
    let named = ElementSize::Shift(5);
    let twelve = ElementSize::Divide(12);
    e.call(CLEAR_12, &args![points]);
    let mut index = 0u32;
    while index < vector_size(e, mesh + NAMED_20, named) {
        check(e, index < vector_size(e, mesh + NAMED_20, named));
        let list = e.mem.u32(mesh + NAMED_20 + VECTOR_FIRST);
        let entry = (index << 5).wrapping_add(list);
        if e.mem.u32(entry) < vector_size(e, mesh + VERTICES, twelve) {
            check(e, index < vector_size(e, mesh + NAMED_20, named));
            let list = e.mem.u32(mesh + NAMED_20 + VECTOR_FIRST);
            let vertex = e.mem.u32((index << 5).wrapping_add(list));
            check(e, vertex < vector_size(e, mesh + VERTICES, twelve));
            let at = vertex
                .wrapping_mul(12)
                .wrapping_add(e.mem.u32(mesh + VERTICES + VECTOR_FIRST));
            e.call(PUSH_BACK_12, &args![points, at]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00ab5e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ab5e20 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`cdecl`, one word: the address of a file name string): whether the
/// file is a TRI file. A `FutBinaryFileC` is made on the stack with the id `"FRTRI003"`
/// (`0044a4c0` into the by-value argument, `00ac3df0`), checks the name (`00ac3fe0`) and is
/// destroyed (`00aaaa80`); the answer is the check's byte.
pub fn fn_00ab5e20(e: &mut Engine, name: u32) -> bool {
    e.with_stack(0x40, |e, file| {
        let file: Ptr<BSFaceGenBinaryFile> = Ptr::new(file.addr());
        let id = e.with_stack(0x1c, |e, id| {
            e.call(STRING_FROM_C, &args![id, TRI_FILE_ID]);
            StringArg::load(e, id.addr())
        });
        e.call(FUT_BINARY_FILE_CONSTRUCT, &args![file, id]);
        let found = e.call(FUT_BINARY_FILE_CHECK, &args![file, name]).u8();
        e.call(FUT_BINARY_FILE_DESTRUCT, &args![file]);
        found != 0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_46f0, fn_00ab46f0(Ptr, Ptr)),
        entry!(0x00ab_5720, fn_00ab5720(Ptr, u32)),
        entry!(0x00ab_5ce0, fn_00ab5ce0(Ptr, u32)),
        entry!(0x00ab_5e20, fn_00ab5e20(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::facegen::binaryfile::INVALID_PARAMETER;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The 0x34-byte element's destructor (`0065ed70`, `thiscall`), which `00abb700` runs on its
    /// value.
    const VALUE_34_DESTRUCT: u32 = 0x0065_ed70;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Sets a vector at `at` to a new block of `count` elements of `size` bytes.
    fn vector(e: &mut Engine, at: u32, count: u32, size: u32) -> u32 {
        let block = e.mem.alloc((count * size).max(4));
        e.mem.set_u32(at + VECTOR_FIRST, block);
        e.mem.set_u32(at + VECTOR_LAST, block + count * size);
        block
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        for (addr, size) in [
            (RESIZE_12, 12u32),
            (RESIZE_16, 16),
            (RESIZE_8, 8),
            (RESIZE_4, 4),
            (RESIZE_2C, 0x2c),
            (RESIZE_20, 0x20),
        ] {
            e.register_double(addr, move |e, a| {
                vector(e, a[0], a[1], size);
                Ret::default()
            });
        }
        // The 0x38-byte resize (00abbae0) runs on the value in place: its insert grows the
        // vector to the count (this, proxy, where, count, value) and the value's destructor is
        // quiet.
        e.register_double(INSERT_38, |e, a| {
            vector(e, a[0], a[3], 0x38);
            Ret::default()
        });
        e.register(DESTRUCT_38, |_, _| Ret::default());
        for (addr, size) in [
            (INCREMENT_20, 0x20u32),
            (INCREMENT_2C, 0x2c),
            (INCREMENT_34, 0x34),
            (INCREMENT_38, 0x38),
        ] {
            e.register_double(addr, move |e, a| {
                let pointer = e.mem.u32(a[0] + 4);
                e.mem.set_u32(a[0] + 4, pointer + size);
                a[0].into_ret()
            });
        }
        e.register(MAKE_ITERATOR, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[1]);
            a[0].into_ret()
        });
        e.register(ITERATORS_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(DEREFERENCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        for addr in [
            DEFAULT_2C,
            DEFAULT_34,
            DEFAULT_38,
            VALUE_34_DESTRUCT,
            STRING_ASSIGN,
            INVALID_PARAMETER,
            CLEAR_12,
            PUSH_BACK_12,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn mesh_assignment_copies_every_vector() {
        let mut e = engine();
        let source = e.mem.alloc(0xf0);
        let mesh = e.mem.alloc(0xf0);
        let vertices = vector(&mut e, source, 2, 12);
        for i in 0..6u32 {
            e.mem.set_u32(vertices + 4 * i, 0x100 + i);
        }
        for (offset, size) in [
            (0x18u32, 12u32),
            (0x30, 16),
            (0x48, 8),
            (0x60, 12),
            (0x78, 16),
        ] {
            let block = vector(&mut e, source + offset, 1, size);
            e.mem.set_u32(block, offset);
        }
        let named = vector(&mut e, source + 0x90, 1, 0x20);
        e.mem.set_u32(named, 0x77);
        let weighted = vector(&mut e, source + 0xa8, 1, 0x2c);
        for i in 0..4u32 {
            e.mem.set_u32(weighted + 4 * i, 0x200 + i);
        }
        let with_points = vector(&mut e, source + 0xc0, 1, 0x34);
        let inner = vector(&mut e, with_points + 0x1c, 1, 12);
        e.mem.set_u32(inner + 8, 0x300);
        let with_words = vector(&mut e, source + 0xd8, 1, 0x38);
        e.mem.set_u32(with_words + 0x1c, 0x400);
        let words = vector(&mut e, with_words + 0x20, 2, 4);
        e.mem.set_u32(words + 4, 0x500);
        // The mesh's 0x34-byte elements already number the source's one, so the real `00abb700`
        // keeps them and only destroys its value.
        vector(&mut e, mesh + 0xc0, 1, 0x34);
        e.call_log = Some(vec![]);
        e.call(0x00ab_46f0, &args![mesh, source]);
        let log = e.call_log.take().unwrap();
        let copied = e.mem.u32(mesh + VECTOR_FIRST);
        assert_eq!(e.mem.u32(copied + 0x14), 0x105);
        for offset in [0x18u32, 0x30, 0x48, 0x60, 0x78] {
            let block = e.mem.u32(mesh + offset + VECTOR_FIRST);
            assert_eq!(e.mem.u32(block), offset);
        }
        assert_eq!(calls_to(&log, RESIZE_12)[0], vec![mesh, 2, 0, 0, 0]);
        assert_eq!(calls_to(&log, RESIZE_8)[0], vec![mesh + 0x48, 1, 0, 0]);
        let named_copy = e.mem.u32(mesh + 0x90 + VECTOR_FIRST);
        assert_eq!(e.mem.u32(named_copy), 0x77);
        let weighted_copy = e.mem.u32(mesh + 0xa8 + VECTOR_FIRST);
        assert_eq!(e.mem.u32(weighted_copy + 0xc), 0x203);
        assert_eq!(calls_to(&log, RESIZE_2C)[0].len(), 13);
        let built = calls_to(&log, DEFAULT_38)[0][0];
        assert_eq!(
            calls_to(&log, INSERT_38),
            vec![vec![mesh + 0xd8, 0, 0, 1, built]]
        );
        assert_eq!(calls_to(&log, DESTRUCT_38), vec![vec![built]]);
        let points_copy = e.mem.u32(mesh + 0xc0 + VECTOR_FIRST);
        let inner_copy = e.mem.u32(points_copy + 0x1c + VECTOR_FIRST);
        assert_eq!(e.mem.u32(inner_copy + 8), 0x300);
        let words_copy = e.mem.u32(mesh + 0xd8 + VECTOR_FIRST);
        assert_eq!(e.mem.u32(words_copy + 0x1c), 0x400);
        let inner_words = e.mem.u32(words_copy + 0x20 + VECTOR_FIRST);
        assert_eq!(e.mem.u32(inner_words + 4), 0x500);
        assert_eq!(calls_to(&log, RESIZE_4)[0], vec![words_copy + 0x20, 2, 0]);
        let names = calls_to(&log, STRING_ASSIGN);
        assert_eq!(
            names,
            vec![
                vec![named_copy + 4, named + 4, 0, u32::MAX],
                vec![weighted_copy + 0x10, weighted + 0x10, 0, u32::MAX],
                vec![points_copy, with_points, 0, u32::MAX],
                vec![words_copy, with_words, 0, u32::MAX],
            ]
        );
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
    }

    /// 00ab5165..00ab518e: the default 0x34-byte element (`00aba270`) is built in the argument
    /// area of `00abb700`, which resizes with it and destroys it there (`0065ed70`), not in a
    /// copy of its words somewhere else.
    #[test]
    fn the_default_named_element_is_destroyed_where_it_was_built() {
        let mut e = engine();
        let source = e.mem.alloc(0xf0);
        let mesh = e.mem.alloc(0xf0);
        // One zeroed element in every vector of the source (and in the inner vectors of its
        // named elements); the mesh's 0x34-byte elements already number one, so the real
        // `00abb700` keeps them.
        for (offset, size) in [
            (0u32, 12u32),
            (0x18, 12),
            (0x30, 16),
            (0x48, 8),
            (0x60, 12),
            (0x78, 16),
            (0x90, 0x20),
            (0xa8, 0x2c),
            (0xc0, 0x34),
            (0xd8, 0x38),
        ] {
            vector(&mut e, source + offset, 1, size);
        }
        let with_points = e.mem.u32(source + 0xc0 + VECTOR_FIRST);
        vector(&mut e, with_points + 0x1c, 1, 12);
        let with_words = e.mem.u32(source + 0xd8 + VECTOR_FIRST);
        vector(&mut e, with_words + 0x20, 1, 4);
        vector(&mut e, mesh + 0xc0, 1, 0x34);
        e.call_log = Some(vec![]);
        e.call(0x00ab_46f0, &args![mesh, source]);
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, DEFAULT_34);
        assert_eq!(built.len(), 1);
        assert_eq!(calls_to(&log, VALUE_34_DESTRUCT), built);
    }

    /// `00abbae0`'s insert of `count` copies (`00abfa90`) and the 0x38-byte element's
    /// destructor (`0065edd0`).
    const INSERT_38: u32 = 0x00ab_fa90;
    const DESTRUCT_38: u32 = 0x0065_edd0;

    #[test]
    fn mesh_assignment_grows_the_0x38_vector_from_the_element_its_proxy_names() {
        // 00ab5467..00ab5490: the default 0x38-byte element (00aba2f0) is built in 00abbae0's
        // argument area, so the proxy of its 4-byte vector (+0x20, 00aba4f0: pointing back at
        // that vector) names the vector of the value the insert (00abfa90) copies from, and
        // 00abbae0 destroys the element there (0065edd0). Built in a local and passed as
        // words, the value the resize used was a copy elsewhere.
        let mut e = engine();
        e.register_double(DEFAULT_38, |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0] + 0x20);
            e.mem.set_u32(a[0] + 0x20, proxy);
            Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(INSERT_38, move |e, a| {
            let proxy = e.mem.u32(a[4] + 0x20);
            sink.borrow_mut().push((a[4], e.mem.u32(proxy)));
            vector(e, a[0], a[3], 0x38);
            Ret::default()
        });
        let source = e.mem.alloc(0xf0);
        let mesh = e.mem.alloc(0xf0);
        let with_words = vector(&mut e, source + 0xd8, 1, 0x38);
        vector(&mut e, with_words + 0x20, 1, 4);
        e.call_log = Some(vec![]);
        e.call(0x00ab_46f0, &args![mesh, source]);
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, DEFAULT_38)[0][0];
        assert_eq!(*seen.borrow(), vec![(built, built + 0x20)]);
        assert_eq!(calls_to(&log, DESTRUCT_38), vec![vec![built]]);
    }

    /// A mesh of four vertices, one triangle (0, 1, 2) and one quad (0, 1, 2, 3).
    fn small_mesh(e: &mut Engine) -> (u32, u32) {
        let mesh = e.mem.alloc(0xf0);
        let vertices = vector(e, mesh, 4, 12);
        let triangle = vector(e, mesh + 0x18, 1, 12);
        for i in 0..3u32 {
            e.mem.set_u32(triangle + 4 * i, i);
        }
        let quad = vector(e, mesh + 0x30, 1, 16);
        for i in 0..4u32 {
            e.mem.set_u32(quad + 4 * i, i);
        }
        (mesh, vertices)
    }

    #[test]
    fn weighted_points_from_triangles_and_quads() {
        let mut e = engine();
        let (mesh, vertices) = small_mesh(&mut e);
        // Faces 0 (the triangle), 1 (the quad's first half), 2 (its second) and 3 (none).
        let list = vector(&mut e, mesh + 0xa8, 4, 0x2c);
        for i in 0..4u32 {
            e.mem.set_u32(list + 0x2c * i, i);
            for w in 0..3u32 {
                e.mem
                    .set_f32(list + 0x2c * i + 4 + 4 * w, (10 * i + w) as f32);
            }
        }
        e.register(VECTOR_SCALE, |_, a| a[1].into_ret());
        e.register(VECTOR_ADD, |_, a| a[1].into_ret());
        let points = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00ab_5720, &args![mesh, points]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLEAR_12), vec![vec![points]]);
        let scaled: Vec<(u32, f32)> = calls_to(&log, VECTOR_SCALE)
            .iter()
            .map(|w| ((w[0] - vertices) / 12, f32::from_bits(w[2])))
            .collect();
        assert_eq!(
            scaled,
            vec![
                (2, 2.0),
                (1, 1.0),
                (0, 0.0),
                (2, 12.0),
                (1, 11.0),
                (0, 10.0),
                (0, 22.0),
                (3, 21.0),
                (2, 20.0)
            ]
        );
        let pushes = calls_to(&log, PUSH_BACK_12);
        assert_eq!(pushes.len(), 3);
        let adds = calls_to(&log, VECTOR_ADD);
        assert_eq!(pushes[0][1], adds[1][1]);
        assert_eq!(adds[1][0], adds[0][1]);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn named_points_skip_out_of_range_vertices() {
        let mut e = engine();
        let (mesh, vertices) = small_mesh(&mut e);
        let list = vector(&mut e, mesh + 0x90, 3, 0x20);
        for (i, vertex) in [1u32, 9, 3].iter().enumerate() {
            e.mem.set_u32(list + 0x20 * i as u32, *vertex);
        }
        let points = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00ab_5ce0, &args![mesh, points]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CLEAR_12), vec![vec![points]]);
        assert_eq!(
            calls_to(&log, PUSH_BACK_12),
            vec![vec![points, vertices + 12], vec![points, vertices + 36]]
        );
    }

    #[test]
    fn tri_check_builds_checks_and_destroys_a_file() {
        let mut e = Engine::new();
        for addr in [
            STRING_FROM_C,
            FUT_BINARY_FILE_CONSTRUCT,
            FUT_BINARY_FILE_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FUT_BINARY_FILE_CHECK, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ab_5e20, &args![0x1234u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_FROM_C)[0][1], TRI_FILE_ID);
        let file = calls_to(&log, FUT_BINARY_FILE_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, FUT_BINARY_FILE_CHECK),
            vec![vec![file, 0x1234]]
        );
        assert_eq!(calls_to(&log, FUT_BINARY_FILE_DESTRUCT), vec![vec![file]]);
        e.register(FUT_BINARY_FILE_CHECK, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00ab_5e20, &args![0x1234u32]).bool());
    }
}

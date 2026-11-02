//! `Havok SDK/run_00ce51f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator blocks are freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00cb2d90` (`this` = the matrix it fills, one word: the address of the vector at `this +
/// 0x10` of the object): builds a 3x3 matrix (`0x30` bytes).
const BUILD_MATRIX: u32 = 0x00cb_2d90;
/// `hkMatrix3::setMul` (`00cd55b0`, `this` = the result, stack `(a, b)`: the product `a * b`;
/// named by Ghidra from the Xbox PDB).
const MATRIX_MULTIPLY: u32 = 0x00cd_55b0;
/// `00cb2050` (`this` = a 4x4 matrix, one word: the destination; translated in this crate).
const COPY_MASKED_MATRIX: u32 = 0x00cb_2050;
/// `hkMathUtil::decompose4x4ColTransform` (`00d72e90`, cdecl `(input, result)`; named by Ghidra
/// from the Xbox PDB).
const DECOMPOSE_TRANSFORM: u32 = 0x00d7_2e90;
/// `00c905b0` (`this` only): run on a pointer field that is not null when it is cleared.
const RELEASE_FIELD: u32 = 0x00c9_05b0;
/// The vector `(0, 0, 0, 1)` (`010c71b0`) and the float `1.0` (`01012068`).
const UNIT_W_VECTOR: u32 = 0x010c_71b0;
const ONE: u32 = 0x0101_2068;
/// The capacity word of an `hkArray` that does not own its memory (bit 31 set).
const NOT_OWNED: u32 = 0x8000_0000;
/// The mask of the capacity bits of an `hkArray`'s capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator (`00c85750`, field `+0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00ce51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the 4x4 transform of an object (the class is unconfirmed) into `out`: `00cb2d90` fills
/// a 3x3 matrix from the object's vector at `+0x10`; a diagonal matrix whose diagonal is the
/// three floats at `+0x20`, `+0x24` and `+0x28` (each copied through the x87 stack, the rest
/// zero) is built; `hkMatrix3::setMul` (`00cd55b0`) multiplies the first by the second; the
/// three columns of the product followed by the object's vector at `+0` form a 4x4 matrix
/// which `00cb2050` copies to `out`.
pub fn fn_00ce51f0(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    e.with_stack(0x30, |e, rotation| {
        e.call(BUILD_MATRIX, &args![rotation, base + 0x10]);
        e.with_stack(0x30, |e, scale| {
            for (row, source) in [0x20u32, 0x24, 0x28].into_iter().enumerate() {
                let row = row as u32;
                for lane in 0..4 {
                    e.mem.set_u32(scale.addr() + 0x10 * row + 4 * lane, 0);
                }
                let bits = x87_quiet_bits(e.mem.u32(base + source));
                e.mem.set_u32(scale.addr() + 0x10 * row + 4 * row, bits);
            }
            e.with_stack(0x30, |e, product| {
                e.call(MATRIX_MULTIPLY, &args![product, rotation, scale]);
                e.with_stack(0x40, |e, transform| {
                    for row in 0..3 {
                        copy_vector(
                            e,
                            product.addr() + 0x10 * row,
                            transform.addr() + 0x10 * row,
                        );
                    }
                    copy_vector(e, base, transform.addr() + 0x30);
                    e.call(COPY_MASKED_MATRIX, &args![transform, out]);
                });
            });
        });
    });
}

// Translated from 00ce5290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decomposes the 4x4 transform at `input` with `hkMathUtil::decompose4x4ColTransform` into a
/// result block on the stack, copies its first three vectors (`0x30` bytes) to `this` and
/// writes to the byte at `out` whether the byte at `+0x31` of the result is zero. The class is
/// unconfirmed.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00ce5290(e: &mut Engine, this: Ptr, out: Ptr, input: Ptr) -> Ptr {
    let base = this.addr();
    e.with_stack(0x40, |e, result| {
        e.call(DECOMPOSE_TRANSFORM, &args![input, result]);
        for row in 0..3 {
            copy_vector(e, result.addr() + 0x10 * row, base + 0x10 * row);
        }
        let flag = e.mem.u8(result.addr() + 0x31);
        e.mem.set_u8(out.addr(), u8::from(flag == 0));
    });
    out
}

// Translated from 00ce52e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same decomposition with the flag byte thrown away: the exe passes the address of its
/// own argument slot as the destination of the flag. `this` and `input` are passed on to
/// `00ce5290`.
pub fn fn_00ce52e0(e: &mut Engine, this: Ptr, input: Ptr) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), input.addr());
        fn_00ce5290(e, this, slot, input);
    });
}

// Translated from 00ce5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `JMP` to `00ce51f0` (the linker's identical-code copy): the same function.
pub fn fn_00ce5300(e: &mut Engine, this: Ptr, out: Ptr) {
    fn_00ce51f0(e, this, out);
}

// Translated from 00ce5310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises an object (the class is unconfirmed) and returns `this`: two null words at `+0`
/// and `+4` (pointers that `00ce5370` releases) and three empty `hkArray`s (data 0, size 0,
/// capacity word `0x80000000`, not owned) at `+8`, `+0x14` and `+0x20` are written, then the
/// byte at `+0x60` is set to 1, the vector at `+0x30` zeroed, the vector `(0, 0, 0, 1)` copied to
/// `+0x40` and `1.0` splatted to `+0x50`.
pub fn fn_00ce5310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for (offset, word) in [
        (0x0, 0),
        (0x4, 0),
        (0x8, 0),
        (0xc, 0),
        (0x10, NOT_OWNED),
        (0x14, 0),
        (0x18, 0),
        (0x1c, NOT_OWNED),
        (0x20, 0),
        (0x24, 0),
        (0x28, NOT_OWNED),
    ] {
        e.mem.set_u32(base + offset, word);
    }
    e.mem.set_u8(base + 0x60, 1);
    for lane in 0..4 {
        e.mem.set_u32(base + 0x30 + 4 * lane, 0);
    }
    copy_vector(e, UNIT_W_VECTOR, base + 0x40);
    let one = e.mem.u32(ONE);
    for lane in 0..4 {
        e.mem.set_u32(base + 0x50 + 4 * lane, one);
    }
    this
}

// Translated from 00ce5360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `this`; the `RET 4` takes one word it never reads.
pub fn fn_00ce5360(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// Empties the `hkArray` at `array` (data `+0`, size `+4`, capacity word `+8`): the size is set
/// to 0 and, unless the capacity word says the memory is not owned, the data is returned to
/// the memory router's allocator (slot `0x10`, `(capacity & 0x3fffffff) * element_size`
/// bytes); then the data pointer is cleared and the capacity word set to `0x80000000`.
fn clear_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00ce5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the object `00ce5310` initialises: empties the `hkArray` of 2-byte
/// elements at `+0x20`, of `0x70`-byte elements at `+0x14` and of `0x40`-byte elements at `+8`
/// (in that order, see `clear_array`), then runs `00c905b0` on the pointers at `+4` and at `+0`
/// when they are not null and clears them.
pub fn fn_00ce5370(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    clear_array(e, base + 0x20, 2);
    clear_array(e, base + 0x14, 0x70);
    clear_array(e, base + 8, 0x40);
    for field in [base + 4, base] {
        let pointer = e.mem.u32(field);
        if pointer != 0 {
            e.call(RELEASE_FIELD, &args![pointer]);
        }
        e.mem.set_u32(field, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_51f0, fn_00ce51f0(Ptr, Ptr)),
        entry!(0x00ce_5290, fn_00ce5290(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ce_52e0, fn_00ce52e0(Ptr, Ptr)),
        entry!(0x00ce_5300, fn_00ce5300(Ptr, Ptr)),
        entry!(0x00ce_5310, fn_00ce5310(Ptr) -> Ptr),
        entry!(0x00ce_5360, fn_00ce5360(Ptr, u32) -> Ptr),
        entry!(0x00ce_5370, fn_00ce5370(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0101;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// Doubles of the three helpers: the matrix builder fills a numbered matrix from the
    /// vector it was given; the multiplication writes a product that records its inputs; the
    /// 4x4 copy logs the matrix and the destination.
    #[test]
    fn transform_is_built_from_the_product_and_the_translation() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(BUILD_MATRIX, move |e, a| {
            sink.borrow_mut().push((1, vec![a[1]]));
            for i in 0..12 {
                e.mem.set_u32(a[0] + 4 * i, 0xa000 + i);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(MATRIX_MULTIPLY, move |e, a| {
            // The scale matrix: diagonal from the three floats, zero elsewhere.
            let scale: Vec<u32> = (0..12).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            let rotation: Vec<u32> = (0..12).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push((2, [scale, rotation].concat()));
            for i in 0..12 {
                e.mem.set_u32(a[0] + 4 * i, 0xb000 + i);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(COPY_MASKED_MATRIX, move |e, a| {
            let matrix: Vec<u32> = (0..16).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            sink.borrow_mut().push((3, [matrix, vec![a[1]]].concat()));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0xc000 + i);
        }
        e.mem.set_u32(this + 0x20, 0x4000_0000);
        e.mem.set_u32(this + 0x24, 0x7f80_0001);
        e.mem.set_u32(this + 0x28, 0x4040_0000);
        let out = e.mem.alloc(0x40);
        e.call(0x00ce_51f0, &args![this, out]);
        let calls = log.borrow().clone();
        assert_eq!(calls[0], (1, vec![this + 0x10]));
        let diagonal = [
            0x4000_0000,
            0,
            0,
            0,
            0,
            0x7fc0_0001,
            0,
            0,
            0,
            0,
            0x4040_0000,
            0,
        ];
        let rotation: Vec<u32> = (0..12).map(|i| 0xa000 + i).collect();
        assert_eq!(calls[1], (2, [diagonal.to_vec(), rotation].concat()));
        let mut matrix: Vec<u32> = (0..12).map(|i| 0xb000 + i).collect();
        matrix.extend([0xc000, 0xc001, 0xc002, 0xc003]);
        matrix.push(out);
        assert_eq!(calls[2], (3, matrix));
        // The thunk copy does the same.
        e.call(0x00ce_5300, &args![this, out]);
        assert_eq!(log.borrow().len(), 6);
    }

    /// The decomposition result's first three vectors are copied to `this`; the flag is true
    /// exactly when the result byte at `+0x31` is zero.
    #[test]
    fn decomposition_copies_three_vectors_and_reports_a_zero_flag_byte() {
        let mut e = engine();
        e.register_double(DECOMPOSE_TRANSFORM, |e, a| {
            for i in 0..12 {
                e.mem.set_u32(a[1] + 4 * i, 0xd000 + i);
            }
            e.mem.set_u8(a[1] + 0x31, 0);
            e.mem.set_u8(a[1] + 0x30, 9);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let input = e.mem.alloc(0x40);
        e.mem.set_u8(out, 0xee);
        e.call(0x00ce_5290, &args![this, out, input]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            words(&e, this, 12),
            (0..12).map(|i| 0xd000 + i).collect::<Vec<_>>()
        );
        assert_eq!(e.mem.u32(this + 0x30), 0);
        // A non-zero byte gives 0; the one-argument form decomposes the same way.
        e.register_double(DECOMPOSE_TRANSFORM, |e, a| {
            e.mem.set_u8(a[1] + 0x31, 1);
            Ret::default()
        });
        e.call(0x00ce_5290, &args![this, out, input]);
        assert_eq!(e.mem.u8(out), 0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(DECOMPOSE_TRANSFORM, move |e, a| {
            sink.borrow_mut().push(a[0]);
            for i in 0..12 {
                e.mem.set_u32(a[1] + 4 * i, 0xe000 + i);
            }
            Ret::default()
        });
        e.call(0x00ce_52e0, &args![this, input]);
        assert_eq!(*seen.borrow(), vec![input]);
        assert_eq!(e.mem.u32(this), 0xe000);
    }

    #[test]
    fn decomposition_copies_three_vectors_and_reports_a_zero_flag_byte_answers_its_result_pointer()
    {
        let mut e = engine();
        e.register_double(DECOMPOSE_TRANSFORM, |e, a| {
            for i in 0..12 {
                e.mem.set_u32(a[1] + 4 * i, 0xd000 + i);
            }
            e.mem.set_u8(a[1] + 0x31, 0);
            e.mem.set_u8(a[1] + 0x30, 9);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let input = e.mem.alloc(0x40);
        e.mem.set_u8(out, 0xee);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00ce_5290, &args![this, out, input]).u32(),
            args![out][0]
        );
    }

    /// Every word the initialiser writes, and `this` returned; the no-op returns `this`.
    #[test]
    fn initialiser_writes_the_defaults() {
        let mut e = engine();
        e.mem.set_u32(UNIT_W_VECTOR + 12, 0x3f80_0000);
        e.mem.set_u32(ONE, 0x3f80_0000);
        let this = e.mem.alloc(0x70);
        e.mem.write(this, &[0xaa; 0x70]);
        let got = e.call(0x00ce_5310, &args![this]).u32();
        assert_eq!(got, this);
        let mut image = [0xaau8; 0x70];
        for (offset, word) in [
            (0x0usize, 0u32),
            (0x4, 0),
            (0x8, 0),
            (0xc, 0),
            (0x10, 0x8000_0000),
            (0x14, 0),
            (0x18, 0),
            (0x1c, 0x8000_0000),
            (0x20, 0),
            (0x24, 0),
            (0x28, 0x8000_0000),
            (0x30, 0),
            (0x34, 0),
            (0x38, 0),
            (0x3c, 0),
            (0x40, 0),
            (0x44, 0),
            (0x48, 0),
            (0x4c, 0x3f80_0000),
            (0x50, 0x3f80_0000),
            (0x54, 0x3f80_0000),
            (0x58, 0x3f80_0000),
            (0x5c, 0x3f80_0000),
        ] {
            image[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        }
        image[0x60] = 1;
        assert_eq!(e.mem.bytes(this, 0x70), image.to_vec());
        assert_eq!(e.call(0x00ce_5360, &args![this, 5u32]).u32(), this);
    }

    /// The three arrays are freed with their element sizes when owned, the two pointers
    /// released when set; everything is cleared either way.
    #[test]
    fn destructor_frees_owned_arrays_and_releases_the_pointers() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push((1, a[1], a[2]));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(RELEASE_FIELD, move |_, a| {
            sink.borrow_mut().push((2, a[0], 0));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x111);
        e.mem.set_u32(this + 4, 0x222);
        for (offset, data, capacity) in [
            (0x8u32, 0x5000u32, 3u32),
            (0x14, 0x6000, 0x4000_0002),
            (0x20, 0x7000, 0x8000_0004),
        ] {
            e.mem.set_u32(this + offset, data);
            e.mem.set_u32(this + offset + 4, 9);
            e.mem.set_u32(this + offset + 8, capacity);
        }
        e.call(0x00ce_5370, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (1, 0x6000, 0x70 * 2),
                (1, 0x5000, 0x40 * 3),
                (2, 0x222, 0),
                (2, 0x111, 0)
            ]
        );
        for offset in [0x8u32, 0x14, 0x20] {
            assert_eq!(e.mem.u32(this + offset), 0);
            assert_eq!(e.mem.u32(this + offset + 4), 0);
            assert_eq!(e.mem.u32(this + offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

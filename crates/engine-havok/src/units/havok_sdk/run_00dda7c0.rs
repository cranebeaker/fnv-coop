//! `Havok SDK/run_00dda7c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ddabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_0590`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ddabb0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_0590, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkOArchive` members called on the archive (Xbox PDB names): `write8` (`00d605b0`, one word),
/// `write32` (`00d60940`, one word), `writeFloat32` (`00d609c0`, one word holding the `float`);
/// `00dda730` (one pointer to a vector) and `hkDisplaySerializeOStream::writeTransform`
/// (`00dda740`, one pointer to a transform), `00dda6b0` (one pointer to three words).
const ARCHIVE_WRITE_8: u32 = 0x00d6_05b0;
const ARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE_FLOAT32: u32 = 0x00d6_09c0;
const WRITE_VECTOR: u32 = 0x00dd_a730;
const WRITE_TRANSFORM: u32 = 0x00dd_a740;
const WRITE_THREE_WORDS: u32 = 0x00dd_a6b0;
/// The `hkIArchive` constructor (`00dd9130`, `thiscall`, `RET 8`: the reader and a word whose low
/// byte is the byte-swap flag) and the vtable of `hkDisplaySerializeIStream` (`010d7dc8`).
const IARCHIVE_CONSTRUCTOR: u32 = 0x00dd_9130;
const DISPLAY_ISTREAM_VTABLE: u32 = 0x010d_7dc8;

// Translated from 00dda7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an `hkOArchive`), `lists`). `lists`
/// holds two arrays of 16-byte records: one at `+0` (count at `+4`) and one at `+0xc` (count at
/// `+0x10`). Writes the first count (`write32`) and every record of the first array as a vector
/// (`00dda730`), then the second count and every record of the second array as three words
/// (`00dda6b0`).
pub fn fn_00dda7c0(e: &mut Engine, this: u32, lists: u32) {
    let count = e.mem.i32(lists + 4);
    e.call(ARCHIVE_WRITE_32, &args![this, count as u32]);
    if count > 0 {
        let mut offset = 0u32;
        let mut left = count;
        loop {
            let data = e.mem.u32(lists);
            e.call(WRITE_VECTOR, &args![this, data.wrapping_add(offset)]);
            offset = offset.wrapping_add(0x10);
            left -= 1;
            if left == 0 {
                break;
            }
        }
    }
    let count = e.mem.i32(lists + 0x10);
    e.call(ARCHIVE_WRITE_32, &args![this, count as u32]);
    if count > 0 {
        let mut offset = 0u32;
        let mut left = count;
        loop {
            let data = e.mem.u32(lists + 0xc);
            e.call(WRITE_THREE_WORDS, &args![this, data.wrapping_add(offset)]);
            offset = offset.wrapping_add(0x10);
            left -= 1;
            if left == 0 {
                break;
            }
        }
    }
}

// Translated from 00dda830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word `lists`, as for `00dda7c0`). Answers the size
/// `00dda7c0` writes for it: `(count at +0x10 + count at +4) * 12 + 8`.
pub fn fn_00dda830(e: &mut Engine, lists: u32) -> i32 {
    let total = e.mem.i32(lists + 0x10).wrapping_add(e.mem.i32(lists + 4));
    total.wrapping_mul(12).wrapping_add(8)
}

// Translated from 00dda850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word `shape`). Answers the size of the record
/// `00dda8c0` writes for a display geometry, by the type word at `shape + 0x50`: types 1 and 5
/// 0x35, 2 and 7 0x29, 3 0x19, 4 0x25, 8 and 9 0x41, 6 `00dda830` of the pointer at `shape + 8`
/// plus 0x1d, and 1 for any other type.
pub fn fn_00dda850(e: &mut Engine, shape: u32) -> i32 {
    match e.mem.u32(shape + 0x50) {
        1 | 5 => 0x35,
        2 | 7 => 0x29,
        3 => 0x19,
        4 => 0x25,
        6 => {
            let lists = e.mem.u32(shape + 8);
            fn_00dda830(e, lists).wrapping_add(0x1d)
        }
        8 | 9 => 0x41,
        _ => 1,
    }
}

/// Writes the `float` a getter left in `ST0` to the archive the way the code does (`FSTP` to a
/// stack slot, then pushed).
fn write_float_result(e: &mut Engine, archive: u32, value: f64) {
    e.call(ARCHIVE_WRITE_FLOAT32, &args![archive, x87_store32(value)]);
}

/// Calls the vector getter `function` on `shape` with the scratch vector `scratch` and writes the
/// vector it answers (a pointer) with `00dda730`.
fn write_vector_from(e: &mut Engine, archive: u32, function: u32, shape: u32, scratch: u32) {
    let vector = e.call(function, &args![shape, scratch]).u32();
    e.call(WRITE_VECTOR, &args![archive, vector]);
}

// Translated from 00dda8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an `hkOArchive`), `shape` (a display
/// geometry)). Writes the type word at `shape + 0x50` as a byte (`write8`, `00d605b0`) and then,
/// by type (1 to 9; anything else writes nothing more):
/// - 1: the transform at `+0x10`, the lane 3 of the vector at `+0x60` as a `float`, that vector
///   (a local copy), and the words at `+0x70` and `+0x74` (`write32`);
/// - 2: the transform at `+0x10` and the vector at `+0x60`;
/// - 3: the vectors answered by `00de1150` and `00de1160` (`+0x60`, `+0x70`);
/// - 4: the vectors from `00de5300` and `00de5320` (each given a scratch vector), the `float`s
///   from `00de5350` and `00de5340` (answered in `ST0`) and the word from `00de5360`;
/// - 5: the vectors from `00de4920`, `00de48a0` and `00de48c0`, the `float`s from `00de4900`,
///   `00de48e0` and `00de48f0` and the word from `00de4910`;
/// - 6: the transform at `+0x10` and `00dda7c0` on the pointer at `+8`;
/// - 7: the vectors from `00de4df0`, `00de4de0` and `00de4e00` and the `float` `00de4e10` points at;
/// - 8 and 9 (identical code): the transform at `+0x10`, the `float` at `+0x80`, the vectors at
///   `+0x60` and `+0x70`, and the words at `+0x84` and `+0x88`.
///
/// Every `float` goes through the x87 stack, so a signalling NaN is written quiet.
pub fn fn_00dda8c0(e: &mut Engine, this: u32, shape: u32) {
    // The exe aligns ESP, then `SUB ESP,0x64` and three pushes: a 0x70-byte frame with the vector
    // copy at +0x10 (type 1, 00dda91b), the two vectors of type 4 at +0x20 and +0x30 and the three
    // of type 5 at +0x40, +0x50 and +0x60 (00ddaa4e..00ddaada).
    with_aligned_frame(e, 0x70, |e, frame| {
        let kind = e.mem.u32(shape + 0x50);
        e.call(ARCHIVE_WRITE_8, &args![this, kind]);
        if kind.wrapping_sub(1) > 8 {
            return;
        }
        match kind {
            1 => {
                e.call(WRITE_TRANSFORM, &args![this, shape + 0x10]);
                let copy = frame + 0x10;
                for lane in 0..4 {
                    let word = e.mem.u32(shape + 0x60 + 4 * lane);
                    e.mem.set_u32(copy + 4 * lane, word);
                }
                let lane_three = e.mem.u32(copy + 0xc);
                e.call(
                    ARCHIVE_WRITE_FLOAT32,
                    &args![this, x87_quiet_bits(lane_three)],
                );
                e.call(WRITE_VECTOR, &args![this, copy]);
                let word = e.mem.u32(shape + 0x70);
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
                let word = e.mem.u32(shape + 0x74);
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
            }
            2 => {
                e.call(WRITE_TRANSFORM, &args![this, shape + 0x10]);
                e.call(WRITE_VECTOR, &args![this, shape + 0x60]);
            }
            3 => {
                for getter in [0x00de_1150, 0x00de_1160] {
                    let vector = e.call(getter, &args![shape]).u32();
                    e.call(WRITE_VECTOR, &args![this, vector]);
                }
            }
            4 => {
                let scratch = frame + 0x20;
                write_vector_from(e, this, 0x00de_5300, shape, scratch);
                write_vector_from(e, this, 0x00de_5320, shape, scratch + 0x10);
                for getter in [0x00de_5350, 0x00de_5340] {
                    let value = e.call(getter, &args![shape]).f64();
                    write_float_result(e, this, value);
                }
                let word = e.call(0x00de_5360, &args![shape]).u32();
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
            }
            5 => {
                let scratch = frame + 0x40;
                write_vector_from(e, this, 0x00de_4920, shape, scratch);
                write_vector_from(e, this, 0x00de_48a0, shape, scratch + 0x10);
                write_vector_from(e, this, 0x00de_48c0, shape, scratch + 0x20);
                for getter in [0x00de_4900, 0x00de_48e0, 0x00de_48f0] {
                    let value = e.call(getter, &args![shape]).f64();
                    write_float_result(e, this, value);
                }
                let word = e.call(0x00de_4910, &args![shape]).u32();
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
            }
            6 => {
                e.call(WRITE_TRANSFORM, &args![this, shape + 0x10]);
                let lists = e.mem.u32(shape + 8);
                fn_00dda7c0(e, this, lists);
            }
            7 => {
                for getter in [0x00de_4df0, 0x00de_4de0, 0x00de_4e00] {
                    let vector = e.call(getter, &args![shape]).u32();
                    e.call(WRITE_VECTOR, &args![this, vector]);
                }
                let at = e.call(0x00de_4e10, &args![shape]).u32();
                let bits = e.mem.u32(at);
                e.call(ARCHIVE_WRITE_FLOAT32, &args![this, x87_quiet_bits(bits)]);
            }
            _ => {
                // Types 8 and 9: two identical blocks in the code.
                e.call(WRITE_TRANSFORM, &args![this, shape + 0x10]);
                let bits = e.mem.u32(shape + 0x80);
                e.call(ARCHIVE_WRITE_FLOAT32, &args![this, x87_quiet_bits(bits)]);
                e.call(WRITE_VECTOR, &args![this, shape + 0x60]);
                e.call(WRITE_VECTOR, &args![this, shape + 0x70]);
                let word = e.mem.u32(shape + 0x84);
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
                let word = e.mem.u32(shape + 0x88);
                e.call(ARCHIVE_WRITE_32, &args![this, word]);
            }
        }
    })
}

// Translated from 00ddabe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `reader`): the constructor of
/// `hkDisplaySerializeIStream` (the vtable's RTTI name): runs the `hkIArchive` constructor
/// (`00dd9130`) with `(reader, this & 0xffffff00)` (the code pushes `this` with its low byte
/// cleared; the callee reads the low byte as the byte-swap flag, so 0) and stores the
/// `hkDisplaySerializeIStream` vtable (`010d7dc8`). Answers `this`.
pub fn fn_00ddabe0(e: &mut Engine, this: u32, reader: u32) -> u32 {
    e.call(
        IARCHIVE_CONSTRUCTOR,
        &args![this, reader, this & 0xffff_ff00],
    );
    e.mem.set_u32(this, DISPLAY_ISTREAM_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_abb0, deleting_destructor_00ddabb0(Ptr, u8) -> Ptr),
        entry!(0x00dda7c0, fn_00dda7c0(u32, u32)),
        entry!(0x00dda830, fn_00dda830(u32) -> i32),
        entry!(0x00dda850, fn_00dda850(u32) -> i32),
        entry!(0x00dda8c0, fn_00dda8c0(u32, u32)),
        entry!(0x00ddabe0, fn_00ddabe0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00ddabb0_frees_only_when_bit_zero_is_set() {
        check(0x00dd_abb0, 0x00d6_0590);
    }
}

#[cfg(test)]
mod a07_geometry_writer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    fn c(name: &'static str, words: &[u32]) -> (&'static str, Vec<u32>) {
        (name, words.to_vec())
    }

    /// An archive address and doubles for the archive's functions, which log what they are given
    /// (the vector writers the words they point at).
    fn env() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let archive = e.mem.alloc(0x10);
        for (name, function) in [
            ("write8", ARCHIVE_WRITE_8),
            ("write32", ARCHIVE_WRITE_32),
            ("writeFloat32", ARCHIVE_WRITE_FLOAT32),
        ] {
            let seen = Rc::clone(&log);
            e.register_double(function, move |_, a| {
                assert_eq!(a[0], archive);
                seen.borrow_mut().push((name, a[1..].to_vec()));
                Ret::default()
            });
        }
        let seen = Rc::clone(&log);
        e.register_double(WRITE_VECTOR, move |e, a| {
            assert_eq!(a[0], archive);
            let words = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push(("vector", words));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(WRITE_TRANSFORM, move |e, a| {
            assert_eq!(a[0], archive);
            let first = e.mem.u32(a[1]);
            seen.borrow_mut().push(("transform", vec![first]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(WRITE_THREE_WORDS, move |e, a| {
            assert_eq!(a[0], archive);
            let words = (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push(("three", words));
            Ret::default()
        });
        (e, log, archive)
    }

    /// A shape of type `kind` with a transform word, the vectors and words the simple types read.
    fn shape(e: &mut Engine, kind: u32) -> u32 {
        let shape = e.mem.alloc(0xa0);
        e.mem.set_u32(shape + 0x50, kind);
        e.mem.set_u32(shape + 0x10, 0x111);
        for (i, w) in [1u32, 2, 3, 0x7f80_0001].into_iter().enumerate() {
            e.mem.set_u32(shape + 0x60 + 4 * i as u32, w);
        }
        for (i, w) in [5u32, 6, 7, 8].into_iter().enumerate() {
            e.mem.set_u32(shape + 0x70 + 4 * i as u32, w);
        }
        e.mem.set_u32(shape + 0x74, 0x74);
        e.mem.set_u32(shape + 0x80, 0x7f80_0002);
        e.mem.set_u32(shape + 0x84, 0x84);
        e.mem.set_u32(shape + 0x88, 0x88);
        shape
    }

    #[test]
    fn the_first_types_write_their_transform_and_vectors() {
        let (mut e, log, archive) = env();
        let s = shape(&mut e, 1);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[1]),
                c("transform", &[0x111]),
                c("writeFloat32", &[0x7fc0_0001]),
                c("vector", &[1, 2, 3, 0x7f80_0001]),
                c("write32", &[5]),
                c("write32", &[0x74]),
            ]
        );
        log.borrow_mut().clear();
        let s = shape(&mut e, 2);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[2]),
                c("transform", &[0x111]),
                c("vector", &[1, 2, 3, 0x7f80_0001]),
            ]
        );
    }

    #[test]
    fn types_eight_and_nine_write_the_float_the_vectors_and_two_words() {
        for kind in [8, 9] {
            let (mut e, log, archive) = env();
            let s = shape(&mut e, kind);
            e.call(0x00dd_a8c0, &args![archive, s]);
            assert_eq!(
                *log.borrow(),
                vec![
                    c("write8", &[kind]),
                    c("transform", &[0x111]),
                    c("writeFloat32", &[0x7fc0_0002]),
                    c("vector", &[1, 2, 3, 0x7f80_0001]),
                    c("vector", &[5, 0x74, 7, 8]),
                    c("write32", &[0x84]),
                    c("write32", &[0x88]),
                ]
            );
        }
    }

    #[test]
    fn type_three_writes_the_two_vectors_the_getters_point_at_and_unknown_types_nothing() {
        let (mut e, log, archive) = env();
        let s = shape(&mut e, 3);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[3]),
                c("vector", &[1, 2, 3, 0x7f80_0001]),
                c("vector", &[5, 0x74, 7, 8]),
            ]
        );
        for kind in [0, 10, 0xffff_ffff] {
            log.borrow_mut().clear();
            let s = shape(&mut e, kind);
            e.call(0x00dd_a8c0, &args![archive, s]);
            assert_eq!(*log.borrow(), vec![c("write8", &[kind])]);
        }
    }

    /// Doubles for the getters of types 4, 5 and 7: the vector getters write `[id; 4]` into the
    /// scratch vector they are given and answer it; the float getters answer `id as f64 + 0.5`.
    fn getters(e: &mut Engine) {
        for id in [0x5300u32, 0x5320, 0x4920, 0x48a0, 0x48c0] {
            e.register_double(0x00de_0000 + id, move |e, a| {
                for lane in 0..4 {
                    e.mem.set_u32(a[1] + 4 * lane, id);
                }
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
        for id in [0x5350u32, 0x5340, 0x4900, 0x48e0, 0x48f0] {
            e.register_double(0x00de_0000 + id, move |_, _| Ret {
                st0: f64::from(id),
                ..Ret::default()
            });
        }
        for id in [0x5360u32, 0x4910] {
            e.register_double(0x00de_0000 + id, move |_, _| Ret {
                eax: id,
                ..Ret::default()
            });
        }
        for id in [0x4df0u32, 0x4de0, 0x4e00] {
            e.register_double(0x00de_0000 + id, move |e, _| {
                let at = e.mem.alloc(16);
                for lane in 0..4 {
                    e.mem.set_u32(at + 4 * lane, id);
                }
                Ret {
                    eax: at,
                    ..Ret::default()
                }
            });
        }
        e.register_double(0x00de_4e10, |e, _| {
            let at = e.mem.alloc(8);
            e.mem.set_u32(at, 0x7f80_0003);
            Ret {
                eax: at,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn types_four_five_and_seven_use_the_getters() {
        let (mut e, log, archive) = env();
        getters(&mut e);
        let s = shape(&mut e, 4);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[4]),
                c("vector", &[0x5300; 4]),
                c("vector", &[0x5320; 4]),
                c("writeFloat32", &[f32::from(0x5350u16).to_bits()]),
                c("writeFloat32", &[f32::from(0x5340u16).to_bits()]),
                c("write32", &[0x5360]),
            ]
        );
        log.borrow_mut().clear();
        let s = shape(&mut e, 5);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[5]),
                c("vector", &[0x4920; 4]),
                c("vector", &[0x48a0; 4]),
                c("vector", &[0x48c0; 4]),
                c("writeFloat32", &[f32::from(0x4900u16).to_bits()]),
                c("writeFloat32", &[f32::from(0x48e0u16).to_bits()]),
                c("writeFloat32", &[f32::from(0x48f0u16).to_bits()]),
                c("write32", &[0x4910]),
            ]
        );
        log.borrow_mut().clear();
        let s = shape(&mut e, 7);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[7]),
                c("vector", &[0x4df0; 4]),
                c("vector", &[0x4de0; 4]),
                c("vector", &[0x4e00; 4]),
                c("writeFloat32", &[0x7fc0_0003]),
            ]
        );
    }

    #[test]
    fn the_type_vectors_sit_at_their_offsets_in_the_aligned_frame() {
        let (mut e, _log, archive) = env();
        getters(&mut e);
        let words =
            |e: &Engine, at: u32| (0..4).map(|i| e.mem.u32(at + 4 * i)).collect::<Vec<u32>>();
        for (kind, slots) in [
            (1u32, vec![(0x10u32, vec![1, 2, 3, 0x7f80_0001])]),
            (4, vec![(0x20, vec![0x5300; 4]), (0x30, vec![0x5320; 4])]),
            (
                5,
                vec![
                    (0x40, vec![0x4920; 4]),
                    (0x50, vec![0x48a0; 4]),
                    (0x60, vec![0x48c0; 4]),
                ],
            ),
        ] {
            let s = shape(&mut e, kind);
            // The 0x80-byte local `with_aligned_frame` takes; its words stay on the thread's stack
            // after the call returns (only the next local there zeroes them).
            let raw = e.next_stack_local(0x80);
            e.call(0x00dd_a8c0, &args![archive, s]);
            let frame = aligned_frame_at(raw, 0x70);
            for (offset, expected) in slots {
                assert_eq!(
                    words(&e, frame + offset),
                    expected,
                    "type {kind} +{offset:x}"
                );
            }
        }
    }

    /// An `hkArray`-pair record: `first` vectors (count at `+4`), `second` records (count `+0x10`).
    fn lists(e: &mut Engine, first: &[[u32; 4]], second: &[[u32; 4]]) -> u32 {
        let at = e.mem.alloc(0x20);
        for (offset, entries) in [(0u32, first), (0xc, second)] {
            let data = e.mem.alloc(16 * entries.len().max(1) as u32);
            for (i, record) in entries.iter().enumerate() {
                for (lane, w) in record.iter().enumerate() {
                    e.mem.set_u32(data + 16 * i as u32 + 4 * lane as u32, *w);
                }
            }
            e.mem.set_u32(at + offset, data);
            e.mem.set_u32(at + offset + 4, entries.len() as u32);
        }
        at
    }

    #[test]
    fn the_two_lists_are_written_as_vectors_then_as_three_words() {
        let (mut e, log, archive) = env();
        let l = lists(&mut e, &[[1, 2, 3, 4], [5, 6, 7, 8]], &[[9, 10, 11, 12]]);
        e.call(0x00dd_a7c0, &args![archive, l]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write32", &[2]),
                c("vector", &[1, 2, 3, 4]),
                c("vector", &[5, 6, 7, 8]),
                c("write32", &[1]),
                c("three", &[9, 10, 11]),
            ]
        );
        log.borrow_mut().clear();
        let empty = lists(&mut e, &[], &[]);
        e.call(0x00dd_a7c0, &args![archive, empty]);
        assert_eq!(*log.borrow(), vec![c("write32", &[0]), c("write32", &[0])]);
    }

    #[test]
    fn type_six_writes_its_transform_and_the_lists() {
        let (mut e, log, archive) = env();
        let l = lists(&mut e, &[[1, 1, 1, 1]], &[]);
        let s = shape(&mut e, 6);
        e.mem.set_u32(s + 8, l);
        e.call(0x00dd_a8c0, &args![archive, s]);
        assert_eq!(
            *log.borrow(),
            vec![
                c("write8", &[6]),
                c("transform", &[0x111]),
                c("write32", &[1]),
                c("vector", &[1, 1, 1, 1]),
                c("write32", &[0]),
            ]
        );
    }

    #[test]
    fn the_sizes_depend_on_the_type() {
        let mut e = Engine::new();
        let l = lists(&mut e, &[[0; 4]; 2], &[[0; 4]; 3]);
        assert_eq!(e.call(0x00dd_a830, &args![l]).i32(), (3 + 2) * 12 + 8);
        let s = shape(&mut e, 6);
        e.mem.set_u32(s + 8, l);
        let mut sizes = Vec::new();
        for kind in 0..=10 {
            e.mem.set_u32(s + 0x50, kind);
            sizes.push(e.call(0x00dd_a850, &args![s]).i32());
        }
        assert_eq!(
            sizes,
            vec![
                1,
                0x35,
                0x29,
                0x19,
                0x25,
                0x35,
                68 + 0x1d,
                0x29,
                0x41,
                0x41,
                1
            ]
        );
    }

    #[test]
    fn the_display_input_stream_constructor_clears_the_flag_byte() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(IARCHIVE_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let block = e.mem.alloc(0x100);
        let this = block + 0x5a;
        assert_eq!(e.call(0x00dd_abe0, &args![this, 0x9000u32]).u32(), this);
        assert_eq!(*log.borrow(), vec![vec![this, 0x9000, this & 0xffff_ff00]]);
        assert_eq!(e.mem.u32(this), DISPLAY_ISTREAM_VTABLE);
    }
}

//! `fallout shared/facegen/run_00658f30` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! FaceGen's float matrix `FR2MatrixVTC<float>` (Xbox PDB: a `std::vector<float>` then
//! `nrows` and `ncols`; on PC the vector is 0x18 bytes, so the counts sit at +0x18 and
//! +0x1C), a cleanup of an element of a list, two post-increments of a vector iterator and
//! the per-element square the matrix code passes around.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `FR2MatrixVTC<float>` (Xbox PDB, 0x18 bytes there), 0x20 bytes on PC: its
    /// `std::vector<float>` at +0x00 (0x18 bytes).
    pub struct Fr2MatrixVtc: 0x20 {
        /// `nrows` (Xbox PDB).
        0x18 nrows: u32,
        /// `ncols` (Xbox PDB).
        0x1C ncols: u32,
    }
}

/// The matrix's vector: its constructor (`00449fd0`, `thiscall`, no stack words), resize
/// (`0060b3f0`, `thiscall`: the element count), the size test the copy makes (`00662820`,
/// `thiscall`; not 0 when it has elements) and the element address (`006578b0`, `thiscall`:
/// the index).
const VECTOR_CONSTRUCT: u32 = 0x0044_9fd0;
const VECTOR_RESIZE: u32 = 0x0060_b3f0;
const VECTOR_SIZE: u32 = 0x0066_2820;
const VECTOR_ELEMENT: u32 = 0x0065_78b0;
/// `00401460` (`cdecl`: destination, source, byte count): the copy.
const MEMORY_COPY: u32 = 0x0040_1460;
/// `006a68b0` (`thiscall`, no stack words): the element's destructor `00659140` runs.
const ELEMENT_DESTRUCT: u32 = 0x006a_68b0;
/// `operator delete` (`00401030`, `cdecl`, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The iterator's debug checks: `005ca4f0` (`thiscall` on the iterator; answers a byte, 0
/// when the iterator has no container) and `0060b980` (`thiscall` on the iterator; answers
/// the container, whose last-element pointer is at +0x10), and `_invalid_parameter_noinfo`
/// (`00ec7c56`, no arguments) they call on a failed check.
const ITERATOR_HAS_CONTAINER: u32 = 0x005c_a4f0;
const ITERATOR_CONTAINER: u32 = 0x0060_b980;
const INVALID_PARAMETER: u32 = 0x00ec_7c56;

// Translated from 00658f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00658f30 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FR2MatrixVTC<float>`'s copy constructor (`thiscall`, the source
/// matrix). Constructs the vector (`00449fd0`), copies `nrows` and `ncols`, resizes the vector
/// to `nrows * ncols` and, when both vectors have elements (`00662820`), copies
/// `nrows * ncols` floats from the source's element 0 to this one's (`006578b0`, `00401460`).
/// Returns `this`.
pub fn fn_00658f30(
    e: &mut Engine,
    this: Ptr<Fr2MatrixVtc>,
    source: Ptr<Fr2MatrixVtc>,
) -> Ptr<Fr2MatrixVtc> {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    let rows = e.get(source, Fr2MatrixVtc::nrows);
    e.set(this, Fr2MatrixVtc::nrows, rows);
    let columns = e.get(source, Fr2MatrixVtc::ncols);
    e.set(this, Fr2MatrixVtc::ncols, columns);
    let count = e
        .get(this, Fr2MatrixVtc::nrows)
        .wrapping_mul(e.get(this, Fr2MatrixVtc::ncols));
    e.call(VECTOR_RESIZE, &args![this, count]);
    if e.call(VECTOR_SIZE, &args![this]).u32() != 0
        && e.call(VECTOR_SIZE, &args![source]).u32() != 0
    {
        let bytes = e
            .get(this, Fr2MatrixVtc::nrows)
            .wrapping_mul(e.get(this, Fr2MatrixVtc::ncols))
            .wrapping_shl(2);
        let from = e.call(VECTOR_ELEMENT, &args![source, 0u32]).u32();
        let to = e.call(VECTOR_ELEMENT, &args![this, 0u32]).u32();
        e.call(MEMORY_COPY, &args![to, from, bytes]);
    }
    this
}

// Translated from 00659000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00659000 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FR2MatrixVTC<float>`'s constructor from a row and a column
/// count (`thiscall`, two words): constructs the vector (`00449fd0`), stores the counts and
/// resizes the vector to `ncols * nrows` (`0060b3f0`). Returns `this`.
pub fn fn_00659000(
    e: &mut Engine,
    this: Ptr<Fr2MatrixVtc>,
    rows: u32,
    columns: u32,
) -> Ptr<Fr2MatrixVtc> {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    e.set(this, Fr2MatrixVtc::nrows, rows);
    e.set(this, Fr2MatrixVtc::ncols, columns);
    let count = e
        .get(this, Fr2MatrixVtc::ncols)
        .wrapping_mul(e.get(this, Fr2MatrixVtc::nrows));
    e.call(VECTOR_RESIZE, &args![this, count]);
    this
}

// Translated from 00659080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `this` is not read): deletes `element`
/// through its scalar deleting destructor (`00659140` with flag 1) when it is not null.
/// Leaves `EAX` as the code does: the destructor's result, or the null element.
pub fn fn_00659080(e: &mut Engine, _this: Ptr, element: Ptr) -> Ptr {
    if element.is_null() {
        return element;
    }
    fn_00659140(e, element, 1)
}

// Translated from 006590c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an iterator's post-increment (`thiscall`, two words: the result
/// and the unused `int` of `operator++(int)`): copies the iterator's two words, advances it
/// (`00659170`) and writes the copy to `result`. Returns `result`.
pub fn fn_006590c0(e: &mut Engine, this: Ptr, result: Ptr, _unused_2: u32) -> Ptr {
    let first = e.mem.u32(this.addr());
    let second = e.mem.u32(this.addr() + 4);
    fn_00659170(e, this);
    e.mem.set_u32(result.addr(), first);
    e.mem.set_u32(result.addr() + 4, second);
    result
}

// Translated from 00659100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the same post-increment as `006590c0`, advancing with
/// `00659190` directly.
pub fn fn_00659100(e: &mut Engine, this: Ptr, result: Ptr, _unused_2: u32) -> Ptr {
    let first = e.mem.u32(this.addr());
    let second = e.mem.u32(this.addr() + 4);
    fn_00659190(e, this);
    e.mem.set_u32(result.addr(), first);
    e.mem.set_u32(result.addr() + 4, second);
    result
}

// Translated from 00659140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor (`thiscall`, the flags): runs the
/// destructor `006a68b0` and, with bit 0 of `flags`, `operator delete`. Returns `this`.
pub fn fn_00659140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ELEMENT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00659170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the iterator's pre-increment (`00659190`), returning `this`.
pub fn fn_00659170(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00659190(e, this);
    this
}

// Translated from 00659190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked vector iterator's pre-increment over 4-byte elements.
/// The iterator must have a container (`005ca4f0`) and its pointer (+4) must be below the
/// container's last-element pointer (+0x10, container from `0060b980`); a failed check calls
/// `_invalid_parameter_noinfo` (`00ec7c56`) and goes on. The pointer moves by 4. Returns
/// `this`.
pub fn fn_00659190(e: &mut Engine, this: Ptr) -> Ptr {
    if e.call(ITERATOR_HAS_CONTAINER, &args![this]).u8() == 0 {
        e.call(INVALID_PARAMETER, &[]);
    }
    let container = e.call(ITERATOR_CONTAINER, &args![this]).u32();
    if e.mem.u32(this.addr() + 4) >= e.mem.u32(container.wrapping_add(0x10)) {
        e.call(INVALID_PARAMETER, &[]);
    }
    let pointer = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr() + 4, pointer.wrapping_add(4));
    this
}

// Translated from 006591f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one float): the square `value * value`, stored as a
/// `float` and returned in `ST0`.
pub fn fn_006591f0(_e: &mut Engine, value: f32) -> f32 {
    let bits = value.to_bits();
    let square = x87_mul(x87_load32(bits), x87_m32(bits));
    f32::from_bits(x87_store32(square))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0065_8f30,
            fn_00658f30(Ptr<Fr2MatrixVtc>, Ptr<Fr2MatrixVtc>) -> Ptr<Fr2MatrixVtc>
        ),
        entry!(
            0x0065_9000,
            fn_00659000(Ptr<Fr2MatrixVtc>, u32, u32) -> Ptr<Fr2MatrixVtc>
        ),
        entry!(0x0065_9080, fn_00659080(Ptr, Ptr) -> Ptr),
        entry!(0x0065_90c0, fn_006590c0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0065_9100, fn_00659100(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0065_9140, fn_00659140(Ptr, u32) -> Ptr),
        entry!(0x0065_9170, fn_00659170(Ptr) -> Ptr),
        entry!(0x0065_9190, fn_00659190(Ptr) -> Ptr),
        entry!(0x0065_91f0, fn_006591f0(f32) -> f32),
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

    /// Doubles for the matrix's vector: the element block is the word at +0x0C.
    fn matrix_engine() -> Engine {
        let mut e = Engine::new();
        e.register(VECTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0);
            Ret::default()
        });
        e.register(VECTOR_RESIZE, |e, a| {
            let block = if a[1] == 0 { 0 } else { e.mem.alloc(a[1] * 4) };
            e.mem.set_u32(a[0] + 0xc, block);
            Ret::default()
        });
        e.register(VECTOR_SIZE, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc) != 0).into_ret()
        });
        e.register(VECTOR_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 0xc) + 4 * a[1]).into_ret()
        });
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e
    }

    #[test]
    fn copy_constructor_copies_counts_and_elements() {
        let mut e = matrix_engine();
        let source: Ptr<Fr2MatrixVtc> = e.new_object();
        e.call(0x0065_9000, &args![source, 2u32, 3u32]);
        let block = e.mem.u32(source.addr() + 0xc);
        for i in 0..6 {
            e.mem.set_f32(block + 4 * i, i as f32 + 0.5);
        }
        let copy: Ptr<Fr2MatrixVtc> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8f30, &args![copy, source]).u32(), copy.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, VECTOR_RESIZE), vec![vec![copy.addr(), 6]]);
        let copied = e.mem.u32(copy.addr() + 0xc);
        assert_eq!(calls_to(&log, MEMORY_COPY), vec![vec![copied, block, 24]]);
        assert_eq!(e.get(copy, Fr2MatrixVtc::nrows), 2);
        assert_eq!(e.get(copy, Fr2MatrixVtc::ncols), 3);
        assert_eq!(e.mem.f32(copied + 20), 5.5);

        // An empty source: no copy.
        let empty: Ptr<Fr2MatrixVtc> = e.new_object();
        e.call(0x0065_9000, &args![empty, 0u32, 3u32]);
        let other: Ptr<Fr2MatrixVtc> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0065_8f30, &args![other, empty]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMORY_COPY).is_empty());
    }

    #[test]
    fn sized_constructor_stores_counts_and_resizes() {
        let mut e = matrix_engine();
        let m: Ptr<Fr2MatrixVtc> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9000, &args![m, 4u32, 5u32]).u32(), m.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, VECTOR_CONSTRUCT), vec![vec![m.addr()]]);
        assert_eq!(calls_to(&log, VECTOR_RESIZE), vec![vec![m.addr(), 20]]);
        assert_eq!(e.get(m, Fr2MatrixVtc::nrows), 4);
        assert_eq!(e.get(m, Fr2MatrixVtc::ncols), 5);
    }

    #[test]
    fn element_cleanup_deletes_non_null_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let element = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9080, &args![0u32, element]).u32(), element);
        e.call(0x0065_9080, &args![0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ELEMENT_DESTRUCT), vec![vec![element]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![element]]);
    }

    #[test]
    fn deleting_destructor_deletes_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let element = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9140, &args![element, 0u32]).u32(), element);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ELEMENT_DESTRUCT).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0065_9140, &args![element, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![element]]);
    }

    /// An iterator (two words: the container proxy and the pointer) over a container whose
    /// last pointer is `last`.
    fn iterator_engine(last: u32, has_container: bool) -> (Engine, Ptr) {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x18);
        e.mem.set_u32(container + 0x10, last);
        e.register_double(ITERATOR_HAS_CONTAINER, move |_, _| {
            u32::from(has_container).into_ret()
        });
        e.register_double(ITERATOR_CONTAINER, move |_, _| container.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let it: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(it.addr(), 0x77);
        e.mem.set_u32(it.addr() + 4, 0x1000);
        (e, it)
    }

    #[test]
    fn pre_increment_checks_and_advances() {
        let (mut e, it) = iterator_engine(0x1010, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9190, &args![it]).u32(), it.addr());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        assert_eq!(e.mem.u32(it.addr() + 4), 0x1004);
        // At the end and without a container: both checks fail, it still advances.
        let (mut e, it) = iterator_engine(0x1000, false);
        e.call_log = Some(vec![]);
        e.call(0x0065_9190, &args![it]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, INVALID_PARAMETER).len(), 2);
        assert_eq!(e.mem.u32(it.addr() + 4), 0x1004);
    }

    #[test]
    fn increment_wrapper_returns_this() {
        let (mut e, it) = iterator_engine(0x1010, true);
        assert_eq!(e.call(0x0065_9170, &args![it]).u32(), it.addr());
        assert_eq!(e.mem.u32(it.addr() + 4), 0x1004);
    }

    #[test]
    fn post_increments_return_the_old_position() {
        for addr in [0x0065_90c0u32, 0x0065_9100] {
            let (mut e, it) = iterator_engine(0x1010, true);
            let result: Ptr = Ptr::new(e.mem.alloc(8));
            assert_eq!(e.call(addr, &args![it, result, 0u32]).u32(), result.addr());
            assert_eq!(e.mem.u32(result.addr()), 0x77);
            assert_eq!(e.mem.u32(result.addr() + 4), 0x1000);
            assert_eq!(e.mem.u32(it.addr() + 4), 0x1004);
        }
    }

    #[test]
    fn square_rounds_to_float() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0065_91f0, &args![3.0f32]).f32(), 9.0);
        let x = 1.000_000_1f32;
        assert_eq!(e.call(0x0065_91f0, &args![x]).f32(), x * x);
        assert!(e.call(0x0065_91f0, &args![f32::NAN]).f32().is_nan());
    }
}

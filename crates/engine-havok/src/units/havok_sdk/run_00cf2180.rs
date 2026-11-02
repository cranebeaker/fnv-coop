//! `Havok SDK/run_00cf2180` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{addps, mulps, splat, subps};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};

/// The helpers `00cf2180` calls on a 16-byte local (`this` = the local): `00c74ac0` (stack
/// `(vector, target)`, callee-cleaned, fills the local), `00c74c20` (no arguments, result in
/// `ST0`), `00c74c90` (stack `(result vector)`, callee-cleaned) and `00ce1500` (stack `(flag
/// byte)`, returns that pointer). `00cb2d90` builds a 3x3 matrix at its `this` from the vector
/// it is given.
const FILL_LOCAL: u32 = 0x00c7_4ac0;
const LOCAL_SCALAR: u32 = 0x00c7_4c20;
const LOCAL_VECTOR: u32 = 0x00c7_4c90;
const LENGTH_FLAG: u32 = 0x00ce_1500;
const BUILD_MATRIX: u32 = 0x00cb_2d90;

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// Copies the float at `from` to `to` through the x87 stack (`FLD`/`FSTP`).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// Mask applied to the first vector (a 16-byte constant in `.rdata`).
const MASK_010C4C10: u32 = 0x010c_4c10;

/// Four words at `addr`.
fn read_words_00cf2280(e: &Engine, addr: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(addr + offset))
}

fn write_words_00cf2280(e: &mut Engine, addr: u32, words: [u32; 4]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(addr + 4 * i as u32, *word);
    }
}

// Translated from 00cf2280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, two words (`RET 8`), 16-byte vectors: fills the
/// 0x50 bytes at `this` with `first & mask` (twice, at `+0` and `+0x10`,
/// `mask` being the 16 bytes at `010c4c10`), `second` (twice, at `+0x20`
/// and `+0x30`) and zeros (`+0x40`). Each source is read again for its
/// second copy, as the code does.
pub fn fn_00cf2280(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let mask = read_words_00cf2280(e, MASK_010C4C10);
    let value = read_words_00cf2280(e, first.addr());
    write_words_00cf2280(e, this.addr(), [0, 1, 2, 3].map(|i| value[i] & mask[i]));
    let value = read_words_00cf2280(e, first.addr());
    let mask = read_words_00cf2280(e, MASK_010C4C10);
    write_words_00cf2280(
        e,
        this.addr() + 0x10,
        [0, 1, 2, 3].map(|i| value[i] & mask[i]),
    );
    let value = read_words_00cf2280(e, second.addr());
    write_words_00cf2280(e, this.addr() + 0x20, value);
    let value = read_words_00cf2280(e, second.addr());
    write_words_00cf2280(e, this.addr() + 0x30, value);
    write_words_00cf2280(e, this.addr() + 0x40, [0; 4]);
}

// Translated from 00cf2180 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(position, first, second, state)`: updates a state block (the class is unconfirmed,
/// at least `0xa0` bytes). The vectors at `+0x50` and `+0x70` move to `+0x40` and `+0x60`;
/// `first` and `second` are copied to `+0x50` and `+0x70`. A 16-byte local is filled by `00c74ac0`
/// from `(second, state + 0x60)`; `00c74c20` gives a float (stored as a float), and `00ce1500`
/// says whether the local is long enough: when it is, `00c74c90` makes a vector from it,
/// otherwise the vector is zero. `state + 0x90` becomes the float times that vector (four
/// lanes), then its last lane (`+0x9c`) the float itself (through the x87 stack). The floats at
/// `+0x4c` and `+0x5c` are copied (x87) from `position[0]` and `position[3]`, `00cb2d90` builds
/// the 3x3 matrix at `state` from the vector at `+0x70`, and `state + 0x30` becomes the vector at
/// `+0x50` minus `v1 * row1 + v0 * row0 + v2 * row2` (each `v` splatted, `v` the vector at `+0x80`).
pub fn fn_00cf2180(e: &mut Engine, position: Ptr, first: Ptr, second: Ptr, state: Ptr) {
    let block = state.addr();
    copy_vector(e, block + 0x50, block + 0x40);
    copy_vector(e, block + 0x70, block + 0x60);
    copy_vector(e, first.addr(), block + 0x50);
    copy_vector(e, second.addr(), block + 0x70);
    e.with_stack(0x10, |e, local| {
        e.call(FILL_LOCAL, &args![local, second, block + 0x60]);
        let scalar = e.call(LOCAL_SCALAR, &args![local]).f64();
        let scalar_bits = x87_store32(scalar);
        let vector = e.with_stack(4, |e, flag| {
            let reply = e.call(LENGTH_FLAG, &args![local, flag]).u32();
            if e.mem.u8(reply) != 0 {
                e.with_stack(0x10, |e, made| {
                    e.call(LOCAL_VECTOR, &args![local, made]);
                    load_vec(e, made.addr())
                })
            } else {
                [0.0; 4]
            }
        });
        let scaled = mulps(splat(f32::from_bits(scalar_bits)), vector);
        store_vec(e, block + 0x90, scaled);
        e.mem.set_u32(block + 0x9c, x87_quiet_bits(scalar_bits));
    });
    copy_float(e, position.addr(), block + 0x4c);
    copy_float(e, position.addr() + 0xc, block + 0x5c);
    e.call(BUILD_MATRIX, &args![state, block + 0x70]);
    let offset = load_vec(e, block + 0x80);
    let rows = [
        load_vec(e, block + 0x10),
        load_vec(e, block),
        load_vec(e, block + 0x20),
    ];
    let sum = addps(
        mulps(splat(offset[1]), rows[0]),
        mulps(splat(offset[0]), rows[1]),
    );
    let sum = addps(sum, mulps(splat(offset[2]), rows[2]));
    let base = load_vec(e, block + 0x50);
    store_vec(e, block + 0x30, subps(base, sum));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf2280, fn_00cf2280(Ptr, Ptr, Ptr)),
        entry!(0x00cf_2180, fn_00cf2180(Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    fn get(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// Doubles for the five helpers; the flag byte is `flag`.
    fn rig(flag: u8) -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(FILL_LOCAL, move |_, a| {
            sink.borrow_mut().push(("fill", a.to_vec()));
            Ret::default()
        });
        e.register_double(LOCAL_SCALAR, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register_double(LENGTH_FLAG, move |e, a| {
            e.mem.set_u8(a[1], flag);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(LOCAL_VECTOR, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BUILD_MATRIX, move |e, a| {
            sink.borrow_mut().push(("matrix", a.to_vec()));
            let rows = [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ];
            for (row, values) in rows.into_iter().enumerate() {
                for (lane, value) in values.into_iter().enumerate() {
                    e.mem
                        .set_f32(a[0] + 0x10 * row as u32 + 4 * lane as u32, value);
                }
            }
            Ret::default()
        });
        (e, log)
    }

    fn run(flag: u8) -> (Engine, Log, u32, u32, u32, u32) {
        let (mut e, log) = rig(flag);
        let state = e.mem.alloc(0xa0);
        put(&mut e, state + 0x50, [1.0, 2.0, 3.0, 4.0]);
        put(&mut e, state + 0x70, [5.0, 6.0, 7.0, 8.0]);
        put(&mut e, state + 0x80, [2.0, 3.0, 4.0, 0.0]);
        let first = e.mem.alloc(0x10);
        put(&mut e, first, [10.0, 11.0, 12.0, 13.0]);
        let second = e.mem.alloc(0x10);
        put(&mut e, second, [20.0, 21.0, 22.0, 23.0]);
        let position = e.mem.alloc(0x10);
        put(&mut e, position, [30.0, 31.0, 32.0, 33.0]);
        e.call(0x00cf_2180, &args![position, first, second, state]);
        (e, log, state, first, second, position)
    }

    /// The vectors shift and are replaced, the position's first and last lanes land in the
    /// shifted vectors, the long local gives the scaled vector with the float in its last
    /// lane, and the final vector is the base minus the combination of the matrix rows.
    #[test]
    fn state_is_updated_when_the_local_is_long() {
        let (e, log, state, _, second, _) = run(1);
        assert_eq!(get(&e, state + 0x40), [1.0, 2.0, 3.0, 30.0]);
        assert_eq!(get(&e, state + 0x50), [10.0, 11.0, 12.0, 33.0]);
        assert_eq!(get(&e, state + 0x60), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(get(&e, state + 0x70), [20.0, 21.0, 22.0, 23.0]);
        assert_eq!(get(&e, state + 0x90), [2.5, 5.0, 7.5, 2.5]);
        assert_eq!(get(&e, state + 0x30), [8.0, 8.0, 8.0, 33.0]);
        let calls = log.borrow();
        assert_eq!(calls[0].1[1..], [second, state + 0x60]);
        assert_eq!(calls[1], ("matrix", vec![state, state + 0x70]));
    }

    /// A short local gives a zero vector: the first three lanes are 0 and the last the float.
    #[test]
    fn a_short_local_gives_a_zero_vector() {
        let (e, _, state, ..) = run(0);
        assert_eq!(get(&e, state + 0x90), [0.0, 0.0, 0.0, 2.5]);
    }

    #[test]
    fn fn_00cf2280_masks_the_first_vector_and_copies_the_second() {
        let mut e = Engine::new();
        e.map(0x010c_4000, 0x1000);
        for (i, word) in [0xffff_ffffu32, 0xffff_ffff, 0xffff_ffff, 0]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(MASK_010C4C10 + 4 * i as u32, *word);
        }
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(first + 4 * i, 0x1111_1111 * (i + 1));
            e.mem.set_u32(second + 4 * i, 0xa000_0000 + i);
        }
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x44, 0x1234);
        e.call(
            0x00cf_2280,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(first),
                Ptr::<()>::new(second)
            ],
        );
        let words = |e: &Engine, offset: u32| [0, 4, 8, 12].map(|o| e.mem.u32(this + offset + o));
        let masked = [0x1111_1111, 0x2222_2222, 0x3333_3333, 0];
        assert_eq!(words(&e, 0), masked);
        assert_eq!(words(&e, 0x10), masked);
        let copied = [0xa000_0000, 0xa000_0001, 0xa000_0002, 0xa000_0003];
        assert_eq!(words(&e, 0x20), copied);
        assert_eq!(words(&e, 0x30), copied);
        assert_eq!(words(&e, 0x40), [0; 4]);
    }
}

//! `BSCore/bsrandom.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `GetTickCount` (the import slot at `00fdf060`, called through it).
const GET_TICK_COUNT: u32 = 0x00fd_f060;
/// The seeding routine (`00aa51a0`, `thiscall`, one `uint`; translated below):
/// fills the state from a seed.
const SEED: u32 = 0x00aa_51a0;
/// The Mersenne twister `mag01` table (two words: 0 and `0x9908b0df`).
const MAG01: u32 = 0x011a_c3b0;

/// The factor `00aa51a0` multiplies each state word by (`IMUL ECX,ECX,0x10dcd`).
const SEED_MULTIPLIER: u32 = 0x0001_0dcd;

/// Number of state words.
const STATE_SIZE: u32 = 0x270;
/// Distance, in words, to the state word mixed into each new word.
const STATE_SHIFT: u32 = 397;
/// The index that means "never seeded".
const NOT_SEEDED: u32 = 0x271;

layout! {
    /// `BSRandom` (Xbox PDB), 0x9cc bytes: a Mersenne twister generator.
    pub struct BSRandom: 0x9cc {
        /// `rseed` (Xbox PDB).
        0x000 rseed: u32,
        /// `rseed_sp` (Xbox PDB).
        0x004 rseed_sp: u32,
        /// `mt` (Xbox PDB): the 624 state words start here (`mt[0]` at +8).
        0x008 mt: u32,
        /// `mti` (Xbox PDB): the index of the next state word to hand out;
        /// `0x271` means the state was never seeded.
        0x9c8 mti: u32,
    }
}

/// Address of state word `index`.
fn state_word(this: Ptr<BSRandom>, index: u32) -> u32 {
    this.addr() + 8 + index * 4
}

/// One twist step: the new value of `state[index]`, from the high bit of
/// `state[index]`, the low bits of `state[next]` and the word `state[other]`.
fn twist(e: &Engine, this: Ptr<BSRandom>, index: u32, next: u32, other: u32) -> u32 {
    let high = e.mem.u32(state_word(this, index));
    let low = e.mem.u32(state_word(this, next));
    let mixed = (high & 0x8000_0000) | (low & 0x7fff_ffff);
    let magic: u32 = e.global(MAG01 + (low & 1) * 4);
    (mixed >> 1) ^ e.mem.u32(state_word(this, other)) ^ magic
}

// Translated from 00aa5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRandom::BSRandom` (Xbox PDB): `rseed` 1, `rseed_sp` 0 and the index
/// `0x271` (not seeded: the first draw seeds from `GetTickCount`).
pub fn bs_random_bs_random(e: &mut Engine, this: Ptr<BSRandom>) -> Ptr<BSRandom> {
    e.set(this, BSRandom::rseed, 1);
    e.set(this, BSRandom::rseed_sp, 0);
    e.set(this, BSRandom::mti, NOT_SEEDED);
    this
}

// Translated from 00aa51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, one `uint`): seeds the state.
/// `mt[0]` is the seed; each following word, up to the index `0x270`, is the
/// previous word times `0x10dcd` (69069, 32-bit wrapping). Leaves `mti` at `0x270`
/// (a full state, regenerated on the next draw). Returns nothing.
pub fn fn_00aa51a0(e: &mut Engine, this: Ptr<BSRandom>, seed: u32) {
    e.mem.set_u32(state_word(this, 0), seed);
    e.set(this, BSRandom::mti, 1);
    while e.get(this, BSRandom::mti) < STATE_SIZE {
        let index = e.get(this, BSRandom::mti);
        let previous = e.mem.u32(state_word(this, index - 1));
        e.mem.set_u32(
            state_word(this, index),
            previous.wrapping_mul(SEED_MULTIPLIER),
        );
        e.set(this, BSRandom::mti, index + 1);
    }
}

// Translated from 00aa5230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRandom::UnsignedInt` (Xbox PDB): the next state word (untempered),
/// reduced to `maximum` (0 answers 0). Once all 624 words are used (index
/// `>= 0x270`) the state is regenerated first, seeding from `GetTickCount`
/// through `00aa51a0` when the index is `0x271`. A maximum of `0xffffffff` or
/// `0x7fff` masks the word with it; any other maximum takes the remainder.
pub fn bs_random_unsigned_int(e: &mut Engine, this: Ptr<BSRandom>, maximum: u32) -> u32 {
    if maximum == 0 {
        return 0;
    }
    if e.get(this, BSRandom::mti) >= STATE_SIZE {
        if e.get(this, BSRandom::mti) == NOT_SEEDED {
            let ticks = e.call(GET_TICK_COUNT, &[]).u32();
            e.call(SEED, &args![this, ticks]);
        }
        for k in 0..STATE_SIZE - STATE_SHIFT {
            let value = twist(e, this, k, k + 1, k + STATE_SHIFT);
            e.mem.set_u32(state_word(this, k), value);
        }
        for k in STATE_SIZE - STATE_SHIFT..STATE_SIZE - 1 {
            let value = twist(e, this, k, k + 1, k + STATE_SHIFT - STATE_SIZE);
            e.mem.set_u32(state_word(this, k), value);
        }
        let last = twist(e, this, STATE_SIZE - 1, 0, STATE_SHIFT - 1);
        e.mem.set_u32(state_word(this, STATE_SIZE - 1), last);
        e.set(this, BSRandom::mti, 0);
    }
    let index = e.get(this, BSRandom::mti);
    let word = e.mem.u32(state_word(this, index));
    e.set(this, BSRandom::mti, index.wrapping_add(1));
    if maximum == 0xffff_ffff || maximum == 0x7fff {
        word & maximum
    } else {
        word % maximum
    }
}

// Translated from 00aa5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, one word, `RET 4`): a
/// `BSRandom` constructor with a seed: `rseed_sp` 0, `mti` 0x271, then the seeding
/// `00aa51a0` with `seed`. Answers `this`: the exe ends on that call and leaves its `EAX`,
/// which `00aa51a0` last loads with `this` before its loop test.
pub fn fn_00aa5170(e: &mut Engine, this: Ptr<BSRandom>, seed: u32) -> Ptr<BSRandom> {
    e.set(this, BSRandom::rseed_sp, 0);
    e.set(this, BSRandom::mti, NOT_SEEDED);
    fn_00aa51a0(e, this, seed);
    this
}

// Translated from 00aa5220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, no stack words): does
/// nothing.
pub fn fn_00aa5220(_e: &mut Engine, _this: Ptr<BSRandom>) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_5170,
            fn_00aa5170(Ptr<BSRandom>, u32) -> Ptr<BSRandom>
        ),
        entry!(0x00aa_5220, fn_00aa5220(Ptr<BSRandom>)),
        entry!(0x00aa51a0, fn_00aa51a0(Ptr<BSRandom>, u32)),
        entry!(
            0x00aa5140,
            bs_random_bs_random(Ptr<BSRandom>) -> Ptr<BSRandom>
        ),
        entry!(
            0x00aa5230,
            bs_random_unsigned_int(Ptr<BSRandom>, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAGIC: u32 = 0x9908_b0df;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(MAG01, 8);
        e.mem.set_u32(MAG01, 0);
        e.mem.set_u32(MAG01 + 4, MAGIC);
        e
    }

    /// A generator whose state word `k` is `k * 0x9e3779b9 + 1`.
    fn generator(e: &mut Engine, index: u32) -> (Ptr<BSRandom>, Vec<u32>) {
        let object = e.new_object::<BSRandom>();
        let mut state = Vec::new();
        for k in 0..STATE_SIZE {
            let word = k.wrapping_mul(0x9e37_79b9).wrapping_add(1);
            e.mem.set_u32(state_word(object, k), word);
            state.push(word);
        }
        e.set(object, BSRandom::mti, index);
        (object, state)
    }

    /// The regeneration as the reference Mersenne twister writes it.
    fn regenerate(s: &mut [u32]) {
        let mag = |y: u32| if y & 1 == 0 { 0 } else { MAGIC };
        for k in 0..227 {
            let y = (s[k] & 0x8000_0000) | (s[k + 1] & 0x7fff_ffff);
            s[k] = s[k + 397] ^ (y >> 1) ^ mag(y);
        }
        for k in 227..623 {
            let y = (s[k] & 0x8000_0000) | (s[k + 1] & 0x7fff_ffff);
            s[k] = s[k - 227] ^ (y >> 1) ^ mag(y);
        }
        let y = (s[623] & 0x8000_0000) | (s[0] & 0x7fff_ffff);
        s[623] = s[396] ^ (y >> 1) ^ mag(y);
    }

    #[test]
    fn the_constructor_marks_the_generator_unseeded() {
        let mut e = engine();
        let object = e.mem.alloc(0x9cc);
        e.mem.write(object, &[0xcd; 0x20]);
        assert_eq!(e.call(0x00aa5140, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 1);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 0x9c8), 0x271);
        assert_eq!(e.mem.u32(object + 8), 0xcdcd_cdcd);
    }

    #[test]
    fn seeding_chains_each_word_from_the_previous_one() {
        let mut e = engine();
        let object = e.new_object::<BSRandom>();
        e.set(object, BSRandom::mti, 0x271);
        e.call(0x00aa51a0, &args![object, 0x1234_5678u32]);
        let mut word = 0x1234_5678u32;
        assert_eq!(e.mem.u32(state_word(object, 0)), word);
        for k in 1..STATE_SIZE {
            word = word.wrapping_mul(69069);
            assert_eq!(e.mem.u32(state_word(object, k)), word, "word {k}");
        }
        assert_eq!(e.mem.u32(object.addr() + 0x9c8), 0x270);
        // The first product wraps in 32 bits.
        e.call(0x00aa51a0, &args![object, 0xffff_ffffu32]);
        assert_eq!(
            e.mem.u32(state_word(object, 1)),
            0xffff_ffffu32.wrapping_mul(69069)
        );
    }

    #[test]
    fn a_zero_maximum_answers_zero_and_draws_nothing() {
        let mut e = engine();
        let (object, _) = generator(&mut e, 5);
        assert_eq!(e.call(0x00aa5230, &args![object, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(object.addr() + 0x9c8), 5);
    }

    #[test]
    fn draws_the_next_word_reduced_to_the_maximum() {
        let mut e = engine();
        let (object, state) = generator(&mut e, 5);
        let by_remainder = e.call(0x00aa5230, &args![object, 100u32]).u32();
        assert_eq!(by_remainder, state[5] % 100);
        assert_eq!(e.mem.u32(object.addr() + 0x9c8), 6);
        let masked = e.call(0x00aa5230, &args![object, 0x7fffu32]).u32();
        assert_eq!(masked, state[6] & 0x7fff);
        let whole = e.call(0x00aa5230, &args![object, 0xffff_ffffu32]).u32();
        assert_eq!(whole, state[7]);
        assert_eq!(e.mem.u32(object.addr() + 0x9c8), 8);
    }

    #[test]
    fn a_used_up_state_is_regenerated() {
        let mut e = engine();
        let (object, mut state) = generator(&mut e, 0x270);
        e.register(SEED, |_, _| {
            panic!("a seeded generator is not seeded again")
        });
        let first = e.call(0x00aa5230, &args![object, 0xffff_ffffu32]).u32();
        regenerate(&mut state);
        assert_eq!(first, state[0]);
        assert_eq!(e.mem.u32(object.addr() + 0x9c8), 1);
        for (k, word) in state.iter().enumerate() {
            assert_eq!(e.mem.u32(state_word(object, k as u32)), *word, "word {k}");
        }
    }

    #[test]
    fn an_unseeded_generator_is_seeded_from_the_tick_count_first() {
        let mut e = engine();
        let (object, _) = generator(&mut e, 0x271);
        e.register(GET_TICK_COUNT, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(SEED, |e, a| {
            assert_eq!(a[1], 0x1234);
            for k in 0..STATE_SIZE {
                e.mem.set_u32(state_word(Ptr::new(a[0]), k), k + 7);
            }
            Ret::default()
        });
        let mut state: Vec<u32> = (0..STATE_SIZE).map(|k| k + 7).collect();
        e.call_log = Some(vec![]);
        let first = e.call(0x00aa5230, &args![object, 0xffff_ffffu32]).u32();
        regenerate(&mut state);
        assert_eq!(first, state[0]);
        let log = e.call_log.take().unwrap();
        let seeds: Vec<_> = log.iter().filter(|c| c.0 == SEED).collect();
        assert_eq!(seeds.len(), 1);
        assert_eq!(seeds[0].1, vec![object.addr(), 0x1234]);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00aa5170_seeds_a_new_generator_and_answers_this() {
        let mut e = Engine::new();
        let random = e.new_object::<BSRandom>();
        e.set(random, BSRandom::rseed, 9);
        e.set(random, BSRandom::rseed_sp, 5);
        assert_eq!(
            e.call(0x00aa_5170, &args![random, 7u32]).u32(),
            random.addr()
        );
        assert_eq!(e.get(random, BSRandom::rseed), 9);
        assert_eq!(e.get(random, BSRandom::rseed_sp), 0);
        assert_eq!(e.get(random, BSRandom::mti), STATE_SIZE);
        assert_eq!(e.mem.u32(random.addr() + 8), 7);
        assert_eq!(e.mem.u32(random.addr() + 12), 7 * SEED_MULTIPLIER);
    }

    #[test]
    fn fn_00aa5220_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00aa_5220, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

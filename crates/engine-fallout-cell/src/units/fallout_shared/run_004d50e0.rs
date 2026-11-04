//! `fallout shared/run_004d50e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::lipsynchanim::LipSynchAnim;

/// `LipSynchAnim::fPhonemeCoefficients` (Xbox PDB static), the 16 phoneme
/// weights' scales.
const PHONEME_COEFFICIENTS: u32 = 0x0118_9230;
/// `5.0` and `2.0` (`double`).
const FIVE: u32 = 0x0102_0998;
const TWO: u32 = 0x0101_1590;
/// `00559450`: the word at `this` (a string's text).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `LipSynchAnim::LoadFromFile` (engine map), `cdecl`, one word.
const LIP_SYNCH_ANIM_LOAD_FROM_FILE: u32 = 0x004d_52f0;

/// Phoneme `index`'s weight in the row of frame `row` (a float's bits).
fn weight(e: &Engine, rows: u32, row: u32, index: u32) -> u32 {
    let row = e.mem.u32(rows.wrapping_add(row.wrapping_mul(4)));
    e.mem.u32(row.wrapping_add(index.wrapping_mul(4)))
}

fn coefficient(e: &Engine, index: u32) -> u32 {
    e.mem.u32(PHONEME_COEFFICIENTS + index * 4)
}

// Translated from 004d50e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the lip-sync value of the animation at game frame `frame`:
/// the strongest scaled phoneme of that frame (weight times its
/// coefficient; the first of equals), its weight smoothed over the
/// neighbouring frames (five frames when two exist on each side, else the
/// frame and the previous one, else the frame and the next one, else the
/// frame alone), times its coefficient. 0 for a frame before the start
/// (the difference negative) or past the last.
pub fn fn_004d50e0(e: &mut Engine, this: Ptr<LipSynchAnim>, frame: u32) -> f32 {
    let index = frame.wrapping_sub(e.get(this, LipSynchAnim::iStartingFrame) as u32);
    let count = e.get(this, LipSynchAnim::iNumFrames);
    if (index as i32) < 0 || index >= count {
        return 0.0;
    }
    let rows = e.get(this, LipSynchAnim::ppPhonemes);
    let mut best = 0u32;
    let mut best_value = x87_store32(x87_mul(
        x87_load32(weight(e, rows, index, 0)),
        x87_m32(coefficient(e, 0)),
    ));
    for phoneme in 1..0x10u32 {
        let value = x87_mul(
            x87_load32(weight(e, rows, index, phoneme)),
            x87_m32(coefficient(e, phoneme)),
        );
        if x87_load32(best_value) < value {
            best = phoneme;
            best_value = x87_store32(x87_mul(
                x87_load32(weight(e, rows, index, phoneme)),
                x87_m32(coefficient(e, phoneme)),
            ));
        }
    }
    let rows = e.get(this, LipSynchAnim::ppPhonemes);
    let count = e.get(this, LipSynchAnim::iNumFrames);
    let value = if index as i32 > 1 && index < count.wrapping_sub(2) {
        let mut sum = x87_load32(weight(e, rows, index, best));
        for row in [
            index.wrapping_sub(1),
            index.wrapping_add(1),
            index.wrapping_sub(2),
            index.wrapping_add(2),
        ] {
            sum = x87_add(sum, x87_m32(weight(e, rows, row, best)));
        }
        x87_store32(x87_div(sum, e.global::<f64>(FIVE)))
    } else if index as i32 > 0 {
        let sum = x87_add(
            x87_load32(weight(e, rows, index, best)),
            x87_m32(weight(e, rows, index.wrapping_sub(1), best)),
        );
        x87_store32(x87_div(sum, e.global::<f64>(TWO)))
    } else if index < count.wrapping_sub(1) {
        let sum = x87_add(
            x87_load32(weight(e, rows, index, best)),
            x87_m32(weight(e, rows, index.wrapping_add(1), best)),
        );
        x87_store32(x87_div(sum, e.global::<f64>(TWO)))
    } else {
        x87_quiet_bits(weight(e, rows, index, best))
    };
    let result = x87_store32(x87_mul(x87_load32(value), x87_m32(coefficient(e, best))));
    f32::from_bits(result)
}

// Translated from 004d52d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: `LipSynchAnim::LoadFromFile` on the text of the
/// string `path`. `EAX` is left as that call's result, which this returns.
pub fn fn_004d52d0(e: &mut Engine, path: Ptr) -> u32 {
    let text = e.call(WORD_AT_ZERO, &args![path]).u32();
    e.call(LIP_SYNCH_ANIM_LOAD_FROM_FILE, &args![text]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_50e0, fn_004d50e0(Ptr<LipSynchAnim>, u32) -> f32),
        entry!(0x004d_52d0, fn_004d52d0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An animation of `frames` frames from frame 10 whose phoneme `p` of
    /// frame `f` weighs `f + 1` for `p == 3` and 0.5 elsewhere.
    fn anim(e: &mut Engine, frames: u32) -> Ptr<LipSynchAnim> {
        e.map(0x0118_9000, 0x1000);
        for p in 0..16 {
            e.mem.set_f32(PHONEME_COEFFICIENTS + 4 * p, 1.0);
        }
        e.mem.set_f32(PHONEME_COEFFICIENTS + 4 * 3, 2.0);
        e.map(0x0102_0000, 0x1000);
        e.set_global(FIVE, 5.0f64);
        e.map(0x0101_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        let this = e.new_object::<LipSynchAnim>();
        let rows = e.mem.alloc(4 * frames);
        for f in 0..frames {
            let row = e.mem.alloc(0x40);
            for p in 0..16 {
                let w = if p == 3 { (f + 1) as f32 } else { 0.5 };
                e.mem.set_f32(row + 4 * p, w);
            }
            e.mem.set_u32(rows + 4 * f, row);
        }
        e.set(this, LipSynchAnim::iNumFrames, frames);
        e.set(this, LipSynchAnim::iStartingFrame, 10);
        e.set(this, LipSynchAnim::ppPhonemes, rows);
        this
    }

    #[test]
    fn value_is_smoothed_by_position() {
        let mut e = Engine::new();
        let this = anim(&mut e, 6);
        let value = |e: &mut Engine, frame: u32| e.call(0x004d_50e0, &args![this, frame]).f32();
        // Out of range.
        assert_eq!(value(&mut e, 9), 0.0);
        assert_eq!(value(&mut e, 16), 0.0);
        // Index 2 (of 6): five frames, indices 2, 1, 3, 0 and 4.
        assert_eq!(value(&mut e, 12), (3.0 + 2.0 + 4.0 + 1.0 + 5.0) / 5.0 * 2.0);
        // Index 1: with the previous.
        assert_eq!(value(&mut e, 11), (2.0 + 1.0) / 2.0 * 2.0);
        // Index 5 (last but none after): with the previous.
        assert_eq!(value(&mut e, 15), (6.0 + 5.0) / 2.0 * 2.0);
        // Index 0: with the next.
        assert_eq!(value(&mut e, 10), (1.0 + 2.0) / 2.0 * 2.0);
        // A single frame: alone.
        let one = anim(&mut e, 1);
        assert_eq!(e.call(0x004d_50e0, &args![one, 10u32]).f32(), 2.0);
    }

    #[test]
    fn strongest_phoneme_wins_first_of_equals() {
        let mut e = Engine::new();
        let this = anim(&mut e, 1);
        let row = e.mem.u32(e.get(this, LipSynchAnim::ppPhonemes));
        // Phoneme 3 scaled is 2.0; phoneme 7 at 2.0 equals it: 3 is kept.
        e.mem.set_f32(row + 4 * 7, 2.0);
        assert_eq!(e.call(0x004d_50e0, &args![this, 10u32]).f32(), 2.0);
        // Phoneme 9 at 3.0 is stronger (coefficient 1).
        e.mem.set_f32(row + 4 * 9, 3.0);
        assert_eq!(e.call(0x004d_50e0, &args![this, 10u32]).f32(), 3.0);
    }

    #[test]
    fn load_passes_the_text() {
        let mut e = Engine::new();
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIP_SYNCH_ANIM_LOAD_FROM_FILE, |_, a| (a[0] + 1).into_ret());
        let path = e.mem.alloc(8);
        e.mem.set_u32(path, 0x4440);
        assert_eq!(e.call(0x004d_52d0, &args![path]).u32(), 0x4441);
    }
}

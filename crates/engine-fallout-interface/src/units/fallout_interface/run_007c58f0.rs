//! `fallout/interface/run_007c58f0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `abs` (`00ec7d40`, cdecl), the random index (`00944460`, cdecl: low and high),
/// the reels' spin (`007c56e0`, `thiscall`) and `SlotMachineMenu::InterpretSpin`
/// (`007c5760`, `thiscall`).
const ABS: u32 = 0x00ec_7d40;
const RANDOM_INDEX: u32 = 0x0094_4460;
const SPIN_007C56E0: u32 = 0x007c_56e0;
const INTERPRET_SPIN: u32 = 0x007c_5760;

/// The three reels' symbols at +0xe8, +0xec and +0xf0.
const REELS: u32 = 0xe8;

// Translated from 007c58f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the spin's outcome; the slot machine's luck nudge). The
/// luck (+0xc8) gives a chance: `abs((luck - 5) * 10)` plus a roll of 0 to 100 must pass
/// 100, else false. Below luck 5, outcomes 4 and 5 turn every symbol 12 into 13 and 6
/// sets all reels to the first reel's symbol plus one (0 past 6), both marking +0xd8;
/// false. From luck 5 (jump table `007c5e48`): 0 respins until `007c56e0` answers at
/// least 0 and nudges the interpreted spin; 1 and 5 make two matching reels three when
/// the squared difference to the third is at most the luck; 2 redraws the two reels
/// that are not 12 (or the second and third when the first is 12, the first and third
/// when the second is) from the symbol table (+0x2c, 8 bytes a symbol, +0xd0 of them)
/// and succeeds when the spin's value beats the outcome; 4 pulls the reels around the
/// symbol 12 together (the odd one changed) when their difference is at most the luck.
/// True when a reel was changed to win.
pub fn fn_007c58f0(e: &mut Engine, this: Ptr, outcome: i32) -> bool {
    let base = this.addr();
    let luck = |e: &Engine| e.mem.i32(base + 0xc8);
    let reel_at = |index: i32| {
        base.wrapping_add(REELS)
            .wrapping_add((index as u32).wrapping_mul(4))
    };
    let reel = |e: &Engine, index: i32| e.mem.i32(reel_at(index));
    let bonus = e
        .call(ABS, &args![luck(e).wrapping_sub(5).wrapping_mul(10)])
        .i32();
    let roll = e.call(RANDOM_INDEX, &args![0u32, 0x64u32]).i32();
    if roll.wrapping_add(bonus) <= 0x64 {
        return false;
    }
    if luck(e).wrapping_sub(5) < 0 {
        if outcome < 4 {
            return false;
        }
        if outcome <= 5 {
            for k in 0..3 {
                if reel(e, k) == 0xc {
                    e.mem.set_i32(reel_at(k), 0xd);
                }
            }
            e.mem.set_u8(base + 0xd8, 1);
        } else if outcome == 6 {
            let symbol = if reel(e, 0).wrapping_add(1) > 6 {
                0
            } else {
                reel(e, 0).wrapping_add(1)
            };
            for k in [2, 1, 0] {
                e.mem.set_i32(reel_at(k), symbol);
            }
            e.mem.set_u8(base + 0xd8, 1);
        }
        return false;
    }
    let squared_difference = |e: &mut Engine, a: i32, b: i32| {
        let difference = e
            .call(ABS, &args![reel(e, a).wrapping_sub(reel(e, b))])
            .i32();
        difference.wrapping_mul(difference)
    };
    let draw = |e: &mut Engine, index: i32| {
        let count = e.mem.u32(base + 0xd0);
        let pick = e.call(RANDOM_INDEX, &args![0u32, count]).u32();
        let symbol = e.mem.i32(
            base.wrapping_add(0x2c)
                .wrapping_add(pick.wrapping_shl(1).wrapping_mul(4)),
        );
        e.mem.set_i32(reel_at(index), symbol);
    };
    match outcome {
        0 => {
            while e.call(SPIN_007C56E0, &args![this]).i32() < outcome {}
            let spin = e.call(INTERPRET_SPIN, &args![this]).i32();
            fn_007c58f0(e, this, spin)
        }
        1 | 5 => {
            if reel(e, 0) == reel(e, 1) {
                if squared_difference(e, 0, 2) <= luck(e) {
                    let symbol = reel(e, 0);
                    e.mem.set_i32(reel_at(2), symbol);
                    return true;
                }
            } else if reel(e, 0) == reel(e, 2) {
                if squared_difference(e, 0, 1) <= luck(e) {
                    let symbol = reel(e, 0);
                    e.mem.set_i32(reel_at(1), symbol);
                    return true;
                }
            } else if squared_difference(e, 1, 0) <= luck(e) {
                let symbol = reel(e, 1);
                e.mem.set_i32(reel_at(0), symbol);
                return true;
            }
            false
        }
        2 => {
            if reel(e, 0) == 0xc {
                draw(e, 1);
                draw(e, 2);
            } else if reel(e, 1) == 0xc {
                draw(e, 0);
                draw(e, 2);
            } else {
                draw(e, 0);
                draw(e, 1);
            }
            e.call(INTERPRET_SPIN, &args![this]).i32() > outcome
        }
        4 => {
            let mut seven = 0i32;
            let mut odd = -1i32;
            for k in 0..3 {
                if reel(e, k) == 0xc {
                    seven = k;
                }
                if reel(e, k) % 2 != 0 {
                    odd = k;
                }
            }
            let before = if seven.wrapping_sub(1) < 0 {
                2
            } else {
                seven.wrapping_sub(1)
            };
            let after = if seven.wrapping_add(1) < 2 {
                seven.wrapping_add(1)
            } else {
                0
            };
            let difference = e
                .call(ABS, &args![reel(e, before).wrapping_sub(reel(e, after))])
                .i32();
            if difference <= luck(e) && odd < 0 {
                let symbol = reel(e, after);
                e.mem.set_i32(reel_at(before), symbol);
                return true;
            }
            if difference <= luck(e) {
                if before == odd {
                    let symbol = reel(e, after);
                    e.mem.set_i32(reel_at(before), symbol);
                } else {
                    let symbol = reel(e, before);
                    e.mem.set_i32(reel_at(after), symbol);
                }
                return true;
            }
            false
        }
        _ => false,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007c_58f0, fn_007c58f0(Ptr, i32) -> bool)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    fn machine(e: &mut Engine, luck: i32, reels: [i32; 3]) -> u32 {
        let this = e.mem.alloc(0x100);
        e.mem.set_i32(this + 0xc8, luck);
        for (k, symbol) in reels.iter().enumerate() {
            e.mem.set_i32(this + REELS + 4 * k as u32, *symbol);
        }
        this
    }

    fn reels(e: &Engine, this: u32) -> [i32; 3] {
        [
            e.mem.i32(this + REELS),
            e.mem.i32(this + REELS + 4),
            e.mem.i32(this + REELS + 8),
        ]
    }

    fn engine(roll: u32) -> Engine {
        let mut e = Engine::new();
        e.register(ABS, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        e.register_double(RANDOM_INDEX, move |_, a| {
            if a[1] == 0x64 { roll } else { 1 }.into_ret()
        });
        e
    }

    #[test]
    fn no_nudge_without_the_luck_roll() {
        let mut e = engine(50);
        let this = machine(&mut e, 9, [1, 1, 2]);
        assert!(!e.call(0x007c_58f0, &args![this, 1u32]).bool());
        assert_eq!(reels(&e, this), [1, 1, 2]);
    }

    #[test]
    fn bad_luck_spoils_sevens_and_sets_a_row() {
        let mut e = engine(90);
        let this = machine(&mut e, 2, [0xc, 3, 0xc]);
        assert!(!e.call(0x007c_58f0, &args![this, 4u32]).bool());
        assert_eq!(reels(&e, this), [0xd, 3, 0xd]);
        assert_eq!(e.mem.u8(this + 0xd8), 1);
        let this = machine(&mut e, 2, [6, 3, 4]);
        assert!(!e.call(0x007c_58f0, &args![this, 6u32]).bool());
        assert_eq!(reels(&e, this), [0, 0, 0]);
        let this = machine(&mut e, 2, [2, 3, 4]);
        e.call(0x007c_58f0, &args![this, 6u32]);
        assert_eq!(reels(&e, this), [3, 3, 3]);
        e.call(0x007c_58f0, &args![this, 3u32]);
        assert_eq!(reels(&e, this), [3, 3, 3]);
    }

    #[test]
    fn good_luck_completes_pairs_and_redraws() {
        let mut e = engine(95);
        let this = machine(&mut e, 9, [4, 4, 6]);
        assert!(e.call(0x007c_58f0, &args![this, 1u32]).bool());
        assert_eq!(reels(&e, this), [4, 4, 4]);
        let this = machine(&mut e, 8, [4, 7, 4]);
        assert!(!e.call(0x007c_58f0, &args![this, 5u32]).bool());
        let this = machine(&mut e, 8, [5, 7, 2]);
        assert!(e.call(0x007c_58f0, &args![this, 5u32]).bool());
        assert_eq!(reels(&e, this), [7, 7, 2]);
        // Outcome 2: the first reel is 12, the others drawn from symbol 1 (+0x34).
        let this = machine(&mut e, 9, [0xc, 1, 2]);
        e.mem.set_i32(this + 0x34, 8);
        e.mem.set_u32(this + 0xd0, 3);
        e.register(INTERPRET_SPIN, |_, _| 3u32.into_ret());
        assert!(e.call(0x007c_58f0, &args![this, 2u32]).bool());
        assert_eq!(reels(&e, this), [0xc, 8, 8]);
        // Outcome 4: the reels around the 12 pulled together.
        let this = machine(&mut e, 9, [0xc, 2, 4]);
        assert!(e.call(0x007c_58f0, &args![this, 4u32]).bool());
        assert_eq!(reels(&e, this), [0xc, 2, 2]);
        let this = machine(&mut e, 9, [0xc, 2, 5]);
        assert!(e.call(0x007c_58f0, &args![this, 4u32]).bool());
        assert_eq!(reels(&e, this), [0xc, 2, 2]);
        let this = machine(&mut e, 9, [0xc, 3, 4]);
        assert!(e.call(0x007c_58f0, &args![this, 4u32]).bool());
        assert_eq!(reels(&e, this), [0xc, 4, 4]);
        // The 12 in the middle pulls a reel onto itself.
        let this = machine(&mut e, 9, [2, 0xc, 4]);
        assert!(e.call(0x007c_58f0, &args![this, 4u32]).bool());
        assert_eq!(reels(&e, this), [2, 0xc, 4]);
        assert!(!e.call(0x007c_58f0, &args![this, 3u32]).bool());
    }

    #[test]
    fn outcome_zero_respins_and_nudges_the_result() {
        let mut e = engine(95);
        e.register(SPIN_007C56E0, |_, _| 0u32.into_ret());
        e.register(INTERPRET_SPIN, |_, _| 1u32.into_ret());
        let this = machine(&mut e, 9, [4, 4, 6]);
        assert!(e.call(0x007c_58f0, &args![this, 0u32]).bool());
        assert_eq!(reels(&e, this), [4, 4, 4]);
    }
}

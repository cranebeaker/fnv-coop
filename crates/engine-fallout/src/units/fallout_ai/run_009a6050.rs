//! `fallout/ai/run_009a6050` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00451010` (no words, unit `fallout shared/tes.cpp`): answers an object (it calls
/// `00559450` on `011ca0d8`).
const GET_OBJECT: u32 = 0x0045_1010;
/// `00560d30` (`thiscall`, one word: the index; unit `fallout shared/tesobjectrefr.cpp`):
/// answers `this + index * 4`.
const ELEMENT_ADDRESS: u32 = 0x0056_0d30;
/// `004587d0` (`cdecl`, one float; unit `fallout shared/tes.cpp`): answers the float times
/// the float at `011c3d18`, in `ST0`.
const SCALE_FLOAT: u32 = 0x0045_87d0;
/// The float `fn_009a6050` stores.
const RESULT_FLOAT: u32 = 0x011f_1c00;
/// `006219c0` (`thiscall`, no stack words): called twice by `fn_009a6080`.
const CALLED_AROUND: u32 = 0x0062_19c0;
/// `00460140` (`thiscall`, unit `fallout shared/tesdatahandler.cpp`): answers `this + 0x10`.
const MEMBER_AT_10: u32 = 0x0046_0140;
/// `00458b20` (no words, unit `fallout shared/tes.cpp`): answers `01267e30`.
const DEFAULT_OBJECT: u32 = 0x0045_8b20;

// Translated from 009a6050 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words; called by `00990f20`): takes the object `00451010`
/// answers, its `fn_009a6080` result, the address of word 2 of that (`00560d30` with 2; the
/// `PUSH 2` before the first call is that call's argument, `RET 4`), passes the float there
/// to `004587d0` and stores the negated result (`FCHS`) as a float at `011f1c00`.
pub fn fn_009a6050(e: &mut Engine) {
    let object = e.call(GET_OBJECT, &[]).u32();
    let base = fn_009a6080(e, object);
    let address = e.call(ELEMENT_ADDRESS, &args![base, 2u32]).u32();
    let value = x87_quiet_bits(e.mem.u32(address));
    let scaled = e.call(SCALE_FLOAT, &args![value]).f64();
    e.mem.set_u32(RESULT_FLOAT, x87_store32(-scaled));
}

// Translated from 009a6080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): asks `this` for an object (vtable
/// slot +0x94), calls `006219c0` on `this`, then answers the object's `00460140` result
/// (`object + 0x10`) or, when the object is null, `00458b20`'s (`01267e30`); calls
/// `006219c0` on `this` once more before answering.
pub fn fn_009a6080(e: &mut Engine, this: u32) -> u32 {
    let object = e.vcall(this, 0x94, &[]).u32();
    e.call(CALLED_AROUND, &args![this]);
    let result = if object != 0 {
        e.call(MEMBER_AT_10, &args![object]).u32()
    } else {
        e.call(DEFAULT_OBJECT, &[]).u32()
    };
    e.call(CALLED_AROUND, &args![this]);
    result
}

/// `004030b0` (unnamed, `alchemyitem.cpp` per the engine map): `thiscall` on the object
/// slot `+0x428` gives; a reference (or null).
const SLOT_OBJECT_REFERENCE: u32 = 0x0040_30b0;
/// `009a6ae0` (unnamed, `combatutilities.cpp`): `cdecl`, three words (the actor, the
/// candidate, a float out pointer or null); a `bool`.
const CAN_REACH: u32 = 0x009a_6ae0;
/// `CombatUtilities::GetDistanceBetweenFacingActors` (`009a64d0`, Xbox PDB): `cdecl`,
/// three words; a float in `ST0`.
const DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
/// `0x011e0e80`: the object `00717e50` is called on.
const PROCESS_OWNER: u32 = 0x011e_0e80;
/// `00717e50` (named `BaseProcess::GetActorPackageThatIsRunning` in the engine map; a
/// folded body): `thiscall`, no stack words; the list the loop walks.
const ACTOR_LIST: u32 = 0x0071_7e50;
/// `005be5c0` (unnamed): `thiscall`, one word (`RET 4`, given 0); the list's count.
const LIST_COUNT: u32 = 0x005b_e5c0;
/// `00968670` (unnamed, `playercharacter.cpp`): `thiscall`, one word (`RET 4`); element
/// `index` of the list.
const LIST_ELEMENT: u32 = 0x0096_8670;
/// `0x011f2250`: the array whose count (`0044ddc0`) is compared with 4.
const COUNTED_ARRAY: u32 = 0x011f_2250;
/// `0044ddc0` (unnamed): `thiscall`, returns the word at `+8`.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// The player character pointer (`0x011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `005d43c0` (unnamed): `thiscall`; the candidate's extra-data list.
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `00422700` (unnamed, `extradatalist.cpp`): `thiscall` on that list; an extra-data
/// object whose `+0xc` is the first node of a list of references.
const FOLLOWER_EXTRA: u32 = 0x0042_2700;
/// `006815c0` (unnamed): `thiscall`, returns `this` (a list node's item slot).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (unnamed): `thiscall`; the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `MobileObject::GetCurrentProcessType` (`00931850`, Xbox PDB): `thiscall`, a word.
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `Actor::LineOfSight` (`0088b880`, Xbox PDB): `thiscall`, five words (`RET 0x14`).
const LINE_OF_SIGHT: u32 = 0x0088_b880;
/// The float `FLT_MAX` (`0x01016970`) each distance starts from.
const FLOAT_MAX: u32 = 0x0101_6970;

/// Whether the actor's reach test passes for `candidate` and gives a distance no larger
/// than `best` (`FCOMPP` on `best` against it: taken when `C0` is clear). Returns the
/// distance float when taken.
fn reach_distance(e: &mut Engine, actor: u32, candidate: u32, best: u32) -> Option<u32> {
    e.with_stack(4, |e, local| {
        e.mem
            .set_u32(local.addr(), x87_quiet_bits(e.mem.u32(FLOAT_MAX)));
        let reached = e.call(CAN_REACH, &args![actor, candidate, local]).u8() != 0;
        let distance = e.mem.u32(local.addr());
        if reached && x87_load32(best) >= x87_load32(distance) {
            Some(distance)
        } else {
            None
        }
    })
}

/// The candidate filter the loop applies to the list's actors and to their followers:
/// not the actor itself, `owner`'s slot `+0x22c` (with 0) false or the actor is the
/// player, slot `+0x1d0` non-zero, and within `range` (`009a64d0` with 0; `FCOMPP` of
/// `range` against it, taken when `C0` is clear).
fn in_range(e: &mut Engine, actor: u32, owner: u32, candidate: u32, range: f32) -> bool {
    if candidate == actor {
        return false;
    }
    if e.vcall(owner, 0x22c, &args![0u32]).u8() != 0 && actor != e.mem.u32(PLAYER) {
        return false;
    }
    if e.vcall(candidate, 0x1d0, &[]).u32() == 0 {
        return false;
    }
    let distance = e
        .call(
            DISTANCE_BETWEEN_FACING_ACTORS,
            &args![actor, candidate, 0u32],
        )
        .f64();
    x87_load32(range.to_bits()) >= distance
}

// Translated from 009a60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words; `crates/world/src/melee.rs` traces it):
/// the actor `actor` should go for within `range`. The object its slot `+0x428` gives
/// (as a reference, `004030b0`) is answered when `009a6ae0` passes for it and it is within
/// `range` (`009a64d0` with 1); when it is set but fails, 0. Without one, the actors of the
/// list `00717e50` gives (count `005be5c0`, elements `00968670`; an element counts when its
/// slot `+0x100` is true) are searched, skipping one whose slot `+0x22c` is true while the
/// array at `0x011f2250` holds 4 items: each that passes the filter (`in_range`) and is
/// reached with a distance no larger than the best so far becomes the best, and so does each
/// reference in its follower list (`005d43c0`, `00422700`, from `+0xc`) that is set, has
/// process type 0 (`00931850`) and passes the same filter (the owner's slot `+0x22c` is
/// asked, not the follower's). The player, when not `actor`, then replaces the best when
/// within `range` and reached no farther (the best distance is not updated). A best
/// actor that `actor` must see (slot `+0x360`) but does not (`Actor::LineOfSight` of the
/// player with 1, it, 1, 0, 0) gives 0.
pub fn fn_009a60e0(e: &mut Engine, actor: u32, range: f32) -> u32 {
    let target = if e.vcall(actor, 0x428, &[]).u32() == 0 {
        0
    } else {
        let object = e.vcall(actor, 0x428, &[]).u32();
        e.call(SLOT_OBJECT_REFERENCE, &args![object]).u32()
    };
    if target != 0 && e.call(CAN_REACH, &args![actor, target, 0u32]).u8() != 0 {
        let distance = e
            .call(DISTANCE_BETWEEN_FACING_ACTORS, &args![actor, target, 1u32])
            .f64();
        if x87_load32(range.to_bits()) >= distance {
            return target;
        }
    }
    if target != 0 {
        return 0;
    }

    let mut best = 0u32;
    let mut best_distance = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
    let mut index = 0u32;
    loop {
        let list = e.call(ACTOR_LIST, &args![PROCESS_OWNER]).u32();
        let count = e.call(LIST_COUNT, &args![list, 0u32]).u32();
        if index >= count {
            break;
        }
        let list = e.call(ACTOR_LIST, &args![PROCESS_OWNER]).u32();
        let element = e.call(LIST_ELEMENT, &args![list, index]).u32();
        let candidate = if element != 0 && e.vcall(element, 0x100, &[]).u8() != 0 {
            element
        } else {
            0
        };
        let skip = e.call(ARRAY_COUNT, &args![COUNTED_ARRAY]).u32() == 4
            && candidate != 0
            && e.vcall(candidate, 0x22c, &args![0u32]).u8() != 0;
        if !skip {
            if candidate != 0 && in_range(e, actor, candidate, candidate, range) {
                if let Some(distance) = reach_distance(e, actor, candidate, best_distance) {
                    best = candidate;
                    best_distance = x87_quiet_bits(distance);
                }
            }
            if candidate != 0 {
                let extra_list = e.call(EXTRA_LIST, &args![candidate]).u32();
                let followers = e.call(FOLLOWER_EXTRA, &args![extra_list]).u32();
                if followers != 0 {
                    let mut node = e.mem.u32(followers + 0xc);
                    while node != 0 {
                        let slot = e.call(NODE_ITEM, &args![node]).u32();
                        if e.mem.u32(slot) == 0 {
                            break;
                        }
                        let slot = e.call(NODE_ITEM, &args![node]).u32();
                        let follower = e.mem.u32(slot);
                        if follower != 0
                            && e.call(GET_CURRENT_PROCESS_TYPE, &args![follower]).u32() == 0
                            && in_range(e, actor, candidate, follower, range)
                        {
                            if let Some(distance) =
                                reach_distance(e, actor, follower, best_distance)
                            {
                                best = follower;
                                best_distance = x87_quiet_bits(distance);
                            }
                        }
                        node = e.call(NODE_NEXT, &args![node]).u32();
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }

    if actor != e.mem.u32(PLAYER) {
        let player = e.mem.u32(PLAYER);
        let distance = e
            .call(DISTANCE_BETWEEN_FACING_ACTORS, &args![actor, player, 0u32])
            .f64();
        let distance = x87_store32(distance);
        let player = e.mem.u32(PLAYER);
        let reached = e.with_stack(4, |e, local| {
            e.mem
                .set_u32(local.addr(), x87_quiet_bits(e.mem.u32(FLOAT_MAX)));
            let reached = e.call(CAN_REACH, &args![actor, player, local]).u8() != 0;
            (reached, e.mem.u32(local.addr()))
        });
        if x87_load32(range.to_bits()) >= x87_load32(distance)
            && reached.0
            && x87_load32(best_distance) >= x87_load32(reached.1)
        {
            best = e.mem.u32(PLAYER);
        }
    }

    if best != 0 && e.vcall(actor, 0x360, &[]).u8() != 0 {
        let player = e.mem.u32(PLAYER);
        let seen = e
            .call(LINE_OF_SIGHT, &args![player, 1u32, best, 1u32, 0u32, 0u32])
            .u8();
        if seen == 0 {
            best = 0;
        }
    }
    best
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_6050, fn_009a6050()),
        entry!(0x009a_6080, fn_009a6080(u32) -> u32),
        entry!(0x009a_60e0, fn_009a60e0(u32, f32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn actor(e: &mut Engine, slot_94: u32) -> u32 {
        let this = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x94, slot_94);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn fn_009a6080_answers_the_member_or_the_default() {
        let mut e = Engine::new();
        e.register(0x0090_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0090_2000, |_, _| Ret::default());
        e.register(CALLED_AROUND, |_, _| Ret::default());
        e.register(MEMBER_AT_10, |_, a| (a[0] + 0x10).into_ret());
        e.register(DEFAULT_OBJECT, |_, _| 0x0126_7e30u32.into_ret());
        let with = actor(&mut e, 0x0090_1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_6080, &[with]).u32(), 0x7010);
        assert_eq!(calls_to(&e, CALLED_AROUND), vec![vec![with], vec![with]]);
        assert!(calls_to(&e, DEFAULT_OBJECT).is_empty());

        let without = actor(&mut e, 0x0090_2000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_6080, &[without]).u32(), 0x0126_7e30);
        assert!(calls_to(&e, MEMBER_AT_10).is_empty());
        assert_eq!(calls_to(&e, CALLED_AROUND).len(), 2);
    }

    #[test]
    fn fn_009a6050_stores_the_negated_scaled_float() {
        let mut e = Engine::new();
        e.map(RESULT_FLOAT & !0xfff, 0x1000);
        e.register(0x0090_1000, |_, _| 0x7000u32.into_ret());
        let this = actor(&mut e, 0x0090_1000);
        e.register_double(GET_OBJECT, move |_, _| this.into_ret());
        e.register(CALLED_AROUND, |_, _| Ret::default());
        e.register(MEMBER_AT_10, |_, a| (a[0] + 0x10).into_ret());
        let floats = e.mem.alloc(0x10);
        e.mem.set_f32(floats + 8, 1.5);
        e.register_double(ELEMENT_ADDRESS, move |_, a| {
            assert_eq!(a, &[0x7010, 2]);
            (floats + 8).into_ret()
        });
        e.register(SCALE_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_6050, &[]);
        assert_eq!(e.mem.f32(RESULT_FLOAT), -3.0);
        assert_eq!(calls_to(&e, SCALE_FLOAT), vec![vec![1.5f32.to_bits()]]);

        // A signalling NaN is quieted by the FLD/FSTP copy before the call.
        e.mem.set_u32(floats + 8, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x009a_6050, &[]);
        assert_eq!(calls_to(&e, SCALE_FLOAT), vec![vec![0x7fc0_0001]]);
    }
}

#[cfg(test)]
mod target_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The doubles read each object's answers from its own words: slot `+0x100` at
    /// `+0x10`, `+0x1d0` at `+0x14`, `+0x22c` at `+0x18`, `+0x360` at `+0x1c`, `+0x428`
    /// at `+0x20`, the reach distance (float) at `+0x24`, reach success at `+0x28`, the
    /// facing distance (float) at `+0x2c`, the follower extra at `+0x30`, the process
    /// type at `+0x34`.
    fn doubles(e: &mut Engine) {
        let slots = [
            (0x100u32, 0x10u32),
            (0x1d0, 0x14),
            (0x22c, 0x18),
            (0x360, 0x1c),
            (0x428, 0x20),
        ];
        let mut table = vec![0u32; 0x430 / 4];
        for (slot, _) in slots {
            table[slot as usize / 4] = 0x7a00_0000 + slot;
        }
        e.put_vtable(0x7000, &table);
        e.register(0x7a00_0100, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x7a00_01d0, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(0x7a00_022c, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(0x7a00_0360, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(0x7a00_0428, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(SLOT_OBJECT_REFERENCE, |_, a| ret(a[0]));
        e.register(CAN_REACH, |e, a| {
            if a[2] != 0 {
                let distance = e.mem.u32(a[1] + 0x24);
                e.mem.set_u32(a[2], distance);
            }
            ret(e.mem.u32(a[1] + 0x28))
        });
        e.register(DISTANCE_BETWEEN_FACING_ACTORS, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[1] + 0x2c)),
            ..Ret::default()
        });
        e.register(EXTRA_LIST, |_, a| ret(a[0] + 0x30));
        e.register(FOLLOWER_EXTRA, |e, a| ret(e.mem.u32(a[0])));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GET_CURRENT_PROCESS_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        e.register(ACTOR_LIST, |e, _| ret(e.mem.u32(0x7f00)));
        e.register(LIST_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        e.register(ARRAY_COUNT, |_, _| ret(0));
        e.register(LINE_OF_SIGHT, |e, a| ret(e.mem.u32(a[2] + 0x38)));
        e.map(PLAYER & !0xfff, 0x1000);
        e.map(FLOAT_MAX & !0xfff, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
    }

    /// An object answering the doubles: reach distance `reach`, facing distance `facing`.
    fn actor(e: &mut Engine, reach: f32, facing: f32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x7000);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x14, 1);
        e.mem.set_f32(object + 0x24, reach);
        e.mem.set_u32(object + 0x28, 1);
        e.mem.set_f32(object + 0x2c, facing);
        e.mem.set_u32(object + 0x38, 1);
        object
    }

    fn set_list(e: &mut Engine, actors: &[u32]) {
        e.map(0x7f00, 0x100);
        let list = e.mem.alloc(4 + 4 * actors.len() as u32);
        e.mem.set_u32(list, actors.len() as u32);
        for (i, a) in actors.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *a);
        }
        e.mem.set_u32(0x7f00, list);
    }

    #[test]
    fn fn_009a60e0_answers_the_slot_target_in_range() {
        let mut e = Engine::new();
        doubles(&mut e);
        let me = actor(&mut e, 0.0, 0.0);
        let target = actor(&mut e, 1.0, 5.0);
        e.mem.set_u32(me + 0x20, target);
        let find = |e: &mut Engine, range: f32| e.call(0x009a_60e0, &args![me, range]).u32();
        assert_eq!(find(&mut e, 10.0), target);
        assert_eq!(find(&mut e, 5.0), target);
        // Set but out of range, or not reached: 0 without searching.
        assert_eq!(find(&mut e, 4.0), 0);
        e.mem.set_u32(target + 0x28, 0);
        assert_eq!(find(&mut e, 10.0), 0);
    }

    #[test]
    fn fn_009a60e0_searches_the_list_and_followers() {
        let mut e = Engine::new();
        doubles(&mut e);
        let me = actor(&mut e, 0.0, 0.0);
        let first = actor(&mut e, 7.0, 4.0);
        let second = actor(&mut e, 3.0, 6.0);
        let far = actor(&mut e, 0.5, 20.0);
        let follower = actor(&mut e, 1.0, 2.0);
        // The first actor's follower list holds the follower.
        let extra = e.mem.alloc(0x10);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, follower);
        e.mem.set_u32(extra + 0xc, node);
        e.mem.set_u32(first + 0x30, extra);
        set_list(&mut e, &[first, second, far]);
        let player = actor(&mut e, 0.1, 30.0);
        e.mem.set_u32(PLAYER, player);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), follower);
        // With process type 1 the follower is skipped: the second actor (3.0 < 7.0).
        e.mem.set_u32(follower + 0x34, 1);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), second);
        // The player within range and reached no farther replaces the best.
        e.mem.set_f32(player + 0x2c, 9.0);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), player);
    }

    #[test]
    fn fn_009a60e0_needs_line_of_sight_when_asked() {
        let mut e = Engine::new();
        doubles(&mut e);
        let me = actor(&mut e, 0.0, 0.0);
        let other = actor(&mut e, 3.0, 6.0);
        set_list(&mut e, &[other]);
        e.mem.set_u32(PLAYER, me);
        e.mem.set_u32(me + 0x1c, 1);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), other);
        e.mem.set_u32(other + 0x38, 0);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), 0);
        // A candidate whose slot +0x22c is true is skipped while the actor is not the
        // player.
        e.mem.set_u32(other + 0x38, 1);
        e.mem.set_u32(other + 0x18, 1);
        let player = actor(&mut e, 0.1, 30.0);
        e.mem.set_u32(PLAYER, player);
        assert_eq!(e.call(0x009a_60e0, &args![me, 10.0f32]).u32(), 0);
    }
}

//! `(unplaced)/run_00666080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `LockFreeMap<TESObjectREFR *, BSTreeNode *>` (RTTI) that
/// `006661e0` stores.
const LOCK_FREE_MAP_VTABLE: u32 = 0x0106_6454;
/// The two blocks the destructor frees (+0x0c, then +0x04).
const MAP_BLOCK_A: u32 = 0x0c;
const MAP_BLOCK_B: u32 = 0x04;
/// The two hazard records (+0x14 and +0x18) and the word at +0x10 that
/// `00666080` uses.
const RECORD_A: u32 = 0x14;
const RECORD_B: u32 = 0x18;
const WORD_10: u32 = 0x10;

/// Callees by address: `006662a0` (`thiscall`, two words; a byte),
/// `0044dec0` and `0044df00` (`thiscall` on a record; a word),
/// `006ecba0` (`thiscall` on a stack word, `(index, value)`), `00559450`
/// (`thiscall` on a stack word; a word), `004491c0` (`cdecl`, three words; a
/// byte), `00666510` (`thiscall`, one word), `00529000` (`thiscall`), the
/// map's virtual slot `0x40` on the object at +0x00, `0044c480`
/// (`thiscall`, one word, given 1), `00401030` (`cdecl` free) and
/// `0044b4b0` (the base destructor body).
const TRY_ENTER: u32 = 0x0066_62a0;
const RECORD_VALUE: u32 = 0x0044_dec0;
const RECORD_LAST: u32 = 0x0044_df00;
const SET_HAZARD: u32 = 0x006e_cba0;
const HAZARD_VALUE: u32 = 0x0055_9450;
const VALIDATE: u32 = 0x0044_91c0;
const COMMIT: u32 = 0x0066_6510;
const LEAVE: u32 = 0x0052_9000;
const SLOT_FINISHED: u32 = 0x40;
const LOCK_FREE_MAP_CLEAR: u32 = 0x0044_c480;
const FREE: u32 = 0x0040_1030;
const LOCK_FREE_MAP_BASE_DESTRUCT: u32 = 0x0044_b4b0;

// Translated from 00666080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00666080`): a lock-free operation
/// on the map with two words (the meaning of the callees is not named in
/// the code; they are called by address). It loops: `006662a0` with the
/// two words (a zero byte ends with false); the value of record +0x18 is
/// set into two stack hazards (indices 0 and 1, `006ecba0`), and
/// `004491c0` checks the last of record +0x14 (+8) against them, a zero
/// byte trying again. Then the value of record +0x14 goes into a third
/// hazard and `004491c0` checks the word at +0x10 against it and the
/// first: when it passes `00666510` takes the record's value, otherwise
/// `006662a0` runs again with the two words. The map's slot `0x40` runs on
/// the object at +0x00 and the result is true. Every exit runs `00529000`.
///
/// The compiler's constant loop condition never ends the loop.
pub fn fn_00666080(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let this = this.addr();
    // The three stack hazards: -0x10, -0x0c and -0x08 from the frame.
    let result = e.with_stack(12, |e, frame| {
        let hazard_a = frame.addr();
        let hazard_b = hazard_a + 4;
        let hazard_c = hazard_a + 8;
        loop {
            if e.call(TRY_ENTER, &args![this, first, second]).u8() == 0 {
                return false;
            }
            let value = e
                .call(RECORD_VALUE, &args![this.wrapping_add(RECORD_B)])
                .u32();
            e.call(SET_HAZARD, &args![hazard_a, 0u32, value]);
            let value = e
                .call(RECORD_VALUE, &args![this.wrapping_add(RECORD_B)])
                .u32();
            e.call(SET_HAZARD, &args![hazard_b, 1u32, value]);
            let from_a = e.call(HAZARD_VALUE, &args![hazard_a]).u32();
            let from_b = e.call(HAZARD_VALUE, &args![hazard_b]).u32();
            let last = e
                .call(RECORD_LAST, &args![this.wrapping_add(RECORD_A)])
                .u32();
            if e.call(VALIDATE, &args![last.wrapping_add(8), from_b, from_a])
                .u8()
                != 0
            {
                break;
            }
        }
        let value = e
            .call(RECORD_VALUE, &args![this.wrapping_add(RECORD_A)])
            .u32();
        e.call(SET_HAZARD, &args![hazard_c, 0u32, value]);
        let from_c = e.call(HAZARD_VALUE, &args![hazard_c]).u32();
        let from_a = e.call(HAZARD_VALUE, &args![hazard_a]).u32();
        let word = e.mem.u32(this.wrapping_add(WORD_10));
        if e.call(VALIDATE, &args![word, from_a, from_c]).u8() != 0 {
            let value = e
                .call(RECORD_VALUE, &args![this.wrapping_add(RECORD_A)])
                .u32();
            e.call(COMMIT, &args![this, value]);
        } else {
            e.call(TRY_ENTER, &args![this, first, second]);
        }
        let object = e.mem.u32(this);
        e.vcall(object, SLOT_FINISHED, &args![]);
        true
    });
    e.call(LEAVE, &args![this]);
    result
}

// Translated from 006661b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `LockFreeMap<TESObjectREFR *, BSTreeNode *>`: the destructor
/// (`006661e0`), then the free (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_006661b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006661e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006661e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006661e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<TESObjectREFR *, BSTreeNode *>`: the vtable, the entries
/// cleared (`0044c480` with 1), the blocks at +0x0c and +0x04 freed, then
/// the base destructor body (`0044b4b0`).
pub fn fn_006661e0(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    e.mem.set_u32(map, LOCK_FREE_MAP_VTABLE);
    e.call(LOCK_FREE_MAP_CLEAR, &args![this, 1u32]);
    let block = e.mem.u32(map.wrapping_add(MAP_BLOCK_A));
    e.call(FREE, &args![block]);
    let block = e.mem.u32(map.wrapping_add(MAP_BLOCK_B));
    e.call(FREE, &args![block]);
    e.call(LOCK_FREE_MAP_BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_6080, fn_00666080(Ptr, u32, u32) -> bool),
        entry!(0x0066_61b0, fn_006661b0(Ptr, u32) -> Ptr),
        entry!(0x0066_61e0, fn_006661e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const FINISHED: u32 = 0x00ee_0100;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The map with doubles: `006662a0` answers `enter` in turn, `004491c0`
    /// answers `validate` in turn; hazards hold what was set.
    fn map(enter: &'static [u32], validate: &'static [u32]) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        let mut answers = enter.iter();
        let l = log.clone();
        e.register_double(TRY_ENTER, move |_, a| {
            l.borrow_mut().push((TRY_ENTER, a.to_vec()));
            answers.next().copied().unwrap_or(0).into_ret()
        });
        let mut answers = validate.iter();
        let l = log.clone();
        e.register_double(VALIDATE, move |_, a| {
            l.borrow_mut().push((VALIDATE, a.to_vec()));
            answers.next().copied().unwrap_or(0).into_ret()
        });
        e.register(RECORD_VALUE, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RECORD_LAST, |_, a| (a[0] + 0x2000).into_ret());
        e.register(SET_HAZARD, |e, a| {
            e.mem.set_u32(a[0], a[2] + a[1]);
            Ret::default()
        });
        e.register(HAZARD_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        logged(&mut e, &log, COMMIT, 0);
        logged(&mut e, &log, LEAVE, 0);
        logged(&mut e, &log, FINISHED, 0);
        let vtable = e.mem.alloc(0x44);
        e.mem.set_u32(vtable + SLOT_FINISHED, FINISHED);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, object);
        e.mem.set_u32(this + WORD_10, 0x6600);
        (e, log, this)
    }

    #[test]
    fn fn_00666080_commits_after_validating() {
        // Validation fails once (loop again), then passes twice.
        let (mut e, log, this) = map(&[1, 1], &[0, 1, 1]);
        assert!(e.call(0x0066_6080, &args![this, 5u32, 6u32]).bool());
        let b = this + RECORD_B + 0x1000;
        let a = this + RECORD_A + 0x1000;
        let last = this + RECORD_A + 0x2000;
        assert_eq!(
            calls(&log, VALIDATE),
            [
                vec![last + 8, b + 1, b],
                vec![last + 8, b + 1, b],
                vec![0x6600, b, a],
            ]
        );
        assert_eq!(calls(&log, COMMIT), [vec![this, a]]);
        assert_eq!(calls(&log, TRY_ENTER).len(), 2);
        assert_eq!(calls(&log, FINISHED).len(), 1);
        assert_eq!(calls(&log, LEAVE), [vec![this]]);
    }

    #[test]
    fn fn_00666080_retries_the_entry_or_fails() {
        let (mut e, log, this) = map(&[1, 1], &[1, 0]);
        assert!(e.call(0x0066_6080, &args![this, 5u32, 6u32]).bool());
        assert!(calls(&log, COMMIT).is_empty());
        assert_eq!(calls(&log, TRY_ENTER), [vec![this, 5, 6], vec![this, 5, 6]]);
        let (mut e, log, this) = map(&[0], &[]);
        assert!(!e.call(0x0066_6080, &args![this, 5u32, 6u32]).bool());
        assert!(calls(&log, FINISHED).is_empty());
        assert_eq!(calls(&log, LEAVE), [vec![this]]);
    }

    #[test]
    fn destructor_and_deleting_destructor() {
        let mut e = Engine::new();
        let log = Log::default();
        for callee in [LOCK_FREE_MAP_CLEAR, FREE, LOCK_FREE_MAP_BASE_DESTRUCT] {
            logged(&mut e, &log, callee, 0);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + MAP_BLOCK_A, 0x5500);
        e.mem.set_u32(this + MAP_BLOCK_B, 0x4400);
        assert_eq!(e.call(0x0066_61b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LOCK_FREE_MAP_VTABLE);
        assert_eq!(
            *log.borrow(),
            [
                (LOCK_FREE_MAP_CLEAR, vec![this, 1]),
                (FREE, vec![0x5500]),
                (FREE, vec![0x4400]),
                (LOCK_FREE_MAP_BASE_DESTRUCT, vec![this]),
                (FREE, vec![this]),
            ]
        );
        log.borrow_mut().clear();
        e.call(0x0066_61b0, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), 4);
    }
}

//! `(unplaced)/run_00453850` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004538e0`: enters the critical section at `this` (`RET 4`, one unused
/// stack word); its own translation is in `run_004538e0.rs`.
const ENTER: u32 = 0x0045_38e0;

// Translated from 004538a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (`RET 4`): calls `004538e0(this, argument)`, which
/// enters the critical section at `this`.
pub fn fn_004538a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ENTER, &args![this, argument]);
}

/// `0082f1f0(this)`: leaves the critical section at `this`.
const LEAVE: u32 = 0x0082_f1f0;

// Translated from 004538c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding wrapper (no stack words): calls `0082f1f0(this)`, which
/// leaves the critical section at `this`. The engine map names this address
/// `RtlLeaveCriticalSection` after the Xbox import it stands for; on PC it is
/// this ordinary function.
pub fn fn_004538c0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE, &args![this]);
}

/// `0043d4d0` (thiscall): the address of a state word of the object at
/// `011c3ea4` (1 skips the lock change).
const STATE_WORD: u32 = 0x0043_d4d0;
/// The object `0043d4d0` is called on.
const STATE_OWNER: u32 = 0x011c_3ea4;
/// The critical section `004538a0` / `004538c0` enter and leave.
const LOCK: u32 = 0x011c_3d84;
/// A float global (`011dea30`).
const GLOBAL_011DEA30: u32 = 0x011d_ea30;

// Translated from 00453850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at `011dea30` (loaded into ST0).
pub fn fn_00453850(e: &mut Engine) -> f32 {
    x87_quiet(e.global::<f32>(GLOBAL_011DEA30))
}

// Translated from 00453860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless the state word of `011c3ea4`
/// (`0043d4d0`) is 1, enters (`enter` non-zero, `004538a0(0)`) or leaves
/// (`004538c0`) the critical section at `011c3d84`. `this` is not read.
pub fn fn_00453860(e: &mut Engine, _this: Ptr, enter: u8) {
    let state = e.call(STATE_WORD, &args![STATE_OWNER]).u32();
    if e.mem.u32(state) == 1 {
        return;
    }
    if enter != 0 {
        fn_004538a0(e, Ptr::new(LOCK), 0);
    } else {
        fn_004538c0(e, Ptr::new(LOCK));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004538a0, fn_004538a0(Ptr, u32)),
        entry!(0x004538c0, fn_004538c0(Ptr)),
        entry!(0x0045_3850, fn_00453850() -> f32),
        entry!(0x0045_3860, fn_00453860(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004538c0_forwards_this() {
        let mut e = Engine::new();
        e.register(LEAVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LEAVE, vec![0x1000]));
    }

    #[test]
    fn forwards_this_and_the_stack_word() {
        let mut e = Engine::new();
        e.register(ENTER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_38a0, &args![0x1000u32, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER, vec![0x1000, 7]));
    }

    #[test]
    fn fn_00453850_loads_the_float_global() {
        let mut e = Engine::new();
        e.map(GLOBAL_011DEA30, 4);
        e.set_global(GLOBAL_011DEA30, 0.75f32);
        assert_eq!(e.call(0x0045_3850, &[]).f32(), 0.75);
    }

    #[test]
    fn fn_00453860_enters_or_leaves_unless_the_state_is_1() {
        let mut e = Engine::new();
        let state = e.mem.alloc(4);
        e.register_double(STATE_WORD, move |_, _| Ret {
            eax: state,
            ..Ret::default()
        });
        e.register(ENTER, |_, _| Ret::default());
        e.register(LEAVE, |_, _| Ret::default());
        let run = |e: &mut Engine, enter: u32| -> Vec<(u32, Vec<u32>)> {
            e.call_log = Some(vec![]);
            e.call(0x0045_3860, &args![0x1234u32, enter]);
            e.call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == ENTER || c.0 == LEAVE)
                .collect()
        };
        assert_eq!(run(&mut e, 1), vec![(ENTER, vec![LOCK, 0])]);
        assert_eq!(run(&mut e, 0x100), vec![(LEAVE, vec![LOCK])]);
        e.mem.set_u32(state, 1);
        assert!(run(&mut e, 1).is_empty());
        assert!(run(&mut e, 0).is_empty());
    }
}

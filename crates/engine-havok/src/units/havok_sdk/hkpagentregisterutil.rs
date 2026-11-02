//! `Havok SDK/hkpagentregisterutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The argument of every function here is the collision dispatcher the agent functions are
//! registered with (the argument of `registerAllAgents`; its identity is unconfirmed). The
//! byte at `dispatcher + 0x1e23` is set by `00cfb1e0` (thiscall `this` = the dispatcher,
//! `(flag)`, `RET 4`); the callers push a dword of which only the low byte (0 or 1) is read.

#[allow(unused_imports)]
use crate::prelude::*;

/// Sets the byte at `dispatcher + 0x1e23` (`00cfb1e0`, `this` = dispatcher, `(flag)`).
const SET_FLAG: u32 = 0x00cf_b1e0;
/// Registers the agent functions of one group (`cdecl`, `(dispatcher)`), in the order the
/// three group functions of this file call them.
const REGISTER_00D314E0: u32 = 0x00d3_14e0;
const REGISTER_00D2F2C0: u32 = 0x00d2_f2c0;
const REGISTER_00D2E6D0: u32 = 0x00d2_e6d0;
const REGISTER_00D2DC70: u32 = 0x00d2_dc70;
const REGISTER_00D2DA20: u32 = 0x00d2_da20;
const REGISTER_00D36C60: u32 = 0x00d3_6c60;
const REGISTER_00D367D0: u32 = 0x00d3_67d0;
const REGISTER_00D362C0: u32 = 0x00d3_62c0;
const REGISTER_00D346B0: u32 = 0x00d3_46b0;
const REGISTER_00D331A0: u32 = 0x00d3_31a0;
const REGISTER_00D321E0: u32 = 0x00d3_21e0;
const REGISTER_00D41FD0: u32 = 0x00d4_1fd0;
/// `00d41f00` (`cdecl`, `(dispatcher, 0x12, 0x12)`).
const REGISTER_00D41F00: u32 = 0x00d4_1f00;
const REGISTER_00D41710: u32 = 0x00d4_1710;
const REGISTER_00D40910: u32 = 0x00d4_0910;
const REGISTER_00D40130: u32 = 0x00d4_0130;
const REGISTER_00D3FD00: u32 = 0x00d3_fd00;
const REGISTER_00D3EF70: u32 = 0x00d3_ef70;
const REGISTER_00D3DCA0: u32 = 0x00d3_dca0;
const REGISTER_00D3CEF0: u32 = 0x00d3_cef0;
const REGISTER_00D3C010: u32 = 0x00d3_c010;
const REGISTER_00D3ABC0: u32 = 0x00d3_abc0;
const REGISTER_00D39840: u32 = 0x00d3_9840;
const REGISTER_00D389F0: u32 = 0x00d3_89f0;
const REGISTER_00D377D0: u32 = 0x00d3_77d0;
const REGISTER_00D2E760: u32 = 0x00d2_e760;
const REGISTER_00D46410: u32 = 0x00d4_6410;
const REGISTER_00D45F80: u32 = 0x00d4_5f80;
const REGISTER_00D44D50: u32 = 0x00d4_4d50;
const REGISTER_00D44310: u32 = 0x00d4_4310;
const REGISTER_00D43120: u32 = 0x00d4_3120;
const REGISTER_00D42960: u32 = 0x00d4_2960;

// Translated from 00cbfc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: with the flag cleared, registers the five groups `00d314e0`, `00d2f2c0`,
/// `00d2e6d0`, `00d2dc70` and `00d2da20`, then sets the flag again.
pub fn fn_00cbfc10(e: &mut Engine, dispatcher: u32) {
    e.call(SET_FLAG, &args![dispatcher, 0u32]);
    for group in [
        REGISTER_00D314E0,
        REGISTER_00D2F2C0,
        REGISTER_00D2E6D0,
        REGISTER_00D2DC70,
        REGISTER_00D2DA20,
    ] {
        e.call(group, &args![dispatcher]);
    }
    e.call(SET_FLAG, &args![dispatcher, 1u32]);
}

// Translated from 00cbfc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: registers `00d36c60`, then (with the flag cleared) `00d367d0` and `00d362c0`,
/// sets the flag again and registers `00d346b0`, `00d331a0` and `00d321e0`.
pub fn fn_00cbfc60(e: &mut Engine, dispatcher: u32) {
    e.call(REGISTER_00D36C60, &args![dispatcher]);
    e.call(SET_FLAG, &args![dispatcher, 0u32]);
    e.call(REGISTER_00D367D0, &args![dispatcher]);
    e.call(REGISTER_00D362C0, &args![dispatcher]);
    e.call(SET_FLAG, &args![dispatcher, 1u32]);
    for group in [REGISTER_00D346B0, REGISTER_00D331A0, REGISTER_00D321E0] {
        e.call(group, &args![dispatcher]);
    }
}

// Translated from 00cbfcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: registers `00d41fd0`, `00d41f00` (with the two words `0x12`, `0x12`) and then
/// `00d41710`, `00d40910`, `00d40130`, `00d3fd00`, `00d3ef70`, `00d3dca0`, `00d3cef0`,
/// `00d3c010`, `00d3abc0`, `00d39840`, `00d389f0`, `00d377d0` and `00d2e760`.
pub fn fn_00cbfcb0(e: &mut Engine, dispatcher: u32) {
    e.call(REGISTER_00D41FD0, &args![dispatcher]);
    e.call(REGISTER_00D41F00, &args![dispatcher, 0x12u32, 0x12u32]);
    for group in [
        REGISTER_00D41710,
        REGISTER_00D40910,
        REGISTER_00D40130,
        REGISTER_00D3FD00,
        REGISTER_00D3EF70,
        REGISTER_00D3DCA0,
        REGISTER_00D3CEF0,
        REGISTER_00D3C010,
        REGISTER_00D3ABC0,
        REGISTER_00D39840,
        REGISTER_00D389F0,
        REGISTER_00D377D0,
        REGISTER_00D2E760,
    ] {
        e.call(group, &args![dispatcher]);
    }
}

// Translated from 00cbfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAgentRegisterUtil::registerAllAgents` (named in the engine map): registers `00d46410`,
/// `00d45f80` and `00d44d50`, then [`fn_00cbfc10`]; with the flag cleared `00d44310`, then the
/// flag set again; then [`fn_00cbfc60`], `00d43120`, `00d42960` and [`fn_00cbfcb0`].
pub fn hkp_agent_register_util_register_all_agents(e: &mut Engine, dispatcher: u32) {
    for group in [REGISTER_00D46410, REGISTER_00D45F80, REGISTER_00D44D50] {
        e.call(group, &args![dispatcher]);
    }
    fn_00cbfc10(e, dispatcher);
    e.call(SET_FLAG, &args![dispatcher, 0u32]);
    e.call(REGISTER_00D44310, &args![dispatcher]);
    e.call(SET_FLAG, &args![dispatcher, 1u32]);
    fn_00cbfc60(e, dispatcher);
    e.call(REGISTER_00D43120, &args![dispatcher]);
    e.call(REGISTER_00D42960, &args![dispatcher]);
    fn_00cbfcb0(e, dispatcher);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_fc10, fn_00cbfc10(u32)),
        entry!(0x00cb_fc60, fn_00cbfc60(u32)),
        entry!(0x00cb_fcb0, fn_00cbfcb0(u32)),
        entry!(
            0x00cb_fd20,
            hkp_agent_register_util_register_all_agents(u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every callee outside this file.
    const CALLEES: [u32; 33] = [
        SET_FLAG,
        REGISTER_00D314E0,
        REGISTER_00D2F2C0,
        REGISTER_00D2E6D0,
        REGISTER_00D2DC70,
        REGISTER_00D2DA20,
        REGISTER_00D36C60,
        REGISTER_00D367D0,
        REGISTER_00D362C0,
        REGISTER_00D346B0,
        REGISTER_00D331A0,
        REGISTER_00D321E0,
        REGISTER_00D41FD0,
        REGISTER_00D41F00,
        REGISTER_00D41710,
        REGISTER_00D40910,
        REGISTER_00D40130,
        REGISTER_00D3FD00,
        REGISTER_00D3EF70,
        REGISTER_00D3DCA0,
        REGISTER_00D3CEF0,
        REGISTER_00D3C010,
        REGISTER_00D3ABC0,
        REGISTER_00D39840,
        REGISTER_00D389F0,
        REGISTER_00D377D0,
        REGISTER_00D2E760,
        REGISTER_00D46410,
        REGISTER_00D45F80,
        REGISTER_00D44D50,
        REGISTER_00D44310,
        REGISTER_00D43120,
        REGISTER_00D42960,
    ];

    /// An engine with a double for every callee, logging the calls.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for address in CALLEES {
            e.register_double(address, |_, _| Ret::default());
        }
        e.call_log = Some(Vec::new());
        e
    }

    /// The calls to the callees, in order, as `(address, words)`; the flag calls keep only
    /// the low byte of the pushed dword.
    fn callee_calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| CALLEES.contains(a))
            .map(|(a, words)| {
                let mut words = words.clone();
                if *a == SET_FLAG {
                    words[1] &= 0xff;
                }
                (*a, words)
            })
            .collect()
    }

    /// The expected calls: `(address, extra words)` with the dispatcher first.
    fn expected(dispatcher: u32, list: &[(u32, &[u32])]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|(a, extra)| {
                let mut words = vec![dispatcher];
                words.extend_from_slice(extra);
                (*a, words)
            })
            .collect()
    }

    const FIRST_GROUP: [u32; 5] = [
        REGISTER_00D314E0,
        REGISTER_00D2F2C0,
        REGISTER_00D2E6D0,
        REGISTER_00D2DC70,
        REGISTER_00D2DA20,
    ];

    #[test]
    fn first_group_clears_the_flag_around_the_registrations() {
        let mut e = engine();
        e.call(0x00cb_fc10, &args![0x1234u32]);
        let mut list: Vec<(u32, &[u32])> = vec![(SET_FLAG, &[0])];
        list.extend(FIRST_GROUP.map(|a| (a, &[][..])));
        list.push((SET_FLAG, &[1]));
        assert_eq!(callee_calls(&e), expected(0x1234, &list));
    }

    #[test]
    fn second_group_sets_the_flag_around_two_registrations() {
        let mut e = engine();
        e.call(0x00cb_fc60, &args![0x1234u32]);
        let none: &[u32] = &[];
        assert_eq!(
            callee_calls(&e),
            expected(
                0x1234,
                &[
                    (REGISTER_00D36C60, none),
                    (SET_FLAG, &[0]),
                    (REGISTER_00D367D0, none),
                    (REGISTER_00D362C0, none),
                    (SET_FLAG, &[1]),
                    (REGISTER_00D346B0, none),
                    (REGISTER_00D331A0, none),
                    (REGISTER_00D321E0, none),
                ]
            )
        );
    }

    #[test]
    fn third_group_passes_the_two_extra_words_to_the_second_registration() {
        let mut e = engine();
        e.call(0x00cb_fcb0, &args![0x1234u32]);
        let none: &[u32] = &[];
        assert_eq!(
            callee_calls(&e),
            expected(
                0x1234,
                &[
                    (REGISTER_00D41FD0, none),
                    (REGISTER_00D41F00, &[0x12, 0x12]),
                    (REGISTER_00D41710, none),
                    (REGISTER_00D40910, none),
                    (REGISTER_00D40130, none),
                    (REGISTER_00D3FD00, none),
                    (REGISTER_00D3EF70, none),
                    (REGISTER_00D3DCA0, none),
                    (REGISTER_00D3CEF0, none),
                    (REGISTER_00D3C010, none),
                    (REGISTER_00D3ABC0, none),
                    (REGISTER_00D39840, none),
                    (REGISTER_00D389F0, none),
                    (REGISTER_00D377D0, none),
                    (REGISTER_00D2E760, none),
                ]
            )
        );
    }

    #[test]
    fn register_all_agents_runs_the_three_groups_in_order() {
        let mut e = engine();
        e.call(0x00cb_fd20, &args![0x1234u32]);
        let calls = callee_calls(&e);
        let addresses: Vec<u32> = calls.iter().map(|(a, _)| *a).collect();
        let mut want = vec![REGISTER_00D46410, REGISTER_00D45F80, REGISTER_00D44D50];
        // First group.
        want.push(SET_FLAG);
        want.extend(FIRST_GROUP);
        want.push(SET_FLAG);
        // The flag around 00d44310.
        want.extend([SET_FLAG, REGISTER_00D44310, SET_FLAG]);
        // Second group.
        want.extend([
            REGISTER_00D36C60,
            SET_FLAG,
            REGISTER_00D367D0,
            REGISTER_00D362C0,
            SET_FLAG,
            REGISTER_00D346B0,
            REGISTER_00D331A0,
            REGISTER_00D321E0,
            REGISTER_00D43120,
            REGISTER_00D42960,
            REGISTER_00D41FD0,
            REGISTER_00D41F00,
        ]);
        want.extend([
            REGISTER_00D41710,
            REGISTER_00D40910,
            REGISTER_00D40130,
            REGISTER_00D3FD00,
            REGISTER_00D3EF70,
            REGISTER_00D3DCA0,
            REGISTER_00D3CEF0,
            REGISTER_00D3C010,
            REGISTER_00D3ABC0,
            REGISTER_00D39840,
            REGISTER_00D389F0,
            REGISTER_00D377D0,
            REGISTER_00D2E760,
        ]);
        assert_eq!(addresses, want);
        assert!(calls.iter().all(|(_, words)| words[0] == 0x1234));
        let flags: Vec<u32> = calls
            .iter()
            .filter(|(a, _)| *a == SET_FLAG)
            .map(|(_, words)| words[1])
            .collect();
        assert_eq!(flags, vec![0, 1, 0, 1, 0, 1]);
    }
}

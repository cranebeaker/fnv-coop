//! `fallout/misc/run_008467b0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The save/load manager pointer (`011de134`).
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// `00850760` (`thiscall`, `RET 0x10`; engine-map unit
/// `bgssaveloadmanager.cpp`).
const MANAGER_00850760: u32 = 0x0085_0760;

// Translated from 008467b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `00850760` on the save/load
/// manager (`011de134`) with this object's words at +0 and +0x34, then 0
/// and 0. Answers that call's `EAX` (whether the load went ahead; the start
/// menu's `007d3543` keeps its `AL`).
pub fn fn_008467b0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.mem.u32(this.addr());
    let second = e.mem.u32(this.addr() + 0x34);
    let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
    // 008467ce CALL 00850760; 008467d3..008467d6 (MOV ESP,EBP / POP EBP / RET) keep its EAX.
    e.call(MANAGER_00850760, &args![manager, first, second, 0u32, 0u32])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    // 008467d6 RET: EAX is still 00850760's answer.
    vec![entry!(0x0084_67b0, fn_008467b0(Ptr) -> u32)]
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_008467b0_passes_the_two_words() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(SAVE_LOAD_MANAGER, 0x6000);
        e.register(MANAGER_00850760, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this, 0x11);
        e.mem.set_u32(this + 0x34, 0x22);
        e.call_log = Some(vec![]);
        e.call(0x0084_67b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == MANAGER_00850760).collect();
        assert_eq!(calls[0].1, vec![0x6000, 0x11, 0x22, 0, 0]);
    }

    /// The answer is `00850760`'s `EAX`, passed on unchanged (both a load that went ahead and
    /// one that did not).
    #[test]
    fn fn_008467b0_answers_the_managers_eax() {
        for answer in [1u32, 0] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            e.set_global::<u32>(SAVE_LOAD_MANAGER, 0x6000);
            e.register_double(MANAGER_00850760, move |_, _| Ret {
                eax: answer,
                ..Ret::default()
            });
            let this = e.mem.alloc(0x38);
            assert_eq!(e.call(0x0084_67b0, &args![this]).u32(), answer);
        }
    }
}

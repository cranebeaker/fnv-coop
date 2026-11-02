//! `NiParticle/run_00c283b0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a6d220` (unnamed, `NiMain/nitimecontroller.cpp`, `thiscall`, no
/// words): the time-controller constructor the creation function runs.
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
/// The `NiPSysResetOnLoopCtlr` vtable (RTTI `.?AVNiPSysResetOnLoopCtlr@@`).
const VTABLE: u32 = 0x010b_e8e4;
/// The float copied to +0x34: `01015f5c` (the bits 0xFF7FFFFF, -FLT_MAX).
const FIELD_INIT: u32 = 0x0101_5f5c;

// Translated from 00c283b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysResetOnLoopCtlr` creation function (Ghidra made no function
/// of it; `cdecl`, no words): 0x38 bytes from the Ni allocator, constructed
/// by `00a6d220`, the float at +0x34 from `01015f5c` (`FLD`/`FSTP`) and the
/// vtable; null when the allocation fails.
pub fn fn_00c283b0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![block]);
    let value = x87_quiet_bits(e.global::<u32>(FIELD_INIT));
    e.mem.set_u32(block + 0x34, value);
    e.mem.set_u32(block, VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_83b0, fn_00c283b0() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c283b0_constructs_a_reset_on_loop_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_83b0, &args![]).u32(), 0);
        e.map(FIELD_INIT, 4);
        e.set_global(FIELD_INIT, 0xff7f_ffffu32);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TIME_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_83b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(e.mem.u32(made + 0x34), 0xff7f_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x38]));
        assert_eq!(log[2], (TIME_CONTROLLER_CONSTRUCTOR, vec![made]));
    }
}

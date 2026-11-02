//! `NiParticle/run_00c2b520` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00c32d80` (unnamed, `thiscall`, no words): the controller constructor
/// the creation function runs.
const CONTROLLER_CONSTRUCTOR: u32 = 0x00c3_2d80;
/// The `NiPSysEmitterPlanarAngleCtlr` vtable (RTTI `.?AVNiPSysEmitterPlanarAngleCtlr@@`).
const VTABLE: u32 = 0x010b_f854;

// Translated from 00c2b520 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterPlanarAngleCtlr` creation function (Ghidra made no function of
/// it; `cdecl`, no words): 0x40 bytes from the Ni allocator, constructed by
/// `00c32d80` and given the `NiPSysEmitterPlanarAngleCtlr` vtable; null when the allocation
/// fails.
pub fn fn_00c2b520(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(CONTROLLER_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_b520, fn_00c2b520() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c2b520_constructs_the_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(CONTROLLER_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_b520, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_b520, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x40]));
        assert_eq!(log[2], (CONTROLLER_CONSTRUCTOR, vec![made]));
    }
}

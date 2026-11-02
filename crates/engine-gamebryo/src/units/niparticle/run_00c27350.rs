//! `NiParticle/run_00c27350` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiPSysFieldModifier` constructor (`00c31ff0`, `thiscall`, no words).
const FIELD_CONSTRUCTOR: u32 = 0x00c3_1ff0;
/// The `NiPSysTurbulenceFieldModifier` vtable (RTTI
/// `.?AVNiPSysTurbulenceFieldModifier@@`).
const VTABLE: u32 = 0x010b_e774;
/// The floats the creation function copies: `01015f5c` (the bits
/// 0xFF7FFFFF, -FLT_MAX) and `01022958` (10000.0).
const LAST_UPDATE_INIT: u32 = 0x0101_5f5c;
const UPDATE_TIME_INIT: u32 = 0x0102_2958;

// Translated from 00c27350 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysTurbulenceFieldModifier` creation function (Ghidra made no
/// function of it; `cdecl`, no words): 0x3C bytes from the Ni allocator,
/// constructed as a field modifier (`00c31ff0`), then `m_fFrequency`
/// (+0x30) 0.0 (`FLDZ`), the vtable, `m_fLastUpdateTime` (+0x38) from
/// `01015f5c` and `m_fFrequencyUpdateTime` (+0x34) from `01022958`
/// (`FLD`/`FSTP`, Xbox PDB names); null when the allocation fails.
pub fn fn_00c27350(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(FIELD_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block + 0x30, 0.0f32.to_bits());
    e.mem.set_u32(block, VTABLE);
    let last = x87_quiet_bits(e.global::<u32>(LAST_UPDATE_INIT));
    e.mem.set_u32(block + 0x38, last);
    let update = x87_quiet_bits(e.global::<u32>(UPDATE_TIME_INIT));
    e.mem.set_u32(block + 0x34, update);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_7350, fn_00c27350() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c27350_constructs_a_turbulence_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(FIELD_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_7350, &args![]).u32(), 0);
        e.map(LAST_UPDATE_INIT, 4);
        e.set_global(LAST_UPDATE_INIT, 0xff7f_ffffu32);
        e.map(UPDATE_TIME_INIT, 4);
        e.set_global(UPDATE_TIME_INIT, 10000.0f32);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FIELD_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x5555_5555);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_7350, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(e.mem.u32(made + 0x30), 0);
        assert_eq!(e.mem.f32(made + 0x34), 10000.0);
        assert_eq!(e.mem.u32(made + 0x38), 0xff7f_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x3c]));
    }
}

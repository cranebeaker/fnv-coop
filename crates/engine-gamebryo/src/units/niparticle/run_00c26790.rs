//! `NiParticle/run_00c26790` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiPSysFieldModifier` constructor (`00c31ff0`, `thiscall`, no words).
const FIELD_CONSTRUCTOR: u32 = 0x00c3_1ff0;
/// The `NiPSysVortexFieldModifier` vtable (RTTI
/// `.?AVNiPSysVortexFieldModifier@@`).
const VTABLE: u32 = 0x010b_e604;
/// `NiPoint3::ZERO` (`011f426c`, three floats).
const POINT_ZERO: u32 = 0x011f_426c;

// Translated from 00c26790 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysVortexFieldModifier` creation function (Ghidra made no
/// function of it; `cdecl`, no words): 0x3C bytes from the Ni allocator,
/// constructed as a field modifier (`00c31ff0`), given the vortex vtable and
/// the direction (+0x30) `NiPoint3::ZERO` (copied by `MOV`); null when the
/// allocation fails.
pub fn fn_00c26790(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(FIELD_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, VTABLE);
    for i in 0..3 {
        let value = e.global::<u32>(POINT_ZERO + 4 * i);
        e.mem.set_u32(block + 0x30 + 4 * i, value);
    }
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_6790, fn_00c26790() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c26790_constructs_a_vortex_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(FIELD_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_6790, &args![]).u32(), 0);
        e.map(POINT_ZERO, 12);
        e.set_global(POINT_ZERO + 8, 0x8000_0000u32);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FIELD_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_6790, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(e.mem.u32(made + 0x38), 0x8000_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x3c]));
        assert_eq!(log[2], (FIELD_CONSTRUCTOR, vec![made]));
    }
}

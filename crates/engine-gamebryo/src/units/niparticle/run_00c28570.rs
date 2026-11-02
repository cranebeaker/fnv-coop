//! `NiParticle/run_00c28570` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiPSysFieldModifier` constructor (`00c31ff0`, `thiscall`, no words).
const FIELD_CONSTRUCTOR: u32 = 0x00c3_1ff0;
/// The `NiPSysRadialFieldModifier` vtable (RTTI
/// `.?AVNiPSysRadialFieldModifier@@`).
const VTABLE: u32 = 0x010b_e99c;

// Translated from 00c28570 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysRadialFieldModifier` creation function (Ghidra made no
/// function of it; `cdecl`, no words): 0x34 bytes from the Ni allocator,
/// constructed as a field modifier (`00c31ff0`), the float at +0x30 0.0
/// (`FLDZ`) and the vtable; null when the allocation fails.
pub fn fn_00c28570(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(FIELD_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block + 0x30, 0.0f32.to_bits());
    e.mem.set_u32(block, VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_8570, fn_00c28570() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c28570_constructs_a_radial_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(FIELD_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_8570, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FIELD_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x5555_5555);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_8570, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(e.mem.u32(made + 0x30), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x34]));
    }
}

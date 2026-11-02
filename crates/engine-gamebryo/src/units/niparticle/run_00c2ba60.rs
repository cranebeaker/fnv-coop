//! `NiParticle/run_00c2ba60` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a5d3a0` (unnamed, `NiMain/niobject.cpp`, `thiscall`, no words): the
/// base constructor the creation function runs.
const BASE_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `NiPSysEmitterCtlrData` vtable (RTTI `.?AVNiPSysEmitterCtlrData@@`).
const VTABLE: u32 = 0x010b_fd54;

// Translated from 00c2ba60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitterCtlrData` creation function (Ghidra made no function
/// of it; `cdecl`, no words): 0x28 bytes from the Ni allocator, constructed
/// by `00a5d3a0`, the words at +8, +0xC, +0x10, +0x18, +0x1C and +0x20
/// cleared and the vtable set; null when the allocation fails.
pub fn fn_00c2ba60(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x28u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCTOR, &args![block]);
    for off in [8, 0xc, 0x10, 0x18, 0x1c, 0x20] {
        e.mem.set_u32(block + off, 0);
    }
    e.mem.set_u32(block, VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_ba60, fn_00c2ba60() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c2ba60_constructs_emitter_controller_data_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(BASE_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_ba60, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR, |e, a| {
            for off in (0..0x28).step_by(4) {
                e.mem.set_u32(a[0] + off, 0xffff_ffff);
            }
            Ret::default()
        });
        let made = e.call(0x00c2_ba60, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        for off in [8, 0xc, 0x10, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(made + off), 0);
        }
        assert_eq!(e.mem.u32(made + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(made + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(made + 0x24), 0xffff_ffff);
    }
}

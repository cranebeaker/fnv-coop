//! `NiParticle/run_00c25850` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00c1b5c0` (unnamed, `NiParticle/niparticlesystem.cpp`, `thiscall`, no
/// words): the particle-system constructor the factory runs first.
const PARTICLE_SYSTEM_CONSTRUCTOR: u32 = 0x00c1_b5c0;
/// The `BSStripParticleSystem` vtable (RTTI `.?AVBSStripParticleSystem@@`).
const STRIP_PARTICLE_SYSTEM_VTABLE: u32 = 0x010b_e35c;
/// `00a5d3a0` (unnamed, `NiMain/niobject.cpp`, `thiscall`, no words): the
/// `NiTask` part of the constructor below.
const TASK_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `NiPSysUpdateTask` vtable (RTTI `.?AVNiPSysUpdateTask@@`).
const UPDATE_TASK_VTABLE: u32 = 0x010b_e45c;

// Translated from 00c25820 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `BSStripParticleSystem` creation function (Ghidra made no function
/// of it; `cdecl`, no words): 0x110 bytes from the Ni allocator, constructed
/// by `00c1b5c0` and given the `BSStripParticleSystem` vtable; null when
/// the allocation fails.
pub fn fn_00c25820(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x110u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(PARTICLE_SYSTEM_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, STRIP_PARTICLE_SYSTEM_VTABLE);
    Ptr::new(block)
}

// Translated from 00c25850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysUpdateTask` constructor
/// (`thiscall`, no words): the `NiTask` part (`00a5d3a0`), the word at +8
/// and the system pointer at +0xC cleared, the vtable set and the float at
/// +0x10 set to 0.0 (`FLDZ`). Answers `this`.
pub fn fn_00c25850(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TASK_CONSTRUCTOR, &args![this]);
    let t = this.addr();
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t, UPDATE_TASK_VTABLE);
    e.mem.set_u32(t + 0xc, 0);
    e.mem.set_u32(t + 0x10, 0.0f32.to_bits());
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_5820, fn_00c25820() -> Ptr),
        entry!(0x00c2_5850, fn_00c25850(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c25820_constructs_a_strip_particle_system_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(PARTICLE_SYSTEM_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_5820, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PARTICLE_SYSTEM_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_5820, &args![]).u32();
        assert_eq!(e.mem.u32(made), STRIP_PARTICLE_SYSTEM_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![0x110]));
        assert_eq!(log[2], (PARTICLE_SYSTEM_CONSTRUCTOR, vec![made]));
    }

    #[test]
    fn fn_00c25850_constructs_the_task_with_no_system() {
        let mut e = Engine::new();
        e.register(TASK_CONSTRUCTOR, |e, a| {
            for off in (0..0x14).step_by(4) {
                e.mem.set_u32(a[0] + off, 0xffff_ffff);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00c2_5850, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), UPDATE_TASK_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        for off in [8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
    }
}

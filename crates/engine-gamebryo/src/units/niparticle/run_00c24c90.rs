//! `NiParticle/run_00c24c90` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a963f0` (unnamed, `thiscall`, no words): the base part of the
/// constructor below.
const BASE_CONSTRUCTOR: u32 = 0x00a9_63f0;
/// The `NiPSysData` vtable (RTTI `.?AVNiPSysData@@`).
const PSYS_DATA_VTABLE: u32 = 0x010b_e254;

// Translated from 00c24c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysData` default constructor (it
/// installs that class's vtable; `thiscall`, no words). The base part
/// (`00a963f0`), then the words at +0x70 and +0x74 and the 16-bit counts at
/// +0x78 and +0x7A cleared and the vtable set. Answers `this`.
pub fn fn_00c24c90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    let t = this.addr();
    e.mem.set_u32(t + 0x70, 0);
    e.mem.set_u32(t + 0x74, 0);
    e.mem.set_u16(t + 0x78, 0);
    e.mem.set_u32(t, PSYS_DATA_VTABLE);
    e.mem.set_u16(t + 0x7a, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_4c90, fn_00c24c90(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c24c90_constructs_the_base_then_clears_its_fields() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            for off in (0..0x7c).step_by(4) {
                e.mem.set_u32(a[0] + off, 0xffff_ffff);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x7c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_4c90, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PSYS_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCTOR, vec![this]));
    }
}

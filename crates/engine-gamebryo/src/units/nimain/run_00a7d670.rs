//! `NiMain/run_00a7d670` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiLight` constructor (`00a71f60`, thiscall).
const NI_LIGHT_CONSTRUCT: u32 = 0x00a7_1f60;
/// `NiPointLight`'s vtable (RTTI `.?AVNiPointLight@@`).
const NI_POINT_LIGHT_VTABLE: u32 = 0x0109_dd0c;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiPointLight`'s size on PC.
const NI_POINT_LIGHT_SIZE: u32 = 0xfc;

// Translated from 00a7d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointLight` constructor (the engine map gives it no name): the
/// `NiLight` base (`00a71f60`), then the attenuations `m_fAtten0` (+0xF0)
/// 0.0, `m_fAtten1` (+0xF4) 1.0 and `m_fAtten2` (+0xF8) 0.0 (Xbox PDB
/// names, 0x30 lower on PC), the class vtable and 2 in the byte at +0x9D.
/// Returns `this`.
pub fn fn_00a7d670(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(NI_LIGHT_CONSTRUCT, &args![this]);
    e.mem.set_f32(t + 0xf0, 0.0);
    e.mem.set_u32(t, NI_POINT_LIGHT_VTABLE);
    e.mem.set_u8(t + 0x9d, 2);
    e.mem.set_f32(t + 0xf4, 1.0);
    e.mem.set_f32(t + 0xf8, 0.0);
    this
}

// Translated from 00a7d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointLight`'s create function (no function in Ghidra; read from the
/// bytes): allocates 0xFC bytes and runs the constructor's body on them
/// (inlined in the exe), or returns null when the allocation fails.
pub fn fn_00a7d6e0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_POINT_LIGHT_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00a7d670(e, Ptr::new(block))
}

/// `00a71ec0` (unnamed in the engine map, `thiscall`, clone and cloning process): the
/// light base's member copy.
const LIGHT_COPY_MEMBERS: u32 = 0x00a7_1ec0;

// Translated from 00a7d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointLight` member copy (the engine map gives it no name; `thiscall`, clone and
/// cloning process, `RET 8`): the light base's copy (`00a71ec0`), then the three
/// attenuations (+0xF0, +0xF4, +0xF8) copied into the clone with `FLD`/`FSTP`, which quiets
/// a signalling NaN.
pub fn fn_00a7d6a0(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(LIGHT_COPY_MEMBERS, &args![this, clone, cloning_process]);
    for offset in [0xf0u32, 0xf4, 0xf8] {
        let value = x87_quiet_bits(e.mem.u32(this.addr() + offset));
        e.mem.set_u32(clone.addr() + offset, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_d670, fn_00a7d670(Ptr) -> Ptr),
        entry!(0x00a7_d6e0, fn_00a7d6e0() -> Ptr),
        entry!(0x00a7_d6a0, fn_00a7d6a0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a7d670_constructs_a_point_light() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xfc);
        e.mem.write(this, &[0xaa; 0xfc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_d670, &args![this]).u32(), this);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (NI_LIGHT_CONSTRUCT, vec![this])
        );
        assert_eq!(e.mem.u32(this), NI_POINT_LIGHT_VTABLE);
        assert_eq!(e.mem.u8(this + 0x9d), 2);
        assert_eq!(e.mem.u32(this + 0xf0), 0);
        assert_eq!(e.mem.f32(this + 0xf4), 1.0);
        assert_eq!(e.mem.u32(this + 0xf8), 0);
    }

    #[test]
    fn fn_00a7d6e0_allocates_and_constructs_or_returns_null() {
        let mut e = Engine::new();
        e.register(NI_LIGHT_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_d6e0, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let light = e.call(0x00a7_d6e0, &[]).u32();
        assert_eq!(e.call_log.take().unwrap()[1], (NI_ALLOCATE, vec![0xfc]));
        assert_eq!(e.mem.u32(light), NI_POINT_LIGHT_VTABLE);
        assert_eq!(e.mem.f32(light + 0xf4), 1.0);
    }
}

#[cfg(test)]
mod copy_members_tests {
    use super::*;

    #[test]
    fn copies_the_base_then_the_attenuations() {
        let mut e = Engine::new();
        e.register(LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0xfc);
        let clone = e.mem.alloc(0xfc);
        e.mem.set_f32(this + 0xf0, 0.5);
        e.mem.set_f32(this + 0xf4, 2.0);
        e.mem.set_u32(this + 0xf8, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00a7_d6a0, &args![this, clone, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(LIGHT_COPY_MEMBERS, vec![this, clone, 0x77])));
        assert_eq!(e.mem.f32(clone + 0xf0), 0.5);
        assert_eq!(e.mem.f32(clone + 0xf4), 2.0);
        assert_eq!(e.mem.u32(clone + 0xf8), 0x7fc0_0001);
    }
}

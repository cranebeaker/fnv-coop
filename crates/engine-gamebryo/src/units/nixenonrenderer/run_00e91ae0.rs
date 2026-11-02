//! `NiXenonRenderer/run_00e91ae0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::ALLOCATE;

/// `00e93e70` (`cdecl`, two words; `NiD3DShaderProgramFactory::RegisterShaderCreator`,
/// Xbox PDB): registers a creator object under a name.
const REGISTER_SHADER_CREATOR: u32 = 0x00e9_3e70;

// Translated from 00e91af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): builds an 8-byte object (allocation
/// `00aa13e0`, vtable `010f0a90`, word `+4` set to 0), keeps it in `0126fa3c` (null when the
/// allocation fails) and registers it under the name at `010f0054`.
pub fn fn_00e91af0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block != 0 {
        e.mem.set_u32(block, 0x010f_0a90);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(0x0126_fa3c, block);
        e.call(REGISTER_SHADER_CREATOR, &args![0x010f_0054u32, block]);
    } else {
        e.mem.set_u32(0x0126_fa3c, 0);
        e.call(REGISTER_SHADER_CREATOR, &args![0x010f_0054u32, 0u32]);
    }
}

// Translated from 00e91b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): builds a 0x10-byte object (allocation
/// `00aa13e0`, vtable `010f0ac0`, the words at `+4`, `+8`, `+0xc` set to 0), keeps it in
/// `0126fa50` (null when the allocation fails) and registers it twice, under the names at
/// `010f0aec` and `0103a464`.
pub fn fn_00e91b40(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block != 0 {
        e.mem.set_u32(block, 0x010f_0ac0);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 0xc, 0);
    }
    e.mem.set_u32(0x0126_fa50, block);
    e.call(REGISTER_SHADER_CREATOR, &args![0x010f_0aecu32, block]);
    let kept = e.mem.u32(0x0126_fa50);
    e.call(REGISTER_SHADER_CREATOR, &args![0x0103_a464u32, kept]);
}

// Translated from 00e91b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): builds a 0x10-byte object (allocation
/// `00aa13e0`, the words at `+4`, `+8`, `+0xc` set to 0, vtable `010f0af4`), keeps it in
/// `0126fa54` (null when the allocation fails) and registers it twice, under the names at
/// `010bc868` and `010bc8a0`.
pub fn fn_00e91b90(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block != 0 {
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block, 0x010f_0af4);
    }
    e.mem.set_u32(0x0126_fa54, block);
    e.call(REGISTER_SHADER_CREATOR, &args![0x010b_c868u32, block]);
    let kept = e.mem.u32(0x0126_fa54);
    e.call(REGISTER_SHADER_CREATOR, &args![0x010b_c8a0u32, kept]);
}

// Translated from 00e91ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e91ae0`), `thiscall` with one word: stores it at +4.
pub fn fn_00e91ae0(e: &mut Engine, this: Ptr, word: u32) {
    e.mem.set_u32(this.addr().wrapping_add(4), word);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_1ae0, fn_00e91ae0(Ptr, u32)),
        entry!(0x00e9_1af0, fn_00e91af0()),
        entry!(0x00e9_1b40, fn_00e91b40()),
        entry!(0x00e9_1b90, fn_00e91b90()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x2000);
        e.register(REGISTER_SHADER_CREATOR, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn single_registration() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00e9_1af0, &args![]);
        let object = e.mem.u32(0x0126_fa3c);
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), 0x010f_0a90);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(
            calls_to(&e, REGISTER_SHADER_CREATOR),
            vec![vec![0x010f_0054, object]]
        );
    }

    #[test]
    fn double_registrations() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00e9_1b40, &args![]);
        e.call(0x00e9_1b90, &args![]);
        let first = e.mem.u32(0x0126_fa50);
        let second = e.mem.u32(0x0126_fa54);
        assert_eq!(e.mem.u32(first), 0x010f_0ac0);
        assert_eq!(e.mem.u32(second), 0x010f_0af4);
        assert_eq!(
            calls_to(&e, REGISTER_SHADER_CREATOR),
            vec![
                vec![0x010f_0aec, first],
                vec![0x0103_a464, first],
                vec![0x010b_c868, second],
                vec![0x010b_c8a0, second],
            ]
        );
    }

    #[test]
    fn failed_allocations_register_null() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_1af0, &args![]);
        e.call(0x00e9_1b40, &args![]);
        e.call(0x00e9_1b90, &args![]);
        assert_eq!(e.mem.u32(0x0126_fa3c), 0);
        let calls = calls_to(&e, REGISTER_SHADER_CREATOR);
        assert_eq!(calls.len(), 5);
        assert!(calls.iter().all(|c| c[1] == 0));
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    #[test]
    fn fn_00e91ae0_stores_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x00e9_1ae0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this + 4), 0x1234_5678);
        assert_eq!(e.mem.u32(this), 0);
    }
}

//! `Havok SDK/hkpnullagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d107a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpNullAgent::getNullAgent` (Xbox PDB): takes no arguments and returns the constant 0x0126_87fc.
pub fn hkp_null_agent_get_null_agent(_e: &mut Engine) -> u32 {
    0x0126_87fc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d107a0, hkp_null_agent_get_null_agent() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_null_agent_get_null_agent_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d1_07a0, &args![]).u32(), 0x0126_87fc);
    }
}

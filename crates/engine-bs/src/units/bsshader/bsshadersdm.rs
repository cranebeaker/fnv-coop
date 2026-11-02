//! `BSShader/bsshadersdm.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `bool` `BSShaderSDM::InitSDM` sets once it has run (`01201f70`).
const SDM_INITIALIZED: u32 = 0x0120_1f70;
/// `BSFadeNode::InitSDM` (`00b4e270`, no arguments).
const FADE_NODE_INIT_SDM: u32 = 0x00b4_e270;
/// `BSShaderUtil::InitSDM` (`00b6ba70`, no arguments).
const SHADER_UTIL_INIT_SDM: u32 = 0x00b6_ba70;

// Translated from 00bc2940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderSDM::InitSDM` (Xbox PDB): no arguments. The first time (the byte at `01201f70`
/// clear) runs `BSFadeNode::InitSDM` and `BSShaderUtil::InitSDM`, then sets the byte.
pub fn bs_shader_sdm_init_sdm(e: &mut Engine) {
    if e.global::<u8>(SDM_INITIALIZED) == 0 {
        e.call(FADE_NODE_INIT_SDM, &args![]);
        e.call(SHADER_UTIL_INIT_SDM, &args![]);
        e.set_global(SDM_INITIALIZED, 1u8);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2940, bs_shader_sdm_init_sdm())]
}

#[cfg(test)]
mod b048_tests {
    use super::*;

    #[test]
    fn init_sdm_runs_both_initializers_once() {
        let mut e = Engine::new();
        e.map(SDM_INITIALIZED, 1);
        e.register(FADE_NODE_INIT_SDM, |_, _| Ret::default());
        e.register(SHADER_UTIL_INIT_SDM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00bc_2940, &args![]);
        e.call(0x00bc_2940, &args![]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|&a| a == FADE_NODE_INIT_SDM || a == SHADER_UTIL_INIT_SDM)
            .collect();
        assert_eq!(order, vec![FADE_NODE_INIT_SDM, SHADER_UTIL_INIT_SDM]);
        assert_eq!(e.global::<u8>(SDM_INITIALIZED), 1);
    }
}

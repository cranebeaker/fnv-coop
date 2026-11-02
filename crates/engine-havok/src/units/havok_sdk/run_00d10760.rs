//! `Havok SDK/run_00d10760` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d10760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default agent's function word at offset `+8` of `m_agent2Func[0]` (the collision dispatcher's initialisation stores it, `00cfb115`; see `hkpcollisiondispatcher`): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00d10760(_e: &mut Engine) {}

// Translated from 00d10770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default agent's function word at offset `+4` of `m_agent2Func[0]` (stored by the initialisation at `00cfb10b`): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00d10770(_e: &mut Engine) {}

// Translated from 00d10780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default agent's function word at offset `+0xc` of `m_agent2Func[0]` (stored by the initialisation at `00cfb11f`): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00d10780(_e: &mut Engine) {}

// Translated from 00d10790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default agent's function word at offset `+0` of `m_agent2Func[0]` (stored by the initialisation at `00cfb101`): reads no argument and answers the constant 0x0126_87fc (an address in the data section; what it points to is not read here).
pub fn return_constant_00d10790(_e: &mut Engine) -> u32 {
    0x0126_87fc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d10760, fn_00d10760()),
        entry!(0x00d10770, fn_00d10770()),
        entry!(0x00d10780, fn_00d10780()),
        entry!(0x00d10790, return_constant_00d10790() -> u32),
    ]
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    #[test]
    fn the_default_agent_functions_only_return() {
        let mut e = Engine::new();
        for address in [0x00d1_0760u32, 0x00d1_0770, 0x00d1_0780] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![]).u32(), 0);
            assert_eq!(e.call_log.take().unwrap().len(), 1);
        }
    }

    #[test]
    fn return_constant_00d10790_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d1_0790, &args![]).u32(), 0x0126_87fc);
    }
}

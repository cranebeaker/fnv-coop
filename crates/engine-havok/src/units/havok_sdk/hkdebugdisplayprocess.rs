//! `Havok SDK/hkdebugdisplayprocess.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The process id `registerProcess` stores (`01269cdc`), the process factory object
/// (`01269cd0`), `hkProcessFactory::registerProcess` (`00dd9380`, `this` the factory, the
/// name and the creation function) and its two arguments: the string `"Debug Display"` at
/// `010cae0c` and the creation function at `00ddcb90`.
const PROCESS_ID: u32 = 0x0126_9cdc;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_NAME: u32 = 0x010c_ae0c;
const PROCESS_CREATE: u32 = 0x00dd_cb90;

// Translated from 00ddcbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplayProcess::registerProcess` (engine map): registers the process `"Debug
/// Display"` with the process factory at `01269cd0` (creation function `00ddcb90`) and stores
/// its id at `01269cdc`.
pub fn hk_debug_display_process_register_process(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATE],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dd_cbc0,
        hk_debug_display_process_register_process()
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_process_registers_the_name_and_stores_the_id() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.register_double(REGISTER_PROCESS, |_, _| Ret {
            eax: 0x4321,
            ..Ret::default()
        });
        e.set_global(PROCESS_FACTORY, 0x5555u32);
        e.call_log = Some(Vec::new());
        e.call(0x00dd_cbc0, &args![]);
        let registered: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == REGISTER_PROCESS)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(registered, vec![vec![0x5555, 0x010c_ae0c, 0x00dd_cb90]]);
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4321);
    }
}

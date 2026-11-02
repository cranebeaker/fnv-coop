//! `Havok SDK/hkprocessregisterutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkDebugDisplayProcess::registerProcess`, `hkStatisticsProcess::registerProcess` and
/// `hkInspectProcess::registerProcess`, in the order they are called.
const REGISTER_DEBUG_DISPLAY: u32 = 0x00dd_cbc0;
const REGISTER_STATISTICS: u32 = 0x00dd_c1a0;
const REGISTER_INSPECT: u32 = 0x00dd_b750;

// Translated from 00d61420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcessRegisterUtil::registerAllCommonProcesses` (Xbox PDB): registers the debug display,
/// statistics and inspect processes. The last registration is a tail jump, so the caller sees
/// what it leaves in EAX, which is answered here.
pub fn hk_process_register_util_register_all_common_processes(e: &mut Engine) -> u32 {
    e.call(REGISTER_DEBUG_DISPLAY, &args![]);
    e.call(REGISTER_STATISTICS, &args![]);
    e.call(REGISTER_INSPECT, &args![]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d6_1420,
        hk_process_register_util_register_all_common_processes() -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{engine, log_double, new_log};

    #[test]
    fn the_three_processes_are_registered_in_order_00d61420() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, REGISTER_DEBUG_DISPLAY, 1);
        log_double(&mut e, &log, REGISTER_STATISTICS, 2);
        log_double(&mut e, &log, REGISTER_INSPECT, 3);
        assert_eq!(e.call(0x00d6_1420, &args![]).u32(), 3);
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                REGISTER_DEBUG_DISPLAY,
                REGISTER_STATISTICS,
                REGISTER_INSPECT
            ]
        );
    }
}

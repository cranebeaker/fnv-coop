//! `NiXenonRenderer/run_00e94d70` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The group singleton (`0126fa34`), the Ni allocation `00aa13e0` (`cdecl`, size), the base
/// constructor `00e986f0` (unnamed; `thiscall`, no stack words) and the table the group gets
/// (`010f1008`).
const GROUP_0126FA34: u32 = 0x0126_fa34;
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const BASE_CONSTRUCT_00E986F0: u32 = 0x00e9_86f0;
const GROUP_VTABLE_010F1008: u32 = 0x010f_1008;

// Translated from 00e94d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e94d70`): the geometry group `CreateGroup` makes for type
/// 1, no arguments. Answers the singleton at `0126fa34`; when there is none, makes it (0xc
/// bytes, the base constructor `00e986f0`, the table `010f1008`), stores and answers it (a
/// failed allocation stores and answers 0).
pub fn fn_00e94d70(e: &mut Engine) -> u32 {
    let group = e.mem.u32(GROUP_0126FA34);
    if group != 0 {
        return group;
    }
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0xcu32]).u32();
    if block == 0 {
        e.mem.set_u32(GROUP_0126FA34, 0);
        return 0;
    }
    e.call(BASE_CONSTRUCT_00E986F0, &args![block]);
    e.mem.set_u32(block, GROUP_VTABLE_010F1008);
    e.mem.set_u32(GROUP_0126FA34, block);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_4d70, fn_00e94d70() -> u32)]
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    #[test]
    fn fn_00e94d70_makes_the_singleton_once() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00E986F0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let group = e.call(0x00e9_4d70, &args![]).u32();
        assert_ne!(group, 0);
        assert_eq!(e.mem.u32(GROUP_0126FA34), group);
        assert_eq!(e.mem.u32(group), GROUP_VTABLE_010F1008);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT_00E986F0, vec![group])));
        // Already made: answered as is, nothing allocated.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_4d70, &args![]).u32(), group);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // A failed allocation leaves 0.
        e.mem.set_u32(GROUP_0126FA34, 0);
        e.register(NI_ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e9_4d70, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(GROUP_0126FA34), 0);
    }
}

//! `(unplaced)/run_00682320` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The parameter count of a script command (`00682270`, cdecl, the
/// compiled command; 0 for null): the `u16` at +0x22 of its 0x28-byte
/// record.
const COMMAND_PARAMETER_COUNT: u32 = 0x0068_2270;
/// The parameter-table pointer of command record 0 (+0x24 of the 0x28-byte
/// records from `01190900` on, indexed by the command's first `u16`).
const COMMAND_PARAMETERS: u32 = 0x0119_0924;
/// The size of one command record.
const COMMAND_RECORD_SIZE: u32 = 0x28;
/// The size of one parameter entry (its type word at +4).
const PARAMETER_SIZE: u32 = 0xc;

// Translated from 00682320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. cdecl: whether parameter `index` of the command
/// `command` exists (below `00682270`'s count, unsigned) and has type 4 or
/// 6 (the word at +4 of its 12-byte entry in the command's parameter table).
/// False for a null command.
pub fn fn_00682320(e: &mut Engine, command: Ptr, index: u32) -> bool {
    if command.is_null() {
        return false;
    }
    let count = e.call(COMMAND_PARAMETER_COUNT, &args![command]).u32();
    if index >= count {
        return false;
    }
    let opcode = u32::from(e.mem.u16(command.addr()));
    let table = e
        .mem
        .u32(COMMAND_PARAMETERS.wrapping_add(opcode.wrapping_mul(COMMAND_RECORD_SIZE)));
    let kind = e.mem.u32(
        table
            .wrapping_add(index.wrapping_mul(PARAMETER_SIZE))
            .wrapping_add(4),
    );
    kind == 4 || kind == 6
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00682320, fn_00682320(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_type_4_or_6_00682320() {
        let mut e = Engine::new();
        e.register(COMMAND_PARAMETER_COUNT, |_, _| 3u32.into_ret());
        e.map(0x0119_0900, 0x28 * 3);
        let parameters = e.mem.alloc(0x24);
        for (i, kind) in [4u32, 5, 6].iter().enumerate() {
            e.mem.set_u32(parameters + 0xc * i as u32 + 4, *kind);
        }
        e.mem.set_u32(COMMAND_PARAMETERS + 0x28 * 2, parameters);
        let command = e.mem.alloc(4);
        e.mem.set_u16(command, 2);
        assert!(e.call(0x0068_2320, &args![command, 0u32]).bool());
        assert!(!e.call(0x0068_2320, &args![command, 1u32]).bool());
        assert!(e.call(0x0068_2320, &args![command, 2u32]).bool());
        assert!(!e.call(0x0068_2320, &args![command, 3u32]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_2320, &args![0u32, 0u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

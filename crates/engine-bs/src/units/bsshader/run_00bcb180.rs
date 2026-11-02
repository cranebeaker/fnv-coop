//! `BSShader/run_00bcb180` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two slots of the `GeometryDecalShader` vtable `010bb588` (`+0xf0` and `+0xfc`), each taking
//! a pass number; the decal shader's two passes are numbers 0x24c and 0x24d.

#[allow(unused_imports)]
use crate::prelude::*;

/// The current-pass word (`0126f74c`) the pass selection sets.
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
/// The two global declaration `NiPointer`s of the decal shader (`012021e0` for pass 0x24c,
/// `012021e4` for 0x24d).
const DECLARATION_POINTER_012021E0: u32 = 0x0120_21e0;
const DECLARATION_POINTER_012021E4: u32 = 0x0120_21e4;
/// `00a5f860` (unnamed, `thiscall` on an `NiPointer`, one stack word: the object it assigns).
const POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
/// The first of the decal shader's pass numbers.
const FIRST_PASS: u32 = 0x24c;
/// The second.
const SECOND_PASS: u32 = 0x24d;

// Translated from 00bcb180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader` vtable slot `+0xf0` (Ghidra made no function of it; `thiscall`, one
/// stack word: the pass number, `RET 4`). For pass 0x24c: the pass `+0x6c` becomes current
/// (`0126f74c`) and the declaration `012021e0` is assigned to the shader's `NiPointer` at
/// `+0x28` (`00a5f860`, reached by a tail jump); for 0x24d the same with `+0x70` and
/// `012021e4`; any other number does nothing.
pub fn fn_00bcb180(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let (pass_offset, declaration) = match pass.wrapping_sub(FIRST_PASS) {
        0 => (0x6c, DECLARATION_POINTER_012021E0),
        1 => (0x70, DECLARATION_POINTER_012021E4),
        _ => return,
    };
    let current = e.mem.u32(base + pass_offset);
    let declaration = e.global::<u32>(declaration);
    e.set_global(CURRENT_PASS_0126F74C, current);
    e.call(POINTER_ASSIGN_00A5F860, &args![base + 0x28, declaration]);
}

// Translated from 00bcb1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader` vtable slot `+0xfc` (Ghidra made no function of it; `thiscall`, one
/// stack word: the pass number, `RET 4`). Sets the byte `+8` of the constant entry `+0x84` to
/// 0 for pass 0x24d and to 1 for any other number, and the byte `+8` of the entries `+0x88`
/// and `+0x8c` to the opposite; then calls slot `+0x24` of the object at `+0x34` (no stack
/// words) and passes its `EAX` on.
pub fn fn_00bcb1d0(e: &mut Engine, this: Ptr, pass: u32) -> u32 {
    let base = this.addr();
    let first_entry = e.mem.u32(base + 0x84);
    let second_pass = if pass == SECOND_PASS {
        e.mem.set_u8(first_entry + 8, 0);
        1u8
    } else {
        e.mem.set_u8(first_entry + 8, 1);
        0
    };
    let entry = e.mem.u32(base + 0x88);
    e.mem.set_u8(entry + 8, second_pass);
    let entry = e.mem.u32(base + 0x8c);
    e.mem.set_u8(entry + 8, second_pass);
    let object = e.mem.u32(base + 0x34);
    e.vcall(object, 0x24, &args![]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_b180, fn_00bcb180(Ptr, u32)),
        entry!(0x00bc_b1d0, fn_00bcb1d0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decal_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global(DECLARATION_POINTER_012021E0, 0x5100u32);
        e.set_global(DECLARATION_POINTER_012021E4, 0x5200u32);
        e.register(POINTER_ASSIGN_00A5F860, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00bcb180_selects_the_pass_and_its_declaration() {
        for (pass, current, declaration) in
            [(0x24cu32, 0x6100u32, 0x5100u32), (0x24d, 0x6200, 0x5200)]
        {
            let mut e = decal_engine();
            let shader = e.mem.alloc(0x90);
            e.mem.set_u32(shader + 0x6c, 0x6100);
            e.mem.set_u32(shader + 0x70, 0x6200);
            e.call_log = Some(vec![]);
            e.call(0x00bc_b180, &args![shader, pass]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), current);
            let assigns: Vec<_> = log
                .iter()
                .filter(|c| c.0 == POINTER_ASSIGN_00A5F860)
                .collect();
            assert_eq!(assigns.len(), 1);
            assert_eq!(assigns[0].1, vec![shader + 0x28, declaration]);
        }
        // Any other number does nothing.
        let mut e = decal_engine();
        let shader = e.mem.alloc(0x90);
        e.set_global(CURRENT_PASS_0126F74C, 0x77u32);
        e.call_log = Some(vec![]);
        e.call(0x00bc_b180, &args![shader, 0x24bu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x77);
        assert!(log.iter().all(|c| c.0 != POINTER_ASSIGN_00A5F860));
    }

    #[test]
    fn fn_00bcb1d0_sets_the_entry_flags_and_calls_the_map() {
        for (pass, first, others) in [(0x24du32, 0u8, 1u8), (0x24c, 1, 0), (0, 1, 0)] {
            let mut e = Engine::new();
            let shader = e.mem.alloc(0x90);
            let entries = [e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10)];
            for (i, entry) in entries.iter().enumerate() {
                e.mem.set_u8(entry + 8, 0x55);
                e.mem.set_u32(shader + 0x84 + 4 * i as u32, *entry);
            }
            let map = e.mem.alloc(0x10);
            e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0070_1000]);
            e.mem.set_u32(map, 0x0070_0000);
            e.mem.set_u32(shader + 0x34, map);
            e.register(0x0070_1000, |_, a| (a[0] ^ 0x99).into_ret());
            assert_eq!(e.call(0x00bc_b1d0, &args![shader, pass]).u32(), map ^ 0x99);
            assert_eq!(e.mem.u8(entries[0] + 8), first);
            assert_eq!(e.mem.u8(entries[1] + 8), others);
            assert_eq!(e.mem.u8(entries[2] + 8), others);
        }
    }
}

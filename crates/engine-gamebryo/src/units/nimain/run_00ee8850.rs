//! `NiMain/run_00ee8850` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject`'s constructor (`00a5d3a0`, unit `niobject.cpp`; `thiscall`, no stack words) and
/// `NiShaderDeclaration`'s vtable (`011011ec`, RTTI `.?AVNiShaderDeclaration@@`).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
const SHADER_DECLARATION_VTABLE_011011EC: u32 = 0x0110_11ec;

// Translated from 00ee88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ee8850`): the `NiShaderDeclaration` constructor (the
/// vtable it stores has that class's RTTI), `thiscall`, no stack words: `NiObject`'s
/// constructor, then the vtable `011011ec`. Answers `this`.
pub fn fn_00ee88b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![this]);
    e.mem
        .set_u32(this.addr(), SHADER_DECLARATION_VTABLE_011011EC);
    this
}

/// `NiObject`'s destructor (`00a5d3d0`, unit `niobject.cpp`; `thiscall`, no
/// stack words).
const NI_OBJECT_DESTRUCT_00A5D3D0: u32 = 0x00a5_d3d0;

// Translated from 00ee88d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiShaderDeclaration` destructor body
/// (`thiscall`, no stack words): stores the vtable `011011ec`, then jumps to
/// `NiObject`'s destructor `00a5d3d0` with the same `this`.
pub fn fn_00ee88d0(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), SHADER_DECLARATION_VTABLE_011011EC);
    e.call(NI_OBJECT_DESTRUCT_00A5D3D0, &args![this]);
}

/// `00eea4e0` (`thiscall`, no stack words): the 16-bit word at +0x18 of an entry, in `AX`.
const ENTRY_WORD: u32 = 0x00ee_a4e0;

// Translated from 00ee8850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a kind word; `RET 4`): a total: kind 0 starts from
/// the 16-bit word at +0x8e4 shifted left by 16, kind 2 from the one at +0x8e6, any other
/// kind from 0; then each non-null entry of the array at +0xc (the count at +0x14, read
/// again each pass) adds its 16-bit word (`00eea4e0`).
pub fn fn_00ee8850(e: &mut Engine, this: Ptr, kind: u32) -> u32 {
    let base = this.addr();
    let mut total = match kind {
        0 => u32::from(e.mem.u16(base + 0x8e4)) << 16,
        2 => u32::from(e.mem.u16(base + 0x8e6)) << 16,
        _ => 0,
    };
    let mut i = 0u32;
    while i < e.mem.u32(base + 0x14) {
        let entry = e.mem.u32(e.mem.u32(base + 0xc).wrapping_add(4 * i));
        if entry != 0 {
            let word = e.call(ENTRY_WORD, &args![entry]).u16();
            total = total.wrapping_add(u32::from(word));
        }
        i += 1;
    }
    total
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_88b0, fn_00ee88b0(Ptr) -> Ptr),
        entry!(0x00ee_88d0, fn_00ee88d0(Ptr)),
        entry!(0x00ee_8850, fn_00ee8850(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ee88b0_runs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |e, a| {
            e.mem.set_u32(a[0], 1);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00ee_88b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_DECLARATION_VTABLE_011011EC);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_OBJECT_CONSTRUCT_00A5D3A0, vec![this]));
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn fn_00ee88d0_stores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT_00A5D3D0, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00ee_88d0, &args![this]);
        assert_eq!(e.mem.u32(this), SHADER_DECLARATION_VTABLE_011011EC);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_OBJECT_DESTRUCT_00A5D3D0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this]);
    }
}

#[cfg(test)]
mod total_tests {
    use super::*;

    #[test]
    fn totals_by_kind_and_entries() {
        let mut e = Engine::new();
        e.register(ENTRY_WORD, |e, a| {
            u32::from(e.mem.u16(a[0] + 0x18)).into_ret()
        });
        let this = e.mem.alloc(0x8e8);
        e.mem.set_u16(this + 0x8e4, 2);
        e.mem.set_u16(this + 0x8e6, 3);
        let entries = e.mem.alloc(0xc);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u16(first + 0x18, 0xffff);
        e.mem.set_u16(second + 0x18, 5);
        e.mem.set_u32(entries, first);
        e.mem.set_u32(entries + 8, second);
        e.mem.set_u32(this + 0xc, entries);
        e.mem.set_u32(this + 0x14, 3);
        assert_eq!(
            e.call(0x00ee_8850, &args![this, 0u32]).u32(),
            0x0002_0000 + 0x1_0004
        );
        assert_eq!(
            e.call(0x00ee_8850, &args![this, 2u32]).u32(),
            0x0003_0000 + 0x1_0004
        );
        assert_eq!(e.call(0x00ee_8850, &args![this, 1u32]).u32(), 0x1_0004);
        e.mem.set_u32(this + 0x14, 0);
        assert_eq!(e.call(0x00ee_8850, &args![this, 1u32]).u32(), 0);
    }
}

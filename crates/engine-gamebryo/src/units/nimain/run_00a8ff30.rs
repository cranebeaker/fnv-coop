//! `NiMain/run_00a8ff30` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `011f5e44`: the global array of material libraries (data pointer; count at `011f5e4c`).
const LIBRARY_LIST: u32 = 0x011f_5e44;
/// `00a311e0` (`thiscall`, `RET 4`, engine map `NiTSet<...>::Realloc`): gives the array the
/// capacity given.
const LIST_REALLOC: u32 = 0x00a3_11e0;

// Translated from 00a8ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): empties the global array of material libraries at `011f5e44` (count at
/// `011f5e4c` set to 0) and gives it capacity 0 (`00a311e0`).
pub fn fn_00a8ff30(e: &mut Engine) {
    e.mem.set_u32(LIBRARY_LIST + 8, 0);
    e.call(LIST_REALLOC, &args![LIBRARY_LIST, 0u32]);
}

/// The vtable of `NiScreenGeometryData::ScreenElement` (its RTTI locator names
/// `.?AVScreenElement@NiScreenGeometryData@@`).
const SCREEN_ELEMENT_VTABLE: u32 = 0x0109_f814;

// Translated from 00a8ff50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, no stack
/// words): the `NiScreenGeometryData::ScreenElement` constructor by the vtable it stores:
/// the two 16-bit words at +4 and +6 and the words at +8, +0xc and +0x10 cleared. Answers
/// `this`.
pub fn fn_00a8ff50(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, SCREEN_ELEMENT_VTABLE);
    e.mem.set_u16(base + 4, 0);
    e.mem.set_u16(base + 6, 0);
    for offset in [8u32, 0xc, 0x10] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_ff30, fn_00a8ff30()),
        entry!(0x00a8_ff50, fn_00a8ff50(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_the_list_then_reallocates_it_to_nothing() {
        let mut e = Engine::new();
        e.map(LIBRARY_LIST & !0xfff, 0x1000);
        e.mem.set_u32(LIBRARY_LIST + 8, 3);
        e.register(LIST_REALLOC, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 8), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_ff30, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(LIST_REALLOC, vec![LIBRARY_LIST, 0])]);
    }
}

#[cfg(test)]
mod screen_element_tests {
    use super::*;

    #[test]
    fn constructs_an_empty_element() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xaa; 0x18]);
        assert_eq!(e.call(0x00a8_ff50, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SCREEN_ELEMENT_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(this + 0x14), 0xaaaa_aaaa);
    }
}

//! `BSShader/run_00b71b40` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two tables of objects `fn_00b71b90` walks: 0x9c words at `011fdd88` (slot +0xa0 each,
/// only when the word at `+0x28` is set) and 0xd0 words at `011fda48` (slot +0x88 each).
const OBJECTS_011FDD88: u32 = 0x011f_dd88;
const OBJECTS_011FDD88_BYTES: u32 = 0x270;
const OBJECTS_011FDA48: u32 = 0x011f_da48;
const OBJECTS_011FDA48_BYTES: u32 = 0x340;
/// Where `fn_00b71b90` jumps at the end (`00e814b0`, unit `NiXenonRenderer`; `thiscall`, no
/// stack words).
const TAIL_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00b71b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the address of three words; the lighting
/// shader's clip-plane code `00bbef90` calls it): copies the three words at
/// `+0x1e4`, `+0x1e8` and `+0x1ec` there.
pub fn fn_00b71b40(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    let first = e.mem.u32(base + 0x1e4);
    e.mem.set_u32(out.addr(), first);
    let second = e.mem.u32(base + 0x1e8);
    let third = e.mem.u32(base + 0x1ec);
    e.mem.set_u32(out.addr() + 4, second);
    e.mem.set_u32(out.addr() + 8, third);
}

// Translated from 00b71b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a slot of the vtables at `010af3c8`
/// and `010bc140`, also called by `00bcc740`, `00bce940` and `00bd04a0`). When the word at
/// `+0x28` is set, each non-null object of the 0x9c words at `011fdd88` gets its slot +0xa0;
/// then each non-null object of the 0xd0 words at `011fda48` gets its slot +0x88; then a jump
/// to `00e814b0` with `this`.
pub fn fn_00b71b90(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x28) != 0 {
        let mut offset = 0u32;
        while offset < OBJECTS_011FDD88_BYTES {
            let object = e.mem.u32(OBJECTS_011FDD88 + offset);
            if object != 0 {
                e.vcall(object, 0xa0, &args![]);
            }
            offset += 4;
        }
    }
    let mut offset = 0u32;
    while offset < OBJECTS_011FDA48_BYTES {
        let object = e.mem.u32(OBJECTS_011FDA48 + offset);
        if object != 0 {
            e.vcall(object, 0x88, &args![]);
        }
        offset += 4;
    }
    e.call(TAIL_00E814B0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_1b40, fn_00b71b40(Ptr, Ptr)),
        entry!(0x00b7_1b90, fn_00b71b90(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00b71b40_copies_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1f0);
        for i in 0..3 {
            e.mem.set_u32(this + 0x1e4 + 4 * i, 0x10 + i);
        }
        let out = e.mem.alloc(12);
        e.call(0x00b7_1b40, &args![this, out]);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [0x10, 0x11, 0x12]
        );
    }

    #[test]
    fn fn_00b71b90_walks_both_tables() {
        let mut e = Engine::new();
        e.map(0x011f_d000, 0x1000);
        e.put_vtable(0x0f00_1000, &[0x0f00_0000; 0x29]);
        e.mem.set_u32(0x0f00_1088, 0x0f00_0088);
        e.mem.set_u32(0x0f00_10a0, 0x0f00_00a0);
        e.register(0x0f00_0088, |_, _| Ret::default());
        e.register(0x0f00_00a0, |_, _| Ret::default());
        e.register(TAIL_00E814B0, |_, _| Ret::default());
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0x0f00_1000);
        e.mem.set_u32(b, 0x0f00_1000);
        // First and last slots of each table.
        e.set_global(OBJECTS_011FDD88, a);
        e.set_global(OBJECTS_011FDD88 + OBJECTS_011FDD88_BYTES - 4, b);
        e.set_global(OBJECTS_011FDA48, b);
        e.set_global(OBJECTS_011FDA48 + OBJECTS_011FDA48_BYTES - 4, a);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1b90, &args![this]);
        assert!(calls(&e, 0x0f00_00a0).is_empty());
        assert_eq!(calls(&e, 0x0f00_0088), vec![vec![b], vec![a]]);
        assert_eq!(calls(&e, TAIL_00E814B0), vec![vec![this]]);
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1b90, &args![this]);
        assert_eq!(calls(&e, 0x0f00_00a0), vec![vec![a], vec![b]]);
        assert_eq!(calls(&e, 0x0f00_0088).len(), 2);
    }
}

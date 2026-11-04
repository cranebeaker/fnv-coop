//! `(unplaced)/run_006ff380` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0056c7f0` (`thiscall`, no stack word; unit `fallout shared/tesobjectrefr.cpp`).
const GET_TARGET: u32 = 0x0056_c7f0;
/// `006fb940` (`thiscall`, `RET 0xc`: three floats; unit
/// `fallout shared/distant terrain system/bgsterrainchunk.cpp`).
const SET_THREE_FLOATS: u32 = 0x006f_b940;

// Translated from 006ff380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): clears the byte at +0x54, gets an
/// object from the word at +0x10 (`0056c7f0`) and, when there is one, calls `006fb940` on it
/// with the floats 0.0, 0.0 and 1.0, then its vtable slot +0xf0 twice, with (0, 1, 0) and
/// then (1, 1, 0).
pub fn fn_006ff380(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x54, 0);
    let source = e.mem.u32(this.addr() + 0x10);
    let target = e.call(GET_TARGET, &args![source]).u32();
    if target != 0 {
        e.call(SET_THREE_FLOATS, &args![target, 0.0f32, 0.0f32, 1.0f32]);
        e.vcall(target, 0xf0, &args![0u32, 1u32, 0u32]);
        e.vcall(target, 0xf0, &args![1u32, 1u32, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006f_f380, fn_006ff380(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resets_the_target_when_there_is_one() {
        let mut e = Engine::new();
        e.register(SET_THREE_FLOATS, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x3d]);
        e.mem.set_u32(0x0150_00f0, 0x0150_1000);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x58);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, 0x0150_0000);
        e.mem.set_u32(this + 0x10, 0x6000);
        e.mem.set_u8(this + 0x54, 1);
        e.register(GET_TARGET, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_f380, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(GET_TARGET, vec![0x6000])]);
        assert_eq!(e.mem.u8(this + 0x54), 0);
        e.register_double(GET_TARGET, move |_, _| target.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_f380, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (GET_TARGET, vec![0x6000]),
                (SET_THREE_FLOATS, vec![target, 0, 0, 0x3f80_0000]),
                (0x0150_1000, vec![target, 0, 1, 0]),
                (0x0150_1000, vec![target, 1, 1, 0]),
            ]
        );
    }
}

//! `BSShader/run_00bc5510` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// `00be0870` (unnamed, `thiscall`, no stack words; answers a byte in `AL`).
const SHADER_CALL_00BE0870: u32 = 0x00be_0870;

// Translated from 00bc5450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; reached from the vtable `010baf60`, slot +0x4c),
/// `thiscall`, no stack words. Runs `00be0870` on `this` and keeps its byte, clears the three
/// pass pointers at +0x6c to +0x74 (each pass's count at +0x64 down by one, deleted through
/// `00e80190` at 0) and the four `NiPointer`s at +0x78 to +0x84, and answers the kept byte.
pub fn fn_00bc5450(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    let answer = e.call(SHADER_CALL_00BE0870, &args![base]).u32() as u8;
    for offset in [0x6cu32, 0x70, 0x74] {
        pass_pointer_clear(e, base + offset);
    }
    for offset in [0x78u32, 0x7c, 0x80, 0x84] {
        ni_pointer_clear(e, base + offset);
    }
    answer
}

/// The `bool` setting that enables the third pass (`011ad80e`).
const THIRD_PASS_ENABLED_011AD80E: u32 = 0x011a_d80e;

/// The render states every pass of `00bc5510` gets before the per-pass one (state, value).
const STATES_BEFORE: [(u32, u32); 5] = [(0xa8, 0xf), (7, 0), (0xe, 0), (0x17, 8), (0x1b, 1)];
/// The render states after the per-pass one.
const STATES_AFTER: [(u32, u32); 5] = [(0x34, 1), (0x38, 8), (0x39, 1), (0x37, 3), (0x36, 3)];

// Translated from 00bc5510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc5510`; the vtable `010baf60`, slot +0x148), `thiscall`,
/// no stack words; answers true. For the passes 0 to 2 at +0x6c + 4 * index (pass 2 only when
/// the byte setting `011ad80e` is set): a null pass pointer gets a new pass (`00e804d0`) with one
/// texture stage (`SetupDefaultStage(stage, 0, 3, 2, 1)`, filter mode 0, `SetStage` with the
/// pass's word at +0x14, the stage's reference dropped). Passes 0 and 1 take the pixel shader
/// at +0x80 and the vertex shader at +0x78, pass 2 the pixel shader at +0x84 and the vertex
/// shader at +0x7c (the pass's `NiPointer`s +0x44 and +0x5c). Then the render states (0xa8,
/// 0xf), (7, 0), (0xe, 0), (0x17, 8), (0x1b, 1), (0x13, 2), (0x14, 2) (the last two on the pass
/// read once), (0x16, 2 / 3 / 1 for the passes 0 / 1 / 2, on the pass read through its own
/// offset), (0x34, 1), (0x38, 8), (0x39, 1), (0x37, 3), (0x36, 3).
// Translation incomplete: 00bc5510 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bc5510(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for index in 0..3u32 {
        if index == 2 && e.global::<u8>(THIRD_PASS_ENABLED_011AD80E) == 0 {
            continue;
        }
        let slot = base + 0x6c + 4 * index;
        if e.mem.u32(slot) == 0 {
            e.call(NEW_PASS_00E804D0, &args![slot]);
            add_default_stages(e, slot, 1, [3, 2, 1], 0);
        }
        let shaders = match index {
            0 | 1 => Some((0x80u32, 0x78u32)),
            2 => Some((0x84, 0x7c)),
            _ => None,
        };
        if let Some((pixel, vertex)) = shaders {
            let pass = e.mem.u32(slot);
            let shader = e.mem.u32(base + pixel);
            ni_pointer_assign(e, pass + 0x44, shader);
            let shader = e.mem.u32(base + vertex);
            let pass = e.mem.u32(slot);
            ni_pointer_assign(e, pass + 0x5c, shader);
        }
        for (state, value) in STATES_BEFORE {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
        let pass = e.mem.u32(slot);
        pass_render_state(e, pass, 0x13, 2);
        pass_render_state(e, pass, 0x14, 2);
        let blend = match index {
            1 => Some((0x70u32, 3u32)),
            0 => Some((0x6c, 2)),
            2 => Some((0x74, 1)),
            _ => None,
        };
        if let Some((offset, value)) = blend {
            let pass = e.mem.u32(base + offset);
            pass_render_state(e, pass, 0x16, value);
        }
        for (state, value) in STATES_AFTER {
            let pass = e.mem.u32(slot);
            pass_render_state(e, pass, state, value);
        }
    }
    true
}

/// The record `00bc5440` answers (`01201fd8`).
const RECORD_01201FD8: u32 = 0x0120_1fd8;

// Translated from 00bc5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010baf68`, `ECX` unread):
/// answers the address `01201fd8`.
pub fn fn_00bc5440(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RECORD_01201FD8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_5440, fn_00bc5440(Ptr) -> Ptr),
        entry!(0x00bc_5450, fn_00bc5450(Ptr) -> u8),
        entry!(0x00bc_5510, fn_00bc5510(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bc5450_clears_the_passes_and_shaders_and_answers_the_byte() {
        let mut e = rig();
        e.register(SHADER_CALL_00BE0870, |_, _| 0x1234_5602u32.into_ret());
        let this = e.mem.alloc(0x88);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        let shader = object(&mut e, 1);
        e.mem.set_u32(this + 0x70, pass);
        e.mem.set_u32(this + 0x80, shader);
        assert_eq!(e.call(0x00bc_5450, &args![this]).u32() & 0xff, 2);
        assert_eq!(calls(&e, SHADER_CALL_00BE0870), vec![vec![this]]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shader]]);
        for offset in (0x6cu32..=0x84).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    fn shader(e: &mut Engine) -> (u32, [u32; 4]) {
        let this = e.mem.alloc(0x90);
        let mut shaders = [0u32; 4];
        for (index, offset) in [0x78u32, 0x7c, 0x80, 0x84].iter().enumerate() {
            shaders[index] = object(e, 1);
            e.mem.set_u32(this + offset, shaders[index]);
        }
        (this, shaders)
    }

    #[test]
    fn fn_00bc5510_builds_three_passes_with_the_setting() {
        let mut e = pass_rig();
        e.map(0x011a_d000, 0x1000);
        e.set_global::<u8>(THIRD_PASS_ENABLED_011AD80E, 1);
        let (this, [vertex_a, vertex_b, pixel_a, pixel_b]) = shader(&mut e);
        assert!(e.call(0x00bc_5510, &args![this]).bool());
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 3);
        for (index, blend) in [(0u32, 2u32), (1, 3), (2, 1)] {
            let pass = e.mem.u32(this + 0x6c + 4 * index);
            assert_eq!(stages(&e, pass), vec![(0, [3, 2, 1], 0)]);
            let (vertex, pixel) = if index == 2 {
                (vertex_b, pixel_b)
            } else {
                (vertex_a, pixel_a)
            };
            assert_eq!(e.mem.u32(pass + 0x5c), vertex);
            assert_eq!(e.mem.u32(pass + 0x44), pixel);
            let mut want = STATES_BEFORE.to_vec();
            want.extend([(0x13, 2), (0x14, 2), (0x16, blend)]);
            want.extend(STATES_AFTER);
            assert_eq!(render_states(&e, pass), want);
        }
    }

    #[test]
    fn fn_00bc5510_skips_the_third_pass_and_keeps_existing_ones() {
        let mut e = pass_rig();
        e.map(0x011a_d000, 0x1000);
        let (this, _) = shader(&mut e);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x6c, pass);
        e.call(0x00bc_5510, &args![this]);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x70]]);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(render_states(&e, pass).len(), 13);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;

    #[test]
    fn fn_00bc5440_answers_its_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_5440, &args![0u32]).u32(), RECORD_01201FD8);
    }
}

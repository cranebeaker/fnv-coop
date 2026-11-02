//! `NiXenonRenderer/run_00e7ca20` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two steps of a default-shader variant's pass set-up (unit `nid3ddefaultshader.cpp` holds
//! the callees). The shader's fields are read at their offsets: a flag byte (+0x3c), the
//! highest texture-coordinate index (+0x40) and two words cleared together (+0x44, +0x48).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pass the shader works on (`0126f74c`).
const CURRENT_PASS: u32 = 0x0126_f74c;
/// `NiD3DDefaultShader::AddTextureStage` (`00e7c830`, `thiscall`, two words: the texture and
/// its coordinate index; answers the new stage), `00e7c790` (`thiscall`, four words) and the
/// stage-state setter `00e88fc0` (`thiscall` on a stage's +0x10, three words).
const ADD_TEXTURE_STAGE: u32 = 0x00e7_c830;
const SETUP_00E7C790: u32 = 0x00e7_c790;
const SET_STAGE_STATE: u32 = 0x00e8_8fc0;
/// `00e8eeb0` (unit `nigeometrybufferdata.cpp`, `thiscall`, a `bool`).
const BUFFER_DATA_00E8EEB0: u32 = 0x00e8_eeb0;

/// A texture's coordinate index as both steps clip it: the low byte of its `u16` +4, at most
/// the shader's +0x40 (taken as a `u16` when smaller).
fn coordinate_index(e: &Engine, this: u32, texture: u32) -> u32 {
    let index = u32::from(e.mem.u16(texture + 4) & 0xff);
    if index > e.mem.u32(this + 0x40) {
        u32::from(e.mem.u16(this + 0x40))
    } else {
        index
    }
}

fn set_stage_state(e: &mut Engine, stage: u32, state: u32, value: u32) {
    let states = e.mem.u32(stage + 0x10);
    e.call(SET_STAGE_STATE, &args![states, state, value, 0u32]);
}

// Translated from 00e7ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a texture or null). Nothing without a
/// current pass (`0126f74c`). Otherwise, with the flag byte (+0x3c) set and a texture, adds a
/// stage for it (`AddTextureStage` with its clipped coordinate index) and, when that gives a
/// stage, sets the stage states 1 = 3, then with the flag 4 = 4 and 6 = 0, without it 4 = 2.
/// Then +0x44 and +0x48 are cleared.
pub fn fn_00e7ca20(e: &mut Engine, this: Ptr, texture: Ptr) {
    if e.mem.u32(CURRENT_PASS) == 0 {
        return;
    }
    let base = this.addr();
    let texture = texture.addr();
    if e.mem.u8(base + 0x3c) != 0 && texture != 0 {
        let index = coordinate_index(e, base, texture);
        let stage = e
            .call(ADD_TEXTURE_STAGE, &args![base, texture, index])
            .u32();
        if stage != 0 {
            set_stage_state(e, stage, 1, 3);
            if e.mem.u8(base + 0x3c) != 0 {
                set_stage_state(e, stage, 4, 4);
                set_stage_state(e, stage, 6, 0);
            } else {
                set_stage_state(e, stage, 4, 2);
            }
        }
    }
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
}

// Translated from 00e7cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven stack words of which the first, third and
/// fourth are read: the geometry, a property block and a pass description; answers -1 or 0).
/// -1 without a property block, and, without a geometry, when the block fails `00e8eeb0` or
/// there is no description. Otherwise (a missing block, which cannot happen here, would be
/// the geometry's +0xb8's +0x34) the block's byte +7 gives the coordinate count: when it is
/// not 0, +0x40 becomes the count less one and the texture is the first entry of the array at
/// the description's +0x14's +0x20, used when it and its +8 are not null. The second state
/// (`(u16 +0x18 >> 1) & 7` of the description's +0x14 not zero) and the texture decide: with
/// neither the answer is 0 at once. Else `00e7c790` runs with 1, 3, 0, 0; with the texture its
/// stage is added (clipped index) and gets 1 = 4, 3 = 0, 4 = 4, 6 = 0 with the second state
/// or 1 = 2, 4 = 2 without; without the texture an empty stage is added (`AddTextureStage`
/// with 0, 0) and gets 1 = 2, 4 = 2. Finally `fn_00e7ca20` runs with the texture (or null) and
/// the answer is 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7cac0(
    e: &mut Engine,
    this: Ptr,
    geometry: Ptr,
    _unused_2: u32,
    block: Ptr,
    description: Ptr,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    let base = this.addr();
    let geometry = geometry.addr();
    let mut block = block.addr();
    let description = description.addr();
    if geometry == 0 {
        if block == 0 || !e.call(BUFFER_DATA_00E8EEB0, &args![block]).bool() || description == 0 {
            return 0xffff_ffff;
        }
    } else if block == 0 {
        return 0xffff_ffff;
    }
    let pass = e.mem.u32(description + 0x14);
    let mut texture = 0u32;
    if block == 0 {
        // Unreachable here (a null block returned above), translated as the exe has it.
        let data = e.mem.u32(geometry + 0xb8);
        block = e.mem.u32(data + 0x34);
    }
    let coordinates = u32::from(e.mem.u8(block + 7));
    e.mem.set_u32(base + 0x40, 0);
    let mut has_texture = false;
    if coordinates != 0 {
        let textures = e.mem.u32(pass + 0x20);
        texture = e.mem.u32(textures);
        e.mem.set_u32(base + 0x40, coordinates - 1);
        has_texture = texture != 0 && e.mem.u32(texture + 8) != 0;
    }
    let second = (u32::from(e.mem.u16(pass + 0x18)) >> 1) & 7 != 0;
    if !second && !has_texture {
        return 0;
    }
    e.call(SETUP_00E7C790, &args![base, 1u32, 3u32, 0u32, 0u32]);
    if has_texture {
        let index = coordinate_index(e, base, texture);
        let stage = e
            .call(ADD_TEXTURE_STAGE, &args![base, texture, index])
            .u32();
        if second {
            set_stage_state(e, stage, 1, 4);
            set_stage_state(e, stage, 3, 0);
            set_stage_state(e, stage, 4, 4);
            set_stage_state(e, stage, 6, 0);
        } else {
            set_stage_state(e, stage, 1, 2);
            set_stage_state(e, stage, 4, 2);
        }
    } else {
        let stage = e.call(ADD_TEXTURE_STAGE, &args![base, 0u32, 0u32]).u32();
        set_stage_state(e, stage, 1, 2);
        set_stage_state(e, stage, 4, 2);
    }
    fn_00e7ca20(e, this, Ptr::new(texture));
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_ca20, fn_00e7ca20(Ptr, Ptr)),
        entry!(
            0x00e7_cac0,
            fn_00e7cac0(Ptr, Ptr, u32, Ptr, Ptr, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATES: u32 = 0x0e30_0000;

    fn states(e: &Engine) -> Vec<(u32, u32)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SET_STAGE_STATE)
            .map(|c| (c.1[1], c.1[2]))
            .collect()
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A shader with the flag byte `flag` and +0x40 = 2, a stage whose +0x10 is `STATES`, and
    /// a texture whose `u16` +4 is `index_word`.
    fn rig(flag: u8, index_word: u16) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(CURRENT_PASS, 0x0e10_0000);
        let stage = e.mem.alloc(0x14);
        e.mem.set_u32(stage + 0x10, STATES);
        e.register_double(ADD_TEXTURE_STAGE, move |_, _| stage.into_ret());
        e.register(SET_STAGE_STATE, |_, _| Ret::default());
        e.register(SETUP_00E7C790, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u8(this + 0x3c, flag);
        e.mem.set_u32(this + 0x40, 2);
        e.mem.set_u32(this + 0x44, 7);
        e.mem.set_u32(this + 0x48, 7);
        let texture = e.mem.alloc(0xc);
        e.mem.set_u16(texture + 4, index_word);
        e.mem.set_u32(texture + 8, 1);
        e.call_log = Some(vec![]);
        (e, this, texture, stage)
    }

    #[test]
    fn fn_00e7ca20_adds_the_stage_with_the_flag() {
        let (mut e, this, texture, _) = rig(1, 0x1205);
        e.call(0x00e7_ca20, &args![this, texture]);
        // Index 5 clipped to +0x40 (2).
        assert_eq!(
            calls_to(&e, ADD_TEXTURE_STAGE),
            vec![vec![this, texture, 2]]
        );
        assert_eq!(states(&e), vec![(1, 3), (4, 4), (6, 0)]);
        assert!(calls_to(&e, SET_STAGE_STATE).iter().all(|c| c[0] == STATES));
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }

    #[test]
    fn fn_00e7ca20_without_the_flag_or_a_pass() {
        let (mut e, this, texture, _) = rig(0, 1);
        e.call(0x00e7_ca20, &args![this, texture]);
        assert!(calls_to(&e, ADD_TEXTURE_STAGE).is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 0);
        // No current pass: nothing at all.
        let (mut e, this, texture, _) = rig(1, 1);
        e.mem.set_u32(CURRENT_PASS, 0);
        e.call(0x00e7_ca20, &args![this, texture]);
        assert!(calls_to(&e, ADD_TEXTURE_STAGE).is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 7);
        // A null stage: no states.
        let (mut e, this, texture, _) = rig(1, 1);
        e.register(ADD_TEXTURE_STAGE, |_, _| Ret::default());
        e.call(0x00e7_ca20, &args![this, texture]);
        assert_eq!(
            calls_to(&e, ADD_TEXTURE_STAGE),
            vec![vec![this, texture, 1]]
        );
        assert!(states(&e).is_empty());
    }

    /// A description whose +0x14 has the second state bits `bits` (`u16` +0x18) and the
    /// texture array +0x20 holding `texture`, and a property block with byte +7 `count`.
    fn pass_rig(e: &mut Engine, bits: u16, texture: u32, count: u8) -> (u32, u32) {
        let pass = e.mem.alloc(0x24);
        e.mem.set_u16(pass + 0x18, bits);
        let textures = e.mem.alloc(4);
        e.mem.set_u32(textures, texture);
        e.mem.set_u32(pass + 0x20, textures);
        let description = e.mem.alloc(0x18);
        e.mem.set_u32(description + 0x14, pass);
        let block = e.mem.alloc(8);
        e.mem.set_u8(block + 7, count);
        (block, description)
    }

    fn run(e: &mut Engine, this: u32, geometry: u32, block: u32, description: u32) -> u32 {
        e.call(
            0x00e7_cac0,
            &args![this, geometry, 0u32, block, description, 0u32, 0u32, 0u32],
        )
        .u32()
    }

    #[test]
    fn fn_00e7cac0_refuses_missing_inputs() {
        let (mut e, this, _, _) = rig(1, 1);
        e.register(BUFFER_DATA_00E8EEB0, |_, _| 0u32.into_ret());
        let (block, description) = pass_rig(&mut e, 0, 0, 0);
        assert_eq!(run(&mut e, this, 0x0e60_0000, 0, description), 0xffff_ffff);
        assert_eq!(run(&mut e, this, 0, 0, description), 0xffff_ffff);
        assert_eq!(run(&mut e, this, 0, block, description), 0xffff_ffff);
        e.register(BUFFER_DATA_00E8EEB0, |_, _| 1u32.into_ret());
        assert_eq!(run(&mut e, this, 0, block, 0), 0xffff_ffff);
        // Accepted, but neither a texture nor the second state: 0 at once.
        assert_eq!(run(&mut e, this, 0, block, description), 0);
        assert!(calls_to(&e, SETUP_00E7C790).is_empty());
    }

    #[test]
    fn fn_00e7cac0_with_a_texture_and_the_second_state() {
        let (mut e, this, texture, _) = rig(0, 7);
        let (block, description) = pass_rig(&mut e, 0x2, texture, 3);
        assert_eq!(run(&mut e, this, 0x0e60_0000, block, description), 0);
        assert_eq!(e.mem.u32(this + 0x40), 2);
        assert_eq!(calls_to(&e, SETUP_00E7C790), vec![vec![this, 1, 3, 0, 0]]);
        assert_eq!(
            calls_to(&e, ADD_TEXTURE_STAGE),
            vec![vec![this, texture, 2]]
        );
        // The flag byte is clear, so the closing `fn_00e7ca20` adds nothing more.
        assert_eq!(states(&e), vec![(1, 4), (3, 0), (4, 4), (6, 0)]);
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn fn_00e7cac0_with_a_texture_only() {
        let (mut e, this, texture, _) = rig(0, 0);
        let (block, description) = pass_rig(&mut e, 0x10, texture, 1);
        assert_eq!(run(&mut e, this, 0x0e60_0000, block, description), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(states(&e), vec![(1, 2), (4, 2)]);
    }

    #[test]
    fn fn_00e7cac0_with_the_second_state_only() {
        let (mut e, this, texture, _) = rig(1, 0);
        // The texture has a null +8: not used.
        e.mem.set_u32(texture + 8, 0);
        let (block, description) = pass_rig(&mut e, 0xe, texture, 1);
        assert_eq!(run(&mut e, this, 0x0e60_0000, block, description), 0);
        // The empty stage, then `fn_00e7ca20` with the unused texture (the flag is set).
        assert_eq!(
            calls_to(&e, ADD_TEXTURE_STAGE),
            vec![vec![this, 0, 0], vec![this, texture, 0]]
        );
        assert_eq!(states(&e), vec![(1, 2), (4, 2), (1, 3), (4, 4), (6, 0)]);
    }
}

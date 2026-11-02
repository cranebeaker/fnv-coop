//! `BSShader/run_00b98380` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two counted render-state setters. Each Direct3D render state has a word in the table at
//! `011ffa30` (filled by `00b97c10`) naming its counter in the table of words at `011ff9d8`;
//! a setter only sets the state while that counter is 0, and always adds its second argument
//! to the counter.

#[allow(unused_imports)]
use crate::prelude::*;

/// The counters (`011ff9d8`, one word per counter index).
const COUNTERS_011FF9D8: u32 = 0x011f_f9d8;
/// The `011ffa30` table's words for render state 0xA8 (`D3DRS_COLORWRITEENABLE`, at
/// `011ffcd0`) and render state 8 (`D3DRS_FILLMODE`, at `011ffa50`): their counter indices.
const COLOR_WRITE_COUNTER_INDEX: u32 = 0x011f_fcd0;
const FILL_MODE_COUNTER_INDEX: u32 = 0x011f_fa50;
/// The renderer (`011f9508`); its word +0x8B8 is the render-state object.
const RENDERER_011F9508: u32 = 0x011f_9508;
const RENDERER_RENDER_STATE: u32 = 0x8b8;
/// The render-state object's virtual +0x68 (`thiscall`, four words: the render state, its
/// value and two zero words).
const RENDER_STATE_SET: u32 = 0x68;

fn counter_address(e: &Engine, index_word: u32) -> u32 {
    let index = e.global::<u32>(index_word);
    COUNTERS_011FF9D8.wrapping_add(index.wrapping_mul(4))
}

fn set_render_state(e: &mut Engine, state: u32, value: u32) {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let object = e.mem.u32(renderer.wrapping_add(RENDERER_RENDER_STATE));
    e.vcall(object, RENDER_STATE_SET, &args![state, value, 0u32, 0u32]);
}

// Translated from 00b98380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): while the colour-write counter is 0, sets
/// render state 0xA8 (`D3DRS_COLORWRITEENABLE`) to `mask`; then adds `delta` to the counter.
pub fn fn_00b98380(e: &mut Engine, mask: u32, delta: u32) {
    let counter = counter_address(e, COLOR_WRITE_COUNTER_INDEX);
    if e.mem.u32(counter) == 0 {
        set_render_state(e, 0xa8, mask);
    }
    let count = e.mem.u32(counter);
    e.mem.set_u32(counter, count.wrapping_add(delta));
}

// Translated from 00b98480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): while the fill-mode counter is 0, sets
/// render state 8 (`D3DRS_FILLMODE`) to 2 (`D3DFILL_WIREFRAME`) when the low byte of
/// `wireframe` is not 0, else 3 (`D3DFILL_SOLID`); then adds `delta` to the counter.
pub fn fn_00b98480(e: &mut Engine, wireframe: u8, delta: u32) {
    let counter = counter_address(e, FILL_MODE_COUNTER_INDEX);
    if e.mem.u32(counter) == 0 {
        let mode = u32::from(wireframe == 0) + 2;
        set_render_state(e, 8, mode);
    }
    let count = e.mem.u32(counter);
    e.mem.set_u32(counter, count.wrapping_add(delta));
}

/// The `011ffa30` table's words for render states 0xbe, 0xbf and 0xc0
/// (`D3DRS_COLORWRITEENABLE1` to `3` in Direct3D 9's numbering, at `011ffd28`..`011ffd30`).
const COLOR_WRITE_1_COUNTER_INDEX: u32 = 0x011f_fd28;
const COLOR_WRITE_2_COUNTER_INDEX: u32 = 0x011f_fd2c;
const COLOR_WRITE_3_COUNTER_INDEX: u32 = 0x011f_fd30;

// Translated from 00b983d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: three masks and a delta): for render states
/// 0xbe, 0xbf and 0xc0 in turn (`D3DRS_COLORWRITEENABLE1` to `3`), while the state's counter is
/// 0 the state is set to its mask; then the delta is added to the counter.
pub fn fn_00b983d0(e: &mut Engine, first: u32, second: u32, third: u32, delta: u32) {
    for (index_word, state, mask) in [
        (COLOR_WRITE_1_COUNTER_INDEX, 0xbeu32, first),
        (COLOR_WRITE_2_COUNTER_INDEX, 0xbf, second),
        (COLOR_WRITE_3_COUNTER_INDEX, 0xc0, third),
    ] {
        let counter = counter_address(e, index_word);
        if e.mem.u32(counter) == 0 {
            set_render_state(e, state, mask);
        }
        let count = e.mem.u32(counter);
        e.mem.set_u32(counter, count.wrapping_add(delta));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_83d0, fn_00b983d0(u32, u32, u32, u32)),
        entry!(0x00b9_8380, fn_00b98380(u32, u32)),
        entry!(0x00b9_8480, fn_00b98480(u8, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_FN: u32 = 0x0040_5000;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.register(SET_FN, |_, _| Ret::default());
        let mut slots = vec![0u32; 27];
        slots[26] = SET_FN;
        e.put_vtable(0x0090_7000, &slots);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0090_7000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + RENDERER_RENDER_STATE, object);
        e.set_global(RENDERER_011F9508, renderer);
        (e, object)
    }

    fn sets(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == SET_FN)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn color_write_sets_only_at_zero_and_counts() {
        let (mut e, object) = engine();
        e.set_global(COLOR_WRITE_COUNTER_INDEX, 0x10u32);
        let counter = COUNTERS_011FF9D8 + 0x40;
        e.call_log = Some(vec![]);
        e.call(0x00b9_8380, &args![0xfu32, 1u32]);
        assert_eq!(sets(&mut e), vec![vec![object, 0xa8, 0xf, 0, 0]]);
        assert_eq!(e.mem.u32(counter), 1);
        e.call_log = Some(vec![]);
        e.call(0x00b9_8380, &args![0u32, u32::MAX]);
        assert!(sets(&mut e).is_empty());
        assert_eq!(e.mem.u32(counter), 0);
    }

    #[test]
    fn fill_mode_picks_wireframe_or_solid() {
        let (mut e, object) = engine();
        e.set_global(FILL_MODE_COUNTER_INDEX, 0x12u32);
        let counter = COUNTERS_011FF9D8 + 0x48;
        e.call_log = Some(vec![]);
        // Only the low byte counts: 0x100 is "solid".
        e.call(0x00b9_8480, &[0x100, 0]);
        e.call(0x00b9_8480, &args![1u8, 2u32]);
        assert_eq!(
            sets(&mut e),
            vec![vec![object, 8, 3, 0, 0], vec![object, 8, 2, 0, 0]]
        );
        assert_eq!(e.mem.u32(counter), 2);
        e.call_log = Some(vec![]);
        e.call(0x00b9_8480, &args![1u8, 1u32]);
        assert!(sets(&mut e).is_empty());
        assert_eq!(e.mem.u32(counter), 3);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    const SET_FN: u32 = 0x0040_5000;

    #[test]
    fn fn_00b983d0_sets_the_three_masks_at_zero_and_counts() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.register(SET_FN, |_, _| Ret::default());
        let mut slots = vec![0u32; 27];
        slots[26] = SET_FN;
        e.put_vtable(0x0090_7000, &slots);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0090_7000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + RENDERER_RENDER_STATE, object);
        e.set_global(RENDERER_011F9508, renderer);
        e.set_global(COLOR_WRITE_1_COUNTER_INDEX, 1u32);
        e.set_global(COLOR_WRITE_2_COUNTER_INDEX, 2u32);
        e.set_global(COLOR_WRITE_3_COUNTER_INDEX, 3u32);
        // The second state is held already.
        e.set_global(COUNTERS_011FF9D8 + 8, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x00b9_83d0, &args![0xau32, 0xbu32, 0xcu32, 1u32]);
        let sets: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == SET_FN)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            sets,
            vec![vec![object, 0xbe, 0xa, 0, 0], vec![object, 0xc0, 0xc, 0, 0]]
        );
        assert_eq!(e.global::<u32>(COUNTERS_011FF9D8 + 4), 1);
        assert_eq!(e.global::<u32>(COUNTERS_011FF9D8 + 8), 6);
        assert_eq!(e.global::<u32>(COUNTERS_011FF9D8 + 0xc), 1);
    }
}

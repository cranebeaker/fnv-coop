//! `BSShader/run_00b984f0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the 0x14-byte list element `00b988c0` constructs (the
/// destructor `00b656b0` sets the same one).
const PERSISTENT_LIST_VTABLE: u32 = 0x010b_7dc0;

// Translated from 00b988c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of one `BSTPersistentList` element of `BSShaderAccumulator`'s
/// rendered-geometry caches (Ghidra made no function here; read from the
/// bytes): sets the vtable and clears the words at +0x04, +0x08 and +0x0C.
/// Returns `this`.
pub fn fn_00b988c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, PERSISTENT_LIST_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    this
}

/// Slot-index word of render state 0xAE in the table at `011ffa30`.
const SLOT_SCISSOR_TEST_011FFCE8: u32 = 0x011f_fce8;

// Translated from 00b984f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): render state 0xAE
/// (`D3DRS_SCISSORTESTENABLE` in Direct3D 9's numbering) with `enable`,
/// unless its lock slot is locked; adds `lock` (the lock scheme of
/// `bsrenderstate_xenon.cpp`).
pub fn fn_00b984f0(e: &mut Engine, enable: u8, lock: i32) {
    use crate::units::bsshader::bsrenderstate_xenon::{add_lock, lock_slot, send_unless_locked};
    let slot = lock_slot(e, SLOT_SCISSOR_TEST_011FFCE8);
    send_unless_locked(e, slot, 0xae, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

/// The alpha-to-coverage lock counter (a single word).
const COVERAGE_LOCK_011FFA28: u32 = 0x011f_fa28;
/// Which alpha-to-coverage switch the renderer uses (1 or 2; others none).
const COVERAGE_MODE_011F94B8: u32 = 0x011f_94b8;
/// The renderer singleton pointer.
const RENDERER: u32 = 0x011f_9508;

/// Sends (`state`, `value`, 0, 0) to the renderer's render-state object
/// (renderer +0x8B8, virtual slot +0x68).
fn send_state(e: &mut Engine, state: u32, value: u32) {
    let renderer = e.mem.u32(RENDERER);
    let object = e.mem.u32(renderer.wrapping_add(0x8b8));
    e.vcall(object, 0x68, &[state, value, 0, 0]);
}

// Translated from 00b98540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): alpha to coverage through
/// a driver switch. When the counter `011ffa28` is 0: mode 1 (`011f94b8`)
/// sends render state 0xB5 with `0x434f5441` (the bytes `ATOC`) when
/// `enable` is set, else 0.0; mode 2 sends state 0x9A with `0x304d3241`
/// (`A2M0`), plus 0x0100_0000 (`A2M1`) when `enable` is set. The value
/// names are the four-character codes the immediates spell, not confirmed
/// further. `lock` is added to the counter in every case.
pub fn fn_00b98540(e: &mut Engine, enable: u8, lock: i32) {
    let count = e.mem.u32(COVERAGE_LOCK_011FFA28);
    if count == 0 {
        let mode = e.mem.u32(COVERAGE_MODE_011F94B8);
        if mode == 1 {
            let value = if enable != 0 {
                0x434f_5441
            } else {
                x87_store32(0.0)
            };
            send_state(e, 0xb5, value);
            let count = e.mem.u32(COVERAGE_LOCK_011FFA28).wrapping_add(lock as u32);
            e.mem.set_u32(COVERAGE_LOCK_011FFA28, count);
            return;
        }
        if mode == 2 {
            let on = if enable != 0 { 0x0100_0000 } else { 0 };
            send_state(e, 0x9a, on + 0x304d_3241);
            let count = e.mem.u32(COVERAGE_LOCK_011FFA28).wrapping_add(lock as u32);
            e.mem.set_u32(COVERAGE_LOCK_011FFA28, count);
            return;
        }
    }
    e.mem
        .set_u32(COVERAGE_LOCK_011FFA28, count.wrapping_add(lock as u32));
}

// Translated from 00b98610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): puts render state `state`
/// back. The states with their own setter call it with `value` (the
/// compiler's jump table, as match arms): 7 `00714a40`, 0x0E `00714a60`,
/// 0x0F `00714aa0`, 0x13 and 0x14 `00714b00`, 0x17 `00714a80`, 0x18 and
/// 0x19 `00714ac0`, 0x1B `00714ae0`, 0x34 `004ecb40`, 0x35..0x37
/// `00714b20`, 0x38..0x3A `00714b50`, 0x3B `00714b80`, 0x98 `004eb510`,
/// 0xA8 `004eced0`, 0xBE `00b985f0`, 0xC3 `00714bf0` (all cdecl, one word).
/// Any other state sends the render-state object's own stored value for
/// it (its word at +0x124 + state * 8) back through its slot +0x68.
pub fn fn_00b98610(e: &mut Engine, state: u32, value: u32) {
    let setter = match state {
        7 => 0x0071_4a40,
        0xe => 0x0071_4a60,
        0xf => 0x0071_4aa0,
        0x13 | 0x14 => 0x0071_4b00,
        0x17 => 0x0071_4a80,
        0x18 | 0x19 => 0x0071_4ac0,
        0x1b => 0x0071_4ae0,
        0x34 => 0x004e_cb40,
        0x35..=0x37 => 0x0071_4b20,
        0x38..=0x3a => 0x0071_4b50,
        0x3b => 0x0071_4b80,
        0x98 => 0x004e_b510,
        0xa8 => 0x004e_ced0,
        0xbe => 0x00b9_85f0,
        0xc3 => 0x0071_4bf0,
        _ => {
            let renderer = e.mem.u32(RENDERER);
            let object = e.mem.u32(renderer.wrapping_add(0x8b8));
            let stored = e.mem.u32(
                object
                    .wrapping_add(state.wrapping_mul(8))
                    .wrapping_add(0x124),
            );
            e.vcall(object, 0x68, &[state, stored, 0, 0]);
            return;
        }
    };
    e.call(setter, &[value]);
}

// Translated from 00b988a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function here; read from the
/// bytes; cdecl, two words, a table entry at `00b99dca` refers to it):
/// compares the low 16 bits of the two words unsigned: -1 when the first
/// is below the second, else 1.
pub fn fn_00b988a0(_e: &mut Engine, first: u32, second: u32) -> i32 {
    if (first as u16) < (second as u16) {
        -1
    } else {
        1
    }
}

/// The counter word `fn_00b985f0` takes from (`011ffa1c`, index 0x11 of the counters at
/// `011ff9d8`) and the colour-write setter it then calls (`00b983d0`, unit `run_00b98380`;
/// `cdecl`, four words).
const COUNTER_011FFA1C: u32 = 0x011f_fa1c;
const COLOR_WRITE_123_00B983D0: u32 = 0x00b9_83d0;

// Translated from 00b985f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word; `fn_00b98610`'s setter for render state 0xbe):
/// takes the word from the counter at `011ffa1c`, then calls `00b983d0` with four zero words.
pub fn fn_00b985f0(e: &mut Engine, value: u32) {
    let counter = e.mem.u32(COUNTER_011FFA1C);
    e.mem.set_u32(COUNTER_011FFA1C, counter.wrapping_sub(value));
    e.call(COLOR_WRITE_123_00B983D0, &args![0u32, 0u32, 0u32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_85f0, fn_00b985f0(u32)),
        entry!(0x00b9_88c0, fn_00b988c0(Ptr) -> Ptr),
        entry!(0x00b9_84f0, fn_00b984f0(u8, i32)),
        entry!(0x00b9_8540, fn_00b98540(u8, i32)),
        entry!(0x00b9_8610, fn_00b98610(u32, u32)),
        entry!(0x00b9_88a0, fn_00b988a0(u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod state_tests {
    use super::*;

    const STATE_SETTER: u32 = 0x0066_0068;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x7000);
        let renderer = e.mem.alloc(0x8bc);
        let object = e.mem.alloc(0x800);
        let vtable = e.mem.alloc(0x6c);
        e.mem.set_u32(vtable + 0x68, STATE_SETTER);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(renderer + 0x8b8, object);
        e.mem.set_u32(RENDERER, renderer);
        e.register(STATE_SETTER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, object)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn scissor_test_enable_uses_its_slot() {
        let (mut e, object) = engine();
        e.mem.set_u32(SLOT_SCISSOR_TEST_011FFCE8, 3);
        e.call(0x00b9_84f0, &[1, 1]);
        e.call(0x00b9_84f0, &[0, 1]);
        assert_eq!(calls(&e, STATE_SETTER), vec![vec![object, 0xae, 1, 0, 0]]);
        assert_eq!(e.mem.u32(0x011f_f9d8 + 12), 2);
    }

    #[test]
    fn coverage_switch_by_mode() {
        let (mut e, object) = engine();
        e.mem.set_u32(COVERAGE_MODE_011F94B8, 1);
        e.call(0x00b9_8540, &[1, 0]);
        e.call(0x00b9_8540, &[0, 0]);
        e.mem.set_u32(COVERAGE_MODE_011F94B8, 2);
        e.call(0x00b9_8540, &[1, 0]);
        e.call(0x00b9_8540, &[0, 1]);
        assert_eq!(
            calls(&e, STATE_SETTER),
            vec![
                vec![object, 0xb5, 0x434f_5441, 0, 0],
                vec![object, 0xb5, 0, 0, 0],
                vec![object, 0x9a, 0x314d_3241, 0, 0],
                vec![object, 0x9a, 0x304d_3241, 0, 0],
            ]
        );
        assert_eq!(e.mem.u32(COVERAGE_LOCK_011FFA28), 1);
        // Locked, or mode 0: only the counter moves.
        e.call(0x00b9_8540, &[1, 2]);
        e.mem.set_u32(COVERAGE_LOCK_011FFA28, 0);
        e.mem.set_u32(COVERAGE_MODE_011F94B8, 0);
        e.call(0x00b9_8540, &[1, 5]);
        assert_eq!(calls(&e, STATE_SETTER).len(), 4);
        assert_eq!(e.mem.u32(COVERAGE_LOCK_011FFA28), 5);
    }

    #[test]
    fn state_restore_calls_the_setter_or_resends_the_stored_value() {
        let (mut e, object) = engine();
        for setter in [0x0071_4b50, 0x00b9_85f0, 0x004e_b510] {
            e.register(setter, |_, _| Ret::default());
        }
        e.call(0x00b9_8610, &[0x39, 0x77]);
        e.call(0x00b9_8610, &[0xbe, 0x78]);
        e.call(0x00b9_8610, &[0x98, 0x79]);
        assert_eq!(calls(&e, 0x0071_4b50), vec![vec![0x77]]);
        assert_eq!(calls(&e, 0x00b9_85f0), vec![vec![0x78]]);
        assert_eq!(calls(&e, 0x004e_b510), vec![vec![0x79]]);
        assert!(calls(&e, STATE_SETTER).is_empty());
        e.mem.set_u32(object + 0x124 + 0x16 * 8, 0x55);
        e.mem.set_u32(object + 0x124 + 4 * 8, 0x44);
        e.call(0x00b9_8610, &[0x16, 0x99]);
        e.call(0x00b9_8610, &[4, 0x99]);
        assert_eq!(
            calls(&e, STATE_SETTER),
            vec![vec![object, 0x16, 0x55, 0, 0], vec![object, 4, 0x44, 0, 0]]
        );
    }

    #[test]
    fn comparator_orders_the_low_halves() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b9_88a0, &[0x0001_0002, 3]).i32(), -1);
        assert_eq!(e.call(0x00b9_88a0, &[3, 3]).i32(), 1);
        assert_eq!(e.call(0x00b9_88a0, &[4, 0xffff_0003]).i32(), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_constructor_sets_the_vtable_and_clears_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for off in (0..0x14).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b9_88c0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), PERSISTENT_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // +0x10 is not touched.
        assert_eq!(e.mem.u32(this + 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00b985f0_takes_from_the_counter_then_resets_the_masks() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        e.set_global(COUNTER_011FFA1C, 5u32);
        e.register(COLOR_WRITE_123_00B983D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b9_85f0, &args![7u32]);
        assert_eq!(e.global::<u32>(COUNTER_011FFA1C), 0xffff_fffe);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(COLOR_WRITE_123_00B983D0, vec![0, 0, 0, 0])));
    }
}

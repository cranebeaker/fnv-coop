//! `BSShader/bsrenderstate_xenon.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Each setter here looks up a lock slot for its render state: the word
//! table at `011ffa30` maps a render-state number to a slot index (`00b97c10`
//! fills it), the slot's counter sits in the word array at `011ff9d8` (the
//! Xbox PDB's static `BSRenderState::iLock` is the matching array; not
//! confirmed for PC). When the counter is 0 the setter passes the state to
//! the renderer's render-state object (renderer `011f9508` +0x8B8, virtual
//! slot +0x68, four words: state number, value, 0, 0); then it adds its
//! `lock` argument to the counter. The state numbers are the immediates of
//! the code; the names given for them are Direct3D 9's numbering
//! (`D3DRS_...`), not confirmed against the renderer.

#[allow(unused_imports)]
use crate::prelude::*;

/// The lock counters, one word per slot.
pub(crate) const LOCK_COUNTS: u32 = 0x011f_f9d8;
/// The renderer singleton pointer.
const RENDERER: u32 = 0x011f_9508;
/// The renderer's render-state object (renderer +0x8B8).
const RENDERER_STATE_OBJECT: u32 = 0x8b8;
/// The render-state object's setter (virtual slot +0x68; thiscall, four
/// words: state, value, 0, 0).
const SET_STATE_SLOT: u32 = 0x68;
/// Value table the comparison-function setters index (`011ad888`, a word
/// per entry).
const COMPARE_TABLE: u32 = 0x011a_d888;
/// Value table the blend setter indexes (`011ad8a8`).
const BLEND_TABLE: u32 = 0x011a_d8a8;
/// Value table the stencil-operation setter indexes (`011ad8d4`).
const STENCIL_OP_TABLE: u32 = 0x011a_d8d4;

/// Slot-index words (of the table at `011ffa30`) the setters read.
const SLOT_Z_WRITE: u32 = 0x011f_fa68;
const SLOT_Z_FUNC: u32 = 0x011f_fa8c;
const SLOT_ALPHA_TEST: u32 = 0x011f_fa6c;
const SLOT_ALPHA_FUNC: u32 = 0x011f_fa94;
const SLOT_ALPHA_REF: u32 = 0x011f_fa90;
const SLOT_ALPHA_BLEND: u32 = 0x011f_fa9c;
const SLOT_SRC_BLEND: u32 = 0x011f_fa7c;
const SLOT_DEST_BLEND: u32 = 0x011f_fa80;
const SLOT_STENCIL_ENABLE: u32 = 0x011f_fb00;
const SLOT_STENCIL_FAIL: u32 = 0x011f_fb04;
const SLOT_STENCIL_Z_FAIL: u32 = 0x011f_fb08;
const SLOT_STENCIL_PASS: u32 = 0x011f_fb0c;
const SLOT_STENCIL_FUNC: u32 = 0x011f_fb10;
const SLOT_STENCIL_REF: u32 = 0x011f_fb14;
const SLOT_STENCIL_MASK: u32 = 0x011f_fb18;
const SLOT_STENCIL_WRITE_MASK: u32 = 0x011f_fb1c;
const SLOT_CLIP_PLANES: u32 = 0x011f_fc90;
const SLOT_DEPTH_BIAS: u32 = 0x011f_fd3c;

/// The lock counter of the slot whose index is in the word at `slot_word`
/// (index read once; the address wraps as the exe's `[index*4 + table]`).
pub(crate) fn lock_slot(e: &Engine, slot_word: u32) -> u32 {
    let index = e.mem.u32(slot_word);
    LOCK_COUNTS.wrapping_add(index.wrapping_mul(4))
}

/// Value of a table entry `table[index]` (a word).
fn table_value(e: &Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// Sends `state` with the value `value` gives to the renderer's
/// render-state object when the lock counter `slot` is 0 (the value is
/// only computed then, as the exe reads it inside the branch).
pub(crate) fn send_unless_locked(
    e: &mut Engine,
    slot: u32,
    state: u32,
    value: impl FnOnce(&Engine) -> u32,
) {
    if e.mem.u32(slot) == 0 {
        let renderer = e.mem.u32(RENDERER);
        let object = e.mem.u32(renderer.wrapping_add(RENDERER_STATE_OBJECT));
        let value = value(e);
        e.vcall(object, SET_STATE_SLOT, &[state, value, 0, 0]);
    }
}

/// Adds `lock` to the counter `slot`.
pub(crate) fn add_lock(e: &mut Engine, slot: u32, lock: i32) {
    let count = e.mem.u32(slot).wrapping_add(lock as u32);
    e.mem.set_u32(slot, count);
}

// Translated from 00b97e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRenderState::SetZWriteEnable` (Xbox PDB; cdecl, two words): state
/// 0x0E (`D3DRS_ZWRITEENABLE`) with `enable`, unless locked; adds `lock`.
pub fn bs_render_state_set_z_write_enable(e: &mut Engine, enable: u8, lock: i32) {
    let slot = lock_slot(e, SLOT_Z_WRITE);
    send_unless_locked(e, slot, 0xe, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

// Translated from 00b97e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): state 0x17
/// (`D3DRS_ZFUNC`) with the entry `function` of the table at `011ad888`,
/// unless locked; adds `lock`.
pub fn fn_00b97e80(e: &mut Engine, function: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_Z_FUNC);
    send_unless_locked(e, slot, 0x17, |e| table_value(e, COMPARE_TABLE, function));
    add_lock(e, slot, lock);
}

// Translated from 00b97ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): state 0x0F
/// (`D3DRS_ALPHATESTENABLE`) with `enable`, unless locked; adds `lock`.
pub fn fn_00b97ed0(e: &mut Engine, enable: u8, lock: i32) {
    let slot = lock_slot(e, SLOT_ALPHA_TEST);
    send_unless_locked(e, slot, 0xf, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

// Translated from 00b97f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): state 0x19
/// (`D3DRS_ALPHAFUNC`) with the entry `function` of the table at
/// `011ad888`, then state 0x18 (`D3DRS_ALPHAREF`) with `reference`, each
/// unless its slot is locked; adds `lock` to both slots.
pub fn fn_00b97f20(e: &mut Engine, function: u32, reference: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_ALPHA_FUNC);
    send_unless_locked(e, slot, 0x19, |e| table_value(e, COMPARE_TABLE, function));
    add_lock(e, slot, lock);
    let slot = lock_slot(e, SLOT_ALPHA_REF);
    send_unless_locked(e, slot, 0x18, |_| reference);
    add_lock(e, slot, lock);
}

// Translated from 00b97fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): state 0x1B
/// (`D3DRS_ALPHABLENDENABLE`) with `enable`, unless locked; adds `lock`.
pub fn fn_00b97fa0(e: &mut Engine, enable: u8, lock: i32) {
    let slot = lock_slot(e, SLOT_ALPHA_BLEND);
    send_unless_locked(e, slot, 0x1b, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

// Translated from 00b97ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): state 0x13
/// (`D3DRS_SRCBLEND`) with the entry `source` and state 0x14
/// (`D3DRS_DESTBLEND`) with the entry `destination` of the table at
/// `011ad8a8`, each unless its slot is locked; adds `lock` to both slots.
pub fn fn_00b97ff0(e: &mut Engine, source: u32, destination: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_SRC_BLEND);
    send_unless_locked(e, slot, 0x13, |e| table_value(e, BLEND_TABLE, source));
    add_lock(e, slot, lock);
    let slot = lock_slot(e, SLOT_DEST_BLEND);
    send_unless_locked(e, slot, 0x14, |e| table_value(e, BLEND_TABLE, destination));
    add_lock(e, slot, lock);
}

// Translated from 00b98070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): state 0x34
/// (`D3DRS_STENCILENABLE`) with `enable`, unless locked; adds `lock`.
pub fn fn_00b98070(e: &mut Engine, enable: u8, lock: i32) {
    let slot = lock_slot(e, SLOT_STENCIL_ENABLE);
    send_unless_locked(e, slot, 0x34, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

// Translated from 00b980c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): states 0x35, 0x36 and
/// 0x37 (`D3DRS_STENCILFAIL`, `STENCILZFAIL`, `STENCILPASS`) with the
/// entries `fail`, `z_fail` and `pass` of the table at `011ad8d4`, each
/// unless its slot is locked. Only the third slot gets `lock` added: the
/// exe adds nothing to the first two.
pub fn fn_00b980c0(e: &mut Engine, fail: u32, z_fail: u32, pass: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_STENCIL_FAIL);
    send_unless_locked(e, slot, 0x35, |e| table_value(e, STENCIL_OP_TABLE, fail));
    let slot = lock_slot(e, SLOT_STENCIL_Z_FAIL);
    send_unless_locked(e, slot, 0x36, |e| table_value(e, STENCIL_OP_TABLE, z_fail));
    let slot = lock_slot(e, SLOT_STENCIL_PASS);
    send_unless_locked(e, slot, 0x37, |e| table_value(e, STENCIL_OP_TABLE, pass));
    add_lock(e, slot, lock);
}

// Translated from 00b98180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): state 0x38
/// (`D3DRS_STENCILFUNC`) with the entry `function` of the table at
/// `011ad888`, state 0x39 (`D3DRS_STENCILREF`) with `reference` and state
/// 0x3A (`D3DRS_STENCILMASK`) with `mask`, each unless its slot is locked.
/// Only the third slot gets `lock` added.
pub fn fn_00b98180(e: &mut Engine, function: u32, reference: u32, mask: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_STENCIL_FUNC);
    send_unless_locked(e, slot, 0x38, |e| table_value(e, COMPARE_TABLE, function));
    let slot = lock_slot(e, SLOT_STENCIL_REF);
    send_unless_locked(e, slot, 0x39, |_| reference);
    let slot = lock_slot(e, SLOT_STENCIL_MASK);
    send_unless_locked(e, slot, 0x3a, |_| mask);
    add_lock(e, slot, lock);
}

// Translated from 00b98230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): state 0x3B
/// (`D3DRS_STENCILWRITEMASK`) with `mask`, unless locked; adds `lock`.
pub fn fn_00b98230(e: &mut Engine, mask: u32, lock: i32) {
    let slot = lock_slot(e, SLOT_STENCIL_WRITE_MASK);
    send_unless_locked(e, slot, 0x3b, |_| mask);
    add_lock(e, slot, lock);
}

// Translated from 00b98280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, seven words): state 0x98
/// (`D3DRS_CLIPPLANEENABLE`) with bit `n` set for each of the six byte
/// flags `plane0`..`plane5` that is not 0, unless locked; adds `lock`.
pub fn fn_00b98280(
    e: &mut Engine,
    plane0: u8,
    plane1: u8,
    plane2: u8,
    plane3: u8,
    plane4: u8,
    plane5: u8,
    lock: i32,
) {
    let slot = lock_slot(e, SLOT_CLIP_PLANES);
    send_unless_locked(e, slot, 0x98, |_| {
        let mut bits = 0;
        for (bit, plane) in [plane0, plane1, plane2, plane3, plane4, plane5]
            .into_iter()
            .enumerate()
        {
            if plane != 0 {
                bits |= 1 << bit;
            }
        }
        bits
    });
    add_lock(e, slot, lock);
}

// Translated from 00b98320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRenderState::SetDepthBias` (Xbox PDB; cdecl, two words): reloads
/// `bias` through x87 into its own argument slot (a signalling NaN comes
/// out quiet), then state 0xC3 (`D3DRS_DEPTHBIAS`) with its bit pattern,
/// unless locked; adds `lock`.
pub fn bs_render_state_set_depth_bias(e: &mut Engine, bias: f32, lock: i32) {
    let bits = x87_quiet_bits(bias.to_bits());
    let slot = lock_slot(e, SLOT_DEPTH_BIAS);
    send_unless_locked(e, slot, 0xc3, |_| bits);
    add_lock(e, slot, lock);
}

/// The depth bias `00b64e30` uses when the flag at `011ad80f` is set.
const DECAL_DEPTH_BIAS: u32 = 0x0120_0458;
/// The byte flag that selects it (otherwise the bias is 0).
const DECAL_DEPTH_BIAS_ENABLED: u32 = 0x011a_d80f;
/// `BSShaderAccumulator`'s group renderer `00b639e0` (thiscall, two words;
/// `bsshaderaccumulator.cpp`).
const ACCUMULATOR_RENDER_GROUP: u32 = 0x00b6_39e0;
/// The lock counters of slots 1 and 3, which `00b64e30` decrements directly.
const LOCK_COUNT_1: u32 = 0x011f_f9dc;
const LOCK_COUNT_3: u32 = 0x011f_f9e4;

// Translated from 00b64e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSRenderState::SetRenderState` in the engine map, but the body is
/// a `BSShaderAccumulator` method (it reads `bRenderDecals` +0x16C and
/// `eRenderMode` +0x19C of `this`), so it keeps its address name. When
/// decals are on and the render mode is not 8: z-writes off (lock 1), the
/// depth bias at `01200458` (0 when the flag at `011ad80f` is clear; the
/// float goes through x87, so a signalling NaN is quieted) with lock 1,
/// renders the groups 2 and 3 (`00b639e0` with 1), decrements the lock
/// counter of slot 1 directly, z-writes on (lock 0), decrements the counter
/// of slot 3 and sets the depth bias 0 (lock 0).
pub fn fn_00b64e30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x16c) == 0 || e.mem.u32(base + 0x19c) == 8 {
        return;
    }
    let mut bias = x87_quiet_bits(e.mem.u32(DECAL_DEPTH_BIAS));
    if e.mem.u8(DECAL_DEPTH_BIAS_ENABLED) == 0 {
        bias = x87_store32(0.0);
    }
    bs_render_state_set_z_write_enable(e, 0, 1);
    bs_render_state_set_depth_bias(e, f32::from_bits(bias), 1);
    e.call(ACCUMULATOR_RENDER_GROUP, &[base, 2, 1]);
    e.call(ACCUMULATOR_RENDER_GROUP, &[base, 3, 1]);
    let count = e.mem.u32(LOCK_COUNT_1).wrapping_sub(1);
    e.mem.set_u32(LOCK_COUNT_1, count);
    bs_render_state_set_z_write_enable(e, 1, 0);
    let count = e.mem.u32(LOCK_COUNT_3).wrapping_sub(1);
    e.mem.set_u32(LOCK_COUNT_3, count);
    bs_render_state_set_depth_bias(e, 0.0, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_7e30, bs_render_state_set_z_write_enable(u8, i32)),
        entry!(0x00b9_7e80, fn_00b97e80(u32, i32)),
        entry!(0x00b9_7ed0, fn_00b97ed0(u8, i32)),
        entry!(0x00b9_7f20, fn_00b97f20(u32, u32, i32)),
        entry!(0x00b9_7fa0, fn_00b97fa0(u8, i32)),
        entry!(0x00b9_7ff0, fn_00b97ff0(u32, u32, i32)),
        entry!(0x00b9_8070, fn_00b98070(u8, i32)),
        entry!(0x00b9_80c0, fn_00b980c0(u32, u32, u32, i32)),
        entry!(0x00b9_8180, fn_00b98180(u32, u32, u32, i32)),
        entry!(0x00b9_8230, fn_00b98230(u32, i32)),
        entry!(0x00b9_8280, fn_00b98280(u8, u8, u8, u8, u8, u8, i32)),
        entry!(0x00b9_8320, bs_render_state_set_depth_bias(f32, i32)),
        entry!(0x00b6_4e30, fn_00b64e30(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The render-state object's setter double's address.
    const STATE_SETTER: u32 = 0x0077_0068;

    /// An engine with the lock tables mapped, a renderer whose render-state
    /// object's slot +0x68 is a double, and slot index `n` at each slot
    /// word the tests use (`n` = position in the list + 1).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x7000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        let renderer = e.mem.alloc(0x900);
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x70);
        e.mem.set_u32(vtable + SET_STATE_SLOT, STATE_SETTER);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(renderer + RENDERER_STATE_OBJECT, object);
        e.mem.set_u32(RENDERER, renderer);
        e.register(STATE_SETTER, |_, _| Ret::default());
        let slots = [
            SLOT_Z_WRITE,
            SLOT_Z_FUNC,
            SLOT_ALPHA_TEST,
            SLOT_ALPHA_FUNC,
            SLOT_ALPHA_REF,
            SLOT_ALPHA_BLEND,
            SLOT_SRC_BLEND,
            SLOT_DEST_BLEND,
            SLOT_STENCIL_ENABLE,
            SLOT_STENCIL_FAIL,
            SLOT_STENCIL_Z_FAIL,
            SLOT_STENCIL_PASS,
            SLOT_STENCIL_FUNC,
            SLOT_STENCIL_REF,
            SLOT_STENCIL_MASK,
            SLOT_STENCIL_WRITE_MASK,
            SLOT_CLIP_PLANES,
            SLOT_DEPTH_BIAS,
        ];
        for (n, slot) in slots.iter().enumerate() {
            e.mem.set_u32(*slot, n as u32 + 1);
        }
        for i in 0..8 {
            e.mem.set_u32(COMPARE_TABLE + i * 4, 0x100 + i);
            e.mem.set_u32(BLEND_TABLE + i * 4, 0x200 + i);
            e.mem.set_u32(STENCIL_OP_TABLE + i * 4, 0x300 + i);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The (state, value) pairs sent to the render-state object.
    fn sent(e: &Engine) -> Vec<(u32, u32)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == STATE_SETTER)
            .map(|(_, w)| {
                assert_eq!(&w[3..], &[0, 0]);
                (w[1], w[2])
            })
            .collect()
    }

    fn count(e: &Engine, slot_index: u32) -> u32 {
        e.mem.u32(LOCK_COUNTS + slot_index * 4)
    }

    #[test]
    fn z_write_enable_sends_unless_locked_and_adds_the_lock() {
        let mut e = engine();
        e.call(0x00b9_7e30, &[0x101, 1]);
        assert_eq!(sent(&e), vec![(0xe, 1)]);
        assert_eq!(count(&e, 1), 1);
        // Locked now: nothing is sent, the lock is still added.
        e.call(0x00b9_7e30, &[0, 0xffff_ffff]);
        assert_eq!(sent(&e).len(), 1);
        assert_eq!(count(&e, 1), 0);
    }

    #[test]
    fn z_func_looks_up_its_value() {
        let mut e = engine();
        e.call(0x00b9_7e80, &[3, 2]);
        assert_eq!(sent(&e), vec![(0x17, 0x103)]);
        assert_eq!(count(&e, 2), 2);
    }

    #[test]
    fn alpha_test_enable_skips_a_locked_slot() {
        let mut e = engine();
        e.mem.set_u32(LOCK_COUNTS + 3 * 4, 5);
        e.call(0x00b9_7ed0, &[1, 1]);
        assert!(sent(&e).is_empty());
        assert_eq!(count(&e, 3), 6);
    }

    #[test]
    fn alpha_function_and_reference_lock_both_slots() {
        let mut e = engine();
        e.mem.set_u32(LOCK_COUNTS + 5 * 4, 1);
        e.call(0x00b9_7f20, &[4, 0x80, 1]);
        // The reference slot (5) is locked.
        assert_eq!(sent(&e), vec![(0x19, 0x104)]);
        assert_eq!(count(&e, 4), 1);
        assert_eq!(count(&e, 5), 2);
    }

    #[test]
    fn alpha_blend_enable_sends_the_byte() {
        let mut e = engine();
        e.call(0x00b9_7fa0, &[0x200, 1]);
        assert_eq!(sent(&e), vec![(0x1b, 0)]);
        assert_eq!(count(&e, 6), 1);
    }

    #[test]
    fn blend_functions_look_up_both_values() {
        let mut e = engine();
        e.call(0x00b9_7ff0, &[1, 6, 1]);
        assert_eq!(sent(&e), vec![(0x13, 0x201), (0x14, 0x206)]);
        assert_eq!(count(&e, 7), 1);
        assert_eq!(count(&e, 8), 1);
    }

    #[test]
    fn stencil_enable_sends_the_byte() {
        let mut e = engine();
        e.call(0x00b9_8070, &[1, 1]);
        assert_eq!(sent(&e), vec![(0x34, 1)]);
        assert_eq!(count(&e, 9), 1);
    }

    #[test]
    fn stencil_operations_lock_only_the_pass_slot() {
        let mut e = engine();
        e.call(0x00b9_80c0, &[0, 1, 2, 1]);
        assert_eq!(sent(&e), vec![(0x35, 0x300), (0x36, 0x301), (0x37, 0x302)]);
        assert_eq!(count(&e, 10), 0);
        assert_eq!(count(&e, 11), 0);
        assert_eq!(count(&e, 12), 1);
    }

    #[test]
    fn stencil_function_locks_only_the_mask_slot() {
        let mut e = engine();
        e.mem.set_u32(LOCK_COUNTS + 13 * 4, 1);
        e.call(0x00b9_8180, &[2, 7, 0xff, 1]);
        // The function slot (13) is locked; the other two are sent.
        assert_eq!(sent(&e), vec![(0x39, 7), (0x3a, 0xff)]);
        assert_eq!(count(&e, 13), 1);
        assert_eq!(count(&e, 14), 0);
        assert_eq!(count(&e, 15), 1);
    }

    #[test]
    fn stencil_write_mask_sends_the_mask() {
        let mut e = engine();
        e.call(0x00b9_8230, &[0xf0, 1]);
        assert_eq!(sent(&e), vec![(0x3b, 0xf0)]);
        assert_eq!(count(&e, 16), 1);
    }

    #[test]
    fn clip_planes_become_bits() {
        let mut e = engine();
        e.call(0x00b9_8280, &[1, 0, 0x100, 2, 0, 0xff, 1]);
        // 0x100 is not a byte flag: its low byte is 0.
        assert_eq!(sent(&e), vec![(0x98, 0b10_1001)]);
        assert_eq!(count(&e, 17), 1);
        e.call(0x00b9_8280, &[1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(sent(&e).len(), 1);
    }

    #[test]
    fn depth_bias_quiets_a_signalling_nan() {
        let mut e = engine();
        e.call(0x00b9_8320, &[0x7f80_0001, 1]);
        assert_eq!(sent(&e), vec![(0xc3, 0x7fc0_0001)]);
        assert_eq!(count(&e, 18), 1);
    }

    fn groups(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == ACCUMULATOR_RENDER_GROUP)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn decal_pass_renders_two_groups_between_the_state_changes() {
        let mut e = engine();
        e.register(ACCUMULATOR_RENDER_GROUP, |_, _| Ret::default());
        let this = e.mem.alloc(0x280);
        e.mem.set_u8(this + 0x16c, 1);
        e.mem.set_u32(DECAL_DEPTH_BIAS, 0x3a83_126f);
        e.mem.set_u8(DECAL_DEPTH_BIAS_ENABLED, 1);
        // Slot 1 (z-write) is locked by 5 so neither z-write is sent.
        e.mem.set_u32(LOCK_COUNT_1, 5);
        e.mem.set_u32(LOCK_COUNT_3, 9);
        e.call(0x00b6_4e30, &[this]);
        assert_eq!(sent(&e), vec![(0xc3, 0x3a83_126f)]);
        assert_eq!(groups(&e), vec![vec![this, 2, 1], vec![this, 3, 1]]);
        // +1, -1 and +0 for slot 1; -1 for slot 3.
        assert_eq!(e.mem.u32(LOCK_COUNT_1), 5);
        assert_eq!(e.mem.u32(LOCK_COUNT_3), 8);
        // The depth-bias slot (18) got +1, so the second bias was locked.
        assert_eq!(count(&e, 18), 1);
    }

    #[test]
    fn decal_pass_uses_bias_zero_and_skips_mode_eight() {
        let mut e = engine();
        e.register(ACCUMULATOR_RENDER_GROUP, |_, _| Ret::default());
        let this = e.mem.alloc(0x280);
        e.mem.set_u8(this + 0x16c, 1);
        e.mem.set_u32(DECAL_DEPTH_BIAS, 0x3a83_126f);
        e.call(0x00b6_4e30, &[this]);
        assert_eq!(sent(&e), vec![(0xe, 0), (0xc3, 0), (0xe, 1)]);
        assert_eq!(e.mem.u32(LOCK_COUNT_3), 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x19c, 8);
        e.call(0x00b6_4e30, &[this]);
        e.mem.set_u32(this + 0x19c, 0);
        e.mem.set_u8(this + 0x16c, 0);
        e.call(0x00b6_4e30, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}

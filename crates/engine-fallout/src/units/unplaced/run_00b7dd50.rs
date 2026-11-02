//! `(unplaced)/run_00b7dd50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Six shader pass functions (no map names; they follow
//! `ShadowLightShader::SetupGeometryConstants` in the exe). Two run from
//! `BeginPass` (calls at `00b99422`..`00b99449`), four from
//! `RenderPassImmediately_Standard` (`00b98f00`..`00b98f7c`) with the
//! render pass. They fill the constant entries of the shader state at
//! `0126f74c` (`+0x24`: an array of entry pointers; an entry's value word is
//! at `+8`) from the pass's geometry, and set vertex shader constants
//! through the device at `[[0126f74c]+0x5c]+0x20` (vtable slot `0x178`,
//! `SetVertexShaderConstantF`'s place in `IDirect3DDevice9`; not confirmed
//! beyond the slot).
//!
//! The functions keep several locals in one stack frame and reuse slots
//! (`00b7dfe0` hands slot `+4` to the final call after it held the
//! transformed vector); the translations keep one block of the frame's size
//! and pass the same offsets.

#[allow(unused_imports)]
use crate::prelude::*;

/// The shader state global (a pointer).
const SHADER_STATE: u32 = 0x0126_f74c;
/// Two objects whose word at `+0x20` `00b7dd50` sets to 1.
const GLOBAL_011FEBF0: u32 = 0x011f_ebf0;
const GLOBAL_011FEC38: u32 = 0x011f_ec38;
/// The global pointer to the object whose `+0x68` matrix the passes use.
const GLOBAL_011F91E0: u32 = 0x011f_91e0;
/// The word stored in constant entry 3 when the geometry has no slot-`0xf4`
/// value.
const GLOBAL_011F951C: u32 = 0x011f_951c;
/// The vector `D3DXVec3TransformCoord` transforms (passed by address).
const VECTOR_011FA2A0: u32 = 0x011f_a2a0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;

/// `00e7eb00`: `thiscall` on a constant entry, one word (0 here).
const ENTRY_00E7EB00: u32 = 0x00e7_eb00;
/// `00e7de90`: `thiscall` on a constant entry, one word.
const ENTRY_00E7DE90: u32 = 0x00e7_de90;
/// `00c03230`: `stdcall` (RET 8), two words (geometry, 0); the caller's ECX is not one.
const SHADER_00C03230: u32 = 0x00c0_3230;
/// `00c4c2d0`: `cdecl`, three words (matrix, 0, 64-byte output).
const MATRIX_00C4C2D0: u32 = 0x00c4_c2d0;
/// `00b7c750`: `thiscall` on the shader. `00b7c7b0`, `00b7c850`, `00b7cb00`: `stdcall`
/// (RET 4 / 8 / 0xc, ECX not read); the caller's ECX (the shader) is not one of their
/// words.
const SHADER_00B7C750: u32 = 0x00b7_c750;
const SHADER_00B7C7B0: u32 = 0x00b7_c7b0;
const SHADER_00B7C850: u32 = 0x00b7_c850;
const SHADER_00B7CB00: u32 = 0x00b7_cb00;
/// `00b7c3a0`, `00b7c510`, `00b7c580`, `00b7c600`: `thiscall` on the
/// shader, no arguments.
const SHADER_00B7C3A0: u32 = 0x00b7_c3a0;
const SHADER_00B7C510: u32 = 0x00b7_c510;
const SHADER_00B7C580: u32 = 0x00b7_c580;
const SHADER_00B7C600: u32 = 0x00b7_c600;
/// `D3DXVec3TransformCoord` (engine map; a 6-byte jump thunk), `stdcall`,
/// three words (output, vector, matrix).
const D3DX_VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;

/// The shader state's constant entry `index` (read afresh, as the code does
/// each time).
fn constant_entry(e: &Engine, index: u32) -> u32 {
    let state = e.global::<u32>(SHADER_STATE);
    let entries = e.mem.u32(state + 0x24);
    e.mem.u32(entries + 4 * index)
}

/// The device the vertex shader constants are set on (`[[0126f74c]+0x5c]+0x20`).
fn device(e: &Engine) -> u32 {
    let state = e.global::<u32>(SHADER_STATE);
    let holder = e.mem.u32(state + 0x5c);
    e.mem.u32(holder + 0x20)
}

/// The final call of three passes: when the shader has an object at `+0x30`
/// and the shader state a word at `+0x44`, that object's slot `0x78` with
/// the word, six zeros, the frame slots `big` (0x34 bytes) and `small`
/// (0x10 bytes), 0 and 1.
fn final_slot_78(e: &mut Engine, this: u32, big: u32, small: u32) {
    let object = e.mem.u32(this + 0x30);
    if object == 0 {
        return;
    }
    let state = e.global::<u32>(SHADER_STATE);
    let word = e.mem.u32(state + 0x44);
    if word == 0 {
        return;
    }
    e.vcall(
        object,
        0x78,
        &args![word, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, big, small, 0u32, 1u32],
    );
}

/// The geometry part shared by the four render-pass functions: constant
/// entry 0 gets the word at `[geometry+0xac]`, entry 1 `00c03230(geometry,
/// 0)`, each followed by `00e7de90(entry, geometry slot 0xe0)`.
fn geometry_entries(e: &mut Engine, geometry: u32) {
    let source = e.mem.u32(geometry + 0xac);
    let entry0 = constant_entry(e, 0);
    let value = e.mem.u32(source);
    e.mem.set_u32(entry0 + 8, value);
    let word = e.vcall(geometry, 0xe0, &args![]).u32();
    e.call(ENTRY_00E7DE90, &args![entry0, word]);
    let entry1 = constant_entry(e, 1);
    // 00b7de2a: `00c03230` is stdcall (RET 8): the geometry and 0; ECX is not a word.
    let value = e.call(SHADER_00C03230, &args![geometry, 0u32]).u32();
    e.mem.set_u32(entry1 + 8, value);
    let word = e.vcall(geometry, 0xe0, &args![]).u32();
    e.call(ENTRY_00E7DE90, &args![entry1, word]);
}

/// `00e7eb00(entry, 0)` on constant entries 0 and 1.
fn clear_entries_0_1(e: &mut Engine) {
    let entry0 = constant_entry(e, 0);
    e.call(ENTRY_00E7EB00, &args![entry0, 0u32]);
    let entry1 = constant_entry(e, 1);
    e.call(ENTRY_00E7EB00, &args![entry1, 0u32]);
}

/// The matrix part: `00c4c2d0(object + 0x68, 0, out)` and
/// `00b7c750(this, object, object + 0x68)` for the object at
/// `[011f91e0]`. Returns the object.
fn matrix_part(e: &mut Engine, this: u32, out: u32) -> u32 {
    let holder = e.global::<u32>(GLOBAL_011F91E0);
    let object = e.mem.u32(holder);
    let matrix = object + 0x68;
    e.call(MATRIX_00C4C2D0, &args![matrix, 0u32, out]);
    e.call(SHADER_00B7C750, &args![this, object, matrix]);
    object
}

// Translated from 00b7dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pass set-up (from `BeginPass`): `00e7eb00(entry 5, 0)`, `00b7c3a0`, the
/// words at `+0x20` of the objects at `011febf0` and `011fec38` set to 1,
/// then `00b7c510`, `00b7c580`, `00b7c600`, and the final slot-`0x78` call.
pub fn fn_00b7dd50(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let entry5 = constant_entry(e, 5);
    e.call(ENTRY_00E7EB00, &args![entry5, 0u32]);
    e.call(SHADER_00B7C3A0, &args![this]);
    let first = e.global::<u32>(GLOBAL_011FEBF0);
    e.mem.set_u32(first + 0x20, 1);
    let second = e.global::<u32>(GLOBAL_011FEC38);
    e.mem.set_u32(second + 0x20, 1);
    e.call(SHADER_00B7C510, &args![this]);
    e.call(SHADER_00B7C580, &args![this]);
    e.call(SHADER_00B7C600, &args![this]);
    // Frame: 0x44 bytes; the small slot at +0, the big one at +0x10.
    e.with_stack(0x44, |e, frame| {
        let frame = frame.addr();
        final_slot_78(e, this, frame + 0x10, frame);
    });
}

// Translated from 00b7dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Render pass (geometry `[pass+0xc]`): the geometry entries, entries 0 and
/// 1 cleared, the matrix part into a 64-byte local, then `00b7c7b0(local)`.
pub fn fn_00b7dde0(e: &mut Engine, this: Ptr, pass: Ptr) {
    let this = this.addr();
    let geometry = e.mem.u32(pass.addr() + 0xc);
    geometry_entries(e, geometry);
    clear_entries_0_1(e);
    e.with_stack(0x40, |e, frame| {
        let local = frame.addr();
        matrix_part(e, this, local);
        // 00b7de95: `00b7c7b0` is stdcall (RET 4) and does not read ECX.
        e.call(SHADER_00B7C7B0, &args![local]);
    });
}

// Translated from 00b7deb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pass set-up (from `BeginPass`): `00b7c3a0`, `00b7c510`, `00b7c580`,
/// `00b7c600` (the last a tail jump).
pub fn fn_00b7deb0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.call(SHADER_00B7C3A0, &args![this]);
    e.call(SHADER_00B7C510, &args![this]);
    e.call(SHADER_00B7C580, &args![this]);
    e.call(SHADER_00B7C600, &args![this]);
}

// Translated from 00b7ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Render pass: as [`fn_00b7dde0`] up to the matrix part (output at frame
/// `+0x10`), then `00b7c850(output, float at object+0x98)` and the final
/// slot-`0x78` call (big slot `+0x50`, small slot `+0`).
pub fn fn_00b7ded0(e: &mut Engine, this: Ptr, pass: Ptr) {
    let this = this.addr();
    let geometry = e.mem.u32(pass.addr() + 0xc);
    geometry_entries(e, geometry);
    clear_entries_0_1(e);
    e.with_stack(0x84, |e, frame| {
        let frame = frame.addr();
        let local = frame + 0x10;
        let object = matrix_part(e, this, local);
        let value = x87_quiet_bits(e.mem.u32(object + 0x68 + 0x30));
        // 00b7df8f: `00b7c850` is stdcall (RET 8) and does not read ECX.
        e.call(SHADER_00B7C850, &args![local, value]);
        final_slot_78(e, this, frame + 0x50, frame);
    });
}

// Translated from 00b7dfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Render pass: the geometry entries, entries 0 and 1 cleared, the matrix
/// part (output at frame `+0x14`), `00b7cb00(output, float at
/// object+0x98, [pass+8])`; then the byte at `[011f91e0]+9` minus 1.0 as
/// the fourth float of `D3DXVec3TransformCoord(frame+4, 011fa2a0, output)`,
/// that vector set as vertex shader constant 0x10 (one register), and the
/// final slot-`0x78` call (big slot `+0x54`, small slot `+4`).
pub fn fn_00b7dfe0(e: &mut Engine, this: Ptr, pass: Ptr) {
    let this = this.addr();
    let geometry = e.mem.u32(pass.addr() + 0xc);
    geometry_entries(e, geometry);
    clear_entries_0_1(e);
    e.with_stack(0x88, |e, frame| {
        let frame = frame.addr();
        let local = frame + 0x14;
        let holder = e.global::<u32>(GLOBAL_011F91E0);
        let object = matrix_part(e, this, local);
        let value = x87_quiet_bits(e.mem.u32(object + 0x68 + 0x30));
        let pass_word = e.mem.u32(pass.addr() + 8);
        // 00b7e0ae: `00b7cb00` is stdcall (RET 0xc) and does not read ECX.
        e.call(SHADER_00B7CB00, &args![local, value, pass_word]);
        let byte = e.mem.u8(holder + 9);
        let one: f64 = e.global(ONE);
        let w = x87_store32(x87_sub(f64::from(byte), one));
        e.mem.set_u32(frame, w);
        let vector = frame + 4;
        e.call(
            D3DX_VEC3_TRANSFORM_COORD,
            &args![vector, VECTOR_011FA2A0, local],
        );
        let w = x87_quiet_bits(e.mem.u32(frame));
        e.mem.set_u32(frame + 0x10, w);
        let device = device(e);
        e.vcall(device, 0x178, &args![0x10u32, vector, 1u32]);
        final_slot_78(e, this, frame + 0x54, frame + 4);
    });
}

// Translated from 00b7e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Render pass: the geometry entries; constant entry 3 gets the geometry's
/// slot `0xf4(3, 0)` (asked twice when non-zero) or the word at
/// `011f951c`; entries 0, 1 and 3 cleared; the matrix part (output at frame
/// `+0x10`) and `00b7c7b0(output)`; then the float at object+0x98 as the
/// fourth float of `D3DXVec3TransformCoord(frame, 011fa2a0, output)`, set
/// as vertex shader constant 0x10 (one register).
pub fn fn_00b7e150(e: &mut Engine, this: Ptr, pass: Ptr) {
    let this = this.addr();
    let geometry = e.mem.u32(pass.addr() + 0xc);
    geometry_entries(e, geometry);
    let entry3 = constant_entry(e, 3);
    let value = if e.vcall(geometry, 0xf4, &args![3u32, 0u32]).u32() != 0 {
        e.vcall(geometry, 0xf4, &args![3u32, 0u32]).u32()
    } else {
        e.global::<u32>(GLOBAL_011F951C)
    };
    e.mem.set_u32(entry3 + 8, value);
    clear_entries_0_1(e);
    let entry3 = constant_entry(e, 3);
    e.call(ENTRY_00E7EB00, &args![entry3, 0u32]);
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        let local = frame + 0x10;
        let object = matrix_part(e, this, local);
        // 00b7e255: `00b7c7b0` is stdcall (RET 4) and does not read ECX.
        e.call(SHADER_00B7C7B0, &args![local]);
        // The float is kept in the dead argument slot over the call.
        let w = x87_quiet_bits(e.mem.u32(object + 0x68 + 0x30));
        e.call(
            D3DX_VEC3_TRANSFORM_COORD,
            &args![frame, VECTOR_011FA2A0, local],
        );
        e.mem.set_u32(frame + 0xc, w);
        let device = device(e);
        e.vcall(device, 0x178, &args![0x10u32, frame, 1u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_dd50, fn_00b7dd50(Ptr)),
        entry!(0x00b7_dde0, fn_00b7dde0(Ptr, Ptr)),
        entry!(0x00b7_deb0, fn_00b7deb0(Ptr)),
        entry!(0x00b7_ded0, fn_00b7ded0(Ptr, Ptr)),
        entry!(0x00b7_dfe0, fn_00b7dfe0(Ptr, Ptr)),
        entry!(0x00b7_e150, fn_00b7e150(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    struct World {
        this: u32,
        pass: u32,
        geometry: u32,
        entries: [u32; 6],
        object: u32,
        device: u32,
        listener: u32,
    }

    fn vtable(e: &mut Engine, object: u32, base: u32, slots: &[u32]) {
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(object, table);
        for slot in slots {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| 0x55u32.into_ret());
        }
    }

    fn world(e: &mut Engine) -> World {
        let state = e.mem.alloc(0x60);
        e.map(SHADER_STATE, 4);
        e.set_global(SHADER_STATE, state);
        let array = e.mem.alloc(0x18);
        e.mem.set_u32(state + 0x24, array);
        let mut entries = [0u32; 6];
        for (i, entry) in entries.iter_mut().enumerate() {
            *entry = e.mem.alloc(0x10);
            e.mem.set_u32(array + 4 * i as u32, *entry);
        }
        let device = e.mem.alloc(4);
        vtable(e, device, 0x7300_0000, &[0x178]);
        let device_holder = e.mem.alloc(0x24);
        e.mem.set_u32(device_holder + 0x20, device);
        e.mem.set_u32(state + 0x5c, device_holder);
        e.mem.set_u32(state + 0x44, 0x4444);
        let this = e.mem.alloc(0x40);
        let listener = e.mem.alloc(4);
        vtable(e, listener, 0x7100_0000, &[0x78]);
        e.mem.set_u32(this + 0x30, listener);
        let geometry = e.mem.alloc(0xb0);
        vtable(e, geometry, 0x7200_0000, &[0xe0, 0xf4]);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x5050);
        e.mem.set_u32(geometry + 0xac, source);
        let pass = e.mem.alloc(0x10);
        e.mem.set_u32(pass + 0xc, geometry);
        e.mem.set_u32(pass + 8, 0x0808);
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0x98, 2.5);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, object);
        e.mem.set_u8(holder + 9, 4);
        e.map(GLOBAL_011F91E0, 4);
        e.set_global(GLOBAL_011F91E0, holder);
        e.map(GLOBAL_011F951C, 4);
        e.set_global(GLOBAL_011F951C, 0x951cu32);
        e.map(ONE, 8);
        e.set_global(ONE, 1.0f64);
        for addr in [
            ENTRY_00E7EB00,
            ENTRY_00E7DE90,
            MATRIX_00C4C2D0,
            SHADER_00B7C750,
            SHADER_00B7C7B0,
            SHADER_00B7C850,
            SHADER_00B7CB00,
            SHADER_00B7C3A0,
            SHADER_00B7C510,
            SHADER_00B7C580,
            SHADER_00B7C600,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SHADER_00C03230, |_, _| 0xc032u32.into_ret());
        e.register(D3DX_VEC3_TRANSFORM_COORD, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0x1000 + i);
            }
            Ret::default()
        });
        World {
            this,
            pass,
            geometry,
            entries,
            object,
            device,
            listener,
        }
    }

    #[test]
    fn begin_pass_setups() {
        let mut e = Engine::new();
        let w = world(&mut e);
        let first = e.mem.alloc(0x24);
        let second = e.mem.alloc(0x24);
        e.map(GLOBAL_011FEBF0, 4);
        e.set_global(GLOBAL_011FEBF0, first);
        e.map(GLOBAL_011FEC38, 4);
        e.set_global(GLOBAL_011FEC38, second);
        e.call_log = Some(vec![]);
        e.call(0x00b7_dd50, &args![w.this]);
        assert_eq!(calls_to(&e, ENTRY_00E7EB00), vec![vec![w.entries[5], 0]]);
        assert_eq!(e.mem.u32(first + 0x20), 1);
        assert_eq!(e.mem.u32(second + 0x20), 1);
        assert_eq!(calls_to(&e, SHADER_00B7C600), vec![vec![w.this]]);
        let last = calls_to(&e, 0x7100_0078);
        assert_eq!(last.len(), 1);
        let small = last[0][9];
        assert_eq!(last[0][8], small + 0x10);
        assert_eq!(
            [&last[0][..8], &last[0][10..]].concat(),
            vec![w.listener, 0x4444, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        // No object at +0x30: no final call.
        e.mem.set_u32(w.this + 0x30, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b7_dd50, &args![w.this]);
        assert!(calls_to(&e, 0x7100_0078).is_empty());

        e.call_log = Some(vec![]);
        e.call(0x00b7_deb0, &args![w.this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SHADER_00B7C3A0, vec![w.this]),
                (SHADER_00B7C510, vec![w.this]),
                (SHADER_00B7C580, vec![w.this]),
                (SHADER_00B7C600, vec![w.this]),
            ]
        );
    }

    fn check_geometry_part(e: &Engine, w: &World) {
        assert_eq!(e.mem.u32(w.entries[0] + 8), 0x5050);
        assert_eq!(e.mem.u32(w.entries[1] + 8), 0xc032);
        assert_eq!(
            calls_to(e, ENTRY_00E7DE90),
            vec![vec![w.entries[0], 0x55], vec![w.entries[1], 0x55]]
        );
        assert_eq!(calls_to(e, SHADER_00C03230), vec![vec![w.geometry, 0]]);
        let matrix = w.object + 0x68;
        assert_eq!(
            calls_to(e, SHADER_00B7C750),
            vec![vec![w.this, w.object, matrix]]
        );
    }

    #[test]
    fn render_pass_dde0() {
        let mut e = Engine::new();
        let w = world(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b7_dde0, &args![w.this, w.pass]);
        check_geometry_part(&e, &w);
        assert_eq!(
            calls_to(&e, ENTRY_00E7EB00),
            vec![vec![w.entries[0], 0], vec![w.entries[1], 0]]
        );
        let matrix = calls_to(&e, MATRIX_00C4C2D0);
        assert_eq!(matrix[0][..2], [w.object + 0x68, 0]);
        assert_eq!(calls_to(&e, SHADER_00B7C7B0), vec![vec![matrix[0][2]]]);
    }

    #[test]
    fn render_pass_ded0() {
        let mut e = Engine::new();
        let w = world(&mut e);
        e.mem.set_u32(w.object + 0x98, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00b7_ded0, &args![w.this, w.pass]);
        check_geometry_part(&e, &w);
        let out = calls_to(&e, MATRIX_00C4C2D0)[0][2];
        assert_eq!(calls_to(&e, SHADER_00B7C850), vec![vec![out, 0x7fc0_0001]]);
        let last = calls_to(&e, 0x7100_0078);
        assert_eq!(last[0][9], out - 0x10);
        assert_eq!(last[0][8], out + 0x40);
    }

    #[test]
    fn render_pass_dfe0() {
        let mut e = Engine::new();
        let w = world(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b7_dfe0, &args![w.this, w.pass]);
        check_geometry_part(&e, &w);
        let out = calls_to(&e, MATRIX_00C4C2D0)[0][2];
        assert_eq!(
            calls_to(&e, SHADER_00B7CB00),
            vec![vec![out, 2.5f32.to_bits(), 0x0808]]
        );
        let transform = calls_to(&e, D3DX_VEC3_TRANSFORM_COORD);
        let vector = transform[0][0];
        assert_eq!(vector, out - 0x10);
        assert_eq!(transform[0][1..], [VECTOR_011FA2A0, out]);
        // The fourth float is the byte at holder+9 (4) minus 1.0.
        assert_eq!(e.mem.f32(vector + 0xc), 3.0);
        assert_eq!(
            calls_to(&e, 0x7300_0178),
            vec![vec![w.device, 0x10, vector, 1]]
        );
        let last = calls_to(&e, 0x7100_0078);
        assert_eq!(last[0][9], vector);
        assert_eq!(last[0][8], vector + 0x50);
    }

    #[test]
    fn render_pass_e150() {
        let mut e = Engine::new();
        let w = world(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b7_e150, &args![w.this, w.pass]);
        check_geometry_part(&e, &w);
        // Slot 0xf4 answered non-zero: asked twice, its value kept.
        assert_eq!(calls_to(&e, 0x7200_00f4).len(), 2);
        assert_eq!(e.mem.u32(w.entries[3] + 8), 0x55);
        assert_eq!(
            calls_to(&e, ENTRY_00E7EB00),
            vec![
                vec![w.entries[0], 0],
                vec![w.entries[1], 0],
                vec![w.entries[3], 0]
            ]
        );
        let out = calls_to(&e, MATRIX_00C4C2D0)[0][2];
        assert_eq!(calls_to(&e, SHADER_00B7C7B0), vec![vec![out]]);
        let transform = calls_to(&e, D3DX_VEC3_TRANSFORM_COORD);
        let vector = transform[0][0];
        assert_eq!(vector, out - 0x10);
        assert_eq!(e.mem.f32(vector + 0xc), 2.5);
        assert_eq!(
            calls_to(&e, 0x7300_0178),
            vec![vec![w.device, 0x10, vector, 1]]
        );
        assert!(calls_to(&e, 0x7100_0078).is_empty());

        // Slot 0xf4 zero: the word at 011f951c.
        e.register(0x7200_00f4, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b7_e150, &args![w.this, w.pass]);
        assert_eq!(calls_to(&e, 0x7200_00f4).len(), 1);
        assert_eq!(e.mem.u32(w.entries[3] + 8), 0x951c);
    }
}

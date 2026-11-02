//! `BSShader/run_00b97900` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa1070` (`cdecl`, size).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// CRT `_memset` (`00ec61c0`, `cdecl`: destination, byte value word, size).
const MEMSET: u32 = 0x00ec_61c0;
/// `ImageSpaceManager::Initialize` (engine map, `00b977d0`): `thiscall`, no stack words.
const IMAGE_SPACE_MANAGER_INITIALIZE: u32 = 0x00b9_77d0;
/// The vtable `00b97a80` puts at +4 first (`010b9d24`) and the one that replaces it after
/// the allocation (`010b8004`); neither has an RTTI locator before it.
const VTABLE_010B9D24: u32 = 0x010b_9d24;
const VTABLE_010B8004: u32 = 0x010b_8004;

/// The words `00b97a80` clears with `FLDZ`/`FST` (0.0 as a float), in the exe's order.
const ZERO_FLOATS_00B97A80: [u32; 24] = [
    0x214, 0x218, 0x21c, 0x220, 0x224, 0x228, 0x22c, 0x230, 0x238, 0x23c, 0x240, 0x244, 0x248,
    0x24c, 0x250, 0x254, 0x2f8, 0x2fc, 0x300, 0x304, 0x308, 0x30c, 0x310, 0x314,
];

// Translated from 00b97a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b97900`): the constructor of the 0x31c-byte object the
/// shader manager's set-up (`00b57420`) keeps at `011f91ac`. `thiscall`, no stack words.
/// Sets the 16-bit words at +0xc (0x74), +0x12 (1), +0xe and +0x10 (0) and the vtable
/// `010b9d24` at +4; stores the result of `00aa1070(0x1d0)` at +8; puts the vtable `010b8004`
/// at +4; clears the words and bytes from +0xac to +0x210 that the exe clears (the word at
/// +0x1ec is set to -1) and 24 floats from +0x214 to +0x314 (not +0x234); clears 0x98 bytes
/// at +0x14 (`_memset`); then runs `ImageSpaceManager::Initialize` (`00b977d0`) on it.
/// Answers `this`.
// Translation incomplete: 00b97a80 - the C++ exception frame (FS:[0] chain, unwind state 5 around the field set-up) is not translated.
pub fn fn_00b97a80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let at = |offset: u32| base.wrapping_add(offset);
    e.mem.set_u16(at(0xc), 0x74);
    e.mem.set_u32(at(4), VTABLE_010B9D24);
    e.mem.set_u16(at(0x12), 1);
    e.mem.set_u16(at(0xe), 0);
    e.mem.set_u16(at(0x10), 0);
    let block = e.call(ALLOCATE_00AA1070, &args![0x1d0u32]).u32();
    e.mem.set_u32(at(8), block);
    e.mem.set_u32(at(4), VTABLE_010B8004);
    e.mem.set_u32(at(0xac), 0);
    e.mem.set_u32(at(0xb0), 0);
    e.mem.set_u8(at(0x1e4), 0);
    e.mem.set_u32(at(0x1ec), 0xffff_ffff);
    e.mem.set_u32(at(0x1f0), 0);
    e.mem.set_u32(at(0x1f4), 0);
    e.mem.set_u32(at(0x1f8), 0);
    e.mem.set_u8(at(0x1fc), 0);
    e.mem.set_u32(at(0x200), 0);
    e.mem.set_u8(at(0x204), 0);
    e.mem.set_u8(at(0x205), 0);
    e.mem.set_u8(at(0x206), 0);
    e.mem.set_u32(at(0x208), 0);
    e.mem.set_u32(at(0x20c), 0);
    e.mem.set_u32(at(0x210), 0);
    let zero = x87_store32(0.0);
    for offset in ZERO_FLOATS_00B97A80 {
        e.mem.set_u32(at(offset), zero);
    }
    e.call(MEMSET, &args![at(0x14), 0u32, 0x98u32]);
    e.call(IMAGE_SPACE_MANAGER_INITIALIZE, &args![this]);
    this
}

/// The two blocks `00b97c10` fills: 0x3fc bytes of 0xff at `011ffa30` and 0x54 bytes of 0
/// at `011ff9d8`.
const BLOCK_011FFA30: u32 = 0x011f_fa30;
const BLOCK_011FF9D8: u32 = 0x011f_f9d8;

/// The (word, value) stores `00b97c10` makes after the two fills, in the exe's order.
const STORES_00B97C10: [(u32, u32); 27] = [
    (0x011f_fa94, 5),
    (0x011f_fa90, 5),
    (0x011f_fa7c, 7),
    (0x011f_fa80, 7),
    (0x011f_fb04, 10),
    (0x011f_fb08, 10),
    (0x011f_fb0c, 10),
    (0x011f_fb10, 0xb),
    (0x011f_fb14, 0xb),
    (0x011f_fb18, 0xb),
    (0x011f_fa4c, 0),
    (0x011f_fa68, 1),
    (0x011f_fa8c, 2),
    (0x011f_fd3c, 3),
    (0x011f_fa6c, 4),
    (0x011f_fa9c, 6),
    (0x011f_fcdc, 8),
    (0x011f_fb00, 9),
    (0x011f_fb1c, 0xc),
    (0x011f_fce8, 0xd),
    (0x011f_fc90, 0xf),
    (0x011f_fcd0, 0x10),
    (0x011f_fd28, 0x11),
    (0x011f_fd2c, 0x11),
    (0x011f_fd30, 0x11),
    (0x011f_fa50, 0x12),
    (0x011f_fa88, 0x13),
];

// Translated from 00b97c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b97900`). No arguments: fills 0x3fc bytes at `011ffa30`
/// with 0xff and 0x54 bytes at `011ff9d8` with 0 (`_memset`, the fill value passed as the
/// word the exe pushes), then stores 27 small numbers into words of the `011ffa30` table (the
/// immediates of its `MOV` instructions, in the exe's order).
pub fn fn_00b97c10(e: &mut Engine) {
    e.call(MEMSET, &args![BLOCK_011FFA30, 0xffff_ffffu32, 0x3fcu32]);
    e.call(MEMSET, &args![BLOCK_011FF9D8, 0u32, 0x54u32]);
    for (word, value) in STORES_00B97C10 {
        e.set_global(word, value);
    }
}

/// The image-space pass set-up `00b8b3f0` (`imagespacemanager.cpp`;
/// thiscall, no stack words).
const IMAGE_SPACE_PREPARE_00B8B3F0: u32 = 0x00b8_b3f0;
/// Renders one image-space effect (`00b97550`, `imagespacemanager.cpp`;
/// thiscall, six words: effect index, the caller's first word, source,
/// destination, 0, 1).
const IMAGE_SPACE_RENDER_EFFECT_00B97550: u32 = 0x00b9_7550;
/// The texture manager singleton pointer.
const TEXTURE_MANAGER_011F91A8: u32 = 0x011f_91a8;
/// Borrows a render target from the texture manager (`00b6e110`;
/// thiscall, five words).
const TEXTURE_MANAGER_BORROW_00B6E110: u32 = 0x00b6_e110;
/// `BSTextureManager::ReturnRenderedTexture` (engine map; thiscall, one
/// word).
const TEXTURE_MANAGER_RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// Word of the thread's TLS block `00b97900` sets to 0x27 and restores.
const TLS_WORD_2B4: u32 = 0x2b4;

// Translated from 00b97900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on the 0x31c-byte object `00b97a80`
/// constructs, three words: a word passed on, and pointers to the source
/// and destination targets): with word +0x2B4 of the thread block set to
/// 0x27, runs `00b8b3f0`, stores `*destination` in the object's first word
/// and renders its effects (`00b97550`). With no last effect (+0x1EC is
/// -1) that is effect 0x22 from `*source` to `*destination`. Otherwise each
/// effect `i` from 0 to +0x1EC whose slot (+0x08 array) is set and whose
/// virtual slot +0x18 answers true renders from the current source to the
/// current target, and then the two swap. The first target is
/// `*destination` (mode +0x1E8 is 1) or the target +0x318; with a mode
/// above 2 a borrowed render target (`00b6e110(word, 3, 0, 0, 0)`) is the
/// target after the first effect and goes back to the texture manager at
/// the end. The last effect always targets `*destination`. The TLS word is
/// restored on both paths.
// Translation incomplete: 00b97900 - the C++ exception frame (FS:[0] chain, unwind state 0 around the effects) is not translated.
pub fn fn_00b97900(e: &mut Engine, this: Ptr, word: u32, source: Ptr, destination: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_WORD_2B4);
    e.mem.set_u32(tls + TLS_WORD_2B4, 0x27);
    e.call(IMAGE_SPACE_PREPARE_00B8B3F0, &[base]);
    let last = e.mem.u32(base + 0x1ec);
    let first_word = e.mem.u32(destination.addr());
    e.mem.set_u32(base, first_word);
    if last == 0xffff_ffff {
        let target = e.mem.u32(destination.addr());
        let from = e.mem.u32(source.addr());
        e.call(
            IMAGE_SPACE_RENDER_EFFECT_00B97550,
            &[base, 0x22, word, from, target, 0, 1],
        );
        e.mem.set_u32(tls + TLS_WORD_2B4, saved);
        return;
    }
    let mut from = e.mem.u32(source.addr());
    let mode = e.mem.u32(base + 0x1e8) as i32;
    let mut borrowed = 0u32;
    let mut target;
    if mode == 1 {
        target = e.mem.u32(destination.addr());
    } else {
        target = e.mem.u32(base + 0x318);
        if mode > 2 {
            let manager = e.mem.u32(TEXTURE_MANAGER_011F91A8);
            borrowed = e
                .call(
                    TEXTURE_MANAGER_BORROW_00B6E110,
                    &[manager, word, 3, 0, 0, 0],
                )
                .u32();
        }
    }
    let mut rendered = 0u32;
    let mut index = 0i32;
    if e.mem.u32(base + 0x1ec) as i32 >= index {
        loop {
            if rendered == 1 && borrowed != 0 {
                target = borrowed;
            }
            if index == e.mem.u32(base + 0x1ec) as i32 {
                target = e.mem.u32(destination.addr());
            }
            let effects = e.mem.u32(base + 8);
            let effect = e
                .mem
                .u32(effects.wrapping_add((index as u32).wrapping_mul(4)));
            if effect != 0 && e.vcall(effect, 0x18, &[]).u8() != 0 {
                e.call(
                    IMAGE_SPACE_RENDER_EFFECT_00B97550,
                    &[base, index as u32, word, from, target, 0, 1],
                );
                rendered = rendered.wrapping_add(1);
                std::mem::swap(&mut from, &mut target);
            }
            index += 1;
            if index > e.mem.u32(base + 0x1ec) as i32 {
                break;
            }
        }
    }
    if borrowed != 0 {
        let manager = e.mem.u32(TEXTURE_MANAGER_011F91A8);
        e.call(
            TEXTURE_MANAGER_RETURN_RENDERED_TEXTURE,
            &[manager, borrowed],
        );
    }
    e.mem.set_u32(tls + TLS_WORD_2B4, saved);
}

/// The scissor lock counter (a single word).
const SCISSOR_LOCK_011FFA10: u32 = 0x011f_fa10;
/// The object whose +0x288 is the Direct3D device `00b97d20` calls.
const DEVICE_HOLDER_011F4748: u32 = 0x011f_4748;
/// The device's `SetScissorRect` slot (+0x12C, `IDirect3DDevice9` order;
/// called with the device pushed first).
const DEVICE_SET_SCISSOR_RECT: u32 = 0x12c;

// Translated from 00b97d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words): when the scissor lock
/// counter `011ffa10` is 0, passes the rectangle (`left`, `top`, `right`,
/// `bottom`, copied to the stack) to the device's `SetScissorRect`; adds
/// `lock` to the counter either way.
pub fn fn_00b97d20(e: &mut Engine, left: u32, top: u32, right: u32, bottom: u32, lock: i32) {
    if e.mem.u32(SCISSOR_LOCK_011FFA10) == 0 {
        e.with_stack(0x10, |e, rect| {
            let rect = rect.addr();
            e.mem.set_u32(rect, left);
            e.mem.set_u32(rect + 4, top);
            e.mem.set_u32(rect + 8, right);
            e.mem.set_u32(rect + 0xc, bottom);
            let holder = e.mem.u32(DEVICE_HOLDER_011F4748);
            let device = e.mem.u32(holder.wrapping_add(0x288));
            e.vcall(device, DEVICE_SET_SCISSOR_RECT, &[rect]);
        });
    }
    let count = e.mem.u32(SCISSOR_LOCK_011FFA10).wrapping_add(lock as u32);
    e.mem.set_u32(SCISSOR_LOCK_011FFA10, count);
}

/// The table that maps a render-state number to its lock slot.
const STATE_SLOTS_011FFA30: u32 = 0x011f_fa30;

// Translated from 00b97d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the general render-state
/// setter of `bsrenderstate_xenon.cpp`'s lock scheme: looks up the lock
/// slot of `state` in the table at `011ffa30`, sends (`state`, `value`) to
/// the renderer's render-state object unless the slot is locked, and adds
/// `lock` to the slot's counter.
pub fn fn_00b97d90(e: &mut Engine, state: u32, value: u32, lock: i32) {
    use crate::units::bsshader::bsrenderstate_xenon::{add_lock, lock_slot, send_unless_locked};
    let slot = lock_slot(e, STATE_SLOTS_011FFA30.wrapping_add(state.wrapping_mul(4)));
    send_unless_locked(e, slot, state, |_| value);
    add_lock(e, slot, lock);
}

/// Slot-index word of render state 7 in the table at `011ffa30`.
const SLOT_Z_ENABLE_011FFA4C: u32 = 0x011f_fa4c;

// Translated from 00b97de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): render state 7
/// (`D3DRS_ZENABLE` in Direct3D 9's numbering) with `enable`, unless its
/// slot is locked; adds `lock`.
pub fn fn_00b97de0(e: &mut Engine, enable: u8, lock: i32) {
    use crate::units::bsshader::bsrenderstate_xenon::{add_lock, lock_slot, send_unless_locked};
    let slot = lock_slot(e, SLOT_Z_ENABLE_011FFA4C);
    send_unless_locked(e, slot, 7, |_| u32::from(enable));
    add_lock(e, slot, lock);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_7a80, fn_00b97a80(Ptr) -> Ptr),
        entry!(0x00b9_7c10, fn_00b97c10()),
        entry!(0x00b9_7900, fn_00b97900(Ptr, u32, Ptr, Ptr)),
        entry!(0x00b9_7d20, fn_00b97d20(u32, u32, u32, u32, i32)),
        entry!(0x00b9_7d90, fn_00b97d90(u32, u32, i32)),
        entry!(0x00b9_7de0, fn_00b97de0(u8, i32)),
    ]
}

#[cfg(test)]
mod effect_tests {
    use super::*;

    const EFFECT_ACTIVE: u32 = 0x0066_0018;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An image-space object with `count` effect slots: effect `i` is
    /// present when bit `i` of `present` is set and active when bit `i` of
    /// `active` is.
    fn engine(count: u32, present: u32, active: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.mem.set_u32(TEXTURE_MANAGER_011F91A8, 0x0077_0000);
        e.register(IMAGE_SPACE_PREPARE_00B8B3F0, |_, _| Ret::default());
        e.register(IMAGE_SPACE_RENDER_EFFECT_00B97550, |_, _| Ret::default());
        e.register(TEXTURE_MANAGER_BORROW_00B6E110, |_, _| 0xb0b0u32.into_ret());
        e.register(TEXTURE_MANAGER_RETURN_RENDERED_TEXTURE, |_, _| {
            Ret::default()
        });
        e.register(EFFECT_ACTIVE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(vtable + 0x18, EFFECT_ACTIVE);
        let this = e.mem.alloc(0x31c);
        let slots = e.mem.alloc(4 * count.max(1));
        for i in 0..count {
            if present & (1 << i) != 0 {
                let effect = e.mem.alloc(8);
                e.mem.set_u32(effect, vtable);
                e.mem.set_u8(effect + 4, u8::from(active & (1 << i) != 0));
                e.mem.set_u32(slots + i * 4, effect);
            }
        }
        e.mem.set_u32(this + 8, slots);
        e.mem.set_u32(this + 0x1ec, count.wrapping_sub(1));
        e.mem.set_u32(this + 0x318, 0x3180);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x5000);
        let destination = e.mem.alloc(4);
        e.mem.set_u32(destination, 0xd000);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2B4, 9);
        e.call_log = Some(vec![]);
        (e, this, source, destination)
    }

    #[test]
    fn without_effects_renders_effect_22() {
        let (mut e, this, source, destination) = engine(0, 0, 0);
        e.call(0x00b9_7900, &[this, 0x42, source, destination]);
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT_00B97550),
            vec![vec![this, 0x22, 0x42, 0x5000, 0xd000, 0, 1]]
        );
        assert_eq!(e.mem.u32(this), 0xd000);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 9);
    }

    #[test]
    fn active_effects_ping_pong_and_the_last_hits_the_destination() {
        // Effects 0, 1 and 3 active, 2 present but inactive; mode 0.
        let (mut e, this, source, destination) = engine(4, 0b1111, 0b1011);
        e.call(0x00b9_7900, &[this, 0x42, source, destination]);
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT_00B97550),
            vec![
                vec![this, 0, 0x42, 0x5000, 0x3180, 0, 1],
                vec![this, 1, 0x42, 0x3180, 0x5000, 0, 1],
                vec![this, 3, 0x42, 0x5000, 0xd000, 0, 1],
            ]
        );
        assert!(calls(&e, TEXTURE_MANAGER_BORROW_00B6E110).is_empty());
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 9);
    }

    #[test]
    fn mode_three_borrows_a_target_and_returns_it() {
        let (mut e, this, source, destination) = engine(3, 0b101, 0b101);
        e.mem.set_u32(this + 0x1e8, 3);
        e.call(0x00b9_7900, &[this, 0x42, source, destination]);
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_BORROW_00B6E110),
            vec![vec![0x0077_0000, 0x42, 3, 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT_00B97550),
            vec![
                vec![this, 0, 0x42, 0x5000, 0x3180, 0, 1],
                vec![this, 2, 0x42, 0x3180, 0xd000, 0, 1],
            ]
        );
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_RETURN_RENDERED_TEXTURE),
            vec![vec![0x0077_0000, 0xb0b0]]
        );
        // Mode 1 starts on the destination.
        let (mut e, this, source, destination) = engine(2, 0b11, 0b11);
        e.mem.set_u32(this + 0x1e8, 1);
        e.call(0x00b9_7900, &[this, 0x42, source, destination]);
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT_00B97550)[0],
            vec![this, 0, 0x42, 0x5000, 0xd000, 0, 1]
        );
    }

    #[test]
    fn scissor_rectangle_goes_to_the_device_unless_locked() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let holder = e.mem.alloc(0x28c);
        let device = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x130);
        e.mem.set_u32(vtable + DEVICE_SET_SCISSOR_RECT, 0x0066_012c);
        e.mem.set_u32(device, vtable);
        e.mem.set_u32(holder + 0x288, device);
        e.mem.set_u32(DEVICE_HOLDER_011F4748, holder);
        e.register(0x0066_012c, |e, a| {
            let rect: Vec<u32> = (0..4).map(|i| e.mem.u32(a[1] + i * 4)).collect();
            assert_eq!(rect, vec![1, 2, 3, 4]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b9_7d20, &[1, 2, 3, 4, 1]);
        assert_eq!(e.mem.u32(SCISSOR_LOCK_011FFA10), 1);
        e.call(0x00b9_7d20, &[1, 2, 3, 4, 0xffff_ffff]);
        assert_eq!(e.mem.u32(SCISSOR_LOCK_011FFA10), 0);
        assert_eq!(calls(&e, 0x0066_012c).len(), 1);
    }

    fn state_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x7000);
        let renderer = e.mem.alloc(0x8bc);
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x6c);
        e.mem.set_u32(vtable + 0x68, 0x0066_0068);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(renderer + 0x8b8, object);
        e.mem.set_u32(0x011f_9508, renderer);
        e.register(0x0066_0068, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn general_setter_uses_the_state_slot_table() {
        let mut e = state_engine();
        // State 0x16 uses slot 4, which slot 4's counter (2) locks at first.
        e.mem.set_u32(STATE_SLOTS_011FFA30 + 0x16 * 4, 4);
        e.mem.set_u32(0x011f_f9d8 + 16, 2);
        e.call(0x00b9_7d90, &[0x16, 9, 0xffff_fffe]);
        assert!(calls(&e, 0x0066_0068).is_empty());
        assert_eq!(e.mem.u32(0x011f_f9d8 + 16), 0);
        e.call(0x00b9_7d90, &[0x16, 9, 1]);
        let object = e.mem.u32(e.mem.u32(0x011f_9508) + 0x8b8);
        assert_eq!(calls(&e, 0x0066_0068), vec![vec![object, 0x16, 9, 0, 0]]);
        assert_eq!(e.mem.u32(0x011f_f9d8 + 16), 1);
    }

    #[test]
    fn z_enable_sends_state_seven() {
        let mut e = state_engine();
        e.mem.set_u32(SLOT_Z_ENABLE_011FFA4C, 2);
        e.call(0x00b9_7de0, &[0x301, 1]);
        let object = e.mem.u32(e.mem.u32(0x011f_9508) + 0x8b8);
        assert_eq!(calls(&e, 0x0066_0068), vec![vec![object, 7, 1, 0, 0]]);
        assert_eq!(e.mem.u32(0x011f_f9d8 + 8), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// `_memset` double that fills like the CRT (low byte of the value word).
    fn memset(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..a[2] {
            e.mem.set_u8(a[0] + i, a[1] as u8);
        }
        a[0].into_ret()
    }

    #[test]
    fn fn_00b97a80_sets_its_fields_then_initializes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x31c);
        for offset in (0..0x31c).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.register(ALLOCATE_00AA1070, |_, _| 0x0abc_0000u32.into_ret());
        e.register(MEMSET, memset);
        e.register(IMAGE_SPACE_MANAGER_INITIALIZE, |e, a| {
            // Everything is set before Initialize runs.
            assert_eq!(e.mem.u32(a[0] + 0x1ec), 0xffff_ffff);
            assert_eq!(e.mem.u32(a[0] + 0x14), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_7a80, &args![this]).u32(), this);

        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![0x1d0]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x14, 0, 0x98]]);
        assert_eq!(calls(&e, IMAGE_SPACE_MANAGER_INITIALIZE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), VTABLE_010B8004);
        assert_eq!(e.mem.u32(this + 8), 0x0abc_0000);
        assert_eq!(e.mem.u32(this + 0xc), 0x0000_0074);
        assert_eq!(e.mem.u32(this + 0x10), 0x0001_0000);
        assert_eq!(e.mem.bytes(this + 0x14, 0x98), vec![0; 0x98]);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x1e4), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x1e8), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x1fc), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x204), 0xde00_0000);
        for offset in [0x1f0, 0x1f4, 0x1f8, 0x200, 0x208, 0x20c, 0x210] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        for offset in ZERO_FLOATS_00B97A80 {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x234), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x258), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x318), 0xdead_beef);
    }

    #[test]
    fn fn_00b97c10_fills_the_tables_then_stores_the_numbers() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        e.register(MEMSET, memset);
        e.call_log = Some(vec![]);
        e.call(0x00b9_7c10, &args![]);
        assert_eq!(
            calls(&e, MEMSET),
            vec![
                vec![BLOCK_011FFA30, 0xffff_ffff, 0x3fc],
                vec![BLOCK_011FF9D8, 0, 0x54]
            ]
        );
        for (word, value) in STORES_00B97C10 {
            assert_eq!(e.global::<u32>(word), value, "{word:#x}");
        }
        // Untouched words of the table keep the fill.
        assert_eq!(e.global::<u32>(0x011f_fa30), 0xffff_ffff);
        assert_eq!(e.global::<u32>(0x011f_fe28), 0xffff_ffff);
        assert_eq!(e.global::<u32>(0x011f_fe2c), 0);
        assert_eq!(e.global::<u32>(0x011f_f9d8), 0);
    }
}

//! `BSShader/run_00b6adb0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderedCubeMap::ms_RTTI` (`012708e4`: its static initializer `00fc7950` gives it the
/// name `"NiRenderedCubeMap"`), the renderer (`011f9508`) and the render target maker
/// `00a9ff00` (unnamed; `cdecl`: a buffer, the renderer).
const RENDERED_CUBE_MAP_RTTI_012708E4: u32 = 0x0127_08e4;
const RENDERER_011F9508: u32 = 0x011f_9508;
const RENDER_TARGET_00A9FF00: u32 = 0x00a9_ff00;

/// The word at +0x10 of the object at +0x10 of `holder`, or 0 when there is none.
fn inner_word(e: &Engine, holder: u32) -> u32 {
    let inner = e.mem.u32(holder.wrapping_add(0x10));
    if inner == 0 {
        0
    } else {
        e.mem.u32(inner + 0x10)
    }
}

// Translated from 00b6b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): a texture's render target, `cdecl` with two
/// words: the texture and a target that may be reused. For an `NiRenderedCubeMap` texture
/// (`GetRTTI`, vtable slot +8, then each base through the RTTI's +4): the target is kept when
/// it is set and at least as wide (+8) and high (+0xc) as the cube map's buffer (texture
/// +0x4c), else `00a9ff00(buffer, renderer)` is answered. Other textures (also a null one, whose
/// vtable is then read): the target is kept when it is set, at least as wide as the texture's
/// width (slot +0x94) and high as its height (slot +0x98), and its +0x10's +0x10 matches that of
/// the texture's buffer (slot +0xa4); else `00a9ff00` of the texture's buffer (the renderer read
/// first) is answered.
pub fn fn_00b6b180(e: &mut Engine, texture: Ptr, target: Ptr) -> u32 {
    let texture = texture.addr();
    let target = target.addr();
    if texture != 0 {
        let mut rtti = e.vcall(texture, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == RENDERED_CUBE_MAP_RTTI_012708E4 {
                let buffer = e.mem.u32(texture + 0x4c);
                if target != 0
                    && e.mem.u32(target + 8) >= e.mem.u32(buffer.wrapping_add(8))
                    && e.mem.u32(target + 0xc) >= e.mem.u32(buffer.wrapping_add(0xc))
                {
                    return target;
                }
                let renderer = e.global::<u32>(RENDERER_011F9508);
                return e
                    .call(RENDER_TARGET_00A9FF00, &args![buffer, renderer])
                    .u32();
            }
            rtti = e.mem.u32(rtti + 4);
        }
    }
    if target != 0 {
        let width = e.mem.u32(target + 8);
        if width >= e.vcall(texture, 0x94, &args![]).u32() {
            let height = e.mem.u32(target + 0xc);
            if height >= e.vcall(texture, 0x98, &args![]).u32() {
                let kept = inner_word(e, target);
                let buffer = e.vcall(texture, 0xa4, &args![]).u32();
                if kept == inner_word(e, buffer) {
                    return target;
                }
            }
        }
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let buffer = e.vcall(texture, 0xa4, &args![]).u32();
    e.call(RENDER_TARGET_00A9FF00, &args![buffer, renderer])
        .u32()
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00b6aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): an `NiPointer` constructor, `thiscall` with the
/// object: stores it at `this` and, when not null, counts its reference count (+4) up through
/// `InterlockedIncrement`. Answers `this`.
pub fn fn_00b6aed0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
    this
}

/// What the functions below use: the destructor `00b6a520` (unnamed; `thiscall`, no stack
/// words), the sized free `00aa1460` (`cdecl`: block, size), the imported
/// `InterlockedDecrement` (import slot), the table `00b6aef0` stores (`010ae500`; no RTTI
/// locator precedes it), the compiler's `eh vector destructor iterator` (`00ec5fce`,
/// `stdcall`: array, element size, count, destructor) with the two element destructors it is
/// given (`0045cec0`, `00b66f20`) and the `NiObject` destructor `00a5d3d0` (`niobject.cpp`;
/// `thiscall`).
const DESTRUCTOR_00B6A520: u32 = 0x00b6_a520;
const FREE_00AA1460: u32 = 0x00aa_1460;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const TABLE_010AE500: u32 = 0x010a_e500;
const VECTOR_DESTRUCTOR_00EC5FCE: u32 = 0x00ec_5fce;
const ELEMENT_DESTRUCTOR_0045CEC0: u32 = 0x0045_cec0;
const ELEMENT_DESTRUCTOR_00B66F20: u32 = 0x00b6_6f20;
const NI_OBJECT_DOWN_00A5D3D0: u32 = 0x00a5_d3d0;

/// An `NiPointer` let go: the object's count (+4) down through `InterlockedDecrement` and, at
/// zero, its virtual slot 4 (delete).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b6adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): a deleting destructor, `thiscall` with the
/// flags: the destructor `00b6a520`, then with bit 0 the 0x150 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn fn_00b6adb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00B6A520, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x150u32]);
    }
    this
}

// Translated from 00b6aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): a destructor, `thiscall`. The table `010ae500`;
/// the six `NiPointer`s at +8 and the four at +0x30 let go (count down, deleted at zero) and
/// cleared; the compiler's array destructor (`00ec5fce`) run over the four at +0x30
/// (`0045cec0`); the `NiPointer`s at +0x28, +0x24 and +0x20 let go (not cleared); the array
/// destructor run over the six at +8 (`00b66f20`); then the `NiObject` destructor
/// (`00a5d3d0`).
// Translation incomplete: 00b6aef0 - C++ exception unwinding (the FS:[0] frame and its states) is not translated.
pub fn fn_00b6aef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, TABLE_010AE500);
    for (start, count) in [(8u32, 6u32), (0x30, 4)] {
        for index in 0..count {
            let at = base.wrapping_add(start + 4 * index);
            let object = e.mem.u32(at);
            if object != 0 {
                release_reference(e, object);
                e.mem.set_u32(at, 0);
            }
        }
    }
    e.call(
        VECTOR_DESTRUCTOR_00EC5FCE,
        &args![
            base.wrapping_add(0x30),
            4u32,
            4u32,
            ELEMENT_DESTRUCTOR_0045CEC0
        ],
    );
    for offset in [0x28u32, 0x24, 0x20] {
        let object = e.mem.u32(base.wrapping_add(offset));
        release_reference(e, object);
    }
    e.call(
        VECTOR_DESTRUCTOR_00EC5FCE,
        &args![
            base.wrapping_add(8),
            4u32,
            6u32,
            ELEMENT_DESTRUCTOR_00B66F20
        ],
    );
    e.call(NI_OBJECT_DOWN_00A5D3D0, &args![base]);
}

/// A stack of at most ten `NiPointer`s: its count (`011fa020`) and its slots (`011fa024`).
const POINTER_STACK_COUNT_011FA020: u32 = 0x011f_a020;
const POINTER_STACK_011FA024: u32 = 0x011f_a024;

// Translated from 00b6b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`), `cdecl` with an object: pushes it on the
/// pointer stack at `011fa024` when the count (`011fa020`) is below 10. The slot at the count
/// gets it as an `NiPointer` assignment (unless it already holds it: the old one let go, the
/// new one counted when not null); the count goes up either way.
pub fn fn_00b6b050(e: &mut Engine, object: u32) {
    let count = e.mem.u32(POINTER_STACK_COUNT_011FA020);
    if count >= 10 {
        return;
    }
    let slot = POINTER_STACK_011FA024.wrapping_add(count.wrapping_mul(4));
    let held = e.mem.u32(slot);
    if held != object {
        release_reference(e, held);
        e.mem.set_u32(slot, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    let count = e.mem.u32(POINTER_STACK_COUNT_011FA020).wrapping_add(1);
    e.mem.set_u32(POINTER_STACK_COUNT_011FA020, count);
}

// Translated from 00b6b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`; no arguments): the top of the pointer stack (the
/// word at `011fa020 + 4 * count`), or 0 when the count is 0 (then read from a null
/// `NiPointer` temporary). The temporary's release (count down through `InterlockedDecrement`,
/// deleted at zero) is guarded by a flag set only on the empty path, where the temporary is
/// null, so it never runs (Ghidra drops it as unreachable); it is translated as the code has
/// it.
pub fn fn_00b6b100(e: &mut Engine) -> u32 {
    let count = e.mem.u32(POINTER_STACK_COUNT_011FA020);
    let (top, made_temporary, temporary) = if count != 0 {
        let at = POINTER_STACK_COUNT_011FA020.wrapping_add(count.wrapping_mul(4));
        (e.mem.u32(at), false, 0u32)
    } else {
        (0, true, 0u32)
    };
    if made_temporary && temporary != 0 {
        release_reference(e, temporary);
    }
    top
}

// Translated from 00b6b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): the deleting destructor of [`fn_00b6aef0`],
/// `thiscall` with the flags: the destructor, then with bit 0 the 0x40 bytes freed
/// (`00aa1460`). Answers `this`.
pub fn fn_00b6b160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b6aef0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x40u32]);
    }
    this
}

/// The allocation `00aa13e0` (`cdecl`, one word: the size), `WaterShaderProperty`'s
/// constructor (`00b6abb0`, engine map; `thiscall`, no words) and the water property's member
/// copy `00b6a6c0` (unit `run_00b69e10`; `thiscall`, two words: the clone and the process).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const WATER_SHADER_PROPERTY_CONSTRUCT: u32 = 0x00b6_abb0;
const WATER_COPY_MEMBERS_00B6A6C0: u32 = 0x00b6_a6c0;

/// 0x150 bytes from `00aa13e0` constructed as a `WaterShaderProperty` (`00b6abb0`), or null
/// when the allocation gave none.
fn new_water_property(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x150u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(WATER_SHADER_PROPERTY_CONSTRUCT, &args![block]).u32()
}

// Translated from 00b6ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `WaterShaderProperty`'s vtable slot +0x48 (`CreateClone` in the
/// Xbox PDB's slot list; `thiscall`, one word: the cloning process). A new water property
/// (0x150 bytes, `00b6abb0`; null when the allocation gave none) gets this one's members
/// (`00b6a6c0` with it and the process, also with a null one). Answers the clone.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b6ade0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b6ade0(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let clone = new_water_property(e);
    e.call(WATER_COPY_MEMBERS_00B6A6C0, &args![this, clone, process]);
    clone
}

// Translated from 00b6ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra's library match `CreateObject` is the decompiler's;
/// `cdecl`, no arguments): a new `WaterShaderProperty` (0x150 bytes from `00aa13e0`,
/// constructed by `00b6abb0`), or null when the allocation gave none.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b6ae60 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b6ae60(e: &mut Engine) -> u32 {
    new_water_property(e)
}

// Translated from 00b6b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): pops the pointer stack of
/// `fn_00b6b050`. When the count (`011fa020`) is not 0 it goes down by one and the slot at the
/// new count, when it holds an object, is let go (count down, deleted at zero) and cleared.
pub fn fn_00b6b0b0(e: &mut Engine) {
    let count = e.mem.u32(POINTER_STACK_COUNT_011FA020);
    if count == 0 {
        return;
    }
    let count = count.wrapping_sub(1);
    let slot = POINTER_STACK_011FA024.wrapping_add(count.wrapping_mul(4));
    let object = e.mem.u32(slot);
    e.mem.set_u32(POINTER_STACK_COUNT_011FA020, count);
    if object != 0 {
        release_reference(e, object);
        e.mem.set_u32(slot, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_ade0, fn_00b6ade0(Ptr, u32) -> u32),
        entry!(0x00b6_ae60, fn_00b6ae60() -> u32),
        entry!(0x00b6_b0b0, fn_00b6b0b0()),
        entry!(0x00b6_b180, fn_00b6b180(Ptr, Ptr) -> u32),
        entry!(0x00b6_aed0, fn_00b6aed0(Ptr, u32) -> Ptr),
        entry!(0x00b6_adb0, fn_00b6adb0(Ptr, u32) -> Ptr),
        entry!(0x00b6_aef0, fn_00b6aef0(Ptr)),
        entry!(0x00b6_b050, fn_00b6b050(u32)),
        entry!(0x00b6_b100, fn_00b6b100() -> u32),
        entry!(0x00b6_b160, fn_00b6b160(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTURE_VTABLE: u32 = 0x0e00_0000;
    const GET_RTTI: u32 = 0x0e01_0008;
    const WIDTH: u32 = 0x0e01_0094;
    const HEIGHT: u32 = 0x0e01_0098;
    const BUFFER_OF: u32 = 0x0e01_00a4;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A texture whose RTTI chain ends in `base` (one RTTI in front), 64 x 32, its buffer's
    /// +0x10 object holding 5 at +0x10; render targets answered as 0x0e20_0000.
    fn engine(base: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(RENDERER_011F9508, 0x0e10_0000u32);
        let mut slots = vec![0u32; 0x2a];
        slots[2] = GET_RTTI;
        slots[0x25] = WIDTH;
        slots[0x26] = HEIGHT;
        slots[0x29] = BUFFER_OF;
        e.put_vtable(TEXTURE_VTABLE, &slots);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base);
        e.register_double(GET_RTTI, move |_, _| rtti.into_ret());
        e.register(WIDTH, |_, _| 64u32.into_ret());
        e.register(HEIGHT, |_, _| 32u32.into_ret());
        let buffer = e.mem.alloc(0x14);
        let inner = e.mem.alloc(0x14);
        e.mem.set_u32(inner + 0x10, 5);
        e.mem.set_u32(buffer + 0x10, inner);
        e.mem.set_u32(buffer + 8, 16);
        e.mem.set_u32(buffer + 0xc, 16);
        e.register_double(BUFFER_OF, move |_, _| buffer.into_ret());
        e.register(RENDER_TARGET_00A9FF00, |_, _| 0x0e20_0000u32.into_ret());
        let texture = e.mem.alloc(0x50);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        e.mem.set_u32(texture + 0x4c, buffer);
        e.call_log = Some(vec![]);
        (e, texture, buffer)
    }

    /// A target `width` x `height` whose +0x10 object holds `word` at +0x10.
    fn target(e: &mut Engine, width: u32, height: u32, word: u32) -> u32 {
        let target = e.mem.alloc(0x14);
        let inner = e.mem.alloc(0x14);
        e.mem.set_u32(inner + 0x10, word);
        e.mem.set_u32(target + 0x10, inner);
        e.mem.set_u32(target + 8, width);
        e.mem.set_u32(target + 0xc, height);
        target
    }

    #[test]
    fn fn_00b6b180_reuses_a_large_enough_cube_target() {
        let (mut e, texture, buffer) = engine(RENDERED_CUBE_MAP_RTTI_012708E4);
        let kept = target(&mut e, 16, 16, 0);
        assert_eq!(e.call(0x00b6_b180, &args![texture, kept]).u32(), kept);
        let small = target(&mut e, 16, 8, 0);
        assert_eq!(
            e.call(0x00b6_b180, &args![texture, small]).u32(),
            0x0e20_0000
        );
        assert_eq!(
            e.call(0x00b6_b180, &args![texture, 0u32]).u32(),
            0x0e20_0000
        );
        assert_eq!(
            calls(&e, RENDER_TARGET_00A9FF00),
            vec![vec![buffer, 0x0e10_0000], vec![buffer, 0x0e10_0000]]
        );
        assert!(calls(&e, WIDTH).is_empty());
    }

    #[test]
    fn fn_00b6b180_checks_size_and_format_of_other_textures() {
        let (mut e, texture, buffer) = engine(0);
        let kept = target(&mut e, 64, 32, 5);
        assert_eq!(e.call(0x00b6_b180, &args![texture, kept]).u32(), kept);
        for (width, height, word) in [(63, 32, 5), (64, 31, 5), (64, 32, 6)] {
            let other = target(&mut e, width, height, word);
            assert_eq!(
                e.call(0x00b6_b180, &args![texture, other]).u32(),
                0x0e20_0000
            );
        }
        assert_eq!(calls(&e, RENDER_TARGET_00A9FF00).len(), 3);
        assert_eq!(
            calls(&e, RENDER_TARGET_00A9FF00)[0],
            vec![buffer, 0x0e10_0000]
        );
        // Width fails first: no height asked; the format check asks for the buffer twice.
        assert_eq!(calls(&e, HEIGHT).len(), 3);
        assert_eq!(calls(&e, BUFFER_OF).len(), 5);
    }
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00b6aed0_holds_and_counts_the_object() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let slot = e.mem.alloc(4);
        assert_eq!(e.call(0x00b6_aed0, &args![slot, 0x500u32]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x500);
        e.call(0x00b6_aed0, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        let counted: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == IMPORT_INTERLOCKED_INCREMENT)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(counted, vec![vec![0x504]]);
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const OBJECT_VTABLE: u32 = 0x6000;
    const DELETE: u32 = 0x7004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_a000, 0x1000);
        for no_op in [
            DESTRUCTOR_00B6A520,
            FREE_00AA1460,
            VECTOR_DESTRUCTOR_00EC5FCE,
            NI_OBJECT_DOWN_00A5D3D0,
            DELETE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        // The interlocked doubles move the count they are given and answer it.
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn deleting_destructors_free_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x150);
        assert_eq!(e.call(0x00b6_adb0, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, DESTRUCTOR_00B6A520), vec![vec![this]]);
        assert!(calls(&e, FREE_00AA1460).is_empty());
        e.call(0x00b6_adb0, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, 0x150]]);
        e.call(0x00b6_b160, &args![this, 3u32]);
        assert_eq!(calls(&e, FREE_00AA1460)[1], vec![this, 0x40]);
        assert_eq!(calls(&e, NI_OBJECT_DOWN_00A5D3D0), vec![vec![this]]);
    }

    #[test]
    fn fn_00b6aef0_lets_go_of_its_pointers() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let last = object(&mut e, 1);
        let shared = object(&mut e, 5);
        e.mem.set_u32(this + 8, last);
        e.mem.set_u32(this + 0x34, shared);
        e.mem.set_u32(this + 0x20, shared);
        e.mem.set_u32(this + 0x28, shared);
        e.call(0x00b6_aef0, &args![this]);
        assert_eq!(e.mem.u32(this), TABLE_010AE500);
        assert_eq!(calls(&e, DELETE), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x20), shared);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_00EC5FCE),
            vec![
                vec![this + 0x30, 4, 4, ELEMENT_DESTRUCTOR_0045CEC0],
                vec![this + 8, 4, 6, ELEMENT_DESTRUCTOR_00B66F20]
            ]
        );
        assert_eq!(calls(&e, NI_OBJECT_DOWN_00A5D3D0), vec![vec![this]]);
    }

    #[test]
    fn fn_00b6b050_pushes_up_to_ten_pointers() {
        let mut e = engine();
        let old = object(&mut e, 1);
        let new = object(&mut e, 0);
        e.mem.set_u32(POINTER_STACK_011FA024 + 8, old);
        e.mem.set_u32(POINTER_STACK_COUNT_011FA020, 2);
        e.call(0x00b6_b050, &args![new]);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(POINTER_STACK_011FA024 + 8), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(e.mem.u32(POINTER_STACK_COUNT_011FA020), 3);
        // The same object again: only the count moves.
        e.mem.set_u32(POINTER_STACK_COUNT_011FA020, 2);
        e.call(0x00b6_b050, &args![new]);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(e.mem.u32(POINTER_STACK_COUNT_011FA020), 3);
        e.mem.set_u32(POINTER_STACK_COUNT_011FA020, 10);
        e.call(0x00b6_b050, &args![new]);
        assert_eq!(e.mem.u32(POINTER_STACK_COUNT_011FA020), 10);
    }

    #[test]
    fn fn_00b6b100_answers_the_top_of_the_stack() {
        let mut e = engine();
        assert_eq!(e.call(0x00b6_b100, &args![]).u32(), 0);
        e.mem.set_u32(POINTER_STACK_COUNT_011FA020, 2);
        e.mem.set_u32(POINTER_STACK_011FA024 + 4, 0x55);
        assert_eq!(e.call(0x00b6_b100, &args![]).u32(), 0x55);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }
}

#[cfg(test)]
mod n013_tests {
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
    fn create_clone_makes_and_copies() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(WATER_SHADER_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(WATER_COPY_MEMBERS_00B6A6C0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00b6_ade0, &args![0x400u32, 0x77u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x150]]);
        assert_eq!(
            calls(&e, WATER_COPY_MEMBERS_00B6A6C0),
            vec![vec![0x400, clone, 0x77]]
        );
        // No block: the copy still runs, with a null clone.
        e.register(ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b6_ade0, &args![0x400u32, 0x77u32]).u32(), 0);
        assert_eq!(
            calls(&e, WATER_COPY_MEMBERS_00B6A6C0)[1],
            vec![0x400, 0, 0x77]
        );
        assert_eq!(calls(&e, WATER_SHADER_PROPERTY_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_object_makes_a_water_property() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(WATER_SHADER_PROPERTY_CONSTRUCT, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00b6_ae60, &args![]).u32();
        let block = calls(&e, WATER_SHADER_PROPERTY_CONSTRUCT)[0][0];
        assert_eq!(made, block + 1);
        e.register(ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b6_ae60, &args![]).u32(), 0);
    }

    #[test]
    fn pop_releases_the_top() {
        let mut e = Engine::new();
        e.map(0x011f_a000, 0x1000);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004]);
        e.register(0x0f00_0004, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_1000);
        e.mem.set_u32(object + 4, 1);
        e.set_global(POINTER_STACK_COUNT_011FA020, 2u32);
        e.set_global(POINTER_STACK_011FA024 + 4, object);
        e.call_log = Some(vec![]);
        e.call(0x00b6_b0b0, &args![]);
        assert_eq!(e.global::<u32>(POINTER_STACK_COUNT_011FA020), 1);
        assert_eq!(e.global::<u32>(POINTER_STACK_011FA024 + 4), 0);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![object]]);
        // An empty slot: only the count moves.
        e.call(0x00b6_b0b0, &args![]);
        assert_eq!(e.global::<u32>(POINTER_STACK_COUNT_011FA020), 0);
        // Empty stack: nothing.
        e.call(0x00b6_b0b0, &args![]);
        assert_eq!(e.global::<u32>(POINTER_STACK_COUNT_011FA020), 0);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
    }
}

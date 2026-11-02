//! `NiMain/nirenderedcubemap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ee8440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedCubeMap::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ee82b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x64 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_rendered_cube_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ee_82b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x64u32]);
    }
    this
}

/// `NiRenderedCubeMap::ms_RTTI` (the class's `NiRTTI` record).
const NI_RENDERED_CUBE_MAP_RTTI: u32 = 0x0127_08e4;
/// `NiTexture`'s and `NiRenderedCubeMap`'s vtables (their RTTI names the classes).
const NI_TEXTURE_VTABLE: u32 = 0x0109_b944;
const NI_RENDERED_CUBE_MAP_VTABLE: u32 = 0x0110_102c;
/// `00a5c200` and `00a5c240` (`thiscall`, no stack word; unit `niobjectnet.cpp`): the
/// object base constructor and destructor body.
const OBJECT_NET_CONSTRUCTOR: u32 = 0x00a5_c200;
const OBJECT_NET_DESTRUCTOR: u32 = 0x00a5_c240;
/// `00a5f7b0` (`thiscall`, no stack word; unplaced in the engine map): the step of the
/// `NiTexture` constructor after its fields are set.
const TEXTURE_CONSTRUCTOR_STEP: u32 = 0x00a5_f7b0;
/// `NiTexture::~NiTexture` (engine map, `00a5f800`; `thiscall`, no stack word).
const TEXTURE_DESTRUCTOR: u32 = 0x00a5_f800;
/// The import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a8f260` (`cdecl`, (width, height); unit `ni2dbuffer.cpp`): makes a 2D buffer.
const MAKE_2D_BUFFER: u32 = 0x00a8_f260;
/// The globals the new cube map's format bytes and word come from: `011f4ac0` (byte to
/// +0x41), `011ab3c8` (word to +0x44), `011ab3c4` (byte to +0x40).
const DEFAULT_BYTE_41: u32 = 0x011f_4ac0;
const DEFAULT_WORD_44: u32 = 0x011a_b3c8;
const DEFAULT_BYTE_40: u32 = 0x011a_b3c4;

// Translated from 00ee8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedCubeMap::GetWidth` (Xbox PDB): the width (+8) of the first face's buffer
/// (+0x4c).
pub fn ni_rendered_cube_map_get_width(e: &mut Engine, this: Ptr) -> u32 {
    let face = e.mem.u32(this.addr() + 0x4c);
    e.mem.u32(face.wrapping_add(8))
}

// Translated from 00ee8230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedCubeMap::GetHeight` (Xbox PDB): the height (+0xc) of the first face's buffer
/// (+0x4c).
pub fn ni_rendered_cube_map_get_height(e: &mut Engine, this: Ptr) -> u32 {
    let face = e.mem.u32(this.addr() + 0x4c);
    e.mem.u32(face.wrapping_add(0xc))
}

// Translated from 00ee8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `NiRenderedCubeMap`'s
/// constructor. The object base constructor, `NiTexture`'s vtable and its defaults (+0x18
/// 6, +0x1c 3, +0x20 2, +0x24 0), the texture constructor's step (`00a5f7b0`), +0x30 0, the
/// class's vtable, the six face buffers (+0x4c .. +0x60) and +0x48 cleared. Answers
/// `this`.
pub fn fn_00ee8240(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(OBJECT_NET_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_TEXTURE_VTABLE);
    e.mem.set_u32(base + 0x18, 6);
    e.mem.set_u32(base + 0x1c, 3);
    e.mem.set_u32(base + 0x20, 2);
    e.mem.set_u32(base + 0x24, 0);
    e.call(TEXTURE_CONSTRUCTOR_STEP, &args![this]);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base, NI_RENDERED_CUBE_MAP_VTABLE);
    for offset in (0x4cu32..=0x60).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x48, 0);
    this
}

// Translated from 00ee82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedCubeMap::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_rendered_cube_map_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RENDERED_CUBE_MAP_RTTI)
}

/// Drops one reference of the counted object at `object` (`InterlockedDecrement` of its
/// count at +4; at zero its virtual +4).
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
}

// Translated from 00ee82b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedCubeMap::~NiRenderedCubeMap` (Xbox PDB): releases the six face buffers (from
/// +0x60 down to +0x4c) and the object at +0x30, sets `NiTexture`'s vtable, deletes the
/// object at +0x24 when set (its virtual +0 with 1), runs `NiTexture::~NiTexture`
/// (`00a5f800`) and continues in the object base's destructor body (`00a5c240`, a `JMP`;
/// its result is passed on).
pub fn ni_rendered_cube_map_destructor(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    for face in (0..6u32).rev() {
        let object = e.mem.u32(base + 0x4c + 4 * face);
        release(e, object);
    }
    let object = e.mem.u32(base + 0x30);
    release(e, object);
    let renderer_data = e.mem.u32(base + 0x24);
    e.mem.set_u32(base, NI_TEXTURE_VTABLE);
    if renderer_data != 0 {
        e.vcall(renderer_data, 0, &args![1u32]);
    }
    e.call(TEXTURE_DESTRUCTOR, &args![this]);
    e.call(OBJECT_NET_DESTRUCTOR, &args![this]).u32()
}

// Translated from 00ee8330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (size, renderer, format)): by its body
/// `NiRenderedCubeMap::Create`. Null without a renderer, for a size of 0 or a size that is
/// not a power of two. Otherwise a new cube map (100 bytes from `00aa13e0`,
/// [`fn_00ee8240`]; null when the allocation fails, written through all the same) gets the
/// three format words at +0x18, six face buffers of `size` x `size` (`00a8f260`, each
/// assigned as a counted pointer), the defaults at +0x41, +0x44 and +0x40, and is offered to
/// the renderer (virtual +0x148). When the renderer refuses, the cube map deletes itself
/// (its virtual +0 with 1) and the answer is null.
pub fn fn_00ee8330(e: &mut Engine, size: u32, renderer: Ptr, format: Ptr) -> Ptr {
    if renderer.addr() == 0 || size == 0 || size & size.wrapping_sub(1) != 0 {
        return Ptr::new(0);
    }
    let block = e.call(ALLOCATE, &args![100u32]).u32();
    let cube = if block == 0 {
        0
    } else {
        fn_00ee8240(e, Ptr::new(block)).addr()
    };
    for i in 0..3 {
        let word = e.mem.u32(format.addr() + 4 * i);
        e.mem.set_u32(cube.wrapping_add(0x18 + 4 * i), word);
    }
    for face in 0..6u32 {
        let buffer = e.call(MAKE_2D_BUFFER, &args![size, size]).u32();
        let slot = cube.wrapping_add(0x4c + 4 * face);
        let old = e.mem.u32(slot);
        if old != buffer {
            release(e, old);
            e.mem.set_u32(slot, buffer);
            if buffer != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![buffer.wrapping_add(4)]);
            }
        }
    }
    let byte = e.mem.u8(DEFAULT_BYTE_41);
    e.mem.set_u8(cube.wrapping_add(0x41), byte);
    let word = e.mem.u32(DEFAULT_WORD_44);
    e.mem.set_u32(cube.wrapping_add(0x44), word);
    let byte = e.mem.u8(DEFAULT_BYTE_40);
    e.mem.set_u8(cube.wrapping_add(0x40), byte);
    if e.vcall(renderer.addr(), 0x148, &args![cube]).bool() {
        return Ptr::new(cube);
    }
    e.vcall(cube, 0, &args![1u32]);
    Ptr::new(0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee8440, ni_rendered_cube_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ee_8220, ni_rendered_cube_map_get_width(Ptr) -> u32),
        entry!(0x00ee_8230, ni_rendered_cube_map_get_height(Ptr) -> u32),
        entry!(0x00ee_8240, fn_00ee8240(Ptr) -> Ptr),
        entry!(0x00ee_82a0, ni_rendered_cube_map_get_rtti() -> Ptr),
        entry!(
            0x00ee_82b0,
            ni_rendered_cube_map_destructor(Ptr) -> u32
        ),
        entry!(0x00ee_8330, fn_00ee8330(u32, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_rendered_cube_map_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ee_82b0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ee8440, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ee8440, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x64]]);
        assert_eq!(calls_to(&e, 0x00ee_82b0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A counted object (count `count`) whose vtable holds `slots`.
    fn counted(e: &mut Engine, count: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x011a_b000, 0x1000);
        for double in [
            OBJECT_NET_CONSTRUCTOR,
            OBJECT_NET_DESTRUCTOR,
            TEXTURE_CONSTRUCTOR_STEP,
            TEXTURE_DESTRUCTOR,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn size_and_rtti() {
        let mut e = engine();
        let this = e.mem.alloc(0x64);
        let face = e.mem.alloc(0x10);
        e.mem.set_u32(face + 8, 256);
        e.mem.set_u32(face + 0xc, 128);
        e.mem.set_u32(this + 0x4c, face);
        assert_eq!(e.call(0x00ee_8220, &args![this]).u32(), 256);
        assert_eq!(e.call(0x00ee_8230, &args![this]).u32(), 128);
        assert_eq!(
            e.call(0x00ee_82a0, &args![]).u32(),
            NI_RENDERED_CUBE_MAP_RTTI
        );
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = engine();
        let this = e.mem.alloc(0x64);
        for offset in (0x18..0x64).step_by(4) {
            e.mem.set_u32(this + offset, 0xcdcd_cdcd);
        }
        e.register(TEXTURE_CONSTRUCTOR_STEP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_TEXTURE_VTABLE);
            Ret::default()
        });
        assert_eq!(e.call(0x00ee_8240, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_RENDERED_CUBE_MAP_VTABLE);
        let words: Vec<u32> = [0x18, 0x1c, 0x20, 0x24, 0x30]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![6, 3, 2, 0, 0]);
        assert_eq!(e.mem.bytes(this + 0x48, 0x1c), vec![0; 0x1c]);
        // Words the constructor does not touch keep their values.
        assert_eq!(e.mem.u32(this + 0x28), 0xcdcd_cdcd);
    }

    #[test]
    fn destructor_releases_faces_and_deletes_the_renderer_data() {
        let mut e = engine();
        e.register(0x0300_1000, |_, _| Ret::default());
        e.register(0x0300_2000, |_, _| Ret::default());
        let this = e.mem.alloc(0x64);
        let mut faces = vec![];
        for face in 0..6 {
            let object = counted(&mut e, 1, &[(4, 0x0300_1000)]);
            e.mem.set_u32(this + 0x4c + 4 * face, object);
            faces.push(object);
        }
        let kept = counted(&mut e, 2, &[(4, 0x0300_1000)]);
        e.mem.set_u32(this + 0x30, kept);
        let data = counted(&mut e, 1, &[(0, 0x0300_2000)]);
        e.mem.set_u32(this + 0x24, data);
        e.register(OBJECT_NET_DESTRUCTOR, |_, _| 0x77u32.into_ret());
        assert_eq!(e.call(0x00ee_82b0, &args![this]).u32(), 0x77);
        let freed: Vec<u32> = calls_to(&e, 0x0300_1000).iter().map(|c| c[0]).collect();
        faces.reverse();
        assert_eq!(freed, faces);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(this), NI_TEXTURE_VTABLE);
        assert_eq!(calls_to(&e, TEXTURE_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_refuses_bad_sizes_and_a_missing_renderer() {
        let mut e = engine();
        let format = e.mem.alloc(12);
        for (size, renderer) in [(64u32, 0u32), (0, 0x5000), (48, 0x5000)] {
            assert_eq!(e.call(0x00ee_8330, &args![size, renderer, format]).u32(), 0);
        }
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn create_builds_six_faces_and_asks_the_renderer() {
        let mut e = engine();
        e.register_double(MAKE_2D_BUFFER, {
            move |e, _| {
                let buffer = e.mem.alloc(0x10);
                buffer.into_ret()
            }
        });
        e.mem.set_u8(DEFAULT_BYTE_41, 7);
        e.mem.set_u32(DEFAULT_WORD_44, 0x1234);
        e.mem.set_u8(DEFAULT_BYTE_40, 9);
        let format = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(format + 4 * i, 0x100 + i);
        }
        e.register(0x0300_3000, |_, _| 1u32.into_ret());
        let renderer = counted(&mut e, 1, &[(0x148, 0x0300_3000)]);
        let cube = e.call(0x00ee_8330, &args![64u32, renderer, format]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![100]]);
        assert_eq!(calls_to(&e, MAKE_2D_BUFFER), vec![vec![64, 64]; 6]);
        assert_eq!(e.mem.u32(cube + 0x1c), 0x101);
        for face in 0..6 {
            let buffer = e.mem.u32(cube + 0x4c + 4 * face);
            assert_eq!(e.mem.u32(buffer + 4), 1);
        }
        assert_eq!(e.mem.u8(cube + 0x41), 7);
        assert_eq!(e.mem.u32(cube + 0x44), 0x1234);
        assert_eq!(e.mem.u8(cube + 0x40), 9);
        assert_eq!(calls_to(&e, 0x0300_3000), vec![vec![renderer, cube]]);
        // A refusing renderer: the cube map deletes itself and the answer is null.
        e.register(0x0300_3000, |_, _| 0x100u32.into_ret());
        e.register(0x0300_4000, |_, _| Ret::default());
        // The constructor sets the class's vtable; its slot 0 is a double here.
        e.put_vtable(NI_RENDERED_CUBE_MAP_VTABLE, &[0x0300_4000]);
        let refused = e.call(0x00ee_8330, &args![64u32, renderer, format]).u32();
        assert_eq!(refused, 0);
        assert_eq!(calls_to(&e, 0x0300_4000).len(), 1);
        assert_eq!(calls_to(&e, 0x0300_4000)[0][1], 1);
    }
}

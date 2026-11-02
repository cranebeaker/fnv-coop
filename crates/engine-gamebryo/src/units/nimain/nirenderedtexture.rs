//! `NiMain/nirenderedtexture.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderedTexture::ms_RTTI` (the class's `NiRTTI` record).
const NI_RENDERED_TEXTURE_RTTI: u32 = 0x011f_4ac4;

// Translated from 00a7fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedTexture::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_rendered_texture_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RENDERED_TEXTURE_RTTI)
}

layout! {
    /// `NiRenderedTexture` (Xbox PDB, 0x48 bytes; the PC code uses the same offsets).
    pub struct NiRenderedTexture: 0x48 {
        /// `NiTexture::m_kFormatPrefs` (Xbox PDB): three words.
        0x18 m_kFormatPrefs: u32,
        /// `NiTexture::m_pkRendererData` (Xbox PDB).
        0x24 m_pkRendererData: u32,
        /// `m_spBuffer` (Xbox PDB): the `Ni2DBuffer`, one reference held.
        0x30 m_spBuffer: u32,
        /// `uiAliasOffset` (Xbox PDB).
        0x34 uiAliasOffset: u32,
        /// `pAliasTexture` (Xbox PDB).
        0x38 pAliasTexture: u32,
        /// `eMSAA` (Xbox PDB).
        0x3C eMSAA: u32,
        /// `bRenderedTexture` (Xbox PDB).
        0x40 bRenderedTexture: u8,
        /// `bForceD3DFormat` (Xbox PDB).
        0x41 bForceD3DFormat: u8,
        /// `iForcedD3DFormat` (Xbox PDB).
        0x44 iForcedD3DFormat: u32,
    }
}

/// The `NiTexture` and `NiRenderedTexture` vtables (RTTI `NiTexture`, `NiRenderedTexture`).
const TEXTURE_VTABLE: u32 = 0x0109_b944;
const RENDERED_TEXTURE_VTABLE: u32 = 0x0109_df8c;
/// `00a5c200` / `00a5c240` (`thiscall`, unit `niobjectnet.cpp`): the `NiObjectNET`
/// constructor and destructor bodies.
const OBJECT_NET_CONSTRUCTOR: u32 = 0x00a5_c200;
const OBJECT_NET_DESTRUCTOR: u32 = 0x00a5_c240;
/// `00a5f7b0` (`thiscall`, no unit in the engine map): run by the constructor after the
/// `NiTexture` fields are set.
const TEXTURE_00A5F7B0: u32 = 0x00a5_f7b0;
/// `NiTexture::~NiTexture` (`00a5f800`, `thiscall`).
const TEXTURE_DESTRUCTOR: u32 = 0x00a5_f800;
/// `00a8f260` (`cdecl`, `(width, height)`, unit `ni2dbuffer.cpp`): the buffer the texture
/// renders to.
const CREATE_2D_BUFFER: u32 = 0x00a8_f260;
/// `00aa13e0` (`cdecl`, one word): allocates; `00aa1460` (`cdecl`, `(block, size)`): the
/// sized delete.
const ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The globals `00a7fc00` copies into a new texture: a byte for `bForceD3DFormat`
/// (`011f4ac0`), a word for `iForcedD3DFormat` (`011ab3c8`) and a byte for
/// `bRenderedTexture` (`011ab3c4`).
const FORCE_D3D_FORMAT: u32 = 0x011f_4ac0;
const FORCED_D3D_FORMAT: u32 = 0x011a_b3c8;
const RENDERED_TEXTURE_FLAG: u32 = 0x011a_b3c4;

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

// Translated from 00a7fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtables the `NiRenderedTexture` constructor
/// (`thiscall`): the `NiObjectNET` constructor (`00a5c200`), the `NiTexture` vtable and
/// format preferences 6, 3, 2, no renderer data, then `00a5f7b0`, the `NiRenderedTexture`
/// vtable and no buffer. Answers `this`.
pub fn fn_00a7fb80(e: &mut Engine, this: Ptr<NiRenderedTexture>) -> Ptr<NiRenderedTexture> {
    let at = this.addr();
    e.call(OBJECT_NET_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(at, TEXTURE_VTABLE);
    e.mem.set_u32(at + 0x18, 6);
    e.mem.set_u32(at + 0x1c, 3);
    e.mem.set_u32(at + 0x20, 2);
    e.set(this, NiRenderedTexture::m_pkRendererData, 0);
    e.call(TEXTURE_00A5F7B0, &args![this]);
    e.mem.set_u32(at, RENDERED_TEXTURE_VTABLE);
    e.set(this, NiRenderedTexture::m_spBuffer, 0);
    this
}

// Translated from 00a7fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedTexture::GetWidth` (Xbox PDB): the word at `+8` of the buffer, 0 without one.
pub fn ni_rendered_texture_get_width(e: &mut Engine, this: Ptr<NiRenderedTexture>) -> u32 {
    let buffer = e.get(this, NiRenderedTexture::m_spBuffer);
    if buffer == 0 {
        0
    } else {
        // Ni2DBuffer width (+0x08)
        e.mem.u32(buffer + 8)
    }
}

// Translated from 00a7fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedTexture::GetHeight` (Xbox PDB): the word at `+0xc` of the buffer, 0 without
/// one.
pub fn ni_rendered_texture_get_height(e: &mut Engine, this: Ptr<NiRenderedTexture>) -> u32 {
    let buffer = e.get(this, NiRenderedTexture::m_spBuffer);
    if buffer == 0 {
        0
    } else {
        // Ni2DBuffer height (+0x0C)
        e.mem.u32(buffer + 0xc)
    }
}

// Translated from 00a7fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nirenderedtexture.cpp`; a `cdecl` factory of eight
/// words, the first never read): null without a renderer. When `width` or `height` is 0
/// or not a power of two, the renderer's slot `+0xa0` must answer bit 3, or (asked again)
/// bit 2, or the answer is null. Then a 0x48-byte block (`00aa13e0`) is constructed
/// (`00a7fb80`; a failed allocation leaves a null object, which the exe still writes to),
/// gets the three words of `format` as its format preferences, `alias_texture` at `+0x38`
/// and `alias_offset` at `+0x34`, the buffer `00a8f260(width, height)` (counted, any old
/// one released), and the globals `011f4ac0` (byte, `+0x41`), `011ab3c8` (`+0x44`) and
/// `011ab3c4` (byte, `+0x40`). The renderer's slot `+0x140` `(texture, msaa)` decides:
/// false deletes the texture (its slot 0 with 1) and answers null; true stores `msaa` at
/// `+0x3c`, clears `+0x38` and answers the texture.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a7fc00(
    e: &mut Engine,
    _unused_0: u32,
    width: u32,
    height: u32,
    renderer: Ptr,
    format: Ptr,
    msaa: u32,
    alias_texture: u32,
    alias_offset: u32,
) -> Ptr<NiRenderedTexture> {
    if renderer.is_null() {
        return Ptr::NULL;
    }
    let power_of_two = |n: u32| n != 0 && n & n.wrapping_sub(1) == 0;
    if !(power_of_two(width) && power_of_two(height)) {
        let flags = e.vcall(renderer.addr(), 0xa0, &[]).u32();
        if flags & 8 == 0 {
            let flags = e.vcall(renderer.addr(), 0xa0, &[]).u32();
            if flags & 4 == 0 {
                return Ptr::NULL;
            }
        }
    }
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    let texture: Ptr<NiRenderedTexture> = if block == 0 {
        Ptr::NULL
    } else {
        fn_00a7fb80(e, Ptr::new(block))
    };
    let at = texture.addr();
    for word in 0..3 {
        let value = e.mem.u32(format.addr() + 4 * word);
        e.mem.set_u32(at.wrapping_add(0x18 + 4 * word), value);
    }
    e.set(texture, NiRenderedTexture::pAliasTexture, alias_texture);
    e.set(texture, NiRenderedTexture::uiAliasOffset, alias_offset);
    let buffer = e.call(CREATE_2D_BUFFER, &args![width, height]).u32();
    let old = e.get(texture, NiRenderedTexture::m_spBuffer);
    if old != buffer {
        if old != 0 {
            release(e, old);
        }
        e.set(texture, NiRenderedTexture::m_spBuffer, buffer);
        if buffer != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![buffer + 4]);
        }
    }
    let force = e.mem.u8(FORCE_D3D_FORMAT);
    e.set(texture, NiRenderedTexture::bForceD3DFormat, force);
    let forced = e.mem.u32(FORCED_D3D_FORMAT);
    e.set(texture, NiRenderedTexture::iForcedD3DFormat, forced);
    let rendered = e.mem.u8(RENDERED_TEXTURE_FLAG);
    e.set(texture, NiRenderedTexture::bRenderedTexture, rendered);
    let created = e.vcall(renderer.addr(), 0x140, &args![texture, msaa]).u8();
    if created == 0 {
        e.vcall(at, 0, &args![1u32]);
        return Ptr::NULL;
    }
    e.set(texture, NiRenderedTexture::eMSAA, msaa);
    e.set(texture, NiRenderedTexture::pAliasTexture, 0);
    texture
}

// Translated from 00a7fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderedTexture::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// releases the buffer, sets the `NiTexture` vtable, deletes the renderer data through its
/// slot 0 (with 1) when there is one, runs `NiTexture::~NiTexture` (`00a5f800`) and the
/// `NiObjectNET` destructor body (`00a5c240`), then the sized delete of 0x48 bytes when
/// bit 0 of `flags` is set. Answers `this`.
pub fn ni_rendered_texture_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiRenderedTexture>,
    flags: u32,
) -> Ptr<NiRenderedTexture> {
    let buffer = e.get(this, NiRenderedTexture::m_spBuffer);
    if buffer != 0 {
        release(e, buffer);
    }
    let renderer_data = e.get(this, NiRenderedTexture::m_pkRendererData);
    e.mem.set_u32(this.addr(), TEXTURE_VTABLE);
    if renderer_data != 0 {
        e.vcall(renderer_data, 0, &args![1u32]);
    }
    e.call(TEXTURE_DESTRUCTOR, &args![this]);
    e.call(OBJECT_NET_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7fbd0, ni_rendered_texture_get_rtti() -> Ptr),
        entry!(
            0x00a7_fb80,
            fn_00a7fb80(Ptr<NiRenderedTexture>) -> Ptr<NiRenderedTexture>
        ),
        entry!(
            0x00a7_fbe0,
            ni_rendered_texture_get_width(Ptr<NiRenderedTexture>) -> u32
        ),
        entry!(
            0x00a7_fbf0,
            ni_rendered_texture_get_height(Ptr<NiRenderedTexture>) -> u32
        ),
        entry!(
            0x00a7_fc00,
            fn_00a7fc00(u32, u32, u32, Ptr, Ptr, u32, u32, u32) -> Ptr<NiRenderedTexture>
        ),
        entry!(
            0x00a7_fd30,
            ni_rendered_texture_scalar_deleting_destructor(
                Ptr<NiRenderedTexture>,
                u32,
            ) -> Ptr<NiRenderedTexture>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_rendered_texture_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7fbd0, &args![]).u32(), NI_RENDERED_TEXTURE_RTTI);
    }
}

#[cfg(test)]
mod rendered_texture_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    const RENDERER_VTABLE: u32 = 0x0300_0000;
    const RENDERER_FLAGS: u32 = 0x0300_1000;
    const RENDERER_CREATE: u32 = 0x0300_2000;
    const DELETING_DESTRUCTOR: u32 = 0x0300_3000;
    const BUFFER: u32 = 0x0300_4000;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x011a_b000, 0x1000);
        e.register(OBJECT_NET_CONSTRUCTOR, |_, _| Ret::default());
        e.register(OBJECT_NET_DESTRUCTOR, |_, _| Ret::default());
        e.register(TEXTURE_00A5F7B0, |_, _| Ret::default());
        e.register(TEXTURE_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn construction_sets_both_vtables_and_the_format_preferences() {
        let mut e = engine();
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcc; 0x48]);
        assert_eq!(e.call(0x00a7_fb80, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_NET_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, TEXTURE_00A5F7B0), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), RENDERED_TEXTURE_VTABLE);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 0x18 + 4 * i)).collect();
        assert_eq!(words, vec![6, 3, 2, 0]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn width_and_height_come_from_the_buffer() {
        let mut e = engine();
        let this = e.mem.alloc(0x48);
        assert_eq!(e.call(0x00a7_fbe0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00a7_fbf0, &args![this]).u32(), 0);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer + 8, 640);
        e.mem.set_u32(buffer + 0xc, 480);
        e.mem.set_u32(this + 0x30, buffer);
        assert_eq!(e.call(0x00a7_fbe0, &args![this]).u32(), 640);
        assert_eq!(e.call(0x00a7_fbf0, &args![this]).u32(), 480);
    }

    /// Runs the factory with a renderer whose flags slot answers `flags` and whose create
    /// slot answers `created`; answers the engine, the result and the renderer.
    fn create(width: u32, height: u32, flags: u32, created: bool) -> (Engine, u32, u32) {
        let mut e = engine();
        e.mem.set_u8(FORCE_D3D_FORMAT, 1);
        e.mem.set_u32(FORCED_D3D_FORMAT, 0x15);
        e.mem.set_u8(RENDERED_TEXTURE_FLAG, 1);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, BUFFER);
        e.register_double(CREATE_2D_BUFFER, move |_, _| buffer.into_ret());
        e.register_double(RENDERER_FLAGS, move |_, _| flags.into_ret());
        e.register_double(RENDERER_CREATE, move |_, _| u32::from(created).into_ret());
        e.register(DELETING_DESTRUCTOR, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x51];
        slots[0xa0 / 4] = RENDERER_FLAGS;
        slots[0x140 / 4] = RENDERER_CREATE;
        e.put_vtable(RENDERER_VTABLE, &slots);
        e.put_vtable(RENDERED_TEXTURE_VTABLE, &[DELETING_DESTRUCTOR]);
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        let format = e.mem.alloc(0xc);
        e.mem.set_u32(format, 7);
        e.mem.set_u32(format + 4, 8);
        e.mem.set_u32(format + 8, 9);
        let texture = e
            .call(
                0x00a7_fc00,
                &args![0u32, width, height, renderer, format, 4u32, 0x55u32, 0x66u32],
            )
            .u32();
        (e, texture, renderer)
    }

    #[test]
    fn a_power_of_two_texture_is_created_without_asking_the_flags() {
        let (e, texture, renderer) = create(256, 128, 0, true);
        assert_ne!(texture, 0);
        assert!(calls_to(&e, RENDERER_FLAGS).is_empty());
        assert_eq!(calls_to(&e, CREATE_2D_BUFFER), vec![vec![256, 128]]);
        assert_eq!(
            calls_to(&e, RENDERER_CREATE),
            vec![vec![renderer, texture, 4]]
        );
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(texture + 0x18 + 4 * i)).collect();
        assert_eq!(words, vec![7, 8, 9]);
        let buffer = e.mem.u32(texture + 0x30);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        assert_eq!(e.mem.u32(texture + 0x34), 0x66);
        assert_eq!(e.mem.u32(texture + 0x38), 0);
        assert_eq!(e.mem.u32(texture + 0x3c), 4);
        assert_eq!(e.mem.u8(texture + 0x40), 1);
        assert_eq!(e.mem.u8(texture + 0x41), 1);
        assert_eq!(e.mem.u32(texture + 0x44), 0x15);
    }

    #[test]
    fn other_sizes_need_a_renderer_flag() {
        let (e, texture, _) = create(300, 128, 0, true);
        assert_eq!(texture, 0);
        assert_eq!(calls_to(&e, RENDERER_FLAGS).len(), 2);
        let (e, texture, _) = create(256, 0, 8, true);
        assert_ne!(texture, 0);
        assert_eq!(calls_to(&e, RENDERER_FLAGS).len(), 1);
        let (_, texture, _) = create(3, 5, 4, true);
        assert_ne!(texture, 0);
    }

    #[test]
    fn a_refused_texture_is_deleted() {
        let (e, texture, _) = create(64, 64, 0, false);
        assert_eq!(texture, 0);
        let deleted = calls_to(&e, DELETING_DESTRUCTOR);
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0][1], 1);
    }

    #[test]
    fn no_renderer_answers_null() {
        let mut e = engine();
        let answer = e
            .call(
                0x00a7_fc00,
                &args![0u32, 64u32, 64u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(answer, 0);
    }

    #[test]
    fn the_deleting_destructor_releases_the_buffer_and_the_renderer_data() {
        let mut e = engine();
        let buffer_vtable = 0x0300_5000;
        let buffer_delete = 0x0300_6000;
        let data_vtable = 0x0300_7000;
        let data_delete = 0x0300_8000;
        e.register(buffer_delete, |_, _| Ret::default());
        e.register(data_delete, |_, _| Ret::default());
        e.put_vtable(buffer_vtable, &[0, buffer_delete]);
        e.put_vtable(data_vtable, &[data_delete]);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, buffer_vtable);
        e.mem.set_u32(buffer + 4, 1);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, data_vtable);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x30, buffer);
        e.mem.set_u32(this + 0x24, data);
        assert_eq!(e.call(0x00a7_fd30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, buffer_delete), vec![vec![buffer]]);
        assert_eq!(calls_to(&e, data_delete), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(this), TEXTURE_VTABLE);
        assert_eq!(calls_to(&e, TEXTURE_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, OBJECT_NET_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x48]]);
        // Without buffer and renderer data, and without bit 0.
        e.mem.set_u32(this + 0x30, 0);
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00a7_fd30, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }
}

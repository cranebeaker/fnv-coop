//! `BSShader/imagespacetexture.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ImageSpaceTexture` (Xbox PDB), 0x10 bytes; the PC code reads the same offsets.
    pub struct ImageSpaceTexture: 0x10 {
        /// `bForceAniso` (Xbox PDB).
        0x00 bForceAniso: u8,
        /// `bIsRenderedTexture` (Xbox PDB).
        0x01 bIsRenderedTexture: u8,
        /// `bIsTextureBorrowed` (Xbox PDB).
        0x02 bIsTextureBorrowed: u8,
        /// `pTexture` (Xbox PDB): a `NiPointer<NiObject>`.
        0x04 pTexture: u32,
        /// `iFilterMode` (Xbox PDB).
        0x08 iFilterMode: u32,
        /// `iClampMode` (Xbox PDB).
        0x0C iClampMode: u32,
    }
}

/// `ImageSpaceTexture::ReturnRenderedTexture` (engine map, `00ba39a0`, `thiscall`, no stack
/// words).
const RETURN_RENDERED_TEXTURE: u32 = 0x00ba_39a0;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Releases one reference of the `NiRefObject` at `object` (count at +4): its vtable slot +4
/// when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00ba3a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3a00 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2a7fb, state variable) is not translated
/// `ImageSpaceTexture::~ImageSpaceTexture` (Xbox PDB), `thiscall`: a borrowed texture
/// (`bIsTextureBorrowed`) is returned first (`ReturnRenderedTexture`); then the texture
/// pointer is released and cleared, and the `NiPointer` member's own destructor releases
/// what the member holds after that (it re-reads the member, which the code just cleared).
pub fn image_space_texture_destructor(e: &mut Engine, this: Ptr<ImageSpaceTexture>) {
    if e.get(this, ImageSpaceTexture::bIsTextureBorrowed) != 0 {
        e.call(RETURN_RENDERED_TEXTURE, &args![this]);
    }
    let texture = e.get(this, ImageSpaceTexture::pTexture);
    if texture != 0 {
        release(e, texture);
        e.set(this, ImageSpaceTexture::pTexture, 0);
    }
    let texture = e.get(this, ImageSpaceTexture::pTexture);
    if texture != 0 {
        release(e, texture);
    }
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, ni_pointer_clear, release_reference, IMPORT_INTERLOCKED_INCREMENT,
};

/// `BSRenderedTexture::GetTexture` (engine map, `004bc320`, `thiscall` on the rendered
/// texture, one word; these callers pass 0).
const RENDERED_TEXTURE_GET_TEXTURE_004BC320: u32 = 0x004b_c320;
/// The texture manager singleton (`011f91a8`), the renderer (`011f9508`) and the manager's
/// borrow `00b6e110` (unnamed; `thiscall` on the manager: the renderer, the kind word, then
/// three words; this caller passes 0, 0, 0).
const TEXTURE_MANAGER_011F91A8: u32 = 0x011f_91a8;
const RENDERER_011F9508: u32 = 0x011f_9508;
const BORROW_RENDERED_00B6E110: u32 = 0x00b6_e110;
/// `BSTextureManager::ReturnRenderedTexture` (engine map, `00b6da10`, `thiscall` on the
/// manager, one word).
const TEXTURE_MANAGER_RETURN_00B6DA10: u32 = 0x00b6_da10;
/// The texture stage's mode setter `00e7de90` (unnamed; `thiscall` on the stage, one word:
/// this caller passes `iClampMode`) and `NiD3DTextureStage::SetFilterMode` (engine map,
/// `00e7def0`, `thiscall`, one word).
const STAGE_SET_MODE_00E7DE90: u32 = 0x00e7_de90;
const STAGE_SET_FILTER_MODE_00E7DEF0: u32 = 0x00e7_def0;

/// The texture the member holds as the shaders see it: through
/// `BSRenderedTexture::GetTexture(0)` when it is a rendered texture, else the member itself.
fn texture_of(e: &mut Engine, this: Ptr<ImageSpaceTexture>) -> u32 {
    if e.get(this, ImageSpaceTexture::bIsRenderedTexture) != 0 {
        let rendered = e.get(this, ImageSpaceTexture::pTexture);
        e.call(
            RENDERED_TEXTURE_GET_TEXTURE_004BC320,
            &args![rendered, 0u32],
        )
        .u32()
    } else {
        e.get(this, ImageSpaceTexture::pTexture)
    }
}

// Translated from 00ba3770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::GetRenderedTexture` (Xbox PDB), `thiscall`, no stack words: `pTexture`
/// when `bIsRenderedTexture` is set, else null.
pub fn image_space_texture_get_rendered_texture(
    e: &mut Engine,
    this: Ptr<ImageSpaceTexture>,
) -> u32 {
    if e.get(this, ImageSpaceTexture::bIsRenderedTexture) != 0 {
        e.get(this, ImageSpaceTexture::pTexture)
    } else {
        0
    }
}

// Translated from 00ba3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::GetTexture` (Xbox PDB), `thiscall`, no stack words: for a rendered
/// texture `BSRenderedTexture::GetTexture(0)` on it (`004bc320`), else `pTexture`.
pub fn image_space_texture_get_texture(e: &mut Engine, this: Ptr<ImageSpaceTexture>) -> u32 {
    texture_of(e, this)
}

// Translated from 00ba37a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::SetTexture` (Xbox PDB), `thiscall`, one word: the `NiPointer` member
/// takes `texture` (the old object released, the new one counted) and `bIsRenderedTexture`
/// is set.
pub fn image_space_texture_set_texture(e: &mut Engine, this: Ptr<ImageSpaceTexture>, texture: u32) {
    ni_pointer_assign(e, this.addr() + 4, texture);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, 1);
}

// Translated from 00ba37f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacetexture.cpp`), `thiscall`, one word: the body of
/// `SetTexture` (`00ba37a0`) for a texture that is not rendered: the `NiPointer` member takes
/// `texture` and `bIsRenderedTexture` is cleared.
pub fn fn_00ba37f0(e: &mut Engine, this: Ptr<ImageSpaceTexture>, texture: u32) {
    ni_pointer_assign(e, this.addr() + 4, texture);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
}

// Translated from 00ba3840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::BorrowRenderedTexture` (Xbox PDB), `thiscall`, one word: borrows a
/// rendered texture from the texture manager (`00b6e110` on `011f91a8` with the renderer
/// `011f9508`, `kind`, 0, 0, 0), keeps it in the `NiPointer` member and sets
/// `bIsRenderedTexture` and `bIsTextureBorrowed`.
pub fn image_space_texture_borrow_rendered_texture(
    e: &mut Engine,
    this: Ptr<ImageSpaceTexture>,
    kind: u32,
) {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let manager = e.global::<u32>(TEXTURE_MANAGER_011F91A8);
    let texture = e
        .call(
            BORROW_RENDERED_00B6E110,
            &args![manager, renderer, kind, 0u32, 0u32, 0u32],
        )
        .u32();
    ni_pointer_assign(e, this.addr() + 4, texture);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, 1);
    e.set(this, ImageSpaceTexture::bIsTextureBorrowed, 1);
}

// Translated from 00ba38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::ClearTexture` (Xbox PDB), `thiscall`, no stack words: the `NiPointer`
/// member released and cleared, then `bIsRenderedTexture`, `iFilterMode` and `iClampMode`
/// cleared.
pub fn image_space_texture_clear_texture(e: &mut Engine, this: Ptr<ImageSpaceTexture>) {
    ni_pointer_clear(e, this.addr() + 4);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
    e.set(this, ImageSpaceTexture::iFilterMode, 0);
    e.set(this, ImageSpaceTexture::iClampMode, 0);
}

// Translated from 00ba38f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacetexture.cpp`), `thiscall`, one word (a texture
/// stage): sets the stage's mode through `00e7de90` with `iClampMode` and its filter mode
/// (`NiD3DTextureStage::SetFilterMode`, `iFilterMode`), then stores the texture (as
/// `GetTexture` answers it, without counting a reference) in the stage's word at +8.
pub fn fn_00ba38f0(e: &mut Engine, this: Ptr<ImageSpaceTexture>, stage: u32) {
    let clamp = e.get(this, ImageSpaceTexture::iClampMode);
    e.call(STAGE_SET_MODE_00E7DE90, &args![stage, clamp]);
    let filter = e.get(this, ImageSpaceTexture::iFilterMode);
    e.call(STAGE_SET_FILTER_MODE_00E7DEF0, &args![stage, filter]);
    let texture = texture_of(e, this);
    e.mem.set_u32(stage + 8, texture);
}

// Translated from 00ba3940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::operator=` (Xbox PDB), `thiscall`, one word (the source): copies
/// `bIsRenderedTexture` and `bIsTextureBorrowed`, the `NiPointer` member (when the two differ:
/// the old object released, the source's member read again after the release, stored and
/// counted) and `iFilterMode` (not `iClampMode`), then clears the source's
/// `bIsTextureBorrowed` (the borrow moves to `this`). Answers `this`.
pub fn image_space_texture_assign(
    e: &mut Engine,
    this: Ptr<ImageSpaceTexture>,
    source: Ptr<ImageSpaceTexture>,
) -> Ptr<ImageSpaceTexture> {
    let rendered = e.get(source, ImageSpaceTexture::bIsRenderedTexture);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, rendered);
    let borrowed = e.get(source, ImageSpaceTexture::bIsTextureBorrowed);
    e.set(this, ImageSpaceTexture::bIsTextureBorrowed, borrowed);
    let old = e.get(this, ImageSpaceTexture::pTexture);
    if old != e.get(source, ImageSpaceTexture::pTexture) {
        if old != 0 {
            release_reference(e, old);
        }
        let texture = e.get(source, ImageSpaceTexture::pTexture);
        e.set(this, ImageSpaceTexture::pTexture, texture);
        if texture != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
        }
    }
    let filter = e.get(source, ImageSpaceTexture::iFilterMode);
    e.set(this, ImageSpaceTexture::iFilterMode, filter);
    e.set(source, ImageSpaceTexture::bIsTextureBorrowed, 0);
    this
}

// Translated from 00ba39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceTexture::ReturnRenderedTexture` (Xbox PDB), `thiscall`, no stack words: nothing
/// unless `bIsTextureBorrowed`; else that flag is cleared, the rendered texture (null when not
/// rendered) handed back to the texture manager (`BSTextureManager::ReturnRenderedTexture` on
/// `011f91a8`), the `NiPointer` member released and cleared, and `bIsRenderedTexture`,
/// `iFilterMode` and `iClampMode` cleared.
pub fn image_space_texture_return_rendered_texture(e: &mut Engine, this: Ptr<ImageSpaceTexture>) {
    if e.get(this, ImageSpaceTexture::bIsTextureBorrowed) == 0 {
        return;
    }
    e.set(this, ImageSpaceTexture::bIsTextureBorrowed, 0);
    let rendered = image_space_texture_get_rendered_texture(e, this);
    let manager = e.global::<u32>(TEXTURE_MANAGER_011F91A8);
    e.call(TEXTURE_MANAGER_RETURN_00B6DA10, &args![manager, rendered]);
    ni_pointer_clear(e, this.addr() + 4);
    e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
    e.set(this, ImageSpaceTexture::iFilterMode, 0);
    e.set(this, ImageSpaceTexture::iClampMode, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_3a00,
            image_space_texture_destructor(Ptr<ImageSpaceTexture>)
        ),
        entry!(
            0x00ba_3770,
            image_space_texture_get_rendered_texture(Ptr<ImageSpaceTexture>) -> u32
        ),
        entry!(
            0x00ba_3780,
            image_space_texture_get_texture(Ptr<ImageSpaceTexture>) -> u32
        ),
        entry!(
            0x00ba_37a0,
            image_space_texture_set_texture(Ptr<ImageSpaceTexture>, u32)
        ),
        entry!(0x00ba_37f0, fn_00ba37f0(Ptr<ImageSpaceTexture>, u32)),
        entry!(
            0x00ba_3840,
            image_space_texture_borrow_rendered_texture(Ptr<ImageSpaceTexture>, u32)
        ),
        entry!(
            0x00ba_38b0,
            image_space_texture_clear_texture(Ptr<ImageSpaceTexture>)
        ),
        entry!(0x00ba_38f0, fn_00ba38f0(Ptr<ImageSpaceTexture>, u32)),
        entry!(
            0x00ba_3940,
            image_space_texture_assign(
                Ptr<ImageSpaceTexture>,
                Ptr<ImageSpaceTexture>,
            ) -> Ptr<ImageSpaceTexture>
        ),
        entry!(
            0x00ba_39a0,
            image_space_texture_return_rendered_texture(Ptr<ImageSpaceTexture>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_2000;
    const DELETE_THIS: u32 = 0x0f00_2004;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig(borrowed: u8, refs: u32) -> (Engine, Ptr<ImageSpaceTexture>, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE_THIS]);
        e.register(DELETE_THIS, |_, _| Ret::default());
        let texture = e.mem.alloc(8);
        e.mem.set_u32(texture, VTABLE);
        e.mem.set_u32(texture + 4, refs);
        let this = e.new_object::<ImageSpaceTexture>();
        e.set(this, ImageSpaceTexture::bIsTextureBorrowed, borrowed);
        e.set(this, ImageSpaceTexture::pTexture, texture);
        e.call_log = Some(vec![]);
        (e, this, texture)
    }

    #[test]
    fn fn_00ba3a00_returns_a_borrowed_texture_and_releases_the_pointer() {
        let (mut e, this, texture) = rig(1, 1);
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![texture]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![texture + 4]]);
    }

    #[test]
    fn fn_00ba3a00_keeps_a_held_texture_and_skips_an_empty_pointer() {
        let (mut e, this, texture) = rig(0, 2);
        e.call(0x00ba_3a00, &args![this]);
        assert!(calls(&e, RETURN_RENDERED_TEXTURE).is_empty());
        assert!(calls(&e, DELETE_THIS).is_empty());
        assert_eq!(e.mem.u32(texture + 4), 1);
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00ba3a00_clears_what_the_release_put_back() {
        let (mut e, this, texture) = rig(0, 1);
        let other = e.mem.alloc(8);
        // The object's last release stores another texture in the member; the clear after
        // it wins, so nothing more is released.
        e.register_double(DELETE_THIS, move |e, _| {
            e.mem.set_u32(this.addr() + 4, other);
            Ret::default()
        });
        e.call(0x00ba_3a00, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![texture + 4]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
    }
}

#[cfg(test)]
mod texture_access_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::IMPORT_INTERLOCKED_DECREMENT;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// The rig with the texture getter, the manager's return and the stage setters doubled,
    /// the manager and renderer globals mapped, and a texture holding an object (count 1),
    /// filter mode 3 and clamp mode 2.
    fn engine(rendered: u8, borrowed: u8) -> (Engine, Ptr<ImageSpaceTexture>, u32) {
        let mut e = rig();
        e.register(RENDERED_TEXTURE_GET_TEXTURE_004BC320, |_, a| {
            (a[0] + 0x100).into_ret()
        });
        e.register(TEXTURE_MANAGER_RETURN_00B6DA10, |_, _| Ret::default());
        e.register(STAGE_SET_MODE_00E7DE90, |_, _| Ret::default());
        e.register(STAGE_SET_FILTER_MODE_00E7DEF0, |_, _| Ret::default());
        e.map(0x011f_9000, 0x1000);
        e.set_global(TEXTURE_MANAGER_011F91A8, 0x0e60_0000u32);
        e.set_global(RENDERER_011F9508, 0x0e61_0000u32);
        let held = object(&mut e, 1);
        let this = e.new_object::<ImageSpaceTexture>();
        e.set(this, ImageSpaceTexture::bIsRenderedTexture, rendered);
        e.set(this, ImageSpaceTexture::bIsTextureBorrowed, borrowed);
        e.set(this, ImageSpaceTexture::pTexture, held);
        e.set(this, ImageSpaceTexture::iFilterMode, 3);
        e.set(this, ImageSpaceTexture::iClampMode, 2);
        (e, this, held)
    }

    #[test]
    fn fn_00ba3770_answers_only_a_rendered_texture() {
        let (mut e, this, held) = engine(1, 0);
        assert_eq!(e.call(0x00ba_3770, &args![this]).u32(), held);
        e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
        assert_eq!(e.call(0x00ba_3770, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_00ba3780_asks_a_rendered_texture_for_its_texture() {
        let (mut e, this, held) = engine(1, 0);
        assert_eq!(e.call(0x00ba_3780, &args![this]).u32(), held + 0x100);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE_004BC320),
            vec![vec![held, 0]]
        );
        e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
        assert_eq!(e.call(0x00ba_3780, &args![this]).u32(), held);
        assert_eq!(calls(&e, RENDERED_TEXTURE_GET_TEXTURE_004BC320).len(), 1);
    }

    #[test]
    fn fn_00ba37a0_sets_a_rendered_texture() {
        let (mut e, this, held) = engine(0, 0);
        let other = object(&mut e, 0);
        e.call(0x00ba_37a0, &args![this, other]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), other);
        assert_eq!(e.mem.u32(other + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 1);
    }

    #[test]
    fn fn_00ba37f0_sets_a_plain_texture() {
        let (mut e, this, held) = engine(1, 0);
        // The same texture again: no counting, only the flag.
        e.call(0x00ba_37f0, &args![this, held]);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        e.call(0x00ba_37f0, &args![this, 0u32]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
    }

    #[test]
    fn fn_00ba3840_borrows_from_the_manager() {
        let (mut e, this, held) = engine(0, 0);
        let borrowed = object(&mut e, 0);
        e.register_double(BORROW_RENDERED_00B6E110, move |_, _| borrowed.into_ret());
        e.call(0x00ba_3840, &args![this, 0x27u32]);
        assert_eq!(
            calls(&e, BORROW_RENDERED_00B6E110),
            vec![vec![0x0e60_0000, 0x0e61_0000, 0x27, 0, 0, 0]]
        );
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), borrowed);
        assert_eq!(e.mem.u32(borrowed + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 1);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsTextureBorrowed), 1);
    }

    #[test]
    fn fn_00ba38b0_clears_the_texture_and_modes() {
        let (mut e, this, held) = engine(1, 1);
        e.call(0x00ba_38b0, &args![this]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsTextureBorrowed), 1);
        assert_eq!(e.get(this, ImageSpaceTexture::iFilterMode), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::iClampMode), 0);
        // An empty member: nothing released.
        e.call(0x00ba_38b0, &args![this]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00ba38f0_sets_up_the_stage() {
        let (mut e, this, held) = engine(1, 0);
        let stage = e.mem.alloc(0x10);
        e.call(0x00ba_38f0, &args![this, stage]);
        assert_eq!(calls(&e, STAGE_SET_MODE_00E7DE90), vec![vec![stage, 2]]);
        assert_eq!(
            calls(&e, STAGE_SET_FILTER_MODE_00E7DEF0),
            vec![vec![stage, 3]]
        );
        assert_eq!(e.mem.u32(stage + 8), held + 0x100);
        e.set(this, ImageSpaceTexture::bIsRenderedTexture, 0);
        e.call(0x00ba_38f0, &args![this, stage]);
        assert_eq!(e.mem.u32(stage + 8), held);
        assert_eq!(e.mem.u32(held + 4), 1);
    }

    #[test]
    fn fn_00ba3940_copies_and_moves_the_borrow() {
        let (mut e, this, held) = engine(0, 0);
        let source = e.new_object::<ImageSpaceTexture>();
        let other = object(&mut e, 1);
        e.set(source, ImageSpaceTexture::bIsRenderedTexture, 1);
        e.set(source, ImageSpaceTexture::bIsTextureBorrowed, 1);
        e.set(source, ImageSpaceTexture::pTexture, other);
        e.set(source, ImageSpaceTexture::iFilterMode, 9);
        e.set(source, ImageSpaceTexture::iClampMode, 8);
        let answer = e.call(0x00ba_3940, &args![this, source]).u32();
        assert_eq!(answer, this.addr());
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 1);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsTextureBorrowed), 1);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), other);
        assert_eq!(e.mem.u32(other + 4), 2);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(e.get(this, ImageSpaceTexture::iFilterMode), 9);
        assert_eq!(e.get(this, ImageSpaceTexture::iClampMode), 2);
        assert_eq!(e.get(source, ImageSpaceTexture::bIsTextureBorrowed), 0);
    }

    #[test]
    fn fn_00ba3940_rereads_the_source_after_the_release() {
        let (mut e, this, _) = engine(0, 0);
        let source = e.new_object::<ImageSpaceTexture>();
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        e.set(source, ImageSpaceTexture::pTexture, first);
        // The old object's delete puts another texture in the source.
        e.register_double(REFERENCE_DELETE, move |e, _| {
            e.mem.set_u32(source.addr() + 4, second);
            Ret::default()
        });
        e.call(0x00ba_3940, &args![this, source]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), second);
        assert_eq!(e.mem.u32(second + 4), 2);
        assert_eq!(e.mem.u32(first + 4), 1);
    }

    #[test]
    fn fn_00ba39a0_returns_a_borrowed_texture() {
        let (mut e, this, held) = engine(1, 1);
        e.call(0x00ba_39a0, &args![this]);
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_RETURN_00B6DA10),
            vec![vec![0x0e60_0000, held]]
        );
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![held]]);
        assert_eq!(e.get(this, ImageSpaceTexture::pTexture), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsTextureBorrowed), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::bIsRenderedTexture), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::iFilterMode), 0);
        assert_eq!(e.get(this, ImageSpaceTexture::iClampMode), 0);
    }

    #[test]
    fn fn_00ba39a0_hands_back_null_for_a_plain_texture_and_skips_when_not_borrowed() {
        let (mut e, this, _) = engine(0, 1);
        e.call(0x00ba_39a0, &args![this]);
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_RETURN_00B6DA10),
            vec![vec![0x0e60_0000, 0]]
        );
        let (mut e, this, _) = engine(1, 0);
        e.call(0x00ba_39a0, &args![this]);
        assert!(calls(&e, TEXTURE_MANAGER_RETURN_00B6DA10).is_empty());
        assert_eq!(e.get(this, ImageSpaceTexture::iFilterMode), 3);
    }
}

//! `(unplaced)/run_00b4f510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four small renderer helpers on globals, between `BSShaderManager`
//! functions in the exe; the engine map does not name them.

#[allow(unused_imports)]
use crate::prelude::*;

/// A rendered texture held between frames (`011f949c`), given back to the
/// texture manager (`011f91a8`) by `BSTextureManager::ReturnRenderedTexture`
/// (`00b6da10`, engine map).
const HELD_TEXTURE: u32 = 0x011f_949c;
const TEXTURE_MANAGER: u32 = 0x011f_91a8;
const RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// The image-space manager (`011f91ac`) and
/// `ImageSpaceManager::GetDepthRenderedTexture` (`00b8b190`, engine map).
const IMAGE_SPACE_MANAGER: u32 = 0x011f_91ac;
const GET_DEPTH_RENDERED_TEXTURE: u32 = 0x00b8_b190;
/// `00b97d20` (cdecl, five words) and `00b984f0` (cdecl, two words).
const RENDER_STEP: u32 = 0x00b9_7d20;
const RENDER_FINISH: u32 = 0x00b9_84f0;
/// A word cleared by `fn_00b4f570`.
const CLEARED_WORD: u32 = 0x011f_9168;

// Translated from 00b4f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the held rendered texture back to the texture manager and forgets
/// it, when there is one.
pub fn fn_00b4f510(e: &mut Engine) {
    let texture: u32 = e.global(HELD_TEXTURE);
    if texture != 0 {
        let manager: u32 = e.global(TEXTURE_MANAGER);
        e.call(RETURN_RENDERED_TEXTURE, &args![manager, texture]);
        e.set_global(HELD_TEXTURE, 0u32);
    }
}

// Translated from 00b4f530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The image-space manager's depth rendered texture (a tail call).
pub fn fn_00b4f530(e: &mut Engine) -> u32 {
    let manager: u32 = e.global(IMAGE_SPACE_MANAGER);
    e.call(GET_DEPTH_RENDERED_TEXTURE, &args![manager]).u32()
}

// Translated from 00b4f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00b97d20(a, b, c, d, 1)` then `00b984f0(1, 1)`.
pub fn fn_00b4f540(e: &mut Engine, a: u32, b: u32, c: u32, d: u32) {
    e.call(RENDER_STEP, &args![a, b, c, d, 1u32]);
    e.call(RENDER_FINISH, &args![1u32, 1u32]);
}

// Translated from 00b4f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `011f9168`.
pub fn fn_00b4f570(e: &mut Engine) {
    e.set_global(CLEARED_WORD, 0u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4f510, fn_00b4f510()),
        entry!(0x00b4f530, fn_00b4f530() -> u32),
        entry!(0x00b4f540, fn_00b4f540(u32, u32, u32, u32)),
        entry!(0x00b4f570, fn_00b4f570()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e
    }

    #[test]
    fn held_texture_is_returned_once() {
        let mut e = engine();
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.set_global(TEXTURE_MANAGER, 0x5000u32);
        e.set_global(HELD_TEXTURE, 0x6000u32);
        e.call_log = Some(vec![]);
        e.call(0x00b4_f510, &args![]);
        e.call(0x00b4_f510, &args![]);
        let log = e.call_log.take().unwrap();
        let returns: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == RETURN_RENDERED_TEXTURE)
            .collect();
        assert_eq!(returns.len(), 1);
        assert_eq!(returns[0].1, vec![0x5000, 0x6000]);
        assert_eq!(e.global::<u32>(HELD_TEXTURE), 0);
    }

    #[test]
    fn depth_texture_comes_from_the_image_space_manager() {
        let mut e = engine();
        e.register(GET_DEPTH_RENDERED_TEXTURE, |_, a| (a[0] + 1).into_ret());
        e.set_global(IMAGE_SPACE_MANAGER, 0x7000u32);
        assert_eq!(e.call(0x00b4_f530, &args![]).u32(), 0x7001);
    }

    #[test]
    fn render_step_then_finish() {
        let mut e = engine();
        e.register(RENDER_STEP, |_, _| Ret::default());
        e.register(RENDER_FINISH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b4_f540, &args![1u32, 2u32, 3u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (RENDER_STEP, vec![1, 2, 3, 4, 1]),
                (RENDER_FINISH, vec![1, 1])
            ]
        );
    }

    #[test]
    fn clears_the_word() {
        let mut e = engine();
        e.set_global(CLEARED_WORD, 9u32);
        e.call(0x00b4_f570, &args![]);
        assert_eq!(e.global::<u32>(CLEARED_WORD), 0);
    }
}

//! `fallout/debug/debugtextpages/debugtexttexture.cpp` (Xbox PDB source unit), subsystem `fallout/debug`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    funcs_n086()
}

/// `0043b1b0` (`thiscall`, no words): the name of what the texture's
/// virtual slot `+0x9c` answers.
const NAME_0043B1B0: u32 = 0x0043_b1b0;
/// `strchr` (`00ec7690`, `cdecl`) and the CRT compare `00ec7ec0` (`cdecl`:
/// two texts and a count; 0 when they match).
const STRCHR: u32 = 0x00ec_7690;
const COMPARE_00EC7EC0: u32 = 0x00ec_7ec0;
/// The text `"\menus"` (`0106fe88`).
const TEXT_MENUS: u32 = 0x0106_fe88;

// Translated from 0071dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DebugTextTexture::IsMenuTexture` (engine map; `thiscall`, `RET 4`):
/// true when the texture's name (`0043b1b0` of its virtual slot `+0x9c`,
/// asked again for the search) has a backslash from which the next six
/// characters compare equal to `"\menus"` (`00ec7ec0`); each backslash is
/// tried in turn (`strchr` from the one after it). False without a texture
/// or a name.
pub fn debug_text_texture_is_menu_texture(e: &mut Engine, _this: Ptr, texture: u32) -> u8 {
    let mut found = 0;
    if texture == 0 {
        return found;
    }
    let object = e.vcall(texture, 0x9c, &[]).u32();
    if e.call(NAME_0043B1B0, &args![object]).u32() == 0 {
        return found;
    }
    let object = e.vcall(texture, 0x9c, &[]).u32();
    let name = e.call(NAME_0043B1B0, &args![object]).u32();
    let mut at = e.call(STRCHR, &args![name, 0x5cu32]).u32();
    while at != 0 {
        if e.call(COMPARE_00EC7EC0, &args![at, TEXT_MENUS, 6u32]).u32() == 0 {
            found = 1;
            at = 0;
        } else {
            at = e.call(STRCHR, &args![at.wrapping_add(1), 0x5cu32]).u32();
        }
    }
    found
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0071_dca0,
        debug_text_texture_is_menu_texture(Ptr, u32) -> u8
    )]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn setup(e: &mut Engine, name: &[u8]) -> u32 {
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.write(text, name);
        let texture = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(texture, vtable);
        e.mem.set_u32(vtable + 0x9c, 0x7400_009c);
        e.register_double(NAME_0043B1B0, move |_, _| text.into_ret());
        e.register(0x7400_009c, |_, _| 0x1234u32.into_ret());
        e.register(STRCHR, |e, a| {
            let mut at = a[0];
            loop {
                let c = e.mem.u8(at);
                if u32::from(c) == a[1] {
                    return at.into_ret();
                }
                if c == 0 {
                    return 0u32.into_ret();
                }
                at += 1;
            }
        });
        // Matches when the text at the first word starts "\m".
        e.register(COMPARE_00EC7EC0, |e, a| {
            let same = e.mem.u8(a[0]) == b'\\' && e.mem.u8(a[0] + 1) == b'm';
            u32::from(!same).into_ret()
        });
        texture
    }

    #[test]
    fn is_menu_texture_finds_a_menus_folder() {
        let mut e = Engine::new();
        let texture = setup(&mut e, b"a\\b\\m");
        assert_eq!(e.call(0x0071_dca0, &args![0u32, texture]).u32() & 0xff, 1);
        let texture = setup(&mut e, b"a\\b\\c");
        assert_eq!(e.call(0x0071_dca0, &args![0u32, texture]).u32() & 0xff, 0);
        assert_eq!(e.call(0x0071_dca0, &args![0u32, 0u32]).u32() & 0xff, 0);
        e.register(NAME_0043B1B0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0071_dca0, &args![0u32, texture]).u32() & 0xff, 0);
        assert!(e.call_log.as_ref().unwrap().iter().all(|c| c.0 != STRCHR));
    }
}

//! `(unplaced)/run_007cb330` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `StartMenu`'s vtable (RTTI `.?AVStartMenu@@`).
const START_MENU_VTABLE: u32 = 0x0107_6d1c;
/// The menu base constructor (`00a1c4a0`, `thiscall`; `BSMenu` by the
/// engine map).
const MENU_BASE_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The member constructors (`thiscall`): `007cb540` (four members),
/// `007642f0`, `007d7c60` (no stack words) and `00633c90` (one word).
const MEMBER_CONSTRUCTOR_007CB540: u32 = 0x007c_b540;
const MEMBER_CONSTRUCTOR_007642F0: u32 = 0x0076_42f0;
const MEMBER_CONSTRUCTOR_007D7C60: u32 = 0x007d_7c60;
const MEMBER_CONSTRUCTOR_00633C90: u32 = 0x0063_3c90;
/// `Tile::AddUserTrait` (`00a00940`, engine map; `cdecl`: name, -1) and
/// `"_enabled"` (`01070244`).
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
const ENABLED_TRAIT: u32 = 0x0107_0244;
/// `memset` (`00403d30`, `cdecl`).
const MEMSET: u32 = 0x0040_3d30;
/// The global word the constructor clears (`011daab4`).
const STATE_011DAAB4: u32 = 0x011d_aab4;
/// `BSAudio::QInstance` (`00453a70`) and `0070bbe0` (`thiscall` on the audio
/// object, no stack words).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_0070BBE0: u32 = 0x0070_bbe0;

// Translation incomplete: 007cb330 - the FS:[0] exception frame (destroying the constructed members when a constructor throws) is not translated
// Translated from 007cb330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StartMenu` constructor (by its vtable's RTTI; unnamed in the engine
/// map; `thiscall`, no stack words). Runs the menu base constructor, sets
/// the vtable, constructs the members at +0x84, +0xB4, +0xE4, +0x114
/// (`007cb540`), +0x144 (`007642f0`), +0x174 (`007d7c60`) and +0x1B8
/// (`00633c90` with 0), stores the `"_enabled"` user trait id at +0x1D0,
/// clears 0x5C bytes at +0x28, then the words +0x1A8, +0x1A4, the float
/// +0x1AC, +0x1B0, +0x1C4, +0x1C0, +0x1BC, sets +0x1C8 to 0xFF, clears
/// +0x1CC, the global `011daab4` and +0x1B4, and runs `0070bbe0` on the
/// audio object. Returns `this` (fields unnamed here).
pub fn fn_007cb330(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(MENU_BASE_CONSTRUCTOR, &args![at]);
    e.mem.set_u32(at, START_MENU_VTABLE);
    for offset in [0x84u32, 0xb4, 0xe4, 0x114] {
        e.call(MEMBER_CONSTRUCTOR_007CB540, &args![at + offset]);
    }
    e.call(MEMBER_CONSTRUCTOR_007642F0, &args![at + 0x144]);
    e.call(MEMBER_CONSTRUCTOR_007D7C60, &args![at + 0x174]);
    e.call(MEMBER_CONSTRUCTOR_00633C90, &args![at + 0x1b8, 0u32]);
    let trait_id = e
        .call(TILE_ADD_USER_TRAIT, &args![ENABLED_TRAIT, 0xffff_ffffu32])
        .u32();
    e.mem.set_u32(at + 0x1d0, trait_id);
    e.call(MEMSET, &args![at + 0x28, 0u32, 0x5cu32]);
    e.mem.set_u32(at + 0x1a8, 0);
    e.mem.set_u32(at + 0x1a4, 0);
    e.mem.set_u32(at + 0x1ac, 0.0f32.to_bits());
    e.mem.set_u32(at + 0x1b0, 0);
    e.mem.set_u32(at + 0x1c4, 0);
    e.mem.set_u32(at + 0x1c0, 0);
    e.mem.set_u32(at + 0x1bc, 0);
    e.mem.set_u32(at + 0x1c8, 0xff);
    e.mem.set_u32(at + 0x1cc, 0);
    e.mem.set_u32(STATE_011DAAB4, 0);
    e.mem.set_u32(at + 0x1b4, 0);
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(AUDIO_0070BBE0, &args![audio]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007cb330, fn_007cb330(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_menu_constructor_sets_the_fields() {
        let mut e = Engine::new();
        for address in [
            MENU_BASE_CONSTRUCTOR,
            MEMBER_CONSTRUCTOR_007CB540,
            MEMBER_CONSTRUCTOR_007642F0,
            MEMBER_CONSTRUCTOR_007D7C60,
            MEMBER_CONSTRUCTOR_00633C90,
            MEMSET,
            AUDIO_0070BBE0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TILE_ADD_USER_TRAIT, |_, _| 0x0abcu32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x0a0du32.into_ret());
        e.map(0x011d_a000, 0x1000);
        e.mem.set_u32(STATE_011DAAB4, 5);
        let this = e.mem.alloc(0x1d8);
        e.mem.set_u32(this + 0x1cc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007c_b330, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), START_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1d0), 0x0abc);
        assert_eq!(e.mem.u32(this + 0x1c8), 0xff);
        assert_eq!(e.mem.u32(this + 0x1cc), 0);
        assert_eq!(e.mem.u32(STATE_011DAAB4), 0);
        let log = e.call_log.take().unwrap();
        let members: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == MEMBER_CONSTRUCTOR_007CB540)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(members, [0x84, 0xb4, 0xe4, 0x114]);
        assert!(log.contains(&(MEMBER_CONSTRUCTOR_00633C90, vec![this + 0x1b8, 0])));
        assert!(log.contains(&(TILE_ADD_USER_TRAIT, vec![ENABLED_TRAIT, 0xffff_ffff])));
        assert!(log.contains(&(MEMSET, vec![this + 0x28, 0, 0x5c])));
        assert!(log.contains(&(AUDIO_0070BBE0, vec![0x0a0d])));
    }
}

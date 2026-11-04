//! `fallout/interface/run_00788440` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `LoadingMenu`'s vtable (`01073ebc`, RTTI `.?AVLoadingMenu@@`).
const LOADING_MENU_VTABLE: u32 = 0x0107_3ebc;
/// The `Menu` constructor (`00a1c4a0`).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The constructors of the members: `0048f200` (+0xD0 .. +0x100),
/// `BSStringT<char>` (`004037b0`), `NiColorA` (`00414430`, four floats),
/// `0096a2d0` (+0x1D0), `BSSoundHandle` (`0041a250`), and the array
/// element pointer (`006694e0`, with destructor `0045cec0`) through
/// `` `eh vector constructor iterator' `` (`00ec782f`, stdcall).
const MEMBER_CONSTRUCTOR_0048F200: u32 = 0x0048_f200;
const BS_STRING_CONSTRUCTOR: u32 = 0x0040_37b0;
const COLOR_A_CONSTRUCTOR: u32 = 0x0041_4430;
const MEMBER_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const NI_POINTER_ELEMENT_CONSTRUCT: u32 = 0x0066_94e0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The fill of +0x28 (`00403d30`, cdecl: destination, value, size; a
/// `memset` wrapper).
const FILL: u32 = 0x0040_3d30;
/// The `NiPointer` assignment (`0066b0d0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The object whose slot `+0xB4` receives the colour (`0043c4b0`, no
/// words).
const COLOR_OWNER: u32 = 0x0043_c4b0;
const COLOR_OWNER_SLOT: u32 = 0xb4;
/// The menu flag setter (`00458080`, thiscall: flag, value).
const SET_MENU_FLAG: u32 = 0x0045_8080;
/// The byte at `011a0294` whose clear state sets flag 0.
const FLAG_ZERO_SOURCE: u32 = 0x011a_0294;
/// Answers its `this` (`006815c0`).
const IDENTITY: u32 = 0x0068_15c0;
/// The function the engine map names `ActorMover::GetPreferredMoveMode`
/// (`005f36f0`; ENGINE_CRATE.md notes it answers `TES::pInteriorCell` for
/// another caller), and `LoadingMenu::CalcLoadScreenTypeTextWindowFaces`
/// (`0078b9b0`, Xbox PDB name; one word).
const FOLDED_005F36F0: u32 = 0x005f_36f0;
const CALC_LOAD_SCREEN_TYPE_TEXT_WINDOW_FACES: u32 = 0x0078_b9b0;

// Translated from 00788440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoadingMenu` constructor (no PDB name; vtable `01073ebc`): the
/// `Menu` constructor and the members' constructors, +0x28 filled with
/// 0x98 zero bytes, the counters and pointers cleared (+0x1E0, +0x1E4,
/// +0x21C, +0x220, +0x222, +0x5B8, +0x5BC, +0x1E8, +0x1EC, +0x1F0,
/// +0x5B4), the four pointers at +0x20C dropped, the 43 words at +0x10C
/// set to `0x2B`, the colour (+0x1C0) handed to the object `0043c4b0`
/// answers (slot `+0xB4`), +0x1D8 pointed at +0x1D0, menu flag 0 set to
/// whether the byte at `011a0294` is clear and flag 4 set; then, when the
/// word at +0x1D0 (+0x1DC) is not null and `005f36f0` answers an object for
/// it, the load-screen text window faces are computed for that object's
/// +0x18. Returns `this`.
// Translation incomplete: 00788440 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_00788440(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(t, LOADING_MENU_VTABLE);
    for offset in [0xd0u32, 0xdc, 0xe8, 0xf4, 0x100] {
        e.call(MEMBER_CONSTRUCTOR_0048F200, &args![t + offset]);
    }
    e.call(BS_STRING_CONSTRUCTOR, &args![t + 0x1b8]);
    e.call(
        COLOR_A_CONSTRUCTOR,
        &args![t + 0x1c0, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
    );
    e.call(MEMBER_CONSTRUCTOR_0096A2D0, &args![t + 0x1d0]);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![t + 0x1f4]);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![t + 0x200]);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            t + 0x20c,
            4u32,
            4u32,
            NI_POINTER_ELEMENT_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(FILL, &args![t + 0x28, 0u32, 0x98u32]);
    e.mem.set_u32(t + 0x1e4, 0);
    e.mem.set_u32(t + 0x1e0, 0);
    e.mem.set_u16(t + 0x222, 0);
    e.mem.set_u16(t + 0x220, 0);
    e.mem.set_u32(t + 0x5b8, 0);
    e.mem.set_u32(t + 0x21c, 0);
    for i in 0..4u32 {
        e.call(NI_POINTER_ASSIGN, &args![t + 0x20c + 4 * i, 0u32]);
    }
    e.mem.set_u32(t + 0x5bc, 0);
    e.mem.set_u32(t + 0x1e8, 0);
    for i in 0..0x2bu32 {
        e.mem.set_u32(t + 0x10c + 4 * i, 0x2b);
    }
    let owner = e.call(COLOR_OWNER, &args![]).u32();
    e.vcall(owner, COLOR_OWNER_SLOT, &args![t + 0x1c0]);
    e.mem.set_u32(t + 0x1f0, 0);
    e.mem.set_u32(t + 0x1ec, 0);
    e.mem.set_u32(t + 0x1d8, t + 0x1d0);
    e.mem.set_u32(t + 0x5b4, 0);
    let clear = (e.global::<u8>(FLAG_ZERO_SOURCE) == 0) as u32;
    e.call(SET_MENU_FLAG, &args![this, 0u32, clear]);
    e.call(SET_MENU_FLAG, &args![this, 4u32, 1u32]);
    let source = e.mem.u32(t + 0x1d8);
    let source = e.call(IDENTITY, &args![source]).u32();
    let word = e.mem.u32(source);
    e.mem.set_u32(t + 0x1dc, word);
    if e.mem.u32(t + 0x1dc) != 0 {
        let word = e.mem.u32(t + 0x1dc);
        let object = e.call(FOLDED_005F36F0, &args![word]).u32();
        if object != 0 {
            e.call(
                CALC_LOAD_SCREEN_TYPE_TEXT_WINDOW_FACES,
                &args![this, object + 0x18],
            );
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0078_8440, fn_00788440(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine(source_byte: u8) -> Engine {
        let mut e = Engine::new();
        for address in [
            MENU_CONSTRUCTOR,
            MEMBER_CONSTRUCTOR_0048F200,
            BS_STRING_CONSTRUCTOR,
            COLOR_A_CONSTRUCTOR,
            MEMBER_CONSTRUCTOR_0096A2D0,
            SOUND_HANDLE_CONSTRUCTOR,
            VECTOR_CONSTRUCTOR_ITERATOR,
            FILL,
            NI_POINTER_ASSIGN,
            SET_MENU_FLAG,
            CALC_LOAD_SCREEN_TYPE_TEXT_WINDOW_FACES,
            0x0000_10b4,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(IDENTITY, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let mut slots = vec![0u32; 0x2e];
        slots[(COLOR_OWNER_SLOT / 4) as usize] = 0x0000_10b4;
        e.put_vtable(0x0003_0000, &slots);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0003_0000);
        e.register_double(COLOR_OWNER, move |_, _| Ret {
            eax: owner,
            ..Ret::default()
        });
        e.map(FLAG_ZERO_SOURCE, 1);
        e.set_global(FLAG_ZERO_SOURCE, source_byte);
        e
    }

    #[test]
    fn fn_00788440_constructs_the_loading_menu() {
        let mut e = engine(0);
        e.register(FOLDED_005F36F0, |_, _| Ret {
            eax: 0x0000_6000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x5c0);
        e.mem.set_u32(this + 0x1d0, 0x7777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_8440, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LOADING_MENU_VTABLE);
        assert_eq!(calls_to(&e, MEMBER_CONSTRUCTOR_0048F200).len(), 5);
        assert_eq!(calls_to(&e, FILL), vec![vec![this + 0x28, 0, 0x98]]);
        assert_eq!(e.mem.u32(this + 0x10c), 0x2b);
        assert_eq!(e.mem.u32(this + 0x10c + 4 * 42), 0x2b);
        assert_eq!(e.mem.u32(this + 0x10c + 4 * 43), 0);
        assert_eq!(calls_to(&e, 0x0000_10b4)[0][1], this + 0x1c0);
        assert_eq!(e.mem.u32(this + 0x1d8), this + 0x1d0);
        assert_eq!(
            calls_to(&e, SET_MENU_FLAG),
            vec![vec![this, 0, 1], vec![this, 4, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x1dc), 0x7777);
        assert_eq!(calls_to(&e, FOLDED_005F36F0), vec![vec![0x7777]]);
        assert_eq!(
            calls_to(&e, CALC_LOAD_SCREEN_TYPE_TEXT_WINDOW_FACES),
            vec![vec![this, 0x6018]]
        );
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN).len(), 4);
    }

    #[test]
    fn fn_00788440_skips_the_faces_without_an_object() {
        let mut e = engine(1);
        e.register(FOLDED_005F36F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x5c0);
        e.call_log = Some(vec![]);
        e.call(0x0078_8440, &args![this]);
        assert_eq!(calls_to(&e, SET_MENU_FLAG)[0], vec![this, 0, 0]);
        assert!(calls_to(&e, FOLDED_005F36F0).is_empty());
        e.mem.set_u32(this + 0x1d0, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0078_8440, &args![this]);
        assert_eq!(calls_to(&e, FOLDED_005F36F0).len(), 1);
        assert!(calls_to(&e, CALC_LOAD_SCREEN_TYPE_TEXT_WINDOW_FACES).is_empty());
    }
}

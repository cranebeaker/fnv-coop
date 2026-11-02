//! `fallout/interface/run_007fa8d0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here belong to `FORenderedMenu` (the rendered-menu base
//! of the Pip-Boy and the HUD screens; vtable `0107841c`, RTTI
//! `.?AVFORenderedMenu@@`): its constructor, destructor and clean-up. The
//! PC code uses the Xbox PDB's offsets of the class.

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_007fa970` calls on the member (`00559450`).
const CALLEE_00559450: u32 = 0x0055_9450;

// Translated from 007fa970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00559450` with `ECX` = `this + 0xe0` and passes on its result.
pub fn fn_007fa970(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(0xe0)]).u32()
}

/// `FORenderedMenu`'s vtable (`0107841c`).
const FO_RENDERED_MENU_VTABLE: u32 = 0x0107_841c;
/// The `NiPointer` constructor (`00633c90`, thiscall: the pointer),
/// assignment (`0066b0d0`) and destructor (`0045cec0`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The `NiColor` (`00416870`, three floats) and `NiColorA` (`00414430`,
/// four floats) constructors; the latter returns `this`.
const COLOR_CONSTRUCTOR: u32 = 0x0041_6870;
const COLOR_A_CONSTRUCTOR: u32 = 0x0041_4430;
/// The game time in milliseconds (`00825c00`, thiscall on `011f6394`).
const TIME_GLOBALS: u32 = 0x011f_6394;
const GET_MILLISECONDS: u32 = 0x0082_5c00;
/// `rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// An integer setting's value (`0043d4d0`) and a float setting's value
/// (`00403e20`) (thiscall on the setting: a pointer to the value).
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The settings: the random part of the next distortion time
/// (`011db4f8`) and the distortion duration (`011db4d0`).
const DISTORT_INTERVAL_SETTING: u32 = 0x011d_b4f8;
const DISTORT_DURATION_SETTING: u32 = 0x011d_b4d0;
/// `FORenderedMenu::ClearMenuLight` (`007fc6f0`, Xbox PDB name).
const CLEAR_MENU_LIGHT: u32 = 0x007f_c6f0;
/// The calls of `fn_007fa990`: the step of the object at the word
/// `01202d98` (`00c3dfa0`, one word), the player's object (`008d8520` on
/// the player at the word `011dea3c`; the engine map names it
/// `MiddleHighProcess::GetSavedAcquireObject`) and the step `005db4c0` (one
/// word).
const OBJECT_01202D98: u32 = 0x0120_2d98;
const STEP_00C3DFA0: u32 = 0x00c3_dfa0;
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_OBJECT: u32 = 0x008d_8520;
const STEP_005DB4C0: u32 = 0x005d_b4c0;

// Translated from 007fa950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 4` (through `00559450`).
pub fn fn_007fa950(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(4)]).u32()
}

// Translated from 007fa990 (decompiled, FalloutNV.exe 1.4.0.525)
/// A rendered menu's opening step (no PDB name): `00c3dfa0(5)` on the
/// object at `01202d98`; on the player's object (`008d8520`), the virtual
/// slots `+0x170` (the player), `+0x468` (1) and `+0x464` (the player);
/// then this menu's slots `+0x28` and `+0x24` and `005db4c0(0)`.
pub fn fn_007fa990(e: &mut Engine, this: Ptr) {
    let object = e.global::<u32>(OBJECT_01202D98);
    e.call(STEP_00C3DFA0, &args![object, 5u32]);
    let player = e.global::<u32>(PLAYER);
    let player_object = e.call(PLAYER_OBJECT, &args![player]).u32();
    let player = e.global::<u32>(PLAYER);
    e.vcall(player_object, 0x170, &args![player]);
    e.vcall(player_object, 0x468, &args![1u32]);
    let player = e.global::<u32>(PLAYER);
    e.vcall(player_object, 0x464, &args![player]);
    e.vcall(this.addr(), 0x28, &args![]);
    e.vcall(this.addr(), 0x24, &args![]);
    e.call(STEP_005DB4C0, &args![this, 0u32]);
}

// Translated from 007faa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORenderedMenu` constructor (no PDB name; vtable `0107841c`):
/// constructs the pointers (+4 .. +0x24, +0xBC, +0xD4) and colours (the
/// light colour +0x2C, the background +0xA8, the tint +0xC0) empty, drops
/// the geometry pointers, clears the menu tile (+0x18), the flags (+0x28,
/// +0x29, +0x2A, +0x4D, +0x60, +0x74, +0x94, +0x95, +0xB8, +0xB9) and the
/// pulse values (+0x8C, +0x90), sets the next distortion time (+0xA4) to
/// the game time plus `rand() %` the setting at `011db4f8` and the
/// distortion duration (+0xA0) to the setting at `011db4d0`, and clears
/// the shadow scene node (+0x20). Returns `this`.
// Translation incomplete: 007faa20 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_007faa20(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, FO_RENDERED_MENU_VTABLE);
    for offset in [0x04u32, 0x08, 0x0c, 0x10, 0x14, 0x1c, 0x24] {
        e.call(NI_POINTER_CONSTRUCT, &args![t + offset, 0u32]);
    }
    e.call(COLOR_CONSTRUCTOR, &args![t + 0x2c, 0.0f32, 0.0f32, 0.0f32]);
    e.call(
        COLOR_A_CONSTRUCTOR,
        &args![t + 0xa8, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![t + 0xbc, 0u32]);
    e.call(
        COLOR_A_CONSTRUCTOR,
        &args![t + 0xc0, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![t + 0xd4, 0u32]);
    for offset in [0x24u32, 0x04, 0x14, 0x0c, 0x10] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
    e.mem.set_u32(t + 0x18, 0);
    for offset in [0x28u32, 0x29, 0x4d, 0x60, 0x74, 0xb8, 0xb9] {
        e.mem.set_u8(t + offset, 0);
    }
    e.mem.set_f32(t + 0x8c, 0.0);
    e.mem.set_f32(t + 0x90, 0.0);
    e.call(NI_POINTER_ASSIGN, &args![t + 0x1c, 0u32]);
    // A cleared `NiColorA` temporary copied into the background colour.
    e.with_stack(0x10, |e, color| {
        let color = e
            .call(
                COLOR_A_CONSTRUCTOR,
                &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        for i in 0..4 {
            let word = e.mem.u32(color + 4 * i);
            e.mem.set_u32(t + 0xa8 + 4 * i, word);
        }
    });
    e.mem.set_u8(t + 0x2a, 0);
    e.mem.set_u8(t + 0x95, 0);
    let now = e.call(GET_MILLISECONDS, &args![TIME_GLOBALS]).u32();
    let now = f64::from(now);
    let random = e.call(RAND, &args![]).i32();
    let interval = e
        .call(SETTING_INT_VALUE, &args![DISTORT_INTERVAL_SETTING])
        .u32();
    let part = random % e.mem.i32(interval);
    e.mem
        .set_u32(t + 0xa4, x87_store32(x87_add(f64::from(part), now)));
    let duration = e
        .call(SETTING_FLOAT_VALUE, &args![DISTORT_DURATION_SETTING])
        .u32();
    let duration = x87_quiet_bits(e.mem.u32(duration));
    e.mem.set_u32(t + 0xa0, duration);
    e.mem.set_u8(t + 0x94, 0);
    e.mem.set_u32(t + 0x20, 0);
    this
}

// Translated from 007fad10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORenderedMenu` destructor (no PDB name; vtable `0107841c`): the
/// clean-up ([`fn_007fadf0`]), then the pointer members' destructors
/// (+0xD4, +0xBC, +0x24, +0x1C, +0x14, +0x10, +0xC, +8, +4).
// Translation incomplete: 007fad10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_007fad10(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, FO_RENDERED_MENU_VTABLE);
    fn_007fadf0(e, this);
    for offset in [0xd4u32, 0xbc, 0x24, 0x1c, 0x14, 0x10, 0x0c, 0x08, 0x04] {
        e.call(NI_POINTER_DESTRUCT, &args![t + offset]);
    }
}

// Translated from 007fadf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORenderedMenu` clean-up (no PDB name): clears the menu light
/// (`ClearMenuLight`), drops the pointers (+4, +0x14, +0xC, +0x10, +0x1C,
/// +0x24, +0xD4) and clears `bInitialized` (+0x28).
pub fn fn_007fadf0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.call(CLEAR_MENU_LIGHT, &args![this]);
    for offset in [0x04u32, 0x14, 0x0c, 0x10, 0x1c, 0x24, 0xd4] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
    e.mem.set_u8(t + 0x28, 0);
}

/// The app-culled setter (`00450f90`, `RET 4`) and `MemoryManager` free (`00401030`,
/// cdecl).
const SET_APP_CULLED: u32 = 0x0045_0f90;
const FREE: u32 = 0x0040_1030;

// Translated from 007fa8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on an `FORenderedMenu`, `RET 4`: a `bool`): with both the screen
/// parent (+4, `007fa950`) and the object at +0xe0 (`007fa970`), hides both (app-culled
/// 1) when `hide` is set, else shows both (0).
pub fn fn_007fa8d0(e: &mut Engine, this: Ptr, hide: u8) {
    if fn_007fa950(e, this) == 0 || fn_007fa970(e, this) == 0 {
        return;
    }
    let culled = u32::from(hide != 0);
    let object = fn_007fa950(e, this);
    e.call(SET_APP_CULLED, &args![object, culled]);
    let object = fn_007fa970(e, this);
    e.call(SET_APP_CULLED, &args![object, culled]);
}

// Translated from 007face0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu`'s scalar deleting destructor (`thiscall`, `RET 4`): the destructor
/// (`007fad10`), then the block freed when bit 0 of `flags` is set. Answers `this`.
pub fn fn_007face0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007fad10(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_a970, fn_007fa970(Ptr) -> u32),
        entry!(0x007f_a950, fn_007fa950(Ptr) -> u32),
        entry!(0x007f_a990, fn_007fa990(Ptr)),
        entry!(0x007f_aa20, fn_007faa20(Ptr) -> Ptr),
        entry!(0x007f_ad10, fn_007fad10(Ptr)),
        entry!(0x007f_adf0, fn_007fadf0(Ptr)),
        entry!(0x007f_a8d0, fn_007fa8d0(Ptr, u8)),
        entry!(0x007f_ace0, fn_007face0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_007fa970_passes_this_plus_e0_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_a970, &args![0x4100u32]).u32(), 0x14b5);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00559450).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xe0]);
    }
}

#[cfg(test)]
mod rendered_menu_tests {
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

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_007fa950_reads_the_word_at_4() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x007f_a950, &args![0x5000u32]).u32(), 0x5005);
    }

    #[test]
    fn fn_007fa990_runs_the_opening_steps() {
        let mut e = Engine::new();
        e.map(OBJECT_01202D98, 4);
        e.set_global(OBJECT_01202D98, 0x0000_6100u32);
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x0000_6200u32);
        let object = e.mem.alloc(8);
        let menu = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x11c];
        slots[0x170 / 4] = 0x1170;
        slots[0x468 / 4] = 0x1468;
        slots[0x464 / 4] = 0x1464;
        slots[0x28 / 4] = 0x1028;
        slots[0x24 / 4] = 0x1024;
        e.put_vtable(0x0003_0000, &slots);
        e.mem.set_u32(object, 0x0003_0000);
        e.mem.set_u32(menu, 0x0003_0000);
        e.register_double(PLAYER_OBJECT, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                STEP_00C3DFA0,
                STEP_005DB4C0,
                0x1170,
                0x1468,
                0x1464,
                0x1028,
                0x1024,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_a990, &args![menu]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(
            order[1..],
            [
                STEP_00C3DFA0,
                PLAYER_OBJECT,
                0x1170,
                0x1468,
                0x1464,
                0x1028,
                0x1024,
                STEP_005DB4C0
            ]
        );
        assert_eq!(calls_to(&e, STEP_00C3DFA0), vec![vec![0x6100, 5]]);
        assert_eq!(calls_to(&e, PLAYER_OBJECT), vec![vec![0x6200]]);
        assert_eq!(calls_to(&e, 0x1170), vec![vec![object, 0x6200]]);
        assert_eq!(calls_to(&e, 0x1468), vec![vec![object, 1]]);
        assert_eq!(calls_to(&e, STEP_005DB4C0), vec![vec![menu, 0]]);
    }

    #[test]
    fn fn_007faa20_constructs_the_rendered_menu() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[NI_POINTER_CONSTRUCT, NI_POINTER_ASSIGN, COLOR_CONSTRUCTOR],
        );
        e.register(COLOR_A_CONSTRUCTOR, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(GET_MILLISECONDS, |_, _| Ret {
            eax: 9000,
            ..Ret::default()
        });
        e.register(RAND, |_, _| Ret {
            eax: 77,
            ..Ret::default()
        });
        e.register(SETTING_INT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_FLOAT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.map(DISTORT_INTERVAL_SETTING, 8);
        e.mem.set_i32(DISTORT_INTERVAL_SETTING + 4, 50);
        e.map(DISTORT_DURATION_SETTING, 8);
        e.mem.set_f32(DISTORT_DURATION_SETTING + 4, 1.25);
        let this = e.mem.alloc(0xd8);
        e.mem.set_u8(this + 0x28, 1);
        e.mem.set_u32(this + 0x20, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_aa20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FO_RENDERED_MENU_VTABLE);
        let constructed: Vec<u32> = calls_to(&e, NI_POINTER_CONSTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(
            constructed,
            vec![4, 8, 0xc, 0x10, 0x14, 0x1c, 0x24, 0xbc, 0xd4]
        );
        assert_eq!(
            calls_to(&e, COLOR_CONSTRUCTOR),
            vec![vec![this + 0x2c, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, COLOR_A_CONSTRUCTOR).len(), 3);
        let dropped: Vec<u32> = calls_to(&e, NI_POINTER_ASSIGN)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(dropped, vec![0x24, 4, 0x14, 0xc, 0x10, 0x1c]);
        assert_eq!(e.mem.u8(this + 0x28), 0);
        // 9000 + 77 % 50.
        assert_eq!(e.mem.f32(this + 0xa4), 9027.0);
        assert_eq!(e.mem.f32(this + 0xa0), 1.25);
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }

    #[test]
    fn fn_007fad10_cleans_up_and_destroys_the_pointers() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[CLEAR_MENU_LIGHT, NI_POINTER_ASSIGN, NI_POINTER_DESTRUCT],
        );
        let this = e.mem.alloc(0xd8);
        e.mem.set_u8(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_ad10, &args![this]);
        assert_eq!(e.mem.u32(this), FO_RENDERED_MENU_VTABLE);
        assert_eq!(calls_to(&e, CLEAR_MENU_LIGHT), vec![vec![this]]);
        let dropped: Vec<u32> = calls_to(&e, NI_POINTER_ASSIGN)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(dropped, vec![4, 0x14, 0xc, 0x10, 0x1c, 0x24, 0xd4]);
        assert_eq!(e.mem.u8(this + 0x28), 0);
        let destroyed: Vec<u32> = calls_to(&e, NI_POINTER_DESTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(
            destroyed,
            vec![0xd4, 0xbc, 0x24, 0x1c, 0x14, 0x10, 0xc, 8, 4]
        );
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn hides_or_shows_both_screens() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SET_APP_CULLED, |_, _| Ret::default());
        let this = e.mem.alloc(0xe4);
        e.mem.set_u32(this + 4, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x007f_a8d0, &args![this, 1u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == SET_APP_CULLED));
        e.mem.set_u32(this + 0xe0, 0x5000);
        e.call(0x007f_a8d0, &args![this, 1u32]);
        e.call(0x007f_a8d0, &args![this, 0u32]);
        let culls: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == SET_APP_CULLED)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            culls,
            vec![
                vec![0x4000, 1],
                vec![0x5000, 1],
                vec![0x4000, 0],
                vec![0x5000, 0]
            ]
        );
    }

    #[test]
    fn deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        for address in [
            FREE,
            CLEAR_MENU_LIGHT,
            NI_POINTER_ASSIGN,
            NI_POINTER_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let (first, second) = (e.mem.alloc(0xe4), e.mem.alloc(0xe4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_ace0, &args![first, 1u32]).u32(), first);
        e.call(0x007f_ace0, &args![second, 0u32]);
        assert_eq!(e.mem.u32(first), FO_RENDERED_MENU_VTABLE);
        let frees: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == FREE)
            .collect();
        assert_eq!(frees, vec![(FREE, vec![first])]);
    }
}

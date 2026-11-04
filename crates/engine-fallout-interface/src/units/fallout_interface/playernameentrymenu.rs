//! `fallout/interface/menus/playernameentrymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007ab7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010757fc` (RTTI `.?AVPlayerNameEntryMenu@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00a1c520`. One word, plain `RET`, returns nothing.
pub fn fn_007ab7a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_57fc);
    e.call(0x00a1_c520, &args![this]);
}

/// The free `00401030` (`cdecl`, one word).
const FREE_00401030: u32 = 0x0040_1030;
/// The entered name (`011daed8`, a string) and its clear `00438390` (`thiscall`, one
/// word).
const ENTERED_NAME: u32 = 0x011d_aed8;
const STRING_CLEAR_00438390: u32 = 0x0043_8390;
/// `007ab810` (no words): the entered name's characters.
const ENTERED_NAME_007AB810: u32 = 0x007a_b810;
/// `007ab9a0` (`thiscall`, one word: a name): takes the entered name.
const TAKE_NAME_007AB9A0: u32 = 0x007a_b9a0;

// Translated from 007ab770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerNameEntryMenu::_scalar_deleting_destructor_` (engine map; `thiscall`, `RET 4`:
/// the flags): runs the destructor body ([`fn_007ab7a0`]) and frees the menu
/// (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn player_name_entry_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_007ab7a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

// Translated from 007ab7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): clears the entered name at `011daed8`
/// (`00438390` with 0).
pub fn fn_007ab7c0(e: &mut Engine) {
    e.call(STRING_CLEAR_00438390, &args![ENTERED_NAME, 0u32]);
}

// Translated from 007ab7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerNameEntryMenu::DoIdle` (engine map; `thiscall`): when a name has been entered
/// (`007ab810`), the menu takes it (`007ab9a0`) and the entered name is cleared
/// ([`fn_007ab7c0`]).
pub fn player_name_entry_menu_do_idle(e: &mut Engine, this: Ptr) {
    if e.call(ENTERED_NAME_007AB810, &args![]).u32() == 0 {
        return;
    }
    let name = e.call(ENTERED_NAME_007AB810, &args![]).u32();
    e.call(TAKE_NAME_007AB9A0, &args![this, name]);
    fn_007ab7c0(e);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_b7a0, fn_007ab7a0(Ptr)),
        entry!(
            0x007a_b770,
            player_name_entry_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007a_b7c0, fn_007ab7c0()),
        entry!(0x007a_b7e0, player_name_entry_menu_do_idle(Ptr)),
    ]
}

#[cfg(test)]
mod menu_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.register(0x00a1_c520, |_, _| Ret::default());
        e.register(FREE_00401030, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x007a_b770, &[this, flags]).u32(), this);
        }
        assert_eq!(called(&e, 0x00a1_c520).len(), 4);
        assert_eq!(called(&e, FREE_00401030), vec![vec![this], vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0107_57fc);
    }

    #[test]
    fn clear_name_clears_the_entered_name() {
        let mut e = Engine::new();
        e.register(STRING_CLEAR_00438390, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007a_b7c0, &[]);
        assert_eq!(
            called(&e, STRING_CLEAR_00438390),
            vec![vec![ENTERED_NAME, 0]]
        );
    }

    #[test]
    fn do_idle_takes_an_entered_name() {
        for name in [0u32, 0x4e00] {
            let mut e = Engine::new();
            e.register_double(ENTERED_NAME_007AB810, move |_, _| name.into_ret());
            e.register(TAKE_NAME_007AB9A0, |_, _| Ret::default());
            e.register(STRING_CLEAR_00438390, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x007a_b7e0, &[0x3000]);
            if name == 0 {
                assert!(called(&e, TAKE_NAME_007AB9A0).is_empty());
                assert!(called(&e, STRING_CLEAR_00438390).is_empty());
            } else {
                assert_eq!(called(&e, TAKE_NAME_007AB9A0), vec![vec![0x3000, name]]);
                assert_eq!(called(&e, STRING_CLEAR_00438390).len(), 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_007ab7a0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00a1_c520, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007a_b7a0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00a1_c520)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_57fc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_57fc,
            "the vtable was stored before the call"
        );
    }
}

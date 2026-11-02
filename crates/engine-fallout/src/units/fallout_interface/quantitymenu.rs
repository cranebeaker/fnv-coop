//! `fallout/interface/menus/quantitymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007abdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all six tile slots at +0x28 (one word each) are set: false at
/// the first null slot. The menu class is not confirmed here.
pub fn fn_007abdb0(e: &mut Engine, this: Ptr) -> bool {
    let mut index = 0u32;
    while index < 6 {
        let slot = this.addr().wrapping_add(0x28).wrapping_add(index * 4);
        if e.mem.u32(slot) == 0 {
            return false;
        }
        index += 1;
    }
    true
}

/// `QuantityMenu::pMe` (Xbox PDB static): the open menu; `fpResultCallback` (Xbox PDB static):
/// the `cdecl` function `DoClick` gives the chosen count to.
const QUANTITY_MENU: u32 = 0x011d_a618;
const RESULT_CALLBACK: u32 = 0x011d_a614;
/// The menu class number.
const QUANTITY_MENU_CLASS: u32 = 0x3f8;

/// The interface manager (`004b7210`), its tile reader (`00586150`), `Tile::ReadFile`
/// (`RET 4`), `Tile::GetMenu`, `Tile::GetMenuByClass` (`cdecl`), `Interface::GetTopMenuID`
/// (`007023c0`, no arguments), `Menu::SetMenuTile` (`RET 8`), the print function
/// (`cdecl`), `Tile::GetMaximumDepth` (`ST0`), `Menu::PrepForVisibility` (`RET 4`), the
/// word at +4 (`00726070`: a menu's tile), `Tile::GetChildByName` (`RET 4`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const TILE_READER_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const GET_TOP_MENU_ID: u32 = 0x0070_23c0;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const WORD_AT_FOUR: u32 = 0x0072_6070;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// Tile values: the float getter (`00a011b0`, `RET 4`, `ST0`), the float setter
/// (`00a012d0`, `RET 0xc`), the integer setter (`00700320`, `RET 8`), the string setter
/// (`00a01350`, `RET 0xc`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `BSAudio::QInstance` (no arguments), `BSAudio::GetSoundHandleByName` (`RET 0xc`: the
/// output handle, the name, the flags), `BSSoundHandle::Play` (`RET 4`), the sound handle's
/// destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `004a8f20` (`cdecl`, two words), `00647b70` (`cdecl`, two `int`s), `00403df0` and
/// `00403e20` (`thiscall` on a game setting: its string, its value's address), `00406d90`
/// (`cdecl`, a `float`), `00404010` and `0040ebd0` (`cdecl`, two `float`s, `ST0`), `00408820`
/// (`cdecl`, a `float`, `ST0`), `Interface::PlayMenuSound` (`cdecl`),
/// `Interface::GetCurrentRenderedMenu` (`00707ad0`, no arguments), `Menu::InstantFadeOut`
/// (`00a1d9e0`), `Menu::StartFadeOut` (`00a1d910`).
const INT_PAIR_004A8F20: u32 = 0x004a_8f20;
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
const MAGNITUDE_00408820: u32 = 0x0040_8820;
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const GET_CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
const MENU_INSTANT_FADE_OUT: u32 = 0x00a1_d9e0;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// The menu file, the creation failure message, `"UIPopUpMessageWindow"`,
/// `"QM_HowManyText"`, `"QM_Background"`.
const MENU_FILE_TEXT: u32 = 0x0107_58c4;
const CREATION_FAILED_TEXT: u32 = 0x0107_5870;
const POP_UP_SOUND_TEXT: u32 = 0x0107_5770;
const HOW_MANY_TEXT: u32 = 0x0107_5860;
const BACKGROUND_TEXT: u32 = 0x0107_5850;
/// The `double`s `6006.0`, `102.0` (the stacking types the depth is set for) and `255.0`,
/// the `float` `-1.0`.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const TWO_FIFTY_FIVE_DOUBLE: u32 = 0x0101_e568;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// The game settings: the question text (`011d3ac8`), two button texts (`011d38b8`,
/// `011d29f4`), the background alpha (`011d3bc8`).
const SETTING_011D3AC8: u32 = 0x011d_3ac8;
const SETTING_011D38B8: u32 = 0x011d_38b8;
const SETTING_011D29F4: u32 = 0x011d_29f4;
const SETTING_011D3BC8: u32 = 0x011d_3bc8;

/// The open menu.
fn quantity_me(e: &Engine) -> u32 {
    e.global::<u32>(QUANTITY_MENU)
}

/// The tile word at `offset` of the open menu.
fn quantity_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(quantity_me(e).wrapping_add(offset))
}

// Translated from 007aba00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007aba00 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `QuantityMenu::Create` (Xbox PDB, `cdecl`: count, callback, maximum). Deletes an open
/// menu of the class, notes whether a menu is on top (`Interface::GetTopMenuID`), reads the
/// menu file and takes its menu as `pMe`. For a menu of this class: attaches the tile;
/// with a tile missing prints the failure message and answers 0; otherwise plays
/// `"UIPopUpMessageWindow"`, sets the depth for the two stacking types, gives the tile at
/// +0x28 the count (`0x1004`), `004a8f20(maximum, count)` (`0x1006`), the step
/// `00647b70(count / 20, 1)` rounded down to tens above 10 (`0x1007`), the step
/// `00647b70(count / 4, 1)` (`0x1008`) and `0.0` (`0xff0`), keeps the callback, shows the
/// three setting texts and, with a menu on top, the background alpha (setting `011d3bc8`
/// times 255), and prepares the menu for showing. Returns the root tile. A menu of another
/// class is deleted (when it has a tile) and the result is 0.
pub fn quantity_menu_create(e: &mut Engine, count: i32, callback: u32, maximum: u32) -> u32 {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let class = fn_007abda0(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let on_top = e.call(GET_TOP_MENU_ID, &args![]).u32() != 0;
    let reader = e.call(TILE_READER_00586150, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(QUANTITY_MENU, menu);
    if quantity_me(e) != 0 {
        let class = e.vcall(quantity_me(e), 0x34, &args![]).u32();
        if class == fn_007abda0(e) {
            let menu = quantity_me(e);
            e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
            if !fn_007abdb0(e, Ptr::new(quantity_me(e))) {
                e.call(PRINT_MESSAGE, &args![CREATION_FAILED_TEXT]);
                return 0;
            }
            e.with_stack(0xc, |e, handle| {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let found = e
                    .call(
                        AUDIO_GET_SOUND_HANDLE_BY_NAME,
                        &args![audio, handle, POP_UP_SOUND_TEXT, 0x121u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            });
            // `FCOMP double; TEST AH,0x44`: equal (and ordered) to either stacking type.
            let kind = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
            let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
                let again = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
                again == e.global::<f64>(STACKING_TYPE_TWO)
            };
            if matches {
                let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
                e.call(
                    TILE_SET_FLOAT,
                    &args![root, 0xfadu32, x87_store32(depth), 1u32],
                );
            }
            let limit = e.call(INT_PAIR_004A8F20, &args![maximum, count]).u32();
            let mut step = e.call(INT_PAIR_00647B70, &args![count / 0x14, 1i32]).i32();
            let quarter = e.call(INT_PAIR_00647B70, &args![count / 4, 1i32]).i32();
            if step > 10 {
                step -= step % 10;
            }
            for (trait_id, value) in [
                (0x1004u32, count as u32),
                (0x1006, limit),
                (0x1007, step as u32),
                (0x1008, quarter as u32),
            ] {
                let tile = quantity_tile(e, 0x28);
                e.call(TILE_SET_INT, &args![tile, trait_id, value]);
            }
            let tile = quantity_tile(e, 0x28);
            e.call(
                TILE_SET_FLOAT,
                &args![tile, 0xff0u32, 0.0f32.to_bits(), 1u32],
            );
            e.set_global(RESULT_CALLBACK, callback);
            let text = e.call(SETTING_STRING, &args![SETTING_011D3AC8]).u32();
            let child = e
                .call(TILE_GET_CHILD_BY_NAME, &args![root, HOW_MANY_TEXT])
                .u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, text, 1u32]);
            for (setting, offset) in [(SETTING_011D38B8, 0x38u32), (SETTING_011D29F4, 0x3c)] {
                let text = e.call(SETTING_STRING, &args![setting]).u32();
                let tile = quantity_tile(e, offset);
                e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
            }
            if on_top {
                let value = e.call(SETTING_VALUE, &args![SETTING_011D3BC8]).u32();
                let total: f64 = e.global(TWO_FIFTY_FIVE_DOUBLE);
                let alpha = x87_store32(x87_mul(x87_m32(e.mem.u32(value)), total));
                let menu_tile = e.call(WORD_AT_FOUR, &args![quantity_me(e)]).u32();
                let background = e
                    .call(TILE_GET_CHILD_BY_NAME, &args![menu_tile, BACKGROUND_TEXT])
                    .u32();
                e.call(
                    TILE_SET_FLOAT,
                    &args![background, 0xfa9u32, x87_quiet_bits(alpha), 1u32],
                );
            }
            let menu = quantity_me(e);
            e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
            return root;
        }
    }
    if quantity_me(e) != 0
        && e.call(WORD_AT_FOUR, &args![quantity_me(e)]).u32() != 0
        && quantity_me(e) != 0
    {
        let menu = quantity_me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

// Translated from 007abda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x3f8`, without an object (no `this`).
pub fn fn_007abda0(_e: &mut Engine) -> u32 {
    QUANTITY_MENU_CLASS
}

// Translated from 007abdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::Close` (Xbox PDB, no arguments): with an open menu, gives its tile trait
/// `0x1772` the value 1 and fades it out, at once (`Menu::InstantFadeOut`) when
/// `Interface::GetCurrentRenderedMenu` answers, else `Menu::StartFadeOut`.
pub fn quantity_menu_close(e: &mut Engine) {
    if quantity_me(e) == 0 {
        return;
    }
    let tile = e.call(WORD_AT_FOUR, &args![quantity_me(e)]).u32();
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    let menu = quantity_me(e);
    if e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32() != 0 {
        e.call(MENU_INSTANT_FADE_OUT, &args![menu]);
    } else {
        e.call(MENU_START_FADE_OUT, &args![menu]);
    }
}

/// `DoClick`'s and `DoGamepad`'s step: the tile at +0x28 gets the current value (`0xfc4` of
/// the tile at +0x34) plus `delta`, kept at or above `0.0` (`00404010`) and at or below the
/// count (`0040ebd0` with trait `0x1004`), as `0x1006`, and `0.0` as `0xff0`.
fn step_value(e: &mut Engine, this: Ptr, current: u32, maximum: u32, delta: u32) {
    let sum = x87_store32(x87_add(x87_load32(current), x87_m32(delta)));
    let low = e
        .call(
            FLOAT_PAIR_00404010,
            &args![x87_quiet_bits(sum), 0.0f32.to_bits()],
        )
        .f64();
    let value = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_store32(low), x87_quiet_bits(maximum)],
        )
        .f64();
    let tile = e.mem.u32(this.addr() + 0x28);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0x1006u32, x87_store32(value), 1u32],
    );
    let tile = e.mem.u32(this.addr() + 0x28);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0xff0u32, 0.0f32.to_bits(), 1u32],
    );
}

// Translated from 007abe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::DoClick` (Xbox PDB; `RET 8`: id, tile; without a tile an id below 6 reads
/// `pTiles[id]`, unused). Reads the current value (`0xfc4` of the tile at +0x34) first. The
/// switch (table `007abf6c`): ids 1 and 2 step the value down and up by 1 (`-1.0` from
/// `01012054`); ids 4 and 5 call the callback (when there is one) with the value converted
/// by `00406d90` for 4 and 0 for 5, then close the menu; id 3 and others do nothing.
pub fn quantity_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let field = e.mem.u32(this.addr() + 0x34);
    let current = x87_store32(e.call(TILE_GET_FLOAT, &args![field, 0xfc4u32]).f64());
    let _tile = if tile == 0 && (0..6).contains(&id) {
        e.mem.u32(this.addr().wrapping_add(0x28 + 4 * id as u32))
    } else {
        tile
    };
    match id {
        4 | 5 => {
            if e.global::<u32>(RESULT_CALLBACK) != 0 {
                let chosen = if id == 4 {
                    e.call(FLOAT_TO_INT_00406D90, &args![x87_quiet_bits(current)])
                        .u32()
                } else {
                    0
                };
                let callback = e.global::<u32>(RESULT_CALLBACK);
                e.call(callback, &args![chosen]);
            }
            quantity_menu_close(e);
        }
        1 | 2 => {
            let tile = e.mem.u32(this.addr() + 0x28);
            let maximum = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64());
            let delta = if id == 2 {
                1.0f32.to_bits()
            } else {
                x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT))
            };
            step_value(e, this, current, maximum, delta);
        }
        _ => {}
    }
}

/// `DoGamepad`'s step: the value (`0xfc4` of the tile at +0x34), the count (`0x1004`) and
/// the step trait of the tile at +0x28, negated unless `up`.
fn gamepad_step(e: &mut Engine, this: Ptr, step_trait: u32, up: bool) {
    let field = e.mem.u32(this.addr() + 0x34);
    let current = x87_store32(e.call(TILE_GET_FLOAT, &args![field, 0xfc4u32]).f64());
    let tile = e.mem.u32(this.addr() + 0x28);
    let maximum = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64());
    let tile = e.mem.u32(this.addr() + 0x28);
    let step = x87_store32(e.call(TILE_GET_FLOAT, &args![tile, step_trait]).f64());
    let delta = if up {
        x87_quiet_bits(step)
    } else {
        // `FLD; FCHS; FSTP`: the sign flipped.
        x87_store32(-x87_load32(step))
    };
    step_value(e, this, current, maximum, delta);
}

// Translated from 007abf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::DoGamepad` (Xbox PDB; `RET 8`: a button code and an unused word). The
/// switch (table `007ac120`): codes 13 and 14 step the value down and up by trait `0x1008`,
/// codes 15 and 16 by trait `0x1007`. Returns whether the code was handled.
pub fn quantity_menu_do_gamepad(e: &mut Engine, this: Ptr, code: u32, _unused_2: u32) -> bool {
    match code {
        13 | 14 => {
            gamepad_step(e, this, 0x1008, code == 14);
            true
        }
        15 | 16 => {
            gamepad_step(e, this, 0x1007, code == 16);
            true
        }
        _ => false,
    }
}

// Translated from 007ac130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::DoGamepadUpEvent` (Xbox PDB; `RET 8`: a button code and an unused word):
/// code 10 clicks (vtable +0xc) id 5. Returns whether it did.
pub fn quantity_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    code: u32,
    _unused_2: u32,
) -> bool {
    if code == 10 {
        e.vcall(this.addr(), 0xc, &args![5u32, 0u32]);
        return true;
    }
    false
}

// Translated from 007ac170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::DoIdle` (Xbox PDB): when `00408820` of the change of the value (`0xfc4` of
/// the tile at +0x34) since `fPrevQuantity` (+0x40) is at or above the step (`0x1007` of
/// the tile at +0x28), plays menu sound 4 and takes the value as `fPrevQuantity`.
pub fn quantity_menu_do_idle(e: &mut Engine, this: Ptr) {
    let field = e.mem.u32(this.addr() + 0x34);
    let value = e.call(TILE_GET_FLOAT, &args![field, 0xfc4u32]).f64();
    let previous = e.mem.u32(this.addr() + 0x40);
    let change = x87_store32(x87_sub(value, x87_m32(previous)));
    let moved = e
        .call(MAGNITUDE_00408820, &args![x87_quiet_bits(change)])
        .f64();
    let tile = e.mem.u32(this.addr() + 0x28);
    let step = e.call(TILE_GET_FLOAT, &args![tile, 0x1007u32]).f64();
    // `FCOMP double; TEST AH,0x41; JP`: on when the step is at or below the change
    // (ordered).
    if step <= moved {
        e.call(INTERFACE_PLAY_MENU_SOUND, &args![4u32]);
        let field = e.mem.u32(this.addr() + 0x34);
        let value = e.call(TILE_GET_FLOAT, &args![field, 0xfc4u32]).f64();
        e.mem.set_u32(this.addr() + 0x40, x87_store32(value));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_bdb0, fn_007abdb0(Ptr) -> bool),
        entry!(0x007a_ba00, quantity_menu_create(i32, u32, u32) -> u32),
        entry!(0x007a_bda0, fn_007abda0() -> u32),
        entry!(0x007a_bdf0, quantity_menu_close()),
        entry!(0x007a_be40, quantity_menu_do_click(Ptr, i32, u32)),
        entry!(0x007a_bf80, quantity_menu_do_gamepad(Ptr, u32, u32) -> bool),
        entry!(0x007a_c130, quantity_menu_do_gamepad_up_event(Ptr, u32, u32) -> bool),
        entry!(0x007a_c170, quantity_menu_do_idle(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_only_with_all_six_tiles() {
        let mut e = Engine::new();
        let menu = e.mem.alloc(0x48);
        for i in 0..5u32 {
            e.mem.set_u32(menu + 0x28 + i * 4, 1);
        }
        assert!(!e.call(0x007a_bdb0, &args![menu]).bool());
        e.mem.set_u32(menu + 0x3c, 1);
        assert!(e.call(0x007a_bdb0, &args![menu]).bool());
        e.mem.set_u32(menu + 0x28, 0);
        assert!(!e.call(0x007a_bdb0, &args![menu]).bool());
    }
}

#[cfg(test)]
mod n057_tests {
    use super::*;

    /// Every callee outside this file the translations above reach.
    const EXTERNAL: &[u32] = &[
        0x00403df0, 0x00403e20, 0x00404010, 0x00406d90, 0x00408820, 0x0040ebd0, 0x00453a70,
        0x00483710, 0x004a8f20, 0x004b7210, 0x00586150, 0x005b5e40, 0x00647b70, 0x00700320,
        0x007023c0, 0x00706f30, 0x00707ad0, 0x00726070, 0x00a011b0, 0x00a012d0, 0x00a01350,
        0x00a01530, 0x00a01b00, 0x00a03c90, 0x00a03da0, 0x00a09030, 0x00a1d910, 0x00a1d9e0,
        0x00a1dc20, 0x00a1dc70, 0x00ad7550, 0x00ad8830,
    ];

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| eax.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for page in [0x011d_a000, 0x0107_0000, 0x0101_e000, 0x0101_2000] {
            e.map(page, 0x1000);
        }
        e
    }

    /// A quantity menu with its six tiles set and a vtable of doubles.
    fn menu(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0x44);
        e.put_vtable(0x0e00_0000, &[0x0000_8000; 0x20]);
        e.register(0x0000_8000, |_, _| Ret::default());
        e.mem.set_u32(menu, 0x0e00_0000);
        for i in 0..6 {
            e.mem.set_u32(menu + 0x28 + 4 * i, 0x2000 + 4 * i);
        }
        menu
    }

    #[test]
    fn create_sets_the_counts() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(0x0e00_0000 + 0x34, 0x0000_8034);
        answer(&mut e, 0x0000_8034, 0x3f8);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, menu);
        answer(&mut e, INT_PAIR_004A8F20, 77);
        e.register(INT_PAIR_00647B70, |_, a| {
            (a[0] as i32).max(a[1] as i32).into_ret()
        });
        answer(&mut e, GET_TOP_MENU_ID, 1);
        let alpha = e.mem.alloc(4);
        e.mem.set_u32(alpha, 0.5f32.to_bits());
        answer(&mut e, SETTING_VALUE, alpha);
        e.set_global(TWO_FIFTY_FIVE_DOUBLE, 255.0f64);
        answer(&mut e, TILE_GET_CHILD_BY_NAME, 0x2400);
        e.mem.set_u32(menu + 4, 0x2300);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007a_ba00, &args![250i32, 0x1234u32, 1000u32])
                .u32(),
            0x4444
        );
        assert_eq!(calls_to(&e, INT_PAIR_004A8F20), vec![vec![1000, 250]]);
        assert_eq!(
            calls_to(&e, INT_PAIR_00647B70),
            vec![vec![12, 1], vec![62, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x2000, 0x1004, 250],
                vec![0x2000, 0x1006, 77],
                vec![0x2000, 0x1007, 10],
                vec![0x2000, 0x1008, 62]
            ]
        );
        assert_eq!(e.global::<u32>(RESULT_CALLBACK), 0x1234);
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [POP_UP_SOUND_TEXT, 0x121]
        );
        assert_eq!(
            calls_to(&e, TILE_GET_CHILD_BY_NAME),
            vec![vec![0x4444, HOW_MANY_TEXT], vec![0x2300, BACKGROUND_TEXT]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT).last().unwrap(),
            &vec![0x2400, 0xfa9, 127.5f32.to_bits(), 1]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
    }

    #[test]
    fn create_fails_without_tiles_or_for_another_class() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(0x0e00_0000 + 0x34, 0x0000_8034);
        answer(&mut e, 0x0000_8034, 0x3f8);
        answer(&mut e, TILE_GET_MENU, menu);
        e.mem.set_u32(menu + 0x3c, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_ba00, &args![5i32, 0u32, 9u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        answer(&mut e, 0x0000_8034, 0x3f1);
        e.mem.set_u32(menu + 4, 0x2300);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_ba00, &args![5i32, 0u32, 9u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0000_8000), vec![vec![menu, 1]]);
        assert_eq!(e.call(0x007a_bda0, &args![]).u32(), 0x3f8);
    }

    #[test]
    fn close_fades_the_menu_out() {
        let mut e = engine();
        e.call(0x007a_bdf0, &args![]);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 4, 0x2300);
        e.set_global(QUANTITY_MENU, menu);
        e.call_log = Some(vec![]);
        e.call(0x007a_bdf0, &args![]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x2300, 0x1772, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        answer(&mut e, GET_CURRENT_RENDERED_MENU, 1);
        e.call_log = Some(vec![]);
        e.call(0x007a_bdf0, &args![]);
        assert_eq!(calls_to(&e, MENU_INSTANT_FADE_OUT), vec![vec![menu]]);
    }

    fn values(e: &mut Engine) {
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: match a[1] {
                0xfc4 => 4.0,
                0x1004 => 10.0,
                0x1007 => 5.0,
                _ => 3.0,
            },
            ..Ret::default()
        });
        e.register(FLOAT_PAIR_00404010, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
    }

    #[test]
    fn do_click_steps_and_answers() {
        let mut e = engine();
        values(&mut e);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007a_be40, &args![menu, 2i32, 0u32]);
        e.call(0x007a_be40, &args![menu, 1i32, 0u32]);
        e.call(0x007a_be40, &args![menu, 3i32, 0u32]);
        let set: Vec<_> = calls_to(&e, TILE_SET_FLOAT)
            .into_iter()
            .filter(|c| c[1] == 0x1006)
            .collect();
        assert_eq!(
            set,
            vec![
                vec![0x2000, 0x1006, 5.0f32.to_bits(), 1],
                vec![0x2000, 0x1006, 3.0f32.to_bits(), 1]
            ]
        );
        e.register(0x0000_7000, |_, _| Ret::default());
        e.set_global(RESULT_CALLBACK, 0x0000_7000u32);
        answer(&mut e, FLOAT_TO_INT_00406D90, 4);
        e.call_log = Some(vec![]);
        e.call(0x007a_be40, &args![menu, 4i32, 0u32]);
        e.call(0x007a_be40, &args![menu, 5i32, 0u32]);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![4], vec![0]]);
        assert_eq!(
            calls_to(&e, FLOAT_TO_INT_00406D90),
            vec![vec![4.0f32.to_bits()]]
        );
    }

    #[test]
    fn gamepad_steps_by_the_traits() {
        let mut e = engine();
        values(&mut e);
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007a_bf80, &args![menu, 14u32, 0u32]).bool());
        assert!(e.call(0x007a_bf80, &args![menu, 15u32, 0u32]).bool());
        assert!(!e.call(0x007a_bf80, &args![menu, 12u32, 0u32]).bool());
        let set: Vec<_> = calls_to(&e, TILE_SET_FLOAT)
            .into_iter()
            .filter(|c| c[1] == 0x1006)
            .collect();
        assert_eq!(
            set,
            vec![
                vec![0x2000, 0x1006, 7.0f32.to_bits(), 1],
                vec![0x2000, 0x1006, 0.0f32.to_bits(), 1]
            ]
        );
        assert!(e.call(0x007a_c130, &args![menu, 10u32, 0u32]).bool());
        assert!(!e.call(0x007a_c130, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x0000_8000), vec![vec![menu, 5, 0]]);
    }

    #[test]
    fn idle_ticks_on_a_large_enough_change() {
        let mut e = engine();
        values(&mut e);
        e.register(MAGNITUDE_00408820, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Ret::default()
        });
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x40, 1.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x007a_c170, &args![menu]);
        assert!(calls_to(&e, INTERFACE_PLAY_MENU_SOUND).is_empty());
        e.mem.set_u32(menu + 0x40, (-1.0f32).to_bits());
        e.call(0x007a_c170, &args![menu]);
        assert_eq!(calls_to(&e, INTERFACE_PLAY_MENU_SOUND), vec![vec![4]]);
        assert_eq!(e.mem.u32(menu + 0x40), 4.0f32.to_bits());
    }
}

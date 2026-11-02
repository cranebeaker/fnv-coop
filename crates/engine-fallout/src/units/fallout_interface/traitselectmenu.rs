//! `fallout/interface/menus/traitselectmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `TraitSelectMenu` (Xbox PDB, 0x88 bytes): eighteen tiles (`pTiles`, +0x28), the scroll
// (`mfCurrentYScroll` +0x70, `m_fDesiredYScroll` +0x74), the highlighted and checked
// tiles (`m_CurrentlySelectedTile` +0x78, `m_CurrentlyCheckedTile` +0x7c), the
// checkmark animation start (+0x80) and the last update (+0x84). `pMe` is `011db00c`.

/// `TraitSelectMenu::pMe` (`011db00c`, by its use) and the class number.
const TRAIT_SELECT_MENU_ME: u32 = 0x011d_b00c;
const TRAIT_SELECT_MENU_CLASS: u32 = 0x434;
/// `pTiles` (+0x28), 18 of them; the selected (+0x78) and checked (+0x7c) indices.
const TILES: u32 = 0x28;
const SELECTED: u32 = 0x78;
const CHECKED: u32 = 0x7c;
/// `TraitSelectMenu`'s vtable (`01077abc`).
const TRAIT_SELECT_MENU_VTABLE: u32 = 0x0107_7abc;

/// `MemoryManager` free (`00401030`), the `Menu` destructor body (`00a1c520`), the
/// interface manager (`004b7210`), the menu by class (`00a09030`, cdecl), the manager's
/// file reader (`00586150`), `Tile::ReadFile` (`00a01b00`, `RET 4`) of
/// `"Data\Menus\trait_select_menu.xml"` (`01077b04`), `Tile::GetMenu` (`00a03c90`),
/// `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), the failure message (`005b5e40`, cdecl;
/// `01071d58`, the companion wheel's text), `Menu::PrepForVisibility` (`00a1dc20`,
/// `RET 4`), `Menu::StartFadeOut` (`00a1d910`), the menu's tile (+4, `00726070`).
const FREE: u32 = 0x0040_1030;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const MENU_FILE_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_FILE_NAME: u32 = 0x0107_7b04;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const PRINT_FAILURE: u32 = 0x005b_5e40;
const CREATION_FAILED_TEXT: u32 = 0x0107_1d58;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const SECOND_WORD: u32 = 0x0072_6070;
/// The tile's float trait (`00a011b0`, `RET 4`, `ST0`) and setter (`00a012d0`, `RET
/// 0xc`), the integer setter (`00700320`, `RET 8`), `Tile::Poke` (`00a01290`, `RET 8`),
/// `Tile::TextToTrait` (`00a01860`, cdecl) of `"_current_value"` (`010702d8`) and
/// `"_SetInCode"` (`01070250`), `Tile::GetMaximumDepth` (`00a01530`), the depth traits
/// 6006.0 (`010704f0`) and 102.0 (`010704e8`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_POKE: u32 = 0x00a0_1290;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const CURRENT_VALUE_NAME: u32 = 0x0107_02d8;
const SET_IN_CODE_NAME: u32 = 0x0107_0250;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const DEPTH_TRAIT_6006: u32 = 0x0107_04f0;
const DEPTH_TRAIT_102: u32 = 0x0107_04e8;
/// Sounds: `BSAudio::QInstance` (`00453a70`), `GetSoundHandleByName` (`00ad7550`, `RET
/// 0xc`), `BSSoundHandle::Play` (`00ad8830`, `RET 4`), the handle's destructor
/// (`00483710`); `"UIPopUpMessageGeneral"` (`01025cc4`), `"UITraitSelectErase"`
/// (`01077b40`), `"UITraitSelectCheckmark"` (`01077b28`); `Interface::PlayMenuSound`
/// (`00706f30`, cdecl) and `InterfaceManager::PlayMenuSound` (`00717280`, cdecl).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const POPUP_SOUND: u32 = 0x0102_5cc4;
const ERASE_SOUND: u32 = 0x0107_7b40;
const CHECKMARK_SOUND: u32 = 0x0107_7b28;
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const MANAGER_PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// The clock (`00457fe0`), whether a gamepad is in use (`004b71d0`, no words) and the
/// manager's `00755680` (`thiscall`), the manager's wheel delta (`0070ec70`), the left
/// stick's axes (`011d8a5e`, `011d8a62`, 16-bit) and `abs` (`00ec7d40`, cdecl).
const CLOCK: u32 = 0x0045_7fe0;
const GAMEPAD_IN_USE_004B71D0: u32 = 0x004b_71d0;
const MANAGER_00755680: u32 = 0x0075_5680;
const WHEEL_DELTA_0070EC70: u32 = 0x0070_ec70;
const STICK_X_011D8A5E: u32 = 0x011d_8a5e;
const STICK_Y_011D8A62: u32 = 0x011d_8a62;
const ABS: u32 = 0x00ec_7d40;
/// `TraitSelectMenu::SetBoxAsChecked`'s companion `007e8840` (unit run_007e8840),
/// the object `011ddf38` and its flag (`0042ce10`), `ProcessLists::Update3DAfterResting`
/// (`00975d10`) on the process lists (`011e0e80`).
const UNCHECK_007E8840: u32 = 0x007e_8840;
const FLAGS_OBJECT_011DDF38: u32 = 0x011d_df38;
const FLAG_0042CE10: u32 = 0x0042_ce10;
const UPDATE_3D_AFTER_RESTING: u32 = 0x0097_5d10;
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// Constants: 10.0 (`01020758`), 0.1 (`0101ffa0`), 0.001 (`01016978`), 1.0
/// (`01012070`), 0.0 (`01012060`) and 8.0 (`010240b0`) as `double`s; 48.0 (`0101e340`),
/// 64.0 (`0101e6ec`), 0.05 (`010181bc`) and -0.05 (`01050e24`) as `float`s.
const TEN_DOUBLE: u32 = 0x0102_0758;
const TENTH_DOUBLE: u32 = 0x0101_ffa0;
const THOUSANDTH_DOUBLE: u32 = 0x0101_6978;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const EIGHT_DOUBLE: u32 = 0x0102_40b0;
const CHECKMARK_48: u32 = 0x0101_e340;
const CHECKMARK_64: u32 = 0x0101_e6ec;
const SCROLL_UP_STEP: u32 = 0x0101_81bc;
const SCROLL_DOWN_STEP: u32 = 0x0105_0e24;

/// The tile at `index` (`pTiles`).
fn tile_at(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(TILES)
            .wrapping_add(index.wrapping_mul(4)),
    )
}

/// Whether a gamepad is in use (`004b71d0`) and the manager's `00755680` answers false.
fn gamepad_active(e: &mut Engine) -> bool {
    e.call(INTERFACE_MANAGER, &args![]);
    if !e.call(GAMEPAD_IN_USE_004B71D0, &args![]).bool() {
        return false;
    }
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    !e.call(MANAGER_00755680, &args![manager]).bool()
}

/// Plays a sound by name through the 12-byte handle local at `handle`.
fn play_sound(e: &mut Engine, handle: u32, name: u32) {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let sound = e
        .call(
            GET_SOUND_HANDLE_BY_NAME,
            &args![audio, handle, name, 0x121u32],
        )
        .u32();
    e.call(SOUND_PLAY, &args![sound, 0u32]);
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
}

/// A tile's float trait (`00a011b0`) stored as a `float`.
fn tile_float(e: &mut Engine, tile: u32, which: u32) -> u32 {
    let value = e.call(TILE_GET_FLOAT, &args![tile, which]).f64();
    x87_store32(value)
}

// Translated from 007e7ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::GetClass` (Xbox PDB): the class number `0x434`.
pub fn trait_select_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    TRAIT_SELECT_MENU_CLASS
}

// Translated from 007e7ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::AttachTileByID` (Xbox PDB, `RET 8`): ids 0 to 17 store the tile in
/// `pTiles`; others are ignored.
pub fn trait_select_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0x12).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + 4 * id as u32, tile);
    }
}

// Translated from 007e7b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::`scalar deleting destructor'` (Xbox PDB, `RET 4`): the destructor
/// (`007e7b30`), then the block freed when bit 0 of `flags` is set. Answers `this`.
pub fn trait_select_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007e7b30(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007e7b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TraitSelectMenu` destructor body (unnamed in the engine map; `thiscall`): the
/// vtable, `pMe` cleared, then the `Menu` destructor.
pub fn fn_007e7b30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TRAIT_SELECT_MENU_VTABLE);
    e.set_global(TRAIT_SELECT_MENU_ME, 0u32);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 007e7b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e7b60 - the C++ exception-unwinding frame (FS:[0] around the sound handle) and the /GS cookie check (00ec408c) are not translated.
/// `TraitSelectMenu::Create` (Xbox PDB; cdecl, no words): closes an open trait select
/// menu (slot 0 with 1), reads the menu file and, when it makes a trait select menu:
/// the menu tile; without all tiles prints the failure text (the companion wheel's) and
/// answers 0; otherwise the scroll 0.0, the first tile highlighted
/// (`SetButtonAsHighlighted(0, 0)`), the last update now, the popup sound, the depth
/// (when trait 0xfa7 is 6006 or 102) and the menu shown; answers the file's tile. A
/// menu of another class is closed when `00726070` answers non-zero, and 0 answered.
pub fn trait_select_menu_create(e: &mut Engine) -> u32 {
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let class = fn_007e7d90(e);
    let open = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if open != 0 {
        e.vcall(open, 0, &args![1u32]);
    }
    let file = e.call(MENU_FILE_00586150, &args![manager]).u32();
    let tile = e.call(TILE_READ_FILE, &args![file, MENU_FILE_NAME]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(TRAIT_SELECT_MENU_ME, menu);
    let me = |e: &Engine| e.global::<u32>(TRAIT_SELECT_MENU_ME);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_007e7d90(e) {
            e.call(MENU_SET_MENU_TILE, &args![me(e), tile, 0u32]);
            if !fn_007e7da0(e, Ptr::new(me(e))) {
                e.call(PRINT_FAILURE, &args![CREATION_FAILED_TEXT]);
                return 0;
            }
            let zero = x87_store32(0.0);
            e.mem.set_u32(me(e) + 0x70, zero);
            e.mem.set_u32(me(e) + 0x74, zero);
            trait_select_menu_set_button_as_highlighted(e, Ptr::new(me(e)), 0, 0);
            let now = e.call(CLOCK, &args![]).u32();
            e.mem.set_u32(me(e) + 0x84, now);
            // The handle local at `EBP - 0x2c`.
            e.with_stack(0xc, |e, handle| play_sound(e, handle.addr(), POPUP_SOUND));
            let depth = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
            // FCOMP; TEST AH,0x44; JNP: equal (NaN is not).
            let deep = depth == e.global::<f64>(DEPTH_TRAIT_6006) || {
                let depth = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
                depth == e.global::<f64>(DEPTH_TRAIT_102)
            };
            if deep {
                let maximum = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
                e.call(
                    TILE_SET_FLOAT,
                    &args![tile, 0xfadu32, x87_store32(maximum), 1u32],
                );
            }
            e.call(MENU_PREP_FOR_VISIBILITY, &args![me(e), 0u32]);
            return tile;
        }
    }
    if me(e) != 0 && e.call(SECOND_WORD, &args![me(e)]).u32() != 0 && me(e) != 0 {
        e.vcall(me(e), 0, &args![1u32]);
    }
    0
}

// Translated from 007e7d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): the trait select menu's class number `0x434`.
pub fn fn_007e7d90(_e: &mut Engine) -> u32 {
    TRAIT_SELECT_MENU_CLASS
}

// Translated from 007e7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): whether all eighteen tiles are attached.
pub fn fn_007e7da0(e: &mut Engine, this: Ptr) -> bool {
    (0..0x12).all(|i| tile_at(e, this, i) != 0)
}

// Translated from 007e7de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::Close` (Xbox PDB; no words): with the menu open, updates the 3D
/// after resting (`ProcessLists::Update3DAfterResting`) unless the object `011ddf38`
/// has flag 2 (`0042ce10`), sets the menu tile's trait 0x1772 to 1 and fades the menu
/// out.
pub fn trait_select_menu_close(e: &mut Engine) {
    if e.global::<u32>(TRAIT_SELECT_MENU_ME) == 0 {
        return;
    }
    let object = e.global::<u32>(FLAGS_OBJECT_011DDF38);
    if !e.call(FLAG_0042CE10, &args![object]).bool() {
        e.call(UPDATE_3D_AFTER_RESTING, &args![PROCESS_LISTS]);
    }
    let me = e.global::<u32>(TRAIT_SELECT_MENU_ME);
    let tile = e.call(SECOND_WORD, &args![me]).u32();
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    let me = e.global::<u32>(TRAIT_SELECT_MENU_ME);
    e.call(MENU_START_FADE_OUT, &args![me]);
}

// Translated from 007e7e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::DoClick` (Xbox PDB, `RET 8`: the id and the tile; a missing tile
/// for ids 0 to 17 would be taken from `pTiles`, but it is not used): ids 0 to 12 check
/// that box (`007e8860`); id 14 plays menu sound 1 and closes the menu.
pub fn trait_select_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let mut _tile = tile;
    if _tile == 0 && (0..0x12).contains(&id) {
        _tile = tile_at(e, this, id as u32);
    }
    if (0..=0xc).contains(&id) {
        fn_007e8860(e, this, id);
    } else if id == 0xe {
        e.call(MANAGER_PLAY_MENU_SOUND, &args![1u32]);
        trait_select_menu_close(e);
    }
}

// Translated from 007e7ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::DoIdle` (Xbox PDB): with the scroll bar tile (13, +0x5c) whose
/// `"_current_value"` is not the scroll times 10, scrolls by its value times 0.1 less
/// the scroll. With a gamepad the left stick's axes outside their dead zones (0x1ea9,
/// 0x21f1) are read (not used further). With a box checked and the checkmark tile (17,
/// +0x6c), animates the checkmark's traits 0xfc0 and 0xfbf by the time since it was
/// checked (0 then 48, then 64 and 0, then 64 and 48 at 50, 100 and 150 ms). With a
/// gamepad the last update becomes now and the scroll moves toward the desired one by
/// at most 0.001 per millisecond (`ScrollForm`).
pub fn trait_select_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let scroll_bar = |e: &Engine| e.mem.u32(base + 0x5c);
    if scroll_bar(e) != 0 {
        let current = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_NAME]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![scroll_bar(e), current]).f64();
        let shown = x87_mul(
            x87_load32(e.mem.u32(base + 0x70)),
            e.global::<f64>(TEN_DOUBLE),
        );
        // FUCOMPP; TEST AH,0x44; JNP: nothing when equal.
        if shown != value {
            let value = e.call(TILE_GET_FLOAT, &args![scroll_bar(e), current]).f64();
            let target = x87_mul(value, e.global::<f64>(TENTH_DOUBLE));
            let delta = x87_store32(x87_sub(target, x87_m32(e.mem.u32(base + 0x70))));
            trait_select_menu_scroll_form(e, this, f32::from_bits(x87_quiet_bits(delta)));
        }
    }
    if gamepad_active(e) && gamepad_active(e) {
        let x = i32::from(e.global::<u16>(STICK_X_011D8A5E) as i16);
        let _moved_x = e.call(ABS, &args![x]).i32() > 0x1ea9;
        let y = i32::from(e.global::<u16>(STICK_Y_011D8A62) as i16);
        let _moved_y = e.call(ABS, &args![y]).i32() > 0x21f1;
    }
    let checkmark = |e: &Engine| e.mem.u32(base + 0x6c);
    if e.mem.i32(base + CHECKED) != -1 && checkmark(e) != 0 {
        let now = e.call(CLOCK, &args![]).u32();
        let since = now.wrapping_sub(e.mem.u32(base + 0x80));
        let zero = x87_store32(0.0);
        let low = x87_quiet_bits(e.global::<u32>(CHECKMARK_48));
        let high = x87_quiet_bits(e.global::<u32>(CHECKMARK_64));
        let (first, second) = if since < 0x32 {
            (zero, zero)
        } else if since < 0x64 {
            (zero, low)
        } else if since < 0x96 {
            (high, zero)
        } else {
            (high, low)
        };
        e.call(TILE_SET_FLOAT, &args![checkmark(e), 0xfc0u32, first, 1u32]);
        e.call(TILE_SET_FLOAT, &args![checkmark(e), 0xfbfu32, second, 1u32]);
    }
    if !gamepad_active(e) {
        return;
    }
    let now = e.call(CLOCK, &args![]).u32();
    let since = now.wrapping_sub(e.mem.u32(base + 0x84));
    let now = e.call(CLOCK, &args![]).u32();
    e.mem.set_u32(base + 0x84, now);
    let desired = x87_load32(e.mem.u32(base + 0x74));
    // FUCOMPP of the scroll against the desired one; TEST AH,0x44; JNP: equal stops.
    if x87_load32(e.mem.u32(base + 0x70)) == desired {
        return;
    }
    let step = x87_store32(x87_mul(
        f64::from(since),
        e.global::<f64>(THOUSANDTH_DOUBLE),
    ));
    let mut delta = x87_store32(x87_sub(desired, x87_m32(e.mem.u32(base + 0x70))));
    // FCOMPP: the step below the delta caps it; the delta below minus the step raises it.
    if x87_load32(step) < x87_load32(delta) {
        delta = x87_quiet_bits(step);
    }
    if x87_load32(delta) < -x87_load32(step) {
        delta = x87_store32(-x87_load32(step));
    }
    trait_select_menu_scroll_form(e, this, f32::from_bits(x87_quiet_bits(delta)));
}

// Translated from 007e81b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::DoGamepad` (Xbox PDB, `RET 8`: the button and a word not read;
/// tables `007e8258`/`007e826c`): 1 navigates up, 2 down; 9 checks the highlighted box
/// (0 to 12, `007e8860`) or clicks the done button (14, slot +0xc with 14 and tile 14);
/// 10 is only handled. Answers whether the button was handled.
pub fn trait_select_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr,
    button: u32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    match button.wrapping_sub(1) {
        0 => {
            trait_select_menu_navigate(e, this, 1);
            true
        }
        1 => {
            trait_select_menu_navigate(e, this, 0);
            true
        }
        8 => {
            let selected = e.mem.i32(base + SELECTED);
            if (0..=0xc).contains(&selected) {
                fn_007e8860(e, this, selected);
            } else if selected == 0xe {
                let tile = e.mem.u32(base + 0x60);
                e.vcall(base, 0xc, &args![0xeu32, tile]);
            }
            true
        }
        9 => true,
        _ => false,
    }
}

// Translated from 007e8280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: an id and a tile): a missing tile for ids 0 to 17 is
/// taken from `pTiles` into the argument, and nothing else is done.
pub fn fn_007e8280(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let mut _tile = tile;
    if _tile == 0 && (0..0x12).contains(&id) {
        _tile = tile_at(e, this, id as u32);
    }
}

// Translated from 007e82b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::DoWheelMove` (Xbox PDB, `RET 8`: two words not read): scrolls by
/// 0.05 for a negative wheel delta (`0070ec70` of the interface manager) and by -0.05
/// for a positive one.
pub fn trait_select_menu_do_wheel_move(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    if e.call(WHEEL_DELTA_0070EC70, &args![manager]).i32() < 0 {
        let step = x87_quiet_bits(e.global::<u32>(SCROLL_UP_STEP));
        trait_select_menu_scroll_form(e, this, f32::from_bits(step));
    }
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    if e.call(WHEEL_DELTA_0070EC70, &args![manager]).i32() > 0 {
        let step = x87_quiet_bits(e.global::<u32>(SCROLL_DOWN_STEP));
        trait_select_menu_scroll_form(e, this, f32::from_bits(step));
    }
}

// Translated from 007e8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::ScrollForm` (Xbox PDB, `RET 4`: the step): the scroll (+0x70) moves
/// by the step, kept within 0 and 1. With the menu found by class: the list tile (15,
/// +0x64) is placed at its trait 0x1004 less the scroll times 0x1008 (trait 0xfa2),
/// menu sound 4 plays, and the scroll bar tile (13, +0x5c) gets `"_current_value"` the
/// scroll times 10 and a poke of `"_SetInCode"`. The traits 0x1005 to 0x1007 are read and
/// not used.
pub fn trait_select_menu_scroll_form(e: &mut Engine, this: Ptr, step: f32) {
    let base = this.addr();
    let scroll = x87_store32(x87_add(
        x87_load32(e.mem.u32(base + 0x70)),
        x87_m32(step.to_bits()),
    ));
    e.mem.set_u32(base + 0x70, scroll);
    // FCOMP with 1.0; TEST AH,0x41: above 1 only.
    if x87_load32(e.mem.u32(base + 0x70)) > e.global::<f64>(ONE_DOUBLE) {
        e.mem.set_u32(base + 0x70, x87_store32(1.0));
    }
    // FCOMP with 0.0; TEST AH,5: below 0 only.
    if x87_load32(e.mem.u32(base + 0x70)) < e.global::<f64>(ZERO_DOUBLE) {
        e.mem.set_u32(base + 0x70, x87_store32(0.0));
    }
    let menu = e
        .call(GET_MENU_BY_CLASS, &args![TRAIT_SELECT_MENU_CLASS])
        .u32();
    if menu == 0 {
        return;
    }
    let top = tile_float(e, menu, 0x1004);
    let _unused_1005 = tile_float(e, menu, 0x1005);
    let _unused_1006 = tile_float(e, menu, 0x1006);
    let _unused_1007 = tile_float(e, menu, 0x1007);
    let spacing = tile_float(e, menu, 0x1008);
    let list = e.mem.u32(base + 0x64);
    if list != 0 {
        let offset = x87_mul(x87_load32(e.mem.u32(base + 0x70)), x87_m32(spacing));
        let place = x87_store32(x87_sub(x87_m32(top), offset));
        e.call(
            TILE_SET_FLOAT,
            &args![list, 0xfa2u32, x87_quiet_bits(place), 1u32],
        );
    }
    e.call(PLAY_MENU_SOUND, &args![4u32]);
    if e.mem.u32(base + 0x5c) != 0 {
        let current = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_NAME]).u32();
        let value = x87_store32(x87_mul(
            x87_load32(e.mem.u32(base + 0x70)),
            e.global::<f64>(TEN_DOUBLE),
        ));
        let bar = e.mem.u32(base + 0x5c);
        e.call(
            TILE_SET_FLOAT,
            &args![bar, current, x87_quiet_bits(value), 0u32],
        );
        let set_in_code = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_NAME]).u32();
        let bar = e.mem.u32(base + 0x5c);
        e.call(TILE_POKE, &args![bar, set_in_code, x87_store32(1.0)]);
    }
}

// Translated from 007e8470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::SetButtonAsHighlighted` (Xbox PDB, `RET 8`: the index and a `bool`
/// to force it): nothing when already highlighted unless forced. The old tile's trait
/// 0x1005 is cleared, the new index kept and (unless -1) its tile's 0x1005 set. With a
/// gamepad and the menu and list tile (+0x64): the desired scroll (+0x74) is the tile's
/// trait 0xfa2 over the list's 0xfb0, or 0 when that is 0.
pub fn trait_select_menu_set_button_as_highlighted(
    e: &mut Engine,
    this: Ptr,
    index: i32,
    force: u8,
) {
    let base = this.addr();
    let selected = |e: &Engine| e.mem.i32(base + SELECTED);
    if force == 0 && selected(e) == index {
        return;
    }
    if selected(e) != -1 && tile_at(e, this, selected(e) as u32) != 0 {
        let tile = tile_at(e, this, selected(e) as u32);
        e.call(TILE_SET_INT, &args![tile, 0x1005u32, 0u32]);
    }
    e.mem.set_i32(base + SELECTED, index);
    if selected(e) == -1 {
        return;
    }
    if tile_at(e, this, selected(e) as u32) != 0 {
        let tile = tile_at(e, this, selected(e) as u32);
        e.call(TILE_SET_INT, &args![tile, 0x1005u32, 1u32]);
    }
    if !gamepad_active(e) {
        return;
    }
    let menu = e
        .call(GET_MENU_BY_CLASS, &args![TRAIT_SELECT_MENU_CLASS])
        .u32();
    if menu == 0 || e.mem.u32(base + 0x64) == 0 {
        return;
    }
    let _spacing = tile_float(e, menu, 0x1008);
    let list = e.mem.u32(base + 0x64);
    let height = tile_float(e, list, 0xfb0);
    let tile = tile_at(e, this, selected(e) as u32);
    let position = tile_float(e, tile, 0xfa2);
    // FCOMP with 0.0; TEST AH,0x44; JNP: a zero height gives 0.
    if x87_load32(height) == e.global::<f64>(ZERO_DOUBLE) {
        e.mem.set_u32(base + 0x74, x87_store32(0.0));
    } else {
        let desired = x87_store32(x87_div(x87_load32(position), x87_m32(height)));
        e.mem.set_u32(base + 0x74, desired);
    }
}

// Translated from 007e85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitSelectMenu::Navigate` (Xbox PDB, `RET 4`: a `bool` for up; tables `007e8680`
/// and `007e86a0`): up from a box moves to the one above (none from 0) and from the done
/// button (14) to box 12; down moves to the next box, from 12 to the done button (none
/// from 14); tile 13 and anything past 14 go to box 0 (`SetButtonAsHighlighted(.., 0)`).
pub fn trait_select_menu_navigate(e: &mut Engine, this: Ptr, up: u8) {
    let selected = e.mem.i32(this.addr() + SELECTED);
    let target = if up != 0 {
        match selected as u32 {
            0 => None,
            1..=0xc => Some(selected.wrapping_sub(1)),
            0xe => Some(0xc),
            _ => Some(0),
        }
    } else {
        match selected as u32 {
            0..=0xb => Some(selected.wrapping_add(1)),
            0xc => Some(0xe),
            0xe => None,
            _ => Some(0),
        }
    };
    if let Some(index) = target {
        trait_select_menu_set_button_as_highlighted(e, this, index, 0);
    }
}

// Translated from 007e86c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e86c0 - the C++ exception-unwinding frame (FS:[0] around the sound handles) is not translated.
/// `TraitSelectMenu::SetBoxAsChecked` (Xbox PDB, `RET 8`: the index and a `bool` to
/// force it): nothing when already checked unless forced. The checkmark animation starts
/// now (+0x80); with the checkmark tile (17, +0x6c): index -1 hides it (0xfa3) and plays
/// `"UITraitSelectErase"`; a box with a tile shows it, places it 8 above the box (trait
/// 0xfa2) and plays `"UITraitSelectCheckmark"`. The index is kept (+0x7c).
pub fn trait_select_menu_set_box_as_checked(e: &mut Engine, this: Ptr, index: i32, force: u8) {
    let base = this.addr();
    if force == 0 && e.mem.i32(base + CHECKED) == index {
        return;
    }
    let now = e.call(CLOCK, &args![]).u32();
    e.mem.set_u32(base + 0x80, now);
    let checkmark = |e: &Engine| e.mem.u32(base + 0x6c);
    if checkmark(e) != 0 {
        if index == -1 {
            e.call(TILE_SET_INT, &args![checkmark(e), 0xfa3u32, 0u32]);
            // The handle local at `EBP - 0x18`.
            e.with_stack(0xc, |e, handle| play_sound(e, handle.addr(), ERASE_SOUND));
        } else if tile_at(e, this, index as u32) != 0 {
            e.call(TILE_SET_INT, &args![checkmark(e), 0xfa3u32, 1u32]);
            let tile = tile_at(e, this, index as u32);
            let position = e.call(TILE_GET_FLOAT, &args![tile, 0xfa2u32]).f64();
            let place = x87_store32(x87_sub(position, e.global::<f64>(EIGHT_DOUBLE)));
            e.call(
                TILE_SET_FLOAT,
                &args![checkmark(e), 0xfa2u32, x87_quiet_bits(place), 1u32],
            );
            // The handle local at `EBP - 0x24`.
            e.with_stack(0xc, |e, handle| {
                play_sound(e, handle.addr(), CHECKMARK_SOUND)
            });
        }
    }
    e.mem.set_i32(base + CHECKED, index);
}

// Translated from 007e8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a box index): unchecks (`007e8840`) when it is the
/// checked box, else checks it (`SetBoxAsChecked(index, 0)`).
pub fn fn_007e8860(e: &mut Engine, this: Ptr, index: i32) {
    if e.mem.i32(this.addr() + CHECKED) == index {
        e.call(UNCHECK_007E8840, &args![this]);
    } else {
        trait_select_menu_set_box_as_checked(e, this, index, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007e_7ac0, trait_select_menu_get_class(Ptr) -> u32),
        entry!(
            0x007e_7ad0,
            trait_select_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(
            0x007e_7b00,
            trait_select_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007e_7b30, fn_007e7b30(Ptr)),
        entry!(0x007e_7b60, trait_select_menu_create() -> u32),
        entry!(0x007e_7d90, fn_007e7d90() -> u32),
        entry!(0x007e_7da0, fn_007e7da0(Ptr) -> bool),
        entry!(0x007e_7de0, trait_select_menu_close()),
        entry!(0x007e_7e30, trait_select_menu_do_click(Ptr, i32, u32)),
        entry!(0x007e_7ea0, trait_select_menu_do_idle(Ptr)),
        entry!(
            0x007e_81b0,
            trait_select_menu_do_gamepad(Ptr, u32, u32) -> bool
        ),
        entry!(0x007e_8280, fn_007e8280(Ptr, i32, u32)),
        entry!(0x007e_82b0, trait_select_menu_do_wheel_move(Ptr, u32, u32)),
        entry!(0x007e_8310, trait_select_menu_scroll_form(Ptr, f32)),
        entry!(
            0x007e_8470,
            trait_select_menu_set_button_as_highlighted(Ptr, i32, u8)
        ),
        entry!(0x007e_85c0, trait_select_menu_navigate(Ptr, u8)),
        entry!(
            0x007e_86c0,
            trait_select_menu_set_box_as_checked(Ptr, i32, u8)
        ),
        entry!(0x007e_8860, fn_007e8860(Ptr, i32)),
    ]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    const STUBBED: &[u32] = &[
        FREE,
        MENU_DESTRUCT,
        INTERFACE_MANAGER,
        GET_MENU_BY_CLASS,
        MENU_FILE_00586150,
        TILE_READ_FILE,
        TILE_GET_MENU,
        MENU_SET_MENU_TILE,
        PRINT_FAILURE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        TILE_GET_FLOAT,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_POKE,
        TILE_GET_MAXIMUM_DEPTH,
        AUDIO_INSTANCE,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_PLAY,
        SOUND_HANDLE_DESTRUCT,
        PLAY_MENU_SOUND,
        MANAGER_PLAY_MENU_SOUND,
        CLOCK,
        GAMEPAD_IN_USE_004B71D0,
        MANAGER_00755680,
        WHEEL_DELTA_0070EC70,
        UNCHECK_007E8840,
        FLAG_0042CE10,
        UPDATE_3D_AFTER_RESTING,
        ABS,
    ];

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_b000,
            0x011d_d000,
            0x011d_8000,
            0x0101_2000,
            0x0102_0000,
            0x0101_f000,
            0x0101_6000,
            0x0102_4000,
            0x0101_e000,
            0x0101_8000,
            0x0105_0000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TEN_DOUBLE, 10.0f64);
        e.set_global(TENTH_DOUBLE, 0.1f64);
        e.set_global(THOUSANDTH_DOUBLE, 0.001f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(EIGHT_DOUBLE, 8.0f64);
        e.set_global(CHECKMARK_48, 48.0f32);
        e.set_global(CHECKMARK_64, 64.0f32);
        e.set_global(SCROLL_UP_STEP, 0.05f32);
        e.set_global(SCROLL_DOWN_STEP, -0.05f32);
        e.set_global(DEPTH_TRAIT_6006, 6006.0f64);
        e.set_global(DEPTH_TRAIT_102, 102.0f64);
        for &address in STUBBED {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(SECOND_WORD, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let slots: Vec<u32> = (0..0x10).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    /// A menu with all eighteen tiles (`0x1000 + index` for the tile at index).
    fn menu(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this, VTABLE);
        for i in 0..0x12u32 {
            e.call(0x007e_7ad0, &args![this, i, 0x1000 + i]);
        }
        e.mem.set_i32(this + SELECTED, -1);
        e.mem.set_i32(this + CHECKED, -1);
        Ptr::new(this)
    }

    #[test]
    fn class_tiles_and_destruction() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x007e_7ac0, &args![this]).u32(), 0x434);
        assert_eq!(e.call(0x007e_7d90, &args![]).u32(), 0x434);
        assert!(e.call(0x007e_7da0, &args![this]).bool());
        e.call(0x007e_7ad0, &args![this, 0x12i32, 0u32]);
        assert!(e.call(0x007e_7da0, &args![this]).bool());
        e.call(0x007e_7ad0, &args![this, 0x11i32, 0u32]);
        assert!(!e.call(0x007e_7da0, &args![this]).bool());
        e.set_global(TRAIT_SELECT_MENU_ME, this.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_7b00, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), TRAIT_SELECT_MENU_VTABLE);
        assert_eq!(e.global::<u32>(TRAIT_SELECT_MENU_ME), 0);
        assert_eq!(calls(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, FREE), vec![vec![this.addr()]]);
        // The tile-only function reads a tile and changes nothing.
        e.call(0x007e_8280, &args![this, 3i32, 0u32]);
    }

    #[test]
    fn create_sets_up_and_close_fades_out() {
        let mut e = engine();
        let this = menu(&mut e);
        answer(&mut e, TILE_READ_FILE, 0x6600);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer(&mut e, slot(0x34), 0x434);
        answer(&mut e, CLOCK, 777);
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 6006.0,
            ..Ret::default()
        });
        e.mem.set_f32(this.addr() + 0x70, 0.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_7b60, &args![]).u32(), 0x6600);
        assert_eq!(e.mem.f32(this.addr() + 0x70), 0.0);
        assert_eq!(e.mem.i32(this.addr() + SELECTED), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x84), 777);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], POPUP_SOUND);
        assert_eq!(calls(&e, TILE_SET_FLOAT)[0][..2], [0x6600, 0xfad]);
        assert_eq!(
            calls(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
        // Close: the 3D update (no flag), the menu tile trait 0x1772, the fade.
        e.mem.set_u32(this.addr() + 4, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x007e_7de0, &args![]);
        assert_eq!(
            calls(&e, UPDATE_3D_AFTER_RESTING),
            vec![vec![PROCESS_LISTS]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x4444, 0x1772, 1]]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
        // Without all tiles: the failure text and 0.
        e.call(0x007e_7ad0, &args![this, 5i32, 0u32]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_7b60, &args![]).u32(), 0);
        assert_eq!(calls(&e, PRINT_FAILURE), vec![vec![CREATION_FAILED_TEXT]]);
    }

    #[test]
    fn clicks_check_boxes_and_the_gamepad_navigates() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: f64::from(a[0] & 0xff),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007e_7e30, &args![this, 3i32, 0u32]);
        assert_eq!(e.mem.i32(base + CHECKED), 3);
        // Tile 3 (0x1003) at 3 places the checkmark at -5.
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x1011, 0xfa2, (-5.0f32).to_bits(), 1]]
        );
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], CHECKMARK_SOUND);
        // The same box again unchecks it through 007e8840.
        e.call(0x007e_7e30, &args![this, 3i32, 0u32]);
        assert_eq!(calls(&e, UNCHECK_007E8840), vec![vec![base]]);
        e.call(0x007e_86c0, &args![this, -1i32, 1u32]);
        assert_eq!(e.mem.i32(base + CHECKED), -1);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[1][2], ERASE_SOUND);
        // Done: the sound and the close.
        e.call(0x007e_7e30, &args![this, 0xei32, 0u32]);
        assert_eq!(calls(&e, MANAGER_PLAY_MENU_SOUND), vec![vec![1]]);
        // Navigation: down from 12 to 14, up from 14 to 12, up from 0 nowhere.
        e.mem.set_i32(base + SELECTED, 0xc);
        assert!(e.call(0x007e_81b0, &args![this, 2u32, 0u32]).bool());
        assert_eq!(e.mem.i32(base + SELECTED), 0xe);
        assert!(!e.call(0x007e_81b0, &args![this, 3u32, 0u32]).bool());
        assert!(e.call(0x007e_81b0, &args![this, 1u32, 0u32]).bool());
        assert_eq!(e.mem.i32(base + SELECTED), 0xc);
        e.mem.set_i32(base + SELECTED, 0);
        e.call(0x007e_85c0, &args![this, 1u32]);
        assert_eq!(e.mem.i32(base + SELECTED), 0);
        e.mem.set_i32(base + SELECTED, 0xd);
        e.call(0x007e_85c0, &args![this, 0u32]);
        assert_eq!(e.mem.i32(base + SELECTED), 0);
        // Button 9 on the done button clicks it.
        e.mem.set_i32(base + SELECTED, 0xe);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_81b0, &args![this, 9u32, 0u32]).bool());
        assert_eq!(calls(&e, slot(0xc)), vec![vec![base, 0xe, 0x100e]]);
        assert!(e.call(0x007e_81b0, &args![this, 0xau32, 0u32]).bool());
    }

    #[test]
    fn highlighting_scrolls_with_the_gamepad() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 1);
        answer(&mut e, GET_MENU_BY_CLASS, 0x7000);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfb0 {
                200.0
            } else {
                f64::from(a[0] & 0xff) * 10.0
            },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007e_8470, &args![this, 5i32, 0u32]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x1005, 0x1005, 1]]);
        assert_eq!(e.mem.f32(base + 0x74), 0.25);
        e.call(0x007e_8470, &args![this, 6i32, 0u32]);
        assert_eq!(calls(&e, TILE_SET_INT)[1], vec![0x1005, 0x1005, 0]);
        // Already highlighted: nothing unless forced.
        e.call_log = Some(vec![]);
        e.call(0x007e_8470, &args![this, 6i32, 0u32]);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        e.call(0x007e_8470, &args![this, -1i32, 1u32]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x1006, 0x1005, 0]]);
    }

    #[test]
    fn scrolling_clamps_and_places_the_list() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        answer(&mut e, GET_MENU_BY_CLASS, 0x7000);
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0x1004 { 100.0 } else { 40.0 },
            ..Ret::default()
        });
        e.mem.set_f32(base + 0x70, 0.9);
        e.call_log = Some(vec![]);
        e.call(0x007e_8310, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.mem.f32(base + 0x70), 1.0);
        let sets = calls(&e, TILE_SET_FLOAT);
        assert_eq!(sets[0], vec![0x100f, 0xfa2, 60.0f32.to_bits(), 1]);
        assert_eq!(sets[1], vec![0x100d, 0x02d8, 10.0f32.to_bits(), 0]);
        assert_eq!(
            calls(&e, TILE_POKE),
            vec![vec![0x100d, 0x0250, 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![4]]);
        e.call(0x007e_8310, &args![this, (-3.0f32).to_bits()]);
        assert_eq!(e.mem.f32(base + 0x70), 0.0);
        // The wheel: a negative delta scrolls down the list by 0.05.
        answer(&mut e, WHEEL_DELTA_0070EC70, 0xffff_ffff);
        e.call(0x007e_82b0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(base + 0x70), 0.05);
    }

    #[test]
    fn idle_animates_the_checkmark_and_eases_the_scroll() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        // The scroll bar value matches the scroll (0 and 0).
        e.register(TILE_GET_FLOAT, |_, _| Ret::default());
        e.mem.set_i32(base + CHECKED, 2);
        e.mem.set_u32(base + 0x80, 1000);
        answer(&mut e, CLOCK, 1070);
        e.call_log = Some(vec![]);
        e.call(0x007e_7ea0, &args![this]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![
                vec![0x1011, 0xfc0, 0, 1],
                vec![0x1011, 0xfbf, 48.0f32.to_bits(), 1]
            ]
        );
        // With the gamepad: 100 ms since the last update moves the scroll 0.1 at most.
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 1);
        e.mem.set_i32(base + CHECKED, -1);
        e.mem.set_u32(base + 0x84, 970);
        e.mem.set_f32(base + 0x74, 0.5);
        e.call(0x007e_7ea0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x84), 1070);
        assert_eq!(e.mem.u32(base + 0x70), x87_store32(x87_mul(100.0, 0.001)));
    }
}

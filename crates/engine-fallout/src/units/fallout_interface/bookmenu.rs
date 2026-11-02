//! `fallout/interface/menus/bookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0073ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01070ecc` (RTTI `.?AVBookMenu@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00a1c520`. One word, plain `RET`, returns nothing.
pub fn fn_0073ad30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_0ecc);
    e.call(0x00a1_c520, &args![this]);
}

/// The interface height (`00715da0`, no arguments; the value in `ST0`).
const INTERFACE_HEIGHT: u32 = 0x0071_5da0;
/// The `double` `01011590` (2.0 in the exe) the second height is divided by.
const DIVISOR_01011590: u32 = 0x0101_1590;
/// `_ftol2_sse` (`00ec62c0`): the value in `ST0` (a leading `f64`) to an integer in
/// `EDX:EAX`.
const FTOL: u32 = 0x00ec_62c0;

// Translated from 0073b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bookmenu.cpp`; `thiscall`, no stack words): the
/// interface height minus (the interface height / 2.0 + the `float` at +0x34 of `this`),
/// truncated to an integer (`_ftol2_sse`), whose `EDX:EAX` it leaves. The x87 steps run in the
/// exe's order (`FDIV`, `FADD` of the float, `FSUBR` from the first height kept as a
/// `double`).
pub fn fn_0073b020(e: &mut Engine, this: Ptr) -> u64 {
    let first = e.call(INTERFACE_HEIGHT, &args![]).f64();
    let second = e.call(INTERFACE_HEIGHT, &args![]).f64();
    let divisor: f64 = e.global(DIVISOR_01011590);
    let half = x87_div(second, divisor);
    let offset = x87_m32(e.mem.u32(this.addr() + 0x34));
    let sum = x87_add(half, offset);
    let value = x87_sub(first, sum);
    e.call(FTOL, &args![value]).u64()
}

// ---------------------------------------------------------------------------
// `BookMenu` (batch n051)

layout! {
    /// `BookMenu` (the fields the PC code uses; the Xbox PDB's layout is not
    /// confirmed here).
    pub struct BookMenu: 0x3c {
        /// The text tile (`AttachTileByID` id 0xb).
        0x28 pTextTile: u32,
        /// The scroll bar tile (`AttachTileByID` id 0xc).
        0x2C pScrollTile: u32,
        /// The reference the book was opened from (the second word of `0073b2f0`).
        0x30 pBookRef: u32,
        /// The book (`Create` stores it before checking the menu).
        0x34 pBook: u32,
        /// The grab offset `DoDownClick` stores for `DoStillDown`.
        0x38 fGrabOffset: f32,
    }
}

/// The book menu's class number.
const BOOK_MENU_CLASS: u32 = 0x402;
/// `MemoryManager` free (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;
/// `Tile::GetMenuByClass` (`00a09030`, cdecl), the interface manager singleton
/// (`004b7210`), its tile reader (`00586150`), `Tile::ReadFile` (`00a01b00`, `RET 4`),
/// `Tile::GetMenu` (`00a03c90`), `Tile::GetMaximumDepth` (`00a01530`, `ST0`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const TILE_READER: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type descriptors it is
/// given: the root tile to `TileMenu` (`0119f4cc` to `0119f49c`) and the menu to
/// `BookMenu` (`0119f34c` to `0119f400`), as read from the code.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TILE_TYPE: u32 = 0x0119_f4cc;
const TILE_MENU_TYPE: u32 = 0x0119_f49c;
const MENU_TYPE: u32 = 0x0119_f34c;
const BOOK_MENU_TYPE: u32 = 0x0119_f400;
/// `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), `Menu::PrepForVisibility` (`00a1dc20`,
/// `RET 4`), `Menu::StartFadeOut` (`00a1d910`).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// The print function (`005b5e40`, cdecl) and the console print (`00703c00`, cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The tile's float getter (`00a011b0`, `RET 4`, `ST0`), float setter (`00a012d0`,
/// `RET 0xc`), string setter (`00a01350`, `RET 0xc`), `bool` getter `Tile::IsTrue`
/// (`00a01230`, `RET 4`), the getter `00a01440` (`ST0`, no stack words) and the integer
/// setter `00700320` (`RET 8`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_VALUE_00A01440: u32 = 0x00a0_1440;
const TILE_SET_INT: u32 = 0x0070_0320;
/// The word at +4 of an object (`00726070`: a menu's tile) and its first word
/// (`00559450`).
const WORD_AT_FOUR: u32 = 0x0072_6070;
const FIRST_WORD: u32 = 0x0055_9450;
/// `00409700` (`thiscall`, a `bool`, `enchantmentitem.cpp` in the engine map) on the
/// book: picks the open and close sounds and the menu's mode trait `0x1004`.
const BOOK_TEST_00409700: u32 = 0x0040_9700;
/// `InterfaceManager::PlayMenuSound` (`00717280`, cdecl).
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// The reference's extra data list (`005d43c0`) and `ExtraDataList::GetCount`
/// (`00418770`, 16 bits).
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The wheel direction (`0070ec70`, `thiscall` on the interface manager) and
/// `InterfaceManager::ClearOverTileTarget` (`007157b0`, `RET 4`).
const WHEEL_DIRECTION: u32 = 0x0070_ec70;
const CLEAR_OVER_TILE_TARGET: u32 = 0x0071_57b0;
/// `BSString` construction from a text (`0040c0e0`, `RET 4`), default construction
/// (`004037b0`), destruction (`004037d0`) and its length (`004048e0`).
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_LENGTH_004048E0: u32 = 0x0040_48e0;
/// `BSFile::BSFile` (`00b00260`, `RET 0x10`), `BSFile::Open` (`00aff300`, `RET 8`),
/// `008905f0` (`thiscall`, a `bool`: whether it opened), `BSFile::Read` (`00aff7c0`,
/// `RET 8`: a string and a length) and `BSFile::~BSFile` (`00aff240`).
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
const BS_FILE_OPEN: u32 = 0x00af_f300;
const BS_FILE_IS_OPEN_008905F0: u32 = 0x0089_05f0;
const BS_FILE_READ: u32 = 0x00af_f7c0;
const BS_FILE_DESTRUCT: u32 = 0x00af_f240;
/// `"Data\Menus\book_menu.xml"` (`01070f68`), `"MENUS: Book Menu ..."` (`01070f18`),
/// the workbook file name (`01070fa8`) and `"ERROR: Opening workbook file: %s"`
/// (`01070f84`).
const BOOK_MENU_FILE: u32 = 0x0107_0f68;
const BOOK_MENU_FAILED_TEXT: u32 = 0x0107_0f18;
const WORKBOOK_FILE: u32 = 0x0107_0fa8;
const WORKBOOK_ERROR_FORMAT: u32 = 0x0107_0f84;
/// The doubles 6006.0 (`010704f0`) and 102.0 (`010704e8`) the root's stacking type is
/// compared with.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;

/// The menu's own tile (`00726070(this)`).
fn menu_tile(e: &mut Engine, this: Ptr<BookMenu>) -> u32 {
    e.call(WORD_AT_FOUR, &args![this]).u32()
}

// Translated from 0073acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::GetClass` (Xbox PDB): the class number `0x402`.
pub fn book_menu_get_class(_e: &mut Engine, _this: Ptr<BookMenu>) -> u32 {
    BOOK_MENU_CLASS
}

// Translated from 0073ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::`scalar deleting destructor'` (Xbox PDB): the destructor body
/// (`0073ad30`), then the block freed (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn book_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BookMenu>,
    flags: u32,
) -> Ptr<BookMenu> {
    fn_0073ad30(e, this.cast());
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0073ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::Create` (Xbox PDB, cdecl: the book and a reference). Deletes an open menu
/// of the class; without a book answers 0. Takes the maximum depth, reads
/// `"Data\Menus\book_menu.xml"` and stores the book in the menu read (+0x34) before
/// checking it. For a menu of the class: attaches the root (cast to `TileMenu`), and for
/// the `BookMenu` cast without its two tiles (`0073af60`) prints the creation-failed
/// message and answers 0; sets the depth trait `0xfad` for the stacking types 6006 and
/// 102 (trait `0xfa7`), fills the menu (`0073b2f0` with the book and the reference),
/// plays menu sound 0x18 (0x1a when `00409700` of the book is false) and prepares the
/// menu for showing. Returns the root. A menu of another class is deleted (when it has a
/// tile) and the answer is 0.
pub fn book_menu_create(e: &mut Engine, book: u32, reference: u32) -> u32 {
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![BOOK_MENU_CLASS])
        .u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    if book == 0 {
        return 0;
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
    let depth = x87_store32(depth);
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, BOOK_MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.mem.set_u32(menu.wrapping_add(0x34), book);
    if menu != 0 && e.vcall(menu, 0x34, &args![]).u32() == BOOK_MENU_CLASS {
        let tile_menu = e
            .call(
                RT_DYNAMIC_CAST,
                &args![root, 0u32, TILE_TYPE, TILE_MENU_TYPE, 0u32],
            )
            .u32();
        e.call(MENU_SET_MENU_TILE, &args![menu, tile_menu, 0u32]);
        let this = e
            .call(
                RT_DYNAMIC_CAST,
                &args![menu, 0u32, MENU_TYPE, BOOK_MENU_TYPE, 0u32],
            )
            .ptr::<BookMenu>();
        if !fn_0073af60(e, this) {
            e.call(PRINT_MESSAGE, &args![BOOK_MENU_FAILED_TEXT]);
            return 0;
        }
        let kind = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
        // FCOMP; TEST AH,0x44: the depth for either stacking type (equal, not NaN).
        let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
            let again = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
            again == e.global::<f64>(STACKING_TYPE_TWO)
        };
        if matches {
            e.call(
                TILE_SET_FLOAT,
                &args![root, 0xfadu32, x87_quiet_bits(depth), 1u32],
            );
        }
        fn_0073b2f0(e, this, book, reference);
        let book = e.get(this, BookMenu::pBook);
        let sound = if e.call(BOOK_TEST_00409700, &args![book]).bool() {
            0x18u32
        } else {
            0x1a
        };
        e.call(PLAY_MENU_SOUND, &args![sound]);
        e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
        return root;
    }
    if menu != 0 && e.call(WORD_AT_FOUR, &args![menu]).u32() != 0 {
        // The code tests the pointer again before the deleting call.
        let deleting = menu;
        if deleting != 0 {
            e.vcall(deleting, 0, &args![1u32]);
        }
    }
    0
}

// Translated from 0073af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): whether both tiles (+0x28 and +0x2c) are attached.
pub fn fn_0073af60(e: &mut Engine, this: Ptr<BookMenu>) -> bool {
    e.get(this, BookMenu::pTextTile) != 0 && e.get(this, BookMenu::pScrollTile) != 0
}

// Translated from 0073af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::AttachTileByID` (Xbox PDB, `RET 8`): tile id 0xb is the text tile (+0x28),
/// 0xc the scroll tile (+0x2c); others are ignored.
pub fn book_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<BookMenu>, id: i32, tile: u32) {
    match id {
        0xb => e.set(this, BookMenu::pTextTile, tile),
        0xc => e.set(this, BookMenu::pScrollTile, tile),
        _ => {}
    }
}

/// `0073b020` on the interface manager: the low word of what it leaves (`EAX`).
fn interface_offset(e: &mut Engine, interface: u32) -> i32 {
    fn_0073b020(e, Ptr::new(interface)) as u32 as i32
}

// Translated from 0073afd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::DoDownClick` (Xbox PDB, `RET 8`; the tile word is not read): on the scroll
/// tile (id 0xc), stores the grab offset: `00a01440` of the scroll tile less
/// `0073b020` of the interface manager (`FSUBR`).
pub fn book_menu_do_down_click(e: &mut Engine, this: Ptr<BookMenu>, id: i32, _unused_2: u32) {
    if id != 0xc {
        return;
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let scroll = e.get(this, BookMenu::pScrollTile);
    let position = e.call(TILE_VALUE_00A01440, &args![scroll]).f64();
    let offset = interface_offset(e, interface);
    e.mem.set_u32(
        this.addr() + 0x38,
        x87_store32(x87_sub(position, f64::from(offset))),
    );
}

// Translated from 0073b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::DoStillDown` (Xbox PDB, `RET 8`; the tile word is not read): while the
/// scroll tile (id 0xc) is held, `(0073b020 + grab offset - 00a01440(text tile)) /
/// scroll tile's trait 0x100c` (as a `float`, truncated by `_ftol2`) is set on the text
/// tile's trait `0x1009`, after -999999999 and before 0.
pub fn book_menu_do_still_down(e: &mut Engine, this: Ptr<BookMenu>, id: i32, _unused_2: u32) {
    if id != 0xc {
        return;
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let offset = interface_offset(e, interface);
    let grab = x87_m32(e.mem.u32(this.addr() + 0x38));
    let top = x87_add(f64::from(offset), grab);
    let text = e.get(this, BookMenu::pTextTile);
    let position = e.call(TILE_VALUE_00A01440, &args![text]).f64();
    let moved = x87_sub(top, position);
    let scroll = e.get(this, BookMenu::pScrollTile);
    let step = e.call(TILE_GET_FLOAT, &args![scroll, 0x100cu32]).f64();
    let lines = x87_store32(x87_div(moved, step));
    let text = e.get(this, BookMenu::pTextTile);
    e.call(TILE_SET_INT, &args![text, 0x1009u32, -999_999_999i32]);
    let lines = e.call(FTOL, &args![x87_load32(lines)]).u32();
    let text = e.get(this, BookMenu::pTextTile);
    e.call(TILE_SET_INT, &args![text, 0x1009u32, lines]);
    let text = e.get(this, BookMenu::pTextTile);
    e.call(TILE_SET_INT, &args![text, 0x1009u32, 0u32]);
}

// Translated from 0073b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::DoClick` (Xbox PDB, `RET 8`; the tile word is not read): id 0x1f closes;
/// id 0x20 first has the player take the reference (the player's slot +0x3d0 with the
/// reference, its count `ExtraDataList::GetCount` sign-extended, and 0), then closes.
pub fn book_menu_do_click(e: &mut Engine, this: Ptr<BookMenu>, id: i32, _unused_2: u32) {
    if id == 0x1f {
        book_menu_close(e, this);
    } else if id == 0x20 {
        let reference = e.get(this, BookMenu::pBookRef);
        let extras = e.call(EXTRA_LIST_005D43C0, &args![reference]).u32();
        let count = i32::from(e.call(EXTRA_GET_COUNT, &args![extras]).u16() as i16);
        let reference = e.get(this, BookMenu::pBookRef);
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3d0, &args![reference, count, 0u32]);
        book_menu_close(e, this);
    }
}

// Translated from 0073b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::DoWheelMove` (Xbox PDB, `RET 8`). In scroll mode (the menu tile's trait
/// `0x1004` false): sets the scroll tile's trait `0x100d` to minus the wheel direction /
/// 120 (`IDIV`) and back to 0, and when the text tile's trait `0x100b` (truncated) then
/// differs, leaves the tile (slot +0x14 with both words) and clears the interface
/// manager's over-tile target. In page mode: sets the menu tile's trait `0x100a` to -2
/// (wheel up) or 2 and back to 0, and when its trait `0x100d` differs from what it was,
/// plays the menu sound its trait `0x100c` names (when not 0).
pub fn book_menu_do_wheel_move(e: &mut Engine, this: Ptr<BookMenu>, first: u32, second: u32) {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let tile = menu_tile(e, this);
    if !e.call(TILE_IS_TRUE, &args![tile, 0x1004u32]).bool() {
        let text = e.get(this, BookMenu::pTextTile);
        let value = e.call(TILE_GET_FLOAT, &args![text, 0x100bu32]).f64();
        let before = e.call(FTOL, &args![value]).i32();
        let wheel = e.call(WHEEL_DIRECTION, &args![interface]).i32();
        let notches = wheel.wrapping_div(0x78);
        let scroll = e.get(this, BookMenu::pScrollTile);
        e.call(
            TILE_SET_INT,
            &args![scroll, 0x100du32, notches.wrapping_neg()],
        );
        let scroll = e.get(this, BookMenu::pScrollTile);
        e.call(TILE_SET_INT, &args![scroll, 0x100du32, 0u32]);
        let text = e.get(this, BookMenu::pTextTile);
        let after = e.call(TILE_GET_FLOAT, &args![text, 0x100bu32]).f64();
        // FUCOMPP; TEST AH,0x44; JNP: nothing more when equal.
        if f64::from(before) != after {
            e.vcall(this.addr(), 0x14, &args![first, second]);
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            e.call(CLEAR_OVER_TILE_TARGET, &args![manager, 0u32]);
        }
        return;
    }
    let tile = menu_tile(e, this);
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0x100du32]).f64();
    let before = e.call(FTOL, &args![value]).i32();
    let step = if e.call(WHEEL_DIRECTION, &args![interface]).i32() > 0 {
        -2i32
    } else {
        2
    };
    let tile = menu_tile(e, this);
    e.call(TILE_SET_INT, &args![tile, 0x100au32, step]);
    let tile = menu_tile(e, this);
    e.call(TILE_SET_INT, &args![tile, 0x100au32, 0u32]);
    let tile = menu_tile(e, this);
    let after = e.call(TILE_GET_FLOAT, &args![tile, 0x100du32]).f64();
    if f64::from(before) != after {
        let tile = menu_tile(e, this);
        let value = e.call(TILE_GET_FLOAT, &args![tile, 0x100cu32]).f64();
        let sound = e.call(FTOL, &args![value]).u32();
        if sound != 0 {
            e.call(INTERFACE_MANAGER, &args![]);
            e.call(PLAY_MENU_SOUND, &args![sound]);
        }
    }
}

// Translated from 0073b2f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0073b2f0 - the C++ exception-unwinding frame (FS:[0] around the local string) is not translated.
/// Unnamed (`thiscall`, `RET 8`: the book and the reference; the book menu's fill):
/// the menu tile's trait `0x1004` is 0 when `00409700` of the book holds, else 1; its
/// string trait `0x1006` gets the book's text (`0073b430`); the reference is stored
/// (+0x30); the trait `0x1007` is 0 without a reference or for a book with flag 2
/// (`0073b460`), else 1. A local `BSString` of the text is built (`0040c0e0`) around
/// setting the trait `0x100a` to -99999 and back to 0, and destroyed.
pub fn fn_0073b2f0(e: &mut Engine, this: Ptr<BookMenu>, book: u32, reference: u32) {
    let mode = if e.call(BOOK_TEST_00409700, &args![book]).bool() {
        0u32
    } else {
        1
    };
    let tile = menu_tile(e, this);
    e.call(TILE_SET_INT, &args![tile, 0x1004u32, mode]);
    let text = fn_0073b430(e, book);
    let tile = menu_tile(e, this);
    e.call(TILE_SET_STRING, &args![tile, 0x1006u32, text, 1u32]);
    e.set(this, BookMenu::pBookRef, reference);
    let takeable = if reference == 0 || fn_0073b460(e, book) {
        0u32
    } else {
        1
    };
    let tile = menu_tile(e, this);
    e.call(TILE_SET_INT, &args![tile, 0x1007u32, takeable]);
    let text = fn_0073b430(e, book);
    // The local string at `EBP - 0x14`.
    e.with_stack(8, |e, string| {
        e.call(STRING_FROM_TEXT, &args![string, text]);
        let tile = menu_tile(e, this);
        e.call(TILE_SET_INT, &args![tile, 0x100au32, -99_999i32]);
        let tile = menu_tile(e, this);
        e.call(TILE_SET_INT, &args![tile, 0x100au32, 0u32]);
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

// Translated from 0073b430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a book): the description of its component at +0x94 (that
/// component's slot +0x10 with 0 and `'DESC'`, `0x43534544`).
pub fn fn_0073b430(e: &mut Engine, book: u32) -> u32 {
    e.vcall(book.wrapping_add(0x94), 0x10, &args![0u32, 0x4353_4544u32])
        .u32()
}

// Translated from 0073b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a book): whether bit 1 of its byte at +0xc0 is set.
pub fn fn_0073b460(e: &mut Engine, book: u32) -> bool {
    e.mem.u8(book.wrapping_add(0xc0)) & 2 != 0
}

// Translated from 0073b480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::Close` (Xbox PDB): with the class's menu tile and its menu, sets the tile's
/// trait `0x1772` to 1, starts the menu's fade out and plays menu sound 0x19 (0x1b when
/// `00409700` of the book is false).
pub fn book_menu_close(e: &mut Engine, this: Ptr<BookMenu>) {
    let tile = e
        .call(TILE_GET_MENU_BY_CLASS, &args![BOOK_MENU_CLASS])
        .u32();
    if tile == 0 || e.call(TILE_GET_MENU, &args![tile]).u32() == 0 {
        return;
    }
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(MENU_START_FADE_OUT, &args![menu]);
    let book = e.get(this, BookMenu::pBook);
    let sound = if e.call(BOOK_TEST_00409700, &args![book]).bool() {
        0x19u32
    } else {
        0x1b
    };
    e.call(PLAY_MENU_SOUND, &args![sound]);
}

// Translated from 0073b500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0073b500 - the C++ exception-unwinding frame (FS:[0]: the strings and the file) and the /GS cookie check (00ec408c) are not translated.
/// `BookMenu::UseWorkbook` (Xbox PDB): opens the workbook file (a local `BSString` of
/// `01070fa8`, a local `BSFile` with mode 0x4000, `BSFile::Open(0, 0)`); when it did not
/// open (`008905f0`) prints `"ERROR: Opening workbook file: %s"` to the console;
/// otherwise reads it into a local string (`BSFile::Read(string, -1)`, its length taken
/// by `004048e0`), sets that text as the menu tile's string trait `0x1006` and the trait
/// `0x100a` to -99999 and back to 0. The locals are destroyed.
pub fn book_menu_use_workbook(e: &mut Engine, this: Ptr<BookMenu>) {
    // The frame below `EBP`: the name at `EBP - 0x14`, the file at `EBP - 0x174`, the
    // text at `EBP - 0x17c`.
    e.with_stack(0x170, |e, frame| {
        let text = frame.addr();
        let file = frame.addr() + 8;
        let name = frame.addr() + 0x168;
        e.call(STRING_FROM_TEXT, &args![name, WORKBOOK_FILE]);
        let path = e.call(FIRST_WORD, &args![name]).u32();
        e.call(BS_FILE_CONSTRUCT, &args![file, path, 0u32, 0x4000u32, 0u32]);
        e.call(STRING_CONSTRUCT, &args![text]);
        e.call(BS_FILE_OPEN, &args![file, 0u32, 0u32]);
        if !e.call(BS_FILE_IS_OPEN_008905F0, &args![file]).bool() {
            let path = e.call(FIRST_WORD, &args![name]).u32();
            e.call(CONSOLE_PRINT, &args![WORKBOOK_ERROR_FORMAT, path]);
        } else {
            e.call(BS_FILE_READ, &args![file, text, 0xffff_ffffu32]);
            let _length = e.call(STRING_LENGTH_004048E0, &args![text]).u32();
            let contents = e.call(FIRST_WORD, &args![text]).u32();
            let tile = menu_tile(e, this);
            e.call(TILE_SET_STRING, &args![tile, 0x1006u32, contents, 1u32]);
            let tile = menu_tile(e, this);
            e.call(TILE_SET_INT, &args![tile, 0x100au32, -99_999i32]);
            let tile = menu_tile(e, this);
            e.call(TILE_SET_INT, &args![tile, 0x100au32, 0u32]);
        }
        e.call(STRING_DESTRUCT, &args![text]);
        e.call(BS_FILE_DESTRUCT, &args![file]);
        e.call(STRING_DESTRUCT, &args![name]);
    });
}

// Translated from 0073b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BookMenu::DoKey` (Xbox PDB, `RET 4`): key 0x20 closes the menu and answers true;
/// other keys false.
pub fn book_menu_do_key(e: &mut Engine, this: Ptr<BookMenu>, key: i32) -> bool {
    if key == 0x20 {
        book_menu_close(e, this);
        true
    } else {
        false
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_ad30, fn_0073ad30(Ptr)),
        entry!(0x0073_b020, fn_0073b020(Ptr) -> u64),
        entry!(0x0073_acf0, book_menu_get_class(Ptr<BookMenu>) -> u32),
        entry!(
            0x0073_ad00,
            book_menu_scalar_deleting_destructor(Ptr<BookMenu>, u32) -> Ptr<BookMenu>
        ),
        entry!(0x0073_ad50, book_menu_create(u32, u32) -> u32),
        entry!(0x0073_af60, fn_0073af60(Ptr<BookMenu>) -> bool),
        entry!(
            0x0073_af90,
            book_menu_attach_tile_by_id(Ptr<BookMenu>, i32, u32)
        ),
        entry!(
            0x0073_afd0,
            book_menu_do_down_click(Ptr<BookMenu>, i32, u32)
        ),
        entry!(
            0x0073_b050,
            book_menu_do_still_down(Ptr<BookMenu>, i32, u32)
        ),
        entry!(0x0073_b0f0, book_menu_do_click(Ptr<BookMenu>, i32, u32)),
        entry!(
            0x0073_b150,
            book_menu_do_wheel_move(Ptr<BookMenu>, u32, u32)
        ),
        entry!(0x0073_b2f0, fn_0073b2f0(Ptr<BookMenu>, u32, u32)),
        entry!(0x0073_b430, fn_0073b430(u32) -> u32),
        entry!(0x0073_b460, fn_0073b460(u32) -> bool),
        entry!(0x0073_b480, book_menu_close(Ptr<BookMenu>)),
        entry!(0x0073_b500, book_menu_use_workbook(Ptr<BookMenu>)),
        entry!(0x0073_b6b0, book_menu_do_key(Ptr<BookMenu>, i32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0073ad30_stores_the_vtable_then_forwards_this() {
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
        e.call(0x0073_ad30, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00a1_c520)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_0ecc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_0ecc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod height_tests {
    use super::*;

    #[test]
    fn fn_0073b020_takes_half_the_height_and_the_offset_from_the_height() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(DIVISOR_01011590, 2.0f64);
        e.register(INTERFACE_HEIGHT, |_, _| 960.0f32.into_ret());
        // The truncation double sees the value and answers it plus a marker in EDX.
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                edx: 0x77,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x34, 100.5);
        e.call_log = Some(vec![]);
        let result = e.call(0x0073_b020, &args![this]).u64();
        assert_eq!(result, (0x77 << 32) | 379);
        let heights = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == INTERFACE_HEIGHT)
            .count();
        assert_eq!(heights, 2);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        FREE,
        TILE_GET_MENU_BY_CLASS,
        INTERFACE_MANAGER,
        TILE_READER,
        TILE_READ_FILE,
        TILE_GET_MENU,
        TILE_GET_MAXIMUM_DEPTH,
        RT_DYNAMIC_CAST,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        PRINT_MESSAGE,
        CONSOLE_PRINT,
        TILE_GET_FLOAT,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        TILE_IS_TRUE,
        TILE_VALUE_00A01440,
        TILE_SET_INT,
        WORD_AT_FOUR,
        FIRST_WORD,
        BOOK_TEST_00409700,
        PLAY_MENU_SOUND,
        EXTRA_LIST_005D43C0,
        EXTRA_GET_COUNT,
        WHEEL_DIRECTION,
        CLEAR_OVER_TILE_TARGET,
        STRING_FROM_TEXT,
        STRING_CONSTRUCT,
        STRING_DESTRUCT,
        STRING_LENGTH_004048E0,
        BS_FILE_CONSTRUCT,
        BS_FILE_OPEN,
        BS_FILE_IS_OPEN_008905F0,
        BS_FILE_READ,
        BS_FILE_DESTRUCT,
        INTERFACE_HEIGHT,
        FTOL,
        0x00a1_c520,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// An engine with the callees doubled (`00726070` gives the word at +4, `_ftol2`
    /// truncates), the test vtable and the stacking doubles mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0107_0000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(STACKING_TYPE_ONE, 6006.0f64);
        e.set_global(STACKING_TYPE_TWO, 102.0f64);
        e.map(0x0101_1000, 0x1000);
        e.set_global(DIVISOR_01011590, 2.0f64);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        let slots: Vec<u32> = (0..0x100).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    /// A book menu with the test vtable and its menu tile 0x5000.
    fn menu(e: &mut Engine) -> Ptr<BookMenu> {
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + 4, 0x5000);
        this
    }

    #[test]
    fn the_class_and_the_deleting_destructor() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x0073_acf0, &args![this]).u32(), 0x402);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_ad00, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, 0x00a1_c520), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_ad00, &args![this, 0u32]);
        assert!(calls_to(&e, FREE).is_empty());
    }

    #[test]
    fn create_opens_the_book() {
        let mut e = engine();
        assert_eq!(e.call(0x0073_ad50, &args![0u32, 0u32]).u32(), 0);
        let this = menu(&mut e);
        answer(&mut e, TILE_READ_FILE, 0x5800);
        answer(&mut e, TILE_GET_MENU, this.addr());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Another class: deleted.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_ad50, &args![0x9000u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, slot(0)), vec![vec![this.addr(), 1]]);
        assert_eq!(e.get(this, BookMenu::pBook), 0x9000);
        // The class without its tiles.
        e.register(slot(0x34), |_, _| 0x402u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_ad50, &args![0x9000u32, 0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![BOOK_MENU_FAILED_TEXT]]
        );
        e.set(this, BookMenu::pTextTile, 0x5100);
        e.set(this, BookMenu::pScrollTile, 0x5200);
        e.register(TILE_GET_FLOAT, |_, _| st0(6006.0));
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| st0(4.0));
        answer(&mut e, BOOK_TEST_00409700, 1);
        let book = e.mem.alloc(0x100);
        e.mem.set_u32(book + 0x94, VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_ad50, &args![book, 0x9100u32]).u32(), 0x5800);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this.addr(), 0x5800, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5800, 0xfad, 4.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(this, BookMenu::pBookRef), 0x9100);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![0x18]]);
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
    }

    #[test]
    fn tiles_attach_by_id_and_both_are_needed() {
        let mut e = engine();
        let this = menu(&mut e);
        assert!(!e.call(0x0073_af60, &args![this]).bool());
        e.call(0x0073_af90, &args![this, 0xbi32, 0x5100u32]);
        assert!(!e.call(0x0073_af60, &args![this]).bool());
        e.call(0x0073_af90, &args![this, 0xci32, 0x5200u32]);
        e.call(0x0073_af90, &args![this, 0xdi32, 0x5300u32]);
        assert!(e.call(0x0073_af60, &args![this]).bool());
        assert_eq!(e.get(this, BookMenu::pTextTile), 0x5100);
        assert_eq!(e.get(this, BookMenu::pScrollTile), 0x5200);
    }

    #[test]
    fn dragging_the_scroll_tile_moves_the_text() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BookMenu::pTextTile, 0x5100);
        e.set(this, BookMenu::pScrollTile, 0x5200);
        // `0073b020` answers 960 - (480 + 0.0) = 480 on a zeroed interface object.
        let interface = e.mem.alloc(0x40);
        answer(&mut e, INTERFACE_MANAGER, interface);
        e.register(INTERFACE_HEIGHT, |_, _| st0(960.0));
        e.register(TILE_VALUE_00A01440, |_, a| {
            st0(if a[0] == 0x5200 { 500.0 } else { 100.0 })
        });
        e.call(0x0073_afd0, &args![this, 0xci32, 0u32]);
        assert_eq!(e.get(this, BookMenu::fGrabOffset), 20.0);
        e.register(TILE_GET_FLOAT, |_, _| st0(4.0));
        e.call_log = Some(vec![]);
        e.call(0x0073_b050, &args![this, 0xci32, 0u32]);
        // (480 + 20 - 100) / 4 = 100.
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x5100, 0x1009, (-999_999_999i32) as u32],
                vec![0x5100, 0x1009, 100],
                vec![0x5100, 0x1009, 0]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0073_b050, &args![this, 0xbi32, 0u32]);
        e.call(0x0073_afd0, &args![this, 0xbi32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn clicks_close_or_take_the_book() {
        let mut e = engine();
        let this = menu(&mut e);
        let player = e.mem.alloc(4);
        e.mem.set_u32(player, VTABLE);
        e.set_global(PLAYER, player);
        e.set(this, BookMenu::pBookRef, 0x9100);
        answer(&mut e, EXTRA_GET_COUNT, 0xffff);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0x5400);
        answer(&mut e, TILE_GET_MENU, 0x5500);
        e.call_log = Some(vec![]);
        e.call(0x0073_b0f0, &args![this, 0x20i32, 0u32]);
        assert_eq!(
            calls_to(&e, slot(0x3d0)),
            vec![vec![player, 0x9100, 0xffff_ffff, 0]]
        );
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![0x5500]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![0x1b]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_b0f0, &args![this, 0x1fi32, 0u32]);
        assert!(calls_to(&e, slot(0x3d0)).is_empty());
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT).len(), 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_b6b0, &args![this, 0x20i32]).bool());
        assert!(!e.call(0x0073_b6b0, &args![this, 0x21i32]).bool());
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT).len(), 1);
    }

    #[test]
    fn close_needs_the_menu() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_b480, &args![this]);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0x5400);
        answer(&mut e, TILE_GET_MENU, 0x5500);
        answer(&mut e, BOOK_TEST_00409700, 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_b480, &args![this]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x5400, 0x1772, 1]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![0x19]]);
    }

    #[test]
    fn the_wheel_scrolls_or_turns_pages() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, BookMenu::pTextTile, 0x5100);
        e.set(this, BookMenu::pScrollTile, 0x5200);
        answer(&mut e, INTERFACE_MANAGER, 0x6000);
        answer(&mut e, WHEEL_DIRECTION, 240);
        // Scroll mode: the text's position moves from 3 to 4.
        e.register_double(TILE_GET_FLOAT, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                st0(if calls == 1 { 3.0 } else { 4.0 })
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_b150, &args![this, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x5200, 0x100d, (-2i32) as u32],
                vec![0x5200, 0x100d, 0]
            ]
        );
        assert_eq!(calls_to(&e, slot(0x14)), vec![vec![this.addr(), 7, 8]]);
        assert_eq!(calls_to(&e, CLEAR_OVER_TILE_TARGET), vec![vec![0x6000, 0]]);
        // Page mode: the page changes, its sound plays.
        answer(&mut e, TILE_IS_TRUE, 1);
        e.register_double(TILE_GET_FLOAT, {
            let mut calls = 0;
            move |_, a| {
                calls += 1;
                st0(match (calls, a[1]) {
                    (_, 0x100c) => 9.0,
                    (1, _) => 1.0,
                    _ => 2.0,
                })
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_b150, &args![this, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x5000, 0x100a, (-2i32) as u32],
                vec![0x5000, 0x100a, 0]
            ]
        );
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![9]]);
    }

    #[test]
    fn the_fill_sets_the_modes_and_the_text() {
        let mut e = engine();
        let this = menu(&mut e);
        let book = e.mem.alloc(0x100);
        e.mem.set_u32(book + 0x94, VTABLE);
        e.register(slot(0x10), |_, a| (a[2] & 0xff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_b2f0, &args![this, book, 0x9100u32]);
        assert_eq!(
            calls_to(&e, slot(0x10))[0],
            vec![book + 0x94, 0, 0x4353_4544]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x5000, 0x1004, 1],
                vec![0x5000, 0x1007, 1],
                vec![0x5000, 0x100a, (-99_999i32) as u32],
                vec![0x5000, 0x100a, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5000, 0x1006, 0x44, 1]]
        );
        assert_eq!(e.get(this, BookMenu::pBookRef), 0x9100);
        assert_eq!(calls_to(&e, STRING_FROM_TEXT)[0][1], 0x44);
        // A book with flag 2 cannot be taken.
        e.mem.set_u8(book + 0xc0, 2);
        assert!(e.call(0x0073_b460, &args![book]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0073_b2f0, &args![this, book, 0x9100u32]);
        assert_eq!(calls_to(&e, TILE_SET_INT)[1], vec![0x5000, 0x1007, 0]);
        assert_eq!(e.call(0x0073_b430, &args![book]).u32(), 0x44);
    }

    #[test]
    fn the_workbook_is_read_into_the_text() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0073_b500, &args![this]);
        assert_eq!(calls_to(&e, CONSOLE_PRINT)[0][0], WORKBOOK_ERROR_FORMAT);
        assert!(calls_to(&e, BS_FILE_READ).is_empty());
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&e, BS_FILE_DESTRUCT).len(), 1);
        answer(&mut e, BS_FILE_IS_OPEN_008905F0, 1);
        answer(&mut e, FIRST_WORD, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0073_b500, &args![this]);
        let constructed = calls_to(&e, BS_FILE_CONSTRUCT);
        assert_eq!(constructed[0][1..], [0x7700, 0, 0x4000, 0]);
        assert_eq!(calls_to(&e, BS_FILE_READ)[0][2], 0xffff_ffff);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5000, 0x1006, 0x7700, 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT).len(), 2);
    }
}

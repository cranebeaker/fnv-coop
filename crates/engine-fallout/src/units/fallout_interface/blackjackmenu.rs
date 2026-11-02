//! `fallout/interface/menus/blackjackmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00733fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, one `float`): stores the float at +0xf8 (`FLD`/`FSTP`, so a
/// signalling NaN is stored quieted).
pub fn fn_00733fd0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0xf8),
        x87_quiet_bits(value.to_bits()),
    );
}

// ---------------------------------------------------------------------------
// `BlackJackMenu` (batch n051). The PC layout differs from the Xbox PDB's (the flags
// word the PDB has at +0xdb4 is at +0xdb0 here); offsets below are the PC code's.

/// `BlackJackMenu::pMe` (Xbox PDB static, by its use: the menu whose state +0x28 is set).
const BLACKJACK_MENU: u32 = 0x011d_9050;
/// Two statics the bet label reads: an object whose word at +4 is the bet shown
/// (`011d9054`) and the casino whose name component (+0x18) is shown (`011d9058`);
/// their names are not confirmed.
const BET_DATA_011D9054: u32 = 0x011d_9054;
const CASINO_011D9058: u32 = 0x011d_9058;
/// The trait id static the bet label is set with (`011d9044`).
const LABEL_TRAIT_011D9044: u32 = 0x011d_9044;
/// The table model holder (`011d9098`, an `NiPointer`).
const TABLE_MODEL_011D9098: u32 = 0x011d_9098;
/// `pTiles` (Xbox PDB name) at +0x34 on PC, ten tiles.
const BLACKJACK_TILES: u32 = 0x34;
/// The flags word at +0xdb0 (`iFlags` in the Xbox PDB at +0xdb4).
const BLACKJACK_FLAGS: u32 = 0xdb0;
/// The class number.
const BLACKJACK_MENU_CLASS: u32 = 0x439;

/// `MemoryManager` free (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;
/// The `NiPointer` constructor (`00633c90`, `RET 4`) and assignment from another
/// `NiPointer` (`006e5cc0`, `RET 4`; the map's folded name).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_COPY_006E5CC0: u32 = 0x006e_5cc0;
/// The interface manager (`004b7210`), its menu tile (`0045cd60`), the tile float setter
/// (`00a012d0`, `RET 0xc`) and the string trait setter (`00a01350`, `RET 0xc`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const WORD_AT_0X28: u32 = 0x0045_cd60;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `255.0` (`01023cd8`) and `-5.0` (`0102caf8`), `float`s.
const FULL_ALPHA: u32 = 0x0102_3cd8;
const MINUS_FIVE_0102CAF8: u32 = 0x0102_caf8;
/// The first word of an object (`00559450`), a list node's item slot (`006815c0`) and
/// next node (`00726070`).
const FIRST_WORD: u32 = 0x0055_9450;
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// A tile's child list walk: `0073ac90` (the current item's slot), `006317a0` (the child
/// at a node, `RET 4`) and `007b52d0` (the next node, `RET 4`).
const CHILD_LIST_ITEM_0073AC90: u32 = 0x0073_ac90;
const CHILD_LIST_AT_006317A0: u32 = 0x0063_17a0;
const CHILD_LIST_FOLLOWING_007B52D0: u32 = 0x007b_52d0;
/// The light's three-float set (`004bc1f0`, `RET 0xc`).
const SET_FLOATS_004BC1F0: u32 = 0x004b_c1f0;
/// A named child object (`004aae30`, cdecl: the root and the name) and the app-culled
/// setter (`00450f90`, `RET 4`).
const FIND_OBJECT_004AAE30: u32 = 0x004a_ae30;
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
/// The update data's initialisation (`0043d410`, `RET 0xc`), the node getter
/// `0043b4a0` (`RET 4`) and the object update (`00a59c60`, `RET 4`).
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
const NODE_0043B4A0: u32 = 0x0043_b4a0;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// A setting's string (`00403df0`), the name of a name component (`00408da0`), the
/// formatter (`00406d00`, cdecl, variadic) and `strlen` (`0044a670`, cdecl).
const SETTING_STRING: u32 = 0x0040_3df0;
const NAME_OF_00408DA0: u32 = 0x0040_8da0;
const FORMAT_TEXT: u32 = 0x0040_6d00;
const STRING_LENGTH_0044A670: u32 = 0x0044_a670;
/// `"Hand1_Arrow:0"`, `"Hand1_Arrow:0@#2..."` (`01070e58`, `01070e44`), `"%s %s%i"`
/// (`01070e80`), `"%s %s"` (`01012058`) and `"%s%i"` (`01029ca8`).
const HAND_ARROW_NAME: u32 = 0x0107_0e58;
const SPLIT_HAND_ARROW_NAME: u32 = 0x0107_0e44;
const LABEL_VALUE_FORMAT: u32 = 0x0107_0e80;
const LABEL_FORMAT: u32 = 0x0101_2058;
const PADDED_VALUE_FORMAT: u32 = 0x0102_9ca8;

/// The menu singleton's address.
fn blackjack_menu(e: &Engine) -> u32 {
    e.global::<u32>(BLACKJACK_MENU)
}

// Translated from 00731710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731710 - the C++ exception-unwinding frame (FS:[0] around the first pointer) is not translated.
/// Unnamed (`thiscall`; a card's constructor): the two `NiPointer`s at +8 and +0xc
/// empty, the words at +0 and +4 set to 0xe and 5. Returns `this`.
pub fn fn_00731710(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 8, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0xc, 0u32]);
    e.mem.set_u32(this.addr(), 0xe);
    e.mem.set_u32(this.addr() + 4, 5);
    this
}

// Translated from 00731790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::GetClass` (Xbox PDB): the class number `0x439`.
pub fn black_jack_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    BLACKJACK_MENU_CLASS
}

// Translated from 007317a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::AttachTileByID` (Xbox PDB, `RET 8`): ids 0 to 9 store the tile in
/// `pTiles` (+0x34); others are ignored.
pub fn black_jack_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0xa).contains(&id) {
        e.mem
            .set_u32(this.addr() + BLACKJACK_TILES + 4 * id as u32, tile);
    }
}

// Translated from 007317d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::`scalar deleting destructor'` (Xbox PDB): the destructor body
/// (`00731800`), then the block freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn black_jack_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00731800(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00731d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): with the menu open, sets its state (+0x28) to 0xd and the
/// interface manager's menu tile's alpha (trait `0xfa9`) to 255.
pub fn fn_00731d30(e: &mut Engine) {
    if blackjack_menu(e) == 0 {
        return;
    }
    let me = blackjack_menu(e);
    e.mem.set_u32(me + 0x28, 0xd);
    let alpha = x87_quiet_bits(e.global::<u32>(FULL_ALPHA));
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let tile = e.call(WORD_AT_0X28, &args![manager]).u32();
    e.call(TILE_SET_FLOAT, &args![tile, 0xfa9u32, alpha, 1u32]);
}

// Translated from 007324f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; a card's assignment): copies the two words and assigns
/// the two `NiPointer`s (+8, +0xc, `006e5cc0`). Returns `this`.
pub fn fn_007324f0(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    let word = e.mem.u32(other);
    e.mem.set_u32(this.addr(), word);
    let word = e.mem.u32(other + 4);
    e.mem.set_u32(this.addr() + 4, word);
    e.call(NI_POINTER_COPY_006E5CC0, &args![this.addr() + 8, other + 8]);
    e.call(
        NI_POINTER_COPY_006E5CC0,
        &args![this.addr() + 0xc, other + 0xc],
    );
    this
}

// Translated from 00733f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): clears the words at +0 and +4 and the 16-bit word at +8.
/// Returns `this`.
pub fn fn_00733f50(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 8, 0);
    this
}

// Translated from 00733f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the blackjack menu's class number `0x439`.
pub fn fn_00733f80(_e: &mut Engine) -> u32 {
    BLACKJACK_MENU_CLASS
}

// Translated from 00733f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): whether all ten tiles are attached.
pub fn fn_00733f90(e: &mut Engine, this: Ptr) -> bool {
    (0..0xa).all(|i| e.mem.u32(this.addr() + BLACKJACK_TILES + 4 * i) != 0)
}

// Translated from 00734d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): whether the flags word (+0xdb0) has a bit of the
/// mask (0 or 1).
pub fn fn_00734d30(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + BLACKJACK_FLAGS) & mask != 0
}

// Translated from 00734d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a mask and a `bool`): sets the mask's bits in the flags
/// word (+0xdb0), or clears them.
pub fn fn_00734d50(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = e.mem.u32(this.addr() + BLACKJACK_FLAGS);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.mem.set_u32(this.addr() + BLACKJACK_FLAGS, flags);
}

// Translated from 00737e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the word at +0x210.
pub fn fn_00737e30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x210)
}

// Translated from 00738c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::SetTileTextAlpha` (Xbox PDB, `thiscall`, `RET 8`: a tile index and an
/// alpha; `this` is not read): sets the alpha (trait `0xfa9`) of every child of the open
/// menu's tile at that index (its child list at +4, walked with `0073ac90`, `006317a0`
/// and `007b52d0`).
pub fn black_jack_menu_set_tile_text_alpha(e: &mut Engine, _this: Ptr, index: i32, alpha: f32) {
    let tile_of = |e: &Engine| {
        let me = blackjack_menu(e);
        e.mem.u32(
            me.wrapping_add(BLACKJACK_TILES)
                .wrapping_add((index as u32).wrapping_mul(4)),
        )
    };
    let list = tile_of(e).wrapping_add(4);
    let mut node = e.call(FIRST_WORD, &args![list]).u32();
    let list = tile_of(e).wrapping_add(4);
    while node != 0 && list != 0 {
        let item = e.call(CHILD_LIST_ITEM_0073AC90, &args![list]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let slot = e.call(CHILD_LIST_AT_006317A0, &args![list, node]).u32();
        let child = e.mem.u32(slot);
        e.call(
            TILE_SET_FLOAT,
            &args![child, 0xfa9u32, x87_quiet_bits(alpha.to_bits()), 1u32],
        );
        node = e
            .call(CHILD_LIST_FOLLOWING_007B52D0, &args![list, node])
            .u32();
    }
}

// Translated from 00738fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; `this` is not read): a card's points: 10 for ranks 9 to
/// 11, 11 for rank 12 (the ace), else the rank plus 2.
pub fn fn_00738fb0(_e: &mut Engine, _this: Ptr, rank: i32) -> i32 {
    if rank >= 9 {
        if rank <= 0xb {
            return 0xa;
        }
        if rank == 0xc {
            return 0xb;
        }
    }
    rank.wrapping_add(2)
}

// Translated from 00738ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a hand's first node): the hand's value: the points of
/// every card (`00738fb0` of its rank, the card's first word), each ace (rank 12) counted
/// as 1 instead of 11 while the total is above 21.
pub fn fn_00738ff0(e: &mut Engine, this: Ptr, hand: u32) -> i32 {
    let mut total = 0i32;
    let mut aces = 0i32;
    let mut node = hand;
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_SELF, &args![node]).u32();
        let card = e.mem.u32(slot);
        let rank = e.mem.i32(card);
        total = fn_00738fb0(e, this, rank).wrapping_add(total);
        if e.mem.u32(card) == 0xc {
            aces += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    while total > 0x15 && aces > 0 {
        total -= 0xa;
        aces -= 1;
    }
    total
}

// Translated from 007395a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; whether the player may split): true when the player's hand
/// (+0xd94) holds exactly two cards of the same rank and the chips (+0x74) are at least
/// twice the bet (+0x6c); stored in flag 4 (`00734d50`).
pub fn fn_007395a0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let mut node = e.mem.u32(base + 0xd94);
    let mut first = 0;
    let mut index = 0;
    let mut split = false;
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        if index == 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            first = e.mem.u32(slot);
        } else if index == 1 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            let card = e.mem.u32(slot);
            if e.mem.u32(first) == e.mem.u32(card)
                && e.mem.i32(base + 0x74) >= e.mem.i32(base + 0x6c).wrapping_shl(1)
            {
                split = true;
            }
        } else {
            split = false;
            break;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
        index += 1;
    }
    fn_00734d50(e, this, 4, u8::from(split));
    split
}

// Translated from 00739650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: whether for the split hand; whether the player may double
/// down): false when the chips (+0x74) are below the bet (+0x6c) plus twice the split
/// bet (+0x70) for the split hand, or the split bet plus twice the bet otherwise; stored
/// in flag 0x200 (split) or 0x40.
pub fn fn_00739650(e: &mut Engine, this: Ptr, split: u8) -> bool {
    let base = this.addr();
    let mut allowed = true;
    let bet = |e: &Engine| e.mem.i32(base + 0x6c);
    let split_bet = |e: &Engine| e.mem.i32(base + 0x70);
    let chips = |e: &Engine| e.mem.i32(base + 0x74);
    if split != 0 && chips(e) < bet(e).wrapping_add(split_bet(e).wrapping_mul(2)) {
        allowed = false;
    }
    if split == 0 && chips(e) < split_bet(e).wrapping_add(bet(e).wrapping_mul(2)) {
        allowed = false;
    }
    let mask = if split != 0 { 0x200 } else { 0x40 };
    fn_00734d50(e, this, mask, u8::from(allowed));
    allowed
}

// Translated from 00739970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `bool`): sets the three floats of the hand geometry
/// (+0xdbc, `004bc1f0`) to 0, 0 and -5.0 without the flag, else to zero.
pub fn fn_00739970(e: &mut Engine, this: Ptr, flag: u8) {
    let zero = x87_store32(0.0);
    let last = if flag == 0 {
        x87_quiet_bits(e.global::<u32>(MINUS_FIVE_0102CAF8))
    } else {
        zero
    };
    let geometry = e.mem.u32(this.addr() + 0xdbc);
    e.call(SET_FLOATS_004BC1F0, &args![geometry, zero, zero, last]);
}

// Translated from 007399d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the cards remaining (+0xda0) become the deck count (+0xd9c)
/// times 0x34.
pub fn fn_007399d0(e: &mut Engine, this: Ptr) {
    let decks = e.mem.u32(this.addr() + 0xd9c);
    e.mem.set_u32(this.addr() + 0xda0, decks.wrapping_mul(0x34));
}

/// Shows or hides a named object of the table (`004aae30` on +0xdb4, then `00450f90`).
fn set_table_object_culled(e: &mut Engine, this: Ptr, name: u32, culled: u32) {
    let root = e.mem.u32(this.addr() + 0xdb4);
    let object = e.call(FIND_OBJECT_004AAE30, &args![root, name]).u32();
    e.call(SET_APP_CULLED_00450F90, &args![object, culled]);
}

// Translated from 007399f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; points the hand arrow): with flag 0x10 shows the split hand's
/// arrow (`"Hand1_Arrow:0@#2..."`) and hides the hand's (`"Hand1_Arrow:0"`), else the
/// other way round; then updates the table model's node (`0043b4a0(1)` of `011d9098`).
pub fn fn_007399f0(e: &mut Engine, this: Ptr) {
    if fn_00734d30(e, this, 0x10) {
        set_table_object_culled(e, this, HAND_ARROW_NAME, 1);
        set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 0);
    } else {
        set_table_object_culled(e, this, HAND_ARROW_NAME, 0);
        set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 1);
    }
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
        let node = e.call(NODE_0043B4A0, &args![model, 1u32]).u32();
        e.call(UPDATE_00A59C60, &args![node, data]);
    });
}

// Translated from 00739ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): hides both hand arrows.
pub fn fn_00739ad0(e: &mut Engine, this: Ptr) {
    set_table_object_culled(e, this, HAND_ARROW_NAME, 1);
    set_table_object_culled(e, this, SPLIT_HAND_ARROW_NAME, 1);
}

// Translated from 0073ab40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0073ab40 - the /GS cookie check (00ec408c) on the label buffer is not translated.
/// Unnamed (`thiscall`; the bet label): `"%s %s%i"` of the casino's name, the label
/// setting (`011d3e08`) and the bet (the word at +4 of `011d9054`) into a 0x104-byte
/// buffer; when shorter than 23 characters it is written again as `"%s %s"`, padded with
/// spaces to 23 (each step writes a space over the last character and ends the text
/// after it), and the bet appended (`"%s%i"`). Set on `pTiles[2]` (+0x3c) with the trait
/// id at `011d9044`.
pub fn fn_0073ab40(e: &mut Engine, this: Ptr) {
    // The frame below `EBP`: the buffer is at `EBP - 0x110`.
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        let data = e.global::<u32>(BET_DATA_011D9054);
        let value = e.mem.u32(data + 4);
        let label = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
        let casino = e.global::<u32>(CASINO_011D9058);
        let name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
        e.call(
            FORMAT_TEXT,
            &args![buffer, 0x104u32, LABEL_VALUE_FORMAT, name, label, value],
        );
        let length = e.call(STRING_LENGTH_0044A670, &args![buffer]).i32();
        if length < 0x17 {
            let label = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
            let casino = e.global::<u32>(CASINO_011D9058);
            let name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, LABEL_FORMAT, name, label],
            );
            let mut i = 0i32;
            while i < 0x17 - length {
                let at = length.wrapping_add(i).wrapping_sub(1) as u32;
                e.mem.set_u8(buffer.wrapping_add(at), 0x20);
                let at = length.wrapping_add(i) as u32;
                e.mem.set_u8(buffer.wrapping_add(at), 0);
                i += 1;
            }
            let data = e.global::<u32>(BET_DATA_011D9054);
            let value = e.mem.u32(data + 4);
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, PADDED_VALUE_FORMAT, buffer, value],
            );
        }
        let which = e.global::<u32>(LABEL_TRAIT_011D9044);
        let tile = e.mem.u32(this.addr() + 0x3c);
        e.call(TILE_SET_STRING, &args![tile, which, buffer, 1u32]);
    });
}

/// `BlackJackMenu`'s vtable (`010708fc`), the `Menu` destructor body (`00a1c520`) and
/// `` `eh vector destructor iterator' `` (`00ec5fce`, stdcall: array, size, count,
/// destructor) with the card destructor `00914ae0`.
const BLACKJACK_MENU_VTABLE: u32 = 0x0107_08fc;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
const CARD_DESTRUCTOR_00914AE0: u32 = 0x0091_4ae0;
/// The player (`PlayerCharacter *` global), `00969b20` on it, the camera holder
/// (`011d90bc`, an `NiPointer`), `NiPointer` assignment (`0066b0d0`, `RET 4`) and
/// destruction (`0045cec0`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_00969B20: u32 = 0x0096_9b20;
const CAMERA_011D90BC: u32 = 0x011d_90bc;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The casino's entry by index (`00504e00`, `thiscall`, `RET 4`; its slot +0x14 gives a
/// model path), the model loader (`011c3b3c`) and its release by path (`0045a5e0`,
/// `RET 4`).
const CASINO_ENTRY_00504E00: u32 = 0x0050_4e00;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const MODEL_RELEASE_0045A5E0: u32 = 0x0045_a5e0;
/// The three terminal meshes released (`010709c0`, `01070988`, `01070950`) and the
/// `"_Shadow.NIF"` suffix (`01070944`).
const TABLE_MESH_ONE: u32 = 0x0107_09c0;
const TABLE_MESH_TWO: u32 = 0x0107_0988;
const TABLE_MESH_THREE: u32 = 0x0107_0950;
const SHADOW_SUFFIX: u32 = 0x0107_0944;
/// The shadow scene node (`00450b80`, cdecl: 1), `00b5d180` on it and `00572160` on the
/// table model.
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const SCENE_00B5D180: u32 = 0x00b5_d180;
const MODEL_00572160: u32 = 0x0057_2160;
/// `004702f0` (`thiscall`, `RET 4`): a hand list's deletion.
const LIST_DELETE_004702F0: u32 = 0x0047_02f0;
/// The copier (`00406d30`, cdecl, three words).
const COPY_TEXT: u32 = 0x0040_6d30;
/// The menu render: the renderer (`0043c4b0`) and its `007148c0` (`RET 8`), `0045c670`,
/// the culler (`008d80e0`), `00c4f270` (`RET 4`), `00709d50` (cdecl: the camera and a
/// `float`, 0.15 at `01030ff4`), the interface manager's texture (`007fd640`), the
/// culler's accumulator (`004a0fd0`, `RET 4`), `006629f0`,
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, cdecl, three words), `00b6c0d0` (cdecl,
/// three words) and `00c4f2d0`.
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
const RENDERER_007148C0: u32 = 0x0071_48c0;
const GETTER_0045C670: u32 = 0x0045_c670;
const CULLER_008D80E0: u32 = 0x008d_80e0;
const CULLER_00C4F270: u32 = 0x00c4_f270;
const CAMERA_00709D50: u32 = 0x0070_9d50;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
const TEXTURE_007FD640: u32 = 0x007f_d640;
const CULLER_SET_ACCUMULATOR: u32 = 0x004a_0fd0;
const GETTER_006629F0: u32 = 0x0066_29f0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
const CULLER_00C4F2D0: u32 = 0x00c4_f2d0;

/// The model path of the casino's entry `index` (`00504e00`, its slot +0x14).
fn casino_entry_path(e: &mut Engine, index: u32) -> u32 {
    let casino = e.global::<u32>(CASINO_011D9058);
    let entry = e.call(CASINO_ENTRY_00504E00, &args![casino, index]).u32();
    e.vcall(entry, 0x14, &args![]).u32()
}

/// Releases a model by path (`0045a5e0` on the model loader).
fn release_model(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(MODEL_RELEASE_0045A5E0, &args![loader, path]);
}

// Translated from 00731800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731800 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// The `BlackJackMenu` destructor body (unnamed in the engine map): the vtable; tells the
/// player (`00969b20`), clears the camera holder (`011d90bc`), releases the casino's
/// entry 8 model and the three table meshes, detaches the table model from the shadow
/// scene node (slot +0xe8), `00b5d180` on that node, `00572160` on the model and clears
/// its holder (`011d9098`); clears the geometry and animation pointers (+0xdb4..+0xdc0,
/// +0xdf4..+0xdfc, the three tables of 8 at +0xe0c, +0xe2c, +0xe4c); for the casino's
/// entries 0 to 5 releases each model and its shadow model (the path with everything
/// from its first `.` replaced by `"_Shadow.NIF"`, in a 0x104-byte buffer) and clears
/// the chip pointers (+0xdc4, +0xddc), which a second loop clears again; clears +0xe6c,
/// +0xe04, +0xe08, the three hands (+0xd90..+0xd98) and the statics `011d9054`,
/// `011d9058` and the singleton; deletes the hands (read after clearing, so never),
/// destroys the 0xd0 cards at +0x90 and runs the `Menu` destructor.
pub fn fn_00731800(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BLACKJACK_MENU_VTABLE);
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_00969B20, &args![player]);
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, 0u32]);
    let path = casino_entry_path(e, 8);
    release_model(e, path);
    for mesh in [TABLE_MESH_ONE, TABLE_MESH_TWO, TABLE_MESH_THREE] {
        release_model(e, mesh);
    }
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.vcall(scene, 0xe8, &args![model]);
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    e.call(SCENE_00B5D180, &args![scene]);
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.call(MODEL_00572160, &args![model]);
    e.call(NI_POINTER_ASSIGN, &args![TABLE_MODEL_011D9098, 0u32]);
    for offset in [0xdb4u32, 0xdb8, 0xdbc, 0xdc0, 0xdf4, 0xdf8, 0xdfc] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, 0u32]);
    for i in 0..8 {
        e.mem.set_u32(base + 0xe0c + 4 * i, 0);
        e.mem.set_u32(base + 0xe2c + 4 * i, 0);
        e.mem.set_u32(base + 0xe4c + 4 * i, 0);
    }
    // The frame below `EBP`: the path buffer at `EBP - 0x124`.
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        for i in 0..6 {
            let mut dot = 0;
            let path = casino_entry_path(e, i);
            e.call(COPY_TEXT, &args![buffer, 0x104u32, path]);
            let mut j = 0u32;
            while j < e.call(STRING_LENGTH_0044A670, &args![buffer]).u32() {
                if e.mem.u8(buffer + j) as i8 as i32 == 0x2e {
                    dot = buffer + j;
                    break;
                }
                j += 1;
            }
            if dot != 0 {
                e.call(COPY_TEXT, &args![dot, 0xfu32, SHADOW_SUFFIX]);
            }
            let path = casino_entry_path(e, i);
            release_model(e, path);
            release_model(e, buffer);
            e.mem.set_u32(base + 0xdc4 + 4 * i, 0);
            e.mem.set_u32(base + 0xddc + 4 * i, 0);
        }
    });
    for i in 0..6 {
        e.mem.set_u32(base + 0xdc4 + 4 * i, 0);
        e.mem.set_u32(base + 0xddc + 4 * i, 0);
    }
    for offset in [0xe6cu32, 0xe04, 0xe08, 0xd90, 0xd94, 0xd98] {
        e.mem.set_u32(base + offset, 0);
    }
    e.set_global(BET_DATA_011D9054, 0u32);
    e.set_global(CASINO_011D9058, 0u32);
    e.set_global(BLACKJACK_MENU, 0u32);
    for offset in [0xd94u32, 0xd98, 0xd90] {
        let hand = e.mem.u32(base + offset);
        if hand != 0 {
            e.call(LIST_DELETE_004702F0, &args![hand, 1u32]);
        }
    }
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 0x90, 0x10u32, 0xd0u32, CARD_DESTRUCTOR_00914AE0],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 00733500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00733500 - the C++ exception-unwinding frame (FS:[0] around the local NiPointer) is not translated.
/// Unnamed (no arguments; the table's render): `007148c0(renderer, 0, 4)`; the culler of
/// `0045c670()` (`008d80e0`, `00c4f270(1)`), the camera (`011d90bc`) given 0.15
/// (`00709d50`), a local `NiPointer` to the interface manager's texture (`007fd640`) made
/// the culler's accumulator; the scene is accumulated
/// (`BSShaderUtil::AccumulateScene(camera, shadow scene node, culler)`, the camera being
/// `006629f0` of the object when the holder is empty) and rendered (`00b6c0d0(camera,
/// texture, 0)`); the accumulator is cleared, the local released and destroyed and
/// `00c4f2d0` closes the culler.
pub fn fn_00733500(e: &mut Engine) {
    let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
    e.call(RENDERER_007148C0, &args![renderer, 0u32, 4u32]);
    let object = e.call(GETTER_0045C670, &args![]).u32();
    let culler = e.call(CULLER_008D80E0, &args![object]).u32();
    e.call(CULLER_00C4F270, &args![culler, 1u32]);
    let camera = e.call(FIRST_WORD, &args![CAMERA_011D90BC]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(CAMERA_00709D50, &args![camera, value]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let texture = e.call(TEXTURE_007FD640, &args![manager]).u32();
    // The local `NiPointer` at `EBP - 0x20`.
    e.with_stack(4, |e, pointer| {
        e.call(NI_POINTER_CONSTRUCT, &args![pointer, texture]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, held]);
        let view = if camera != 0 {
            camera
        } else {
            e.call(GETTER_006629F0, &args![object]).u32()
        };
        let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![view, scene, culler]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(RENDER_00B6C0D0, &args![view, held, 0u32]);
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
        e.call(CULLER_00C4F2D0, &args![culler]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// `Tile::Poke` (`00a01290`, `RET 8`) and the integer trait setter (`00700320`,
/// `RET 8`).
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_SET_INT: u32 = 0x0070_0320;
/// The bet tiles' label settings without a bet in play (`011d4c78`, `011d4234`,
/// `011d4e28`, `011d25ec`, `011d30fc`) and with one (`011d3da8`, `011d3354`,
/// `011d487c`, `011d37f8`, `011d33fc`).
const IDLE_BET_LABELS: [u32; 5] = [
    0x011d_4c78,
    0x011d_4234,
    0x011d_4e28,
    0x011d_25ec,
    0x011d_30fc,
];
const PLAY_BET_LABELS: [u32; 5] = [
    0x011d_3da8,
    0x011d_3354,
    0x011d_487c,
    0x011d_37f8,
    0x011d_33fc,
];
/// The random index (`00944460`, cdecl: the low and high bounds), `abs` (`00ec7d40`,
/// cdecl) and the card destructor (`00914ae0`).
const RANDOM_INDEX_00944460: u32 = 0x0094_4460;
const ABS_00EC7D40: u32 = 0x00ec_7d40;
/// A hand list's empty test (`008256d0`, a `bool`), its adds: `00726c60` and
/// `005ae3d0` (`RET 4`, the address of the item word) and `00905820` (`RET 4`).
const LIST_IS_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_ADD_00726C60: u32 = 0x0072_6c60;
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
const LIST_APPEND_00905820: u32 = 0x0090_5820;
/// `NiAVObject::GetProperty` (`00a59d30`, `RET 4`) and the property's type word
/// (`00441110`; the map's folded name is `PathingLocation::GetWorldspace`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE_00441110: u32 = 0x0044_1110;
/// The card mesh names: `"Hand1_0%i:%i"`, `"Hand1_0%i_Back:0"`, `"Hand2_0%i:%i"`,
/// `"Hand2_0%i_Back:0"`, `"Dealer_0%i:0"`, `"Dealer_0%i_Back:0"`.
const HAND_ONE_FRONT: u32 = 0x0107_0e34;
const HAND_ONE_BACK: u32 = 0x0107_0e20;
const HAND_TWO_FRONT: u32 = 0x0107_0dc4;
const HAND_TWO_BACK: u32 = 0x0107_0db0;
const DEALER_FRONT: u32 = 0x0107_0dfc;
const DEALER_BACK: u32 = 0x0107_0de8;

/// The card at `index` of the deck (+0x90, 0x10 bytes each).
fn card_at(this: Ptr, index: u32) -> u32 {
    this.addr()
        .wrapping_add(0x90)
        .wrapping_add(index.wrapping_shl(4))
}

// Translated from 00738cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::SetBetTiles` (Xbox PDB; no `this`, one `bool`): pokes the five bet
/// tiles (+0x44..+0x54 of the open menu, trait `0xfc4` with 1.0) and labels them: without
/// a bet in play with the idle labels and shows the bet tile (+0x40, trait `0xfa3` 1);
/// with one with the play labels, the text of tiles 4 to 8 at full alpha
/// (`SetTileTextAlpha` with 255), and hides the bet tile.
pub fn black_jack_menu_set_bet_tiles(e: &mut Engine, playing: u8) {
    let labels = if playing == 0 {
        IDLE_BET_LABELS
    } else {
        PLAY_BET_LABELS
    };
    for (i, setting) in labels.iter().enumerate() {
        let offset = 0x44 + 4 * i as u32;
        let tile = e.mem.u32(blackjack_menu(e) + offset);
        e.call(TILE_POKE, &args![tile, 0xfc4u32, x87_store32(1.0)]);
        let text = e.call(SETTING_STRING, &args![*setting]).u32();
        let tile = e.mem.u32(blackjack_menu(e) + offset);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        if playing != 0 {
            let alpha = f32::from_bits(x87_quiet_bits(e.global::<u32>(FULL_ALPHA)));
            let me = Ptr::new(blackjack_menu(e));
            black_jack_menu_set_tile_text_alpha(e, me, 4 + i as i32, alpha);
        }
    }
    let tile = e.mem.u32(blackjack_menu(e) + 0x40);
    let shown = if playing == 0 { 1u32 } else { 0 };
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, shown]);
}

/// Adds the address of the card at `index` to a hand through a local holding it: with
/// `first`, `00726c60` (an empty or missing hand) or `005ae3d0`; otherwise `00905820`.
fn add_card(e: &mut Engine, this: Ptr, hand: u32, index: u32, adder: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), card_at(this, index));
        e.call(adder, &args![hand, local]);
    });
}

// Translated from 007393f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; the deal): clears the two hand results (+0x2c, +0x30), deals a
/// card (`00739090(0, 0, 0)`) into the dealer's hand (+0xd90; `00726c60` when it is
/// missing or empty, else `005ae3d0`), one into the player's (+0xd94; the non-empty case
/// adds to the dealer's hand, as the code does), then one more to each with luck (the
/// dealer's with the dealer flag), appended (`00905820`). Returns the player's hand
/// value (`00738ff0`).
pub fn fn_007393f0(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, 0);
    for (hand, other) in [(0xd90u32, 0xd90u32), (0xd94, 0xd90)] {
        let index = fn_00739090(e, this, 0, 0, 0);
        let list = e.mem.u32(base + hand);
        let fresh = list == 0 || e.call(LIST_IS_EMPTY_008256D0, &args![list]).bool();
        if fresh {
            let list = e.mem.u32(base + hand);
            add_card(e, this, list, index, LIST_ADD_00726C60);
        } else {
            let list = e.mem.u32(base + other);
            add_card(e, this, list, index, LIST_ADD_005AE3D0);
        }
    }
    let dealer = e.mem.u32(base + 0xd90);
    let index = fn_00739090(e, this, dealer, 1, 1);
    let dealer = e.mem.u32(base + 0xd90);
    add_card(e, this, dealer, index, LIST_APPEND_00905820);
    let player = e.mem.u32(base + 0xd94);
    let index = fn_00739090(e, this, player, 1, 0);
    let player = e.mem.u32(base + 0xd94);
    add_card(e, this, player, index, LIST_APPEND_00905820);
    let player = e.mem.u32(base + 0xd94);
    fn_00738ff0(e, this, player)
}

// Translated from 00739090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00739090 - the C++ exception-unwinding frame (FS:[0] around the two local cards) is not translated.
/// Unnamed (`thiscall`, `RET 0xc`: a hand, a `bool` for luck, a `bool` for the dealer;
/// draws a card). Reshuffles (`007399d0`) below 5 cards left. Without a hand or luck, or
/// with a luck skill (+0x68) between 5 and 6 for the dealer's side (above 4 for the
/// dealer, below 6 for the player), picks a random index among the remaining cards
/// (`00944460(0, remaining)`, the count lowered first). Otherwise draws `|luck - 5|`
/// candidates and keeps the one that brings the hand closest below 22 (an ace counted 11
/// only up to 21; for hands up to 9 also closest to 11), and when that is not the first
/// candidate marks good luck (+0x78) or, for the dealer, bad luck (+0x79). The chosen
/// card is swapped with the last remaining one (`007324f0` through two local cards,
/// `00731710` and `00914ae0`), the count lowered, and the old count returned.
pub fn fn_00739090(e: &mut Engine, this: Ptr, hand: u32, luck: u8, dealer: u8) -> u32 {
    let base = this.addr();
    // The frame below `EBP`: the candidates at `EBP - 0x50` (five words), then the
    // lower candidate's index, the candidate count and the best distance, the two local
    // cards at `EBP - 0x30` and `EBP - 0x20`, and the chosen index at `EBP - 0x10`.
    e.with_stack(0x50, |e, frame| {
        let ebp = frame.addr() + 0x50;
        let local = |offset: u32| ebp - offset;
        let (picks, low_index, count, best) = (local(0x50), local(0x3c), local(0x38), local(0x34));
        let (swap_out, swap_in, chosen) = (local(0x30), local(0x20), local(0x10));
        fn_00731710(e, Ptr::new(swap_out));
        fn_00731710(e, Ptr::new(swap_in));
        e.mem.set_u32(chosen, 0);
        if e.mem.i32(base + 0xda0) < 5 {
            fn_007399d0(e, this);
        }
        let skill = |e: &Engine| e.mem.i32(base + 0x68);
        let lucky = hand != 0
            && luck != 0
            && ((dealer != 0 && skill(e) <= 4) || (dealer == 0 && skill(e) >= 6));
        if !lucky {
            let remaining = e.mem.u32(base + 0xda0);
            e.mem.set_u32(base + 0xda0, remaining.wrapping_sub(1));
            let index = e.call(RANDOM_INDEX_00944460, &args![0u32, remaining]).u32();
            e.mem.set_u32(chosen, index);
        } else {
            let current = fn_00738ff0(e, this, hand);
            let mut best_index = 0i32;
            e.mem.set_u32(low_index, 0);
            e.mem.set_i32(best, current);
            let mut low = current;
            let distance = e.call(ABS_00EC7D40, &args![skill(e).wrapping_sub(5)]).i32();
            e.mem.set_i32(count, distance);
            let mut i = 0i32;
            while i < e.mem.i32(count) {
                let remaining = e.mem.u32(base + 0xda0);
                let index = e.call(RANDOM_INDEX_00944460, &args![0u32, remaining]).u32();
                e.mem
                    .set_u32(picks.wrapping_add((i as u32).wrapping_mul(4)), index);
                i += 1;
            }
            let pick =
                |e: &Engine, i: i32| e.mem.u32(picks.wrapping_add((i as u32).wrapping_mul(4)));
            let mut j = 0i32;
            while j < e.mem.i32(count) {
                let rank = e.mem.i32(card_at(this, pick(e, j)));
                let mut points = fn_00738fb0(e, this, rank);
                if points == 0xb {
                    points = if points.wrapping_add(current) <= 0x15 {
                        0xb
                    } else {
                        1
                    };
                }
                if current > 9 {
                    let total = points.wrapping_add(current);
                    if total > e.mem.i32(best) && total < 0x16 {
                        e.mem.set_i32(best, total);
                        best_index = j;
                    }
                } else {
                    let total = points.wrapping_add(current);
                    if 0x15i32.wrapping_sub(total) < e.mem.i32(best) && total < 0x16 {
                        e.mem.set_i32(best, 0x15i32.wrapping_sub(total));
                        best_index = j;
                    }
                    if 0xbi32.wrapping_sub(total) < low && total < 0xc {
                        low = 0xbi32.wrapping_sub(total);
                        e.mem.set_i32(low_index, j);
                    }
                }
                let mark = |e: &mut Engine, picked: i32| {
                    if picked != 0 {
                        if dealer != 0 {
                            e.mem.set_u8(base + 0x79, 1);
                        } else {
                            e.mem.set_u8(base + 0x78, 1);
                        }
                    }
                };
                if e.mem.i32(best) < 4 {
                    let value = pick(e, best_index);
                    e.mem.set_u32(chosen, value);
                    mark(e, best_index);
                } else if low < e.mem.i32(best) && low < 2 {
                    let low_pick = e.mem.i32(low_index);
                    let value = pick(e, low_pick);
                    e.mem.set_u32(chosen, value);
                    mark(e, low_pick);
                } else {
                    let value = pick(e, best_index);
                    e.mem.set_u32(chosen, value);
                }
                j += 1;
            }
        }
        let picked = e.mem.u32(chosen);
        fn_007324f0(e, Ptr::new(swap_in), card_at(this, picked));
        let last = e.mem.u32(base + 0xda0);
        fn_007324f0(e, Ptr::new(swap_out), card_at(this, last));
        let last = e.mem.u32(base + 0xda0);
        fn_007324f0(e, Ptr::new(card_at(this, last)), swap_in);
        let picked = e.mem.u32(chosen);
        fn_007324f0(e, Ptr::new(card_at(this, picked)), swap_out);
        let drawn = e.mem.u32(base + 0xda0);
        e.mem.set_u32(base + 0xda0, drawn.wrapping_sub(1));
        e.call(CARD_DESTRUCTOR_00914AE0, &args![swap_in]);
        e.call(CARD_DESTRUCTOR_00914AE0, &args![swap_out]);
        drawn
    })
}

/// The texturing property of a named card mesh of the table (`004aae30` on the table
/// model's node `0043b4a0(hand)`; property 3 when its type word is 8 to 12, else none).
fn card_property(e: &mut Engine, hand: u32, name: u32) -> u32 {
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    let node = e.call(NODE_0043B4A0, &args![model, hand]).u32();
    let object = e.call(FIND_OBJECT_004AAE30, &args![node, name]).u32();
    let usable = e.call(GET_PROPERTY, &args![object, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        e.call(PROPERTY_TYPE_00441110, &args![property]).i32() >= 8 && {
            let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
            e.call(PROPERTY_TYPE_00441110, &args![property]).i32() <= 0xc
        }
    };
    if usable {
        e.call(GET_PROPERTY, &args![object, 3u32]).u32()
    } else {
        0
    }
}

// Translated from 007396e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007396e0 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// `BlackJackMenu::SwapCardTextures` (Xbox PDB, `RET 0xc`: the hand (0 the player's, 1
/// the split, 2 the dealer's), the card's place 1 to 7, the card). Names the card's front
/// and back meshes for the hand (`"Hand1_0%i:%i"` with the place and place - 1 and
/// `"Hand1_0%i_Back:0"`, the `"Hand2_..."` pair, or `"Dealer_0%i:0"` and its back, in
/// two 0x1e-byte buffers) and gives each mesh's texturing property (slot +0xf0 with 0, 0)
/// the card's front (+8) and back (+0xc) textures. Nothing for other places or hands, or
/// without a card.
pub fn black_jack_menu_swap_card_textures(
    e: &mut Engine,
    _this: Ptr,
    hand: i32,
    place: i32,
    card: u32,
) {
    let hand = hand.wrapping_add(1);
    if !(1..=7).contains(&place) || card == 0 {
        return;
    }
    // The frame below `EBP`: the front name at `EBP - 0x28`, the back name at
    // `EBP - 0x4c`.
    e.with_stack(0x48, |e, frame| {
        let back = frame.addr();
        let front = frame.addr() + 0x24;
        match hand {
            1 => {
                e.call(
                    FORMAT_TEXT,
                    &args![front, 0x1eu32, HAND_ONE_FRONT, place, place.wrapping_sub(1)],
                );
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, HAND_ONE_BACK, place]);
            }
            2 => {
                e.call(
                    FORMAT_TEXT,
                    &args![front, 0x1eu32, HAND_TWO_FRONT, place, place.wrapping_sub(1)],
                );
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, HAND_TWO_BACK, place]);
            }
            3 => {
                e.call(FORMAT_TEXT, &args![front, 0x1eu32, DEALER_FRONT, place]);
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, DEALER_BACK, place]);
            }
            _ => return,
        }
        for (name, texture) in [(front, 8u32), (back, 0xc)] {
            let property = card_property(e, hand as u32, name);
            if property != 0 {
                let image = e.call(FIRST_WORD, &args![card + texture]).u32();
                e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
            }
        }
    });
}

/// `"Hand1_0%i_Shadow..."`, `"Dealer_0%i_Shadow..."`, `"Hand2_0%i_Shadow..."`.
const HAND_ONE_SHADOW: u32 = 0x0107_0e0c;
const DEALER_SHADOW: u32 = 0x0107_0dd4;
const HAND_TWO_SHADOW: u32 = 0x0107_0d9c;

/// One hand's card meshes: the root object's offset in the menu, the front name format
/// (and whether it takes the place minus one too), the back and shadow formats, and the
/// offset of the hand's card count.
struct HandMeshes {
    root: u32,
    front: u32,
    front_takes_previous: bool,
    back: u32,
    shadow: u32,
    count: u32,
}

/// The player's (+0xdb8, count +0x84), the dealer's (+0xdc0, count +0x8c) and the split
/// (+0xdbc, count +0x88) hands, in the order the code walks them.
const HAND_MESHES: [HandMeshes; 3] = [
    HandMeshes {
        root: 0xdb8,
        front: HAND_ONE_FRONT,
        front_takes_previous: true,
        back: HAND_ONE_BACK,
        shadow: HAND_ONE_SHADOW,
        count: 0x84,
    },
    HandMeshes {
        root: 0xdc0,
        front: DEALER_FRONT,
        front_takes_previous: false,
        back: DEALER_BACK,
        shadow: DEALER_SHADOW,
        count: 0x8c,
    },
    HandMeshes {
        root: 0xdbc,
        front: HAND_TWO_FRONT,
        front_takes_previous: true,
        back: HAND_TWO_BACK,
        shadow: HAND_TWO_SHADOW,
        count: 0x88,
    },
];

/// For each hand and card place 1 to 7, names the front, back and shadow meshes (three
/// 0x1e-byte buffers: `EBP - 0x44`, `- 0x68`, `- 0x24`) and sets each one's app-culled
/// flag (`00450f90`) to what `culled` answers for the place (the first mesh is looked up
/// before it is asked); then updates the table model (`00a59c60` with a fresh update
/// data).
fn cull_card_meshes(e: &mut Engine, this: Ptr, culled: impl Fn(&Engine, &HandMeshes, i32) -> u32) {
    let base = this.addr();
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let front = ebp - 0x44;
        let back = ebp - 0x68;
        let shadow = ebp - 0x24;
        let data = ebp - 0x80;
        for hand in &HAND_MESHES {
            for place in 1..=7i32 {
                if hand.front_takes_previous {
                    e.call(
                        FORMAT_TEXT,
                        &args![front, 0x1eu32, hand.front, place, place - 1],
                    );
                } else {
                    e.call(FORMAT_TEXT, &args![front, 0x1eu32, hand.front, place]);
                }
                e.call(FORMAT_TEXT, &args![back, 0x1eu32, hand.back, place]);
                e.call(FORMAT_TEXT, &args![shadow, 0x1eu32, hand.shadow, place]);
                let root = e.mem.u32(base + hand.root);
                let object = e.call(FIND_OBJECT_004AAE30, &args![root, front]).u32();
                let flag = culled(e, hand, place);
                e.call(SET_APP_CULLED_00450F90, &args![object, flag]);
                for name in [back, shadow] {
                    let root = e.mem.u32(base + hand.root);
                    let object = e.call(FIND_OBJECT_004AAE30, &args![root, name]).u32();
                    e.call(SET_APP_CULLED_00450F90, &args![object, flag]);
                }
            }
        }
        let data = e
            .call(
                UPDATE_DATA_INIT_0043D410,
                &args![data, x87_store32(0.0), 0u32, 0u32],
            )
            .u32();
        let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
        e.call(UPDATE_00A59C60, &args![model, data]);
    });
}

// Translated from 00737e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00737e50 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// Unnamed (`thiscall`; shows the dealt cards): for the player's, the dealer's and the
/// split hand, shows the card meshes (front, back, shadow) of the places below the hand's
/// card count (+0x84, +0x8c, +0x88) and hides the others; then updates the table model.
pub fn fn_00737e50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    cull_card_meshes(e, this, |e, hand, place| {
        u32::from(place >= e.mem.i32(base + hand.count))
    });
}

// Translated from 00738270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00738270 - the /GS cookie check (00ec408c) on the name buffers is not translated.
/// Unnamed (`thiscall`): shows every card mesh (front, back, shadow) of the three hands'
/// seven places, then updates the table model.
pub fn fn_00738270(e: &mut Engine, this: Ptr) {
    cull_card_meshes(e, this, |_, _, _| 0);
}

/// The casino's data (`00504e60`, `thiscall`) and its entries by index (`00504e20`,
/// `RET 4`); `ModelLoader::LoadFile` (`00447080`, `RET 0x18`); the texture manager
/// (`011dea10`) and `TES::CreateTextureImage` (`004568c0`, `RET 0x10`);
/// `SwapPlatformLanguageTexturePath` (`004b7240`, cdecl, three words); `004812f0`
/// (cdecl: a text and a set of characters; the text from the first of them).
const CASINO_DATA_00504E60: u32 = 0x0050_4e60;
const CASINO_DECK_00504E20: u32 = 0x0050_4e20;
const MODEL_LOAD_FILE: u32 = 0x0044_7080;
const TEXTURE_MANAGER_011DEA10: u32 = 0x011d_ea10;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const SWAP_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
const FIND_CHARACTERS_004812F0: u32 = 0x0048_12f0;
/// `"\"` (`01013444`), `"%s%s"` (`0101996c`), `"Textures\Terminals\..."` (`01070a18`),
/// `"%s%s_Back.dds"` (`01070a08`), `"%s%s_%c%c%c.dds"` (`010709f8`).
const BACKSLASH_TEXT: u32 = 0x0101_3444;
const TWO_STRINGS_FORMAT: u32 = 0x0101_996c;
const CARD_TEXTURE_FOLDER: u32 = 0x0107_0a18;
const CARD_BACK_FORMAT: u32 = 0x0107_0a08;
const CARD_FACE_FORMAT: u32 = 0x0107_09f8;
/// The defaults of the two casino values: 1.5 (`01016088`) and 0.25 (`0101622c`), and
/// the dimmed alpha 128 (`0101e704`).
const DEFAULT_WIN_RATIO: u32 = 0x0101_6088;
const DEFAULT_SHUFFLE: u32 = 0x0101_622c;
const DIM_ALPHA: u32 = 0x0101_e704;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `ModelLoader::LoadFile(path, 0, 1, 0, 0, 0)` on the model loader.
fn load_model(e: &mut Engine, path: u32) -> u32 {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(
        MODEL_LOAD_FILE,
        &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00731d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00731d70 - the C++ exception-unwinding frame (FS:[0]: the local card and texture pointer) and the /GS cookie check (00ec408c) are not translated.
/// `BlackJackMenu::InitCasinoData` (Xbox PDB; `thiscall`, a `bool`): false without a
/// casino (`011d9058`). Copies the casino data's two floats (+4 into +0x60, +0 into
/// +0x64), deck count (+0x24 into +0xd9c) and flag (+0x34 into +0xdac); loads the table
/// model (entry 8, into +0xdb4) and the six chip models and their shadow models (the path
/// with everything from its first `.` replaced by `"_Shadow.NIF"`; +0xdc4, +0xddc). At
/// least one deck; zero floats become 1.5 and 0.25. For each deck: the deck entry
/// (`00504e20(deck + 7)`) names a texture path; past its first two `\` the folder (up to
/// the next `\`, kept with it) and the card name (up to `_`) are cut out (byte copies into
/// stack buffers); the folder is prefixed with `"Textures\Terminals\..."`; the back
/// texture (`"%s%s_Back.dds"`) and the 52 face textures (`"%s%s_%c%c%c.dds"` with the
/// suit letter `h`, `c`, `d`, `s` and the two-digit number 01 to 13) are made
/// (`SwapPlatformLanguageTexturePath`, `TES::CreateTextureImage`) and the cards (rank
/// `(i + 12) % 13`, suit `i / 13`, face and back) copied into the deck (+0x90). Then the
/// cards remaining (+0xda0) are 0x34 per deck, the three counts (+0x84..) 1, the byte at
/// +0x80 cleared; true.
pub fn black_jack_menu_init_casino_data(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if e.global::<u32>(CASINO_011D9058) == 0 {
        return false;
    }
    let data = |e: &mut Engine| {
        let casino = e.global::<u32>(CASINO_011D9058);
        e.call(CASINO_DATA_00504E60, &args![casino]).u32()
    };
    let d = data(e);
    e.mem.set_u32(base + 0x60, x87_quiet_bits(e.mem.u32(d + 4)));
    let d = data(e);
    e.mem.set_u32(base + 0x64, x87_quiet_bits(e.mem.u32(d)));
    let d = data(e);
    let decks = e.mem.u32(d + 0x24);
    e.mem.set_u32(base + 0xd9c, decks);
    let d = data(e);
    let flag = e.mem.u8(d + 0x34);
    e.mem.set_u8(base + 0xdac, flag);
    let path = casino_entry_path(e, 8);
    let table = load_model(e, path);
    e.mem.set_u32(base + 0xdb4, table);
    // The frame below `EBP`: `local(n)` is `EBP - n` (0x4d0 bytes).
    e.with_stack(0x4d0, |e, frame| {
        let ebp = frame.addr() + 0x4d0;
        let local = |offset: u32| ebp - offset;
        let model_path = local(0x49c);
        for i in 0..6u32 {
            let path = casino_entry_path(e, i);
            let chip = load_model(e, path);
            e.mem.set_u32(base + 0xdc4 + 4 * i, chip);
            let mut dot = 0;
            let path = casino_entry_path(e, i);
            e.call(COPY_TEXT, &args![model_path, 0x104u32, path]);
            let mut j = 0u32;
            while j < e.call(STRING_LENGTH_0044A670, &args![model_path]).u32() {
                if e.mem.u8(model_path + j) as i8 as i32 == 0x2e {
                    dot = model_path + j;
                    break;
                }
                j += 1;
            }
            if dot != 0 {
                e.call(COPY_TEXT, &args![dot, 0xfu32, SHADOW_SUFFIX]);
                let shadow = load_model(e, model_path);
                e.mem.set_u32(base + 0xddc + 4 * i, shadow);
            }
        }
        if e.mem.u32(base + 0xd9c) == 0 {
            e.mem.set_u32(base + 0xd9c, 1);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        // FCOMP against 0.0; TEST AH,0x44; JP: the default only for exactly 0.0.
        if x87_load32(e.mem.u32(base + 0x60)) == zero {
            let value = x87_quiet_bits(e.global::<u32>(DEFAULT_WIN_RATIO));
            e.mem.set_u32(base + 0x60, value);
        }
        if x87_load32(e.mem.u32(base + 0x64)) == zero {
            let value = x87_quiet_bits(e.global::<u32>(DEFAULT_SHUFFLE));
            e.mem.set_u32(base + 0x64, value);
        }
        let card = local(0x14c);
        fn_00731710(e, Ptr::new(card));
        let suits = local(0x388);
        e.mem.write(suits, b"hcds");
        let path = local(0x11c);
        let folder = local(0x13c);
        let name = local(0x274);
        let full = local(0x384);
        let swapped = local(0x254);
        let texture = local(0x4b0);
        let mut deck = 0i32;
        while deck < e.mem.i32(base + 0xd9c) {
            let casino = e.global::<u32>(CASINO_011D9058);
            let entry = e
                .call(CASINO_DECK_00504E20, &args![casino, (deck + 7) as u32])
                .u32();
            let text = e.call(NAME_OF_00408DA0, &args![entry]).u32();
            e.call(COPY_TEXT, &args![path, 0x104u32, text]);
            let mut p = e
                .call(FIND_CHARACTERS_004812F0, &args![path, BACKSLASH_TEXT])
                .u32()
                .wrapping_add(1);
            p = e
                .call(FIND_CHARACTERS_004812F0, &args![p, BACKSLASH_TEXT])
                .u32()
                .wrapping_add(1);
            let mut k = 0u32;
            while p != 0 && e.mem.u8(p) as i8 as i32 != 0x5c {
                let byte = e.mem.u8(p);
                e.mem.set_u8(folder.wrapping_add(k), byte);
                p = p.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            p = p.wrapping_add(1);
            e.mem.set_u8(folder.wrapping_add(k), 0x5c);
            k = k.wrapping_add(1);
            e.mem.set_u8(folder.wrapping_add(k), 0);
            let mut k = 0u32;
            while p != 0 && e.mem.u8(p) as i8 as i32 != 0x5f {
                let byte = e.mem.u8(p);
                e.mem.set_u8(name.wrapping_add(k), byte);
                p = p.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            e.mem.set_u8(name.wrapping_add(k), 0);
            e.call(
                FORMAT_TEXT,
                &args![
                    path,
                    0x104u32,
                    TWO_STRINGS_FORMAT,
                    CARD_TEXTURE_FOLDER,
                    folder
                ],
            );
            let mut tens = b'0';
            let mut units = b'1';
            let mut suit = 0u32;
            e.call(
                FORMAT_TEXT,
                &args![full, 0x104u32, CARD_BACK_FORMAT, path, name],
            );
            e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
            e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![full, swapped, 0x104u32]);
            let manager = e.global::<u32>(TEXTURE_MANAGER_011DEA10);
            e.call(
                CREATE_TEXTURE_IMAGE,
                &args![manager, swapped, texture, 0u32, 0u32],
            );
            for i in 0..0x34i32 {
                e.mem.set_i32(card, (i + 0xc) % 0xd);
                e.mem.set_i32(card + 4, i / 0xd);
                let letter = e.mem.u8(suits.wrapping_add(suit)) as i8 as i32;
                e.call(
                    FORMAT_TEXT,
                    &args![
                        full,
                        0x104u32,
                        CARD_FACE_FORMAT,
                        path,
                        name,
                        letter,
                        tens as i8 as i32,
                        units as i8 as i32
                    ],
                );
                e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![full, swapped, 0x104u32]);
                let manager = e.global::<u32>(TEXTURE_MANAGER_011DEA10);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![manager, swapped, card + 8, 0u32, 0u32],
                );
                e.call(NI_POINTER_COPY_006E5CC0, &args![card + 0xc, texture]);
                let slot = (deck.wrapping_mul(0x34).wrapping_add(i)) as u32;
                fn_007324f0(e, Ptr::new(card_at(this, slot)), card);
                units = units.wrapping_add(1);
                if units as i8 as i32 > 0x39 {
                    units = b'0';
                    tens = tens.wrapping_add(1);
                }
                if tens == b'1' && units == b'4' {
                    tens = b'0';
                    units = b'1';
                    suit = suit.wrapping_add(1);
                }
            }
            e.call(NI_POINTER_DESTRUCT, &args![texture]);
            deck += 1;
        }
        let decks = e.mem.u32(base + 0xd9c);
        e.mem.set_u32(base + 0xda0, decks.wrapping_mul(0x34));
        for i in 0..3 {
            e.mem.set_u32(base + 0x84 + 4 * i, 1);
        }
        e.mem.set_u8(base + 0x80, 0);
        e.call(CARD_DESTRUCTOR_00914AE0, &args![card]);
    });
    true
}

// Translated from 00738560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::UpdateTileAlpha` (Xbox PDB, `RET 4`: a tile index; nothing in state
/// 0xa). Dims (128) or lights (255) the text of the button tiles by the flags
/// (`00734d30`; jump table `00738c08`): tile 3 by 0x800 (split hand, flag 0x10) or 0x400;
/// tile 4 lit by 0x100 (split) or 0x40; tile 5 dimmed on the split hand, else lit by 4;
/// tile 6 dimmed by 0x400 (split), else lit only by 8 without 0x800; tile 7 dimmed by
/// 1; tile 8 lit; tile 9 untouched; tile 10 updates them all (on the split hand 3 and 4
/// by 0x800 and 0x100, 5 and 7 dimmed, 8 lit, 6 by 0x400; otherwise all of 3, 4, 5, 7
/// dimmed by 0x400 or 0x20, else each by its flag, then 8 dimmed by 0x20 unless 8 is set
/// without 0x800, and 6 lit only by 8 without 0x800).
pub fn black_jack_menu_update_tile_alpha(e: &mut Engine, this: Ptr, tile: i32) {
    if e.mem.u32(this.addr() + 0x28) == 0xa {
        return;
    }
    let dim = f32::from_bits(x87_quiet_bits(e.global::<u32>(DIM_ALPHA)));
    let full = f32::from_bits(x87_quiet_bits(e.global::<u32>(FULL_ALPHA)));
    let flag = |e: &mut Engine, mask: u32| fn_00734d30(e, this, mask);
    let set = |e: &mut Engine, index: i32, alpha: f32| {
        black_jack_menu_set_tile_text_alpha(e, this, index, alpha);
    };
    let pick = |lit: bool| if lit { full } else { dim };
    match tile {
        3 => {
            let alpha = if flag(e, 0x10) {
                pick(!flag(e, 0x800))
            } else {
                pick(!flag(e, 0x400))
            };
            set(e, 3, alpha);
        }
        4 => {
            let alpha = if flag(e, 0x10) {
                pick(flag(e, 0x100))
            } else {
                pick(flag(e, 0x40))
            };
            set(e, 4, alpha);
        }
        5 => {
            if flag(e, 0x10) {
                set(e, 5, dim);
            } else {
                let alpha = pick(flag(e, 4));
                set(e, 5, alpha);
            }
        }
        6 => {
            let alpha = if flag(e, 0x10) {
                pick(!flag(e, 0x400))
            } else {
                pick(!flag(e, 0x800) && flag(e, 8))
            };
            set(e, 6, alpha);
        }
        7 => {
            let alpha = pick(!flag(e, 1));
            set(e, 7, alpha);
        }
        8 => set(e, 8, full),
        0xa => {
            if flag(e, 0x10) {
                if flag(e, 0x800) {
                    set(e, 3, dim);
                    set(e, 4, dim);
                } else {
                    set(e, 3, full);
                    let alpha = pick(flag(e, 0x100));
                    set(e, 4, alpha);
                }
                set(e, 5, dim);
                set(e, 7, dim);
                set(e, 8, full);
                let alpha = pick(!flag(e, 0x400));
                set(e, 6, alpha);
            } else {
                if flag(e, 0x400) || flag(e, 0x20) {
                    for index in [3, 4, 5, 7] {
                        set(e, index, dim);
                    }
                } else {
                    set(e, 3, full);
                    let alpha = pick(flag(e, 0x40));
                    set(e, 4, alpha);
                    let alpha = pick(flag(e, 4));
                    set(e, 5, alpha);
                    let alpha = pick(!flag(e, 1));
                    set(e, 7, alpha);
                }
                let dimmed = flag(e, 0x20) && (!flag(e, 8) || flag(e, 0x800));
                set(e, 8, pick(!dimmed));
                let alpha = pick(!flag(e, 0x800) && flag(e, 8));
                set(e, 6, alpha);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Batch n059x: the creation, the 3D set-up, the bet chips and the clicks.

/// The chip stack step `0.18` (`double`, `01070e78`) and the fixed stack heights
/// 0.9 (`010177e8`), 1.8 (`01053154`), 1.44 (`01070e74`), 0.36 (`01070e70`), 1.62
/// (`01070e6c`) and 0.18 (`01070e68`), `float`s.
const CHIP_STEP_01070E78: u32 = 0x0107_0e78;
const CHIP_HEIGHT_010177E8: u32 = 0x0101_77e8;
const CHIP_HEIGHT_01053154: u32 = 0x0105_3154;
const CHIP_HEIGHT_01070E74: u32 = 0x0107_0e74;
const CHIP_HEIGHT_01070E70: u32 = 0x0107_0e70;
const CHIP_HEIGHT_01070E6C: u32 = 0x0107_0e6c;
const CHIP_HEIGHT_01070E68: u32 = 0x0107_0e68;

/// One chip model's height, the third float `004bc1f0` gives it.
#[derive(Clone, Copy)]
enum ChipHeight {
    /// `FLDZ`.
    Zero,
    /// A `float` constant at its address.
    Constant(u32),
    /// The bet (+0x6c, read when the chip is set) through the function, times 0.18.
    OfBet(fn(i32) -> i32),
    /// A count computed beforehand, times 0.18.
    Count(i32),
}

/// `count * 0.18` stored as a `float` (`FILD`, `FMUL double`, `FSTP`), then copied
/// (`FLD`/`FSTP`).
fn chip_step(e: &Engine, count: i32) -> u32 {
    let step: f64 = e.global(CHIP_STEP_01070E78);
    x87_quiet_bits(x87_store32(x87_mul(f64::from(count), step)))
}

/// Sets each chip model (the word at its offset) to `(0, 0, height)` (`004bc1f0`), in
/// the order given; each height is computed just before its call.
fn set_chip_heights(e: &mut Engine, this: Ptr, heights: &[(u32, ChipHeight)]) {
    let base = this.addr();
    for &(offset, height) in heights {
        let bits = match height {
            ChipHeight::Zero => x87_store32(0.0),
            ChipHeight::Constant(addr) => x87_quiet_bits(e.global::<u32>(addr)),
            ChipHeight::OfBet(count) => {
                let bet = e.mem.i32(base + 0x6c);
                chip_step(e, count(bet))
            }
            ChipHeight::Count(count) => chip_step(e, count),
        };
        let chip = e.mem.u32(base + offset);
        let zero = x87_store32(0.0);
        e.call(SET_FLOATS_004BC1F0, &args![chip, zero, zero, bits]);
    }
}

/// Updates the table model (`00a59c60`) with fresh update data (`0043d410` with 0.0, 0,
/// 0) in the 12-byte local at `data`.
fn update_table_model_at(e: &mut Engine, data: u32) {
    e.call(
        UPDATE_DATA_INIT_0043D410,
        &args![data, x87_store32(0.0), 0u32, 0u32],
    );
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.call(UPDATE_00A59C60, &args![model, data]);
}

// Translated from 00739b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlackJackMenu::SetBetChips` (Xbox PDB, `thiscall`): stacks the twelve chip models
/// (+0xdc4..+0xdf0, the six chips and their shadows) to show the bet (+0x6c): each gets
/// `(0, 0, height)` (`004bc1f0`), the height 0, -5 (hidden) or a stack height, in eight
/// bet ranges (below 7, 40, up to 100, 300, 500, below 1000, 1500, and above; the last
/// stacks `bet / 500 - 3`, at most 10, chips of 0.18); then updates the table model.
/// Some ranges set a model out of order or one model twice, as the code does.
pub fn black_jack_menu_set_bet_chips(e: &mut Engine, this: Ptr) {
    use ChipHeight::{Constant, Count, OfBet, Zero};
    let base = this.addr();
    let bet = |e: &Engine| e.mem.i32(base + 0x6c);
    let low = Constant(MINUS_FIVE_0102CAF8);
    if bet(e) < 7 {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, OfBet(|bet| bet)),
                (0xdc8, Zero),
                (0xdcc, Zero),
                (0xdd0, Zero),
                (0xdd4, Zero),
                (0xdd8, Zero),
                (0xddc, Zero),
                (0xde0, low),
                (0xde4, low),
                (0xde8, low),
                (0xdec, low),
                (0xdf0, low),
            ],
        );
    } else if bet(e) < 0x28 {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, OfBet(|bet| bet % 5 + 5)),
                (0xdc8, OfBet(|bet| bet / 5 - 1)),
                (0xdcc, Zero),
                (0xdd0, Zero),
                (0xdd4, Zero),
                (0xdd8, Zero),
                (0xddc, Zero),
            ],
        );
        let height = if bet(e) <= 0xa { low } else { Zero };
        set_chip_heights(
            e,
            this,
            &[
                (0xde0, height),
                (0xde4, low),
                (0xde8, low),
                (0xdec, low),
                (0xdf0, low),
            ],
        );
    } else if bet(e) <= 0x64 {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_010177E8)),
                (0xdc8, Constant(CHIP_HEIGHT_010177E8)),
                (0xdcc, OfBet(|bet| bet / 10 - 3)),
                (0xdd0, Zero),
                (0xdd4, Zero),
                (0xdd8, Zero),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
                (0xde8, low),
                (0xdec, low),
                (0xdf0, low),
            ],
        );
    } else if bet(e) <= 0x12c {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_01053154)),
                (0xdc8, Constant(CHIP_HEIGHT_01070E74)),
                (0xdcc, OfBet(|bet| bet % 0x19 / 10 + 5)),
                (0xdd0, OfBet(|bet| bet / 0x19 - 4)),
                (0xdd4, Zero),
                (0xdd8, Zero),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
            ],
        );
        let height = if bet(e) / 0x19 - 4 == 0 { low } else { Zero };
        set_chip_heights(e, this, &[(0xde8, height), (0xdec, low), (0xdf0, low)]);
    } else if bet(e) <= 0x1f4 {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_01053154)),
                (0xdc8, Constant(CHIP_HEIGHT_01070E74)),
                (0xdcc, Constant(CHIP_HEIGHT_010177E8)),
                (0xdd4, Constant(CHIP_HEIGHT_01070E70)),
                (0xdd0, OfBet(|bet| bet / 0x19 - 0xc)),
                (0xdd8, Zero),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
                (0xde8, Zero),
                (0xdec, Zero),
                (0xdf0, low),
            ],
        );
    } else if bet(e) < 0x3e8 {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_01053154)),
                (0xdc8, Constant(CHIP_HEIGHT_01053154)),
                (0xdcc, Constant(CHIP_HEIGHT_01070E6C)),
                (0xdd0, OfBet(|bet| bet % 100 / 0x19 + 6)),
                (0xdd4, OfBet(|bet| bet / 100 - 3)),
                (0xdd8, Zero),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
                (0xde8, Zero),
            ],
        );
        let height = if bet(e) / 100 - 3 == 0 { low } else { Zero };
        set_chip_heights(e, this, &[(0xdec, height), (0xdec, Zero), (0xdf0, low)]);
    } else if bet(e) < 0x5dc {
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_01053154)),
                (0xdc8, Constant(CHIP_HEIGHT_01053154)),
                (0xdcc, Constant(CHIP_HEIGHT_01070E6C)),
                (0xdd0, Constant(CHIP_HEIGHT_01053154)),
                (0xdd8, Constant(CHIP_HEIGHT_01070E68)),
                (0xdd4, OfBet(|bet| bet / 100 - 9)),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
                (0xde8, Zero),
                (0xdec, Zero),
                (0xdf0, Zero),
            ],
        );
    } else {
        let count = if bet(e) / 0x1f4 - 3 > 0xa {
            0xa
        } else {
            bet(e) / 0x1f4 - 3
        };
        set_chip_heights(
            e,
            this,
            &[
                (0xdc4, Constant(CHIP_HEIGHT_01053154)),
                (0xdc8, Constant(CHIP_HEIGHT_01053154)),
                (0xdcc, Constant(CHIP_HEIGHT_01053154)),
                (0xdd0, Constant(CHIP_HEIGHT_01053154)),
                (0xdd4, Constant(CHIP_HEIGHT_01053154)),
                (0xdd8, Count(count)),
                (0xddc, Zero),
                (0xde0, Zero),
                (0xde4, Zero),
                (0xde8, Zero),
                (0xdec, Zero),
            ],
        );
        let height = if bet(e) == 0x5dc { low } else { Zero };
        set_chip_heights(e, this, &[(0xdf0, height)]);
    }
    // The update data at `EBP - 0xc`.
    e.with_stack(0xc, |e, data| update_table_model_at(e, data.addr()));
}

/// `"object1"` (`01070ba0`, the camera object) and `"Deck:0"` (`01070a3c`).
const CAMERA_OBJECT_NAME: u32 = 0x0107_0ba0;
const DECK_OBJECT_NAME: u32 = 0x0107_0a3c;
/// The allocator (`00aa13e0`, cdecl) and the `NiNode` constructor (`00a5ecb0`, `RET 4`,
/// with 0) the table model's root is made with (0xac bytes).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NODE_CONSTRUCT_00A5ECB0: u32 = 0x00a5_ecb0;
/// `NiObjectNET::GetController` (`00a5c570`, `RET 4`) with the RTTI record `011f36ac`
/// (unconfirmed which class), the fixed-string constructor (`00438170`, `RET 4`) and
/// destructor (`004381b0`), the sequence lookup `0047a520` (`RET 4`),
/// `NiControllerManager::DeactivateAll` (`0048fef0`, `RET 4`, a `float`) and
/// `NiTimeController::StartAnimations` (`00a6d2d0`, cdecl).
const GET_CONTROLLER: u32 = 0x00a5_c570;
const CONTROLLER_RTTI_011F36AC: u32 = 0x011f_36ac;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const SEQUENCE_0047A520: u32 = 0x0047_a520;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// The camera maker `007c2180` (`thiscall`), `NiAVObject::GetWorldBound` (`0043d450`)
/// and the bound's radius (`0084d030`, `ST0`), `3.0` (`double`, `01021928`) and
/// `ShadowSceneNode::AddAllLightsFromNode_Recurse` (`00b5ca70`, `RET 8`).
const MAKE_CAMERA_007C2180: u32 = 0x007c_2180;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const THREE_01021928: u32 = 0x0102_1928;
const ADD_ALL_LIGHTS: u32 = 0x00b5_ca70;

/// One model's animation sequences: the model's offset, where its controller is kept,
/// and each sequence's name and the offset it is kept at.
struct ModelSequences {
    model: u32,
    controller: u32,
    sequences: &'static [(u32, u32)],
}

/// The table's (`"Players_Discard"`, `"Dealers_Discard"`), the player's hand
/// (`"Hand1_Discard"`, `"Hand1_01"`..`"Hand1_07"`), the split hand's (`"Hand2_..."`) and
/// the dealer's (`"Dealer_Discard"`, `"Dealer_01"`..`"Dealer_07"`, `"Dealer_Reveal"`).
const MODEL_SEQUENCES: [ModelSequences; 4] = [
    ModelSequences {
        model: 0xdb4,
        controller: 0xdf4,
        sequences: &[(0x0107_0b90, 0xe04), (0x0107_0b80, 0xe08)],
    },
    ModelSequences {
        model: 0xdb8,
        controller: 0xdf8,
        sequences: &[
            (0x0107_0b70, 0xe0c),
            (0x0107_0b64, 0xe10),
            (0x0107_0b58, 0xe14),
            (0x0107_0b4c, 0xe18),
            (0x0107_0b40, 0xe1c),
            (0x0107_0b34, 0xe20),
            (0x0107_0b28, 0xe24),
            (0x0107_0b1c, 0xe28),
        ],
    },
    ModelSequences {
        model: 0xdbc,
        controller: 0xdfc,
        sequences: &[
            (0x0107_0b0c, 0xe2c),
            (0x0107_0b00, 0xe30),
            (0x0107_0af4, 0xe34),
            (0x0107_0ae8, 0xe38),
            (0x0107_0adc, 0xe3c),
            (0x0107_0ad0, 0xe40),
            (0x0107_0ac4, 0xe44),
            (0x0107_0ab8, 0xe48),
        ],
    },
    ModelSequences {
        model: 0xdc0,
        controller: 0xe00,
        sequences: &[
            (0x0107_0aa8, 0xe4c),
            (0x0107_0a9c, 0xe50),
            (0x0107_0a90, 0xe54),
            (0x0107_0a84, 0xe58),
            (0x0107_0a78, 0xe5c),
            (0x0107_0a6c, 0xe60),
            (0x0107_0a60, 0xe64),
            (0x0107_0a54, 0xe68),
            (0x0107_0a44, 0xe6c),
        ],
    },
];

/// The texturing property of `object` (property 3 when its type word is 8 to 12, else
/// none), asked for as the code asks.
fn texturing_property(e: &mut Engine, object: u32) -> u32 {
    let usable = e.call(GET_PROPERTY, &args![object, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        e.call(PROPERTY_TYPE_00441110, &args![property]).i32() >= 8 && {
            let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
            e.call(PROPERTY_TYPE_00441110, &args![property]).i32() <= 0xc
        }
    };
    if usable {
        e.call(GET_PROPERTY, &args![object, 3u32]).u32()
    } else {
        0
    }
}

// Translated from 00732540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00732540 - the C++ exception-unwinding frame (FS:[0] around the node allocation and the fixed strings) is not translated.
/// `BlackJackMenu::Prepare3DElements` (Xbox PDB, `thiscall`): loads the three hand
/// meshes (+0xdb8, +0xdbc, +0xdc0); the table's `"object1"` becomes the camera
/// (`011d90bc`); a new node (0xac bytes, `00a5ecb0`) the table model (`011d9098`), with
/// the table, the hands and the twelve chip models attached (slot +0xdc with 1); the bet
/// chips (`SetBetChips`), the hand geometry (`00739970(0)`) and the hidden arrows
/// (`00739ad0`); the model updated. For the table and each hand with a controller
/// (`GetController`), keeps it and its named sequences (`0047a520` of a fixed string),
/// deactivates all (0.0) and starts the model's animations. Updates the model again;
/// without a camera makes one (`007c2180`); gives the deck's texturing property (slot
/// +0xf0) the first card's back texture (+0x9c); attaches the model to the shadow scene
/// node and adds its lights within three times its bound's radius.
pub fn black_jack_menu_prepare_3d_elements(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, mesh) in [
        (0xdb8u32, TABLE_MESH_ONE),
        (0xdbc, TABLE_MESH_TWO),
        (0xdc0, TABLE_MESH_THREE),
    ] {
        let model = load_model(e, mesh);
        e.mem.set_u32(base + offset, model);
    }
    let root = e.mem.u32(base + 0xdb4);
    let camera = e
        .call(FIND_OBJECT_004AAE30, &args![root, CAMERA_OBJECT_NAME])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, camera]);
    let block = e.call(ALLOCATE_00AA13E0, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT_00A5ECB0, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![TABLE_MODEL_011D9098, node]);
    let attach = |e: &mut Engine, offset: u32| {
        let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
        let child = e.mem.u32(base + offset);
        e.vcall(model, 0xdc, &args![child, 1u32]);
    };
    for offset in [0xdb4u32, 0xdb8, 0xdbc, 0xdc0] {
        attach(e, offset);
    }
    for i in 0..6 {
        attach(e, 0xdc4 + 4 * i);
        attach(e, 0xddc + 4 * i);
    }
    black_jack_menu_set_bet_chips(e, this);
    fn_00739970(e, this, 0);
    fn_00739ad0(e, this);
    // The frame below `EBP`: the update data at `EBP - 0x2c` and `EBP - 0x1c`, the
    // fixed strings at `EBP - 0x54` downwards.
    e.with_stack(0xc0, |e, frame| {
        let ebp = frame.addr() + 0xc0;
        update_table_model_at(e, ebp - 0x2c);
        let mut string = ebp - 0x54;
        for set in &MODEL_SEQUENCES {
            let model = e.mem.u32(base + set.model);
            let controller = e
                .call(GET_CONTROLLER, &args![model, CONTROLLER_RTTI_011F36AC])
                .u32();
            if controller == 0 {
                string -= 4 * set.sequences.len() as u32;
                continue;
            }
            e.mem.set_u32(base + set.controller, controller);
            for &(name, offset) in set.sequences {
                let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![string, name]).u32();
                let controller = e.mem.u32(base + set.controller);
                let sequence = e.call(SEQUENCE_0047A520, &args![controller, fixed]).u32();
                e.mem.set_u32(base + offset, sequence);
                e.call(FIXED_STRING_DESTRUCT, &args![string]);
                string -= 4;
            }
            let controller = e.mem.u32(base + set.controller);
            e.call(DEACTIVATE_ALL, &args![controller, x87_store32(0.0)]);
            let model = e.mem.u32(base + set.model);
            e.call(START_ANIMATIONS, &args![model]);
        }
        update_table_model_at(e, ebp - 0x1c);
    });
    if e.call(FIRST_WORD, &args![CAMERA_011D90BC]).u32() == 0 {
        let camera = e.call(MAKE_CAMERA_007C2180, &args![this]).u32();
        e.call(NI_POINTER_ASSIGN, &args![CAMERA_011D90BC, camera]);
    }
    let root = e.mem.u32(base + 0xdb4);
    let deck = e
        .call(FIND_OBJECT_004AAE30, &args![root, DECK_OBJECT_NAME])
        .u32();
    let property = texturing_property(e, deck);
    if property != 0 {
        let image = e.call(FIRST_WORD, &args![base + 0x9c]).u32();
        e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
    }
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.vcall(scene, 0xdc, &args![model, 1u32]);
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![model]).u32();
    let radius = e.call(BOUND_RADIUS_0084D030, &args![bound]).f64();
    let three: f64 = e.global(THREE_01021928);
    let reach = x87_quiet_bits(x87_store32(x87_mul(radius, three)));
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    let scene = e.call(SHADOW_SCENE_NODE, &args![1u32]).u32();
    e.call(ADD_ALL_LIGHTS, &args![scene, model, reach]);
}

/// `Tile::GetMenuByClass` (`00a09030`, cdecl), the bet limits the menu is opened with
/// (`0119fe40`, `0119fe3c`; named by their use: the bet is kept at or above the first
/// and at or below the second), and the value trait id static (`011d9048`).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const MINIMUM_BET_0119FE40: u32 = 0x0119_fe40;
const MAXIMUM_BET_0119FE3C: u32 = 0x0119_fe3c;
const VALUE_TRAIT_011D9048: u32 = 0x011d_9048;
/// The player's casino records list (`005a6260`: the player's +0x610), a form's id
/// (`0084e3a0`: the word at +0xc), the allocator (`00401000`, cdecl).
const CASINO_RECORDS_005A6260: u32 = 0x005a_6260;
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
const ALLOCATE_00401000: u32 = 0x0040_1000;
/// Whether the player is barred from the casinos (`00969b80`) and the time left
/// (`00969b40`), both `thiscall` on the player.
const PLAYER_BARRED_00969B80: u32 = 0x0096_9b80;
const PLAYER_BAR_TIME_00969B40: u32 = 0x0096_9b40;
/// The casino's chip form (`007316d0`, `thiscall`) and
/// `Actor::GetItemCountinContainer` (`00575610`, `RET 4`).
const CASINO_CHIP_007316D0: u32 = 0x0073_16d0;
const GET_ITEM_COUNT: u32 = 0x0057_5610;
/// The popup message (`007052f0`, cdecl, six words) with its icon (`01049638`) and
/// `2.0` seconds (`010162c0`), the sounds: `BSAudio::QInstance` (`00453a70`),
/// `BSAudio::GetSoundHandleByName` (`00ad7550`, `RET 0xc`), `BSSoundHandle::Play`
/// (`00ad8830`, `RET 4`) and the handle's destructor (`00483710`);
/// `"UIPopUpMessageGeneral"` (`01025cc4`).
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
const MESSAGE_ICON_01049638: u32 = 0x0104_9638;
const MESSAGE_TIME_010162C0: u32 = 0x0101_62c0;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const POPUP_SOUND: u32 = 0x0102_5cc4;
/// The menu file: the manager's `00586150` and `Tile::ReadFile` (`00a01b00`, `RET 4`)
/// of `"Data\Menus\black_jack_menu.xml"` (`01070c38`), `Tile::GetMenu` (`00a03c90`),
/// `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), the failure message (`005b5e40`, cdecl;
/// `01070be0`), the tile's float trait (`00a011b0`, `RET 4`, `ST0`) compared with
/// 6006.0 (`010704f0`) and 102.0 (`010704e8`), `Tile::GetMaximumDepth` (`00a01530`),
/// `"GAMEBlackJackCardShuffle"` (`01070bc0`).
const MENU_FILE_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_FILE_NAME: u32 = 0x0107_0c38;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const PRINT_FAILURE_005B5E40: u32 = 0x005b_5e40;
const CREATION_FAILED_TEXT: u32 = 0x0107_0be0;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const DEPTH_TRAIT_6006: u32 = 0x0107_04f0;
const DEPTH_TRAIT_102: u32 = 0x0107_04e8;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const SHUFFLE_SOUND: u32 = 0x0107_0bc0;
/// `ActorValueOwner::GetClampedActorValue` (`0066ef20`, `RET 4`; on the player's
/// +0xa4), the named child tile (`00a08b20`, cdecl) `"BJM_TotalEarningsValue"`
/// (`01070ba8`), `Menu::PrepForVisibility` (`00a1dc20`, `RET 4`) and `007316f0`
/// (`thiscall`).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const CHILD_TILE_00A08B20: u32 = 0x00a0_8b20;
const EARNINGS_TILE_NAME: u32 = 0x0107_0ba8;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_007316F0: u32 = 0x0073_16f0;

/// Plays a sound by name through the 12-byte handle local at `handle`
/// (`GetSoundHandleByName(handle, name, 0x121)`, `Play(0)`, the destructor).
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

/// Shows the text at `buffer` as a popup message (the icon, 2.0 seconds) and plays
/// `"UIPopUpMessageGeneral"` through the handle local at `handle`.
fn show_popup(e: &mut Engine, buffer: u32, handle: u32) {
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME_010162C0));
    e.call(
        SHOW_MESSAGE_007052F0,
        &args![buffer, 0u32, MESSAGE_ICON_01049638, 0u32, time, 0u32],
    );
    play_sound(e, handle, POPUP_SOUND);
}

// Translated from 00733630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00733630 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// `BlackJackMenu::Create` (Xbox PDB, cdecl: the casino, the two bet limits and a
/// required earnings figure). Keeps the casino and the limits in their statics; finds
/// the player's record for the casino (+0x610 list, by the casino's form id) or adds a
/// new one (12 bytes, `00733f50`; `00726c60` to a missing or empty list, else
/// `005ae3d0`). While the player is barred, shows `"%s\nTime Remaining: %i"` and
/// returns 0; with the record's earnings (+4) at the casino's limit (+0x28), fewer
/// chips than the minimum bet, or earnings below a non-zero `required`, shows the
/// matching message and returns 0. Otherwise closes an open blackjack menu (slot 0 with
/// 1), reads the menu file and, when it makes a blackjack menu, sets it up: the menu
/// tile, the depth (when trait 0xfa7 is 6006 or 102), the shuffle sound, the luck
/// (+0x68, actor value 0xb) and bet (+0x6c), the label/value tiles (+0x34, +0x38, +0x3c:
/// minimum bet, chips held, the casino's name with its label and the earnings), the
/// earnings tile hidden, the bet label (`0073ab40`) and the message tile (+0x40); with
/// the casino data loaded the bet tiles, the 3D elements and the menu shown; returns the
/// file's tile. A menu of another class (or a failed set-up) is closed when `00726070`
/// answers non-zero, and 0 returned.
pub fn black_jack_menu_create(
    e: &mut Engine,
    casino: u32,
    minimum_bet: u32,
    maximum_bet: u32,
    required: i32,
) -> u32 {
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x510, |e, frame| {
        let ebp = frame.addr() + 0x510;
        let local = |offset: u32| ebp - offset;
        let open = e
            .call(GET_MENU_BY_CLASS, &args![BLACKJACK_MENU_CLASS])
            .u32();
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        e.set_global(CASINO_011D9058, casino);
        e.set_global(MINIMUM_BET_0119FE40, minimum_bet);
        e.set_global(MAXIMUM_BET_0119FE3C, maximum_bet);
        e.set_global(BET_DATA_011D9054, 0u32);
        let player = e.global::<u32>(PLAYER);
        let mut node = e.call(CASINO_RECORDS_005A6260, &args![player]).u32();
        while node != 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let casino = e.global::<u32>(CASINO_011D9058);
            let id = e.call(FORM_ID_0084E3A0, &args![casino]).u32();
            let slot = e.call(NODE_SELF, &args![node]).u32();
            let record = e.mem.u32(slot);
            if id == e.mem.u32(record) {
                let slot = e.call(NODE_SELF, &args![node]).u32();
                let record = e.mem.u32(slot);
                e.set_global(BET_DATA_011D9054, record);
                break;
            }
            if e.call(NODE_NEXT, &args![node]).u32() == 0 {
                break;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        if e.global::<u32>(BET_DATA_011D9054) == 0 {
            let block = e.call(ALLOCATE_00401000, &args![0xcu32]).u32();
            let record = if block != 0 {
                fn_00733f50(e, Ptr::new(block)).addr()
            } else {
                0
            };
            let holder = local(0x20);
            e.mem.set_u32(holder, record);
            let casino = e.global::<u32>(CASINO_011D9058);
            let id = e.call(FORM_ID_0084E3A0, &args![casino]).u32();
            let record = e.mem.u32(holder);
            e.mem.set_u32(record, id);
            let record = e.mem.u32(holder);
            e.mem.set_u32(record.wrapping_add(4), 0);
            let player = e.global::<u32>(PLAYER);
            let list = e.call(CASINO_RECORDS_005A6260, &args![player]).u32();
            if list == 0 || e.call(LIST_IS_EMPTY_008256D0, &args![list]).bool() {
                e.call(LIST_ADD_00726C60, &args![list, holder]);
            } else {
                e.call(LIST_ADD_005AE3D0, &args![list, holder]);
            }
            let record = e.mem.u32(holder);
            e.set_global(BET_DATA_011D9054, record);
        }
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_BARRED_00969B80, &args![player]).bool() {
            let player = e.global::<u32>(PLAYER);
            let left = e.call(PLAYER_BAR_TIME_00969B40, &args![player]).u32();
            let text = e.call(SETTING_STRING, &args![0x011d_451cu32]).u32();
            let buffer = local(0x12c);
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, 0x0107_0c58u32, text, left],
            );
            show_popup(e, buffer, local(0x498));
            return 0;
        }
        let message = |e: &mut Engine, setting: u32, buffer: u32, handle: u32| {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            e.call(FORMAT_TEXT, &args![buffer, 0x104u32, text]);
            show_popup(e, buffer, handle);
        };
        let casino = e.global::<u32>(CASINO_011D9058);
        let data = e.call(CASINO_DATA_00504E60, &args![casino]).u32();
        let record = e.global::<u32>(BET_DATA_011D9054);
        if e.mem.i32(record + 4) >= e.mem.i32(data + 0x28) {
            message(e, 0x011d_3abc, local(0x234), local(0x4a4));
            return 0;
        }
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        let player = e.global::<u32>(PLAYER);
        let chips = e.call(GET_ITEM_COUNT, &args![player, chip]).i32();
        if chips < e.global::<i32>(MINIMUM_BET_0119FE40) {
            message(e, 0x011d_2424, local(0x33c), local(0x4b0));
            return 0;
        }
        let record = e.global::<u32>(BET_DATA_011D9054);
        if e.mem.i32(record + 4) < required && required != 0 {
            message(e, 0x011d_3594, local(0x444), local(0x4bc));
            return 0;
        }
        if open != 0 {
            e.vcall(open, 0, &args![1u32]);
        }
        let file = e.call(MENU_FILE_00586150, &args![manager]).u32();
        let tile = e.call(TILE_READ_FILE, &args![file, MENU_FILE_NAME]).u32();
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        e.set_global(BLACKJACK_MENU, menu);
        let me = |e: &Engine| blackjack_menu(e);
        if me(e) != 0 {
            let class = e.vcall(me(e), 0x34, &args![]).u32();
            if class == fn_00733f80(e) {
                e.call(MENU_SET_MENU_TILE, &args![me(e), tile, 0u32]);
                if !fn_00733f90(e, Ptr::new(me(e))) {
                    e.call(PRINT_FAILURE_005B5E40, &args![CREATION_FAILED_TEXT]);
                    return 0;
                }
                return set_up_menu(e, tile, local(0x4d0), local(0x484));
            }
        }
        if me(e) != 0 && e.call(NODE_NEXT, &args![me(e)]).u32() != 0 && me(e) != 0 {
            e.vcall(me(e), 0, &args![1u32]);
        }
        0
    })
}

/// The rest of `BlackJackMenu::Create` once the menu is made (see there); `handle` is
/// the shuffle sound's local, `name` the 0x40-byte casino name buffer. Returns the
/// file's tile, or 0 when the casino data is missing.
fn set_up_menu(e: &mut Engine, tile: u32, handle: u32, name: u32) -> u32 {
    let me = |e: &Engine| blackjack_menu(e);
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
    play_sound(e, handle, SHUFFLE_SOUND);
    let player = e.global::<u32>(PLAYER);
    let luck = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![player + 0xa4, 0xbu32])
        .u32();
    e.mem.set_u32(me(e) + 0x68, luck);
    let minimum = e.global::<u32>(MINIMUM_BET_0119FE40);
    e.mem.set_u32(me(e) + 0x6c, minimum);
    let label = |e: &mut Engine, offset: u32, setting: u32| {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let which = e.global::<u32>(LABEL_TRAIT_011D9044);
        let tile = e.mem.u32(me(e) + offset);
        e.call(TILE_SET_STRING, &args![tile, which, text, 1u32]);
    };
    let poke = |e: &mut Engine, offset: u32| {
        let which = e.global::<u32>(VALUE_TRAIT_011D9048);
        let tile = e.mem.u32(me(e) + offset);
        e.call(TILE_POKE, &args![tile, which, x87_store32(1.0)]);
    };
    let value = |e: &mut Engine, offset: u32, value: u32| {
        let which = e.global::<u32>(VALUE_TRAIT_011D9048);
        let tile = e.mem.u32(me(e) + offset);
        e.call(TILE_SET_INT, &args![tile, which, value]);
    };
    label(e, 0x34, 0x011d_3804);
    poke(e, 0x34);
    let minimum = e.global::<u32>(MINIMUM_BET_0119FE40);
    value(e, 0x34, minimum);
    label(e, 0x38, 0x011d_3300);
    poke(e, 0x38);
    let casino = e.global::<u32>(CASINO_011D9058);
    let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER);
    let chips = e.call(GET_ITEM_COUNT, &args![player, chip]).u32();
    value(e, 0x38, chips);
    let casino = e.global::<u32>(CASINO_011D9058);
    let full_name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
    if e.call(STRING_LENGTH_0044A670, &args![full_name]).u32() != 0 {
        let text = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
        let casino = e.global::<u32>(CASINO_011D9058);
        let full_name = e.call(NAME_OF_00408DA0, &args![casino + 0x18]).u32();
        e.call(
            FORMAT_TEXT,
            &args![name, 0x40u32, LABEL_FORMAT, full_name, text],
        );
    } else {
        let text = e.call(SETTING_STRING, &args![0x011d_3e08u32]).u32();
        let casino = e.global::<u32>(CASINO_011D9058);
        let full_name = e.vcall(casino, 0x130, &args![]).u32();
        e.call(
            FORMAT_TEXT,
            &args![name, 0x40u32, LABEL_FORMAT, full_name, text],
        );
    }
    let which = e.global::<u32>(LABEL_TRAIT_011D9044);
    let label_tile = e.mem.u32(me(e) + 0x3c);
    e.call(TILE_SET_STRING, &args![label_tile, which, name, 1u32]);
    poke(e, 0x3c);
    let record = e.global::<u32>(BET_DATA_011D9054);
    let earnings = e.mem.u32(record + 4);
    value(e, 0x3c, earnings);
    let label_tile = e.mem.u32(me(e) + 0x3c);
    let earnings_tile = e
        .call(CHILD_TILE_00A08B20, &args![label_tile, EARNINGS_TILE_NAME])
        .u32();
    e.call(TILE_SET_INT, &args![earnings_tile, 0xfa3u32, 0u32]);
    fn_0073ab40(e, Ptr::new(me(e)));
    let message = e.mem.u32(me(e) + 0x40);
    e.call(TILE_POKE, &args![message, 0xfc4u32, x87_store32(1.0)]);
    let text = e.call(SETTING_STRING, &args![0x011d_4c84u32]).u32();
    let message = e.mem.u32(me(e) + 0x40);
    e.call(TILE_SET_STRING, &args![message, 0xfc4u32, text, 1u32]);
    if !black_jack_menu_init_casino_data(e, Ptr::new(me(e))) {
        return 0;
    }
    black_jack_menu_set_bet_tiles(e, 1);
    black_jack_menu_prepare_3d_elements(e, Ptr::new(me(e)));
    e.call(MENU_PREP_FOR_VISIBILITY, &args![me(e), 0u32]);
    let message = e.mem.u32(me(e) + 0x40);
    e.call(TILE_SET_INT, &args![message, 0xfa3u32, 0u32]);
    e.call(MENU_007316F0, &args![me(e)]);
    tile
}

/// `"You've reached the max hand size"` (`01070cc0`), the empty string (`01011584`),
/// `"GAMEBlackJackIncreaseBet"` (`01070ca4`) and `"GAMEBlackJackDecreaseBet"`
/// (`01070c88`).
const MAX_HAND_TEXT: u32 = 0x0107_0cc0;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
const INCREASE_BET_SOUND: u32 = 0x0107_0ca4;
const DECREASE_BET_SOUND: u32 = 0x0107_0c88;

/// Pokes the open menu's tile +0x34 (the value trait, 1.0), then sets its value to
/// what `value` reads.
fn show_bet_value(e: &mut Engine, value: impl Fn(&Engine) -> u32) {
    let which = e.global::<u32>(VALUE_TRAIT_011D9048);
    let tile = e.mem.u32(blackjack_menu(e) + 0x34);
    e.call(TILE_POKE, &args![tile, which, x87_store32(1.0)]);
    let value = value(e);
    let which = e.global::<u32>(VALUE_TRAIT_011D9048);
    let tile = e.mem.u32(blackjack_menu(e) + 0x34);
    e.call(TILE_SET_INT, &args![tile, which, value]);
}

/// The number of cards in a hand (nodes with an item, from `node`).
fn hand_size(e: &mut Engine, mut node: u32) -> i32 {
    let mut count = 0i32;
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        count += 1;
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    count
}

/// A hit on one hand (`hand` +0xd94 with result +0x2c, bust flag 0x400 and double flag
/// 0x40, or the split hand +0xd98 with +0x30, 0x800 and 0x100): draws a card with luck
/// into the hand through the local at `local` (`00905820`); at 21 or more sets the bust
/// flag and above 21 the result 4; clears the double flag; with seven cards or more sets
/// the bust flag and shows the max-hand text on tile +0x58.
fn hit_hand(e: &mut Engine, this: Ptr, hand: u32, result: u32, bust: u32, double: u32, local: u32) {
    let base = this.addr();
    let list = e.mem.u32(base + hand);
    let index = fn_00739090(e, this, list, 1, 0);
    e.mem.set_u32(local, card_at(this, index));
    let list = e.mem.u32(base + hand);
    e.call(LIST_APPEND_00905820, &args![list, local]);
    let list = e.mem.u32(base + hand);
    let value = fn_00738ff0(e, this, list);
    if value >= 0x15 {
        fn_00734d50(e, this, bust, 1);
        if value > 0x15 {
            e.mem.set_u32(base + result, 4);
        }
    }
    fn_00734d50(e, this, double, 0);
    let list = e.mem.u32(base + hand);
    if hand_size(e, list) >= 7 {
        fn_00734d50(e, this, bust, 1);
        let tile = e.mem.u32(base + 0x58);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, MAX_HAND_TEXT, 1u32]);
    }
}

// Translated from 00733ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00733ff0 - the C++ exception-unwinding frame (FS:[0] around the sound handles) is not translated.
/// `BlackJackMenu::DoClick` (Xbox PDB, `thiscall`, `RET 8`: the clicked tile id and a
/// word not read), in state (+0x28) 10 (betting) or 4 (playing); jump table `00734d18`:
/// - 3, playing: hit. State 2, flag 1, the stand tile updated (7), flags 0x100, 0x40
///   and 4 cleared; on the split hand (0x10 without 0x800) a hit on it, else without
///   0x10 or 0x400 a hit on the player's hand; then all tiles updated.
/// - 4, betting: deals (state 1, `007316f0`, luck marks and split bet cleared, flag
///   0x40, tiles 4 to 8 dimmed, `007393f0`). Playing: doubles down unless on the split
///   hand with 0x100, when 0x40 is set (the bet doubled, flags 0x80 and 0x400, a card
///   drawn with luck, `00739650(0)` when allowed, state 2, flag 1, the bet shown with the
///   split bet, a hand above 21 lost (+0x2c 4), 0x40 cleared); all tiles updated.
/// - 5, betting: raises the bet by a step for its range (1, 5, 10, 25, 100, 500), to at
///   most the chips (+0x74) and the maximum bet, with a sound when it grew; the chips and
///   value shown. Playing with flag 4: splits (state 3, flag 1; the second card moved to
///   the split hand, the split bet the bet, one card dealt to each hand, blackjacks
///   marked, doubling checked for both hands, flag 8 set, 4 cleared, the arrow shown).
/// - 6, betting: lowers the bet to the step below for its range, to at least the minimum
///   bet, with a sound when it shrank. Playing with flag 8 (split): switches the hand
///   (0x10) when the target hand is not bust; the arrow and an empty message.
/// - 7, betting: the bet becomes the maximum (at most the chips). Playing without flag
///   1: stands (0x400, result 6, state 8).
/// - 8, betting: closes (`00731d30`). Playing: state 8.
pub fn black_jack_menu_do_click(e: &mut Engine, this: Ptr, id: i32, _unused_2: u32) {
    let base = this.addr();
    let state = |e: &Engine| e.mem.u32(base + 0x28);
    if state(e) != 0xa && state(e) != 4 {
        return;
    }
    let flag = |e: &mut Engine, mask: u32| fn_00734d30(e, this, mask);
    let set = |e: &mut Engine, mask: u32, on: u8| fn_00734d50(e, this, mask, on);
    let bet = |e: &Engine| e.mem.i32(base + 0x6c);
    // The frame below `EBP`: the card holders at `EBP - 0x4c`..`EBP - 0x68` and the
    // sound handles at `EBP - 0x60` and `EBP - 0x74`.
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        match id {
            3 => {
                if state(e) != 4 {
                    return;
                }
                e.mem.set_u32(base + 0x28, 2);
                set(e, 1, 1);
                black_jack_menu_update_tile_alpha(e, this, 7);
                set(e, 0x100, 0);
                set(e, 0x40, 0);
                set(e, 4, 0);
                if flag(e, 0x10) && !flag(e, 0x800) {
                    hit_hand(e, this, 0xd98, 0x30, 0x800, 0x100, ebp - 0x4c);
                } else if !flag(e, 0x10) && !flag(e, 0x400) {
                    hit_hand(e, this, 0xd94, 0x2c, 0x400, 0x40, ebp - 0x50);
                }
                black_jack_menu_update_tile_alpha(e, this, 0xa);
            }
            4 => {
                if state(e) == 0xa {
                    e.mem.set_u32(base + 0x28, 1);
                    e.call(MENU_007316F0, &args![this]);
                    e.mem.set_u8(base + 0x79, 0);
                    e.mem.set_u8(base + 0x78, 0);
                    e.mem.set_u32(base + 0x70, 0);
                    set(e, 0x40, 1);
                    for index in 4..=8 {
                        let dim = f32::from_bits(x87_quiet_bits(e.global::<u32>(DIM_ALPHA)));
                        black_jack_menu_set_tile_text_alpha(e, this, index, dim);
                    }
                    fn_007393f0(e, this);
                } else if state(e) == 4 {
                    let split_doubled = flag(e, 0x10) && flag(e, 0x100);
                    if !split_doubled && flag(e, 0x40) {
                        e.mem.set_u32(base + 0x6c, (bet(e) as u32).wrapping_shl(1));
                        set(e, 0x80, 1);
                        set(e, 0x400, 1);
                        let list = e.mem.u32(base + 0xd94);
                        let index = fn_00739090(e, this, list, 1, 0);
                        let holder = ebp - 0x54;
                        e.mem.set_u32(holder, card_at(this, index));
                        let list = e.mem.u32(base + 0xd94);
                        e.call(LIST_APPEND_00905820, &args![list, holder]);
                        if !flag(e, 0x400) && flag(e, 0x40) {
                            fn_00739650(e, this, 0);
                        }
                        e.mem.set_u32(base + 0x28, 2);
                        set(e, 1, 1);
                        show_bet_value(e, |e| {
                            e.mem.u32(base + 0x70).wrapping_add(e.mem.u32(base + 0x6c))
                        });
                        let list = e.mem.u32(base + 0xd94);
                        if fn_00738ff0(e, this, list) > 0x15 {
                            e.mem.set_u32(base + 0x2c, 4);
                        }
                        set(e, 0x40, 0);
                    }
                    black_jack_menu_update_tile_alpha(e, this, 0xa);
                }
            }
            5 => {
                if state(e) == 0xa {
                    let old = bet(e);
                    let step = if bet(e) < 0xa {
                        1
                    } else if bet(e) < 0x28 {
                        5
                    } else if bet(e) < 0x64 {
                        0xa
                    } else if bet(e) < 0x190 {
                        0x19
                    } else if bet(e) < 0x5dc {
                        100
                    } else {
                        0x1f4
                    };
                    e.mem.set_i32(base + 0x6c, bet(e).wrapping_add(step));
                    if bet(e) > e.mem.i32(base + 0x74) {
                        let chips = e.mem.u32(base + 0x74);
                        e.mem.set_u32(base + 0x6c, chips);
                    }
                    let maximum = e.global::<i32>(MAXIMUM_BET_0119FE3C);
                    if bet(e) > maximum {
                        e.mem.set_i32(base + 0x6c, maximum);
                    }
                    if old < bet(e) {
                        play_sound(e, ebp - 0x60, INCREASE_BET_SOUND);
                    }
                    black_jack_menu_set_bet_chips(e, this);
                    show_bet_value(e, |e| e.mem.u32(base + 0x6c));
                } else if state(e) == 4 && flag(e, 4) {
                    split_hand(e, this, ebp);
                }
            }
            6 => {
                if state(e) == 0xa {
                    let old = bet(e);
                    let round_down = |value: i32, step: i32| {
                        if value % step == 0 {
                            value - step
                        } else {
                            value / step * step
                        }
                    };
                    let lowered = if bet(e) <= 0xa {
                        bet(e).wrapping_sub(1)
                    } else if bet(e) <= 0x28 {
                        round_down(bet(e), 5)
                    } else if bet(e) <= 0x64 {
                        round_down(bet(e), 0xa)
                    } else if bet(e) <= 0x190 {
                        round_down(bet(e), 0x19)
                    } else if bet(e) <= 0x5dc {
                        round_down(bet(e), 100)
                    } else {
                        round_down(bet(e), 0x1f4)
                    };
                    e.mem.set_i32(base + 0x6c, lowered);
                    let minimum = e.global::<i32>(MINIMUM_BET_0119FE40);
                    if bet(e) < minimum {
                        e.mem.set_i32(base + 0x6c, minimum);
                    }
                    if old > bet(e) {
                        play_sound(e, ebp - 0x74, DECREASE_BET_SOUND);
                    }
                    black_jack_menu_set_bet_chips(e, this);
                    show_bet_value(e, |e| e.mem.u32(base + 0x6c));
                } else if state(e) == 4 && flag(e, 8) {
                    if flag(e, 0x10) && !flag(e, 0x400) {
                        set(e, 0x10, 0);
                    } else if !flag(e, 0x10) && !flag(e, 0x800) {
                        set(e, 0x10, 1);
                    }
                    fn_007399f0(e, this);
                    let tile = e.mem.u32(base + 0x58);
                    e.call(
                        TILE_SET_STRING,
                        &args![tile, 0xfc4u32, EMPTY_STRING_01011584, 1u32],
                    );
                    black_jack_menu_update_tile_alpha(e, this, 0xa);
                }
            }
            7 => {
                if state(e) == 0xa {
                    let maximum = e.global::<u32>(MAXIMUM_BET_0119FE3C);
                    e.mem.set_u32(base + 0x6c, maximum);
                    if bet(e) > e.mem.i32(base + 0x74) {
                        let chips = e.mem.u32(base + 0x74);
                        e.mem.set_u32(base + 0x6c, chips);
                    }
                    black_jack_menu_set_bet_chips(e, this);
                    show_bet_value(e, |e| e.mem.u32(base + 0x6c));
                }
                if state(e) == 4 && !flag(e, 1) {
                    set(e, 0x400, 1);
                    e.mem.set_u32(base + 0x2c, 6);
                    e.mem.set_u32(base + 0x28, 8);
                    black_jack_menu_update_tile_alpha(e, this, 0xa);
                }
            }
            8 => {
                if state(e) == 0xa {
                    fn_00731d30(e);
                } else if state(e) == 4 {
                    e.mem.set_u32(base + 0x28, 8);
                    black_jack_menu_update_tile_alpha(e, this, 0xa);
                }
            }
            _ => {}
        }
    });
}

/// The split of `BlackJackMenu::DoClick` (button 5 while playing, see there); `ebp` is
/// its frame's base (the card holder at `EBP - 0x38`, the drawn cards' holders at
/// `EBP - 0x64` and `EBP - 0x68`).
fn split_hand(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    let set = |e: &mut Engine, mask: u32, on: u8| fn_00734d50(e, this, mask, on);
    set(e, 0x100, 0);
    set(e, 0x40, 0);
    e.mem.set_u32(base + 0x28, 3);
    set(e, 1, 1);
    let hand = e.mem.u32(base + 0xd94);
    let second = e.call(NODE_NEXT, &args![hand]).u32();
    let slot = e.call(NODE_SELF, &args![second]).u32();
    let card = ebp - 0x38;
    let moved = e.mem.u32(slot);
    e.mem.set_u32(card, moved);
    let bet = e.mem.u32(base + 0x6c);
    e.mem.set_u32(base + 0x70, bet);
    let split = e.mem.u32(base + 0xd98);
    if split == 0 || e.call(LIST_IS_EMPTY_008256D0, &args![split]).bool() {
        let split = e.mem.u32(base + 0xd98);
        e.call(LIST_ADD_00726C60, &args![split, card]);
    } else {
        let split = e.mem.u32(base + 0xd98);
        e.call(LIST_ADD_005AE3D0, &args![split, card]);
    }
    let moved = e.mem.u32(card);
    black_jack_menu_swap_card_textures(e, this, 1, 1, moved);
    let list = e.mem.u32(base + 0xd94);
    let index = fn_00739090(e, this, list, 0, 0);
    let holder = ebp - 0x64;
    e.mem.set_u32(holder, card_at(this, index));
    e.call(LIST_ADD_00726C60, &args![second, holder]);
    let list = e.mem.u32(base + 0xd98);
    let index = fn_00739090(e, this, list, 0, 0);
    let holder = ebp - 0x68;
    e.mem.set_u32(holder, card_at(this, index));
    let list = e.mem.u32(base + 0xd98);
    e.call(LIST_APPEND_00905820, &args![list, holder]);
    // Both values are of the player's hand (+0xd94), as the code reads them.
    let list = e.mem.u32(base + 0xd94);
    let first_value = fn_00738ff0(e, this, list);
    let list = e.mem.u32(base + 0xd94);
    let second_value = fn_00738ff0(e, this, list);
    if first_value == 0x15 {
        e.mem.set_u32(base + 0x2c, 5);
        set(e, 0x400, 1);
    }
    fn_00739650(e, this, 0);
    if second_value == 0x15 {
        e.mem.set_u32(base + 0x30, 5);
        set(e, 0x800, 1);
    }
    fn_00739650(e, this, 1);
    set(e, 8, 1);
    set(e, 4, 0);
    fn_007399f0(e, this);
    black_jack_menu_update_tile_alpha(e, this, 0xa);
    show_bet_value(e, |e| {
        e.mem.u32(base + 0x70).wrapping_add(e.mem.u32(base + 0x6c))
    });
}

/// `DoIdle`'s statics: the guard bits (`011d90f0`), the object its first call
/// constructs (`011d90e4`, `0041a250`) with its exit destructor (`00fd7de0`, through
/// `_atexit` `00ec658f`), the state time (`011d90e0`, a `float` in seconds), the state
/// it belongs to (`011d90dc`) and the time of the next step (`011d90d8`, a `float`).
const IDLE_GUARD_011D90F0: u32 = 0x011d_90f0;
const IDLE_OBJECT_011D90E4: u32 = 0x011d_90e4;
const IDLE_OBJECT_CONSTRUCT_0041A250: u32 = 0x0041_a250;
const IDLE_OBJECT_DESTRUCT_00FD7DE0: u32 = 0x00fd_7de0;
const ATEXIT: u32 = 0x00ec_658f;
const IDLE_TIME_011D90E0: u32 = 0x011d_90e0;
const IDLE_STATE_011D90DC: u32 = 0x011d_90dc;
const NEXT_STEP_011D90D8: u32 = 0x011d_90d8;
/// `Interface::IsTopMenuID` (`00702450`, cdecl), the list constructor `0096a2d0`
/// (`thiscall`, 8 bytes), the clock (`011f6394`, its milliseconds through `00825c00`:
/// the word at +0x14) and the constants 1000.0 (`01017b70`), 1.0 (`01012070`), 0.0
/// (`01012060`) and 2.0 (`01011590`), `double`s.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const LIST_CONSTRUCT_0096A2D0: u32 = 0x0096_a2d0;
const CLOCK_011F6394: u32 = 0x011f_6394;
const MILLISECONDS_00825C00: u32 = 0x0082_5c00;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const ONE_DOUBLE: u32 = 0x0101_2070;
const TWO_DOUBLE: u32 = 0x0101_1590;
/// A sequence's length (`00508100`: the `float` at +0x30, in `ST0`), the sequence play
/// (`0047aab0`, `RET 0x18`) and stop (`0047b220`, `RET 8`) on a controller manager.
const SEQUENCE_LENGTH_00508100: u32 = 0x0050_8100;
const PLAY_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const STOP_SEQUENCE_0047B220: u32 = 0x0047_b220;
/// `"GAMEBlackJackCardDeal"` (`01070d74`), `"...CardFlip"` (`01070d5c`),
/// `"...CardCollect"` (`01070d40`), `"...CardCollectDealer"` (`01070d20`), `"...Win"`
/// (`01070cf8`) and `"...Lose"` (`01070ce4`).
const DEAL_SOUND: u32 = 0x0107_0d74;
const FLIP_SOUND: u32 = 0x0107_0d5c;
const COLLECT_SOUND: u32 = 0x0107_0d40;
const COLLECT_DEALER_SOUND: u32 = 0x0107_0d20;
const WIN_SOUND: u32 = 0x0107_0cf8;
const LOSE_SOUND: u32 = 0x0107_0ce4;
/// The menu's +0x24 (`0059bb30`; the map's folded name is `D3DTexture_LockRect`), its
/// tile (+4, through `00726070`) and `Menu::StartFadeOut` (`00a1d910`).
const MENU_STATE_0059BB30: u32 = 0x0059_bb30;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// The player's item removal (slot +0x17c, ten words), `BSString`'s constructor
/// (`004037b0`) and destructor (`004037d0`), `TESFullName::GetFullName` (`00482720`,
/// cdecl), the string formatter (`00406f60`, cdecl, variadic),
/// `Actor::GetPickUpSoundName` (`008adcf0`, `RET 0xc`), the two icons (`010208a0`,
/// `0103a830`) and the formats `"%i %s%s %s"` (`0101c178`) and `"%i %s%s %s\n%s"`
/// (`01070d8c`).
const STRING_CONSTRUCT_004037B0: u32 = 0x0040_37b0;
const STRING_DESTRUCT_004037D0: u32 = 0x0040_37d0;
const GET_FULL_NAME: u32 = 0x0048_2720;
const STRING_FORMAT_00406F60: u32 = 0x0040_6f60;
const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
const LOSS_ICON_010208A0: u32 = 0x0102_08a0;
const GAIN_ICON_0103A830: u32 = 0x0103_a830;
const LOSS_FORMAT_0101C178: u32 = 0x0101_c178;
const GAIN_FORMAT_01070D8C: u32 = 0x0107_0d8c;
/// `TESContainer::TESContainer` (`00481610`), its item add (`004818e0`, `RET 0xc`),
/// its transfer to a reference (`004821a0`, `RET 8`) and its destructor (`00481680`);
/// the casino's quest (`00504e40`) and `0060c9c0` (`RET 4`) on it.
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
const CONTAINER_ADD_004818E0: u32 = 0x0048_18e0;
const CONTAINER_GIVE_004821A0: u32 = 0x0048_21a0;
const CONTAINER_DESTRUCT_00481680: u32 = 0x0048_1680;
const CASINO_QUEST_00504E40: u32 = 0x0050_4e40;
const QUEST_0060C9C0: u32 = 0x0060_c9c0;
/// `MiscStatManager::Increment` (`004d5c60`, cdecl), `_ftol2` (`00ec62c0`, `ST0`), 0.51
/// (`01070d18`), 0.5 (`01011588`) and 0.25 (`010290b0`), `double`s; the hand list's
/// clear (`00470470`); `"%s %d %s%s"` (`01070d0c`) and `"%s"` (`01019f08`).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const FTOL2: u32 = 0x00ec_62c0;
const ROUNDING_0_51: u32 = 0x0107_0d18;
const HALF_DOUBLE: u32 = 0x0101_1588;
const QUARTER_DOUBLE: u32 = 0x0102_90b0;
const LIST_CLEAR_00470470: u32 = 0x0047_0470;
const RESULT_FORMAT: u32 = 0x0107_0d0c;
const STRING_FORMAT_TEXT: u32 = 0x0101_9f08;

/// The state time (`FLD dword`).
fn idle_time(e: &Engine) -> f64 {
    x87_load32(e.global::<u32>(IDLE_TIME_011D90E0))
}

/// The state time as an arithmetic operand (`FADD`/`FSUBR dword`).
fn idle_time_operand(e: &Engine) -> f64 {
    x87_m32(e.global::<u32>(IDLE_TIME_011D90E0))
}

/// The length of a sequence (`00508100`).
fn sequence_length(e: &mut Engine, sequence: u32) -> f64 {
    e.call(SEQUENCE_LENGTH_00508100, &args![sequence]).f64()
}

/// Plays the sequence kept at `sequence` on the controller kept at `controller`
/// (offsets in the menu; `0047aab0(sequence, 0, 0, 1.0, 0.0, 0)`).
fn play_sequence(e: &mut Engine, this: Ptr, controller: u32, sequence: u32) {
    let sequence = e.mem.u32(this.addr().wrapping_add(sequence));
    let controller = e.mem.u32(this.addr() + controller);
    e.call(
        PLAY_SEQUENCE_0047AAB0,
        &args![
            controller,
            sequence,
            0u32,
            0u32,
            x87_store32(1.0),
            x87_store32(0.0),
            0u32
        ],
    );
}

/// Stops the sequence kept at `sequence` on the controller kept at `controller`
/// (`0047b220(sequence, 0.0)`).
fn stop_sequence(e: &mut Engine, this: Ptr, controller: u32, sequence: u32) {
    let sequence = e.mem.u32(this.addr().wrapping_add(sequence));
    let controller = e.mem.u32(this.addr() + controller);
    e.call(
        STOP_SEQUENCE_0047B220,
        &args![controller, sequence, x87_store32(0.0)],
    );
}

/// Initialises the update data at `data` with the time `time` (`0043d410(time, 1, 0)`).
fn init_update_data(e: &mut Engine, data: u32, time: u32) {
    e.call(UPDATE_DATA_INIT_0043D410, &args![data, time, 1u32, 0u32]);
}

/// Updates the table model's node `node` (`0043b4a0`) with the data at `data`.
fn update_model_node(e: &mut Engine, data: u32, node: u32) {
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    let object = e.call(NODE_0043B4A0, &args![model, node]).u32();
    e.call(UPDATE_00A59C60, &args![object, data]);
}

/// Updates the table model itself with the data at `data`.
fn update_model(e: &mut Engine, data: u32) {
    let model = e.call(FIRST_WORD, &args![TABLE_MODEL_011D9098]).u32();
    e.call(UPDATE_00A59C60, &args![model, data]);
}

/// Hides the message tile's value (+0x58, trait `0xfa3` 0) once the state time is
/// past 1.0 (`FCOMP`; `TEST AH,0x41`).
fn hide_message_after_a_second(e: &mut Engine, this: Ptr) {
    if idle_time(e) > e.global::<f64>(ONE_DOUBLE) {
        let tile = e.mem.u32(this.addr() + 0x58);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    }
}

/// The address of the word at `offset + 4 * index` in the menu.
fn indexed(offset: u32, index: u32) -> u32 {
    offset.wrapping_add(index.wrapping_mul(4))
}

// Translated from 00734db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00734db0 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// `BlackJackMenu::DoIdle` (Xbox PDB, `thiscall`): the table's per-frame step. The first
/// call constructs the static `011d90e4` (destructor given to `_atexit`). Without a last
/// tick (+0x7c) makes the three hand lists (+0xd94, +0xd98, +0xd90) and clears the state
/// time; otherwise, when the menu is on top, adds the milliseconds since the last tick
/// (none above 1000) to the state time, or clears it when the state changed. Keeps the
/// tick and the state; when not on top lights the interface's menu tile. Then by state
/// (+0x28, jump table `00737dcc`):
/// - 1: deals the opening cards along the sequences (sounds, card textures, the dealer's
///   reveal), then scores blackjacks, sets the doubling and split flags and moves to 4
///   (or 8 on a blackjack).
/// - 2: plays the hit card's sequence for the player's or the split hand, then moves on
///   (3 after a split, 8 when a hand is done, else 4).
/// - 3: the split's set-up and hand switching.
/// - 4: moves to 8 when the hands are done.
/// - 7: collects the cards, then resets the table and the bet tiles and updates the
///   casino record's level (each quarter of the casino's limit earned), closing at a new
///   level (`00731d30`) or going back to betting (10).
/// - 8: reveals the dealer's card; 9: the dealer's draw; 11: the dealer's turn; 12:
///   settles the bets (winnings 1.5 times for blackjack through the ratio +0x60, half
///   back on surrender), shows the result and moves to 7.
/// - 10: reshuffles at the casino's shuffle point, keeps the bet within the chips and
///   closes (`00731d30`) when the chips are below the minimum bet.
/// - 13: once faded (one second), gives or takes the chips the player won or lost and
///   rewards the casino's quest at a new level.
pub fn black_jack_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let guard = e.global::<u32>(IDLE_GUARD_011D90F0);
    if guard & 1 == 0 {
        e.set_global(IDLE_GUARD_011D90F0, guard | 1);
        e.call(IDLE_OBJECT_CONSTRUCT_0041A250, &args![IDLE_OBJECT_011D90E4]);
        e.call(ATEXIT, &args![IDLE_OBJECT_DESTRUCT_00FD7DE0]);
    }
    let class = e.vcall(base, 0x34, &args![]).u32();
    let top = e.call(IS_TOP_MENU_ID, &args![class]).bool();
    if e.mem.u32(base + 0x7c) == 0 {
        for offset in [0xd94u32, 0xd98, 0xd90] {
            let block = e.call(ALLOCATE_00401000, &args![8u32]).u32();
            let list = if block != 0 {
                e.call(LIST_CONSTRUCT_0096A2D0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(base + offset, list);
        }
        e.set_global(IDLE_TIME_011D90E0, x87_store32(0.0));
        e.set_global(IDLE_STATE_011D90DC, 0u32);
    } else if top {
        let now = e.call(MILLISECONDS_00825C00, &args![CLOCK_011F6394]).i32();
        let mut elapsed = now.wrapping_sub(e.mem.i32(base + 0x7c));
        if elapsed > 0x3e8 {
            elapsed = 0;
        }
        if e.global::<u32>(IDLE_STATE_011D90DC) != e.mem.u32(base + 0x28) {
            e.set_global(IDLE_TIME_011D90E0, x87_store32(0.0));
        } else {
            let seconds = x87_div(f64::from(elapsed), e.global::<f64>(THOUSAND_DOUBLE));
            let time = x87_add(seconds, idle_time_operand(e));
            e.set_global(IDLE_TIME_011D90E0, x87_store32(time));
        }
    }
    let now = e.call(MILLISECONDS_00825C00, &args![CLOCK_011F6394]).u32();
    e.mem.set_u32(base + 0x7c, now);
    let state = e.mem.u32(base + 0x28);
    e.set_global(IDLE_STATE_011D90DC, state);
    if !top {
        let alpha = x87_quiet_bits(e.global::<u32>(FULL_ALPHA));
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        let tile = e.call(WORD_AT_0X28, &args![manager]).u32();
        e.call(TILE_SET_FLOAT, &args![tile, 0xfa9u32, alpha, 1u32]);
    }
    // The frame below `EBP`: `local(n)` is `EBP - n` (0x520 bytes).
    e.with_stack(0x520, |e, frame| {
        let ebp = frame.addr() + 0x520;
        match e.mem.u32(base + 0x28) {
            1 => idle_deal(e, this, ebp),
            2 => idle_hit(e, this, ebp),
            3 => idle_split(e, this, ebp),
            4 => idle_play(e, this),
            7 => idle_collect(e, this, ebp),
            8 => idle_reveal(e, this, ebp),
            9 => idle_dealer_draw(e, this, ebp),
            0xa => idle_bet(e, this, ebp),
            0xb => idle_dealer_turn(e, this, ebp),
            0xc => idle_settle(e, this, ebp),
            0xd => idle_close(e, this, ebp),
            _ => {}
        }
    });
}

/// State 1 of `DoIdle`: the deal.
fn idle_deal(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    let dealer = e.mem.u32(base + 0x8c);
    let sequence = e.mem.u32(base.wrapping_add(indexed(0xe0c, dealer)));
    let length = sequence_length(e, sequence);
    let half = x87_store32(x87_div(length, e.global::<f64>(TWO_DOUBLE)));
    let guard = e.global::<u32>(IDLE_GUARD_011D90F0);
    if guard & 2 == 0 {
        e.set_global(IDLE_GUARD_011D90F0, guard | 2);
        let sequence = e.mem.u32(base + 0xe10);
        let length = sequence_length(e, sequence);
        e.set_global(NEXT_STEP_011D90D8, x87_store32(length));
    }
    if e.mem.u32(base.wrapping_add(indexed(0xe4c, dealer))) == 0
        || e.mem.u32(base.wrapping_add(indexed(0xe0c, dealer))) == 0
    {
        return;
    }
    hide_message_after_a_second(e, this);
    let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
    init_update_data(e, ebp - 0x50, time);
    update_model_node(e, ebp - 0x50, 0);
    let card_of = |e: &mut Engine, node: u32| {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        e.mem.u32(slot)
    };
    if idle_time(e) == e.global::<f64>(ZERO_DOUBLE) {
        init_update_data(e, ebp - 0x64, x87_store32(0.0));
        update_model(e, ebp - 0x64);
        play_sound(e, ebp - 0x3e8, DEAL_SOUND);
        e.mem.set_u8(base + 0xda4, 1);
        let mut player = e.mem.u32(base + 0xd94);
        let mut house = e.mem.u32(base + 0xd90);
        let card = card_of(e, player);
        black_jack_menu_swap_card_textures(e, this, 0, 1, card);
        player = e.call(NODE_NEXT, &args![player]).u32();
        let card = card_of(e, player);
        black_jack_menu_swap_card_textures(e, this, 0, 2, card);
        let card = card_of(e, house);
        black_jack_menu_swap_card_textures(e, this, 2, 1, card);
        house = e.call(NODE_NEXT, &args![house]).u32();
        let card = card_of(e, house);
        black_jack_menu_swap_card_textures(e, this, 2, 2, card);
        play_sequence(e, this, 0xdf8, 0xe10);
        play_sequence(e, this, 0xe00, 0xe50);
        let sequence = e.mem.u32(base + 0xe10);
        let length = sequence_length(e, sequence);
        e.set_global(NEXT_STEP_011D90D8, x87_store32(length));
    }
    if e.mem.u32(base + 0x84) == 1 {
        let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
        init_update_data(e, ebp - 0x70, time);
        update_model_node(e, ebp - 0x70, 1);
    }
    if e.mem.u32(base + 0x8c) == 1 && x87_load32(half) <= idle_time(e) {
        if e.mem.u8(base + 0xda4) != 0 {
            let sequence = e.mem.u32(base + 0xe10);
            let length = sequence_length(e, sequence);
            if idle_time(e) < length {
                e.mem.set_u8(base + 0xda4, 0);
                play_sound(e, ebp - 0x3f4, DEAL_SOUND);
            }
        }
        let time = x87_store32(x87_sub(idle_time(e), x87_m32(half)));
        init_update_data(e, ebp - 0x7c, x87_quiet_bits(time));
        update_model_node(e, ebp - 0x7c, 3);
    }
    if e.mem.u32(base + 0x84) == 2 {
        let sequence = e.mem.u32(base + 0xe10);
        let length = sequence_length(e, sequence);
        // FCOMPP; TEST AH,1: on when the time is not below the length (and not NaN).
        if idle_time(e) >= length {
            if e.mem.u8(base + 0xda4) == 0 {
                let sequence = e.mem.u32(base + 0xe10);
                let end = x87_add(sequence_length(e, sequence), x87_m32(half));
                if idle_time(e) < end {
                    e.mem.set_u8(base + 0xda4, 1);
                    play_sound(e, ebp - 0x400, DEAL_SOUND);
                }
            }
            let sequence = e.mem.u32(base.wrapping_add(indexed(0xe0c, dealer)));
            let length = sequence_length(e, sequence);
            let time = x87_store32(x87_sub(idle_time_operand(e), length));
            init_update_data(e, ebp - 0x88, x87_quiet_bits(time));
            update_model_node(e, ebp - 0x88, 1);
        }
    }
    if e.mem.u32(base + 0x8c) == 2 {
        let sequence = e.mem.u32(base + 0xe10);
        let end = x87_add(sequence_length(e, sequence), x87_m32(half));
        if idle_time(e) >= end {
            if e.mem.u8(base + 0xda4) != 0 {
                e.mem.set_u8(base + 0xda4, 0);
                play_sound(e, ebp - 0x40c, FLIP_SOUND);
            }
            let sequence = e.mem.u32(base + 0xe10);
            let end = x87_add(sequence_length(e, sequence), x87_m32(half));
            let time = x87_store32(x87_sub(idle_time_operand(e), end));
            init_update_data(e, ebp - 0x94, x87_quiet_bits(time));
            update_model_node(e, ebp - 0x94, 3);
        }
    }
    // FCOMPP of the next step against the time; TEST AH,0x41: the step is due when it
    // is at most the time.
    let next = x87_load32(e.global::<u32>(NEXT_STEP_011D90D8));
    if !(next <= idle_time(e)) {
        return;
    }
    if e.mem.u32(base + 0x84) == 1 {
        let count = e.mem.u32(base + 0x84).wrapping_add(1);
        e.mem.set_u32(base + 0x84, count);
        stop_sequence(e, this, 0xdf8, indexed(0xe0c, dealer));
        let count = e.mem.u32(base + 0x84);
        play_sequence(e, this, 0xdf8, indexed(0xe0c, count));
        let sequence = e.mem.u32(base + 0xe50);
        let next = x87_add(sequence_length(e, sequence), x87_m32(half));
        e.set_global(NEXT_STEP_011D90D8, x87_store32(next));
    } else if e.mem.u32(base + 0x8c) == 1 {
        let count = e.mem.u32(base + 0x8c).wrapping_add(1);
        e.mem.set_u32(base + 0x8c, count);
        stop_sequence(e, this, 0xe00, indexed(0xe4c, dealer));
        let count = e.mem.u32(base + 0x8c);
        play_sequence(e, this, 0xe00, indexed(0xe4c, count));
        let sequence = e.mem.u32(base + 0xe10);
        let first = sequence_length(e, sequence);
        let sequence = e.mem.u32(base + 0xe50);
        let next = x87_add(sequence_length(e, sequence), first);
        e.set_global(NEXT_STEP_011D90D8, x87_store32(next));
    } else if e.mem.u32(base + 0x84) == 2 {
        let count = e.mem.u32(base + 0x84).wrapping_add(1);
        e.mem.set_u32(base + 0x84, count);
        stop_sequence(e, this, 0xdf8, indexed(0xe0c, dealer));
        let sequence = e.mem.u32(base + 0xe50);
        let first = x87_add(sequence_length(e, sequence), x87_m32(half));
        let sequence = e.mem.u32(base + 0xe54);
        let next = x87_add(sequence_length(e, sequence), first);
        e.set_global(NEXT_STEP_011D90D8, x87_store32(next));
    } else {
        let list = e.mem.u32(base + 0xd94);
        let player = fn_00738ff0(e, this, list);
        if player == 0x15 {
            e.mem.set_u32(base + 0x2c, 5);
            fn_00734d50(e, this, 0x20, 1);
        }
        let list = e.mem.u32(base + 0xd90);
        if fn_00738ff0(e, this, list) == 0x15 {
            fn_00734d50(e, this, 0x20, 1);
            let result = if e.mem.u32(base + 0x2c) == 5 { 2 } else { 1 };
            e.mem.set_u32(base + 0x2c, result);
        }
        if !fn_00734d30(e, this, 0x20) {
            black_jack_menu_set_bet_tiles(e, 0);
        }
        let doubled = e.mem.i32(base + 0x6c).wrapping_shl(1);
        if e.mem.i32(base + 0x74) < doubled {
            fn_00734d50(e, this, 0x40, 0);
            fn_00734d50(e, this, 0x100, 0);
            fn_00734d50(e, this, 4, 0);
        } else if player == 0xb {
            fn_00734d50(e, this, 0x40, 1);
        } else {
            fn_007395a0(e, this);
        }
        black_jack_menu_update_tile_alpha(e, this, 0xa);
        e.set_global(NEXT_STEP_011D90D8, x87_store32(0.0));
        let count = e.mem.u32(base + 0x8c).wrapping_add(1);
        e.mem.set_u32(base + 0x8c, count);
        stop_sequence(e, this, 0xe00, indexed(0xe4c, dealer));
        let state = if fn_00734d30(e, this, 0x20) { 8 } else { 4 };
        e.mem.set_u32(base + 0x28, state);
    }
}

/// State 2 of `DoIdle`: the hit card's sequence on the player's or (flag 0x10) the
/// split hand.
fn idle_hit(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    let split = fn_00734d30(e, this, 0x10);
    // The hand's controller, its sequences, its card count and its list.
    let (controller, sequences, count, list) = if split {
        (0xdfcu32, 0xe2cu32, 0x88u32, 0xd98u32)
    } else {
        (0xdf8, 0xe0c, 0x84, 0xd94)
    };
    let sequence_at = |e: &Engine| indexed(sequences, e.mem.u32(base + count));
    if e.mem.u32(base.wrapping_add(sequence_at(e))) == 0 {
        return;
    }
    hide_message_after_a_second(e, this);
    let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
    init_update_data(e, ebp - 0xb0, time);
    update_model_node(e, ebp - 0xb0, 0);
    if idle_time(e) == e.global::<f64>(ZERO_DOUBLE) {
        play_sound(e, ebp - 0x418, DEAL_SOUND);
        let hand = u32::from(split);
        let at = sequence_at(e);
        play_sequence(e, this, controller, at);
        let mut node = e.mem.u32(base + list);
        let mut card = 0u32;
        let mut place = 0u32;
        while node != 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            if e.mem.u32(slot) == 0 || place == e.mem.u32(base + count) {
                break;
            }
            let slot = e.call(NODE_SELF, &args![node]).u32();
            card = e.mem.u32(slot);
            node = e.call(NODE_NEXT, &args![node]).u32();
            place = place.wrapping_add(1);
        }
        while node != 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            if e.mem.u32(slot) == 0 || place == 0 {
                break;
            }
            let slot = e.call(NODE_SELF, &args![node]).u32();
            card = e.mem.u32(slot);
            node = e.call(NODE_NEXT, &args![node]).u32();
            place = place.wrapping_add(1);
        }
        black_jack_menu_swap_card_textures(e, this, hand as i32, place as i32, card);
    }
    let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
    init_update_data(e, ebp - 0xa4, time);
    update_model_node(e, ebp - 0xa4, if split { 2 } else { 1 });
    let sequence = e.mem.u32(base.wrapping_add(sequence_at(e)));
    let length = sequence_length(e, sequence);
    // FCOMPP; TEST AH,1: done when the time is not below the length (and not NaN).
    if !(idle_time(e) >= length) {
        return;
    }
    let at = sequence_at(e);
    stop_sequence(e, this, controller, at);
    let placed = e.mem.u32(base + count).wrapping_add(1);
    e.mem.set_u32(base + count, placed);
    let flag = |e: &mut Engine, mask: u32| fn_00734d30(e, this, mask);
    let state = if flag(e, 0x4000) {
        3
    } else if flag(e, 0x1000)
        || (flag(e, 8) && flag(e, 0x400) && flag(e, 0x800))
        || (!flag(e, 8) && flag(e, 0x400))
    {
        // The exe tests these in this order (two branches to the same store).
        8
    } else {
        4
    };
    e.mem.set_u32(base + 0x28, state);
}

/// State 3 of `DoIdle`: after a split (needs the sequences +0xe30, +0xe34, +0xe14).
fn idle_split(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0xe30) == 0 || e.mem.u32(base + 0xe34) == 0 || e.mem.u32(base + 0xe14) == 0
    {
        return;
    }
    let set = |e: &mut Engine, mask: u32, on: u8| fn_00734d50(e, this, mask, on);
    if e.mem.u32(base + 0x88) == 1 {
        set(e, 0x4000, 1);
        fn_00739970(e, this, 1);
        play_sequence(e, this, 0xdf8, 0xe14);
        init_update_data(e, ebp - 0xdc, x87_store32(0.0));
        init_update_data(e, ebp - 0xd0, x87_store32(0.0));
        update_model_node(e, ebp - 0xd0, 3);
        update_model_node(e, ebp - 0xdc, 1);
        set(e, 0x10, 1);
        let count = e.mem.u32(base + 0x84).wrapping_sub(1);
        e.mem.set_u32(base + 0x84, count);
        e.mem.set_u32(base + 0x28, 2);
    } else if e.mem.u32(base + 0x84) == 2 {
        set(e, 0x10, 0);
        e.mem.set_u32(base + 0x28, 2);
    } else if e.mem.u32(base + 0x88) == 2 {
        set(e, 0x10, 1);
        e.mem.set_u32(base + 0x28, 2);
    } else {
        set(e, 0x4000, 0);
        set(e, 0x10, 0);
        let both = e.mem.u32(base + 0x2c) == 5 && e.mem.u32(base + 0x30) == 5;
        e.mem.set_u32(base + 0x28, if both { 0xc } else { 4 });
    }
}

/// State 4 of `DoIdle`: to state 8 once a result is set (both on a split) or the hand
/// (both on a split, 0x400 and 0x800) is done.
fn idle_play(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let flag = |e: &mut Engine, mask: u32| fn_00734d30(e, this, mask);
    if e.mem.u32(base + 0x2c) != 0 && (!flag(e, 8) || e.mem.u32(base + 0x30) != 0) {
        e.mem.set_u32(base + 0x28, 8);
    }
    if flag(e, 0x400) && (!flag(e, 8) || flag(e, 0x800)) {
        e.mem.set_u32(base + 0x28, 8);
    }
}

/// State 7 of `DoIdle`: collects the cards (without the sequences +0xe0c, +0xe2c and
/// +0xe4c, straight back to betting).
fn idle_collect(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0xe0c) == 0 || e.mem.u32(base + 0xe2c) == 0 || e.mem.u32(base + 0xe4c) == 0
    {
        e.mem.set_u32(base + 0x28, 0xa);
        return;
    }
    hide_message_after_a_second(e, this);
    let collecting = |e: &Engine| e.mem.u8(base + 0xda4) != 0;
    if idle_time(e) == e.global::<f64>(ZERO_DOUBLE) && !collecting(e) {
        play_sound(e, ebp - 0x424, COLLECT_SOUND);
        init_update_data(e, ebp - 0xe8, x87_store32(0.0));
        update_model(e, ebp - 0xe8);
        play_sequence(e, this, 0xdf4, 0xe04);
        play_sequence(e, this, 0xdf8, 0xe0c);
        play_sequence(e, this, 0xdfc, 0xe2c);
        fn_00737e50(e, this);
    }
    // The time less the collection start (+0xda8), as a `double` local.
    let since_start = |e: &Engine| {
        let start = x87_m32(e.mem.u32(base + 0xda8));
        x87_sub(idle_time(e), start)
    };
    let mut collected = false;
    if !collecting(e) {
        let sequence = e.mem.u32(base + 0xe0c);
        let length = sequence_length(e, sequence);
        if idle_time(e) < length {
            let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
            init_update_data(e, ebp - 0x100, time);
            init_update_data(e, ebp - 0xf4, time);
            init_update_data(e, ebp - 0x10c, time);
            update_model_node(e, ebp - 0x100, 0);
            update_model_node(e, ebp - 0xf4, 1);
            update_model_node(e, ebp - 0x10c, 2);
            collected = true;
        }
    }
    if !collected && collecting(e) {
        let since = since_start(e);
        let sequence = e.mem.u32(base + 0xe4c);
        // FCOMP of the length against the local; TEST AH,0x41: on when it is above.
        if sequence_length(e, sequence) > since {
            let time = x87_quiet_bits(x87_store32(since_start(e)));
            init_update_data(e, ebp - 0x124, time);
            let time = x87_quiet_bits(x87_store32(since_start(e)));
            init_update_data(e, ebp - 0x118, time);
            update_model_node(e, ebp - 0x124, 0);
            update_model_node(e, ebp - 0x118, 3);
        }
    }
    let sequence = e.mem.u32(base + 0xe0c);
    let length = sequence_length(e, sequence);
    if idle_time(e) >= length && !collecting(e) {
        stop_sequence(e, this, 0xdf4, 0xe04);
        play_sequence(e, this, 0xdf4, 0xe08);
        play_sequence(e, this, 0xe00, 0xe4c);
        play_sound(e, ebp - 0x430, COLLECT_DEALER_SOUND);
        e.mem.set_u8(base + 0xda4, 1);
        let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
        e.mem.set_u32(base + 0xda8, time);
        init_update_data(e, ebp - 0x13c, x87_store32(0.0));
        init_update_data(e, ebp - 0x130, x87_store32(0.0));
        update_model_node(e, ebp - 0x13c, 0);
        update_model_node(e, ebp - 0x130, 3);
        return;
    }
    let since = since_start(e);
    let sequence = e.mem.u32(base + 0xe4c);
    // FCOMP; TEST AH,0x41; JP: done when the length is at most the local.
    if !(sequence_length(e, sequence) <= since) || !collecting(e) {
        return;
    }
    stop_sequence(e, this, 0xdf4, 0xe08);
    stop_sequence(e, this, 0xe00, 0xe4c);
    for controller in [0xdf4u32, 0xe00, 0xdf8, 0xdfc] {
        let controller = e.mem.u32(base + controller);
        e.call(DEACTIVATE_ALL, &args![controller, x87_store32(0.0)]);
    }
    fn_00738270(e, this);
    e.call(MENU_007316F0, &args![this]);
    fn_00739970(e, this, 0);
    e.mem.set_u8(base + 0xda4, 0);
    black_jack_menu_set_bet_tiles(e, 1);
    let casino = e.global::<u32>(CASINO_011D9058);
    let data = e.call(CASINO_DATA_00504E60, &args![casino]).u32();
    let limit = e.mem.i32(data + 0x28);
    let record = e.global::<u32>(BET_DATA_011D9054);
    let earnings = e.mem.i32(record + 4);
    let record = e.global::<u32>(BET_DATA_011D9054);
    let level = i32::from(e.mem.u16(record + 8) as i16);
    let quarter: f64 = e.global(QUARTER_DOUBLE);
    for step in 1..5i32 {
        if step <= level {
            e.mem.set_u8(base + 0x80, 0);
        } else {
            let share = x87_mul(f64::from(limit), x87_mul(f64::from(step), quarter));
            // FCOMPP of the earnings against the share; TEST AH,1: on when not below.
            if f64::from(earnings) >= share {
                let record = e.global::<u32>(BET_DATA_011D9054);
                e.mem.set_u16(record + 8, step as u16);
                e.mem.set_u8(base + 0x80, 1);
            }
        }
    }
    if e.mem.u8(base + 0x80) == 0 {
        e.mem.set_u32(base + 0x28, 0xa);
    } else {
        fn_00731d30(e);
    }
    for i in 0..3 {
        e.mem.set_u32(base + 0x84 + 4 * i, 1);
    }
}

/// State 8 of `DoIdle`: the dealer's reveal (+0xe6c on +0xe00); without it, to 11.
fn idle_reveal(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    hide_message_after_a_second(e, this);
    if e.mem.u32(base + 0xe6c) == 0 {
        e.mem.set_u32(base + 0x28, 0xb);
        return;
    }
    let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
    init_update_data(e, ebp - 0x15c, time);
    update_model_node(e, ebp - 0x15c, 0);
    if idle_time(e) == e.global::<f64>(ZERO_DOUBLE) {
        fn_00734d50(e, this, 0x400, 1);
        fn_00734d50(e, this, 0x800, 1);
        fn_00739ad0(e, this);
        play_sound(e, ebp - 0x43c, FLIP_SOUND);
        play_sequence(e, this, 0xe00, 0xe6c);
    }
    let sequence = e.mem.u32(base + 0xe6c);
    if idle_time(e) < sequence_length(e, sequence) {
        let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
        init_update_data(e, ebp - 0x168, time);
        update_model_node(e, ebp - 0x168, 3);
    }
    let sequence = e.mem.u32(base + 0xe6c);
    if idle_time(e) >= sequence_length(e, sequence) {
        stop_sequence(e, this, 0xe00, 0xe6c);
        e.mem.set_u32(base + 0x28, 0xb);
    }
}

/// The dealer's next state: 11 (draw again) while the hand is at most 21 and flag
/// 0x2000 is clear, else 12.
fn dealer_next_state(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.mem.u32(this.addr() + 0xd90);
    if fn_00738ff0(e, this, list) <= 0x15 && !fn_00734d30(e, this, 0x2000) {
        0xb
    } else {
        0xc
    }
}

/// State 9 of `DoIdle`: the dealer's drawn card (sequence +0xe4c by the dealer's count
/// +0x8c, on +0xe00).
fn idle_dealer_draw(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    hide_message_after_a_second(e, this);
    let sequence_at = |e: &Engine| indexed(0xe4c, e.mem.u32(base + 0x8c));
    if e.mem.u32(base.wrapping_add(sequence_at(e))) == 0 {
        let state = dealer_next_state(e, this);
        e.mem.set_u32(base + 0x28, state);
        return;
    }
    let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
    init_update_data(e, ebp - 0x174, time);
    update_model_node(e, ebp - 0x174, 0);
    if idle_time(e) == e.global::<f64>(ZERO_DOUBLE) {
        play_sound(e, ebp - 0x448, FLIP_SOUND);
        fn_00739ad0(e, this);
        let mut node = e.mem.u32(base + 0xd90);
        let mut card = 0u32;
        let mut place = 0i32;
        while node != 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(NODE_SELF, &args![node]).u32();
            card = e.mem.u32(slot);
            node = e.call(NODE_NEXT, &args![node]).u32();
            place = place.wrapping_add(1);
        }
        if card != 0 {
            black_jack_menu_swap_card_textures(e, this, 2, place, card);
        }
        let at = sequence_at(e);
        play_sequence(e, this, 0xe00, at);
    }
    let sequence = e.mem.u32(base.wrapping_add(sequence_at(e)));
    if idle_time(e) < sequence_length(e, sequence) {
        let time = x87_quiet_bits(e.global::<u32>(IDLE_TIME_011D90E0));
        init_update_data(e, ebp - 0x18c, time);
        update_model_node(e, ebp - 0x18c, 3);
    }
    let sequence = e.mem.u32(base.wrapping_add(sequence_at(e)));
    if idle_time(e) >= sequence_length(e, sequence) {
        let at = sequence_at(e);
        stop_sequence(e, this, 0xe00, at);
        let count = e.mem.u32(base + 0x8c).wrapping_add(1);
        e.mem.set_u32(base + 0x8c, count);
        let state = dealer_next_state(e, this);
        e.mem.set_u32(base + 0x28, state);
    }
}

/// State 10 of `DoIdle`: betting.
fn idle_bet(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    let remaining = f64::from(e.mem.i32(base + 0xda0));
    let cards = f64::from(e.mem.i32(base + 0xd9c).wrapping_mul(0x34));
    let point = x87_mul(cards, x87_m32(e.mem.u32(base + 0x64)));
    // FCOMPP of the shuffle point against the cards left; TEST AH,0x41: on above.
    if point > remaining {
        fn_007399d0(e, this);
    }
    hide_message_after_a_second(e, this);
    if e.mem.i32(base + 0x74) < e.mem.i32(base + 0x6c) {
        let chips = e.mem.u32(base + 0x74);
        e.mem.set_u32(base + 0x6c, chips);
        show_bet_value(e, |e| e.mem.u32(base + 0x6c));
    }
    if e.mem.i32(base + 0x74) < e.global::<i32>(MINIMUM_BET_0119FE40) {
        let text = e.call(SETTING_STRING, &args![0x011d_2424u32]).u32();
        let buffer = ebp - 0x3c4;
        e.call(FORMAT_TEXT, &args![buffer, 0x104u32, text]);
        show_popup(e, buffer, ebp - 0x470);
        fn_00731d30(e);
    }
}

/// State 11 of `DoIdle`: the dealer's turn: flag 0x1000; on surrendered or bust hands
/// (results 4 or 6; both on a split) flag 0x2000 and state 12; with a blackjack (0x20)
/// 12; otherwise the dealer draws (state 9) below 17 or on a soft 17 (an ace, +0xdac
/// clear) with fewer than seven cards, else 12.
fn idle_dealer_turn(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    fn_00734d50(e, this, 0x1000, 1);
    let lost = if fn_00734d30(e, this, 8) {
        e.mem.u32(base + 0x2c) == 4 && e.mem.u32(base + 0x30) == 4
    } else {
        e.mem.u32(base + 0x2c) == 4 || e.mem.u32(base + 0x2c) == 6
    };
    if lost {
        fn_00734d50(e, this, 0x2000, 1);
        e.mem.set_u32(base + 0x28, 0xc);
        return;
    }
    if fn_00734d30(e, this, 0x20) {
        e.mem.set_u32(base + 0x28, 0xc);
        return;
    }
    let list = e.mem.u32(base + 0xd90);
    let value = fn_00738ff0(e, this, list);
    let mut cards = 0i32;
    let mut ace = false;
    let mut node = e.mem.u32(base + 0xd90);
    while node != 0 {
        let slot = e.call(NODE_SELF, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        cards += 1;
        let slot = e.call(NODE_SELF, &args![node]).u32();
        let card = e.mem.u32(slot);
        if e.mem.u32(card) == 0xc {
            ace = true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let draws = value < 0x11 || (value == 0x11 && ace && e.mem.u8(base + 0xdac) == 0);
    if draws && cards < 7 {
        let list = e.mem.u32(base + 0xd90);
        let index = fn_00739090(e, this, list, 1, 1);
        let holder = ebp - 0x44c;
        e.mem.set_u32(holder, card_at(this, index));
        let list = e.mem.u32(base + 0xd90);
        e.call(LIST_APPEND_00905820, &args![list, holder]);
        e.mem.set_u32(base + 0x28, 9);
    } else {
        e.mem.set_u32(base + 0x28, 0xc);
    }
}

/// A hand's result against the dealer's value: 3 (won) when above the dealer's and at
/// most 21 or when the dealer is bust and the hand is not; else 1 (lost) when below the
/// dealer's at most 21 or bust; else 2 (push).
fn hand_result(hand: i32, dealer: i32) -> u32 {
    if (hand > dealer && hand < 0x16) || (hand < 0x16 && dealer > 0x15) {
        3
    } else if (hand < dealer && dealer < 0x16) || hand > 0x15 {
        1
    } else {
        2
    }
}

/// `_ftol2(bet * ratio + rounding)` (`FILD`, `FMUL dword`, `FADD double`).
fn scaled_winnings(e: &mut Engine, bet: i32, ratio: u32, rounding: u32) -> i32 {
    let value = x87_add(
        x87_mul(f64::from(bet), x87_m32(ratio)),
        e.global::<f64>(rounding),
    );
    e.call(FTOL2, &args![value]).i32()
}

/// State 12 of `DoIdle`: settles the bets.
fn idle_settle(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    e.call(MISC_STAT_INCREMENT, &args![0x29u32]);
    let list = e.mem.u32(base + 0xd94);
    let mut hand = fn_00738ff0(e, this, list);
    let list = e.mem.u32(base + 0xd90);
    let dealer = fn_00738ff0(e, this, list);
    let mut net = 0i32;
    if e.mem.u32(base + 0x2c) == 0 {
        e.mem.set_u32(base + 0x2c, hand_result(hand, dealer));
    }
    if fn_00734d30(e, this, 8) {
        let list = e.mem.u32(base + 0xd98);
        hand = fn_00738ff0(e, this, list);
        if e.mem.u32(base + 0x30) == 0 {
            e.mem.set_u32(base + 0x30, hand_result(hand, dealer));
        }
    }
    // Jump tables `00737e00` and `00737e18`.
    match e.mem.u32(base + 0x2c) {
        1 | 4 => net = net.wrapping_sub(e.mem.i32(base + 0x6c)),
        3 => net = net.wrapping_add(e.mem.i32(base + 0x6c)),
        5 => {
            let bet = e.mem.i32(base + 0x6c);
            let ratio = e.mem.u32(base + 0x60);
            net = scaled_winnings(e, bet, ratio, ROUNDING_0_51).wrapping_add(net);
        }
        6 => {
            let bet = f64::from(e.mem.i32(base + 0x6c));
            let value = x87_add(
                x87_div(bet, e.global::<f64>(TWO_DOUBLE)),
                e.global::<f64>(ROUNDING_0_51),
            );
            let back = e.call(FTOL2, &args![value]).i32();
            net = back.wrapping_neg().wrapping_add(net);
        }
        _ => {}
    }
    match e.mem.u32(base + 0x30) {
        1 | 4 => net = net.wrapping_sub(e.mem.i32(base + 0x70)),
        3 => net = net.wrapping_add(e.mem.i32(base + 0x70)),
        5 => {
            let bet = e.mem.i32(base + 0x70);
            let ratio = e.mem.u32(base + 0x60);
            net = scaled_winnings(e, bet, ratio, HALF_DOUBLE).wrapping_add(net);
        }
        _ => {}
    }
    let chips = e.mem.i32(base + 0x74).wrapping_add(net);
    e.mem.set_i32(base + 0x74, chips);
    let which = e.global::<u32>(VALUE_TRAIT_011D9048);
    let tile = e.mem.u32(blackjack_menu(e) + 0x38);
    e.call(TILE_POKE, &args![tile, which, x87_store32(1.0)]);
    let chips = e.mem.u32(base + 0x74);
    let which = e.global::<u32>(VALUE_TRAIT_011D9048);
    let tile = e.mem.u32(blackjack_menu(e) + 0x38);
    e.call(TILE_SET_INT, &args![tile, which, chips]);
    let record = e.global::<u32>(BET_DATA_011D9054);
    let earnings = e.mem.i32(record + 4).wrapping_add(net);
    let record = e.global::<u32>(BET_DATA_011D9054);
    e.mem.set_i32(record + 4, earnings);
    fn_0073ab40(e, this);
    let buffer = ebp - 0x2b4;
    let result = |e: &mut Engine, setting: u32, amount: i32| {
        let coins = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
        let chips = e.call(SETTING_STRING, &args![0x011d_5050u32]).u32();
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        e.call(
            FORMAT_TEXT,
            &args![buffer, 0x104u32, RESULT_FORMAT, text, amount, chips, coins],
        );
    };
    if net > 0 {
        let setting = if e.mem.u8(base + 0x78) != 0 {
            0x011d_3a98
        } else {
            0x011d_2ff4
        };
        result(e, setting, net);
        play_sound(e, ebp - 0x458, WIN_SOUND);
    } else if net < 0 {
        let setting = if e.mem.u8(base + 0x79) != 0 {
            0x011d_25e0
        } else {
            0x011d_3474
        };
        result(e, setting, net.wrapping_neg());
        play_sound(e, ebp - 0x464, LOSE_SOUND);
    } else {
        let text = e.call(SETTING_STRING, &args![0x011d_3f88u32]).u32();
        e.call(
            FORMAT_TEXT,
            &args![buffer, 0x104u32, STRING_FORMAT_TEXT, text],
        );
    }
    let tile = e.mem.u32(blackjack_menu(e) + 0x58);
    e.call(TILE_POKE, &args![tile, 0xfc4u32, x87_store32(1.0)]);
    let tile = e.mem.u32(blackjack_menu(e) + 0x58);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
    let tile = e.mem.u32(blackjack_menu(e) + 0x58);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    for offset in [0xd94u32, 0xd90] {
        let list = e.mem.u32(base + offset);
        e.call(LIST_CLEAR_00470470, &args![list]);
    }
    if fn_00734d30(e, this, 8) {
        let list = e.mem.u32(base + 0xd98);
        e.call(LIST_CLEAR_00470470, &args![list]);
        show_bet_value(e, |e| e.mem.u32(base + 0x6c));
        e.mem.set_u32(base + 0x70, 0);
    }
    if fn_00734d30(e, this, 0x80) {
        let halved = e.mem.i32(base + 0x6c) / 2;
        e.mem.set_i32(base + 0x6c, halved);
        show_bet_value(e, |e| e.mem.u32(base + 0x6c));
    }
    if e.mem.i32(base + 0x74) < e.mem.i32(base + 0x6c) {
        let chips = e.mem.u32(base + 0x74);
        e.mem.set_u32(base + 0x6c, chips);
        show_bet_value(e, |e| e.mem.u32(base + 0x6c));
    }
    e.mem.set_u32(base + 0x28, 7);
}

/// State 13 of `DoIdle`: closing.
fn idle_close(e: &mut Engine, this: Ptr, ebp: u32) {
    let base = this.addr();
    // FCOMP with 1.0; TEST AH,5; JP: on unless the time is below 1.0 (NaN goes on).
    if idle_time(e) < e.global::<f64>(ONE_DOUBLE) {
        return;
    }
    if e.call(MENU_STATE_0059BB30, &args![this]).u32() == 2 {
        return;
    }
    let alpha = x87_quiet_bits(e.global::<u32>(FULL_ALPHA));
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let tile = e.call(WORD_AT_0X28, &args![manager]).u32();
    e.call(TILE_SET_FLOAT, &args![tile, 0xfa9u32, alpha, 1u32]);
    let tile = e.call(NODE_NEXT, &args![this]).u32();
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![this]);
    let casino = e.global::<u32>(CASINO_011D9058);
    let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER);
    let held = e.call(GET_ITEM_COUNT, &args![player, chip]).i32();
    let change = e.mem.i32(base + 0x74).wrapping_sub(held);
    let chip_name = |e: &mut Engine| {
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        e.call(GET_FULL_NAME, &args![chip]).u32()
    };
    let pick_up_sound = |e: &mut Engine| {
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        let player = e.global::<u32>(PLAYER);
        e.call(GET_PICK_UP_SOUND_NAME, &args![player, chip, 1u32, 0u32])
            .u32()
    };
    if change < 0 {
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        let player = e.global::<u32>(PLAYER);
        let lost = change.wrapping_neg();
        e.vcall(
            player,
            0x17c,
            &args![chip, 0u32, lost, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        let text = ebp - 0x24;
        e.call(STRING_CONSTRUCT_004037B0, &args![text]);
        let first = e.call(SETTING_STRING, &args![0x011d_3db4u32]).u32();
        let second = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
        let name = chip_name(e);
        e.call(
            STRING_FORMAT_00406F60,
            &args![text, LOSS_FORMAT_0101C178, lost, name, second, first],
        );
        let seconds = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME_010162C0));
        let sound = pick_up_sound(e);
        let message = e.call(FIRST_WORD, &args![text]).u32();
        e.call(
            SHOW_MESSAGE_007052F0,
            &args![message, 0u32, LOSS_ICON_010208A0, sound, seconds, 0u32],
        );
        e.call(STRING_DESTRUCT_004037D0, &args![text]);
    } else if change > 0 {
        let container = ebp - 0x30;
        e.call(CONTAINER_CONSTRUCT, &args![container]);
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        e.call(
            CONTAINER_ADD_004818E0,
            &args![container, chip, change, 0u32],
        );
        let casino = e.global::<u32>(CASINO_011D9058);
        let cap = fn_00737e30(e, Ptr::new(casino)) as i32;
        let record = e.global::<u32>(BET_DATA_011D9054);
        let player = e.global::<u32>(PLAYER);
        if e.mem.i32(record + 4) < cap {
            e.call(CONTAINER_GIVE_004821A0, &args![container, player, 0u32]);
        } else {
            e.call(CONTAINER_GIVE_004821A0, &args![container, player, 1u32]);
            let text = ebp - 0x38;
            e.call(STRING_CONSTRUCT_004037B0, &args![text]);
            let first = e.call(SETTING_STRING, &args![0x011d_3abcu32]).u32();
            let second = e.call(SETTING_STRING, &args![0x011d_4e7cu32]).u32();
            let third = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let name = chip_name(e);
            e.call(
                STRING_FORMAT_00406F60,
                &args![
                    text,
                    GAIN_FORMAT_01070D8C,
                    change,
                    name,
                    third,
                    second,
                    first
                ],
            );
            let seconds = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME_010162C0));
            let sound = pick_up_sound(e);
            let message = e.call(FIRST_WORD, &args![text]).u32();
            e.call(
                SHOW_MESSAGE_007052F0,
                &args![message, 0u32, GAIN_ICON_0103A830, sound, seconds, 0u32],
            );
            e.call(STRING_DESTRUCT_004037D0, &args![text]);
        }
        e.call(CONTAINER_DESTRUCT_00481680, &args![container]);
    }
    if e.mem.u8(base + 0x80) != 0 {
        let casino = e.global::<u32>(CASINO_011D9058);
        let quest = e.call(CASINO_QUEST_00504E40, &args![casino]).u32();
        if quest != 0 {
            e.call(QUEST_0060C9C0, &args![quest, 1u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_1d70, black_jack_menu_init_casino_data(Ptr) -> bool),
        entry!(0x0073_8560, black_jack_menu_update_tile_alpha(Ptr, i32)),
        entry!(0x0073_7e50, fn_00737e50(Ptr)),
        entry!(0x0073_8270, fn_00738270(Ptr)),
        entry!(0x0073_8cc0, black_jack_menu_set_bet_tiles(u8)),
        entry!(0x0073_93f0, fn_007393f0(Ptr) -> i32),
        entry!(0x0073_9090, fn_00739090(Ptr, u32, u8, u8) -> u32),
        entry!(
            0x0073_96e0,
            black_jack_menu_swap_card_textures(Ptr, i32, i32, u32)
        ),
        entry!(0x0073_1800, fn_00731800(Ptr)),
        entry!(0x0073_3500, fn_00733500()),
        entry!(0x0073_3fd0, fn_00733fd0(Ptr, f32)),
        entry!(0x0073_1710, fn_00731710(Ptr) -> Ptr),
        entry!(0x0073_1790, black_jack_menu_get_class(Ptr) -> u32),
        entry!(
            0x0073_17a0,
            black_jack_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(
            0x0073_17d0,
            black_jack_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0073_1d30, fn_00731d30()),
        entry!(0x0073_24f0, fn_007324f0(Ptr, u32) -> Ptr),
        entry!(0x0073_3f50, fn_00733f50(Ptr) -> Ptr),
        entry!(0x0073_3f80, fn_00733f80() -> u32),
        entry!(0x0073_3f90, fn_00733f90(Ptr) -> bool),
        entry!(0x0073_4d30, fn_00734d30(Ptr, u32) -> bool),
        entry!(0x0073_4d50, fn_00734d50(Ptr, u32, u8)),
        entry!(0x0073_7e30, fn_00737e30(Ptr) -> u32),
        entry!(
            0x0073_8c30,
            black_jack_menu_set_tile_text_alpha(Ptr, i32, f32)
        ),
        entry!(0x0073_8fb0, fn_00738fb0(Ptr, i32) -> i32),
        entry!(0x0073_8ff0, fn_00738ff0(Ptr, u32) -> i32),
        entry!(0x0073_95a0, fn_007395a0(Ptr) -> bool),
        entry!(0x0073_9650, fn_00739650(Ptr, u8) -> bool),
        entry!(0x0073_9970, fn_00739970(Ptr, u8)),
        entry!(0x0073_99d0, fn_007399d0(Ptr)),
        entry!(0x0073_99f0, fn_007399f0(Ptr)),
        entry!(0x0073_9ad0, fn_00739ad0(Ptr)),
        entry!(0x0073_ab40, fn_0073ab40(Ptr)),
        entry!(0x0073_9b30, black_jack_menu_set_bet_chips(Ptr)),
        entry!(0x0073_2540, black_jack_menu_prepare_3d_elements(Ptr)),
        entry!(
            0x0073_3630,
            black_jack_menu_create(u32, u32, u32, i32) -> u32
        ),
        entry!(0x0073_3ff0, black_jack_menu_do_click(Ptr, i32, u32)),
        entry!(0x0073_4db0, black_jack_menu_do_idle(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_float_quieted() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.call(0x0073_3fd0, &args![this, 1.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xf8), 1.5);
        e.call(0x0073_3fd0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xf8), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        FREE,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_COPY_006E5CC0,
        INTERFACE_MANAGER,
        WORD_AT_0X28,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        CHILD_LIST_ITEM_0073AC90,
        CHILD_LIST_AT_006317A0,
        CHILD_LIST_FOLLOWING_007B52D0,
        SET_FLOATS_004BC1F0,
        FIND_OBJECT_004AAE30,
        SET_APP_CULLED_00450F90,
        UPDATE_DATA_INIT_0043D410,
        NODE_0043B4A0,
        UPDATE_00A59C60,
        SETTING_STRING,
        NAME_OF_00408DA0,
        FORMAT_TEXT,
        STRING_LENGTH_0044A670,
        MENU_DESTRUCT,
        VECTOR_DESTRUCTOR_ITERATOR,
        CARD_DESTRUCTOR_00914AE0,
        PLAYER_00969B20,
        NI_POINTER_ASSIGN,
        NI_POINTER_DESTRUCT,
        CASINO_ENTRY_00504E00,
        MODEL_RELEASE_0045A5E0,
        SHADOW_SCENE_NODE,
        SCENE_00B5D180,
        MODEL_00572160,
        LIST_DELETE_004702F0,
        COPY_TEXT,
        RENDERER_0043C4B0,
        RENDERER_007148C0,
        GETTER_0045C670,
        CULLER_008D80E0,
        CULLER_00C4F270,
        CAMERA_00709D50,
        TEXTURE_007FD640,
        CULLER_SET_ACCUMULATOR,
        GETTER_006629F0,
        ACCUMULATE_SCENE,
        RENDER_00B6C0D0,
        CULLER_00C4F2D0,
        TILE_POKE,
        TILE_SET_INT,
        RANDOM_INDEX_00944460,
        ABS_00EC7D40,
        LIST_IS_EMPTY_008256D0,
        LIST_ADD_00726C60,
        LIST_ADD_005AE3D0,
        LIST_APPEND_00905820,
        GET_PROPERTY,
        PROPERTY_TYPE_00441110,
        CASINO_DATA_00504E60,
        CASINO_DECK_00504E20,
        MODEL_LOAD_FILE,
        CREATE_TEXTURE_IMAGE,
        SWAP_LANGUAGE_TEXTURE_PATH,
        FIND_CHARACTERS_004812F0,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
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

    /// An engine with the statics and constants mapped, the callees doubled (list nodes
    /// are `[item, next]`), and the test vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_9000,
            0x011d_e000,
            0x011c_3000,
            0x0102_3000,
            0x0102_c000,
            0x0103_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FULL_ALPHA, 255.0f32);
        e.set_global(MINUS_FIVE_0102CAF8, -5.0f32);
        e.set_global(CAMERA_VALUE_01030FF4, 0.15f32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    fn menu(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0xe80);
        e.mem.set_u32(this, VTABLE);
        e.set_global(BLACKJACK_MENU, this);
        Ptr::new(this)
    }

    /// A hand of cards of the given ranks: the node list and the cards.
    fn hand(e: &mut Engine, ranks: &[u32]) -> u32 {
        let mut next = 0;
        for &rank in ranks.iter().rev() {
            let card = e.mem.alloc(0x10);
            e.mem.set_u32(card, rank);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, card);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn cards_and_the_small_accessors() {
        let mut e = engine();
        let card = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0073_1710, &args![card]).u32(), card);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [0xe, 5]);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other, 3);
        e.mem.set_u32(other + 4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_24f0, &args![card, other]).u32(), card);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [3, 1]);
        assert_eq!(
            calls_to(&e, NI_POINTER_COPY_006E5CC0),
            vec![vec![card + 8, other + 8], vec![card + 0xc, other + 0xc]]
        );
        e.mem.set_u16(card + 8, 7);
        e.call(0x0073_3f50, &args![card]);
        assert_eq!([e.mem.u32(card), e.mem.u32(card + 4)], [0, 0]);
        assert_eq!(e.mem.u16(card + 8), 0);
        let this = menu(&mut e);
        assert_eq!(e.call(0x0073_1790, &args![this]).u32(), 0x439);
        assert_eq!(e.call(0x0073_3f80, &args![]).u32(), 0x439);
        e.mem.set_u32(this.addr() + 0x210, 0x99);
        assert_eq!(e.call(0x0073_7e30, &args![this]).u32(), 0x99);
        e.mem.set_u32(this.addr() + 0xd9c, 2);
        e.call(0x0073_99d0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 0x68);
    }

    #[test]
    fn tiles_attach_and_all_are_needed() {
        let mut e = engine();
        let this = menu(&mut e);
        for id in 0..9 {
            e.call(0x0073_17a0, &args![this, id, 0x5000u32 + id as u32]);
        }
        e.call(0x0073_17a0, &args![this, 0xai32, 0x6000u32]);
        e.call(0x0073_17a0, &args![this, -1i32, 0x6000u32]);
        assert!(!e.call(0x0073_3f90, &args![this]).bool());
        e.call(0x0073_17a0, &args![this, 9i32, 0x5009u32]);
        assert!(e.call(0x0073_3f90, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0x34 + 4 * 9), 0x5009);
    }

    #[test]
    fn flags_set_clear_and_test() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call(0x0073_4d50, &args![this, 0x14u32, 1u32]);
        assert!(e.call(0x0073_4d30, &args![this, 0x10u32]).bool());
        e.call(0x0073_4d50, &args![this, 0x10u32, 0u32]);
        assert!(!e.call(0x0073_4d30, &args![this, 0x10u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0), 4);
    }

    #[test]
    fn the_deleting_destructor_and_the_body() {
        let mut e = engine();
        let this = menu(&mut e);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, VTABLE);
        answer(&mut e, CASINO_ENTRY_00504E00, entry);
        e.register(slot(0x14), |_, _| 0x7700u32.into_ret());
        // The path copy writes "a.nif" into the buffer.
        e.register(COPY_TEXT, |e, a| {
            if a[1] == 0x104 {
                e.mem.write(a[0], b"a.nif\0");
            }
            Ret::default()
        });
        e.register(STRING_LENGTH_0044A670, |_, _| 5u32.into_ret());
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, SHADOW_SCENE_NODE, scene);
        e.mem.set_u32(this.addr() + 0xd94, 0x8800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_17d0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BLACKJACK_MENU_VTABLE);
        assert_eq!(e.global::<u32>(BLACKJACK_MENU), 0);
        // Entry 8's model, three meshes, six models and six shadow models.
        assert_eq!(calls_to(&e, MODEL_RELEASE_0045A5E0).len(), 16);
        let shadows: Vec<_> = calls_to(&e, COPY_TEXT)
            .into_iter()
            .filter(|c| c[1] == 0xf)
            .collect();
        assert_eq!(shadows.len(), 6);
        assert_eq!(shadows[0][2], SHADOW_SUFFIX);
        assert!(calls_to(&e, LIST_DELETE_004702F0).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0xd94), 0);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![
                this.addr() + 0x90,
                0x10,
                0xd0,
                CARD_DESTRUCTOR_00914AE0
            ]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn closing_sets_the_state_and_the_alpha() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0073_1d30, &args![]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        let this = menu(&mut e);
        answer(&mut e, WORD_AT_0X28, 0x5100);
        e.call_log = Some(vec![]);
        e.call(0x0073_1d30, &args![]);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0xd);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5100, 0xfa9, 255.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn the_table_renders_through_the_culler() {
        let mut e = engine();
        answer(&mut e, CULLER_008D80E0, 0x6100);
        answer(&mut e, TEXTURE_007FD640, 0x6200);
        answer(&mut e, GETTER_006629F0, 0x6300);
        answer(&mut e, SHADOW_SCENE_NODE, 0x6400);
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_3500, &args![]);
        assert_eq!(calls_to(&e, RENDERER_007148C0)[0][1..], [0, 4]);
        assert_eq!(calls_to(&e, CULLER_00C4F270), vec![vec![0x6100, 1]]);
        assert_eq!(
            calls_to(&e, CULLER_SET_ACCUMULATOR),
            vec![vec![0x6100, 0x6200], vec![0x6100, 0]]
        );
        assert_eq!(
            calls_to(&e, ACCUMULATE_SCENE),
            vec![vec![0x6300, 0x6400, 0x6100]]
        );
        assert_eq!(calls_to(&e, RENDER_00B6C0D0), vec![vec![0x6300, 0x6200, 0]]);
        assert_eq!(calls_to(&e, CULLER_00C4F2D0), vec![vec![0x6100]]);
    }

    #[test]
    fn text_alpha_walks_the_tiles_children() {
        let mut e = engine();
        let this = menu(&mut e);
        let tile = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + 0x34 + 8, tile);
        e.mem.set_u32(tile + 4, 0x7000);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 1);
        answer(&mut e, CHILD_LIST_ITEM_0073AC90, item);
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, 0x7100);
        answer(&mut e, CHILD_LIST_AT_006317A0, child);
        e.register(CHILD_LIST_FOLLOWING_007B52D0, |_, a| {
            u32::from(a[1] == 0x7000).wrapping_mul(0x7001).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_8c30, &args![this, 2i32, 0.5f32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x7100, 0xfa9, 0.5f32.to_bits(), 1],
                vec![0x7100, 0xfa9, 0.5f32.to_bits(), 1]
            ]
        );
        assert_eq!(calls_to(&e, CHILD_LIST_AT_006317A0)[1][1], 0x7001);
    }

    #[test]
    fn card_points_and_hand_values() {
        let mut e = engine();
        let this = menu(&mut e);
        let points: Vec<i32> = [0i32, 7, 9, 11, 12, 13]
            .iter()
            .map(|&r| e.call(0x0073_8fb0, &args![this, r]).i32())
            .collect();
        assert_eq!(points, vec![2, 9, 10, 10, 11, 15]);
        let two_aces_and_nine = hand(&mut e, &[12, 12, 7]);
        assert_eq!(
            e.call(0x0073_8ff0, &args![this, two_aces_and_nine]).i32(),
            21
        );
        let king_queen = hand(&mut e, &[11, 10]);
        assert_eq!(e.call(0x0073_8ff0, &args![this, king_queen]).i32(), 20);
    }

    #[test]
    fn split_and_double_need_the_chips() {
        let mut e = engine();
        let this = menu(&mut e);
        let pair = hand(&mut e, &[5, 5]);
        e.mem.set_u32(this.addr() + 0xd94, pair);
        e.mem.set_i32(this.addr() + 0x6c, 10);
        e.mem.set_i32(this.addr() + 0x74, 20);
        assert!(e.call(0x0073_95a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 4, 4);
        let three = hand(&mut e, &[5, 5, 2]);
        e.mem.set_u32(this.addr() + 0xd94, three);
        assert!(!e.call(0x0073_95a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 4, 0);
        e.mem.set_i32(this.addr() + 0x70, 4);
        // Split: 20 < 10 + 2 * 4 is false; not split: 20 < 4 + 2 * 10.
        assert!(e.call(0x0073_9650, &args![this, 1u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 0x200, 0x200);
        assert!(!e.call(0x0073_9650, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + 0xdb0) & 0x40, 0);
    }

    #[test]
    fn hand_geometry_and_arrows() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xdbc, 0x6600);
        e.mem.set_u32(this.addr() + 0xdb4, 0x6700);
        e.call_log = Some(vec![]);
        e.call(0x0073_9970, &args![this, 0u32]);
        e.call(0x0073_9970, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, SET_FLOATS_004BC1F0),
            vec![
                vec![0x6600, 0, 0, (-5.0f32).to_bits()],
                vec![0x6600, 0, 0, 0]
            ]
        );
        e.register(FIND_OBJECT_004AAE30, |_, a| (a[1] & 0xfff).into_ret());
        e.mem.set_u32(this.addr() + 0xdb0, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0073_99f0, &args![this]);
        assert_eq!(
            calls_to(&e, SET_APP_CULLED_00450F90),
            vec![vec![0xe58, 1], vec![0xe44, 0]]
        );
        assert_eq!(calls_to(&e, NODE_0043B4A0)[0][1], 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_9ad0, &args![this]);
        assert_eq!(
            calls_to(&e, SET_APP_CULLED_00450F90),
            vec![vec![0xe58, 1], vec![0xe44, 1]]
        );
    }

    #[test]
    fn the_bet_label_is_padded() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0x3c, 0x5300);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data + 4, 25);
        e.set_global(BET_DATA_011D9054, data);
        e.set_global(LABEL_TRAIT_011D9044, 0x45u32);
        e.register(FORMAT_TEXT, |e, a| {
            if a[2] == LABEL_FORMAT {
                e.mem.write(a[0], b"Bet\0");
            }
            Ret::default()
        });
        answer(&mut e, STRING_LENGTH_0044A670, 20);
        e.call_log = Some(vec![]);
        e.call(0x0073_ab40, &args![this]);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats.len(), 3);
        assert_eq!(formats[0][5], 25);
        let buffer = formats[0][0];
        // Three steps from length 20: spaces at 19, 20, 21, the end at 22.
        assert_eq!(e.mem.u8(buffer + 21), 0x20);
        assert_eq!(e.mem.u8(buffer + 22), 0);
        assert_eq!(formats[2][2..], [PADDED_VALUE_FORMAT, buffer, 25]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5300, 0x45, buffer, 1]]
        );
        answer(&mut e, STRING_LENGTH_0044A670, 30);
        e.call_log = Some(vec![]);
        e.call(0x0073_ab40, &args![this]);
        assert_eq!(calls_to(&e, FORMAT_TEXT).len(), 1);
    }

    #[test]
    fn bet_tiles_take_their_labels() {
        let mut e = engine();
        let this = menu(&mut e);
        let mut tiles = vec![];
        for i in 0..6 {
            let tile = e.mem.alloc(8);
            tiles.push(tile);
            e.mem.set_u32(this.addr() + 0x40 + 4 * i, tile);
        }
        e.register(SETTING_STRING, |_, a| (a[0] & 0xffff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0073_8cc0, &args![0u32]);
        assert_eq!(calls_to(&e, TILE_POKE).len(), 5);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING)[0],
            vec![tiles[1], 0xfc4, 0x4c78, 1]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![tiles[0], 0xfa3, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_8cc0, &args![1u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING)[4],
            vec![tiles[5], 0xfc4, 0x33fc, 1]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![tiles[0], 0xfa3, 0]]);
        assert_eq!(calls_to(&e, CHILD_LIST_ITEM_0073AC90).len(), 0);
    }

    #[test]
    fn drawing_a_card_swaps_it_to_the_end() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xda0, 10);
        e.register(RANDOM_INDEX_00944460, |_, a| (a[1] / 2).into_ret());
        for i in 0..11u32 {
            e.mem.set_u32(this.addr() + 0x90 + 0x10 * i, i);
        }
        e.register(NI_POINTER_COPY_006E5CC0, |_, _| Ret::default());
        // No luck: a random index below the count (lowered first).
        assert_eq!(e.call(0x0073_9090, &args![this, 0u32, 0u32, 0u32]).u32(), 9);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 8);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 9), 5);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 5), 9);
        // With good luck (skill 7) two candidates; the one that makes 21 wins.
        e.mem.set_i32(this.addr() + 0x68, 7);
        e.register(ABS_00EC7D40, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        let low = hand(&mut e, &[9]);
        e.register_double(RANDOM_INDEX_00944460, {
            let mut n = 0;
            move |_, _| {
                n += 1;
                (if n == 1 { 1u32 } else { 7 }).into_ret()
            }
        });
        e.mem.set_u32(this.addr() + 0x90 + 0x10, 3);
        e.mem.set_u32(this.addr() + 0x90 + 0x10 * 7, 9);
        assert_eq!(e.call(0x0073_9090, &args![this, low, 1u32, 0u32]).u32(), 8);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 8), 9);
        // Neither the best nor the lower candidate is close enough to mark luck.
        assert_eq!(e.mem.u8(this.addr() + 0x78), 0);
    }

    #[test]
    fn the_deal_gives_two_cards_each() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u32(this.addr() + 0xda0, 20);
        let dealer = e.mem.alloc(8);
        let player = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0xd90, dealer);
        e.mem.set_u32(this.addr() + 0xd94, player);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_93f0, &args![this]).i32(), 0);
        assert_eq!(calls_to(&e, LIST_ADD_005AE3D0).len(), 2);
        assert!(calls_to(&e, LIST_ADD_005AE3D0)
            .iter()
            .all(|c| c[0] == dealer));
        let appended = calls_to(&e, LIST_APPEND_00905820);
        assert_eq!(appended.len(), 2);
        assert_eq!([appended[0][0], appended[1][0]], [dealer, player]);
        // The plain draws lower the count twice, the dealer's lucky one (no candidates)
        // once.
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 13);
        answer(&mut e, LIST_IS_EMPTY_008256D0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_93f0, &args![this]);
        assert_eq!(
            calls_to(&e, LIST_ADD_00726C60)
                .iter()
                .map(|c| c[0])
                .collect::<Vec<_>>(),
            vec![dealer, player]
        );
    }

    #[test]
    fn card_textures_go_to_the_named_meshes() {
        let mut e = engine();
        let this = menu(&mut e);
        let property = e.mem.alloc(4);
        e.mem.set_u32(property, VTABLE);
        answer(&mut e, GET_PROPERTY, property);
        answer(&mut e, PROPERTY_TYPE_00441110, 9);
        let card = e.mem.alloc(0x10);
        e.mem.set_u32(card + 8, 0x7300);
        e.mem.set_u32(card + 0xc, 0x7400);
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 2i32, 3i32, card]);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats[0][2..], [DEALER_FRONT, 3]);
        assert_eq!(formats[1][2..], [DEALER_BACK, 3]);
        assert_eq!(calls_to(&e, NODE_0043B4A0)[0][1], 3);
        assert_eq!(
            calls_to(&e, slot(0xf0)),
            vec![vec![property, 0, 0, 0x7300], vec![property, 0, 0, 0x7400]]
        );
        answer(&mut e, PROPERTY_TYPE_00441110, 13);
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 0i32, 3i32, card]);
        assert!(calls_to(&e, slot(0xf0)).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0073_96e0, &args![this, 0i32, 8i32, card]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn casino_data_builds_the_decks() {
        let mut e = engine();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(DEFAULT_WIN_RATIO, 1.5f32);
        e.set_global(DEFAULT_SHUFFLE, 0.25f32);
        let this = menu(&mut e);
        assert!(!e.call(0x0073_1d70, &args![this]).bool());
        let casino = e.mem.alloc(0x40);
        e.set_global(CASINO_011D9058, casino);
        let data = e.mem.alloc(0x40);
        e.mem.set_f32(data + 4, 2.0);
        e.mem.set_u32(data + 0x24, 0);
        e.mem.set_u8(data + 0x34, 1);
        answer(&mut e, CASINO_DATA_00504E60, data);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, VTABLE);
        answer(&mut e, CASINO_ENTRY_00504E00, entry);
        answer(&mut e, MODEL_LOAD_FILE, 0x6600);
        // The deck's path: "a\b\folder\card_x".
        e.register(COPY_TEXT, |e, a| {
            if a[1] == 0x104 {
                e.mem.write(a[0], b"a\\b\\folder\\card_x\0");
            }
            Ret::default()
        });
        e.register(FIND_CHARACTERS_004812F0, |e, a| {
            let mut p = a[0];
            while e.mem.u8(p) != b'\\' {
                p += 1;
            }
            p.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_1d70, &args![this]).bool());
        assert_eq!(e.mem.f32(this.addr() + 0x60), 2.0);
        assert_eq!(e.mem.f32(this.addr() + 0x64), 0.25);
        assert_eq!(e.mem.u32(this.addr() + 0xd9c), 1);
        assert_eq!(e.mem.u8(this.addr() + 0xdac), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xdb4), 0x6600);
        assert_eq!(e.mem.u32(this.addr() + 0xda0), 0x34);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(formats.len(), 2 + 0x34);
        let folder = formats[0][4];
        assert_eq!(e.mem.u8(folder), b'f');
        assert_eq!(e.mem.u8(folder + 6), b'\\');
        let name = formats[1][4];
        assert_eq!(e.mem.u8(name), b'c');
        assert_eq!(e.mem.u8(name + 4), 0);
        // The last face: spades 13.
        assert_eq!(
            formats[0x35][5..],
            [u32::from(b's'), u32::from(b'1'), u32::from(b'3')]
        );
        // Card 0 is the ace (rank 12) of suit 0.
        assert_eq!(e.mem.u32(this.addr() + 0x90), 12);
        assert_eq!(e.mem.u32(this.addr() + 0x90 + 0x10 * 0x33 + 4), 3);
        assert_eq!(calls_to(&e, CREATE_TEXTURE_IMAGE).len(), 0x35);
    }

    #[test]
    fn tile_alpha_follows_the_flags() {
        let mut e = engine();
        e.map(0x0101_e000, 0x1000);
        e.set_global(DIM_ALPHA, 128.0f32);
        let this = menu(&mut e);
        // Each tile's child list holds one child, 0x100 plus the tile index.
        for i in 0..10u32 {
            let tile = e.mem.alloc(8);
            e.mem.set_u32(tile + 4, 0x100 + i);
            e.mem.set_u32(this.addr() + 0x34 + 4 * i, tile);
        }
        for i in 0..10u32 {
            let tile = e.mem.u32(this.addr() + 0x34 + 4 * i);
            let list = e.mem.alloc(4);
            e.mem.set_u32(list, 0x100 + i);
            e.mem.set_u32(tile + 4, list);
        }
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 1);
        answer(&mut e, CHILD_LIST_ITEM_0073AC90, item);
        e.register(CHILD_LIST_AT_006317A0, |e, a| {
            let slot = e.mem.alloc(4);
            let child = e.mem.u32(a[1]);
            e.mem.set_u32(slot, child);
            slot.into_ret()
        });
        let alphas = |e: &Engine| -> Vec<(u32, f32)> {
            calls_to(e, TILE_SET_FLOAT)
                .iter()
                .map(|c| (c[0], f32::from_bits(c[2])))
                .collect()
        };
        e.mem.set_u32(this.addr() + 0xdb0, 0x400);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 3i32]);
        e.call(0x0073_8560, &args![this, 7i32]);
        e.call(0x0073_8560, &args![this, 9i32]);
        assert_eq!(alphas(&e), vec![(0x103, 128.0), (0x107, 255.0)]);
        e.mem.set_u32(this.addr() + 0xdb0, 0x10 | 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 0xai32]);
        assert_eq!(
            alphas(&e),
            vec![
                (0x103, 255.0),
                (0x104, 255.0),
                (0x105, 128.0),
                (0x107, 128.0),
                (0x108, 255.0),
                (0x106, 255.0)
            ]
        );
        e.mem.set_u32(this.addr() + 0x28, 0xa);
        e.call_log = Some(vec![]);
        e.call(0x0073_8560, &args![this, 3i32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
    }

    #[test]
    fn card_meshes_show_up_to_the_counts() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_i32(this.addr() + 0x84, 3);
        e.mem.set_i32(this.addr() + 0x8c, 0);
        e.mem.set_i32(this.addr() + 0x88, 8);
        e.call_log = Some(vec![]);
        e.call(0x0073_7e50, &args![this]);
        let culled: Vec<u32> = calls_to(&e, SET_APP_CULLED_00450F90)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(culled.len(), 63);
        assert_eq!(culled[..9], [0, 0, 0, 0, 0, 0, 1, 1, 1]);
        assert!(culled[21..42].iter().all(|&c| c == 1));
        assert!(culled[42..].iter().all(|&c| c == 0));
        assert_eq!(calls_to(&e, UPDATE_00A59C60).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0073_8270, &args![this]);
        assert!(calls_to(&e, SET_APP_CULLED_00450F90)
            .iter()
            .all(|c| c[1] == 0));
        assert_eq!(calls_to(&e, FORMAT_TEXT)[0][2..], [HAND_ONE_FRONT, 1, 0]);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    /// Every callee outside the file the n059 functions reach, doubled with no effect.
    const STUBBED: &[u32] = &[
        FREE,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_COPY_006E5CC0,
        INTERFACE_MANAGER,
        WORD_AT_0X28,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        CHILD_LIST_ITEM_0073AC90,
        CHILD_LIST_AT_006317A0,
        CHILD_LIST_FOLLOWING_007B52D0,
        SET_FLOATS_004BC1F0,
        FIND_OBJECT_004AAE30,
        SET_APP_CULLED_00450F90,
        UPDATE_DATA_INIT_0043D410,
        NODE_0043B4A0,
        UPDATE_00A59C60,
        SETTING_STRING,
        NAME_OF_00408DA0,
        FORMAT_TEXT,
        STRING_LENGTH_0044A670,
        CARD_DESTRUCTOR_00914AE0,
        NI_POINTER_DESTRUCT,
        CASINO_ENTRY_00504E00,
        MODEL_LOAD_FILE,
        SHADOW_SCENE_NODE,
        COPY_TEXT,
        TILE_POKE,
        TILE_SET_INT,
        RANDOM_INDEX_00944460,
        ABS_00EC7D40,
        LIST_IS_EMPTY_008256D0,
        LIST_ADD_00726C60,
        LIST_ADD_005AE3D0,
        LIST_APPEND_00905820,
        GET_PROPERTY,
        PROPERTY_TYPE_00441110,
        CASINO_DATA_00504E60,
        ALLOCATE_00AA13E0,
        NODE_CONSTRUCT_00A5ECB0,
        GET_CONTROLLER,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTRUCT,
        SEQUENCE_0047A520,
        DEACTIVATE_ALL,
        START_ANIMATIONS,
        MAKE_CAMERA_007C2180,
        GET_WORLD_BOUND,
        BOUND_RADIUS_0084D030,
        ADD_ALL_LIGHTS,
        GET_MENU_BY_CLASS,
        CASINO_RECORDS_005A6260,
        FORM_ID_0084E3A0,
        ALLOCATE_00401000,
        PLAYER_BARRED_00969B80,
        PLAYER_BAR_TIME_00969B40,
        CASINO_CHIP_007316D0,
        GET_ITEM_COUNT,
        SHOW_MESSAGE_007052F0,
        AUDIO_INSTANCE,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_PLAY,
        SOUND_HANDLE_DESTRUCT,
        MENU_FILE_00586150,
        TILE_READ_FILE,
        TILE_GET_MENU,
        MENU_SET_MENU_TILE,
        PRINT_FAILURE_005B5E40,
        TILE_GET_FLOAT,
        TILE_GET_MAXIMUM_DEPTH,
        GET_CLAMPED_ACTOR_VALUE,
        CHILD_TILE_00A08B20,
        MENU_PREP_FOR_VISIBILITY,
        MENU_007316F0,
        IDLE_OBJECT_CONSTRUCT_0041A250,
        ATEXIT,
        IS_TOP_MENU_ID,
        LIST_CONSTRUCT_0096A2D0,
        MILLISECONDS_00825C00,
        SEQUENCE_LENGTH_00508100,
        PLAY_SEQUENCE_0047AAB0,
        STOP_SEQUENCE_0047B220,
        MENU_STATE_0059BB30,
        MENU_START_FADE_OUT,
        STRING_CONSTRUCT_004037B0,
        STRING_DESTRUCT_004037D0,
        GET_FULL_NAME,
        STRING_FORMAT_00406F60,
        GET_PICK_UP_SOUND_NAME,
        CONTAINER_CONSTRUCT,
        CONTAINER_ADD_004818E0,
        CONTAINER_GIVE_004821A0,
        CONTAINER_DESTRUCT_00481680,
        CASINO_QUEST_00504E40,
        QUEST_0060C9C0,
        MISC_STAT_INCREMENT,
        FTOL2,
        LIST_CLEAR_00470470,
    ];

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
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

    fn st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// An engine with the statics and constants mapped and set, the callees doubled
    /// (list nodes are `[item, next]`, `NiPointer` assignment writes the holder) and the
    /// test vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_9000,
            0x011d_e000,
            0x011c_3000,
            0x0119_f000,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0102_1000,
            0x0102_3000,
            0x0102_9000,
            0x0102_c000,
            0x0105_3000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FULL_ALPHA, 255.0f32);
        e.set_global(DIM_ALPHA, 128.0f32);
        e.set_global(MINUS_FIVE_0102CAF8, -5.0f32);
        e.set_global(MESSAGE_TIME_010162C0, 2.0f32);
        e.set_global(CHIP_STEP_01070E78, 0.18f64);
        e.set_global(CHIP_HEIGHT_010177E8, 0.9f32);
        e.set_global(CHIP_HEIGHT_01053154, 1.8f32);
        e.set_global(CHIP_HEIGHT_01070E74, 1.44f32);
        e.set_global(CHIP_HEIGHT_01070E70, 0.36f32);
        e.set_global(CHIP_HEIGHT_01070E6C, 1.62f32);
        e.set_global(CHIP_HEIGHT_01070E68, 0.18f32);
        e.set_global(THREE_01021928, 3.0f64);
        e.set_global(DEPTH_TRAIT_6006, 6006.0f64);
        e.set_global(DEPTH_TRAIT_102, 102.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(ROUNDING_0_51, 0.51f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(QUARTER_DOUBLE, 0.25f64);
        e.set_global(IDLE_GUARD_011D90F0, 3u32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let slots: Vec<u32> = (0..0x60).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    /// An object with the test vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, VTABLE);
        block
    }

    /// The open menu: the test vtable and ten empty tiles.
    fn menu(e: &mut Engine) -> Ptr {
        let this = object(e, 0xe80);
        for i in 0..10 {
            let tile = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0x34 + 4 * i, tile);
        }
        e.set_global(BLACKJACK_MENU, this);
        Ptr::new(this)
    }

    /// A hand of cards of the given ranks: the node list (`[item, next]`).
    fn hand(e: &mut Engine, ranks: &[u32]) -> u32 {
        let mut next = 0;
        for &rank in ranks.iter().rev() {
            let card = e.mem.alloc(0x10);
            e.mem.set_u32(card, rank);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, card);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn step(count: i32) -> u32 {
        x87_store32(x87_mul(f64::from(count), 0.18))
    }

    #[test]
    fn set_bet_chips_stacks_each_bet_range() {
        let mut e = engine();
        let this = menu(&mut e);
        for i in 0..12 {
            e.mem.set_u32(this.addr() + 0xdc4 + 4 * i, 0x7000 + i);
        }
        let low = (-5.0f32).to_bits();
        let heights = |e: &mut Engine, bet: i32| {
            e.mem.set_i32(this.addr() + 0x6c, bet);
            e.call_log = Some(vec![]);
            e.call(0x0073_9b30, &args![this]);
            assert_eq!(calls_to(e, UPDATE_00A59C60).len(), 1);
            calls_to(e, SET_FLOATS_004BC1F0)
                .iter()
                .map(|call| (call[0] - 0x7000, call[3]))
                .collect::<Vec<_>>()
        };
        let small = heights(&mut e, 5);
        assert_eq!(small.len(), 12);
        assert_eq!(small[0], (0, step(5)));
        assert_eq!(small[6], (6, 0));
        assert_eq!(small[7], (7, low));
        let ten = heights(&mut e, 10);
        assert_eq!(ten[0], (0, step(5)));
        assert_eq!(ten[1], (1, step(1)));
        assert_eq!(ten[7], (7, low));
        assert_eq!(heights(&mut e, 20)[7], (7, 0));
        let five_hundred = heights(&mut e, 400);
        assert_eq!(five_hundred[3], (4, 0.36f32.to_bits()));
        assert_eq!(five_hundred[4], (3, step(4)));
        let seven_hundred = heights(&mut e, 700);
        assert_eq!(seven_hundred.len(), 13);
        assert_eq!(seven_hundred[10], (10, 0));
        assert_eq!(seven_hundred[11], (10, 0));
        assert_eq!(seven_hundred[12], (11, low));
        let fifteen_hundred = heights(&mut e, 0x5dc);
        assert_eq!(fifteen_hundred[5], (5, step(0)));
        assert_eq!(fifteen_hundred[11], (11, low));
        let many = heights(&mut e, 9000);
        assert_eq!(many[5], (5, step(10)));
        assert_eq!(many[11], (11, 0));
    }

    #[test]
    fn prepare_3d_elements_loads_attaches_and_lights() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + 0xdb4, 0x6000);
        e.register(MODEL_LOAD_FILE, |_, a| (a[1] & 0xfff).into_ret());
        let node = object(&mut e, 0xac);
        answer(&mut e, ALLOCATE_00AA13E0, node);
        e.register(NODE_CONSTRUCT_00A5ECB0, |_, a| a[0].into_ret());
        let deck = e.mem.alloc(0x10);
        let property = object(&mut e, 0x10);
        e.register_double(FIND_OBJECT_004AAE30, move |_, a| {
            if a[1] == DECK_OBJECT_NAME {
                deck.into_ret()
            } else {
                Ret::default()
            }
        });
        answer(&mut e, GET_PROPERTY, property);
        answer(&mut e, PROPERTY_TYPE_00441110, 9);
        // Only the player's hand (+0xdb8, loaded as 0x9c0) has a controller.
        e.register(GET_CONTROLLER, |_, a| {
            if a[0] == 0x9c0 { 0x5100u32 } else { 0 }.into_ret()
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[1].into_ret());
        e.register(SEQUENCE_0047A520, |_, a| (a[1] + 1).into_ret());
        answer(&mut e, MAKE_CAMERA_007C2180, 0x5500);
        let scene = object(&mut e, 0x10);
        answer(&mut e, SHADOW_SCENE_NODE, scene);
        st0(&mut e, BOUND_RADIUS_0084D030, 2.0);
        e.mem.set_u32(base + 0x9c, 0x4400);
        e.call_log = Some(vec![]);
        e.call(0x0073_2540, &args![this]);
        assert_eq!(e.mem.u32(base + 0xdb8), 0x9c0);
        assert_eq!(e.mem.u32(base + 0xdc0), 0x950);
        assert_eq!(e.mem.u32(TABLE_MODEL_011D9098), node);
        // The table, three hands and twelve chips attached to the model, and the model
        // to the shadow scene node.
        assert_eq!(calls_to(&e, slot(0xdc)).len(), 17);
        assert_eq!(e.mem.u32(base + 0xdf8), 0x5100);
        assert_eq!(e.mem.u32(base + 0xe0c), 0x0107_0b71);
        assert_eq!(e.mem.u32(base + 0xe28), 0x0107_0b1d);
        assert_eq!(e.mem.u32(base + 0xe04), 0);
        assert_eq!(calls_to(&e, DEACTIVATE_ALL), vec![vec![0x5100, 0]]);
        assert_eq!(calls_to(&e, START_ANIMATIONS), vec![vec![0x9c0]]);
        // No camera object: one is made.
        assert_eq!(e.mem.u32(CAMERA_011D90BC), 0x5500);
        assert_eq!(calls_to(&e, slot(0xf0)), vec![vec![property, 0, 0, 0x4400]]);
        assert_eq!(
            calls_to(&e, ADD_ALL_LIGHTS),
            vec![vec![scene, node, 6.0f32.to_bits()]]
        );
    }

    #[test]
    fn create_stops_while_barred_and_sets_up_the_labels() {
        let mut e = engine();
        let player = e.mem.alloc(0x700);
        e.set_global(PLAYER, player);
        let casino = object(&mut e, 0x220);
        e.mem.set_u32(casino + 0xc, 0x1234);
        answer(&mut e, FORM_ID_0084E3A0, 0x1234);
        // The player's records: one for another casino, then this one's.
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(other, 0x99);
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, 0x1234);
        e.mem.set_u32(record + 4, 50);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, record);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, other);
        e.mem.set_u32(first + 4, second);
        answer(&mut e, CASINO_RECORDS_005A6260, first);
        answer(&mut e, PLAYER_BARRED_00969B80, 1);
        answer(&mut e, PLAYER_BAR_TIME_00969B40, 42);
        answer(&mut e, SETTING_STRING, 0x3300);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x0073_3630, &args![casino, 5u32, 500u32, 0u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(e.global::<u32>(BET_DATA_011D9054), record);
        assert_eq!(e.global::<u32>(MINIMUM_BET_0119FE40), 5);
        assert_eq!(e.global::<u32>(MAXIMUM_BET_0119FE3C), 500);
        let format = calls_to(&e, FORMAT_TEXT);
        assert_eq!(format[0][1..], [0x104, 0x0107_0c58, 0x3300, 42]);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_007052F0).len(), 1);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], POPUP_SOUND);
        // Not barred, enough chips: the menu file read and the labels set up; the casino
        // data is missing (no casino), so 0 comes back after the set-up.
        answer(&mut e, PLAYER_BARRED_00969B80, 0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x28, 1000);
        answer(&mut e, CASINO_DATA_00504E60, data);
        answer(&mut e, GET_ITEM_COUNT, 30);
        answer(&mut e, CASINO_RECORDS_005A6260, first);
        answer(&mut e, CASINO_RECORDS_005A6260, 0);
        let fresh = e.mem.alloc(0xc);
        answer(&mut e, ALLOCATE_00401000, fresh);
        let menu = menu(&mut e);
        answer(&mut e, TILE_READ_FILE, 0x6600);
        answer(&mut e, TILE_GET_MENU, menu.addr());
        answer(&mut e, slot(0x34), BLACKJACK_MENU_CLASS);
        st0(&mut e, TILE_GET_FLOAT, 102.0);
        st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 3.5);
        answer(&mut e, GET_CLAMPED_ACTOR_VALUE, 7);
        answer(&mut e, STRING_LENGTH_0044A670, 4);
        e.call_log = Some(vec![]);
        let got = e.call(0x0073_3630, &args![0u32, 5u32, 500u32, 0u32]).u32();
        assert_eq!(got, 0);
        // No records: a new one for the casino.
        assert_eq!(calls_to(&e, ALLOCATE_00401000), vec![vec![0xc]]);
        assert_eq!(e.global::<u32>(BET_DATA_011D9054), fresh);
        assert_eq!(e.mem.u32(fresh), 0x1234);
        assert_eq!(e.mem.u32(menu.addr() + 0x68), 7);
        assert_eq!(e.mem.u32(menu.addr() + 0x6c), 5);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6600, 0xfad, 3.5f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], SHUFFLE_SOUND);
        let values = calls_to(&e, TILE_SET_INT);
        assert_eq!(values[0][2], 5);
        assert_eq!(values[1][2], 30);
        assert!(calls_to(&e, MENU_PREP_FOR_VISIBILITY).is_empty());
    }

    #[test]
    fn do_click_bets_hits_and_closes() {
        let mut e = engine();
        let this = menu(&mut e);
        let base = this.addr();
        e.set_global(MINIMUM_BET_0119FE40, 5u32);
        e.set_global(MAXIMUM_BET_0119FE3C, 1800u32);
        e.mem.set_u32(base + 0x28, 0xa);
        e.mem.set_u32(base + 0x74, 5000);
        let click = |e: &mut Engine, id: i32| {
            e.call_log = Some(vec![]);
            e.call(0x0073_3ff0, &args![this, id, 0u32]);
        };
        let bet = |e: &Engine| e.mem.i32(base + 0x6c);
        e.mem.set_i32(base + 0x6c, 38);
        click(&mut e, 5);
        assert_eq!(bet(&e), 43);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2],
            INCREASE_BET_SOUND
        );
        assert_eq!(calls_to(&e, TILE_SET_INT).last().unwrap()[2], 43);
        e.mem.set_i32(base + 0x6c, 1500);
        click(&mut e, 5);
        assert_eq!(bet(&e), 1800);
        e.mem.set_i32(base + 0x6c, 45);
        click(&mut e, 6);
        assert_eq!(bet(&e), 40);
        click(&mut e, 6);
        assert_eq!(bet(&e), 35);
        e.mem.set_i32(base + 0x6c, 3);
        click(&mut e, 6);
        assert_eq!(bet(&e), 5);
        assert!(calls_to(&e, GET_SOUND_HANDLE_BY_NAME).is_empty());
        click(&mut e, 7);
        assert_eq!(bet(&e), 1800);
        // Playing: a hit on the player's hand (8 and 9 make 20 with the drawn card's
        // value not added by the double).
        e.mem.set_u32(base + 0x28, 4);
        let player = hand(&mut e, &[8, 9]);
        e.mem.set_u32(base + 0xd94, player);
        e.mem.set_u32(base + 0xda0, 0x30);
        e.mem.set_i32(base + 0x68, 5);
        click(&mut e, 3);
        assert_eq!(e.mem.u32(base + 0x28), 2);
        assert_eq!(calls_to(&e, LIST_APPEND_00905820).len(), 1);
        assert_eq!(e.mem.u32(base + 0xdb0) & 0x401, 1);
        // Stand without flag 1 is ignored; button 8 ends the round; on betting it closes.
        click(&mut e, 7);
        assert_eq!(e.mem.u32(base + 0x28), 2);
        e.mem.set_u32(base + 0x28, 4);
        click(&mut e, 8);
        assert_eq!(e.mem.u32(base + 0x28), 8);
        e.mem.set_u32(base + 0x28, 0xa);
        click(&mut e, 8);
        assert_eq!(e.mem.u32(base + 0x28), 0xd);
        // Other states and ids do nothing.
        e.mem.set_u32(base + 0x28, 2);
        click(&mut e, 5);
        assert_eq!(calls_to(&e, 0x0073_3ff0).len(), 1);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A menu ready for `DoIdle`: on top, ticking, in `state`.
    fn idle_menu(e: &mut Engine, state: u32) -> Ptr {
        let this = menu(e);
        e.mem.set_u32(this.addr() + 0x28, state);
        e.mem.set_u32(this.addr() + 0x7c, 100);
        e.set_global(IDLE_STATE_011D90DC, state);
        answer(e, IS_TOP_MENU_ID, 1);
        answer(e, MILLISECONDS_00825C00, 600);
        this
    }

    #[test]
    fn do_idle_ticks_and_bets() {
        let mut e = engine();
        let this = idle_menu(&mut e, 0xa);
        let base = this.addr();
        e.set_global(IDLE_TIME_011D90E0, 0.25f32);
        e.set_global(MINIMUM_BET_0119FE40, 5u32);
        e.mem.set_u32(base + 0xd9c, 1);
        e.mem.set_u32(base + 0xda0, 10);
        e.mem.set_f32(base + 0x64, 0.25);
        e.mem.set_i32(base + 0x74, 3);
        e.set_global(IDLE_GUARD_011D90F0, 2u32);
        e.mem.set_i32(base + 0x6c, 5);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(
            calls_to(&e, IDLE_OBJECT_CONSTRUCT_0041A250),
            vec![vec![IDLE_OBJECT_011D90E4]]
        );
        assert_eq!(e.global::<u32>(IDLE_GUARD_011D90F0), 3);
        assert_eq!(e.global::<f32>(IDLE_TIME_011D90E0), 0.75);
        assert_eq!(e.mem.u32(base + 0x7c), 600);
        // Shuffled (13 below 52 * 0.25), the bet cut to the chips, and closed for want of
        // the minimum bet.
        assert_eq!(e.mem.u32(base + 0xda0), 0x34);
        assert_eq!(e.mem.i32(base + 0x6c), 3);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_007052F0).len(), 1);
        assert_eq!(e.mem.u32(base + 0x28), 0xd);
        // A state change clears the time; the first call makes the hand lists.
        e.set_global(IDLE_STATE_011D90DC, 0xdu32);
        e.mem.set_u32(base + 0x28, 4);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.global::<f32>(IDLE_TIME_011D90E0), 0.0);
        e.mem.set_u32(base + 0x7c, 0);
        answer(&mut e, ALLOCATE_00401000, 0x8800);
        e.register(LIST_CONSTRUCT_0096A2D0, |_, a| a[0].into_ret());
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0xd90), 0x8800);
        // State 4: the player's bust (0x400) ends the round.
        e.mem.set_u32(base + 0xdb0, 0x400);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x28), 8);
    }

    #[test]
    fn do_idle_settles_a_win() {
        let mut e = engine();
        let this = idle_menu(&mut e, 0xc);
        e.register(SETTING_STRING, |_, a| a[0].into_ret());
        let base = this.addr();
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record + 4, 100);
        e.set_global(BET_DATA_011D9054, record);
        let player = hand(&mut e, &[8, 9]);
        let dealer = hand(&mut e, &[5, 8]);
        e.mem.set_u32(base + 0xd94, player);
        e.mem.set_u32(base + 0xd90, dealer);
        e.mem.set_i32(base + 0x6c, 10);
        e.mem.set_i32(base + 0x74, 200);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x2c), 3);
        assert_eq!(e.mem.i32(base + 0x74), 210);
        assert_eq!(e.mem.i32(record + 4), 110);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x29]]);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], WIN_SOUND);
        assert_eq!(
            calls_to(&e, FORMAT_TEXT).last().unwrap()[3..5],
            [0x011d_2ff4, 10]
        );
        assert_eq!(
            calls_to(&e, LIST_CLEAR_00470470),
            vec![vec![player], vec![dealer]]
        );
        assert_eq!(e.mem.u32(base + 0x28), 7);
        // A blackjack paid through the ratio: 10 * 1.5 + 0.51 truncated.
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.mem.set_f32(base + 0x60, 1.5);
        e.mem.set_u32(base + 0x2c, 5);
        e.mem.set_u32(base + 0x28, 0xc);
        e.set_global(IDLE_STATE_011D90DC, 0xcu32);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.i32(base + 0x74), 225);
    }

    #[test]
    fn do_idle_dealer_draws_below_seventeen() {
        let mut e = engine();
        let this = idle_menu(&mut e, 0xb);
        let base = this.addr();
        let dealer = hand(&mut e, &[3, 8]);
        e.mem.set_u32(base + 0xd90, dealer);
        e.mem.set_u32(base + 0xda0, 0x30);
        e.mem.set_i32(base + 0x68, 5);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x28), 9);
        assert_eq!(calls_to(&e, LIST_APPEND_00905820).len(), 1);
        assert_eq!(e.mem.u32(base + 0xdb0) & 0x1000, 0x1000);
        // A bust player (result 4) skips the dealer.
        e.mem.set_u32(base + 0x28, 0xb);
        e.set_global(IDLE_STATE_011D90DC, 0xbu32);
        e.mem.set_u32(base + 0x2c, 4);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x28), 0xc);
        assert_eq!(e.mem.u32(base + 0xdb0) & 0x2000, 0x2000);
    }

    #[test]
    fn do_idle_closes_and_takes_the_lost_chips() {
        let mut e = engine();
        let this = idle_menu(&mut e, 0xd);
        let base = this.addr();
        let player = object(&mut e, 0x10);
        e.set_global(PLAYER, player);
        e.mem.set_u32(base + 4, 0x7700);
        e.set_global(IDLE_TIME_011D90E0, 0.25f32);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert!(calls_to(&e, MENU_START_FADE_OUT).is_empty());
        // Half a second more makes one: the player held 30 and has 20 left.
        answer(&mut e, MILLISECONDS_00825C00, 1100);
        answer(&mut e, CASINO_CHIP_007316D0, 0x4100);
        answer(&mut e, GET_ITEM_COUNT, 30);
        e.mem.set_i32(base + 0x74, 20);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![base]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x7700, 0x1772, 1]]);
        assert_eq!(
            calls_to(&e, slot(0x17c)),
            vec![vec![player, 0x4100, 0, 10, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, STRING_FORMAT_00406F60)[0][1..3],
            [LOSS_FORMAT_0101C178, 10]
        );
        assert!(calls_to(&e, CONTAINER_CONSTRUCT).is_empty());
    }

    #[test]
    fn do_idle_reveals_splits_and_collects() {
        let mut e = engine();
        let this = idle_menu(&mut e, 8);
        let base = this.addr();
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x28), 0xb);
        // With the reveal sequence, at time 0: flags, the flip and the sequence played.
        e.mem.set_u32(base + 0x28, 8);
        e.set_global(IDLE_STATE_011D90DC, 7u32);
        e.mem.set_u32(base + 0xe6c, 0x5e6c);
        e.mem.set_u32(base + 0xe00, 0x5e00);
        st0(&mut e, SEQUENCE_LENGTH_00508100, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0xdb0), 0xc00);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], FLIP_SOUND);
        assert_eq!(
            calls_to(&e, PLAY_SEQUENCE_0047AAB0),
            vec![vec![0x5e00, 0x5e6c, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(calls_to(&e, NODE_0043B4A0).len(), 2);
        assert_eq!(e.mem.u32(base + 0x28), 8);
        // State 3 with the split's sequences and the split hand's first card.
        e.mem.set_u32(base + 0x28, 3);
        for offset in [0xe30u32, 0xe34, 0xe14] {
            e.mem.set_u32(base + offset, 0x5000 + offset);
        }
        e.mem.set_u32(base + 0x88, 1);
        e.mem.set_u32(base + 0x84, 2);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0xdb0) & 0x4010, 0x4010);
        assert_eq!(e.mem.u32(base + 0x84), 1);
        assert_eq!(e.mem.u32(base + 0x28), 2);
        // State 7 without the hands' sequences goes back to betting.
        e.mem.set_u32(base + 0x28, 7);
        e.call(0x0073_4db0, &args![this]);
        assert_eq!(e.mem.u32(base + 0x28), 0xa);
    }
}

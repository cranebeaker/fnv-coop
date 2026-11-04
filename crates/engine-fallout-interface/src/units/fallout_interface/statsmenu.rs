//! `fallout/interface/menus/statsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007df1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range assigned to `statsmenu.cpp`; `fastcall`/`thiscall`
/// with `this` in `ECX`, plain `RET`): returns the float at `this + 0x20` in `ST0`
/// (`FLD`, which quiets a signalling NaN). The process classes call it as
/// `fHourLastProcessed` (Xbox PDB) of a `LowProcess`; the body is a plain field getter, so
/// it is named by address and takes any object.
pub fn fn_007df1f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x20)))
}

/// The open stats menu (`StatsMenu::Create` stores the new menu here).
const OPEN_MENU_011DACE0: u32 = 0x011d_ace0;
/// The 0x4c-byte fade object `007db1c0` makes on first use.
const FADE_OBJECT_011DACE4: u32 = 0x011d_ace4;
/// The time `DoIdle` next updates the meters at (milliseconds).
const NEXT_UPDATE_011DAD74: u32 = 0x011d_ad74;
/// The byte `DoIdle` keeps the last hardcore state in.
const HARDCORE_SHOWN_011A0AE1: u32 = 0x011a_0ae1;
/// `UpdateHealingMode`'s function-local statics: the guard word and the `"_xbox_button"`
/// (bit 1) and `"_enabled"` (bit 2) trait ids.
const HEALING_GUARD_011DAD80: u32 = 0x011d_ad80;
const XBOX_BUTTON_TRAIT_011DAD7C: u32 = 0x011d_ad7c;
const ENABLED_TRAIT_011DAD78: u32 = 0x011d_ad78;
/// `Interface::GetPipboy` (`00705990`), `InterfaceManager::IsInPipboyMenu` (`007178a0`), the time
/// (`00457fe0`, milliseconds), `Interface::IsTopMenuID` (`00702450`, `cdecl`), the
/// hardcore meters (`007ddc20`, `007ddd10`, `007dde00`), `007e0060` (`RET 4`), the Pip-Boy
/// burst (`FORenderedMenu::TriggerBurst`, `007fb150`, `RET 0xc`) and its two game settings
/// and `float`, `StatsMenu::SetKnobPosition` (`007dfc90`, `cdecl`),
/// `StatsMenu::SetIconAndDescription` (`007dc7e0`, `RET 4`), `ListBox<PerkRankData_P>::NumUnfiltered`
/// (`00726710`), `007d86c0` (`RET 8`), `007dc9b0` (`RET 4`) and the item list callback walk
/// `007314c0` (`RET 0xc`; in `bartermenu.cpp`). The ones in `statsmenu.cpp` are not
/// translated here.
const INTERFACE_GET_PIPBOY: u32 = 0x0070_5990;
const INTERFACE_MANAGER_IS_IN_PIPBOY_MENU: u32 = 0x0071_78a0;
const TIME_00457FE0: u32 = 0x0045_7fe0;
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
const STATS_007DDC20: u32 = 0x007d_dc20;
const STATS_007DDD10: u32 = 0x007d_dd10;
const STATS_007DDE00: u32 = 0x007d_de00;
const STATS_007E0060: u32 = 0x007e_0060;
const FO_RENDERED_MENU_TRIGGER_BURST: u32 = 0x007f_b150;
const BURST_SCALE_01016248: u32 = 0x0101_6248;
const BURST_SETTING_011DB37C: u32 = 0x011d_b37c;
const BURST_SETTING_011DB458: u32 = 0x011d_b458;
const STATS_MENU_SET_KNOB_POSITION: u32 = 0x007d_fc90;
const STATS_MENU_SET_ICON_AND_DESCRIPTION: u32 = 0x007d_c7e0;
const PERK_RANK_NUM_UNFILTERED: u32 = 0x0072_6710;
const LIST_BOX_DATA_AT_007D86C0: u32 = 0x007d_86c0;
const STATS_007DC9B0: u32 = 0x007d_c9b0;
const ITEM_CHANGE_007314C0: u32 = 0x0073_14c0;
/// The player (`PlayerCharacter*`); its actor values are the object at +0xa4 (vtable
/// slots +8: the value as an `int`, +0xc: as a `float` in `ST0`, +0x1c: the base value).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
const AV_VALUE: u32 = 0x08;
const AV_FLOAT: u32 = 0x0c;
const AV_BASE: u32 = 0x1c;

/// `MemoryManager` allocate and free (`00401000`, `00401030`, `cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The `eh vector constructor iterator` (`00ec782f`, `stdcall`, five words) and the
/// embedded list's constructor (`0096a2d0`) and destructor (`0046ffb0`) it is given.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `00403d30` (`cdecl`, three words: `memset`).
const FILL_00403D30: u32 = 0x0040_3d30;
/// List helpers: `005ae3d0` (append the pointed-to word, `RET 4`), `006815c0` (a node's
/// item slot), `00726070` (the word at +4: a node's next, a menu's tile), `008256d0` (the
/// list is empty, `AL`), `0063f7b0` (removes the first node), `00470470` (frees the
/// nodes).
const LIST_APPEND: u32 = 0x005a_e3d0;
const NODE_SELF: u32 = 0x0068_15c0;
const WORD_AT_4_00726070: u32 = 0x0072_6070;
const LIST_IS_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_REMOVE_FIRST_0063F7B0: u32 = 0x0063_f7b0;
const LIST_FREE_NODES_00470470: u32 = 0x0047_0470;
/// `0071f9c0` (`thiscall`, `RET 4`): a status effect's deleting destructor.
const STATUS_EFFECT_DELETE: u32 = 0x0071_f9c0;
/// The `StatusDataList` vtable (`01012650`).
const STATUS_DATA_LIST_VTABLE: u32 = 0x0101_2650;
/// Tile functions (`BSMenu/tile.cpp`).
const TILE_ADD_FADE_CONTROL: u32 = 0x00a0_7c60;
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// The fade the callback adds: the `float` at `01023cd8`.
const FADE_TARGET_01023CD8: u32 = 0x0102_3cd8;
/// The `float` `007ddef0` returns (`01013974`).
const VALUE_01013974: u32 = 0x0101_3974;
/// `00810fd0` (`thiscall`: the word at +0x98) and `00403c80` (`cdecl`: a flag test).
const WORD_AT_98_00810FD0: u32 = 0x0081_0fd0;
const FLAG_TEST_00403C80: u32 = 0x0040_3c80;
/// `00825c00` (`thiscall`: the word at +0x14).
const WORD_AT_14_00825C00: u32 = 0x0082_5c00;
/// `ListBox<StartMenu::Option_P>::GetTileFor` (`007a22d0`, `RET 4`; the template code).
const LIST_BOX_GET_TILE_FOR: u32 = 0x007a_22d0;
/// `005a3370` (`cdecl`): a misc stat's value.
const MISC_STAT_VALUE_005A3370: u32 = 0x005a_3370;
/// `ActorValueOwner::GetClampedActorValue` (`0066ef20`, `RET 4`).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `AiFormulas::AVDeriveSkill` (`00643c90`, `cdecl`, three words, `ST0`) and `00406d90`
/// (`cdecl`, a `float`: rounds to an `int`).
const AV_DERIVE_SKILL: u32 = 0x0064_3c90;
const ROUND_00406D90: u32 = 0x0040_6d90;
/// `0045cd60` (`thiscall`: the word at +0x28).
const WORD_AT_28_0045CD60: u32 = 0x0045_cd60;
/// The up and down arrow strings `"+"`-like (`01077560`, `0107755c`).
const ARROW_UP_01077560: u32 = 0x0107_7560;
const ARROW_DOWN_0107755C: u32 = 0x0107_755c;
/// `_ftol2_sse` (`00ec62c0`).
const FTOL2: u32 = 0x00ec_62c0;
/// `5.0`, `0.0`, `1.0`, `1000.0`, `2.0`, `pi`, `pi / 2` (`double`s) and `0.0005` (`float`).
const FIVE_DOUBLE: u32 = 0x0102_0998;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const TWO_DOUBLE: u32 = 0x0101_1590;
const PI_DOUBLE: u32 = 0x0101_ff40;
const HALF_PI_DOUBLE: u32 = 0x0106_4f50;
const KNOB_RATE_010780A4: u32 = 0x0107_80a4;
/// `0055b980` (`thiscall` on an active effect: its kind), `00833c20` (`thiscall`, a `bool`).
const EFFECT_KIND_0055B980: u32 = 0x0055_b980;
const EFFECT_ACTIVE_00833C20: u32 = 0x0083_3c20;
/// `004d1360` (`thiscall` on the player: hardcore mode).
const HARDCORE_004D1360: u32 = 0x004d_1360;
/// `007e06b0` (no arguments; in `statsmenu.cpp`, not translated here).
const STATS_007E06B0: u32 = 0x007e_06b0;
/// `004b71d0` (no arguments, a `bool`), `004b7210` (the interface manager), `00704370`
/// (its +0x174), `007f86f0` (`RET 0xc`; in `fopipboymanager.cpp`), `00974d90` (`thiscall`:
/// the byte at +0x28).
const FLAG_004B71D0: u32 = 0x004b_71d0;
const INTERFACE_MANAGER_004B7210: u32 = 0x004b_7210;
const WORD_AT_174_00704370: u32 = 0x0070_4370;
const PIPBOY_007F86F0: u32 = 0x007f_86f0;
const BYTE_AT_28_00974D90: u32 = 0x0097_4d90;
/// `005c5420` (no arguments, `ST0`), `00404010` and `004bd510` (`cdecl`, two `float`s,
/// `ST0`).
const VALUE_005C5420: u32 = 0x005c_5420;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
const FLOAT_PAIR_004BD510: u32 = 0x004b_d510;
/// `00408da0` (`thiscall` on a string: its text), `00404dc0` (`cdecl`, two strings).
const STRING_CHARS_00408DA0: u32 = 0x0040_8da0;
const COMPARE_00404DC0: u32 = 0x0040_4dc0;
/// `0084e3a0`-free helpers of the effect items: `00403ea0` (`thiscall`: the effect's
/// key), `EffectItem::GetMagnitude` (`00404390`), `EffectItem::GetDuration` (`004044d0`),
/// `00403e60` (`thiscall`, `RET 4`: a flag test), `00403ba0` (`thiscall`: the 0x14-byte
/// entry constructor).
const EFFECT_KEY_00403EA0: u32 = 0x0040_3ea0;
const EFFECT_GET_MAGNITUDE: u32 = 0x0040_4390;
const EFFECT_GET_DURATION: u32 = 0x0040_44d0;
const FLAG_TEST_00403E60: u32 = 0x0040_3e60;
const ENTRY_CONSTRUCT_00403BA0: u32 = 0x0040_3ba0;
/// The `BSString` helpers (`004037b0` construct, `004037d0` destroy, `00406f60` format
/// (`cdecl`), `00559450` text, `00438390` assign (`RET 4`)) and the game setting string
/// (`00403df0`, `thiscall` on the setting).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_CHARS_00559450: u32 = 0x0055_9450;
const STRING_ASSIGN_00438390: u32 = 0x0043_8390;
const SETTING_STRING_00403DF0: u32 = 0x0040_3df0;
/// The player's level (`0087f9f0`, `AX`) and name (`0055d520`).
const PLAYER_LEVEL_0087F9F0: u32 = 0x0087_f9f0;
const REFERENCE_NAME_0055D520: u32 = 0x0055_d520;
/// `TESActorBaseData::GetKarmicTitle` (`0047e0e0`, `cdecl`).
const GET_KARMIC_TITLE: u32 = 0x0047_e0e0;
/// `TESObjectREFR::GetInventoryItem` (`00576260`, `RET 8`),
/// `Actor::GetItemCountinContainer` (`00575610`, `RET 4`), `004459e0` (`RET 4`: the
/// inventory entry's deleting call).
const GET_INVENTORY_ITEM: u32 = 0x0057_6260;
const GET_ITEM_COUNT_IN_CONTAINER: u32 = 0x0057_5610;
const INVENTORY_ENTRY_DELETE: u32 = 0x0044_59e0;
/// `007e0230` (`thiscall`; in `statsmenu.cpp`, not translated here).
const STATS_007E0230: u32 = 0x007e_0230;
/// `"%d"`-like formats and strings (`.rdata`).
const HEALING_FORMAT_01077548: u32 = 0x0107_7548;
const NAME_FORMAT_01077550: u32 = 0x0107_7550;
const ONE_STRING_FORMAT: u32 = 0x0101_9f08;
const TOPIC_ID_FORMAT: u32 = 0x0106_eb00;
const XBOX_BUTTON_TEXT: u32 = 0x0107_34d0;
const ENABLED_TEXT: u32 = 0x0107_0244;
/// The game settings `UpdateHealingMode` and `UpdatePlayerName` read.
const SETTING_011D4708: u32 = 0x011d_4708;
const SETTING_011D2658: u32 = 0x011d_2658;
const SETTING_011D2028: u32 = 0x011d_2028;
/// `ListBox<PerkRankData_P>`'s and `ListBox<MiscStatID>`'s `Synchronize` and
/// `SetTotalListHeight` (`menumanager.cpp`).
const PERK_RANK_SYNCHRONIZE: u32 = 0x0072_4930;
const PERK_RANK_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_4dd0;
const MISC_STAT_SYNCHRONIZE: u32 = 0x0072_4f00;
const MISC_STAT_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_53a0;
/// `ListBox` helpers the `AddItem` instances use (as in `levelupmenu.cpp`).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
const ITEM_CONSTRUCT: u32 = 0x007a_62e0;
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
const LIST_APPEND_00905820: u32 = 0x0090_5820;
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
const LIST_BOX_COUNT_00726C40: u32 = 0x0072_6c40;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
const VALUE_CALCULATE: u32 = 0x00a0_9410;
const NUMBER_OF_VISIBLE_ITEMS_TEXT: u32 = 0x0106_f860;

/// The player.
fn player(e: &Engine) -> u32 {
    e.global(PLAYER_011DEA3C)
}

/// The player's actor values (`player + 0xa4`).
fn actor_values(e: &Engine) -> u32 {
    player(e).wrapping_add(0xa4)
}

/// The open stats menu.
fn open_menu(e: &Engine) -> u32 {
    e.global(OPEN_MENU_011DACE0)
}

/// A tile's integer trait (`00700320`).
fn set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// A tile's string trait (`00a01350` with 1).
fn set_string(e: &mut Engine, tile: u32, trait_id: u32, text: u32) {
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);
}

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node (`00726070`).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4_00726070, &args![node]).u32()
}

/// `FCOMP; FNSTSW AX; TEST AH,0x41; JNZ`: on only when strictly above.
fn above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater)
}

// Translated from 00406de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00406de0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the body) is not translated.
/// The `StatsMenu::StatusDataList` destructor (the engine map's `StatusDataList::Flush`
/// name is the one its body calls first; `thiscall`): the vtable `01012650`, the entries
/// freed (`007e0850`), then the embedded list destroyed (`0046ffb0` on +4).
pub fn fn_00406de0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STATUS_DATA_LIST_VTABLE);
    fn_007e0850(e, this);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 007db1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007db1c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the construction) is not translated.
/// Unnamed (no arguments): the fade object at `011dace4`, made on first use (0x4c bytes
/// from the allocator, constructed by `007db2c0`; 0 when the allocation fails).
pub fn fn_007db1c0(e: &mut Engine) -> u32 {
    if e.global::<u32>(FADE_OBJECT_011DACE4) == 0 {
        let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
        let object = if block != 0 {
            fn_007db2c0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(FADE_OBJECT_011DACE4, object);
    }
    e.global(FADE_OBJECT_011DACE4)
}

// Translated from 007db250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 8`: a group and a tile): appends the tile
/// to the group's list (`005ae3d0` on `this + 4 + group * 8`, given the address of the
/// tile's argument slot).
pub fn fn_007db250(e: &mut Engine, this: Ptr, group: u32, tile: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), tile);
        let list = this
            .addr()
            .wrapping_add(group.wrapping_mul(8))
            .wrapping_add(4);
        e.call(LIST_APPEND, &args![list, slot]);
    });
}

// Translated from 007db270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 0xc`: a group, a byte index and a value):
/// stores the value's low byte as byte `index` of the group's word at +0x34 (the shifts
/// by `CL`, so the count is taken modulo 32).
pub fn fn_007db270(e: &mut Engine, this: Ptr, group: u32, index: u32, value: u32) {
    let field = this
        .addr()
        .wrapping_add(group.wrapping_mul(4))
        .wrapping_add(0x34);
    let shift = index.wrapping_shl(3);
    let mask = !0xffu32.wrapping_shl(shift);
    let word = e.mem.u32(field) & mask;
    e.mem.set_u32(field, word);
    let word = value.wrapping_shl(shift) | e.mem.u32(field);
    e.mem.set_u32(field, word);
}

// Translated from 007db2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007db2c0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the lists are built) is not translated.
/// Unnamed (`thiscall`, the fade object's constructor): six 8-byte lists from +4 built by
/// the vector constructor (`0096a2d0`, destroyed by `0046ffb0`), the word at +0 cleared and
/// the six group words at +0x34 filled with -1 (`00403d30(this + 0x34, -1, 0x18)`).
/// Returns `this`.
pub fn fn_007db2c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this.addr() + 4, 8u32, 6u32, LIST_CONSTRUCT, LIST_DESTRUCT],
    );
    e.mem.set_u32(this.addr(), 0);
    e.call(
        FILL_00403D30,
        &args![this.addr() + 0x34, 0xffff_ffffu32, 0x18u32],
    );
    this
}

// Translated from 007db340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): whether all 60 `pTiles` (+0x90) are set.
pub fn fn_007db340(e: &mut Engine, this: Ptr) -> bool {
    (0..0x3cu32).all(|index| e.mem.u32(this.addr() + 0x90 + 4 * index) != 0)
}

// Translated from 007dbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 4`): runs `FadeControlCallback` with the
/// argument over the tiles of the current group (the group number at +0; `007e0e30` on
/// `this + 4 + group * 8`).
pub fn fn_007dbfe0(e: &mut Engine, this: Ptr, flag: u32) {
    let group = e.mem.u32(this.addr());
    let list = this
        .addr()
        .wrapping_add(group.wrapping_mul(8))
        .wrapping_add(4);
    fn_007e0e30(e, Ptr::new(list), 0x007d_c040, flag);
}

// Translated from 007dc040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::FadeControlCallback` (Xbox PDB; `cdecl`, a tile and a flag): with the flag,
/// adds the tile a fade control on trait `0xfa9` (to the `float` at `01023cd8`, from 0.0
/// over 1.0, looping); without it, removes the fade control and sets the trait to 0xff.
/// Returns true.
pub fn stats_menu_fade_control_callback(e: &mut Engine, tile: u32, flag: u32) -> bool {
    if flag != 0 {
        let target = x87_quiet_bits(e.global::<u32>(FADE_TARGET_01023CD8));
        e.call(
            TILE_ADD_FADE_CONTROL,
            &args![
                tile,
                0xfa9u32,
                target,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                1u32
            ],
        );
    } else {
        e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, 0xfa9u32]);
        set_int(e, tile, 0xfa9, 0xff);
    }
    true
}

// Translated from 007dc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a pointer): increments the word it points to and returns the pointer.
pub fn fn_007dc0a0(e: &mut Engine, value: Ptr) -> Ptr {
    let word = e.mem.u32(value.addr()).wrapping_add(1);
    e.mem.set_u32(value.addr(), word);
    value
}

// Translated from 007dc4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::DoIdle` (Xbox PDB). Clears `bActiveEffectsUpdatedThisFrame`. While the
/// Pip-Boy is up (`Interface::GetPipboy`, its byte +0x28) and in the Pip-Boy menu, once the
/// time (`00457fe0`) reaches the next update time, the meters are updated (`007ddb00`, and
/// in hardcore mode `007ddc20`, `007ddd10`, `007dde00`) and the next time set to the time
/// plus the menu tile's trait `0x1007` times 1000 (truncated). With `bLimbMode` and the
/// stats menu not on top (`0x3eb`), `007e0230`. On the first page with the skill page
/// index (`007dcc40`) above 2 (5 in hardcore mode), `007e0060(0)`. When hardcore mode
/// changed since the last idle (`011a0ae1`), the three hardcore tiles (+0x164, +0x174,
/// +0x16c of the open menu) get traits `0xfaf` and `0xfa3` set to it; the state is kept.
pub fn stats_menu_do_idle(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x2a1, 0);
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    if e.call(BYTE_AT_28_00974D90, &args![pipboy]).bool() {
        e.call(INTERFACE_MANAGER_004B7210, &args![]);
        if e.call(INTERFACE_MANAGER_IS_IN_PIPBOY_MENU, &args![]).bool() {
            let time = e.call(TIME_00457FE0, &args![]).u32();
            if time >= e.global::<u32>(NEXT_UPDATE_011DAD74) {
                fn_007ddb00(e, this);
                if player(e) != 0 && e.call(HARDCORE_004D1360, &args![player(e)]).bool() {
                    e.call(STATS_007DDC20, &args![this]);
                    e.call(STATS_007DDD10, &args![this]);
                    e.call(STATS_007DDE00, &args![this]);
                }
                let time = e.call(TIME_00457FE0, &args![]).u32();
                let start = f64::from(time);
                let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
                let delay = e.call(TILE_GET_FLOAT, &args![root, 0x1007u32]).f64();
                let thousand: f64 = e.global(THOUSAND_DOUBLE);
                let next = x87_add(x87_mul(delay, thousand), start);
                let next = x87_fistp64_chop(X87Ext::from(next)) as u32;
                e.set_global(NEXT_UPDATE_011DAD74, next);
            }
        }
        if e.mem.u8(this.addr() + 0x2a0) != 0
            && !e.call(INTERFACE_IS_TOP_MENU_ID, &args![0x3ebu32]).bool()
        {
            e.call(STATS_007E0230, &args![this]);
        }
    }
    if fn_007dcc00(e, this) == 0 {
        let page = fn_007dcc40(e, this);
        let limit = if e.call(HARDCORE_004D1360, &args![player(e)]).bool() {
            5
        } else {
            2
        };
        if page > limit {
            e.call(STATS_007E0060, &args![this, 0u32]);
        }
    }
    let set_all = |e: &mut Engine, value: u32| {
        for offset in [0x164u32, 0x174, 0x16c] {
            for trait_id in [0xfafu32, 0xfa3] {
                let tile = e.mem.u32(open_menu(e) + offset);
                set_int(e, tile, trait_id, value);
            }
        }
    };
    let mut done = false;
    if player(e) != 0
        && !e.call(HARDCORE_004D1360, &args![player(e)]).bool()
        && e.global::<u8>(HARDCORE_SHOWN_011A0AE1) != 0
    {
        set_all(e, 0);
        e.set_global(HARDCORE_SHOWN_011A0AE1, 0u8);
        done = true;
    }
    if !done
        && player(e) != 0
        && e.call(HARDCORE_004D1360, &args![player(e)]).bool()
        && e.global::<u8>(HARDCORE_SHOWN_011A0AE1) == 0
    {
        set_all(e, 1);
    }
    let hardcore = e.call(HARDCORE_004D1360, &args![player(e)]).u32() as u8;
    e.set_global(HARDCORE_SHOWN_011A0AE1, hardcore);
}

// Translated from 007dcc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the menu tile's (+4) trait `0x1004` (the
/// current page), truncated (`FISTP qword` with chopping; the low word).
pub fn fn_007dcc00(e: &mut Engine, this: Ptr) -> i32 {
    let tile = e.mem.u32(this.addr() + 4);
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    x87_fistp64_chop(X87Ext::from(value)) as i32
}

// Translated from 007dcc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): trait `0x1004` of the tile at +0xa4,
/// truncated.
pub fn fn_007dcc40(e: &mut Engine, this: Ptr) -> i32 {
    let tile = e.mem.u32(this.addr() + 0xa4);
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    x87_fistp64_chop(X87Ext::from(value)) as i32
}

// Translated from 007dcfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::UpdateAllActorValues` (Xbox PDB): `UpdateActorValue` for 0x10, 0xc, 0x18,
/// 0x36, 0x14 and 0x19 to 0x1f; both list pages reset (trait `0x1004` of +0xa8 and +0xac to
/// 0) and refreshed through `007314c0` with `SPECIALUpdateFn` / `SkillsUpdateFn` for every
/// index; then `UpdateActorValue(0x17)`.
pub fn stats_menu_update_all_actor_values(e: &mut Engine, this: Ptr) {
    for value in [0x10u32, 0xc, 0x18, 0x36, 0x14] {
        stats_menu_update_actor_value(e, this, value);
    }
    for value in 0x19..0x20u32 {
        stats_menu_update_actor_value(e, this, value);
    }
    for (tile, list, callback) in [
        (0xa8u32, 0x180u32, 0x007d_fa30u32),
        (0xac, 0x1b0, 0x007d_fb00),
    ] {
        let tile = e.mem.u32(this.addr() + tile);
        set_int(e, tile, 0x1004, 0);
        e.call(
            ITEM_CHANGE_007314C0,
            &args![this.addr() + list, callback, 0u32, 0x7fff_ffffu32],
        );
    }
    stats_menu_update_actor_value(e, this, 0x17);
}

/// `UpdateActorValue`'s strings and settings: `"%d/%d"`, the face path format, the
/// rads-like format of value 0x14, the body part formats (`"%s.dds"` when above 0, the
/// broken one otherwise), `"_Value"`, the karma icon paths by alignment (0 to 4) and the
/// game settings with the alignment names, the experience cap setting and the "max level"
/// string setting.
const VALUE_PAIR_FORMAT: u32 = 0x0107_3b14;
const FACE_FORMAT: u32 = 0x0107_3868;
const VALUE_14_FORMAT: u32 = 0x0107_74c4;
const BODY_PART_FORMAT: u32 = 0x0107_3850;
const BODY_PART_BROKEN_FORMAT: u32 = 0x0107_3830;
const VALUE_TEXT: u32 = 0x0106_ed18;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const LEVEL_CAP_OWNER_011D0C60: u32 = 0x011d_0c60;
const MAX_LEVEL_SETTING_011D472C: u32 = 0x011d_472c;
const VALUE_14_SETTING_011D0AD8: u32 = 0x011d_0ad8;
/// The body part names by actor value (`011a0a80 + 4 * value`).
const BODY_PART_NAMES_011A0A80: u32 = 0x011a_0a80;
/// `0043d4d0` (`thiscall` on `011d0c60`: the address of the level cap),
/// `GamePlayFormulas::GetRequiredExperiencePoints` (`00648b50`, `cdecl`),
/// `TESActorBaseData::GetAlignmentForKarma` (`0047e040`, `cdecl`, a `float`), `004a8f20`
/// (`cdecl`, two words).
const LEVEL_CAP_0043D4D0: u32 = 0x0043_d4d0;
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;
const ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
const VALUE_14_004A8F20: u32 = 0x004a_8f20;
/// `00403e20` (`thiscall` on a game setting: its value's address).
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;

/// The value text of the tile of `UpdateActorValue`'s value pair: `"%d/%d"` of the
/// value (slot +8) and the base value (slot +0x1c), on the menu's tile at the trait of
/// `007dfdf0`.
fn show_value_pair(e: &mut Engine, this: Ptr, text: u32, value: u32) {
    let base = e.vcall(actor_values(e), AV_BASE, &args![value]).u32();
    let current = e.vcall(actor_values(e), AV_VALUE, &args![value]).u32();
    e.call(
        STRING_FORMAT,
        &args![text, VALUE_PAIR_FORMAT, current, base],
    );
    show_on_menu_tile(e, this, text, value);
}

/// The string's text on the menu's tile at the trait `007dfdf0` gives for `value`.
fn show_on_menu_tile(e: &mut Engine, this: Ptr, text: u32, value: u32) {
    let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
    let trait_id = fn_007dfdf0(e, this, value);
    let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
    set_string(e, root, trait_id, string);
}

// Translated from 007dd090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007dd090 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// `StatsMenu::UpdateActorValue` (Xbox PDB; `RET 4`, an actor value). With a local
/// `BSString`:
///
/// - 0x10: the face image (`007dfe40`) on tile +0xf0, the value pair, then
///   `UpdateHealingOptions(0)`.
/// - 0xc: the value pair.
/// - 0x18 (experience): the value and the next level's required points, or at the level
///   cap (`0043d4d0`) the "max level" setting's string.
/// - 0x36: the meters (`007ddb00`), `UpdateHealingOptions(2)`, `UpdateActiveEffects`.
/// - 0x14: `004a8f20` of the truncated setting value and the clamped value, formatted on
///   tile +0x128, then `UpdateActiveEffects`.
/// - 0x17 (karma): the alignment's icon on tile +0x120 and its name on tile +0x11c, then
///   `UpdatePlayerTitle`.
/// - 0x19 to 0x1e (body parts): the part's image (broken when the condition is not above
///   0) on tile `pTiles[0xc + 2 * (value - 0x19)]` and the condition / 100 as `"_Value"` of
///   the next tile; `UpdateHealingOptions(0)` in limb mode.
/// - Any other value: `SPECIALUpdateFn` and `SkillsUpdateFn` on its tiles in the SPECIAL and
///   skills lists (`GetTileFor` given the address of the value's argument slot).
pub fn stats_menu_update_actor_value(e: &mut Engine, this: Ptr, value: u32) {
    e.with_stack(0xc, |e, frame| {
        let text = frame.addr();
        let value_slot = text + 8;
        e.mem.set_u32(value_slot, value);
        e.call(STRING_CONSTRUCT, &args![text]);
        match value {
            0x10 => {
                let face = fn_007dfe40(e, this);
                e.call(STRING_FORMAT, &args![text, FACE_FORMAT, face]);
                let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
                let tile = e.mem.u32(this.addr() + 0xf0);
                set_string(e, tile, 0xfcc, string);
                show_value_pair(e, this, text, 0x10);
                stats_menu_update_healing_options(e, this, 0);
            }
            0xc => show_value_pair(e, this, text, 0xc),
            0x18 => {
                let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
                let cap = e
                    .call(LEVEL_CAP_0043D4D0, &args![LEVEL_CAP_OWNER_011D0C60])
                    .u32();
                if (level as i32) < e.mem.u32(cap) as i32 {
                    let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
                    let required = e.call(REQUIRED_EXPERIENCE, &args![level + 1]).u32();
                    let current = e.vcall(actor_values(e), AV_VALUE, &args![0x18u32]).u32();
                    e.call(
                        STRING_FORMAT,
                        &args![text, VALUE_PAIR_FORMAT, current, required],
                    );
                } else {
                    let string = e
                        .call(SETTING_STRING_00403DF0, &args![MAX_LEVEL_SETTING_011D472C])
                        .u32();
                    e.call(STRING_FORMAT, &args![text, ONE_STRING_FORMAT, string]);
                }
                show_on_menu_tile(e, this, text, 0x18);
            }
            0x36 => {
                fn_007ddb00(e, this);
                stats_menu_update_healing_options(e, this, 2);
                stats_menu_update_active_effects(e, this);
            }
            0x14 => {
                let clamped = e
                    .call(GET_CLAMPED_ACTOR_VALUE, &args![actor_values(e), 0x14u32])
                    .u32();
                let setting = e
                    .call(SETTING_VALUE_00403E20, &args![VALUE_14_SETTING_011D0AD8])
                    .u32();
                let whole = e.call(FTOL2, &args![x87_load32(e.mem.u32(setting))]).u32();
                let result = e.call(VALUE_14_004A8F20, &args![whole, clamped]).u32();
                e.call(STRING_FORMAT, &args![text, VALUE_14_FORMAT, result]);
                let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
                let tile = e.mem.u32(this.addr() + 0x128);
                set_string(e, tile, 0xfc4, string);
                stats_menu_update_active_effects(e, this);
            }
            0x17 => {
                let karma = x87_store32(e.vcall(actor_values(e), AV_FLOAT, &args![0x17u32]).f64());
                let alignment = e.call(ALIGNMENT_FOR_KARMA, &args![karma]).u32();
                // The jump table of the alignments 0 to 4: the icon path and the name setting.
                let karma = match alignment {
                    0 => Some((0x0107_7450u32, 0x011d_41b0u32)),
                    1 => Some((0x0107_7488, 0x011d_3204)),
                    2 => Some((0x0107_7418, 0x011d_457c)),
                    3 => Some((0x0107_73dc, 0x011d_4ffc)),
                    4 => Some((0x0107_73a4, 0x011d_31d4)),
                    _ => None,
                };
                if let Some((icon, setting)) = karma {
                    let icon_tile = e.mem.u32(this.addr() + 0x120);
                    set_string(e, icon_tile, 0xfcc, icon);
                    let name = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
                    let name_tile = e.mem.u32(this.addr() + 0x11c);
                    set_string(e, name_tile, 0xfc4, name);
                }
                stats_menu_update_player_title(e);
            }
            0x19..=0x1e => {
                let image = 0xc + 2 * (value - 0x19);
                let bar = image + 1;
                let condition = e.vcall(actor_values(e), AV_FLOAT, &args![value]).f64();
                let hundred: f64 = e.global(HUNDRED_DOUBLE);
                let fraction = x87_store32(x87_div(condition, hundred));
                let zero: f64 = e.global(ZERO_DOUBLE);
                let format = if above(x87_load32(fraction), zero) {
                    BODY_PART_FORMAT
                } else {
                    BODY_PART_BROKEN_FORMAT
                };
                let name = e.mem.u32(BODY_PART_NAMES_011A0A80 + 4 * value);
                e.call(STRING_FORMAT, &args![text, format, name]);
                let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
                let tile = e.mem.u32(this.addr() + 0x90 + 4 * image);
                set_string(e, tile, 0xfcc, string);
                let value_trait = e
                    .call(ADD_USER_TRAIT, &args![VALUE_TEXT, 0xffff_ffffu32])
                    .u32();
                let tile = e.mem.u32(this.addr() + 0x90 + 4 * bar);
                e.call(
                    TILE_SET_FLOAT,
                    &args![tile, value_trait, x87_quiet_bits(fraction), 1u32],
                );
                if e.mem.u8(this.addr() + 0x2a0) != 0 {
                    stats_menu_update_healing_options(e, this, 0);
                }
            }
            _ => {
                for (list, update) in [(0x180u32, true), (0x1b0, false)] {
                    let value = e.mem.u32(value_slot);
                    let tile = e
                        .call(
                            LIST_BOX_GET_TILE_FOR,
                            &args![this.addr() + list, value_slot],
                        )
                        .u32();
                    if update {
                        stats_menu_special_update_fn(e, tile, value);
                    } else {
                        stats_menu_skills_update_fn(e, tile, value);
                    }
                }
            }
        }
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007dd710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007dd710 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// `StatsMenu::UpdatePerks` (Xbox PDB; no arguments). With the open menu and its perk
/// tile (+0x118): empties the perk list (+0x1e0, its vtable slot +0x1c), then for every
/// perk rank of the player (slot +0x4e8(0) gives the list) whose perk is not hidden
/// (`007dd8a0`) adds it to the list (`007e0e70`, sorted by `007e04e0`) with its name, and
/// `"%s (%d)"` with the rank when that is above 1.
pub fn stats_menu_update_perks(e: &mut Engine) {
    let menu = open_menu(e);
    if menu == 0 || e.mem.u32(menu + 0x118) == 0 {
        return;
    }
    let list = open_menu(e) + 0x1e0;
    e.vcall(list, 0x1c, &args![]);
    let mut node = e.vcall(player(e), 0x4e8, &args![0u32]).u32();
    e.with_stack(8, |e, text| {
        let text = text.addr();
        while node != 0 && node_item(e, node) != 0 {
            let rank = node_item(e, node);
            let perk = e.mem.u32(rank);
            if fn_007dd8a0(e, Ptr::new(perk)) == 0 {
                e.call(STRING_CONSTRUCT, &args![text]);
                let rank = node_item(e, node);
                if e.mem.u8(rank + 4) > 1 {
                    let rank = node_item(e, node);
                    let level = u32::from(e.mem.u8(rank + 4));
                    let rank = node_item(e, node);
                    let perk = e.mem.u32(rank);
                    let name = e
                        .call(STRING_CHARS_00408DA0, &args![perk.wrapping_add(0x18)])
                        .u32();
                    e.call(STRING_FORMAT, &args![text, TOPIC_ID_FORMAT, name, level]);
                } else {
                    let rank = node_item(e, node);
                    let perk = e.mem.u32(rank);
                    let name = e
                        .call(STRING_CHARS_00408DA0, &args![perk.wrapping_add(0x18)])
                        .u32();
                    e.call(STRING_FORMAT, &args![text, ONE_STRING_FORMAT, name]);
                }
                let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
                let rank = node_item(e, node);
                let list = open_menu(e) + 0x1e0;
                fn_007e0e70(e, Ptr::new(list), rank, string, 0x007e_04e0, 0);
                e.call(STRING_DESTROY, &args![text]);
            }
            node = node_next(e, node);
        }
    });
}

// Translated from 007dd8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the byte at +0x3c in `AL` (a perk's hidden flag where
/// `UpdatePerks` uses it).
pub fn fn_007dd8a0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x3c))
}

// Translated from 007ddad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu, `RET 4`, a misc stat): its tile in the general
/// list (+0x210, `GetTileFor` given the address of the argument slot) gets the stat's
/// value (`007dfc60`).
pub fn fn_007ddad0(e: &mut Engine, this: Ptr, stat: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stat);
        let tile = e
            .call(LIST_BOX_GET_TILE_FOR, &args![this.addr() + 0x210, slot])
            .u32();
        fn_007dfc60(e, tile, stat);
    });
}

// Translated from 007ddb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the radiation meter. With `a` the menu tile's
/// trait `0x1008` times 1000 and `b = -a / 2`, `c = 005c5420() * a + b`; `d` is
/// `00404010(0.0, rads)` (the player's actor value 0x36) and the knob angle `pi/2 - (d + c)
/// / 1000 * pi`. A `d` strictly between 0 and 1 becomes 1, any other `004bd510(d, 1.0)`;
/// it goes to trait `0x1005` of tile +4, and the Pip-Boy's knob (`007f86f0` on the
/// interface manager's +0x174) is set to (1, angle, the `float` at `010780a4`). Every step
/// is the exe's x87 operation, rounded to `float` where the code stores one.
pub fn fn_007ddb00(e: &mut Engine, this: Ptr) {
    let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
    let value = e.call(TILE_GET_FLOAT, &args![root, 0x1008u32]).f64();
    let thousand: f64 = e.global(THOUSAND_DOUBLE);
    let two: f64 = e.global(TWO_DOUBLE);
    let a = x87_store32(x87_mul(value, thousand));
    // `FLD a; FCHS; FDIV 2.0; FSTP double`.
    let b = x87_div(-x87_load32(a), two);
    let scale = e.call(VALUE_005C5420, &args![]).f64();
    let c = x87_store32(x87_add(x87_mul(scale, x87_m32(a)), b));
    let rads = x87_store32(e.vcall(actor_values(e), AV_FLOAT, &args![0x36u32]).f64());
    let d = x87_store32(
        e.call(FLOAT_PAIR_00404010, &args![0.0f32.to_bits(), rads])
            .f64(),
    );
    let pi: f64 = e.global(PI_DOUBLE);
    let half_pi: f64 = e.global(HALF_PI_DOUBLE);
    let sum = x87_add(x87_load32(d), x87_m32(c));
    let angle = x87_store32(x87_sub(half_pi, x87_mul(x87_div(sum, thousand), pi)));
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    // `TEST AH,5; JP`: on only when `d` is below 1.0 (ordered).
    let d = if above(x87_load32(d), zero) && x87_load32(d) < one {
        1.0f32.to_bits()
    } else {
        x87_store32(
            e.call(
                FLOAT_PAIR_004BD510,
                &args![x87_quiet_bits(d), 1.0f32.to_bits()],
            )
            .f64(),
        )
    };
    let tile = e.mem.u32(this.addr() + 4);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0x1005u32, x87_quiet_bits(d), 1u32],
    );
    let rate = x87_quiet_bits(e.global::<u32>(KNOB_RATE_010780A4));
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    let pipboy = e.call(WORD_AT_174_00704370, &args![interface]).u32();
    e.call(
        PIPBOY_007F86F0,
        &args![pipboy, 1u32, x87_quiet_bits(angle), rate],
    );
}

// Translated from 007ddef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the `float` at `01013974`, loaded with `FLD` (quieting a
/// signalling NaN) and returned in `ST0`.
pub fn fn_007ddef0(e: &mut Engine) -> f32 {
    x87_quiet(e.global::<f32>(VALUE_01013974))
}

// Translated from 007df1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): `00403c80(00810fd0(this), 0x40)`, its `EAX` returned (the flag
/// test's 0 or 1).
pub fn fn_007df1b0(e: &mut Engine, this: Ptr) -> bool {
    let value = e.call(WORD_AT_98_00810FD0, &args![this]).u32();
    e.call(FLAG_TEST_00403C80, &args![value, 0x40u32]).bool()
}

// Translated from 007df1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): `00825c00` (the word at +0x14) of the object at +0xc.
pub fn fn_007df1d0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr().wrapping_add(0xc));
    e.call(WORD_AT_14_00825C00, &args![object]).u32()
}

// Translated from 007df210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): 1 when bit 7 of the word at +0x18 is set, else 0 (in `EAX`).
pub fn fn_007df210(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u32(this.addr().wrapping_add(0x18)) & 0x80 != 0)
}

// Translated from 007df230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007df230 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// `StatsMenu::UpdateHealingOptions` (Xbox PDB; `RET 4`, an option 0 to 3 or 4 for all).
/// Nothing without a player. For each chosen option (0x10-byte records from +0x28: the
/// item form, its tile, the inventory entry, the availability callback): the old entry is
/// deleted (`004459e0` with 1), the player's entry for the form taken
/// (`GetInventoryItem(form, 0)`), the tile's `0xfaf` set to the callback's answer (`cdecl`,
/// given the record); with an entry the tile's text is `"(%d) %s"` of the entry's count
/// (its +4) and the tile's string `0x1004` (option 0 outside hardcore mode, and option 3,
/// also enable the healing tile +0xb4), without one just that string. For options 0, 3
/// and 4, when neither the first nor the fourth option's item is carried (in hardcore mode
/// the fourth alone decides), limb mode ends (`007e0230`) and the healing tile is disabled.
pub fn stats_menu_update_healing_options(e: &mut Engine, this: Ptr, which: u32) {
    if player(e) == 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        let text = text.addr();
        for index in 0..4u32 {
            if which != index && which != 4 {
                continue;
            }
            let record = this.addr() + 0x28 + index * 0x10;
            e.call(STRING_CONSTRUCT, &args![text]);
            let old = e.mem.u32(record + 8);
            if old != 0 {
                e.call(INVENTORY_ENTRY_DELETE, &args![old, 1u32]);
            }
            let form = e.mem.u32(record);
            let entry = e
                .call(GET_INVENTORY_ITEM, &args![player(e), form, 0u32])
                .u32();
            e.mem.set_u32(record + 8, entry);
            let callback = e.mem.u32(record + 0xc);
            let available = e.call(callback, &args![record]).u32() as u8;
            let tile = e.mem.u32(record + 4);
            set_int(e, tile, 0xfaf, u32::from(available));
            if e.mem.u32(record + 8) != 0 {
                let tile = e.mem.u32(record + 4);
                let name = e.call(TILE_GET_STRING, &args![tile, 0x1004u32]).u32();
                let entry = e.mem.u32(record + 8);
                let count = e.call(WORD_AT_4_00726070, &args![entry]).u32();
                e.call(
                    STRING_FORMAT,
                    &args![text, HEALING_FORMAT_01077548, count, name],
                );
                let enable = (index == 0 && !e.call(HARDCORE_004D1360, &args![player(e)]).bool())
                    || index == 3;
                if enable {
                    let healing = e.mem.u32(this.addr() + 0xb4);
                    set_int(e, healing, 0xfaf, 1);
                }
            } else {
                let tile = e.mem.u32(record + 4);
                let name = e.call(TILE_GET_STRING, &args![tile, 0x1004u32]).u32();
                e.call(STRING_ASSIGN_00438390, &args![text, name]);
            }
            let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
            let tile = e.mem.u32(record + 4);
            set_string(e, tile, 0xfc4, string);
            e.call(STRING_DESTROY, &args![text]);
        }
    });
    if which == 0 || which == 3 || which == 4 {
        let first = e.mem.u32(this.addr() + 0x28);
        let first_count = e
            .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player(e), first])
            .u32();
        let fourth = e.mem.u32(this.addr() + 0x58);
        let fourth_count = e
            .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player(e), fourth])
            .u32();
        let unavailable =
            if e.call(HARDCORE_004D1360, &args![player(e)]).bool() && fourth_count == 0 {
                true
            } else {
                !e.call(HARDCORE_004D1360, &args![player(e)]).bool()
                    && fourth_count == 0
                    && first_count == 0
            };
        if unavailable {
            if e.mem.u8(this.addr() + 0x2a0) != 0 {
                e.call(STATS_007E0230, &args![this]);
            }
            let healing = e.mem.u32(this.addr() + 0xb4);
            set_int(e, healing, 0xfaf, 0);
        }
    }
}

// Translated from 007df4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007df4e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// `StatsMenu::UpdatePlayerName` (Xbox PDB; no arguments): with the open menu and the
/// player, tile +0xfc shows `"%s - %s %hd"` of the player's name, the level setting's
/// string (`011d2028`) and the level, and the menu's tile gets the level as `0x100c`.
pub fn stats_menu_update_player_name(e: &mut Engine) {
    if open_menu(e) == 0 || player(e) == 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        let text = text.addr();
        e.call(STRING_CONSTRUCT, &args![text]);
        let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
        let label = e
            .call(SETTING_STRING_00403DF0, &args![SETTING_011D2028])
            .u32();
        let name = e.call(REFERENCE_NAME_0055D520, &args![player(e)]).u32();
        e.call(
            STRING_FORMAT,
            &args![text, NAME_FORMAT_01077550, name, label, level],
        );
        let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
        let tile = e.mem.u32(open_menu(e) + 0xfc);
        set_string(e, tile, 0xfc4, string);
        let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
        let root = e.call(WORD_AT_4_00726070, &args![open_menu(e)]).u32();
        set_int(e, root, 0x100c, level);
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007df5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::UpdatePlayerTitle` (Xbox PDB; no arguments): with the open menu's title tile
/// (+0x124), its text becomes the player's karmic title (`GetKarmicTitle(player)`).
pub fn stats_menu_update_player_title(e: &mut Engine) {
    if open_menu(e) == 0 || e.mem.u32(open_menu(e) + 0x124) == 0 {
        return;
    }
    let title = e.call(GET_KARMIC_TITLE, &args![player(e)]).u32();
    let tile = e.mem.u32(open_menu(e) + 0x124);
    set_string(e, tile, 0xfc4, title);
}

// Translated from 007df620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007df620 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' look-ups) is not translated.
/// `StatsMenu::UpdateHealingMode` (Xbox PDB). Looks up `"_xbox_button"` and `"_enabled"`
/// (function-local statics). With the healing tile (+0xb4) and `004b71d0`: its
/// `"_xbox_button"` is 10 in limb mode, 11 otherwise, its text the mode's setting string,
/// and the eleven page tiles (+0x90, +0x94, +0x98, +0x9c, +0xa0, +0x100, +0x104, +0x164,
/// +0x174, +0x16c, +0x108) are enabled outside limb mode. Then the fade object
/// (`007db1c0`) fades its current group in limb mode (`007dbfe0`).
pub fn stats_menu_update_healing_mode(e: &mut Engine, this: Ptr) {
    if e.global::<u32>(HEALING_GUARD_011DAD80) & 1 == 0 {
        let guard: u32 = e.global(HEALING_GUARD_011DAD80);
        e.set_global(HEALING_GUARD_011DAD80, guard | 1);
        let id = e
            .call(ADD_USER_TRAIT, &args![XBOX_BUTTON_TEXT, 0xffff_ffffu32])
            .u32();
        e.set_global(XBOX_BUTTON_TRAIT_011DAD7C, id);
    }
    if e.global::<u32>(HEALING_GUARD_011DAD80) & 2 == 0 {
        let guard: u32 = e.global(HEALING_GUARD_011DAD80);
        e.set_global(HEALING_GUARD_011DAD80, guard | 2);
        let id = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        e.set_global(ENABLED_TRAIT_011DAD78, id);
    }
    let limb = |e: &Engine| e.mem.u8(this.addr() + 0x2a0) != 0;
    if e.mem.u32(this.addr() + 0xb4) != 0 && e.call(FLAG_004B71D0, &args![]).bool() {
        let button = if limb(e) { 0xa } else { 0xb };
        let id: u32 = e.global(XBOX_BUTTON_TRAIT_011DAD7C);
        let healing = e.mem.u32(this.addr() + 0xb4);
        set_int(e, healing, id, button);
        let setting = if limb(e) {
            SETTING_011D4708
        } else {
            SETTING_011D2658
        };
        let text = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
        let healing = e.mem.u32(this.addr() + 0xb4);
        set_string(e, healing, 0xfc4, text);
        for offset in [
            0x90u32, 0x94, 0x98, 0x9c, 0xa0, 0x100, 0x104, 0x164, 0x174, 0x16c, 0x108,
        ] {
            let enabled = u32::from(!limb(e));
            let id: u32 = e.global(ENABLED_TRAIT_011DAD78);
            let tile = e.mem.u32(this.addr() + offset);
            set_int(e, tile, id, enabled);
        }
    }
    let flag = u32::from(e.mem.u8(this.addr() + 0x2a0));
    let fade = fn_007db1c0(e);
    fn_007dbfe0(e, Ptr::new(fade), flag);
}

/// The arrow and the change flag `SPECIALUpdateFn` and `SkillsUpdateFn` give a tile:
/// `0x1004` 1, the arrow (`0x1006`; up for a positive change) and the item tile's (+0x28)
/// `0x1004` 1.
fn show_change(e: &mut Engine, tile: u32, up: bool) {
    set_int(e, tile, 0x1004, 1);
    let arrow = if up {
        ARROW_UP_01077560
    } else {
        ARROW_DOWN_0107755C
    };
    set_string(e, tile, 0x1006, arrow);
    let item = e.call(WORD_AT_28_0045CD60, &args![tile]).u32();
    set_int(e, item, 0x1004, 1);
}

// Translated from 007dfa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SPECIALUpdateFn` (Xbox PDB; `cdecl`, a tile and an actor value): the tile
/// shows the clamped value (`0x1005`); when it differs from the base value the change
/// marker is shown (the arrow up when the value is above the base), else `0x1004` is 0.
pub fn stats_menu_special_update_fn(e: &mut Engine, tile: u32, value: u32) {
    if tile == 0 {
        return;
    }
    let current = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![actor_values(e), value])
        .i32();
    let base = e.vcall(actor_values(e), AV_BASE, &args![value]).i32();
    set_int(e, tile, 0x1005, current as u32);
    if base == current {
        set_int(e, tile, 0x1004, 0);
    } else {
        show_change(e, tile, current > base);
    }
}

// Translated from 007dfb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SkillsUpdateFn` (Xbox PDB; `cdecl`, a tile and an actor value): the change
/// is `current - base` plus the rounded difference of the derived skill with and without
/// modifiers (`AVDeriveSkill(values, value, 0)` and `(.., 1)`, each stored as a `float`,
/// subtracted and stored again, then `00406d90`); the tile shows the clamped value and,
/// for a non-zero change, the change marker (up when positive).
pub fn stats_menu_skills_update_fn(e: &mut Engine, tile: u32, value: u32) {
    if tile == 0 {
        return;
    }
    let current = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![actor_values(e), value])
        .i32();
    let base = e.vcall(actor_values(e), AV_BASE, &args![value]).i32();
    let values = if player(e) != 0 { actor_values(e) } else { 0 };
    let derived = x87_store32(e.call(AV_DERIVE_SKILL, &args![values, value, 0u32]).f64());
    let values = if player(e) != 0 { actor_values(e) } else { 0 };
    let modified = x87_store32(e.call(AV_DERIVE_SKILL, &args![values, value, 1u32]).f64());
    let difference = x87_store32(x87_sub(x87_load32(derived), x87_m32(modified)));
    let rounded = e
        .call(ROUND_00406D90, &args![x87_quiet_bits(difference)])
        .i32();
    let change = current.wrapping_sub(base).wrapping_add(rounded);
    set_int(e, tile, 0x1005, current as u32);
    if change != 0 {
        show_change(e, tile, change > 0);
    } else {
        set_int(e, tile, 0x1004, 0);
    }
}

// Translated from 007dfc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a tile and a misc stat): a non-null tile shows the stat's value
/// (`005a3370`) as `0x1005`.
pub fn fn_007dfc60(e: &mut Engine, tile: u32, stat: u32) {
    if tile != 0 {
        let value = e.call(MISC_STAT_VALUE_005A3370, &args![stat]).u32();
        set_int(e, tile, 0x1005, value);
    }
}

// Translated from 007dfcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the low byte of the open stats menu's current page
/// (`007dcc00`), or 0 without an open menu.
pub fn fn_007dfcd0(e: &mut Engine) -> u8 {
    let menu = open_menu(e);
    if menu == 0 {
        return 0;
    }
    fn_007dcc00(e, Ptr::new(menu)) as u8
}

// Translated from 007dfdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, an actor value): the trait its value pair is shown in:
/// 0x10 `0x1009`, 0xc `0x100a`, 0x18 `0x100b`, anything else `0x80000000`.
pub fn fn_007dfdf0(_e: &mut Engine, _this: Ptr, value: u32) -> u32 {
    match value {
        0xc => 0x100a,
        0x10 => 0x1009,
        0x18 => 0x100b,
        _ => 0x8000_0000,
    }
}

// Translated from 007dfe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the face image number. 10 while the player has an active effect
/// of kind 0xa (`0055b980`) that is active (`00833c20`); otherwise `_ftol2(((base - value)
/// / base) * 5)` of actor value 0x10 (the base as an `int` stored as a `float`, the value
/// subtracted from it and stored), kept within 0 to 4.
pub fn fn_007dfe40(e: &mut Engine, _this: Ptr) -> i32 {
    let effects = player(e).wrapping_add(0x94);
    let mut node = e.vcall(effects, 8, &args![]).u32();
    while node != 0 && node_item(e, node) != 0 {
        let effect = node_item(e, node);
        if e.call(EFFECT_KIND_0055B980, &args![effect]).u32() == 0xa {
            let effect = node_item(e, node);
            if e.call(EFFECT_ACTIVE_00833C20, &args![effect]).bool() {
                return 0xa;
            }
        }
        node = node_next(e, node);
    }
    let base = e.vcall(actor_values(e), AV_BASE, &args![0x10u32]).i32();
    let base = x87_store32(f64::from(base));
    let value = e.vcall(actor_values(e), AV_FLOAT, &args![0x10u32]).f64();
    let missing = x87_store32(x87_sub(x87_m32(base), value));
    let five: f64 = e.global(FIVE_DOUBLE);
    let ratio = x87_mul(x87_div(x87_load32(missing), x87_m32(base)), five);
    e.call(FTOL2, &args![ratio]).i32().clamp(0, 4)
}

// Translated from 007dff40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SetCurrentPage` (Xbox PDB; `RET 4`, a page 0 to 4; others do nothing): the
/// Pip-Boy bursts (`FORenderedMenu::TriggerBurst` with two game settings and the `float` at
/// `01016248`), the menu tile's `0x1004` becomes the page, the knob moves when the Pip-Boy
/// is up (`SetKnobPosition(1)`), limb mode ends (`007e0230`); page 1 shows icon 5, page 2
/// icon 0x20 (`SetIconAndDescription`), page 3 with perks the first perk's
/// (`007dc9b0(007d86c0(perks, 0, 0))`), any other the icon tile (+0xf4) is hidden.
pub fn stats_menu_set_current_page(e: &mut Engine, this: Ptr, page: i32) {
    if !(0..=4).contains(&page) {
        return;
    }
    let burst = x87_quiet_bits(e.global::<u32>(BURST_SCALE_01016248));
    let first = e
        .call(SETTING_VALUE_00403E20, &args![BURST_SETTING_011DB37C])
        .u32();
    let first = x87_quiet_bits(e.mem.u32(first));
    let second = e
        .call(SETTING_VALUE_00403E20, &args![BURST_SETTING_011DB458])
        .u32();
    let second = x87_quiet_bits(e.mem.u32(second));
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    let pipboy = e.call(WORD_AT_174_00704370, &args![interface]).u32();
    e.call(
        FO_RENDERED_MENU_TRIGGER_BURST,
        &args![pipboy, second, first, burst],
    );
    let root = e.mem.u32(this.addr() + 4);
    set_int(e, root, 0x1004, page as u32);
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    let pipboy = e.call(WORD_AT_174_00704370, &args![interface]).u32();
    if e.call(BYTE_AT_28_00974D90, &args![pipboy]).bool() {
        e.call(STATS_MENU_SET_KNOB_POSITION, &args![1u32]);
    }
    if e.mem.u8(this.addr() + 0x2a0) != 0 {
        e.call(STATS_007E0230, &args![this]);
    }
    match page {
        1 => {
            e.call(STATS_MENU_SET_ICON_AND_DESCRIPTION, &args![this, 5u32]);
        }
        2 => {
            e.call(STATS_MENU_SET_ICON_AND_DESCRIPTION, &args![this, 0x20u32]);
        }
        _ => {
            if page == 3
                && e.call(PERK_RANK_NUM_UNFILTERED, &args![this.addr() + 0x1e0])
                    .u32()
                    != 0
            {
                let perk = e
                    .call(
                        LIST_BOX_DATA_AT_007D86C0,
                        &args![this.addr() + 0x1e0, 0u32, 0u32],
                    )
                    .u32();
                e.call(STATS_007DC9B0, &args![this, perk]);
            } else {
                let tile = e.mem.u32(this.addr() + 0xf4);
                set_int(e, tile, 0xfa3, 0);
            }
        }
    }
}

// Translated from 007e04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two list items: the perk list's sort): `00404dc0` of the names (the
/// string at +0x18 of each item's data's first word), the second's looked up first.
pub fn fn_007e04e0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let data = e.mem.u32(second.wrapping_add(4));
    let perk = e.mem.u32(data);
    let second_name = e
        .call(STRING_CHARS_00408DA0, &args![perk.wrapping_add(0x18)])
        .u32();
    let data = e.mem.u32(first.wrapping_add(4));
    let perk = e.mem.u32(data);
    let first_name = e
        .call(STRING_CHARS_00408DA0, &args![perk.wrapping_add(0x18)])
        .u32();
    e.call(COMPARE_00404DC0, &args![first_name, second_name])
        .i32()
}

// Translated from 007e05f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a healing option record: the first option's availability callback):
/// false without an inventory entry (+8). With `007e06b0`'s actor value (-1 for none): in
/// hardcore mode any value but -1 answers false; outside it a value is used only when
/// `004b71d0` says so, else 0x10. The answer is whether the player's value (slot +8) is
/// below its base (slot +0x1c).
pub fn fn_007e05f0(e: &mut Engine, record: Ptr) -> bool {
    let hardcore = e.call(HARDCORE_004D1360, &args![player(e)]).bool();
    if e.mem.u32(record.addr() + 8) == 0 {
        return false;
    }
    let mut value = e.call(STATS_007E06B0, &args![]).u32();
    if hardcore && value != 0xffff_ffff {
        return false;
    }
    if hardcore || !e.call(FLAG_004B71D0, &args![]).bool() || value == 0xffff_ffff {
        value = 0x10;
    }
    let current = e.vcall(actor_values(e), AV_VALUE, &args![value]).i32();
    let base = e.vcall(actor_values(e), AV_BASE, &args![value]).i32();
    current < base
}

// Translated from 007e07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a healing option record: the radiation option's availability): an
/// inventory entry (+8) and radiation (actor value 0x36) above 0.0.
pub fn fn_007e07a0(e: &mut Engine, record: Ptr) -> bool {
    if e.mem.u32(record.addr() + 8) == 0 {
        return false;
    }
    let rads = e.vcall(actor_values(e), AV_FLOAT, &args![0x36u32]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    above(rads, zero)
}

// Translated from 007e07f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a status effect list): until the embedded list (+4) is empty
/// (`008256d0`), deletes its first item (`0071f9c0` with 1, when non-null) and removes the
/// first node (`0063f7b0`).
pub fn fn_007e07f0(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    while !e.call(LIST_IS_EMPTY_008256D0, &args![list]).bool() {
        let item = node_item(e, list);
        if item != 0 {
            e.call(STATUS_EFFECT_DELETE, &args![item, 1u32]);
        }
        e.call(LIST_REMOVE_FIRST_0063F7B0, &args![list]);
    }
}

// Translated from 007e0850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a status data list): until the embedded list (+4) is empty,
/// frees its first item and removes the first node.
pub fn fn_007e0850(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    while !e.call(LIST_IS_EMPTY_008256D0, &args![list]).bool() {
        let item = node_item(e, list);
        e.call(FREE, &args![item]);
        e.call(LIST_REMOVE_FIRST_0063F7B0, &args![list]);
    }
}

// Translated from 007e09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): frees the nodes of the embedded list (`00470470` on +4).
pub fn fn_007e09b0(e: &mut Engine, this: Ptr) {
    e.call(LIST_FREE_NODES_00470470, &args![this.addr() + 4]);
}

// Translated from 007e0b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e0b20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the entry's construction) is not translated.
/// `StatsMenu::StatusDataList::Register` (Xbox PDB; `RET 4`, an effect item): an entry
/// with the effect's key (`00403ea0`, at entry +0x10; `007e0ce0`) gains its magnitude;
/// otherwise a new 0x14-byte entry (`00403ba0`) takes the key and the magnitude (negated
/// when the effect setting has flag 4), the duration unless the setting has flag 2, and is
/// appended to the list (+4).
pub fn stats_menu_status_data_list_register(e: &mut Engine, this: Ptr, effect: u32) {
    if effect == 0 {
        return;
    }
    let key = e.call(EFFECT_KEY_00403EA0, &args![effect]).u32();
    let found = fn_007e0ce0(e, this, key);
    if found != 0 {
        let magnitude = e.call(EFFECT_GET_MAGNITUDE, &args![effect]).u32();
        let total = magnitude.wrapping_add(e.mem.u32(found));
        e.mem.set_u32(found, total);
        return;
    }
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let entry = if block != 0 {
        e.call(ENTRY_CONSTRUCT_00403BA0, &args![block]).u32()
    } else {
        0
    };
    let key = e.call(EFFECT_KEY_00403EA0, &args![effect]).u32();
    e.mem.set_u32(entry + 0x10, key);
    let magnitude = e.call(EFFECT_GET_MAGNITUDE, &args![effect]).u32();
    e.mem.set_u32(entry, magnitude);
    let setting = e.call(WORD_AT_14_00825C00, &args![effect]).u32();
    if e.call(FLAG_TEST_00403E60, &args![setting, 4u32]).bool() {
        let negated = e.mem.u32(entry).wrapping_mul(0xffff_ffff);
        e.mem.set_u32(entry, negated);
    }
    let setting = e.call(WORD_AT_14_00825C00, &args![effect]).u32();
    if !e.call(FLAG_TEST_00403E60, &args![setting, 2u32]).bool() {
        let duration = e.call(EFFECT_GET_DURATION, &args![effect]).u32();
        e.mem.set_u32(entry + 8, duration);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_APPEND, &args![this.addr() + 4, slot]);
    });
}

// Translated from 007e0ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a status data list, `RET 4`, a key): the first entry whose +0x10
/// is the key, or 0 (the walk advances once more past a match before it stops).
pub fn fn_007e0ce0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut found = 0u32;
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while found == 0 && node != 0 && node_item(e, node) != 0 {
        let entry = node_item(e, node);
        if e.mem.u32(entry + 0x10) == key {
            found = node_item(e, node);
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 007e0e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a list node, `RET 8`: a callback and a word): calls the callback
/// (`cdecl`, the node's item and the word); while it answers true, goes on with the next
/// node (+4) until there is none.
pub fn fn_007e0e30(e: &mut Engine, this: Ptr, callback: u32, value: u32) {
    let item = e.mem.u32(this.addr());
    if e.call(callback, &args![item, value]).bool() && e.mem.u32(this.addr() + 4) != 0 {
        let next = e.mem.u32(this.addr() + 4);
        fn_007e0e30(e, Ptr::new(next), callback, value);
    }
}

/// `ListBox<T>::AddItem` as the `levelupmenu.cpp` instance does it (the two instances here
/// match it instruction for instruction), with the instance's `Synchronize` and
/// `SetTotalListHeight`.
fn list_box_add_item(
    e: &mut Engine,
    this: Ptr,
    (data, text, compare, template): (u32, u32, u32, u32),
    (synchronize, set_total_list_height): (u32, u32),
) -> u32 {
    let base = this.addr();
    let container = e.mem.u32(base + 0xc);
    if container == 0 {
        return 0;
    }
    if e.mem.u32(base + 0x18) == 0 && template == 0 {
        return 0;
    }
    let name = if template != 0 {
        template
    } else {
        e.mem.u32(base + 0x18)
    };
    let container = e.mem.u32(base + 0xc);
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let container = e.mem.u32(base + 0xc);
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, container, name, 0u32])
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        set_int(e, tile, 0xfaa, 0xffff_ffff);
    }
    if text != 0 {
        set_string(e, tile, 0xfc4, text);
    }
    let item = e.with_stack(4, |e, data_slot| {
        e.mem.set_u32(data_slot.addr(), data);
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        if block != 0 {
            e.call(ITEM_CONSTRUCT, &args![block, tile, data_slot]).u32()
        } else {
            0
        }
    });
    if compare != 0 {
        e.call(LIST_INSERT_SORTED, &args![base + 4, item, compare]);
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(synchronize, &args![this]);
        }
    } else {
        e.with_stack(4, |e, item_slot| {
            e.mem.set_u32(item_slot.addr(), item);
            e.call(LIST_APPEND_00905820, &args![base + 4, item_slot]);
        });
        if e.call(LIST_BOX_HAS_FLAG, &args![this, 0u32]).bool() {
            e.call(INIT_TILE_POSITION, &args![this, tile]);
            e.call(set_total_list_height, &args![this]);
        }
        let index = e.mem.u16(base + 0x1c);
        e.mem.set_u16(base + 0x1c, index.wrapping_add(1));
        set_int(e, tile, 0xfac, u32::from(index));
    }
    if e.call(LIST_BOX_COUNT_00726C40, &args![this]).u32() == 1 {
        let visible = e
            .call(
                ADD_USER_TRAIT,
                &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
            )
            .u32();
        let container = e.mem.u32(base + 0xc);
        let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if above(count, zero) {
            let container = e.mem.u32(base + 0xc);
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let visible = e
                .call(
                    ADD_USER_TRAIT,
                    &args![NUMBER_OF_VISIBLE_ITEMS_TEXT, 0xffff_ffffu32],
                )
                .u32();
            let container = e.mem.u32(base + 0xc);
            let count = e.call(TILE_GET_FLOAT, &args![container, visible]).f64();
            e.call(
                VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, x87_store32(count)],
            );
            e.call(VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

// Translated from 007e0e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e0e70 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<PerkRankData_P>::AddItem` (unnamed in the engine map; `RET 0x10`; its
/// `Synchronize` `00724930` and `SetTotalListHeight` `00724dd0` are in `menumanager.cpp`):
/// as `ListBox<BGSPerk_P>::AddItem` of `levelupmenu.cpp`. Returns the new tile, or 0
/// without a container or template.
pub fn fn_007e0e70(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(
        e,
        this,
        (data, text, compare, template),
        (PERK_RANK_SYNCHRONIZE, PERK_RANK_SET_TOTAL_LIST_HEIGHT),
    )
}

// Translated from 007e1110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a list box, `RET 4`, a tile): the data of the item whose tile it
/// is, or 0 (also for a null tile).
pub fn fn_007e1110(e: &mut Engine, this: Ptr, tile: u32) -> u32 {
    if tile == 0 {
        return 0;
    }
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item) == tile {
            let item = node_item(e, node);
            return e.mem.u32(item + 4);
        }
        node = node_next(e, node);
    }
    0
}

// Translated from 007e1190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e1190 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<enum_MiscStatManager::MiscStatID>::AddItem` (engine map; `RET 0x10`; its
/// `Synchronize` `00724f00` and `SetTotalListHeight` `007253a0` are in `menumanager.cpp`):
/// as `ListBox<BGSPerk_P>::AddItem` of `levelupmenu.cpp`.
pub fn list_box_misc_stat_id_add_item(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(
        e,
        this,
        (data, text, compare, template),
        (MISC_STAT_SYNCHRONIZE, MISC_STAT_SET_TOTAL_LIST_HEIGHT),
    )
}

/// `StatsMenu::Create`'s callees and data: the menu class (`004d5e00`, no arguments),
/// `Tile::GetMenuByClass` (`00a09030`, `cdecl`), `Tile::GetMaximumDepth` (`00a01530`,
/// `ST0`), the interface manager's +0x9c (`00586150`), `Tile::ReadFile` (`00a01b00`, `RET
/// 4`), `Tile::GetMenu` (`00a03c90`), `Menu::RemoveFromXMLArchive` (`00a1ca20`, `cdecl`),
/// `00867a20` (`thiscall` on the menu), `Menu::SetMenuTile` (`00a1dc70`, `RET 8`), the
/// debug message (`005b5e40`), the tutorial (`00718630`, `RET 0xc`), the radiation stage
/// (`00592220`, `cdecl`, a `float`), `00715c60` (`RET 8`), the list initialisation
/// (`0075a0d0`, `RET 8`), the actor value sections (`0066ecf0`, `0066ec10`, `cdecl`), the
/// actor value flags (`00406d70`, `cdecl`), the actor value list's `AddItem` (`00754690`,
/// `RET 0x10`), the actor value names (`0066e950`, `cdecl`), the tile names (`00707e10`,
/// `RET 4`), the misc stat names (`0078a260`, `cdecl`), the default objects (`0058db10`,
/// `cdecl`), `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words), the meter text
/// (`007b52a0`, `thiscall` on the string) and `Menu::PrepForVisibility` (`00a1dc20`, `RET
/// 4`).
const MENU_CLASS_004D5E00: u32 = 0x004d_5e00;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const WORD_AT_9C_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
const MENU_00867A20: u32 = 0x0086_7a20;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
const FIND_STAGE_FOR_RAD_LEVEL: u32 = 0x0059_2220;
const WORD_AT_18_009611E0: u32 = 0x0096_11e0;
const TILE_SET_INTEGER_VALUE: u32 = 0x0071_5c60;
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
const AV_SECTION_COUNT: u32 = 0x0066_ecf0;
const AV_TO_ACTOR_VALUE: u32 = 0x0066_ec10;
const AV_FLAG_TEST_00406D70: u32 = 0x0040_6d70;
const AV_LIST_BOX_ADD_ITEM: u32 = 0x0075_4690;
const AV_NAME: u32 = 0x0066_e950;
const TILE_SET_NAME_00707E10: u32 = 0x0070_7e10;
const MISC_STAT_NAME_0078A260: u32 = 0x0078_a260;
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const METER_LABEL_007B52A0: u32 = 0x007b_52a0;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The RTTI type descriptors the healing options' default objects are cast with.
const TYPE_FORM_01183028: u32 = 0x0118_3028;
const TYPE_ITEM_011830CC: u32 = 0x0118_30cc;
const TYPE_NAMED_01183158: u32 = 0x0118_3158;
/// Create's strings: the file, the failure message, the list templates, the black image,
/// the meter traits and `"_Title"`.
const MENU_FILE_TEXT: u32 = 0x0107_730c;
const CREATION_FAILED_TEXT: u32 = 0x0107_72b8;
const LIST_TEMPLATE_TEXT: u32 = 0x0107_72a0;
const SOLID_BLACK_TEXT: u32 = 0x0107_727c;
const EFFECTS_TEMPLATE_TEXT: u32 = 0x0107_7264;
const TITLE_TEXT: u32 = 0x0106_ed20;
/// `6006.0` and `102.0` (`double`s) and the largest `float` (`01016970`).
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
const FLOAT_MAX_01016970: u32 = 0x0101_6970;

// Translated from 007da2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007da2c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// `StatsMenu::Create` (Xbox PDB; no arguments). An open stats menu is deleted first. The
/// menu is read from `stats_menu.xml` (the open menu at `011dace0`, dropped from the XML
/// archive, `00867a20`); a missing menu or one of another class is deleted (when it has a
/// tile) and 0 returned; without all its tiles a message is logged and 0 returned.
/// Otherwise the menu is set up in the exe's order: the depth trait, the tutorial message,
/// the player's name, the radiation stage limit (`0x1006`, 1000 without a stage), the
/// health, action point and experience pairs (`0x1009`, `0x100a`, `0x100b`) and the level
/// (`0x100c`); the SPECIAL and skill lists with their actor values (no hidden ones nor
/// 0x1f), sorted by `007da270`; the perks (`UpdatePerks`), reputation and general lists
/// with the 0x2b misc stats; the black image; the four healing options from the default
/// objects 0, 0x15, 2, 3 (their tiles, names and callbacks `007e05f0`, `007e06f0`,
/// `007e07a0`); the effects list; the fade groups (`007db250`, `007db270`); the actor
/// values and the healing mode; the four meter names (`_rads`, `_h20`, `_hunger`, `_sleep`)
/// and the page and title strings from the game settings. Returns the menu's tile.
pub fn stats_menu_create(e: &mut Engine) -> u32 {
    let class = e.call(MENU_CLASS_004D5E00, &args![]).u32();
    let existing = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    let depth = x87_store32(e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64());
    let root = e.call(WORD_AT_9C_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(OPEN_MENU_011DACE0, menu);
    e.call(
        MENU_REMOVE_FROM_XML_ARCHIVE,
        &args![MENU_FILE_TEXT, open_menu(e)],
    );
    e.call(MENU_00867A20, &args![open_menu(e)]);
    let fail = |e: &mut Engine| {
        let menu = open_menu(e);
        if menu != 0 && e.call(WORD_AT_4_00726070, &args![menu]).u32() != 0 {
            let menu = open_menu(e);
            if menu != 0 {
                e.vcall(menu, 0, &args![1u32]);
            }
        }
        0
    };
    if open_menu(e) == 0 {
        return fail(e);
    }
    let menu_class = e.vcall(open_menu(e), 0x34, &args![]).u32();
    if menu_class != e.call(MENU_CLASS_004D5E00, &args![]).u32() {
        return fail(e);
    }
    e.call(MENU_SET_MENU_TILE, &args![open_menu(e), tile, 0u32]);
    if !fn_007db340(e, Ptr::new(open_menu(e))) {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    let first: f64 = e.global(VALUE_6006_DOUBLE);
    let second: f64 = e.global(VALUE_102_DOUBLE);
    if e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64() == first
        || e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64() == second
    {
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfadu32, x87_quiet_bits(depth), 1u32],
        );
    }
    let class = e.call(MENU_CLASS_004D5E00, &args![]).u32();
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![interface.wrapping_add(0x4d4), 0x19u32, class, 0x200u32],
    );
    stats_menu_update_player_name(e);
    let limit = x87_quiet_bits(e.global::<u32>(FLOAT_MAX_01016970));
    let stage = e.call(FIND_STAGE_FOR_RAD_LEVEL, &args![limit]).u32();
    let stage_limit = if stage != 0 {
        e.call(WORD_AT_18_009611E0, &args![stage]).u32()
    } else {
        1000
    };
    e.call(TILE_SET_INTEGER_VALUE, &args![tile, 0x1006u32, stage_limit]);
    e.with_stack(0xc, |e, frame| {
        let text = frame.addr();
        let index_slot = text + 8;
        e.call(STRING_CONSTRUCT, &args![text]);
        for (value, trait_id) in [(0x10u32, 0x1009u32), (0xc, 0x100a)] {
            let base = e.vcall(actor_values(e), AV_BASE, &args![value]).u32();
            let current = e.vcall(actor_values(e), AV_VALUE, &args![value]).u32();
            e.call(
                STRING_FORMAT,
                &args![text, VALUE_PAIR_FORMAT, current, base],
            );
            let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
            set_string(e, tile, trait_id, string);
        }
        let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
        let cap = e
            .call(LEVEL_CAP_0043D4D0, &args![LEVEL_CAP_OWNER_011D0C60])
            .u32();
        if (level as i32) < e.mem.u32(cap) as i32 {
            let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
            let required = e.call(REQUIRED_EXPERIENCE, &args![level + 1]).u32();
            let current = e.vcall(actor_values(e), AV_VALUE, &args![0x18u32]).u32();
            e.call(
                STRING_FORMAT,
                &args![text, VALUE_PAIR_FORMAT, current, required],
            );
        } else {
            let string = e
                .call(SETTING_STRING_00403DF0, &args![MAX_LEVEL_SETTING_011D472C])
                .u32();
            e.call(STRING_FORMAT, &args![text, ONE_STRING_FORMAT, string]);
        }
        let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
        set_string(e, tile, 0x100b, string);
        let level = u32::from(e.call(PLAYER_LEVEL_0087F9F0, &args![player(e)]).u16());
        set_int(e, tile, 0x100c, level);
        e.call(STRING_ASSIGN_00438390, &args![text, LIST_TEMPLATE_TEXT]);
        for (section, list, container) in [(0u32, 0x180u32, 0xa8u32), (2, 0x1b0, 0xac)] {
            let template = e.call(STRING_CHARS_00559450, &args![text]).u32();
            let container = e.mem.u32(open_menu(e) + container);
            e.call(
                LIST_BOX_INITIALIZE,
                &args![open_menu(e) + list, container, template],
            );
            let mut index = 0u32;
            while index < e.call(AV_SECTION_COUNT, &args![section]).u32() {
                let value = e
                    .call(AV_TO_ACTOR_VALUE, &args![section, index & 0xff])
                    .u32();
                if !e
                    .call(AV_FLAG_TEST_00406D70, &args![value, 0x2000u32])
                    .bool()
                    && value != 0x1f
                {
                    let item = e
                        .call(
                            AV_LIST_BOX_ADD_ITEM,
                            &args![open_menu(e) + list, value, 0u32, 0u32, 0u32],
                        )
                        .u32();
                    let name = e.call(AV_NAME, &args![value]).u32();
                    set_string(e, item, 0xfc4, name);
                    let name = e.call(AV_NAME, &args![value]).u32();
                    e.call(TILE_SET_NAME_00707E10, &args![item, name]);
                }
                index = index.wrapping_add(1);
            }
        }
        let skills = open_menu(e) + 0x1b0;
        e.vcall(skills, 0x20, &args![0x007d_a270u32]);
        let template = e.call(STRING_CHARS_00559450, &args![text]).u32();
        let container = e.mem.u32(open_menu(e) + 0x118);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![open_menu(e) + 0x1e0, container, template],
        );
        stats_menu_update_perks(e);
        for (list, container) in [(0x270u32, 0x148u32), (0x210, 0xb0)] {
            let template = e.call(STRING_CHARS_00559450, &args![text]).u32();
            let container = e.mem.u32(open_menu(e) + container);
            e.call(
                LIST_BOX_INITIALIZE,
                &args![open_menu(e) + list, container, template],
            );
        }
        e.mem.set_u32(index_slot, 0);
        while (e.mem.u32(index_slot) as i32) < 0x2b {
            let stat = e.mem.u32(index_slot);
            let item =
                list_box_misc_stat_id_add_item(e, Ptr::new(open_menu(e) + 0x210), stat, 0, 0, 0);
            let stat = e.mem.u32(index_slot);
            let name = e.call(MISC_STAT_NAME_0078A260, &args![stat]).u32();
            set_string(e, item, 0xfc4, name);
            let stat = e.mem.u32(index_slot);
            let value = e.call(MISC_STAT_VALUE_005A3370, &args![stat]).u32();
            set_int(e, item, 0x1005, value);
            e.call(TILE_SET_NAME_00707E10, &args![item, name]);
            fn_007dc0a0(e, Ptr::new(index_slot));
        }
        let black = e.mem.u32(open_menu(e) + 0x14c);
        set_string(e, black, 0xfcc, SOLID_BLACK_TEXT);
        let black = e.mem.u32(open_menu(e) + 0x14c);
        set_int(e, black, 0xfa3, 0);
        // The healing options: the default object, the record, the tile it takes and the
        // availability callback (none for the third).
        for (default, record, tile_offset, callback) in [
            (0u32, 0x28u32, 0xb8u32, Some(0x007e_05f0u32)),
            (0x15, 0x58, 0x17c, Some(0x007e_06f0)),
            (2, 0x38, 0x10c, None),
            (3, 0x48, 0xbc, Some(0x007e_07a0)),
        ] {
            let object = e.call(GET_DEFAULT_OBJECT, &args![default]).u32();
            let item = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![object, 0u32, TYPE_FORM_01183028, TYPE_ITEM_011830CC, 0u32],
                )
                .u32();
            e.mem.set_u32(open_menu(e) + record, item);
            let option_tile = e.mem.u32(open_menu(e) + tile_offset);
            e.mem.set_u32(open_menu(e) + record + 4, option_tile);
            let named = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![object, 0u32, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0u32],
                )
                .u32();
            let name = e.call(STRING_CHARS_00408DA0, &args![named]).u32();
            let option_tile = e.mem.u32(open_menu(e) + record + 4);
            set_string(e, option_tile, 0x1004, name);
            if let Some(callback) = callback {
                e.mem.set_u32(open_menu(e) + record + 0xc, callback);
            }
        }
        let container = e.mem.u32(open_menu(e) + 0x114);
        e.call(
            LIST_BOX_INITIALIZE,
            &args![open_menu(e) + 0x240, container, EFFECTS_TEMPLATE_TEXT],
        );
        for (group, offset) in [
            (0u32, 0xc0u32),
            (0, 0xc4),
            (0, 0xf0),
            (1, 0xd8),
            (1, 0xdc),
            (2, 0xc8),
            (2, 0xcc),
            (3, 0xd0),
            (3, 0xd4),
            (4, 0xe8),
            (4, 0xec),
            (5, 0xe0),
            (5, 0xe4),
        ] {
            let group_tile = e.mem.u32(open_menu(e) + offset);
            let fade = fn_007db1c0(e);
            fn_007db250(e, Ptr::new(fade), group, group_tile);
        }
        for (group, index, value) in [
            (0u32, 1u32, 2u32),
            (0, 2, 1),
            (0, 3, 3),
            (2, 0, 0),
            (2, 1, 4),
            (2, 2, 1),
            (2, 3, 3),
            (1, 0, 0),
            (1, 1, 4),
            (1, 3, 2),
            (3, 0, 0),
            (3, 1, 5),
            (3, 2, 2),
            (4, 0, 2),
            (4, 3, 5),
            (5, 0, 2),
            (5, 2, 4),
        ] {
            let fade = fn_007db1c0(e);
            fn_007db270(e, Ptr::new(fade), group, index, value);
        }
        stats_menu_update_all_actor_values(e, Ptr::new(open_menu(e)));
        stats_menu_update_healing_mode(e, Ptr::new(open_menu(e)));
        for (setting, name) in [
            (0x011d_30c0u32, 0x0107_725cu32),
            (0x011d_1f08, 0x0107_7254),
            (0x011d_37c8, 0x0107_724c),
            (0x011d_246c, 0x0107_7244),
        ] {
            let value = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
            e.call(STRING_ASSIGN_00438390, &args![text, value]);
            e.call(METER_LABEL_007B52A0, &args![text]);
            let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
            let trait_id = e.call(ADD_USER_TRAIT, &args![name, 0xffff_ffffu32]).u32();
            e.call(TILE_SET_STRING, &args![tile, trait_id, string, 1u32]);
        }
        let value = e
            .call(SETTING_STRING_00403DF0, &args![0x011d_3fdcu32])
            .u32();
        let target = e.mem.u32(open_menu(e) + 0x12c);
        set_string(e, target, 0xfc4, value);
        for (setting, offset) in [
            (0x011d_1f5cu32, 0x130u32),
            (0x011d_50b0, 0x134),
            (0x011d_44a4, 0x138),
            (0x011d_3bd4, 0x13c),
        ] {
            let value = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
            let trait_id = e
                .call(ADD_USER_TRAIT, &args![TITLE_TEXT, 0xffff_ffffu32])
                .u32();
            let target = e.mem.u32(open_menu(e) + offset);
            e.call(TILE_SET_STRING, &args![target, trait_id, value, 1u32]);
        }
        for (setting, target, trait_id) in [
            (0x011d_48d0u32, Some(0x100u32), 0xfc4u32),
            (0x011d_3e74, Some(0x104), 0xfc4),
            (0x011d_2e80, None, 0x100d),
            (0x011d_41bc, None, 0x100e),
            (0x011d_47ec, Some(0x140), 0xfc4),
            (0x011d_2ebc, Some(0x144), 0xfc4),
            (0x011d_2f88, Some(0x90), 0xfc4),
            (0x011d_2994, Some(0x94), 0xfc4),
            (0x011d_3b08, Some(0x98), 0xfc4),
            (0x011d_4bd0, Some(0x9c), 0xfc4),
            (0x011d_4ef4, Some(0xa0), 0xfc4),
        ] {
            let value = e.call(SETTING_STRING_00403DF0, &args![setting]).u32();
            let target = match target {
                Some(offset) => e.mem.u32(open_menu(e) + offset),
                None => tile,
            };
            set_string(e, target, trait_id, value);
        }
        e.call(MENU_PREP_FOR_VISIBILITY, &args![open_menu(e), 1u32]);
        e.call(STRING_DESTROY, &args![text]);
    });
    tile
}

/// `UpdateActiveEffects`'s callees: the meters' stages (`00592220` radiation, `004018b0`,
/// `00402760`, `00402030`; `cdecl`, a `float`), `0084e3a0` (an active effect's effect
/// item), `EffectItem::IsUsageMonitorEffect` (`00404730`), `0044ddc0` (the effect's
/// target form), `PathingLocation::GetWorldspace` (`00441110`, `thiscall` on a stage), the
/// stage effect constructor (`007df140`, `RET 4`) and its effect list's add (`007e0a20`,
/// `RET 4`), `00403ee0` (`thiscall`, a `bool`), `ActorValue::GetActorValueAbbreviation`
/// (`0066eb00`, `cdecl`), `StatusEffectList::Register` (`007e08a0`, `RET 4`),
/// `Tile::DeleteChildren` (`00a04150`), `Menu::RenderTemplate` (`00a1ddb0`, `RET 0xc`), the
/// string append (`00404820`, `RET 4`), `007df120` (`RET 4`: names a tile after a string),
/// the status effect list's `AddItem` (`007e1430`, `RET 0x10`), `004046b0` (`RET 4`: an
/// entry's text into a string), the string append of a string (`007e0d80`, `RET 4`), the
/// string length (`004048e0`), `VATS::GetCount` (`005ae380`), the item count
/// (`00726c40`), `0042f5a0` (`cdecl`, two words: the smaller) and the status effect list's
/// `Synchronize` (`007254d0`).
const DEHYDRATION_STAGE_004018B0: u32 = 0x0040_18b0;
const SLEEP_STAGE_00402760: u32 = 0x0040_2760;
const HUNGER_STAGE_00402030: u32 = 0x0040_2030;
const EFFECT_ITEM_0084E3A0: u32 = 0x0084_e3a0;
const IS_USAGE_MONITOR_EFFECT: u32 = 0x0040_4730;
const EFFECT_TARGET_0044DDC0: u32 = 0x0044_ddc0;
const STAGE_GET_WORLDSPACE: u32 = 0x0044_1110;
const STAGE_EFFECT_CONSTRUCT_007DF140: u32 = 0x007d_f140;
const STAGE_EFFECT_ADD_007E0A20: u32 = 0x007e_0a20;
const EFFECT_FLAG_00403EE0: u32 = 0x0040_3ee0;
const AV_ABBREVIATION: u32 = 0x0066_eb00;
const STATUS_EFFECT_LIST_REGISTER: u32 = 0x007e_08a0;
const TILE_DELETE_CHILDREN: u32 = 0x00a0_4150;
const STRING_APPEND_00404820: u32 = 0x0040_4820;
const TILE_NAME_FROM_STRING_007DF120: u32 = 0x007d_f120;
const STATUS_EFFECT_ADD_ITEM: u32 = 0x007e_1430;
const ENTRY_STRING_004046B0: u32 = 0x0040_46b0;
const STRING_APPEND_STRING_007E0D80: u32 = 0x007e_0d80;
const STRING_LENGTH_004048E0: u32 = 0x0040_48e0;
const VATS_GET_COUNT: u32 = 0x005a_e380;
const MIN_0042F5A0: u32 = 0x0042_f5a0;
const STATUS_EFFECT_SYNCHRONIZE: u32 = 0x0072_54d0;
/// The RTTI type descriptors of the effect's target cast (`01183140` to `01183060`).
const TYPE_TARGET_01183140: u32 = 0x0118_3140;
const TYPE_WORLD_01183060: u32 = 0x0118_3060;
/// `UpdateActiveEffects`'s strings: the stage template, the four tile name prefixes,
/// `"%+d"`, `", "`, `"%s %+d"`; and the game setting the status effects of form kind 0xd
/// are named by.
const STAGE_TEMPLATE_TEXT: u32 = 0x0107_752c;
const SIGNED_FORMAT: u32 = 0x0107_7514;
const SEPARATOR_TEXT: u32 = 0x0101_2630;
const ABBREVIATION_FORMAT: u32 = 0x0107_74cc;
const KIND_D_SETTING_011D1EF0: u32 = 0x011d_1ef0;

// Translated from 007ddf00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ddf00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the string locals and the stage effect allocations) is not translated.
/// `StatsMenu::UpdateActiveEffects` (Xbox PDB), once per frame
/// (`bActiveEffectsUpdatedThisFrame`). The status effect list (+0x84) is emptied and the
/// four stage effects (+0x74 radiation, +0x78, +0x80, +0x7c) deleted. The current stages
/// of radiation (actor value 0x36), 0x49, 0x4b and 0x4a are found. Every active effect of
/// the player that is not a usage monitor (and, when its setting has `007df1b0`, has the
/// setting flag 0x2000) is matched against the four stages' world spaces: a match makes
/// the stage effect on first use (`007df140`) and adds the effect to it (`007e0a20`). An
/// effect with an actor value abbreviation and a magnitude (`007df1f0`) other than 0.0, or
/// of kind 1, 6, 0xa, or 4 with `007df210`, is registered with the status effect list;
/// otherwise only with the setting flag 0x2000.
///
/// Each stage effect's tile (+0x110, +0x168, +0x170, +0x178) gets one rendered template per
/// entry (index `0xfac`, its name from the prefix and the actor value's abbreviation,
/// `"%+d"` of the entry's value and the abbreviation as `0x1005`) and is shown (`0xfa3`),
/// or hidden without a stage effect. The effects list (+0x240) is rebuilt with one item
/// per status effect: its name (`0x1004`: a setting for form kind 0xd, the named form's
/// name, or the form's slot +0x130) and its description (`0x1005`: the entries' texts,
/// then `"%s %+d"` of each entry with a value, separated by `", "`). The list's container
/// (+0x114) height is the last item's height times `min(8, count)` (0 without items), and
/// the list is synchronized.
pub fn stats_menu_update_active_effects(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x2a1) != 0 {
        return;
    }
    e.mem.set_u8(base + 0x2a1, 1);
    e.with_stack(0x20, |e, frame| {
        let text = frame.addr();
        let description = text + 8;
        let piece = text + 0x10;
        let second_piece = text + 0x18;
        e.call(STRING_CONSTRUCT, &args![text]);
        fn_007e07f0(e, Ptr::new(base + 0x84));
        for slot in [0x74u32, 0x78, 0x80, 0x7c] {
            let effect = e.mem.u32(base + slot);
            if effect != 0 {
                e.call(STATUS_EFFECT_DELETE, &args![effect, 1u32]);
            }
            e.mem.set_u32(base + slot, 0);
        }
        let mut stages = [0u32; 4];
        for (index, (value, finder)) in [
            (0x36u32, FIND_STAGE_FOR_RAD_LEVEL),
            (0x49, DEHYDRATION_STAGE_004018B0),
            (0x4b, SLEEP_STAGE_00402760),
            (0x4a, HUNGER_STAGE_00402030),
        ]
        .into_iter()
        .enumerate()
        {
            let level = x87_store32(e.vcall(actor_values(e), AV_FLOAT, &args![value]).f64());
            stages[index] = e.call(finder, &args![level]).u32();
        }
        let [radiation, dehydration, sleep, hunger] = stages;
        let effects = player(e).wrapping_add(0x94);
        let mut node = e.vcall(effects, 8, &args![]).u32();
        while node != 0 {
            let effect = node_item(e, node);
            if effect == 0 {
                break;
            }
            if e.call(EFFECT_ACTIVE_00833C20, &args![effect]).bool() {
                let item = e.call(EFFECT_ITEM_0084E3A0, &args![effect]).u32();
                if !e.call(IS_USAGE_MONITOR_EFFECT, &args![item]).bool() {
                    let setting = fn_007df1d0(e, Ptr::new(effect));
                    let skip = fn_007df1b0(e, Ptr::new(setting)) && {
                        let setting = fn_007df1d0(e, Ptr::new(effect));
                        !e.call(FLAG_TEST_00403E60, &args![setting, 0x2000u32])
                            .bool()
                    };
                    if !skip {
                        update_effect(e, this, effect, [radiation, dehydration, hunger, sleep]);
                    }
                }
            }
            node = node_next(e, node);
        }
        for (slot, tile_offset, prefix) in [
            (0x74u32, 0x110u32, 0x0107_7518u32),
            (0x78, 0x168, 0x0107_7500),
            (0x7c, 0x170, 0x0107_74e8),
            (0x80, 0x178, 0x0107_74d4),
        ] {
            if e.mem.u32(base + slot) == 0 {
                let tile = e.mem.u32(base + tile_offset);
                set_int(e, tile, 0xfa3, 0);
                continue;
            }
            let tile = e.mem.u32(base + tile_offset);
            e.call(TILE_DELETE_CHILDREN, &args![tile]);
            let mut index = 0u32;
            let effect = e.mem.u32(base + slot);
            let mut entry_node = if effect.wrapping_add(4) == 0 {
                0
            } else {
                e.mem.u32(base + slot).wrapping_add(8)
            };
            while entry_node != 0 && node_item(e, entry_node) != 0 {
                let parent = e.mem.u32(base + tile_offset);
                let line = e
                    .call(
                        MENU_RENDER_TEMPLATE,
                        &args![this, parent, STAGE_TEMPLATE_TEXT, 0u32],
                    )
                    .u32();
                set_int(e, line, 0xfac, index);
                index = index.wrapping_add(1);
                e.call(STRING_ASSIGN_00438390, &args![text, prefix]);
                let entry = node_item(e, entry_node);
                let abbreviation = e
                    .call(AV_ABBREVIATION, &args![e.mem.u32(entry + 0x10)])
                    .u32();
                e.call(STRING_APPEND_00404820, &args![text, abbreviation]);
                e.call(TILE_NAME_FROM_STRING_007DF120, &args![line, text]);
                let entry = node_item(e, entry_node);
                let value = e.mem.u32(entry);
                e.call(STRING_FORMAT, &args![text, SIGNED_FORMAT, value]);
                let string = e.call(STRING_CHARS_00559450, &args![text]).u32();
                set_string(e, line, 0xfc4, string);
                let entry = node_item(e, entry_node);
                let abbreviation = e
                    .call(AV_ABBREVIATION, &args![e.mem.u32(entry + 0x10)])
                    .u32();
                set_string(e, line, 0x1005, abbreviation);
                entry_node = node_next(e, entry_node);
            }
            let tile = e.mem.u32(base + tile_offset);
            set_int(e, tile, 0xfa3, 1);
        }
        let mut last = 0u32;
        let list = base + 0x240;
        e.vcall(list, 0x1c, &args![]);
        let mut node = if base.wrapping_add(0x84) == 0 {
            0
        } else {
            base + 0x88
        };
        while node != 0 && node_item(e, node) != 0 {
            let status = node_item(e, node);
            last = e
                .call(
                    STATUS_EFFECT_ADD_ITEM,
                    &args![list, status, 0u32, 0u32, 0u32],
                )
                .u32();
            let status = node_item(e, node);
            let form = e.mem.u32(status);
            let item = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_FORM_01183028, TYPE_ITEM_011830CC, 0u32],
                )
                .u32();
            if item != 0
                && e.call(WORD_AT_4_00726070, &args![item.wrapping_add(0x9c)])
                    .u32()
                    == 0xd
            {
                let name = e
                    .call(SETTING_STRING_00403DF0, &args![KIND_D_SETTING_011D1EF0])
                    .u32();
                set_string(e, last, 0x1004, name);
            } else {
                let status = node_item(e, node);
                let form = e.mem.u32(status);
                let named = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0u32],
                    )
                    .u32();
                let name = if named != 0 {
                    e.call(STRING_CHARS_00408DA0, &args![named]).u32()
                } else {
                    let status = node_item(e, node);
                    let form = e.mem.u32(status);
                    e.vcall(form, 0x130, &args![]).u32()
                };
                set_string(e, last, 0x1004, name);
            }
            e.call(STRING_CONSTRUCT, &args![description]);
            let status = node_item(e, node);
            let mut entry_node = if status.wrapping_add(0x10) == 0 {
                0
            } else {
                status.wrapping_add(0x14)
            };
            while entry_node != 0 && node_item(e, entry_node) != 0 {
                let entry = node_item(e, entry_node);
                let result = e.call(ENTRY_STRING_004046B0, &args![entry, piece]).u32();
                let string = e.call(STRING_CHARS_00559450, &args![result]).u32();
                e.call(STRING_DESTROY, &args![piece]);
                if string != 0 {
                    let entry = node_item(e, entry_node);
                    let result = e
                        .call(ENTRY_STRING_004046B0, &args![entry, second_piece])
                        .u32();
                    let string = e.call(STRING_CHARS_00559450, &args![result]).u32();
                    e.call(STRING_FORMAT, &args![text, ONE_STRING_FORMAT, string]);
                    e.call(STRING_DESTROY, &args![second_piece]);
                    e.call(STRING_APPEND_STRING_007E0D80, &args![description, text]);
                    if node_next(e, entry_node) != 0 {
                        e.call(STRING_APPEND_00404820, &args![description, SEPARATOR_TEXT]);
                    }
                }
                entry_node = node_next(e, entry_node);
            }
            if e.call(STRING_LENGTH_004048E0, &args![description]).u32() != 0 {
                let status = node_item(e, node);
                if e.call(VATS_GET_COUNT, &args![status.wrapping_add(8)]).u32() != 0 {
                    e.call(STRING_APPEND_00404820, &args![description, SEPARATOR_TEXT]);
                }
            }
            let status = node_item(e, node);
            let mut entry_node = if status.wrapping_add(4) == 0 {
                0
            } else {
                status.wrapping_add(8)
            };
            while entry_node != 0 && node_item(e, entry_node) != 0 {
                let entry = node_item(e, entry_node);
                if e.mem.u32(entry) != 0 {
                    let entry = node_item(e, entry_node);
                    let value = e.mem.u32(entry);
                    let entry = node_item(e, entry_node);
                    let abbreviation = e
                        .call(AV_ABBREVIATION, &args![e.mem.u32(entry + 0x10)])
                        .u32();
                    e.call(
                        STRING_FORMAT,
                        &args![text, ABBREVIATION_FORMAT, abbreviation, value],
                    );
                    e.call(STRING_APPEND_STRING_007E0D80, &args![description, text]);
                    if node_next(e, entry_node) != 0 {
                        e.call(STRING_APPEND_00404820, &args![description, SEPARATOR_TEXT]);
                    }
                }
                entry_node = node_next(e, entry_node);
            }
            let string = e.call(STRING_CHARS_00559450, &args![description]).u32();
            set_string(e, last, 0x1005, string);
            e.call(STRING_DESTROY, &args![description]);
            node = node_next(e, node);
        }
        let container = e.mem.u32(base + 0x114);
        if last != 0 {
            let count = e.call(LIST_BOX_COUNT_00726C40, &args![list]).u32();
            let shown = e.call(MIN_0042F5A0, &args![8u32, count]).u32();
            let height = e.call(TILE_GET_FLOAT, &args![last, 0xfb0u32]).f64();
            let height = x87_store32(x87_mul(height, f64::from(shown)));
            let container = e.mem.u32(base + 0x114);
            e.call(
                TILE_SET_FLOAT,
                &args![container, 0xfb0u32, x87_quiet_bits(height), 1u32],
            );
            set_int(e, last, 0x1006, 0);
        } else {
            set_int(e, container, 0xfb0, 0);
        }
        e.call(STATUS_EFFECT_SYNCHRONIZE, &args![list]);
        e.call(STRING_DESTROY, &args![text]);
    });
}

/// One active effect of `UpdateActiveEffects`: matched against the four stages (in the
/// order radiation +0x74, +0x78, +0x7c, +0x80) and registered with the status effect list.
fn update_effect(e: &mut Engine, this: Ptr, effect: u32, stages: [u32; 4]) {
    let base = this.addr();
    let target = e.call(EFFECT_TARGET_0044DDC0, &args![effect]).u32();
    let world = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                target,
                0u32,
                TYPE_TARGET_01183140,
                TYPE_WORLD_01183060,
                0u32
            ],
        )
        .u32();
    for (stage, slot) in stages.into_iter().zip([0x74u32, 0x78, 0x7c, 0x80]) {
        if world == 0 || stage == 0 {
            continue;
        }
        if world != e.call(STAGE_GET_WORLDSPACE, &args![stage]).u32() {
            continue;
        }
        if e.mem.u32(base + slot) == 0 {
            let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
            let made = if block != 0 {
                e.call(STAGE_EFFECT_CONSTRUCT_007DF140, &args![block, world])
                    .u32()
            } else {
                0
            };
            e.mem.set_u32(base + slot, made);
        }
        let made = e.mem.u32(base + slot);
        if e.mem.u32(made) == world {
            let made = e.mem.u32(base + slot);
            e.call(
                STAGE_EFFECT_ADD_007E0A20,
                &args![made.wrapping_add(4), effect],
            );
        }
    }
    let item = e.call(EFFECT_ITEM_0084E3A0, &args![effect]).u32();
    if e.call(EFFECT_FLAG_00403EE0, &args![item]).bool() {
        let item = e.call(EFFECT_ITEM_0084E3A0, &args![effect]).u32();
        let key = e.call(EFFECT_KEY_00403EA0, &args![item]).u32();
        if e.call(AV_ABBREVIATION, &args![key]).u32() != 0 {
            let magnitude = fn_007df1f0(e, Ptr::new(effect));
            let zero: f64 = e.global(ZERO_DOUBLE);
            // `TEST AH,0x44; JP`: a magnitude other than 0.0 (or NaN) registers it.
            let register = f64::from(magnitude) != zero || {
                let kind = |e: &mut Engine| e.call(EFFECT_KIND_0055B980, &args![effect]).u32();
                kind(e) == 1
                    || kind(e) == 6
                    || kind(e) == 0xa
                    || (kind(e) == 4 && fn_007df210(e, Ptr::new(effect)) as u8 != 0)
            };
            if register {
                e.call(STATUS_EFFECT_LIST_REGISTER, &args![base + 0x84, effect]);
                return;
            }
        }
    }
    let setting = fn_007df1d0(e, Ptr::new(effect));
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x2000u32])
        .bool()
    {
        e.call(STATUS_EFFECT_LIST_REGISTER, &args![base + 0x84, effect]);
    }
}

/// `Tile::IsTrue` (`00a01230`, `RET 4`), `Tile::TextToTrait` (`00a01860`, `cdecl`),
/// `Tile::UpdateTile` (`00a04640`, `RET 4`), `Tile::Poke` (`00a01290`, `RET 8`),
/// `Tile::GetChildByName` (`00a03da0`, `RET 4`), `Tile::PlayTileSound` (`00a0b110`, `RET 4`)
/// and `Tile::IsVisible` (`00a040a0`).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_UPDATE_TILE: u32 = 0x00a0_4640;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
/// A menu sound by name: `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByName`
/// (`00ad7550`, `RET 0xc`, the handle returned through its first word),
/// `BSSoundHandle::Play` (`00ad8830`, `RET 4`) and the handle's destructor (`00483710`);
/// the two sounds this unit plays (`01074f40`, `0106f380`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
const SOUND_01074F40: u32 = 0x0107_4f40;
const SOUND_0106F380: u32 = 0x0106_f380;
/// `InterfaceManager::PlayMenuSound` (`00717280`, `cdecl`), `Interface::SetMapMenuVisible`
/// (`00704170`, `cdecl`, two words) and `Interface::SetInventoryMenuVisible` (`007048f0`,
/// `cdecl`, three words).
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
const SET_MAP_MENU_VISIBLE: u32 = 0x0070_4170;
const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
/// `FOPipboyManager::SetTabKnobPosition` (`007fa0f0`, `RET 8`) and
/// `FOPipboyManager::MoveKnob` (`007f8610`, `RET 0x10`), with the two game settings the knob
/// moves by.
const PIPBOY_SET_TAB_KNOB_POSITION: u32 = 0x007f_a0f0;
const PIPBOY_MOVE_KNOB: u32 = 0x007f_8610;
const KNOB_SETTING_011DB358: u32 = 0x011d_b358;
const KNOB_SETTING_011DB2F0: u32 = 0x011d_b2f0;
/// The words `MoveKnob` keeps each page's last knob step in (`011dad60` to `011dad70`).
const KNOB_STEP_011DAD60: u32 = 0x011d_ad60;
const KNOB_STEP_011DAD64: u32 = 0x011d_ad64;
const KNOB_STEP_011DAD68: u32 = 0x011d_ad68;
const KNOB_STEP_011DAD6C: u32 = 0x011d_ad6c;
const KNOB_STEP_011DAD70: u32 = 0x011d_ad70;
/// `005f65d0` (`thiscall` on a list, `RET 4`: whether it holds the pointed-to word),
/// `00501c10` (`thiscall`, `RET 4`), `00406db0` and `0071f540` (`thiscall` constructors of
/// the stage entry's two lists), `0070ed80` (no arguments, `AL`), `00495460` (`thiscall`, an
/// effect's magnitude in `ST0`).
const LIST_CONTAINS_005F65D0: u32 = 0x005f_65d0;
const FIELD_SET_00501C10: u32 = 0x0050_1c10;
const LIST_CONSTRUCT_00406DB0: u32 = 0x0040_6db0;
const LIST_CONSTRUCT_0071F540: u32 = 0x0071_f540;
const FLAG_0070ED80: u32 = 0x0070_ed80;
const MAGNITUDE_00495460: u32 = 0x0049_5460;
/// The reputation's icon (`006166e0`) and `TESReputation::GetReputationTitle` (`00616710`),
/// both `thiscall`; `006155c0` (`thiscall`, `RET 4`, a value in `ST0`).
const REPUTATION_ICON_006166E0: u32 = 0x0061_66e0;
const REPUTATION_GET_TITLE: u32 = 0x0061_6710;
const REPUTATION_VALUE_006155C0: u32 = 0x0061_55c0;
/// The reputation list (`009c1a50`, `thiscall` on the object at `011c3f2c`).
const REPUTATION_LIST_009C1A50: u32 = 0x009c_1a50;
const REPUTATIONS_011C3F2C: u32 = 0x011c_3f2c;
/// `ActorValue::GetActorValueIcon` (`0066e9f0`), `ActorValue::GetActorValueDesc` (`0066e9b0`)
/// and `ActorValue::GetItemWeaponTypeBadge` (`0066ea20`), all `cdecl`.
const AV_GET_ICON: u32 = 0x0066_e9f0;
const AV_GET_DESCRIPTION: u32 = 0x0066_e9b0;
const AV_GET_BADGE: u32 = 0x0066_ea20;
/// `0073ac90` and `007b5370` (`thiscall` on the description box at +0xf8, plus 4: the slots
/// of its text tile and its scroll tile), `004e4430` (`cdecl`, a `float`, `ST0`).
const TEXT_TILE_SLOT_0073AC90: u32 = 0x0073_ac90;
const SCROLL_TILE_SLOT_007B5370: u32 = 0x007b_5370;
const SCROLL_VALUE_004E4430: u32 = 0x004e_4430;
/// The trait names the description scroll uses.
const SCROLL_TRAIT_NAME_01074EB8: u32 = 0x0107_4eb8;
const SCROLL_TRAIT_NAME_0106F8F8: u32 = 0x0106_f8f8;
const SCROLL_TRAIT_NAME_010702D8: u32 = 0x0107_02d8;
const SCROLL_TRAIT_NAME_01070250: u32 = 0x0107_0250;
/// `00408b20` (`cdecl`, two strings), the setting it reads (`011c3cb4`) and the text it is
/// compared with (`01018348`).
const COMPARE_00408B20: u32 = 0x0040_8b20;
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
const COMPARED_STRING_01018348: u32 = 0x0101_8348;
/// `Actor::QueueEquipObject` (`0088c650`, `RET 0x18`) and the RTTI types the healing item's
/// effect list is cast with.
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
const TYPE_01183108: u32 = 0x0118_3108;
const TYPE_01183194: u32 = 0x0118_3194;
/// `00709d40` (no arguments, `AL`), the input object (`011dea0c`, `00877720`), its stick
/// value (`00a23390`, `RET 8`), `00408820` (`cdecl`, a `float`, `ST0`), the dead zone
/// (`010745f8`, a `double`) and the time the scroll repeats at (`011dad5c`).
const FLAG_00709D40: u32 = 0x0070_9d40;
const INPUT_011DEA0C: u32 = 0x011d_ea0c;
const INPUT_00877720: u32 = 0x0087_7720;
const STICK_00A23390: u32 = 0x00a2_3390;
const ABS_00408820: u32 = 0x0040_8820;
const DEAD_ZONE_DOUBLE: u32 = 0x0107_45f8;
const SCROLL_REPEAT_011DAD5C: u32 = 0x011d_ad5c;
/// `ToggleGenRepMode`'s function-local statics (the guard word and the two trait ids), the
/// mode byte (`011a0ae0`), the second trait's name and the two game settings it shows.
const GEN_REP_GUARD_011DAD8C: u32 = 0x011d_ad8c;
const GEN_REP_TRAIT_011DAD88: u32 = 0x011d_ad88;
const GEN_REP_TRAIT_011DAD84: u32 = 0x011d_ad84;
const GEN_REP_TRAIT_NAME_01077564: u32 = 0x0107_7564;
const GEN_REP_MODE_011A0AE0: u32 = 0x011a_0ae0;
const SETTING_011D4EF4: u32 = 0x011d_4ef4;
const SETTING_011D4144: u32 = 0x011d_4144;
/// The `BSStringT` helpers `operator+=` uses: `004039a0` (capacity), `004037f0` (`RET 8`,
/// grow), `004f4280` (`thiscall` on the source: its length as the copy takes it),
/// `00401460` (`cdecl`, three words: the copy), `004039c0` (`RET 4`, the new length) and
/// `BSStringT<char>::Set` (`00438470`, `RET 4`).
const STRING_CAPACITY_004039A0: u32 = 0x0040_39a0;
const STRING_GROW_004037F0: u32 = 0x0040_37f0;
const STRING_COPY_LENGTH_004F4280: u32 = 0x004f_4280;
const COPY_00401460: u32 = 0x0040_1460;
const STRING_SET_LENGTH_004039C0: u32 = 0x0040_39c0;
const STRING_SET: u32 = 0x0043_8470;
/// The status effect and reputation list instances' `Synchronize` and
/// `SetTotalListHeight` (`menumanager.cpp`).
const STATUS_EFFECT_SYNCHRONIZE_007254D0: u32 = 0x0072_54d0;
const STATUS_EFFECT_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_5970;
const REPUTATION_SYNCHRONIZE: u32 = 0x0072_5aa0;
const REPUTATION_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_5f40;
/// `007242f0` (`thiscall` on a list box, `RET 4`: whether a tile is one of its items).
const LIST_BOX_HAS_TILE_007242F0: u32 = 0x0072_42f0;

/// Plays a menu sound by name (`QInstance`, `GetSoundHandleByName(&handle, name, 0x121)`,
/// `Play(0)`, the handle destroyed).
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// The Pip-Boy manager (`InterfaceManager` +0x174).
fn pipboy_manager(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_MANAGER_004B7210, &args![]).u32();
    e.call(WORD_AT_174_00704370, &args![interface]).u32()
}

/// A game setting's `float`, as `FLD dword` loads it (the value's address from `00403e20`).
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE_00403E20, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

/// A game setting's string (`00403df0`).
fn setting_string(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_STRING_00403DF0, &args![setting]).u32()
}

/// The menu's virtual slot +0xc with an id and 0 (the call the menu makes for a button).
fn menu_click(e: &mut Engine, this: Ptr, id: u32) {
    e.vcall(this.addr(), 0xc, &args![id, 0u32]);
}

/// The fade object (`007db1c0`).
fn fade(e: &mut Engine) -> Ptr {
    Ptr::new(fn_007db1c0(e))
}

/// The tile the description box keeps in a slot (`0073ac90` or `007b5370` on +0xf8, plus 4).
fn description_tile(e: &mut Engine, this: Ptr, slot_fn: u32) -> u32 {
    let box_tile = e.mem.u32(this.addr() + 0xf8);
    let slot = e.call(slot_fn, &args![box_tile.wrapping_add(4)]).u32();
    e.mem.u32(slot)
}

/// The tail `SetIconAndDescription` and `007dc9b0` share: the text tile is updated, the
/// scroll range is its height minus its child's (`0xfb0`), kept at least 0 (`00404010`),
/// divided by the scroll tile's scale and passed through `004e4430`; that plus 1.0 becomes
/// the scroll tile's first trait, the second is set to 0.0 and the third poked with 1.0.
fn update_description_scroll(e: &mut Engine, text_tile: u32, scroll_tile: u32, scale: u32) {
    e.call(TILE_UPDATE_TILE, &args![text_tile, 0u32]);
    let height = e.call(TILE_GET_FLOAT, &args![text_tile, 0xfb0u32]).f64();
    let child = e.call(WORD_AT_28_0045CD60, &args![text_tile]).u32();
    let child_height = e.call(TILE_GET_FLOAT, &args![child, 0xfb0u32]).f64();
    // `FSUBR double [height]`: the height minus the child's.
    let range = x87_store32(x87_sub(height, child_height));
    let range = x87_store32(
        e.call(FLOAT_PAIR_00404010, &args![0.0f32.to_bits(), range])
            .f64(),
    );
    let ratio = x87_store32(x87_div(x87_load32(range), x87_m32(scale)));
    let value = e.call(SCROLL_VALUE_004E4430, &args![ratio]).f64();
    let one: f64 = e.global(ONE_DOUBLE);
    let value = x87_store32(x87_add(value, one));
    let first = e
        .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_0106F8F8])
        .u32();
    e.call(TILE_SET_FLOAT, &args![scroll_tile, first, value, 1u32]);
    let second = e
        .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_010702D8])
        .u32();
    e.call(
        TILE_SET_FLOAT,
        &args![scroll_tile, second, 0.0f32.to_bits(), 0u32],
    );
    let third = e
        .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_01070250])
        .u32();
    e.call(TILE_POKE, &args![scroll_tile, third, 1.0f32.to_bits()]);
}

// Translated from 00704390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::ToggleHealingMode` (Xbox PDB; no arguments): with an open stats menu in
/// limb mode (+0x2a0), `007e0230` on it.
pub fn stats_menu_toggle_healing_mode(e: &mut Engine) {
    let menu = open_menu(e);
    if menu != 0 && e.mem.u8(menu + 0x2a0) != 0 {
        fn_007e0230(e, Ptr::new(menu));
    }
}

// Translated from 007db380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::DoClick` (engine map, vtable; `RET 8`: a tile id and a tile). Without a tile, an id 0 to 0x3b takes `pTiles[id]` (+0x90). When
/// `004b71d0` holds and the tile's `0xfaf` is not true, nothing happens. A page button
/// (`007e00e0`) sets the page; a status page button (`007e0160`) goes to it (`007e0060`); id
/// 9, or 5 in limb mode, toggles limb mode (`007e0230`); 0x31 toggles the general and
/// reputation lists (`007e02b0`). One of the four healing options (records at +0x2c, 16
/// bytes apart: the tile, then the entry at +0x30) whose tile it is and that has an entry
/// queues its item for equipping on the player (`Actor::QueueEquipObject(item form, count,
/// first extra list, 0, 0, 1)`), updates the actor value of each of the item's effects
/// (`007dd090`), the healing options (`007df230`) and the active effects (`007ddf00`, after
/// clearing +0x2a1). Then, in limb mode without `004b71d0`, a tile in a fade group
/// (`007db630`) clicks button 0x3b in hardcore mode or 0xa otherwise (vtable +0xc).
pub fn stats_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x3c).contains(&id) {
        tile = e.mem.u32(base + 0x90 + 4 * id as u32);
    }
    if e.call(FLAG_004B71D0, &args![]).bool()
        && !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool()
    {
        return;
    }
    let page = e.with_stack(4, |e, slot| {
        stats_menu_get_page_for(e, this, id as u32, slot).then(|| e.mem.u32(slot.addr()))
    });
    if let Some(page) = page {
        stats_menu_set_current_page(e, this, page as i32);
        return;
    }
    let status = e.with_stack(4, |e, slot| {
        fn_007e0160(e, this, id as u32, slot).then(|| e.mem.u32(slot.addr()))
    });
    if let Some(status) = status {
        fn_007e0060(e, this, status as i32);
        return;
    }
    if id == 9 || (e.mem.u8(base + 0x2a0) != 0 && id == 5) {
        fn_007e0230(e, this);
        return;
    }
    if id == 0x31 {
        stats_menu_toggle_gen_rep_mode(e, this);
        return;
    }
    let mut index = 0u32;
    while index < 4 {
        let record = base + index * 0x10;
        if tile == e.mem.u32(record + 0x2c) && e.mem.u32(record + 0x30) != 0 {
            let entry = e.mem.u32(record + 0x30);
            let extra_lists = e.call(STRING_CHARS_00559450, &args![entry]).u32();
            let extra_list = if extra_lists != 0 {
                let entry = e.mem.u32(record + 0x30);
                let lists = e.call(STRING_CHARS_00559450, &args![entry]).u32();
                node_item(e, lists)
            } else {
                0
            };
            let entry = e.mem.u32(record + 0x30);
            let count = e.call(WORD_AT_4_00726070, &args![entry]).u32();
            let entry = e.mem.u32(record + 0x30);
            let form = e.call(EFFECT_TARGET_0044DDC0, &args![entry]).u32();
            e.call(
                ACTOR_QUEUE_EQUIP_OBJECT,
                &args![player(e), form, count, extra_list, 0u32, 0u32, 1u32],
            );
            let entry = e.mem.u32(record + 0x30);
            let form = e.call(EFFECT_TARGET_0044DDC0, &args![entry]).u32();
            let mut node = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_01183108, TYPE_01183194, 0u32],
                )
                .u32();
            while node != 0 && node_item(e, node) != 0 {
                let effect = node_item(e, node);
                let value = e.call(EFFECT_KEY_00403EA0, &args![effect]).u32();
                stats_menu_update_actor_value(e, this, value);
                node = node_next(e, node);
            }
            stats_menu_update_healing_options(e, this, index);
            e.mem.set_u8(base + 0x2a1, 0);
            stats_menu_update_active_effects(e, this);
            index = 4;
        }
        index += 1;
    }
    if e.mem.u8(base + 0x2a0) != 0 && !e.call(FLAG_004B71D0, &args![]).bool() {
        let object = fade(e);
        if fn_007db630(e, object, tile) != 6 {
            if e.call(HARDCORE_004D1360, &args![player(e)]).bool() {
                menu_click(e, this, 0x3b);
            } else {
                menu_click(e, this, 0xa);
            }
        }
    }
}

// Translated from 007db630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 4`, a tile): the first of the six groups
/// whose list holds the tile (`005f65d0` given the address of the argument slot), or 6. The
/// count goes up through `007dc0a0`.
pub fn fn_007db630(e: &mut Engine, this: Ptr, tile: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), tile);
        let mut group = 0u32;
        while group < 6 {
            let list = this.addr().wrapping_add(group * 8).wrapping_add(4);
            if e.call(LIST_CONTAINS_005F65D0, &args![list, slot]).bool() {
                return group;
            }
            group += 1;
        }
        6
    })
}

// Translated from 007db680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007db680 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the sound handles) is not translated.
/// `StatsMenu::DoGamepad` (engine map, vtable; `RET 8`: a key and a `float`). Only with the stats menu on top (`0x3eb`) and the Pip-Boy up;
/// returns whether the key was taken. In limb mode keys 1 to 4 pick a fade group (1, 2, 4, 3
/// give 0, 1, 2, 3; `007dc010` of the current group), which, when below 6, is faded in
/// (`007dbfa0`), the healing options' tiles updated and sound `01074f40` played. Keys 0xd
/// and 0xe with the value at least 1.0 play menu sound 0x24 and open the map
/// (`SetMapMenuVisible(1, 0)`) or the inventory (`SetInventoryMenuVisible(1, 0, 1)`). Keys
/// 4 and 3 step the page down and up (wrapping between 0 and 4) and play the tile sound
/// `0xfcb` of the first page button. On page 4 key 0xb clicks 0x31. On page 0: keys 1 and 2
/// step the SPECIAL/skill/status sub page (`007e0060`), play the matching button's tile
/// sound and move the knob (`MoveKnob(0, step * setting, setting, 1)`); key 9 clicks 0xa or
/// 0xb by sub page; keys 0xb and 0xc on sub page 0 click 9 (when not in limb mode) or 0x3b
/// when the button +0xb4 is enabled (with sound `0106f380`), on sub page 1 click 0x1f. Any
/// other key (or a sub page step that fails) scrolls the description or the list with the
/// right stick (`00a23390`, its magnitude through `00408820` above the dead zone at
/// `010745f8`): once the repeat time (`011dad5c`) has passed, the scroll trait moves by one,
/// the third scroll trait is poked when the value changed, and the next repeat time is the
/// time plus trait `0x1004` (repeating) or `0x1005` (first) of tile +0x114. Without stick
/// input the repeat time is cleared.
pub fn stats_menu_do_gamepad(e: &mut Engine, this: Ptr, key: u32, value: u32) -> bool {
    let base = this.addr();
    if !e.call(INTERFACE_IS_TOP_MENU_ID, &args![0x3ebu32]).bool() {
        return false;
    }
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    if !e.call(BYTE_AT_28_00974D90, &args![pipboy]).bool() {
        return false;
    }
    if !e.call(INTERFACE_IS_TOP_MENU_ID, &args![0x3ebu32]).bool() {
        return false;
    }
    if e.mem.u8(base + 0x2a0) != 0 && (key == 1 || key == 2 || key == 4 || key == 3) {
        let group = match key {
            1 => 0,
            2 => 1,
            3 => 3,
            4 => 2,
            _ => return false,
        };
        let object = fade(e);
        let current = e.call(STRING_CHARS_00559450, &args![object]).u32();
        let object = fade(e);
        let picked = fn_007dc010(e, object, current, group);
        if (picked as i32) < 6 {
            let object = fade(e);
            fn_007dbfa0(e, object, picked, 1);
            let first = fn_007e05f0(e, Ptr::new(base + 0x28));
            let tile = e.mem.u32(base + 0x2c);
            set_int(e, tile, 0xfaf, u32::from(first));
            let second = fn_007e06f0(e, Ptr::new(base + 0x58));
            let tile = e.mem.u32(base + 0x5c);
            set_int(e, tile, 0xfaf, u32::from(second));
            play_sound(e, SOUND_01074F40);
        }
        return true;
    }
    if (key == 0xd || key == 0xe) && x87_load32(value) >= 1.0 {
        e.call(PLAY_MENU_SOUND, &args![0x24u32]);
        if key == 0xd {
            e.call(SET_MAP_MENU_VISIBLE, &args![1u32, 0u32]);
        } else {
            e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 1u32]);
        }
        return true;
    }
    if key == 4 {
        let page = fn_007dcc00(e, this);
        let page = if page == 0 { 4 } else { page - 1 };
        stats_menu_set_current_page(e, this, page);
        let tile = e.mem.u32(base + 0x90);
        e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
        return true;
    }
    if key == 3 {
        let page = fn_007dcc00(e, this);
        let page = if page == 4 { 0 } else { page + 1 };
        stats_menu_set_current_page(e, this, page);
        let tile = e.mem.u32(base + 0x90);
        e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
        return true;
    }
    if fn_007dcc00(e, this) == 4 {
        if key == 0xb {
            menu_click(e, this, 0x31);
            return true;
        }
    } else if fn_007dcc00(e, this) == 0 {
        if key == 1 || key == 2 {
            let step: i32 = if key != 1 { 1 } else { -1 };
            let target = fn_007dcc40(e, this).wrapping_add(step);
            if fn_007e0060(e, this, target) {
                e.with_stack(4, |e, status| {
                    e.mem.set_u32(status.addr(), 6);
                    for (id, offset) in [
                        (0x1cu32, 0x100u32),
                        (0x1d, 0x104),
                        (0x1e, 0x108),
                        (0x35, 0x164),
                        (0x37, 0x16c),
                        (0x39, 0x174),
                    ] {
                        if fn_007e0160(e, this, id, status)
                            && e.mem.u32(status.addr()) as i32 == target
                        {
                            let tile = e.mem.u32(base + offset);
                            e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
                            break;
                        }
                    }
                });
                let speed = setting_float(e, KNOB_SETTING_011DB358);
                let distance = e
                    .call(SETTING_VALUE_00403E20, &args![KNOB_SETTING_011DB2F0])
                    .u32();
                let distance = e.mem.u32(distance);
                let moved = x87_store32(x87_mul(f64::from(step), x87_m32(distance)));
                let manager = pipboy_manager(e);
                e.call(PIPBOY_MOVE_KNOB, &args![manager, 0u32, moved, speed, 1u32]);
                return true;
            }
        } else if key == 9 {
            return match fn_007dcc40(e, this) {
                0 => {
                    menu_click(e, this, 0xa);
                    true
                }
                1 => {
                    menu_click(e, this, 0xb);
                    true
                }
                _ => false,
            };
        } else if key == 0xb {
            return match fn_007dcc40(e, this) {
                0 => {
                    let button = e.mem.u32(base + 0xb4);
                    if e.mem.u8(base + 0x2a0) == 0
                        && e.call(TILE_IS_TRUE, &args![button, 0xfafu32]).bool()
                    {
                        menu_click(e, this, 9);
                        play_sound(e, SOUND_0106F380);
                        true
                    } else {
                        false
                    }
                }
                1 => {
                    menu_click(e, this, 0x1f);
                    true
                }
                _ => false,
            };
        } else if key == 0xc {
            return match fn_007dcc40(e, this) {
                0 => {
                    let button = e.mem.u32(base + 0xb4);
                    if e.call(TILE_IS_TRUE, &args![button, 0xfafu32]).bool() {
                        menu_click(e, this, 0x3b);
                        play_sound(e, SOUND_0106F380);
                        true
                    } else {
                        false
                    }
                }
                1 => {
                    menu_click(e, this, 0x1f);
                    true
                }
                _ => false,
            };
        }
    }
    scroll_with_the_stick(e, this);
    false
}

/// The stick scrolling at the end of `DoGamepad` (see there).
fn scroll_with_the_stick(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !e.call(FLAG_00709D40, &args![]).bool() {
        return;
    }
    let list = e.call(WORD_AT_14_00825C00, &args![base + 0x240]).u32();
    if !e.call(TILE_IS_VISIBLE, &args![list]).bool() {
        let scroll = description_tile(e, this, SCROLL_TILE_SLOT_007B5370);
        if !e.call(TILE_IS_TRUE, &args![scroll, 0xfa3u32]).bool() {
            return;
        }
    }
    let input = e.global::<u32>(INPUT_011DEA0C);
    let input = e.call(INPUT_00877720, &args![input]).u32();
    let mut stick = e.call(STICK_00A23390, &args![input, 0u32, 0xau32]).i32();
    let magnitude = x87_store32(f64::from(stick));
    let magnitude = e.call(ABS_00408820, &args![magnitude]).f64();
    let dead_zone: f64 = e.global(DEAD_ZONE_DOUBLE);
    // `FCOMP; TEST AH,0x41; JP`: cleared when below or equal (ordered).
    if magnitude <= dead_zone {
        stick = 0;
    }
    let mut step = 0i32;
    if stick != 0 {
        step = if stick < 0 { 1 } else { -1 };
    }
    if step != 0 && e.call(TIME_00457FE0, &args![]).u32() > e.global::<u32>(SCROLL_REPEAT_011DAD5C)
    {
        let tile = if fn_007dcc00(e, this) == 0 {
            e.call(WORD_AT_14_00825C00, &args![base + 0x240]).u32()
        } else {
            description_tile(e, this, SCROLL_TILE_SLOT_007B5370)
        };
        let scroll = e
            .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_010702D8])
            .u32();
        let current = e.call(TILE_GET_FLOAT, &args![tile, scroll]).f64();
        let current = e.call(FTOL2, &args![current]).i32();
        let moved = x87_store32(f64::from(current.wrapping_add(step)));
        e.call(TILE_SET_FLOAT, &args![tile, scroll, moved, 0u32]);
        let now = e.call(TILE_GET_FLOAT, &args![tile, scroll]).f64();
        // `FUCOMPP; TEST AH,0x44; JNP`: poked unless equal (ordered).
        if f64::from(current) != now {
            let poke = e
                .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_01070250])
                .u32();
            e.call(TILE_POKE, &args![tile, poke, 1.0f32.to_bits()]);
        }
        let timing = e.mem.u32(base + 0x114);
        let delay = if e.global::<u32>(SCROLL_REPEAT_011DAD5C) != 0 {
            let value = e.call(TILE_GET_FLOAT, &args![timing, 0x1004u32]).f64();
            e.call(FTOL2, &args![value]).u32()
        } else {
            let value = e.call(TILE_GET_FLOAT, &args![timing, 0x1005u32]).f64();
            e.call(FTOL2, &args![value]).u32()
        };
        let time = e.call(TIME_00457FE0, &args![]).u32();
        e.set_global(SCROLL_REPEAT_011DAD5C, time.wrapping_add(delay));
    } else if step == 0 {
        e.set_global(SCROLL_REPEAT_011DAD5C, 0u32);
    }
}

// Translated from 007dbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 8`: a group and a flag byte): with the
/// flag, the current group's fade is stopped (`007dbfe0(0)`) before the group becomes
/// current and started after (`007dbfe0(1)`); without it the group is only stored.
pub fn fn_007dbfa0(e: &mut Engine, this: Ptr, group: u32, flag: u8) {
    if flag != 0 {
        fn_007dbfe0(e, this, 0);
    }
    e.mem.set_u32(this.addr(), group);
    if flag != 0 {
        fn_007dbfe0(e, this, 1);
    }
}

// Translated from 007dc010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the fade object, `RET 8`: a group and a byte index): byte `index`
/// of the group's word at +0x34 (the shifts by `CL`, so modulo 32).
pub fn fn_007dc010(e: &mut Engine, this: Ptr, group: u32, index: u32) -> u32 {
    let shift = index.wrapping_shl(3);
    let field = this
        .addr()
        .wrapping_add(group.wrapping_mul(4))
        .wrapping_add(0x34);
    (0xffu32.wrapping_shl(shift) & e.mem.u32(field)).wrapping_shr(shift)
}

// Translated from 007dc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a pointer): decrements the word it points to and returns the pointer.
pub fn fn_007dc0c0(e: &mut Engine, value: Ptr) -> Ptr {
    let word = e.mem.u32(value.addr()).wrapping_sub(1);
    e.mem.set_u32(value.addr(), word);
    value
}

// Translated from 007dc0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007dc0e0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `StatsMenu::DoGamepadUpEvent` (engine map, vtable; `RET 8`, a key and an unread word): in limb mode,
/// key 10 with the button +0xb4 enabled (`0xfaf`) clicks 9 (vtable +0xc), plays sound
/// `0106f380` and answers true; anything else false.
pub fn stats_menu_do_gamepad_up_event(e: &mut Engine, this: Ptr, key: u32, _unused_2: u32) -> bool {
    if e.mem.u8(this.addr() + 0x2a0) == 0 || key != 0xa {
        return false;
    }
    let button = e.mem.u32(this.addr() + 0xb4);
    if !e.call(TILE_IS_TRUE, &args![button, 0xfafu32]).bool() {
        return false;
    }
    menu_click(e, this, 9);
    play_sound(e, SOUND_0106F380);
    true
}

// Translated from 007dc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007dc1b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `StatsMenu::DoEnter` (engine map, vtable; `RET 8`: a
/// tile id and a tile; without a tile an id 0 to 0x3b takes `pTiles[id]`). By page
/// (`007dcc00`): page 0 moves the knob for a tile with `0xfac` above 0.0 (`007dfd20` with
/// the step at `011dad60`); then, without `004b71d0` and with the healing option record's
/// tile (+0x60 in hardcore mode, else +0x30), a tile in a fade group (`007db630`) fades that
/// group in (`007dbfa0`), plays sound `01074f40` and turns limb mode on (`007e0230`) when it
/// is off. Pages 1 to 3: an item tile of the SPECIAL (+0x180), skill (+0x1b0) or perk
/// (+0x1e0) list (`007242f0`) moves the knob (steps `011dad64`, `011dad68`, `011dad6c`) and
/// shows the item's icon and description (`SetIconAndDescription`, `007dc9b0` for perks).
/// Page 4: an item of the general list (+0x210) moves the knob; an item of the reputation
/// list (+0x270) shows the reputation (`007dcb40`) and moves the knob (both with the step
/// at `011dad70`).
pub fn stats_menu_do_enter(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && (0..0x3c).contains(&id) {
        tile = e.mem.u32(base + 0x90 + 4 * id as u32);
    }
    match fn_007dcc00(e, this) as u32 {
        0 => {
            let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
            let zero: f64 = e.global(ZERO_DOUBLE);
            if above(value, zero) {
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD60));
            }
            if e.call(FLAG_004B71D0, &args![]).bool() {
                return;
            }
            let record = if player(e) != 0 && e.call(HARDCORE_004D1360, &args![player(e)]).bool() {
                e.mem.u32(base + 0x60)
            } else {
                e.mem.u32(base + 0x30)
            };
            if record == 0 {
                return;
            }
            let object = fade(e);
            let group = fn_007db630(e, object, tile);
            if group == 6 {
                return;
            }
            let object = fade(e);
            fn_007dbfa0(e, object, group, 1);
            play_sound(e, SOUND_01074F40);
            if e.mem.u8(base + 0x2a0) == 0 {
                fn_007e0230(e, this);
            }
        }
        1 => {
            if e.call(LIST_BOX_HAS_TILE_007242F0, &args![base + 0x180, tile])
                .bool()
            {
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD64));
                let value = fn_007e1110(e, Ptr::new(base + 0x180), tile);
                fn_007dc7e0(e, this, value);
            }
        }
        2 => {
            if e.call(LIST_BOX_HAS_TILE_007242F0, &args![base + 0x1b0, tile])
                .bool()
            {
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD68));
                let value = fn_007e1110(e, Ptr::new(base + 0x1b0), tile);
                fn_007dc7e0(e, this, value);
            }
        }
        3 => {
            if e.call(LIST_BOX_HAS_TILE_007242F0, &args![base + 0x1e0, tile])
                .bool()
            {
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD6C));
                let perk = fn_007e1110(e, Ptr::new(base + 0x1e0), tile);
                fn_007dc9b0(e, this, Ptr::new(perk));
            }
        }
        4 => {
            if e.call(LIST_BOX_HAS_TILE_007242F0, &args![base + 0x210, tile])
                .bool()
            {
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD70));
            }
            if e.call(LIST_BOX_HAS_TILE_007242F0, &args![base + 0x270, tile])
                .bool()
            {
                let reputation = fn_007e1110(e, Ptr::new(base + 0x270), tile);
                fn_007dcb40(e, this, reputation);
                stats_menu_move_knob(e, this, tile, Ptr::new(KNOB_STEP_011DAD70));
            }
        }
        _ => {}
    }
}

// Translated from 007dc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::DoLeave` (engine map, vtable; `RET 8`: an unread tile id and a tile): in limb mode without `004b71d0`, leaving
/// a tile of a fade group (`007db630`) turns limb mode off (`007e0230`); the badge tile
/// (+0x160) is hidden.
pub fn stats_menu_do_leave(e: &mut Engine, this: Ptr, _unused_1: u32, tile: u32) {
    if e.mem.u8(this.addr() + 0x2a0) != 0 && !e.call(FLAG_004B71D0, &args![]).bool() {
        let object = fade(e);
        if fn_007db630(e, object, tile) != 6 {
            fn_007e0230(e, this);
        }
    }
    let badge = e.mem.u32(this.addr() + 0x160);
    set_int(e, badge, 0xfa3, 0);
}

// Translated from 007dc7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SetIconAndDescription` (Xbox PDB; `RET 4`, an actor value): the icon tile
/// (+0xf4) is shown with the value's icon (`0xfcc`), the description box's text tile its
/// description (`0xfc4`); a value with a weapon type badge shows it on tile +0x160
/// (`0xfa3` set, the badge as `0xfcc`), else that tile is hidden. Then the description
/// scroll is set up (see `update_description_scroll`) with the scroll tile's scale trait.
pub fn fn_007dc7e0(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let text_tile = description_tile(e, this, TEXT_TILE_SLOT_0073AC90);
    let scroll_tile = description_tile(e, this, SCROLL_TILE_SLOT_007B5370);
    let trait_id = e
        .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_01074EB8])
        .u32();
    let scale = x87_store32(e.call(TILE_GET_FLOAT, &args![scroll_tile, trait_id]).f64());
    let icon_tile = e.mem.u32(base + 0xf4);
    set_int(e, icon_tile, 0xfa3, 1);
    let icon = e.call(AV_GET_ICON, &args![value]).u32();
    let icon_tile = e.mem.u32(base + 0xf4);
    set_string(e, icon_tile, 0xfcc, icon);
    let description = e.call(AV_GET_DESCRIPTION, &args![value]).u32();
    set_string(e, text_tile, 0xfc4, description);
    let badge = e.call(AV_GET_BADGE, &args![value]).u32();
    let badge_tile = e.mem.u32(base + 0x160);
    if badge != 0 {
        set_int(e, badge_tile, 0xfa3, 1);
        let badge_tile = e.mem.u32(base + 0x160);
        set_string(e, badge_tile, 0xfcc, badge);
    } else {
        set_int(e, badge_tile, 0xfa3, 0);
    }
    update_description_scroll(e, text_tile, scroll_tile, scale);
}

// Translated from 007dc9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu, `RET 4`, a perk list item's data: the perk at
/// +0): as `SetIconAndDescription` for a perk: the icon tile shows the perk's icon (the
/// string at perk +0x2c), the text tile the description (vtable +0x10 of the perk's
/// description at +0x24, given 0 and `'CSED'`); no badge. Then the description scroll.
pub fn fn_007dc9b0(e: &mut Engine, this: Ptr, data: Ptr) {
    let base = this.addr();
    let text_tile = description_tile(e, this, TEXT_TILE_SLOT_0073AC90);
    let scroll_tile = description_tile(e, this, SCROLL_TILE_SLOT_007B5370);
    let trait_id = e
        .call(TILE_TEXT_TO_TRAIT, &args![SCROLL_TRAIT_NAME_01074EB8])
        .u32();
    let scale = x87_store32(e.call(TILE_GET_FLOAT, &args![scroll_tile, trait_id]).f64());
    let icon_tile = e.mem.u32(base + 0xf4);
    set_int(e, icon_tile, 0xfa3, 1);
    let perk = e.mem.u32(data.addr());
    let icon = e
        .call(STRING_CHARS_00408DA0, &args![perk.wrapping_add(0x2c)])
        .u32();
    let icon_tile = e.mem.u32(base + 0xf4);
    set_string(e, icon_tile, 0xfcc, icon);
    let perk = e.mem.u32(data.addr());
    let description = e
        .vcall(perk.wrapping_add(0x24), 0x10, &args![0u32, 0x4353_4544u32])
        .u32();
    set_string(e, text_tile, 0xfc4, description);
    update_description_scroll(e, text_tile, scroll_tile, scale);
}

// Translated from 007dcb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu, `RET 4`, a reputation): a non-null reputation
/// shows its icon (`006166e0`) on tile +0x14c, the tiles +0x14c, +0x150 and +0x15c, its name
/// (the string at +0x18) on +0x15c and its title (`GetReputationTitle`) on +0x150.
pub fn fn_007dcb40(e: &mut Engine, this: Ptr, reputation: u32) {
    if reputation == 0 {
        return;
    }
    let base = this.addr();
    let icon = e.call(REPUTATION_ICON_006166E0, &args![reputation]).u32();
    let tile = e.mem.u32(base + 0x14c);
    set_string(e, tile, 0xfcc, icon);
    for offset in [0x14cu32, 0x150, 0x15c] {
        let tile = e.mem.u32(base + offset);
        set_int(e, tile, 0xfa3, 1);
    }
    let name = e
        .call(STRING_CHARS_00408DA0, &args![reputation.wrapping_add(0x18)])
        .u32();
    let tile = e.mem.u32(base + 0x15c);
    set_string(e, tile, 0xfc4, name);
    let title = e.call(REPUTATION_GET_TITLE, &args![reputation]).u32();
    let tile = e.mem.u32(base + 0x150);
    set_string(e, tile, 0xfc4, title);
}

/// `UpdateAll`'s labels: the tile offsets on the menu and the game settings they show.
const UPDATE_ALL_LABELS: [(u32, u32); 10] = [
    (0x90, 0x011d_2f88),
    (0x94, 0x011d_2994),
    (0x98, 0x011d_3b08),
    (0x9c, 0x011d_4bd0),
    (0xa0, 0x011d_4ef4),
    (0x100, 0x011d_48d0),
    (0x104, 0x011d_3e74),
    (0x164, 0x011d_1f08),
    (0x174, 0x011d_246c),
    (0x16c, 0x011d_37c8),
];
/// `UpdateAll`'s named children of the menu tile, the game setting each shows and whether
/// it goes to `"_Title"` (else `0xfc4`).
const UPDATE_ALL_CHILDREN: [(u32, u32, bool); 7] = [
    (0x0107_7398, 0x011d_3fdc, false),
    (0x0107_7388, 0x011d_1f5c, true),
    (0x0107_7378, 0x011d_50b0, true),
    (0x0107_7368, 0x011d_44a4, true),
    (0x0107_7358, 0x011d_3bd4, true),
    (0x0107_7340, 0x011d_47ec, false),
    (0x0107_732c, 0x011d_2ebc, false),
];

// Translated from 007dcc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::UpdateAll` (Xbox PDB; `thiscall`): the actor values (`007dcfc0`), the general
/// list (`007ddaa0`), the reputations (`007dd8c0`), the healing options (`007df230(4)`) and
/// the fade group (`007df920`). When the setting at `011c3cb4` differs from `01018348`
/// (`00408b20`), the labels are set again from the game settings: ten tiles' `0xfc4`, the
/// menu tile's `0x100d` and `0x100e`, and seven named children (`GetChildByName`) on
/// `0xfc4` or `"_Title"`.
pub fn stats_menu_update_all(e: &mut Engine, this: Ptr) {
    stats_menu_update_all_actor_values(e, this);
    fn_007ddaa0(e, this);
    fn_007dd8c0(e, this);
    stats_menu_update_healing_options(e, this, 4);
    fn_007df920(e, this);
    let setting = setting_string(e, SETTING_011C3CB4);
    if e.call(COMPARE_00408B20, &args![setting, COMPARED_STRING_01018348])
        .u32()
        == 0
    {
        return;
    }
    for (offset, setting) in UPDATE_ALL_LABELS {
        let text = setting_string(e, setting);
        let tile = e.mem.u32(this.addr() + offset);
        set_string(e, tile, 0xfc4, text);
    }
    for (trait_id, setting) in [(0x100du32, 0x011d_2e80u32), (0x100e, 0x011d_41bc)] {
        let text = setting_string(e, setting);
        let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
        set_string(e, root, trait_id, text);
    }
    for (name, setting, title) in UPDATE_ALL_CHILDREN {
        let text = setting_string(e, setting);
        let trait_id = if title {
            e.call(TILE_TEXT_TO_TRAIT, &args![TITLE_TEXT]).u32()
        } else {
            0xfc4
        };
        let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
        let child = e.call(TILE_GET_CHILD_BY_NAME, &args![root, name]).u32();
        set_string(e, child, trait_id, text);
    }
}

// Translated from 007dd8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007dd8c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the string local) is not translated.
/// Unnamed (`thiscall` on the stats menu): the reputation list. With an open menu whose
/// reputation page tile (+0x148) exists: the reputation tiles (+0x14c, +0x15c, +0x150) are
/// hidden, the open menu's list (+0x270) cleared (vtable +0x1c), and each reputation of
/// the list at `011c3f2c` (`009c1a50`) with either value (`006155c0(1)`, `006155c0(0)`)
/// above 0.0 is added (`007e16d0`, sorted by `007e0510`) with its name formatted
/// (`01019f08`).
pub fn fn_007dd8c0(e: &mut Engine, this: Ptr) {
    let menu = open_menu(e);
    if menu == 0 || e.mem.u32(menu + 0x148) == 0 {
        return;
    }
    let base = this.addr();
    for offset in [0x14cu32, 0x15c, 0x150] {
        let tile = e.mem.u32(base + offset);
        set_int(e, tile, 0xfa3, 0);
    }
    let menu = open_menu(e);
    e.vcall(menu + 0x270, 0x1c, &args![]);
    let reputations = e.global::<u32>(REPUTATIONS_011C3F2C);
    let mut node = e.call(REPUTATION_LIST_009C1A50, &args![reputations]).u32();
    while node != 0 && node_item(e, node) != 0 {
        let reputation = node_item(e, node);
        if reputation != 0 {
            let zero: f64 = e.global(ZERO_DOUBLE);
            let first = e
                .call(REPUTATION_VALUE_006155C0, &args![reputation, 1u32])
                .f64();
            // `TEST AH,0x41; JZ`: added when above 0.0; else the second value decides.
            let shown = above(first, zero) || {
                let second = e
                    .call(REPUTATION_VALUE_006155C0, &args![reputation, 0u32])
                    .f64();
                above(second, zero)
            };
            if shown {
                e.with_stack(8, |e, text| {
                    e.call(STRING_CONSTRUCT, &args![text]);
                    let name = e
                        .call(STRING_CHARS_00408DA0, &args![reputation.wrapping_add(0x18)])
                        .u32();
                    e.call(STRING_FORMAT, &args![text, ONE_STRING_FORMAT, name]);
                    let chars = e.call(STRING_CHARS_00559450, &args![text]).u32();
                    let menu = open_menu(e);
                    fn_007e16d0(e, Ptr::new(menu + 0x270), reputation, chars, 0x007e_0510, 0);
                    e.call(STRING_DESTROY, &args![text]);
                });
            }
        }
        node = node_next(e, node);
    }
}

// Translated from 007dda60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): with an open stats menu whose +0x118 is set, its perk list
/// (+0x1e0) is cleared (vtable +0x1c).
pub fn fn_007dda60(e: &mut Engine) {
    let menu = open_menu(e);
    if menu != 0 && e.mem.u32(menu + 0x118) != 0 {
        let menu = open_menu(e);
        e.vcall(menu + 0x1e0, 0x1c, &args![]);
    }
}

// Translated from 007ddaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the general list's (+0x210) items each get their
/// misc stat's value (`007314c0` with `007dfc60`, 0 and `0x7fffffff`).
pub fn fn_007ddaa0(e: &mut Engine, this: Ptr) {
    e.call(
        ITEM_CHANGE_007314C0,
        &args![this.addr() + 0x210, 0x007d_fc60u32, 0u32, 0x7fff_ffffu32],
    );
}

/// The three hardcore meters (`007ddc20`, `007ddd10`, `007dde00`): with `a` the menu tile's
/// trait `0x1008` times 1000 and `b = -a / 2`, `c = 005c5420() * a + b`; `d` is
/// `00404010(0.0, value)` of the player's actor value; `pi/2 - (d + c) / 1000 * pi` is
/// computed and stored but not used. A `d` strictly between 0 and 1 becomes 1, any other
/// `004bd510(d, 1.0)`; it goes to the trait of tile +4.
fn hardcore_meter(e: &mut Engine, this: Ptr, value: u32, trait_id: u32) {
    let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
    let range = e.call(TILE_GET_FLOAT, &args![root, 0x1008u32]).f64();
    let thousand: f64 = e.global(THOUSAND_DOUBLE);
    let two: f64 = e.global(TWO_DOUBLE);
    let a = x87_store32(x87_mul(range, thousand));
    // `FLD a; FCHS; FDIV 2.0; FSTP double`.
    let b = x87_div(-x87_load32(a), two);
    let scale = e.call(VALUE_005C5420, &args![]).f64();
    let c = x87_store32(x87_add(x87_mul(scale, x87_m32(a)), b));
    let current = x87_store32(e.vcall(actor_values(e), AV_FLOAT, &args![value]).f64());
    let d = x87_store32(
        e.call(FLOAT_PAIR_00404010, &args![0.0f32.to_bits(), current])
            .f64(),
    );
    let pi: f64 = e.global(PI_DOUBLE);
    let half_pi: f64 = e.global(HALF_PI_DOUBLE);
    let sum = x87_add(x87_load32(d), x87_m32(c));
    let _angle = x87_store32(x87_sub(half_pi, x87_mul(x87_div(sum, thousand), pi)));
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    // `TEST AH,0x41; JNZ` then `TEST AH,5; JP`: 1.0 only when strictly between 0 and 1.
    let d = if above(x87_load32(d), zero) && x87_load32(d) < one {
        1.0f32.to_bits()
    } else {
        x87_store32(
            e.call(
                FLOAT_PAIR_004BD510,
                &args![x87_quiet_bits(d), 1.0f32.to_bits()],
            )
            .f64(),
        )
    };
    let tile = e.mem.u32(this.addr() + 4);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, trait_id, x87_quiet_bits(d), 1u32],
    );
}

// Translated from 007ddc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the hardcore meter of actor value 0x49 on trait
/// `0x100f` (see `hardcore_meter`).
pub fn fn_007ddc20(e: &mut Engine, this: Ptr) {
    hardcore_meter(e, this, 0x49, 0x100f);
}

// Translated from 007ddd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the hardcore meter of actor value 0x4a on trait
/// `0x1010` (see `hardcore_meter`).
pub fn fn_007ddd10(e: &mut Engine, this: Ptr) {
    hardcore_meter(e, this, 0x4a, 0x1010);
}

// Translated from 007dde00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the hardcore meter of actor value 0x4b on trait
/// `0x1011` (see `hardcore_meter`).
pub fn fn_007dde00(e: &mut Engine, this: Ptr) {
    hardcore_meter(e, this, 0x4b, 0x1011);
}

// Translated from 007df120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): `00501c10` on `this + 0x20` with the word.
pub fn fn_007df120(e: &mut Engine, this: Ptr, value: u32) {
    e.call(FIELD_SET_00501C10, &args![this.addr() + 0x20, value]);
}

// Translated from 007df140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007df140 - the C++ exception frame (FS:[0] chain, unwind state 0 after the first list) is not translated.
/// Unnamed (`thiscall`, `RET 4`: the 0x1c-byte status effect entry's constructor): the
/// lists at +4 (`00406db0`) and +0x10 (`0071f540`) built, the word stored at +0. Returns
/// `this`.
pub fn fn_007df140(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(LIST_CONSTRUCT_00406DB0, &args![this.addr() + 4]);
    e.call(LIST_CONSTRUCT_0071F540, &args![this.addr() + 0x10]);
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 007df920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): the limb with the lowest condition. Over the six
/// limb groups (`007df9c0`'s actor values, read as `float`s), the first strictly below the
/// lowest so far (starting from the largest `float`, `01016970`) wins (6 when none); the
/// fade object makes it current (`007dbfa0`), fading when limb mode (+0x2a0) is on.
pub fn fn_007df920(e: &mut Engine, this: Ptr) {
    let mut best_group = 6u32;
    let mut best = x87_quiet_bits(e.global::<u32>(FLOAT_MAX_01016970));
    let mut group = 0u32;
    while (group as i32) < 6 {
        let value = fn_007df9c0(e, group);
        let value = x87_store32(e.vcall(actor_values(e), AV_FLOAT, &args![value]).f64());
        // `FLD value; FLD best; FCOMPP; TEST AH,0x41; JNZ`: taken when best is above it.
        if above(x87_load32(best), x87_load32(value)) {
            best = value;
            best_group = group;
        }
        group += 1;
    }
    let flag = e.mem.u8(this.addr() + 0x2a0);
    let object = fade(e);
    fn_007dbfa0(e, object, best_group, flag);
}

// Translated from 007df9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a limb group 0 to 5): its actor value (0x19, 0x1c, 0x1a, 0x1b, 0x1e,
/// 0x1d), or -1 for any other.
pub fn fn_007df9c0(_e: &mut Engine, group: u32) -> u32 {
    match group {
        0 => 0x19,
        1 => 0x1c,
        2 => 0x1a,
        3 => 0x1b,
        4 => 0x1e,
        5 => 0x1d,
        _ => 0xffff_ffff,
    }
}

// Translated from 007dfc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SetKnobPosition` (Xbox PDB; `cdecl`, a byte): with an open stats menu, the
/// Pip-Boy's tab knob goes to its page (`FOPipboyManager::SetTabKnobPosition(page, byte)`).
pub fn stats_menu_set_knob_position(e: &mut Engine, flag: u8) {
    let menu = open_menu(e);
    if menu == 0 {
        return;
    }
    let page = fn_007dcc00(e, Ptr::new(menu));
    let manager = pipboy_manager(e);
    e.call(
        PIPBOY_SET_TAB_KNOB_POSITION,
        &args![manager, page as u32, u32::from(flag)],
    );
}

// Translated from 007dfd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::SetCurrentStatsPage` (Xbox PDB; `cdecl`, a byte): `SetCurrentPage` of the
/// open stats menu, if any.
pub fn stats_menu_set_current_stats_page(e: &mut Engine, page: u8) {
    let menu = open_menu(e);
    if menu != 0 {
        stats_menu_set_current_page(e, Ptr::new(menu), i32::from(page));
    }
}

// Translated from 007dfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::MoveKnob` (Xbox PDB; `RET 8`: a tile and the address of the last step): the
/// tile's `0xfac` truncated (`FISTP qword` with the rounding mode set to chop; its low
/// word). Above the last step (unsigned) the Pip-Boy's knob moves forward
/// (`MoveKnob(0, distance, speed, 1)` with the game settings at `011db2f0` and `011db358`),
/// below it backward (the distance negated); the step is kept.
pub fn stats_menu_move_knob(e: &mut Engine, _this: Ptr, tile: u32, last: Ptr) {
    let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    let step = x87_fistp64_chop(X87Ext::load64(value)) as u32;
    let previous = e.mem.u32(last.addr());
    if previous < step {
        let speed = setting_float(e, KNOB_SETTING_011DB358);
        let distance = setting_float(e, KNOB_SETTING_011DB2F0);
        let manager = pipboy_manager(e);
        e.call(
            PIPBOY_MOVE_KNOB,
            &args![manager, 0u32, distance, speed, 1u32],
        );
    } else if previous > step {
        let speed = setting_float(e, KNOB_SETTING_011DB358);
        // `FLD; FCHS`: the loaded value with its sign flipped.
        let distance = setting_float(e, KNOB_SETTING_011DB2F0) ^ 0x8000_0000;
        let manager = pipboy_manager(e);
        e.call(
            PIPBOY_MOVE_KNOB,
            &args![manager, 0u32, distance, speed, 1u32],
        );
    }
    e.mem.set_u32(last.addr(), step);
}

// Translated from 007e0060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu, `RET 4`, a sub page): a sub page from 0 to 2 (5 in
/// hardcore mode) other than the current one (`007dcc40`) becomes current: limb mode ends
/// (`007e0230`) and tile +0xa4's `0x1004` is set to it; true. Otherwise false.
pub fn fn_007e0060(e: &mut Engine, this: Ptr, page: i32) -> bool {
    if page < 0 {
        return false;
    }
    let last = if e.call(HARDCORE_004D1360, &args![player(e)]).bool() {
        5
    } else {
        2
    };
    if page > last || fn_007dcc40(e, this) == page {
        return false;
    }
    if e.mem.u8(this.addr() + 0x2a0) != 0 {
        fn_007e0230(e, this);
    }
    let tile = e.mem.u32(this.addr() + 0xa4);
    set_int(e, tile, 0x1004, page as u32);
    true
}

// Translated from 007e00e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::GetPageFor` (Xbox PDB; `RET 8`: a tile id and the page's address): ids 0 to
/// 4 are the page buttons: the page is stored and true returned; any other id false.
pub fn stats_menu_get_page_for(e: &mut Engine, _this: Ptr, id: u32, page: Ptr) -> bool {
    if id > 4 {
        return false;
    }
    e.mem.set_u32(page.addr(), id);
    true
}

// Translated from 007e0160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed here (the engine map names it `StatsMenu::SetStatusPage`, but the body only looks
/// the id up; `RET 8`: a tile id and the
/// sub page's address): ids 0x1c, 0x1d, 0x1e, 0x35, 0x37 and 0x39 are the status sub page
/// buttons 0 to 5: the sub page is stored and true returned; any other id false.
pub fn fn_007e0160(e: &mut Engine, _this: Ptr, id: u32, page: Ptr) -> bool {
    let value = match id.wrapping_sub(0x1c) {
        0 => 0,
        1 => 1,
        2 => 2,
        0x19 => 3,
        0x1b => 4,
        0x1d => 5,
        _ => return false,
    };
    e.mem.set_u32(page.addr(), value);
    true
}

// Translated from 007e0230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the stats menu): toggles limb mode (+0x2a0), updates the two
/// healing options' tiles (`0xfaf` of +0x2c from `007e05f0(this + 0x28)`, of +0x5c from
/// `007e06f0(this + 0x58)`) and the healing mode (`007df620`).
pub fn fn_007e0230(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mode = u8::from(e.mem.u8(base + 0x2a0) == 0);
    e.mem.set_u8(base + 0x2a0, mode);
    let first = fn_007e05f0(e, Ptr::new(base + 0x28));
    let tile = e.mem.u32(base + 0x2c);
    set_int(e, tile, 0xfaf, u32::from(first));
    let second = fn_007e06f0(e, Ptr::new(base + 0x58));
    let tile = e.mem.u32(base + 0x5c);
    set_int(e, tile, 0xfaf, u32::from(second));
    stats_menu_update_healing_mode(e, this);
}

// Translated from 007e02b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e02b0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the statics' initialisers) is not translated.
/// `StatsMenu::ToggleGenRepMode` (Xbox PDB; `thiscall`): the function-local trait ids
/// (`"_enabled"` and `01077564`, `Tile::AddUserTrait` once each, guard bits 1 and 2 at
/// `011dad8c`); the mode byte (`011a0ae0`) toggled; the open menu's reputation tile (+0x148)
/// gets the first trait and `0xfa3` set to the mode, the general tile (+0xb0) both set to
/// the opposite; the title tile (+0x154) shows the game setting `011d4ef4` (reputations) or
/// `011d4144` (general) and tile +0x158's `0x1004` is 1 or 0.
pub fn stats_menu_toggle_gen_rep_mode(e: &mut Engine, _this: Ptr) {
    let guard = e.global::<u32>(GEN_REP_GUARD_011DAD8C);
    if guard & 1 == 0 {
        e.set_global(GEN_REP_GUARD_011DAD8C, guard | 1);
        let trait_id = e
            .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
            .u32();
        e.set_global(GEN_REP_TRAIT_011DAD88, trait_id);
    }
    let guard = e.global::<u32>(GEN_REP_GUARD_011DAD8C);
    if guard & 2 == 0 {
        e.set_global(GEN_REP_GUARD_011DAD8C, guard | 2);
        let trait_id = e
            .call(
                ADD_USER_TRAIT,
                &args![GEN_REP_TRAIT_NAME_01077564, 0xffff_ffffu32],
            )
            .u32();
        e.set_global(GEN_REP_TRAIT_011DAD84, trait_id);
    }
    let mode = u8::from(e.global::<u8>(GEN_REP_MODE_011A0AE0) == 0);
    e.set_global(GEN_REP_MODE_011A0AE0, mode);
    let enabled = e.global::<u32>(GEN_REP_TRAIT_011DAD88);
    let mode = u32::from(e.global::<u8>(GEN_REP_MODE_011A0AE0));
    let tile = e.mem.u32(open_menu(e) + 0x148);
    set_int(e, tile, enabled, mode);
    let mode = u32::from(e.global::<u8>(GEN_REP_MODE_011A0AE0));
    let tile = e.mem.u32(open_menu(e) + 0x148);
    set_int(e, tile, 0xfa3, mode);
    let other = u32::from(e.global::<u8>(GEN_REP_MODE_011A0AE0) == 0);
    let enabled = e.global::<u32>(GEN_REP_TRAIT_011DAD88);
    let tile = e.mem.u32(open_menu(e) + 0xb0);
    set_int(e, tile, enabled, other);
    let other = u32::from(e.global::<u8>(GEN_REP_MODE_011A0AE0) == 0);
    let tile = e.mem.u32(open_menu(e) + 0xb0);
    set_int(e, tile, 0xfa3, other);
    if e.global::<u8>(GEN_REP_MODE_011A0AE0) != 0 {
        let text = setting_string(e, SETTING_011D4EF4);
        let tile = e.mem.u32(open_menu(e) + 0x154);
        set_string(e, tile, 0xfc4, text);
        let tile = e.mem.u32(open_menu(e) + 0x158);
        set_int(e, tile, 0x1004, 1);
    } else {
        let text = setting_string(e, SETTING_011D4144);
        let tile = e.mem.u32(open_menu(e) + 0x154);
        set_string(e, tile, 0xfc4, text);
        let tile = e.mem.u32(open_menu(e) + 0x158);
        set_int(e, tile, 0x1004, 0);
    }
}

// Translated from 007e0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two status effect entries: the effect list's sort): `00404dc0` of the
/// names (`00408da0` of each entry's first word cast by `__RTDynamicCast` from `01183028`
/// to `01183158`), the second's looked up first.
pub fn fn_007e0480(e: &mut Engine, first: u32, second: u32) -> i32 {
    let object = e.mem.u32(second);
    let named = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0u32, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0u32],
        )
        .u32();
    let second_name = e.call(STRING_CHARS_00408DA0, &args![named]).u32();
    let object = e.mem.u32(first);
    let named = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0u32, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0u32],
        )
        .u32();
    let first_name = e.call(STRING_CHARS_00408DA0, &args![named]).u32();
    e.call(COMPARE_00404DC0, &args![first_name, second_name])
        .i32()
}

// Translated from 007e0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two list items: the reputation list's sort): `00404dc0` of the names
/// (the string at +0x18 of each item's data at +4), the second's looked up first.
pub fn fn_007e0510(e: &mut Engine, first: u32, second: u32) -> i32 {
    let data = e.mem.u32(second.wrapping_add(4));
    let second_name = e
        .call(STRING_CHARS_00408DA0, &args![data.wrapping_add(0x18)])
        .u32();
    let data = e.mem.u32(first.wrapping_add(4));
    let first_name = e
        .call(STRING_CHARS_00408DA0, &args![data.wrapping_add(0x18)])
        .u32();
    e.call(COMPARE_00404DC0, &args![first_name, second_name])
        .i32()
}

// Translated from 007e0540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e0540 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two string locals) is not translated.
/// Unnamed (`cdecl`, two stage effects: the stage list's sort): each gives a string
/// (`004046b0` into a local, the second's first), and `00404dc0` compares their texts
/// (first, second); the locals are destroyed (the first's, then the second's).
pub fn fn_007e0540(e: &mut Engine, first: u32, second: u32) -> i32 {
    e.with_stack(8, |e, second_text| {
        let made = e
            .call(ENTRY_STRING_004046B0, &args![second, second_text])
            .u32();
        let second_chars = e.call(STRING_CHARS_00559450, &args![made]).u32();
        e.with_stack(8, |e, first_text| {
            let made = e
                .call(ENTRY_STRING_004046B0, &args![first, first_text])
                .u32();
            let first_chars = e.call(STRING_CHARS_00559450, &args![made]).u32();
            let result = e
                .call(COMPARE_00404DC0, &args![first_chars, second_chars])
                .i32();
            e.call(STRING_DESTROY, &args![first_text]);
            e.call(STRING_DESTROY, &args![second_text]);
            result
        })
    })
}

// Translated from 007e06b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): with `0070ed80`, the actor value of the fade object's current
/// group (`007df9c0` of its first word, through `00559450`); otherwise -1.
pub fn fn_007e06b0(e: &mut Engine) -> u32 {
    if !e.call(FLAG_0070ED80, &args![]).bool() {
        return 0xffff_ffff;
    }
    let object = fn_007db1c0(e);
    let group = e.call(STRING_CHARS_00559450, &args![object]).u32();
    fn_007df9c0(e, group)
}

// Translated from 007e06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, a healing option record: the limb option's availability callback):
/// false without an inventory entry (+8), and outside hardcore mode with a limb picked
/// (`007e06b0` not -1). Otherwise true when any limb (actor values 0x19 to 0x1e) is below
/// its base (slot +8 against +0x1c).
pub fn fn_007e06f0(e: &mut Engine, record: Ptr) -> bool {
    let hardcore = e.call(HARDCORE_004D1360, &args![player(e)]).bool();
    if e.mem.u32(record.addr() + 8) == 0 {
        return false;
    }
    let limb = fn_007e06b0(e);
    if !hardcore && limb != 0xffff_ffff {
        return false;
    }
    for value in 0x19u32..0x1f {
        let current = e.vcall(actor_values(e), AV_VALUE, &args![value]).i32();
        let base = e.vcall(actor_values(e), AV_BASE, &args![value]).i32();
        if current < base {
            return true;
        }
    }
    false
}

// Translated from 007e08a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e08a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the entry's construction) is not translated.
/// `StatsMenu::StatusEffectList::Register` (Xbox PDB; `RET 4`, an active effect): the
/// effect's target (`0044ddc0`) cast from `01183140` to `01183028`; the entry for its key
/// (`0084e3a0`, `007e0c50`) is found or made (0x1c bytes, `007df140`) and inserted sorted
/// (`007e0480`). An effect whose setting has flag `0x2000` goes to the entry's stage list
/// (+0x10, `007e09d0`), any other to its data list (+4, `007e0a20`).
pub fn stats_menu_status_effect_list_register(e: &mut Engine, this: Ptr, effect: u32) {
    let target = e.call(EFFECT_TARGET_0044DDC0, &args![effect]).u32();
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, TYPE_TARGET_01183140, TYPE_FORM_01183028, 0u32],
        )
        .u32();
    let key = e.call(EFFECT_ITEM_0084E3A0, &args![form]).u32();
    let mut entry = fn_007e0c50(e, this, key);
    if entry == 0 {
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let made = if block != 0 {
            fn_007df140(e, Ptr::new(block), form).addr()
        } else {
            0
        };
        entry = made;
        e.call(
            LIST_INSERT_SORTED,
            &args![this.addr() + 4, entry, 0x007e_0480u32],
        );
    }
    let setting = fn_007df1d0(e, Ptr::new(effect));
    if e.call(FLAG_TEST_00403E60, &args![setting, 0x2000u32])
        .bool()
    {
        fn_007e09d0(e, Ptr::new(entry.wrapping_add(0x10)), effect);
    } else {
        fn_007e0a20(e, Ptr::new(entry.wrapping_add(4)), effect);
    }
}

// Translated from 007e09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a stage list, `RET 4`, an active effect): the effect's setting
/// (`007df1d0`) is inserted (sorted by `007e0540`) unless the list (+4) holds it already
/// (`005f65d0`).
pub fn fn_007e09d0(e: &mut Engine, this: Ptr, effect: u32) {
    let list = this.addr() + 4;
    let setting = fn_007df1d0(e, Ptr::new(effect));
    let present = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), setting);
        e.call(LIST_CONTAINS_005F65D0, &args![list, slot]).bool()
    });
    if !present {
        let setting = fn_007df1d0(e, Ptr::new(effect));
        e.call(LIST_INSERT_SORTED, &args![list, setting, 0x007e_0540u32]);
    }
}

// Translated from 007e0a20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e0a20 - the C++ exception frame (FS:[0] chain, unwind state 0 around the entry's construction) is not translated.
/// Unnamed (`thiscall` on a status data list, `RET 4`, an active effect): as `Register` of
/// the status data list for an active effect: the key is `00403ea0` of its effect item
/// (`0084e3a0`), the magnitude `_ftol2` of `00495460`. An entry with the key gains the
/// magnitude; otherwise a new 0x14-byte entry (`00403ba0`) takes key and magnitude and is
/// appended (+4).
pub fn fn_007e0a20(e: &mut Engine, this: Ptr, effect: u32) {
    if effect == 0 {
        return;
    }
    let item = e.call(EFFECT_ITEM_0084E3A0, &args![effect]).u32();
    let key = e.call(EFFECT_KEY_00403EA0, &args![item]).u32();
    let found = fn_007e0ce0(e, this, key);
    if found != 0 {
        let magnitude = e.call(MAGNITUDE_00495460, &args![effect]).f64();
        let magnitude = e.call(FTOL2, &args![magnitude]).u32();
        let total = magnitude.wrapping_add(e.mem.u32(found));
        e.mem.set_u32(found, total);
        return;
    }
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let entry = if block != 0 {
        e.call(ENTRY_CONSTRUCT_00403BA0, &args![block]).u32()
    } else {
        0
    };
    let item = e.call(EFFECT_ITEM_0084E3A0, &args![effect]).u32();
    let key = e.call(EFFECT_KEY_00403EA0, &args![item]).u32();
    e.mem.set_u32(entry.wrapping_add(0x10), key);
    let magnitude = e.call(MAGNITUDE_00495460, &args![effect]).f64();
    let magnitude = e.call(FTOL2, &args![magnitude]).u32();
    e.mem.set_u32(entry, magnitude);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_APPEND, &args![this.addr() + 4, slot]);
    });
}

// Translated from 007e0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a status effect list, `RET 4`, a key): the first entry whose first
/// word's `0084e3a0` is the key, or 0 (the walk advances once more past a match before it
/// stops).
pub fn fn_007e0c50(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut found = 0u32;
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while found == 0 && node != 0 && node_item(e, node) != 0 {
        let entry = node_item(e, node);
        let form = e.mem.u32(entry);
        if e.call(EFFECT_ITEM_0084E3A0, &args![form]).u32() == key {
            found = node_item(e, node);
        }
        node = node_next(e, node);
    }
    found
}

// Translated from 007e0d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::DoControllerSwitch` (engine map, vtable; `RET 4`, an unread word): the
/// healing mode is shown again (`007df620`).
pub fn stats_menu_do_controller_switch(e: &mut Engine, this: Ptr, _unused_1: u32) {
    stats_menu_update_healing_mode(e, this);
}

// Translated from 007e0d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStringT<char>::operator+=` with another string (engine map `operator+=` (ov2); `RET
/// 4`): a source with text is appended to a string with text (grown with `004037f0` when
/// the summed length (`004048e0`) passes the capacity (`004039a0`), copied by `00401460`
/// to the end, the length set by `004039c0`); a string without text takes the source
/// (`BSStringT::Set`). Returns `this`.
pub fn bs_string_operator_add_assign_ov2(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    if e.call(STRING_CHARS_00559450, &args![source]).u32() == 0 {
        return this;
    }
    if e.call(STRING_CHARS_00559450, &args![this]).u32() == 0 {
        e.call(STRING_SET, &args![this, source]);
        return this;
    }
    let own = e.call(STRING_LENGTH_004048E0, &args![this]).u32();
    let added = e.call(STRING_LENGTH_004048E0, &args![source]).u32();
    let length = own.wrapping_add(added);
    if length > e.call(STRING_CAPACITY_004039A0, &args![this]).u32() {
        let chars = e.call(STRING_CHARS_00559450, &args![this]).u32();
        e.call(STRING_GROW_004037F0, &args![this, chars, length]);
    }
    let count = e.call(STRING_COPY_LENGTH_004F4280, &args![source]).u32();
    let from = e.call(STRING_CHARS_00559450, &args![source]).u32();
    let own = e.call(STRING_LENGTH_004048E0, &args![this]).u32();
    let to = own.wrapping_add(e.mem.u32(this.addr()));
    e.call(COPY_00401460, &args![to, from, count]);
    e.call(STRING_SET_LENGTH_004039C0, &args![this, length]);
    this
}

// Translated from 007e1430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e1430 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// `ListBox<StatsMenu::StatusEffect_P>::AddItem` (engine map; `RET 0x10`; its
/// `Synchronize` `007254d0` and `SetTotalListHeight` `00725970` are in `menumanager.cpp`):
/// as `ListBox<BGSPerk_P>::AddItem` of `levelupmenu.cpp`.
pub fn list_box_status_effect_add_item(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(
        e,
        this,
        (data, text, compare, template),
        (
            STATUS_EFFECT_SYNCHRONIZE_007254D0,
            STATUS_EFFECT_SET_TOTAL_LIST_HEIGHT,
        ),
    )
}

// Translated from 007e16d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e16d0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the item allocations) is not translated.
/// Unnamed in the engine map (`RET 0x10`): the `ListBox` `AddItem` instance whose
/// `Synchronize` and `SetTotalListHeight` are `ListBox<TESReputation_P>`'s (`00725aa0`,
/// `00725f40`, `menumanager.cpp`); the reputation list (+0x270) adds through it. As
/// `ListBox<BGSPerk_P>::AddItem` of `levelupmenu.cpp`.
pub fn fn_007e16d0(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    list_box_add_item(
        e,
        this,
        (data, text, compare, template),
        (REPUTATION_SYNCHRONIZE, REPUTATION_SET_TOTAL_LIST_HEIGHT),
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007d_f1f0, fn_007df1f0(Ptr) -> f32),
        entry!(0x0040_6de0, fn_00406de0(Ptr)),
        entry!(0x007d_a2c0, stats_menu_create() -> u32),
        entry!(0x007d_b1c0, fn_007db1c0() -> u32),
        entry!(0x007d_b250, fn_007db250(Ptr, u32, u32)),
        entry!(0x007d_b270, fn_007db270(Ptr, u32, u32, u32)),
        entry!(0x007d_b2c0, fn_007db2c0(Ptr) -> Ptr),
        entry!(0x007d_b340, fn_007db340(Ptr) -> bool),
        entry!(0x007d_bfe0, fn_007dbfe0(Ptr, u32)),
        entry!(0x007d_c040, stats_menu_fade_control_callback(u32, u32) -> bool),
        entry!(0x007d_c0a0, fn_007dc0a0(Ptr) -> Ptr),
        entry!(0x007d_c4f0, stats_menu_do_idle(Ptr)),
        entry!(0x007d_cc00, fn_007dcc00(Ptr) -> i32),
        entry!(0x007d_cc40, fn_007dcc40(Ptr) -> i32),
        entry!(0x007d_cfc0, stats_menu_update_all_actor_values(Ptr)),
        entry!(0x007d_d090, stats_menu_update_actor_value(Ptr, u32)),
        entry!(0x007d_d710, stats_menu_update_perks()),
        entry!(0x007d_d8a0, fn_007dd8a0(Ptr) -> u8),
        entry!(0x007d_dad0, fn_007ddad0(Ptr, u32)),
        entry!(0x007d_db00, fn_007ddb00(Ptr)),
        entry!(0x007d_def0, fn_007ddef0() -> f32),
        entry!(0x007d_df00, stats_menu_update_active_effects(Ptr)),
        entry!(0x007d_f1b0, fn_007df1b0(Ptr) -> bool),
        entry!(0x007d_f1d0, fn_007df1d0(Ptr) -> u32),
        entry!(0x007d_f210, fn_007df210(Ptr) -> u32),
        entry!(0x007d_f230, stats_menu_update_healing_options(Ptr, u32)),
        entry!(0x007d_f4e0, stats_menu_update_player_name()),
        entry!(0x007d_f5d0, stats_menu_update_player_title()),
        entry!(0x007d_f620, stats_menu_update_healing_mode(Ptr)),
        entry!(0x007d_fa30, stats_menu_special_update_fn(u32, u32)),
        entry!(0x007d_fb00, stats_menu_skills_update_fn(u32, u32)),
        entry!(0x007d_fc60, fn_007dfc60(u32, u32)),
        entry!(0x007d_fcd0, fn_007dfcd0() -> u8),
        entry!(0x007d_fdf0, fn_007dfdf0(Ptr, u32) -> u32),
        entry!(0x007d_fe40, fn_007dfe40(Ptr) -> i32),
        entry!(0x007d_ff40, stats_menu_set_current_page(Ptr, i32)),
        entry!(0x007e_04e0, fn_007e04e0(u32, u32) -> i32),
        entry!(0x007e_05f0, fn_007e05f0(Ptr) -> bool),
        entry!(0x007e_07a0, fn_007e07a0(Ptr) -> bool),
        entry!(0x007e_07f0, fn_007e07f0(Ptr)),
        entry!(0x007e_0850, fn_007e0850(Ptr)),
        entry!(0x007e_09b0, fn_007e09b0(Ptr)),
        entry!(0x007e_0b20, stats_menu_status_data_list_register(Ptr, u32)),
        entry!(0x007e_0ce0, fn_007e0ce0(Ptr, u32) -> u32),
        entry!(0x007e_0e30, fn_007e0e30(Ptr, u32, u32)),
        entry!(0x007e_0e70, fn_007e0e70(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x007e_1110, fn_007e1110(Ptr, u32) -> u32),
        entry!(
            0x007e_1190,
            list_box_misc_stat_id_add_item(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0070_4390, stats_menu_toggle_healing_mode()),
        entry!(0x007d_b380, stats_menu_do_click(Ptr, i32, u32)),
        entry!(0x007d_b630, fn_007db630(Ptr, u32) -> u32),
        entry!(0x007d_b680, stats_menu_do_gamepad(Ptr, u32, u32) -> bool),
        entry!(0x007d_bfa0, fn_007dbfa0(Ptr, u32, u8)),
        entry!(0x007d_c010, fn_007dc010(Ptr, u32, u32) -> u32),
        entry!(0x007d_c0c0, fn_007dc0c0(Ptr) -> Ptr),
        entry!(0x007d_c0e0, stats_menu_do_gamepad_up_event(Ptr, u32, u32) -> bool),
        entry!(0x007d_c1b0, stats_menu_do_enter(Ptr, i32, u32)),
        entry!(0x007d_c490, stats_menu_do_leave(Ptr, u32, u32)),
        entry!(0x007d_c7e0, fn_007dc7e0(Ptr, u32)),
        entry!(0x007d_c9b0, fn_007dc9b0(Ptr, Ptr)),
        entry!(0x007d_cb40, fn_007dcb40(Ptr, u32)),
        entry!(0x007d_cc80, stats_menu_update_all(Ptr)),
        entry!(0x007d_d8c0, fn_007dd8c0(Ptr)),
        entry!(0x007d_da60, fn_007dda60()),
        entry!(0x007d_daa0, fn_007ddaa0(Ptr)),
        entry!(0x007d_dc20, fn_007ddc20(Ptr)),
        entry!(0x007d_dd10, fn_007ddd10(Ptr)),
        entry!(0x007d_de00, fn_007dde00(Ptr)),
        entry!(0x007d_f120, fn_007df120(Ptr, u32)),
        entry!(0x007d_f140, fn_007df140(Ptr, u32) -> Ptr),
        entry!(0x007d_f920, fn_007df920(Ptr)),
        entry!(0x007d_f9c0, fn_007df9c0(u32) -> u32),
        entry!(0x007d_fc90, stats_menu_set_knob_position(u8)),
        entry!(0x007d_fd00, stats_menu_set_current_stats_page(u8)),
        entry!(0x007d_fd20, stats_menu_move_knob(Ptr, u32, Ptr)),
        entry!(0x007e_0060, fn_007e0060(Ptr, i32) -> bool),
        entry!(0x007e_00e0, stats_menu_get_page_for(Ptr, u32, Ptr) -> bool),
        entry!(0x007e_0160, fn_007e0160(Ptr, u32, Ptr) -> bool),
        entry!(0x007e_0230, fn_007e0230(Ptr)),
        entry!(0x007e_02b0, stats_menu_toggle_gen_rep_mode(Ptr)),
        entry!(0x007e_0480, fn_007e0480(u32, u32) -> i32),
        entry!(0x007e_0510, fn_007e0510(u32, u32) -> i32),
        entry!(0x007e_0540, fn_007e0540(u32, u32) -> i32),
        entry!(0x007e_06b0, fn_007e06b0() -> u32),
        entry!(0x007e_06f0, fn_007e06f0(Ptr) -> bool),
        entry!(
            0x007e_08a0,
            stats_menu_status_effect_list_register(Ptr, u32)
        ),
        entry!(0x007e_09d0, fn_007e09d0(Ptr, u32)),
        entry!(0x007e_0a20, fn_007e0a20(Ptr, u32)),
        entry!(0x007e_0c50, fn_007e0c50(Ptr, u32) -> u32),
        entry!(0x007e_0d60, stats_menu_do_controller_switch(Ptr, u32)),
        entry!(0x007e_0d80, bs_string_operator_add_assign_ov2(Ptr, u32) -> Ptr),
        entry!(0x007e_1430, list_box_status_effect_add_item(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x007e_16d0, fn_007e16d0(Ptr, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_float_at_0x20() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x20, 13.25f32.to_bits());
        assert_eq!(e.call(0x007d_f1f0, &args![object]).f32(), 13.25);
    }

    #[test]
    fn quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x20, 0x7fa0_0001);
        let bits = e.call(0x007d_f1f0, &args![object]).f32().to_bits();
        assert_eq!(bits, 0x7fe0_0001);
    }
}

#[cfg(test)]
mod b069_tests {
    use super::*;

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

    /// The target of slot `slot` of fake vtable `index` (each slot its own double).
    fn slot(index: u32, slot: u32) -> u32 {
        0x0e00_0000 + index * 0x1000 + slot
    }

    fn fake_vtable(e: &mut Engine, index: u32) -> u32 {
        let vtable = e.mem.alloc(0x700);
        for offset in (0..0x700).step_by(4) {
            let target = slot(index, offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        vtable
    }

    fn fake_object(e: &mut Engine, index: u32) -> u32 {
        let vtable = fake_vtable(e, index);
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Every callee outside this file, doubled (0 or nothing); the list helpers act on the
    /// node words; the pages of the statics and constants are mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011a_0000,
            0x011d_e000,
            0x011d_0000,
            0x011d_1000,
            0x011d_2000,
            0x011d_3000,
            0x011d_4000,
            0x011d_5000,
            0x011d_b000,
            0x0101_1000,
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0102_0000,
            0x0102_3000,
            0x0106_4000,
            0x0107_0000,
            0x0107_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(PI_DOUBLE, std::f64::consts::PI);
        e.set_global(HALF_PI_DOUBLE, std::f64::consts::FRAC_PI_2);
        e.set_global(FIVE_DOUBLE, 5.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(VALUE_6006_DOUBLE, 6006.0f64);
        e.set_global(VALUE_102_DOUBLE, 102.0f64);
        for addr in [
            FREE,
            VECTOR_CONSTRUCT,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            FILL_00403D30,
            LIST_APPEND,
            LIST_REMOVE_FIRST_0063F7B0,
            LIST_FREE_NODES_00470470,
            STATUS_EFFECT_DELETE,
            TILE_ADD_FADE_CONTROL,
            TILE_REMOVE_FADE_CONTROL,
            TILE_SET_INT,
            TILE_GET_FLOAT,
            TILE_SET_FLOAT,
            TILE_SET_STRING,
            TILE_GET_STRING,
            ADD_USER_TRAIT,
            WORD_AT_98_00810FD0,
            FLAG_TEST_00403C80,
            LIST_BOX_GET_TILE_FOR,
            MISC_STAT_VALUE_005A3370,
            GET_CLAMPED_ACTOR_VALUE,
            AV_DERIVE_SKILL,
            ROUND_00406D90,
            EFFECT_KIND_0055B980,
            EFFECT_ACTIVE_00833C20,
            HARDCORE_004D1360,
            STATS_007E06B0,
            FLAG_004B71D0,
            INTERFACE_MANAGER_004B7210,
            WORD_AT_174_00704370,
            PIPBOY_007F86F0,
            BYTE_AT_28_00974D90,
            VALUE_005C5420,
            FLOAT_PAIR_00404010,
            FLOAT_PAIR_004BD510,
            STRING_CHARS_00408DA0,
            COMPARE_00404DC0,
            EFFECT_KEY_00403EA0,
            EFFECT_GET_MAGNITUDE,
            EFFECT_GET_DURATION,
            FLAG_TEST_00403E60,
            ENTRY_CONSTRUCT_00403BA0,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            STRING_FORMAT,
            STRING_ASSIGN_00438390,
            SETTING_STRING_00403DF0,
            PLAYER_LEVEL_0087F9F0,
            REFERENCE_NAME_0055D520,
            GET_KARMIC_TITLE,
            GET_INVENTORY_ITEM,
            GET_ITEM_COUNT_IN_CONTAINER,
            INVENTORY_ENTRY_DELETE,
            STATS_007E0230,
            PERK_RANK_SYNCHRONIZE,
            PERK_RANK_SET_TOTAL_LIST_HEIGHT,
            MISC_STAT_SYNCHRONIZE,
            MISC_STAT_SET_TOTAL_LIST_HEIGHT,
            TILE_GET_MENU,
            MENU_RENDER_TEMPLATE,
            TILE_GET_VALUE,
            ITEM_CONSTRUCT,
            LIST_INSERT_SORTED,
            LIST_APPEND_00905820,
            LIST_BOX_HAS_FLAG,
            INIT_TILE_POSITION,
            LIST_BOX_COUNT_00726C40,
            TILE_GET_VALUE_Q,
            VALUE_CLEAR_ACTIONS,
            VALUE_ADD_ACTION_TILE,
            VALUE_ADD_ACTION_FLOAT,
            VALUE_CALCULATE,
            REQUIRED_EXPERIENCE,
            ALIGNMENT_FOR_KARMA,
            VALUE_14_004A8F20,
            MENU_CLASS_004D5E00,
            TILE_GET_MENU_BY_CLASS,
            TILE_GET_MAXIMUM_DEPTH,
            WORD_AT_9C_00586150,
            TILE_READ_FILE,
            MENU_REMOVE_FROM_XML_ARCHIVE,
            MENU_00867A20,
            MENU_SET_MENU_TILE,
            DEBUG_MESSAGE,
            TUTORIAL_SHOW_MESSAGE,
            FIND_STAGE_FOR_RAD_LEVEL,
            WORD_AT_18_009611E0,
            TILE_SET_INTEGER_VALUE,
            LIST_BOX_INITIALIZE,
            AV_SECTION_COUNT,
            AV_TO_ACTOR_VALUE,
            AV_FLAG_TEST_00406D70,
            AV_LIST_BOX_ADD_ITEM,
            AV_NAME,
            TILE_SET_NAME_00707E10,
            MISC_STAT_NAME_0078A260,
            GET_DEFAULT_OBJECT,
            RT_DYNAMIC_CAST,
            METER_LABEL_007B52A0,
            MENU_PREP_FOR_VISIBILITY,
            DEHYDRATION_STAGE_004018B0,
            SLEEP_STAGE_00402760,
            HUNGER_STAGE_00402030,
            EFFECT_ITEM_0084E3A0,
            IS_USAGE_MONITOR_EFFECT,
            EFFECT_TARGET_0044DDC0,
            STAGE_GET_WORLDSPACE,
            STAGE_EFFECT_CONSTRUCT_007DF140,
            STAGE_EFFECT_ADD_007E0A20,
            EFFECT_FLAG_00403EE0,
            AV_ABBREVIATION,
            STATUS_EFFECT_LIST_REGISTER,
            TILE_DELETE_CHILDREN,
            STRING_APPEND_00404820,
            TILE_NAME_FROM_STRING_007DF120,
            STATUS_EFFECT_ADD_ITEM,
            ENTRY_STRING_004046B0,
            STRING_APPEND_STRING_007E0D80,
            STRING_LENGTH_004048E0,
            VATS_GET_COUNT,
            MIN_0042F5A0,
            STATUS_EFFECT_SYNCHRONIZE,
            INTERFACE_GET_PIPBOY,
            INTERFACE_MANAGER_IS_IN_PIPBOY_MENU,
            TIME_00457FE0,
            INTERFACE_IS_TOP_MENU_ID,
            STATS_007DDC20,
            STATS_007DDD10,
            STATS_007DDE00,
            STATS_007E0060,
            FO_RENDERED_MENU_TRIGGER_BURST,
            STATS_MENU_SET_KNOB_POSITION,
            STATS_MENU_SET_ICON_AND_DESCRIPTION,
            PERK_RANK_NUM_UNFILTERED,
            LIST_BOX_DATA_AT_007D86C0,
            STATS_007DC9B0,
            ITEM_CHANGE_007314C0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00ab_0034, |_, _| Ret::default());
        e.register(LIST_IS_EMPTY_008256D0, |_, _| 1u32.into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_14_00825C00, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(WORD_AT_28_0045CD60, |e, a| {
            e.mem.u32(a[0] + 0x28).into_ret()
        });
        e.register(STRING_CHARS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.register(LEVEL_CAP_0043D4D0, |_, _| 0x011d_af00u32.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        e
    }

    /// The player: a fake object (index 1) whose actor values (+0xa4) and active effects
    /// (+0x94) have fake vtables (indices 2 and 3).
    fn with_player(e: &mut Engine) -> u32 {
        let player = fake_object(e, 1);
        let values = fake_vtable(e, 2);
        e.mem.set_u32(player + 0xa4, values);
        let effects = fake_vtable(e, 3);
        e.mem.set_u32(player + 0x94, effects);
        e.set_global(PLAYER_011DEA3C, player);
        player
    }

    fn menu(e: &mut Engine) -> u32 {
        let menu = fake_object(e, 4);
        for index in 0..0x3cu32 {
            e.mem.set_u32(menu + 0x90 + 4 * index, 0x7000 + 4 * index);
        }
        e.mem.set_u32(menu + 4, 0x6100);
        for record in [0x34u32, 0x44, 0x54, 0x64] {
            e.mem.set_u32(menu + record, 0x00ab_0034);
        }
        let lists = fake_vtable(e, 8);
        for list in [0x180u32, 0x1b0, 0x1e0, 0x240] {
            e.mem.set_u32(menu + list, lists);
        }
        menu
    }

    #[test]
    fn the_status_data_list_destructor_frees_its_entries() {
        let mut e = engine();
        e.register(LIST_IS_EMPTY_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(LIST_REMOVE_FIRST_0063F7B0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, 0x5500);
        e.call_log = Some(vec![]);
        e.call(0x0040_6de0, &args![list]);
        assert_eq!(e.mem.u32(list), STATUS_DATA_LIST_VTABLE);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x5500]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![list + 4]]);
        // `007e07f0` deletes non-null items, `007e09b0` frees the nodes.
        e.mem.set_u32(list + 4, 0x5600);
        e.call(0x007e_07f0, &args![list]);
        assert_eq!(calls_to(&e, STATUS_EFFECT_DELETE), vec![vec![0x5600, 1]]);
        e.call(0x007e_09b0, &args![list]);
        assert_eq!(calls_to(&e, LIST_FREE_NODES_00470470), vec![vec![list + 4]]);
    }

    #[test]
    fn the_fade_object_is_made_once_and_groups_its_tiles() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let fade = e.call(0x007d_b1c0, &args![]).u32();
        assert_ne!(fade, 0);
        assert_eq!(e.call(0x007d_b1c0, &args![]).u32(), fade);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![fade + 4, 8, 6, LIST_CONSTRUCT, LIST_DESTRUCT]]
        );
        assert_eq!(
            calls_to(&e, FILL_00403D30),
            vec![vec![fade + 0x34, 0xffff_ffff, 0x18]]
        );
        e.register(LIST_APPEND, |e, a| e.mem.u32(a[1]).into_ret());
        e.call(0x007d_b250, &args![fade, 2u32, 0x6600u32]);
        let append = calls_to(&e, LIST_APPEND);
        assert_eq!(append[0][0], fade + 4 + 16);
        e.mem.set_u32(fade + 0x34 + 8, 0xffff_ffff);
        e.call(0x007d_b270, &args![fade, 2u32, 1u32, 0x1234_5605u32]);
        assert_eq!(e.mem.u32(fade + 0x3c), 0xffff_05ff);
        // Index 4 shifts by 32: the count wraps to 0 (the low byte).
        e.call(0x007d_b270, &args![fade, 2u32, 4u32, 0x07u32]);
        assert_eq!(e.mem.u32(fade + 0x3c), 0xffff_0507);
    }

    #[test]
    fn the_fade_callback_walks_the_current_group() {
        let mut e = engine();
        let fade = e.mem.alloc(0x4c);
        e.mem.set_u32(fade, 1);
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(fade + 4 + 8, 0x6100);
        e.mem.set_u32(fade + 4 + 8 + 4, node2);
        e.mem.set_u32(node2, 0x6200);
        e.call_log = Some(vec![]);
        e.call(0x007d_bfe0, &args![fade, 1u32]);
        let fades = calls_to(&e, TILE_ADD_FADE_CONTROL);
        assert_eq!(fades.len(), 2);
        assert_eq!(fades[1][..2], [0x6200, 0xfa9]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_c040, &args![0x6300u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x6300, 0xfa9]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6300, 0xfa9, 0xff]]);
    }

    #[test]
    fn the_small_getters() {
        let mut e = engine();
        let menu = menu(&mut e);
        assert!(e.call(0x007d_b340, &args![menu]).bool());
        e.mem.set_u32(menu + 0x90 + 4 * 0x3b, 0);
        assert!(!e.call(0x007d_b340, &args![menu]).bool());
        let counter = e.mem.alloc(4);
        assert_eq!(e.call(0x007d_c0a0, &args![counter]).u32(), counter);
        assert_eq!(e.mem.u32(counter), 1);
        e.register(TILE_GET_FLOAT, |_, _| st0(-2.75));
        assert_eq!(e.call(0x007d_cc00, &args![menu]).i32(), -2);
        assert_eq!(e.call(0x007d_cc40, &args![menu]).i32(), -2);
        assert_eq!(e.call(0x007d_fcd0, &args![]).u8(), 0);
        e.set_global(OPEN_MENU_011DACE0, menu);
        assert_eq!(e.call(0x007d_fcd0, &args![]).u8(), 0xfe);
        let perk = e.mem.alloc(0x40);
        e.mem.set_u8(perk + 0x3c, 3);
        assert_eq!(e.call(0x007d_d8a0, &args![perk]).u8(), 3);
        e.set_global(VALUE_01013974, 1000.0f32);
        assert_eq!(e.call(0x007d_def0, &args![]).f32(), 1000.0);
        for (value, expected) in [
            (0xcu32, 0x100au32),
            (0x10, 0x1009),
            (0x18, 0x100b),
            (0x11, 0x8000_0000),
        ] {
            assert_eq!(e.call(0x007d_fdf0, &args![menu, value]).u32(), expected);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x18, 0x80);
        assert_eq!(e.call(0x007d_f210, &args![object]).u32(), 1);
        e.mem.set_u32(object + 0x18, 0x7f);
        assert_eq!(e.call(0x007d_f210, &args![object]).u32(), 0);
        let setting = e.mem.alloc(0x20);
        e.mem.set_u32(setting + 0x14, 0x77);
        e.mem.set_u32(object + 0xc, setting);
        assert_eq!(e.call(0x007d_f1d0, &args![object]).u32(), 0x77);
        e.register(FLAG_TEST_00403C80, |_, a| {
            u32::from(a[1] == 0x40).into_ret()
        });
        assert!(e.call(0x007d_f1b0, &args![object]).bool());
    }

    #[test]
    fn do_idle_updates_the_meters_and_follows_hardcore_mode() {
        let mut e = engine();
        let player = with_player(&mut e);
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        e.register(BYTE_AT_28_00974D90, |_, _| 1u32.into_ret());
        e.register(INTERFACE_MANAGER_IS_IN_PIPBOY_MENU, |_, _| 1u32.into_ret());
        e.register(TIME_00457FE0, |_, _| 5000u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0x1007 { 0.25 } else { 0.0 })
        });
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        e.mem.set_u8(menu + 0x2a0, 1);
        e.mem.set_u8(menu + 0x2a1, 1);
        e.call_log = Some(vec![]);
        e.call(0x007d_c4f0, &args![menu]);
        assert_eq!(e.mem.u8(menu + 0x2a1), 0);
        assert_eq!(e.global::<u32>(NEXT_UPDATE_011DAD74), 5250);
        assert_eq!(calls_to(&e, STATS_007DDC20), vec![vec![menu]]);
        assert_eq!(calls_to(&e, STATS_007E0230), vec![vec![menu]]);
        // Page 0 and a skill page index of 0: no page change.
        assert!(calls_to(&e, STATS_007E0060).is_empty());
        // Hardcore came on: the three tiles get 1 on both traits.
        let sets: Vec<Vec<u32>> = calls_to(&e, TILE_SET_INT)
            .into_iter()
            .filter(|c| c[1] == 0xfaf || c[1] == 0xfa3)
            .collect();
        assert_eq!(sets.len(), 6);
        assert!(sets.iter().all(|c| c[2] == 1));
        assert_eq!(e.global::<u8>(HARDCORE_SHOWN_011A0AE1), 1);
        // Before the next update time nothing is updated; hardcore going off clears them.
        e.register(TIME_00457FE0, |_, _| 5100u32.into_ret());
        e.register(HARDCORE_004D1360, |_, _| Ret::default());
        e.register(TILE_GET_FLOAT, |_, a| {
            st0(if a[1] == 0x1004 { 3.0 } else { 0.0 })
        });
        e.call_log = Some(vec![]);
        e.call(0x007d_c4f0, &args![menu]);
        assert!(calls_to(&e, STATS_007DDC20).is_empty());
        let sets: Vec<Vec<u32>> = calls_to(&e, TILE_SET_INT)
            .into_iter()
            .filter(|c| c[1] == 0xfaf || c[1] == 0xfa3)
            .collect();
        assert_eq!(sets.len(), 6);
        assert!(sets.iter().all(|c| c[2] == 0));
        assert_eq!(e.global::<u8>(HARDCORE_SHOWN_011A0AE1), 0);
        let _ = player;
    }

    #[test]
    fn update_actor_value_shows_pairs_karma_and_body_parts() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(slot(2, AV_BASE), |_, a| (a[1] + 100).into_ret());
        e.register(slot(2, AV_VALUE), |_, a| (a[1] + 50).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0xcu32]);
        let formats = calls_to(&e, STRING_FORMAT);
        assert_eq!(formats[0][1..], [VALUE_PAIR_FORMAT, 0xc + 50, 0xc + 100]);
        assert_eq!(calls_to(&e, TILE_SET_STRING)[0][..2], [0x6100, 0x100a]);
        // Karma: alignment 2 sets its icon and name.
        e.register(ALIGNMENT_FOR_KARMA, |_, _| 2u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0x17u32]);
        let strings = calls_to(&e, TILE_SET_STRING);
        assert_eq!(
            strings[0],
            vec![e.mem.u32(menu + 0x120), 0xfcc, 0x0107_7418, 1]
        );
        assert_eq!(
            strings[1],
            vec![e.mem.u32(menu + 0x11c), 0xfc4, 0x011d_457d, 1]
        );
        // A body part: the broken image at 0 condition and the value on the next tile.
        e.register(slot(2, AV_FLOAT), |_, _| st0(0.0));
        e.mem
            .set_u32(BODY_PART_NAMES_011A0A80 + 4 * 0x1a, 0x0101_aaaa);
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0x1au32]);
        assert_eq!(
            calls_to(&e, STRING_FORMAT)[0][1..],
            [BODY_PART_BROKEN_FORMAT, 0x0101_aaaa]
        );
        assert_eq!(calls_to(&e, TILE_SET_STRING)[0][0], 0x7000 + 4 * 0xe);
        assert_eq!(calls_to(&e, TILE_SET_FLOAT)[0][0], 0x7000 + 4 * 0xf);
        // Another value: SPECIAL and skills updates on its list tiles.
        e.register(LIST_BOX_GET_TILE_FOR, |_, _| 0x6600u32.into_ret());
        e.register(WORD_AT_28_0045CD60, |_, _| Ret::default());
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0x22u32]);
        let lists: Vec<u32> = calls_to(&e, LIST_BOX_GET_TILE_FOR)
            .into_iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(lists, [menu + 0x180, menu + 0x1b0]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT)
                .into_iter()
                .filter(|c| c[1] == 0x1005)
                .count(),
            2
        );
    }

    #[test]
    fn update_actor_value_shows_the_level_cap_and_experience() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.set_global(0x011d_af00, 30u32);
        e.register(PLAYER_LEVEL_0087F9F0, |_, _| 0xffff_0004u32.into_ret());
        e.register(REQUIRED_EXPERIENCE, |_, a| (a[0] * 1000).into_ret());
        e.register(slot(2, AV_VALUE), |_, _| 3500u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0x18u32]);
        assert_eq!(calls_to(&e, REQUIRED_EXPERIENCE), vec![vec![5]]);
        assert_eq!(
            calls_to(&e, STRING_FORMAT)[0][1..],
            [VALUE_PAIR_FORMAT, 3500, 5000]
        );
        e.set_global(0x011d_af00, 4u32);
        e.call_log = Some(vec![]);
        e.call(0x007d_d090, &args![menu, 0x18u32]);
        assert_eq!(
            calls_to(&e, SETTING_STRING_00403DF0),
            vec![vec![MAX_LEVEL_SETTING_011D472C]]
        );
    }

    #[test]
    fn update_perks_lists_the_visible_ranks() {
        let mut e = engine();
        let player = with_player(&mut e);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x118, 0x6800);
        e.set_global(OPEN_MENU_011DACE0, menu);
        let list_vtable = fake_vtable(&mut e, 5);
        e.mem.set_u32(menu + 0x1e0, list_vtable);
        let visible = e.mem.alloc(0x40);
        let hidden = e.mem.alloc(0x40);
        e.mem.set_u8(hidden + 0x3c, 1);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, visible);
        e.mem.set_u8(first + 4, 2);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, hidden);
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(node2, second);
        let node1 = e.mem.alloc(8);
        e.mem.set_u32(node1, first);
        e.mem.set_u32(node1 + 4, node2);
        e.set_global(0x011d_af10, node1);
        e.register(slot(1, 0x4e8), |e, _| e.mem.u32(0x011d_af10).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d710, &args![]);
        assert_eq!(calls_to(&e, slot(5, 0x1c)), vec![vec![menu + 0x1e0]]);
        assert_eq!(calls_to(&e, STRING_FORMAT).len(), 1);
        assert_eq!(calls_to(&e, STRING_FORMAT)[0][1], TOPIC_ID_FORMAT);
        assert_eq!(calls_to(&e, STRING_FORMAT)[0][3], 2);
        // Without a container the list box's AddItem returns early.
        assert!(calls_to(&e, MENU_RENDER_TEMPLATE).is_empty());
        let _ = player;
    }

    #[test]
    fn the_misc_stat_tile_and_its_value() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.register(LIST_BOX_GET_TILE_FOR, |e, a| {
            u32::from(e.mem.u32(a[1]) == 9)
                .wrapping_mul(0x6700)
                .into_ret()
        });
        e.register(MISC_STAT_VALUE_005A3370, |_, a| (a[0] * 2).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_dad0, &args![menu, 9u32]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6700, 0x1005, 18]]);
        e.call_log = Some(vec![]);
        e.call(0x007d_dad0, &args![menu, 8u32]);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
    }

    #[test]
    fn the_radiation_meter_follows_the_x87_steps() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(TILE_GET_FLOAT, |_, _| st0(0.5));
        e.register(VALUE_005C5420, |_, _| st0(0.25));
        e.register(slot(2, AV_FLOAT), |_, _| st0(300.0));
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(FLOAT_PAIR_004BD510, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.set_global(KNOB_RATE_010780A4, 0.0005f32);
        e.call_log = Some(vec![]);
        e.call(0x007d_db00, &args![menu]);
        // a = 500, b = -250, c = 0.25 * 500 - 250 = -125, d = 300.
        let angle =
            (std::f64::consts::FRAC_PI_2 - (175.0f64 / 1000.0) * std::f64::consts::PI) as f32;
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6100, 0x1005, 300.0f32.to_bits(), 1]]
        );
        let knob = calls_to(&e, PIPBOY_007F86F0);
        assert_eq!(knob[0][1..], [1, angle.to_bits(), 0.0005f32.to_bits()]);
        // A d between 0 and 1 becomes 1.0 without the clamp call.
        e.register(slot(2, AV_FLOAT), |_, _| st0(0.5));
        e.call_log = Some(vec![]);
        e.call(0x007d_db00, &args![menu]);
        assert!(calls_to(&e, FLOAT_PAIR_004BD510).is_empty());
        assert_eq!(calls_to(&e, TILE_SET_FLOAT)[0][2], 1.0f32.to_bits());
    }

    #[test]
    fn update_active_effects_builds_the_stage_and_status_lists() {
        let mut e = engine();
        let _player = with_player(&mut e);
        let menu = menu(&mut e);
        let list_vtable = fake_vtable(&mut e, 5);
        e.mem.set_u32(menu + 0x240, list_vtable);
        // One active effect, in the radiation stage's world space.
        let effect = e.mem.alloc(0x40);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, effect);
        e.set_global(0x011d_af10, node);
        e.register(slot(3, 8), |e, _| e.mem.u32(0x011d_af10).into_ret());
        e.register(EFFECT_ACTIVE_00833C20, |_, _| 1u32.into_ret());
        e.register(FIND_STAGE_FOR_RAD_LEVEL, |_, _| 0x4400u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == TYPE_WORLD_01183060 {
                0x4500u32.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(STAGE_GET_WORLDSPACE, |_, _| 0x4500u32.into_ret());
        e.register(STAGE_EFFECT_CONSTRUCT_007DF140, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FLAG_TEST_00403E60, |_, a| {
            u32::from(a[1] == 0x2000).into_ret()
        });
        // The effect's setting (for `007df1d0`).
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(effect + 0xc, holder);
        e.call_log = Some(vec![]);
        e.call(0x007d_df00, &args![menu]);
        assert_eq!(e.mem.u8(menu + 0x2a1), 1);
        let made = e.mem.u32(menu + 0x74);
        assert_ne!(made, 0);
        assert_eq!(
            calls_to(&e, STAGE_EFFECT_ADD_007E0A20),
            vec![vec![made + 4, effect]]
        );
        // No abbreviation: only the setting flag 0x2000 registers it.
        assert_eq!(
            calls_to(&e, STATUS_EFFECT_LIST_REGISTER),
            vec![vec![menu + 0x84, effect]]
        );
        // The radiation tile is rebuilt (no entries) and shown; the others hidden.
        assert_eq!(
            calls_to(&e, TILE_DELETE_CHILDREN),
            vec![vec![e.mem.u32(menu + 0x110)]]
        );
        let shown: Vec<Vec<u32>> = calls_to(&e, TILE_SET_INT)
            .into_iter()
            .filter(|c| c[1] == 0xfa3)
            .collect();
        assert_eq!(
            shown,
            vec![
                vec![e.mem.u32(menu + 0x110), 0xfa3, 1],
                vec![e.mem.u32(menu + 0x168), 0xfa3, 0],
                vec![e.mem.u32(menu + 0x170), 0xfa3, 0],
                vec![e.mem.u32(menu + 0x178), 0xfa3, 0]
            ]
        );
        // No status effects: the container's height is 0.
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![e.mem.u32(menu + 0x114), 0xfb0, 0]));
        assert_eq!(
            calls_to(&e, STATUS_EFFECT_SYNCHRONIZE),
            vec![vec![menu + 0x240]]
        );
        // Once per frame.
        e.call_log = Some(vec![]);
        e.call(0x007d_df00, &args![menu]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn update_active_effects_describes_each_status_effect() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        let list_vtable = fake_vtable(&mut e, 5);
        e.mem.set_u32(menu + 0x240, list_vtable);
        let form = fake_object(&mut e, 6);
        e.register(slot(6, 0x130), |_, _| 0x0101_0130u32.into_ret());
        let status = e.mem.alloc(0x20);
        e.mem.set_u32(status, form);
        let entry = e.mem.alloc(0x14);
        e.mem.set_u32(entry, 5);
        e.mem.set_u32(entry + 0x10, 0x20);
        e.mem.set_u32(status + 8, entry);
        e.mem.set_u32(menu + 0x88, status);
        e.register(STATUS_EFFECT_ADD_ITEM, |_, _| 0x6900u32.into_ret());
        e.register(AV_ABBREVIATION, |_, a| (0x0101_0000 + a[0]).into_ret());
        e.register(LIST_BOX_COUNT_00726C40, |_, _| 3u32.into_ret());
        e.register(MIN_0042F5A0, |_, a| a[0].min(a[1]).into_ret());
        e.register(TILE_GET_FLOAT, |_, _| st0(20.0));
        e.call_log = Some(vec![]);
        e.call(0x007d_df00, &args![menu]);
        assert_eq!(
            calls_to(&e, STATUS_EFFECT_ADD_ITEM),
            vec![vec![menu + 0x240, status, 0, 0, 0]]
        );
        assert!(calls_to(&e, TILE_SET_STRING).contains(&vec![0x6900, 0x1004, 0x0101_0130, 1]));
        let formats = calls_to(&e, STRING_FORMAT);
        assert_eq!(formats[0][1..], [ABBREVIATION_FORMAT, 0x0101_0020, 5]);
        assert_eq!(calls_to(&e, STRING_APPEND_STRING_007E0D80).len(), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![e.mem.u32(menu + 0x114), 0xfb0, 60.0f32.to_bits(), 1]]
        );
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x6900, 0x1006, 0]));
    }

    #[test]
    fn update_healing_options_takes_the_entries_and_disables_without_items() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x28, 0x4100);
        e.mem.set_u32(menu + 0x2c, 0x6a00);
        e.mem.set_u32(menu + 0x30, 0x5000);
        e.mem.set_u32(menu + 0x34, 0x00ab_0034);
        e.mem.set_u32(menu + 0xb4, 0x6b00);
        e.register(0x00ab_0034, |_, _| 0xffff_ff01u32.into_ret());
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry + 4, 3);
        e.set_global(0x011d_af10, entry);
        e.register(GET_INVENTORY_ITEM, |e, _| e.mem.u32(0x011d_af10).into_ret());
        e.register(TILE_GET_STRING, |_, _| 0x0101_5555u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_f230, &args![menu, 0u32]);
        assert_eq!(calls_to(&e, INVENTORY_ENTRY_DELETE), vec![vec![0x5000, 1]]);
        assert_eq!(e.mem.u32(menu + 0x30), entry);
        assert_eq!(calls_to(&e, 0x00ab_0034), vec![vec![menu + 0x28]]);
        let sets = calls_to(&e, TILE_SET_INT);
        assert_eq!(sets[0], vec![0x6a00, 0xfaf, 1]);
        assert_eq!(sets[1], vec![0x6b00, 0xfaf, 1]);
        assert_eq!(
            calls_to(&e, STRING_FORMAT)[0][1..],
            [HEALING_FORMAT_01077548, 3, 0x0101_5555]
        );
        // Neither item carried: limb mode ends and the healing tile is disabled.
        assert_eq!(sets[2], vec![0x6b00, 0xfaf, 0]);
        e.mem.set_u8(menu + 0x2a0, 1);
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, a| {
            u32::from(a[1] == 0x4100).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007d_f230, &args![menu, 1u32]);
        assert!(calls_to(&e, STATS_007E0230).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x007d_f230, &args![menu, 3u32]);
        assert!(calls_to(&e, STATS_007E0230).is_empty());
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_f230, &args![menu, 4u32]);
        assert_eq!(calls_to(&e, STATS_007E0230), vec![vec![menu]]);
    }

    #[test]
    fn the_player_name_title_and_healing_mode() {
        let mut e = engine();
        let player = with_player(&mut e);
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007d_f4e0, &args![]);
        assert!(calls_to(&e, STRING_FORMAT).is_empty());
        e.set_global(OPEN_MENU_011DACE0, menu);
        e.register(PLAYER_LEVEL_0087F9F0, |_, _| 0x0001_0007u32.into_ret());
        e.register(REFERENCE_NAME_0055D520, |_, _| 0x0101_4444u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, a| (a[0] + 1).into_ret());
        e.call(0x007d_f4e0, &args![]);
        assert_eq!(
            calls_to(&e, STRING_FORMAT)[0][1..],
            [NAME_FORMAT_01077550, 0x0101_4444, SETTING_011D2028 + 1, 7]
        );
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![0x6100, 0x100c, 7]));
        e.mem.set_u32(menu + 0x124, 0x6c00);
        e.register(GET_KARMIC_TITLE, |_, _| 0x0101_7777u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_f5d0, &args![]);
        assert_eq!(calls_to(&e, GET_KARMIC_TITLE), vec![vec![player]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6c00, 0xfc4, 0x0101_7777, 1]]
        );
        // The healing mode in limb mode: button 10, the tiles disabled, the fade on.
        e.mem.set_u32(menu + 0xb4, 0x6b00);
        e.mem.set_u8(menu + 0x2a0, 1);
        e.register(FLAG_004B71D0, |_, _| 1u32.into_ret());
        e.register(ADD_USER_TRAIT, |_, a| (a[0] & 0xfff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_f620, &args![menu]);
        let sets = calls_to(&e, TILE_SET_INT);
        assert_eq!(sets[0], vec![0x6b00, 0x4d0, 0xa]);
        assert_eq!(sets.len(), 12);
        assert!(sets[1..].iter().all(|c| c[1] == 0x244 && c[2] == 0));
        assert_eq!(e.global::<u32>(HEALING_GUARD_011DAD80) & 3, 3);
        assert_eq!(calls_to(&e, TILE_SET_STRING)[0][2], SETTING_011D4708 + 1);
        assert_ne!(e.global::<u32>(FADE_OBJECT_011DACE4), 0);
    }

    #[test]
    fn the_special_and_skill_markers() {
        let mut e = engine();
        with_player(&mut e);
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| 6u32.into_ret());
        e.register(slot(2, AV_BASE), |_, _| 5u32.into_ret());
        e.register(WORD_AT_28_0045CD60, |_, _| 0x6e00u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_fa30, &args![0x6d00u32, 3u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x6d00, 0x1005, 6],
                vec![0x6d00, 0x1004, 1],
                vec![0x6e00, 0x1004, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6d00, 0x1006, ARROW_UP_01077560, 1]]
        );
        // Skills: 6 - 5 plus the rounded derived difference -3 is -2: the down arrow.
        e.register(AV_DERIVE_SKILL, |_, a| {
            st0(if a[2] == 0 { 10.0 } else { 13.0 })
        });
        e.register(ROUND_00406D90, |_, a| {
            (f32::from_bits(a[0]) as i32 as u32).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007d_fb00, &args![0x6d00u32, 0x20u32]);
        assert_eq!(
            calls_to(&e, ROUND_00406D90),
            vec![vec![(-3.0f32).to_bits()]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x6d00, 0x1006, ARROW_DOWN_0107755C, 1]]
        );
        // No tile: nothing.
        e.call_log = Some(vec![]);
        e.call(0x007d_fb00, &args![0u32, 0x20u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn the_face_number() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(slot(2, AV_BASE), |_, _| 100u32.into_ret());
        e.register(slot(2, AV_FLOAT), |_, _| st0(30.0));
        assert_eq!(e.call(0x007d_fe40, &args![menu]).i32(), 3);
        e.register(slot(2, AV_FLOAT), |_, _| st0(-50.0));
        assert_eq!(e.call(0x007d_fe40, &args![menu]).i32(), 4);
        e.register(slot(2, AV_FLOAT), |_, _| st0(150.0));
        assert_eq!(e.call(0x007d_fe40, &args![menu]).i32(), 0);
        let effect = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, effect);
        e.set_global(0x011d_af10, node);
        e.register(slot(3, 8), |e, _| e.mem.u32(0x011d_af10).into_ret());
        e.register(EFFECT_KIND_0055B980, |_, _| 0xau32.into_ret());
        e.register(EFFECT_ACTIVE_00833C20, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x007d_fe40, &args![menu]).i32(), 0xa);
    }

    #[test]
    fn set_current_page_shows_the_page_icons() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0xf4, 0x6f00);
        e.call_log = Some(vec![]);
        e.call(0x007d_ff40, &args![menu, 5i32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x007d_ff40, &args![menu, 1i32]);
        assert_eq!(
            calls_to(&e, STATS_MENU_SET_ICON_AND_DESCRIPTION),
            vec![vec![menu, 5]]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x6100, 0x1004, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x007d_ff40, &args![menu, 3i32]);
        assert_eq!(calls_to(&e, TILE_SET_INT)[1], vec![0x6f00, 0xfa3, 0]);
        e.register(PERK_RANK_NUM_UNFILTERED, |_, _| 2u32.into_ret());
        e.register(LIST_BOX_DATA_AT_007D86C0, |_, _| 0x4700u32.into_ret());
        e.register(BYTE_AT_28_00974D90, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_ff40, &args![menu, 3i32]);
        assert_eq!(calls_to(&e, STATS_007DC9B0), vec![vec![menu, 0x4700]]);
        assert_eq!(calls_to(&e, STATS_MENU_SET_KNOB_POSITION), vec![vec![1]]);
        assert_eq!(calls_to(&e, FO_RENDERED_MENU_TRIGGER_BURST).len(), 1);
    }

    #[test]
    fn the_option_callbacks_and_the_perk_order() {
        let mut e = engine();
        with_player(&mut e);
        let record = e.mem.alloc(0x10);
        assert!(!e.call(0x007e_05f0, &args![record]).bool());
        assert!(!e.call(0x007e_07a0, &args![record]).bool());
        e.mem.set_u32(record + 8, 0x5000);
        e.register(slot(2, AV_VALUE), |_, a| a[1].into_ret());
        e.register(slot(2, AV_BASE), |_, a| {
            u32::from(a[1] == 0x10).wrapping_mul(0x40).into_ret()
        });
        // Outside hardcore mode without `004b71d0` the value is 0x10: 0x10 < 0x40.
        e.register(STATS_007E06B0, |_, _| 0x12u32.into_ret());
        assert!(e.call(0x007e_05f0, &args![record]).bool());
        // In hardcore mode any value but -1 answers false.
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        assert!(!e.call(0x007e_05f0, &args![record]).bool());
        e.register(STATS_007E06B0, |_, _| 0xffff_ffffu32.into_ret());
        assert!(e.call(0x007e_05f0, &args![record]).bool());
        e.register(slot(2, AV_FLOAT), |_, _| st0(0.5));
        assert!(e.call(0x007e_07a0, &args![record]).bool());
        // The perk sort compares the names, the second looked up first.
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let rank1 = e.mem.alloc(4);
        let rank2 = e.mem.alloc(4);
        e.mem.set_u32(rank1, 0x4000);
        e.mem.set_u32(rank2, 0x4100);
        e.mem.set_u32(first + 4, rank1);
        e.mem.set_u32(second + 4, rank2);
        e.register(STRING_CHARS_00408DA0, |_, a| (a[0] + 1).into_ret());
        e.register(COMPARE_00404DC0, |_, a| {
            (a[0].wrapping_sub(a[1])).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_04e0, &args![first, second]).i32(), -0x100);
        assert_eq!(
            calls_to(&e, STRING_CHARS_00408DA0),
            vec![vec![0x4118], vec![0x4018]]
        );
    }

    #[test]
    fn the_status_data_list_registers_and_finds_entries() {
        let mut e = engine();
        e.register(LIST_APPEND, |e, a| {
            let entry = e.mem.u32(a[1]);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(a[0] + 4, node);
            Ret::default()
        });
        e.register(EFFECT_KEY_00403EA0, |_, _| 0x99u32.into_ret());
        e.register(EFFECT_GET_MAGNITUDE, |_, _| 7u32.into_ret());
        e.register(EFFECT_GET_DURATION, |_, _| 30u32.into_ret());
        e.register(ENTRY_CONSTRUCT_00403BA0, |_, a| a[0].into_ret());
        e.register(FLAG_TEST_00403E60, |_, a| u32::from(a[1] == 4).into_ret());
        let list = e.mem.alloc(0x10);
        let effect = e.mem.alloc(0x20);
        e.call(0x007e_0b20, &args![list, effect]);
        let entry = e.mem.u32(e.mem.u32(list + 8));
        assert_eq!(e.mem.u32(entry + 0x10), 0x99);
        assert_eq!(e.mem.u32(entry) as i32, -7);
        assert_eq!(e.mem.u32(entry + 8), 30);
        // The list's first node is +4: the appended node is its next.
        e.mem.set_u32(list + 4, entry);
        assert_eq!(e.call(0x007e_0ce0, &args![list, 0x99u32]).u32(), entry);
        assert_eq!(e.call(0x007e_0ce0, &args![list, 0x98u32]).u32(), 0);
        e.call(0x007e_0b20, &args![list, effect]);
        assert_eq!(e.mem.u32(entry) as i32, 0);
        e.call(0x007e_0b20, &args![list, 0u32]);
    }

    #[test]
    fn the_list_box_instances_add_items_with_their_own_helpers() {
        let mut e = engine();
        e.register(TILE_GET_MENU, |_, _| 0x5000u32.into_ret());
        e.register(MENU_RENDER_TEMPLATE, |_, _| 0x6000u32.into_ret());
        e.register(LIST_BOX_HAS_FLAG, |_, _| 1u32.into_ret());
        e.register(ITEM_CONSTRUCT, |_, a| a[0].into_ret());
        for (address, set_total) in [
            (0x007e_0e70u32, PERK_RANK_SET_TOTAL_LIST_HEIGHT),
            (0x007e_1190, MISC_STAT_SET_TOTAL_LIST_HEIGHT),
        ] {
            let list = e.mem.alloc(0x30);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(address, &args![list, 1u32, 0u32, 0u32, 0u32]).u32(),
                0
            );
            e.mem.set_u32(list + 0xc, 0x3000);
            e.mem.set_u32(list + 0x18, 0x4100);
            assert_eq!(
                e.call(address, &args![list, 1u32, 0u32, 0u32, 0u32]).u32(),
                0x6000
            );
            assert_eq!(calls_to(&e, set_total), vec![vec![list]]);
            assert_eq!(e.mem.u16(list + 0x1c), 1);
        }
        let list = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(
            0x007e_1190,
            &args![list, 1u32, 0u32, 0x00ab_cdefu32, 0x4300u32],
        );
        assert!(calls_to(&e, MISC_STAT_SYNCHRONIZE).is_empty());
        e.mem.set_u32(list + 0xc, 0x3000);
        e.call(
            0x007e_1190,
            &args![list, 1u32, 0u32, 0x00ab_cdefu32, 0x4300u32],
        );
        assert_eq!(calls_to(&e, MISC_STAT_SYNCHRONIZE), vec![vec![list]]);
    }

    #[test]
    fn fn_007e1110_finds_the_data_of_a_tile() {
        let mut e = engine();
        let list = e.mem.alloc(0x30);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x6100);
        e.mem.set_u32(first + 4, 0x11);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x6200);
        e.mem.set_u32(second + 4, 0x22);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(list + 4, first);
        e.mem.set_u32(list + 8, node);
        assert_eq!(e.call(0x007e_1110, &args![list, 0x6200u32]).u32(), 0x22);
        assert_eq!(e.call(0x007e_1110, &args![list, 0x6100u32]).u32(), 0x11);
        assert_eq!(e.call(0x007e_1110, &args![list, 0x6300u32]).u32(), 0);
        assert_eq!(e.call(0x007e_1110, &args![list, 0u32]).u32(), 0);
    }

    #[test]
    fn update_all_actor_values_covers_every_value_and_both_lists() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007d_cfc0, &args![menu]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_007314C0),
            vec![
                vec![menu + 0x180, 0x007d_fa30, 0, 0x7fff_ffff],
                vec![menu + 0x1b0, 0x007d_fb00, 0, 0x7fff_ffff]
            ]
        );
        // The value pairs of 0x10 and 0xc, the karma alignment once, the body parts 0x19 to
        // 0x1e.
        let pairs = calls_to(&e, STRING_FORMAT)
            .into_iter()
            .filter(|c| c[1] == VALUE_PAIR_FORMAT)
            .count();
        assert_eq!(pairs, 2);
        assert_eq!(calls_to(&e, ALIGNMENT_FOR_KARMA).len(), 1);
        let parts = calls_to(&e, STRING_FORMAT)
            .into_iter()
            .filter(|c| c[1] == BODY_PART_BROKEN_FORMAT)
            .count();
        assert_eq!(parts, 6);
    }

    #[test]
    fn create_fails_early_or_builds_the_menu() {
        let mut e = engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x00ab_0100u32.into_ret());
        e.map(0x00ab_0000, 0x1000);
        e.mem.set_u32(0x00ab_0100, 0x00ab_0200);
        e.mem.set_u32(0x00ab_0200, 0x00ab_0300);
        e.register(0x00ab_0300, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a2c0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00ab_0300), vec![vec![0x00ab_0100, 1]]);
        // A menu whose class matches but that misses a tile.
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| Ret::default());
        e.set_global(0x011d_af10, menu);
        e.register(TILE_GET_MENU, |e, _| e.mem.u32(0x011d_af10).into_ret());
        e.register(TILE_READ_FILE, |_, _| 0x6400u32.into_ret());
        e.mem.set_u32(menu + 0x90 + 4 * 5, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a2c0, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGE),
            vec![vec![CREATION_FAILED_TEXT]]
        );
        assert_eq!(e.global::<u32>(OPEN_MENU_011DACE0), menu);
        // With every tile: the whole set-up runs and the tile is returned.
        e.mem.set_u32(menu + 0x90 + 4 * 5, 0x7014);
        for list in [0x180u32, 0x1b0, 0x1e0, 0x240] {
            let vtable = fake_vtable(&mut e, 7 + list / 0x30);
            e.mem.set_u32(menu + list, vtable);
        }
        e.register(FIND_STAGE_FOR_RAD_LEVEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a2c0, &args![]).u32(), 0x6400);
        assert_eq!(
            calls_to(&e, TILE_SET_INTEGER_VALUE),
            vec![vec![0x6400, 0x1006, 1000]]
        );
        assert_eq!(calls_to(&e, MISC_STAT_NAME_0078A260).len(), 0x2b);
        assert_eq!(
            calls_to(&e, GET_DEFAULT_OBJECT),
            vec![vec![0], vec![0x15], vec![2], vec![3]]
        );
        assert_eq!(e.mem.u32(menu + 0x34), 0x007e_05f0);
        assert_eq!(e.mem.u32(menu + 0x64), 0x007e_06f0);
        assert_eq!(e.mem.u32(menu + 0x54), 0x007e_07a0);
        assert_eq!(e.mem.u32(menu + 0x2c), e.mem.u32(menu + 0xb8));
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 1]]);
        assert_eq!(calls_to(&e, METER_LABEL_007B52A0).len(), 4);
        let fade = e.global::<u32>(FADE_OBJECT_011DACE4);
        assert_eq!(e.mem.u32(fade + 0x34), 0x0301_0200);
    }
}

#[cfg(test)]
mod n058_tests {
    use super::*;

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

    /// The target of slot `slot` of fake vtable `index` (each slot its own double).
    fn slot(index: u32, slot: u32) -> u32 {
        0x0e00_0000 + index * 0x1000 + slot
    }

    fn fake_vtable(e: &mut Engine, index: u32) -> u32 {
        let vtable = e.mem.alloc(0x700);
        for offset in (0..0x700).step_by(4) {
            let target = slot(index, offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        vtable
    }

    fn fake_object(e: &mut Engine, index: u32) -> u32 {
        let vtable = fake_vtable(e, index);
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Every callee outside this file, doubled (0 or nothing); the list helpers act on the
    /// node words; the pages of the statics and constants are mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011a_0000,
            0x011d_e000,
            0x011d_0000,
            0x011d_1000,
            0x011d_2000,
            0x011d_3000,
            0x011d_4000,
            0x011d_5000,
            0x011d_b000,
            0x0101_1000,
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0102_0000,
            0x0102_3000,
            0x0106_4000,
            0x0107_0000,
            0x0107_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(PI_DOUBLE, std::f64::consts::PI);
        e.set_global(HALF_PI_DOUBLE, std::f64::consts::FRAC_PI_2);
        e.set_global(FIVE_DOUBLE, 5.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(VALUE_6006_DOUBLE, 6006.0f64);
        e.set_global(VALUE_102_DOUBLE, 102.0f64);
        for addr in [
            FREE,
            VECTOR_CONSTRUCT,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            FILL_00403D30,
            LIST_APPEND,
            LIST_REMOVE_FIRST_0063F7B0,
            LIST_FREE_NODES_00470470,
            STATUS_EFFECT_DELETE,
            TILE_ADD_FADE_CONTROL,
            TILE_REMOVE_FADE_CONTROL,
            TILE_SET_INT,
            TILE_GET_FLOAT,
            TILE_SET_FLOAT,
            TILE_SET_STRING,
            TILE_GET_STRING,
            ADD_USER_TRAIT,
            WORD_AT_98_00810FD0,
            FLAG_TEST_00403C80,
            LIST_BOX_GET_TILE_FOR,
            MISC_STAT_VALUE_005A3370,
            GET_CLAMPED_ACTOR_VALUE,
            AV_DERIVE_SKILL,
            ROUND_00406D90,
            EFFECT_KIND_0055B980,
            EFFECT_ACTIVE_00833C20,
            HARDCORE_004D1360,
            STATS_007E06B0,
            FLAG_004B71D0,
            INTERFACE_MANAGER_004B7210,
            WORD_AT_174_00704370,
            PIPBOY_007F86F0,
            BYTE_AT_28_00974D90,
            VALUE_005C5420,
            FLOAT_PAIR_00404010,
            FLOAT_PAIR_004BD510,
            STRING_CHARS_00408DA0,
            COMPARE_00404DC0,
            EFFECT_KEY_00403EA0,
            EFFECT_GET_MAGNITUDE,
            EFFECT_GET_DURATION,
            FLAG_TEST_00403E60,
            ENTRY_CONSTRUCT_00403BA0,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            STRING_FORMAT,
            STRING_ASSIGN_00438390,
            SETTING_STRING_00403DF0,
            PLAYER_LEVEL_0087F9F0,
            REFERENCE_NAME_0055D520,
            GET_KARMIC_TITLE,
            GET_INVENTORY_ITEM,
            GET_ITEM_COUNT_IN_CONTAINER,
            INVENTORY_ENTRY_DELETE,
            STATS_007E0230,
            PERK_RANK_SYNCHRONIZE,
            PERK_RANK_SET_TOTAL_LIST_HEIGHT,
            MISC_STAT_SYNCHRONIZE,
            MISC_STAT_SET_TOTAL_LIST_HEIGHT,
            TILE_GET_MENU,
            MENU_RENDER_TEMPLATE,
            TILE_GET_VALUE,
            ITEM_CONSTRUCT,
            LIST_INSERT_SORTED,
            LIST_APPEND_00905820,
            LIST_BOX_HAS_FLAG,
            INIT_TILE_POSITION,
            LIST_BOX_COUNT_00726C40,
            TILE_GET_VALUE_Q,
            VALUE_CLEAR_ACTIONS,
            VALUE_ADD_ACTION_TILE,
            VALUE_ADD_ACTION_FLOAT,
            VALUE_CALCULATE,
            REQUIRED_EXPERIENCE,
            ALIGNMENT_FOR_KARMA,
            VALUE_14_004A8F20,
            MENU_CLASS_004D5E00,
            TILE_GET_MENU_BY_CLASS,
            TILE_GET_MAXIMUM_DEPTH,
            WORD_AT_9C_00586150,
            TILE_READ_FILE,
            MENU_REMOVE_FROM_XML_ARCHIVE,
            MENU_00867A20,
            MENU_SET_MENU_TILE,
            DEBUG_MESSAGE,
            TUTORIAL_SHOW_MESSAGE,
            FIND_STAGE_FOR_RAD_LEVEL,
            WORD_AT_18_009611E0,
            TILE_SET_INTEGER_VALUE,
            LIST_BOX_INITIALIZE,
            AV_SECTION_COUNT,
            AV_TO_ACTOR_VALUE,
            AV_FLAG_TEST_00406D70,
            AV_LIST_BOX_ADD_ITEM,
            AV_NAME,
            TILE_SET_NAME_00707E10,
            MISC_STAT_NAME_0078A260,
            GET_DEFAULT_OBJECT,
            RT_DYNAMIC_CAST,
            METER_LABEL_007B52A0,
            MENU_PREP_FOR_VISIBILITY,
            DEHYDRATION_STAGE_004018B0,
            SLEEP_STAGE_00402760,
            HUNGER_STAGE_00402030,
            EFFECT_ITEM_0084E3A0,
            IS_USAGE_MONITOR_EFFECT,
            EFFECT_TARGET_0044DDC0,
            STAGE_GET_WORLDSPACE,
            STAGE_EFFECT_CONSTRUCT_007DF140,
            STAGE_EFFECT_ADD_007E0A20,
            EFFECT_FLAG_00403EE0,
            AV_ABBREVIATION,
            STATUS_EFFECT_LIST_REGISTER,
            TILE_DELETE_CHILDREN,
            STRING_APPEND_00404820,
            TILE_NAME_FROM_STRING_007DF120,
            STATUS_EFFECT_ADD_ITEM,
            ENTRY_STRING_004046B0,
            STRING_APPEND_STRING_007E0D80,
            STRING_LENGTH_004048E0,
            VATS_GET_COUNT,
            MIN_0042F5A0,
            STATUS_EFFECT_SYNCHRONIZE,
            INTERFACE_GET_PIPBOY,
            INTERFACE_MANAGER_IS_IN_PIPBOY_MENU,
            TIME_00457FE0,
            INTERFACE_IS_TOP_MENU_ID,
            STATS_007DDC20,
            STATS_007DDD10,
            STATS_007DDE00,
            STATS_007E0060,
            FO_RENDERED_MENU_TRIGGER_BURST,
            STATS_MENU_SET_KNOB_POSITION,
            STATS_MENU_SET_ICON_AND_DESCRIPTION,
            PERK_RANK_NUM_UNFILTERED,
            LIST_BOX_DATA_AT_007D86C0,
            STATS_007DC9B0,
            ITEM_CHANGE_007314C0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00ab_0034, |_, _| Ret::default());
        e.register(LIST_IS_EMPTY_008256D0, |_, _| 1u32.into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_14_00825C00, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(WORD_AT_28_0045CD60, |e, a| {
            e.mem.u32(a[0] + 0x28).into_ret()
        });
        e.register(STRING_CHARS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 0x100).into_ret());
        e.register(LEVEL_CAP_0043D4D0, |_, _| 0x011d_af00u32.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32).into_ret()
        });
        e
    }

    /// The player: a fake object (index 1) whose actor values (+0xa4) and active effects
    /// (+0x94) have fake vtables (indices 2 and 3).
    fn with_player(e: &mut Engine) -> u32 {
        let player = fake_object(e, 1);
        let values = fake_vtable(e, 2);
        e.mem.set_u32(player + 0xa4, values);
        let effects = fake_vtable(e, 3);
        e.mem.set_u32(player + 0x94, effects);
        e.set_global(PLAYER_011DEA3C, player);
        player
    }

    fn menu(e: &mut Engine) -> u32 {
        let menu = fake_object(e, 4);
        for index in 0..0x3cu32 {
            e.mem.set_u32(menu + 0x90 + 4 * index, 0x7000 + 4 * index);
        }
        e.mem.set_u32(menu + 4, 0x6100);
        for record in [0x34u32, 0x44, 0x54, 0x64] {
            e.mem.set_u32(menu + record, 0x00ab_0034);
        }
        let lists = fake_vtable(e, 8);
        for list in [0x180u32, 0x1b0, 0x1e0, 0x240] {
            e.mem.set_u32(menu + list, lists);
        }
        menu
    }

    /// Every callee of this batch outside the file, doubled (0 or nothing), on top of
    /// `engine`'s; the batch's statics mapped.
    fn batch_engine() -> Engine {
        let mut e = engine();
        for page in [0x011c_3000, 0x0107_4000, 0x0101_8000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEAD_ZONE_DOUBLE, 8689.0f64);
        e.set_global(FLOAT_MAX_01016970, f32::MAX);
        for addr in [
            TILE_IS_TRUE,
            TILE_UPDATE_TILE,
            TILE_POKE,
            TILE_GET_CHILD_BY_NAME,
            TILE_PLAY_TILE_SOUND,
            TILE_IS_VISIBLE,
            AUDIO_INSTANCE,
            GET_SOUND_HANDLE_BY_NAME,
            SOUND_HANDLE_PLAY,
            SOUND_HANDLE_DESTROY,
            PLAY_MENU_SOUND,
            SET_MAP_MENU_VISIBLE,
            SET_INVENTORY_MENU_VISIBLE,
            PIPBOY_SET_TAB_KNOB_POSITION,
            PIPBOY_MOVE_KNOB,
            LIST_CONTAINS_005F65D0,
            FIELD_SET_00501C10,
            LIST_CONSTRUCT_00406DB0,
            LIST_CONSTRUCT_0071F540,
            FLAG_0070ED80,
            MAGNITUDE_00495460,
            REPUTATION_ICON_006166E0,
            REPUTATION_GET_TITLE,
            REPUTATION_VALUE_006155C0,
            REPUTATION_LIST_009C1A50,
            AV_GET_ICON,
            AV_GET_DESCRIPTION,
            AV_GET_BADGE,
            SCROLL_VALUE_004E4430,
            COMPARE_00408B20,
            ACTOR_QUEUE_EQUIP_OBJECT,
            FLAG_00709D40,
            INPUT_00877720,
            STICK_00A23390,
            ABS_00408820,
            STRING_CAPACITY_004039A0,
            STRING_GROW_004037F0,
            STRING_COPY_LENGTH_004F4280,
            COPY_00401460,
            STRING_SET_LENGTH_004039C0,
            STRING_SET,
            LIST_BOX_HAS_TILE_007242F0,
            STATUS_EFFECT_SYNCHRONIZE_007254D0,
            STATUS_EFFECT_SET_TOTAL_LIST_HEIGHT,
            REPUTATION_SYNCHRONIZE,
            REPUTATION_SET_TOTAL_LIST_HEIGHT,
            TILE_TEXT_TO_TRAIT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // The description box's slots: +4 of the box is the slot itself.
        e.register(TEXT_TILE_SLOT_0073AC90, |_, a| a[0].into_ret());
        e.register(SCROLL_TILE_SLOT_007B5370, |_, a| (a[0] + 4).into_ret());
        // This file's own functions, over the doubles `engine` gives some of them.
        for (addr, f) in funcs() {
            e.register(addr, f);
        }
        e
    }

    /// `Tile::GetFloat` answering from a table of (tile, trait) pairs, 0.0 for the rest.
    fn tile_floats(e: &mut Engine, values: Vec<((u32, u32), f64)>) {
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            let value = values
                .iter()
                .find(|(key, _)| *key == (a[0], a[1]))
                .map_or(0.0, |(_, value)| *value);
            st0(value)
        });
    }

    fn set_ints(e: &Engine, tile: u32) -> Vec<Vec<u32>> {
        calls_to(e, TILE_SET_INT)
            .into_iter()
            .filter(|words| words[0] == tile)
            .collect()
    }

    #[test]
    fn toggle_healing_mode_needs_an_open_menu_in_limb_mode() {
        let mut e = batch_engine();
        with_player(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0070_4390, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        e.call(0x0070_4390, &args![]);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.mem.set_u8(menu + 0x2a0, 1);
        e.call(0x0070_4390, &args![]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 0);
    }

    #[test]
    fn toggle_limb_mode_updates_both_healing_tiles() {
        let mut e = batch_engine();
        let player = with_player(&mut e);
        let values = e.mem.u32(player + 0xa4);
        // Actor value 0x1b is below its base.
        e.register(e.mem.u32(values + 8), |_, a| {
            u32::from(a[1] == 0x1b).wrapping_neg().into_ret()
        });
        e.register(FLAG_0070ED80, |_, _| 0u32.into_ret());
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x2c, 0x5100);
        e.mem.set_u32(menu + 0x5c, 0x5200);
        e.mem.set_u32(menu + 0x60, 1);
        e.call_log = Some(vec![]);
        e.call(0x007e_0230, &args![menu]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 1);
        assert_eq!(set_ints(&e, 0x5100), vec![vec![0x5100, 0xfaf, 0]]);
        assert_eq!(set_ints(&e, 0x5200), vec![vec![0x5200, 0xfaf, 1]]);
        e.call(0x007e_0230, &args![menu]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 0);
    }

    #[test]
    fn the_limb_option_needs_an_entry_and_a_hurt_limb() {
        let mut e = batch_engine();
        let player = with_player(&mut e);
        let values = e.mem.u32(player + 0xa4);
        let record = e.mem.alloc(0x10);
        assert!(!fn_007e06f0(&mut e, Ptr::new(record)));
        e.mem.set_u32(record + 8, 1);
        assert!(!fn_007e06f0(&mut e, Ptr::new(record)));
        e.register(e.mem.u32(values + 8), |_, a| {
            u32::from(a[1] == 0x1e).wrapping_neg().into_ret()
        });
        assert!(fn_007e06f0(&mut e, Ptr::new(record)));
        // Outside hardcore mode a picked limb answers false.
        e.register(FLAG_0070ED80, |_, _| 1u32.into_ret());
        e.register(STRING_CHARS_00559450, |_, _| 2u32.into_ret());
        assert!(!fn_007e06f0(&mut e, Ptr::new(record)));
        assert_eq!(fn_007e06b0(&mut e), 0x1a);
        e.register(FLAG_0070ED80, |_, _| 0u32.into_ret());
        assert_eq!(fn_007e06b0(&mut e), 0xffff_ffff);
    }

    #[test]
    fn the_limb_groups_and_their_actor_values() {
        let mut e = batch_engine();
        let values: Vec<u32> = (0..7).map(|group| fn_007df9c0(&mut e, group)).collect();
        assert_eq!(
            values,
            vec![0x19, 0x1c, 0x1a, 0x1b, 0x1e, 0x1d, 0xffff_ffff]
        );
    }

    #[test]
    fn the_lowest_limb_becomes_the_fade_group() {
        let mut e = batch_engine();
        let player = with_player(&mut e);
        let values = e.mem.u32(player + 0xa4);
        // Limb values: 0x1a (group 2) and 0x1d (group 5) tie at 20; the first wins.
        e.register(e.mem.u32(values + 0xc), |_, a| {
            st0(match a[1] {
                0x1a | 0x1d => 20.0,
                _ => 80.0,
            })
        });
        let menu = menu(&mut e);
        e.call(0x007d_f920, &args![menu]);
        let object = fn_007db1c0(&mut e);
        assert_eq!(e.mem.u32(object), 2);
        // A NaN never wins.
        e.register(e.mem.u32(values + 0xc), |_, _| st0(f64::NAN));
        e.call(0x007d_f920, &args![menu]);
        assert_eq!(e.mem.u32(object), 6);
    }

    #[test]
    fn the_fade_group_helpers() {
        let mut e = batch_engine();
        let object = fn_007db1c0(&mut e);
        e.mem.set_u32(object + 0x38, 0x4433_2211);
        assert_eq!(e.call(0x007d_c010, &args![object, 1u32, 2u32]).u32(), 0x33);
        // The shift is taken modulo 32: index 4 is index 0.
        assert_eq!(e.call(0x007d_c010, &args![object, 1u32, 4u32]).u32(), 0x11);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 5);
        assert_eq!(e.call(0x007d_c0c0, &args![word]).u32(), word);
        assert_eq!(e.mem.u32(word), 4);
        e.call_log = Some(vec![]);
        e.call(0x007d_bfa0, &args![object, 3u32, 0u32]);
        assert_eq!(e.mem.u32(object), 3);
        e.call(0x007d_bfa0, &args![object, 1u32, 1u32]);
        assert_eq!(e.mem.u32(object), 1);
        // Group 1's list holds tile 0x7777: found through `005f65d0`.
        e.register(LIST_CONTAINS_005F65D0, |e, a| {
            u32::from(
                a[0] == e.global::<u32>(FADE_OBJECT_011DACE4) + 0xc && e.mem.u32(a[1]) == 0x7777,
            )
            .into_ret()
        });
        assert_eq!(e.call(0x007d_b630, &args![object, 0x7777u32]).u32(), 1);
        assert_eq!(e.call(0x007d_b630, &args![object, 0x7778u32]).u32(), 6);
    }

    #[test]
    fn do_click_pages_status_pages_limb_mode_and_the_lists() {
        let mut e = batch_engine();
        e.register(STATS_MENU_SET_ICON_AND_DESCRIPTION, |_, _| Ret::default());
        with_player(&mut e);
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        e.mem.set_u32(menu + 0xa4, 0x5a00);
        // Disabled tile: nothing happens.
        e.register(FLAG_004B71D0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 2u32, 0u32]);
        assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x7008, 0xfaf]]);
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        e.register(FLAG_004B71D0, |_, _| 0u32.into_ret());
        // A page button.
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 2u32, 0u32]);
        assert!(set_ints(&e, 0x6100).contains(&vec![0x6100, 0x1004, 2]));
        // A status page button (0x37 is sub page 4: past 2 outside hardcore mode).
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 0x1du32, 0u32]);
        assert_eq!(set_ints(&e, 0x5a00), vec![vec![0x5a00, 0x1004, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 0x37u32, 0u32]);
        assert!(set_ints(&e, 0x5a00).is_empty());
        // Id 9 toggles limb mode.
        e.call(0x007d_b380, &args![menu, 9u32, 0u32]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 1);
        e.call(0x007d_b380, &args![menu, 5u32, 0u32]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 0);
        // Id 0x31 toggles the general and reputation lists.
        e.call(0x007d_b380, &args![menu, 0x31u32, 0u32]);
        assert_eq!(e.global::<u8>(GEN_REP_MODE_011A0AE0), 1);
    }

    #[test]
    fn do_click_on_a_healing_option_equips_its_item() {
        let mut e = batch_engine();
        let player = with_player(&mut e);
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        // Option 1: tile 0x5500, entry 0x00ab_1000 whose extra lists start at node 0x00ab_1100.
        e.map(0x00ab_1000, 0x1000);
        e.mem.set_u32(menu + 0x3c, 0x5500);
        e.mem.set_u32(menu + 0x40, 0x00ab_1000);
        e.mem.set_u32(0x00ab_1000, 0x00ab_1100);
        e.mem.set_u32(0x00ab_1100, 0x00ab_1200);
        e.mem.set_u32(0x00ab_1004, 3);
        e.register(STRING_CHARS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(EFFECT_TARGET_0044DDC0, |_, _| 0x00ab_1300u32.into_ret());
        // The cast effect list: one node with effect 0x00ab_1400.
        e.register(RT_DYNAMIC_CAST, |_, _| 0x00ab_1500u32.into_ret());
        e.mem.set_u32(0x00ab_1500, 0x00ab_1400);
        e.register(EFFECT_KEY_00403EA0, |_, _| 0x30u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 0x7fu32, 0x5500u32]);
        assert_eq!(
            calls_to(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player, 0x00ab_1300, 3, 0x00ab_1200, 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[0],
            vec![0x00ab_1300, 0, TYPE_01183108, TYPE_01183194, 0]
        );
        assert_eq!(calls_to(&e, EFFECT_KEY_00403EA0), vec![vec![0x00ab_1400]]);
        // In limb mode a tile of a fade group clicks 0xa (0x3b in hardcore mode).
        e.mem.set_u8(menu + 0x2a0, 1);
        e.register(LIST_CONTAINS_005F65D0, |_, _| 1u32.into_ret());
        let click = e.mem.u32(e.mem.u32(menu) + 0xc);
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 0x7fu32, 0x5501u32]);
        assert_eq!(calls_to(&e, click), vec![vec![menu, 0xa, 0]]);
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_b380, &args![menu, 0x7fu32, 0x5501u32]);
        assert_eq!(calls_to(&e, click), vec![vec![menu, 0x3b, 0]]);
    }

    #[test]
    fn do_gamepad_limb_groups_menus_and_pages() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        // Not on top: false.
        assert!(!e.call(0x007d_b680, &args![menu, 4u32, 0u32]).bool());
        e.register(INTERFACE_IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        assert!(!e.call(0x007d_b680, &args![menu, 4u32, 0u32]).bool());
        e.register(BYTE_AT_28_00974D90, |_, _| 1u32.into_ret());
        // Limb mode, key 4: group 2 of the current group's word.
        e.mem.set_u8(menu + 0x2a0, 1);
        let object = fn_007db1c0(&mut e);
        e.mem.set_u32(object + 0x34, 0x0003_0000);
        e.register(STRING_CHARS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 4u32, 0u32]).bool());
        assert_eq!(e.mem.u32(object), 3);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [SOUND_01074F40, 0x121]
        );
        e.mem.set_u8(menu + 0x2a0, 0);
        // Key 0xd at 1.0 opens the map; at 0.5 it falls through to the stick.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007d_b680, &args![menu, 0xdu32, 1.0f32.to_bits()])
            .bool());
        assert_eq!(calls_to(&e, SET_MAP_MENU_VISIBLE), vec![vec![1, 0]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![0x24]]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007d_b680, &args![menu, 0xeu32, 2.0f32.to_bits()])
            .bool());
        assert_eq!(
            calls_to(&e, SET_INVENTORY_MENU_VISIBLE),
            vec![vec![1, 0, 1]]
        );
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x007d_b680, &args![menu, 0xdu32, 0.5f32.to_bits()])
            .bool());
        assert!(calls_to(&e, SET_MAP_MENU_VISIBLE).is_empty());
        // Keys 4 and 3 wrap the page.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 4u32, 0u32]).bool());
        assert!(set_ints(&e, 0x6100).contains(&vec![0x6100, 0x1004, 4]));
        assert_eq!(
            calls_to(&e, TILE_PLAY_TILE_SOUND),
            vec![vec![0x7000, 0xfcb]]
        );
        tile_floats(&mut e, vec![((0x6100, 0x1004), 4.0)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 3u32, 0u32]).bool());
        assert!(set_ints(&e, 0x6100).contains(&vec![0x6100, 0x1004, 0]));
        // Page 4, key 0xb: click 0x31.
        let click = e.mem.u32(e.mem.u32(menu) + 0xc);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 0xbu32, 0u32]).bool());
        assert_eq!(calls_to(&e, click), vec![vec![menu, 0x31, 0]]);
    }

    #[test]
    fn do_gamepad_on_the_first_page() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(INTERFACE_IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.register(BYTE_AT_28_00974D90, |_, _| 1u32.into_ret());
        e.mem.set_u32(menu + 0xa4, 0x5a00);
        e.mem.set_u32(menu + 0x104, 0x5104);
        e.mem.set_u32(menu + 0xb4, 0x50b4);
        e.set_global(KNOB_SETTING_011DB358 + 0x100, 0.25f32);
        e.set_global(KNOB_SETTING_011DB2F0 + 0x100, 3.0f32);
        let click = e.mem.u32(e.mem.u32(menu) + 0xc);
        // Key 2 steps sub page 0 to 1: its button sounds and the knob moves forward.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 2u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_PLAY_TILE_SOUND),
            vec![vec![0x5104, 0xfcb]]
        );
        assert_eq!(
            calls_to(&e, PIPBOY_MOVE_KNOB)[0][1..],
            [0, 3.0f32.to_bits(), 0.25f32.to_bits(), 1]
        );
        // Key 1 would go to -1: refused, so the stick runs (no input: the repeat clears).
        e.set_global(SCROLL_REPEAT_011DAD5C, 9u32);
        e.register(FLAG_00709D40, |_, _| 1u32.into_ret());
        e.register(TILE_IS_VISIBLE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x007d_b680, &args![menu, 1u32, 0u32]).bool());
        assert_eq!(e.global::<u32>(SCROLL_REPEAT_011DAD5C), 0);
        // Key 9 by sub page; keys 0xb and 0xc on sub page 0 with the button enabled.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 9u32, 0u32]).bool());
        assert!(!e.call(0x007d_b680, &args![menu, 0xbu32, 0u32]).bool());
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        assert!(e.call(0x007d_b680, &args![menu, 0xbu32, 0u32]).bool());
        assert!(e.call(0x007d_b680, &args![menu, 0xcu32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, click),
            vec![vec![menu, 0xa, 0], vec![menu, 9, 0], vec![menu, 0x3b, 0]]
        );
        tile_floats(&mut e, vec![((0x5a00, 0x1004), 1.0)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_b680, &args![menu, 0xcu32, 0u32]).bool());
        assert!(e.call(0x007d_b680, &args![menu, 9u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, click),
            vec![vec![menu, 0x1f, 0], vec![menu, 0xb, 0]]
        );
    }

    #[test]
    fn do_gamepad_scrolls_with_the_stick() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.register(INTERFACE_IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.register(BYTE_AT_28_00974D90, |_, _| 1u32.into_ret());
        e.register(FLAG_00709D40, |_, _| 1u32.into_ret());
        e.register(TILE_IS_VISIBLE, |_, _| 1u32.into_ret());
        e.register(WORD_AT_14_00825C00, |_, _| 0x5800u32.into_ret());
        e.register(TILE_TEXT_TO_TRAIT, |_, _| 0x7777u32.into_ret());
        e.register(ABS_00408820, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.register(TIME_00457FE0, |_, _| 1000u32.into_ret());
        e.mem.set_u32(menu + 0x114, 0x5114);
        // The scroll trait reads 3.0, then 4.0 once it has been set.
        e.register_double(TILE_GET_FLOAT, |e, a| {
            st0(match (a[0], a[1]) {
                (0x5800, 0x7777) if calls_to(e, TILE_SET_FLOAT).is_empty() => 3.0,
                (0x5800, 0x7777) => 4.0,
                (0x5114, 0x1004) => 100.0,
                (0x5114, 0x1005) => 250.0,
                _ => 0.0,
            })
        });
        // Inside the dead zone: nothing moves and the repeat time clears.
        e.register(STICK_00A23390, |_, _| 8689u32.into_ret());
        e.set_global(SCROLL_REPEAT_011DAD5C, 5u32);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007d_b680, &args![menu, 0x20u32, 0u32]).bool());
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(e.global::<u32>(SCROLL_REPEAT_011DAD5C), 0);
        // Stick down (negative): one step forward, poked (the value read back stays 3.0).
        e.register(STICK_00A23390, |_, _| (-9000i32 as u32).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_b680, &args![menu, 0x20u32, 0u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5800, 0x7777, 4.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x5800, 0x7777, 1.0f32.to_bits()]]
        );
        assert_eq!(e.global::<u32>(SCROLL_REPEAT_011DAD5C), 1250);
        // Before the repeat time nothing moves.
        e.call_log = Some(vec![]);
        e.call(0x007d_b680, &args![menu, 0x20u32, 0u32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(e.global::<u32>(SCROLL_REPEAT_011DAD5C), 1250);
    }

    #[test]
    fn do_gamepad_up_event_clicks_nine_in_limb_mode() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0xb4, 0x50b4);
        let click = e.mem.u32(e.mem.u32(menu) + 0xc);
        assert!(!e.call(0x007d_c0e0, &args![menu, 0xau32, 0u32]).bool());
        e.mem.set_u8(menu + 0x2a0, 1);
        assert!(!e.call(0x007d_c0e0, &args![menu, 0xau32, 0u32]).bool());
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x007d_c0e0, &args![menu, 0xbu32, 0u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x007d_c0e0, &args![menu, 0xau32, 0u32]).bool());
        assert_eq!(calls_to(&e, click), vec![vec![menu, 9, 0]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY).len(), 1);
    }

    #[test]
    fn do_enter_by_page() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x30, 1);
        e.register(LIST_CONTAINS_005F65D0, |_, _| 1u32.into_ret());
        // Page 0: the knob moves for tile 0x7004 (`0xfac` 2.0) and the group fades in.
        tile_floats(&mut e, vec![((0x7004, 0xfac), 2.0)]);
        e.call_log = Some(vec![]);
        e.call(0x007d_c1b0, &args![menu, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(KNOB_STEP_011DAD60), 2);
        assert_eq!(e.mem.u8(menu + 0x2a0), 1);
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY).len(), 1);
        // In hardcore mode the record at +0x60 decides: none, so nothing fades.
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_c1b0, &args![menu, 1u32, 0u32]);
        assert!(calls_to(&e, SOUND_HANDLE_PLAY).is_empty());
        // Page 2: an item of the skill list shows its description.
        e.mem.set_u32(menu + 0xf8, 0x00ac_0000);
        e.map(0x00ac_0000, 0x1000);
        e.mem.set_u32(0x00ac_0004, 0x5c00);
        e.mem.set_u32(0x00ac_0008, 0x5d00);
        e.register(WORD_AT_28_0045CD60, |_, _| 0u32.into_ret());
        tile_floats(&mut e, vec![((0x6100, 0x1004), 2.0)]);
        e.register(LIST_BOX_HAS_TILE_007242F0, |_, a| {
            u32::from(a[1] == 0x5e00).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007d_c1b0, &args![menu, 0x7fu32, 0x5e00u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_HAS_TILE_007242F0),
            vec![vec![menu + 0x1b0, 0x5e00]]
        );
        assert_eq!(calls_to(&e, AV_GET_DESCRIPTION).len(), 1);
        // Page 4: the reputation list's tile shows the reputation.
        tile_floats(&mut e, vec![((0x6100, 0x1004), 4.0)]);
        e.call_log = Some(vec![]);
        e.call(0x007d_c1b0, &args![menu, 0x7fu32, 0x5e00u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_HAS_TILE_007242F0),
            vec![vec![menu + 0x210, 0x5e00], vec![menu + 0x270, 0x5e00]]
        );
    }

    #[test]
    fn do_leave_hides_the_badge_and_ends_limb_mode() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0x160, 0x5160);
        e.call_log = Some(vec![]);
        e.call(0x007d_c490, &args![menu, 0u32, 0x7000u32]);
        assert_eq!(set_ints(&e, 0x5160), vec![vec![0x5160, 0xfa3, 0]]);
        e.mem.set_u8(menu + 0x2a0, 1);
        e.call(0x007d_c490, &args![menu, 0u32, 0x7000u32]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 1);
        e.register(LIST_CONTAINS_005F65D0, |_, _| 1u32.into_ret());
        e.call(0x007d_c490, &args![menu, 0u32, 0x7000u32]);
        assert_eq!(e.mem.u8(menu + 0x2a0), 0);
    }

    /// A menu whose description box (+0xf8) has text tile 0x5c00 and scroll tile 0x5d00.
    fn description_menu(e: &mut Engine) -> u32 {
        let menu = menu(e);
        let description = e.mem.alloc(0x10);
        e.mem.set_u32(menu + 0xf8, description);
        e.mem.set_u32(description + 4, 0x5c00);
        e.mem.set_u32(description + 8, 0x5d00);
        e.mem.set_u32(menu + 0xf4, 0x50f4);
        e.mem.set_u32(menu + 0x160, 0x5160);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(WORD_AT_28_0045CD60, |_, _| 0x5c28u32.into_ret());
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(SCROLL_VALUE_004E4430, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        tile_floats(
            e,
            vec![
                ((0x5d00, 0x4eb8), 2.0),
                ((0x5c00, 0xfb0), 30.0),
                ((0x5c28, 0xfb0), 10.0),
            ],
        );
        menu
    }

    #[test]
    fn set_icon_and_description_shows_the_badge_and_the_scroll() {
        let mut e = batch_engine();
        let menu = description_menu(&mut e);
        e.register(AV_GET_ICON, |_, _| 0x00ad_0001u32.into_ret());
        e.register(AV_GET_DESCRIPTION, |_, _| 0x00ad_0002u32.into_ret());
        e.register(AV_GET_BADGE, |_, _| 0x00ad_0003u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_c7e0, &args![menu, 0x20u32]);
        assert_eq!(calls_to(&e, AV_GET_ICON), vec![vec![0x20]]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x50f4, 0xfcc, 0x00ad_0001, 1],
                vec![0x5c00, 0xfc4, 0x00ad_0002, 1],
                vec![0x5160, 0xfcc, 0x00ad_0003, 1],
            ]
        );
        assert_eq!(set_ints(&e, 0x5160), vec![vec![0x5160, 0xfa3, 1]]);
        assert_eq!(calls_to(&e, TILE_UPDATE_TILE), vec![vec![0x5c00, 0]]);
        // (30 - 10) / 2 + 1 = 11.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x5d00, 0xf8f8, 11.0f32.to_bits(), 1],
                vec![0x5d00, 0x02d8, 0, 0],
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![vec![0x5d00, 0x0250, 1.0f32.to_bits()]]
        );
        e.register(AV_GET_BADGE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_c7e0, &args![menu, 0x20u32]);
        assert_eq!(set_ints(&e, 0x5160), vec![vec![0x5160, 0xfa3, 0]]);
    }

    #[test]
    fn the_perk_description_uses_the_perk_strings() {
        let mut e = batch_engine();
        let menu = description_menu(&mut e);
        let perk = fake_object(&mut e, 9);
        let description_vtable = fake_vtable(&mut e, 10);
        e.mem.set_u32(perk + 0x24, description_vtable);
        e.register(slot(10, 0x10), |_, _| 0x00ad_0005u32.into_ret());
        e.register(STRING_CHARS_00408DA0, |_, a| a[0].into_ret());
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, perk);
        e.call_log = Some(vec![]);
        e.call(0x007d_c9b0, &args![menu, data]);
        assert_eq!(
            calls_to(&e, slot(10, 0x10)),
            vec![vec![perk + 0x24, 0, 0x4353_4544]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x50f4, 0xfcc, perk + 0x2c, 1],
                vec![0x5c00, 0xfc4, 0x00ad_0005, 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_POKE).len(), 1);
    }

    #[test]
    fn the_reputation_tiles() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        for (offset, tile) in [(0x14cu32, 0x514cu32), (0x150, 0x5150), (0x15c, 0x515c)] {
            e.mem.set_u32(menu + offset, tile);
        }
        e.call_log = Some(vec![]);
        e.call(0x007d_cb40, &args![menu, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.register(REPUTATION_ICON_006166E0, |_, _| 0x00ad_0001u32.into_ret());
        e.register(REPUTATION_GET_TITLE, |_, _| 0x00ad_0002u32.into_ret());
        e.register(STRING_CHARS_00408DA0, |_, a| a[0].into_ret());
        e.call(0x007d_cb40, &args![menu, 0x00ab_0000u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![
                vec![0x514c, 0xfcc, 0x00ad_0001, 1],
                vec![0x515c, 0xfc4, 0x00ab_0018, 1],
                vec![0x5150, 0xfc4, 0x00ad_0002, 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_INT).len(), 3);
    }

    #[test]
    fn update_all_sets_the_labels_only_for_another_language() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007d_cc80, &args![menu]);
        assert!(calls_to(&e, ITEM_CHANGE_007314C0).contains(&vec![
            menu + 0x210,
            0x007d_fc60,
            0,
            0x7fff_ffff
        ]));
        assert!(calls_to(&e, TILE_GET_CHILD_BY_NAME).is_empty());
        e.register(COMPARE_00408B20, |_, _| 1u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, a| a[0].into_ret());
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| (a[1] & 0xffff).into_ret());
        e.register(TILE_TEXT_TO_TRAIT, |_, _| 0x1234u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_cc80, &args![menu]);
        let strings = calls_to(&e, TILE_SET_STRING);
        assert!(strings.contains(&vec![0x7000, 0xfc4, 0x011d_2f88, 1]));
        assert!(strings.contains(&vec![0x6100, 0x100e, 0x011d_41bc, 1]));
        assert!(strings.contains(&vec![0x7398, 0xfc4, 0x011d_3fdc, 1]));
        assert!(strings.contains(&vec![0x7388, 0x1234, 0x011d_1f5c, 1]));
        assert!(strings.contains(&vec![0x732c, 0xfc4, 0x011d_2ebc, 1]));
    }

    #[test]
    fn the_reputation_list_takes_known_reputations() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        e.mem.set_u32(menu + 0x148, 0x5148);
        let list_vtable = fake_vtable(&mut e, 11);
        e.mem.set_u32(menu + 0x270, list_vtable);
        // Two nodes: reputation 0x00ab_0100 (0.0 and 2.0) and 0x00ab_0200 (0.0 and NaN).
        e.map(0x00ab_0000, 0x1000);
        e.mem.set_u32(0x00ab_0000, 0x00ab_0100);
        e.mem.set_u32(0x00ab_0004, 0x00ab_0010);
        e.mem.set_u32(0x00ab_0010, 0x00ab_0200);
        e.register(REPUTATION_LIST_009C1A50, |_, _| 0x00ab_0000u32.into_ret());
        e.register(REPUTATION_VALUE_006155C0, |_, a| {
            st0(match (a[0], a[1]) {
                (0x00ab_0100, 0) => 2.0,
                (0x00ab_0200, 0) => f64::NAN,
                _ => 0.0,
            })
        });
        e.register(STRING_CHARS_00559450, |_, _| 0x00ad_0009u32.into_ret());
        e.register(STRING_CHARS_00408DA0, |_, a| a[0].into_ret());
        e.register(TILE_GET_MENU, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_d8c0, &args![menu]);
        assert_eq!(calls_to(&e, slot(11, 0x1c)), vec![vec![menu + 0x270]]);
        assert_eq!(
            calls_to(&e, STRING_FORMAT)
                .iter()
                .map(|w| w[2])
                .collect::<Vec<_>>(),
            vec![0x00ab_0118]
        );
        // The list box (no container at +0xc) adds nothing but was asked.
        assert_eq!(calls_to(&e, STRING_DESTROY).len(), 1);
        // Without the reputation tile nothing happens.
        e.mem.set_u32(menu + 0x148, 0);
        e.call_log = Some(vec![]);
        e.call(0x007d_d8c0, &args![menu]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn the_perk_list_clear_and_the_general_list_walk() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        let list_vtable = fake_vtable(&mut e, 12);
        e.mem.set_u32(menu + 0x1e0, list_vtable);
        e.mem.set_u32(menu + 0x118, 0);
        e.call_log = Some(vec![]);
        e.call(0x007d_da60, &args![]);
        assert!(calls_to(&e, slot(12, 0x1c)).is_empty());
        e.mem.set_u32(menu + 0x118, 1);
        e.call(0x007d_da60, &args![]);
        assert_eq!(calls_to(&e, slot(12, 0x1c)), vec![vec![menu + 0x1e0]]);
        e.call(0x007d_daa0, &args![menu]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_007314C0),
            vec![vec![menu + 0x210, 0x007d_fc60, 0, 0x7fff_ffff]]
        );
    }

    #[test]
    fn the_hardcore_meters() {
        let mut e = batch_engine();
        let player = with_player(&mut e);
        let values = e.mem.u32(player + 0xa4);
        let menu = menu(&mut e);
        tile_floats(&mut e, vec![((0x6100, 0x1008), 0.5)]);
        e.register(VALUE_005C5420, |_, _| st0(0.25));
        e.register(e.mem.u32(values + 0xc), |_, a| st0(f64::from(a[1]) / 200.0));
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(FLOAT_PAIR_004BD510, |_, _| st0(7.0));
        e.call_log = Some(vec![]);
        for addr in [0x007d_dc20u32, 0x007d_dd10, 0x007d_de00] {
            e.call(addr, &args![menu]);
        }
        // Values 0x49 to 0x4b over 200 lie between 0 and 1: each meter shows 1.0.
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![0x6100, 0x100f, 1.0f32.to_bits(), 1],
                vec![0x6100, 0x1010, 1.0f32.to_bits(), 1],
                vec![0x6100, 0x1011, 1.0f32.to_bits(), 1],
            ]
        );
        e.register(e.mem.u32(values + 0xc), |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        e.call(0x007d_dc20, &args![menu]);
        assert_eq!(
            calls_to(&e, FLOAT_PAIR_004BD510),
            vec![vec![0, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x6100, 0x100f, 7.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn the_status_effect_entry_constructor_and_setter() {
        let mut e = batch_engine();
        let entry = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_f140, &args![entry, 0x99u32]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0x99);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT_00406DB0), vec![vec![entry + 4]]);
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCT_0071F540),
            vec![vec![entry + 0x10]]
        );
        e.call(0x007d_f120, &args![entry, 7u32]);
        assert_eq!(
            calls_to(&e, FIELD_SET_00501C10),
            vec![vec![entry + 0x20, 7]]
        );
    }

    #[test]
    fn knob_position_and_stats_page_need_an_open_menu() {
        let mut e = batch_engine();
        e.register(STATS_MENU_SET_ICON_AND_DESCRIPTION, |_, _| Ret::default());
        with_player(&mut e);
        e.register(INTERFACE_MANAGER_004B7210, |_, _| 0x00ab_0000u32.into_ret());
        e.register(WORD_AT_174_00704370, |_, _| 0x00ab_0174u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007d_fc90, &args![1u32]);
        e.call(0x007d_fd00, &args![2u32]);
        assert!(calls_to(&e, PIPBOY_SET_TAB_KNOB_POSITION).is_empty());
        assert!(calls_to(&e, TILE_SET_INT).is_empty());
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        tile_floats(&mut e, vec![((0x6100, 0x1004), 3.0)]);
        e.call(0x007d_fc90, &args![0x101u32]);
        assert_eq!(
            calls_to(&e, PIPBOY_SET_TAB_KNOB_POSITION),
            vec![vec![0x00ab_0174, 3, 1]]
        );
        e.call(0x007d_fd00, &args![0x102u32]);
        assert!(set_ints(&e, 0x6100).contains(&vec![0x6100, 0x1004, 2]));
    }

    #[test]
    fn move_knob_steps_forward_and_back() {
        let mut e = batch_engine();
        e.set_global(KNOB_SETTING_011DB358 + 0x100, 0.25f32);
        e.set_global(KNOB_SETTING_011DB2F0 + 0x100, 3.0f32);
        let last = e.mem.alloc(4);
        e.mem.set_u32(last, 2);
        tile_floats(&mut e, vec![((0x5000, 0xfac), 4.9), ((0x5001, 0xfac), 1.2)]);
        e.call_log = Some(vec![]);
        e.call(0x007d_fd20, &args![0u32, 0x5000u32, last]);
        assert_eq!(e.mem.u32(last), 4);
        e.call(0x007d_fd20, &args![0u32, 0x5000u32, last]);
        e.call(0x007d_fd20, &args![0u32, 0x5001u32, last]);
        assert_eq!(e.mem.u32(last), 1);
        let moves: Vec<_> = calls_to(&e, PIPBOY_MOVE_KNOB)
            .into_iter()
            .map(|w| w[1..].to_vec())
            .collect();
        assert_eq!(
            moves,
            vec![
                vec![0, 3.0f32.to_bits(), 0.25f32.to_bits(), 1],
                vec![0, (-3.0f32).to_bits(), 0.25f32.to_bits(), 1],
            ]
        );
    }

    #[test]
    fn sub_pages_and_page_lookups() {
        let mut e = batch_engine();
        with_player(&mut e);
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 0xa4, 0x5a00);
        assert!(!e.call(0x007e_0060, &args![menu, 0xffff_ffffu32]).bool());
        assert!(!e.call(0x007e_0060, &args![menu, 3u32]).bool());
        assert!(!e.call(0x007e_0060, &args![menu, 0u32]).bool());
        e.mem.set_u8(menu + 0x2a0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_0060, &args![menu, 2u32]).bool());
        assert_eq!(e.mem.u8(menu + 0x2a0), 0);
        assert_eq!(set_ints(&e, 0x5a00), vec![vec![0x5a00, 0x1004, 2]]);
        e.register(HARDCORE_004D1360, |_, _| 1u32.into_ret());
        assert!(e.call(0x007e_0060, &args![menu, 5u32]).bool());
        assert!(!e.call(0x007e_0060, &args![menu, 6u32]).bool());
        let out = e.mem.alloc(4);
        for id in 0..5u32 {
            assert!(e.call(0x007e_00e0, &args![menu, id, out]).bool());
            assert_eq!(e.mem.u32(out), id);
        }
        assert!(!e.call(0x007e_00e0, &args![menu, 5u32, out]).bool());
        for (id, page) in [
            (0x1cu32, 0u32),
            (0x1d, 1),
            (0x1e, 2),
            (0x35, 3),
            (0x37, 4),
            (0x39, 5),
        ] {
            assert!(e.call(0x007e_0160, &args![menu, id, out]).bool());
            assert_eq!(e.mem.u32(out), page);
        }
        e.mem.set_u32(out, 9);
        for id in [0x1bu32, 0x1f, 0x36, 0x38, 0x3a] {
            assert!(!e.call(0x007e_0160, &args![menu, id, out]).bool());
        }
        assert_eq!(e.mem.u32(out), 9);
    }

    #[test]
    fn toggle_gen_rep_mode_switches_the_tiles() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        e.set_global(OPEN_MENU_011DACE0, menu);
        for (offset, tile) in [
            (0x148u32, 0x5148u32),
            (0xb0, 0x50b0),
            (0x154, 0x5154),
            (0x158, 0x5158),
        ] {
            e.mem.set_u32(menu + offset, tile);
        }
        e.register(ADD_USER_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(SETTING_STRING_00403DF0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007e_02b0, &args![menu]);
        assert_eq!(e.global::<u32>(GEN_REP_TRAIT_011DAD88), 0x0244);
        assert_eq!(e.global::<u32>(GEN_REP_TRAIT_011DAD84), 0x7564);
        assert_eq!(
            set_ints(&e, 0x5148),
            vec![vec![0x5148, 0x0244, 1], vec![0x5148, 0xfa3, 1]]
        );
        assert_eq!(
            set_ints(&e, 0x50b0),
            vec![vec![0x50b0, 0x0244, 0], vec![0x50b0, 0xfa3, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5154, 0xfc4, SETTING_011D4EF4, 1]]
        );
        assert_eq!(set_ints(&e, 0x5158), vec![vec![0x5158, 0x1004, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x007e_02b0, &args![menu]);
        assert_eq!(calls_to(&e, ADD_USER_TRAIT).len(), 0);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5154, 0xfc4, SETTING_011D4144, 1]]
        );
        assert_eq!(set_ints(&e, 0x5158), vec![vec![0x5158, 0x1004, 0]]);
    }

    #[test]
    fn the_three_sorts_compare_the_names() {
        let mut e = batch_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(STRING_CHARS_00408DA0, |_, a| (a[0] + 2).into_ret());
        e.register(COMPARE_00404DC0, |_, a| a[0].wrapping_sub(a[1]).into_ret());
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x100);
        e.mem.set_u32(second, 0x200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_0480, &args![first, second]).i32(), -0x100);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![
                vec![0x200, 0, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0],
                vec![0x100, 0, TYPE_FORM_01183028, TYPE_NAMED_01183158, 0],
            ]
        );
        e.mem.set_u32(first + 4, 0x1000);
        e.mem.set_u32(second + 4, 0x3000);
        assert_eq!(e.call(0x007e_0510, &args![first, second]).i32(), -0x2000);
        // The stage sort: each makes its string into a local, the second first.
        e.register(ENTRY_STRING_004046B0, |_, a| a[1].into_ret());
        e.register(STRING_CHARS_00559450, |_, a| (a[0] & 0xff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007e_0540, &args![first, second]);
        let made = calls_to(&e, ENTRY_STRING_004046B0);
        assert_eq!(
            made.iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![second, first]
        );
        let destroyed = calls_to(&e, STRING_DESTROY);
        assert_eq!(destroyed, vec![vec![made[1][1]], vec![made[0][1]]]);
    }

    #[test]
    fn the_status_effect_list_registers_by_kind() {
        let mut e = batch_engine();
        e.register(EFFECT_TARGET_0044DDC0, |_, a| (a[0] + 0x10).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 0x20).into_ret());
        e.register(EFFECT_ITEM_0084E3A0, |_, _| 0x77u32.into_ret());
        e.register(WORD_AT_14_00825C00, |_, _| 0u32.into_ret());
        e.register(ENTRY_CONSTRUCT_00403BA0, |_, a| a[0].into_ret());
        let list = e.mem.alloc(0x10);
        // Not found: a new entry is made and inserted sorted; flag 0x2000 off.
        e.call_log = Some(vec![]);
        let effect = e.mem.alloc(0x20);
        e.call(0x007e_08a0, &args![list, effect]);
        let inserted = calls_to(&e, LIST_INSERT_SORTED);
        assert_eq!(inserted[0][0], list + 4);
        assert_eq!(inserted[0][2], 0x007e_0480);
        let entry = inserted[0][1];
        assert_eq!(e.mem.u32(entry), effect + 0x30);
        // `007e0a20` on the entry's data list (+4): a new 0x14-byte entry.
        assert_eq!(calls_to(&e, ENTRY_CONSTRUCT_00403BA0).len(), 1);
        // Flag 0x2000: the stage list (+0x10) takes the setting unless it has it.
        e.register(FLAG_TEST_00403E60, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007e_08a0, &args![list, effect]);
        let inserted = calls_to(&e, LIST_INSERT_SORTED);
        assert_eq!(inserted.len(), 2);
        assert_eq!(inserted[1][0], inserted[0][1] + 0x14);
        assert_eq!(inserted[1][2], 0x007e_0540);
        e.register(LIST_CONTAINS_005F65D0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        fn_007e09d0(&mut e, Ptr::new(0x0e10_0000), effect);
        assert!(calls_to(&e, LIST_INSERT_SORTED).is_empty());
    }

    #[test]
    fn the_status_data_entries_sum_magnitudes() {
        let mut e = batch_engine();
        e.register(EFFECT_ITEM_0084E3A0, |_, a| a[0].into_ret());
        e.register(EFFECT_KEY_00403EA0, |_, a| (a[0] & 0xf).into_ret());
        e.register(MAGNITUDE_00495460, |_, _| st0(2.75));
        e.register(ENTRY_CONSTRUCT_00403BA0, |_, a| a[0].into_ret());
        // Appending puts the entry first in the embedded list.
        e.register(LIST_APPEND, |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], entry);
            Ret::default()
        });
        let list = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x007e_0a20, &args![list, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x007e_0a20, &args![list, 0x0010_0003u32]);
        assert_eq!(calls_to(&e, LIST_APPEND)[0][0], list + 4);
        let entry = e.mem.u32(list + 4);
        assert_eq!(e.mem.u32(entry + 0x10), 3);
        assert_eq!(e.mem.u32(entry), 2);
        // Found by key: the magnitude is added.
        e.call(0x007e_0a20, &args![list, 0x0020_0003u32]);
        assert_eq!(e.mem.u32(entry), 4);
        // The status effect list finds its entries by the first word's item.
        let found = e.mem.alloc(4);
        e.mem.set_u32(found, 0x55);
        e.mem.set_u32(list + 4, found);
        assert_eq!(e.call(0x007e_0c50, &args![list, 0x55u32]).u32(), found);
        assert_eq!(e.call(0x007e_0c50, &args![list, 0x56u32]).u32(), 0);
        assert_eq!(e.call(0x007e_0c50, &args![0u32, 0x55u32]).u32(), 0);
    }

    #[test]
    fn controller_switch_shows_the_healing_mode() {
        let mut e = batch_engine();
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007e_0d60, &args![menu, 0u32]);
        assert_eq!(calls_to(&e, ADD_USER_TRAIT).len(), 2);
    }

    #[test]
    fn string_append_grows_copies_or_sets() {
        let mut e = batch_engine();
        let text = e.mem.alloc(8);
        let source = e.mem.alloc(8);
        e.mem.set_u32(text, 0x00ab_0000);
        e.register(STRING_CHARS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_LENGTH_004048E0, |_, a| (a[0] & 0xf).into_ret());
        e.register(STRING_CAPACITY_004039A0, |_, _| 4u32.into_ret());
        e.register(STRING_COPY_LENGTH_004F4280, |_, _| 9u32.into_ret());
        // Without source text: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_0d80, &args![text, source]).u32(), text);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.mem.set_u32(source, 0x00ac_0000);
        e.call_log = Some(vec![]);
        e.call(0x007e_0d80, &args![text, source]);
        let own = text & 0xf;
        let added = source & 0xf;
        let grown = calls_to(&e, STRING_GROW_004037F0);
        assert_eq!(grown.len(), usize::from(own + added > 4));
        assert_eq!(
            calls_to(&e, COPY_00401460),
            vec![vec![own + 0x00ab_0000, 0x00ac_0000, 9]]
        );
        assert_eq!(
            calls_to(&e, STRING_SET_LENGTH_004039C0),
            vec![vec![text, own + added]]
        );
        // A string without text takes the source.
        e.mem.set_u32(text, 0);
        e.call_log = Some(vec![]);
        e.call(0x007e_0d80, &args![text, source]);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![text, source]]);
        assert!(calls_to(&e, COPY_00401460).is_empty());
    }

    #[test]
    fn the_two_list_box_instances_use_their_own_helpers() {
        let mut e = batch_engine();
        e.register(TILE_GET_MENU, |_, _| 0x00ab_0000u32.into_ret());
        e.register(MENU_RENDER_TEMPLATE, |_, _| 0x5900u32.into_ret());
        e.register(LIST_BOX_HAS_FLAG, |_, _| 1u32.into_ret());
        let list = e.mem.alloc(0x30);
        e.mem.set_u32(list + 0xc, 0x5800);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007e_1430, &args![list, 1u32, 0u32, 0x99u32, 0x1234u32])
                .u32(),
            0x5900
        );
        assert_eq!(
            calls_to(&e, STATUS_EFFECT_SYNCHRONIZE_007254D0),
            vec![vec![list]]
        );
        e.call(0x007e_16d0, &args![list, 1u32, 0u32, 0u32, 0x1234u32]);
        assert_eq!(
            calls_to(&e, REPUTATION_SET_TOTAL_LIST_HEIGHT),
            vec![vec![list]]
        );
        assert!(calls_to(&e, REPUTATION_SYNCHRONIZE).is_empty());
    }
}

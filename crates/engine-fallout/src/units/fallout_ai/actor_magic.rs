//! `fallout/ai/actor_magic.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the magic half of `Actor`: eating and drinking, the permanent
//! magic (abilities and enchanted worn items), `AddSpell`/`RemoveSpell`,
//! `UpdateMagic`, and the actor's overrides of the `MagicCaster` and
//! `MagicTarget` virtuals. An `Actor` carries those two bases at `+0x88`
//! (magic caster) and `+0x94` (magic target), so the overrides receive that
//! interior pointer and subtract `0x88` / `0x94` to reach the actor. The
//! vtables at `010841f4` (caster) and `010841bc` (target) have the RTTI
//! locator of class `Actor` (offsets `0x88` and `0x94`).
//!
//! The unit is compiled without inlining: every field read the source had
//! as an inline accessor is a call to a tiny function in the exe (`+4`,
//! `+8`, `+0xc`, ... of its `this`). They are called by address like any
//! other function; the ones whose body is just a field read say so.
//!
//! x87 notes. The game computes floats in extended precision and stores
//! `float` results; the translations compute in `f64` and round to `f32`
//! where the code stores a `float`. `FLD`/`FSTP` of a `float` turns a
//! signalling NaN into a quiet one, which [`quiet`] does wherever the code
//! copies a float that way. Float tests keep the exact flag tests of the
//! code (`FCOMP` then `TEST AH,5`/`0x41`/`0x44` and the parity jump): a NaN
//! never takes the "less than"/"greater than"/"equal" side.
//!
//! Functions whose disassembly has a C++ exception frame (`FS:[0]`) are
//! marked `// Translation incomplete:`; their ordinary behaviour is
//! translated, the unwinding is not.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layout offsets used throughout (PC build)

/// The `MagicCaster` base inside an `Actor`.
const MAGIC_CASTER: u32 = 0x88;
/// The `MagicTarget` base inside an `Actor`.
const MAGIC_TARGET: u32 = 0x94;

// ---- Exe data

/// The player character singleton pointer (`thePlayer`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A global pointer to an object `0047c850` is called on (that function
/// returns 0 whatever it is).
const OBJECT_011DE45C: u32 = 0x011d_e45c;
/// The "cannot use" message setting shown for a form that can not be eaten.
const CANNOT_USE_MESSAGE: u32 = 0x011d_39fc;
/// The `ProcessLists` singleton (the `this` of `00974e90`, `00973fd0`,
/// `0096d490`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The calendar singleton.
const CALENDAR_OBJECT: u32 = 0x011d_e7b8;
/// Settings whose float is reached through `00403e20`.
const SETTING_011D0AD8: u32 = 0x011d_0ad8;
const SETTING_011D1B38: u32 = 0x011d_1b38;
/// Words returned by `008c4e00` / `008c4f00`.
const ID_011C3584: u32 = 0x011c_3584;
const ID_011C3580: u32 = 0x011c_3580;
/// The text `"Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds"`.
const SAD_ICON_PATH: u32 = 0x0102_08a0;
/// The label string of the profiling scope `UpdateMagic` opens.
const SCOPE_NAME_01085384: u32 = 0x0108_5384;
/// A table of five `u32` worn-slot indices, walked by the permanent magic
/// functions.
const WORN_SLOT_TABLE: u32 = 0x011a_337c;
/// Floats and doubles the code loads (`FLD`) from the exe's data.
const MESSAGE_SCALE_010162C0: u32 = 0x0101_62c0;
const FIXED_STEP_01017D00: u32 = 0x0101_7d00;
const RESET_VALUE_01012054: u32 = 0x0101_2054;
const LIMIT_01016410: u32 = 0x0101_6410;
const FADE_01085350: u32 = 0x0108_5350;
const PLAYER_ALPHA_SCALE_011A3B38: u32 = 0x011a_3b38;
const THRESHOLD_01012060: u32 = 0x0101_2060;
const UPPER_BOUND_010290D0: u32 = 0x0102_90d0;
const FACTOR_01016408: u32 = 0x0101_6408;
const LIGHT_SCALE_0101DE30: u32 = 0x0101_de30;
const SECONDS_PER_HOUR_01012640: u32 = 0x0101_2640;
/// The number of worn-item slots `CastPermanentMagic` walks.
const WORN_SLOT_COUNT: usize = 20;
/// `UpdateMagic`'s switch on the animation group id minus `0x1a` (`0..=0x42`): the groups
/// (as that offset) whose case skips the call that clears the caster's source.
fn animation_group_skips_caster_call(offset: u32) -> bool {
    matches!(
        offset,
        0 | 3 | 6 | 9 | 12 | 15 | 18 | 21 | 24 | 27 | 30 | 33 | 36 | 39 | 42 | 45 | 48 | 51 | 66
    )
}

// ---- Run-time type descriptors given to `__RTDynamicCast` (RTTI names read
// from the exe)

/// `.?AVTESBoundObject@@`
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
/// `.?AVTESEnchantableForm@@`
const TYPE_ENCHANTABLE_FORM: u32 = 0x0118_39b4;
/// `.?AVTESForm@@`
const TYPE_FORM: u32 = 0x0118_3028;
/// `.?AVSpellItem@@`
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// `.?AVMagicItem@@`
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
/// `.?AVAlchemyItem@@`
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;
/// `.?AVIngredientItem@@`
const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
/// `.?AVEnchantmentItem@@`
const TYPE_ENCHANTMENT_ITEM: u32 = 0x0118_3594;
/// `.?AVActor@@`
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `.?AVMagicCaster@@`
const TYPE_MAGIC_CASTER: u32 = 0x011a_1224;
/// The data address `00653270` receives with a node (other units call it
/// the controller type descriptor).
const TYPE_DATA_011F36AC: u32 = 0x011f_36ac;

// ---- Virtual slots (byte offsets into the vtable of the receiver)

/// Form (`TESForm`-derived) virtuals: `+0x18` is the form type id.
const FORM_SLOT_0X18: u32 = 0x18;
const FORM_SLOT_0X94: u32 = 0x94;
const FORM_SLOT_0XEC: u32 = 0xec;
/// `Actor` virtuals.
const ACTOR_SLOT_0X17C: u32 = 0x17c;
const ACTOR_SLOT_0X1D0: u32 = 0x1d0;
const ACTOR_SLOT_0X1E4: u32 = 0x1e4;
const ACTOR_SLOT_0X22C: u32 = 0x22c;
const ACTOR_SLOT_0X360: u32 = 0x360;
const ACTOR_SLOT_0X37C: u32 = 0x37c;
const ACTOR_SLOT_0X384: u32 = 0x384;
const ACTOR_SLOT_0X3DC: u32 = 0x3dc;
const ACTOR_SLOT_0X41C: u32 = 0x41c;
const ACTOR_SLOT_0X4BC: u32 = 0x4bc;
/// The object at actor `+0xa4` (`this + 0x10` from the target): `+0xc` reads
/// an actor value as a float.
const ACTOR_VALUE_SLOT_0XC: u32 = 0xc;
/// Virtuals of the `MagicCaster` base (the actor's table at `010841f4`).
const CASTER_SLOT_0X00: u32 = 0x00;
const CASTER_SLOT_0X10: u32 = 0x10;
const CASTER_SLOT_0X14: u32 = 0x14;
const CASTER_SLOT_0X28: u32 = 0x28;
const CASTER_SLOT_0X30: u32 = 0x30;
const CASTER_SLOT_0X34: u32 = 0x34;
const CASTER_SLOT_0X40: u32 = 0x40;
const CASTER_SLOT_0X48: u32 = 0x48;
/// Virtual of the `MagicTarget` base (the actor's table at `010841bc`).
const TARGET_SLOT_0X8: u32 = 0x8;
/// Virtual `+0x48` of the base data object.
const BASE_DATA_SLOT_0X48: u32 = 0x48;
/// Virtuals of the object `CASTER_SLOT_0X30` returns.
const ITEM_SLOT_0XDC: u32 = 0xdc;
const ITEM_SLOT_0XF0: u32 = 0xf0;
/// Virtuals of an effect object.
const OBJECT_SLOT_0X38: u32 = 0x38;
const OBJECT_SLOT_0XC4: u32 = 0xc4;
/// Virtuals of the actor's process (the pointer at actor `+0x68`).
const PROCESS_SLOT_0X3B8: u32 = 0x3b8;
const PROCESS_SLOT_0X3C0: u32 = 0x3c0;
const PROCESS_SLOT_0X3C4: u32 = 0x3c4;
const PROCESS_SLOT_0X3C8: u32 = 0x3c8;
const PROCESS_SLOT_0X3CC: u32 = 0x3cc;
const PROCESS_SLOT_0X3D0: u32 = 0x3d0;
const PROCESS_SLOT_0X3D4: u32 = 0x3d4;
const PROCESS_SLOT_0X3D8: u32 = 0x3d8;
const PROCESS_SLOT_0X3DC: u32 = 0x3dc;
const PROCESS_SLOT_0X3E8: u32 = 0x3e8;
const PROCESS_SLOT_0X5AC: u32 = 0x5ac;
const PROCESS_SLOT_0X5B0: u32 = 0x5b0;
const PROCESS_SLOT_0X5B4: u32 = 0x5b4;
const PROCESS_SLOT_0X5C8: u32 = 0x5c8;
const PROCESS_SLOT_0X5CC: u32 = 0x5cc;
const PROCESS_SLOT_0X610: u32 = 0x610;
const PROCESS_SLOT_0X700: u32 = 0x700;

// ---- Callees outside this unit (called by address)

/// `__RTDynamicCast` (CRT), `cdecl`: (object, 0, source type, target type, 0).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `ECX` = x; returns the first word of x (`[x]`).
const FIRST_WORD: u32 = 0x0055_9450;
/// `ECX` = x; returns x (the address of a list node's data word).
const IDENTITY: u32 = 0x0068_15c0;
/// `ECX` = x; returns `[x + 8]`.
const FIELD_8: u32 = 0x0044_ddc0;
/// `ECX` = x; returns `[x + 4]` (a list node's next pointer, or a field).
const FIELD_4: u32 = 0x0072_6070;
/// `ECX` = x; returns the byte at `x + 4` (a form's type byte).
const TYPE_BYTE: u32 = 0x0040_1170;
/// `ECX` = x; returns `[x + 0xc]`.
const FIELD_C: u32 = 0x0084_e3a0;
/// `ECX` = x; returns `[x + 0xf8]`.
const FIELD_F8: u32 = 0x0047_6c90;
/// `ECX` = x; returns `[x + 0x84]`.
const FIELD_84: u32 = 0x004f_d3c0;
/// `ECX` = x; returns `[x + 0x70]`.
const FIELD_70: u32 = 0x004f_d380;
/// `ECX` = x; returns `[x + 0x20]` (`007af430`).
const FIELD_20: u32 = 0x007a_f430;
/// `ECX` = x; returns `[x + 0x68]` (the actor's process pointer).
const PROCESS_OF: u32 = 0x008d_8520;
/// `ECX` = reference; returns `reference + 0x44` (its extra data list).
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ECX` = x; returns the float at `x + 0x30` in `ST0`.
const FLOAT_FIELD_30: u32 = 0x0050_8100;
/// `ECX` = x; returns the float at `x + 0x34` in `ST0`.
const FLOAT_FIELD_34: u32 = 0x0050_8120;
/// `Script::SetActionFlag` (engine map), `cdecl`: (object, extra list, flag).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `PlayerCharacter::AddEatDrinkItem` (engine map): `ECX` = player, one word.
const ADD_EAT_DRINK_ITEM: u32 = 0x0095_d0b0;
/// `MagicCaster::ReleaseCast` (engine map): `ECX` = caster, one word.
const RELEASE_CAST: u32 = 0x0081_5870;
/// `cdecl`, one word (a size): returns a new block.
const FN_00401000: u32 = 0x0040_1000;
/// `cdecl`, one word: frees that block.
const FN_00401030: u32 = 0x0040_1030;
/// `ECX` = a string object on the stack: destroys it.
const FN_004037D0: u32 = 0x0040_37d0;
/// `ECX` = a setting object: returns its field at `+4`, 0 for null.
const FN_00403DF0: u32 = 0x0040_3df0;
/// `ECX` = a setting object: the address of its float at `+4`.
const FN_00403E20: u32 = 0x0040_3e20;
/// `ECX` = x, one word (a mask): a bool.
const FN_00403E60: u32 = 0x0040_3e60;
/// `ECX` = x: `00403e60` of the word `00825c00(x)` reads, with 1.
const FN_00404710: u32 = 0x0040_4710;
/// The profiling scope: `ECX` = a 4-byte stack object, four words (id, 1,
/// label, line) construct it; `00404ee0` destroys it.
const FN_00404EB0: u32 = 0x0040_4eb0;
const FN_00404EE0: u32 = 0x0040_4ee0;
/// `ECX` = x: whether the word at `x + 0xc` is above 0.
const FN_00405D80: u32 = 0x0040_5d80;
/// `ECX` = x: a bool.
const FN_00405DA0: u32 = 0x0040_5da0;
/// `ECX` = x; returns something not read here.
const FN_004065E0: u32 = 0x0040_65e0;
/// `ECX` = x, one word (0): the cast-source check of `fn_008c39b0`.
const FN_0040A300: u32 = 0x0040_a300;
/// `MagicItem::Preload` (engine map): `ECX` = item, one word.
const FN_0040A420: u32 = 0x0040_a420;
/// `MagicItem::Unload` (engine map): `ECX` = item, one word.
const FN_0040B800: u32 = 0x0040_b800;
/// `ECX` = item, two words (an output string object, a kind): builds the
/// HUD text into the string and returns it.
const FN_0040BEF0: u32 = 0x0040_bef0;
/// `cdecl`, one word.
const FN_0040CA30: u32 = 0x0040_ca30;
/// `cdecl`, one word: returns 0 in this build.
const FN_0040DF90: u32 = 0x0040_df90;
/// `ECX` = x, one word (the byte-mask test): `0040e210(x, 4)` is
/// `byte[x + 0x40] & 4 != 0`.
const FN_0040E210: u32 = 0x0040_e210;
/// `cdecl`, two floats: the smaller (the second on a tie).
const FN_0040EBD0: u32 = 0x0040_ebd0;
/// `ECX` = x; returns `[x + 0x20]` through `007af430` (the actor's base object).
const FN_004181E0: u32 = 0x0041_81e0;
/// `ECX` = extra data list, two words (flag, float).
const FN_00422750: u32 = 0x0042_2750;
/// `ECX` = extra data list: the refraction extra data, 0 when absent.
const FN_00422820: u32 = 0x0042_2820;
/// `ECX` = x; returns the first word of the word list at `x + 0xc`.
const FN_0043B230: u32 = 0x0043_b230;
/// `ECX` = x; the number of entries.
const FN_0043B480: u32 = 0x0043_b480;
/// `ECX` = x, one word (index): the entry.
const FN_0043B4A0: u32 = 0x0043_b4a0;
/// `ECX` = x, one word (flags): the scalar deleting destructor of a worn
/// item (calls `004bc5f0`, frees when bit 0 is set).
const FN_004459E0: u32 = 0x0044_59e0;
/// `ECX` = x; the number of entries (unsigned).
const FN_00453470: u32 = 0x0045_3470;
/// `ECX` = x: a bool.
const FN_00456610: u32 = 0x0045_6610;
/// `ECX` = x; returns `[x + 0x28]`.
const FN_0045CD60: u32 = 0x0045_cd60;
/// `ECX` = list: clears it.
const FN_00470470: u32 = 0x0047_0470;
/// `ECX` = x, one word (a global): the part for that global.
const FN_0047A520: u32 = 0x0047_a520;
/// `ECX` = part, one word.
const FN_0047AA40: u32 = 0x0047_aa40;
/// `ECX` = part, six words.
const FN_0047AAB0: u32 = 0x0047_aab0;
/// `ECX` = x; returns 0.
const FN_0047C850: u32 = 0x0047_c850;
/// `ECX` = x: an unsigned count.
const FN_0048CEE0: u32 = 0x0048_cee0;
/// `ECX` = spell list, one word (spell): a bool (added).
const FN_0048D170: u32 = 0x0048_d170;
/// `ECX` = spell list, one word (spell): a bool (contained).
const FN_0048D830: u32 = 0x0048_d830;
/// `ECX` = spell list, one word (spell): removes it.
const FN_0048D8D0: u32 = 0x0048_d8d0;
/// `ECX` = animation; returns the first word of the list at `+0x74`.
const FN_0048F7F0: u32 = 0x0048_f7f0;
/// `ECX` = part, one float.
const FN_0048FEF0: u32 = 0x0048_fef0;
/// `ECX` = process object, one word (2): a state or 0.
const FN_00491040: u32 = 0x0049_1040;
/// `ECX` = x.
const FN_00496940: u32 = 0x0049_6940;
/// No arguments: returns the word at `011c61bc`.
const FN_00499B70: u32 = 0x0049_9b70;
/// No arguments: returns the word at `011c61c0`.
const FN_00499B80: u32 = 0x0049_9b80;
/// `ECX` = x; returns the word at `x + 8` (`TESActorBaseData` fatigue
/// field, a `u16`).
const FN_004A8AE0: u32 = 0x004a_8ae0;
/// `ECX` = item change; returns its poison entry or 0.
const FN_004BDCC0: u32 = 0x004b_dcc0;
/// `ECX` = item change: run after a poison was cast from it.
const FN_004BDD80: u32 = 0x004b_dd80;
/// `cdecl`, one word (the actor): the actor's inventory changes.
const FN_004BF220: u32 = 0x004b_f220;
/// `ECX` = inventory changes, two words (slot, 0): the worn item entry or 0.
const FN_004C8C10: u32 = 0x004c_8c10;
/// `ECX` = x, one word (an object).
const FN_004DEF90: u32 = 0x004d_ef90;
/// No arguments: a bool.
const FN_00525420: u32 = 0x0052_5420;
/// `ECX` = x.
const FN_00537BD0: u32 = 0x0053_7bd0;
/// `ECX` = x; returns `[x + 0x2c]`.
const FN_0055B980: u32 = 0x0055_b980;
/// `ECX` = actor: a bool; `UpdateMagic` stops when it holds.
const FN_00576D30: u32 = 0x0057_6d30;
/// `cdecl`, one word (an id): the default object of that id.
const FN_0058DB10: u32 = 0x0058_db10;
/// `ECX` = x: the node count of a list (unsigned).
const FN_005AE380: u32 = 0x005a_e380;
/// No arguments: a bool.
const FN_005B9B00: u32 = 0x005b_9b00;
/// `ECX` = animation group data: the group id (translated in
/// `fallout_shared/tesanimgroup.rs`).
const FN_005F2420: u32 = 0x005f_2420;
/// `ECX` = x; returns `[x + 0x34]` (the engine map names it
/// `ActorMover::GetPreferredMoveMode`; the body is a field read).
const FN_005F36F0: u32 = 0x005f_36f0;
/// `ECX` = x; returns `[x + 0x3c]`.
const FN_00639B40: u32 = 0x0063_9b40;
/// `cdecl`, two floats: a float.
const FN_006492A0: u32 = 0x0064_92a0;
/// `cdecl`, two words (a data address, a node): the part found for the node.
const FN_00653270: u32 = 0x0065_3270;
/// `ECX` = caster, one word (the light object).
const FN_006ECD40: u32 = 0x006e_cd40;
/// `cdecl`, one word (the actor).
const FN_007038B0: u32 = 0x0070_38b0;
/// `cdecl`, six words (text, 0, icon path, 0, float, 0): shows a HUD message.
const FN_007052F0: u32 = 0x0070_52f0;
/// `ECX` = list; returns `list + 4` (the first node, stored inline).
const FN_00717E50: u32 = 0x0071_7e50;
/// `ECX` = x; returns `00825c00([x + 0xc])`, that is `[[x + 0xc] + 0x14]`.
const FN_007DF1D0: u32 = 0x007d_f1d0;
/// `ECX` = x; returns `[x + 0x14]`.
const FN_00825C00: u32 = 0x0082_5c00;
/// `ECX` = x: a value compared with 1.
const FN_008041A0: u32 = 0x0080_41a0;
/// `cdecl`, one word (a form type): true for 1, 4 and 10.
const FN_00804B30: u32 = 0x0080_4b30;
/// `ECX` = effect; whether bit `0x100` of `[x + 0x18]` is set.
const FN_00804B60: u32 = 0x0080_4b60;
/// `ECX` = effect.
const FN_00805650: u32 = 0x0080_5650;
/// `ECX` = caster, two words.
const FN_0080F7D0: u32 = 0x0080_f7d0;
/// `ECX` = light object, one word (flags): the scalar deleting destructor.
const FN_0080FCC0: u32 = 0x0080_fcc0;
/// `ECX` = caster, two words (flag, float).
const FN_008153A0: u32 = 0x0081_53a0;
/// `ECX` = caster, one float.
const FN_00818B10: u32 = 0x0081_8b10;
/// `ECX` = new block, two words (radius, item): builds the light object.
const FN_0081BF80: u32 = 0x0081_bf80;
/// `ECX` = light object, one float.
const FN_0081C320: u32 = 0x0081_c320;
/// `ECX` = light object, two words (1, float).
const FN_0081C470: u32 = 0x0081_c470;
/// `cdecl`, one word (the checked target): the object made from it.
const FN_0081E440: u32 = 0x0081_e440;
/// `ECX` = new block, three words (actor, id, float): builds the object.
const FN_0081F580: u32 = 0x0081_f580;
/// `ECX` = magic target, two words (spell magic item, 1): a bool.
const FN_00822B90: u32 = 0x0082_2b90;
/// `ECX` = magic target, four words.
const FN_008230F0: u32 = 0x0082_30f0;
/// `ECX` = magic target, one float.
const FN_00823C40: u32 = 0x0082_3c40;
/// `ECX` = magic target, three words (magic item, 0, 0).
const FN_00824400: u32 = 0x0082_4400;
/// `ECX` = magic target, two words (form, 0).
const FN_008248E0: u32 = 0x0082_48e0;
/// `ECX` = actor, one float.
const FN_0088B510: u32 = 0x0088_b510;
const FN_0088B5A0: u32 = 0x0088_b5a0;
const FN_0088B660: u32 = 0x0088_b660;
/// `ECX` = actor; a float in `ST0`.
const FN_00893500: u32 = 0x0089_3500;
/// `ECX` = actor, two words (1, 1).
const FN_008A7D50: u32 = 0x008a_7d50;
/// `ECX` = actor: a bool.
const FN_008ACE90: u32 = 0x008a_ce90;
/// `ECX` = process, one word (a mask): a bool.
const FN_008ACED0: u32 = 0x008a_ced0;
/// `ECX` = process; a float in `ST0`.
const FN_008BCDD0: u32 = 0x008b_cdd0;
/// `ECX` = list at actor `+0xf4`, one word (the address of a local holding
/// the entry).
const FN_00905330: u32 = 0x0090_5330;
/// `ECX` = player, two words (0 or 1).
const FN_00950A60: u32 = 0x0095_0a60;
const FN_00950BB0: u32 = 0x0095_0bb0;
/// `ECX` = player, one word.
const FN_00962030: u32 = 0x0096_2030;
/// `ECX` = process lists; a float in `ST0` (the clock).
const FN_0096D490: u32 = 0x0096_d490;
/// `ECX` = process lists, one word (the object).
const FN_00973FD0: u32 = 0x0097_3fd0;
/// `ECX` = process lists, two words (actor, key): a bool.
const FN_00974E90: u32 = 0x0097_4e90;
/// `ECX` = effect, one float.
const FN_0098ADB0: u32 = 0x0098_adb0;
/// `cdecl`, one word (a size): returns a new block (`NiSystem`).
const FN_00AA13E0: u32 = 0x00aa_13e0;
/// `cdecl`, six words (source, flag, float, 0, 0.0, 0).
const FN_00B68770: u32 = 0x00b6_8770;
/// `ECX` = calendar, `Calendar::GetTimeScale` (engine map), a float.
const CALENDAR_GET_TIME_SCALE: u32 = 0x0086_7950;

// ---- Helpers

/// `FLD`/`FSTP` of a `float` (and `FCHS` then `FSTP`): a signalling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `pointer ? pointer + offset : 0`, the compiler's null-preserving
/// adjustment to a base class.
fn plus(pointer: u32, offset: u32) -> u32 {
    if pointer == 0 {
        0
    } else {
        pointer + offset
    }
}

fn player(e: &Engine) -> u32 {
    e.global(PLAYER_CHARACTER)
}

/// `__RTDynamicCast(object, 0, source, target, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, source: u32, target: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .u32()
}

/// The first word of `pointer` (`00559450`).
fn first_word(e: &mut Engine, pointer: u32) -> u32 {
    e.call(FIRST_WORD, &args![pointer]).u32()
}

/// The data word of a list node: `[node]` through `006815c0`.
fn node_value(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(slot)
}

/// A virtual call on the actor's magic caster base.
fn caster_vcall(e: &mut Engine, actor: u32, slot: u32, arguments: &[u32]) -> Ret {
    e.vcall(actor + MAGIC_CASTER, slot, arguments)
}

/// `MagicCaster::ReleaseCast(value)` on the actor's magic caster base.
fn release_cast(e: &mut Engine, actor: u32, value: u32) {
    e.call(RELEASE_CAST, &args![actor + MAGIC_CASTER, value]);
}

/// The actor's virtual `+0x17c` with the ten words the use-item code always
/// passes: (item, extra, 1, 0, 0, 0, 0, 0, 1, 0).
fn finish_use(e: &mut Engine, this: Ptr, item: Ptr, extra: Ptr) {
    e.vcall(
        this.addr(),
        ACTOR_SLOT_0X17C,
        &args![item, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

/// Casts one entry of a permanent-magic list: virtual `+0x00` (entry, 1),
/// then virtual `+0x10` (entry, actor + 0x94, 1).
fn cast_list_entry(e: &mut Engine, actor: u32, node: u32) {
    let entry = node_value(e, node);
    if entry != 0 {
        caster_vcall(e, actor, CASTER_SLOT_0X00, &args![entry, 1u32]);
        let magic_target = plus(actor, MAGIC_TARGET);
        caster_vcall(
            e,
            actor,
            CASTER_SLOT_0X10,
            &args![entry, magic_target, 1u32],
        );
    }
}

/// Walks the actor's two effect lists (the list at `+0x7c` of its
/// `004181e0` object, then the list at `+0x2c` of the object virtual
/// `+0x37c` returns, when it returns one), calling `visit(effect, second
/// list)` with the data word of every node.
fn for_each_effect(e: &mut Engine, this: Ptr, mut visit: impl FnMut(&mut Engine, u32, bool)) {
    let base = e.call(FN_004181E0, &args![this]).u32();
    let mut node = e.call(FN_00717E50, &args![base + 0x7c]).u32();
    while node != 0 {
        let effect = node_value(e, node);
        visit(e, effect, false);
        node = e.call(FIELD_4, &args![node]).u32();
    }
    let owner = e.vcall(this.addr(), ACTOR_SLOT_0X37C, &[]).u32();
    let mut node = if owner != 0 {
        let owner = e.vcall(this.addr(), ACTOR_SLOT_0X37C, &[]).u32();
        e.call(FN_00717E50, &args![owner + 0x2c]).u32()
    } else {
        0
    };
    while node != 0 {
        let effect = node_value(e, node);
        visit(e, effect, true);
        node = e.call(FIELD_4, &args![node]).u32();
    }
}

/// Walks the five worn slots of `WORN_SLOT_TABLE`: for each, the worn item
/// (`004c8c10`), its enchantable part's enchantment (0 when it has none) go
/// to `visit(worn, enchantment)`; the item is then deleted.
fn for_each_worn_enchantment(
    e: &mut Engine,
    this: Ptr,
    mut visit: impl FnMut(&mut Engine, u32, u32),
) {
    let changes = e.call(FN_004BF220, &args![this]).u32();
    for index in 0..5u32 {
        let slot: u32 = e.global(WORN_SLOT_TABLE + 4 * index);
        let worn = e.call(FN_004C8C10, &args![changes, slot, 0u32]).u32();
        let enchantable = if worn != 0 {
            let form = e.call(FIELD_8, &args![worn]).u32();
            dynamic_cast(e, form, TYPE_BOUND_OBJECT, TYPE_ENCHANTABLE_FORM)
        } else {
            0
        };
        let enchantment = if enchantable != 0 {
            e.call(FIELD_4, &args![enchantable]).u32()
        } else {
            0
        };
        visit(e, worn, enchantment);
        if worn != 0 {
            e.call(FN_004459E0, &args![worn, 1u32]);
        }
    }
}

/// The default object `0x13` cast to a spell item, 0 when there is none.
fn default_spell(e: &mut Engine) -> u32 {
    let object = e.call(FN_0058DB10, &args![0x13u32]).u32();
    if object == 0 {
        return 0;
    }
    dynamic_cast(e, object, TYPE_FORM, TYPE_SPELL_ITEM)
}

// Translated from 008c1cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0080f7d0` on the actor with two words taken from `worn_item`: its
/// field at `+8` and the value two pointers deep behind its first word
/// (`**worn_item`, 0 when the first word is 0). Returns that call's result.
///
/// `CastPermanentMagic` (`008c26e0`) calls it with the entries of the
/// worn-item list; the callee (`0080f7d0`) is in the `fallout/magic` unit.
pub fn fn_008c1cd0(e: &mut Engine, this: Ptr, worn_item: Ptr) -> bool {
    let first = first_word(e, worn_item.addr());
    let value = if first != 0 {
        let first_again = first_word(e, worn_item.addr());
        node_value(e, first_again)
    } else {
        0
    };
    let item_form = e.call(FIELD_8, &args![worn_item]).u32();
    e.call(FN_0080F7D0, &args![this, item_form, value]).bool()
}

// Translated from 008c1de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Eat` (Xbox PDB): the actor consumes `item`.
///
/// A form whose virtual `+0x94` is true cannot be used: for the player a
/// "cannot use" message is shown. Otherwise the script action flags are set
/// on the actor and the item (`005ac750`), the item's effect list (form
/// `+0x30`, only when virtual `+0xec` says it has one) is handed to the
/// actor's magic caster, and the actor's own virtual `+0x17c` is called to
/// finish the use. The player, unless `consume_now` is set, only queues the
/// item with `PlayerCharacter::AddEatDrinkItem` and finishes the use when
/// that accepts it.
pub fn actor_eat(e: &mut Engine, this: Ptr, item: Ptr, extra: Ptr, consume_now: bool) {
    if item.is_null() {
        return;
    }
    if e.vcall(item.addr(), FORM_SLOT_0X94, &[]).bool() {
        if this.addr() == player(e) {
            let text = e.call(FN_00403DF0, &args![CANNOT_USE_MESSAGE]).u32();
            let scale = quiet(e.global::<f32>(MESSAGE_SCALE_010162C0));
            e.call(
                FN_007052F0,
                &args![text, 0u32, SAD_ICON_PATH, 0u32, scale, 0u32],
            );
        }
        return;
    }
    let singleton: u32 = e.global(OBJECT_011DE45C);
    if !e.call(FN_0047C850, &args![singleton]).bool() {
        e.call(SET_ACTION_FLAG, &args![this, extra, 2u32]);
        let actor_flags = e.call(REFERENCE_EXTRA_LIST, &args![this]).u32();
        e.call(SET_ACTION_FLAG, &args![item, actor_flags, 2u32]);
    }
    let mut effects = 0u32;
    if e.vcall(item.addr(), FORM_SLOT_0XEC, &[]).bool() {
        effects = item.addr() + 0x30;
    }
    let is_player = this.addr() == player(e);
    if !is_player || consume_now {
        caster_vcall(e, this.addr(), CASTER_SLOT_0X40, &args![effects]);
        release_cast(e, this.addr(), 0);
        if !is_player {
            finish_use(e, this, item, extra);
        }
    } else {
        let player_ptr = player(e);
        if e.call(ADD_EAT_DRINK_ITEM, &args![player_ptr, effects])
            .bool()
        {
            finish_use(e, this, item, extra);
        }
    }
}

// Translated from 008c1f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DrinkPotion` (Xbox PDB): like [`actor_eat`] for a potion; false
/// when `item` is null, otherwise true (for the player's queued path, what
/// `PlayerCharacter::AddEatDrinkItem` returned).
pub fn actor_drink_potion(
    e: &mut Engine,
    this: Ptr,
    item: Ptr,
    extra: Ptr,
    consume_now: bool,
) -> bool {
    if item.is_null() {
        return false;
    }
    let mut result = true;
    let effects = item.addr() + 0x30;
    let is_player = this.addr() == player(e);
    if !is_player || consume_now {
        caster_vcall(e, this.addr(), CASTER_SLOT_0X40, &args![effects]);
        release_cast(e, this.addr(), 0);
        if !is_player {
            finish_use(e, this, item, extra);
        }
    } else {
        let player_ptr = player(e);
        result = e
            .call(ADD_EAT_DRINK_ITEM, &args![player_ptr, effects])
            .bool();
        if result {
            finish_use(e, this, item, extra);
        }
    }
    result
}

// Translated from 008c2090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CastAlchemy` (Xbox PDB): uses an alchemy item. `item` must have
/// form type 7 or 8 (virtual `+0x18`), otherwise nothing happens. With
/// `consume` the item is eaten (type 8) or drunk (type 7); without it the
/// item is only handed to the magic caster and released before the actor's
/// virtual `+0x17c` runs.
pub fn actor_cast_alchemy(e: &mut Engine, this: Ptr, item: Ptr, consume: bool) {
    let kind = e.vcall(item.addr(), FORM_SLOT_0X18, &[]).i32();
    if kind != 7 && e.vcall(item.addr(), FORM_SLOT_0X18, &[]).i32() != 8 {
        return;
    }
    let bound = dynamic_cast(e, item.addr(), TYPE_MAGIC_ITEM, TYPE_BOUND_OBJECT);
    if consume {
        if e.vcall(item.addr(), FORM_SLOT_0X18, &[]).i32() == 7 {
            let potion = dynamic_cast(e, item.addr(), TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM);
            actor_drink_potion(e, this, Ptr::new(potion), Ptr::NULL, true);
        } else {
            actor_eat(e, this, Ptr::new(bound), Ptr::NULL, true);
        }
    } else {
        caster_vcall(e, this.addr(), CASTER_SLOT_0X40, &args![item]);
        release_cast(e, this.addr(), 0);
        finish_use(e, this, Ptr::new(bound), Ptr::NULL);
    }
}

// Translated from 008c2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands up to three magic items to the actor's magic caster (virtual `+0x40`
/// sets the source, `+0x48` the target, then `MagicCaster::ReleaseCast`
/// fires): a spell, an effect source found through the item or through
/// `source`, and a poison. Returns false (casting nothing) when `source`
/// answers 3 to its virtual `+0x304`, or when neither of the three exists.
///
/// The spell comes from the actor's own base data (`+0x74` of its `+0x70`
/// member) when `item_change` is null. It is only used when the actor's
/// fatigue value is `0xff` or equals the animation group of the saved
/// acquire object. `out_flag` is cleared and never set again. A fourth item
/// (`[EBP-0xc]`) is tested twice but nothing ever assigns it, so it stays 0.
pub fn fn_008c2190(
    e: &mut Engine,
    this: Ptr,
    item_change: Ptr,
    target: Ptr,
    source: Ptr,
    out_flag: Ptr,
) -> bool {
    let actor = this.addr();
    e.mem.set_u8(out_flag.addr(), 0);
    let mut spell = 0u32;
    let mut effect_source = 0u32;
    let mut poison = 0u32;
    let fourth_item = 0u32;
    if !source.is_null() && e.vcall(source.addr(), 0x304, &[]).i32() == 3 {
        return false;
    }
    if item_change.is_null() {
        let base = e.call(FN_004181E0, &args![this]).u32();
        let node = e.call(FIELD_4, &args![base + 0x70]).u32();
        spell = plus(node, 0x18);
    }
    if spell == 0 && item_change.is_null() && source.is_null() {
        return false;
    }
    if effect_source == 0 && spell == 0 {
        let mut form = 0u32;
        if !item_change.is_null() && e.call(FIELD_8, &args![item_change]).u32() != 0 {
            let inner = e.call(FIELD_8, &args![item_change]).u32();
            if e.call(TYPE_BYTE, &args![inner]).i32() == 0x28 {
                form = e.call(FIELD_8, &args![item_change]).u32();
            }
        }
        if form == 0 && !source.is_null() && e.call(FIELD_F8, &args![source]).u32() != 0 {
            form = e.call(FIELD_F8, &args![source]).u32();
        }
        if !item_change.is_null() {
            poison = e.call(FN_004BDCC0, &args![item_change]).u32();
        }
        if form != 0 {
            effect_source = e.call(FIELD_4, &args![form + 0x74]).u32();
            if effect_source == 0 && !source.is_null() {
                let source_base = e.call(FN_004181E0, &args![source]).u32();
                let owner = if source_base != 0 {
                    let source_base = e.call(FN_004181E0, &args![source]).u32();
                    e.call(FIELD_84, &args![source_base]).u32()
                } else {
                    0
                };
                if owner != 0 {
                    effect_source = e.call(FIELD_4, &args![owner + 0x54]).u32();
                }
            }
        }
    }
    if spell == 0 && effect_source == 0 && poison == 0 && fourth_item == 0 {
        return false;
    }
    let released = if spell != 0 {
        e.call(FN_004181E0, &args![this]).u32()
    } else if !source.is_null() {
        e.call(FIELD_F8, &args![source]).u32()
    } else {
        e.call(FIELD_8, &args![item_change]).u32()
    };
    let target_magic = plus(target.addr(), 0x94);
    if spell != 0 {
        let mut fatigue_matches = false;
        let base = e.call(FN_004181E0, &args![this]).u32();
        let fatigue = e.call(FN_004A8AE0, &args![base + 0x70]).u16() as u32;
        if fatigue == 0xff {
            fatigue_matches = true;
        } else {
            let process = e.call(PROCESS_OF, &args![this]).u32();
            if process != 0 {
                let first = e.vcall(process, PROCESS_SLOT_0X3E8, &[]).u32();
                if first != 0 {
                    let process = e.call(PROCESS_OF, &args![this]).u32();
                    let animation = e.vcall(process, PROCESS_SLOT_0X3E8, &[]).u32();
                    let group_data = e.call(FN_0048F7F0, &args![animation]).u32();
                    if group_data != 0 {
                        let process = e.call(PROCESS_OF, &args![this]).u32();
                        let animation = e.vcall(process, PROCESS_SLOT_0X3E8, &[]).u32();
                        let group_data = e.call(FN_0048F7F0, &args![animation]).u32();
                        if e.call(FN_005F2420, &args![group_data]).u32() == fatigue {
                            fatigue_matches = true;
                        }
                    }
                }
            }
        }
        if fatigue_matches {
            caster_vcall(e, actor, CASTER_SLOT_0X40, &args![spell]);
            caster_vcall(e, actor, CASTER_SLOT_0X48, &args![target_magic]);
            release_cast(e, actor, released);
        }
    }
    if effect_source != 0 {
        caster_vcall(e, actor, CASTER_SLOT_0X40, &args![effect_source + 0x18]);
        caster_vcall(e, actor, CASTER_SLOT_0X48, &args![target_magic]);
        release_cast(e, actor, released);
    }
    if poison != 0 {
        caster_vcall(e, actor, CASTER_SLOT_0X40, &args![poison + 0x30]);
        caster_vcall(e, actor, CASTER_SLOT_0X48, &args![target_magic]);
        release_cast(e, actor, released);
        if !item_change.is_null() {
            e.call(FN_004BDD80, &args![item_change]);
        }
    }
    if fourth_item != 0 {
        caster_vcall(e, actor, CASTER_SLOT_0X40, &args![plus(fourth_item, 0x18)]);
        caster_vcall(e, actor, CASTER_SLOT_0X48, &args![target_magic]);
        release_cast(e, actor, released);
    }
    true
}

// Translated from 008c2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `form` has type byte `0x18` or `0x1a`, casts it to its enchantable
/// part; with an enchantment behind that, calls the magic caster's virtual
/// `+0x14` with (the enchantment's magic item at `+0x18`, `form`, 0).
pub fn fn_008c2630(e: &mut Engine, this: Ptr, form: Ptr, _unused_2: u32) {
    if e.call(TYPE_BYTE, &args![form]).i32() != 0x18
        && e.call(TYPE_BYTE, &args![form]).i32() != 0x1a
    {
        return;
    }
    let enchantable = dynamic_cast(e, form.addr(), TYPE_BOUND_OBJECT, TYPE_ENCHANTABLE_FORM);
    let enchantment = if enchantable != 0 {
        e.call(FIELD_4, &args![enchantable]).u32()
    } else {
        0
    };
    if enchantment != 0 {
        caster_vcall(
            e,
            this.addr(),
            CASTER_SLOT_0X14,
            &args![enchantment + 0x18, form, 0u32],
        );
    }
}

// Translated from 008c26e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CastPermanentMagic` (Xbox PDB). Without `from_worn_items` the
/// magic caster is given every entry of two lists (virtual `+0x00` with
/// (entry, 1), then virtual `+0x10` with (entry, this + 0x94, 1)): the list
/// at `+0x7c` of the base data, and the list at `+0x2c` of the object the
/// actor's virtual `+0x37c` returns (when there is one). With it, the 20
/// worn-item slots are walked; each enchantable item whose enchantment is
/// not yet taken from an earlier slot's item and for which `fn_008c1cd0`
/// accepts it is given to virtual `+0x14` with (enchantment's item, form, 1).
/// Both ways end by casting the default object `0x13` as a spell (when it is
/// not already a spell target of this actor).
pub fn actor_cast_permanent_magic(e: &mut Engine, this: Ptr, from_worn_items: bool) {
    let actor = this.addr();
    if !from_worn_items {
        let base = e.call(FN_004181E0, &args![this]).u32();
        let mut node = e.call(FN_00717E50, &args![base + 0x7c]).u32();
        while node != 0 {
            cast_list_entry(e, actor, node);
            node = e.call(FIELD_4, &args![node]).u32();
        }
        let extra_owner = e.vcall(actor, ACTOR_SLOT_0X37C, &[]).u32();
        let mut node = if extra_owner != 0 {
            let extra_owner = e.vcall(actor, ACTOR_SLOT_0X37C, &[]).u32();
            e.call(FN_00717E50, &args![extra_owner + 0x2c]).u32()
        } else {
            0
        };
        while node != 0 {
            cast_list_entry(e, actor, node);
            node = e.call(FIELD_4, &args![node]).u32();
        }
    } else {
        let changes = e.call(FN_004BF220, &args![this]).u32();
        let mut taken = [0u32; WORN_SLOT_COUNT];
        for slot in 0..WORN_SLOT_COUNT {
            let worn = e
                .call(FN_004C8C10, &args![changes, slot as u32, 0u32])
                .u32();
            let enchantable = if worn != 0 {
                let form = e.call(FIELD_8, &args![worn]).u32();
                dynamic_cast(e, form, TYPE_BOUND_OBJECT, TYPE_ENCHANTABLE_FORM)
            } else {
                0
            };
            let enchantment = if enchantable != 0 {
                e.call(FIELD_4, &args![enchantable]).u32()
            } else {
                0
            };
            if enchantment != 0 {
                let mut seen_before = false;
                let mut earlier = 0;
                while earlier < slot && !seen_before {
                    seen_before = taken[earlier] != 0
                        && e.call(FIELD_8, &args![worn]).u32() == taken[earlier];
                    earlier += 1;
                }
                if !seen_before && fn_008c1cd0(e, this, Ptr::new(worn)) {
                    let form = e.call(FIELD_8, &args![worn]).u32();
                    caster_vcall(
                        e,
                        actor,
                        CASTER_SLOT_0X14,
                        &args![enchantment + 0x18, form, 1u32],
                    );
                    taken[slot] = e.call(FIELD_8, &args![worn]).u32();
                }
            }
            if worn != 0 {
                e.call(FN_004459E0, &args![worn, 1u32]);
            }
        }
    }
    let default_object = e.call(FN_0058DB10, &args![0x13u32]).u32();
    if default_object != 0 {
        let spell = dynamic_cast(e, default_object, TYPE_FORM, TYPE_SPELL_ITEM);
        if spell != 0 {
            let is_target = e
                .call(FN_00822B90, &args![plus(actor, 0x94), spell + 0x18, 1u32])
                .bool();
            if !is_target {
                caster_vcall(e, actor, CASTER_SLOT_0X00, &args![spell, 1u32]);
            }
        }
    }
}

// Translated from 008c2b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the effects of the actor's permanent magic from its magic target
/// (`+0x94`): the entries of both effect lists whose kind (virtual `+0x18` of
/// the entry's member at `+0x18`) `00804b30` accepts, the first list only
/// when `fn_008c2e30` says the entry is not flagged, then the effects of the
/// forms worn in the five slots of `WORN_SLOT_TABLE` that are enchantable, and the
/// default object `0x13` when it is such a spell.
pub fn fn_008c2b60(e: &mut Engine, this: Ptr) {
    let actor = this.addr();
    for_each_effect(e, this, |e, effect, second_list| {
        if effect != 0 {
            let kind = e.vcall(effect + 0x18, 0x18, &[]).u32();
            if e.call(FN_00804B30, &args![kind]).bool()
                && (second_list || !fn_008c2e30(e, Ptr::new(effect)))
            {
                e.call(
                    FN_00824400,
                    &args![plus(actor, 0x94), effect + 0x18, 0u32, 0u32],
                );
            }
        }
    });
    for_each_worn_enchantment(e, this, |e, worn, enchantment| {
        if enchantment != 0 {
            let form = e.call(FIELD_8, &args![worn]).u32();
            e.call(FN_008248E0, &args![plus(actor, 0x94), form, 0u32]);
        }
    });
    let spell = default_spell(e);
    if spell != 0 {
        let kind = e.vcall(spell + 0x18, 0x18, &[]).u32();
        if e.call(FN_00804B30, &args![kind]).bool() {
            e.call(
                FN_00824400,
                &args![plus(actor, 0x94), spell + 0x18, 0u32, 0u32],
            );
        }
    }
}

// Translated from 008c2e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of the byte at `+0x40` is set (`0040e210` with 4).
pub fn fn_008c2e30(e: &mut Engine, this: Ptr) -> bool {
    e.call(FN_0040E210, &args![this, 4u32]).bool()
}

// Translated from 008c2e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Preloads (`0040a420` with 0) the magic items of the actor's permanent
/// magic: the entries of both effect lists that `00804b30` accepts, the
/// enchantments of the forms worn in the five `WORN_SLOT_TABLE` slots, and the default
/// object `0x13` when it is such a spell.
pub fn fn_008c2e50(e: &mut Engine, this: Ptr) {
    for_each_effect(e, this, |e, effect, _second_list| {
        if effect != 0 {
            let kind = e.vcall(effect + 0x18, 0x18, &[]).u32();
            if e.call(FN_00804B30, &args![kind]).bool() {
                e.call(FN_0040A420, &args![effect + 0x18, 0u32]);
            }
        }
    });
    for_each_worn_enchantment(e, this, |e, _worn, enchantment| {
        if enchantment != 0 {
            e.call(FN_0040A420, &args![enchantment + 0x18, 0u32]);
        }
    });
    let spell = default_spell(e);
    if spell != 0 {
        let kind = e.vcall(spell + 0x18, 0x18, &[]).u32();
        if e.call(FN_00804B30, &args![kind]).bool() {
            e.call(FN_0040A420, &args![spell + 0x18, 0u32]);
        }
    }
}

// Translated from 008c30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The mirror of `fn_008c2e50`: unloads (`0040b800` with 1) the same magic
/// items, except that the default object is unloaded with 0.
pub fn fn_008c30a0(e: &mut Engine, this: Ptr) {
    for_each_effect(e, this, |e, effect, _second_list| {
        if effect != 0 {
            let kind = e.vcall(effect + 0x18, 0x18, &[]).u32();
            if e.call(FN_00804B30, &args![kind]).bool() {
                e.call(FN_0040B800, &args![effect + 0x18, 1u32]);
            }
        }
    });
    for_each_worn_enchantment(e, this, |e, _worn, enchantment| {
        if enchantment != 0 {
            e.call(FN_0040B800, &args![enchantment + 0x18, 1u32]);
        }
    });
    let spell = default_spell(e);
    if spell != 0 {
        let kind = e.vcall(spell + 0x18, 0x18, &[]).u32();
        if e.call(FN_00804B30, &args![kind]).bool() {
            e.call(FN_0040B800, &args![spell + 0x18, 0u32]);
        }
    }
}

// Translated from 008c32f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AddSpell` (Xbox PDB): adds `spell` to the base data's spell list
/// (`+0x7c`); when that added something, the base data's virtual `+0x48`
/// runs with 0x10. A spell whose kind `00804b30` accepts that is not yet in
/// the magic target (`00822b90`) is also cast through the magic caster
/// (virtual `+0x00` and `+0x10`). True when either happened.
pub fn actor_add_spell(e: &mut Engine, this: Ptr, spell: Ptr) -> bool {
    let actor = this.addr();
    let base = e.call(FN_004181E0, &args![this]).u32();
    let mut result = e.call(FN_0048D170, &args![base + 0x7c, spell]).bool();
    if result {
        let base = e.call(FN_004181E0, &args![this]).u32();
        e.vcall(base, BASE_DATA_SLOT_0X48, &args![0x10u32]);
    }
    let kind = e.vcall(spell.addr() + 0x18, 0x18, &[]).u32();
    if e.call(FN_00804B30, &args![kind]).bool() {
        let present = e
            .call(
                FN_00822B90,
                &args![plus(actor, 0x94), plus(spell.addr(), 0x18), 1u32],
            )
            .bool();
        if !present {
            result = true;
            caster_vcall(e, actor, CASTER_SLOT_0X00, &args![spell, 0u32]);
            caster_vcall(
                e,
                actor,
                CASTER_SLOT_0X10,
                &args![spell, plus(actor, 0x94), 0u32],
            );
        }
    }
    result
}

// Translated from 008c3400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RemoveSpell` (Xbox PDB): the reverse of [`actor_add_spell`]. Takes
/// `spell` out of the base data's list when it is in it (then virtual `+0x48`
/// with 0x10 on the base data), dispels it from the magic target when
/// `00804b30` accepts its kind and the target has it, and for a form of kind
/// 2 removes the `+0xf4` list entry that `fn_008c4fe0` finds for it. True when
/// something was removed.
pub fn actor_remove_spell(e: &mut Engine, this: Ptr, spell: Ptr) -> bool {
    let actor = this.addr();
    let mut result = false;
    let base = e.call(FN_004181E0, &args![this]).u32();
    if e.call(FN_0048D830, &args![base + 0x7c, spell]).bool() {
        let base = e.call(FN_004181E0, &args![this]).u32();
        e.call(FN_0048D8D0, &args![base + 0x7c, spell]);
        let base = e.call(FN_004181E0, &args![this]).u32();
        e.vcall(base, BASE_DATA_SLOT_0X48, &args![0x10u32]);
        result = true;
    }
    let kind = e.vcall(spell.addr() + 0x18, 0x18, &[]).u32();
    if e.call(FN_00804B30, &args![kind]).bool() {
        let present = e
            .call(
                FN_00822B90,
                &args![plus(actor, 0x94), plus(spell.addr(), 0x18), 1u32],
            )
            .bool();
        if present {
            e.call(
                FN_00824400,
                &args![plus(actor, 0x94), plus(spell.addr(), 0x18), 0u32, 0u32],
            );
            result = true;
        }
    }
    if e.vcall(spell.addr() + 0x18, 0x18, &[]).i32() == 2 && fn_008c4fe0(e, this, spell) {
        fn_008c4f90(e, this, spell);
        result = true;
    }
    result
}

// Translated from 008c3530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c3530 - C++ exception unwinding frame (FS:[0], cookie 011c16bc) not translated
/// Magic caster virtual `+0x20` (`this` is the actor's magic caster at actor
/// `+0x88`; the vtable at `010841f4` is the actor's, RTTI `Actor`). Asks
/// virtual `+0x28` (with the item virtual `+0x34` returns) whether the
/// caster can go on. When it can not, and the actor is the player, a HUD
/// message is built from the item (`0040bef0`, kind taken from the local that
/// `+0x28` wrote) and shown (`007052f0`); when it can, `fn_008c45a0` runs on
/// the actor with 1.
pub fn fn_008c3530(e: &mut Engine, this: Ptr) {
    let actor = this.addr().wrapping_sub(MAGIC_CASTER);
    e.with_stack(12, |e, locals| {
        let kind_slot = locals.addr();
        let text = locals.addr() + 4;
        let item = e.vcall(this.addr(), CASTER_SLOT_0X34, &[]).u32();
        let can_go_on = e
            .vcall(
                this.addr(),
                CASTER_SLOT_0X28,
                &args![item, 0u32, kind_slot, 0u32],
            )
            .bool();
        if can_go_on {
            fn_008c45a0(e, Ptr::new(actor), true);
            return;
        }
        if actor != player(e) {
            return;
        }
        let item = e.vcall(this.addr(), CASTER_SLOT_0X34, &[]).u32();
        let name = e.call(FN_004065E0, &args![item + 0xc]).u32();
        let hashed = e.call(FN_0040CA30, &args![name]).u32();
        let player_ptr = player(e);
        e.call(FN_00962030, &args![player_ptr, hashed]);
        let scale = quiet(e.global::<f32>(MESSAGE_SCALE_010162C0));
        let kind = e.mem.u32(kind_slot);
        let item = e.vcall(this.addr(), CASTER_SLOT_0X34, &[]).u32();
        let message = e.call(FN_0040BEF0, &args![item, text, kind]).u32();
        let first = first_word(e, message);
        e.call(FN_007052F0, &args![first, 0u32, 0u32, 0u32, scale, 0u32]);
        e.call(FN_004037D0, &args![text]);
    });
}

// Translated from 008c3650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x30`: finds the casting item's object. 0 when the
/// actor's virtual `+0x1d0` answers 0. Otherwise it starts from the player's
/// (`00950a60` with 1, when the actor is the player and `00950bb0(0)` yields
/// something that `00456610` accepts) or the actor's virtual `+0x1e4`
/// result, then follows `00496940`, `00537bd0` and the virtual `+0x8c` of
/// the result (with `00499b70`'s global), and returns its virtual `+0xc`.
pub fn fn_008c3650(e: &mut Engine, this: Ptr) -> u32 {
    let actor = this.addr().wrapping_sub(MAGIC_CASTER);
    if e.vcall(actor, ACTOR_SLOT_0X1D0, &[]).u32() == 0 {
        return 0;
    }
    let player_ptr = player(e);
    let start =
        if actor == player_ptr && e.call(FN_00950BB0, &args![player_ptr, 0u32]).u32() != 0 && {
            let sub = e.call(FN_00950BB0, &args![player_ptr, 0u32]).u32();
            e.call(FN_00456610, &args![sub]).bool()
        } {
            e.call(FN_00950A60, &args![player_ptr, 1u32]).u32()
        } else {
            e.vcall(actor, ACTOR_SLOT_0X1E4, &[]).u32()
        };
    let step_one = if start != 0 {
        e.call(FN_00496940, &args![start]).u32()
    } else {
        0
    };
    let step_two = if step_one != 0 {
        e.call(FN_00537BD0, &args![step_one]).u32()
    } else {
        0
    };
    let step_three = if step_two != 0 {
        let argument = e.call(FN_00499B70, &[]).u32();
        e.vcall(step_two, 0x8c, &args![argument]).u32()
    } else {
        0
    };
    if step_three != 0 {
        e.vcall(step_three, 0xc, &[]).u32()
    } else {
        0
    }
}

// Translated from 008c3790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor-side step of `UpdateMagic`: looks at the magic caster's item
/// (virtual `+0x30`, `fn_008c3650`) and target (`+0x34`). Without an item it
/// just runs `fn_008c45a0(0)`. Whether the actor's process (virtual `+0x1e4`)
/// has the state `008041a0() == 1` for `00491040(.., 2)` decides `ready`. With
/// a target, `fn_008c39b0` runs when the item's count `00453470` is 0 and the
/// process is ready. Without one, every entry of the item whose
/// `508100() < 508120()` (two floats read through the entry) gets the item's
/// virtual `+0xf0` with its index; an item left with no entries runs
/// `fn_008c45a0(0)`.
pub fn fn_008c3790(e: &mut Engine, this: Ptr) {
    let actor = this.addr();
    let item = e.vcall(actor + MAGIC_CASTER, CASTER_SLOT_0X30, &[]).u32();
    if item == 0 {
        fn_008c45a0(e, this, false);
        return;
    }
    let target = e.vcall(actor + MAGIC_CASTER, CASTER_SLOT_0X34, &[]).u32();
    let mut ready = false;
    let process = e.vcall(actor, ACTOR_SLOT_0X1E4, &[]).u32();
    if process != 0 {
        // Both branches push 2: the byte local [EBP-0x15] is the constant 0.
        let state = e.call(FN_00491040, &args![process, 2u32]).u32();
        if state != 0 && e.call(FN_008041A0, &args![state]).u32() == 1 {
            ready = true;
        }
    }
    if target != 0 {
        if e.call(FN_00453470, &args![item]).u32() == 0 && ready {
            fn_008c39b0(e, this, Ptr::new(target));
        }
        return;
    }
    if e.call(FN_00453470, &args![item]).u32() != 0 {
        if !ready {
            e.call(FN_004DEF90, &args![item]);
        }
        let mut index = 0u32;
        while index < e.call(FN_0043B480, &args![item]).u32() {
            let entry = e.call(FN_0043B4A0, &args![item, index]).u32();
            let object = if entry != 0 {
                e.call(FN_0043B230, &args![entry]).u32()
            } else {
                0
            };
            let mut effect = 0u32;
            if object != 0 {
                let cast = e
                    .call(FN_00653270, &args![TYPE_DATA_011F36AC, object])
                    .u32();
                if cast != 0 {
                    let global = e.call(FN_00499B80, &[]).u32();
                    effect = e.call(FN_0047A520, &args![cast, global]).u32();
                }
            }
            if effect != 0 {
                let limit = e.call(FLOAT_FIELD_34, &args![effect]).f64();
                let current = e.call(FLOAT_FIELD_30, &args![effect]).f64();
                if current < limit {
                    e.vcall(item, ITEM_SLOT_0XF0, &args![index]);
                }
            }
            index += 1;
        }
    }
    if e.call(FN_00453470, &args![item]).u32() == 0 {
        fn_008c45a0(e, this, false);
    }
}

// Translated from 008c39b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c39b0 - C++ exception unwinding frame (FS:[0], cookie 011c16bc) not translated
/// Part of `UpdateMagic` for a casting target: when `target` (checked with
/// `0040a300(.., 0)`) has a nonzero count at `+0x18` (`0048cee0`), `0081e440`
/// makes an object from it. With the magic caster's item (virtual `+0x30`)
/// and that object it prepares the item, hands the object to the item's
/// virtual `+0xdc`, and, for a target whose `004fd380` is nonzero while
/// `00525420` is false, replaces the caster's light object (caster `+4`,
/// `0x1c` bytes built by `0081bf80`). A float pair read from the object's
/// part (`00508100`) then drives that light (`0098adb0`, `0081c470`,
/// `0081c320`).
pub fn fn_008c39b0(e: &mut Engine, this: Ptr, target: Ptr) {
    let actor = this.addr();
    let caster = actor + MAGIC_CASTER;
    let mut object = 0u32;
    let mut checked = 0u32;
    if !target.is_null() {
        checked = e.call(FN_0040A300, &args![target, 0u32]).u32();
        if checked != 0 && e.call(FN_0048CEE0, &args![checked + 0x18]).u32() != 0 {
            object = e.call(FN_0081E440, &args![checked]).u32();
        }
    }
    let item = e.vcall(caster, CASTER_SLOT_0X30, &[]).u32();
    if item != 0 && object != 0 {
        e.call(FN_004DEF90, &args![item]);
        e.vcall(item, ITEM_SLOT_0XDC, &args![object, 1u32]);
        if e.call(FIELD_70, &args![checked]).u32() != 0 && !e.call(FN_00525420, &[]).bool() {
            let old_light = e.mem.u32(caster + 4);
            if old_light != 0 {
                let old_light = e.mem.u32(caster + 4);
                e.call(FN_0080FCC0, &args![old_light, 1u32]);
            }
            e.mem.set_u32(caster + 4, 0);
            let memory = e.call(FN_00401000, &args![0x1cu32]).u32();
            let light = if memory != 0 {
                let field = e.call(FIELD_70, &args![checked]).u32();
                let radius = e.call(FIELD_C, &args![field]).u32();
                e.call(FN_0081BF80, &args![memory, radius, item]).u32()
            } else {
                0
            };
            e.call(FN_006ECD40, &args![caster, light]);
        }
        let node = e.call(FN_0043B230, &args![object]).u32();
        let part = e.call(FN_00653270, &args![TYPE_DATA_011F36AC, node]).u32();
        let effect = if part != 0 {
            let global = e.call(FN_00499B80, &[]).u32();
            e.call(FN_0047A520, &args![part, global]).u32()
        } else {
            0
        };
        if effect != 0 {
            e.call(FN_0048FEF0, &args![part, 0.0f32]);
            e.call(
                FN_0047AAB0,
                &args![part, effect, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
            );
            e.call(FN_0047AA40, &args![part, 1u32]);
            let negated = -quiet(e.global::<f32>(FADE_01085350));
            e.call(FN_0098ADB0, &args![effect, quiet(negated)]);
            if e.mem.u32(caster + 4) != 0 {
                let current = e.call(FLOAT_FIELD_30, &args![effect]).f64();
                let scale: f64 = e.global(LIGHT_SCALE_0101DE30);
                let value = (current * scale) as f32;
                let light = e.mem.u32(caster + 4);
                e.call(FN_0081C470, &args![light, 1u32, quiet(value)]);
            }
        }
        if e.mem.u32(caster + 4) != 0 {
            let light = e.mem.u32(caster + 4);
            e.call(FN_0081C320, &args![light, 0.0f32]);
        }
    }
}

// Translated from 008c3c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c3c40 - C++ exception unwinding frame (FS:[0], cookie 011c16bc) not translated
/// `Actor::UpdateMagic` (Xbox PDB): the per-frame magic update of the actor.
///
/// Unless `00576d30` says to skip the actor, it measures the time step
/// (`delta`): `3600 / calendar time scale` when `use_game_time` is set, the
/// float at `01017d00` when `use_fixed_step` is set, otherwise the change of
/// the process-list clock (`0096d490`) since the actor's last value at
/// `+0x114` (a clock that went backwards, or a stored value below the
/// constant at `01012060`, restarts it: the step is then 0, or the clock
/// itself when it lies strictly between the constants at `01012060` and
/// `010290d0`, with `+0x114` cleared). The step goes to the process's
/// virtual `+0x700`, the magic caster, the magic target (`00823c40`) and the
/// actor's own updates. The scope object `00404eb0` .. `00404ee0` (id 0x32,
/// line 0x4e5) brackets the work.
pub fn actor_update_magic(e: &mut Engine, this: Ptr, use_game_time: bool, use_fixed_step: bool) {
    let actor = this.addr();
    if e.call(FN_00576D30, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, scope| {
        e.call(
            FN_00404EB0,
            &args![scope, 0x32u32, 1u32, SCOPE_NAME_01085384, 0x4e5u32],
        );
        let current = e.call(FN_0096D490, &args![PROCESS_LISTS]).f32();
        let zero: f64 = e.global(THRESHOLD_01012060);
        let delta = if use_game_time {
            let time_scale = e
                .call(CALENDAR_GET_TIME_SCALE, &args![CALENDAR_OBJECT])
                .f64();
            let seconds: f64 = e.global(SECONDS_PER_HOUR_01012640);
            (seconds / time_scale) as f32
        } else if use_fixed_step {
            quiet(e.global::<f32>(FIXED_STEP_01017D00))
        } else {
            let stored = e.mem.f32(actor + 0x114);
            let mut step = (current as f64 - stored as f64) as f32;
            let stored = e.mem.f32(actor + 0x114);
            // x87 compare of `current` with `stored`: restart when the clock
            // went backwards, or the stored value is below the constant.
            if current < stored || (stored as f64) < zero {
                e.mem.set_f32(actor + 0x114, quiet(current));
                step = 0.0;
                let upper: f64 = e.global(UPPER_BOUND_010290D0);
                if (current as f64) > zero && (current as f64) < upper {
                    e.mem.set_f32(actor + 0x114, 0.0);
                    step = quiet(current);
                }
            }
            step
        };
        let process = e.mem.u32(actor + 0x68);
        if process != 0 {
            e.vcall(process, PROCESS_SLOT_0X700, &args![this, quiet(delta)]);
        }
        if e.vcall(actor + MAGIC_CASTER, CASTER_SLOT_0X34, &[]).u32() != 0 {
            let mut keep = true;
            let process = e.call(PROCESS_OF, &args![this]).u32();
            let animation = e.vcall(process, PROCESS_SLOT_0X3E8, &[]).u32();
            if animation != 0 {
                let group_data = e.call(FN_0048F7F0, &args![animation]).u32();
                let group = e.call(FN_005F2420, &args![group_data]).u32();
                let offset = group.wrapping_sub(0x1a);
                if animation_group_skips_caster_call(offset) {
                    keep = false;
                }
            }
            if keep {
                caster_vcall(e, actor, CASTER_SLOT_0X40, &args![0u32]);
            }
        }
        if fn_008c4560(e, this) {
            fn_008c3790(e, this);
        }
        if e.mem.u32(actor + 0x8c) != 0 {
            e.call(FN_00818B10, &args![actor + MAGIC_CASTER, quiet(delta)]);
        }
        e.call(FN_00823C40, &args![actor + MAGIC_TARGET, quiet(delta)]);
        if !e.vcall(actor, ACTOR_SLOT_0X22C, &args![0u32]).bool() {
            if e.vcall(actor, ACTOR_SLOT_0X360, &[]).bool() {
                e.call(FN_0088B660, &args![this, quiet(delta)]);
            }
            let process = e.mem.u32(actor + 0x68);
            if process != 0 && e.call(FN_008ACED0, &args![process, 0x2000_0000u32]).bool() {
                e.call(FN_0088B510, &args![this, quiet(delta)]);
            }
            let process = e.mem.u32(actor + 0x68);
            let flagged =
                process != 0 && e.call(FN_008ACED0, &args![process, 0x4000_0000u32]).bool();
            if flagged || e.call(FN_00893500, &args![this]).f64() < zero {
                e.call(FN_0088B5A0, &args![this, quiet(delta)]);
            }
            e.vcall(actor, ACTOR_SLOT_0X3DC, &args![quiet(delta)]);
        }
        e.mem.set_f32(actor + 0x114, quiet(current));
        e.call(FN_00404EE0, &args![scope]);
    });
}

// Translated from 008c3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x00` (`this` is the caster at actor `+0x88`):
/// forwards to the actor's process (`008d8520`), virtual `+0x3c0`, with
/// (the caster, `first`, `second`). Does nothing without a process.
pub fn fn_008c3fb0(e: &mut Engine, this: Ptr, first: u32, second: bool) {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return;
    }
    let caster = if actor.is_null() { 0 } else { this.addr() };
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3C0, &args![caster, first, second]);
}

// Translated from 008c4020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x10`: like `fn_008c3fb0` with a third word, to the
/// process's virtual `+0x3c4`: (the caster, `first`, `second`, `third`).
pub fn fn_008c4020(e: &mut Engine, this: Ptr, first: u32, second: u32, third: bool) {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return;
    }
    let caster = if actor.is_null() { 0 } else { this.addr() };
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(
        process,
        PROCESS_SLOT_0X3C4,
        &args![caster, first, second, third],
    );
}

// Translated from 008c4090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic target virtual (`this` is the target at actor `+0x94`): the result
/// of the actor's process virtual `+0x3b8`, 0 without a process.
pub fn fn_008c4090(e: &mut Engine, this: Ptr) -> u32 {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_TARGET));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return 0;
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3B8, &[]).u32()
}

// Translated from 008c4330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetRadiationResistanceMult` (Xbox PDB): the actor's cached
/// radiation resistance multiplier (`+0x19c`). While the cache is below the
/// constant at `01012060` (reset by `fn_008c4400`) it is recomputed as
/// `1 - m * c`, where `m` is `0040ebd0(limit, actor value 0x14)` (0 when
/// below the same constant) and `limit` is the setting at `011d0ad8` when
/// the actor's virtual `+0x360` holds, else the float at `01016410`; `c` is
/// the double at `01016408`.
pub fn actor_get_radiation_resistance_mult(e: &mut Engine, this: Ptr) -> f32 {
    let actor = this.addr();
    let zero: f64 = e.global(THRESHOLD_01012060);
    if (e.mem.f32(actor + 0x19c) as f64) < zero {
        let limit = if e.vcall(actor, ACTOR_SLOT_0X360, &[]).bool() {
            let setting = e.call(FN_00403E20, &args![SETTING_011D0AD8]).u32();
            quiet(e.mem.f32(setting))
        } else {
            quiet(e.global::<f32>(LIMIT_01016410))
        };
        let resistance = e
            .vcall(actor + 0xa4, ACTOR_VALUE_SLOT_0XC, &args![0x14u32])
            .f32();
        let mut mix = e.call(FN_0040EBD0, &args![limit, resistance]).f32();
        if (mix as f64) < zero {
            mix = 0.0;
        }
        let factor: f64 = e.global(FACTOR_01016408);
        e.mem
            .set_f32(actor + 0x19c, (1.0 - mix as f64 * factor) as f32);
    }
    quiet(e.mem.f32(actor + 0x19c))
}

// Translated from 008c4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the cached radiation resistance multiplier (`+0x19c`) to the
/// float at `01012054`.
pub fn fn_008c4400(e: &mut Engine, this: Ptr) {
    let value = quiet(e.global::<f32>(RESET_VALUE_01012054));
    e.mem.set_f32(this.addr() + 0x19c, value);
}

// Translated from 008c4420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x34`: the actor's process virtual `+0x3c8`
/// (0 without a process).
pub fn fn_008c4420(e: &mut Engine, this: Ptr) -> u32 {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return 0;
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3C8, &[]).u32()
}

// Translated from 008c4470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x40`: the actor's process virtual `+0x3cc` with
/// `value`.
pub fn fn_008c4470(e: &mut Engine, this: Ptr, value: u32) {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return;
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3CC, &args![value]);
}

// Translated from 008c44c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x44`: the actor's process virtual `+0x3d0`
/// (0 without a process).
pub fn fn_008c44c0(e: &mut Engine, this: Ptr) -> u32 {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return 0;
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3D0, &[]).u32()
}

// Translated from 008c4510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x48`: the actor's process virtual `+0x3d4` with
/// `value`.
pub fn fn_008c4510(e: &mut Engine, this: Ptr, value: u32) {
    let actor: Ptr = Ptr::new(this.addr().wrapping_sub(MAGIC_CASTER));
    if e.call(PROCESS_OF, &args![actor]).u32() == 0 {
        return;
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_0X3D4, &args![value]);
}

// Translated from 008c4560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's process virtual `+0x3d8` (false without a process).
pub fn fn_008c4560(e: &mut Engine, this: Ptr) -> bool {
    if e.call(PROCESS_OF, &args![this]).u32() == 0 {
        return false;
    }
    let process = e.call(PROCESS_OF, &args![this]).u32();
    e.vcall(process, PROCESS_SLOT_0X3D8, &[]).bool()
}

// Translated from 008c45a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's process virtual `+0x3dc` with `value`; nothing without a
/// process.
pub fn fn_008c45a0(e: &mut Engine, this: Ptr, value: bool) {
    if e.call(PROCESS_OF, &args![this]).u32() == 0 {
        return;
    }
    let process = e.call(PROCESS_OF, &args![this]).u32();
    e.vcall(process, PROCESS_SLOT_0X3DC, &args![value]);
}

// Translated from 008c45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x28`: always true. Clears the integer at
/// `out_count` and sets the float at `out_mult` to 1.0 when they are given.
pub fn fn_008c45e0(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    out_mult: Ptr,
    out_count: Ptr,
    _unused_4: u32,
) -> bool {
    if !out_count.is_null() {
        e.mem.set_u32(out_count.addr(), 0);
    }
    if !out_mult.is_null() {
        e.mem.set_f32(out_mult.addr(), 1.0);
    }
    true
}

// Translated from 008c4610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic caster virtual `+0x3c`: forwards (`flag`, `value`) to `008153a0` on
/// the same caster. Answers its `ST0` (the call at 008c4626 is the last before the RET
/// 008c462e): the slot is `GetMagicEffectivenessModifier`, a `float` in the Xbox PDB
/// (`?GetMagicEffectivenessModifier@Actor@@MAAM_NM@Z`), and MagicCaster::FindTargets_ov2 stores
/// it from the caster's own slot +0x3c call (`FSTP` after 008171ca).
pub fn fn_008c4610(e: &mut Engine, this: Ptr, flag: bool, value: f32) -> f64 {
    e.call(FN_008153A0, &args![this, flag, quiet(value)]).f64()
}

// Translated from 008c40e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic target virtual `+0x24` (`this` is the target at actor `+0x94`): a
/// multiplier for an effect, 1.0 when it does not apply.
///
/// `caster` is a magic caster (cast to `Actor` for a result that is not used
/// any further), `magic_item` the item (virtual `+0x18` is its form type) and
/// `effect` the effect entry. It returns 1.0 early when the item's list at
/// `+0xc` is empty (`00405d80`) or the effect's flag `0x100` is set
/// (`00804b60`); for an alchemy item (type 7) that is not ingestible
/// (`00405da0` of its `+0x3c` part) when `00404710` of the effect's `0084e3a0`
/// is false; and for an ingredient (type 8) whose part at `+0x3c` answers
/// true to its virtual `+4`. Otherwise the result is `006492a0(first,
/// second)`: `first` is the actor value `0x13` (only for an ingestible
/// alchemy item or a type-5 item, else 0), `second` the actor value picked
/// by the effect (`00825c00`, `008d8520`; only while `first` equals the
/// constant at `01012060`), lowered to the setting at `011d0ad8` when the
/// actor's virtual `+0x360` holds.
pub fn fn_008c40e0(e: &mut Engine, this: Ptr, caster: Ptr, magic_item: Ptr, effect: Ptr) -> f32 {
    let actor = this.addr().wrapping_sub(MAGIC_TARGET);
    if !e.call(FN_00405D80, &args![magic_item.addr() + 0xc]).bool()
        || e.call(FN_00804B60, &args![effect]).bool()
    {
        return 1.0;
    }
    let alchemy = if e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() == 7 {
        dynamic_cast(e, magic_item.addr(), TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM)
    } else {
        0
    };
    let ingestible = alchemy != 0 && e.call(FN_00405DA0, &args![alchemy + 0x3c]).bool();
    if alchemy != 0 && !ingestible {
        let part = e.call(FIELD_C, &args![effect]).u32();
        if !e.call(FN_00404710, &args![part]).bool() {
            return 1.0;
        }
    }
    if e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() == 8 {
        let ingredient = dynamic_cast(e, magic_item.addr(), TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM);
        if ingredient != 0 && e.vcall(ingredient + 0x3c, 4, &[]).bool() {
            return 1.0;
        }
    }
    // The cast result is stored and never read again.
    let _casting_actor = dynamic_cast(e, caster.addr(), TYPE_MAGIC_CASTER, TYPE_ACTOR);
    let first = if ingestible || e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() == 5 {
        // The actor value owner sits at actor + 0xa4 (this + 0x10).
        e.vcall(this.addr() + 0x10, ACTOR_VALUE_SLOT_0XC, &args![0x13u32])
            .f32()
    } else {
        0.0
    };
    let zero: f64 = e.global(THRESHOLD_01012060);
    let mut second = 0.0f32;
    if actor != 0 {
        let part = e.call(FIELD_C, &args![effect]).u32();
        let resolved = e.call(FN_00825C00, &args![part]).u32();
        let index = e.call(PROCESS_OF, &args![resolved]).u32();
        if index != 0xffff_ffff && (first as f64) == zero {
            second = e
                .vcall(actor + 0xa4, ACTOR_VALUE_SLOT_0XC, &args![index])
                .f32();
        }
        if e.vcall(actor, ACTOR_SLOT_0X360, &[]).bool() {
            let setting = e.call(FN_00403E20, &args![SETTING_011D0AD8]).u32();
            if e.mem.f32(setting) < second {
                let setting = e.call(FN_00403E20, &args![SETTING_011D0AD8]).u32();
                second = quiet(e.mem.f32(setting));
            }
        }
    }
    e.call(FN_006492A0, &args![first, second]).f32()
}

// Translated from 008c4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UpdateAlpha` (Xbox PDB): sets the actor's alpha through its
/// virtual `+0x41c` (one `float`).
///
/// When `005b9b00` holds and the actor is refractive (`fn_008c51c0`), virtual
/// `+0x384` is called instead with (1, the float at `+0xc` of the
/// refraction extra data) and nothing else happens. Otherwise the alpha is
/// the product of four factors, each 1.0 unless the process (`+0x68`) exists
/// and its `+0x28` word (`0045cd60`) is 0: the process's virtual `+0x5ac`,
/// `008bcdd0` of the process, and (when its virtual `+0x610` is 2) a 1.0
/// that replaces a first factor which is always 1.0 here. For the player the
/// product is also scaled by the float at `011a3b38`.
///
/// A local byte that is only ever 0 (`[EBP-5]`) guards a clamp against the
/// setting at `011d1b38`; the code is kept, it can not run.
pub fn actor_update_alpha(e: &mut Engine, this: Ptr) {
    let actor = this.addr();
    if e.call(FN_005B9B00, &[]).bool() && fn_008c51c0(e, this) {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![this]).u32();
        let extra = e.call(FN_00422820, &args![list]).u32();
        let value = quiet(e.mem.f32(extra + 0xc));
        e.vcall(actor, ACTOR_SLOT_0X384, &args![1u32, value]);
        return;
    }
    let base_factor = 1.0f32;
    let never_set = false;
    let mut flag_factor = if never_set { 0.0f32 } else { 1.0 };
    let mut process_factor = 1.0f32;
    let mut alpha_mult = 1.0f32;
    let process = e.mem.u32(actor + 0x68);
    if process != 0 {
        let process = e.mem.u32(actor + 0x68);
        if e.call(FN_0045CD60, &args![process]).u32() == 0 {
            let process = e.mem.u32(actor + 0x68);
            alpha_mult = e.vcall(process, PROCESS_SLOT_0X5AC, &[]).f32();
            let process = e.mem.u32(actor + 0x68);
            process_factor = e.call(FN_008BCDD0, &args![process]).f32();
            let process = e.mem.u32(actor + 0x68);
            if e.vcall(process, PROCESS_SLOT_0X610, &[]).i32() == 2 {
                flag_factor = 1.0;
            }
        }
    }
    let mut alpha = (((base_factor as f64 * flag_factor as f64) * process_factor as f64)
        * alpha_mult as f64) as f32;
    if actor == player(e) {
        if never_set {
            let setting = e.call(FN_00403E20, &args![SETTING_011D1B38]).u32();
            if (e.mem.f32(setting) as f64) > alpha as f64 {
                let setting = e.call(FN_00403E20, &args![SETTING_011D1B38]).u32();
                alpha = quiet(e.mem.f32(setting));
            }
        }
        let scale: f32 = e.global(PLAYER_ALPHA_SCALE_011A3B38);
        alpha = (alpha as f64 * scale as f64) as f32;
    }
    e.vcall(actor, ACTOR_SLOT_0X41C, &args![quiet(alpha)]);
}

// Translated from 008c4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `value` to `fn_008c47d0` of the actor's process when there is one
/// and its `+0x28` word (`0045cd60`) is 0.
pub fn fn_008c4790(e: &mut Engine, this: Ptr, value: f32) {
    let actor = this.addr();
    if e.mem.u32(actor + 0x68) != 0 {
        let process = e.mem.u32(actor + 0x68);
        if e.call(FN_0045CD60, &args![process]).u32() == 0 {
            let process = e.mem.u32(actor + 0x68);
            fn_008c47d0(e, Ptr::new(process), value);
        }
    }
}

// Translated from 008c47d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x2b0` (the process's alpha multiplier).
pub fn fn_008c47d0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x2b0, quiet(value));
}

// Translated from 008c47f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_008c4830` of the actor's process when there is one and its `+0x28`
/// word (`0045cd60`) is 0, otherwise 1.0.
pub fn fn_008c47f0(e: &mut Engine, this: Ptr) -> f32 {
    let actor = this.addr();
    if e.mem.u32(actor + 0x68) != 0 {
        let process = e.mem.u32(actor + 0x68);
        if e.call(FN_0045CD60, &args![process]).u32() == 0 {
            let process = e.mem.u32(actor + 0x68);
            return fn_008c4830(e, Ptr::new(process));
        }
    }
    1.0
}

// Translated from 008c4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x2b0`.
pub fn fn_008c4830(e: &mut Engine, this: Ptr) -> f32 {
    quiet(e.mem.f32(this.addr() + 0x2b0))
}

// Translated from 008c4850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAlphaMult` (Xbox PDB): the process's virtual `+0x5ac`, 1.0
/// without a process.
pub fn actor_get_alpha_mult(e: &mut Engine, this: Ptr) -> f32 {
    let actor = this.addr();
    if e.mem.u32(actor + 0x68) != 0 {
        let process = e.mem.u32(actor + 0x68);
        e.vcall(process, PROCESS_SLOT_0X5AC, &[]).f32()
    } else {
        1.0
    }
}

// Translated from 008c4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetAlphaMult` (Xbox PDB): the process's virtual `+0x5b0` with
/// `value`; nothing without a process.
pub fn actor_set_alpha_mult(e: &mut Engine, this: Ptr, value: f32) {
    let actor = this.addr();
    if e.mem.u32(actor + 0x68) != 0 {
        let process = e.mem.u32(actor + 0x68);
        e.vcall(process, PROCESS_SLOT_0X5B0, &args![quiet(value)]);
    }
}

// Translated from 008c48d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Magic target virtual `+0x00` (`this` is the target at actor `+0x94`):
/// applies an effect. Returns false when `008ace90` holds for the actor but
/// the item's form type (virtual `+0x18`) is not 4. Otherwise it runs
/// `008230f0(caster, magic_item, effect, 0)` on the target and returns its
/// result, and for an item whose list at `+0xc` is not empty and whose kind
/// `00804b30` does not accept, with the caster cast to `Actor` and a
/// different `007af430` value than this actor's: sets action flag `0x80`
/// (`005ac750`), and (unless the item is an enchantment of kind 2 whose
/// `00639b40` holder does not have type `0x29` with `flag` set, or type 7) calls
/// the actor's virtual `+0x4bc` with (the caster actor, `effect`); the player
/// as caster actor is told with `007038b0`.
pub fn fn_008c48d0(
    e: &mut Engine,
    this: Ptr,
    caster: Ptr,
    magic_item: Ptr,
    effect: Ptr,
    flag: bool,
) -> bool {
    let actor = this.addr().wrapping_sub(MAGIC_TARGET);
    if e.call(FN_008ACE90, &args![actor]).bool()
        && e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() != 4
    {
        return false;
    }
    let result = e
        .call(FN_008230F0, &args![this, caster, magic_item, effect, 0u32])
        .bool();
    if e.call(FN_00405D80, &args![magic_item.addr() + 0xc]).bool() {
        let kind = e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).u32();
        if !e.call(FN_00804B30, &args![kind]).bool() {
            let caster_actor = dynamic_cast(e, caster.addr(), TYPE_MAGIC_CASTER, TYPE_ACTOR);
            if caster_actor != 0 {
                let caster_value = e.call(FIELD_20, &args![caster_actor]).u32();
                let own_value = e.call(FIELD_20, &args![actor]).u32();
                if caster_value != own_value {
                    let list = e.call(REFERENCE_EXTRA_LIST, &args![actor]).u32();
                    e.call(SET_ACTION_FLAG, &args![caster_actor, list, 0x80u32]);
                    let mut apply = true;
                    if e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() == 6 {
                        let enchantment = dynamic_cast(
                            e,
                            magic_item.addr(),
                            TYPE_MAGIC_ITEM,
                            TYPE_ENCHANTMENT_ITEM,
                        );
                        if enchantment != 0 && e.call(FN_005F36F0, &args![enchantment]).i32() == 2 {
                            let holder = e.call(FN_00639B40, &args![effect]).u32();
                            // The exe reads the type byte a second time only when the first
                            // test passes, so both reads stay, in order.
                            if holder == 0
                                || e.call(TYPE_BYTE, &args![holder]).i32() != 0x29
                                || (e.call(TYPE_BYTE, &args![holder]).i32() == 0x29 && !flag)
                            {
                                apply = false;
                            }
                        }
                    }
                    if e.vcall(magic_item.addr(), FORM_SLOT_0X18, &[]).i32() == 7 {
                        apply = false;
                    }
                    if apply {
                        e.vcall(actor, ACTOR_SLOT_0X4BC, &args![caster_actor, effect]);
                    }
                    if caster_actor == player(e) {
                        e.call(FN_007038B0, &args![actor]);
                    }
                }
            }
        }
    }
    result
}

// Translated from 008c4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EffectAdded` (Xbox PDB), a magic target virtual (`this` is the
/// target at actor `+0x94`): reacts to the active effect `effect` (not null,
/// and the actor needs a process). The process's virtual `+0x5c8` is asked
/// twice about the pair (target virtual `+8`, the `0040df90` value of the
/// effect's `007df1d0` part): the first time its virtual `+0x5cc` follows
/// when the answer is false, the second time `00974e90` on the process list
/// and `00805650` on the effect follow when it is true. Then, for an effect
/// whose `0084e3a0` part passes `00404710` and whose type (`0055b980`) is
/// not 4, when `00403e60` of the part with `0x1000000` is false and no
/// ingestible flag was found (the item is an ingredient or alchemy item
/// whose `+0x3c` part answers true to its virtual `+4`), `008a7d50` runs on
/// the actor with (1, 1).
pub fn fn_008c4ab0(e: &mut Engine, this: Ptr, effect: Ptr) {
    let actor = this.addr().wrapping_sub(MAGIC_TARGET);
    let process_slot = this.addr().wrapping_sub(0x2c);
    if effect.is_null() || e.mem.u32(process_slot) == 0 {
        return;
    }
    let part = e.call(FN_007DF1D0, &args![effect]).u32();
    let key = e.call(FN_0040DF90, &args![part]).u32();
    if key != 0 {
        let target_value = e.vcall(this.addr(), TARGET_SLOT_0X8, &[]).u32();
        let process = e.mem.u32(process_slot);
        if !e
            .vcall(process, PROCESS_SLOT_0X5C8, &args![target_value, key])
            .bool()
        {
            let process = e.mem.u32(process_slot);
            e.vcall(process, PROCESS_SLOT_0X5CC, &[]);
        }
    }
    let part = e.call(FN_007DF1D0, &args![effect]).u32();
    let key = e.call(FN_0040DF90, &args![part]).u32();
    if key != 0 {
        let target_value = e.vcall(this.addr(), TARGET_SLOT_0X8, &[]).u32();
        let process = e.mem.u32(process_slot);
        if e.vcall(process, PROCESS_SLOT_0X5C8, &args![target_value, key])
            .bool()
            && e.call(FN_00974E90, &args![PROCESS_LISTS, actor, key])
                .bool()
        {
            e.call(FN_00805650, &args![effect]);
        }
    }
    let mut ingestible = false;
    let first_type = e.call(FN_0055B980, &args![effect]).i32();
    if first_type != 7 || e.call(FN_0055B980, &args![effect]).i32() != 8 {
        let form = e.call(FIELD_8, &args![effect]).u32();
        let ingredient = dynamic_cast(e, form, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM);
        if ingredient != 0 {
            ingestible = e.vcall(ingredient + 0x3c, 4, &[]).bool();
        } else {
            let form = e.call(FIELD_8, &args![effect]).u32();
            let alchemy = dynamic_cast(e, form, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM);
            if alchemy != 0 {
                ingestible = e.vcall(alchemy + 0x3c, 4, &[]).bool();
            }
        }
    }
    let part = e.call(FIELD_C, &args![effect]).u32();
    if e.call(FN_00404710, &args![part]).bool() && e.call(FN_0055B980, &args![effect]).i32() != 4 {
        let list = e.call(FN_007DF1D0, &args![effect]).u32();
        if !e.call(FN_00403E60, &args![list, 0x0100_0000u32]).bool() && !ingestible {
            e.call(FN_008A7D50, &args![actor, 1u32, 1u32]);
        }
    }
}

// Translated from 008c4cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A magic target virtual (`this` is the target at actor `+0x94`): when the
/// effect `effect` has a `0040df90` value for its `007df1d0` part and the
/// actor has a process, calls the process's virtual `+0x5cc`.
pub fn fn_008c4cb0(e: &mut Engine, this: Ptr, effect: Ptr) {
    let process_slot = this.addr().wrapping_sub(0x2c);
    if !effect.is_null() && e.mem.u32(process_slot) != 0 {
        let part = e.call(FN_007DF1D0, &args![effect]).u32();
        let key = e.call(FN_0040DF90, &args![part]).u32();
        if key != 0 {
            let process = e.mem.u32(process_slot);
            e.vcall(process, PROCESS_SLOT_0X5CC, &[]);
        }
    }
}

// Translated from 008c4d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c4d10 - C++ exception unwinding frame (FS:[0], cookie 011c16bc) not translated
/// Builds the `0x6c`-byte object `0081f580(actor, 008c4e00(), 0.0)` for this
/// actor (`this` is the magic target at actor `+0x94`) and, when its virtual
/// `+0xc4` is true, registers it with the process lists (`00973fd0`);
/// otherwise it is deleted through its virtual `+0` with 1.
pub fn fn_008c4d10(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    let actor = this.addr().wrapping_sub(MAGIC_TARGET);
    let id = fn_008c4e00(e);
    let memory = e.call(FN_00AA13E0, &args![0x6cu32]).u32();
    let object = if memory != 0 {
        e.call(FN_0081F580, &args![memory, actor, id, 0.0f32]).u32()
    } else {
        0
    };
    if e.vcall(object, OBJECT_SLOT_0XC4, &[]).bool() {
        e.call(FN_00973FD0, &args![PROCESS_LISTS, object]);
    } else if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 008c4e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c3584`.
pub fn fn_008c4e00(e: &mut Engine) -> u32 {
    e.global(ID_011C3584)
}

// Translated from 008c4e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c4e10 - C++ exception unwinding frame (FS:[0], cookie 011c16bc) not translated
/// The same as `fn_008c4d10`, with the id from `fn_008c4f00`.
pub fn fn_008c4e10(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    let actor = this.addr().wrapping_sub(MAGIC_TARGET);
    let id = fn_008c4f00(e);
    let memory = e.call(FN_00AA13E0, &args![0x6cu32]).u32();
    let object = if memory != 0 {
        e.call(FN_0081F580, &args![memory, actor, id, 0.0f32]).u32()
    } else {
        0
    };
    if e.vcall(object, OBJECT_SLOT_0XC4, &[]).bool() {
        e.call(FN_00973FD0, &args![PROCESS_LISTS, object]);
    } else if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 008c4f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c3580`.
pub fn fn_008c4f00(e: &mut Engine) -> u32 {
    e.global(ID_011C3580)
}

// Translated from 008c4f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every node of the list the actor's magic target virtual `+8` returns
/// (while `005ae380` accepts the node), calls the node's object's virtual
/// `+0x38` with `value`.
pub fn fn_008c4f10(e: &mut Engine, this: Ptr, value: u32) {
    if value == 0 {
        return;
    }
    let mut node = e
        .vcall(this.addr() + MAGIC_TARGET, TARGET_SLOT_0X8, &[])
        .u32();
    while node != 0 {
        if e.call(FN_005AE380, &args![node]).u32() == 0 {
            break;
        }
        let slot = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(slot) != 0 {
            let object = node_value(e, node);
            e.vcall(object, OBJECT_SLOT_0X38, &args![value]);
        }
        node = e.call(FIELD_4, &args![node]).u32();
    }
}

// Translated from 008c4f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the entry `fn_008c5000` finds for `key` out of the list at `+0xf4`
/// (`00905330`) and frees it (`00401030`).
pub fn fn_008c4f90(e: &mut Engine, this: Ptr, key: Ptr) {
    let found = fn_008c5000(e, this, key);
    if found == 0 {
        return;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), found);
        e.call(FN_00905330, &args![this.addr() + 0xf4, local]);
        let entry = e.mem.u32(local.addr());
        e.call(FN_00401030, &args![entry]);
    });
}

// Translated from 008c4fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `fn_008c5000` finds an entry for `key`.
pub fn fn_008c4fe0(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
    fn_008c5000(e, this, key) != 0
}

// Translated from 008c5000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list at `+0xf4` for the first node whose object (the node's
/// first word) starts with the word `key` and returns that object; 0 when
/// `key` is null, the list is empty or nothing matches.
pub fn fn_008c5000(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let mut found = 0u32;
    if !key.is_null() {
        let head = this.addr() + 0xf4;
        let slot = e.call(IDENTITY, &args![head]).u32();
        if e.mem.u32(slot) != 0 {
            let mut node = head;
            while node != 0 && found == 0 {
                let slot = e.call(IDENTITY, &args![node]).u32();
                if e.mem.u32(slot) != 0 {
                    let object = node_value(e, node);
                    if e.mem.u32(object) == key.addr() {
                        found = node_value(e, node);
                    }
                }
                node = e.call(FIELD_4, &args![node]).u32();
            }
        }
    }
    found
}

// Translated from 008c5090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DeleteCastPowers` (Xbox PDB): frees (`00401030`) every object in
/// the list at `+0xf4`, then clears the list (`00470470`).
pub fn actor_delete_cast_powers(e: &mut Engine, this: Ptr) {
    let head = this.addr() + 0xf4;
    let mut node = head;
    while node != 0 {
        let object = node_value(e, node);
        if object != 0 {
            e.call(FN_00401030, &args![object]);
        }
        node = e.call(FIELD_4, &args![node]).u32();
    }
    e.call(FN_00470470, &args![head]);
}

// Translated from 008c50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the actor's refraction. `value` is the amount: with `enabled` and
/// `value <= 0` the process's virtual `+0x5b4` replaces it, and `enabled`
/// is cleared again when it is still `<= 0`. `00b68770` then gets the
/// result of the actor's virtual `+0x1d0`, `enabled`, the amount, 0, 0.0 and
/// 0; and the refraction extra data (`00422750`) of the actor's extra list
/// gets (`enabled`, the original `value`).
pub fn fn_008c50f0(e: &mut Engine, this: Ptr, enabled: bool, value: f32) {
    let actor = this.addr();
    let zero: f64 = e.global(THRESHOLD_01012060);
    let mut amount = quiet(value);
    if enabled && (amount as f64) <= zero && e.call(PROCESS_OF, &args![this]).u32() != 0 {
        let process = e.call(PROCESS_OF, &args![this]).u32();
        amount = e.vcall(process, PROCESS_SLOT_0X5B4, &[]).f32();
    }
    let mut flag = enabled;
    if flag && (amount as f64) <= zero {
        flag = false;
    }
    let source = e.vcall(actor, ACTOR_SLOT_0X1D0, &[]).u32();
    e.call(
        FN_00B68770,
        &args![source, flag, amount, 0u32, 0.0f32, 0u32],
    );
    let list = e.call(REFERENCE_EXTRA_LIST, &args![this]).u32();
    if list != 0 {
        e.call(FN_00422750, &args![list, flag, quiet(value)]);
    }
}

// Translated from 008c51c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsRefractive` (Xbox PDB): whether the actor's extra data list
/// (`+0x44`) has the refraction extra data (`00422820`).
pub fn fn_008c51c0(e: &mut Engine, this: Ptr) -> bool {
    let list = e.call(REFERENCE_EXTRA_LIST, &args![this]).u32();
    list != 0 && e.call(FN_00422820, &args![list]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c1cd0, fn_008c1cd0(Ptr, Ptr) -> bool),
        entry!(0x008c1de0, actor_eat(Ptr, Ptr, Ptr, bool)),
        entry!(0x008c1f80, actor_drink_potion(Ptr, Ptr, Ptr, bool) -> bool),
        entry!(0x008c2090, actor_cast_alchemy(Ptr, Ptr, bool)),
        entry!(0x008c2190, fn_008c2190(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x008c2630, fn_008c2630(Ptr, Ptr, u32)),
        entry!(0x008c26e0, actor_cast_permanent_magic(Ptr, bool)),
        entry!(0x008c2b60, fn_008c2b60(Ptr)),
        entry!(0x008c2e30, fn_008c2e30(Ptr) -> bool),
        entry!(0x008c2e50, fn_008c2e50(Ptr)),
        entry!(0x008c30a0, fn_008c30a0(Ptr)),
        entry!(0x008c32f0, actor_add_spell(Ptr, Ptr) -> bool),
        entry!(0x008c3400, actor_remove_spell(Ptr, Ptr) -> bool),
        entry!(0x008c3530, fn_008c3530(Ptr)),
        entry!(0x008c3650, fn_008c3650(Ptr) -> u32),
        entry!(0x008c3790, fn_008c3790(Ptr)),
        entry!(0x008c39b0, fn_008c39b0(Ptr, Ptr)),
        entry!(0x008c3c40, actor_update_magic(Ptr, bool, bool)),
        entry!(0x008c3fb0, fn_008c3fb0(Ptr, u32, bool)),
        entry!(0x008c4020, fn_008c4020(Ptr, u32, u32, bool)),
        entry!(0x008c4090, fn_008c4090(Ptr) -> u32),
        entry!(0x008c40e0, fn_008c40e0(Ptr, Ptr, Ptr, Ptr) -> f32),
        entry!(0x008c4330, actor_get_radiation_resistance_mult(Ptr) -> f32),
        entry!(0x008c4400, fn_008c4400(Ptr)),
        entry!(0x008c4420, fn_008c4420(Ptr) -> u32),
        entry!(0x008c4470, fn_008c4470(Ptr, u32)),
        entry!(0x008c44c0, fn_008c44c0(Ptr) -> u32),
        entry!(0x008c4510, fn_008c4510(Ptr, u32)),
        entry!(0x008c4560, fn_008c4560(Ptr) -> bool),
        entry!(0x008c45a0, fn_008c45a0(Ptr, bool)),
        entry!(0x008c45e0, fn_008c45e0(Ptr, u32, Ptr, Ptr, u32) -> bool),
        entry!(0x008c4610, fn_008c4610(Ptr, bool, f32) -> f64),
        entry!(0x008c4640, actor_update_alpha(Ptr)),
        entry!(0x008c4790, fn_008c4790(Ptr, f32)),
        entry!(0x008c47d0, fn_008c47d0(Ptr, f32)),
        entry!(0x008c47f0, fn_008c47f0(Ptr) -> f32),
        entry!(0x008c4830, fn_008c4830(Ptr) -> f32),
        entry!(0x008c4850, actor_get_alpha_mult(Ptr) -> f32),
        entry!(0x008c4890, actor_set_alpha_mult(Ptr, f32)),
        entry!(0x008c48d0, fn_008c48d0(Ptr, Ptr, Ptr, Ptr, bool) -> bool),
        entry!(0x008c4ab0, fn_008c4ab0(Ptr, Ptr)),
        entry!(0x008c4cb0, fn_008c4cb0(Ptr, Ptr)),
        entry!(0x008c4d10, fn_008c4d10(Ptr, u32, u32)),
        entry!(0x008c4e00, fn_008c4e00() -> u32),
        entry!(0x008c4e10, fn_008c4e10(Ptr, u32, u32)),
        entry!(0x008c4f00, fn_008c4f00() -> u32),
        entry!(0x008c4f10, fn_008c4f10(Ptr, u32)),
        entry!(0x008c4f90, fn_008c4f90(Ptr, Ptr)),
        entry!(0x008c4fe0, fn_008c4fe0(Ptr, Ptr) -> bool),
        entry!(0x008c5000, fn_008c5000(Ptr, Ptr) -> u32),
        entry!(0x008c5090, actor_delete_cast_powers(Ptr)),
        entry!(0x008c50f0, fn_008c50f0(Ptr, bool, f32)),
        entry!(0x008c51c0, fn_008c51c0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Vtables and the fake functions behind their slots live outside the
    // executable image and the heap.
    const FORM_VT: u32 = 0x7c10_0000;
    const ACTOR_VT: u32 = 0x7c20_0000;
    const CASTER_VT: u32 = 0x7c30_0000;
    const TARGET_VT: u32 = 0x7c40_0000;
    const PROCESS_VT: u32 = 0x7c50_0000;
    const VALUE_VT: u32 = 0x7c60_0000;
    const OTHER_VT: u32 = 0x7c70_0000;

    /// The address of the fake function behind `slot` of `vtable`.
    fn slot_fn(vtable: u32, slot: u32) -> u32 {
        vtable + 0x1_0000 + slot
    }

    /// Every callee outside the unit, answered with a default result until a
    /// test says otherwise.
    const STUBBED: &[u32] = &[
        RELEASE_CAST,
        SET_ACTION_FLAG,
        ADD_EAT_DRINK_ITEM,
        CALENDAR_GET_TIME_SCALE,
        FN_00401000,
        FN_00401030,
        FN_004037D0,
        FN_00403DF0,
        FN_00403E20,
        FN_00403E60,
        FN_00404710,
        FN_00404EB0,
        FN_00404EE0,
        FN_00405D80,
        FN_00405DA0,
        FN_004065E0,
        FN_0040A300,
        FN_0040A420,
        FN_0040B800,
        FN_0040BEF0,
        FN_0040CA30,
        FN_0040DF90,
        FN_0040E210,
        FN_0040EBD0,
        FN_004181E0,
        FN_00422750,
        FN_00422820,
        FN_0043B230,
        FN_0043B480,
        FN_0043B4A0,
        FN_004459E0,
        FN_00453470,
        FN_00456610,
        FN_0045CD60,
        FN_00470470,
        FN_0047A520,
        FN_0047AA40,
        FN_0047AAB0,
        FN_0047C850,
        FN_0048CEE0,
        FN_0048D170,
        FN_0048D830,
        FN_0048D8D0,
        FN_0048F7F0,
        FN_0048FEF0,
        FN_00491040,
        FN_00496940,
        FN_00499B70,
        FN_00499B80,
        FN_004A8AE0,
        FN_004BDCC0,
        FN_004BDD80,
        FN_004BF220,
        FN_004C8C10,
        FN_004DEF90,
        FN_00525420,
        FN_00537BD0,
        FN_0055B980,
        FN_00576D30,
        FN_0058DB10,
        FN_005AE380,
        FN_005B9B00,
        FN_005F2420,
        FN_005F36F0,
        FN_00639B40,
        FN_006492A0,
        FN_00653270,
        FN_006ECD40,
        FN_007038B0,
        FN_007052F0,
        FN_00717E50,
        FN_007DF1D0,
        FN_00825C00,
        FN_008041A0,
        FN_00804B30,
        FN_00804B60,
        FN_00805650,
        FN_0080F7D0,
        FN_0080FCC0,
        FN_008153A0,
        FN_00818B10,
        FN_0081BF80,
        FN_0081C320,
        FN_0081C470,
        FN_0081E440,
        FN_0081F580,
        FN_00822B90,
        FN_008230F0,
        FN_00823C40,
        FN_00824400,
        FN_008248E0,
        FN_0088B510,
        FN_0088B5A0,
        FN_0088B660,
        FN_00893500,
        FN_008A7D50,
        FN_008ACE90,
        FN_008ACED0,
        FN_008BCDD0,
        FN_00905330,
        FN_00950A60,
        FN_00950BB0,
        FN_00962030,
        FN_0096D490,
        FN_00973FD0,
        FN_00974E90,
        FN_0098ADB0,
        FN_00AA13E0,
        FN_00B68770,
    ];

    /// What the tiny field readers, the kind tables and the settings answer;
    /// and the pages of exe data the unit reads.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_9000,
            0x0108_5000,
            0x011a_3000,
            0x011c_3000,
            0x011d_0000,
            0x011d_1000,
            0x011d_3000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        for &address in STUBBED {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FIELD_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FIELD_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FIELD_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(FIELD_20, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FIELD_70, |e, a| e.mem.u32(a[0] + 0x70).into_ret());
        e.register(FIELD_84, |e, a| e.mem.u32(a[0] + 0x84).into_ret());
        e.register(FIELD_F8, |e, a| e.mem.u32(a[0] + 0xf8).into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.register(TYPE_BYTE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(PROCESS_OF, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(FN_004181E0, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FN_00717E50, |_, a| (a[0] + 4).into_ret());
        e.register(FN_00825C00, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(FN_007DF1D0, |e, a| {
            let part = e.mem.u32(a[0] + 0xc);
            e.mem.u32(part + 0x14).into_ret()
        });
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(FN_0045CD60, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(FN_0055B980, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(FN_00639B40, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(FN_005F36F0, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(FN_004A8AE0, |e, a| {
            u32::from(e.mem.u16(a[0] + 8)).into_ret()
        });
        e.register(FN_00804B30, |_, a| matches!(a[0], 1 | 4 | 10).into_ret());
        e.register(FN_00804B60, |e, a| {
            (e.mem.u32(a[0] + 0x18) & 0x100 != 0).into_ret()
        });
        e.register(FN_00405D80, |e, a| (e.mem.i32(a[0] + 0xc) > 0).into_ret());
        e.register(FN_0040E210, |e, a| {
            (u32::from(e.mem.u8(a[0] + 0x40)) & a[1] != 0).into_ret()
        });
        e.register(FN_00403DF0, |e, a| {
            (if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 4) }).into_ret()
        });
        e.register(FN_00403E20, |_, a| (a[0] + 4).into_ret());
        e.register(FLOAT_FIELD_30, |e, a| e.mem.f32(a[0] + 0x30).into_ret());
        e.register(FLOAT_FIELD_34, |e, a| e.mem.f32(a[0] + 0x34).into_ret());
        e.register(FN_0040EBD0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(FN_006492A0, |_, a| {
            (f32::from_bits(a[0]) + f32::from_bits(a[1])).into_ret()
        });
        // `__RTDynamicCast` answers the object it was given until a test
        // says otherwise.
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Forms and sub-objects answer their kind (virtual +0x18), "cannot
        // be used" (+0x94) and "has effects" (+0xec) from fields.
        let mut slots = vec![0u32; 0x200];
        for (i, slot) in slots.iter_mut().enumerate() {
            *slot = slot_fn(FORM_VT, 4 * i as u32);
        }
        e.put_vtable(FORM_VT, &slots);
        for vtable in [
            ACTOR_VT, CASTER_VT, TARGET_VT, PROCESS_VT, VALUE_VT, OTHER_VT,
        ] {
            let mut slots = vec![0u32; 0x400];
            for (i, slot) in slots.iter_mut().enumerate() {
                *slot = slot_fn(vtable, 4 * i as u32);
            }
            e.put_vtable(vtable, &slots);
            for i in 0..0x400u32 {
                e.register(slot_fn(vtable, 4 * i), |_, _| Ret::default());
            }
        }
        for i in 0..0x200u32 {
            e.register(slot_fn(FORM_VT, 4 * i), |_, _| Ret::default());
        }
        e.register(slot_fn(FORM_VT, 0x18), |e, a| {
            e.mem.u32(a[0] + 0x70).into_ret()
        });
        e.register(slot_fn(FORM_VT, 0x94), |e, a| {
            e.mem.u32(a[0] + 0x74).into_ret()
        });
        e.register(slot_fn(FORM_VT, 0xec), |e, a| {
            e.mem.u32(a[0] + 0x78).into_ret()
        });
        // The actor value owner at actor + 0xa4 reads actor value `a[1]` as
        // a float from actor + 0x200 + 4 * index.
        e.register(slot_fn(VALUE_VT, 0xc), |e, a| {
            e.mem.f32(a[0] - 0xa4 + 0x200 + 4 * a[1]).into_ret()
        });
        e
    }

    /// An actor with the three bases, a process and base data object.
    struct World {
        actor: u32,
        caster: u32,
        target: u32,
        process: u32,
        base: u32,
    }

    fn setup() -> (Engine, World) {
        let mut e = engine();
        let actor = e.mem.alloc(0x400);
        let process = e.mem.alloc(0x400);
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(actor, ACTOR_VT);
        e.mem.set_u32(actor + 0x88, CASTER_VT);
        e.mem.set_u32(actor + 0x94, TARGET_VT);
        e.mem.set_u32(actor + 0xa4, VALUE_VT);
        e.mem.set_u32(actor + 0x68, process);
        e.mem.set_u32(actor + 0x20, base);
        e.mem.set_u32(process, PROCESS_VT);
        e.mem.set_u32(base, OTHER_VT);
        let world = World {
            actor,
            caster: actor + 0x88,
            target: actor + 0x94,
            process,
            base,
        };
        (e, world)
    }

    /// A form-like object (kind at `+0x70`, "cannot use" at `+0x74`, "has
    /// effects" at `+0x78`).
    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, FORM_VT);
        object
    }

    /// An effect or spell: the sub-object at `+0x18` answers `kind`.
    fn magic_object(e: &mut Engine, kind: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x18, FORM_VT);
        e.mem.set_u32(object + 0x18 + 0x70, kind);
        object
    }

    /// A list node with data `value` and next pointer `next`.
    fn node(e: &mut Engine, value: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, value);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// Fills the list header at `list` (its first node is inline at +4).
    fn fill_list(e: &mut Engine, list: u32, values: &[u32]) {
        let mut next = 0;
        for &value in values.iter().skip(1).rev() {
            next = node(e, value, next);
        }
        e.mem.set_u32(list + 4, values[0]);
        e.mem.set_u32(list + 8, next);
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(logged, _)| *logged == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn caster_slot(slot: u32) -> u32 {
        slot_fn(CASTER_VT, slot)
    }

    fn process_slot(slot: u32) -> u32 {
        slot_fn(PROCESS_VT, slot)
    }

    fn actor_slot(slot: u32) -> u32 {
        slot_fn(ACTOR_VT, slot)
    }

    #[test]
    fn every_function_is_registered() {
        let e = Engine::new();
        let table = funcs();
        assert_eq!(table.len(), 53);
        for (address, _) in table {
            assert!(e.is_translated(address), "{address:08x}");
        }
    }

    #[test]
    fn worn_item_is_passed_to_the_caster_helper() {
        let (mut e, w) = setup();
        let worn = e.mem.alloc(0x20);
        let first = e.mem.alloc(8);
        e.mem.set_u32(worn, first);
        e.mem.set_u32(first, 0x1234);
        e.mem.set_u32(worn + 8, 0x5678);
        e.register(FN_0080F7D0, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(fn_008c1cd0(&mut e, Ptr::new(w.actor), Ptr::new(worn)));
        assert_eq!(calls(&e, FN_0080F7D0), vec![vec![w.actor, 0x5678, 0x1234]]);
        // Without a first word the second value is 0.
        e.mem.set_u32(worn, 0);
        start_log(&mut e);
        fn_008c1cd0(&mut e, Ptr::new(w.actor), Ptr::new(worn));
        assert_eq!(calls(&e, FN_0080F7D0), vec![vec![w.actor, 0x5678, 0]]);
        assert_eq!(calls(&e, FIRST_WORD).len(), 1);
    }

    #[test]
    fn eat_uses_the_item_or_queues_it_for_the_player() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let item = object(&mut e);
        let extra = 0x4444u32;
        start_log(&mut e);
        actor_eat(&mut e, actor, Ptr::NULL, Ptr::NULL, false);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        // An item that can not be used: only the player is told.
        e.mem.set_u32(item + 0x74, 1);
        e.set_global(MESSAGE_SCALE_010162C0, 0.5f32);
        e.mem.set_u32(CANNOT_USE_MESSAGE + 4, 0x7777);
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), false);
        assert!(calls(&e, FN_007052F0).is_empty());
        e.set_global(PLAYER_CHARACTER, w.actor);
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), false);
        assert_eq!(
            calls(&e, FN_007052F0),
            vec![vec![0x7777, 0, SAD_ICON_PATH, 0, 0.5f32.to_bits(), 0]]
        );
        assert!(calls(&e, SET_ACTION_FLAG).is_empty());
        // Another actor uses the item at once.
        e.mem.set_u32(item + 0x74, 0);
        e.mem.set_u32(item + 0x78, 1);
        e.set_global(PLAYER_CHARACTER, 0);
        start_log(&mut e);
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), false);
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![w.actor, extra, 2], vec![item, w.actor + 0x44, 2]]
        );
        assert_eq!(
            calls(&e, caster_slot(0x40)),
            vec![vec![w.caster, item + 0x30]]
        );
        assert_eq!(calls(&e, RELEASE_CAST), vec![vec![w.caster, 0]]);
        assert_eq!(
            calls(&e, actor_slot(0x17c)),
            vec![vec![w.actor, item, extra, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        // The player queues the item and eats it only when it is accepted.
        e.set_global(PLAYER_CHARACTER, w.actor);
        start_log(&mut e);
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), false);
        assert_eq!(
            calls(&e, ADD_EAT_DRINK_ITEM),
            vec![vec![w.actor, item + 0x30]]
        );
        assert!(calls(&e, actor_slot(0x17c)).is_empty());
        assert!(calls(&e, caster_slot(0x40)).is_empty());
        e.register(ADD_EAT_DRINK_ITEM, |_, _| 1u32.into_ret());
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), false);
        assert_eq!(calls(&e, actor_slot(0x17c)).len(), 1);
        // An item without effects hands 0 to the caster.
        e.mem.set_u32(item + 0x78, 0);
        start_log(&mut e);
        actor_eat(&mut e, actor, Ptr::new(item), Ptr::new(extra), true);
        assert_eq!(calls(&e, caster_slot(0x40)), vec![vec![w.caster, 0]]);
        assert!(calls(&e, ADD_EAT_DRINK_ITEM).is_empty());
    }

    #[test]
    fn drink_potion_reports_whether_it_drank() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let potion = object(&mut e);
        assert!(!actor_drink_potion(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::NULL,
            false
        ));
        start_log(&mut e);
        assert!(actor_drink_potion(
            &mut e,
            actor,
            Ptr::new(potion),
            Ptr::new(9),
            false
        ));
        assert_eq!(
            calls(&e, caster_slot(0x40)),
            vec![vec![w.caster, potion + 0x30]]
        );
        assert_eq!(calls(&e, RELEASE_CAST), vec![vec![w.caster, 0]]);
        assert_eq!(
            calls(&e, actor_slot(0x17c)),
            vec![vec![w.actor, potion, 9, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        // The player only queues the potion; the answer is the queue's.
        e.set_global(PLAYER_CHARACTER, w.actor);
        start_log(&mut e);
        assert!(!actor_drink_potion(
            &mut e,
            actor,
            Ptr::new(potion),
            Ptr::new(9),
            false
        ));
        assert!(calls(&e, actor_slot(0x17c)).is_empty());
        assert!(calls(&e, caster_slot(0x40)).is_empty());
        e.register(ADD_EAT_DRINK_ITEM, |_, _| 1u32.into_ret());
        assert!(actor_drink_potion(
            &mut e,
            actor,
            Ptr::new(potion),
            Ptr::new(9),
            false
        ));
        assert_eq!(calls(&e, actor_slot(0x17c)).len(), 1);
        // `consume_now` skips the queue and, for the player, the finish.
        start_log(&mut e);
        assert!(actor_drink_potion(
            &mut e,
            actor,
            Ptr::new(potion),
            Ptr::new(9),
            true
        ));
        assert_eq!(calls(&e, caster_slot(0x40)).len(), 1);
        assert!(calls(&e, ADD_EAT_DRINK_ITEM).is_empty());
        assert!(calls(&e, actor_slot(0x17c)).is_empty());
    }

    #[test]
    fn cast_alchemy_dispatches_on_the_form_kind() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let item = object(&mut e);
        let potion = object(&mut e);
        let bound = object(&mut e);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (match a[3] {
                TYPE_ALCHEMY_ITEM => potion,
                TYPE_BOUND_OBJECT => bound,
                _ => a[0],
            })
            .into_ret()
        });
        // Other kinds do nothing; the kind is asked twice.
        e.mem.set_u32(item + 0x70, 5);
        start_log(&mut e);
        actor_cast_alchemy(&mut e, actor, Ptr::new(item), true);
        assert_eq!(calls(&e, slot_fn(FORM_VT, 0x18)).len(), 2);
        assert!(calls(&e, caster_slot(0x40)).is_empty());
        // Kind 7 with `consume` is drunk, kind 8 eaten.
        e.mem.set_u32(item + 0x70, 7);
        start_log(&mut e);
        actor_cast_alchemy(&mut e, actor, Ptr::new(item), true);
        assert_eq!(
            calls(&e, caster_slot(0x40)),
            vec![vec![w.caster, potion + 0x30]]
        );
        e.mem.set_u32(item + 0x70, 8);
        start_log(&mut e);
        actor_cast_alchemy(&mut e, actor, Ptr::new(item), true);
        assert_eq!(calls(&e, caster_slot(0x40)), vec![vec![w.caster, 0]]);
        assert_eq!(
            calls(&e, actor_slot(0x17c)),
            vec![vec![w.actor, bound, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        // Without `consume` the item goes to the caster as it is.
        start_log(&mut e);
        actor_cast_alchemy(&mut e, actor, Ptr::new(item), false);
        assert_eq!(calls(&e, caster_slot(0x40)), vec![vec![w.caster, item]]);
        assert_eq!(calls(&e, RELEASE_CAST), vec![vec![w.caster, 0]]);
        assert_eq!(
            calls(&e, actor_slot(0x17c)),
            vec![vec![w.actor, bound, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn fn_008c2190_casts_spell_effect_source_and_poison() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let out = e.mem.alloc(8);
        let target = e.mem.alloc(0x100);
        // A source answering 3 to its virtual +0x304 stops everything.
        let source = e.mem.alloc(0x100);
        e.mem.set_u32(source, OTHER_VT);
        e.register(slot_fn(OTHER_VT, 0x304), |_, _| 3u32.into_ret());
        e.mem.set_u8(out, 0xff);
        assert!(!fn_008c2190(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::new(target),
            Ptr::new(source),
            Ptr::new(out)
        ));
        assert_eq!(e.mem.u8(out), 0);
        e.register(slot_fn(OTHER_VT, 0x304), |_, _| Ret::default());
        // Nothing to cast: no spell on the base data, no item, no source.
        assert!(!fn_008c2190(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::new(target),
            Ptr::NULL,
            Ptr::new(out)
        ));
        // The base data's spell is cast when the fatigue is 0xff.
        let spell_node = e.mem.alloc(0x40);
        e.mem.set_u32(w.base + 0x74, spell_node);
        e.mem.set_u16(w.base + 0x78, 0xff);
        start_log(&mut e);
        assert!(fn_008c2190(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::new(target),
            Ptr::NULL,
            Ptr::new(out)
        ));
        assert_eq!(
            calls(&e, caster_slot(0x40)),
            vec![vec![w.caster, spell_node + 0x18]]
        );
        assert_eq!(
            calls(&e, caster_slot(0x48)),
            vec![vec![w.caster, target + 0x94]]
        );
        assert_eq!(calls(&e, RELEASE_CAST), vec![vec![w.caster, w.base]]);
        // A different fatigue needs the saved acquire object's animation
        // group to match.
        e.mem.set_u16(w.base + 0x78, 5);
        start_log(&mut e);
        assert!(fn_008c2190(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::new(target),
            Ptr::NULL,
            Ptr::new(out)
        ));
        assert!(calls(&e, caster_slot(0x40)).is_empty());
        e.register(process_slot(0x3e8), |_, _| 0x6000u32.into_ret());
        e.register(FN_0048F7F0, |_, _| 0x6100u32.into_ret());
        e.register(FN_005F2420, |_, _| 5u32.into_ret());
        start_log(&mut e);
        fn_008c2190(
            &mut e,
            actor,
            Ptr::NULL,
            Ptr::new(target),
            Ptr::NULL,
            Ptr::new(out),
        );
        assert_eq!(calls(&e, caster_slot(0x40)).len(), 1);
        // An item change of type 0x28 gives the effect source and a poison.
        let change = e.mem.alloc(0x40);
        let form = e.mem.alloc(0x100);
        let effect_source = e.mem.alloc(0x100);
        e.mem.set_u32(change + 8, form);
        e.mem.set_u8(form + 4, 0x28);
        e.mem.set_u32(form + 0x78, effect_source);
        e.register(FN_004BDCC0, |_, _| 0x6200u32.into_ret());
        start_log(&mut e);
        assert!(fn_008c2190(
            &mut e,
            actor,
            Ptr::new(change),
            Ptr::new(target),
            Ptr::NULL,
            Ptr::new(out)
        ));
        assert_eq!(
            calls(&e, caster_slot(0x40)),
            vec![
                vec![w.caster, effect_source + 0x18],
                vec![w.caster, 0x6200 + 0x30]
            ]
        );
        assert_eq!(
            calls(&e, RELEASE_CAST),
            vec![vec![w.caster, form], vec![w.caster, form]]
        );
        assert_eq!(calls(&e, FN_004BDD80), vec![vec![change]]);
    }

    #[test]
    fn fn_008c2630_casts_the_enchantment_of_weapons_and_armour() {
        let (mut e, w) = setup();
        let form = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x100);
        let enchantment = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 4, enchantment);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[3] == TYPE_ENCHANTABLE_FORM {
                holder
            } else {
                0
            })
            .into_ret()
        });
        e.mem.set_u8(form + 4, 0x18);
        start_log(&mut e);
        fn_008c2630(&mut e, Ptr::new(w.actor), Ptr::new(form), 0);
        assert_eq!(
            calls(&e, caster_slot(0x14)),
            vec![vec![w.caster, enchantment + 0x18, form, 0]]
        );
        e.mem.set_u8(form + 4, 0x1a);
        start_log(&mut e);
        fn_008c2630(&mut e, Ptr::new(w.actor), Ptr::new(form), 0);
        assert_eq!(calls(&e, caster_slot(0x14)).len(), 1);
        e.mem.set_u8(form + 4, 0x19);
        start_log(&mut e);
        fn_008c2630(&mut e, Ptr::new(w.actor), Ptr::new(form), 0);
        assert!(calls(&e, RT_DYNAMIC_CAST).is_empty());
        // No enchantment behind the cast: nothing.
        e.mem.set_u8(form + 4, 0x18);
        e.mem.set_u32(holder + 4, 0);
        fn_008c2630(&mut e, Ptr::new(w.actor), Ptr::new(form), 0);
        assert!(calls(&e, caster_slot(0x14)).is_empty());
    }

    #[test]
    fn cast_permanent_magic_walks_the_lists_or_the_worn_items() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let (entry_one, entry_two, entry_three) = (0x7001u32, 0x7002u32, 0x7003u32);
        fill_list(&mut e, w.base + 0x7c, &[entry_one, entry_two]);
        let owner = e.mem.alloc(0x100);
        fill_list(&mut e, owner + 0x2c, &[entry_three]);
        e.register_double(actor_slot(0x37c), move |_, _| owner.into_ret());
        start_log(&mut e);
        actor_cast_permanent_magic(&mut e, actor, false);
        assert_eq!(
            calls(&e, caster_slot(0x00)),
            vec![
                vec![w.caster, entry_one, 1],
                vec![w.caster, entry_two, 1],
                vec![w.caster, entry_three, 1]
            ]
        );
        assert_eq!(
            calls(&e, caster_slot(0x10)),
            vec![
                vec![w.caster, entry_one, w.target, 1],
                vec![w.caster, entry_two, w.target, 1],
                vec![w.caster, entry_three, w.target, 1]
            ]
        );
        // The default object, when it is a spell the actor does not carry,
        // is cast as well.
        let default_object = e.mem.alloc(0x40);
        let spell = e.mem.alloc(0x100);
        e.register_double(FN_0058DB10, move |_, _| default_object.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[3] == TYPE_SPELL_ITEM { spell } else { a[0] }).into_ret()
        });
        start_log(&mut e);
        actor_cast_permanent_magic(&mut e, actor, false);
        assert_eq!(
            calls(&e, FN_00822B90),
            vec![vec![w.target, spell + 0x18, 1]]
        );
        assert_eq!(calls(&e, caster_slot(0x00)).len(), 4);
        e.register(FN_00822B90, |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_cast_permanent_magic(&mut e, actor, false);
        assert_eq!(calls(&e, caster_slot(0x00)).len(), 3);
    }

    #[test]
    fn cast_permanent_magic_gives_each_worn_enchantment_once() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let form = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x100);
        let enchantment = e.mem.alloc(0x100);
        let first = e.mem.alloc(8);
        e.mem.set_u32(holder + 4, enchantment);
        let worn_one = e.mem.alloc(0x100);
        let worn_two = e.mem.alloc(0x100);
        for worn in [worn_one, worn_two] {
            e.mem.set_u32(worn, first);
            e.mem.set_u32(worn + 8, form);
        }
        e.register(FN_004BF220, |_, _| 0x5555u32.into_ret());
        e.register_double(FN_004C8C10, move |_, a| {
            (match a[1] {
                0 => worn_one,
                1 => worn_two,
                _ => 0,
            })
            .into_ret()
        });
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[3] == TYPE_ENCHANTABLE_FORM {
                holder
            } else {
                0
            })
            .into_ret()
        });
        e.register(FN_0080F7D0, |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_cast_permanent_magic(&mut e, actor, true);
        // Both slots hold the same form: only the first is cast.
        assert_eq!(
            calls(&e, caster_slot(0x14)),
            vec![vec![w.caster, enchantment + 0x18, form, 1]]
        );
        assert_eq!(calls(&e, FN_004BF220), vec![vec![w.actor]]);
        assert_eq!(
            calls(&e, FN_004459E0),
            vec![vec![worn_one, 1], vec![worn_two, 1]]
        );
        assert_eq!(calls(&e, FN_0080F7D0).len(), 1);
        // The helper can refuse an item.
        e.register(FN_0080F7D0, |_, _| Ret::default());
        start_log(&mut e);
        actor_cast_permanent_magic(&mut e, actor, true);
        assert!(calls(&e, caster_slot(0x14)).is_empty());
    }

    /// An actor whose permanent magic has effects in both lists, a worn
    /// enchantment in the first slot and a default spell.
    struct Magic {
        w: World,
        accepted: u32,
        flagged: u32,
        owner_effect: u32,
        enchantment: u32,
        form: u32,
        worn: u32,
        spell: u32,
    }

    fn magic_world() -> (Engine, Magic) {
        let (mut e, w) = setup();
        let accepted = magic_object(&mut e, 1);
        let flagged = magic_object(&mut e, 4);
        e.mem.set_u8(flagged + 0x40, 4);
        let ignored = magic_object(&mut e, 2);
        let owner_effect = magic_object(&mut e, 10);
        e.mem.set_u8(owner_effect + 0x40, 4);
        fill_list(&mut e, w.base + 0x7c, &[accepted, flagged, ignored]);
        let owner = e.mem.alloc(0x100);
        fill_list(&mut e, owner + 0x2c, &[owner_effect]);
        e.register_double(actor_slot(0x37c), move |_, _| owner.into_ret());
        for index in 0..5u32 {
            e.set_global(WORN_SLOT_TABLE + 4 * index, 100 + index);
        }
        let form = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x100);
        let enchantment = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 4, enchantment);
        let worn = e.mem.alloc(0x100);
        e.mem.set_u32(worn + 8, form);
        e.register(FN_004BF220, |_, _| 0x5555u32.into_ret());
        e.register_double(FN_004C8C10, move |_, a| {
            (if a[1] == 100 { worn } else { 0 }).into_ret()
        });
        let spell = magic_object(&mut e, 1);
        let default_object = e.mem.alloc(0x40);
        e.register_double(FN_0058DB10, move |_, _| default_object.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (match a[3] {
                TYPE_ENCHANTABLE_FORM => holder,
                TYPE_SPELL_ITEM => spell,
                _ => a[0],
            })
            .into_ret()
        });
        let magic = Magic {
            w,
            accepted,
            flagged,
            owner_effect,
            enchantment,
            form,
            worn,
            spell,
        };
        (e, magic)
    }

    #[test]
    fn fn_008c2b60_dispels_the_permanent_magic() {
        let (mut e, m) = magic_world();
        start_log(&mut e);
        fn_008c2b60(&mut e, Ptr::new(m.w.actor));
        let target = m.w.target;
        // The flagged effect of the first list is skipped, the one of the
        // second list is not.
        assert_eq!(
            calls(&e, FN_00824400),
            vec![
                vec![target, m.accepted + 0x18, 0, 0],
                vec![target, m.owner_effect + 0x18, 0, 0],
                vec![target, m.spell + 0x18, 0, 0]
            ]
        );
        assert_eq!(calls(&e, FN_008248E0), vec![vec![target, m.form, 0]]);
        assert_eq!(calls(&e, FN_004459E0), vec![vec![m.worn, 1]]);
        assert_eq!(calls(&e, FN_004BF220), vec![vec![m.w.actor]]);
        assert!(!calls(&e, FN_00824400).contains(&vec![target, m.flagged + 0x18, 0, 0]));
    }

    #[test]
    fn fn_008c2e30_tests_bit_four_of_the_flag_byte() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        assert!(!fn_008c2e30(&mut e, Ptr::new(object)));
        e.mem.set_u8(object + 0x40, 4);
        assert!(fn_008c2e30(&mut e, Ptr::new(object)));
        assert_eq!(e.call(0x008c2e30, &args![object]).u32() & 0xff, 1);
    }

    #[test]
    fn fn_008c2e50_preloads_the_permanent_magic() {
        let (mut e, m) = magic_world();
        start_log(&mut e);
        fn_008c2e50(&mut e, Ptr::new(m.w.actor));
        assert_eq!(
            calls(&e, FN_0040A420),
            vec![
                vec![m.accepted + 0x18, 0],
                vec![m.flagged + 0x18, 0],
                vec![m.owner_effect + 0x18, 0],
                vec![m.enchantment + 0x18, 0],
                vec![m.spell + 0x18, 0]
            ]
        );
        assert_eq!(calls(&e, FN_004459E0), vec![vec![m.worn, 1]]);
    }

    #[test]
    fn fn_008c30a0_unloads_the_permanent_magic() {
        let (mut e, m) = magic_world();
        start_log(&mut e);
        fn_008c30a0(&mut e, Ptr::new(m.w.actor));
        assert_eq!(
            calls(&e, FN_0040B800),
            vec![
                vec![m.accepted + 0x18, 1],
                vec![m.flagged + 0x18, 1],
                vec![m.owner_effect + 0x18, 1],
                vec![m.enchantment + 0x18, 1],
                vec![m.spell + 0x18, 0]
            ]
        );
    }

    #[test]
    fn add_spell_adds_to_the_list_and_casts_it() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let spell = magic_object(&mut e, 1);
        e.register(FN_0048D170, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(actor_add_spell(&mut e, actor, Ptr::new(spell)));
        assert_eq!(calls(&e, FN_0048D170), vec![vec![w.base + 0x7c, spell]]);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0x48)), vec![vec![w.base, 0x10]]);
        assert_eq!(
            calls(&e, FN_00822B90),
            vec![vec![w.target, spell + 0x18, 1]]
        );
        assert_eq!(calls(&e, caster_slot(0x00)), vec![vec![w.caster, spell, 0]]);
        assert_eq!(
            calls(&e, caster_slot(0x10)),
            vec![vec![w.caster, spell, w.target, 0]]
        );
        // Already in the list and already a target: nothing is cast.
        e.register(FN_0048D170, |_, _| Ret::default());
        e.register(FN_00822B90, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(!actor_add_spell(&mut e, actor, Ptr::new(spell)));
        assert!(calls(&e, caster_slot(0x00)).is_empty());
        assert!(calls(&e, slot_fn(OTHER_VT, 0x48)).is_empty());
        // A kind `00804b30` rejects is only added to the list.
        e.mem.set_u32(spell + 0x18 + 0x70, 2);
        e.register(FN_0048D170, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(actor_add_spell(&mut e, actor, Ptr::new(spell)));
        assert!(calls(&e, FN_00822B90).is_empty());
    }

    #[test]
    fn remove_spell_takes_it_out_of_the_list_and_the_target() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let spell = magic_object(&mut e, 1);
        start_log(&mut e);
        assert!(!actor_remove_spell(&mut e, actor, Ptr::new(spell)));
        e.register(FN_0048D830, |_, _| 1u32.into_ret());
        e.register(FN_00822B90, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(actor_remove_spell(&mut e, actor, Ptr::new(spell)));
        assert_eq!(calls(&e, FN_0048D8D0), vec![vec![w.base + 0x7c, spell]]);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0x48)), vec![vec![w.base, 0x10]]);
        assert_eq!(
            calls(&e, FN_00824400),
            vec![vec![w.target, spell + 0x18, 0, 0]]
        );
        // A kind-2 form is also looked up in the list at +0xf4.
        e.mem.set_u32(spell + 0x18 + 0x70, 2);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, spell);
        e.mem.set_u32(w.actor + 0xf4, entry);
        start_log(&mut e);
        assert!(actor_remove_spell(&mut e, actor, Ptr::new(spell)));
        assert_eq!(calls(&e, FN_00905330).len(), 1);
        assert_eq!(calls(&e, FN_00401030), vec![vec![entry]]);
    }

    #[test]
    fn fn_008c3530_shows_a_refused_cast_to_the_player() {
        let (mut e, w) = setup();
        let item = e.mem.alloc(0x100);
        e.register_double(caster_slot(0x34), move |_, _| item.into_ret());
        e.register(caster_slot(0x28), |e, a| {
            e.mem.set_u32(a[3], 9);
            Ret::default()
        });
        e.register(FN_004065E0, |_, _| 0x11u32.into_ret());
        e.register(FN_0040CA30, |_, _| 0x22u32.into_ret());
        e.register(FN_0040BEF0, |e, a| {
            e.mem.set_u32(a[1], 0xcafe);
            a[1].into_ret()
        });
        e.set_global(MESSAGE_SCALE_010162C0, 0.25f32);
        // Another actor: nothing is shown.
        start_log(&mut e);
        fn_008c3530(&mut e, Ptr::new(w.caster));
        assert!(calls(&e, FN_007052F0).is_empty());
        assert_eq!(calls(&e, caster_slot(0x28)).len(), 1);
        // The player is shown a message built from the item and the kind
        // the check wrote.
        e.set_global(PLAYER_CHARACTER, w.actor);
        start_log(&mut e);
        fn_008c3530(&mut e, Ptr::new(w.caster));
        assert_eq!(calls(&e, FN_004065E0), vec![vec![item + 0xc]]);
        assert_eq!(calls(&e, FN_0040CA30), vec![vec![0x11]]);
        assert_eq!(calls(&e, FN_00962030), vec![vec![w.actor, 0x22]]);
        let text = calls(&e, FN_004037D0)[0][0];
        assert_eq!(calls(&e, FN_0040BEF0), vec![vec![item, text, 9]]);
        assert_eq!(
            calls(&e, FN_007052F0),
            vec![vec![0xcafe, 0, 0, 0, 0.25f32.to_bits(), 0]]
        );
        // A caster that can go on makes the process do its thing instead.
        e.register(caster_slot(0x28), |_, _| 1u32.into_ret());
        start_log(&mut e);
        fn_008c3530(&mut e, Ptr::new(w.caster));
        assert!(calls(&e, FN_007052F0).is_empty());
        assert_eq!(calls(&e, process_slot(0x3dc)), vec![vec![w.process, 1]]);
    }

    #[test]
    fn fn_008c3650_follows_the_actor_to_its_casting_object() {
        let (mut e, w) = setup();
        let caster = Ptr::new(w.caster);
        let third = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(third, OTHER_VT);
        e.mem.set_u32(second, OTHER_VT);
        // The actor's virtual +0x1d0 answering 0 ends it.
        assert_eq!(fn_008c3650(&mut e, caster), 0);
        e.register(actor_slot(0x1d0), |_, _| 1u32.into_ret());
        e.register(actor_slot(0x1e4), |_, _| 0x7100u32.into_ret());
        e.register(FN_00496940, |_, _| 0x7200u32.into_ret());
        e.register_double(FN_00537BD0, move |_, _| second.into_ret());
        e.register_double(slot_fn(OTHER_VT, 0x8c), move |_, _| third.into_ret());
        e.register(slot_fn(OTHER_VT, 0xc), |_, _| 0x42u32.into_ret());
        e.register(FN_00499B70, |_, _| 0x77u32.into_ret());
        start_log(&mut e);
        assert_eq!(fn_008c3650(&mut e, caster), 0x42);
        assert_eq!(calls(&e, FN_00496940), vec![vec![0x7100]]);
        assert_eq!(calls(&e, FN_00537BD0), vec![vec![0x7200]]);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0x8c)), vec![vec![second, 0x77]]);
        // The player starts from its own object when it has one.
        e.set_global(PLAYER_CHARACTER, w.actor);
        e.register(FN_00950BB0, |_, _| 0x7300u32.into_ret());
        e.register(FN_00456610, |_, _| 1u32.into_ret());
        e.register(FN_00950A60, |_, _| 0x7400u32.into_ret());
        start_log(&mut e);
        assert_eq!(fn_008c3650(&mut e, caster), 0x42);
        assert_eq!(calls(&e, FN_00950A60), vec![vec![w.actor, 1]]);
        assert_eq!(calls(&e, FN_00496940), vec![vec![0x7400]]);
        // A broken chain gives 0.
        e.register(FN_00496940, |_, _| Ret::default());
        assert_eq!(fn_008c3650(&mut e, caster), 0);
    }

    #[test]
    fn fn_008c3790_steps_the_casting_object() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let item = e.mem.alloc(0x100);
        e.mem.set_u32(item, OTHER_VT);
        // Without an item the process is told 0.
        start_log(&mut e);
        fn_008c3790(&mut e, actor);
        assert_eq!(calls(&e, process_slot(0x3dc)), vec![vec![w.process, 0]]);
        e.register_double(caster_slot(0x30), move |_, _| item.into_ret());
        e.register(actor_slot(0x1e4), |_, _| 0x7500u32.into_ret());
        e.register(FN_00491040, |_, _| 0x7600u32.into_ret());
        e.register(FN_008041A0, |_, _| 1u32.into_ret());
        // With a target and a ready process whose item has no entries,
        // fn_008c39b0 runs.
        e.register(caster_slot(0x34), |_, _| 0x7700u32.into_ret());
        start_log(&mut e);
        fn_008c3790(&mut e, actor);
        assert_eq!(calls(&e, FN_00491040), vec![vec![0x7500, 2]]);
        assert_eq!(calls(&e, FN_0040A300), vec![vec![0x7700, 0]]);
        e.register(FN_008041A0, |_, _| Ret::default());
        start_log(&mut e);
        fn_008c3790(&mut e, actor);
        assert!(calls(&e, FN_0040A300).is_empty());
        // Without a target, the entries whose first float is below the second
        // are told, by index.
        e.register(caster_slot(0x34), |_, _| Ret::default());
        let entries: Vec<u32> = (0..3)
            .map(|index| {
                let entry = e.mem.alloc(0x100);
                let (current, limit) = match index {
                    0 => (1.0f32, 2.0f32),
                    1 => (3.0, 2.0),
                    _ => (f32::NAN, 2.0),
                };
                e.mem.set_f32(entry + 0x30, current);
                e.mem.set_f32(entry + 0x34, limit);
                entry
            })
            .collect();
        e.register(FN_00453470, |_, _| 3u32.into_ret());
        e.register(FN_0043B480, |_, _| 3u32.into_ret());
        let table = entries.clone();
        e.register_double(FN_0043B4A0, move |_, a| table[a[1] as usize].into_ret());
        e.register(FN_0043B230, |_, a| a[0].into_ret());
        e.register(FN_00653270, |_, a| a[1].into_ret());
        e.register(FN_0047A520, |_, a| a[0].into_ret());
        start_log(&mut e);
        fn_008c3790(&mut e, actor);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0xf0)), vec![vec![item, 0]]);
        assert_eq!(calls(&e, FN_004DEF90), vec![vec![item]]);
        assert_eq!(
            calls(&e, FN_00653270)[0],
            vec![TYPE_DATA_011F36AC, entries[0]]
        );
        assert!(calls(&e, process_slot(0x3dc)).is_empty());
        // Ready: the item is not prepared. An emptied item tells the process.
        e.register(FN_008041A0, |_, _| 1u32.into_ret());
        e.register(FN_00453470, |_, _| Ret::default());
        start_log(&mut e);
        fn_008c3790(&mut e, actor);
        assert!(calls(&e, FN_004DEF90).is_empty());
        assert_eq!(calls(&e, process_slot(0x3dc)), vec![vec![w.process, 0]]);
    }

    #[test]
    fn fn_008c39b0_replaces_the_light_and_drives_it() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let item = e.mem.alloc(0x100);
        e.mem.set_u32(item, OTHER_VT);
        let checked = e.mem.alloc(0x100);
        let field = e.mem.alloc(0x20);
        e.mem.set_u32(checked + 0x70, field);
        e.mem.set_u32(field + 0xc, 0x99);
        let object = e.mem.alloc(0x100);
        let new_light = e.mem.alloc(0x40);
        let part = e.mem.alloc(0x40);
        let effect = e.mem.alloc(0x100);
        e.mem.set_f32(effect + 0x30, 4.0);
        let old_light = e.mem.alloc(0x40);
        e.mem.set_u32(w.actor + 0x8c, old_light);
        e.set_global(FADE_01085350, 2.0f32);
        e.set_global(LIGHT_SCALE_0101DE30, 0.5f64);
        e.register_double(FN_0040A300, move |_, _| checked.into_ret());
        e.register(FN_0048CEE0, |_, _| 1u32.into_ret());
        e.register_double(FN_0081E440, move |_, _| object.into_ret());
        e.register_double(caster_slot(0x30), move |_, _| item.into_ret());
        e.register(FN_00401000, |_, _| 0x8100u32.into_ret());
        e.register_double(FN_0081BF80, move |_, _| new_light.into_ret());
        e.register(FN_006ECD40, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(FN_0043B230, |_, _| 0x8200u32.into_ret());
        e.register_double(FN_00653270, move |_, _| part.into_ret());
        e.register_double(FN_0047A520, move |_, _| effect.into_ret());
        start_log(&mut e);
        fn_008c39b0(&mut e, actor, Ptr::new(0x8000));
        assert_eq!(calls(&e, FN_0040A300), vec![vec![0x8000, 0]]);
        assert_eq!(calls(&e, FN_0081E440), vec![vec![checked]]);
        assert_eq!(calls(&e, FN_004DEF90), vec![vec![item]]);
        assert_eq!(
            calls(&e, slot_fn(OTHER_VT, 0xdc)),
            vec![vec![item, object, 1]]
        );
        assert_eq!(calls(&e, FN_0080FCC0), vec![vec![old_light, 1]]);
        assert_eq!(calls(&e, FN_00401000), vec![vec![0x1c]]);
        assert_eq!(calls(&e, FN_0081BF80), vec![vec![0x8100, 0x99, item]]);
        assert_eq!(calls(&e, FN_006ECD40), vec![vec![w.caster, new_light]]);
        assert_eq!(calls(&e, FN_0048FEF0), vec![vec![part, 0.0f32.to_bits()]]);
        assert_eq!(
            calls(&e, FN_0047AAB0),
            vec![vec![
                part,
                effect,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(calls(&e, FN_0047AA40), vec![vec![part, 1]]);
        assert_eq!(
            calls(&e, FN_0098ADB0),
            vec![vec![effect, (-2.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&e, FN_0081C470),
            vec![vec![new_light, 1, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, FN_0081C320),
            vec![vec![new_light, 0.0f32.to_bits()]]
        );
        // No target: nothing is done.
        start_log(&mut e);
        fn_008c39b0(&mut e, actor, Ptr::NULL);
        assert!(calls(&e, FN_004DEF90).is_empty());
        assert!(calls(&e, FN_0081C320).is_empty());
    }

    #[test]
    fn update_magic_measures_the_step_and_updates_the_parts() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        // An actor the update skips.
        e.register(FN_00576D30, |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, true, false);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.register(FN_00576D30, |_, _| Ret::default());
        // Game time: 3600 over the time scale.
        e.set_global(SECONDS_PER_HOUR_01012640, 3600.0f64);
        e.register(FN_0096D490, |_, _| 10.0f32.into_ret());
        e.register(CALENDAR_GET_TIME_SCALE, |_, _| 2.0f32.into_ret());
        e.register(actor_slot(0x360), |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, true, false);
        let delta = 1800.0f32.to_bits();
        assert_eq!(
            calls(&e, process_slot(0x700)),
            vec![vec![w.process, w.actor, delta]]
        );
        assert_eq!(calls(&e, FN_00823C40), vec![vec![w.target, delta]]);
        assert_eq!(calls(&e, FN_0088B660), vec![vec![w.actor, delta]]);
        assert!(calls(&e, FN_0088B510).is_empty());
        assert_eq!(calls(&e, actor_slot(0x3dc)), vec![vec![w.actor, delta]]);
        assert_eq!(e.mem.f32(w.actor + 0x114), 10.0);
        let scope = calls(&e, FN_00404EB0)[0][0];
        assert_eq!(
            calls(&e, FN_00404EB0),
            vec![vec![scope, 0x32, 1, SCOPE_NAME_01085384, 0x4e5]]
        );
        assert_eq!(calls(&e, FN_00404EE0), vec![vec![scope]]);
        // The fixed step, and the process flags.
        e.set_global(FIXED_STEP_01017D00, 0.5f32);
        e.register(FN_008ACED0, |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, true);
        let delta = 0.5f32.to_bits();
        assert_eq!(
            calls(&e, process_slot(0x700)),
            vec![vec![w.process, w.actor, delta]]
        );
        assert_eq!(calls(&e, FN_0088B510), vec![vec![w.actor, delta]]);
        assert_eq!(calls(&e, FN_0088B5A0), vec![vec![w.actor, delta]]);
        // Otherwise the clock difference, restarting when it ran backwards.
        e.mem.set_f32(w.actor + 0x114, 4.0);
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, false);
        assert_eq!(
            calls(&e, process_slot(0x700)),
            vec![vec![w.process, w.actor, 6.0f32.to_bits()]]
        );
        e.mem.set_f32(w.actor + 0x114, 20.0);
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, false);
        assert_eq!(
            calls(&e, process_slot(0x700)),
            vec![vec![w.process, w.actor, 0.0f32.to_bits()]]
        );
        // Between the two constants the restart steps by the clock itself.
        e.set_global(UPPER_BOUND_010290D0, 100.0f64);
        e.mem.set_f32(w.actor + 0x114, 20.0);
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, false);
        assert_eq!(
            calls(&e, process_slot(0x700)),
            vec![vec![w.process, w.actor, 10.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(w.actor + 0x114), 10.0);
    }

    #[test]
    fn update_magic_clears_the_source_for_most_animation_groups() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        e.register(FN_0096D490, |_, _| 1.0f32.into_ret());
        e.register(caster_slot(0x34), |_, _| 0x9000u32.into_ret());
        // No animation: the source is cleared.
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, true);
        assert_eq!(calls(&e, caster_slot(0x40)), vec![vec![w.caster, 0]]);
        e.register(process_slot(0x3e8), |_, _| 0x9100u32.into_ret());
        e.register(FN_0048F7F0, |_, _| 0x9200u32.into_ret());
        // Group 0x1a is one of the groups that clear nothing; 0x1b clears.
        e.register(FN_005F2420, |_, _| 0x1au32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, true);
        assert!(calls(&e, caster_slot(0x40)).is_empty());
        e.register(FN_005F2420, |_, _| 0x1bu32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, true);
        assert_eq!(calls(&e, caster_slot(0x40)).len(), 1);
        // Outside the table the source is cleared too.
        e.register(FN_005F2420, |_, _| 5u32.into_ret());
        start_log(&mut e);
        actor_update_magic(&mut e, actor, false, true);
        assert_eq!(calls(&e, caster_slot(0x40)).len(), 1);
    }

    #[test]
    fn caster_overrides_forward_to_the_process() {
        let (mut e, w) = setup();
        let caster = Ptr::new(w.caster);
        e.register(process_slot(0x3c8), |_, _| 0x44u32.into_ret());
        e.register(process_slot(0x3d0), |_, _| 0x55u32.into_ret());
        start_log(&mut e);
        fn_008c3fb0(&mut e, caster, 5, true);
        fn_008c4020(&mut e, caster, 5, 6, true);
        assert_eq!(fn_008c4420(&mut e, caster), 0x44);
        fn_008c4470(&mut e, caster, 7);
        assert_eq!(fn_008c44c0(&mut e, caster), 0x55);
        fn_008c4510(&mut e, caster, 8);
        assert_eq!(
            calls(&e, process_slot(0x3c0)),
            vec![vec![w.process, w.caster, 5, 1]]
        );
        assert_eq!(
            calls(&e, process_slot(0x3c4)),
            vec![vec![w.process, w.caster, 5, 6, 1]]
        );
        assert_eq!(calls(&e, process_slot(0x3cc)), vec![vec![w.process, 7]]);
        assert_eq!(calls(&e, process_slot(0x3d4)), vec![vec![w.process, 8]]);
        // Without a process nothing is forwarded and the getters answer 0.
        e.mem.set_u32(w.actor + 0x68, 0);
        start_log(&mut e);
        fn_008c3fb0(&mut e, caster, 5, true);
        fn_008c4020(&mut e, caster, 5, 6, true);
        assert_eq!(fn_008c4420(&mut e, caster), 0);
        fn_008c4470(&mut e, caster, 7);
        assert_eq!(fn_008c44c0(&mut e, caster), 0);
        fn_008c4510(&mut e, caster, 8);
        for slot in [0x3c0, 0x3c4, 0x3c8, 0x3cc, 0x3d0, 0x3d4] {
            assert!(calls(&e, process_slot(slot)).is_empty());
        }
    }

    #[test]
    fn target_and_actor_forwarders_ask_the_process() {
        let (mut e, w) = setup();
        e.register(process_slot(0x3b8), |_, _| 0x33u32.into_ret());
        e.register(process_slot(0x3d8), |_, _| 1u32.into_ret());
        assert_eq!(fn_008c4090(&mut e, Ptr::new(w.target)), 0x33);
        assert!(fn_008c4560(&mut e, Ptr::new(w.actor)));
        start_log(&mut e);
        fn_008c45a0(&mut e, Ptr::new(w.actor), true);
        assert_eq!(calls(&e, process_slot(0x3dc)), vec![vec![w.process, 1]]);
        e.mem.set_u32(w.actor + 0x68, 0);
        start_log(&mut e);
        assert_eq!(fn_008c4090(&mut e, Ptr::new(w.target)), 0);
        assert!(!fn_008c4560(&mut e, Ptr::new(w.actor)));
        fn_008c45a0(&mut e, Ptr::new(w.actor), true);
        assert!(calls(&e, process_slot(0x3dc)).is_empty());
    }

    #[test]
    fn fn_008c40e0_scales_an_effect() {
        let (mut e, w) = setup();
        let this = Ptr::new(w.target);
        let caster = Ptr::new(0x6001);
        let magic_item = object(&mut e);
        let effect = e.mem.alloc(0x100);
        let alchemy = e.mem.alloc(0x100);
        let ingredient = e.mem.alloc(0x100);
        e.mem.set_u32(ingredient + 0x3c, OTHER_VT);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (match a[3] {
                TYPE_ALCHEMY_ITEM => alchemy,
                TYPE_INGREDIENT_ITEM => ingredient,
                _ => a[0],
            })
            .into_ret()
        });
        let scale =
            |e: &mut Engine| fn_008c40e0(e, this, caster, Ptr::new(magic_item), Ptr::new(effect));
        // An empty list, and an effect with the 0x100 flag.
        assert_eq!(scale(&mut e), 1.0);
        e.mem.set_u32(magic_item + 0x18, 5);
        e.mem.set_u32(effect + 0x18, 0x100);
        assert_eq!(scale(&mut e), 1.0);
        e.mem.set_u32(effect + 0x18, 0);
        // The effect's actor value chain: effect +0xc, +0x14, +0x68.
        let part = e.mem.alloc(0x40);
        let resolved = e.mem.alloc(0x100);
        e.mem.set_u32(effect + 0xc, part);
        e.mem.set_u32(part + 0x14, resolved);
        e.mem.set_u32(resolved + 0x68, 0xffff_ffff);
        e.mem.set_f32(w.actor + 0x200 + 4 * 0x13, 4.0);
        e.mem.set_f32(w.actor + 0x200 + 4 * 0x77, 5.0);
        // Kind 5 reads actor value 0x13; no second value.
        e.mem.set_u32(magic_item + 0x70, 5);
        start_log(&mut e);
        assert_eq!(scale(&mut e), 4.0);
        assert_eq!(
            calls(&e, FN_006492A0),
            vec![vec![4.0f32.to_bits(), 0.0f32.to_bits()]]
        );
        // Another kind: only the second value, lowered to the setting.
        e.mem.set_u32(magic_item + 0x70, 0);
        e.mem.set_u32(resolved + 0x68, 0x77);
        assert_eq!(scale(&mut e), 5.0);
        e.register(actor_slot(0x360), |_, _| 1u32.into_ret());
        e.mem.set_f32(SETTING_011D0AD8 + 4, 2.0);
        assert_eq!(scale(&mut e), 2.0);
        // An alchemy item that is not ingestible needs the effect's part to
        // pass `00404710`.
        e.mem.set_u32(magic_item + 0x70, 7);
        assert_eq!(scale(&mut e), 1.0);
        e.register(FN_00404710, |_, _| 1u32.into_ret());
        assert_eq!(scale(&mut e), 2.0);
        // An ingestible one reads actor value 0x13 and ignores the second.
        e.register(FN_00405DA0, |_, _| 1u32.into_ret());
        assert_eq!(scale(&mut e), 4.0);
        // An ingredient whose part answers true applies fully.
        e.mem.set_u32(magic_item + 0x70, 8);
        e.register(slot_fn(OTHER_VT, 4), |_, _| 1u32.into_ret());
        assert_eq!(scale(&mut e), 1.0);
        e.register(slot_fn(OTHER_VT, 4), |_, _| Ret::default());
        assert_eq!(scale(&mut e), 2.0);
    }

    #[test]
    fn radiation_resistance_is_cached_until_reset() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        e.set_global(LIMIT_01016410, 2.0f32);
        e.set_global(FACTOR_01016408, 0.25f64);
        e.mem.set_f32(w.actor + 0x200 + 4 * 0x14, 3.0);
        e.mem.set_f32(w.actor + 0x19c, -1.0);
        start_log(&mut e);
        assert_eq!(actor_get_radiation_resistance_mult(&mut e, actor), 0.5);
        assert_eq!(e.mem.f32(w.actor + 0x19c), 0.5);
        assert_eq!(
            calls(&e, FN_0040EBD0),
            vec![vec![2.0f32.to_bits(), 3.0f32.to_bits()]]
        );
        // A cached value is returned as it is.
        start_log(&mut e);
        assert_eq!(actor_get_radiation_resistance_mult(&mut e, actor), 0.5);
        assert!(calls(&e, FN_0040EBD0).is_empty());
        // The setting replaces the limit when the actor's virtual holds.
        e.register(actor_slot(0x360), |_, _| 1u32.into_ret());
        e.mem.set_f32(SETTING_011D0AD8 + 4, 1.5);
        e.mem.set_f32(w.actor + 0x19c, -1.0);
        assert_eq!(actor_get_radiation_resistance_mult(&mut e, actor), 0.625);
        // A negative mix is clamped to 0.
        e.mem.set_f32(w.actor + 0x200 + 4 * 0x14, -1.0);
        e.mem.set_f32(w.actor + 0x19c, -1.0);
        assert_eq!(actor_get_radiation_resistance_mult(&mut e, actor), 1.0);
        // The reset restores the exe's float.
        e.set_global(RESET_VALUE_01012054, -2.0f32);
        fn_008c4400(&mut e, actor);
        assert_eq!(e.mem.f32(w.actor + 0x19c), -2.0);
    }

    #[test]
    fn fn_008c4610_answers_the_modifier_in_st0() {
        let (mut e, w) = setup();
        e.register(FN_008153A0, |_, _| Ret {
            st0: 0.75,
            ..Ret::default()
        });
        // ST0 at the RET 008c462e is 008153a0's; FindTargets_ov2 stores it (after 008171ca).
        let answer = e.call(0x008c_4610, &args![w.caster, 1u32, 1.5f32.to_bits()]);
        assert_eq!(answer.f64(), 0.75);
    }

    #[test]
    fn fn_008c45e0_and_fn_008c4610_are_small_caster_virtuals() {
        let (mut e, w) = setup();
        let float_out = e.mem.alloc(8);
        let int_out = e.mem.alloc(8);
        e.mem.set_u32(int_out, 7);
        assert!(fn_008c45e0(
            &mut e,
            Ptr::new(w.caster),
            1,
            Ptr::new(float_out),
            Ptr::new(int_out),
            2
        ));
        assert_eq!(e.mem.f32(float_out), 1.0);
        assert_eq!(e.mem.u32(int_out), 0);
        assert!(fn_008c45e0(
            &mut e,
            Ptr::new(w.caster),
            1,
            Ptr::NULL,
            Ptr::NULL,
            2
        ));
        start_log(&mut e);
        fn_008c4610(&mut e, Ptr::new(w.caster), true, 1.5);
        assert_eq!(
            calls(&e, FN_008153A0),
            vec![vec![w.caster, 1, 1.5f32.to_bits()]]
        );
    }

    #[test]
    fn update_alpha_multiplies_the_factors() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        e.register(process_slot(0x5ac), |_, _| 0.5f32.into_ret());
        e.register(FN_008BCDD0, |_, _| 0.5f32.into_ret());
        e.register(process_slot(0x610), |_, _| 2u32.into_ret());
        start_log(&mut e);
        actor_update_alpha(&mut e, actor);
        assert_eq!(
            calls(&e, actor_slot(0x41c)),
            vec![vec![w.actor, 0.25f32.to_bits()]]
        );
        // The player's alpha is scaled.
        e.set_global(PLAYER_CHARACTER, w.actor);
        e.set_global(PLAYER_ALPHA_SCALE_011A3B38, 2.0f32);
        start_log(&mut e);
        actor_update_alpha(&mut e, actor);
        assert_eq!(
            calls(&e, actor_slot(0x41c)),
            vec![vec![w.actor, 0.5f32.to_bits()]]
        );
        // A process with its +0x28 word set contributes nothing.
        e.mem.set_u32(w.process + 0x28, 1);
        start_log(&mut e);
        actor_update_alpha(&mut e, actor);
        assert_eq!(
            calls(&e, actor_slot(0x41c)),
            vec![vec![w.actor, 2.0f32.to_bits()]]
        );
        // A refractive actor gets its refraction value when `005b9b00` holds.
        let extra = e.mem.alloc(0x40);
        e.mem.set_f32(extra + 0xc, 0.75);
        e.register_double(FN_00422820, move |_, _| extra.into_ret());
        e.register(FN_005B9B00, |_, _| 1u32.into_ret());
        start_log(&mut e);
        actor_update_alpha(&mut e, actor);
        assert_eq!(
            calls(&e, actor_slot(0x384)),
            vec![vec![w.actor, 1, 0.75f32.to_bits()]]
        );
        assert!(calls(&e, actor_slot(0x41c)).is_empty());
        // Without the process the alpha is 1.
        e.register(FN_005B9B00, |_, _| Ret::default());
        e.mem.set_u32(w.actor + 0x68, 0);
        e.set_global(PLAYER_CHARACTER, 0);
        start_log(&mut e);
        actor_update_alpha(&mut e, actor);
        assert_eq!(
            calls(&e, actor_slot(0x41c)),
            vec![vec![w.actor, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn alpha_multiplier_accessors_go_through_the_process() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let process = Ptr::new(w.process);
        fn_008c47d0(&mut e, process, 0.4);
        assert_eq!(e.mem.f32(w.process + 0x2b0), 0.4);
        assert_eq!(fn_008c4830(&mut e, process), 0.4);
        assert_eq!(fn_008c47f0(&mut e, actor), 0.4);
        fn_008c4790(&mut e, actor, 0.9);
        assert_eq!(e.mem.f32(w.process + 0x2b0), 0.9);
        // A process with its +0x28 word set is left alone.
        e.mem.set_u32(w.process + 0x28, 1);
        fn_008c4790(&mut e, actor, 0.1);
        assert_eq!(e.mem.f32(w.process + 0x2b0), 0.9);
        assert_eq!(fn_008c47f0(&mut e, actor), 1.0);
        // The Xbox-named pair asks the process virtuals.
        e.register(process_slot(0x5ac), |_, _| 0.3f32.into_ret());
        assert_eq!(actor_get_alpha_mult(&mut e, actor), 0.3);
        start_log(&mut e);
        actor_set_alpha_mult(&mut e, actor, 0.6);
        assert_eq!(
            calls(&e, process_slot(0x5b0)),
            vec![vec![w.process, 0.6f32.to_bits()]]
        );
        e.mem.set_u32(w.actor + 0x68, 0);
        start_log(&mut e);
        assert_eq!(actor_get_alpha_mult(&mut e, actor), 1.0);
        actor_set_alpha_mult(&mut e, actor, 0.6);
        fn_008c4790(&mut e, actor, 0.2);
        assert_eq!(fn_008c47f0(&mut e, actor), 1.0);
        assert!(calls(&e, process_slot(0x5b0)).is_empty());
    }

    #[test]
    fn fn_008c48d0_applies_an_effect_and_flags_the_caster() {
        let (mut e, w) = setup();
        let this = Ptr::new(w.target);
        let magic_item = object(&mut e);
        e.mem.set_u32(magic_item + 0x18, 5);
        let effect = e.mem.alloc(0x100);
        let other_actor = e.mem.alloc(0x100);
        let enchantment = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(effect + 0x3c, holder);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (match a[3] {
                TYPE_ACTOR => other_actor,
                TYPE_ENCHANTMENT_ITEM => enchantment,
                _ => a[0],
            })
            .into_ret()
        });
        let apply = |e: &mut Engine, flag: bool| {
            fn_008c48d0(
                e,
                this,
                Ptr::new(0x6002),
                Ptr::new(magic_item),
                Ptr::new(effect),
                flag,
            )
        };
        // A restricted actor only takes kind 4.
        e.register(FN_008ACE90, |_, _| 1u32.into_ret());
        e.mem.set_u32(magic_item + 0x70, 3);
        start_log(&mut e);
        assert!(!apply(&mut e, false));
        assert!(calls(&e, FN_008230F0).is_empty());
        e.register(FN_008ACE90, |_, _| Ret::default());
        // Kind 2 is not accepted by `00804b30`: the caster is flagged and the
        // actor's virtual +0x4bc runs.
        e.register(FN_008230F0, |_, _| 1u32.into_ret());
        e.mem.set_u32(magic_item + 0x70, 2);
        start_log(&mut e);
        assert!(apply(&mut e, false));
        assert_eq!(
            calls(&e, FN_008230F0),
            vec![vec![w.target, 0x6002, magic_item, effect, 0]]
        );
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![other_actor, w.actor + 0x44, 0x80]]
        );
        assert_eq!(
            calls(&e, actor_slot(0x4bc)),
            vec![vec![w.actor, other_actor, effect]]
        );
        assert!(calls(&e, FN_007038B0).is_empty());
        // The player as caster actor is told.
        e.set_global(PLAYER_CHARACTER, other_actor);
        start_log(&mut e);
        apply(&mut e, false);
        assert_eq!(calls(&e, FN_007038B0), vec![vec![w.actor]]);
        e.set_global(PLAYER_CHARACTER, 0);
        // An alchemy item (kind 7) is not applied.
        e.mem.set_u32(magic_item + 0x70, 7);
        start_log(&mut e);
        apply(&mut e, false);
        assert!(calls(&e, actor_slot(0x4bc)).is_empty());
        // An enchantment whose `005f36f0` is 2 needs a holder of type 0x29
        // and the flag.
        e.mem.set_u32(magic_item + 0x70, 6);
        e.mem.set_u32(enchantment + 0x34, 2);
        start_log(&mut e);
        apply(&mut e, true);
        assert!(calls(&e, actor_slot(0x4bc)).is_empty());
        e.mem.set_u8(holder + 4, 0x29);
        start_log(&mut e);
        apply(&mut e, false);
        assert!(calls(&e, actor_slot(0x4bc)).is_empty());
        start_log(&mut e);
        apply(&mut e, true);
        assert_eq!(calls(&e, actor_slot(0x4bc)).len(), 1);
        // The same base object for caster and actor does nothing more.
        e.mem.set_u32(other_actor + 0x20, w.base);
        start_log(&mut e);
        apply(&mut e, true);
        assert!(calls(&e, SET_ACTION_FLAG).is_empty());
    }

    #[test]
    fn effect_added_reacts_to_an_active_effect() {
        let (mut e, w) = setup();
        let this = Ptr::new(w.target);
        let effect = e.mem.alloc(0x100);
        let part = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x40);
        let form = e.mem.alloc(0x100);
        e.mem.set_u32(effect + 0xc, part);
        e.mem.set_u32(part + 0x14, list);
        e.mem.set_u32(effect + 8, form);
        e.mem.set_u32(form + 0x3c, OTHER_VT);
        e.mem.set_u32(effect + 0x2c, 5);
        e.register(FN_0040DF90, |_, _| 5u32.into_ret());
        e.register(FN_00404710, |_, _| 1u32.into_ret());
        e.register(FN_00974E90, |_, _| 1u32.into_ret());
        e.register(target_slot(8), |_, _| 0x77u32.into_ret());
        let mut asked = 0;
        e.register_double(process_slot(0x5c8), move |_, _| {
            asked += 1;
            (asked == 2).into_ret()
        });
        start_log(&mut e);
        fn_008c4ab0(&mut e, this, Ptr::new(effect));
        assert_eq!(
            calls(&e, process_slot(0x5c8)),
            vec![vec![w.process, 0x77, 5], vec![w.process, 0x77, 5]]
        );
        assert_eq!(calls(&e, process_slot(0x5cc)), vec![vec![w.process]]);
        assert_eq!(
            calls(&e, FN_00974E90),
            vec![vec![PROCESS_LISTS, w.actor, 5]]
        );
        assert_eq!(calls(&e, FN_00805650), vec![vec![effect]]);
        assert_eq!(calls(&e, FN_008A7D50), vec![vec![w.actor, 1, 1]]);
        assert_eq!(calls(&e, FN_00403E60), vec![vec![list, 0x0100_0000]]);
        // An ingestible item or a type-4 effect leaves the actor alone.
        e.register(slot_fn(OTHER_VT, 4), |_, _| 1u32.into_ret());
        start_log(&mut e);
        fn_008c4ab0(&mut e, this, Ptr::new(effect));
        assert!(calls(&e, FN_008A7D50).is_empty());
        e.register(slot_fn(OTHER_VT, 4), |_, _| Ret::default());
        e.mem.set_u32(effect + 0x2c, 4);
        start_log(&mut e);
        fn_008c4ab0(&mut e, this, Ptr::new(effect));
        assert!(calls(&e, FN_008A7D50).is_empty());
        // No process or no effect: nothing.
        start_log(&mut e);
        fn_008c4ab0(&mut e, this, Ptr::NULL);
        e.mem.set_u32(w.actor + 0x68, 0);
        fn_008c4ab0(&mut e, this, Ptr::new(effect));
        assert!(e.call_log.as_ref().unwrap().is_empty());
        // The sibling virtual only pokes the process.
        e.mem.set_u32(w.actor + 0x68, w.process);
        start_log(&mut e);
        fn_008c4cb0(&mut e, this, Ptr::new(effect));
        assert_eq!(calls(&e, process_slot(0x5cc)), vec![vec![w.process]]);
        e.register(FN_0040DF90, |_, _| Ret::default());
        start_log(&mut e);
        fn_008c4cb0(&mut e, this, Ptr::new(effect));
        assert!(calls(&e, process_slot(0x5cc)).is_empty());
    }

    fn target_slot(slot: u32) -> u32 {
        slot_fn(TARGET_VT, slot)
    }

    #[test]
    fn the_two_object_makers_register_what_they_build() {
        let (mut e, w) = setup();
        let this = Ptr::new(w.target);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, OTHER_VT);
        e.register(FN_00AA13E0, |_, _| 0x8300u32.into_ret());
        e.register_double(FN_0081F580, move |_, _| object.into_ret());
        e.set_global(ID_011C3584, 7u32);
        e.set_global(ID_011C3580, 9u32);
        assert_eq!(fn_008c4e00(&mut e), 7);
        assert_eq!(fn_008c4f00(&mut e), 9);
        // An object whose virtual +0xc4 holds goes to the process lists.
        e.register(slot_fn(OTHER_VT, 0xc4), |_, _| 1u32.into_ret());
        start_log(&mut e);
        fn_008c4d10(&mut e, this, 0, 0);
        assert_eq!(calls(&e, FN_00AA13E0), vec![vec![0x6c]]);
        assert_eq!(
            calls(&e, FN_0081F580),
            vec![vec![0x8300, w.actor, 7, 0.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, FN_00973FD0), vec![vec![PROCESS_LISTS, object]]);
        start_log(&mut e);
        fn_008c4e10(&mut e, this, 0, 0);
        assert_eq!(
            calls(&e, FN_0081F580),
            vec![vec![0x8300, w.actor, 9, 0.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, FN_00973FD0), vec![vec![PROCESS_LISTS, object]]);
        // Otherwise it is deleted through its virtual +0.
        e.register(slot_fn(OTHER_VT, 0xc4), |_, _| Ret::default());
        start_log(&mut e);
        fn_008c4d10(&mut e, this, 0, 0);
        assert!(calls(&e, FN_00973FD0).is_empty());
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0)), vec![vec![object, 1]]);
        start_log(&mut e);
        fn_008c4e10(&mut e, this, 0, 0);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0)), vec![vec![object, 1]]);
    }

    #[test]
    fn fn_008c4f10_tells_every_accepted_node() {
        let (mut e, w) = setup();
        let first_object = e.mem.alloc(0x40);
        let second_object = e.mem.alloc(0x40);
        e.mem.set_u32(first_object, OTHER_VT);
        e.mem.set_u32(second_object, OTHER_VT);
        let last = node(&mut e, second_object, 0);
        let first = node(&mut e, first_object, last);
        e.register_double(target_slot(8), move |_, _| first.into_ret());
        e.register(FN_005AE380, |_, _| 1u32.into_ret());
        start_log(&mut e);
        fn_008c4f10(&mut e, Ptr::new(w.actor), 0x55);
        assert_eq!(
            calls(&e, slot_fn(OTHER_VT, 0x38)),
            vec![vec![first_object, 0x55], vec![second_object, 0x55]]
        );
        // The walk stops at the first node `005ae380` refuses.
        e.register_double(FN_005AE380, move |_, a| (a[0] == first).into_ret());
        start_log(&mut e);
        fn_008c4f10(&mut e, Ptr::new(w.actor), 0x55);
        assert_eq!(calls(&e, slot_fn(OTHER_VT, 0x38)).len(), 1);
        // A null value does nothing.
        start_log(&mut e);
        fn_008c4f10(&mut e, Ptr::new(w.actor), 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn the_cast_power_list_is_searched_removed_and_deleted() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        let (key_one, key_two) = (0x9001u32, 0x9002u32);
        let first_entry = e.mem.alloc(8);
        let second_entry = e.mem.alloc(8);
        e.mem.set_u32(first_entry, key_one);
        e.mem.set_u32(second_entry, key_two);
        let last = node(&mut e, second_entry, 0);
        e.mem.set_u32(w.actor + 0xf4, first_entry);
        e.mem.set_u32(w.actor + 0xf8, last);
        assert_eq!(fn_008c5000(&mut e, actor, Ptr::new(key_one)), first_entry);
        assert_eq!(fn_008c5000(&mut e, actor, Ptr::new(key_two)), second_entry);
        assert_eq!(fn_008c5000(&mut e, actor, Ptr::new(0x1234)), 0);
        assert_eq!(fn_008c5000(&mut e, actor, Ptr::NULL), 0);
        assert!(fn_008c4fe0(&mut e, actor, Ptr::new(key_two)));
        assert!(!fn_008c4fe0(&mut e, actor, Ptr::new(0x1234)));
        // Removal hands the list and a local holding the entry to `00905330`
        // and frees the entry.
        start_log(&mut e);
        fn_008c4f90(&mut e, actor, Ptr::new(key_two));
        let removal = calls(&e, FN_00905330);
        assert_eq!(removal.len(), 1);
        assert_eq!(removal[0][0], w.actor + 0xf4);
        assert_eq!(calls(&e, FN_00401030), vec![vec![second_entry]]);
        start_log(&mut e);
        fn_008c4f90(&mut e, actor, Ptr::new(0x1234));
        assert!(calls(&e, FN_00905330).is_empty());
        assert!(calls(&e, FN_00401030).is_empty());
        // Deleting frees every object and clears the list.
        start_log(&mut e);
        actor_delete_cast_powers(&mut e, actor);
        assert_eq!(
            calls(&e, FN_00401030),
            vec![vec![first_entry], vec![second_entry]]
        );
        assert_eq!(calls(&e, FN_00470470), vec![vec![w.actor + 0xf4]]);
    }

    #[test]
    fn fn_008c50f0_sets_the_refraction() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        e.register(actor_slot(0x1d0), |_, _| 0x6666u32.into_ret());
        e.register(process_slot(0x5b4), |_, _| 2.0f32.into_ret());
        // Enabled with a non-positive amount: the process supplies it.
        start_log(&mut e);
        fn_008c50f0(&mut e, actor, true, -1.0);
        assert_eq!(
            calls(&e, FN_00B68770),
            vec![vec![0x6666, 1, 2.0f32.to_bits(), 0, 0.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&e, FN_00422750),
            vec![vec![w.actor + 0x44, 1, (-1.0f32).to_bits()]]
        );
        // The process gives nothing either: the flag is cleared.
        e.register(process_slot(0x5b4), |_, _| 0.0f32.into_ret());
        start_log(&mut e);
        fn_008c50f0(&mut e, actor, true, -1.0);
        assert_eq!(
            calls(&e, FN_00B68770),
            vec![vec![0x6666, 0, 0.0f32.to_bits(), 0, 0.0f32.to_bits(), 0]]
        );
        // A positive amount is kept, and a disabled one asks nobody.
        start_log(&mut e);
        fn_008c50f0(&mut e, actor, true, 3.0);
        assert_eq!(
            calls(&e, FN_00B68770),
            vec![vec![0x6666, 1, 3.0f32.to_bits(), 0, 0.0f32.to_bits(), 0]]
        );
        start_log(&mut e);
        fn_008c50f0(&mut e, actor, false, -1.0);
        assert!(calls(&e, process_slot(0x5b4)).is_empty());
        assert_eq!(
            calls(&e, FN_00B68770),
            vec![vec![0x6666, 0, (-1.0f32).to_bits(), 0, 0.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn is_refractive_asks_the_extra_data_list() {
        let (mut e, w) = setup();
        let actor = Ptr::new(w.actor);
        assert!(!fn_008c51c0(&mut e, actor));
        e.register(FN_00422820, |_, _| 0x7777u32.into_ret());
        start_log(&mut e);
        assert!(fn_008c51c0(&mut e, actor));
        assert_eq!(calls(&e, FN_00422820), vec![vec![w.actor + 0x44]]);
    }
}

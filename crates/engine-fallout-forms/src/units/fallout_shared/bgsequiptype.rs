//! `fallout shared/bgsequiptype.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (`ECX`): the form type byte at +4, zero-extended.
const FORM_TYPE_00401170: u32 = 0x0040_1170;
/// `00726070` (`ECX`): the word at +4.
const WORD_4_00726070: u32 = 0x0072_6070;
/// The guard bits of the menu icon name's static (`011c4ba0`) and the static (`011c4b9c`,
/// the length of `"MenuIconEquipType"`).
const NAME_GUARD: u32 = 0x011c_4ba0;
const NAME_LENGTH: u32 = 0x011c_4b9c;
/// `"MenuIconEquipType"`, `"TypeIcons\hot_keys_icon_"`, `".dds"`.
const MENU_ICON_PREFIX: u32 = 0x0101_aa48;
const ICON_PATH_PREFIX: u32 = 0x0101_aa2c;
const ICON_PATH_SUFFIX: u32 = 0x0101_aa24;
/// The fourteen equip type names (`011869bc`, `"BigGuns"` ... `"Alcohol"`).
const EQUIP_TYPE_NAMES: u32 = 0x0118_69bc;
/// The fourteen default menu icons (`011c4b54`).
const DEFAULT_MENU_ICONS: u32 = 0x011c_4b54;
/// The first equip type menu icon's form id (`0x258`, one per type after it).
const FIRST_ICON_ID: u32 = 0x258;
/// `strlen` (`0044a670`, `cdecl`).
const STRLEN: u32 = 0x0044_a670;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVBGSMenuIcon@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_MENU_ICON: u32 = 0x0118_63f0;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `BGSMenuIcon::BGSMenuIcon` (Xbox PDB, `00590210`, `ECX`).
const MENU_ICON_CTOR: u32 = 0x0059_0210;
/// `strcpy_s` (`00406d30`) and `strcat_s` (`00406d50`), `cdecl`: destination, size, source.
const STRCPY_S: u32 = 0x0040_6d30;
const STRCAT_S: u32 = 0x0040_6d50;
/// `TESSoundFile::SetSoundFile` (engine map, `00489100`; by its body: copies a C string into
/// the `BSStringT` at `this + 4`), `thiscall`, one stack word.
const SET_STRING_00489100: u32 = 0x0048_9100;
/// The data handler (`011c3f2c`) and `TESDataHandler::AddFormToDataHandler` (Xbox PDB,
/// `004603b0`, `ECX` = data handler, one stack word).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;

// Translated from 00479430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEquipType::GetEquipType` (Xbox PDB), `cdecl` with one word, a form: -1 for null;
/// otherwise by its form type (`00401170`; the exe's byte map and jump table in `.text`,
/// `004794f0` and `004794d8`, are these arms) the word `00726070` reads at a component of the
/// form (its +4): type 0x18 at +0x154, 0x1a at +0x14c, 0x1d at +0x8c, 0x29 at +0xc4, 0x2f at
/// +0x9c; -1 for any other type.
pub fn bgs_equip_type_get_equip_type(e: &mut Engine, form: Ptr) -> u32 {
    if form.is_null() {
        return u32::MAX;
    }
    let form_type = e.call(FORM_TYPE_00401170, &args![form]).u32();
    let component = match form_type.wrapping_sub(0x18) {
        0x00 => 0x154,
        0x02 => 0x14c,
        0x05 => 0x8c,
        0x11 => 0xc4,
        0x17 => 0x9c,
        _ => return u32::MAX,
    };
    e.call(WORD_4_00726070, &args![form.addr() + component])
        .u32()
}

// Translated from 00479570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00479570 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeecfc, state variable) and the /GS cookie check (00ec408c) are not translated
/// `BGSEquipType::GenerateDefaultMenuIcons` (Xbox PDB), no words. The first time (bit 0 of
/// `011c4ba0`) the length of `"MenuIconEquipType"` is kept in `011c4b9c`. Then for each of
/// the fourteen equip types the menu icon form `0x258 + type` (cast to `BGSMenuIcon`) is
/// looked up; a missing one is made (`00401000` of 0x24 bytes and `BGSMenuIcon::BGSMenuIcon`,
/// null when the allocation fails) with that form id (vtable slot +0x128 with 1), the editor
/// id `"MenuIconEquipType"` + the type's name (slot +0x134), the texture
/// `"TypeIcons\hot_keys_icon_"` + name + `".dds"` (`00489100` on +0x18), built in a
/// 0x104-byte stack buffer, and is filed in the data handler. The icon (found or made) is kept
/// in `011c4b54 + type * 4`.
pub fn bgs_equip_type_generate_default_menu_icons(e: &mut Engine) {
    let guard = e.global::<u32>(NAME_GUARD);
    if guard & 1 == 0 {
        e.set_global(NAME_GUARD, guard | 1);
        let length = e.call(STRLEN, &args![MENU_ICON_PREFIX]).u32();
        e.set_global(NAME_LENGTH, length);
    }
    e.with_stack(0x104, |e, name| {
        for equip_type in 0..14u32 {
            let id = equip_type + FIRST_ICON_ID;
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let mut icon = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_MENU_ICON, 0u32],
                )
                .u32();
            if icon == 0 {
                let block = e.call(ALLOCATE, &args![0x24u32]).u32();
                icon = if block != 0 {
                    e.call(MENU_ICON_CTOR, &args![block]).u32()
                } else {
                    0
                };
                e.vcall(icon, 0x128, &args![id, 1u32]);
                let type_name = e.global::<u32>(EQUIP_TYPE_NAMES + equip_type * 4);
                e.call(STRCPY_S, &args![name, 0x104u32, MENU_ICON_PREFIX]);
                e.call(STRCAT_S, &args![name, 0x104u32, type_name]);
                e.vcall(icon, 0x134, &args![name]);
                e.call(STRCPY_S, &args![name, 0x104u32, ICON_PATH_PREFIX]);
                let type_name = e.global::<u32>(EQUIP_TYPE_NAMES + equip_type * 4);
                e.call(STRCAT_S, &args![name, 0x104u32, type_name]);
                e.call(STRCAT_S, &args![name, 0x104u32, ICON_PATH_SUFFIX]);
                e.call(SET_STRING_00489100, &args![icon + 0x18, name]);
                let data_handler = e.global::<u32>(DATA_HANDLER);
                e.call(ADD_FORM_TO_DATA_HANDLER, &args![data_handler, icon]);
            }
            e.set_global(DEFAULT_MENU_ICONS + equip_type * 4, icon);
        }
    });
}

/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The chunk signature `ETYP` (`0x50595445`).
const CHUNK_ETYP: u32 = 0x5059_5445;
/// The `BaseFormComponent` and `BGSEquipType` type descriptors (`01183040`, `01183290`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_BGS_EQUIP_TYPE: u32 = 0x0118_3290;
/// `006ecd40` (`ECX` = object, `RET 4`): stores its argument at +4 (the engine map names it
/// `CombatProcedureAttackMelee::Initialize`, a folded body).
const SET_WORD_4: u32 = 0x006e_cd40;
/// `00408da0` (`ECX` = string holder): its text (`00559450` on +4), or the empty string
/// `01011584` (the engine map names it `MapMarkerData::GetLocationName`, a folded body).
const STRING_TEXT: u32 = 0x0040_8da0;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// The number of equip types with a default menu icon.
const EQUIP_TYPE_COUNT: i32 = 14;

/// `__RTDynamicCast` of `other` from `BaseFormComponent` to `BGSEquipType`.
fn cast_to_equip_type(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_BGS_EQUIP_TYPE,
            0u32
        ],
    )
    .u32()
}

// Translated from 00479350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEquipType::Save` (Xbox PDB; `thiscall`, no stack word): saves the equip type (the
/// word at +4) in an `ETYP` chunk (`00485910`).
pub fn bgs_equip_type_save(e: &mut Engine, this: Ptr) {
    let equip_type = e.mem.u32(this.addr() + 4);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_ETYP, equip_type]);
}

// Translated from 00479380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEquipType::CopyComponent` (Xbox PDB; `RET 4`): casts `other` to a `BGSEquipType`
/// and, when it is one, copies its equip type (the word at +4, read through `00726070`
/// and stored through `006ecd40`).
pub fn bgs_equip_type_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = cast_to_equip_type(e, other);
    if cast == 0 {
        return;
    }
    let equip_type = e.call(WORD_4_00726070, &args![cast]).u32();
    e.call(SET_WORD_4, &args![this, equip_type]);
}

// Translated from 004793d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEquipType::CompareComponent` (Xbox PDB; `RET 4`, result in `AL`): true ("differs")
/// when `other` is not a `BGSEquipType` or its equip type (+4, through `00726070`)
/// differs from this one's.
pub fn bgs_equip_type_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_to_equip_type(e, other);
    if cast == 0 {
        return true;
    }
    let ours = e.call(WORD_4_00726070, &args![this]).u32();
    let theirs = e.call(WORD_4_00726070, &args![cast]).u32();
    ours != theirs
}

// Translated from 00479510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default menu icon of `equip_type` (`cdecl`): the entry of the table at `011c4b54`
/// that `GenerateDefaultMenuIcons` fills, 0 for a type outside 0..14 (signed compare).
pub fn fn_00479510(e: &mut Engine, equip_type: i32) -> u32 {
    if equip_type > -1 && equip_type < EQUIP_TYPE_COUNT {
        e.global::<u32>(DEFAULT_MENU_ICONS.wrapping_add((equip_type as u32).wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 00479540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEquipType::GetEquipTypeIconFilename` (Xbox PDB; `cdecl`): the icon file name of
/// the default menu icon of `equip_type` (`00408da0` on the icon's string at +0x18), or
/// the empty string `01011584` when there is no icon.
pub fn bgs_equip_type_get_equip_type_icon_filename(e: &mut Engine, equip_type: i32) -> u32 {
    let icon = fn_00479510(e, equip_type);
    if icon == 0 {
        return EMPTY_STRING;
    }
    e.call(STRING_TEXT, &args![icon + 0x18]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_9430, bgs_equip_type_get_equip_type(Ptr) -> u32),
        entry!(0x0047_9570, bgs_equip_type_generate_default_menu_icons()),
        entry!(0x0047_9350, bgs_equip_type_save(Ptr)),
        entry!(0x0047_9380, bgs_equip_type_copy_component(Ptr, Ptr)),
        entry!(0x0047_93d0, bgs_equip_type_compare_component(Ptr, Ptr) -> bool),
        entry!(0x0047_9510, fn_00479510(i32) -> u32),
        entry!(
            0x0047_9540,
            bgs_equip_type_get_equip_type_icon_filename(i32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00479430_reads_the_component_of_each_type() {
        let mut e = Engine::new();
        e.register(FORM_TYPE_00401170, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(WORD_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let form = e.mem.alloc(0x160);
        for (offset, value) in [
            (0x154u32, 1u32),
            (0x14c, 2),
            (0x8c, 3),
            (0xc4, 4),
            (0x9c, 5),
        ] {
            e.mem.set_u32(form + offset + 4, value);
        }
        for (form_type, want) in [
            (0x18u8, 1u32),
            (0x1a, 2),
            (0x1d, 3),
            (0x29, 4),
            (0x2f, 5),
            (0x19, u32::MAX),
            (0x30, u32::MAX),
            (0x17, u32::MAX),
        ] {
            e.mem.set_u8(form + 4, form_type);
            assert_eq!(
                e.call(0x0047_9430, &args![form]).u32(),
                want,
                "type {form_type:#x}"
            );
        }
        assert_eq!(e.call(0x0047_9430, &args![0u32]).u32(), u32::MAX);
    }

    fn icons_rig(found_below: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_4000, 0x1000);
        e.map(0x0118_6000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        for equip_type in 0..14u32 {
            e.mem
                .set_u32(EQUIP_TYPE_NAMES + equip_type * 4, 0x0a00_0000 + equip_type);
        }
        e.set_global(DATA_HANDLER, 0x0555_0000u32);
        e.register(STRLEN, |_, _| 17u32.into_ret());
        // Ids below `found_below` have an icon.
        e.register_double(LOOKUP_FORM, move |_, a| {
            if a[0] < found_below {
                (0x0900_0000 + a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MENU_ICON_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0f01_2000);
            a[0].into_ret()
        });
        e.put_vtable(0x0f01_2000, &[0; 0x4e]);
        e.mem.set_u32(0x0f01_2128, 0x0f01_2228);
        e.mem.set_u32(0x0f01_2134, 0x0f01_2234);
        e.register(0x0f01_2228, |_, _| Ret::default());
        e.register(0x0f01_2234, |_, _| Ret::default());
        for addr in [
            STRCPY_S,
            STRCAT_S,
            SET_STRING_00489100,
            ADD_FORM_TO_DATA_HANDLER,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00479570_keeps_found_icons_and_makes_the_rest() {
        let mut e = icons_rig(FIRST_ICON_ID + 13);
        e.call(0x0047_9570, &args![]);
        assert_eq!(e.global::<u32>(NAME_LENGTH), 17);
        assert_eq!(e.global::<u32>(NAME_GUARD), 1);
        assert_eq!(
            e.global::<u32>(DEFAULT_MENU_ICONS),
            0x0900_0000 + FIRST_ICON_ID
        );
        let made = e.global::<u32>(DEFAULT_MENU_ICONS + 13 * 4);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x24]]);
        assert_eq!(
            calls(&e, 0x0f01_2228),
            vec![vec![made, FIRST_ICON_ID + 13, 1]]
        );
        let name = calls(&e, 0x0f01_2234)[0][1];
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![
                vec![name, 0x104, MENU_ICON_PREFIX],
                vec![name, 0x104, ICON_PATH_PREFIX]
            ]
        );
        assert_eq!(
            calls(&e, STRCAT_S),
            vec![
                vec![name, 0x104, 0x0a00_000d],
                vec![name, 0x104, 0x0a00_000d],
                vec![name, 0x104, ICON_PATH_SUFFIX]
            ]
        );
        assert_eq!(
            calls(&e, SET_STRING_00489100),
            vec![vec![made + 0x18, name]]
        );
        assert_eq!(
            calls(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x0555_0000, made]]
        );
        // The second time the name length is not taken again.
        e.call(0x0047_9570, &args![]);
        assert_eq!(calls(&e, STRLEN).len(), 1);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_writes_the_equip_type() {
        let mut e = Engine::new();
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0047_9350, &args![this]);
        assert_eq!(
            calls(&e, ADD_CHUNK_WORD),
            vec![vec![CHUNK_ETYP, 0xffff_ffff]]
        );
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(WORD_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SET_WORD_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn copy_component_copies_the_equip_type_of_an_equip_type() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 9);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_9380, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_EQUIP_TYPE,
                0
            ]]
        );
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call(0x0047_9380, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 9);
    }

    #[test]
    fn compare_component_differs_unless_the_same_equip_type() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(other + 4, 3);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        assert!(!e.call(0x0047_93d0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 4);
        assert!(e.call(0x0047_93d0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 3);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0047_93d0, &args![this, other]).bool());
    }

    fn engine_with_icons() -> Engine {
        let mut e = Engine::new();
        e.mem.map(DEFAULT_MENU_ICONS, 14 * 4);
        for i in 0..14u32 {
            e.set_global(DEFAULT_MENU_ICONS + i * 4, 0x9000 + i * 0x100);
        }
        e
    }

    #[test]
    fn default_menu_icon_reads_the_table_inside_its_range_only() {
        let mut e = engine_with_icons();
        assert_eq!(e.call(0x0047_9510, &args![0u32]).u32(), 0x9000);
        assert_eq!(e.call(0x0047_9510, &args![13u32]).u32(), 0x9d00);
        assert_eq!(e.call(0x0047_9510, &args![14u32]).u32(), 0);
        assert_eq!(e.call(0x0047_9510, &args![-1i32]).u32(), 0);
    }

    #[test]
    fn icon_filename_is_the_icon_string_or_empty() {
        let mut e = engine_with_icons();
        e.set_global(DEFAULT_MENU_ICONS + 4, 0u32);
        e.register(STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_9540, &args![2u32]).u32(), 0x9218 + 1);
        assert_eq!(calls(&e, STRING_TEXT), vec![vec![0x9218]]);
        assert_eq!(e.call(0x0047_9540, &args![1u32]).u32(), EMPTY_STRING);
        assert_eq!(e.call(0x0047_9540, &args![20u32]).u32(), EMPTY_STRING);
    }
}

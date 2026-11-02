//! `BSMenu/menu.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the base class of every menu (`Menu`): its fades, its menu
//! tile, the templates it renders, and the menus' XML archive: the menu
//! files read into memory and compressed (`Menu::XMLHash`, a string map of
//! `XMLStorage *`), the parsed files (`Menu::XMLParsedHash`, a string map of
//! `Tile::BuildStorage *`) and the packed `final_master_xml.dat` built from
//! and loaded into them. The container code the unit emitted follows: the
//! two string maps' destructors, constructors and members, and the
//! `NiTArray` code of the `XMLStorage *` array and of the menu list
//! (`Menu::xMenuList`).
//!
//! Session 1 translated the whole unit (`00a1c6f0` to `00a1ee60`, 51
//! functions). Ten of them have a C++ exception frame or a stack cookie
//! check, which are not translated; each says so with a
//! `Translation incomplete` line.
//!
//! Conventions of the whole unit:
//!
//! * Static members of `Menu` (Xbox PDB) are globals: `pTileptrStr` at
//!   [`TILE_PTR_STRING`], `XMLHash` at [`XML_HASH`], `XMLParsedHash` at
//!   [`XML_PARSED_HASH`] and `xMenuList` at [`MENU_LIST`]. The two maps were
//!   matched by what [`menu_free_xml_archive`] deletes from each.
//! * Files are `BSFile`s used through their `NiBinaryStream` members: the
//!   absolute position (+4), the read function (+8) and the write function
//!   (+0xC), each `cdecl (file, buffer, size, &count, 1)`; vtable slot 0 is
//!   the deleting destructor.
//! * Locals the game keeps on its stack and passes by address are
//!   [`Engine::with_stack`] blocks.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, NiTArray, NiTPointerMap};
use crate::units::bsmenu::tile::{BuildStorage, Tile, TileTemplate, XmlStorage};
use crate::units::platform::MEMORY_MANAGER;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `Menu` (Xbox PDB), 0x28 bytes; the constructor (`00a1c4a0`) writes
    /// the same fields as the PDB at the same offsets.
    pub struct Menu: 0x28 {
        /// `pMenu` (Xbox PDB): the menu's `TileMenu`.
        0x04 pMenu: Ptr<Tile>,
        /// `xTemplates` (Xbox PDB): `BSSimpleList<Tile::TileTemplate *>`,
        /// first node embedded (item at +8, next at +0xC).
        0x08 xTemplates: Inline<BSSimpleList>,
        /// `pLastTile` (Xbox PDB).
        0x10 pLastTile: Ptr<Tile>,
        /// `iIsModal` (Xbox PDB). On PC [`menu_set_menu_tile`] stores the
        /// answer of the class slot (+0x34) here and the enter stack is
        /// popped with it.
        0x14 iIsModal: i32,
        /// `iMenuThickness` (Xbox PDB).
        0x18 iMenuThickness: i32,
        /// `bRenderedMenu` (Xbox PDB).
        0x1C bRenderedMenu: bool,
        /// `bDeleteTemplates` (Xbox PDB).
        0x1D bDeleteTemplates: bool,
        /// `iClass` (Xbox PDB): the menu id.
        0x20 iClass: i32,
        /// `xFadeState` (Xbox PDB): an `Interface::FADE_STATE`
        /// (`FADE_SHOWN` 1, `FADING_OUT` 2, `FADE_HIDDEN` 4, `FADING_IN` 8).
        0x24 xFadeState: i32,
    }
}

/// `Interface::FADE_STATE` (Xbox PDB) values.
const FADE_SHOWN: i32 = 1;
const FADING_OUT: i32 = 2;
const FADE_HIDDEN: i32 = 4;
const FADING_IN: i32 = 8;

// ---------------------------------------------------------------------------
// Constants

/// `Menu::pTileptrStr` (Xbox PDB static): a heap `NiFixedString` holding
/// `"Tileptr"` ([`TILEPTR_NAME`]).
pub(crate) const TILE_PTR_STRING: u32 = 0x011f_34cc;
const TILEPTR_NAME: u32 = 0x0109_54cc;
/// `Menu::XMLHash` (Xbox PDB static): `NiTStringPointerMap<XMLStorage *>`
/// (bucket count at +4, buckets at +8, copy flag at +0x10).
pub(crate) const XML_HASH: u32 = 0x011f_34e0;
/// `Menu::XMLParsedHash` (Xbox PDB static):
/// `NiTStringPointerMap<Tile::BuildStorage *>`.
pub(crate) const XML_PARSED_HASH: u32 = 0x011f_34f4;
/// `Menu::xMenuList` (Xbox PDB static): the array of menu tiles by menu id
/// (index `id - 1001`); `m_usMaxSize` at +8.
pub(crate) const MENU_LIST: u32 = 0x011f_3508;
/// The `Tile::BuildStorage *` that [`menu_render_template`] installs while it
/// builds (only that function uses the word).
const RENDER_BUILD_STORAGE: u32 = 0x011f_32d0;

/// `MemoryManager::Allocate(size)` / `Deallocate(block)` on the singleton.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `alloc(bytes)` and `free(block)` of the Gamebryo allocator (`cdecl`; the
/// container buffers and copied keys).
const ALLOC_BYTES: u32 = 0x00aa_1070;
const FREE_BYTES: u32 = 0x00aa_10f0;
/// `NiFixedString` creation from a `char *` (answers the counted handle),
/// the handle of the empty string (the word at `0109b220`) and the
/// `InterlockedDecrement` import slot that releases a handle (`handle - 8`).
const NI_FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Hash map steps used on the two string maps (no PDB names): the iteration
/// step `(map, &node, &key, &value)`, `SetAt(key, value)` and `RemoveAll`.
const MAP_NEXT: u32 = 0x00a0_c850;
const MAP_SET_AT: u32 = 0x00a0_c900;
const MAP_REMOVE_ALL: u32 = 0x00ae_91f0;
/// Map vtable slots: `KeyToHashIndex`, `IsKeysEqual`, `ClearValue` and
/// `DeleteItem` (Xbox PDB `NiTMapBase`).
const MAP_SLOT_HASH: u32 = 0x04;
const MAP_SLOT_KEYS_EQUAL: u32 = 0x08;
const MAP_SLOT_CLEAR_VALUE: u32 = 0x10;
const MAP_SLOT_DELETE_ITEM: u32 = 0x18;

/// `XMLStorage::_scalar_deleting_destructor_` (Xbox PDB), `(this, 1)`.
const XML_STORAGE_SCALAR_DELETING_DESTRUCTOR: u32 = 0x00a0_be10;
/// `Tile::BuildStorage::BuildStorage` / `~BuildStorage` (Xbox PDB).
const BUILD_STORAGE_CONSTRUCT: u32 = 0x00a0_ac80;
const BUILD_STORAGE_DESTROY: u32 = 0x00a0_ad40;
/// `Tile::TileTemplate::~TileTemplate` (Xbox PDB).
const TEMPLATE_DESTROY: u32 = 0x00a0_c190;
/// `Tile::ParseFile(name, buffer, length)` (Xbox PDB, `cdecl`), the release
/// of the pool of unused template items (`00a0ba50`), and
/// `Tile::InsertIncludedFiles(xml)` (Xbox PDB, `cdecl`).
const TILE_PARSE_FILE: u32 = 0x00a0_1e20;
const TEMPLATE_ITEM_POOL_FREE: u32 = 0x00a0_ba50;
const TILE_INSERT_INCLUDED_FILES: u32 = 0x00a0_2d40;
/// `Tile::BuildAndNameTree(template)` and `Tile::ConnectTraitsToTree
/// (template)` (Xbox PDB, `thiscall` on a tile).
const TILE_BUILD_AND_NAME_TREE: u32 = 0x00a0_31b0;
const TILE_CONNECT_TRAITS_TO_TREE: u32 = 0x00a0_33b0;
/// Tile trait access (`thiscall` on a tile): the float of a trait (`ST0`),
/// `Tile::IsTrue(trait)` (Xbox PDB), the float setter `(trait, value,
/// flag)` and `Tile::GetMenu()` (Xbox PDB).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `BSSimpleList<...>::AddHead(&item)` (`thiscall` on the list; the map
/// names the folded body `BSSimpleList<Tile::FadeControl *>::AddHead`).
const SIMPLE_LIST_ADD_HEAD: u32 = 0x00a0_c7c0;
/// `~Menu` (`00a1c520`, `thiscall`).
const MENU_DESTROY: u32 = 0x00a1_c520;

/// `Interface` functions (Xbox PDB names, `cdecl`): `NewTimer(menu,
/// seconds)`, `UpdateAllTimers()`, `ClearTimer(menu)`,
/// `PopFromEnterStack(id, flag)`, `AddToEnterStack(id, flag)`,
/// `SetCurrentFocusTarget(tile)`, `RecursiveFade(node, a, b)` and
/// `GetMenusRoot()`.
const INTERFACE_NEW_TIMER: u32 = 0x0070_6f50;
const INTERFACE_UPDATE_ALL_TIMERS: u32 = 0x0070_6f70;
const INTERFACE_CLEAR_TIMER: u32 = 0x0070_6f90;
const INTERFACE_POP_FROM_ENTER_STACK: u32 = 0x0070_6fd0;
const INTERFACE_ADD_TO_ENTER_STACK: u32 = 0x0070_6fb0;
const INTERFACE_SET_CURRENT_FOCUS_TARGET: u32 = 0x0070_6ff0;
const INTERFACE_RECURSIVE_FADE: u32 = 0x0070_7010;
const INTERFACE_GET_MENUS_ROOT: u32 = 0x0070_6cd0;
/// Interface manager words without PDB names: the getter of +0xCC and its
/// setter, the getter of +0xD0 and its setter, and the tile `007070a0`
/// answers.
const INTERFACE_GET_CC: u32 = 0x0070_7040;
const INTERFACE_SET_CC: u32 = 0x0070_7050;
const INTERFACE_GET_D0: u32 = 0x0070_7070;
const INTERFACE_SET_D0: u32 = 0x0070_7080;
const INTERFACE_DEPTH_TILE: u32 = 0x0070_70a0;

/// `FileFinder::GetFile_ov2(name, 0, size, flags)` and
/// `FileFinder::GetFile(name, 0, 0)` (Xbox PDB, `cdecl`), `BSFile::ReadF
/// (buffer, size)` and `BSFile::BSFile(name, mode, size, flag)` (Xbox PDB).
const FILE_FINDER_GET_FILE_OV2: u32 = 0x00af_df20;
const FILE_FINDER_GET_FILE: u32 = 0x00af_df00;
const BS_FILE_READ: u32 = 0x00b0_0650;
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// The `NiBinaryStream` read and write function members of a file.
const FILE_READ_FUNCTION: u32 = 0x08;
const FILE_WRITE_FUNCTION: u32 = 0x0c;
/// The byte at +0x2C of a file that the code tests before reading it (the
/// file opened), and the vtable slots the code calls: +0x20 with `(0, 0)`
/// before reading the packed file, +0x28 for the number of bytes to read.
const FILE_OPENED: u32 = 0x2c;
const FILE_SLOT_REWIND: u32 = 0x20;
const FILE_SLOT_SIZE: u32 = 0x28;

/// CRT functions (`cdecl`).
const STRRCHR: u32 = 0x00ec_6e30;
const STRSTR: u32 = 0x00ec_7750;
const STRLEN: u32 = 0x00ec_6130;
const STRCMP: u32 = 0x00ec_6da0;
const STRCPY_S: u32 = 0x00ec_65a6;
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiStricmp(a, b)` (`cdecl`).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `_ftol2_sse`: truncates the value in `ST0` (an `f64` argument here).
const FTOL: u32 = 0x00ec_62c0;
/// `sprintf_s`-like formatter `(buffer, size, format, ...)` and `Error(text)`
/// (`cdecl`).
const FORMAT: u32 = 0x0040_6d00;
const ERROR: u32 = 0x0040_fbe0;
/// Constructor of an `NiTPrimitiveArray<char const *>` `(max, grow)` and
/// its `Add(&item)` (`thiscall`).
const STRING_ARRAY_CONSTRUCT: u32 = 0x005e_0a30;
const STRING_ARRAY_ADD: u32 = 0x0096_a610;

/// Strings in `.rdata`.
const MASTER_MENU_FILE: u32 = 0x0109_54e4; // "Data\Menus\master_menu_file.txt"
const FILE_SEPARATOR: u32 = 0x0109_54dc; // "@#$%"
const XML_EXTENSION: u32 = 0x0109_54d4; // ".xml"
const FINAL_MASTER_XML: u32 = 0x0109_5504; // "DATA\FINAL_MASTER_XML.DAT"
const FINAL_MASTER_XML_OUT: u32 = 0x0109_5528; // "D:\Data\final_master_xml.dat"
const ADDED_FORMAT: u32 = 0x0109_5548; // "Added %s of size %i to FinalMasterXML\n"
/// Version word at the start of `final_master_xml.dat`.
const FINAL_MASTER_VERSION: u32 = 100;
/// Bytes of a packed entry's header: the 0x80-byte name and the size word.
const PACKED_ENTRY_HEADER: u32 = 0x84;

/// Constants in `.rdata`: `0.0` and `2.0` (`double`), `0.25` (`float`, the
/// fade-in time of a menu without tile), the four values of trait `0xfa7`
/// that let [`menu_set_menu_tile`] continue (`102.0`, `6006.0`, `103.0`,
/// `6007.0`, `double`), `10.0` (`float`), `100.0` and `-0.008` (`double`).
const ZERO: u32 = 0x0101_2060;
const TWO: u32 = 0x0101_1590;
const DEFAULT_FADE_IN_TIME: u32 = 0x0101_622c;
const MENU_TYPE_VALUES: [u32; 4] = [0x0107_04e8, 0x0107_04f0, 0x0109_4bb8, 0x0109_5578];
const MIN_DEPTH: u32 = 0x0101_7b78;
const DEPTH_OFFSET: u32 = 0x0101_7a40;
const DEPTH_SCALE: u32 = 0x0106_f290;

/// Trait ids the menu code reads and writes (numbers as the code uses them).
const TRAIT_VISIBLE: u32 = 0xfa3;
const TRAIT_MENU_TYPE: u32 = 0xfa7;
const TRAIT_DEPTH: u32 = 0xfad;
const TRAIT_FADE_TIME: u32 = 0xfc1;
const TRAIT_FADE_TIME_DEFAULT: u32 = 0xfc2;
const TRAIT_FOCUS: u32 = 0xfc3;
const TRAIT_POPS_ENTER_STACK: u32 = 0x1772;

/// `Menu` vtable slot +0x34 (`GetClass`, Xbox PDB).
const MENU_SLOT_GET_CLASS: u32 = 0x34;

/// Vtables written by the container code (RTTI names from the exe).
/// `NiTArray<XMLStorage *, NiTMallocInterface<XMLStorage *>>` and the
/// one-slot `NiTPrimitiveArray<XMLStorage *>`.
const VTABLE_XML_ARRAY: u32 = 0x0109_5524;
const VTABLE_XML_PRIMITIVE_ARRAY: u32 = 0x0109_5574;
/// `NiTArray<char const *, ...>` and `NiTPrimitiveArray<char const *>`.
const VTABLE_STRING_ARRAY: u32 = 0x0103_d418;
const VTABLE_STRING_PRIMITIVE_ARRAY: u32 = 0x0103_d420;
/// `NiTStringTemplateMap<NiTPointerMap<char const *, XMLStorage *>, ...>`,
/// `NiTPointerMap<char const *, XMLStorage *>` and its `NiTMapBase`.
const VTABLE_XML_STRING_MAP: u32 = 0x0109_55a4;
const VTABLE_XML_POINTER_MAP: u32 = 0x0109_55c4;
const VTABLE_XML_MAP_BASE: u32 = 0x0109_55e4;
/// The same three levels for `Tile::BuildStorage *`.
const VTABLE_BUILD_STRING_MAP: u32 = 0x0109_5624;
const VTABLE_BUILD_POINTER_MAP: u32 = 0x0109_5644;
const VTABLE_BUILD_MAP_BASE: u32 = 0x0109_5664;
/// `NiTArray<Tile *, NiTMallocInterface<Tile *>>`.
const VTABLE_TILE_ARRAY: u32 = 0x0109_568c;

// ---------------------------------------------------------------------------
// Helpers

/// `new`: a block from the memory manager singleton.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// `delete`: gives a block back to the memory manager singleton.
fn deallocate(e: &mut Engine, block: u32) {
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// Releases an `NiFixedString` handle unless it is the empty string's.
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The first node of the first non-empty bucket of a hash map, as the
/// inlined `GetFirstPos` reads it (bucket count and buckets re-read each
/// step).
fn first_map_node(e: &mut Engine, map: u32) -> u32 {
    let mut bucket = 0;
    while bucket < e.mem.u32(map + 4) {
        let node = e.mem.u32(e.mem.u32(map + 8) + bucket * 4);
        if node != 0 {
            return node;
        }
        bucket += 1;
    }
    0
}

/// One `NiBinaryStream` transfer: calls the read (+8) or write (+0xC)
/// function member `(file, buffer, size, &count, 1)` with a count word of 1
/// on the stack, adds the answer to the position (+4) and answers it.
fn file_transfer(e: &mut Engine, file: u32, member: u32, buffer: u32, size: u32) -> u32 {
    let function = e.mem.u32(file + member);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(function, &args![file, buffer, size, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(done));
    done
}

/// Deletes a file object through its vtable slot 0 with 1.
fn delete_file(e: &mut Engine, file: u32) {
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
}

/// Deletes a `Tile::BuildStorage` (`~BuildStorage`, then the block).
fn delete_build_storage(e: &mut Engine, storage: u32) {
    if storage != 0 {
        e.call(BUILD_STORAGE_DESTROY, &args![storage]);
        deallocate(e, storage);
    }
}

/// `GetAt(key, &value)` on one of the archive maps, starting from `null`.
fn archive_lookup(e: &mut Engine, map: u32, key: u32) -> u32 {
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), 0);
        fn_00a1e380(e, Ptr::new(map), key, value.addr());
        e.mem.u32(value.addr())
    })
}

/// The `Tile` of a menu (`pMenu`).
fn menu_tile(e: &Engine, this: Ptr<Menu>) -> Ptr<Tile> {
    e.get(this, Menu::pMenu)
}

/// Sets the float of a trait of a tile with the flag 1.
fn set_trait(e: &mut Engine, tile: u32, trait_id: u32, value: f32) {
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, 1u32]);
}

/// The float of a trait of a tile, as the `ST0` it answers.
fn trait_float(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64()
}

/// The fade of a menu's model: `Interface::RecursiveFade(node, 1.0, 0.0)`
/// on the `spModel` (+0x2C) of the menu tile.
fn fade_model(e: &mut Engine, this: Ptr<Menu>) {
    let tile = menu_tile(e, this);
    let node = e.get(tile, Tile::spModel);
    e.call(INTERFACE_RECURSIVE_FADE, &args![node, 1.0f32, 0.0f32]);
}

/// When the menu holds an enter-stack id (+0x14) and its tile's trait
/// `0x1772` is true: pops the id from the enter stack and clears the focus.
fn pop_enter_stack_if_needed(e: &mut Engine, this: Ptr<Menu>) {
    if e.get(this, Menu::iIsModal) != 0 {
        let tile = menu_tile(e, this);
        if e.call(TILE_IS_TRUE, &args![tile, TRAIT_POPS_ENTER_STACK])
            .bool()
        {
            let id = e.get(this, Menu::iIsModal);
            e.call(INTERFACE_POP_FROM_ENTER_STACK, &args![id, 0u32]);
            e.call(INTERFACE_SET_CURRENT_FOCUS_TARGET, &args![0u32]);
        }
    }
}

/// A map base level's destructor: sets `vtable`, empties the map and frees
/// the bucket array.
fn map_base_destroy(e: &mut Engine, map: u32, vtable: u32) {
    e.mem.set_u32(map, vtable);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let buckets = e.mem.u32(map + 8);
    e.call(FREE_BYTES, &args![buckets]);
}

/// A pointer map level's destructor: empties the map under `vtable`, then
/// the base level's destructor.
fn pointer_map_destroy(e: &mut Engine, map: u32, vtable: u32, base_vtable: u32) {
    e.mem.set_u32(map, vtable);
    e.call(MAP_REMOVE_ALL, &args![map]);
    map_base_destroy(e, map, base_vtable);
}

/// A string template map's destructor: with the copy flag (+0x10), frees
/// the key of every node of every bucket; then the two lower levels.
fn string_map_destroy(e: &mut Engine, map: u32, vtables: [u32; 3]) {
    e.mem.set_u32(map, vtables[0]);
    if e.mem.u8(map + 0x10) != 0 {
        let mut bucket = 0;
        while bucket < e.mem.u32(map + 4) {
            let mut node = e.mem.u32(e.mem.u32(map + 8) + bucket * 4);
            while node != 0 {
                let following = e.mem.u32(node);
                let key = e.mem.u32(node + 4);
                e.call(FREE_BYTES, &args![key]);
                node = following;
            }
            bucket += 1;
        }
    }
    pointer_map_destroy(e, map, vtables[1], vtables[2]);
}

/// A map base level's constructor: vtable, bucket count, no items, and a
/// cleared bucket array of `4 * buckets` bytes.
fn map_base_construct(e: &mut Engine, map: u32, vtable: u32, bucket_count: u32) {
    e.mem.set_u32(map, vtable);
    e.mem.set_u32(map + 4, bucket_count);
    e.mem.set_u32(map + 0xc, 0);
    let size = e.mem.u32(map + 4) << 2;
    let buckets = e.call(ALLOC_BYTES, &args![size]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4) << 2;
    let buckets = e.mem.u32(map + 8);
    e.call(MEMSET, &args![buckets, 0u32, size]);
}

/// Frees the object when bit 0 of `flags` is set (deleting destructors).
fn delete_if(e: &mut Engine, this: u32, flags: u32) {
    if flags & 1 != 0 {
        deallocate(e, this);
    }
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 00a1c6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1c6f0 - C++ exception frame (FS:[0]) not translated
/// `Menu::InitSDM` (Xbox PDB): allocates `Menu::pTileptrStr`, an
/// `NiFixedString` of `"Tileptr"` (null when the allocation fails). The
/// branch for a null name (copying the empty string's handle) is compiled
/// as `MOV EAX,<name>; TEST EAX,EAX` and never runs; it is left out.
pub fn menu_init_sdm(e: &mut Engine) {
    let block = allocate(e, 4);
    let string = if block != 0 {
        let handle = e.call(NI_FIXED_STRING_CREATE, &args![TILEPTR_NAME]).u32();
        e.mem.set_u32(block, handle);
        block
    } else {
        0
    };
    e.set_global(TILE_PTR_STRING, string);
}

// Translated from 00a1c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees `Menu::pTileptrStr` (the counterpart of [`menu_init_sdm`]): its
/// handle is released unless it is the empty string's, then the block is
/// deleted. The global keeps its value.
pub fn fn_00a1c790(e: &mut Engine) {
    let string = e.global::<u32>(TILE_PTR_STRING);
    if string != 0 {
        let handle = e.mem.u32(string);
        release_fixed_string(e, handle);
        deallocate(e, string);
    }
}

// Translated from 00a1c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::FreeXMLArchive` (Xbox PDB): deletes every `XMLStorage` of
/// `Menu::XMLHash` (`XMLStorage::_scalar_deleting_destructor_(1)`) and
/// empties the map, then deletes every `Tile::BuildStorage` of
/// `Menu::XMLParsedHash` and empties that map.
pub fn menu_free_xml_archive(e: &mut Engine) {
    e.with_stack(12, |e, locals| {
        let (node, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = first_map_node(e, XML_HASH);
        e.mem.set_u32(node, first);
        while e.mem.u32(node) != 0 {
            e.call(MAP_NEXT, &args![XML_HASH, node, key, value]);
            let storage = e.mem.u32(value);
            if storage != 0 {
                e.call(
                    XML_STORAGE_SCALAR_DELETING_DESTRUCTOR,
                    &args![storage, 1u32],
                );
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![XML_HASH]);
    e.with_stack(12, |e, locals| {
        let (node, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = first_map_node(e, XML_PARSED_HASH);
        e.mem.set_u32(node, first);
        while e.mem.u32(node) != 0 {
            e.call(MAP_NEXT, &args![XML_PARSED_HASH, node, key, value]);
            let storage = e.mem.u32(value);
            delete_build_storage(e, storage);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![XML_PARSED_HASH]);
}

// Translated from 00a1c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::StripFileName` (Xbox PDB, `cdecl`): the part of `path` after its
/// last backslash, or `path` itself without one.
pub fn menu_strip_file_name(e: &mut Engine, path: Ptr) -> Ptr {
    let slash = e.call(STRRCHR, &args![path, 0x5cu32]).u32();
    if slash != 0 {
        Ptr::new(slash + 1)
    } else {
        path
    }
}

// Translated from 00a1c9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::GetXML` (Xbox PDB, `cdecl`): the `XMLStorage` of the file in
/// `Menu::XMLHash` (by its name without directories); when it is not there,
/// reads and compresses the file ([`menu_compress_and_store_xml`]) and, with
/// `store`, enters the result into the map.
pub fn menu_get_xml(e: &mut Engine, path: Ptr, store: bool) -> Ptr<XmlStorage> {
    let name = menu_strip_file_name(e, path);
    let mut storage = archive_lookup(e, XML_HASH, name.addr());
    if storage == 0 {
        storage = menu_compress_and_store_xml(e, path, Ptr::NULL).addr();
        if store {
            e.call(MAP_SET_AT, &args![XML_HASH, name, storage]);
        }
    }
    Ptr::new(storage)
}

// Translated from 00a1ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::RemoveFromXMLArchive` (Xbox PDB, `cdecl`): deletes the file's
/// `XMLStorage` and removes its entry from `Menu::XMLHash`, then does the
/// same with its `Tile::BuildStorage` in `Menu::XMLParsedHash`. When `menu`
/// is given and the storage was to delete its templates
/// (`bDeleteTemplates`), the menu takes that over: the storage's flag is
/// cleared and the menu's `bDeleteTemplates` set.
pub fn menu_remove_from_xml_archive(e: &mut Engine, path: Ptr, menu: Ptr<Menu>) {
    let name = menu_strip_file_name(e, path).addr();
    let storage = archive_lookup(e, XML_HASH, name);
    if storage != 0 {
        e.call(
            XML_STORAGE_SCALAR_DELETING_DESTRUCTOR,
            &args![storage, 1u32],
        );
    }
    fn_00a1e240(e, Ptr::new(XML_HASH), name);
    let parsed: Ptr<BuildStorage> = Ptr::new(archive_lookup(e, XML_PARSED_HASH, name));
    if !parsed.is_null() {
        if !menu.is_null() && e.get(parsed, BuildStorage::bDeleteTemplates) {
            e.set(parsed, BuildStorage::bDeleteTemplates, false);
            e.set(menu, Menu::bDeleteTemplates, true);
        }
        delete_build_storage(e, parsed.addr());
    }
    fn_00a1e240(e, Ptr::new(XML_PARSED_HASH), name);
}

// Translated from 00a1cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Tile::BuildStorage` of a name in `Menu::XMLParsedHash`, or null
/// (`cdecl`).
pub fn fn_00a1cb30(e: &mut Engine, name: Ptr) -> Ptr<BuildStorage> {
    Ptr::new(archive_lookup(e, XML_PARSED_HASH, name.addr()))
}

// Translated from 00a1cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enters a parsed file into `Menu::XMLParsedHash` under `name` (`cdecl`).
pub fn fn_00a1cb60(e: &mut Engine, name: Ptr, storage: Ptr<BuildStorage>) {
    e.call(MAP_SET_AT, &args![XML_PARSED_HASH, name, storage]);
}

// Translated from 00a1cb80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1cb80 - /GS stack cookie check (00ec408c) not translated
/// Loads `Data\Menus\master_menu_file.txt` into `Menu::XMLHash` and answers
/// the number of files (0 when the file is missing or not opened). The
/// file is the menu files one after another, each followed by `"@#$%"`;
/// a file's name is the word that ends in `.xml` (from the space or `-`
/// before it) in its text. Each file is compressed from the text
/// ([`menu_compress_and_store_xml`]) and entered under its name without
/// directories. The name buffer is 256 bytes on the game's stack, a
/// separate block here.
pub fn fn_00a1cb80(e: &mut Engine) -> i32 {
    let file = e
        .call(
            FILE_FINDER_GET_FILE_OV2,
            &args![MASTER_MENU_FILE, 0u32, 0x4000u32, 4u32],
        )
        .u32();
    if file == 0 || e.mem.u8(file + FILE_OPENED) == 0 {
        return 0;
    }
    let size = e
        .vcall(file, FILE_SLOT_SIZE, &args![])
        .u32()
        .wrapping_add(1);
    let buffer = allocate(e, size);
    let mut count = 0i32;
    let read = file_transfer(e, file, FILE_READ_FUNCTION, buffer, size);
    delete_file(e, file);
    let mut text = buffer;
    let mut separator = e.call(STRSTR, &args![text, FILE_SEPARATOR]).u32();
    let name_size = 0x100u32;
    while separator != 0 {
        e.with_stack(name_size, |e, name| {
            e.call(MEMSET, &args![name, 0u32, name_size]);
            let mut start = e.call(STRSTR, &args![text, XML_EXTENSION]).u32();
            let end = start.wrapping_add(4);
            loop {
                start = start.wrapping_sub(1);
                let c = e.mem.u8(start);
                if c == b' ' || c == b'-' {
                    break;
                }
            }
            start = start.wrapping_add(1);
            let length = end.wrapping_sub(start);
            e.call(MEMCPY, &args![name, start, length]);
            e.mem.set_u8(separator, 0);
            let storage = menu_compress_and_store_xml(e, name, Ptr::new(text));
            let key = menu_strip_file_name(e, name);
            e.call(MAP_SET_AT, &args![XML_HASH, key, storage]);
        });
        count = count.wrapping_add(1);
        text = separator.wrapping_add(4);
        separator = 0;
        if (text.wrapping_sub(buffer) as i32) < read as i32 {
            separator = e.call(STRSTR, &args![text, FILE_SEPARATOR]).u32();
        }
    }
    deallocate(e, buffer);
    count
}

// Translated from 00a1ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::CompressAndStoreXML` (Xbox PDB, `cdecl`): a new `XMLStorage` of
/// the menu XML with its insignificant characters removed. The text is
/// `text` (up to its terminator) or, without it, the file `path` read whole
/// (null when the file is missing or not opened). Tabs and line breaks are
/// dropped, runs of spaces and spaces after `>` or before `<` / `>` are
/// removed, `" />"` becomes `"/>"`, and `<!-- ... -->` comments are cut
/// out. After 100000 characters on a line, a line break is put after the
/// next closing tag (`</...>`). The output buffer has the size of the input
/// and is not grown for those line breaks.
pub fn menu_compress_and_store_xml(e: &mut Engine, path: Ptr, text: Ptr) -> Ptr<XmlStorage> {
    let (source, size, owns_text) = if text.is_null() {
        let file = e
            .call(
                FILE_FINDER_GET_FILE_OV2,
                &args![path, 0u32, 0x4000u32, 0x100u32],
            )
            .u32();
        if file == 0 || e.mem.u8(file + FILE_OPENED) == 0 {
            delete_file(e, file);
            return Ptr::NULL;
        }
        let size = e.vcall(file, FILE_SLOT_SIZE, &args![]).u32();
        let source = allocate(e, size);
        file_transfer(e, file, FILE_READ_FUNCTION, source, size);
        delete_file(e, file);
        (source, size, true)
    } else {
        let size = e.call(STRLEN, &args![text]).u32();
        (text.addr(), size, false)
    };

    let output = allocate(e, size);
    let mut since_line_break: u32 = 0;
    let mut in_closing_tag = false;
    let mut break_line = false;
    let mut in_comment = false;
    let mut length: i32 = 0;
    let mut last: u8 = 0;
    let mut second_last: u8 = 0;
    let mut third_last: u8 = 0;
    let mut index: u32 = 0;
    while index < size && e.mem.u8(source.wrapping_add(index)) != 0 {
        let c = e.mem.u8(source.wrapping_add(index));
        index += 1;
        if c == b'\t' || c == b'\n' || c == b'\r' {
            continue;
        }
        if in_comment {
            if second_last == b'-' && last == b'-' && c == b'>' {
                in_comment = false;
            }
            third_last = second_last;
            second_last = last;
            last = c;
            continue;
        }
        if c == b' ' {
            if last == b' ' || last == b'>' {
                continue;
            }
        } else if last == b' ' && (c == b'<' || c == b'>') {
            length = length.wrapping_sub(1);
        } else if second_last == b' ' && last == b'/' && c == b'>' {
            length = length.wrapping_sub(1);
            let at = output.wrapping_add(length as u32).wrapping_sub(1);
            e.mem.set_u8(at, b'/');
        } else if third_last == b'<' && second_last == b'!' && last == b'-' && c == b'-' {
            last = 0;
            second_last = 0;
            third_last = 0;
            length = length.wrapping_sub(3);
            in_comment = true;
            continue;
        }
        if since_line_break > 100_000 {
            if in_closing_tag && c == b'>' {
                break_line = true;
            } else if last == b'<' && c == b'/' {
                in_closing_tag = true;
            }
        }
        e.mem.set_u8(output.wrapping_add(length as u32), c);
        length = length.wrapping_add(1);
        third_last = second_last;
        second_last = last;
        last = c;
        since_line_break = since_line_break.wrapping_add(1);
        if break_line {
            in_closing_tag = false;
            break_line = false;
            e.mem.set_u8(output.wrapping_add(length as u32), b'\n');
            length = length.wrapping_add(1);
            since_line_break = 0;
        }
    }
    e.mem.set_u8(output.wrapping_add(length as u32), 0);
    if owns_text {
        deallocate(e, source);
    }
    let storage: Ptr<XmlStorage> = Ptr::new(allocate(e, 8));
    if storage.is_null() {
        return Ptr::NULL;
    }
    e.set(storage, XmlStorage::iFileSize, length as u32);
    e.set(storage, XmlStorage::pXMLData, Ptr::new(output));
    storage
}

// Translated from 00a1d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the packed `DATA\FINAL_MASTER_XML.DAT` into `Menu::XMLParsedHash`
/// and answers whether its entries were read. The file is a 12-byte header
/// (version 100, entry count, size of the entries) and the entries: a
/// 0x80-byte name, a size word and that many bytes of XML, each parsed
/// with `Tile::ParseFile(name, xml, size)` and entered under the name
/// ([`fn_00a1cb60`]). Afterwards the pool of unused template items is
/// freed. A file with another header is left unread (answer false).
pub fn fn_00a1d280(e: &mut Engine) -> bool {
    let mut loaded = false;
    let file = e
        .call(FILE_FINDER_GET_FILE, &args![FINAL_MASTER_XML, 0u32, 0u32])
        .u32();
    if file == 0 {
        return false;
    }
    e.vcall(file, FILE_SLOT_REWIND, &args![0u32, 0u32]);
    e.with_stack(12, |e, header| {
        let read = e.call(BS_FILE_READ, &args![file, header, 12u32]).u32();
        if read != 12 || e.mem.u32(header.addr()) != FINAL_MASTER_VERSION {
            return;
        }
        let count = e.mem.u32(header.addr() + 4);
        let size = e.mem.u32(header.addr() + 8);
        let data = allocate(e, size);
        let read = e.call(BS_FILE_READ, &args![file, data, size]).u32();
        if read != e.mem.u32(header.addr() + 8) {
            deallocate(e, data);
            return;
        }
        loaded = true;
        let mut entry = data;
        let mut index = 0;
        while index < count {
            let length = e.mem.u32(entry + 0x80);
            let storage = e
                .call(
                    TILE_PARSE_FILE,
                    &args![entry, entry + PACKED_ENTRY_HEADER, length],
                )
                .u32();
            fn_00a1cb60(e, Ptr::new(entry), Ptr::new(storage));
            let length = e.mem.u32(entry + 0x80);
            entry = entry.wrapping_add(length).wrapping_add(PACKED_ENTRY_HEADER);
            index += 1;
        }
        e.call(TEMPLATE_ITEM_POOL_FREE, &args![]);
        deallocate(e, data);
    });
    delete_file(e, file);
    loaded
}

// Translated from 00a1d410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1d410 - C++ exception frame (FS:[0]) and /GS stack cookie check (00ec408c) not translated
/// `Menu::CreateFinalMasterXMLFile` (Xbox PDB): writes every file of
/// `Menu::XMLHash` into `D:\Data\final_master_xml.dat` in the format
/// [`fn_00a1d280`] reads, and answers whether every write was complete.
/// Each file first has its `<include>` tags expanded
/// (`Tile::InsertIncludedFiles`) and is reported with `Error("Added %s of
/// size %i to FinalMasterXML\n")`; the storages and names are collected in
/// two stack arrays (an `NiTPrimitiveArray<XMLStorage *>` and an
/// `NiTPrimitiveArray<char const *>`) whose buffers are freed at the end.
/// The writing stops at the first incomplete write. The file object is
/// used without a null check after its allocation, as in the game.
pub fn menu_create_final_master_xml_file(e: &mut Engine) -> bool {
    e.with_stack(12, |e, header| {
        e.with_stack(0x10, |e, storages| {
            e.with_stack(0x10, |e, names| {
                write_final_master(e, header.addr(), storages.addr(), names.addr())
            })
        })
    })
}

/// The body of [`menu_create_final_master_xml_file`] with its stack
/// objects: the header (version, count, total size) and the two arrays.
fn write_final_master(e: &mut Engine, header: u32, storages: u32, names: u32) -> bool {
    let mut complete = true;
    e.mem.set_u32(header, FINAL_MASTER_VERSION);
    e.mem.set_u32(header + 8, 0);
    e.mem.set_u32(header + 4, 0);
    fn_00a1e450(e, Ptr::new(storages), 0, 1);
    e.mem.set_u32(storages, VTABLE_XML_PRIMITIVE_ARRAY);
    e.call(STRING_ARRAY_CONSTRUCT, &args![names, 0u32, 1u32]);
    e.mem.set_u32(names, VTABLE_STRING_PRIMITIVE_ARRAY);

    e.with_stack(12, |e, locals| {
        let (node, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = first_map_node(e, XML_HASH);
        e.mem.set_u32(node, first);
        while e.mem.u32(node) != 0 {
            e.call(MAP_NEXT, &args![XML_HASH, node, key, value]);
            let storage = e.mem.u32(value);
            e.call(TILE_INSERT_INCLUDED_FILES, &args![storage]);
            let index = e.mem.u16(storages + 0xa) as u32;
            fn_00a1e4d0(e, Ptr::new(storages), index, value);
            e.call(STRING_ARRAY_ADD, &args![names, key]);
            let name = e.mem.u32(key);
            let storage = e.mem.u32(value);
            let file_size = e.mem.u32(storage);
            e.with_stack(0x104, |e, message| {
                e.call(
                    FORMAT,
                    &args![message, 0x104u32, ADDED_FORMAT, name, file_size],
                );
                e.call(ERROR, &args![message]);
            });
            let storage = e.mem.u32(value);
            let file_size = e.mem.u32(storage);
            let total = e.mem.u32(header + 8);
            e.mem.set_u32(
                header + 8,
                total
                    .wrapping_add(file_size)
                    .wrapping_add(PACKED_ENTRY_HEADER),
            );
            let count = e.mem.u32(header + 4);
            e.mem.set_u32(header + 4, count.wrapping_add(1));
        }
    });

    let block = allocate(e, 0x158);
    let file = if block != 0 {
        e.call(
            BS_FILE_CONSTRUCT,
            &args![block, FINAL_MASTER_XML_OUT, 1u32, 0x4000u32, 0u32],
        )
        .u32()
    } else {
        0
    };
    let written = file_transfer(e, file, FILE_WRITE_FUNCTION, header, 12);
    if written == 12 {
        let mut index = 0u32;
        while index < e.mem.u32(header + 4) {
            let storage = e.mem.u32(e.mem.u32(storages + 4) + index * 4);
            let name = e.mem.u32(e.mem.u32(names + 4) + index * 4);
            e.with_stack(0x80, |e, name_field| {
                e.call(MEMSET, &args![name_field, 0u32, 0x80u32]);
                e.call(STRCPY_S, &args![name_field, 0x80u32, name]);
                let done = file_transfer(e, file, FILE_WRITE_FUNCTION, name_field.addr(), 0x80);
                if done != 0x80 {
                    complete = false;
                }
            });
            let done = file_transfer(e, file, FILE_WRITE_FUNCTION, storage, 4);
            if done != 4 {
                complete = false;
            }
            let file_size = e.mem.u32(storage);
            let data = e.mem.u32(storage + 4);
            let done = file_transfer(e, file, FILE_WRITE_FUNCTION, data, file_size);
            if done != e.mem.u32(storage) {
                complete = false;
            }
            if !complete {
                break;
            }
            index += 1;
        }
    } else {
        complete = false;
    }
    delete_file(e, file);

    e.mem.set_u32(names, VTABLE_STRING_ARRAY);
    let buffer = e.mem.u32(names + 4);
    e.call(FREE_BYTES, &args![buffer]);
    e.mem.set_u32(storages, VTABLE_XML_ARRAY);
    let buffer = e.mem.u32(storages + 4);
    e.call(FREE_BYTES, &args![buffer]);
    complete
}

// Translated from 00a1d910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::StartFadeOut` (Xbox PDB): when the menu tile's trait `0xfa3` is
/// true, starts the interface timer for the menu with the tile's fade time
/// (trait `0xfc1`, or `0xfc2` when that is exactly 0.0), sets
/// `FADING_OUT`, pops the menu from the enter stack when it holds an id and
/// its trait `0x1772` is true, and updates all timers.
pub fn menu_start_fade_out(e: &mut Engine, this: Ptr<Menu>) {
    let tile = menu_tile(e, this);
    if !e.call(TILE_IS_TRUE, &args![tile, TRAIT_VISIBLE]).bool() {
        return;
    }
    let tile = menu_tile(e, this).addr();
    let mut time = trait_float(e, tile, TRAIT_FADE_TIME) as f32;
    if x87_quiet(time) as f64 == e.global::<f64>(ZERO) {
        let tile = menu_tile(e, this).addr();
        time = trait_float(e, tile, TRAIT_FADE_TIME_DEFAULT) as f32;
    }
    e.call(INTERFACE_NEW_TIMER, &args![this, x87_quiet(time)]);
    e.set(this, Menu::xFadeState, FADING_OUT);
    pop_enter_stack_if_needed(e, this);
    e.call(INTERFACE_UPDATE_ALL_TIMERS, &args![]);
}

// Translated from 00a1d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::InstantFadeOut` (Xbox PDB): unless the menu is `FADE_HIDDEN`, sets
/// the tile's trait `0xfa3` to 0.0, fades its model
/// (`Interface::RecursiveFade(node, 1.0, 0.0)`), sets the state to
/// `FADING_OUT` (2, as the code writes), pops the enter stack as
/// [`menu_start_fade_out`] does and clears the menu's timer.
pub fn menu_instant_fade_out(e: &mut Engine, this: Ptr<Menu>) {
    if e.get(this, Menu::xFadeState) == FADE_HIDDEN {
        return;
    }
    let tile = menu_tile(e, this).addr();
    set_trait(e, tile, TRAIT_VISIBLE, 0.0);
    fade_model(e, this);
    e.set(this, Menu::xFadeState, FADING_OUT);
    pop_enter_stack_if_needed(e, this);
    e.call(INTERFACE_CLEAR_TIMER, &args![this]);
}

// Translated from 00a1daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends a running fade-out (`FADING_OUT`): the tile's trait `0xfa3` becomes
/// 1.0, its model is faded (`RecursiveFade(node, 1.0, 0.0)`), the state
/// becomes `FADE_SHOWN` and the menu's timer is cleared.
pub fn fn_00a1daa0(e: &mut Engine, this: Ptr<Menu>) {
    if e.get(this, Menu::xFadeState) != FADING_OUT {
        return;
    }
    let tile = menu_tile(e, this).addr();
    set_trait(e, tile, TRAIT_VISIBLE, 1.0);
    fade_model(e, this);
    e.set(this, Menu::xFadeState, FADE_SHOWN);
    e.call(INTERFACE_CLEAR_TIMER, &args![this]);
}

// Translated from 00a1db20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::StartFadeIn` (Xbox PDB): starts the interface timer with the fade
/// time (trait `0xfc1` of the menu tile, 0.25 without a tile; trait `0xfc2`
/// when that is exactly 0.0, read from the tile even when there is none),
/// sets `FADING_IN` and updates all timers.
pub fn menu_start_fade_in(e: &mut Engine, this: Ptr<Menu>) {
    let first = if !menu_tile(e, this).is_null() {
        let tile = menu_tile(e, this).addr();
        trait_float(e, tile, TRAIT_FADE_TIME) as f32
    } else {
        x87_quiet(e.global::<f32>(DEFAULT_FADE_IN_TIME))
    };
    let mut time = x87_quiet(first);
    if x87_quiet(time) as f64 == e.global::<f64>(ZERO) {
        let tile = menu_tile(e, this).addr();
        time = trait_float(e, tile, TRAIT_FADE_TIME_DEFAULT) as f32;
    }
    e.call(INTERFACE_NEW_TIMER, &args![this, x87_quiet(time)]);
    e.set(this, Menu::xFadeState, FADING_IN);
    e.call(INTERFACE_UPDATE_ALL_TIMERS, &args![]);
}

// Translated from 00a1dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends a running fade-in (`FADING_IN`): the tile's trait `0xfa3` becomes
/// 0.0, its model is faded (`RecursiveFade(node, 1.0, 0.0)`), the state
/// becomes `FADE_HIDDEN` and the menu's timer is cleared.
pub fn fn_00a1dba0(e: &mut Engine, this: Ptr<Menu>) {
    if e.get(this, Menu::xFadeState) != FADING_IN {
        return;
    }
    let tile = menu_tile(e, this).addr();
    set_trait(e, tile, TRAIT_VISIBLE, 0.0);
    fade_model(e, this);
    e.set(this, Menu::xFadeState, FADE_HIDDEN);
    e.call(INTERFACE_CLEAR_TIMER, &args![this]);
}

// Translated from 00a1dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::PrepForVisibility` (Xbox PDB): sets the tile's trait `0xfa3` to
/// 0.0, starts the fade-in unless `skip_fade`, clears the focus target and
/// recomputes the menus' depth ([`menu_get_max_depth`], answer dropped).
pub fn menu_prep_for_visibility(e: &mut Engine, this: Ptr<Menu>, skip_fade: bool) {
    let tile = menu_tile(e, this).addr();
    set_trait(e, tile, TRAIT_VISIBLE, 0.0);
    if !skip_fade {
        menu_start_fade_in(e, this);
    }
    e.call(INTERFACE_SET_CURRENT_FOCUS_TARGET, &args![0u32]);
    menu_get_max_depth(e);
}

// Translated from 00a1dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::SetMenuTile` (Xbox PDB): stores the tile. When its trait `0xfa7`
/// is one of 102, 6006, 103 or 6007 (each compared after a new read; not a
/// number matches none), the menu's class (vtable slot +0x34) goes to +0x14
/// and, unless `skip_enter_stack`, onto the enter stack
/// (`AddToEnterStack(class, 0)`); then, when the menu is not the
/// interface's word at +0xD0, the tile in its +0xCC (if any) gets trait
/// `0xfc3` = 0.0 and both words are cleared.
pub fn menu_set_menu_tile(
    e: &mut Engine,
    this: Ptr<Menu>,
    tile: Ptr<Tile>,
    skip_enter_stack: bool,
) {
    e.set(this, Menu::pMenu, tile);
    if tile.is_null() {
        return;
    }
    let mut matched = false;
    for constant in MENU_TYPE_VALUES {
        let value = trait_float(e, tile.addr(), TRAIT_MENU_TYPE);
        if value == e.global::<f64>(constant) {
            matched = true;
            break;
        }
    }
    if !matched {
        return;
    }
    let class = e.vcall(this.addr(), MENU_SLOT_GET_CLASS, &args![]).i32();
    e.set(this, Menu::iIsModal, class);
    if !skip_enter_stack {
        let class = e.vcall(this.addr(), MENU_SLOT_GET_CLASS, &args![]).u32();
        e.call(INTERFACE_ADD_TO_ENTER_STACK, &args![class, 0u32]);
    }
    if e.call(INTERFACE_GET_D0, &args![]).u32() != this.addr() {
        if e.call(INTERFACE_GET_CC, &args![]).u32() != 0 {
            let focus = e.call(INTERFACE_GET_CC, &args![]).u32();
            set_trait(e, focus, TRAIT_FOCUS, 0.0);
        }
        e.call(INTERFACE_SET_CC, &args![0u32]);
        e.call(INTERFACE_SET_D0, &args![0u32]);
    }
}

// Translated from 00a1dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a template to the menu's `xTemplates` (`AddHead(&template)`); a
/// null template is ignored.
pub fn fn_00a1dd80(e: &mut Engine, this: Ptr<Menu>, template: Ptr<TileTemplate>) {
    if template.is_null() {
        return;
    }
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), template.addr());
        let list = this.addr() + 8;
        e.call(SIMPLE_LIST_ADD_HEAD, &args![list, item]);
    });
}

// Translated from 00a1ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1ddb0 - C++ exception frame (FS:[0]) not translated
/// `Menu::RenderTemplate` (Xbox PDB): builds the tile tree of the menu's
/// template named `name` under `parent` and answers its root (null when
/// nothing was built). A non-null `last_tile` becomes `pLastTile`. The name
/// becomes an `NiFixedString` (the empty string's handle for null) and is
/// compared with each template's name (same handle, else `strcmp`); the
/// walk keeps the item of the last node it read, so without a match the
/// last node's template is built. The build runs with a new
/// `Tile::BuildStorage` in [`RENDER_BUILD_STORAGE`] holding the template
/// (`SetTemplate`): `Tile::BuildAndNameTree` on `parent`, then
/// `ConnectTraitsToTree` on the result; the storage's template is cleared
/// before the storage is deleted and the previous global restored.
pub fn menu_render_template(
    e: &mut Engine,
    this: Ptr<Menu>,
    parent: Ptr<Tile>,
    name: Ptr,
    last_tile: Ptr<Tile>,
) -> Ptr<Tile> {
    let mut root = 0u32;
    if !last_tile.is_null() {
        e.set(this, Menu::pLastTile, last_tile);
    }
    let mut template = 0u32;
    let mut node = this.addr() + 8;
    let handle = if name.is_null() {
        e.global::<u32>(EMPTY_FIXED_STRING)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![name]).u32()
    };
    while node != 0 {
        template = e.mem.u32(node);
        if template != 0 {
            let template_name = e.mem.u32(template);
            let equal = if handle == template_name {
                true
            } else if template_name == 0 || handle == 0 {
                false
            } else {
                e.call(STRCMP, &args![handle, template_name]).i32() == 0
            };
            if equal {
                break;
            }
        }
        node = e.mem.u32(node + 4);
    }
    if template != 0 {
        let previous = e.global::<u32>(RENDER_BUILD_STORAGE);
        let block = allocate(e, 0x14);
        let storage = if block != 0 {
            e.call(BUILD_STORAGE_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.set_global(RENDER_BUILD_STORAGE, storage);
        let current = e.global::<u32>(RENDER_BUILD_STORAGE);
        build_storage_set_template(e, Ptr::new(current), Ptr::new(template));
        root = e
            .call(TILE_BUILD_AND_NAME_TREE, &args![parent, template])
            .u32();
        e.call(TILE_CONNECT_TRAITS_TO_TREE, &args![root, template]);
        let current = e.global::<u32>(RENDER_BUILD_STORAGE);
        e.mem.set_u32(current, 0);
        let current = e.global::<u32>(RENDER_BUILD_STORAGE);
        delete_build_storage(e, current);
        e.set_global(RENDER_BUILD_STORAGE, previous);
    }
    release_fixed_string(e, handle);
    Ptr::new(root)
}

// Translated from 00a1dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::GetMaxDepth` (Xbox PDB): the greatest depth of the open menus
/// plus 2.0. Each child tile of the menus root that has a `Menu` gives its
/// trait `0xfad` plus the menu's `iMenuThickness` (a candidate that is not
/// less than the maximum, or not a number, replaces it). The truncated
/// maximum, at least 10, plus 100 is then given to the tile `007070a0`
/// answers (if any): its trait `0xfad`, and -0.008 times it as the second
/// of the three floats at +0x58 of its model (+0x2C).
pub fn menu_get_max_depth(e: &mut Engine) -> f32 {
    let mut max_depth = 0.0f32;
    let root = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).u32();
    let mut node = e.mem.u32(root + 4);
    while node != 0 {
        let item = node + 8;
        node = e.mem.u32(node);
        let tile = e.mem.u32(item);
        if tile == 0 || e.call(TILE_GET_MENU, &args![tile]).u32() == 0 {
            continue;
        }
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        let thickness = e.mem.i32(menu + 0x18);
        let depth = trait_float(e, tile, TRAIT_DEPTH);
        let candidate = (depth + thickness as f64) as f32;
        max_depth = if candidate < max_depth {
            x87_quiet(max_depth)
        } else {
            x87_quiet(candidate)
        };
    }
    let whole = e.call(FTOL, &args![max_depth as f64]).i32() as f32;
    let minimum = e.global::<f32>(MIN_DEPTH);
    let at_least = if whole < minimum {
        x87_quiet(minimum)
    } else {
        x87_quiet(whole)
    };
    let depth = (at_least as f64 + e.global::<f64>(DEPTH_OFFSET)) as f32;
    if e.call(INTERFACE_DEPTH_TILE, &args![]).u32() != 0 {
        let tile = e.call(INTERFACE_DEPTH_TILE, &args![]).u32();
        set_trait(e, tile, TRAIT_DEPTH, x87_quiet(depth));
        let tile = e.call(INTERFACE_DEPTH_TILE, &args![]).u32();
        let model = e.mem.u32(tile + 0x2c);
        let x = e.mem.u32(model + 0x58);
        let z = e.mem.u32(model + 0x60);
        let y = (depth as f64 * e.global::<f64>(DEPTH_SCALE)) as f32;
        let model = e.mem.u32(tile + 0x2c);
        e.mem.set_u32(model + 0x58, x);
        e.mem.set_f32(model + 0x5c, y);
        e.mem.set_u32(model + 0x60, z);
    }
    (max_depth as f64 + e.global::<f64>(TWO)) as f32
}

// Translated from 00a1e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `tile` in `Menu::xMenuList` at `id - 1001` (`cdecl`) when that
/// index is not negative and below the array's capacity.
pub fn fn_00a1e130(e: &mut Engine, id: i32, tile: u32) {
    let index = id.wrapping_sub(0x3e9);
    if index < 0 || index >= e.mem.u16(MENU_LIST + 8) as i32 {
        return;
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), tile);
        fn_00a1e680(e, Ptr::new(MENU_LIST), index as u32, value.addr());
    });
}

// Translated from 00a1e170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::_scalar_deleting_destructor_` (Xbox PDB): `~Menu` and, with bit 0
/// of `flags`, frees the object.
pub fn menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr<Menu>, flags: u32) -> Ptr<Menu> {
    e.call(MENU_DESTROY, &args![this]);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTArray<XMLStorage *, NiTMallocInterface<...>>`
/// (vtable `01095524`): frees the element buffer.
pub fn fn_00a1e1a0(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), VTABLE_XML_ARRAY);
    let buffer = e.get(this, NiTArray::m_pBase);
    e.call(FREE_BYTES, &args![buffer]);
}

// Translated from 00a1e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::BuildStorage::SetTemplate` (Xbox PDB): deletes the current
/// template (`~TileTemplate`, then the block) and stores `template`.
pub fn build_storage_set_template(
    e: &mut Engine,
    this: Ptr<BuildStorage>,
    template: Ptr<TileTemplate>,
) {
    let current = e.get(this, BuildStorage::pTemplate);
    if !current.is_null() {
        e.call(TEMPLATE_DESTROY, &args![current]);
        deallocate(e, current.addr());
    }
    e.set(this, BuildStorage::pTemplate, template);
}

// Translated from 00a1e240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase::RemoveAt(key)` (the map has no name; the bucket and the key
/// test come from vtable slots +4 and +8): unlinks the matching node, gives
/// it to `ClearValue` (+0x10) and `DeleteItem` (+0x18), counts one item
/// less and answers true; false when the key is not there.
pub fn fn_00a1e240(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32) -> bool {
    let map = this.addr();
    let index = e.vcall(map, MAP_SLOT_HASH, &args![key]).u32();
    let slot = e.mem.u32(map + 8) + index * 4;
    let head = e.mem.u32(slot);
    if head == 0 {
        return false;
    }
    let head_key = e.mem.u32(head + 4);
    if e.vcall(map, MAP_SLOT_KEYS_EQUAL, &args![key, head_key])
        .bool()
    {
        let following = e.mem.u32(head);
        let slot = e.mem.u32(map + 8) + index * 4;
        e.mem.set_u32(slot, following);
        remove_node(e, map, head);
        return true;
    }
    let mut previous = head;
    let mut node = e.mem.u32(head);
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, MAP_SLOT_KEYS_EQUAL, &args![key, node_key])
            .bool()
        {
            break;
        }
        previous = node;
        node = e.mem.u32(node);
    }
    if node == 0 {
        return false;
    }
    let following = e.mem.u32(node);
    e.mem.set_u32(previous, following);
    remove_node(e, map, node);
    true
}

/// The release of an unlinked node in [`fn_00a1e240`].
fn remove_node(e: &mut Engine, map: u32, node: u32) {
    e.vcall(map, MAP_SLOT_CLEAR_VALUE, &args![node]);
    e.vcall(map, MAP_SLOT_DELETE_ITEM, &args![node]);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count.wrapping_sub(1));
}

// Translated from 00a1e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase::GetAt(key, &value)` (the map has no name): true and the
/// value in `*value_out` when a node of the key's bucket (vtable +4) matches
/// (vtable +8); false otherwise. Only `AL` is the answer.
pub fn fn_00a1e380(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value_out: u32) -> bool {
    let map = this.addr();
    let index = e.vcall(map, MAP_SLOT_HASH, &args![key]).u32();
    let mut node = e.mem.u32(e.mem.u32(map + 8) + index * 4);
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, MAP_SLOT_KEYS_EQUAL, &args![key, node_key])
            .bool()
        {
            let value = e.mem.u32(node + 8);
            e.mem.set_u32(value_out, value);
            return true;
        }
        node = e.mem.u32(node);
    }
    false
}

// Translated from 00a1e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<XMLStorage *, NiTMallocInterface<XMLStorage *>>::
/// _scalar_deleting_destructor_` (Xbox PDB): frees the element buffer and,
/// with bit 0 of `flags`, the object.
pub fn ni_t_array_xml_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    flags: u32,
) -> Ptr<NiTArray> {
    fn_00a1e1a0(e, this);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTArray<XMLStorage *, ...>` `(max_size, grow_by)` (the
/// map has no name): no elements, and a buffer of `4 * max_size` bytes
/// (none for 0). Answers `this`.
pub fn fn_00a1e450(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), VTABLE_XML_ARRAY);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let capacity = e.get(this, NiTArray::m_usMaxSize) as u32;
    if capacity == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let buffer = e.call(ALLOC_BYTES, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, buffer);
    }
    this
}

// Translated from 00a1e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetAtGrow(index, &element)` (the map has no name): grows the
/// array to `index + m_usGrowBy` when `index` is not below the capacity,
/// then [`fn_00a1e680`]. Answers `index`.
pub fn fn_00a1e4d0(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: u32) -> u32 {
    if index >= e.get(this, NiTArray::m_usMaxSize) as u32 {
        let grow_by = e.get(this, NiTArray::m_usGrowBy) as u32;
        fn_00a1e520(e, this, grow_by.wrapping_add(index));
    }
    fn_00a1e680(e, this, index, element);
    index
}

// Translated from 00a1e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetSize(size)` (the map has no name): nothing when `size` is
/// the capacity. Shrinking below the used size clears the elements from
/// `size` on (counting the non-null ones off) and cuts the used size. A
/// new buffer of `4 * size` bytes (none for 0) receives the used elements
/// and zeros; the old buffer is freed.
pub fn fn_00a1e520(e: &mut Engine, this: Ptr<NiTArray>, size: u32) {
    if size == e.get(this, NiTArray::m_usMaxSize) as u32 {
        return;
    }
    if size < e.get(this, NiTArray::m_usSize) as u32 {
        let mut index = size as u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let at = e.get(this, NiTArray::m_pBase) + index as u32 * 4;
            if e.mem.u32(at) != 0 {
                e.mem.set_u32(at, 0);
                let used = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, used.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, size as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, size as u16);
    if size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let capacity = e.get(this, NiTArray::m_usMaxSize) as u32;
        let buffer = e.call(ALLOC_BYTES, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, buffer);
        let mut index = 0u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let element = e.mem.u32(old + index as u32 * 4);
            let at = e.get(this, NiTArray::m_pBase) + index as u32 * 4;
            e.mem.set_u32(at, element);
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        while index < e.get(this, NiTArray::m_usMaxSize) {
            let at = e.get(this, NiTArray::m_pBase) + index as u32 * 4;
            e.mem.set_u32(at, 0);
            index = index.wrapping_add(1);
        }
    }
    e.call(FREE_BYTES, &args![old]);
}

// Translated from 00a1e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray::SetAt(index, &element)` (the map has no name): an index past
/// the used size makes it `index + 1`; the count of non-null elements
/// follows the change; the element is stored. The capacity is not checked.
pub fn fn_00a1e680(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: u32) {
    let new = e.mem.u32(element);
    if index >= e.get(this, NiTArray::m_usSize) as u32 {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if new != 0 {
            let used = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, used.wrapping_add(1));
        }
    } else {
        let at = e
            .get(this, NiTArray::m_pBase)
            .wrapping_add(index.wrapping_mul(4));
        if new != 0 {
            if e.mem.u32(at) == 0 {
                let used = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else if e.mem.u32(at) != 0 {
            let used = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, used.wrapping_sub(1));
        }
    }
    let at = e
        .get(this, NiTArray::m_pBase)
        .wrapping_add(index.wrapping_mul(4));
    let new = e.mem.u32(element);
    e.mem.set_u32(at, new);
}

// Translated from 00a1e720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap::SetValue(node, key, value)` of the archive maps (vtable
/// slot +0xC of the two lower levels): stores the key and the value in the
/// node.
pub fn fn_00a1e720(e: &mut Engine, _this: Ptr, node: u32, key: u32, value: u32) {
    e.mem.set_u32(node + 4, key);
    e.mem.set_u32(node + 8, value);
}

// Translated from 00a1e740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IsKeysEqual(a, b)` of the archive string maps (vtable slot +8): the two
/// names are equal ignoring case (`NiStricmp` is 0).
pub fn fn_00a1e740(e: &mut Engine, _this: Ptr, a: u32, b: u32) -> bool {
    e.call(NI_STRICMP, &args![a, b]).i32() == 0
}

// Translated from 00a1e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap::SetValue(node, key, value)` of the archive maps
/// (vtable slot +0xC): with the copy flag (+0x10) the node gets its own copy
/// of the key (`strlen + 1` bytes, `strcpy_s`), else the key pointer; then
/// the value.
pub fn fn_00a1e770(e: &mut Engine, this: Ptr, node: u32, key: u32, value: u32) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let size = e.call(STRLEN, &args![key]).u32().wrapping_add(1);
        let copy = e.call(ALLOC_BYTES, &args![size]).u32();
        e.mem.set_u32(node + 4, copy);
        let copy = e.mem.u32(node + 4);
        e.call(STRCPY_S, &args![copy, size, key]);
    } else {
        e.mem.set_u32(node + 4, key);
    }
    e.mem.set_u32(node + 8, value);
}

// Translated from 00a1e7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1e7e0 - C++ exception frame (FS:[0]) not translated
/// `NiTPointerMap<char const *, XMLStorage *>::_scalar_deleting_destructor_`
/// (Xbox PDB): empties the map under its own vtable and its base's, frees
/// the bucket array and, with bit 0 of `flags`, the object.
pub fn ni_t_pointer_map_xml_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    pointer_map_destroy(e, this.addr(), VTABLE_XML_POINTER_MAP, VTABLE_XML_MAP_BASE);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTMapBase<NiTPointerAllocator<unsigned int>, char
/// const *, XMLStorage *>` (the map has no name): sets its vtable, empties
/// the map and frees the bucket array.
pub fn fn_00a1e880(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    map_base_destroy(e, this.addr(), VTABLE_XML_MAP_BASE);
}

// Translated from 00a1e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const *,
/// XMLStorage *>::_scalar_deleting_destructor_` (Xbox PDB): the base
/// destructor ([`fn_00a1e880`]) and, with bit 0 of `flags`, frees the
/// object.
pub fn ni_t_map_base_xml_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    map_base_destroy(e, this.addr(), VTABLE_XML_MAP_BASE);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1e900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1e900 - C++ exception frame (FS:[0]) not translated
/// The destructor of `NiTStringTemplateMap<NiTPointerMap<char const *,
/// XMLStorage *>, XMLStorage *>` (the map has no name): with the copy flag
/// (+0x10) frees every node's key, then the destructors of the two lower
/// levels run (each empties the map under its vtable) and the bucket array
/// is freed.
pub fn fn_00a1e900(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_destroy(
        e,
        this.addr(),
        [
            VTABLE_XML_STRING_MAP,
            VTABLE_XML_POINTER_MAP,
            VTABLE_XML_MAP_BASE,
        ],
    );
}

// Translated from 00a1e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1e9f0 - C++ exception frame (FS:[0]) not translated
/// The destructor of `NiTPointerMap<char const *, XMLStorage *>` (the map
/// has no name): empties the map under its vtable, then the base
/// destructor.
pub fn fn_00a1e9f0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    pointer_map_destroy(e, this.addr(), VTABLE_XML_POINTER_MAP, VTABLE_XML_MAP_BASE);
}

// Translated from 00a1ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *, XMLStorage *>,
/// XMLStorage *>::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// ([`fn_00a1e900`]) and, with bit 0 of `flags`, frees the object.
pub fn ni_t_string_template_map_xml_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00a1e900(e, this);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base class `NiTMapBase<..., char const *,
/// XMLStorage *>` `(bucket_count)` (the map has no name): vtable, bucket
/// count, no items and a cleared bucket array. Answers `this`.
pub fn fn_00a1eaa0(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    bucket_count: u32,
) -> Ptr<NiTPointerMap> {
    map_base_construct(e, this.addr(), VTABLE_XML_MAP_BASE, bucket_count);
    this
}

// Translated from 00a1eb20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1eb20 - C++ exception frame (FS:[0]) not translated
/// `NiTPointerMap<char const *, Tile::BuildStorage *>::
/// _scalar_deleting_destructor_` (Xbox PDB): as
/// [`ni_t_pointer_map_xml_storage_scalar_deleting_destructor`] with this
/// map's vtables.
pub fn ni_t_pointer_map_build_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    pointer_map_destroy(
        e,
        this.addr(),
        VTABLE_BUILD_POINTER_MAP,
        VTABLE_BUILD_MAP_BASE,
    );
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1ebc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTMapBase<..., char const *, Tile::BuildStorage *>`
/// (the map has no name): as [`fn_00a1e880`] with this map's vtable.
pub fn fn_00a1ebc0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    map_base_destroy(e, this.addr(), VTABLE_BUILD_MAP_BASE);
}

// Translated from 00a1ebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const *,
/// Tile::BuildStorage *>::_scalar_deleting_destructor_` (Xbox PDB): the base
/// destructor ([`fn_00a1ebc0`]) and, with bit 0 of `flags`, frees the
/// object.
pub fn ni_t_map_base_build_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    map_base_destroy(e, this.addr(), VTABLE_BUILD_MAP_BASE);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *, Tile::BuildStorage *>,
/// Tile::BuildStorage *>::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor ([`fn_00a1ec70`]) and, with bit 0 of `flags`, frees the
/// object.
pub fn ni_t_string_template_map_build_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00a1ec70(e, this);
    delete_if(e, this.addr(), flags);
    this
}

// Translated from 00a1ec70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1ec70 - C++ exception frame (FS:[0]) not translated
/// The destructor of `NiTStringTemplateMap<NiTPointerMap<char const *,
/// Tile::BuildStorage *>, ...>` (the map has no name): as [`fn_00a1e900`]
/// with this map's vtables.
pub fn fn_00a1ec70(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_destroy(
        e,
        this.addr(),
        [
            VTABLE_BUILD_STRING_MAP,
            VTABLE_BUILD_POINTER_MAP,
            VTABLE_BUILD_MAP_BASE,
        ],
    );
}

// Translated from 00a1ed60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a1ed60 - C++ exception frame (FS:[0]) not translated
/// The destructor of `NiTPointerMap<char const *, Tile::BuildStorage *>`
/// (the map has no name): as [`fn_00a1e9f0`] with this map's vtables.
pub fn fn_00a1ed60(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    pointer_map_destroy(
        e,
        this.addr(),
        VTABLE_BUILD_POINTER_MAP,
        VTABLE_BUILD_MAP_BASE,
    );
}

// Translated from 00a1ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base class `NiTMapBase<..., char const *,
/// Tile::BuildStorage *>` `(bucket_count)` (the map has no name): as
/// [`fn_00a1eaa0`] with this map's vtable.
pub fn fn_00a1ede0(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    bucket_count: u32,
) -> Ptr<NiTPointerMap> {
    map_base_construct(e, this.addr(), VTABLE_BUILD_MAP_BASE, bucket_count);
    this
}

// Translated from 00a1ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<Tile *>::_scalar_deleting_destructor_` (Xbox PDB):
/// sets the vtable of `NiTArray<Tile *, NiTMallocInterface<Tile *>>`, frees
/// the element buffer and, with bit 0 of `flags`, the object.
pub fn ni_t_primitive_array_tile_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    flags: u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), VTABLE_TILE_ARRAY);
    let buffer = e.get(this, NiTArray::m_pBase);
    e.call(FREE_BYTES, &args![buffer]);
    delete_if(e, this.addr(), flags);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1c6f0, menu_init_sdm()),
        entry!(0x00a1c790, fn_00a1c790()),
        entry!(0x00a1c800, menu_free_xml_archive()),
        entry!(0x00a1c970, menu_strip_file_name(Ptr) -> Ptr),
        entry!(0x00a1c9b0, menu_get_xml(Ptr, bool) -> Ptr<XmlStorage>),
        entry!(0x00a1ca20, menu_remove_from_xml_archive(Ptr, Ptr<Menu>)),
        entry!(0x00a1cb30, fn_00a1cb30(Ptr) -> Ptr<BuildStorage>),
        entry!(0x00a1cb60, fn_00a1cb60(Ptr, Ptr<BuildStorage>)),
        entry!(0x00a1cb80, fn_00a1cb80() -> i32),
        entry!(0x00a1ce70, menu_compress_and_store_xml(Ptr, Ptr) -> Ptr<XmlStorage>),
        entry!(0x00a1d280, fn_00a1d280() -> bool),
        entry!(0x00a1d410, menu_create_final_master_xml_file() -> bool),
        entry!(0x00a1d910, menu_start_fade_out(Ptr<Menu>)),
        entry!(0x00a1d9e0, menu_instant_fade_out(Ptr<Menu>)),
        entry!(0x00a1daa0, fn_00a1daa0(Ptr<Menu>)),
        entry!(0x00a1db20, menu_start_fade_in(Ptr<Menu>)),
        entry!(0x00a1dba0, fn_00a1dba0(Ptr<Menu>)),
        entry!(0x00a1dc20, menu_prep_for_visibility(Ptr<Menu>, bool)),
        entry!(0x00a1dc70, menu_set_menu_tile(Ptr<Menu>, Ptr<Tile>, bool)),
        entry!(0x00a1dd80, fn_00a1dd80(Ptr<Menu>, Ptr<TileTemplate>)),
        entry!(
            0x00a1ddb0,
            menu_render_template(Ptr<Menu>, Ptr<Tile>, Ptr, Ptr<Tile>) -> Ptr<Tile>
        ),
        entry!(0x00a1dfb0, menu_get_max_depth() -> f32),
        entry!(0x00a1e130, fn_00a1e130(i32, u32)),
        entry!(
            0x00a1e170,
            menu_scalar_deleting_destructor(Ptr<Menu>, u32) -> Ptr<Menu>
        ),
        entry!(0x00a1e1a0, fn_00a1e1a0(Ptr<NiTArray>)),
        entry!(
            0x00a1e1d0,
            build_storage_set_template(Ptr<BuildStorage>, Ptr<TileTemplate>)
        ),
        entry!(0x00a1e240, fn_00a1e240(Ptr<NiTPointerMap>, u32) -> bool),
        entry!(
            0x00a1e380,
            fn_00a1e380(Ptr<NiTPointerMap>, u32, u32) -> bool
        ),
        entry!(
            0x00a1e400,
            ni_t_array_xml_storage_scalar_deleting_destructor(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x00a1e450,
            fn_00a1e450(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x00a1e4d0, fn_00a1e4d0(Ptr<NiTArray>, u32, u32) -> u32),
        entry!(0x00a1e520, fn_00a1e520(Ptr<NiTArray>, u32)),
        entry!(0x00a1e680, fn_00a1e680(Ptr<NiTArray>, u32, u32)),
        entry!(0x00a1e720, fn_00a1e720(Ptr, u32, u32, u32)),
        entry!(0x00a1e740, fn_00a1e740(Ptr, u32, u32) -> bool),
        entry!(0x00a1e770, fn_00a1e770(Ptr, u32, u32, u32)),
        entry!(
            0x00a1e7e0,
            ni_t_pointer_map_xml_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a1e880, fn_00a1e880(Ptr<NiTPointerMap>)),
        entry!(
            0x00a1e8b0,
            ni_t_map_base_xml_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a1e900, fn_00a1e900(Ptr<NiTPointerMap>)),
        entry!(0x00a1e9f0, fn_00a1e9f0(Ptr<NiTPointerMap>)),
        entry!(
            0x00a1ea70,
            ni_t_string_template_map_xml_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a1eaa0,
            fn_00a1eaa0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a1eb20,
            ni_t_pointer_map_build_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a1ebc0, fn_00a1ebc0(Ptr<NiTPointerMap>)),
        entry!(
            0x00a1ebf0,
            ni_t_map_base_build_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a1ec40,
            ni_t_string_template_map_build_storage_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a1ec70, fn_00a1ec70(Ptr<NiTPointerMap>)),
        entry!(0x00a1ed60, fn_00a1ed60(Ptr<NiTPointerMap>)),
        entry!(
            0x00a1ede0,
            fn_00a1ede0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a1ee60,
            ni_t_primitive_array_tile_scalar_deleting_destructor(
                Ptr<NiTArray>,
                u32,
            ) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Test hash map vtable: dtor, hash (always bucket 0), key test (byte
    /// strings equal), set value, clear value, new item, delete item.
    const TEST_MAP_VTABLE: u32 = 0x0300_0000;
    const TEST_HASH: u32 = 0x7000_1000;
    const TEST_EQUAL: u32 = 0x7000_1004;
    const TEST_CLEAR: u32 = 0x7000_1008;
    const TEST_DELETE: u32 = 0x7000_100c;
    /// Test file: vtable, delete, size, rewind, and its read and write
    /// function members.
    const TEST_FILE_VTABLE: u32 = 0x0300_0100;
    const TEST_FILE_DELETE: u32 = 0x7000_2000;
    const TEST_FILE_SIZE: u32 = 0x7000_2004;
    const TEST_FILE_REWIND: u32 = 0x7000_2008;
    const TEST_FILE_READ: u32 = 0x7000_200c;
    const TEST_FILE_WRITE: u32 = 0x7000_2010;
    const TEST_GET_CLASS: u32 = 0x7000_3000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(at, _)| *at == addr)
            .map(|(_, a)| a.clone())
            .collect()
    }

    fn string(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text.as_bytes());
        at
    }

    /// The engine with the data pages the unit reads, the constants, and
    /// doubles for the CRT, the Gamebryo allocator, the map iteration step
    /// and a test map and file vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0106_f000,
            0x0107_0000,
            0x0109_4000,
            0x0109_5000,
            0x0109_b000,
            0x011f_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(DEFAULT_FADE_IN_TIME, 0.25f32);
        for (at, value) in MENU_TYPE_VALUES
            .iter()
            .zip([102.0f64, 6006.0, 103.0, 6007.0])
        {
            e.set_global(*at, value);
        }
        e.set_global(MIN_DEPTH, 10.0f32);
        e.set_global(DEPTH_OFFSET, 100.0f64);
        e.set_global(DEPTH_SCALE, f64::from(-0.008f32));
        e.register(ALLOC_BYTES, |e, a| ret(e.mem.alloc(a[0])));
        stub(&mut e, FREE_BYTES);
        e.register(STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(STRSTR, |e, a| {
            let hay = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let at = hay
                .windows(needle.len())
                .position(|w| w == needle.as_slice());
            ret(at.map_or(0, |p| a[0] + p as u32))
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|c| *c as u32 == a[1]);
            ret(at.map_or(0, |p| a[0] + p as u32))
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(0)
        });
        e.register(STRCMP, |e, a| {
            ret((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        e.register(MAP_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let key = e.mem.u32(node + 4);
            e.mem.set_u32(a[2], key);
            let value = e.mem.u32(node + 8);
            e.mem.set_u32(a[3], value);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        e.put_vtable(
            TEST_MAP_VTABLE,
            &[0, TEST_HASH, TEST_EQUAL, 0, TEST_CLEAR, 0, TEST_DELETE],
        );
        e.register(TEST_HASH, |_, _| ret(0));
        e.register(TEST_EQUAL, |e, a| {
            ret((e.mem.cstr(a[1]) == e.mem.cstr(a[2])) as u32)
        });
        stub(&mut e, TEST_CLEAR);
        stub(&mut e, TEST_DELETE);
        let mut file_slots = [0u32; 11];
        file_slots[0] = TEST_FILE_DELETE;
        file_slots[8] = TEST_FILE_REWIND;
        file_slots[10] = TEST_FILE_SIZE;
        e.put_vtable(TEST_FILE_VTABLE, &file_slots);
        stub(&mut e, TEST_FILE_DELETE);
        stub(&mut e, TEST_FILE_REWIND);
        e.call_log = Some(vec![]);
        e
    }

    /// An empty one-bucket test map at `map`.
    fn make_map(e: &mut Engine, map: u32) {
        e.mem.set_u32(map, TEST_MAP_VTABLE);
        e.mem.set_u32(map + 4, 1);
        let buckets = e.mem.alloc(4);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(map + 0xc, 0);
    }

    /// Adds a node at the head of the test map's bucket; answers the node.
    fn add_node(e: &mut Engine, map: u32, key: &str, value: u32) -> u32 {
        let node = e.mem.alloc(12);
        let key = string(e, key);
        e.mem.set_u32(node + 4, key);
        e.mem.set_u32(node + 8, value);
        let buckets = e.mem.u32(map + 8);
        let head = e.mem.u32(buckets);
        e.mem.set_u32(node, head);
        e.mem.set_u32(buckets, node);
        let count = e.mem.u32(map + 0xc);
        e.mem.set_u32(map + 0xc, count + 1);
        node
    }

    /// A test file holding `content`, opened unless `opened` is false.
    fn make_file(e: &mut Engine, content: &[u8], opened: bool) -> u32 {
        let file = e.mem.alloc(0x158);
        e.mem.set_u32(file, TEST_FILE_VTABLE);
        e.mem.set_u32(file + FILE_READ_FUNCTION, TEST_FILE_READ);
        e.mem.set_u32(file + FILE_WRITE_FUNCTION, TEST_FILE_WRITE);
        e.mem.set_u8(file + FILE_OPENED, opened as u8);
        let length = content.len() as u32;
        e.register_double(TEST_FILE_SIZE, move |_, _| ret(length));
        let bytes = content.to_vec();
        e.register_double(TEST_FILE_READ, move |e, a| {
            let n = a[2].min(bytes.len() as u32);
            e.mem.write(a[1], &bytes[..n as usize]);
            ret(n)
        });
        file
    }

    /// The bytes of an `XMLStorage`.
    fn storage(e: &Engine, storage: u32) -> Vec<u8> {
        let size = e.mem.u32(storage);
        e.mem.bytes(e.mem.u32(storage + 4), size)
    }

    /// A menu with a tile that has a model.
    fn new_menu(e: &mut Engine) -> (Ptr<Menu>, u32) {
        let menu: Ptr<Menu> = e.new_object();
        let tile = e.mem.alloc(0x40);
        let model = e.mem.alloc(0x70);
        e.mem.set_u32(tile + 0x2c, model);
        e.set(menu, Menu::pMenu, Ptr::new(tile));
        (menu, tile)
    }

    /// Doubles for the tile trait functions (`IsTrue` answers true for
    /// `true_traits`, the float getter answers from `floats`) and stubs for
    /// the interface calls of the fades.
    fn tile_traits(e: &mut Engine, true_traits: &'static [u32], floats: &'static [(u32, f32)]) {
        e.register_double(TILE_IS_TRUE, move |_, a| {
            ret(true_traits.contains(&a[1]) as u32)
        });
        e.register_double(TILE_GET_FLOAT, move |_, a| {
            let value = floats.iter().find(|(t, _)| *t == a[1]).map_or(0.0, |v| v.1);
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        for addr in [
            TILE_SET_FLOAT,
            INTERFACE_NEW_TIMER,
            INTERFACE_UPDATE_ALL_TIMERS,
            INTERFACE_CLEAR_TIMER,
            INTERFACE_POP_FROM_ENTER_STACK,
            INTERFACE_SET_CURRENT_FOCUS_TARGET,
            INTERFACE_RECURSIVE_FADE,
            INTERFACE_ADD_TO_ENTER_STACK,
            INTERFACE_SET_CC,
            INTERFACE_SET_D0,
        ] {
            stub(e, addr);
        }
    }

    #[test]
    fn init_sdm_allocates_the_tileptr_string() {
        let mut e = engine();
        e.register(NI_FIXED_STRING_CREATE, |_, _| ret(0x1234));
        e.call(0x00a1_c6f0, &[]);
        let block = e.global::<u32>(TILE_PTR_STRING);
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), 0x1234);
        assert_eq!(calls(&e, NI_FIXED_STRING_CREATE), vec![vec![TILEPTR_NAME]]);
    }

    #[test]
    fn free_tileptr_string_releases_a_counted_handle() {
        let mut e = engine();
        stub(&mut e, INTERLOCKED_DECREMENT);
        let block = e.mem.alloc(4);
        e.mem.set_u32(block, 0x5008);
        e.set_global(TILE_PTR_STRING, block);
        e.call(0x00a1_c790, &[]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x5000]]);
        assert_eq!(e.mem.block_size(block), None);
        // The empty string's handle is not released.
        let block = e.mem.alloc(4);
        e.set_global(EMPTY_FIXED_STRING, 0x7008u32);
        e.mem.set_u32(block, 0x7008);
        e.set_global(TILE_PTR_STRING, block);
        e.call_log = Some(vec![]);
        e.call(0x00a1_c790, &[]);
        assert!(calls(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn free_xml_archive_deletes_every_storage() {
        let mut e = engine();
        make_map(&mut e, XML_HASH);
        make_map(&mut e, XML_PARSED_HASH);
        add_node(&mut e, XML_HASH, "a.xml", 0x100);
        add_node(&mut e, XML_HASH, "b.xml", 0x200);
        let parsed = e.mem.alloc(0x14);
        add_node(&mut e, XML_PARSED_HASH, "a.xml", parsed);
        stub(&mut e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR);
        stub(&mut e, BUILD_STORAGE_DESTROY);
        stub(&mut e, MAP_REMOVE_ALL);
        e.call(0x00a1_c800, &[]);
        assert_eq!(
            calls(&e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![0x200, 1], vec![0x100, 1]]
        );
        assert_eq!(calls(&e, BUILD_STORAGE_DESTROY), vec![vec![parsed]]);
        assert_eq!(e.mem.block_size(parsed), None);
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL),
            vec![vec![XML_HASH], vec![XML_PARSED_HASH]]
        );
    }

    #[test]
    fn strip_file_name_keeps_the_part_after_the_last_backslash() {
        let mut e = engine();
        let path = string(&mut e, "Data\\Menus\\a.xml");
        assert_eq!(e.call(0x00a1_c970, &[path]).u32(), path + 11);
        let name = string(&mut e, "a.xml");
        assert_eq!(e.call(0x00a1_c970, &[name]).u32(), name);
    }

    #[test]
    fn get_xml_answers_the_stored_file_or_reads_it() {
        let mut e = engine();
        make_map(&mut e, XML_HASH);
        add_node(&mut e, XML_HASH, "a.xml", 0x100);
        stub(&mut e, MAP_SET_AT);
        let path = string(&mut e, "Menus\\a.xml");
        assert_eq!(e.call(0x00a1_c9b0, &[path, 1]).u32(), 0x100);
        assert!(calls(&e, MAP_SET_AT).is_empty());
        // Not stored: the file is read (missing here), and stored when asked.
        e.register(FILE_FINDER_GET_FILE_OV2, |_, _| ret(0));
        let path = string(&mut e, "Menus\\b.xml");
        assert_eq!(e.call(0x00a1_c9b0, &[path, 1]).u32(), 0);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![XML_HASH, path + 6, 0]]);
        assert_eq!(calls(&e, FILE_FINDER_GET_FILE_OV2)[0][0], path);
    }

    #[test]
    fn remove_from_xml_archive_hands_the_templates_to_the_menu() {
        let mut e = engine();
        make_map(&mut e, XML_HASH);
        make_map(&mut e, XML_PARSED_HASH);
        add_node(&mut e, XML_HASH, "a.xml", 0x100);
        let parsed = e.mem.alloc(0x14);
        e.mem.set_u8(parsed + 0x10, 1);
        add_node(&mut e, XML_PARSED_HASH, "a.xml", parsed);
        stub(&mut e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR);
        let flags = Rc::new(RefCell::new(vec![]));
        let seen = flags.clone();
        e.register_double(BUILD_STORAGE_DESTROY, move |e, a| {
            seen.borrow_mut().push(e.mem.u8(a[0] + 0x10));
            Ret::default()
        });
        let menu: Ptr<Menu> = e.new_object();
        let path = string(&mut e, "x\\a.xml");
        e.call(0x00a1_ca20, &args![path, menu]);
        assert_eq!(
            calls(&e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![0x100, 1]]
        );
        assert_eq!(*flags.borrow(), vec![0]);
        assert!(e.get(menu, Menu::bDeleteTemplates));
        assert_eq!(e.mem.u32(XML_HASH + 0xc), 0);
        assert_eq!(e.mem.u32(XML_PARSED_HASH + 0xc), 0);
    }

    #[test]
    fn parsed_hash_lookup() {
        let mut e = engine();
        make_map(&mut e, XML_PARSED_HASH);
        add_node(&mut e, XML_PARSED_HASH, "a.xml", 0x300);
        let a = string(&mut e, "a.xml");
        let b = string(&mut e, "b.xml");
        assert_eq!(e.call(0x00a1_cb30, &[a]).u32(), 0x300);
        assert_eq!(e.call(0x00a1_cb30, &[b]).u32(), 0);
    }

    #[test]
    fn parsed_hash_insert() {
        let mut e = engine();
        stub(&mut e, MAP_SET_AT);
        e.call(0x00a1_cb60, &[0x1000, 0x400]);
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![XML_PARSED_HASH, 0x1000, 0x400]]
        );
    }

    #[test]
    fn master_menu_file_enters_each_file() {
        let mut e = engine();
        // The two strings the code searches for (in `.rdata`).
        e.mem.set_cstr(FILE_SEPARATOR, b"@#$%");
        e.mem.set_cstr(XML_EXTENSION, b".xml");
        let file = make_file(&mut e, b"<a/>\n menus\\x.xml@#$%<b/> -y.xml@#$%", true);
        e.register_double(FILE_FINDER_GET_FILE_OV2, move |_, _| ret(file));
        let entered = Rc::new(RefCell::new(vec![]));
        let seen = entered.clone();
        e.register_double(MAP_SET_AT, move |e, a| {
            let key = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            seen.borrow_mut().push((key, storage(e, a[2])));
            Ret::default()
        });
        assert_eq!(e.call(0x00a1_cb80, &[]).i32(), 2);
        {
            let entered = entered.borrow();
            assert_eq!(entered[0].0, "x.xml");
            assert_eq!(entered[0].1, b"<a/>menus\\x.xml".to_vec());
            assert_eq!(entered[1].0, "y.xml");
            assert_eq!(entered[1].1, b"<b/>-y.xml".to_vec());
        }
        assert_eq!(calls(&e, TEST_FILE_DELETE), vec![vec![file, 1]]);
        assert_eq!(
            calls(&e, FILE_FINDER_GET_FILE_OV2),
            vec![vec![MASTER_MENU_FILE, 0, 0x4000, 4]]
        );
        // A file that is not opened: nothing read.
        let closed = make_file(&mut e, b"", false);
        e.register_double(FILE_FINDER_GET_FILE_OV2, move |_, _| ret(closed));
        assert_eq!(e.call(0x00a1_cb80, &[]).i32(), 0);
    }

    #[test]
    fn compress_removes_whitespace_and_comments() {
        let mut e = engine();
        let text = string(&mut e, "<a  x=\"1\" />\n\t<!-- c --><b >");
        let result = e.call(0x00a1_ce70, &[0, text]).u32();
        assert_eq!(storage(&e, result), b"<a x=\"1\"/><b>".to_vec());
        assert_eq!(e.mem.u8(e.mem.u32(result + 4) + 13), 0);
    }

    #[test]
    fn compress_breaks_long_lines_after_a_closing_tag() {
        let mut e = engine();
        let mut text = b"<a>".to_vec();
        text.resize(3 + 100_000, b'x');
        text.extend_from_slice(b"</a><b>");
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, &text);
        let result = e.call(0x00a1_ce70, &[0, at]).u32();
        let out = storage(&e, result);
        assert_eq!(out.len(), 3 + 100_000 + 4 + 1 + 3);
        assert_eq!(&out[100_003..], b"</a>\n<b>");
    }

    #[test]
    fn compress_reads_the_file_when_no_text_is_given() {
        let mut e = engine();
        let file = make_file(&mut e, b"<a>\r\n</a>", true);
        e.register_double(FILE_FINDER_GET_FILE_OV2, move |_, _| ret(file));
        let path = string(&mut e, "a.xml");
        let result = e.call(0x00a1_ce70, &[path, 0]).u32();
        assert_eq!(storage(&e, result), b"<a></a>".to_vec());
        assert_eq!(
            calls(&e, FILE_FINDER_GET_FILE_OV2),
            vec![vec![path, 0, 0x4000, 0x100]]
        );
        assert_eq!(e.mem.u32(file + 4), 9);
        // A file that is not opened gives null and is deleted.
        let closed = make_file(&mut e, b"", false);
        e.register_double(FILE_FINDER_GET_FILE_OV2, move |_, _| ret(closed));
        assert_eq!(e.call(0x00a1_ce70, &[path, 0]).u32(), 0);
        assert!(calls(&e, TEST_FILE_DELETE).contains(&vec![closed, 1]));
    }

    /// The packed file: header and two entries.
    fn packed_entries() -> (Vec<u8>, Vec<u8>) {
        let mut data = vec![];
        for (name, xml) in [
            ("a.xml", b"<a/>".as_slice()),
            ("b.xml", b"<bb/>".as_slice()),
        ] {
            let mut field = name.as_bytes().to_vec();
            field.resize(0x80, 0);
            data.extend(field);
            data.extend((xml.len() as u32).to_le_bytes());
            data.extend_from_slice(xml);
        }
        let mut header = vec![];
        for word in [100u32, 2, data.len() as u32] {
            header.extend(word.to_le_bytes());
        }
        (header, data)
    }

    #[test]
    fn final_master_file_is_parsed_into_the_archive() {
        let mut e = engine();
        let file = make_file(&mut e, b"", true);
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| ret(file));
        let (header, data) = packed_entries();
        let mut reads = vec![header, data].into_iter();
        e.register_double(BS_FILE_READ, move |e, a| {
            let bytes = reads.next().unwrap();
            e.mem.write(a[1], &bytes);
            ret(bytes.len() as u32)
        });
        e.register(TILE_PARSE_FILE, |e, a| ret(0x9000 + e.mem.u32(a[0] + 0x80)));
        stub(&mut e, MAP_SET_AT);
        stub(&mut e, TEMPLATE_ITEM_POOL_FREE);
        assert!(e.call(0x00a1_d280, &[]).bool());
        let parses = calls(&e, TILE_PARSE_FILE);
        assert_eq!(parses.len(), 2);
        assert_eq!(parses[0][1], parses[0][0] + 0x84);
        assert_eq!(parses[1][0], parses[0][0] + 0x88);
        assert_eq!(parses[1][2], 5);
        let sets = calls(&e, MAP_SET_AT);
        assert_eq!(sets[1], vec![XML_PARSED_HASH, parses[1][0], 0x9005]);
        assert_eq!(calls(&e, TEMPLATE_ITEM_POOL_FREE).len(), 1);
        assert_eq!(calls(&e, TEST_FILE_REWIND), vec![vec![file, 0, 0]]);
        assert_eq!(calls(&e, TEST_FILE_DELETE), vec![vec![file, 1]]);
    }

    #[test]
    fn final_master_file_with_another_version_is_not_read() {
        let mut e = engine();
        let file = make_file(&mut e, b"", true);
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| ret(file));
        e.register(BS_FILE_READ, |e, a| {
            e.mem.set_u32(a[1], 99);
            ret(12)
        });
        stub(&mut e, TILE_PARSE_FILE);
        assert!(!e.call(0x00a1_d280, &[]).bool());
        assert!(calls(&e, TILE_PARSE_FILE).is_empty());
        assert_eq!(calls(&e, TEST_FILE_DELETE), vec![vec![file, 1]]);
    }

    #[test]
    fn create_final_master_file_writes_every_entry() {
        let mut e = engine();
        make_map(&mut e, XML_HASH);
        let data = string(&mut e, "abc");
        let xml = e.mem.alloc(8);
        e.mem.set_u32(xml, 3);
        e.mem.set_u32(xml + 4, data);
        add_node(&mut e, XML_HASH, "a.xml", xml);
        stub(&mut e, TILE_INSERT_INCLUDED_FILES);
        stub(&mut e, FORMAT);
        stub(&mut e, ERROR);
        e.register(STRING_ARRAY_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 4, buffer);
            ret(a[0])
        });
        e.register(STRING_ARRAY_ADD, |e, a| {
            let index = e.mem.u16(a[0] + 0xa) as u32;
            let item = e.mem.u32(a[1]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + index * 4, item);
            e.mem.set_u16(a[0] + 0xa, index as u16 + 1);
            Ret::default()
        });
        e.register(BS_FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_FILE_VTABLE);
            e.mem.set_u32(a[0] + FILE_WRITE_FUNCTION, TEST_FILE_WRITE);
            ret(a[0])
        });
        let written = Rc::new(RefCell::new(vec![]));
        let seen = written.clone();
        e.register_double(TEST_FILE_WRITE, move |e, a| {
            seen.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            ret(a[2])
        });
        assert!(e.call(0x00a1_d410, &[]).bool());
        {
            let written = written.borrow();
            assert_eq!(written.len(), 4);
            let mut header = vec![];
            for word in [100u32, 1, 0x87] {
                header.extend(word.to_le_bytes());
            }
            assert_eq!(written[0], header);
            assert_eq!(&written[1][..6], b"a.xml\0");
            assert_eq!(written[2], 3u32.to_le_bytes().to_vec());
            assert_eq!(written[3], b"abc".to_vec());
        }
        let format = &calls(&e, FORMAT)[0];
        assert_eq!(format[1..3], [0x104, ADDED_FORMAT]);
        assert_eq!(format[4], 3);
        assert_eq!(calls(&e, TILE_INSERT_INCLUDED_FILES), vec![vec![xml]]);
        // The empty first buffer of the storage array (grown from 0), then the two
        // arrays' buffers.
        assert_eq!(calls(&e, FREE_BYTES).len(), 3);
        let file = calls(&e, BS_FILE_CONSTRUCT)[0].clone();
        assert_eq!(file[1..], [FINAL_MASTER_XML_OUT, 1, 0x4000, 0]);
        assert_eq!(calls(&e, TEST_FILE_DELETE), vec![vec![file[0], 1]]);
    }

    #[test]
    fn start_fade_out_uses_the_default_time_when_zero() {
        let mut e = engine();
        let (menu, tile) = new_menu(&mut e);
        tile_traits(
            &mut e,
            &[TRAIT_VISIBLE, TRAIT_POPS_ENTER_STACK],
            &[(TRAIT_FADE_TIME_DEFAULT, 0.5)],
        );
        e.set(menu, Menu::iIsModal, 1004);
        e.call(0x00a1_d910, &args![menu]);
        assert_eq!(
            calls(&e, INTERFACE_NEW_TIMER),
            vec![vec![menu.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(e.get(menu, Menu::xFadeState), FADING_OUT);
        assert_eq!(
            calls(&e, INTERFACE_POP_FROM_ENTER_STACK),
            vec![vec![1004, 0]]
        );
        assert_eq!(calls(&e, INTERFACE_SET_CURRENT_FOCUS_TARGET), vec![vec![0]]);
        assert_eq!(calls(&e, INTERFACE_UPDATE_ALL_TIMERS).len(), 1);
        assert_eq!(calls(&e, TILE_GET_FLOAT)[0], vec![tile, TRAIT_FADE_TIME]);
        // Not visible: nothing happens.
        let (hidden, _) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[]);
        e.call_log = Some(vec![]);
        e.call(0x00a1_d910, &args![hidden]);
        assert!(calls(&e, INTERFACE_NEW_TIMER).is_empty());
    }

    #[test]
    fn instant_fade_out_hides_the_tile() {
        let mut e = engine();
        let (menu, tile) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[]);
        e.set(menu, Menu::xFadeState, FADE_SHOWN);
        e.call(0x00a1_d9e0, &args![menu]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![tile, TRAIT_VISIBLE, 0, 1]]
        );
        let model = e.mem.u32(tile + 0x2c);
        assert_eq!(
            calls(&e, INTERFACE_RECURSIVE_FADE),
            vec![vec![model, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(e.get(menu, Menu::xFadeState), FADING_OUT);
        assert!(calls(&e, INTERFACE_POP_FROM_ENTER_STACK).is_empty());
        assert_eq!(calls(&e, INTERFACE_CLEAR_TIMER), vec![vec![menu.addr()]]);
        // Already hidden: nothing.
        e.set(menu, Menu::xFadeState, FADE_HIDDEN);
        e.call_log = Some(vec![]);
        e.call(0x00a1_d9e0, &args![menu]);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
    }

    #[test]
    fn end_of_fade_out_shows_the_menu() {
        let mut e = engine();
        let (menu, tile) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[]);
        e.set(menu, Menu::xFadeState, FADING_OUT);
        e.call(0x00a1_daa0, &args![menu]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![tile, TRAIT_VISIBLE, 1.0f32.to_bits(), 1]]
        );
        assert_eq!(e.get(menu, Menu::xFadeState), FADE_SHOWN);
        assert_eq!(calls(&e, INTERFACE_CLEAR_TIMER), vec![vec![menu.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x00a1_daa0, &args![menu]);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
    }

    #[test]
    fn start_fade_in_without_tile_takes_a_quarter_second() {
        let mut e = engine();
        tile_traits(&mut e, &[], &[(TRAIT_FADE_TIME, 0.75)]);
        let menu: Ptr<Menu> = e.new_object();
        e.call(0x00a1_db20, &args![menu]);
        assert_eq!(
            calls(&e, INTERFACE_NEW_TIMER),
            vec![vec![menu.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(e.get(menu, Menu::xFadeState), FADING_IN);
        assert_eq!(calls(&e, INTERFACE_UPDATE_ALL_TIMERS).len(), 1);
        let (with_tile, _) = new_menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a1_db20, &args![with_tile]);
        assert_eq!(calls(&e, INTERFACE_NEW_TIMER)[0][1], 0.75f32.to_bits());
    }

    #[test]
    fn end_of_fade_in_hides_the_tile() {
        let mut e = engine();
        let (menu, tile) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[]);
        e.set(menu, Menu::xFadeState, FADING_IN);
        e.call(0x00a1_dba0, &args![menu]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![tile, TRAIT_VISIBLE, 0, 1]]
        );
        assert_eq!(e.get(menu, Menu::xFadeState), FADE_HIDDEN);
        assert_eq!(calls(&e, INTERFACE_CLEAR_TIMER).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00a1_dba0, &args![menu]);
        assert!(calls(&e, INTERFACE_CLEAR_TIMER).is_empty());
    }

    /// Doubles for [`menu_get_max_depth`]: a menus root with two child
    /// tiles, the first with a `Menu` of thickness 3, and `depth_tile` as
    /// the interface's depth tile. Answers the first tile.
    fn depth_world(e: &mut Engine, depth_tile: u32) -> u32 {
        let root = e.mem.alloc(0x40);
        let first = e.mem.alloc(12);
        let second = e.mem.alloc(12);
        let tile_with_menu = e.mem.alloc(0x40);
        let tile_without = e.mem.alloc(0x40);
        e.mem.set_u32(root + 4, first);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, tile_with_menu);
        e.mem.set_u32(second + 8, tile_without);
        let menu = e.mem.alloc(0x28);
        e.mem.set_i32(menu + 0x18, 3);
        e.register_double(INTERFACE_GET_MENUS_ROOT, move |_, _| ret(root));
        e.register_double(TILE_GET_MENU, move |_, a| {
            ret(if a[0] == tile_with_menu { menu } else { 0 })
        });
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register_double(INTERFACE_DEPTH_TILE, move |_, _| ret(depth_tile));
        tile_with_menu
    }

    #[test]
    fn prep_for_visibility_fades_in_unless_told_not_to() {
        let mut e = engine();
        let (menu, tile) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[(TRAIT_FADE_TIME, 1.0)]);
        depth_world(&mut e, 0);
        e.call(0x00a1_dc20, &args![menu, true]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![tile, TRAIT_VISIBLE, 0, 1]]
        );
        assert!(calls(&e, INTERFACE_NEW_TIMER).is_empty());
        assert_eq!(calls(&e, INTERFACE_SET_CURRENT_FOCUS_TARGET), vec![vec![0]]);
        assert_eq!(calls(&e, INTERFACE_GET_MENUS_ROOT).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00a1_dc20, &args![menu, false]);
        assert_eq!(calls(&e, INTERFACE_NEW_TIMER).len(), 1);
        assert_eq!(e.get(menu, Menu::xFadeState), FADING_IN);
    }

    #[test]
    fn set_menu_tile_enters_menus_of_the_listed_types() {
        let mut e = engine();
        let (menu, _) = new_menu(&mut e);
        tile_traits(&mut e, &[], &[(TRAIT_MENU_TYPE, 103.0)]);
        let vtable = 0x0300_0200;
        let mut slots = [0u32; 14];
        slots[(MENU_SLOT_GET_CLASS / 4) as usize] = TEST_GET_CLASS;
        e.put_vtable(vtable, &slots);
        e.register(TEST_GET_CLASS, |_, _| ret(1004));
        e.mem.set_u32(menu.addr(), vtable);
        let focus = e.mem.alloc(0x40);
        e.register(INTERFACE_GET_D0, |_, _| ret(0));
        e.register_double(INTERFACE_GET_CC, move |_, _| ret(focus));
        let tile = e.mem.alloc(0x40);
        e.call(0x00a1_dc70, &args![menu, tile, false]);
        assert_eq!(e.get(menu, Menu::pMenu).addr(), tile);
        assert_eq!(e.get(menu, Menu::iIsModal), 1004);
        assert_eq!(calls(&e, INTERFACE_ADD_TO_ENTER_STACK), vec![vec![1004, 0]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![focus, TRAIT_FOCUS, 0, 1]]
        );
        assert_eq!(calls(&e, INTERFACE_SET_CC), vec![vec![0]]);
        assert_eq!(calls(&e, INTERFACE_SET_D0), vec![vec![0]]);
        assert_eq!(calls(&e, TILE_GET_FLOAT).len(), 3);
        // Another type: only the tile is stored.
        tile_traits(&mut e, &[], &[(TRAIT_MENU_TYPE, 5.0)]);
        e.call_log = Some(vec![]);
        e.call(0x00a1_dc70, &args![menu, tile, true]);
        assert_eq!(calls(&e, TILE_GET_FLOAT).len(), 4);
        assert!(calls(&e, TEST_GET_CLASS).is_empty());
        // No tile: nothing read.
        e.call_log = Some(vec![]);
        e.call(0x00a1_dc70, &args![menu, 0u32, true]);
        assert!(e.get(menu, Menu::pMenu).is_null());
        assert!(calls(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn add_template_puts_it_at_the_head_of_the_list() {
        let mut e = engine();
        let menu: Ptr<Menu> = e.new_object();
        let items = Rc::new(RefCell::new(vec![]));
        let seen = items.clone();
        e.register_double(SIMPLE_LIST_ADD_HEAD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x00a1_dd80, &args![menu, 0u32]);
        assert!(items.borrow().is_empty());
        e.call(0x00a1_dd80, &args![menu, 0x4444u32]);
        assert_eq!(*items.borrow(), vec![(menu.addr() + 8, 0x4444)]);
    }

    #[test]
    fn render_template_builds_the_named_template() {
        let mut e = engine();
        let menu: Ptr<Menu> = e.new_object();
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        let first_name = string(&mut e, "other");
        let second_name = string(&mut e, "main");
        e.mem.set_u32(first, first_name);
        e.mem.set_u32(second, second_name);
        let node = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 8, first);
        e.mem.set_u32(menu.addr() + 0xc, node);
        e.mem.set_u32(node, second);
        let handle = string(&mut e, "main");
        e.register_double(NI_FIXED_STRING_CREATE, move |_, _| ret(handle));
        e.register(BUILD_STORAGE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TILE_BUILD_AND_NAME_TREE, |e, _| {
            let storage = e.global::<u32>(RENDER_BUILD_STORAGE);
            ret(0x5000 + e.mem.u32(storage))
        });
        stub(&mut e, TILE_CONNECT_TRAITS_TO_TREE);
        let templates = Rc::new(RefCell::new(vec![]));
        let seen = templates.clone();
        e.register_double(BUILD_STORAGE_DESTROY, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        stub(&mut e, INTERLOCKED_DECREMENT);
        e.set_global(RENDER_BUILD_STORAGE, 0x6666u32);
        let parent = 0x7777u32;
        let name = string(&mut e, "main");
        let root = e
            .call(0x00a1_ddb0, &args![menu, parent, name, 0x8888u32])
            .u32();
        assert_eq!(root, 0x5000 + second);
        assert_eq!(
            calls(&e, TILE_BUILD_AND_NAME_TREE),
            vec![vec![parent, second]]
        );
        assert_eq!(
            calls(&e, TILE_CONNECT_TRAITS_TO_TREE),
            vec![vec![root, second]]
        );
        assert_eq!(*templates.borrow(), vec![0]);
        assert_eq!(e.global::<u32>(RENDER_BUILD_STORAGE), 0x6666);
        assert_eq!(e.get(menu, Menu::pLastTile).addr(), 0x8888);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![handle - 8]]);
        assert_eq!(calls(&e, NI_FIXED_STRING_CREATE), vec![vec![name]]);
    }

    #[test]
    fn get_max_depth_places_the_depth_tile_above_the_menus() {
        let mut e = engine();
        let depth_tile = e.mem.alloc(0x40);
        let model = e.mem.alloc(0x70);
        e.mem.set_u32(depth_tile + 0x2c, model);
        e.mem.set_f32(model + 0x58, 1.0);
        e.mem.set_f32(model + 0x60, 3.0);
        tile_traits(&mut e, &[], &[(TRAIT_DEPTH, 4.0)]);
        let tile_with_menu = depth_world(&mut e, depth_tile);
        let depth = e.call(0x00a1_dfb0, &[]).f32();
        // 4.0 + thickness 3 = 7; at least 10, plus 100 for the depth tile.
        assert_eq!(depth, 9.0);
        assert_eq!(
            calls(&e, TILE_GET_FLOAT),
            vec![vec![tile_with_menu, TRAIT_DEPTH]]
        );
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![depth_tile, TRAIT_DEPTH, 110.0f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.f32(model + 0x58), 1.0);
        assert_eq!(
            e.mem.f32(model + 0x5c),
            (110.0f64 * f64::from(-0.008f32)) as f32
        );
        assert_eq!(e.mem.f32(model + 0x60), 3.0);
    }

    #[test]
    fn menu_list_store_checks_the_index() {
        let mut e = engine();
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(MENU_LIST + 4, buffer);
        e.mem.set_u16(MENU_LIST + 8, 4);
        e.call(0x00a1_e130, &args![1002i32, 0x1111u32]);
        assert_eq!(e.mem.u32(buffer + 4), 0x1111);
        assert_eq!(e.mem.u16(MENU_LIST + 0xa), 2);
        e.call(0x00a1_e130, &args![1000i32, 0x2222u32]);
        e.call(0x00a1_e130, &args![1005i32, 0x2222u32]);
        assert_eq!(e.mem.u16(MENU_LIST + 0xa), 2);
        assert_eq!(e.mem.u16(MENU_LIST + 0xc), 1);
    }

    #[test]
    fn menu_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        stub(&mut e, MENU_DESTROY);
        let menu: Ptr<Menu> = e.new_object();
        e.call(0x00a1_e170, &args![menu, 0u32]);
        assert!(e.mem.block_size(menu.addr()).is_some());
        assert_eq!(e.call(0x00a1_e170, &args![menu, 1u32]).u32(), menu.addr());
        assert_eq!(e.mem.block_size(menu.addr()), None);
        assert_eq!(calls(&e, MENU_DESTROY).len(), 2);
    }

    #[test]
    fn xml_array_destructor_frees_the_buffer() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_pBase, 0x1230);
        e.call(0x00a1_e1a0, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_XML_ARRAY);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x1230]]);
        assert!(e.mem.block_size(array.addr()).is_some());
    }

    #[test]
    fn xml_array_deleting_destructor_frees_the_object() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_pBase, 0x1230);
        e.call(0x00a1_e400, &args![array, 0u32]);
        assert!(e.mem.block_size(array.addr()).is_some());
        e.call(0x00a1_e400, &args![array, 1u32]);
        assert_eq!(e.mem.block_size(array.addr()), None);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x1230], vec![0x1230]]);
    }

    #[test]
    fn set_template_deletes_the_previous_one() {
        let mut e = engine();
        stub(&mut e, TEMPLATE_DESTROY);
        let storage: Ptr<BuildStorage> = e.new_object();
        let old = e.mem.alloc(0x14);
        e.mem.set_u32(storage.addr(), old);
        e.call(0x00a1_e1d0, &args![storage, 0x4444u32]);
        assert_eq!(calls(&e, TEMPLATE_DESTROY), vec![vec![old]]);
        assert_eq!(e.mem.block_size(old), None);
        assert_eq!(e.mem.u32(storage.addr()), 0x4444);
        e.mem.set_u32(storage.addr(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00a1_e1d0, &args![storage, 0x5555u32]);
        assert!(calls(&e, TEMPLATE_DESTROY).is_empty());
    }

    #[test]
    fn remove_at_unlinks_head_and_chained_nodes() {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        make_map(&mut e, map);
        let a = add_node(&mut e, map, "a", 1);
        let b = add_node(&mut e, map, "b", 2);
        let c = add_node(&mut e, map, "c", 3);
        let key = string(&mut e, "b");
        assert!(e.call(0x00a1_e240, &[map, key]).bool());
        assert_eq!(e.mem.u32(c), a);
        assert_eq!(calls(&e, TEST_CLEAR), vec![vec![map, b]]);
        assert_eq!(calls(&e, TEST_DELETE), vec![vec![map, b]]);
        let key = string(&mut e, "c");
        assert!(e.call(0x00a1_e240, &[map, key]).bool());
        assert_eq!(e.mem.u32(e.mem.u32(map + 8)), a);
        let key = string(&mut e, "z");
        assert!(!e.call(0x00a1_e240, &[map, key]).bool());
        assert_eq!(e.mem.u32(map + 0xc), 1);
    }

    #[test]
    fn get_at_answers_the_value() {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        make_map(&mut e, map);
        add_node(&mut e, map, "a", 7);
        add_node(&mut e, map, "b", 8);
        let out = e.mem.alloc(4);
        let key = string(&mut e, "a");
        assert!(e.call(0x00a1_e380, &[map, key, out]).bool());
        assert_eq!(e.mem.u32(out), 7);
        let key = string(&mut e, "q");
        e.mem.set_u32(out, 99);
        assert!(!e.call(0x00a1_e380, &[map, key, out]).bool());
        assert_eq!(e.mem.u32(out), 99);
    }

    #[test]
    fn array_constructor_allocates_the_capacity() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.call(0x00a1_e450, &args![array, 4u32, 2u32]);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_XML_ARRAY);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 4);
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
        assert_eq!(calls(&e, ALLOC_BYTES), vec![vec![16]]);
        assert_ne!(e.get(array, NiTArray::m_pBase), 0);
        e.call(0x00a1_e450, &args![array, 0u32, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }

    #[test]
    fn set_at_grow_grows_past_the_capacity() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        fn_00a1e450(&mut e, array, 0, 2);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x99);
        assert_eq!(e.call(0x00a1_e4d0, &args![array, 1u32, value]).u32(), 1);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        let base = e.get(array, NiTArray::m_pBase);
        assert_eq!(e.mem.u32(base + 4), 0x99);
    }

    #[test]
    fn set_size_shrinks_and_copies() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        fn_00a1e450(&mut e, array, 4, 1);
        let value = e.mem.alloc(4);
        for (index, item) in [(0u32, 5u32), (1, 6), (3, 7)] {
            e.mem.set_u32(value, item);
            fn_00a1e680(&mut e, array, index, value);
        }
        let old = e.get(array, NiTArray::m_pBase);
        e.call(0x00a1_e520, &args![array, 2u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        let base = e.get(array, NiTArray::m_pBase);
        assert_ne!(base, old);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 4)), (5, 6));
        assert!(calls(&e, FREE_BYTES).contains(&vec![old]));
        e.call(0x00a1_e520, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }

    #[test]
    fn set_at_counts_the_non_null_elements() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        fn_00a1e450(&mut e, array, 4, 1);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 1);
        e.call(0x00a1_e680, &args![array, 2u32, value]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        e.call(0x00a1_e680, &args![array, 0u32, value]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        e.mem.set_u32(value, 0);
        e.call(0x00a1_e680, &args![array, 2u32, value]);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
    }

    #[test]
    fn pointer_map_set_value_stores_key_and_value() {
        let mut e = engine();
        let node = e.mem.alloc(12);
        e.call(0x00a1_e720, &[0, node, 0x11, 0x22]);
        assert_eq!((e.mem.u32(node + 4), e.mem.u32(node + 8)), (0x11, 0x22));
    }

    #[test]
    fn keys_equal_ignores_case() {
        let mut e = engine();
        e.register(NI_STRICMP, |e, a| {
            let first = e.mem.cstr(a[0]).to_ascii_lowercase();
            ret((first != e.mem.cstr(a[1]).to_ascii_lowercase()) as u32)
        });
        let a = string(&mut e, "Menu.XML");
        let b = string(&mut e, "menu.xml");
        let c = string(&mut e, "other");
        assert!(e.call(0x00a1_e740, &[0, a, b]).bool());
        assert!(!e.call(0x00a1_e740, &[0, a, c]).bool());
    }

    #[test]
    fn string_map_set_value_copies_the_key() {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        let node = e.mem.alloc(12);
        let key = string(&mut e, "a.xml");
        e.call(0x00a1_e770, &[map, node, key, 0x33]);
        assert_eq!(e.mem.u32(node + 4), key);
        e.mem.set_u8(map + 0x10, 1);
        e.call(0x00a1_e770, &[map, node, key, 0x44]);
        let copy = e.mem.u32(node + 4);
        assert_ne!(copy, key);
        assert_eq!(e.mem.cstr(copy), b"a.xml");
        assert_eq!(e.mem.u32(node + 8), 0x44);
        assert_eq!(calls(&e, ALLOC_BYTES), vec![vec![6]]);
    }

    /// Records the vtable each `RemoveAll` runs under.
    fn record_remove_all(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let inner = log.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            inner.borrow_mut().push(vec![e.mem.u32(a[0])]);
            Ret::default()
        });
        log
    }

    fn map_with_buckets(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x4560);
        map
    }

    /// A pointer map deleting destructor: both levels emptied, buckets and
    /// object freed.
    fn check_pointer_map_deleting_destructor(addr: u32, levels: [u32; 2]) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = map_with_buckets(&mut e);
        assert_eq!(e.call(addr, &[map, 1]).u32(), map);
        assert_eq!(*log.borrow(), vec![vec![levels[0]], vec![levels[1]]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x4560]]);
        assert_eq!(e.mem.block_size(map), None);
    }

    /// A pointer map destructor: both levels emptied, the object kept.
    fn check_pointer_map_destructor(addr: u32, levels: [u32; 2]) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = map_with_buckets(&mut e);
        e.call(addr, &[map]);
        assert_eq!(*log.borrow(), vec![vec![levels[0]], vec![levels[1]]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x4560]]);
        assert!(e.mem.block_size(map).is_some());
    }

    /// A map base destructor: emptied under the base vtable, buckets freed.
    fn check_map_base_destructor(addr: u32, vtable: u32) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = map_with_buckets(&mut e);
        e.call(addr, &[map]);
        assert_eq!(*log.borrow(), vec![vec![vtable]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x4560]]);
    }

    /// A map base deleting destructor: frees the object only with bit 0.
    fn check_map_base_deleting_destructor(addr: u32, vtable: u32) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = map_with_buckets(&mut e);
        e.call(addr, &[map, 0]);
        assert!(e.mem.block_size(map).is_some());
        assert_eq!(e.call(addr, &[map, 1]).u32(), map);
        assert_eq!(e.mem.block_size(map), None);
        assert_eq!(*log.borrow(), vec![vec![vtable], vec![vtable]]);
    }

    /// A string map destructor: copied keys freed, then the lower levels.
    fn check_string_map_destructor(addr: u32, levels: [u32; 3]) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = e.mem.alloc(0x14);
        make_map(&mut e, map);
        let node = add_node(&mut e, map, "a", 1);
        let key = e.mem.u32(node + 4);
        e.mem.set_u8(map + 0x10, 1);
        let buckets = e.mem.u32(map + 8);
        e.call(addr, &[map]);
        assert_eq!(e.mem.u32(map), levels[2]);
        assert_eq!(*log.borrow(), vec![vec![levels[1]], vec![levels[2]]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![key], vec![buckets]]);
        // Without the copy flag no key is freed.
        e.mem.set_u8(map + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(addr, &[map]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![buckets]]);
    }

    /// A string map deleting destructor: the destructor, then the object.
    fn check_string_map_deleting_destructor(addr: u32, levels: [u32; 3]) {
        let mut e = engine();
        let log = record_remove_all(&mut e);
        let map = map_with_buckets(&mut e);
        assert_eq!(e.call(addr, &[map, 1]).u32(), map);
        assert_eq!(*log.borrow(), vec![vec![levels[1]], vec![levels[2]]]);
        assert_eq!(e.mem.block_size(map), None);
    }

    /// A map base constructor.
    fn check_map_base_constructor(addr: u32, vtable: u32) {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 0xc, 9);
        assert_eq!(e.call(addr, &[map, 5]).u32(), map);
        assert_eq!(e.mem.u32(map), vtable);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(calls(&e, ALLOC_BYTES), vec![vec![20]]);
        let buckets = e.mem.u32(map + 8);
        assert_eq!(calls(&e, MEMSET), vec![vec![buckets, 0, 20]]);
    }

    const XML_LEVELS: [u32; 3] = [
        VTABLE_XML_STRING_MAP,
        VTABLE_XML_POINTER_MAP,
        VTABLE_XML_MAP_BASE,
    ];
    const BUILD_LEVELS: [u32; 3] = [
        VTABLE_BUILD_STRING_MAP,
        VTABLE_BUILD_POINTER_MAP,
        VTABLE_BUILD_MAP_BASE,
    ];

    #[test]
    fn xml_pointer_map_deleting_destructor() {
        check_pointer_map_deleting_destructor(0x00a1_e7e0, [XML_LEVELS[1], XML_LEVELS[2]]);
    }

    #[test]
    fn xml_map_base_destructor() {
        check_map_base_destructor(0x00a1_e880, VTABLE_XML_MAP_BASE);
    }

    #[test]
    fn xml_map_base_deleting_destructor() {
        check_map_base_deleting_destructor(0x00a1_e8b0, VTABLE_XML_MAP_BASE);
    }

    #[test]
    fn xml_string_map_destructor() {
        check_string_map_destructor(0x00a1_e900, XML_LEVELS);
    }

    #[test]
    fn xml_pointer_map_destructor() {
        check_pointer_map_destructor(0x00a1_e9f0, [XML_LEVELS[1], XML_LEVELS[2]]);
    }

    #[test]
    fn xml_string_map_deleting_destructor() {
        check_string_map_deleting_destructor(0x00a1_ea70, XML_LEVELS);
    }

    #[test]
    fn xml_map_base_constructor() {
        check_map_base_constructor(0x00a1_eaa0, VTABLE_XML_MAP_BASE);
    }

    #[test]
    fn build_pointer_map_deleting_destructor() {
        check_pointer_map_deleting_destructor(0x00a1_eb20, [BUILD_LEVELS[1], BUILD_LEVELS[2]]);
    }

    #[test]
    fn build_map_base_destructor() {
        check_map_base_destructor(0x00a1_ebc0, VTABLE_BUILD_MAP_BASE);
    }

    #[test]
    fn build_map_base_deleting_destructor() {
        check_map_base_deleting_destructor(0x00a1_ebf0, VTABLE_BUILD_MAP_BASE);
    }

    #[test]
    fn build_string_map_deleting_destructor() {
        check_string_map_deleting_destructor(0x00a1_ec40, BUILD_LEVELS);
    }

    #[test]
    fn build_string_map_destructor() {
        check_string_map_destructor(0x00a1_ec70, BUILD_LEVELS);
    }

    #[test]
    fn build_pointer_map_destructor() {
        check_pointer_map_destructor(0x00a1_ed60, [BUILD_LEVELS[1], BUILD_LEVELS[2]]);
    }

    #[test]
    fn build_map_base_constructor() {
        check_map_base_constructor(0x00a1_ede0, VTABLE_BUILD_MAP_BASE);
    }

    #[test]
    fn tile_array_deleting_destructor() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_pBase, 0x1240);
        e.call(0x00a1_ee60, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_TILE_ARRAY);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x1240]]);
        assert!(e.mem.block_size(array.addr()).is_some());
        e.call(0x00a1_ee60, &args![array, 1u32]);
        assert_eq!(e.mem.block_size(array.addr()), None);
    }
}

//! `fallout shared/teleportdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `DoorTeleportData`: a door's link to the door it leads to, with the
//! position and rotation the traveller arrives at.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `DoorTeleportData` (Xbox PDB), 0x20 bytes (the same on PC: the
    /// record's `XTEL` chunk is read straight into it).
    pub struct DoorTeleportData: 0x20 {
        /// `pLinkedDoor` (Xbox PDB): `TESObjectREFR*`; a form id until
        /// `InitItem` resolves it.
        0x00 pLinkedDoor: u32,
        /// `position` (Xbox PDB): `NiPoint3`, x at +4.
        0x04 position: f32,
        /// `rotation` (Xbox PDB): `NiPoint3`, x at +0x10.
        0x10 rotation: f32,
        /// `cFlags` (Xbox PDB).
        0x1c cFlags: u8,
    }
}

/// `006815c0` (`ECX` = object): run on each point before it is set.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// `00416870` (`ECX` = point; x, y, z floats; `RET 0xc`): sets a point and
/// returns it.
const POINT_SET: u32 = 0x0041_6870;
/// The float both points start at (`01016450`, read as the exe loads it).
const START_COORDINATE: u32 = 0x0101_6450;
/// `008d6f30` (`ECX` = reference): the reference's cell.
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetOwner` (Xbox PDB name of `00546a40`; `ECX` = cell).
const CELL_GET_OWNER: u32 = 0x0054_6a40;
/// `TESObjectCELL::SetTempPublic` (Xbox PDB name of `00544390`; `ECX` =
/// cell, one byte word; `RET 4`).
const CELL_SET_TEMP_PUBLIC: u32 = 0x0054_4390;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB name of `0054ddd0`; `ECX` = cell).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `00578870` (`ECX` = reference, one word: the string; `RET 4`).
const REFERENCE_MAP_NAME: u32 = 0x0057_8870;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `0055a400` (`ECX` = the stack copy of the data).
const DATA_PREPARE: u32 = 0x0055_a400;
/// `00403d30` (cdecl: destination, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`; `ECX` =
/// file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the data read needs its byte order
/// swapped.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00414030` (`ECX` = the stack copy): its byte swap.
const DATA_SWAP: u32 = 0x0041_4030;
/// The `XTEL` chunk tag.
const CHUNK_XTEL: u32 = 0x4c45_5458;
/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, one word:
/// an index).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; cdecl: address
/// of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, offset, source type,
/// target type, is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`01183028`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The `TESObjectREFR` type descriptor (`011841cc`).
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find linked door (%08X) in teleport data
/// init."` (`01016548`).
const FORMAT_NO_DOOR: u32 = 0x0101_6548;
/// `"MASTERFILE: Linked door (%08X) in teleport data points to invalid
/// object (%s %s(%08X))."` (`010164f0`).
const FORMAT_NOT_A_DOOR: u32 = 0x0101_64f0;
/// The warnings for a position and a rotation that are not finite numbers
/// (`010164a0`, `01016458`).
const FORMAT_BAD_POSITION: u32 = 0x0101_64a0;
const FORMAT_BAD_ROTATION: u32 = 0x0101_6458;
/// `007af430` (`ECX` = reference): its base object.
const BASE_OBJECT: u32 = 0x007a_f430;
/// `00401170` (`ECX` = form): its form type.
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type a linked door's base must have (0x1c).
const DOOR_FORM_TYPE: u32 = 0x1c;
/// `0084e3a0` (`ECX` = form): its form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The form type names, three words per type (`01187004` + 12 * type).
const FORM_TYPE_NAMES: u32 = 0x0118_7004;
/// `_finite` (`00ec7595`, cdecl: a double).
const FINITE: u32 = 0x00ec_7595;
/// `_isnan` (`00ec75b1`, cdecl: a double).
const IS_NAN: u32 = 0x00ec_75b1;
/// The point a bad position or rotation is reset to (`011f426c`, three
/// floats).
const ORIGIN: u32 = 0x011f_426c;

// Translated from 0043a160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::DoorTeleportData` (Xbox PDB): runs `006815c0` on the
/// position and the rotation, clears the linked door and the flags, and
/// sets both points (through a stack point set by `00416870`) to the float
/// at `01016450` in all three coordinates. Returns `this`.
pub fn door_teleport_data_door_teleport_data(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
) -> Ptr<DoorTeleportData> {
    let base = this.addr();
    e.call(POINT_PREPARE, &args![base + 0x4]);
    e.call(POINT_PREPARE, &args![base + 0x10]);
    e.set(this, DoorTeleportData::pLinkedDoor, 0);
    e.set(this, DoorTeleportData::cFlags, 0);
    for target in [base + 0x4, base + 0x10] {
        // FLD float / FSTP float onto the stack, three times.
        let coordinate = x87_quiet_bits(e.mem.u32(START_COORDINATE));
        e.with_stack(0xc, |e, point| {
            let set = e
                .call(POINT_SET, &args![point, coordinate, coordinate, coordinate])
                .u32();
            for i in 0..3 {
                let word = e.mem.u32(set + 4 * i);
                e.mem.set_u32(target + 4 * i, word);
            }
        });
    }
    this
}

// Translated from 0043a210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::SetConnectedCellsPublic` (Xbox PDB; a reference and a
/// byte; `RET 8`): nothing for a null reference; otherwise the linked
/// door's cell (`GetTeleportCell`) and the reference's cell (`008d6f30`),
/// each when it exists and has an owner, get `SetTempPublic` with the byte.
pub fn door_teleport_data_set_connected_cells_public(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
    reference: Ptr,
    public: u8,
) {
    if reference.is_null() {
        return;
    }
    let cell = door_teleport_data_get_teleport_cell(e, this);
    if cell != 0 && e.call(CELL_GET_OWNER, &args![cell]).u32() != 0 {
        e.call(CELL_SET_TEMP_PUBLIC, &args![cell, u32::from(public)]);
    }
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    if cell != 0 && e.call(CELL_GET_OWNER, &args![cell]).u32() != 0 {
        e.call(CELL_SET_TEMP_PUBLIC, &args![cell, u32::from(public)]);
    }
}

// Translated from 0043a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::GetTeleportCell` (Xbox PDB): the linked door's cell
/// (`008d6f30`), 0 without a linked door.
pub fn door_teleport_data_get_teleport_cell(e: &mut Engine, this: Ptr<DoorTeleportData>) -> u32 {
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    if door == 0 {
        return 0;
    }
    e.call(REFERENCE_CELL, &args![door]).u32()
}

// Translated from 0043a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::GetTeleportCellMapName` (Xbox PDB; the string to fill;
/// `RET 4`): the linked door's `00578870` with the string, or the string
/// set empty (`004037f0` with `01011584`, 0) without a linked door.
pub fn door_teleport_data_get_teleport_cell_map_name(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
    name: Ptr,
) {
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    if door == 0 {
        e.call(STRING_SET, &args![name, EMPTY_STRING, 0u32]);
    } else {
        e.call(REFERENCE_MAP_NAME, &args![door, name]);
    }
}

// Translated from 0043a320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::GetTeleportWorldSpace` (Xbox PDB): the world space
/// (`TESObjectCELL::GetWorldSpace`) of the linked door's cell (`008d6f30`),
/// or, when that is null, of the cell slot 0 of the vtable at the door's
/// +0x18 gives (`ECX` = door + 0x18); 0 without a door or a cell.
pub fn door_teleport_data_get_teleport_world_space(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
) -> u32 {
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    if door == 0 {
        return 0;
    }
    let cell = e.call(REFERENCE_CELL, &args![door]).u32();
    if cell != 0 {
        return e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
    }
    let cell = e.vcall(door + 0x18, 0, &[]).u32();
    if cell == 0 {
        return 0;
    }
    e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32()
}

// Translated from 0043a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `XTEL` chunk (`thiscall`, the file; `RET 4`; the engine map
/// gives it no name): nothing for a null file or another chunk; otherwise
/// a 0x20-byte stack copy is prepared (`0055a400`), cleared, filled from the
/// chunk (0x20 bytes), byte-swapped when the file needs it (`00414030`) and
/// copied into `this` (seven words and the flags byte).
pub fn fn_0043a4e0(e: &mut Engine, this: Ptr<DoorTeleportData>, file: Ptr) {
    if file.is_null() {
        return;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_XTEL {
        return;
    }
    let base = this.addr();
    e.with_stack(0x20, |e, copy| {
        e.call(DATA_PREPARE, &args![copy]);
        e.call(FILL_MEMORY, &args![copy, 0u32, 0x20u32]);
        e.call(GET_CHUNK_DATA, &args![file, copy, 0x20u32]);
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            e.call(DATA_SWAP, &args![copy]);
        }
        for i in 0..7 {
            let word = e.mem.u32(copy.addr() + 4 * i);
            e.mem.set_u32(base + 4 * i, word);
        }
        let flags = e.mem.u8(copy.addr() + 0x1c);
        e.set(this, DoorTeleportData::cFlags, flags);
    });
}

/// `_finite` and `_isnan` on the three floats at `point` (each `FLD float`,
/// `FSTP double`), in the exe's order: true when one is not finite or is a
/// NaN.
fn point_is_bad(e: &mut Engine, point: u32) -> bool {
    for i in 0..3 {
        let value = x87_load32(e.mem.u32(point + 4 * i));
        if e.call(FINITE, &args![value]).u32() == 0 {
            return true;
        }
    }
    for i in 0..3 {
        let value = x87_load32(e.mem.u32(point + 4 * i));
        if e.call(IS_NAN, &args![value]).u32() != 0 {
            return true;
        }
    }
    false
}

// Translated from 0043a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::InitItem` (Xbox PDB; the owning form; `RET 4`): the
/// linked door's form id gets the owner's file's compile index and is
/// looked up and cast to `TESObjectREFR`. Without a door it warns and
/// returns false. A door whose base object is not of form type 0x1c is
/// dropped with a warning (its id, its type name from the table at
/// `01187004`, its slot `0x130` and its form id) and false. Otherwise a
/// position and a rotation that are not finite numbers are each reset to
/// the point at `011f426c` with a warning; true.
pub fn door_teleport_data_init_item(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
    owner: Ptr,
) -> bool {
    let base = this.addr();
    let door = e.with_stack(4, |e, id| {
        let linked = e.get(this, DoorTeleportData::pLinkedDoor);
        e.mem.set_u32(id.addr(), linked);
        let file = e.call(GET_FILE, &args![owner, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id, file]);
        let form_id = e.mem.u32(id.addr());
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        let door = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        e.set(this, DoorTeleportData::pLinkedDoor, door);
        (door, form_id)
    });
    let (door, form_id) = door;
    if door == 0 {
        e.call(LOG_WARNING, &args![FORMAT_NO_DOOR, form_id]);
        return false;
    }
    let object = e.call(BASE_OBJECT, &args![door]).u32();
    if e.call(FORM_TYPE, &args![object]).u32() != DOOR_FORM_TYPE {
        let object = e.call(BASE_OBJECT, &args![door]).u32();
        let named = e.call(BASE_OBJECT, &args![door]).u32();
        let id = e.call(FORM_ID, &args![named]).u32();
        let editor_id = e.vcall(object, 0x130, &[]).u32();
        let typed = e.call(BASE_OBJECT, &args![door]).u32();
        let kind = e.call(FORM_TYPE, &args![typed]).u32();
        let type_name = e
            .mem
            .u32(kind.wrapping_mul(0xc).wrapping_add(FORM_TYPE_NAMES));
        e.call(
            LOG_WARNING,
            &args![FORMAT_NOT_A_DOOR, form_id, type_name, editor_id, id],
        );
        e.set(this, DoorTeleportData::pLinkedDoor, 0);
        return false;
    }
    for (offset, format) in [(0x4, FORMAT_BAD_POSITION), (0x10, FORMAT_BAD_ROTATION)] {
        if point_is_bad(e, base + offset) {
            e.call(LOG_WARNING, &args![format]);
            for i in 0..3 {
                let word = e.mem.u32(ORIGIN + 4 * i);
                e.mem.set_u32(base + offset + 4 * i, word);
            }
        }
    }
    true
}

/// `00717e50` (`ECX` = object): returns the object's address + 4 (the
/// engine map's name, `BaseProcess::GetActorPackageThatIsRunning`, is a
/// folded body).
const ADDRESS_PLUS_4: u32 = 0x0071_7e50;
/// `00460140` (`ECX` = object): returns the object's address + 0x10.
const ADDRESS_PLUS_16: u32 = 0x0046_0140;
/// `00559450` (`ECX` = object): returns the object's first word.
const FIRST_WORD: u32 = 0x0055_9450;
/// `00537e90` (`ECX` = object, one word; `RET 4`): stores the word as the
/// object's first word (the engine map's name,
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer`, is a folded body).
const SET_FIRST_WORD: u32 = 0x0053_7e90;
/// `006f4ed0` (`ECX` = object, a point; `RET 4`): copies the point's three
/// words to the object's +4.
const SET_POINT_AT_4: u32 = 0x006f_4ed0;
/// `00439090` (`ECX` = point, the other point; `RET 4`): the comparison of
/// two points this unit's `Compare` uses (nonzero byte = they differ).
const POINT_DIFFERS: u32 = 0x0043_9090;
/// `TESObjectREFR::GetSpace` (Xbox PDB name of `00575ca0`; `ECX` =
/// reference).
const REFERENCE_GET_SPACE: u32 = 0x0057_5ca0;
/// `00401500`: returns the byte at `011c54ba` (whether chunk data is
/// byte-swapped when written).
const WRITE_NEEDS_SWAP: u32 = 0x0040_1500;
/// `TESForm::__AddChunkData` (Xbox PDB name of `00485990`; cdecl: tag,
/// buffer, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// The save/load game object (`011de45c`, a pointer).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008579b0` (`ECX` = save/load game; buffer, size; `RET 8`): writes bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
/// `TESSaveLoadGame::SaveNumericID` (Xbox PDB name of `00857a10`; buffer,
/// size).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `008579e0` (`ECX` = save/load game; buffer, size; `RET 8`): reads bytes.
const LOAD_READ: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::LoadNumericID` (Xbox PDB name of `00857aa0`; buffer,
/// size).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `00865e50` (`ECX` = save game buffer; data, size, a word; `RET 0xc`):
/// writes bytes.
const BUFFER_WRITE: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB name of `00865df0`; `ECX` =
/// buffer; form, a word).
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `00864980` (`ECX` = load game buffer; data, size; `RET 8`): reads bytes.
const BUFFER_READ: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB name of `008648e0`; `ECX`
/// = buffer; the address of the form id to read).
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48e0;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 0043a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the rotation (+0x10) to the three words of the point at `point`
/// (`RET 4`).
pub fn fn_0043a280(e: &mut Engine, this: Ptr<DoorTeleportData>, point: Ptr) {
    copy_point(e, this.addr() + 0x10, point.addr());
}

// Translated from 0043a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::GetTeleportWorldLocation` (Xbox PDB; the location to
/// fill; `RET 4`): fills `result` (`0043a3c0`) with the position (`this` +
/// 4 through `00717e50`) and the linked door's space (`0043a3f0`); returns
/// `result`.
pub fn door_teleport_data_get_teleport_world_location(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
    result: Ptr,
) -> Ptr {
    let space = fn_0043a3f0(e, this);
    let position = e.call(ADDRESS_PLUS_4, &args![this]).u32();
    bgs_world_location_bgs_world_location(e, result, Ptr::new(position), space);
    result
}

// Translated from 0043a3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::BGSWorldLocation` (Xbox PDB; a point and a space;
/// `RET 8`): copies the point's three words to `this` and the space word to
/// +0xC; returns `this`.
pub fn bgs_world_location_bgs_world_location(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    space: u32,
) -> Ptr {
    copy_point(e, this.addr(), point.addr());
    e.mem.set_u32(this.addr() + 0xc, space);
    this
}

// Translated from 0043a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The linked door's space (`TESObjectREFR::GetSpace`), 0 without a door.
pub fn fn_0043a3f0(e: &mut Engine, this: Ptr<DoorTeleportData>) -> u32 {
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    if door == 0 {
        return 0;
    }
    e.call(REFERENCE_GET_SPACE, &args![door]).u32()
}

// Translated from 0043a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the `XTEL` chunk: nothing without a linked door; otherwise a
/// 0x20-byte stack copy is prepared (`0055a400`) and cleared, filled with
/// the door's form id (`0084e3a0`), the position, the rotation and the
/// flags byte, byte-swapped when `00401500` says so, added as chunk data
/// (`TESForm::__AddChunkData`) and swapped back when `00401500` says so.
pub fn fn_0043a420(e: &mut Engine, this: Ptr<DoorTeleportData>) {
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    if door == 0 {
        return;
    }
    let base = this.addr();
    e.with_stack(0x20, |e, copy| {
        let buffer = copy.addr();
        e.call(DATA_PREPARE, &args![copy]);
        e.call(FILL_MEMORY, &args![copy, 0u32, 0x20u32]);
        let door = e.mem.u32(base);
        let id = e.call(FORM_ID, &args![door]).u32();
        e.mem.set_u32(buffer, id);
        copy_point(e, buffer + 4, base + 4);
        copy_point(e, buffer + 0x10, base + 0x10);
        let flags = e.get(this, DoorTeleportData::cFlags);
        e.mem.set_u8(buffer + 0x1c, flags);
        if e.call(WRITE_NEEDS_SWAP, &[]).u8() != 0 {
            e.call(DATA_SWAP, &args![copy]);
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_XTEL, copy, 0x20u32]);
        if e.call(WRITE_NEEDS_SWAP, &[]).u8() != 0 {
            e.call(DATA_SWAP, &args![copy]);
        }
    });
}

// Translated from 0043a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another teleport data (`RET 4`): the linked door (`00559450`,
/// `00537e90`), the position (`00717e50`, `006f4ed0`), the rotation
/// (`00460140`, `0043a280`) and the flags byte.
pub fn fn_0043a810(e: &mut Engine, this: Ptr<DoorTeleportData>, other: Ptr<DoorTeleportData>) {
    let door = e.call(FIRST_WORD, &args![other]).u32();
    e.call(SET_FIRST_WORD, &args![this, door]);
    let position = e.call(ADDRESS_PLUS_4, &args![other]).u32();
    e.call(SET_POINT_AT_4, &args![this, position]);
    let rotation = e.call(ADDRESS_PLUS_16, &args![other]).u32();
    fn_0043a280(e, this, Ptr::new(rotation));
    let flags = e.get(other, DoorTeleportData::cFlags);
    e.set(this, DoorTeleportData::cFlags, flags);
}

// Translated from 0043a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::Compare` (Xbox PDB; the other data; `RET 4`): true
/// when the linked doors (`00559450`), the positions (`00439090` on the two
/// `00717e50` points), the rotations (`00439090` on the two `00460140`
/// points) or the flags differ, checked in that order.
pub fn door_teleport_data_compare(
    e: &mut Engine,
    this: Ptr<DoorTeleportData>,
    other: Ptr<DoorTeleportData>,
) -> bool {
    let mine = e.call(FIRST_WORD, &args![this]).u32();
    let theirs = e.call(FIRST_WORD, &args![other]).u32();
    if mine != theirs {
        return true;
    }
    let theirs = e.call(ADDRESS_PLUS_4, &args![other]).u32();
    let mine = e.call(ADDRESS_PLUS_4, &args![this]).u32();
    if e.call(POINT_DIFFERS, &args![mine, theirs]).u8() != 0 {
        return true;
    }
    let theirs = e.call(ADDRESS_PLUS_16, &args![other]).u32();
    let mine = e.call(ADDRESS_PLUS_16, &args![this]).u32();
    if e.call(POINT_DIFFERS, &args![mine, theirs]).u8() != 0 {
        return true;
    }
    e.get(this, DoorTeleportData::cFlags) != e.get(other, DoorTeleportData::cFlags)
}

// Translated from 0043a8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The save size: 0xC + 0xC + 4 = 0x1C (a `u16` summed on the stack).
pub fn fn_0043a8f0(_e: &mut Engine, _this: Ptr<DoorTeleportData>) -> u16 {
    let mut size: u16 = 0;
    size += 0xc;
    size += 0xc;
    size += 4;
    size
}

// Translated from 0043a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::SaveGame` (Xbox PDB): writes the position and the
/// rotation (0xC bytes each, `008579b0` on the save/load game at
/// `011de45c`), then the linked door's form id (0 without a door) through
/// `SaveNumericID`.
pub fn door_teleport_data_save_game(e: &mut Engine, this: Ptr<DoorTeleportData>) {
    let base = this.addr();
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_WRITE, &args![game, base + 4, 0xcu32]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_WRITE, &args![game, base + 0x10, 0xcu32]);
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), 0);
        let door = e.get(this, DoorTeleportData::pLinkedDoor);
        if door != 0 {
            let form_id = e.call(FORM_ID, &args![door]).u32();
            e.mem.set_u32(id.addr(), form_id);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![game, id, 4u32]);
    });
}

// Translated from 0043a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::LoadGame` (Xbox PDB): reads the position and the
/// rotation (0xC bytes each, `008579e0`), then the linked door's form id
/// through `LoadNumericID` into the door word.
pub fn door_teleport_data_load_game(e: &mut Engine, this: Ptr<DoorTeleportData>) {
    let base = this.addr();
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_READ, &args![game, base + 4, 0xcu32]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_READ, &args![game, base + 0x10, 0xcu32]);
    e.with_stack(4, |e, id| {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![game, id, 4u32]);
        let form_id = e.mem.u32(id.addr());
        e.set(this, DoorTeleportData::pLinkedDoor, form_id);
    });
}

// Translated from 0043a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the linked door's form id: when nonzero, the form with that id
/// (`004839c0`) cast to `TESObjectREFR` (`__RTDynamicCast`) replaces it.
pub fn fn_0043a9f0(e: &mut Engine, this: Ptr<DoorTeleportData>) {
    let id = e.get(this, DoorTeleportData::pLinkedDoor);
    if id == 0 {
        return;
    }
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let door = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
        )
        .u32();
    e.set(this, DoorTeleportData::pLinkedDoor, door);
}

// Translated from 0043aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::SaveGame_ov2` (Xbox PDB; the save game buffer;
/// `RET 4`): writes the position, the rotation (0xC bytes each) and the
/// flags byte (`00865e50`, each with 0), then the linked door
/// (`SaveFormID_ov2` with 0).
pub fn door_teleport_data_save_game_ov2(e: &mut Engine, this: Ptr<DoorTeleportData>, buffer: Ptr) {
    let base = this.addr();
    e.call(BUFFER_WRITE, &args![buffer, base + 4, 0xcu32, 0u32]);
    e.call(BUFFER_WRITE, &args![buffer, base + 0x10, 0xcu32, 0u32]);
    e.call(BUFFER_WRITE, &args![buffer, base + 0x1c, 1u32, 0u32]);
    let door = e.get(this, DoorTeleportData::pLinkedDoor);
    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, door, 0u32]);
}

// Translated from 0043aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorTeleportData::LoadGame_ov2` (Xbox PDB; the load game buffer;
/// `RET 4`): reads the position, the rotation (0xC bytes each) and the
/// flags byte (`00864980`), then the linked door's form id into the door
/// word (`LoadFormID_ov2`).
pub fn door_teleport_data_load_game_ov2(e: &mut Engine, this: Ptr<DoorTeleportData>, buffer: Ptr) {
    let base = this.addr();
    e.call(BUFFER_READ, &args![buffer, base + 4, 0xcu32]);
    e.call(BUFFER_READ, &args![buffer, base + 0x10, 0xcu32]);
    e.call(BUFFER_READ, &args![buffer, base + 0x1c, 1u32]);
    e.call(BUFFER_LOAD_FORM_ID, &args![buffer, this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0043_a160,
            door_teleport_data_door_teleport_data(Ptr<DoorTeleportData>) -> Ptr<DoorTeleportData>
        ),
        entry!(
            0x0043_a210,
            door_teleport_data_set_connected_cells_public(Ptr<DoorTeleportData>, Ptr, u8)
        ),
        entry!(
            0x0043_a2b0,
            door_teleport_data_get_teleport_cell(Ptr<DoorTeleportData>) -> u32
        ),
        entry!(
            0x0043_a2e0,
            door_teleport_data_get_teleport_cell_map_name(Ptr<DoorTeleportData>, Ptr)
        ),
        entry!(
            0x0043_a320,
            door_teleport_data_get_teleport_world_space(Ptr<DoorTeleportData>) -> u32
        ),
        entry!(0x0043_a4e0, fn_0043a4e0(Ptr<DoorTeleportData>, Ptr)),
        entry!(
            0x0043_a590,
            door_teleport_data_init_item(Ptr<DoorTeleportData>, Ptr) -> bool
        ),
        entry!(0x0043_a280, fn_0043a280(Ptr<DoorTeleportData>, Ptr)),
        entry!(
            0x0043_a390,
            door_teleport_data_get_teleport_world_location(Ptr<DoorTeleportData>, Ptr) -> Ptr
        ),
        entry!(
            0x0043_a3c0,
            bgs_world_location_bgs_world_location(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(0x0043_a3f0, fn_0043a3f0(Ptr<DoorTeleportData>) -> u32),
        entry!(0x0043_a420, fn_0043a420(Ptr<DoorTeleportData>)),
        entry!(
            0x0043_a810,
            fn_0043a810(Ptr<DoorTeleportData>, Ptr<DoorTeleportData>)
        ),
        entry!(
            0x0043_a860,
            door_teleport_data_compare(Ptr<DoorTeleportData>, Ptr<DoorTeleportData>) -> bool
        ),
        entry!(0x0043_a8f0, fn_0043a8f0(Ptr<DoorTeleportData>) -> u16),
        entry!(
            0x0043_a930,
            door_teleport_data_save_game(Ptr<DoorTeleportData>)
        ),
        entry!(
            0x0043_a9a0,
            door_teleport_data_load_game(Ptr<DoorTeleportData>)
        ),
        entry!(0x0043_a9f0, fn_0043a9f0(Ptr<DoorTeleportData>)),
        entry!(
            0x0043_aa40,
            door_teleport_data_save_game_ov2(Ptr<DoorTeleportData>, Ptr)
        ),
        entry!(
            0x0043_aaa0,
            door_teleport_data_load_game_ov2(Ptr<DoorTeleportData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        e
    }

    #[test]
    fn constructor_sets_both_points_to_the_start_coordinate() {
        let mut e = engine(&[POINT_PREPARE]);
        e.map(START_COORDINATE, 4);
        // A signalling NaN: FLD/FSTP quiets it.
        e.mem.set_u32(START_COORDINATE, 0x7f80_0001);
        e.register(POINT_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize] + i);
            }
            ret(a[0])
        });
        let this = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        let (result, log) = run(&mut e, 0x0043_a160, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            calls(&log, POINT_PREPARE),
            vec![vec![this + 4], vec![this + 0x10]]
        );
        let sets = calls(&log, POINT_SET);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][1..], [0x7fc0_0001; 3]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        let n = 0x7fc0_0001;
        assert_eq!(
            words,
            vec![0, n, n + 1, n + 2, n, n + 1, n + 2, 0xffff_ff00]
        );
    }

    #[test]
    fn connected_cells_are_made_public_when_owned() {
        let mut e = engine(&[CELL_SET_TEMP_PUBLIC]);
        e.register(REFERENCE_CELL, |_, a| ret(a[0] + 0x100));
        e.register(CELL_GET_OWNER, |_, a| ret(u32::from(a[0] == 0x5100)));
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x5000);
        let (_, log) = run(&mut e, 0x0043_a210, &[this, 0x6000, 0x201]);
        assert_eq!(
            calls(&log, CELL_GET_OWNER),
            vec![vec![0x5100], vec![0x6100]]
        );
        assert_eq!(calls(&log, CELL_SET_TEMP_PUBLIC), vec![vec![0x5100, 1]]);
        let (_, log) = run(&mut e, 0x0043_a210, &[this, 0, 1]);
        assert!(log.is_empty());
        e.mem.set_u32(this, 0);
        e.register(CELL_GET_OWNER, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0043_a210, &[this, 0x6000, 0]);
        assert_eq!(calls(&log, CELL_SET_TEMP_PUBLIC), vec![vec![0x6100, 0]]);
    }

    #[test]
    fn teleport_cell_and_map_name_follow_the_linked_door() {
        let mut e = engine(&[STRING_SET, REFERENCE_MAP_NAME]);
        e.register(REFERENCE_CELL, |_, a| ret(a[0] + 0x100));
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0043_a2b0, &args![this]).u32(), 0);
        let (_, log) = run(&mut e, 0x0043_a2e0, &[this, 0x7000]);
        assert_eq!(log, vec![(STRING_SET, vec![0x7000, EMPTY_STRING, 0])]);
        e.mem.set_u32(this, 0x5000);
        assert_eq!(e.call(0x0043_a2b0, &args![this]).u32(), 0x5100);
        let (_, log) = run(&mut e, 0x0043_a2e0, &[this, 0x7000]);
        assert_eq!(log, vec![(REFERENCE_MAP_NAME, vec![0x5000, 0x7000])]);
    }

    #[test]
    fn world_space_comes_from_the_cell_or_the_doors_parent_cell() {
        let mut e = engine(&[]);
        e.register(CELL_GET_WORLD_SPACE, |_, a| ret(a[0] + 1));
        e.register(REFERENCE_CELL, |_, _| ret(0x5100));
        e.register(0x0f00_1000, |_, _| ret(0x5200));
        e.put_vtable(0x0f00_0000, &[0x0f00_1000]);
        let door = e.mem.alloc(0x20);
        e.mem.set_u32(door + 0x18, 0x0f00_0000);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0043_a320, &args![this]).u32(), 0);
        e.mem.set_u32(this, door);
        assert_eq!(e.call(0x0043_a320, &args![this]).u32(), 0x5101);
        e.register(REFERENCE_CELL, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x0043_a320, &[this]);
        assert_eq!(result.u32(), 0x5201);
        assert_eq!(calls(&log, 0x0f00_1000), vec![vec![door + 0x18]]);
        e.register(0x0f00_1000, |_, _| ret(0));
        assert_eq!(e.call(0x0043_a320, &args![this]).u32(), 0);
    }

    #[test]
    fn xtel_chunk_is_copied_in() {
        let mut e = engine(&[DATA_PREPARE, FILL_MEMORY, DATA_SWAP]);
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_XTEL));
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register(GET_CHUNK_DATA, |e, a| {
            for i in 0..8 {
                e.mem.set_u32(a[1] + 4 * i, 0x100 + i);
            }
            ret(0)
        });
        let this = e.mem.alloc(0x20);
        let (_, log) = run(&mut e, 0x0043_a4e0, &[this, 0x7000]);
        let copy = calls(&log, DATA_PREPARE)[0][0];
        assert_eq!(calls(&log, FILL_MEMORY), vec![vec![copy, 0, 0x20]]);
        assert_eq!(calls(&log, GET_CHUNK_DATA), vec![vec![0x7000, copy, 0x20]]);
        assert_eq!(calls(&log, DATA_SWAP), vec![vec![copy]]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x100, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x07]
        );
        let (_, log) = run(&mut e, 0x0043_a4e0, &[this, 0]);
        assert!(log.is_empty());
        e.register(GET_TES_CHUNK, |_, _| ret(0x1234));
        let (_, log) = run(&mut e, 0x0043_a4e0, &[this, 0x7000]);
        assert_eq!(log, vec![(GET_TES_CHUNK, vec![0x7000])]);
    }

    /// An engine for `InitItem`: the file is 0x7100, the compile index adds
    /// 0x0100_0000, the lookup gives `form + 0x10` and the cast passes it on.
    fn init_engine(door_type: u32) -> Engine {
        let mut e = engine(&[LOG_WARNING]);
        e.register(GET_FILE, |_, _| ret(0x7100));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == 0 {
                0
            } else {
                0x2000_0000 | (a[0] & 0xffff)
            })
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register_double(FORM_TYPE, move |_, _| ret(door_type));
        e.register(FORM_ID, |_, _| ret(0x0012_3456));
        e.register(FINITE, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(u32::from(value.is_finite()))
        });
        e.register(IS_NAN, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(u32::from(value.is_nan()))
        });
        e.map(ORIGIN, 0xc);
        for i in 0..3 {
            e.mem.set_u32(ORIGIN + 4 * i, 0x4000_0000 + i);
        }
        e.map(FORM_TYPE_NAMES, 0x200);
        e
    }

    #[test]
    fn init_item_warns_without_a_door() {
        let mut e = init_engine(DOOR_FORM_TYPE);
        let this = e.mem.alloc(0x20);
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.mem.set_u32(this, 0x55);
        let (result, log) = run(&mut e, 0x0043_a590, &[this, 0x4000]);
        assert!(!result.bool());
        assert_eq!(calls(&log, GET_FILE), vec![vec![0x4000, 0xffff_ffff]]);
        assert_eq!(calls(&log, LOOKUP_FORM), vec![vec![0x0100_0055]]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x2000_0055, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_DOOR, 0x0100_0055]]
        );
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn init_item_drops_a_link_to_something_not_a_door() {
        let mut e = init_engine(0x1f);
        e.register(BASE_OBJECT, |_, _| ret(0x0f10_0000));
        e.register(0x0f00_1300, |_, _| ret(0x6600));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x0f00_1300;
        e.put_vtable(0x0f00_0000, &slots);
        e.map(0x0f10_0000, 4);
        e.mem.set_u32(0x0f10_0000, 0x0f00_0000);
        e.mem.set_u32(FORM_TYPE_NAMES + 0x1f * 0xc, 0x6700);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x55);
        let (result, log) = run(&mut e, 0x0043_a590, &[this, 0x4000]);
        assert!(!result.bool());
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![
                FORMAT_NOT_A_DOOR,
                0x0100_0055,
                0x6700,
                0x6600,
                0x0012_3456
            ]]
        );
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn init_item_resets_points_that_are_not_finite() {
        let mut e = init_engine(DOOR_FORM_TYPE);
        e.register(BASE_OBJECT, |_, _| ret(0x6000));
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x55);
        e.mem.set_u32(this + 4, 1.0f32.to_bits());
        e.mem.set_u32(this + 8, f32::INFINITY.to_bits());
        e.mem.set_u32(this + 0xc, 2.0f32.to_bits());
        for i in 0..3 {
            e.mem.set_u32(this + 0x10 + 4 * i, (i as f32).to_bits());
        }
        let (result, log) = run(&mut e, 0x0043_a590, &[this, 0x4000]);
        assert!(result.bool());
        assert_eq!(e.mem.u32(this), 0x2000_0055);
        assert_eq!(calls(&log, LOG_WARNING), vec![vec![FORMAT_BAD_POSITION]]);
        // The first infinity stops the `_finite` checks of the position.
        assert_eq!(calls(&log, FINITE).len(), 2 + 3);
        assert_eq!(calls(&log, IS_NAN).len(), 3);
        let words: Vec<u32> = (1..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                0x4000_0000,
                0x4000_0001,
                0x4000_0002,
                0,
                1.0f32.to_bits(),
                2.0f32.to_bits()
            ]
        );
        // A signalling NaN rotation is quieted by the load and fails `_finite`.
        e.mem.set_u32(this + 0x14, 0x7f80_0001);
        let (_, log) = run(&mut e, 0x0043_a590, &[this, 0x4000]);
        assert_eq!(calls(&log, LOG_WARNING), vec![vec![FORMAT_BAD_ROTATION]]);
        let rotation = calls(&log, FINITE)[4].clone();
        assert_eq!(
            u64::from(rotation[0]) | u64::from(rotation[1]) << 32,
            0x7ff8_0000_2000_0000
        );
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| Ret::default());
        }
        e.register(ADDRESS_PLUS_4, |_, a| (a[0] + 4).into_ret());
        e.register(ADDRESS_PLUS_16, |_, a| (a[0] + 0x10).into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    fn data(e: &mut Engine, seed: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(this + 4 * i, seed + i);
        }
        this
    }

    #[test]
    fn rotation_setter_and_world_location_copy_points() {
        let mut e = engine(&[]);
        let this = data(&mut e, 0x100);
        let point = data(&mut e, 0x900);
        e.call(0x0043_a280, &args![this, point]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x100, 0x101, 0x102, 0x103, 0x900, 0x901, 0x902, 0x107]
        );
        e.register(REFERENCE_GET_SPACE, |_, a| (a[0] + 1).into_ret());
        let result = e.mem.alloc(0x10);
        let (ret, log) = run(&mut e, 0x0043_a390, &[this, result]);
        assert_eq!(ret.u32(), result);
        assert_eq!(calls(&log, REFERENCE_GET_SPACE), vec![vec![0x100]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(result + 4 * i)).collect();
        assert_eq!(words, vec![0x101, 0x102, 0x103, 0x101]);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x0043_a3f0, &args![this]).u32(), 0);
        let (ret, _) = run(&mut e, 0x0043_a3c0, &[result, point, 0x77]);
        assert_eq!(ret.u32(), result);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(result + 4 * i)).collect();
        assert_eq!(words, vec![0x900, 0x901, 0x902, 0x77]);
    }

    #[test]
    fn xtel_chunk_is_written_with_the_doors_form_id() {
        let mut e = engine(&[DATA_PREPARE, DATA_SWAP]);
        e.register(FORM_ID, |_, _| 0xabcdu32.into_ret());
        e.register(WRITE_NEEDS_SWAP, |_, _| 1u32.into_ret());
        e.register(ADD_CHUNK_DATA, |e, a| {
            let words: Vec<u32> = (0..8).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            assert_eq!(
                words,
                vec![0xabcd, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x07]
            );
            Ret::default()
        });
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        let this = data(&mut e, 0x100);
        let (_, log) = run(&mut e, 0x0043_a420, &[this]);
        let copy = calls(&log, DATA_PREPARE)[0][0];
        assert_eq!(calls(&log, FORM_ID), vec![vec![0x100]]);
        assert_eq!(
            calls(&log, ADD_CHUNK_DATA),
            vec![vec![CHUNK_XTEL, copy, 0x20]]
        );
        assert_eq!(calls(&log, DATA_SWAP), vec![vec![copy], vec![copy]]);
        e.register(WRITE_NEEDS_SWAP, |_, _| 0x100u32.into_ret());
        let (_, log) = run(&mut e, 0x0043_a420, &[this]);
        assert!(calls(&log, DATA_SWAP).is_empty());
        e.mem.set_u32(this, 0);
        let (_, log) = run(&mut e, 0x0043_a420, &[this]);
        assert!(log.is_empty());
    }

    #[test]
    fn copy_takes_door_points_and_flags() {
        let mut e = engine(&[]);
        e.register(SET_FIRST_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(SET_POINT_AT_4, |e, a| {
            for i in 0..3 {
                let w = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 + 4 * i, w);
            }
            Ret::default()
        });
        let this = data(&mut e, 0x100);
        let other = data(&mut e, 0x200);
        e.call(0x0043_a810, &args![this, other]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x200, 0x201, 0x202, 0x203, 0x204, 0x205, 0x206, 0x107]
        );
    }

    #[test]
    fn compare_checks_door_points_and_flags_in_order() {
        let mut e = engine(&[]);
        e.register(POINT_DIFFERS, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let this = data(&mut e, 0x100);
        let other = data(&mut e, 0x100);
        let (ret, log) = run(&mut e, 0x0043_a860, &[this, other]);
        assert!(!ret.bool());
        assert_eq!(
            calls(&log, POINT_DIFFERS),
            vec![vec![this + 4, other + 4], vec![this + 0x10, other + 0x10]]
        );
        e.mem.set_u8(other + 0x1c, 9);
        assert!(e.call(0x0043_a860, &args![this, other]).bool());
        e.mem.set_u32(other + 0x10, 9);
        let (ret, log) = run(&mut e, 0x0043_a860, &[this, other]);
        assert!(ret.bool());
        assert_eq!(calls(&log, POINT_DIFFERS).len(), 2);
        e.mem.set_u32(other + 4, 9);
        let (ret, log) = run(&mut e, 0x0043_a860, &[this, other]);
        assert!(ret.bool());
        assert_eq!(calls(&log, POINT_DIFFERS).len(), 1);
        e.mem.set_u32(other, 9);
        let (ret, log) = run(&mut e, 0x0043_a860, &[this, other]);
        assert!(ret.bool());
        assert!(calls(&log, POINT_DIFFERS).is_empty());
    }

    #[test]
    fn save_size_is_0x1c() {
        let mut e = engine(&[]);
        assert_eq!(e.call(0x0043_a8f0, &args![0x1234u32]).u32() & 0xffff, 0x1c);
    }

    #[test]
    fn save_and_load_game_go_through_the_save_load_game() {
        let mut e = engine(&[SAVE_WRITE, LOAD_READ]);
        e.map(SAVE_LOAD_GAME, 4);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x7700);
        e.register(FORM_ID, |_, _| 0xabcdu32.into_ret());
        e.register(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0xabcd);
            Ret::default()
        });
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            Ret::default()
        });
        let this = data(&mut e, 0x100);
        let (_, log) = run(&mut e, 0x0043_a930, &[this]);
        assert_eq!(
            calls(&log, SAVE_WRITE),
            vec![vec![0x7700, this + 4, 0xc], vec![0x7700, this + 0x10, 0xc]]
        );
        let saved = calls(&log, SAVE_NUMERIC_ID);
        assert_eq!(saved.len(), 1);
        assert_eq!((saved[0][0], saved[0][2]), (0x7700, 4));
        e.register(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e.mem.set_u32(this, 0);
        let (_, log) = run(&mut e, 0x0043_a930, &[this]);
        assert!(calls(&log, FORM_ID).is_empty());
        let (_, log) = run(&mut e, 0x0043_a9a0, &[this]);
        assert_eq!(
            calls(&log, LOAD_READ),
            vec![vec![0x7700, this + 4, 0xc], vec![0x7700, this + 0x10, 0xc]]
        );
        assert_eq!(e.mem.u32(this), 0x4444);
    }

    #[test]
    fn resolve_replaces_the_form_id_with_the_cast_reference() {
        let mut e = engine(&[]);
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x1000).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        let this = data(&mut e, 0x100);
        let (_, log) = run(&mut e, 0x0043_a9f0, &[this]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x1100, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(e.mem.u32(this), 0x1101);
        e.mem.set_u32(this, 0);
        let (_, log) = run(&mut e, 0x0043_a9f0, &[this]);
        assert!(log.is_empty());
    }

    #[test]
    fn buffer_save_and_load_write_points_flags_and_door() {
        let mut e = engine(&[
            BUFFER_WRITE,
            BUFFER_SAVE_FORM_ID,
            BUFFER_READ,
            BUFFER_LOAD_FORM_ID,
        ]);
        let this = data(&mut e, 0x100);
        let (_, log) = run(&mut e, 0x0043_aa40, &[this, 0x8800]);
        assert_eq!(
            log,
            vec![
                (BUFFER_WRITE, vec![0x8800, this + 4, 0xc, 0]),
                (BUFFER_WRITE, vec![0x8800, this + 0x10, 0xc, 0]),
                (BUFFER_WRITE, vec![0x8800, this + 0x1c, 1, 0]),
                (BUFFER_SAVE_FORM_ID, vec![0x8800, 0x100, 0]),
            ]
        );
        let (_, log) = run(&mut e, 0x0043_aaa0, &[this, 0x8800]);
        assert_eq!(
            log,
            vec![
                (BUFFER_READ, vec![0x8800, this + 4, 0xc]),
                (BUFFER_READ, vec![0x8800, this + 0x10, 0xc]),
                (BUFFER_READ, vec![0x8800, this + 0x1c, 1]),
                (BUFFER_LOAD_FORM_ID, vec![0x8800, this]),
            ]
        );
    }
}

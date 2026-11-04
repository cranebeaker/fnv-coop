//! `(unplaced)/run_00673420` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0041ca90` (`extradatalist.cpp`): the package's type (`ECX` = the
/// package).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `0055b980` (`tesobjectrefr.cpp`): the package's location (`ECX` = the
/// package; null when it has none).
const PACKAGE_LOCATION: u32 = 0x0055_b980;
/// `PackageLocation::GetLocType` (`00678ca0`, engine map).
const PACKAGE_LOCATION_GET_LOC_TYPE: u32 = 0x0067_8ca0;
/// `00569b80` (`tesobjectrefr.cpp`): asked of the argument (`ECX`); the
/// test is whether it returns 0.
const ACTOR_CHECK_00569B80: u32 = 0x0056_9b80;

/// The location type and argument test shared by the branches of
/// `fn_006737a0`: false only when the location type is 6 and
/// `00569b80(actor)` is 0.
fn location_type_6_check(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    let location = e.call(PACKAGE_LOCATION, &args![this]).u32();
    let location_type = e
        .call(PACKAGE_LOCATION_GET_LOC_TYPE, &args![location])
        .u32();
    !(location_type == 6 && e.call(ACTOR_CHECK_00569B80, &args![actor]).u32() == 0)
}

// Translated from 006737a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A test on a package (`ECX`) for `actor` (one stack word), by the package
/// type (`0041ca90`): types 6 and 0xD are false only when the location's
/// type is 6 and `00569b80(actor)` returns 0; type 0xC is false only when
/// `00569b80(actor)` returns 0 and either flag 0x40 of the word at +0x24
/// ([`fn_00673890`]) is set or the package has a location of type 6; every
/// other type is true.
pub fn fn_006737a0(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    match e.call(PACKAGE_TYPE, &args![this]).u32() {
        6 | 0xd => location_type_6_check(e, this, actor),
        0xc => {
            if !fn_00673890(e, this) {
                if e.call(PACKAGE_LOCATION, &args![this]).u32() == 0 {
                    return true;
                }
                let location = e.call(PACKAGE_LOCATION, &args![this]).u32();
                if e.call(PACKAGE_LOCATION_GET_LOC_TYPE, &args![location])
                    .u32()
                    != 6
                {
                    return true;
                }
            }
            e.call(ACTOR_CHECK_00569B80, &args![actor]).u32() != 0
        }
        _ => true,
    }
}

// Translated from 00673890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x40 of the `u16` at `this + 0x24` is set.
pub fn fn_00673890(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u16(this.addr() + 0x24) & 0x40 != 0
}

/// `TESFile::GetTESForm` (`00472660`, engine map): the record type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, engine map).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (`004726b0`, engine map).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false when there is none.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, engine map): one word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401660`: the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680`: whether the file's data needs its byte order converted.
const NEEDS_BYTE_SWAP: u32 = 0x0040_1680;
/// `00401000`: `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00403d30`: `memset` shape (cdecl: destination, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// `00483710` (`tesform.cpp`): run on the stack temporaries when they go
/// (and, in the exe, as the `PKPT` byte swap).
const TEMPORARY_DONE: u32 = 0x0048_3710;
/// `PackageEventAction::Load` (`0067dd20`, engine map).
const PACKAGE_EVENT_ACTION_LOAD: u32 = 0x0067_dd20;
/// `PackageLocation` constructor from its chunk data (`0067f060`).
const PACKAGE_LOCATION_FROM_DATA: u32 = 0x0067_f060;
/// `PackageLocation` destructor (`0067f110`).
const PACKAGE_LOCATION_DESTRUCT: u32 = 0x0067_f110;
/// `PackageTarget` constructor from its chunk data (`0067ffa0`).
const PACKAGE_TARGET_FROM_DATA: u32 = 0x0067_ffa0;

const CHUNK_PKDD: u32 = 0x4444_4b50;
const CHUNK_POCA: u32 = 0x4143_4f50;
const CHUNK_PKW2: u32 = 0x3257_4b50;
const CHUNK_PLD2: u32 = 0x3244_4c50;
const CHUNK_PTD2: u32 = 0x3244_5450;
const CHUNK_PKE2: u32 = 0x3245_4b50;
const CHUNK_PKW3: u32 = 0x3357_4b50;
const CHUNK_POBA: u32 = 0x4142_4f50;
const CHUNK_POEA: u32 = 0x4145_4f50;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_IDLA: u32 = 0x414c_4449;
const CHUNK_IDLB: u32 = 0x424c_4449;
const CHUNK_IDLC: u32 = 0x434c_4449;
const CHUNK_IDLF: u32 = 0x464c_4449;
const CHUNK_IDLN: u32 = 0x4e4c_4449;
const CHUNK_IDLT: u32 = 0x544c_4449;
const CHUNK_PKDT: u32 = 0x5444_4b50;
const CHUNK_PKFD: u32 = 0x4446_4b50;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_PTDT: u32 = 0x5444_5450;
const CHUNK_PLDT: u32 = 0x5444_4c50;
const CHUNK_PSDT: u32 = 0x5444_5350;
const CHUNK_PKPT: u32 = 0x5450_4b50;

/// Reads the current chunk into a zeroed stack buffer of `size` bytes
/// (`00403d30`, then `GetChunkData_ov3` with that size), byte-swaps it with
/// `swap` (`ECX` = the buffer) when the file needs it, and hands the buffer
/// to `then`.
fn read_chunk<R>(
    e: &mut Engine,
    file: Ptr,
    size: u32,
    swap: u32,
    then: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(size, |e, buffer| {
        let buffer = buffer.addr();
        e.call(MEMSET, &args![buffer, 0u32, size]);
        e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
        if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
            e.call(swap, &args![buffer]);
        }
        then(e, buffer)
    })
}

/// Deletes the package data at +0x28 when there is one (vtable slot 0 with
/// 1).
fn delete_package_data(e: &mut Engine, this: Ptr) {
    let data = e.mem.u32(this.addr() + 0x28);
    if data != 0 {
        e.vcall(data, 0x0, &args![1u32]);
    }
}

/// `operator new(size)`, then `construct` on the block (`ECX`) with
/// `words`; null when the allocation fails.
fn new_with(e: &mut Engine, size: u32, construct: u32, words: &[u32]) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut call = vec![block];
    call.extend_from_slice(words);
    e.call(construct, &call).u32()
}

// Translated from 00673a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00673a40 - the exception-unwinding frame (FS:[0]) and the /GS security-cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) EDID buffer is a with_stack block of the chunk size rounded up to 16 bytes.
/// The package record reader (`TESPackage`, record type 0x49; false for any
/// other), `file` one stack word: `TESForm::LoadForm`, then every chunk
/// (each read into a zeroed stack buffer and byte-swapped by its own
/// helper when the file needs it) until none is left, returning true:
/// - `EDID` (slot 0x134), `OBND` (slot 0xE0), `CTDA` (`TESCondition::Load`
///   on the conditions `0087eaa0` gives), `CNAM` (`00673400` with the word);
/// - `PKDT` (12 bytes, `00670760`): `TESPackage::SetPackType` with its
///   signed byte +4, then the three words into +0x1C;
/// - `PLDT`/`PTDT`/`PSDT`: a stack `PackageLocation` / `PackageTarget` /
///   8-byte value made from the data and given to `SetPackageLocation`
///   (`00671d30`) / `SetPackageTarget` (`00672fc0`) / `00673180`;
/// - `PLD2` (types 0xF and 0x1C: a new location into the data's +0xC; 0x10:
///   into its +0x8; else `SetPackageSecondLocation` `00671e10`), `PTD2`
///   (type 0x10: the data's target +0x4 replaced, the old one deleted by
///   `007b3fa0` with 1);
/// - the package data at +0x28 (the old one deleted through its slot 0):
///   `PKDD` dialogue (0x20 bytes, `0067b210`, the 0x18 bytes by value),
///   `PKFD` follow (0xC, `0067c040`, the word), `PKE2` escort (0xC,
///   `0067bc60`, its address), `PKW2`/`PKW3` weapon (0x24, `0067cc20` with
///   5 words / `0067cb70` with 6), `PKPT` patrol (8, `0067c450`, the
///   16-bit word); `PKDD`, `PKFD` and `PKE2` clear +0x28 before the new
///   one, the others do not;
/// - `IDLA`..`IDLT`: the idle collection at +0x34 (made when missing: 0x10
///   bytes, `00479790`) loads (`BGSIdleCollection::Load`);
/// - `POBA`/`POEA`/`POCA`: the next chunk, then `PackageEventAction::Load`
///   on +0x4C / +0x5C / +0x6C.
pub fn fn_00673a40(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if u32::from(e.call(GET_TES_FORM, &args![file]).u8()) != 0x49 {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_PKDT => {
                read_chunk(e, file, 0xc, 0x0067_0760, |e, buffer| {
                    let kind = e.mem.i8(buffer + 4) as i32;
                    e.call(0x0067_0fc0, &args![this, kind]);
                    for word in 0..3 {
                        let value = e.mem.u32(buffer + 4 * word);
                        e.mem.set_u32(base + 0x1c + 4 * word, value);
                    }
                });
            }
            CHUNK_PLDT => {
                read_chunk(e, file, 0xc, 0x0067_eff0, |e, buffer| {
                    e.with_stack(0xc, |e, location| {
                        e.call(PACKAGE_LOCATION_FROM_DATA, &args![location, buffer]);
                        e.call(0x0067_1d30, &args![this, location]);
                        e.call(PACKAGE_LOCATION_DESTRUCT, &args![location]);
                    });
                });
            }
            CHUNK_PKDD => {
                read_chunk(e, file, 0x18, 0x0067_b100, |e, buffer| {
                    delete_package_data(e, this);
                    e.mem.set_u32(base + 0x28, 0);
                    let words: Vec<u32> = (0..6).map(|i| e.mem.u32(buffer + 4 * i)).collect();
                    let data = new_with(e, 0x20, 0x0067_b210, &words);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PKFD => {
                read_chunk(e, file, 4, 0x0050_3210, |e, buffer| {
                    delete_package_data(e, this);
                    e.mem.set_u32(base + 0x28, 0);
                    let target = e.mem.u32(buffer);
                    let data = new_with(e, 0xc, 0x0067_c040, &[target]);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PKE2 => {
                read_chunk(e, file, 4, 0x0050_3210, |e, buffer| {
                    delete_package_data(e, this);
                    e.mem.set_u32(base + 0x28, 0);
                    let data = new_with(e, 0xc, 0x0067_bc60, &[buffer]);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PKW2 => {
                read_chunk(e, file, 0x14, 0x0067_ca10, |e, buffer| {
                    delete_package_data(e, this);
                    let words: Vec<u32> = (0..5).map(|i| e.mem.u32(buffer + 4 * i)).collect();
                    let data = new_with(e, 0x24, 0x0067_cc20, &words);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PKW3 => {
                read_chunk(e, file, 0x18, 0x0067_ca10, |e, buffer| {
                    delete_package_data(e, this);
                    let words: Vec<u32> = (0..6).map(|i| e.mem.u32(buffer + 4 * i)).collect();
                    let data = new_with(e, 0x24, 0x0067_cb70, &words);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PKPT => {
                read_chunk(e, file, 2, TEMPORARY_DONE, |e, buffer| {
                    delete_package_data(e, this);
                    let value = u32::from(e.mem.u16(buffer));
                    let data = new_with(e, 8, 0x0067_c450, &[value]);
                    e.mem.set_u32(base + 0x28, data);
                });
            }
            CHUNK_PLD2 => {
                read_chunk(e, file, 0xc, 0x0067_eff0, |e, buffer| {
                    match e.call(PACKAGE_TYPE, &args![this]).u32() {
                        0xf | 0x1c => {
                            let data = e.mem.u32(base + 0x28);
                            let location = new_with(e, 0xc, PACKAGE_LOCATION_FROM_DATA, &[buffer]);
                            e.mem.set_u32(data.wrapping_add(0xc), location);
                        }
                        0x10 => {
                            let data = e.mem.u32(base + 0x28);
                            let location = new_with(e, 0xc, PACKAGE_LOCATION_FROM_DATA, &[buffer]);
                            e.mem.set_u32(data.wrapping_add(8), location);
                        }
                        _ => {
                            e.with_stack(0xc, |e, location| {
                                e.call(PACKAGE_LOCATION_FROM_DATA, &args![location, buffer]);
                                e.call(0x0067_1e10, &args![this, location]);
                                e.call(PACKAGE_LOCATION_DESTRUCT, &args![location]);
                            });
                        }
                    }
                });
            }
            CHUNK_PTD2 => {
                read_chunk(e, file, 0x10, 0x0067_ff10, |e, buffer| {
                    if e.call(PACKAGE_TYPE, &args![this]).u32() == 0x10 {
                        let data = e.mem.u32(base + 0x28);
                        let old = e.mem.u32(data.wrapping_add(4));
                        if old != 0 {
                            e.call(0x007b_3fa0, &args![old, 1u32]);
                        }
                        let target = new_with(e, 0x10, PACKAGE_TARGET_FROM_DATA, &[buffer]);
                        e.mem.set_u32(data.wrapping_add(4), target);
                    }
                });
            }
            CHUNK_PTDT => {
                read_chunk(e, file, 0x10, 0x0067_ff10, |e, buffer| {
                    e.with_stack(0x10, |e, target| {
                        e.call(PACKAGE_TARGET_FROM_DATA, &args![target, buffer]);
                        e.call(0x0067_2fc0, &args![this, target]);
                        e.call(TEMPORARY_DONE, &args![target]);
                    });
                });
            }
            CHUNK_PSDT => {
                read_chunk(e, file, 8, 0x0060_ce80, |e, buffer| {
                    e.with_stack(8, |e, value| {
                        e.call(0x0067_fcf0, &args![value, buffer]);
                        e.call(0x0067_3180, &args![this, value]);
                        e.call(TEMPORARY_DONE, &args![value]);
                    });
                });
            }
            CHUNK_CNAM => {
                let value = e.with_stack(4, |e, local| {
                    e.call(GET_CHUNK_WORD, &args![file, local]);
                    e.mem.u32(local.addr())
                });
                e.call(0x0067_3400, &args![this, value]);
            }
            CHUNK_CTDA => {
                let conditions = e.call(0x0087_eaa0, &args![this]).u32();
                e.call(0x0068_0960, &args![conditions, file]);
            }
            CHUNK_IDLA | CHUNK_IDLB | CHUNK_IDLC | CHUNK_IDLF | CHUNK_IDLN | CHUNK_IDLT => {
                if e.mem.u32(base + 0x34) == 0 {
                    let collection = new_with(e, 0x10, 0x0047_9790, &[]);
                    e.mem.set_u32(base + 0x34, collection);
                }
                let collection = e.mem.u32(base + 0x34);
                e.call(0x0047_9c40, &args![collection, file]);
            }
            CHUNK_POBA | CHUNK_POEA | CHUNK_POCA => {
                e.call(NEXT_CHUNK, &args![file]);
                let action = match chunk {
                    CHUNK_POBA => base + 0x4c,
                    CHUNK_POEA => base + 0x5c,
                    _ => base + 0x6c,
                };
                e.call(PACKAGE_EVENT_ACTION_LOAD, &args![action, file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            return true;
        }
    }
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl five words) and the type descriptors of `TESForm`
/// (`01183028`) and `TESPackage` (`011846a0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_PACKAGE_TYPE: u32 = 0x0118_46a0;
/// The part getters of a package (thiscall, no arguments; null when it has none): the
/// second location `00672dd0`, the target `00671d10`, the schedule `0041d8a0`, the
/// conditions `0087eaa0`, the idle collection `005f36f0`, the form `006733e0` and the
/// getter `00441110` (the map's names for `005f36f0`, `006733e0` and `00441110` are folded
/// bodies of other classes).
const SECOND_LOCATION: u32 = 0x0067_2dd0;
const PACKAGE_TARGET: u32 = 0x0067_1d10;
const PACKAGE_SCHEDULE: u32 = 0x0041_d8a0;
const PACKAGE_CONDITIONS: u32 = 0x0087_eaa0;
const IDLE_COLLECTION: u32 = 0x005f_36f0;
const PACKAGE_FORM: u32 = 0x0067_33e0;
const GETTER_00441110: u32 = 0x0044_1110;
/// The part comparisons (thiscall, the other part; `AL` true when they differ):
/// `TESForm::CompareAllComponents` (`00485270`), `PackageLocation::Compare` (`0067f600`),
/// `PackageTarget::Compare` (`00680220`), the schedule's `0067fe10`, the conditions'
/// `00680b70` and `PackageEventAction::Compare` (`0067dbf0`), all names from the engine
/// map; the idle collection compares through its slot +0x0C and the form through its
/// slot +0x10C (`TESForm::Compare`, Xbox PDB), the package data through its slot +0x08.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const LOCATION_COMPARE: u32 = 0x0067_f600;
const TARGET_COMPARE: u32 = 0x0068_0220;
const SCHEDULE_COMPARE: u32 = 0x0067_fe10;
const CONDITIONS_COMPARE: u32 = 0x0068_0b70;
const EVENT_ACTION_COMPARE: u32 = 0x0067_dbf0;

/// One part of [`tes_package_compare`]: when `this` has the part (`get`), whether the
/// comparison says it differs from `other`'s; otherwise whether `other` has one. A
/// `slot` compares through the part's vtable (the getter on `this` asked before the one
/// on `other`), a `compare` address directly (the getter on `other` first).
fn part_differs(e: &mut Engine, this: u32, other: u32, get: u32, compare: Compare) -> bool {
    if e.call(get, &args![this]).u32() == 0 {
        return e.call(get, &args![other]).u32() != 0;
    }
    match compare {
        Compare::Slot(slot) => {
            let mine = e.call(get, &args![this]).u32();
            let theirs = e.call(get, &args![other]).u32();
            e.vcall(mine, slot, &args![theirs]).u8() != 0
        }
        Compare::Call(address) => {
            let theirs = e.call(get, &args![other]).u32();
            let mine = e.call(get, &args![this]).u32();
            e.call(address, &args![mine, theirs]).u8() != 0
        }
    }
}

/// How a part of a package is compared.
#[derive(Clone, Copy)]
enum Compare {
    Slot(u32),
    Call(u32),
}

// Translated from 00673420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::Compare` (Xbox PDB slot name `TESForm::Compare`, vtable +0x10C; at +0x10C
/// of `TESPackage`'s PC vtable `0106847c`; `thiscall`, `RET 4`): true (differs) when
/// `form` is not a `TESPackage`, when the components differ, when the type (`0041ca90`),
/// the `00441110` value or the 16-bit words at `+0x22` and `+0x24` differ, when any part
/// differs (idle collection, location, second location, target, schedule, conditions,
/// form; see [`part_differs`]), when only one has package data or both have and it
/// differs, or when any of the event actions at `+0x4C`, `+0x5C`, `+0x6C` differs; false
/// otherwise.
pub fn tes_package_compare(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, TES_PACKAGE_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let mine = this.addr();
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).u8() != 0 {
        return true;
    }
    for getter in [PACKAGE_TYPE, GETTER_00441110] {
        let value = e.call(getter, &args![this]).u32();
        if value != e.call(getter, &args![other]).u32() {
            return true;
        }
    }
    for offset in [0x22u32, 0x24] {
        if e.mem.u16(mine + offset) != e.mem.u16(other.wrapping_add(offset)) {
            return true;
        }
    }
    for (get, compare) in [
        (IDLE_COLLECTION, Compare::Slot(0xc)),
        (PACKAGE_LOCATION, Compare::Call(LOCATION_COMPARE)),
        (SECOND_LOCATION, Compare::Call(LOCATION_COMPARE)),
        (PACKAGE_TARGET, Compare::Call(TARGET_COMPARE)),
        (PACKAGE_SCHEDULE, Compare::Call(SCHEDULE_COMPARE)),
        (PACKAGE_CONDITIONS, Compare::Call(CONDITIONS_COMPARE)),
        (PACKAGE_FORM, Compare::Slot(0x10c)),
    ] {
        if part_differs(e, mine, other, get, compare) {
            return true;
        }
    }
    let data = e.mem.u32(mine + 0x28);
    let other_data = e.mem.u32(other.wrapping_add(0x28));
    if data != 0 && other_data != 0 {
        if e.vcall(data, 8, &args![other_data]).u8() != 0 {
            return true;
        }
    } else if data != 0 || other_data != 0 {
        return true;
    }
    for offset in [0x4cu32, 0x5c, 0x6c] {
        let action = mine + offset;
        let theirs = other.wrapping_add(offset);
        if e.call(EVENT_ACTION_COMPARE, &args![action, theirs]).u8() != 0 {
            return true;
        }
    }
    false
}

/// The save side (`TESForm::StartForm` `004855a0`, `TESForm::CloseForm` `00485680`,
/// `TESForm::__AddChunkData` `00485990` (cdecl chunk, data, size) and the one-word chunk
/// writer `00485910` (cdecl chunk, value), names from the engine map), the byte-order test
/// `00401500` (no arguments) and the package data's byte swap `00670760` (thiscall on
/// `+0x1C`); the parts' savers (thiscall): `PackageLocation::Save` (`0067f6c0`, chunk),
/// the schedule's `0067fec0`, `PackageTarget::Save` (`006802f0`, chunk),
/// `TESCondition::Save` (`00680910`), `BGSIdleCollection::Save` (`004799f0`) and
/// `PackageEventAction::Save` (`0067dc80`, chunk); the form's id getter `0084e3a0`.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const SAVE_NEEDS_BYTE_SWAP: u32 = 0x0040_1500;
const PACKAGE_DATA_SWAP: u32 = 0x0067_0760;
const LOCATION_SAVE: u32 = 0x0067_f6c0;
const SCHEDULE_SAVE: u32 = 0x0067_fec0;
const TARGET_SAVE: u32 = 0x0068_02f0;
const CONDITIONS_SAVE: u32 = 0x0068_0910;
const IDLE_COLLECTION_SAVE: u32 = 0x0047_99f0;
const EVENT_ACTION_SAVE: u32 = 0x0067_dc80;
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;

/// Swaps the package data at `+0x1C` when the save needs its byte order converted.
fn swap_package_data(e: &mut Engine, this: Ptr) {
    if e.call(SAVE_NEEDS_BYTE_SWAP, &args![]).u8() != 0 {
        e.call(PACKAGE_DATA_SWAP, &args![this.addr() + 0x1c]);
    }
}

// Translated from 006738b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::Save` (Xbox PDB slot name `TESForm::Save`, vtable +0x2C; at +0x2C of
/// `TESPackage`'s PC vtable `0106847c`; `thiscall`): starts the form, writes `PKDT` (the
/// 0xC bytes at `+0x1C`, byte-swapped before and back after when the save needs it), then
/// each part it has: `PLDT` (location), `PLD2` (second location), the schedule, `PTDT`
/// (target), the conditions, the idle collection and `CNAM` (the form's id word,
/// `0084e3a0`); the package data's slot +0x0C when there is one; the event actions `POBA`
/// (`+0x4C`), `POEA` (`+0x5C`) and `POCA` (`+0x6C`); and closes the form.
pub fn tes_package_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    swap_package_data(e, this);
    e.call(ADD_CHUNK_DATA, &args![CHUNK_PKDT, base + 0x1c, 0xcu32]);
    swap_package_data(e, this);
    for (get, chunk) in [
        (PACKAGE_LOCATION, CHUNK_PLDT),
        (SECOND_LOCATION, CHUNK_PLD2),
    ] {
        if e.call(get, &args![this]).u32() != 0 {
            let location = e.call(get, &args![this]).u32();
            e.call(LOCATION_SAVE, &args![location, chunk]);
        }
    }
    if e.call(PACKAGE_SCHEDULE, &args![this]).u32() != 0 {
        let schedule = e.call(PACKAGE_SCHEDULE, &args![this]).u32();
        e.call(SCHEDULE_SAVE, &args![schedule]);
    }
    if e.call(PACKAGE_TARGET, &args![this]).u32() != 0 {
        let target = e.call(PACKAGE_TARGET, &args![this]).u32();
        e.call(TARGET_SAVE, &args![target, CHUNK_PTDT]);
    }
    for (get, save) in [
        (PACKAGE_CONDITIONS, CONDITIONS_SAVE),
        (IDLE_COLLECTION, IDLE_COLLECTION_SAVE),
    ] {
        if e.call(get, &args![this]).u32() != 0 {
            let part = e.call(get, &args![this]).u32();
            e.call(save, &args![part]);
        }
    }
    if e.call(PACKAGE_FORM, &args![this]).u32() != 0 {
        let form = e.call(PACKAGE_FORM, &args![this]).u32();
        let id = e.call(FORM_ID_0084E3A0, &args![form]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_CNAM, id]);
    }
    let data = e.mem.u32(base + 0x28);
    if data != 0 {
        e.vcall(data, 0xc, &[]);
    }
    for (offset, chunk) in [
        (0x4cu32, CHUNK_POBA),
        (0x5c, CHUNK_POEA),
        (0x6c, CHUNK_POCA),
    ] {
        e.call(EVENT_ACTION_SAVE, &args![base + offset, chunk]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_37a0, fn_006737a0(Ptr, Ptr) -> bool),
        entry!(0x0067_3890, fn_00673890(Ptr) -> bool),
        entry!(0x0067_3a40, fn_00673a40(Ptr, Ptr) -> bool),
        entry!(0x0067_3420, tes_package_compare(Ptr, u32) -> bool),
        entry!(0x0067_38b0, tes_package_save(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    /// The package (+0 type, +4 location, +0x24 flags); the location holds
    /// its type at +0, the actor its check value at +0.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(PACKAGE_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PACKAGE_LOCATION, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PACKAGE_LOCATION_GET_LOC_TYPE, |e, a| {
            e.mem.u32(a[0]).into_ret()
        });
        e.register(ACTOR_CHECK_00569B80, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    fn package(e: &mut Engine, kind: u32, location_type: Option<u32>, flags: u16) -> u32 {
        let package = e.mem.alloc(0x28);
        e.mem.set_u32(package, kind);
        if let Some(location_type) = location_type {
            let location = e.mem.alloc(4);
            e.mem.set_u32(location, location_type);
            e.mem.set_u32(package + 4, location);
        }
        e.mem.set_u16(package + 0x24, flags);
        package
    }

    #[test]
    fn fn_00673890_tests_bit_0x40() {
        let mut e = engine();
        let with = package(&mut e, 0, None, 0x41);
        let without = package(&mut e, 0, None, 0xffbf);
        assert!(e.call(0x0067_3890, &args![with]).bool());
        assert!(!e.call(0x0067_3890, &args![without]).bool());
    }

    #[test]
    fn fn_006737a0_follows_the_package_type() {
        let mut e = engine();
        let zero = e.mem.alloc(4);
        let one = e.mem.alloc(4);
        e.mem.set_u32(one, 1);
        let run = |e: &mut Engine, package: u32, actor: u32| {
            e.call(0x0067_37a0, &args![package, actor]).bool()
        };
        for kind in [6u32, 0xd] {
            let p = package(&mut e, kind, Some(6), 0);
            assert!(!run(&mut e, p, zero));
            assert!(run(&mut e, p, one));
            let p = package(&mut e, kind, Some(5), 0);
            assert!(run(&mut e, p, zero));
        }
        // Type 0xC with the flag: only the actor check decides.
        let p = package(&mut e, 0xc, None, 0x40);
        assert!(!run(&mut e, p, zero));
        assert!(run(&mut e, p, one));
        // Without the flag: no location, or a location not of type 6, is true.
        let p = package(&mut e, 0xc, None, 0);
        assert!(run(&mut e, p, zero));
        let p = package(&mut e, 0xc, Some(2), 0);
        assert!(run(&mut e, p, zero));
        let p = package(&mut e, 0xc, Some(6), 0);
        assert!(!run(&mut e, p, zero));
        assert!(run(&mut e, p, one));
        // Any other type is true.
        let p = package(&mut e, 7, Some(6), 0x40);
        assert!(run(&mut e, p, zero));
    }
}

#[cfg(test)]
mod b028_load_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Chunk reader doubles over (tag, data) chunks, with every callee of
    /// the reader doubled (constructors return their block, `0041ca90`
    /// the package type at +0x80 of the package).
    fn load_engine(record: u32, chunks: Vec<(u32, Vec<u8>)>, swap: bool) -> Engine {
        let mut e = Engine::new();
        let list = Rc::new(chunks);
        let current = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| record.into_ret());
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            l.get(c.get()).map_or(0, |chunk| chunk.0).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(NEXT_CHUNK, move |_, _| {
            c.set(c.get() + 1);
            u32::from(c.get() < l.len()).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(CHUNK_SIZE, move |_, _| {
            (l[c.get()].1.len() as u32).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = &l[c.get()].1;
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let (l, c) = (list, current);
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.write(a[1], &l[c.get()].1[..4]);
            Ret::default()
        });
        e.register_double(NEEDS_BYTE_SWAP, move |_, _| u32::from(swap).into_ret());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(PACKAGE_TYPE, |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        for constructor in [
            0x0067_b210,
            0x0067_c040,
            0x0067_bc60,
            0x0067_cc20,
            0x0067_cb70,
            0x0067_c450,
            PACKAGE_LOCATION_FROM_DATA,
            PACKAGE_TARGET_FROM_DATA,
            0x0047_9790,
            0x0067_fcf0,
        ] {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(0x0087_eaa0, |_, a| (a[0] + 0x40).into_ret());
        for callee in [
            TES_FORM_LOAD_FORM,
            TEMPORARY_DONE,
            PACKAGE_EVENT_ACTION_LOAD,
            PACKAGE_LOCATION_DESTRUCT,
            0x0067_0760,
            0x0067_eff0,
            0x0067_b100,
            0x0050_3210,
            0x0067_ca10,
            0x0067_ff10,
            0x0060_ce80,
            0x0067_0fc0,
            0x0067_1d30,
            0x0067_1e10,
            0x0067_2fc0,
            0x0067_3180,
            0x0067_3400,
            0x0068_0960,
            0x0047_9c40,
            0x007b_3fa0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // The package's vtable: EDID (0x134) and OBND (0xE0); the package
        // data's slot 0 deletes.
        e.put_vtable(0x0200_0000, &[0; 0x50]);
        e.mem.set_u32(0x0200_0000 + 0x134, 0x0090_0000);
        e.mem.set_u32(0x0200_0000 + 0xe0, 0x0090_0010);
        e.put_vtable(0x0200_1000, &[0x0090_0020]);
        for target in [0x0090_0000, 0x0090_0010, 0x0090_0020] {
            e.register(target, |_, _| Ret::default());
        }
        e
    }

    fn package(e: &mut Engine, kind: u32) -> u32 {
        let p = e.mem.alloc(0x90);
        e.mem.set_u32(p, 0x0200_0000);
        e.mem.set_u32(p + 0x80, kind);
        p
    }

    fn old_data(e: &mut Engine, package: u32) -> u32 {
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data, 0x0200_1000);
        e.mem.set_u32(package + 0x28, data);
        data
    }

    #[test]
    fn the_reader_refuses_other_records() {
        let mut e = load_engine(0x48, vec![], false);
        let p = package(&mut e, 0);
        assert!(!e.call(0x0067_3a40, &args![p, 0x7000u32]).bool());
    }

    #[test]
    fn the_reader_handles_the_package_chunks() {
        let chunks = vec![
            (CHUNK_EDID, b"Pkg\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_PKDT, vec![1, 2, 3, 4, 0xfe, 6, 7, 8, 9, 10, 11, 12]),
            (CHUNK_PLDT, vec![0; 12]),
            (CHUNK_PTDT, vec![0; 16]),
            (CHUNK_PSDT, vec![0; 8]),
            (CHUNK_CNAM, vec![0x44, 0, 0, 0]),
            (CHUNK_CTDA, vec![0; 28]),
            (CHUNK_IDLA, vec![0; 4]),
            (CHUNK_IDLT, vec![0; 4]),
            (CHUNK_POBA, vec![]),
            (0x1234, vec![]),
            (CHUNK_POEA, vec![]),
            (0x1234, vec![]),
            (CHUNK_POCA, vec![]),
            (0x1234, vec![]),
            (0, vec![]),
        ];
        let mut e = load_engine(0x49, chunks, true);
        let p = package(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0067_3a40, &args![p, 0x7000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, TES_FORM_LOAD_FORM), vec![vec![p, 0x7000]]);
        assert_eq!(calls_to(&log, 0x0090_0000).len(), 1);
        assert_eq!(calls_to(&log, 0x0090_0010), vec![vec![p, 0x7000]]);
        // PKDT: the signed byte +4 and the three words.
        assert_eq!(calls_to(&log, 0x0067_0fc0), vec![vec![p, 0xffff_fffe]]);
        assert_eq!(
            e.mem.bytes(p + 0x1c, 12),
            vec![1, 2, 3, 4, 0xfe, 6, 7, 8, 9, 10, 11, 12]
        );
        assert_eq!(calls_to(&log, 0x0067_0760).len(), 1);
        // The stack temporaries are made, used and destroyed.
        let location = calls_to(&log, 0x0067_1d30)[0][1];
        assert_eq!(
            calls_to(&log, PACKAGE_LOCATION_DESTRUCT),
            vec![vec![location]]
        );
        let target = calls_to(&log, 0x0067_2fc0)[0][1];
        let value = calls_to(&log, 0x0067_3180)[0][1];
        assert_eq!(
            calls_to(&log, TEMPORARY_DONE),
            vec![vec![target], vec![value]]
        );
        assert_eq!(calls_to(&log, 0x0067_3400), vec![vec![p, 0x44]]);
        assert_eq!(calls_to(&log, 0x0068_0960), vec![vec![p + 0x40, 0x7000]]);
        // The idle collection is made once and loads twice.
        let collection = e.mem.u32(p + 0x34);
        assert_ne!(collection, 0);
        assert_eq!(calls_to(&log, 0x0047_9790).len(), 1);
        assert_eq!(
            calls_to(&log, 0x0047_9c40),
            vec![vec![collection, 0x7000], vec![collection, 0x7000]]
        );
        assert_eq!(
            calls_to(&log, PACKAGE_EVENT_ACTION_LOAD),
            vec![
                vec![p + 0x4c, 0x7000],
                vec![p + 0x5c, 0x7000],
                vec![p + 0x6c, 0x7000]
            ]
        );
    }

    #[test]
    fn the_reader_replaces_the_package_data() {
        let cases: Vec<(u32, Vec<u8>, u32, u32, usize, bool)> = vec![
            (CHUNK_PKDD, (1..=24).collect(), 0x0067_b210, 0x20, 6, true),
            (
                CHUNK_PKFD,
                vec![0x78, 0x56, 0x34, 0x12],
                0x0067_c040,
                0xc,
                1,
                true,
            ),
            (CHUNK_PKE2, vec![1, 0, 0, 0], 0x0067_bc60, 0xc, 1, true),
            (CHUNK_PKW2, (1..=20).collect(), 0x0067_cc20, 0x24, 5, false),
            (CHUNK_PKW3, (1..=24).collect(), 0x0067_cb70, 0x24, 6, false),
            (CHUNK_PKPT, vec![0x34, 0x12], 0x0067_c450, 8, 1, false),
        ];
        for (chunk, data, constructor, size, words, clears) in cases {
            let mut e = load_engine(0x49, vec![(chunk, data.clone())], false);
            let p = package(&mut e, 0);
            let old = old_data(&mut e, p);
            // What +0x28 holds when the new data is allocated.
            let seen = Rc::new(Cell::new(0xffff_ffffu32));
            let s = seen.clone();
            e.register_double(OPERATOR_NEW, move |e, a| {
                s.set(e.mem.u32(p + 0x28));
                e.mem.alloc(a[0]).into_ret()
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x0067_3a40, &args![p, 0x7000u32]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0090_0020), vec![vec![old, 1]]);
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![size]]);
            let made = calls_to(&log, constructor);
            assert_eq!(made.len(), 1);
            assert_eq!(made[0].len(), 1 + words);
            assert_eq!(e.mem.u32(p + 0x28), made[0][0]);
            match chunk {
                CHUNK_PKFD => assert_eq!(made[0][1], 0x1234_5678),
                CHUNK_PKPT => assert_eq!(made[0][1], 0x1234),
                CHUNK_PKDD => assert_eq!(made[0][1], 0x0403_0201),
                _ => {}
            }
            assert_eq!(seen.get(), if clears { 0 } else { old });
        }
    }

    #[test]
    fn the_second_location_and_target_follow_the_package_type() {
        for (kind, field) in [(0xf, 0xc), (0x1c, 0xc), (0x10, 8)] {
            let mut e = load_engine(0x49, vec![(CHUNK_PLD2, vec![0; 12])], false);
            let p = package(&mut e, kind);
            let data = old_data(&mut e, p);
            e.call_log = Some(vec![]);
            e.call(0x0067_3a40, &args![p, 0x7000u32]);
            let log = e.call_log.take().unwrap();
            let made = calls_to(&log, PACKAGE_LOCATION_FROM_DATA);
            assert_eq!(e.mem.u32(data + field), made[0][0]);
            assert!(calls_to(&log, 0x0067_1e10).is_empty());
        }
        let mut e = load_engine(0x49, vec![(CHUNK_PLD2, vec![0; 12])], false);
        let p = package(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x0067_3a40, &args![p, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let location = calls_to(&log, 0x0067_1e10)[0][1];
        assert_eq!(
            calls_to(&log, PACKAGE_LOCATION_DESTRUCT),
            vec![vec![location]]
        );

        let mut e = load_engine(0x49, vec![(CHUNK_PTD2, vec![0; 16])], false);
        let p = package(&mut e, 0x10);
        let data = old_data(&mut e, p);
        e.mem.set_u32(data + 4, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0067_3a40, &args![p, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x007b_3fa0), vec![vec![0x7777, 1]]);
        let made = calls_to(&log, PACKAGE_TARGET_FROM_DATA);
        assert_eq!(e.mem.u32(data + 4), made[0][0]);
        // Any other type ignores PTD2.
        let mut e = load_engine(0x49, vec![(CHUNK_PTD2, vec![0; 16])], false);
        let p = package(&mut e, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x0067_3a40, &args![p, 0x7000u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), PACKAGE_TARGET_FROM_DATA).is_empty());
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const GETTERS: [u32; 9] = [
        PACKAGE_TYPE,
        GETTER_00441110,
        IDLE_COLLECTION,
        PACKAGE_LOCATION,
        SECOND_LOCATION,
        PACKAGE_TARGET,
        PACKAGE_SCHEDULE,
        PACKAGE_CONDITIONS,
        PACKAGE_FORM,
    ];

    /// Two packages whose getters answer the word at `+0x70` (every part the same
    /// value), and comparisons that all say "same".
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let other = e.mem.alloc(0x80);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            if a[0] == other {
                other.into_ret()
            } else {
                Ret::default()
            }
        });
        for getter in GETTERS {
            e.register(getter, |e, a| e.mem.u32(a[0] + 0x70).into_ret());
        }
        for compare in [
            COMPARE_ALL_COMPONENTS,
            LOCATION_COMPARE,
            TARGET_COMPARE,
            SCHEDULE_COMPARE,
            CONDITIONS_COMPARE,
            EVENT_ACTION_COMPARE,
        ] {
            e.register(compare, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this, other)
    }

    fn differs(e: &mut Engine, this: u32, form: u32) -> bool {
        e.call(0x0067_3420, &args![this, form]).u8() != 0
    }

    #[test]
    fn packages_without_parts_compare_equal() {
        let (mut e, this, other) = engine();
        assert!(differs(&mut e, this, 0x1234));
        assert!(!differs(&mut e, this, other));
        assert_eq!(calls_to(&e, EVENT_ACTION_COMPARE).len(), 3);
        e.mem.set_u16(other + 0x24, 1);
        assert!(differs(&mut e, this, other));
        e.mem.set_u16(other + 0x24, 0);
        // Only one has package data.
        e.mem.set_u32(other + 0x28, 0x5000);
        assert!(differs(&mut e, this, other));
    }

    #[test]
    fn parts_are_compared_through_their_comparisons() {
        let (mut e, this, other) = engine();
        // Both have every part: a vtable object answering "same" at +0x0C, +0x10C, +0x08.
        let part = e.mem.alloc(8);
        let table = e.mem.alloc(0x110);
        for slot in [0x8u32, 0xc, 0x10c] {
            e.mem.set_u32(table + slot, 0x0153_0000 + slot);
            e.register(0x0153_0000 + slot, |_, _| Ret::default());
        }
        e.mem.set_u32(part, table);
        e.mem.set_u32(this + 0x70, part);
        e.mem.set_u32(other + 0x70, part);
        e.mem.set_u32(this + 0x28, part);
        e.mem.set_u32(other + 0x28, part);
        assert!(!differs(&mut e, this, other));
        assert_eq!(calls_to(&e, TARGET_COMPARE), vec![vec![part, part]]);
        assert_eq!(calls_to(&e, LOCATION_COMPARE).len(), 2);
        assert_eq!(calls_to(&e, 0x0153_010c).len(), 1);
        assert_eq!(calls_to(&e, 0x0153_0008).len(), 1);
        // A differing schedule stops there.
        e.register(SCHEDULE_COMPARE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(differs(&mut e, this, other));
        assert!(calls_to(&e, CONDITIONS_COMPARE).is_empty());
    }

    #[test]
    fn save_writes_the_chunks_of_the_parts_it_has() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        for getter in GETTERS {
            e.register(getter, |e, a| e.mem.u32(a[0] + 0x70).into_ret());
        }
        for callee in [
            START_FORM,
            CLOSE_FORM,
            ADD_CHUNK_DATA,
            ADD_CHUNK_WORD,
            PACKAGE_DATA_SWAP,
            LOCATION_SAVE,
            SCHEDULE_SAVE,
            TARGET_SAVE,
            CONDITIONS_SAVE,
            IDLE_COLLECTION_SAVE,
            EVENT_ACTION_SAVE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(SAVE_NEEDS_BYTE_SWAP, |_, _| 1u32.into_ret());
        e.register(FORM_ID_0084E3A0, |_, _| 0x0012_3456u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0067_38b0, &args![this]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_DATA),
            vec![vec![CHUNK_PKDT, this + 0x1c, 0xc]]
        );
        assert_eq!(calls_to(&e, PACKAGE_DATA_SWAP).len(), 2);
        assert!(calls_to(&e, LOCATION_SAVE).is_empty());
        assert!(calls_to(&e, ADD_CHUNK_WORD).is_empty());
        assert_eq!(
            calls_to(&e, EVENT_ACTION_SAVE)[2],
            vec![this + 0x6c, CHUNK_POCA]
        );
        // With every part.
        e.mem.set_u32(this + 0x70, 0x6000);
        e.register(SAVE_NEEDS_BYTE_SWAP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_38b0, &args![this]);
        assert_eq!(
            calls_to(&e, LOCATION_SAVE),
            vec![vec![0x6000, CHUNK_PLDT], vec![0x6000, CHUNK_PLD2]]
        );
        assert_eq!(calls_to(&e, TARGET_SAVE), vec![vec![0x6000, CHUNK_PTDT]]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_WORD),
            vec![vec![CHUNK_CNAM, 0x0012_3456]]
        );
        assert_eq!(calls_to(&e, IDLE_COLLECTION_SAVE), vec![vec![0x6000]]);
        assert!(calls_to(&e, PACKAGE_DATA_SWAP).is_empty());
        assert_eq!(calls_to(&e, CLOSE_FORM), vec![vec![this]]);
    }
}

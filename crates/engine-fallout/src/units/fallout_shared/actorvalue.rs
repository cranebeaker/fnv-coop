//! `fallout shared/actorvalue.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066fe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066fe40 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x59 (0xc4 bytes); `ECX` is the object,
/// `id` the actor value index and `name` the name string pointer (`RET 8`). The engine map gives it no name; the RTTI name
/// of the vtable it stores at `+0` (`0x0106_7a2c`) is `ActorValueInfo`, so this is the constructor of `ActorValueInfo`.
/// It runs the base and component constructors by address, stores the four vtable pointers and registers the form type
/// (`004f15a0`). When `id` is not -1 it gives the form the form ID `0066fdf0(id)` (`00485c10(this, formID, 0)`). It keeps
/// `name` at `+0x38`; when `name` is not null it formats `"AV%s"` (`010679d0`) into a 0x64-byte stack buffer
/// (`00406d00`) and passes the buffer to `00401290`. It assigns the empty string at `01011584` to the string at `+0x3c`
/// (`00438390`), clears `+0x44`, `+0x4c..+0x58` and `+0x98`, fills the fifteen words from `+0x5c` with 0x4d and zeroes the
/// ten words from `+0x9c`. The exception-unwinding frame and the security-cookie check are not translated.
pub fn fn_0066fe40(e: &mut Engine, this: Ptr, id: u32, name: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0040_2c20, &args![base + 0x2c]);
    e.mem.set_u32(base, 0x0106_7a2c);
    e.mem.set_u32(base + 0x18, 0x0106_7a14);
    e.mem.set_u32(base + 0x24, 0x0106_79fc);
    e.mem.set_u32(base + 0x2c, 0x0106_79dc);
    e.call(0x0040_37b0, &args![base + 0x3c]);
    e.call(0x004f_15a0, &args![this, 0x59u32]);
    if id != 0xffff_ffff {
        let form_id = e.call(0x0066_fdf0, &args![id]).u32();
        e.call(0x0048_5c10, &args![this, form_id, 0u32]);
    }
    e.mem.set_u32(base + 0x38, name);
    if name != 0 {
        e.with_stack(0x64, |e, buffer| {
            e.call(0x0040_6d00, &args![buffer, 0x64u32, 0x0106_79d0u32, name]);
            e.call(0x0040_1290, &args![this, buffer]);
        });
    }
    e.call(0x0043_8390, &args![base + 0x3c, 0x0101_1584u32]);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x58, 0);
    for i in 0..15 {
        e.mem.set_u32(base + 0x5c + i * 4, 0x4d);
    }
    e.mem.set_u32(base + 0x98, 0);
    for i in 0..10 {
        e.mem.set_u32(base + 0x9c + i * 4, 0);
    }
    this
}

// Translated from 0066eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name; `tesobjectweap.rs` calls it as a
/// resistance validity check (unconfirmed here). `cdecl`, one `uint` (an
/// actor value index): true for 0x12, 0x13, 0x14, 0x34 and 0x3a to 0x3d (a
/// jump table over `index - 0x12`, 44 entries), false for every other value.
pub fn fn_0066eba0(_e: &mut Engine, index: u32) -> bool {
    matches!(index, 0x12..=0x14 | 0x34 | 0x3a..=0x3d)
}

// Translated from 0066ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `actorvalue.cpp`): clears the first 0x4d words (0x134
/// bytes) of `this`, signed loop. Returns `this`.
pub fn fn_0066ef90(e: &mut Engine, this: Ptr) -> Ptr {
    for index in 0..0x4d_u32 {
        e.mem.set_u32(this.addr() + index * 4, 0);
    }
    this
}

layout! {
    /// `ActorValueInfo` (Xbox PDB, 0xD4 bytes there; 0xC4 on PC, every field
    /// 0x10 lower because `TESForm` is 0x18 bytes on PC), the fields used
    /// here.
    pub struct ActorValueInfo: 0xC4 {
        /// `sScriptName` (Xbox PDB): `char*`.
        0x38 sScriptName: u32,
        /// `uFlags` (Xbox PDB).
        0x44 uFlags: u32,
        /// `eAVType` (Xbox PDB): `ActorValue::Section`.
        0x48 eAVType: u32,
        /// `pDerivationFunction` (Xbox PDB).
        0x4C pDerivationFunction: u32,
        /// `pMappingFunction` (Xbox PDB).
        0x50 pMappingFunction: u32,
        /// `pModifiedCallback` (Xbox PDB).
        0x54 pModifiedCallback: u32,
        /// `iNumDependentActorValues` (Xbox PDB).
        0x58 iNumDependentActorValues: u32,
        /// `iEnumCount` (Xbox PDB).
        0x98 iEnumCount: u32,
    }
}

/// `DependentActorValues` (Xbox PDB): 15 `int`s from +0x5C.
const DEPENDENT_ACTOR_VALUES: u32 = 0x5c;
/// `sEnumNames` (Xbox PDB): 10 `char*` from +0x9C.
const ENUM_NAMES: u32 = 0x9c;
/// The registered `ActorValueInfo*` of each of the 0x4D actor values.
const ACTOR_VALUE_INFOS: u32 = 0x011d_61c8;
/// Set once `RegisterActorValues` has run (cleared by `0066f110`).
const ACTOR_VALUES_REGISTERED: u32 = 0x011d_6154;
/// `0066e920` (`cdecl`, unit `run_0066e910`): the info of an actor value, 0
/// outside `0..0x4d`.
const ACTOR_VALUE_INFO: u32 = 0x0066_e920;
/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `004a8f20` (`cdecl`): the smaller of two `int`s.
const MIN_INT: u32 = 0x004a_8f20;
/// `_ftol2` (`00ec62c0`), the value in ST0 as a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `00408da0` (`thiscall`; the engine map's `MapMarkerData::GetLocationName`
/// is folded code): the name string of a `TESFullName` or texture
/// component.
const COMPONENT_NAME: u32 = 0x0040_8da0;
/// `0044a670` (`cdecl`): the length of a C string.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `00559450` (`thiscall`): the characters of a `BSStringT`.
const STRING_CHARS: u32 = 0x0055_9450;
/// `00406d70` (`cdecl`: actor value, flag mask): whether the actor value's
/// info has the flag.
const HAS_FLAG: u32 = 0x0040_6d70;
/// `0040ebd0` (`cdecl`, two `float`s): their minimum, in ST0.
const MIN_FLOAT: u32 = 0x0040_ebd0;
/// `00404010` (`cdecl`, two `float`s): their maximum, in ST0.
const MAX_FLOAT: u32 = 0x0040_4010;
/// `float` 100.0 (`01016410`).
const HUNDRED_FLOAT: u32 = 0x0101_6410;
/// `float` 10.0 (`01017b78`).
const TEN_FLOAT: u32 = 0x0101_7b78;
/// `"DESC"` as a little-endian chunk word.
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// The info of actor value `index` through `0066e920`.
fn info_of(e: &mut Engine, index: u32) -> u32 {
    e.call(ACTOR_VALUE_INFO, &args![index]).u32()
}

// Translated from 0066e950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetActorValueName` (Xbox PDB, matched by the engine map;
/// `cdecl`): the full name (`TESFullName` at +0x18) when it is set and not
/// empty, otherwise the script name (+0x38); 0 without an info.
pub fn actor_value_get_actor_value_name(e: &mut Engine, index: u32) -> u32 {
    let info = info_of(e, index);
    if info == 0 {
        return 0;
    }
    if e.call(COMPONENT_NAME, &args![info + 0x18]).u32() != 0 {
        let name = e.call(COMPONENT_NAME, &args![info + 0x18]).u32();
        if e.call(STRING_LENGTH, &args![name]).u32() != 0 {
            return e.call(COMPONENT_NAME, &args![info + 0x18]).u32();
        }
    }
    e.get(
        Ptr::<ActorValueInfo>::new(info),
        ActorValueInfo::sScriptName,
    )
}

// Translated from 0066e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetActorValueDesc` (Xbox PDB, matched by the engine map;
/// `cdecl`): the description of the `TESDescription` at +0x24 (its virtual
/// slot 0x10 with 0 and the chunk `DESC`); 0 without an info.
pub fn actor_value_get_actor_value_desc(e: &mut Engine, index: u32) -> u32 {
    let info = info_of(e, index);
    if info == 0 {
        return 0;
    }
    e.vcall(info + 0x24, 0x10, &args![0u32, CHUNK_DESC]).u32()
}

// Translated from 0066e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetActorValueIcon` (Xbox PDB, matched by the engine map;
/// `cdecl`): the icon path of the texture at +0x2C (`00408da0`); 0 without
/// an info.
pub fn actor_value_get_actor_value_icon(e: &mut Engine, index: u32) -> u32 {
    let info = info_of(e, index);
    if info == 0 {
        return 0;
    }
    e.call(COMPONENT_NAME, &args![info + 0x2c]).u32()
}

// Translated from 0066eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetActorValueAbbreviation` (Xbox PDB, matched by the engine
/// map; `cdecl`): the characters of `sAbbreviation` (+0x3C); 0 without an
/// info.
pub fn actor_value_get_actor_value_abbreviation(e: &mut Engine, index: u32) -> u32 {
    let info = info_of(e, index);
    if info == 0 {
        return 0;
    }
    e.call(STRING_CHARS, &args![info + 0x3c]).u32()
}

// Translated from 0066ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::ToActorValue` (Xbox PDB, matched by the engine map;
/// `cdecl`): the actor value of entry `index` of `section` (a jump table on
/// the section): 0 → `index + 5`, 1 → `+ 0xc`, 2 → `+ 0x20`, 3 → `index`,
/// 4 → `+ 0x19`, 5 → `+ 0x2e`; any other section 0. `index` is a `char`.
pub fn actor_value_to_actor_value(_e: &mut Engine, section: u32, index: i8) -> i32 {
    let index = i32::from(index);
    match section {
        0 => index + 5,
        1 => index + 0xc,
        2 => index + 0x20,
        3 => index,
        4 => index + 0x19,
        5 => index + 0x2e,
        _ => 0,
    }
}

// Translated from 0066ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of [`actor_value_to_actor_value`] (`cdecl`; no Xbox PDB
/// name): the entry of actor value `value` in `section`, as a `char` in
/// `AL` (`value - 5`, `- 0xc`, `- 0x20`, itself, `- 0x19`, `- 0x2e` for
/// sections 0 to 5), 0 for any other section.
pub fn fn_0066ec80(_e: &mut Engine, section: u32, value: u32) -> u8 {
    match section {
        0 => value.wrapping_sub(5) as u8,
        1 => value.wrapping_sub(0xc) as u8,
        2 => value.wrapping_sub(0x20) as u8,
        3 => value as u8,
        4 => value.wrapping_sub(0x19) as u8,
        5 => value.wrapping_sub(0x2e) as u8,
        _ => 0,
    }
}

// Translated from 0066ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetSectionCount` (Xbox PDB, matched by the engine map;
/// `cdecl`): the number of actor values of a section (7, 0x14, 0xe, 5, 7,
/// 0x10 for sections 0 to 5), 0 otherwise.
pub fn actor_value_get_section_count(_e: &mut Engine, section: u32) -> u32 {
    match section {
        0 => 7,
        1 => 0x14,
        2 => 0xe,
        3 => 5,
        4 => 7,
        5 => 0x10,
        _ => 0,
    }
}

// Translated from 0066ed60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Derives an actor value (`cdecl`: owner, actor value, `float*` result; no
/// Xbox PDB name): `*result` becomes 0, and when the info has a derivation
/// function (+0x4C), its `float` for `(owner, index)`, answering true.
pub fn fn_0066ed60(e: &mut Engine, owner: u32, index: u32, result: Ptr) -> bool {
    e.mem.set_u32(result.addr(), x87_store32(0.0));
    let info = info_of(e, index);
    if info == 0 {
        return false;
    }
    let info = Ptr::<ActorValueInfo>::new(info);
    if e.get(info, ActorValueInfo::pDerivationFunction) == 0 {
        return false;
    }
    let derive = e.get(info, ActorValueInfo::pDerivationFunction);
    let value = e.call(derive, &args![owner, index]).f64();
    e.mem.set_u32(result.addr(), x87_store32(value));
    true
}

// Translated from 0066edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetDependentActorValues` (Xbox PDB, matched by the engine
/// map; `cdecl`): the address of the info's dependent list (+0x5C) with
/// its count in `*count`; 0 and a count of 0 without an info.
pub fn actor_value_get_dependent_actor_values(e: &mut Engine, index: u32, count: Ptr) -> u32 {
    e.mem.set_u32(count.addr(), 0);
    let info = info_of(e, index);
    if info == 0 {
        return 0;
    }
    let n = e.get(
        Ptr::<ActorValueInfo>::new(info),
        ActorValueInfo::iNumDependentActorValues,
    );
    e.mem.set_u32(count.addr(), n);
    info + DEPENDENT_ACTOR_VALUES
}

// Translated from 0066ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor value has a modified callback (+0x54) (`cdecl`; no
/// Xbox PDB name).
pub fn fn_0066ee10(e: &mut Engine, index: u32) -> bool {
    let info = info_of(e, index);
    info != 0
        && e.get(
            Ptr::<ActorValueInfo>::new(info),
            ActorValueInfo::pModifiedCallback,
        ) != 0
}

// Translated from 0066ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the actor value's modified callback (+0x54) when it has one
/// (`cdecl`: owner, actor value, old value, delta, one more word passed on;
/// no Xbox PDB name).
pub fn fn_0066ee50(e: &mut Engine, owner: u32, index: u32, old: f32, delta: f32, extra: u32) {
    let info = info_of(e, index);
    if info == 0 {
        return;
    }
    let info = Ptr::<ActorValueInfo>::new(info);
    if e.get(info, ActorValueInfo::pModifiedCallback) == 0 {
        return;
    }
    let callback = e.get(info, ActorValueInfo::pModifiedCallback);
    let old = x87_quiet_bits(old.to_bits());
    let delta = x87_quiet_bits(delta.to_bits());
    e.call(callback, &args![owner, index, old, delta, extra]);
}

// Translated from 0066ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueOwner::GetClampedActorValue` (Xbox PDB, matched by the engine
/// map): [`actor_value_owner_get_clamped_actor_float_value`] stored to a
/// `float`, through `00406ce0`, truncated to an `int` (`_ftol2`).
pub fn actor_value_owner_get_clamped_actor_value(e: &mut Engine, this: Ptr, index: u32) -> i32 {
    let value = actor_value_owner_get_clamped_actor_float_value(e, this, index);
    let rounded = e.call(0x0040_6ce0, &args![x87_store32(value)]).f64();
    e.call(FTOL, &args![rounded]).i32()
}

// Translated from 0066ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueOwner::GetClampedActorFloatValue` (Xbox PDB, matched by the
/// engine map): the owner's value (virtual slot 0xC) clamped by
/// [`actor_value_clamp_actor_value`], as a `float` in ST0.
pub fn actor_value_owner_get_clamped_actor_float_value(
    e: &mut Engine,
    this: Ptr,
    index: u32,
) -> f64 {
    let value = e.vcall(this.addr(), 0xc, &args![index]).f64();
    let bits = x87_quiet_bits(x87_store32(value));
    let clamped = actor_value_clamp_actor_value(e, index, f32::from_bits(bits));
    x87_load32(x87_store32(clamped))
}

// Translated from 0066efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: [`fn_0066f110`] (it takes no arguments; `ECX` is
/// passed on unused).
pub fn fn_0066efd0(e: &mut Engine, _this: Ptr) {
    fn_0066f110(e);
}

// Translated from 0066efe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066efe0 - the C++ exception unwinding states (FS:[0] frame, handler 00eefe9b) are not translated
/// Registers an actor value (`cdecl`, variadic; no Xbox PDB name): when
/// the table at `011d61c8` has no info for `index`, a 0xC4-byte block is
/// constructed as `ActorValueInfo(index, name)` ([`fn_0066fe40`]; null
/// when the allocation fails, and then the stores below write at low
/// addresses as in the exe) with its section, flags, derivation, mapping
/// and modified callback, and up to 15 dependent actor values from the
/// words after `count` (only when `count` is positive); the info goes into
/// the table.
pub fn fn_0066efe0(
    e: &mut Engine,
    index: u32,
    name: u32,
    section: u32,
    flags: u32,
    derivation: u32,
    mapping: u32,
    callback: u32,
    count: u32,
    dependents: &[u32],
) {
    let slot = ACTOR_VALUE_INFOS.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(slot) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0xc4u32]).u32();
    let info = if block == 0 {
        0
    } else {
        fn_0066fe40(e, Ptr::new(block), index, name).addr()
    };
    let info_ptr = Ptr::<ActorValueInfo>::new(info);
    e.set(info_ptr, ActorValueInfo::eAVType, section);
    e.set(info_ptr, ActorValueInfo::uFlags, flags);
    e.set(info_ptr, ActorValueInfo::pDerivationFunction, derivation);
    e.set(info_ptr, ActorValueInfo::pMappingFunction, mapping);
    e.set(info_ptr, ActorValueInfo::pModifiedCallback, callback);
    let n = e.call(MIN_INT, &args![count, 0xfu32]).u32();
    e.set(info_ptr, ActorValueInfo::iNumDependentActorValues, n);
    if count as i32 > 0 {
        let mut i = 0i32;
        while i < e.get(info_ptr, ActorValueInfo::iNumDependentActorValues) as i32 {
            let at = info
                .wrapping_add(DEPENDENT_ACTOR_VALUES)
                .wrapping_add((i as u32).wrapping_mul(4));
            e.mem.set_u32(at, dependents[i as usize]);
            i += 1;
        }
    }
    e.mem.set_u32(slot, info);
}

/// The registered form of [`fn_0066efe0`]: eight fixed words, then the
/// dependent actor values.
fn fn_0066efe0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(words.len() >= 8, "0066efe0 needs 8 argument words");
    fn_0066efe0(
        e,
        words[0],
        words[1],
        words[2],
        words[3],
        words[4],
        words[5],
        words[6],
        words[7],
        &words[8..],
    );
    Ret::default()
}

// Translated from 0066f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unregisters every actor value (no arguments; no Xbox PDB name): each
/// info in the table is deleted (virtual slot 0x10 with 1) and its entry
/// cleared, then the registered flag (`011d6154`) is cleared.
pub fn fn_0066f110(e: &mut Engine) {
    for i in 0..0x4du32 {
        let slot = ACTOR_VALUE_INFOS + 4 * i;
        if e.mem.u32(slot) != 0 {
            let info = e.mem.u32(slot);
            if info != 0 {
                e.vcall(info, 0x10, &args![1u32]);
            }
            e.mem.set_u32(slot, 0);
        }
    }
    e.mem.set_u8(ACTOR_VALUES_REGISTERED, 0);
}

// Translated from 0066f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::ClampActorValue` (Xbox PDB, matched by the engine map;
/// `cdecl`): with flag 0x10, `max(0, min(value, 100))`; otherwise with flag
/// 8, `max(low, min(value, 10))` where `low` is 1 with flag 0x8000 and 0
/// without; any other value is kept. In ST0, as a `float`.
pub fn actor_value_clamp_actor_value(e: &mut Engine, index: u32, value: f32) -> f64 {
    let mut local = x87_quiet_bits(value.to_bits());
    let ld = |e: &Engine, a: u32| x87_store32(x87_load32(e.mem.u32(a)));
    if e.call(HAS_FLAG, &args![index, 0x10u32]).bool() {
        let limit = ld(e, HUNDRED_FLOAT);
        let low = e.call(MIN_FLOAT, &args![local, limit]).f64();
        let r = e
            .call(MAX_FLOAT, &args![x87_store32(0.0), x87_store32(low)])
            .f64();
        local = x87_store32(r);
    } else if e.call(HAS_FLAG, &args![index, 8u32]).bool() {
        let floor = if e.call(HAS_FLAG, &args![index, 0x8000u32]).bool() {
            x87_store32(1.0)
        } else {
            x87_store32(0.0)
        };
        let limit = ld(e, TEN_FLOAT);
        let low = e.call(MIN_FLOAT, &args![local, limit]).f64();
        let r = e
            .call(MAX_FLOAT, &args![x87_quiet_bits(floor), x87_store32(low)])
            .f64();
        local = x87_store32(r);
    }
    x87_load32(local)
}

/// One `00670120` call of `RegisterActorValues`, on the info of `index`
/// when there is one.
fn register_enum_names(e: &mut Engine, index: u32, count: u32, names: &[u32]) {
    let info = info_of(e, index);
    if info != 0 {
        fn_00670120(e, info, count, names);
    }
}

// Translated from 0066f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::RegisterActorValues` (Xbox PDB, matched by the engine map):
/// once (flag `011d6154`), registers the 0x4D actor values with
/// [`fn_0066efe0`] in the exe's order, with their names (string
/// addresses), sections, flags, functions and dependents as the code
/// pushes them, and the enumeration names of Aggression, Assistance,
/// Confidence and Mood ([`fn_00670120`]). An empty loop over the 0x4D
/// values (left by the compiler) runs before the flag is set.
pub fn actor_value_register_actor_values(e: &mut Engine) {
    if e.mem.u8(ACTOR_VALUES_REGISTERED) != 0 {
        return;
    }
    fn_0066efe0(e, 0x0, 0x10679c4, 0x3, 0x101, 0x0, 0x0, 0x8ba140, 0x0, &[]);
    register_enum_names(e, 0x0, 0x4, &[0x1067998, 0x10679a8, 0x10679b4, 0x1064d04]);
    fn_0066efe0(e, 0x39, 0x106798c, 0x3, 0x101, 0x0, 0x0, 0x8ba210, 0x0, &[]);
    register_enum_names(e, 0x39, 0x3, &[0x1067950, 0x1067960, 0x1067970]);
    fn_0066efe0(e, 0x1, 0x1067944, 0x3, 0x101, 0x0, 0x0, 0x0, 0x0, &[]);
    register_enum_names(
        e,
        0x1,
        0x5,
        &[0x1067918, 0x1067924, 0x1016118, 0x1067930, 0x1067938],
    );
    fn_0066efe0(e, 0x2, 0x1067910, 0x3, 0x1, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3, 0x1067900, 0x3, 0x1, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x4, 0x10678f8, 0x3, 0x101, 0x0, 0x0, 0x0, 0x0, &[]);
    register_enum_names(
        e,
        0x4,
        0x9,
        &[
            0x101b090, 0x10678b8, 0x10678c0, 0x10678c8, 0x10678d0, 0x10678d8, 0x10678e4, 0x104c920,
            0x10678ec,
        ],
    );
    fn_0066efe0(
        e,
        0x5,
        0x10678ac,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0x3,
        &[0xd, 0x26, 0x11],
    );
    fn_0066efe0(
        e,
        0x6,
        0x10678a0,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0x3,
        &[0x22, 0x24, 0x23],
    );
    fn_0066efe0(
        e,
        0x7,
        0x1067894,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x8b9ea0,
        0x7,
        &[0x10, 0xf, 0x13, 0x14, 0x21, 0x2d, 0x2c],
    );
    fn_0066efe0(
        e,
        0x8,
        0x1067888,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0x2,
        &[0x20, 0x2b],
    );
    fn_0066efe0(
        e,
        0x9,
        0x1067878,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0x3,
        &[0x28, 0x27, 0x25],
    );
    fn_0066efe0(
        e,
        0xa,
        0x1067870,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0x3,
        &[0xc, 0x29, 0x2a],
    );
    fn_0066efe0(
        e,
        0xb,
        0x1067868,
        0x0,
        0x8009,
        0x0,
        0x0,
        0x0,
        0xf,
        &[
            0xe, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2d, 0x2c,
        ],
    );
    fn_0066efe0(e, 0xc, 0x1067858, 0x1, 0x0, 0x6439d0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0xd, 0x106784c, 0x1, 0x801, 0x643a10, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0xe, 0x1067840, 0x1, 0x800, 0x643b10, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0xf,
        0x1067834,
        0x1,
        0x801,
        0x643a70,
        0x0,
        0x8b9db0,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x10,
        0x105ac74,
        0x1,
        0x10c1,
        0x643670,
        0x0,
        0x8b9960,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x11, 0x1067828, 0x1, 0x800, 0x643b50, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x12,
        0x1067818,
        0x6,
        0x800,
        0x8a0a90,
        0x0,
        0x8ba270,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x13, 0x1067808, 0x6, 0x800, 0x643e40, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x14,
        0x10677fc,
        0x6,
        0x800,
        0x643e40,
        0x0,
        0x8ba2b0,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x34, 0x10677f0, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3a, 0x10677e0, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3b, 0x10677d4, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3c, 0x10677c4, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3d, 0x10677b8, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x15, 0x10677ac, 0x1, 0x801, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x16,
        0x10677a4,
        0x1,
        0x1841,
        0x643820,
        0x0,
        0x8b9960,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x19,
        0x1067790,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9240,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1a,
        0x106777c,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9240,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1b,
        0x1067768,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9240,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1c,
        0x1067750,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9240,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1d,
        0x1067738,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9830,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1e,
        0x1067720,
        0x4,
        0x821,
        0x643a50,
        0x0,
        0x8b9830,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x1f,
        0x1067710,
        0x4,
        0x820,
        0x643a50,
        0x0,
        0x8b9240,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x17, 0x1067708, 0x1, 0x4000, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x18,
        0x105f298,
        0x1,
        0x4000,
        0x0,
        0x0,
        0x8d53f0,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x38, 0x10676f8, 0x1, 0x800, 0x643ed0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x20, 0x10676f0, 0x2, 0x10, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x21, 0x101a9d4, 0x2, 0x2000, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x22, 0x101a9c4, 0x2, 0x410, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x23, 0x10676e4, 0x2, 0x410, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x24, 0x10676d8, 0x2, 0x10, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x25,
        0x10676cc,
        0x2,
        0x10,
        0x643bd0,
        0x0,
        0x8b9ab0,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x26, 0x101a9a8, 0x2, 0x410, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x27, 0x104ce58, 0x2, 0x10, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x28, 0x10676c4, 0x2, 0x10, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x29, 0x10676bc, 0x2, 0x410, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x2a, 0x1044fd8, 0x2, 0x11, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x2b, 0x10676b4, 0x2, 0x10, 0x643bd0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(
        e,
        0x2c,
        0x10676a8,
        0x2,
        0x10,
        0x643bd0,
        0x0,
        0x8b9b10,
        0x0,
        &[],
    );
    fn_0066efe0(
        e,
        0x2d,
        0x10676a0,
        0x2,
        0x410,
        0x643bd0,
        0x0,
        0x0,
        0x1,
        &[0x38],
    );
    fn_0066efe0(
        e,
        0x2e,
        0x1067690,
        0x1,
        0x1,
        0x8a0cd0,
        0x0,
        0x8ba2f0,
        0x0,
        &[],
    );
    fn_0066efe0(e, 0x2f, 0x1011ec8, 0x5, 0x1, 0x0, 0x0, 0x8b9cc0, 0x0, &[]);
    fn_0066efe0(e, 0x30, 0x1011f6c, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x31, 0x1011f60, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x32, 0x1067684, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x33, 0x1011e2c, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x35, 0x1067674, 0x5, 0x1, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x36, 0x1067664, 0x5, 0x220, 0x0, 0x0, 0x8c5350, 0x0, &[]);
    fn_0066efe0(e, 0x37, 0x1067658, 0x5, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3e, 0x106764c, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x3f, 0x1067640, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x40, 0x1067634, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x41, 0x1067628, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x42, 0x106761c, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x43, 0x1067610, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x44, 0x1067604, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x45, 0x10675f8, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x46, 0x10675ec, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x47, 0x10675e0, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, &[]);
    fn_0066efe0(e, 0x48, 0x10675cc, 0x7, 0x1, 0x0, 0x0, 0x8b9240, 0x0, &[]);
    fn_0066efe0(e, 0x49, 0x10675c0, 0x5, 0x221, 0x0, 0x0, 0x8c5610, 0x0, &[]);
    fn_0066efe0(e, 0x4a, 0x10675b8, 0x5, 0x221, 0x0, 0x0, 0x8c5890, 0x0, &[]);
    fn_0066efe0(e, 0x4b, 0x10675a4, 0x5, 0x221, 0x0, 0x0, 0x8c5b10, 0x0, &[]);
    fn_0066efe0(
        e,
        0x4c,
        0x1067594,
        0x6,
        0x800,
        0x8a0ad0,
        0x0,
        0x8ba270,
        0x0,
        &[],
    );
    // The compiler left an empty loop over the 0x4D values here.
    for _ in 0..0x4d {}
    e.mem.set_u8(ACTOR_VALUES_REGISTERED, 1);
}

// Translated from 0066fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The section of an actor value (`cdecl`; no Xbox PDB name): 0 when
/// `005d1260` says so, 1 for 0xC to 0x1F, 2 when `0047f060` says so, 3 for
/// 0 to 4, otherwise 5.
pub fn fn_0066fd20(e: &mut Engine, index: u32) -> u32 {
    if e.call(0x005d_1260, &args![index]).bool() {
        0
    } else if fn_0066fd90(e, index) {
        1
    } else if e.call(0x0047_f060, &args![index]).bool() {
        2
    } else if fn_0066fdc0(e, index) {
        3
    } else {
        5
    }
}

// Translated from 0066fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `index` is from 0xC to 0x1F (signed; `cdecl`; no Xbox PDB name).
pub fn fn_0066fd90(_e: &mut Engine, index: u32) -> bool {
    (0xc..0x20).contains(&(index as i32))
}

// Translated from 0066fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `index` is from 0 to 4 (signed; `cdecl`; no Xbox PDB name).
pub fn fn_0066fdc0(_e: &mut Engine, index: u32) -> bool {
    (0..5).contains(&(index as i32))
}

// Translated from 0066fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form ID of an actor value's info (`cdecl`; no Xbox PDB name):
/// `section * 100 + 1000` plus the entry in the section ([`fn_0066ec80`],
/// sign-extended).
pub fn fn_0066fdf0(e: &mut Engine, index: u32) -> u32 {
    let section = fn_0066fd20(e, index);
    let base = section.wrapping_mul(100).wrapping_add(1000);
    let entry = fn_0066ec80(e, section, index) as i8;
    base.wrapping_add(i32::from(entry) as u32)
}

// Translated from 00670050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueInfo`'s deleting destructor (no Xbox PDB name): the
/// destructor ([`fn_00670080`]), then with bit 0 of `flags` the block
/// freed. Returns `this`.
pub fn fn_00670050(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00670080(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00670080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00670080 - the C++ exception unwinding states (FS:[0] frame, handler 00f006ce) are not translated
/// `ActorValueInfo`'s destructor (no Xbox PDB name): the four vtables of
/// the constructor stored back, then the abbreviation string (+0x3C,
/// `004037d0`), the icon (+0x2C, `00402c40`), the full name (+0x18,
/// `00591300`) and `TESForm::~TESForm` (`00483630`).
pub fn fn_00670080(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0106_7a2c);
    e.mem.set_u32(base + 0x18, 0x0106_7a14);
    e.mem.set_u32(base + 0x24, 0x0106_79fc);
    e.mem.set_u32(base + 0x2c, 0x0106_79dc);
    e.call(0x0040_37d0, &args![base + 0x3c]);
    e.call(0x0040_2c40, &args![base + 0x2c]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00670120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets an actor value's enumeration names (`cdecl`, variadic; no Xbox PDB
/// name): `iEnumCount` becomes `min(count, 10)` and that many words after
/// `count` go to `sEnumNames`.
pub fn fn_00670120(e: &mut Engine, info: u32, count: u32, names: &[u32]) {
    let info_ptr = Ptr::<ActorValueInfo>::new(info);
    let n = e.call(MIN_INT, &args![count, 10u32]).u32();
    e.set(info_ptr, ActorValueInfo::iEnumCount, n);
    let mut i = 0i32;
    while i < e.get(info_ptr, ActorValueInfo::iEnumCount) as i32 {
        let at = info
            .wrapping_add(ENUM_NAMES)
            .wrapping_add((i as u32).wrapping_mul(4));
        e.mem.set_u32(at, names[i as usize]);
        i += 1;
    }
}

/// The registered form of [`fn_00670120`]: two fixed words, then the names.
fn fn_00670120_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(words.len() >= 2, "00670120 needs 2 argument words");
    fn_00670120(e, words[0], words[1], &words[2..]);
    Ret::default()
}

// Translated from 006701a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006701a0 - the /GS cookie check (00ec408c) of its frame is not translated
/// `ActorValueInfo::Load` (Xbox PDB, matched by the engine map; `RET 4`):
/// false unless the file's current form type is 0x59; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)`, then every chunk: `FULL` (full name
/// at +0x18, `00487050`), `DESC` (+0x24, `TESDescription::LoadDescriptionData`),
/// `EDID` (read into a stack buffer of the chunk's size, at most 0x200
/// bytes, and given to virtual slot 0x134), `OBND` (virtual slot 0xE0 with
/// the file), `ANAM` (the abbreviation at +0x3C set from the chunk, or to
/// the empty string for an empty chunk), `ICON` (+0x2C,
/// `TESTexture::LoadTextureChunk`); true at the end.
pub fn actor_value_info_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let component = |offset: u32| if base != 0 { base + offset } else { 0 };
    if e.call(0x0047_2660, &args![file]).u8() != 0x59 {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                e.call(0x0048_7050, &args![component(0x18), file]);
            }
            CHUNK_DESC => {
                e.call(0x0048_2d50, &args![component(0x24), file]);
            }
            CHUNK_EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ANAM => {
                if e.call(0x0040_1660, &args![file]).u32() != 0 {
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                        e.call(0x0047_2890, &args![file, text, 0u32]);
                        e.call(0x0043_8390, &args![base + 0x3c, text]);
                    });
                } else {
                    e.call(0x0043_8390, &args![base + 0x3c, 0x0101_1584u32]);
                }
            }
            CHUNK_ICON => {
                e.call(0x0048_e3d0, &args![component(0x2c), file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// The weapon-skill badge textures `0066ea20` answers
/// (`Interface\Icons\TypeIcons\weap_skill_icon_<skill>.dds`).
const BADGE_BIG_GUNS: u32 = 0x0106_755c;
const BADGE_ENERGY: u32 = 0x0106_7524;
const BADGE_EXPLOSIVES: u32 = 0x0106_74e8;
const BADGE_MELEE: u32 = 0x0106_74b4;
const BADGE_SMALL_ARMS: u32 = 0x0106_747c;
const BADGE_UNARMED: u32 = 0x0106_7444;
/// The `double` 0.0 at `01012060`.
const ZERO_01012060: u32 = 0x0101_2060;

// Translated from 0066ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetItemWeaponTypeBadge` (engine map), `cdecl`: the badge
/// texture path of a weapon skill's actor value (the compiler's byte map
/// and jump table over `index - 0x21` as `match` arms): 0x21 big guns,
/// 0x22 energy weapons, 0x23 explosives, 0x26 melee, 0x29 small arms,
/// 0x2D unarmed; null for any other.
pub fn actor_value_get_item_weapon_type_badge(_e: &mut Engine, index: u32) -> u32 {
    match index.wrapping_sub(0x21) {
        0 => BADGE_BIG_GUNS,
        1 => BADGE_ENERGY,
        2 => BADGE_EXPLOSIVES,
        5 => BADGE_MELEE,
        8 => BADGE_SMALL_ARMS,
        0xc => BADGE_UNARMED,
        _ => 0,
    }
}

// Translated from 0066eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::GetActorValueScriptName` (engine map), `cdecl`: the script
/// name (+0x38) of the actor value's info (`0066e920`), null without one.
pub fn actor_value_get_actor_value_script_name(e: &mut Engine, index: u32) -> u32 {
    let info = info_of(e, index);
    if info == 0 {
        0
    } else {
        e.mem.u32(info + 0x38)
    }
}

// Translated from 0066eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`): the first actor value whose
/// registered info's script name (+0x38, the infos at `011d61c8` read
/// without a null test) compares equal to `name` (`00404dc0` answers 0),
/// else 0x4D.
pub fn fn_0066eb40(e: &mut Engine, name: u32) -> u32 {
    let mut found = 0x4du32;
    let mut index = 0u32;
    while (index as i32) < 0x4d && found == 0x4d {
        let info = e.mem.u32(ACTOR_VALUE_INFOS + index * 4);
        let script_name = e.mem.u32(info + 0x38);
        if e.call(0x0040_4dc0, &args![script_name, name]).u32() == 0 {
            found = index;
        }
        index += 1;
    }
    found
}

// Translated from 0066eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValue::CheckClampDamageModifier` (engine map), `cdecl`: `delta`,
/// unless it is below 0.0 (the `double` at `01012060`; `FCOMP`, false on
/// NaN) for an actor value with flag 0x20 (`00406d70`): then the owner's
/// slot 5 (+0x14) plus slot 1 (+0x4) value for it (`FADD`, stored as a
/// `float`) is, when below `-delta`, negated and answered instead.
pub fn actor_value_check_clamp_damage_modifier(
    e: &mut Engine,
    owner: u32,
    index: u32,
    delta: f32,
) -> f32 {
    let mut result = x87_quiet_bits(delta.to_bits());
    let zero = f64::from_bits(e.mem.u64(ZERO_01012060));
    if x87_load32(delta.to_bits()) < zero && e.call(HAS_FLAG, &args![index, 0x20u32]).bool() {
        let first = e.vcall(owner, 0x4, &args![index]).f64();
        let second = e.vcall(owner, 0x14, &args![index]).f64();
        let sum = x87_store32(x87_add(second, first));
        if x87_load32(sum) < -x87_load32(delta.to_bits()) {
            result = x87_store32(-x87_load32(sum));
        }
    }
    f32::from_bits(result)
}

// Translated from 006703b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueInfo::Save` (engine map): starts the form (`004855a0`), saves
/// the full name (`00487010`, +0x18), the description (`00482cb0`, +0x24)
/// and the icon (`0048e370`, +0x2C, with `ICON`); when the abbreviation
/// string (+0x3C, length `004048e0`) is not empty, it is added as `ANAM`
/// with its terminator (`004856f0`, characters from `00559450`); then
/// closes the form (`00485680`).
pub fn actor_value_info_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    e.call(0x0048_2cb0, &args![base + 0x24]);
    e.call(0x0048_e370, &args![base + 0x2c, CHUNK_ICON]);
    if e.call(0x0040_48e0, &args![base + 0x3c]).u32() != 0 {
        let length = e.call(0x0040_48e0, &args![base + 0x3c]).u32();
        let chars = e.call(STRING_CHARS, &args![base + 0x3c]).u32();
        e.call(
            0x0048_56f0,
            &args![CHUNK_ANAM, chars, length.wrapping_add(1)],
        );
    }
    e.call(0x0048_5680, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_fe40, fn_0066fe40(Ptr, u32, u32) -> Ptr),
        entry!(0x0066_eba0, fn_0066eba0(u32) -> bool),
        entry!(0x0066_ef90, fn_0066ef90(Ptr) -> Ptr),
        entry!(0x0066e950, actor_value_get_actor_value_name(u32) -> u32),
        entry!(0x0066e9b0, actor_value_get_actor_value_desc(u32) -> u32),
        entry!(0x0066e9f0, actor_value_get_actor_value_icon(u32) -> u32),
        entry!(
            0x0066eb00,
            actor_value_get_actor_value_abbreviation(u32) -> u32
        ),
        entry!(0x0066ec10, actor_value_to_actor_value(u32, i8) -> i32),
        entry!(0x0066ec80, fn_0066ec80(u32, u32) -> u8),
        entry!(0x0066ecf0, actor_value_get_section_count(u32) -> u32),
        entry!(0x0066ed60, fn_0066ed60(u32, u32, Ptr) -> bool),
        entry!(
            0x0066edc0,
            actor_value_get_dependent_actor_values(u32, Ptr) -> u32
        ),
        entry!(0x0066ee10, fn_0066ee10(u32) -> bool),
        entry!(0x0066ee50, fn_0066ee50(u32, u32, f32, f32, u32)),
        entry!(
            0x0066ef20,
            actor_value_owner_get_clamped_actor_value(Ptr, u32) -> i32
        ),
        entry!(
            0x0066ef50,
            actor_value_owner_get_clamped_actor_float_value(Ptr, u32) -> f64
        ),
        entry!(0x0066efd0, fn_0066efd0(Ptr)),
        (0x0066efe0, fn_0066efe0_entry as AbiFn),
        entry!(0x0066f110, fn_0066f110()),
        entry!(0x0066f190, actor_value_clamp_actor_value(u32, f32) -> f64),
        entry!(0x0066f260, actor_value_register_actor_values()),
        entry!(0x0066fd20, fn_0066fd20(u32) -> u32),
        entry!(0x0066fd90, fn_0066fd90(u32) -> bool),
        entry!(0x0066fdc0, fn_0066fdc0(u32) -> bool),
        entry!(0x0066fdf0, fn_0066fdf0(u32) -> u32),
        entry!(0x00670050, fn_00670050(Ptr, u32) -> Ptr),
        entry!(0x00670080, fn_00670080(Ptr)),
        (0x00670120, fn_00670120_entry as AbiFn),
        entry!(0x006701a0, actor_value_info_load(Ptr, Ptr) -> bool),
        entry!(
            0x0066_ea20,
            actor_value_get_item_weapon_type_badge(u32) -> u32
        ),
        entry!(
            0x0066_eac0,
            actor_value_get_actor_value_script_name(u32) -> u32
        ),
        entry!(0x0066_eb40, fn_0066eb40(u32) -> u32),
        entry!(
            0x0066_eea0,
            actor_value_check_clamp_damage_modifier(u32, u32, f32) -> f32
        ),
        entry!(0x0067_03b0, actor_value_info_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    const CALLEES: [u32; 8] = [
        0x0048_3370,
        0x0040_2d00,
        0x0048_2c50,
        0x0040_2c20,
        0x0040_37b0,
        0x004f_15a0,
        0x0048_5c10,
        0x0043_8390,
    ];

    /// Call log, object bytes and return value of one constructor run.
    type CtorRun = (Vec<(u32, Vec<u32>)>, Vec<u8>, u32);

    fn build(id: u32, name: u32) -> CtorRun {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.mem.write(this, &[0xcd; 0xc4]);
        for a in CALLEES {
            e.register(a, |_, _| Ret::default());
        }
        e.register(0x0040_6d00, |_, _| Ret::default());
        e.register(0x0040_1290, |_, _| Ret::default());
        e.register(0x0066_fdf0, |_, a| Ret {
            eax: a[0] + 1000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let ret = e.call(0x0066_fe40, &args![this, id, name]).u32();
        assert_eq!(ret, this);
        let log = e.call_log.take().unwrap();
        (
            log.into_iter().skip(1).collect(),
            e.mem.bytes(this, 0xc4),
            this,
        )
    }

    fn expected(name: u32) -> Vec<u8> {
        let mut b = vec![0xcd_u8; 0xc4];
        let mut put = |off: usize, v: u32| b[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, 0x0106_7a2c);
        put(0x18, 0x0106_7a14);
        put(0x24, 0x0106_79fc);
        put(0x2c, 0x0106_79dc);
        put(0x38, name);
        for off in [0x44, 0x4c, 0x50, 0x54, 0x58, 0x98] {
            put(off, 0);
        }
        for i in 0..15 {
            put(0x5c + i * 4, 0x4d);
        }
        for i in 0..10 {
            put(0x9c + i * 4, 0);
        }
        b
    }

    #[test]
    fn fn_0066fe40_with_an_index_and_a_name() {
        let (calls, bytes, this) = build(7, 0x2000_0000);
        assert_eq!(bytes, expected(0x2000_0000));
        let addrs: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                0x0048_3370,
                0x0040_2d00,
                0x0048_2c50,
                0x0040_2c20,
                0x0040_37b0,
                0x004f_15a0,
                0x0066_fdf0,
                0x0048_5c10,
                0x0040_6d00,
                0x0040_1290,
                0x0043_8390
            ]
        );
        assert_eq!(calls[5].1, vec![this, 0x59]);
        assert_eq!(calls[6].1, vec![7]);
        assert_eq!(calls[7].1, vec![this, 1007, 0]);
        let buffer = calls[8].1[0];
        assert_eq!(calls[8].1[1..], [0x64, 0x0106_79d0, 0x2000_0000]);
        assert_eq!(calls[9].1, vec![this, buffer]);
        assert_eq!(calls[10].1, vec![this + 0x3c, 0x0101_1584]);
    }

    #[test]
    fn fn_0066fe40_without_an_index_or_a_name() {
        let (calls, bytes, this) = build(0xffff_ffff, 0);
        assert_eq!(bytes, expected(0));
        let addrs: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                0x0048_3370,
                0x0040_2d00,
                0x0048_2c50,
                0x0040_2c20,
                0x0040_37b0,
                0x004f_15a0,
                0x0043_8390
            ]
        );
        assert_eq!(calls[0].1, vec![this]);
        assert_eq!(calls[1].1, vec![this + 0x18]);
        assert_eq!(calls[2].1, vec![this + 0x24]);
        assert_eq!(calls[3].1, vec![this + 0x2c]);
        assert_eq!(calls[4].1, vec![this + 0x3c]);
    }
}

#[cfg(test)]
mod table_tests {
    use crate::prelude::*;

    #[test]
    fn fn_0066eba0_is_true_for_exactly_the_tabled_indices() {
        let mut e = Engine::new();
        let yes = [0x12, 0x13, 0x14, 0x34, 0x3a, 0x3b, 0x3c, 0x3d];
        for index in 0..0x60u32 {
            let got = e.call(0x0066_eba0, &args![index]).bool();
            assert_eq!(got, yes.contains(&index), "index {index:#x}");
        }
        assert!(!e.call(0x0066_eba0, &args![0xffff_ffffu32]).bool());
        assert!(!e
            .call(0x0066_eba0, &args![0x12u32.wrapping_add(0x1_0000)])
            .bool());
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    #[test]
    fn clears_the_first_4d_words_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x140);
        for i in 0..0x50u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0066_ef90, &args![this]).u32(), this);
        for i in 0..0x4du32 {
            assert_eq!(e.mem.u32(this + i * 4), 0);
        }
        assert_eq!(e.mem.u32(this + 0x4d * 4), 0xffff_ffff);
    }
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The info table mapped and empty; `0066e920` reads it for real (it
    /// is outside this file, so it gets a double doing the same).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_6000, 0x1000);
        e.map(0x0101_6000, 0x2000);
        e.set_global(HUNDRED_FLOAT, 100.0f32);
        e.set_global(TEN_FLOAT, 10.0f32);
        e.register(ACTOR_VALUE_INFO, |e, a| Ret {
            eax: if (a[0] as i32) >= 0 && (a[0] as i32) < 0x4d {
                e.mem.u32(ACTOR_VALUE_INFOS + 4 * a[0])
            } else {
                0
            },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// An info block for `index` in the table.
    fn info(e: &mut Engine, index: u32) -> u32 {
        let info = e.mem.alloc(0xc4);
        e.mem.set_u32(ACTOR_VALUE_INFOS + 4 * index, info);
        info
    }

    #[test]
    fn name_prefers_a_non_empty_full_name() {
        let mut e = engine();
        assert_eq!(e.call(0x0066_e950, &args![3u32]).u32(), 0);
        let i = info(&mut e, 3);
        e.mem.set_u32(i + 0x38, 0x5555);
        e.register(COMPONENT_NAME, |_, _| Ret::default());
        assert_eq!(e.call(0x0066_e950, &args![3u32]).u32(), 0x5555);
        e.register(COMPONENT_NAME, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(STRING_LENGTH, |_, _| Ret::default());
        assert_eq!(e.call(0x0066_e950, &args![3u32]).u32(), 0x5555);
        e.register(STRING_LENGTH, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0066_e950, &args![3u32]).u32(), i + 0x19);
    }

    #[test]
    fn description_icon_and_abbreviation_come_from_their_components() {
        let mut e = engine();
        assert_eq!(e.call(0x0066_e9b0, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x0066_e9f0, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x0066_eb00, &args![2u32]).u32(), 0);
        let i = info(&mut e, 2);
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(i + 0x24, vtable);
        e.mem.set_u32(vtable + 0x10, 0x00ab_0010);
        e.register(0x00ab_0010, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        e.register(COMPONENT_NAME, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(STRING_CHARS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0066_e9b0, &args![2u32]).u32(), CHUNK_DESC);
        assert_eq!(calls(&e, 0x00ab_0010)[0], vec![i + 0x24, 0, CHUNK_DESC]);
        assert_eq!(e.call(0x0066_e9f0, &args![2u32]).u32(), i + 0x2c);
        assert_eq!(e.call(0x0066_eb00, &args![2u32]).u32(), i + 0x3c);
    }

    #[test]
    fn section_entries_map_both_ways() {
        let mut e = engine();
        let bases = [5, 0xc, 0x20, 0, 0x19, 0x2e];
        let counts = [7, 0x14, 0xe, 5, 7, 0x10];
        for section in 0..6u32 {
            let value = e.call(0x0066_ec10, &args![section, 2u32]).u32();
            assert_eq!(value, bases[section as usize] + 2);
            let back = e.call(0x0066_ec80, &args![section, value]).u32() & 0xff;
            assert_eq!(back, 2);
            assert_eq!(
                e.call(0x0066_ecf0, &args![section]).u32(),
                counts[section as usize]
            );
        }
        assert_eq!(e.call(0x0066_ec10, &args![6u32, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0066_ec80, &args![6u32, 9u32]).u32() & 0xff, 0);
        assert_eq!(e.call(0x0066_ecf0, &args![6u32]).u32(), 0);
        // The entry is a signed char.
        assert_eq!(e.call(0x0066_ec10, &args![3u32, 0xffu32]).i32(), -1);
    }

    #[test]
    fn derivation_writes_zero_without_a_function() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        e.mem.set_f32(out, 9.0);
        assert!(!e.call(0x0066_ed60, &args![0x100u32, 5u32, out]).bool());
        assert_eq!(e.mem.f32(out), 0.0);
        let i = info(&mut e, 5);
        assert!(!e.call(0x0066_ed60, &args![0x100u32, 5u32, out]).bool());
        e.mem.set_u32(i + 0x4c, 0x00ab_004c);
        e.register(0x00ab_004c, |_, a| Ret {
            st0: f64::from(a[0] + a[1]),
            ..Ret::default()
        });
        assert!(e.call(0x0066_ed60, &args![0x100u32, 5u32, out]).bool());
        assert_eq!(e.mem.f32(out), 261.0);
    }

    #[test]
    fn dependents_and_modified_callbacks() {
        let mut e = engine();
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 7);
        assert_eq!(e.call(0x0066_edc0, &args![6u32, count]).u32(), 0);
        assert_eq!(e.mem.u32(count), 0);
        assert!(!e.call(0x0066_ee10, &args![6u32]).bool());
        let i = info(&mut e, 6);
        e.mem.set_u32(i + 0x58, 3);
        assert_eq!(e.call(0x0066_edc0, &args![6u32, count]).u32(), i + 0x5c);
        assert_eq!(e.mem.u32(count), 3);
        assert!(!e.call(0x0066_ee10, &args![6u32]).bool());
        e.call(0x0066_ee50, &args![1u32, 6u32, 1.0f32, 2.0f32, 9u32]);
        e.mem.set_u32(i + 0x54, 0x00ab_0054);
        e.register(0x00ab_0054, |_, _| Ret::default());
        assert!(e.call(0x0066_ee10, &args![6u32]).bool());
        e.call(0x0066_ee50, &args![1u32, 6u32, 1.0f32, 2.0f32, 9u32]);
        assert_eq!(
            calls(&e, 0x00ab_0054),
            vec![vec![1, 6, 1.0f32.to_bits(), 2.0f32.to_bits(), 9]]
        );
    }

    #[test]
    fn clamped_values_go_through_the_owner_and_the_clamp() {
        let mut e = engine();
        let owner = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(vtable + 0xc, 0x00ab_000c);
        e.register(0x00ab_000c, |_, _| Ret {
            st0: 150.5,
            ..Ret::default()
        });
        e.register(HAS_FLAG, |_, a| Ret {
            eax: (a[1] == 0x10) as u32,
            ..Ret::default()
        });
        e.register(MIN_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(MAX_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        assert_eq!(e.call(0x0066_ef50, &args![owner, 3u32]).f64(), 100.0);
        e.register(0x0040_6ce0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).floor(),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0066_ef20, &args![owner, 3u32]).i32(), 100);
    }

    #[test]
    fn clamp_by_flags() {
        let mut e = engine();
        e.register(MIN_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(MAX_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        let clamp = |e: &mut Engine, flags: u32, v: f32| {
            e.register_double(HAS_FLAG, move |_, a| Ret {
                eax: (flags & a[1] != 0) as u32,
                ..Ret::default()
            });
            e.call(0x0066_f190, &args![3u32, v]).f64()
        };
        assert_eq!(clamp(&mut e, 0x10, -5.0), 0.0);
        assert_eq!(clamp(&mut e, 0x10, 500.0), 100.0);
        assert_eq!(clamp(&mut e, 8, 50.0), 10.0);
        assert_eq!(clamp(&mut e, 8, -2.0), 0.0);
        assert_eq!(clamp(&mut e, 0x8008, -2.0), 1.0);
        assert_eq!(clamp(&mut e, 0, -2.0), -2.0);
    }

    /// [`engine`] with doubles for everything `0066efe0` and the
    /// `ActorValueInfo` constructor reach outside this file.
    fn registering_engine() -> Engine {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        for address in [
            0x0048_3370,
            0x0040_2d00,
            0x0048_2c50,
            0x0040_2c20,
            0x0040_37b0,
            0x004f_15a0,
            0x0048_5c10,
            0x0043_8390,
            0x0040_6d00,
            0x0040_1290,
            0x0047_f060,
            0x005d_1260,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MIN_INT, |_, a| Ret {
            eax: (a[0] as i32).min(a[1] as i32) as u32,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn registration_builds_an_info_once() {
        let mut e = registering_engine();
        e.call(
            0x0066_efe0,
            &args![7u32, 0x2000u32, 0u32, 0x8009u32, 1u32, 2u32, 3u32, 2u32, 0x10u32, 0x13u32],
        );
        let i = e.mem.u32(ACTOR_VALUE_INFOS + 4 * 7);
        assert_ne!(i, 0);
        let info = Ptr::<ActorValueInfo>::new(i);
        assert_eq!(e.get(info, ActorValueInfo::uFlags), 0x8009);
        assert_eq!(e.get(info, ActorValueInfo::pDerivationFunction), 1);
        assert_eq!(e.get(info, ActorValueInfo::pMappingFunction), 2);
        assert_eq!(e.get(info, ActorValueInfo::pModifiedCallback), 3);
        assert_eq!(e.get(info, ActorValueInfo::iNumDependentActorValues), 2);
        assert_eq!((e.mem.u32(i + 0x5c), e.mem.u32(i + 0x60)), (0x10, 0x13));
        // A second registration of the same value does nothing.
        e.call_log = Some(vec![]);
        e.call(
            0x0066_efe0,
            &args![7u32, 0x2000u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert!(calls(&e, ALLOCATE).is_empty());
        // Enumeration names, at most 10.
        let mut words = vec![i, 12];
        words.extend(0..12u32);
        e.call(0x0067_0120, &words);
        assert_eq!(e.get(info, ActorValueInfo::iEnumCount), 10);
        assert_eq!(e.mem.u32(i + 0x9c + 4 * 9), 9);
    }

    #[test]
    fn register_all_runs_once_and_unregister_clears() {
        let mut e = registering_engine();
        e.call(0x0066_f260, &args![]);
        assert_eq!(e.mem.u8(ACTOR_VALUES_REGISTERED), 1);
        assert_eq!(calls(&e, ALLOCATE).len(), 0x4d);
        let at = |e: &Engine, index: u32| {
            Ptr::<ActorValueInfo>::new(e.mem.u32(ACTOR_VALUE_INFOS + 4 * index))
        };
        // Health: section 1, flags 0x10c1, its derivation and callback.
        let health = at(&e, 0x10);
        assert_eq!(e.get(health, ActorValueInfo::eAVType), 1);
        assert_eq!(e.get(health, ActorValueInfo::uFlags), 0x10c1);
        assert_eq!(
            e.get(health, ActorValueInfo::pDerivationFunction),
            0x0064_3670
        );
        assert_eq!(
            e.get(health, ActorValueInfo::pModifiedCallback),
            0x008b_9960
        );
        // Strength: three dependents.
        let strength = at(&e, 5);
        assert_eq!(e.get(strength, ActorValueInfo::iNumDependentActorValues), 3);
        let s = strength.addr();
        assert_eq!(
            [0x5c, 0x60, 0x64].map(|o| e.mem.u32(s + o)),
            [0xd, 0x26, 0x11]
        );
        // Aggression: four enumeration names.
        let aggression = at(&e, 0);
        assert_eq!(e.get(aggression, ActorValueInfo::iEnumCount), 4);
        assert_eq!(e.mem.u32(aggression.addr() + 0x9c), 0x0106_7998);
        // Hunger takes the handler at 008c5890.
        assert_eq!(
            e.get(at(&e, 0x4a), ActorValueInfo::pModifiedCallback),
            0x008c_5890
        );
        // Already registered: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x0066_f260, &args![]);
        assert!(calls(&e, ALLOCATE).is_empty());
        // Unregistering deletes each info through its vtable (slot 0x10).
        e.map(0x0106_7000, 0x1000);
        e.mem.set_u32(0x0106_7a2c + 0x10, 0x00ab_0010);
        e.register(0x00ab_0010, |_, _| Ret::default());
        e.call(0x0066_efd0, &args![0u32]);
        assert_eq!(calls(&e, 0x00ab_0010).len(), 0x4d);
        assert_eq!(e.mem.u32(ACTOR_VALUE_INFOS + 16), 0);
        assert_eq!(e.mem.u8(ACTOR_VALUES_REGISTERED), 0);
    }

    #[test]
    fn form_ids_by_section() {
        let mut e = engine();
        e.register(0x005d_1260, |_, a| Ret {
            eax: (a[0] == 0x30) as u32,
            ..Ret::default()
        });
        e.register(0x0047_f060, |_, a| Ret {
            eax: (a[0] == 0x22) as u32,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0066_fd20, &args![0x30u32]).u32(), 0);
        assert_eq!(e.call(0x0066_fd20, &args![0xcu32]).u32(), 1);
        assert_eq!(e.call(0x0066_fd20, &args![0x22u32]).u32(), 2);
        assert_eq!(e.call(0x0066_fd20, &args![4u32]).u32(), 3);
        assert_eq!(e.call(0x0066_fd20, &args![0x40u32]).u32(), 5);
        assert!(e.call(0x0066_fd90, &args![0x1fu32]).bool());
        assert!(!e.call(0x0066_fd90, &args![0x20u32]).bool());
        assert!(e.call(0x0066_fdc0, &args![0u32]).bool());
        assert!(!e.call(0x0066_fdc0, &args![0xffff_ffffu32]).bool());
        assert_eq!(e.call(0x0066_fdf0, &args![0xeu32]).u32(), 1102);
        assert_eq!(e.call(0x0066_fdf0, &args![2u32]).u32(), 1302);
        // Section 5: 1500 plus 0x40 - 0x2e.
        assert_eq!(e.call(0x0066_fdf0, &args![0x40u32]).u32(), 1518);
    }

    #[test]
    fn destructor_restores_vtables_and_runs_the_bases() {
        let mut e = engine();
        for address in [0x0040_37d0, 0x0040_2c40, 0x0059_1300, 0x0048_3630, FREE] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xc4);
        assert_eq!(e.call(0x0067_0050, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0106_7a2c);
        assert_eq!(e.mem.u32(this + 0x2c), 0x0106_79dc);
        assert_eq!(calls(&e, 0x0040_37d0), vec![vec![this + 0x3c]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0067_0050, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn load_reads_each_chunk() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        let chunks = Rc::new(RefCell::new(vec![
            CHUNK_FULL,
            CHUNK_DESC,
            CHUNK_EDID,
            CHUNK_OBND,
            CHUNK_ANAM,
            CHUNK_ICON,
            0x1234_5678,
        ]));
        e.register(0x0047_2660, |_, _| Ret {
            eax: 0x59,
            ..Ret::default()
        });
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| Ret {
            eax: c.borrow().first().copied().unwrap_or(0),
            ..Ret::default()
        });
        let c = chunks.clone();
        e.register_double(0x0047_26f0, move |_, _| {
            c.borrow_mut().remove(0);
            Ret {
                eax: !c.borrow().is_empty() as u32,
                ..Ret::default()
            }
        });
        let c = chunks.clone();
        e.register_double(0x0040_1660, move |_, _| Ret {
            eax: if c.borrow().first() == Some(&CHUNK_ANAM) {
                0
            } else {
                8
            },
            ..Ret::default()
        });
        for address in [
            0x0048_5110,
            0x0048_4ab0,
            0x0048_7050,
            0x0048_2d50,
            0x0047_2890,
            0x0043_8390,
            0x0048_e3d0,
            0x00ab_0134,
            0x00ab_00e0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xc4);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x134, 0x00ab_0134);
        e.mem.set_u32(vtable + 0xe0, 0x00ab_00e0);
        let file = 0x7000u32;
        assert!(e.call(0x0067_01a0, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(calls(&e, 0x0048_2d50), vec![vec![this + 0x24, file]]);
        assert_eq!(calls(&e, 0x0048_e3d0), vec![vec![this + 0x2c, file]]);
        assert_eq!(calls(&e, 0x00ab_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0047_2890)[0][2], 0x200);
        assert_eq!(calls(&e, 0x0043_8390), vec![vec![this + 0x3c, 0x0101_1584]]);
        // Another form type: false at once.
        e.register(0x0047_2660, |_, _| Ret {
            eax: 0x58,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0067_01a0, &args![this, file]).bool());
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn weapon_badges_by_skill() {
        let mut e = Engine::new();
        let badge = |e: &mut Engine, index: u32| e.call(0x0066_ea20, &args![index]).u32();
        assert_eq!(badge(&mut e, 0x21), BADGE_BIG_GUNS);
        assert_eq!(badge(&mut e, 0x22), BADGE_ENERGY);
        assert_eq!(badge(&mut e, 0x23), BADGE_EXPLOSIVES);
        assert_eq!(badge(&mut e, 0x26), BADGE_MELEE);
        assert_eq!(badge(&mut e, 0x29), BADGE_SMALL_ARMS);
        assert_eq!(badge(&mut e, 0x2d), BADGE_UNARMED);
        for other in [0x20, 0x24, 0x25, 0x27, 0x28, 0x2a, 0x2c, 0x2e, 0] {
            assert_eq!(badge(&mut e, other), 0);
        }
    }

    #[test]
    fn script_name_and_lookup_by_name() {
        let mut e = Engine::new();
        e.map(0x011d_6000, 0x1000);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x38, 0x7700);
        e.register_double(ACTOR_VALUE_INFO, move |_, a| {
            (if a[0] == 3 { info } else { 0 }).into_ret()
        });
        assert_eq!(e.call(0x0066_eac0, &args![3u32]).u32(), 0x7700);
        assert_eq!(e.call(0x0066_eac0, &args![4u32]).u32(), 0);

        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0x38, 0x7100);
        for i in 0..0x4d {
            e.mem
                .set_u32(ACTOR_VALUE_INFOS + i * 4, if i == 7 { info } else { other });
        }
        e.register(0x0040_4dc0, |_, a| u32::from(a[0] != a[1]).into_ret());
        assert_eq!(e.call(0x0066_eb40, &args![0x7700u32]).u32(), 7);
        assert_eq!(e.call(0x0066_eb40, &args![0x7100u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_eb40, &args![0x7200u32]).u32(), 0x4d);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0040_4dc0).len(),
            0x4d
        );
    }

    #[test]
    fn clamp_damage_modifier_limits_negative_deltas() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.register(HAS_FLAG, |_, a| (a[0] == 1).into_ret());
        e.put_vtable(0x0207_0000, &[0, 0x0090_0004, 0, 0, 0, 0x0090_0014]);
        e.register(0x0090_0004, |_, _| 10.0f64.into_ret());
        e.register(0x0090_0014, |_, _| (-4.0f64).into_ret());
        let owner = e.mem.alloc(4);
        e.mem.set_u32(owner, 0x0207_0000);
        let clamp = |e: &mut Engine, index: u32, delta: f32| {
            e.call(0x0066_eea0, &args![owner, index, delta.to_bits()])
                .f64()
        };
        // 10 - 4 = 6: a delta of -8 is limited to -6, -5 kept.
        assert_eq!(clamp(&mut e, 1, -8.0), -6.0);
        assert_eq!(clamp(&mut e, 1, -5.0), -5.0);
        // Without the flag, or a non-negative delta, unchanged.
        assert_eq!(clamp(&mut e, 2, -8.0), -8.0);
        assert_eq!(clamp(&mut e, 1, 3.0), 3.0);
        e.call_log = Some(vec![]);
        assert!(clamp(&mut e, 1, f32::NAN).is_nan());
        assert!(calls_to(&e.call_log.take().unwrap(), HAS_FLAG).is_empty());
    }

    #[test]
    fn info_save_writes_components_and_abbreviation() {
        for length in [0u32, 3] {
            let mut e = Engine::new();
            for callee in [
                0x0048_55a0,
                0x0048_7010,
                0x0048_2cb0,
                0x0048_e370,
                0x0048_56f0,
                0x0048_5680,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            e.register_double(0x0040_48e0, move |_, _| length.into_ret());
            e.register(STRING_CHARS, |_, a| (a[0] + 0x100).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0067_03b0, &args![0x5000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0048_e370), vec![vec![0x502c, CHUNK_ICON]]);
            assert_eq!(calls_to(&log, 0x0048_2cb0), vec![vec![0x5024]]);
            let anam = calls_to(&log, 0x0048_56f0);
            if length == 0 {
                assert!(anam.is_empty());
            } else {
                assert_eq!(anam, vec![vec![CHUNK_ANAM, 0x513c, 4]]);
            }
            assert_eq!(calls_to(&log, 0x0048_5680), vec![vec![0x5000]]);
        }
    }
}

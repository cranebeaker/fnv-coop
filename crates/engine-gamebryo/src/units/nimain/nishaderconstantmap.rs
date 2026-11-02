//! `NiMain/nishaderconstantmap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, interlocked_increment, release_fixed_string, EMPTY_FIXED_STRING,
    GET_FIXED_STRING,
};

/// `011f6024`: the pointer to the first table (0x88 entries of 8 bytes: a number and a
/// fixed-string handle) that `00a9c2a0` builds.
const FIRST_TABLE: u32 = 0x011f_6024;
/// `011f6028`: the number of entries of the first table.
const FIRST_TABLE_COUNT: u32 = 0x011f_6028;
/// `011f602c`: the pointer to the second table (0x19 entries of 8 bytes).
const SECOND_TABLE: u32 = 0x011f_602c;
/// `011f6030`: the number of entries of the second table.
const SECOND_TABLE_COUNT: u32 = 0x011f_6030;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00401050` (`cdecl`: array, element size, count, constructor): constructs every element
/// of an array with the constructor.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `00a9bdf0`: the element constructor handed to `00401050`.
const ENTRY_CONSTRUCTOR: u32 = 0x00a9_bdf0;
/// `00438170` (`thiscall`, `fixed_string_construct`): constructs a fixed-string handle from
/// a C string in the 4 bytes `this` points to and answers `this`.
const CONSTRUCT_FIXED_STRING: u32 = 0x0043_8170;

const INLINE_ENTRIES: [(u32, u32); 20] = [
    (0x0104_8858, 0),
    (0x010a_2074, 1),
    (0x010a_2068, 1),
    (0x010a_2060, 2),
    (0x010a_2050, 2),
    (0x010a_2048, 3),
    (0x010a_2040, 4),
    (0x0108_2c9c, 5),
    (0x010a_2034, 6),
    (0x010a_2028, 7),
    (0x010a_2018, 8),
    (0x010a_200c, 9),
    (0x010a_1ffc, 9),
    (0x010a_1ff0, 10),
    (0x010a_1fdc, 10),
    (0x010a_1fcc, 11),
    (0x010a_1fb8, 11),
    (0x010a_1fa4, 12),
    (0x010a_1f8c, 12),
    (0x010a_1f7c, 13),
];

const PAIR_ENTRIES: [(u32, u32); 116] = [
    (0x010a_1f68, 13),
    (0x010a_1f54, 14),
    (0x010a_1f3c, 14),
    (0x010a_1f2c, 15),
    (0x010a_1f18, 16),
    (0x010a_1f08, 17),
    (0x010a_1ef4, 18),
    (0x010a_1ee0, 19),
    (0x010a_1ec8, 20),
    (0x010a_1eb4, 21),
    (0x010a_1e9c, 21),
    (0x010a_1e84, 22),
    (0x010a_1e68, 22),
    (0x010a_1e50, 23),
    (0x010a_1e30, 23),
    (0x010a_1e14, 24),
    (0x010a_1df4, 24),
    (0x010a_1de4, 25),
    (0x010a_1dd0, 26),
    (0x010a_1db8, 27),
    (0x010a_1d9c, 28),
    (0x010a_1d88, 29),
    (0x010a_1d70, 29),
    (0x010a_1d58, 30),
    (0x010a_1d3c, 30),
    (0x010a_1d20, 31),
    (0x010a_1cfc, 31),
    (0x010a_1cdc, 32),
    (0x010a_1cb8, 32),
    (0x010a_1cac, 33),
    (0x010a_1ca0, 34),
    (0x010a_1c90, 35),
    (0x010a_1c80, 36),
    (0x010a_1c6c, 37),
    (0x010a_1c58, 38),
    (0x010a_1c3c, 39),
    (0x010a_1c1c, 40),
    (0x010a_1c08, 41),
    (0x010a_1bf4, 42),
    (0x010a_1bd8, 43),
    (0x010a_1bb8, 44),
    (0x010a_1ba4, 45),
    (0x010a_1b8c, 46),
    (0x010a_1b70, 47),
    (0x010a_1b50, 48),
    (0x010a_1b3c, 49),
    (0x010a_1b24, 50),
    (0x010a_1b08, 51),
    (0x010a_1ae8, 52),
    (0x010a_1ad4, 53),
    (0x010a_1ac0, 54),
    (0x010a_1aa4, 55),
    (0x010a_1a84, 56),
    (0x010a_1a70, 57),
    (0x010a_1a5c, 58),
    (0x010a_1a40, 59),
    (0x010a_1a20, 60),
    (0x010a_1a0c, 61),
    (0x010a_19f4, 62),
    (0x010a_19d8, 63),
    (0x010a_19b8, 64),
    (0x010a_19a4, 65),
    (0x010a_198c, 66),
    (0x010a_1970, 67),
    (0x010a_1950, 68),
    (0x010a_1944, 69),
    (0x010a_1938, 70),
    (0x010a_192c, 71),
    (0x010a_1920, 72),
    (0x010a_1910, 73),
    (0x0109_8dbc, 73),
    (0x010a_1900, 74),
    (0x0109_8dc4, 74),
    (0x010a_18ec, 75),
    (0x010a_18e0, 75),
    (0x010a_18cc, 76),
    (0x010a_18bc, 77),
    (0x010a_18ac, 77),
    (0x010a_18a4, 78),
    (0x010a_189c, 79),
    (0x010a_1890, 80),
    (0x010a_1884, 81),
    (0x010a_187c, 82),
    (0x010a_1870, 83),
    (0x010a_1864, 84),
    (0x010a_1858, 85),
    (0x010a_1844, 86),
    (0x010a_1834, 87),
    (0x010a_181c, 92),
    (0x010a_1804, 93),
    (0x010a_17e4, 94),
    (0x010a_17c0, 95),
    (0x010a_17ac, 88),
    (0x010a_1794, 89),
    (0x010a_1778, 90),
    (0x010a_1758, 91),
    (0x010a_174c, 96),
    (0x010a_173c, 97),
    (0x010a_1728, 98),
    (0x010a_1710, 99),
    (0x010a_1704, 100),
    (0x010a_16f8, 101),
    (0x010a_16ec, 102),
    (0x010a_16e0, 103),
    (0x010a_16cc, 104),
    (0x010a_16bc, 105),
    (0x010a_16b0, 106),
    (0x010a_16a4, 107),
    (0x010a_1694, 108),
    (0x010a_1684, 109),
    (0x010a_1678, 110),
    (0x010a_166c, 111),
    (0x010a_165c, 111),
    (0x010a_164c, 111),
    (0x010a_163c, 114),
    (0x010a_162c, 115),
];

const SECOND_PAIR_STRINGS: [u32; 12] = [
    0x010a_1624,
    0x010a_1614,
    0x010a_1604,
    0x010a_15f0,
    0x0109_8dc4,
    0x0109_8dbc,
    0x010a_18e0,
    0x010a_15e0,
    0x010a_15d0,
    0x010a_15c0,
    0x010a_15b0,
    0x010a_15a0,
];

const SECOND_CONSTRUCTED_STRINGS: [u32; 12] = [
    0x010a_1590,
    0x010a_1580,
    0x010a_1574,
    0x010a_155c,
    0x010a_1544,
    0x010a_1528,
    0x010a_150c,
    0x010a_14f8,
    0x010a_14e4,
    0x010a_14d8,
    0x010a_14bc,
    0x010a_14a0,
];

/// Table entry shape one (the first 20 entries of the first table): the handle for `string`
/// is made, held in a local, the entry's number is set and its handle replaced (counted),
/// and the local handles are released.
fn set_entry_inline(e: &mut Engine, slot: u32, string: u32, value: u32) {
    let result = e.call(GET_FIXED_STRING, &args![string]).u32();
    let mut local = e.mem.u32(EMPTY_FIXED_STRING);
    if local != result {
        interlocked_increment(e, result.wrapping_sub(8));
        if local != e.mem.u32(EMPTY_FIXED_STRING) {
            release_fixed_string(e, local);
        }
        local = result;
    }
    e.mem.set_u32(slot, value);
    if e.mem.u32(slot + 4) != local {
        acquire_fixed_string(e, local);
        let old = e.mem.u32(slot + 4);
        release_fixed_string(e, old);
        e.mem.set_u32(slot + 4, local);
    }
    release_fixed_string(e, local);
    release_fixed_string(e, result);
}

/// Table entry shape two: the handle for `string` is made and kept in a local, an entry is
/// made from `value` and the local's address (`00a9bda0`), assigned to the slot
/// (`00a9be30`), and the entry's handle and the local handle are released.
fn set_entry_pair(e: &mut Engine, slot: u32, string: u32, value: u32) {
    let name = e.call(GET_FIXED_STRING, &args![string]).u32();
    e.with_stack(0xc, |e, locals| {
        e.mem.set_u32(locals.addr(), name);
        let entry = locals.addr() + 4;
        let made = fn_00a9bda0(e, Ptr::new(entry), value, Ptr::new(locals.addr())).addr();
        fn_00a9be30(e, Ptr::new(slot), Ptr::new(made));
        let entry_handle = e.mem.u32(entry + 4);
        release_fixed_string(e, entry_handle);
        release_fixed_string(e, name);
    });
}

/// Table entry shape three: like shape two, but the local handle is constructed from the C
/// string by `00438170`.
fn set_entry_constructed(e: &mut Engine, slot: u32, string: u32, value: u32) {
    e.with_stack(0xc, |e, locals| {
        let text = e
            .call(CONSTRUCT_FIXED_STRING, &args![locals.addr(), string])
            .u32();
        let entry = locals.addr() + 4;
        let made = fn_00a9bda0(e, Ptr::new(entry), value, Ptr::new(text)).addr();
        fn_00a9be30(e, Ptr::new(slot), Ptr::new(made));
        let entry_handle = e.mem.u32(entry + 4);
        release_fixed_string(e, entry_handle);
        let local_handle = e.mem.u32(locals.addr());
        release_fixed_string(e, local_handle);
    });
}

/// Allocates a table of `count` 8-byte entries (a 4-byte count in front, the size the
/// exe's overflow-checked `count * 8 + 4` gives for these counts), constructs the entries
/// with `00a9bdf0`, records the count and the table in the globals and answers the table
/// (null when the allocation fails).
fn allocate_table(e: &mut Engine, count_slot: u32, table_slot: u32, count: u32) -> u32 {
    e.mem.set_u32(count_slot, count);
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, count * 8 + 4],
        )
        .u32();
    let table = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, count);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![block + 4, 8u32, count, ENTRY_CONSTRUCTOR],
        );
        block + 4
    };
    e.mem.set_u32(table_slot, table);
    table
}

// Translated from 00a9c2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no
/// arguments, 15104 bytes): builds two tables of 8-byte entries, each a number and a
/// fixed-string handle. The first (`011f6024`, 0x88 entries) is filled with the entries of
/// `INLINE_ENTRIES` (the first of them at the pointer just allocated) and `PAIR_ENTRIES`;
/// the second (`011f602c`, 0x19 entries) with the string at `01048858` (number 0), the
/// strings of `SECOND_PAIR_STRINGS` (numbers 1 to 12) and those of
/// `SECOND_CONSTRUCTED_STRINGS` (numbers 13 to 24). Every entry is written at the table
/// pointer read again from its global (the exe re-reads it), in the exe's order. The three
/// entry shapes differ only in how the handles are made and counted.
pub fn ni_shader_constant_map_sdm_init(e: &mut Engine) {
    let first = allocate_table(e, FIRST_TABLE_COUNT, FIRST_TABLE, 0x88);
    let (string, value) = INLINE_ENTRIES[0];
    set_entry_inline(e, first, string, value);
    for (index, (string, value)) in INLINE_ENTRIES.into_iter().enumerate().skip(1) {
        let slot = e.mem.u32(FIRST_TABLE) + 8 * index as u32;
        set_entry_inline(e, slot, string, value);
    }
    for (index, (string, value)) in PAIR_ENTRIES.into_iter().enumerate() {
        let slot = e.mem.u32(FIRST_TABLE) + 8 * (INLINE_ENTRIES.len() + index) as u32;
        set_entry_pair(e, slot, string, value);
    }
    let second = allocate_table(e, SECOND_TABLE_COUNT, SECOND_TABLE, 0x19);
    set_entry_pair(e, second, 0x0104_8858, 0);
    for (index, string) in SECOND_PAIR_STRINGS.into_iter().enumerate() {
        let slot = e.mem.u32(SECOND_TABLE) + 8 * (index as u32 + 1);
        set_entry_pair(e, slot, string, index as u32 + 1);
    }
    for (index, string) in SECOND_CONSTRUCTED_STRINGS.into_iter().enumerate() {
        let number = SECOND_PAIR_STRINGS.len() as u32 + 1 + index as u32;
        let slot = e.mem.u32(SECOND_TABLE) + 8 * number;
        set_entry_constructed(e, slot, string, number);
    }
}

// Translated from 00a9bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nishaderconstantmap.cpp`; `thiscall`, `RET 8`):
/// constructs a table entry in `this` from `value` and the address `name` of a
/// fixed-string handle: the handle starts as the empty string's, the number is `value`,
/// and when the handle at `name` differs it is counted and replaces the empty one. Returns
/// `this`.
pub fn fn_00a9bda0(e: &mut Engine, this: Ptr, value: u32, name: Ptr) -> Ptr {
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 4, empty);
    e.mem.set_u32(this.addr(), value);
    let handle = e.mem.u32(name.addr());
    if e.mem.u32(this.addr() + 4) != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(this.addr() + 4);
        release_fixed_string(e, current);
        e.mem.set_u32(this.addr() + 4, handle);
    }
    this
}

// Translated from 00a9be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nishaderconstantmap.cpp`; `thiscall`, `RET 4`):
/// assigns the table entry `entry` to `this`: the number is copied, and when the handle
/// differs it is counted and replaces the old one (released). Returns `this`.
pub fn fn_00a9be30(e: &mut Engine, this: Ptr, entry: Ptr) -> Ptr {
    let value = e.mem.u32(entry.addr());
    e.mem.set_u32(this.addr(), value);
    let handle = e.mem.u32(entry.addr() + 4);
    if e.mem.u32(this.addr() + 4) != handle {
        acquire_fixed_string(e, handle);
        let current = e.mem.u32(this.addr() + 4);
        release_fixed_string(e, current);
        e.mem.set_u32(this.addr() + 4, handle);
    }
    this
}

// Translated from 00a9bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the element constructor handed to `00401050` by
/// `ni_shader_constant_map_sdm_init`; `thiscall`, no stack word): the entry's handle
/// starts as the empty string's and its number as 0; since the handle is not 0, it is
/// replaced by what `00a5b690` answers for a null string and the old handle released.
/// Returns `this`.
pub fn fn_00a9bdf0(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 4, empty);
    e.mem.set_u32(this.addr(), 0);
    let old = e.mem.u32(this.addr() + 4);
    if old != 0 {
        let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
        e.mem.set_u32(this.addr() + 4, handle);
        release_fixed_string(e, old);
    }
    this
}

/// CRT functions `ParseKeyName` calls (`cdecl`): `_strcpy_s` (`00ec65a6`), `strchr`
/// (`00ec7690`), `isdigit` (`00ec8860`), `strtok_s` (`00ecc5f8`) and `atol` (`00eca6d3`),
/// and the bounded copy `00404d80` (destination, size, source; engine map unit
/// `effectitem.cpp`, unnamed). `NiStricmp` (engine map, `00aa1360`, `cdecl`).
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCHR: u32 = 0x00ec_7690;
const ISDIGIT: u32 = 0x00ec_8860;
const STRTOK_S: u32 = 0x00ec_c5f8;
const ATOL: u32 = 0x00ec_a6d3;
const BOUNDED_COPY_00404D80: u32 = 0x0040_4d80;
const NI_STRICMP: u32 = 0x00aa_1360;
/// The delimiter string `"_"` (`01063a04`).
const UNDERSCORE: u32 = 0x0106_3a04;

// Translated from 00a9bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::LookUpPredefinedMappingType` (Xbox PDB; `cdecl`: mapping, object
/// type): the register count of a predefined mapping, from the compiler's switch (a byte
/// table of the mapping less one, then six cases): 8 or 9 (9 unless the second word is 3)
/// for mappings 1 to 24, 37 to 68 and 88 to 95; 9 for 25 to 32, 34, 36 and 96 to 99; 8 for
/// 33 and 35; 7 for 69 to 87 and 100 to 103; 5 for 104 and 106 to 115; 4 for 105; 0
/// otherwise.
pub fn ni_shader_constant_map_look_up_predefined_mapping_type(
    _e: &mut Engine,
    mapping: u32,
    object_type: u32,
) -> u32 {
    match mapping {
        1..=24 | 37..=68 | 88..=95 => u32::from(object_type != 3) + 8,
        25..=32 | 34 | 36 | 96..=99 => 9,
        33 | 35 => 8,
        69..=87 | 100..=103 => 7,
        104 | 106..=115 => 5,
        105 => 4,
        _ => 0,
    }
}

// Translated from 00a9bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::ParseKeyName` (Xbox PDB; `cdecl`: key, buffer, buffer size, index
/// out). False when `size + 1` is below 0x104. Otherwise copies the key into a local and into
/// `buffer` (`_strcpy_s` with 0x104 each); when the key's first `_` is followed by one digit
/// and the end, splits the local at `_` (`strtok_s`) and, when a second token exists, stores
/// its value (`atol`) at `index` and copies the first token into `buffer` (`00404d80` with
/// `size`). True.
// Translation incomplete: 00a9bee0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_shader_constant_map_parse_key_name(
    e: &mut Engine,
    key: u32,
    buffer: u32,
    size: u32,
    index: Ptr,
) -> bool {
    if size.wrapping_add(1) < 0x104 {
        return false;
    }
    // Locals: +0 the strtok context, +4 the 0x104-byte copy.
    e.with_stack(0x108, |e, locals| {
        let context = locals.addr();
        let copy = context + 4;
        e.call(STRCPY_S, &args![copy, 0x104u32, key]);
        e.call(STRCPY_S, &args![buffer, 0x104u32, key]);
        let underscore = e.call(STRCHR, &args![copy, 0x5fu32]).u32();
        if underscore == 0 {
            return;
        }
        let digit = e.mem.i8(underscore + 1) as i32 as u32;
        if e.call(ISDIGIT, &args![digit]).u32() == 0 {
            return;
        }
        if e.mem.u8(underscore + 2) != 0 {
            return;
        }
        let first = e.call(STRTOK_S, &args![copy, UNDERSCORE, context]).u32();
        let second = e.call(STRTOK_S, &args![0u32, UNDERSCORE, context]).u32();
        if second == 0 {
            return;
        }
        let value = e.call(ATOL, &args![second]).u32();
        e.mem.set_u32(index.addr(), value);
        e.call(BOUNDED_COPY_00404D80, &args![buffer, size, first]);
    });
    true
}

// Translated from 00a9bfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::LookUpPredefinedMapping` (Xbox PDB; `cdecl`: key, mapping out,
/// index out): parses the key into a 0x104-byte local (`ParseKeyName`; false when that
/// fails), then finds the first entry of the first table (`011f6024`, count `011f6028`)
/// whose name (+4) matches it (`NiStricmp`), stores that entry's number (+0) at `mapping`
/// and answers true; false when none matches.
// Translation incomplete: 00a9bfd0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_shader_constant_map_look_up_predefined_mapping(
    e: &mut Engine,
    key: u32,
    mapping: Ptr,
    index: Ptr,
) -> bool {
    e.with_stack(0x104, |e, name| {
        if !ni_shader_constant_map_parse_key_name(e, key, name.addr(), 0x104, index) {
            return false;
        }
        let mut entry = 0u32;
        while entry < e.mem.u32(FIRST_TABLE_COUNT) {
            let table = e.mem.u32(FIRST_TABLE);
            let slot = table.wrapping_add(entry * 8);
            let entry_name = e.mem.u32(table.wrapping_add(entry * 8 + 4));
            if e.call(NI_STRICMP, &args![entry_name, name.addr()]).u32() == 0 {
                let number = e.mem.u32(slot);
                e.mem.set_u32(mapping.addr(), number);
                return true;
            }
            entry += 1;
        }
        false
    })
}

// Translated from 00a9bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::LookUpObjectMappingType` (Xbox PDB; cdecl: mapping, register count
/// out, element size out): the count and size start at 1 and 4; then by the compiler's switch
/// (a byte table of the mapping less one, then four cases): 1 and 22 set the size to 1 and
/// answer 4; 2 to 11, 14, 15 and 18 to 21 answer 7; 12, 13, 23 and 24 set 4 registers of 16
/// bytes and answer 9; 16 and 17 set 3 registers of 12 bytes and answer 8; anything else
/// answers 0.
pub fn ni_shader_constant_map_look_up_object_mapping_type(
    e: &mut Engine,
    mapping: u32,
    count: Ptr,
    size: Ptr,
) -> u32 {
    e.mem.set_u32(count.addr(), 1);
    e.mem.set_u32(size.addr(), 4);
    match mapping {
        1 | 22 => {
            e.mem.set_u32(size.addr(), 1);
            4
        }
        2..=11 | 14 | 15 | 18..=21 => 7,
        12 | 13 | 23 | 24 => {
            e.mem.set_u32(count.addr(), 4);
            e.mem.set_u32(size.addr(), 0x10);
            9
        }
        16 | 17 => {
            e.mem.set_u32(count.addr(), 3);
            e.mem.set_u32(size.addr(), 0xc);
            8
        }
        _ => 0,
    }
}

/// What the four `Add...Entry` stubs answer (0x30).
const ADD_ENTRY_RESULT: u32 = 0x30;

// Translated from 00a9bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::AddPredefinedEntry` (Xbox PDB; `__thiscall`, four words, none read):
/// answers 0x30.
pub fn ni_shader_constant_map_add_predefined_entry(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    ADD_ENTRY_RESULT
}

// Translated from 00a9bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::AddEntry` (Xbox PDB; `__thiscall`, ten words, none read): answers
/// 0x30.
pub fn ni_shader_constant_map_add_entry(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
) -> u32 {
    ADD_ENTRY_RESULT
}

// Translated from 00a9bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::AddOperatorEntry` (Xbox PDB; `__thiscall`, six words, none read):
/// answers 0x30.
pub fn ni_shader_constant_map_add_operator_entry(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    ADD_ENTRY_RESULT
}

// Translated from 00a9bd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::AddObjectEntry` (Xbox PDB; `__thiscall`, five words, none read):
/// answers 0x30.
pub fn ni_shader_constant_map_add_object_entry(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    ADD_ENTRY_RESULT
}

// Translated from 00a9c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::LookUpPredefinedMappingType_ov2` (Xbox PDB; cdecl, a key): the
/// register count (`LookUpPredefinedMappingType`) of the predefined mapping the key names
/// (`LookUpPredefinedMapping` into two stack words); 0 when it names none or mapping 0.
pub fn ni_shader_constant_map_look_up_predefined_mapping_type_ov2(e: &mut Engine, key: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let (mapping, index) = (locals.addr(), locals.addr() + 4);
        if !ni_shader_constant_map_look_up_predefined_mapping(
            e,
            key,
            Ptr::new(mapping),
            Ptr::new(index),
        ) {
            return 0;
        }
        let found = e.mem.u32(mapping);
        if found == 0 {
            return 0;
        }
        let object_type = e.mem.u32(index);
        ni_shader_constant_map_look_up_predefined_mapping_type(e, found, object_type)
    })
}

// Translated from 00a9c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::LookUpObjectMapping` (Xbox PDB; cdecl: a name, the mapping out): the
/// first entry of the second table (`011f602c`, count `011f6030`) whose name (+4) matches
/// (`NiStricmp`) gives its number (+0) and true; false when none does.
pub fn ni_shader_constant_map_look_up_object_mapping(
    e: &mut Engine,
    name: u32,
    mapping: Ptr,
) -> bool {
    let mut entry = 0u32;
    while entry < e.mem.u32(SECOND_TABLE_COUNT) {
        let slot = e.mem.u32(SECOND_TABLE).wrapping_add(entry * 8);
        let entry_name = e.mem.u32(slot + 4);
        if e.call(NI_STRICMP, &args![entry_name, name]).u32() == 0 {
            let number = e.mem.u32(slot);
            e.mem.set_u32(mapping.addr(), number);
            return true;
        }
        entry += 1;
    }
    false
}

/// `NiDynamicEffect`'s switch byte (+0x9c, the effect is on) and type byte (+0x9d).
const EFFECT_ON: u32 = 0x9c;
const EFFECT_TYPE: u32 = 0x9d;

/// The `index`th effect that is on in a list (nodes: next +0, effect +4); 0 when none.
fn nth_effect_on(e: &Engine, list: u32, index: u32) -> u32 {
    let mut node = list;
    let mut seen = 0u32;
    while node != 0 {
        let effect = e.mem.u32(node + 4);
        node = e.mem.u32(node);
        if effect != 0 && e.mem.u8(effect + EFFECT_ON) != 0 {
            let at = seen;
            seen += 1;
            if at == index {
                return effect;
            }
        }
    }
    0
}

/// A single effect field of the state: the effect when it is on, else 0.
fn effect_if_on(e: &Engine, effect: u32) -> u32 {
    if effect == 0 || e.mem.u8(effect + EFFECT_ON) == 0 {
        0
    } else {
        effect
    }
}

// Translated from 00a9c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMap::GetDynamicEffectForObject` (Xbox PDB; cdecl: the dynamic effect
/// state, the object type, an index): 0 without a state. By type (the compiler's jump table):
/// 1 to 4 walk the light list (+0xc) and count the lights that are on whose type byte is the
/// object type (or, for type 1, any type above it); 6 and 7 count the effects that are on in
/// the lists at +0x14 and +0x10; the answer is the one at the index. 5 and 8 answer the effect
/// at +0x18 or +0x1c when it is on. Other types, and no match, answer 0.
pub fn ni_shader_constant_map_get_dynamic_effect_for_object(
    e: &mut Engine,
    state: Ptr,
    object_type: u32,
    index: u32,
) -> Ptr {
    let state = state.addr();
    if state == 0 {
        return Ptr::NULL;
    }
    let found = match object_type {
        1..=4 => {
            let mut node = e.mem.u32(state + 0xc);
            if node == 0 {
                return Ptr::NULL;
            }
            let mut seen = 0u32;
            let mut effect;
            // The code tests the node for null once more here (it cannot be); a null node would
            // look at the effect 0.
            if node == 0 {
                effect = 0;
            } else {
                effect = e.mem.u32(node + 4);
                node = e.mem.u32(node);
            }
            loop {
                let kind = u32::from(e.mem.u8(effect.wrapping_add(EFFECT_TYPE)));
                let on = e.mem.u8(effect.wrapping_add(EFFECT_ON));
                let above = kind > object_type && object_type == 1;
                if (kind == object_type) != above && on != 0 {
                    let at = seen;
                    seen += 1;
                    if at == index {
                        break effect;
                    }
                }
                if node == 0 {
                    break 0;
                }
                effect = e.mem.u32(node + 4);
                node = e.mem.u32(node);
            }
        }
        5 => effect_if_on(e, e.mem.u32(state + 0x18)),
        6 => nth_effect_on(e, e.mem.u32(state + 0x14), index),
        7 => nth_effect_on(e, e.mem.u32(state + 0x10), index),
        8 => effect_if_on(e, e.mem.u32(state + 0x1c)),
        _ => 0,
    };
    Ptr::new(found)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_bbf0,
            ni_shader_constant_map_look_up_predefined_mapping_type(u32, u32) -> u32
        ),
        entry!(
            0x00a9_bee0,
            ni_shader_constant_map_parse_key_name(u32, u32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a9_bfd0,
            ni_shader_constant_map_look_up_predefined_mapping(u32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a9_c2a0, ni_shader_constant_map_sdm_init()),
        entry!(0x00a9_bda0, fn_00a9bda0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00a9_bdf0, fn_00a9bdf0(Ptr) -> Ptr),
        entry!(0x00a9_be30, fn_00a9be30(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_bcd0,
            ni_shader_constant_map_look_up_object_mapping_type(u32, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a9_bd60,
            ni_shader_constant_map_add_predefined_entry(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a9_bd70,
            ni_shader_constant_map_add_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00a9_bd80,
            ni_shader_constant_map_add_operator_entry(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a9_bd90,
            ni_shader_constant_map_add_object_entry(Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a9_c0a0,
            ni_shader_constant_map_look_up_predefined_mapping_type_ov2(u32) -> u32
        ),
        entry!(
            0x00a9_c0e0,
            ni_shader_constant_map_look_up_object_mapping(u32, Ptr) -> bool
        ),
        entry!(
            0x00a9_c130,
            ni_shader_constant_map_get_dynamic_effect_for_object(Ptr, u32, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Handles = Rc<RefCell<HashMap<u32, u32>>>;

    /// The doubles for everything outside the unit: fixed strings are interned (count at
    /// `-8`), `00a9bda0` and `00a9be30` keep the counts the way the real ones must.
    fn engine() -> (Engine, Handles) {
        let mut e = engine_with_counting();
        let handles: Handles = Rc::default();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for index in 0..a[2] {
                e.mem.set_u32(a[0] + 8 * index + 4, empty);
            }
            Ret::default()
        });
        let table = handles.clone();
        e.register_double(GET_FIXED_STRING, move |e, a| {
            let handle = intern(e, &table, a[0]);
            e.mem.set_u32(handle - 8, e.mem.u32(handle - 8) + 1);
            handle.into_ret()
        });
        let table = handles.clone();
        e.register_double(CONSTRUCT_FIXED_STRING, move |e, a| {
            let handle = intern(e, &table, a[1]);
            e.mem.set_u32(handle - 8, e.mem.u32(handle - 8) + 1);
            e.mem.set_u32(a[0], handle);
            a[0].into_ret()
        });
        (e, handles)
    }

    fn intern(e: &mut Engine, table: &Handles, string: u32) -> u32 {
        if let Some(handle) = table.borrow().get(&string) {
            return *handle;
        }
        let handle = e.mem.alloc(0x20) + 8;
        table.borrow_mut().insert(string, handle);
        handle
    }

    #[test]
    fn builds_both_tables_with_balanced_counts() {
        let (mut e, handles) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00a9_c2a0, &args![]);
        // 136 + 13 strings looked up, 12 constructed.
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 149);
        assert_eq!(calls_to(&e, CONSTRUCT_FIXED_STRING).len(), 12);
        let first = e.mem.u32(FIRST_TABLE);
        let second = e.mem.u32(SECOND_TABLE);
        assert_eq!(e.mem.u32(FIRST_TABLE_COUNT), 0x88);
        assert_eq!(e.mem.u32(SECOND_TABLE_COUNT), 0x19);
        assert_eq!(e.mem.u32(first - 4), 0x88);
        assert_eq!(e.mem.u32(second - 4), 0x19);
        let vector_calls = calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR);
        assert_eq!(
            vector_calls,
            vec![
                vec![first, 8, 0x88, ENTRY_CONSTRUCTOR],
                vec![second, 8, 0x19, ENTRY_CONSTRUCTOR]
            ]
        );
        let handle_of = |string: u32| handles.borrow()[&string];
        // Entries of the three shapes at their offsets.
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(first + 4), handle_of(0x0104_8858));
        for (index, (string, value)) in INLINE_ENTRIES.into_iter().enumerate() {
            assert_eq!(e.mem.u32(first + 8 * index as u32), value);
            assert_eq!(e.mem.u32(first + 8 * index as u32 + 4), handle_of(string));
        }
        for (index, (string, value)) in PAIR_ENTRIES.into_iter().enumerate() {
            let slot = first + 8 * (20 + index as u32);
            assert_eq!(e.mem.u32(slot), value);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        assert_eq!(e.mem.u32(second), 0);
        assert_eq!(e.mem.u32(second + 4), handle_of(0x0104_8858));
        for (index, string) in SECOND_PAIR_STRINGS.into_iter().enumerate() {
            let slot = second + 8 * (index as u32 + 1);
            assert_eq!(e.mem.u32(slot), index as u32 + 1);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        for (index, string) in SECOND_CONSTRUCTED_STRINGS.into_iter().enumerate() {
            let slot = second + 8 * (13 + index as u32);
            assert_eq!(e.mem.u32(slot), 13 + index as u32);
            assert_eq!(e.mem.u32(slot + 4), handle_of(string));
        }
        // Every handle ends up counted once per table slot that holds it.
        let mut holders: HashMap<u32, u32> = HashMap::new();
        for index in 0..0x88u32 {
            *holders.entry(e.mem.u32(first + 8 * index + 4)).or_default() += 1;
        }
        for index in 0..0x19u32 {
            *holders
                .entry(e.mem.u32(second + 8 * index + 4))
                .or_default() += 1;
        }
        for handle in handles.borrow().values() {
            assert_eq!(e.mem.u32(handle - 8), holders[handle], "{handle:#x}");
        }
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn made_entry_counts_a_new_handle_and_keeps_the_empty_one_uncounted() {
        let mut e = engine_with_counting();
        let entry = e.mem.alloc(8);
        let name = e.mem.alloc(4);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(name, handle);
        assert_eq!(
            e.call(0x00a9_bda0, &args![entry, 0x42u32, name]).u32(),
            entry
        );
        assert_eq!(e.mem.u32(entry), 0x42);
        assert_eq!(e.mem.u32(entry + 4), handle);
        assert_eq!(e.mem.u32(handle - 8), 1);
        // The empty handle stays uncounted and the entry holds it as is.
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(name, empty);
        e.call(0x00a9_bda0, &args![entry, 7u32, name]);
        assert_eq!(e.mem.u32(entry), 7);
        assert_eq!(e.mem.u32(entry + 4), empty);
    }

    #[test]
    fn assigned_entry_replaces_and_recounts_the_handle() {
        let mut e = engine_with_counting();
        let old = e.mem.alloc(0x20) + 8;
        let new = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(old - 8, 1);
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot + 4, old);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 9);
        e.mem.set_u32(entry + 4, new);
        assert_eq!(e.call(0x00a9_be30, &args![slot, entry]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 9);
        assert_eq!(e.mem.u32(slot + 4), new);
        assert_eq!(e.mem.u32(new - 8), 1);
        assert_eq!(e.mem.u32(old - 8), 0);
        // The same handle again: nothing is counted.
        e.call(0x00a9_be30, &args![slot, entry]);
        assert_eq!(e.mem.u32(new - 8), 1);
    }

    #[test]
    fn element_constructor_replaces_the_empty_handle_with_the_null_string_handle() {
        let mut e = engine_with_counting();
        let handle = e.mem.alloc(0x20) + 8;
        e.register_double(GET_FIXED_STRING, move |_, a| {
            assert_eq!(a, &[0]);
            handle.into_ret()
        });
        let entry = e.mem.alloc(8);
        e.mem.write(entry, &[0xcc; 8]);
        assert_eq!(e.call(0x00a9_bdf0, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0);
        assert_eq!(e.mem.u32(entry + 4), handle);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    /// CRT doubles that work on the test memory (`strcpy_s`, `strchr`, `isdigit`,
    /// `strtok_s`, `atol`, the bounded copy and a case-insensitive compare).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.map(0x0106_3000, 0x1000);
        e.mem.set_cstr(UNDERSCORE, b"_");
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        });
        e.register(BOUNDED_COPY_00404D80, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|&c| u32::from(c) == a[1]) {
                Some(at) => (a[0] + at as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(ISDIGIT, |_, a| {
            u32::from((a[0] as u8).is_ascii_digit()).into_ret()
        });
        e.register(STRTOK_S, |e, a| {
            // Splits at '_' only, keeping the rest pointer in the context.
            let start = if a[0] != 0 { a[0] } else { e.mem.u32(a[2]) };
            if start == 0 || e.mem.u8(start) == 0 {
                return 0u32.into_ret();
            }
            let text = e.mem.cstr(start);
            match text.iter().position(|&c| c == b'_') {
                Some(at) => {
                    e.mem.set_u8(start + at as u32, 0);
                    e.mem.set_u32(a[2], start + at as u32 + 1);
                }
                None => e.mem.set_u32(a[2], start + text.len() as u32),
            }
            start.into_ret()
        });
        e.register(ATOL, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            text.parse::<u32>().unwrap().into_ret()
        });
        e.register(NI_STRICMP, |e, a| {
            let x = e.mem.cstr(a[0]).to_ascii_lowercase();
            let y = e.mem.cstr(a[1]).to_ascii_lowercase();
            u32::from(x != y).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    #[test]
    fn mapping_type_follows_the_switch() {
        let mut e = Engine::new();
        let expect = |mapping: u32, kind: u32| -> u32 {
            match mapping {
                1..=24 | 37..=68 | 88..=95 => {
                    if kind == 3 {
                        8
                    } else {
                        9
                    }
                }
                25..=32 | 34 | 36 | 96..=99 => 9,
                33 | 35 => 8,
                69..=87 | 100..=103 => 7,
                104 | 106..=115 => 5,
                105 => 4,
                _ => 0,
            }
        };
        for mapping in [
            0u32, 1, 24, 25, 32, 33, 34, 35, 36, 37, 68, 69, 87, 88, 95, 96, 99,
        ]
        .into_iter()
        .chain([100, 103, 104, 105, 106, 115, 116, 0xffff_ffff])
        {
            for kind in [3u32, 4] {
                assert_eq!(
                    e.call(0x00a9_bbf0, &args![mapping, kind]).u32(),
                    expect(mapping, kind),
                    "{mapping} {kind}"
                );
            }
        }
        assert_eq!(e.call(0x00a9_bbf0, &args![10u32, 3u32]).u32(), 8);
        assert_eq!(e.call(0x00a9_bbf0, &args![10u32, 0u32]).u32(), 9);
    }

    #[test]
    fn parse_key_name_splits_a_digit_suffix() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x104);
        let index = e.mem.alloc(4);
        e.mem.set_u32(index, 0xeeee);
        let key = string(&mut e, b"LightPos_3");
        assert!(e
            .call(0x00a9_bee0, &args![key, buffer, 0x104u32, index])
            .bool());
        assert_eq!(e.mem.cstr(buffer), b"LightPos".to_vec());
        assert_eq!(e.mem.u32(index), 3);
        // Two digits, no digit, no underscore: the whole key, the index untouched.
        for text in [&b"Light_12"[..], b"Light_x", b"Light"] {
            let key = string(&mut e, text);
            e.mem.set_u32(index, 0xeeee);
            assert!(e
                .call(0x00a9_bee0, &args![key, buffer, 0x200u32, index])
                .bool());
            assert_eq!(e.mem.cstr(buffer), text.to_vec());
            assert_eq!(e.mem.u32(index), 0xeeee);
        }
        // A buffer size below 0x103: false, nothing copied.
        let calls_before = e.call_log.as_ref().unwrap().len();
        assert!(!e
            .call(0x00a9_bee0, &args![key, buffer, 0x102u32, index])
            .bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), calls_before + 1);
    }

    #[test]
    fn look_up_predefined_mapping_finds_the_entry() {
        let mut e = engine();
        let table = e.mem.alloc(0x18);
        let names = [&b"World"[..], b"LightPos", b"Fog"];
        for (i, name) in names.iter().enumerate() {
            let text = string(&mut e, name);
            e.mem.set_u32(table + 8 * i as u32, 100 + i as u32);
            e.mem.set_u32(table + 8 * i as u32 + 4, text);
        }
        e.mem.set_u32(FIRST_TABLE, table);
        e.mem.set_u32(FIRST_TABLE_COUNT, 3);
        let mapping = e.mem.alloc(4);
        let index = e.mem.alloc(4);
        let key = string(&mut e, b"lightpos_2");
        assert!(e.call(0x00a9_bfd0, &args![key, mapping, index]).bool());
        assert_eq!((e.mem.u32(mapping), e.mem.u32(index)), (101, 2));
        let key = string(&mut e, b"Missing");
        e.mem.set_u32(mapping, 0);
        assert!(!e.call(0x00a9_bfd0, &args![key, mapping, index]).bool());
        assert_eq!(e.mem.u32(mapping), 0);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn object_mapping_types_by_case() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let (count, size) = (out, out + 4);
        let look = |e: &mut Engine, mapping: u32| {
            let kind = e.call(0x00a9_bcd0, &args![mapping, count, size]).u32();
            (kind, e.mem.u32(count), e.mem.u32(size))
        };
        assert_eq!(look(&mut e, 1), (4, 1, 1));
        assert_eq!(look(&mut e, 22), (4, 1, 1));
        for mapping in [2, 11, 14, 15, 18, 21] {
            assert_eq!(look(&mut e, mapping), (7, 1, 4));
        }
        for mapping in [12, 13, 23, 24] {
            assert_eq!(look(&mut e, mapping), (9, 4, 0x10));
        }
        assert_eq!(look(&mut e, 16), (8, 3, 0xc));
        assert_eq!(look(&mut e, 17), (8, 3, 0xc));
        assert_eq!(look(&mut e, 0), (0, 1, 4));
        assert_eq!(look(&mut e, 25), (0, 1, 4));
    }

    #[test]
    fn the_add_entry_stubs_answer_0x30() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9_bd60, &[0; 5]).u32(), 0x30);
        assert_eq!(e.call(0x00a9_bd70, &[0; 11]).u32(), 0x30);
        assert_eq!(e.call(0x00a9_bd80, &[0; 7]).u32(), 0x30);
        assert_eq!(e.call(0x00a9_bd90, &[0; 6]).u32(), 0x30);
    }

    /// A table of (number, name) entries at `table`/`count`; names are small integers the
    /// `NiStricmp` double compares as words.
    fn table(e: &mut Engine, table: u32, count: u32, entries: &[(u32, u32)]) {
        e.map(table, 4);
        e.map(count, 4);
        let block = e.mem.alloc(8 * entries.len() as u32);
        for (i, (number, name)) in entries.iter().enumerate() {
            e.mem.set_u32(block + 8 * i as u32, *number);
            e.mem.set_u32(block + 8 * i as u32 + 4, *name);
        }
        e.mem.set_u32(table, block);
        e.mem.set_u32(count, entries.len() as u32);
    }

    #[test]
    fn look_up_object_mapping_by_name() {
        let mut e = Engine::new();
        e.register(NI_STRICMP, |_, a| word(u32::from(a[0] != a[1])));
        table(
            &mut e,
            SECOND_TABLE,
            SECOND_TABLE_COUNT,
            &[(5, 0x10), (9, 0x20)],
        );
        let out = e.mem.alloc(4);
        assert!(e.call(0x00a9_c0e0, &args![0x20u32, out]).bool());
        assert_eq!(e.mem.u32(out), 9);
        assert!(!e.call(0x00a9_c0e0, &args![0x30u32, out]).bool());
    }

    #[test]
    fn predefined_mapping_type_of_a_key() {
        let mut e = Engine::new();
        // The key has no `_`: `ParseKeyName` copies it and keeps it whole.
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCHR, |_, _| word(0));
        e.register(NI_STRICMP, |e, a| {
            word(u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])))
        });
        let name = |e: &mut Engine, text: &[u8]| {
            let p = e.mem.alloc(16);
            e.mem.set_cstr(p, text);
            p
        };
        let (world, zero, view) = (
            name(&mut e, b"World"),
            name(&mut e, b"Zero"),
            name(&mut e, b"View"),
        );
        table(
            &mut e,
            FIRST_TABLE,
            FIRST_TABLE_COUNT,
            &[(0, zero), (33, world), (2, view)],
        );
        let key = name(&mut e, b"World");
        assert_eq!(e.call(0x00a9_c0a0, &args![key]).u32(), 8);
        let key = name(&mut e, b"View");
        assert_eq!(e.call(0x00a9_c0a0, &args![key]).u32(), 9);
        // Mapping 0 and an unknown key answer 0.
        let key = name(&mut e, b"Zero");
        assert_eq!(e.call(0x00a9_c0a0, &args![key]).u32(), 0);
        let key = name(&mut e, b"None");
        assert_eq!(e.call(0x00a9_c0a0, &args![key]).u32(), 0);
    }

    fn effect(e: &mut Engine, kind: u8, on: bool) -> u32 {
        let effect = e.mem.alloc(0xa0);
        e.mem.set_u8(effect + EFFECT_TYPE, kind);
        e.mem.set_u8(effect + EFFECT_ON, u8::from(on));
        effect
    }

    fn list(e: &mut Engine, effects: &[u32]) -> u32 {
        let mut head = 0;
        for effect in effects.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, head);
            e.mem.set_u32(node + 4, *effect);
            head = node;
        }
        head
    }

    #[test]
    fn dynamic_effects_by_type_and_index() {
        let mut e = Engine::new();
        let ambient = effect(&mut e, 1, true);
        let point = effect(&mut e, 2, true);
        let off_point = effect(&mut e, 2, false);
        let spot = effect(&mut e, 3, true);
        let lights = list(&mut e, &[ambient, off_point, point, spot]);
        let state = e.mem.alloc(0x20);
        e.mem.set_u32(state + 0xc, lights);
        let get = |e: &mut Engine, kind: u32, index: u32| {
            e.call(0x00a9_c130, &args![state, kind, index]).u32()
        };
        assert_eq!(get(&mut e, 2, 0), point);
        assert_eq!(get(&mut e, 2, 1), 0);
        // Type 1 also takes every type above it.
        assert_eq!(get(&mut e, 1, 1), point);
        assert_eq!(get(&mut e, 1, 2), spot);
        assert_eq!(get(&mut e, 4, 0), 0);
        let projector = effect(&mut e, 5, true);
        let off = effect(&mut e, 5, false);
        let lists = list(&mut e, &[off, 0, projector]);
        e.mem.set_u32(state + 0x14, lists);
        let single = list(&mut e, &[projector]);
        e.mem.set_u32(state + 0x10, single);
        assert_eq!(get(&mut e, 6, 0), projector);
        assert_eq!(get(&mut e, 6, 1), 0);
        assert_eq!(get(&mut e, 7, 0), projector);
        e.mem.set_u32(state + 0x18, projector);
        e.mem.set_u32(state + 0x1c, off);
        assert_eq!(get(&mut e, 5, 9), projector);
        assert_eq!(get(&mut e, 8, 0), 0);
        assert_eq!(get(&mut e, 9, 0), 0);
        assert_eq!(get(&mut e, 0, 0), 0);
        assert_eq!(e.call(0x00a9_c130, &args![0u32, 2u32, 0u32]).u32(), 0);
        e.mem.set_u32(state + 0xc, 0);
        assert_eq!(get(&mut e, 2, 0), 0);
    }
}

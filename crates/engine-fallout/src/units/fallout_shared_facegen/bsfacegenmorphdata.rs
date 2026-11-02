//! `fallout shared/facegen/bsfacegenmorphdata.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00661d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `00661d40` and frees 0x8 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_00661d60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0066_1d40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x8u32]);
    }
    this
}

// Translated from 00661db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHead` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00661de0` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_morph_data_head_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_1de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 006623a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHair` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006623d0` and frees 0xc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_morph_data_hair_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_23d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xcu32]);
    }
    this
}

// Translated from 006626f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBaseMorphExtraData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00662720` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_base_morph_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0066_2720, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00661d30 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5b94 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00661d30(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5b94
}

// Translated from 00661d90 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5bb8 without reading the receiver. The map names this body `BSFaceGenMorphDataHead::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00661d90(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5bb8
}

// Translated from 00662390 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5ba8 without reading the receiver. The map names this body `BSFaceGenMorphDataHair::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00662390(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5ba8
}

// Translated from 006626e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5ba0 without reading the receiver. The map names this body `BSFaceGenBaseMorphExtraData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006626e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5ba0
}

/// `BSFaceGenMorphDataHead`'s vtable (RTTI `.?AVBSFaceGenMorphDataHead@@`),
/// `BSFaceGenMorphData`'s (`.?AVBSFaceGenMorphData@@`) and
/// `BSFaceGenBaseMorphExtraData`'s (`.?AVBSFaceGenBaseMorphExtraData@@`).
const MORPH_DATA_HEAD_VTABLE: u32 = 0x0106_5eb4;
const MORPH_DATA_VTABLE: u32 = 0x0106_5f04;
const BASE_MORPH_EXTRA_DATA_VTABLE: u32 = 0x0106_5fac;
/// The base constructor and destructor bodies of `BSFaceGenMorphData`
/// (`004968b0`, `00496910`) and of the extra data (`00a7b440`: `this`;
/// name, `00a7b300`).
const MORPH_DATA_BASE_CONSTRUCT: u32 = 0x0049_68b0;
const MORPH_DATA_BASE_DESTRUCT: u32 = 0x0049_6910;
const EXTRA_DATA_BASE_CONSTRUCT: u32 = 0x00a7_b440;
const EXTRA_DATA_BASE_DESTRUCT: u32 = 0x00a7_b300;
/// The extra data's name (`00653460`, no arguments).
const EXTRA_DATA_NAME: u32 = 0x0065_3460;
/// The custom morph names (`char *[]`, one entry used: `"VampireMorph"`).
const CUSTOM_MORPH_NAMES: u32 = 0x0119_b49c;
/// `operator new` (cdecl: size), `operator delete` (cdecl) and the
/// `memset` wrapper `00403d30` (cdecl: buffer, value, size), the
/// `memcpy` wrapper `00401460` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
/// `_vector constructor iterator` (`00401050`, stdcall: array, size,
/// count, constructor) and the element constructor it is given
/// (`006815c0`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// The CRT's invalid-index report (`00ec7c56`, no arguments).
const INVALID_INDEX: u32 = 0x00ec_7c56;
/// The size of the differential morph collection (`00662840`) and of the
/// statistical one (`0065cc30`), `ECX` = the collection.
const DIFFERENTIAL_COUNT: u32 = 0x0066_2840;
const STATISTICAL_COUNT: u32 = 0x0065_cc30;
/// The statistical morph entry (`00662890`: collection; index).
const STATISTICAL_ENTRY: u32 = 0x0066_2890;
/// A morph entry's name (`0044a560`, `ECX` = entry).
const ENTRY_NAME: u32 = 0x0044_a560;
/// The vertex list of a differential entry (+0x1C): its count
/// (`0065c700`) and vertex `i` (`0065c720`).
const VERTEX_COUNT: u32 = 0x0065_c700;
const VERTEX_AT: u32 = 0x0065_c720;
/// The index list of a statistical entry (+0x20): its count (`00662820`)
/// and the address of index `i` (`006578b0`).
const INDEX_COUNT: u32 = 0x0066_2820;
const INDEX_AT: u32 = 0x0065_78b0;
/// `NiPoint3`'s set (`00416870`: `this`; x, y, z), returns `this`.
const POINT_SET: u32 = 0x0041_6870;
/// The constructors of `BSFaceGenMorphDifferential` (`00660700`: `this`;
/// count) and `BSFaceGenMorphStatistical` (`00660970`: `this`; count,
/// data), and the morph's data array (`00726070`: the word at +4).
const DIFFERENTIAL_CONSTRUCT: u32 = 0x0066_0700;
const STATISTICAL_CONSTRUCT: u32 = 0x0066_0970;
const MORPH_DATA: u32 = 0x0072_6070;
/// The expression, modifier and phoneme names (`005dd830`, `005dd840`,
/// `005dd850`, cdecl: index) and the name compare (`00404dc0`, cdecl: two
/// strings, 0 when equal).
const EXPRESSION_NAME: u32 = 0x005d_d830;
const MODIFIER_NAME: u32 = 0x005d_d840;
const PHONEME_NAME: u32 = 0x005d_d850;
const NAME_COMPARE: u32 = 0x0040_4dc0;
/// The name's text (`00464f30`, cdecl: name, 0) and the log print
/// (`005b5e40`, cdecl: format, text).
const NAME_TEXT: u32 = 0x0046_4f30;
const PRINT: u32 = 0x005b_5e40;
/// `"MODELS: Statistical and Differential FaceGen morphs found for ..."`
/// for expressions, modifiers, phonemes and the custom morph.
const TEXT_BOTH_EXPRESSION: u32 = 0x0106_5e40;
const TEXT_BOTH_MODIFIER: u32 = 0x0106_5dd0;
const TEXT_BOTH_PHONEME: u32 = 0x0106_5d60;
const TEXT_BOTH_CUSTOM: u32 = 0x0106_5ce8;
/// The morph getter of kind 3 (`00662190`, `this`; index).
const OTHER_MORPH: u32 = 0x0066_2190;
/// The geometry of an object (`005495f0`, `ECX` = object), its lock test
/// (`00a67360`: `this`; 1), its vertex iterator start (`00a67410`:
/// `this`; iterator) and unlock (`00a673d0`); the iterator's constructor
/// (`00534550`), validity (`00559450`), read (`00656fd0`: `this`; point)
/// and advance (`00657be0`); the object's vertex count (`00456650`).
const OBJECT_GEOMETRY: u32 = 0x0054_95f0;
const GEOMETRY_LOCK: u32 = 0x00a6_7360;
const GEOMETRY_BEGIN: u32 = 0x00a6_7410;
const GEOMETRY_UNLOCK: u32 = 0x00a6_73d0;
const ITERATOR_CONSTRUCT: u32 = 0x0053_4550;
const ITERATOR_VALID: u32 = 0x0055_9450;
const ITERATOR_READ: u32 = 0x0065_6fd0;
const ITERATOR_NEXT: u32 = 0x0065_7be0;
const OBJECT_VERTEX_COUNT: u32 = 0x0045_6650;

/// The four morph categories of a head: table offset, slot count, table
/// size and the "both kinds" message.
const CATEGORIES: [(u32, u32, u32, u32); 4] = [
    (0x8, 15, 0x3c, TEXT_BOTH_EXPRESSION),
    (0xc, 17, 0x44, TEXT_BOTH_MODIFIER),
    (0x10, 16, 0x40, TEXT_BOTH_PHONEME),
    (0x14, 1, 0x4, TEXT_BOTH_CUSTOM),
];

/// Name `index` of category `category` (0 expressions, 1 modifiers, 2
/// phonemes, 3 the custom morph).
fn category_name(e: &mut Engine, category: usize, index: u32) -> u32 {
    match category {
        0 => e.call(EXPRESSION_NAME, &args![index]).u32(),
        1 => e.call(MODIFIER_NAME, &args![index]).u32(),
        2 => e.call(PHONEME_NAME, &args![index]).u32(),
        _ => fn_00661da0(e, index),
    }
}

/// The table at `this + offset`, allocated (and zeroed) on first use.
fn ensure_table(e: &mut Engine, this: Ptr, offset: u32, size: u32) {
    let slot = this.addr().wrapping_add(offset);
    if e.mem.u32(slot) == 0 {
        let table = e.call(OPERATOR_NEW, &args![size]).u32();
        e.mem.set_u32(slot, table);
        let table = e.mem.u32(slot);
        e.call(MEMSET, &args![table, 0u32, size]);
    }
}

/// Makes the differential morph of entry `index` of `collection` in slot
/// `slot` of the table at `this + offset`, copying the entry's vertices.
fn make_differential(
    e: &mut Engine,
    this: Ptr,
    collection: u32,
    index: u32,
    offset: u32,
    slot: u32,
) {
    let entry = fn_00662860(e, Ptr::new(collection), index);
    let count = e.call(VERTEX_COUNT, &args![entry + 0x1c]).u32();
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let morph = if block != 0 {
        e.call(DIFFERENTIAL_CONSTRUCT, &args![block, count]).u32()
    } else {
        0
    };
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    e.mem.set_u32(table.wrapping_add(slot * 4), morph);
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    let morph = e.mem.u32(table.wrapping_add(slot * 4));
    let data = e.call(MORPH_DATA, &args![morph]).u32();
    let mut vertex = 0u32;
    while vertex < count {
        let coordinate = |e: &mut Engine, at: u32| {
            let entry = fn_00662860(e, Ptr::new(collection), index);
            let point = e
                .call(VERTEX_AT, &args![entry.wrapping_add(0x1c), vertex])
                .u32();
            x87_quiet_bits(e.mem.u32(point.wrapping_add(at)))
        };
        let z = coordinate(e, 8);
        let y = coordinate(e, 4);
        let x = coordinate(e, 0);
        e.with_stack(0xc, |e, local| {
            let point = e.call(POINT_SET, &args![local, x, y, z]).u32();
            let target = data.wrapping_add(vertex.wrapping_mul(0xc));
            for i in 0..3u32 {
                let word = e.mem.u32(point.wrapping_add(i * 4));
                e.mem.set_u32(target.wrapping_add(i * 4), word);
            }
        });
        vertex += 1;
    }
}

/// Makes the statistical morph of entry `index` of `collection` in slot
/// `slot` of the table at `this + offset` (replacing, with a message, a
/// differential one already there), copying the entry's indices.
fn make_statistical(
    e: &mut Engine,
    this: Ptr,
    collection: u32,
    index: u32,
    offset: u32,
    slot: u32,
    message: u32,
) {
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    if e.mem.u32(table.wrapping_add(slot * 4)) != 0 {
        let entry = e.call(STATISTICAL_ENTRY, &args![collection, index]).u32();
        let name = e.call(ENTRY_NAME, &args![entry]).u32();
        let text = e.call(NAME_TEXT, &args![name, 0u32]).u32();
        e.call(PRINT, &args![message, text]);
        let table = e.mem.u32(this.addr().wrapping_add(offset));
        let old = e.mem.u32(table.wrapping_add(slot * 4));
        if old != 0 {
            e.vcall(old, 0x0, &args![1u32]);
        }
    }
    let entry = e.call(STATISTICAL_ENTRY, &args![collection, index]).u32();
    let count = e.call(INDEX_COUNT, &args![entry.wrapping_add(0x20)]).u32();
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let morph = if block != 0 {
        let data = e.mem.u32(entry.wrapping_add(0x1c));
        e.call(STATISTICAL_CONSTRUCT, &args![block, count, data])
            .u32()
    } else {
        0
    };
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    e.mem.set_u32(table.wrapping_add(slot * 4), morph);
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    let morph = e.mem.u32(table.wrapping_add(slot * 4));
    let data = e.call(MORPH_DATA, &args![morph]).u32();
    let mut i = 0u32;
    while i < count {
        let at = e.call(INDEX_AT, &args![entry.wrapping_add(0x20), i]).u32();
        let value = e.mem.u32(at);
        e.mem.set_u32(data.wrapping_add(i * 4), value);
        i += 1;
    }
}

// Translated from 00660ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHead::BSFaceGenMorphDataHead` (Xbox PDB): with a
/// FaceGen model (`model`), sorts each differential morph (`model + 0xC0`)
/// and then each statistical morph (`model + 0xD8`) by name into the
/// expression (15), modifier (17), phoneme (16) or custom (1) table
/// (+0x8, +0xC, +0x10, +0x14, allocated and zeroed on first use); a
/// statistical morph replaces a differential one of the same name, with a
/// log message. Names that match no table are ignored.
// Translation incomplete: 00660ba0 - SEH exception frame (FS:[0]) that frees the new morphs on unwinding is not modelled.
pub fn bs_face_gen_morph_data_head_construct(e: &mut Engine, this: Ptr, model: u32) -> Ptr {
    fn_00661d10(e, this);
    e.mem.set_u32(this.addr(), MORPH_DATA_HEAD_VTABLE);
    for offset in [0x8u32, 0xc, 0x10, 0x14] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    if model == 0 {
        return this;
    }
    let differential = model.wrapping_add(0xc0);
    let mut index = 0u32;
    while index < e.call(DIFFERENTIAL_COUNT, &args![differential]).u32() {
        'sorted: for (category, &(offset, count, size, _)) in CATEGORIES.iter().enumerate() {
            for slot in 0..count {
                let entry = fn_00662860(e, Ptr::new(differential), index);
                let name = e.call(ENTRY_NAME, &args![entry]).u32();
                let wanted = category_name(e, category, slot);
                if e.call(NAME_COMPARE, &args![wanted, name]).u32() == 0 {
                    ensure_table(e, this, offset, size);
                    make_differential(e, this, differential, index, offset, slot);
                    break 'sorted;
                }
            }
        }
        index += 1;
    }
    let statistical = model.wrapping_add(0xd8);
    let mut index = 0u32;
    while index < e.call(STATISTICAL_COUNT, &args![statistical]).u32() {
        'sorted: for (category, &(offset, count, size, message)) in CATEGORIES.iter().enumerate() {
            for slot in 0..count {
                let entry = e.call(STATISTICAL_ENTRY, &args![statistical, index]).u32();
                let name = e.call(ENTRY_NAME, &args![entry]).u32();
                let wanted = category_name(e, category, slot);
                if e.call(NAME_COMPARE, &args![wanted, name]).u32() == 0 {
                    ensure_table(e, this, offset, size);
                    make_statistical(e, this, statistical, index, offset, slot, message);
                    break 'sorted;
                }
            }
        }
        index += 1;
    }
    this
}

// Translated from 00661d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphData`'s constructor body: the base constructor
/// (`004968b0`) and the vtable (`01065f04`). Returns `this`.
pub fn fn_00661d10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MORPH_DATA_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), MORPH_DATA_VTABLE);
    this
}

// Translated from 00661d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphData`'s destructor body: the vtable (`01065f04`), then
/// the base destructor body (`00496910`).
pub fn fn_00661d40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MORPH_DATA_VTABLE);
    e.call(MORPH_DATA_BASE_DESTRUCT, &args![this]);
}

// Translated from 00661da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The custom morph name `index` (cdecl): entry `index` of the name table
/// at `0119b49c` (the exe's data, read as it is, without a bounds check).
pub fn fn_00661da0(e: &mut Engine, index: u32) -> u32 {
    e.mem
        .u32(CUSTOM_MORPH_NAMES.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00661de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDataHead`'s destructor body: the vtable, then for each
/// of the four tables (15, 17, 16 and 1 slots) deletes every morph (its
/// slot 0 with 1) and frees the table; then the `BSFaceGenMorphData` part
/// (`00661d40`).
// Translation incomplete: 00661de0 - SEH exception frame (FS:[0]) for the base part's unwinding is not modelled.
pub fn fn_00661de0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MORPH_DATA_HEAD_VTABLE);
    for &(offset, count, _, _) in CATEGORIES.iter() {
        let slot = this.addr().wrapping_add(offset);
        if e.mem.u32(slot) == 0 {
            continue;
        }
        let mut i = 0u32;
        while i < count {
            let table = e.mem.u32(slot);
            if e.mem.u32(table.wrapping_add(i * 4)) != 0 {
                let morph = e.mem.u32(table.wrapping_add(i * 4));
                if morph != 0 {
                    e.vcall(morph, 0x0, &args![1u32]);
                }
            }
            i += 1;
        }
        let table = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![table]);
    }
    fn_00661d40(e, this);
}

// Translated from 00662030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies morph `index` of kind `kind` (0: phoneme table, 1: expression,
/// 2: modifier, 3: `00662190`; the exe's `switch` table) to `target`
/// (`ApplyMorph`, the morph's slot +4, with `target`, `base` and
/// `weight`), when there is one.
pub fn fn_00662030(
    e: &mut Engine,
    this: Ptr,
    kind: u32,
    index: i32,
    target: u32,
    base: u32,
    weight: f32,
) {
    let morph = match kind {
        0 => fn_00662150(e, this, index),
        1 => fn_006620d0(e, this, index),
        2 => fn_00662110(e, this, index),
        3 => e.call(OTHER_MORPH, &args![this, index]).u32(),
        _ => 0,
    };
    if morph != 0 {
        let weight = x87_quiet_bits(weight.to_bits());
        e.vcall(morph, 0x4, &args![target, base, weight]);
    }
}

/// Slot `index` (`0..count`) of the table at `this + offset`, or null.
fn table_slot(e: &mut Engine, this: Ptr, offset: u32, count: i32, index: i32) -> u32 {
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    if table == 0 || index <= -1 || index >= count {
        return 0;
    }
    let table = e.mem.u32(this.addr().wrapping_add(offset));
    e.mem
        .u32(table.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 006620d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The expression morph `index` (0..15), or null.
pub fn fn_006620d0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    table_slot(e, this, 0x8, 15, index)
}

// Translated from 00662110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The modifier morph `index` (0..17), or null.
pub fn fn_00662110(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    table_slot(e, this, 0xc, 17, index)
}

// Translated from 00662150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The phoneme morph `index` (0..16), or null.
pub fn fn_00662150(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    table_slot(e, this, 0x10, 16, index)
}

// Translated from 006624c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBaseMorphExtraData`'s constructor (`this`; object, extra
/// points, extra count): the extra-data base with its name, the vtable;
/// when the object's geometry can be locked, copies its vertices (the
/// object's vertex count, +0x14) into a new point array (+0xC) made for
/// them and `extra count` more (+0x10), which `extra points` fill when
/// given, and unlocks it; otherwise the array and counts are 0.
// Translation incomplete: 006624c0 - SEH exception frame (FS:[0]) that frees the array on unwinding is not modelled.
pub fn fn_006624c0(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    extra_points: u32,
    extra_count: u32,
) -> Ptr {
    let name = e.call(EXTRA_DATA_NAME, &[]).u32();
    e.call(EXTRA_DATA_BASE_CONSTRUCT, &args![this, name]);
    e.mem.set_u32(this.addr(), BASE_MORPH_EXTRA_DATA_VTABLE);
    e.with_stack(0xc, |e, iterator| {
        e.call(ITERATOR_CONSTRUCT, &args![iterator]);
        if object != 0 && e.call(OBJECT_GEOMETRY, &args![object]).u32() != 0 {
            let geometry = e.call(OBJECT_GEOMETRY, &args![object]).u32();
            if e.call(GEOMETRY_LOCK, &args![geometry, 1u32]).bool() {
                let geometry = e.call(OBJECT_GEOMETRY, &args![object]).u32();
                e.call(GEOMETRY_BEGIN, &args![geometry, iterator]);
            }
        }
        let this_at = |off: u32| this.addr().wrapping_add(off);
        if e.call(ITERATOR_VALID, &args![iterator]).u32() == 0 {
            e.mem.set_u32(this_at(0xc), 0);
            e.mem.set_u32(this_at(0x14), 0);
            e.mem.set_u32(this_at(0x10), 0);
            return;
        }
        let vertices = e.call(OBJECT_VERTEX_COUNT, &args![object]).u32() & 0xffff;
        e.mem.set_u32(this_at(0x14), vertices);
        let total = e.mem.u32(this_at(0x14));
        e.mem.set_u32(this_at(0x10), total);
        if extra_points != 0 && extra_count != 0 {
            let total = e.mem.u32(this_at(0x10)).wrapping_add(extra_count);
            e.mem.set_u32(this_at(0x10), total);
        }
        let total = e.mem.u32(this_at(0x10));
        let size = total.checked_mul(0xc).unwrap_or(u32::MAX);
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        let array = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, 0xcu32, total, POINT_CONSTRUCT],
            );
            block
        } else {
            0
        };
        e.mem.set_u32(this_at(0xc), array);
        let mut target = e.mem.u32(this_at(0xc));
        let mut i = 0u32;
        while i < e.mem.u32(this_at(0x14)) {
            e.with_stack(0xc, |e, point| {
                e.call(POINT_CONSTRUCT, &args![point]);
                e.call(ITERATOR_READ, &args![iterator, point]);
                for k in 0..3u32 {
                    let word = e.mem.u32(point.addr() + k * 4);
                    e.mem.set_u32(target.wrapping_add(k * 4), word);
                }
            });
            target = target.wrapping_add(0xc);
            e.call(ITERATOR_NEXT, &args![iterator]);
            i += 1;
        }
        if extra_points != 0 && extra_count != 0 {
            let at = e
                .mem
                .u32(this_at(0x14))
                .wrapping_mul(0xc)
                .wrapping_add(e.mem.u32(this_at(0xc)));
            e.call(
                MEMCPY,
                &args![at, extra_points, extra_count.wrapping_mul(0xc)],
            );
        }
        let geometry = e.call(OBJECT_GEOMETRY, &args![object]).u32();
        e.call(GEOMETRY_UNLOCK, &args![geometry]);
    });
    this
}

// Translated from 00662720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBaseMorphExtraData`'s destructor body: the vtable, the point
/// array (+0xC) freed when set, then the extra-data base (`00a7b300`).
pub fn fn_00662720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BASE_MORPH_EXTRA_DATA_VTABLE);
    let array = e.mem.u32(this.addr().wrapping_add(0xc));
    if array != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
    }
    e.call(EXTRA_DATA_BASE_DESTRUCT, &args![this]);
}

// Translated from 00662860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of a differential morph collection (0x34 bytes each,
/// array at +0xC): reports an index at or past the count (`00662840`) to
/// the CRT (`00ec7c56`) and returns the address anyway.
pub fn fn_00662860(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let count = e.call(DIFFERENTIAL_COUNT, &args![this]).u32();
    if index >= count {
        e.call(INVALID_INDEX, &[]);
    }
    index
        .wrapping_mul(0x34)
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
}

// Translated from 00662190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The custom morph `index` (0..1, the table at +0x14), or null; the kind-3 getter of
/// `fn_00662030`.
pub fn fn_00662190(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    table_slot(e, this, 0x14, 1, index)
}

/// `BSFaceGenMorphDataHair`'s vtable.
const MORPH_DATA_HAIR_VTABLE: u32 = 0x0106_5f5c;
/// `"HairMorph"`.
const HAIR_MORPH_NAME: u32 = 0x0106_5f4c;

// Translated from 006621d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006621d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenMorphDataHair::BSFaceGenMorphDataHair` (Xbox PDB, `RET 4`): the
/// `BSFaceGenMorphData` part, the vtable, no morph (+8); with a FaceGen model whose first
/// differential morph (`model + 0xC0`, entry 0) is named `"HairMorph"`, a new
/// `BSFaceGenMorphDifferential` (0xc bytes; null when the allocation fails) of that entry's
/// vertex count at +8, its offsets copied from the entry's vertices. Returns `this`.
pub fn bs_face_gen_morph_data_hair_construct(e: &mut Engine, this: Ptr, model: u32) -> Ptr {
    fn_00661d10(e, this);
    e.mem.set_u32(this.addr(), MORPH_DATA_HAIR_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(8), 0);
    if model == 0 {
        return this;
    }
    let collection = Ptr::new(model.wrapping_add(0xc0));
    let entry = fn_00662860(e, collection, 0);
    let name = e.call(ENTRY_NAME, &args![entry]).u32();
    if e.call(NAME_COMPARE, &args![HAIR_MORPH_NAME, name]).u32() != 0 {
        return this;
    }
    let entry = fn_00662860(e, collection, 0);
    let count = e.call(VERTEX_COUNT, &args![entry.wrapping_add(0x1c)]).u32();
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let morph = if block != 0 {
        e.call(DIFFERENTIAL_CONSTRUCT, &args![block, count]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr().wrapping_add(8), morph);
    let morph = e.mem.u32(this.addr().wrapping_add(8));
    let data = e.call(MORPH_DATA, &args![morph]).u32();
    let mut vertex = 0u32;
    while vertex < count {
        let coordinate = |e: &mut Engine, at: u32| {
            let entry = fn_00662860(e, collection, 0);
            let point = e
                .call(VERTEX_AT, &args![entry.wrapping_add(0x1c), vertex])
                .u32();
            x87_quiet_bits(e.mem.u32(point.wrapping_add(at)))
        };
        let z = coordinate(e, 8);
        let y = coordinate(e, 4);
        let x = coordinate(e, 0);
        e.with_stack(0xc, |e, local| {
            let point = e.call(POINT_SET, &args![local, x, y, z]).u32();
            let target = data.wrapping_add(vertex.wrapping_mul(0xc));
            for i in 0..3u32 {
                let word = e.mem.u32(point.wrapping_add(i * 4));
                e.mem.set_u32(target.wrapping_add(i * 4), word);
            }
        });
        vertex += 1;
    }
    this
}

// Translated from 006623d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006623d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSFaceGenMorphDataHair`'s destructor body (unnamed in the map): the vtable, the morph
/// (+8) deleted through its slot 0 with 1 when set, then the `BSFaceGenMorphData` part
/// (`fn_00661d40`).
pub fn fn_006623d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MORPH_DATA_HAIR_VTABLE);
    if e.mem.u32(this.addr().wrapping_add(8)) != 0 {
        let morph = e.mem.u32(this.addr().wrapping_add(8));
        if morph != 0 {
            e.vcall(morph, 0x0, &args![1u32]);
        }
    }
    fn_00661d40(e, this);
}

// Translated from 00662460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 0xc`): applies the hair morph (+8; its slot +4, `ApplyMorph`)
/// to `target` with `base` and `weight`, when there is a morph and a target and the weight
/// is not below 0 (`FCOMP; TEST AH,5; JP`: a NaN applies).
pub fn fn_00662460(e: &mut Engine, this: Ptr, target: u32, base: u32, weight: f32) {
    let morph = e.mem.u32(this.addr().wrapping_add(8));
    if morph == 0 || target == 0 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if x87_load32(weight.to_bits()) < zero {
        return;
    }
    let weight = x87_quiet_bits(weight.to_bits());
    let morph = e.mem.u32(this.addr().wrapping_add(8));
    e.vcall(morph, 0x4, &args![target, base, weight]);
}

/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_0ba0, bs_face_gen_morph_data_head_construct(Ptr, u32) -> Ptr),
        entry!(0x0066_1d10, fn_00661d10(Ptr) -> Ptr),
        entry!(0x0066_1d40, fn_00661d40(Ptr)),
        entry!(0x0066_1da0, fn_00661da0(u32) -> u32),
        entry!(0x0066_1de0, fn_00661de0(Ptr)),
        entry!(0x0066_2030, fn_00662030(Ptr, u32, i32, u32, u32, f32)),
        entry!(0x0066_20d0, fn_006620d0(Ptr, i32) -> u32),
        entry!(0x0066_2110, fn_00662110(Ptr, i32) -> u32),
        entry!(0x0066_2150, fn_00662150(Ptr, i32) -> u32),
        entry!(0x0066_24c0, fn_006624c0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0066_2720, fn_00662720(Ptr)),
        entry!(0x0066_2860, fn_00662860(Ptr, u32) -> u32),
        entry!(0x0066_2190, fn_00662190(Ptr, i32) -> u32),
        entry!(0x0066_21d0, bs_face_gen_morph_data_hair_construct(Ptr, u32) -> Ptr),
        entry!(0x0066_23d0, fn_006623d0(Ptr)),
        entry!(0x0066_2460, fn_00662460(Ptr, u32, u32, f32)),
        entry!(0x0066_1d30, fn_00661d30(Ptr) -> u32),
        entry!(0x0066_1d90, fn_00661d90(Ptr) -> u32),
        entry!(0x0066_2390, fn_00662390(Ptr) -> u32),
        entry!(0x0066_26e0, fn_006626e0(Ptr) -> u32),
        entry!(0x0066_1d60, fn_00661d60(Ptr, u32) -> Ptr),
        entry!(0x0066_1db0, bs_face_gen_morph_data_head_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_23a0, bs_face_gen_morph_data_hair_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_26f0, bs_face_gen_base_morph_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn fn_00661d60_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_1d60, 0x0066_1d40, 0x8);
    }

    #[test]
    fn bs_face_gen_morph_data_head_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_1db0, 0x0066_1de0, 0x18);
    }

    #[test]
    fn bs_face_gen_morph_data_hair_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_23a0, 0x0066_23d0, 0xc);
    }

    #[test]
    fn bs_face_gen_base_morph_extra_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero()
    {
        check_deleting_destructor(0x0066_26f0, 0x0066_2720, 0x18);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00661d30_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_1d30, &args![0x1000u32]).u32(), 0x011d_5b94);
        assert_eq!(e.call(0x0066_1d30, &args![0u32]).u32(), 0x011d_5b94);
    }

    #[test]
    fn fn_00661d90_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_1d90, &args![0x1000u32]).u32(), 0x011d_5bb8);
        assert_eq!(e.call(0x0066_1d90, &args![0u32]).u32(), 0x011d_5bb8);
    }

    #[test]
    fn fn_00662390_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_2390, &args![0x1000u32]).u32(), 0x011d_5ba8);
        assert_eq!(e.call(0x0066_2390, &args![0u32]).u32(), 0x011d_5ba8);
    }

    #[test]
    fn fn_006626e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_26e0, &args![0x1000u32]).u32(), 0x011d_5ba0);
        assert_eq!(e.call(0x0066_26e0, &args![0u32]).u32(), 0x011d_5ba0);
    }
}

#[cfg(test)]
mod morph_data_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const MORPH_VTABLE: u32 = 0x00d1_0000;
    const MORPH_DELETE: u32 = 0x00d1_1000;
    const MORPH_APPLY: u32 = 0x00d1_1004;

    /// Doubles for the collections and morph constructors of a FaceGen
    /// model: a collection's count is its first word, its array its word at
    /// +4 (statistical) or +0xC (differential); names are numbers compared
    /// for equality.
    fn morph_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0119_b000, 0x1000);
        e.mem.set_u32(CUSTOM_MORPH_NAMES, 0x400);
        e.put_vtable(MORPH_VTABLE, &[MORPH_DELETE, MORPH_APPLY]);
        e.register(MORPH_DELETE, |_, _| Ret::default());
        e.register(MORPH_APPLY, |_, _| Ret::default());
        e.register(MORPH_DATA_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(DIFFERENTIAL_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(STATISTICAL_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(STATISTICAL_ENTRY, |e, a| {
            ret(e.mem.u32(a[0] + 4) + a[1] * 0x38)
        });
        e.register(ENTRY_NAME, |e, a| ret(e.mem.u32(a[0])));
        e.register(EXPRESSION_NAME, |_, a| ret(0x100 + a[0]));
        e.register(MODIFIER_NAME, |_, a| ret(0x200 + a[0]));
        e.register(PHONEME_NAME, |_, a| ret(0x300 + a[0]));
        e.register(NAME_COMPARE, |_, a| ret(u32::from(a[0] != a[1])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            let zeros = vec![0u8; a[2] as usize];
            e.mem.write(a[0], &zeros);
            Ret::default()
        });
        e.register(VERTEX_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(VERTEX_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 0xc));
        e.register(POINT_SET, |e, a| {
            for i in 0..3u32 {
                e.mem.set_u32(a[0] + i * 4, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(DIFFERENTIAL_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MORPH_VTABLE);
            let data = e.mem.alloc(a[1] * 0xc + 4);
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u32(a[0] + 8, a[1]);
            ret(a[0])
        });
        e.register(STATISTICAL_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MORPH_VTABLE);
            let data = e.mem.alloc(a[1] * 4 + 4);
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u32(a[0] + 0xc, a[2]);
            ret(a[0])
        });
        e.register(MORPH_DATA, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(INDEX_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(INDEX_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
        e.register(NAME_TEXT, |_, a| ret(a[0] + 0x1000));
        e.register(PRINT, |_, _| Ret::default());
        e
    }

    #[test]
    fn head_sorts_the_morphs_by_name() {
        let mut e = morph_engine();
        let model = e.mem.alloc(0x100);
        // Two differential morphs: expression 2 and the custom morph.
        let entries = e.mem.alloc(0x34 * 2);
        let vertices = e.mem.alloc(0x18);
        for (i, word) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(vertices + i as u32 * 4, *word);
        }
        e.mem.set_u32(entries, 0x102);
        e.mem.set_u32(entries + 0x1c, 2);
        e.mem.set_u32(entries + 0x20, vertices);
        e.mem.set_u32(entries + 0x34, 0x400);
        e.mem.set_u32(model + 0xc0, 2);
        e.mem.set_u32(model + 0xcc, entries);
        // One statistical morph: phoneme 3, two indices.
        let stats = e.mem.alloc(0x38);
        let indices = e.mem.alloc(8);
        e.mem.set_u32(indices, 7);
        e.mem.set_u32(indices + 4, 9);
        e.mem.set_u32(stats, 0x303);
        e.mem.set_u32(stats + 0x1c, 0x5555);
        e.mem.set_u32(stats + 0x20, 2);
        e.mem.set_u32(stats + 0x24, indices);
        e.mem.set_u32(model + 0xd8, 1);
        e.mem.set_u32(model + 0xdc, stats);
        let head = Ptr::<()>::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        let r = e.call(0x0066_0ba0, &args![head, model]);
        assert_eq!(r.u32(), head.addr());
        assert_eq!(e.mem.u32(head.addr()), MORPH_DATA_HEAD_VTABLE);
        let expressions = e.mem.u32(head.addr() + 8);
        let morph = e.mem.u32(expressions + 2 * 4);
        let data = e.mem.u32(morph + 4);
        let copied: Vec<f32> = (0..6).map(|i| e.mem.f32(data + i * 4)).collect();
        assert_eq!(copied, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u32(expressions), 0);
        // The modifier table was never needed.
        assert_eq!(e.mem.u32(head.addr() + 0xc), 0);
        let custom = e.mem.u32(head.addr() + 0x14);
        assert_ne!(e.mem.u32(custom), 0);
        let phonemes = e.mem.u32(head.addr() + 0x10);
        let statistical = e.mem.u32(phonemes + 3 * 4);
        assert_eq!(e.mem.u32(statistical + 0xc), 0x5555);
        let data = e.mem.u32(statistical + 4);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (7, 9));
        assert!(calls_to(&e, PRINT).is_empty());
        // A statistical morph of a name already used: replaced, with the
        // message.
        e.mem.set_u32(stats, 0x102);
        let head = Ptr::<()>::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        bs_face_gen_morph_data_head_construct(&mut e, head, model);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![TEXT_BOTH_EXPRESSION, 0x1102]]
        );
        assert_eq!(calls_to(&e, MORPH_DELETE).len(), 1);
        // No model: only the base and empty tables.
        let head = Ptr::<()>::new(e.mem.alloc(0x18));
        e.mem.write(head.addr(), &[0xffu8; 0x18]);
        bs_face_gen_morph_data_head_construct(&mut e, head, 0);
        assert_eq!(e.mem.u32(head.addr() + 0x14), 0);
    }

    #[test]
    fn head_destructor_deletes_every_morph() {
        let mut e = morph_engine();
        e.register(MORPH_DATA_BASE_DESTRUCT, |_, _| Ret::default());
        let head = e.mem.alloc(0x18);
        let table = e.mem.alloc(0x44);
        let morph = e.mem.alloc(8);
        e.mem.set_u32(morph, MORPH_VTABLE);
        e.mem.set_u32(table + 16 * 4, morph);
        e.mem.set_u32(head + 0xc, table);
        e.call_log = Some(vec![]);
        e.call(0x0066_1de0, &args![head]);
        assert_eq!(calls_to(&e, MORPH_DELETE), vec![vec![morph, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![table]]);
        assert_eq!(calls_to(&e, MORPH_DATA_BASE_DESTRUCT), vec![vec![head]]);
        assert_eq!(e.mem.u32(head), MORPH_DATA_VTABLE);
    }

    #[test]
    fn morph_data_base_parts() {
        let mut e = morph_engine();
        e.register(MORPH_DATA_BASE_DESTRUCT, |_, _| Ret::default());
        let data = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_1d10, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), MORPH_DATA_VTABLE);
        e.mem.set_u32(data, 0);
        e.call(0x0066_1d40, &args![data]);
        assert_eq!(e.mem.u32(data), MORPH_DATA_VTABLE);
        assert_eq!(calls_to(&e, MORPH_DATA_BASE_CONSTRUCT), vec![vec![data]]);
        assert_eq!(calls_to(&e, MORPH_DATA_BASE_DESTRUCT), vec![vec![data]]);
        assert_eq!(e.call(0x0066_1da0, &args![0u32]).u32(), 0x400);
    }

    #[test]
    fn apply_picks_the_table_by_kind() {
        let mut e = morph_engine();
        e.register(OTHER_MORPH, |_, _| ret(0));
        let head = e.mem.alloc(0x18);
        let tables: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x44)).collect();
        let morph = e.mem.alloc(8);
        e.mem.set_u32(morph, MORPH_VTABLE);
        e.mem.set_u32(head + 0x8, tables[0]);
        e.mem.set_u32(head + 0xc, tables[1]);
        e.mem.set_u32(head + 0x10, tables[2]);
        e.mem.set_u32(tables[2] + 4 * 4, morph);
        e.call_log = Some(vec![]);
        let words = args![head, 0u32, 4i32, 0x11u32, 0x22u32, 0.5f32];
        e.call(0x0066_2030, &words);
        assert_eq!(
            calls_to(&e, MORPH_APPLY),
            vec![vec![morph, 0x11, 0x22, 0.5f32.to_bits()]]
        );
        // Kinds 1, 2, 3 and 7 find nothing here.
        for kind in [1u32, 2, 3, 7] {
            fn_00662030(&mut e, Ptr::new(head), kind, 4, 0, 0, 1.0);
        }
        assert_eq!(calls_to(&e, MORPH_APPLY).len(), 1);
        assert_eq!(calls_to(&e, OTHER_MORPH), vec![vec![head, 4]]);
    }

    #[test]
    fn table_getters_check_the_index() {
        let mut e = morph_engine();
        let head = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0066_20d0, &args![head, 0i32]).u32(), 0);
        let table = e.mem.alloc(0x44);
        for i in 0..17u32 {
            e.mem.set_u32(table + i * 4, 0x10 + i);
        }
        for offset in [0x8u32, 0xc, 0x10] {
            e.mem.set_u32(head + offset, table);
        }
        assert_eq!(e.call(0x0066_20d0, &args![head, 14i32]).u32(), 0x1e);
        assert_eq!(e.call(0x0066_20d0, &args![head, 15i32]).u32(), 0);
        assert_eq!(e.call(0x0066_2110, &args![head, 16i32]).u32(), 0x20);
        assert_eq!(e.call(0x0066_2110, &args![head, 17i32]).u32(), 0);
        assert_eq!(e.call(0x0066_2150, &args![head, 15i32]).u32(), 0x1f);
        assert_eq!(e.call(0x0066_2150, &args![head, 16i32]).u32(), 0);
        assert_eq!(e.call(0x0066_2150, &args![head, -1i32]).u32(), 0);
    }

    #[test]
    fn extra_data_copies_the_vertices_and_extra_points() {
        let mut e = morph_engine();
        e.register(EXTRA_DATA_NAME, |_, _| ret(0x77));
        e.register(EXTRA_DATA_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(ITERATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(OBJECT_GEOMETRY, |_, _| ret(0x00d2_0000));
        e.register(GEOMETRY_LOCK, |_, _| ret(1));
        e.register(GEOMETRY_BEGIN, |e, a| {
            e.mem.set_u32(a[1], 1);
            Ret::default()
        });
        e.register(GEOMETRY_UNLOCK, |_, _| Ret::default());
        e.register(ITERATOR_VALID, |e, a| ret(e.mem.u32(a[0])));
        e.register(OBJECT_VERTEX_COUNT, |_, _| ret(0x1_0002));
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ITERATOR_READ, |e, a| {
            // Point (n, n, n) for the iterator's count n.
            let n = e.mem.u32(a[0]);
            for i in 0..3u32 {
                e.mem.set_u32(a[1] + i * 4, n);
            }
            Ret::default()
        });
        e.register(ITERATOR_NEXT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            Ret::default()
        });
        e.register(MEMCPY, |_, _| Ret::default());
        let data = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        let words = args![data, 0x500u32, 0x600u32, 3u32];
        assert_eq!(e.call(0x0066_24c0, &words).u32(), data);
        assert_eq!(e.mem.u32(data), BASE_MORPH_EXTRA_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_BASE_CONSTRUCT),
            vec![vec![data, 0x77]]
        );
        assert_eq!(e.mem.u32(data + 0x14), 2);
        assert_eq!(e.mem.u32(data + 0x10), 5);
        let array = e.mem.u32(data + 0xc);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![array, 0xc, 5, POINT_CONSTRUCT]]
        );
        assert_eq!(e.mem.u32(array), 1);
        assert_eq!(e.mem.u32(array + 0xc), 2);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![array + 0x18, 0x600, 0x24]]);
        assert_eq!(calls_to(&e, GEOMETRY_UNLOCK).len(), 1);
        // No object: no geometry, empty data.
        let data = e.mem.alloc(0x18);
        e.mem.write(data, &[0xffu8; 0x18]);
        fn_006624c0(&mut e, Ptr::new(data), 0, 0, 0);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x10), 0);
        assert_eq!(e.mem.u32(data + 0x14), 0);
    }

    #[test]
    fn extra_data_destructor_frees_the_array() {
        let mut e = morph_engine();
        e.register(EXTRA_DATA_BASE_DESTRUCT, |_, _| Ret::default());
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 0xc, 0x900);
        e.call_log = Some(vec![]);
        e.call(0x0066_2720, &args![data]);
        e.mem.set_u32(data + 0xc, 0);
        fn_00662720(&mut e, Ptr::new(data));
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x900]]);
        assert_eq!(calls_to(&e, EXTRA_DATA_BASE_DESTRUCT).len(), 2);
        assert_eq!(e.mem.u32(data), BASE_MORPH_EXTRA_DATA_VTABLE);
    }

    #[test]
    fn differential_entry_reports_a_bad_index() {
        let mut e = morph_engine();
        e.register(INVALID_INDEX, |_, _| Ret::default());
        let collection = e.mem.alloc(0x10);
        e.mem.set_u32(collection, 2);
        e.mem.set_u32(collection + 0xc, 0x1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_2860, &args![collection, 1u32]).u32(), 0x1034);
        assert!(calls_to(&e, INVALID_INDEX).is_empty());
        assert_eq!(fn_00662860(&mut e, Ptr::new(collection), 2), 0x1068);
        assert_eq!(calls_to(&e, INVALID_INDEX).len(), 1);
    }

    #[test]
    fn custom_morph_getter() {
        let mut e = Engine::new();
        let head = Ptr::<()>::new(e.mem.alloc(0x18));
        assert_eq!(e.call(0x0066_2190, &args![head, 0u32]).u32(), 0);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 0x4444);
        e.mem.set_u32(table + 4, 0x5555);
        e.mem.set_u32(head.addr() + 0x14, table);
        assert_eq!(e.call(0x0066_2190, &args![head, 0u32]).u32(), 0x4444);
        assert_eq!(e.call(0x0066_2190, &args![head, 1u32]).u32(), 0);
        assert_eq!(e.call(0x0066_2190, &args![head, u32::MAX]).u32(), 0);
    }

    #[test]
    fn hair_takes_the_hair_morph() {
        let mut e = morph_engine();
        let model = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0x34);
        let vertices = e.mem.alloc(0x18);
        for (i, word) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(vertices + i as u32 * 4, *word);
        }
        e.mem.set_u32(entries, HAIR_MORPH_NAME);
        e.mem.set_u32(entries + 0x1c, 2);
        e.mem.set_u32(entries + 0x20, vertices);
        e.mem.set_u32(model + 0xc0, 1);
        e.mem.set_u32(model + 0xcc, entries);
        let hair = Ptr::<()>::new(e.mem.alloc(0xc));
        assert_eq!(e.call(0x0066_21d0, &args![hair, model]).u32(), hair.addr());
        assert_eq!(e.mem.u32(hair.addr()), MORPH_DATA_HAIR_VTABLE);
        let morph = e.mem.u32(hair.addr() + 8);
        let data = e.mem.u32(morph + 4);
        let copied: Vec<f32> = (0..6).map(|i| e.mem.f32(data + i * 4)).collect();
        assert_eq!(copied, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // Another name, or no model: no morph.
        e.mem.set_u32(entries, 0x102);
        bs_face_gen_morph_data_hair_construct(&mut e, hair, model);
        assert_eq!(e.mem.u32(hair.addr() + 8), 0);
        e.mem.set_u32(hair.addr() + 8, 0x77);
        bs_face_gen_morph_data_hair_construct(&mut e, hair, 0);
        assert_eq!(e.mem.u32(hair.addr() + 8), 0);
        // Allocation failure: a null morph.
        e.mem.set_u32(entries, HAIR_MORPH_NAME);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(MORPH_DATA, |_, _| Ret::default());
        e.mem.set_u32(entries + 0x1c, 0);
        bs_face_gen_morph_data_hair_construct(&mut e, hair, model);
        assert_eq!(e.mem.u32(hair.addr() + 8), 0);
    }

    #[test]
    fn hair_destructor_and_apply() {
        let mut e = morph_engine();
        e.register(MORPH_DATA_BASE_DESTRUCT, |_, _| Ret::default());
        e.map(ZERO_DOUBLE, 8);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let hair = Ptr::<()>::new(e.mem.alloc(0xc));
        let morph = e.mem.alloc(0xc);
        e.mem.set_u32(morph, MORPH_VTABLE);
        e.mem.set_u32(hair.addr() + 8, morph);
        e.call_log = Some(vec![]);
        e.call(0x0066_2460, &args![hair, 0x10u32, 0x20u32, 0.5f32]);
        e.call(0x0066_2460, &args![hair, 0x10u32, 0x20u32, -0.5f32]);
        e.call(0x0066_2460, &args![hair, 0u32, 0x20u32, 0.5f32]);
        e.call(0x0066_2460, &args![hair, 0x10u32, 0x20u32, 0x7f80_0001u32]);
        assert_eq!(
            calls_to(&e, MORPH_APPLY),
            vec![
                vec![morph, 0x10, 0x20, 0.5f32.to_bits()],
                vec![morph, 0x10, 0x20, 0x7fc0_0001]
            ]
        );
        e.call(0x0066_23d0, &args![hair]);
        assert_eq!(e.mem.u32(hair.addr()), MORPH_DATA_VTABLE);
        assert_eq!(calls_to(&e, MORPH_DELETE), vec![vec![morph, 1]]);
        assert_eq!(calls_to(&e, MORPH_DATA_BASE_DESTRUCT).len(), 1);
        // Without a morph: nothing deleted, nothing applied.
        e.mem.set_u32(hair.addr() + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_23d0, &args![hair]);
        e.call(0x0066_2460, &args![hair, 0x10u32, 0x20u32, 0.5f32]);
        assert!(calls_to(&e, MORPH_DELETE).is_empty());
        assert!(calls_to(&e, MORPH_APPLY).is_empty());
    }
}

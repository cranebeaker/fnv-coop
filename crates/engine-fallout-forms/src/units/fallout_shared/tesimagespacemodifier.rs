//! `fallout shared/tesimagespacemodifier.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0052b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x54 (0x730 bytes); `ECX` is the object.
/// The engine map names it `TESImageSpaceModifier::TESImageSpaceModifier` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_d97c`) is `TESImageSpaceModifier`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It builds the 0x2a-element array of 0x18-byte members at `+0x114` with the compiler's vector constructor (`00ec782f`), and, when the object is the one at `011c9750`, also calls `00484490` on it at the end.
/// The exception-unwinding frame is not translated.
pub fn fn_0052b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_d97c);
    e.call(0x0052_bad0, &args![base + 0x20]);
    e.call(
        0x00ec_782f,
        &args![
            base + 0x114,
            0x18u32,
            0x2au32,
            0x0052_baf0u32,
            0x0052_bb10u32
        ],
    );
    e.call(0x00a3_c340, &args![base + 0x504, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x51c, 0x0u32]);
    e.call(0x00a3_bd10, &args![base + 0x534, 0x0u32]);
    e.call(0x00a3_bd10, &args![base + 0x558, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x57c, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x594, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5ac, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5c4, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5dc, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5f4, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x60c, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x624, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x63c, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x54u32]);
    e.call(0x0052_bd40, &args![this]);
    if base == 0x011c_9750 {
        e.call(0x0048_4490, &args![this]);
    }
    this
}

// Translated from 0052bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006815c0` on the member at `+0xcc` of `this` (`ECX`) and returns `this`. The disassembly
/// shows no other work; `006815c0` takes only `ECX`.
pub fn fn_0052bad0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this.addr() + 0xcc]);
    this
}

// Translated from 0052d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0052d240`, which only forwards `this` to
/// `00483710`, an empty function, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `00483710`'s
/// disassembly is `PUSH EBP; MOV EBP,ESP; PUSH ECX; MOV [EBP-4],ECX; MOV ESP,EBP; POP EBP;
/// RET`, so nothing is returned.
pub fn fn_0052d220(e: &mut Engine, this: Ptr) {
    e.call(0x0052_d240, &args![this]);
}

// Translated from 0052baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0052bbd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0052baa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0052_bbd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESImageSpaceModifier`'s vtable (RTTI `.?AVTESImageSpaceModifier@@`).
const IMAGE_SPACE_MODIFIER_VTABLE: u32 = 0x0102_d97c;
/// `NiFloatData`'s and `NiColorData`'s vtables (RTTI `.?AVNiFloatData@@`,
/// `.?AVNiColorData@@`).
const FLOAT_DATA_VTABLE: u32 = 0x0102_dabc;
const COLOR_DATA_VTABLE: u32 = 0x0102_db4c;
/// `NiPointer` members: the pointer (`00559450`), the destructor
/// (`0045cec0`) and the assignment (`0066b0d0`).
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The key-based interpolator's constructor (`00a3c340(this, 0)`) and
/// destructor (`00a3ca30`, `NiAnimation/nikeybasedinterpolator.cpp`).
const KEY_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c340;
const KEY_INTERPOLATOR_DESTRUCT: u32 = 0x00a3_ca30;
/// `NiFloatData::ReplaceAnim` and `NiColorData::ReplaceAnim` (engine map;
/// `thiscall(data, keys, count, type)`).
const FLOAT_DATA_REPLACE_ANIM: u32 = 0x00a3_cdc0;
const COLOR_DATA_REPLACE_ANIM: u32 = 0x00a3_d210;
/// `NiObject`'s constructor (`00a5d3a0`) and the two data destructors
/// (`00a3cc80`, `00a3d120`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const FLOAT_DATA_DESTRUCT: u32 = 0x00a3_cc80;
const COLOR_DATA_DESTRUCT: u32 = 0x00a3_d120;
/// The Gamebryo allocator's new (`00aa13e0(size)`) and delete
/// (`00aa1460(block, size)`), cdecl.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The key constructors the vector constructor gets: the float key's
/// (`0052d200` runs `006240d0`) and the colour key's (`0052d160`); the
/// destructor both get is `0052d220`.
const FLOAT_KEY_CONSTRUCT: u32 = 0x0052_d200;
const COLOR_KEY_CONSTRUCT: u32 = 0x0052_d160;
const KEY_DESTRUCT: u32 = 0x0052_d220;
/// `eh vector constructor iterator(array, size, count, ctor, dtor)` and
/// `eh vector destructor iterator(array, size, count, dtor)`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `TESForm` members: the destructor (`00483630`), the empty `00483710`,
/// `004013e0` (asked first by `InitItem`), `GetFile` (`00484e60`),
/// `AddCompileIndex` (`00485d50`, cdecl), the form lookup (`004839c0`,
/// cdecl), `00484ab0(flag)`, `LoadForm` (`00485110`), the form id
/// (`0084e3a0`).
const TES_FORM_DESTRUCT: u32 = 0x0048_3630;
const TES_FORM_EMPTY: u32 = 0x0048_3710;
const TES_FORM_FIRST_TEST: u32 = 0x0040_13e0;
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const TES_FORM_SET_FLAG: u32 = 0x0048_4ab0;
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
const FORM_ID: u32 = 0x0084_e3a0;
/// `__RTDynamicCast` and the `TESForm` / `TESSound` type descriptors.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_SOUND_TYPE: u32 = 0x0118_32fc;
/// The log call (`005b5e40`, a no-op in this build) and `InitItem`'s two
/// messages ("MASTERFILE: Could not find pIntroSound ..." and "...
/// pOutroSound ...").
const LOG: u32 = 0x005b_5e40;
const MISSING_INTRO_SOUND: u32 = 0x0102_dc30;
const MISSING_OUTRO_SOUND: u32 = 0x0102_dbd8;
/// The empty allocator constructor (`006815c0`), `NiColorA(r, g, b, a)`
/// (`00414430`), `NiPoint2(x, y)` (`00452dc0`), the float key's value
/// setter (`004f5d90`), the clear `007fc2b0(0)` and the fill
/// `00403d30(dest, value, count)`.
const ALLOCATOR_CONSTRUCT: u32 = 0x0068_15c0;
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const FLOAT_KEY_SET_VALUE: u32 = 0x004f_5d90;
const INTERPOLATOR_CLEAR: u32 = 0x007f_c2b0;
const FILL_BYTES: u32 = 0x0040_3d30;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// Data the default keys use: two key times (`0118aba0`) and two values
/// (`0118aba8`); the `float` 0.5 (`01016248`).
const DEFAULT_KEY_TIMES: u32 = 0x0118_aba0;
const DEFAULT_KEY_VALUES: u32 = 0x0118_aba8;
const HALF_F: u32 = 0x0101_6248;
/// `IsExpired`'s constants: the `double` -10000.0 and the `float` clock at
/// `011c96fc`.
const NEVER: u32 = 0x0102_d268;
const CLOCK: u32 = 0x011c_96fc;
/// `TESFile` members (`thiscall` on the file): the form type
/// (`00472660`), the chunk (`004726b0`), the next chunk (`004726f0`), the
/// chunk data (`00472890(buffer, size)`), one word of it
/// (`004727f0(&word)`), the chunk's size (`00401660`) and whether the data
/// needs its byte order swapped (`00401680`); the word swap
/// `00401080(word, 0)` (cdecl).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_CHUNK_WORD: u32 = 0x0047_27f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_SWAPS: u32 = 0x0040_1680;
const SWAP_WORD: u32 = 0x0040_1080;
/// `ImageSpaceModifierData::Endian` (decompiler name; `00531490`).
const DATA_ENDIAN: u32 = 0x0053_1490;

/// The modifier's interpolators: the 0x15 pairs at +0x114 (0x30 apart, the
/// two of a pair 0x18 apart), and the others with their key count and key
/// array fields: (count, keys, interpolator).
const PAIRS: u32 = 0x114;
const PAIR_COUNTS: u32 = 0x28;
const PAIR_KEYS: u32 = 0x654;
const FLOAT_TRACKS: [(u32, u32, u32); 11] = [
    (0xd4, 0x6fc, 0x504),
    (0xd8, 0x700, 0x51c),
    (0xdc, 0x70c, 0x57c),
    (0xe0, 0x710, 0x594),
    (0xe4, 0x714, 0x5ac),
    (0x104, 0x718, 0x5c4),
    (0x108, 0x71c, 0x5dc),
    (0xf4, 0x720, 0x5f4),
    (0xf8, 0x724, 0x60c),
    (0xfc, 0x728, 0x624),
    (0x110, 0x72c, 0x63c),
];
const COLOR_TRACKS: [(u32, u32, u32); 2] = [(0xd0, 0x704, 0x534), (0x10c, 0x708, 0x558)];
/// The other float interpolators, in the destructor's order.
const FLOAT_MEMBERS_REVERSED: [u32; 11] = [
    0x63c, 0x624, 0x60c, 0x5f4, 0x5dc, 0x5c4, 0x5ac, 0x594, 0x57c, 0x51c, 0x504,
];

/// A key array with a count prefix (`new[]`): `count * size + 4` bytes
/// (all ones on overflow), the count, then the keys built by the vector
/// constructor; null when the allocation fails.
fn new_key_array(e: &mut Engine, count: u32, size: u32, construct: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
    let block = e.call(OPERATOR_NEW, &args![bytes]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![block + 4, size, count, construct, KEY_DESTRUCT],
    );
    block + 4
}

/// A float key's time (`FLD`/`FSTP` of the default table's entry).
fn default_time(e: &Engine, index: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(DEFAULT_KEY_TIMES + index * 4))
}

// Translated from 0052baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x18-byte interpolator of the modifier (the vector
/// constructor's element constructor): the key-based interpolator's
/// constructor with 0; answers its result.
pub fn fn_0052baf0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(KEY_INTERPOLATOR_CONSTRUCT, &args![this, 0u32]).u32()
}

// Translated from 0052bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a float interpolator: releases its data pointer at +0x10,
/// then the key-based interpolator's destructor.
// Translation incomplete: 0052bb10 - the FS:[0] exception frame (runs the base destructor if the release throws) is not translated
pub fn fn_0052bb10(e: &mut Engine, this: Ptr) {
    e.call(POINTER_DESTRUCT, &args![this.addr() + 0x10]);
    e.call(KEY_INTERPOLATOR_DESTRUCT, &args![this]);
}

// Translated from 0052bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a colour interpolator: releases its data pointer at
/// +0x1c, then the key-based interpolator's destructor.
// Translation incomplete: 0052bb70 - the FS:[0] exception frame (runs the base destructor if the release throws) is not translated
pub fn fn_0052bb70(e: &mut Engine, this: Ptr) {
    e.call(POINTER_DESTRUCT, &args![this.addr() + 0x1c]);
    e.call(KEY_INTERPOLATOR_DESTRUCT, &args![this]);
}

// Translated from 0052bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::~TESImageSpaceModifier` (the class by its
/// vtable): the vtable, the key clear `0052d260`, the destructors of the
/// float interpolators (+0x63c down to +0x57c), the two colour ones
/// (+0x558, +0x534), the floats at +0x51c and +0x504, the 0x2a pair
/// interpolators at +0x114 (vector destructor), then `TESForm`'s.
// Translation incomplete: 0052bbd0 - the FS:[0] exception frame (destroys the remaining members if one destructor throws) is not translated
pub fn tes_image_space_modifier_destruct(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, IMAGE_SPACE_MODIFIER_VTABLE);
    fn_0052d260(e, this);
    for offset in &FLOAT_MEMBERS_REVERSED[..9] {
        fn_0052bb10(e, Ptr::new(at + offset));
    }
    fn_0052bb70(e, Ptr::new(at + 0x558));
    fn_0052bb70(e, Ptr::new(at + 0x534));
    fn_0052bb10(e, Ptr::new(at + 0x51c));
    fn_0052bb10(e, Ptr::new(at + 0x504));
    e.call(
        VECTOR_DESTRUCT,
        &args![at + PAIRS, 0x18u32, 0x2au32, 0x0052_bb10u32],
    );
    e.call(TES_FORM_DESTRUCT, &args![this]);
}

// Translated from 0052bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::InitializeData` (Xbox PDB): clears the 0xf4
/// bytes of data at +0x20, sets the duration (+0x24) to 1.0, the byte at
/// +0xe8 to 0, the pair at +0xec to (0.5, 0.5) and the bytes at +0x100 and
/// +0x101 to 0; gives every interpolator two default keys: each of the 0x15
/// pairs (key times from `0118aba0`, values from `0118aba8` by the pair's
/// side), the two colour ones (`NiColorA(1, 1, 1, 0)`)
/// and the eleven other float ones (value 0), each with key count 2;
/// clears the two sound ids (+0x1c, +0x18).
// Translation incomplete: 0052bd40 - the FS:[0] exception frame (frees a key array if its vector constructor throws) is not translated
pub fn tes_image_space_modifier_initialize_data(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FILL_BYTES, &args![at + 0x20, 0u32, 0xf4u32]);
    e.mem.set_u8(at + 0x20, 0);
    e.mem.set_f32(at + 0x24, 1.0);
    e.mem.set_u8(at + 0xe8, 0);
    let half = x87_quiet_bits(e.mem.u32(HALF_F));
    // The pair is the frame's `NiPoint2` at `EBP-0x20`: `00452dc0` builds it there and answers
    // its address, and both words are copied through that address, so they are read while it is
    // there. (Scoped to `00452dc0` alone, the copy read a freed block.)
    let (x, y) = e.with_stack(8, |e, point| {
        let pair = e.call(POINT2_CONSTRUCT, &args![point, half, half]).u32();
        (e.mem.u32(pair), e.mem.u32(pair + 4))
    });
    e.mem.set_u32(at + 0xec, x);
    e.mem.set_u32(at + 0xf0, y);
    e.mem.set_u8(at + 0x100, 0);
    e.mem.set_u8(at + 0x101, 0);
    for pair_index in 0..0x15u32 {
        for side in 0..2u32 {
            let slot = pair_index * 8 + side * 4;
            e.mem.set_u32(at + PAIR_COUNTS + slot, 2);
            let keys = new_key_array(e, 2, 8, FLOAT_KEY_CONSTRUCT);
            e.mem.set_u32(at + PAIR_KEYS + slot, keys);
            for key in 0..2u32 {
                let time = default_time(e, key);
                let keys = e.mem.u32(at + PAIR_KEYS + slot);
                fn_0052ce20(e, Ptr::new(keys + key * 8), f32::from_bits(time));
                let value = x87_quiet_bits(e.mem.u32(DEFAULT_KEY_VALUES + side * 4));
                let keys = e.mem.u32(at + PAIR_KEYS + slot);
                e.call(FLOAT_KEY_SET_VALUE, &args![keys + key * 8, value]);
            }
            let count = e.mem.u32(at + PAIR_COUNTS + slot);
            let keys = e.mem.u32(at + PAIR_KEYS + slot);
            let interpolator = at + PAIRS + pair_index * 0x30 + side * 0x18;
            ni_float_interpolator_replace_keys(e, Ptr::new(interpolator), keys, count, 1);
        }
    }
    for (count_at, keys_at, interpolator) in COLOR_TRACKS {
        e.mem.set_u32(at + count_at, 2);
        let keys = new_key_array(e, 2, 0x14, COLOR_KEY_CONSTRUCT);
        e.mem.set_u32(at + keys_at, keys);
        for key in 0..2u32 {
            let time = default_time(e, key);
            let keys = e.mem.u32(at + keys_at);
            fn_0052ce20(e, Ptr::new(keys + key * 0x14), f32::from_bits(time));
            e.with_stack(0x10, |e, color| {
                let made = e
                    .call(
                        COLOR_CONSTRUCT,
                        &args![color, 1.0f32, 1.0f32, 1.0f32, 0.0f32],
                    )
                    .u32();
                let keys = e.mem.u32(at + keys_at);
                fn_0052cfc0(e, Ptr::new(keys + key * 0x14), Ptr::new(made));
            });
        }
        let keys = e.mem.u32(at + keys_at);
        let count = e.mem.u32(at + count_at);
        ni_color_interpolator_replace_keys(e, Ptr::new(at + interpolator), keys, count, 1);
    }
    for (count_at, keys_at, interpolator) in FLOAT_TRACKS {
        e.mem.set_u32(at + count_at, 2);
        let keys = new_key_array(e, 2, 8, FLOAT_KEY_CONSTRUCT);
        e.mem.set_u32(at + keys_at, keys);
        for key in 0..2u32 {
            let time = default_time(e, key);
            let keys = e.mem.u32(at + keys_at);
            fn_0052ce20(e, Ptr::new(keys + key * 8), f32::from_bits(time));
            let keys = e.mem.u32(at + keys_at);
            e.call(FLOAT_KEY_SET_VALUE, &args![keys + key * 8, 0.0f32]);
        }
        let keys = e.mem.u32(at + keys_at);
        let count = e.mem.u32(at + count_at);
        ni_float_interpolator_replace_keys(e, Ptr::new(at + interpolator), keys, count, 1);
    }
    e.mem.set_u32(at + 0x1c, 0);
    e.mem.set_u32(at + 0x18, 0);
}

// Translated from 0052ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A key's time: stores the `float` at `+0` (`FLD`/`FSTP`).
pub fn fn_0052ce20(e: &mut Engine, this: Ptr, time: f32) {
    e.mem.set_u32(this.addr(), x87_quiet_bits(time.to_bits()));
}

// Translated from 0052ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::ReplaceKeys(keys, count, type)` (engine map):
/// with keys, makes the `NiFloatData` at +0x10 when there is none (a new
/// 0x18-byte one, `0052cf30`), hands it the keys (`ReplaceAnim`) and
/// clears +0x14; with none, empties an existing data (`ReplaceAnim(0, 0,
/// 0)`).
// Translation incomplete: 0052ce40 - the FS:[0] exception frame (frees the new data block if its constructor throws) is not translated
pub fn ni_float_interpolator_replace_keys(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    count: u32,
    kind: u32,
) {
    let holder = this.addr() + 0x10;
    if count > 0 {
        if e.call(POINTER_GET, &args![holder]).u32() == 0 {
            let block = e.call(NI_NEW, &args![0x18u32]).u32();
            let data = if block == 0 {
                0
            } else {
                fn_0052cf30(e, Ptr::new(block)).addr()
            };
            e.call(POINTER_ASSIGN, &args![holder, data]);
        }
        let data = e.call(POINTER_GET, &args![holder]).u32();
        e.call(FLOAT_DATA_REPLACE_ANIM, &args![data, keys, count, kind]);
        e.mem.set_u32(this.addr() + 0x14, 0);
    } else if e.call(POINTER_GET, &args![holder]).u32() != 0 {
        let data = e.call(POINTER_GET, &args![holder]).u32();
        e.call(FLOAT_DATA_REPLACE_ANIM, &args![data, 0u32, 0u32, 0u32]);
    }
}

// Translated from 0052cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::NiFloatData` (the class by its vtable): `NiObject`'s
/// constructor, the vtable, the key count, keys and stride words
/// (+8, +0xc, +0x10) cleared and the type byte (+0x14) cleared. Returns
/// `this`.
pub fn fn_0052cf30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    let at = this.addr();
    e.mem.set_u32(at, FLOAT_DATA_VTABLE);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 0x10, 0);
    e.mem.set_u8(at + 0x14, 0);
    this
}

// Translated from 0052cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData`'s scalar deleting destructor (engine map): the destructor
/// `00a3cc80`, then, with bit 0 of `flags`, the Gamebryo delete of 0x18
/// bytes. Returns `this`.
pub fn ni_float_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FLOAT_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 0052cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A colour key's value: copies the four words of `color` to +4.
pub fn fn_0052cfc0(e: &mut Engine, this: Ptr, color: Ptr) {
    for word in 0..4u32 {
        let value = e.mem.u32(color.addr() + word * 4);
        e.mem.set_u32(this.addr() + 4 + word * 4, value);
    }
}

// Translated from 0052cff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorInterpolator::ReplaceKeys(keys, count, type)` (engine map): as
/// the float one, with the `NiColorData` at +0x1c (`0052d0e0`) and +0x20
/// cleared.
// Translation incomplete: 0052cff0 - the FS:[0] exception frame (frees the new data block if its constructor throws) is not translated
pub fn ni_color_interpolator_replace_keys(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    count: u32,
    kind: u32,
) {
    let holder = this.addr() + 0x1c;
    if count > 0 {
        if e.call(POINTER_GET, &args![holder]).u32() == 0 {
            let block = e.call(NI_NEW, &args![0x18u32]).u32();
            let data = if block == 0 {
                0
            } else {
                fn_0052d0e0(e, Ptr::new(block)).addr()
            };
            e.call(POINTER_ASSIGN, &args![holder, data]);
        }
        let data = e.call(POINTER_GET, &args![holder]).u32();
        e.call(COLOR_DATA_REPLACE_ANIM, &args![data, keys, count, kind]);
        e.mem.set_u32(this.addr() + 0x20, 0);
    } else if e.call(POINTER_GET, &args![holder]).u32() != 0 {
        let data = e.call(POINTER_GET, &args![holder]).u32();
        e.call(COLOR_DATA_REPLACE_ANIM, &args![data, 0u32, 0u32, 0u32]);
    }
}

// Translated from 0052d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorData::NiColorData` (the class by its vtable): `NiObject`'s
/// constructor, the vtable, +8, +0xc and +0x10 cleared. Returns `this`.
pub fn fn_0052d0e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    let at = this.addr();
    e.mem.set_u32(at, COLOR_DATA_VTABLE);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 0x10, 0);
    this
}

// Translated from 0052d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorData`'s scalar deleting destructor (engine map): the destructor
/// `00a3d120`, then, with bit 0 of `flags`, the Gamebryo delete of 0x18
/// bytes. Returns `this`.
pub fn ni_color_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(COLOR_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 0052d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The colour key's element constructor: `0052d180`. Returns `this`.
pub fn fn_0052d160(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0052d180(e, this);
    this
}

// Translated from 0052d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Colour key constructor: the empty allocator constructor on `this`,
/// then `NiColorA(0, 0, 0, 0)` at +4. Returns `this`.
// Translation incomplete: 0052d180 - the FS:[0] exception frame (no cleanup after the first step) is not translated
pub fn fn_0052d180(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ALLOCATOR_CONSTRUCT, &args![this]);
    e.call(
        COLOR_CONSTRUCT,
        &args![this.addr() + 4, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
    );
    this
}

// Translated from 0052d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float key's element constructor: `006240d0` on `this`. Returns
/// `this`.
pub fn fn_0052d200(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0062_40d0, &args![this]);
    this
}

// Translated from 0052d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the empty `00483710` on `this`.
pub fn fn_0052d240(e: &mut Engine, this: Ptr) {
    e.call(TES_FORM_EMPTY, &args![this]);
}

// Translated from 0052d260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears every interpolator's keys (`007fc2b0(0)` on the 0x2a pair ones
/// and the eleven float ones, `0052d3b0(0)` on the two colour ones),
/// runs the empty `00483710` and clears the two sound ids (+0x1c, +0x18).
pub fn fn_0052d260(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for pair_index in 0..0x15u32 {
        for side in 0..2u32 {
            let interpolator = at + PAIRS + pair_index * 0x30 + side * 0x18;
            e.call(INTERPOLATOR_CLEAR, &args![interpolator, 0u32]);
        }
    }
    fn_0052d3b0(e, Ptr::new(at + 0x534), 0);
    fn_0052d3b0(e, Ptr::new(at + 0x558), 0);
    for offset in [
        0x504u32, 0x51c, 0x57c, 0x594, 0x5ac, 0x5c4, 0x5dc, 0x5f4, 0x60c, 0x624, 0x63c,
    ] {
        e.call(INTERPOLATOR_CLEAR, &args![at + offset, 0u32]);
    }
    e.call(TES_FORM_EMPTY, &args![this]);
    e.mem.set_u32(at + 0x1c, 0);
    e.mem.set_u32(at + 0x18, 0);
}

// Translated from 0052d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A colour interpolator's data assignment: `NiPointer` assignment at
/// +0x1c.
pub fn fn_0052d3b0(e: &mut Engine, this: Ptr, data: u32) {
    e.call(POINTER_ASSIGN, &args![this.addr() + 0x1c, data]);
}

/// `InitItem`'s resolution of one sound id field: the form for the id
/// (with the file's compile index), cast to `TESSound`; a missing one is
/// logged with `message`.
fn resolve_sound(e: &mut Engine, this: Ptr, field: u32, message: u32) {
    let id = e.mem.u32(this.addr() + field);
    if id == 0 {
        return;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        let file = e
            .call(TES_FORM_GET_FILE, &args![this, 0xffff_ffffu32])
            .u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![local, file]);
        let id = e.mem.u32(local.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let sound = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TES_FORM_TYPE, TES_SOUND_TYPE, 0u32],
            )
            .u32();
        e.mem.set_u32(this.addr() + field, sound);
        if e.mem.u32(this.addr() + field) == 0 {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            let id = e.mem.u32(local.addr());
            e.call(LOG, &args![message, id, name, form_id]);
        }
    });
}

// Translated from 0052d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::InitItem` (Xbox PDB): unless `004013e0`
/// answers true, resolves the intro sound id (+0x1c) and the outro sound
/// id (+0x18) to `TESSound` forms (logging a missing one) and runs
/// `00484ab0(1)`.
pub fn tes_image_space_modifier_init_item(e: &mut Engine, this: Ptr) {
    if e.call(TES_FORM_FIRST_TEST, &args![this]).bool() {
        return;
    }
    resolve_sound(e, this, 0x1c, MISSING_INTRO_SOUND);
    resolve_sound(e, this, 0x18, MISSING_OUTRO_SOUND);
    e.call(TES_FORM_SET_FLAG, &args![this, 1u32]);
}

/// One keyed chunk of `Load`: a new key array of the count at
/// `count_at` (8-byte float keys or 0x14-byte colour keys) stored at
/// `keys_at`, filled from the chunk, byte-swapped word by word when the
/// file asks for it, and handed to the interpolator.
fn load_track(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    (count_at, keys_at, interpolator): (u32, u32, u32),
    color: bool,
) {
    let at = this.addr();
    let count = e.mem.u32(at + count_at);
    let (size, construct) = if color {
        (0x14, COLOR_KEY_CONSTRUCT)
    } else {
        (8, FLOAT_KEY_CONSTRUCT)
    };
    let keys = new_key_array(e, count, size, construct);
    e.mem.set_u32(at + keys_at, keys);
    let bytes = if color {
        count.wrapping_mul(0x14)
    } else {
        count << 3
    };
    let keys = e.mem.u32(at + keys_at);
    e.call(FILE_CHUNK_DATA, &args![file, keys, bytes]);
    if e.call(FILE_SWAPS, &args![file]).bool() {
        let keys = e.mem.u32(at + keys_at);
        let mut word = 0;
        while word < bytes >> 2 {
            e.call(
                SWAP_WORD,
                &args![keys.wrapping_add(word.wrapping_mul(4)), 0u32],
            );
            word += 1;
        }
    }
    let keys = e.mem.u32(at + keys_at);
    if color {
        ni_color_interpolator_replace_keys(e, Ptr::new(at + interpolator), keys, count, 1);
    } else {
        ni_float_interpolator_replace_keys(e, Ptr::new(at + interpolator), keys, count, 1);
    }
}

/// `u32::from_le_bytes` of a record or chunk tag.
const fn tag(text: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*text)
}

// Translated from 0052d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::Load` (Xbox PDB): answers false unless the
/// file's form is type 0x54; otherwise `TESForm::LoadForm`,
/// `00484ab0(0)`, then for every chunk: `EDID` (the editor id, through
/// vtable slot `+0x134`), `OBND` (slot `+0xe0`), `DNAM` (the 0xf4 bytes of
/// data at +0x20, byte-swapped by `ImageSpaceModifierData::Endian` when
/// the file asks), `RDSD`/`RDSI` (the intro and outro sound ids, +0x1c and
/// +0x18), the keyed chunks (`BNAM`, `VNAM`, `RNAM`, `SNAM`, `UNAM`,
/// `NAM1`, `NAM2`, `WNAM`, `XNAM`, `YNAM`, `NAM4` float, `TNAM`, `NAM3`
/// colour) and the `?IAD` range (0x44414900..=0x444149ff: pair
/// `tag & 0x3f`, side `(tag & 0xc0) >> 6`, the index not bounded). Answers
/// true.
///
/// The compiler's `switch` on `tag - 'DNAM'` (byte map `0052ed74`, jump
/// table `0052ed4c`) and its compare tree are the `match` arms.
// Translation incomplete: 0052d510 - the FS:[0] exception frame and the security-cookie check (00ec408c) are not translated
pub fn tes_image_space_modifier_load(e: &mut Engine, this: Ptr, file: u32) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x54 {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(TES_FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let float_track = |name: &[u8; 4]| -> Option<(u32, u32, u32)> {
            let index = match name {
                b"BNAM" => 0,
                b"VNAM" => 1,
                b"RNAM" => 2,
                b"SNAM" => 3,
                b"UNAM" => 4,
                b"NAM1" => 5,
                b"NAM2" => 6,
                b"WNAM" => 7,
                b"XNAM" => 8,
                b"YNAM" => 9,
                b"NAM4" => 10,
                _ => return None,
            };
            Some(FLOAT_TRACKS[index])
        };
        let name = chunk.to_le_bytes();
        if let Some(track) = float_track(&name) {
            load_track(e, this, file, track, false);
        } else if chunk == tag(b"TNAM") {
            load_track(e, this, file, COLOR_TRACKS[0], true);
        } else if chunk == tag(b"NAM3") {
            load_track(e, this, file, COLOR_TRACKS[1], true);
        } else if chunk == tag(b"EDID") {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            // The `_alloca` buffer of the chunk's size (rounded to 16).
            let buffer = e.mem.alloc(size.wrapping_add(15) & !15);
            e.call(FILE_CHUNK_DATA, &args![file, buffer, 0x200u32]);
            e.vcall(this.addr(), 0x134, &args![buffer]);
            e.mem.free(buffer);
        } else if chunk == tag(b"OBND") {
            e.vcall(this.addr(), 0xe0, &args![file]);
        } else if chunk == tag(b"DNAM") {
            e.call(FILE_CHUNK_DATA, &args![file, this.addr() + 0x20, 0xf4u32]);
            if e.call(FILE_SWAPS, &args![file]).bool() {
                e.call(DATA_ENDIAN, &args![this.addr() + 0x20]);
            }
            // The exe also sets a local "data loaded" byte here, which
            // nothing reads.
        } else if chunk == tag(b"RDSD") || chunk == tag(b"RDSI") {
            let field = if chunk == tag(b"RDSD") { 0x1c } else { 0x18 };
            let id = e.with_stack(4, |e, word| {
                e.call(FILE_CHUNK_WORD, &args![file, word]);
                e.mem.u32(word.addr())
            });
            e.mem.set_u32(this.addr() + field, id);
        } else if (0x4441_4900..=0x4441_49ff).contains(&(chunk as i32)) {
            let pair_index = chunk & 0x3f;
            let side = ((chunk & 0xc0) as i32 >> 6) as u32;
            let slot = pair_index
                .wrapping_mul(8)
                .wrapping_add(side.wrapping_mul(4));
            let interpolator = PAIRS
                .wrapping_add(pair_index.wrapping_mul(0x30))
                .wrapping_add(side.wrapping_mul(0x18));
            load_track(
                e,
                this,
                file,
                (PAIR_COUNTS + slot, PAIR_KEYS + slot, interpolator),
                false,
            );
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005316c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::IsExpired(start)` (Xbox PDB): false unless the
/// byte at +0x20 is set and `start` is not below -10000 (a NaN passes);
/// then whether the duration (+0x24) is below the clock (`011c96fc`)
/// minus `start` (false for NaN).
pub fn tes_image_space_modifier_is_expired(e: &mut Engine, this: Ptr, start: f32) -> bool {
    if e.mem.u8(this.addr() + 0x20) == 0 {
        return false;
    }
    let start_value = x87_load32(start.to_bits());
    // `TEST AH,0x5; JP`: on to the clock unless `start < -10000`.
    if start_value < e.global::<f64>(NEVER) {
        return false;
    }
    let elapsed = x87_sub(x87_load32(e.mem.u32(CLOCK)), x87_m32(start.to_bits()));
    let duration = x87_load32(e.mem.u32(this.addr() + 0x24));
    // `FCOMPP` of the duration with the elapsed time; `TEST AH,0x5; JP`.
    duration < elapsed
}

/// The keyed tracks in the order `FlushData`, `Save`, `Copy` and `Compare`
/// take them: (key count field, key array field, interpolator, chunk,
/// colour keys).
const KEYED_TRACKS: [(u32, u32, u32, u32, bool); 13] = [
    (0xd4, 0x6fc, 0x504, 0x4d41_4e42, false),
    (0xd8, 0x700, 0x51c, 0x4d41_4e56, false),
    (0xd0, 0x704, 0x534, 0x4d41_4e54, true),
    (0x10c, 0x708, 0x558, 0x334d_414e, true),
    (0xdc, 0x70c, 0x57c, 0x4d41_4e52, false),
    (0xe0, 0x710, 0x594, 0x4d41_4e53, false),
    (0xe4, 0x714, 0x5ac, 0x4d41_4e55, false),
    (0x104, 0x718, 0x5c4, 0x314d_414e, false),
    (0x108, 0x71c, 0x5dc, 0x324d_414e, false),
    (0xf4, 0x720, 0x5f4, 0x4d41_4e57, false),
    (0xf8, 0x724, 0x60c, 0x4d41_4e58, false),
    (0xfc, 0x728, 0x624, 0x4d41_4e59, false),
    (0x110, 0x72c, 0x63c, 0x344d_414e, false),
];
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESForm::__AddChunkData` (`00485990`, `cdecl (chunk, data, size)`),
/// `TESForm::AddChunk` with a word (`00485910`, `cdecl`), the byte
/// `00401500` answers (whether saving swaps bytes) and `operator delete`
/// (`00401030`, `cdecl`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
const FORM_ADD_CHUNK_WORD: u32 = 0x0048_5910;
const SAVE_SWAPS: u32 = 0x0040_1500;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the copy `00401460` (`cdecl (to, from, size)`) and
/// `memcmp` (`00ec4835`).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The `TESImageSpaceModifier` type descriptor (`011862b8`, its name read
/// at +8).
const TES_IMAGE_SPACE_MODIFIER_TYPE: u32 = 0x0118_62b8;
/// The RTTI records `NiFloatData::GetRTTI` and the colour data's answer
/// (`011f3748`, `011f3750`).
const FLOAT_DATA_RTTI: u32 = 0x011f_3748;
const COLOR_DATA_RTTI: u32 = 0x011f_3750;

/// Side `side` of pair `pair` (0..0x15, 0..2): its key count field, key
/// array field and interpolator.
fn pair_track(pair: u32, side: u32) -> (u32, u32, u32) {
    let slot = pair * 8 + side * 4;
    (
        PAIR_COUNTS + slot,
        PAIR_KEYS + slot,
        PAIRS + pair * 0x30 + side * 0x18,
    )
}

// Translated from 0052cf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::GetRTTI` (engine map, `thiscall`): answers the RTTI
/// record at `011f3748`; `this` is not read.
pub fn ni_float_data_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    FLOAT_DATA_RTTI
}

// Translated from 0052d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorData::GetRTTI` (engine map, `thiscall`): answers the RTTI
/// record at `011f3750`; `this` is not read.
pub fn ni_color_data_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    COLOR_DATA_RTTI
}

// Translated from 0052ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::FlushData` (Xbox PDB): takes every
/// interpolator's key count (virtual slot `+0xE0`, given 0) and key array
/// (slot `+0xEC`, given 0) back into the modifier's fields: the 0x15 pairs
/// first, then the other tracks.
pub fn tes_image_space_modifier_flush_data(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for pair in 0..0x15u32 {
        for side in 0..2u32 {
            let (count, keys, interpolator) = pair_track(pair, side);
            let value = e.vcall(at + interpolator, 0xe0, &args![0u32]).u32();
            e.mem.set_u32(at + count, value);
            let value = e.vcall(at + interpolator, 0xec, &args![0u32]).u32();
            e.mem.set_u32(at + keys, value);
        }
    }
    for (count, keys, interpolator, _, _) in KEYED_TRACKS {
        let value = e.vcall(at + interpolator, 0xe0, &args![0u32]).u32();
        e.mem.set_u32(at + count, value);
        let value = e.vcall(at + interpolator, 0xec, &args![0u32]).u32();
        e.mem.set_u32(at + keys, value);
    }
}

/// One keyed chunk of `Save`: `(size * count) >> 2` words copied into a new
/// block (`operator new` of four times that, all ones on overflow),
/// swapped word by word when saving swaps bytes, added as `chunk` and
/// freed.
fn save_track(e: &mut Engine, this: u32, count_at: u32, keys_at: u32, chunk: u32, size: u32) {
    let words = size.wrapping_mul(e.mem.u32(this + count_at)) >> 2;
    let bytes = words.checked_mul(4).unwrap_or(u32::MAX);
    let block = e.call(OPERATOR_NEW, &args![bytes]).u32();
    let keys = e.mem.u32(this + keys_at);
    e.call(COPY_BYTES, &args![block, keys, words << 2]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        let mut word = 0u32;
        while word < words {
            e.call(SWAP_WORD, &args![block.wrapping_add(word * 4), 0u32]);
            word += 1;
        }
    }
    e.call(FORM_ADD_CHUNK_DATA, &args![chunk, block, words << 2]);
    e.call(OPERATOR_DELETE, &args![block]);
}

// Translated from 0052f250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::Save` (Xbox PDB): starts the form, flushes the
/// interpolators' keys back ([`tes_image_space_modifier_flush_data`]), adds
/// the 0xF4-byte data at +0x20 as `DNAM` (swapped by
/// `ImageSpaceModifierData::Endian` before and after when saving swaps
/// bytes), each keyed track (`BNAM`, `VNAM`, `TNAM`, `NAM3`, `RNAM`,
/// `SNAM`, `UNAM`, `NAM1`, `NAM2`, `WNAM`, `XNAM`, `YNAM`, `NAM4`) and
/// each pair (`0x44414900 | pair | side << 6`), then with an intro sound
/// (+0x1C) its form id as `RDSD` and with an outro sound (+0x18) as
/// `RDSI`; closes the form.
pub fn tes_image_space_modifier_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FORM_START, &args![this]);
    tes_image_space_modifier_flush_data(e, this);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        image_space_modifier_data_endian(e, Ptr::new(at + 0x20));
    }
    e.call(
        FORM_ADD_CHUNK_DATA,
        &args![0x4d41_4e44u32, at + 0x20, 0xf4u32],
    );
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        image_space_modifier_data_endian(e, Ptr::new(at + 0x20));
    }
    for (count, keys, _, chunk, color) in KEYED_TRACKS {
        let size = if color { 0x14 } else { 8 };
        save_track(e, at, count, keys, chunk, size);
    }
    for pair in 0..0x15u32 {
        for side in 0..2u32 {
            let (count, keys, _) = pair_track(pair, side);
            let chunk = pair | 0x4441_4900 | side << 6;
            save_track(e, at, count, keys, chunk, 8);
        }
    }
    let intro = e.mem.u32(at + 0x1c);
    if intro != 0 {
        let id = e.call(FORM_ID, &args![intro]).u32();
        e.call(FORM_ADD_CHUNK_WORD, &args![0x4453_4452u32, id]);
    }
    let outro = e.mem.u32(at + 0x18);
    if outro != 0 {
        let id = e.call(FORM_ID, &args![outro]).u32();
        e.call(FORM_ADD_CHUNK_WORD, &args![0x4953_4452u32, id]);
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// One keyed track of `Copy`: a new key array of this modifier's count
/// (already copied with the data), filled from `source`'s keys and handed
/// to the interpolator (`ReplaceKeys` with type 1).
fn copy_track(
    e: &mut Engine,
    this: u32,
    source: u32,
    (count_at, keys_at, interpolator): (u32, u32, u32),
    color: bool,
) {
    let count = e.mem.u32(this + count_at);
    let (size, construct) = if color {
        (0x14, COLOR_KEY_CONSTRUCT)
    } else {
        (8, FLOAT_KEY_CONSTRUCT)
    };
    let keys = new_key_array(e, count, size, construct);
    e.mem.set_u32(this + keys_at, keys);
    let to = e.mem.u32(this + keys_at);
    let from = e.mem.u32(source + keys_at);
    let bytes = if color {
        count.wrapping_mul(0x14)
    } else {
        count << 3
    };
    e.call(COPY_BYTES, &args![to, from, bytes]);
    let keys = e.mem.u32(this + keys_at);
    if color {
        ni_color_interpolator_replace_keys(e, Ptr::new(this + interpolator), keys, count, 1);
    } else {
        ni_float_interpolator_replace_keys(e, Ptr::new(this + interpolator), keys, count, 1);
    }
}

// Translated from 00530160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00530160 - the C++ exception frame (FS:[0], states for the fourteen key array allocations) is not translated
/// `TESImageSpaceModifier::Copy` (Xbox PDB): when `other` is a
/// `TESImageSpaceModifier`, flushes its keys
/// ([`tes_image_space_modifier_flush_data`] on `other`), copies all
/// components, runs virtual slot `+0x18`, takes its two sounds (+0x1C,
/// +0x18) and its 0xF4-byte data, then gives every keyed track and every
/// pair a new key array copied from `other`'s.
pub fn tes_image_space_modifier_copy(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_FORM_TYPE,
                TES_IMAGE_SPACE_MODIFIER_TYPE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    let at = this.addr();
    tes_image_space_modifier_flush_data(e, Ptr::new(source));
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    e.vcall(at, 0x18, &args![]);
    let intro = e.mem.u32(source + 0x1c);
    e.mem.set_u32(at + 0x1c, intro);
    let outro = e.mem.u32(source + 0x18);
    e.mem.set_u32(at + 0x18, outro);
    e.call(COPY_BYTES, &args![at + 0x20, source + 0x20, 0xf4u32]);
    for (count, keys, interpolator, _, color) in KEYED_TRACKS {
        copy_track(e, at, source, (count, keys, interpolator), color);
    }
    for pair in 0..0x15u32 {
        for side in 0..2u32 {
            copy_track(e, at, source, pair_track(pair, side), false);
        }
    }
}

// Translated from 005310b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::Compare` (Xbox PDB): true (different) when
/// `other` is no `TESImageSpaceModifier`; otherwise flushes both
/// ([`tes_image_space_modifier_flush_data`]) and answers true at the first
/// difference: the intro sound, the outro sound, `CompareAllComponents`,
/// the 0xF4-byte data, each keyed track's keys (`memcmp` of this
/// modifier's count of keys) and each pair's keys; else false.
pub fn tes_image_space_modifier_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_FORM_TYPE,
                TES_IMAGE_SPACE_MODIFIER_TYPE,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    let at = this.addr();
    tes_image_space_modifier_flush_data(e, this);
    tes_image_space_modifier_flush_data(e, Ptr::new(source));
    if e.mem.u32(at + 0x1c) != e.mem.u32(source + 0x1c) {
        return true;
    }
    if e.mem.u32(at + 0x18) != e.mem.u32(source + 0x18) {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    if e.call(MEMCMP, &args![at + 0x20, source + 0x20, 0xf4u32])
        .u32()
        != 0
    {
        return true;
    }
    for (count, keys, _, _, color) in KEYED_TRACKS {
        let count = e.mem.u32(at + count);
        let bytes = if color {
            count.wrapping_mul(0x14)
        } else {
            count << 3
        };
        let mine = e.mem.u32(at + keys);
        let theirs = e.mem.u32(source + keys);
        if e.call(MEMCMP, &args![mine, theirs, bytes]).u32() != 0 {
            return true;
        }
    }
    for pair in 0..0x15u32 {
        for side in 0..2u32 {
            let (count, keys, _) = pair_track(pair, side);
            let bytes = e.mem.u32(at + count) << 3;
            let mine = e.mem.u32(at + keys);
            let theirs = e.mem.u32(source + keys);
            if e.call(MEMCMP, &args![mine, theirs, bytes]).u32() != 0 {
                return true;
            }
        }
    }
    false
}

// Translated from 00531490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::ImageSpaceModifierData::Endian` (Xbox PDB):
/// swaps the words of the modifier data (`00401080` with flag 0): +4, the
/// 0x2A pair counts from +8, then +0xB4, +0xB8, +0xB0, +0xEC, +0xBC, +0xC0,
/// +0xC4, +0xE4, +0xE8, +0xCC, +0xD0, +0xD4, +0xD8, +0xDC, +0xF0.
pub fn image_space_modifier_data_endian(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(SWAP_WORD, &args![at + 4, 0u32]);
    for pair in 0..0x15u32 {
        for side in 0..2u32 {
            e.call(SWAP_WORD, &args![at + 8 + pair * 8 + side * 4, 0u32]);
        }
    }
    for offset in [
        0xb4u32, 0xb8, 0xb0, 0xec, 0xbc, 0xc0, 0xc4, 0xe4, 0xe8, 0xcc, 0xd0, 0xd4, 0xd8, 0xdc, 0xf0,
    ] {
        e.call(SWAP_WORD, &args![at + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_d220, fn_0052d220(Ptr)),
        entry!(0x0052_b8f0, fn_0052b8f0(Ptr) -> Ptr),
        entry!(0x0052_bad0, fn_0052bad0(Ptr) -> Ptr),
        entry!(0x0052_baa0, fn_0052baa0(Ptr, u32) -> Ptr),
        entry!(0x0052baf0, fn_0052baf0(Ptr) -> u32),
        entry!(0x0052bb10, fn_0052bb10(Ptr)),
        entry!(0x0052bb70, fn_0052bb70(Ptr)),
        entry!(0x0052bbd0, tes_image_space_modifier_destruct(Ptr)),
        entry!(0x0052bd40, tes_image_space_modifier_initialize_data(Ptr)),
        entry!(0x0052ce20, fn_0052ce20(Ptr, f32)),
        entry!(
            0x0052ce40,
            ni_float_interpolator_replace_keys(Ptr, u32, u32, u32)
        ),
        entry!(0x0052cf30, fn_0052cf30(Ptr) -> Ptr),
        entry!(
            0x0052cf90,
            ni_float_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0052cfc0, fn_0052cfc0(Ptr, Ptr)),
        entry!(
            0x0052cff0,
            ni_color_interpolator_replace_keys(Ptr, u32, u32, u32)
        ),
        entry!(0x0052d0e0, fn_0052d0e0(Ptr) -> Ptr),
        entry!(
            0x0052d130,
            ni_color_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0052d160, fn_0052d160(Ptr) -> Ptr),
        entry!(0x0052d180, fn_0052d180(Ptr) -> Ptr),
        entry!(0x0052d200, fn_0052d200(Ptr) -> Ptr),
        entry!(0x0052d240, fn_0052d240(Ptr)),
        entry!(0x0052d260, fn_0052d260(Ptr)),
        entry!(0x0052d3b0, fn_0052d3b0(Ptr, u32)),
        entry!(0x0052d3d0, tes_image_space_modifier_init_item(Ptr)),
        entry!(0x0052d510, tes_image_space_modifier_load(Ptr, u32) -> bool),
        entry!(
            0x005316c0,
            tes_image_space_modifier_is_expired(Ptr, f32) -> bool
        ),
        entry!(0x0052_cf80, ni_float_data_get_rtti(Ptr) -> u32),
        entry!(0x0052_d120, ni_color_data_get_rtti(Ptr) -> u32),
        entry!(0x0052_ed90, tes_image_space_modifier_flush_data(Ptr)),
        entry!(0x0052_f250, tes_image_space_modifier_save(Ptr)),
        entry!(0x0053_0160, tes_image_space_modifier_copy(Ptr, Ptr)),
        entry!(0x0053_10b0, tes_image_space_modifier_compare(Ptr, Ptr) -> bool),
        entry!(0x0053_1490, image_space_modifier_data_endian(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0052b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_b8f0,
            size: 0x730,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0052_bad0, &[A::T(0x20)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x114),
                        A::V(0x18),
                        A::V(0x2a),
                        A::V(0x0052_baf0),
                        A::V(0x0052_bb10),
                    ],
                ),
                (0x00a3_c340, &[A::T(0x504), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x51c), A::V(0x0)]),
                (0x00a3_bd10, &[A::T(0x534), A::V(0x0)]),
                (0x00a3_bd10, &[A::T(0x558), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x57c), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x594), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5ac), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5c4), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5dc), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5f4), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x60c), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x624), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x63c), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x54)]),
                (0x0052_bd40, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d97c)],
        });
    }

    #[test]
    fn fn_0052b8f0_runs_the_extra_call_on_the_default_object() {
        // The object at `011c9750` also gets `00484490` called on it, last.
        let mut e = Engine::new();
        e.mem.map(0x011c_9750, 0x730);
        for a in [
            0x0048_3370u32,
            0x0052_bad0,
            0x00ec_782f,
            0x00a3_c340,
            0x00a3_bd10,
            0x004f_15a0,
            0x0052_bd40,
            0x0048_4490,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0052_b8f0, &args![0x011c_9750u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(0x0048_4490, vec![0x011c_9750]));
    }
}

#[cfg(test)]
mod thunk_tests {
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

    #[test]
    fn fwd_a03_0052d220_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0052_d240, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0052_d220, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0052_d240), vec![vec![0x5000]]);
    }

    #[test]
    fn at_0052baa0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0052_bbd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0052_baa0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0052_baa0)
                .collect();
            let mut expected = vec![(0x0052_bbd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod process_list_tests {
    use crate::prelude::*;

    #[test]
    fn fn_0052bad0_runs_the_member_constructor_and_returns_this() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_bad0, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        let members: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0068_15c0).collect();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].1, vec![0x50cc]);
    }
}

#[cfg(test)]
mod image_space_modifier_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// An engine with the pointer, allocation and data doubles the key
    /// code uses, and the pages of the constants.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0118_a000u32, 0x0101_6000, 0x0102_d000, 0x011c_9000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(DEFAULT_KEY_TIMES, 0.0);
        e.mem.set_f32(DEFAULT_KEY_TIMES + 4, 1.0);
        e.mem.set_f32(DEFAULT_KEY_VALUES, 0.25);
        e.mem.set_f32(DEFAULT_KEY_VALUES + 4, 0.75);
        e.mem.set_f32(HALF_F, 0.5);
        e.mem.set_f64(NEVER, -10000.0);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for addr in [
            VECTOR_CONSTRUCT,
            NI_OBJECT_CONSTRUCT,
            FLOAT_DATA_REPLACE_ANIM,
            COLOR_DATA_REPLACE_ANIM,
            FILL_BYTES,
        ] {
            e.register(addr, no_op);
        }
        e.register(FLOAT_KEY_SET_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e
    }

    fn modifier(e: &mut Engine) -> u32 {
        e.mem.alloc(0x730)
    }

    #[test]
    fn interpolator_constructor_and_destructors() {
        let mut e = engine();
        e.register(KEY_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(KEY_INTERPOLATOR_DESTRUCT, no_op);
        e.register(POINTER_DESTRUCT, no_op);
        let (r, log) = run(&mut e, 0x0052_baf0, &args![0x4000u32]);
        assert_eq!(r.u32(), 0x4000);
        assert_eq!(
            calls(&log, KEY_INTERPOLATOR_CONSTRUCT),
            vec![vec![0x4000, 0]]
        );
        for (addr, holder) in [(0x0052_bb10u32, 0x10u32), (0x0052_bb70, 0x1c)] {
            let (_, log) = run(&mut e, addr, &args![0x4000u32]);
            let order: Log = log.into_iter().skip(1).collect();
            assert_eq!(
                order,
                vec![
                    (POINTER_DESTRUCT, vec![0x4000 + holder]),
                    (KEY_INTERPOLATOR_DESTRUCT, vec![0x4000]),
                ]
            );
        }
    }

    #[test]
    fn destructor_clears_then_destroys_every_member() {
        let mut e = engine();
        for addr in [
            KEY_INTERPOLATOR_DESTRUCT,
            POINTER_DESTRUCT,
            VECTOR_DESTRUCT,
            TES_FORM_DESTRUCT,
            INTERPOLATOR_CLEAR,
            TES_FORM_EMPTY,
        ] {
            e.register(addr, no_op);
        }
        let this = modifier(&mut e);
        let (_, log) = run(&mut e, 0x0052_bbd0, &args![this]);
        assert_eq!(e.mem.u32(this), IMAGE_SPACE_MODIFIER_VTABLE);
        let released: Vec<u32> = calls(&log, POINTER_DESTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        let mut want: Vec<u32> = FLOAT_MEMBERS_REVERSED[..9]
            .iter()
            .map(|o| o + 0x10)
            .collect();
        want.extend([0x558 + 0x1c, 0x534 + 0x1c, 0x51c + 0x10, 0x504 + 0x10]);
        assert_eq!(released, want);
        assert_eq!(
            calls(&log, VECTOR_DESTRUCT),
            vec![vec![this + 0x114, 0x18, 0x2a, 0x0052_bb10]]
        );
        assert_eq!(calls(&log, TES_FORM_DESTRUCT), vec![vec![this]]);
        // The key clear ran first.
        assert_eq!(calls(&log, INTERPOLATOR_CLEAR).len(), 0x2a + 11);
    }

    #[test]
    fn initialize_data_gives_every_interpolator_two_keys() {
        let mut e = engine();
        let this = modifier(&mut e);
        let (_, log) = run(&mut e, 0x0052_bd40, &args![this]);
        assert_eq!(calls(&log, FILL_BYTES), vec![vec![this + 0x20, 0, 0xf4]]);
        assert_eq!(e.mem.f32(this + 0x24), 1.0);
        assert_eq!(e.mem.f32(this + 0xec), 0.5);
        assert_eq!(e.mem.f32(this + 0xf0), 0.5);
        // Pair 3, side 1: count 2, keys with times 0 and 1 and value 0.75.
        let slot = 3 * 8 + 4;
        assert_eq!(e.mem.u32(this + PAIR_COUNTS + slot), 2);
        let keys = e.mem.u32(this + PAIR_KEYS + slot);
        assert_eq!(e.mem.u32(keys - 4), 2);
        assert_eq!(e.mem.f32(keys), 0.0);
        assert_eq!(e.mem.f32(keys + 8), 1.0);
        assert_eq!(e.mem.f32(keys + 4), 0.75);
        assert_eq!(e.mem.f32(keys + 12), 0.75);
        // Every float interpolator's data got its keys.
        let floats = calls(&log, FLOAT_DATA_REPLACE_ANIM);
        assert_eq!(floats.len(), 0x2a + 11);
        assert!(floats.iter().all(|w| w[2] == 2 && w[3] == 1));
        let pair_data = e.mem.u32(this + PAIRS + 3 * 0x30 + 0x18 + 0x10);
        assert!(floats.contains(&vec![pair_data, keys, 2, 1]));
        assert_eq!(e.mem.u32(pair_data), FLOAT_DATA_VTABLE);
        // The colour keys: white with alpha 0.
        let colors = calls(&log, COLOR_DATA_REPLACE_ANIM);
        assert_eq!(colors.len(), 2);
        let keys = e.mem.u32(this + 0x708);
        assert_eq!(colors[1][1..], [keys, 2, 1]);
        assert_eq!(e.mem.f32(keys + 0x14), 1.0);
        let rgba: Vec<f32> = (0..4).map(|i| e.mem.f32(keys + 4 + 4 * i)).collect();
        assert_eq!(rgba, vec![1.0, 1.0, 1.0, 0.0]);
        // A float track: value 0.
        let keys = e.mem.u32(this + 0x72c);
        assert_eq!(e.mem.u32(this + 0x110), 2);
        assert_eq!(e.mem.f32(keys + 4), 0.0);
        // The vector constructor built each array.
        let built = calls(&log, VECTOR_CONSTRUCT);
        assert_eq!(built.len(), 0x2a + 2 + 11);
        assert_eq!(built[0][1..], [8, 2, FLOAT_KEY_CONSTRUCT, KEY_DESTRUCT]);
        assert_eq!(
            built[0x2a][1..],
            [0x14, 2, COLOR_KEY_CONSTRUCT, KEY_DESTRUCT]
        );
    }

    /// The pair at +0xec is copied from the frame's `NiPoint2` (`EBP-0x20`) while it is still
    /// there: `00452dc0` builds it there and answers its address. With the freed-block check a
    /// point freed before the copy would give the fill word.
    #[test]
    fn initialize_data_copies_the_pair_from_a_live_point() {
        let mut e = engine();
        e.mem.enable_freed_check(true);
        let this = modifier(&mut e);
        e.call(0x0052_bd40, &args![this]);
        assert_eq!(e.mem.f32(this + 0xec), 0.5);
        assert_eq!(e.mem.f32(this + 0xf0), 0.5);
    }

    #[test]
    fn key_time_is_a_float_copy() {
        let mut e = engine();
        let key = e.mem.alloc(8);
        e.call(0x0052_ce20, &args![key, 2.5f32]);
        assert_eq!(e.mem.f32(key), 2.5);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.call(0x0052_ce20, &args![key, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(key), 0x7fc0_0001);
    }

    #[test]
    fn float_replace_keys_makes_data_once() {
        let mut e = engine();
        let interpolator = e.mem.alloc(0x18);
        let (_, log) = run(
            &mut e,
            0x0052_ce40,
            &args![interpolator, 0x500u32, 3u32, 1u32],
        );
        let data = e.mem.u32(interpolator + 0x10);
        assert_eq!(calls(&log, NI_NEW), vec![vec![0x18]]);
        assert_eq!(
            calls(&log, FLOAT_DATA_REPLACE_ANIM),
            vec![vec![data, 0x500, 3, 1]]
        );
        e.mem.set_u32(interpolator + 0x14, 9);
        let (_, log) = run(
            &mut e,
            0x0052_ce40,
            &args![interpolator, 0x600u32, 1u32, 2u32],
        );
        assert!(calls(&log, NI_NEW).is_empty());
        assert_eq!(
            calls(&log, FLOAT_DATA_REPLACE_ANIM),
            vec![vec![data, 0x600, 1, 2]]
        );
        assert_eq!(e.mem.u32(interpolator + 0x14), 0);
        // No keys: the data is emptied.
        let (_, log) = run(
            &mut e,
            0x0052_ce40,
            &args![interpolator, 0x600u32, 0u32, 2u32],
        );
        assert_eq!(
            calls(&log, FLOAT_DATA_REPLACE_ANIM),
            vec![vec![data, 0, 0, 0]]
        );
        // No keys and no data: nothing.
        let bare = e.mem.alloc(0x18);
        let (_, log) = run(&mut e, 0x0052_ce40, &args![bare, 0u32, 0u32, 0u32]);
        assert!(calls(&log, FLOAT_DATA_REPLACE_ANIM).is_empty());
        // A failed allocation assigns null.
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        let (_, log) = run(&mut e, 0x0052_ce40, &args![bare, 0x500u32, 1u32, 1u32]);
        assert_eq!(calls(&log, POINTER_ASSIGN), vec![vec![bare + 0x10, 0]]);
    }

    #[test]
    fn color_replace_keys_uses_plus_1c() {
        let mut e = engine();
        let interpolator = e.mem.alloc(0x24);
        e.mem.set_u32(interpolator + 0x20, 9);
        let (_, log) = run(
            &mut e,
            0x0052_cff0,
            &args![interpolator, 0x500u32, 2u32, 1u32],
        );
        let data = e.mem.u32(interpolator + 0x1c);
        assert_eq!(e.mem.u32(data), COLOR_DATA_VTABLE);
        assert_eq!(
            calls(&log, COLOR_DATA_REPLACE_ANIM),
            vec![vec![data, 0x500, 2, 1]]
        );
        assert_eq!(e.mem.u32(interpolator + 0x20), 0);
        let (_, log) = run(&mut e, 0x0052_cff0, &args![interpolator, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls(&log, COLOR_DATA_REPLACE_ANIM),
            vec![vec![data, 0, 0, 0]]
        );
    }

    #[test]
    fn data_constructors_and_destructors() {
        let mut e = engine();
        e.register(FLOAT_DATA_DESTRUCT, no_op);
        e.register(COLOR_DATA_DESTRUCT, no_op);
        e.register(NI_DELETE, no_op);
        let block = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(block + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0052_cf30, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), FLOAT_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(block + 0x14), 0xffff_ff00);
        assert_eq!(e.call(0x0052_d0e0, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), COLOR_DATA_VTABLE);
        for (addr, body) in [
            (0x0052_cf90u32, FLOAT_DATA_DESTRUCT),
            (0x0052_d130, COLOR_DATA_DESTRUCT),
        ] {
            for (flags, frees) in [(0u32, false), (1, true)] {
                let (r, log) = run(&mut e, addr, &args![block, flags]);
                assert_eq!(r.u32(), block);
                assert_eq!(calls(&log, body), vec![vec![block]]);
                assert_eq!(
                    calls(&log, NI_DELETE),
                    if frees {
                        vec![vec![block, 0x18]]
                    } else {
                        vec![]
                    }
                );
            }
        }
    }

    #[test]
    fn color_key_value_and_constructors() {
        let mut e = engine();
        let key = e.mem.alloc(0x14);
        let color = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(color + 4 * i, 0x10 + i);
        }
        e.call(0x0052_cfc0, &args![key, color]);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(key + 4 * i)).collect();
        assert_eq!(words, vec![0, 0x10, 0x11, 0x12, 0x13]);
        e.register(ALLOCATOR_CONSTRUCT, no_op);
        let (r, log) = run(&mut e, 0x0052_d160, &args![key]);
        assert_eq!(r.u32(), key);
        assert_eq!(calls(&log, ALLOCATOR_CONSTRUCT), vec![vec![key]]);
        assert_eq!(
            calls(&log, COLOR_CONSTRUCT),
            vec![vec![key + 4, 0, 0, 0, 0]]
        );
        e.register(0x0062_40d0, no_op);
        let (r, log) = run(&mut e, 0x0052_d200, &args![key]);
        assert_eq!(r.u32(), key);
        assert_eq!(calls(&log, 0x0062_40d0), vec![vec![key]]);
        e.register(TES_FORM_EMPTY, no_op);
        let (_, log) = run(&mut e, 0x0052_d240, &args![key]);
        assert_eq!(calls(&log, TES_FORM_EMPTY), vec![vec![key]]);
    }

    #[test]
    fn key_clear_resets_every_interpolator() {
        let mut e = engine();
        e.register(INTERPOLATOR_CLEAR, no_op);
        e.register(TES_FORM_EMPTY, no_op);
        let this = modifier(&mut e);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0x1c, 6);
        let (_, log) = run(&mut e, 0x0052_d260, &args![this]);
        let cleared = calls(&log, INTERPOLATOR_CLEAR);
        assert_eq!(cleared.len(), 0x2a + 11);
        assert_eq!(cleared[0], vec![this + 0x114, 0]);
        assert_eq!(cleared[1], vec![this + 0x114 + 0x18, 0]);
        assert_eq!(cleared[2], vec![this + 0x114 + 0x30, 0]);
        assert_eq!(cleared[0x2a], vec![this + 0x504, 0]);
        assert_eq!(cleared[0x2a + 10], vec![this + 0x63c, 0]);
        assert_eq!(
            calls(&log, POINTER_ASSIGN),
            vec![vec![this + 0x534 + 0x1c, 0], vec![this + 0x558 + 0x1c, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        let (_, log) = run(&mut e, 0x0052_d3b0, &args![this, 0x77u32]);
        assert_eq!(calls(&log, POINTER_ASSIGN), vec![vec![this + 0x1c, 0x77]]);
    }

    fn init_item_engine(found: bool) -> (Engine, u32) {
        let mut e = engine();
        e.register(TES_FORM_FIRST_TEST, |_, _| false.into_ret());
        e.register(TES_FORM_GET_FILE, |_, _| 0x900u32.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]) | 0x0100_0000;
            e.mem.set_u32(a[0], id);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x1000).into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if found { a[0] } else { 0 }).into_ret()
        });
        e.register(FORM_ID, |_, _| 0x00ab_cdefu32.into_ret());
        e.register(LOG, no_op);
        e.register(TES_FORM_SET_FLAG, no_op);
        let this = modifier(&mut e);
        let vtable = e.mem.alloc(0x134);
        e.mem.set_u32(vtable + 0x130, 0x0f60_0130);
        e.register(0x0f60_0130, |_, _| 0x0f61_0000u32.into_ret());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x1c, 0x11);
        e.mem.set_u32(this + 0x18, 0x22);
        (e, this)
    }

    #[test]
    fn init_item_resolves_the_sounds() {
        let (mut e, this) = init_item_engine(true);
        let (_, log) = run(&mut e, 0x0052_d3d0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0100_1011);
        assert_eq!(e.mem.u32(this + 0x18), 0x0100_1022);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST)[0],
            vec![0x0100_1011, 0, TES_FORM_TYPE, TES_SOUND_TYPE, 0]
        );
        assert_eq!(calls(&log, TES_FORM_GET_FILE)[0], vec![this, 0xffff_ffff]);
        assert!(calls(&log, LOG).is_empty());
        assert_eq!(calls(&log, TES_FORM_SET_FLAG), vec![vec![this, 1]]);
    }

    #[test]
    fn init_item_logs_missing_sounds_and_skips_unset_ones() {
        let (mut e, this) = init_item_engine(false);
        e.mem.set_u32(this + 0x18, 0);
        let (_, log) = run(&mut e, 0x0052_d3d0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(
            calls(&log, LOG),
            vec![vec![
                MISSING_INTRO_SOUND,
                0x0100_0011,
                0x0f61_0000,
                0x00ab_cdef
            ]]
        );
        assert_eq!(calls(&log, LOOKUP_FORM).len(), 1);
        // Answered true by 004013e0: nothing at all.
        e.register(TES_FORM_FIRST_TEST, |_, _| true.into_ret());
        let (_, log) = run(&mut e, 0x0052_d3d0, &args![this]);
        assert_eq!(log.len(), 2);
    }

    /// A file whose chunks come from `chunks` in turn; `swaps` is what
    /// `00401680` answers. The chunk data double fills the buffer with the
    /// byte 0x11 and the word reader answers 0x5555.
    fn load_engine(chunks: Vec<u32>, swaps: bool) -> (Engine, u32, Rc<RefCell<Log>>) {
        let mut e = engine();
        e.register(FILE_FORM_TYPE, |_, _| 0x54u32.into_ret());
        e.register(TES_FORM_LOAD_FORM, no_op);
        e.register(TES_FORM_SET_FLAG, no_op);
        let mut queue = chunks.into_iter();
        e.register_double(FILE_CHUNK, move |_, _| queue.next().unwrap_or(0).into_ret());
        e.register(FILE_NEXT_CHUNK, |_, _| true.into_ret());
        e.register(FILE_CHUNK_SIZE, |_, _| 0x10u32.into_ret());
        e.register(FILE_CHUNK_DATA, |e, a| {
            // DNAM (0xf4 bytes) marks only its first word, keeping the key
            // counts the test set; key reads are filled with 0x11.
            if a[2] == 0xf4 {
                e.mem.set_u32(a[1], 0x1111_1111);
            } else if a[2] < 0xf4 {
                e.mem.write(a[1], &vec![0x11; a[2] as usize]);
            }
            Ret::default()
        });
        e.register(FILE_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x5555);
            Ret::default()
        });
        e.register_double(FILE_SWAPS, move |_, _| swaps.into_ret());
        e.register(SWAP_WORD, no_op);
        e.register(DATA_ENDIAN, no_op);
        let this = modifier(&mut e);
        let vtable = e.mem.alloc(0x138);
        e.mem.set_u32(vtable + 0xe0, 0x0f62_00e0);
        e.mem.set_u32(vtable + 0x134, 0x0f62_0134);
        let edids = Rc::new(RefCell::new(Vec::new()));
        let sink = edids.clone();
        e.register_double(0x0f62_0134, move |e, a| {
            sink.borrow_mut().push((a[1], vec![e.mem.u32(a[1])]));
            Ret::default()
        });
        e.register(0x0f62_00e0, no_op);
        e.mem.set_u32(this, vtable);
        (e, this, edids)
    }

    #[test]
    fn load_refuses_another_form_type() {
        let (mut e, this, _) = load_engine(vec![], false);
        e.register(FILE_FORM_TYPE, |_, _| 0x53u32.into_ret());
        let (r, log) = run(&mut e, 0x0052_d510, &args![this, 0x900u32]);
        assert!(!r.bool());
        assert!(calls(&log, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn load_reads_every_kind_of_chunk() {
        let pair_tag = 0x4441_4900 | 0x40 | 5; // pair 5, side 1
        let chunks = vec![
            tag(b"DNAM"),
            tag(b"BNAM"),
            tag(b"TNAM"),
            tag(b"RDSD"),
            tag(b"RDSI"),
            tag(b"EDID"),
            tag(b"OBND"),
            pair_tag,
            tag(b"ZNAM"),
        ];
        let (mut e, this, edids) = load_engine(chunks, true);
        e.mem.set_u32(this + 0xd4, 3);
        e.mem.set_u32(this + 0xd0, 2);
        e.mem.set_u32(this + PAIR_COUNTS + 5 * 8 + 4, 1);
        let (r, log) = run(&mut e, 0x0052_d510, &args![this, 0x900u32]);
        assert!(r.bool());
        assert_eq!(calls(&log, TES_FORM_LOAD_FORM), vec![vec![this, 0x900]]);
        assert_eq!(calls(&log, TES_FORM_SET_FLAG), vec![vec![this, 0]]);
        // DNAM: the data and its byte swap.
        assert_eq!(e.mem.u32(this + 0x20), 0x1111_1111);
        assert_eq!(calls(&log, DATA_ENDIAN), vec![vec![this + 0x20]]);
        // BNAM: three float keys, swapped word by word, to +0x504.
        let keys = e.mem.u32(this + 0x6fc);
        assert_eq!(e.mem.u32(keys - 4), 3);
        let reads = calls(&log, FILE_CHUNK_DATA);
        assert!(reads.contains(&vec![0x900, keys, 24]));
        let swaps = calls(&log, SWAP_WORD);
        assert_eq!(
            swaps[..6].iter().map(|w| w[0]).collect::<Vec<_>>(),
            (0..6).map(|i| keys + 4 * i).collect::<Vec<_>>()
        );
        let data = e.mem.u32(this + 0x504 + 0x10);
        let floats = calls(&log, FLOAT_DATA_REPLACE_ANIM);
        assert_eq!(floats[0], vec![data, keys, 3, 1]);
        // TNAM: two colour keys, ten words swapped, to +0x534.
        let keys = e.mem.u32(this + 0x704);
        assert!(reads.contains(&vec![0x900, keys, 0x28]));
        assert_eq!(swaps.len(), 6 + 10 + 2);
        let data = e.mem.u32(this + 0x534 + 0x1c);
        assert_eq!(
            calls(&log, COLOR_DATA_REPLACE_ANIM),
            vec![vec![data, keys, 2, 1]]
        );
        // RDSD and RDSI: the sound ids.
        assert_eq!(e.mem.u32(this + 0x1c), 0x5555);
        assert_eq!(e.mem.u32(this + 0x18), 0x5555);
        // EDID: 0x200 bytes asked for, handed to slot 0x134.
        let edid = edids.borrow();
        assert_eq!(edid.len(), 1);
        assert!(reads.contains(&vec![0x900, edid[0].0, 0x200]));
        // OBND: slot 0xe0 with the file.
        assert_eq!(calls(&log, 0x0f62_00e0), vec![vec![this, 0x900]]);
        // The pair chunk: one key to the pair's second interpolator.
        let keys = e.mem.u32(this + PAIR_KEYS + 5 * 8 + 4);
        let data = e.mem.u32(this + PAIRS + 5 * 0x30 + 0x18 + 0x10);
        assert_eq!(floats[1], vec![data, keys, 1, 1]);
        assert_eq!(floats.len(), 2);
        // Ten chunks asked for (the last answers 0).
        assert_eq!(calls(&log, FILE_CHUNK).len(), 10);
    }

    #[test]
    fn load_stops_when_there_is_no_next_chunk() {
        let (mut e, this, _) = load_engine(vec![tag(b"ZNAM"), tag(b"ZNAM")], false);
        e.register(FILE_NEXT_CHUNK, |_, _| false.into_ret());
        let (r, log) = run(&mut e, 0x0052_d510, &args![this, 0x900u32]);
        assert!(r.bool());
        assert_eq!(calls(&log, FILE_CHUNK).len(), 1);
        // DNAM without a byte swap: no Endian.
        let (mut e, this, _) = load_engine(vec![tag(b"DNAM")], false);
        let (_, log) = run(&mut e, 0x0052_d510, &args![this, 0x900u32]);
        assert!(calls(&log, DATA_ENDIAN).is_empty());
    }

    #[test]
    fn is_expired_compares_the_duration_with_the_elapsed_time() {
        let mut e = engine();
        let this = modifier(&mut e);
        e.mem.set_f32(CLOCK, 10.0);
        e.mem.set_f32(this + 0x24, 3.0);
        let expired = |e: &mut Engine, start: f32| e.call(0x0053_16c0, &args![this, start]).bool();
        // Not active: never expired.
        assert!(!expired(&mut e, 1.0));
        e.mem.set_u8(this + 0x20, 1);
        assert!(expired(&mut e, 6.0));
        assert!(!expired(&mut e, 7.0));
        assert!(!expired(&mut e, 8.0));
        // A start below -10000 is never expired; -10000 itself is.
        assert!(!expired(&mut e, -10001.0));
        assert!(expired(&mut e, -10000.0));
        // A NaN start passes the first test and fails the second.
        assert!(!expired(&mut e, f32::NAN));
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const MODIFIER: u32 = 0x7600_0000;
    const OTHER: u32 = 0x7700_0000;
    const STATE: u32 = 0x7300_0000;
    const SLOT_COUNT: u32 = 0x7100_00e0;
    const SLOT_KEYS: u32 = 0x7100_00ec;
    const SLOT_18: u32 = 0x7100_0018;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Every interpolator of a modifier at `at`.
    fn interpolators(at: u32) -> Vec<u32> {
        let mut all = vec![];
        for pair in 0..0x15 {
            for side in 0..2 {
                all.push(at + pair_track(pair, side).2);
            }
        }
        all.extend(KEYED_TRACKS.iter().map(|t| at + t.2));
        all
    }

    /// Two modifiers whose interpolators answer 2 keys (slot +0xE0) at
    /// `0x7900_0000 + interpolator` (slot +0xEC).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.map(MODIFIER, 0x800);
        e.map(OTHER, 0x800);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x18, SLOT_18);
        e.mem.set_u32(vtable + 0xe0, SLOT_COUNT);
        e.mem.set_u32(vtable + 0xec, SLOT_KEYS);
        for at in [MODIFIER, OTHER] {
            e.mem.set_u32(at, vtable);
            for interpolator in interpolators(at) {
                e.mem.set_u32(interpolator, vtable);
            }
        }
        e.register(SLOT_COUNT, |_, _| 2u32.into_ret());
        e.register(SLOT_KEYS, |_, a| (0x7900_0000 + (a[0] & 0xffff)).into_ret());
        e.register(SLOT_18, |_, _| Ret::default());
        e
    }

    #[test]
    fn rtti_records() {
        let mut e = engine();
        assert_eq!(e.call(0x0052_cf80, &args![0x1234u32]).u32(), 0x011f_3748);
        assert_eq!(e.call(0x0052_d120, &args![0x1234u32]).u32(), 0x011f_3750);
    }

    #[test]
    fn endian_swaps_the_data_words() {
        let mut e = engine();
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0053_1490, &args![0x1000u32]);
        let swaps: Vec<u32> = calls(&e, SWAP_WORD).iter().map(|c| c[0]).collect();
        assert_eq!(swaps.len(), 1 + 0x2a + 15);
        assert_eq!(&swaps[..3], &[0x1004, 0x1008, 0x100c]);
        assert_eq!(swaps[0x2a], 0x10ac);
        assert_eq!(&swaps[0x2b..0x2e], &[0x10b4, 0x10b8, 0x10b0]);
        assert_eq!(*swaps.last().unwrap(), 0x10f0);
    }

    #[test]
    fn flush_takes_counts_and_keys_back() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0052_ed90, &args![MODIFIER]);
        assert_eq!(calls(&e, SLOT_COUNT).len(), 0x2a + 13);
        assert_eq!(calls(&e, SLOT_COUNT)[0], vec![MODIFIER + 0x114, 0]);
        assert_eq!(calls(&e, SLOT_KEYS)[1], vec![MODIFIER + 0x12c, 0]);
        assert_eq!(e.mem.u32(MODIFIER + 0x28), 2);
        assert_eq!(e.mem.u32(MODIFIER + 0x654), 0x7900_0114);
        assert_eq!(e.mem.u32(MODIFIER + 0x658), 0x7900_012c);
        assert_eq!(e.mem.u32(MODIFIER + 0xd0), 2);
        assert_eq!(e.mem.u32(MODIFIER + 0x704), 0x7900_0534);
        assert_eq!(e.mem.u32(MODIFIER + 0x110), 2);
        assert_eq!(e.mem.u32(MODIFIER + 0x72c), 0x7900_063c);
    }

    #[test]
    fn save_adds_every_chunk() {
        let mut e = engine();
        for addr in [
            FORM_START,
            FORM_CLOSE,
            FORM_ADD_CHUNK_DATA,
            FORM_ADD_CHUNK_WORD,
            OPERATOR_DELETE,
            COPY_BYTES,
            SWAP_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |_, a| (0x7a00_0000 + a[0]).into_ret());
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(STATE).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.mem.set_u32(MODIFIER + 0x1c, 0x5000);
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0052_f250, &args![MODIFIER]);
        let chunks = calls(&e, FORM_ADD_CHUNK_DATA);
        assert_eq!(chunks.len(), 1 + 13 + 0x2a);
        assert_eq!(chunks[0], vec![0x4d41_4e44, MODIFIER + 0x20, 0xf4]);
        // BNAM: two 8-byte keys; TNAM: two 0x14-byte keys.
        assert_eq!(chunks[1], vec![0x4d41_4e42, 0x7a00_0010, 0x10]);
        assert_eq!(chunks[3], vec![0x4d41_4e54, 0x7a00_0028, 0x28]);
        assert_eq!(chunks[13][0], 0x344d_414e);
        assert_eq!(chunks[14][0], 0x4441_4900);
        assert_eq!(chunks[15][0], 0x4441_4940);
        assert_eq!(chunks[55][0], 0x4441_4954);
        assert_eq!(
            calls(&e, COPY_BYTES)[0],
            vec![0x7a00_0010, 0x7900_0504, 0x10]
        );
        // DNAM swapped twice (58 words each), each track's words once.
        let words = 2 * 58 + 11 * 4 + 2 * 10 + 0x2a * 4;
        assert_eq!(calls(&e, SWAP_WORD).len(), words);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 13 + 0x2a);
        assert_eq!(
            calls(&e, FORM_ADD_CHUNK_WORD),
            vec![vec![0x4453_4452, 0x5001]]
        );
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![MODIFIER]]);
        // No swapping, both sounds.
        e.mem.set_u32(STATE, 0);
        e.mem.set_u32(MODIFIER + 0x18, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0052_f250, &args![MODIFIER]);
        assert!(calls(&e, SWAP_WORD).is_empty());
        assert_eq!(
            calls(&e, FORM_ADD_CHUNK_WORD),
            vec![vec![0x4453_4452, 0x5001], vec![0x4953_4452, 0x6001]]
        );
    }

    fn copy_engine() -> Engine {
        let mut e = engine();
        e.register(FORM_COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(POINTER_GET, |_, _| 0x5555u32.into_ret());
        e.register(FLOAT_DATA_REPLACE_ANIM, |_, _| Ret::default());
        e.register(COLOR_DATA_REPLACE_ANIM, |_, _| Ret::default());
        e.register(COPY_BYTES, |e, a| {
            if a[2] == 0xf4 {
                for offset in 0..0xf4 {
                    let byte = e.mem.u8(a[1] + offset);
                    e.mem.set_u8(a[0] + offset, byte);
                }
            }
            Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn copy_takes_data_sounds_and_new_keys() {
        let mut e = copy_engine();
        e.mem.set_u32(OTHER + 0x1c, 0x5000);
        e.mem.set_u32(OTHER + 0x18, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0053_0160, &args![MODIFIER, OTHER]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                OTHER,
                0,
                TES_FORM_TYPE,
                TES_IMAGE_SPACE_MODIFIER_TYPE,
                0
            ]]
        );
        // The source is flushed, not this one.
        assert!(calls(&e, SLOT_COUNT).iter().all(|c| c[0] >= OTHER));
        assert_eq!(calls(&e, SLOT_18), vec![vec![MODIFIER]]);
        assert_eq!(e.mem.u32(MODIFIER + 0x1c), 0x5000);
        assert_eq!(e.mem.u32(MODIFIER + 0x18), 0x6000);
        assert_eq!(e.mem.u32(MODIFIER + 0xd4), 2);
        let copies = calls(&e, COPY_BYTES);
        assert_eq!(copies.len(), 1 + 13 + 0x2a);
        assert_eq!(copies[1][1..], [0x7900_0504, 0x10]);
        assert_eq!(copies[3][1..], [0x7900_0534, 0x28]);
        let keys = e.mem.u32(MODIFIER + 0x6fc);
        assert_eq!(copies[1][0], keys);
        assert_eq!(e.mem.u32(keys - 4), 2);
        let floats = calls(&e, FLOAT_DATA_REPLACE_ANIM);
        assert_eq!(floats.len(), 11 + 0x2a);
        assert_eq!(floats[0], vec![0x5555, keys, 2, 1]);
        assert_eq!(calls(&e, COLOR_DATA_REPLACE_ANIM).len(), 2);
        let constructs = calls(&e, VECTOR_CONSTRUCT);
        assert_eq!(
            constructs[2][1..],
            [0x14, 2, COLOR_KEY_CONSTRUCT, KEY_DESTRUCT]
        );
        // Not a modifier: nothing.
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0053_0160, &args![MODIFIER, OTHER]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn compare_stops_at_the_first_difference() {
        let mut e = engine();
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE + 4).into_ret()
        });
        // memcmp answers 1 on the call whose number is the state word.
        e.mem.set_u32(STATE, 0xffff);
        e.register_double(MEMCMP, |e, _| {
            let n = e.mem.u32(STATE + 8);
            e.mem.set_u32(STATE + 8, n + 1);
            u32::from(n == e.mem.u32(STATE)).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
        let compares = calls(&e, MEMCMP);
        assert_eq!(compares.len(), 1 + 13 + 0x2a);
        assert_eq!(compares[0], vec![MODIFIER + 0x20, OTHER + 0x20, 0xf4]);
        assert_eq!(compares[3], vec![0x7900_0534, 0x7900_0534, 0x28]);
        assert_eq!(compares[14], vec![0x7900_0114, 0x7900_0114, 0x10]);
        assert_eq!(calls(&e, SLOT_COUNT).len(), 2 * (0x2a + 13));
        for stop in [0, 5, 20] {
            e.mem.set_u32(STATE, stop);
            e.mem.set_u32(STATE + 8, 0);
            e.call_log = Some(vec![]);
            assert!(e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
            assert_eq!(calls(&e, MEMCMP).len(), stop as usize + 1);
        }
        e.mem.set_u32(STATE, 0xffff);
        e.mem.set_u32(STATE + 4, 1);
        assert!(e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
        e.mem.set_u32(STATE + 4, 0);
        e.mem.set_u32(OTHER + 0x18, 9);
        assert!(e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
        e.mem.set_u32(OTHER + 0x18, 0);
        e.mem.set_u32(OTHER + 0x1c, 9);
        assert!(e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0053_10b0, &args![MODIFIER, OTHER]).bool());
    }
}

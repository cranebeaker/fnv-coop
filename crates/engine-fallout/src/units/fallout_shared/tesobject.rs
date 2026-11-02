//! `fallout shared/tesobject.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the `TESObject` base class and its bound-box derived
//! classes (the vtables `0102960c`, `0102980c`, `01029994` and `01029b1c` are
//! `TESBoundObject`, `TESBoundAnimObject`, `TESBoundTreeObject` and
//! `TESObject`, read from their RTTI locators), the model cloning code
//! `Clone3D`, and a debug-heap `std::basic_string<char>` instance
//! (`0044a4c0`..`0044a7f0`) that the linker placed with them.
//!
//! Layouts, constants and helpers are at the top; the functions follow in
//! address order. The last function of the unit, `0050fdf0`
//! (`TESObject::GetScaledModelFileName`), is translated; the functions still
//! marked `Translation incomplete` are compiler-only partials (exception
//! frames, security cookie).
//!
//! Float notes: the game copies floats through `FLD` / `FSTP`, which turns a
//! signalling NaN into a quiet one ([`x87_quiet_bits`]), and compares them
//! with `FCOM` / `FUCOMPP` and `TEST AH,0x44`, where only "ordered and equal"
//! leaves the parity flag clear: every comparison here is an `==` or `!=` on
//! `f64` values, which gives the same answer for NaN.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiNPShortPoint3` (Xbox PDB): three signed 16-bit components.
    pub struct NiNPShortPoint3: 0x06 {
        0x00 x: i16,
        0x02 y: i16,
        0x04 z: i16,
    }

    /// `TESBoundObject::BOUND_DATA` (Xbox PDB): the `OBND` box, two points.
    pub struct BoundData: 0x0C {
        /// `BoundMin` (Xbox PDB).
        0x00 BoundMin: Inline<NiNPShortPoint3>,
        /// `BoundMax` (Xbox PDB).
        0x06 BoundMax: Inline<NiNPShortPoint3>,
    }

    /// `std::basic_string<char, char_traits<char>, _DebugHeapAllocator<char>>`
    /// (Visual Studio 2008 debug library): the allocator proxy at +0, the
    /// storage union at +4 (a 16-byte buffer, or a heap pointer once the
    /// reserve reaches 16), the length and the reserve.
    pub struct DebugString: 0x1C {
        /// The heap pointer when `reserve >= 0x10`; otherwise the first
        /// word of the inline 16-byte buffer.
        0x04 buffer_pointer: Ptr,
        /// `_Mysize`.
        0x14 size: u32,
        /// `_Myres`: the capacity, 0xF while the inline buffer is used.
        0x18 reserve: u32,
    }

    /// `TESObject` (Xbox PDB) on PC: the 0x18 byte `TESForm` base, then the
    /// intrusive list node.
    pub struct TESObject: 0x24 {
        /// `m_pParentList` (Xbox PDB, +0x28 there): `TESObjectList*`.
        0x18 m_pParentList: Ptr,
        /// `nod_lpPrev` (Xbox PDB).
        0x1C nod_lpPrev: Ptr,
        /// `nod_lpNext` (Xbox PDB).
        0x20 nod_lpNext: Ptr,
    }

    /// `TESBoundObject` (Xbox PDB) on PC: `TESObject` plus the `OBND` box
    /// (`BoundData`, +0x34 on the Xbox).
    pub struct TESBoundObject: 0x30 {
        0x24 BoundData: Inline<BoundData>,
    }
}

/// Vtable of `TESBoundObject` (RTTI `.?AVTESBoundObject@@`).
const TES_BOUND_OBJECT_VTABLE: u32 = 0x0102_960c;
/// Vtable of `TESBoundAnimObject` (RTTI `.?AVTESBoundAnimObject@@`).
const TES_BOUND_ANIM_OBJECT_VTABLE: u32 = 0x0102_980c;
/// Vtable of `TESBoundTreeObject` (RTTI `.?AVTESBoundTreeObject@@`).
const TES_BOUND_TREE_OBJECT_VTABLE: u32 = 0x0102_9994;
/// Vtable of `TESObject` (RTTI `.?AVTESObject@@`).
const TES_OBJECT_VTABLE: u32 = 0x0102_9b1c;

/// The function-local default `BoundData`'s guard word (bit 0 set once it
/// is constructed) and its 6 bytes, as used by `0050e8f0`.
const DEFAULT_BOUND_GUARD: u32 = 0x011c_912c;
const DEFAULT_BOUND: u32 = 0x011c_9124;

/// `std::string::npos`.
const STRING_NPOS: u32 = 0x0101_73f0;
/// Reads the word at +0x14 of its `ECX` (`_Mysize`).
const STRING_SIZE_GETTER: u32 = 0x0082_5c00;
/// `_Xran` (throws `out_of_range`; never returns in the game).
const STRING_OUT_OF_RANGE: u32 = 0x00ec_30e8;
/// `basic_string::_Eos(newsize)`: stores the size and the terminator.
const STRING_SET_LENGTH: u32 = 0x0044_a880;
/// `basic_string::_Grow(newsize, trim)`.
const STRING_GROW: u32 = 0x0044_a8d0;
/// `basic_string::_Myptr()`.
const STRING_DATA: u32 = 0x0044_a970;
/// `basic_string::_Inside(ptr)`.
const STRING_INSIDE: u32 = 0x0044_ab90;
/// `_Traits_helper::copy_s(dest, destsize, src, count)`.
const COPY_CHARS: u32 = 0x0044_ccd0;
/// `_Traits_helper::move_s(dest, destsize, src, count)`.
const MOVE_CHARS: u32 = 0x0044_d580;
/// The allocator's `Deallocate(ptr, size)`, called with the string as
/// `ECX`.
const STRING_FREE: u32 = 0x0064_df10;
/// The empty `_DebugHeapAllocator` constructor and the proxy constructor.
const ALLOCATOR_CONSTRUCT: u32 = 0x0068_15c0;
const ALLOCATOR_PROXY_CONSTRUCT: u32 = 0x0065_de30;
/// `strlen`.
const STRLEN: u32 = 0x00ec_6130;
/// `operator delete(ptr)` (the deleting destructors' last step).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The form-type getter (the byte at +4 of a form).
const FORM_TYPE: u32 = 0x0040_1170;

/// `TESForm::~TESForm` and `TESForm::TESForm` (engine map names).
const TES_FORM_DESTRUCT: u32 = 0x0048_3630;
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
/// Removes an object from a list; called with the list as `ECX`.
const LIST_REMOVE: u32 = 0x0051_0130;

/// `NiPoint3(x, y, z)` constructor.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// Subtraction / addition of two `NiPoint3` into a result.
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_ADD: u32 = 0x0043_9e90;
/// `sqrtf` wrapper (`float` argument on the stack, result in `ST0`).
const SQUARE_ROOT: u32 = 0x0045_79e0;
/// `memcpy`.
const MEMORY_COPY: u32 = 0x0040_1460;
/// Returns the global byte that says whether data is byte-swapped.
const BYTE_SWAP_FLAG_GETTER: u32 = 0x0040_1500;
/// Swaps the bytes of the 16-bit word at its first argument in place.
const BYTE_SWAP_WORD: u32 = 0x0040_7a90;
/// Chunk writer (cdecl: type, data, size), current chunk type, chunk reader
/// and flag getter of the save/load file object (engine map names
/// `TESForm::__AddChunkData`, `TESFile::GetTESChunk`,
/// `TESFile::GetChunkData_ov3`; unconfirmed).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const FILE_CURRENT_CHUNK: u32 = 0x0047_26b0;
const FILE_READ_CHUNK: u32 = 0x0047_2890;
const FILE_FLAG_GETTER: u32 = 0x0040_1680;
/// `'OBND'` as a little-endian chunk type.
const OBND_CHUNK: u32 = 0x444e_424f;

/// Returns the model path of a form (cdecl, engine map name
/// `TESModel::GetModel`; unconfirmed).
const FORM_MODEL_PATH: u32 = 0x0048_95b0;
/// Engine map name `TESModel::GetModelLength`; unconfirmed.
const FORM_MODEL_LENGTH: u32 = 0x0048_9540;
/// Name comparison (0 when equal).
const COMPARE_NAMES: u32 = 0x0040_4dc0;
/// A second type code of a form, used by `IsMarker` (`0084e3a0`).
const FORM_SUBTYPE: u32 = 0x0084_e3a0;
/// `"EditorMarker.NIF"`.
const EDITOR_MARKER_MODEL: u32 = 0x0102_9c94;

/// The global form pointer `IsMarker` compares a type 0x1c form with.
const MARKER_FORM_TYPE_1C: u32 = 0x011c_a258;
/// Two of the fourteen type 0x20 forms; for these `Clone3D` and `0050fbf0`
/// ignore the reference.
const SPECIAL_FORM_A: u32 = 0x011c_a220;
const SPECIAL_FORM_B: u32 = 0x011c_a250;
/// The fourteen globals `IsMarker` compares a type 0x20 form with.
const MARKER_FORMS_TYPE_20: [u32; 14] = [
    0x011c_a220,
    0x011c_a250,
    0x011c_a25c,
    0x011c_a260,
    0x011c_a254,
    0x011c_a224,
    0x011c_a26c,
    0x011c_a244,
    0x011c_a248,
    0x011c_a24c,
    0x011c_a230,
    0x011c_a264,
    0x011c_a228,
    0x011c_a22c,
];
/// The three globals `0050fb90` compares a type 0x20 form with.
const NAMED_FORMS_TYPE_20: [u32; 3] = [0x011c_a238, 0x011c_a234, 0x011c_a23c];

/// The object whose methods register, find and release models by file name
/// (the engine map names `004472a0` `ModelLoader::FindModel`; unconfirmed).
const MODEL_LOADER_POINTER: u32 = 0x011c_3b3c;
/// Two counters `0050fbf0` and `Clone3D` adjust.
const COUNTER_A: u32 = 0x011c_90a8;
const COUNTER_B: u32 = 0x011c_90ac;
/// `Clone3D`'s helper object (`457ba0`'s `ECX`) and the addresses it passes.
const CLONE_HELPER_POINTER: u32 = 0x011d_ea10;
const CLONE_ADDRESS_ARGUMENT: u32 = 0x011f_36ac;
const CLONE_NAME_TABLE: u32 = 0x011a_9448;
const CLONE_DATA_TABLE: u32 = 0x011f_426c;
/// `1.0` and `0.0` as `double`s.
const ONE: u32 = 0x0101_2070;
const ZERO: u32 = 0x0101_2060;
/// `0.5` as a `double`: the bound's diagonal is halved.
const HALF: u32 = 0x0101_1588;
/// `"MODELS: %s has a scale of 0.  Change it in the editor."` and
/// `"Invalid scale found on ModelLoader copy of file '%s'"`.
const SCALE_ZERO_MESSAGE: u32 = 0x0102_97d0;
const SCALE_INVALID_MESSAGE: u32 = 0x0102_9798;
/// The message printer (cdecl, printf style).
const PRINT_MESSAGE: u32 = 0x005b_5e40;

/// A `float` argument pushed with `FLD` / `FSTP [ESP]`.
fn pushed_float(value: f32) -> u32 {
    x87_quiet_bits(value.to_bits())
}

/// Copies three words from `from` to `to` with integer moves.
fn copy_three_words(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

// Translated from 0044a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_string<char,...,_DebugHeapAllocator<char>>::basic_string(const char*)`
/// (Xbox PDB name `basic_string::basic_string`): builds the empty allocator
/// argument on the stack (the slot holds the caller's `ECX`, which is
/// `this`), constructs the proxy, empties the string and assigns `text`.
pub fn std_string_construct_c_str(
    e: &mut Engine,
    this: Ptr<DebugString>,
    text: Ptr,
) -> Ptr<DebugString> {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, this.addr());
    e.call(ALLOCATOR_CONSTRUCT, &args![slot]);
    let allocator = e.mem.u32(slot);
    e.mem.free(slot);
    fn_0044a7b0(e, this, allocator);
    std_string_tidy(e, this, false, 0);
    std_string_assign_c_str(e, this, text);
    this
}

// Translated from 0044a500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor of the debug-heap `std::string`: the argument is
/// the allocator reference and the source string at once; empties the
/// string and assigns the whole of `right`.
pub fn std_string_copy_construct(
    e: &mut Engine,
    this: Ptr<DebugString>,
    right: Ptr<DebugString>,
) -> Ptr<DebugString> {
    fn_0044a7d0(e, this, right.cast());
    std_string_tidy(e, this, false, 0);
    let npos = e.global::<u32>(STRING_NPOS);
    std_string_assign_substring(e, this, right, 0, npos);
    this
}

// Translated from 0044a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the debug-heap `std::string`: `_Tidy(true, 0)`.
pub fn std_string_destruct(e: &mut Engine, this: Ptr<DebugString>) {
    std_string_tidy(e, this, true, 0);
}

// Translated from 0044a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `c_str` of the debug-heap `std::string`: `_Myptr()`.
pub fn std_string_c_str(e: &mut Engine, this: Ptr<DebugString>) -> Ptr {
    e.call(STRING_DATA, &args![this]).ptr()
}

// Translated from 0044a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::assign(const basic_string& right, size_type roff,
/// size_type count)` of the debug-heap `std::string`: throws (`_Xran`) when
/// `offset` is past the end of `right`, clamps `count` to what is left, then
/// either erases around the range (when `right` is `this`) or grows the
/// string and copies the range in.
pub fn std_string_assign_substring(
    e: &mut Engine,
    this: Ptr<DebugString>,
    right: Ptr<DebugString>,
    offset: u32,
    count: u32,
) -> Ptr<DebugString> {
    let right_size = e.call(STRING_SIZE_GETTER, &args![right]).u32();
    if right_size < offset {
        e.call(STRING_OUT_OF_RANGE, &[]);
    }
    let right_size = e.call(STRING_SIZE_GETTER, &args![right]).u32();
    let mut length = right_size.wrapping_sub(offset);
    if count < length {
        length = count;
    }
    if this == right {
        let npos = e.global::<u32>(STRING_NPOS);
        std_string_erase(e, this, offset.wrapping_add(length), npos);
        std_string_erase(e, this, 0, offset);
    } else if e.call(STRING_GROW, &args![this, length, 0u32]).bool() {
        let source = e
            .call(STRING_DATA, &args![right])
            .u32()
            .wrapping_add(offset);
        let reserve = e.get(this, DebugString::reserve);
        let destination = e.call(STRING_DATA, &args![this]).u32();
        e.call(COPY_CHARS, &args![destination, reserve, source, length]);
        e.call(STRING_SET_LENGTH, &args![this, length]);
    }
    this
}

// Translated from 0044a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::assign(const char*)`: assigns `text` with its `strlen`.
/// The result is what `assign(ptr, count)` returned (`this`).
pub fn std_string_assign_c_str(
    e: &mut Engine,
    this: Ptr<DebugString>,
    text: Ptr,
) -> Ptr<DebugString> {
    let length = fn_0044a670(e, text);
    std_string_assign_chars(e, this, text, length)
}

// Translated from 0044a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `strlen` wrapper (cdecl): the length of the C string at `text`.
pub fn fn_0044a670(e: &mut Engine, text: Ptr) -> u32 {
    e.call(STRLEN, &args![text]).u32()
}

// Translated from 0044a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::erase(pos, count)` of the debug-heap `std::string`:
/// throws (`_Xran`) when `pos` is past the end, clamps `count`, moves the
/// tail down and sets the new length.
pub fn std_string_erase(
    e: &mut Engine,
    this: Ptr<DebugString>,
    position: u32,
    count: u32,
) -> Ptr<DebugString> {
    let mut count = count;
    if e.get(this, DebugString::size) < position {
        e.call(STRING_OUT_OF_RANGE, &[]);
    }
    let after_position = e.get(this, DebugString::size).wrapping_sub(position);
    if after_position < count {
        count = after_position;
    }
    if count != 0 {
        let tail = e
            .get(this, DebugString::size)
            .wrapping_sub(position)
            .wrapping_sub(count);
        let source = e
            .call(STRING_DATA, &args![this])
            .u32()
            .wrapping_add(position)
            .wrapping_add(count);
        let capacity = e.get(this, DebugString::reserve).wrapping_sub(position);
        let destination = e
            .call(STRING_DATA, &args![this])
            .u32()
            .wrapping_add(position);
        // 0044a6fb: `0044d580` reads the ECX it is entered with (its first parameter);
        // `0044a970` (called last, at 0044a6f2) leaves ECX = this on both of its paths.
        e.call(
            MOVE_CHARS,
            &args![this, destination, capacity, source, tail],
        );
        let new_size = e.get(this, DebugString::size).wrapping_sub(count);
        e.call(STRING_SET_LENGTH, &args![this, new_size]);
    }
    this
}

// Translated from 0044a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Tidy(built, newsize)` of the debug-heap `std::string`:
/// when `built` and the string uses a heap buffer, copies the first
/// `newsize` characters back into the inline buffer and frees the heap
/// block; then resets the reserve to 0xF and the length to `newsize`.
pub fn std_string_tidy(e: &mut Engine, this: Ptr<DebugString>, built: bool, new_size: u32) {
    if built && e.get(this, DebugString::reserve) >= 0x10 {
        let heap = e.get(this, DebugString::buffer_pointer);
        if new_size > 0 {
            let inline_buffer = this.addr().wrapping_add(4);
            e.call(COPY_CHARS, &args![inline_buffer, 0x10u32, heap, new_size]);
        }
        let block_size = e.get(this, DebugString::reserve).wrapping_add(1);
        e.call(STRING_FREE, &args![this, heap, block_size]);
    }
    e.set(this, DebugString::reserve, 0xf);
    e.call(STRING_SET_LENGTH, &args![this, new_size]);
}

// Translated from 0044a7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator-proxy constructor of the debug-heap `std::string`, taking
/// the allocator by value: its address is passed to the proxy constructor.
pub fn fn_0044a7b0(e: &mut Engine, this: Ptr<DebugString>, allocator: u32) -> Ptr<DebugString> {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, allocator);
    e.call(ALLOCATOR_PROXY_CONSTRUCT, &args![this, slot]);
    e.mem.free(slot);
    this
}

// Translated from 0044a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator-proxy constructor of the debug-heap `std::string`, taking
/// the allocator by reference.
pub fn fn_0044a7d0(e: &mut Engine, this: Ptr<DebugString>, allocator: Ptr) -> Ptr<DebugString> {
    e.call(ALLOCATOR_PROXY_CONSTRUCT, &args![this, allocator]);
    this
}

// Translated from 0044a7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_string<...>::assign(const char* ptr, size_type count)`
/// (Xbox PDB name `basic_string::assign`): when `text` points into the
/// string itself, assigns that range of the string (`assign(*this, off,
/// count)`); otherwise grows to `count` and copies the characters in.
/// Returns `this`.
pub fn std_string_assign_chars(
    e: &mut Engine,
    this: Ptr<DebugString>,
    text: Ptr,
    count: u32,
) -> Ptr<DebugString> {
    if e.call(STRING_INSIDE, &args![this, text]).bool() {
        let data = e.call(STRING_DATA, &args![this]).u32();
        let offset = text.addr().wrapping_sub(data);
        return std_string_assign_substring(e, this, this, offset, count);
    }
    if e.call(STRING_GROW, &args![this, count, 0u32]).bool() {
        let reserve = e.get(this, DebugString::reserve);
        let destination = e.call(STRING_DATA, &args![this]).u32();
        e.call(COPY_CHARS, &args![destination, reserve, text, count]);
        e.call(STRING_SET_LENGTH, &args![this, count]);
    }
    this
}

// Translated from 0050e820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050e820 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESBoundObject::TESBoundObject` (Xbox PDB): the `TESObject` constructor,
/// the vtable, the empty `BoundData` and both bound points set to the
/// function-local default bound (`0050e8f0`). The compiler's
/// exception-unwinding frame is not translated.
pub fn tes_bound_object_construct(
    e: &mut Engine,
    this: Ptr<TESBoundObject>,
) -> Ptr<TESBoundObject> {
    tes_object_construct(e, this.cast());
    e.mem.set_u32(this.addr(), TES_BOUND_OBJECT_VTABLE);
    fn_0050e9c0(e, this.at(TESBoundObject::BoundData));
    let default = fn_0050e8f0(e);
    fn_0050e8b0(
        e,
        this.at(TESBoundObject::BoundData).at(BoundData::BoundMin),
        default,
    );
    let default = fn_0050e8f0(e);
    fn_0050e8b0(
        e,
        this.at(TESBoundObject::BoundData).at(BoundData::BoundMax),
        default,
    );
    this
}

// Translated from 0050e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three 16-bit components of a `NiNPShortPoint3` (unnamed in the
/// engine map) and returns `this`.
pub fn fn_0050e8b0(
    e: &mut Engine,
    this: Ptr<NiNPShortPoint3>,
    source: Ptr<NiNPShortPoint3>,
) -> Ptr<NiNPShortPoint3> {
    let x = e.get(source, NiNPShortPoint3::x);
    e.set(this, NiNPShortPoint3::x, x);
    let y = e.get(source, NiNPShortPoint3::y);
    e.set(this, NiNPShortPoint3::y, y);
    let z = e.get(source, NiNPShortPoint3::z);
    e.set(this, NiNPShortPoint3::z, z);
    this
}

// Translated from 0050e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050e8f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The function-local `static` default bound: constructs it as (0, 0, 0)
/// the first time (bit 0 of the guard word) and returns its address. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_0050e8f0(e: &mut Engine) -> Ptr<NiNPShortPoint3> {
    if e.global::<u32>(DEFAULT_BOUND_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(DEFAULT_BOUND_GUARD);
        e.set_global(DEFAULT_BOUND_GUARD, guard | 1);
        fn_0050e960(e, Ptr::new(DEFAULT_BOUND), 0, 0, 0);
    }
    Ptr::new(DEFAULT_BOUND)
}

// Translated from 0050e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNPShortPoint3::NiNPShortPoint3(x, y, z)`-shaped constructor (unnamed
/// in the engine map): stores the low 16 bits of the three arguments.
pub fn fn_0050e960(
    e: &mut Engine,
    this: Ptr<NiNPShortPoint3>,
    x: i16,
    y: i16,
    z: i16,
) -> Ptr<NiNPShortPoint3> {
    e.set(this, NiNPShortPoint3::x, x);
    e.set(this, NiNPShortPoint3::y, y);
    e.set(this, NiNPShortPoint3::z, z);
    this
}

// Translated from 0050e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESBoundObject`: runs the destructor
/// body (`0050e9f0`) and, when bit 0 of `flags` is set, `operator delete`.
pub fn fn_0050e990(e: &mut Engine, this: Ptr<TESBoundObject>, flags: u32) -> Ptr<TESBoundObject> {
    tes_bound_object_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0050e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BoundData` (Xbox PDB `TESBoundObject::BOUND_DATA`):
/// runs the empty constructors of the two points (at +0 and +6).
pub fn fn_0050e9c0(e: &mut Engine, this: Ptr<BoundData>) -> Ptr<BoundData> {
    e.call(ALLOCATOR_CONSTRUCT, &args![this]);
    e.call(ALLOCATOR_CONSTRUCT, &args![this.byte_add(6)]);
    this
}

// Translated from 0050e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `TESBoundObject`: stores the vtable and runs the
/// `TESObject` destructor body (`0050f930`).
pub fn tes_bound_object_destruct(e: &mut Engine, this: Ptr<TESBoundObject>) {
    e.mem.set_u32(this.addr(), TES_BOUND_OBJECT_VTABLE);
    tes_object_destruct(e, this.cast());
}

// Translated from 0050ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the difference of the `NiPoint3`s at +0xC and +0x18 of the object
/// (`439ef0`) to `result`. The class is not named by the exe; the register
/// `EAX` is left holding the third result word, no result is returned.
pub fn fn_0050ea10(e: &mut Engine, this: Ptr, result: Ptr) {
    let base = this.addr();
    e.with_stack(12, |e, temporary| {
        let difference = e
            .call(
                POINT3_SUBTRACT,
                &args![base.wrapping_add(0xc), temporary, base.wrapping_add(0x18)],
            )
            .u32();
        copy_three_words(e, difference, result.addr());
    });
}

// Translated from 0050ea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the sum of the `NiPoint3`s at +0xC and +0x18 of the object
/// (`439e90`) to `result`; same shape as `0050ea10`.
pub fn fn_0050ea50(e: &mut Engine, this: Ptr, result: Ptr) {
    let base = this.addr();
    e.with_stack(12, |e, temporary| {
        let sum = e
            .call(
                POINT3_ADD,
                &args![base.wrapping_add(0xc), temporary, base.wrapping_add(0x18)],
            )
            .u32();
        copy_three_words(e, sum, result.addr());
    });
}

// Translated from 0050ea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `012031e4`.
pub fn fn_0050ea90(e: &mut Engine) -> u32 {
    e.global(0x0120_31e4)
}

// Translated from 0050eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 12 bytes of `BoundData` (+0x24) from `source`; `memcpy`'s
/// result (the destination) is what the function leaves in `EAX`.
pub fn fn_0050eaa0(e: &mut Engine, this: Ptr<TESBoundObject>, source: Ptr<TESBoundObject>) -> u32 {
    let destination = this.at(TESBoundObject::BoundData);
    let source = source.at(TESBoundObject::BoundData);
    e.call(MEMORY_COPY, &args![destination, source, 12u32])
        .u32()
}

// Translated from 0050ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `BoundMin` of the object as a `NiPoint3` (three floats) to
/// `result` and returns `result`.
pub fn fn_0050ead0(e: &mut Engine, this: Ptr<TESBoundObject>, result: Ptr) -> Ptr {
    let point = this.at(TESBoundObject::BoundData).at(BoundData::BoundMin);
    point_to_floats(e, point, result)
}

/// The body shared by `0050ead0` and `0050eb90`: reads z, y, x of the point,
/// builds a `NiPoint3` in a temporary and copies it to `result`.
fn point_to_floats(e: &mut Engine, point: Ptr<NiNPShortPoint3>, result: Ptr) -> Ptr {
    let z = fn_0050eb70(e, point);
    let y = fn_0050eb50(e, point);
    let x = fn_0050eb30(e, point);
    e.with_stack(12, |e, temporary| {
        e.call(POINT3_CONSTRUCT, &args![temporary, x, y, z]);
        copy_three_words(e, temporary.addr(), result.addr());
    });
    result
}

// Translated from 0050eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `x` component of a `NiNPShortPoint3`, converted to `float`.
pub fn fn_0050eb30(e: &mut Engine, this: Ptr<NiNPShortPoint3>) -> f32 {
    e.get(this, NiNPShortPoint3::x) as f32
}

// Translated from 0050eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `y` component of a `NiNPShortPoint3`, converted to `float`.
pub fn fn_0050eb50(e: &mut Engine, this: Ptr<NiNPShortPoint3>) -> f32 {
    e.get(this, NiNPShortPoint3::y) as f32
}

// Translated from 0050eb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `z` component of a `NiNPShortPoint3`, converted to `float`.
pub fn fn_0050eb70(e: &mut Engine, this: Ptr<NiNPShortPoint3>) -> f32 {
    e.get(this, NiNPShortPoint3::z) as f32
}

// Translated from 0050eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `BoundMax` of the object as a `NiPoint3` (three floats) to
/// `result` and returns `result`.
pub fn fn_0050eb90(e: &mut Engine, this: Ptr<TESBoundObject>, result: Ptr) -> Ptr {
    let point = this.at(TESBoundObject::BoundData).at(BoundData::BoundMax);
    point_to_floats(e, point, result)
}

// Translated from 0050ebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBoundObject::GetBoundSize` (Xbox PDB): half the length of the box's
/// diagonal (`BoundMin - BoundMax`, its length, times the `double` 0.5).
pub fn tes_bound_object_get_bound_size(e: &mut Engine, this: Ptr<TESBoundObject>) -> f32 {
    let minimum = this.at(TESBoundObject::BoundData).at(BoundData::BoundMin);
    let maximum = this.at(TESBoundObject::BoundData).at(BoundData::BoundMax);
    let length = e.with_stack(6, |e, difference| {
        let difference = fn_0050ec30(e, minimum, difference.cast(), maximum);
        fn_0050ec80(e, difference)
    });
    let half: f64 = e.global(HALF);
    (length * half) as f32
}

// Translated from 0050ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this - other` of two `NiNPShortPoint3`, stored in `result` (the low 16
/// bits of each difference); returns `result`.
pub fn fn_0050ec30(
    e: &mut Engine,
    this: Ptr<NiNPShortPoint3>,
    result: Ptr<NiNPShortPoint3>,
    other: Ptr<NiNPShortPoint3>,
) -> Ptr<NiNPShortPoint3> {
    let z =
        i32::from(e.get(this, NiNPShortPoint3::z)) - i32::from(e.get(other, NiNPShortPoint3::z));
    let y =
        i32::from(e.get(this, NiNPShortPoint3::y)) - i32::from(e.get(other, NiNPShortPoint3::y));
    let x =
        i32::from(e.get(this, NiNPShortPoint3::x)) - i32::from(e.get(other, NiNPShortPoint3::x));
    fn_0050e960(e, result, x as i16, y as i16, z as i16);
    result
}

// Translated from 0050ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of a `NiNPShortPoint3`: the 32-bit sum of the squares of the
/// components, rounded to `float` and passed to the square root (`4579e0`),
/// whose `ST0` result is returned unchanged.
pub fn fn_0050ec80(e: &mut Engine, this: Ptr<NiNPShortPoint3>) -> f64 {
    let x = i32::from(e.get(this, NiNPShortPoint3::x));
    let y = i32::from(e.get(this, NiNPShortPoint3::y));
    let z = i32::from(e.get(this, NiNPShortPoint3::z));
    let sum = x
        .wrapping_mul(x)
        .wrapping_add(y.wrapping_mul(y))
        .wrapping_add(z.wrapping_mul(z));
    e.call(SQUARE_ROOT, &args![sum as f32]).f64()
}

// Translated from 0050ece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the `OBND` chunk: when the byte-swap flag (`00401500`) is set,
/// swaps the six words of `BoundData` before and after writing the 12 bytes
/// with the chunk type `'OBND'` (`00485990`).
pub fn fn_0050ece0(e: &mut Engine, this: Ptr<TESBoundObject>) {
    let bound = this.at(TESBoundObject::BoundData);
    if e.call(BYTE_SWAP_FLAG_GETTER, &[]).bool() {
        fn_0050ed30(e, bound);
    }
    e.call(ADD_CHUNK_DATA, &args![OBND_CHUNK, bound, 12u32]);
    if e.call(BYTE_SWAP_FLAG_GETTER, &[]).bool() {
        fn_0050ed30(e, bound);
    }
}

// Translated from 0050ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the six 16-bit words of a `BoundData` in place (`00407a90`
/// on +6, +8, +0xA, +0, +2, +4, in that order).
pub fn fn_0050ed30(e: &mut Engine, this: Ptr<BoundData>) {
    for offset in [6u32, 8, 0xa, 0, 2, 4] {
        e.call(BYTE_SWAP_WORD, &args![this.byte_add(offset), 0u32]);
    }
}

// Translated from 0050edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the `OBND` chunk from the file object `file`: when it is non-null
/// and its current chunk type is `'OBND'`, reads 12 bytes into `BoundData`
/// and swaps them when the file's flag (`00401680`) says so.
pub fn fn_0050edd0(e: &mut Engine, this: Ptr<TESBoundObject>, file: Ptr) {
    if file.is_null() {
        return;
    }
    if e.call(FILE_CURRENT_CHUNK, &args![file]).u32() != OBND_CHUNK {
        return;
    }
    let bound = this.at(TESBoundObject::BoundData);
    e.call(FILE_READ_CHUNK, &args![file, bound, 12u32]);
    if e.call(FILE_FLAG_GETTER, &args![file]).bool() {
        fn_0050ed30(e, bound);
    }
}

// Translated from 0050ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the virtual `Clone3D` (slot +0x14C) with `reference` and 0.
pub fn fn_0050ee20(e: &mut Engine, this: Ptr<TESBoundObject>, reference: Ptr) -> Ptr {
    e.vcall(this.addr(), 0x14c, &args![reference, 0u32]).ptr()
}

// Translated from 0050ee50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050ee50 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// `TESBoundObject::Clone3D` (Xbox PDB): returns a scene-graph copy of the
/// form's model for `reference` (the object reference the model is for,
/// null for none; `00567400` reads its scale, the engine map names it
/// `TESObjectREFR::GetScale`, unconfirmed). `flag` asks for a deep copy
/// that keeps textures separate.
///
/// Steps, in the game's order: the base model comes from the virtual at
/// +0x174 (`reference` is dropped for the two special forms); a reference
/// whose owner is not form type 0x1e gets the model back unchanged. Unless
/// `flag` is set, a model whose `726070` is 1 can be shared as is (the
/// per-type switch, the extra-data, texture-swap and emittance checks, and
/// the scale equal to the model's `009cdae0` scale). Otherwise the model is
/// cloned (`a5d2c0`, or a deep copy through the helper object when `flag` is
/// set), with the scale `1.0` unless the reference has another; a scale of 0
/// prints a message and becomes 1.0. When the scale is not 1.0 and the source model
/// reports 1.0, the model is looked up by its scaled file name
/// (`0050fdf0`) in the model loader, added when missing, and cloned again
/// at scale 1.0 (a found model of a matching scale is cloned through the
/// virtual at +0x14C instead, a mismatch prints a message). The result gets
/// the final fix-ups (`440490`, `450f90`, `43fa80`, `440460`, `50f5a0`).
/// The exception-unwinding frame and the cookie check are not translated.
pub fn tes_bound_object_clone_3d(
    e: &mut Engine,
    this: Ptr<TESBoundObject>,
    reference: Ptr,
    flag: u8,
) -> Ptr {
    let mut reference = reference;
    let mut flag = flag;
    let mut clone: u32 = 0;
    if this.addr() == e.global::<u32>(SPECIAL_FORM_A)
        || this.addr() == e.global::<u32>(SPECIAL_FORM_B)
    {
        reference = Ptr::NULL;
    }
    let source = e.vcall(this.addr(), 0x174, &args![reference]).u32();
    if !reference.is_null() && e.call(0x0044_4ed0, &args![reference]).bool() {
        let owner = e.call(0x007a_f430, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![owner]).u32() != 0x1e {
            return Ptr::new(source);
        }
    }
    if source == 0 {
        return Ptr::new(clone);
    }
    if e.call(0x0050_d4a0, &args![this]).bool() && !reference.is_null() {
        e.call(0x0069_3ef0, &args![reference, 1u32]);
    }
    if flag == 0 && e.call(0x0072_6070, &args![source]).u32() == 1 {
        let mut shareable = false;
        let form_type = e.call(FORM_TYPE, &args![this]).u32();
        match form_type {
            0x15 | 0x2b => {
                if e.call(0x004b_5bf0, &args![source]).bool() {
                    flag = 1;
                }
            }
            0x1c => {
                shareable = true;
                let node = e.vcall(source, 0xc, &[]).u32();
                if node != 0 && e.call(0x0045_bc00, &args![node, 0u32]).u32() != 0 {
                    let first = e.call(0x0043_b4a0, &args![node, 0u32]).u32();
                    if e.call(0x0043_b230, &args![first]).u32() != 0 {
                        let second = e.call(0x0043_b4a0, &args![node, 0u32]).u32();
                        let key = e.call(0x0043_b230, &args![second]).u32();
                        let found = e
                            .call(0x0065_3270, &args![CLONE_ADDRESS_ARGUMENT, key])
                            .u32();
                        if found != 0 {
                            let table = fn_0050f5c0(e);
                            if e.call(0x0047_a520, &args![found, table]).u32() != 0 {
                                shareable = false;
                            }
                        }
                    }
                }
            }
            0x23 | 0x28 | 0x29 | 0x2a | 0x33 | 0x51 => {}
            _ => {
                shareable = e.call(0x0062_c3c0, &args![reference, source]).u32() as u8 == 0;
            }
        }
        if shareable {
            let property_name = e.call(0x0044_8a80, &[]).u32();
            let extra = e.call(0x00a5_bdd0, &args![source, property_name]).u32();
            if extra != 0 && e.call(0x0044_8a40, &args![extra]).bool() {
                shareable = false;
            }
        }
        if shareable && e.call(0x0048_b310, &args![this, reference]).bool() {
            shareable = false;
        }
        if shareable && e.call(0x0056_9410, &args![source]).bool() {
            shareable = false;
        }
        if shareable {
            let mut same_scale = true;
            if !reference.is_null() {
                let reference_scale = e.call(0x0056_7400, &args![reference]).f64();
                let model_scale = e.call(0x009c_dae0, &args![source]).f64();
                same_scale = reference_scale == model_scale;
            }
            if same_scale {
                clone = source;
            }
        }
    }
    if clone == 0 {
        let mut scale = 1.0f32;
        if !reference.is_null() {
            scale = e.call(0x0056_7400, &args![reference]).f32();
        }
        let model_scale = e.call(0x009c_dae0, &args![source]).f64();
        if model_scale == f64::from(scale) {
            scale = 1.0;
        }
        let zero: f64 = e.global(ZERO);
        if f64::from(scale) == zero {
            let name = e.call(0x0055_d520, &args![reference]).u32();
            e.call(PRINT_MESSAGE, &args![SCALE_ZERO_MESSAGE, name]);
            scale = 1.0;
        }
        let process = e.mem.alloc(0x1c);
        e.call(0x004a_d050, &args![process, pushed_float(scale)]);
        if flag != 0 {
            let helper: u32 = e.global(CLONE_HELPER_POINTER);
            let copy = e.call(0x0045_7ba0, &args![helper, source, process]).u32();
            if copy != 0 && e.vcall(copy, 0xc, &[]).u32() != 0 {
                clone = e.vcall(copy, 0xc, &[]).u32();
            } else {
                let block = e.call(0x00aa_13e0, &args![0xacu32]).u32();
                let node = if block != 0 {
                    e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
                } else {
                    0
                };
                e.vcall(node, 0xdc, &args![copy, 1u32]);
                clone = node;
            }
        } else {
            clone = e.call(0x00a5_d2c0, &args![source, process]).u32();
        }
        let one: f64 = e.global(ONE);
        if f64::from(scale) != one && e.call(0x009c_dae0, &args![source]).f64() == one {
            e.call(0x0044_0490, &args![clone, pushed_float(scale)]);
            let name = e.mem.alloc(0x108);
            e.call(0x0050_fdf0, &args![this, reference, name]);
            let model = e.mem.alloc(4);
            e.call(0x0063_3c90, &args![model, 0u32]);
            let loader: u32 = e.global(MODEL_LOADER_POINTER);
            e.call(0x0044_72a0, &args![loader, name, model]);
            if e.call(0x0055_9450, &args![model]).u32() == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
                let created = if block != 0 {
                    e.call(0x0043_abf0, &args![block, name, clone]).u32()
                } else {
                    0
                };
                e.call(0x0044_aed0, &args![model, created]);
                let current = e.call(0x0055_9450, &args![model]).u32();
                e.call(0x0092_c870, &args![current]);
                let current = e.call(0x0055_9450, &args![model]).u32();
                let loader: u32 = e.global(MODEL_LOADER_POINTER);
                if e.call(0x0044_7010, &args![loader, name, current]).bool() {
                    let counter: u32 = e.global(COUNTER_A);
                    e.set_global(COUNTER_A, counter.wrapping_add(1));
                } else {
                    let current = e.call(0x0055_9450, &args![model]).u32();
                    e.call(0x0044_aed0, &args![model, 0u32]);
                    if current != 0 {
                        e.call(0x0044_31f0, &args![current, 1u32]);
                    }
                }
                let second_process = e.mem.alloc(0x1c);
                e.call(0x004a_d050, &args![second_process, pushed_float(1.0)]);
                clone = e.call(0x00a5_d2c0, &args![clone, second_process]).u32();
                e.call(0x004a_d270, &args![second_process]);
                e.mem.free(second_process);
            } else {
                let current = e.call(0x0055_9450, &args![model]).u32();
                if e.call(0x0043_b230, &args![current]).u32() != 0 {
                    let current = e.call(0x0055_9450, &args![model]).u32();
                    let found = e.call(0x0043_b230, &args![current]).u32();
                    let found_scale = e.call(0x009c_dae0, &args![found]).f64();
                    let reference_scale = e.call(0x0056_7400, &args![reference]).f64();
                    if found_scale == reference_scale {
                        let result = e
                            .vcall(this.addr(), 0x14c, &args![reference, u32::from(flag)])
                            .u32();
                        e.call(0x0040_c110, &args![model]);
                        e.call(0x004a_d270, &args![process]);
                        e.mem.free(model);
                        e.mem.free(name);
                        e.mem.free(process);
                        return Ptr::new(result);
                    }
                }
                e.call(PRINT_MESSAGE, &args![SCALE_INVALID_MESSAGE, name]);
            }
            e.call(0x0040_c110, &args![model]);
            e.mem.free(model);
            e.mem.free(name);
        }
        e.call(0x004a_d270, &args![process]);
        e.mem.free(process);
    }
    if clone != 0 {
        e.call(0x0044_0490, &args![clone, pushed_float(1.0)]);
        e.call(0x0045_0f90, &args![clone, 0u32]);
        e.call(0x0043_fa80, &args![clone, CLONE_NAME_TABLE]);
        e.call(0x0044_0460, &args![clone, CLONE_DATA_TABLE]);
        if e.call(0x0096_11e0, &args![clone]).u32() != 0 {
            fn_0050f5a0(e, Ptr::new(clone));
        }
    }
    Ptr::new(clone)
}

// Translated from 0050f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `m_pParentList` (+0x18) of a `TESObject`.
pub fn fn_0050f5a0(e: &mut Engine, this: Ptr<TESObject>) {
    e.set(this, TESObject::m_pParentList, Ptr::NULL);
}

// Translated from 0050f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `011c6218`.
pub fn fn_0050f5c0(e: &mut Engine) -> u32 {
    e.global(0x011c_6218)
}

// Translated from 0050f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the virtual at +0x17C with the form's model path (`004895b0`) and
/// returns its result.
pub fn fn_0050f5d0(e: &mut Engine, this: Ptr<TESBoundObject>) -> u32 {
    let model = e.call(FORM_MODEL_PATH, &args![this]).u32();
    e.vcall(this.addr(), 0x17c, &args![model]).u32()
}

// Translated from 0050f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the form's model path with the model loader (`0045a5e0`), loads
/// the scene node through the virtual at +0x174 (argument 0), runs `a5a040`
/// on it and sets its local translation (`a59c60`) to the point (0, 0, 0)
/// built by `43d410`. Always true; its one argument is not read.
pub fn fn_0050f600(e: &mut Engine, this: Ptr<TESBoundObject>, _unused_1: u32) -> bool {
    let model = e.call(FORM_MODEL_PATH, &args![this]).u32();
    let loader: u32 = e.global(MODEL_LOADER_POINTER);
    e.call(0x0045_a5e0, &args![loader, model]);
    let node = e.vcall(this.addr(), 0x174, &args![0u32]).u32();
    e.call(0x00a5_a040, &args![node]);
    e.with_stack(12, |e, origin| {
        e.call(0x0043_d410, &args![origin, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![node, origin]);
    });
    true
}

// Translated from 0050f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic-casts `object` (the exe's type descriptors `011841cc` to
/// `011846d4`) and, when both the cast result and `target` are non-null,
/// calls its virtual at +0x3D0 with (`target`, `extra`, `flag`). True when
/// it called. The fourth argument is not read.
pub fn fn_0050f670(
    e: &mut Engine,
    _this: Ptr,
    target: Ptr,
    object: Ptr,
    flag: u8,
    _unused_4: u32,
    extra: u32,
) -> bool {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![object, 0u32, 0x0118_41ccu32, 0x0118_46d4u32, 0u32],
        )
        .u32();
    if cast != 0 && !target.is_null() {
        e.vcall(cast, 0x3d0, &args![target, extra, flag]);
        true
    } else {
        false
    }
}

// Translated from 0050f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `011f81dc`.
pub fn fn_0050f6e0(e: &mut Engine) -> u32 {
    e.global(0x011f_81dc)
}

// Translated from 0050f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBoundAnimObject::TESBoundAnimObject` (Xbox PDB): the `TESBoundObject`
/// constructor, then the `TESBoundAnimObject` vtable.
pub fn tes_bound_anim_object_construct(
    e: &mut Engine,
    this: Ptr<TESBoundObject>,
) -> Ptr<TESBoundObject> {
    tes_bound_object_construct(e, this);
    e.mem.set_u32(this.addr(), TES_BOUND_ANIM_OBJECT_VTABLE);
    this
}

// Translated from 0050f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESBoundAnimObject`.
pub fn fn_0050f710(e: &mut Engine, this: Ptr<TESBoundObject>, flags: u32) -> Ptr<TESBoundObject> {
    tes_bound_anim_object_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0050f740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `TESBoundAnimObject`: its vtable, then the
/// `TESBoundObject` destructor body.
pub fn tes_bound_anim_object_destruct(e: &mut Engine, this: Ptr<TESBoundObject>) {
    e.mem.set_u32(this.addr(), TES_BOUND_ANIM_OBJECT_VTABLE);
    tes_bound_object_destruct(e, this);
}

// Translated from 0050f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0050f600` with its argument and returns its result (`AL`).
pub fn fn_0050f760(e: &mut Engine, this: Ptr<TESBoundObject>, argument: u32) -> bool {
    fn_0050f600(e, this, argument)
}

// Translated from 0050f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards (`argument`, `flag`) to `008d0600` (the engine map names it
/// `ProcessLists::PrintLists`; unconfirmed) with the same `this`. Both are
/// `void` (Ghidra; `008d0600` returns at once): nothing is answered.
pub fn fn_0050f780(e: &mut Engine, this: Ptr, argument: u32, flag: u8) {
    e.call(0x008d_0600, &args![this, argument, flag]);
}

// Translated from 0050f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBoundTreeObject`'s constructor: the `TESBoundObject` constructor,
/// then the `TESBoundTreeObject` vtable.
pub fn tes_bound_tree_object_construct(
    e: &mut Engine,
    this: Ptr<TESBoundObject>,
) -> Ptr<TESBoundObject> {
    tes_bound_object_construct(e, this);
    e.mem.set_u32(this.addr(), TES_BOUND_TREE_OBJECT_VTABLE);
    this
}

// Translated from 0050f7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESBoundTreeObject`.
pub fn fn_0050f7c0(e: &mut Engine, this: Ptr<TESBoundObject>, flags: u32) -> Ptr<TESBoundObject> {
    tes_bound_tree_object_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0050f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `TESBoundTreeObject`: its vtable, then the
/// `TESBoundObject` destructor body.
pub fn tes_bound_tree_object_destruct(e: &mut Engine, this: Ptr<TESBoundObject>) {
    e.mem.set_u32(this.addr(), TES_BOUND_TREE_OBJECT_VTABLE);
    tes_bound_object_destruct(e, this);
}

// Translated from 0050f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a form of type 0x25 (`401170`): asks the object manager (`00664840`
/// with 1) to build something for (`argument`, this) (`00664f50`), hands the
/// result to the virtual at +0x180, and, when the result's virtual at +0x100
/// answers non-zero and `argument` is non-null, passes that answer's
/// `008d6f30` to `00569350` on `argument`. Returns the built result, or 0
/// for another form type.
/// The second stack word (`RET 8`) is not read.
pub fn fn_0050f810(e: &mut Engine, this: Ptr, argument: Ptr, _unused_2: u32) -> Ptr {
    let mut result = Ptr::NULL;
    if e.call(FORM_TYPE, &args![this]).u32() != 0x25 {
        return result;
    }
    if e.call(0x0066_4840, &args![1u32]).u32() != 0 {
        let manager = e.call(0x0066_4840, &args![1u32]).u32();
        result = e.call(0x0066_4f50, &args![manager, argument, this]).ptr();
    }
    e.vcall(this.addr(), 0x180, &args![result]);
    if !result.is_null() && e.vcall(result.addr(), 0x100, &[]).u32() != 0 && !argument.is_null() {
        let answer = e.vcall(result.addr(), 0x100, &[]).u32();
        let value = e.call(0x008d_6f30, &args![answer]).u32();
        e.call(0x0056_9350, &args![argument, value]);
    }
    result
}

// Translated from 0050f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObject::TESObject`: the `TESForm` constructor, the `TESObject` vtable
/// and the list node (`nod_lpNext`, `nod_lpPrev`, `m_pParentList`) cleared.
pub fn tes_object_construct(e: &mut Engine, this: Ptr<TESObject>) -> Ptr<TESObject> {
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_OBJECT_VTABLE);
    e.set(this, TESObject::nod_lpPrev, Ptr::NULL);
    e.set(this, TESObject::nod_lpNext, Ptr::NULL);
    e.set(this, TESObject::m_pParentList, Ptr::NULL);
    this
}

// Translated from 0050f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESObject`.
pub fn fn_0050f900(e: &mut Engine, this: Ptr<TESObject>, flags: u32) -> Ptr<TESObject> {
    tes_object_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0050f930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050f930 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body of `TESObject`: its vtable, removes the object from
/// its parent list (`00510130`, list as `ECX`) when it has one, then runs
/// the `TESForm` destructor body. The exception-unwinding frame is not
/// translated.
pub fn tes_object_destruct(e: &mut Engine, this: Ptr<TESObject>) {
    e.mem.set_u32(this.addr(), TES_OBJECT_VTABLE);
    let list = e.get(this, TESObject::m_pParentList);
    if !list.is_null() {
        e.call(LIST_REMOVE, &args![list, this]);
    }
    e.call(TES_FORM_DESTRUCT, &args![this]);
}

// Translated from 0050f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `nod_lpPrev` (+0x1C) of a `TESObject`.
pub fn fn_0050f9a0(e: &mut Engine, this: Ptr<TESObject>, previous: Ptr) {
    e.set(this, TESObject::nod_lpPrev, previous);
}

// Translated from 0050f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x20 (`RET 4`). The PC body is shared by otherwise
/// unrelated callers; the inventory buffer save `004d4090` uses it to switch
/// a save buffer's mode word, as `007037c0` does for the word at +0x24 when
/// loading.
pub fn fn_0050f9c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x20), value);
}

// Translated from 0050f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObject::IsMarker` (Xbox PDB): true for a non-null form that is a
/// marker. By form type (`401170`): 4 and 0x30 are markers; type 0x15 when
/// its model path (`004895b0`) equals `"EditorMarker.NIF"`; type 0x1c when
/// it is the global form `011ca258`; type 0x1e when its model length
/// (`00489540`) is 0; type 0x20 when it is one of the fourteen global forms.
/// Everything else (and the cases above that did not match) is a marker when
/// the form's second type code (`0084e3a0`) lies in 100..=0x77.
pub fn tes_object_is_marker(e: &mut Engine, this: Ptr) -> bool {
    if this.is_null() {
        return false;
    }
    let form_type = e.call(FORM_TYPE, &args![this]).u32();
    match form_type {
        0x04 | 0x30 => return true,
        0x15 => {
            let model = e.call(FORM_MODEL_PATH, &args![this]).u32();
            if e.call(COMPARE_NAMES, &args![model, EDITOR_MARKER_MODEL])
                .u32()
                == 0
            {
                return true;
            }
        }
        0x1c => {
            if e.global::<u32>(MARKER_FORM_TYPE_1C) == this.addr() {
                return true;
            }
        }
        0x1e => {
            if e.call(FORM_MODEL_LENGTH, &args![this]).u32() == 0 {
                return true;
            }
        }
        0x20 => {
            for global in MARKER_FORMS_TYPE_20 {
                if e.global::<u32>(global) == this.addr() {
                    return true;
                }
            }
        }
        _ => {}
    }
    let subtype = e.call(FORM_SUBTYPE, &args![this]).u32();
    (0x64..=0x77).contains(&subtype)
}

// Translated from 0050fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the form is non-null, of type 0x20 and one of the three
/// globals `011ca238`, `011ca234`, `011ca23c`.
pub fn fn_0050fb90(e: &mut Engine, this: Ptr) -> bool {
    if this.is_null() {
        return false;
    }
    if e.call(FORM_TYPE, &args![this]).u32() != 0x20 {
        return false;
    }
    for global in NAMED_FORMS_TYPE_20 {
        if e.global::<u32>(global) == this.addr() {
            return true;
        }
    }
    false
}

// Translated from 0050fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes two words it never reads and returns 0.
pub fn fn_0050fbe0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 0050fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050fbf0 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// Registers the form's model with the model loader. The reference is
/// dropped for the two special forms; the model path comes from
/// `0050fd90`. A non-empty path with a reference whose scale (`00567400`)
/// is not 1.0 registers the scaled file name (`0050fdf0`) instead: it is
/// registered with the loader (`0045a5e0`), looked up (`004472a0`), and
/// depending on the found model's reference count (`0044ddc0`) either the
/// plain path is registered too and counter A decremented, or counter B is
/// decremented. Otherwise the path is registered. A form of type 0x2a then
/// runs `00603bb0`. The exception-unwinding frame and the cookie check are
/// not translated.
pub fn fn_0050fbf0(e: &mut Engine, this: Ptr, reference: Ptr) {
    let mut reference = reference;
    if this.addr() == e.global::<u32>(SPECIAL_FORM_A)
        || this.addr() == e.global::<u32>(SPECIAL_FORM_B)
    {
        reference = Ptr::NULL;
    }
    let path = fn_0050fd90(e, this, reference);
    let loader: u32 = e.global(MODEL_LOADER_POINTER);
    if path != 0 && e.mem.i8(path) != 0 {
        let mut scaled = false;
        if !reference.is_null() {
            scaled = e.call(0x0056_7400, &args![reference]).f64() != e.global::<f64>(ONE);
        }
        if scaled {
            let name = e.mem.alloc(0x108);
            e.call(0x0050_fdf0, &args![this, reference, name]);
            e.call(0x0045_a5e0, &args![loader, name]);
            let mut register_plain = true;
            let model = e.mem.alloc(4);
            e.call(0x0063_3c90, &args![model, 0u32]);
            if e.call(0x0044_72a0, &args![loader, name, model]).bool() {
                let found = e.call(0x0055_9450, &args![model]).u32();
                if e.call(0x0044_ddc0, &args![found]).i32() > 0 {
                    register_plain = false;
                }
            }
            if register_plain {
                e.call(0x0045_a5e0, &args![loader, path]);
                let counter: u32 = e.global(COUNTER_A);
                e.set_global(COUNTER_A, counter.wrapping_sub(1));
            } else {
                let counter: u32 = e.global(COUNTER_B);
                e.set_global(COUNTER_B, counter.wrapping_sub(1));
            }
            e.call(0x0040_c110, &args![model]);
            e.mem.free(model);
            e.mem.free(name);
        } else {
            e.call(0x0045_a5e0, &args![loader, path]);
        }
    }
    if e.call(FORM_TYPE, &args![this]).u32() == 0x2a {
        e.call(0x0060_3bb0, &args![this]);
    }
}

// Translated from 0050fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The model path for a form and an optional reference: the reference's
/// own (`005715d0`, engine map `TESObjectREFR::GetModel`) if it has one,
/// else the form's biped-model one (`00480e60`, engine map
/// `TESBipedModelForm::GetIcon`; unconfirmed), else the plain model path
/// (`004895b0`).
pub fn fn_0050fd90(e: &mut Engine, this: Ptr, reference: Ptr) -> u32 {
    let mut path = 0;
    if !reference.is_null() {
        path = e.call(0x0057_15d0, &args![reference]).u32();
    }
    if path == 0 {
        path = e.call(0x0048_0e60, &args![this, reference]).u32();
    }
    if path == 0 {
        path = e.call(FORM_MODEL_PATH, &args![this]).u32();
    }
    path
}

/// Format string of `0050fdf0`: `"%s%i"` (path, then the scale in percent).
const SCALED_NAME_FORMAT: u32 = 0x0102_9ca8;
/// The double `100.0` that `0050fdf0` multiplies the reference's scale by.
const ONE_HUNDRED: u32 = 0x0101_7a40;

// Translated from 0050fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObject::GetScaledModelFileName` (Xbox PDB): writes the form's model
/// path (`0050fd90`) followed by the reference's scale in percent
/// (`00567400` times 100.0, stored as a float and rounded by the
/// float-to-int helper `00406d90`) into the 0x104 byte buffer `name`, with
/// the `sprintf_s` variant `00406d00` and the format `"%s%i"`.
pub fn tes_object_get_scaled_model_file_name(e: &mut Engine, this: Ptr, reference: Ptr, name: Ptr) {
    let path = fn_0050fd90(e, this, reference);
    let scale = e.call(0x0056_7400, &args![reference]).f64();
    let percent = e.global::<f64>(ONE_HUNDRED);
    // FMUL, FSTP to a float, then FLD/FSTP again to push it on the stack.
    let product = x87_quiet_bits(((scale * percent) as f32).to_bits());
    let rounded = e.call(0x0040_6d90, &args![product]).u32();
    e.call(
        0x0040_6d00,
        &args![name, 0x104u32, SCALED_NAME_FORMAT, path, rounded],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0044a4c0,
            std_string_construct_c_str(Ptr<DebugString>, Ptr) -> Ptr<DebugString>
        ),
        entry!(
            0x0044a500,
            std_string_copy_construct(Ptr<DebugString>, Ptr<DebugString>) -> Ptr<DebugString>
        ),
        entry!(0x0044a540, std_string_destruct(Ptr<DebugString>)),
        entry!(0x0044a560, std_string_c_str(Ptr<DebugString>) -> Ptr),
        entry!(
            0x0044a580,
            std_string_assign_substring(
                Ptr<DebugString>,
                Ptr<DebugString>,
                u32,
                u32,
            ) -> Ptr<DebugString>
        ),
        entry!(
            0x0044a640,
            std_string_assign_c_str(Ptr<DebugString>, Ptr) -> Ptr<DebugString>
        ),
        entry!(0x0044a670, fn_0044a670(Ptr) -> u32),
        entry!(
            0x0044a690,
            std_string_erase(Ptr<DebugString>, u32, u32) -> Ptr<DebugString>
        ),
        entry!(0x0044a730, std_string_tidy(Ptr<DebugString>, bool, u32)),
        entry!(
            0x0044a7b0,
            fn_0044a7b0(Ptr<DebugString>, u32) -> Ptr<DebugString>
        ),
        entry!(
            0x0044a7d0,
            fn_0044a7d0(Ptr<DebugString>, Ptr) -> Ptr<DebugString>
        ),
        entry!(
            0x0044a7f0,
            std_string_assign_chars(Ptr<DebugString>, Ptr, u32) -> Ptr<DebugString>
        ),
        entry!(
            0x0050e820,
            tes_bound_object_construct(Ptr<TESBoundObject>) -> Ptr<TESBoundObject>
        ),
        entry!(
            0x0050e8b0,
            fn_0050e8b0(Ptr<NiNPShortPoint3>, Ptr<NiNPShortPoint3>) -> Ptr<NiNPShortPoint3>
        ),
        entry!(0x0050e8f0, fn_0050e8f0() -> Ptr<NiNPShortPoint3>),
        entry!(
            0x0050e960,
            fn_0050e960(Ptr<NiNPShortPoint3>, i16, i16, i16) -> Ptr<NiNPShortPoint3>
        ),
        entry!(
            0x0050e990,
            fn_0050e990(Ptr<TESBoundObject>, u32) -> Ptr<TESBoundObject>
        ),
        entry!(0x0050e9c0, fn_0050e9c0(Ptr<BoundData>) -> Ptr<BoundData>),
        entry!(0x0050e9f0, tes_bound_object_destruct(Ptr<TESBoundObject>)),
        entry!(0x0050ea10, fn_0050ea10(Ptr, Ptr)),
        entry!(0x0050ea50, fn_0050ea50(Ptr, Ptr)),
        entry!(0x0050ea90, fn_0050ea90() -> u32),
        entry!(
            0x0050eaa0,
            fn_0050eaa0(Ptr<TESBoundObject>, Ptr<TESBoundObject>) -> u32
        ),
        entry!(0x0050ead0, fn_0050ead0(Ptr<TESBoundObject>, Ptr) -> Ptr),
        entry!(0x0050eb30, fn_0050eb30(Ptr<NiNPShortPoint3>) -> f32),
        entry!(0x0050eb50, fn_0050eb50(Ptr<NiNPShortPoint3>) -> f32),
        entry!(0x0050eb70, fn_0050eb70(Ptr<NiNPShortPoint3>) -> f32),
        entry!(0x0050eb90, fn_0050eb90(Ptr<TESBoundObject>, Ptr) -> Ptr),
        entry!(
            0x0050ebf0,
            tes_bound_object_get_bound_size(Ptr<TESBoundObject>) -> f32
        ),
        entry!(
            0x0050ec30,
            fn_0050ec30(
                Ptr<NiNPShortPoint3>,
                Ptr<NiNPShortPoint3>,
                Ptr<NiNPShortPoint3>,
            ) -> Ptr<NiNPShortPoint3>
        ),
        entry!(0x0050ec80, fn_0050ec80(Ptr<NiNPShortPoint3>) -> f64),
        entry!(0x0050ece0, fn_0050ece0(Ptr<TESBoundObject>)),
        entry!(0x0050ed30, fn_0050ed30(Ptr<BoundData>)),
        entry!(0x0050edd0, fn_0050edd0(Ptr<TESBoundObject>, Ptr)),
        entry!(0x0050ee20, fn_0050ee20(Ptr<TESBoundObject>, Ptr) -> Ptr),
        entry!(
            0x0050ee50,
            tes_bound_object_clone_3d(Ptr<TESBoundObject>, Ptr, u8) -> Ptr
        ),
        entry!(0x0050f5a0, fn_0050f5a0(Ptr<TESObject>)),
        entry!(0x0050f5c0, fn_0050f5c0() -> u32),
        entry!(0x0050f5d0, fn_0050f5d0(Ptr<TESBoundObject>) -> u32),
        entry!(0x0050f600, fn_0050f600(Ptr<TESBoundObject>, u32) -> bool),
        entry!(0x0050f670, fn_0050f670(Ptr, Ptr, Ptr, u8, u32, u32) -> bool),
        entry!(0x0050f6e0, fn_0050f6e0() -> u32),
        entry!(
            0x0050f6f0,
            tes_bound_anim_object_construct(Ptr<TESBoundObject>) -> Ptr<TESBoundObject>
        ),
        entry!(
            0x0050f710,
            fn_0050f710(Ptr<TESBoundObject>, u32) -> Ptr<TESBoundObject>
        ),
        entry!(
            0x0050f740,
            tes_bound_anim_object_destruct(Ptr<TESBoundObject>)
        ),
        entry!(0x0050f760, fn_0050f760(Ptr<TESBoundObject>, u32) -> bool),
        entry!(0x0050f780, fn_0050f780(Ptr, u32, u8)),
        entry!(
            0x0050f7a0,
            tes_bound_tree_object_construct(Ptr<TESBoundObject>) -> Ptr<TESBoundObject>
        ),
        entry!(
            0x0050f7c0,
            fn_0050f7c0(Ptr<TESBoundObject>, u32) -> Ptr<TESBoundObject>
        ),
        entry!(
            0x0050f7f0,
            tes_bound_tree_object_destruct(Ptr<TESBoundObject>)
        ),
        entry!(0x0050f810, fn_0050f810(Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x0050f8c0,
            tes_object_construct(Ptr<TESObject>) -> Ptr<TESObject>
        ),
        entry!(
            0x0050f900,
            fn_0050f900(Ptr<TESObject>, u32) -> Ptr<TESObject>
        ),
        entry!(0x0050f930, tes_object_destruct(Ptr<TESObject>)),
        entry!(0x0050f9a0, fn_0050f9a0(Ptr<TESObject>, Ptr)),
        entry!(0x0050f9c0, fn_0050f9c0(Ptr, u32)),
        entry!(0x0050f9e0, tes_object_is_marker(Ptr) -> bool),
        entry!(0x0050fb90, fn_0050fb90(Ptr) -> bool),
        entry!(0x0050fbe0, fn_0050fbe0(Ptr, u32, u32) -> u32),
        entry!(0x0050fbf0, fn_0050fbf0(Ptr, Ptr)),
        entry!(0x0050fd90, fn_0050fd90(Ptr, Ptr) -> u32),
        entry!(
            0x0050fdf0,
            tes_object_get_scaled_model_file_name(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch word in a mapped page, for doubles to pass values to tests.
    const SCRATCH: u32 = 0x0101_7100;
    const SCRATCH_TYPE: u32 = 0x0101_7110;
    const SCRATCH_SUBTYPE: u32 = 0x0101_7114;
    const SCRATCH_COMPARE: u32 = 0x0101_7118;
    const SCRATCH_LENGTH: u32 = 0x0101_711c;
    const SCRATCH_COUNT: u32 = 0x0101_7120;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the pages that hold the globals these functions read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_7000,
            0x011c_3000,
            0x011c_6000,
            0x011c_9000,
            0x011c_a000,
            0x011d_e000,
            0x011f_8000,
            0x0120_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(STRING_NPOS, u32::MAX);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(HALF, 0.5f64);
        e
    }

    /// Registers do-nothing doubles (result 0) at the addresses.
    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| ret(0));
        }
    }

    /// Starts recording calls.
    fn start_log(e: &mut Engine) {
        e.call_log = Some(Vec::new());
    }

    /// The recorded calls, without the test's own top-level call.
    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        let mut log = e.call_log.take().expect("the call log was started");
        log.remove(0);
        log
    }

    fn addresses(calls: &[(u32, Vec<u32>)]) -> Vec<u32> {
        calls.iter().map(|call| call.0).collect()
    }

    /// An object whose vtable (at word 0) holds the `(offset, target)` slots.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(offset, target) in slots {
            e.mem.set_u32(vtable + offset, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn c_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text);
        address
    }

    // ----- the debug-heap std::string -----

    /// Doubles for the string helpers the instance calls; the data pointer,
    /// the length setter and the byte copies behave like the library's.
    fn string_engine() -> Engine {
        let mut e = engine();
        e.register(ALLOCATOR_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ALLOCATOR_PROXY_CONSTRUCT, |e, a| {
            let slot_word = e.mem.u32(a[1]);
            e.set_global(SCRATCH, slot_word);
            ret(a[0])
        });
        e.register(STRING_DATA, |e, a| {
            if e.mem.u32(a[0] + 0x18) >= 0x10 {
                ret(e.mem.u32(a[0] + 4))
            } else {
                ret(a[0] + 4)
            }
        });
        e.register(STRING_SET_LENGTH, |e, a| {
            let data = if e.mem.u32(a[0] + 0x18) >= 0x10 {
                e.mem.u32(a[0] + 4)
            } else {
                a[0] + 4
            };
            e.mem.set_u32(a[0] + 0x14, a[1]);
            e.mem.set_u8(data + a[1], 0);
            ret(0)
        });
        e.register(STRING_SIZE_GETTER, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(STRING_GROW, |_, _| ret(1));
        e.register(STRING_INSIDE, |e, a| {
            let data = a[0] + 4;
            let size = e.mem.u32(a[0] + 0x14);
            ret(u32::from(a[1] >= data && a[1] < data + size))
        });
        e.register(COPY_CHARS, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            ret(0)
        });
        // `0044d580` takes the ECX it is entered with first.
        e.register(MOVE_CHARS, |e, a| {
            let bytes = e.mem.bytes(a[3], a[4]);
            e.mem.write(a[1], &bytes);
            ret(0)
        });
        e.register(STRING_OUT_OF_RANGE, |_, _| ret(0));
        e.register(STRING_FREE, |_, _| ret(0));
        e.register(STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e
    }

    /// A string using its inline buffer.
    fn new_string(e: &mut Engine, text: &[u8]) -> u32 {
        let string = e.mem.alloc(0x1c);
        e.mem.set_u32(string + 0x18, 0xf);
        e.mem.set_cstr(string + 4, text);
        e.mem.set_u32(string + 0x14, text.len() as u32);
        string
    }

    fn string_text(e: &Engine, string: u32) -> Vec<u8> {
        let size = e.mem.u32(string + 0x14);
        let data = if e.mem.u32(string + 0x18) >= 0x10 {
            e.mem.u32(string + 4)
        } else {
            string + 4
        };
        e.mem.bytes(data, size)
    }

    #[test]
    fn string_from_c_str_builds_the_proxy_and_assigns_the_text() {
        let mut e = string_engine();
        let this = e.mem.alloc(0x1c);
        let text = c_string(&mut e, b"abc");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a4c0, &[this, text]).u32(), this);
        assert_eq!(string_text(&e, this), b"abc");
        assert_eq!(e.mem.u32(this + 0x18), 0xf);
        // The by-value allocator slot holds the caller's `ECX`, which is `this`.
        assert_eq!(e.global::<u32>(SCRATCH), this);
        let log = calls(&mut e);
        assert_eq!(
            addresses(&log)[..2].to_vec(),
            vec![ALLOCATOR_CONSTRUCT, ALLOCATOR_PROXY_CONSTRUCT]
        );
    }

    #[test]
    fn string_copy_construct_assigns_the_whole_source() {
        let mut e = string_engine();
        let right = new_string(&mut e, b"hello");
        let this = e.mem.alloc(0x1c);
        start_log(&mut e);
        assert_eq!(e.call(0x0044a500, &[this, right]).u32(), this);
        assert_eq!(string_text(&e, this), b"hello");
        let log = calls(&mut e);
        assert_eq!(log[0], (ALLOCATOR_PROXY_CONSTRUCT, vec![this, right]));
        assert!(log.contains(&(STRING_GROW, vec![this, 5, 0])));
    }

    #[test]
    fn string_destruct_frees_only_a_heap_buffer() {
        let mut e = string_engine();
        let inline = new_string(&mut e, b"ab");
        start_log(&mut e);
        e.call(0x0044a540, &[inline]);
        assert_eq!(calls(&mut e), vec![(STRING_SET_LENGTH, vec![inline, 0])]);
        assert_eq!(e.mem.u32(inline + 0x14), 0);

        let heap_string = e.mem.alloc(0x1c);
        let heap = c_string(&mut e, b"a long string");
        e.mem.set_u32(heap_string + 4, heap);
        e.mem.set_u32(heap_string + 0x14, 13);
        e.mem.set_u32(heap_string + 0x18, 0x20);
        start_log(&mut e);
        e.call(0x0044a540, &[heap_string]);
        let log = calls(&mut e);
        assert_eq!(log[0], (STRING_FREE, vec![heap_string, heap, 0x21]));
        assert_eq!(e.mem.u32(heap_string + 0x18), 0xf);
    }

    #[test]
    fn string_c_str_is_the_data_pointer() {
        let mut e = string_engine();
        let string = new_string(&mut e, b"x");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a560, &[string]).u32(), string + 4);
        assert_eq!(calls(&mut e), vec![(STRING_DATA, vec![string])]);
    }

    #[test]
    fn string_assign_substring_clamps_copies_and_handles_self_assignment() {
        let mut e = string_engine();
        let right = new_string(&mut e, b"abcdefg");
        let this = new_string(&mut e, b"");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a580, &[this, right, 2, 3]).u32(), this);
        assert_eq!(string_text(&e, this), b"cde");
        assert!(calls(&mut e).contains(&(STRING_GROW, vec![this, 3, 0])));

        // The count is clamped to what is left after the offset.
        e.call(0x0044a580, &[this, right, 5, u32::MAX]);
        assert_eq!(string_text(&e, this), b"fg");

        // An offset past the end throws first (and the game never returns).
        let other = new_string(&mut e, b"");
        start_log(&mut e);
        e.call(0x0044a580, &[other, right, 8, 1]);
        assert!(calls(&mut e).contains(&(STRING_OUT_OF_RANGE, vec![])));

        // Self-assignment erases the tail, then the head.
        let same = new_string(&mut e, b"abcdef");
        e.call(0x0044a580, &[same, same, 1, 3]);
        assert_eq!(string_text(&e, same), b"bcd");

        // A refused grow leaves the string alone.
        let kept = new_string(&mut e, b"keep");
        e.register(STRING_GROW, |_, _| ret(0));
        e.call(0x0044a580, &[kept, right, 0, 4]);
        assert_eq!(string_text(&e, kept), b"keep");
    }

    #[test]
    fn string_assign_c_str_measures_and_assigns() {
        let mut e = string_engine();
        let this = new_string(&mut e, b"");
        let text = c_string(&mut e, b"xyz");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a640, &[this, text]).u32(), this);
        assert_eq!(string_text(&e, this), b"xyz");
        assert_eq!(calls(&mut e)[0], (STRLEN, vec![text]));
    }

    #[test]
    fn string_length_wrapper_calls_strlen() {
        let mut e = string_engine();
        let text = c_string(&mut e, b"four");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a670, &[text]).u32(), 4);
        assert_eq!(calls(&mut e), vec![(STRLEN, vec![text])]);
    }

    #[test]
    fn string_erase_moves_the_tail_and_validates() {
        let mut e = string_engine();
        let string = new_string(&mut e, b"abcdef");
        start_log(&mut e);
        assert_eq!(e.call(0x0044a690, &[string, 1, 2]).u32(), string);
        assert_eq!(string_text(&e, string), b"adef");
        let log = calls(&mut e);
        // (destination, capacity left, source, tail length)
        assert!(log.contains(&(MOVE_CHARS, vec![string, string + 5, 14, string + 7, 3])));

        // A count of 0 changes nothing; npos erases to the end.
        e.call(0x0044a690, &[string, 1, 0]);
        assert_eq!(string_text(&e, string), b"adef");
        e.call(0x0044a690, &[string, 2, u32::MAX]);
        assert_eq!(string_text(&e, string), b"ad");

        // A position past the end throws first (the game never returns); the
        // copy double is made harmless for the nonsense length that follows.
        e.register(MOVE_CHARS, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0044a690, &[string, 3, 1]);
        assert_eq!(calls(&mut e)[0], (STRING_OUT_OF_RANGE, vec![]));
    }

    #[test]
    fn string_tidy_returns_to_the_inline_buffer() {
        let mut e = string_engine();
        let string = e.mem.alloc(0x1c);
        let heap = c_string(&mut e, b"wxyz and more text");
        e.mem.set_u32(string + 4, heap);
        e.mem.set_u32(string + 0x14, 18);
        e.mem.set_u32(string + 0x18, 0x20);
        start_log(&mut e);
        e.call(0x0044a730, &[string, 1, 4]);
        let log = calls(&mut e);
        assert_eq!(
            log,
            vec![
                (COPY_CHARS, vec![string + 4, 0x10, heap, 4]),
                (STRING_FREE, vec![string, heap, 0x21]),
                (STRING_SET_LENGTH, vec![string, 4]),
            ]
        );
        assert_eq!(string_text(&e, string), b"wxyz");
        assert_eq!(e.mem.u32(string + 0x18), 0xf);

        // Not built: the reserve and length are reset without touching memory.
        let fresh = e.mem.alloc(0x1c);
        start_log(&mut e);
        e.call(0x0044a730, &[fresh, 0, 0]);
        assert_eq!(calls(&mut e), vec![(STRING_SET_LENGTH, vec![fresh, 0])]);
        assert_eq!(e.mem.u32(fresh + 0x18), 0xf);
    }

    #[test]
    fn string_proxy_constructors_pass_the_allocator() {
        let mut e = string_engine();
        let this = e.mem.alloc(0x1c);
        start_log(&mut e);
        assert_eq!(e.call(0x0044a7b0, &[this, 0x1234]).u32(), this);
        assert_eq!(e.global::<u32>(SCRATCH), 0x1234);
        assert_eq!(calls(&mut e)[0].1[0], this);

        let allocator = 0x4321_0000;
        start_log(&mut e);
        e.register(ALLOCATOR_PROXY_CONSTRUCT, |_, a| ret(a[0]));
        assert_eq!(e.call(0x0044a7d0, &[this, allocator]).u32(), this);
        assert_eq!(
            calls(&mut e),
            vec![(ALLOCATOR_PROXY_CONSTRUCT, vec![this, allocator])]
        );
    }

    #[test]
    fn string_assign_chars_handles_self_overlap_and_copies() {
        let mut e = string_engine();
        // `text` points into the string: assigns that range of itself.
        let own = new_string(&mut e, b"abcdef");
        assert_eq!(e.call(0x0044a7f0, &[own, own + 4 + 2, 3]).u32(), own);
        assert_eq!(string_text(&e, own), b"cde");

        // Outside the string: grow, then copy.
        let this = new_string(&mut e, b"");
        let text = c_string(&mut e, b"xy");
        start_log(&mut e);
        e.call(0x0044a7f0, &[this, text, 2]);
        assert_eq!(string_text(&e, this), b"xy");
        let log = calls(&mut e);
        assert_eq!(
            addresses(&log)[..2].to_vec(),
            vec![STRING_INSIDE, STRING_GROW]
        );
        assert!(log.contains(&(COPY_CHARS, vec![this + 4, 0xf, text, 2])));

        // A refused grow returns `this` unchanged.
        e.register(STRING_GROW, |_, _| ret(0));
        assert_eq!(e.call(0x0044a7f0, &[this, text, 1]).u32(), this);
        assert_eq!(string_text(&e, this), b"xy");
    }

    // ----- TESObject and the bound classes -----

    fn object_engine() -> Engine {
        let mut e = engine();
        e.register(TES_FORM_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TES_FORM_DESTRUCT, |_, _| ret(0));
        e.register(LIST_REMOVE, |_, _| ret(0));
        e.register(OPERATOR_DELETE, |_, _| ret(0));
        e.register(ALLOCATOR_CONSTRUCT, |_, a| ret(a[0]));
        e
    }

    fn set_point(e: &mut Engine, point: u32, x: i16, y: i16, z: i16) {
        e.mem.set_i16(point, x);
        e.mem.set_i16(point + 2, y);
        e.mem.set_i16(point + 4, z);
    }

    fn point(e: &Engine, point: u32) -> (i16, i16, i16) {
        (e.mem.i16(point), e.mem.i16(point + 2), e.mem.i16(point + 4))
    }

    #[test]
    fn bound_object_constructor_copies_the_default_bound() {
        let mut e = object_engine();
        let object = e.mem.alloc(0x30);
        e.mem.write(object, &[0xa5; 0x30]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050e820, &[object]).u32(), object);
        assert_eq!(e.mem.u32(object), TES_BOUND_OBJECT_VTABLE);
        for offset in [0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(point(&e, object + 0x24), (0, 0, 0));
        assert_eq!(point(&e, object + 0x2a), (0, 0, 0));
        assert_eq!(e.global::<u32>(DEFAULT_BOUND_GUARD) & 1, 1);
        let log = calls(&mut e);
        assert_eq!(log[0], (TES_FORM_CONSTRUCT, vec![object]));
        assert!(log.contains(&(ALLOCATOR_CONSTRUCT, vec![object + 0x24])));

        // With the default already built, both points take its value.
        set_point(&mut e, DEFAULT_BOUND, 1, -2, 3);
        let second = e.mem.alloc(0x30);
        e.call(0x0050e820, &[second]);
        assert_eq!(point(&e, second + 0x24), (1, -2, 3));
        assert_eq!(point(&e, second + 0x2a), (1, -2, 3));
    }

    #[test]
    fn point_copy_moves_three_words_only() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        set_point(&mut e, source, -1, 2, -3);
        let this = e.mem.alloc(16);
        e.mem.write(this, &[0xa5; 16]);
        assert_eq!(e.call(0x0050e8b0, &[this, source]).u32(), this);
        assert_eq!(point(&e, this), (-1, 2, -3));
        assert_eq!(e.mem.bytes(this + 6, 10), vec![0xa5; 10]);
    }

    #[test]
    fn default_bound_is_built_once() {
        let mut e = engine();
        e.mem.write(DEFAULT_BOUND, &[0x55; 6]);
        e.set_global(DEFAULT_BOUND_GUARD, 0x10u32);
        assert_eq!(e.call(0x0050e8f0, &[]).u32(), DEFAULT_BOUND);
        assert_eq!(point(&e, DEFAULT_BOUND), (0, 0, 0));
        assert_eq!(e.global::<u32>(DEFAULT_BOUND_GUARD), 0x11);

        set_point(&mut e, DEFAULT_BOUND, 7, 8, 9);
        assert_eq!(e.call(0x0050e8f0, &[]).u32(), DEFAULT_BOUND);
        assert_eq!(point(&e, DEFAULT_BOUND), (7, 8, 9));
    }

    #[test]
    fn point_constructor_keeps_the_low_sixteen_bits() {
        let mut e = engine();
        let this = e.mem.alloc(16);
        e.mem.write(this, &[0xa5; 16]);
        assert_eq!(
            e.call(0x0050e960, &[this, 0x0001_0002, 0xffff, 5]).u32(),
            this
        );
        assert_eq!(point(&e, this), (2, -1, 5));
        assert_eq!(e.mem.bytes(this + 6, 10), vec![0xa5; 10]);
    }

    #[test]
    fn bound_object_deleting_destructor_deletes_on_flag() {
        let mut e = object_engine();
        let object = e.mem.alloc(0x30);
        let list = e.mem.alloc(16);
        e.mem.set_u32(object + 0x18, list);
        start_log(&mut e);
        assert_eq!(e.call(0x0050e990, &[object, 0]).u32(), object);
        assert_eq!(e.mem.u32(object), TES_OBJECT_VTABLE);
        let log = calls(&mut e);
        assert_eq!(
            log,
            vec![
                (LIST_REMOVE, vec![list, object]),
                (TES_FORM_DESTRUCT, vec![object]),
            ]
        );
        start_log(&mut e);
        e.call(0x0050e990, &[object, 3]);
        let log = calls(&mut e);
        assert_eq!(log.last().unwrap(), &(OPERATOR_DELETE, vec![object]));
    }

    #[test]
    fn bound_data_constructor_runs_both_point_constructors() {
        let mut e = object_engine();
        let data = e.mem.alloc(16);
        start_log(&mut e);
        assert_eq!(e.call(0x0050e9c0, &[data]).u32(), data);
        assert_eq!(
            calls(&mut e),
            vec![
                (ALLOCATOR_CONSTRUCT, vec![data]),
                (ALLOCATOR_CONSTRUCT, vec![data + 6]),
            ]
        );
    }

    #[test]
    fn bound_object_destructor_body_ends_at_the_object_base() {
        let mut e = object_engine();
        let object = e.mem.alloc(0x30);
        start_log(&mut e);
        e.call(0x0050e9f0, &[object]);
        assert_eq!(e.mem.u32(object), TES_OBJECT_VTABLE);
        assert_eq!(calls(&mut e), vec![(TES_FORM_DESTRUCT, vec![object])]);
    }

    #[test]
    fn point_difference_and_sum_copy_words_untouched() {
        let mut e = engine();
        e.register(POINT3_SUBTRACT, |e, a| {
            e.mem.set_f32(a[1], 1.5);
            e.mem.set_u32(a[1] + 4, 0x7fa0_0001);
            e.mem.set_f32(a[1] + 8, 4.0);
            ret(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            e.mem.set_f32(a[1], -1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            ret(a[1])
        });
        let object = e.mem.alloc(0x30);
        let result = e.mem.alloc(16);
        start_log(&mut e);
        e.call(0x0050ea10, &[object, result]);
        assert_eq!(e.mem.f32(result), 1.5);
        assert_eq!(e.mem.u32(result + 4), 0x7fa0_0001);
        assert_eq!(e.mem.f32(result + 8), 4.0);
        let log = calls(&mut e);
        assert_eq!(log[0].0, POINT3_SUBTRACT);
        assert_eq!(log[0].1[0], object + 0xc);
        assert_eq!(log[0].1[2], object + 0x18);

        start_log(&mut e);
        e.call(0x0050ea50, &[object, result]);
        assert_eq!(
            (
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ),
            (-1.0, 2.0, 3.0)
        );
        let log = calls(&mut e);
        assert_eq!(log[0].0, POINT3_ADD);
        assert_eq!(log[0].1[0], object + 0xc);
        assert_eq!(log[0].1[2], object + 0x18);
    }

    #[test]
    fn global_getters_read_their_words() {
        let mut e = engine();
        e.set_global(0x0120_31e4, 0x1111u32);
        e.set_global(0x011c_6218, 0x2222u32);
        e.set_global(0x011f_81dc, 0x3333u32);
        assert_eq!(e.call(0x0050ea90, &[]).u32(), 0x1111);
        assert_eq!(e.call(0x0050f5c0, &[]).u32(), 0x2222);
        assert_eq!(e.call(0x0050f6e0, &[]).u32(), 0x3333);
    }

    #[test]
    fn bound_copy_passes_the_twelve_bytes_to_memcpy() {
        let mut e = engine();
        e.register(MEMORY_COPY, |_, a| ret(a[0]));
        let (this, source) = (0x1000_0000, 0x2000_0000);
        start_log(&mut e);
        assert_eq!(e.call(0x0050eaa0, &[this, source]).u32(), this + 0x24);
        assert_eq!(
            calls(&mut e),
            vec![(MEMORY_COPY, vec![this + 0x24, source + 0x24, 12])]
        );
    }

    #[test]
    fn bound_corners_become_point3_floats() {
        let mut e = engine();
        e.register(POINT3_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            ret(a[0])
        });
        let object = e.mem.alloc(0x30);
        set_point(&mut e, object + 0x24, -3, 4, 5);
        set_point(&mut e, object + 0x2a, 7, -8, 9);
        let result = e.mem.alloc(16);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ead0, &[object, result]).u32(), result);
        assert_eq!(
            (
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ),
            (-3.0, 4.0, 5.0)
        );
        let log = calls(&mut e);
        assert_eq!(
            log[0].1[1..].to_vec(),
            vec![(-3.0f32).to_bits(), 4.0f32.to_bits(), 5.0f32.to_bits()]
        );
        assert_eq!(e.call(0x0050eb90, &[object, result]).u32(), result);
        assert_eq!(
            (
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ),
            (7.0, -8.0, 9.0)
        );
    }

    #[test]
    fn point_components_convert_with_sign() {
        let mut e = engine();
        let point_address = e.mem.alloc(8);
        set_point(&mut e, point_address, -32768, 32767, -2);
        assert_eq!(e.call(0x0050eb30, &[point_address]).f32(), -32768.0);
        assert_eq!(e.call(0x0050eb50, &[point_address]).f32(), 32767.0);
        assert_eq!(e.call(0x0050eb70, &[point_address]).f32(), -2.0);
    }

    #[test]
    fn bound_size_is_half_the_diagonal() {
        let mut e = engine();
        e.register(SQUARE_ROOT, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).sqrt()))
        });
        let object = e.mem.alloc(0x30);
        set_point(&mut e, object + 0x24, 10, 20, 30);
        set_point(&mut e, object + 0x2a, 4, 8, 12);
        start_log(&mut e);
        let size = e.call(0x0050ebf0, &[object]).f32();
        let expected = (f64::from(504.0f32.sqrt()) * 0.5) as f32;
        assert_eq!(size, expected);
        // 6, 12 and 18 squared: 36 + 144 + 324.
        assert_eq!(calls(&mut e), vec![(SQUARE_ROOT, vec![504.0f32.to_bits()])]);
    }

    #[test]
    fn point_difference_truncates_to_sixteen_bits() {
        let mut e = engine();
        let (first, second, result) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        set_point(&mut e, first, 30000, -5, 7);
        set_point(&mut e, second, -30000, 5, 9);
        assert_eq!(e.call(0x0050ec30, &[first, result, second]).u32(), result);
        assert_eq!(point(&e, result), (-5536, -10, -2));
    }

    #[test]
    fn point_length_squares_in_thirty_two_bits() {
        let mut e = engine();
        e.register(SQUARE_ROOT, |e, a| {
            e.set_global(SCRATCH, a[0]);
            float_ret(7.5)
        });
        let point_address = e.mem.alloc(8);
        set_point(&mut e, point_address, 3, 4, 0);
        assert_eq!(e.call(0x0050ec80, &[point_address]).f64(), 7.5);
        assert_eq!(e.global::<u32>(SCRATCH), 25.0f32.to_bits());

        // Three times 32767 squared overflows the signed sum.
        set_point(&mut e, point_address, 32767, 32767, 32767);
        e.call(0x0050ec80, &[point_address]);
        assert_eq!(
            e.global::<u32>(SCRATCH),
            (-1_073_938_429i32 as f32).to_bits()
        );
    }

    #[test]
    fn bound_save_swaps_around_the_chunk_write() {
        let mut e = engine();
        e.register(BYTE_SWAP_FLAG_GETTER, |e, _| ret(e.global(SCRATCH_COUNT)));
        e.register(BYTE_SWAP_WORD, |e, a| {
            let value = e.mem.u16(a[0]);
            e.mem.set_u16(a[0], value.swap_bytes());
            ret(0)
        });
        e.register(ADD_CHUNK_DATA, |e, a| {
            e.set_global(SCRATCH, e.mem.u16(a[1]));
            ret(0)
        });
        let object = e.mem.alloc(0x30);
        e.mem.set_u16(object + 0x24, 0x1234);
        e.set_global(SCRATCH_COUNT, 1u32);
        start_log(&mut e);
        e.call(0x0050ece0, &[object]);
        // The writer saw the swapped word, and it is swapped back after.
        assert_eq!(e.global::<u32>(SCRATCH), 0x3412);
        assert_eq!(e.mem.u16(object + 0x24), 0x1234);
        let log = calls(&mut e);
        assert_eq!(log.len(), 1 + 6 + 1 + 1 + 6);
        assert_eq!(
            log[7],
            (ADD_CHUNK_DATA, vec![OBND_CHUNK, object + 0x24, 12])
        );

        e.set_global(SCRATCH_COUNT, 0u32);
        start_log(&mut e);
        e.call(0x0050ece0, &[object]);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![BYTE_SWAP_FLAG_GETTER, ADD_CHUNK_DATA, BYTE_SWAP_FLAG_GETTER]
        );
    }

    #[test]
    fn bound_swap_goes_through_the_six_words_in_order() {
        let mut e = engine();
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        let data = e.mem.alloc(16);
        start_log(&mut e);
        e.call(0x0050ed30, &[data]);
        let expected: Vec<(u32, Vec<u32>)> = [6, 8, 10, 0, 2, 4]
            .iter()
            .map(|offset| (BYTE_SWAP_WORD, vec![data + offset, 0]))
            .collect();
        assert_eq!(calls(&mut e), expected);
    }

    #[test]
    fn bound_load_reads_the_obnd_chunk_only() {
        let mut e = engine();
        e.register(FILE_CURRENT_CHUNK, |e, _| ret(e.global(SCRATCH_TYPE)));
        e.register(FILE_READ_CHUNK, |_, _| ret(1));
        e.register(FILE_FLAG_GETTER, |e, _| ret(e.global(SCRATCH_COUNT)));
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        let object = e.mem.alloc(0x30);
        let file = 0x5000_0000;
        start_log(&mut e);
        e.call(0x0050edd0, &[object, 0]);
        assert!(calls(&mut e).is_empty());

        e.set_global(SCRATCH_TYPE, 0x1234u32);
        start_log(&mut e);
        e.call(0x0050edd0, &[object, file]);
        assert_eq!(calls(&mut e), vec![(FILE_CURRENT_CHUNK, vec![file])]);

        e.set_global(SCRATCH_TYPE, OBND_CHUNK);
        e.set_global(SCRATCH_COUNT, 0u32);
        start_log(&mut e);
        e.call(0x0050edd0, &[object, file]);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![FILE_CURRENT_CHUNK, FILE_READ_CHUNK, FILE_FLAG_GETTER]
        );

        e.set_global(SCRATCH_COUNT, 1u32);
        start_log(&mut e);
        e.call(0x0050edd0, &[object, file]);
        let log = calls(&mut e);
        assert_eq!(log[1], (FILE_READ_CHUNK, vec![file, object + 0x24, 12]));
        assert_eq!(log.len(), 3 + 6);
    }

    #[test]
    fn clone_wrapper_calls_the_clone_slot_with_no_flag() {
        let mut e = engine();
        e.register(0x7000_014c, |_, _| ret(0xabc));
        let object = object_with_vtable(&mut e, 0x30, &[(0x14c, 0x7000_014c)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee20, &[object, 0x77]).u32(), 0xabc);
        assert_eq!(calls(&mut e), vec![(0x7000_014c, vec![object, 0x77, 0])]);
    }

    // ----- Clone3D -----

    /// Everything `Clone3D` calls, as do-nothing doubles; tests override.
    fn clone_engine() -> Engine {
        let mut e = object_engine();
        stub(
            &mut e,
            &[
                0x0044_4ed0,
                0x007a_f430,
                FORM_TYPE,
                0x0050_d4a0,
                0x0069_3ef0,
                0x0072_6070,
                0x004b_5bf0,
                0x0045_bc00,
                0x0043_b4a0,
                0x0043_b230,
                0x0065_3270,
                0x0047_a520,
                0x0062_c3c0,
                0x0044_8a80,
                0x00a5_bdd0,
                0x0044_8a40,
                0x0048_b310,
                0x0056_9410,
                0x0055_d520,
                PRINT_MESSAGE,
                0x004a_d050,
                0x0045_7ba0,
                0x00aa_13e0,
                0x00a5_ecb0,
                0x00a5_d2c0,
                0x0044_0490,
                0x0050_fdf0,
                0x0063_3c90,
                0x0044_72a0,
                0x0055_9450,
                OPERATOR_NEW,
                0x0043_abf0,
                0x0044_aed0,
                0x0092_c870,
                0x0044_7010,
                0x0044_31f0,
                0x004a_d270,
                0x0040_c110,
                0x0045_0f90,
                0x0043_fa80,
                0x0044_0460,
                0x0096_11e0,
            ],
        );
        e.register(0x0056_7400, |_, _| float_ret(1.0));
        e.register(0x009c_dae0, |_, _| float_ret(1.0));
        e
    }

    /// The addresses of the clone calls, as a sequence.
    fn clone_addresses(e: &mut Engine) -> Vec<u32> {
        addresses(&calls(e))
    }

    #[test]
    fn clone_without_a_source_model_returns_null() {
        let mut e = clone_engine();
        e.register(0x7000_0174, |_, _| ret(0));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 0]).u32(), 0);
        assert_eq!(calls(&mut e), vec![(0x7000_0174, vec![object, 0])]);
    }

    #[test]
    fn clone_drops_the_reference_for_the_special_forms() {
        let mut e = clone_engine();
        e.register(0x7000_0174, |_, _| ret(0));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        e.set_global(SPECIAL_FORM_B, object);
        start_log(&mut e);
        e.call(0x0050ee50, &[object, 0x4000, 0]);
        assert_eq!(calls(&mut e), vec![(0x7000_0174, vec![object, 0])]);
    }

    #[test]
    fn clone_returns_the_model_when_the_owner_is_not_type_1e() {
        let mut e = clone_engine();
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x0044_4ed0, |_, _| ret(1));
        e.register(0x007a_f430, |_, _| ret(0x2000));
        e.register(FORM_TYPE, |_, _| ret(0x20));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0x4000, 0]).u32(), 0x3000);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![0x7000_0174, 0x0044_4ed0, 0x007a_f430, FORM_TYPE]
        );
    }

    #[test]
    fn clone_makes_a_plain_copy_and_finishes_it() {
        let mut e = clone_engine();
        let clone = e.mem.alloc(0x30);
        e.mem.set_u32(clone + 0x18, 0xdead);
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x0072_6070, |_, _| ret(2));
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        e.register(0x0096_11e0, |_, _| ret(1));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 0]).u32(), clone);
        let log = calls(&mut e);
        assert_eq!(
            addresses(&log),
            vec![
                0x7000_0174,
                0x0050_d4a0,
                0x0072_6070,
                0x009c_dae0,
                0x004a_d050,
                0x00a5_d2c0,
                0x004a_d270,
                0x0044_0490,
                0x0045_0f90,
                0x0043_fa80,
                0x0044_0460,
                0x0096_11e0,
            ]
        );
        assert_eq!(log[4].1[1], 1.0f32.to_bits());
        assert_eq!(log[5].1[0], 0x3000);
        assert_eq!(log[8], (0x0045_0f90, vec![clone, 0]));
        assert_eq!(log[9], (0x0043_fa80, vec![clone, CLONE_NAME_TABLE]));
        assert_eq!(log[10], (0x0044_0460, vec![clone, CLONE_DATA_TABLE]));
        // The final fix-up clears the parent list.
        assert_eq!(e.mem.u32(clone + 0x18), 0);
    }

    #[test]
    fn clone_shares_the_model_when_every_check_passes() {
        let mut e = clone_engine();
        let model = e.mem.alloc(0x30);
        e.mem.set_u32(model + 0x18, 0xdead);
        e.register_double(0x7000_0174, move |_, _| ret(model));
        e.register(0x0072_6070, |_, _| ret(1));
        e.register(FORM_TYPE, |_, _| ret(0x10));
        e.register(0x0062_c3c0, |_, _| ret(0));
        e.register(0x0044_8a80, |_, _| ret(7));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 0]).u32(), model);
        let sequence = clone_addresses(&mut e);
        assert!(!sequence.contains(&0x00a5_d2c0));
        assert!(sequence.contains(&0x0062_c3c0));
        assert!(sequence.contains(&0x0056_9410));
        assert_eq!(e.mem.u32(model + 0x18), 0xdead);

        // Any failed check (here the emittance property) forces a copy.
        e.register(0x0056_9410, |_, _| ret(1));
        let clone = e.mem.alloc(0x30);
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 0]).u32(), clone);
        assert!(clone_addresses(&mut e).contains(&0x00a5_d2c0));
    }

    #[test]
    fn clone_with_a_scale_looks_up_and_registers_the_scaled_model() {
        let mut e = clone_engine();
        let (first, second) = (e.mem.alloc(0x30), e.mem.alloc(0x30));
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x0072_6070, |_, _| ret(2));
        e.register(0x0056_7400, |_, _| float_ret(2.0));
        e.register_double(0x00a5_d2c0, {
            let mut results = vec![second, first];
            move |_, _| ret(results.pop().unwrap())
        });
        e.register(OPERATOR_NEW, |e, _| ret(e.mem.alloc(16)));
        e.register(0x0043_abf0, |_, _| ret(0x6000));
        e.register(0x0044_7010, |_, _| ret(1));
        e.set_global(COUNTER_A, 5u32);
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0x4000, 0]).u32(), second);
        let log = calls(&mut e);
        assert_eq!(
            addresses(&log),
            vec![
                0x7000_0174,
                0x0044_4ed0,
                0x0050_d4a0,
                0x0072_6070,
                0x0056_7400,
                0x009c_dae0,
                0x004a_d050,
                0x00a5_d2c0,
                0x009c_dae0,
                0x0044_0490,
                0x0050_fdf0,
                0x0063_3c90,
                0x0044_72a0,
                0x0055_9450,
                OPERATOR_NEW,
                0x0043_abf0,
                0x0044_aed0,
                0x0055_9450,
                0x0092_c870,
                0x0055_9450,
                0x0044_7010,
                0x004a_d050,
                0x00a5_d2c0,
                0x004a_d270,
                0x0040_c110,
                0x004a_d270,
                0x0044_0490,
                0x0045_0f90,
                0x0043_fa80,
                0x0044_0460,
                0x0096_11e0,
            ]
        );
        // The first process is built with the reference's scale.
        assert_eq!(log[6].1[1], 2.0f32.to_bits());
        assert_eq!(log[9], (0x0044_0490, vec![first, 2.0f32.to_bits()]));
        assert_eq!(e.global::<u32>(COUNTER_A), 6);
    }

    #[test]
    fn clone_prints_a_message_for_a_scale_of_zero() {
        let mut e = clone_engine();
        let clone = e.mem.alloc(0x30);
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x0072_6070, |_, _| ret(2));
        e.register(0x0056_7400, |_, _| float_ret(0.0));
        e.register(0x009c_dae0, |_, _| float_ret(5.0));
        e.register(0x0055_d520, |_, _| ret(0xaaa));
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0x4000, 0]).u32(), clone);
        let log = calls(&mut e);
        assert!(log.contains(&(PRINT_MESSAGE, vec![SCALE_ZERO_MESSAGE, 0xaaa])));
        let process = log.iter().find(|call| call.0 == 0x004a_d050).unwrap();
        assert_eq!(process.1[1], 1.0f32.to_bits());
    }

    #[test]
    fn clone_with_a_nan_scale_takes_the_rescale_path() {
        let mut e = clone_engine();
        let clone = e.mem.alloc(0x30);
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x0072_6070, |_, _| ret(2));
        e.register(0x0056_7400, |_, _| float_ret(f64::from(f32::NAN)));
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        e.register(OPERATOR_NEW, |e, _| ret(e.mem.alloc(16)));
        e.register(0x0044_7010, |_, _| ret(1));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        e.call(0x0050ee50, &[object, 0x4000, 0]);
        let log = calls(&mut e);
        let process = log.iter().find(|call| call.0 == 0x004a_d050).unwrap();
        assert!(f32::from_bits(process.1[1]).is_nan());
        // NaN is neither equal to the model's scale nor to 1.0.
        assert!(log.iter().any(|call| call.0 == 0x0050_fdf0));
    }

    #[test]
    fn clone_uses_an_existing_model_of_the_right_scale() {
        let mut e = clone_engine();
        let clone = e.mem.alloc(0x30);
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x7000_014c, |_, _| ret(0x9999));
        e.register(0x0072_6070, |_, _| ret(2));
        e.register(0x0056_7400, |_, _| float_ret(2.0));
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        e.register(0x0055_9450, |_, _| ret(0x6000));
        e.register(0x0043_b230, |_, _| ret(0x7000));
        // The model's scale, the source model's again (0050f25c: not the clone), then the found
        // model's.
        e.register_double(0x009c_dae0, {
            let mut results = vec![2.0, 1.0, 1.0];
            move |_, _| float_ret(results.pop().unwrap())
        });
        let object =
            object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174), (0x14c, 0x7000_014c)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0x4000, 0]).u32(), 0x9999);
        let log = calls(&mut e);
        assert!(log.contains(&(0x7000_014c, vec![object, 0x4000, 0])));
        // The scale test after cloning reads the source model (0x3000), not the clone.
        let scaled: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == 0x009c_dae0)
            .map(|call| call.1[0])
            .collect();
        assert_eq!(scaled.get(1), Some(&0x3000));
        assert_eq!(log[log.len() - 2].0, 0x0040_c110);
        assert_eq!(log[log.len() - 1].0, 0x004a_d270);
        assert!(!log.iter().any(|call| call.0 == 0x0044_0460));

        // A found model of another scale prints a message and the copy goes on.
        e.register_double(0x009c_dae0, {
            let mut results = vec![3.0, 1.0, 1.0];
            move |_, _| float_ret(results.pop().unwrap())
        });
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0x4000, 0]).u32(), clone);
        let log = calls(&mut e);
        let message = log.iter().find(|call| call.0 == PRINT_MESSAGE).unwrap();
        assert_eq!(message.1[0], SCALE_INVALID_MESSAGE);
    }

    #[test]
    fn clone_deep_copy_uses_the_helper_or_builds_a_node() {
        let mut e = clone_engine();
        e.set_global(CLONE_HELPER_POINTER, 0x1111u32);
        e.register(0x7000_0174, |_, _| ret(0x3000));
        e.register(0x7000_000c, |_, _| ret(0x5555));
        e.register(0x7000_000d, |_, _| ret(0));
        e.register(0x7000_00dc, |_, _| ret(0));
        let copy = object_with_vtable(&mut e, 0x20, &[(0xc, 0x7000_000c)]);
        e.register_double(0x0045_7ba0, move |_, _| ret(copy));
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 1]).u32(), 0x5555);
        let log = calls(&mut e);
        // (helper, source model, cloning process)
        assert_eq!(log[4].0, 0x0045_7ba0);
        assert_eq!(log[4].1[..2].to_vec(), vec![0x1111, 0x3000]);
        assert!(!addresses(&log).contains(&0x00a5_d2c0));

        // A helper copy that cannot clone itself is adopted by a new node.
        let bare = object_with_vtable(&mut e, 0x20, &[(0xc, 0x7000_000d)]);
        e.register_double(0x0045_7ba0, move |_, _| ret(bare));
        e.register(0x00aa_13e0, |e, _| ret(e.mem.alloc(0xac)));
        let node = object_with_vtable(&mut e, 0xac, &[(0xdc, 0x7000_00dc)]);
        e.register_double(0x00a5_ecb0, move |_, _| ret(node));
        start_log(&mut e);
        assert_eq!(e.call(0x0050ee50, &[object, 0, 1]).u32(), node);
        let log = calls(&mut e);
        assert!(log.contains(&(0x7000_00dc, vec![node, bare, 1])));
    }

    // ----- the rest of the unit -----

    #[test]
    fn small_setters_write_their_fields() {
        let mut e = engine();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x18, 5);
        e.call(0x0050f5a0, &[object]);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        e.call(0x0050f9a0, &[object, 0x1234]);
        assert_eq!(e.mem.u32(object + 0x1c), 0x1234);
    }

    #[test]
    fn model_path_virtual_gets_the_form_model_path() {
        let mut e = engine();
        e.register(FORM_MODEL_PATH, |_, _| ret(0x4444));
        e.register(0x7000_017c, |_, _| ret(0x77));
        let object = object_with_vtable(&mut e, 0x30, &[(0x17c, 0x7000_017c)]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050f5d0, &[object]).u32(), 0x77);
        assert_eq!(
            calls(&mut e),
            vec![
                (FORM_MODEL_PATH, vec![object]),
                (0x7000_017c, vec![object, 0x4444]),
            ]
        );
    }

    #[test]
    fn model_registration_sets_the_node_origin() {
        let mut e = engine();
        let node = e.mem.alloc(0x40);
        e.set_global(MODEL_LOADER_POINTER, 0x1234u32);
        e.register(FORM_MODEL_PATH, |_, _| ret(0x4444));
        e.register_double(0x7000_0174, move |_, _| ret(node));
        stub(
            &mut e,
            &[0x0045_a5e0, 0x00a5_a040, 0x0043_d410, 0x00a5_9c60],
        );
        let object = object_with_vtable(&mut e, 0x30, &[(0x174, 0x7000_0174)]);
        start_log(&mut e);
        assert!(e.call(0x0050f600, &[object, 0x99]).bool());
        let log = calls(&mut e);
        assert_eq!(
            addresses(&log),
            vec![
                FORM_MODEL_PATH,
                0x0045_a5e0,
                0x7000_0174,
                0x00a5_a040,
                0x0043_d410,
                0x00a5_9c60,
            ]
        );
        assert_eq!(log[1].1, vec![0x1234, 0x4444]);
        assert_eq!(log[2].1, vec![object, 0]);
        let origin = log[4].1[0];
        assert_eq!(log[4].1[1..].to_vec(), vec![0, 0, 0]);
        assert_eq!(log[5].1, vec![node, origin]);
        // The wrapper forwards its result.
        assert!(e.call(0x0050f760, &[object, 0x99]).bool());
    }

    #[test]
    fn cast_and_call_needs_both_pointers() {
        let mut e = engine();
        e.register(0x7000_03d0, |_, _| ret(0));
        let target = object_with_vtable(&mut e, 0x20, &[(0x3d0, 0x7000_03d0)]);
        e.register_double(0x00ec_43fb, move |e, _| ret(e.global(SCRATCH_COUNT)));
        e.set_global(SCRATCH_COUNT, target);
        start_log(&mut e);
        assert!(e
            .call(0x0050f670, &[0, 0x10, 0x20, 0x1ff, 0x77, 0x30])
            .bool());
        let log = calls(&mut e);
        assert_eq!(
            log[0],
            (0x00ec_43fb, vec![0x20, 0, 0x0118_41cc, 0x0118_46d4, 0])
        );
        assert_eq!(log[1], (0x7000_03d0, vec![target, 0x10, 0x30, 0xff]));

        // A null target or a failed cast answers false without the call.
        start_log(&mut e);
        assert!(!e.call(0x0050f670, &[0, 0, 0x20, 1, 0, 0]).bool());
        assert_eq!(calls(&mut e).len(), 1);
        e.set_global(SCRATCH_COUNT, 0u32);
        start_log(&mut e);
        assert!(!e.call(0x0050f670, &[0, 0x10, 0x20, 1, 0, 0]).bool());
        assert_eq!(calls(&mut e).len(), 1);
    }

    #[test]
    fn anim_and_tree_objects_set_their_vtables() {
        let mut e = object_engine();
        let anim = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0050f6f0, &[anim]).u32(), anim);
        assert_eq!(e.mem.u32(anim), TES_BOUND_ANIM_OBJECT_VTABLE);
        let tree = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0050f7a0, &[tree]).u32(), tree);
        assert_eq!(e.mem.u32(tree), TES_BOUND_TREE_OBJECT_VTABLE);
    }

    #[test]
    fn anim_and_tree_destructors_end_at_the_object_base() {
        let mut e = object_engine();
        for (body, deleting) in [(0x0050f740, 0x0050f710), (0x0050f7f0, 0x0050f7c0)] {
            let object = e.mem.alloc(0x30);
            start_log(&mut e);
            e.call(body, &[object]);
            assert_eq!(e.mem.u32(object), TES_OBJECT_VTABLE);
            assert_eq!(calls(&mut e), vec![(TES_FORM_DESTRUCT, vec![object])]);

            start_log(&mut e);
            assert_eq!(e.call(deleting, &[object, 1]).u32(), object);
            let log = calls(&mut e);
            assert_eq!(log.last().unwrap(), &(OPERATOR_DELETE, vec![object]));
            start_log(&mut e);
            e.call(deleting, &[object, 0]);
            assert!(!addresses(&calls(&mut e)).contains(&OPERATOR_DELETE));
        }
    }

    #[test]
    fn forwarder_passes_the_flag_byte() {
        let mut e = engine();
        e.register(0x008d_0600, |_, _| ret(0x42));
        start_log(&mut e);
        e.call(0x0050f780, &[0x1000, 0x2000, 0x1ff]);
        assert_eq!(
            calls(&mut e),
            vec![(0x008d_0600, vec![0x1000, 0x2000, 0xff])]
        );
    }

    #[test]
    fn type_0x25_form_builds_and_announces_its_result() {
        let mut e = engine();
        e.register(FORM_TYPE, |e, _| ret(e.global(SCRATCH_TYPE)));
        e.register(0x0066_4840, |e, _| ret(e.global(SCRATCH_COUNT)));
        e.register(0x0066_4f50, |_, _| ret(0));
        e.register(0x7000_0180, |_, _| ret(0));
        e.register(0x7000_0100, |_, _| ret(0x6006));
        e.register(0x008d_6f30, |_, _| ret(0x8d));
        e.register(0x0056_9350, |_, _| ret(0));
        let object = object_with_vtable(&mut e, 0x30, &[(0x180, 0x7000_0180)]);
        let result = object_with_vtable(&mut e, 0x30, &[(0x100, 0x7000_0100)]);
        e.register_double(0x0066_4f50, move |_, _| ret(result));
        e.set_global(SCRATCH_TYPE, 0x24u32);
        start_log(&mut e);
        assert_eq!(e.call(0x0050f810, &[object, 0x5000, 0]).u32(), 0);
        assert_eq!(calls(&mut e).len(), 1);

        e.set_global(SCRATCH_TYPE, 0x25u32);
        e.set_global(SCRATCH_COUNT, 0x7001u32);
        start_log(&mut e);
        assert_eq!(e.call(0x0050f810, &[object, 0x5000, 0]).u32(), result);
        let log = calls(&mut e);
        assert_eq!(log[3], (0x0066_4f50, vec![0x7001, 0x5000, object]));
        assert_eq!(log[4], (0x7000_0180, vec![object, result]));
        assert_eq!(log[7], (0x008d_6f30, vec![0x6006]));
        assert_eq!(log[8], (0x0056_9350, vec![0x5000, 0x8d]));

        // No manager: the virtual hears about a null result and that is all.
        e.set_global(SCRATCH_COUNT, 0u32);
        start_log(&mut e);
        assert_eq!(e.call(0x0050f810, &[object, 0x5000, 0]).u32(), 0);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![FORM_TYPE, 0x0066_4840, 0x7000_0180]
        );
    }

    #[test]
    fn object_constructor_clears_the_list_node() {
        let mut e = object_engine();
        let object = e.mem.alloc(0x24);
        e.mem.write(object, &[0xa5; 0x24]);
        start_log(&mut e);
        assert_eq!(e.call(0x0050f8c0, &[object]).u32(), object);
        assert_eq!(e.mem.u32(object), TES_OBJECT_VTABLE);
        for offset in [0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u32(object + 4), 0xa5a5_a5a5);
        assert_eq!(calls(&mut e), vec![(TES_FORM_CONSTRUCT, vec![object])]);
    }

    #[test]
    fn object_destructor_leaves_its_parent_list() {
        let mut e = object_engine();
        let object = e.mem.alloc(0x24);
        let list = e.mem.alloc(16);
        e.mem.set_u32(object + 0x18, list);
        start_log(&mut e);
        e.call(0x0050f930, &[object]);
        assert_eq!(
            calls(&mut e),
            vec![
                (LIST_REMOVE, vec![list, object]),
                (TES_FORM_DESTRUCT, vec![object]),
            ]
        );
        assert_eq!(e.mem.u32(object), TES_OBJECT_VTABLE);

        e.mem.set_u32(object + 0x18, 0);
        start_log(&mut e);
        e.call(0x0050f930, &[object]);
        assert_eq!(calls(&mut e), vec![(TES_FORM_DESTRUCT, vec![object])]);

        start_log(&mut e);
        assert_eq!(e.call(0x0050f900, &[object, 1]).u32(), object);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![TES_FORM_DESTRUCT, OPERATOR_DELETE]
        );
    }

    #[test]
    fn shared_setter_keeps_neighbors_and_accepts_all_word_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x28);
        e.mem.write(object, &[0xa5; 0x28]);
        for value in [0, 0x2000_0040, u32::MAX] {
            e.call(0x0050f9c0, &[object, value]);
            assert_eq!(e.mem.u32(object + 0x20), value);
            assert_eq!(e.mem.bytes(object, 0x20), vec![0xa5; 0x20]);
            assert_eq!(e.mem.u32(object + 0x24), 0xa5a5_a5a5);
        }
    }

    #[test]
    fn marker_forms_by_type() {
        let mut e = engine();
        e.register(FORM_TYPE, |e, _| ret(e.global(SCRATCH_TYPE)));
        e.register(FORM_SUBTYPE, |e, _| ret(e.global(SCRATCH_SUBTYPE)));
        e.register(FORM_MODEL_PATH, |_, _| ret(0x4444));
        e.register(COMPARE_NAMES, |e, _| ret(e.global(SCRATCH_COMPARE)));
        e.register(FORM_MODEL_LENGTH, |e, _| ret(e.global(SCRATCH_LENGTH)));
        let form = 0x1234_5678;
        assert_eq!(e.call(0x0050f9e0, &[0]).u32() & 0xff, 0);

        // Types 4 and 0x30 are always markers.
        for form_type in [4u32, 0x30] {
            e.set_global(SCRATCH_TYPE, form_type);
            e.set_global(SCRATCH_SUBTYPE, 0u32);
            assert!(e.call(0x0050f9e0, &[form]).bool());
        }
        // Type 0x15: only the editor marker model.
        e.set_global(SCRATCH_TYPE, 0x15u32);
        e.set_global(SCRATCH_COMPARE, 1u32);
        assert!(!e.call(0x0050f9e0, &[form]).bool());
        e.set_global(SCRATCH_COMPARE, 0u32);
        start_log(&mut e);
        assert!(e.call(0x0050f9e0, &[form]).bool());
        assert_eq!(
            calls(&mut e)[2],
            (COMPARE_NAMES, vec![0x4444, EDITOR_MARKER_MODEL])
        );
        // Type 0x1c: the one global form.
        e.set_global(SCRATCH_TYPE, 0x1cu32);
        assert!(!e.call(0x0050f9e0, &[form]).bool());
        e.set_global(MARKER_FORM_TYPE_1C, form);
        assert!(e.call(0x0050f9e0, &[form]).bool());
        // Type 0x1e: an empty model.
        e.set_global(SCRATCH_TYPE, 0x1eu32);
        e.set_global(SCRATCH_LENGTH, 3u32);
        assert!(!e.call(0x0050f9e0, &[form]).bool());
        e.set_global(SCRATCH_LENGTH, 0u32);
        assert!(e.call(0x0050f9e0, &[form]).bool());
        // Type 0x20: any of the fourteen globals.
        e.set_global(SCRATCH_TYPE, 0x20u32);
        assert!(!e.call(0x0050f9e0, &[form]).bool());
        for global in MARKER_FORMS_TYPE_20 {
            e.set_global(global, form);
            assert!(e.call(0x0050f9e0, &[form]).bool());
            e.set_global(global, 0u32);
        }
        // Everything else (and the misses above): the subtype window.
        for (subtype, expected) in [(99u32, false), (100, true), (0x77, true), (0x78, false)] {
            e.set_global(SCRATCH_TYPE, 0x40u32);
            e.set_global(SCRATCH_SUBTYPE, subtype);
            assert_eq!(e.call(0x0050f9e0, &[form]).bool(), expected);
        }
        e.set_global(SCRATCH_TYPE, 0x20u32);
        e.set_global(SCRATCH_SUBTYPE, 0x70u32);
        assert!(e.call(0x0050f9e0, &[form]).bool());
    }

    #[test]
    fn named_forms_are_type_0x20_and_global() {
        let mut e = engine();
        e.register(FORM_TYPE, |e, _| ret(e.global(SCRATCH_TYPE)));
        let form = 0x1234_5678;
        assert!(!e.call(0x0050fb90, &[0]).bool());
        e.set_global(SCRATCH_TYPE, 0x20u32);
        assert!(!e.call(0x0050fb90, &[form]).bool());
        for global in NAMED_FORMS_TYPE_20 {
            e.set_global(global, form);
            assert!(e.call(0x0050fb90, &[form]).bool());
            e.set_global(SCRATCH_TYPE, 0x21u32);
            assert!(!e.call(0x0050fb90, &[form]).bool());
            e.set_global(SCRATCH_TYPE, 0x20u32);
            e.set_global(global, 0u32);
        }
    }

    #[test]
    fn two_word_stub_returns_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x0050fbe0, &[1, 2, 3]).u32(), 0);
    }

    #[test]
    fn model_registration_by_scale() {
        let mut e = engine();
        let path = c_string(&mut e, b"a.nif");
        e.set_global(MODEL_LOADER_POINTER, 0x1234u32);
        e.register_double(0x0057_15d0, move |_, _| ret(path));
        e.register(0x0048_0e60, |_, _| ret(0));
        e.register(FORM_MODEL_PATH, |_, _| ret(0));
        e.register(FORM_TYPE, |e, _| ret(e.global(SCRATCH_TYPE)));
        e.register(0x0056_7400, |e, _| {
            float_ret(f64::from(e.global::<f32>(SCRATCH_COUNT)))
        });
        e.register(0x0050_fdf0, |_, _| ret(0));
        e.register(0x0063_3c90, |_, _| ret(0));
        e.register(0x0044_72a0, |e, _| ret(e.global(SCRATCH_LENGTH)));
        e.register(0x0055_9450, |_, _| ret(0x6000));
        e.register(0x0044_ddc0, |e, _| ret(e.global(SCRATCH)));
        stub(&mut e, &[0x0045_a5e0, 0x0040_c110, 0x0060_3bb0]);
        e.set_global(SCRATCH_TYPE, 0x2au32);
        e.set_global(COUNTER_A, 10u32);
        e.set_global(COUNTER_B, 20u32);
        let form = 0x3000;

        // Scale 1.0 registers the plain path, and type 0x2a runs its hook.
        e.set_global(SCRATCH_COUNT, 1.0f32);
        start_log(&mut e);
        e.call(0x0050fbf0, &[form, 0x4000]);
        let log = calls(&mut e);
        assert!(log.contains(&(0x0045_a5e0, vec![0x1234, path])));
        assert_eq!(log.last().unwrap(), &(0x0060_3bb0, vec![form]));

        // Another scale registers the scaled name; a found model that is
        // still referenced decrements counter B.
        e.set_global(SCRATCH_COUNT, 2.0f32);
        e.set_global(SCRATCH_LENGTH, 1u32);
        e.set_global(SCRATCH, 3u32);
        start_log(&mut e);
        e.call(0x0050fbf0, &[form, 0x4000]);
        let log = calls(&mut e);
        assert!(!log
            .iter()
            .any(|call| call.0 == 0x0045_a5e0 && call.1[1] == path));
        assert_eq!(e.global::<u32>(COUNTER_B), 19);
        assert_eq!(e.global::<u32>(COUNTER_A), 10);

        // Not referenced (or not found): the plain path too, counter A down.
        e.set_global(SCRATCH, 0u32);
        e.call(0x0050fbf0, &[form, 0x4000]);
        assert_eq!(e.global::<u32>(COUNTER_A), 9);
        e.set_global(SCRATCH_LENGTH, 0u32);
        e.call(0x0050fbf0, &[form, 0x4000]);
        assert_eq!(e.global::<u32>(COUNTER_A), 8);

        // An empty path registers nothing; the special forms ignore the reference.
        let empty = c_string(&mut e, b"");
        e.register_double(0x0057_15d0, move |_, _| ret(empty));
        start_log(&mut e);
        e.call(0x0050fbf0, &[form, 0x4000]);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![0x0057_15d0, FORM_TYPE, 0x0060_3bb0]
        );
        e.set_global(SPECIAL_FORM_A, form);
        start_log(&mut e);
        e.call(0x0050fbf0, &[form, 0x4000]);
        assert!(!addresses(&calls(&mut e)).contains(&0x0057_15d0));
    }

    #[test]
    fn model_path_falls_back_in_order() {
        let mut e = engine();
        e.register(0x0057_15d0, |_, _| ret(0x11));
        e.register(0x0048_0e60, |_, _| ret(0x22));
        e.register(FORM_MODEL_PATH, |_, _| ret(0x33));
        start_log(&mut e);
        assert_eq!(e.call(0x0050fd90, &[0x1000, 0x2000]).u32(), 0x11);
        assert_eq!(calls(&mut e), vec![(0x0057_15d0, vec![0x2000])]);

        e.register(0x0057_15d0, |_, _| ret(0));
        start_log(&mut e);
        assert_eq!(e.call(0x0050fd90, &[0x1000, 0x2000]).u32(), 0x22);
        assert_eq!(calls(&mut e)[1], (0x0048_0e60, vec![0x1000, 0x2000]));

        e.register(0x0048_0e60, |_, _| ret(0));
        assert_eq!(e.call(0x0050fd90, &[0x1000, 0x2000]).u32(), 0x33);
        // Without a reference the reference lookup is skipped.
        start_log(&mut e);
        e.call(0x0050fd90, &[0x1000, 0]);
        assert_eq!(
            addresses(&calls(&mut e)),
            vec![0x0048_0e60, FORM_MODEL_PATH]
        );
    }

    #[test]
    fn scaled_model_file_name_formats_path_and_percent() {
        let mut e = engine();
        e.set_global(ONE_HUNDRED, 100.0f64);
        e.register(0x0057_15d0, |_, _| ret(0x11));
        e.register(0x0056_7400, |_, _| float_ret(1.5));
        e.register(0x0040_6d90, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        e.register(0x0040_6d00, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0050fdf0, &[0x1000, 0x2000, 0x3000]);
        assert_eq!(
            calls(&mut e),
            vec![
                (0x0057_15d0, vec![0x2000]),
                (0x0056_7400, vec![0x2000]),
                (0x0040_6d90, vec![150.0f32.to_bits()]),
                (
                    0x0040_6d00,
                    vec![0x3000, 0x104, SCALED_NAME_FORMAT, 0x11, 150]
                ),
            ]
        );
    }

    #[test]
    fn scaled_model_file_name_passes_a_nan_scale_on_quietly() {
        let mut e = engine();
        e.set_global(ONE_HUNDRED, 100.0f64);
        e.register(0x0057_15d0, |_, _| ret(0x11));
        e.register(0x0056_7400, |_, _| float_ret(f64::NAN));
        e.register(0x0040_6d90, |_, _| ret(0x8000_0000));
        e.register(0x0040_6d00, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x0050fdf0, &[0x1000, 0x2000, 0x3000]);
        let log = calls(&mut e);
        let pushed = log[2].1[0];
        assert!(f32::from_bits(pushed).is_nan());
        assert_ne!(pushed & 0x0040_0000, 0);
    }
}

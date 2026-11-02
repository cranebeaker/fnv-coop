//! `fallout/misc/seendata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SeenData` keeps one bit per piece of a cell (the local map's fog of war);
//! `IntSeenData` is the interior form, one section of bits per 4096-unit
//! square, chained through `pNextSection`.
//!
//! Notes on the exe's code:
//! - The Xbox PDB gives `SeenData` two statics, `fPosSize` and
//!   `iTotalBitCount`. The code reads a float at `011a31e8` as the size of
//!   one bit's square and a word at `011a31ec` as the number of bits; the
//!   names fit, but the addresses are not tied to the symbols, so the
//!   constants here are named by use.
//! - Floats are x87: each `FLD`/`FSTP` pair quiets a signalling NaN, each
//!   arithmetic instruction goes through `crate::x87`, and floats pushed as
//!   arguments are passed as bit patterns.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `SeenData` (Xbox PDB), 0x24 bytes.
    pub struct SeenData: 0x24 {
        /// `pSeenBits` (Xbox PDB): 0x20 bytes of bits.
        0x04 pSeenBits: u32,
    }

    /// `IntSeenData` (Xbox PDB), 0x2c bytes: a `SeenData` and its section.
    pub struct IntSeenData: 0x2c {
        /// `cSectionX` (Xbox PDB).
        0x24 cSectionX: u8,
        /// `cSectionY` (Xbox PDB).
        0x25 cSectionY: u8,
        /// `pNextSection` (Xbox PDB): `IntSeenData*`.
        0x28 pNextSection: u32,
    }
}

/// The `SeenData` vtable.
const SEEN_DATA_VTABLE: u32 = 0x0108_3fc4;
/// The `IntSeenData` vtable.
const INT_SEEN_DATA_VTABLE: u32 = 0x0108_3fe4;
/// The size of one seen bit's square (a `float`; see the module notes).
const BIT_SQUARE_SIZE: u32 = 0x011a_31e8;
/// The number of seen bits (a word; see the module notes).
const BIT_COUNT: u32 = 0x011a_31ec;
/// `FLT_MAX` (`float`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// The setting whose value (`00403e20`) is the distance under which a piece
/// counts as seen.
const SEEN_DISTANCE_SETTING: u32 = 0x011c_a094;
/// The setting whose value (`0043d4d0`) is the number of seen pieces that
/// makes a section count as fully seen.
const SEEN_COUNT_SETTING: u32 = 0x011d_f198;

/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiPoint3` constructor (`00416870`, `thiscall`: x, y, z), answers `this`.
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `00406d90` (`cdecl`, one `float`): the float as an integer (`EAX`).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0` (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// A float setting's value address (`00403e20`, `thiscall`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// An integer setting's value address (`0043d4d0`, `thiscall`).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `006815c0` (`thiscall`): called on the point local before it is filled.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// `0063c8a0` (`thiscall`, `RET 4`): `point += other`.
const POINT_ADD: u32 = 0x0063_c8a0;
/// `00439ef0` (`thiscall`, `RET 8`): `(point, result, other)`, the
/// difference of two points into `result`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990` (`thiscall`): a point's length, in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `thiscall`.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map),
/// `thiscall`: count, handle.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// The `SeenData` constructor (`00879940`, unit `run_008798e0`).
const SEEN_DATA_CONSTRUCT: u32 = 0x0087_9940;

// Translated from 00879970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData` scalar deleting destructor (Xbox PDB name from the engine
/// map): runs the destructor body `008799a0`, frees `this` (`00401030`) when
/// `flags & 1`, returns `this`.
pub fn seen_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008799a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008799a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SeenData` destructor body: stores the `SeenData` vtable.
pub fn fn_008799a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SEEN_DATA_VTABLE);
}

/// The word of the seen bits that holds `bit` and the bit's mask in it.
fn bit_word_and_mask(this: Ptr, bit: u32) -> (u32, u32) {
    let word = bit >> 5;
    let shift = bit.wrapping_sub(word << 5);
    (
        this.addr()
            .wrapping_add(word.wrapping_mul(4))
            .wrapping_add(4),
        1u32 << (shift & 0x1f),
    )
}

// Translated from 008799f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::IsSeenBitTrue_ov2` (Xbox PDB): whether bit `bit` of the seen
/// bits is set.
pub fn seen_data_is_seen_bit_true_ov2(e: &mut Engine, this: Ptr, bit: u32) -> bool {
    let (address, mask) = bit_word_and_mask(this, bit);
    mask & e.mem.u32(address) != 0
}

// Translated from 00879a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit `bit` of the seen bits and returns
/// whether it was set before.
pub fn fn_00879a40(e: &mut Engine, this: Ptr, bit: u32, value: u8) -> bool {
    let (address, mask) = bit_word_and_mask(this, bit);
    let was_set = mask & e.mem.u32(address) != 0;
    let word = e.mem.u32(address);
    if value != 0 {
        e.mem.set_u32(address, mask | word);
    } else {
        e.mem.set_u32(address, !mask & word);
    }
    was_set
}

// Translated from 00879ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks every unseen bit whose piece is near enough as seen and answers
/// whether enough bits are seen (`thiscall`, `RET 8`). For each bit below
/// the bit count (re-read each pass, unsigned): a set bit counts; for an
/// unset one, the piece's point (`00879bd0`) is moved by `origin`
/// (`0063c8a0`), its difference from `position` (`00439ef0`) measured
/// (`00457990`), and when the seen distance (`00879bb0`) is greater than
/// that length the bit is set (`00879a40`) and counts. Answers whether the
/// count (signed) reaches the value of the setting at `011df198`.
pub fn fn_00879ad0(e: &mut Engine, this: Ptr, position: u32, origin: u32) -> bool {
    e.with_stack(0x18, |e, locals| {
        let point = locals.addr();
        let difference = locals.addr() + 0xc;
        let mut count = 0i32;
        let mut bit = 0u32;
        while bit < e.global::<u32>(BIT_COUNT) {
            if seen_data_is_seen_bit_true_ov2(e, this, bit) {
                count = count.wrapping_add(1);
            } else {
                e.call(POINT_PREPARE, &args![point]);
                fn_00879bd0(e, bit, point);
                e.call(POINT_ADD, &args![point, origin]);
                e.call(POINT_SUBTRACT, &args![point, difference, position]);
                let length = e.call(POINT_LENGTH, &args![difference]).f64();
                let seen_distance = fn_00879bb0(e) as f64;
                if seen_distance > length {
                    fn_00879a40(e, this, bit, 1);
                    count = count.wrapping_add(1);
                }
            }
            bit = bit.wrapping_add(1);
        }
        let needed = e.call(INT_SETTING_VALUE, &args![SEEN_COUNT_SETTING]).u32();
        count >= e.mem.i32(needed)
    })
}

// Translated from 00879bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the float setting at `011ca094` (the seen distance), loaded
/// with `FLD`.
pub fn fn_00879bb0(e: &mut Engine) -> f32 {
    let value = e
        .call(FLOAT_SETTING_VALUE, &args![SEEN_DISTANCE_SETTING])
        .u32();
    x87_quiet(e.mem.f32(value))
}

// Translated from 00879bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The point of bit `bit` (`cdecl`, two words), written to `out`: for a bit
/// below the bit count, `(size * (bit % 16), size * (bit / 16), 0)` built
/// with the `NiPoint3` constructor (`00416870`) on a local, with `size` the
/// float at `011a31e8`; otherwise `(FLT_MAX, FLT_MAX, FLT_MAX)`.
pub fn fn_00879bd0(e: &mut Engine, bit: u32, out: u32) {
    e.with_stack(0xc, |e, local| {
        let point = if bit < e.global::<u32>(BIT_COUNT) {
            let column = x87_store32(f64::from(bit % 0x10));
            let row = x87_store32(f64::from(bit >> 4));
            let size = e.global::<u32>(BIT_SQUARE_SIZE);
            let y = x87_store32(x87_mul(x87_load32(size), x87_m32(row)));
            let x = x87_store32(x87_mul(x87_load32(size), x87_m32(column)));
            e.call(
                POINT_CONSTRUCT,
                &args![local, x87_quiet_bits(x), x87_quiet_bits(y), 0u32],
            )
            .u32()
        } else {
            let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
            e.call(POINT_CONSTRUCT, &args![local, max, max, max]).u32()
        };
        for i in 0..3 {
            let word = e.mem.u32(point + 4 * i);
            e.mem.set_u32(out + 4 * i, word);
        }
    });
}

// Translated from 00879ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Snaps `point` (`cdecl`, one word) to the corner of its seen bit's square:
/// for x and y, the 4096-unit cell origin (`00406d90(value) >> 12 << 12`)
/// plus the whole number of squares (`_ftol2` of
/// `00406d90(value - origin) / size`) times the square size; z becomes 0.
pub fn fn_00879ca0(e: &mut Engine, point: u32) {
    let x_bits = e.mem.u32(point);
    let cell_x = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(x_bits)]).i32() >> 12;
    let y_bits = e.mem.u32(point + 4);
    let cell_y = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(y_bits)]).i32() >> 12;

    let size = e.global::<u32>(BIT_SQUARE_SIZE);

    let x_bits = e.mem.u32(point);
    let offset_x = x87_store32(x87_sub(x87_m32(x_bits), f64::from(cell_x.wrapping_shl(12))));
    let whole_x = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(offset_x)]).i32();
    let squares_x = e
        .call(FTOL, &args![x87_div(f64::from(whole_x), x87_m32(size))])
        .i32();

    let y_bits = e.mem.u32(point + 4);
    let offset_y = x87_store32(x87_sub(x87_m32(y_bits), f64::from(cell_y.wrapping_shl(12))));
    let whole_y = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(offset_y)]).i32();
    let squares_y = e
        .call(FTOL, &args![x87_div(f64::from(whole_y), x87_m32(size))])
        .i32();

    let x = x87_add(
        f64::from(cell_x.wrapping_shl(12)),
        x87_mul(f64::from(squares_x), x87_m32(size)),
    );
    e.mem.set_u32(point, x87_store32(x));
    let y = x87_add(
        f64::from(cell_y.wrapping_shl(12)),
        x87_mul(f64::from(squares_y), x87_m32(size)),
    );
    e.mem.set_u32(point + 4, x87_store32(y));
    e.mem.set_u32(point + 8, 0);
}

// Translated from 0087a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::SaveGame_ov2` (Xbox PDB): saves the 0x20 bytes of seen bits
/// (`00865e50(buffer, this + 4, 0x20, 0)`).
pub fn seen_data_save_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(
        SAVE_DATA,
        &args![buffer, this.addr().wrapping_add(4), 0x20u32, 0u32],
    );
}

// Translated from 0087a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IntSeenData` constructor (no name in the engine map): the
/// `SeenData` constructor (`00879940`), the `IntSeenData` vtable, the
/// section's x and y bytes and no next section. Returns `this`.
pub fn fn_0087a100(e: &mut Engine, this: Ptr<IntSeenData>, x: u8, y: u8) -> Ptr<IntSeenData> {
    e.call(SEEN_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), INT_SEEN_DATA_VTABLE);
    e.set(this, IntSeenData::cSectionX, x);
    e.set(this, IntSeenData::cSectionY, y);
    e.set(this, IntSeenData::pNextSection, 0);
    this
}

// Translated from 0087a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData` scalar deleting destructor (Xbox PDB name from the engine
/// map): runs the destructor body `0087a170`, frees `this` (`00401030`) when
/// `flags & 1`, returns `this`.
pub fn int_seen_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<IntSeenData>,
    flags: u32,
) -> Ptr<IntSeenData> {
    fn_0087a170(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0087a170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0087a170 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `IntSeenData` destructor body: stores the `IntSeenData` vtable,
/// deletes the next section through its virtual slot 0 (the scalar deleting
/// destructor, flags 1) when there is one, then runs the `SeenData`
/// destructor body (`008799a0`).
pub fn fn_0087a170(e: &mut Engine, this: Ptr<IntSeenData>) {
    e.mem.set_u32(this.addr(), INT_SEEN_DATA_VTABLE);
    let next = e.get(this, IntSeenData::pNextSection);
    if next != 0 {
        e.vcall(next, 0, &args![1u32]);
    }
    fn_008799a0(e, this.cast());
}

// Translated from 0087a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::SaveGame_ov2` (Xbox PDB): starts a variable-sized value
/// (`00865f20`), saves each section of the chain from `this` on (its x and
/// y bytes with `00865e50`, then its bits, `0087a0c0`) and ends the value
/// with the number of sections (`00865ff0`).
pub fn int_seen_data_save_game_ov2(e: &mut Engine, this: Ptr<IntSeenData>, buffer: u32) {
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut section = this;
    while !section.is_null() {
        e.call(
            SAVE_DATA,
            &args![buffer, section.addr().wrapping_add(0x24), 1u32, 0u32],
        );
        e.call(
            SAVE_DATA,
            &args![buffer, section.addr().wrapping_add(0x25), 1u32, 0u32],
        );
        seen_data_save_game_ov2(e, section.cast(), buffer);
        section = Ptr::new(e.get(section, IntSeenData::pNextSection));
        count = count.wrapping_add(1);
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

/// The `TESSaveLoadGame` pointer (`011de45c`) and its byte writer
/// (`008579b0`) and reader (`008579e0`), `thiscall`, `RET 8`: address,
/// size.
const TES_SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const TES_SAVE_WRITE: u32 = 0x0085_79b0;
const TES_SAVE_READ: u32 = 0x0085_79e0;
/// `BGSLoadGameBuffer`'s raw load (`00864980`, `thiscall`: destination,
/// size) and variable-sized value (`00864a60`, `thiscall`).
const BUFFER_LOAD_RAW: u32 = 0x0086_4980;
const BUFFER_LOAD_VARIABLE: u32 = 0x0086_4a60;
/// The game's `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00414430` (`thiscall`, `RET 0x10`: four floats): the colour local's
/// constructor.
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `MakeRectangle` (`004b3b90`, cdecl: size float, colour, 1).
const MAKE_RECTANGLE: u32 = 0x004b_3b90;
/// `004bc1f0` (`thiscall`, `RET 0xc`: x, y, z): places the rectangle.
const PLACE_RECTANGLE: u32 = 0x004b_c1f0;
/// The `double` the square size is halved with (`01011590`, 2.0 in the
/// exe).
const TWO_DOUBLE: u32 = 0x0101_1590;

// Translated from 008799c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::IsSeenBitTrue` (Xbox PDB; `thiscall`, `RET 8`: x, y): bit
/// `y * 16 + x` (`008799f0`).
pub fn seen_data_is_seen_bit_true(e: &mut Engine, this: Ptr, x: u32, y: u32) -> bool {
    seen_data_is_seen_bit_true_ov2(e, this, (y << 4).wrapping_add(x))
}

// Translated from 00879d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument): the square size float at
/// `011a31e8` (`FLD`, in `ST0`).
pub fn fn_00879d90(e: &mut Engine) -> f32 {
    f32::from_bits(x87_quiet_bits(e.global::<u32>(BIT_SQUARE_SIZE)))
}

/// `00406d90` on a float word (`FLD`/`FSTP` onto the stack).
fn float_word_to_int(e: &mut Engine, bits: u32) -> i32 {
    e.call(FLOAT_TO_INT, &args![x87_quiet_bits(bits)]).i32()
}

// Translated from 00879da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: point, byte, x out, y out): the
/// square of `point` within its section, `(coordinate - section origin) /
/// size` with the size truncated (`_ftol2`) and the division signed. With
/// the byte set the sections are centred (`(i - 0x800) >> 12` and the
/// origin `section << 12 + 0x800`), otherwise `i >> 12` and `section << 12`,
/// `i` being `00406d90` of the coordinate.
pub fn fn_00879da0(e: &mut Engine, point: u32, centred: u8, x_out: u32, y_out: u32) {
    let size = e.global::<u32>(BIT_SQUARE_SIZE);
    let x_bits = e.mem.u32(point);
    let y_bits = e.mem.u32(point.wrapping_add(4));
    let (section_x, section_y) = if centred != 0 {
        let x = float_word_to_int(e, x_bits).wrapping_sub(0x800) >> 12;
        let y_bits = e.mem.u32(point.wrapping_add(4));
        let y = float_word_to_int(e, y_bits).wrapping_sub(0x800) >> 12;
        (x, y)
    } else {
        let x = float_word_to_int(e, x_bits) >> 12;
        let y = float_word_to_int(e, y_bits) >> 12;
        (x, y)
    };
    let origin = |section: i32| {
        let base = section.wrapping_shl(12);
        if centred != 0 {
            base.wrapping_add(0x800)
        } else {
            base
        }
    };
    for (index, (out, section)) in [(x_out, section_x), (y_out, section_y)]
        .into_iter()
        .enumerate()
    {
        let bits = e.mem.u32(point.wrapping_add(4 * index as u32));
        let offset = float_word_to_int(e, bits).wrapping_sub(origin(section));
        e.mem.set_i32(out, offset);
        let divisor = e.call(FTOL, &args![x87_load32(size)]).i32();
        let value = e.mem.i32(out) / divisor;
        e.mem.set_i32(out, value);
    }
}

// Translated from 00879f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::DrawSeenData` (Xbox PDB; `thiscall`, `RET 0xc`: node,
/// origin point, a word not read): nothing without a node. With a colour
/// `(1, 0, 0, 0)` (`00414430`) and half the square size (divided by the
/// `double` at `01011590`), each seen bit's square (`00879bd0` moved by
/// `origin`) becomes a rectangle (`MakeRectangle` with the size, the
/// colour and 1) attached to the node (vtable `+0xdc` with 1) and placed at
/// the square's centre (`004bc1f0` with x and y plus the half, and z).
pub fn seen_data_draw_seen_data(e: &mut Engine, this: Ptr, node: u32, origin: u32, _unused_3: u32) {
    if node == 0 {
        return;
    }
    e.with_stack(0x1c, |e, locals| {
        let color = locals.addr();
        let point = color + 0x10;
        let one = x87_store32(1.0);
        let zero = x87_store32(0.0);
        e.call(COLOR_CONSTRUCT, &args![color, one, zero, zero, zero]);
        let size = e.global::<u32>(BIT_SQUARE_SIZE);
        let two: f64 = e.global(TWO_DOUBLE);
        let half = x87_store32(x87_div(x87_load32(size), two));
        let mut bit = 0u32;
        while bit < e.global::<u32>(BIT_COUNT) {
            if seen_data_is_seen_bit_true_ov2(e, this, bit) {
                e.call(POINT_PREPARE, &args![point]);
                fn_00879bd0(e, bit, point);
                e.call(POINT_ADD, &args![point, origin]);
                let size = x87_quiet_bits(e.global::<u32>(BIT_SQUARE_SIZE));
                let rectangle = e.call(MAKE_RECTANGLE, &args![size, color, 1u32]).u32();
                e.vcall(node, 0xdc, &args![rectangle, 1u32]);
                let z = x87_quiet_bits(e.mem.u32(point + 8));
                let y = x87_store32(x87_add(x87_load32(e.mem.u32(point + 4)), x87_m32(half)));
                let x = x87_store32(x87_add(x87_load32(e.mem.u32(point)), x87_m32(half)));
                e.call(
                    PLACE_RECTANGLE,
                    &args![rectangle, x87_quiet_bits(x), x87_quiet_bits(y), z],
                );
            }
            bit = bit.wrapping_add(1);
        }
    });
}

// Translated from 0087a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::GetSaveSize` (unnamed in the engine map; `thiscall`, `RET 4`,
/// the word not read): 0x20 (16-bit).
pub fn fn_0087a030(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u16 {
    0x20
}

// Translated from 0087a060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::SaveGame` (Xbox PDB; `thiscall`, `RET 4`, the word not
/// read): writes the 0x20 bytes of bits (`008579b0` on `011de45c`).
pub fn seen_data_save_game(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
    e.call(
        TES_SAVE_WRITE,
        &args![game, this.addr().wrapping_add(4), 0x20u32],
    );
}

// Translated from 0087a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::LoadGame` (Xbox PDB; `thiscall`, `RET 4`, the word not
/// read): reads the 0x20 bytes of bits (`008579e0` on `011de45c`).
pub fn seen_data_load_game(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
    e.call(
        TES_SAVE_READ,
        &args![game, this.addr().wrapping_add(4), 0x20u32],
    );
}

// Translated from 0087a0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::LoadGame_ov2` (Xbox PDB; `thiscall`, `RET 4`): loads the
/// 0x20 bytes of bits from the load buffer (`00864980`).
pub fn seen_data_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(
        BUFFER_LOAD_RAW,
        &args![buffer, this.addr().wrapping_add(4), 0x20u32],
    );
}

// Translated from 0087a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::DrawSeenData` (Xbox PDB; `thiscall`, `RET 0xc`: node,
/// point, byte): draws this section (`00879f20`) from the corner
/// `(cSectionX * 4096 + 2048, cSectionY * 4096 + 2048, point.z)` (signed
/// bytes, `FILD`, a `NiPoint3` local) and then the next section through
/// its vtable `+4` with the same arguments.
pub fn int_seen_data_draw_seen_data(
    e: &mut Engine,
    this: Ptr<IntSeenData>,
    node: u32,
    point: u32,
    flag: u8,
) {
    let section_x = i32::from(e.get(this, IntSeenData::cSectionX) as i8);
    let section_y = i32::from(e.get(this, IntSeenData::cSectionY) as i8);
    e.with_stack(0xc, |e, corner| {
        let z = x87_quiet_bits(e.mem.u32(point.wrapping_add(8)));
        let y = x87_store32(f64::from(section_y.wrapping_shl(12).wrapping_add(0x800)));
        let x = x87_store32(f64::from(section_x.wrapping_shl(12).wrapping_add(0x800)));
        e.call(POINT_CONSTRUCT, &args![corner, x, y, z]);
        seen_data_draw_seen_data(e, this.cast(), node, corner.addr(), u32::from(flag));
    });
    let next = e.get(this, IntSeenData::pNextSection);
    if next != 0 {
        e.vcall(next, 4, &args![node, point, u32::from(flag)]);
    }
}

// Translated from 0087a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::GetSaveSize` (Xbox PDB; `thiscall`, `RET 4`: a byte): the
/// bits (`0087a030`), two more for the section count unless `nested`, the
/// two section bytes, and the next section's size (vtable `+8` with 1);
/// 16-bit arithmetic.
pub fn int_seen_data_get_save_size(e: &mut Engine, this: Ptr<IntSeenData>, nested: u8) -> u16 {
    let mut size = fn_0087a030(e, this.cast(), 0);
    if nested == 0 {
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(1).wrapping_add(1);
    let next = e.get(this, IntSeenData::pNextSection);
    if next != 0 {
        let more = e.vcall(next, 8, &args![1u32]).u16();
        size = size.wrapping_add(more);
    }
    size
}

// Translated from 0087a330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::SaveGame` (Xbox PDB; `thiscall`, `RET 4`: a byte): the
/// bits (`0087a060`), then unless `nested` the 16-bit number of sections
/// in the chain, then the two section bytes (`008579b0`), then the next
/// section (vtable `+0x10` with 1).
pub fn int_seen_data_save_game(e: &mut Engine, this: Ptr<IntSeenData>, nested: u8) {
    seen_data_save_game(e, this.cast(), 0);
    if nested == 0 {
        let mut count = 1u16;
        let mut section = this.addr();
        loop {
            section = e.mem.u32(section.wrapping_add(0x28));
            if section == 0 {
                break;
            }
            count = count.wrapping_add(1);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u16(slot.addr(), count);
            let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
            e.call(TES_SAVE_WRITE, &args![game, slot, 2u32]);
        });
    }
    for offset in [0x24u32, 0x25] {
        let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
        e.call(
            TES_SAVE_WRITE,
            &args![game, this.addr().wrapping_add(offset), 1u32],
        );
    }
    let next = e.get(this, IntSeenData::pNextSection);
    if next != 0 {
        e.vcall(next, 0x10, &args![1u32]);
    }
}

/// A new section (0x2c bytes, `00401000`, `0087a100` with 0 and 0; null
/// when the allocation fails).
fn new_section(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
    if block == 0 {
        return 0;
    }
    fn_0087a100(e, Ptr::new(block), 0, 0).addr()
}

// Translated from 0087a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::LoadGame` (Xbox PDB; `thiscall`, `RET 4`: the sections
/// left, 16-bit): the bits (`0087a090`); with 0xffff the count is read
/// first (two bytes into the argument word, `008579e0`); then the two
/// section bytes. With sections left after this one, a new section
/// becomes the next (`0087a100`) and loads (vtable `+0x18`) the rest.
// Translation incomplete: 0087a3e0 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new section) and the /GS cookie word are not translated.
pub fn int_seen_data_load_game(e: &mut Engine, this: Ptr<IntSeenData>, left: u32) {
    seen_data_load_game(e, this.cast(), 0xffff);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), left);
        if e.mem.u16(slot.addr()) == 0xffff {
            let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
            e.call(TES_SAVE_READ, &args![game, slot, 2u32]);
        }
        for offset in [0x24u32, 0x25] {
            let game = e.global::<u32>(TES_SAVE_LOAD_GAME);
            e.call(
                TES_SAVE_READ,
                &args![game, this.addr().wrapping_add(offset), 1u32],
            );
        }
        let rest = e.mem.u16(slot.addr()).wrapping_sub(1);
        e.mem.set_u16(slot.addr(), rest);
        if rest != 0 {
            let section = new_section(e);
            e.set(this, IntSeenData::pNextSection, section);
            let next = e.get(this, IntSeenData::pNextSection);
            e.vcall(next, 0x18, &args![u32::from(rest)]);
        }
    });
}

// Translated from 0087a570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::LoadGame_ov2` (Xbox PDB; `thiscall`, `RET 4`: the load
/// buffer): reads the section count (`00864a60`); for each section from
/// `this` its two bytes (`00864980`) and bits (`0087a0e0`), and before the
/// last a new section (`0087a100`) as the next one.
// Translation incomplete: 0087a570 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees a new section) and the /GS cookie word are not translated.
pub fn int_seen_data_load_game_ov2(e: &mut Engine, this: Ptr<IntSeenData>, buffer: u32) {
    let count = e.call(BUFFER_LOAD_VARIABLE, &args![buffer]).u32();
    let mut section = this.addr();
    let mut index = 0u32;
    while index < count {
        for offset in [0x24u32, 0x25] {
            e.call(
                BUFFER_LOAD_RAW,
                &args![buffer, section.wrapping_add(offset), 1u32],
            );
        }
        seen_data_load_game_ov2(e, Ptr::new(section), buffer);
        if index < count.wrapping_sub(1) {
            let next = new_section(e);
            e.mem.set_u32(section.wrapping_add(0x28), next);
        }
        section = e.mem.u32(section.wrapping_add(0x28));
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_99c0, seen_data_is_seen_bit_true(Ptr, u32, u32) -> bool),
        entry!(0x0087_9d90, fn_00879d90() -> f32),
        entry!(0x0087_9da0, fn_00879da0(u32, u8, u32, u32)),
        entry!(0x0087_9f20, seen_data_draw_seen_data(Ptr, u32, u32, u32)),
        entry!(0x0087_a030, fn_0087a030(Ptr, u32) -> u16),
        entry!(0x0087_a060, seen_data_save_game(Ptr, u32)),
        entry!(0x0087_a090, seen_data_load_game(Ptr, u32)),
        entry!(0x0087_a0e0, seen_data_load_game_ov2(Ptr, u32)),
        entry!(
            0x0087_a200,
            int_seen_data_draw_seen_data(Ptr<IntSeenData>, u32, u32, u8)
        ),
        entry!(
            0x0087_a2a0,
            int_seen_data_get_save_size(Ptr<IntSeenData>, u8) -> u16
        ),
        entry!(0x0087_a330, int_seen_data_save_game(Ptr<IntSeenData>, u8)),
        entry!(0x0087_a3e0, int_seen_data_load_game(Ptr<IntSeenData>, u32)),
        entry!(
            0x0087_a570,
            int_seen_data_load_game_ov2(Ptr<IntSeenData>, u32)
        ),
        entry!(0x0087_9970, seen_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0087_99a0, fn_008799a0(Ptr)),
        entry!(0x0087_99f0, seen_data_is_seen_bit_true_ov2(Ptr, u32) -> bool),
        entry!(0x0087_9a40, fn_00879a40(Ptr, u32, u8) -> bool),
        entry!(0x0087_9ad0, fn_00879ad0(Ptr, u32, u32) -> bool),
        entry!(0x0087_9bb0, fn_00879bb0() -> f32),
        entry!(0x0087_9bd0, fn_00879bd0(u32, u32)),
        entry!(0x0087_9ca0, fn_00879ca0(u32)),
        entry!(0x0087_a0c0, seen_data_save_game_ov2(Ptr, u32)),
        entry!(
            0x0087_a100,
            fn_0087a100(Ptr<IntSeenData>, u8, u8) -> Ptr<IntSeenData>
        ),
        entry!(
            0x0087_a140,
            int_seen_data_scalar_deleting_destructor(Ptr<IntSeenData>, u32) -> Ptr<IntSeenData>
        ),
        entry!(0x0087_a170, fn_0087a170(Ptr<IntSeenData>)),
        entry!(
            0x0087_a4e0,
            int_seen_data_save_game_ov2(Ptr<IntSeenData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_9970, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0087_9970, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn fn_008799a0_restores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x0087_99a0, &args![this]);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
    }

    #[test]
    fn is_seen_bit_true_reads_the_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 4, 0x8000_0001);
        e.mem.set_u32(this + 8, 0x0000_0004);
        assert!(e.call(0x0087_99f0, &args![this, 0u32]).bool());
        assert!(e.call(0x0087_99f0, &args![this, 31u32]).bool());
        assert!(!e.call(0x0087_99f0, &args![this, 1u32]).bool());
        assert!(e.call(0x0087_99f0, &args![this, 34u32]).bool());
        assert!(!e.call(0x0087_99f0, &args![this, 35u32]).bool());
    }

    #[test]
    fn fn_00879a40_sets_and_clears_and_answers_the_old_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        assert!(!e.call(0x0087_9a40, &args![this, 33u32, 1u8]).bool());
        assert_eq!(e.mem.u32(this + 8), 2);
        assert!(e.call(0x0087_9a40, &args![this, 33u32, 1u8]).bool());
        assert!(e.call(0x0087_9a40, &args![this, 33u32, 0u8]).bool());
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(!e.call(0x0087_9a40, &args![this, 33u32, 0u8]).bool());
    }

    #[test]
    fn fn_00879bb0_reads_the_seen_distance_setting() {
        let mut e = Engine::new();
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 512.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, a| {
            assert_eq!(a[0], SEEN_DISTANCE_SETTING);
            word(value)
        });
        assert_eq!(e.call(0x0087_9bb0, &[]).f64(), 512.0);
    }

    fn map_seen_globals(e: &mut Engine, size: f32, count: u32) {
        e.map(0x011a_3000, 0x1000);
        e.set_global(BIT_SQUARE_SIZE, size);
        e.set_global(BIT_COUNT, count);
        e.map(0x0101_6000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
    }

    fn point_constructor(e: &mut Engine) {
        e.register(POINT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            word(a[0])
        });
    }

    #[test]
    fn fn_00879bd0_builds_the_point_of_a_bit() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 256.0, 256);
        point_constructor(&mut e);
        let out = e.mem.alloc(0xc);
        e.call(0x0087_9bd0, &args![0x23u32, out]);
        assert_eq!(e.mem.f32(out), 768.0);
        assert_eq!(e.mem.f32(out + 4), 512.0);
        assert_eq!(e.mem.f32(out + 8), 0.0);
        e.call(0x0087_9bd0, &args![256u32, out]);
        for i in 0..3 {
            assert_eq!(e.mem.f32(out + 4 * i), f32::MAX);
        }
    }

    #[test]
    fn fn_00879ca0_snaps_the_point_to_its_square() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 256.0, 256);
        e.register(
            FLOAT_TO_INT,
            |_, a| word(f32::from_bits(a[0]) as i32 as u32),
        );
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 5000.0);
        e.mem.set_f32(point + 4, -100.0);
        e.mem.set_f32(point + 8, 77.0);
        e.call(0x0087_9ca0, &args![point]);
        // x: cell 4096 + 3 squares of 256 (904 / 256 = 3).
        assert_eq!(e.mem.f32(point), 4096.0 + 768.0);
        // y: -100 >> 12 is cell -1 (-4096); -100 + 4096 = 3996, 15 squares.
        assert_eq!(e.mem.f32(point + 4), -4096.0 + 15.0 * 256.0);
        assert_eq!(e.mem.u32(point + 8), 0);
    }

    #[test]
    fn fn_00879ad0_marks_near_pieces_and_compares_the_count() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 1.0, 4);
        point_constructor(&mut e);
        let this = e.mem.alloc(0x24);
        // Bit 0 already seen.
        e.mem.set_u32(this + 4, 1);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 2.5);
        let needed = e.mem.alloc(4);
        e.mem.set_u32(needed, 3);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| word(distance));
        e.register_double(INT_SETTING_VALUE, move |_, a| {
            assert_eq!(a[0], SEEN_COUNT_SETTING);
            word(needed)
        });
        e.register(POINT_PREPARE, |_, a| word(a[0]));
        e.register(POINT_ADD, |_, _| Ret::default());
        // The difference keeps the point (bit index as x).
        e.register(POINT_SUBTRACT, |e, a| {
            let x = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], x);
            word(a[1])
        });
        e.register(POINT_LENGTH, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        // Bits 1 and 2 are nearer than 2.5, bit 3 is not: 3 seen, 3 needed.
        assert!(e.call(0x0087_9ad0, &args![this, 0x10u32, 0x20u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 0b0111);
        assert_eq!(calls_to(&e, POINT_ADD).len(), 3);
        assert_eq!(calls_to(&e, POINT_ADD)[0][1], 0x20);
        assert_eq!(calls_to(&e, POINT_SUBTRACT)[0][2], 0x10);
        // With 4 needed the answer is false.
        e.mem.set_u32(needed, 4);
        assert!(!e.call(0x0087_9ad0, &args![this, 0x10u32, 0x20u32]).bool());
    }

    #[test]
    fn save_game_ov2_saves_the_bits() {
        let mut e = Engine::new();
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_a0c0, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls_to(&e, SAVE_DATA), vec![vec![0x9000, 0x4004, 0x20, 0]]);
    }

    #[test]
    fn int_seen_data_constructor_sets_its_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, 0x1234);
        e.register(SEEN_DATA_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a100, &args![this, 3u8, 0xfeu8]).u32(), this);
        assert_eq!(calls_to(&e, SEEN_DATA_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), INT_SEEN_DATA_VTABLE);
        assert_eq!(e.mem.u8(this + 0x24), 3);
        assert_eq!(e.mem.u8(this + 0x25), 0xfe);
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn int_seen_data_destructor_deletes_the_next_section() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        let next = e.mem.alloc(0x2c);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(next, 0x0300_0000);
        e.mem.set_u32(this + 0x28, next);
        e.register(0x0300_1000, |_, a| word(a[0]));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a140, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![next, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        // The body ends with the base vtable.
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        // Without a next section nothing is deleted through the vtable.
        e.mem.set_u32(this + 0x28, 0);
        e.call_log = Some(vec![]);
        e.call(0x0087_a170, &args![this]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
    }

    #[test]
    fn int_seen_data_save_game_saves_every_section() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x2c);
        let second = e.mem.alloc(0x2c);
        e.mem.set_u32(first + 0x28, second);
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| word(0x77));
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_a4e0, &args![first, 0x9000u32]);
        assert_eq!(calls_to(&e, START_VARIABLE_SIZED_VALUE), vec![vec![0x9000]]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x9000, first + 0x24, 1, 0],
                vec![0x9000, first + 0x25, 1, 0],
                vec![0x9000, first + 4, 0x20, 0],
                vec![0x9000, second + 0x24, 1, 0],
                vec![0x9000, second + 0x25, 1, 0],
                vec![0x9000, second + 4, 0x20, 0],
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 2, 0x77]]
        );
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Square size 256 and 256 bits, `011de45c` set.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.set_global::<f32>(BIT_SQUARE_SIZE, 256.0);
        e.set_global::<u32>(BIT_COUNT, 0x100);
        e.set_global::<f64>(TWO_DOUBLE, 2.0);
        e.set_global::<u32>(TES_SAVE_LOAD_GAME, 0x6000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_seen_bit_true_uses_rows_of_16() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        // Bit 2 * 16 + 3 = 35: word 1, bit 3.
        e.mem.set_u32(this + 8, 1 << 3);
        assert!(e.call(0x0087_99c0, &args![this, 3u32, 2u32]).bool());
        assert!(!e.call(0x0087_99c0, &args![this, 2u32, 3u32]).bool());
    }

    #[test]
    fn fn_00879d90_answers_the_square_size() {
        let mut e = engine();
        assert_eq!(e.call(0x0087_9d90, &args![]).f32(), 256.0);
    }

    #[test]
    fn fn_00879da0_finds_the_square_in_the_section() {
        let mut e = engine();
        e.register(FLOAT_TO_INT, |_, a| {
            (f32::from_bits(a[0]) as i32).into_ret()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 5000.0);
        e.mem.set_f32(point + 4, -1000.0);
        let out = e.mem.alloc(8);
        e.call(0x0087_9da0, &args![point, 0u32, out, out + 4]);
        // 5000 - 4096 = 904 -> 3; -1000 - (-4096) = 3096 -> 12.
        assert_eq!((e.mem.i32(out), e.mem.i32(out + 4)), (3, 12));
        e.call(0x0087_9da0, &args![point, 1u32, out, out + 4]);
        // Centred: (5000 - 2048) >> 12 = 0, 5000 - 2048 = 2952 -> 11;
        // (-1000 - 2048) >> 12 = -1, -1000 + 2048 = 1048 -> 4.
        assert_eq!((e.mem.i32(out), e.mem.i32(out + 4)), (11, 4));
    }

    #[test]
    fn draw_seen_data_places_a_rectangle_per_seen_bit() {
        let mut e = engine();
        for a in [COLOR_CONSTRUCT, POINT_PREPARE, POINT_ADD] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(POINT_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(MAKE_RECTANGLE, |_, _| 0x7700u32.into_ret());
        e.register(PLACE_RECTANGLE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x38]);
        e.mem.set_u32(0x0150_0000 + 0xdc, 0x0150_10dc);
        e.register(0x0150_10dc, |_, _| Ret::default());
        let node = e.mem.alloc(4);
        e.mem.set_u32(node, 0x0150_0000);
        let this = e.mem.alloc(0x24);
        // Bit 17: column 1, row 1.
        e.mem.set_u32(this + 4, 1 << 17);
        e.call(0x0087_9f20, &args![this, 0u32, 0x10u32, 0u32]);
        assert!(calls(&e, MAKE_RECTANGLE).is_empty());
        e.call(0x0087_9f20, &args![this, node, 0x10u32, 0u32]);
        let color = calls(&e, COLOR_CONSTRUCT)[0].clone();
        assert_eq!(color[1..], [1.0f32.to_bits(), 0, 0, 0]);
        assert_eq!(
            calls(&e, MAKE_RECTANGLE),
            vec![vec![256.0f32.to_bits(), color[0], 1]]
        );
        assert_eq!(calls(&e, 0x0150_10dc), vec![vec![node, 0x7700, 1]]);
        assert_eq!(
            calls(&e, PLACE_RECTANGLE),
            vec![vec![0x7700, 384.0f32.to_bits(), 384.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, POINT_ADD)[0][1], 0x10);
    }

    #[test]
    fn the_seen_data_save_and_load_calls() {
        let mut e = engine();
        for a in [TES_SAVE_WRITE, TES_SAVE_READ, BUFFER_LOAD_RAW] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x24);
        assert_eq!(e.call(0x0087_a030, &args![this, 9u32]).u16(), 0x20);
        e.call(0x0087_a060, &args![this, 9u32]);
        e.call(0x0087_a090, &args![this, 9u32]);
        e.call(0x0087_a0e0, &args![this, 0x8000u32]);
        assert_eq!(
            calls(&e, TES_SAVE_WRITE),
            vec![vec![0x6000, this + 4, 0x20]]
        );
        assert_eq!(calls(&e, TES_SAVE_READ), vec![vec![0x6000, this + 4, 0x20]]);
        assert_eq!(
            calls(&e, BUFFER_LOAD_RAW),
            vec![vec![0x8000, this + 4, 0x20]]
        );
    }

    fn section(e: &mut Engine, x: u8, y: u8, next: u32) -> u32 {
        let s = e.mem.alloc(0x2c);
        e.mem.set_u32(s, INT_SEEN_DATA_VTABLE);
        e.mem.set_u8(s + 0x24, x);
        e.mem.set_u8(s + 0x25, y);
        e.mem.set_u32(s + 0x28, next);
        s
    }

    /// The `IntSeenData` vtable mapped onto this unit's translations.
    fn int_vtable(e: &mut Engine) {
        e.put_vtable(
            INT_SEEN_DATA_VTABLE,
            &[0, 0x0087_a200, 0x0087_a2a0, 0, 0x0087_a330, 0, 0x0087_a3e0],
        );
    }

    #[test]
    fn int_draw_seen_data_draws_each_section_from_its_corner() {
        let mut e = engine();
        int_vtable(&mut e);
        e.register(POINT_CONSTRUCT, |_, a| a[0].into_ret());
        let corners: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let log = corners.clone();
        e.register_double(POINT_CONSTRUCT, move |_, a| {
            log.borrow_mut().push(a[1..].to_vec());
            a[0].into_ret()
        });
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point + 8, 7.0);
        let second = section(&mut e, 0xff, 2, 0);
        let first = section(&mut e, 1, 0, second);
        // No node: the draw itself does nothing.
        e.call(0x0087_a200, &args![first, 0u32, point, 1u32]);
        assert_eq!(
            *corners.borrow(),
            vec![
                vec![6144.0f32.to_bits(), 2048.0f32.to_bits(), 7.0f32.to_bits()],
                vec![
                    (-2048.0f32).to_bits(),
                    10240.0f32.to_bits(),
                    7.0f32.to_bits()
                ],
            ]
        );
    }

    #[test]
    fn int_get_save_size_adds_the_chain() {
        let mut e = engine();
        int_vtable(&mut e);
        let second = section(&mut e, 0, 0, 0);
        let first = section(&mut e, 0, 0, second);
        assert_eq!(e.call(0x0087_a2a0, &args![first, 0u32]).u16(), 0x24 + 0x22);
        assert_eq!(e.call(0x0087_a2a0, &args![first, 1u32]).u16(), 0x22 + 0x22);
    }

    #[test]
    fn int_save_game_writes_the_count_once() {
        let mut e = engine();
        int_vtable(&mut e);
        let written: Rc<RefCell<Vec<(u32, Vec<u8>)>>> = Rc::default();
        let log = written.clone();
        e.register_double(TES_SAVE_WRITE, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        let third = section(&mut e, 5, 6, 0);
        let second = section(&mut e, 3, 4, third);
        let first = section(&mut e, 1, 2, second);
        e.call(0x0087_a330, &args![first, 0u32]);
        let written = written.borrow();
        let sizes: Vec<usize> = written.iter().map(|w| w.1.len()).collect();
        assert_eq!(sizes, vec![0x20, 2, 1, 1, 0x20, 1, 1, 0x20, 1, 1]);
        assert_eq!(written[1].1, vec![3, 0]);
        assert_eq!(written[2], (first + 0x24, vec![1]));
        assert_eq!(written[9], (third + 0x25, vec![6]));
    }

    #[test]
    fn int_load_game_reads_the_count_and_chains_sections() {
        let mut e = engine();
        int_vtable(&mut e);
        e.register(TES_SAVE_READ, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 2);
            } else {
                e.mem.set_u8(a[1], 9);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SEEN_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let first = section(&mut e, 0, 0, 0);
        e.call(0x0087_a3e0, &args![first, 0xffffu32]);
        let second = e.mem.u32(first + 0x28);
        assert_ne!(second, 0);
        assert_eq!(e.mem.u32(second + 0x28), 0);
        assert_eq!(e.mem.u8(second + 0x24), 9);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x2c]]);
        // One read of the count, then 0x20 + 1 + 1 bytes per section.
        let reads: Vec<u32> = calls(&e, TES_SAVE_READ).iter().map(|c| c[2]).collect();
        assert_eq!(reads, vec![0x20, 2, 1, 1, 0x20, 1, 1]);
    }

    #[test]
    fn int_load_game_ov2_builds_the_chain() {
        let mut e = engine();
        e.register(BUFFER_LOAD_VARIABLE, |_, _| 3u32.into_ret());
        e.register(BUFFER_LOAD_RAW, |e, a| {
            e.mem.set_u8(a[1], a[2] as u8);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SEEN_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let first = section(&mut e, 0, 0, 0);
        e.call(0x0087_a570, &args![first, 0x8000u32]);
        let second = e.mem.u32(first + 0x28);
        let third = e.mem.u32(second + 0x28);
        assert!(second != 0 && third != 0);
        assert_eq!(e.mem.u32(third + 0x28), 0);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 2);
        assert_eq!(calls(&e, BUFFER_LOAD_RAW).len(), 9);
        assert_eq!(e.mem.u8(third + 0x25), 1);
    }
}

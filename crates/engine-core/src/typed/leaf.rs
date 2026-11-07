//! The first typed functions (T2 of the migration, docs/ENGINE_CRATE.md, "Typed code"): small functions
//! that call nothing and are called most often. A translation reaches them through
//! [`typed_call!`](crate::typed_call), so with the family on the call by address goes away; everything
//! else (vtables, callback words, calls not rewritten) still reaches them at their exe addresses.
//!
//! Identical code folding makes most of these one body for many classes (`word_at_0` serves about 1,800
//! callers), so a parameter here is whatever word the caller holds: a `Ptr<T>` of any class or a plain
//! `u32`. Words read here are returned as words, as the translations hold them.
//!
//! The family's row is [`Threads::Both`] until a per-thread table says otherwise, so the two that compute
//! in x87 ([`point3_dot`], [`plane_side`]) keep the exact helpers: plain `f32` only matches the exe under
//! the main thread's control word, and `f64` rounded at the store only under the other threads'.

use super::{Family, FamilyId, Threads};
use crate::abi::{AbiFn, OneWord};
use crate::ptr::Ptr;
use crate::units::crt::{set_x87_control_word, x87_control_word};
use crate::x87::{x87_add, x87_load32, x87_m32, x87_mul, x87_quiet_bits, x87_store32, x87_sub};
use crate::Engine;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(0);

/// `NV_TYPED=leaves`. Past its gate G1 (2026-11-06: Goodsprings +7.1%, Doc's room +3.0%, both pairs up): on
/// unless `NV_TYPED` says otherwise (`NV_TYPED=off` or `leaves:off`).
pub static LEAVES: Family = Family {
    name: "leaves",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// The low 16 bits of a word, what `FLDCW` loads from it.
#[inline(always)]
fn low_half(word: u32) -> u16 {
    #[allow(clippy::cast_possible_truncation)]
    {
        word as u16
    }
}

/// The low 8 bits of a word, what a byte store or byte compare of it sees.
#[inline(always)]
fn low_byte(word: u32) -> u8 {
    #[allow(clippy::cast_possible_truncation)]
    {
        word as u8
    }
}

// Typed from 00559450
/// The word at +0 of whatever `this` is: a list's first element, among many others.
#[inline(always)]
pub fn word_at_0(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word())
}

// Typed from 00726070
/// The word at +4: a list node's next pointer, an inventory entry's count.
#[inline(always)]
pub fn word_at_4(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(4))
}

// Typed from 0044ddc0
/// The word at +8.
#[inline(always)]
pub fn word_at_8(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(8))
}

// Typed from 0084e3a0
/// The word at +0xc.
#[inline(always)]
pub fn word_at_c(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(0xc))
}

// Typed from 007af430
/// The word at +0x20.
#[inline(always)]
pub fn word_at_20(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(0x20))
}

// Typed from 0045cd60
/// The word at +0x28.
#[inline(always)]
pub fn word_at_28(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(0x28))
}

// Typed from 00671d10
/// The word at +0x30.
#[inline(always)]
pub fn word_at_30(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(0x30))
}

// Typed from 008d8520
/// The word at +0x68.
#[inline(always)]
pub fn word_at_68(e: &mut Engine, this: impl OneWord) -> u32 {
    e.mem.u32(this.one_word().wrapping_add(0x68))
}

// Typed from 00437bb0
/// The byte at +0x87.
#[inline(always)]
pub fn byte_at_87(e: &mut Engine, this: impl OneWord) -> u8 {
    e.mem.u8(this.one_word().wrapping_add(0x87))
}

// Typed from 006815c0
/// `this` itself.
#[inline(always)]
pub fn same_word(_e: &mut Engine, this: impl OneWord) -> u32 {
    this.one_word()
}

// Typed from 0040fbe0
/// Nothing at all. Many callers push words (it is the release build's empty error report as well as
/// other empty functions), and it reads none: `typed_call!`'s `; words` form passes them by address.
#[inline(always)]
pub fn nothing(_e: &mut Engine) {}

// Typed from 00acbb70
/// 0, with `this` unread.
#[inline(always)]
pub fn zero_of_this(_e: &mut Engine, _this: impl OneWord) -> u32 {
    0
}

// Typed from 00e7cd40
/// 0, with the word unread.
#[inline(always)]
pub fn zero(_e: &mut Engine, _this: impl OneWord) -> u32 {
    0
}

// Typed from 00416870
/// A point's constructor: three floats through the x87 stack to +0, +4 and +8, each `FLD` quieting a
/// signalling NaN. Returns `this`.
#[inline(always)]
pub fn point3_new(
    e: &mut Engine,
    this: impl OneWord,
    x: impl OneWord,
    y: impl OneWord,
    z: impl OneWord,
) -> u32 {
    let this = this.one_word();
    e.mem.set_u32(this, x87_quiet_bits(x.one_word()));
    e.mem
        .set_u32(this.wrapping_add(4), x87_quiet_bits(y.one_word()));
    e.mem
        .set_u32(this.wrapping_add(8), x87_quiet_bits(z.one_word()));
    this
}

// Typed from 0043d450
/// The object's bound (the pointer at +0x20), or the empty bound when it has none. The exe reads the
/// field twice, with nothing between; one read does the same.
#[inline(always)]
pub fn world_bound(e: &mut Engine, this: impl OneWord) -> u32 {
    /// The shared empty bound.
    const EMPTY_BOUND: u32 = 0x011f_4288;
    let bound = e.mem.u32(this.one_word().wrapping_add(0x20));
    if bound == 0 {
        EMPTY_BOUND
    } else {
        bound
    }
}

// Typed from 00877a30
/// An array's element address: the base at +4, plus `index` words.
#[inline(always)]
pub fn array_element(e: &mut Engine, this: impl OneWord, index: impl OneWord) -> u32 {
    let base = e.mem.u32(this.one_word().wrapping_add(4));
    base.wrapping_add(index.one_word().wrapping_mul(4))
}

// Typed from 00440d80
/// Whether bit 0x20 of the form flags at +8 is set.
#[inline(always)]
pub fn form_flag_20(e: &mut Engine, this: impl OneWord) -> bool {
    e.mem.u32(this.one_word().wrapping_add(8)) & 0x20 != 0
}

// Typed from 00440da0
/// Whether bit 0x800 of the form flags at +8 is set.
#[inline(always)]
pub fn form_flag_800(e: &mut Engine, this: impl OneWord) -> bool {
    e.mem.u32(this.one_word().wrapping_add(8)) & 0x800 != 0
}

// Typed from 00576d30
/// Whether bit 0x200000 of the form flags at +8 is set.
#[inline(always)]
pub fn form_flag_200000(e: &mut Engine, this: impl OneWord) -> bool {
    e.mem.u32(this.one_word().wrapping_add(8)) & 0x0020_0000 != 0
}

// Typed from 00456630
/// Whether the flag word at +0x30 has any bit of `mask`.
#[inline(always)]
pub fn flags_at_30_any(e: &mut Engine, this: impl OneWord, mask: impl OneWord) -> bool {
    e.mem.u32(this.one_word().wrapping_add(0x30)) & mask.one_word() != 0
}

// Typed from 008256d0
/// Whether the words at +4 and +0 are both zero, read in that order and the second only when the first
/// is: an empty list.
#[inline(always)]
pub fn both_words_zero(e: &mut Engine, this: impl OneWord) -> bool {
    let this = this.one_word();
    e.mem.u32(this.wrapping_add(4)) == 0 && e.mem.u32(this) == 0
}

// Typed from 004e20c0
/// Four words into entry `index` of the 16-byte table of the water's colours.
#[inline(always)]
pub fn store_colour(
    e: &mut Engine,
    index: impl OneWord,
    red: impl OneWord,
    green: impl OneWord,
    blue: impl OneWord,
    alpha: impl OneWord,
) {
    /// The table.
    const COLOURS: u32 = 0x011f_a0c0;
    let entry = (index.one_word() << 4).wrapping_add(COLOURS);
    e.mem.set_u32(entry, red.one_word());
    e.mem.set_u32(entry.wrapping_add(4), green.one_word());
    e.mem.set_u32(entry.wrapping_add(8), blue.one_word());
    e.mem.set_u32(entry.wrapping_add(12), alpha.one_word());
}

// Typed from 004b6190
/// Dot product of the points at `this` and `other`: each product and sum rounded as the running thread's
/// control word says, in the exe's order, stored to a `float` and loaded back. Exact helpers: see the
/// module's comment.
#[inline(always)]
pub fn point3_dot(e: &mut Engine, this: impl OneWord, other: impl OneWord) -> f32 {
    let (a, b) = (this.one_word(), other.one_word());
    let x = x87_mul(x87_load32(e.mem.u32(a)), x87_m32(e.mem.u32(b)));
    let y = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(4))),
        x87_m32(e.mem.u32(b.wrapping_add(4))),
    );
    let sum = x87_add(x, y);
    let z = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(8))),
        x87_m32(e.mem.u32(b.wrapping_add(8))),
    );
    f32::from_bits(x87_store32(x87_add(sum, z)))
}

// Typed from 0049da80
/// Which side of the plane at `plane` the point at `point` is on: 2 behind, 1 in front, else 0, also for
/// a NaN. The distance rounds as [`point3_dot`]'s sum does and goes through a `float` local; it compares
/// twice against the double zero the exe keeps in its constants.
#[inline(always)]
pub fn plane_side(e: &mut Engine, plane: impl OneWord, point: impl OneWord) -> u32 {
    /// The constant both compares read.
    const ZERO_DOUBLE: u32 = 0x0101_2060;
    let (p, q) = (plane.one_word(), point.one_word());
    let x = x87_mul(x87_load32(e.mem.u32(p)), x87_m32(e.mem.u32(q)));
    let y = x87_mul(
        x87_load32(e.mem.u32(p.wrapping_add(4))),
        x87_m32(e.mem.u32(q.wrapping_add(4))),
    );
    let sum = x87_add(x, y);
    let z = x87_mul(
        x87_load32(e.mem.u32(p.wrapping_add(8))),
        x87_m32(e.mem.u32(q.wrapping_add(8))),
    );
    let sum = x87_add(sum, z);
    let distance = x87_store32(x87_sub(sum, x87_m32(e.mem.u32(p.wrapping_add(0xc)))));
    let distance = x87_load32(distance);
    if distance < e.global::<f64>(ZERO_DOUBLE) {
        2
    } else if distance > e.global::<f64>(ZERO_DOUBLE) {
        1
    } else {
        0
    }
}

// Typed from 00ed5abb
/// `_ctrlfp(new, mask)`: reads the x87 control word, loads `(old & !mask) | (new & mask)` and returns
/// the old word sign-extended.
#[inline(always)]
pub fn ctrlfp(e: &mut Engine, new: impl OneWord, mask: impl OneWord) -> i32 {
    let mask = mask.one_word();
    let old = x87_control_word(e);
    let word = (!mask & u32::from(old)) | (new.one_word() & mask);
    set_x87_control_word(e, low_half(word));
    i32::from(i16::from_ne_bytes(old.to_ne_bytes()))
}

// Typed from 00ed2795
/// The math bodies' control word load: `(saved & 0x300) | 0x7f`, the caller's precision with everything
/// else at its default.
#[inline(always)]
pub fn load_math_control_word(e: &mut Engine, saved: impl OneWord) {
    set_x87_control_word(e, low_half((saved.one_word() & 0x300) | 0x7f));
}

// Typed from 00ed5ae4
/// The status flag helper of the math bodies' exception path. Each set bit runs an x87 operation for its
/// status flag, which the engine doesn't model; of those only three read memory, and those reads stay.
#[inline(always)]
pub fn raise_status_flags(e: &mut Engine, flags: impl OneWord) {
    /// The huge and the tiny extended constants the operations load.
    const HUGE: u32 = 0x011c_2520;
    const TINY: u32 = 0x011c_252c;
    let flags = flags.one_word();
    if flags & 1 != 0 {
        let _ = e.mem.bytes(HUGE, 10);
    }
    if flags & 8 != 0 {
        let _ = e.mem.bytes(HUGE, 10);
    }
    if flags & 0x10 != 0 {
        let _ = e.mem.bytes(TINY, 10);
    }
}

// Typed from 00ed2808
/// The register entries' exponent test: `high & 0x7ff00000`, or all of `high` when that is all ones.
#[inline(always)]
pub fn double_exponent(_e: &mut Engine, _low: impl OneWord, high: impl OneWord) -> u32 {
    let high = high.one_word();
    let exponent = high & 0x7ff0_0000;
    if exponent == 0x7ff0_0000 {
        high
    } else {
        exponent
    }
}

// Typed from 00e890c0
/// A renderer state's value, by the slot table at `0126f92c`; slots 5 and up do nothing. `enable` picks
/// the marked byte, value word and count of its side (+0x80, +0x6c, +0x64 set; +0xa4, +0x90, +0x88
/// clear). A slot marked there just takes the value and updates the second flag: the count at +0xac goes
/// up when the byte at +0xb0 goes from clear to set, down the other way. A slot marked on the other side
/// is unmarked first, then marked here, counted, and the second flag stored.
#[inline(always)]
pub fn set_state_slot(
    e: &mut Engine,
    this: impl OneWord,
    state: impl OneWord,
    value: impl OneWord,
    enable: impl OneWord,
    extra: impl OneWord,
) {
    /// The 16-bit slot of each state.
    const SLOTS: u32 = 0x0126_f92c;
    let base = this.one_word();
    let value = value.one_word();
    let enable = low_byte(enable.one_word());
    let extra = low_byte(extra.one_word());
    let slot = u32::from(
        e.mem
            .u16(SLOTS.wrapping_add(state.one_word().wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let add = |e: &mut Engine, offset: u32, delta: u32| {
        let at = base.wrapping_add(offset);
        let counted = e.mem.u32(at).wrapping_add(delta);
        e.mem.set_u32(at, counted);
    };
    let at = |offset: u32| base.wrapping_add(slot + offset);
    // (marked byte, value word, count) of the side `enable` picks, then the other side's.
    let (mine, word, count, other, other_count) = if enable != 0 {
        (0x80u32, 0x6cu32, 0x64u32, 0xa4u32, 0x88u32)
    } else {
        (0xa4, 0x90, 0x88, 0x80, 0x64)
    };
    let word = base.wrapping_add(word + slot * 4);
    if e.mem.u8(at(mine)) != 0 {
        e.mem.set_u32(word, value);
        let extra_byte = at(0xb0);
        if extra != 0 {
            if e.mem.u8(extra_byte) == 0 {
                add(e, 0xac, 1);
            }
        } else if e.mem.u8(extra_byte) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(extra_byte, extra);
        return;
    }
    if e.mem.u8(at(other)) != 0 {
        e.mem.set_u8(at(other), 0);
        add(e, other_count, 0xffff_ffff);
        if e.mem.u8(at(0xb0)) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(at(0xb0), 0);
    }
    e.mem.set_u8(at(mine), 1);
    e.mem.set_u32(word, value);
    add(e, count, 1);
    if extra != 0 {
        add(e, 0xac, 1);
    }
    e.mem.set_u8(at(0xb0), extra);
}

/// The family's functions at their exe addresses.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        crate::entry!(0x0055_9450, word_at_0(Ptr) -> u32),
        crate::entry!(0x0072_6070, word_at_4(Ptr) -> u32),
        crate::entry!(0x0044_ddc0, word_at_8(Ptr) -> u32),
        crate::entry!(0x0084_e3a0, word_at_c(Ptr) -> u32),
        crate::entry!(0x007a_f430, word_at_20(Ptr) -> u32),
        crate::entry!(0x0045_cd60, word_at_28(Ptr) -> u32),
        crate::entry!(0x0067_1d10, word_at_30(Ptr) -> u32),
        crate::entry!(0x008d_8520, word_at_68(Ptr) -> u32),
        crate::entry!(0x0043_7bb0, byte_at_87(Ptr) -> u8),
        crate::entry!(0x0068_15c0, same_word(Ptr) -> u32),
        crate::entry!(0x0040_fbe0, nothing()),
        crate::entry!(0x00ac_bb70, zero_of_this(Ptr) -> u32),
        crate::entry!(0x00e7_cd40, zero(Ptr) -> u32),
        crate::entry!(0x0041_6870, point3_new(Ptr, f32, f32, f32) -> u32),
        crate::entry!(0x0043_d450, world_bound(Ptr) -> u32),
        crate::entry!(0x0087_7a30, array_element(Ptr, u32) -> u32),
        crate::entry!(0x0044_0d80, form_flag_20(Ptr) -> bool),
        crate::entry!(0x0044_0da0, form_flag_800(Ptr) -> bool),
        crate::entry!(0x0057_6d30, form_flag_200000(Ptr) -> bool),
        crate::entry!(0x0045_6630, flags_at_30_any(Ptr, u32) -> bool),
        crate::entry!(0x0082_56d0, both_words_zero(Ptr) -> bool),
        crate::entry!(0x004e_20c0, store_colour(u32, u32, u32, u32, u32)),
        crate::entry!(0x004b_6190, point3_dot(Ptr, u32) -> f32),
        crate::entry!(0x0049_da80, plane_side(Ptr, Ptr) -> u32),
        crate::entry!(0x00ed_5abb, ctrlfp(u32, u32) -> i32),
        crate::entry!(0x00ed_2795, load_math_control_word(u32)),
        crate::entry!(0x00ed_5ae4, raise_status_flags(u32)),
        crate::entry!(0x00ed_2808, double_exponent(u32, u32) -> u32),
        crate::entry!(0x00e8_90c0, set_state_slot(Ptr, u32, u32, u32, u32)),
    ]
}

//! `fallout shared/run_0044a880` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Members of the debug-heap `std::basic_string<char>` (Visual Studio 2008
//! library, the `DebugString` layout of `tesobject.rs`: storage union at
//! +4, `_Mysize` at +0x14, `_Myres` at +0x18) and two small helpers the
//! linker placed next to them. The engine map gives no names for this run;
//! the names below are the library members these bodies match (the same
//! names `tesobject.rs` gives these addresses as its callees).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesobject::DebugString;

/// `basic_string::_Tidy(built, newsize)` (`tesobject.rs`).
const STRING_TIDY: u32 = 0x0044_a730;
/// `_Xlen` (`00ec30b0`): throws `length_error`; never returns in the game.
const STRING_TOO_LONG: u32 = 0x00ec_30b0;
/// The string's allocator `max_size()` (`thiscall` on the string).
const ALLOCATOR_MAX_SIZE: u32 = 0x0065_f7f0;
/// The string's allocator `allocate(count)` (`thiscall` on the string).
const ALLOCATOR_ALLOCATE: u32 = 0x0065_f7a0;
/// `_Traits_helper::copy_s(dest, destsize, src, count)` (cdecl).
const COPY_CHARS: u32 = 0x0044_ccd0;
/// The cdecl helper `0044abd0` forwards to (two words).
const FORWARD_TARGET: u32 = 0x0044_d670;

// Translated from 0044a880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Eos(newsize)`: stores the length and writes the
/// terminating `'\0'` at `_Myptr() + newsize`.
pub fn std_string_eos(e: &mut Engine, this: Ptr<DebugString>, new_size: u32) {
    e.set(this, DebugString::size, new_size);
    e.with_stack(4, |e, terminator| {
        e.mem.set_u8(terminator.addr(), 0);
        let end = std_string_myptr(e, this).addr().wrapping_add(new_size);
        char_traits_assign(e, Ptr::new(end), terminator);
    });
}

// Translated from 0044a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `char_traits<char>::assign(left, right)` (cdecl): `*left = *right`.
pub fn char_traits_assign(e: &mut Engine, left: Ptr, right: Ptr) {
    let value = e.mem.u8(right.addr());
    e.mem.set_u8(left.addr(), value);
}

// Translated from 0044a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Grow(newsize, trim)`: `_Xlen` when `newsize` exceeds
/// `max_size()`; reallocates (`_Copy`) when the reserve is too small; with
/// `trim` and a size under 16, returns to the inline buffer (`_Tidy`);
/// an empty request just terminates the string. Returns `newsize > 0`.
pub fn std_string_grow(e: &mut Engine, this: Ptr<DebugString>, new_size: u32, trim: bool) -> bool {
    if std_string_max_size(e, this) < new_size {
        e.call(STRING_TOO_LONG, &[]);
    }
    if e.get(this, DebugString::reserve) < new_size {
        let old_size = e.get(this, DebugString::size);
        std_string_copy(e, this, new_size, old_size);
    } else if trim && new_size < 0x10 {
        let size = e.get(this, DebugString::size);
        let keep = if new_size < size { new_size } else { size };
        e.call(STRING_TIDY, &args![this, 1u32, keep]);
    } else if new_size == 0 {
        std_string_eos(e, this, 0);
    }
    new_size > 0
}

// Translated from 0044a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Myptr()`: the heap buffer once the reserve is 16 or
/// more, otherwise the inline buffer at +4.
pub fn std_string_myptr(e: &mut Engine, this: Ptr<DebugString>) -> Ptr {
    if e.get(this, DebugString::reserve) >= 0x10 {
        e.get(this, DebugString::buffer_pointer)
    } else {
        Ptr::new(this.addr().wrapping_add(4))
    }
}

// Translated from 0044a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::max_size()`: the allocator's `max_size()` less one for
/// the terminator, at least 1.
pub fn std_string_max_size(e: &mut Engine, this: Ptr<DebugString>) -> u32 {
    let count = e.call(ALLOCATOR_MAX_SIZE, &args![this]).u32();
    if count <= 1 {
        1
    } else {
        count - 1
    }
}

// Translated from 0044aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Copy(newsize, oldlen)`: picks the new reserve
/// (`newsize | 0xF`, or `newsize` when that exceeds `max_size()`, or 1.5
/// times the old reserve when that is larger and fits), allocates
/// `reserve + 1` bytes, copies the first `oldlen` characters, frees the old
/// buffer (`_Tidy(true, 0)`) and installs the new one with length `oldlen`.
// Translation incomplete: 0044aa10 - the try/catch around the allocation (FS:[0] frame; handler 0044aac8 retries with newsize + 1, else _Tidy(true, 0) and rethrows) is not translated
pub fn std_string_copy(e: &mut Engine, this: Ptr<DebugString>, new_size: u32, old_length: u32) {
    let mut new_reserve = new_size | 0xf;
    if std_string_max_size(e, this) < new_reserve {
        new_reserve = new_size;
    } else if new_reserve / 3 < e.get(this, DebugString::reserve) >> 1 {
        let half = e.get(this, DebugString::reserve) >> 1;
        let room = std_string_max_size(e, this).wrapping_sub(half);
        if e.get(this, DebugString::reserve) <= room {
            let reserve = e.get(this, DebugString::reserve);
            new_reserve = (reserve >> 1).wrapping_add(reserve);
        }
    }
    let buffer = e
        .call(
            ALLOCATOR_ALLOCATE,
            &args![this, new_reserve.wrapping_add(1)],
        )
        .u32();
    if old_length > 0 {
        let source = std_string_myptr(e, this);
        e.call(
            COPY_CHARS,
            &args![buffer, new_reserve.wrapping_add(1), source, old_length],
        );
    }
    e.call(STRING_TIDY, &args![this, 1u32, 0u32]);
    e.set(this, DebugString::buffer_pointer, Ptr::new(buffer));
    e.set(this, DebugString::reserve, new_reserve);
    std_string_eos(e, this, old_length);
}

// Translated from 0044ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `basic_string::_Inside(ptr)`: whether `text` points into the string's
/// characters (`_Myptr() <= text < _Myptr() + size`); false for null.
pub fn std_string_inside(e: &mut Engine, this: Ptr<DebugString>, text: Ptr) -> bool {
    if text.is_null() || text.addr() < std_string_myptr(e, this).addr() {
        return false;
    }
    let end = std_string_myptr(e, this)
        .addr()
        .wrapping_add(e.get(this, DebugString::size));
    end > text.addr()
}

// Translated from 0044abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `thiscall` member that ignores `this` and calls the cdecl `0044d670` (the allocator's
/// allocate) with its argument and 0, answering its `EAX` (untouched after the call).
pub fn fn_0044abd0(e: &mut Engine, _this: Ptr, value: u32) -> u32 {
    e.call(FORWARD_TARGET, &args![value, 0u32]).u32()
}

// Translated from 0044abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An allocator `max_size()` body returning `0x3FFFFFFF` (the
/// library's `count > 0 ? count : 1`, whose second arm the constant makes
/// unreachable but which is translated as written).
pub fn fn_0044abf0(_e: &mut Engine, _this: Ptr) -> u32 {
    let count: u32 = std::hint::black_box(0x3fff_ffff);
    if count > 0 {
        count
    } else {
        1
    }
}

// Translated from 0044ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies one word: `*this = *source` (a one-word object's copy).
pub fn fn_0044ac20(e: &mut Engine, this: Ptr, source: Ptr) {
    let value = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), value);
}

/// `0044a450` (`thiscall`, one word: the source; unnamed in the engine
/// map): the base class's copy constructor `0044a9a0` runs first.
const BASE_COPY_0044A450: u32 = 0x0044_a450;
/// The vtable `0044a9a0` stores (RTTI `.?AVlength_error@std@@`).
const LENGTH_ERROR_VTABLE: u32 = 0x0101_73dc;

// Translated from 0044a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `std::length_error` copy constructor (by the vtable it stores,
/// whose RTTI name is `length_error`; `RET 4`: the source): the base's
/// copy (`0044a450`), then the vtable. Returns `this`.
pub fn fn_0044a9a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(BASE_COPY_0044A450, &args![this, source]);
    e.mem.set_u32(this.addr(), LENGTH_ERROR_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_a9a0, fn_0044a9a0(Ptr, Ptr) -> Ptr),
        entry!(0x0044a880, std_string_eos(Ptr<DebugString>, u32)),
        entry!(0x0044a8c0, char_traits_assign(Ptr, Ptr)),
        entry!(
            0x0044a8d0,
            std_string_grow(Ptr<DebugString>, u32, bool) -> bool
        ),
        entry!(0x0044a970, std_string_myptr(Ptr<DebugString>) -> Ptr),
        entry!(0x0044a9d0, std_string_max_size(Ptr<DebugString>) -> u32),
        entry!(0x0044aa10, std_string_copy(Ptr<DebugString>, u32, u32)),
        entry!(0x0044ab90, std_string_inside(Ptr<DebugString>, Ptr) -> bool),
        entry!(0x0044abd0, fn_0044abd0(Ptr, u32) -> u32),
        entry!(0x0044abf0, fn_0044abf0(Ptr) -> u32),
        entry!(0x0044ac20, fn_0044ac20(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where the allocator double puts a new buffer.
    const NEW_BUFFER: u32 = 0x0900_0000;

    /// A string with the given length and reserve (inline buffer below 16).
    fn string(e: &mut Engine, size: u32, reserve: u32) -> Ptr<DebugString> {
        let s: Ptr<DebugString> = e.new_object();
        e.set(s, DebugString::size, size);
        e.set(s, DebugString::reserve, reserve);
        s
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATOR_MAX_SIZE, |_, _| 100u32.into_ret());
        e.register(STRING_TIDY, |_, _| Ret::default());
        e.register(STRING_TOO_LONG, |_, _| Ret::default());
        e.register(COPY_CHARS, |_, _| Ret::default());
        e.register(ALLOCATOR_ALLOCATE, |_, _| NEW_BUFFER.into_ret());
        e.map(NEW_BUFFER, 0x1000);
        e
    }

    #[test]
    fn eos_sets_the_length_and_terminates() {
        let mut e = engine();
        let s = string(&mut e, 0, 0xf);
        e.mem.set_u32(s.addr() + 4, 0x4141_4141);
        e.call(0x0044_a880, &args![s, 2u32]);
        assert_eq!(e.get(s, DebugString::size), 2);
        assert_eq!(e.mem.u8(s.addr() + 6), 0);
        assert_eq!(e.mem.u8(s.addr() + 5), 0x41);
    }

    #[test]
    fn assign_copies_one_char() {
        let mut e = engine();
        let a = e.mem.alloc(2);
        e.mem.set_u8(a + 1, b'x');
        e.call(0x0044_a8c0, &args![a, a + 1]);
        assert_eq!(e.mem.u8(a), b'x');
    }

    #[test]
    fn grow_picks_copy_tidy_or_eos() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        // Reserve too small: _Copy (allocates).
        let s = string(&mut e, 3, 0xf);
        assert!(e.call(0x0044_a8d0, &args![s, 20u32, false]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == ALLOCATOR_ALLOCATE));
        assert!(!log.iter().any(|(a, _)| *a == STRING_TOO_LONG));
        // Trim with a short size: _Tidy(true, min(newsize, size)).
        let s = string(&mut e, 3, 0x1f);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_a8d0, &args![s, 5u32, true]).bool());
        let log = e.call_log.take().unwrap();
        let tidy = log.iter().find(|(a, _)| *a == STRING_TIDY).unwrap();
        assert_eq!(tidy.1, vec![s.addr(), 1, 3]);
        // Zero without trim: terminated, false.
        let s = string(&mut e, 3, 0xf);
        assert!(!e.call(0x0044_a8d0, &args![s, 0u32, false]).bool());
        assert_eq!(e.get(s, DebugString::size), 0);
        // Past max_size: _Xlen is called.
        let s = string(&mut e, 0, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x0044_a8d0, &args![s, 200u32, false]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == STRING_TOO_LONG));
    }

    #[test]
    fn myptr_is_inline_below_16() {
        let mut e = engine();
        let s = string(&mut e, 0, 0xf);
        assert_eq!(e.call(0x0044_a970, &args![s]).u32(), s.addr() + 4);
        let s = string(&mut e, 0, 0x10);
        e.set(s, DebugString::buffer_pointer, Ptr::new(0x1234));
        assert_eq!(e.call(0x0044_a970, &args![s]).u32(), 0x1234);
    }

    #[test]
    fn max_size_is_allocator_less_one_at_least_one() {
        let mut e = engine();
        let s = string(&mut e, 0, 0xf);
        assert_eq!(e.call(0x0044_a9d0, &args![s]).u32(), 99);
        e.register(ALLOCATOR_MAX_SIZE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0044_a9d0, &args![s]).u32(), 1);
        e.register(ALLOCATOR_MAX_SIZE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0044_a9d0, &args![s]).u32(), 1);
    }

    #[test]
    fn copy_chooses_the_reserve_and_installs_the_buffer() {
        let mut e = engine();
        // newsize | 0xF.
        let s = string(&mut e, 2, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x0044_aa10, &args![s, 0x12u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let alloc = log.iter().find(|(a, _)| *a == ALLOCATOR_ALLOCATE).unwrap();
        assert_eq!(alloc.1, vec![s.addr(), 0x20]);
        let copy = log.iter().find(|(a, _)| *a == COPY_CHARS).unwrap();
        assert_eq!(copy.1, vec![NEW_BUFFER, 0x20, s.addr() + 4, 2]);
        let tidy = log.iter().find(|(a, _)| *a == STRING_TIDY).unwrap();
        assert_eq!(tidy.1, vec![s.addr(), 1, 0]);
        assert_eq!(e.get(s, DebugString::reserve), 0x1f);
        assert_eq!(e.get(s, DebugString::buffer_pointer).addr(), NEW_BUFFER);
        assert_eq!(e.get(s, DebugString::size), 2);
        assert_eq!(e.mem.u8(NEW_BUFFER + 2), 0);
        // 1.5 times the old reserve (0x40 -> 0x60), no copy for length 0.
        let s = string(&mut e, 0, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0044_aa10, &args![s, 0x41u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == COPY_CHARS));
        assert_eq!(e.get(s, DebugString::reserve), 0x60);
        // Beyond max_size: exactly newsize.
        let s = string(&mut e, 0, 0xf);
        e.call(0x0044_aa10, &args![s, 0x63u32, 0u32]);
        assert_eq!(e.get(s, DebugString::reserve), 0x63);
        // 1.5 times would not fit under max_size: keeps newsize | 0xF.
        let s = string(&mut e, 0, 0x50);
        e.call(0x0044_aa10, &args![s, 0x51u32, 0u32]);
        assert_eq!(e.get(s, DebugString::reserve), 0x5f);
    }

    #[test]
    fn inside_checks_the_character_range() {
        let mut e = engine();
        let s = string(&mut e, 4, 0xf);
        let base = s.addr() + 4;
        assert!(!e.call(0x0044_ab90, &args![s, 0u32]).bool());
        assert!(!e.call(0x0044_ab90, &args![s, base - 1]).bool());
        assert!(e.call(0x0044_ab90, &args![s, base]).bool());
        assert!(e.call(0x0044_ab90, &args![s, base + 3]).bool());
        assert!(!e.call(0x0044_ab90, &args![s, base + 4]).bool());
    }

    #[test]
    fn forwarder_passes_the_value_and_zero() {
        let mut e = engine();
        e.register(FORWARD_TARGET, |_, _| 0x5000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_abd0, &args![0x10u32, 0x77u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == FORWARD_TARGET).unwrap();
        assert_eq!(call.1, vec![0x77, 0]);
    }

    #[test]
    fn element_max_size_is_constant() {
        let mut e = engine();
        assert_eq!(e.call(0x0044_abf0, &args![0u32]).u32(), 0x3fff_ffff);
    }

    #[test]
    fn word_copy_copies_the_word() {
        let mut e = engine();
        let a = e.mem.alloc(8);
        e.mem.set_u32(a + 4, 0xdead_beef);
        e.call(0x0044_ac20, &args![a, a + 4]);
        assert_eq!(e.mem.u32(a), 0xdead_beef);
    }

    #[test]
    fn forwarder_answers_the_allocation() {
        let mut e = engine();
        e.register(FORWARD_TARGET, |_, _| 0x6000u32.into_ret());
        assert_eq!(e.call(0x0044_abd0, &args![0x10u32, 0x77u32]).u32(), 0x6000);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    #[test]
    fn length_error_copy_runs_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_COPY_0044A450, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_a9a0, &args![this, 0x7000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LENGTH_ERROR_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_COPY_0044A450, vec![this, 0x7000])));
    }
}

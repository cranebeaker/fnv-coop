//! The C runtime family (F6 of the migration plan): the maths, conversion, copy and string routines
//! of the statically linked MSVC 2008 runtime (LIBCMT) that every thread calls. The hottest of them
//! is the square root with the x87 error machinery it ends in: on the main thread, whose control word
//! has 24-bit precision and the precision exception masked, every `sqrt` reports an inexact result
//! through `__math_exit`, `__startOneArgErrorHandling`, `__87except` and `__handle_exc`, so a call is
//! about ten calls deep. Typed, the chain runs as Rust calls.
//!
//! Stage 1: nothing here owns an object; the routines read and write guest memory where their
//! callers point. Each function stays registered at its exe address; a call by address inside the
//! family is a `typed_call!`, and every call the exe makes is still made, so the call tree is the
//! exe's.
//!
//! **Floats.** Every thread runs these, so the family's row is [`Threads::Both`]: the x87 code uses
//! the exact helpers ([`engine_core::x87`]), which round as the running thread's control word says.
//!
//! **Memory.** The copy, fill and scan routines move whole runs of bytes at once ([`move_up`],
//! [`move_down`], [`chunk`]) where the exe moves bytes, dwords or 16-byte blocks: the memory they
//! leave is the same, because a run moved from the bottom up (or from the top down) gives what any
//! width of moves in that direction gives as long as the destination does not overlap the source
//! from above (from below). Where it does, the exe's own steps are kept. Scans read at most to the
//! end of the page of a byte the exe reads, so they never touch a page the exe doesn't.

use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn, Engine};

pub mod math;
pub mod mem;
pub mod string;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(8);

/// `NV_TYPED=crt`. Not past its gate.
pub static CRT: Family = Family {
    name: "crt",
    id: FAMILY,
    past_gate: false,
    threads: Threads::Both,
    funcs,
};

/// `__sse2_available` (`01270a6c`): nonzero when the runtime may use SSE2 (set at start-up).
pub const SSE2_AVAILABLE: u32 = 0x0127_0a6c;

/// The guest page: no scan reads past the end of the page of a byte the exe reads.
const PAGE: u32 = 0x1000;
/// The most bytes a scan reads at once (strings are short; a longer run takes more chunks).
pub(crate) const SCAN: usize = 64;
/// The most bytes a copy moves at once.
const COPY: usize = 0x1000;
/// The buffers short and middling copies and fills use, so a few bytes don't clear or build a
/// page-sized one.
const SHORT: usize = 64;
const MIDDLE: usize = 0x200;
const SHORT_WORDS: u32 = 64;
const MIDDLE_WORDS: u32 = 0x200;
/// [`SCAN`] as a word.
const SCAN_WORDS: u32 = 64;

/// A length below 2^32 (a chunk, a position in one) as a word.
#[allow(clippy::cast_possible_truncation)]
#[inline(always)]
pub(crate) fn small(n: usize) -> u32 {
    n as u32
}

/// The low word of a 64-bit value.
#[allow(clippy::cast_possible_truncation)]
#[inline(always)]
pub(crate) fn low_word(value: u64) -> u32 {
    value as u32
}

/// The low and high words of a 64-bit value (a double's bits as the stack holds them).
#[inline(always)]
pub(crate) fn words(value: u64) -> (u32, u32) {
    (low_word(value), low_word(value >> 32))
}

/// The low 16 bits of a word (`MOV AX, ..`, a control word).
#[allow(clippy::cast_possible_truncation)]
#[inline(always)]
pub(crate) fn low_half(word: u32) -> u16 {
    word as u16
}

/// The low byte of a word (`MOV AL, ..`).
#[allow(clippy::cast_possible_truncation)]
#[inline(always)]
pub(crate) fn low_byte(word: u32) -> u8 {
    word as u8
}

/// A word's bits as a signed word.
#[inline(always)]
pub(crate) fn signed(word: u32) -> i32 {
    i32::from_ne_bytes(word.to_ne_bytes())
}

/// A signed word's bits as a word.
#[inline(always)]
pub(crate) fn unsigned(word: i32) -> u32 {
    u32::from_ne_bytes(word.to_ne_bytes())
}

/// The bytes left on `at`'s page.
#[inline(always)]
fn page_room(at: u32) -> u32 {
    PAGE - (at & (PAGE - 1))
}

/// The bytes from `at`, at most `max` and [`SCAN`], none past the end of `at`'s page.
#[inline(always)]
pub(crate) fn chunk<'b>(e: &Engine, at: u32, max: u32, buf: &'b mut [u8; SCAN]) -> &'b [u8] {
    let n = max.min(SCAN_WORDS).min(page_room(at));
    let bytes = &mut buf[..n as usize];
    e.mem.read(at, bytes);
    bytes
}

/// How many bytes two scans at `a` and `b` may read at once: at most `max`, [`SCAN`], and neither
/// past the end of its page.
#[inline(always)]
pub(crate) fn pair_room(a: u32, b: u32, max: u32) -> usize {
    max.min(SCAN_WORDS).min(page_room(a)).min(page_room(b)) as usize
}

/// `count` bytes moved from `src` to `dst` from the bottom up, a run at a time: what moves of any
/// width from the bottom up leave when the destination is not above the source inside its range.
pub(crate) fn move_up(e: &Engine, dst: u32, src: u32, count: u32) {
    if count <= SHORT_WORDS {
        move_up_in::<SHORT>(e, dst, src, count);
    } else if count <= MIDDLE_WORDS {
        move_up_in::<MIDDLE>(e, dst, src, count);
    } else {
        move_up_in::<COPY>(e, dst, src, count);
    }
}

/// [`move_up`] through an `N`-byte buffer.
fn move_up_in<const N: usize>(e: &Engine, dst: u32, src: u32, count: u32) {
    let mut buf = [0u8; N];
    let mut done = 0u32;
    while done < count {
        let n = (count - done).min(small(N));
        let run = &mut buf[..n as usize];
        e.mem.read(src.wrapping_add(done), run);
        e.mem.write(dst.wrapping_add(done), run);
        done += n;
    }
}

/// `count` bytes moved from `src` to `dst` from the top down, a run at a time: what moves of any
/// width from the top down leave when the destination is not below the source inside its range.
pub(crate) fn move_down(e: &Engine, dst: u32, src: u32, count: u32) {
    if count <= SHORT_WORDS {
        move_down_in::<SHORT>(e, dst, src, count);
    } else if count <= MIDDLE_WORDS {
        move_down_in::<MIDDLE>(e, dst, src, count);
    } else {
        move_down_in::<COPY>(e, dst, src, count);
    }
}

/// [`move_down`] through an `N`-byte buffer.
fn move_down_in<const N: usize>(e: &Engine, dst: u32, src: u32, count: u32) {
    let mut buf = [0u8; N];
    let mut left = count;
    while left > 0 {
        let n = left.min(small(N));
        left -= n;
        let run = &mut buf[..n as usize];
        e.mem.read(src.wrapping_add(left), run);
        e.mem.write(dst.wrapping_add(left), run);
    }
}

/// Whether a copy of `count` bytes from `src` to `dst` has the destination above the source inside
/// its range (`dst > src && dst < src + count`, the exe's test, with its wrap).
#[inline(always)]
pub(crate) fn overlaps_above(dst: u32, src: u32, count: u32) -> bool {
    dst > src && dst < count.wrapping_add(src)
}

/// `count` bytes of `byte` from `at`.
pub(crate) fn fill(e: &Engine, at: u32, byte: u8, count: u32) {
    if byte == 0 {
        e.mem.zero(at, count);
        return;
    }
    if count <= SHORT_WORDS {
        fill_in::<SHORT>(e, at, byte, count);
    } else if count <= MIDDLE_WORDS {
        fill_in::<MIDDLE>(e, at, byte, count);
    } else {
        fill_in::<COPY>(e, at, byte, count);
    }
}

/// [`fill`] from an `N`-byte run of the byte.
fn fill_in<const N: usize>(e: &Engine, at: u32, byte: u8, count: u32) {
    let run = [byte; N];
    let mut done = 0u32;
    while done < count {
        let n = (count - done).min(small(N));
        e.mem.write(at.wrapping_add(done), &run[..n as usize]);
        done += n;
    }
}

/// The family's functions, by exe address.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        // The square roots and the x87 error machinery.
        entry!(0x00ec_6040, math::ci_sqrt(f64) -> f64),
        entry!(0x00ec_6054, math::sqrt(f64) -> f64),
        entry!(
            0x00ec_605d,
            math::sqrt_body(bool, f64, u32, u32) -> f64
        ),
        entry!(
            0x00ed_27ac,
            math::nan_error_type(u32, f64) -> math::EaxSt0
        ),
        entry!(
            0x00ed_27c5,
            math::fload_with_fb(u32, engine_core::Ptr) -> math::EaxSt0
        ),
        entry!(
            0x00ed_282b,
            math::math_exit(u32, u32, f64, u32, u32, u32, u32) -> f64
        ),
        entry!(
            0x00ed_2737,
            math::start_one_arg_error_handling(u32, u32, u32, f64, u32, u32, u32, u32) -> f64
        ),
        entry!(
            0x00ee_1b04,
            math::except_87(u32, engine_core::Ptr, engine_core::Ptr)
        ),
        entry!(
            0x00ed_5554,
            math::handle_exc(u32, engine_core::Ptr, u32) -> i32
        ),
        entry!(
            0x00ed_5531,
            math::raise_exc(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ed_5763, math::errcode(u32) -> i32),
        entry!(0x00ed_5736, math::set_errno_from_matherr(i32)),
        entry!(
            0x00ee_0a3a,
            math::default_matherr(engine_core::Ptr) -> u32
        ),
        entry!(
            0x00ed_59e1,
            math::decomp(f64, engine_core::Ptr) -> f64
        ),
        entry!(0x00ed_5955, math::set_exp(f64, i32) -> f64),
        entry!(0x00ec_85e3, math::errno() -> engine_core::Ptr),
        // Conversions and integer arithmetic.
        entry!(0x00ec_62c0, math::ftol2_sse(f64) -> u64),
        entry!(0x00ec_62f6, math::ftol2_x87(f64) -> u64),
        entry!(0x00ec_6940, math::floor(f64) -> f64),
        entry!(0x00ec_75b1, math::isnan(u32, u32) -> u32),
        entry!(0x00ec_7595, math::finite(u32, u32) -> u32),
        entry!(0x00ec_913a, math::ci_fmod(f64, f64) -> f64),
        entry!(0x00ec_c300, math::allmul(u64, u64) -> u64),
        entry!(0x00ec_c250, math::alldiv(u64, u64) -> u64),
        entry!(
            0x00ec_c590,
            math::aulldiv(u32, u32, u32, u32) -> u64
        ),
        entry!(0x00ec_adb8, math::rand() -> i32),
        entry!(0x00ec_ada6, math::srand(u32)),
        // Memory.
        entry!(0x00ec_44d0, mem::memcpy(u32, u32, u32) -> u32),
        entry!(0x00ec_7230, mem::memmove(u32, u32, u32) -> u32),
        entry!(0x00ec_61c0, mem::memset(u32, u32, u32) -> u32),
        entry!(0x00ed_17f4, mem::vec_memcpy(u32, u32, u32) -> u32),
        entry!(0x00ed_176d, mem::vec_memcpy_blocks(u32, u32, u32)),
        entry!(0x00ed_2c7e, mem::fastzero_i(u32, u32)),
        entry!(0x00ed_2cd5, mem::vec_memzero(u32, u32, u32) -> u32),
        entry!(0x00ec_4835, mem::memcmp(u32, u32, u32) -> i32),
        entry!(0x00ec_a2e0, mem::memchr(u32, u32, u32) -> u32),
        entry!(
            0x0040_1460,
            mem::memcpy_wrapper(engine_core::Ptr, engine_core::Ptr, u32) -> engine_core::Ptr
        ),
        entry!(
            0x0040_3d30,
            mem::memset_wrapper(engine_core::Ptr, i32, u32) -> engine_core::Ptr
        ),
        entry!(
            0x00ec_7c66,
            mem::memcpy_s(engine_core::Ptr, u32, engine_core::Ptr, u32) -> u32
        ),
        // Strings.
        entry!(0x00ec_6130, string::strlen(u32) -> u32),
        entry!(0x00ec_6da0, string::strcmp(u32, u32) -> i32),
        entry!(0x00ec_6370, string::strcpy(u32, u32) -> u32),
        entry!(0x00ec_7690, string::strchr(u32, u32) -> u32),
        entry!(0x00ec_8a19, string::strncmp(u32, u32, u32) -> i32),
        entry!(0x00ec_67aa, string::tolower(u32) -> u32),
        entry!(0x00ec_a7f4, string::toupper(u32) -> u32),
        entry!(0x00ec_68e4, string::stricmp(u32, u32) -> i32),
        entry!(
            0x00ec_67d6,
            string::ascii_stricmp(engine_core::Ptr, engine_core::Ptr) -> i32
        ),
        entry!(0x00ec_7ec0, string::strnicmp(u32, u32, u32) -> i32),
        entry!(
            0x00ed_5bc0,
            string::ascii_strnicmp(u32, u32, u32) -> i32
        ),
        entry!(0x00ec_6bd2, string::strcat_s(u32, u32, u32) -> u32),
        entry!(
            0x00ec_8d5f,
            string::strncpy_s(u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ec_dbb0, string::strcspn(u32, u32) -> u32),
        entry!(0x00ec_3bad, string::byteswap_ulong(u32) -> u32),
        entry!(0x00ec_40cc, string::type_info_equal(u32, u32) -> u32),
        entry!(0x00ec_40ec, string::find_complete_object(u32) -> u32),
        entry!(0x00ec_4179, string::pmd_to_offset(u32, u32) -> u32),
    ]
}

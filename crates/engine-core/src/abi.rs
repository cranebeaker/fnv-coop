//! The uniform form every translated function also has, so that any
//! translation can call any other by its exe address (directly, or through
//! a vtable) without knowing whether it has been translated yet.
//!
//! Arguments are 32-bit words in declaration order, `this` first for
//! `__thiscall` methods (the register conventions `__thiscall` and
//! `__fastcall` are an x86 detail and do not matter here). An 8-byte value
//! (`f64`, `u64`) takes two words, low word first. A struct returned by
//! value is a hidden pointer argument, written out explicitly. A float
//! result (x87 `ST0`) is in [`Ret::st0`]; an integer or pointer result in
//! `eax` (and `edx` for 64-bit results).

use crate::ptr::Ptr;
use crate::Engine;
use std::mem::MaybeUninit;

/// Result of a call in the uniform form.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ret {
    pub eax: u32,
    pub edx: u32,
    pub st0: f64,
}

impl Ret {
    #[inline]
    pub fn u32(self) -> u32 {
        self.eax
    }
    #[inline]
    pub fn i32(self) -> i32 {
        self.eax as i32
    }
    /// The low byte, as MSVC returns `bool` (`AL`).
    #[inline]
    pub fn bool(self) -> bool {
        self.eax as u8 != 0
    }
    #[inline]
    pub fn u8(self) -> u8 {
        self.eax as u8
    }
    #[inline]
    pub fn u16(self) -> u16 {
        self.eax as u16
    }
    #[inline]
    pub fn u64(self) -> u64 {
        self.eax as u64 | (self.edx as u64) << 32
    }
    #[inline]
    pub fn f32(self) -> f32 {
        self.st0 as f32
    }
    #[inline]
    pub fn f64(self) -> f64 {
        self.st0
    }
    #[inline]
    pub fn ptr<T>(self) -> Ptr<T> {
        Ptr::new(self.eax)
    }
}

/// A function in the uniform form.
pub type AbiFn = fn(&mut Engine, &[u32]) -> Ret;

/// A parameter type: how it is read from the argument words.
pub trait Arg: Sized {
    const WORDS: usize;
    fn take(words: &[u32], i: &mut usize) -> Self;
    fn put(self, out: &mut Vec<u32>);
    /// Appends the words to a [`Words`] (what [`args!`] builds). The default goes through
    /// [`Arg::put`] into a buffer the thread keeps (no allocation per call once it has grown);
    /// the word types write in place.
    fn put_words(self, out: &mut Words) {
        thread_local! {
            static PUT: std::cell::Cell<Vec<u32>> = const { std::cell::Cell::new(Vec::new()) };
        }
        // Taken out while in use: a `put` that built arguments itself would get a buffer of its
        // own.
        let mut v = PUT.with(|b| b.take());
        v.clear();
        self.put(&mut v);
        out.extend_from_slice(&v);
        PUT.with(|b| b.set(v));
    }
}

/// Inlined everywhere (also into the opt-level 1 engine crates): every argument of every call
/// through a function table entry is read through it (a29).
#[inline(always)]
fn word(words: &[u32], i: &mut usize) -> u32 {
    let w = *words
        .get(*i)
        .unwrap_or_else(|| panic!("call has {} argument words, needs more", words.len()));
    *i += 1;
    w
}

/// The one argument word of a one-word type ([`Arg::put`]'s word without a buffer): what
/// [`RetVal::into_ret`] puts in `EAX`.
trait OneWord {
    fn one_word(self) -> u32;
}

macro_rules! word_arg {
    ($($t:ty => |$w:ident| $from:expr, |$v:ident| $to:expr;)*) => {$(
        impl OneWord for $t {
            #[inline(always)]
            fn one_word(self) -> u32 {
                let $v = self;
                $to
            }
        }
        impl Arg for $t {
            const WORDS: usize = 1;
            #[inline(always)]
            fn take(words: &[u32], i: &mut usize) -> Self {
                let $w = word(words, i);
                $from
            }
            fn put(self, out: &mut Vec<u32>) {
                let $v = self;
                out.push($to);
            }
            #[inline(always)]
            fn put_words(self, out: &mut Words) {
                out.push(self.one_word());
            }
        }
    )*};
}
word_arg! {
    u32 => |w| w, |v| v;
    i32 => |w| w as i32, |v| v as u32;
    u16 => |w| w as u16, |v| v as u32;
    i16 => |w| w as i16, |v| v as i16 as i32 as u32;
    u8 => |w| w as u8, |v| v as u32;
    i8 => |w| w as i8, |v| v as i8 as i32 as u32;
    bool => |w| w as u8 != 0, |v| v as u32;
    f32 => |w| f32::from_bits(w), |v| v.to_bits();
}

impl<T> Arg for Ptr<T> {
    const WORDS: usize = 1;
    #[inline(always)]
    fn take(words: &[u32], i: &mut usize) -> Self {
        Ptr::new(word(words, i))
    }
    fn put(self, out: &mut Vec<u32>) {
        out.push(self.0)
    }
    #[inline(always)]
    fn put_words(self, out: &mut Words) {
        out.push(self.0)
    }
}

impl Arg for u64 {
    const WORDS: usize = 2;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let lo = word(words, i) as u64;
        let hi = word(words, i) as u64;
        lo | hi << 32
    }
    fn put(self, out: &mut Vec<u32>) {
        out.push(self as u32);
        out.push((self >> 32) as u32);
    }
    #[inline]
    fn put_words(self, out: &mut Words) {
        out.push(self as u32);
        out.push((self >> 32) as u32);
    }
}

impl Arg for f64 {
    const WORDS: usize = 2;
    fn take(words: &[u32], i: &mut usize) -> Self {
        f64::from_bits(u64::take(words, i))
    }
    fn put(self, out: &mut Vec<u32>) {
        self.to_bits().put(out)
    }
    #[inline]
    fn put_words(self, out: &mut Words) {
        self.to_bits().put_words(out)
    }
}

/// A result type: how it is put into a [`Ret`] and read back.
pub trait RetVal: Sized {
    fn into_ret(self) -> Ret;
    fn from_ret(r: Ret) -> Self;
}

macro_rules! int_ret {
    ($($t:ty),*) => {$(
        impl RetVal for $t {
            #[inline]
            fn into_ret(self) -> Ret {
                Ret { eax: self.one_word(), ..Ret::default() }
            }
            #[inline]
            fn from_ret(r: Ret) -> Self {
                <$t as Arg>::take(&[r.eax], &mut 0)
            }
        }
    )*};
}
int_ret!(u32, i32, u16, i16, u8, i8, bool);

impl<T> RetVal for Ptr<T> {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret {
            eax: self.0,
            ..Ret::default()
        }
    }
    #[inline]
    fn from_ret(r: Ret) -> Self {
        Ptr::new(r.eax)
    }
}
impl RetVal for () {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret::default()
    }
    #[inline]
    fn from_ret(_: Ret) -> Self {}
}
impl RetVal for u64 {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret {
            eax: self as u32,
            edx: (self >> 32) as u32,
            st0: 0.0,
        }
    }
    #[inline]
    fn from_ret(r: Ret) -> Self {
        r.u64()
    }
}
impl RetVal for f32 {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret {
            st0: self as f64,
            ..Ret::default()
        }
    }
    #[inline]
    fn from_ret(r: Ret) -> Self {
        r.st0 as f32
    }
}
impl RetVal for f64 {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret {
            st0: self,
            ..Ret::default()
        }
    }
    #[inline]
    fn from_ret(r: Ret) -> Self {
        r.st0
    }
}

/// How many argument words [`Words`] keeps on the stack before it moves them to the heap.
pub const WORDS_INLINE: usize = 12;

/// Argument words as [`args!`] builds them: on the stack up to [`WORDS_INLINE`] words, so a
/// call through `e.call(addr, &args![..])` allocates nothing (it runs millions of times a
/// second); longer lists move to the heap. Reads as a `[u32]` slice and compares equal to a
/// `Vec<u32>`, an array or a slice with the same words.
///
/// Neither the inline words past the list nor the heap list are written until used, so building
/// a short list at a call site stores its length and its words and nothing else.
pub struct Words {
    len: usize,
    /// The words while there are at most [`WORDS_INLINE`]: the first `len` are written.
    inline: MaybeUninit<[u32; WORDS_INLINE]>,
    /// All the words once there are more than [`WORDS_INLINE`]; written (and owned) only then.
    spill: MaybeUninit<Vec<u32>>,
}

impl Words {
    /// No words.
    #[inline(always)]
    pub fn new() -> Self {
        // Only `len` is written. (A struct expression with `MaybeUninit::uninit()` fields is
        // folded into a constant, and the uninitialised words were then copied out of it at
        // every call site: a dozen stores of zeros. Not a `const fn`: these writes are allowed
        // there only from Rust 1.83, past the workspace's 1.75.)
        let mut w = MaybeUninit::<Words>::uninit();
        // SAFETY: `len` is written; the other fields are `MaybeUninit`, valid uninitialised.
        unsafe {
            std::ptr::addr_of_mut!((*w.as_mut_ptr()).len).write(0);
            w.assume_init()
        }
    }

    /// Appends one word.
    #[inline(always)]
    pub fn push(&mut self, w: u32) {
        if self.len < WORDS_INLINE {
            let words = self.inline.as_mut_ptr().cast::<u32>();
            // SAFETY: `len` is below the inline array's length.
            unsafe { words.add(self.len).write(w) };
        } else {
            self.push_spilled(w);
        }
        self.len += 1;
    }

    /// [`Words::push`] past the inline words (`len` not yet counting `w`): the first moves them
    /// to the heap list.
    #[cold]
    #[inline(never)]
    fn push_spilled(&mut self, w: u32) {
        if self.len == WORDS_INLINE {
            // SAFETY: all the inline words are written (`len` is their count).
            let inline = unsafe { self.inline.assume_init_ref() };
            // Room for twice the inline words at once: one allocation for lists up to that.
            let mut spill = Vec::with_capacity(2 * WORDS_INLINE);
            spill.extend_from_slice(inline);
            self.spill.write(spill);
        }
        // SAFETY: `len >= WORDS_INLINE`: the heap list was written when `len` reached it.
        unsafe { self.spill.assume_init_mut() }.push(w);
    }

    /// Appends the words of `words`.
    pub fn extend_from_slice(&mut self, words: &[u32]) {
        for &w in words {
            self.push(w);
        }
    }

    /// The words.
    #[inline(always)]
    pub fn as_slice(&self) -> &[u32] {
        if self.len <= WORDS_INLINE {
            // SAFETY: the first `len` inline words are written.
            unsafe { std::slice::from_raw_parts(self.inline.as_ptr().cast::<u32>(), self.len) }
        } else {
            // SAFETY: `len > WORDS_INLINE`: the heap list holds the words.
            unsafe { self.spill.assume_init_ref() }
        }
    }

    /// The words, writable.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u32] {
        if self.len <= WORDS_INLINE {
            // SAFETY: the first `len` inline words are written.
            unsafe {
                std::slice::from_raw_parts_mut(self.inline.as_mut_ptr().cast::<u32>(), self.len)
            }
        } else {
            // SAFETY: `len > WORDS_INLINE`: the heap list holds the words.
            unsafe { self.spill.assume_init_mut() }
        }
    }
}

impl Drop for Words {
    #[inline(always)]
    fn drop(&mut self) {
        if self.len > WORDS_INLINE {
            // SAFETY: the heap list is written once there are more than WORDS_INLINE words.
            unsafe { self.spill.assume_init_drop() }
        }
    }
}

impl Clone for Words {
    fn clone(&self) -> Words {
        Words::from(self.as_slice())
    }
}

impl Default for Words {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for Words {
    type Target = [u32];
    #[inline(always)]
    fn deref(&self) -> &[u32] {
        self.as_slice()
    }
}

impl std::ops::DerefMut for Words {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u32] {
        self.as_mut_slice()
    }
}

impl AsRef<[u32]> for Words {
    fn as_ref(&self) -> &[u32] {
        self.as_slice()
    }
}

impl std::borrow::Borrow<[u32]> for Words {
    fn borrow(&self) -> &[u32] {
        self.as_slice()
    }
}

impl std::fmt::Debug for Words {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl PartialEq for Words {
    fn eq(&self, other: &Words) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Words {}

impl std::hash::Hash for Words {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl PartialEq<Vec<u32>> for Words {
    fn eq(&self, other: &Vec<u32>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialEq<Words> for Vec<u32> {
    fn eq(&self, other: &Words) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialEq<[u32]> for Words {
    fn eq(&self, other: &[u32]) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<&[u32]> for Words {
    fn eq(&self, other: &&[u32]) -> bool {
        self.as_slice() == *other
    }
}

impl PartialEq<Words> for [u32] {
    fn eq(&self, other: &Words) -> bool {
        self == other.as_slice()
    }
}

impl PartialEq<Words> for &[u32] {
    fn eq(&self, other: &Words) -> bool {
        *self == other.as_slice()
    }
}

impl<const N: usize> PartialEq<[u32; N]> for Words {
    fn eq(&self, other: &[u32; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const N: usize> PartialEq<Words> for [u32; N] {
    fn eq(&self, other: &Words) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl From<Words> for Vec<u32> {
    fn from(w: Words) -> Vec<u32> {
        if w.len <= WORDS_INLINE {
            w.as_slice().to_vec()
        } else {
            let w = std::mem::ManuallyDrop::new(w);
            // SAFETY: `len > WORDS_INLINE`: the heap list is written; it is moved out once, and
            // `w` is not dropped.
            unsafe { w.spill.assume_init_read() }
        }
    }
}

impl From<&[u32]> for Words {
    fn from(words: &[u32]) -> Words {
        let mut w = Words::new();
        w.extend_from_slice(words);
        w
    }
}

impl Extend<u32> for Words {
    fn extend<I: IntoIterator<Item = u32>>(&mut self, iter: I) {
        for w in iter {
            self.push(w);
        }
    }
}

impl<'a> Extend<&'a u32> for Words {
    fn extend<I: IntoIterator<Item = &'a u32>>(&mut self, iter: I) {
        for &w in iter {
            self.push(w);
        }
    }
}

impl FromIterator<u32> for Words {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Words {
        let mut w = Words::new();
        w.extend(iter);
        w
    }
}

impl IntoIterator for Words {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<u32>;
    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

impl<'a> IntoIterator for &'a Words {
    type Item = &'a u32;
    type IntoIter = std::slice::Iter<'a, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

/// `NV_WORD_CHECK=FILE` (a diagnostic, not exe behaviour): a call that hands a translation more
/// words than it takes is appended to FILE once per (caller, callee, given, taken), tab-separated:
/// the sign of words given to the wrong call (the exe's caller pushed a word for a later call),
/// unless the callee's own `RET n` pops the extra words (a trailing word it never reads).
#[cold]
#[inline(never)]
pub fn extra_words(e: &crate::Engine, callee: u32, given: usize, taken: usize) {
    use std::io::Write;
    type Seen = std::collections::HashSet<(u32, u32, usize, usize)>;
    type Sink = Option<std::sync::Mutex<(std::fs::File, Seen)>>;
    static SINK: std::sync::OnceLock<Sink> = std::sync::OnceLock::new();
    let sink = SINK.get_or_init(|| {
        let path = std::env::var_os("NV_WORD_CHECK")?;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()?;
        Some(std::sync::Mutex::new((file, Seen::new())))
    });
    let Some(sink) = sink else { return };
    let stack = &e.call_stack;
    let caller = stack.len().checked_sub(2).map_or(0, |k| stack[k]);
    if let Ok(mut s) = sink.lock() {
        if s.1.insert((caller, callee, given, taken)) {
            let _ = writeln!(s.0, "{caller:08x}\t{callee:08x}\t{given}\t{taken}");
        }
    }
}

/// Builds argument words: `args![this, 1.5f32, true]`.
#[macro_export]
macro_rules! args {
    ($($a:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut v = $crate::abi::Words::new();
        $( $crate::abi::Arg::put_words($a, &mut v); )*
        v
    }};
}

/// One entry of a unit's function table: the exe address and the uniform
/// form of a typed translation.
///
/// ```ignore
/// pub fn funcs() -> Vec<(u32, AbiFn)> {
///     vec![entry!(0x00d87410, get_iterator(Ptr<HkCachedHashMap>) -> i32)]
/// }
/// ```
#[macro_export]
macro_rules! entry {
    ($addr:literal, $($f:ident)::+ ( $($t:ty),* $(,)? ) $(-> $r:ty)?) => {
        ($addr, (|e: &mut $crate::Engine, a: &[u32]| {
            let _ = a;
            #[allow(unused_mut, unused_variables)]
            let mut i = 0usize;
            let r = $($f)::+(e, $( <$t as $crate::abi::Arg>::take(a, &mut i) ),*);
            if a.len() > i {
                $crate::abi::extra_words(e, $addr, a.len(), i);
            }
            $crate::abi::RetVal::into_ret(r)
        }) as $crate::abi::AbiFn)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_round_trip() {
        let w = args![7u32, -2i32, 1.5f32, 2.25f64, true, Ptr::<()>::new(0x1234)];
        assert_eq!(w.len(), 7);
        let mut i = 0;
        assert_eq!(u32::take(&w, &mut i), 7);
        assert_eq!(i32::take(&w, &mut i), -2);
        assert_eq!(f32::take(&w, &mut i), 1.5);
        assert_eq!(f64::take(&w, &mut i), 2.25);
        assert!(bool::take(&w, &mut i));
        assert_eq!(Ptr::<()>::take(&w, &mut i).0, 0x1234);
    }

    /// Past the words kept on the stack the list moves to the heap and reads the same; it
    /// compares equal to a `Vec`, an array and a slice with the same words.
    #[test]
    fn long_word_lists_move_to_the_heap() {
        let w = args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12.5f64];
        assert_eq!(w.len(), WORDS_INLINE + 1);
        let mut v: Vec<u32> = (1..=11).collect();
        v.extend_from_slice(&[0, 0x4029_0000]);
        assert_eq!(w, v);
        assert_eq!(v, w);
        assert_eq!(Vec::from(w.clone()), v);
        assert_eq!(args![7u32, 8u32], [7, 8]);
        assert_eq!(args![7u32, 8u32], &[7u32, 8][..]);
        assert_eq!(format!("{:?}", args![7u32]), "[7]");
    }

    /// Every way to read, change, copy and take apart a list gives its words, on either side of
    /// the move to the heap (the inline words past the list and the heap list are only written
    /// once used).
    #[test]
    fn word_lists_read_the_same_inline_and_on_the_heap() {
        for n in [0u32, 1, 5, 11, 12, 13, 14, 24, 25, 40] {
            let want: Vec<u32> = (0..n).map(|i| i * 3 + 1).collect();
            let mut w = Words::new();
            w.extend_from_slice(&want);
            assert_eq!(w.len(), n as usize);
            assert_eq!(w, want);
            let copy = w.clone();
            w.as_mut_slice().iter_mut().for_each(|x| *x += 1);
            assert_eq!(copy, want, "a copy keeps its own words");
            let bumped: Vec<u32> = want.iter().map(|x| x + 1).collect();
            assert_eq!(w, bumped);
            assert_eq!((&w).into_iter().copied().collect::<Vec<_>>(), bumped);
            assert_eq!(w.clone().into_iter().collect::<Vec<_>>(), bumped);
            assert_eq!(Vec::from(w), bumped);
            assert_eq!(want.iter().copied().collect::<Words>(), want);
            assert_eq!(Words::from(&want[..]), copy);
        }
    }

    #[test]
    fn bool_is_the_low_byte() {
        let r = Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        };
        assert!(!r.bool());
    }
}

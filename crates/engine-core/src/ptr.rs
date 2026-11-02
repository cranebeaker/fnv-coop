//! Typed 32-bit pointers and fields at game offsets.
//!
//! A game class is a zero-sized marker type declared with [`crate::layout!`];
//! its fields are constants of type [`Field`] that carry their offset and
//! value type. A [`Ptr<T>`] is the game's 32-bit address of a `T`.
//!
//! ```
//! use engine_core::{layout, Engine, Ptr};
//! # // A program linking the core alone assembles no part crates.
//! # engine_core::assemble!();
//! layout! {
//!     /// TESForm (Xbox PDB), 0x18 bytes on PC.
//!     pub struct TESForm: 0x18 {
//!         0x08 iFormFlags: u32,
//!         0x0C iFormID: u32,
//!     }
//! }
//! let mut e = Engine::new();
//! let form: Ptr<TESForm> = e.new_object();
//! e.set(form, TESForm::iFormID, 0x0001_0000);
//! assert_eq!(e.get(form, TESForm::iFormID), 0x0001_0000);
//! ```

use crate::mem::Mem;
use std::fmt;
use std::marker::PhantomData;

/// The game's address of a `T`. `T` is a layout marker, or `()` for an
/// untyped pointer.
pub struct Ptr<T = ()>(pub u32, PhantomData<fn() -> T>);

impl<T> Ptr<T> {
    pub const NULL: Ptr<T> = Ptr(0, PhantomData);

    pub const fn new(addr: u32) -> Self {
        Ptr(addr, PhantomData)
    }
    pub const fn addr(self) -> u32 {
        self.0
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// The same address seen as another type (a base class, a field's
    /// struct, or untyped).
    pub const fn cast<U>(self) -> Ptr<U> {
        Ptr(self.0, PhantomData)
    }
    /// The address `off` bytes further, untyped.
    pub const fn byte_add(self, off: u32) -> Ptr<()> {
        Ptr(self.0.wrapping_add(off), PhantomData)
    }
    /// The address of an embedded struct field.
    pub const fn at<U>(self, f: Field<T, Inline<U>>) -> Ptr<U> {
        Ptr(self.0.wrapping_add(f.off), PhantomData)
    }
    /// Element `i` of an array of `stride`-byte entries starting here.
    pub const fn index(self, i: u32, stride: u32) -> Ptr<T> {
        Ptr(self.0.wrapping_add(i.wrapping_mul(stride)), PhantomData)
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Ptr<T> {}
impl<T> PartialEq for Ptr<T> {
    fn eq(&self, o: &Self) -> bool {
        self.0 == o.0
    }
}
impl<T> Eq for Ptr<T> {}
impl<T> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ptr({:08x})", self.0)
    }
}
impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Self::NULL
    }
}

/// A value that lives in game memory as little-endian bytes.
pub trait Scalar: Copy {
    const SIZE: u32;
    fn load(m: &Mem, a: u32) -> Self;
    fn store(self, m: &mut Mem, a: u32);
}

macro_rules! scalar {
    ($t:ty, $n:expr, $get:ident, $set:ident) => {
        impl Scalar for $t {
            const SIZE: u32 = $n;
            #[inline]
            fn load(m: &Mem, a: u32) -> Self {
                m.$get(a)
            }
            #[inline]
            fn store(self, m: &mut Mem, a: u32) {
                m.$set(a, self)
            }
        }
    };
}
scalar!(u8, 1, u8, set_u8);
scalar!(i8, 1, i8, set_i8);
scalar!(u16, 2, u16, set_u16);
scalar!(i16, 2, i16, set_i16);
scalar!(u32, 4, u32, set_u32);
scalar!(i32, 4, i32, set_i32);
scalar!(u64, 8, u64, set_u64);
scalar!(f32, 4, f32, set_f32);
scalar!(f64, 8, f64, set_f64);

impl Scalar for bool {
    const SIZE: u32 = 1;
    #[inline]
    fn load(m: &Mem, a: u32) -> Self {
        m.u8(a) != 0
    }
    #[inline]
    fn store(self, m: &mut Mem, a: u32) {
        m.set_u8(a, self as u8)
    }
}

impl<T> Scalar for Ptr<T> {
    const SIZE: u32 = 4;
    #[inline]
    fn load(m: &Mem, a: u32) -> Self {
        Ptr::new(m.u32(a))
    }
    #[inline]
    fn store(self, m: &mut Mem, a: u32) {
        m.set_u32(a, self.0)
    }
}

/// Marker for a struct embedded by value; see [`Ptr::at`].
pub struct Inline<U>(PhantomData<fn() -> U>);

/// A field of `S` at byte offset `off` holding a `T`.
pub struct Field<S, T> {
    pub off: u32,
    _p: PhantomData<fn() -> (S, T)>,
}

impl<S, T> Field<S, T> {
    pub const fn new(off: u32) -> Self {
        Field {
            off,
            _p: PhantomData,
        }
    }
}
impl<S, T> Clone for Field<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S, T> Copy for Field<S, T> {}

/// Size of a layout, for allocation.
pub trait Layout {
    const SIZE: u32;
}

/// Declares a game class: a marker type with its PC size and the fields
/// translations have needed so far, at their PC offsets. Field names follow
/// the Xbox PDB where it has the class (mark the doc comment `(Xbox PDB)`);
/// check each offset against the PC code that uses it, since the 2010 Xbox
/// build can differ. Embedded structs use `Inline<T>`.
#[macro_export]
macro_rules! layout {
    ($(
        $(#[$m:meta])*
        $vis:vis struct $name:ident : $size:literal {
            $( $(#[$fm:meta])* $off:literal $field:ident : $ty:ty ),* $(,)?
        }
    )*) => {$(
        $(#[$m])*
        #[derive(Debug, Clone, Copy)]
        $vis struct $name;
        impl $crate::ptr::Layout for $name {
            const SIZE: u32 = $size;
        }
        #[allow(non_upper_case_globals)]
        impl $name {
            $( $(#[$fm])* pub const $field: $crate::ptr::Field<$name, $ty> = $crate::ptr::Field::new($off); )*
        }
    )*};
}

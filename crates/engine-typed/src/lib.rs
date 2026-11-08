//! Typed families of the 64-bit migration that live above the core (ADR-0014, docs/ENGINE_CRATE.md,
//! "Typed code"). A family here is registered over the translations by engine-core's seam: this
//! crate's [`units::funcs`] hands the family to [`engine_core::typed::provide`], and
//! `Engine::new` registers it after every translation when `NV_TYPED` has it on.
//!
//! The first family is [`lockfree`]: the game's lock-free hash maps, their iterators and the
//! per-thread interfaces they keep. Stage 1 of the plan: the objects stay where the game made them
//! in guest memory, and the code that reads them is typed.
//!
//! Each class is declared once, with [`image!`]: the marker type and its fields for
//! [`Engine::get`](engine_core::Engine::get) and [`Engine::set`](engine_core::Engine::set), and a
//! `#[repr(C)]` image of the guest bytes whose field offsets and size are checked at compile time.

// Type-aware where the repository's text rule against `as` from a float can only read words. It
// also stops `u32 as u8` and `f64 as f32`: those go through a named helper with a local `allow`.
#![deny(clippy::cast_possible_truncation)]

pub mod crt;
pub mod culling;
pub mod dispatch;
pub mod lockfree;
pub mod pathing;
pub mod shader;

/// Declares a class of the game once: a marker type with its PC size and fields at their PC offsets
/// (as [`engine_core::layout!`] does), and beside it a `#[repr(C)]` struct of the same bytes, whose
/// field offsets and size are asserted at compile time against the offsets written here. A field
/// whose type isn't its bytes (an embedded struct, `Inline<T>`) names its bytes after `=`.
///
/// The image is the guest form of the class: what the bytes are, field by field, with no gap left
/// unnamed. It isn't read through; typed code reads guest memory through the fields. It is what a
/// later stage moves to host memory, once nothing untyped reads the fields.
#[macro_export]
macro_rules! image {
    (@bytes $ty:ty) => { $ty };
    (@bytes $ty:ty = $bytes:ty) => { $bytes };
    ($(
        $(#[$m:meta])*
        $vis:vis struct $name:ident / $image:ident : $size:literal {
            $( $(#[$fm:meta])* $off:literal $field:ident : $ty:ty $(= $bytes:ty)? ),* $(,)?
        }
    )*) => {$(
        $(#[$m])*
        #[derive(Debug, Clone, Copy)]
        $vis struct $name;
        impl ::engine_core::ptr::Layout for $name {
            const SIZE: u32 = $size;
        }
        #[allow(non_upper_case_globals)]
        impl $name {
            $( $(#[$fm])* pub const $field: ::engine_core::ptr::Field<$name, $ty> =
                ::engine_core::ptr::Field::new($off); )*
        }
        #[doc = concat!("The guest bytes of [`", stringify!($name), "`], field by field.")]
        #[repr(C)]
        #[allow(non_snake_case, dead_code)]
        $vis struct $image {
            $( $field: $crate::image!(@bytes $ty $(= $bytes)?), )*
        }
        const _: () = {
            $( assert!(::core::mem::offset_of!($image, $field) == $off); )*
            assert!(::core::mem::size_of::<$image>() == $size);
        };
    )*};
}

/// This crate's part in `Engine::new`, as the `engine` facade takes each engine crate's: it
/// registers nothing at an address itself, and hands its families to the core's seam, which
/// registers the ones `NV_TYPED` has on after every translation.
pub mod units {
    pub fn funcs(_out: &mut Vec<(u32, engine_core::AbiFn)>) {
        engine_core::typed::provide(&crate::lockfree::LOCKFREE);
        engine_core::typed::provide(&crate::culling::CULLING);
        engine_core::typed::provide(&crate::pathing::PATHING);
        engine_core::typed::provide(&crate::shader::SHADER);
        engine_core::typed::provide(&crate::crt::CRT);
        engine_core::typed::provide(&crate::dispatch::DISPATCH);
    }
}

// This crate's unit tests: `Engine::new()` registers the core's functions and provides the family.
#[cfg(test)]
engine_core::assemble!(units::funcs);

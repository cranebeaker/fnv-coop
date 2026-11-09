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

#[cfg(test)]
mod tests {
    use engine_core::typed::{parse_switch, Family, Mode};

    /// This crate's families, in the order `units::funcs` provides them.
    fn families() -> [&'static Family; 6] {
        [
            &crate::lockfree::LOCKFREE,
            &crate::culling::CULLING,
            &crate::pathing::PATHING,
            &crate::shader::SHADER,
            &crate::crt::CRT,
            &crate::dispatch::DISPATCH,
        ]
    }

    /// Which of this crate's families are past their gate, and so on with no switch: all six since
    /// their speed gate together (2026-11-08); `NV_TYPED=off` or `<family>:off` turns one off.
    #[test]
    fn families_past_their_gate_are_on_by_default() {
        let families = families();
        let past: Vec<&str> = families
            .iter()
            .filter(|f| f.past_gate)
            .map(|f| f.name)
            .collect();
        assert_eq!(
            past,
            ["lockfree", "culling", "pathing", "shader", "crt", "dispatch"]
        );
        let defaults = parse_switch(None, &families).unwrap();
        for (f, m) in families.iter().zip(&defaults) {
            assert_eq!(*m == Mode::On, f.past_gate, "{}", f.name);
        }
        let off = parse_switch(Some("off"), &families).unwrap();
        assert!(off.iter().all(|m| *m == Mode::Off));
        let one_off = parse_switch(Some("shader:off"), &families).unwrap();
        assert_eq!(one_off[3], Mode::Off);
        assert!(one_off.iter().filter(|m| **m == Mode::On).count() == 5);
    }

    /// With no switch, `Engine::new` registers every function of each family, and its calls between
    /// typed functions go direct. Not the default, so not checked: a test process started with
    /// `NV_TYPED` set (the differential runs) or as a run compared with the original's record
    /// (`typed::COMPARED_RUN_VARIABLES`, every family off). A diagnostic the shell has on at start (a
    /// profile, a block check, a census, the touch record) keeps every call by address: the functions
    /// are registered, and only that is checked.
    #[test]
    fn engine_new_has_them_on_with_no_switch() {
        let set = |name: &str| std::env::var_os(name).is_some();
        if set("NV_TYPED")
            || engine_core::typed::COMPARED_RUN_VARIABLES
                .iter()
                .any(|n| set(n))
        {
            return;
        }
        let diagnosing = [
            "NV_PROFILE",
            "NV_BLOCK_CHECK",
            "NV_CENSUS",
            "NV_RACE_CENSUS",
            "NV_TOUCH",
        ]
        .iter()
        .any(|n| set(n));
        let e = engine_core::Engine::new();
        for f in families() {
            for (addr, _) in (f.funcs)() {
                assert!(e.is_registered(addr), "{} at {addr:08x}", f.name);
                if !diagnosing {
                    assert!(e.typed_direct(f.id, addr), "{} at {addr:08x}", f.name);
                }
            }
        }
    }
}

//! `fallout shared/distant terrain system/run_006faa30` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three smart-pointer getters and a setter; the exe map names neither the
//! functions nor their receivers' class. The getters' offsets (`+0x48`,
//! `+0x50`, `+0x54`) are smart pointers that `006fb980` (called by the chunk
//! constructor `006fa210` to build a 0x60-byte object) initializes to null,
//! so the receiver of the getters is probably that object; the setter's
//! fields (`+4` a smart pointer, `+0x2c` another) agree with
//! `BGSTerrainChunk::spMesh` and `spLightingProp` of the Xbox PDB. Neither
//! class is confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer` dereference (`*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<..>::operator=(NiPointer const&)` (Xbox PDB map name
/// `NiPointer<PathingDebugGeometryData>::operator_`; the body is the generic
/// smart-pointer assignment): `this` is the target, the argument the address
/// of the source smart pointer; returns the target.
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Reads the field at `+0x2c` of its `this` through [`POINTER_GET`] (the
/// engine map names it in `tesobjectrefr.cpp`).
const POINTER_FIELD_2C: u32 = 0x0056_c7f0;

// Translated from 006faa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the smart pointer at `this + 0x48` to the smart pointer at `out`
/// and returns `out`.
pub fn fn_006faa30(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(POINTER_ASSIGN, &args![out, this.addr() + 0x48])
        .ptr()
}

// Translated from 006faa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the smart pointer at `this + 0x50` to the smart pointer at `out`
/// and returns `out`.
pub fn fn_006faa50(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(POINTER_ASSIGN, &args![out, this.addr() + 0x50])
        .ptr()
}

// Translated from 006faa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the smart pointer at `this + 0x54` to the smart pointer at `out`
/// and returns `out`.
pub fn fn_006faa70(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(POINTER_ASSIGN, &args![out, this.addr() + 0x54])
        .ptr()
}

/// What `FLD dword` then `FSTP dword` does to a single-precision value: the
/// bits are kept, except that a signalling NaN becomes the quiet NaN with
/// the same payload.
fn x87_copy(value: f32) -> u32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

// Translated from 006faa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the smart pointer at `this + 4` is set and the object found through
/// `this + 0x2c` (`0056c7f0`) is not null, stores `value` at `+0x80` of that
/// object (an x87 `FLD`/`FSTP` copy: a signalling NaN is stored quieted).
pub fn fn_006faa90(e: &mut Engine, this: Ptr, value: f32) {
    if e.call(POINTER_GET, &args![this.addr() + 4]).u32() != 0 {
        let target = e.call(POINTER_FIELD_2C, &args![this]).u32();
        if target != 0 {
            e.mem.set_u32(target + 0x80, x87_copy(value));
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006faa30, fn_006faa30(Ptr, Ptr) -> Ptr),
        entry!(0x006faa50, fn_006faa50(Ptr, Ptr) -> Ptr),
        entry!(0x006faa70, fn_006faa70(Ptr, Ptr) -> Ptr),
        entry!(0x006faa90, fn_006faa90(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn getters_assign_their_field_to_the_out_pointer_and_return_it() {
        let mut e = Engine::new();
        e.register(POINTER_ASSIGN, |e, a| {
            let source = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], source);
            ret(a[0])
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x60));
        e.mem.set_u32(this.addr() + 0x48, 0xa);
        e.mem.set_u32(this.addr() + 0x50, 0xb);
        e.mem.set_u32(this.addr() + 0x54, 0xc);
        let out = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        for (function, expected) in [(0x006f_aa30, 0xa), (0x006f_aa50, 0xb), (0x006f_aa70, 0xc)] {
            let back = e.call(function, &args![this, out]).ptr::<()>();
            assert_eq!(back, out);
            assert_eq!(e.mem.u32(out.addr()), expected);
        }
        let log = e.call_log.take().unwrap();
        let assigns: Vec<_> = log.iter().filter(|(a, _)| *a == POINTER_ASSIGN).collect();
        assert_eq!(assigns.len(), 3);
        assert_eq!(assigns[1].1, vec![out.addr(), this.addr() + 0x50]);
    }

    /// Doubles: the smart-pointer read, and `0056c7f0` (the word at `+0x2c`).
    fn setter_engine() -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_FIELD_2C, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        let this = Ptr::<()>::new(e.mem.alloc(0x30));
        let target = Ptr::<()>::new(e.mem.alloc(0x90));
        (e, this, target)
    }

    #[test]
    fn setter_stores_the_value_when_both_pointers_are_set() {
        let (mut e, this, target) = setter_engine();
        e.mem.set_u32(this.addr() + 4, 0x1234);
        e.mem.set_u32(this.addr() + 0x2c, target.addr());
        e.call(0x006f_aa90, &args![this, 0.75f32]);
        assert_eq!(e.mem.f32(target.addr() + 0x80), 0.75);
    }

    #[test]
    fn setter_does_nothing_without_the_first_or_the_second_pointer() {
        let (mut e, this, target) = setter_engine();
        // First pointer null.
        e.mem.set_u32(this.addr() + 0x2c, target.addr());
        e.call(0x006f_aa90, &args![this, 0.75f32]);
        assert_eq!(e.mem.u32(target.addr() + 0x80), 0);
        // Second pointer null: no store anywhere (a store through null would panic).
        e.mem.set_u32(this.addr() + 4, 0x1234);
        e.mem.set_u32(this.addr() + 0x2c, 0);
        e.call(0x006f_aa90, &args![this, 0.75f32]);
        assert_eq!(e.mem.u32(target.addr() + 0x80), 0);
    }

    #[test]
    fn setter_quiets_a_signalling_nan_and_keeps_other_bit_patterns() {
        let (mut e, this, target) = setter_engine();
        e.mem.set_u32(this.addr() + 4, 0x1234);
        e.mem.set_u32(this.addr() + 0x2c, target.addr());
        let store = |e: &mut Engine, bits: u32| {
            e.call(0x006f_aa90, &args![this, f32::from_bits(bits)]);
            e.mem.u32(target.addr() + 0x80)
        };
        // Signalling NaN, either sign: the quiet bit is set, the payload kept.
        assert_eq!(store(&mut e, 0x7f80_0001), 0x7fc0_0001);
        assert_eq!(store(&mut e, 0xffbf_ffff), 0xffff_ffff);
        // Quiet NaN, infinities, denormals and negative zero are copied as they are.
        assert_eq!(store(&mut e, 0x7fc1_2345), 0x7fc1_2345);
        assert_eq!(store(&mut e, 0xff80_0000), 0xff80_0000);
        assert_eq!(store(&mut e, 0x0000_0001), 0x0000_0001);
        assert_eq!(store(&mut e, 0x8000_0000), 0x8000_0000);
    }
}

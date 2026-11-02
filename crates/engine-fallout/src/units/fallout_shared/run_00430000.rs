//! `fallout shared/run_00430000` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004300a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the float at `this` to 0 (`FLDZ`, `FSTP`) and returns `this` (no
/// name in the engine map).
pub fn fn_004300a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_f32(this.addr(), 0.0);
    this
}

/// `006e6da0` (`cdecl`, two words: size, place): the placement `new`; it
/// answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;

// Translated from 00430000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00430000 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` floats in the array at `array` (`thiscall`, `RET 8`;
/// `this` is not read; the engine map gives it no name): for each element
/// the placement `new` (`006e6da0` with 4 and the element's address) and,
/// when it answers non-null, `004300a0` on it (sets it to 0).
pub fn fn_00430000(e: &mut Engine, _this: Ptr, array: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![4u32, array.wrapping_add(index.wrapping_mul(4))],
            )
            .u32();
        if place != 0 {
            fn_004300a0(e, Ptr::new(place));
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0043_0000, fn_00430000(Ptr, u32, u32)),
        entry!(0x0043_00a0, fn_004300a0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004300a0_stores_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x0043_00a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00430000_zeroes_each_placed_element() {
        let mut e = Engine::new();
        // The placement new answers its place, except for the second element.
        e.register(PLACEMENT_NEW, |e, a| {
            let skip = e.mem.u32(0x0f00_0000);
            Ret {
                eax: if a[1] == skip { 0 } else { a[1] },
                ..Ret::default()
            }
        });
        e.mem.map(0x0f00_0000, 4);
        let array = e.mem.alloc(12);
        e.mem.write(array, &[0xff; 12]);
        e.mem.set_u32(0x0f00_0000, array + 4);
        e.call_log = Some(vec![]);
        e.call(0x0043_0000, &args![0u32, array, 3u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<_> = log.iter().filter(|c| c.0 == PLACEMENT_NEW).collect();
        assert_eq!(placed.len(), 3);
        assert_eq!(placed[2].1, vec![4, array + 8]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(array + 8), 0);
        // A count of 0 places nothing.
        e.call_log = Some(vec![]);
        e.call(0x0043_0000, &args![0u32, array, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

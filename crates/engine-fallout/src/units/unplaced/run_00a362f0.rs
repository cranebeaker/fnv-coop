//! `(unplaced)/run_00a362f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three words at `011a8400` (three `-FLT_MAX` floats in the exe image).
const POINT_SOURCE: u32 = 0x011a_8400;
/// The four words at `011f3704` (zero in the exe image; a static set up at run time).
const QUATERNION_SOURCE: u32 = 0x011f_3704;
/// The floats at `01096b8c` and `01015f5c` (both `-FLT_MAX` in the exe image).
const LOW_EXTENT: u32 = 0x0109_6b8c;
const NO_DISTANCE: u32 = 0x0101_5f5c;
/// The nine words at `011a9448` (an identity 3x3 matrix in the exe image), copied to +0x44.
const MATRIX_SOURCE: u32 = 0x011a_9448;
const MATRIX: u32 = 0x44;
/// The offsets the float at `01015f5c` is stored at last, in the exe's order.
const SENTINEL_FIELDS: [u32; 7] = [0x0, 0x4, 0x14, 0x20, 0x24, 0x34, 0x40];

// Translated from 00a362f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, no stack words; answers `this` in `EAX`): the element constructor of
/// 0x68-byte elements that `00a363b0` (in `NiAnimation/nitransformcontroller.cpp`) hands
/// the array constructor `00401050` (pushed at `00a36452`). Twice, at +0x4 and at +0x24: the
/// three words at `011a8400`, the four at `011f3704` and the float at `01096b8c`; then the
/// float at `01015f5c` at +0x0, +0x4, +0x14, +0x20, +0x24, +0x34 and +0x40 (over some of
/// those copies) and the nine words at `011a9448` at +0x44.
pub fn fn_00a362f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for half in [0x4u32, 0x24] {
        for word in 0..3 {
            let value = e.mem.u32(POINT_SOURCE + word * 4);
            e.mem.set_u32(base + half + word * 4, value);
        }
        for word in 0..4 {
            let value = e.mem.u32(QUATERNION_SOURCE + word * 4);
            e.mem.set_u32(base + half + 0xc + word * 4, value);
        }
        let value = x87_quiet_bits(e.mem.u32(LOW_EXTENT));
        e.mem.set_u32(base + half + 0x1c, value);
    }
    let sentinel = x87_quiet_bits(e.mem.u32(NO_DISTANCE));
    for offset in SENTINEL_FIELDS {
        e.mem.set_u32(base + offset, sentinel);
    }
    for word in 0..9 {
        let value = e.mem.u32(MATRIX_SOURCE + word * 4);
        e.mem.set_u32(base + MATRIX + word * 4, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_62f0, fn_00a362f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_element_constructor_fills_all_0x68_bytes() {
        let mut e = Engine::new();
        let fill = |e: &mut Engine, at: u32, words: u32, first: u32| {
            e.map(at, words * 4);
            for i in 0..words {
                e.mem.set_u32(at + i * 4, first + i);
            }
        };
        fill(&mut e, POINT_SOURCE, 3, 0x1000);
        fill(&mut e, QUATERNION_SOURCE, 4, 0x2000);
        fill(&mut e, MATRIX_SOURCE, 9, 0x3000);
        e.map(LOW_EXTENT, 4);
        e.mem.set_u32(LOW_EXTENT, 0x4000_0000);
        e.map(NO_DISTANCE, 4);
        // A signalling NaN, so the x87 quieting shows.
        e.mem.set_u32(NO_DISTANCE, 0xff80_0001);
        let this = e.mem.alloc(0x68);
        assert_eq!(e.call(0x00a3_62f0, &args![this]).u32(), this);
        let words: Vec<u32> = (0..0x1a).map(|i| e.mem.u32(this + i * 4)).collect();
        let s = 0xffc0_0001;
        let expected = [
            s, s, 0x1001, 0x1002, 0x2000, s, 0x2002, 0x2003, s, s, 0x1001, 0x1002, 0x2000, s,
            0x2002, 0x2003, s, 0x3000, 0x3001, 0x3002, 0x3003, 0x3004, 0x3005, 0x3006, 0x3007,
            0x3008,
        ];
        assert_eq!(words, expected);
    }
}

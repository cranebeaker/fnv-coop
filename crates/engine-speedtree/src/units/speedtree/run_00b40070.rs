//! `SpeedTree/run_00b40070` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00b3f030`, no stack words).
const BASE_CONSTRUCT_00B3F030: u32 = 0x00b3_f030;
/// The vtable of `CBillboardLeaf` (RTTI `.?AVCBillboardLeaf@@`).
const BILLBOARD_LEAF_VTABLE: u32 = 0x010a_58fc;

// Translated from 00b40070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf`'s constructor (by the vtable it stores; thiscall,
/// `RET 0x14`; unnamed in the engine map): the base, the vtable, the byte
/// `second_flag` at +0x10, -1 at +0x14, the byte `first_flag` at +0x18,
/// three zero vectors (+0x1c, +0x28, +0x34, each `FLDZ` stored z, y, x),
/// the byte at +0x40 cleared, `value` (an `FLD`/`FSTP` copy) at +0x44,
/// `word` at +0x48 and the three words at `position` at +4. Returns
/// `this`.
pub fn fn_00b40070(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    first_flag: u8,
    second_flag: u8,
    value: f32,
    word: u32,
) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00B3F030, &args![this]);
    e.mem.set_u32(base, BILLBOARD_LEAF_VTABLE);
    e.mem.set_u8(base + 0x10, second_flag);
    e.mem.set_u32(base + 0x14, u32::MAX);
    e.mem.set_u8(base + 0x18, first_flag);
    for vector in [0x1cu32, 0x28, 0x34] {
        for component in [8u32, 4, 0] {
            e.mem.set_u32(base + vector + component, x87_store32(0.0));
        }
    }
    e.mem.set_u8(base + 0x40, 0);
    e.mem.set_u32(base + 0x44, x87_quiet_bits(value.to_bits()));
    e.mem.set_u32(base + 0x48, word);
    let source = position.addr();
    for i in 0..3 {
        let component = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(base + 4 + 4 * i, component);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b4_0070,
        fn_00b40070(Ptr, Ptr, u8, u8, f32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_leaf_fields() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00B3F030, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x4c);
        for offset in (0..0x4c).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let position = e.mem.alloc(12);
        for (i, word) in [0x10u32, 0x20, 0x7f80_0001].iter().enumerate() {
            e.mem.set_u32(position + 4 * i as u32, *word);
        }
        let result = e
            .call(
                0x00b4_0070,
                &args![
                    this,
                    position,
                    0x0102u32,
                    0x0304u32,
                    0x7f80_0002u32,
                    0x55u32
                ],
            )
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), BILLBOARD_LEAF_VTABLE);
        // The position is moved as words (a NaN keeps its bits).
        assert_eq!(e.mem.u32(this + 4), 0x10);
        assert_eq!(e.mem.u32(this + 0xc), 0x7f80_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0xdead_be04);
        assert_eq!(e.mem.u32(this + 0x14), u32::MAX);
        assert_eq!(e.mem.u32(this + 0x18), 0xdead_be02);
        for offset in (0x1c..0x40).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x40), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x44), 0x7fc0_0002);
        assert_eq!(e.mem.u32(this + 0x48), 0x55);
    }
}

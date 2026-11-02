//! `NiCollision/nibox.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColor::LoadBinary` / `NiColor::SaveBinary` (engine map, `00a59be0` /
/// `00a69780`; `thiscall` on three floats, the stream; `RET 4`): here the
/// centre and the three axes.
const LOAD_THREE_FLOATS: u32 = 0x00a5_9be0;
const SAVE_THREE_FLOATS: u32 = 0x00a6_9780;
/// `NiStream::m_pkIstr` / `m_pkOstr` (Xbox PDB) +0x24C / +0x250, and the
/// binary stream's position (+4), read (+8) and write (+0xc) functions
/// (cdecl: stream, buffer, size, the address of the component size, the
/// component count; they answer the bytes moved).
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;

/// The three extents (`m_afExtent`, +0x30, 12 bytes as one item of three
/// 4-byte components) through the binary stream's function at `slot`; the
/// component size lives in a stack word (the caller's argument slot in the
/// exe). The position advances by the bytes moved.
fn transfer_extents(e: &mut Engine, this: Ptr, binary: u32, slot: u32) {
    e.with_stack(4, |e, component_size| {
        e.mem.set_u32(component_size.addr(), 4);
        let function = e.mem.u32(binary.wrapping_add(slot));
        let moved = e
            .call(
                function,
                &args![
                    binary,
                    this.addr().wrapping_add(0x30),
                    0xcu32,
                    component_size,
                    1u32
                ],
            )
            .u32();
        let position = binary.wrapping_add(BINARY_STREAM_POSITION);
        let advanced = e.mem.u32(position).wrapping_add(moved);
        e.mem.set_u32(position, advanced);
    });
}

// Translated from 00eae520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBox::operator==` (Xbox PDB; `thiscall`; the other box; `RET 4`):
/// false unless the centres are equal component by component (`FUCOMPP`,
/// a NaN is unequal); then the axes and extents ([`fn_00eae556`]).
pub fn ni_box_operator_(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    for offset in [0u32, 4, 8] {
        let mine = x87_load32(e.mem.u32(this.addr() + offset));
        let theirs = x87_load32(e.mem.u32(other.addr() + offset));
        if theirs != mine {
            return false;
        }
    }
    fn_00eae556(e, this, other, other.addr())
}

// Translated from 00eae556 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second half of [`ni_box_operator_`] (Ghidra makes a function of it;
/// it is entered only by that function's jump, with this box in `ECX`, the
/// other in `EDI`, and leaves through its `RET 4`, so it takes the stack
/// word without reading it): for each of the three axes, the axis
/// components and then the extent must be equal (`FUCOMPP`, a NaN is
/// unequal).
pub fn fn_00eae556(e: &mut Engine, this: Ptr, other: Ptr, _unused_0: u32) -> bool {
    let (this, other) = (this.addr(), other.addr());
    for axis in 0..3u32 {
        let mine = this + 0xc + 0xc * axis;
        let theirs = other + 0xc + 0xc * axis;
        for offset in [0u32, 4, 8] {
            if x87_load32(e.mem.u32(theirs + offset)) != x87_load32(e.mem.u32(mine + offset)) {
                return false;
            }
        }
        let mine = x87_load32(e.mem.u32(this + 0x30 + 4 * axis));
        let theirs = x87_load32(e.mem.u32(other + 0x30 + 4 * axis));
        if theirs != mine {
            return false;
        }
    }
    true
}

// Translated from 00eae5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBox::LoadBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`): the
/// centre and the three axes (`NiColor::LoadBinary` each), then the three
/// extents in one read.
pub fn ni_box_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_THREE_FLOATS, &args![this, stream]);
    for axis in 0..3u32 {
        e.call(
            LOAD_THREE_FLOATS,
            &args![this.addr() + 0xc + 0xc * axis, stream],
        );
    }
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    transfer_extents(e, this, input, BINARY_STREAM_READ);
}

// Translated from 00eae630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBox::SaveBinary` (Xbox PDB; `thiscall`; the stream; `RET 4`): the
/// centre and the three axes (`NiColor::SaveBinary` each), then the three
/// extents in one write.
pub fn ni_box_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_THREE_FLOATS, &args![this, stream]);
    for axis in 0..3u32 {
        e.call(
            SAVE_THREE_FLOATS,
            &args![this.addr() + 0xc + 0xc * axis, stream],
        );
    }
    let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
    transfer_extents(e, this, output, BINARY_STREAM_WRITE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ea_e520, ni_box_operator_(Ptr, Ptr) -> bool),
        entry!(0x00ea_e556, fn_00eae556(Ptr, Ptr, u32) -> bool),
        entry!(0x00ea_e5d0, ni_box_load_binary(Ptr, Ptr)),
        entry!(0x00ea_e630, ni_box_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxed(e: &mut Engine, values: &[f32; 15]) -> u32 {
        let b = e.mem.alloc(0x3c);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(b + 4 * i as u32, *v);
        }
        b
    }

    const SAMPLE: [f32; 15] = [
        1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 4.0, 5.0, 6.0,
    ];

    #[test]
    fn operator_compares_centre_axes_and_extents() {
        let mut e = Engine::new();
        let a = boxed(&mut e, &SAMPLE);
        let b = boxed(&mut e, &SAMPLE);
        assert!(e.call(0x00ea_e520, &args![a, b]).bool());
        // Any one component different (each of the 15 words), or a NaN.
        for word in 0..15u32 {
            let saved = e.mem.u32(b + 4 * word);
            e.mem.set_f32(b + 4 * word, 9.5);
            assert!(!e.call(0x00ea_e520, &args![a, b]).bool(), "word {word}");
            e.mem.set_u32(b + 4 * word, 0x7fc0_0000);
            assert!(!e.call(0x00ea_e520, &args![a, b]).bool(), "word {word}");
            e.mem.set_u32(b + 4 * word, saved);
        }
        // -0.0 equals 0.0.
        e.mem.set_u32(b + 0x10, 0x8000_0000);
        assert!(e.call(0x00ea_e520, &args![a, b]).bool());
        // The second half alone, entered with the stack word.
        assert!(e.call(0x00ea_e556, &args![a, b, 0u32]).bool());
        e.mem.set_f32(b + 0x38, 0.5);
        assert!(!e.call(0x00ea_e556, &args![a, b, b]).bool());
    }

    #[test]
    fn load_and_save_binary_move_the_parts() {
        for (addr, three, slot) in [
            (0x00ea_e5d0u32, LOAD_THREE_FLOATS, BINARY_STREAM_READ),
            (0x00ea_e630, SAVE_THREE_FLOATS, BINARY_STREAM_WRITE),
        ] {
            let mut e = Engine::new();
            e.register(three, |_, _| Ret::default());
            e.register(0x0f00_0000, |e, a| {
                assert_eq!(e.mem.u32(a[3]), 4);
                (a[2] * a[4]).into_ret()
            });
            let binary = e.mem.alloc(0x10);
            e.mem.set_u32(binary + 4, 0x20);
            e.mem.set_u32(binary + slot, 0x0f00_0000);
            let stream = e.mem.alloc(0x254);
            e.mem.set_u32(stream + STREAM_INPUT, binary);
            e.mem.set_u32(stream + STREAM_OUTPUT, binary);
            let this = e.mem.alloc(0x3c);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, stream]);
            let log = e.call_log.take().unwrap();
            let parts: Vec<u32> = log
                .iter()
                .filter(|(a, _)| *a == three)
                .map(|(_, w)| {
                    assert_eq!(w[1], stream);
                    w[0] - this
                })
                .collect();
            assert_eq!(parts, vec![0, 0xc, 0x18, 0x24]);
            let extents: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0f00_0000).collect();
            assert_eq!(extents.len(), 1);
            assert_eq!(extents[0].1[0], binary);
            assert_eq!(extents[0].1[1], this + 0x30);
            assert_eq!(extents[0].1[2], 0xc);
            assert_eq!(extents[0].1[4], 1);
            assert_eq!(e.mem.u32(binary + 4), 0x2c);
        }
    }
}

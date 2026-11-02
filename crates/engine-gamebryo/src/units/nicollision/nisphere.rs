//! `NiCollision/nisphere.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three-float `LoadBinary` run on the centre (`00a59be0`, `RET 4`;
/// engine map `NiColor::LoadBinary`, the same code for any three floats).
const LOAD_THREE_FLOATS: u32 = 0x00a5_9be0;
/// The three-float `SaveBinary` (`00a69780`, `RET 4`; engine map
/// `NiColor::SaveBinary`).
const SAVE_THREE_FLOATS: u32 = 0x00a6_9780;
/// `NiSphere::m_fRadius` (Xbox PDB) +0xC (after the centre `m_kCenter`).
const RADIUS: u32 = 0xc;
/// `NiStream::m_pkIstr` / `m_pkOstr` (Xbox PDB) +0x24C / +0x250.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4, `m_pfnRead` +8,
/// `m_pfnWrite` +0xC (`cdecl (stream, buffer, size, &component size,
/// count)`, answer the bytes moved).
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;

/// The radius through the stream's function at `slot` (4 bytes, component
/// size 4 in the stream argument's stack word), the position advanced.
fn transfer_radius(e: &mut Engine, this: Ptr, binary: u32, slot: u32) {
    let function = e.mem.u32(binary.wrapping_add(slot));
    e.with_stack(4, |e, component_size| {
        e.mem.set_u32(component_size.addr(), 4);
        let got = e
            .call(
                function,
                &args![
                    binary,
                    this.addr().wrapping_add(RADIUS),
                    4u32,
                    component_size,
                    1u32
                ],
            )
            .u32();
        let position = binary.wrapping_add(BINARY_STREAM_POSITION);
        let advanced = e.mem.u32(position).wrapping_add(got);
        e.mem.set_u32(position, advanced);
    });
}

// Translated from 00a69ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphere::LoadBinary` (Xbox PDB): the centre (`00a59be0` on `this`),
/// then the 4-byte radius through the input stream's `m_pfnRead`.
pub fn ni_sphere_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_THREE_FLOATS, &args![this, stream]);
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    transfer_radius(e, this, input, BINARY_STREAM_READ);
}

// Translated from 00eb5410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSphere::SaveBinary` (Xbox PDB): the centre (`00a69780` on `this`),
/// then the 4-byte radius through the output stream's `m_pfnWrite`.
pub fn ni_sphere_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_THREE_FLOATS, &args![this, stream]);
    let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
    transfer_radius(e, this, output, BINARY_STREAM_WRITE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a69ac0, ni_sphere_load_binary(Ptr, Ptr)),
        entry!(0x00eb5410, ni_sphere_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE_TRANSFER: u32 = 0x7100_0000;

    fn run(address: u32, parent: u32, stream_slot: u32, function_slot: u32) {
        let mut e = Engine::new();
        e.register(parent, |_, _| Ret::default());
        e.register(FAKE_TRANSFER, |e, a| {
            assert_eq!(&a[2..], [4, a[3], 1]);
            assert_eq!(e.mem.u32(a[3]), 4);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x10);
        let stream = e.mem.alloc(0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 0x20);
        e.mem.set_u32(binary + function_slot, FAKE_TRANSFER);
        e.mem.set_u32(stream + stream_slot, binary);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let first = log.iter().find(|c| c.0 == parent).unwrap();
        assert_eq!(first.1, [this, stream]);
        let moved = log.iter().find(|c| c.0 == FAKE_TRANSFER).unwrap();
        assert_eq!(&moved.1[..2], [binary, this + 0xc]);
        assert_eq!(e.mem.u32(binary + 4), 0x24);
    }

    #[test]
    fn load_binary_reads_the_centre_then_the_radius() {
        run(0x00a69ac0, LOAD_THREE_FLOATS, 0x24c, 8);
    }

    #[test]
    fn save_binary_writes_the_centre_then_the_radius() {
        run(0x00eb5410, SAVE_THREE_FLOATS, 0x250, 0xc);
    }
}

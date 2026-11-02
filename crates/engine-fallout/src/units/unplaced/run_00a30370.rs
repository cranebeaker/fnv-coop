//! `(unplaced)/run_00a30370` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a2f8c0` (`thiscall`): `NiInterpController`'s constructor (the vtable it stores).
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
/// `NiMultiTargetTransformController`'s vtable (by its RTTI type descriptor).
const NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE: u32 = 0x0109_6444;

// Translated from 00a30330 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiMultiTargetTransformController`): allocates 0x40 bytes (`operator new`);
/// when that succeeds, the `NiInterpController` constructor (`00a2f8c0`), the 16-bit word at
/// +0x3c 0, the vtable `01096444` and the words at +0x34 and +0x38 0. Answers the object, or
/// null.
pub fn fn_00a30330(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![block]);
    e.mem.set_u16(block + 0x3c, 0);
    e.mem
        .set_u32(block, NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE);
    e.mem.set_u32(block + 0x34, 0);
    e.mem.set_u32(block + 0x38, 0);
    Ptr::new(block)
}

// Translated from 00a30370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, destination)): reads 2 bytes from the stream's
/// input (`NiStream::m_pkIstr`) into `destination`: the input's read function (+8) is called
/// as `function(input, destination, 2, &count, 1)` with `count` (the stream argument's stack
/// word) set to 2, and the byte count it answers is added to the input's position.
pub fn fn_00a30370(e: &mut Engine, stream: Ptr<NiStream>, destination: Ptr) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 2);
        e.call(function, &args![input, destination, 2u32, count, 1u32])
            .u32()
    });
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_0330, fn_00a30330() -> Ptr),
        entry!(0x00a3_0370, fn_00a30370(Ptr<NiStream>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a30330_builds_the_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xff; 0x40]);
            block.into_ret()
        });
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a3_0330, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(
            e.mem.u32(object),
            NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE
        );
        assert_eq!(e.mem.u16(object + 0x3c), 0);
        assert_eq!(e.mem.u16(object + 0x3e), 0xffff);
        assert_eq!(e.mem.u32(object + 0x34), 0);
        assert_eq!(e.mem.u32(object + 0x38), 0);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == INTERP_CONTROLLER_CONSTRUCT && w[0] == object));
        assert!(log.iter().any(|(a, w)| *a == OPERATOR_NEW && w[0] == 0x40));
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a3_0330, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00a30370_reads_two_bytes_through_the_input() {
        const READ: u32 = 0x0300_0010;
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        e.set(input, NiBinaryStream::m_uiAbsoluteCurrentPos, 6);
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            e.mem.set_u16(a[1], 0xbeef);
            2u32.into_ret()
        });
        let destination = e.mem.alloc(4);
        e.call(0x00a3_0370, &args![stream, destination]);
        assert_eq!(e.mem.u16(destination), 0xbeef);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 8);
    }
}

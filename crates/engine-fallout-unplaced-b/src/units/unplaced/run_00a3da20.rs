//! `(unplaced)/run_00a3da20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a3da20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word): reads 4 bytes from the stream's
/// input (`NiStream::m_pkIstr`) into `this`: the input's read function (+8) is called as
/// `function(input, this, 4, &count, 1)` with `count` (the stream argument's stack word) set
/// to 4, and the byte count it answers is added to the input's position.
pub fn fn_00a3da20(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(function, &args![input, this, 4u32, count, 1u32])
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
    vec![entry!(0x00a3_da20, fn_00a3da20(Ptr, Ptr<NiStream>))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a3da20_reads_four_bytes_into_this() {
        const READ: u32 = 0x0300_0010;
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        e.set(input, NiBinaryStream::m_uiAbsoluteCurrentPos, 6);
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], 0x1234_5678);
            3u32.into_ret()
        });
        let this = e.mem.alloc(4);
        e.call(0x00a3_da20, &args![this, stream]);
        assert_eq!(e.mem.u32(this), 0x1234_5678);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 9);
    }
}

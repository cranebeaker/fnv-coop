//! `NiAnimation/nitextkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiStream::LoadFixedString` and `NiStream::LoadCStringAsFixedString` (`thiscall` on the
/// stream, the fixed string to fill).
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;

// Translated from 00a578d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKey::LoadBinary` (Xbox PDB): the 4-byte time from the stream's input into the key
/// (`+0`; the stream argument's stack word is the byte-count word, set to 4), then the text
/// (`+4`): before file version `0x14010001` (`NiStream` +0xd8) as a C string made a fixed
/// string (`NiStream::LoadCStringAsFixedString`), from then on as a fixed string
/// (`NiStream::LoadFixedString`).
pub fn ni_text_key_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
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
    let text = this.addr() + 4;
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x1401_0001 {
        e.call(STREAM_LOAD_C_STRING_AS_FIXED_STRING, &args![stream, text]);
    } else {
        e.call(STREAM_LOAD_FIXED_STRING, &args![stream, text]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a5_78d0,
        ni_text_key_load_binary(Ptr, Ptr<NiStream>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_binary_reads_the_time_and_the_text_by_version() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_f32(a[1], 1.25);
            4u32.into_ret()
        });
        e.register(STREAM_LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(STREAM_LOAD_C_STRING_AS_FIXED_STRING, |_, _| Ret::default());
        let key = e.mem.alloc(8);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x1401_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a5_78d0, &args![key, stream]);
        assert_eq!(e.mem.f32(key), 1.25);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x1401_0001);
        e.call(0x00a5_78d0, &args![key, stream]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| {
                c.0 == STREAM_LOAD_FIXED_STRING || c.0 == STREAM_LOAD_C_STRING_AS_FIXED_STRING
            })
            .cloned()
            .collect();
        assert_eq!(
            calls,
            vec![
                (
                    STREAM_LOAD_C_STRING_AS_FIXED_STRING,
                    vec![stream.addr(), key + 4]
                ),
                (STREAM_LOAD_FIXED_STRING, vec![stream.addr(), key + 4]),
            ]
        );
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 8);
    }
}

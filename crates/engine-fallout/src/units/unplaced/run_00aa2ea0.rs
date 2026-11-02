//! `(unplaced)/run_00aa2ea0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory stream write body (`00aa2dd0`, `thiscall`, `RET 8`: source,
/// count); it is not translated here.
const MEM_STREAM_WRITE_BODY: u32 = 0x00aa_2dd0;

// Translated from 00aa2ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The write function `NiMemStream` (`00aa2fb0`) stores in `m_pfnWrite`. Ghidra
/// has no function at this address; the bytes are code (`MOV EAX,[ESP+0xc]` ...
/// `RET`) followed by `INT3` padding. `cdecl`, five stack words: the stream, the
/// source, the byte count, and two words this body never reads (the component
/// sizes array and the number of components `NiStream` passes). It calls
/// `00aa2dd0` with the stream as `this`, the source and the count, and answers
/// its result.
pub fn fn_00aa2ea0(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    count: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    e.call(MEM_STREAM_WRITE_BODY, &args![this, source, count])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa2ea0, fn_00aa2ea0(Ptr, u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_the_stream_source_and_count_and_answers_the_result() {
        let mut e = Engine::new();
        e.register(MEM_STREAM_WRITE_BODY, |_, a| Ret {
            eax: a[2] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(
            0x00aa2ea0,
            &args![0x5000u32, 0x6000u32, 9u32, 0x7000u32, 1u32],
        );
        assert_eq!(result.u32(), 10);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(MEM_STREAM_WRITE_BODY, vec![0x5000, 0x6000, 9])]);
    }
}

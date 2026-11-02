//! `NiAnimation/run_00a57930` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStream::SaveFixedString` (engine map; `thiscall` on the stream, the string's address).
const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;

// Translated from 00a57930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream, `RET 4`): writes the word at `this`
/// (4 bytes through the stream's output, `NiStream` +0x250: its write function +0xc, its
/// position +4 advanced by the answer; the stream argument's stack word is the byte-count
/// word), then the fixed string at +4 (`NiStream::SaveFixedString`).
pub fn fn_00a57930(e: &mut Engine, this: Ptr, stream: Ptr) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(function, &args![output, this, 4u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
    e.call(SAVE_FIXED_STRING, &args![stream, this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_7930, fn_00a57930(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_word_then_the_name() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x034c_0000);
        e.register(0x034c_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        e.register(SAVE_FIXED_STRING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(8);
        e.call(0x00a5_7930, &args![this, stream]);
        assert_eq!(e.mem.u32(output + 4), 4);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|c| c.0 == 0x034c_0000 || c.0 == SAVE_FIXED_STRING)
            .map(|c| (c.0, c.1[..2].to_vec()))
            .collect();
        assert_eq!(
            calls,
            vec![
                (0x034c_0000, vec![output, this]),
                (SAVE_FIXED_STRING, vec![stream, this + 4])
            ]
        );
    }
}

//! `(unplaced)/run_00a471d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStream` `+0x24c`: the binary stream it reads from; the stream keeps its
/// position at `+4` and the read function pointer at `+8`.
const STREAM_OFFSET: u32 = 0x24c;

// Translated from 00a471d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamLoadBinary<unsigned char>` (engine map name; `cdecl`, three stack
/// words): `file` is the `NiStream`. It calls the read function stored in the
/// binary stream at `file + 0x24c` (at `+8`) with (the stream, `buffer`,
/// `count`, the address of a local word holding 1, 1) and adds the number of
/// bytes it answers to the stream position at `+4`. The local word reuses the
/// stack slot of the first argument.
pub fn ni_stream_load_binary_u8(e: &mut Engine, file: Ptr, buffer: u32, count: u32) {
    let stream = e.mem.u32(file.addr() + STREAM_OFFSET);
    let read = e.mem.u32(stream + 8);
    let done = e.with_stack(4, |e, one| {
        e.mem.set_u32(one.addr(), 1);
        e.call(read, &args![stream, buffer, count, one, 1u32]).u32()
    });
    let position = e.mem.u32(stream + 4);
    e.mem.set_u32(stream + 4, position.wrapping_add(done));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a471d0, ni_stream_load_binary_u8(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_through_the_stream_function_and_advances_the_position() {
        let mut e = Engine::new();
        let reader = 0x0050_0001;
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen_in_double = seen.clone();
        e.register_double(reader, move |e, a| {
            seen_in_double.set(e.mem.u32(a[3]));
            Ret {
                eax: 3,
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x250);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(file + 0x24c, stream);
        e.mem.set_u32(stream + 4, 10);
        e.mem.set_u32(stream + 8, reader);
        e.call_log = Some(vec![]);
        e.call(0x00a471d0, &args![file, 0x9000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, reader);
        let words = &log[1].1;
        assert_eq!(
            (words[0], words[1], words[2], words[4]),
            (stream, 0x9000, 3, 1)
        );
        assert_eq!(seen.get(), 1);
        assert_eq!(e.mem.u32(stream + 4), 13);
    }
}

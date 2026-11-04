//! `(unplaced)/run_00a99a90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiExtraData::LoadBinary` (engine map), `00a7b4a0` (thiscall: stream).
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// Allocates through the Ni allocator (`00aa1070`, cdecl: the size).
const NI_ALLOC: u32 = 0x00aa_1070;

/// Reads `size` bytes into `buffer` through the read function at +8 of the
/// stream's input object (`NiStream` +0x24C; cdecl: input, buffer, size,
/// &count, 1, with `count` 4 as the code sets it, from the stream
/// argument's slot) and adds its result to the input's counter at +4.
fn read(e: &mut Engine, stream: u32, buffer: u32, size: u32) {
    // NiStream +0x24C: the stream's input object (read function at +8,
    // bytes-read counter at +4).
    let input = e.mem.u32(stream + 0x24c);
    let function = e.mem.u32(input + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let n = e
            .call(function, &[input, buffer, size, count.addr(), 1])
            .u32();
        let total = e.mem.u32(input + 4).wrapping_add(n);
        e.mem.set_u32(input + 4, total);
    });
}

// Translated from 00a99a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadBinary` of an array extra data (slot +0x4C of both
/// `NiIntegersExtraData`'s vtable `010a0c7c` and `NiFloatsExtraData`'s
/// `0109ec0c`, by their RTTI; also called by `00c61450`):
/// `NiExtraData::LoadBinary`, then the 4-byte count into +0x0C and, for a
/// count above 0, `count * 4` bytes into a block from `00aa1070` kept at
/// +0x10 (the count word passed to the read is 4 both times); a count of 0
/// leaves +0x10 null.
pub fn fn_00a99a90(e: &mut Engine, this: u32, stream: u32) {
    e.call(EXTRA_DATA_LOAD_BINARY, &[this, stream]);
    read(e, stream, this + 0xc, 4);
    let count = e.mem.u32(this + 0xc);
    if count == 0 {
        e.mem.set_u32(this + 0x10, 0);
        return;
    }
    let size = count.wrapping_mul(4);
    let values = e.call(NI_ALLOC, &[size]).u32();
    e.mem.set_u32(this + 0x10, values);
    read(e, stream, values, size);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9a90, fn_00a99a90(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00de_ad00;

    /// A stream whose input reads from `bytes` through a double at `READ`.
    fn stream(e: &mut Engine, bytes: Vec<u8>) -> u32 {
        let stream = e.mem.alloc(0x250);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, READ);
        let mut at = 0usize;
        e.register_double(READ, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            let n = a[2] as usize;
            e.mem.write(a[1], &bytes[at..at + n]);
            at += n;
            (n as u32).into_ret()
        });
        stream
    }

    #[test]
    fn values_follow_the_count() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        let s = stream(&mut e, vec![2, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0]);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x00a9_9a90, &[this, s]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EXTRA_DATA_LOAD_BINARY, vec![this, s]));
        let values = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!((e.mem.u32(values), e.mem.u32(values + 4)), (1, 9));
        let input = e.mem.u32(s + 0x24c);
        assert_eq!(e.mem.u32(input + 4), 12);
        // A count of 0 leaves no array.
        let s = stream(&mut e, vec![0, 0, 0, 0]);
        e.mem.set_u32(this + 0x10, 0x77);
        e.call(0x00a9_9a90, &[this, s]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}

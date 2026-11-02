//! `NiMain/run_00a86cd0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMatrix3::SaveBinary` (`00a59a90`, engine map; `thiscall`, the stream) and the
/// three-float save the engine map names `NiColor::SaveBinary` (`00a69780`, `thiscall`, the
/// stream).
const MATRIX_SAVE_BINARY: u32 = 0x00a5_9a90;
const THREE_FLOATS_SAVE_BINARY: u32 = 0x00a6_9780;

// Translated from 00a86cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (between `NiTransform::LoadBinary` and the next unit; `thiscall`
/// on a transform, the stream; `RET 4`): the save counterpart of the transform load: the
/// rotation matrix at +0 (`00a59a90`), the three floats at +0x24 (`00a69780`), then the
/// 4-byte scale at +0x30 through the stream output's write function (`NiStream` +0x250, the
/// function +0xc, the position +4 advanced by its answer), the byte count in the stream
/// argument's own word.
pub fn fn_00a86cd0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let this = this.addr();
    e.call(MATRIX_SAVE_BINARY, &args![this, stream]);
    e.call(THREE_FLOATS_SAVE_BINARY, &args![this + 0x24, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        e.mem.set_u32(count, stream.addr());
        let output = e.mem.u32(stream.addr() + 0x250);
        let write = e.mem.u32(output + 0xc);
        e.mem.set_u32(count, 4);
        let done = e
            .call(write, &args![output, this + 0x30, 4u32, count, 1u32])
            .u32();
        let position = e.mem.u32(output + 4).wrapping_add(done);
        e.mem.set_u32(output + 4, position);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_6cd0, fn_00a86cd0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_matrix_translation_and_scale() {
        let mut e = Engine::new();
        e.register(MATRIX_SAVE_BINARY, |_, _| Ret::default());
        e.register(THREE_FLOATS_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(output + 4, 6);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6cd0, &args![this, stream]);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(log[0], (MATRIX_SAVE_BINARY, vec![this, stream]));
        assert_eq!(
            log[1],
            (THREE_FLOATS_SAVE_BINARY, vec![this + 0x24, stream])
        );
        assert_eq!(log[2].0, 0x0300_0000);
        assert_eq!(&log[2].1[..3], &[output, this + 0x30, 4]);
        assert_eq!(e.mem.u32(output + 4), 10);
    }
}

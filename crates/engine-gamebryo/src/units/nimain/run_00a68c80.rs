//! `NiMain/run_00a68c80` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C; the input stream's
/// `m_uiAbsoluteCurrentPos` (+4) and `m_pfnRead` (+8, `cdecl` (stream,
/// buffer, size, &component size, component count), returns the bytes
/// read).
const STREAM_INPUT: u32 = 0x24c;
/// `NiGeometry::CopyMembers` (`00a5ac90`, engine map; `thiscall`, clone and
/// cloning process).
const GEOMETRY_COPY_MEMBERS: u32 = 0x00a5_ac90;

// Translated from 00a68c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream; `RET 4`): reads two
/// 4-byte components into `this` and `this + 4` through the stream input's
/// read function, adding each answer to the input's position. The texture
/// transform loader (`00a88f00`) calls it for its points, so by its use it
/// is the inlined two-float point load. The component size word is the
/// argument's own stack slot.
pub fn fn_00a68c80(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.with_stack(4, |e, size_word| {
        let size_word = size_word.addr();
        e.mem.set_u32(size_word, stream);
        for offset in [0u32, 4] {
            let input = e.mem.u32(stream + STREAM_INPUT);
            let read = e.mem.u32(input + 8);
            e.mem.set_u32(size_word, 4);
            let count = e
                .call(read, &args![input, this + offset, 4u32, size_word, 1u32])
                .u32();
            let position = e.mem.u32(input + 4);
            e.mem.set_u32(input + 4, position.wrapping_add(count));
        }
    });
}

// Translated from 00a68d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, clone and cloning process; `RET
/// 8`): a geometry class's member copy: `NiGeometry::CopyMembers`
/// (`00a5ac90`), then the words at +0xA4 and +0xA8 and the byte at +0x9C
/// copied into the clone (fields unnamed here).
pub fn fn_00a68d40(e: &mut Engine, this: Ptr, clone: u32, cloning: u32) {
    let this = this.addr();
    // `ECX` is still `this` at the call (the exe copies it to `ESI` only
    // after the pushes): `CopyMembers` runs on this object.
    e.call(GEOMETRY_COPY_MEMBERS, &args![this, clone, cloning]);
    let first = e.mem.u32(this + 0xa4);
    e.mem.set_u32(clone + 0xa4, first);
    let second = e.mem.u32(this + 0xa8);
    e.mem.set_u32(clone + 0xa8, second);
    let flag = e.mem.u8(this + 0x9c);
    e.mem.set_u8(clone + 0x9c, flag);
}

/// `NiStream::m_pkOstr` (Xbox PDB) +0x250; the output stream's position (+4) and write
/// function (+0xc, `cdecl` (stream, buffer, size, &component size, component count),
/// returns the bytes written).
const STREAM_OUTPUT: u32 = 0x250;

// Translated from 00a68ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the stream; `RET 4`): the save counterpart of
/// [`fn_00a68c80`]: writes the two 4-byte components at `this` and `this + 4` through the
/// stream output's write function, adding each answer to the output's position. The
/// component size word is the argument's own stack slot.
pub fn fn_00a68ce0(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    e.with_stack(4, |e, size_word| {
        let size_word = size_word.addr();
        e.mem.set_u32(size_word, stream);
        for offset in [0u32, 4] {
            let output = e.mem.u32(stream + STREAM_OUTPUT);
            let write = e.mem.u32(output + 0xc);
            e.mem.set_u32(size_word, 4);
            let count = e
                .call(write, &args![output, this + offset, 4u32, size_word, 1u32])
                .u32();
            let position = e.mem.u32(output + 4);
            e.mem.set_u32(output + 4, position.wrapping_add(count));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a68c80, fn_00a68c80(Ptr, u32)),
        entry!(0x00a68d40, fn_00a68d40(Ptr, u32, u32)),
        entry!(0x00a6_8ce0, fn_00a68ce0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_two_components_and_advances() {
        let mut e = Engine::new();
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 4, 10);
        e.mem.set_u32(input + 8, 0x0017_0000);
        e.register(0x0017_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let n = e.mem.u32(a[0] + 0xc) + 1;
            e.mem.set_u32(a[0] + 0xc, n);
            e.mem.set_u32(a[1], 0x100 * n);
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let point = e.mem.alloc(8);
        e.call(0x00a6_8c80, &args![point, stream]);
        assert_eq!(e.mem.u32(point), 0x100);
        assert_eq!(e.mem.u32(point + 4), 0x200);
        assert_eq!(e.mem.u32(input + 4), 18);
    }

    #[test]
    fn copies_the_geometry_members() {
        let mut e = Engine::new();
        e.register(GEOMETRY_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        let clone = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa4, 7);
        e.mem.set_u32(this + 0xa8, 8);
        e.mem.set_u8(this + 0x9c, 9);
        e.call_log = Some(vec![]);
        e.call(0x00a6_8d40, &args![this, clone, 0x33u32]);
        assert_eq!(e.mem.u32(clone + 0xa4), 7);
        assert_eq!(e.mem.u32(clone + 0xa8), 8);
        assert_eq!(e.mem.u8(clone + 0x9c), 9);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(GEOMETRY_COPY_MEMBERS, vec![this, clone, 0x33])));
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    #[test]
    fn writes_two_components_and_advances() {
        let mut e = Engine::new();
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, 10);
        e.mem.set_u32(output + 0xc, 0x0017_0000);
        e.register(0x0017_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            3u32.into_ret()
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let point = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00a6_8ce0, &args![point, stream]);
        let writes: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0017_0000)
            .map(|(_, w)| (w[0], w[1]))
            .collect();
        assert_eq!(writes, vec![(output, point), (output, point + 4)]);
        assert_eq!(e.mem.u32(output + 4), 16);
    }
}

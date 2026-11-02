//! `NiMain/run_00a7ced0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The stream load and save of an `NiPixelFormat` (Xbox PDB layout, 0x44 bytes: `m_uFlags`
//! +0, `m_ucBitsPerPixel` +1, `m_eFormat` +4, `m_eTiling` +8, `m_uiRendererHint` +0xc,
//! `m_uiExtraData` +0x10, then four 12-byte component specifications from +0x14: two words,
//! a byte and a `bool`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pixel-format setter from a format, bits per pixel and channel masks (`00a7c9f0`,
/// `thiscall`, three words).
const PIXEL_FORMAT_FROM_MASKS: u32 = 0x00a7_c9f0;

/// Reads `size` bytes from the stream's input (`NiStream` +0x24c, the read function +8, the
/// position +4) into `destination`, the byte-count word at `count` set to `size` first.
fn stream_read(e: &mut Engine, stream: u32, destination: u32, size: u32, count: u32) {
    let input = e.mem.u32(stream + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![input, destination, size, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

/// Writes `size` bytes at `source` to the stream's output (`NiStream` +0x250, the write
/// function +0xc, the position +4), the byte-count word at `count` set to `size` first.
fn stream_write(e: &mut Engine, stream: u32, source: u32, size: u32, count: u32) {
    let output = e.mem.u32(stream + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![output, source, size, count, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a7ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPixelFormat`, the stream; `RET 4`): loads
/// the pixel format.
///
/// - Files older than version `0x0a030003` (`NiStream` +0xd8): the format (4 bytes, stored
///   at +4), four 4-byte channel masks into the frame, the bits per pixel (4 bytes, the low
///   byte stored at +1), two 4-byte words read and dropped, and from version `0x0a00010b`
///   one more; then [`fn_00a7c9f0`](`00a7c9f0`) with the format, the bits and the masks.
/// - Newer files: the format (4 bytes, through a frame word), the bits per pixel (1 byte),
///   the renderer hint (+0xc) and extra data (+0x10, 4 bytes each), the flags (+0, 1 byte),
///   the tiling (4 bytes, through a frame word), then for each of the four components its
///   two words (through frame words), its byte (+8) and its `bool` (+9: the byte read is
///   not 0).
///
/// The byte-count and buffer words are the exe's frame words as the disassembly uses them:
/// eight locals (`W0`..`W7`; `W0` the loop counter, `W4`..`W7` the masks) and the stream
/// argument's own word, which holds the old path's mask pointer and the new path's `bool`
/// bytes.
pub fn fn_00a7ced0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let this = this.addr();
    let stream = stream.addr();
    // Frame: W0..W7 at +0..+0x1c, the return address word +0x20, the argument word +0x24.
    e.with_stack(0x28, |e, frame| {
        let w = |i: u32| frame.addr() + 4 * i;
        let argument = frame.addr() + 0x24;
        e.mem.set_u32(argument, stream);
        if e.mem.u32(stream + 0xd8) < 0x0a03_0003 {
            stream_read(e, stream, w(0), 4, argument);
            let format = e.mem.u32(w(0));
            e.mem.set_u32(this + 4, format);
            e.mem.set_u32(argument, w(4));
            e.mem.set_u32(w(0), 4);
            loop {
                let mask = e.mem.u32(argument);
                stream_read(e, stream, mask, 4, w(1));
                e.mem.set_u32(argument, mask.wrapping_add(4));
                let left = e.mem.u32(w(0)).wrapping_sub(1);
                e.mem.set_u32(w(0), left);
                if left == 0 {
                    break;
                }
            }
            stream_read(e, stream, w(2), 4, argument);
            let bits = e.mem.u8(w(2));
            e.mem.set_u8(this + 1, bits);
            stream_read(e, stream, w(3), 4, argument);
            stream_read(e, stream, w(3), 4, argument);
            if e.mem.u32(stream + 0xd8) >= 0x0a00_010b {
                stream_read(e, stream, w(1), 4, argument);
            }
            let bits = u32::from(e.mem.u8(this + 1));
            let format = e.mem.u32(this + 4);
            e.call(PIXEL_FORMAT_FROM_MASKS, &args![this, format, bits, w(4)]);
            return;
        }
        stream_read(e, stream, w(3), 4, argument);
        let format = e.mem.u32(w(3));
        e.mem.set_u32(this + 4, format);
        stream_read(e, stream, this + 1, 1, argument);
        stream_read(e, stream, this + 0xc, 4, argument);
        stream_read(e, stream, this + 0x10, 4, argument);
        stream_read(e, stream, this, 1, argument);
        stream_read(e, stream, w(3), 4, argument);
        let tiling = e.mem.u32(w(3));
        e.mem.set_u32(this + 8, tiling);
        let mut component = this + 0x18;
        e.mem.set_u32(w(0), 4);
        loop {
            stream_read(e, stream, w(2), 4, w(3));
            let first = e.mem.u32(w(2));
            e.mem.set_u32(component - 4, first);
            stream_read(e, stream, w(1), 4, w(3));
            let second = e.mem.u32(w(1));
            e.mem.set_u32(component, second);
            stream_read(e, stream, component + 4, 1, w(3));
            stream_read(e, stream, argument, 1, w(3));
            let flag = e.mem.u8(argument) > 0;
            e.mem.set_u8(component + 5, flag as u8);
            component += 0xc;
            let left = e.mem.u32(w(0)).wrapping_sub(1);
            e.mem.set_u32(w(0), left);
            if left == 0 {
                break;
            }
        }
    });
}

// Translated from 00a7d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPixelFormat`, the stream; `RET 4`): saves
/// the pixel format the way [`fn_00a7ced0`] loads newer files: the format (+4, 4 bytes), the
/// bits per pixel (+1, 1 byte), the renderer hint (+0xc) and extra data (+0x10), the flags
/// (+0, 1 byte), the tiling (+8), then for each of the four components its two words (+0x14,
/// +0x18), its byte (+0x1c) and its `bool` (+0x1d, copied into the low byte of the stream
/// argument's word and written from there). The byte-count words are the argument's word
/// (the fields) and the second local (the components).
pub fn fn_00a7d1a0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let this = this.addr();
    let stream = stream.addr();
    // Frame: two locals at +0 and +4 (the loop counter and the components' count word), the
    // return address word +8, the argument word +0xc.
    e.with_stack(0x10, |e, frame| {
        let counter = frame.addr();
        let count = frame.addr() + 4;
        let argument = frame.addr() + 0xc;
        e.mem.set_u32(argument, stream);
        stream_write(e, stream, this + 4, 4, argument);
        stream_write(e, stream, this + 1, 1, argument);
        stream_write(e, stream, this + 0xc, 4, argument);
        stream_write(e, stream, this + 0x10, 4, argument);
        stream_write(e, stream, this, 1, argument);
        stream_write(e, stream, this + 8, 4, argument);
        let mut component = this + 0x18;
        e.mem.set_u32(counter, 4);
        loop {
            stream_write(e, stream, component - 4, 4, count);
            stream_write(e, stream, component, 4, count);
            stream_write(e, stream, component + 4, 1, count);
            let flag = e.mem.u8(component + 5);
            e.mem.set_u8(argument, flag);
            stream_write(e, stream, argument, 1, count);
            component += 0xc;
            let left = e.mem.u32(counter).wrapping_sub(1);
            e.mem.set_u32(counter, left);
            if left == 0 {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_ced0, fn_00a7ced0(Ptr, Ptr)),
        entry!(0x00a7_d1a0, fn_00a7d1a0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const IO: u32 = 0x0300_0000;

    type Calls = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A stream whose read function hands out the next of `values` (little-endian, cut to
    /// the size) and records (destination, size, count word).
    fn reading(version: u32, values: Vec<u32>) -> (Engine, u32, u32, Calls) {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x260);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(input + 8, IO);
        let calls: Calls = Rc::default();
        let seen = calls.clone();
        let mut next = values.into_iter();
        e.register_double(IO, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            seen.borrow_mut().push((a[1], a[2], a[3]));
            let value = next.next().unwrap();
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        e.register(PIXEL_FORMAT_FROM_MASKS, |_, _| Ret::default());
        let this = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        (e, this, stream, calls)
    }

    #[test]
    fn old_files_read_the_masks_and_ask_for_the_preset() {
        let values = vec![3, 0xf800, 0x7e0, 0x1f, 0, 0x110, 7, 8, 9];
        let (mut e, this, stream, calls) = reading(0x0a00_010b, values);
        e.call(0x00a7_ced0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u8(this + 1), 0x10);
        let set: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == PIXEL_FORMAT_FROM_MASKS)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(set.len(), 1);
        assert_eq!(&set[0][..3], &[this, 3, 0x10]);
        let masks = set[0][3];
        let read: Vec<_> = (0..4).map(|i| e.mem.u32(masks + 4 * i)).collect();
        assert_eq!(read, vec![0xf800, 0x7e0, 0x1f, 0]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 9);
        // The masks' count word is another frame word than the argument's.
        assert_eq!(calls[1].0, masks);
        assert_ne!(calls[1].2, calls[0].2);
        assert!(calls.iter().all(|c| c.1 == 4));
        // Before version 0x0a00010b the last word is not read.
        let values = vec![3, 1, 2, 3, 4, 0x20, 7, 8];
        let (mut e, this, stream, calls) = reading(0x0a00_010a, values);
        e.call(0x00a7_ced0, &args![this, stream]);
        assert_eq!(calls.borrow().len(), 8);
        assert_eq!(e.mem.u8(this + 1), 0x20);
    }

    #[test]
    fn new_files_read_every_field() {
        let mut values = vec![5, 0x20, 0x11, 0x22, 0x81, 2];
        for k in 0..4u32 {
            values.extend([0x100 + k, 0x200 + k, 0x30 + k, k]);
        }
        let (mut e, this, stream, calls) = reading(0x0a03_0003, values);
        e.mem.write(this, &[0xaa; 0x44]);
        e.call(0x00a7_ced0, &args![this, stream]);
        assert_eq!(e.mem.u8(this), 0x81);
        assert_eq!(e.mem.u8(this + 1), 0x20);
        assert_eq!(e.mem.u16(this + 2), 0xaaaa);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 0x11);
        assert_eq!(e.mem.u32(this + 0x10), 0x22);
        for k in 0..4u32 {
            let component = this + 0x14 + 0xc * k;
            assert_eq!(e.mem.u32(component), 0x100 + k);
            assert_eq!(e.mem.u32(component + 4), 0x200 + k);
            assert_eq!(e.mem.u8(component + 8), 0x30 + k as u8);
            assert_eq!(e.mem.u8(component + 9), u8::from(k != 0));
            assert_eq!(e.mem.u16(component + 0xa), 0xaaaa);
        }
        let calls = calls.borrow();
        assert_eq!(calls.len(), 22);
        let sizes: Vec<_> = calls.iter().map(|c| c.1).collect();
        assert_eq!(&sizes[..10], &[4, 1, 4, 4, 1, 4, 4, 4, 1, 1]);
        assert!(e
            .call_log
            .iter()
            .flatten()
            .all(|(a, _)| *a != PIXEL_FORMAT_FROM_MASKS));
    }

    #[test]
    fn save_writes_every_field_in_order() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, IO);
        let calls: Calls = Rc::default();
        let seen = calls.clone();
        e.register_double(IO, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value = match a[2] {
                1 => u32::from(e.mem.u8(a[1])),
                _ => e.mem.u32(a[1]),
            };
            seen.borrow_mut().push((a[1], a[2], value));
            a[2].into_ret()
        });
        let this = e.mem.alloc(0x44);
        for i in 0..0x11u32 {
            e.mem.set_u32(this + 4 * i, 0x0101_0101 * i);
        }
        e.call(0x00a7_d1a0, &args![this, stream]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 22);
        let fields: Vec<_> = calls[..6].iter().map(|c| (c.0 - this, c.1)).collect();
        assert_eq!(
            fields,
            vec![(4, 4), (1, 1), (0xc, 4), (0x10, 4), (0, 1), (8, 4)]
        );
        // Each component's bool goes through a frame word with the field's byte.
        for k in 0..4u32 {
            let call = calls[6 + 4 * k as usize + 3];
            assert_eq!(call.1, 1);
            assert_eq!(call.2, u32::from(e.mem.u8(this + 0x1d + 0xc * k)));
            assert!(call.0 < this || call.0 >= this + 0x44);
        }
        assert_eq!(e.mem.u32(output + 4), 18 + 4 * 10);
    }
}

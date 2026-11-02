//! `Havok SDK/hkdisplayserializeostream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplaySerializeOStream` is an `hkOArchive` (`00d60830` constructs the base): its words
//! are written with `hkOArchive::writeArrayFloat32` (`00d60b00`, `(this, array, count)`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{load_vec, store_vec};

/// The memory router (`00c85750`): its field `+0x10` is the allocator the writer comes from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkBufferedStreamWriter::hkBufferedStreamWriter` (`00d27460`, `this`, `(stream, buffer
/// size)`, returns `this`), `hkOArchive::hkOArchive` (`00d60830`, `this`, `(writer, byte-swap
/// flag word)`), the reference release (`00c905b0`, `this` only) and `hkOArchive::
/// writeArrayFloat32` (`00d60b00`, `this`, `(array, count)`).
const BUFFERED_WRITER_CONSTRUCT: u32 = 0x00d2_7460;
const ARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const WRITE_ARRAY_FLOAT32: u32 = 0x00d6_0b00;
/// `hkQuaternion::set` (`00cb26e0`, `this` = the quaternion, one word: the rotation rows).
const QUATERNION_SET: u32 = 0x00cb_26e0;
/// The vtable `hkDisplaySerializeOStream`'s constructor stores at `+0`.
const OSTREAM_VTABLE: u32 = 0x010d_7db8;

// Translated from 00dda6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeOStream::hkDisplaySerializeOStream` (Xbox PDB), thiscall, `RET 4`: makes
/// a 0x1c-byte `hkBufferedStreamWriter` over the argument (the router's allocator, slot `4`; its
/// 16-bit size at `+4` set to 0x1c; the constructor gets the argument and a buffer size of
/// 0x2000), constructs the `hkOArchive` base on it with a byte-swap flag whose low byte is 0
/// (the callee reads one byte of the word; its other bytes are whatever the exe's register held
/// and are 0 here), stores the vtable and releases one reference on the writer the base took.
/// Returns `this`.
pub fn hk_display_serialize_o_stream_constructor(e: &mut Engine, this: Ptr, stream: u32) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let writer = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(writer + 4, 0x1c);
    let writer = e
        .call(BUFFERED_WRITER_CONSTRUCT, &args![writer, stream, 0x2000u32])
        .u32();
    e.call(ARCHIVE_CONSTRUCT, &args![this, writer, 0u32]);
    let held = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(this.addr(), OSTREAM_VTABLE);
    e.call(REMOVE_REFERENCE, &args![held]);
    this
}

// Translated from 00dda730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`): writes the first three floats of the vector at
/// `vector` (`hkOArchive::writeArrayFloat32(vector, 3)`); returns what that leaves in EAX.
pub fn fn_00dda730(e: &mut Engine, this: Ptr, vector: u32) -> u32 {
    e.call(WRITE_ARRAY_FLOAT32, &args![this, vector, 3u32])
        .u32()
}

// Translated from 00dda740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeOStream::writeTransform` (Xbox PDB), thiscall, `RET 4`: copies the
/// transform's three rotation rows (`+0`, `+0x10`, `+0x20`) and its translation row (`+0x30`) to
/// the frame, makes a quaternion from the rows (`hkQuaternion::set`), writes the translation's
/// first three floats and then the quaternion's four (`writeArrayFloat32`). Returns what the
/// last write leaves in EAX.
pub fn hk_display_serialize_o_stream_write_transform(
    e: &mut Engine,
    this: Ptr,
    transform: u32,
) -> u32 {
    e.with_stack(0x80, |e, block| {
        let b = (block.addr() + 0xf) & !0xf;
        let row = load_vec(e, transform);
        store_vec(e, b + 0x30, row);
        for word in 0..4 {
            let value = e.mem.u32(transform + 0x30 + 4 * word);
            e.mem.set_u32(b + 0x10 + 4 * word, value);
        }
        let row = load_vec(e, transform + 0x10);
        store_vec(e, b + 0x40, row);
        let row = load_vec(e, transform + 0x20);
        store_vec(e, b + 0x50, row);
        e.call(QUATERNION_SET, &args![b + 0x20, b + 0x30]);
        e.call(WRITE_ARRAY_FLOAT32, &args![this, b + 0x10, 3u32]);
        e.call(WRITE_ARRAY_FLOAT32, &args![this, b + 0x20, 4u32])
            .u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_a6e0, hk_display_serialize_o_stream_constructor(Ptr, u32) -> Ptr),
        entry!(0x00dd_a730, fn_00dda730(Ptr, u32) -> u32),
        entry!(0x00dd_a740, hk_display_serialize_o_stream_write_transform(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE_DOUBLE: u32 = 0x4900_0004;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A router whose allocator's slot 4 hands out blocks.
    fn install_router(e: &mut Engine) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
    }

    #[test]
    fn the_constructor_builds_the_writer_and_the_archive_base() {
        let mut e = Engine::new();
        install_router(&mut e);
        e.register(BUFFERED_WRITER_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // The archive constructor keeps the writer at +8.
        e.register(ARCHIVE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00dd_a6e0, &args![this, 0x5000u32]).u32(), this);
        let writer = e.mem.u32(this + 8);
        assert_eq!(e.mem.u16(writer + 4), 0x1c);
        assert_eq!(e.mem.u32(this), OSTREAM_VTABLE);
        assert_eq!(
            calls_to(&e, BUFFERED_WRITER_CONSTRUCT),
            vec![vec![writer, 0x5000, 0x2000]]
        );
        assert_eq!(calls_to(&e, ARCHIVE_CONSTRUCT), vec![vec![this, writer, 0]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![writer]]);
    }

    #[test]
    fn the_vector_writer_writes_three_floats() {
        let mut e = Engine::new();
        e.register(WRITE_ARRAY_FLOAT32, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00dd_a730, &args![0x1000u32, 0x2000u32]).u32(),
            0x77
        );
        assert_eq!(
            calls_to(&e, WRITE_ARRAY_FLOAT32),
            vec![vec![0x1000, 0x2000, 3]]
        );
    }

    #[test]
    fn write_transform_writes_the_translation_then_the_quaternion() {
        let mut e = Engine::new();
        // The quaternion double writes the first lane of each row into the quaternion's lanes.
        e.register(QUATERNION_SET, |e, a| {
            for row in 0..3 {
                let value = e.mem.u32(a[1] + 0x10 * row);
                e.mem.set_u32(a[0] + 4 * row, value);
            }
            e.mem.set_u32(a[0] + 12, 0x3f80_0000);
            Ret::default()
        });
        // The write double records the floats it is given.
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = std::rc::Rc::clone(&written);
        e.register_double(WRITE_ARRAY_FLOAT32, move |e, a| {
            let words: Vec<u32> = (0..a[2]).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push(words);
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
        let transform = (e.mem.alloc(0x50) + 15) & !15;
        for row in 0..4u32 {
            for lane in 0..4u32 {
                e.mem
                    .set_u32(transform + 0x10 * row + 4 * lane, 0x100 * (row + 1) + lane);
            }
        }
        assert_eq!(
            e.call(0x00dd_a740, &args![0x1000u32, transform]).u32(),
            0x55
        );
        assert_eq!(
            *written.borrow(),
            vec![
                vec![0x400, 0x401, 0x402],
                vec![0x100, 0x200, 0x300, 0x3f80_0000]
            ]
        );
    }
}

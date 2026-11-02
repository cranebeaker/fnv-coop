//! `NiMain/nivectorextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiVectorExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_VECTOR_EXTRA_DATA_RTTI: u32 = 0x011f_5e84;

// Translated from 00a92070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_vector_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_VECTOR_EXTRA_DATA_RTTI)
}

/// `NiVectorExtraData`'s vtable (its RTTI locator names `.?AVNiVectorExtraData@@`) and
/// object size; the vector is four floats at +0xc.
const NI_VECTOR_EXTRA_DATA_VTABLE: u32 = 0x0109_fb94;
const NI_VECTOR_EXTRA_DATA_SIZE: u32 = 0x1c;
const VECTOR: u32 = 0xc;
/// `00aa13e0` (`cdecl`, the size): `NiNew`'s allocator; `00aa1460` (`cdecl`, the block and
/// its size): its release; `00aa1070` (`cdecl`, the size): the viewer-string allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiExtraData` (unit `niextradata.cpp`): `LoadBinary` (`00a7b4a0`), `SaveBinary`
/// (`00a7b2c0`) and `GetViewerStrings` (`00a7b580`), each `thiscall` with one word, and three
/// functions the engine map leaves unnamed: the constructor (`00a7b2e0`), the destructor
/// body (`00a7b300`) and the member copy (`00a7b330`, clone and cloning process).
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// The three-float load the engine map names `NiColor::LoadBinary` (`00a59be0`, `thiscall`
/// on the three floats, the stream).
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiGetViewerString` of a class name (`00a7d330`, `cdecl`), `NiSprintf` (`00a305b0`,
/// `cdecl`), and the viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_SPRINTF: u32 = 0x00a3_05b0;
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"Vector = (%5.3f,%5.3f,%5.3f,%5.3f)"`.
const FORMAT_VECTOR: u32 = 0x0109_fc28;

// Translated from 00a92020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::IsEqual` (Xbox PDB): false for a null other object; otherwise true
/// only when the four floats at +0xc are equal one by one (`FUCOMPP`: a NaN is equal to
/// nothing). The base class is not compared.
pub fn ni_vector_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    for i in 0..4u32 {
        let mine = x87_load32(e.mem.u32(this.addr() + VECTOR + 4 * i));
        let theirs = x87_load32(e.mem.u32(other.addr() + VECTOR + 4 * i));
        if mine != theirs {
            return false;
        }
    }
    true
}

// Translated from 00a92080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::_scalar_deleting_destructor_` (Xbox PDB): the class's vtable, the
/// `NiExtraData` destructor body (`00a7b300`), then, when bit 0 of the flags is set, the
/// 0x1c-byte block released (`00aa1460`). Answers `this`.
pub fn ni_vector_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_VECTOR_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, NI_VECTOR_EXTRA_DATA_SIZE]);
    }
    this
}

// Translated from 00a920b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::CreateClone` (Xbox PDB): a new 0x1c-byte object (`00aa13e0`, the
/// `NiExtraData` constructor `00a7b2e0`, the vector zeroed with `FLDZ`/`FST`, the class's
/// vtable; null when the allocation fails), the base's member copy (`00a7b330`, also run
/// with a null clone), then the four floats copied into it (`FLD`/`FSTP`, quieting a
/// signalling NaN). Answers the clone.
pub fn ni_vector_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_VECTOR_EXTRA_DATA_SIZE]).u32();
    if block != 0 {
        e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
        e.mem.set_f32(block + 0x18, 0.0);
        e.mem.set_u32(block, NI_VECTOR_EXTRA_DATA_VTABLE);
        for offset in [0x14u32, 0x10, 0xc] {
            e.mem.set_f32(block + offset, 0.0);
        }
    }
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, block, cloning_process],
    );
    for i in 0..4u32 {
        let value = x87_quiet_bits(e.mem.u32(this.addr() + VECTOR + 4 * i));
        e.mem.set_u32(block.wrapping_add(VECTOR + 4 * i), value);
    }
    Ptr::new(block)
}

/// Reads `size` bytes from the stream's input (`NiStream` +0x24c, the read function +8, the
/// position +4) into `destination`, the byte-count word at `count` set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32, count: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
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
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32, count: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![output, source, size, count, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a92110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`; files older than
/// version `0x0500000e` (`NiStream` +0xd8) hold three floats, loaded into frame locals by
/// `00a59be0` and copied in (`FLD`/`FSTP`) with the fourth float set to 0; newer files read
/// the four floats (4 bytes each, the byte count in the stream argument's own word).
pub fn ni_vector_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) < 0x0500_000e {
        e.with_stack(0xc, |e, point| {
            let point = point.addr();
            e.call(THREE_FLOATS_LOAD_BINARY, &args![point, stream]);
            for i in 0..3u32 {
                let value = x87_quiet_bits(e.mem.u32(point + 4 * i));
                e.mem.set_u32(base + VECTOR + 4 * i, value);
            }
        });
        e.mem.set_f32(base + 0x18, 0.0);
        return;
    }
    e.with_stack(4, |e, count| {
        let count = count.addr();
        e.mem.set_u32(count, stream.addr());
        for i in 0..4u32 {
            stream_read(e, stream, base + VECTOR + 4 * i, 4, count);
        }
    });
}

// Translated from 00a921f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary` (`00a7b2c0`), then
/// the four floats (4 bytes each, the byte count in the stream argument's own word).
pub fn ni_vector_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        e.mem.set_u32(count, stream.addr());
        for i in 0..4u32 {
            stream_write(e, stream, base + VECTOR + 4 * i, 4, count);
        }
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a92290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVectorExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`
/// (`00a7b580`), then two strings appended: the class name (`NiGetViewerString` of the first
/// word of the `NiRTTI` at `011f5e84`) and `"Vector = (x,y,z,w)"` (`%5.3f` each; a 0x37-byte
/// text from `00aa1070` written by `NiSprintf` with the four floats as `double`s, `FLD` then
/// `FSTP double`, which quiets a signalling NaN). Each string pointer is kept in the
/// `strings` argument's stack word, whose address is passed.
pub fn ni_vector_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let base = this.addr();
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_VECTOR_EXTRA_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let text = e.call(ALLOCATE_TEXT, &args![0x37u32]).u32();
        let component = |e: &Engine, i: u32| x87_load32(e.mem.u32(base + VECTOR + 4 * i));
        let (x, y, z, w) = (
            component(e, 0),
            component(e, 1),
            component(e, 2),
            component(e, 3),
        );
        e.mem.set_u32(slot, text);
        e.call(NI_SPRINTF, &args![text, 0x37u32, FORMAT_VECTOR, x, y, z, w]);
        append_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a92070, ni_vector_extra_data_get_rtti() -> Ptr),
        entry!(0x00a9_2020, ni_vector_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_2080,
            ni_vector_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_20b0, ni_vector_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_2110, ni_vector_extra_data_load_binary(Ptr, Ptr)),
        entry!(0x00a9_21f0, ni_vector_extra_data_save_binary(Ptr, Ptr)),
        entry!(
            0x00a9_2290,
            ni_vector_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_vector_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a92070, &args![]).u32(),
            NI_VECTOR_EXTRA_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod class_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w)
            .collect()
    }

    fn vector(e: &mut Engine, values: [f32; 4]) -> u32 {
        let object = e.mem.alloc(NI_VECTOR_EXTRA_DATA_SIZE);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(object + VECTOR + 4 * i as u32, *value);
        }
        object
    }

    #[test]
    fn is_equal_compares_the_four_floats() {
        let mut e = Engine::new();
        let a = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        assert!(e.call(0x00a9_2020, &args![a, b]).bool());
        assert!(!e.call(0x00a9_2020, &args![a, 0u32]).bool());
        for i in 0..4u32 {
            let c = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
            e.mem.set_f32(c + VECTOR + 4 * i, 9.0);
            assert!(!e.call(0x00a9_2020, &args![a, c]).bool());
        }
        let n = vector(&mut e, [f32::NAN, 2.0, 3.0, 4.0]);
        assert!(!e.call(0x00a9_2020, &args![n, n]).bool());
    }

    #[test]
    fn deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(NI_VECTOR_EXTRA_DATA_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_2080, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_VECTOR_EXTRA_DATA_VTABLE);
        assert!(calls(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00a9_2080, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED), vec![vec![this, 0x1c]]);
        assert_eq!(calls(&e, EXTRA_DATA_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_copies_the_vector() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        let this = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(this + 0x18, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a9_20b0, &args![this, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_VECTOR_EXTRA_DATA_VTABLE);
        assert_eq!(
            calls(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        assert_eq!(e.mem.f32(clone + 0xc), 1.0);
        assert_eq!(e.mem.f32(clone + 0x14), 3.0);
        assert_eq!(e.mem.u32(clone + 0x18), 0x7fc0_0001);
    }

    fn stream_engine(version: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 8, 0x0300_0000);
        e.mem.set_u32(io + 0xc, 0x0300_0000);
        let mut next = 1.5f32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            e.mem.set_f32(a[1], next);
            next += 1.0;
            4u32.into_ret()
        });
        e.call_log = Some(vec![]);
        (e, stream, io)
    }

    #[test]
    fn load_binary_reads_four_floats_or_three_and_zero() {
        let (mut e, stream, io) = stream_engine(0x0500_000e);
        let this = vector(&mut e, [9.0; 4]);
        e.call(0x00a9_2110, &args![this, stream]);
        assert_eq!(calls(&e, EXTRA_DATA_LOAD_BINARY), vec![vec![this, stream]]);
        let read: Vec<_> = (0..4).map(|i| e.mem.f32(this + 0xc + 4 * i)).collect();
        assert_eq!(read, vec![1.5, 2.5, 3.5, 4.5]);
        assert_eq!(e.mem.u32(io + 4), 16);
        let (mut e, stream, _) = stream_engine(0x0500_000d);
        e.register(THREE_FLOATS_LOAD_BINARY, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[0] + 4 * i, 7.0 + i as f32);
            }
            Ret::default()
        });
        let this = vector(&mut e, [9.0; 4]);
        e.call(0x00a9_2110, &args![this, stream]);
        assert_eq!(calls(&e, THREE_FLOATS_LOAD_BINARY)[0][1], stream);
        let read: Vec<_> = (0..4).map(|i| e.mem.f32(this + 0xc + 4 * i)).collect();
        assert_eq!(read, vec![7.0, 8.0, 9.0, 0.0]);
    }

    #[test]
    fn save_binary_writes_the_four_floats() {
        let (mut e, stream, io) = stream_engine(0);
        let this = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00a9_21f0, &args![this, stream]);
        assert_eq!(calls(&e, EXTRA_DATA_SAVE_BINARY), vec![vec![this, stream]]);
        let sources: Vec<_> = calls(&e, 0x0300_0000).iter().map(|w| w[1]).collect();
        assert_eq!(
            sources,
            vec![this + 0xc, this + 0x10, this + 0x14, this + 0x18]
        );
        assert_eq!(e.mem.u32(io + 4), 16);
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_vector() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_VECTOR_EXTRA_DATA_RTTI, 0x0109_0000);
        for f in [
            EXTRA_DATA_GET_VIEWER_STRINGS,
            ARRAY_SET_SIZE,
            ARRAY_SET_AT,
            NI_SPRINTF,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(GET_VIEWER_STRING, |_, _| 0x1234u32.into_ret());
        e.register(ALLOCATE_TEXT, |_, _| 0x5678u32.into_ret());
        let this = vector(&mut e, [1.0, 2.0, 3.0, 0.5]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a9_2290, &args![this, strings]);
        assert_eq!(calls(&e, GET_VIEWER_STRING), vec![vec![0x0109_0000]]);
        let printed = calls(&e, NI_SPRINTF);
        assert_eq!(&printed[0][..3], &[0x5678, 0x37, FORMAT_VECTOR]);
        let w = 0.5f64.to_bits();
        assert_eq!(&printed[0][9..], &[w as u32, (w >> 32) as u32]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE),
            vec![vec![strings, 5], vec![strings, 5]]
        );
        assert_eq!(calls(&e, ARRAY_SET_AT).len(), 2);
    }
}

//! `fallout/misc/saveload/bgsunloadedformbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BGSUnloadedFormBuffer` owns one heap block (`pBuffer`): a two-byte
//! header (byte 0 carries the size type in its top two bits, as `00864520`
//! reads it; byte 1 is a version byte), then the data size in the size
//! type's encoding (`SaveGameSizeToBuffer` / `LoadGameSizeFromBuffer`), then
//! the data.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSUnloadedFormBuffer` (Xbox PDB), 4 bytes.
    pub struct BGSUnloadedFormBuffer: 0x04 {
        /// `pBuffer` (Xbox PDB): `char*`.
        0x00 pBuffer: u32,
    }
}

/// `operator new` (`00401000`, cdecl `(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401460` (cdecl `(destination, source, count)`): forwards its three
/// words to `00ec44d0`.
const COPY: u32 = 0x0040_1460;
/// `00864450` (thiscall, no stack words): initializes the 9-byte record
/// `Save` and `Load` keep on the stack (it calls `0084eaf0` on it,
/// `0044dee0` on +3, `00864490` on +7 and clears +8).
const RECORD_INITIALIZER: u32 = 0x0086_4450;
/// `00864520` (thiscall on a header byte): the byte shifted right by 6, the
/// size type.
const HEADER_SIZE_TYPE: u32 = 0x0086_4520;
/// `00865aa0` (thiscall `(header, size_type)`, `RET 4`).
const HEADER_SET_SIZE_TYPE: u32 = 0x0086_5aa0;
/// `BGSSaveLoadFormInfo::SetFormType` (`00853670`, engine map; thiscall
/// `(header, form_type)`, `RET 4`).
const HEADER_SET_FORM_TYPE: u32 = 0x0085_3670;
/// `00853640` (thiscall): looks the low six bits of the byte at `this` up in
/// the word table at `011a2428`.
const FORM_TYPE_OF: u32 = 0x0085_3640;
/// `BGSNumericIDIndex::SetNumericID` (`00853570`, engine map; thiscall
/// `(record, id)`, `RET 4`).
const SET_NUMERIC_ID: u32 = 0x0085_3570;
/// `BGSSaveLoadFile::Write` (`00846330`, engine map; thiscall
/// `(file, data, count)`, `RET 8`).
const FILE_WRITE: u32 = 0x0084_6330;
/// `BGSSaveLoadFile::Read` (`00846300`, engine map; thiscall
/// `(file, data, count)`, `RET 8`).
const FILE_READ: u32 = 0x0084_6300;
/// `GetSizeTypeForSize` (`00845d40`, engine map; cdecl `(size)`).
const SIZE_TYPE_FOR_SIZE: u32 = 0x0084_5d40;
/// `GetSizeTypeSize` (`00845d70`, engine map; cdecl `(size_type)`).
const SIZE_TYPE_SIZE: u32 = 0x0084_5d70;
/// `SaveGameSizeToFile` (`00845d80`, engine map; cdecl
/// `(file, size, size_type)`).
const SAVE_SIZE_TO_FILE: u32 = 0x0084_5d80;
/// `SaveGameSizeToBuffer` (`00845de0`, engine map; cdecl
/// `(buffer, size, size_type)`).
const SAVE_SIZE_TO_BUFFER: u32 = 0x0084_5de0;
/// `LoadGameSizeFromFile` (`00845e50`, engine map; cdecl
/// `(file, &size, size_type)`), returns the bytes the size took.
const LOAD_SIZE_FROM_FILE: u32 = 0x0084_5e50;
/// `LoadGameSizeFromBuffer` (`00845ee0`, engine map; cdecl
/// `(buffer, &size, size_type)`), returns the bytes the size took.
const LOAD_SIZE_FROM_BUFFER: u32 = 0x0084_5ee0;
/// `BGSLoadFormBuffer::BGSLoadFormBuffer` (`008643b0`, engine map; thiscall,
/// no stack words, returns `this`).
const LOAD_FORM_BUFFER_CONSTRUCTOR: u32 = 0x0086_43b0;
/// `BGSLoadGameBuffer::SetBuffer` (`008647f0`, engine map; thiscall
/// `(buffer, total_size, offset)`, `RET 0xc`).
const LOAD_GAME_BUFFER_SET_BUFFER: u32 = 0x0086_47f0;
/// `0084b450` (thiscall on a `BGSUnloadedFormBuffer`): the byte at
/// `pBuffer + 1`.
const VERSION_BYTE: u32 = 0x0084_b450;
/// `008a8150` (thiscall `(load_form_buffer, byte)`, `RET 4`).
const LOAD_FORM_BUFFER_SET_BYTE: u32 = 0x008a_8150;
/// `BGSLoadFormBuffer::GetVersion` (`008a81c0`, engine map; thiscall, byte
/// result).
const LOAD_FORM_BUFFER_GET_VERSION: u32 = 0x008a_81c0;
/// `0084e3a0` (thiscall on the load form buffer, no stack words): the size
/// `00866570` passes to `Initialize`.
const LOAD_FORM_BUFFER_SIZE: u32 = 0x0084_e3a0;
/// `00726070` (thiscall on the load form buffer, no stack words): the data
/// `00866570` passes to `Initialize`.
const LOAD_FORM_BUFFER_DATA: u32 = 0x0072_6070;

fn buffer(e: &Engine, this: Ptr<BGSUnloadedFormBuffer>) -> u32 {
    e.get(this, BGSUnloadedFormBuffer::pBuffer)
}

// Translated from 00537e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` (Xbox PDB name from the
/// engine map): stores `buffer` in `pBuffer` (the body is a one-word store,
/// which identical-code folding may share with other methods).
pub fn bgs_unloaded_form_buffer_bgs_unloaded_form_buffer(
    e: &mut Engine,
    this: Ptr<BGSUnloadedFormBuffer>,
    buffer: u32,
) {
    e.set(this, BGSUnloadedFormBuffer::pBuffer, buffer);
}

// Translated from 00841100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer_ov2`
/// (Xbox PDB), but the body is a folded "or into the first word":
/// `*this |= bits`.
pub fn fn_00841100(e: &mut Engine, this: Ptr, bits: u32) {
    let value = e.mem.u32(this.addr()) | bits;
    e.mem.set_u32(this.addr(), value);
}

// Translated from 00866200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::Save` (Xbox PDB): writes a 9-byte record to
/// `file` (initialized by `00864450`, numeric id set from `id`, `word` at +3,
/// the buffer's two header bytes at +7 and +8), then the data size
/// (`SaveGameSizeToFile`, in the header's size type) and, if it is not zero,
/// the data.
pub fn bgs_unloaded_form_buffer_save(
    e: &mut Engine,
    this: Ptr<BGSUnloadedFormBuffer>,
    file: u32,
    id: u32,
    word: u32,
) {
    let header = buffer(e, this);
    let mut data = header.wrapping_add(2);
    // The frame from EBP-0x20: the record at -0x1c, the size at -0x8.
    e.with_stack(0x20, |e, frame| {
        let record = frame.addr() + 4;
        let size = frame.addr() + 0x18;
        e.call(RECORD_INITIALIZER, &args![record]);
        e.call(SET_NUMERIC_ID, &args![record, id]);
        e.mem.set_u32(record + 3, word);
        let first = e.mem.u8(header);
        e.mem.set_u8(record + 7, first);
        let second = e.mem.u8(header.wrapping_add(1));
        e.mem.set_u8(record + 8, second);
        e.call(FILE_WRITE, &args![file, record, 9u32]);
        let size_type = e.call(HEADER_SIZE_TYPE, &args![record + 7]).u32();
        e.mem.set_u32(size, 0);
        let taken = e
            .call(LOAD_SIZE_FROM_BUFFER, &args![data, size, size_type])
            .u32();
        data = taken.wrapping_add(data);
        let count = e.mem.u32(size);
        e.call(SAVE_SIZE_TO_FILE, &args![file, count, size_type]);
        if count != 0 {
            e.call(FILE_WRITE, &args![file, data, count]);
        }
    });
}

// Translated from 008662c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::Load` (Xbox PDB): reads the 9-byte record `Save`
/// writes and the data size from `file`, allocates `size + 2 + size bytes`,
/// stores the record's bytes +7 and +8 as the header, the size after it and,
/// if it is not zero, reads the data behind that.
pub fn bgs_unloaded_form_buffer_load(e: &mut Engine, this: Ptr<BGSUnloadedFormBuffer>, file: u32) {
    // The frame from EBP-0x2c: the record at -0x24, the size at -0x4.
    e.with_stack(0x2c, |e, frame| {
        let record = frame.addr() + 8;
        let size = frame.addr() + 0x28;
        e.call(RECORD_INITIALIZER, &args![record]);
        e.call(FILE_READ, &args![file, record, 9u32]);
        let size_type = e.call(HEADER_SIZE_TYPE, &args![record + 7]).u32();
        e.mem.set_u32(size, 0);
        let taken = e
            .call(LOAD_SIZE_FROM_FILE, &args![file, size, size_type])
            .u32();
        let count = e.mem.u32(size);
        let total = count.wrapping_add(taken).wrapping_add(2);
        let block = e.call(OPERATOR_NEW, &args![total]).u32();
        e.set(this, BGSUnloadedFormBuffer::pBuffer, block);
        let header = buffer(e, this);
        let second = e.mem.u8(record + 8);
        e.mem.set_u8(header.wrapping_add(1), second);
        let first = e.mem.u8(record + 7);
        e.mem.set_u8(header, first);
        let sized = header.wrapping_add(2);
        e.call(SAVE_SIZE_TO_BUFFER, &args![sized, count, size_type]);
        let data = sized.wrapping_add(taken);
        if count != 0 {
            e.call(FILE_READ, &args![file, data, count]);
        }
    });
}

// Translated from 00866390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::GetAdvancedBuffer` (Xbox PDB): the address of the
/// data (past the header and the size), or 0 without a buffer.
pub fn bgs_unloaded_form_buffer_get_advanced_buffer(
    e: &mut Engine,
    this: Ptr<BGSUnloadedFormBuffer>,
) -> u32 {
    let header = buffer(e, this);
    if header == 0 {
        return 0;
    }
    let size_type = e.call(HEADER_SIZE_TYPE, &args![header]).u32();
    let size_bytes = e.call(SIZE_TYPE_SIZE, &args![size_type]).u32();
    buffer(e, this).wrapping_add(size_bytes).wrapping_add(2)
}

// Translated from 008663e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::GetSize` (Xbox PDB): the data size stored after
/// the header, or 0 without a buffer.
pub fn bgs_unloaded_form_buffer_get_size(e: &mut Engine, this: Ptr<BGSUnloadedFormBuffer>) -> u32 {
    if buffer(e, this) == 0 {
        return 0;
    }
    let size_type = fn_00866430(e, this);
    let sized = buffer(e, this).wrapping_add(2);
    e.with_stack(4, |e, size| {
        e.call(LOAD_SIZE_FROM_BUFFER, &args![sized, size, size_type]);
        e.mem.u32(size.addr())
    })
}

// Translated from 00866430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the size type of the buffer's header
/// (`00864520` on `pBuffer`; EAX is its result).
pub fn fn_00866430(e: &mut Engine, this: Ptr<BGSUnloadedFormBuffer>) -> u32 {
    let header = buffer(e, this);
    e.call(HEADER_SIZE_TYPE, &args![header]).u32()
}

// Translated from 00866450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the bytes of the header's size type plus 9
/// (the record `Save` writes before the size).
pub fn fn_00866450(e: &mut Engine, this: Ptr<BGSUnloadedFormBuffer>) -> u32 {
    let size_type = fn_00866430(e, this);
    let size_bytes = e.call(SIZE_TYPE_SIZE, &args![size_type]).u32();
    size_bytes.wrapping_add(9)
}

// Translated from 00866480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::CreateLoadFormBuffer` (Xbox PDB): allocates and
/// constructs a 0x30-byte `BGSLoadFormBuffer` (null if the allocation
/// failed), passes it the header's version byte (`0084b450`, `008a8150`;
/// both run even for a null object) and points it at this buffer
/// (`SetBuffer(pBuffer, size + offset, offset)`, the offset being 2 plus
/// the bytes of the size). Returns the new object.
// Translation incomplete: 00866480 - FS:[0] exception frame (unwind of the allocation if the constructor throws) is not translated.
pub fn bgs_unloaded_form_buffer_create_load_form_buffer(
    e: &mut Engine,
    this: Ptr<BGSUnloadedFormBuffer>,
) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let load = if block == 0 {
        0
    } else {
        e.call(LOAD_FORM_BUFFER_CONSTRUCTOR, &args![block]).u32()
    };
    let version = e.call(VERSION_BYTE, &args![this]).u8();
    e.call(LOAD_FORM_BUFFER_SET_BYTE, &args![load, version as u32]);
    let size_type = fn_00866430(e, this);
    let sized = buffer(e, this).wrapping_add(2);
    let (taken, count) = e.with_stack(4, |e, size| {
        let taken = e
            .call(LOAD_SIZE_FROM_BUFFER, &args![sized, size, size_type])
            .u32();
        (taken, e.mem.u32(size.addr()))
    });
    let offset = taken.wrapping_add(2);
    let header = buffer(e, this);
    e.call(
        LOAD_GAME_BUFFER_SET_BUFFER,
        &args![load, header, count.wrapping_add(offset), offset],
    );
    load
}

// Translated from 00866570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called by `UnloadForm`): fills this buffer from
/// a load form buffer: `Initialize(data, size, form_type, version)` with the
/// version (`BGSLoadFormBuffer::GetVersion`), the form type (`008665b0`), the
/// size (`0084e3a0`) and the data (`00726070`), read in that order. `RET 4`.
pub fn fn_00866570(e: &mut Engine, this: Ptr<BGSUnloadedFormBuffer>, load: u32) {
    let version = e.call(LOAD_FORM_BUFFER_GET_VERSION, &args![load]).u8();
    let form_type = fn_008665b0(e, Ptr::new(load));
    let size = e.call(LOAD_FORM_BUFFER_SIZE, &args![load]).u32();
    let data = e.call(LOAD_FORM_BUFFER_DATA, &args![load]).u32();
    bgs_unloaded_form_buffer_initialize(e, this, data, size, form_type, version);
}

// Translated from 008665b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00853640` on `this + 0x1b` (the table word for
/// the low six bits of that byte); EAX is its result.
pub fn fn_008665b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FORM_TYPE_OF, &args![this.addr().wrapping_add(0x1b)])
        .u32()
}

// Translated from 008665d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSUnloadedFormBuffer::Initialize` (Xbox PDB): allocates
/// `size + 2 + size bytes`, stores `version` in header byte 1, the size type
/// for `size` and `form_type` in the header (`00865aa0`, `SetFormType`), the
/// size after the header and copies `size` bytes of `data` behind it.
/// `RET 0x10`.
pub fn bgs_unloaded_form_buffer_initialize(
    e: &mut Engine,
    this: Ptr<BGSUnloadedFormBuffer>,
    data: u32,
    size: u32,
    form_type: u32,
    version: u8,
) {
    let size_type = e.call(SIZE_TYPE_FOR_SIZE, &args![size]).u32();
    let size_bytes = e.call(SIZE_TYPE_SIZE, &args![size_type]).u32();
    let total = size.wrapping_add(size_bytes).wrapping_add(2);
    let block = e.call(OPERATOR_NEW, &args![total]).u32();
    e.set(this, BGSUnloadedFormBuffer::pBuffer, block);
    let header = buffer(e, this);
    e.mem.set_u8(header.wrapping_add(1), version);
    e.call(HEADER_SET_SIZE_TYPE, &args![header, size_type]);
    e.call(HEADER_SET_FORM_TYPE, &args![header, form_type]);
    let sized = header.wrapping_add(2);
    e.call(SAVE_SIZE_TO_BUFFER, &args![sized, size, size_type]);
    e.call(COPY, &args![sized.wrapping_add(size_bytes), data, size]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00537e90,
            bgs_unloaded_form_buffer_bgs_unloaded_form_buffer(Ptr<BGSUnloadedFormBuffer>, u32)
        ),
        entry!(0x00841100, fn_00841100(Ptr, u32)),
        entry!(
            0x00866200,
            bgs_unloaded_form_buffer_save(Ptr<BGSUnloadedFormBuffer>, u32, u32, u32)
        ),
        entry!(
            0x008662c0,
            bgs_unloaded_form_buffer_load(Ptr<BGSUnloadedFormBuffer>, u32)
        ),
        entry!(
            0x00866390,
            bgs_unloaded_form_buffer_get_advanced_buffer(Ptr<BGSUnloadedFormBuffer>) -> u32
        ),
        entry!(
            0x008663e0,
            bgs_unloaded_form_buffer_get_size(Ptr<BGSUnloadedFormBuffer>) -> u32
        ),
        entry!(0x00866430, fn_00866430(Ptr<BGSUnloadedFormBuffer>) -> u32),
        entry!(0x00866450, fn_00866450(Ptr<BGSUnloadedFormBuffer>) -> u32),
        entry!(
            0x00866480,
            bgs_unloaded_form_buffer_create_load_form_buffer(Ptr<BGSUnloadedFormBuffer>) -> u32
        ),
        entry!(0x00866570, fn_00866570(Ptr<BGSUnloadedFormBuffer>, u32)),
        entry!(0x008665b0, fn_008665b0(Ptr) -> u32),
        entry!(
            0x008665d0,
            bgs_unloaded_form_buffer_initialize(Ptr<BGSUnloadedFormBuffer>, u32, u32, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for the size helpers: the size type is the header byte >> 6,
    /// a size type takes `type + 1` bytes; a size is read from its first
    /// byte and written as a word (enough to follow the arithmetic).
    fn size_doubles(e: &mut Engine) {
        e.register(HEADER_SIZE_TYPE, |e, a| {
            ((e.mem.u8(a[0]) >> 6) as u32).into_ret()
        });
        e.register(SIZE_TYPE_SIZE, |_, a| (a[0] + 1).into_ret());
        e.register(SIZE_TYPE_FOR_SIZE, |_, a| {
            (if a[0] < 0x100 { 0u32 } else { 1 }).into_ret()
        });
        e.register(LOAD_SIZE_FROM_BUFFER, |e, a| {
            let v = e.mem.u8(a[0]) as u32;
            e.mem.set_u32(a[1], v);
            (a[2] + 1).into_ret()
        });
        e.register(SAVE_SIZE_TO_BUFFER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0] + 8).into_ret());
    }

    /// A buffer: header (size type 1, version 7), size 3 (two bytes), data
    /// "abc".
    fn sample_buffer(e: &mut Engine) -> (Ptr<BGSUnloadedFormBuffer>, u32) {
        let block = e.mem.alloc(16);
        e.mem.write(block, &[0x45, 7, 3, 0, b'a', b'b', b'c']);
        let this = e.new_object::<BGSUnloadedFormBuffer>();
        e.set(this, BGSUnloadedFormBuffer::pBuffer, block);
        (this, block)
    }

    #[test]
    fn constructor_stores_and_ov2_ors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.call(0x00537e90, &args![this, 0x0f0u32]);
        assert_eq!(e.mem.u32(this), 0x0f0);
        e.call(0x00841100, &args![this, 0x10fu32]);
        assert_eq!(e.mem.u32(this), 0x1ff);
    }

    #[test]
    fn save_writes_record_size_and_data() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        e.register(RECORD_INITIALIZER, |e, a| {
            e.mem.write(a[0], &[0xee; 9]);
            a[0].into_ret()
        });
        e.register(SET_NUMERIC_ID, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        e.register(FILE_WRITE, |_, _| Ret::default());
        e.register(SAVE_SIZE_TO_FILE, |_, _| Ret::default());
        let (this, block) = sample_buffer(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00866200, &args![this, 0x500u32, 0x12u32, 0x0403_0201u32]);
        let log = e.call_log.take().unwrap();
        let writes = calls_to(&log, FILE_WRITE);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][0], 0x500);
        assert_eq!(writes[0][2], 9);
        assert_eq!(writes[1], vec![0x500, block + 4, 3]);
        assert_eq!(calls_to(&log, SAVE_SIZE_TO_FILE), vec![vec![0x500, 3, 1]]);
        assert_eq!(calls_to(&log, LOAD_SIZE_FROM_BUFFER)[0][0], block + 2);
        // Size zero: no data write.
        e.mem.set_u8(block + 2, 0);
        e.call_log = Some(vec![]);
        e.call(0x00866200, &args![this, 0x500u32, 0x12u32, 0u32]);
        assert_eq!(calls_to(&e.call_log.take().unwrap(), FILE_WRITE).len(), 1);
    }

    #[test]
    fn save_record_bytes() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        e.register(RECORD_INITIALIZER, |e, a| {
            e.mem.write(a[0], &[0xee; 9]);
            a[0].into_ret()
        });
        e.register(SET_NUMERIC_ID, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        e.register(SAVE_SIZE_TO_FILE, |_, _| Ret::default());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let s = seen.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            s.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });
        let (this, _) = sample_buffer(&mut e);
        e.call(0x00866200, &args![this, 0x500u32, 0x12u32, 0x0403_0201u32]);
        let seen = seen.borrow();
        assert_eq!(seen[0], vec![0x12, 0xee, 0xee, 1, 2, 3, 4, 0x45, 7]);
        assert_eq!(seen[1], b"abc".to_vec());
    }

    #[test]
    fn load_reads_record_size_and_data() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        e.register(RECORD_INITIALIZER, |_, a| a[0].into_ret());
        e.register(FILE_READ, |e, a| {
            if a[2] == 9 {
                e.mem.write(a[1], &[0, 0, 0, 0, 0, 0, 0, 0x85, 9]);
            } else {
                e.mem.write(a[1], &vec![b'z'; a[2] as usize]);
            }
            Ret::default()
        });
        e.register(LOAD_SIZE_FROM_FILE, |e, a| {
            e.mem.set_u32(a[1], 5);
            (a[2] + 1).into_ret()
        });
        let this = e.new_object::<BGSUnloadedFormBuffer>();
        e.call_log = Some(vec![]);
        e.call(0x008662c0, &args![this, 0x600u32]);
        let log = e.call_log.take().unwrap();
        // Size type 2 takes 3 bytes: 5 + 3 + 2.
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![10]]);
        let block = e.get(this, BGSUnloadedFormBuffer::pBuffer);
        assert_eq!(
            e.mem.bytes(block, 10),
            vec![0x85, 9, 5, 0, 0, b'z', b'z', b'z', b'z', b'z']
        );
        assert_eq!(calls_to(&log, FILE_READ)[1], vec![0x600, block + 5, 5]);
        assert_eq!(
            calls_to(&log, SAVE_SIZE_TO_BUFFER),
            vec![vec![block + 2, 5, 2]]
        );
        // Size zero: no data read.
        e.register(LOAD_SIZE_FROM_FILE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008662c0, &args![this, 0x600u32]);
        assert_eq!(calls_to(&e.call_log.take().unwrap(), FILE_READ).len(), 1);
    }

    #[test]
    fn advanced_buffer_size_and_size_type() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        let (this, block) = sample_buffer(&mut e);
        assert_eq!(e.call(0x00866390, &args![this]).u32(), block + 4);
        assert_eq!(e.call(0x008663e0, &args![this]).u32(), 3);
        assert_eq!(e.call(0x00866430, &args![this]).u32(), 1);
        assert_eq!(e.call(0x00866450, &args![this]).u32(), 11);
        let empty = e.new_object::<BGSUnloadedFormBuffer>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00866390, &args![empty]).u32(), 0);
        assert_eq!(e.call(0x008663e0, &args![empty]).u32(), 0);
        assert_eq!(
            e.call_log.take().unwrap().len(),
            2,
            "no callee without a buffer"
        );
    }

    #[test]
    fn create_load_form_buffer_points_it_at_the_data() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        e.register(LOAD_FORM_BUFFER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(VERSION_BYTE, |e, a| {
            let b = e.mem.u32(a[0]);
            (e.mem.u8(b + 1) as u32 | 0xab00).into_ret()
        });
        e.register(LOAD_FORM_BUFFER_SET_BYTE, |_, _| Ret::default());
        e.register(LOAD_GAME_BUFFER_SET_BUFFER, |_, _| Ret::default());
        let (this, block) = sample_buffer(&mut e);
        e.call_log = Some(vec![]);
        let load = e.call(0x00866480, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x30]]);
        assert_ne!(load, 0);
        assert_eq!(
            calls_to(&log, LOAD_FORM_BUFFER_SET_BYTE),
            vec![vec![load, 7]]
        );
        // Offset 2 + 2 size bytes, total 3 + 4.
        assert_eq!(
            calls_to(&log, LOAD_GAME_BUFFER_SET_BUFFER),
            vec![vec![load, block, 7, 4]]
        );
        // A failed allocation leaves a null object, still passed on.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00866480, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LOAD_FORM_BUFFER_CONSTRUCTOR).is_empty());
        assert_eq!(calls_to(&log, LOAD_GAME_BUFFER_SET_BUFFER)[0][0], 0);
    }

    #[test]
    fn initialize_from_a_load_form_buffer() {
        let mut e = Engine::new();
        size_doubles(&mut e);
        e.register(LOAD_FORM_BUFFER_GET_VERSION, |_, _| 0x1209u32.into_ret());
        e.register(FORM_TYPE_OF, |_, a| (a[0] + 0x1000).into_ret());
        e.register(LOAD_FORM_BUFFER_SIZE, |_, _| 0x104u32.into_ret());
        e.register(LOAD_FORM_BUFFER_DATA, |_, _| 0x7777u32.into_ret());
        e.register(HEADER_SET_SIZE_TYPE, |e, a| {
            e.mem.set_u8(a[0], (a[1] << 6) as u8);
            Ret::default()
        });
        e.register(HEADER_SET_FORM_TYPE, |_, _| Ret::default());
        e.register(COPY, |_, _| Ret::default());
        let this = e.new_object::<BGSUnloadedFormBuffer>();
        e.call_log = Some(vec![]);
        e.call(0x00866570, &args![this, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_TYPE_OF), vec![vec![0x91b]]);
        // Size 0x104 is size type 1 (2 bytes): 0x104 + 2 + 2.
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x108]]);
        let block = e.get(this, BGSUnloadedFormBuffer::pBuffer);
        assert_eq!(
            e.mem.bytes(block, 2),
            vec![0x40, 9],
            "only the version's low byte"
        );
        assert_eq!(calls_to(&log, HEADER_SET_SIZE_TYPE), vec![vec![block, 1]]);
        assert_eq!(
            calls_to(&log, HEADER_SET_FORM_TYPE),
            vec![vec![block, 0x191b]]
        );
        assert_eq!(
            calls_to(&log, SAVE_SIZE_TO_BUFFER),
            vec![vec![block + 2, 0x104, 1]]
        );
        assert_eq!(calls_to(&log, COPY), vec![vec![block + 4, 0x7777, 0x104]]);
        // The registered entry takes the version from the low byte of its word.
        e.call(0x008665d0, &args![this, 1u32, 2u32, 3u32, 0x1ffu32]);
        let block = e.get(this, BGSUnloadedFormBuffer::pBuffer);
        assert_eq!(e.mem.u8(block + 1), 0xff);
    }
}

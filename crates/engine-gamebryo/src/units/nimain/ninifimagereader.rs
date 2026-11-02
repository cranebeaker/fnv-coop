//! `NiMain/ninifimagereader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00fdf054`: import slot of `InitializeCriticalSection` (`stdcall`, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `00a66150` (`thiscall`, engine map unit `nistream.cpp`): constructs the part at `+0x100`.
const PART_CONSTRUCTOR: u32 = 0x00a6_6150;

// Translated from 00a8ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `ninifimagereader.cpp`; the body of a constructor,
/// `thiscall`, no stack word): sets the vtable `0109f3d0`, initializes the critical
/// section at `this + 0x80`, sets the vtable `0109f3e4`, constructs the part at
/// `this + 0x100` with `00a66150` and returns `this`.
pub fn fn_00a8ae50(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0109_f3d0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this.addr() + 0x80]);
    e.mem.set_u32(this.addr(), 0x0109_f3e4);
    e.call(PART_CONSTRUCTOR, &args![this.addr() + 0x100]);
    this
}

/// `00a65300` (`thiscall`, engine map unit `nistream.cpp`): destroys the part at
/// `+0x100` (the counterpart of `00a66150`).
const PART_DESTRUCTOR: u32 = 0x00a6_5300;
/// `00fdf058`: import slot of `DeleteCriticalSection` (`stdcall`, one word).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;

// Translated from 00a8ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `ninifimagereader.cpp`; the destructor body
/// matching `00a8ae50`, `thiscall`, no stack words): stores the vtable
/// `0109f3e4` (RTTI `NiNIFImageReader`), destroys the part at `this + 0x100`
/// (`00a65300`), stores the vtable `0109f3d0` (RTTI `NiImageReader`) and
/// deletes the critical section at `this + 0x80`.
pub fn fn_00a8ada0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_f3e4);
    e.call(PART_DESTRUCTOR, &args![this.addr() + 0x100]);
    e.mem.set_u32(this.addr(), 0x0109_f3d0);
    e.call(DELETE_CRITICAL_SECTION, &args![this.addr() + 0x80]);
}

/// `NiStricmp` (`00aa1360`, `cdecl`: two strings; 0 when equal ignoring case).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `".nif"` (`01020e98`).
const TEXT_NIF: u32 = 0x0102_0e98;
/// Import slots of `InterlockedIncrement` / `InterlockedDecrement` and of
/// `EnterCriticalSection` / `LeaveCriticalSection` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The memory manager singleton (`011f6238`) and its `Deallocate` (`00aa4060`, `thiscall`,
/// the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiStream::RemoveAllObjects` (`00a644f0`, `thiscall`) and the stream's `Load` (vtable
/// +4, the file name).
const STREAM_REMOVE_ALL_OBJECTS: u32 = 0x00a6_44f0;
const STREAM_LOAD_SLOT: u32 = 0x4;
/// The reader's `NiStream` `m_kStream` (Xbox PDB) +0x100; in it, the top objects' first
/// element block word (+0x220) and their 32-bit count (+0x228), as the PC code reads them.
const STREAM: u32 = 0x100;
const TOP_OBJECTS_BASE: u32 = 0x320;
const TOP_OBJECTS_COUNT: u32 = 0x328;
/// The reader's critical section (+0x80).
const LOCK: u32 = 0x80;
/// `NiPixelData::ms_RTTI` (`011f4a90`); an `NiRTTI`'s base class record is at +4.
const NI_PIXEL_DATA_RTTI: u32 = 0x011f_4a90;
const RTTI_BASE: u32 = 0x4;
/// `NiObject::GetRTTI` (vtable +8) and the reader's `ReadFile` (vtable +8).
const GET_RTTI_SLOT: u32 = 0x8;
const READ_FILE_SLOT: u32 = 0x8;

// Translated from 00a8ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNIFImageReader::CanReadFile` (Xbox PDB; virtual, `this` unused): whether the
/// extension is `".nif"` ignoring case (`NiStricmp`).
pub fn ni_nif_image_reader_can_read_file(e: &mut Engine, _this: Ptr, extension: Ptr) -> bool {
    e.call(NI_STRICMP, &args![extension, TEXT_NIF]).u32() == 0
}

// Translated from 00a8add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNIFImageReader::ReadHeader` (Xbox PDB; (file, width, height, format, mipmap,
/// faces)): reads the file through the reader's own `ReadFile` (vtable +8, with no pixel
/// data to reuse); false when that gives nothing. Otherwise, holding a reference, it copies
/// the pixel data's first width (`m_puiWidth[0]`, Xbox PDB +0x54) and height
/// (`m_puiHeight[0]`, +0x58), its 0x44-byte `m_kPixelFormat` (+8), whether
/// `m_uiMipmapLevels` (+0x60) is above 1 (a byte) and `m_uiFaces` (+0x6c), drops the
/// reference (`DeleteThis`, slot +4, at zero) and answers true.
pub fn ni_nif_image_reader_read_header(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
    width: Ptr,
    height: Ptr,
    format: Ptr,
    mipmap: Ptr,
    faces: Ptr,
) -> bool {
    let data = e
        .vcall(this.addr(), READ_FILE_SLOT, &args![file, 0u32])
        .u32();
    if data == 0 {
        return false;
    }
    e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    let first_width = e.mem.u32(e.mem.u32(data + 0x54));
    e.mem.set_u32(width.addr(), first_width);
    let first_height = e.mem.u32(e.mem.u32(data + 0x58));
    e.mem.set_u32(height.addr(), first_height);
    for word in 0..0x11 {
        let value = e.mem.u32(data + 8 + 4 * word);
        e.mem.set_u32(format.addr() + 4 * word, value);
    }
    let levels = e.mem.u32(data + 0x60);
    e.mem.set_u8(mipmap.addr(), u8::from(levels > 1));
    let count = e.mem.u32(data + 0x6c);
    e.mem.set_u32(faces.addr(), count);
    if e.call(INTERLOCKED_DECREMENT, &args![data + 4]).u32() == 0 {
        e.vcall(data, 4, &args![]);
    }
    true
}

// Translated from 00a8ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNIFImageReader::_scalar_deleting_destructor_` (Xbox PDB): the inlined destructor
/// (the vtable `0109f3e4`, the stream at +0x100 destroyed by `00a65300`, the vtable
/// `0109f3d0`, the critical section at +0x80 deleted), then the block given back to the
/// memory manager (`00aa4060` on `011f6238`) when bit 0 of `flags` is set. Answers `this`.
pub fn ni_nif_image_reader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0109_f3e4);
    e.call(PART_DESTRUCTOR, &args![this.addr() + STREAM]);
    e.mem.set_u32(this.addr(), 0x0109_f3d0);
    e.call(DELETE_CRITICAL_SECTION, &args![this.addr() + LOCK]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a8aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNIFImageReader::ReadFile` (Xbox PDB; (file, a second word it never reads)): under the
/// reader's critical section the stream drops its objects (`NiStream::RemoveAllObjects`)
/// and loads the file (its vtable +4); when it holds exactly one top object whose class
/// derives from `NiPixelData` (its RTTI chain, through the base records at +4, reaches
/// `011f4a90`), that object is the answer, otherwise null.
pub fn ni_nif_image_reader_read_file(e: &mut Engine, this: Ptr, file: Ptr, _unused_2: u32) -> Ptr {
    let reader = this.addr();
    let lock = reader + LOCK;
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let stream = reader + STREAM;
    e.call(STREAM_REMOVE_ALL_OBJECTS, &args![stream]);
    e.vcall(stream, STREAM_LOAD_SLOT, &args![file]);
    if e.mem.u32(reader + TOP_OBJECTS_COUNT) == 1 {
        let object = e.mem.u32(e.mem.u32(reader + TOP_OBJECTS_BASE));
        if object != 0 {
            let mut rtti = e.vcall(object, GET_RTTI_SLOT, &args![]).u32();
            while rtti != 0 {
                if rtti == NI_PIXEL_DATA_RTTI {
                    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
                    return Ptr::new(object);
                }
                rtti = e.mem.u32(rtti + RTTI_BASE);
            }
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    Ptr::NULL
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_ad80,
            ni_nif_image_reader_can_read_file(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a8_add0,
            ni_nif_image_reader_read_header(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a8_ae80,
            ni_nif_image_reader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a8_aec0,
            ni_nif_image_reader_read_file(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(0x00a8_ae50, fn_00a8ae50(Ptr) -> Ptr),
        entry!(0x00a8_ada0, fn_00a8ada0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_the_lock_then_the_part_and_ends_on_the_second_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        e.register_double(INITIALIZE_CRITICAL_SECTION, move |e, a| {
            assert_eq!(a, &[this + 0x80]);
            assert_eq!(e.mem.u32(this), 0x0109_f3d0);
            Ret::default()
        });
        e.register_double(PART_CONSTRUCTOR, move |e, a| {
            assert_eq!(a, &[this + 0x100]);
            assert_eq!(e.mem.u32(this), 0x0109_f3e4);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_ae50, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(
            order,
            vec![0x00a8_ae50, INITIALIZE_CRITICAL_SECTION, PART_CONSTRUCTOR]
        );
        assert_eq!(e.mem.u32(this), 0x0109_f3e4);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn fn_00a8ada0_destroys_the_part_then_the_lock() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        e.register_double(PART_DESTRUCTOR, move |e, a| {
            assert_eq!(a, &[this + 0x100]);
            assert_eq!(e.mem.u32(this), 0x0109_f3e4);
            Ret::default()
        });
        e.register_double(DELETE_CRITICAL_SECTION, move |e, a| {
            assert_eq!(a, &[this + 0x80]);
            assert_eq!(e.mem.u32(this), 0x0109_f3d0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_ada0, &args![this]);
        let order: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(
            order,
            vec![0x00a8_ada0, PART_DESTRUCTOR, DELETE_CRITICAL_SECTION]
        );
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable slots point at `targets + slot`.
    fn object(e: &mut Engine, size: u32, targets: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for slot in 0..0x10 {
            e.mem.set_u32(vtable + 4 * slot, targets + 4 * slot);
        }
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, vtable);
        p
    }

    #[test]
    fn can_read_file_accepts_nif() {
        let mut e = Engine::new();
        e.register(NI_STRICMP, |e, a| {
            let name = e.mem.cstr(a[0]).to_ascii_lowercase();
            u32::from(name != b".nif").into_ret()
        });
        let text = e.mem.alloc(8);
        e.mem.write(text, b".NIF\0");
        assert!(e.call(0x00a8_ad80, &args![0u32, text]).bool());
        e.mem.write(text, b".dds\0");
        assert!(!e.call(0x00a8_ad80, &args![0u32, text]).bool());
    }

    #[test]
    fn read_header_copies_the_pixel_data_description() {
        let mut e = Engine::new();
        let reader = object(&mut e, 0x400, 0x0360_0000);
        let data = object(&mut e, 0x74, 0x0370_0000);
        e.mem.set_u32(data + 4, 1);
        let (widths, heights) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(widths, 256);
        e.mem.set_u32(heights, 128);
        e.mem.set_u32(data + 0x54, widths);
        e.mem.set_u32(data + 0x58, heights);
        for word in 0..0x11 {
            e.mem.set_u32(data + 8 + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(data + 0x60, 2);
        e.mem.set_u32(data + 0x6c, 6);
        e.register_double(0x0360_0000 + READ_FILE_SLOT, move |_, _| data.into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(0x0370_0004, |_, _| Ret::default());
        let out = e.mem.alloc(0x60);
        let (w, h, format, mipmap, faces) = (out, out + 4, out + 8, out + 0x4c, out + 0x50);
        e.mem.set_u32(mipmap, 0xaaaa_aa00);
        e.call_log = Some(vec![]);
        let read = |e: &mut Engine| {
            e.call(
                0x00a8_add0,
                &args![reader, 0x1234u32, w, h, format, mipmap, faces],
            )
            .bool()
        };
        assert!(read(&mut e));
        assert_eq!(
            calls(&e, 0x0360_0000 + READ_FILE_SLOT),
            vec![vec![reader, 0x1234, 0]]
        );
        assert_eq!((e.mem.u32(w), e.mem.u32(h)), (256, 128));
        let copied: Vec<u32> = (0..0x11).map(|i| e.mem.u32(format + 4 * i)).collect();
        assert_eq!(copied, (0x100..0x111).collect::<Vec<u32>>());
        // Only the flag's byte is written.
        assert_eq!(e.mem.u32(mipmap), 0xaaaa_aa01);
        assert_eq!(e.mem.u32(faces), 6);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(calls(&e, 0x0370_0004).is_empty());
        // One mipmap level gives false; dropping the last reference deletes the data.
        e.mem.set_u32(data + 0x60, 1);
        e.mem.set_u32(data + 4, 0);
        assert!(read(&mut e));
        assert_eq!(e.mem.u8(mipmap), 0);
        assert_eq!(calls(&e, 0x0370_0004), vec![vec![data]]);
        // Nothing read: false.
        e.register(0x0360_0000 + READ_FILE_SLOT, |_, _| Ret::default());
        assert!(!read(&mut e));
    }

    #[test]
    fn scalar_deleting_destructor_tears_down_and_frees_on_bit_0() {
        let mut e = Engine::new();
        for addr in [PART_DESTRUCTOR, DELETE_CRITICAL_SECTION, MEMORY_DEALLOCATE] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_ae80, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0109_f3d0);
        assert_eq!(calls(&e, PART_DESTRUCTOR), vec![vec![this + 0x100]]);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION), vec![vec![this + 0x80]]);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00a8_ae80, &args![this, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn read_file_answers_a_single_pixel_data_top_object() {
        let mut e = Engine::new();
        let reader = e.mem.alloc(0x400);
        let stream_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(stream_vtable + STREAM_LOAD_SLOT, 0x0380_0000);
        e.mem.set_u32(reader + STREAM, stream_vtable);
        let top = e.mem.alloc(8);
        e.mem.set_u32(reader + TOP_OBJECTS_BASE, top);
        let object = object(&mut e, 0x74, 0x0390_0000);
        e.mem.set_u32(top, object);
        // A class derived from NiPixelData: its record's base is NiPixelData's.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + RTTI_BASE, NI_PIXEL_DATA_RTTI);
        e.register_double(0x0390_0000 + GET_RTTI_SLOT, move |_, _| derived.into_ret());
        for addr in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            STREAM_REMOVE_ALL_OBJECTS,
            0x0380_0000,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let read = |e: &mut Engine| e.call(0x00a8_aec0, &args![reader, 0x77u32, 0u32]).u32();
        e.mem.set_u32(reader + TOP_OBJECTS_COUNT, 1);
        assert_eq!(read(&mut e), object);
        assert_eq!(calls(&e, 0x0380_0000), vec![vec![reader + STREAM, 0x77]]);
        assert_eq!(
            calls(&e, STREAM_REMOVE_ALL_OBJECTS),
            vec![vec![reader + STREAM]]
        );
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![reader + LOCK]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![reader + LOCK]]);
        // Another class, or two top objects: null (the lock still released).
        e.mem.set_u32(derived + RTTI_BASE, 0);
        assert_eq!(read(&mut e), 0);
        e.mem.set_u32(derived + RTTI_BASE, NI_PIXEL_DATA_RTTI);
        e.mem.set_u32(reader + TOP_OBJECTS_COUNT, 2);
        assert_eq!(read(&mut e), 0);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 3);
    }
}

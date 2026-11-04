//! `fallout/misc/saveload/bgssaveloadfile.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// Save/load manager pointer; each call reloads it as the PC does.
const MANAGER: u32 = 0x011de134;

layout! {
    /// `BGSSaveLoadFile` (Xbox PDB); PC allocation at 00846545 is 0x110.
    pub struct BGSSaveLoadFile: 0x110 {
        0x104 pFile: u32,
        0x108 iSimulatedFileSize: u32,
        0x10c iSimulatedFilePosition: u32,
    }
    /// `BGSSaveLoadFileEntry` (Xbox PDB); PC allocation at 00851708 is 0x7C.
    /// The constructor leaves the padding and trailing version/language untouched.
    pub struct BGSSaveLoadFileEntry: 0x7c {
        0x00 pFileName: u32,
        0x04 bLoaded: u8,
        0x05 bCorrupt: u8,
        0x08 iSaveGameNumber: u32,
        0x0c pPlayerName: u32,
        0x10 pPlayerTitle: u32,
        0x14 pLocation: u32,
        0x18 pPlayTime: u32,
        0x1c iPlayerLevel: u32,
        0x20 iScreenshotWidth: u32,
        0x24 iScreenshotHeight: u32,
        0x28 iScreenshotOffset: u32,
        0x2c FileTimeLow: u32,
        0x30 FileTimeHigh: u32,
        0x34 iDeviceID: u32,
    }
}

fn simulated(e: &mut Engine) -> bool {
    let manager = e.mem.u32(MANAGER);
    e.call(0x00424940, &args![manager]).bool()
}

// Translation incomplete: 008461e0 - C++ allocation exception-unwind frame omitted.
// Translated from 008461e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::BGSSaveLoadFile` (Xbox PDB).
pub fn file_construct(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadFile>,
    name: u32,
    writing: u8,
    size: u32,
) -> Ptr<BGSSaveLoadFile> {
    e.set(this, BGSSaveLoadFile::pFile, 0);
    e.set(this, BGSSaveLoadFile::iSimulatedFileSize, 0);
    e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, 0);
    e.mem.set_u8(this.addr(), 0);
    if !simulated(e) {
        let allocation = e.call(0x00401000, &args![0x158u32]).u32();
        let file = if allocation == 0 {
            0
        } else {
            e.call(
                0x00b00260,
                &args![allocation, name, u32::from(writing != 0), size, 0u32],
            )
            .u32()
        };
        e.set(this, BGSSaveLoadFile::pFile, file);
    }
    this
}

// Translated from 008462c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Invoke the file's deleting destructor when present.
pub fn fn_008462c0(e: &mut Engine, this: Ptr<BGSSaveLoadFile>) {
    let file = e.get(this, BGSSaveLoadFile::pFile);
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
}

// Translated from 00846300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::Read` (Xbox PDB); forwards even in simulated mode.
pub fn file_read(e: &mut Engine, this: Ptr<BGSSaveLoadFile>, buffer: u32, size: u32) -> u32 {
    let file = e.get(this, BGSSaveLoadFile::pFile);
    e.call(0x00462d80, &args![file, buffer, size]).u32()
}

// Translated from 00846330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::Write` (Xbox PDB); simulated arithmetic is unsigned and wrapping.
pub fn file_write(e: &mut Engine, this: Ptr<BGSSaveLoadFile>, buffer: u32, size: u32) -> u32 {
    if simulated(e) {
        let remaining = e
            .get(this, BGSSaveLoadFile::iSimulatedFileSize)
            .wrapping_sub(e.get(this, BGSSaveLoadFile::iSimulatedFilePosition));
        if remaining < size {
            let length = size
                .wrapping_sub(remaining)
                .wrapping_add(e.get(this, BGSSaveLoadFile::iSimulatedFileSize));
            e.set(this, BGSSaveLoadFile::iSimulatedFileSize, length);
        }
        let position = e
            .get(this, BGSSaveLoadFile::iSimulatedFilePosition)
            .wrapping_add(size);
        e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, position);
        size
    } else {
        let file = e.get(this, BGSSaveLoadFile::pFile);
        e.call(0x00473180, &args![file, buffer, size]).u32()
    }
}

// Translated from 008463c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Simulated files succeed; real files invoke slot +0x20 with two zero words.
pub fn fn_008463c0(e: &mut Engine, this: Ptr<BGSSaveLoadFile>) -> u8 {
    if simulated(e) {
        1
    } else {
        let file = e.get(this, BGSSaveLoadFile::pFile);
        e.vcall(file, 0x20, &args![0u32, 0u32]).u32() as u8
    }
}

// Translated from 00846400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::GetFilePosition` (Xbox PDB).
pub fn get_file_position(e: &mut Engine, this: Ptr<BGSSaveLoadFile>) -> u32 {
    if simulated(e) {
        e.get(this, BGSSaveLoadFile::iSimulatedFilePosition)
    } else {
        let file = e.get(this, BGSSaveLoadFile::pFile);
        e.call(0x004720a0, &args![file]).u32()
    }
}

// Translated from 00846440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::SeekSet` (Xbox PDB); real origin comes from 010A2480.
pub fn seek_set(e: &mut Engine, this: Ptr<BGSSaveLoadFile>, position: u32) {
    if simulated(e) {
        e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, position);
    } else {
        let origin = e.mem.u32(0x010a2480);
        let file = e.get(this, BGSSaveLoadFile::pFile);
        e.vcall(file, 0x14, &args![position, origin]);
    }
}

// Translated from 00846490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile::SeekCurrent` (Xbox PDB); adds the raw signed-offset bits.
pub fn seek_current(e: &mut Engine, this: Ptr<BGSSaveLoadFile>, offset: u32) {
    if simulated(e) {
        let position = e
            .get(this, BGSSaveLoadFile::iSimulatedFilePosition)
            .wrapping_add(offset);
        e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, position);
    } else {
        let origin = e.mem.u32(0x010a2484);
        let file = e.get(this, BGSSaveLoadFile::pFile);
        e.vcall(file, 0x14, &args![offset, origin]);
    }
}

// Translated from 008464f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy a name to the inline 260-byte filename buffer.
pub fn fn_008464f0(e: &mut Engine, this: Ptr<BGSSaveLoadFile>, name: u32) {
    e.call(0x00406d30, &args![this, 0x104u32, name]);
}

// Translation incomplete: 00846520 - C++ allocation exception-unwind frame omitted.
// Translated from 00846520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocate and construct a save/load file; this function is cdecl.
pub fn fn_00846520(e: &mut Engine, name: u32, writing: u8, size: u32) -> Ptr<BGSSaveLoadFile> {
    let allocation = e.call(0x00401000, &args![0x110u32]).u32();
    if allocation == 0 {
        Ptr::NULL
    } else {
        file_construct(e, Ptr::new(allocation), name, writing, size)
    }
}

// Translated from 008465b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct a `BGSSaveLoadFileEntry` (Xbox PDB) and own a copy of its filename.
pub fn fn_008465b0(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadFileEntry>,
    name: u32,
    device: u32,
) -> Ptr<BGSSaveLoadFileEntry> {
    e.set(this, BGSSaveLoadFileEntry::bLoaded, 0);
    e.set(this, BGSSaveLoadFileEntry::bCorrupt, 0);
    for offset in (8..=0x28).step_by(4) {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    e.set(this, BGSSaveLoadFileEntry::iDeviceID, device);
    e.call(
        0x00403d30,
        &args![this.addr().wrapping_add(0x2c), 0u32, 8u32],
    );
    let length = e.call(0x00ec6130, &args![name]).u32();
    let buffer = e.call(0x00401000, &args![length.wrapping_add(1)]).u32();
    e.set(this, BGSSaveLoadFileEntry::pFileName, buffer);
    e.call(0x00406d30, &args![buffer, length.wrapping_add(1), name]);
    this
}

// Translated from 00846690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Release the five owned strings, reading each field after the previous release.
pub fn fn_00846690(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) {
    for offset in [0, 0x18, 0x14, 0x0c, 0x10] {
        let buffer = e.mem.u32(this.addr().wrapping_add(offset));
        e.call(0x00401030, &args![buffer]);
    }
}

// Translated from 00846710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry::SaveGame` (Xbox PDB).
pub fn entry_save_game(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> bool {
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    let mut generated = e.call(0x00851980, &args![manager, name]).bool();
    if !generated {
        let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
        let manager = e.mem.u32(MANAGER);
        generated = e.call(0x00851a20, &args![manager, name]).bool();
    }
    let device = e.get(this, BGSSaveLoadFileEntry::iDeviceID);
    let name = if generated {
        0
    } else {
        e.get(this, BGSSaveLoadFileEntry::pFileName)
    };
    let manager = e.mem.u32(MANAGER);
    if !e
        .call(0x008503b0, &args![manager, name, device, 0u32])
        .bool()
    {
        return false;
    }
    if generated {
        e.call(0x008467e0, &args![this]);
    } else {
        let manager = e.mem.u32(MANAGER);
        e.call(0x00851620, &args![manager, this]);
    }
    true
}

fn mark_bad_header(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) {
    if e.get(this, BGSSaveLoadFileEntry::iScreenshotWidth) == 0
        && e.get(this, BGSSaveLoadFileEntry::iScreenshotHeight) == 0
    {
        e.set(this, BGSSaveLoadFileEntry::bCorrupt, 1);
    }
    if e.get(this, BGSSaveLoadFileEntry::pLocation) == 0 {
        e.set(this, BGSSaveLoadFileEntry::bCorrupt, 1);
    }
}

// Translated from 00846820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry::GetScreenshot` (Xbox PDB).
pub fn get_screenshot(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> u32 {
    let device = e.get(this, BGSSaveLoadFileEntry::iDeviceID);
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    let file = e
        .call(0x00850030, &args![manager, name, 0u32, 0u32, device])
        .u32();
    if file == 0 {
        return 0;
    }
    if e.get(this, BGSSaveLoadFileEntry::bLoaded) == 0 {
        e.call(0x0084dab0, &args![file, this]);
        mark_bad_header(e, this);
        e.set(this, BGSSaveLoadFileEntry::bLoaded, 1);
    }
    let mut result = 0;
    if e.get(this, BGSSaveLoadFileEntry::iScreenshotWidth) != 0
        && e.get(this, BGSSaveLoadFileEntry::iScreenshotHeight) != 0
    {
        let height = e.get(this, BGSSaveLoadFileEntry::iScreenshotHeight);
        let width = e.get(this, BGSSaveLoadFileEntry::iScreenshotWidth);
        let offset = e.get(this, BGSSaveLoadFileEntry::iScreenshotOffset);
        result = e
            .call(0x0084dda0, &args![file, offset, width, height])
            .u32();
    }
    let manager = e.mem.u32(MANAGER);
    e.call(0x00850100, &args![manager, file, 0u32]);
    result
}

// Translated from 00846900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry::LoadData` (Xbox PDB).
pub fn load_data(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) {
    if e.get(this, BGSSaveLoadFileEntry::bLoaded) != 0
        || e.get(this, BGSSaveLoadFileEntry::bCorrupt) != 0
    {
        return;
    }
    let device = e.get(this, BGSSaveLoadFileEntry::iDeviceID);
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    let file = e
        .call(0x00850030, &args![manager, name, 0u32, 1u32, device])
        .u32();
    if file == 0 {
        e.set(this, BGSSaveLoadFileEntry::bCorrupt, 1);
        return;
    }
    e.call(0x0084dab0, &args![file, this]);
    mark_bad_header(e, this);
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    if e.call(0x00851aa0, &args![manager, name]).bool() {
        let old = e.get(this, BGSSaveLoadFileEntry::pLocation);
        e.call(0x00401030, &args![old]);
        let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
        let size = e.call(0x0044a670, &args![name]).u32().wrapping_add(1);
        let replacement = e.call(0x00401000, &args![size]).u32();
        e.set(this, BGSSaveLoadFileEntry::pLocation, replacement);
        let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
        let location = e.get(this, BGSSaveLoadFileEntry::pLocation);
        e.call(0x00406d30, &args![location, size, name]);
    }
    let manager = e.mem.u32(MANAGER);
    e.call(0x00850100, &args![manager, file, 0u32]);
    e.set(this, BGSSaveLoadFileEntry::bLoaded, 1);
}

// Translation incomplete: 00846a20 - compiler security-cookie check 00ec408c omitted.
// Translated from 00846a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Populate a missing file timestamp; FindClose is called even for an invalid handle.
pub fn fn_00846a20(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) {
    if e.get(this, BGSSaveLoadFileEntry::FileTimeHigh) != 0
        || e.get(this, BGSSaveLoadFileEntry::FileTimeLow) != 0
    {
        return;
    }
    // PC stack: 264-byte path followed by WIN32_FIND_DATAA (320 bytes).
    e.with_stack(584, |e, temporary| {
        let path = temporary.addr();
        let data = path + 264;
        let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
        let manager = e.mem.u32(MANAGER);
        e.call(0x0084ff90, &args![manager, name, path, 0u32]);
        let handle = e.call(0x00fdf070, &args![path, data]).u32();
        if handle != u32::MAX {
            let low = e.mem.u32(data + 0x14);
            e.set(this, BGSSaveLoadFileEntry::FileTimeLow, low);
            let high = e.mem.u32(data + 0x18);
            e.set(this, BGSSaveLoadFileEntry::FileTimeHigh, high);
        }
        e.call(0x00fdf064, &args![handle]);
    });
}

// Translated from 00846ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward this entry's filename to the manager's 00851A60 predicate.
pub fn fn_00846ad0(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> u8 {
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    e.call(0x00851a60, &args![manager, name]).u32() as u8
}

// Translated from 00846af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry::IsAutosave` (Xbox PDB).
pub fn is_autosave(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> u8 {
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    e.call(0x00851a20, &args![manager, name]).u32() as u8
}

// Translated from 00846b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward this entry's filename to the manager's 00851B00 predicate.
pub fn fn_00846b10(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> u8 {
    let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
    let manager = e.mem.u32(MANAGER);
    e.call(0x00851b00, &args![manager, name]).u32() as u8
}

// Translated from 00846b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry::IsCorrupt` (Xbox PDB); existing nonzero means 1, newly loaded byte is returned raw.
pub fn is_corrupt(e: &mut Engine, this: Ptr<BGSSaveLoadFileEntry>) -> u8 {
    if e.get(this, BGSSaveLoadFileEntry::bCorrupt) != 0 {
        1
    } else {
        load_data(e, this);
        e.get(this, BGSSaveLoadFileEntry::bCorrupt)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008461e0,
            file_construct(Ptr<BGSSaveLoadFile>, u32, u8, u32) -> Ptr<BGSSaveLoadFile>
        ),
        entry!(0x008462c0, fn_008462c0(Ptr<BGSSaveLoadFile>) -> ()),
        entry!(0x00846300, file_read(Ptr<BGSSaveLoadFile>, u32, u32) -> u32),
        entry!(
            0x00846330,
            file_write(Ptr<BGSSaveLoadFile>, u32, u32) -> u32
        ),
        entry!(0x008463c0, fn_008463c0(Ptr<BGSSaveLoadFile>) -> u8),
        entry!(0x00846400, get_file_position(Ptr<BGSSaveLoadFile>) -> u32),
        entry!(0x00846440, seek_set(Ptr<BGSSaveLoadFile>, u32) -> ()),
        entry!(0x00846490, seek_current(Ptr<BGSSaveLoadFile>, u32) -> ()),
        entry!(0x008464f0, fn_008464f0(Ptr<BGSSaveLoadFile>, u32) -> ()),
        entry!(0x00846520, fn_00846520(u32, u8, u32) -> Ptr<BGSSaveLoadFile>),
        entry!(
            0x008465b0,
            fn_008465b0(Ptr<BGSSaveLoadFileEntry>, u32, u32) -> Ptr<BGSSaveLoadFileEntry>
        ),
        entry!(0x00846690, fn_00846690(Ptr<BGSSaveLoadFileEntry>) -> ()),
        entry!(
            0x00846710,
            entry_save_game(Ptr<BGSSaveLoadFileEntry>) -> bool
        ),
        entry!(0x00846820, get_screenshot(Ptr<BGSSaveLoadFileEntry>) -> u32),
        entry!(0x00846900, load_data(Ptr<BGSSaveLoadFileEntry>) -> ()),
        entry!(0x00846a20, fn_00846a20(Ptr<BGSSaveLoadFileEntry>) -> ()),
        entry!(0x00846ad0, fn_00846ad0(Ptr<BGSSaveLoadFileEntry>) -> u8),
        entry!(0x00846af0, is_autosave(Ptr<BGSSaveLoadFileEntry>) -> u8),
        entry!(0x00846b10, fn_00846b10(Ptr<BGSSaveLoadFileEntry>) -> u8),
        entry!(0x00846b30, is_corrupt(Ptr<BGSSaveLoadFileEntry>) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Engine {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x6000, 0x2000);
        e.map(0x010a2000, 0x1000);
        e.map(MANAGER & !0xfff, 0x1000);
        e.set_global(MANAGER, 0x6000u32);
        e.set_global(0x1000, 0u32);
        e.call_log = Some(vec![]);
        e.register(0x00424940, |e, a| {
            assert_eq!(a, &[0x6000]);
            e.mem.u32(0x1000).into_ret()
        });
        e.register(0x00401000, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00401030, |_, _| ().into_ret());
        e.register(0x00403d30, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(0x00406d30, |_, _| 0u32.into_ret());
        e.register(0x00850030, |_, _| 0x7000u32.into_ret());
        e.register(0x0084dab0, |_, _| ().into_ret());
        e.register(0x00850100, |_, _| ().into_ret());
        e.register(0x00851aa0, |_, _| 0u32.into_ret());
        e.register(0x00851980, |_, _| 0u32.into_ret());
        e.register(0x00851a20, |_, _| 0u32.into_ret());
        e.register(0x008503b0, |_, _| 1u32.into_ret());
        e.register(0x00851620, |_, _| ().into_ret());
        e.register(0x008467e0, |_, _| ().into_ret());
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, a)| a.clone())
            .collect()
    }

    fn entry(e: &mut Engine) -> Ptr<BGSSaveLoadFileEntry> {
        let this = e.new_object::<BGSSaveLoadFileEntry>();
        e.set(this, BGSSaveLoadFileEntry::pFileName, 0x8000);
        e.set(this, BGSSaveLoadFileEntry::pLocation, 0x9000);
        e.set(this, BGSSaveLoadFileEntry::iScreenshotWidth, 640);
        e.set(this, BGSSaveLoadFileEntry::iScreenshotHeight, 480);
        e.set(this, BGSSaveLoadFileEntry::iScreenshotOffset, 123);
        e.set(this, BGSSaveLoadFileEntry::iDeviceID, u32::MAX);
        this
    }

    #[test]
    fn file_constructor_covers_mode_and_allocation_branches() {
        for (mode, writing, allocation_succeeds) in
            [(1, 0, true), (0, 0, true), (0, 0x80, true), (0, 1, false)]
        {
            let mut e = setup();
            let this = e.new_object::<BGSSaveLoadFile>();
            e.mem.write(this.addr(), &[0xa5; 0x110]);
            e.set_global(0x1000, mode as u32);
            e.register_double(0x00401000, move |e, a| {
                assert_eq!(a, &[0x158]);
                if allocation_succeeds {
                    e.mem.alloc(0x158).into_ret()
                } else {
                    0u32.into_ret()
                }
            });
            e.register_double(0x00b00260, move |_, a| {
                assert_eq!(&a[1..], &[0x8000, u32::from(writing != 0), 4096, 0]);
                0x76543210u32.into_ret()
            });
            assert_eq!(file_construct(&mut e, this, 0x8000, writing, 4096), this);
            assert_eq!(e.mem.u8(this.addr()), 0);
            assert_eq!(e.mem.u8(this.addr() + 1), 0xa5);
            assert_eq!(
                e.get(this, BGSSaveLoadFile::pFile),
                if mode == 0 && allocation_succeeds {
                    0x76543210
                } else {
                    0
                }
            );
            assert_eq!(e.get(this, BGSSaveLoadFile::iSimulatedFileSize), 0);
            assert_eq!(e.get(this, BGSSaveLoadFile::iSimulatedFilePosition), 0);
            assert_eq!(calls(&e, 0x00401000).len(), usize::from(mode == 0));
        }
    }

    #[test]
    fn file_destructor_calls_optional_virtual_delete() {
        for present in [false, true] {
            let mut e = setup();
            let this = e.new_object::<BGSSaveLoadFile>();
            e.put_vtable(0x5000, &[0x123456]);
            e.set_global(0x7000, 0x5000u32);
            if present {
                e.set(this, BGSSaveLoadFile::pFile, 0x7000);
            }
            e.register(0x123456, |_, a| {
                assert_eq!(a, &[0x7000, 1]);
                0u32.into_ret()
            });
            fn_008462c0(&mut e, this);
            assert_eq!(calls(&e, 0x123456).len(), usize::from(present));
            assert_eq!(
                e.get(this, BGSSaveLoadFile::pFile),
                if present { 0x7000 } else { 0 }
            );
        }
    }

    #[test]
    fn read_forwards_words_and_full_result_without_mode_check() {
        let mut e = setup();
        let this = e.new_object::<BGSSaveLoadFile>();
        e.set(this, BGSSaveLoadFile::pFile, 0x7000);
        e.register(0x00462d80, |_, a| {
            assert_eq!(a, &[0x7000, 0x8000, u32::MAX]);
            0xfedcba98u32.into_ret()
        });
        assert_eq!(file_read(&mut e, this, 0x8000, u32::MAX), 0xfedcba98);
        assert!(calls(&e, 0x00424940).is_empty());
    }

    #[test]
    fn simulated_write_matches_native_overflow_and_guard_vectors() {
        for mode in [1, 0x80, 0xff] {
            for (size, cursor, count, expected_size, expected_cursor) in [
                (10, 2, 0, 10, 2),
                (10, 2, 8, 10, 10),
                (10, 2, 9, 11, 11),
                (0, 0, 5, 5, 5),
                (10, 12, 1, 10, 13),
                (0, u32::MAX, 2, 1, 1),
                (0xfffffff0, 0xfffffff0, 32, 16, 16),
                (u32::MAX, 0, u32::MAX, u32::MAX, u32::MAX),
                (0, 1, u32::MAX, 0, 0),
            ] {
                let mut e = setup();
                e.set_global(0x1000, mode as u32);
                let allocation = e.mem.alloc(320);
                e.mem.write(allocation, &[0xa5; 320]);
                let this = Ptr::<BGSSaveLoadFile>::new(allocation + 16);
                e.set(this, BGSSaveLoadFile::iSimulatedFileSize, size);
                e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, cursor);
                let mut expected = [0; 320];
                e.mem.read(allocation, &mut expected);
                expected[0x118..0x11c].copy_from_slice(&expected_size.to_le_bytes());
                expected[0x11c..0x120].copy_from_slice(&expected_cursor.to_le_bytes());
                assert_eq!(file_write(&mut e, this, 0xdeadbeef, count), count);
                let mut actual = [0; 320];
                e.mem.read(allocation, &mut actual);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn real_write_forwards_after_raw_al_mode_test() {
        let mut e = setup();
        let this = e.new_object::<BGSSaveLoadFile>();
        e.set_global(0x1000, 0x100u32);
        e.set(this, BGSSaveLoadFile::pFile, 0x7000);
        e.register(0x00473180, |_, a| {
            assert_eq!(a, &[0x7000, 0x8000, 20]);
            7u32.into_ret()
        });
        assert_eq!(file_write(&mut e, this, 0x8000, 20), 7);
        assert_eq!(e.get(this, BGSSaveLoadFile::iSimulatedFilePosition), 0);
    }

    #[test]
    fn file_status_forwards_two_zeros_and_preserves_al() {
        for mode in [0, 0x80] {
            let mut e = setup();
            let this = e.new_object::<BGSSaveLoadFile>();
            e.set_global(0x1000, mode as u32);
            e.set(this, BGSSaveLoadFile::pFile, 0x7000);
            e.put_vtable(0x5000, &[0, 0, 0, 0, 0, 0, 0, 0, 0x123456]);
            e.set_global(0x7000, 0x5000u32);
            e.register(0x123456, |_, a| {
                assert_eq!(a, &[0x7000, 0, 0]);
                0xab80u32.into_ret()
            });
            assert_eq!(fn_008463c0(&mut e, this), if mode == 0 { 0x80 } else { 1 });
            assert_eq!(calls(&e, 0x123456).len(), usize::from(mode == 0));
        }
    }

    #[test]
    fn get_position_switches_between_cursor_and_file() {
        for mode in [0, 1] {
            let mut e = setup();
            let this = e.new_object::<BGSSaveLoadFile>();
            e.set_global(0x1000, mode as u32);
            e.set(this, BGSSaveLoadFile::pFile, 0x7000);
            e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, u32::MAX);
            e.register(0x004720a0, |_, a| {
                assert_eq!(a, &[0x7000]);
                123u32.into_ret()
            });
            assert_eq!(
                get_file_position(&mut e, this),
                if mode == 0 { 123 } else { u32::MAX }
            );
        }
    }

    fn seek_fixture(e: &mut Engine) -> Ptr<BGSSaveLoadFile> {
        let this = e.new_object::<BGSSaveLoadFile>();
        e.set(this, BGSSaveLoadFile::pFile, 0x7000);
        e.set(this, BGSSaveLoadFile::iSimulatedFilePosition, 2);
        e.put_vtable(0x5000, &[0, 0, 0, 0, 0, 0x123456]);
        e.set_global(0x7000, 0x5000u32);
        e.register(0x123456, |_, _| ().into_ret());
        this
    }

    #[test]
    fn seek_set_uses_runtime_origin_word() {
        for mode in [0, 1] {
            let mut e = setup();
            let this = seek_fixture(&mut e);
            e.set_global(0x1000, mode as u32);
            e.set_global(0x010a2480, 0x1234u32);
            seek_set(&mut e, this, u32::MAX);
            assert_eq!(
                e.get(this, BGSSaveLoadFile::iSimulatedFilePosition),
                if mode == 0 { 2 } else { u32::MAX }
            );
            if mode == 0 {
                assert_eq!(calls(&e, 0x123456), vec![vec![0x7000, u32::MAX, 0x1234]]);
            }
        }
    }

    #[test]
    fn seek_current_wraps_and_uses_runtime_origin_word() {
        for mode in [0, 1] {
            let mut e = setup();
            let this = seek_fixture(&mut e);
            e.set_global(0x1000, mode as u32);
            e.set_global(0x010a2484, 0x5678u32);
            seek_current(&mut e, this, u32::MAX);
            assert_eq!(
                e.get(this, BGSSaveLoadFile::iSimulatedFilePosition),
                if mode == 0 { 2 } else { 1 }
            );
            if mode == 0 {
                assert_eq!(calls(&e, 0x123456), vec![vec![0x7000, u32::MAX, 0x5678]]);
            }
        }
    }

    #[test]
    fn filename_copy_uses_inline_capacity() {
        let mut e = setup();
        let this = e.new_object::<BGSSaveLoadFile>();
        fn_008464f0(&mut e, this, 0x8000);
        assert_eq!(
            calls(&e, 0x00406d30),
            vec![vec![this.addr(), 0x104, 0x8000]]
        );
    }

    #[test]
    fn file_factory_covers_null_and_success_without_receiver() {
        for succeeds in [false, true] {
            let mut e = setup();
            e.set_global(0x1000, 1u32);
            e.register_double(0x00401000, move |e, a| {
                assert_eq!(a, &[0x110]);
                (if succeeds { e.mem.alloc(a[0]) } else { 0 }).into_ret()
            });
            let result = e
                .call(0x00846520, &args![0x8000u32, 0x180u32, 4096u32])
                .ptr::<BGSSaveLoadFile>();
            assert_eq!(!result.is_null(), succeeds);
            assert_eq!(calls(&e, 0x00424940).len(), usize::from(succeeds));
        }
    }

    #[test]
    fn entry_constructor_preserves_padding_and_trailing_fields() {
        let mut e = setup();
        let this = e.new_object::<BGSSaveLoadFileEntry>();
        e.mem.write(this.addr(), &[0xa5; 0x7c]);
        e.register(0x00ec6130, |_, a| {
            assert_eq!(a, &[0x8000]);
            7u32.into_ret()
        });
        assert_eq!(fn_008465b0(&mut e, this, 0x8000, u32::MAX), this);
        let name = e.get(this, BGSSaveLoadFileEntry::pFileName);
        assert_eq!(calls(&e, 0x00401000), vec![vec![8]]);
        assert_eq!(calls(&e, 0x00406d30), vec![vec![name, 8, 0x8000]]);
        let mut bytes = [0; 0x7c];
        e.mem.read(this.addr(), &mut bytes);
        assert_eq!(&bytes[4..6], &[0, 0]);
        assert_eq!(&bytes[6..8], &[0xa5; 2]);
        assert!(bytes[8..0x34].iter().all(|b| *b == 0));
        assert_eq!(&bytes[0x34..0x38], &[0xff; 4]);
        assert!(bytes[0x38..].iter().all(|b| *b == 0xa5));
    }

    #[test]
    fn entry_destructor_reloads_each_owned_string_after_release() {
        let mut e = setup();
        let this = entry(&mut e);
        e.register_double(0x00401030, move |e, a| {
            if a[0] == 0x8000 {
                e.set(this, BGSSaveLoadFileEntry::pPlayTime, 11);
            }
            if a[0] == 11 {
                e.set(this, BGSSaveLoadFileEntry::pLocation, 12);
            }
            if a[0] == 12 {
                e.set(this, BGSSaveLoadFileEntry::pPlayerName, 13);
            }
            if a[0] == 13 {
                e.set(this, BGSSaveLoadFileEntry::pPlayerTitle, 14);
            }
            ().into_ret()
        });
        fn_00846690(&mut e, this);
        assert_eq!(
            calls(&e, 0x00401030),
            vec![vec![0x8000], vec![11], vec![12], vec![13], vec![14]]
        );
    }

    #[test]
    fn save_entry_selects_generated_name_and_success_followup() {
        for (generated, auto, success) in [
            (false, false, false),
            (false, false, true),
            (true, false, false),
            (true, false, true),
            (false, true, true),
        ] {
            let mut e = setup();
            let this = entry(&mut e);
            e.register_double(0x00851980, move |_, _| {
                (if generated { 0x180u32 } else { 0x100u32 }).into_ret()
            });
            e.register_double(0x00851a20, move |_, _| auto.into_ret());
            e.register_double(0x008503b0, move |_, a| {
                assert_eq!(
                    a,
                    &[
                        0x6000,
                        if generated || auto { 0 } else { 0x8000 },
                        u32::MAX,
                        0
                    ]
                );
                success.into_ret()
            });
            assert_eq!(entry_save_game(&mut e, this), success);
            assert_eq!(calls(&e, 0x00851a20).len(), usize::from(!generated));
            assert_eq!(
                calls(&e, 0x008467e0).len(),
                usize::from(success && (generated || auto))
            );
            assert_eq!(
                calls(&e, 0x00851620).len(),
                usize::from(success && !generated && !auto)
            );
        }
    }

    #[test]
    fn screenshot_header_and_dimension_branches_close_open_file() {
        for (opens, loaded, width, height, location, corrupt, loads_image) in [
            (false, false, 640, 480, 1, false, false),
            (true, false, 640, 480, 1, false, true),
            (true, false, 0, 0, 1, true, false),
            (true, false, 0, 480, 1, false, false),
            (true, false, 640, 0, 1, false, false),
            (true, false, 640, 480, 0, true, true),
            (true, true, 640, 480, 1, false, true),
        ] {
            let mut e = setup();
            let this = entry(&mut e);
            e.set(this, BGSSaveLoadFileEntry::bLoaded, u8::from(loaded));
            e.set(this, BGSSaveLoadFileEntry::iScreenshotWidth, width);
            e.set(this, BGSSaveLoadFileEntry::iScreenshotHeight, height);
            e.set(this, BGSSaveLoadFileEntry::pLocation, location);
            e.register_double(0x00850030, move |_, a| {
                assert_eq!(a, &[0x6000, 0x8000, 0, 0, u32::MAX]);
                (if opens { 0x7000u32 } else { 0u32 }).into_ret()
            });
            e.register_double(0x0084dda0, move |_, a| {
                assert_eq!(a, &[0x7000, 123, width, height]);
                0xabcdu32.into_ret()
            });
            assert_eq!(
                get_screenshot(&mut e, this),
                if loads_image { 0xabcd } else { 0 }
            );
            assert_eq!(
                e.get(this, BGSSaveLoadFileEntry::bCorrupt),
                u8::from(corrupt)
            );
            assert_eq!(calls(&e, 0x0084dab0).len(), usize::from(opens && !loaded));
            assert_eq!(calls(&e, 0x00850100).len(), usize::from(opens));
        }
    }

    #[test]
    fn load_data_handles_shortcuts_and_open_failure() {
        for (loaded, corrupt, opens) in [(1, 0, true), (0, 0x80, true), (0, 0, false), (0, 0, true)]
        {
            let mut e = setup();
            let this = entry(&mut e);
            e.set(this, BGSSaveLoadFileEntry::bLoaded, loaded);
            e.set(this, BGSSaveLoadFileEntry::bCorrupt, corrupt);
            e.register_double(0x00850030, move |_, a| {
                assert_eq!(a, &[0x6000, 0x8000, 0, 1, u32::MAX]);
                (if opens { 0x7000u32 } else { 0u32 }).into_ret()
            });
            load_data(&mut e, this);
            let reaches_open = loaded == 0 && corrupt == 0;
            assert_eq!(calls(&e, 0x00850030).len(), usize::from(reaches_open));
            assert_eq!(
                calls(&e, 0x00850100).len(),
                usize::from(reaches_open && opens)
            );
            assert_eq!(
                e.get(this, BGSSaveLoadFileEntry::bLoaded),
                if reaches_open && opens { 1 } else { loaded }
            );
            assert_eq!(
                e.get(this, BGSSaveLoadFileEntry::bCorrupt),
                if reaches_open && !opens { 1 } else { corrupt }
            );
        }
    }

    #[test]
    fn load_data_replaces_location_and_reloads_mutated_name_and_manager() {
        let mut e = setup();
        let this = entry(&mut e);
        e.register_double(0x0084dab0, move |e, a| {
            assert_eq!(a, &[0x7000, this.addr()]);
            e.set(this, BGSSaveLoadFileEntry::iScreenshotWidth, 0);
            e.set(this, BGSSaveLoadFileEntry::iScreenshotHeight, 0);
            e.set(this, BGSSaveLoadFileEntry::pLocation, 0);
            e.set_global(MANAGER, 0x6100u32);
            ().into_ret()
        });
        e.register(0x00851aa0, |_, a| {
            assert_eq!(a, &[0x6100, 0x8000]);
            0x180u32.into_ret()
        });
        e.register_double(0x00401030, move |e, a| {
            assert_eq!(a, &[0]);
            e.set(this, BGSSaveLoadFileEntry::pFileName, 0x8100);
            ().into_ret()
        });
        e.register(0x0044a670, |_, a| {
            assert_eq!(a, &[0x8100]);
            5u32.into_ret()
        });
        e.register_double(0x00401000, move |e, a| {
            assert_eq!(a, &[6]);
            e.set(this, BGSSaveLoadFileEntry::pFileName, 0x8200);
            e.mem.alloc(6).into_ret()
        });
        load_data(&mut e, this);
        let location = e.get(this, BGSSaveLoadFileEntry::pLocation);
        assert_eq!(calls(&e, 0x00406d30), vec![vec![location, 6, 0x8200]]);
        assert_eq!(calls(&e, 0x00850100), vec![vec![0x6100, 0x7000, 0]]);
        assert_eq!(e.get(this, BGSSaveLoadFileEntry::bCorrupt), 1);
        assert_eq!(e.get(this, BGSSaveLoadFileEntry::bLoaded), 1);
    }

    #[test]
    fn timestamp_checks_both_words_and_closes_invalid_handle() {
        for (low, high, found) in [(1, 0, true), (0, 1, true), (0, 0, false), (0, 0, true)] {
            let mut e = setup();
            let this = entry(&mut e);
            e.set(this, BGSSaveLoadFileEntry::FileTimeLow, low);
            e.set(this, BGSSaveLoadFileEntry::FileTimeHigh, high);
            e.register(0x0084ff90, |e, a| {
                assert_eq!((a[0], a[1], a[3]), (0x6000, 0x8000, 0));
                e.mem.set_u8(a[2], b'X');
                ().into_ret()
            });
            e.register_double(0x00fdf070, move |e, a| {
                assert_eq!(a[1] - a[0], 264);
                assert_eq!(e.mem.u8(a[0]), b'X');
                e.mem.set_u32(a[1] + 0x14, 0x87654321);
                e.mem.set_u32(a[1] + 0x18, 0x12345678);
                (if found { 0x42u32 } else { u32::MAX }).into_ret()
            });
            e.register(0x00fdf064, |_, _| 1u32.into_ret());
            fn_00846a20(&mut e, this);
            let queries = low == 0 && high == 0;
            assert_eq!(calls(&e, 0x00fdf070).len(), usize::from(queries));
            if queries {
                assert_eq!(
                    calls(&e, 0x00fdf064),
                    vec![vec![if found { 0x42 } else { u32::MAX }]]
                );
            }
            assert_eq!(
                e.get(this, BGSSaveLoadFileEntry::FileTimeLow),
                if queries && found { 0x87654321 } else { low }
            );
            assert_eq!(
                e.get(this, BGSSaveLoadFileEntry::FileTimeHigh),
                if queries && found { 0x12345678 } else { high }
            );
        }
    }

    #[test]
    fn first_filename_predicate_forwards_raw_al() {
        let mut e = setup();
        let this = entry(&mut e);
        e.register(0x00851a60, |_, a| {
            assert_eq!(a, &[0x6000, 0x8000]);
            0x12ffu32.into_ret()
        });
        assert_eq!(fn_00846ad0(&mut e, this), 0xff);
    }

    #[test]
    fn autosave_predicate_forwards_raw_al() {
        let mut e = setup();
        let this = entry(&mut e);
        e.register(0x00851a20, |_, a| {
            assert_eq!(a, &[0x6000, 0x8000]);
            0x1280u32.into_ret()
        });
        assert_eq!(is_autosave(&mut e, this), 0x80);
    }

    #[test]
    fn last_filename_predicate_forwards_raw_al() {
        let mut e = setup();
        let this = entry(&mut e);
        e.register(0x00851b00, |_, a| {
            assert_eq!(a, &[0x6000, 0x8000]);
            0x1200u32.into_ret()
        });
        assert_eq!(fn_00846b10(&mut e, this), 0);
    }

    #[test]
    fn corruption_query_normalizes_cached_flag_but_returns_new_byte_raw() {
        for cached in [0, 0x80] {
            let mut e = setup();
            let this = entry(&mut e);
            e.set(this, BGSSaveLoadFileEntry::bCorrupt, cached);
            e.register_double(0x0084dab0, move |e, _| {
                e.set(this, BGSSaveLoadFileEntry::bCorrupt, 0x80);
                ().into_ret()
            });
            assert_eq!(is_corrupt(&mut e, this), if cached == 0 { 0x80 } else { 1 });
            assert_eq!(calls(&e, 0x00850030).len(), usize::from(cached == 0));
        }
    }
}

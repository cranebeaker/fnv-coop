//! Platform functions: the game's own versions are replaced by Rust
//! (docs/ENGINE_PORT_PLAN.md, "platform"). Hand-written, owned by the lead.

#[allow(unused_imports)]
use crate::prelude::*;

pub mod bink;
pub mod d3d9;
pub mod d3d9device;
pub mod d3d9query;
#[cfg(windows)]
pub mod d3d9real;
pub mod d3d9resource;
pub mod d3dx;
#[cfg(windows)]
pub mod d3dxreal;
pub mod dinput;
pub mod ds3d;
pub mod dsfx;
pub mod dshow;
pub mod dsound;
pub mod files;
pub mod loader;
pub mod mixer;
pub mod mmio;
pub mod nls;
pub mod nvapi;
pub mod process;
pub mod resampler;
pub mod sound_out;
pub mod time;
pub mod vorbisfile;
pub mod window;
pub mod zone;

/// `NV_FACE_CHECK` (diagnostic only, not exe behaviour): the sizes of the FaceGen face images
/// (`0064ceb0`) not yet matched, each `width << 32 | height`. The real-device side (`d3d9real`)
/// follows the first uncompressed texture of such a size created after it (and drops the entry).
pub static FACE_CHECK_IMAGES: std::sync::Mutex<Vec<u64>> = std::sync::Mutex::new(Vec::new());
/// `NV_FACE_CHECK` (diagnostic only): the file name of each Direct3D texture an `NiSourceTexture`
/// got (by the modelled texture's address), filled where the renderer data is made (`00e68ef0`).
pub static FACE_CHECK_NAMES: std::sync::Mutex<std::collections::BTreeMap<u32, String>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

/// The `MemoryManager` singleton (`011f6238`); [`get_memory_manager`]
/// returns its address.
pub const MEMORY_MANAGER: u32 = 0x011f_6238;

// Translated from 00401020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the `MemoryManager` singleton.
pub fn get_memory_manager(_e: &mut Engine) -> Ptr {
    Ptr::new(MEMORY_MANAGER)
}

// Translated from 00401030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `operator delete`: frees through the `MemoryManager`.
pub fn operator_delete(e: &mut Engine, block: Ptr) {
    let mm: Ptr = get_memory_manager(e);
    e.call(0x00aa_4060, &args![mm, block]);
}

// Platform: replaces 00aa3e40 (MemoryManager::Allocate (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Allocates from [`crate::Mem`]'s heap instead of the game's pools.
/// Blocks are 8-byte aligned and zero-filled. With no pools, every block is one the game's
/// manager would hand out from the CRT heap (its `00aa4290` is `_malloc`), so it is a CRT
/// block for `_msize` and `realloc` (`MemoryManager`'s reallocation `00aa4150` sends a block
/// no pool knows to `realloc`).
pub fn memory_manager_allocate(e: &mut Engine, _this: Ptr, size: u32) -> Ptr {
    if e.mem.block_check_on() {
        e.mem.note_path(&e.call_stack);
    }
    let block = e.mem.alloc(size);
    e.kernel.crt_block_sizes.insert(block, size.max(1));
    Ptr::new(block)
}

// Platform: replaces 00aa4060 (MemoryManager::Deallocate (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Frees a block from [`memory_manager_allocate`]; null does nothing.
pub fn memory_manager_deallocate(e: &mut Engine, _this: Ptr, block: Ptr) {
    if let (Some(hook), true) = (e.free_hook, block.addr() != 0) {
        hook(e, block.addr());
    }
    if e.mem.block_check_on() {
        e.mem.note_path(&e.call_stack);
    }
    e.kernel.crt_block_sizes.remove(&block.addr());
    e.mem.free(block.addr());
}

/// `BSSystemUtility` singleton (`bssystemutility_xenon.cpp`, Xbox PDB): its
/// storage in `.data` (`011f71a8`, 0x96 bytes) and the function-local static's
/// guard word (`011f7240`, bit 0 = constructed).
pub const SYSTEM_UTILITY: u32 = 0x011f_71a8;
const SYSTEM_UTILITY_GUARD: u32 = 0x011f_7240;
/// Vtable stored by the constructor (`010a42b4`, first slot the scalar
/// deleting destructor) and the `atexit` destructor stub (`00fdc2f0`).
const SYSTEM_UTILITY_VTABLE: u32 = 0x010a_42b4;
const SYSTEM_UTILITY_ATEXIT: u32 = 0x00fd_c2f0;
/// The C runtime's `_memset` and `_atexit`.
const MEMSET: u32 = 0x00ec_61c0;
const ATEXIT: u32 = 0x00ec_658f;

// Translated from 00af30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the object `BSSystemUtility::QInstance` builds
/// (`fastcall`, `this` in ECX). The map gives it no name (range of
/// `BSSystemUtilities`); it is called only from `QInstance` and stores the
/// vtable at `010a42b4`, zeroes the fields at +0x84, +0x88, +0x8c (byte),
/// +0x90, +0x94 (byte) and +0x95 (byte), and clears the 0x80 bytes at +4.
/// Returns `this`.
pub fn fn_00af30a0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, SYSTEM_UTILITY_VTABLE);
    e.mem.set_u32(t + 0x84, 0);
    e.mem.set_u32(t + 0x88, 0);
    e.mem.set_u8(t + 0x8c, 0);
    e.mem.set_u32(t + 0x90, 0);
    e.mem.set_u8(t + 0x94, 0);
    e.mem.set_u8(t + 0x95, 0);
    e.call(MEMSET, &args![Ptr::<()>::new(t + 4), 0i32, 0x80u32]);
    this
}

// Translated from 00af2640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af2640 - the SEH frame (FS:[0] handler 00f22f4e, state -1/0 around the constructor call) and the 00ec408c security cookie are not translated.
/// `BSSystemUtility::QInstance` (Xbox PDB name; the exe's body is shared
/// with the other `_Immortalize` singleton getters, Ghidra's name): the
/// function-local static singleton. The first call sets bit 0 of the guard
/// word `011f7240`, constructs the object at `011f71a8` with `00af30a0` and
/// registers its destructor stub `00fdc2f0` with `_atexit`; every call
/// returns `011f71a8`.
pub fn bs_system_utility_q_instance(e: &mut Engine) -> Ptr {
    let guard = e.mem.u32(SYSTEM_UTILITY_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(SYSTEM_UTILITY_GUARD, guard | 1);
        fn_00af30a0(e, Ptr::new(SYSTEM_UTILITY));
        e.call(ATEXIT, &args![Ptr::<()>::new(SYSTEM_UTILITY_ATEXIT)]);
    }
    Ptr::new(SYSTEM_UTILITY)
}

/// `BSSaveDataSystemUtility` singleton (`BSSystemUtilities`; the vtable's
/// RTTI name is `.?AVBSSaveDataSystemUtility@@`): its storage in `.data`
/// (`011f72a0`, at least 0x48 bytes) and the function-local static's guard
/// word (`011f72e8`, bit 0 = constructed).
pub const SAVE_DATA_UTILITY: u32 = 0x011f_72a0;
const SAVE_DATA_UTILITY_GUARD: u32 = 0x011f_72e8;
/// Vtable stored by the constructor and the destructor (`010a4288`, first
/// slot the scalar deleting destructor `00af2fc0`) and the `atexit`
/// destructor stub (`00fdc300`, a row of the `atexit` table: it calls
/// `00af2f20` on `011f72a0`).
const SAVE_DATA_UTILITY_VTABLE: u32 = 0x010a_4288;
const SAVE_DATA_UTILITY_ATEXIT: u32 = 0x00fd_c300;

// Translated from 00af2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the object the `00af2e50` getter builds (`__thiscall`,
/// plain `RET`; the map gives no name): stores the vtable `010a4288`,
/// clears the bytes at +4 and +0xC and the words at +8, +0x2C and +0x44.
/// Returns `this`.
pub fn fn_00af2ed0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, SAVE_DATA_UTILITY_VTABLE);
    e.mem.set_u8(t + 4, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u8(t + 0xc, 0);
    e.mem.set_u32(t + 0x2c, 0);
    e.mem.set_u32(t + 0x44, 0);
    this
}

// Translated from 00af2e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af2e50 - the SEH frame (FS:[0] handler 00f2303e, state -1/0 around the constructor call) is not translated.
/// The `BSSaveDataSystemUtility` singleton getter (the map gives no name;
/// its body is the same `_Immortalize` shape as
/// [`bs_system_utility_q_instance`]): the first call sets bit 0 of the
/// guard word `011f72e8`, constructs the object at `011f72a0` with
/// [`fn_00af2ed0`] and registers its destructor stub `00fdc300` with
/// `_atexit`; every call returns `011f72a0`.
pub fn bs_save_data_system_utility_instance(e: &mut Engine) -> Ptr {
    let guard = e.mem.u32(SAVE_DATA_UTILITY_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(SAVE_DATA_UTILITY_GUARD, guard | 1);
        fn_00af2ed0(e, Ptr::new(SAVE_DATA_UTILITY));
        e.call(ATEXIT, &args![Ptr::<()>::new(SAVE_DATA_UTILITY_ATEXIT)]);
    }
    Ptr::new(SAVE_DATA_UTILITY)
}

// Translated from 00af2f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of that object (`__thiscall`, plain `RET`; the map gives
/// no name; the scalar deleting destructor `00af2fc0` and the `atexit` stub
/// `00fdc300` call it): stores the vtable `010a4288` again and does nothing
/// else.
pub fn fn_00af2f20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SAVE_DATA_UTILITY_VTABLE);
}

/// `BSNetworkSystemUtility` (`BSSystemUtilities`): the instance pointer (`011f7270`, null until
/// the first [`bs_network_system_utility_q_instance`]).
const NETWORK_UTILITY_INSTANCE: u32 = 0x011f_7270;

// Translated from 00af2c90 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af2c90 - the SEH frame (FS:[0] handler 00f2300b, state -1/0 around the constructor call) is not translated.
/// `BSNetworkSystemUtility::QInstance` (engine map): when the pointer at `011f7270` is null, an
/// 8-byte block from the memory manager (`00aa3e40` on `011f6238`) constructed by
/// [`fn_00af2d40`] (null when the allocation fails) is stored there; answers the pointer.
pub fn bs_network_system_utility_q_instance(e: &mut Engine) -> Ptr {
    if e.mem.u32(NETWORK_UTILITY_INSTANCE) == 0 {
        let block = e.call(0x00aa_3e40, &args![MEMORY_MANAGER, 8u32]).u32();
        let made = if block != 0 {
            fn_00af2d40(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(NETWORK_UTILITY_INSTANCE, made);
    }
    Ptr::new(e.mem.u32(NETWORK_UTILITY_INSTANCE))
}

// Translated from 00af2d40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `BSNetworkSystemUtility` constructor (`thiscall`, plain `RET`; the map gives no name):
/// the byte at +0 and the word at +4 cleared; answers `this`.
pub fn fn_00af2d40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00af2d70 (disassembled, FalloutNV.exe 1.4.0.525)
/// Sets the `BSNetworkSystemUtility`'s callback (`thiscall`, `RET 4`; the map gives no name):
/// stores it at +4 and, when it is not null, calls it at once (cdecl) with 0 and whether the
/// byte at +0 is nonzero.
pub fn fn_00af2d70(e: &mut Engine, this: Ptr, callback: u32) {
    e.mem.set_u32(this.addr() + 4, callback);
    if callback != 0 {
        let on = u32::from(e.mem.u8(this.addr()) != 0);
        e.call(callback, &args![0u32, on]);
    }
}

/// `BSMsgDialogSystemUtility` singleton (`BSSystemUtilities`): its storage in `.data`
/// (`011f72ec`, 8 bytes), the function-local static's guard word (`011f72f4`, bit 0 =
/// constructed), its vtable (`010a42cc`) and its `atexit` destructor stub (`00fdc310`).
const MSG_DIALOG_UTILITY: u32 = 0x011f_72ec;
const MSG_DIALOG_UTILITY_GUARD: u32 = 0x011f_72f4;
const MSG_DIALOG_UTILITY_VTABLE: u32 = 0x010a_42cc;
const MSG_DIALOG_UTILITY_ATEXIT: u32 = 0x00fd_c310;

// Translated from 00af3020 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af3020 - the SEH frame (FS:[0] handler 00f2306e, state -1/0 around the constructor call) is not translated.
/// The `BSMsgDialogSystemUtility` singleton getter (the map gives no name; the same
/// `_Immortalize` shape as [`bs_system_utility_q_instance`]): the first call sets bit 0 of the
/// guard word `011f72f4`, constructs the object at `011f72ec` with [`fn_00af33f0`] and
/// registers its destructor stub `00fdc310` with `_atexit`; every call returns `011f72ec`.
pub fn fn_00af3020(e: &mut Engine) -> Ptr {
    let guard = e.mem.u32(MSG_DIALOG_UTILITY_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(MSG_DIALOG_UTILITY_GUARD, guard | 1);
        fn_00af33f0(e, Ptr::new(MSG_DIALOG_UTILITY));
        e.call(ATEXIT, &args![Ptr::<()>::new(MSG_DIALOG_UTILITY_ATEXIT)]);
    }
    Ptr::new(MSG_DIALOG_UTILITY)
}

// Translated from 00af33f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `BSMsgDialogSystemUtility` constructor (`thiscall`, plain `RET`; the map gives no name):
/// the vtable `010a42cc` at +0, the word at +4 cleared; answers `this`.
pub fn fn_00af33f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), MSG_DIALOG_UTILITY_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 008526c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSMsgDialogSystemUtility::QInstance` (engine map; `BSSystemUtilities/
/// bsmsgdialogsystemutility_xenon.cpp`, `thiscall` with `ECX` unused, plain `RET`): answers
/// [`fn_00af3020`]'s singleton.
pub fn bs_msg_dialog_system_utility_q_instance(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call_as(0x00af_3020, &args![])
}

// ---- Files ----------------------------------------------------------------------

/// `BSFile` (Xbox PDB, `BSSystem/bsfile_xenon.cpp`, platform): the game's
/// file object, a Gamebryo `NiFile` with a read buffer (data +0x20, size
/// +0x10, bytes held +0x14, cursor +0x18, `FILE*` +0x24, "ready" byte
/// +0x2C) and two copies of its position (+0x38, +0x150; `004720a0`,
/// `00472380` read them). Its size, 0x158 bytes, and its vtable.
pub const BSFILE_SIZE: u32 = 0x158;
pub const BSFILE_VTABLE: u32 = 0x010a_4764;
const BSFILE_READY: u32 = 0x2c;
const BSFILE_POSITION: u32 = 0x38;
const BSFILE_POSITION_COPY: u32 = 0x150;

/// A file the host supplies in memory, behind a `BSFile`: its name, its
/// size, and the byte ranges supplied, at their offsets in the real file.
/// Only what the host has supplied can be read; a read of anything else
/// stops (the engine panics with the range), it never reads zeros.
#[derive(Debug, Clone)]
pub struct MemoryFile {
    pub name: String,
    pub size: u32,
    pieces: std::collections::BTreeMap<u32, Vec<u8>>,
}

impl MemoryFile {
    /// The `n` bytes at `at`, when supplied.
    fn bytes(&self, at: u32, n: u32) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(n as usize);
        let mut pos = at;
        let end = at + n;
        while pos < end {
            let (&start, piece) = self.pieces.range(..=pos).next_back()?;
            let offset = (pos - start) as usize;
            if offset >= piece.len() {
                return None;
            }
            let take = (piece.len() - offset).min((end - pos) as usize);
            out.extend_from_slice(&piece[offset..offset + take]);
            pos += take as u32;
        }
        Some(out)
    }
}

/// Opens an in-memory file (`name`, `size` bytes in the real file) as a
/// `BSFile` and answers the object. Stands in for what the `BSFile`
/// constructor (`00b00260`) and its open (vtable +0x20, `00aff300`) leave
/// after a buffered read: an object with the game's `BSFile` vtable, ready
/// (+0x2C), at position 0. Its reads and seeks go through [`bsfile_read_f`]
/// and [`bsfile_seek`]; supply its bytes with [`supply_file_bytes`].
///
/// `BSFile::ReadF` (`00b00650`) and the seek (`00b00590`) are translated in `engine-bs`
/// (`bssystem::bsfile`) for the game's real files. An engine without them (this crate's own
/// tests) gets [`bsfile_read_f`] and [`bsfile_seek`] registered there here; with them, the
/// program that makes memory files routes those two addresses to the stand-ins for the files
/// [`is_memory_file`] names (the host's `game_file` does).
pub fn open_memory_file(e: &mut Engine, name: &str, size: u32) -> u32 {
    if !e.is_translated(0x00b0_0650) {
        e.register(
            0x00b0_0650,
            entry!(0, bsfile_read_f(Ptr, Ptr, u32) -> u32).1,
        );
    }
    if !e.is_translated(0x00b0_0590) {
        e.register(0x00b0_0590, entry!(0, bsfile_seek(Ptr, i32, i32)).1);
    }
    let file = e.mem.alloc(BSFILE_SIZE);
    e.mem.set_u32(file, BSFILE_VTABLE);
    e.mem.set_u8(file + BSFILE_READY, 1);
    e.mem.set_u32(file + BSFILE_POSITION, 0);
    e.mem.set_u32(file + BSFILE_POSITION_COPY, 0);
    e.kernel.files.insert(
        file,
        MemoryFile {
            name: name.to_string(),
            size,
            pieces: std::collections::BTreeMap::new(),
        },
    );
    file
}

/// Whether `file` is a `BSFile` [`open_memory_file`] made.
pub fn is_memory_file(e: &Engine, file: u32) -> bool {
    e.kernel.files.contains_key(&file)
}

/// Supplies `bytes` of the in-memory file behind `file`, at `offset`.
pub fn supply_file_bytes(e: &mut Engine, file: u32, offset: u32, bytes: &[u8]) {
    let f = e
        .kernel
        .files
        .get_mut(&file)
        .unwrap_or_else(|| panic!("{file:08x}: not an in-memory file"));
    f.pieces.insert(offset, bytes.to_vec());
}

fn file_position(e: &Engine, file: u32) -> u32 {
    e.mem.u32(file + BSFILE_POSITION_COPY)
}

fn set_file_position(e: &mut Engine, file: u32, position: u32) {
    e.mem.set_u32(file + BSFILE_POSITION, position);
    e.mem.set_u32(file + BSFILE_POSITION_COPY, position);
}

// Platform: replaces 00b00650 (BSFile::ReadF (Xbox PDB), FalloutNV.exe 1.4.0.525)
/// Reads `size` bytes at the file's position into `buffer` and moves the
/// position on (+0x150, and +0x38 with it), answering how many were read:
/// fewer at the end of the file. The game's reads its `NiFile` buffer and
/// refills it with `fread` (`00aa1750`); this one reads the in-memory file.
/// A range the host hasn't supplied stops with the file's name and range.
pub fn bsfile_read_f(e: &mut Engine, this: Ptr, buffer: Ptr, size: u32) -> u32 {
    let file = this.addr();
    let position = file_position(e, file);
    let f = e
        .kernel
        .files
        .get(&file)
        .unwrap_or_else(|| panic!("BSFile {file:08x}: not an in-memory file"));
    let n = size.min(f.size.saturating_sub(position));
    let bytes = f.bytes(position, n).unwrap_or_else(|| {
        panic!(
            "BSFile {}: bytes {position:#x}..{:#x} not supplied",
            f.name,
            position + n
        )
    });
    e.mem.write(buffer.addr(), &bytes);
    set_file_position(e, file, position + n);
    n
}

// Platform: replaces 00b00590 (BSFile seek, vtable +0x14, FalloutNV.exe 1.4.0.525)
/// Moves the file's position: `whence` 0 to `offset`, 1 by `offset`, 2 to
/// the size less `offset` (as the game's computes it), else nowhere. The
/// game's seeks its `NiFile` (`00aa16d0`) when the position changes; this
/// one only sets it.
pub fn bsfile_seek(e: &mut Engine, this: Ptr, offset: i32, whence: i32) {
    let file = this.addr();
    let position = file_position(e, file);
    let size = e.kernel.files.get(&file).map_or(0, |f| f.size);
    let target = match whence {
        0 => offset as u32,
        1 => position.wrapping_add(offset as u32),
        2 => size.wrapping_sub(offset as u32),
        _ => position,
    };
    if target != position {
        set_file_position(e, file, target);
    }
}

/// The import slots of `WaitForSingleObject` and `ReleaseSemaphore` (KERNEL32), and the
/// game's counter helpers `004019a0` (`InterlockedDecrement` of its word) and `0040b460`
/// (`InterlockedIncrement` of its word), cdecl.
const SLOT_WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const SLOT_RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const COUNT_DOWN: u32 = 0x0040_19a0;
const COUNT_UP: u32 = 0x0040_b460;

// Platform: replaces 004424e0 (WaitForSingleObject (engine map, XAPILIB/waitforsingleobject.c), FalloutNV.exe 1.4.0.525)
/// The game's counted semaphore's wait (`thiscall`, no stack words; the record: its count
/// word at +0, the semaphore handle at +4): `WaitForSingleObject(handle, INFINITE)`, then
/// `004019a0(this)` counts it down. As the exe's code, from its disassembly.
pub fn counted_semaphore_wait(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr() + 4);
    e.call(SLOT_WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
    // The exe ends on this call: its `EAX` is the answer (00442510 passes it on).
    e.call(COUNT_DOWN, &args![this]).u32()
}

// Platform: replaces 00442550 (ReleaseSemaphore (engine map, XAPILIB/synch.c), FalloutNV.exe 1.4.0.525)
/// The game's counted semaphore's release (`thiscall`, no stack words; the record as for
/// [`counted_semaphore_wait`]): `ReleaseSemaphore(handle, 1, NULL)`, then `0040b460(this)`
/// counts it up. As the exe's code, from its disassembly.
pub fn counted_semaphore_release(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr() + 4);
    e.call(SLOT_RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    // The exe ends on this call: its `EAX` is the answer.
    e.call(COUNT_UP, &args![this]).u32()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_193c, d3d9::cpu_id_flag_test() -> u32),
        entry!(0x00ed_1951, d3d9::cpuid(u32, Ptr)),
        entry!(0x00ed_1964, d3d9::cpuid(u32, Ptr)),
        entry!(0x0044_24e0, counted_semaphore_wait(Ptr) -> u32),
        entry!(0x0044_2550, counted_semaphore_release(Ptr) -> u32),
        entry!(0x00401020, get_memory_manager() -> Ptr),
        entry!(0x00401030, operator_delete(Ptr)),
        entry!(0x00af2640, bs_system_utility_q_instance() -> Ptr),
        entry!(0x00af2c90, bs_network_system_utility_q_instance() -> Ptr),
        entry!(0x00af2d40, fn_00af2d40(Ptr) -> Ptr),
        entry!(0x00af2d70, fn_00af2d70(Ptr, u32)),
        entry!(0x00af30a0, fn_00af30a0(Ptr) -> Ptr),
        entry!(0x00af2e50, bs_save_data_system_utility_instance() -> Ptr),
        entry!(0x00af2ed0, fn_00af2ed0(Ptr) -> Ptr),
        entry!(0x00af2f20, fn_00af2f20(Ptr)),
        entry!(0x00af_3020, fn_00af3020() -> Ptr),
        entry!(0x00af_33f0, fn_00af33f0(Ptr) -> Ptr),
        entry!(0x0085_26c0, bs_msg_dialog_system_utility_q_instance(Ptr) -> Ptr),
        entry!(0x00aa3e40, memory_manager_allocate(Ptr, u32) -> Ptr),
        entry!(0x00aa4060, memory_manager_deallocate(Ptr, Ptr)),
    ]
}

// Imported Windows functions. The game calls them through its import
// table (`call dword ptr [slot]`); translations call the slot address, and
// `Engine::with_exe` registers these there by DLL and name. One thread, so
// the interlocked operations are plain memory operations and the critical
// sections do nothing.

// Platform: replaces KERNEL32.DLL InterlockedIncrement (import)
fn interlocked_increment(e: &mut Engine, target: Ptr) -> i32 {
    let v = e.mem.i32(target.addr()).wrapping_add(1);
    e.mem.set_i32(target.addr(), v);
    v
}

// Platform: replaces KERNEL32.DLL InterlockedDecrement (import)
fn interlocked_decrement(e: &mut Engine, target: Ptr) -> i32 {
    let v = e.mem.i32(target.addr()).wrapping_sub(1);
    e.mem.set_i32(target.addr(), v);
    v
}

// Platform: replaces KERNEL32.DLL InterlockedExchange (import)
fn interlocked_exchange(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    e.mem.set_i32(target.addr(), value);
    old
}

// Platform: replaces KERNEL32.DLL InterlockedExchangeAdd (import)
fn interlocked_exchange_add(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    e.mem.set_i32(target.addr(), old.wrapping_add(value));
    old
}

// Platform: replaces KERNEL32.DLL InterlockedCompareExchange (import)
fn interlocked_compare_exchange(e: &mut Engine, target: Ptr, exchange: i32, comparand: i32) -> i32 {
    let old = e.mem.i32(target.addr());
    if old == comparand {
        e.mem.set_i32(target.addr(), exchange);
    }
    if crate::queue_check::enabled() {
        // Diagnostic only (`NV_QUEUE_CHECK`, not exe behaviour).
        crate::queue_check::exchange(e, target.addr(), exchange, comparand, old);
    }
    old
}

// Platform: replaces KERNEL32.DLL critical sections (imports)
// A critical section belongs to one thread at a time, as Windows has it: each is a mutex
// kernel object of this table (by the section's address; `RTL_CRITICAL_SECTION` itself is left
// as the game made it). Entering one another thread holds is a switch point (the thread waits
// as on an unsignalled object until the owner leaves); the owner may enter again (counted).
// Why it matters here: the IO thread holds an archive's lock across a seek and a read while
// the main thread reads the same archive; without the lock the reads interleaved (p112).

/// The mutex standing for the critical section at `section` (made on first use).
fn critical_section_mutex(e: &mut Engine, section: Ptr) -> u32 {
    if let Some(&handle) = e.kernel.critical_sections.get(&section.addr()) {
        return handle;
    }
    let handle = e.kernel.insert(KernelObject::Mutex {
        name: Vec::new(),
        owner: 0,
        count: 0,
    });
    e.kernel.critical_sections.insert(section.addr(), handle);
    handle
}

// Platform: replaces KERNEL32.DLL InitializeCriticalSection and DeleteCriticalSection (imports)
/// Makes the section new (unowned) / forgets it.
fn critical_section_reset(e: &mut Engine, section: Ptr) {
    if let Some(handle) = e.kernel.critical_sections.remove(&section.addr()) {
        e.kernel.objects.remove(&handle);
    }
}

// Platform: replaces KERNEL32.DLL EnterCriticalSection (import)
/// Takes the section for the running thread, waiting (a switch point) while another holds it.
fn enter_critical_section(e: &mut Engine, section: Ptr) {
    let handle = critical_section_mutex(e, section);
    wait_one(e, handle, INFINITE);
}

// Platform: replaces KERNEL32.DLL LeaveCriticalSection (import)
/// Counts the owner's hold down; at 0 the section is free and its waiters may go on.
fn leave_critical_section(e: &mut Engine, section: Ptr) {
    let Some(&handle) = e.kernel.critical_sections.get(&section.addr()) else {
        return;
    };
    let freed = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Mutex { owner, count, .. }) if *count > 0 => {
            *count -= 1;
            if *count == 0 {
                *owner = 0;
                true
            } else {
                false
            }
        }
        _ => false,
    };
    if freed {
        crate::thread_host::signal(e);
    }
}

// Platform: replaces KERNEL32.DLL TryEnterCriticalSection (import)
/// Takes the section when it is free or the running thread's (1), else 0 without waiting.
fn try_enter_critical_section(e: &mut Engine, section: Ptr) -> i32 {
    let handle = critical_section_mutex(e, section);
    if signalled(e, handle) {
        take(e, handle);
        1
    } else {
        0
    }
}

// Platform: replaces OLE32.DLL CoInitializeEx (import)
/// `CoInitializeEx(reserved, mode)` (the documented answers): COM on the running thread in that
/// concurrency mode, `S_OK` the first time on a thread, `S_FALSE` again in the same mode,
/// `RPC_E_CHANGED_MODE` in another; a reserved word other than null is `E_INVALIDARG`. COM
/// itself is not provided beyond this (`CoCreateInstance` stops).
fn co_initialize_ex(e: &mut Engine, reserved: u32, mode: u32) -> u32 {
    if reserved != 0 {
        return 0x8007_0057;
    }
    let thread = crate::thread_host::current_id(e);
    match e.kernel.com_threads.get(&thread) {
        None => {
            e.kernel.com_threads.insert(thread, mode);
            0
        }
        Some(&m) if m == mode => 1,
        Some(_) => 0x8001_0106,
    }
}

// Platform: replaces OLE32.DLL CoUninitialize (import)
/// Ends the running thread's COM (what `CoInitializeEx` began).
fn co_uninitialize(e: &mut Engine) {
    let thread = crate::thread_host::current_id(e);
    e.kernel.com_threads.remove(&thread);
}

// Platform: replaces OLE32.DLL CoTaskMemAlloc (import)
/// A block of `size` bytes from the platform's heap (COM's task allocator); 0 when it cannot
/// be had. A size of 0 still gives a block, as the task allocator does.
fn co_task_mem_alloc(e: &mut Engine, size: u32) -> u32 {
    e.mem.alloc(size.max(1))
}

// Platform: replaces OLE32.DLL CoTaskMemFree (import)
/// Frees a block from [`co_task_mem_alloc`]; null is allowed.
fn co_task_mem_free(e: &mut Engine, block: Ptr) {
    e.mem.free(block.addr());
}

// Platform: replaces OLE32.DLL CoFreeUnusedLibraries (import)
/// No COM servers are loaded from DLLs here, so there is nothing to unload.
fn co_free_unused_libraries(_e: &mut Engine) {}

// Platform: replaces KERNEL32.DLL InitializeCriticalSectionAndSpinCount (import)
fn initialize_critical_section_and_spin_count(e: &mut Engine, section: Ptr, _spin: u32) -> i32 {
    critical_section_reset(e, section);
    1
}

// Platform: replaces KERNEL32.DLL InitializeCriticalSectionAndSpinCount
// (import, slot 00fdf130): one thread, nothing to set up; nonzero is success.
// Platform: replaces KERNEL32.DLL GetCurrentThreadId (import): the running
// game thread's id (`crate::thread_host`; the main thread's is 1).
pub const MAIN_THREAD_ID: u32 = crate::thread_host::MAIN_THREAD_ID;
fn get_current_thread_id(e: &mut Engine) -> u32 {
    crate::thread_host::current_id(e)
}

// Platform: replaces KERNEL32.DLL Sleep (import)
/// A switch point (`crate::thread_host`): on the main thread the host waits the time asked for
/// (`Engine::wait_clock_until_us`: really, with the real clock on) and the others get a pass;
/// another thread sleeps until the clock reaches its wake time, without moving it.
fn sleep(e: &mut Engine, ms: u32) {
    crate::thread_host::host().sleep(e, ms, signalled);
}

/// The exception code of the debugger thread-naming convention (`SetThreadName`, `00aa2740`).
pub const MS_VC_EXCEPTION: u32 = 0x406d_1388;

// Platform: replaces KERNEL32.DLL RaiseException (import)
/// `RaiseException(code, flags, argument count, arguments)` with no debugger attached. The game
/// raises only [`MS_VC_EXCEPTION`] (`SetThreadName` `00aa2740`, inside its own `__try`, whose
/// handler takes it and goes on): that one comes back at once, as the handler would make it.
/// Any other code stops the engine (structured exceptions are not provided).
fn raise_exception(_e: &mut Engine, code: u32, flags: u32, count: u32, _arguments: Ptr) {
    if code != MS_VC_EXCEPTION {
        panic!(
            "RaiseException(code {code:#x}, flags {flags:#x}, {count} arguments): \
             structured exceptions are not provided"
        );
    }
}

// The user-interface and registry calls the game's input start-up makes (`00a22660`): the
// Windows defaults of a machine with nothing changed, read-only. The window is the viewer's.

/// `HKEY_CURRENT_USER`, the Win32 error codes these return, and `REG_SZ`.
const HKEY_CURRENT_USER: u32 = 0x8000_0001;
const ERROR_SUCCESS: u32 = 0;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_MORE_DATA: u32 = 234;
const REG_SZ: u32 = 1;
/// The double-click time Windows starts with (milliseconds).
pub const DEFAULT_DOUBLE_CLICK_TIME: u32 = 500;
/// `HKCU\Control Panel\Mouse` values Windows starts with, the ones a key here answers.
const MOUSE_VALUES: [(&[u8], &[u8]); 1] = [(b"MouseSensitivity", b"10")];

/// `ERROR_DEVICE_NOT_CONNECTED`.
const ERROR_DEVICE_NOT_CONNECTED: u32 = 0x48f;

// Platform: replaces XINPUT1_3.DLL XInputGetState (import by ordinal 2; `Controls::Poll`
// `00a23010` calls it through the thunk `009f996e`)
/// No controller is connected (the platform offers no XInput pad): `ERROR_DEVICE_NOT_CONNECTED`
/// for every user index, the state left as it is.
fn xinput_get_state(_e: &mut Engine, _user: u32, _state: Ptr) -> u32 {
    ERROR_DEVICE_NOT_CONNECTED
}

// Platform: replaces XINPUT1_3.DLL XInputSetState (import by ordinal 3, slot `00fdf398`; the
// controller rumble `00a255b0` calls it through the thunk `00ee820a`, e.g. when the player fires)
/// No controller is connected, as for [`xinput_get_state`]: `ERROR_DEVICE_NOT_CONNECTED` for every
/// user index (documented for a controller that is not connected), the vibration not read.
fn xinput_set_state(_e: &mut Engine, _user: u32, _vibration: Ptr) -> u32 {
    ERROR_DEVICE_NOT_CONNECTED
}

// Platform: replaces USER32.DLL GetActiveWindow (import)
/// The game's main window ([`create_window`]), 0 before there is one.
fn get_active_window(e: &mut Engine) -> u32 {
    e.kernel.main_window
}

// Platform: replaces USER32.DLL SwapMouseButton (import)
/// Sets whether the buttons are swapped and answers whether they were (not at first).
fn swap_mouse_button(e: &mut Engine, swap: u32) -> u32 {
    let was = e.kernel.mouse_buttons_swapped;
    e.kernel.mouse_buttons_swapped = swap != 0;
    u32::from(was)
}

// Platform: replaces USER32.DLL GetDoubleClickTime (import)
fn get_double_click_time(_e: &mut Engine) -> u32 {
    DEFAULT_DOUBLE_CLICK_TIME
}

// Platform: replaces ADVAPI32.DLL RegOpenKeyExA (import)
/// Opens `HKCU\Control Panel\Mouse` (the only key provided, read-only, with the Windows
/// defaults); any other key is `ERROR_FILE_NOT_FOUND`.
fn reg_open_key_ex_a(
    e: &mut Engine,
    root: u32,
    name: Ptr,
    _options: u32,
    _access: u32,
    out: Ptr,
) -> u32 {
    let name = e.mem.cstr(name.addr());
    if root != HKEY_CURRENT_USER || !name.eq_ignore_ascii_case(br"Control Panel\Mouse") {
        return ERROR_FILE_NOT_FOUND;
    }
    let key = e.kernel.insert(KernelObject::RegistryKey);
    e.mem.set_u32(out.addr(), key);
    ERROR_SUCCESS
}

// Platform: replaces ADVAPI32.DLL RegQueryValueExA (import)
/// A string value of the mouse key ([`MOUSE_VALUES`], `REG_SZ`, with its terminating zero):
/// the type through `kind`, the bytes into `data` when `*size` holds them (else
/// `ERROR_MORE_DATA`), the size needed in `*size` either way; no data buffer asks the size only.
fn reg_query_value_ex_a(
    e: &mut Engine,
    key: u32,
    name: Ptr,
    _reserved: u32,
    kind: Ptr,
    data: Ptr,
    size: Ptr,
) -> u32 {
    if !matches!(e.kernel.objects.get(&key), Some(KernelObject::RegistryKey)) {
        return ERROR_INVALID_HANDLE;
    }
    let name = e.mem.cstr(name.addr());
    let Some((_, value)) = MOUSE_VALUES
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(&name))
    else {
        return ERROR_FILE_NOT_FOUND;
    };
    let needed = value.len() as u32 + 1;
    if !kind.is_null() {
        e.mem.set_u32(kind.addr(), REG_SZ);
    }
    if data.is_null() {
        if !size.is_null() {
            e.mem.set_u32(size.addr(), needed);
        }
        return ERROR_SUCCESS;
    }
    let room = e.mem.u32(size.addr());
    e.mem.set_u32(size.addr(), needed);
    if room < needed {
        return ERROR_MORE_DATA;
    }
    e.mem.set_cstr(data.addr(), value);
    ERROR_SUCCESS
}

// Platform: replaces ADVAPI32.DLL RegCloseKey (import)
fn reg_close_key(e: &mut Engine, key: u32) -> u32 {
    match e.kernel.objects.get(&key) {
        Some(KernelObject::RegistryKey) => {
            e.kernel.objects.remove(&key);
            ERROR_SUCCESS
        }
        _ => ERROR_INVALID_HANDLE,
    }
}

/// ASCII white space taken off both ends (`<[u8]>::trim_ascii` is newer than the MSRV).
trait TrimSpaces {
    fn trim_spaces(&self) -> &Self;
}

impl TrimSpaces for [u8] {
    fn trim_spaces(&self) -> &[u8] {
        let start = self
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(self.len());
        let end = self
            .iter()
            .rposition(|b| !b.is_ascii_whitespace())
            .map_or(start, |i| i + 1);
        &self[start..end.max(start)]
    }
}

/// The value of `key` in section `section` of the INI text `text` (names compared without case,
/// spaces around names and values trimmed, one pair of surrounding quotes taken off), as
/// `GetPrivateProfileStringA` finds it.
fn ini_value(text: &[u8], section: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    let mut inside = false;
    for line in text.split(|&b| b == b'\n') {
        let line = line.trim_spaces();
        if let Some(rest) = line.strip_prefix(b"[") {
            let name = rest.split(|&b| b == b']').next().unwrap_or(rest);
            inside = name.trim_spaces().eq_ignore_ascii_case(section);
            continue;
        }
        if !inside {
            continue;
        }
        let Some(at) = line.iter().position(|&b| b == b'=') else {
            continue;
        };
        if line[..at].trim_spaces().eq_ignore_ascii_case(key) {
            let value = line[at + 1..].trim_spaces();
            let value = match value {
                [b'"', middle @ .., b'"'] => middle,
                _ => value,
            };
            return Some(value.to_vec());
        }
    }
    None
}

/// The text of the profile file `file` names: a name without a full path is in the Windows
/// directory, as the profile functions look for it (`d3d9::windows_directory`, from the
/// player's table); the file as the game made it in this run when it did (`files`), else the
/// player's file, read only. `None` when there is none.
fn profile_text(e: &mut Engine, file: Ptr) -> Option<Vec<u8>> {
    let name = e.mem.cstr(file.addr());
    let path = if files::is_full_path(&name) {
        name
    } else {
        let mut path = d3d9::windows_directory(e);
        path.push(b'\\');
        path.extend_from_slice(&name);
        path
    };
    files::read_whole(e, &path)
}

// Platform: replaces KERNEL32.DLL GetPrivateProfileIntA (import)
/// `GetPrivateProfileIntA(section, key, default, file)`: the key's value as an integer, or
/// `default` when the file, the section or the key isn't there ([`profile_text`]). The value is
/// read as [`profile_int`] does, which follows Windows' answers measured on the player's machine
/// (2026-10-21, 38 values).
fn get_private_profile_int_a(
    e: &mut Engine,
    section: Ptr,
    key: Ptr,
    default: u32,
    file: Ptr,
) -> u32 {
    let section = e.mem.cstr(section.addr());
    let key = e.mem.cstr(key.addr());
    let Some(value) = profile_text(e, file).and_then(|text| ini_value(&text, &section, &key))
    else {
        return default;
    };
    profile_int(&value)
}

/// A profile value as Windows' `GetPrivateProfileIntA` reads it (measured): leading spaces and
/// tabs, then one optional `+` or `-`, then an optional lower-case base prefix `0x` (16), `0o`
/// (8) or `0b` (2), else base 10; digits of that base up to the first other character, kept
/// modulo 2^32; a `-` negates the result modulo 2^32. No digits (`-`, `0x`, `0b2`, `x5`) give 0;
/// upper-case prefixes are not prefixes (`0X1F` is 0).
fn profile_int(value: &[u8]) -> u32 {
    let mut rest = value;
    while let [b' ' | b'\t', tail @ ..] = rest {
        rest = tail;
    }
    let negative = match rest {
        [b'-', tail @ ..] => {
            rest = tail;
            true
        }
        [b'+', tail @ ..] => {
            rest = tail;
            false
        }
        _ => false,
    };
    let base = match rest {
        [b'0', b'x', tail @ ..] => {
            rest = tail;
            16
        }
        [b'0', b'o', tail @ ..] => {
            rest = tail;
            8
        }
        [b'0', b'b', tail @ ..] => {
            rest = tail;
            2
        }
        _ => 10,
    };
    let mut n = 0u32;
    for &c in rest {
        let Some(digit) = char::from(c).to_digit(base) else {
            break;
        };
        n = n.wrapping_mul(base).wrapping_add(digit);
    }
    if negative {
        n.wrapping_neg()
    } else {
        n
    }
}

// Platform: replaces KERNEL32.DLL GetPrivateProfileStringA (import)
/// The value of `key` in `section` of the INI file `file` ([`profile_text`]), or
/// `default` (none: empty) when the file, the section or the key isn't there: copied into
/// `buffer`, cut to `size - 1` bytes and zero-terminated; answers the bytes copied. Listing a
/// file's sections or keys (a null section or key) is not provided.
fn get_private_profile_string_a(
    e: &mut Engine,
    section: Ptr,
    key: Ptr,
    default: Ptr,
    buffer: Ptr,
    size: u32,
    file: Ptr,
) -> u32 {
    if section.is_null() || key.is_null() {
        panic!("GetPrivateProfileStringA with a null section or key is not provided");
    }
    let section = e.mem.cstr(section.addr());
    let key = e.mem.cstr(key.addr());
    let found = profile_text(e, file).and_then(|text| ini_value(&text, &section, &key));
    let value = match found {
        Some(value) => value,
        None if default.is_null() => Vec::new(),
        None => e.mem.cstr(default.addr()),
    };
    if size == 0 {
        return 0;
    }
    let copied = value.len().min(size as usize - 1);
    e.mem.set_cstr(buffer.addr(), &value[..copied]);
    copied as u32
}

// Platform: replaces KERNEL32.DLL WritePrivateProfileStringA (import)
/// Answers success without writing: the platform never changes the player's files.
fn write_private_profile_string_a(
    _e: &mut Engine,
    _section: Ptr,
    _key: Ptr,
    _value: Ptr,
    _file: Ptr,
) -> u32 {
    1
}

// Platform: replaces KERNEL32.DLL GetLastError (import): the thread's
// last-error code, which only SetLastError changes here (one thread).
fn get_last_error(e: &mut Engine) -> u32 {
    e.kernel.last_error
}

// Platform: replaces KERNEL32.DLL SetLastError (import)
fn set_last_error(e: &mut Engine, code: u32) {
    e.kernel.last_error = code;
}

// Platform: replaces USER32.DLL GetSystemMetrics (import)
/// `GetSystemMetrics(index)`: the screen metrics the Direct3D 9 layer's capture recorded for a
/// program without a DPI-awareness manifest (`d3d9::system_metric`: `SM_CXSCREEN`,
/// `SM_CYSCREEN`, `SM_CMONITORS`). Other metrics are not provided.
fn get_system_metrics(e: &mut Engine, index: u32) -> u32 {
    match d3d9::system_metric(e, index) {
        Some(value) => value,
        None => panic!("GetSystemMetrics({index:#x}) is not provided"),
    }
}

// Platform: replaces D3D9.DLL D3DPERF_SetOptions (import)
/// `D3DPERF_SetOptions(options)`: bit 0 asks PIX not to profile the program; there is no
/// profiler here, so nothing changes.
fn d3dperf_set_options(_e: &mut Engine, _options: u32) {}

// Platform: replaces KERNEL32.DLL lstrcpyA (import)
/// `lstrcpyA(destination, source)`: the source string with its terminating zero copied to the
/// destination; answers the destination. A null pointer (Windows catches the fault and answers
/// null) is not provided.
fn lstrcpy_a(e: &mut Engine, destination: Ptr, source: Ptr) -> u32 {
    assert!(
        !destination.is_null() && !source.is_null(),
        "lstrcpyA with a null pointer is not provided"
    );
    let text = e.mem.cstr(source.addr());
    e.mem.set_cstr(destination.addr(), &text);
    destination.addr()
}

// Platform: replaces KERNEL32.DLL lstrcatA (import)
/// `lstrcatA(destination, source)`: the source string with its terminating zero copied after the
/// destination's; answers the destination. A null pointer is not provided.
fn lstrcat_a(e: &mut Engine, destination: Ptr, source: Ptr) -> u32 {
    assert!(
        !destination.is_null() && !source.is_null(),
        "lstrcatA with a null pointer is not provided"
    );
    let end = destination.addr() + e.mem.cstr(destination.addr()).len() as u32;
    let text = e.mem.cstr(source.addr());
    e.mem.set_cstr(end, &text);
    destination.addr()
}

// Platform: replaces WINMM.DLL timeGetTime (import): the platform clock in milliseconds. It moved
// in 1 ms steps on the reference machine (its timer resolution was 1 ms; a29, see
// [`TICK_INTERVAL_US`]). The platform's own time stamps (DirectInput's) read it too.
fn tick_count(e: &mut Engine) -> u32 {
    e.clock_now_ms()
}

/// How far Windows' `GetTickCount` moves at once: one period of the clock interrupt at its
/// longest, 15.625 ms, whatever timer resolution a program asks for. Measured on the reference
/// machine (Windows 11 26200, a29 `ticks.ps1`, 2026-10-28): over 2 s, steps of 15 ms (48) and
/// 16 ms (79), the same after `timeBeginPeriod(1)`; `timeGetTime` moved in 1 ms steps.
pub const TICK_INTERVAL_US: u64 = 15_625;

// Platform: replaces KERNEL32.DLL GetTickCount (import): the platform clock in whole ticks of
// [`TICK_INTERVAL_US`], in milliseconds rounded down, as the real one counts
// (`TickCount * 15.625`). The game's frame timer (`BSTimer`, `00aa4ee0`) paces frames by it.
fn get_tick_count(e: &mut Engine) -> u32 {
    // `NV_TICK_COUNT=1ms` (a measurement setting of the host, not Windows' behaviour): the
    // millisecond clock as before, to compare the frame pacing with it.
    static MILLISECONDS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *MILLISECONDS.get_or_init(|| std::env::var("NV_TICK_COUNT").is_ok_and(|v| v == "1ms")) {
        return tick_count(e);
    }
    whole_ticks_ms(e.clock_now_us())
}

/// The milliseconds `GetTickCount` reads at `us` microseconds of the clock.
fn whole_ticks_ms(us: u64) -> u32 {
    (us / TICK_INTERVAL_US * TICK_INTERVAL_US / 1000) as u32
}

// Platform: replaces KERNEL32.DLL QueryPerformanceFrequency (import): 1 MHz.
fn query_performance_frequency(e: &mut Engine, out: Ptr) -> i32 {
    e.mem.set_u64(out.addr(), 1_000_000);
    1
}

// Platform: replaces KERNEL32.DLL QueryPerformanceCounter (import): the
// engine clock in microseconds (`Engine::clock_now_us`).
fn query_performance_counter(e: &mut Engine, out: Ptr) -> i32 {
    let now = e.clock_now_us();
    e.mem.set_u64(out.addr(), now);
    1
}

// Kernel objects and virtual memory. Handles are small integers (4, 8, ...,
// never reused) into the table `Engine::kernel`; events and semaphores keep
// their state as Windows does. Game threads run cooperatively, one at a time,
// on the main thread's turn (`crate::thread_host`): a wait on an object that
// is not signalled is a switch point, and a wait that nothing can satisfy
// (no thread can go on) stops the engine with a message naming the import and
// handle, as a call to an untranslated function does.

/// `WaitForSingleObject` timeout and results.
const INFINITE: u32 = 0xffff_ffff;
const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x102;
const WAIT_FAILED: u32 = 0xffff_ffff;
/// `GetCurrentThread`'s pseudo handle.
pub const CURRENT_THREAD: u32 = 0xffff_fffe;
/// `CreateThread` flag.
const CREATE_SUSPENDED: u32 = 4;
/// `VirtualAlloc`/`VirtualFree` types and the one protection the game
/// passes.
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_DECOMMIT: u32 = 0x4000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 4;
const PAGE_BYTES: u32 = 0x1000;
/// Reservations start on 64 KiB boundaries, as on Windows.
pub const ALLOCATION_GRANULARITY: u32 = 0x1_0000;
/// End of FalloutNV.exe's image (`.reloc` ends at `014089ff`): a reserve
/// at a fixed address inside the image fails, as on Windows.
const IMAGE_END: u32 = 0x0140_9000;

/// A kernel object behind a handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelObject {
    Event {
        manual_reset: bool,
        signalled: bool,
    },
    Semaphore {
        count: i32,
        maximum: i32,
    },
    /// A thread made by `CreateThread`; it runs through the thread host
    /// (`crate::thread_host`), which keeps its run state.
    Thread {
        start: u32,
        parameter: u32,
        id: u32,
        suspend_count: u32,
        /// The thread's x87 control word while it does not run (its FPU
        /// context): [`crate::x87::DEFAULT_CONTROL_WORD`] for a new thread,
        /// as measured on the game's AI linear task threads.
        /// [`run_as_thread`] swaps it in.
        x87_control_word: u16,
    },
    /// The game's main window ([`create_window`]): only a handle; the window itself is the
    /// viewer's (platform).
    Window,
    /// An open registry key ([`reg_open_key_ex_a`]): `HKCUControl PanelMouse`.
    RegistryKey,
    /// A mutex ([`window::create_mutex_a`]): its name, the owning thread's id (0: none) and how
    /// many times the owner took it.
    Mutex {
        name: Vec<u8>,
        owner: u32,
        count: u32,
    },
    /// A USER32 or GDI32 object this layer only hands out a handle for (an icon, a stock brush).
    UserObject(&'static str),
    /// An open file ([`files::create_file_a`]): its path and position.
    File(files::OpenFile),
    /// A file search ([`files::find_first_file_a`]): the entries it has left to answer.
    FindFile(files::Search),
    /// A heap from [`heap_create`]: only its handle.
    Heap,
}

/// A `VirtualAlloc` reservation: its size and which pages are committed.
/// `block` is the [`crate::Mem`] heap block behind a reservation the
/// system placed (address 0), 0 for one at an address the caller chose.
#[derive(Debug, Clone)]
pub struct Region {
    size: u32,
    block: u32,
    committed: Vec<bool>,
}

/// The handle table and the `VirtualAlloc` reservations (`Engine::kernel`).
#[derive(Debug, Default)]
pub struct Kernel {
    objects: crate::engine::AddrMap<u32, KernelObject>,
    last_handle: u32,
    /// The game threads and their scheduling (`crate::thread_host`).
    pub threads: crate::thread_host::Threads,
    /// The reservations by base address (public for the scrap heap's tests in
    /// `engine-bs`).
    pub regions: std::collections::BTreeMap<u32, Region>,
    /// The in-memory files behind `BSFile`s ([`open_memory_file`]), by the
    /// `BSFile`'s address.
    files: std::collections::BTreeMap<u32, MemoryFile>,
    /// Which thread-local storage indexes `TlsAlloc` handed out.
    tls_allocated: Vec<bool>,
    /// The values `TlsSetValue` stored, by (thread id, index).
    tls_values: crate::engine::AddrMap<(u32, u32), u32>,
    /// The process's current directory (`GetCurrentDirectoryA`), in the
    /// game's ANSI form; `Engine::with_exe` sets it to the executable's
    /// folder, where Steam starts the game.
    pub current_directory: Vec<u8>,
    /// The game's main window ([`create_window`]), which `GetActiveWindow` answers.
    pub main_window: u32,
    /// Whether `SwapMouseButton` swapped the buttons.
    mouse_buttons_swapped: bool,
    /// The DirectInput 8 objects and the keyboard and mouse state behind them ([`dinput`]).
    pub input: dinput::Input,
    /// Fiber-local storage ([`loader`]).
    pub fls: loader::Fls,
    /// The DirectSound 8 objects ([`dsound`]).
    pub sound: dsound::Sound,
    /// The Direct3D 9 objects ([`d3d9`]).
    pub graphics: d3d9::Graphics,
    /// The sizes the CRT's heap blocks were asked for (`malloc`, `realloc`, `__calloc_crt` on
    /// the platform allocator), as `HeapSize` answers them for `_msize` (`crate::units::crt`).
    pub crt_block_sizes: std::collections::HashMap<u32, u32>,
    /// Window classes, windows, the message queue and the command line ([`window`]).
    pub windows: window::Windows,
    /// The files the game made in this run ([`files`]).
    pub disk: files::Disk,
    /// The threads that called `CoInitializeEx`, with their concurrency mode.
    pub com_threads: std::collections::BTreeMap<u32, u32>,
    /// The critical sections by address: the mutex object standing for each.
    pub critical_sections: crate::engine::AddrMap<u32, u32>,
    /// The process side: the ANSI command line and the environment blocks ([`process`]).
    pub process: process::Process,
    /// The thread's last-error code (`GetLastError`/`SetLastError`).
    last_error: u32,
}

impl Kernel {
    /// The object behind `handle`, if it is open.
    pub fn object(&self, handle: u32) -> Option<&KernelObject> {
        self.objects.get(&handle)
    }

    fn insert(&mut self, object: KernelObject) -> u32 {
        self.last_handle += 4;
        self.objects.insert(self.last_handle, object);
        self.last_handle
    }

    /// Base of the reservation holding all of `[start, end)`.
    fn region_containing(&self, start: u64, end: u64) -> Option<u32> {
        let (&base, region) = self.regions.range(..=start as u32).next_back()?;
        (end <= base as u64 + region.size as u64).then_some(base)
    }

    /// Whether `[start, end)` overlaps a reservation.
    fn overlaps_region(&self, start: u64, end: u64) -> bool {
        self.regions
            .iter()
            .any(|(&b, r)| start < b as u64 + r.size as u64 && (b as u64) < end)
    }
}

fn page_up(v: u64) -> u64 {
    v.div_ceil(PAGE_BYTES as u64) * PAGE_BYTES as u64
}

/// Reserves `size` bytes at `address` (0: anywhere). Returns the base.
fn reserve(e: &mut Engine, address: u32, size: u32) -> Option<u32> {
    let (base, len, block) = if address == 0 {
        // From the Mem heap, aligned within an oversized block.
        let len = page_up(size as u64);
        if len > (crate::mem::HEAP_END - crate::mem::HEAP_BASE) as u64 {
            return None;
        }
        let len = len as u32;
        let block = e.mem.alloc(len + ALLOCATION_GRANULARITY);
        let base = block.next_multiple_of(ALLOCATION_GRANULARITY);
        (base, len, block)
    } else {
        let base = address & !(ALLOCATION_GRANULARITY - 1);
        let end = page_up(address as u64 + size as u64);
        let overlaps = |lo: u32, hi: u32| (base as u64) < hi as u64 && end > lo as u64;
        if base < ALLOCATION_GRANULARITY
            || end > 1 << 32
            || overlaps(crate::exe::IMAGE_BASE, IMAGE_END)
            || overlaps(crate::mem::HEAP_BASE, crate::mem::HEAP_END)
            || e.kernel.overlaps_region(base as u64, end)
            || (base as u64..end)
                .step_by(PAGE_BYTES as usize)
                .any(|p| e.mem.is_mapped(p as u32))
        {
            return None;
        }
        (base, (end - base as u64) as u32, 0)
    };
    // Reserved pages are not addressable until committed.
    e.mem.unmap(base, len);
    e.kernel.regions.insert(
        base,
        Region {
            size: len,
            block,
            committed: vec![false; (len / PAGE_BYTES) as usize],
        },
    );
    Some(base)
}

/// Commits the pages of `[start, start + len)` in the reservation at
/// `base`; newly committed pages read as zero.
fn commit(e: &mut Engine, base: u32, start: u32, len: u32) {
    for page in (start..start.wrapping_add(len)).step_by(PAGE_BYTES as usize) {
        let i = ((page - base) / PAGE_BYTES) as usize;
        let region = e.kernel.regions.get_mut(&base).expect("reservation");
        if !region.committed[i] {
            region.committed[i] = true;
            e.mem.map(page, PAGE_BYTES);
            e.mem.write(page, &[0; PAGE_BYTES as usize]);
        }
    }
}

/// Decommits the pages of `[start, start + len)` in the reservation at
/// `base`: they are no longer addressable.
fn decommit(e: &mut Engine, base: u32, start: u32, len: u32) {
    for page in (start..start.wrapping_add(len)).step_by(PAGE_BYTES as usize) {
        let i = ((page - base) / PAGE_BYTES) as usize;
        let region = e.kernel.regions.get_mut(&base).expect("reservation");
        if region.committed[i] {
            region.committed[i] = false;
            e.mem.unmap(page, PAGE_BYTES);
        }
    }
}

// Platform: replaces KERNEL32.DLL VirtualAlloc (import)
/// `VirtualAlloc(address, size, type, protect)` with the types the game
/// passes (`ScrapHeapManager`, `MemoryPool`, the CRT small-block heap):
/// `MEM_RESERVE` at address 0 takes a 64 KiB-aligned range from the
/// [`crate::Mem`] heap (as [`memory_manager_allocate`] does); at a chosen
/// address it succeeds only where nothing is (not the image, the heap
/// region, another reservation or mapped memory) and otherwise returns 0,
/// which the memory pools' address search relies on. `MEM_COMMIT` commits
/// whole pages inside a reservation (0 if the range is not inside one), or
/// reserves and commits at address 0. Reserved pages that are not
/// committed are unmapped, so touching them faults as on Windows.
/// Returns the (rounded) start, 0 on failure.
fn virtual_alloc(e: &mut Engine, address: u32, size: u32, kind: u32, protect: u32) -> u32 {
    if kind == 0 || kind & !(MEM_COMMIT | MEM_RESERVE) != 0 || protect != PAGE_READWRITE {
        panic!(
            "VirtualAlloc({address:08x}, {size:#x}, {kind:#x}, {protect:#x}): only \
             MEM_RESERVE and MEM_COMMIT with PAGE_READWRITE are provided (platform, units/platform.rs)"
        );
    }
    if size == 0 {
        return 0;
    }
    if kind & MEM_RESERVE != 0 || address == 0 {
        let Some(base) = reserve(e, address, size) else {
            return 0;
        };
        if kind & MEM_COMMIT != 0 {
            let len = e.kernel.regions[&base].size;
            commit(e, base, base, len);
        }
        return base;
    }
    let start = address & !(PAGE_BYTES - 1);
    let end = page_up(address as u64 + size as u64);
    match e.kernel.region_containing(start as u64, end) {
        Some(base) => {
            commit(e, base, start, (end - start as u64) as u32);
            start
        }
        None => 0,
    }
}

// Platform: replaces KERNEL32.DLL VirtualFree (import)
/// `VirtualFree(address, size, type)`: `MEM_DECOMMIT` decommits the whole
/// pages of the range (size 0 at a reservation's start: all of it), which
/// must lie in one reservation; `MEM_RELEASE` (size 0, the reservation's
/// start) frees the reservation and its heap block. Returns 1, or 0 for a
/// range Windows would reject.
fn virtual_free(e: &mut Engine, address: u32, size: u32, kind: u32) -> i32 {
    match kind {
        MEM_RELEASE => {
            if size != 0 || !e.kernel.regions.contains_key(&address) {
                return 0;
            }
            let len = e.kernel.regions[&address].size;
            decommit(e, address, address, len);
            let region = e.kernel.regions.remove(&address).expect("reservation");
            e.mem.free(region.block);
            1
        }
        MEM_DECOMMIT => {
            let (start, end) = if size == 0 {
                match e.kernel.regions.get(&address) {
                    Some(r) => (address, address as u64 + r.size as u64),
                    None => return 0,
                }
            } else {
                (
                    address & !(PAGE_BYTES - 1),
                    page_up(address as u64 + size as u64),
                )
            };
            match e.kernel.region_containing(start as u64, end) {
                Some(base) => {
                    decommit(e, base, start, (end - start as u64) as u32);
                    1
                }
                None => 0,
            }
        }
        _ => panic!(
            "VirtualFree({address:08x}, {size:#x}, {kind:#x}): only MEM_DECOMMIT and \
             MEM_RELEASE are provided (platform, units/platform.rs)"
        ),
    }
}

/// Named kernel objects (shared between processes) are not provided.
fn unnamed(call: &str, name: Ptr) {
    if !name.is_null() {
        panic!(
            "{call}: named object at {:08x}; named kernel objects are not provided \
             (platform, units/platform.rs)",
            name.addr()
        );
    }
}

/// Stops the engine: a wait nothing can satisfy.
fn stop_blocked(what: String) -> ! {
    panic!(
        "{what}: no game thread can go on to signal it \
         (KERNEL32.DLL import, platform; thread host)"
    )
}

/// Whether an object would let a waiter go on now, without taking it: a
/// signalled event, a semaphore with a count, an ended thread.
pub fn signalled(e: &Engine, handle: u32) -> bool {
    match e.kernel.objects.get(&handle) {
        Some(KernelObject::Event { signalled, .. }) => *signalled,
        Some(KernelObject::Semaphore { count, .. }) => *count > 0,
        Some(KernelObject::Thread { .. }) => {
            crate::thread_host::thread_ended(e, handle).unwrap_or(false)
        }
        Some(KernelObject::Mutex { owner, .. }) => {
            *owner == 0 || *owner == crate::thread_host::current_id(e)
        }
        _ => false,
    }
}

/// Takes an object a waiter goes on with: resets an auto-reset event, counts a
/// semaphore down; an ended thread stays signalled. `None` for a handle that
/// is not open.
fn take(e: &mut Engine, handle: u32) -> Option<bool> {
    let me = crate::thread_host::current_id(e);
    let taken = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Event {
            manual_reset,
            signalled,
        }) => {
            let was = *signalled;
            if was && !*manual_reset {
                *signalled = false;
            }
            was
        }
        Some(KernelObject::Semaphore { count, .. }) => {
            let was = *count > 0;
            if was {
                *count -= 1;
            }
            was
        }
        Some(KernelObject::Thread { .. }) => {
            crate::thread_host::thread_ended(e, handle).unwrap_or(false)
        }
        Some(KernelObject::Window) => false,
        Some(KernelObject::RegistryKey) => false,
        Some(KernelObject::UserObject(_)) => false,
        // A mutex is taken by its owner again (recursively) or by anyone when unowned.
        Some(KernelObject::Mutex { owner, count, .. }) => {
            if *owner == 0 || *owner == me {
                *owner = me;
                *count += 1;
                true
            } else {
                false
            }
        }
        // A file handle is signalled while no I/O is pending (all of it is synchronous here).
        Some(KernelObject::File(_)) => true,
        // Waiting on a file search handle is not provided (not measured).
        Some(KernelObject::FindFile(_)) => {
            panic!("waiting on file search {handle:#x} is not provided")
        }
        Some(KernelObject::Heap) => panic!("waiting on heap {handle:#x} is not provided"),
        None if handle == CURRENT_THREAD => false,
        None => return None,
    };
    if taken {
        crate::thread_host::signal(e);
    }
    Some(taken)
}

/// What a waiter waits on, for a stop's message.
fn describe(e: &Engine, handle: u32) -> String {
    match e.kernel.objects.get(&handle) {
        Some(KernelObject::Event { manual_reset, .. }) => format!(
            "{} event not signalled",
            if *manual_reset {
                "manual-reset"
            } else {
                "auto-reset"
            }
        ),
        Some(KernelObject::Semaphore { maximum, .. }) => {
            format!("semaphore count 0 (maximum {maximum})")
        }
        Some(KernelObject::Thread { start, .. }) => {
            format!("thread (start {start:08x}) still running")
        }
        _ if handle == CURRENT_THREAD => "the current thread".to_string(),
        _ => "closed handle".to_string(),
    }
}

// Platform: replaces KERNEL32.DLL CreateEventA (import)
/// `CreateEventA(security, manual reset, initial state, name)`: a new
/// event's handle. Named events stop the engine.
fn create_event_a(
    e: &mut Engine,
    _security: u32,
    manual_reset: i32,
    initial: i32,
    name: Ptr,
) -> u32 {
    unnamed("CreateEventA", name);
    e.kernel.insert(KernelObject::Event {
        manual_reset: manual_reset != 0,
        signalled: initial != 0,
    })
}

/// Sets an event's state; 0 if `handle` is not an open event.
fn set_event_state(e: &mut Engine, handle: u32, state: bool) -> i32 {
    match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Event { signalled, .. }) => {
            *signalled = state;
            if state {
                crate::thread_host::signal(e);
            }
            1
        }
        _ => 0,
    }
}

// Platform: replaces KERNEL32.DLL SetEvent (import)
/// On the main thread, the threads it lets go on run before it goes on
/// (`thread_host::Scheme::wake`; `thread_host::request_wake`).
fn set_event(e: &mut Engine, handle: u32) -> i32 {
    let done = set_event_state(e, handle, true);
    if done != 0 {
        crate::thread_host::request_wake(e, signalled);
    }
    done
}

// Platform: replaces KERNEL32.DLL ResetEvent (import)
fn reset_event(e: &mut Engine, handle: u32) -> i32 {
    set_event_state(e, handle, false)
}

// Platform: replaces KERNEL32.DLL CreateSemaphoreA (import)
/// `CreateSemaphoreA(security, initial count, maximum count, name)`: a new
/// semaphore's handle, or 0 for counts Windows rejects (maximum not
/// positive, initial outside 0..=maximum). Named semaphores stop the
/// engine.
fn create_semaphore_a(
    e: &mut Engine,
    _security: u32,
    initial: i32,
    maximum: i32,
    name: Ptr,
) -> u32 {
    unnamed("CreateSemaphoreA", name);
    if maximum <= 0 || initial < 0 || initial > maximum {
        return 0;
    }
    e.kernel.insert(KernelObject::Semaphore {
        count: initial,
        maximum,
    })
}

// Platform: replaces KERNEL32.DLL ReleaseSemaphore (import)
/// `ReleaseSemaphore(handle, release count, previous count out)`: adds to
/// the count and stores the previous one (if the pointer is not null).
/// Returns 0 without a change if `handle` is not a semaphore, the release
/// count is not positive or the count would pass the maximum.
fn release_semaphore(e: &mut Engine, handle: u32, release: i32, previous: Ptr) -> i32 {
    let old = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Semaphore { count, maximum })
            if release > 0 && release <= *maximum - *count =>
        {
            let old = *count;
            *count += release;
            old
        }
        _ => return 0,
    };
    crate::thread_host::signal(e);
    if !previous.is_null() {
        e.mem.set_i32(previous.addr(), old);
    }
    // On the main thread, the threads it lets go on run now (`thread_host::Scheme::WAKE`).
    crate::thread_host::request_wake(e, signalled);
    1
}

// Platform: replaces KERNEL32.DLL WaitForSingleObject (import)
/// `WaitForSingleObject(handle, timeout)`: a signalled object is taken at
/// once (`WAIT_OBJECT_0`), resetting an auto-reset event or counting a
/// semaphore down; an ended thread's handle is signalled. Otherwise the wait
/// is a switch point (`crate::thread_host`): the other game threads run until
/// the object is signalled, none can go on, or (on the main thread) the clock
/// passes the timeout (after one pass, so a timeout of 0 gives each one
/// turn). Still not signalled, a finite wait times out (`WAIT_TIMEOUT`; the
/// clock is not advanced) and an `INFINITE` one would block forever and
/// stops the engine. An unknown handle gives `WAIT_FAILED`. A blocking call: a turn that used
/// up its time slice ends first (`thread_host::blocking_call`).
fn wait_for_single_object(e: &mut Engine, handle: u32, timeout: u32) -> u32 {
    crate::thread_host::blocking_call(e);
    wait_one(e, handle, timeout)
}

/// [`wait_for_single_object`] without the blocking call's turn end: `EnterCriticalSection`'s
/// wait, which switches only when the section is held (contended).
fn wait_one(e: &mut Engine, handle: u32, timeout: u32) -> u32 {
    let finite = timeout != INFINITE;
    let deadline = finite.then(|| e.clock_now_us() + u64::from(timeout) * 1000);
    loop {
        match take(e, handle) {
            None => return WAIT_FAILED,
            Some(true) => return WAIT_OBJECT_0,
            Some(false) => {}
        }
        if crate::thread_host::host().block(e, &[handle], false, deadline, signalled) {
            continue;
        }
        if finite {
            return WAIT_TIMEOUT;
        }
        let what = describe(e, handle);
        stop_blocked(format!(
            "WaitForSingleObject(handle {handle:#x}, INFINITE) would block forever on a {what}"
        ));
    }
}

// Platform: replaces KERNEL32.DLL WaitForMultipleObjects (import)
/// `WaitForMultipleObjects(count, handles, wait all, timeout)`: with `wait
/// all`, when every object is signalled they are all taken and
/// `WAIT_OBJECT_0` returned; otherwise the first signalled one (by index) is
/// taken and `WAIT_OBJECT_0 + index` returned. Not ready, it is a switch
/// point as in `WaitForSingleObject`, then times out or stops. A count of 0
/// or above 64, or a closed handle, gives `WAIT_FAILED`.
fn wait_for_multiple_objects(
    e: &mut Engine,
    count: u32,
    handles: Ptr,
    all: i32,
    timeout: u32,
) -> u32 {
    if count == 0 || count > 64 {
        return WAIT_FAILED;
    }
    // A blocking call: a turn that used up its time slice ends first.
    crate::thread_host::blocking_call(e);
    // At most 64 handles (checked above): on the stack.
    let mut handles_read = [0u32; 64];
    for (i, h) in handles_read[..count as usize].iter_mut().enumerate() {
        *h = e.mem.u32(handles.addr() + 4 * i as u32);
    }
    let list = &handles_read[..count as usize];
    if list
        .iter()
        .any(|&h| !e.kernel.objects.contains_key(&h) && h != CURRENT_THREAD)
    {
        return WAIT_FAILED;
    }
    let all = all != 0;
    let finite = timeout != INFINITE;
    let deadline = finite.then(|| e.clock_now_us() + u64::from(timeout) * 1000);
    loop {
        if all {
            if list.iter().all(|&h| signalled(e, h)) {
                for &h in list {
                    take(e, h);
                }
                return WAIT_OBJECT_0;
            }
        } else if let Some(i) = list.iter().position(|&h| signalled(e, h)) {
            take(e, list[i]);
            return WAIT_OBJECT_0 + i as u32;
        }
        if crate::thread_host::host().block(e, list, all, deadline, signalled) {
            continue;
        }
        if finite {
            return WAIT_TIMEOUT;
        }
        let what: Vec<String> = list.iter().map(|&h| describe(e, h)).collect();
        stop_blocked(format!(
            "WaitForMultipleObjects({count}, all {all}, INFINITE) would block forever on {}",
            what.join(", ")
        ));
    }
}

// Platform: replaces KERNEL32.DLL CloseHandle (import)
/// Closes a handle from this table; 0 for one that is not open.
fn close_handle(e: &mut Engine, handle: u32) -> i32 {
    e.kernel.objects.remove(&handle).is_some() as i32
}

// Platform: replaces KERNEL32.DLL GetCurrentThread (import)
fn get_current_thread(_e: &mut Engine) -> u32 {
    CURRENT_THREAD
}

// Platform: replaces KERNEL32.DLL CreateThread (import)
/// `CreateThread(security, stack size, start, parameter, flags, id out)`:
/// the thread is recorded with the thread host (`crate::thread_host`; handle
/// returned, its id stored through the last argument if not null). It runs
/// `start(parameter)` at the next switch point once its suspend count is 0
/// (at once runnable without `CREATE_SUSPENDED`).
fn create_thread(
    e: &mut Engine,
    _security: u32,
    _stack_size: u32,
    start: u32,
    parameter: u32,
    flags: u32,
    id_out: Ptr,
) -> u32 {
    let suspended = flags & CREATE_SUSPENDED != 0;
    let handle = e.kernel.insert(KernelObject::Thread {
        start,
        parameter,
        id: 0,
        suspend_count: u32::from(suspended),
        x87_control_word: crate::x87::DEFAULT_CONTROL_WORD,
    });
    let id = crate::thread_host::create(e, handle, start, parameter, suspended);
    if let Some(KernelObject::Thread { id: slot, .. }) = e.kernel.objects.get_mut(&handle) {
        *slot = id;
    }
    if !id_out.is_null() {
        e.mem.set_u32(id_out.addr(), id);
    }
    handle
}

/// Runs `f` as the thread `handle` (made by `CreateThread`) as far as the FPU
/// goes: the thread's x87 control word is loaded for `f` (into the thread
/// block's word and the FPU register, `crt::set_x87_control_word`), and when
/// `f` returns the word `f` left is kept in the thread's kernel object and the
/// running thread's word is loaded again. This is the FPU half of a thread
/// switch: work the game runs on another thread (the AI linear task threads
/// run the Havok step at 53-bit precision while the main thread runs at
/// 24-bit) runs with that thread's precision. The rest of the thread block is
/// the main thread's (one block). Panics when `handle` is not a thread.
pub fn run_as_thread<R>(e: &mut Engine, handle: u32, f: impl FnOnce(&mut Engine) -> R) -> R {
    let theirs = match e.kernel.objects.get(&handle) {
        Some(KernelObject::Thread {
            x87_control_word, ..
        }) => *x87_control_word,
        _ => panic!("run_as_thread: handle {handle:#x} is not a thread"),
    };
    let ours = crate::units::crt::x87_control_word(e);
    crate::units::crt::set_x87_control_word(e, theirs);
    let result = f(e);
    let left = crate::units::crt::x87_control_word(e);
    if let Some(KernelObject::Thread {
        x87_control_word, ..
    }) = e.kernel.objects.get_mut(&handle)
    {
        *x87_control_word = left;
    }
    crate::units::crt::set_x87_control_word(e, ours);
    result
}

// Direct3D 9 and the FPU. Direct3D is the platform layer's (ADR-0007); what its
// device creation does to the calling thread's FPU is behaviour the game's
// results depend on, so it is kept here.

/// `D3DCREATE_FPU_PRESERVE`: the `IDirect3D9::CreateDevice` behaviour flag
/// that leaves the calling thread's FPU state alone.
pub const D3DCREATE_FPU_PRESERVE: u32 = 0x2;
/// The x87 control word Direct3D 9 leaves on the thread that creates a device
/// without `D3DCREATE_FPU_PRESERVE`: 24-bit precision, round to nearest, all
/// exceptions masked (measured, see [`direct3d_device_created`]).
pub const DIRECT3D_CONTROL_WORD: u16 = 0x007f;

// Platform: replaces what IDirect3D9::CreateDevice (vtable +0x40) and
// IDirect3D9Ex::CreateDeviceEx (vtable +0x50) do to the calling thread's FPU
/// The FPU side of creating a Direct3D 9 device on the running thread with the
/// behaviour flags `behavior_flags`: without `D3DCREATE_FPU_PRESERVE` the
/// thread's x87 control word becomes [`DIRECT3D_CONTROL_WORD`] (24-bit
/// precision) and stays so; with it nothing changes.
///
/// The game creates its device in `NiDX9Renderer::Initialize` (`00e72e60`,
/// called by `NiDX9Renderer::Create` `00e76210` from the game's start-up
/// `004daa4c`, on the main thread): `CreateDevice` through the `IDirect3D9`
/// vtable at `00e73202` and `00e732e9` (`CreateDeviceEx` at `00e731c9` and
/// `00e732c0` when `0126f0d0` is set), with the behaviour flags of the
/// renderer's `+0x5cc`. `00e69510` puts the vertex-processing flag there, and
/// `00e72e60` adds `D3DCREATE_FPU_PRESERVE` only when its renderer flags (its
/// fourth argument, from `004dc220`: the global `01189478`, 0x29 in the exe,
/// ORed with 4 on one start-up path) have 0x40, which they never do.
/// Measured with nv-probe `fpu=1` (FalloutNV.exe 1.4.0.525, a walk in Doc
/// Mitchell's house): the main thread enters `00e72e60` with `0x027F` and
/// leaves it with `0x007F`, and keeps `0x007F` for the rest of the run
/// (`Main::OnIdle` `0086e650`, the player's mover `009e9e50`, the character
/// controller `00c73170`); the AI linear task threads, which run the Havok
/// step, keep `0x027F`.
///
/// When `00e72e60` is translated, the platform's Direct3D stand-in for those
/// two vtable slots calls this. Until then the host calls it, with the game's
/// flags (no `D3DCREATE_FPU_PRESERVE`), on the main thread at the point it
/// creates its own device, before the first frame: everything the game runs
/// after its renderer start-up runs at 24-bit precision on that thread.
pub fn direct3d_device_created(e: &mut Engine, behavior_flags: u32) {
    if behavior_flags & D3DCREATE_FPU_PRESERVE == 0 {
        crate::units::crt::set_x87_control_word(e, DIRECT3D_CONTROL_WORD);
    }
}

// Platform: replaces the window `main` makes with `CreateWindowExA` (USER32 import, slot
// `00fdf2b8`, called at `0086af42`)
/// A handle for the game's main window, which `main` passes to `Main::Main` (`0086c160`, at
/// `0086af75`). The host makes it where `main` would create the window; the window itself (its
/// size from `AdjustWindowRect`, `00fdf310`, its messages) is the viewer's.
pub fn create_window(e: &mut Engine) -> u32 {
    let window = e.kernel.insert(KernelObject::Window);
    e.kernel.main_window = window;
    window
}

/// Changes a thread's suspend count by `delta` (both the handle's record and
/// the thread host's); the previous count, or -1 for a handle that is not a
/// thread. The current thread's pseudo handle reports 0.
fn change_suspend_count(e: &mut Engine, handle: u32, delta: i32) -> u32 {
    if handle == CURRENT_THREAD {
        return 0;
    }
    let previous = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::Thread { suspend_count, .. }) => {
            let previous = *suspend_count;
            *suspend_count = previous.saturating_add_signed(delta);
            previous
        }
        _ => return 0xffff_ffff,
    };
    if let Some(t) = crate::thread_host::by_handle(e, handle) {
        t.suspend_count = previous.saturating_add_signed(delta);
    }
    previous
}

// Platform: replaces KERNEL32.DLL ResumeThread (import)
/// Lowers a thread's suspend count (not below 0) and returns the previous
/// one; at 0 the thread runs at the next switch point.
fn resume_thread(e: &mut Engine, handle: u32) -> u32 {
    change_suspend_count(e, handle, -1)
}

// Platform: replaces KERNEL32.DLL SuspendThread (import)
/// Raises a thread's suspend count and returns the previous one. (A thread
/// suspends another only between switch points here, so it never stops one
/// in the middle of its work.)
fn suspend_thread(e: &mut Engine, handle: u32) -> u32 {
    change_suspend_count(e, handle, 1)
}

/// `STILL_ACTIVE`: what `GetExitCodeThread` answers for a thread that has not ended.
const STILL_ACTIVE: u32 = 0x103;

// Platform: replaces KERNEL32.DLL GetExitCodeThread (import)
/// `GetExitCodeThread(handle, code out)`: a thread that ended (its start function returned or
/// it called `ExitThread`) gives its code, any other thread (not started, running, waiting, or
/// stopped by the run) `STILL_ACTIVE`, the current thread's pseudo handle too; TRUE. A handle
/// that is no thread: FALSE with `ERROR_INVALID_HANDLE`.
fn get_exit_code_thread(e: &mut Engine, handle: u32, code: Ptr) -> u32 {
    let value = if handle == CURRENT_THREAD {
        STILL_ACTIVE
    } else if matches!(
        e.kernel.objects.get(&handle),
        Some(KernelObject::Thread { .. })
    ) {
        match crate::thread_host::by_handle(e, handle).map(|t| t.state.clone()) {
            Some(crate::thread_host::RunState::Finished { code }) => code,
            _ => STILL_ACTIVE,
        }
    } else {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        return 0;
    };
    e.mem.set_u32(code.addr(), value);
    1
}

// Platform: replaces KERNEL32.DLL ExitThread (import)
/// Ends the running game thread with `code` (its handle becomes signalled).
fn exit_thread(e: &mut Engine, code: u32) {
    crate::thread_host::host().exit(e, code);
}

/// Thread-local storage indexes Windows provides (`TLS_MINIMUM_AVAILABLE`
/// 64 plus the 1024 expansion slots), and `TlsAlloc`'s failure value.
const TLS_INDEXES: usize = 1088;
const TLS_OUT_OF_INDEXES: u32 = 0xffff_ffff;

// Platform: replaces KERNEL32.DLL TlsAlloc (import)
/// The lowest free thread-local storage index (its value reads 0 on every
/// thread until set), or `TLS_OUT_OF_INDEXES` when all are taken.
fn tls_alloc(e: &mut Engine) -> u32 {
    let used = &mut e.kernel.tls_allocated;
    if used.is_empty() {
        used.resize(TLS_INDEXES, false);
    }
    match used.iter().position(|u| !u) {
        Some(i) => {
            used[i] = true;
            i as u32
        }
        None => TLS_OUT_OF_INDEXES,
    }
}

fn tls_in_use(e: &Engine, index: u32) -> bool {
    e.kernel
        .tls_allocated
        .get(index as usize)
        .copied()
        .unwrap_or(false)
}

// Platform: replaces KERNEL32.DLL TlsFree (import)
/// Frees an index and forgets its value on every thread; 0 for an index
/// that isn't allocated.
fn tls_free(e: &mut Engine, index: u32) -> i32 {
    if !tls_in_use(e, index) {
        return 0;
    }
    e.kernel.tls_allocated[index as usize] = false;
    e.kernel.tls_values.retain(|&(_, i), _| i != index);
    1
}

// Platform: replaces KERNEL32.DLL TlsGetValue (import)
/// The calling thread's value at `index` (0 if never set). The calling
/// thread is `GetCurrentThreadId`'s.
fn tls_get_value(e: &mut Engine, index: u32) -> u32 {
    let thread = get_current_thread_id(e);
    e.kernel
        .tls_values
        .get(&(thread, index))
        .copied()
        .unwrap_or(0)
}

// Platform: replaces KERNEL32.DLL TlsSetValue (import)
/// Stores the calling thread's value at `index`; 0 for an index past the
/// last one Windows provides. As on Windows, the index isn't checked against
/// the ones `TlsAlloc` handed out.
fn tls_set_value(e: &mut Engine, index: u32, value: u32) -> i32 {
    if index as usize >= TLS_INDEXES {
        return 0;
    }
    let thread = get_current_thread_id(e);
    e.kernel.tls_values.insert((thread, index), value);
    1
}

// Platform: replaces KERNEL32.DLL GetStartupInfoA (import)
/// `GetStartupInfoA(info)`: the `STARTUPINFOA` (0x44 bytes) of a GUI process started without
/// inherited handles: `cb` 0x44, `dwFlags` `STARTF_USESHOWWINDOW` (1) with `wShowWindow`
/// `SW_SHOWNORMAL` (1), no `cbReserved2`/`lpReserved2` (no CRT handles passed), the three
/// standard handles -1, everything else 0. Measured on this machine (2026-10-21): a
/// GUI-subsystem program started with PowerShell's `Start-Process` got exactly these, and
/// pointers in `lpReserved`, `lpDesktop` and `lpTitle`, which are not provided here (left
/// null; the CRT's `_ioinit` reads only `cbReserved2` and `lpReserved2`).
fn get_startup_info_a(e: &mut Engine, info: Ptr) {
    let at = info.addr();
    e.mem.write(at, &[0; 0x44]);
    e.mem.set_u32(at, 0x44);
    e.mem.set_u32(at + 0x2c, 1);
    e.mem.set_u16(at + 0x30, 1);
    for offset in [0x38u32, 0x3c, 0x40] {
        e.mem.set_u32(at + offset, 0xffff_ffff);
    }
}

// Platform: replaces KERNEL32.DLL GetStdHandle (import)
/// `GetStdHandle(which)`: 0 (no handle) for `STD_INPUT_HANDLE`, `STD_OUTPUT_HANDLE` and
/// `STD_ERROR_HANDLE` (-10, -11, -12), the last error left as it was: a GUI process has none.
/// Measured on this machine (2026-10-21) in a GUI-subsystem program started with
/// `Start-Process`. Any other value is not provided (stops the run).
fn get_std_handle(_e: &mut Engine, which: u32) -> u32 {
    assert!(
        matches!(which, 0xffff_fff4..=0xffff_fff6),
        "GetStdHandle({which:#x}) is not provided"
    );
    0
}

// Platform: replaces KERNEL32.DLL SetHandleCount (import)
/// `SetHandleCount(count)`: answers `count` (it has no effect on Win32; measured on this
/// machine, 2026-10-21: 32 gave 32).
fn set_handle_count(_e: &mut Engine, count: u32) -> u32 {
    count
}

// Platform: replaces KERNEL32.DLL HeapCreate (import)
/// `HeapCreate(options, initial, maximum)` for a growable heap (`maximum` 0, options 0 or
/// `HEAP_NO_SERIALIZE` 1), as the CRT's `_heap_init` makes `_crtheap`: a handle (Windows
/// answers a nonzero heap handle for these arguments). Only the handle: the CRT's allocations
/// are the engine's heap (`malloc`, `free`, `__calloc_crt` are platform; [`heap_alloc`] hands
/// out the same blocks); the other heap calls on it are not provided (they stop the run). Other options or a fixed
/// size are not provided.
fn heap_create(e: &mut Engine, options: u32, _initial: u32, maximum: u32) -> u32 {
    assert!(
        options <= 1 && maximum == 0,
        "HeapCreate({options:#x}, maximum {maximum:#x}) is not provided"
    );
    e.kernel.insert(KernelObject::Heap)
}

// Platform: replaces KERNEL32.DLL HeapAlloc (import)
/// `HeapAlloc(heap, flags, size)` on the CRT's heap (`_crtheap`, from [`heap_create`]), as the
/// exe's `malloc` body (`00ecd1c7`) asks it with `__active_heap` 1: a block of the platform
/// allocator, the same one `malloc` and `free` use, so `free` and `_msize` take it
/// (`crate::units::crt`). `HEAP_ZERO_MEMORY` (8) clears it. Another heap or other flags are not
/// provided.
fn heap_alloc(e: &mut Engine, heap: u32, flags: u32, size: u32) -> u32 {
    assert!(
        matches!(e.kernel.object(heap), Some(KernelObject::Heap)) && flags & !8 == 0,
        "HeapAlloc(heap {heap:#x}, flags {flags:#x}) is not provided"
    );
    let block = crate::units::crt::crt_alloc(e, size);
    if flags & 8 != 0 {
        e.mem.write(block, &vec![0u8; size as usize]);
    }
    block
}

// Platform: replaces KERNEL32.DLL GetCurrentDirectoryA (import)
/// `GetCurrentDirectoryA(size, buffer)`: with room for the directory and its
/// zero, copies them and returns the length without the zero; otherwise
/// leaves the buffer and returns the size needed, zero included (as Windows
/// does).
fn get_current_directory_a(e: &mut Engine, size: u32, buffer: Ptr) -> u32 {
    let directory = e.kernel.current_directory.clone();
    let needed = directory.len() as u32 + 1;
    if size < needed || buffer.is_null() {
        return needed;
    }
    e.mem.set_cstr(buffer.addr(), &directory);
    directory.len() as u32
}

/// `GetCurrentProcess`'s pseudo handle.
pub const CURRENT_PROCESS: u32 = 0xffff_ffff;

// Platform: replaces KERNEL32.DLL GetCurrentProcess (import)
fn get_current_process(_e: &mut Engine) -> u32 {
    CURRENT_PROCESS
}

// Platform: replaces KERNEL32.DLL GetSystemInfo (import)
/// Fills the 0x24-byte `SYSTEM_INFO` as 32-bit Windows does for this
/// process: x86 (architecture 0), 4 KiB pages, application addresses
/// 0x10000-0x7ffeffff, one mask bit per processor, the machine's number of
/// processors (as Windows reports the real machine; the game's Havok caps it,
/// `hkBaseSystem` at 4, the job thread pool at 5), processor type 586
/// (`PROCESSOR_INTEL_PENTIUM`), 64 KiB allocation granularity, level 6.
fn get_system_info(e: &mut Engine, out: Ptr) {
    let processors = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(32) as u32;
    let p = out.addr();
    e.mem.set_u32(p, 0); // wProcessorArchitecture, wReserved
    e.mem.set_u32(p + 0x04, PAGE_BYTES);
    e.mem.set_u32(p + 0x08, 0x0001_0000);
    e.mem.set_u32(p + 0x0c, 0x7ffe_ffff);
    e.mem.set_u32(
        p + 0x10,
        if processors == 32 {
            u32::MAX
        } else {
            (1 << processors) - 1
        },
    );
    e.mem.set_u32(p + 0x14, processors);
    e.mem.set_u32(p + 0x18, 586);
    e.mem.set_u32(p + 0x1c, ALLOCATION_GRANULARITY);
    e.mem.set_u16(p + 0x20, 6);
    e.mem.set_u16(p + 0x22, 0);
}

// Platform: replaces KERNEL32.DLL SetThreadPriority (import)
/// Accepts any priority for a thread handle or the current thread (the
/// threads do not run, so it has no effect); 0 for another handle.
fn set_thread_priority(e: &mut Engine, handle: u32, _priority: i32) -> i32 {
    (handle == CURRENT_THREAD
        || matches!(
            e.kernel.objects.get(&handle),
            Some(KernelObject::Thread { .. })
        )) as i32
}

/// (DLL, name, platform version) of the imports with a Rust stand-in.
pub fn imports() -> Vec<(&'static str, &'static str, AbiFn)> {
    let k = "KERNEL32.dll";
    #[allow(unused_mut)]
    let mut imports = vec![
        (
            k,
            "InterlockedIncrement",
            entry!(0, interlocked_increment(Ptr) -> i32).1,
        ),
        (
            k,
            "InterlockedDecrement",
            entry!(0, interlocked_decrement(Ptr) -> i32).1,
        ),
        (
            k,
            "InterlockedExchange",
            entry!(0, interlocked_exchange(Ptr, i32) -> i32).1,
        ),
        (
            k,
            "InterlockedExchangeAdd",
            entry!(0, interlocked_exchange_add(Ptr, i32) -> i32).1,
        ),
        (
            k,
            "InterlockedCompareExchange",
            entry!(0, interlocked_compare_exchange(Ptr, i32, i32) -> i32).1,
        ),
        (
            k,
            "EnterCriticalSection",
            entry!(0, enter_critical_section(Ptr)).1,
        ),
        (
            k,
            "LeaveCriticalSection",
            entry!(0, leave_critical_section(Ptr)).1,
        ),
        (
            k,
            "InitializeCriticalSection",
            entry!(0, critical_section_reset(Ptr)).1,
        ),
        (
            k,
            "DeleteCriticalSection",
            entry!(0, critical_section_reset(Ptr)).1,
        ),
        (
            k,
            "InitializeCriticalSectionAndSpinCount",
            entry!(0, initialize_critical_section_and_spin_count(Ptr, u32) -> i32).1,
        ),
        (
            k,
            "TryEnterCriticalSection",
            entry!(0, try_enter_critical_section(Ptr) -> i32).1,
        ),
        (
            k,
            "GetCurrentThreadId",
            entry!(0, get_current_thread_id() -> u32).1,
        ),
        (k, "Sleep", entry!(0, sleep(u32)).1),
        (k, "GetLastError", entry!(0, get_last_error() -> u32).1),
        (k, "SetLastError", entry!(0, set_last_error(u32)).1),
        (k, "GetTickCount", entry!(0, get_tick_count() -> u32).1),
        (
            k,
            "GetPrivateProfileStringA",
            entry!(0, get_private_profile_string_a(Ptr, Ptr, Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            k,
            "WritePrivateProfileStringA",
            entry!(0, write_private_profile_string_a(Ptr, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetAsyncKeyState",
            entry!(0, dinput::get_async_key_state(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetActiveWindow",
            entry!(0, get_active_window() -> u32).1,
        ),
        (
            "USER32.dll",
            "SwapMouseButton",
            entry!(0, swap_mouse_button(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetDoubleClickTime",
            entry!(0, get_double_click_time() -> u32).1,
        ),
        (
            "USER32.dll",
            "GetSystemMetrics",
            entry!(0, get_system_metrics(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "EnumDisplayDevicesA",
            entry!(0, d3d9::enum_display_devices_a(Ptr, u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "LoadLibraryA",
            entry!(0, loader::load_library_a(Ptr) -> u32).1,
        ),
        (k, "FreeLibrary", entry!(0, loader::free_library(u32) -> u32).1),
        (
            "d3dx9_38.dll",
            "D3DXMatrixMultiply",
            entry!(0, d3dx::matrix_multiply(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixMultiplyTranspose",
            entry!(0, d3dx::matrix_multiply_transpose(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixRotationYawPitchRoll",
            entry!(0, d3dx::matrix_rotation_yaw_pitch_roll(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixTranspose",
            entry!(0, d3dx::matrix_transpose(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3TransformNormal",
            entry!(0, d3dx::vec3_transform_normal(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3Normalize",
            entry!(0, d3dx::vec3_normalize(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec3TransformCoord",
            entry!(0, d3dx::vec3_transform_coord(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXMatrixInverse",
            entry!(0, d3dx::matrix_inverse(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXPlaneNormalize",
            entry!(0, d3dx::plane_normalize(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXPlaneTransform",
            entry!(0, d3dx::plane_transform(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXVec4Transform",
            entry!(0, d3dx::vec4_transform_import(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetImageInfoFromFileInMemory",
            entry!(0, d3dx::get_image_info_from_file_in_memory(Ptr, u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateTextureFromFileInMemory",
            entry!(0, d3dx::create_texture_from_file_in_memory(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateCubeTextureFromFileInMemory",
            entry!(0, d3dx::create_cube_texture_from_file_in_memory(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            "d3d9.dll",
            "D3DPERF_SetOptions",
            entry!(0, d3dperf_set_options(u32)).1,
        ),
        (
            k,
            "CreateMutexA",
            entry!(0, window::create_mutex_a(u32, u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "MessageBoxA",
            entry!(0, window::message_box_a(u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (k, "ExitProcess", entry!(0, window::exit_process(u32)).1),
        (
            "SHELL32.dll",
            "ShellExecuteA",
            entry!(0, window::shell_execute_a(u32, u32, Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "IsDebuggerPresent",
            entry!(0, window::is_debugger_present() -> u32).1,
        ),
        (
            "USER32.dll",
            "FindWindowA",
            entry!(0, window::find_window_a(u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetForegroundWindow",
            entry!(0, window::set_foreground_window(u32) -> u32).1,
        ),
        (
            k,
            "GetCommandLineW",
            entry!(0, window::get_command_line_w() -> u32).1,
        ),
        (
            "SHELL32.dll",
            "CommandLineToArgvW",
            entry!(0, window::command_line_to_argv_w(Ptr, Ptr) -> u32).1,
        ),
        (k, "LocalFree", entry!(0, window::local_free(u32) -> u32).1),
        (
            "USER32.dll",
            "LoadIconA",
            entry!(0, window::load_icon_a(u32, u32) -> u32).1,
        ),
        (
            "GDI32.dll",
            "GetStockObject",
            entry!(0, window::get_stock_object(u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "RegisterClassA",
            entry!(0, window::register_class_a(Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "AdjustWindowRect",
            entry!(0, window::adjust_window_rect(Ptr, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "AdjustWindowRectEx",
            entry!(0, window::adjust_window_rect_ex(Ptr, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "CreateWindowExA",
            entry!(0, window::create_window_ex_a(u32, u32, Ptr, u32, i32, i32, i32, i32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetWindowLongA",
            entry!(0, window::get_window_long_a(u32, i32) -> u32).1,
        ),
        (
            "USER32.dll",
            "GetClassLongA",
            entry!(0, window::get_class_long_a(u32, i32) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetWindowTextA",
            entry!(0, window::set_window_text_a(u32, Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "SetWindowPos",
            entry!(0, window::set_window_pos(u32, u32, i32, i32, i32, i32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "PeekMessageA",
            entry!(0, window::peek_message_a(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (
            "USER32.dll",
            "TranslateMessage",
            entry!(0, window::translate_message(Ptr) -> u32).1,
        ),
        (
            "USER32.dll",
            "DispatchMessageA",
            entry!(0, window::dispatch_message_a(Ptr) -> u32).1,
        ),
        (
            k,
            "GetPrivateProfileIntA",
            entry!(0, get_private_profile_int_a(Ptr, Ptr, u32, Ptr) -> u32).1,
        ),
        (
            k,
            "GetFileAttributesA",
            entry!(0, files::get_file_attributes_a(Ptr) -> u32).1,
        ),
        (
            k,
            "CreateDirectoryA",
            entry!(0, files::create_directory_a(Ptr, u32) -> u32).1,
        ),
        (k, "DeleteFileA", entry!(0, files::delete_file_a(Ptr) -> u32).1),
        (
            k,
            "CopyFileA",
            entry!(0, files::copy_file_a(Ptr, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "MoveFileA",
            entry!(0, files::move_file_a(Ptr, Ptr) -> u32).1,
        ),
        (
            "SHELL32.dll",
            "SHGetFolderPathA",
            entry!(0, files::sh_get_folder_path_a(u32, u32, u32, u32, Ptr) -> u32).1,
        ),
        (
            "ADVAPI32.dll",
            "GetUserNameA",
            entry!(0, window::get_user_name_a(Ptr, Ptr) -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_Init",
            entry!(0, window::steam_api_init() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_RegisterCallback",
            entry!(0, window::steam_api_register_callback(Ptr, u32)).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_UnregisterCallback",
            entry!(0, window::steam_api_unregister_callback(Ptr)).1,
        ),
        // The platform5 imports (slot a04): begin.
        (
            "steam_api.dll",
            "SteamAPI_IsSteamRunning",
            entry!(0, window::steam_api_is_steam_running() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUserStats",
            entry!(0, window::steam_user_stats() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamFriends",
            entry!(0, window::steam_friends() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUtils",
            entry!(0, window::steam_utils() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamUser",
            entry!(0, window::steam_user() -> u32).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_RunCallbacks",
            entry!(0, window::steam_api_run_callbacks()).1,
        ),
        (
            "steam_api.dll",
            "SteamAPI_Shutdown",
            entry!(0, window::steam_api_shutdown()).1,
        ),
        (
            "steam_api.dll",
            "SteamApps",
            entry!(0, window::steam_apps() -> u32).1,
        ),
        (
            k,
            "GetModuleFileNameA",
            entry!(0, process::get_module_file_name_a(u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "GetCommandLineA",
            entry!(0, process::get_command_line_a() -> u32).1,
        ),
        (
            k,
            "GetEnvironmentStringsW",
            entry!(0, process::get_environment_strings_w() -> u32).1,
        ),
        (
            k,
            "FreeEnvironmentStringsW",
            entry!(0, process::free_environment_strings_w(u32) -> u32).1,
        ),
        (
            k,
            "GetCurrentProcessId",
            entry!(0, process::get_current_process_id() -> u32).1,
        ),
        (
            k,
            "HeapFree",
            entry!(0, process::heap_free(u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "HeapReAlloc",
            entry!(0, process::heap_re_alloc(u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "CreateFileW",
            entry!(0, files::create_file_w(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "GetFileSizeEx",
            entry!(0, files::get_file_size_ex(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "SystemTimeToTzSpecificLocalTime",
            entry!(0, time::system_time_to_tz_specific_local_time(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "SetUnhandledExceptionFilter",
            entry!(0, process::set_unhandled_exception_filter(u32) -> u32).1,
        ),
        (
            k,
            "GetStringTypeW",
            entry!(0, nls::get_string_type_w(u32, Ptr, i32, Ptr) -> u32).1,
        ),
        // The platform5 imports: end.
        (
            "ADVAPI32.dll",
            "RegOpenKeyExA",
            entry!(0, reg_open_key_ex_a(u32, Ptr, u32, u32, Ptr) -> u32).1,
        ),
        (
            "ADVAPI32.dll",
            "RegQueryValueExA",
            entry!(0, reg_query_value_ex_a(u32, Ptr, u32, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "ADVAPI32.dll",
            "RegCloseKey",
            entry!(0, reg_close_key(u32) -> u32).1,
        ),
        (
            k,
            "GetModuleHandleW",
            entry!(0, loader::get_module_handle_w(Ptr) -> u32).1,
        ),
        (
            k,
            "GetModuleHandleA",
            entry!(0, loader::get_module_handle_a(Ptr) -> u32).1,
        ),
        (
            k,
            "GetProcAddress",
            entry!(0, loader::get_proc_address(u32, u32) -> u32).1,
        ),
        (
            "DSOUND.dll",
            "#11",
            entry!(0, dsound::direct_sound_create8(u32, Ptr, u32) -> u32).1,
        ),
        (
            "ole32.dll",
            "CoInitializeEx",
            entry!(0, co_initialize_ex(u32, u32) -> u32).1,
        ),
        ("ole32.dll", "CoUninitialize", entry!(0, co_uninitialize()).1),
        (
            "ole32.dll",
            "CoTaskMemAlloc",
            entry!(0, co_task_mem_alloc(u32) -> u32).1,
        ),
        (
            "ole32.dll",
            "CoTaskMemFree",
            entry!(0, co_task_mem_free(Ptr)).1,
        ),
        (
            "ole32.dll",
            "CoFreeUnusedLibraries",
            entry!(0, co_free_unused_libraries()).1,
        ),
        (
            "ole32.dll",
            "CoInitialize",
            entry!(0, dsound::co_initialize(u32) -> u32).1,
        ),
        (
            "DINPUT8.dll",
            "DirectInput8Create",
            entry!(0, dinput::direct_input8_create(u32, u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            "XINPUT1_3.dll",
            "#2",
            entry!(0, xinput_get_state(u32, Ptr) -> u32).1,
        ),
        (
            "XINPUT1_3.dll",
            "#3",
            entry!(0, xinput_set_state(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "RaiseException",
            entry!(0, raise_exception(u32, u32, u32, Ptr)).1,
        ),
        ("WINMM.dll", "timeGetTime", entry!(0, tick_count() -> u32).1),
        (
            k,
            "QueryPerformanceFrequency",
            entry!(0, query_performance_frequency(Ptr) -> i32).1,
        ),
        (
            k,
            "QueryPerformanceCounter",
            entry!(0, query_performance_counter(Ptr) -> i32).1,
        ),
        (
            k,
            "VirtualAlloc",
            entry!(0, virtual_alloc(u32, u32, u32, u32) -> u32).1,
        ),
        (
            k,
            "VirtualFree",
            entry!(0, virtual_free(u32, u32, u32) -> i32).1,
        ),
        (
            k,
            "CreateEventA",
            entry!(0, create_event_a(u32, i32, i32, Ptr) -> u32).1,
        ),
        (k, "SetEvent", entry!(0, set_event(u32) -> i32).1),
        (k, "ResetEvent", entry!(0, reset_event(u32) -> i32).1),
        (
            k,
            "CreateSemaphoreA",
            entry!(0, create_semaphore_a(u32, i32, i32, Ptr) -> u32).1,
        ),
        (
            k,
            "ReleaseSemaphore",
            entry!(0, release_semaphore(u32, i32, Ptr) -> i32).1,
        ),
        (
            k,
            "WaitForSingleObject",
            entry!(0, wait_for_single_object(u32, u32) -> u32).1,
        ),
        (k, "CloseHandle", entry!(0, close_handle(u32) -> i32).1),
        (
            k,
            "CreateFileA",
            entry!(0, files::create_file_a(Ptr, u32, u32, u32, u32, u32, u32) -> u32).1,
        ),
        (k, "GetFileType", entry!(0, files::get_file_type(u32) -> u32).1),
        (
            k,
            "SetFilePointer",
            entry!(0, files::set_file_pointer(u32, u32, Ptr, u32) -> u32).1,
        ),
        (
            k,
            "WriteFile",
            entry!(0, files::write_file(u32, Ptr, u32, Ptr, u32) -> i32).1,
        ),
        (
            k,
            "ReadFile",
            entry!(0, files::read_file(u32, Ptr, u32, Ptr, u32) -> i32).1,
        ),
        (
            k,
            "FindFirstFileA",
            entry!(0, files::find_first_file_a(Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "FindNextFileA",
            entry!(0, files::find_next_file_a(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "CompareFileTime",
            entry!(0, files::compare_file_time(Ptr, Ptr) -> i32).1,
        ),
        (k, "FindClose", entry!(0, files::find_close(u32) -> u32).1),
        (
            k,
            "GetFullPathNameA",
            entry!(0, files::get_full_path_name_a(Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "GetCurrentThread",
            entry!(0, get_current_thread() -> u32).1,
        ),
        (
            k,
            "CreateThread",
            entry!(0, create_thread(u32, u32, u32, u32, u32, Ptr) -> u32).1,
        ),
        (k, "ResumeThread", entry!(0, resume_thread(u32) -> u32).1),
        (k, "lstrcpyA", entry!(0, lstrcpy_a(Ptr, Ptr) -> u32).1),
        (k, "lstrcatA", entry!(0, lstrcat_a(Ptr, Ptr) -> u32).1),
        (k, "SuspendThread", entry!(0, suspend_thread(u32) -> u32).1),
        (k, "ExitThread", entry!(0, exit_thread(u32)).1),
        (
            k,
            "GetExitCodeThread",
            entry!(0, get_exit_code_thread(u32, Ptr) -> u32).1,
        ),
        (
            k,
            "WaitForMultipleObjects",
            entry!(0, wait_for_multiple_objects(u32, Ptr, i32, u32) -> u32).1,
        ),
        (
            k,
            "SetThreadPriority",
            entry!(0, set_thread_priority(u32, i32) -> i32).1,
        ),
        (k, "TlsAlloc", entry!(0, tls_alloc() -> u32).1),
        (k, "TlsFree", entry!(0, tls_free(u32) -> i32).1),
        (k, "TlsGetValue", entry!(0, tls_get_value(u32) -> u32).1),
        (
            k,
            "TlsSetValue",
            entry!(0, tls_set_value(u32, u32) -> i32).1,
        ),
        (
            k,
            "GetCurrentProcess",
            entry!(0, get_current_process() -> u32).1,
        ),
        (k, "GetSystemInfo", entry!(0, get_system_info(Ptr)).1),
        (
            k,
            "GetCurrentDirectoryA",
            entry!(0, get_current_directory_a(u32, Ptr) -> u32).1,
        ),
        (k, "GetStartupInfoA", entry!(0, get_startup_info_a(Ptr)).1),
        (k, "GetStdHandle", entry!(0, get_std_handle(u32) -> u32).1),
        (k, "SetHandleCount", entry!(0, set_handle_count(u32) -> u32).1),
        (k, "HeapCreate", entry!(0, heap_create(u32, u32, u32) -> u32).1),
        (k, "HeapAlloc", entry!(0, heap_alloc(u32, u32, u32) -> u32).1),
        (
            k,
            "GetSystemTimeAsFileTime",
            entry!(0, time::get_system_time_as_file_time(Ptr)).1,
        ),
        (
            k,
            "GetTimeZoneInformation",
            entry!(0, time::get_time_zone_information(Ptr) -> u32).1,
        ),
        (k, "GetACP", entry!(0, d3d9::get_acp() -> u32).1),
        (k, "GetOEMCP", entry!(0, d3d9::get_oemcp() -> u32).1),
        (
            k,
            "IsValidCodePage",
            entry!(0, d3d9::is_valid_code_page(u32) -> u32).1,
        ),
        (k, "GetCPInfo", entry!(0, d3d9::get_cp_info(u32, Ptr) -> u32).1),
        (
            k,
            "FileTimeToLocalFileTime",
            entry!(0, time::file_time_to_local_file_time(Ptr, Ptr) -> u32).1,
        ),
        (k, "GetLocalTime", entry!(0, time::get_local_time(Ptr)).1),
        (
            k,
            "FileTimeToSystemTime",
            entry!(0, time::file_time_to_system_time(Ptr, Ptr) -> u32).1,
        ),
        (
            k,
            "WideCharToMultiByte",
            entry!(0, time::wide_char_to_multi_byte(u32, u32, Ptr, i32, Ptr, i32, Ptr, Ptr) -> u32).1,
        ),
        // The Bink imports (`bink`, slot a11): begin.
        (
            "binkw32.dll",
            "_BinkSetSoundSystem@8",
            entry!(0, bink::set_sound_system(u32, u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkOpenDirectSound@4",
            entry!(0, bink::open_direct_sound(u32) -> u32).1,
        ),
        (
            "binkw32.dll",
            "_BinkOpen@8",
            entry!(0, bink::open(Ptr, u32) -> u32).1,
        ),
        (
            "binkw32.dll",
            "_BinkClose@4",
            entry!(0, bink::close(u32)).1,
        ),
        (
            "binkw32.dll",
            "_BinkDoFrame@4",
            entry!(0, bink::do_frame(u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkNextFrame@4",
            entry!(0, bink::next_frame(u32)).1,
        ),
        (
            "binkw32.dll",
            "_BinkWait@4",
            entry!(0, bink::wait(u32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkPause@8",
            entry!(0, bink::pause(u32, i32) -> i32).1,
        ),
        (
            "binkw32.dll",
            "_BinkCopyToBufferRect@44",
            entry!(0, bink::copy_to_buffer_rect(u32, Ptr, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> i32).1,
        ),
        // The Bink imports: end.
        // The Ogg Vorbis imports (`vorbisfile`, slot a11): begin.
        (
            "libvorbisfile.dll",
            "ov_open_callbacks",
            entry!(0, vorbisfile::ov_open_callbacks(u32, Ptr, u32, i32, u32, u32, u32, u32) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_info",
            entry!(0, vorbisfile::ov_info(Ptr, i32) -> u32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_seekable",
            entry!(0, vorbisfile::ov_seekable(Ptr) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_pcm_total",
            entry!(0, vorbisfile::ov_pcm_total(Ptr, i32) -> u64).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_read",
            entry!(0, vorbisfile::ov_read(Ptr, u32, i32, i32, i32, i32, Ptr) -> i32).1,
        ),
        (
            "libvorbisfile.dll",
            "ov_clear",
            entry!(0, vorbisfile::ov_clear(Ptr) -> i32).1,
        ),
        // The Ogg Vorbis imports: end.
        // The sound imports (`mmio`, `dsound`, slot a11): begin.
        (
            "WINMM.dll",
            "mmioOpenA",
            entry!(0, mmio::mmio_open_a(Ptr, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioClose",
            entry!(0, mmio::mmio_close(u32, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioRead",
            entry!(0, mmio::mmio_read(u32, Ptr, i32) -> i32).1,
        ),
        (
            "WINMM.dll",
            "mmioDescend",
            entry!(0, mmio::mmio_descend(u32, Ptr, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioAscend",
            entry!(0, mmio::mmio_ascend(u32, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioGetInfo",
            entry!(0, mmio::mmio_get_info(u32, Ptr, u32) -> u32).1,
        ),
        (
            "WINMM.dll",
            "mmioAdvance",
            entry!(0, mmio::mmio_advance(u32, Ptr, u32) -> u32).1,
        ),
        (
            "ole32.dll",
            "CoCreateInstance",
            entry!(0, dsound::co_create_instance(Ptr, u32, u32, Ptr, Ptr) -> u32).1,
        ),
        // The sound imports: end.
    ];
    // The shader compiler through the player's own d3dx9_38.dll (ADR-0008, Windows only).
    #[cfg(windows)]
    imports.extend([
        (
            "d3dx9_38.dll",
            "D3DXCompileShader",
            entry!(0, d3dxreal::compile_shader(Ptr, u32, Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetShaderConstantTable",
            entry!(0, d3dxreal::get_shader_constant_table(Ptr, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetVertexShaderProfile",
            entry!(0, d3dxreal::get_vertex_shader_profile(Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXGetPixelShaderProfile",
            entry!(0, d3dxreal::get_pixel_shader_profile(Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXCreateBuffer",
            entry!(0, d3dxreal::create_buffer(u32, Ptr) -> u32).1,
        ),
        (
            "d3dx9_38.dll",
            "D3DXAssembleShader",
            entry!(0, d3dxreal::assemble_shader(Ptr, u32, Ptr, Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
    ]);
    imports
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No XInput controller: reading and rumbling (the shot's vibration, `00a255b0`) both answer
    /// `ERROR_DEVICE_NOT_CONNECTED` for every user index, without touching the game's memory.
    #[test]
    fn xinput_answers_no_controller_for_reads_and_rumble() {
        let mut e = Engine::new();
        for user in 0..4 {
            assert_eq!(
                xinput_get_state(&mut e, user, Ptr::new(0)),
                ERROR_DEVICE_NOT_CONNECTED
            );
            assert_eq!(
                xinput_set_state(&mut e, user, Ptr::new(0)),
                ERROR_DEVICE_NOT_CONNECTED
            );
        }
    }

    #[test]
    fn a_memory_file_reads_and_seeks_what_was_supplied() {
        let mut e = Engine::new();
        // The vtable is in the exe's data; its seek slot (+0x14).
        e.put_vtable(BSFILE_VTABLE, &[0, 0, 0, 0, 0, 0x00b0_0590]);
        let f = open_memory_file(&mut e, "Test.esm", 0x100);
        assert_eq!(e.mem.u32(f), BSFILE_VTABLE);
        assert_eq!(e.mem.u8(f + 0x2c), 1);
        supply_file_bytes(&mut e, f, 0x10, b"abcdefgh");
        supply_file_bytes(&mut e, f, 0x18, b"ijkl");
        let buffer = e.mem.alloc(0x20);
        // Seek through the vtable's slot +0x14, as TESFile does.
        e.vcall(f, 0x14, &[0x12, 0]);
        assert_eq!(e.mem.u32(f + 0x150), 0x12);
        // Across two supplied pieces.
        assert_eq!(e.call(0x00b00650, &[f, buffer, 8]).u32(), 8);
        assert_eq!(e.mem.bytes(buffer, 8), b"cdefghij");
        assert_eq!(e.mem.u32(f + 0x150), 0x1a);
        assert_eq!(e.mem.u32(f + 0x38), 0x1a);
        // Relative seek.
        e.vcall(f, 0x14, &[(-2i32) as u32, 1]);
        assert_eq!(e.mem.u32(f + 0x150), 0x18);
        // At the end of the file, fewer.
        supply_file_bytes(&mut e, f, 0xfc, b"wxyz");
        e.vcall(f, 0x14, &[0xfe, 0]);
        assert_eq!(e.call(0x00b00650, &[f, buffer, 8]).u32(), 2);
        assert_eq!(e.mem.bytes(buffer, 2), b"yz");
    }

    #[test]
    #[should_panic(expected = "not supplied")]
    fn bytes_not_supplied_stop() {
        let mut e = Engine::new();
        e.put_vtable(BSFILE_VTABLE, &[0, 0, 0, 0, 0, 0x00b0_0590]);
        let f = open_memory_file(&mut e, "Test.esm", 0x100);
        supply_file_bytes(&mut e, f, 0x10, b"abcd");
        let buffer = e.mem.alloc(8);
        e.vcall(f, 0x14, &[0x12, 0]);
        e.call(0x00b00650, &[f, buffer, 4]);
    }

    #[test]
    fn allocate_then_delete_through_the_game_calls() {
        let mut e = Engine::new();
        let mm = MEMORY_MANAGER;
        let p: Ptr = e.call_as(0x00aa_3e40, &args![mm, 24u32]);
        assert_eq!(e.mem.block_size(p.addr()), Some(24));
        e.call(0x0040_1030, &args![p]);
        assert_eq!(e.mem.block_size(p.addr()), None);
    }

    #[test]
    fn system_utility_is_built_once_and_registered_for_exit() {
        let mut e = Engine::new();
        e.mem.map(SYSTEM_UTILITY, 0x100);
        e.mem.map(SYSTEM_UTILITY_GUARD, 4);
        e.mem.write(SYSTEM_UTILITY + 4, &[0xaa; 0x80]);
        e.mem.set_u32(SYSTEM_UTILITY + 0x84, 7);
        e.mem.set_u8(SYSTEM_UTILITY + 0x95, 9);
        e.mem.set_u32(SYSTEM_UTILITY_GUARD, 0x10); // other guard bits are kept
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let p: Ptr = e.call_as(0x00af_2640, &args![]);
        assert_eq!(p.addr(), SYSTEM_UTILITY);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY_GUARD), 0x11);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY), SYSTEM_UTILITY_VTABLE);
        assert_eq!(e.mem.bytes(SYSTEM_UTILITY + 4, 0x80), vec![0u8; 0x80]);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY + 0x84), 0);
        assert_eq!(e.mem.u8(SYSTEM_UTILITY + 0x95), 0);
        let log = e.call_log.clone().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == ATEXIT).count(), 1);

        // The second call neither rebuilds nor registers again.
        e.mem.set_u32(SYSTEM_UTILITY + 0x84, 5);
        e.call_log = Some(vec![]);
        let q: Ptr = e.call_as(0x00af_2640, &args![]);
        assert_eq!(q.addr(), SYSTEM_UTILITY);
        assert_eq!(e.mem.u32(SYSTEM_UTILITY + 0x84), 5);
        assert!(e.call_log.unwrap().iter().all(|c| c.0 != ATEXIT));
    }

    #[test]
    fn save_data_utility_is_built_once_and_registered_for_exit() {
        let mut e = Engine::new();
        e.mem.map(SAVE_DATA_UTILITY, 0x48);
        e.mem.map(SAVE_DATA_UTILITY_GUARD, 4);
        e.mem.write(SAVE_DATA_UTILITY, &[0xaa; 0x48]);
        e.mem.set_u32(SAVE_DATA_UTILITY_GUARD, 0x10); // other guard bits are kept
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let p: Ptr = e.call_as(0x00af_2e50, &args![]);
        assert_eq!(p.addr(), SAVE_DATA_UTILITY);
        assert_eq!(e.mem.u32(SAVE_DATA_UTILITY_GUARD), 0x11);
        let u = SAVE_DATA_UTILITY;
        assert_eq!(e.mem.u32(u), SAVE_DATA_UTILITY_VTABLE);
        assert_eq!(e.mem.u8(u + 4), 0);
        assert_eq!(e.mem.u32(u + 8), 0);
        assert_eq!(e.mem.u8(u + 0xc), 0);
        assert_eq!(e.mem.u32(u + 0x2c), 0);
        assert_eq!(e.mem.u32(u + 0x44), 0);
        // Bytes the constructor does not write keep their value.
        assert_eq!(e.mem.u8(u + 5), 0xaa);
        assert_eq!(e.mem.u8(u + 0xd), 0xaa);
        assert_eq!(e.mem.u32(u + 0x30), 0xaaaa_aaaa);
        let log = e.call_log.clone().unwrap();
        let exits: Vec<_> = log.iter().filter(|c| c.0 == ATEXIT).collect();
        assert_eq!(exits.len(), 1);
        assert_eq!(exits[0].1, vec![SAVE_DATA_UTILITY_ATEXIT]);

        // The second call neither rebuilds nor registers again.
        e.mem.set_u32(u + 0x44, 5);
        e.call_log = Some(vec![]);
        let q: Ptr = e.call_as(0x00af_2e50, &args![]);
        assert_eq!(q.addr(), SAVE_DATA_UTILITY);
        assert_eq!(e.mem.u32(u + 0x44), 5);
        assert!(e.call_log.unwrap().iter().all(|c| c.0 != ATEXIT));
    }

    #[test]
    fn save_data_utility_destructor_restores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, 0x1234);
        e.mem.set_u32(this + 8, 0x77);
        e.call(0x00af_2f20, &args![this]);
        assert_eq!(e.mem.u32(this), SAVE_DATA_UTILITY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x77);
    }

    // Import slots of FalloutNV.exe 1.4.0.525 (Ghidra `PTR_<name>_<slot>`).
    const CLOSE_HANDLE: u32 = 0x00fd_f07c;
    const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
    const CREATE_EVENT_A: u32 = 0x00fd_f0cc;
    const SET_EVENT: u32 = 0x00fd_f0d0;
    const VIRTUAL_FREE: u32 = 0x00fd_f10c;
    const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
    const SET_THREAD_PRIORITY: u32 = 0x00fd_f11c;
    const GET_CURRENT_THREAD: u32 = 0x00fd_f120;
    const CREATE_THREAD: u32 = 0x00fd_f124;
    const RESUME_THREAD: u32 = 0x00fd_f158;
    // Stand-in slots for the tests (any free address works: registration is by name).
    const SLEEP: u32 = 0x00ab_f000;
    const WAIT_FOR_MULTIPLE_OBJECTS: u32 = 0x00ab_f004;
    const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
    const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
    const GET_EXIT_CODE_THREAD: u32 = 0x00fd_f168;
    const RESET_EVENT: u32 = 0x00fd_f1cc;

    /// An engine with the kernel-object and virtual-memory stand-ins
    /// registered at their import slots, as `Engine::with_exe` does.
    fn kernel_engine() -> Engine {
        let mut e = Engine::new();
        let slots = [
            (CLOSE_HANDLE, "CloseHandle"),
            (CREATE_SEMAPHORE_A, "CreateSemaphoreA"),
            (CREATE_EVENT_A, "CreateEventA"),
            (SET_EVENT, "SetEvent"),
            (VIRTUAL_FREE, "VirtualFree"),
            (VIRTUAL_ALLOC, "VirtualAlloc"),
            (SET_THREAD_PRIORITY, "SetThreadPriority"),
            (GET_CURRENT_THREAD, "GetCurrentThread"),
            (CREATE_THREAD, "CreateThread"),
            (RESUME_THREAD, "ResumeThread"),
            (RELEASE_SEMAPHORE, "ReleaseSemaphore"),
            (WAIT_FOR_SINGLE_OBJECT, "WaitForSingleObject"),
            (RESET_EVENT, "ResetEvent"),
            (SLEEP, "Sleep"),
            (WAIT_FOR_MULTIPLE_OBJECTS, "WaitForMultipleObjects"),
            (GET_EXIT_CODE_THREAD, "GetExitCodeThread"),
        ];
        let imports: Vec<crate::exe::Import> = slots
            .iter()
            .map(|&(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            })
            .collect();
        assert_eq!(e.register_imports(&imports), slots.len());
        e
    }

    fn call(e: &mut Engine, slot: u32, a: &[u32]) -> u32 {
        e.call(slot, a).u32()
    }

    #[test]
    fn thread_local_storage_allocates_stores_and_frees() {
        let mut e = Engine::new();
        let a = tls_alloc(&mut e);
        let b = tls_alloc(&mut e);
        assert_eq!((a, b), (0, 1));
        assert_eq!(tls_get_value(&mut e, b), 0, "unset reads 0");
        assert_eq!(tls_set_value(&mut e, b, 0x1234), 1);
        assert_eq!(tls_get_value(&mut e, b), 0x1234);
        assert_eq!(tls_set_value(&mut e, 2000, 1), 0, "past the last index");
        assert_eq!(tls_free(&mut e, a), 1);
        assert_eq!(tls_free(&mut e, a), 0, "already free");
        // The freed index is handed out again, its value gone.
        assert_eq!(tls_alloc(&mut e), 0);
        assert_eq!(tls_get_value(&mut e, 0), 0);
        assert_eq!(tls_get_value(&mut e, b), 0x1234);
    }

    #[test]
    fn the_current_directory_is_copied_when_it_fits() {
        let mut e = Engine::new();
        e.kernel.current_directory = b"C:\\Games\\FNV".to_vec();
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u8(buffer, 0x55);
        // Too small: the size needed, the buffer untouched.
        assert_eq!(get_current_directory_a(&mut e, 5, Ptr::new(buffer)), 13);
        assert_eq!(e.mem.u8(buffer), 0x55);
        assert_eq!(get_current_directory_a(&mut e, 0x20, Ptr::new(buffer)), 12);
        assert_eq!(e.mem.cstr(buffer), b"C:\\Games\\FNV");
    }

    #[test]
    fn system_info_reports_x86_pages_and_processors() {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        get_system_info(&mut e, Ptr::new(0x1000));
        assert_eq!(e.mem.u32(0x1004), 0x1000);
        let n = e.mem.u32(0x1014);
        assert!(n >= 1);
        assert_eq!(e.mem.u32(0x1010).count_ones(), n);
        assert_eq!(e.mem.u32(0x1018), 586);
        assert_eq!(e.mem.u32(0x101c), 0x1_0000);
        assert_eq!(get_current_process(&mut e), 0xffff_ffff);
    }

    #[test]
    fn handles_are_small_distinct_and_closed_once() {
        let mut e = kernel_engine();
        let a = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        let b = call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        assert_eq!((a, b), (4, 8));
        assert_eq!(
            e.kernel.object(b),
            Some(&KernelObject::Semaphore {
                count: 0,
                maximum: 1
            })
        );
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[a]), 1);
        assert_eq!(e.kernel.object(a), None);
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[a]), 0, "already closed");
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[a, 0]), WAIT_FAILED);
        assert_eq!(call(&mut e, SET_EVENT, &[a]), 0);
        // Handles are not reused.
        assert_eq!(call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]), 12);
    }

    #[test]
    fn auto_reset_event_is_consumed_by_a_wait_manual_reset_is_not() {
        let mut e = kernel_engine();
        let auto = call(&mut e, CREATE_EVENT_A, &[0, 0, 1, 0]);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, INFINITE]), 0);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, 100]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut e, SET_EVENT, &[auto]), 1);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[auto, 0]), 0);

        let manual = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut e, SET_EVENT, &[manual]), 1);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, INFINITE]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, INFINITE]), 0);
        assert_eq!(call(&mut e, RESET_EVENT, &[manual]), 1);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[manual, 0]),
            WAIT_TIMEOUT
        );
        // Not an event.
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 1, 1, 0]);
        assert_eq!(call(&mut e, RESET_EVENT, &[s]), 0);
    }

    #[test]
    fn semaphore_counts_waits_and_releases() {
        let mut e = kernel_engine();
        e.mem.map(0x1000, 0x1000);
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 1, 2, 0]);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, INFINITE]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), WAIT_TIMEOUT);
        e.mem.set_i32(0x1000, -5);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 2, 0x1000]), 1);
        assert_eq!(e.mem.i32(0x1000), 0, "previous count");
        // Past the maximum, or a count that is not positive: no change.
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 1, 0]), 0);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[s, 0, 0]), 0);
        assert_eq!(
            e.kernel.object(s),
            Some(&KernelObject::Semaphore {
                count: 2,
                maximum: 2
            })
        );
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), 0);
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, 0]), WAIT_TIMEOUT);
        // Counts Windows rejects.
        assert_eq!(call(&mut e, CREATE_SEMAPHORE_A, &[0, 3, 2, 0]), 0);
        assert_eq!(call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 0, 0]), 0);
        // Not a semaphore.
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]);
        assert_eq!(call(&mut e, RELEASE_SEMAPHORE, &[ev, 1, 0]), 0);
    }

    #[test]
    #[should_panic(
        expected = "WaitForSingleObject(handle 0x4, INFINITE) would block forever on a semaphore count 0 (maximum 1)"
    )]
    fn infinite_wait_on_an_empty_semaphore_stops() {
        let mut e = kernel_engine();
        let s = call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[s, INFINITE]);
    }

    #[test]
    #[should_panic(
        expected = "WaitForSingleObject(handle 0x4, INFINITE) would block forever on a manual-reset event not signalled"
    )]
    fn infinite_wait_on_an_unsignalled_event_stops() {
        let mut e = kernel_engine();
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[ev, INFINITE]);
    }

    #[test]
    #[should_panic(expected = "named kernel objects are not provided")]
    fn named_events_stop() {
        let mut e = kernel_engine();
        call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0x0100_0000]);
    }

    /// The main window's handle is an open object that nothing waits on.
    #[test]
    fn the_main_window_is_a_handle() {
        let mut e = kernel_engine();
        let window = create_window(&mut e);
        assert_ne!(window, 0);
        assert_eq!(e.kernel.object(window), Some(&KernelObject::Window));
        assert!(!signalled(&e, window));
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[window, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[window]), 1);
    }

    #[test]
    fn suspended_thread_is_recorded_prioritized_and_closed() {
        let mut e = kernel_engine();
        e.mem.map(0x1000, 0x1000);
        assert_eq!(call(&mut e, GET_CURRENT_THREAD, &[]), CURRENT_THREAD);
        let t = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0x1_0000, 0x00aa_64d0, 0x4444, CREATE_SUSPENDED, 0x1000],
        );
        assert_eq!(t, 4);
        assert_eq!(e.mem.u32(0x1000), MAIN_THREAD_ID + 1);
        assert_eq!(
            e.kernel.object(t),
            Some(&KernelObject::Thread {
                start: 0x00aa_64d0,
                parameter: 0x4444,
                id: MAIN_THREAD_ID + 1,
                suspend_count: 1,
                x87_control_word: 0x027f,
            })
        );
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[t, -1i32 as u32]), 1);
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[CURRENT_THREAD, 2]), 1);
        assert_eq!(call(&mut e, SET_THREAD_PRIORITY, &[0x99, 0]), 0);
        assert_eq!(call(&mut e, RESUME_THREAD, &[CURRENT_THREAD]), 0);
        assert_eq!(call(&mut e, RESUME_THREAD, &[0x99]), 0xffff_ffff);
        // A suspended thread does not run, so it is not signalled.
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, 10]), WAIT_TIMEOUT);
        assert_eq!(call(&mut e, CLOSE_HANDLE, &[t]), 1);
    }

    #[test]
    fn creating_the_device_without_fpu_preserve_sets_24_bit_precision() {
        let mut e = Engine::new();
        // The game's flags keep the FPU state only with FPU_PRESERVE.
        direct3d_device_created(&mut e, 0x40 | 0x4 | D3DCREATE_FPU_PRESERVE);
        assert_eq!(crate::units::crt::x87_control_word(&mut e), 0x027f);
        assert_eq!(
            crate::x87::x87_add(1.0, 2f64.powi(-30)),
            1.0 + 2f64.powi(-30)
        );
        direct3d_device_created(&mut e, 0x40 | 0x4);
        assert_eq!(crate::units::crt::x87_control_word(&mut e), 0x007f);
        assert_eq!(crate::x87::control_word(), 0x007f);
        assert_eq!(crate::x87::x87_add(1.0, 2f64.powi(-30)), 1.0);
        // A new engine is a new main thread again.
        drop(e);
        let mut e = Engine::new();
        assert_eq!(crate::units::crt::x87_control_word(&mut e), 0x027f);
        assert_eq!(crate::x87::control_word(), 0x027f);
    }

    #[test]
    fn a_thread_runs_with_its_own_control_word() {
        let mut e = kernel_engine();
        let t = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0, 0x00aa_64d0, 0x4444, CREATE_SUSPENDED, 0],
        );
        // The main thread at 24-bit precision, the new thread at its default.
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        let one_third = |_: &mut Engine| crate::x87::x87_div(1.0, 3.0);
        assert_eq!(one_third(&mut e), f64::from(1.0f32 / 3.0));
        let (word, value) = run_as_thread(&mut e, t, |e| {
            let word = crate::units::crt::x87_control_word(e);
            let value = one_third(e);
            // The thread changes its own word (round down).
            crate::units::crt::ctrlfp(e, 0x0400, 0x0c00);
            (word, value)
        });
        assert_eq!(word, 0x027f);
        assert_eq!(value, 1.0 / 3.0);
        assert_eq!(crate::units::crt::x87_control_word(&mut e), 0x007f);
        assert_eq!(crate::x87::control_word(), 0x007f);
        assert!(matches!(
            e.kernel.object(t),
            Some(KernelObject::Thread {
                x87_control_word: 0x067f,
                ..
            })
        ));
    }

    /// A game thread's start function, standing in for a `ThreadProc`: logs
    /// `parameter` and its thread id to the word log at 0x2000, then waits on
    /// the event in `0x2f00` and logs again (`parameter + 1`).
    fn waiting_thread(e: &mut Engine, a: &[u32]) -> Ret {
        log(e, a[0]);
        let ev = e.mem.u32(0x2f00);
        assert_eq!(wait_for_single_object(e, ev, INFINITE), WAIT_OBJECT_0);
        log(e, a[0] + 1);
        Ret {
            eax: 0x77,
            ..Ret::default()
        }
    }

    fn log(e: &mut Engine, word: u32) {
        let n = e.mem.u32(0x2ff0);
        e.mem.set_u32(0x2000 + 4 * n, word);
        let id = get_current_thread_id(e);
        e.mem.set_u32(0x2000 + 4 * n + 0x800, id);
        e.mem.set_u32(0x2ff0, n + 1);
    }

    fn logged(e: &Engine) -> Vec<(u32, u32)> {
        let n = e.mem.u32(0x2ff0);
        (0..n)
            .map(|i| (e.mem.u32(0x2000 + 4 * i), e.mem.u32(0x2800 + 4 * i)))
            .collect()
    }

    fn thread_engine() -> Engine {
        let mut e = kernel_engine();
        e.mem.map(0x2000, 0x1000);
        e.register(0x00ab_0000, waiting_thread);
        e
    }

    /// A worker for [`allocations_per_operation_of_a_thread_switch`]: for ever, waits on the
    /// semaphore in 0x2f20, keeps its OS thread's allocation count in 0x2f28, sets the event in
    /// 0x2f24.
    fn ping_thread(e: &mut Engine, _: &[u32]) -> Ret {
        loop {
            let start = e.mem.u32(0x2f20);
            assert_eq!(wait_for_single_object(e, start, INFINITE), WAIT_OBJECT_0);
            e.mem.set_u64(0x2f28, crate::alloc_count::made());
            let done = e.mem.u32(0x2f24);
            set_event(e, done);
        }
    }

    /// Not a check: the host heap allocations of one round of a worker thread kicked by the main
    /// thread and waited for (two switches), on each side, for the frame-rate work (`cargo test
    /// --release -p engine-core --lib allocations_per -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_a_thread_switch() {
        use crate::alloc_count::{per_run, row, Per};
        let mut e = thread_engine();
        e.register(0x00ab_0900, ping_thread);
        let start = call(&mut e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        let done = call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]);
        e.mem.set_u32(0x2f20, start);
        e.mem.set_u32(0x2f24, done);
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0900, 0, 0, 0]);
        let round = |e: &mut Engine| {
            assert_eq!(call(e, RELEASE_SEMAPHORE, &[start, 1, 0]), 1);
            assert_eq!(
                call(e, WAIT_FOR_SINGLE_OBJECT, &[done, INFINITE]),
                WAIT_OBJECT_0
            );
        };
        for _ in 0..100 {
            round(&mut e);
        }
        let before = e.mem.u64(0x2f28);
        let main = per_run(1000, || round(&mut e));
        let worker = Per {
            allocations: (e.mem.u64(0x2f28) - before) as f64 / 1001.0,
            ns: main.ns,
        };
        row("thread switch round, main thread's side", main);
        row(
            "thread switch round, worker thread's side (same round)",
            worker,
        );
    }

    /// A thread that takes the critical section at 0x2e00, gives up its turn while it holds
    /// it, then leaves it.
    fn locking_thread(e: &mut Engine, a: &[u32]) -> Ret {
        enter_critical_section(e, Ptr::new(0x2e00));
        log(e, a[0]);
        sleep(e, 0);
        log(e, a[0] + 1);
        leave_critical_section(e, Ptr::new(0x2e00));
        Ret::default()
    }

    #[test]
    fn the_task_allocator_gives_and_takes_blocks() {
        let mut e = thread_engine();
        let block = co_task_mem_alloc(&mut e, 0x48);
        assert_ne!(block, 0);
        e.mem.set_u32(block + 0x44, 1);
        co_task_mem_free(&mut e, Ptr::new(block));
        co_task_mem_free(&mut e, Ptr::new(0));
        assert_ne!(co_task_mem_alloc(&mut e, 0), 0);
        assert_eq!(co_initialize_ex(&mut e, 0, 0), 0);
        co_uninitialize(&mut e);
        assert_eq!(
            co_initialize_ex(&mut e, 0, 2),
            0,
            "a new start after CoUninitialize"
        );
    }

    #[test]
    fn co_initialize_ex_answers_per_thread_and_mode() {
        let mut e = thread_engine();
        assert_eq!(co_initialize_ex(&mut e, 0, 0), 0);
        assert_eq!(co_initialize_ex(&mut e, 0, 0), 1, "S_FALSE");
        assert_eq!(
            co_initialize_ex(&mut e, 0, 2),
            0x8001_0106,
            "RPC_E_CHANGED_MODE"
        );
        assert_eq!(co_initialize_ex(&mut e, 4, 0), 0x8007_0057, "E_INVALIDARG");
    }

    #[test]
    fn the_network_utility_is_made_once_and_its_callback_hears_the_state() {
        let mut e = thread_engine();
        e.mem.map(NETWORK_UTILITY_INSTANCE, 4);
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0xffff_ffff);
        e.register_double(0x00aa_3e40, move |_, words| {
            assert_eq!(words, &[MEMORY_MANAGER, 8]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let made = bs_network_system_utility_q_instance(&mut e).addr();
        assert_eq!(made, block);
        assert_eq!((e.mem.u8(block), e.mem.u32(block + 4)), (0, 0));
        assert_eq!(bs_network_system_utility_q_instance(&mut e).addr(), block);
        let heard = e.mem.alloc(8);
        e.register_double(0x0012_3450, move |e, words| {
            e.mem.set_u32(heard, words[0] + 0x10);
            e.mem.set_u32(heard + 4, words[1]);
            Ret::default()
        });
        e.mem.set_u8(block, 1);
        fn_00af2d70(&mut e, Ptr::new(block), 0x0012_3450);
        assert_eq!(e.mem.u32(block + 4), 0x0012_3450);
        assert_eq!((e.mem.u32(heard), e.mem.u32(heard + 4)), (0x10, 1));
    }

    #[test]
    fn heap_alloc_on_the_crt_heap_hands_out_platform_blocks() {
        let mut e = thread_engine();
        let heap = heap_create(&mut e, 0, 0x1000, 0);
        let block = heap_alloc(&mut e, heap, 0, 0x20);
        assert_ne!(block, 0);
        assert_eq!(e.kernel.crt_block_sizes.get(&block), Some(&0x20));
        e.mem.set_u32(block, 0xdead_beef);
        crate::units::crt::free(&mut e, Ptr::new(block));
        let zeroed = heap_alloc(&mut e, heap, 8, 0x10);
        assert_eq!(e.mem.u32(zeroed), 0);
    }

    #[test]
    fn a_critical_section_held_by_another_thread_makes_the_main_thread_wait() {
        let mut e = thread_engine();
        e.register(0x00ab_0600, locking_thread);
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0600, 20, 0, 0]);
        // A switch point lets the thread take the section and give up its turn.
        sleep(&mut e, 0);
        assert_eq!(logged(&e), [(20, 2)]);
        // The main thread waits for it to leave before it gets the section.
        enter_critical_section(&mut e, Ptr::new(0x2e00));
        log(&mut e, 30);
        assert_eq!(logged(&e), [(20, 2), (21, 2), (30, MAIN_THREAD_ID)]);
        // The owner may enter again; it leaves as many times.
        enter_critical_section(&mut e, Ptr::new(0x2e00));
        assert_eq!(try_enter_critical_section(&mut e, Ptr::new(0x2e00)), 1);
        for _ in 0..3 {
            leave_critical_section(&mut e, Ptr::new(0x2e00));
        }
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
    }

    #[test]
    fn a_created_thread_runs_at_the_next_switch_point_and_waits_its_turn() {
        let mut e = thread_engine();
        // The scheme before: a signal switches nothing.
        e.kernel.threads.scheme = crate::thread_host::Scheme::SLICE;
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]);
        e.mem.set_u32(0x2f00, ev);
        let done = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0000, 10, 0, 0]);
        // Created running, but it runs only at a switch point.
        assert!(logged(&e).is_empty());
        // The main thread's wait with a timeout is a switch point: the thread
        // runs up to its own wait, and the main thread's wait times out.
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[done, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(logged(&e), [(10, 2)]);
        // SetEvent wakes it at the next switch point: waiting on its handle.
        assert_eq!(call(&mut e, SET_EVENT, &[ev]), 1);
        assert_eq!(logged(&e).len(), 1);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(logged(&e), [(10, 2), (11, 2)]);
        // The main thread is itself again.
        assert_eq!(get_current_thread_id(&mut e), MAIN_THREAD_ID);
        assert!(matches!(
            crate::thread_host::by_handle(&mut e, t).unwrap().state,
            crate::thread_host::RunState::Finished { code: 0x77 }
        ));
    }

    /// A worker kicked as the AI linear task threads are (`008c78c0` releases each one's start
    /// semaphore): waits on the semaphore in 0x2f04, logs `parameter`, then sets the event in
    /// 0x2f08 (as `008c79e0` does for the other worker).
    fn kicked_thread(e: &mut Engine, a: &[u32]) -> Ret {
        let start = e.mem.u32(0x2f04);
        assert_eq!(wait_for_single_object(e, start, INFINITE), WAIT_OBJECT_0);
        log(e, a[0]);
        let next = e.mem.u32(0x2f08);
        set_event(e, next);
        Ret::default()
    }

    /// A worker that waits on the event in 0x2f08, then logs `parameter`.
    fn chained_thread(e: &mut Engine, a: &[u32]) -> Ret {
        let ev = e.mem.u32(0x2f08);
        assert_eq!(wait_for_single_object(e, ev, INFINITE), WAIT_OBJECT_0);
        log(e, a[0]);
        Ret::default()
    }

    /// Two workers waiting (the chained one made first), the kick from the main thread, then
    /// the main thread logs 50: what was logged. The engine stays where it is (the
    /// coroutines hold its address).
    fn kick(e: &mut Engine, scheme: crate::thread_host::Scheme, delay: u64) -> Vec<(u32, u32)> {
        e.kernel.threads.scheme = scheme;
        e.kernel.threads.wake_delay = delay;
        e.register(0x00ab_0700, kicked_thread);
        e.register(0x00ab_0710, chained_thread);
        let start = call(e, CREATE_SEMAPHORE_A, &[0, 0, 1, 0]);
        let next = call(e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        e.mem.set_u32(0x2f04, start);
        e.mem.set_u32(0x2f08, next);
        call(e, CREATE_THREAD, &[0, 0, 0x00ab_0710, 40, 0, 0]);
        call(e, CREATE_THREAD, &[0, 0, 0x00ab_0700, 30, 0, 0]);
        // A switch point: both run up to their waits.
        call(e, SLEEP, &[0]);
        assert!(logged(e).is_empty());
        assert_eq!(call(e, RELEASE_SEMAPHORE, &[start, 1, 0]), 1);
        log(e, 50);
        logged(e)
    }

    #[test]
    fn a_signal_from_the_main_thread_runs_the_threads_it_wakes_first() {
        use crate::thread_host::Scheme;
        // `WAKE` with no delay: the kicked worker runs inside the signalling call, and the
        // worker it woke in the next round (made earlier, so not in the first).
        let mut e = thread_engine();
        let seen = kick(&mut e, Scheme::WAKE, 0);
        assert_eq!(seen, [(30, 3), (40, 2), (50, MAIN_THREAD_ID)]);
        // With a delay of 2 calls: main's next steps come first; the workers run at its first
        // platform call after the delay (the third `GetCurrentThread`).
        let mut e = thread_engine();
        let seen = kick(&mut e, Scheme::WAKE, 2);
        assert_eq!(seen, [(50, MAIN_THREAD_ID)]);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        assert_eq!(logged(&e), [(50, MAIN_THREAD_ID)]);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        assert_eq!(logged(&e), [(50, MAIN_THREAD_ID), (30, 3), (40, 2)]);
        assert!(!e.kernel.threads.wake_pending);
        // `SLICE`: main goes on first; the workers run at its next switch point.
        let mut e = thread_engine();
        let seen = kick(&mut e, Scheme::SLICE, 0);
        assert_eq!(seen, [(50, MAIN_THREAD_ID)]);
        // One pass in creation order: the chained worker comes before the one that wakes it,
        // so it waits for the pass after.
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(50, MAIN_THREAD_ID), (30, 3)]);
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(50, MAIN_THREAD_ID), (30, 3), (40, 2)]);
    }

    /// A plain (non-platform) function: counts its calls in 0x2f10.
    const PLAIN: u32 = 0x00ab_0800;

    /// Ten rounds of ten plain calls and one platform call (`GetCurrentThread`), with a wait
    /// on the (signalled) event in 0x2f14 after the fourth round: a blocking call that does
    /// not block.
    fn counting_thread(e: &mut Engine, _: &[u32]) -> Ret {
        for round in 0..10 {
            for _ in 0..10 {
                e.call(PLAIN, &[]);
            }
            e.call(GET_CURRENT_THREAD, &[]);
            if round == 3 {
                let ev = e.mem.u32(0x2f14);
                e.call(WAIT_FOR_SINGLE_OBJECT, &[ev, INFINITE]);
            }
        }
        Ret::default()
    }

    /// Only plain calls, 300 of them.
    fn plain_thread(e: &mut Engine, _: &[u32]) -> Ret {
        for _ in 0..300 {
            e.call(PLAIN, &[]);
        }
        Ret::default()
    }

    /// The plain calls the thread at `start` had made when its first turn (slices of `slice`
    /// calls) ended; it then runs to its end with `total` plain calls.
    fn first_turn_end(
        switch_at: crate::thread_host::SwitchAt,
        slice: u64,
        start: u32,
        total: u32,
    ) -> u32 {
        let mut e = thread_engine();
        e.kernel.threads.scheme.switch_at = switch_at;
        e.register(PLAIN, |e, _| {
            let n = e.mem.u32(0x2f10);
            e.mem.set_u32(0x2f10, n + 1);
            Ret::default()
        });
        e.register(0x00ab_0810, counting_thread);
        e.register(0x00ab_0820, plain_thread);
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 1, 0]);
        e.mem.set_u32(0x2f14, ev);
        crate::thread_host::start_slicing(&mut e, slice);
        let t = call(&mut e, CREATE_THREAD, &[0, 0, start, 0, 0, 0]);
        call(&mut e, SLEEP, &[0]);
        let at = e.mem.u32(0x2f10);
        assert!(matches!(
            crate::thread_host::by_handle(&mut e, t).unwrap().state,
            crate::thread_host::RunState::Sleeping { .. }
        ));
        assert_eq!(e.kernel.threads.preempted, 1);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(e.mem.u32(0x2f10), total);
        at
    }

    #[test]
    fn after_its_slice_a_thread_gives_up_its_turn_where_the_scheme_says() {
        use crate::thread_host::SwitchAt;
        // Calls of the turn: 1 the start function, 2-11 plain, 12 the platform call, 13-22
        // plain, 23 platform, 24-33 plain, 34 platform, 35-44 plain, 45 platform, 46 the wait.
        // The 26th call finds the slice of 25 used up.
        // `Blocking` (the default): the turn ends before the wait, after 40 plain calls.
        assert_eq!(first_turn_end(SwitchAt::Blocking, 25, 0x00ab_0810, 100), 40);
        // `Platform`: before the next platform call (the 34th), after 30 plain calls.
        assert_eq!(first_turn_end(SwitchAt::Platform, 25, 0x00ab_0810, 100), 30);
        // `Any`: at the 26th call itself, in the middle of a round.
        assert_eq!(first_turn_end(SwitchAt::Any, 25, 0x00ab_0810, 100), 22);
    }

    #[test]
    fn a_turn_without_a_blocking_call_ends_at_the_hard_cap() {
        use crate::thread_host::{SwitchAt, HARD_CAP_SLICES};
        // Slices of 2: the start function and one plain call use the first; the turn goes on
        // to 50 slices in all, so it ends after 100 plain calls.
        assert_eq!(
            first_turn_end(SwitchAt::Blocking, 2, 0x00ab_0820, 300),
            (2 * HARD_CAP_SLICES) as u32
        );
    }

    /// A thread that waits 100 ms on the event in 0x2f00, then logs the answer and the clock.
    fn timed_thread(e: &mut Engine, _: &[u32]) -> Ret {
        let ev = e.mem.u32(0x2f00);
        let answer = wait_for_single_object(e, ev, 100);
        log(e, answer);
        let now = tick_count(e);
        log(e, now);
        Ret::default()
    }

    #[test]
    fn a_game_threads_timed_wait_times_out_at_its_deadline() {
        let mut e = thread_engine();
        e.register(0x00ab_0900, timed_thread);
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        e.mem.set_u32(0x2f00, ev);
        e.clock_ms = 1000;
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0900, 0, 0, 0]);
        call(&mut e, SLEEP, &[0]);
        // Before its deadline it still waits.
        e.clock_ms = 1099;
        call(&mut e, SLEEP, &[0]);
        assert!(logged(&e).is_empty());
        // At it, it times out.
        e.clock_ms = 1100;
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(WAIT_TIMEOUT, 2), (1100, 2)]);
        // With main waiting and nothing else able to run, the clock jumps to the deadline.
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0900, 0, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(logged(&e)[2..], [(WAIT_TIMEOUT, 3), (1200, 3)]);
    }

    #[test]
    fn under_the_slice_scheme_a_game_threads_timed_wait_goes_on_only_when_signalled() {
        let mut e = thread_engine();
        e.kernel.threads.scheme = crate::thread_host::Scheme::SLICE;
        e.register(0x00ab_0900, timed_thread);
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        e.mem.set_u32(0x2f00, ev);
        e.clock_ms = 1000;
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0900, 0, 0, 0]);
        call(&mut e, SLEEP, &[0]);
        e.clock_ms = 5000;
        call(&mut e, SLEEP, &[0]);
        assert!(logged(&e).is_empty());
        call(&mut e, SET_EVENT, &[ev]);
        assert!(logged(&e).is_empty());
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(WAIT_OBJECT_0, 2), (5000, 2)]);
    }

    #[test]
    fn nv_threads_names_the_scheme() {
        use crate::thread_host::{Scheme, SwitchAt};
        assert_eq!(Scheme::from_name(None), Ok(Scheme::WAKE));
        assert_eq!(Scheme::WAKE.switch_at, SwitchAt::Blocking);
        assert_eq!(Scheme::from_name(Some("")), Ok(Scheme::WAKE));
        assert_eq!(Scheme::from_name(Some("slice")), Ok(Scheme::SLICE));
        assert_eq!(Scheme::from_name(Some(" SLICE ")), Ok(Scheme::SLICE));
        assert_eq!(Scheme::from_name(Some("wake")), Ok(Scheme::WAKE));
        assert_eq!(
            Scheme::from_name(Some("nowake, notimeout")),
            Ok(Scheme {
                wake: false,
                switch_at: SwitchAt::Blocking,
                timed_waits: false,
            })
        );
        assert_eq!(
            Scheme::from_name(Some("platformcall")),
            Ok(Scheme {
                switch_at: SwitchAt::Platform,
                ..Scheme::WAKE
            })
        );
        assert_eq!(
            Scheme::from_name(Some("anycall")),
            Ok(Scheme {
                switch_at: SwitchAt::Any,
                ..Scheme::WAKE
            })
        );
        assert!(Scheme::from_name(Some("fast")).is_err());
        assert_eq!(Scheme::default(), Scheme::WAKE);
    }

    /// A thread that stores its id + 0x100 in TLS slot 0 and its TLS block at 0x2f20 + 4 *
    /// `parameter`, waits on the event at 0x2f00 + 4 * `parameter`, then logs its TLS slot 0,
    /// whether its TLS block is still its own, and whether the x87 control register holds its
    /// own word (each with its thread id).
    fn tls_thread(e: &mut Engine, a: &[u32]) -> Ret {
        let id = get_current_thread_id(e);
        tls_set_value(e, 0, id + 0x100);
        let block = e.tls();
        e.mem.set_u32(0x2f20 + 4 * a[0], block);
        let ev = e.mem.u32(0x2f00 + 4 * a[0]);
        assert_eq!(wait_for_single_object(e, ev, INFINITE), WAIT_OBJECT_0);
        let value = tls_get_value(e, 0);
        log(e, value);
        let own = e.tls() == e.mem.u32(0x2f20 + 4 * a[0]);
        log(e, u32::from(own));
        let word = crate::x87::control_word() == crate::units::crt::x87_control_word(e);
        log(e, u32::from(word));
        Ret::default()
    }

    /// A thread run because the main thread woke it ([`crate::thread_host::request_wake`]),
    /// inside the signalling call or after the delay, sees its own TLS slot values, TLS block,
    /// thread id and control word, as at any switch; the main thread keeps its own.
    #[test]
    fn a_thread_run_on_wake_sees_its_own_tls_id_and_control_word() {
        for delay in [0u64, 3] {
            let mut e = thread_engine();
            e.kernel.threads.wake_delay = delay;
            e.register(0x00ab_0a00, tls_thread);
            assert_eq!(tls_alloc(&mut e), 0);
            tls_set_value(&mut e, 0, 0x1111);
            let main_block = e.tls();
            direct3d_device_created(&mut e, 0);
            for slot in 0..2 {
                let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
                e.mem.set_u32(0x2f00 + 4 * slot, ev);
                call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0a00, slot, 0, 0]);
            }
            call(&mut e, SLEEP, &[0]);
            assert!(logged(&e).is_empty());
            // Wake the second thread (id 3) only.
            let ev = e.mem.u32(0x2f04);
            call(&mut e, SET_EVENT, &[ev]);
            if delay != 0 {
                for _ in 0..delay {
                    call(&mut e, GET_CURRENT_THREAD, &[]);
                }
                assert!(logged(&e).is_empty(), "not before the delay is over");
                call(&mut e, GET_CURRENT_THREAD, &[]);
            }
            assert_eq!(logged(&e), [(0x103, 3), (1, 3), (1, 3)], "delay {delay}");
            // The main thread is itself again.
            assert_eq!(get_current_thread_id(&mut e), MAIN_THREAD_ID);
            assert_eq!(tls_get_value(&mut e, 0), 0x1111);
            assert_eq!(e.tls(), main_block);
            assert_eq!(crate::x87::control_word(), 0x007f);
            // The other thread still waits.
            let other = e.mem.u32(0x2f00);
            call(&mut e, SET_EVENT, &[other]);
            call(&mut e, SLEEP, &[0]);
            assert_eq!(logged(&e)[3..], [(0x102, 2), (1, 2), (1, 2)]);
        }
    }

    /// A thread that logs `parameter`, gives up its turn (`Sleep(0)`, as a spin lock's backoff
    /// does), then logs `parameter + 1`.
    fn yielding_thread(e: &mut Engine, a: &[u32]) -> Ret {
        log(e, a[0]);
        sleep(e, 0);
        log(e, a[0] + 1);
        Ret::default()
    }

    /// Main gives the thread a turn (it logs 60 and `Sleep(0)`s), then makes `GetCurrentThread`
    /// calls; answers what was logged after 2 and after 3 of them.
    fn parked(scheme: crate::thread_host::Scheme) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut e = thread_engine();
        e.kernel.threads.scheme = scheme;
        e.kernel.threads.wake_delay = 2;
        e.register(0x00ab_0b00, yielding_thread);
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0b00, 60, 0, 0]);
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(60, 2)]);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        let after_two = logged(&e);
        call(&mut e, GET_CURRENT_THREAD, &[]);
        let after_three = logged(&e);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        (after_two, after_three)
    }

    /// A thread left runnable while main runs on (here after its `Sleep(0)`) gets its turn
    /// after main's wake delay, at main's next platform call, not only at main's next
    /// blocking call (p187).
    #[test]
    fn a_thread_parked_runnable_runs_after_the_wake_delay() {
        use crate::thread_host::Scheme;
        let (two, three) = parked(Scheme::WAKE);
        assert_eq!(two, [(60, 2)]);
        assert_eq!(three, [(60, 2), (61, 2)]);
        // `SLICE`: it waits for main's next switch point.
        let (two, three) = parked(Scheme::SLICE);
        assert_eq!(two, [(60, 2)]);
        assert_eq!(three, [(60, 2)]);
    }

    /// Each switch is in the trace: main to the thread and back, with both ids.
    #[test]
    fn the_trace_marks_thread_switches() {
        use crate::thread_host::TRACE_SWITCH;
        let mut e = thread_engine();
        e.register(0x00ab_0b00, yielding_thread);
        e.trace = Some(Default::default());
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0b00, 60, 0, 0]);
        call(&mut e, SLEEP, &[0]);
        let switches: Vec<(Vec<u32>, u32)> = e
            .trace
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(addr, _, _)| *addr == TRACE_SWITCH)
            .map(|(_, words, eax)| (words.clone(), *eax))
            .collect();
        assert_eq!(
            switches,
            [
                (vec![MAIN_THREAD_ID, 2], 2),
                (vec![2, MAIN_THREAD_ID], MAIN_THREAD_ID)
            ]
        );
    }

    /// A game thread's start function that records its control word (0x2100)
    /// and its 1/3 (0x2108, `x87_div`), sets round down on itself, waits on
    /// the event in `0x2f00`, then records its word again (0x2104).
    fn precision_thread(e: &mut Engine, _: &[u32]) -> Ret {
        let word = crate::units::crt::x87_control_word(e);
        e.mem.set_u32(0x2100, u32::from(word));
        let third = crate::x87::x87_div(1.0, 3.0);
        e.mem.set_u64(0x2108, third.to_bits());
        crate::units::crt::ctrlfp(e, 0x0400, 0x0c00);
        let ev = e.mem.u32(0x2f00);
        assert_eq!(wait_for_single_object(e, ev, INFINITE), WAIT_OBJECT_0);
        e.mem.set_u32(0x2104, u32::from(crate::x87::control_word()));
        Ret::default()
    }

    /// The thread host's game threads keep their own x87 precision: the AI
    /// linear task threads at 0x027F (53-bit) while the main thread is at
    /// 0x007F once the device exists, each word loaded again when it runs.
    #[test]
    fn a_stepped_game_thread_keeps_its_own_precision() {
        let mut e = thread_engine();
        e.register(0x00ab_0100, precision_thread);
        direct3d_device_created(&mut e, 0);
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 0, 0, 0]);
        e.mem.set_u32(0x2f00, ev);
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0100, 0, 0, 0]);
        let done = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[done, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(e.mem.u32(0x2100), 0x027f);
        assert_eq!(f64::from_bits(e.mem.u64(0x2108)), 1.0 / 3.0);
        // Back on the main thread: 24-bit.
        assert_eq!(crate::x87::control_word(), 0x007f);
        assert_eq!(crate::x87::x87_div(1.0, 3.0), f64::from(1.0f32 / 3.0));
        call(&mut e, SET_EVENT, &[ev]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        // The thread's own change (round down) came back with it.
        assert_eq!(e.mem.u32(0x2104), 0x067f);
        assert_eq!(crate::units::crt::x87_control_word(&mut e), 0x007f);
    }

    #[test]
    fn two_threads_take_turns_in_creation_order() {
        let mut e = thread_engine();
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        e.mem.set_u32(0x2f00, ev);
        let a = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0000, 10, 0, 0]);
        let b = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0, 0x00ab_0000, 20, CREATE_SUSPENDED, 0],
        );
        assert_eq!(call(&mut e, RESUME_THREAD, &[b]), 1);
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e), [(10, 2), (20, 3)]);
        // A manual-reset event lets both go on, in creation order.
        call(&mut e, SET_EVENT, &[ev]);
        e.mem.set_u32(0x2e00, a);
        e.mem.set_u32(0x2e04, b);
        assert_eq!(
            call(&mut e, WAIT_FOR_MULTIPLE_OBJECTS, &[2, 0x2e00, 1, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(logged(&e), [(10, 2), (20, 3), (11, 2), (21, 3)]);
    }

    #[test]
    fn each_thread_has_its_own_tls_block_and_slot_values() {
        let mut e = thread_engine();
        let index = tls_alloc(&mut e);
        tls_set_value(&mut e, index, 0x1111);
        let main_block = e.tls();
        e.register(0x00ab_0100, |e, _| {
            let block = e.tls();
            Ret {
                eax: block,
                edx: tls_get_value(e, 0),
                ..Ret::default()
            }
        });
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0100, 0, 0, 0]);
        call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]);
        let crate::thread_host::RunState::Finished { code } =
            crate::thread_host::by_handle(&mut e, t).unwrap().state
        else {
            panic!("not finished")
        };
        assert_ne!(code, main_block, "its own block");
        assert_eq!(e.tls(), main_block);
        assert_eq!(tls_get_value(&mut e, index), 0x1111);
    }

    #[test]
    fn a_stopped_thread_stops_the_run_that_switched_to_it() {
        let mut e = thread_engine();
        e.register(0x00ab_0200, |e, _| e.call(0x00ab_cdef, &[]));
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0200, 0, 0, 0]);
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]);
        }))
        .unwrap_err();
        let text = stop.downcast_ref::<String>().cloned().unwrap_or_default();
        assert!(text.starts_with("open function 00abcdef"), "{text}");
        assert!(text.contains("in game thread 2, start 00ab0200"), "{text}");
        // Its handle is signalled now (it ended).
        assert_eq!(call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, 0]), WAIT_OBJECT_0);
        assert!(e.call_stack.is_empty() || e.call_stack.iter().all(|&a| a != 0x00ab_0200));
    }

    #[test]
    fn lstrcpy_and_lstrcat_copy_with_the_zero_and_answer_the_destination() {
        let mut e = Engine::new();
        let d = e.mem.alloc(32);
        let s = e.mem.alloc(8);
        e.mem.set_cstr(s, b"abc");
        e.mem.set_cstr(d, b"zzzzzzzz");
        assert_eq!(lstrcpy_a(&mut e, Ptr::new(d), Ptr::new(s)), d);
        assert_eq!(e.mem.cstr(d), b"abc".to_vec());
        assert_eq!(e.mem.u8(d + 4), b'z', "only the string and its zero");
        assert_eq!(lstrcat_a(&mut e, Ptr::new(d), Ptr::new(s)), d);
        assert_eq!(e.mem.cstr(d), b"abcabc".to_vec());
    }

    /// `GetTickCount` moves in whole ticks of 15.625 ms (steps of 15 and 16 ms, as measured)
    /// while the clock and `timeGetTime` move by the millisecond; it never reads ahead of them.
    #[test]
    fn get_tick_count_moves_in_whole_ticks_of_the_clock_interrupt() {
        let mut e = thread_engine();
        e.clock_ms = 1000;
        let mut steps = std::collections::BTreeMap::new();
        let mut last = get_tick_count(&mut e);
        assert_eq!(last, 1000, "64 ticks");
        for _ in 0..1000 {
            e.advance_clock_us(1000);
            let now = get_tick_count(&mut e);
            assert!(now <= tick_count(&mut e));
            assert!(tick_count(&mut e) - now <= 16, "under a tick behind");
            if now != last {
                *steps.entry(now - last).or_insert(0) += 1;
                last = now;
            }
        }
        assert_eq!(steps.keys().copied().collect::<Vec<_>>(), [15, 16]);
        assert_eq!(steps.values().sum::<u32>(), 64, "1000 ms is 64 ticks");
        // A step of under a tick reads the same value.
        e.clock_ms = 1015;
        e.clock_us = 600;
        assert_eq!(get_tick_count(&mut e), 1000, "1015.6 ms is still 64 ticks");
        e.clock_us = 700;
        assert_eq!(get_tick_count(&mut e), 1015, "65 ticks: 1015.625 ms");
        assert_eq!(whole_ticks_ms(15_624), 0);
        assert_eq!(whole_ticks_ms(31_250), 31);
    }

    #[test]
    fn one_clock_sleep_moves_it_and_the_real_clock_adds_passed_time() {
        let mut e = thread_engine();
        e.clock_ms = 1000;
        assert_eq!(tick_count(&mut e), 1000);
        call(&mut e, SLEEP, &[25]);
        assert_eq!(
            tick_count(&mut e),
            1025,
            "Sleep moves the clock by its time"
        );
        let out = e.mem.alloc(8);
        query_performance_counter(&mut e, Ptr::new(out));
        assert_eq!(
            e.mem.u64(out),
            1_025_000,
            "the counter reads the same clock"
        );
        e.start_real_clock();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let later = tick_count(&mut e);
        assert!(later >= 1030, "real time passed: {later}");
        query_performance_counter(&mut e, Ptr::new(out));
        assert!(e.mem.u64(out) >= u64::from(later) * 1000);
    }

    /// A thread that logs the clock, sleeps 50 ms, and does so three times.
    fn sleeping_thread(e: &mut Engine, _: &[u32]) -> Ret {
        for _ in 0..3 {
            let now = tick_count(e);
            log(e, now);
            sleep(e, 50);
        }
        Ret::default()
    }

    #[test]
    fn a_game_threads_sleep_leaves_the_clock_and_waits_for_its_wake_time() {
        let mut e = thread_engine();
        e.register(0x00ab_0500, sleeping_thread);
        e.clock_ms = 1000;
        call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0500, 0, 0, 0]);
        // Main's Sleep(0): the thread runs and sleeps; the clock stays.
        call(&mut e, SLEEP, &[0]);
        assert_eq!(
            tick_count(&mut e),
            1000,
            "a game thread's Sleep keeps the clock"
        );
        // Passes before its wake time leave it asleep.
        e.clock_ms = 1049;
        call(&mut e, SLEEP, &[0]);
        assert_eq!(logged(&e).len(), 1);
        // From its wake time it runs again.
        e.clock_ms = 1050;
        call(&mut e, SLEEP, &[0]);
        assert_eq!(
            logged(&e).iter().map(|l| l.0).collect::<Vec<_>>(),
            [1000, 1050]
        );
        // Main's own Sleep moves the clock as before, and the thread runs when due.
        call(&mut e, SLEEP, &[60]);
        assert_eq!(tick_count(&mut e), 1110);
        assert_eq!(
            logged(&e).iter().map(|l| l.0).collect::<Vec<_>>(),
            [1000, 1050, 1110]
        );
    }

    #[test]
    fn with_the_real_clock_mains_sleep_really_waits_and_the_clock_keeps_to_real_time() {
        let mut e = thread_engine();
        e.start_real_clock();
        let start = std::time::Instant::now();
        let before = u64::from(tick_count(&mut e));
        call(&mut e, SLEEP, &[30]);
        let after = u64::from(tick_count(&mut e));
        let real = start.elapsed().as_millis() as u64;
        assert!(real >= 30, "the host waited: {real} ms");
        assert!(
            after - before >= 30,
            "the clock moved on: {}",
            after - before
        );
        assert!(
            after - before <= real + 1,
            "the clock is not ahead of the real time: {} > {real}",
            after - before
        );
    }

    #[test]
    fn main_waiting_while_every_thread_sleeps_jumps_the_clock_to_the_earliest_wake() {
        let mut e = thread_engine();
        e.register(0x00ab_0500, sleeping_thread);
        e.clock_ms = 1000;
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0500, 0, 0, 0]);
        // Main waits for the thread's end: the clock jumps to each wake time in turn.
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(
            logged(&e).iter().map(|l| l.0).collect::<Vec<_>>(),
            [1000, 1050, 1100]
        );
        assert_eq!(tick_count(&mut e), 1150);
        // A timed wait on an event nobody sets, with nobody sleeping: the clock jumps to
        // its deadline and it times out.
        let ev = call(&mut e, CREATE_EVENT_A, &[0, 1, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[ev, 30]),
            WAIT_TIMEOUT
        );
        assert_eq!(tick_count(&mut e), 1180);
    }

    #[test]
    fn get_exit_code_thread_is_still_active_until_the_thread_ends() {
        let mut e = thread_engine();
        e.register(0x00ab_0400, |_, _| 9u32.into_ret());
        let t = call(
            &mut e,
            CREATE_THREAD,
            &[0, 0, 0x00ab_0400, 0, CREATE_SUSPENDED, 0],
        );
        let out = e.mem.alloc(4);
        assert_eq!(call(&mut e, GET_EXIT_CODE_THREAD, &[t, out]), 1);
        assert_eq!(e.mem.u32(out), STILL_ACTIVE);
        call(&mut e, RESUME_THREAD, &[t]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert_eq!(call(&mut e, GET_EXIT_CODE_THREAD, &[t, out]), 1);
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(call(&mut e, GET_EXIT_CODE_THREAD, &[0x1234, out]), 0);
        assert_eq!(e.kernel.last_error, ERROR_INVALID_HANDLE);
    }

    #[test]
    fn exit_thread_ends_the_thread_with_its_code() {
        let mut e = thread_engine();
        e.register(0x00ab_0300, |e, _| {
            exit_thread(e, 5);
            unreachable!("ExitThread returned")
        });
        let t = call(&mut e, CREATE_THREAD, &[0, 0, 0x00ab_0300, 0, 0, 0]);
        assert_eq!(
            call(&mut e, WAIT_FOR_SINGLE_OBJECT, &[t, INFINITE]),
            WAIT_OBJECT_0
        );
        assert!(matches!(
            crate::thread_host::by_handle(&mut e, t).unwrap().state,
            crate::thread_host::RunState::Finished { code: 5 }
        ));
    }

    #[test]
    fn reserve_commit_decommit_release_on_the_heap() {
        let mut e = kernel_engine();
        let base = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x80_0000, MEM_RESERVE, PAGE_READWRITE],
        );
        assert_ne!(base, 0);
        assert_eq!(base % ALLOCATION_GRANULARITY, 0);
        assert!(!e.mem.is_mapped(base), "reserved, not committed");
        // Commit rounds out to whole pages and returns the page start.
        let p = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base + 0x10, 0x1ff0, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(p, base);
        assert_eq!(e.mem.bytes(base, 0x2000), vec![0; 0x2000]);
        assert!(!e.mem.is_mapped(base + 0x2000));
        e.mem.set_u32(base + 0x1000, 7);
        // Committing again keeps the contents.
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base, 0x2000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(e.mem.u32(base + 0x1000), 7);
        // Outside the reservation: fails.
        let end = base + 0x80_0000;
        assert_eq!(
            call(
                &mut e,
                VIRTUAL_ALLOC,
                &[end - 0x1000, 0x2000, MEM_COMMIT, PAGE_READWRITE]
            ),
            0
        );
        // Decommit the tail page; recommitting gives zeros again.
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[base + 0x1000, 0x1000, MEM_DECOMMIT]),
            1
        );
        assert!(!e.mem.is_mapped(base + 0x1000));
        assert!(e.mem.is_mapped(base));
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[base + 0x1000, 0x1000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(e.mem.u32(base + 0x1000), 0);
        // Release needs size 0 and the reservation's start.
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0x1000, MEM_RELEASE]), 0);
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[base + 0x1000, 0, MEM_RELEASE]),
            0
        );
        let block = e.kernel.regions[&base].block;
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 1);
        assert!(!e.mem.is_mapped(base));
        assert_eq!(e.mem.block_size(block), None, "heap block freed");
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 0);
    }

    #[test]
    fn commit_at_address_zero_reserves_and_commits() {
        let mut e = kernel_engine();
        let base = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x1800, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(base % ALLOCATION_GRANULARITY, 0);
        assert_eq!(e.mem.bytes(base, 0x2000), vec![0; 0x2000]);
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_DECOMMIT]), 1);
        assert!(!e.mem.is_mapped(base));
        assert_eq!(call(&mut e, VIRTUAL_FREE, &[base, 0, MEM_RELEASE]), 1);
    }

    #[test]
    fn reserve_at_a_fixed_address_only_where_nothing_is() {
        let mut e = kernel_engine();
        let reserve = |e: &mut Engine, at: u32| {
            call(
                e,
                VIRTUAL_ALLOC,
                &[at, 0x2_0000, MEM_RESERVE, PAGE_READWRITE],
            )
        };
        // The image, the heap region: fail (the memory pools then try the
        // next 16 MiB step).
        assert_eq!(reserve(&mut e, 0x0100_0000), 0);
        assert_eq!(reserve(&mut e, 0x2000_0000), 0);
        assert_eq!(reserve(&mut e, 0x0200_0000), 0x0200_0000);
        assert_eq!(reserve(&mut e, 0x0201_0000), 0, "overlaps a reservation");
        e.mem.map(0x0300_0000, 4);
        assert_eq!(reserve(&mut e, 0x0300_0000), 0, "mapped memory");
        let page = call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0x0201_0000, 0x1000, MEM_COMMIT, PAGE_READWRITE],
        );
        assert_eq!(page, 0x0201_0000);
        assert!(e.mem.is_mapped(0x0201_0000));
        assert_eq!(
            call(&mut e, VIRTUAL_FREE, &[0x0200_0000, 0, MEM_RELEASE]),
            1
        );
        assert!(!e.mem.is_mapped(0x0201_0000));
        assert_eq!(reserve(&mut e, 0x0200_0000), 0x0200_0000, "free again");
    }

    #[test]
    #[should_panic(expected = "only MEM_RESERVE and MEM_COMMIT with PAGE_READWRITE")]
    fn other_allocation_types_stop() {
        let mut e = kernel_engine();
        call(
            &mut e,
            VIRTUAL_ALLOC,
            &[0, 0x1000, MEM_COMMIT | 0x10_0000, PAGE_READWRITE],
        );
    }
}

#[cfg(test)]
mod os_defaults_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x1000, 0x1000);
        e
    }

    #[test]
    fn the_active_window_is_the_main_window() {
        let mut e = engine();
        assert_eq!(get_active_window(&mut e), 0);
        let window = create_window(&mut e);
        assert_eq!(get_active_window(&mut e), window);
        assert_eq!(swap_mouse_button(&mut e, 0), 0);
        assert_eq!(swap_mouse_button(&mut e, 1), 0);
        assert_eq!(swap_mouse_button(&mut e, 0), 1);
        assert_eq!(get_double_click_time(&mut e), 500);
    }

    /// `HKCU\Control Panel\Mouse` opens and answers `MouseSensitivity` = "10" (`REG_SZ`, three
    /// bytes with the zero); a short buffer gets `ERROR_MORE_DATA`; other keys and values aren't
    /// there.
    #[test]
    fn the_mouse_key_has_the_windows_defaults() {
        let mut e = engine();
        e.mem.set_cstr(0x1000, br"control panel\mouse");
        e.mem.set_cstr(0x1040, b"MouseSensitivity");
        e.mem.set_cstr(0x1060, br"Software\Other");
        assert_eq!(
            reg_open_key_ex_a(
                &mut e,
                HKEY_CURRENT_USER,
                Ptr::new(0x1060),
                0,
                0,
                Ptr::new(0x1100)
            ),
            ERROR_FILE_NOT_FOUND
        );
        assert_eq!(
            reg_open_key_ex_a(
                &mut e,
                0x8000_0002,
                Ptr::new(0x1000),
                0,
                0,
                Ptr::new(0x1100)
            ),
            ERROR_FILE_NOT_FOUND
        );
        assert_eq!(
            reg_open_key_ex_a(
                &mut e,
                HKEY_CURRENT_USER,
                Ptr::new(0x1000),
                0,
                0,
                Ptr::new(0x1100)
            ),
            ERROR_SUCCESS
        );
        let key = e.mem.u32(0x1100);
        let (kind, data, size) = (Ptr::new(0x1104), Ptr::new(0x1200), Ptr::new(0x1108));
        e.mem.set_u32(size.addr(), 2);
        assert_eq!(
            reg_query_value_ex_a(&mut e, key, Ptr::new(0x1040), 0, kind, data, size),
            ERROR_MORE_DATA
        );
        assert_eq!(e.mem.u32(size.addr()), 3);
        e.mem.set_u32(size.addr(), 10);
        assert_eq!(
            reg_query_value_ex_a(&mut e, key, Ptr::new(0x1040), 0, kind, data, size),
            ERROR_SUCCESS
        );
        assert_eq!(e.mem.cstr(data.addr()), b"10");
        assert_eq!(
            (e.mem.u32(kind.addr()), e.mem.u32(size.addr())),
            (REG_SZ, 3)
        );
        assert_eq!(
            reg_query_value_ex_a(&mut e, key, Ptr::new(0x1060), 0, kind, data, size),
            ERROR_FILE_NOT_FOUND
        );
        assert_eq!(reg_close_key(&mut e, key), ERROR_SUCCESS);
        assert_eq!(reg_close_key(&mut e, key), ERROR_INVALID_HANDLE);
    }

    #[test]
    fn ini_values_are_found_by_section_and_key() {
        let text = b"; comment\r\n[General]\r\n sLanguage = ENGLISH \r\n[Controls]\r\nForward=\"0011FF00\"\r\nBack=1F\r\n";
        assert_eq!(
            ini_value(text, b"controls", b"forward").unwrap(),
            b"0011FF00"
        );
        assert_eq!(ini_value(text, b"Controls", b"Back").unwrap(), b"1F");
        assert_eq!(
            ini_value(text, b"General", b"sLanguage").unwrap(),
            b"ENGLISH"
        );
        assert_eq!(ini_value(text, b"General", b"Back"), None);
    }

    /// Windows' `GetPrivateProfileIntA` answers, measured on the player's machine (2026-10-21).
    #[test]
    fn profile_ints_read_as_windows_does() {
        let measured: &[(&[u8], u32)] = &[
            (b"-5", 0xffff_fffb),
            (b"-0", 0),
            (b"0x10", 16),
            (b"0X1F", 0),
            (b"-0x10", 0xffff_fff0),
            (b"12abc", 12),
            (b"  7", 7),
            (b"+3", 3),
            (b"4294967295", 0xffff_ffff),
            (b"4294967296", 0),
            (b"99999999999", 1_215_752_191),
            (b"1e3", 1),
            (b"0", 0),
            (b"08", 8),
            (b"010", 10),
            (b"-", 0),
            (b"x5", 0),
            (b"0b101", 5),
            (b"0o17", 15),
            (b"0x", 0),
            (b"0xg", 0),
            (b"-  5", 0),
            (b" -5", 0xffff_fffb),
            (b"0b2", 0),
            (b"0x10abc", 0x10abc),
            (b"0xFFFFFFFF1", 0xffff_fff1),
            (b"0B11", 0),
            (b"0O7", 0),
            (b"\t9", 9),
            (b"--5", 0),
            (b"+-5", 0),
            (b"- 5", 0),
            (b"00x10", 0),
            (b"0x-5", 0),
            (b"0b", 0),
            (b"0o8", 0),
            (b"5 6", 5),
        ];
        for &(value, want) in measured {
            assert_eq!(profile_int(value), want, "{:?}", std::str::from_utf8(value));
        }
    }

    /// A missing file gives the default, cut to the buffer; writing changes nothing and answers 1.
    #[test]
    fn profile_strings_fall_back_to_the_default() {
        let mut e = engine();
        e.mem.set_cstr(0x1000, b"Controls");
        e.mem.set_cstr(0x1020, b"Forward");
        e.mem.set_cstr(0x1040, b"default");
        e.mem.set_cstr(0x1060, br"Z:\no\such\file.ini");
        let n = get_private_profile_string_a(
            &mut e,
            Ptr::new(0x1000),
            Ptr::new(0x1020),
            Ptr::new(0x1040),
            Ptr::new(0x1100),
            4,
            Ptr::new(0x1060),
        );
        assert_eq!(n, 3);
        assert_eq!(e.mem.cstr(0x1100), b"def");
        let n = get_private_profile_string_a(
            &mut e,
            Ptr::new(0x1000),
            Ptr::new(0x1020),
            Ptr::NULL,
            Ptr::new(0x1100),
            0x40,
            Ptr::new(0x1060),
        );
        assert_eq!((n, e.mem.u8(0x1100)), (0, 0));
        assert_eq!(
            write_private_profile_string_a(
                &mut e,
                Ptr::new(0x1000),
                Ptr::new(0x1020),
                Ptr::new(0x1040),
                Ptr::new(0x1060)
            ),
            1
        );
    }
}

#[cfg(test)]
mod tests_counted_semaphore {
    use super::*;

    #[test]
    fn wait_and_release_go_through_the_handle_and_the_count() {
        let mut e = Engine::new();
        for f in [
            SLOT_WAIT_FOR_SINGLE_OBJECT,
            SLOT_RELEASE_SEMAPHORE,
            COUNT_DOWN,
            COUNT_UP,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record + 4, 0x44);
        e.call_log = Some(vec![]);
        e.call(0x0044_24e0, &args![record]);
        e.call(0x0044_2550, &args![record]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0044_24e0 && *a != 0x0044_2550)
            .collect();
        assert_eq!(
            log,
            [
                (SLOT_WAIT_FOR_SINGLE_OBJECT, vec![0x44, 0xffff_ffff]),
                (COUNT_DOWN, vec![record]),
                (SLOT_RELEASE_SEMAPHORE, vec![0x44, 1, 0]),
                (COUNT_UP, vec![record]),
            ]
        );
    }
}

#[cfg(test)]
mod tests_allocations {
    use super::*;

    /// Not a check: the host heap allocations of the kernel imports a frame calls most (on the
    /// main thread, nothing to wait for), for the frame-rate work (`cargo test --release -p
    /// engine-core --lib allocations_per -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_kernel() {
        use crate::alloc_count::{per_run, row};
        let mut e = Engine::new();
        let table = imports();
        let slot = |name: &str| {
            0x7f00_0000 + 4 * table.iter().position(|(_, n, _)| *n == name).unwrap() as u32
        };
        for (i, (_, _, f)) in table.iter().enumerate() {
            e.register(0x7f00_0000 + 4 * i as u32, *f);
        }
        let word = e.mem.alloc(0x40);
        let section = e.mem.alloc(0x18);
        e.call(slot("InitializeCriticalSection"), &[section]);
        let events = e.mem.alloc(8);
        for i in 0..2 {
            let h = e.call(slot("CreateEventA"), &[0, 1, 1, 0]).u32();
            e.mem.set_u32(events + 4 * i, h);
        }
        let event = e.mem.u32(events);
        let tls = e.call(slot("TlsAlloc"), &[]).u32();
        let message = e.mem.alloc(0x1c);
        for (what, name, args) in [
            ("InterlockedIncrement", "InterlockedIncrement", vec![word]),
            (
                "InterlockedCompareExchange",
                "InterlockedCompareExchange",
                vec![word, 0, 0],
            ),
            (
                "EnterCriticalSection",
                "EnterCriticalSection",
                vec![section],
            ),
            (
                "LeaveCriticalSection",
                "LeaveCriticalSection",
                vec![section],
            ),
            ("TlsSetValue", "TlsSetValue", vec![tls, 5]),
            ("TlsGetValue", "TlsGetValue", vec![tls]),
            ("GetCurrentThreadId", "GetCurrentThreadId", vec![]),
            (
                "QueryPerformanceCounter",
                "QueryPerformanceCounter",
                vec![word],
            ),
            ("timeGetTime", "timeGetTime", vec![]),
            ("GetTickCount", "GetTickCount", vec![]),
            ("SetEvent (manual reset)", "SetEvent", vec![event]),
            (
                "WaitForSingleObject, signalled",
                "WaitForSingleObject",
                vec![event, 0],
            ),
            (
                "WaitForMultipleObjects of 2, signalled",
                "WaitForMultipleObjects",
                vec![2, events, 0, 0],
            ),
            ("GetAsyncKeyState", "GetAsyncKeyState", vec![0x09]),
            (
                "PeekMessageA, none",
                "PeekMessageA",
                vec![message, 0, 0, 0, 1],
            ),
        ] {
            let f = slot(name);
            row(
                &format!("kernel {what}"),
                per_run(1000, || {
                    e.call(f, &args);
                }),
            );
        }
    }
}

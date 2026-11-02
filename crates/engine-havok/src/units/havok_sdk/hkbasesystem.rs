//! `Havok SDK/hkbasesystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The base system keeps a linked list of registered singletons (head at `01268790`; each node
//! has a creator function at `+4`, the next node at `+8` and the address of the singleton's
//! slot at `+0xc`) and a handful of globals (the "initialised" byte `01268414`, two objects
//! kept alive in `01268360` and `01268364`). The classes behind the nodes and the two objects
//! are described by offset only (unconfirmed).
//!
//! The memory router (`00c85750`) hands out allocators: `+0xc` is the one the arrays grow and
//! free through, `+0x10` the one the base system takes small objects from (slot `+4`: allocate
//! `(size)`; slot `+0x10`: free `(block, bytes)`).
//!
//! The arrays are the exe's `hkArray` words: data pointer, size, and a capacity whose bit 31 says
//! the buffer is not owned (bits 0..29 are the capacity).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;

/// `hkMemoryRouter::replaceInstance` (`00c85730`, cdecl `(router)`).
const REPLACE_INSTANCE: u32 = 0x00c8_5730;
/// `hkMemoryRouter::getInstancePtr` (`00c85760`).
const GET_INSTANCE_PTR: u32 = 0x00c8_5760;
/// `hkMonitorStream::init` (`00cb0db0`) and `hkMonitorStream::quit` (`00cb0df0`, cdecl `(stream)`).
const MONITOR_STREAM_INIT: u32 = 0x00cb_0db0;
const MONITOR_STREAM_QUIT: u32 = 0x00cb_0df0;
/// Import slot of `TlsGetValue` and of `GetSystemInfo`.
const API_TLS_GET_VALUE: u32 = 0x00fd_f044;
const API_GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
/// The exe's global with the TLS index of the monitor stream.
const MONITOR_TLS_INDEX: u32 = 0x0126_835c;
/// `hkArrayUtil` growth routines: `00c90940` (cdecl `(allocator, array, element size)`) and
/// `_reserve` `00c908c0` (cdecl `(allocator, array, capacity, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::removeReferenceLockUnchecked` (`00c903b0`, thiscall) and
/// `hkReferencedObject::setLockMode` (`00c90410`, cdecl `(mode)`); `removeReference` (`00c905b0`,
/// thiscall).
const REMOVE_REFERENCE_UNCHECKED: u32 = 0x00c9_03b0;
const SET_LOCK_MODE: u32 = 0x00c9_0410;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The constructor of the 0x28-byte object kept in `01268360` (`00d2b640`, thiscall, two words).
const OBJECT_CONSTRUCT: u32 = 0x00d2_b640;
/// Constructors the file system opens streams with: `00d2bd60` (thiscall, one word) and
/// `00d2bc30` (thiscall, `(reader, size)`) for readers, `00d2bf70` and `00d27460` for writers.
const STDIO_READER_CONSTRUCT: u32 = 0x00d2_bd60;
const BUFFERED_READER_CONSTRUCT: u32 = 0x00d2_bc30;
const STDIO_WRITER_CONSTRUCT: u32 = 0x00d2_bf70;
const BUFFERED_WRITER_CONSTRUCT: u32 = 0x00d2_7460;

/// The "initialised" byte of the base system.
const INITIALISED: u32 = 0x0126_8414;
/// Head of the singleton list.
const SINGLETON_LIST: u32 = 0x0126_8790;
/// The two objects the base system holds a reference to.
const OBJECT_01268360: u32 = 0x0126_8360;
const OBJECT_01268364: u32 = 0x0126_8364;
/// The object whose virtual slot `+0xc` the start-up calls last.
const STARTUP_OBJECT: u32 = 0x0126_8410;
/// The "quit callback needs calling" byte and the callback pointer.
const QUIT_CALLBACK_PENDING: u32 = 0x0126_8a44;
const QUIT_CALLBACK: u32 = 0x011b_3dec;
/// The file system function pointers: list a directory (`011b3dbc`, cdecl `(a, b)`) and open
/// a file (`011b3dc0`, cdecl `(path, string buffer)`).
const LIST_DIRECTORY_POINTER: u32 = 0x011b_3dbc;
const OPEN_FILE_POINTER: u32 = 0x011b_3dc0;
/// The vtable written into the 8-byte object created by the start-up.
const SMALL_OBJECT_VTABLE: u32 = 0x010c_8eb8;
/// Bit 31 of an array's capacity word: the buffer is not owned.
const NOT_OWNED: u32 = 0x8000_0000;
/// Bits 0..29 of the capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The allocator at `+offset` of the current memory router.
fn router_allocator(e: &mut Engine, offset: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + offset)
}

/// Allocates `size` bytes through slot `+4` of the router's `+0x10` allocator and stores the
/// size in the 16-bit word at `+4` of the block.
fn allocate_sized(e: &mut Engine, size: u32) -> u32 {
    let allocator = router_allocator(e, 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// Gives back an array's buffer through slot `+0x10` of `allocator` when it owns it (bit 31 of
/// `capacity` clear): `(data, bytes)` with `bytes` = `(capacity & mask) * element_size`.
fn free_array_buffer(e: &mut Engine, allocator: u32, data: u32, capacity: u32, element_size: u32) {
    if capacity as i32 >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00cbb930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBaseSystem::initThread` (Xbox PDB): makes `router` the thread's memory router
/// (`hkMemoryRouter::replaceInstance`) and starts the thread's monitor stream
/// (`hkMonitorStream::init`). Returns 0.
pub fn hk_base_system_init_thread(e: &mut Engine, router: u32) -> u32 {
    e.call(REPLACE_INSTANCE, &args![router]);
    e.call(MONITOR_STREAM_INIT, &args![]);
    0
}

// Translated from 00cbb960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the number of processors, at most 4, to `result`: `GetSystemInfo`'s
/// `dwNumberOfProcessors` (`+0x14` of the 0x24-byte structure), compared signed. Unnamed in the
/// engine map.
pub fn fn_00cbb960(e: &mut Engine, result: u32) {
    e.with_stack(0x24, |e, info| {
        e.call(API_GET_SYSTEM_INFO, &args![info]);
        let processors = e.mem.i32(info.addr() + 0x14);
        e.mem.set_i32(result, processors.min(4));
    });
}

// Translated from 00cbb990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBaseSystem::quitThread` (Xbox PDB): ends the thread's monitor stream (the `TlsGetValue`
/// of the index in `0126835c`, when not null) and, when the thread has a memory router, replaces
/// it with null. Returns 0.
pub fn hk_base_system_quit_thread(e: &mut Engine) -> u32 {
    let index = e.mem.u32(MONITOR_TLS_INDEX);
    let stream = e.call(API_TLS_GET_VALUE, &args![index]).u32();
    if stream != 0 {
        e.call(MONITOR_STREAM_QUIT, &args![stream]);
    }
    if e.call(GET_INSTANCE_PTR, &args![]).u32() != 0 {
        e.call(REPLACE_INSTANCE, &args![0u32]);
    }
    0
}

// Translated from 00cbb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the registered singletons. Each node of the list at `01268790` whose slot (`*(node +
/// 0xc)`) is still null and whose creator (`+4`) is set is created by calling the creator (no
/// arguments); a non-null answer fills the slot. A node whose creator answers null is unlinked,
/// its next pointer cleared, and put on a local array. The nodes on the array are then retried
/// from the last to the first, over and over until the array is empty; each that succeeds fills
/// its slot, is moved off the array (the last element takes its place) and is linked back at the
/// end of the list. Finally the array's buffer is freed through the router's `+0xc` allocator
/// when it grew (the array starts not owning a buffer).
pub fn fn_00cbb9f0(e: &mut Engine) {
    // The local array: data pointer, size, capacity word. The exe's frame (`SUB ESP,0xc`,
    // three pushes): 0x18 bytes, the array at +0xc (00cbba03).
    e.with_stack(0x18, |e, frame| {
        let array = frame.addr() + 0xc;
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let mut link = SINGLETON_LIST;
        let mut node = e.mem.u32(SINGLETON_LIST);
        if node != 0 {
            loop {
                let slot = e.mem.u32(node + 0xc);
                let creator = e.mem.u32(node + 4);
                if e.mem.u32(slot) == 0 && creator != 0 {
                    let created = e.call(creator, &args![]).u32();
                    if created == 0 {
                        // Creation failed: move the node onto the array.
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let size = e.mem.u32(array + 4);
                        if size == (e.mem.u32(array + 8) & CAPACITY_MASK) {
                            let allocator = e.mem.u32(router + 0xc);
                            e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
                        }
                        let size = e.mem.u32(array + 4);
                        let data = e.mem.u32(array);
                        e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), node);
                        e.mem.set_u32(array + 4, size.wrapping_add(1));
                        let following = e.mem.u32(node + 8);
                        let linked = e.mem.u32(link);
                        e.mem.set_u32(linked + 8, 0);
                        e.mem.set_u32(link, following);
                        node = following;
                        if node == 0 {
                            break;
                        }
                        continue;
                    }
                    e.mem.set_u32(slot, created);
                }
                link = node + 8;
                node = e.mem.u32(link);
                if node == 0 {
                    break;
                }
            }
            let mut size = e.mem.u32(array + 4);
            while size != 0 {
                let mut index = size.wrapping_sub(1) as i32;
                while index >= 0 {
                    let data = e.mem.u32(array);
                    let retry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                    let creator = e.mem.u32(retry + 4);
                    let created = e.call(creator, &args![]).u32();
                    if created != 0 {
                        let slot = e.mem.u32(retry + 0xc);
                        e.mem.set_u32(slot, created);
                        e.mem.set_u32(link, retry);
                        let remaining = e.mem.u32(array + 4).wrapping_sub(1);
                        link = retry + 8;
                        e.mem.set_u32(array + 4, remaining);
                        if remaining != index as u32 {
                            let data = e.mem.u32(array);
                            let moved = e.mem.u32(data.wrapping_add(remaining.wrapping_mul(4)));
                            e.mem
                                .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
                        }
                    }
                    index = index.wrapping_sub(1);
                }
                size = e.mem.u32(array + 4);
            }
        }
        let allocator = router_allocator(e, 0xc);
        let capacity = e.mem.u32(array + 8);
        let data = e.mem.u32(array);
        e.mem.set_u32(array + 4, 0);
        free_array_buffer(e, allocator, data, capacity, 4);
    });
}

// Translated from 00cbbb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the base system (looks like `hkBaseSystem::init(router, error report function, error
/// report object)`; the engine map does not name it). Does nothing when the byte at `01268414`
/// is set. Otherwise it starts the thread (`00cbb930` on `router`), makes the 8-byte object with
/// the vtable `010c8eb8` (size word 8 and count word 1 at `+4`, `+6`) and keeps it in
/// `01268364`, makes the 0x28-byte object with the constructor `00d2b640` (given
/// `error_report_function` and `error_report_object`) and keeps it in `01268360` (each time
/// letting go of what was there), creates the registered singletons (`00cbb9f0`), calls slot
/// `+0xc` of the object at `01268410`, and sets the byte. Returns 0.
pub fn fn_00cbbb10(
    e: &mut Engine,
    router: u32,
    error_report_function: u32,
    error_report_object: u32,
) -> u32 {
    if e.mem.u8(INITIALISED) == 0 {
        hk_base_system_init_thread(e, router);
        let small = allocate_sized(e, 8);
        e.mem.set_u16(small + 6, 1);
        e.mem.set_u32(small, SMALL_OBJECT_VTABLE);
        let previous = e.mem.u32(OBJECT_01268364);
        if previous != 0 {
            e.call(REMOVE_REFERENCE_UNCHECKED, &args![previous]);
        }
        e.mem.set_u32(OBJECT_01268364, small);
        let block = allocate_sized(e, 0x28);
        let object = e
            .call(
                OBJECT_CONSTRUCT,
                &args![block, error_report_function, error_report_object],
            )
            .u32();
        let previous = e.mem.u32(OBJECT_01268360);
        if previous != 0 {
            e.call(REMOVE_REFERENCE_UNCHECKED, &args![previous]);
        }
        e.mem.set_u32(OBJECT_01268360, object);
        fn_00cbb9f0(e);
        let startup = e.mem.u32(STARTUP_OBJECT);
        e.vcall(startup, 0xc, &args![]);
        e.mem.set_u8(INITIALISED, 1);
    }
    0
}

// Translated from 00cbbbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets go of the registered singletons: collects the nodes of the list at `01268790` on a local
/// array (128 elements of its own storage, grown through the router's `+0x10` allocator) and
/// walks it from the last node to the first; where the slot (`*(node + 0xc)`) is not null it calls
/// `removeReferenceLockUnchecked` on the singleton and clears the slot. The array's buffer is
/// freed through the router's `+0x10` allocator when it grew.
pub fn fn_00cbbbd0(e: &mut Engine) {
    // The local array (12 header bytes, then 512 bytes of storage). The exe's frame
    // (`SUB ESP,0x20c`, one push): 0x210 bytes, the header at +0x4 and the storage at +0x10
    // (00cbbbdd).
    e.with_stack(0x210, |e, frame| {
        let array = frame.addr() + 4;
        e.mem.set_u32(array, array + 12);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0080);
        let mut node = e.mem.u32(SINGLETON_LIST);
        while node != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let size = e.mem.u32(array + 4);
            if size == (e.mem.u32(array + 8) & CAPACITY_MASK) {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
            }
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), node);
            e.mem.set_u32(array + 4, size.wrapping_add(1));
            node = e.mem.u32(node + 8);
        }
        let mut index = e.mem.u32(array + 4).wrapping_sub(1) as i32;
        while index >= 0 {
            let data = e.mem.u32(array);
            let entry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let slot = e.mem.u32(entry + 0xc);
            let singleton = e.mem.u32(slot);
            if singleton != 0 {
                e.call(REMOVE_REFERENCE_UNCHECKED, &args![singleton]);
                let data = e.mem.u32(array);
                let entry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                let slot = e.mem.u32(entry + 0xc);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_sub(1);
        }
        let allocator = router_allocator(e, 0x10);
        let capacity = e.mem.u32(array + 8);
        let data = e.mem.u32(array);
        e.mem.set_u32(array + 4, 0);
        free_array_buffer(e, allocator, data, capacity, 4);
    });
}

// Translated from 00cbbcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shuts the base system down (looks like `hkBaseSystem::quit`; unnamed in the engine map).
/// Does nothing when the byte at `01268414` is clear. Otherwise: `setLockMode(0)`, lets go of
/// the singletons (`00cbbbd0`), calls the quit callback at `011b3dec` (when the byte at
/// `01268a44` is set and the pointer is not null, then clears the byte), lets go of and clears
/// the objects in `01268360` and `01268364`, ends the thread (`00cbb990`) and clears the
/// byte. Returns 0.
pub fn fn_00cbbcb0(e: &mut Engine) -> u32 {
    if e.mem.u8(INITIALISED) != 0 {
        e.call(SET_LOCK_MODE, &args![0u32]);
        fn_00cbbbd0(e);
        if e.mem.u8(QUIT_CALLBACK_PENDING) != 0 {
            let callback = e.mem.u32(QUIT_CALLBACK);
            if callback != 0 {
                e.call(callback, &args![]);
                e.mem.set_u8(QUIT_CALLBACK_PENDING, 0);
            }
        }
        let object = e.mem.u32(OBJECT_01268360);
        if object != 0 {
            e.call(REMOVE_REFERENCE_UNCHECKED, &args![object]);
        }
        let small = e.mem.u32(OBJECT_01268364);
        e.mem.set_u32(OBJECT_01268360, 0);
        if small != 0 {
            e.call(REMOVE_REFERENCE_UNCHECKED, &args![small]);
        }
        e.mem.set_u32(OBJECT_01268364, 0);
        hk_base_system_quit_thread(e);
        e.mem.set_u8(INITIALISED, 0);
    }
    0
}

// Translated from 00cbbd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativeFileSystem::listDirectory` (Xbox PDB): calls the function pointer at `011b3dbc` with
/// `(first, second)` and passes its `EAX` result through (the decompiler shows `void`, but the
/// code returns straight after the call).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_native_file_system_list_directory(
    e: &mut Engine,
    _this: u32,
    first: u32,
    second: u32,
) -> u32 {
    let function = e.mem.u32(LIST_DIRECTORY_POINTER);
    e.call(function, &args![first, second]).u32()
}

// Translated from 00cbbd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears and frees a char array (`this` = the array's data/size/capacity words; the engine map
/// does not name it): sets the size to 0, gives the buffer back through the router's `+0xc`
/// allocator when it owns it (`(data, capacity & mask)`), then resets the capacity word to
/// `0x80000000` and the data pointer to null.
pub fn fn_00cbbd40(e: &mut Engine, this: u32) {
    let allocator = router_allocator(e, 0xc);
    e.mem.set_u32(this + 4, 0);
    let capacity = e.mem.u32(this + 8);
    let data = e.mem.u32(this);
    free_array_buffer(e, allocator, data, capacity, 1);
    e.mem.set_u32(this + 8, NOT_OWNED);
    e.mem.set_u32(this, 0);
}

// Translated from 00cbbd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a char array with 128 bytes of its own storage at `this + 0xc` holding one zero
/// byte (looks like an `hkStringBuf` constructor; unconfirmed). When the capacity is below 1
/// (never, with the storage in place) it grows the buffer to `max(2 * capacity, 1)` through
/// `_reserve` on the router's `+0xc` allocator. Returns `this`.
pub fn fn_00cbbd80(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, this + 0xc);
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 8, 0x8000_0080);
    let allocator = router_allocator(e, 0xc);
    let capacity = (e.mem.u32(this + 8) & CAPACITY_MASK) as i32;
    if capacity < 1 {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = doubled.max(1);
        e.call(ARRAY_RESERVE, &args![allocator, this, wanted, 1u32]);
    }
    let data = e.mem.u32(this);
    e.mem.set_u32(this + 4, 1);
    e.mem.set_u8(data, 0);
    let data = e.mem.u32(this);
    e.mem.set_u8(data, 0);
    this
}

/// The temporary string buffer of the file-system openers ends the same way each time: its
/// buffer is freed through the router's `+0xc` allocator when it owns it.
fn free_string_buffer(e: &mut Engine, buffer: u32) {
    let allocator = router_allocator(e, 0xc);
    let capacity = e.mem.u32(buffer + 8);
    let data = e.mem.u32(buffer);
    e.mem.set_u32(buffer + 4, 0);
    free_array_buffer(e, allocator, data, capacity, 1);
}

// Translated from 00cbbde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativeFileSystem::openReader` (Xbox PDB): builds a string buffer (`00cbbd80`), a 0x14-byte
/// object (size word 0x14 at `+4`), asks the function pointer at `011b3dc0` for the stream of
/// `path` into the buffer, constructs the stdio reader on it (`00d2bd60`) and calls its virtual
/// slot `+0x18` with the address of a temporary byte. A non-zero byte at the answered address
/// returns the reader as is; a zero byte wraps it in a 0x24-byte buffered reader of 0x1000 bytes
/// (`00d2bc30`), lets go of the stdio reader (`00c905b0`) and returns the buffered one. The
/// string buffer is freed either way.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_native_file_system_open_reader(e: &mut Engine, _this: u32, path: u32) -> u32 {
    // The exe's frame (`SUB ESP,0x90`, one push): 0x94 bytes, the answer byte at +0x7
    // (00cbbe31) and the string buffer at +0x8 (00cbbde7).
    e.with_stack(0x94, |e, frame| {
        let buffer = frame.addr() + 8;
        let temporary = frame.addr() + 7;
        fn_00cbbd80(e, buffer);
        let block = allocate_sized(e, 0x14);
        let function = e.mem.u32(OPEN_FILE_POINTER);
        let stream = e.call(function, &args![path, buffer]).u32();
        let reader = e.call(STDIO_READER_CONSTRUCT, &args![block, stream]).u32();
        let answered = e.vcall(reader, 0x18, &args![temporary]).u32();
        let result = if e.mem.u8(answered) == 0 {
            let block = allocate_sized(e, 0x24);
            let buffered = e
                .call(BUFFERED_READER_CONSTRUCT, &args![block, reader, 0x1000u32])
                .u32();
            e.call(REMOVE_REFERENCE, &args![reader]);
            buffered
        } else {
            reader
        };
        free_string_buffer(e, buffer);
        result
    })
}

// Translated from 00cbbee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativeFileSystem::openWriter` (Xbox PDB): like the reader's opener, builds a string buffer,
/// a 0x10-byte object, asks `011b3dc0` for the stream of `path`, constructs the stdio writer
/// on it (`00d2bf70`), then a 0x1c-byte `hkBufferedStreamWriter` of 0x1000 bytes over it
/// (`00d27460`), lets go of the stdio writer (`00c905b0`), frees the string buffer and returns
/// the buffered writer.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_native_file_system_open_writer(e: &mut Engine, _this: u32, path: u32) -> u32 {
    // The exe's frame (`SUB ESP,0x8c`, two pushes): 0x94 bytes, the string buffer at +0x8
    // (00cbbee8).
    e.with_stack(0x94, |e, frame| {
        let buffer = frame.addr() + 8;
        fn_00cbbd80(e, buffer);
        let block = allocate_sized(e, 0x10);
        let function = e.mem.u32(OPEN_FILE_POINTER);
        let stream = e.call(function, &args![path, buffer]).u32();
        let writer = e.call(STDIO_WRITER_CONSTRUCT, &args![block, stream]).u32();
        let block = allocate_sized(e, 0x1c);
        let buffered = e
            .call(BUFFERED_WRITER_CONSTRUCT, &args![block, writer, 0x1000u32])
            .u32();
        e.call(REMOVE_REFERENCE, &args![writer]);
        free_string_buffer(e, buffer);
        buffered
    })
}

// Translated from 00cbb9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 8 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6 and the vtable word `010c8ed4` at +0. Returns the block. Ghidra made no function here; the address is stored in the data word at 011b2028.
pub fn fn_00cbb9c0(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![8u32]).u32();
    e.mem.set_u16(block + 4, 8);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010c_8ed4);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_b930, hk_base_system_init_thread(u32) -> u32),
        entry!(0x00cb_b960, fn_00cbb960(u32)),
        entry!(0x00cb_b990, hk_base_system_quit_thread() -> u32),
        entry!(0x00cb_b9f0, fn_00cbb9f0()),
        entry!(0x00cb_bb10, fn_00cbbb10(u32, u32, u32) -> u32),
        entry!(0x00cb_bbd0, fn_00cbbbd0()),
        entry!(0x00cb_bcb0, fn_00cbbcb0() -> u32),
        entry!(0x00cb_bd20, hk_native_file_system_list_directory(u32, u32, u32) -> u32),
        entry!(0x00cb_bd40, fn_00cbbd40(u32)),
        entry!(0x00cb_bd80, fn_00cbbd80(u32) -> u32),
        entry!(0x00cb_bde0, hk_native_file_system_open_reader(u32, u32) -> u32),
        entry!(0x00cb_bee0, hk_native_file_system_open_writer(u32, u32) -> u32),
        entry!(0x00cb_b9c0, fn_00cbb9c0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Slots of the two fake allocators (`+0xc` and `+0x10` of the router): allocate (`+4`) and
    /// free (`+0x10`).
    const HEAP_ALLOCATE: u32 = 0x0900_0104;
    const HEAP_FREE: u32 = 0x0900_0110;
    const SMALL_ALLOCATE: u32 = 0x0900_0204;
    const SMALL_FREE: u32 = 0x0900_0210;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    /// A double that records its call and answers `answer`.
    fn double(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(called, _)| *called).collect()
    }

    /// Installs a memory router whose allocators (`+0xc` and `+0x10`) are objects with a vtable
    /// of fake slots. Returns (router, heap allocator, small allocator). The small allocator
    /// hands out `blocks` in order.
    fn install_router(e: &mut Engine, log: &Log, blocks: Vec<u32>) -> (u32, u32, u32) {
        let heap_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(heap_vtable + 4, HEAP_ALLOCATE);
        e.mem.set_u32(heap_vtable + 0x10, HEAP_FREE);
        let small_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(small_vtable + 4, SMALL_ALLOCATE);
        e.mem.set_u32(small_vtable + 0x10, SMALL_FREE);
        let heap = e.mem.alloc(8);
        e.mem.set_u32(heap, heap_vtable);
        let small = e.mem.alloc(8);
        e.mem.set_u32(small, small_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, heap);
        e.mem.set_u32(router + 0x10, small);
        double(e, log, MEMORY_ROUTER, router);
        double(e, log, HEAP_FREE, 0);
        double(e, log, SMALL_FREE, 0);
        let next = Cell::new(0usize);
        let allocate_log = Rc::clone(log);
        e.register_double(SMALL_ALLOCATE, move |_, a| {
            allocate_log.borrow_mut().push((SMALL_ALLOCATE, a.to_vec()));
            let block = blocks[next.get()];
            next.set(next.get() + 1);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        (router, heap, small)
    }

    /// A singleton list node: creator at `+4`, next at `+8`, slot address at `+0xc`.
    fn node(e: &mut Engine, creator: u32, slot: u32) -> u32 {
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node + 4, creator);
        e.mem.set_u32(node + 0xc, slot);
        node
    }

    /// The nodes of the list at `01268790`, in order.
    fn list(e: &Engine) -> Vec<u32> {
        let mut nodes = vec![];
        let mut node = e.mem.u32(SINGLETON_LIST);
        while node != 0 {
            nodes.push(node);
            node = e.mem.u32(node + 8);
        }
        nodes
    }

    fn link(e: &mut Engine, nodes: &[u32]) {
        e.mem
            .set_u32(SINGLETON_LIST, nodes.first().copied().unwrap_or(0));
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 8, pair[1]);
        }
    }

    /// A growth double standing in for `00c90940`: gives the array a buffer of `capacity`
    /// words (keeping what it holds) and clears bit 31.
    fn install_grow(e: &mut Engine, log: &Log, buffer: u32, capacity: u32) {
        let log = Rc::clone(log);
        e.register_double(ARRAY_GROW, move |e, a| {
            log.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let array = a[1];
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            for word in 0..size {
                let value = e.mem.u32(old + word * 4);
                e.mem.set_u32(buffer + word * 4, value);
            }
            e.mem.set_u32(array, buffer);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
    }

    #[test]
    fn init_thread_replaces_the_router_and_starts_the_monitor_stream() {
        let (mut e, log) = engine();
        double(&mut e, &log, REPLACE_INSTANCE, 7);
        double(&mut e, &log, MONITOR_STREAM_INIT, 7);
        let got = e.call(0x00cb_b930, &args![0x1234u32]).u32();
        assert_eq!(got, 0);
        assert_eq!(
            *log.borrow(),
            vec![
                (REPLACE_INSTANCE, vec![0x1234]),
                (MONITOR_STREAM_INIT, vec![])
            ]
        );
    }

    fn processors(count: u32) -> u32 {
        let (mut e, _log) = engine();
        e.register_double(API_GET_SYSTEM_INFO, move |e, a| {
            e.mem.set_u32(a[0] + 0x14, count);
            Ret::default()
        });
        let result = e.mem.alloc(4);
        e.call(0x00cb_b960, &args![result]);
        e.mem.u32(result)
    }

    #[test]
    fn processor_count_is_capped_at_four_and_compared_signed() {
        assert_eq!(processors(2), 2);
        assert_eq!(processors(3), 3);
        assert_eq!(processors(4), 4);
        assert_eq!(processors(16), 4);
        // Signed compare: a negative count is below 4 and stored as it is.
        assert_eq!(processors(0xffff_ffff), 0xffff_ffff);
    }

    #[test]
    fn quit_thread_ends_the_monitor_stream_and_the_router_only_when_present() {
        let (mut e, log) = engine();
        e.mem.set_u32(MONITOR_TLS_INDEX, 3);
        double(&mut e, &log, API_TLS_GET_VALUE, 0);
        double(&mut e, &log, GET_INSTANCE_PTR, 0);
        double(&mut e, &log, MONITOR_STREAM_QUIT, 0);
        double(&mut e, &log, REPLACE_INSTANCE, 0);
        assert_eq!(e.call(0x00cb_b990, &args![]).u32(), 0);
        assert_eq!(calls(&log, API_TLS_GET_VALUE), vec![vec![3]]);
        assert!(calls(&log, MONITOR_STREAM_QUIT).is_empty());
        assert!(calls(&log, REPLACE_INSTANCE).is_empty());

        double(&mut e, &log, API_TLS_GET_VALUE, 0x5000);
        double(&mut e, &log, GET_INSTANCE_PTR, 0x6000);
        assert_eq!(e.call(0x00cb_b990, &args![]).u32(), 0);
        assert_eq!(calls(&log, MONITOR_STREAM_QUIT), vec![vec![0x5000]]);
        assert_eq!(calls(&log, REPLACE_INSTANCE), vec![vec![0]]);
    }

    #[test]
    fn creating_the_singletons_retries_the_failed_ones_and_relinks_them_at_the_end() {
        let (mut e, log) = engine();
        let (_router, heap, _small) = install_router(&mut e, &log, vec![]);
        let slots = e.mem.alloc(0x10);
        let (slot_a, slot_b, slot_c) = (slots, slots + 4, slots + 8);
        e.mem.set_u32(slot_c, 0x333);
        let a = node(&mut e, 0x0900_0a01, slot_a);
        let b = node(&mut e, 0x0900_0b01, slot_b);
        let c = node(&mut e, 0x0900_0c01, slot_c);
        // `d` has no creator and an empty slot: it is skipped.
        let d = node(&mut e, 0, slots + 12);
        link(&mut e, &[a, b, c, d]);
        double(&mut e, &log, 0x0900_0a01, 0x111);
        let tries = Rc::new(Cell::new(0u32));
        let counted = Rc::clone(&tries);
        e.register_double(0x0900_0b01, move |_, _| {
            counted.set(counted.get() + 1);
            Ret {
                eax: if counted.get() == 1 { 0 } else { 0x222 },
                ..Ret::default()
            }
        });
        let buffer = e.mem.alloc(16);
        install_grow(&mut e, &log, buffer, 4);
        e.call(0x00cb_b9f0, &args![]);
        assert_eq!(tries.get(), 2);
        assert_eq!(e.mem.u32(slot_a), 0x111);
        assert_eq!(e.mem.u32(slot_b), 0x222);
        assert_eq!(e.mem.u32(slot_c), 0x333);
        assert_eq!(list(&e), vec![a, c, d, b]);
        assert_eq!(e.mem.u32(b + 8), 0);
        // The array grew once (the first failure) and its buffer went back to the heap allocator.
        assert_eq!(calls(&log, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&log, ARRAY_GROW)[0][0], heap);
        assert_eq!(calls(&log, ARRAY_GROW)[0][2], 4);
        assert_eq!(calls(&log, HEAP_FREE), vec![vec![heap, buffer, 16]]);
    }

    #[test]
    fn creating_the_singletons_swaps_the_last_element_in_when_a_retry_succeeds() {
        let (mut e, log) = engine();
        let (_router, heap, _small) = install_router(&mut e, &log, vec![]);
        let slots = e.mem.alloc(8);
        let x = node(&mut e, 0x0900_0a01, slots);
        let y = node(&mut e, 0x0900_0b01, slots + 4);
        link(&mut e, &[x, y]);
        // x fails first and succeeds on its first retry; y fails twice and then succeeds.
        let x_tries = Rc::new(Cell::new(0u32));
        let counted = Rc::clone(&x_tries);
        e.register_double(0x0900_0a01, move |_, _| {
            counted.set(counted.get() + 1);
            Ret {
                eax: if counted.get() == 1 { 0 } else { 0xaa },
                ..Ret::default()
            }
        });
        let y_tries = Rc::new(Cell::new(0u32));
        let counted = Rc::clone(&y_tries);
        e.register_double(0x0900_0b01, move |_, _| {
            counted.set(counted.get() + 1);
            Ret {
                eax: if counted.get() < 3 { 0 } else { 0xbb },
                ..Ret::default()
            }
        });
        let buffer = e.mem.alloc(16);
        install_grow(&mut e, &log, buffer, 4);
        // The array sits at +0xc of the exe's 0x18-byte frame.
        let frame = e.next_stack_local(0x18);
        e.call(0x00cb_b9f0, &args![]);
        assert_eq!(calls(&log, ARRAY_GROW)[0][1], frame + 0xc);
        assert_eq!(e.mem.u32(slots), 0xaa);
        assert_eq!(e.mem.u32(slots + 4), 0xbb);
        // Retries go from the last element to the first: y fails (try 2), x succeeds (its try 2)
        // and y moves down to index 0, where it is tried again (try 3).
        assert_eq!(x_tries.get(), 2);
        assert_eq!(y_tries.get(), 3);
        assert_eq!(list(&e), vec![x, y]);
        assert_eq!(e.mem.u32(y + 8), 0);
        assert_eq!(calls(&log, HEAP_FREE), vec![vec![heap, buffer, 16]]);
    }

    #[test]
    fn creating_the_singletons_of_an_empty_list_frees_nothing() {
        let (mut e, log) = engine();
        install_router(&mut e, &log, vec![]);
        e.mem.set_u32(SINGLETON_LIST, 0);
        e.call(0x00cb_b9f0, &args![]);
        assert_eq!(addresses(&log), vec![MEMORY_ROUTER]);
    }

    #[test]
    fn start_up_makes_the_objects_and_runs_once() {
        let (mut e, log) = engine();
        let small_block = e.mem.alloc(8);
        let big_block = e.mem.alloc(0x28);
        let (_router, _heap, small) = install_router(&mut e, &log, vec![small_block, big_block]);
        e.mem.set_u32(SINGLETON_LIST, 0);
        double(&mut e, &log, REPLACE_INSTANCE, 0);
        double(&mut e, &log, MONITOR_STREAM_INIT, 0);
        let made = e.mem.alloc(0x28);
        double(&mut e, &log, OBJECT_CONSTRUCT, made);
        double(&mut e, &log, REMOVE_REFERENCE_UNCHECKED, 0);
        let startup_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(startup_vtable + 0xc, 0x0900_0f0c);
        let startup = e.mem.alloc(8);
        e.mem.set_u32(startup, startup_vtable);
        e.mem.set_u32(STARTUP_OBJECT, startup);
        double(&mut e, &log, 0x0900_0f0c, 0);
        e.mem.set_u32(OBJECT_01268364, 0x7001);
        e.mem.set_u32(OBJECT_01268360, 0x7002);

        assert_eq!(
            e.call(0x00cb_bb10, &args![0x55u32, 0x66u32, 0x77u32]).u32(),
            0
        );
        assert_eq!(e.mem.u8(INITIALISED), 1);
        assert_eq!(e.mem.u32(OBJECT_01268364), small_block);
        assert_eq!(e.mem.u32(OBJECT_01268360), made);
        assert_eq!(e.mem.u32(small_block), SMALL_OBJECT_VTABLE);
        assert_eq!(e.mem.u16(small_block + 4), 8);
        assert_eq!(e.mem.u16(small_block + 6), 1);
        assert_eq!(e.mem.u16(big_block + 4), 0x28);
        assert_eq!(calls(&log, REPLACE_INSTANCE), vec![vec![0x55]]);
        assert_eq!(
            calls(&log, SMALL_ALLOCATE),
            vec![vec![small, 8], vec![small, 0x28]]
        );
        assert_eq!(
            calls(&log, REMOVE_REFERENCE_UNCHECKED),
            vec![vec![0x7001], vec![0x7002]]
        );
        assert_eq!(
            calls(&log, OBJECT_CONSTRUCT),
            vec![vec![big_block, 0x66, 0x77]]
        );
        assert_eq!(calls(&log, 0x0900_0f0c), vec![vec![startup]]);
        // A second start does nothing.
        let before = log.borrow().len();
        assert_eq!(
            e.call(0x00cb_bb10, &args![0x55u32, 0x66u32, 0x77u32]).u32(),
            0
        );
        assert_eq!(log.borrow().len(), before);
    }

    #[test]
    fn start_up_with_nothing_to_let_go_of_skips_the_removals() {
        let (mut e, log) = engine();
        let small_block = e.mem.alloc(8);
        let big_block = e.mem.alloc(0x28);
        install_router(&mut e, &log, vec![small_block, big_block]);
        double(&mut e, &log, REPLACE_INSTANCE, 0);
        double(&mut e, &log, MONITOR_STREAM_INIT, 0);
        double(&mut e, &log, OBJECT_CONSTRUCT, 0x9000);
        double(&mut e, &log, REMOVE_REFERENCE_UNCHECKED, 0);
        let startup_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(startup_vtable + 0xc, 0x0900_0f0c);
        let startup = e.mem.alloc(8);
        e.mem.set_u32(startup, startup_vtable);
        e.mem.set_u32(STARTUP_OBJECT, startup);
        double(&mut e, &log, 0x0900_0f0c, 0);
        e.call(0x00cb_bb10, &args![1u32, 2u32, 3u32]);
        assert!(calls(&log, REMOVE_REFERENCE_UNCHECKED).is_empty());
        assert_eq!(e.mem.u32(OBJECT_01268360), 0x9000);
    }

    #[test]
    fn releasing_the_singletons_walks_the_nodes_from_the_last_and_clears_the_slots() {
        let (mut e, log) = engine();
        install_router(&mut e, &log, vec![]);
        double(&mut e, &log, REMOVE_REFERENCE_UNCHECKED, 0);
        let slots = e.mem.alloc(0x10);
        e.mem.set_u32(slots, 0x1001);
        e.mem.set_u32(slots + 8, 0x1003);
        let a = node(&mut e, 0, slots);
        let b = node(&mut e, 0, slots + 4);
        let c = node(&mut e, 0, slots + 8);
        link(&mut e, &[a, b, c]);
        // The array header at +0x4 and its storage at +0x10 of the exe's 0x210-byte frame (read
        // after the call: the ended frame's bytes stay on the thread's stack).
        let frame = e.next_stack_local(0x210);
        e.call(0x00cb_bbd0, &args![]);
        assert_eq!(e.mem.u32(frame + 4), frame + 0x10);
        assert_eq!(e.mem.u32(frame + 0x10), a);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE_UNCHECKED),
            vec![vec![0x1003], vec![0x1001]]
        );
        assert_eq!(e.mem.u32(slots), 0);
        assert_eq!(e.mem.u32(slots + 4), 0);
        assert_eq!(e.mem.u32(slots + 8), 0);
        // The array's own storage held all three nodes: nothing grew and nothing is freed.
        assert!(calls(&log, ARRAY_GROW).is_empty());
        assert!(calls(&log, SMALL_FREE).is_empty());
    }

    #[test]
    fn releasing_more_singletons_than_the_local_storage_holds_grows_the_array() {
        let (mut e, log) = engine();
        let (_router, _heap, small) = install_router(&mut e, &log, vec![]);
        double(&mut e, &log, REMOVE_REFERENCE_UNCHECKED, 0);
        let buffer = e.mem.alloc(0x400);
        install_grow(&mut e, &log, buffer, 256);
        let slots = e.mem.alloc(130 * 4);
        let nodes: Vec<u32> = (0..130u32)
            .map(|i| node(&mut e, 0, slots + i * 4))
            .collect();
        e.mem.set_u32(slots + 129 * 4, 0x2002);
        link(&mut e, &nodes);
        e.call(0x00cb_bbd0, &args![]);
        // The 129th node found the 128-element storage full: the growth ran once, with the
        // `+0x10` allocator.
        assert_eq!(calls(&log, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&log, ARRAY_GROW)[0][0], small);
        assert_eq!(calls(&log, ARRAY_GROW)[0][2], 4);
        assert_eq!(calls(&log, REMOVE_REFERENCE_UNCHECKED), vec![vec![0x2002]]);
        assert_eq!(e.mem.u32(slots + 129 * 4), 0);
        assert_eq!(calls(&log, SMALL_FREE), vec![vec![small, buffer, 256 * 4]]);
    }

    #[test]
    fn shutdown_does_nothing_unless_started() {
        let (mut e, log) = engine();
        install_router(&mut e, &log, vec![]);
        assert_eq!(e.call(0x00cb_bcb0, &args![]).u32(), 0);
        assert!(log.borrow().is_empty());
    }

    /// The shutdown with everything in place: `has_callback` says whether the quit callback
    /// pointer is set.
    fn shut_down(has_callback: bool) -> (Engine, Log) {
        let (mut e, log) = engine();
        install_router(&mut e, &log, vec![]);
        e.mem.set_u8(INITIALISED, 1);
        e.mem.set_u8(QUIT_CALLBACK_PENDING, 1);
        e.mem
            .set_u32(QUIT_CALLBACK, if has_callback { 0x0900_0e01 } else { 0 });
        e.mem.set_u32(OBJECT_01268360, 0x4001);
        e.mem.set_u32(OBJECT_01268364, 0x4002);
        e.mem.set_u32(MONITOR_TLS_INDEX, 1);
        double(&mut e, &log, SET_LOCK_MODE, 0);
        double(&mut e, &log, REMOVE_REFERENCE_UNCHECKED, 0);
        double(&mut e, &log, 0x0900_0e01, 0);
        double(&mut e, &log, API_TLS_GET_VALUE, 0);
        double(&mut e, &log, GET_INSTANCE_PTR, 0);
        e.call(0x00cb_bcb0, &args![]);
        (e, log)
    }

    #[test]
    fn shutdown_calls_the_quit_callback_and_lets_go_of_both_objects() {
        let (e, log) = shut_down(true);
        assert_eq!(
            addresses(&log),
            vec![
                SET_LOCK_MODE,
                MEMORY_ROUTER,
                0x0900_0e01,
                REMOVE_REFERENCE_UNCHECKED,
                REMOVE_REFERENCE_UNCHECKED,
                API_TLS_GET_VALUE,
                GET_INSTANCE_PTR,
            ]
        );
        assert_eq!(calls(&log, SET_LOCK_MODE), vec![vec![0]]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE_UNCHECKED),
            vec![vec![0x4001], vec![0x4002]]
        );
        assert_eq!(e.mem.u8(QUIT_CALLBACK_PENDING), 0);
        assert_eq!(e.mem.u8(INITIALISED), 0);
        assert_eq!(e.mem.u32(OBJECT_01268360), 0);
        assert_eq!(e.mem.u32(OBJECT_01268364), 0);
    }

    #[test]
    fn shutdown_without_a_callback_pointer_leaves_the_pending_byte_set() {
        let (e, log) = shut_down(false);
        assert!(calls(&log, 0x0900_0e01).is_empty());
        assert_eq!(e.mem.u8(QUIT_CALLBACK_PENDING), 1);
        assert_eq!(e.mem.u8(INITIALISED), 0);
    }

    #[test]
    fn listing_a_directory_passes_both_words_and_the_result_through() {
        let (mut e, log) = engine();
        e.mem.set_u32(LIST_DIRECTORY_POINTER, 0x0900_0d01);
        double(&mut e, &log, 0x0900_0d01, 0x55);
        let got = e
            .call(0x00cb_bd20, &args![VCALL_THIS, 0x10u32, 0x20u32])
            .u32();
        assert_eq!(got, 0x55);
        assert_eq!(calls(&log, 0x0900_0d01), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn clearing_a_char_array_frees_only_a_buffer_it_owns() {
        let (mut e, log) = engine();
        let (_router, heap, _small) = install_router(&mut e, &log, vec![]);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 0x4000_0005);
        e.call(0x00cb_bd40, &args![array]);
        assert_eq!(calls(&log, HEAP_FREE), vec![vec![heap, 0x5000, 5]]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        // A buffer it does not own (bit 31) is not freed.
        e.mem.set_u32(array, 0x6000);
        e.mem.set_u32(array + 4, 2);
        e.mem.set_u32(array + 8, 0x8000_0080);
        e.call(0x00cb_bd40, &args![array]);
        assert_eq!(calls(&log, HEAP_FREE).len(), 1);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    #[test]
    fn string_buffer_construction_points_at_its_own_storage_and_holds_one_zero_byte() {
        let (mut e, log) = engine();
        install_router(&mut e, &log, vec![]);
        let buffer = e.mem.alloc(0x8c);
        e.mem.set_u8(buffer + 0xc, 0x41);
        let got = e.call(0x00cb_bd80, &args![buffer]).u32();
        assert_eq!(got, buffer);
        assert_eq!(e.mem.u32(buffer), buffer + 0xc);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        assert_eq!(e.mem.u32(buffer + 8), 0x8000_0080);
        assert_eq!(e.mem.u8(buffer + 0xc), 0);
        // With 128 bytes in place the growth is not needed.
        assert!(calls(&log, ARRAY_RESERVE).is_empty());
    }

    /// The pieces the openers share: a router, the file function (answers `0x7777`) and a stdio
    /// object whose virtual slot `+0x18` answers the address of the byte `answer`.
    fn install_stream(e: &mut Engine, log: &Log, construct: u32, answer: u8) -> u32 {
        e.mem.set_u32(OPEN_FILE_POINTER, 0x0900_0c01);
        double(e, log, 0x0900_0c01, 0x7777);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, answer);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, 0x0900_0c18);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, vtable);
        double(e, log, construct, stream);
        double(e, log, 0x0900_0c18, flag);
        double(e, log, REMOVE_REFERENCE, 0);
        stream
    }

    #[test]
    fn opening_a_reader_wraps_it_in_a_buffered_reader_when_the_byte_is_zero() {
        let (mut e, log) = engine();
        let small_block = e.mem.alloc(0x20);
        let wrapper_block = e.mem.alloc(0x30);
        let (_router, _heap, small) =
            install_router(&mut e, &log, vec![small_block, wrapper_block]);
        let reader = install_stream(&mut e, &log, STDIO_READER_CONSTRUCT, 0);
        double(&mut e, &log, BUFFERED_READER_CONSTRUCT, 0x9999);
        // The string buffer at +0x8, the answer byte at +0x7 of the exe's 0x94-byte frame.
        let frame = e.next_stack_local(0x94);
        let got = e.call(0x00cb_bde0, &args![VCALL_THIS, 0xabcdu32]).u32();
        assert_eq!(got, 0x9999);
        assert_eq!(calls(&log, 0x0900_0c01)[0], vec![0xabcd, frame + 8]);
        assert_eq!(calls(&log, 0x0900_0c18)[0][1], frame + 7);
        assert_eq!(
            calls(&log, SMALL_ALLOCATE),
            vec![vec![small, 0x14], vec![small, 0x24]]
        );
        assert_eq!(e.mem.u16(small_block + 4), 0x14);
        assert_eq!(e.mem.u16(wrapper_block + 4), 0x24);
        let opened = calls(&log, 0x0900_0c01);
        assert_eq!(opened.len(), 1);
        assert_eq!(opened[0][0], 0xabcd);
        assert_eq!(
            calls(&log, STDIO_READER_CONSTRUCT),
            vec![vec![small_block, 0x7777]]
        );
        assert_eq!(calls(&log, 0x0900_0c18)[0][0], reader);
        assert_eq!(
            calls(&log, BUFFERED_READER_CONSTRUCT),
            vec![vec![wrapper_block, reader, 0x1000]]
        );
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![reader]]);
    }

    #[test]
    fn opening_a_reader_returns_the_stdio_reader_when_the_byte_is_set() {
        let (mut e, log) = engine();
        let small_block = e.mem.alloc(0x20);
        install_router(&mut e, &log, vec![small_block]);
        let reader = install_stream(&mut e, &log, STDIO_READER_CONSTRUCT, 1);
        double(&mut e, &log, BUFFERED_READER_CONSTRUCT, 0x9999);
        let got = e.call(0x00cb_bde0, &args![VCALL_THIS, 0xabcdu32]).u32();
        assert_eq!(got, reader);
        assert!(calls(&log, BUFFERED_READER_CONSTRUCT).is_empty());
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(calls(&log, SMALL_ALLOCATE).len(), 1);
    }

    #[test]
    fn opening_a_writer_builds_the_buffered_writer_and_lets_go_of_the_stdio_writer() {
        let (mut e, log) = engine();
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x30);
        let (_router, _heap, small) = install_router(&mut e, &log, vec![first, second]);
        let writer = install_stream(&mut e, &log, STDIO_WRITER_CONSTRUCT, 0);
        double(&mut e, &log, BUFFERED_WRITER_CONSTRUCT, 0x8888);
        // The string buffer at +0x8 of the exe's 0x94-byte frame.
        let frame = e.next_stack_local(0x94);
        let got = e.call(0x00cb_bee0, &args![VCALL_THIS, 0xabcdu32]).u32();
        assert_eq!(got, 0x8888);
        assert_eq!(calls(&log, 0x0900_0c01)[0], vec![0xabcd, frame + 8]);
        assert_eq!(
            calls(&log, SMALL_ALLOCATE),
            vec![vec![small, 0x10], vec![small, 0x1c]]
        );
        assert_eq!(e.mem.u16(first + 4), 0x10);
        assert_eq!(e.mem.u16(second + 4), 0x1c);
        assert_eq!(
            calls(&log, STDIO_WRITER_CONSTRUCT),
            vec![vec![first, 0x7777]]
        );
        assert_eq!(
            calls(&log, BUFFERED_WRITER_CONSTRUCT),
            vec![vec![second, writer, 0x1000]]
        );
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![writer]]);
    }

    /// A memory router at `0x7000` whose `+0x10` allocator (`0x7100`) hands out the block at
    /// `block` through its slot `+4` (the double `0x00ab_0004`), checking the size asked.
    fn creator_engine(block: u32, size: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0000_7000, 0x1000);
        e.mem.set_u32(0x7010, 0x7100);
        e.mem.set_u32(0x7100, 0x7200);
        e.mem.set_u32(0x7204, 0x00ab_0004);
        e.register(0x00c8_5750, |_, _| 0x7000u32.into_ret());
        e.register_double(0x00ab_0004, move |_, a| {
            assert_eq!(a, [0x7100, size]);
            block.into_ret()
        });
        e
    }

    #[test]
    fn the_dummy_singleton_is_an_eight_byte_object_with_its_table() {
        let mut e = creator_engine(0x7800, 8);
        assert_eq!(e.call(0x00cb_b9c0, &args![]).u32(), 0x7800);
        assert_eq!(e.mem.u16(0x7804), 8);
        assert_eq!(e.mem.u16(0x7806), 1);
        assert_eq!(e.mem.u32(0x7800), 0x010c_8ed4);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00cbb9c0_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00cb_b9c0, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![8]);
        assert_eq!(e.mem.u16(block + 4), 8);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010c_8ed4);
    }
}

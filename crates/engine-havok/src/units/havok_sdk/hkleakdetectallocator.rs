//! `Havok SDK/hkleakdetectallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkLeakDetectAllocator` wraps another allocator and remembers every live block with the call
//! stack that made it. The Xbox PDB has no type for it, so its layout is described from the
//! code: `+0` virtual table; `+4` the stack tracer (0 for none); `+8` the call tree the stacks
//! are stored in (an object, not a pointer); `+0x18` the critical section guarding the rest;
//! `+0x30` the wrapped allocator; `+0x34` a word given to the map insert; `+0x38` and `+0x3c`
//! the error output function (cdecl `(text, context)`) and its context; `+0x40` the map of live
//! blocks; `+0x44` the total of the sizes the wrapped allocator reports for them.
//!
//! The map has slots of 16 bytes (the block address as key, `0xffffffff` for an empty slot,
//! then the call tree entry, the size and a flag word); `00d5f520` and `00d5f570` are its
//! lookup and removal (the same table as the neighbouring `hkMap`, see
//! [`crate::units::havok_sdk::hkmap`]).
//!
//! The checks that stop the program (`INT3` after printing a complaint) panic with the address
//! of the `INT3`. Locks are called through their import slots exactly as the exe does.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkmap::{find_key_word, remove_slot};

/// The memory router (`00c85750`): `+0x10` and `+0x14` are two allocators.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Import slots: `EnterCriticalSection`, `LeaveCriticalSection`, `GetCurrentProcess`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const GET_CURRENT_PROCESS: u32 = 0x00fd_f110;
/// Clears a memory snapshot structure (`00dd6ec0`, `this` the snapshot, frees its arrays).
const SNAPSHOT_CLEAR: u32 = 0x00dd_6ec0;
/// `hkMemUtil::memSet` (`00d2cab0`, cdecl `(destination, value, bytes)`), `hkMemUtil::memCpy`
/// (`00d2caa0`) and `hkString::memCpy` (`00cb1260`), both cdecl `(destination, source, bytes)`.
const MEM_SET: u32 = 0x00d2_cab0;
const MEM_COPY: u32 = 0x00d2_caa0;
const STRING_MEM_COPY: u32 = 0x00cb_1260;
/// `hkString::snprintf` (`00cb0f60`, cdecl `(buffer, size, format, ...)`) and the CRT's
/// `__snprintf` (`00eca38d`).
const HK_SNPRINTF: u32 = 0x00cb_0f60;
const CRT_SNPRINTF: u32 = 0x00ec_a38d;
/// `hkStackTracer::getStackTrace` (`00da2b20`, `this` the tracer, `(buffer, count)`), the call
/// tree's `getCallStack` (`00da2b50`, `this` the tree, `(entry, buffer, count)`, answers how many
/// words it wrote) and `insertCallStack` (`00da2bd0`, `this` the tree, `(buffer, count)`, answers
/// the entry).
const GET_STACK_TRACE: u32 = 0x00da_2b20;
const GET_CALL_STACK: u32 = 0x00da_2b50;
const INSERT_CALL_STACK: u32 = 0x00da_2bd0;
/// The map insert (`00d5f480`, `this` the map, `(word, key, entry, size, flag)`).
const MAP_INSERT: u32 = 0x00d5_f480;
/// The exe's texts: the format of `danger`'s first line, its closing rule, and the complaints
/// `checkedFree` makes.
const DANGER_FORMAT: u32 = 0x010d_1190;
const DANGER_RULE: u32 = 0x010d_1148;
const MESSAGE_NONZERO_BYTES_FOR_NULL: u32 = 0x010d_11f0;
const MESSAGE_ZERO_BYTES_FOR_BLOCK: u32 = 0x010d_1224;
const MESSAGE_NOT_FROM_THIS_ALLOCATOR: u32 = 0x010d_11c8;
const MESSAGE_INCORRECT_SIZE: u32 = 0x010d_11a8;
/// The text `00da2990` prints for an address no symbol is found for (once), the format of each
/// line, and the names it compares with (`"WinMain"`) and falls back to (`"(unknown)"`).
const NO_SYMBOL_WARNING: u32 = 0x010d_4e18;
const LINE_FORMAT: u32 = 0x010d_4df0;
const WIN_MAIN: u32 = 0x010d_4e00;
const UNKNOWN_NAME: u32 = 0x010d_4e08;
/// The byte that says the no-symbol warning was printed (`01269a88`).
const WARNING_PRINTED: u32 = 0x0126_9a88;
/// The map's empty-slot key and the mask of an element count.
const EMPTY: u32 = 0xffff_ffff;
const COUNT_MASK: u32 = 0x7fff_ffff;

/// Calls the error output function at `+0x38` with `(text, context at +0x3c)`.
fn print(e: &mut Engine, base: u32, text: u32) {
    let context = e.mem.u32(base + 0x3c);
    let function = e.mem.u32(base + 0x38);
    e.call(function, &args![text, context]);
}

// Translated from 00d5edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::danger` (Xbox PDB), `thiscall`, `RET 0xc` `(message, address,
/// record)`: prints `message`, a line `"\nAddress=0x%p size=%i\n"` for `address` and the record's
/// size (`+4`), the call stack the record's call tree entry (`+0`) stands for (through
/// `00da2990`), and a closing rule, all through the error output function.
pub fn hk_leak_detect_allocator_danger(
    e: &mut Engine,
    this: Ptr,
    message: u32,
    address: u32,
    record: Ptr,
) {
    let base = this.addr();
    e.with_stack(0x180, |e, frame| {
        let line = frame.addr();
        let stack = frame.addr() + 0x100;
        let size = e.mem.u32(record.addr() + 4);
        e.call(
            HK_SNPRINTF,
            &args![line, 0x100u32, DANGER_FORMAT, address, size],
        );
        print(e, base, message);
        print(e, base, line);
        let entry = e.mem.u32(record.addr());
        let count = e
            .call(GET_CALL_STACK, &args![base + 8, entry, stack, 0x20u32])
            .u32();
        let tracer = e.mem.u32(base + 4);
        let function = e.mem.u32(base + 0x38);
        let context = e.mem.u32(base + 0x3c);
        fn_00da2990(
            e,
            Ptr::new(tracer),
            Ptr::new(stack),
            count as i32,
            function,
            context,
        );
        print(e, base, DANGER_RULE);
    });
}

// Translated from 00d5ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::getMemoryStatistics` (Xbox PDB, virtual slot `0x28`): jumps to slot
/// `0x28` of the wrapped allocator (`+0x30`), passing its one stack word (the statistics) on:
/// the slot ends in `RET 4` (`00a89af0`). Answers that call's `EAX`.
pub fn hk_leak_detect_allocator_get_memory_statistics(
    e: &mut Engine,
    this: Ptr,
    stats: u32,
) -> u32 {
    let child = e.mem.u32(this.addr() + 0x30);
    e.vcall(child, 0x28, &args![stats]).u32()
}

// Translated from 00d5ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::getAllocatedSize` (Xbox PDB, virtual slot `0x2c`), `RET 8` (the
/// callee's): jumps to slot `0x2c` of the wrapped allocator with its own stack words `(block,
/// bytes)`. Answers that call's `EAX`.
pub fn hk_leak_detect_allocator_get_allocated_size(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    bytes: u32,
) -> u32 {
    let child = e.mem.u32(this.addr() + 0x30);
    e.vcall(child, 0x2c, &args![block, bytes]).u32()
}

// Translated from 00d5ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::getMemorySnapshot` (Xbox PDB), `RET 4`: with the lock held, clears
/// the snapshot at `out` (`00dd6ec0`) and, when the map has live blocks, stores their count at
/// `out + 8`, a zeroed array of 0x48-byte records at `out + 4` and an array of block addresses at
/// `out` (both from the router's second allocator), then for each live block in slot order fills
/// the next record (the size at `+0`, the call stack's words from `+4`, their count at `+0x44`;
/// `getCallStack` with at most 16 words) and the next address, asking the router's first
/// allocator for the block's size (slot `0x2c`; the answer is not used). Answers 0.
pub fn hk_leak_detect_allocator_get_memory_snapshot(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let base = this.addr();
    let section = base + 0x18;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.call(SNAPSHOT_CLEAR, &args![out]);
    let map = e.mem.u32(base + 0x40);
    let count = e.mem.u32(map + 4) & COUNT_MASK;
    if count == 0 {
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return 0;
    }
    e.mem.set_u32(out.addr() + 8, count);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let second = e.mem.u32(router + 0x14);
    let bytes = count.wrapping_mul(9).wrapping_mul(8);
    let records = e.vcall(second, 4, &args![bytes]).u32();
    e.mem.set_u32(out.addr() + 4, records);
    e.call(MEM_SET, &args![records, 0u32, bytes]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let second = e.mem.u32(router + 0x14);
    let addresses = e.vcall(second, 4, &args![count.wrapping_mul(4)]).u32();
    let mut record = e.mem.u32(out.addr() + 4);
    e.mem.set_u32(out.addr(), addresses);
    let mut address_cursor = addresses;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let first_slot = |e: &Engine, from: i32| -> i32 {
        let mask = e.mem.i32(map + 8);
        let slots = e.mem.u32(map);
        let mut slot = from;
        if mask >= 0 {
            while e
                .mem
                .u32(slots.wrapping_add((slot as u32).wrapping_mul(16)))
                == EMPTY
            {
                slot += 1;
                if slot > mask {
                    break;
                }
            }
        }
        slot
    };
    let mut slot = first_slot(&*e, 0);
    if slot <= e.mem.i32(map + 8) {
        e.with_stack(0x40, |e, stack| loop {
            let slots = e.mem.u32(map);
            let entry = slots.wrapping_add((slot as u32).wrapping_mul(16));
            let call_entry = e.mem.u32(entry + 4);
            let size = e.mem.u32(entry + 8);
            let key = e.mem.u32(entry);
            e.mem.set_u32(address_cursor, key);
            let words = e
                .call(GET_CALL_STACK, &args![base + 8, call_entry, stack, 0x10u32])
                .u32();
            e.mem.set_u32(record + 0x44, words);
            e.call(
                STRING_MEM_COPY,
                &args![record + 4, stack, words.wrapping_mul(4)],
            );
            e.mem.set_u32(record, size);
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 0x2c, &args![key, size]);
            slot += 1;
            record = record.wrapping_add(0x48);
            address_cursor = address_cursor.wrapping_add(4);
            if slot > e.mem.i32(map + 8) {
                break;
            }
            slot = first_slot(&*e, slot);
            if slot > e.mem.i32(map + 8) {
                break;
            }
        });
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    0
}

// Translated from 00d5f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::checkedAlloc` (Xbox PDB), `RET 8` `(is_buffer, size)`: with the lock
/// held allocates `size` bytes from the wrapped allocator (slot `4`), adds the size it reports
/// for the block (slot `0x2c`) to the total at `+0x44`, takes the call stack (when there is a
/// tracer: `getStackTrace` with up to 32 words, then `insertCallStack` on the call tree) and
/// records `(block -> call tree entry, size, is_buffer)` in the map (`00d5f480`). Answers the
/// block. (The exe stores the flag as a whole word of which only the low byte was set; the
/// upper bytes are stale stack, 0 here, and nothing reads them.)
pub fn hk_leak_detect_allocator_checked_alloc(
    e: &mut Engine,
    this: Ptr,
    is_buffer: u32,
    size: u32,
) -> u32 {
    let base = this.addr();
    let section = base + 0x18;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let child = e.mem.u32(base + 0x30);
    let block = e.vcall(child, 4, &args![size]).u32();
    let child = e.mem.u32(base + 0x30);
    let allocated = e.vcall(child, 0x2c, &args![block, size]).u32();
    let total = e.mem.u32(base + 0x44).wrapping_add(allocated);
    e.mem.set_u32(base + 0x44, total);
    let flag = u32::from(is_buffer != 0);
    let tracer = e.mem.u32(base + 4);
    let entry = if tracer == 0 {
        0
    } else {
        e.with_stack(0x80, |e, stack| {
            let words = e
                .call(GET_STACK_TRACE, &args![tracer, stack, 0x20u32])
                .u32();
            e.call(INSERT_CALL_STACK, &args![base + 8, stack, words])
                .u32()
        })
    };
    let map = e.mem.u32(base + 0x40);
    let word = e.mem.u32(base + 0x34);
    e.call(MAP_INSERT, &args![map, word, block, entry, size, flag]);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    block
}

// Translated from 00d5f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::checkedFree` (Xbox PDB), `RET 0xc` `(is_buffer, block, size)`: with
/// the lock held, a null block with no size just leaves; the other combinations the exe
/// complains about through the error function and then executes `INT3` (a panic here with the
/// address): a size for a null block (`00d5f1b2`), no size for a block (`00d5f188`), a block not
/// in the map (`00d5f1e7`) and a recorded size that differs (`danger`, then `00d5f220`).
/// Otherwise the block leaves the map (`00d5f570`), the size the wrapped allocator reports for it
/// comes off the total at `+0x44` and the wrapped allocator frees it (slots `0x2c` and `8`).
pub fn hk_leak_detect_allocator_checked_free(
    e: &mut Engine,
    this: Ptr,
    _is_buffer: u32,
    block: u32,
    size: u32,
) {
    let base = this.addr();
    let section = base + 0x18;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if block == 0 {
        if size != 0 {
            print(e, base, MESSAGE_NONZERO_BYTES_FOR_NULL);
            panic!("INT3 at 00d5f1b2 (hkLeakDetectAllocator::checkedFree)");
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return;
    }
    if size == 0 {
        print(e, base, MESSAGE_ZERO_BYTES_FOR_BLOCK);
        panic!("INT3 at 00d5f188 (hkLeakDetectAllocator::checkedFree)");
    }
    let map = e.mem.u32(base + 0x40);
    let slot = fn_00d5f520(e, Ptr::new(map), block);
    let map = e.mem.u32(base + 0x40);
    if (slot as i32) > e.mem.i32(map + 8) {
        print(e, base, MESSAGE_NOT_FROM_THIS_ALLOCATOR);
        panic!("INT3 at 00d5f1e7 (hkLeakDetectAllocator::checkedFree)");
    }
    let slots = e.mem.u32(map);
    let entry = slots.wrapping_add(slot.wrapping_shl(4)).wrapping_add(4);
    let first = e.mem.u32(entry);
    let recorded = e.mem.u32(entry + 4);
    let third = e.mem.u32(entry + 8);
    if recorded != size {
        e.with_stack(12, |e, record| {
            e.mem.set_u32(record.addr(), first);
            e.mem.set_u32(record.addr() + 4, recorded);
            e.mem.set_u32(record.addr() + 8, third);
            hk_leak_detect_allocator_danger(e, this, MESSAGE_INCORRECT_SIZE, block, record);
        });
        panic!("INT3 at 00d5f220 (hkLeakDetectAllocator::checkedFree)");
    }
    let map = e.mem.u32(base + 0x40);
    fn_00d5f570(e, Ptr::new(map), slot);
    let child = e.mem.u32(base + 0x30);
    let allocated = e.vcall(child, 0x2c, &args![block, size]).u32();
    let total = e.mem.u32(base + 0x44).wrapping_sub(allocated);
    e.mem.set_u32(base + 0x44, total);
    let child = e.mem.u32(base + 0x30);
    e.vcall(child, 8, &args![block, size]);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::blockAlloc` (Xbox PDB, virtual slot 4), `RET 4`: `checkedAlloc` with
/// the buffer flag 0.
pub fn hk_leak_detect_allocator_block_alloc(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    hk_leak_detect_allocator_checked_alloc(e, this, 0, size)
}

// Translated from 00d5f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::blockFree` (Xbox PDB, virtual slot 8), `RET 8`: `checkedFree` with the
/// buffer flag 0.
pub fn hk_leak_detect_allocator_block_free(e: &mut Engine, this: Ptr, block: u32, size: u32) {
    hk_leak_detect_allocator_checked_free(e, this, 0, block, size);
}

// Translated from 00d5f290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::bufAlloc` (Xbox PDB, virtual slot `0xc`), `RET 4`: `checkedAlloc`
/// with the buffer flag 1 and the byte count the word at `size_cell` holds (which is left as it
/// is).
pub fn hk_leak_detect_allocator_buf_alloc(e: &mut Engine, this: Ptr, size_cell: Ptr) -> u32 {
    let size = e.mem.u32(size_cell.addr());
    hk_leak_detect_allocator_checked_alloc(e, this, 1, size)
}

// Translated from 00d5f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::bufFree` (Xbox PDB, virtual slot `0x10`), `RET 8`: `checkedFree` with
/// the buffer flag 1.
pub fn hk_leak_detect_allocator_buf_free(e: &mut Engine, this: Ptr, block: u32, size: u32) {
    hk_leak_detect_allocator_checked_free(e, this, 1, block, size);
}

// Translated from 00d5f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLeakDetectAllocator::bufRealloc` (Xbox PDB, virtual slot `0x14`), `RET 0xc` `(block,
/// old_bytes, size_cell)`: `checkedAlloc` (buffer flag 1) of the byte count the word at
/// `size_cell` holds, copies the smaller of `old_bytes` and that count (read again after the
/// allocation) from the old block (`hkMemUtil::memCpy`), `checkedFree` of the old block (buffer
/// flag 1, `old_bytes`) and answers the new block.
pub fn hk_leak_detect_allocator_buf_realloc(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    old_bytes: i32,
    size_cell: Ptr,
) -> u32 {
    let wanted = e.mem.u32(size_cell.addr());
    let new_block = hk_leak_detect_allocator_checked_alloc(e, this, 1, wanted);
    let mut copied = e.mem.i32(size_cell.addr());
    if old_bytes < copied {
        copied = old_bytes;
    }
    e.call(MEM_COPY, &args![new_block, block, copied]);
    hk_leak_detect_allocator_checked_free(e, this, 1, block, old_bytes as u32);
    new_block
}

// Translated from 00d5f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the slot index of `key` in the map of live
/// blocks (16-byte slots), or `m_hashMod + 1` when it is not there.
pub fn fn_00d5f520(e: &mut Engine, map: Ptr, key: u32) -> u32 {
    find_key_word(e, map.addr(), key, 4)
}

// Translated from 00d5f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes the element in slot `index` of the map
/// of live blocks, moving later elements of its cluster back into the hole (three value words
/// per element).
pub fn fn_00d5f570(e: &mut Engine, map: Ptr, index: u32) {
    remove_slot(e, map.addr(), index, 4, 3);
}

// Translated from 00da2990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map's name for this address, `hkLeakDetectAllocator::quit`, is
/// for a folded body; this one is a method of the stack tracer, `this` = the tracer, `RET 0x10`
/// `(trace, count, output, context)`): prints the symbols of the `count` addresses in `trace`.
/// For each address asks the symbol functions in the table at `+8` of the tracer: slot `0x18`
/// (`(process, address, address of a displacement word, address of a 0x18-byte symbol header
/// followed by room for a 0x7e8-byte name)`, stdcall) and, on success unless the name is
/// `"WinMain"` (which ends the whole listing), slot `0x28` (`(process, address, 0, the
/// displacement, address of a 0x14-byte line record)`) to get the file and line, then formats
/// `"%s(%i):'%s'\n"` (file, line, name) with the CRT's `snprintf` into a 0x800-byte buffer and
/// gives it to `output(text, context)`. When the symbol call fails the name becomes
/// `"(unknown)"`, and the first time that happens `output` is also given the long warning
/// text (the byte at `01269a88` records that).
///
/// The exe keeps all of this in one 0x101c-byte stack frame, which this does too (the
/// displacement at `+0`, the line record at `+4`, the symbol at `+0x1c`, the output buffer at
/// `+0x81c`).
pub fn fn_00da2990(e: &mut Engine, this: Ptr, trace: Ptr, count: i32, output: u32, context: u32) {
    let tracer = this.addr();
    let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
    if count <= 0 {
        return;
    }
    e.with_stack(0x101c, |e, frame| {
        let f = frame.addr();
        let mut index = 0i32;
        loop {
            let address = e
                .mem
                .u32(trace.addr().wrapping_add((index as u32).wrapping_mul(4)));
            let table = e.mem.u32(tracer + 8);
            for offset in [0x1c, 0x2c, 0x20, 0x24, 0x28, 0x30, 0x00] {
                e.mem.set_u32(f + offset, 0);
            }
            e.mem.set_u32(f + 0x1c, 0x18);
            e.mem.set_u32(f + 0x2c, 0x7e8);
            let symbol_function = e.mem.u32(table + 0x18);
            let found = e
                .call(symbol_function, &args![process, address, f, f + 0x1c])
                .u32();
            if found == 0 {
                if e.mem.u8(WARNING_PRINTED) == 0 {
                    e.call(output, &args![NO_SYMBOL_WARNING, context]);
                    e.mem.set_u8(WARNING_PRINTED, 1);
                }
                for word in 0..2 {
                    let value = e.mem.u32(UNKNOWN_NAME + 4 * word);
                    e.mem.set_u32(f + 0x30 + 4 * word, value);
                }
                let tail = e.mem.u16(UNKNOWN_NAME + 8);
                e.mem.set_u16(f + 0x38, tail);
            } else if e.mem.cstr(f + 0x30) == e.mem.cstr(WIN_MAIN) {
                return;
            }
            for offset in [4, 8, 0xc, 0x10, 0x14, 0x18] {
                e.mem.set_u32(f + offset, 0);
            }
            e.mem.set_u32(f + 4, 0x18);
            let table = e.mem.u32(tracer + 8);
            let line_function = e.mem.u32(table + 0x28);
            e.call(line_function, &args![process, address, 0u32, f, f + 4]);
            let line = e.mem.u32(f + 0xc);
            let file = e.mem.u32(f + 0x10);
            e.call(
                CRT_SNPRINTF,
                &args![f + 0x81c, 0x800u32, LINE_FORMAT, file, line, f + 0x30],
            );
            e.call(output, &args![f + 0x81c, context]);
            index += 1;
            if index >= count {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5edc0,
            hk_leak_detect_allocator_danger(Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00d5ee60,
            hk_leak_detect_allocator_get_memory_statistics(Ptr, u32) -> u32
        ),
        entry!(
            0x00d5ee70,
            hk_leak_detect_allocator_get_allocated_size(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00d5ef20,
            hk_leak_detect_allocator_get_memory_snapshot(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00d5f0a0,
            hk_leak_detect_allocator_checked_alloc(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00d5f150,
            hk_leak_detect_allocator_checked_free(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00d5f260,
            hk_leak_detect_allocator_block_alloc(Ptr, u32) -> u32
        ),
        entry!(
            0x00d5f270,
            hk_leak_detect_allocator_block_free(Ptr, u32, u32)
        ),
        entry!(
            0x00d5f290,
            hk_leak_detect_allocator_buf_alloc(Ptr, Ptr) -> u32
        ),
        entry!(0x00d5f2b0, hk_leak_detect_allocator_buf_free(Ptr, u32, u32)),
        entry!(
            0x00d5f2d0,
            hk_leak_detect_allocator_buf_realloc(Ptr, u32, i32, Ptr) -> u32
        ),
        entry!(0x00d5f520, fn_00d5f520(Ptr, u32) -> u32),
        entry!(0x00d5f570, fn_00d5f570(Ptr, u32)),
        entry!(0x00da2990, fn_00da2990(Ptr, Ptr, i32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ROUTER_CELL: u32 = 0x0127_0800;
    const COUNT_CELL: u32 = 0x0127_0804;
    const CHILD_VTABLE: u32 = 0x0128_6000;
    const FIRST_VTABLE: u32 = 0x0128_6100;
    const SECOND_VTABLE: u32 = 0x0128_6200;
    const CHILD_ALLOC: u32 = 0x0900_7001;
    const CHILD_FREE: u32 = 0x0900_7002;
    const CHILD_STATS: u32 = 0x0900_7003;
    const CHILD_SIZE: u32 = 0x0900_7004;
    const FIRST_SIZE: u32 = 0x0900_7005;
    const SECOND_ALLOC: u32 = 0x0900_7006;
    const ERROR_FUNCTION: u32 = 0x0900_7007;
    const SYMBOL_FUNCTION: u32 = 0x0900_7008;
    const LINE_FUNCTION: u32 = 0x0900_7009;
    const TABLE: u32 = 0x0127_0a00;

    type Texts = Rc<RefCell<Vec<(String, u32)>>>;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the exe's constants the functions read (the texts and the warning flag), the
    /// router and the three allocators, the import and helper doubles, and a recorder for what the
    /// error function receives.
    fn engine() -> (Engine, Texts) {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.map(0x0126_9000, 0x1000);
        e.map(0x010d_1000, 0x1000);
        e.map(0x010d_4000, 0x2000);
        e.mem.set_cstr(WIN_MAIN, b"WinMain");
        e.mem.set_cstr(UNKNOWN_NAME, b"(unknown)");
        e.put_vtable(
            CHILD_VTABLE,
            &[
                0,
                CHILD_ALLOC,
                CHILD_FREE,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                CHILD_STATS,
                CHILD_SIZE,
            ],
        );
        e.put_vtable(FIRST_VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, FIRST_SIZE]);
        e.put_vtable(SECOND_VTABLE, &[0, SECOND_ALLOC]);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, FIRST_VTABLE);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, SECOND_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, first);
        e.mem.set_u32(router + 0x14, second);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| e.mem.u32(ROUTER_CELL).into_ret());
        e.register(CHILD_ALLOC, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CHILD_FREE, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(CHILD_STATS, |_, _| 0x5151u32.into_ret());
        e.register(CHILD_SIZE, |_, a| (a[2] + 0x10).into_ret());
        e.register(FIRST_SIZE, noop);
        e.register(SECOND_ALLOC, |e, a| e.mem.alloc(a[1]).into_ret());
        for address in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            SNAPSHOT_CLEAR,
            HK_SNPRINTF,
        ] {
            e.register(address, noop);
        }
        e.register(GET_CURRENT_PROCESS, |_, _| 0x4242u32.into_ret());
        e.register(MEM_SET, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        for address in [MEM_COPY, STRING_MEM_COPY] {
            e.register(address, |e, a| {
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(a[0], &bytes);
                Ret::default()
            });
        }
        e.register(GET_STACK_TRACE, |e, a| {
            // Three words: 0x31, 0x32, 0x33.
            for (i, word) in [0x31u32, 0x32, 0x33].into_iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, word);
            }
            3u32.into_ret()
        });
        e.register(INSERT_CALL_STACK, |_, _| 0x77u32.into_ret());
        e.register(GET_CALL_STACK, |e, a| {
            // The words 0xa1, 0xa2, ... as many as COUNT_CELL says, at most `a[3]`.
            let count = e.mem.u32(COUNT_CELL).min(a[3]);
            for i in 0..count {
                e.mem.set_u32(a[2] + 4 * i, 0xa1 + i);
            }
            count.into_ret()
        });
        e.register(MAP_INSERT, |e, a| {
            // (map, word, block, entry, size, flag)
            put_entry(e, a[0], a[2], a[3], a[4], a[5]);
            Ret::default()
        });
        let texts: Texts = Rc::default();
        let log = Rc::clone(&texts);
        e.register_double(ERROR_FUNCTION, move |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).into_owned();
            log.borrow_mut().push((text, a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, texts)
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A map of 8 slots with 16-byte slots, all empty.
    fn new_map(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(0xc);
        let slots = e.mem.alloc(8 * 16);
        for slot in 0..8 {
            e.mem.set_u32(slots + 16 * slot, EMPTY);
        }
        e.mem.set_u32(map, slots);
        e.mem.set_u32(map + 8, 7);
        map
    }

    /// Stores `key -> (entry, size, flag)` at the key's home slot or the next free one.
    fn put_entry(e: &mut Engine, map: u32, key: u32, entry: u32, size: u32, flag: u32) {
        let slots = e.mem.u32(map);
        let mask = e.mem.u32(map + 8);
        let mut slot = (key >> 4).wrapping_mul(0x9e37_79b1) & mask;
        while e.mem.u32(slots + 16 * slot) != EMPTY {
            slot = (slot + 1) & mask;
        }
        let at = slots + 16 * slot;
        e.mem.set_u32(at, key);
        e.mem.set_u32(at + 4, entry);
        e.mem.set_u32(at + 8, size);
        e.mem.set_u32(at + 0xc, flag);
        let count = e.mem.u32(map + 4);
        e.mem.set_u32(map + 4, count + 1);
    }

    /// A leak detector with a tracer (or none), the child allocator and an empty map.
    fn allocator(e: &mut Engine, with_tracer: bool) -> (u32, u32) {
        let object = e.mem.alloc(0x50);
        let child = e.mem.alloc(8);
        e.mem.set_u32(child, CHILD_VTABLE);
        let map = new_map(e);
        if with_tracer {
            e.mem.set_u32(object + 4, 0x1234);
        }
        e.mem.set_u32(object + 0x30, child);
        e.mem.set_u32(object + 0x34, 0x5678);
        e.mem.set_u32(object + 0x38, ERROR_FUNCTION);
        e.mem.set_u32(object + 0x3c, 0x9abc);
        e.mem.set_u32(object + 0x40, map);
        (object, map)
    }

    #[test]
    fn checked_alloc_records_the_block_with_its_call_stack() {
        let (mut e, _) = engine();
        let (object, map) = allocator(&mut e, true);
        let block = e.call(0x00d5_f260, &args![object, 0x40u32]).u32();
        assert_ne!(block, 0);
        // The total grew by the size the child reports (the size plus 0x10).
        assert_eq!(e.mem.u32(object + 0x44), 0x50);
        // The tracer was asked for 32 words and the tree for their entry (0x77).
        assert_eq!(calls(&e, GET_STACK_TRACE)[0][0], 0x1234);
        assert_eq!(calls(&e, GET_STACK_TRACE)[0][2], 0x20);
        let insert = &calls(&e, INSERT_CALL_STACK)[0];
        assert_eq!((insert[0], insert[2]), (object + 8, 3));
        // The map holds (entry 0x77, size 0x40, flag 0) under the block.
        let slots = e.mem.u32(map);
        let entries: Vec<_> = (0..8)
            .map(|i| slots + 16 * i)
            .filter(|at| e.mem.u32(*at) != EMPTY)
            .collect();
        assert_eq!(entries.len(), 1);
        let at = entries[0];
        assert_eq!(
            [
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u32(at + 8),
                e.mem.u32(at + 0xc)
            ],
            [block, 0x77, 0x40, 0]
        );
        assert_eq!(calls(&e, MAP_INSERT)[0][..2], [map, 0x5678]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![object + 0x18]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![object + 0x18]]);
    }

    #[test]
    fn checked_alloc_without_a_tracer_records_entry_zero_and_the_buffer_flag() {
        let (mut e, _) = engine();
        let (object, map) = allocator(&mut e, false);
        let size_cell = e.mem.alloc(4);
        e.mem.set_u32(size_cell, 0x30);
        let block = e.call(0x00d5_f290, &args![object, size_cell]).u32();
        assert!(calls(&e, GET_STACK_TRACE).is_empty());
        assert!(calls(&e, INSERT_CALL_STACK).is_empty());
        let insert = &calls(&e, MAP_INSERT)[0];
        assert_eq!(insert[2..], [block, 0, 0x30, 1]);
        assert_eq!(e.mem.u32(size_cell), 0x30);
        assert_eq!(insert[0], map);
    }

    #[test]
    fn checked_free_removes_the_block_and_gives_it_back() {
        let (mut e, _) = engine();
        let (object, map) = allocator(&mut e, true);
        let block = e.call(0x00d5_f260, &args![object, 0x40u32]).u32();
        e.call_log = Some(vec![]);
        e.call(0x00d5_f270, &args![object, block, 0x40u32]);
        assert_eq!(e.mem.u32(object + 0x44), 0);
        assert_eq!(e.mem.u32(map + 4), 0);
        assert_eq!(e.mem.block_size(block), None);
        assert_eq!(calls(&e, CHILD_FREE)[0][1..], [block, 0x40]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        // Freeing nothing with no size just takes and leaves the lock.
        e.call_log = Some(vec![]);
        e.call(0x00d5_f2b0, &args![object, 0u32, 0u32]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        assert!(calls(&e, CHILD_FREE).is_empty());
    }

    /// Runs `e.call(address, args)` expecting an `INT3` panic; answers its message.
    fn int3(e: &mut Engine, address: u32, words: &[u32]) -> String {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(address, words);
        }));
        let payload = result.expect_err("the exe executes INT3 here");
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap()
    }

    #[test]
    fn checked_free_complains_and_breaks_on_bad_arguments() {
        let (mut e, texts) = engine();
        let (object, _) = allocator(&mut e, true);
        let block = e.call(0x00d5_f260, &args![object, 0x40u32]).u32();
        // A size for a null block.
        let message = int3(&mut e, 0x00d5_f150, &args![object, 0u32, 0u32, 8u32]);
        assert!(message.contains("INT3 at 00d5f1b2"), "{message}");
        // No size for a block.
        let message = int3(&mut e, 0x00d5_f150, &args![object, 0u32, block, 0u32]);
        assert!(message.contains("INT3 at 00d5f188"), "{message}");
        // A block the map does not know.
        let message = int3(&mut e, 0x00d5_f150, &args![object, 0u32, 0x5000u32, 8u32]);
        assert!(message.contains("INT3 at 00d5f1e7"), "{message}");
        // A size that differs from the recorded one: danger prints, then INT3.
        let message = int3(&mut e, 0x00d5_f150, &args![object, 0u32, block, 0x41u32]);
        assert!(message.contains("INT3 at 00d5f220"), "{message}");
        let calls_to_print: Vec<u32> = [
            MESSAGE_NONZERO_BYTES_FOR_NULL,
            MESSAGE_ZERO_BYTES_FOR_BLOCK,
            MESSAGE_NOT_FROM_THIS_ALLOCATOR,
            MESSAGE_INCORRECT_SIZE,
        ]
        .to_vec();
        let printed: Vec<u32> = calls(&e, ERROR_FUNCTION).iter().map(|c| c[0]).collect();
        assert_eq!(printed[..3], calls_to_print[..3]);
        assert_eq!(printed[3], MESSAGE_INCORRECT_SIZE);
        assert_eq!(texts.borrow().len(), 6);
        assert!(texts.borrow().iter().all(|(_, context)| *context == 0x9abc));
    }

    #[test]
    fn danger_prints_the_messages_the_stack_and_the_rule() {
        let (mut e, _) = engine();
        let (object, _) = allocator(&mut e, true);
        e.mem.set_u32(COUNT_CELL, 0);
        let record = e.mem.alloc(12);
        e.mem.set_u32(record, 0x77);
        e.mem.set_u32(record + 4, 0x40);
        let message = e.mem.alloc(16);
        e.mem.set_cstr(message, b"msg");
        e.call(0x00d5_edc0, &args![object, message, 0x2222u32, record]);
        let format = &calls(&e, HK_SNPRINTF)[0];
        assert_eq!(format[1..], [0x100, DANGER_FORMAT, 0x2222, 0x40]);
        let printed: Vec<u32> = calls(&e, ERROR_FUNCTION).iter().map(|c| c[0]).collect();
        assert_eq!(printed.len(), 3);
        assert_eq!(printed[0], message);
        assert_eq!(printed[1], format[0]);
        assert_eq!(printed[2], DANGER_RULE);
        // The call stack of the record's entry was asked for with room for 32 words.
        let stack = &calls(&e, GET_CALL_STACK)[0];
        assert_eq!((stack[0], stack[1], stack[3]), (object + 8, 0x77, 0x20));
    }

    #[test]
    fn the_wrapped_allocator_questions_are_passed_on() {
        let (mut e, _) = engine();
        let (object, _) = allocator(&mut e, false);
        assert_eq!(e.call(0x00d5_ee60, &args![object, 0x44u32]).u32(), 0x5151);
        assert_eq!(
            e.call(0x00d5_ee70, &args![object, 0x9000u32, 0x20u32])
                .u32(),
            0x30
        );
        assert_eq!(calls(&e, CHILD_SIZE)[0][1..], [0x9000, 0x20]);
    }

    #[test]
    fn buf_realloc_allocates_copies_and_frees() {
        let (mut e, _) = engine();
        let (object, map) = allocator(&mut e, false);
        let size_cell = e.mem.alloc(4);
        e.mem.set_u32(size_cell, 0x20);
        let old = e.call(0x00d5_f290, &args![object, size_cell]).u32();
        e.mem.write(old, &[9u8; 0x20]);
        // Grow to 0x40: all 0x20 old bytes are copied.
        e.mem.set_u32(size_cell, 0x40);
        e.call_log = Some(vec![]);
        let new = e
            .call(0x00d5_f2d0, &args![object, old, 0x20u32, size_cell])
            .u32();
        assert_ne!(new, old);
        assert_eq!(calls(&e, MEM_COPY), vec![vec![new, old, 0x20]]);
        assert_eq!(e.mem.u8(new + 0x1f), 9);
        assert_eq!(calls(&e, CHILD_FREE)[0][1..], [old, 0x20]);
        // The map now holds the new block only, as a buffer.
        assert_eq!(e.mem.u32(map + 4), 1);
        // Shrinking copies only the new size.
        e.mem.set_u32(size_cell, 0x10);
        let again = e
            .call(0x00d5_f2d0, &args![object, new, 0x40u32, size_cell])
            .u32();
        assert_eq!(calls(&e, MEM_COPY)[1], vec![again, new, 0x10]);
    }

    #[test]
    fn the_map_lookup_and_removal_use_16_byte_slots() {
        let (mut e, _) = engine();
        let map = new_map(&mut e);
        // Three keys with one home slot probe along the table.
        for (i, key) in [0x100u32, 0x101, 0x102].into_iter().enumerate() {
            put_entry(&mut e, map, key, 0x10 + i as u32, 0x20 + i as u32, i as u32);
        }
        let home = (0x100u32 >> 4).wrapping_mul(0x9e37_79b1) & 7;
        assert_eq!(
            e.call(0x00d5_f520, &args![map, 0x101u32]).u32(),
            (home + 1) & 7
        );
        assert_eq!(e.call(0x00d5_f520, &args![map, 0x103u32]).u32(), 8);
        // Removing the middle one moves the last into its place, with its three value words.
        e.call(0x00d5_f570, &args![map, (home + 1) & 7]);
        let slots = e.mem.u32(map);
        let at = slots + 16 * ((home + 1) & 7);
        assert_eq!(
            [
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u32(at + 8),
                e.mem.u32(at + 0xc)
            ],
            [0x102, 0x12, 0x22, 2]
        );
        assert_eq!(e.mem.u32(slots + 16 * ((home + 2) & 7)), EMPTY);
        assert_eq!(e.mem.u32(map + 4), 2);
    }

    #[test]
    fn the_memory_snapshot_lists_every_live_block() {
        let (mut e, _) = engine();
        let (object, map) = allocator(&mut e, true);
        e.mem.set_u32(COUNT_CELL, 2);
        for (key, size) in [(0x1000u32, 0x10u32), (0x2000, 0x20), (0x3000, 0x30)] {
            put_entry(&mut e, map, key, 0x70 + (key >> 12), size, 0);
        }
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d5_ef20, &args![object, out]).u32(), 0);
        assert_eq!(e.mem.u32(out + 8), 3);
        let (addresses, records) = (e.mem.u32(out), e.mem.u32(out + 4));
        // Slots are visited in order; collect what each record says.
        let slots = e.mem.u32(map);
        let occupied: Vec<u32> = (0..8)
            .map(|i| slots + 16 * i)
            .filter(|at| e.mem.u32(*at) != EMPTY)
            .collect();
        for (i, at) in occupied.iter().enumerate() {
            let record = records + 0x48 * i as u32;
            assert_eq!(e.mem.u32(addresses + 4 * i as u32), e.mem.u32(*at));
            assert_eq!(e.mem.u32(record), e.mem.u32(*at + 8));
            assert_eq!(e.mem.u32(record + 0x44), 2);
            assert_eq!([e.mem.u32(record + 4), e.mem.u32(record + 8)], [0xa1, 0xa2]);
        }
        // getCallStack was asked for at most 16 words per block.
        assert!(calls(&e, GET_CALL_STACK).iter().all(|c| c[3] == 0x10));
        assert_eq!(calls(&e, GET_CALL_STACK).len(), 3);
        // The router's first allocator was asked for each block's size.
        assert_eq!(calls(&e, FIRST_SIZE).len(), 3);
        assert_eq!(calls(&e, SNAPSHOT_CLEAR), vec![vec![out]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn the_memory_snapshot_of_an_empty_map_only_clears_the_snapshot() {
        let (mut e, _) = engine();
        let (object, _) = allocator(&mut e, true);
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d5_ef20, &args![object, out]).u32(), 0);
        assert_eq!(e.mem.u32(out + 8), 0);
        assert_eq!(calls(&e, SNAPSHOT_CLEAR), vec![vec![out]]);
        assert!(calls(&e, SECOND_ALLOC).is_empty());
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    /// A tracer whose symbol table (at +8) holds the two stdcall doubles; the symbol double
    /// writes the name `names[address]` (the high address byte picks it), the line double the
    /// line 42 and a file name; the CRT `snprintf` double formats "%s(%i):'%s'\n" for real.
    fn tracer(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x18, SYMBOL_FUNCTION);
        e.mem.set_u32(table + 0x28, LINE_FUNCTION);
        let tracer = e.mem.alloc(0x10);
        e.mem.set_u32(tracer + 8, table);
        e.mem.set_u32(TABLE, 0);
        e.register(SYMBOL_FUNCTION, |e, a| {
            // (process, address, displacement, symbol): the address picks the outcome.
            let name: &[u8] = match a[1] {
                0x2000 => return Ret::default(),
                0x3000 => b"WinMain",
                _ => b"Update",
            };
            e.mem.set_cstr(a[3] + 0x14, name);
            1u32.into_ret()
        });
        e.register(LINE_FUNCTION, |e, a| {
            // (process, address, 0, displacement, line record)
            let file = e.mem.alloc(16);
            e.mem.set_cstr(file, b"file.cpp");
            e.mem.set_u32(a[4] + 8, 42);
            e.mem.set_u32(a[4] + 0xc, file);
            Ret::default()
        });
        e.register(CRT_SNPRINTF, |e, a| {
            let file = String::from_utf8_lossy(&e.mem.cstr(a[3])).into_owned();
            let name = String::from_utf8_lossy(&e.mem.cstr(a[5])).into_owned();
            let text = format!("{}({}):'{}'\n", file, a[4] as i32, name);
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
        tracer
    }

    #[test]
    fn fn_00da2990_prints_a_line_per_address_until_win_main() {
        let (mut e, texts) = engine();
        let tracer = tracer(&mut e);
        let trace = e.mem.alloc(16);
        for (i, address) in [0x1000u32, 0x3000, 0x1100].into_iter().enumerate() {
            e.mem.set_u32(trace + 4 * i as u32, address);
        }
        e.call(
            0x00da_2990,
            &args![tracer, trace, 3u32, ERROR_FUNCTION, 0xc0de_u32],
        );
        // The first address prints; the second is WinMain: the listing ends there.
        assert_eq!(
            *texts.borrow(),
            vec![("file.cpp(42):'Update'\n".to_string(), 0xc0de)]
        );
        let symbol = &calls(&e, SYMBOL_FUNCTION);
        assert_eq!(symbol.len(), 2);
        assert_eq!((symbol[0][0], symbol[0][1]), (0x4242, 0x1000));
        let line = &calls(&e, LINE_FUNCTION)[0];
        assert_eq!((line[0], line[1], line[2]), (0x4242, 0x1000, 0));
        // The line record is the 0x14 bytes after the displacement word, the symbol 0x18 after.
        assert_eq!(line[4], symbol[0][3] - 0x18);
        assert_eq!(line[3], symbol[0][2]);
        assert_eq!(e.mem.u32(symbol[0][3]), 0x18);
    }

    #[test]
    fn fn_00da2990_warns_once_when_no_symbol_is_found_and_calls_the_name_unknown() {
        let (mut e, texts) = engine();
        let tracer = tracer(&mut e);
        let trace = e.mem.alloc(8);
        e.mem.set_u32(trace, 0x2000);
        e.mem.set_u32(trace + 4, 0x2000);
        e.mem.set_cstr(NO_SYMBOL_WARNING, b"warning text");
        e.call(
            0x00da_2990,
            &args![tracer, trace, 2u32, ERROR_FUNCTION, 7u32],
        );
        assert_eq!(
            *texts.borrow(),
            vec![
                ("warning text".to_string(), 7),
                ("file.cpp(42):'(unknown)'\n".to_string(), 7),
                ("file.cpp(42):'(unknown)'\n".to_string(), 7),
            ]
        );
        assert_eq!(e.mem.u8(WARNING_PRINTED), 1);
    }

    #[test]
    fn fn_00da2990_with_no_addresses_prints_nothing() {
        let (mut e, texts) = engine();
        let tracer = tracer(&mut e);
        let trace = e.mem.alloc(8);
        e.call(
            0x00da_2990,
            &args![tracer, trace, 0u32, ERROR_FUNCTION, 7u32],
        );
        assert!(texts.borrow().is_empty());
        assert_eq!(calls(&e, GET_CURRENT_PROCESS).len(), 1);
    }
}

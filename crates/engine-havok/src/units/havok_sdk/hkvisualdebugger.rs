//! `Havok SDK/hkvisualdebugger.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cd1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cd_1970`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cd1e40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cd_1970, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, count,
/// elementSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`, cdecl `(destination, source, bytes)`).
const MEM_MOVE: u32 = 0x00d2_cc70;

// Translated from 00cd1d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splices into an `hkArray` of 4-byte elements (`data`, `size`, `capacityAndFlags` at +0,
/// +4, +8): removes `remove_count` elements at `index` and inserts `insert_count` words
/// copied from `source` there. When the new size exceeds the capacity (low 30 bits) the
/// array is grown to the larger of the new size and twice the capacity with `00c908c0`
/// (`allocator` as its first argument, element size 4); the tail is moved with `memMove` and
/// the words are copied one at a time. The class is unconfirmed (the template instance
/// sits in this unit).
pub fn fn_00cd1d80(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove_count: i32,
    source: u32,
    insert_count: i32,
) {
    let size = e.mem.i32(this.addr() + 4);
    let tail_count = size.wrapping_sub(index).wrapping_sub(remove_count);
    let new_size = size.wrapping_sub(remove_count).wrapping_add(insert_count);
    let capacity = (e.mem.u32(this.addr() + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let count = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, this, count, 4u32]);
    }
    let data = e.mem.u32(this.addr());
    let source_start = data.wrapping_add((index.wrapping_add(remove_count) as u32).wrapping_mul(4));
    let destination = data.wrapping_add((index.wrapping_add(insert_count) as u32).wrapping_mul(4));
    e.call(
        MEM_MOVE,
        &args![
            destination,
            source_start,
            (tail_count as u32).wrapping_mul(4)
        ],
    );
    let mut target = e
        .mem
        .u32(this.addr())
        .wrapping_add((index as u32).wrapping_mul(4));
    let mut from = source;
    let mut remaining = insert_count;
    while remaining > 0 {
        let word = e.mem.u32(from);
        e.mem.set_u32(target, word);
        target = target.wrapping_add(4);
        from = from.wrapping_add(4);
        remaining -= 1;
    }
    e.mem.set_i32(this.addr() + 4, new_size);
}

// Translated from 00cd1e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts `count` words from `source` at `index` of the array: `00cd1d80` with nothing
/// removed. The class is unconfirmed.
pub fn fn_00cd1e20(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    fn_00cd1d80(e, this, allocator, index, 0, source, count);
}

/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The growth routine `(allocator, array, element size)` (`00c90940`, cdecl).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkStringPtr`: constructor (`00cb0a60`), destructor (`00cb0c00`) and the assignment from a
/// `const char *` (`00cb0c10`, one stack word), all `thiscall`.
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN_TEXT: u32 = 0x00cb_0c10;
/// `hkOstream`: constructor over a buffer (`00cb1600`, `RET 8`: buffer, size),
/// `operator<<(const char *)` (`00cc01c0`, `RET 4`), `operator<<(int)` (`00cc0280`, `RET 4`)
/// and the destructor (`00cc00e0`). Each `<<` answers the stream.
const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
const OSTREAM_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_INT: u32 = 0x00cc_0280;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The error handler object (the pointer at `01268360`): slot `+0xc` reports
/// `(severity, id, text, file, line)`, slot `+0x1c` starts a timer `(id, name)` and slot
/// `+0x20` ends it.
const ERROR_HANDLER: u32 = 0x0126_8360;
/// `".\hkVisualDebugger.cpp"` and the size of the message buffer.
const SOURCE_FILE: u32 = 0x010c_ae1c;
const MESSAGE_SIZE: u32 = 0x200;
/// The timer ids and names (`0x1293ade8` `"Serving"`, `0x1293adef` for the shut-down and
/// constructor timers, `0x76e3a642` `"Client Dies"`).
const TIMER_SERVING: u32 = 0x1293_ade8;
const TIMER_VISUAL_DEBUGGER: u32 = 0x1293_adef;
const TIMER_CLIENT_DIES: u32 = 0x76e3_a642;
const NAME_SERVING: u32 = 0x010c_af40;
const NAME_SHUTTING_DOWN: u32 = 0x010c_afb8;
const NAME_CLIENT_DIES: u32 = 0x010c_af8c;
const NAME_VISUAL_DEBUGGER: u32 = 0x010c_b0e0;
/// The message texts, by the address the exe passes.
const TEXT_SERVER_CREATED_PREFIX: u32 = 0x010c_aef8;
const TEXT_SERVER_CREATED_SUFFIX: u32 = 0x010c_af30;
const TEXT_SERVER_CREATE_FAILED: u32 = 0x010c_ae90;
const TEXT_SERVER_EXISTS: u32 = 0x010c_ae38;
const TEXT_CLIENT_DELETED: u32 = 0x010c_afa8;
const TEXT_SERVER_DELETED: u32 = 0x010c_af98;
const TEXT_CLIENT_DIED: u32 = 0x010c_af48;
const TEXT_NEW_CLIENT: u32 = 0x010c_afd8;
const TEXT_INSTANCE_CREATED: u32 = 0x010c_b0b8;
/// The three names the constructor adds with `00cd12e0` (`"Debug Display"` ...).
const PROCESS_NAMES: [u32; 3] = [0x010c_ae0c, 0x010c_41b0, 0x010c_b0a8];
/// `hkStopwatch`: `getTickCounter` (`00d60570`) and `getTicksPerSecond` (`00d60580`) answer a
/// 64-bit number in `EDX:EAX`; `divide64(low, high, divisor low, divisor high)` (`00d60550`,
/// cdecl) answers the quotient in `ST0`.
const TICK_COUNTER: u32 = 0x00d6_0570;
const TICKS_PER_SECOND: u32 = 0x00d6_0580;
const DIVIDE64: u32 = 0x00d6_0550;
/// `1000.0` (`double`), the default step `16.0` (`float`) and the smallest step that replaces
/// it (`float`, `1.1920929e-07`).
const THOUSAND: u32 = 0x0101_7b70;
const DEFAULT_STEP: u32 = 0x0101_e57c;
const STEP_EPSILON: u32 = 0x0104_da4c;
/// `hkOArchive` routines on a client's archive (at `+0x18` of the client): `write32u` (`00d60960`),
/// an unnamed writer (`00d605d0`), `writeFloat32` (`00d609c0`) and `getStreamWriter` (`00d60800`).
const ARCHIVE_WRITE32U: u32 = 0x00d6_0960;
const ARCHIVE_WRITE_FLAG: u32 = 0x00d6_05d0;
const ARCHIVE_WRITE_FLOAT32: u32 = 0x00d6_09c0;
const ARCHIVE_STREAM_WRITER: u32 = 0x00d6_0800;
/// The client constructor (`00d60f30`, `thiscall` on a 0x50-byte block, `RET 0xc`), its start
/// routine (`00d60d80`), `hkVersionReporter::sendVersionInformation` (`00d61290`, cdecl) and
/// `hkProcessRegisterUtil::registerAllCommonProcesses` (`00d61420`, cdecl, no arguments).
const CLIENT_CONSTRUCT: u32 = 0x00d6_0f30;
const CLIENT_START: u32 = 0x00d6_0d80;
const SEND_VERSION_INFORMATION: u32 = 0x00d6_1290;
const REGISTER_COMMON_PROCESSES: u32 = 0x00d6_1420;
/// The variable at `011b3df0` holding the address of the function that creates the server
/// socket object (the exe calls through it).
const SERVER_FACTORY: u32 = 0x011b_3df0;
/// The vtables `hkVisualDebugger` stores (`010cb100`) and the base class's (`0102e388`), and the
/// label the constructor stores at `+0x88` (`"Frame Timer"`).
const VISUAL_DEBUGGER_VTABLE: u32 = 0x010c_b100;
const BASE_VTABLE: u32 = 0x0102_e388;
const FRAME_TIMER_LABEL: u32 = 0x010c_b0f0;

/// The allocator behind the memory router (`00c85750`, its field `+0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

fn error_handler(e: &Engine) -> u32 {
    e.global(ERROR_HANDLER)
}

/// The handler's slot `+0x1c`: starts a timer.
fn timer_begin(e: &mut Engine, id: u32, name: u32) {
    let handler = error_handler(e);
    e.vcall(handler, 0x1c, &args![id, name]);
}

/// The handler's slot `+0x20`: ends the timer.
fn timer_end(e: &mut Engine) {
    let handler = error_handler(e);
    e.vcall(handler, 0x20, &args![]);
}

/// Reports a message: an `hkOstream` over a 0x200-byte buffer, `build` writing to it (each
/// call taking the stream the one before answered), then the handler's slot `+0xc` with
/// `(0, 0xffffffff, buffer, ".\hkVisualDebugger.cpp", line)` and the stream's destructor.
fn report_message(e: &mut Engine, line: u32, build: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(0xc + MESSAGE_SIZE, |e, stream| {
        let buffer = stream.addr() + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, MESSAGE_SIZE]);
        build(e, stream.addr());
        let handler = error_handler(e);
        e.vcall(
            handler,
            0xc,
            &args![0u32, 0xffff_ffffu32, buffer, SOURCE_FILE, line],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
}

/// `stream << text`.
fn write_text(e: &mut Engine, stream: u32, text: u32) -> u32 {
    e.call(OSTREAM_TEXT, &args![stream, text]).u32()
}

fn tick_counter(e: &mut Engine) -> u64 {
    e.call(TICK_COUNTER, &args![]).u64()
}

fn get_u64(e: &Engine, address: u32) -> u64 {
    e.mem.u64(address)
}

fn set_u64(e: &mut Engine, address: u32, value: u64) {
    e.mem.set_u64(address, value);
}

/// Appends `value` to the array of words at `array` (data, size, capacity word), growing it
/// first through the memory router's allocator when the size equals the masked capacity.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The end of every array clear: the size at `size_field` is zeroed and, unless the capacity
/// word at `capacity_field` is negative (the don't-deallocate flag), the data is given back
/// (`bufFree(data, (capacity & 0x3fffffff) * entry_size)`); then the data pointer is null and
/// the capacity word `0x80000000`.
fn clear_array(
    e: &mut Engine,
    allocator: u32,
    data_field: u32,
    size_field: u32,
    capacity_field: u32,
    entry_size: u32,
) {
    let capacity = e.mem.u32(capacity_field);
    e.mem.set_u32(size_field, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(data_field);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(entry_size)],
        );
    }
    e.mem.set_u32(data_field, 0);
    e.mem.set_u32(capacity_field, 0x8000_0000);
}

// Translated from 00cd0d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::serve` (Xbox PDB), `thiscall`, `RET 4` (`port`): starts a timer
/// (`"Serving"`), then, when there is no server yet (`+8` null), creates one by calling the
/// function the variable at `011b3df0` holds. A server that could not be created reports
/// `"Server could not be created ..."` (line 0x6b); a created one is asked (its slot `+0x20`)
/// to listen on `port` and reports `"Server created and will poll for new client(s) on port
/// <port> every frame"` (line 0x67). When there already is a server the message is `"Server
/// has already been created ..."` (line 0x70). The timer ends last.
pub fn hk_visual_debugger_serve(e: &mut Engine, this: Ptr, port: u32) {
    timer_begin(e, TIMER_SERVING, NAME_SERVING);
    if e.mem.u32(this.addr() + 8) == 0 {
        let factory: u32 = e.global(SERVER_FACTORY);
        let server = e.call(factory, &args![]).u32();
        e.mem.set_u32(this.addr() + 8, server);
        if server == 0 {
            report_message(e, 0x6b, |e, stream| {
                write_text(e, stream, TEXT_SERVER_CREATE_FAILED);
            });
        } else {
            e.vcall(server, 0x20, &args![port]);
            report_message(e, 0x67, |e, stream| {
                let stream = write_text(e, stream, TEXT_SERVER_CREATED_PREFIX);
                let stream = e.call(OSTREAM_INT, &args![stream, port]).u32();
                write_text(e, stream, TEXT_SERVER_CREATED_SUFFIX);
            });
        }
    } else {
        report_message(e, 0x70, |e, stream| {
            write_text(e, stream, TEXT_SERVER_EXISTS);
        });
    }
    timer_end(e);
}

// Translated from 00cd0e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::writeStep` (Xbox PDB), `thiscall`, `RET 8` (`index`, `step`): for the
/// client at `index` of the array at `+0x10` (8-byte entries, the client pointer second), asks
/// the object embedded at `client + 0xc` (its slot `+0x10`); when that says yes the client's
/// archive (`client + 0x18`) gets `write32u(5)`, the unnamed writer `00d605d0(0)`,
/// `writeFloat32(step)` (the float through `FLD`/`FSTP`) and the stream writer
/// (`getStreamWriter`, `00d60800`) is flushed (its slot `+0x14`).
pub fn hk_visual_debugger_write_step(e: &mut Engine, this: Ptr, index: u32, step: f32) {
    let clients = e.mem.u32(this.addr() + 0x10);
    let entry = clients.wrapping_add(index.wrapping_mul(8));
    let client = e.mem.u32(entry + 4);
    if !e.vcall(client + 0xc, 0x10, &args![]).bool() {
        return;
    }
    let client = e.mem.u32(entry + 4);
    let archive = e.mem.u32(client + 0x18);
    e.call(ARCHIVE_WRITE32U, &args![archive, 5u32]);
    e.call(ARCHIVE_WRITE_FLAG, &args![archive, 0u32]);
    e.call(ARCHIVE_WRITE_FLOAT32, &args![archive, x87_quiet(step)]);
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    e.vcall(writer, 0x14, &args![]);
}

// Translated from 00cd0e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::deleteClient` (Xbox PDB), `thiscall`, `RET 4` (`index`): releases the two
/// references of the client entry at `index` of the array at `+0x10` (8-byte entries, the
/// second word first; each when not null; they are not cleared), takes one off the count at
/// `+0x14` and, unless the entry was the last, copies the last entry's two words over it.
pub fn hk_visual_debugger_delete_client(e: &mut Engine, this: Ptr, index: u32) {
    let base = this.addr();
    let entry = e.mem.u32(base + 0x10).wrapping_add(index.wrapping_mul(8));
    let client = e.mem.u32(entry + 4);
    if client != 0 {
        e.call(REMOVE_REFERENCE, &args![client]);
    }
    let first = e.mem.u32(entry);
    if first != 0 {
        e.call(REMOVE_REFERENCE, &args![first]);
    }
    let count = e.mem.u32(base + 0x14).wrapping_sub(1);
    e.mem.set_u32(base + 0x14, count);
    if count != index {
        let data = e.mem.u32(base + 0x10);
        let target = data.wrapping_add(index.wrapping_mul(8));
        let last = data.wrapping_add(count.wrapping_mul(8));
        for word in 0..2 {
            let moved = e.mem.u32(last + 4 * word);
            e.mem.set_u32(target + 4 * word, moved);
        }
    }
}

// Translated from 00cd0ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::step` (Xbox PDB), `thiscall`, `RET 4` (`delta`). The step the clients
/// are told is the float at `0101e57c`, replaced by `delta` when its absolute value is above
/// the float at `0104da4c` (a NaN is not). With the timers on (the byte at `+0x5c`) the
/// frame time first becomes the step: the 64-bit total at `+0x68` (plus the time since the
/// start at `+0x60` while the byte at `+0x80` says the timer runs) divided by the ticks per
/// second (`hkStopwatch::divide64`) times 1000 (rounded to a `float`), the byte at `+0x80`
/// cleared, and the totals at `+0x68` and `+0x78` increased by the time since the starts
/// at `+0x60` and `+0x70`, the count at `+0x84` by one. The byte at `+0x5c` is set; then the
/// object's slot `+0x10` runs (`pollForNewClients`). For each client from the last: when
/// its embedded object (`client + 0xc`) says slot `+0x10` is true its slot `+0x18` gets
/// the step and `writeStep` runs; when slot `+0x10` is false afterwards the client died: a
/// timer `"Client Dies"` brackets the message `"Client has died, cleaning up ..."` (line 0x120)
/// and `deleteClient` runs. Last, with the timers on, the counters are cleared, the byte at
/// `+0x80` set and the tick counter stored as the starts at `+0x60` and `+0x70`.
pub fn hk_visual_debugger_step(e: &mut Engine, this: Ptr, delta: f32) {
    let base = this.addr();
    let default_step: u32 = e.global(DEFAULT_STEP);
    let mut step_bits = x87_store32(x87_load32(default_step));
    if e.mem.u8(base + 0x5c) != 0 {
        let mut total = get_u64(e, base + 0x68);
        if e.mem.u8(base + 0x80) != 0 {
            let now = tick_counter(e);
            total = total.wrapping_add(now.wrapping_sub(get_u64(e, base + 0x60)));
        }
        let per_second = e.call(TICKS_PER_SECOND, &args![]).u64();
        let ratio = e
            .call(
                DIVIDE64,
                &args![
                    total as u32,
                    (total >> 32) as u32,
                    per_second as u32,
                    (per_second >> 32) as u32
                ],
            )
            .f64();
        let thousand: f64 = e.global(THOUSAND);
        e.mem.set_u8(base + 0x80, 0);
        step_bits = x87_store32(x87_mul(ratio, thousand));
        let now = tick_counter(e);
        let since_start = now.wrapping_sub(get_u64(e, base + 0x60));
        let first = get_u64(e, base + 0x68).wrapping_add(since_start);
        set_u64(e, base + 0x68, first);
        let since_second_start = now.wrapping_sub(get_u64(e, base + 0x70));
        let second = get_u64(e, base + 0x78).wrapping_add(since_second_start);
        set_u64(e, base + 0x78, second);
        let count = e.mem.u32(base + 0x84).wrapping_add(1);
        e.mem.set_u32(base + 0x84, count);
    }
    e.mem.set_u8(base + 0x5c, 1);
    let magnitude = x87_store32(x87_load32(delta.to_bits()).abs());
    let epsilon: u32 = e.global(STEP_EPSILON);
    if x87_load32(magnitude) > x87_load32(epsilon) {
        step_bits = x87_store32(x87_load32(delta.to_bits()));
    }
    e.vcall(base, 0x10, &args![]);
    let mut index = e.mem.i32(base + 0x14).wrapping_sub(1);
    while index >= 0 {
        let clients = e.mem.u32(base + 0x10);
        let entry = clients.wrapping_add((index as u32).wrapping_mul(8));
        let client = e.mem.u32(entry + 4);
        if e.vcall(client + 0xc, 0x10, &args![]).bool() {
            let client = e.mem.u32(entry + 4);
            let quiet = x87_store32(x87_load32(step_bits));
            e.vcall(client + 0xc, 0x18, &args![quiet]);
            hk_visual_debugger_write_step(e, this, index as u32, f32::from_bits(step_bits));
        }
        let client = e.mem.u32(entry + 4);
        if !e.vcall(client + 0xc, 0x10, &args![]).bool() {
            timer_begin(e, TIMER_CLIENT_DIES, NAME_CLIENT_DIES);
            report_message(e, 0x120, |e, stream| {
                write_text(e, stream, TEXT_CLIENT_DIED);
            });
            timer_end(e);
            hk_visual_debugger_delete_client(e, this, index as u32);
        }
        index = index.wrapping_sub(1);
    }
    if e.mem.u8(base + 0x5c) != 0 {
        for offset in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x84] {
            e.mem.set_u32(base + offset, 0);
        }
        e.mem.set_u8(base + 0x80, 1);
        let now = tick_counter(e);
        set_u64(e, base + 0x60, now);
        set_u64(e, base + 0x70, now);
    }
}

// Translated from 00cd10a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `key`): removes the entry whose first word
/// is `key` from the array at `+0x28` (12-byte entries, count at `+0x2c`; the last entry's
/// three words fill its place) and, only then, calls each function of the array at `+0x34`
/// (count `+0x38`, cdecl): `(key, 0, (i * 4) & 0xffffff00, 0, data40[i])`, the third word being
/// the `bool` false in its low byte (the other bytes are what the exe's register held).
pub fn fn_00cd10a0(e: &mut Engine, this: Ptr, key: u32) {
    let base = this.addr();
    let count = e.mem.i32(base + 0x2c);
    let mut index = 0i32;
    let mut entry = e.mem.u32(base + 0x28);
    if count <= 0 {
        return;
    }
    while e.mem.u32(entry) != key {
        index += 1;
        entry = entry.wrapping_add(12);
        if index >= count {
            return;
        }
    }
    let size = e.mem.u32(base + 0x2c).wrapping_sub(1);
    e.mem.set_u32(base + 0x2c, size);
    if size as i32 != index {
        let data = e.mem.u32(base + 0x28);
        let target = data.wrapping_add((index as u32).wrapping_mul(12));
        let last = data.wrapping_add(size.wrapping_mul(12));
        for word in 0..3 {
            let moved = e.mem.u32(last + 4 * word);
            e.mem.set_u32(target + 4 * word, moved);
        }
    }
    let mut listener = 0u32;
    while (listener as i32) < e.mem.i32(base + 0x38) {
        let offset = listener.wrapping_mul(4);
        let user = e.mem.u32(e.mem.u32(base + 0x40).wrapping_add(offset));
        let function = e.mem.u32(e.mem.u32(base + 0x34).wrapping_add(offset));
        e.call(
            function,
            &args![key, 0u32, offset & 0xffff_ff00, 0u32, user],
        );
        listener += 1;
    }
}

// Translated from 00cd1140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `value`): finds `value` in the array of words
/// at `+0x34` (count `+0x38`) and, when found, removes it (the last word fills its place) and
/// the word at the same index of the array at `+0x40` (count `+0x44`) the same way.
pub fn fn_00cd1140(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let count = e.mem.i32(base + 0x38);
    if count <= 0 {
        return;
    }
    let mut index = 0i32;
    let mut entry = e.mem.u32(base + 0x34);
    while e.mem.u32(entry) != value {
        index += 1;
        entry = entry.wrapping_add(4);
        if index >= count {
            return;
        }
    }
    let size = e.mem.u32(base + 0x38).wrapping_sub(1);
    e.mem.set_u32(base + 0x38, size);
    if size as i32 != index {
        let data = e.mem.u32(base + 0x34);
        let moved = e.mem.u32(data.wrapping_add(size.wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
    let size = e.mem.u32(base + 0x44).wrapping_sub(1);
    e.mem.set_u32(base + 0x44, size);
    if size as i32 != index {
        let data = e.mem.u32(base + 0x40);
        let moved = e.mem.u32(data.wrapping_add(size.wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
}

// Translated from 00cd11a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::shutdown` (Xbox PDB), `thiscall`, no stack words: starts a timer
/// (`"Shuting down Visual Debugger.."`); for each client from the last, `writeStep(index, 0.0)`,
/// `deleteClient(index)` and the message `"Client deleted."` (line 0x51); then a server
/// (`+8`), if there is one, is deleted through its slot `+0` with 1, cleared, and `"Server
/// deleted."` (line 0x58) is reported. The exe ends in a tail call of the handler's timer end
/// (slot `+0x20`), which is made here.
pub fn hk_visual_debugger_shutdown(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    timer_begin(e, TIMER_VISUAL_DEBUGGER, NAME_SHUTTING_DOWN);
    let mut index = e.mem.i32(base + 0x14).wrapping_sub(1);
    while index >= 0 {
        hk_visual_debugger_write_step(e, this, index as u32, 0.0);
        hk_visual_debugger_delete_client(e, this, index as u32);
        report_message(e, 0x51, |e, stream| {
            write_text(e, stream, TEXT_CLIENT_DELETED);
        });
        index = index.wrapping_sub(1);
    }
    let server = e.mem.u32(base + 8);
    if server != 0 {
        e.vcall(server, 0, &args![1u32]);
        e.mem.set_u32(base + 8, 0);
        report_message(e, 0x58, |e, stream| {
            write_text(e, stream, TEXT_SERVER_DELETED);
        });
    }
    timer_end(e);
}

// Translated from 00cd12b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): from the last client to the first,
/// `deleteClient` for each whose first word (at `+0x10` data plus 8 * index) is null.
pub fn fn_00cd12b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = e.mem.i32(base + 0x14).wrapping_sub(1);
    while index >= 0 {
        let first = e.mem.u32(
            e.mem
                .u32(base + 0x10)
                .wrapping_add((index as u32).wrapping_mul(8)),
        );
        if first == 0 {
            hk_visual_debugger_delete_client(e, this, index as u32);
        }
        index = index.wrapping_sub(1);
    }
}

// Translated from 00cd12e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `text`): appends a name to the
/// `hkStringPtr` array at `+0x50` (size `+0x54`, capacity word `+0x58`, 4 bytes an entry),
/// growing it first (`00c90940`, element size 4) when the size equals the masked capacity:
/// the entry is constructed (`00cb0a60`, when its address is not null) and the end of the
/// function is a tail call of the assignment from `text` (`00cb0c10`), whose answer is
/// returned.
pub fn fn_00cd12e0(e: &mut Engine, this: Ptr, text: u32) -> u32 {
    let array = this.addr() + 0x50;
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if entry != 0 {
        e.call(STRING_PTR_CONSTRUCT, &args![entry]);
    }
    let size = e.mem.u32(array + 4);
    let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(STRING_PTR_ASSIGN_TEXT, &args![entry, text]).u32()
}

// Translated from 00cd1330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: `first`, `second`, `_unused_3`,
/// `fourth`; the third word is never read): appends the three words `(first, second,
/// fourth)` to the array at `+0x28` (12-byte entries, size `+0x2c`, capacity word `+0x30`),
/// growing it first (`00c90940`, element size 12) when it is full, then calls each function
/// of the array at `+0x34` (count `+0x38`, cdecl): `(first, second, (i * 4) & 0xffffff00 | 1,
/// fourth, data40[i])`, the third word being the `bool` true in its low byte.
pub fn fn_00cd1330(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    _unused_3: u32,
    fourth: u32,
) {
    let base = this.addr();
    let array = base + 0x28;
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 12u32]);
    }
    let size = e.mem.u32(array + 4);
    let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(12));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(entry, first);
    e.mem.set_u32(entry + 4, second);
    e.mem.set_u32(entry + 8, fourth);
    let mut listener = 0u32;
    while (listener as i32) < e.mem.i32(base + 0x38) {
        let offset = listener.wrapping_mul(4);
        let user = e.mem.u32(e.mem.u32(base + 0x40).wrapping_add(offset));
        let function = e.mem.u32(e.mem.u32(base + 0x34).wrapping_add(offset));
        e.call(
            function,
            &args![first, second, (offset & 0xffff_ff00) | 1, fourth, user],
        );
        listener += 1;
    }
}

// Translated from 00cd13c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `function`, `user`): appends `function` to
/// the array of words at `+0x34` (size `+0x38`, capacity word `+0x3c`) and `user` to the array
/// of words at `+0x40` (size `+0x44`, capacity word `+0x48`), each growing first
/// (`00c90940`, element size 4) when full, each with its own fetch of the memory router's
/// allocator.
pub fn fn_00cd13c0(e: &mut Engine, this: Ptr, function: u32, user: u32) {
    push_word(e, this.addr() + 0x34, function);
    push_word(e, this.addr() + 0x40, user);
}

// Translated from 00cd1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::createClient` (Xbox PDB), `thiscall`, `RET 0xc` (`id`, `stream`,
/// `version_target`): with a `version_target` that is not null sends it the version
/// information (`00d61290`, cdecl). Then a client entry is appended to the array at `+0x10`
/// (8-byte entries, size `+0x14`, capacity word `+0x18`; reserved first, element size 8, for the
/// larger of twice the masked capacity and the new size when it is smaller): `id` first, then a
/// 0x50-byte client allocated through the allocator's slot `+4` (its 16-bit size word at `+4`
/// set to 0x50) and constructed (`00d60f30`, arguments `(this + 0x1c, stream, version_target)`) and
/// started (`00d60d80`). The names at `+0x50` (`hkStringPtr`s, count `+0x54`) are offered to
/// the object embedded at `client + 8`: slot `+8` finds an index (negative: not known), slot
/// `+0x10` is called with each index that is not negative. Last `writeStep(size - 1, 0.0)`.
pub fn hk_visual_debugger_create_client(
    e: &mut Engine,
    this: Ptr,
    id: u32,
    stream: u32,
    version_target: u32,
) {
    let base = this.addr();
    if version_target != 0 {
        e.call(SEND_VERSION_INFORMATION, &args![version_target]);
    }
    let array = base + 0x10;
    let allocator = router_allocator(e);
    let old_size = e.mem.i32(array + 4);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    let wanted = old_size.wrapping_add(1);
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let count = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, count, 8u32]);
    }
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, (old_size as u32).wrapping_add(1));
    let entry = data.wrapping_add((old_size as u32).wrapping_mul(8));
    e.mem.set_u32(entry, id);
    let allocator = router_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x50u32]).u32();
    e.mem.set_u16(block + 4, 0x50);
    let client = e
        .call(
            CLIENT_CONSTRUCT,
            &args![block, base + 0x1c, stream, version_target],
        )
        .u32();
    e.mem.set_u32(entry + 4, client);
    e.call(CLIENT_START, &args![client]);
    let mut name = 0i32;
    while name < e.mem.i32(base + 0x54) {
        let word = e.mem.u32(
            e.mem
                .u32(base + 0x50)
                .wrapping_add((name as u32).wrapping_mul(4)),
        );
        let client = e.mem.u32(entry + 4);
        let found = e.vcall(client + 8, 8, &args![word & 0xffff_fffe]).i32();
        if found >= 0 {
            let client = e.mem.u32(entry + 4);
            e.vcall(client + 8, 0x10, &args![found]);
        }
        name += 1;
    }
    let last = e.mem.u32(base + 0x14).wrapping_sub(1);
    hk_visual_debugger_write_step(e, this, last, 0.0);
}

// Translated from 00cd1520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::pollForNewClients` (Xbox PDB), `thiscall`, no stack words: nothing
/// happens while the byte at `+0xc` is set. With the timers on (`+0x5c`) and running (`+0x80`)
/// the running flag is cleared and the 64-bit totals at `+0x68` and `+0x78` increased by the
/// time since the starts at `+0x60` and `+0x70`, the count at `+0x84` by one. A server (`+8`)
/// is asked for a new client (its slot `+0x28`); one that comes is reported (`"A new network
/// client has been received ..."`, line 0xe8) and `createClient(client, client + 8, client + 0x14)`
/// runs. With the timers on the running flag is set again and the tick counter stored as the
/// starts at `+0x60` and `+0x70`.
pub fn hk_visual_debugger_poll_for_new_clients(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0xc) != 0 {
        return;
    }
    if e.mem.u8(base + 0x5c) != 0 && e.mem.u8(base + 0x80) != 0 {
        e.mem.set_u8(base + 0x80, 0);
        let now = tick_counter(e);
        let first = get_u64(e, base + 0x68).wrapping_add(now.wrapping_sub(get_u64(e, base + 0x60)));
        set_u64(e, base + 0x68, first);
        let second =
            get_u64(e, base + 0x78).wrapping_add(now.wrapping_sub(get_u64(e, base + 0x70)));
        set_u64(e, base + 0x78, second);
        let count = e.mem.u32(base + 0x84).wrapping_add(1);
        e.mem.set_u32(base + 0x84, count);
    }
    let server = e.mem.u32(base + 8);
    if server != 0 {
        let client = e.vcall(server, 0x28, &args![]).u32();
        if client != 0 {
            report_message(e, 0xe8, |e, stream| {
                write_text(e, stream, TEXT_NEW_CLIENT);
            });
            hk_visual_debugger_create_client(e, this, client, client + 8, client + 0x14);
        }
    }
    if e.mem.u8(base + 0x5c) != 0 {
        e.mem.set_u8(base + 0x80, 1);
        let now = tick_counter(e);
        set_u64(e, base + 0x60, now);
        set_u64(e, base + 0x70, now);
    }
}

// Translated from 00cd1780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::hkVisualDebugger` (Xbox PDB), `thiscall`, `RET 8` (`processes`,
/// `reference`): sets the vtable `010cb100`, clears the server (`+8`) and the byte at `+0xc`,
/// sets the 16-bit word at `+6` to 1 and makes the empty arrays at `+0x10`, `+0x1c`, `+0x28`,
/// `+0x34`, `+0x40` and `+0x50` (data 0, size 0, capacity word `0x80000000`), stores `reference`
/// at `+0x4c`, the label `"Frame Timer"` at `+0x88` and zeros in the timer fields `+0x5c` to
/// `+0x84`. The array of words at `+0x1c` is then given room for the `processes` array's
/// words (a block through the memory router's allocator's slot `+0xc`, freeing an old
/// one through slot `+0x10` unless flagged; capacity word = the granted bytes / 4) and they are copied
/// there; each is told the debugger (its slot `+8`). A timer `"Visual Debugger"` brackets the
/// message `"VDB Server instance has been created"` (line 0x28), `registerAllCommonProcesses`
/// (`00d61420`) and the three names `00cd12e0` adds; then `reference` gets a reference added
/// (`00c90510`) when not null. Returns `this`.
pub fn hk_visual_debugger_hk_visual_debugger(
    e: &mut Engine,
    this: Ptr,
    processes: Ptr,
    reference: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VISUAL_DEBUGGER_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0xc, 0);
    e.mem.set_u16(base + 6, 1);
    for array in [0x10u32, 0x1c, 0x28, 0x34, 0x40] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u32(base + array + 4, 0);
        e.mem.set_u32(base + array + 8, 0x8000_0000);
    }
    e.mem.set_u32(base + 0x4c, reference);
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x58, 0x8000_0000);
    e.mem.set_u8(base + 0x5c, 0);
    e.mem.set_u32(base + 0x88, FRAME_TIMER_LABEL);
    for offset in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(base + 0x24);
    let source_size = e.mem.i32(processes.addr() + 4);
    if ((capacity_word & 0x3fff_ffff) as i32) < source_size {
        if capacity_word & 0x8000_0000 == 0 {
            let old = e.mem.u32(base + 0x1c);
            e.vcall(
                allocator,
                0x10,
                &args![old, (capacity_word & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        let mut bytes = (source_size as u32).wrapping_mul(4);
        let block = e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![cell]).u32();
            bytes = e.mem.u32(cell.addr());
            block
        });
        e.mem.set_u32(base + 0x1c, block);
        e.mem.set_u32(base + 0x24, ((bytes as i32) / 4) as u32);
    }
    let count = e.mem.i32(processes.addr() + 4);
    let data = e.mem.u32(base + 0x1c);
    e.mem.set_u32(base + 0x20, count as u32);
    let source = e.mem.u32(processes.addr());
    let mut copied = 0i32;
    while copied < count {
        let word = e
            .mem
            .u32(source.wrapping_add((copied as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((copied as u32).wrapping_mul(4)), word);
        copied += 1;
    }
    let mut index = 0i32;
    while index < e.mem.i32(base + 0x20) {
        let process = e.mem.u32(
            e.mem
                .u32(base + 0x1c)
                .wrapping_add((index as u32).wrapping_mul(4)),
        );
        e.vcall(process, 8, &args![this]);
        index += 1;
    }
    timer_begin(e, TIMER_VISUAL_DEBUGGER, NAME_VISUAL_DEBUGGER);
    report_message(e, 0x28, |e, stream| {
        write_text(e, stream, TEXT_INSTANCE_CREATED);
    });
    e.call(REGISTER_COMMON_PROCESSES, &args![]);
    for name in PROCESS_NAMES {
        fn_00cd12e0(e, this, name);
    }
    let reference = e.mem.u32(base + 0x4c);
    if reference != 0 {
        e.call(ADD_REFERENCE, &args![reference]);
    }
    timer_end(e);
    this
}

// Translated from 00cd1970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVisualDebugger::~hkVisualDebugger` (Xbox PDB), `thiscall`, no stack words: stores the
/// vtable `010cb100` and runs `shutdown` (`00cd11a0`); tells each process of the array at
/// `+0x1c` (slot `+8`) with a null; destroys the names of the `hkStringPtr` array at `+0x50`
/// (`00cb0c00`), clears the count, releases the reference at `+0x4c`, destroys the names again
/// (none are left; the exe loops over the cleared count) and gives back the arrays at `+0x50`
/// (4 bytes an entry), `+0x40` (4), `+0x34` (4), `+0x28` (12), `+0x1c` (4) and `+0x10` (8),
/// each after its own fetch of the memory router's allocator, then stores the base class's
/// vtable (`0102e388`).
pub fn hk_visual_debugger_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VISUAL_DEBUGGER_VTABLE);
    hk_visual_debugger_shutdown(e, this);
    let mut index = 0i32;
    while index < e.mem.i32(base + 0x20) {
        let process = e.mem.u32(
            e.mem
                .u32(base + 0x1c)
                .wrapping_add((index as u32).wrapping_mul(4)),
        );
        e.vcall(process, 8, &args![0u32]);
        index += 1;
    }
    let count = e.mem.i32(base + 0x54);
    let mut name = e.mem.u32(base + 0x50);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![name]);
        name = name.wrapping_add(4);
        left -= 1;
    }
    e.mem.set_u32(base + 0x54, 0);
    let reference = e.mem.u32(base + 0x4c);
    if reference != 0 {
        e.call(REMOVE_REFERENCE, &args![reference]);
    }
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x54);
    let mut name = e.mem.u32(base + 0x50);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![name]);
        name = name.wrapping_add(4);
        left -= 1;
    }
    clear_array(e, allocator, base + 0x50, base + 0x54, base + 0x58, 4);
    for (data, size, capacity, entry) in [
        (0x40u32, 0x44u32, 0x48u32, 4u32),
        (0x34, 0x38, 0x3c, 4),
        (0x28, 0x2c, 0x30, 12),
        (0x1c, 0x20, 0x24, 4),
        (0x10, 0x14, 0x18, 8),
    ] {
        let allocator = router_allocator(e);
        clear_array(
            e,
            allocator,
            base + data,
            base + size,
            base + capacity,
            entry,
        );
    }
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_1e40, deleting_destructor_00cd1e40(Ptr, u8) -> Ptr),
        entry!(0x00cd_1d80, fn_00cd1d80(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00cd_1e20, fn_00cd1e20(Ptr, u32, i32, u32, i32)),
        entry!(0x00cd0d00, hk_visual_debugger_serve(Ptr, u32)),
        entry!(0x00cd0e00, hk_visual_debugger_write_step(Ptr, u32, f32)),
        entry!(0x00cd0e60, hk_visual_debugger_delete_client(Ptr, u32)),
        entry!(0x00cd0ec0, hk_visual_debugger_step(Ptr, f32)),
        entry!(0x00cd10a0, fn_00cd10a0(Ptr, u32)),
        entry!(0x00cd1140, fn_00cd1140(Ptr, u32)),
        entry!(0x00cd11a0, hk_visual_debugger_shutdown(Ptr)),
        entry!(0x00cd12b0, fn_00cd12b0(Ptr)),
        entry!(0x00cd12e0, fn_00cd12e0(Ptr, u32) -> u32),
        entry!(0x00cd1330, fn_00cd1330(Ptr, u32, u32, u32, u32)),
        entry!(0x00cd13c0, fn_00cd13c0(Ptr, u32, u32)),
        entry!(
            0x00cd1440,
            hk_visual_debugger_create_client(Ptr, u32, u32, u32)
        ),
        entry!(0x00cd1520, hk_visual_debugger_poll_for_new_clients(Ptr)),
        entry!(0x00cd1780, hk_visual_debugger_hk_visual_debugger(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cd1970, hk_visual_debugger_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cd1e40_frees_only_when_bit_zero_is_set() {
        check(0x00cd_1e40, 0x00cd_1970);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Reserved = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An array of the given words with room for `capacity`, `_reserve` that moves it to a
    /// fresh block of the requested count, and a real `memMove`.
    fn rig(words: &[u32], capacity: u32) -> (Engine, u32, Reserved) {
        let mut e = test_engine();
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity * 4);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *w);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, 0x8000_0000 | capacity);
        let reserved: Reserved = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserved);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(a[2] * 4);
            let bytes = e.mem.bytes(old, size * 4);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        (e, array, reserved)
    }

    fn contents(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn splice_replaces_a_range_without_growing() {
        let (mut e, array, reserved) = rig(&[1, 2, 3, 4, 5], 8);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 9);
        e.call(
            0x00cd_1d80,
            &args![array, 0x55u32, 1u32, 2u32, source, 1u32],
        );
        assert_eq!(contents(&e, array), vec![1, 9, 4, 5]);
        assert!(reserved.borrow().is_empty());
    }

    #[test]
    fn insert_grows_to_the_larger_of_the_new_size_and_twice_the_capacity() {
        let (mut e, array, reserved) = rig(&[1, 2, 3, 4], 4);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 9);
        e.mem.set_u32(source + 4, 8);
        e.call(0x00cd_1e20, &args![array, 0x55u32, 1u32, source, 2u32]);
        assert_eq!(contents(&e, array), vec![1, 9, 8, 2, 3, 4]);
        // Twice the capacity (8) beats the new size (6).
        assert_eq!(*reserved.borrow(), vec![vec![0x55, array, 8, 4]]);
        let (mut e, array, reserved) = rig(&[1], 1);
        let source = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 10 + i);
        }
        e.call(0x00cd_1e20, &args![array, 7u32, 1u32, source, 4u32]);
        assert_eq!(contents(&e, array), vec![1, 10, 11, 12, 13]);
        // The new size (5) beats twice the capacity (2).
        assert_eq!(*reserved.borrow(), vec![vec![7, array, 5, 4]]);
    }

    #[test]
    fn a_non_positive_count_copies_nothing() {
        let (mut e, array, _) = rig(&[1, 2, 3], 4);
        let source = e.mem.alloc(8);
        e.call(0x00cd_1e20, &args![array, 0u32, 1u32, source, 0u32]);
        assert_eq!(contents(&e, array), vec![1, 2, 3]);
        e.call(0x00cd_1d80, &args![array, 0u32, 0u32, 3u32, source, 0u32]);
        assert_eq!(contents(&e, array), Vec::<u32>::new());
    }
}

#[cfg(test)]
mod debugger_tests {
    use super::*;

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    type Events = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;
    type Lives = Rc<RefCell<HashMap<u32, VecDeque<bool>>>>;

    const HANDLER_TABLE: u32 = 0x0061_0000;
    const ALLOCATOR_OBJECT: u32 = 0x0062_0000;
    const ALLOCATOR_TABLE: u32 = 0x0062_1000;
    const EMBEDDED_TABLE: u32 = 0x0063_0000;
    const WRITER_TABLE: u32 = 0x0064_0000;
    const SERVER_TABLE: u32 = 0x0065_0000;
    const PROCESS_TABLE: u32 = 0x0066_0000;
    const DOUBLES: u32 = 0x0067_0000;

    /// Registers a double at `addr` that appends `(tag, arguments)` to `events` and answers
    /// `answer`.
    fn tag_calls(e: &mut Engine, events: &Events, addr: u32, tag: &'static str, answer: u32) {
        let kept = events.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push((tag, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn tags(events: &Events) -> Vec<&'static str> {
        events.borrow().iter().map(|(tag, _)| *tag).collect()
    }

    fn calls(events: &Events, tag: &str) -> Vec<Vec<u32>> {
        events
            .borrow()
            .iter()
            .filter(|(name, _)| *name == tag)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The error handler (its begin/end timer and report slots), the `hkOstream` functions,
    /// the memory router with an allocator (slot 4 `blockAlloc`, slot 0xc `bufAlloc`, slot 0x10
    /// `bufFree`) and the stream writer of a client's archive, all logging into `events`.
    /// Returns the address of the handler object.
    fn install_world(e: &mut Engine, events: &Events) -> u32 {
        e.map(ERROR_HANDLER, 4);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(ERROR_HANDLER, handler);
        e.mem.set_u32(handler, HANDLER_TABLE);
        let mut table = vec![0u32; 9];
        table[3] = DOUBLES;
        table[7] = DOUBLES + 0x10;
        table[8] = DOUBLES + 0x20;
        e.put_vtable(HANDLER_TABLE, &table);
        tag_calls(e, events, DOUBLES, "report", 0);
        tag_calls(e, events, DOUBLES + 0x10, "begin", 0);
        tag_calls(e, events, DOUBLES + 0x20, "end", 0);
        tag_calls(e, events, OSTREAM_CONSTRUCT, "construct", 0);
        for (address, tag) in [(OSTREAM_TEXT, "text"), (OSTREAM_INT, "int")] {
            let kept = events.clone();
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((tag, a.to_vec()));
                Ret {
                    eax: a[0] + 1,
                    ..Ret::default()
                }
            });
        }
        tag_calls(e, events, OSTREAM_DESTRUCT, "destruct", 0);
        // The allocator.
        e.map(ALLOCATOR_OBJECT, 0x100);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[1] = DOUBLES + 0x30;
        table[3] = DOUBLES + 0x40;
        table[4] = DOUBLES + 0x50;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        let kept = events.clone();
        e.register_double(DOUBLES + 0x30, move |e, a| {
            kept.borrow_mut().push(("alloc", a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let kept = events.clone();
        e.register_double(DOUBLES + 0x40, move |e, a| {
            kept.borrow_mut().push(("buf_alloc", a.to_vec()));
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        tag_calls(e, events, DOUBLES + 0x50, "free", 0);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The archive writer: its slot 0x14 flushes.
        let writer = e.mem.alloc(8);
        e.mem.set_u32(writer, WRITER_TABLE);
        let mut table = vec![0u32; 6];
        table[5] = DOUBLES + 0x60;
        e.put_vtable(WRITER_TABLE, &table);
        tag_calls(e, events, DOUBLES + 0x60, "flush", 0);
        tag_calls(e, events, ARCHIVE_STREAM_WRITER, "stream_writer", writer);
        tag_calls(e, events, ARCHIVE_WRITE32U, "write32u", 0);
        tag_calls(e, events, ARCHIVE_WRITE_FLAG, "write_flag", 0);
        tag_calls(e, events, ARCHIVE_WRITE_FLOAT32, "write_float", 0);
        handler
    }

    /// A client whose embedded object (at +0xc) answers slot 0x10 from `lives` (the value at the
    /// front is taken; the last one stays) and logs slot 0x18; its archive pointer (+0x18) is
    /// `client + 0x40`.
    fn install_client(e: &mut Engine, lives: &Lives, answers: &[bool]) -> u32 {
        let client = e.mem.alloc(0x50);
        e.mem.set_u32(client + 0xc, EMBEDDED_TABLE);
        e.mem.set_u32(client + 0x18, client + 0x40);
        lives
            .borrow_mut()
            .insert(client + 0xc, answers.iter().copied().collect());
        client
    }

    fn install_embedded_table(e: &mut Engine, events: &Events, lives: &Lives) {
        let mut table = vec![0u32; 7];
        table[2] = DOUBLES + 0x70;
        table[4] = DOUBLES + 0x80;
        table[6] = DOUBLES + 0x90;
        e.put_vtable(EMBEDDED_TABLE, &table);
        let lives = lives.clone();
        let kept = events.clone();
        e.register_double(DOUBLES + 0x80, move |_, a| {
            let mut lives = lives.borrow_mut();
            let queue = lives.get_mut(&a[0]).expect("a known client");
            let answer = if queue.len() > 1 {
                queue.pop_front().unwrap()
            } else {
                queue[0]
            };
            kept.borrow_mut()
                .push(("alive", vec![a[0], u32::from(answer)]));
            Ret {
                eax: u32::from(answer),
                ..Ret::default()
            }
        });
        tag_calls(e, events, DOUBLES + 0x90, "send_step", 0);
        // Slot 8 finds a name's index: the word itself when it is below 0x100, else -1.
        e.register(DOUBLES + 0x70, |_, a| Ret {
            eax: if a[1] < 0x100 { a[1] } else { u32::MAX },
            ..Ret::default()
        });
        tag_calls(e, events, DOUBLES + 0xa0, "select", 0);
    }

    /// A debugger object with the client array filled from `clients` (8-byte entries `(id,
    /// client)`).
    fn debugger_with(e: &mut Engine, clients: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x90);
        let data = e.mem.alloc(8 * clients.len() as u32 + 8);
        for (index, (id, client)) in clients.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *id);
            e.mem.set_u32(data + 8 * index as u32 + 4, *client);
        }
        e.mem.set_u32(this + 0x10, data);
        e.mem.set_u32(this + 0x14, clients.len() as u32);
        e.mem
            .set_u32(this + 0x18, 0x4000_0000 + clients.len() as u32 + 4);
        this
    }

    #[test]
    fn serve_creates_the_server_listens_and_reports_the_port() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let handler = install_world(&mut e, &events);
        let server = e.mem.alloc(8);
        e.mem.set_u32(server, SERVER_TABLE);
        let mut table = vec![0u32; 9];
        table[8] = DOUBLES + 0xb0;
        e.put_vtable(SERVER_TABLE, &table);
        tag_calls(&mut e, &events, DOUBLES + 0xb0, "listen", 0);
        e.map(SERVER_FACTORY, 4);
        e.mem.set_u32(SERVER_FACTORY, DOUBLES + 0xc0);
        tag_calls(&mut e, &events, DOUBLES + 0xc0, "factory", server);
        let this = e.mem.alloc(0x90);
        e.call(0x00cd_0d00, &args![this, 25001u32]);
        assert_eq!(e.mem.u32(this + 8), server);
        assert_eq!(
            tags(&events),
            vec![
                "begin",
                "factory",
                "listen",
                "construct",
                "text",
                "int",
                "text",
                "report",
                "destruct",
                "end"
            ]
        );
        assert_eq!(
            calls(&events, "begin"),
            vec![vec![handler, TIMER_SERVING, NAME_SERVING]]
        );
        assert_eq!(calls(&events, "listen"), vec![vec![server, 25001]]);
        let construct = calls(&events, "construct")[0].clone();
        let stream = construct[0];
        let buffer = stream + 0xc;
        assert_eq!(construct, vec![stream, buffer, 0x200]);
        assert_eq!(
            calls(&events, "text"),
            vec![
                vec![stream, TEXT_SERVER_CREATED_PREFIX],
                vec![stream + 2, TEXT_SERVER_CREATED_SUFFIX]
            ]
        );
        assert_eq!(calls(&events, "int"), vec![vec![stream + 1, 25001]]);
        assert_eq!(
            calls(&events, "report"),
            vec![vec![handler, 0, 0xffff_ffff, buffer, SOURCE_FILE, 0x67]]
        );
        assert_eq!(calls(&events, "destruct"), vec![vec![stream]]);
        // A second call finds the server and says so, without the factory.
        events.borrow_mut().clear();
        e.call(0x00cd_0d00, &args![this, 25001u32]);
        assert_eq!(
            tags(&events),
            vec!["begin", "construct", "text", "report", "destruct", "end"]
        );
        assert_eq!(calls(&events, "text")[0][1], TEXT_SERVER_EXISTS);
        assert_eq!(calls(&events, "report")[0][5], 0x70);
    }

    #[test]
    fn serve_reports_a_server_that_could_not_be_created() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_world(&mut e, &events);
        e.map(SERVER_FACTORY, 4);
        e.mem.set_u32(SERVER_FACTORY, DOUBLES + 0xc0);
        tag_calls(&mut e, &events, DOUBLES + 0xc0, "factory", 0);
        let this = e.mem.alloc(0x90);
        e.call(0x00cd_0d00, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            tags(&events),
            vec![
                "begin",
                "factory",
                "construct",
                "text",
                "report",
                "destruct",
                "end"
            ]
        );
        assert_eq!(calls(&events, "text")[0][1], TEXT_SERVER_CREATE_FAILED);
        assert_eq!(calls(&events, "report")[0][5], 0x6b);
    }

    #[test]
    fn write_step_writes_to_the_archive_of_a_live_client_only() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        let live = install_client(&mut e, &lives, &[true]);
        let dead = install_client(&mut e, &lives, &[false]);
        let this = debugger_with(&mut e, &[(1, live), (2, dead)]);
        // A signalling NaN step is quieted by the FLD / FSTP pair.
        e.call(0x00cd_0e00, &args![this, 0u32, f32::from_bits(0x7f80_0001)]);
        let archive = live + 0x40;
        assert_eq!(
            tags(&events),
            vec![
                "alive",
                "write32u",
                "write_flag",
                "write_float",
                "stream_writer",
                "flush"
            ]
        );
        assert_eq!(calls(&events, "write32u"), vec![vec![archive, 5]]);
        assert_eq!(calls(&events, "write_flag"), vec![vec![archive, 0]]);
        assert_eq!(
            calls(&events, "write_float"),
            vec![vec![archive, 0x7fc0_0001]]
        );
        assert_eq!(calls(&events, "stream_writer"), vec![vec![archive]]);
        events.borrow_mut().clear();
        e.call(0x00cd_0e00, &args![this, 1u32, 1.0f32]);
        assert_eq!(tags(&events), vec!["alive"]);
    }

    #[test]
    fn delete_client_releases_and_moves_the_last_entry_in() {
        let mut e = Engine::new();
        let removed = log_calls_to(&mut e, REMOVE_REFERENCE);
        let this = debugger_with(&mut e, &[(0x11, 0x21), (0, 0x22), (0x13, 0)]);
        e.call(0x00cd_0e60, &args![this, 0u32]);
        // The client (second word) then the first word are released; not cleared.
        assert_eq!(*removed.borrow(), vec![vec![0x21], vec![0x11]]);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(data), 0x13);
        assert_eq!(e.mem.u32(data + 4), 0);
        // Deleting the last entry moves nothing.
        removed.borrow_mut().clear();
        e.call(0x00cd_0e60, &args![this, 1u32]);
        assert_eq!(*removed.borrow(), vec![vec![0x22]]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(e.mem.u32(data), 0x13);
    }

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double that logs its argument words.
    fn log_calls_to(e: &mut Engine, addr: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// The timer doubles: the tick counter counts up by 100 a call from 1000; the ticks per
    /// second are 10; `divide64` answers 0.25 (logging its words).
    fn install_clock(e: &mut Engine, events: &Events) {
        let ticks = Rc::new(RefCell::new(1000u64));
        e.register_double(TICK_COUNTER, move |_, _| {
            let mut now = ticks.borrow_mut();
            let value = *now;
            *now += 100;
            Ret {
                eax: value as u32,
                edx: (value >> 32) as u32,
                ..Ret::default()
            }
        });
        e.register(TICKS_PER_SECOND, |_, _| Ret {
            eax: 10,
            ..Ret::default()
        });
        let kept = events.clone();
        e.register_double(DIVIDE64, move |_, a| {
            kept.borrow_mut().push(("divide", a.to_vec()));
            Ret {
                st0: 0.25,
                ..Ret::default()
            }
        });
        e.map(THOUSAND, 8);
        e.mem.set_f64(THOUSAND, 1000.0);
        e.map(DEFAULT_STEP, 4);
        e.mem.set_f32(DEFAULT_STEP, 16.0);
        e.map(STEP_EPSILON, 4);
        e.mem.set_f32(STEP_EPSILON, 1.0e-6);
    }

    #[test]
    fn step_tells_live_clients_the_step_and_removes_the_dead_ones() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        let handler = install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        install_clock(&mut e, &events);
        let removed = log_calls_to(&mut e, REMOVE_REFERENCE);
        let steady = install_client(&mut e, &lives, &[true]);
        // The second client is alive when asked twice (the step and `writeStep`) and dead the third time.
        let dying = install_client(&mut e, &lives, &[true, true, false]);
        let this = debugger_with(&mut e, &[(0x31, steady), (0x32, dying)]);
        // The vtable of the debugger: slot 0x10 is pollForNewClients (logged).
        e.mem.set_u32(this, 0x0068_0000);
        let mut table = vec![0u32; 5];
        table[4] = DOUBLES + 0xd0;
        e.put_vtable(0x0068_0000, &table);
        tag_calls(&mut e, &events, DOUBLES + 0xd0, "poll", 0);
        e.call(0x00cd_0ec0, &args![this, 0.5f32]);
        assert_eq!(e.mem.u8(this + 0x5c), 1);
        let step = 0.5f32.to_bits();
        // The last client first: alive, told, written, then found dead and deleted.
        assert_eq!(
            tags(&events),
            vec![
                "poll",
                "alive",
                "send_step",
                "alive",
                "write32u",
                "write_flag",
                "write_float",
                "stream_writer",
                "flush",
                "alive",
                "begin",
                "construct",
                "text",
                "report",
                "destruct",
                "end",
                "alive",
                "send_step",
                "alive",
                "write32u",
                "write_flag",
                "write_float",
                "stream_writer",
                "flush",
                "alive"
            ]
        );
        assert_eq!(
            calls(&events, "send_step"),
            vec![vec![dying + 0xc, step], vec![steady + 0xc, step]]
        );
        assert_eq!(
            calls(&events, "begin"),
            vec![vec![handler, TIMER_CLIENT_DIES, NAME_CLIENT_DIES]]
        );
        assert_eq!(calls(&events, "text")[0][1], TEXT_CLIENT_DIED);
        assert_eq!(calls(&events, "report")[0][5], 0x120);
        // The dead client's references were released and its entry removed.
        assert_eq!(*removed.borrow(), vec![vec![dying], vec![0x32]]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        // A step that is too small gives the default 16.0 (with the timers off).
        events.borrow_mut().clear();
        e.mem.set_u8(this + 0x5c, 0);
        e.call(0x00cd_0ec0, &args![this, 0.0f32]);
        assert_eq!(calls(&events, "send_step")[0][1], 16.0f32.to_bits());
        // A NaN is not above the epsilon either.
        events.borrow_mut().clear();
        e.mem.set_u8(this + 0x5c, 0);
        e.call(0x00cd_0ec0, &args![this, f32::NAN]);
        assert_eq!(calls(&events, "send_step")[0][1], 16.0f32.to_bits());
    }

    #[test]
    fn step_with_the_timers_on_uses_the_frame_time_and_restarts_the_counters() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        install_clock(&mut e, &events);
        let client = install_client(&mut e, &lives, &[true]);
        let this = debugger_with(&mut e, &[(1, client)]);
        e.mem.set_u32(this, 0x0068_0000);
        let mut table = vec![0u32; 5];
        table[4] = DOUBLES + 0xd0;
        e.put_vtable(0x0068_0000, &table);
        tag_calls(&mut e, &events, DOUBLES + 0xd0, "poll", 0);
        e.mem.set_u8(this + 0x5c, 1);
        e.mem.set_u8(this + 0x80, 1);
        e.mem.set_u64(this + 0x60, 900);
        e.mem.set_u64(this + 0x68, 50);
        e.mem.set_u64(this + 0x70, 800);
        e.mem.set_u64(this + 0x78, 7);
        e.mem.set_u32(this + 0x84, 4);
        e.call(0x00cd_0ec0, &args![this, 0.0f32]);
        // The running timer adds the time since +0x60 (1000 - 900) to the total 50 before
        // dividing: 150 ticks over 10 per second; 0.25 * 1000 is the step.
        assert_eq!(calls(&events, "divide"), vec![vec![150, 0, 10, 0]]);
        assert_eq!(calls(&events, "send_step")[0][1], 250.0f32.to_bits());
        // The next tick (1100) is added to both totals; then everything restarts at 1200.
        assert_eq!(e.mem.u8(this + 0x80), 1);
        assert_eq!(e.mem.u64(this + 0x60), 1200);
        assert_eq!(e.mem.u64(this + 0x70), 1200);
        assert_eq!(e.mem.u64(this + 0x68), 0);
        assert_eq!(e.mem.u64(this + 0x78), 0);
        assert_eq!(e.mem.u32(this + 0x84), 0);
    }

    #[test]
    fn the_key_removal_calls_the_listeners_only_when_it_found_the_key() {
        let mut e = Engine::new();
        let listener_a = log_calls_to(&mut e, 0x0069_0000);
        let listener_b = log_calls_to(&mut e, 0x0069_0010);
        let this = e.mem.alloc(0x50);
        let entries = e.mem.alloc(48);
        for (index, key) in [0x5u32, 0x6, 0x7].iter().enumerate() {
            e.mem.set_u32(entries + 12 * index as u32, *key);
            e.mem.set_u32(entries + 12 * index as u32 + 4, 0x100 + *key);
            e.mem.set_u32(entries + 12 * index as u32 + 8, 0x200 + *key);
        }
        e.mem.set_u32(this + 0x28, entries);
        e.mem.set_u32(this + 0x2c, 3);
        let functions = e.mem.alloc(8);
        e.mem.set_u32(functions, 0x0069_0000);
        e.mem.set_u32(functions + 4, 0x0069_0010);
        let users = e.mem.alloc(8);
        e.mem.set_u32(users, 0xa1);
        e.mem.set_u32(users + 4, 0xa2);
        e.mem.set_u32(this + 0x34, functions);
        e.mem.set_u32(this + 0x38, 2);
        e.mem.set_u32(this + 0x40, users);
        e.mem.set_u32(this + 0x44, 2);
        e.call(0x00cd_10a0, &args![this, 0x9u32]);
        assert!(listener_a.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x2c), 3);
        e.call(0x00cd_10a0, &args![this, 0x5u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 2);
        // The last entry (key 7) filled the removed one's place.
        assert_eq!(e.mem.u32(entries), 7);
        assert_eq!(e.mem.u32(entries + 4), 0x107);
        assert_eq!(e.mem.u32(entries + 8), 0x207);
        assert_eq!(*listener_a.borrow(), vec![vec![5, 0, 0, 0, 0xa1]]);
        assert_eq!(*listener_b.borrow(), vec![vec![5, 0, 0, 0, 0xa2]]);
        // The index word of the second listener is 4 (its low byte is the bool false).
        listener_b.borrow_mut().clear();
        e.mem.set_u32(this + 0x2c, 1);
        e.call(0x00cd_10a0, &args![this, 7u32]);
        assert_eq!(listener_b.borrow()[0][2], 0);
    }

    #[test]
    fn the_listener_removal_takes_the_function_and_its_user_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        let functions = e.mem.alloc(12);
        let users = e.mem.alloc(12);
        for index in 0..3u32 {
            e.mem.set_u32(functions + 4 * index, 0x10 + index);
            e.mem.set_u32(users + 4 * index, 0x20 + index);
        }
        e.mem.set_u32(this + 0x34, functions);
        e.mem.set_u32(this + 0x38, 3);
        e.mem.set_u32(this + 0x40, users);
        e.mem.set_u32(this + 0x44, 3);
        e.call(0x00cd_1140, &args![this, 0x99u32]);
        assert_eq!(e.mem.u32(this + 0x38), 3);
        e.call(0x00cd_1140, &args![this, 0x10u32]);
        assert_eq!(e.mem.u32(this + 0x38), 2);
        assert_eq!(e.mem.u32(this + 0x44), 2);
        assert_eq!(e.mem.u32(functions), 0x12);
        assert_eq!(e.mem.u32(users), 0x22);
        // Removing the last of the two moves nothing.
        e.call(0x00cd_1140, &args![this, 0x11u32]);
        assert_eq!(e.mem.u32(this + 0x38), 1);
        assert_eq!(e.mem.u32(functions), 0x12);
    }

    #[test]
    fn shutdown_writes_a_zero_step_deletes_the_clients_and_the_server() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        let handler = install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        let removed = log_calls_to(&mut e, REMOVE_REFERENCE);
        let first = install_client(&mut e, &lives, &[true]);
        let second = install_client(&mut e, &lives, &[false]);
        let this = debugger_with(&mut e, &[(0x41, first), (0x42, second)]);
        let server = e.mem.alloc(8);
        e.mem.set_u32(server, SERVER_TABLE);
        let mut table = vec![0u32; 2];
        table[0] = DOUBLES + 0xe0;
        e.put_vtable(SERVER_TABLE, &table);
        tag_calls(&mut e, &events, DOUBLES + 0xe0, "delete_server", 0);
        e.mem.set_u32(this + 8, server);
        e.call(0x00cd_11a0, &args![this]);
        assert_eq!(
            tags(&events),
            vec![
                "begin",
                "alive",
                "construct",
                "text",
                "report",
                "destruct",
                "alive",
                "write32u",
                "write_flag",
                "write_float",
                "stream_writer",
                "flush",
                "construct",
                "text",
                "report",
                "destruct",
                "delete_server",
                "construct",
                "text",
                "report",
                "destruct",
                "end"
            ]
        );
        assert_eq!(
            calls(&events, "begin"),
            vec![vec![handler, TIMER_VISUAL_DEBUGGER, NAME_SHUTTING_DOWN]]
        );
        assert_eq!(calls(&events, "write_float")[0][1], 0);
        assert_eq!(calls(&events, "delete_server"), vec![vec![server, 1]]);
        let lines: Vec<u32> = calls(&events, "report").iter().map(|c| c[5]).collect();
        assert_eq!(lines, vec![0x51, 0x51, 0x58]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(removed.borrow().len(), 4);
    }

    #[test]
    fn the_sweep_deletes_the_clients_whose_first_word_is_null() {
        let mut e = Engine::new();
        let removed = log_calls_to(&mut e, REMOVE_REFERENCE);
        let this = debugger_with(&mut e, &[(0, 0x51), (0x12, 0x52), (0, 0x53)]);
        e.call(0x00cd_12b0, &args![this]);
        assert_eq!(*removed.borrow(), vec![vec![0x53], vec![0x51]]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(data), 0x12);
    }

    #[test]
    fn a_name_is_added_to_the_string_array_and_assigned() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_world(&mut e, &events);
        tag_calls(&mut e, &events, STRING_PTR_CONSTRUCT, "construct_string", 0);
        tag_calls(
            &mut e,
            &events,
            STRING_PTR_ASSIGN_TEXT,
            "assign_text",
            0x7777,
        );
        tag_calls(&mut e, &events, ARRAY_RESERVE_MORE, "grow", 0);
        let this = e.mem.alloc(0x90);
        let data = e.mem.alloc(8);
        e.mem.set_u32(this + 0x50, data);
        e.mem.set_u32(this + 0x54, 1);
        e.mem.set_u32(this + 0x58, 0x4000_0002);
        let result = e.call(0x00cd_12e0, &args![this, 0x5555u32]).u32();
        assert_eq!(result, 0x7777);
        assert_eq!(e.mem.u32(this + 0x54), 2);
        assert_eq!(tags(&events), vec!["construct_string", "assign_text"]);
        assert_eq!(calls(&events, "assign_text"), vec![vec![data + 4, 0x5555]]);
        // A full array grows first.
        events.borrow_mut().clear();
        let result = e.call(0x00cd_12e0, &args![this, 0x6666u32]).u32();
        assert_eq!(result, 0x7777);
        assert_eq!(tags(&events)[0], "grow");
        assert_eq!(calls(&events, "grow")[0][1..], [this + 0x50, 4]);
    }

    #[test]
    fn a_registration_is_stored_and_announced_to_the_listeners() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_world(&mut e, &events);
        let grown = log_calls_to(&mut e, ARRAY_RESERVE_MORE);
        let listener = log_calls_to(&mut e, 0x0069_0000);
        let this = e.mem.alloc(0x90);
        let entries = e.mem.alloc(64);
        e.mem.set_u32(this + 0x28, entries);
        e.mem.set_u32(this + 0x2c, 1);
        e.mem.set_u32(this + 0x30, 0x4000_0004);
        let functions = e.mem.alloc(4);
        e.mem.set_u32(functions, 0x0069_0000);
        let users = e.mem.alloc(4);
        e.mem.set_u32(users, 0xb1);
        e.mem.set_u32(this + 0x34, functions);
        e.mem.set_u32(this + 0x38, 1);
        e.mem.set_u32(this + 0x40, users);
        e.mem.set_u32(this + 0x44, 1);
        e.call(
            0x00cd_1330,
            &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert!(grown.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x2c), 2);
        assert_eq!(e.mem.u32(entries + 12), 0x11);
        assert_eq!(e.mem.u32(entries + 16), 0x22);
        assert_eq!(e.mem.u32(entries + 20), 0x44);
        // The third word is never stored; the listener's third word has the true bool.
        assert_eq!(*listener.borrow(), vec![vec![0x11, 0x22, 1, 0x44, 0xb1]]);
        // A full array grows first (element size 12).
        e.mem.set_u32(this + 0x30, 0x4000_0002);
        e.call(0x00cd_1330, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(grown.borrow().len(), 1);
        assert_eq!(grown.borrow()[0][1..], [this + 0x28, 12]);
    }

    #[test]
    fn a_listener_is_added_with_its_user_word() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_world(&mut e, &events);
        let grown = log_calls_to(&mut e, ARRAY_RESERVE_MORE);
        let this = e.mem.alloc(0x90);
        let functions = e.mem.alloc(16);
        let users = e.mem.alloc(16);
        e.mem.set_u32(this + 0x34, functions);
        e.mem.set_u32(this + 0x3c, 4);
        e.mem.set_u32(this + 0x40, users);
        e.mem.set_u32(this + 0x48, 4);
        e.call(0x00cd_13c0, &args![this, 0x1234u32, 0x5678u32]);
        assert_eq!(e.mem.u32(functions), 0x1234);
        assert_eq!(e.mem.u32(users), 0x5678);
        assert_eq!(e.mem.u32(this + 0x38), 1);
        assert_eq!(e.mem.u32(this + 0x44), 1);
        assert!(grown.borrow().is_empty());
        // The first array being full grows it, the second one stays.
        e.mem.set_u32(this + 0x3c, 1);
        e.call(0x00cd_13c0, &args![this, 1u32, 2u32]);
        assert_eq!(grown.borrow().len(), 1);
        assert_eq!(grown.borrow()[0][1..], [this + 0x34, 4]);
    }

    const FINDER_TABLE: u32 = 0x006a_0000;

    /// `_reserve` that gives the array at `a[1]` a fresh block of `a[2]` entries of `a[3]` bytes.
    fn install_reserve(e: &mut Engine, events: &Events) {
        let kept = events.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push(("reserve", a.to_vec()));
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    /// The client the constructor double makes: its +8 object (finder at slot 8, selector at
    /// slot 0x10), its +0xc object (alive for ever) and its archive (+0x18).
    fn install_client_factory(e: &mut Engine, events: &Events, lives: &Lives) -> Rc<RefCell<u32>> {
        let mut table = vec![0u32; 5];
        table[2] = DOUBLES + 0x70;
        table[4] = DOUBLES + 0xa0;
        e.put_vtable(FINDER_TABLE, &table);
        let made = Rc::new(RefCell::new(0u32));
        let kept_made = made.clone();
        let lives = lives.clone();
        let kept = events.clone();
        e.register_double(CLIENT_CONSTRUCT, move |e, a| {
            kept.borrow_mut().push(("construct_client", a.to_vec()));
            let client = a[0];
            e.mem.set_u32(client + 8, FINDER_TABLE);
            e.mem.set_u32(client + 0xc, EMBEDDED_TABLE);
            e.mem.set_u32(client + 0x18, client + 0x40);
            lives
                .borrow_mut()
                .insert(client + 0xc, VecDeque::from([true]));
            *kept_made.borrow_mut() = client;
            Ret {
                eax: client,
                ..Ret::default()
            }
        });
        tag_calls(e, events, CLIENT_START, "start", 0);
        tag_calls(e, events, SEND_VERSION_INFORMATION, "version", 0);
        made
    }

    #[test]
    fn creating_a_client_makes_its_entry_and_selects_the_known_processes() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        install_reserve(&mut e, &events);
        let made = install_client_factory(&mut e, &events, &lives);
        let this = e.mem.alloc(0x90);
        // Two names (words, low bit a flag): 0x21 (index 0x20) and 0x300 (not known).
        let names = e.mem.alloc(8);
        e.mem.set_u32(names, 0x21);
        e.mem.set_u32(names + 4, 0x300);
        e.mem.set_u32(this + 0x50, names);
        e.mem.set_u32(this + 0x54, 2);
        e.call(0x00cd_1440, &args![this, 0x91u32, 0x92u32, 0x93u32]);
        let client = *made.borrow();
        assert_eq!(
            tags(&events),
            vec![
                "version",
                "reserve",
                "alloc",
                "construct_client",
                "start",
                "select",
                "alive",
                "write32u",
                "write_flag",
                "write_float",
                "stream_writer",
                "flush"
            ]
        );
        assert_eq!(calls(&events, "version"), vec![vec![0x93]]);
        assert_eq!(
            calls(&events, "reserve"),
            vec![vec![ALLOCATOR_OBJECT, this + 0x10, 1, 8]]
        );
        assert_eq!(calls(&events, "alloc"), vec![vec![ALLOCATOR_OBJECT, 0x50]]);
        assert_eq!(
            calls(&events, "construct_client"),
            vec![vec![client, this + 0x1c, 0x92, 0x93]]
        );
        assert_eq!(e.mem.u32(this + 0x14), 1);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(data), 0x91);
        assert_eq!(e.mem.u32(data + 4), client);
        assert_eq!(e.mem.u16(client + 4), 0x50);
        assert_eq!(calls(&events, "start"), vec![vec![client]]);
        // Name 0x21 (flag cleared: 0x20) is known and selected through the +8 object.
        assert_eq!(calls(&events, "select"), vec![vec![client + 8, 0x20]]);
        // A null version target sends nothing; the array has room now.
        events.borrow_mut().clear();
        e.mem.set_u32(this + 0x18, 0x4000_0004);
        e.call(0x00cd_1440, &args![this, 0x94u32, 0x95u32, 0u32]);
        assert!(!tags(&events).contains(&"version"));
        assert!(!tags(&events).contains(&"reserve"));
        assert_eq!(e.mem.u32(this + 0x14), 2);
    }

    #[test]
    fn polling_creates_a_client_for_a_new_connection_and_keeps_the_timers() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let lives: Lives = Rc::new(RefCell::new(HashMap::new()));
        let handler = install_world(&mut e, &events);
        install_embedded_table(&mut e, &events, &lives);
        install_reserve(&mut e, &events);
        install_client_factory(&mut e, &events, &lives);
        install_clock(&mut e, &events);
        let this = e.mem.alloc(0x90);
        // Polling switched off: nothing at all.
        e.mem.set_u8(this + 0xc, 1);
        e.call(0x00cd_1520, &args![this]);
        assert!(tags(&events).is_empty());
        e.mem.set_u8(this + 0xc, 0);
        // A server whose slot 0x28 hands out a connection (its stream at +8, a version target at +0x14).
        let connection = e.mem.alloc(0x30);
        let server = e.mem.alloc(8);
        e.mem.set_u32(server, SERVER_TABLE);
        let mut table = vec![0u32; 11];
        table[10] = DOUBLES + 0xf0;
        e.put_vtable(SERVER_TABLE, &table);
        tag_calls(&mut e, &events, DOUBLES + 0xf0, "accept", connection);
        e.mem.set_u32(this + 8, server);
        e.mem.set_u8(this + 0x5c, 1);
        e.mem.set_u8(this + 0x80, 1);
        e.mem.set_u64(this + 0x60, 900);
        e.mem.set_u64(this + 0x68, 50);
        e.mem.set_u64(this + 0x70, 800);
        e.mem.set_u64(this + 0x78, 7);
        e.mem.set_u32(this + 0x84, 4);
        e.call(0x00cd_1520, &args![this]);
        // The running time (tick 1000) went into the totals; the next tick (1100) restarted them.
        assert_eq!(e.mem.u64(this + 0x68), 150);
        assert_eq!(e.mem.u64(this + 0x78), 207);
        assert_eq!(e.mem.u32(this + 0x84), 5);
        assert_eq!(e.mem.u64(this + 0x60), 1100);
        assert_eq!(e.mem.u64(this + 0x70), 1100);
        assert_eq!(e.mem.u8(this + 0x80), 1);
        let order = tags(&events);
        assert_eq!(
            &order[..6],
            &[
                "accept",
                "construct",
                "text",
                "report",
                "destruct",
                "version"
            ]
        );
        assert_eq!(calls(&events, "text")[0][1], TEXT_NEW_CLIENT);
        let report = calls(&events, "report")[0].clone();
        assert_eq!(report[0], handler);
        assert_eq!(report[5], 0xe8);
        // createClient(connection, connection + 8, connection + 0x14).
        assert_eq!(calls(&events, "version"), vec![vec![connection + 0x14]]);
        assert_eq!(
            calls(&events, "construct_client")[0][1..],
            [this + 0x1c, connection + 8, connection + 0x14]
        );
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(data), connection);
        // No new connection: no message. Timers off: the totals stay.
        events.borrow_mut().clear();
        tag_calls(&mut e, &events, DOUBLES + 0xf0, "accept", 0);
        e.mem.set_u8(this + 0x5c, 0);
        e.call(0x00cd_1520, &args![this]);
        assert_eq!(tags(&events), vec!["accept"]);
        assert_eq!(e.mem.u64(this + 0x60), 1100);
    }

    #[test]
    fn the_constructor_sets_up_the_arrays_and_registers_the_processes() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        let handler = install_world(&mut e, &events);
        tag_calls(&mut e, &events, REGISTER_COMMON_PROCESSES, "register", 0);
        tag_calls(&mut e, &events, STRING_PTR_CONSTRUCT, "construct_string", 0);
        tag_calls(&mut e, &events, STRING_PTR_ASSIGN_TEXT, "assign_text", 0);
        tag_calls(&mut e, &events, ADD_REFERENCE, "add_reference", 0);
        let kept = events.clone();
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            kept.borrow_mut().push(("grow", a.to_vec()));
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        // Two processes, each with a slot 8 that is told the debugger.
        let mut table = vec![0u32; 3];
        table[2] = DOUBLES + 0x100;
        e.put_vtable(PROCESS_TABLE, &table);
        tag_calls(&mut e, &events, DOUBLES + 0x100, "attach", 0);
        let process_a = e.mem.alloc(8);
        let process_b = e.mem.alloc(8);
        e.mem.set_u32(process_a, PROCESS_TABLE);
        e.mem.set_u32(process_b, PROCESS_TABLE);
        let array = e.mem.alloc(0x10);
        let words = e.mem.alloc(8);
        e.mem.set_u32(words, process_a);
        e.mem.set_u32(words + 4, process_b);
        e.mem.set_u32(array, words);
        e.mem.set_u32(array + 4, 2);
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xee; 0x90]);
        assert_eq!(
            e.call(0x00cd_1780, &args![this, array, 0x777u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VISUAL_DEBUGGER_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u8(this + 0xc), 0);
        for array_offset in [0x10u32, 0x28, 0x34, 0x40] {
            assert_eq!(e.mem.u32(this + array_offset), 0);
            assert_eq!(e.mem.u32(this + array_offset + 4), 0);
            assert_eq!(e.mem.u32(this + array_offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x4c), 0x777);
        assert_eq!(e.mem.u32(this + 0x88), FRAME_TIMER_LABEL);
        assert_eq!(e.mem.u64(this + 0x60), 0);
        assert_eq!(e.mem.u8(this + 0x5c), 0);
        assert_eq!(e.mem.u8(this + 0x80), 0);
        // The process words were copied to a block of 8 bytes (capacity word 2).
        let copied = e.mem.u32(this + 0x1c);
        assert_eq!(e.mem.u32(this + 0x20), 2);
        assert_eq!(e.mem.u32(this + 0x24), 2);
        assert_eq!(e.mem.u32(copied), process_a);
        assert_eq!(e.mem.u32(copied + 4), process_b);
        assert_eq!(
            tags(&events),
            vec![
                "buf_alloc",
                "attach",
                "attach",
                "begin",
                "construct",
                "text",
                "report",
                "destruct",
                "register",
                "grow",
                "construct_string",
                "assign_text",
                "construct_string",
                "assign_text",
                "construct_string",
                "assign_text",
                "add_reference",
                "end"
            ]
        );
        assert_eq!(
            calls(&events, "attach"),
            vec![vec![process_a, this], vec![process_b, this]]
        );
        assert_eq!(
            calls(&events, "begin"),
            vec![vec![handler, TIMER_VISUAL_DEBUGGER, NAME_VISUAL_DEBUGGER]]
        );
        assert_eq!(calls(&events, "text")[0][1], TEXT_INSTANCE_CREATED);
        assert_eq!(calls(&events, "report")[0][5], 0x28);
        let texts: Vec<u32> = calls(&events, "assign_text").iter().map(|c| c[1]).collect();
        assert_eq!(texts, PROCESS_NAMES.to_vec());
        assert_eq!(calls(&events, "add_reference"), vec![vec![0x777]]);
        // With no processes and no reference nothing is allocated or referenced.
        events.borrow_mut().clear();
        e.mem.set_u32(array + 4, 0);
        e.call(0x00cd_1780, &args![this, array, 0u32]);
        assert!(!tags(&events).contains(&"buf_alloc"));
        assert!(!tags(&events).contains(&"add_reference"));
    }

    #[test]
    fn the_destructor_shuts_down_and_gives_every_array_back() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_world(&mut e, &events);
        tag_calls(&mut e, &events, STRING_PTR_DESTRUCT, "destroy_string", 0);
        tag_calls(&mut e, &events, REMOVE_REFERENCE, "remove", 0);
        let mut table = vec![0u32; 3];
        table[2] = DOUBLES + 0x100;
        e.put_vtable(PROCESS_TABLE, &table);
        tag_calls(&mut e, &events, DOUBLES + 0x100, "detach", 0);
        let process = e.mem.alloc(8);
        e.mem.set_u32(process, PROCESS_TABLE);
        let words = e.mem.alloc(4);
        e.mem.set_u32(words, process);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x1c, words);
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_u32(this + 0x24, 6);
        e.mem.set_u32(this + 0x4c, 0x777);
        // The other arrays: (data field, data, size field, capacity field, capacity word).
        e.mem.set_u32(this + 0x50, 0x2000);
        e.mem.set_u32(this + 0x54, 2);
        e.mem.set_u32(this + 0x58, 4);
        e.mem.set_u32(this + 0x40, 0x3000);
        e.mem.set_u32(this + 0x44, 1);
        e.mem.set_u32(this + 0x48, 3);
        e.mem.set_u32(this + 0x34, 0x4000);
        e.mem.set_u32(this + 0x38, 1);
        e.mem.set_u32(this + 0x3c, 2);
        e.mem.set_u32(this + 0x28, 0x5000);
        e.mem.set_u32(this + 0x2c, 1);
        e.mem.set_u32(this + 0x30, 5);
        e.mem.set_u32(this + 0x10, 0x6000);
        e.mem.set_u32(this + 0x14, 0);
        e.mem.set_u32(this + 0x18, 7);
        e.call(0x00cd_1970, &args![this]);
        assert_eq!(
            tags(&events),
            vec![
                "begin",
                "end",
                "detach",
                "destroy_string",
                "destroy_string",
                "remove",
                "free",
                "free",
                "free",
                "free",
                "free",
                "free"
            ]
        );
        assert_eq!(calls(&events, "detach"), vec![vec![process, 0]]);
        assert_eq!(
            calls(&events, "destroy_string"),
            vec![vec![0x2000], vec![0x2004]]
        );
        assert_eq!(calls(&events, "remove"), vec![vec![0x777]]);
        let frees: Vec<(u32, u32)> = calls(&events, "free")
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        assert_eq!(
            frees,
            vec![
                (0x2000, 16),
                (0x3000, 12),
                (0x4000, 8),
                (0x5000, 60),
                (words, 24),
                (0x6000, 56)
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        for (data, size, capacity) in [
            (0x50, 0x54, 0x58),
            (0x40, 0x44, 0x48),
            (0x34, 0x38, 0x3c),
            (0x28, 0x2c, 0x30),
            (0x1c, 0x20, 0x24),
            (0x10, 0x14, 0x18),
        ] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
    }
}

//! `Havok SDK/hkserverprocesshandler.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkServerProcessHandler` (RTTI-confirmed: its vtables `010d1598`, `010d1578` and `010d1558`
//! carry that class name) derives from `hkProcessHandler` (vtable `010d1528`, at `+8`) and
//! `hkProcess` (vtable `010d1508`, at `+0xc`) and holds an `hkCommandRouter` (vtable `010d1548`)
//! at `+0x24`. The fields the functions here use, by offset from the whole object:
//! `+0x14` and `+0x18` the stream objects, `+0x1c` the object made from the output stream (with
//! `+8` added), `+0x20` the process-handler part, `+0x24` the command router (its string map at
//! `+0x2c`), `+0x38` / `+0x3c` / `+0x40` the process array (data, size, capacity and flags) and
//! `+0x44` / `+0x48` / `+0x4c` the second array.
//!
//! Several functions take a `this` that already points into the middle of the object (the
//! `hkProcessHandler` part, `+8`, or the `hkProcess` part, `+0xc`); each doc comment says which
//! (the vtable slots tell: `010d1578` slot 1 is `registerProcess`, slot 4 `00d60d10`, slot 5
//! `00d60e80`, slot 6 `00d60e20`; `010d1558` slot 3 is `consumeCommand`, slot 4 `isOk`, slot 6
//! `00d60dd0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the handler's blocks use.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The registry of process factories (written at run time): an `hkArray` at `+0xc` (data) and
/// `+0x10` (size) of 12-byte entries (name string pointer, factory function, ...), guarded by a
/// critical section at `+0x18`.
const PROCESS_REGISTRY: u32 = 0x0126_9cd0;

/// The vtables of the class chain (see the module comment) and of `hkBaseObject`.
const SERVER_HANDLER_VTABLE: u32 = 0x010d_1598;
const SERVER_HANDLER_PROCESS_HANDLER_VTABLE: u32 = 0x010d_1578;
const SERVER_HANDLER_PROCESS_VTABLE: u32 = 0x010d_1558;
const COMMAND_ROUTER_VTABLE: u32 = 0x010d_1548;
const PROCESS_HANDLER_VTABLE: u32 = 0x010d_1528;
const PROCESS_VTABLE: u32 = 0x010d_1508;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The address of the two command bytes (`0xc2`, `0xc3`) `consumeCommand` handles.
const CONSUMED_COMMANDS: u32 = 0x011b_5d38;

/// `hkOArchive::write32` (`thiscall`, one word), `write16u`, `writeRaw` (pointer, length) and
/// the byte writer (`00d605d0`, no map name; writes the byte through the stream's slot `0x10`).
const OARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const OARCHIVE_WRITE_16: u32 = 0x00d6_0920;
const OARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const OARCHIVE_WRITE_BYTE: u32 = 0x00d6_05d0;
/// `hkOArchive::isOk` (`thiscall`: the `bool` result pointer; answers it).
const OARCHIVE_IS_OK: u32 = 0x00d6_07e0;
/// `hkIArchive::readArrayGeneric` (`thiscall`: destination, element size, count) and `isOk`.
const IARCHIVE_READ_ARRAY_GENERIC: u32 = 0x00dd_8fe0;
const IARCHIVE_IS_OK: u32 = 0x00dd_90e0;
/// `hkString::strLen` (cdecl).
const STRING_LEN: u32 = 0x00cb_10d0;
/// The memory manager's free (`00401030`, cdecl, the block).
const FREE: u32 = 0x0040_1030;
/// `hkMapBase<uint,uint>::clearAndDeallocate` (`thiscall`: the allocator), the map's own
/// destructor-like cleanup (`00a29680`, no map name, `thiscall`) and `reserve` (allocator,
/// count).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_CLEANUP: u32 = 0x00a2_9680;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkCommandRouter`: registering a process (`00dd9600`), unregistering one (`00dd95b0`) and
/// `consumeCommands` (`thiscall`: the `bool` result pointer, the input archive).
const ROUTER_REGISTER: u32 = 0x00dd_9600;
const ROUTER_UNREGISTER: u32 = 0x00dd_95b0;
const ROUTER_CONSUME_COMMANDS: u32 = 0x00dd_9660;
/// The process registry's "make a process by id" (`00dd92f0`, `thiscall`: id, the array handed to
/// the factory): inside the registry's critical section calls the entry's factory.
const REGISTRY_MAKE_PROCESS: u32 = 0x00dd_92f0;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The stream-object constructors the handler's constructor calls (`thiscall`, one word each):
/// for the input (`00ddabe0`), the display output (`00dda6e0`,
/// `hkDisplaySerializeOStream::hkDisplaySerializeOStream`) and the object made from the output
/// (`00dd9cc0`).
const INPUT_OBJECT_CONSTRUCTOR: u32 = 0x00dd_abe0;
const OUTPUT_OBJECT_CONSTRUCTOR: u32 = 0x00dd_a6e0;
const ROUTER_OBJECT_CONSTRUCTOR: u32 = 0x00dd_9cc0;
/// The command bytes `consumeCommand` reacts to.
const COMMAND_0: u8 = 0xc2;
const COMMAND_1: u8 = 0xc3;

/// The allocator of the memory router (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The size in bytes of an array block, as the code computes it from the capacity word
/// (`(capacity & 0x3fffffff) * 4`).
fn storage_bytes(flags: u32) -> u32 {
    (flags & 0x3fff_ffff).wrapping_mul(4)
}

// Translated from 00d47c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCommandRouter::_scalar_deleting_destructor_` (engine map name), `thiscall`, `RET 4`: clears
/// and deallocates the string map at `+8` (`clearAndDeallocate` with the router's allocator),
/// runs its cleanup (`00a29680`), sets the `hkBaseObject` vtable and, when bit 0 of `flags`
/// is set, frees the block through the allocator's slot `8` with the 16-bit size at `this + 4`.
/// Returns `this`.
pub fn hk_command_router_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let map = this.addr() + 8;
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_CLEANUP, &args![map]);
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The body of the two small scalar deleting destructors: stores `vtable` and, when bit 0 of
/// `flags` is set, frees the block (`00401030`).
fn small_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00d60bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcess::_scalar_deleting_destructor_` (engine map name), `thiscall`, `RET 4`: stores the
/// `hkProcess` vtable (`010d1508`) and, when bit 0 of `flags` is set, frees the block
/// (`00401030`). Returns `this`.
pub fn hk_process_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    small_deleting_destructor(e, this, flags, PROCESS_VTABLE)
}

// Translated from 00d60bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcessHandler::_scalar_deleting_destructor_` (engine map name), `thiscall`, `RET 4`: the
/// same with the `hkProcessHandler` vtable (`010d1528`). Returns `this`.
pub fn hk_process_handler_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    small_deleting_destructor(e, this, flags, PROCESS_HANDLER_VTABLE)
}

// Translated from 00d60bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. A virtual method with `RET 8`, two output pointers on the stack: stores the address `011b5d38` at
/// `commands` (the exe's data there holds the bytes `0xc2`, `0xc3`, the commands
/// `consumeCommand` handles, checked in Ghidra) and the count 2 at `count`.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00d60bf0(e: &mut Engine, _this: u32, commands: u32, count: u32) {
    e.mem.set_u32(commands, CONSUMED_COMMANDS);
    e.mem.set_u32(count, 2);
}

// Translated from 00d60c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkServerProcessHandler::consumeCommand` (engine map name), `thiscall`, `RET 4`; `this` is the
/// `hkProcess` part (`+0xc` of the whole object; slot 3 of its vtable), whose `+8` is the input
/// `hkIArchive`; `this - 4` is the `hkProcessHandler` part that is asked. For command `0xc2`
/// reads one 32-bit word from the archive (`readArrayGeneric(4, 1)`) and, if the archive is still
/// ok, calls slot `0x14` of the object at `this - 4` with it; for `0xc3` the same with slot
/// `0x18` (its `isOk` result goes to the argument slot of the call); any other command does
/// nothing.
pub fn hk_server_process_handler_consume_command(e: &mut Engine, this: u32, command: u8) {
    let slot = match command {
        COMMAND_0 => 0x14,
        COMMAND_1 => 0x18,
        _ => return,
    };
    let archive = e.mem.u32(this + 8);
    // The exe's frame (`SUB ESP,0x8`, one push) taken with the argument: the word read at +0x8
    // (00d60c2e / 00d60c68); the `isOk` byte at +0x7 for `0xc2` (00d60c72) and in the command's
    // own argument slot (+0x10, past the return address at +0xc) for `0xc3` (00d60c38).
    e.with_stack(0x14, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, u32::from(command));
        let value = frame + 8;
        let flag = if slot == 0x14 {
            frame + 7
        } else {
            frame + 0x10
        };
        e.call(
            IARCHIVE_READ_ARRAY_GENERIC,
            &args![archive, value, 4u32, 1u32],
        );
        let archive = e.mem.u32(this + 8);
        let ok_at = e.call(IARCHIVE_IS_OK, &args![archive, flag]).u32();
        if e.mem.u8(ok_at) != 0 {
            let target = this.wrapping_sub(4);
            let read = e.mem.u32(value);
            e.vcall(target, slot, &args![read]);
        }
    });
}

// Translated from 00d60ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkServerProcessHandler::registerProcess` (engine map name), `thiscall`, `RET 8`, `this` the
/// `hkProcessHandler` part (`+8` of the whole object; slot 1 of its vtable `010d1578`; the output
/// `hkOArchive` is at `this + 0x10`): writes the
/// registration record to the output archive: the length (at most `0xffff`) plus 7 as a 32-bit
/// word, the command byte `0xc0`, `id` as a 32-bit word, the length as 16 bits and the `name`
/// bytes. Answers 0.
pub fn hk_server_process_handler_register_process(
    e: &mut Engine,
    this: u32,
    name: u32,
    id: u32,
) -> u32 {
    let length = e.call(STRING_LEN, &args![name]).i32();
    let length = if length > 0xffff { 0xffff } else { length };
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_32, &args![archive, length.wrapping_add(7)]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_BYTE, &args![archive, 0xc0u32]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_32, &args![archive, id]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_16, &args![archive, length]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_RAW, &args![archive, name, length]);
    0
}

// Translated from 00d60d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 4 of the `hkProcessHandler` vtable `010d1578`). `thiscall`, `RET 4`, `this` as
/// for `registerProcess` (the output `hkOArchive` at `+0x10`): writes the `5` (32-bit word), the command byte `0xc1`, `value` (32-bit word).
/// Answers 0.
pub fn fn_00d60d10(e: &mut Engine, this: u32, value: u32) -> u32 {
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_32, &args![archive, 5u32]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_BYTE, &args![archive, 0xc1u32]);
    let archive = e.mem.u32(this + 0x10);
    e.call(OARCHIVE_WRITE_32, &args![archive, value]);
    0
}

// Translated from 00d60d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `thiscall`, `RET 4`, `this` the whole object: the index of the first process in
/// the process array (`+0x38` data, `+0x3c` size) whose virtual function in slot 4 (no arguments)
/// answers `id`; -1 when there is none.
pub fn fn_00d60d40(e: &mut Engine, this: u32, id: u32) -> i32 {
    let count = e.mem.i32(this + 0x3c);
    let mut index: i32 = 0;
    if count > 0 {
        loop {
            let data = e.mem.u32(this + 0x38);
            let process = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            if e.vcall(process, 4, &args![]).u32() == id {
                return index;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    -1
}

// Translated from 00d60d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = `this` (the whole object), no stack words, plain `RET`. For each entry of
/// the process registry's array (`01269cd0`: data at `+0xc`, size at `+0x10`, 12-byte entries)
/// calls slot 4 of the object at `this + 8` (the `hkProcessHandler` part: `registerProcess`) with
/// the entry's name string (its first word, flag bit cleared) and the entry's index. Answers 0.
pub fn fn_00d60d80(e: &mut Engine, this: u32) -> u32 {
    let registry = e.mem.u32(PROCESS_REGISTRY);
    let list = registry + 0xc;
    let target = this + 8;
    let mut index: u32 = 0;
    if e.mem.i32(list + 4) > 0 {
        let mut offset: u32 = 0;
        loop {
            let data = e.mem.u32(list);
            let name = e.mem.u32(data.wrapping_add(offset)) & 0xffff_fffe;
            e.vcall(target, 4, &args![name, index]);
            index += 1;
            offset = offset.wrapping_add(0xc);
            if (index as i32) >= e.mem.i32(list + 4) {
                break;
            }
        }
    }
    0
}

// Translated from 00d60dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 6 of the `hkProcess` vtable `010d1558`). `thiscall`, `RET 4`, `this` the
/// `hkProcess` part (`+0xc` of the whole object), one `float` `step`. Walks the process array
/// (`this + 0x2c` data, `this + 0x30` size) from the last process to the first, calling slot
/// `0x18` of each with the step (through `FLD`/`FSTP`, which quiets a signalling NaN). Then, when
/// the input object (`this + 8`) is set, `hkCommandRouter::consumeCommands` (router at
/// `this + 0x18`) is given a pointer to the argument slot (which it fills with a `bool`) and that
/// object.
pub fn fn_00d60dd0(e: &mut Engine, this: u32, step: f32) {
    let mut index = e.mem.i32(this + 0x30).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(this + 0x2c);
        let process = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(process, 0x18, &args![x87_quiet(step)]);
        index = index.wrapping_sub(1);
    }
    let archive = e.mem.u32(this + 8);
    if archive != 0 {
        // The exe's frame (two pushes) taken with the argument: the router gets `step`'s own
        // argument slot, +0xc past the return address at +0x8 (00d60e02).
        e.with_stack(0x10, |e, frame| {
            let slot = frame.addr() + 0xc;
            e.mem.set_u32(slot, step.to_bits());
            e.call(ROUTER_CONSUME_COMMANDS, &args![this + 0x18, slot, archive]);
        });
    }
}

// Translated from 00d60e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 6 of the `hkProcessHandler` vtable). `thiscall`, `RET 4`, `this` the
/// `hkProcessHandler` part (`+8` of the whole object): removes the process with slot-4 answer `id` (`00d60d40` on `this - 8`): the last
/// process of the array (`+0x30` data, `+0x34` size from `this`) takes its place, the router
/// (`this + 0x1c`) is told (`00dd95b0`) and the process is destroyed (slot 0 with flag 1). Answers
/// 0 whether or not there was such a process.
pub fn fn_00d60e20(e: &mut Engine, this: u32, id: u32) -> u32 {
    let index = fn_00d60d40(e, this.wrapping_sub(8), id);
    if index < 0 {
        return 0;
    }
    let data = e.mem.u32(this + 0x30);
    let slot = (index as u32).wrapping_mul(4);
    let process = e.mem.u32(slot.wrapping_add(data));
    let count = e.mem.u32(this + 0x34).wrapping_sub(1);
    e.mem.set_u32(this + 0x34, count);
    if count != index as u32 {
        let data = e.mem.u32(this + 0x30);
        let last = e.mem.u32(data.wrapping_add(count.wrapping_mul(4)));
        e.mem.set_u32(data.wrapping_add(slot), last);
    }
    e.call(ROUTER_UNREGISTER, &args![this + 0x1c, process]);
    if process != 0 {
        e.vcall(process, 0, &args![1u32]);
    }
    0
}

// Translated from 00d60e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 5 of the `hkProcessHandler` vtable). `thiscall`, `RET 4`, `this` the
/// `hkProcessHandler` part (`+8` of the whole object): adds the process registered under `id`. Answers 0 when there already is a process
/// with that id (`00d60d40` on `this - 8`). Otherwise asks the process registry (`01269cd0`) to
/// make one (`00dd92f0`, handing it the second array at `this + 0x3c`); answers 1 when it makes
/// none. The new process gets the fields `+8` = `[this + 0xc]`, `+0xc` = `[this + 0x10]`, `+0x10` =
/// `[this + 0x14]` and `+0x14` = `this` (0 when the whole object's address is 0), its slot `0x14`
/// is called (no arguments), it is appended to the process array (`this + 0x30`, grown by
/// `00c90940` with element size 4 when size equals capacity) and registered with the router
/// (`this + 0x1c`, `00dd9600`). Answers 0.
pub fn fn_00d60e80(e: &mut Engine, this: u32, id: u32) -> u32 {
    if fn_00d60d40(e, this.wrapping_sub(8), id) >= 0 {
        return 0;
    }
    let registry = e.mem.u32(PROCESS_REGISTRY);
    let process = e
        .call(REGISTRY_MAKE_PROCESS, &args![registry, id, this + 0x3c])
        .u32();
    if process == 0 {
        return 1;
    }
    let input = e.mem.u32(this + 0xc);
    // `-(this - 8 != 0) & this`: `this` unless the whole object's address is 0.
    let handler = if this.wrapping_sub(8) != 0 { this } else { 0 };
    e.mem.set_u32(process + 8, input);
    let output = e.mem.u32(this + 0x10);
    e.mem.set_u32(process + 0xc, output);
    e.mem.set_u32(process + 0x14, handler);
    let third = e.mem.u32(this + 0x14);
    e.mem.set_u32(process + 0x10, third);
    e.vcall(process, 0x14, &args![]);
    let array = this + 0x30;
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), process);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(ROUTER_REGISTER, &args![this + 0x1c, process]);
    0
}

/// Allocates `size` bytes through the allocator's slot 4 and stores `size` as the 16-bit word at
/// `+4` of the block, as every object the handler's constructor makes is set up. Returns the block.
fn allocate_object(e: &mut Engine, allocator: u32, size: u32) -> u32 {
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

// Translated from 00d60f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkServerProcessHandler`'s constructor (no map name; the vtables stored are RTTI-confirmed
/// `hkServerProcessHandler`), `thiscall`, `RET 0xc`. `commands` is an `hkArray` of words
/// (data, size) copied to the second array at `+0x44`; `input` and `output` are optional stream
/// objects: when not 0 each is wrapped in a new 0x10-byte object (`00ddabe0` for the input, kept at
/// `+0x14`; `00dda6e0` for the output, kept at `+0x18`); a 0x14-byte object is made from the output
/// object (`00dd9cc0`) and kept, with 8 added, at `+0x1c` (0 when that gives none), and `+0x20`
/// is `this + 8`. Stores the vtables (base classes first, then the final ones), builds the command
/// router at `+0x24` with its string map at `+0x2c` (`reserve` for 0 entries), registers the
/// `hkProcess` part (`this + 0xc`) with it (`00dd9600`) and empties the process array at `+0x38`.
/// Returns `this`.
pub fn hk_server_process_handler_constructor(
    e: &mut Engine,
    this: u32,
    commands: u32,
    input: u32,
    output: u32,
) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this + 8, PROCESS_HANDLER_VTABLE);
    e.mem.set_u32(this + 0xc, PROCESS_VTABLE);
    e.mem.set_u8(this + 0x10, 0);
    e.mem.set_u32(this, SERVER_HANDLER_VTABLE);
    e.mem
        .set_u32(this + 8, SERVER_HANDLER_PROCESS_HANDLER_VTABLE);
    e.mem.set_u32(this + 0xc, SERVER_HANDLER_PROCESS_VTABLE);
    let router = this + 0x24;
    e.mem.set_u16(router + 6, 1);
    e.mem.set_u32(router, COMMAND_ROUTER_VTABLE);
    let map = router + 8;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.mem.set_u32(this + 0x38, 0);
    e.mem.set_u32(this + 0x3c, 0);
    e.mem.set_u32(this + 0x40, 0x8000_0000);
    e.mem.set_u32(this + 0x44, 0);
    e.mem.set_u32(this + 0x48, 0);
    e.mem.set_u32(this + 0x4c, 0x8000_0000);
    e.call(ROUTER_REGISTER, &args![router, this + 0xc]);
    // The second array takes a copy of `commands` (words).
    let allocator = router_allocator(e);
    let flags = e.mem.u32(this + 0x4c);
    let capacity = (flags & 0x3fff_ffff) as i32;
    if capacity < e.mem.i32(commands + 4) {
        if (flags as i32) >= 0 {
            let data = e.mem.u32(this + 0x44);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        let bytes = e.mem.u32(commands + 4).wrapping_mul(4);
        // The exe's frame (four pushes) taken with the arguments: the byte count goes to the
        // allocator through the `commands` argument's own slot, +0x14 past the return address
        // at +0x10 (00d61000).
        let size = e.with_stack(0x20, |e, frame| {
            let frame = frame.addr();
            let granted = frame + 0x14;
            e.mem.set_u32(frame + 0x18, input);
            e.mem.set_u32(frame + 0x1c, output);
            e.mem.set_u32(granted, bytes);
            let block = e.vcall(allocator, 0xc, &args![granted]).u32();
            e.mem.set_u32(this + 0x44, block);
            e.mem.i32(granted)
        });
        // `CDQ; AND EDX,3; ADD; SAR 2`: division by 4 rounding toward zero.
        e.mem
            .set_i32(this + 0x4c, size.wrapping_add((size >> 31) & 3) >> 2);
    }
    let count = e.mem.i32(commands + 4);
    let destination = e.mem.u32(this + 0x44);
    e.mem.set_i32(this + 0x48, count);
    let source = e.mem.u32(commands);
    if count > 0 {
        for word in 0..count as u32 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(destination.wrapping_add(4 * word), value);
        }
    }
    let input_object = if input == 0 {
        0
    } else {
        let allocator = router_allocator(e);
        let block = allocate_object(e, allocator, 0x10);
        e.call(INPUT_OBJECT_CONSTRUCTOR, &args![block, input]).u32()
    };
    e.mem.set_u32(this + 0x14, input_object);
    let output_object = if output == 0 {
        0
    } else {
        let allocator = router_allocator(e);
        let block = allocate_object(e, allocator, 0x10);
        e.call(OUTPUT_OBJECT_CONSTRUCTOR, &args![block, output])
            .u32()
    };
    e.mem.set_u32(this + 0x18, output_object);
    let allocator = router_allocator(e);
    let block = allocate_object(e, allocator, 0x14);
    let made = e.mem.u32(this + 0x18);
    let made = e.call(ROUTER_OBJECT_CONSTRUCTOR, &args![block, made]).u32();
    let mut adjusted = 0;
    if made != 0 {
        adjusted = made.wrapping_add(8);
    }
    e.mem.set_u32(this + 0x20, this + 8);
    e.mem.set_u32(this + 0x1c, adjusted);
    this
}

// Translated from 00d610e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkServerProcessHandler`'s destructor (no map name; the vtables are RTTI-confirmed), `ECX` =
/// `this`, no stack words, plain `RET`. Stores the final vtables, destroys every process of the
/// process array (slot 0, flag 1; null entries skipped) and the objects at `+0x14`, `+0x18` and
/// `+0x1c` (slot 0, flag 1, when not null), empties both arrays (setting the size to 0 and, if
/// the array owns its storage, freeing it through the allocator's slot `0x10`; the capacity word
/// becomes `0x80000000`), clears and deallocates the router's string map (`+0x2c`) and
/// runs its cleanup, then stores the base-class vtables (`hkBaseObject` at `+0x24` and `+0`,
/// `hkProcess` at `+0xc`, `hkProcessHandler` at `+8`).
pub fn hk_server_process_handler_destructor(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SERVER_HANDLER_VTABLE);
    e.mem
        .set_u32(this + 8, SERVER_HANDLER_PROCESS_HANDLER_VTABLE);
    e.mem.set_u32(this + 0xc, SERVER_HANDLER_PROCESS_VTABLE);
    let count = e.mem.i32(this + 0x3c);
    if count > 0 {
        let mut index: u32 = 0;
        loop {
            let data = e.mem.u32(this + 0x38);
            let process = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            if process != 0 {
                e.vcall(process, 0, &args![1u32]);
            }
            index += 1;
            if (index as i32) >= e.mem.i32(this + 0x3c) {
                break;
            }
        }
    }
    for field in [0x14, 0x18, 0x1c] {
        let object = e.mem.u32(this + field);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
    let allocator = router_allocator(e);
    let flags = e.mem.u32(this + 0x4c);
    e.mem.set_u32(this + 0x48, 0);
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(this + 0x44);
        e.vcall(allocator, 0x10, &args![data, storage_bytes(flags)]);
    }
    e.mem.set_u32(this + 0x44, 0);
    e.mem.set_u32(this + 0x4c, 0x8000_0000);
    let allocator = router_allocator(e);
    let flags = e.mem.u32(this + 0x40);
    e.mem.set_u32(this + 0x3c, 0);
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(this + 0x38);
        e.vcall(allocator, 0x10, &args![data, storage_bytes(flags)]);
    }
    e.mem.set_u32(this + 0x40, 0x8000_0000);
    e.mem.set_u32(this + 0x38, 0);
    let map = this + 0x2c;
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_CLEANUP, &args![map]);
    e.mem.set_u32(this + 0x24, BASE_OBJECT_VTABLE);
    e.mem.set_u32(this + 0xc, PROCESS_VTABLE);
    e.mem.set_u32(this + 8, PROCESS_HANDLER_VTABLE);
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
}

// Translated from 00d611e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkProcess::isOk` (engine map name; slot 4 of the `hkProcess` vtable), `ECX` = `this` (the
/// `hkProcess` part), no stack words, plain `RET`: false when the output `hkOArchive` at `+0xc` is null, else whether its `isOk`
/// (`00d607e0`, result written to a local byte) says so.
pub fn hk_process_is_ok(e: &mut Engine, this: u32) -> bool {
    let archive = e.mem.u32(this + 0xc);
    if archive == 0 {
        return false;
    }
    // The exe's frame (`PUSH ECX`): the flag byte at +0x3 (00d611e8).
    e.with_stack(4, |e, frame| {
        let ok_at = e
            .call(OARCHIVE_IS_OK, &args![archive, frame.addr() + 3])
            .u32();
        e.mem.u8(ok_at) != 0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d47c90, hk_command_router_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00d60bb0, hk_process_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00d60bd0, hk_process_handler_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00d60bf0, fn_00d60bf0(u32, u32, u32)),
        entry!(
            0x00d60c10,
            hk_server_process_handler_consume_command(u32, u8)
        ),
        entry!(0x00d60ca0, hk_server_process_handler_register_process(u32, u32, u32) -> u32),
        entry!(0x00d60d10, fn_00d60d10(u32, u32) -> u32),
        entry!(0x00d60d40, fn_00d60d40(u32, u32) -> i32),
        entry!(0x00d60d80, fn_00d60d80(u32) -> u32),
        entry!(0x00d60dd0, fn_00d60dd0(u32, f32)),
        entry!(0x00d60e20, fn_00d60e20(u32, u32) -> u32),
        entry!(0x00d60e80, fn_00d60e80(u32, u32) -> u32),
        entry!(0x00d60f30, hk_server_process_handler_constructor(u32, u32, u32, u32) -> u32),
        entry!(0x00d610e0, hk_server_process_handler_destructor(u32)),
        entry!(0x00d611e0, hk_process_is_ok(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Every function the unit calls that is not in it, as a do-nothing double.
    const EXTERNAL: &[u32] = &[
        MEMORY_ROUTER,
        OARCHIVE_WRITE_32,
        OARCHIVE_WRITE_16,
        OARCHIVE_WRITE_RAW,
        OARCHIVE_WRITE_BYTE,
        OARCHIVE_IS_OK,
        IARCHIVE_READ_ARRAY_GENERIC,
        IARCHIVE_IS_OK,
        STRING_LEN,
        FREE,
        MAP_CLEAR_AND_DEALLOCATE,
        MAP_CLEANUP,
        MAP_RESERVE,
        ROUTER_REGISTER,
        ROUTER_UNREGISTER,
        ROUTER_CONSUME_COMMANDS,
        REGISTRY_MAKE_PROCESS,
        ARRAY_RESERVE_MORE,
        INPUT_OBJECT_CONSTRUCTOR,
        OUTPUT_OBJECT_CONSTRUCTOR,
        ROUTER_OBJECT_CONSTRUCTOR,
    ];

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        for address in EXTERNAL {
            e.register(*address, |_, _| Ret::default());
        }
        e
    }

    /// An object whose vtable holds `(slot offset, target)`.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A memory router whose allocator (at `+0x10`) has slot 4 (allocate: answers a fresh block of
    /// the requested size), slot 8 (free with size) and slot `0x10` (free), and slot `0xc`
    /// (allocate with an in/out size: grants twice the size). Every call is logged as
    /// `(slot, arguments)`.
    fn install_allocator(e: &mut Engine) -> (u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&log);
        e.register_double(0x0b00_0004, move |e, a| {
            l.borrow_mut().push((4, a[1..].to_vec()));
            answer(e.mem.alloc(a[1]))
        });
        let l = Rc::clone(&log);
        e.register_double(0x0b00_0008, move |_, a| {
            l.borrow_mut().push((8, a[1..].to_vec()));
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(0x0b00_000c, move |e, a| {
            l.borrow_mut().push((0xc, vec![e.mem.u32(a[1])]));
            let size = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], size * 2);
            answer(e.mem.alloc(size * 2))
        });
        let l = Rc::clone(&log);
        e.register_double(0x0b00_0010, move |_, a| {
            l.borrow_mut().push((0x10, a[1..].to_vec()));
            Ret::default()
        });
        let allocator = object_with_slots(
            e,
            &[
                (4, 0x0b00_0004),
                (8, 0x0b00_0008),
                (0xc, 0x0b00_000c),
                (0x10, 0x0b00_0010),
            ],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| answer(router));
        (allocator, log)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn command_router_destructor_cleans_its_map_and_frees_on_request() {
        let mut e = engine();
        let (allocator, log) = install_allocator(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_7c90, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(
            calls_to(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![this + 8, allocator]]
        );
        assert_eq!(calls_to(&e, MAP_CLEANUP), vec![vec![this + 8]]);
        assert!(log.borrow().is_empty());
        // Bit 0 set: the block goes back with its 16-bit size.
        assert_eq!(e.call(0x00d4_7c90, &args![this, 1u32]).u32(), this);
        assert_eq!(*log.borrow(), vec![(8, vec![this, 0x30])]);
    }

    #[test]
    fn small_destructors_store_their_vtables_and_free_on_request() {
        for (address, vtable) in [
            (0x00d6_0bb0, PROCESS_VTABLE),
            (0x00d6_0bd0, PROCESS_HANDLER_VTABLE),
        ] {
            let mut e = engine();
            let this = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert!(calls_to(&e, FREE).is_empty());
            assert_eq!(e.call(address, &args![this, 3u32]).u32(), this);
            assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
        }
    }

    #[test]
    fn fn_00d60bf0_gives_the_command_table_and_count() {
        let mut e = engine();
        let (table, count) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call(0x00d6_0bf0, &args![VCALL_THIS, table, count]);
        assert_eq!(e.mem.u32(table), CONSUMED_COMMANDS);
        assert_eq!(e.mem.u32(count), 2);
    }

    /// The `hkProcess` part at `+0xc` of a 0x40-byte object: its `+8` is the archive; the part
    /// before it (the `hkProcessHandler` part) has slots `0x14` and `0x18` answering 0xa5 and 0xa6.
    fn consume_setup(e: &mut Engine, ok: bool) -> (u32, u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for slot in [0x14u32, 0x18] {
            let l = Rc::clone(&log);
            e.register_double(0x0b10_0000 + slot, move |_, a| {
                l.borrow_mut().push((slot, a[1..].to_vec()));
                Ret::default()
            });
        }
        let handler = object_with_slots(e, &[(0x14, 0x0b10_0014), (0x18, 0x0b10_0018)]);
        // `this - 4` is the object asked: the object's address is `handler`, so `this = handler + 4`.
        let this = handler + 4;
        let archive = e.mem.alloc(8);
        e.mem.set_u32(this + 8, archive);
        e.register_double(IARCHIVE_READ_ARRAY_GENERIC, |e, a| {
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret::default()
        });
        e.register_double(IARCHIVE_IS_OK, move |e, a| {
            e.mem.set_u8(a[1], u8::from(ok));
            answer(a[1])
        });
        (this, archive, log)
    }

    #[test]
    fn consume_command_reads_a_word_and_dispatches_by_command() {
        let mut e = engine();
        let (this, archive, log) = consume_setup(&mut e, true);
        e.call_log = Some(vec![]);
        // The exe's 0x14-byte frame: the word at +0x8; the `isOk` byte at +0x7 for `0xc2` and
        // in the command's argument slot (+0x10) for `0xc3`.
        let frame = e.next_stack_local(0x14);
        e.call(0x00d6_0c10, &args![this, 0xc2u32]);
        e.call(0x00d6_0c10, &args![this, 0xc3u32]);
        let checks = calls_to(&e, IARCHIVE_IS_OK);
        assert_eq!((checks[0][1], checks[1][1]), (frame + 7, frame + 0x10));
        assert_eq!(calls_to(&e, IARCHIVE_READ_ARRAY_GENERIC)[0][1], frame + 8);
        assert_eq!(
            *log.borrow(),
            vec![(0x14, vec![0x1234_5678]), (0x18, vec![0x1234_5678])]
        );
        // One 4-byte element is read each time.
        let reads = calls_to(&e, IARCHIVE_READ_ARRAY_GENERIC);
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0][0], archive);
        assert_eq!(reads[0][2..], [4, 1]);
        // Another command does nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x00d6_0c10, &args![this, 0xc4u32]);
        e.call(0x00d6_0c10, &args![this, 0x42u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn consume_command_ignores_a_word_from_a_failed_archive() {
        let mut e = engine();
        let (this, _, log) = consume_setup(&mut e, false);
        e.call(0x00d6_0c10, &args![this, 0xc2u32]);
        e.call(0x00d6_0c10, &args![this, 0xc3u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn register_process_writes_the_record() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xa4c1);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"physics");
        e.register(STRING_LEN, |_, _| answer(7));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0ca0, &args![this, name, 0x99u32]).u32(), 0);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 != 0x00d6_0ca0 && c.0 != STRING_LEN)
            .cloned()
            .collect();
        assert_eq!(
            order,
            vec![
                (OARCHIVE_WRITE_32, vec![0xa4c1, 14]),
                (OARCHIVE_WRITE_BYTE, vec![0xa4c1, 0xc0]),
                (OARCHIVE_WRITE_32, vec![0xa4c1, 0x99]),
                (OARCHIVE_WRITE_16, vec![0xa4c1, 7]),
                (OARCHIVE_WRITE_RAW, vec![0xa4c1, name, 7]),
            ]
        );
        // A name longer than 0xffff is cut to 0xffff.
        e.register(STRING_LEN, |_, _| answer(0x12345));
        e.call_log = Some(vec![]);
        e.call(0x00d6_0ca0, &args![this, name, 1u32]);
        assert_eq!(calls_to(&e, OARCHIVE_WRITE_32)[0], vec![0xa4c1, 0xffff + 7]);
        assert_eq!(calls_to(&e, OARCHIVE_WRITE_16)[0], vec![0xa4c1, 0xffff]);
        assert_eq!(
            calls_to(&e, OARCHIVE_WRITE_RAW)[0],
            vec![0xa4c1, name, 0xffff]
        );
    }

    #[test]
    fn fn_00d60d10_writes_the_selection_record() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xa4c1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0d10, &args![this, 0x77u32]).u32(), 0);
        let writes: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 != 0x00d6_0d10)
            .cloned()
            .collect();
        assert_eq!(
            writes,
            vec![
                (OARCHIVE_WRITE_32, vec![0xa4c1, 5]),
                (OARCHIVE_WRITE_BYTE, vec![0xa4c1, 0xc1]),
                (OARCHIVE_WRITE_32, vec![0xa4c1, 0x77]),
            ]
        );
    }

    /// A process object whose slot 4 answers `id` (and slot `0x18` logs the step it is given),
    /// slot 0 logs destruction.
    fn process(e: &mut Engine, id: u32, log: &Log) -> u32 {
        let base = 0x0b20_0000 + 0x100 * id;
        e.register_double(base + 4, move |_, _| answer(id));
        let l = Rc::clone(log);
        e.register_double(base + 0x18, move |_, a| {
            l.borrow_mut().push((id, a[1..].to_vec()));
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(base, move |_, a| {
            l.borrow_mut().push((0x1000 + id, a[1..].to_vec()));
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(base + 0x14, move |_, _| {
            l.borrow_mut().push((0x2000 + id, vec![]));
            Ret::default()
        });
        object_with_slots(
            e,
            &[
                (0, base),
                (4, base + 4),
                (0x14, base + 0x14),
                (0x18, base + 0x18),
            ],
        )
    }

    /// The whole object with a process array (`+0x38` data, `+0x3c` size, `+0x40` capacity) of
    /// processes with ids `ids`.
    fn handler_with_processes(e: &mut Engine, ids: &[u32], log: &Log) -> u32 {
        let this = e.mem.alloc(0x60);
        let data = e.mem.alloc(4 * ids.len().max(1) as u32);
        for (i, id) in ids.iter().enumerate() {
            let p = process(e, *id, log);
            e.mem.set_u32(data + 4 * i as u32, p);
        }
        e.mem.set_u32(this + 0x38, data);
        e.mem.set_u32(this + 0x3c, ids.len() as u32);
        e.mem.set_u32(this + 0x40, 8);
        this
    }

    #[test]
    fn fn_00d60d40_finds_the_index_of_a_process_by_id() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let this = handler_with_processes(&mut e, &[10, 20, 30], &log);
        assert_eq!(e.call(0x00d6_0d40, &args![this, 10u32]).i32(), 0);
        assert_eq!(e.call(0x00d6_0d40, &args![this, 30u32]).i32(), 2);
        assert_eq!(e.call(0x00d6_0d40, &args![this, 99u32]).i32(), -1);
        let empty = handler_with_processes(&mut e, &[], &log);
        assert_eq!(e.call(0x00d6_0d40, &args![empty, 10u32]).i32(), -1);
    }

    #[test]
    fn fn_00d60d80_registers_every_known_process_name_with_its_index() {
        let mut e = engine();
        let seen: Log = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&seen);
        e.register_double(0x0b30_0004, move |_, a| {
            l.borrow_mut().push((0, a[1..].to_vec()));
            Ret::default()
        });
        let this = object_with_slots(&mut e, &[(0, 0)]);
        let handler_part = object_with_slots(&mut e, &[(4, 0x0b30_0004)]);
        // The object at `this + 8` is the handler part: copy its vtable pointer there.
        let vtable = e.mem.u32(handler_part);
        e.mem.set_u32(this + 8, vtable);
        // The registry: three 12-byte entries; the first and last names carry the flag bit.
        let registry = e.mem.alloc(0x20);
        let entries = e.mem.alloc(36);
        for (i, name) in [0xa001u32, 0xb000, 0xc001].iter().enumerate() {
            e.mem.set_u32(entries + 12 * i as u32, *name);
        }
        e.mem.set_u32(registry + 0xc, entries);
        e.mem.set_u32(registry + 0x10, 3);
        e.mem.set_u32(PROCESS_REGISTRY, registry);
        assert_eq!(e.call(0x00d6_0d80, &args![this]).u32(), 0);
        assert_eq!(
            *seen.borrow(),
            vec![
                (0, vec![0xa000, 0]),
                (0, vec![0xb000, 1]),
                (0, vec![0xc000, 2])
            ]
        );
        // No entries: nothing.
        seen.borrow_mut().clear();
        e.mem.set_u32(registry + 0x10, 0);
        e.call(0x00d6_0d80, &args![this]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn fn_00d60dd0_steps_the_processes_last_first_then_consumes_commands() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        // `this` is the `hkProcess` part, `+0xc` into the whole object.
        let whole = handler_with_processes(&mut e, &[1, 2, 3], &log);
        let this = whole + 0xc;
        let archive = e.mem.alloc(8);
        e.mem.set_u32(this + 8, archive);
        let step = 0.25f32;
        e.call_log = Some(vec![]);
        // The router gets `step`'s own argument slot, +0xc of the exe's 0x10 bytes.
        let frame = e.next_stack_local(0x10);
        e.call(0x00d6_0dd0, &args![this, step]);
        assert_eq!(calls_to(&e, ROUTER_CONSUME_COMMANDS)[0][1], frame + 0xc);
        let stepped: Vec<(u32, Vec<u32>)> = log.borrow().clone();
        let bits = step.to_bits();
        assert_eq!(
            stepped,
            vec![(3, vec![bits]), (2, vec![bits]), (1, vec![bits])]
        );
        let consume = calls_to(&e, ROUTER_CONSUME_COMMANDS);
        assert_eq!(consume.len(), 1);
        assert_eq!(consume[0][0], this + 0x18);
        assert_eq!(consume[0][2], archive);
        // Without an input object the commands are not consumed; without processes no step.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0x30, 0);
        log.borrow_mut().clear();
        e.call_log = Some(vec![]);
        e.call(0x00d6_0dd0, &args![this, step]);
        assert!(log.borrow().is_empty());
        assert!(calls_to(&e, ROUTER_CONSUME_COMMANDS).is_empty());
        // A signalling NaN step reaches the processes quieted.
        e.mem.set_u32(this + 0x30, 1);
        e.call(0x00d6_0dd0, &args![this, 0x7f80_0001u32]);
        assert_eq!(log.borrow()[0].1, vec![0x7fc0_0001]);
    }

    #[test]
    fn fn_00d60e20_removes_the_process_by_swapping_in_the_last() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let whole = handler_with_processes(&mut e, &[1, 2, 3], &log);
        let this = whole + 8;
        let data = e.mem.u32(whole + 0x38);
        let first = e.mem.u32(data);
        let last = e.mem.u32(data + 8);
        let second = e.mem.u32(data + 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0e20, &args![this, 1u32]).u32(), 0);
        assert_eq!(e.mem.u32(whole + 0x3c), 2);
        assert_eq!(e.mem.u32(data), last);
        assert_eq!(e.mem.u32(data + 4), second);
        assert_eq!(
            calls_to(&e, ROUTER_UNREGISTER),
            vec![vec![this + 0x1c, first]]
        );
        // The removed process is destroyed with flag 1.
        assert_eq!(*log.borrow(), vec![(0x1001, vec![1])]);
        // Removing the last one leaves the array alone otherwise.
        e.call(0x00d6_0e20, &args![this, 2u32]);
        assert_eq!(e.mem.u32(whole + 0x3c), 1);
        assert_eq!(e.mem.u32(data), last);
        // An unknown id changes nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0e20, &args![this, 77u32]).u32(), 0);
        assert_eq!(e.mem.u32(whole + 0x3c), 1);
        assert!(calls_to(&e, ROUTER_UNREGISTER).is_empty());
    }

    #[test]
    fn fn_00d60e80_adds_a_new_process_and_registers_it() {
        let mut e = engine();
        install_allocator(&mut e);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let whole = handler_with_processes(&mut e, &[1], &log);
        let this = whole + 8;
        // The input, output and third words the new process copies.
        e.mem.set_u32(this + 0xc, 0xaaa1);
        e.mem.set_u32(this + 0x10, 0xbbb2);
        e.mem.set_u32(this + 0x14, 0xccc3);
        let created = process(&mut e, 5, &log);
        e.register_double(REGISTRY_MAKE_PROCESS, move |_, _| answer(created));
        let registry = e.mem.alloc(0x20);
        e.mem.set_u32(PROCESS_REGISTRY, registry);
        e.call_log = Some(vec![]);
        // Id 1 exists: nothing is made.
        assert_eq!(e.call(0x00d6_0e80, &args![this, 1u32]).u32(), 0);
        assert!(calls_to(&e, REGISTRY_MAKE_PROCESS).is_empty());
        // Id 5 is made by the registry, given the second array; the process copies the words.
        assert_eq!(e.call(0x00d6_0e80, &args![this, 5u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, REGISTRY_MAKE_PROCESS),
            vec![vec![registry, 5, this + 0x3c]]
        );
        assert_eq!(e.mem.u32(created + 8), 0xaaa1);
        assert_eq!(e.mem.u32(created + 0xc), 0xbbb2);
        assert_eq!(e.mem.u32(created + 0x10), 0xccc3);
        assert_eq!(e.mem.u32(created + 0x14), this);
        // Slot 0x14 was called, the array grew and the router registered the process.
        assert!(log.borrow().contains(&(0x2005, vec![])));
        assert_eq!(e.mem.u32(whole + 0x3c), 2);
        let data = e.mem.u32(whole + 0x38);
        assert_eq!(e.mem.u32(data + 4), created);
        assert_eq!(
            calls_to(&e, ROUTER_REGISTER),
            vec![vec![this + 0x1c, created]]
        );
        // Size 1 against capacity 8: no growth was asked for.
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn fn_00d60e80_grows_a_full_array_and_reports_a_failed_factory() {
        let mut e = engine();
        let (allocator, _) = install_allocator(&mut e);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let whole = handler_with_processes(&mut e, &[1], &log);
        let this = whole + 8;
        // Size 1 equals capacity 1: the array is grown before the append.
        e.mem.set_u32(whole + 0x40, 1);
        let registry = e.mem.alloc(0x20);
        e.mem.set_u32(PROCESS_REGISTRY, registry);
        e.register(REGISTRY_MAKE_PROCESS, |_, _| answer(0));
        e.call_log = Some(vec![]);
        // No process made: answer 1, nothing else.
        assert_eq!(e.call(0x00d6_0e80, &args![this, 5u32]).u32(), 1);
        assert!(calls_to(&e, ROUTER_REGISTER).is_empty());
        let created = process(&mut e, 5, &log);
        e.register_double(REGISTRY_MAKE_PROCESS, move |_, _| answer(created));
        // The growth double makes room for four.
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let data = e.mem.alloc(16);
            let old = e.mem.u32(a[1]);
            let first = e.mem.u32(old);
            e.mem.set_u32(data, first);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        assert_eq!(e.call(0x00d6_0e80, &args![this, 5u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, this + 0x30, 4]]
        );
        assert_eq!(e.mem.u32(whole + 0x3c), 2);
        let data = e.mem.u32(whole + 0x38);
        assert_eq!(e.mem.u32(data + 4), created);
    }

    #[test]
    fn constructor_builds_the_handler_its_router_and_stream_objects() {
        let mut e = engine();
        let (allocator, log) = install_allocator(&mut e);
        e.register(INPUT_OBJECT_CONSTRUCTOR, |_, a| answer(a[0]));
        e.register(OUTPUT_OBJECT_CONSTRUCTOR, |_, a| answer(a[0]));
        e.register_double(ROUTER_OBJECT_CONSTRUCTOR, |e, a| {
            // The object made from the output object records which one it was given.
            e.mem.set_u32(a[0] + 0x10, a[1]);
            answer(a[0])
        });
        let this = e.mem.alloc(0x60);
        // A command array of three words.
        let source = e.mem.alloc(12);
        for (i, v) in [7u32, 8, 9].iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, *v);
        }
        let commands = e.mem.alloc(12);
        e.mem.set_u32(commands, source);
        e.mem.set_u32(commands + 4, 3);
        e.call_log = Some(vec![]);
        // The byte count goes through `commands`' own argument slot, +0x14 of the exe's 0x20.
        let frame = e.next_stack_local(0x20);
        let got = e
            .call(0x00d6_0f30, &args![this, commands, 0x1111u32, 0x2222u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(calls_to(&e, 0x0b00_000c)[0][1], frame + 0x14);
        // The final vtables, the router and its map, the registered `hkProcess` part.
        assert_eq!(e.mem.u32(this), SERVER_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), SERVER_HANDLER_PROCESS_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), SERVER_HANDLER_PROCESS_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x24), COMMAND_ROUTER_VTABLE);
        assert_eq!(e.mem.u16(this + 0x2a), 1);
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_ffff);
        assert_eq!(
            calls_to(&e, MAP_RESERVE),
            vec![vec![this + 0x2c, allocator, 0]]
        );
        assert_eq!(
            calls_to(&e, ROUTER_REGISTER),
            vec![vec![this + 0x24, this + 0xc]]
        );
        // The process array is empty and not owned; the command array was copied into a block
        // granted for 12 bytes (24 granted: capacity 6).
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0x8000_0000);
        assert_eq!(e.mem.i32(this + 0x48), 3);
        assert_eq!(e.mem.i32(this + 0x4c), 6);
        let copy = e.mem.u32(this + 0x44);
        assert_eq!(e.mem.bytes(copy, 12), e.mem.bytes(source, 12));
        // Input and output objects: 0x10-byte blocks with their size word, kept in the fields.
        let input = e.mem.u32(this + 0x14);
        let output = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u16(input + 4), 0x10);
        assert_eq!(e.mem.u16(output + 4), 0x10);
        assert_eq!(
            calls_to(&e, INPUT_OBJECT_CONSTRUCTOR),
            vec![vec![input, 0x1111]]
        );
        assert_eq!(
            calls_to(&e, OUTPUT_OBJECT_CONSTRUCTOR),
            vec![vec![output, 0x2222]]
        );
        // The 0x14-byte object is made from the output object; the field holds it plus 8.
        let router_object = e.mem.u32(this + 0x1c) - 8;
        assert_eq!(e.mem.u16(router_object + 4), 0x14);
        assert_eq!(e.mem.u32(router_object + 0x10), output);
        assert_eq!(e.mem.u32(this + 0x20), this + 8);
        let sizes: Vec<u32> = log
            .borrow()
            .iter()
            .filter(|c| c.0 == 4)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(sizes, vec![0x10, 0x10, 0x14]);
    }

    #[test]
    fn constructor_without_streams_or_commands_keeps_the_fields_empty() {
        let mut e = engine();
        install_allocator(&mut e);
        e.register(ROUTER_OBJECT_CONSTRUCTOR, |_, _| answer(0));
        let this = e.mem.alloc(0x60);
        let commands = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x00d6_0f30, &args![this, commands, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        // The router object construction answers none: the field is 0, not 8.
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert!(calls_to(&e, INPUT_OBJECT_CONSTRUCTOR).is_empty());
        assert!(calls_to(&e, OUTPUT_OBJECT_CONSTRUCTOR).is_empty());
        // No commands: no storage is asked for and the second array stays empty.
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0x8000_0000);
    }

    #[test]
    fn destructor_destroys_the_processes_and_objects_and_empties_the_arrays() {
        let mut e = engine();
        let (allocator, log) = install_allocator(&mut e);
        let events: Log = Rc::new(RefCell::new(Vec::new()));
        let this = handler_with_processes(&mut e, &[1, 2], &events);
        // Slot 0 of the objects at +0x14, +0x18 and +0x1c logs its destruction.
        let object = |e: &mut Engine, tag: u32| {
            let l = Rc::clone(&events);
            e.register_double(0x0b40_0000 + tag, move |_, a| {
                l.borrow_mut().push((tag, a[1..].to_vec()));
                Ret::default()
            });
            object_with_slots(e, &[(0, 0x0b40_0000 + tag)])
        };
        let a = object(&mut e, 0x14);
        let b = object(&mut e, 0x18);
        e.mem.set_u32(this + 0x14, a);
        e.mem.set_u32(this + 0x18, b);
        // `+0x1c` stays null: skipped. The first array owns storage (capacity 8); the second
        // array does not.
        let storage = e.mem.u32(this + 0x38);
        e.mem.set_u32(this + 0x4c, 0x8000_0010);
        e.mem.set_u32(this + 0x48, 5);
        e.call_log = Some(vec![]);
        e.call(0x00d6_10e0, &args![this]);
        assert_eq!(
            *events.borrow(),
            vec![
                (0x1001, vec![1]),
                (0x1002, vec![1]),
                (0x14, vec![1]),
                (0x18, vec![1])
            ]
        );
        // Only the process array (owned, capacity 8 -> 32 bytes) is freed.
        assert_eq!(*log.borrow(), vec![(0x10, vec![storage, 32])]);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0x8000_0000);
        assert_eq!(
            calls_to(&e, MAP_CLEAR_AND_DEALLOCATE),
            vec![vec![this + 0x2c, allocator]]
        );
        assert_eq!(calls_to(&e, MAP_CLEANUP), vec![vec![this + 0x2c]]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), PROCESS_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), PROCESS_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), BASE_OBJECT_VTABLE);
    }

    #[test]
    fn destructor_frees_an_owned_second_array_and_skips_null_processes() {
        let mut e = engine();
        let (_, log) = install_allocator(&mut e);
        let events: Log = Rc::new(RefCell::new(Vec::new()));
        let this = handler_with_processes(&mut e, &[1, 2], &events);
        let data = e.mem.u32(this + 0x38);
        e.mem.set_u32(data, 0);
        let second = e.mem.alloc(16);
        e.mem.set_u32(this + 0x44, second);
        e.mem.set_u32(this + 0x4c, 3);
        e.call(0x00d6_10e0, &args![this]);
        assert_eq!(*events.borrow(), vec![(0x1002, vec![1])]);
        let freed = log.borrow();
        assert_eq!(freed[0], (0x10, vec![second, 12]));
        assert_eq!(freed.len(), 2);
    }

    #[test]
    fn is_ok_asks_the_output_archive() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        // No archive: false without asking.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00d6_11e0, &args![this]).bool());
        assert!(calls_to(&e, OARCHIVE_IS_OK).is_empty());
        e.mem.set_u32(this + 0xc, 0xa4c1);
        for ok in [true, false] {
            e.register_double(OARCHIVE_IS_OK, move |e, a| {
                e.mem.set_u8(a[1], u8::from(ok));
                answer(a[1])
            });
            // The flag byte at +0x3 of the exe's 4-byte frame.
            let frame = e.next_stack_local(4);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00d6_11e0, &args![this]).bool(), ok);
            assert_eq!(calls_to(&e, OARCHIVE_IS_OK)[0][1], frame + 3);
        }
    }
}

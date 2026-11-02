//! `Havok SDK/run_00dd97e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dda660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_9d20`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dda660(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_9d20, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dda650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00dda660`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dda650(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_a660, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// The functions below are members of `hkServerDebugDisplayHandler` (the RTTI name of the vtables
// `010d7da8` and `010d7d54`). The object has `hkDebugDisplayHandler` as a second base at `+8` (the
// vtable `010d7d54`); the virtual functions below are called with that second base as `this`,
// so for them `this + 4` is the `hkOArchive` the commands are written to (the object's `+0xc`)
// and `this + 8` the critical section that guards it (the object's `+0x10`). The Xbox PDB has no
// entry for `hkServerDebugDisplayHandler`, but it names the slots of `hkDebugDisplayHandler`:
// each function below is the entry of the vtable `010d7d54` at the slot named in its doc comment.
// Every one of them returns the exe's 1-for-failure answer: 0 when the archive is there and
// reports ok, else 1.

/// The vtables of `hkServerDebugDisplayHandler` (`010d7da8`, and its second base's `010d7d54`),
/// of `hkDebugDisplayHandler` (`010d7cfc`), of `hkDisplaySerializeOStream` (`010d7db8`) and of
/// `hkBaseObject` (`0102e388`).
const HANDLER_VTABLE: u32 = 0x010d_7da8;
const HANDLER_INTERFACE_VTABLE: u32 = 0x010d_7d54;
const DISPLAY_HANDLER_VTABLE: u32 = 0x010d_7cfc;
const DISPLAY_OSTREAM_VTABLE: u32 = 0x010d_7db8;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The import slots of `EnterCriticalSection` (`00fdf05c`), `LeaveCriticalSection` (`00fdf100`)
/// and `DeleteCriticalSection` (`00fdf058`).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `hkCriticalSection::hkCriticalSection` (Xbox PDB, `00cf6da0`, `thiscall`, `RET 4`: the spin
/// count); answers `this`.
const CRITICAL_SECTION_CONSTRUCTOR: u32 = 0x00cf_6da0;
/// `hkOArchive` members (each `thiscall` on the archive, Xbox PDB names): `write16u` (`00d60920`),
/// `write32` (`00d60940`), `write32u` (`00d60960`), `write64u` (`00d609a0`: low word, high word),
/// `writeFloat32` (`00d609c0`: the float as a word), `writeRaw` (`00d607b0`: pointer, count), and
/// `isOk` (`00d607e0`: a pointer to a local byte; answers a pointer to the byte holding the
/// answer). `00d605d0` (one word, a command number; the engine map gives no name) and
/// `hkDisplaySerializeOStream::writeTransform` (`00dda740`: a pointer to a transform) and `00dda730`
/// (a pointer to a vector) are also called on the archive.
const ARCHIVE_WRITE_16U: u32 = 0x00d6_0920;
const ARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE_32U: u32 = 0x00d6_0960;
const ARCHIVE_WRITE_64U: u32 = 0x00d6_09a0;
const ARCHIVE_WRITE_FLOAT32: u32 = 0x00d6_09c0;
const ARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const ARCHIVE_IS_OK: u32 = 0x00d6_07e0;
const ARCHIVE_WRITE_COMMAND: u32 = 0x00d6_05d0;
const WRITE_TRANSFORM: u32 = 0x00dd_a740;
const WRITE_VECTOR: u32 = 0x00dd_a730;
/// `hkString::strLen` (Xbox PDB, `00cb10d0`, cdecl: the text).
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// `00c90940` (cdecl: allocator, array, element size) makes room for one more element of an `hkArray`.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The size function (`00dda850`, cdecl: one pointer) and the writer (`00dda8c0`, `thiscall` on
/// the archive: one pointer) for one element of the geometry arrays (translated in
/// `run_00dda7c0`).
const ELEMENT_SIZE: u32 = 0x00dd_a850;
const WRITE_ELEMENT: u32 = 0x00dd_a8c0;
/// `00ce5300` (`thiscall` on a 0x30-byte record, `RET 4`: the destination, 0x40 bytes): makes the
/// 0x40-byte transform `writeTransform` takes from a record of the transform array.
const CONVERT_TRANSFORM: u32 = 0x00ce_5300;
/// `hkOArchive::hkOArchive` (`00d60b40`, `thiscall`, `RET 8`: the writer and a word whose low byte
/// is the byte-swap flag).
const ARCHIVE_CONSTRUCTOR: u32 = 0x00d6_0b40;

/// Enters the critical section the object's word at `+8` points at.
fn begin_commands(e: &mut Engine, this: u32) {
    let section = e.mem.u32(this + 8);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
}

/// Calls `function` on the archive at `this + 4` (read each time, as the code does) with the
/// words.
fn emit(e: &mut Engine, this: u32, function: u32, words: &[u32]) {
    let archive = e.mem.u32(this + 4);
    let mut all = vec![archive];
    all.extend_from_slice(words);
    e.call(function, &all);
}

/// `CDQ`: the high word of a sign-extended word.
fn sign_word(value: u32) -> u32 {
    ((value as i32) >> 31) as u32
}

/// Asks the archive (when there is one) whether it is ok, leaves the critical section and
/// answers 1 for failure (no archive or not ok), else 0.
fn end_commands(e: &mut Engine, this: u32) -> u32 {
    let archive = e.mem.u32(this + 4);
    let mut ok = false;
    if archive != 0 {
        let scratch = e.mem.alloc(8);
        let answer = e.call(ARCHIVE_IS_OK, &args![archive, scratch]).u32();
        ok = e.mem.u8(answer) != 0;
        e.mem.free(scratch);
    }
    let section = e.mem.u32(this + 8);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    u32::from(!ok)
}

/// The clamped length of the text the string commands write: `strLen`, at most 0xffff
/// (signed compare).
fn clamped_length(e: &mut Engine, text: u32) -> u32 {
    let length = e.call(STRING_LENGTH, &args![text]).i32();
    if length < 0xffff {
        length as u32
    } else {
        0xffff
    }
}

// Translated from 00dd97e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x08` of the vtable `010d7d54`, which the Xbox PDB's `hkDebugDisplayHandler` names
/// `addGeometryInstance` (`thiscall`, `RET 0x14`: five words it does not read). Writes nothing:
/// it only reports the archive's state (see above).
pub fn hk_server_debug_display_handler_add_geometry_instance(
    e: &mut Engine,
    this: u32,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    begin_commands(e, this);
    end_commands(e, this)
}

// Translated from 00dd9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x0c` of the vtable `010d7d54`, `setGeometryColor` in the Xbox PDB (`thiscall`, `RET
/// 0xc`: `id`, `color`, `tag`). With an archive it writes command 0x11 (`write32u(0x11)`,
/// command word 3), `write32u(id)`, `write64u(color, 0)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_set_geometry_color(
    e: &mut Engine,
    this: u32,
    id: u32,
    color: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[0x11]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[3]);
        emit(e, this, ARCHIVE_WRITE_32U, &[id]);
        emit(e, this, ARCHIVE_WRITE_64U, &[color, 0]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd98c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x10` of the vtable `010d7d54`, `updateGeometry` in the Xbox PDB (`thiscall`, `RET
/// 0xc`: `transform`, `id`, `tag`). With an archive it writes `write32u(0x29)`, command word 2,
/// the transform (`00dda740`), `write64u(id, 0)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_update_geometry(
    e: &mut Engine,
    this: u32,
    transform: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[0x29]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[2]);
        emit(e, this, WRITE_TRANSFORM, &[transform]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, 0]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x14` of the vtable `010d7d54`, `removeGeometry` in the Xbox PDB (`thiscall`, `RET 0xc`:
/// `id`, `tag` and a word it does not read). With an archive it writes `write32u(0xd)`, command
/// word 4, `write64u(id, 0)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_remove_geometry(
    e: &mut Engine,
    this: u32,
    id: u32,
    tag: u32,
    _unused_2: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[0xd]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[4]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, 0]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x18` of the vtable `010d7d54`, `updateCamera` in the Xbox PDB (`thiscall`, `RET 0x1c`:
/// three vector pointers `a`, `b`, `c`, three `float`s, a text). With an archive it writes
/// `write32u(length + 0x33)` (the `strLen` of the text), command word 0xd, the three vectors
/// (`00dda730`), the three floats (each through the x87 stack, so a signalling NaN is quiet),
/// `write16u(length)` and the text (`writeRaw(text, length)`).
pub fn hk_server_debug_display_handler_update_camera(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    first: f32,
    second: f32,
    third: f32,
    text: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let length = e.call(STRING_LENGTH, &args![text]).u32();
        emit(e, this, ARCHIVE_WRITE_32U, &[length.wrapping_add(0x33)]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[0xd]);
        emit(e, this, WRITE_VECTOR, &[a]);
        emit(e, this, WRITE_VECTOR, &[b]);
        emit(e, this, WRITE_VECTOR, &[c]);
        for value in [first, second, third] {
            emit(
                e,
                this,
                ARCHIVE_WRITE_FLOAT32,
                &[x87_quiet_bits(value.to_bits())],
            );
        }
        emit(e, this, ARCHIVE_WRITE_16U, &[length]);
        emit(e, this, ARCHIVE_WRITE_RAW, &[text, length]);
    }
    end_commands(e, this)
}

// Translated from 00dd9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x20` of the vtable `010d7d54`, `displayPoint` in the Xbox PDB (`thiscall`, `RET 0x10`:
/// `position` (a vector pointer), `color`, `id`, `tag`). With an archive it writes
/// `write32u(0x1d)`, command word 7, the vector, `write32u(color)`, `write64u(id, sign of
/// id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_point(
    e: &mut Engine,
    this: u32,
    position: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[0x1d]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[7]);
        emit(e, this, WRITE_VECTOR, &[position]);
        emit(e, this, ARCHIVE_WRITE_32U, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd9b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x24` of the vtable `010d7d54`, `displayLine` in the Xbox PDB (`thiscall`, `RET 0x14`:
/// `from`, `to` (vector pointers), `color`, `id`, `tag`). With an archive it writes
/// `write32u(0x29)`, command word 8, the two vectors, `write32u(color)`, `write64u(id, sign of
/// id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_line(
    e: &mut Engine,
    this: u32,
    from: u32,
    to: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[0x29]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[8]);
        emit(e, this, WRITE_VECTOR, &[from]);
        emit(e, this, WRITE_VECTOR, &[to]);
        emit(e, this, ARCHIVE_WRITE_32U, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd9be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x40` of the vtable `010d7d54`, `sendMemStatsDump` in the Xbox PDB (`thiscall`, `RET 8`:
/// `data`, `size`). With an archive it writes `write32u(size + 5)`, command word 0xf,
/// `write32(size)` and `writeRaw(data, size)`.
pub fn hk_server_debug_display_handler_send_mem_stats_dump(
    e: &mut Engine,
    this: u32,
    data: u32,
    size: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[size.wrapping_add(5)]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[0xf]);
        emit(e, this, ARCHIVE_WRITE_32, &[size]);
        emit(e, this, ARCHIVE_WRITE_RAW, &[data, size]);
    }
    end_commands(e, this)
}

// Translated from 00dd9c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x44` of the vtable `010d7d54`, `holdImmediate` in the Xbox PDB (`fastcall` on `this`,
/// no stack words). With an archive it writes `write32u(1)` and command word 0xb.
pub fn hk_server_debug_display_handler_hold_immediate(e: &mut Engine, this: u32) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        emit(e, this, ARCHIVE_WRITE_32U, &[1]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[0xb]);
    }
    end_commands(e, this)
}

// Translated from 00dd9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `archive`): the constructor of
/// `hkServerDebugDisplayHandler`. Sets the reference count (`u16` at `+6`) to 1, the vtables
/// (`010d7da8` at `+0`, `010d7d54` at `+8`; the code stores `010d7cfc` at `+8` first), the archive
/// at `+0xc`, and takes 0x18 bytes from the memory router's allocator (word at `+0x10`, virtual
/// function `+4`) for a critical section (`00cf6da0` with spin count 1000) whose pointer it stores
/// at `+0x10` (0 when the allocator gave nothing). Answers `this`.
pub fn fn_00dd9cc0(e: &mut Engine, this: u32, archive: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this + 8, DISPLAY_HANDLER_VTABLE);
    e.mem.set_u32(this, HANDLER_VTABLE);
    e.mem.set_u32(this + 8, HANDLER_INTERFACE_VTABLE);
    e.mem.set_u32(this + 0xc, archive);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    if block != 0 {
        let section = e
            .call(CRITICAL_SECTION_CONSTRUCTOR, &args![block, 1000u32])
            .u32();
        e.mem.set_u32(this + 0x10, section);
        return this;
    }
    e.mem.set_u32(this + 0x10, 0);
    this
}

// Translated from 00dd9d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`): the destructor body of
/// `hkServerDebugDisplayHandler`. Sets the two vtables, and when there is a critical section
/// (`+0x10`) deletes it (`DeleteCriticalSection`) and returns its 0x18 bytes to the memory
/// router's allocator (virtual function `+8`: block, size); then leaves the `hkBaseObject`
/// vtable at `+0` and the `hkDebugDisplayHandler` one at `+8`.
pub fn fn_00dd9d20(e: &mut Engine, this: u32) {
    let section = e.mem.u32(this + 0x10);
    e.mem.set_u32(this, HANDLER_VTABLE);
    e.mem.set_u32(this + 8, HANDLER_INTERFACE_VTABLE);
    if section != 0 {
        e.call(DELETE_CRITICAL_SECTION, &args![section]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![section, 0x18u32]);
    }
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
    e.mem.set_u32(this + 8, DISPLAY_HANDLER_VTABLE);
}

// Translated from 00dd9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The array is in a register: `array` (`EDI`, an `hkArray` of
/// pointers), no stack words. Answers 4 plus the sum of `00dda850` (cdecl) over the elements.
pub fn fn_00dd9d70(e: &mut Engine, array: u32) -> i32 {
    let mut total = 4i32;
    let mut index = 0i32;
    if e.mem.i32(array + 4) > 0 {
        loop {
            let data = e.mem.u32(array);
            let element = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let size = e.call(ELEMENT_SIZE, &args![element]).i32();
            index += 1;
            total = total.wrapping_add(size);
            if index >= e.mem.i32(array + 4) {
                break;
            }
        }
    }
    total
}

// Translated from 00dd9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (the handler object itself, not its
/// second base: its archive is at `+0xc` and its critical section at `+0x10`), `array`). Inside the
/// critical section writes the array's count (`write32`) and then every element with `00dda8c0`
/// (`thiscall` on the archive).
pub fn fn_00dd9da0(e: &mut Engine, this: u32, array: u32) {
    let section = e.mem.u32(this + 0x10);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let count = e.mem.u32(array + 4);
    let archive = e.mem.u32(this + 0xc);
    e.call(ARCHIVE_WRITE_32, &args![archive, count]);
    let mut index = 0i32;
    if e.mem.i32(array + 4) > 0 {
        loop {
            let data = e.mem.u32(array);
            let element = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let archive = e.mem.u32(this + 0xc);
            e.call(WRITE_ELEMENT, &args![archive, element]);
            index += 1;
            if index >= e.mem.i32(array + 4) {
                break;
            }
        }
    }
    let section = e.mem.u32(this + 0x10);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00dd9df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x04` of the vtable `010d7d54`, `addGeometry` in the Xbox PDB (`thiscall`, `RET 0x14`:
/// `geometries` (an `hkArray` of pointers), `transform`, `id`, `tag` and a word it does not
/// read). With an archive it writes `write32(00dd9d70(geometries) + 0x29)`, command word 1, the
/// array (`00dd9da0` on the object at `this - 8`), the transform, `write64u(id, 0)` and
/// `write32(tag)`.
pub fn hk_server_debug_display_handler_add_geometry(
    e: &mut Engine,
    this: u32,
    geometries: u32,
    transform: u32,
    id: u32,
    tag: u32,
    _unused_4: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let size = fn_00dd9d70(e, geometries);
        emit(e, this, ARCHIVE_WRITE_32, &[size.wrapping_add(0x29) as u32]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[1]);
        fn_00dd9da0(e, this.wrapping_sub(8), geometries);
        emit(e, this, WRITE_TRANSFORM, &[transform]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, 0]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x3c` of the vtable `010d7d54`, the second `displayGeometry` in the Xbox PDB (`thiscall`,
/// `RET 0x14`: `geometries`, `transform`, `color`, `id`, `tag`). With an archive it writes
/// `write32(00dd9d70(geometries) + 0x2d)`, command word 5, the array (`00dd9da0` on `this - 8`),
/// the transform, `write32u(color)`, `write64u(id, sign of id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_geometry_ov2(
    e: &mut Engine,
    this: u32,
    geometries: u32,
    transform: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let size = fn_00dd9d70(e, geometries);
        emit(e, this, ARCHIVE_WRITE_32, &[size.wrapping_add(0x2d) as u32]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[5]);
        fn_00dd9da0(e, this.wrapping_sub(8), geometries);
        emit(e, this, WRITE_TRANSFORM, &[transform]);
        emit(e, this, ARCHIVE_WRITE_32U, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dd9f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x38` of the vtable `010d7d54`, the first `displayGeometry` in the Xbox PDB (`thiscall`,
/// `RET 0x10`: `geometries`, `color`, `id`, `tag`). With an archive it writes
/// `write32(00dd9d70(geometries) + 0x11)`, command word 6, the array (`00dd9da0` on `this - 8`),
/// `write32u(color)`, `write64u(id, sign of id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_geometry(
    e: &mut Engine,
    this: u32,
    geometries: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let size = fn_00dd9d70(e, geometries);
        emit(e, this, ARCHIVE_WRITE_32, &[size.wrapping_add(0x11) as u32]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[6]);
        fn_00dd9da0(e, this.wrapping_sub(8), geometries);
        emit(e, this, ARCHIVE_WRITE_32U, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

/// `displayTriangle`'s frame under the 16-byte boundary (`SUB ESP,0x54` and three pushes).
const TRIANGLE_FRAME_BELOW: u32 = 0x60;

// Translated from 00dd9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x28` of the vtable `010d7d54`, `displayTriangle` in the Xbox PDB (`thiscall`, `RET 0x18`:
/// three vector pointers `a`, `b`, `c`, `color`, `id`, `tag`). With an archive it copies the three
/// vectors (and `a` again) to a local array and writes, for the three edges `(a, b)`, `(b, c)`,
/// `(c, a)`, the same command as `displayLine`: `write32u(0x29)`, command word 8, the two vectors
/// (`00dda730`), `write32u(color)`, `write64u(id, sign of id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_triangle(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        // The copies sit at +0x20 of the exe's frame (aligned: `SUB ESP,0x54` and three pushes
        // under the boundary, 00dda011..00dda027).
        with_aligned_frame(e, TRIANGLE_FRAME_BELOW, |e, frame| {
            let copies = frame + 0x20;
            let copy_vector = |e: &mut Engine, from: u32, to: u32| {
                for lane in 0..4 {
                    let word = e.mem.u32(from + 4 * lane);
                    e.mem.set_u32(to + 4 * lane, word);
                }
            };
            copy_vector(e, b, copies + 0x10);
            copy_vector(e, a, copies);
            copy_vector(e, c, copies + 0x20);
            copy_vector(e, a, copies + 0x30);
            for edge in 0..3u32 {
                let first = copies + 0x10 * edge;
                emit(e, this, ARCHIVE_WRITE_32U, &[0x29]);
                emit(e, this, ARCHIVE_WRITE_COMMAND, &[8]);
                emit(e, this, WRITE_VECTOR, &[first]);
                emit(e, this, WRITE_VECTOR, &[first + 0x10]);
                emit(e, this, ARCHIVE_WRITE_32U, &[color]);
                emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
                emit(e, this, ARCHIVE_WRITE_32, &[tag]);
            }
        });
    }
    end_commands(e, this)
}

// Translated from 00dda0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x2c` of the vtable `010d7d54`, `displayText` in the Xbox PDB (`thiscall`, `RET 0x10`:
/// `text`, `color`, `id`, `tag`). With an archive it takes `length` as the `strLen` of the text,
/// at most 0xffff (signed compare), and writes `write32u(length + 0x13)`, command word 9,
/// `write16u(length)`, `writeRaw(text, length)`, `write32(color)`, `write64u(id, sign of id)` and
/// `write32(tag)`.
pub fn hk_server_debug_display_handler_display_text(
    e: &mut Engine,
    this: u32,
    text: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let length = clamped_length(e, text);
        emit(e, this, ARCHIVE_WRITE_32U, &[length.wrapping_add(0x13)]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[9]);
        emit(e, this, ARCHIVE_WRITE_16U, &[length]);
        emit(e, this, ARCHIVE_WRITE_RAW, &[text, length]);
        emit(e, this, ARCHIVE_WRITE_32, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

// Translated from 00dda1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x30` of the vtable `010d7d54`, `display3dText` in the Xbox PDB (`thiscall`, `RET 0x14`:
/// `text`, `position` (a pointer to three `float`s), `color`, `id`, `tag`). With an archive it
/// takes `length` as for `displayText` and writes `write32u(length + 0x1f)`, command word 0x11,
/// `write16u(length)`, `writeRaw(text, length)`, `write32(color)`, `write64u(id, sign of id)`,
/// `write32(tag)` and the three floats of `position` (each through the x87 stack).
pub fn hk_server_debug_display_handler_display3d_text(
    e: &mut Engine,
    this: u32,
    text: u32,
    position: u32,
    color: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let length = clamped_length(e, text);
        emit(e, this, ARCHIVE_WRITE_32U, &[length.wrapping_add(0x1f)]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[0x11]);
        emit(e, this, ARCHIVE_WRITE_16U, &[length]);
        emit(e, this, ARCHIVE_WRITE_RAW, &[text, length]);
        emit(e, this, ARCHIVE_WRITE_32, &[color]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
        for lane in 0..3 {
            let bits = e.mem.u32(position + 4 * lane);
            emit(e, this, ARCHIVE_WRITE_FLOAT32, &[x87_quiet_bits(bits)]);
        }
    }
    end_commands(e, this)
}

// Translated from 00dda290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x34` of the vtable `010d7d54`, `displayAnnotation` in the Xbox PDB (`thiscall`, `RET
/// 0xc`: `text`, `id`, `tag`). With an archive it takes `length` as for `displayText` and
/// writes `write32u(length + 0xf)`, command word 0x14, `write16u(length)`, `writeRaw(text,
/// length)`, `write64u(id, sign of id)` and `write32(tag)`.
pub fn hk_server_debug_display_handler_display_annotation(
    e: &mut Engine,
    this: u32,
    text: u32,
    id: u32,
    tag: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let length = clamped_length(e, text);
        emit(e, this, ARCHIVE_WRITE_32U, &[length.wrapping_add(0xf)]);
        emit(e, this, ARCHIVE_WRITE_COMMAND, &[0x14]);
        emit(e, this, ARCHIVE_WRITE_16U, &[length]);
        emit(e, this, ARCHIVE_WRITE_RAW, &[text, length]);
        emit(e, this, ARCHIVE_WRITE_64U, &[id, sign_word(id)]);
        emit(e, this, ARCHIVE_WRITE_32, &[tag]);
    }
    end_commands(e, this)
}

/// `updateBehavior`'s frame under the 16-byte boundary (`SUB ESP,0x64` and three pushes).
const UPDATE_BEHAVIOR_FRAME_BELOW: u32 = 0x70;

// Translated from 00dda340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+0x1c` of the vtable `010d7d54`, `updateBehavior` in the Xbox PDB (`thiscall`, `RET 0x20`:
/// eight `hkArray` pointers `a` to `h` (`a`, `b`, `c`, `f`, `g`: arrays of words; `d`: 16-byte
/// vectors; `e`: pointers to texts (null allowed); `h`: 0x30-byte transform records)). With an
/// archive it first measures the record: `7 * h.size + 2 * (b.size + 2 * d.size) + f.size +
/// g.size`, times 4, plus 0x19, plus for each non-null text `2 + strLen`; the text lengths are
/// kept in a temporary `hkArray` (the router's allocator makes room with `00c90940`, element
/// size 4, and the storage is freed at the end through virtual function `+0x10` when it is on
/// the heap). It then writes `write32u(size)`, command word 0x12, and `b.size` followed by the
/// pairs `(a[i], b[i])`; `d.size` followed by `(c[i], vector d[i])`; `e.size` as a 16-bit word
/// and for each non-null text `write16u(length)` and `writeRaw(text, length)`; `h.size` followed
/// by each transform converted with `00ce5300` into a 0x40-byte local and written with
/// `writeTransform`; then `f.size` and the words of `f`, and `g.size` and the words of `g`.
pub fn hk_server_debug_display_handler_update_behavior(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    texts: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    begin_commands(e, this);
    if e.mem.u32(this + 4) != 0 {
        let b_size = e.mem.u32(b + 4);
        let d_size = e.mem.u32(d + 4);
        let h_size = e.mem.u32(h + 4);
        let mut record = h_size
            .wrapping_mul(7)
            .wrapping_add(b_size.wrapping_add(d_size.wrapping_mul(2)).wrapping_mul(2))
            .wrapping_add(e.mem.u32(f + 4))
            .wrapping_add(e.mem.u32(g + 4));
        record = record.wrapping_mul(4).wrapping_add(0x19);
        // The exe's frame (aligned: `SUB ESP,0x64` and three pushes under the boundary) holds
        // the lengths of the texts at +0x24, an array that starts empty without storage, and the
        // converted transform at +0x30 (00dda563).
        with_aligned_frame(e, UPDATE_BEHAVIOR_FRAME_BELOW, |e, frame| {
            let lengths = frame + 0x24;
            e.mem.set_u32(lengths, 0);
            e.mem.set_u32(lengths + 4, 0);
            e.mem.set_u32(lengths + 8, 0x8000_0000);
            let mut index = 0i32;
            if e.mem.i32(texts + 4) > 0 {
                loop {
                    let data = e.mem.u32(texts);
                    let text = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                    let length = if text != 0 {
                        record = record.wrapping_add(2);
                        let length = e.call(STRING_LENGTH, &args![text]).u32();
                        record = record.wrapping_add(length);
                        length
                    } else {
                        0
                    };
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let capacity = e.mem.u32(lengths + 8) & 0x3fff_ffff;
                    if e.mem.u32(lengths + 4) == capacity {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(ARRAY_RESERVE_MORE, &args![allocator, lengths, 4u32]);
                    }
                    let at = e
                        .mem
                        .u32(lengths)
                        .wrapping_add(e.mem.u32(lengths + 4).wrapping_mul(4));
                    e.mem.set_u32(at, length);
                    let size = e.mem.u32(lengths + 4);
                    e.mem.set_u32(lengths + 4, size.wrapping_add(1));
                    index += 1;
                    if index >= e.mem.i32(texts + 4) {
                        break;
                    }
                }
            }
            emit(e, this, ARCHIVE_WRITE_32U, &[record]);
            emit(e, this, ARCHIVE_WRITE_COMMAND, &[0x12]);
            emit(e, this, ARCHIVE_WRITE_32U, &[b_size]);
            let mut index = 0i32;
            if e.mem.i32(b + 4) > 0 {
                loop {
                    let first = e
                        .mem
                        .u32(e.mem.u32(a).wrapping_add((index as u32).wrapping_mul(4)));
                    emit(e, this, ARCHIVE_WRITE_32U, &[first]);
                    let second = e
                        .mem
                        .u32(e.mem.u32(b).wrapping_add((index as u32).wrapping_mul(4)));
                    emit(e, this, ARCHIVE_WRITE_32U, &[second]);
                    index += 1;
                    if index >= e.mem.i32(b + 4) {
                        break;
                    }
                }
            }
            emit(e, this, ARCHIVE_WRITE_32U, &[d_size]);
            let mut index = 0i32;
            if e.mem.i32(d + 4) > 0 {
                loop {
                    let key = e
                        .mem
                        .u32(e.mem.u32(c).wrapping_add((index as u32).wrapping_mul(4)));
                    emit(e, this, ARCHIVE_WRITE_32U, &[key]);
                    let vector = e.mem.u32(d).wrapping_add((index as u32).wrapping_mul(0x10));
                    emit(e, this, WRITE_VECTOR, &[vector]);
                    index += 1;
                    if index >= e.mem.i32(d + 4) {
                        break;
                    }
                }
            }
            let text_count = u32::from(e.mem.u16(texts + 4));
            emit(e, this, ARCHIVE_WRITE_32U, &[text_count]);
            let mut index = 0i32;
            if e.mem.i32(texts + 4) > 0 {
                loop {
                    let data = e.mem.u32(texts);
                    let text = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                    if text != 0 {
                        let length = e.mem.u32(
                            e.mem
                                .u32(lengths)
                                .wrapping_add((index as u32).wrapping_mul(4)),
                        );
                        emit(e, this, ARCHIVE_WRITE_16U, &[length]);
                        let data = e.mem.u32(texts);
                        let text = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                        emit(e, this, ARCHIVE_WRITE_RAW, &[text, length]);
                    }
                    index += 1;
                    if index >= e.mem.i32(texts + 4) {
                        break;
                    }
                }
            }
            emit(e, this, ARCHIVE_WRITE_32U, &[h_size]);
            let mut index = 0i32;
            if e.mem.i32(h + 4) > 0 {
                let converted = frame + 0x30;
                loop {
                    let record_at = e.mem.u32(h).wrapping_add((index as u32).wrapping_mul(0x30));
                    e.call(CONVERT_TRANSFORM, &args![record_at, converted]);
                    emit(e, this, WRITE_TRANSFORM, &[converted]);
                    index += 1;
                    if index >= e.mem.i32(h + 4) {
                        break;
                    }
                }
            }
            for list in [f, g] {
                let count = e.mem.u32(list + 4);
                emit(e, this, ARCHIVE_WRITE_32U, &[count]);
                let mut index = 0i32;
                if e.mem.i32(list + 4) > 0 {
                    loop {
                        let word = e
                            .mem
                            .u32(e.mem.u32(list).wrapping_add((index as u32).wrapping_mul(4)));
                        emit(e, this, ARCHIVE_WRITE_32U, &[word]);
                        index += 1;
                        if index >= e.mem.i32(list + 4) {
                            break;
                        }
                    }
                }
            }
            release_lengths(e, lengths);
        });
    }
    end_commands(e, this)
}

/// The end of `updateBehavior`: the router is asked, the array's size set to 0, and its storage
/// freed through the allocator's virtual function `+0x10` (block, capacity times 4) unless the
/// capacity word has its sign bit set.
fn release_lengths(e: &mut Engine, lengths: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(lengths + 8);
    e.mem.set_u32(lengths + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(lengths);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 2]);
    }
}

// Translated from 00dda690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `writer`): the constructor of
/// `hkDisplaySerializeOStream` (the vtable's RTTI name): runs the `hkOArchive` constructor
/// (`00d60b40`) with `(writer, this & 0xffffff00)` (the code pushes `this` with its low byte
/// cleared; the callee reads the low byte as the byte-swap flag, so 0) and stores the
/// `hkDisplaySerializeOStream` vtable (`010d7db8`). Answers `this`.
pub fn fn_00dda690(e: &mut Engine, this: u32, writer: u32) -> u32 {
    e.call(
        ARCHIVE_CONSTRUCTOR,
        &args![this, writer, this & 0xffff_ff00],
    );
    e.mem.set_u32(this, DISPLAY_OSTREAM_VTABLE);
    this
}

// Translated from 00dda6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an archive), `source`). Writes the
/// three words at `source` with `hkOArchive::write32` (`00d60940`).
pub fn fn_00dda6b0(e: &mut Engine, this: u32, source: u32) {
    for word in 0..3 {
        let value = e.mem.u32(source + 4 * word);
        e.call(ARCHIVE_WRITE_32, &args![this, value]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_a660, deleting_destructor_00dda660(Ptr, u8) -> Ptr),
        entry!(0x00dd_a650, fn_00dda650(Ptr, u32) -> u32),
        entry!(
            0x00dd97e0,
            hk_server_debug_display_handler_add_geometry_instance(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9830,
            hk_server_debug_display_handler_set_geometry_color(u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd98c0,
            hk_server_debug_display_handler_update_geometry(u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9950,
            hk_server_debug_display_handler_remove_geometry(u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd99d0,
            hk_server_debug_display_handler_update_camera(u32, u32, u32, u32, f32, f32, f32, u32) -> u32
        ),
        entry!(
            0x00dd9ab0,
            hk_server_debug_display_handler_display_point(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9b40,
            hk_server_debug_display_handler_display_line(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9be0,
            hk_server_debug_display_handler_send_mem_stats_dump(u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9c60,
            hk_server_debug_display_handler_hold_immediate(u32) -> u32
        ),
        entry!(0x00dd9cc0, fn_00dd9cc0(u32, u32) -> u32),
        entry!(0x00dd9d20, fn_00dd9d20(u32)),
        entry!(0x00dd9d70, fn_00dd9d70(u32) -> i32),
        entry!(0x00dd9da0, fn_00dd9da0(u32, u32)),
        entry!(
            0x00dd9df0,
            hk_server_debug_display_handler_add_geometry(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9e90,
            hk_server_debug_display_handler_display_geometry_ov2(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9f40,
            hk_server_debug_display_handler_display_geometry(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dd9fe0,
            hk_server_debug_display_handler_display_triangle(u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dda0e0,
            hk_server_debug_display_handler_display_text(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dda1a0,
            hk_server_debug_display_handler_display3d_text(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dda290,
            hk_server_debug_display_handler_display_annotation(u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00dda340,
            hk_server_debug_display_handler_update_behavior(
                u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(0x00dda690, fn_00dda690(u32, u32) -> u32),
        entry!(0x00dda6b0, fn_00dda6b0(u32, u32)),
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
    fn deleting_destructor_00dda660_frees_only_when_bit_zero_is_set() {
        check(0x00dd_a660, 0x00dd_9d20);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dda650() {
        assert_eq!(check(0x00dd_a650, 0x00dd_a660, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod a07_handler_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    fn c(name: &'static str, words: &[u32]) -> (&'static str, Vec<u32>) {
        (name, words.to_vec())
    }

    const ALLOCATE: u32 = 0x007b_0100;
    const FREE_BLOCK: u32 = 0x007b_0200;
    const FREE_ARRAY: u32 = 0x007b_0300;

    struct Env {
        e: Engine,
        log: Log,
        /// The second base (`this` of the virtual functions) and the handler object (`this - 8`).
        this: u32,
        archive: u32,
        section: u32,
        ok: Rc<Cell<bool>>,
    }

    /// An engine with doubles for every function the handler calls, each logging its words
    /// (the archive word left out, after checking it); `this` has an archive and a section.
    fn env() -> Env {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let object = e.mem.alloc(0x20);
        let archive = e.mem.alloc(0x10);
        let section = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, archive);
        e.mem.set_u32(object + 0x10, section);
        let ok = Rc::new(Cell::new(true));
        for (name, slot) in [
            ("enter", ENTER_CRITICAL_SECTION),
            ("leave", LEAVE_CRITICAL_SECTION),
            ("delete", DELETE_CRITICAL_SECTION),
        ] {
            let seen = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                seen.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        for (name, function) in [
            ("write16u", ARCHIVE_WRITE_16U),
            ("write32", ARCHIVE_WRITE_32),
            ("write32u", ARCHIVE_WRITE_32U),
            ("write64u", ARCHIVE_WRITE_64U),
            ("writeFloat32", ARCHIVE_WRITE_FLOAT32),
            ("writeRaw", ARCHIVE_WRITE_RAW),
            ("command", ARCHIVE_WRITE_COMMAND),
        ] {
            let seen = Rc::clone(&log);
            e.register_double(function, move |_, a| {
                assert_eq!(a[0], archive);
                seen.borrow_mut().push((name, a[1..].to_vec()));
                Ret::default()
            });
        }
        let seen = Rc::clone(&log);
        let answer = Rc::clone(&ok);
        e.register_double(ARCHIVE_IS_OK, move |e, a| {
            assert_eq!(a[0], archive);
            seen.borrow_mut().push(("isOk", vec![]));
            e.mem.set_u8(a[1], u8::from(answer.get()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The vector writer logs the four words it is pointed at, the transform writer the first.
        let seen = Rc::clone(&log);
        e.register_double(WRITE_VECTOR, move |e, a| {
            let words = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push(("vector", words));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(WRITE_TRANSFORM, move |e, a| {
            let first = e.mem.u32(a[1]);
            seen.borrow_mut().push(("transform", vec![first]));
            Ret::default()
        });
        e.register_double(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        // An element's size is its own pointer value.
        e.register_double(ELEMENT_SIZE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(WRITE_ELEMENT, move |_, a| {
            seen.borrow_mut().push(("element", a[1..].to_vec()));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(CONVERT_TRANSFORM, move |e, a| {
            // Marks the converted transform with the first word of the record.
            let first = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], first);
            seen.borrow_mut().push(("convert", vec![first]));
            Ret::default()
        });
        // The memory router: an allocator whose virtual functions at +4, +8 and +0x10 log.
        e.put_vtable(0x007b_1000, &[0, ALLOCATE, FREE_BLOCK, 0, FREE_ARRAY]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x007b_1000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ALLOCATE, move |e, a| {
            seen.borrow_mut().push(("allocate", a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_BLOCK, move |_, a| {
            seen.borrow_mut().push(("free", a[1..].to_vec()));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_ARRAY, move |_, a| {
            seen.borrow_mut().push(("free array", a[1..].to_vec()));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            seen.borrow_mut().push(("more", a[2..].to_vec()));
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = (size * 2).max(4);
            let data = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(data + byte, value);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        Env {
            e,
            log,
            this: object + 8,
            archive,
            section,
            ok,
        }
    }

    /// The calls of a command written with an archive that is ok, between the section calls.
    fn wrapped(
        section: u32,
        inner: Vec<(&'static str, Vec<u32>)>,
    ) -> Vec<(&'static str, Vec<u32>)> {
        let mut all = vec![c("enter", &[section])];
        all.extend(inner);
        all.push(c("isOk", &[]));
        all.push(c("leave", &[section]));
        all
    }

    #[test]
    fn the_status_is_one_without_an_archive_or_when_it_is_not_ok() {
        let mut env = env();
        let this = env.this;
        let r = hk_server_debug_display_handler_hold_immediate(&mut env.e, this);
        assert_eq!(r, 0);
        env.ok.set(false);
        assert_eq!(
            hk_server_debug_display_handler_hold_immediate(&mut env.e, this),
            1
        );
        env.log.borrow_mut().clear();
        env.e.mem.set_u32(this + 4, 0);
        assert_eq!(
            hk_server_debug_display_handler_hold_immediate(&mut env.e, this),
            1
        );
        // No archive: the section is entered and left, nothing else.
        assert_eq!(
            *env.log.borrow(),
            vec![c("enter", &[env.section]), c("leave", &[env.section])]
        );
    }

    #[test]
    fn the_instance_slot_only_reports_the_archive_state() {
        let mut env = env();
        let r = hk_server_debug_display_handler_add_geometry_instance(
            &mut env.e, env.this, 1, 2, 3, 4, 5,
        );
        assert_eq!(r, 0);
        assert_eq!(*env.log.borrow(), wrapped(env.section, vec![]));
    }

    #[test]
    fn color_update_and_remove_write_their_commands() {
        let mut env = env();
        let section = env.section;
        hk_server_debug_display_handler_set_geometry_color(&mut env.e, env.this, 5, 6, 7);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[0x11]),
                    c("command", &[3]),
                    c("write32u", &[5]),
                    c("write64u", &[6, 0]),
                    c("write32", &[7]),
                ]
            )
        );
        env.log.borrow_mut().clear();
        let transform = env.e.mem.alloc(8);
        env.e.mem.set_u32(transform, 0x7777);
        hk_server_debug_display_handler_update_geometry(&mut env.e, env.this, transform, 8, 9);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[0x29]),
                    c("command", &[2]),
                    c("transform", &[0x7777]),
                    c("write64u", &[8, 0]),
                    c("write32", &[9]),
                ]
            )
        );
        env.log.borrow_mut().clear();
        hk_server_debug_display_handler_remove_geometry(&mut env.e, env.this, 10, 11, 12);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[0xd]),
                    c("command", &[4]),
                    c("write64u", &[10, 0]),
                    c("write32", &[11]),
                ]
            )
        );
    }

    fn vector(e: &mut Engine, words: [u32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (i, w) in words.into_iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, w);
        }
        at
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 8);
        e.mem.set_cstr(at, s);
        at
    }

    #[test]
    fn the_camera_writes_three_vectors_three_quiet_floats_and_the_text() {
        let mut env = env();
        let section = env.section;
        let (a, b, cc) = (
            vector(&mut env.e, [1, 2, 3, 4]),
            vector(&mut env.e, [5, 6, 7, 8]),
            vector(&mut env.e, [9, 10, 11, 12]),
        );
        let name = text(&mut env.e, b"hi");
        hk_server_debug_display_handler_update_camera(
            &mut env.e,
            env.this,
            a,
            b,
            cc,
            1.0,
            f32::from_bits(0x7f80_0001),
            3.0,
            name,
        );
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[2 + 0x33]),
                    c("command", &[0xd]),
                    c("vector", &[1, 2, 3, 4]),
                    c("vector", &[5, 6, 7, 8]),
                    c("vector", &[9, 10, 11, 12]),
                    c("writeFloat32", &[1.0f32.to_bits()]),
                    c("writeFloat32", &[0x7fc0_0001]),
                    c("writeFloat32", &[3.0f32.to_bits()]),
                    c("write16u", &[2]),
                    c("writeRaw", &[name, 2]),
                ]
            )
        );
    }

    #[test]
    fn points_lines_and_dumps_write_their_commands_with_a_sign_extended_id() {
        let mut env = env();
        let section = env.section;
        let (p, q) = (
            vector(&mut env.e, [1, 2, 3, 4]),
            vector(&mut env.e, [5, 6, 7, 8]),
        );
        hk_server_debug_display_handler_display_point(
            &mut env.e,
            env.this,
            p,
            0x10,
            0xffff_fffb,
            3,
        );
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[0x1d]),
                    c("command", &[7]),
                    c("vector", &[1, 2, 3, 4]),
                    c("write32u", &[0x10]),
                    c("write64u", &[0xffff_fffb, 0xffff_ffff]),
                    c("write32", &[3]),
                ]
            )
        );
        env.log.borrow_mut().clear();
        hk_server_debug_display_handler_display_line(&mut env.e, env.this, p, q, 0x20, 4, 5);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[0x29]),
                    c("command", &[8]),
                    c("vector", &[1, 2, 3, 4]),
                    c("vector", &[5, 6, 7, 8]),
                    c("write32u", &[0x20]),
                    c("write64u", &[4, 0]),
                    c("write32", &[5]),
                ]
            )
        );
        env.log.borrow_mut().clear();
        hk_server_debug_display_handler_send_mem_stats_dump(&mut env.e, env.this, 0x9000, 7);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[12]),
                    c("command", &[0xf]),
                    c("write32", &[7]),
                    c("writeRaw", &[0x9000, 7]),
                ]
            )
        );
    }

    #[test]
    fn a_triangle_is_written_as_three_lines() {
        let mut env = env();
        let section = env.section;
        let a = vector(&mut env.e, [1, 1, 1, 1]);
        let b = vector(&mut env.e, [2, 2, 2, 2]);
        let cc = vector(&mut env.e, [3, 3, 3, 3]);
        hk_server_debug_display_handler_display_triangle(
            &mut env.e, env.this, a, b, cc, 0x30, 6, 7,
        );
        let line = |from: u32, to: u32| {
            vec![
                c("write32u", &[0x29]),
                c("command", &[8]),
                c("vector", &[from; 4]),
                c("vector", &[to; 4]),
                c("write32u", &[0x30]),
                c("write64u", &[6, 0]),
                c("write32", &[7]),
            ]
        };
        let mut inner = line(1, 2);
        inner.extend(line(2, 3));
        inner.extend(line(3, 1));
        assert_eq!(*env.log.borrow(), wrapped(section, inner));
    }

    #[test]
    fn the_triangle_copies_sit_at_0x20_of_the_exe_frame() {
        let mut env = env();
        let a = vector(&mut env.e, [1, 1, 1, 1]);
        let b = vector(&mut env.e, [2, 2, 2, 2]);
        let cc = vector(&mut env.e, [3, 3, 3, 3]);
        // The frame is the next stack local of its size: a, b, c and a again from +0x20.
        let raw = env.e.next_stack_local(TRIANGLE_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, TRIANGLE_FRAME_BELOW);
        hk_server_debug_display_handler_display_triangle(
            &mut env.e, env.this, a, b, cc, 0x30, 6, 7,
        );
        let firsts: Vec<u32> = (0..4)
            .map(|i| env.e.mem.u32(frame + 0x20 + 0x10 * i))
            .collect();
        assert_eq!(firsts, vec![1, 2, 3, 1]);
    }

    #[test]
    fn text_commands_clamp_the_length_and_write_the_text() {
        let mut env = env();
        let section = env.section;
        let abc = text(&mut env.e, b"abc");
        hk_server_debug_display_handler_display_text(&mut env.e, env.this, abc, 1, 2, 3);
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[3 + 0x13]),
                    c("command", &[9]),
                    c("write16u", &[3]),
                    c("writeRaw", &[abc, 3]),
                    c("write32", &[1]),
                    c("write64u", &[2, 0]),
                    c("write32", &[3]),
                ]
            )
        );
        // A text of 70000 characters is written as 0xffff; 0xfffe stays, 0xffff stays.
        for (length, written) in [(70000u32, 0xffffu32), (0xfffe, 0xfffe), (0xffff, 0xffff)] {
            env.log.borrow_mut().clear();
            env.e.register_double(STRING_LENGTH, move |_, _| Ret {
                eax: length,
                ..Ret::default()
            });
            hk_server_debug_display_handler_display_annotation(&mut env.e, env.this, abc, 4, 5);
            assert_eq!(
                *env.log.borrow(),
                wrapped(
                    section,
                    vec![
                        c("write32u", &[written + 0xf]),
                        c("command", &[0x14]),
                        c("write16u", &[written]),
                        c("writeRaw", &[abc, written]),
                        c("write64u", &[4, 0]),
                        c("write32", &[5]),
                    ]
                )
            );
        }
    }

    #[test]
    fn text_in_space_adds_the_position_floats() {
        let mut env = env();
        let section = env.section;
        let abc = text(&mut env.e, b"ab");
        let position = env.e.mem.alloc(16);
        for (i, bits) in [1.0f32.to_bits(), 0x7f80_0002, 3.0f32.to_bits()]
            .into_iter()
            .enumerate()
        {
            env.e.mem.set_u32(position + 4 * i as u32, bits);
        }
        hk_server_debug_display_handler_display3d_text(
            &mut env.e, env.this, abc, position, 7, 8, 9,
        );
        assert_eq!(
            *env.log.borrow(),
            wrapped(
                section,
                vec![
                    c("write32u", &[2 + 0x1f]),
                    c("command", &[0x11]),
                    c("write16u", &[2]),
                    c("writeRaw", &[abc, 2]),
                    c("write32", &[7]),
                    c("write64u", &[8, 0]),
                    c("write32", &[9]),
                    c("writeFloat32", &[1.0f32.to_bits()]),
                    c("writeFloat32", &[0x7fc0_0002]),
                    c("writeFloat32", &[3.0f32.to_bits()]),
                ]
            )
        );
    }

    fn element_array(e: &mut Engine, elements: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * elements.len().max(1) as u32);
        for (i, v) in elements.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        let at = e.mem.alloc(16);
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, elements.len() as u32);
        e.mem.set_u32(at + 8, elements.len() as u32);
        at
    }

    #[test]
    fn the_geometry_array_size_is_four_plus_the_element_sizes_and_empty_arrays_count_four() {
        let mut env = env();
        let array = element_array(&mut env.e, &[3, 4, 5]);
        assert_eq!(env.e.call(0x00dd_9d70, &args![array]).i32(), 16);
        let empty = element_array(&mut env.e, &[]);
        assert_eq!(env.e.call(0x00dd_9d70, &args![empty]).i32(), 4);
    }

    #[test]
    fn the_geometry_array_is_written_inside_its_own_section_visit() {
        let mut env = env();
        let array = element_array(&mut env.e, &[10, 20]);
        let object = env.this - 8;
        env.e.call(0x00dd_9da0, &args![object, array]);
        assert_eq!(
            *env.log.borrow(),
            vec![
                c("enter", &[env.section]),
                c("write32", &[2]),
                c("element", &[10]),
                c("element", &[20]),
                c("leave", &[env.section]),
            ]
        );
    }

    #[test]
    fn geometry_commands_write_the_array_between_the_header_and_the_trailer() {
        let mut env = env();
        let section = env.section;
        let array = element_array(&mut env.e, &[10, 20]);
        let transform = env.e.mem.alloc(8);
        env.e.mem.set_u32(transform, 0x7777);
        // The array's size is 4 + 10 + 20 = 34.
        let nested = vec![
            c("enter", &[section]),
            c("write32", &[2]),
            c("element", &[10]),
            c("element", &[20]),
            c("leave", &[section]),
        ];
        hk_server_debug_display_handler_add_geometry(
            &mut env.e, env.this, array, transform, 5, 6, 0,
        );
        let mut expected = vec![
            c("enter", &[section]),
            c("write32", &[34 + 0x29]),
            c("command", &[1]),
        ];
        expected.extend(nested.clone());
        expected.extend([
            c("transform", &[0x7777]),
            c("write64u", &[5, 0]),
            c("write32", &[6]),
            c("isOk", &[]),
            c("leave", &[section]),
        ]);
        assert_eq!(*env.log.borrow(), expected);
        env.log.borrow_mut().clear();
        hk_server_debug_display_handler_display_geometry_ov2(
            &mut env.e,
            env.this,
            array,
            transform,
            0x40,
            0xffff_ffff,
            7,
        );
        let mut expected = vec![
            c("enter", &[section]),
            c("write32", &[34 + 0x2d]),
            c("command", &[5]),
        ];
        expected.extend(nested.clone());
        expected.extend([
            c("transform", &[0x7777]),
            c("write32u", &[0x40]),
            c("write64u", &[0xffff_ffff, 0xffff_ffff]),
            c("write32", &[7]),
            c("isOk", &[]),
            c("leave", &[section]),
        ]);
        assert_eq!(*env.log.borrow(), expected);
        env.log.borrow_mut().clear();
        hk_server_debug_display_handler_display_geometry(&mut env.e, env.this, array, 0x50, 8, 9);
        let mut expected = vec![
            c("enter", &[section]),
            c("write32", &[34 + 0x11]),
            c("command", &[6]),
        ];
        expected.extend(nested);
        expected.extend([
            c("write32u", &[0x50]),
            c("write64u", &[8, 0]),
            c("write32", &[9]),
            c("isOk", &[]),
            c("leave", &[section]),
        ]);
        assert_eq!(*env.log.borrow(), expected);
    }

    #[test]
    fn behaviour_updates_measure_then_write_every_list() {
        let mut env = env();
        let section = env.section;
        let a = element_array(&mut env.e, &[1, 2]);
        let b = element_array(&mut env.e, &[3, 4]);
        let cc = element_array(&mut env.e, &[5]);
        // One 16-byte vector.
        let d_data = vector(&mut env.e, [21, 22, 23, 24]);
        let d = env.e.mem.alloc(16);
        env.e.mem.set_u32(d, d_data);
        env.e.mem.set_u32(d + 4, 1);
        let hi = text(&mut env.e, b"hi");
        let texts = element_array(&mut env.e, &[hi, 0]);
        let f = element_array(&mut env.e, &[7]);
        let g = element_array(&mut env.e, &[]);
        let record = env.e.mem.alloc(0x30);
        env.e.mem.set_u32(record, 0x5151);
        let h = env.e.mem.alloc(16);
        env.e.mem.set_u32(h, record);
        env.e.mem.set_u32(h + 4, 1);
        hk_server_debug_display_handler_update_behavior(
            &mut env.e, env.this, a, b, cc, d, texts, f, g, h,
        );
        // The record: (7 * 1 + 2 * (2 + 2 * 1) + 1 + 0) * 4 + 0x19 + (2 + 2 for "hi") = 93.
        let lengths_free = env
            .log
            .borrow()
            .iter()
            .find(|c| c.0 == "free array")
            .cloned()
            .expect("the lengths array grew on the heap and was freed");
        assert_eq!(lengths_free.1[1], 16);
        let calls: Vec<_> = env
            .log
            .borrow()
            .iter()
            .filter(|c| c.0 != "free array" && c.0 != "more")
            .cloned()
            .collect();
        assert_eq!(
            calls,
            wrapped(
                section,
                vec![
                    c("write32u", &[93]),
                    c("command", &[0x12]),
                    c("write32u", &[2]),
                    c("write32u", &[1]),
                    c("write32u", &[3]),
                    c("write32u", &[2]),
                    c("write32u", &[4]),
                    c("write32u", &[1]),
                    c("write32u", &[5]),
                    c("vector", &[21, 22, 23, 24]),
                    c("write32u", &[2]),
                    c("write16u", &[2]),
                    c("writeRaw", &[hi, 2]),
                    c("write32u", &[1]),
                    c("convert", &[0x5151]),
                    c("transform", &[0x5151]),
                    c("write32u", &[1]),
                    c("write32u", &[7]),
                    c("write32u", &[0]),
                ]
            )
        );
    }

    #[test]
    fn behaviour_updates_keep_their_locals_at_the_exe_frame_offsets() {
        let mut env = env();
        let a = element_array(&mut env.e, &[]);
        let d = element_array(&mut env.e, &[]);
        let hi = text(&mut env.e, b"hi");
        let texts = element_array(&mut env.e, &[hi]);
        let record = env.e.mem.alloc(0x30);
        env.e.mem.set_u32(record, 0x5151);
        let h = env.e.mem.alloc(16);
        env.e.mem.set_u32(h, record);
        env.e.mem.set_u32(h + 4, 1);
        // The frame is the next stack local of its size: the lengths array at +0x24 (its
        // storage holding the length 2), the converted transform at +0x30.
        let raw = env.e.next_stack_local(UPDATE_BEHAVIOR_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, UPDATE_BEHAVIOR_FRAME_BELOW);
        hk_server_debug_display_handler_update_behavior(
            &mut env.e, env.this, a, a, a, d, texts, a, a, h,
        );
        let data = env.e.mem.u32(frame + 0x24);
        assert_ne!(data, 0);
        assert_eq!(env.e.mem.u32(data), 2);
        assert_eq!(env.e.mem.u32(frame + 0x2c), 4);
        assert_eq!(env.e.mem.u32(frame + 0x30), 0x5151);
    }

    #[test]
    fn the_constructor_builds_the_section_and_the_destructor_deletes_it() {
        let mut env = env();
        let log = Rc::clone(&env.log);
        let seen = Rc::clone(&log);
        env.e
            .register_double(CRITICAL_SECTION_CONSTRUCTOR, move |_, a| {
                seen.borrow_mut().push(("section", a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        let this = env.e.mem.alloc(0x20);
        assert_eq!(env.e.call(0x00dd_9cc0, &args![this, 0xa0a0u32]).u32(), this);
        assert_eq!(env.e.mem.u16(this + 6), 1);
        assert_eq!(env.e.mem.u32(this), HANDLER_VTABLE);
        assert_eq!(env.e.mem.u32(this + 8), HANDLER_INTERFACE_VTABLE);
        assert_eq!(env.e.mem.u32(this + 0xc), 0xa0a0);
        let section = env.e.mem.u32(this + 0x10);
        assert_ne!(section, 0);
        assert_eq!(
            *log.borrow(),
            vec![c("allocate", &[0x18]), c("section", &[section, 1000])]
        );
        log.borrow_mut().clear();
        env.e.call(0x00dd_9d20, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![c("delete", &[section]), c("free", &[section, 0x18])]
        );
        assert_eq!(env.e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(env.e.mem.u32(this + 8), DISPLAY_HANDLER_VTABLE);
        // Without a section the destructor only changes the vtables.
        log.borrow_mut().clear();
        env.e.mem.set_u32(this + 0x10, 0);
        env.e.call(0x00dd_9d20, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_constructor_that_gets_no_memory_leaves_no_section() {
        let mut env = env();
        env.e.register_double(ALLOCATE, |_, _| Ret::default());
        let this = env.e.mem.alloc(0x20);
        env.e.mem.set_u32(this + 0x10, 0x1234);
        assert_eq!(env.e.call(0x00dd_9cc0, &args![this, 1u32]).u32(), this);
        assert_eq!(env.e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn the_stream_constructor_runs_the_archive_constructor_with_a_cleared_flag_byte() {
        let mut env = env();
        let seen = Rc::clone(&env.log);
        env.e.register_double(ARCHIVE_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut().push(("archive", a.to_vec()));
            Ret::default()
        });
        // A `this` whose low byte is not 0, so that the cleared flag byte shows.
        let block = env.e.mem.alloc(0x100);
        let this = block + 0x5a;
        assert_eq!(env.e.call(0x00dd_a690, &args![this, 0x9000u32]).u32(), this);
        assert_eq!(
            *env.log.borrow(),
            vec![c("archive", &[this, 0x9000, this & 0xffff_ff00])]
        );
        assert_eq!(env.e.mem.u32(this), DISPLAY_OSTREAM_VTABLE);
    }

    #[test]
    fn three_words_are_written_one_by_one() {
        let mut env = env();
        let source = env.e.mem.alloc(16);
        for (i, v) in [7u32, 8, 9].into_iter().enumerate() {
            env.e.mem.set_u32(source + 4 * i as u32, v);
        }
        env.e.call(0x00dd_a6b0, &args![env.archive, source]);
        assert_eq!(
            *env.log.borrow(),
            vec![c("write32", &[7]), c("write32", &[8]), c("write32", &[9])]
        );
    }
}

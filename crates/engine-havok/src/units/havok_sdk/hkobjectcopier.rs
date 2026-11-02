//! `Havok SDK/hkobjectcopier.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e049d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_3160`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e049d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_3160, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkClassMember` accessors, all `this` = the member in `ECX`: `getCstyleArraySize`
/// (`00d86940`, the sign-extended word at `+0xE`), the class (`00d86920` and `00d86910`, the
/// word at `+4`), the enum (`00d86930`, the word at `+8`), `getSizeInBytes` (`00d86980`),
/// the array member size (`00d86b40`) and the enum value read at a pointer (`00d86b80`, one
/// word, `RET 4`).
const MEMBER_ARRAY_SIZE: u32 = 0x00d8_6940;
const MEMBER_CLASS: u32 = 0x00d8_6920;
const MEMBER_STRUCT_CLASS: u32 = 0x00d8_6910;
const MEMBER_ENUM: u32 = 0x00d8_6930;
const MEMBER_SIZE_IN_BYTES: u32 = 0x00d8_6980;
const MEMBER_ARRAY_MEMBER_SIZE: u32 = 0x00d8_6b40;
const MEMBER_ENUM_VALUE: u32 = 0x00d8_6b80;
/// `hkClass` accessors, `this` = the class in `ECX`: `getName` (`00d46d40`, the word at `+0`),
/// `getNumMembers` (`00d46f00`), `getMember` (`00d46f20`, one word), `getMemberByName`
/// (`00d46fe0`, one word), `getObjectSize` (`00d470d0`, the word at `+8`) and `getDefault`
/// (`00d471c0`, two words).
const CLASS_NAME: u32 = 0x00d4_6d40;
const CLASS_NUM_MEMBERS: u32 = 0x00d4_6f00;
const CLASS_MEMBER: u32 = 0x00d4_6f20;
const CLASS_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;
const CLASS_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_DEFAULT: u32 = 0x00d4_71c0;
/// `hkClassEnum`, `this` = the enum in `ECX`: `getNameOfValue` (`00db7570`, `(value, &name)`),
/// `getValueOfName` (`00db75b0`, `(name, &value)`) and `decomposeFlags` (`00db76f0`,
/// `(value, &names array, &remainder)`).
const ENUM_NAME_OF_VALUE: u32 = 0x00db_7570;
const ENUM_VALUE_OF_NAME: u32 = 0x00db_75b0;
const ENUM_DECOMPOSE_FLAGS: u32 = 0x00db_76f0;
/// `hkOArchive` pieces (`hkoarchive.cpp`): `getStreamWriter` (`00d60800`), `writeRaw`
/// (`00d607b0`), `writeArrayGeneric` (`00d60600`), `write8`/`write16`/`write32`/`write32u`
/// (`00d605b0`, `00d60900`, `00d60940`, `00d60960`), the constructor taking a writer
/// (`00d60830`), `isOk` (`00d607e0`) and the destructor (`00d60590`).
const ARCHIVE_STREAM_WRITER: u32 = 0x00d6_0800;
const ARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const ARCHIVE_WRITE_ARRAY: u32 = 0x00d6_0600;
const ARCHIVE_WRITE8: u32 = 0x00d6_05b0;
const ARCHIVE_WRITE16: u32 = 0x00d6_0900;
const ARCHIVE_WRITE32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE32U: u32 = 0x00d6_0960;
const ARCHIVE_CONSTRUCTOR: u32 = 0x00d6_0830;
const ARCHIVE_IS_OK: u32 = 0x00d6_07e0;
const ARCHIVE_DESTRUCTOR: u32 = 0x00d6_0590;
/// The relocation list's `addLocal` (`00dbd820`, `this` = the list, `(from, to)`).
const RELOCATION_ADD_LOCAL: u32 = 0x00db_d820;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size), the
/// one-more growth `00c90940` (cdecl: allocator, array, element size), `memset` (`00ec61c0`)
/// and `hkString::strLen` (`00cb10d0`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const MEMSET: u32 = 0x00ec_61c0;
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// The error-report pieces: the string stream's constructor (`00cb1600`, `this` = the 0xC-byte
/// stream, `(buffer, size)`), its text append (`00cc01c0`) and destructor (`00cc00e0`), the
/// logger object's pointer (`01268360`, its slot `0xC` reports `(level, id, text, file, line)`)
/// and the source file name (`".\Copier\hkObjectCopier.cpp"`).
const STREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const STREAM_APPEND: u32 = 0x00cc_01c0;
const STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
const LOGGER_GLOBAL: u32 = 0x0126_8360;
const COPIER_FILE_NAME: u32 = 0x010e_a3c8;
/// The vtable `hkObjectCopier`'s constructor installs and the base vtable its destructor
/// leaves.
const OBJECT_COPIER_VTABLE: u32 = 0x010e_a3b0;
const BASE_VTABLE: u32 = 0x0102_e388;
/// An address in the exe's data that `saveExtras` passes as the class argument of `addGlobal`
/// for the second pointer of a pair; what it holds is unconfirmed.
const SHARED_ARGUMENT_0126CAE8: u32 = 0x0126_cae8;

/// The message texts of the error reports.
const TEXT_UNKNOWN_POD_ARRAY: u32 = 0x010e_a3e4;
const TEXT_UNKNOWN_TYPE: u32 = 0x010e_a424;
const TEXT_UNKNOWN_MEMBER: u32 = 0x010e_a448;
const TEXT_NO_CLASS_PREFIX: u32 = 0x010e_a47c;
const TEXT_NO_CLASS_MIDDLE: u32 = 0x010e_a4ac;
const TEXT_NO_CLASS_SUFFIX: u32 = 0x0104_4c28;
const TEXT_TYPE_ZERO: u32 = 0x010e_a4b0;

/// The allocator object: the word at `+0x10` of the memory router's structure.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room for one more element of `element_size` bytes in the array (data at `+0`, size at
/// `+4`, capacity at `+8` with flag bits in the top two): when the size equals the capacity,
/// calls `00c90940`.
fn grow_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

/// The error report the copier makes: builds a 0x200-byte string stream, appends the texts
/// (each call's result is the stream for the next), reports `(level, id, buffer, file, line)`
/// through the logger and destroys the stream. With `trap` the exe executes `INT3` at that
/// address when the report returns nonzero: that panics here.
fn report_error(
    e: &mut Engine,
    texts: impl FnOnce(&mut Engine) -> Vec<u32>,
    level: u32,
    id: u32,
    line: u32,
    trap: Option<u32>,
) {
    e.with_stack(0x20c, |e, stream| {
        let buffer = stream.addr() + 0xc;
        e.call(STREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
        let mut current = stream.addr();
        for text in texts(e) {
            current = e.call(STREAM_APPEND, &args![current, text]).u32();
        }
        let logger = e.mem.u32(LOGGER_GLOBAL);
        let result = e
            .vcall(
                logger,
                0xc,
                &args![level, id, buffer, COPIER_FILE_NAME, line],
            )
            .u32();
        if let (Some(address), true) = (trap, result != 0) {
            panic!("INT3 executed at {address:08x}");
        }
        e.call(STREAM_DESTRUCTOR, &args![stream]);
    });
}

/// A scratch buffer of `size` zero bytes from the router's temporary stack, which the
/// exe builds the same way in `objectCopierPadUp` and `writeZero`: an array record
/// (data, size, capacity `size | 0x80000000`, saved top) on the stack, the router's top
/// word advanced by `size` rounded up to 128, a `_reserve` when the capacity is below `size`,
/// `memset` of the missing bytes, `body` with the data address, then the top word put
/// back and the storage given back when it is not in place.
fn with_temp_zero_buffer(e: &mut Engine, size: u32, body: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(0x10, |e, record| {
        let record = record.addr();
        e.mem.set_u32(record, 0);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        e.mem.set_u32(
            router,
            (size.wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top),
        );
        e.mem.set_u32(record, top);
        e.mem.set_u32(record + 8, size | 0x8000_0000);
        e.mem.set_u32(record + 0xc, top);
        let allocator = heap_allocator(e);
        let capacity = (e.mem.u32(record + 8) & 0x3fff_ffff) as i32;
        if capacity < size as i32 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (size as i32) < doubled {
                doubled
            } else {
                size as i32
            };
            e.call(ARRAY_RESERVE, &args![allocator, record, wanted, 1u32]);
        }
        let used = e.mem.u32(record + 4);
        let data = e.mem.u32(record);
        let missing = (size as i32).wrapping_sub(used as i32);
        if missing > 0 {
            e.call(MEMSET, &args![data.wrapping_add(used), 0u32, missing]);
        }
        e.mem.set_u32(record + 4, size);
        let data = e.mem.u32(record);
        body(e, data);
        let saved = e.mem.u32(record + 0xc);
        if saved == e.mem.u32(record) {
            e.mem.set_u32(record + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(record + 8);
        e.mem.set_u32(record + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(record);
            e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
        }
    });
}

// Translated from 00e03100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `objectCopier_calcCArraySize` (Xbox PDB), `ESI` = the member: its C-style array size
/// (`getCstyleArraySize`, `00d86940`, called twice as the exe does) or 1 when that is 0.
pub fn calc_c_array_size(e: &mut Engine, member: Ptr) -> i32 {
    if e.call(MEMBER_ARRAY_SIZE, &args![member]).i32() != 0 {
        return e.call(MEMBER_ARRAY_SIZE, &args![member]).i32();
    }
    1
}

// Translated from 00e03120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the constructor of the `hkObjectCopier` class (the class is
/// not in the Xbox PDB; its destructor is `00e03160`): reference count 1 (`+6`), vtable
/// `0x010ea3b0`, the words `*first` at `+8` and `*second` at `+0xC` (the layout rules), `flags`
/// at `+0x10` and at `+0x14` the byte that tells whether the two rules' second bytes (`+9`
/// and `+0xD`) differ. Returns `this`.
pub fn fn_00e03120(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, flags: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OBJECT_COPIER_VTABLE);
    let layout_in = e.mem.u32(first.addr());
    e.mem.set_u32(this.addr() + 8, layout_in);
    let layout_out = e.mem.u32(second.addr());
    e.mem.set_u32(this.addr() + 0xc, layout_out);
    let differ = e.mem.u8(this.addr() + 9) != e.mem.u8(this.addr() + 0xd);
    e.mem.set_u32(this.addr() + 0x10, flags);
    e.mem.set_u8(this.addr() + 0x14, u8::from(differ));
    this
}

// Translated from 00e03160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectCopier::~hkObjectCopier` (Xbox PDB): leaves the base vtable `0x0102e388`.
pub fn hk_object_copier_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00e03170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `writePodArray` (Xbox PDB), registers first: `EAX` = the member type, `ECX` = the element
/// count, `EDX` = the bytes per element; then the stack words `(archive, data)` (the caller
/// removes them). Types 1 to 0xb and 0x20 write `count` elements of `element_size` bytes through
/// `writeArrayGeneric`; types 0xc to 0x12 write `count * (element_size / 4)` elements of 4
/// bytes; any other type reports "Unknown class member found during write of plain data
/// array." (id `0x747e1e03`, line 0xfb) and executes `INT3` (at `00e031fa`) when the report
/// returns nonzero.
pub fn write_pod_array(
    e: &mut Engine,
    member_type: u32,
    count: i32,
    element_size: u32,
    archive: Ptr,
    data: Ptr,
) {
    let (size, count) = match member_type {
        0x0c..=0x12 => (4, count.wrapping_mul((element_size >> 2) as i32)),
        1..=0x0b | 0x20 => (element_size, count),
        _ => {
            report_error(
                e,
                |_| vec![TEXT_UNKNOWN_POD_ARRAY],
                3,
                0x747e_1e03,
                0xfb,
                Some(0x00e0_31fa),
            );
            return;
        }
    };
    e.call(ARCHIVE_WRITE_ARRAY, &args![archive, data, size, count]);
}

// Translated from 00e03240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `applyDefaultsFor` (Xbox PDB), cdecl: the default of member `index` of `class` for the
/// stream (`hkClass::getDefault`, `00d471c0`); for a struct member (type 0x19) with a class,
/// the defaults of every member of that class, as many times as the member's array size, and
/// the result is 0 when any of them is 0.
pub fn apply_defaults_for(e: &mut Engine, class: Ptr, member: Ptr, index: u32, stream: u32) -> u32 {
    let mut result = e.call(CLASS_DEFAULT, &args![class, index, stream]).u32();
    if e.mem.u8(member.addr() + 0xc) == 0x19 {
        let member_class = e.call(MEMBER_CLASS, &args![member]).u32();
        if member_class != 0 {
            let mut repeat = calc_c_array_size(e, member);
            if repeat > 0 {
                loop {
                    let mut i: i32 = 0;
                    if e.call(CLASS_NUM_MEMBERS, &args![member_class]).i32() > 0 {
                        loop {
                            let inner = e.call(CLASS_MEMBER, &args![member_class, i]).u32();
                            if apply_defaults_for(
                                e,
                                Ptr::new(member_class),
                                Ptr::new(inner),
                                i as u32,
                                stream,
                            ) == 0
                            {
                                result = 0;
                            }
                            i += 1;
                            if i >= e.call(CLASS_NUM_MEMBERS, &args![member_class]).i32() {
                                break;
                            }
                        }
                    }
                    repeat -= 1;
                    if repeat == 0 {
                        break;
                    }
                }
            }
        }
    }
    result
}

// Translated from 00e032d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectCopier::areMembersCompatible` (Xbox PDB): 1 when the two members have the same
/// type byte (`+0xC`) and the same subtype byte (`+0xD`) or are of type 0x18, else 0. `this`
/// (the first word) is not read.
pub fn are_members_compatible(e: &mut Engine, _unused_0: Ptr, first: Ptr, second: Ptr) -> u32 {
    let ty = e.mem.u8(first.addr() + 0xc);
    let same_type = ty == e.mem.u8(second.addr() + 0xc);
    let same_subtype = e.mem.u8(first.addr() + 0xd) == e.mem.u8(second.addr() + 0xd);
    u32::from(same_type && (same_subtype || ty == 0x18))
}

// Translated from 00e03300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `writeUlongArray` (Xbox PDB), registers first: `EBX` = the 4-byte source elements, `EDI` =
/// the count; then the stack words `(archive, size)`. With `size` 4 the array is written as it
/// is; otherwise each element is widened to 8 bytes (the high word 0) in a block taken from the
/// router's temporary stack (its top word advanced by `count * 8 + 0x7f` rounded down to a
/// multiple of 128 and put back afterwards) and that is written with the element size
/// `size`.
pub fn write_ulong_array(e: &mut Engine, source: Ptr, count: i32, archive: Ptr, size: i32) {
    if size == 4 {
        e.call(ARCHIVE_WRITE_ARRAY, &args![archive, source, size, count]);
        return;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let rounded = (count as u32).wrapping_mul(8).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(top));
    for i in 0..count.max(0) as u32 {
        let value = e.mem.u32(source.addr().wrapping_add(i.wrapping_mul(4)));
        e.mem.set_u32(top.wrapping_add(i.wrapping_mul(8)), value);
        e.mem.set_u32(top.wrapping_add(i.wrapping_mul(8)) + 4, 0);
    }
    e.call(ARCHIVE_WRITE_ARRAY, &args![archive, top, size, count]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
}

// Translated from 00e03360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `objectCopierPadUp` (Xbox PDB), registers: `EAX` = the alignment, `EDI` = the stream writer.
/// Reads the writer's position (slot `0x20`), and when it is not a multiple of `alignment`
/// writes the zero bytes up to the next one through slot `0x10` from a zero-filled
/// scratch buffer ([`with_temp_zero_buffer`]). It returns nothing.
pub fn pad_up(e: &mut Engine, alignment: u32, writer: Ptr) {
    let position = e.vcall(writer.addr(), 0x20, &args![]).u32();
    with_temp_zero_buffer(e, alignment, |e, data| {
        let misalignment = alignment.wrapping_sub(1) & position;
        if misalignment != 0 {
            e.vcall(
                writer.addr(),
                0x10,
                &args![data, alignment.wrapping_sub(misalignment)],
            );
        }
    });
}

// Translated from 00e03460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectCopier::writeZero` (Xbox PDB), `this` = the copier, `(archive, member)`: writes the
/// zero bytes of one member of the output class. The size is `getSizeInBytes` for types 1 to
/// 0x12, 0x18, 0x1f and 0x20; the array size times the pointer size (the byte at `this + 0xC`)
/// for 0x14, 0x15 and 0x1e; for 0x19 the object size of the struct's class times the array
/// size; for 0x1c twice the array size times the pointer size; for 0x16, 0x17, 0x1a and 0x1b
/// the pointer size plus 4 (plus another pointer size for 0x1b). Type 0x16 writes that many
/// bytes of a 12-byte zeroed stack buffer (the model reads zeros past it) and then
/// `write32u(0x80000000)` and returns; the others write the zero bytes of a scratch buffer
/// ([`with_temp_zero_buffer`]). Any other type reports "Unknown class member type found!"
/// (id `0x5ef4e5a4`, line 0xa8), executes `INT3` (at `00e03586`) when the report returns
/// nonzero, and writes no bytes.
pub fn write_zero(e: &mut Engine, this: Ptr, archive: Ptr, member: Ptr) {
    let ty = e.mem.u8(member.addr() + 0xc);
    let pointer_size = u32::from(e.mem.u8(this.addr() + 0xc));
    let mut size: u32 = 0;
    match ty {
        1..=0x12 | 0x18 | 0x1f | 0x20 => {
            size = e.call(MEMBER_SIZE_IN_BYTES, &args![member]).u32();
        }
        0x14 | 0x15 | 0x1e => {
            size = (calc_c_array_size(e, member) as u32).wrapping_mul(pointer_size);
        }
        0x16 | 0x17 | 0x1a | 0x1b => {
            if ty == 0x1b {
                size = pointer_size;
            }
            size = size.wrapping_add(pointer_size).wrapping_add(4);
            if ty == 0x16 {
                e.with_stack(0x200, |e, zeros| {
                    e.call(ARCHIVE_WRITE_RAW, &args![archive, zeros, size]);
                });
                e.call(ARCHIVE_WRITE32U, &args![archive, 0x8000_0000u32]);
                return;
            }
        }
        0x19 => {
            let class = e.call(MEMBER_STRUCT_CLASS, &args![member]).u32();
            let object_size = e.call(CLASS_OBJECT_SIZE, &args![class]).u32();
            size = object_size.wrapping_mul(calc_c_array_size(e, member) as u32);
        }
        0x1c => {
            size = (calc_c_array_size(e, member) as u32).wrapping_mul(pointer_size);
            size = size.wrapping_add(size);
        }
        _ => {
            report_error(
                e,
                |_| vec![TEXT_UNKNOWN_TYPE],
                3,
                0x5ef4_e5a4,
                0xa8,
                Some(0x00e0_3586),
            );
        }
    }
    with_temp_zero_buffer(e, size, |e, data| {
        e.call(ARCHIVE_WRITE_RAW, &args![archive, data, size]);
    });
}

// Translated from 00e049c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map, cdecl `(a, b)`: the smaller of two signed numbers (`b` unless
/// `a < b`).
pub fn fn_00e049c0(_e: &mut Engine, a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// Translated from 00e04a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRelocationInfo::addGlobal` (Xbox PDB), `this` = the list, `(a, b, c, d)`: appends the
/// four words as a 0x10-byte record to the array at `this + 0xC` (data, size at `+0x10`,
/// capacity at `+0x14`), making room with `00c90940` when it is full.
pub fn hk_relocation_info_add_global(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    let array = this.addr() + 0xc;
    grow_if_full(e, array, 0x10);
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size << 4);
    e.mem.set_u32(slot, a);
    e.mem.set_u32(slot + 4, b);
    e.mem.set_u32(slot + 8, c);
    e.mem.set_u32(slot + 0xc, d);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00e03e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `saveCstring` (Xbox PDB), registers first: `EBX` = the archive, `ESI` = the array of 8-byte
/// records; then the stack words `(text, offset)` (the caller removes them). Pads the writer to
/// 2 bytes ([`pad_up`]), appends the record `(offset, position)` to the array (making room
/// with `00c90940`) and writes the text with its terminating zero through `writeRaw`. Returns
/// 0.
pub fn save_cstring(e: &mut Engine, archive: Ptr, array: Ptr, text: Ptr, offset: u32) -> u32 {
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    pad_up(e, 2, Ptr::new(writer));
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    let position = e.vcall(writer, 0x20, &args![]).u32();
    grow_if_full(e, array.addr(), 8);
    let size = e.mem.u32(array.addr() + 4);
    let slot = e.mem.u32(array.addr()).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(slot, offset);
    e.mem.set_u32(slot + 4, position);
    let size = e.mem.u32(array.addr() + 4);
    e.mem.set_u32(array.addr() + 4, size.wrapping_add(1));
    let length = e.call(STRING_LENGTH, &args![text]).u32();
    e.call(
        ARCHIVE_WRITE_RAW,
        &args![archive, text, length.wrapping_add(1)],
    );
    0
}

/// `write8`, `write16` or `write32` through the archive by the element size (any other size
/// writes nothing).
fn write_by_size(e: &mut Engine, archive: Ptr, element_size: i32, value: u32) {
    match element_size {
        1 => e.call(ARCHIVE_WRITE8, &args![archive, value]),
        2 => e.call(ARCHIVE_WRITE16, &args![archive, value]),
        4 => e.call(ARCHIVE_WRITE32, &args![archive, value]),
        _ => return,
    };
}

/// The scratch frame of `saveBody`: the names array of `decomposeFlags` (data, size,
/// capacity), the two remainders, the name and value slots of the enum lookups, the slot the
/// flag lookups leave their value in (the exe never initialises it; see `saveBody`) and the
/// zero buffers the pointer members write.
const BODY_NAMES: u32 = 0x00;
const BODY_REMAINDER: u32 = 0x10;
const BODY_SECOND_REMAINDER: u32 = 0x14;
const BODY_ENUM_NAME: u32 = 0x18;
const BODY_ENUM_VALUE: u32 = 0x1c;
const BODY_FLAG_VALUE: u32 = 0x20;
const BODY_ZERO_FIRST: u32 = 0x30;
const BODY_ZERO_SECOND: u32 = 0x50;
const BODY_FRAME_SIZE: u32 = 0x80;

/// The members one `saveBody` pass writes: `index` of the output class's members, the member
/// pair and the data. Everything it needs is in this record.
struct BodyPass {
    this: Ptr,
    class_in: Ptr,
    archive: Ptr,
    class_out: Ptr,
    frame: u32,
}

/// Writes one member of `saveBody` that has a counterpart in the input class and is
/// compatible with it: `member` (output class) and `member_in` (input class), whose data is at
/// `in_data`.
fn save_body_member(e: &mut Engine, pass: &BodyPass, member: u32, member_in: u32, in_data: u32) {
    let (this, archive, frame) = (pass.this, pass.archive, pass.frame);
    let ty = e.mem.u8(member + 0xc);
    let pointer_size = u32::from(e.mem.u8(this.addr() + 0xc));
    match ty {
        1..=0x12 | 0x20 => {
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let size = e.call(MEMBER_SIZE_IN_BYTES, &args![member]).i32();
            let count = fn_00e049c0(e, count_in, count_out);
            let element_size = size / count_out;
            write_pod_array(
                e,
                u32::from(ty),
                count,
                element_size as u32,
                archive,
                Ptr::new(in_data),
            );
        }
        0x13 => {}
        0x14 | 0x15 | 0x1d | 0x21 => {
            let mut remaining = calc_c_array_size(e, Ptr::new(member));
            e.mem.set_u32(frame + BODY_ZERO_FIRST, 0);
            e.mem.set_u32(frame + BODY_ZERO_FIRST + 4, 0);
            if remaining > 0 {
                while remaining != 0 {
                    e.call(
                        ARCHIVE_WRITE_RAW,
                        &args![archive, frame + BODY_ZERO_FIRST, pointer_size],
                    );
                    remaining -= 1;
                }
            }
        }
        0x16 | 0x1a | 0x1b => {
            e.mem.set_u32(frame + BODY_ZERO_FIRST, 0);
            e.mem.set_u32(frame + BODY_ZERO_FIRST + 4, 0);
            let value = if ty == 0x1b {
                e.call(
                    ARCHIVE_WRITE_ARRAY,
                    &args![archive, frame + BODY_ZERO_FIRST, pointer_size, 1u32],
                );
                if e.mem.u32(in_data) != 0 {
                    e.mem.u32(in_data + 8)
                } else {
                    let class_in = pass.class_in;
                    report_error(
                        e,
                        |e| {
                            let name = e.mem.u32(member_in);
                            let class_name = e.call(CLASS_NAME, &args![class_in]).u32();
                            vec![
                                TEXT_NO_CLASS_PREFIX,
                                class_name,
                                TEXT_NO_CLASS_MIDDLE,
                                name,
                                TEXT_NO_CLASS_SUFFIX,
                            ]
                        },
                        1,
                        0xabba_5a55,
                        0x1b8,
                        None,
                    );
                    0
                }
            } else if e.mem.u8(member + 0xd) == 0 {
                0
            } else {
                e.mem.u32(in_data + 4)
            };
            e.mem.set_u32(frame + BODY_ZERO_SECOND, 0);
            e.mem.set_u32(frame + BODY_ZERO_SECOND + 4, 0);
            e.call(
                ARCHIVE_WRITE_ARRAY,
                &args![archive, frame + BODY_ZERO_SECOND, pointer_size, 1u32],
            );
            e.call(ARCHIVE_WRITE32, &args![archive, value]);
            if ty == 0x16 {
                e.call(ARCHIVE_WRITE32, &args![archive, value | 0x8000_0000]);
            }
        }
        0x18 => {
            if e.mem.u32(member_in + 8) == 0 || e.mem.u32(member + 8) == 0 {
                return;
            }
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let mut remaining = fn_00e049c0(e, count_in, count_out);
            let enum_in = e.call(MEMBER_ENUM, &args![member_in]).u32();
            let enum_out = e.call(MEMBER_ENUM, &args![member]).u32();
            let size = e.call(MEMBER_SIZE_IN_BYTES, &args![member_in]).i32();
            let element_size = size / count_in;
            if remaining <= 0 {
                return;
            }
            let mut pointer = in_data;
            while remaining != 0 {
                let value = e.call(MEMBER_ENUM_VALUE, &args![member_in, pointer]).u32();
                e.mem.set_u32(frame + BODY_ENUM_VALUE, 0);
                let found = e
                    .call(
                        ENUM_NAME_OF_VALUE,
                        &args![enum_in, value, frame + BODY_ENUM_NAME],
                    )
                    .u32();
                if found == 0 {
                    let name = e.mem.u32(frame + BODY_ENUM_NAME);
                    e.call(
                        ENUM_VALUE_OF_NAME,
                        &args![enum_out, name, frame + BODY_ENUM_VALUE],
                    );
                }
                let written = e.mem.u32(frame + BODY_ENUM_VALUE);
                write_by_size(e, archive, element_size, written);
                pointer = pointer.wrapping_add(element_size as u32);
                remaining -= 1;
            }
        }
        0x19 => {
            let class_in_member = e.call(MEMBER_CLASS, &args![member_in]).u32();
            let class_out_member = e.call(MEMBER_CLASS, &args![member]).u32();
            if class_in_member == 0 || class_out_member == 0 {
                return;
            }
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count = fn_00e049c0(e, count_in, count_out);
            let mut i: i32 = 0;
            while i < count {
                let size = e.call(CLASS_OBJECT_SIZE, &args![class_in_member]).u32();
                save_body(
                    e,
                    this,
                    Ptr::new(size.wrapping_mul(i as u32).wrapping_add(in_data)),
                    Ptr::new(class_in_member),
                    archive,
                    Ptr::new(class_out_member),
                );
                i += 1;
            }
        }
        0x1c => {
            for lane in 0..4 {
                e.mem.set_u32(frame + BODY_ZERO_FIRST + 4 * lane, 0);
            }
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let mut remaining = fn_00e049c0(e, count_in, count_out);
            while remaining > 0 {
                e.call(
                    ARCHIVE_WRITE_ARRAY,
                    &args![archive, frame + BODY_ZERO_FIRST, pointer_size, 2u32],
                );
                remaining -= 1;
            }
        }
        0x1e => {
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let count = fn_00e049c0(e, count_in, count_out);
            write_ulong_array(e, Ptr::new(in_data), count, archive, pointer_size as i32);
        }
        0x1f => save_body_flags(e, pass, member, member_in, in_data),
        _ => {
            report_error(
                e,
                |_| vec![TEXT_UNKNOWN_MEMBER],
                3,
                0x641e_3e03,
                0x225,
                Some(0x00e0_3d4d),
            );
        }
    }
}

/// The flags member (type 0x1f) of `saveBody`: for each element the input value is split into
/// flag names by the input enum (`decomposeFlags`), each name is looked up in the output enum
/// (found: its value joins the result, else it joins the remainder), a nonzero remainder is
/// decomposed by the output enum and its value joins the result, and the result is written by
/// the element size. The value slot of the name lookups is never initialised in the exe:
/// it keeps what the last successful lookup left (see `saveBody`).
fn save_body_flags(e: &mut Engine, pass: &BodyPass, member: u32, member_in: u32, in_data: u32) {
    let (archive, frame) = (pass.archive, pass.frame);
    if e.mem.u32(member_in + 8) == 0 || e.mem.u32(member + 8) == 0 {
        return;
    }
    let count_in = calc_c_array_size(e, Ptr::new(member_in));
    let count_out = calc_c_array_size(e, Ptr::new(member));
    let mut remaining = fn_00e049c0(e, count_in, count_out);
    let enum_in = e.call(MEMBER_ENUM, &args![member_in]).u32();
    let enum_out = e.call(MEMBER_ENUM, &args![member]).u32();
    let size = e.call(MEMBER_SIZE_IN_BYTES, &args![member_in]).i32();
    let element_size = size / count_in;
    if remaining <= 0 {
        return;
    }
    let names = frame + BODY_NAMES;
    let mut pointer = in_data;
    while remaining != 0 {
        let value = e.call(MEMBER_ENUM_VALUE, &args![member_in, pointer]).u32();
        e.mem.set_u32(names, 0);
        e.mem.set_u32(names + 4, 0);
        e.mem.set_u32(names + 8, 0x8000_0000);
        e.mem.set_u32(frame + BODY_REMAINDER, 0);
        e.call(
            ENUM_DECOMPOSE_FLAGS,
            &args![enum_in, value, names, frame + BODY_REMAINDER],
        );
        let mut result: u32 = 0;
        let mut i: i32 = 0;
        if e.mem.u32(names + 4) as i32 > 0 {
            loop {
                let list = e.mem.u32(names);
                let name = e.mem.u32(list.wrapping_add(4 * i as u32));
                let found = e
                    .call(
                        ENUM_VALUE_OF_NAME,
                        &args![enum_out, name, frame + BODY_FLAG_VALUE],
                    )
                    .u32();
                let looked_up = e.mem.u32(frame + BODY_FLAG_VALUE);
                if found == 0 {
                    result |= looked_up;
                } else {
                    let remainder = e.mem.u32(frame + BODY_REMAINDER);
                    e.mem.set_u32(frame + BODY_REMAINDER, remainder | looked_up);
                }
                i += 1;
                if i >= e.mem.u32(names + 4) as i32 {
                    break;
                }
            }
        }
        let remainder = e.mem.u32(frame + BODY_REMAINDER);
        if remainder != 0 {
            e.mem.set_u32(frame + BODY_SECOND_REMAINDER, 0);
            e.call(
                ENUM_DECOMPOSE_FLAGS,
                &args![enum_out, remainder, names, frame + BODY_SECOND_REMAINDER],
            );
            result |= e.mem.u32(frame + BODY_SECOND_REMAINDER);
        }
        write_by_size(e, archive, element_size, result);
        pointer = pointer.wrapping_add(element_size as u32);
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(names + 8);
        e.mem.set_u32(names + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(names);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        remaining -= 1;
        e.mem.set_u32(names, 0);
        e.mem.set_u32(names + 8, 0x8000_0000);
    }
}

// Translated from 00e036b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e036b0 - flags member (type 0x1f): the value slot `getValueOfName` writes only on success (the stack slot at frame offset 0x30) is read uninitialised when the first flag name is not found in the output enum; the model starts it at 0.
/// `hkObjectCopier::saveBody` (Xbox PDB), `this` = the copier, `(data_in, class_in, archive,
/// class_out)`: writes the data of one object of `class_out` laid out as that class. For every
/// member of `class_out` it seeks the writer to the object's start plus the member's offset
/// (`+0x12`), then: a member with the flag `0x400` is zero-filled ([`write_zero`]) when the
/// copier's flags (`+0x10`) have bit 1, takes its default ([`apply_defaults_for`]) when they
/// have bit 0, and is otherwise treated like the others; a member whose name is not in
/// `class_in` (`getMemberByName`) or is not compatible with it (virtual slot `0x10` of the
/// copier) takes its default; the rest are written by type: plain data and enums by element,
/// pointers as zero words, arrays as their size and capacity words, structs by recursion,
/// flags by name. At the end the writer is seeked to the object's start plus the class's
/// object size. Returns the writer's position at the start.
///
/// The data of the frame (names array, remainders, name and value slots, zero buffers) is one
/// stack block. The one uninitialised read is described above.
pub fn save_body(
    e: &mut Engine,
    this: Ptr,
    data_in: Ptr,
    class_in: Ptr,
    archive: Ptr,
    class_out: Ptr,
) -> u32 {
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    let start = e.vcall(writer, 0x20, &args![]).u32();
    e.with_stack(BODY_FRAME_SIZE, |e, frame| {
        let pass = BodyPass {
            this,
            class_in,
            archive,
            class_out,
            frame: frame.addr(),
        };
        let mut index: i32 = 0;
        if e.call(CLASS_NUM_MEMBERS, &args![class_out]).i32() > 0 {
            loop {
                let member = e.call(CLASS_MEMBER, &args![class_out, index]).u32();
                let offset = u32::from(e.mem.u16(member + 0x12)).wrapping_add(start);
                e.vcall(writer, 0x1c, &args![offset, 0u32]);
                save_body_one(e, &pass, writer, index, member, data_in.addr());
                index += 1;
                if index >= e.call(CLASS_NUM_MEMBERS, &args![class_out]).i32() {
                    break;
                }
            }
        }
    });
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    let object_size = e.call(CLASS_OBJECT_SIZE, &args![class_out]).u32();
    e.vcall(writer, 0x1c, &args![object_size.wrapping_add(start), 0u32]);
    start
}

/// One member of the loop of [`save_body`], after the seek.
fn save_body_one(
    e: &mut Engine,
    pass: &BodyPass,
    writer: u32,
    index: i32,
    member: u32,
    data_in: u32,
) {
    let (this, class_in, class_out) = (pass.this, pass.class_in, pass.class_out);
    if e.mem.u16(member + 0x10) & 0x400 != 0 {
        let flags = e.mem.u32(this.addr() + 0x10);
        if flags & 2 != 0 {
            write_zero(e, this, pass.archive, Ptr::new(member));
            return;
        }
        if flags & 1 != 0 {
            apply_defaults_for(e, class_out, Ptr::new(member), index as u32, writer);
            return;
        }
    }
    let name = e.mem.u32(member);
    let member_in = e.call(CLASS_MEMBER_BY_NAME, &args![class_in, name]).u32();
    if member_in == 0 || e.vcall(this.addr(), 0x10, &args![member_in, member]).u32() == 0 {
        apply_defaults_for(e, class_out, Ptr::new(member), index as u32, writer);
        return;
    }
    let in_data = u32::from(e.mem.u16(member_in + 0x12)).wrapping_add(data_in);
    save_body_member(e, pass, member, member_in, in_data);
}

/// The scratch of `saveExtras`: zero buffers for pointer words.
const EXTRAS_ZERO: u32 = 0x00;
const EXTRAS_FRAME_SIZE: u32 = 0x40;

/// The arguments `saveExtras` passes down to itself and keeps for the members.
struct ExtrasPass {
    this: Ptr,
    archive: Ptr,
    base: u32,
    reloc: Ptr,
    depth: u32,
    frame: u32,
}

/// `hkRelocationInfo::addLocal` (`00dbd820`).
fn add_local(e: &mut Engine, reloc: Ptr, from: u32, to: u32) {
    e.call(RELOCATION_ADD_LOCAL, &args![reloc, from, to]);
}

/// The position of the archive's writer (`getStreamWriter`, then slot `0x20`).
fn archive_position(e: &mut Engine, archive: Ptr) -> u32 {
    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
    e.vcall(writer, 0x20, &args![]).u32()
}

// Translated from 00e03ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectCopier::saveExtras` (Xbox PDB), `this` = the copier, `(data, class_in, archive,
/// class_out, base, reloc, depth)`: the second pass over an object, which writes what the
/// members point to and records the pointers in the relocation list `reloc`. At depth 0 the
/// record `(base, name of class_out)` is first appended to the list's array at `+0x18`. For
/// every member of `class_out` that is in `class_in`, compatible with it (virtual slot `0x10`)
/// and not skipped by the copier's flag 2 together with the member flag `0x400`, it handles the
/// member by type (C strings by `saveCstring`, pointers and arrays by `addGlobal` and
/// `addLocal` records with the pointed-to data written by [`save_body`] and recursion, structs
/// by recursion; plain data, enums and flags need nothing; type 0x13 and unknown types report an
/// error and execute `INT3` (at `00e04834` and `00e04883`) when the report returns nonzero),
/// then pads the writer to 16 bytes ([`pad_up`]).
#[allow(clippy::too_many_arguments)]
pub fn save_extras(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    class_in: Ptr,
    archive: Ptr,
    class_out: Ptr,
    base: u32,
    reloc: Ptr,
    depth: u32,
) {
    if depth == 0 {
        let name = e.call(CLASS_NAME, &args![class_out]).u32();
        let array = reloc.addr() + 0x18;
        grow_if_full(e, array, 8);
        let size = e.mem.u32(array + 4);
        let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
        e.mem.set_u32(slot, base);
        e.mem.set_u32(slot + 4, name);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
    e.with_stack(EXTRAS_FRAME_SIZE, |e, frame| {
        let pass = ExtrasPass {
            this,
            archive,
            base,
            reloc,
            depth: depth.wrapping_add(1),
            frame: frame.addr(),
        };
        let mut index: i32 = 0;
        if e.call(CLASS_NUM_MEMBERS, &args![class_out]).i32() <= 0 {
            return;
        }
        loop {
            let member = e.call(CLASS_MEMBER, &args![class_out, index]).u32();
            let name = e.mem.u32(member);
            let member_in = e.call(CLASS_MEMBER_BY_NAME, &args![class_in, name]).u32();
            if member_in != 0 {
                let in_data = u32::from(e.mem.u16(member_in + 0x12)).wrapping_add(data.addr());
                let compatible = e.vcall(this.addr(), 0x10, &args![member_in, member]).u32();
                let skipped =
                    e.mem.u8(this.addr() + 0x10) & 2 != 0 && e.mem.u16(member + 0x10) & 0x400 != 0;
                if compatible != 0 && !skipped {
                    save_extras_member(e, &pass, member, member_in, in_data);
                    let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
                    pad_up(e, 0x10, Ptr::new(writer));
                }
            }
            index += 1;
            if index >= e.call(CLASS_NUM_MEMBERS, &args![class_out]).i32() {
                break;
            }
        }
    });
}

/// One member of [`save_extras`] by type (before the padding).
fn save_extras_member(
    e: &mut Engine,
    pass: &ExtrasPass,
    member: u32,
    member_in: u32,
    in_data: u32,
) {
    let (this, archive, base, reloc, depth) =
        (pass.this, pass.archive, pass.base, pass.reloc, pass.depth);
    let ty = e.mem.u8(member + 0xc);
    let member_offset = u32::from(e.mem.u16(member + 0x12));
    let pointer_out = u32::from(e.mem.u8(this.addr() + 0xc));
    match ty {
        0x1d => {
            let text = e.mem.u32(in_data);
            if text != 0 {
                save_cstring(
                    e,
                    archive,
                    reloc,
                    Ptr::new(text),
                    member_offset.wrapping_add(base),
                );
            }
        }
        0x21 => {
            let text = e.mem.u32(in_data) & 0xffff_fffe;
            if text != 0 {
                save_cstring(
                    e,
                    archive,
                    reloc,
                    Ptr::new(text),
                    member_offset.wrapping_add(base),
                );
            }
        }
        0x14 => save_extras_pointer_array(e, pass, member, member_in, in_data),
        0x16 | 0x17 | 0x1a => save_extras_array(e, pass, member, member_in, in_data),
        0x19 => {
            let class_in_member = e.call(MEMBER_CLASS, &args![member_in]).u32();
            let class_out_member = e.call(MEMBER_CLASS, &args![member]).u32();
            if class_in_member == 0 || class_out_member == 0 {
                return;
            }
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let count = fn_00e049c0(e, count_in, count_out);
            let mut i: i32 = 0;
            while i < count {
                let size_in = e.call(CLASS_OBJECT_SIZE, &args![class_in_member]).u32();
                let size_out = e.call(CLASS_OBJECT_SIZE, &args![class_out_member]).u32();
                save_extras(
                    e,
                    this,
                    Ptr::new(size_in.wrapping_mul(i as u32).wrapping_add(in_data)),
                    Ptr::new(class_in_member),
                    archive,
                    Ptr::new(class_out_member),
                    size_out
                        .wrapping_mul(i as u32)
                        .wrapping_add(member_offset)
                        .wrapping_add(base),
                    reloc,
                    depth,
                );
                i += 1;
            }
        }
        0x1b => {
            let class = e.mem.u32(in_data);
            if class == 0 {
                return;
            }
            hk_relocation_info_add_global(
                e,
                reloc,
                member_offset.wrapping_add(base),
                class,
                SHARED_ARGUMENT_0126CAE8,
                1,
            );
            let position = archive_position(e, archive);
            add_local(
                e,
                reloc,
                member_offset.wrapping_add(pointer_out).wrapping_add(base),
                position,
            );
            let elements = e.mem.u32(in_data + 4);
            let mut i: i32 = 0;
            while i < e.mem.u32(in_data + 8) as i32 {
                let size = e.call(CLASS_OBJECT_SIZE, &args![class]).u32();
                save_body(
                    e,
                    this,
                    Ptr::new(size.wrapping_mul(i as u32).wrapping_add(elements)),
                    Ptr::new(class),
                    archive,
                    Ptr::new(class),
                );
                i += 1;
            }
            i = 0;
            while i < e.mem.u32(in_data + 8) as i32 {
                let size = e.call(CLASS_OBJECT_SIZE, &args![class]).u32();
                let size_again = e.call(CLASS_OBJECT_SIZE, &args![class]).u32();
                save_extras(
                    e,
                    this,
                    Ptr::new(size_again.wrapping_mul(i as u32).wrapping_add(elements)),
                    Ptr::new(class),
                    archive,
                    Ptr::new(class),
                    size.wrapping_mul(i as u32).wrapping_add(position),
                    reloc,
                    depth,
                );
                i += 1;
            }
        }
        0x1c => {
            let count_in = calc_c_array_size(e, Ptr::new(member_in));
            let count_out = calc_c_array_size(e, Ptr::new(member));
            let count = fn_00e049c0(e, count_in, count_out);
            let pointer_in = u32::from(e.mem.u8(this.addr() + 8));
            let mut i: u32 = 0;
            let mut odd: u32 = 1;
            while (i as i32) < count {
                let first = e
                    .mem
                    .u32(in_data.wrapping_add(i.wrapping_mul(pointer_in).wrapping_mul(2)));
                let second = e
                    .mem
                    .u32(in_data.wrapping_add(odd.wrapping_mul(pointer_in)));
                if first != 0 {
                    hk_relocation_info_add_global(
                        e,
                        reloc,
                        member_offset
                            .wrapping_add(base)
                            .wrapping_add(pointer_out.wrapping_mul(i).wrapping_mul(2)),
                        first,
                        second,
                        0,
                    );
                }
                if second != 0 {
                    hk_relocation_info_add_global(
                        e,
                        reloc,
                        member_offset
                            .wrapping_add(base)
                            .wrapping_add(pointer_out.wrapping_mul(odd)),
                        second,
                        SHARED_ARGUMENT_0126CAE8,
                        0,
                    );
                }
                odd = odd.wrapping_add(2);
                i += 1;
            }
        }
        0x13 => report_error(
            e,
            |_| vec![TEXT_TYPE_ZERO],
            3,
            0x641e_3e05,
            0x39e,
            Some(0x00e0_4834),
        ),
        1..=0x12 | 0x15 | 0x18 | 0x1e | 0x1f | 0x20 => {}
        _ => report_error(
            e,
            |_| vec![TEXT_UNKNOWN_MEMBER],
            3,
            0x641e_3e05,
            0x3a3,
            Some(0x00e0_4883),
        ),
    }
}

/// The type 0x14 member of `saveExtras` (an array of pointers inline in the object): for each
/// of `min(count in, count out)` elements, a nonzero pointer is saved as a C string at its
/// offset when the member's subtype (`+0xD`) is 2, else recorded with `addGlobal` with the
/// member's class.
fn save_extras_pointer_array(
    e: &mut Engine,
    pass: &ExtrasPass,
    member: u32,
    member_in: u32,
    in_data: u32,
) {
    let (this, archive, base, reloc) = (pass.this, pass.archive, pass.base, pass.reloc);
    let count_out = calc_c_array_size(e, Ptr::new(member));
    let count_in = calc_c_array_size(e, Ptr::new(member_in));
    let count = fn_00e049c0(e, count_in, count_out);
    let count_in = calc_c_array_size(e, Ptr::new(member_in));
    let size = e.call(MEMBER_SIZE_IN_BYTES, &args![member_in]).i32();
    let stride = size / count_in;
    let mut pointer = in_data;
    let mut i: i32 = 0;
    if count <= 0 {
        return;
    }
    while i < count {
        let value = e.mem.u32(pointer);
        if value != 0 {
            let offset = u32::from(e.mem.u8(this.addr() + 0xc))
                .wrapping_mul(i as u32)
                .wrapping_add(u32::from(e.mem.u16(member + 0x12)));
            if e.mem.u8(member + 0xd) == 2 {
                save_cstring(
                    e,
                    archive,
                    reloc,
                    Ptr::new(value),
                    offset.wrapping_add(base),
                );
            } else {
                let class = e.call(MEMBER_CLASS, &args![member]).u32();
                hk_relocation_info_add_global(e, reloc, offset.wrapping_add(base), value, class, 0);
            }
        }
        pointer = pointer.wrapping_add(stride as u32);
        i += 1;
    }
}

/// The array members (types 0x16, 0x17 and 0x1a) of `saveExtras`, by subtype (`+0xD`): nothing
/// for an empty array or subtype 0; pointers (0x14), C strings (0x1d and 0x21), structs
/// (0x19), pairs (0x1c) and plain elements (any other, 0x1e being 4-byte longs).
fn save_extras_array(e: &mut Engine, pass: &ExtrasPass, member: u32, member_in: u32, in_data: u32) {
    let ExtrasPass {
        this,
        archive,
        base,
        reloc,
        depth,
        frame,
    } = *pass;
    if e.mem.u32(in_data + 4) == 0 {
        return;
    }
    let subtype = e.mem.u8(member + 0xd);
    if subtype == 0 {
        return;
    }
    let member_offset = u32::from(e.mem.u16(member + 0x12));
    let pointer_out = u32::from(e.mem.u8(this.addr() + 0xc));
    let array_data = e.mem.u32(in_data);
    let count = e.mem.u32(in_data + 4) as i32;
    match subtype {
        0x14 => {
            let position = archive_position(e, archive);
            add_local(e, reloc, member_offset.wrapping_add(base), position);
            e.mem.set_u32(frame + EXTRAS_ZERO, 0);
            e.mem.set_u32(frame + EXTRAS_ZERO + 4, 0);
            let stride = e.call(MEMBER_ARRAY_MEMBER_SIZE, &args![member_in]).i32();
            if e.mem.u32(in_data + 4) as i32 > 0 {
                let mut offset: i32 = 0;
                let mut i: i32 = 0;
                loop {
                    let list = e.mem.u32(in_data);
                    let pointer = e.mem.u32(list.wrapping_add(offset as u32));
                    if pointer != 0 {
                        let writer = e.call(ARCHIVE_STREAM_WRITER, &args![archive]).u32();
                        let class = e.call(MEMBER_CLASS, &args![member_in]).u32();
                        let position = e.vcall(writer, 0x20, &args![]).u32();
                        hk_relocation_info_add_global(e, reloc, position, pointer, class, 0);
                    }
                    e.call(
                        ARCHIVE_WRITE_RAW,
                        &args![archive, frame + EXTRAS_ZERO, pointer_out],
                    );
                    offset = offset.wrapping_add(stride);
                    i += 1;
                    if i >= e.mem.u32(in_data + 4) as i32 {
                        break;
                    }
                }
            }
        }
        0x1d | 0x21 => {
            let position = archive_position(e, archive);
            add_local(e, reloc, member_offset.wrapping_add(base), position);
            e.mem.set_u32(frame + EXTRAS_ZERO, 0);
            e.mem.set_u32(frame + EXTRAS_ZERO + 4, 0);
            let start = archive_position(e, archive);
            let mut i: i32 = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                e.call(
                    ARCHIVE_WRITE_RAW,
                    &args![archive, frame + EXTRAS_ZERO, pointer_out],
                );
                i += 1;
            }
            i = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                let list = e.mem.u32(in_data);
                let mut text = e.mem.u32(list.wrapping_add(4 * i as u32));
                if subtype == 0x21 {
                    text &= 0xffff_fffe;
                }
                if text != 0 {
                    let offset = pointer_out.wrapping_mul(i as u32).wrapping_add(start);
                    save_cstring(e, archive, reloc, Ptr::new(text), offset);
                }
                i += 1;
            }
        }
        0x19 => {
            let position = archive_position(e, archive);
            add_local(e, reloc, member_offset.wrapping_add(base), position);
            let class_in_member = e.call(MEMBER_CLASS, &args![member_in]).u32();
            let class_out_member = e.call(MEMBER_CLASS, &args![member]).u32();
            if class_in_member == 0 || class_out_member == 0 {
                return;
            }
            let mut i: i32 = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                let size = e.call(CLASS_OBJECT_SIZE, &args![class_in_member]).u32();
                save_body(
                    e,
                    this,
                    Ptr::new(size.wrapping_mul(i as u32).wrapping_add(array_data)),
                    Ptr::new(class_in_member),
                    archive,
                    Ptr::new(class_out_member),
                );
                i += 1;
            }
            i = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                let size_out = e.call(CLASS_OBJECT_SIZE, &args![class_out_member]).u32();
                let size_in = e.call(CLASS_OBJECT_SIZE, &args![class_in_member]).u32();
                save_extras(
                    e,
                    this,
                    Ptr::new(size_in.wrapping_mul(i as u32).wrapping_add(array_data)),
                    Ptr::new(class_in_member),
                    archive,
                    Ptr::new(class_out_member),
                    size_out.wrapping_mul(i as u32).wrapping_add(position),
                    reloc,
                    depth,
                );
                i += 1;
            }
        }
        0x1c => {
            let position = archive_position(e, archive);
            add_local(e, reloc, member_offset.wrapping_add(base), position);
            let mut i: i32 = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                for lane in 0..4 {
                    e.mem.set_u32(frame + EXTRAS_ZERO + 4 * lane, 0);
                }
                e.call(
                    ARCHIVE_WRITE_ARRAY,
                    &args![archive, frame + EXTRAS_ZERO, pointer_out, 2u32],
                );
                i += 1;
            }
            let mut i: i32 = 0;
            while i < e.mem.u32(in_data + 4) as i32 {
                let list = e.mem.u32(in_data);
                let first = e.mem.u32(list.wrapping_add(8 * i as u32));
                let offset = pointer_out.wrapping_mul(i as u32).wrapping_mul(2);
                if first != 0 {
                    let second = e.mem.u32(list.wrapping_add(8 * i as u32) + 4);
                    hk_relocation_info_add_global(
                        e,
                        reloc,
                        position.wrapping_add(offset),
                        first,
                        second,
                        0,
                    );
                }
                let list = e.mem.u32(in_data);
                let second = e.mem.u32(list.wrapping_add(8 * i as u32) + 4);
                if second != 0 {
                    hk_relocation_info_add_global(
                        e,
                        reloc,
                        pointer_out.wrapping_add(offset).wrapping_add(position),
                        second,
                        SHARED_ARGUMENT_0126CAE8,
                        0,
                    );
                }
                i += 1;
            }
        }
        _ => {
            let position = archive_position(e, archive);
            add_local(e, reloc, member_offset.wrapping_add(base), position);
            let pointer = e.mem.u32(in_data);
            if subtype == 0x1e {
                write_ulong_array(e, Ptr::new(pointer), count, archive, pointer_out as i32);
            } else {
                let element_size = e.call(MEMBER_ARRAY_MEMBER_SIZE, &args![member]).u32();
                write_pod_array(
                    e,
                    u32::from(subtype),
                    count,
                    element_size,
                    archive,
                    Ptr::new(pointer),
                );
            }
        }
    }
}

// Translated from 00e04920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectCopier::copyObject` (Xbox PDB), `this` = the copier, `(data_in, class_in, writer,
/// class_out, reloc)`: makes an `hkOArchive` (`00d60830`) on a 0x10-byte stack block over the
/// stream writer with the swap byte at `this + 0x14`, runs [`save_body`], pads the writer to
/// 16 ([`pad_up`]), runs [`save_extras`] with the body's start position, depth 0, pads again,
/// asks the archive whether it is OK (`isOk`, `00d607e0`, a byte written through a pointer)
/// and destroys it. Returns 1 when that byte is 0, else 0.
pub fn copy_object(
    e: &mut Engine,
    this: Ptr,
    data_in: Ptr,
    class_in: Ptr,
    writer: Ptr,
    class_out: Ptr,
    reloc: Ptr,
) -> u32 {
    e.with_stack(0x10, |e, archive| {
        let swap = u32::from(e.mem.u8(this.addr() + 0x14));
        e.call(ARCHIVE_CONSTRUCTOR, &args![archive, writer, swap]);
        let start = save_body(e, this, data_in, class_in, archive, class_out);
        pad_up(e, 0x10, writer);
        save_extras(
            e, this, data_in, class_in, archive, class_out, start, reloc, 0,
        );
        pad_up(e, 0x10, writer);
        let status = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), data_in.addr());
            let result = e.call(ARCHIVE_IS_OK, &args![archive, slot]).u32();
            e.mem.u8(result)
        });
        e.call(ARCHIVE_DESTRUCTOR, &args![archive]);
        u32::from(status == 0)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_3100, calc_c_array_size(Ptr) -> i32),
        entry!(0x00e0_3120, fn_00e03120(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00e0_3160, hk_object_copier_destructor(Ptr)),
        entry!(0x00e0_3170, write_pod_array(u32, i32, u32, Ptr, Ptr)),
        entry!(0x00e0_3240, apply_defaults_for(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00e0_32d0, are_members_compatible(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e0_3300, write_ulong_array(Ptr, i32, Ptr, i32)),
        entry!(0x00e0_3360, pad_up(u32, Ptr)),
        entry!(0x00e0_3460, write_zero(Ptr, Ptr, Ptr)),
        entry!(0x00e0_36b0, save_body(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e0_3e20, save_cstring(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(
            0x00e0_3ea0,
            save_extras(Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(0x00e0_4920, copy_object(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e0_49c0, fn_00e049c0(i32, i32) -> i32),
        entry!(
            0x00e0_4a00,
            hk_relocation_info_add_global(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00e0_49d0, deleting_destructor_00e049d0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e049d0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_49d0, 0x00e0_3160);
    }
}

#[cfg(test)]
mod copier_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const WRITER_WRITE: u32 = 0x0900_0010;
    const WRITER_SEEK: u32 = 0x0900_0011;
    const WRITER_TELL: u32 = 0x0900_0012;
    const FREE_ARRAY: u32 = 0x0900_0020;
    const LOGGER_REPORT: u32 = 0x0900_0030;

    /// The doubles of everything the copier calls outside its file, and a log.
    struct World {
        e: Engine,
        log: Log,
        writer: u32,
        archive: u32,
        router: u32,
        position: Rc<Cell<u32>>,
        report_result: Rc<Cell<u32>>,
    }

    fn hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join("")
    }

    fn world() -> World {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let position = Rc::new(Cell::new(0));
        let report_result = Rc::new(Cell::new(0));
        // The stream writer.
        let seen = Rc::clone(&log);
        e.register_double(WRITER_WRITE, move |e, a| {
            seen.borrow_mut().push(format!(
                "write:{}:{}",
                a[2],
                hex(&e.mem.bytes(a[1], a[2].min(32)))
            ));
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        let cell = Rc::clone(&position);
        e.register_double(WRITER_SEEK, move |_, a| {
            seen.borrow_mut().push(format!("seek:{}:{}", a[1], a[2]));
            cell.set(a[1]);
            Ret::default()
        });
        let cell = Rc::clone(&position);
        e.register_double(WRITER_TELL, move |_, _| Ret {
            eax: cell.get(),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, WRITER_WRITE);
        e.mem.set_u32(vtable + 0x1c, WRITER_SEEK);
        e.mem.set_u32(vtable + 0x20, WRITER_TELL);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer, vtable);
        let archive = e.mem.alloc(0x10);
        e.register_double(ARCHIVE_STREAM_WRITER, move |_, _| Ret {
            eax: writer,
            ..Ret::default()
        });
        // The archive's writers.
        let seen = Rc::clone(&log);
        e.register_double(ARCHIVE_WRITE_RAW, move |e, a| {
            seen.borrow_mut().push(format!(
                "raw:{}:{}",
                a[2],
                hex(&e.mem.bytes(a[1], a[2].min(32)))
            ));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARCHIVE_WRITE_ARRAY, move |e, a| {
            let total = a[2].wrapping_mul(a[3]).min(32);
            seen.borrow_mut().push(format!(
                "array:{}x{}:{}",
                a[2],
                a[3],
                hex(&e.mem.bytes(a[1], total))
            ));
            Ret::default()
        });
        for (address, name) in [
            (ARCHIVE_WRITE8, "w8"),
            (ARCHIVE_WRITE16, "w16"),
            (ARCHIVE_WRITE32, "w32"),
            (ARCHIVE_WRITE32U, "w32u"),
        ] {
            let seen = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                seen.borrow_mut().push(format!("{name}:{:x}", a[1]));
                Ret::default()
            });
        }
        // The router: the temporary top word and the allocator whose slot 0x10 logs.
        let temp = e.mem.alloc(0x4000);
        let seen = Rc::clone(&log);
        e.register_double(FREE_ARRAY, move |_, a| {
            seen.borrow_mut().push(format!("free:{}", a[2]));
            Ret::default()
        });
        let heap_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(heap_vtable + 0x10, FREE_ARRAY);
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap, heap_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, temp);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // Array growth: a fresh block of the wanted size, the old contents copied.
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            seen.borrow_mut().push(format!("more:{}", a[2]));
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(capacity * a[2]);
            let old = e.mem.u32(a[1]);
            if old != 0 {
                let bytes = e.mem.bytes(old, size * a[2]);
                e.mem.write(block, &bytes);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut().push(format!("reserve:{}:{}", a[2], a[3]));
            let block = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        // The error report.
        let seen = Rc::clone(&log);
        e.register_double(STREAM_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut().push("stream".to_string());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STREAM_APPEND, move |e, a| {
            let text = if a[1] < 0x2000_0000 {
                format!("<{:x}>", a[1])
            } else {
                String::from_utf8_lossy(&e.mem.cstr(a[1])).into_owned()
            };
            seen.borrow_mut().push(format!("text:{text}"));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(STREAM_DESTRUCTOR, move |_, _| {
            seen.borrow_mut().push("stream-end".to_string());
            Ret::default()
        });
        let seen = Rc::clone(&log);
        let result = Rc::clone(&report_result);
        e.register_double(LOGGER_REPORT, move |_, a| {
            seen.borrow_mut()
                .push(format!("report:{}:{:x}:{:x}", a[1], a[2], a[5]));
            Ret {
                eax: result.get(),
                ..Ret::default()
            }
        });
        let logger_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(logger_vtable + 0xc, LOGGER_REPORT);
        let logger = e.mem.alloc(0x10);
        e.mem.set_u32(logger, logger_vtable);
        e.mem.set_u32(LOGGER_GLOBAL, logger);
        install_class_model(&mut e, &log);
        World {
            e,
            log,
            writer,
            archive,
            router,
            position,
            report_result,
        }
    }

    /// The class and member accessors over a test layout. A class: name pointer `+0`, object
    /// size `+8`, member count `+0x1C`, members array `+0x20`; a member (0x30 bytes): name `+0`,
    /// class `+4`, enum `+8`, type `+0xC`, subtype `+0xD`, array size (word) `+0xE`, flags (word)
    /// `+0x10`, offset (word) `+0x12`, size in bytes `+0x14`, array member size `+0x18`.
    fn install_class_model(e: &mut Engine, log: &Log) {
        e.register(CLASS_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CLASS_NUM_MEMBERS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e.register(CLASS_MEMBER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20) + 0x30 * a[1],
            ..Ret::default()
        });
        e.register(CLASS_MEMBER_BY_NAME, |e, a| {
            let members = e.mem.u32(a[0] + 0x20);
            let found = (0..e.mem.u32(a[0] + 0x1c))
                .map(|i| members + 0x30 * i)
                .find(|m| e.mem.u32(*m) == a[1])
                .unwrap_or(0);
            Ret {
                eax: found,
                ..Ret::default()
            }
        });
        e.register(CLASS_OBJECT_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        let seen = Rc::clone(log);
        e.register_double(CLASS_DEFAULT, move |e, a| {
            seen.borrow_mut().push(format!("default:{}", a[1]));
            // A class whose first word is 0xdead has no defaults.
            Ret {
                eax: u32::from(e.mem.u32(a[0] + 0x30) != 0xdead),
                ..Ret::default()
            }
        });
        e.register(MEMBER_ARRAY_SIZE, |e, a| Ret {
            eax: i32::from(e.mem.i16(a[0] + 0xe)) as u32,
            ..Ret::default()
        });
        for address in [MEMBER_CLASS, MEMBER_STRUCT_CLASS] {
            e.register(address, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            });
        }
        e.register(MEMBER_ENUM, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(MEMBER_SIZE_IN_BYTES, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x14),
            ..Ret::default()
        });
        e.register(MEMBER_ARRAY_MEMBER_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
        e.register(MEMBER_ENUM_VALUE, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        let seen = Rc::clone(log);
        e.register_double(RELOCATION_ADD_LOCAL, move |_, a| {
            seen.borrow_mut().push(format!("local:{}:{}", a[1], a[2]));
            Ret::default()
        });
    }

    /// A member of the test layout.
    fn member(e: &mut Engine, name: u32, ty: u8, subtype: u8, offset: u16, size: u32) -> u32 {
        let record = e.mem.alloc(0x30);
        e.mem.set_u32(record, name);
        e.mem.set_u8(record + 0xc, ty);
        e.mem.set_u8(record + 0xd, subtype);
        e.mem.set_i16(record + 0xe, 1);
        e.mem.set_u16(record + 0x12, offset);
        e.mem.set_u32(record + 0x14, size);
        record
    }

    /// A class made of `members` (the member records, copied into one array of 0x30-byte
    /// records).
    fn class(e: &mut Engine, name: &[u8], object_size: u32, members: &[u32]) -> u32 {
        let record = e.mem.alloc(0x40);
        let name_text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(name_text, name);
        e.mem.set_u32(record, name_text);
        e.mem.set_u32(record + 8, object_size);
        e.mem.set_u32(record + 0x1c, members.len() as u32);
        let array = e.mem.alloc(0x30 * members.len().max(1) as u32);
        for (i, source) in members.iter().enumerate() {
            let bytes = e.mem.bytes(*source, 0x30);
            e.mem.write(array + 0x30 * i as u32, &bytes);
        }
        e.mem.set_u32(record + 0x20, array);
        record
    }

    fn name(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text);
        block
    }

    /// A copier whose virtual slot `0x10` is the real `areMembersCompatible`; the two layout
    /// bytes at `+8` and `+0xC` (pointer sizes), the flags at `+0x10` and the swap byte.
    fn copier(e: &mut Engine, flags: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x00e0_32d0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.mem.set_u8(this + 8, 4);
        e.mem.set_u8(this + 0xc, 4);
        e.mem.set_u32(this + 0x10, flags);
        this
    }

    fn lines(w: &World) -> Vec<String> {
        w.log.borrow().clone()
    }

    #[test]
    fn calc_c_array_size_defaults_to_one_and_asks_twice_otherwise() {
        let mut w = world();
        let plain = member(&mut w.e, 0, 1, 0, 0, 4);
        w.e.mem.set_i16(plain + 0xe, 0);
        assert_eq!(w.e.call(0x00e0_3100, &args![plain]).i32(), 1);
        let array = member(&mut w.e, 0, 1, 0, 0, 4);
        w.e.mem.set_i16(array + 0xe, -3);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x00e0_3100, &args![array]).i32(), -3);
        let asked =
            w.e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == MEMBER_ARRAY_SIZE)
                .count();
        assert_eq!(asked, 2);
    }

    #[test]
    fn constructor_and_destructor() {
        let mut w = world();
        let this = w.e.mem.alloc(0x20);
        let first = w.e.mem.alloc(4);
        let second = w.e.mem.alloc(4);
        w.e.mem.set_u32(first, 0x0000_0804);
        w.e.mem.set_u32(second, 0x0000_0108);
        let got =
            w.e.call(0x00e0_3120, &args![this, first, second, 3u32])
                .u32();
        assert_eq!(got, this);
        assert_eq!(w.e.mem.u16(this + 6), 1);
        assert_eq!(w.e.mem.u32(this), OBJECT_COPIER_VTABLE);
        assert_eq!(w.e.mem.u32(this + 8), 0x0804);
        assert_eq!(w.e.mem.u32(this + 0xc), 0x0108);
        assert_eq!(w.e.mem.u32(this + 0x10), 3);
        // The second bytes of the two layouts (0x08 and 0x01) differ.
        assert_eq!(w.e.mem.u8(this + 0x14), 1);
        w.e.mem.set_u32(second, 0x0000_0804);
        w.e.call(0x00e0_3120, &args![this, first, second, 0u32]);
        assert_eq!(w.e.mem.u8(this + 0x14), 0);
        w.e.call(0x00e0_3160, &args![this]);
        assert_eq!(w.e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn write_pod_array_scales_vector_types_and_rejects_unknown_ones() {
        let mut w = world();
        let data = w.e.mem.alloc(0x40);
        // Type 1: written as given, with the element size 2.
        w.e.call(0x00e0_3170, &args![1u32, 3u32, 2u32, w.archive, data]);
        // Type 0xc (a vector of 16 bytes): 4-byte elements, count times four.
        w.e.call(0x00e0_3170, &args![0xcu32, 3u32, 16u32, w.archive, data]);
        // Type 0x20 like type 1.
        w.e.call(0x00e0_3170, &args![0x20u32, 1u32, 8u32, w.archive, data]);
        assert_eq!(
            lines(&w),
            vec![
                format!("array:2x3:{}", "00".repeat(6)),
                format!("array:4x12:{}", "00".repeat(32)),
                format!("array:8x1:{}", "00".repeat(8)),
            ]
        );
        // An unknown type reports (id 0x747e1e03, line 0xfb) and writes nothing.
        w.log.borrow_mut().clear();
        w.e.call(0x00e0_3170, &args![0x13u32, 1u32, 4u32, w.archive, data]);
        assert_eq!(
            lines(&w),
            vec![
                "stream".to_string(),
                format!("text:<{TEXT_UNKNOWN_POD_ARRAY:x}>"),
                "report:3:747e1e03:fb".to_string(),
                "stream-end".to_string(),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "INT3 executed at 00e031fa")]
    fn write_pod_array_traps_when_the_report_asks_for_a_break() {
        let mut w = world();
        w.report_result.set(1);
        let data = w.e.mem.alloc(8);
        w.e.call(0x00e0_3170, &args![0x13u32, 1u32, 4u32, w.archive, data]);
    }

    #[test]
    fn apply_defaults_for_walks_struct_members() {
        let mut w = world();
        // A struct class with two members; the outer member is a struct array of 2.
        let inner_a = member(&mut w.e, 0, 1, 0, 0, 4);
        let inner_b = member(&mut w.e, 0, 1, 0, 4, 4);
        let inner = class(&mut w.e, b"Inner", 8, &[inner_a, inner_b]);
        let outer_member = member(&mut w.e, 0, 0x19, 0, 0, 16);
        w.e.mem.set_u32(outer_member + 4, inner);
        w.e.mem.set_i16(outer_member + 0xe, 2);
        let outer = class(&mut w.e, b"Outer", 16, &[outer_member]);
        let outer_first = w.e.mem.u32(outer + 0x20);
        let got =
            w.e.call(0x00e0_3240, &args![outer, outer_first, 0u32, 0x55u32])
                .u32();
        assert_eq!(got, 1);
        // The outer default, then both members of the inner class, twice.
        assert_eq!(
            lines(&w),
            [
                "default:0",
                "default:0",
                "default:1",
                "default:0",
                "default:1"
            ]
        );
        // A class without defaults (marked by the word 0xdead at +0x30) makes the result 0.
        w.log.borrow_mut().clear();
        w.e.mem.set_u32(inner + 0x30, 0xdead);
        let got =
            w.e.call(0x00e0_3240, &args![outer, outer_first, 0u32, 0x55u32])
                .u32();
        assert_eq!(got, 0);
        // A plain member just takes its default.
        w.log.borrow_mut().clear();
        let plain = w.e.mem.u32(inner + 0x20);
        w.e.call(0x00e0_3240, &args![inner, plain, 7u32, 0x55u32]);
        assert_eq!(lines(&w), ["default:7"]);
    }

    #[test]
    fn are_members_compatible_compares_type_and_subtype() {
        let mut w = world();
        let a = member(&mut w.e, 0, 1, 2, 0, 4);
        let same = member(&mut w.e, 0, 1, 2, 0, 4);
        let other_subtype = member(&mut w.e, 0, 1, 3, 0, 4);
        let other_type = member(&mut w.e, 0, 2, 2, 0, 4);
        let enum_a = member(&mut w.e, 0, 0x18, 1, 0, 4);
        let enum_b = member(&mut w.e, 0, 0x18, 2, 0, 4);
        let check = |w: &mut World, x: u32, y: u32| w.e.call(0x00e0_32d0, &args![0u32, x, y]).u32();
        assert_eq!(check(&mut w, a, same), 1);
        assert_eq!(check(&mut w, a, other_subtype), 0);
        assert_eq!(check(&mut w, a, other_type), 0);
        // Enums are compatible whatever their subtype.
        assert_eq!(check(&mut w, enum_a, enum_b), 1);
    }

    #[test]
    fn write_ulong_array_widens_elements_unless_they_are_four_bytes() {
        let mut w = world();
        let source = w.e.mem.alloc(8);
        w.e.mem.set_u32(source, 0x1111_2222);
        w.e.mem.set_u32(source + 4, 0x3333_4444);
        let top = w.e.mem.u32(w.router);
        w.e.call(0x00e0_3300, &args![source, 2u32, w.archive, 4u32]);
        w.e.call(0x00e0_3300, &args![source, 2u32, w.archive, 8u32]);
        assert_eq!(
            lines(&w),
            vec![
                "array:4x2:2222111144443333".to_string(),
                // Eight-byte elements: each low word followed by a zero word.
                "array:8x2:22221111000000004444333300000000".to_string(),
            ]
        );
        // The router's top word is put back.
        assert_eq!(w.e.mem.u32(w.router), top);
    }

    #[test]
    fn pad_up_writes_zero_bytes_to_the_alignment() {
        let mut w = world();
        let top = w.e.mem.u32(w.router);
        w.position.set(5);
        w.e.call(0x00e0_3360, &args![16u32, w.writer]);
        assert_eq!(lines(&w), vec![format!("write:11:{}", "00".repeat(11))]);
        assert_eq!(w.e.mem.u32(w.router), top);
        w.log.borrow_mut().clear();
        w.position.set(32);
        w.e.call(0x00e0_3360, &args![16u32, w.writer]);
        assert!(lines(&w).is_empty());
        // The scratch buffer is in place (`size | 0x80000000`): nothing is reserved or freed.
        assert!(!lines(&w)
            .iter()
            .any(|l| l.starts_with("free") || l.starts_with("reserve")));
    }

    #[test]
    fn write_zero_by_member_type() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        // A plain member: its size in bytes.
        let plain = member(&mut w.e, 0, 1, 0, 0, 6);
        w.e.call(0x00e0_3460, &args![this, w.archive, plain]);
        // A pointer-like member: pointer size 4 plus 4 bytes.
        let pointer = member(&mut w.e, 0, 0x17, 0, 0, 0);
        w.e.call(0x00e0_3460, &args![this, w.archive, pointer]);
        // The array pair type 0x16 writes its bytes raw and then the flag word.
        let array = member(&mut w.e, 0, 0x16, 0, 0, 0);
        w.e.call(0x00e0_3460, &args![this, w.archive, array]);
        // A struct: object size 0x20 of its class times its array size 2.
        let inner = class(&mut w.e, b"I", 0x20, &[]);
        let structure = member(&mut w.e, 0, 0x19, 0, 0, 0);
        w.e.mem.set_u32(structure + 4, inner);
        w.e.mem.set_i16(structure + 0xe, 2);
        w.e.call(0x00e0_3460, &args![this, w.archive, structure]);
        // Type 0x14: array size 3 times the pointer size.
        let pointers = member(&mut w.e, 0, 0x14, 0, 0, 0);
        w.e.mem.set_i16(pointers + 0xe, 3);
        w.e.call(0x00e0_3460, &args![this, w.archive, pointers]);
        // Type 0x1c: twice the pointer size times 1.
        let pairs = member(&mut w.e, 0, 0x1c, 0, 0, 0);
        w.e.call(0x00e0_3460, &args![this, w.archive, pairs]);
        let zeros = |n: usize| "00".repeat(n.min(32));
        assert_eq!(
            lines(&w),
            vec![
                format!("raw:6:{}", zeros(6)),
                format!("raw:8:{}", zeros(8)),
                format!("raw:8:{}", zeros(8)),
                "w32u:80000000".to_string(),
                format!("raw:64:{}", zeros(64)),
                format!("raw:12:{}", zeros(12)),
                format!("raw:8:{}", zeros(8)),
            ]
        );
        // The router's temporary top is untouched by the buffers at the end.
        // An unknown type reports.
        w.log.borrow_mut().clear();
        let unknown = member(&mut w.e, 0, 0x13, 0, 0, 0);
        w.e.call(0x00e0_3460, &args![this, w.archive, unknown]);
        let log = lines(&w);
        assert_eq!(log[0], "stream");
        assert_eq!(log[2], "report:3:5ef4e5a4:a8");
        // It then writes the empty buffer.
        assert_eq!(log[4], "raw:0:");
    }

    #[test]
    fn save_cstring_pads_records_and_writes_the_text() {
        let mut w = world();
        let array = w.e.mem.alloc(0xc);
        w.position.set(3);
        let text = name(&mut w.e, b"hello");
        let got =
            w.e.call(0x00e0_3e20, &args![w.archive, array, text, 0x44u32])
                .u32();
        assert_eq!(got, 0);
        // Padded to 2 (one zero byte), the record (offset 0x44, position 3) added, the text
        // and its terminator written raw.
        let log = lines(&w);
        assert_eq!(log[0], "write:1:00");
        assert_eq!(log[1], "more:8");
        assert_eq!(log[2], format!("raw:6:{}", hex(b"hello\0")));
        let data = w.e.mem.u32(array);
        assert_eq!(w.e.mem.u32(array + 4), 1);
        assert_eq!((w.e.mem.u32(data), w.e.mem.u32(data + 4)), (0x44, 3));
    }

    #[test]
    fn save_body_writes_matching_members_and_defaults_for_the_rest() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        let a_name = name(&mut w.e, b"a");
        let b_name = name(&mut w.e, b"b");
        let c_name = name(&mut w.e, b"c");
        let in_a = member(&mut w.e, a_name, 1, 0, 8, 4);
        let class_in = class(&mut w.e, b"In", 0x10, &[in_a]);
        let out_a = member(&mut w.e, a_name, 1, 0, 0, 4);
        let out_b = member(&mut w.e, b_name, 1, 0, 4, 4);
        let out_c = member(&mut w.e, c_name, 0x1d, 0, 8, 4);
        let class_out = class(&mut w.e, b"Out", 0x10, &[out_a, out_b, out_c]);
        let data = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(data + 8, 0xaabb_ccdd);
        w.position.set(0x100);
        let start =
            w.e.call(
                0x00e0_36b0,
                &args![this, data, class_in, w.archive, class_out],
            )
            .u32();
        assert_eq!(start, 0x100);
        assert_eq!(
            lines(&w),
            vec![
                "seek:256:0".to_string(),
                format!("array:4x1:{}", hex(&0xaabb_ccddu32.to_le_bytes())),
                "seek:260:0".to_string(),
                "default:1".to_string(),
                "seek:264:0".to_string(),
                "default:2".to_string(),
                // Finally the writer is put at the object's end.
                "seek:272:0".to_string(),
            ]
        );
    }

    #[test]
    fn save_body_zero_fills_flagged_members_when_the_copier_asks_for_it() {
        let mut w = world();
        let this = copier(&mut w.e, 2);
        let a_name = name(&mut w.e, b"a");
        let flagged = member(&mut w.e, a_name, 1, 0, 0, 4);
        w.e.mem.set_u16(flagged + 0x10, 0x400);
        let class_out = class(&mut w.e, b"Out", 4, &[flagged]);
        let class_in = class(&mut w.e, b"In", 4, &[]);
        let data = w.e.mem.alloc(8);
        w.e.call(
            0x00e0_36b0,
            &args![this, data, class_in, w.archive, class_out],
        );
        assert_eq!(
            lines(&w),
            vec![
                "seek:0:0".to_string(),
                "raw:4:00000000".to_string(),
                "seek:4:0".to_string()
            ]
        );
        // With flag 1 instead the default is applied.
        w.log.borrow_mut().clear();
        w.e.mem.set_u32(this + 0x10, 1);
        w.position.set(0);
        w.e.call(
            0x00e0_36b0,
            &args![this, data, class_in, w.archive, class_out],
        );
        assert_eq!(
            lines(&w),
            vec![
                "seek:0:0".to_string(),
                "default:0".to_string(),
                "seek:4:0".to_string()
            ]
        );
    }

    #[test]
    fn save_extras_records_the_class_and_saves_strings() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        let s_name = name(&mut w.e, b"s");
        let in_member = member(&mut w.e, s_name, 0x1d, 0, 4, 4);
        let class_in = class(&mut w.e, b"In", 8, &[in_member]);
        let out_member = member(&mut w.e, s_name, 0x1d, 0, 4, 4);
        let class_out = class(&mut w.e, b"Out", 8, &[out_member]);
        let data = w.e.mem.alloc(0x10);
        let text = name(&mut w.e, b"hey");
        w.e.mem.set_u32(data + 4, text);
        // The relocation info: the imports array at +0x18 starts empty; the string records go
        // to the array at +0.
        let reloc = w.e.mem.alloc(0x40);
        w.position.set(0x40);
        w.e.call(
            0x00e0_3ea0,
            &args![this, data, class_in, w.archive, class_out, 0x100u32, reloc, 0u32],
        );
        // The (base, class name) record of depth 0 went into the imports array.
        let imports = w.e.mem.u32(reloc + 0x18);
        assert_eq!(w.e.mem.u32(reloc + 0x1c), 1);
        assert_eq!(w.e.mem.u32(imports), 0x100);
        assert_eq!(w.e.mem.u32(imports + 4), w.e.mem.u32(class_out));
        // The string was saved ((offset 4 + base 0x100, position)) and the writer padded to 16
        // afterwards.
        let records = w.e.mem.u32(reloc);
        assert_eq!(w.e.mem.u32(reloc + 4), 1);
        assert_eq!(w.e.mem.u32(records), 0x104);
        let log = lines(&w);
        assert!(log.contains(&format!("raw:4:{}", hex(b"hey\0"))));
        // A deeper level does not add the record again.
        let before = w.e.mem.u32(reloc + 0x1c);
        w.e.call(
            0x00e0_3ea0,
            &args![this, data, class_in, w.archive, class_out, 0x100u32, reloc, 1u32],
        );
        assert_eq!(w.e.mem.u32(reloc + 0x1c), before);
    }

    #[test]
    fn save_extras_records_pointers_and_recurses_into_structs() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        let p_name = name(&mut w.e, b"p");
        let t_name = name(&mut w.e, b"t");
        // A pointer member (type 0x14 with a class) in a struct member.
        let target = class(&mut w.e, b"Target", 4, &[]);
        let pointer_member = member(&mut w.e, p_name, 0x14, 0, 0, 4);
        w.e.mem.set_u32(pointer_member + 4, target);
        let inner = class(&mut w.e, b"Inner", 4, &[pointer_member]);
        let struct_member = member(&mut w.e, t_name, 0x19, 0, 8, 4);
        w.e.mem.set_u32(struct_member + 4, inner);
        let outer = class(&mut w.e, b"Outer", 12, &[struct_member]);
        let data = w.e.mem.alloc(0x20);
        // The struct sits at +8 and its pointer is its first word.
        w.e.mem.set_u32(data + 8, 0x7777);
        let reloc = w.e.mem.alloc(0x40);
        w.e.call(
            0x00e0_3ea0,
            &args![this, data, outer, w.archive, outer, 0u32, reloc, 0u32],
        );
        // addGlobal(offset = 8 (struct member) + 0 (pointer member), pointer 0x7777, the
        // member's class `Target`, 0) went into the array at reloc + 0xC.
        assert_eq!(w.e.mem.u32(reloc + 0x10), 1);
        let globals = w.e.mem.u32(reloc + 0xc);
        assert_eq!(w.e.mem.u32(globals), 8);
        assert_eq!(w.e.mem.u32(globals + 4), 0x7777);
        assert_eq!(w.e.mem.u32(globals + 8), target);
    }

    #[test]
    fn save_extras_reports_type_zero_and_unknown_members() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        let n = name(&mut w.e, b"n");
        for (ty, line, id) in [(0x13u8, "39e", "641e3e05"), (0x77u8, "3a3", "641e3e05")] {
            let bad = member(&mut w.e, n, ty, 0, 0, 4);
            let class_a = class(&mut w.e, b"A", 4, &[bad]);
            let data = w.e.mem.alloc(8);
            let reloc = w.e.mem.alloc(0x40);
            w.log.borrow_mut().clear();
            w.e.call(
                0x00e0_3ea0,
                &args![this, data, class_a, w.archive, class_a, 0u32, reloc, 1u32],
            );
            let log = lines(&w);
            assert_eq!(log[0], "stream");
            assert_eq!(log[2], format!("report:3:{id}:{line}"));
            assert_eq!(log[3], "stream-end");
        }
    }

    #[test]
    fn copy_object_runs_both_passes_and_asks_the_archive() {
        let mut w = world();
        let this = copier(&mut w.e, 0);
        w.e.mem.set_u8(this + 0x14, 1);
        let seen = Rc::clone(&w.log);
        w.e.register_double(ARCHIVE_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut()
                .push(format!("archive:{:x}:{:x}", a[1], a[2]));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&w.log);
        w.e.register_double(ARCHIVE_IS_OK, move |e, a| {
            seen.borrow_mut().push("is-ok".to_string());
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&w.log);
        w.e.register_double(ARCHIVE_DESTRUCTOR, move |_, _| {
            seen.borrow_mut().push("archive-end".to_string());
            Ret::default()
        });
        let class_in = class(&mut w.e, b"In", 4, &[]);
        let class_out = class(&mut w.e, b"Out", 4, &[]);
        let data = w.e.mem.alloc(8);
        let reloc = w.e.mem.alloc(0x40);
        w.position.set(0x10);
        let result =
            w.e.call(
                0x00e0_4920,
                &args![this, data, class_in, w.writer, class_out, reloc],
            )
            .u32();
        // `isOk` said the stream is fine (the byte is 1), so the result is 0.
        assert_eq!(result, 0);
        let log = lines(&w);
        assert_eq!(log[0], format!("archive:{:x}:1", w.writer));
        // saveBody ends with the seek past the object (0x10 + 4); both pads find the
        // position unchanged by the doubled seek: 0x14 is not a multiple of 16, so
        // 12 zero bytes follow each pass.
        assert_eq!(log[1], "seek:20:0");
        assert_eq!(log[2], format!("write:12:{}", "00".repeat(12)));
        assert!(log.contains(&"is-ok".to_string()));
        assert_eq!(log.last().map(String::as_str), Some("archive-end"));
    }

    #[test]
    fn min_and_add_global() {
        let mut w = world();
        assert_eq!(w.e.call(0x00e0_49c0, &args![3i32, 5i32]).i32(), 3);
        assert_eq!(w.e.call(0x00e0_49c0, &args![5i32, 3i32]).i32(), 3);
        assert_eq!(w.e.call(0x00e0_49c0, &args![-1i32, 1i32]).i32(), -1);
        assert_eq!(w.e.call(0x00e0_49c0, &args![4i32, 4i32]).i32(), 4);
        let this = w.e.mem.alloc(0x20);
        w.e.call(0x00e0_4a00, &args![this, 1u32, 2u32, 3u32, 4u32]);
        w.e.call(0x00e0_4a00, &args![this, 5u32, 6u32, 7u32, 8u32]);
        assert_eq!(w.e.mem.u32(this + 0x10), 2);
        let data = w.e.mem.u32(this + 0xc);
        let words: Vec<u32> = (0..8).map(|i| w.e.mem.u32(data + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        // The first record made room (capacity 0 = size).
        assert_eq!(lines(&w)[0], "more:16");
    }
}

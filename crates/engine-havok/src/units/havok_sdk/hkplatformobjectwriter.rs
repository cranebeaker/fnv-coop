//! `Havok SDK/hkplatformobjectwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::ARRAY_GROW;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e02e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_2c70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e02e40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_2c70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// The writer (vtable `010ea370`; Xbox PDB `hkPlatformObjectWriter`) keeps a converter object
// at `+8` (0x18 bytes, vtable `010ea3b0`, built by `00e03120`; its slot `+0xC` converts and
// writes an object) and, when the target layout differs from the host one, a cache at `+0xC`
// (0x24 bytes): two maps (`+0` and `+0xC`) and an array of copied objects (`+0x18` data,
// `+0x1C` size, `+0x20` capacity with the not-owned flag in bit 31).
//
// The objects the functions use (unconfirmed names where marked):
//  - `00cb1980`: `hkMapBase::get` (`this` the map, then key and the address of the value
//    to fill; answers 0 when the key is present),
//  - `00cb1fe0`: `hkMapBase::reserve` (`this` the map, then allocator and capacity),
//  - `00cb1a90`: `hkMapBase::clearAndDeallocate` (`this` the map, then the allocator),
//  - `00e4afe0`: `hkDeepCopier::deepCopy` (cdecl, six words),
//  - `00c856c0`: `hkMemoryRouter::easyFree` (cdecl, allocator and block),
//  - `00e02ef0`, `00e02e90`: `hkOffsetOnlyStreamWriter`'s constructor and `seek`,
//  - `00e028e0`: `hkRelocationInfo::~hkRelocationInfo`,
//  - `00d46d40`, `00cb0fa0`: `hkClass::getName` and `hkString::strCmp` (cdecl),
//  - `00dc1680`, `00dc1bb0`: copy of a layout-rules word and the layout-driven copy of an
//    object (names unconfirmed in the map, `this` the layout, then object, map and a flag).
const MAP_GET: u32 = 0x00cb_1980;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const DEEP_COPY: u32 = 0x00e4_afe0;
const EASY_FREE: u32 = 0x00c8_56c0;
const OFFSET_WRITER_CONSTRUCT: u32 = 0x00e0_2ef0;
const OFFSET_WRITER_SEEK: u32 = 0x00e0_2e90;
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const LAYOUT_RULES_COPY: u32 = 0x00dc_1680;
const LAYOUT_COPY: u32 = 0x00dc_1bb0;
const CONVERTER_CONSTRUCT: u32 = 0x00e0_3120;
const NO_OP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Vtable of the writer, of the `hkReferencedObject` base it ends as, the host layout rules
/// (a dword of four bytes), the text `"hkClass"` and the table `deepCopy` takes.
const WRITER_VTABLE: u32 = 0x010e_a370;
const BASE_VTABLE: u32 = 0x0102_e388;
const HOST_RULES: u32 = 0x011b_7b88;
const CLASS_NAME: u32 = 0x010d_66fc;
const DEEP_COPY_TABLE: u32 = 0x0126_cae8;

/// The address of the memory router's allocator (`+0x10` of the router).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00e02a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::writeRaw` (Xbox PDB): asks `stream` (slot `+0x10`) to write
/// `size` bytes from `buffer`; answers 1 when it wrote another count. The writer itself is not
/// read.
pub fn hk_platform_object_writer_write_raw(
    e: &mut Engine,
    _this: Ptr,
    stream: Ptr,
    buffer: u32,
    size: u32,
) -> u32 {
    let written = e.vcall(stream.addr(), 0x10, &args![buffer, size]).u32();
    u32::from(written != size)
}

// Translated from 00e02a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::getLayout` (Xbox PDB): the address `+0xC` into the converter at
/// `+8`, which holds the target layout rules.
pub fn hk_platform_object_writer_get_layout(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8).wrapping_add(0xc)
}

// Translated from 00e02a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::writeObject` (Xbox PDB): converts `object` of `class` with the
/// converter at `+8` (slot `+0xC`: object, class, stream, converted class, extra) and answers
/// 1 when the stream then reports ok.
///
/// With a cache (`+0xC`) the class is first looked up in it (`Cache::get`, keyed by the class
/// with the writer's layout). When the converted class is named `"hkClass"` the object (a
/// class description) is converted too, written to `stream` with a relocation record built in
/// the frame (four empty arrays and a null map pointer, destroyed afterwards), the
/// offset-only writer in the frame is moved to the stream position and the main write goes to
/// that writer instead of `stream`. The frame is the 0x48 bytes the exe keeps on its stack:
/// converted object `+0`, offset-only writer `+4` (0x10 bytes), relocation record `+0x14`
/// (0x34 bytes).
pub fn hk_platform_object_writer_write_object(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    object: Ptr,
    class: Ptr,
    extra: Ptr,
) -> u32 {
    let me = this.addr();
    let cache = e.mem.u32(me + 0xc);
    let converted_class = if cache != 0 {
        let layout = hk_platform_object_writer_get_layout(e, this);
        hk_platform_object_writer_cache_get(e, Ptr::new(cache), class.addr(), layout)
    } else {
        class.addr()
    };
    e.with_stack(0x48, |e, frame| {
        let writer = frame.addr() + 4;
        let relocation = frame.addr() + 0x14;
        e.call(OFFSET_WRITER_CONSTRUCT, &args![writer]);
        let mut target = stream.addr();
        if cache != 0 {
            let name = e.call(CLASS_GET_NAME, &args![converted_class]).u32();
            let order = e.call(STRING_COMPARE, &args![name, CLASS_NAME]).u32();
            if order == 0 {
                let layout = hk_platform_object_writer_get_layout(e, this);
                let converted_object =
                    hk_platform_object_writer_cache_get(e, Ptr::new(cache), object.addr(), layout);
                e.mem.set_u32(frame.addr(), converted_object);
                let position = e.vcall(stream.addr(), 0x20, &args![]).u32();
                e.call(OFFSET_WRITER_SEEK, &args![writer, position, 0u32]);
                for array in [0x08, 0x14, 0x20, 0x2c] {
                    e.mem.set_u32(relocation + array, 0x8000_0000);
                }
                for zero in [0x00, 0x04, 0x0c, 0x10, 0x18, 0x1c, 0x24, 0x28, 0x30] {
                    e.mem.set_u32(relocation + zero, 0);
                }
                let converter = e.mem.u32(me + 8);
                e.vcall(
                    converter,
                    0xc,
                    &args![converted_object, class, stream, converted_class, relocation],
                );
                e.call(RELOCATION_INFO_DESTRUCT, &args![relocation]);
                target = writer;
            }
        }
        let converter = e.mem.u32(me + 8);
        e.vcall(
            converter,
            0xc,
            &args![object, class, target, converted_class, extra],
        );
        e.with_stack(4, |e, ok| {
            let status = e.vcall(target, 0xc, &args![ok]).u32();
            u32::from(e.mem.u8(status) == 0)
        })
    })
}

// Translated from 00e02bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::hkPlatformObjectWriter` (Xbox PDB): reference count 1, the
/// vtable, a converter (0x18 bytes from the memory router's allocator, size kept at `+4`)
/// built from a copy of the host layout rules and `rules` (the target) with mode 2, stored at
/// `+8`. The cache at `+0xC` is built (0x24 bytes) only when any of the four bytes of the
/// target layout differs from the host rules, and is null when the allocation fails.
/// Answers `this`.
pub fn hk_platform_object_writer_construct(e: &mut Engine, this: Ptr, rules: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, WRITER_VTABLE);
    let pool = allocator(e);
    let converter = e.vcall(pool, 4, &args![0x18u32]).u32();
    e.mem.set_u16(converter + 4, 0x18);
    let made = e.with_stack(4, |e, copy| {
        e.call(LAYOUT_RULES_COPY, &args![copy, HOST_RULES]);
        e.call(CONVERTER_CONSTRUCT, &args![converter, copy, rules, 2u32])
            .u32()
    });
    e.mem.set_u32(me + 8, made);
    let layout = hk_platform_object_writer_get_layout(e, this);
    let same = (0..4).all(|byte| e.mem.u8(layout + byte) == e.mem.u8(HOST_RULES + byte));
    if same {
        e.mem.set_u32(me + 0xc, 0);
        return this;
    }
    let pool = allocator(e);
    let block = e.vcall(pool, 4, &args![0x24u32]).u32();
    if block == 0 {
        e.mem.set_u32(me + 0xc, 0);
        return this;
    }
    let cache = hk_platform_object_writer_cache_construct(e, Ptr::new(block));
    e.mem.set_u32(me + 0xc, cache.addr());
    this
}

// Translated from 00e02c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::~hkPlatformObjectWriter` (Xbox PDB): releases the converter at
/// `+8` (no null check), destroys and frees the cache at `+0xC` (0x24 bytes through the
/// allocator's slot `+8`) when there is one, and ends with the `hkReferencedObject` vtable.
pub fn hk_platform_object_writer_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let converter = e.mem.u32(me + 8);
    e.mem.set_u32(me, WRITER_VTABLE);
    e.call(REMOVE_REFERENCE, &args![converter]);
    let cache = e.mem.u32(me + 0xc);
    if cache != 0 {
        hk_platform_object_writer_cache_destruct(e, Ptr::new(cache));
        let pool = allocator(e);
        e.vcall(pool, 8, &args![cache, 0x24u32]);
    }
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00e02cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::Cache::get` (Xbox PDB): the copy of `key` made for `layout`. A
/// key already in the map (`+0`) answers its stored copy. Otherwise `deepCopy` makes one, the
/// router's allocator grows the array of copies (`+0x18`, four-byte elements) when it is full,
/// the copy is appended, and the layout copy (`00dc1bb0`) registers it in the map at `+0xC`.
pub fn hk_platform_object_writer_cache_get(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    layout: u32,
) -> u32 {
    let me = this.addr();
    let found = e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), key);
        let missing = e.call(MAP_GET, &args![this, key, value]).u32();
        (missing == 0).then(|| e.mem.u32(value.addr()))
    });
    if let Some(copy) = found {
        return copy;
    }
    let copy = e
        .call(
            DEEP_COPY,
            &args![key, DEEP_COPY_TABLE, this, 0u32, 0u32, 0u32],
        )
        .u32();
    let array = me + 0x18;
    let pool = allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![pool, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    e.mem.set_u32(slot, copy);
    e.mem
        .set_u32(array + 4, e.mem.u32(array + 4).wrapping_add(1));
    e.call(LAYOUT_COPY, &args![layout, copy, me + 0xc, 1u32]);
    copy
}

// Translated from 00e02d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::Cache::~Cache` (Xbox PDB): frees every copy in the array
/// (`easyFree` with the router's allocator; the array's size and data are read again for each),
/// empties the array, frees its buffer through the allocator's slot `+0x10` when the
/// capacity word does not carry the not-owned flag (bit 31), and clears both maps.
pub fn hk_platform_object_writer_cache_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let mut index = 0u32;
    if (e.mem.i32(me + 0x1c)) > 0 {
        loop {
            let copy = e
                .mem
                .u32(e.mem.u32(me + 0x18).wrapping_add(index.wrapping_mul(4)));
            let pool = allocator(e);
            e.call(EASY_FREE, &args![pool, copy]);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(me + 0x1c) {
                break;
            }
        }
    }
    let pool = allocator(e);
    let capacity = e.mem.u32(me + 0x20);
    e.mem.set_u32(me + 0x1c, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        let data = e.mem.u32(me + 0x18);
        e.vcall(pool, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x20, 0x8000_0000);
    let second = me + 0xc;
    let pool = allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![second, pool]);
    e.call(NO_OP_DESTRUCTOR, &args![second]);
    let pool = allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![this, pool]);
    e.call(NO_OP_DESTRUCTOR, &args![this]);
}

// Translated from 00e02de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlatformObjectWriter::Cache::Cache` (Xbox PDB): both maps (`+0` and `+0xC`) are empty
/// (data null, size 0, mask -1) and reserved with capacity 0 through the router's allocator;
/// the array at `+0x18` is empty with the not-owned flag. Answers `this`.
pub fn hk_platform_object_writer_cache_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    for map in [me, me + 0xc] {
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let pool = allocator(e);
        e.call(MAP_RESERVE, &args![map, pool, 0u32]);
    }
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, 0);
    e.mem.set_u32(me + 0x20, 0x8000_0000);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e0_2a40,
            hk_platform_object_writer_write_raw(Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(0x00e0_2a70, hk_platform_object_writer_get_layout(Ptr) -> u32),
        entry!(
            0x00e0_2a80,
            hk_platform_object_writer_write_object(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e0_2bb0,
            hk_platform_object_writer_construct(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00e0_2c70, hk_platform_object_writer_destruct(Ptr)),
        entry!(
            0x00e0_2cb0,
            hk_platform_object_writer_cache_get(Ptr, u32, u32) -> u32
        ),
        entry!(0x00e0_2d40, hk_platform_object_writer_cache_destruct(Ptr)),
        entry!(
            0x00e0_2de0,
            hk_platform_object_writer_cache_construct(Ptr) -> Ptr
        ),
        entry!(0x00e0_2e40, deleting_destructor_00e02e40(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e02e40_frees_only_when_bit_zero_is_set() {
        check(0x00e0_2e40, 0x00e0_2c70);
    }
}

#[cfg(test)]
mod object_writer_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::rc::Rc;

    const POOL_ALLOC: u32 = 0x0900_0004;
    const POOL_FREE: u32 = 0x0900_0008;
    const POOL_FREE_ARRAY: u32 = 0x0900_0010;
    const SLOT_DOUBLE: u32 = 0x0900_0100;
    const OK_DOUBLE: u32 = 0x0900_0200;
    const CONVERTER_DOUBLE: u32 = 0x0900_0300;
    const TELL_DOUBLE: u32 = 0x0900_0400;
    const WRITER_OK_DOUBLE: u32 = 0x0900_0500;

    /// The memory router answers an allocator whose slots `+4`, `+8` and `+0x10` are doubles:
    /// the first answers a fresh block (0 when asked for `refuse` bytes), all log their
    /// arguments. Answers the allocator.
    fn pool(e: &mut Engine, log: &Log, refuse: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        for (slot, double) in [(4, POOL_ALLOC), (8, POOL_FREE), (0x10, POOL_FREE_ARRAY)] {
            e.mem.set_u32(vtable + slot, double);
        }
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(POOL_ALLOC, move |e, a| {
            sink.borrow_mut().push((POOL_ALLOC, a.to_vec()));
            let block = if a[1] == refuse { 0 } else { e.mem.alloc(a[1]) };
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        log_double(e, log, POOL_FREE, 0);
        log_double(e, log, POOL_FREE_ARRAY, 0);
        allocator
    }

    /// An object with the vtable `slots` (byte offset, function).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, function) in slots {
            e.mem.set_u32(vtable + slot, *function);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn write_raw_reports_a_short_write_00e02a40() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SLOT_DOUBLE, 5);
        let stream = object(&mut e, &[(0x10, SLOT_DOUBLE)]);
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00e0_2a40, &args![this, stream, 0x1234u32, 5u32])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x00e0_2a40, &args![this, stream, 0x1234u32, 6u32])
                .u32(),
            1
        );
        assert_eq!(
            calls(&log, SLOT_DOUBLE),
            vec![vec![stream, 0x1234, 5], vec![stream, 0x1234, 6]]
        );
    }

    #[test]
    fn the_layout_is_inside_the_converter_00e02a70() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x4000);
        assert_eq!(e.call(0x00e0_2a70, &args![this]).u32(), 0x400c);
    }

    /// A writer with a converter (slot `+0xC` logs) and the cache pointer `cache`, a stream
    /// whose slot `+0xC` answers a status byte at `status` and `+0x20` the position `0x44`.
    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        stream: u32,
        converter: u32,
        status: u32,
    }

    fn rig(cache: u32, status_byte: u8) -> Rig {
        let mut e = engine();
        let log = new_log();
        let converter = object(&mut e, &[(0xc, CONVERTER_DOUBLE)]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, converter);
        e.mem.set_u32(this + 0xc, cache);
        let status = e.mem.alloc(4);
        e.mem.set_u8(status, status_byte);
        let stream = object(&mut e, &[(0xc, OK_DOUBLE), (0x20, TELL_DOUBLE)]);
        e.register_double(OK_DOUBLE, move |_, _| Ret {
            eax: status,
            ..Ret::default()
        });
        log_double(&mut e, &log, TELL_DOUBLE, 0x44);
        log_double(&mut e, &log, OFFSET_WRITER_CONSTRUCT, 0);
        Rig {
            e,
            log,
            this,
            stream,
            converter,
            status,
        }
    }

    #[test]
    fn write_object_without_a_cache_writes_to_the_stream_00e02a80() {
        let mut r = rig(0, 0);
        log_double(&mut r.e, &r.log, CONVERTER_DOUBLE, 0);
        let (object, class, extra) = (0x1000u32, 0x2000u32, 0x3000u32);
        let result =
            r.e.call(0x00e0_2a80, &args![r.this, r.stream, object, class, extra])
                .u32();
        assert_eq!(result, 1);
        assert_eq!(
            calls(&r.log, CONVERTER_DOUBLE),
            vec![vec![r.converter, object, class, r.stream, class, extra]]
        );
        assert_eq!(calls(&r.log, OFFSET_WRITER_CONSTRUCT).len(), 1);
        assert!(calls(&r.log, TELL_DOUBLE).is_empty());
        r.e.mem.set_u8(r.status, 1);
        let result =
            r.e.call(0x00e0_2a80, &args![r.this, r.stream, object, class, extra])
                .u32();
        assert_eq!(result, 0);
    }

    /// Doubles for a cache whose map holds every key as `key + 0x100` and for the class
    /// name, which compares as `order` against `"hkClass"`.
    fn cached_names(r: &mut Rig, order: u32) {
        r.e.register_double(MAP_GET, |e, a| {
            e.mem.set_u32(a[2], a[1] + 0x100);
            Ret::default()
        });
        log_double(&mut r.e, &r.log, CLASS_GET_NAME, 0x7000);
        log_double(&mut r.e, &r.log, STRING_COMPARE, order);
        log_double(&mut r.e, &r.log, CONVERTER_DOUBLE, 0);
    }

    #[test]
    fn write_object_with_another_class_name_converts_only_the_class_00e02a80() {
        let cache = 0x5000u32;
        let mut r = rig(cache, 0);
        cached_names(&mut r, 7);
        let result =
            r.e.call(
                0x00e0_2a80,
                &args![r.this, r.stream, 0x1000u32, 0x2000u32, 0x3000u32],
            )
            .u32();
        assert_eq!(result, 1);
        // The class went through the cache (the map answers key + 0x100); its name was
        // compared with "hkClass".
        assert_eq!(calls(&r.log, CLASS_GET_NAME), vec![vec![0x2100]]);
        assert_eq!(
            calls(&r.log, STRING_COMPARE),
            vec![vec![0x7000, CLASS_NAME]]
        );
        assert_eq!(
            calls(&r.log, CONVERTER_DOUBLE),
            vec![vec![r.converter, 0x1000, 0x2000, r.stream, 0x2100, 0x3000]]
        );
        assert!(calls(&r.log, TELL_DOUBLE).is_empty());
    }

    #[test]
    fn write_object_of_a_class_description_goes_through_the_offset_writer_00e02a80() {
        let cache = 0x5000u32;
        let mut r = rig(cache, 0);
        cached_names(&mut r, 0);
        log_double(&mut r.e, &r.log, OFFSET_WRITER_SEEK, 0);
        log_double(&mut r.e, &r.log, RELOCATION_INFO_DESTRUCT, 0);
        // The offset-only writer built in the frame: its slot 3 reports ok through a byte.
        let ok_byte = r.e.mem.alloc(4);
        let writer_vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(writer_vtable + 0xc, WRITER_OK_DOUBLE);
        r.e.register_double(WRITER_OK_DOUBLE, move |_, _| Ret {
            eax: ok_byte,
            ..Ret::default()
        });
        let construct_log = Rc::clone(&r.log);
        r.e.register_double(OFFSET_WRITER_CONSTRUCT, move |e, a| {
            construct_log
                .borrow_mut()
                .push((OFFSET_WRITER_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0], writer_vtable);
            Ret::default()
        });
        // The converter double notes its arguments and, for the relocation record, its words.
        let seen = Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        r.e.register_double(CONVERTER_DOUBLE, move |e, a| {
            let record: Vec<u32> = (0..13).map(|n| e.mem.u32(a[5] + 4 * n)).collect();
            sink.borrow_mut().push((a.to_vec(), record));
            Ret::default()
        });
        let extra = r.e.mem.alloc(0x40);
        let (object, class) = (0x1000u32, 0x2000u32);
        let result =
            r.e.call(0x00e0_2a80, &args![r.this, r.stream, object, class, extra])
                .u32();
        assert_eq!(result, 1);
        let converter = r.e.mem.u32(r.this + 8);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // First: the converted object and class go to the stream with an empty relocation
        // record (four arrays with the not-owned flag and a null pointer).
        let (first, record) = &seen[0];
        assert_eq!(first[..5], [converter, 0x1100, class, r.stream, 0x2100]);
        assert_eq!(
            *record,
            [
                0,
                0,
                0x8000_0000,
                0,
                0,
                0x8000_0000,
                0,
                0,
                0x8000_0000,
                0,
                0,
                0x8000_0000,
                0
            ]
        );
        // The offset-only writer was moved to the stream position, the record destroyed, and
        // the main write went to the writer.
        let writer = calls(&r.log, OFFSET_WRITER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&r.log, OFFSET_WRITER_SEEK),
            vec![vec![writer, 0x44, 0]]
        );
        assert_eq!(
            calls(&r.log, RELOCATION_INFO_DESTRUCT),
            vec![vec![first[5]]]
        );
        assert_eq!(first[5], writer + 0x10);
        let (second, _) = &seen[1];
        assert_eq!(
            second[..6],
            [converter, object, class, writer, 0x2100, extra]
        );
        // The status is read from the writer: a nonzero byte there means "not ok".
        assert_eq!(r.e.mem.u8(ok_byte), 0);
        r.e.mem.set_u8(ok_byte, 3);
        drop(seen);
        let result =
            r.e.call(0x00e0_2a80, &args![r.this, r.stream, object, class, extra])
                .u32();
        assert_eq!(result, 0);
    }

    /// The rules the constructor copies and the converter it builds: the double fills the
    /// target rules at `+0xC` from the word `rules` points to.
    fn construct_doubles(e: &mut Engine, log: &Log) {
        e.set_global(HOST_RULES, 0x0403_0201u32);
        e.register_double(LAYOUT_RULES_COPY, |e, a| {
            let rules = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], rules);
            Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(CONVERTER_CONSTRUCT, move |e, a| {
            sink.borrow_mut()
                .push((CONVERTER_CONSTRUCT, vec![a[0], e.mem.u32(a[1]), a[2], a[3]]));
            let rules = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0xc, rules);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log_double(e, log, MAP_RESERVE, 0);
    }

    #[test]
    fn construct_with_the_host_rules_has_no_cache_00e02bb0() {
        let mut e = engine();
        let log = new_log();
        pool(&mut e, &log, 0);
        construct_doubles(&mut e, &log);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xdead);
        let rules = e.mem.alloc(4);
        e.mem.set_u32(rules, 0x0403_0201);
        assert_eq!(e.call(0x00e0_2bb0, &args![this, rules]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), WRITER_VTABLE);
        let converter = e.mem.u32(this + 8);
        assert_eq!(e.mem.u16(converter + 4), 0x18);
        assert_eq!(
            calls(&log, CONVERTER_CONSTRUCT),
            vec![vec![converter, 0x0403_0201, rules, 2]]
        );
        let sizes: Vec<u32> = calls(&log, POOL_ALLOC).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, [0x18]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn construct_with_other_rules_builds_a_cache_unless_the_allocation_fails_00e02bb0() {
        for (refuse, cache_built) in [(0u32, true), (0x24, false)] {
            for byte in 0..4u32 {
                let mut e = engine();
                let log = new_log();
                pool(&mut e, &log, refuse);
                construct_doubles(&mut e, &log);
                let this = e.mem.alloc(0x20);
                let rules = e.mem.alloc(4);
                let other = 0x0403_0201u32 ^ (0x55 << (8 * byte));
                e.mem.set_u32(rules, other);
                e.call(0x00e0_2bb0, &args![this, rules]);
                let sizes: Vec<u32> = calls(&log, POOL_ALLOC).iter().map(|c| c[1]).collect();
                assert_eq!(sizes, [0x18, 0x24], "byte {byte}");
                let cache = e.mem.u32(this + 0xc);
                if cache_built {
                    assert_ne!(cache, 0);
                    assert_eq!(e.mem.u32(cache + 0x20), 0x8000_0000);
                    assert_eq!(calls(&log, MAP_RESERVE).len(), 2);
                } else {
                    assert_eq!(cache, 0);
                    assert!(calls(&log, MAP_RESERVE).is_empty());
                }
            }
        }
    }

    #[test]
    fn destruct_releases_the_converter_and_frees_the_cache_00e02c70() {
        let mut e = engine();
        let log = new_log();
        let allocator = pool(&mut e, &log, 0);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        log_double(&mut e, &log, MAP_CLEAR_AND_DEALLOCATE, 0);
        log_double(&mut e, &log, NO_OP_DESTRUCTOR, 0);
        log_double(&mut e, &log, EASY_FREE, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x6000);
        e.call(0x00e0_2c70, &args![this]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert!(calls(&log, POOL_FREE).is_empty());
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        let cache = e.mem.alloc(0x24);
        e.mem.set_u32(cache + 0x20, 0x8000_0000);
        e.mem.set_u32(this + 0xc, cache);
        e.call(0x00e0_2c70, &args![this]);
        assert_eq!(calls(&log, POOL_FREE), vec![vec![allocator, cache, 0x24]]);
        assert_eq!(calls(&log, MAP_CLEAR_AND_DEALLOCATE).len(), 2);
    }

    #[test]
    fn cache_get_answers_a_known_key_from_the_map_00e02cb0() {
        let mut e = engine();
        let log = new_log();
        pool(&mut e, &log, 0);
        e.register_double(MAP_GET, |e, a| {
            e.mem.set_u32(a[2], 0xabcd);
            Ret::default()
        });
        log_double(&mut e, &log, DEEP_COPY, 0);
        let cache = e.mem.alloc(0x24);
        assert_eq!(
            e.call(0x00e0_2cb0, &args![cache, 0x77u32, 0x88u32]).u32(),
            0xabcd
        );
        assert!(calls(&log, DEEP_COPY).is_empty());
    }

    #[test]
    fn cache_get_copies_a_new_key_and_grows_a_full_array_00e02cb0() {
        let mut e = engine();
        let log = new_log();
        pool(&mut e, &log, 0);
        log_double(&mut e, &log, MAP_GET, 1);
        log_double(&mut e, &log, DEEP_COPY, 0xc0de);
        log_double(&mut e, &log, LAYOUT_COPY, 0);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_GROW, move |e, a| {
            sink.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let cache = e.mem.alloc(0x24);
        e.mem.set_u32(cache + 0x20, 0x8000_0000);
        assert_eq!(
            e.call(0x00e0_2cb0, &args![cache, 0x77u32, 0x88u32]).u32(),
            0xc0de
        );
        let grows = calls(&log, ARRAY_GROW);
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0][1..], [cache + 0x18, 4]);
        assert_eq!(
            calls(&log, DEEP_COPY),
            vec![vec![0x77, DEEP_COPY_TABLE, cache, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(cache + 0x1c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(cache + 0x18)), 0xc0de);
        assert_eq!(
            calls(&log, LAYOUT_COPY),
            vec![vec![0x88, 0xc0de, cache + 0xc, 1]]
        );
        // With room left the array is not grown and the next copy lands after the first.
        e.call(0x00e0_2cb0, &args![cache, 0x78u32, 0x88u32]);
        assert_eq!(calls(&log, ARRAY_GROW).len(), 1);
        assert_eq!(e.mem.u32(cache + 0x1c), 2);
        assert_eq!(e.mem.u32(e.mem.u32(cache + 0x18) + 4), 0xc0de);
    }

    #[test]
    fn cache_destruct_frees_the_copies_the_buffer_and_the_maps_00e02d40() {
        for flags in [4u32, 0x8000_0004] {
            let mut e = engine();
            let log = new_log();
            let allocator = pool(&mut e, &log, 0);
            for double in [EASY_FREE, MAP_CLEAR_AND_DEALLOCATE, NO_OP_DESTRUCTOR] {
                log_double(&mut e, &log, double, 0);
            }
            let cache = e.mem.alloc(0x24);
            let data = e.mem.alloc(16);
            e.mem.set_u32(data, 0x111);
            e.mem.set_u32(data + 4, 0x222);
            e.mem.set_u32(cache + 0x18, data);
            e.mem.set_u32(cache + 0x1c, 2);
            e.mem.set_u32(cache + 0x20, flags);
            e.call(0x00e0_2d40, &args![cache]);
            assert_eq!(
                calls(&log, EASY_FREE),
                vec![vec![allocator, 0x111], vec![allocator, 0x222]]
            );
            let freed = calls(&log, POOL_FREE_ARRAY);
            if flags & 0x8000_0000 == 0 {
                assert_eq!(freed, vec![vec![allocator, data, 16]]);
            } else {
                assert!(freed.is_empty());
            }
            assert_eq!(
                (
                    e.mem.u32(cache + 0x18),
                    e.mem.u32(cache + 0x1c),
                    e.mem.u32(cache + 0x20)
                ),
                (0, 0, 0x8000_0000)
            );
            assert_eq!(
                calls(&log, MAP_CLEAR_AND_DEALLOCATE),
                vec![vec![cache + 0xc, allocator], vec![cache, allocator]]
            );
            assert_eq!(
                calls(&log, NO_OP_DESTRUCTOR),
                vec![vec![cache + 0xc], vec![cache]]
            );
        }
    }

    #[test]
    fn cache_destruct_of_an_empty_cache_frees_no_copies_00e02d40() {
        let mut e = engine();
        let log = new_log();
        pool(&mut e, &log, 0);
        for double in [EASY_FREE, MAP_CLEAR_AND_DEALLOCATE, NO_OP_DESTRUCTOR] {
            log_double(&mut e, &log, double, 0);
        }
        let cache = e.mem.alloc(0x24);
        e.mem.set_u32(cache + 0x20, 0x8000_0000);
        e.call(0x00e0_2d40, &args![cache]);
        assert!(calls(&log, EASY_FREE).is_empty());
        assert!(calls(&log, POOL_FREE_ARRAY).is_empty());
    }

    #[test]
    fn cache_construct_empties_both_maps_and_the_array_00e02de0() {
        let mut e = engine();
        let log = new_log();
        let allocator = pool(&mut e, &log, 0);
        log_double(&mut e, &log, MAP_RESERVE, 0);
        let cache = e.mem.alloc(0x24);
        e.mem.write(cache, &[0xcd; 0x24]);
        assert_eq!(e.call(0x00e0_2de0, &args![cache]).u32(), cache);
        for map in [cache, cache + 0xc] {
            assert_eq!(
                (e.mem.u32(map), e.mem.u32(map + 4), e.mem.u32(map + 8)),
                (0, 0, 0xffff_ffff)
            );
        }
        assert_eq!(
            (
                e.mem.u32(cache + 0x18),
                e.mem.u32(cache + 0x1c),
                e.mem.u32(cache + 0x20)
            ),
            (0, 0, 0x8000_0000)
        );
        assert_eq!(
            calls(&log, MAP_RESERVE),
            vec![vec![cache, allocator, 0], vec![cache + 0xc, allocator, 0]]
        );
    }
}

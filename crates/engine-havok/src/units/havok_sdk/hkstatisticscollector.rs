//! `Havok SDK/hkstatisticscollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStatisticsCollector` (RTTI vtable `010cce98`) is abstract: its slot `0x14` (`addChunk`, five
//! words: a kind, then the arguments) is pure and the helpers here call it. The collector keeps
//! the object that maps an object to its `hkClass` at `+0xc` and a flag byte at `+0x10`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its `+0x10` is the allocator whose slot `0x2c` answers the size
/// of an allocation.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkClass::getObjectSize` (Xbox PDB name, `thiscall`, no stack words).
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
/// `hkString::strLen` (cdecl).
const STRING_LEN: u32 = 0x00cb_10d0;
/// `hkStatisticsCollector`'s vtable (RTTI-confirmed) and the `hkBaseObject` vtable a destroyed
/// collector is left with.
const COLLECTOR_VTABLE: u32 = 0x010c_ce98;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The kind words `addChunk` is given.
const CHUNK_KIND_NORMAL: u32 = 1;
const CHUNK_KIND_STRING: u32 = 3;

// Translated from 00d06d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollector::getReferencedObjectSize` (Xbox PDB), `thiscall`, `RET 4`: the size of a
/// referenced `object`. An object whose memory-size field (the 16-bit word at `+4`, low 15
/// bits) is not 0 answers that size. Otherwise, when the flag byte at `+0x10` is set, the
/// collector asks the object at `+0xc` for the object's class (slot `0x10`) and answers its
/// `hkClass::getObjectSize`, or 0 without a class; with the flag clear it answers 0.
pub fn hk_statistics_collector_get_referenced_object_size(
    e: &mut Engine,
    this: Ptr,
    object: u32,
) -> u32 {
    let base = this.addr();
    let size = u32::from(e.mem.u16(object + 4)) & 0x7fff;
    if size != 0 || e.mem.u8(base + 0x10) == 0 {
        return size;
    }
    let finder = e.mem.u32(base + 0xc);
    let class = e.vcall(finder, 0x10, &args![object]).u32();
    if class == 0 {
        return 0;
    }
    e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32()
}

// Translated from 00d06f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollector`'s constructor (no map name; the vtable it stores is RTTI-confirmed
/// `hkStatisticsCollector`), `thiscall`, `RET 4`: stores the 16-bit word 1 at `+6`, the vtable at
/// `+0`, `class_finder` at `+0xc` (the object whose slot `0x10` maps an object to its class;
/// its identity is unconfirmed) and clears the flag byte at `+0x10`. Returns `this`.
pub fn hk_statistics_collector_constructor(e: &mut Engine, this: Ptr, class_finder: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, COLLECTOR_VTABLE);
    e.mem.set_u32(base + 0xc, class_finder);
    e.mem.set_u8(base + 0x10, 0);
    this
}

// Translated from 00d06f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the destructor body of `hkStatisticsCollector`). `ECX` = `this`, no stack words,
/// plain `RET`: clears the word at `+8` and stores the `hkBaseObject` vtable.
pub fn fn_00d06f70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, BASE_OBJECT_VTABLE);
}

// Translated from 00d06f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollector::getAllocatedSize` (Xbox PDB), cdecl, `(ptr, size)`: answers `size`
/// for a null `ptr`; otherwise asks the memory router's allocator (slot `0x2c`, arguments `(ptr,
/// size)`) how much the allocation really takes and answers that.
pub fn hk_statistics_collector_get_allocated_size(e: &mut Engine, ptr: u32, size: u32) -> u32 {
    if ptr == 0 {
        return size;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 0x2c, &args![ptr, size]).u32()
}

// Translated from 00d06fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollector::addNormalChunk` (Xbox PDB), `thiscall`, `RET 0x10`; the parameter names
/// follow how the code uses them: a chunk `name`, its address `chunk`, `used_size` and
/// `allocated_size`. When `used_size` is above 0 and `allocated_size` is 0, the allocated size is
/// looked up (`getAllocatedSize(chunk, used_size)`). Then calls slot `0x14` (`addChunk`) with
/// `(1, name, chunk, used_size, allocated_size)`.
pub fn hk_statistics_collector_add_normal_chunk(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    chunk: u32,
    used_size: i32,
    allocated_size: u32,
) {
    let mut allocated_size = allocated_size;
    if used_size > 0 && allocated_size == 0 {
        allocated_size = hk_statistics_collector_get_allocated_size(e, chunk, used_size as u32);
    }
    e.vcall(
        this.addr(),
        0x14,
        &args![CHUNK_KIND_NORMAL, name, chunk, used_size, allocated_size],
    );
}

// Translated from 00d07030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsCollector::addStringPtr` (Xbox PDB), `thiscall`, `RET 8`: `string_ptr` points to
/// an `hkStringPtr`; when its text (the word with the flag bit cleared) is not null, the length is
/// taken (`strLen`), the allocated size looked up (`getAllocatedSize(text, length)`) and slot
/// `0x14` (`addChunk`) called with `(3, name, text, length, allocated size)`.
pub fn hk_statistics_collector_add_string_ptr(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    string_ptr: u32,
) {
    let text = e.mem.u32(string_ptr) & 0xffff_fffe;
    if text == 0 {
        return;
    }
    let length = e.call(STRING_LEN, &args![text]).u32();
    let allocated = hk_statistics_collector_get_allocated_size(e, text, length);
    e.vcall(
        this.addr(),
        0x14,
        &args![CHUNK_KIND_STRING, name, text, length, allocated],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d06d90, hk_statistics_collector_get_referenced_object_size(Ptr, u32) -> u32),
        entry!(0x00d06f50, hk_statistics_collector_constructor(Ptr, u32) -> Ptr),
        entry!(0x00d06f70, fn_00d06f70(Ptr)),
        entry!(0x00d06f80, hk_statistics_collector_get_allocated_size(u32, u32) -> u32),
        entry!(
            0x00d06fb0,
            hk_statistics_collector_add_normal_chunk(Ptr, u32, u32, i32, u32)
        ),
        entry!(
            0x00d07030,
            hk_statistics_collector_add_string_ptr(Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Chunks = Rc<RefCell<Vec<Vec<u32>>>>;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A collector whose slot `0x14` logs `addChunk`'s arguments, over a router whose
    /// allocator's slot `0x2c` answers `size + 0x10` and logs `(ptr, size)`.
    fn collector(e: &mut Engine) -> (u32, Chunks, Chunks) {
        let chunks: Chunks = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&chunks);
        e.register_double(0x0d00_0014, move |_, a| {
            log.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, 0x0d00_0014);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        let sizes: Chunks = Rc::new(RefCell::new(Vec::new()));
        let asked = Rc::clone(&sizes);
        e.register_double(0x0d01_002c, move |_, a| {
            asked.borrow_mut().push(a[1..].to_vec());
            answer(a[2] + 0x10)
        });
        let allocator_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(allocator_vtable + 0x2c, 0x0d01_002c);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| answer(router));
        (this, chunks, sizes)
    }

    #[test]
    fn referenced_object_size_prefers_the_size_field_then_the_class() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        // A size field with the flag bit 0x8000 set: the low 15 bits.
        let sized = e.mem.alloc(0x10);
        e.mem.set_u16(sized + 4, 0x8040);
        assert_eq!(e.call(0x00d0_6d90, &args![this, sized]).u32(), 0x40);
        // No size field and the flag byte clear: 0, the finder is not asked.
        let unsized_object = e.mem.alloc(0x10);
        e.mem.set_u16(unsized_object + 4, 0x8000);
        assert_eq!(e.call(0x00d0_6d90, &args![this, unsized_object]).u32(), 0);
        // With the flag byte set the finder (slot 0x10) is asked for the object's class.
        e.mem.set_u8(this + 0x10, 1);
        e.register_double(0x0d02_0010, move |_, a| {
            answer(if a[1] == unsized_object { 0xc1a5 } else { 0 })
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0d02_0010);
        let finder = e.mem.alloc(0x10);
        e.mem.set_u32(finder, vtable);
        e.mem.set_u32(this + 0xc, finder);
        e.register(CLASS_GET_OBJECT_SIZE, |_, a| answer(a[0] + 1));
        assert_eq!(
            e.call(0x00d0_6d90, &args![this, unsized_object]).u32(),
            0xc1a6
        );
        // An object the finder knows no class of: 0. One with a size field is not looked up.
        let unknown = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d0_6d90, &args![this, unknown]).u32(), 0);
        assert_eq!(e.call(0x00d0_6d90, &args![this, sized]).u32(), 0x40);
    }

    #[test]
    fn constructor_and_destructor_body_store_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x10, 9);
        assert_eq!(e.call(0x00d0_6f50, &args![this, 0xf1deu32]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), COLLECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0xf1de);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        e.mem.set_u32(this + 8, 0x77);
        e.call(0x00d0_6f70, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
    }

    #[test]
    fn allocated_size_asks_the_allocator_for_a_real_pointer_only() {
        let mut e = Engine::new();
        let (_, _, sizes) = collector(&mut e);
        assert_eq!(e.call(0x00d0_6f80, &args![0u32, 0x33u32]).u32(), 0x33);
        assert!(sizes.borrow().is_empty());
        assert_eq!(e.call(0x00d0_6f80, &args![0x1000u32, 0x30u32]).u32(), 0x40);
        assert_eq!(*sizes.borrow(), vec![vec![0x1000, 0x30]]);
    }

    #[test]
    fn add_normal_chunk_looks_up_a_missing_allocated_size() {
        let mut e = Engine::new();
        let (this, chunks, sizes) = collector(&mut e);
        e.call(0x00d0_6fb0, &args![this, 0xaau32, 0x1000u32, 0x30i32, 0u32]);
        assert_eq!(*chunks.borrow(), vec![vec![1, 0xaa, 0x1000, 0x30, 0x40]]);
        assert_eq!(sizes.borrow().len(), 1);
        // A given allocated size is kept; a size of 0 or less is not looked up either.
        e.call(
            0x00d0_6fb0,
            &args![this, 0xaau32, 0x1000u32, 0x30i32, 0x99u32],
        );
        e.call(0x00d0_6fb0, &args![this, 0xaau32, 0x1000u32, 0i32, 0u32]);
        e.call(0x00d0_6fb0, &args![this, 0xaau32, 0x1000u32, -4i32, 0u32]);
        assert_eq!(sizes.borrow().len(), 1);
        let log = chunks.borrow();
        assert_eq!(log[1], vec![1, 0xaa, 0x1000, 0x30, 0x99]);
        assert_eq!(log[2], vec![1, 0xaa, 0x1000, 0, 0]);
        assert_eq!(log[3], vec![1, 0xaa, 0x1000, 0xffff_fffc, 0]);
    }

    #[test]
    fn add_string_ptr_reports_the_text_with_its_length_and_allocated_size() {
        let mut e = Engine::new();
        let (this, chunks, _) = collector(&mut e);
        e.register(STRING_LEN, |e, a| answer(e.mem.cstr(a[0]).len() as u32));
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"hello");
        let string = e.mem.alloc(4);
        // The owned flag in bit 0 is cleared.
        e.mem.set_u32(string, text | 1);
        e.call(0x00d0_7030, &args![this, 0xbbu32, string]);
        assert_eq!(*chunks.borrow(), vec![vec![3, 0xbb, text, 5, 0x15]]);
        // A null text (or just the flag bit) reports nothing.
        e.mem.set_u32(string, 1);
        e.call(0x00d0_7030, &args![this, 0xbbu32, string]);
        e.mem.set_u32(string, 0);
        e.call(0x00d0_7030, &args![this, 0xbbu32, string]);
        assert_eq!(chunks.borrow().len(), 1);
    }
}

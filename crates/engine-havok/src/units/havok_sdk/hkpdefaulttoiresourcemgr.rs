//! `Havok SDK/hkpdefaulttoiresourcemgr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpDefaultToiResourceMgr` is not in the Xbox PDB; the layout is read off the code
//! (descriptive, unconfirmed): `+8` is the size of the buffer it hands out and `+0xc` the size
//! it starts from. The resource record the begin/end functions fill in (the last stack word) has
//! the buffer size at `+0x1c`, the buffer at `+0x18` and six counters at `+0` to `+0x14`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor installs, and the value it gives `+0xc`.
const VTABLE: u32 = 0x010d_4310;
const START_SIZE: u32 = 0x0002_0000;
/// The memory router (`00c85750`): its `+0x18` is an object whose slot `0xc` allocates a
/// buffer (taking the address of a size word it may change) and whose slot `0x10` frees one
/// (block, size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const SLOT_ALLOCATE_BUFFER: u32 = 0x0c;
const SLOT_FREE_BUFFER: u32 = 0x10;
/// The string `calcContentStatistics` hands the collector's slot `0x18`.
const STATISTICS_LABEL: u32 = 0x010d_42f0;
/// The value the counters of a new resource record start from.
const COUNTER_START: u32 = 0x3e8;

// Translated from 00d9fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no name in the engine map: `ECX` = manager (unused), two stack words: the
/// address of a byte and one never read. Sets the byte to 1 and returns its address.
pub fn fn_00d9fb40(e: &mut Engine, _this: Ptr, flag: Ptr, _unused_2: u32) -> Ptr {
    e.mem.set_u8(flag.addr(), 1);
    flag
}

// Translated from 00d9fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::calcContentStatistics` (Ghidra name): `ECX` = manager (unused),
/// two stack words: the collector and one never read. Calls the collector's slot `0x18` with
/// the string at `010d42f0` and then its slot `0x1c` with nothing.
pub fn hkp_default_toi_resource_mgr_calc_content_statistics(
    e: &mut Engine,
    _this: Ptr,
    collector: u32,
    _unused_2: u32,
) {
    e.vcall(collector, 0x18, &args![STATISTICS_LABEL]);
    e.vcall(collector, 0x1c, &args![]);
}

// Translated from 00d9fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::resourcesDepleted` (Ghidra name): returns 1.
pub fn hkp_default_toi_resource_mgr_resources_depleted(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00d9fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::cannotSolve` (Ghidra name): `ECX` = manager, one stack word,
/// neither read; returns 0.
pub fn hkp_default_toi_resource_mgr_cannot_solve(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) -> u32 {
    0
}

// Translated from 00d9fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no name in the engine map; `ECX` = object, nothing on the stack): sets the
/// 16-bit word at `+6` to 1, the vtable `010d4310` and `+0xc` to 0x20000. Returns `this`.
pub fn fn_00d9fb90(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_u32(me + 0xc, START_SIZE);
    this
}

// Translated from 00d9fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::beginToiAndSetupResources` (Ghidra name): `ECX` = manager, three
/// stack words: `first`, one never read, and the resource record. The code uses the stack word
/// that held `first` as scratch: `fn_00d9fb40` sets its low byte to 1 (a zero byte there would
/// make the function return 1 at once), and later it holds the buffer size. The record's `+0x1c`
/// and the manager's `+8` take the start size from `+0xc`; the memory router's `+0x18`
/// object allocates a buffer for the size at `+8` (slot `0xc`, which may change the size, stored
/// back at `+8`); the record gets the buffer at `+0x18`, the counters at `+4`, `+8` and `+0xc`
/// set to 1000, `+0x10` to 3, `+0x14` to 4 and `+0` to 3. Returns 0.
pub fn hkp_default_toi_resource_mgr_begin_toi_and_setup_resources(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    _unused_2: u32,
    record: Ptr,
) -> u32 {
    let (me, record) = (this.addr(), record.addr());
    e.with_stack(4, |e, scratch| {
        e.mem.set_u32(scratch.addr(), first);
        let flag = fn_00d9fb40(e, this, scratch, first);
        if e.mem.u8(flag.addr()) == 0 {
            return 1;
        }
        let start = e.mem.u32(me + 0xc);
        e.mem.set_u32(record + 0x1c, start);
        e.mem.set_u32(me + 8, start);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x18);
        e.mem.set_u32(scratch.addr(), start);
        let buffer = e
            .vcall(allocator, SLOT_ALLOCATE_BUFFER, &args![scratch.addr()])
            .u32();
        let size = e.mem.u32(scratch.addr());
        e.mem.set_u32(me + 8, size);
        e.mem.set_u32(record + 0x18, buffer);
        for offset in [8, 0xc, 4] {
            e.mem.set_u32(record + offset, COUNTER_START);
        }
        e.mem.set_u32(record + 0x10, 3);
        e.mem.set_u32(record + 0x14, 4);
        e.mem.set_u32(record, 3);
        0
    })
}

// Translated from 00d9fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::endToiAndFreeResources` (Ghidra name): `ECX` = manager, three
/// stack words: two never read and the resource record. Frees the record's buffer (`+0x18`)
/// through the memory router's `+0x18` object (slot `0x10`, with the size at the manager's
/// `+8`).
pub fn hkp_default_toi_resource_mgr_end_toi_and_free_resources(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    record: Ptr,
) {
    let size = e.mem.u32(this.addr() + 8);
    let buffer = e.mem.u32(record.addr() + 0x18);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x18);
    e.vcall(allocator, SLOT_FREE_BUFFER, &args![buffer, size]);
}

// Translated from 00d9fc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDefaultToiResourceMgr::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_955c.
pub fn hkp_default_toi_resource_mgr_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_955c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9fb40, fn_00d9fb40(Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x00d9fb50,
            hkp_default_toi_resource_mgr_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(
            0x00d9fb70,
            hkp_default_toi_resource_mgr_resources_depleted(Ptr) -> u32
        ),
        entry!(
            0x00d9fb80,
            hkp_default_toi_resource_mgr_cannot_solve(Ptr, u32) -> u32
        ),
        entry!(0x00d9fb90, fn_00d9fb90(Ptr) -> Ptr),
        entry!(
            0x00d9fbb0,
            hkp_default_toi_resource_mgr_begin_toi_and_setup_resources(Ptr, u32, u32, Ptr) -> u32
        ),
        entry!(
            0x00d9fc30,
            hkp_default_toi_resource_mgr_end_toi_and_free_resources(Ptr, u32, u32, Ptr)
        ),
        entry!(0x00d9fc60, hkp_default_toi_resource_mgr_get_class_type() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, logging_object, new_log, Log,
    };

    #[test]
    fn hkp_default_toi_resource_mgr_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d9_fc60, &args![]).u32(), 0x0126_955c);
    }

    #[test]
    fn flag_function_sets_the_byte_and_returns_its_address() {
        let mut e = engine();
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, 0x1234_5600);
        assert_eq!(e.call(0x00d9_fb40, &args![0u32, cell, 0u32]).u32(), cell);
        assert_eq!(e.mem.u32(cell), 0x1234_5601);
    }

    #[test]
    fn statistics_opens_and_closes_the_collector_section() {
        let mut e = engine();
        let log = new_log();
        let collector = logging_object(&mut e, &log, 0x0900_0000);
        e.call(0x00d9_fb50, &args![0u32, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0018, vec![collector, 0x010d_42f0]),
                (0x0900_001c, vec![collector]),
            ]
        );
    }

    #[test]
    fn small_answers_are_constants() {
        let mut e = engine();
        assert_eq!(e.call(0x00d9_fb70, &args![0u32]).u32(), 1);
        assert_eq!(e.call(0x00d9_fb80, &args![0u32, 7u32]).u32(), 0);
    }

    #[test]
    fn constructor_sets_the_version_word_the_vtable_and_the_start_size() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d9_fb90, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_4310);
        assert_eq!(e.mem.u32(this + 0xc), 0x20000);
    }

    /// A memory router whose `+0x18` object logs both slots; the allocation slot returns
    /// `block` and changes the size word it is given to `granted`.
    fn router(e: &mut Engine, log: &Log, block: u32, granted: u32) -> u32 {
        let allocator = logging_object(e, log, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x18, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = std::rc::Rc::clone(log);
        e.register_double(0x0900_000c, move |e, a| {
            sink.borrow_mut()
                .push((0x0900_000c, vec![a[0], e.mem.u32(a[1])]));
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        allocator
    }

    #[test]
    fn begin_asks_for_a_buffer_and_fills_the_record() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log, 0xb000, 0x1_8000);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x2_0000);
        let record = e.mem.alloc(0x40);
        let result = e
            .call(0x00d9_fbb0, &args![this, 0x55u32, 0u32, record])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(
            *log.borrow(),
            vec![(0x0900_000c, vec![allocator, 0x2_0000])]
        );
        assert_eq!(e.mem.u32(this + 8), 0x1_8000);
        assert_eq!(e.mem.u32(record + 0x1c), 0x2_0000);
        assert_eq!(e.mem.u32(record + 0x18), 0xb000);
        let counters: Vec<u32> = (0..6).map(|word| e.mem.u32(record + 4 * word)).collect();
        assert_eq!(counters, vec![3, 1000, 1000, 1000, 3, 4]);
    }

    #[test]
    fn end_frees_the_buffer_with_the_size_kept_at_eight() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log, 0, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1_8000);
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record + 0x18, 0xb000);
        e.call(0x00d9_fc30, &args![this, 0u32, 0u32, record]);
        assert_eq!(
            calls(&log, 0x0900_0010),
            vec![vec![allocator, 0xb000, 0x1_8000]]
        );
    }
}

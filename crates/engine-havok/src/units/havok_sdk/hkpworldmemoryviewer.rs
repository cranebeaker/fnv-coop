//! `Havok SDK/hkpworldmemoryviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `hkpWorldViewerBase` constructor (`00ddccb0`, `this`, `RET 4`: the process contexts) and
/// destructor (`00ddcc60`, `this`).
const BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
const BASE_DESTRUCT: u32 = 0x00dd_cc60;
/// `hkStreamStatisticsCollector`'s constructor (Xbox PDB, `00ddf170`, `this`, `RET 8`: two
/// words) and destructor (`00ddf240`, `this`).
const COLLECTOR_CONSTRUCT: u32 = 0x00dd_f170;
const COLLECTOR_DESTRUCT: u32 = 0x00dd_f240;
/// Three functions of the object at `this + 0x20` (`this`): `00ddf050` (no arguments),
/// `00ddf030` (`RET 4`, one word) and `00ddf040` (no arguments).
const STREAM_BEGIN: u32 = 0x00dd_f050;
const STREAM_SET_FLAG: u32 = 0x00dd_f030;
const STREAM_END: u32 = 0x00dd_f040;
/// The class's three vtable pointers, stored at `+0`, `+8` and `+0x20`.
const VTABLE_PRIMARY: u32 = 0x010d_1794;
const VTABLE_SECONDARY: u32 = 0x010d_1774;
const VTABLE_STREAM: u32 = 0x010d_1764;
/// The second word handed to the statistics collector's constructor (`0x7a120`).
const COLLECTOR_LIMIT: u32 = 500_000;
/// The string `"World"` the dump passes to the stream object's virtual function at +0xC.
const WORLD_TEXT: u32 = 0x0108_2c9c;
/// The process name `"World Memory"`, its creation function and the process factory
/// object's word with the word that receives the registered process id.
const PROCESS_NAME: u32 = 0x010d_1750;
const PROCESS_CREATE: u32 = 0x00d6_32c0;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const PROCESS_ID: u32 = 0x0126_8fc8;
/// `hkProcessFactory::registerProcess` (Xbox PDB, `00dd9380`, `this`, `RET 8`).
const REGISTER_PROCESS: u32 = 0x00dd_9380;

// Translated from 00d63280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldMemoryViewer::hkpWorldMemoryViewer` (Xbox PDB), `RET 8`: runs the
/// `hkpWorldViewerBase` constructor (`00ddccb0`) on `this` with `process_contexts`, stores the
/// class's three vtable pointers at +0, +8 and +0x20, and constructs the
/// `hkStreamStatisticsCollector` at `this + 0x28` (`00ddf170`) with `collector_arg` and
/// 500000. Returns `this`.
pub fn hkp_world_memory_viewer_construct(
    e: &mut Engine,
    this: Ptr,
    collector_arg: u32,
    process_contexts: Ptr,
) -> Ptr {
    let me = this.addr();
    e.call(BASE_CONSTRUCT, &args![this, process_contexts]);
    e.mem.set_u32(me, VTABLE_PRIMARY);
    e.mem.set_u32(me + 8, VTABLE_SECONDARY);
    e.mem.set_u32(me + 0x20, VTABLE_STREAM);
    e.call(
        COLLECTOR_CONSTRUCT,
        &args![me + 0x28, collector_arg, COLLECTOR_LIMIT],
    );
    this
}

// Translated from 00d63300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall, no stack arguments: for each entry of the array at `world + 0x6c` (count at
/// `world + 0x70`, `world` being the word at `this + 0x1c`; zero skips everything) runs the
/// virtual function at +4 of the object embedded at `this + 0x18` with the entry. The world
/// word and the count are read again after every call.
pub fn fn_00d63300(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let mut world = e.mem.u32(me + 0x1c);
    if world == 0 {
        return;
    }
    let mut index = 0i32;
    if e.mem.i32(world + 0x70) <= index {
        return;
    }
    loop {
        let array = e.mem.u32(world + 0x6c);
        let entry = e.mem.u32(array + index as u32 * 4);
        e.vcall(me + 0x18, 4, &args![entry]);
        world = e.mem.u32(me + 0x1c);
        index += 1;
        if index >= e.mem.i32(world + 0x70) {
            break;
        }
    }
}

// Translated from 00d63340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (fastcall): restores the class's three vtable pointers (+0, +8, +0x20), runs the
/// statistics collector's destructor (`00ddf240`) on `this + 0x28` and ends with the
/// `hkpWorldViewerBase` destructor (`00ddcc60`) on `this` (a tail jump in the exe).
pub fn fn_00d63340(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_PRIMARY);
    e.mem.set_u32(me + 8, VTABLE_SECONDARY);
    e.mem.set_u32(me + 0x20, VTABLE_STREAM);
    e.call(COLLECTOR_DESTRUCT, &args![me + 0x28]);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00d63370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall with one stack word that is never read (`RET 4`): writes the world's contents
/// to the stream object embedded at `this + 0x20`. Runs `00ddf050`, then `00ddf030(0)`, then
/// for each entry of the array at `world + 0x6c` (count at `world + 0x70`, `world` the word at
/// `this + 0x1c`, read again after every call) the stream's virtual function at +0xC with
/// (`"World"`, entry, 0), then `00ddf040`. If the stream's size (`this + 0x38`) is positive
/// and its data pointer (`this + 0x34`) is not 0, the virtual function at +0x40 of the object
/// at `this + 0x10` gets (data pointer, size). The branch that would pass 0 for a size that
/// is not positive follows the exe's second test of the size and cannot be reached after the
/// first.
pub fn fn_00d63370(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let me = this.addr();
    let stream = me + 0x20;
    e.call(STREAM_BEGIN, &args![stream]);
    e.call(STREAM_SET_FLAG, &args![stream, 0u32]);
    let mut world = e.mem.u32(me + 0x1c);
    let mut index = 0i32;
    if e.mem.i32(world + 0x70) > index {
        loop {
            let array = e.mem.u32(world + 0x6c);
            let entry = e.mem.u32(array + index as u32 * 4);
            e.vcall(stream, 0xc, &args![WORLD_TEXT, entry, 0u32]);
            world = e.mem.u32(me + 0x1c);
            index += 1;
            if index >= e.mem.i32(world + 0x70) {
                break;
            }
        }
    }
    e.call(STREAM_END, &args![stream]);
    if e.mem.i32(stream + 0x18) > 0 && e.mem.u32(stream + 0x14) != 0 {
        let size = e.mem.u32(me + 0x38);
        let sink = e.mem.u32(me + 0x10);
        if e.mem.i32(stream + 0x18) > 0 {
            let data = e.mem.u32(stream + 0x14);
            e.vcall(sink, 0x40, &args![data, size]);
        } else {
            e.vcall(sink, 0x40, &args![0u32, size]);
        }
    }
}

// Translated from 00d63400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldMemoryViewer::registerViewer` (Xbox PDB): registers the process `"World Memory"`
/// with the process factory held in the word at `0x01269cd0` (creation function `00d632c0`) and
/// stores the answer in the word at `0x01268fc8`.
pub fn hkp_world_memory_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATE],
        )
        .u32();
    e.mem.set_u32(PROCESS_ID, id);
}

// Translated from 00d632c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x54 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d63280` on the block with the words 0 and the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d63406.
pub fn fn_00d632c0(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x54]).u32();
    e.mem.set_u16(block + 4, 0x54);
    let object = e
        .call(0x00d6_3280, &args![block, 0u32, process_contexts])
        .u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d6_3280,
            hkp_world_memory_viewer_construct(Ptr, u32, Ptr) -> Ptr
        ),
        entry!(0x00d6_3300, fn_00d63300(Ptr)),
        entry!(0x00d6_3340, fn_00d63340(Ptr)),
        entry!(0x00d6_3370, fn_00d63370(Ptr, u32)),
        entry!(0x00d6_3400, hkp_world_memory_viewer_register_viewer()),
        entry!(0x00d6_32c0, fn_00d632c0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable has `hooks` at the given byte offsets.
    fn object_with(e: &mut Engine, hooks: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x80);
        for &(slot, target) in hooks {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// A world with the given entries: array at +0x6c, count at +0x70.
    fn world_with(e: &mut Engine, entries: &[u32]) -> u32 {
        let world = e.mem.alloc(0x80);
        let array = e.mem.alloc(0x40);
        for (i, &entry) in entries.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, entry);
        }
        e.mem.set_u32(world + 0x6c, array);
        e.mem.set_u32(world + 0x70, entries.len() as u32);
        world
    }

    #[test]
    fn constructor_builds_the_base_then_stores_the_vtables_and_the_collector() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, BASE_CONSTRUCT);
        log_double(&mut e, &log, COLLECTOR_CONSTRUCT);
        let this = e.mem.alloc(0x80);
        let got = e
            .call(0x00d6_3280, &args![this, 0x1111u32, 0x2222u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(calls(&log, BASE_CONSTRUCT), vec![vec![this, 0x2222]]);
        assert_eq!(
            calls(&log, COLLECTOR_CONSTRUCT),
            vec![vec![this + 0x28, 0x1111, 500_000]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_PRIMARY);
        assert_eq!(e.mem.u32(this + 8), VTABLE_SECONDARY);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_STREAM);
    }

    #[test]
    fn each_world_entry_goes_to_the_embedded_object() {
        let mut e = engine();
        let log: Log = Rc::default();
        let hook = 0x0900_0001;
        let sink = Rc::clone(&log);
        e.register_double(hook, move |e, a| {
            // The world has two entries; the second call sees the count dropped to 2
            // already, the first sees it was 3 before the callback shrinks it.
            sink.borrow_mut().push((hook, a.to_vec()));
            let world = e.mem.u32(a[0] - 0x18 + 0x1c);
            if a[1] == 0x11 {
                e.mem.set_u32(world + 0x70, 2);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        // No world: nothing happens.
        e.call(0x00d6_3300, &args![this]);
        assert!(calls(&log, hook).is_empty());
        let world = world_with(&mut e, &[0x11, 0x22, 0x33]);
        e.mem.set_u32(this + 0x1c, world);
        let embedded = this + 0x18;
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, hook);
        e.mem.set_u32(embedded, table);
        e.call(0x00d6_3300, &args![this]);
        assert_eq!(
            calls(&log, hook),
            vec![vec![embedded, 0x11], vec![embedded, 0x22]]
        );
        // An empty world does not call.
        log.borrow_mut().clear();
        e.mem.set_u32(world + 0x70, 0);
        e.call(0x00d6_3300, &args![this]);
        assert!(calls(&log, hook).is_empty());
    }

    #[test]
    fn destructor_restores_the_vtables_and_ends_in_the_base_destructor() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, COLLECTOR_DESTRUCT);
        log_double(&mut e, &log, BASE_DESTRUCT);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, 1);
        e.call(0x00d6_3340, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_PRIMARY);
        assert_eq!(e.mem.u32(this + 8), VTABLE_SECONDARY);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_STREAM);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![COLLECTOR_DESTRUCT, BASE_DESTRUCT]);
        assert_eq!(calls(&log, COLLECTOR_DESTRUCT), vec![vec![this + 0x28]]);
        assert_eq!(calls(&log, BASE_DESTRUCT), vec![vec![this]]);
    }

    /// A viewer with a stream object at +0x20 (virtual function +0xC logged), a sink object at
    /// +0x10 (virtual function +0x40 logged) and a world with `entries`.
    fn viewer(e: &mut Engine, log: &Log, entries: &[u32]) -> u32 {
        let (write_hook, sink_hook) = (0x0900_0002, 0x0900_0003);
        log_double(e, log, write_hook);
        log_double(e, log, sink_hook);
        for hook in [STREAM_BEGIN, STREAM_SET_FLAG, STREAM_END] {
            log_double(e, log, hook);
        }
        let this = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0xc, write_hook);
        e.mem.set_u32(this + 0x20, table);
        let sink = object_with(e, &[(0x40, sink_hook)]);
        e.mem.set_u32(this + 0x10, sink);
        let world = world_with(e, entries);
        e.mem.set_u32(this + 0x1c, world);
        this
    }

    #[test]
    fn dump_writes_each_world_entry_and_hands_the_stream_data_on() {
        let mut e = engine();
        let log: Log = Rc::default();
        let this = viewer(&mut e, &log, &[0x11, 0x22]);
        e.mem.set_u32(this + 0x34, 0x7000);
        e.mem.set_u32(this + 0x38, 12);
        e.call(0x00d6_3370, &args![this, 0xdeadu32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                STREAM_BEGIN,
                STREAM_SET_FLAG,
                0x0900_0002,
                0x0900_0002,
                STREAM_END,
                0x0900_0003
            ]
        );
        assert_eq!(calls(&log, STREAM_BEGIN), vec![vec![this + 0x20]]);
        assert_eq!(calls(&log, STREAM_SET_FLAG), vec![vec![this + 0x20, 0]]);
        assert_eq!(
            calls(&log, 0x0900_0002),
            vec![
                vec![this + 0x20, WORLD_TEXT, 0x11, 0],
                vec![this + 0x20, WORLD_TEXT, 0x22, 0]
            ]
        );
        let sink = e.mem.u32(this + 0x10);
        assert_eq!(calls(&log, 0x0900_0003), vec![vec![sink, 0x7000, 12]]);
    }

    #[test]
    fn dump_skips_the_sink_without_data_or_size() {
        let mut e = engine();
        let log: Log = Rc::default();
        let this = viewer(&mut e, &log, &[]);
        // No size, no data pointer.
        e.call(0x00d6_3370, &args![this, 0u32]);
        assert!(calls(&log, 0x0900_0003).is_empty());
        assert!(calls(&log, 0x0900_0002).is_empty());
        assert_eq!(calls(&log, STREAM_END).len(), 1);
        // A size without data pointer, and data with a size of 0 or less, are skipped too.
        e.mem.set_u32(this + 0x38, 4);
        e.call(0x00d6_3370, &args![this, 0u32]);
        e.mem.set_u32(this + 0x34, 0x7000);
        e.mem.set_u32(this + 0x38, 0xffff_fffc);
        e.call(0x00d6_3370, &args![this, 0u32]);
        assert!(calls(&log, 0x0900_0003).is_empty());
    }

    #[test]
    fn register_viewer_stores_the_process_id() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            sink.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            Ret {
                eax: 0x4321,
                ..Ret::default()
            }
        });
        e.mem.set_u32(PROCESS_FACTORY, 0x5555);
        e.call(0x00d6_3400, &args![]);
        assert_eq!(
            calls(&log, REGISTER_PROCESS),
            vec![vec![0x5555, PROCESS_NAME, PROCESS_CREATE]]
        );
        assert_eq!(e.mem.u32(PROCESS_ID), 0x4321);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

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

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00d632c0_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_3280, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_32c0, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x54]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0, 0x1234]);
        assert_eq!(seen[0].1, 0x54);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d632c0_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_3280, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_32c0, &args![0x1234u32]).u32(), 0);
    }
}

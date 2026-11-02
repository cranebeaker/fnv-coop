//! `Havok SDK/hkpworldsnapshotviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB): cdecl `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkArrayStreamWriter::hkArrayStreamWriter` (`ECX` the writer, then the array and a flag;
/// the decompiler calls it `nullTerminate`).
const STREAM_WRITER_CONSTRUCT: u32 = 0x00cc_06f0;
/// `hkArrayStreamWriter` destructor (`ECX` the writer).
const STREAM_WRITER_DESTRUCT: u32 = 0x00cc_0780;
/// `hkpHavokSnapshot::save` (cdecl): `(result byte, world body, writer, allocator word, tag, 0)`.
const SNAPSHOT_SAVE: u32 = 0x00cc_0d60;
/// `hkOArchive` writers (`ECX` the archive): `write32u` (`00d60960`), the byte writer
/// (`00d605d0`, unnamed), `write32` (`00d60940`) and `writeRaw` (`00d607b0`, data and size).
const ARCHIVE_WRITE_32U: u32 = 0x00d6_0960;
const ARCHIVE_WRITE_BYTE: u32 = 0x00d6_05d0;
const ARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
/// `hkProcessFactory::registerProcess` (`ECX` the factory, then the name and the creator).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
/// The process factory (`.data`) `registerViewer` registers with.
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
/// The process identifiers `registerViewer` stores (`.data`).
const NATIVE_PROCESS_ID: u32 = 0x0126_8fc4;
const PC_PROCESS_ID: u32 = 0x0126_8fc0;
/// The names and creator functions of the two processes.
const NATIVE_NAME: u32 = 0x010d_16b0;
const NATIVE_CREATOR: u32 = 0x00d6_2f20;
const PC_NAME: u32 = 0x010d_16d0;
const PC_CREATOR: u32 = 0x00d6_2f60;
/// The four-byte tags `step` writes into every snapshot block (`.rdata`/`.data`): the first
/// when the viewer's byte at `+0x20` is set, the second when it is clear.
const TAG_SET: u32 = 0x010d_6ae8;
const TAG_CLEAR: u32 = 0x011b_7b88;

/// The capacity word of an empty `hkArray` that owns no memory.
const NOT_OWNED: u32 = 0x8000_0000;
/// The initial size `step` reserves for the snapshot buffer.
const BUFFER_SIZE: i32 = 0x19000;

// Offsets in `step`'s scratch block (the exe's stack locals): the result byte of the save, the
// snapshot buffer (`hkArray`: data, size, capacity word) and the `hkArrayStreamWriter` (20 bytes).
const RESULT_BYTE: u32 = 0;
const BUFFER: u32 = 4;
const WRITER: u32 = 0x10;

// Translated from 00d63000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldSnapshotViewer::registerViewer` (Xbox PDB name, engine map): registers the process
/// `"* Grab World Snapshot Native"` (creator `00d62f20`) with the process factory (the global
/// `01269cd0`, `ECX`; `00dd9380`) and keeps the result in `01268fc4`, then the process
/// `"* Grab World Snapshot PC"` (creator `00d62f60`) into `01268fc0`. The factory global is
/// read again for the second call.
pub fn hkp_world_snapshot_viewer_register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let native = e
        .call(
            REGISTER_PROCESS,
            &args![factory, NATIVE_NAME, NATIVE_CREATOR],
        )
        .u32();
    let factory: u32 = e.global(PROCESS_FACTORY);
    e.set_global(NATIVE_PROCESS_ID, native);
    let pc = e
        .call(REGISTER_PROCESS, &args![factory, PC_NAME, PC_CREATOR])
        .u32();
    e.set_global(PC_PROCESS_ID, pc);
}

/// The part of `step` that runs when the viewer has an archive (`this + 0xC`): saves every
/// world body of the context (`this + 0x1C`: array at `+0x6C`, count at `+0x70`) into a growing
/// buffer and writes each non-empty snapshot to the archive as a block (`size + 9`, byte `0xD0`,
/// the four tag bytes, the size, the data). `frame` is a zeroed scratch block of 0x24 bytes.
fn snapshot_worlds(e: &mut Engine, t: u32, frame: u32) {
    let buffer = frame + BUFFER;
    let writer = frame + WRITER;
    e.mem.set_u32(buffer, 0);
    e.mem.set_u32(buffer + 4, 0);
    e.mem.set_u32(buffer + 8, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(buffer + 8) & 0x3fff_ffff;
    if (capacity as i32) < BUFFER_SIZE {
        let doubled = (capacity as i32).wrapping_mul(2);
        let request = if doubled > BUFFER_SIZE {
            doubled
        } else {
            BUFFER_SIZE
        };
        e.call(ARRAY_RESERVE, &args![allocator, buffer, request, 1u32]);
    }
    e.call(STREAM_WRITER_CONSTRUCT, &args![writer, buffer, 1u32]);
    let tag = if e.mem.u8(t + 0x20) == 0 {
        TAG_CLEAR
    } else {
        TAG_SET
    };
    let context = e.mem.u32(t + 0x1c);
    if e.mem.i32(context + 0x70) > 0 {
        let mut index = 0u32;
        loop {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = e.mem.u32(buffer + 8) & 0x3fff_ffff;
            // The masked capacity is never negative, so this growth is not reached.
            if (capacity as i32) < 0 {
                let doubled = (capacity as i32).wrapping_mul(2);
                let request = if doubled <= 0 { 0 } else { doubled };
                e.call(ARRAY_RESERVE, &args![allocator, buffer, request, 1u32]);
            }
            e.mem.set_u32(buffer + 4, 0);
            let context = e.mem.u32(t + 0x1c);
            let bodies = e.mem.u32(context + 0x6c);
            let body = e.mem.u32(bodies.wrapping_add(index.wrapping_mul(4)));
            let word = (allocator & 0xffff_ff00) | 1;
            e.call(
                SNAPSHOT_SAVE,
                &args![frame + RESULT_BYTE, body, writer, word, tag, 0u32],
            );
            let size = e.mem.i32(buffer + 4);
            if size >= 1 {
                let archive = e.mem.u32(t + 0xc);
                e.call(ARCHIVE_WRITE_32U, &args![archive, size.wrapping_add(9)]);
                let archive = e.mem.u32(t + 0xc);
                e.call(ARCHIVE_WRITE_BYTE, &args![archive, 0xd0u32]);
                for byte in 0..4 {
                    let value = e.mem.u8(tag + byte) as u32;
                    let archive = e.mem.u32(t + 0xc);
                    e.call(ARCHIVE_WRITE_BYTE, &args![archive, value]);
                }
                let archive = e.mem.u32(t + 0xc);
                e.call(ARCHIVE_WRITE_32, &args![archive, size]);
                let data = e.mem.u32(buffer);
                let archive = e.mem.u32(t + 0xc);
                e.call(ARCHIVE_WRITE_RAW, &args![archive, data, size]);
            }
            index += 1;
            let context = e.mem.u32(t + 0x1c);
            if index as i32 >= e.mem.i32(context + 0x70) {
                break;
            }
        }
    }
    e.call(STREAM_WRITER_DESTRUCT, &args![writer]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(buffer + 8);
    e.mem.set_u32(buffer + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(buffer);
        e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
    }
}

// Translated from 00d63040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldSnapshotViewer::step` (Xbox PDB name, engine map). Fastcall with one stack word
/// that is overwritten before use (`RET 4`). When the viewer has an archive (`this + 0xC`) it
/// snapshots every world body of its context and writes them to the archive (see
/// `snapshot_worlds`). Then, when there is a next process object (`this + 0x14`), tail-jumps to
/// its virtual function at `0x18` with the process identifier: `01268fc0` when the byte at
/// `this + 0x20` is set, else `01268fc4`.
pub fn hkp_world_snapshot_viewer_step(e: &mut Engine, this: Ptr, _unused_2: u32) {
    let t = this.addr();
    if e.mem.u32(t + 0xc) != 0 {
        e.with_stack(0x24, |e, frame| snapshot_worlds(e, t, frame.addr()));
    }
    let next = e.mem.u32(t + 0x14);
    if next != 0 {
        let id: u32 = if e.mem.u8(t + 0x20) != 0 {
            e.global(PC_PROCESS_ID)
        } else {
            e.global(NATIVE_PROCESS_ID)
        };
        e.vcall(next, 0x18, &args![id]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_3000, hkp_world_snapshot_viewer_register_viewer()),
        entry!(0x00d6_3040, hkp_world_snapshot_viewer_step(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base of the doubles that stand in for vtable slots (`SLOT_BASE + slot offset`).
    const SLOT_BASE: u32 = 0x0900_0000;

    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn addresses(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|(a, _)| *a).collect()
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn register_viewer_registers_both_processes_and_rereads_the_factory() {
        let (mut e, log) = setup();
        e.set_global(PROCESS_FACTORY, 0x100u32);
        let sink = Rc::clone(&log);
        let mut results = vec![0x61u32, 0x62].into_iter();
        e.register_double(REGISTER_PROCESS, move |e, a| {
            sink.borrow_mut().push((REGISTER_PROCESS, a.to_vec()));
            e.set_global(PROCESS_FACTORY, 0x200u32);
            Ret {
                eax: results.next().unwrap(),
                ..Ret::default()
            }
        });
        e.call(0x00d6_3000, &args![]);
        assert_eq!(
            calls_to(&log, REGISTER_PROCESS),
            vec![
                vec![0x100, NATIVE_NAME, NATIVE_CREATOR],
                vec![0x200, PC_NAME, PC_CREATOR],
            ]
        );
        assert_eq!(e.global::<u32>(NATIVE_PROCESS_ID), 0x61);
        assert_eq!(e.global::<u32>(PC_PROCESS_ID), 0x62);
    }

    /// A viewer (returned) with an archive `0xA000` (or none), a context with two world bodies
    /// `0x111` and `0x222`, a next process object (returned) and the flag byte. The memory
    /// router's allocator is returned third, its free slot (`0x10`) logs.
    fn viewer(e: &mut Engine, log: &Log, archive: u32, flag: u8, count: u32) -> (u32, u32, u32) {
        e.set_global(NATIVE_PROCESS_ID, 0x40u32);
        e.set_global(PC_PROCESS_ID, 0x70u32);
        e.mem.set_u8(TAG_SET, 1);
        e.mem.set_u8(TAG_SET + 1, 2);
        e.mem.set_u8(TAG_SET + 2, 3);
        e.mem.set_u8(TAG_SET + 3, 4);
        e.mem.set_u8(TAG_CLEAR, 9);
        let this = e.mem.alloc(0x30);
        let context = e.mem.alloc(0x80);
        let bodies = e.mem.alloc(8);
        e.mem.set_u32(bodies, 0x111);
        e.mem.set_u32(bodies + 4, 0x222);
        e.mem.set_u32(context + 0x6c, bodies);
        e.mem.set_u32(context + 0x70, count);
        e.mem.set_u32(this + 0xc, archive);
        e.mem.set_u32(this + 0x1c, context);
        e.mem.set_u8(this + 0x20, flag);
        let next = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(next, table);
        e.mem.set_u32(table + 0x18, SLOT_BASE + 0x18);
        e.mem.set_u32(this + 0x14, next);
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let allocator_table = e.mem.alloc(0x40);
        e.mem.set_u32(allocator, allocator_table);
        e.mem.set_u32(allocator_table + 0x10, SLOT_BASE + 0x10);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_callees(
            e,
            log,
            &[
                STREAM_WRITER_DESTRUCT,
                ARCHIVE_WRITE_32U,
                ARCHIVE_WRITE_BYTE,
                ARCHIVE_WRITE_32,
                ARCHIVE_WRITE_RAW,
                SLOT_BASE + 0x10,
                SLOT_BASE + 0x18,
            ],
        );
        (this, next, allocator)
    }

    /// Doubles for the reserve (which hands out `buffer` and the requested capacity when
    /// `reserves`), the writer constructor and the save, which leaves `sizes` in the buffer.
    fn buffer_doubles(e: &mut Engine, log: &Log, buffer: u32, reserves: bool, sizes: Vec<u32>) {
        let sink = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            if reserves {
                e.mem.set_u32(a[1], buffer);
                e.mem.set_u32(a[1] + 8, a[2]);
            }
            Ret::default()
        });
        let array = Rc::new(Cell::new(0u32));
        let remember = Rc::clone(&array);
        let sink = Rc::clone(log);
        e.register_double(STREAM_WRITER_CONSTRUCT, move |_, a| {
            sink.borrow_mut()
                .push((STREAM_WRITER_CONSTRUCT, a.to_vec()));
            remember.set(a[1]);
            Ret::default()
        });
        let sink = Rc::clone(log);
        let mut sizes = sizes.into_iter();
        e.register_double(SNAPSHOT_SAVE, move |e, a| {
            sink.borrow_mut().push((SNAPSHOT_SAVE, a.to_vec()));
            assert_eq!(e.mem.u32(array.get() + 4), 0);
            e.mem.set_u32(array.get() + 4, sizes.next().unwrap());
            Ret::default()
        });
    }

    #[test]
    fn step_saves_every_world_body_and_writes_the_nonempty_snapshots() {
        let (mut e, log) = setup();
        let (this, next, allocator) = viewer(&mut e, &log, 0xa000, 1, 2);
        let buffer = e.mem.alloc(0x40);
        buffer_doubles(&mut e, &log, buffer, true, vec![0, 5]);
        e.call(0x00d6_3040, &args![this, 0x1234u32]);
        let word = (allocator & 0xffff_ff00) | 1;
        let reserves = calls_to(&log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][0], allocator);
        assert_eq!(reserves[0][2..], [0x19000, 1]);
        let saves = calls_to(&log, SNAPSHOT_SAVE);
        assert_eq!(saves.len(), 2);
        assert_eq!(saves[0][1], 0x111);
        assert_eq!(saves[1][1], 0x222);
        for save in &saves {
            assert_eq!(save[3..], [word, TAG_SET, 0]);
        }
        let writer = calls_to(&log, STREAM_WRITER_CONSTRUCT);
        assert_eq!(writer.len(), 1);
        assert_eq!(writer[0][2], 1);
        assert_eq!(saves[0][2], writer[0][0]);
        assert_eq!(calls_to(&log, ARCHIVE_WRITE_32U), vec![vec![0xa000, 14]]);
        assert_eq!(
            calls_to(&log, ARCHIVE_WRITE_BYTE),
            vec![
                vec![0xa000, 0xd0],
                vec![0xa000, 1],
                vec![0xa000, 2],
                vec![0xa000, 3],
                vec![0xa000, 4],
            ]
        );
        assert_eq!(calls_to(&log, ARCHIVE_WRITE_32), vec![vec![0xa000, 5]]);
        assert_eq!(
            calls_to(&log, ARCHIVE_WRITE_RAW),
            vec![vec![0xa000, buffer, 5]]
        );
        assert_eq!(calls_to(&log, STREAM_WRITER_DESTRUCT).len(), 1);
        assert_eq!(
            calls_to(&log, SLOT_BASE + 0x10),
            vec![vec![allocator, buffer, 0x19000]]
        );
        assert_eq!(calls_to(&log, SLOT_BASE + 0x18), vec![vec![next, 0x70]]);
        let order = addresses(&log);
        assert_eq!(order.first(), Some(&ARRAY_RESERVE));
        assert_eq!(order.last(), Some(&(SLOT_BASE + 0x18)));
        assert_eq!(order[order.len() - 3], STREAM_WRITER_DESTRUCT);
    }

    #[test]
    fn step_with_the_flag_clear_uses_the_other_tag_and_identifier() {
        let (mut e, log) = setup();
        let (this, next, _) = viewer(&mut e, &log, 0xa000, 0, 1);
        let buffer = e.mem.alloc(0x40);
        buffer_doubles(&mut e, &log, buffer, true, vec![1]);
        e.call(0x00d6_3040, &args![this, 0u32]);
        assert_eq!(calls_to(&log, SNAPSHOT_SAVE)[0][4], TAG_CLEAR);
        let bytes = calls_to(&log, ARCHIVE_WRITE_BYTE);
        assert_eq!(bytes[1], vec![0xa000, 9]);
        assert_eq!(calls_to(&log, ARCHIVE_WRITE_32U), vec![vec![0xa000, 10]]);
        assert_eq!(calls_to(&log, SLOT_BASE + 0x18), vec![vec![next, 0x40]]);
    }

    #[test]
    fn step_without_an_archive_only_calls_the_next_process() {
        let (mut e, log) = setup();
        let (this, next, _) = viewer(&mut e, &log, 0, 0, 2);
        e.call(0x00d6_3040, &args![this, 0u32]);
        assert_eq!(*log.borrow(), vec![(SLOT_BASE + 0x18, vec![next, 0x40])]);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x00d6_3040, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn step_with_no_worlds_keeps_the_unowned_buffer() {
        let (mut e, log) = setup();
        let (this, _, _) = viewer(&mut e, &log, 0xa000, 1, 0);
        e.mem.set_u32(this + 0x14, 0);
        let buffer = e.mem.alloc(0x40);
        buffer_doubles(&mut e, &log, buffer, false, vec![]);
        e.call(0x00d6_3040, &args![this, 0u32]);
        assert_eq!(
            addresses(&log),
            vec![
                ARRAY_RESERVE,
                STREAM_WRITER_CONSTRUCT,
                STREAM_WRITER_DESTRUCT
            ]
        );
    }
}

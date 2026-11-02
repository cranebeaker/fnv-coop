//! `BSMain/run_00c3da50` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base-class constructor `00c3da50` runs first (`00c3e4f0`,
/// `BSMain/iomanager.obj`, `__thiscall`, `RET 0xC`).
const BASE_CTOR: u32 = 0x00c3_e4f0;
/// `IOManager`'s vtable (`010c1604`; the RTTI locator in the word before it
/// names `.?AVIOManager@@`).
const IO_MANAGER_VTABLE: u32 = 0x010c_1604;
/// `MemoryManager::Allocate` (`00aa3e40`, `__thiscall`, `RET 4`) and the
/// memory manager it is called on (`011f6238`).
const ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// Constructor of the 0x10-byte object stored at `+0x64` (`00c3e8b0`,
/// `BSMain/iomanager.obj`, `__thiscall`, `RET 0xC`).
const QUEUE_CTOR: u32 = 0x00c3_e8b0;

// Translated from 00c3da50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3da50 - the C++ exception frame (FS:[0] handler 00f2dd43, state 0 that destroys the base and state 1 that frees the 0x10-byte block when its constructor throws) is not translated.
/// `IOManager`'s constructor (`__thiscall`, plain `RET`; the class is the
/// vtable's RTTI name, the map gives no name). The Xbox PDB's `IOManager` is
/// 0x5C bytes and its offsets do not match the PC code, so the fields are
/// raw PC offsets (as in `iomanager.rs`). Runs the base constructor with
/// (10, 1, 0x78), stores the `IOManager` vtable, zeroes the words at +0x70,
/// +0x78, +0x6C, +0x7C and +0x80, sets +0x68 to 6, allocates 0x10 bytes and
/// constructs them with `00c3e8b0` (6, 7, 8) into +0x64 (null when the
/// allocation fails), and copies into +0x60 the word at +8 of the first
/// entry of the array at +0x50. Returns `this`.
pub fn io_manager_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(BASE_CTOR, &args![this, 10u32, 1u32, 0x78u32]);
    e.mem.set_u32(t, IO_MANAGER_VTABLE);
    e.mem.set_u32(t + 0x70, 0);
    e.mem.set_u32(t + 0x78, 0);
    e.mem.set_u32(t + 0x6c, 0);
    e.mem.set_u32(t + 0x7c, 0);
    e.mem.set_u32(t + 0x80, 0);
    e.mem.set_u32(t + 0x68, 6);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x10u32]).u32();
    let queue = if block == 0 {
        0
    } else {
        e.call(QUEUE_CTOR, &args![block, 6u32, 7u32, 8u32]).u32()
    };
    e.mem.set_u32(t + 0x64, queue);
    // `XOR ECX,ECX; SHL ECX,2`: entry 0 of the pointer array at +0x50.
    let entries = e.mem.u32(t + 0x50);
    let first = e.mem.u32(entries);
    let word = e.mem.u32(first + 8);
    e.mem.set_u32(t + 0x60, word);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3da50, io_manager_ctor(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose base constructor (doubled) leaves at +0x50 an array
    /// whose first entry holds `0x4242` at +8; `operator new` returns a block
    /// when `allocate` is set, and the queue constructor returns `0x9999`.
    fn setup(allocate: bool) -> (Engine, Ptr, u32, Log) {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x84));
        for offset in (0..0x84).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xcccc_cccc);
        }
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, 0x4242);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, entry);
        let block = if allocate { e.mem.alloc(0x10) } else { 0 };
        let log: Log = Rc::new(RefCell::new(vec![]));
        let l = log.clone();
        e.register_double(BASE_CTOR, move |e, a| {
            l.borrow_mut().push((BASE_CTOR, a.to_vec()));
            e.mem.set_u32(a[0] + 0x50, array);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(ALLOCATE, move |_, a| {
            l.borrow_mut().push((ALLOCATE, a.to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(QUEUE_CTOR, move |_, a| {
            l.borrow_mut().push((QUEUE_CTOR, a.to_vec()));
            Ret {
                eax: 0x9999,
                ..Ret::default()
            }
        });
        (e, this, block, log)
    }

    #[test]
    fn constructs_the_manager_and_its_queue() {
        let (mut e, this, block, log) = setup(true);
        let t = this.addr();
        let back = e.call(0x00c3_da50, &args![this]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(calls(&log, BASE_CTOR), [vec![t, 10, 1, 0x78]]);
        assert_eq!(e.mem.u32(t), IO_MANAGER_VTABLE);
        for offset in [0x6cu32, 0x70, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(t + offset), 0, "word at {offset:#x}");
        }
        assert_eq!(e.mem.u32(t + 0x68), 6);
        assert_eq!(calls(&log, ALLOCATE), [vec![MEMORY_MANAGER, 0x10]]);
        assert_eq!(calls(&log, QUEUE_CTOR), [vec![block, 6, 7, 8]]);
        assert_eq!(e.mem.u32(t + 0x64), 0x9999);
        assert_eq!(e.mem.u32(t + 0x60), 0x4242);
        // Words the constructor does not write keep their value.
        assert_eq!(e.mem.u32(t + 0x74), 0xcccc_cccc);
    }

    #[test]
    fn a_failed_allocation_leaves_the_queue_null() {
        let (mut e, this, _, log) = setup(false);
        e.call(0x00c3_da50, &args![this]);
        assert!(calls(&log, QUEUE_CTOR).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x60), 0x4242);
    }
}

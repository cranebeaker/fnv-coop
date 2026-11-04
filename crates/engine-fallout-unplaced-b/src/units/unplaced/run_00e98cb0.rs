//! `(unplaced)/run_00e98cb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `MemoryManager` singleton and its deallocation (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;

/// The inlined `NiPointer` release: `InterlockedDecrement` of the count at +4, `DeleteThis`
/// (vtable +4) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 0x4, &args![]);
    }
}

// Translated from 00e98cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears an `NiTArray` of 8-byte entries holding two
/// reference-counted pointers. The word at +0x10 is cleared first; with `clear` set, each
/// entry below the 16-bit size (+0xa, re-read) has both pointers released and cleared, is
/// then read again from the array and, when not null, its two (now cleared) pointers
/// released again when set and the entry freed through the memory manager; then every slot
/// below the size is cleared and both counts (+0xa, +0xc) zeroed.
pub fn fn_00e98cb0(e: &mut Engine, this: Ptr, clear: u8) {
    let this = this.addr();
    e.mem.set_u32(this + 0x10, 0);
    if clear == 0 {
        return;
    }
    if e.mem.u16(this + 0xa) != 0 {
        let mut index = 0i32;
        loop {
            let entry_at = |e: &Engine, i: i32| {
                let base = e.mem.u32(this + 4);
                e.mem.u32(base.wrapping_add((i as u32).wrapping_mul(4)))
            };
            let entry = entry_at(e, index);
            for slot in [0u32, 4] {
                let object = e.mem.u32(entry + slot);
                if object != 0 {
                    release(e, object);
                    e.mem.set_u32(entry + slot, 0);
                }
            }
            let entry = entry_at(e, index);
            if entry != 0 {
                for slot in [4u32, 0] {
                    let object = e.mem.u32(entry + slot);
                    if object != 0 {
                        release(e, object);
                    }
                }
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, entry]);
            }
            index += 1;
            if index >= i32::from(e.mem.u16(this + 0xa)) {
                break;
            }
        }
    }
    let mut slot: u16 = 0;
    while slot < e.mem.u16(this + 0xa) {
        let base = e.mem.u32(this + 4);
        e.mem.set_u32(base.wrapping_add(u32::from(slot) * 4), 0);
        slot = slot.wrapping_add(1);
    }
    e.mem.set_u16(this + 0xa, 0);
    e.mem.set_u16(this + 0xc, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_8cb0, fn_00e98cb0(Ptr, u8))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn releases_both_pointers_and_frees_each_entry() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(0x0079_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0079_0004);
        let object = |e: &mut Engine, count: u32| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, vtable);
            e.mem.set_u32(o + 4, count);
            o
        };
        let a = object(&mut e, 1);
        let b = object(&mut e, 2);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, a);
        e.mem.set_u32(entry + 4, b);
        let base = e.mem.alloc(8);
        e.mem.set_u32(base, entry);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 0xa, 1);
        e.mem.set_u16(this + 0xc, 1);
        e.mem.set_u32(this + 0x10, 7);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8cb0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        // `a` reaches 0 and deletes itself; `b` goes to 1.
        assert!(log.contains(&(0x0079_0004, vec![a])));
        assert_eq!(e.mem.u32(b + 4), 1);
        assert!(log.contains(&(DEALLOCATE, vec![MEMORY_MANAGER, entry])));
        assert_eq!(log.iter().filter(|c| c.0 == DEALLOCATE).count(), 1);
        assert_eq!((e.mem.u32(entry), e.mem.u32(entry + 4)), (0, 0));
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        // Without the flag only +0x10 is cleared.
        e.mem.set_u16(this + 0xa, 3);
        e.mem.set_u32(this + 0x10, 7);
        e.call(0x00e9_8cb0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}

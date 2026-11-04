//! `(unplaced)/run_0044b660` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The scope marker constructor (`00404eb0`, `thiscall`: kind, flag, source
/// file, line) and its destructor (`00404ee0`).
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.h"`.
const SOURCE_FILE: u32 = 0x0101_71a0;
/// `operator new` (`00401000`, `cdecl`, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The 0x14-byte object's constructor (`0044cd00`, `thiscall`: the
/// structure, the bucket's first word, the bucket's second word).
const ITERATOR_CONSTRUCT: u32 = 0x0044_cd00;

// Translation incomplete: 0044b660 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
// Translated from 0044b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a virtual method, from the vtables at
/// `01017444`/`01017468`; `thiscall`, one word, `RET 4`): inside a scope
/// marker (kind 6, `ThreadSafeStructures.h` line 0x95) allocates 0x14 bytes
/// and constructs them (`0044cd00`) on `this` and the two words of the
/// 8-byte entry `index` of the table at +0x10 (by address); answers the
/// object, or 0 when the allocation fails.
pub fn fn_0044b660(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let this = this.addr();
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_CONSTRUCT,
            &args![scope, 6u32, 1u32, SOURCE_FILE, 0x95u32],
        );
        let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
        let made = if block == 0 {
            0
        } else {
            let table = e.mem.u32(this + 0x10);
            let entry = table.wrapping_add(index.wrapping_shl(1).wrapping_mul(4));
            e.call(ITERATOR_CONSTRUCT, &args![block, this, entry, entry + 4])
                .u32()
        };
        e.call(SCOPE_DESTROY, &args![scope]);
        made
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044b660, fn_0044b660(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SCOPE_CONSTRUCT, |_, _| Ret::default());
        e.register(SCOPE_DESTROY, |_, _| Ret::default());
        e.register(ITERATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn builds_the_object_on_the_entry() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x9000);
        e.call_log = Some(vec![]);
        let made = e.call(0x0044_b660, &args![this, 3u32]).u32();
        assert_ne!(made, 0);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log.iter().filter(|c| c.0 == ITERATOR_CONSTRUCT).collect();
        assert_eq!(built[0].1, [made, this, 0x9018, 0x901c]);
        let scope: Vec<_> = log.iter().filter(|c| c.0 == SCOPE_CONSTRUCT).collect();
        assert_eq!(&scope[0].1[1..], [6, 1, SOURCE_FILE, 0x95]);
        assert_eq!(log.iter().filter(|c| c.0 == SCOPE_DESTROY).count(), 1);
    }

    #[test]
    fn failed_allocation_answers_null() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_b660, &args![this, 0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == ITERATOR_CONSTRUCT));
    }
}

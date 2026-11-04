//! `fallout shared/pathfinding/run_006c72f0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A small object of three words and a counted array of 8-byte elements
//! (`006c73b0`), and the allocation helper that builds the 12-byte object.
//! The map names none of them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new(size)` (`00401000`, cdecl).
const NEW: u32 = 0x0040_1000;
/// `00633c90(this, word)`: constructor of the member at `+8`.
const MEMBER_CONSTRUCT: u32 = 0x0063_3c90;
/// `00404eb0(scope, id, 1, label, line)`: constructs the 4-byte scope object
/// (`ECX`) the game keeps on its stack around the work; `00404ee0` destroys it.
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// `00401050(array, element size, count, constructor)`: the compiler's
/// vector constructor iterator (the constructor is called with `ECX` = each
/// element).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The constructor of an 8-byte element (`0096a2d0`), run by the iterator.
const ELEMENT_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0044e330()`: no arguments; its result is stored at `+4`.
const CURRENT_VALUE: u32 = 0x0044_e330;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl"`.
const SOURCE_FILE_NAME: u32 = 0x0101_73f8;
/// Scope id and source line passed to the scope constructor.
const SCOPE_ID: u32 = 6;
const SOURCE_LINE: u32 = 0x14;

// Translated from 006c72f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c72f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Allocates 12 bytes (`00401000`) and, when the allocation succeeded,
/// constructs the object in it (`006c7370`). Gives back the object, or 0.
/// `this` (the address of the member this object belongs to) is not read.
/// The exception-unwinding frame is not translated.
pub fn fn_006c72f0(e: &mut Engine, _this: Ptr) -> Ptr {
    let block = e.call(NEW, &args![0xcu32]).ptr::<()>();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_006c7370(e, block)
    }
}

// Translated from 006c7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 12-byte object: runs `006c7390`. Returns `this`.
pub fn fn_006c7370(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006c7390(e, this);
    this
}

// Translated from 006c7390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the member at `+8` with `00633c90(this + 8, 0)`. Returns `this`.
pub fn fn_006c7390(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMBER_CONSTRUCT, &args![this.addr().wrapping_add(8), 0u32]);
    this
}

// Translated from 006c73b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c73b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of a counted array of 8-byte elements, between the scope
/// object `00404eb0` .. `00404ee0` (id 6, line 0x14 of
/// `ThreadSafeStructures.inl`): `+0xC` = 0, `+0` = `count`, `+8` = a block
/// of `count * 8` bytes (`0xFFFFFFFF` when that overflows) whose elements
/// are built by the vector constructor iterator (`00401050`) when the
/// allocation succeeded, else 0; `+4` = the result of `0044e330`. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_006c73b0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let base = this.addr();
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_CONSTRUCT,
            &args![scope, SCOPE_ID, 1u32, SOURCE_FILE_NAME, SOURCE_LINE],
        );
        e.mem.set_u32(base.wrapping_add(0xc), 0);
        e.mem.set_u32(base, count);
        let bytes = count.saturating_mul(8);
        let block = e.call(NEW, &args![bytes]).u32();
        let elements = if block == 0 {
            0
        } else {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, 8u32, count, ELEMENT_CONSTRUCT],
            );
            block
        };
        e.mem.set_u32(base.wrapping_add(8), elements);
        let value = e.call(CURRENT_VALUE, &args![]).u32();
        e.mem.set_u32(base.wrapping_add(4), value);
        e.call(SCOPE_DESTROY, &args![scope]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c72f0, fn_006c72f0(Ptr) -> Ptr),
        entry!(0x006c7370, fn_006c7370(Ptr) -> Ptr),
        entry!(0x006c7390, fn_006c7390(Ptr) -> Ptr),
        entry!(0x006c73b0, fn_006c73b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    #[test]
    fn member_constructor_passes_zero() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_CONSTRUCT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c7390, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(calls(&e)[1], (MEMBER_CONSTRUCT, vec![0x4008, 0]));
        assert_eq!(e.call(0x006c7370, &args![0x4000u32]).u32(), 0x4000);
    }

    #[test]
    fn allocation_helper_builds_the_object_only_when_new_succeeds() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_CONSTRUCT]);
        e.register(NEW, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c72f0, &args![0x1000u32]).u32(), 0x7000);
        assert_eq!(
            calls(&e)[1..],
            [(NEW, vec![0xc]), (MEMBER_CONSTRUCT, vec![0x7008, 0])]
        );
        e.register(NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c72f0, &args![0x1000u32]).u32(), 0);
        assert_eq!(calls(&e).len(), 2);
    }

    #[test]
    fn array_constructor_allocates_and_builds_the_elements() {
        let mut e = Engine::new();
        silent(&mut e, &[SCOPE_CONSTRUCT, SCOPE_DESTROY, VECTOR_CONSTRUCT]);
        e.register(NEW, |_, _| Ret {
            eax: 0x8000,
            ..Ret::default()
        });
        e.register(CURRENT_VALUE, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c73b0, &args![this, 4u32]).u32(), this.addr());
        let log = calls(&e);
        assert_eq!(log[1].0, SCOPE_CONSTRUCT);
        assert_eq!(log[1].1[1..], [6, 1, SOURCE_FILE_NAME, 0x14]);
        assert_eq!(log[2], (NEW, vec![32]));
        assert_eq!(
            log[3],
            (VECTOR_CONSTRUCT, vec![0x8000, 8, 4, ELEMENT_CONSTRUCT])
        );
        assert_eq!(log[4], (CURRENT_VALUE, vec![]));
        assert_eq!(log[5].0, SCOPE_DESTROY);
        assert_eq!(log[5].1, vec![log[1].1[0]]);
        assert_eq!(e.mem.u32(this.addr()), 4);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x1234);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x8000);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }

    #[test]
    fn array_constructor_saturates_the_size_and_skips_a_failed_allocation() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[SCOPE_CONSTRUCT, SCOPE_DESTROY, VECTOR_CONSTRUCT, NEW],
        );
        e.register(CURRENT_VALUE, |_, _| Ret::default());
        let this = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 8, 5);
        e.call_log = Some(vec![]);
        e.call(0x006c73b0, &args![this, 0x2000_0000u32]);
        let log = calls(&e);
        assert_eq!(log[2], (NEW, vec![0xffff_ffff]));
        assert!(log.iter().all(|c| c.0 != VECTOR_CONSTRUCT));
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
        assert_eq!(e.mem.u32(this.addr()), 0x2000_0000);
    }
}

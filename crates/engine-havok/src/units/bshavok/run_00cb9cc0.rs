//! `BSHavok/run_00cb9cc0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::MEMORY_MANAGER;

/// Resizes a block of the memory manager (`00aa4150`, `this` = the manager, two arguments:
/// the block and the new size in bytes).
const MEMORY_MANAGER_RESIZE: u32 = 0x00aa_4150;
/// Copies the old contents into a new block (`00cb9930`, `this` plus three arguments).
const COPY_CONTENTS: u32 = 0x00cb_9930;

// Translated from 00cb9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cb9cc0` (unnamed in the Xbox PDB): takes two words. Ignores `this`; runs
/// `00aa4150` on the memory manager with the first word and the second times 4 (a tail jump,
/// so its answer is the answer). Only reached through a vtable in the exe data.
pub fn fn_00cb9cc0(e: &mut Engine, _this: u32, block: u32, count: u32) -> u32 {
    e.call(
        MEMORY_MANAGER_RESIZE,
        &args![MEMORY_MANAGER, block, count.wrapping_mul(4)],
    )
    .u32()
}

// Translated from 00cb9ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cb9ce0` (unnamed in the Xbox PDB): resizes the buffer pointer kept at `+4` through
/// the object's own virtual slots. Takes a new size `size` and a size `old_size` to compare.
/// With no buffer, slot 4 (one word: `size`) answers the new buffer, `+0xC` becomes `size`
/// and `+4` the buffer. With a buffer and `old_size` equal to the word at `+0xC`, slot 0xC
/// (the buffer, `size`) answers the buffer to keep at `+4`. Otherwise slot 4 allocates the new
/// buffer, `00cb9930` copies `(new, old, old_size)`, slot 8 releases the old buffer (read from
/// `+4` again) and `+4` becomes the new one; `+0xC` is not changed.
pub fn fn_00cb9ce0(e: &mut Engine, this: u32, size: u32, old_size: u32) {
    let buffer = e.mem.u32(this + 4);
    if buffer == 0 {
        let fresh = e.vcall(this, 4, &args![size]).u32();
        e.mem.set_u32(this + 0xc, size);
        e.mem.set_u32(this + 4, fresh);
    } else if old_size == e.mem.u32(this + 0xc) {
        let kept = e.vcall(this, 0xc, &args![buffer, size]).u32();
        e.mem.set_u32(this + 4, kept);
    } else {
        let fresh = e.vcall(this, 4, &args![size]).u32();
        let old = e.mem.u32(this + 4);
        e.call(COPY_CONTENTS, &args![this, fresh, old, old_size]);
        let old = e.mem.u32(this + 4);
        e.vcall(this, 8, &args![old]);
        e.mem.set_u32(this + 4, fresh);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb9cc0, fn_00cb9cc0(u32, u32, u32) -> u32),
        entry!(0x00cb9ce0, fn_00cb9ce0(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    const SLOT_ALLOCATE: u32 = 0x0900_0004;
    const SLOT_RELEASE: u32 = 0x0900_0008;
    const SLOT_RESIZE: u32 = 0x0900_000c;

    /// An object whose three virtual slots are doubles: allocate answers 0x9000, resize answers
    /// 0xa000 and release answers nothing.
    fn setup() -> (Engine, u32) {
        let mut e = logging_engine();
        e.register_double(SLOT_ALLOCATE, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register_double(SLOT_RESIZE, |_, _| Ret {
            eax: 0xa000,
            ..Ret::default()
        });
        e.register_double(SLOT_RELEASE, |_, _| Ret::default());
        e.register_double(COPY_CONTENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + 4, SLOT_ALLOCATE);
        e.mem.set_u32(table + 8, SLOT_RELEASE);
        e.mem.set_u32(table + 0xc, SLOT_RESIZE);
        (e, this)
    }

    #[test]
    fn the_resize_thunk_multiplies_the_count_by_four() {
        let mut e = logging_engine();
        e.register_double(MEMORY_MANAGER_RESIZE, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let answer = e.call(0x00cb_9cc0, &args![0x77u32, 0x500u32, 6u32]).u32();
        assert_eq!(answer, 0x1234);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_RESIZE),
            [[MEMORY_MANAGER, 0x500, 24]]
        );
    }

    #[test]
    fn without_a_buffer_one_is_allocated() {
        let (mut e, this) = setup();
        e.call(0x00cb_9ce0, &args![this, 8u32, 3u32]);
        assert_eq!(calls_to(&e, SLOT_ALLOCATE), [[this, 8]]);
        assert_eq!(e.mem.u32(this + 4), 0x9000);
        assert_eq!(e.mem.u32(this + 0xc), 8);
    }

    #[test]
    fn an_equal_old_size_resizes_in_place() {
        let (mut e, this) = setup();
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 0xc, 3);
        e.call(0x00cb_9ce0, &args![this, 8u32, 3u32]);
        assert_eq!(calls_to(&e, SLOT_RESIZE), [[this, 0x5000, 8]]);
        assert_eq!(e.mem.u32(this + 4), 0xa000);
        assert_eq!(e.mem.u32(this + 0xc), 3);
    }

    #[test]
    fn a_different_old_size_copies_into_a_new_buffer_and_releases_the_old() {
        let (mut e, this) = setup();
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 0xc, 4);
        e.call(0x00cb_9ce0, &args![this, 8u32, 3u32]);
        assert_eq!(calls_to(&e, SLOT_ALLOCATE), [[this, 8]]);
        assert_eq!(calls_to(&e, COPY_CONTENTS), [[this, 0x9000, 0x5000, 3]]);
        assert_eq!(calls_to(&e, SLOT_RELEASE), [[this, 0x5000]]);
        assert_eq!(e.mem.u32(this + 4), 0x9000);
        assert_eq!(e.mem.u32(this + 0xc), 4);
    }
}

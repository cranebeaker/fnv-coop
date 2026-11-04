//! `(unplaced)/run_00e7ec60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The empty `NiFixedString` handle (`0109b220`) the reference counting skips.
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// The callees of `00e7ecd0`: `00e7de30` (unnamed, `thiscall` on the stage), the sized delete
/// `00aa1460` (`cdecl`: block and size), `00e891f0` and `00e89220` (unnamed, `thiscall` on the
/// object at +0x10), `NiShaderFactory::ReleaseGlobalShaderConstant` (engine map, `00aa04e0`,
/// `cdecl`: the address of an `NiFixedString`) and `00e7ec00` (engine map name
/// `NiControllerSequence::SetState`, not confirmed for this body; `thiscall` on the object at
/// `0126f6cc`, `RET 4`: the address of a word holding the stage).
const BODY_00E7DE30: u32 = 0x00e7_de30;
const SIZED_DELETE: u32 = 0x00aa_1460;
const CLEAR_00E891F0: u32 = 0x00e8_91f0;
const CLEAR_00E89220: u32 = 0x00e8_9220;
const RELEASE_GLOBAL_SHADER_CONSTANT: u32 = 0x00aa_04e0;
const GIVE_BACK_00E7EC00: u32 = 0x00e7_ec00;
const OBJECT_0126F6CC: u32 = 0x0126_f6cc;

// Translated from 00e7ecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: what the shaders call on a texture
/// stage they let go of (the class is unconfirmed). When the byte at +0x5f is clear, runs
/// `00e7de30` on it and frees its 0x68 bytes (`00aa1460`). Otherwise it is reset and given
/// back: +0x8 cleared, `00e891f0` and `00e89220` run on the object at +0x10 (read for each);
/// when the object at +0x18 is set, its name (`NiFixedString` at its +8) gets a reference
/// (`InterlockedIncrement` on the handle - 8, unless it is the empty handle at `0109b220`),
/// is copied (read again) into a stack word, released through
/// `ReleaseGlobalShaderConstant` (by the word's address), and the word's handle as it is
/// after the call gives up a reference (the same empty-handle test); then +0x18, +0x4, +0xc,
/// the 16-bit +0x5c, +0x14 and the byte +0x5e are cleared and the stage (in the stack word)
/// is handed to `00e7ec00` on the object at `0126f6cc`.
pub fn fn_00e7ecd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(0x5f)) == 0 {
        e.call(BODY_00E7DE30, &args![this]);
        e.call(SIZED_DELETE, &args![this, 0x68u32]);
        return;
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(base.wrapping_add(8), 0);
        let object = e.mem.u32(base.wrapping_add(0x10));
        e.call(CLEAR_00E891F0, &args![object]);
        let object = e.mem.u32(base.wrapping_add(0x10));
        e.call(CLEAR_00E89220, &args![object]);
        let constant = e.mem.u32(base.wrapping_add(0x18));
        if constant != 0 {
            let name = e.mem.u32(constant.wrapping_add(8));
            if name != e.global::<u32>(NULL_NAME_0109B220) {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![name.wrapping_sub(8)]);
            }
            let name = e.mem.u32(constant.wrapping_add(8));
            e.mem.set_u32(local, name);
            e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![local]);
            let name = e.mem.u32(local);
            if name != e.global::<u32>(NULL_NAME_0109B220) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
            }
        }
        e.mem.set_u32(base.wrapping_add(0x18), 0);
        e.mem.set_u32(base.wrapping_add(4), 0);
        e.mem.set_u32(base.wrapping_add(0xc), 0);
        e.mem.set_u16(base.wrapping_add(0x5c), 0);
        e.mem.set_u32(base.wrapping_add(0x14), 0);
        e.mem.set_u8(base.wrapping_add(0x5e), 0);
        let owner = e.global::<u32>(OBJECT_0126F6CC);
        e.mem.set_u32(local, base);
        e.call(GIVE_BACK_00E7EC00, &args![owner, local]);
    });
}

/// `00e7e7c0` (`thiscall`, `RET 4`) and `00e7e7a0` (`thiscall`, `RET 4`):
/// called with 3 and 1 on the parts of a node being taken down.
const NODE_PART_00E7E7C0: u32 = 0x00e7_e7c0;
const NODE_PART_00E7E7A0: u32 = 0x00e7_e7a0;
/// The memory manager (`011f6238`) and its `Deallocate` (`00aa4060`,
/// `thiscall`, `RET 4`: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// The Gamebryo free (`00aa10f0`, `cdecl`, one word).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00e7ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a node, no stack words): when the
/// word at `+0` is set, runs `00e7e7c0` on it with 3; when there is a next
/// node (`+8`), takes it down the same way (recursively) and gives its memory
/// back to the memory manager.
pub fn fn_00e7ec60(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr());
    if first != 0 {
        e.call(NODE_PART_00E7E7C0, &args![first, 3u32]);
    }
    let next = e.mem.u32(this.addr() + 8);
    if next != 0 {
        fn_00e7ec60(e, Ptr::new(next));
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, next]);
    }
}

// Translated from 00e7ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): takes down the object in
/// `0126f6cc` when there is one: clears its `+8`; its node at `+0xc` (when
/// set) gets `00e7e7c0` with 3 on its `+0` and `00e7e7a0` with 1 on its `+8`
/// (each when set) and goes back to the memory manager; the block at `+0` is
/// freed (`00aa10f0`) and the object goes back to the memory manager. The
/// global is cleared either way.
pub fn fn_00e7ed80(e: &mut Engine) {
    let object = e.global::<u32>(OBJECT_0126F6CC);
    if object != 0 {
        let node = e.mem.u32(object + 0xc);
        e.mem.set_u32(object + 8, 0);
        if node != 0 {
            let first = e.mem.u32(node);
            if first != 0 {
                e.call(NODE_PART_00E7E7C0, &args![first, 3u32]);
            }
            let second = e.mem.u32(node + 8);
            if second != 0 {
                e.call(NODE_PART_00E7E7A0, &args![second, 1u32]);
            }
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, node]);
        }
        let block = e.mem.u32(object);
        e.call(NI_FREE, &args![block]);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, object]);
    }
    e.set_global::<u32>(OBJECT_0126F6CC, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_ecd0, fn_00e7ecd0(Ptr)),
        entry!(0x00e7_ec60, fn_00e7ec60(Ptr)),
        entry!(0x00e7_ed80, fn_00e7ed80()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER: u32 = 0x0e70_0000;
    const EMPTY: u32 = 0x0e71_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for addr in [
            BODY_00E7DE30,
            SIZED_DELETE,
            CLEAR_00E891F0,
            CLEAR_00E89220,
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // The release swaps the name in the word for the empty handle.
        e.register(RELEASE_GLOBAL_SHADER_CONSTANT, |e, a| {
            e.mem.set_u32(a[0], EMPTY);
            Ret::default()
        });
        e.register(GIVE_BACK_00E7EC00, |e, a| {
            // The word holds the stage when it is handed over.
            assert_ne!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e.map(0x0109_b000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.set_global(NULL_NAME_0109B220, EMPTY);
        e.set_global(OBJECT_0126F6CC, OWNER);
        let this = e.mem.alloc(0x68);
        for offset in (0..0x68).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00e7ecd0_frees_a_stage_that_is_not_pooled() {
        let (mut e, this) = engine();
        e.mem.set_u8(this + 0x5f, 0);
        e.call(0x00e7_ecd0, &args![this]);
        assert_eq!(calls(&e, BODY_00E7DE30), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x68]]);
        assert!(calls(&e, GIVE_BACK_00E7EC00).is_empty());
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
    }

    #[test]
    fn fn_00e7ecd0_resets_and_gives_back_a_pooled_stage() {
        let (mut e, this) = engine();
        let object = e.mem.alloc(0x100);
        let constant = e.mem.alloc(0x10);
        let name = 0x0e72_0008u32;
        e.mem.set_u32(constant + 8, name);
        e.mem.set_u32(this + 0x10, object);
        e.mem.set_u32(this + 0x18, constant);
        e.call(0x00e7_ecd0, &args![this]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        assert_eq!(calls(&e, CLEAR_00E891F0), vec![vec![object]]);
        assert_eq!(calls(&e, CLEAR_00E89220), vec![vec![object]]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![name - 8]]
        );
        let released = calls(&e, RELEASE_GLOBAL_SHADER_CONSTANT);
        assert_eq!(released.len(), 1);
        // The word came back as the empty handle: no reference given up.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        for offset in [4u32, 8, 0xc, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u16(this + 0x5c), 0);
        assert_eq!(e.mem.u8(this + 0x5e), 0);
        assert_eq!(e.mem.u8(this + 0x5f), 0xff);
        let given = calls(&e, GIVE_BACK_00E7EC00);
        assert_eq!(given.len(), 1);
        assert_eq!(given[0][0], OWNER);
        assert_eq!(given[0][1], released[0][0]);
    }

    #[test]
    fn fn_00e7ecd0_without_a_constant_skips_the_release() {
        let (mut e, this) = engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x10, object);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00e7_ecd0, &args![this]);
        assert!(calls(&e, RELEASE_GLOBAL_SHADER_CONSTANT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(calls(&e, GIVE_BACK_00E7EC00).len(), 1);
    }

    #[test]
    fn fn_00e7ecd0_gives_up_the_copy_of_a_named_constant() {
        let (mut e, this) = engine();
        let object = e.mem.alloc(0x100);
        let constant = e.mem.alloc(0x10);
        e.mem.set_u32(constant + 8, 0x0e73_0008);
        e.mem.set_u32(this + 0x10, object);
        e.mem.set_u32(this + 0x18, constant);
        e.register(RELEASE_GLOBAL_SHADER_CONSTANT, |_, _| Ret::default());
        e.call(0x00e7_ecd0, &args![this]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e73_0000]]
        );
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for f in [NODE_PART_00E7E7C0, NODE_PART_00E7E7A0, DEALLOCATE, NI_FREE] {
            e.register(f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e7ec60_takes_the_chain_down() {
        let mut e = engine();
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let c = e.mem.alloc(12);
        e.mem.set_u32(a, 0x100);
        e.mem.set_u32(a + 8, b);
        e.mem.set_u32(b + 8, c);
        e.mem.set_u32(c, 0x300);
        e.call(0x00e7_ec60, &args![a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (NODE_PART_00E7E7C0, vec![0x100, 3]),
                (NODE_PART_00E7E7C0, vec![0x300, 3]),
                (DEALLOCATE, vec![MEMORY_MANAGER, c]),
                (DEALLOCATE, vec![MEMORY_MANAGER, b]),
            ]
        );
    }

    #[test]
    fn fn_00e7ed80_takes_the_object_down() {
        let mut e = engine();
        e.map(OBJECT_0126F6CC, 4);
        e.call(0x00e7_ed80, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let object = e.mem.alloc(0x10);
        let node = e.mem.alloc(12);
        e.mem.set_u32(object, 0x700);
        e.mem.set_u32(object + 8, 9);
        e.mem.set_u32(object + 0xc, node);
        e.mem.set_u32(node, 0x100);
        e.mem.set_u32(node + 8, 0x200);
        e.set_global::<u32>(OBJECT_0126F6CC, object);
        e.call_log = Some(vec![]);
        e.call(0x00e7_ed80, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (NODE_PART_00E7E7C0, vec![0x100, 3]),
                (NODE_PART_00E7E7A0, vec![0x200, 1]),
                (DEALLOCATE, vec![MEMORY_MANAGER, node]),
                (NI_FREE, vec![0x700]),
                (DEALLOCATE, vec![MEMORY_MANAGER, object]),
            ]
        );
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.global::<u32>(OBJECT_0126F6CC), 0);
    }
}

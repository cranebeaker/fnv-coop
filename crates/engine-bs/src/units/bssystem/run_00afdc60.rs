//! `BSSystem/run_00afdc60` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A search-path list (the engine map names none of these): an array of
//! path strings (`+4` storage, `+8` count) of which the first one made is
//! the global list at `011f81dc`, used by the file-exists callback
//! `00afde50`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The global list (the first constructed).
const GLOBAL_LIST: u32 = 0x011f_81dc;
/// A byte the constructor stores its third argument in.
const GLOBAL_FLAG: u32 = 0x011f_81e0;
/// The heap object (`011f6238`), its allocate (`00aa3e40`) and free
/// (`00aa4060`).
const HEAP: u32 = 0x011f_6238;
const HEAP_ALLOCATE: u32 = 0x00aa_3e40;
const HEAP_FREE: u32 = 0x00aa_4060;
/// `strlen` (`00ec6130`) and `strcpy_s` (`00ec65a6`), cdecl.
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;

// Translated from 00afdc60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afdc60 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Constructor (`thiscall`, three words; the first two are not read): the
/// base array (`00afe9f0` with 0); becomes the global list when there is
/// none; installs two file callbacks (`00aa1b90` with `00afdf00`,
/// `00aa1a70` with `00afde50`) and stores the third argument's low byte at
/// `011f81e0`. Returns `this`.
pub fn fn_00afdc60(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32, flag: u8) -> Ptr {
    e.call(0x00af_e9f0, &args![this, 0u32]);
    if e.global::<u32>(GLOBAL_LIST) == 0 {
        e.set_global(GLOBAL_LIST, this.addr());
    }
    e.call(0x00aa_1b90, &args![0x00af_df00u32]);
    e.call(0x00aa_1a70, &args![0x00af_de50u32]);
    e.set_global(GLOBAL_FLAG, flag);
    this
}

// Translated from 00afdcf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afdcf0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Destructor: frees every path string from the last (the heap's free,
/// then `00afea20` with the index and 1 removes the entry), stops being
/// the global list, and destroys the base array (`005eee10`).
pub fn fn_00afdcf0(e: &mut Engine, this: Ptr) {
    let mut index = e.mem.u32(this.addr().wrapping_add(8));
    loop {
        let previous = index;
        index = index.wrapping_sub(1);
        if previous == 0 {
            break;
        }
        let storage = e.mem.u32(this.addr().wrapping_add(4));
        let path = e.mem.u32(storage.wrapping_add(index.wrapping_mul(4)));
        e.call(HEAP_FREE, &args![HEAP, path]);
        e.call(0x00af_ea20, &args![this, index, 1u32]);
    }
    if e.global::<u32>(GLOBAL_LIST) == this.addr() {
        e.set_global(GLOBAL_LIST, 0u32);
    }
    e.call(0x005e_ee10, &args![this]);
}

// Translated from 00afdda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a search path: a heap copy of `path` that ends in a backslash (one
/// is appended when missing), added to the array (`007cb2e0`, a
/// `BSSimpleArray` add in the engine map, with the address of the copy). Nothing for a null `path`.
pub fn fn_00afdda0(e: &mut Engine, this: Ptr, path: u32) {
    if path == 0 {
        return;
    }
    let length = e.call(STRLEN, &args![path]).u32();
    let last = e.mem.u8(path.wrapping_add(length).wrapping_sub(1)) as i8;
    let needs_slash = u32::from(i32::from(last) != 0x5c);
    let size = e
        .call(STRLEN, &args![path])
        .u32()
        .wrapping_add(needs_slash)
        .wrapping_add(1);
    let copy = e.call(HEAP_ALLOCATE, &args![HEAP, size]).u32();
    e.call(STRCPY_S, &args![copy, size, path]);
    if needs_slash != 0 {
        e.mem.set_u8(copy.wrapping_add(length), 0x5c);
        e.mem.set_u8(copy.wrapping_add(length).wrapping_add(1), 0);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), copy);
        e.call(0x007c_b2e0, &args![this, local]);
    });
}

// Translated from 00afde50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afde50 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// The file-exists callback (cdecl `(name, second)`): without `second`,
/// asks the global list's finder (`00afe0d0` with `name, 0, 0, -1`; false
/// without a global list) for a non-null result; with it, builds a
/// 0x30-byte finder on the stack (`00aa14d0` with `name, second, 0`), asks
/// its vtable +4 and destroys it (`00aa1690`).
pub fn fn_00afde50(e: &mut Engine, name: u32, second: u32) -> bool {
    if second == 0 {
        let list: u32 = e.global(GLOBAL_LIST);
        let found = if list != 0 {
            e.call(0x00af_e0d0, &args![list, name, 0u32, 0u32, u32::MAX])
                .u32()
        } else {
            0
        };
        return found != 0;
    }
    e.with_stack(0x30, |e, finder| {
        e.call(0x00aa_14d0, &args![finder, name, second, 0u32]);
        let result = e.vcall(finder.addr(), 4, &[]).u8() != 0;
        e.call(0x00aa_1690, &args![finder]);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_dc60, fn_00afdc60(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00af_dcf0, fn_00afdcf0(Ptr)),
        entry!(0x00af_dda0, fn_00afdda0(Ptr, u32)),
        entry!(0x00af_de50, fn_00afde50(u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        for target in [
            0x00af_e9f0,
            0x00aa_1b90,
            0x00aa_1a70,
            HEAP_FREE,
            0x00af_ea20,
            0x005e_ee10,
            0x007c_b2e0,
            0x00aa_14d0,
            0x00aa_1690,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn first_list_becomes_the_global_one_and_leaves_it() {
        let mut e = engine();
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_dc60, &args![first, 1u32, 2u32, 0x101u32])
                .u32(),
            first
        );
        e.call(0x00af_dc60, &args![second, 1u32, 2u32, 0u32]);
        assert_eq!(e.global::<u32>(GLOBAL_LIST), first);
        assert_eq!(calls_to(&e, 0x00aa_1a70)[0], vec![0x00af_de50]);
        assert_eq!(e.global::<u8>(GLOBAL_FLAG), 0);
        // The destructor frees the paths from the last.
        let storage = e.mem.alloc(8);
        e.mem.set_u32(storage, 0x100);
        e.mem.set_u32(storage + 4, 0x200);
        e.mem.set_u32(first + 4, storage);
        e.mem.set_u32(first + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00af_dcf0, &args![first]);
        assert_eq!(
            calls_to(&e, HEAP_FREE),
            vec![vec![HEAP, 0x200], vec![HEAP, 0x100]]
        );
        assert_eq!(
            calls_to(&e, 0x00af_ea20),
            vec![vec![first, 1, 1], vec![first, 0, 1]]
        );
        assert_eq!(e.global::<u32>(GLOBAL_LIST), 0);
        assert_eq!(calls_to(&e, 0x005e_ee10), vec![vec![first]]);
    }

    #[test]
    fn added_paths_end_in_a_backslash() {
        let mut e = engine();
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(STRCPY_S, |e, a| {
            for i in 0..a[1] {
                let c = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, c);
                if c == 0 {
                    break;
                }
            }
            Ret::default()
        });
        e.register(0x007c_b2e0, |e, a| {
            let copy = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], copy);
            Ret::default()
        });
        let list = e.mem.alloc(0x10);
        let path = e.mem.alloc(8);
        e.mem.write(path, b"Data\0");
        e.call_log = Some(vec![]);
        e.call(0x00af_dda0, &args![list, path]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![HEAP, 6]]);
        let copy = e.mem.u32(list);
        assert_eq!(e.mem.bytes(copy, 6), b"Data\\\0".to_vec());
        e.mem.write(path, b"Dat\\\0");
        e.call(0x00af_dda0, &args![list, path]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE)[1], vec![HEAP, 5]);
        e.call(0x00af_dda0, &args![list, 0u32]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE).len(), 2);
    }

    #[test]
    fn exists_callback_asks_the_global_list_or_a_finder() {
        let mut e = engine();
        e.register(0x00af_e0d0, |_, a| u32::from(a[1] == 7).into_ret());
        assert!(!e.call(0x00af_de50, &args![7u32, 0u32]).bool());
        e.set_global(GLOBAL_LIST, 0x1234u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_de50, &args![7u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, 0x00af_e0d0),
            vec![vec![0x1234, 7, 0, 0, u32::MAX]]
        );
        assert!(!e.call(0x00af_de50, &args![8u32, 0u32]).bool());
        // With a second word: a finder on the stack answers through +4.
        e.put_vtable(0x0060_0000, &[0, 0x0061_0004]);
        e.register(0x0061_0004, |_, _| 1u32.into_ret());
        e.register(0x00aa_14d0, |e, a| {
            e.mem.set_u32(a[0], 0x0060_0000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_de50, &args![7u32, 9u32]).bool());
        let built = calls_to(&e, 0x00aa_14d0);
        assert_eq!(built[0][1..], [7, 9, 0]);
        assert_eq!(calls_to(&e, 0x00aa_1690), vec![vec![built[0][0]]]);
    }
}

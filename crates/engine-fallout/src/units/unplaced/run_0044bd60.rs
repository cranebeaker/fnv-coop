//! `(unplaced)/run_0044bd60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040fbf0` (`ECX` = the lock at `this + 0x20`, one word; `RET 4`): taken
/// with 0 before the work.
const LOCK: u32 = 0x0040_fbf0;
/// `0040fba0` (`ECX` = the lock at `this + 0x20`): released after it.
const UNLOCK: u32 = 0x0040_fba0;
/// `00449f80` (`ECX` = `this`): the object the work is asked of.
const TARGET: u32 = 0x0044_9f80;
/// `0044c8e0` (`ECX` = that object; two words; `RET 8`): the work, a byte
/// result.
const WORK: u32 = 0x0044_c8e0;

// Translated from 0044bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20` (`0040fbf0` with 0, released by
/// `0040fba0`), asks `00449f80`'s object for `0044c8e0` with the two words
/// given and returns its byte. The decompiler hangs the two words on
/// `00449f80`; that function takes none (`RET`), they belong to `0044c8e0`
/// (`RET 8`). The engine map gives it no name.
pub fn fn_0044bd60(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u8 {
    let lock = this.addr() + 0x20;
    e.call(LOCK, &args![lock, 0u32]);
    let target = e.call(TARGET, &args![this]).u32();
    let result = e.call(WORK, &args![target, first, second]).u8();
    e.call(UNLOCK, &args![lock]);
    result
}

/// `0044e370` (`ECX` = the new pointer word, one stack word: the address of the source):
/// copies a reference-counted pointer.
const COPY_POINTER: u32 = 0x0044_e370;
/// `0044cbf0` (`ECX` = the address of a pointer word): releases a reference-counted pointer.
const RELEASE_POINTER: u32 = 0x0044_cbf0;

// Translated from 0044bdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044bdb0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eecf28, state variable for the by-value argument) and the frame's cookie word are not translated.
/// A slot of seven vtables (`01017498`, `010174e8`, `01017538`, `01017588`, `01017678`,
/// `0102d0a0`, `01088668`; unnamed in the engine map; `ECX` and one stack word, `RET 4`):
/// `pointer` is a reference-counted pointer passed by value. Under the lock at
/// `this + 0x20` (`0040fbf0` with 0, released by `0040fba0`), offers a copy of it
/// (`0044e370`) to `this`'s own vtable slot +0x18 with each index below the count at +8,
/// until one answers true in `AL`. Then releases `pointer` (`0044cbf0`) and answers
/// whether one did.
pub fn fn_0044bdb0(e: &mut Engine, this: Ptr, pointer: u32) -> bool {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), pointer);
        let lock = this.addr() + 0x20;
        e.call(LOCK, &args![lock, 0u32]);
        let mut done = false;
        let mut index = 0u32;
        while index < e.mem.u32(this.addr() + 8) && !done {
            // The copy is the callee's by-value argument; the callee releases it.
            done = e.with_stack(4, |e, copy| {
                e.call(COPY_POINTER, &args![copy, argument]);
                let copied = e.mem.u32(copy.addr());
                e.vcall(this.addr(), 0x18, &args![index, copied]).u8() != 0
            });
            index += 1;
        }
        e.call(UNLOCK, &args![lock]);
        e.call(RELEASE_POINTER, &args![argument]);
        done
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_bd60, fn_0044bd60(Ptr, u32, u32) -> u8),
        entry!(0x0044_bdb0, fn_0044bdb0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0044bd60_works_under_the_lock() {
        let mut e = Engine::new();
        e.register(LOCK, |_, _| Ret::default());
        e.register(UNLOCK, |_, _| Ret::default());
        e.register(TARGET, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register(WORK, |_, _| Ret {
            eax: 0x1234_5602,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_bd60, &args![0x4000u32, 7u32, 9u32]);
        // Only `AL` of the work's result comes back.
        assert_eq!(result.u32(), 2);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0044_bd60)
            .collect();
        assert_eq!(
            log,
            vec![
                (LOCK, vec![0x4020, 0]),
                (TARGET, vec![0x4000]),
                (WORK, vec![0x6000, 7, 9]),
                (UNLOCK, vec![0x4020]),
            ]
        );
    }
}

#[cfg(test)]
mod offer_tests {
    use super::*;

    fn run(count: u32, stop_at: u32) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.register(LOCK, |_, _| Ret::default());
        e.register(UNLOCK, |_, _| Ret::default());
        e.register(RELEASE_POINTER, |_, _| Ret::default());
        e.register(COPY_POINTER, |e, a| {
            let source = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], source + 1);
            Ret::default()
        });
        e.register_double(0x0060_0018, move |_, a| {
            u32::from(a[1] == stop_at).into_ret()
        });
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, 0, 0x0060_0018]);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x0070_0000);
        e.mem.set_u32(this + 8, count);
        e.call_log = Some(vec![]);
        let done = e.call(0x0044_bdb0, &args![this, 0x5000u32]).bool();
        let log = e.call_log.take().unwrap();
        assert_eq!(done, stop_at < count);
        assert_eq!(log.iter().filter(|(a, _)| *a == UNLOCK).count(), 1);
        assert_eq!(log.iter().filter(|(a, _)| *a == RELEASE_POINTER).count(), 1);
        log.into_iter()
            .filter(|(a, _)| *a == 0x0060_0018)
            .map(|(a, w)| (a, w[1..].to_vec()))
            .collect()
    }

    #[test]
    fn fn_0044bdb0_offers_a_copy_to_each_index_until_one_takes_it() {
        let offers = run(4, 1);
        assert_eq!(
            offers,
            vec![
                (0x0060_0018, vec![0, 0x5001]),
                (0x0060_0018, vec![1, 0x5001])
            ]
        );
        assert_eq!(run(3, 9).len(), 3);
        assert!(run(0, 0).is_empty());
    }
}

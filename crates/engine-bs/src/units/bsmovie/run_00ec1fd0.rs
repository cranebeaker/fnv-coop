//! `BSMovie/run_00ec1fd0` (the engine map's run of unnamed functions from `00ec1fd0`),
//! subsystem `BSMovie`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). `00ec1fd0` itself is in `movieplayer.rs`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocation and release (`00aa3e40`, `00aa4060`,
/// `thiscall`, one word); the thread block's allocation context word (+0x2b4) and the movie
/// context 0x14.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
const TLS_CONTEXT: u32 = 0x2b4;
const MOVIE_CONTEXT: u32 = 0x14;

// Translated from 00ec1ff0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec1ff0 - the SEH frame (FS:[0] handler 00f34b48, state 0 around the allocation) is not translated.
/// Unnamed in the engine map (one word: a size, `RET 4`; `ECX` unused): a block of that size
/// from the memory manager (`00aa3e40`) with the thread block's allocation context (+0x2b4)
/// set to 0x14 for the allocation and restored after. Answers the block.
pub fn fn_00ec1ff0(e: &mut Engine, size: u32) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    block
}

// Translated from 00ec20a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (one word: a block, `RET 4`; `ECX` unused): the block given back
/// to the memory manager (`00aa4060`).
pub fn fn_00ec20a0(e: &mut Engine, block: u32) {
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_1ff0, fn_00ec1ff0(u32) -> u32),
        entry!(0x00ec_20a0, fn_00ec20a0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_movie_allocations_use_the_movie_context() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 3);
        e.register_double(ALLOCATE, move |e, a| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), MOVIE_CONTEXT);
            (0x1000 + a[1]).into_ret()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ec_1ff0, &args![0x20u32]).u32(), 0x1020);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 3);
        e.call(0x00ec_20a0, &args![0x1020u32]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
        assert_eq!(frees[0].1, vec![MEMORY_MANAGER, 0x1020]);
    }
}

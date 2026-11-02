//! `BSMovie/run_00ec2200` (the engine map's run of unnamed functions from `00ec2200`),
//! subsystem `BSMovie`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocation (`00aa3e40`, `thiscall`, one word); the
/// movie's constructor (`00ec1f20`), its open (`00ec0ac0`, `thiscall`, four words; answer
/// `AL`) and the player's stop (`00ec25e0`); the thread block's allocation context (+0x2b4)
/// and the movie context 0x14.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const MOVIE_CONSTRUCT: u32 = 0x00ec_1f20;
const MOVIE_OPEN_FILE: u32 = 0x00ec_0ac0;
const PLAYER_STOP: u32 = 0x00ec_25e0;
const TLS_CONTEXT: u32 = 0x2b4;
const MOVIE_CONTEXT: u32 = 0x14;

// Translated from 00ec2200 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec2200 - the SEH frame (FS:[0] handler 00f34b9c, states 0/1 around the allocation and the movie's construction) is not translated.
/// Unnamed in the engine map (`thiscall` on the player, four words: a file and three flags
/// whose low bytes are passed zero-extended, `RET 0x10`): with the thread block's allocation
/// context (+0x2b4) set to 0x14, a new 0x28-byte movie (`00aa3e40`, made by `00ec1f20`; null
/// when the allocation fails) becomes the current one (+0xc) and opens the file
/// (`00ec0ac0(file, a, b, c)`); when that answers 0 the player stops (`00ec25e0`). The context
/// is restored; answers the open's `AL`.
pub fn fn_00ec2200(e: &mut Engine, this: Ptr, file: u32, a: u32, b: u32, c: u32) -> u8 {
    let at = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x28u32]).u32();
    let movie = if block != 0 {
        e.call(MOVIE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(at + 0xc, movie);
    let movie = e.mem.u32(at + 0xc);
    let (a, b, c) = (u32::from(a as u8), u32::from(b as u8), u32::from(c as u8));
    let opened = e.call(MOVIE_OPEN_FILE, &args![movie, file, a, b, c]).u8();
    if opened == 0 {
        e.call(PLAYER_STOP, &args![this]);
    }
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    opened
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ec_2200,
        fn_00ec2200(Ptr, u32, u32, u32, u32) -> u8
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn a_new_movie_opens_the_file_or_the_player_stops() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register(MOVIE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PLAYER_STOP, |_, _| Ret::default());
        e.register_double(MOVIE_OPEN_FILE, move |e, a| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), MOVIE_CONTEXT);
            (if a[1] == 1 { 0x101u32 } else { 0x100 }).into_ret()
        });
        let player = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00ec_2200, &args![player, 1u32, 0x101u32, 0u32, 2u32])
                .u8(),
            1
        );
        let movie = e.mem.u32(player + 0xc);
        assert_ne!(movie, 0);
        assert_eq!(calls(&e, MOVIE_OPEN_FILE), vec![vec![movie, 1, 1, 0, 2]]);
        assert!(calls(&e, PLAYER_STOP).is_empty());
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
        assert_eq!(fn_00ec2200(&mut e, Ptr::new(player), 2, 0, 0, 0), 0);
        assert_eq!(calls(&e, PLAYER_STOP), vec![vec![player]]);
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
    }
}

//! `BSMovie/run_00ec2aa0` (the engine map's run of unnamed functions after `00ec2aa0`),
//! subsystem `BSMovie`: four methods of the movie player the game's movie code calls
//! (`00ec0280`), translated from FalloutNV.exe 1.4.0.525 (docs/ENGINE_CRATE.md): three scale
//! computations on the player's virtual size getters (x87 arithmetic, the result in `ST0`) and
//! the lookup of a movie's path.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::{x87_fadd, x87_fdiv, x87_fmul, x87_fsub};

/// The player's virtual functions (`thiscall`, no stack words, the answer in `EAX`) the scale
/// methods read: slots +0x28 and +0x2c (two sizes), +0x34 (an offset).
const SLOT_28: u32 = 0x28;
const SLOT_2C: u32 = 0x2c;
const SLOT_34: u32 = 0x34;
/// The double the offset computations divide by (`01011590`, read from the executable's data).
const DIVISOR: u32 = 0x0101_1590;
/// `00853130` (`thiscall` on the player's +0x40, two words: a name and the address of a word
/// that receives a path; answers `AL`) and `strcpy_s` (`00ec65a6`, cdecl).
const FIND_PATH: u32 = 0x0085_3130;
const STRCPY_S: u32 = 0x00ec_65a6;

/// `FILD qword` of a word zero-extended to 64 bits (`MOV [t],EAX; MOV [t+4],0`): exact.
fn fild_word(word: u32) -> X87Ext {
    X87Ext::load64(f64::from(word))
}

/// `FSTP float; FLD float`: the value rounded to a float and loaded back, as `ST0`.
fn through_float(x: X87Ext) -> f64 {
    x87_load32(x.store32())
}

// Translated from 00ec2aa0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The scale of one size (no label; `thiscall`, three words, `RET 0xc`): with the flag byte
/// set, the player's slot +0x28 size over `width`; otherwise its slot +0x2c size over `height`
/// (both zero-extended words, `FILD qword`, `FDIVP`), rounded to a float (`FSTP`/`FLD`) and
/// answered in `ST0`.
pub fn movie_scale(e: &mut Engine, this: Ptr, width: u32, height: u32, flag: u32) -> f64 {
    let (size, by) = if flag as u8 != 0 {
        (e.vcall(this.addr(), SLOT_28, &args![]).u32(), width)
    } else {
        (e.vcall(this.addr(), SLOT_2C, &args![]).u32(), height)
    };
    // FILD size; FILD by; FDIVP ST(1),ST(0): size / by.
    through_float(x87_fdiv(fild_word(size), fild_word(by)))
}

// Translated from 00ec2b20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The horizontal offset (no label; `thiscall`, three words: a float scale, a width, the flag
/// byte; `RET 0xc`). With the flag set: the player's slot +0x34 word as is (`FILD`, not
/// rounded). Otherwise `offset + (size - width * scale) / [01011590]`, with `offset` from slot
/// +0x34 (kept as a double, `FSTP qword`) and `size` from slot +0x28, in the code's x87 order
/// (`FMUL` by the float, `FSUBP`, `FDIV` by the double, `FADD` of the kept double), rounded to a
/// float. Answers `ST0`.
pub fn movie_offset_x(e: &mut Engine, this: Ptr, scale: u32, width: u32, flag: u32) -> f64 {
    let offset = e.vcall(this.addr(), SLOT_34, &args![]).u32();
    if flag as u8 != 0 {
        return fild_word(offset).store64();
    }
    let kept = fild_word(offset).store64();
    let size = e.vcall(this.addr(), SLOT_28, &args![]).u32();
    let scaled = x87_fmul(fild_word(width), X87Ext::load32(scale));
    let difference = x87_fsub(fild_word(size), scaled);
    let divisor = f64::from_bits(e.mem.u64(DIVISOR));
    let quotient = x87_fdiv(difference, X87Ext::load64(divisor));
    through_float(x87_fadd(quotient, X87Ext::load64(kept)))
}

// Translated from 00ec2bb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The vertical offset (no label; `thiscall`, three words: a float scale, a height, the flag
/// byte; `RET 0xc`): 0 (`FLDZ`) without the flag; otherwise `(size - height * scale) /
/// [01011590]` with `size` from the player's slot +0x2c, in the code's x87 order, rounded to a
/// float. Answers `ST0`.
pub fn movie_offset_y(e: &mut Engine, this: Ptr, scale: u32, height: u32, flag: u32) -> f64 {
    if flag as u8 == 0 {
        return 0.0;
    }
    let size = e.vcall(this.addr(), SLOT_2C, &args![]).u32();
    let scaled = x87_fmul(fild_word(height), X87Ext::load32(scale));
    let difference = x87_fsub(fild_word(size), scaled);
    let divisor = f64::from_bits(e.mem.u64(DIVISOR));
    through_float(x87_fdiv(difference, X87Ext::load64(divisor)))
}

// Translated from 00ec2df0 (disassembled, FalloutNV.exe 1.4.0.525)
/// A movie's path (no label; `thiscall`, two words: the name and the output buffer, `RET 8`):
/// `00853130` on the player's +0x40 with the name and the address of a zeroed local; when it
/// answers true (`AL`), the path it left there is copied into the buffer with `strcpy_s(buffer,
/// 0x104, path)` and true is answered; otherwise false.
pub fn movie_find_path(e: &mut Engine, this: Ptr, name: u32, buffer: u32) -> bool {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        let found = e
            .call(
                FIND_PATH,
                &args![this.addr().wrapping_add(0x40), name, local.addr()],
            )
            .u8();
        if found == 0 {
            return false;
        }
        let path = e.mem.u32(local.addr());
        e.call(STRCPY_S, &args![buffer, 0x104_u32, path]);
        true
    })
}

/// The sequence thread's priority setter (`00ec18a0`, `thiscall`, one word), the default
/// priority setting's word (`0126fae4`, inside the object at `0126fae0`) and the fallback word
/// `01202800` the exe uses when that object's address is null (it never is).
const SET_PRIORITY: u32 = 0x00ec_18a0;
const DEFAULT_PRIORITY_HOLDER: u32 = 0x0126_fae0;
const DEFAULT_PRIORITY: u32 = 0x0126_fae4;
const NULL_PRIORITY: u32 = 0x0120_2800;
/// The memory manager (`011f6238`) and its allocation (`00aa3e40`), the movie's constructor
/// (`00ec1f20`) and its open (`00ec0ac0`, four words; answer `AL`), the movie settings object
/// (`0126fac4`), `strlen` (`00ec6130`) and the player's name map set (`00ec2f00`, on +0x40).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const MOVIE_CONSTRUCT: u32 = 0x00ec_1f20;
const MOVIE_OPEN_FILE: u32 = 0x00ec_0ac0;
const MOVIE_SETTINGS: u32 = 0x0126_fac4;
const STRLEN: u32 = 0x00ec_6130;
const NAME_MAP_SET: u32 = 0x00ec_2f00;

// Translated from 00ec2c10 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): a priority setting (+0x2c) below 5
/// (signed) goes one up (`00ec18a0` with it plus 1).
pub fn fn_00ec2c10(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.i32(at + 0x2c) < 5 {
        let setting = e.mem.u32(at + 0x2c).wrapping_add(1);
        e.call(SET_PRIORITY, &args![this, setting]);
    }
}

// Translated from 00ec2c40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): a priority setting (+0x2c) above the
/// default one (the word at `0126fae4`, signed compare; the exe tests the constant address
/// `0126fae0` for null first, and when it is, which it never is, clears `01202800` and reads
/// that) goes one down (`00ec18a0` with it less 1).
pub fn fn_00ec2c40(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let word = if DEFAULT_PRIORITY_HOLDER != 0 {
        DEFAULT_PRIORITY
    } else {
        // Never taken: the exe tests a constant address.
        e.mem.set_u32(NULL_PRIORITY, 0);
        NULL_PRIORITY
    };
    if e.mem.i32(at + 0x2c) > e.mem.i32(word) {
        let setting = e.mem.u32(at + 0x2c).wrapping_sub(1);
        e.call(SET_PRIORITY, &args![this, setting]);
    }
}

// Translated from 00ec2ca0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec2ca0 - the SEH frame (FS:[0] handler 00f34c64, state 0 around the movie's construction) is not translated.
/// The movie thread's procedure (unnamed in the engine map; `stdcall`, its parameter, a file
/// name): a 0x28-byte movie (`00aa3e40`, made by `00ec1f20`; null when the allocation fails)
/// opens the file (`00ec0ac0(file, 1, 1, 1)`) and is handed to the movie settings object
/// (`0126fac4`) by [`fn_00ec2d40`]. Answers 0.
pub fn fn_00ec2ca0(e: &mut Engine, file: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x28u32]).u32();
    let movie = if block != 0 {
        e.call(MOVIE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(MOVIE_OPEN_FILE, &args![movie, file, 1u32, 1u32, 1u32]);
    let settings = e.mem.u32(MOVIE_SETTINGS);
    fn_00ec2d40(e, Ptr::new(settings), movie);
    0
}

// Translated from 00ec2d40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a movie, `RET 4`): the movie goes to +0x14,
/// the word at +4 is cleared, the callback at +0x1c runs (no stack words; `ECX` holds the
/// callback's own address) and is cleared.
pub fn fn_00ec2d40(e: &mut Engine, this: Ptr, movie: u32) {
    let at = this.addr();
    e.mem.set_u32(at + 0x14, movie);
    e.mem.set_u32(at + 4, 0);
    let callback = e.mem.u32(at + 0x1c);
    e.call(callback, &args![]);
    e.mem.set_u32(at + 0x1c, 0);
}

// Translated from 00ec2d80 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: a key and a string, `RET 8`): a copy of the
/// string (a block of its length plus one from `00aa3e40`, not checked for null, filled by
/// `strcpy_s` with that size) is set for the key in the name map at +0x40 (`00ec2f00`).
pub fn fn_00ec2d80(e: &mut Engine, this: Ptr, key: u32, text: u32) {
    let length = e.call(STRLEN, &args![text]).u32();
    let size = length.wrapping_add(1);
    let copy = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(STRCPY_S, &args![copy, size, text]);
    e.call(NAME_MAP_SET, &args![this.addr() + 0x40, key, copy]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_2c10, fn_00ec2c10(Ptr)),
        entry!(0x00ec_2c40, fn_00ec2c40(Ptr)),
        entry!(0x00ec_2ca0, fn_00ec2ca0(u32) -> u32),
        entry!(0x00ec_2d40, fn_00ec2d40(Ptr, u32)),
        entry!(0x00ec_2d80, fn_00ec2d80(Ptr, u32, u32)),
        entry!(0x00ec_2aa0, movie_scale(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2b20, movie_offset_x(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2bb0, movie_offset_y(Ptr, u32, u32, u32) -> f64),
        entry!(0x00ec_2df0, movie_find_path(Ptr, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    const PLAYER: u32 = 0x0800_0000;
    const VTABLE: u32 = 0x0900_0000;
    const GET_28: u32 = 0x0a00_0028;
    const GET_2C: u32 = 0x0a00_002c;
    const GET_34: u32 = 0x0a00_0034;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .cloned()
            .collect()
    }

    /// A player whose slots +0x28, +0x2c and +0x34 answer `a`, `b` and `c`, the divisor 2.0,
    /// and the call log on.
    fn engine(a: u32, b: u32, c: u32) -> Engine {
        let mut e = Engine::new();
        e.map(PLAYER, 0x100);
        e.map(VTABLE, 0x100);
        e.mem.set_u32(PLAYER, VTABLE);
        e.mem.set_u32(VTABLE + SLOT_28, GET_28);
        e.mem.set_u32(VTABLE + SLOT_2C, GET_2C);
        e.mem.set_u32(VTABLE + SLOT_34, GET_34);
        e.register_double(GET_28, move |_, _| a.into_ret());
        e.register_double(GET_2C, move |_, _| b.into_ret());
        e.register_double(GET_34, move |_, _| c.into_ret());
        e.map(DIVISOR, 8);
        e.mem.set_u64(DIVISOR, 2.0f64.to_bits());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn scale_divides_the_chosen_size_and_rounds_to_a_float() {
        let mut e = engine(1280, 720, 0);
        let p = Ptr::new(PLAYER);
        assert_eq!(movie_scale(&mut e, p, 640, 480, 1), 2.0);
        assert_eq!(movie_scale(&mut e, p, 640, 480, 0), 1.5);
        // 1280 / 3 rounded to a float.
        assert_eq!(
            movie_scale(&mut e, p, 3, 480, 1),
            f64::from(1280.0f32 / 3.0)
        );
        // Only the low byte of the flag word counts.
        assert_eq!(movie_scale(&mut e, p, 640, 480, 0x100), 1.5);
        assert_eq!(calls(&e, GET_28).len(), 2);
        assert_eq!(calls(&e, GET_2C).len(), 2);
        // Through the entry, with the player as `this`.
        let r = e.call(0x00ec_2aa0, &args![PLAYER, 640u32, 480u32, 1u32]);
        assert_eq!(r.st0, 2.0);
    }

    #[test]
    fn offsets_follow_the_x87_order() {
        let mut e = engine(1280, 720, 7);
        let p = Ptr::new(PLAYER);
        let half = 0.5f32.to_bits();
        // With the flag: the slot +0x34 word as is.
        assert_eq!(movie_offset_x(&mut e, p, half, 100, 1), 7.0);
        // 7 + (1280 - 100 * 0.5) / 2 = 622.
        assert_eq!(movie_offset_x(&mut e, p, half, 100, 0), 622.0);
        // (720 - 100 * 0.5) / 2 = 335; without the flag 0.
        assert_eq!(movie_offset_y(&mut e, p, half, 100, 1), 335.0);
        assert_eq!(movie_offset_y(&mut e, p, half, 100, 0), 0.0);
        // A result past float precision is rounded to a float: 7 + 1/3 * 0.5.
        e.mem.set_u64(DIVISOR, 3.0f64.to_bits());
        let x = movie_offset_x(&mut e, p, half, 2558, 0);
        assert_eq!(x, f64::from((7.0f64 + (1280.0 - 1279.0) / 3.0) as f32));
        // A negative difference stays negative.
        e.mem.set_u64(DIVISOR, 2.0f64.to_bits());
        assert_eq!(movie_offset_y(&mut e, p, half, 2000, 1), -140.0);
    }

    #[test]
    fn find_path_copies_what_00853130_found() {
        let mut e = engine(0, 0, 0);
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"Data\\Video");
        e.register_double(FIND_PATH, move |e, a| {
            e.mem.set_u32(a[2], path);
            1u32.into_ret()
        });
        e.register(STRCPY_S, |_, _| 0u32.into_ret());
        let name = e.mem.alloc(8);
        let buffer = e.mem.alloc(0x104);
        assert!(movie_find_path(&mut e, Ptr::new(PLAYER), name, buffer));
        let find = calls(&e, FIND_PATH);
        assert_eq!((find[0].1[0], find[0].1[1]), (PLAYER + 0x40, name));
        assert_eq!(
            calls(&e, STRCPY_S),
            vec![(STRCPY_S, vec![buffer, 0x104, path])]
        );
        // Not found (the low byte only): nothing copied.
        e.register(FIND_PATH, |_, _| 0x100u32.into_ret());
        assert!(!movie_find_path(&mut e, Ptr::new(PLAYER), name, buffer));
        assert_eq!(calls(&e, STRCPY_S).len(), 1);
    }
}

#[cfg(test)]
mod finish_tests {
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
    fn the_priority_steps_up_to_5_and_down_to_the_default() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(DEFAULT_PRIORITY_HOLDER, 8);
        e.mem.set_u32(DEFAULT_PRIORITY, 2);
        e.register(SET_PRIORITY, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x2c, 4);
        e.call(0x00ec_2c10, &args![this]);
        e.mem.set_u32(this + 0x2c, 5);
        fn_00ec2c10(&mut e, Ptr::new(this));
        e.mem.set_u32(this + 0x2c, 0xffff_ffff);
        fn_00ec2c10(&mut e, Ptr::new(this));
        assert_eq!(calls(&e, SET_PRIORITY), vec![vec![this, 5], vec![this, 0]]);
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x2c, 3);
        e.call(0x00ec_2c40, &args![this]);
        e.mem.set_u32(this + 0x2c, 2);
        fn_00ec2c40(&mut e, Ptr::new(this));
        assert_eq!(calls(&e, SET_PRIORITY), vec![vec![this, 2]]);
    }

    #[test]
    fn the_movie_thread_opens_and_hands_over_its_movie() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(MOVIE_SETTINGS, 4);
        let settings = e.mem.alloc(0x20);
        e.mem.set_u32(MOVIE_SETTINGS, settings);
        e.mem.set_u32(settings + 4, 0x66);
        e.mem.set_u32(settings + 0x1c, 0x0a00_0000);
        e.register(0x0a00_0000, |_, _| Ret::default());
        e.register(MOVIE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MOVIE_OPEN_FILE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00ec_2ca0, &args![0x1234u32]).u32(), 0);
        let movie = e.mem.u32(settings + 0x14);
        assert_ne!(movie, 0);
        assert_eq!(
            calls(&e, MOVIE_OPEN_FILE),
            vec![vec![movie, 0x1234, 1, 1, 1]]
        );
        assert_eq!(calls(&e, 0x0a00_0000).len(), 1);
        assert_eq!(
            (e.mem.u32(settings + 4), e.mem.u32(settings + 0x1c)),
            (0, 0)
        );
    }

    #[test]
    fn a_name_is_copied_into_the_name_map() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NAME_MAP_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"intro\0");
        e.call(0x00ec_2d80, &args![this, 7u32, text]);
        let set = calls(&e, NAME_MAP_SET);
        assert_eq!(set[0][..2], [this + 0x40, 7]);
        assert_eq!(e.mem.cstr(set[0][2]), b"intro".to_vec());
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 6]]);
    }
}

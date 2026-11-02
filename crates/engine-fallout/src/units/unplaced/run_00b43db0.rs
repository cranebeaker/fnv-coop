//! `(unplaced)/run_00b43db0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b43db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: a stream record): resets
/// the stream's state. Without a stream or a state (`+0x1c`) answers -2.
/// Otherwise clears the state's `+0x18`, the stream's `+0x14`, `+8` and
/// `+0x18`, the state's `+0`, `+4`, `+0xc`, `+0x20`, `+0x24`, `+0x30`,
/// `+0x34`, points the state's `+0x64` at its own `+0x528` and copies that to
/// `+0x48` and `+0x44`; answers 0.
pub fn fn_00b43db0(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + 0x1c) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + 0x1c);
    e.mem.set_u32(state + 0x18, 0);
    e.mem.set_u32(stream + 0x14, 0);
    e.mem.set_u32(stream + 8, 0);
    e.mem.set_u32(stream + 0x18, 0);
    for at in [0u32, 4, 0xc, 0x20, 0x24, 0x30, 0x34] {
        e.mem.set_u32(state + at, 0);
    }
    e.mem.set_u32(state + 0x64, state.wrapping_add(0x528));
    let window = e.mem.u32(state + 0x64);
    e.mem.set_u32(state + 0x48, window);
    let window = e.mem.u32(state + 0x48);
    e.mem.set_u32(state + 0x44, window);
    0
}

/// The version text the set-up compares the first character of (`011ad4ac`,
/// `"1.2.1"`).
const VERSION_TEXT: u32 = 0x011a_d4ac;
/// The default allocation and free functions the set-up installs
/// (`00b49020`, `00b49050`).
const DEFAULT_ALLOCATE: u32 = 0x00b4_9020;
const DEFAULT_FREE: u32 = 0x00b4_9050;

// Translated from 00b43e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: the stream record, a
/// window-bits word, a version text and the record's size). A missing
/// version, one whose first character differs from `011ad4ac` or a size
/// other than 0x38 answers -6; no stream -2. Clears `+0x18`, installs the
/// default allocator (`+0x20`, with `+0x28` cleared) and free (`+0x24`) when
/// missing, and makes the 0x1ba8-byte state through the allocator (`+0x28`,
/// 1, 0x1ba8) when there is none (none made answers -4; the flag `+0x34`
/// says it was made here). Negative window bits clear the state's `+8` and
/// are negated; otherwise `+8` is `(bits >> 4) + 1` and bits below 0x30 keep
/// their low four. Bits outside 8..15 free a state made here, clear `+0x1c`
/// and answer -2; otherwise the bits go to the state's `+0x1c`, `+0x2c` is
/// cleared and the stream is reset (`00b43db0`).
pub fn fn_00b43e70(e: &mut Engine, stream: u32, window_bits: i32, version: u32, size: u32) -> i32 {
    if version == 0 || e.mem.u8(version) != e.mem.u8(VERSION_TEXT) || size != 0x38 {
        return -6;
    }
    if stream == 0 {
        return -2;
    }
    e.mem.set_u32(stream + 0x18, 0);
    if e.mem.u32(stream + 0x20) == 0 {
        e.mem.set_u32(stream + 0x20, DEFAULT_ALLOCATE);
        e.mem.set_u32(stream + 0x28, 0);
    }
    if e.mem.u32(stream + 0x24) == 0 {
        e.mem.set_u32(stream + 0x24, DEFAULT_FREE);
    }
    let state = if e.mem.u32(stream + 0x1c) == 0 {
        let allocate = e.mem.u32(stream + 0x20);
        let opaque = e.mem.u32(stream + 0x28);
        let made = e.call(allocate, &args![opaque, 1u32, 0x1ba8u32]).u32();
        if made == 0 {
            return -4;
        }
        e.mem.set_u32(stream + 0x1c, made);
        e.mem.set_u32(stream + 0x34, 1);
        made
    } else {
        e.mem.set_u32(stream + 0x34, 0);
        e.mem.u32(stream + 0x1c)
    };
    let mut bits = window_bits;
    if bits < 0 {
        e.mem.set_u32(state + 8, 0);
        bits = bits.wrapping_neg();
    } else {
        e.mem.set_u32(state + 8, ((bits >> 4) + 1) as u32);
        if bits < 0x30 {
            bits &= 0xf;
        }
    }
    if !(8..=0xf).contains(&bits) {
        if e.mem.u32(stream + 0x34) != 0 {
            let free = e.mem.u32(stream + 0x24);
            let opaque = e.mem.u32(stream + 0x28);
            e.call(free, &args![opaque, state]);
        }
        e.mem.set_u32(stream + 0x1c, 0);
        return -2;
    }
    e.mem.set_u32(state + 0x1c, bits as u32);
    e.mem.set_u32(state + 0x2c, 0);
    fn_00b43db0(e, stream)
}

// Translated from 00b43fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: the stream record, a
/// version text and the record's size): `00b43e70` with 15 window bits.
pub fn fn_00b43fe0(e: &mut Engine, stream: u32, version: u32, size: u32) -> i32 {
    fn_00b43e70(e, stream, 0xf, version, size)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_3db0, fn_00b43db0(u32) -> i32),
        entry!(0x00b4_3e70, fn_00b43e70(u32, i32, u32, u32) -> i32),
        entry!(0x00b4_3fe0, fn_00b43fe0(u32, u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE: u32 = 0x013a_0000;
    const FREE: u32 = 0x013a_0010;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(VERSION_TEXT, 8);
        e.mem.set_cstr(VERSION_TEXT, b"1.2.1");
        let version = e.mem.alloc(8);
        e.mem.set_cstr(version, b"1.2.3");
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let stream = e.mem.alloc(0x38);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, FREE);
        (e, stream, version)
    }

    #[test]
    fn fn_00b43db0_resets_the_state() {
        let (mut e, stream, _) = engine();
        assert_eq!(e.call(0x00b4_3db0, &args![0u32]).i32(), -2);
        assert_eq!(e.call(0x00b4_3db0, &args![stream]).i32(), -2);
        let state = e.mem.alloc(0x530);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 8, 5);
        e.mem.set_u32(state + 0x30, 5);
        assert_eq!(e.call(0x00b4_3db0, &args![stream]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 8), 0);
        assert_eq!(e.mem.u32(state + 0x30), 0);
        assert_eq!(e.mem.u32(state + 0x44), state + 0x528);
        assert_eq!(e.mem.u32(state + 0x64), state + 0x528);
    }

    #[test]
    fn fn_00b43fe0_sets_up_fifteen_bits() {
        let (mut e, stream, version) = engine();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b4_3fe0, &args![stream, version, 0x38u32]).i32(),
            0
        );
        let state = e.mem.u32(stream + 0x1c);
        assert_eq!(e.mem.u32(state + 0x1c), 0xf);
        assert_eq!(e.mem.u32(state + 8), 1);
        assert_eq!(e.mem.u32(stream + 0x34), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ALLOCATE, vec![0, 1, 0x1ba8])));
    }

    #[test]
    fn fn_00b43e70_checks_and_window_bits() {
        let (mut e, stream, version) = engine();
        assert_eq!(
            e.call(0x00b4_3e70, &args![stream, 15i32, version, 0x30u32])
                .i32(),
            -6
        );
        let other = e.mem.alloc(8);
        e.mem.set_cstr(other, b"2.0");
        assert_eq!(
            e.call(0x00b4_3e70, &args![stream, 15i32, other, 0x38u32])
                .i32(),
            -6
        );
        assert_eq!(
            e.call(0x00b4_3e70, &args![0u32, 15i32, version, 0x38u32])
                .i32(),
            -2
        );
        // Raw (negative) bits.
        assert_eq!(
            e.call(0x00b4_3e70, &args![stream, -9i32, version, 0x38u32])
                .i32(),
            0
        );
        let state = e.mem.u32(stream + 0x1c);
        assert_eq!((e.mem.u32(state + 8), e.mem.u32(state + 0x1c)), (0, 9));
        // A kept state is not freed on bad bits.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b4_3e70, &args![stream, 20i32, version, 0x38u32])
                .i32(),
            -2
        );
        assert!(e.call_log.take().unwrap().iter().all(|c| c.0 != FREE));
        assert_eq!(e.mem.u32(stream + 0x1c), 0);
        // A state made here is freed.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b4_3e70, &args![stream, 0x40i32, version, 0x38u32])
                .i32(),
            -2
        );
        assert!(e.call_log.take().unwrap().iter().any(|c| c.0 == FREE));
        // The default allocator goes in when missing.
        let bare = e.mem.alloc(0x38);
        e.register(DEFAULT_ALLOCATE, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00b4_3e70, &args![bare, 15i32, version, 0x38u32])
                .i32(),
            -4
        );
        assert_eq!(e.mem.u32(bare + 0x20), DEFAULT_ALLOCATE);
        assert_eq!(e.mem.u32(bare + 0x24), DEFAULT_FREE);
    }
}

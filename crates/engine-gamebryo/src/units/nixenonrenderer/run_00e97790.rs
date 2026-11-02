//! `NiXenonRenderer/run_00e97790` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, size) and the constructor `00e97790` (unnamed;
/// `thiscall`, two words).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const CONSTRUCT_00E97790: u32 = 0x00e9_7790;

// Translated from 00e978a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e97790`), `cdecl`: two words. Makes a 0x8c-byte object
/// with `00e97790` and the two words; answers what the constructor answers, or null when the
/// allocation fails.
pub fn fn_00e978a0(e: &mut Engine, first: u32, second: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x8cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CONSTRUCT_00E97790, &args![block, first, second])
        .u32()
}

/// `00e97790`'s base constructor `00e98910` (unnamed; `thiscall`, one word) and the table it
/// then gets (`010f1550`).
const BASE_CONSTRUCT_00E98910: u32 = 0x00e9_8910;
const VTABLE_010F1550: u32 = 0x010f_1550;

// Translated from 00e97790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e97790`): a constructor, `thiscall` with two words (the
/// renderer, passed on to the base constructor `00e98910`, and a description record). Sets the
/// table `010f1550` and copies the record's +0xc8 to +0x28. The record's byte +0xc5: 1 clears
/// +0x78 and +0x50; 2 sets +0x7c to 0x10 when the record's +0xfc is nonzero (else 0) and +0x50
/// to 0x10; 3 sets +0x78 and +0x50 to 0x10; anything else leaves them. When +0x28 is then 0 and
/// the renderer's byte +0x5d4 is set, the byte +0x88 becomes 1 and +0x28 0x2000; otherwise the
/// byte +0x88 is cleared. The record's byte +0xcd: 1 sets +0x2c 8 and clears +0x7c and +0x54;
/// 2 sets +0x2c 0x20, +0x54 0x10 and +0x7c 0x10 when the record's +0x10c is nonzero (else 0);
/// 3 sets +0x2c 0xe0, +0x7c and +0x54 0x10. Answers `this`.
pub fn fn_00e97790(e: &mut Engine, this: Ptr, renderer: u32, record: u32) -> Ptr {
    let base = this.addr();
    let set =
        |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(base.wrapping_add(offset), value);
    // NEG/SBB/AND 0x10.
    let flag_0x10 = |word: u32| if word != 0 { 0x10u32 } else { 0 };
    e.call(BASE_CONSTRUCT_00E98910, &args![base, renderer]);
    set(e, 0, VTABLE_010F1550);
    let size = e.mem.u32(record.wrapping_add(0xc8));
    set(e, 0x28, size);
    match e.mem.u8(record.wrapping_add(0xc5)) {
        1 => {
            set(e, 0x78, 0);
            set(e, 0x50, 0);
        }
        2 => {
            let word = e.mem.u32(record.wrapping_add(0xfc));
            set(e, 0x7c, flag_0x10(word));
            set(e, 0x50, 0x10);
        }
        3 => {
            set(e, 0x78, 0x10);
            set(e, 0x50, 0x10);
        }
        _ => {}
    }
    if e.mem.u32(base.wrapping_add(0x28)) == 0 && e.mem.u8(renderer.wrapping_add(0x5d4)) != 0 {
        e.mem.set_u8(base.wrapping_add(0x88), 1);
        set(e, 0x28, 0x2000);
    } else {
        e.mem.set_u8(base.wrapping_add(0x88), 0);
    }
    match e.mem.u8(record.wrapping_add(0xcd)) {
        1 => {
            set(e, 0x2c, 8);
            set(e, 0x7c, 0);
            set(e, 0x54, 0);
        }
        2 => {
            set(e, 0x2c, 0x20);
            let word = e.mem.u32(record.wrapping_add(0x10c));
            set(e, 0x54, 0x10);
            set(e, 0x7c, flag_0x10(word));
        }
        3 => {
            set(e, 0x2c, 0xe0);
            set(e, 0x7c, 0x10);
            set(e, 0x54, 0x10);
        }
        _ => {}
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_78a0, fn_00e978a0(u32, u32) -> u32),
        entry!(0x00e9_7790, fn_00e97790(Ptr, u32, u32) -> Ptr),
        entry!(0x00e9_7870, fn_00e97870(Ptr, u32) -> Ptr),
    ]
}

/// The base destructor `00e989c0` (unnamed; `thiscall`, no stack words) and the Ni free
/// `00aa1460` (`cdecl`: block, size).
const BASE_DESTRUCT_00E989C0: u32 = 0x00e9_89c0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e97870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e97790`): the scalar deleting destructor of the object
/// `00e97790` builds, `thiscall` with one word (the flags). Stores the table `010f1550`, runs
/// the base destructor `00e989c0` and, with bit 0 of the flags set, frees the 0x8c bytes
/// (`00aa1460`). Answers `this`.
pub fn fn_00e97870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010F1550);
    e.call(BASE_DESTRUCT_00E989C0, &args![base]);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![base, 0x8cu32]);
    }
    this
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e978a0_makes_the_object() {
        let mut e = Engine::new();
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_00E97790, |_, a| (a[0] + a[1] + a[2]).into_ret());
        e.call_log = Some(vec![]);
        let made = e.call(0x00e9_78a0, &args![1u32, 2u32]).u32();
        let log = e.call_log.clone().unwrap();
        let block = log
            .iter()
            .find(|(a, _)| *a == CONSTRUCT_00E97790)
            .unwrap()
            .1[0];
        assert_eq!(made, block + 3);
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e9_78a0, &args![1u32, 2u32]).u32(), 0);
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// (+0x28, +0x2c, +0x50, +0x54, +0x78, +0x7c, byte +0x88).
    type View = (u32, u32, u32, u32, u32, u32, u8);

    fn construct(e: &mut Engine, words: &[(u32, u32)], c5: u8, cd: u8, flag: u8) -> View {
        e.register(BASE_CONSTRUCT_00E98910, |_, _| Ret::default());
        let this = e.mem.alloc(0x8c);
        e.mem.write(this, &[0xaa; 0x8c]);
        let renderer = e.mem.alloc(0x5d8);
        e.mem.set_u8(renderer + 0x5d4, flag);
        let record = e.mem.alloc(0x110);
        e.mem.set_u8(record + 0xc5, c5);
        e.mem.set_u8(record + 0xcd, cd);
        for (offset, value) in words {
            e.mem.set_u32(record + offset, *value);
        }
        e.call_log = Some(vec![]);
        let made = e
            .call(CONSTRUCT_00E97790, &args![this, renderer, record])
            .u32();
        assert_eq!(made, this);
        assert_eq!(e.mem.u32(this), VTABLE_010F1550);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT_00E98910, vec![this, renderer])));
        (
            e.mem.u32(this + 0x28),
            e.mem.u32(this + 0x2c),
            e.mem.u32(this + 0x50),
            e.mem.u32(this + 0x54),
            e.mem.u32(this + 0x78),
            e.mem.u32(this + 0x7c),
            e.mem.u8(this + 0x88),
        )
    }

    const UNSET: u32 = 0xaaaa_aaaa;

    #[test]
    fn fn_00e97790_first_byte() {
        let mut e = Engine::new();
        let size = [(0xc8u32, 0x400u32)];
        assert_eq!(
            construct(&mut e, &size, 1, 0, 0),
            (0x400, UNSET, 0, UNSET, 0, UNSET, 0)
        );
        assert_eq!(
            construct(&mut e, &[(0xc8, 0x400), (0xfc, 9)], 2, 0, 0),
            (0x400, UNSET, 0x10, UNSET, UNSET, 0x10, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 2, 0, 0),
            (0x400, UNSET, 0x10, UNSET, UNSET, 0, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 3, 0, 0),
            (0x400, UNSET, 0x10, UNSET, 0x10, UNSET, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 4, 0, 1),
            (0x400, UNSET, UNSET, UNSET, UNSET, UNSET, 0)
        );
    }

    #[test]
    fn fn_00e97790_size_defaults_when_the_renderer_asks() {
        let mut e = Engine::new();
        assert_eq!(
            construct(&mut e, &[], 0, 0, 1),
            (0x2000, UNSET, UNSET, UNSET, UNSET, UNSET, 1)
        );
        assert_eq!(
            construct(&mut e, &[], 0, 0, 0),
            (0, UNSET, UNSET, UNSET, UNSET, UNSET, 0)
        );
    }

    #[test]
    fn fn_00e97790_second_byte() {
        let mut e = Engine::new();
        let size = [(0xc8u32, 0x400u32)];
        assert_eq!(
            construct(&mut e, &[(0xc8, 0x400), (0xfc, 1)], 2, 1, 0),
            (0x400, 8, 0x10, 0, UNSET, 0, 0)
        );
        assert_eq!(
            construct(&mut e, &[(0xc8, 0x400), (0x10c, 1)], 0, 2, 0),
            (0x400, 0x20, UNSET, 0x10, UNSET, 0x10, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 0, 2, 0),
            (0x400, 0x20, UNSET, 0x10, UNSET, 0, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 0, 3, 0),
            (0x400, 0xe0, UNSET, 0x10, UNSET, 0x10, 0)
        );
        assert_eq!(
            construct(&mut e, &size, 0, 5, 0),
            (0x400, UNSET, UNSET, UNSET, UNSET, UNSET, 0)
        );
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    #[test]
    fn fn_00e97870_destroys_and_frees_with_bit_0() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCT_00E989C0, |e, a| {
            // Runs after the table is stored.
            assert_eq!(e.mem.u32(a[0]), VTABLE_010F1550);
            Ret::default()
        });
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let this = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_7870, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_DESTRUCT_00E989C0, vec![this])));
        assert!(log.contains(&(NI_FREE_00AA1460, vec![this, 0x8c])));
        e.call_log = Some(vec![]);
        e.call(0x00e9_7870, &args![this, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == NI_FREE_00AA1460));
    }
}

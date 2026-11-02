//! `fallout/ai/run_009f6c90` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (unnamed): `thiscall`, returns the word at `+0`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `009611e0` (unnamed, `playercharacter.cpp`): `thiscall`; an object whose slot `+0xe8`
/// is called.
const OWNER_009611E0: u32 = 0x0096_11e0;
/// `0066b0d0` (unnamed): `thiscall`, one word (`RET 4`); sets the pointer member.
const POINTER_SET: u32 = 0x0066_b0d0;
/// `0045cec0` (unnamed): the pointer member's destructor (`ECX` only).
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `00457fe0` (unnamed, `tes.cpp`): no words; the import at `0x00fdf060`'s answer (a time).
const CURRENT_TICKS: u32 = 0x0045_7fe0;
/// `0x428` slot object, then `007031c0` (unnamed, `interface.cpp`) and `00446390`
/// (unnamed, `modelloader.cpp`): `thiscall`; the kind the switch picks from.
const SLOT_OBJECT_007031C0: u32 = 0x0070_31c0;
const KIND_00446390: u32 = 0x0044_6390;
/// `512.0` (`0x010231a0`), `256.0` (`0x01032adc`) and `128.0` (`0x0101e704`), `float`s.
const DISTANCE_512: u32 = 0x0102_31a0;
const DISTANCE_256: u32 = 0x0103_2adc;
const DISTANCE_128: u32 = 0x0101_e704;
/// `00647b70` (unnamed): `cdecl` (a word, the level); the new word at `+8`.
const UPDATE_00647B70: u32 = 0x0064_7b70;

// Translated from 009f6c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f6c90 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only; a destructor): when the pointer at `+0x30`
/// (`00559450`) is set and its `009611e0` object is too, that object's slot `+0xe8` gets
/// the pointer; the pointer is then cleared (`0066b0d0` with 0). Finally the member's
/// destructor (`0045cec0`).
pub fn fn_009f6c90(e: &mut Engine, this: Ptr) {
    let member = this.addr() + 0x30;
    if e.call(READ_FIRST_WORD, &args![member]).u32() != 0 {
        let pointer = e.call(READ_FIRST_WORD, &args![member]).u32();
        if e.call(OWNER_009611E0, &args![pointer]).u32() != 0 {
            let pointer = e.call(READ_FIRST_WORD, &args![member]).u32();
            let owner = e.call(OWNER_009611E0, &args![pointer]).u32();
            let pointer = e.call(READ_FIRST_WORD, &args![member]).u32();
            e.vcall(owner, 0xe8, &args![pointer]);
        }
        e.call(POINTER_SET, &args![member, 0u32]);
    }
    e.call(POINTER_DESTRUCT, &args![member]);
}

// Translated from 009f6d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): refreshes the record. With an actor at `+0`
/// the time at `+0xc` becomes `00457fe0`'s (kept otherwise); the points at `+0x14` and
/// `+0x20` become the positions (slot `+0x1f4`) of the objects at `+0` and `+4` (kept
/// when they are null). The actor's slot `+0x428` object (`007031c0`, `00446390`) picks
/// a level and the float at `+0x10` (the compiler's switch, cases 0 to 4: 3 and 512.0, 2
/// and 256.0, 1 and 128.0, 3 and 512.0, 3 and 256.0; otherwise level 0 and the float
/// kept); `+8` becomes `00647b70` of it and the level.
pub fn fn_009f6d50(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    let actor = e.mem.u32(p);
    let time = if actor != 0 {
        e.call(CURRENT_TICKS, &[]).u32()
    } else {
        e.mem.u32(p + 0xc)
    };
    e.mem.set_u32(p + 0xc, time);
    let source = if e.mem.u32(p) != 0 {
        let actor = e.mem.u32(p);
        e.vcall(actor, 0x1f4, &[]).u32()
    } else {
        p + 0x14
    };
    let words = e.mem.bytes(source, 0xc);
    e.mem.write(p + 0x14, &words);
    let source = if e.mem.u32(p + 0x4) != 0 {
        let other = e.mem.u32(p + 0x4);
        e.vcall(other, 0x1f4, &[]).u32()
    } else {
        p + 0x20
    };
    let words = e.mem.bytes(source, 0xc);
    e.mem.write(p + 0x20, &words);
    let object = if e.mem.u32(p) != 0 {
        let actor = e.mem.u32(p);
        e.vcall(actor, 0x428, &[]).u32()
    } else {
        0
    };
    let record = if object != 0 {
        e.call(SLOT_OBJECT_007031C0, &args![object]).u32()
    } else {
        0
    };
    let mut level = 0u32;
    if record != 0 {
        let kind = e.call(KIND_00446390, &args![record]).u32();
        let pick = match kind {
            0 => Some((3, DISTANCE_512)),
            1 => Some((2, DISTANCE_256)),
            2 => Some((1, DISTANCE_128)),
            3 => Some((3, DISTANCE_512)),
            4 => Some((3, DISTANCE_256)),
            _ => None,
        };
        if let Some((picked, distance)) = pick {
            level = picked;
            let value = x87_quiet_bits(e.mem.u32(distance));
            e.mem.set_u32(p + 0x10, value);
        }
    }
    let word = e.mem.u32(p + 0x8);
    let updated = e.call(UPDATE_00647B70, &args![word, level]).u32();
    e.mem.set_u32(p + 0x8, updated);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_6c90, fn_009f6c90(Ptr)),
        entry!(0x009f_6d50, fn_009f6d50(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_009f6c90_hands_the_pointer_back_then_clears_it() {
        let mut e = Engine::new();
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(OWNER_009611E0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINTER_SET, |_, _| Ret::default());
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(0x70e8, |_, _| Ret::default());
        let table = e.mem.alloc(0xec);
        e.mem.set_u32(table + 0xe8, 0x70e8);
        let owner = e.mem.alloc(4);
        e.mem.set_u32(owner, table);
        let pointer = e.mem.alloc(8);
        e.mem.set_u32(pointer + 4, owner);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, pointer);
        e.call_log = Some(vec![]);
        e.call(0x009f_6c90, &args![this]);
        assert_eq!(calls(&e, 0x70e8), vec![vec![owner, pointer]]);
        assert_eq!(calls(&e, POINTER_SET), vec![vec![this + 0x30, 0]]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![this + 0x30]]);
        // No pointer: only the destructor.
        e.mem.set_u32(this + 0x30, 0);
        e.call_log = Some(vec![]);
        e.call(0x009f_6c90, &args![this]);
        assert!(calls(&e, POINTER_SET).is_empty());
        assert_eq!(calls(&e, POINTER_DESTRUCT).len(), 1);
    }

    fn doubles(e: &mut Engine) {
        for page in [DISTANCE_512, DISTANCE_256, DISTANCE_128] {
            e.map(page & !0xfff, 0x1000);
        }
        e.mem.set_f32(DISTANCE_512, 512.0);
        e.mem.set_f32(DISTANCE_256, 256.0);
        e.mem.set_f32(DISTANCE_128, 128.0);
        e.register(CURRENT_TICKS, |_, _| ret(0x7777));
        e.register(0x71f4, |_, a| ret(a[0] + 0x40));
        e.register(0x7428, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(SLOT_OBJECT_007031C0, |_, a| ret(a[0]));
        e.register(KIND_00446390, |e, a| ret(e.mem.u32(a[0])));
        e.register(UPDATE_00647B70, |_, a| ret(a[0] + a[1]));
    }

    fn actor(e: &mut Engine, point: [f32; 3]) -> u32 {
        let table = e.mem.alloc(0x430);
        e.mem.set_u32(table + 0x1f4, 0x71f4);
        e.mem.set_u32(table + 0x428, 0x7428);
        let p = e.mem.alloc(0x60);
        e.mem.set_u32(p, table);
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(p + 0x40 + 4 * i as u32, *v);
        }
        p
    }

    #[test]
    fn fn_009f6d50_refreshes_positions_and_level() {
        let mut e = Engine::new();
        doubles(&mut e);
        let first = actor(&mut e, [1.0, 2.0, 3.0]);
        let second = actor(&mut e, [4.0, 5.0, 6.0]);
        let record = e.mem.alloc(4);
        e.mem.set_u32(record, 1);
        e.mem.set_u32(first + 0x50, record);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this, first);
        e.mem.set_u32(this + 4, second);
        e.mem.set_u32(this + 8, 0x100);
        e.call(0x009f_6d50, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 0x7777);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.f32(this + 0x28), 6.0);
        assert_eq!(e.mem.f32(this + 0x10), 256.0);
        assert_eq!(e.mem.u32(this + 8), 0x102);
        // Kind 4: level 3, 256.0; kind 7: level 0 and the float kept.
        e.mem.set_u32(record, 4);
        e.mem.set_f32(this + 0x10, 1.0);
        e.call(0x009f_6d50, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x105);
        assert_eq!(e.mem.f32(this + 0x10), 256.0);
        e.mem.set_u32(record, 7);
        e.mem.set_f32(this + 0x10, 1.0);
        e.call(0x009f_6d50, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x105);
        assert_eq!(e.mem.f32(this + 0x10), 1.0);
        // No actors: the time and the points are kept.
        e.mem.set_u32(this, 0);
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 0xc, 5);
        e.call(0x009f_6d50, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.u32(this + 8), 0x105);
    }
}

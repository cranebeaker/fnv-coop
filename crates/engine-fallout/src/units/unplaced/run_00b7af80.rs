//! `(unplaced)/run_00b7af80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The per-id table of words (`011fdff8`, indexed by the id) and the global
/// that receives the id's word (`0126f74c`).
const ID_TABLE: u32 = 0x011f_dff8;
const CURRENT_WORD: u32 = 0x0126_f74c;
/// The `NiPointer` assignment `00a5f860` (`thiscall (pointer, object)`) and
/// the pair setter `00c663d0` (`thiscall (this, &source)`; the engine map
/// files it under `bhkworld.obj`).
const POINTER_ASSIGN: u32 = 0x00a5_f860;
const PAIR_SET: u32 = 0x00c6_63d0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word:
/// the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The inline `NiPointer` assignment of the word at `source` to the
/// pointer at `slot`: nothing when they are equal; otherwise the old object
/// released (`DeleteThis`, vtable +4, when its count reaches zero), the new
/// one stored and counted. With `reread` the source word is read again for
/// the store (the exe keeps it in a register for +0x28 only).
fn assign_pointer(e: &mut Engine, slot: u32, source: u32, reread: bool) {
    let old = e.mem.u32(slot);
    let new = e.mem.u32(source);
    if old == new {
        return;
    }
    if old != 0
        && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    let new = if reread { e.mem.u32(source) } else { new };
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

// Translated from 00b7af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects the pointers for an id (the engine map has no name; `thiscall
/// (id)`, `RET 4`): the id's word from `011fdff8` goes to `0126f74c`; the
/// pointer at +0x28 is assigned inline from +0x70 for the ids of the first
/// list below, or through `00a5f860` from +0x74, from +0x78, or (every
/// other id) from +0x6C. Ids 0x250 and 0x251 then take the pointers at
/// +0x30 and +0x34 from +0x7C and +0x80 (inline assignments); every other
/// id gives them +0x84 and +0x88 through `00c663d0`.
///
/// The compiler's four switch tables (byte maps at `00b7b144`, `00b7b24c`,
/// `00b7b2a4` and `00b7b390`, read from the exe's code) are the match arms
/// here.
pub fn fn_00b7af80(e: &mut Engine, this: Ptr, id: u32) {
    let object = this.addr();
    let word = e.mem.u32(ID_TABLE.wrapping_add(id.wrapping_mul(4)));
    e.set_global(CURRENT_WORD, word);
    let source = match id {
        0x3
        | 0x5
        | 0x8
        | 0x12..=0x13
        | 0x15
        | 0x5c..=0x5f
        | 0x6f..=0x79
        | 0x7d
        | 0x89..=0x92
        | 0x9b..=0x9d
        | 0xa6..=0xa8
        | 0xb1..=0xb3
        | 0xbc..=0xbe
        | 0xc6..=0xc7
        | 0xcd..=0xce
        | 0xd4..=0xd5
        | 0xdc..=0xdd
        | 0xe5..=0xe6
        | 0xee..=0xef
        | 0xf7..=0xf8
        | 0x105..=0x106
        | 0x109
        | 0x111..=0x112
        | 0x115
        | 0x11c..=0x11d
        | 0x120
        | 0x127..=0x128
        | 0x12b
        | 0x12e..=0x12f
        | 0x135..=0x136
        | 0x13b..=0x13c
        | 0x141..=0x142
        | 0x147..=0x148
        | 0x14d
        | 0x14f..=0x150
        | 0x152
        | 0x156..=0x157
        | 0x15d..=0x15e
        | 0x163..=0x164
        | 0x169..=0x16a
        | 0x16f..=0x170
        | 0x175..=0x176
        | 0x1e3
        | 0x1e6
        | 0x1e8
        | 0x1eb
        | 0x1ed
        | 0x232
        | 0x246..=0x247
        | 0x249
        | 0x24f
        | 0x251
        | 0x254..=0x255
        | 0x2d7
        | 0x2d9
        | 0x2df
        | 0x2e5
        | 0x2eb => None,
        0x61 | 0xfe | 0x10b | 0x1e9 => Some(0x74),
        0x93..=0x94 | 0x1ef..=0x230 | 0x233 => Some(0x78),
        _ => Some(0x6c),
    };
    match source {
        None => assign_pointer(e, object + 0x28, object + 0x70, false),
        Some(offset) => {
            let value = e.mem.u32(object + offset);
            e.call(POINTER_ASSIGN, &args![object + 0x28, value]);
        }
    }
    if id == 0x250 || id == 0x251 {
        assign_pointer(e, object + 0x30, object + 0x7c, true);
        assign_pointer(e, object + 0x34, object + 0x80, true);
    } else {
        e.call(PAIR_SET, &args![object + 0x30, object + 0x84]);
        e.call(PAIR_SET, &args![object + 0x34, object + 0x88]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b7_af80, fn_00b7af80(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(ID_TABLE, 0x1000);
        e.map(CURRENT_WORD, 4);
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        e.register(PAIR_SET, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let this = e.mem.alloc(0x8c);
        for offset in [0x6cu32, 0x74, 0x78] {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn assigned_from(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .replace(vec![])
            .unwrap()
            .into_iter()
            .filter(|call| call.0 == POINTER_ASSIGN)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn each_switch_range_picks_its_source() {
        let (mut e, this) = select_engine();
        // One id from each table: 0x61 (+0x74), 0x93 (+0x78), 0x4 (+0x6C),
        // 0x10b (+0x74), 0x1ef (+0x78), 0x233 (+0x78), 0x2ec (past the last
        // table: +0x6C), 0x151 (+0x6C).
        for (id, offset) in [
            (0x61u32, 0x74u32),
            (0x93, 0x78),
            (0x4, 0x6c),
            (0x10b, 0x74),
            (0x1ef, 0x78),
            (0x233, 0x78),
            (0x2ec, 0x6c),
            (0x151, 0x6c),
        ] {
            e.call(0x00b7_af80, &args![this, id]);
            assert_eq!(
                assigned_from(&mut e),
                vec![vec![this + 0x28, 0x1000 + offset]]
            );
        }
    }

    #[test]
    fn inline_ids_assign_the_counted_pointer_and_0x250_takes_the_pair() {
        let (mut e, this) = select_engine();
        e.set_global(ID_TABLE + 4 * 0x232, 0x55u32);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(this + 0x70, object);
        e.call(0x00b7_af80, &args![this, 0x232u32]);
        assert_eq!(e.global::<u32>(CURRENT_WORD), 0x55);
        assert_eq!(e.mem.u32(this + 0x28), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert!(assigned_from(&mut e).is_empty());
        // 0x250 (default source) takes the pointers at +0x7C and +0x80.
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(this + 0x7c, first);
        e.mem.set_u32(this + 0x80, second);
        e.call(0x00b7_af80, &args![this, 0x250u32]);
        assert_eq!(e.mem.u32(this + 0x30), first);
        assert_eq!(e.mem.u32(this + 0x34), second);
        assert_eq!(e.mem.u32(second + 4), 1);
        // Other ids hand the pairs to 00c663d0.
        e.call(0x00b7_af80, &args![this, 0x3u32]);
        let log = e.call_log.take().unwrap();
        let pairs: Vec<_> = log.iter().filter(|call| call.0 == PAIR_SET).collect();
        assert_eq!(pairs[0].1, vec![this + 0x30, this + 0x84]);
        assert_eq!(pairs[1].1, vec![this + 0x34, this + 0x88]);
    }
}

//! `Havok SDK/run_00d41710` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d41710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers two groups of
/// functions with `00cfbd60` on `queue`, both with the same 0x34-byte record built on the stack:
/// the function words `+0` = `0x00d40b90`, `+4` = `0x00d418b0`, `+8` = `0x00d41860`, `+0xc` =
/// `0x00d417b0`, `+0x10` = `0x00d417f0`, `+0x14` = `0x00d41830`, `+0x28` = `0x00d41930`, `+0x2c` =
/// `0x00d40d80`, zero words at `+0x18` .. `+0x24` and the bytes `+0x30` = `+0x31` = 1. The first
/// call has the byte at `+0x32` clear and the numbers `(2, 0x12)`; the second has it set and the
/// numbers `(0x12, 2)`. The padding byte at `+0x33` is uninitialised in the exe (zero here).
pub fn fn_00d41710(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(r + offset, 0);
        }
        for (offset, function) in [
            (0u32, 0x00d4_0b90u32),
            (0x2c, 0x00d4_0d80),
            (0x28, 0x00d4_1930),
            (0x08, 0x00d4_1860),
            (0x0c, 0x00d4_17b0),
            (0x10, 0x00d4_17f0),
            (0x14, 0x00d4_1830),
            (0x04, 0x00d4_18b0),
        ] {
            e.mem.set_u32(r + offset, function);
        }
        e.mem.set_u8(r + 0x30, 1);
        e.mem.set_u8(r + 0x31, 1);
        e.call(0x00cf_bd60, &args![queue, record, 2u32, 0x12u32]);
        e.mem.set_u8(r + 0x32, 1);
        e.call(0x00cf_bd60, &args![queue, record, 0x12u32, 2u32]);
    });
}

// Translated from 00d417b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, the first never read: `_unused_1`, `object`, `value`;
/// descriptions, not confirmed): searches the first `n` entries of 8 bytes from `object + 0x12`
/// (`n` is the byte at `object + 0xe`) for the 16-bit word equal to the low 16 bits of `value`;
/// when found answers `00dae340(object + 0xc, index)` (cdecl). Answers `n` when none matches and 0
/// when `n` is 0.
pub fn fn_00d417b0(e: &mut Engine, _unused_1: u32, object: u32, value: u32) -> u32 {
    let count = i32::from(e.mem.u8(object + 0xe));
    let base = object + 0xc;
    if count <= 0 {
        return 0;
    }
    let wanted = value as u16;
    let mut index = 0u32;
    let mut slot = base + 6;
    loop {
        if e.mem.u16(slot) == wanted {
            return e.call(0x00da_e340, &args![base, index]).u32();
        }
        index += 1;
        slot += 8;
        if index as i32 >= count {
            return index;
        }
    }
}

// Translated from 00d417f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, the first never read: `_unused_1`, `object`, `value`;
/// descriptions, not confirmed): the first of the `n` entries of 8 bytes whose 16-bit word at
/// `object + 0x12 + 8 * i` is `0xffff` (`n` is the byte at `object + 0xe`) gets the low 16 bits of
/// `value`. Answers the index reached: the free one, `n` when none is free, 0 when `n` is 0.
pub fn fn_00d417f0(e: &mut Engine, _unused_1: u32, object: u32, value: u32) -> u32 {
    let count = i32::from(e.mem.u8(object + 0xe));
    let mut index = 0u32;
    if count <= 0 {
        return 0;
    }
    let mut slot = object + 0x12;
    loop {
        if e.mem.u16(slot) == 0xffff {
            e.mem.set_u16(object + index * 8 + 0x12, value as u16);
            return index;
        }
        index += 1;
        slot += 8;
        if index as i32 >= count {
            return index;
        }
    }
}

// Translated from 00d41830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: three stack words (cdecl, nothing popped): `_unused_0`, `record` and
/// `id`. The record's byte at +0xe is a count of 8-byte entries that start at +0x10; the first
/// entry whose 16-bit word at +2 equals the low 16 bits of `id` gets its bytes at +0 and +1 set to 0.
/// Answers (in `EAX`) the index of that entry, the count when none matches, 0 when the count is 0.
/// Ghidra made no function here; the address is an immediate operand at 00d4176e.
pub fn fn_00d41830(e: &mut Engine, _unused_0: u32, record: u32, id: u32) -> u32 {
    let count = u32::from(e.mem.u8(record + 0xe));
    if count == 0 {
        return 0;
    }
    let id = id as u16;
    let mut index = 0u32;
    let mut entry = record + 0x10;
    loop {
        if e.mem.u16(entry + 2) == id {
            e.mem.set_u8(entry, 0);
            e.mem.set_u8(entry + 1, 0);
            return index;
        }
        index += 1;
        entry += 8;
        if index >= count {
            return index;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_1710, fn_00d41710(Ptr)),
        entry!(0x00d4_17b0, fn_00d417b0(u32, u32, u32) -> u32),
        entry!(0x00d4_17f0, fn_00d417f0(u32, u32, u32) -> u32),
        entry!(0x00d4_1830, fn_00d41830(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d41710_registers_the_group_twice() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bd60, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..12 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            for offset in [0x30, 0x31, 0x32] {
                words.push(u32::from(e.mem.u8(a[1] + offset)));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_1710, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        let mut expected = vec![
            0x00d4_0b90,
            0x00d4_18b0,
            0x00d4_1860,
            0x00d4_17b0,
            0x00d4_17f0,
            0x00d4_1830,
            0,
            0,
            0,
            0,
            0x00d4_1930,
            0x00d4_0d80,
            1,
            1,
            0,
        ];
        assert_eq!(seen[0][0], queue);
        assert_eq!(seen[0][2..4], [2, 0x12]);
        assert_eq!(seen[0][4..], expected[..]);
        expected[14] = 1;
        assert_eq!(seen[1][2..4], [0x12, 2]);
        assert_eq!(seen[1][4..], expected[..]);
    }

    #[test]
    fn fn_00d417b0_finds_the_entry_and_calls_00dae340() {
        let mut e = Engine::new();
        e.register(0x00da_e340, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x60);
        e.mem.set_u8(object + 0xe, 3);
        for (index, word) in [5u16, 9, 7].into_iter().enumerate() {
            e.mem.set_u16(object + 0x12 + index as u32 * 8, word);
        }
        e.call_log = Some(vec![]);
        let found = e.call(0x00d4_17b0, &args![0u32, object, 0x1_0009u32]).u32();
        assert_eq!(found, 0x4242);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00da_e340, vec![object + 0xc, 1])]);
        // Not found: the count; no entries: 0.
        assert_eq!(e.call(0x00d4_17b0, &args![0u32, object, 99u32]).u32(), 3);
        e.mem.set_u8(object + 0xe, 0);
        assert_eq!(e.call(0x00d4_17b0, &args![0u32, object, 5u32]).u32(), 0);
    }

    #[test]
    fn fn_00d417f0_stores_into_the_first_free_entry() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x60);
        e.mem.set_u8(object + 0xe, 3);
        for index in 0..3 {
            e.mem.set_u16(object + 0x12 + index * 8, 4);
        }
        e.mem.set_u16(object + 0x12 + 8, 0xffff);
        assert_eq!(
            e.call(0x00d4_17f0, &args![0u32, object, 0x2_0033u32]).u32(),
            1
        );
        assert_eq!(e.mem.u16(object + 0x12 + 8), 0x33);
        // No entry is free any more: the count; with none: 0.
        assert_eq!(e.call(0x00d4_17f0, &args![0u32, object, 1u32]).u32(), 3);
        e.mem.set_u8(object + 0xe, 0);
        assert_eq!(e.call(0x00d4_17f0, &args![0u32, object, 1u32]).u32(), 0);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// A record with `count` entries whose ids (word at +2) are `ids` and whose bytes at +0/+1 are 0xaa.
    fn record_with(e: &mut Engine, ids: &[u16]) -> u32 {
        let record = e.mem.alloc(0x40);
        e.mem.set_u8(record + 0xe, ids.len() as u8);
        for (i, id) in ids.iter().enumerate() {
            let entry = record + 0x10 + 8 * i as u32;
            e.mem.set_u8(entry, 0xaa);
            e.mem.set_u8(entry + 1, 0xaa);
            e.mem.set_u16(entry + 2, *id);
        }
        record
    }

    #[test]
    fn fn_00d41830_clears_the_first_entry_with_the_id() {
        let mut e = Engine::new();
        let record = record_with(&mut e, &[5, 7, 7]);
        // Only the low 16 bits of the word are compared.
        assert_eq!(
            e.call(0x00d4_1830, &args![0u32, record, 0x1_0007u32]).u32(),
            1
        );
        assert_eq!(e.mem.u8(record + 0x18), 0);
        assert_eq!(e.mem.u8(record + 0x19), 0);
        assert_eq!(e.mem.u8(record + 0x10), 0xaa);
        assert_eq!(e.mem.u8(record + 0x20), 0xaa);
    }

    #[test]
    fn fn_00d41830_answers_the_count_when_nothing_matches() {
        let mut e = Engine::new();
        let record = record_with(&mut e, &[5, 7]);
        assert_eq!(e.call(0x00d4_1830, &args![0u32, record, 9u32]).u32(), 2);
        assert_eq!(e.mem.u8(record + 0x10), 0xaa);
        assert_eq!(e.mem.u8(record + 0x18), 0xaa);
        let empty = record_with(&mut e, &[]);
        assert_eq!(e.call(0x00d4_1830, &args![0u32, empty, 9u32]).u32(), 0);
    }
}

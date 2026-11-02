//! `Havok SDK/hkclassenum.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkClassEnum` describes one enumeration of a Havok class: its name, the table of items
//! (each a value and a name, 8 bytes) and the number of items. The functions look the items up
//! by value or by name, decompose a flag word into item names and write the enum's signature
//! (name, then every item's name and value, then the count) to an `hkOArchive`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;

layout! {
    /// `hkClassEnum` (Xbox PDB), 0x14 bytes on the Xbox; the fields used so far.
    pub struct HkClassEnum: 0x14 {
        /// `m_name` (Xbox PDB): `const char*`.
        0x00 m_name: Ptr,
        /// `m_items` (Xbox PDB): `const hkClassEnum::Item*`, each item a value (`+0`) and a name
        /// (`+4`).
        0x04 m_items: Ptr,
        /// `m_numItems` (Xbox PDB).
        0x08 m_numItems: i32,
    }
}

/// `hkString::strCasecmp` (`00cb0fe0`, cdecl `(first, second)`) and `hkString::strLen`
/// (`00cb10d0`, cdecl `(text)`).
const STR_CASECMP: u32 = 0x00cb_0fe0;
const STR_LEN: u32 = 0x00cb_10d0;
/// `hkOArchive`: constructor `00d60830` (thiscall `(writer, byte-swap flag)`), `writeRaw`
/// `00d607b0` (`(bytes, length)`), `write32` `00d60940` (`(value)`) and the destructor `00d60590`.
const ARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
const ARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const ARCHIVE_WRITE32: u32 = 0x00d6_0940;
const ARCHIVE_DESTRUCT: u32 = 0x00d6_0590;
/// The CRC stream writer (`hkCrc32StreamWriter`): constructor `00db7870` (thiscall, `(seed)`)
/// and the result accessor `00db7860` (the finished CRC in `EAX`).
const CRC_WRITER_CONSTRUCT: u32 = 0x00db_7870;
const CRC_WRITER_RESULT: u32 = 0x00db_7860;
/// `hkArrayUtil` growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// Bits 0..29 of an array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00db7570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassEnum::getNameOfValue` (Xbox PDB), stack `(value, out)`, `RET 8`: finds the first item
/// whose value is `value`, stores its name pointer at `out` and answers 0; answers 1 when no item
/// has the value. The item count is read once (signed).
pub fn hk_class_enum_get_name_of_value(
    e: &mut Engine,
    this: Ptr<HkClassEnum>,
    value: u32,
    out: u32,
) -> u32 {
    let count = e.get(this, HkClassEnum::m_numItems);
    let items = e.get(this, HkClassEnum::m_items).addr();
    let mut index = 0;
    while index < count {
        let item = items.wrapping_add((index as u32).wrapping_mul(8));
        if e.mem.u32(item) == value {
            let name = e.mem.u32(item + 4);
            e.mem.set_u32(out, name);
            return 0;
        }
        index += 1;
    }
    1
}

// Translated from 00db75b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassEnum::getValueOfName` (Xbox PDB), stack `(name, out)`, `RET 8`: compares `name` with
/// each item's name through `hkString::strCasecmp` (`00cb0fe0`), the first equal one (answer 0)
/// has its value stored at `out`; answers 1 when none matches. The item count and the item table
/// are read again for every item.
pub fn hk_class_enum_get_value_of_name(
    e: &mut Engine,
    this: Ptr<HkClassEnum>,
    name: u32,
    out: u32,
) -> u32 {
    let mut index: i32 = 0;
    if e.get(this, HkClassEnum::m_numItems) > index {
        loop {
            let items = e.get(this, HkClassEnum::m_items).addr();
            let item = items.wrapping_add((index as u32).wrapping_mul(8));
            let item_name = e.mem.u32(item + 4);
            if e.call(STR_CASECMP, &args![name, item_name]).u32() == 0 {
                let items = e.get(this, HkClassEnum::m_items).addr();
                let value = e
                    .mem
                    .u32(items.wrapping_add((index as u32).wrapping_mul(8)));
                e.mem.set_u32(out, value);
                return 0;
            }
            index += 1;
            if index >= e.get(this, HkClassEnum::m_numItems) {
                break;
            }
        }
    }
    1
}

// Translated from 00db7600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassEnum::writeSignature` (Xbox PDB), stack `(writer)`, `RET 4`: makes an `hkOArchive` on
/// `writer` (its second argument is `this` with the low byte cleared; the constructor reads only
/// the byte), writes the enum's name without its terminator (`writeRaw`, length from `strLen`),
/// then for each item (the table is read again each time) its name the same way and its value
/// (`write32`), and last the item count (`write32`). The archive is destroyed at the end.
pub fn hk_class_enum_write_signature(e: &mut Engine, this: Ptr<HkClassEnum>, writer: u32) {
    e.with_stack(0x10, |e, archive| {
        let archive = archive.addr();
        e.call(
            ARCHIVE_CONSTRUCT,
            &args![archive, writer, this.addr() & 0xffff_ff00],
        );
        let name = e.get(this, HkClassEnum::m_name).addr();
        let length = e.call(STR_LEN, &args![name]).u32();
        e.call(ARCHIVE_WRITE_RAW, &args![archive, name, length]);
        let count = e.get(this, HkClassEnum::m_numItems);
        let mut index: i32 = 0;
        if count > 0 {
            loop {
                let items = e.get(this, HkClassEnum::m_items).addr();
                let item = items.wrapping_add((index as u32).wrapping_mul(8));
                let item_name = e.mem.u32(item + 4);
                let length = e.call(STR_LEN, &args![item_name]).u32();
                e.call(ARCHIVE_WRITE_RAW, &args![archive, item_name, length]);
                let value = e.mem.u32(item);
                e.call(ARCHIVE_WRITE32, &args![archive, value]);
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
        e.call(ARCHIVE_WRITE32, &args![archive, count]);
        e.call(ARCHIVE_DESTRUCT, &args![archive]);
    });
}

// Translated from 00db76c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The enum's signature (the map gives it no name): the CRC of what `writeSignature` writes.
/// Makes a 12-byte CRC stream writer with seed 0 (`00db7870`), runs `writeSignature` on it and
/// answers the writer's result (`00db7860`, left in `EAX`; the decompiler shows `void`).
pub fn fn_00db76c0(e: &mut Engine, this: Ptr<HkClassEnum>) -> u32 {
    e.with_stack(0xc, |e, writer| {
        let writer = writer.addr();
        e.call(CRC_WRITER_CONSTRUCT, &args![writer, 0u32]);
        hk_class_enum_write_signature(e, this, writer);
        e.call(CRC_WRITER_RESULT, &args![writer]).u32()
    })
}

// Translated from 00db76f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassEnum::decomposeFlags` (Xbox PDB), stack `(flags, names, rest)`, `RET 0xC`: empties the
/// array `names` (an `hkArray` of name pointers: data, size, capacity word), then walks the items
/// from the last to the first while any flag is left; an item whose value is entirely contained in
/// the remaining flags (`value & flags == value`, so an item of value 0 always) has its name
/// appended (growing the array through `00c90940` on the router's `+0x10` allocator when its size
/// equals its capacity) and its bits cleared from the flags. The flags left over are stored at
/// `rest`; answers whether any are left.
pub fn hk_class_enum_decompose_flags(
    e: &mut Engine,
    this: Ptr<HkClassEnum>,
    flags: u32,
    names: u32,
    rest: u32,
) -> bool {
    let mut flags = flags;
    e.mem.set_u32(names + 4, 0);
    let mut index = e.get(this, HkClassEnum::m_numItems).wrapping_sub(1);
    while index >= 0 && flags != 0 {
        let items = e.get(this, HkClassEnum::m_items).addr();
        let item = items.wrapping_add((index as u32).wrapping_mul(8));
        let value = e.mem.u32(item);
        if value & flags == value {
            let name = e.mem.u32(item + 4);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(names + 8) & CAPACITY_MASK;
            if e.mem.u32(names + 4) == capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_GROW, &args![allocator, names, 4u32]);
            }
            let size = e.mem.u32(names + 4);
            let data = e.mem.u32(names);
            e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), name);
            e.mem.set_u32(names + 4, size.wrapping_add(1));
            flags &= !value;
        }
        index = index.wrapping_sub(1);
    }
    e.mem.set_u32(rest, flags);
    flags != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00db_7570,
            hk_class_enum_get_name_of_value(Ptr<HkClassEnum>, u32, u32) -> u32
        ),
        entry!(
            0x00db_75b0,
            hk_class_enum_get_value_of_name(Ptr<HkClassEnum>, u32, u32) -> u32
        ),
        entry!(
            0x00db_7600,
            hk_class_enum_write_signature(Ptr<HkClassEnum>, u32)
        ),
        entry!(0x00db_76c0, fn_00db76c0(Ptr<HkClassEnum>) -> u32),
        entry!(
            0x00db_76f0,
            hk_class_enum_decompose_flags(Ptr<HkClassEnum>, u32, u32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    fn double(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An enum with the given (value, name) items; returns (enum, the name addresses).
    fn make_enum(
        e: &mut Engine,
        name: &[u8],
        items: &[(u32, &[u8])],
    ) -> (Ptr<HkClassEnum>, Vec<u32>) {
        let this: Ptr<HkClassEnum> = e.new_object();
        let enum_name = e.mem.alloc(32);
        e.mem.set_cstr(enum_name, name);
        e.set(this, HkClassEnum::m_name, Ptr::new(enum_name));
        let table = e.mem.alloc(8 * items.len().max(1) as u32);
        let mut names = vec![];
        for (index, (value, text)) in items.iter().enumerate() {
            let string = e.mem.alloc(32);
            e.mem.set_cstr(string, text);
            e.mem.set_u32(table + 8 * index as u32, *value);
            e.mem.set_u32(table + 8 * index as u32 + 4, string);
            names.push(string);
        }
        e.set(this, HkClassEnum::m_items, Ptr::new(table));
        e.set(this, HkClassEnum::m_numItems, items.len() as i32);
        (this, names)
    }

    #[test]
    fn the_name_of_a_value_is_the_first_item_with_that_value() {
        let (mut e, _log) = engine();
        let (this, names) = make_enum(&mut e, b"E", &[(1, b"A"), (2, b"B"), (2, b"C")]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00db_7570, &args![this, 2u32, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), names[1]);
        e.mem.set_u32(out, 0x55);
        assert_eq!(e.call(0x00db_7570, &args![this, 7u32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0x55);
    }

    #[test]
    fn the_name_of_a_value_with_no_items_is_not_found() {
        let (mut e, _log) = engine();
        let (this, _) = make_enum(&mut e, b"E", &[]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00db_7570, &args![this, 0u32, out]).u32(), 1);
    }

    #[test]
    fn the_value_of_a_name_asks_strcasecmp_for_each_item_in_turn() {
        let (mut e, log) = engine();
        let (this, names) = make_enum(&mut e, b"E", &[(10, b"A"), (20, b"B"), (30, b"C")]);
        let second = names[1];
        let seen = Rc::clone(&log);
        e.register_double(STR_CASECMP, move |_, a| {
            seen.borrow_mut().push((STR_CASECMP, a.to_vec()));
            Ret {
                eax: u32::from(a[1] != second),
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(4);
        let probe = 0x1234_5678;
        assert_eq!(e.call(0x00db_75b0, &args![this, probe, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 20);
        assert_eq!(
            calls(&log, STR_CASECMP),
            vec![vec![probe, names[0]], vec![probe, names[1]]]
        );
    }

    #[test]
    fn the_value_of_an_unknown_name_is_not_found_after_every_item_was_compared() {
        let (mut e, log) = engine();
        let (this, _names) = make_enum(&mut e, b"E", &[(10, b"A"), (20, b"B")]);
        double(&mut e, &log, STR_CASECMP, 1);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x77);
        assert_eq!(e.call(0x00db_75b0, &args![this, 5u32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0x77);
        assert_eq!(calls(&log, STR_CASECMP).len(), 2);
        // No items: no comparison at all.
        let (empty, _) = make_enum(&mut e, b"E", &[]);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00db_75b0, &args![empty, 5u32, out]).u32(), 1);
        assert!(calls(&log, STR_CASECMP).is_empty());
    }

    /// Installs doubles for the archive and string functions that record what a signature
    /// writes: ("raw", text) and ("w32", value) entries.
    fn install_archive(e: &mut Engine) -> Rc<RefCell<Vec<(String, u32)>>> {
        let written = Rc::new(RefCell::new(Vec::new()));
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(ARCHIVE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let raw = Rc::clone(&written);
        e.register_double(ARCHIVE_WRITE_RAW, move |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[1])[..a[2] as usize].to_vec()).unwrap();
            assert_eq!(text.len() as u32, a[2]);
            raw.borrow_mut().push((format!("raw:{text}"), a[2]));
            Ret::default()
        });
        let words = Rc::clone(&written);
        e.register_double(ARCHIVE_WRITE32, move |_, a| {
            words.borrow_mut().push(("w32".to_string(), a[1]));
            Ret::default()
        });
        e.register(ARCHIVE_DESTRUCT, |_, _| Ret::default());
        written
    }

    #[test]
    fn the_signature_is_the_name_then_every_item_then_the_count() {
        let (mut e, log) = engine();
        let written = install_archive(&mut e);
        double(&mut e, &log, ARCHIVE_CONSTRUCT, 0);
        let (this, _) = make_enum(&mut e, b"Mode", &[(1, b"On"), (5, b"Off")]);
        let writer = 0x0abc_0000;
        e.call(0x00db_7600, &args![this, writer]);
        let texts: Vec<String> = written
            .borrow()
            .iter()
            .map(|(text, _)| text.clone())
            .collect();
        assert_eq!(
            texts,
            vec!["raw:Mode", "raw:On", "w32", "raw:Off", "w32", "w32"]
        );
        let words: Vec<u32> = written
            .borrow()
            .iter()
            .filter(|(text, _)| text == "w32")
            .map(|(_, value)| *value)
            .collect();
        assert_eq!(words, vec![1, 5, 2]);
        let constructed = calls(&log, ARCHIVE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1], writer);
        assert_eq!(constructed[0][2], this.addr() & 0xffff_ff00);
    }

    #[test]
    fn the_signature_of_an_enum_without_items_is_the_name_and_a_zero_count() {
        let (mut e, _log) = engine();
        let written = install_archive(&mut e);
        let (this, _) = make_enum(&mut e, b"Empty", &[]);
        e.call(0x00db_7600, &args![this, 0x1000u32]);
        let texts: Vec<String> = written
            .borrow()
            .iter()
            .map(|(text, _)| text.clone())
            .collect();
        assert_eq!(texts, vec!["raw:Empty", "w32"]);
        assert_eq!(written.borrow()[1].1, 0);
    }

    #[test]
    fn the_signature_crc_is_the_writer_result_after_writing_the_signature() {
        let (mut e, log) = engine();
        let written = install_archive(&mut e);
        double(&mut e, &log, CRC_WRITER_CONSTRUCT, 0);
        double(&mut e, &log, CRC_WRITER_RESULT, 0xdead_beef);
        let (this, _) = make_enum(&mut e, b"E", &[(3, b"X")]);
        assert_eq!(e.call(0x00db_76c0, &args![this]).u32(), 0xdead_beef);
        let made = calls(&log, CRC_WRITER_CONSTRUCT);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1], 0);
        let texts: Vec<String> = written
            .borrow()
            .iter()
            .map(|(text, _)| text.clone())
            .collect();
        assert_eq!(texts, vec!["raw:E", "raw:X", "w32", "w32"]);
        assert_eq!(calls(&log, CRC_WRITER_RESULT), vec![vec![made[0][0]]]);
    }

    /// The router and growth doubles `decomposeFlags` needs: the array gets a four word buffer.
    fn install_growth(e: &mut Engine, log: &Log) -> u32 {
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        double(e, log, MEMORY_ROUTER, router);
        let log = Rc::clone(log);
        e.register_double(ARRAY_GROW, move |e, a| {
            log.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let buffer = e.mem.alloc(16);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        allocator
    }

    #[test]
    fn flags_decompose_from_the_last_item_and_leave_the_unknown_bits() {
        let (mut e, log) = engine();
        let allocator = install_growth(&mut e, &log);
        let (this, names) = make_enum(
            &mut e,
            b"Flags",
            &[(1, b"One"), (2, b"Two"), (4, b"Four"), (6, b"SixOnly")],
        );
        let array = e.mem.alloc(12);
        e.mem.set_u32(array + 4, 9);
        let rest = e.mem.alloc(4);
        // 0x17 has bits 0, 1, 2 and 4. From the last item: 6 fits (bits 1 and 2), leaving 0x11, so 4
        // and 2 no longer fit, then 1 fits and bit 4 stays.
        let got = e
            .call(0x00db_76f0, &args![this, 0x17u32, array, rest])
            .u32();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u32(rest), 0x10);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(e.mem.u32(data), names[3]);
        assert_eq!(e.mem.u32(data + 4), names[0]);
        // The first push found size 0 == capacity 0 and grew the array once.
        assert_eq!(calls(&log, ARRAY_GROW), vec![vec![allocator, array, 4]]);
    }

    #[test]
    fn flags_with_nothing_left_answer_false_and_a_value_zero_item_always_matches() {
        let (mut e, log) = engine();
        install_growth(&mut e, &log);
        let (this, names) = make_enum(&mut e, b"Flags", &[(0, b"None"), (1, b"One")]);
        let array = e.mem.alloc(12);
        let rest = e.mem.alloc(4);
        let got = e.call(0x00db_76f0, &args![this, 1u32, array, rest]).u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(rest), 0);
        // The flags ran out after the last item, so the zero item was never reached.
        assert_eq!(e.mem.u32(array + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(array)), names[1]);
        // A word that no item uses leaves everything, and the zero item still matches.
        let got = e.call(0x00db_76f0, &args![this, 8u32, array, rest]).u32();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u32(rest), 8);
        assert_eq!(e.mem.u32(array + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(array)), names[0]);
    }

    #[test]
    fn decomposing_zero_flags_clears_the_array_and_answers_false() {
        let (mut e, log) = engine();
        install_growth(&mut e, &log);
        let (this, _) = make_enum(&mut e, b"Flags", &[(1, b"One")]);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array + 4, 3);
        let rest = e.mem.alloc(4);
        e.mem.set_u32(rest, 9);
        assert_eq!(
            e.call(0x00db_76f0, &args![this, 0u32, array, rest]).u32(),
            0
        );
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(rest), 0);
    }
}

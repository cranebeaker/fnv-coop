//! `fallout shared/run_00438af0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slot (byte offset) called on the table with each removed entry,
/// first.
const FIRST_SLOT: u32 = 0x10;
/// Virtual slot (byte offset) called with the same entry, second.
const SECOND_SLOT: u32 = 0x18;

// Translated from 00438af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a chained hash table: the word at `+4` is the bucket count, the
/// word at `+8` the bucket array, each bucket the head of a singly linked
/// chain whose entries keep the next entry in their first word. Every entry
/// is unlinked from the front of its bucket and handed to the virtual
/// methods at vtable `+0x10` and then `+0x18` (their names are not known).
/// Finally the word at `+0xc` (the entry count) is zeroed.
pub fn fn_00438af0(e: &mut Engine, this: Ptr) {
    let mut bucket = 0u32;
    while bucket < e.mem.u32(this.addr() + 4) {
        loop {
            let buckets = e.mem.u32(this.addr() + 8);
            let entry = e.mem.u32(buckets + 4 * bucket);
            if entry == 0 {
                break;
            }
            let next = e.mem.u32(entry);
            e.mem.set_u32(buckets + 4 * bucket, next);
            e.vcall(this.addr(), FIRST_SLOT, &args![entry]);
            e.vcall(this.addr(), SECOND_SLOT, &args![entry]);
        }
        bucket += 1;
    }
    e.mem.set_u32(this.addr() + 0xc, 0);
}

// Translated from 00438b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): whether the words at
/// `+8` and `+0xc` of `this` are equal.
pub fn fn_00438b90(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) == e.mem.u32(this.addr() + 0xc)
}

/// `00402d00` (thiscall, no stack words): the constructor of the base part
/// `00438bb0` builds first; it installs the vtable `01011d54`, whose RTTI
/// names `TESFullName`.
const BASE_CONSTRUCTOR: u32 = 0x0040_2d00;

// Translated from 00438bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): a constructor. Runs
/// the base constructor `00402d00` on `this`, then clears the bytes at
/// `+0xc` and `+0xd`, the 16-bit word at `+0xe` and the word at `+0x10`.
/// Returns `this`.
pub fn fn_00438bb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u8(this.addr() + 0xc, 0);
    e.mem.set_u8(this.addr() + 0xd, 0);
    e.mem.set_u16(this.addr() + 0xe, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

/// `00591300` (thiscall, no stack words): empties the container at `+4` of
/// its `this`.
const REMOVE_ALL: u32 = 0x0059_1300;

// Translated from 00438bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): calls `00591300` on
/// `this`.
pub fn fn_00438bf0(e: &mut Engine, this: Ptr) {
    e.call(REMOVE_ALL, &args![this]);
}

/// `004858f0` (`cdecl`, chunk type and a byte): adds a one-byte chunk.
const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
/// `TESForm::AddChunkArray` (engine map, `004856f0`, `cdecl`: type, data,
/// size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `TESForm::AddChunk_ov2` (engine map, `00485950`, `cdecl`: type and a
/// 16-bit value).
const ADD_CHUNK_U16: u32 = 0x0048_5950;
/// `00485910` (`cdecl`, chunk type and a word): adds a four-byte chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `0048cee0` (`thiscall`, no stack word): the length of the name string
/// of the `TESFullName` at `this`.
const NAME_LENGTH: u32 = 0x0048_cee0;
/// `00408da0` (`thiscall`, no stack word; the engine map names the folded
/// body `MapMarkerData::GetLocationName`): the name's characters.
const NAME_TEXT: u32 = 0x0040_8da0;
/// `0084e3a0` (`thiscall`, no stack word): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;

// Translated from 00438c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object `00438bb0` builds (the engine map gives it no name; no
/// stack word): the byte at +0xc as chunk `FNAM` (`004858f0`), the name
/// (`00408da0`, length `0048cee0` plus 1) as `FULL`, the 16-bit value at
/// +0xe as `TNAM`, and, when the form at +0x10 is set, its id as `WMI1`.
pub fn fn_00438c10(e: &mut Engine, this: Ptr) {
    let flags = e.mem.u8(this.addr() + 0xc);
    e.call(ADD_CHUNK_BYTE, &args![0x4d41_4e46u32, u32::from(flags)]);
    let length = e.call(NAME_LENGTH, &args![this]).u32();
    let text = e.call(NAME_TEXT, &args![this]).u32();
    e.call(
        ADD_CHUNK_ARRAY,
        &args![0x4c4c_5546u32, text, length.wrapping_add(1)],
    );
    let kind = e.mem.u16(this.addr() + 0xe);
    e.call(ADD_CHUNK_U16, &args![0x4d41_4e54u32, u32::from(kind)]);
    let form = e.mem.u32(this.addr() + 0x10);
    if form != 0 {
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x3149_4d57u32, id]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0043_8c10, fn_00438c10(Ptr)),
        entry!(0x00438af0, fn_00438af0(Ptr)),
        entry!(0x0043_8b90, fn_00438b90(Ptr) -> bool),
        entry!(0x0043_8bb0, fn_00438bb0(Ptr) -> Ptr),
        entry!(0x0043_8bf0, fn_00438bf0(Ptr)),
    ]
}

#[cfg(test)]
mod b006_tests {
    use super::*;

    #[test]
    fn compares_the_words_at_8_and_c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 3);
        assert!(e.call(0x0043_8b90, &args![this]).bool());
        e.mem.set_u32(this + 0xc, 4);
        assert!(!e.call(0x0043_8b90, &args![this]).bool());
    }

    #[test]
    fn constructor_runs_the_base_and_clears_its_fields() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xff; 0x14]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0043_8bb0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let bases: Vec<_> = log.iter().filter(|c| c.0 == BASE_CONSTRUCTOR).collect();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].1, vec![this]);
        assert_eq!(e.mem.bytes(this + 0xc, 8), vec![0; 8]);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
    }

    #[test]
    fn forwards_to_remove_all() {
        let mut e = Engine::new();
        e.register(REMOVE_ALL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0043_8bf0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|c| c.0 == REMOVE_ALL).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0300_0000;
    const FIRST: u32 = 0x0070_0000;
    const SECOND: u32 = 0x0070_0010;

    /// A table with three buckets: bucket 0 holds entries a -> b, bucket 1 is
    /// empty, bucket 2 holds c. Returns (table, [a, b, c]).
    fn table(e: &mut Engine) -> (Ptr, [u32; 3]) {
        let mut slots = vec![0; 0x18 / 4 + 1];
        slots[0x10 / 4] = FIRST;
        slots[0x18 / 4] = SECOND;
        e.put_vtable(VTABLE, &slots);
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        let buckets = e.mem.alloc(12);
        let entries = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + 4, 3);
        e.mem.set_u32(this.addr() + 8, buckets);
        e.mem.set_u32(this.addr() + 0xc, 3);
        e.mem.set_u32(buckets, entries[0]);
        e.mem.set_u32(entries[0], entries[1]);
        e.mem.set_u32(buckets + 8, entries[2]);
        (this, entries)
    }

    #[test]
    fn unlinks_and_releases_every_entry_front_to_back() {
        let mut e = Engine::new();
        e.register(FIRST, |_, _| Ret::default());
        e.register(SECOND, |_, _| Ret::default());
        let (this, [a, b, c]) = table(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0043_8af0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            &log[1..],
            &[
                (FIRST, vec![this.addr(), a]),
                (SECOND, vec![this.addr(), a]),
                (FIRST, vec![this.addr(), b]),
                (SECOND, vec![this.addr(), b]),
                (FIRST, vec![this.addr(), c]),
                (SECOND, vec![this.addr(), c]),
            ]
        );
        let buckets = e.mem.u32(this.addr() + 8);
        assert_eq!(e.mem.u32(buckets), 0);
        assert_eq!(e.mem.u32(buckets + 8), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }

    #[test]
    fn an_empty_table_only_zeroes_the_count() {
        let mut e = Engine::new();
        let (this, _) = table(&mut e);
        e.mem.set_u32(this.addr() + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0043_8af0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00438c10_saves_flags_name_type_and_form() {
        let mut e = Engine::new();
        for addr in [
            ADD_CHUNK_BYTE,
            ADD_CHUNK_ARRAY,
            ADD_CHUNK_U16,
            ADD_CHUNK_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NAME_LENGTH, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register(NAME_TEXT, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0xc, 0x83);
        e.mem.set_u16(this + 0xe, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0043_8c10, &args![this]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0043_8c10)
            .collect();
        assert_eq!(
            log,
            vec![
                (ADD_CHUNK_BYTE, vec![0x4d41_4e46, 0x83]),
                (NAME_LENGTH, vec![this]),
                (NAME_TEXT, vec![this]),
                (ADD_CHUNK_ARRAY, vec![0x4c4c_5546, 0x7000, 6]),
                (ADD_CHUNK_U16, vec![0x4d41_4e54, 0x1234]),
            ]
        );
        e.mem.set_u32(this + 0x10, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x0043_8c10, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[log.len() - 2], (FORM_ID, vec![0x9000]));
        assert_eq!(
            log[log.len() - 1],
            (ADD_CHUNK_WORD, vec![0x3149_4d57, 0x9001])
        );
    }
}

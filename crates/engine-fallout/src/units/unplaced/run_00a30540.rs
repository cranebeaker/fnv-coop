//! `(unplaced)/run_00a30540` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a30540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the array of 0x10-byte entries at `[this + 0x14]` (count at
/// `this + 0x0C`): for each entry whose object pointer (+0x08) is not null,
/// calls that object's vtable slot `0xE0` with the entry's byte at +0x0C
/// (zero-extended). `ECX` only, no stack words.
pub fn fn_00a30540(e: &mut Engine, this: Ptr) {
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < e.mem.u32(this.addr() + 0x0c) {
        let entry = e.mem.u32(this.addr() + 0x14).wrapping_add(offset);
        let object = e.mem.u32(entry + 0x08);
        if object != 0 {
            let value = u32::from(e.mem.u8(entry + 0x0c));
            e.vcall(object, 0xe0, &args![value]);
        }
        index += 1;
        offset = offset.wrapping_add(0x10);
    }
}

// Translated from 00a30580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, one word: an object; `RET 4`): follows the chain of words at
/// +0x58 from `object`'s own +0x58 and answers false as soon as it meets `this`, true when
/// it reaches a null link first (`AL`; the rest of `EAX` keeps the last link, not
/// modelled).
pub fn fn_00a30580(e: &mut Engine, this: Ptr, object: Ptr) -> bool {
    let mut link = e.mem.u32(object.addr().wrapping_add(0x58));
    while link != 0 {
        if link == this.addr() {
            return false;
        }
        link = e.mem.u32(link.wrapping_add(0x58));
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_0540, fn_00a30540(Ptr)),
        entry!(0x00a3_0580, fn_00a30580(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00a30540_calls_slot_0xe0_of_each_present_object() {
        let mut e = Engine::new();
        e.put_vtable(0x0200_0000, &[0; 0x3c]);
        e.mem.set_u32(0x0200_0000 + 0xe0, 0x0090_0000);
        e.register(0x0090_0000, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let entries = e.mem.alloc(0x30);
        let first = e.mem.alloc(4);
        let third = e.mem.alloc(4);
        e.mem.set_u32(first, 0x0200_0000);
        e.mem.set_u32(third, 0x0200_0000);
        e.mem.set_u32(entries + 0x08, first);
        e.mem.set_u8(entries + 0x0c, 0xff);
        e.mem.set_u8(entries + 0x10 + 0x0c, 5);
        e.mem.set_u32(entries + 0x20 + 0x08, third);
        e.mem.set_u8(entries + 0x20 + 0x0c, 2);
        e.mem.set_u32(this + 0x0c, 3);
        e.mem.set_u32(this + 0x14, entries);
        e.call_log = Some(vec![]);
        e.call(0x00a3_0540, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == 0x0090_0000).collect();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].1, vec![first, 0xff]);
        assert_eq!(calls[1].1, vec![third, 2]);

        // An empty array calls nothing.
        e.mem.set_u32(this + 0x0c, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_0540, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00a30580_answers_whether_this_is_outside_the_chain() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x60);
        let b = e.mem.alloc(0x60);
        let c = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        // a -> b -> c -> null
        e.mem.set_u32(a + 0x58, b);
        e.mem.set_u32(b + 0x58, c);
        assert!(!e.call(0x00a3_0580, &args![c, a]).bool());
        assert!(!e.call(0x00a3_0580, &args![b, a]).bool());
        assert!(e.call(0x00a3_0580, &args![other, a]).bool());
        // The object itself is not compared, only its links.
        assert!(e.call(0x00a3_0580, &args![a, a]).bool());
        assert!(e.call(0x00a3_0580, &args![a, c]).bool());
    }
}

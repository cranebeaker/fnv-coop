//! `BSSystem/run_00afff50` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

/// The memory manager singleton, its `Allocate` (`00aa3e40`) and `Deallocate` (`00aa4060`),
/// thiscall, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `wcslen` (`00ecdb00`) and `memcpy` (`00ec44d0`), cdecl.
const WCSLEN: u32 = 0x00ec_db00;
const MEMCPY: u32 = 0x00ec_44d0;

/// A count clamped to a 16-bit length word (0xFFFF from 0x10000 on).
fn length_word(count: u32) -> u16 {
    if count > 0xffff {
        0xffff
    } else {
        count as u16
    }
}

// Translated from 00afff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSStringT<wchar_t>` set (thiscall, two words): makes the
/// string hold `text` (`wcslen` characters, 0 for null) with room for at least `size`
/// characters. Needing more than `sMaxLen`, it takes a new buffer of `(n + 1) * 2` bytes
/// from the memory manager (all ones when that overflows), copies `text` and its
/// terminator (or stores an empty text), frees the old buffer and records the room; with
/// nothing to hold it frees the buffer and clears the pointer and the room; otherwise it
/// copies into the buffer in place. `sLen` becomes the text's length (each length word
/// 0xFFFF from 0x10000 on). Answers whether the text is not empty.
pub fn fn_00afff50(e: &mut Engine, this: Ptr<BSStringT>, text: u32, size: u32) -> bool {
    let length = if text != 0 {
        e.call(WCSLEN, &args![text]).u32()
    } else {
        0
    };
    let mut room = if length > size { length } else { size };
    if room > u32::from(e.get(this, BSStringT::sMaxLen)) {
        let old = e.get(this, BSStringT::pString);
        let bytes = u64::from(room.wrapping_add(1)) * 2;
        let request = if bytes > u64::from(u32::MAX) {
            u32::MAX
        } else {
            bytes as u32
        };
        let buffer = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, request])
            .u32();
        e.set(this, BSStringT::pString, buffer);
        if text != 0 {
            let buffer = e.get(this, BSStringT::pString);
            e.call(
                MEMCPY,
                &args![buffer, text, length.wrapping_mul(2).wrapping_add(2)],
            );
        } else {
            let buffer = e.get(this, BSStringT::pString);
            e.mem.set_u16(buffer, 0);
        }
        if old != 0 {
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, old]);
        }
        e.set(this, BSStringT::sMaxLen, length_word(room));
    } else if room != 0 {
        let buffer = e.get(this, BSStringT::pString);
        if text != 0 {
            e.call(
                MEMCPY,
                &args![buffer, text, length.wrapping_mul(2).wrapping_add(2)],
            );
        } else {
            e.mem.set_u16(buffer, 0);
        }
    } else {
        if e.get(this, BSStringT::pString) != 0 {
            let old = e.get(this, BSStringT::pString);
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, old]);
        }
        e.set(this, BSStringT::pString, 0);
        room = 0;
        e.set(this, BSStringT::sMaxLen, length_word(room));
    }
    e.set(this, BSStringT::sLen, length_word(length));
    length != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00af_ff50,
        fn_00afff50(Ptr<BSStringT>, u32, u32) -> bool
    )]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register_double(WCSLEN, |_, w| (w[0] & 0xff).into_ret());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, w| {
            e.mem.alloc(w[1].min(0x1000)).into_ret()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn growing_takes_a_new_buffer() {
        let mut e = engine();
        let s: Ptr<BSStringT> = e.new_object();
        e.set(s, BSStringT::pString, 0x7000);
        e.set(s, BSStringT::sMaxLen, 2);
        // A text of 3 characters (the double measures the low byte of the address).
        assert!(e.call(0x00af_ff50, &args![s, 0x9003u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 8]]
        );
        let buffer = e.get(s, BSStringT::pString);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![buffer, 0x9003, 8]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7000]]
        );
        assert_eq!(e.get(s, BSStringT::sMaxLen), 3);
        assert_eq!(e.get(s, BSStringT::sLen), 3);
        // A null text with room asked for: an empty text in a new buffer.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00af_ff50, &args![s, 0u32, 0x10u32]).bool());
        let fresh = e.get(s, BSStringT::pString);
        assert_eq!(e.mem.u16(fresh), 0);
        assert_eq!(e.get(s, BSStringT::sMaxLen), 0x10);
        assert_eq!(e.get(s, BSStringT::sLen), 0);
        // Room past 0xFFFF is recorded as 0xFFFF.
        e.call(0x00af_ff50, &args![s, 0u32, 0x1_0000u32]);
        assert_eq!(e.get(s, BSStringT::sMaxLen), 0xffff);
        // An overflowing request asks for all ones.
        e.set(s, BSStringT::sMaxLen, 0);
        e.call_log = Some(vec![]);
        e.call(0x00af_ff50, &args![s, 0u32, 0x8000_0000u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, u32::MAX]]
        );
    }

    #[test]
    fn room_enough_copies_in_place_and_nothing_frees() {
        let mut e = engine();
        let s: Ptr<BSStringT> = e.new_object();
        let buffer = e.mem.alloc(0x20);
        e.set(s, BSStringT::pString, buffer);
        e.set(s, BSStringT::sMaxLen, 8);
        assert!(e.call(0x00af_ff50, &args![s, 0x9002u32, 4u32]).bool());
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![buffer, 0x9002, 6]]);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(e.get(s, BSStringT::sLen), 2);
        e.mem.set_u16(buffer, 0x41);
        assert!(!e.call(0x00af_ff50, &args![s, 0u32, 4u32]).bool());
        assert_eq!(e.mem.u16(buffer), 0);
        // Nothing to hold: the buffer is freed and the string emptied.
        assert!(!e.call(0x00af_ff50, &args![s, 0u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, buffer]]
        );
        assert_eq!(e.get(s, BSStringT::pString), 0);
        assert_eq!(e.get(s, BSStringT::sMaxLen), 0);
        e.call(0x00af_ff50, &args![s, 0u32, 0u32]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }
}

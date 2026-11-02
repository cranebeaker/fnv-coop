//! `BSCore/bscoremessage.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSCoreMessage::~BSCoreMessage` (the body `00aa4c80`, `run_00aa4960.rs`; `thiscall`, no
/// stack words), the heap the deleting destructor frees into (`011f6238`) and its free
/// (`00aa4060`, `thiscall`, one word).
const CORE_MESSAGE_DESTRUCTOR: u32 = 0x00aa_4c80;
const HEAP: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;

// Translated from 00aa4d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCoreMessage::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, one word of flags):
/// the destructor body `00aa4c80`, then with flag bit 0 the block freed into the heap at
/// `011f6238` (`00aa4060`). Answers `this`.
pub fn bs_core_message_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CORE_MESSAGE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_4d00,
        bs_core_message_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(CORE_MESSAGE_DESTRUCTOR, |_, _| Ret::default());
        e.register(HEAP_FREE, |_, _| Ret::default());
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_4d00, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert_eq!(e.call(0x00aa_4d00, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![HEAP, 0x5000]]);
        assert_eq!(calls_to(&e, CORE_MESSAGE_DESTRUCTOR), vec![vec![0x5000]; 2]);
    }
}

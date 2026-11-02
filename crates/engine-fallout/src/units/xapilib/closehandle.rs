//! `XAPILIB/closehandle.c` (Xbox PDB source unit), subsystem `XAPILIB`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of KERNEL32 `CloseHandle` (`stdcall`, one word).
const IMPORT_CLOSE_HANDLE: u32 = 0x00fd_f07c;

// Translated from 00d87920 (disassembled, FalloutNV.exe 1.4.0.525)
/// `CloseHandle` (engine map; `thiscall` on the address of a handle word, plain `RET`): a
/// non-null handle is closed (`CloseHandle`) and the word cleared. `EAX` is `CloseHandle`'s
/// answer then, and the null handle (0) otherwise.
pub fn close_handle(e: &mut Engine, this: u32) -> u32 {
    let handle = e.mem.u32(this);
    if handle == 0 {
        return handle;
    }
    let closed = e.call(IMPORT_CLOSE_HANDLE, &args![handle]).u32();
    e.mem.set_u32(this, 0);
    closed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_7920, close_handle(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_set_handle_is_closed_and_cleared() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(IMPORT_CLOSE_HANDLE, |_, _| 1u32.into_ret());
        let word = e.mem.alloc(4);
        assert_eq!(e.call(0x00d8_7920, &args![word]).u32(), 0);
        e.mem.set_u32(word, 0x44);
        assert_eq!(close_handle(&mut e, word), 1);
        assert_eq!(e.mem.u32(word), 0);
        let log = e.call_log.take().unwrap();
        let closes: Vec<_> = log.iter().filter(|c| c.0 == IMPORT_CLOSE_HANDLE).collect();
        assert_eq!(closes.len(), 1);
        assert_eq!(closes[0].1, vec![0x44]);
    }
}

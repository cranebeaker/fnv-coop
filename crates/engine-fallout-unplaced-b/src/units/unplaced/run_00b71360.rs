//! `(unplaced)/run_00b71360` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object pointer `011f91e0` points at, and the three handlers `fn_00b71360` picks
/// between (`cdecl`, `(first, second, value)`).
const SELECTOR_HOLDER: u32 = 0x011f_91e0;
const HANDLER_00B70820: u32 = 0x00b7_0820;
const HANDLER_00B70DE0: u32 = 0x00b7_0de0;
const HANDLER_00B71130: u32 = 0x00b7_1130;

// Translated from 00b71360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, `(first, second, value)`, `value` a float): when the word
/// at +0x68 of the object at +0xC0 of `*[011f91e0]` is not 0x11, `00b70820`; otherwise, when
/// the byte at `second + 0xF4` is set, `00b71130`, else `00b70de0`; each with the same three
/// words.
pub fn fn_00b71360(e: &mut Engine, first: u32, second: Ptr, value: f32) {
    let holder = e.global::<u32>(SELECTOR_HOLDER);
    let object = e.mem.u32(holder);
    let inner = e.mem.u32(object.wrapping_add(0xc0));
    let value = x87_quiet_bits(value.to_bits());
    let handler = if e.mem.u32(inner.wrapping_add(0x68)) != 0x11 {
        HANDLER_00B70820
    } else if e.mem.u8(second.addr().wrapping_add(0xf4)) != 0 {
        HANDLER_00B71130
    } else {
        HANDLER_00B70DE0
    };
    e.call(handler, &args![first, second, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b71360, fn_00b71360(u32, Ptr, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b71360_picks_the_handler() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for handler in [HANDLER_00B70820, HANDLER_00B70DE0, HANDLER_00B71130] {
            e.register(handler, |_, _| Ret::default());
        }
        e.map(SELECTOR_HOLDER, 4);
        let object = e.mem.alloc(0xc4);
        let inner = e.mem.alloc(0x6c);
        e.mem.set_u32(object + 0xc0, inner);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        e.set_global::<u32>(SELECTOR_HOLDER, holder);
        let second = e.mem.alloc(0xf8);
        e.call(0x00b71360, &args![1u32, second, 0.5f32]);
        e.mem.set_u32(inner + 0x68, 0x11);
        e.call(0x00b71360, &args![1u32, second, 0.5f32]);
        e.mem.set_u8(second + 0xf4, 1);
        e.call(0x00b71360, &args![1u32, second, 0.5f32]);
        let log = e.call_log.take().unwrap();
        let handled: Vec<(u32, Vec<u32>)> = log.into_iter().filter(|c| c.0 != 0x00b71360).collect();
        let words = vec![1, second, 0.5f32.to_bits()];
        assert_eq!(
            handled,
            vec![
                (HANDLER_00B70820, words.clone()),
                (HANDLER_00B70DE0, words.clone()),
                (HANDLER_00B71130, words)
            ]
        );
    }
}

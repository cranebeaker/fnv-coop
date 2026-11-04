//! `(unplaced)/run_008041c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (thiscall): the byte at +4 (a form's type).
const FORM_TYPE: u32 = 0x0040_1170;
/// `00803e80` (thiscall, flag word).
const SET_FLAG: u32 = 0x0080_3e80;

// Translated from 008041c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `form` at +0x3C; when it is not null
/// and its type byte is 0x18 or 0x1A, calls `00803e80(0x100)`.
pub fn fn_008041c0(e: &mut Engine, this: Ptr, form: u32) {
    let t = this.addr();
    e.mem.set_u32(t + 0x3c, form);
    if e.mem.u32(t + 0x3c) == 0 {
        return;
    }
    let stored = e.mem.u32(t + 0x3c);
    if e.call(FORM_TYPE, &args![stored]).u32() != 0x18 {
        let stored = e.mem.u32(t + 0x3c);
        if e.call(FORM_TYPE, &args![stored]).u32() != 0x1a {
            return;
        }
    }
    e.call(SET_FLAG, &args![this, 0x100u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_41c0, fn_008041c0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008041c0_flags_types_18_and_1a() {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| Ret {
            eax: e.mem.u8(a[0] + 4) as u32,
            ..Ret::default()
        });
        e.register(SET_FLAG, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let form = e.mem.alloc(8);
        for (kind, flagged) in [(0x18u8, true), (0x1a, true), (0x19, false)] {
            e.mem.set_u8(form + 4, kind);
            e.call_log = Some(vec![]);
            e.call(0x0080_41c0, &args![this, form]);
            assert_eq!(e.mem.u32(this + 0x3c), form);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log.iter().filter(|c| c.0 == SET_FLAG).collect();
            assert_eq!(hits.len(), flagged as usize, "{kind:#x}");
            if flagged {
                assert_eq!(hits[0].1, vec![this, 0x100]);
            }
        }
        e.call_log = Some(vec![]);
        e.call(0x0080_41c0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

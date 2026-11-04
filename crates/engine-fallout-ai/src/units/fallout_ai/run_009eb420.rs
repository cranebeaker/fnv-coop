//! `fallout/ai/run_009eb420` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0096a2d0` (unnamed): constructs the member at `+0x1c` (`ECX` only).
const MEMBER_CONSTRUCT_0096A2D0: u32 = 0x0096_a2d0;
/// `004300a0` (unnamed): stores 0.0 in the float at `this` (`ECX` only).
const CLEAR_FLOAT: u32 = 0x0043_00a0;
/// `FLT_MAX` (`float`, `0x01092b60`).
const FLOAT_MAX: u32 = 0x0109_2b60;
/// `00435de0` (unnamed, `extradataobjects.cpp`): `thiscall`, one float (`RET 4`); stores
/// it at `this`.
const SET_TIME_STAMP: u32 = 0x0043_5de0;

// Translated from 009eb420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009eb420 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): a constructor. The member at `+0x1c`
/// (`0096a2d0`) and the float at `+0x34` (`004300a0`); then `+0` 0, `+4` -1, `+8` and `+0xc`
/// 0, the byte at `+0x10` 0, `+0x14`, `+0x18`, `+0x24` and `+0x28` 0, `+0x30` -1, and
/// `+0x34` the time stamp of `-FLT_MAX` (`00435de0` on a stack local). Returns `this`.
pub fn fn_009eb420(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(MEMBER_CONSTRUCT_0096A2D0, &args![p + 0x1c]);
    e.call(CLEAR_FLOAT, &args![p + 0x34]);
    e.mem.set_u32(p, 0);
    e.mem.set_u32(p + 0x4, 0xffff_ffff);
    e.mem.set_u32(p + 0x8, 0);
    e.mem.set_u32(p + 0xc, 0);
    e.mem.set_u8(p + 0x10, 0);
    e.mem.set_u32(p + 0x14, 0);
    e.mem.set_u32(p + 0x18, 0);
    e.mem.set_u32(p + 0x24, 0);
    e.mem.set_u32(p + 0x28, 0);
    e.mem.set_u32(p + 0x30, 0xffff_ffff);
    let negated = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
    let stamp = e.with_stack(4, |e, local| {
        e.call(SET_TIME_STAMP, &args![local, negated]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(p + 0x34, stamp);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009e_b420, fn_009eb420(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009eb420_initializes_the_fields() {
        let mut e = Engine::new();
        e.map(FLOAT_MAX & !0xfff, 0x1000);
        e.mem.set_f32(FLOAT_MAX, 8.0);
        e.register(MEMBER_CONSTRUCT_0096A2D0, |_, _| Ret::default());
        e.register(CLEAR_FLOAT, |_, _| Ret::default());
        e.register(SET_TIME_STAMP, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xcd; 0x38]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_b420, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMBER_CONSTRUCT_0096A2D0, vec![this + 0x1c])));
        assert!(log.contains(&(CLEAR_FLOAT, vec![this + 0x34])));
        let words: Vec<u32> = [0u32, 4, 8, 0xc, 0x14, 0x18, 0x24, 0x28, 0x30]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![0, 0xffff_ffff, 0, 0, 0, 0, 0, 0, 0xffff_ffff]);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x11), 0xcd);
        assert_eq!(e.mem.u32(this + 0x1c), 0xcdcd_cdcd);
        assert_eq!(e.mem.f32(this + 0x34), -8.0);
    }
}

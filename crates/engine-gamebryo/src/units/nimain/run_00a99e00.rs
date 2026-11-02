//! `NiMain/run_00a99e00` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The property base member copy (`00a5d7a0`, unnamed in the engine map; `thiscall`, clone
/// and cloning process).
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;

// Translated from 00a99e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the clone and the cloning process; `RET 8`; it
/// sits just before `NiFogProperty`'s functions in the exe): a property member copy: the
/// base copy (`00a5d7a0`), then the 16-bit flags at +0x18, the float at +0x1c (`FLD`/`FSTP`,
/// which quiets a signalling NaN) and the three words at +0x20 to +0x28 copied into the
/// clone.
pub fn fn_00a99e00(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    let source = this.addr();
    let target = clone.addr();
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(source + 0x18);
    e.mem.set_u16(target + 0x18, flags);
    let value = x87_quiet_bits(e.mem.u32(source + 0x1c));
    e.mem.set_u32(target + 0x1c, value);
    for offset in [0x20u32, 0x24, 0x28] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9e00, fn_00a99e00(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_base_then_the_fields() {
        let mut e = Engine::new();
        e.register(PROPERTY_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        let clone = e.mem.alloc(0x2c);
        e.mem.set_u16(this + 0x18, 0x1234);
        e.mem.set_u32(this + 0x1c, 0x7f80_0002);
        for (i, offset) in [0x20u32, 0x24, 0x28].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 + i as u32);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a9_9e00, &args![this, clone, 0x55u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(PROPERTY_COPY_MEMBERS, vec![this, clone, 0x55])));
        assert_eq!(e.mem.u16(clone + 0x18), 0x1234);
        assert_eq!(e.mem.u32(clone + 0x1c), 0x7fc0_0002);
        assert_eq!(e.mem.bytes(clone + 0x20, 12), e.mem.bytes(this + 0x20, 12));
    }
}

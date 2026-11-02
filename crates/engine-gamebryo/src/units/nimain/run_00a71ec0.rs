//! `NiMain/run_00a71ec0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a68d40`: the member copy of the base class (`thiscall` on the source: clone,
/// cloning process).
const BASE_COPY_MEMBERS: u32 = 0x00a6_8d40;

// Translated from 00a71ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `NiAmbientLight::CreateClone` (`00a9b3d0`) calls it on the
/// source light (`thiscall`, `RET 8`: clone, cloning process). Runs the base class's member
/// copy (`00a68d40`) and copies the light's own words: the float at `+0xC4` through the x87
/// stack (`FLD`/`FSTP`, a signalling NaN comes out quiet), then the words `+0xC8` to `+0xE8`
/// as they are.
pub fn fn_00a71ec0(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let source = this.addr();
    let target = clone.addr();
    let first = x87_quiet_bits(e.mem.u32(source + 0xc4));
    e.mem.set_u32(target + 0xc4, first);
    for offset in (0xc8..=0xe8).step_by(4) {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_1ec0, fn_00a71ec0(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_base_members_then_the_light_words() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let source = e.mem.alloc(0xf0);
        let target = e.mem.alloc(0xf0);
        // A signalling NaN at +0xC4 comes out quiet; the plain words are copied as they are.
        e.mem.set_u32(source + 0xc4, 0x7f80_0001);
        for (index, offset) in (0xc8..=0xe8).step_by(4).enumerate() {
            e.mem.set_u32(source + offset, 0x1000 + index as u32);
        }
        e.mem.set_u32(target + 0xec, 0xdead_beef);
        e.call_log = Some(vec![]);
        e.call(0x00a7_1ec0, &args![source, target, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|c| c.0 == BASE_COPY_MEMBERS).collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![source, target, 0x3000]);
        assert_eq!(e.mem.u32(target + 0xc4), 0x7fc0_0001);
        for (index, offset) in (0xc8..=0xe8).step_by(4).enumerate() {
            assert_eq!(e.mem.u32(target + offset), 0x1000 + index as u32);
        }
        assert_eq!(e.mem.u32(target + 0xec), 0xdead_beef);
    }
}

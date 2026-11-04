//! `(unplaced)/run_00717e30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00717e50` (`thiscall`, no stack word; the engine map's name,
/// `BaseProcess::GetActorPackageThatIsRunning`, may be a folded body's): the object
/// `00717e30` runs `00700350` on.
const MEMBER_OBJECT_00717E50: u32 = 0x0071_7e50;
/// `00700350` (`thiscall`, no stack word; the engine map names it
/// `BGSMenuPacker::DumpInfo`).
const RUN_00700350: u32 = 0x0070_0350;

// Translated from 00717e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `00700350` on the object
/// `00717e50` gives for `this + 0x178`; `00700350`'s result (`EAX`, `EDX`, `ST0`) is passed
/// back unchanged (the function returns right after the call).
pub fn fn_00717e30(e: &mut Engine, this: Ptr) -> Ret {
    let object = e
        .call(MEMBER_OBJECT_00717E50, &args![this.addr() + 0x178])
        .u32();
    e.call(RUN_00700350, &args![object])
}

/// The registered form of `00717e30`: `this`, the result passed back whole.
fn fn_00717e30_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00717e30(e, Ptr::new(words[0]))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x0071_7e30, fn_00717e30_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_member_objects_dump_and_passes_its_result_on() {
        let mut e = Engine::new();
        e.register(MEMBER_OBJECT_00717E50, |_, a| (a[0] + 1).into_ret());
        e.register(RUN_00700350, |_, a| Ret {
            eax: a[0] ^ 0xff,
            edx: 7,
            st0: 2.5,
        });
        let this = e.mem.alloc(0x180);
        let r = e.call(0x0071_7e30, &args![this]);
        assert_eq!(r.eax, (this + 0x179) ^ 0xff);
        assert_eq!((r.edx, r.st0), (7, 2.5));
    }
}

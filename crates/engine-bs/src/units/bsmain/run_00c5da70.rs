//! `BSMain/run_00c5da70` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmain::bsmasterparticlesystem::BSMasterParticleSystem;

/// The base constructor `00a5ecb0` (`thiscall`, one word, here 0; no Xbox
/// PDB name).
const NODE_CONSTRUCTOR_00A5ECB0: u32 = 0x00a5_ecb0;
/// The vtable of `BSMasterParticleSystem` (RTTI
/// `.?AVBSMasterParticleSystem@@`).
const VTABLE: u32 = 0x010c_2fb4;
/// The `NiTArray<NiParticles*, ..>` constructor `00c5ec10` (`thiscall`, two
/// 16-bit words: maximum size, grow-by).
const PARTICLES_ARRAY_CONSTRUCTOR: u32 = 0x00c5_ec10;
/// The vtable of `NiTPrimitiveArray<NiParticles*>` (RTTI
/// `.?AV?$NiTPrimitiveArray@PAVNiParticles@@@@`).
const PRIMITIVE_ARRAY_VTABLE: u32 = 0x010c_2fa8;
/// `NiTPointerListBase<..>::RemoveAll` (`00e74d40`, engine map; `thiscall`).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;

// Translated from 00c5da70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5da70 - the C++ exception unwinding frame (FS:[0], handler 00f2fd84) is not translated
/// The `BSMasterParticleSystem` constructor (`thiscall`; no Xbox PDB name;
/// its `CreateObject` `00c5e5a0` runs it): the base constructor
/// (`00a5ecb0(0)`), the class vtable, an empty `lEmitterObjList` (+0xAC:
/// head, tail, count cleared), the `ChildParticles` array at +0xCC
/// (`00c5ec10(0, 1)`, then the `NiTPrimitiveArray` vtable), -1 in
/// `uiIndex` (+0xC4) and `uiEditorIndex` (+0xC8), a maximum of 0x14
/// emitters, no particles per frame (+0xBC) or active emitters, no
/// iterator, the list emptied (`RemoveAll`) and a zero `fMasterPSysTime`.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_00c5da70(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) -> Ptr<BSMasterParticleSystem> {
    let base = this.addr();
    e.call(NODE_CONSTRUCTOR_00A5ECB0, &args![this, 0u32]);
    e.mem.set_u32(base, VTABLE);
    let list = base + 0xac;
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
    let array = base + 0xcc;
    e.call(PARTICLES_ARRAY_CONSTRUCTOR, &args![array, 0u32, 1u32]);
    e.mem.set_u32(array, PRIMITIVE_ARRAY_VTABLE);
    e.mem.set_u32(base + 0xc4, 0xffff_ffff);
    e.mem.set_u32(base + 0xc8, 0xffff_ffff);
    e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 0x14);
    e.mem.set_u16(base + 0xbc, 0);
    e.set(this, BSMasterParticleSystem::usActiveEmitterObjCount, 0);
    e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0);
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.mem.set_u32(base + 0xdc, x87_store32(0.0));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c5_da70,
        fn_00c5da70(Ptr<BSMasterParticleSystem>) -> Ptr<BSMasterParticleSystem>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_master_particle_system() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xe0);
        e.mem.write(this, &[0xcd; 0xe0]);
        for callee in [
            NODE_CONSTRUCTOR_00A5ECB0,
            PARTICLES_ARRAY_CONSTRUCTOR,
            LIST_REMOVE_ALL,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_da70, &args![this]).u32(), this);
        let log: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (NODE_CONSTRUCTOR_00A5ECB0, vec![this, 0]),
                (PARTICLES_ARRAY_CONSTRUCTOR, vec![this + 0xcc, 0, 1]),
                (LIST_REMOVE_ALL, vec![this + 0xac]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.bytes(this + 0xac, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0xcc), PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc4), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0xc8), 0xffff_ffff);
        assert_eq!(e.mem.u16(this + 0xba), 0x14);
        assert_eq!(e.mem.u16(this + 0xbc), 0);
        assert_eq!(e.mem.u16(this + 0xb8), 0);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(e.mem.u32(this + 0xdc), 0);
        // Untouched: the word between the list and the counts.
        assert_eq!(e.mem.u32(this + 0xd0), 0xcdcd_cdcd);
    }
}

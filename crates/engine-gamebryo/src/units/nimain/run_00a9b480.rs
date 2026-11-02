//! `NiMain/run_00a9b480` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable both functions set; its RTTI locator's type descriptor names
/// `.?AVNiAlphaAccumulator@@`.
const ALPHA_ACCUMULATOR_VTABLE: u32 = 0x010a_13ac;
/// The base-class constructor (engine map unit `nibacktofrontaccumulator.cpp`;
/// thiscall, no arguments).
const BACK_TO_FRONT_CONSTRUCTOR: u32 = 0x00aa_0f00;
/// The base-class destructor (engine map unit `nibacktofrontaccumulator.cpp`;
/// thiscall).
const BACK_TO_FRONT_DESTRUCTOR: u32 = 0x00aa_0dc0;

// Translated from 00a9b480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::NiAlphaAccumulator` (class from the vtable's RTTI;
/// fields from the Xbox PDB): runs the `NiBackToFrontAccumulator` base
/// constructor, clears `m_bSortByClosestPoint` (+0x31) and `m_bInterfaceSort`
/// (+0x32), sets the vtable and `m_bObserveNoSortHint` (+0x30) to 1. Returns
/// `this`.
pub fn ni_alpha_accumulator_ni_alpha_accumulator(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BACK_TO_FRONT_CONSTRUCTOR, &[base]);
    e.mem.set_u8(base + 0x31, 0);
    e.mem.set_u8(base + 0x32, 0);
    e.mem.set_u32(base, ALPHA_ACCUMULATOR_VTABLE);
    e.mem.set_u8(base + 0x30, 1);
    this
}

// Translated from 00a9b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::~NiAlphaAccumulator` (class from the vtable's RTTI):
/// sets the vtable, then jumps to the `NiBackToFrontAccumulator` destructor
/// (`00aa0dc0`), whose result it passes on.
pub fn ni_alpha_accumulator_destructor(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), ALPHA_ACCUMULATOR_VTABLE);
    e.call(BACK_TO_FRONT_DESTRUCTOR, &[this.addr()]).u32()
}

/// `00aa13e0` (`cdecl`, the size): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;

// Translated from 00a9b4b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiAlphaAccumulator`): a new 0x34-byte object (`00aa13e0`, the
/// `NiBackToFrontAccumulator` base constructor, the vtable, `m_bObserveNoSortHint` (+0x30)
/// 1, +0x31 and +0x32 cleared), or null when the allocation fails.
pub fn fn_00a9b4b0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BACK_TO_FRONT_CONSTRUCTOR, &[block]);
    e.mem.set_u32(block, ALPHA_ACCUMULATOR_VTABLE);
    e.mem.set_u8(block + 0x30, 1);
    e.mem.set_u8(block + 0x31, 0);
    e.mem.set_u8(block + 0x32, 0);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_b480,
            ni_alpha_accumulator_ni_alpha_accumulator(Ptr) -> Ptr
        ),
        entry!(0x00a9_b4a0, ni_alpha_accumulator_destructor(Ptr) -> u32),
        entry!(0x00a9_b4b0, fn_00a9b4b0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn constructor_runs_the_base_and_sets_the_sort_flags() {
        let mut e = Engine::new();
        // The base constructor writes its own vtable and flags.
        e.register(BACK_TO_FRONT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0102_0304);
            e.mem.set_u8(a[0] + 0x31, 7);
            e.mem.set_u8(a[0] + 0x32, 7);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b480, &[this]).u32(), this);
        assert_eq!(calls_to(&e, BACK_TO_FRONT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), ALPHA_ACCUMULATOR_VTABLE);
        assert_eq!(e.mem.u8(this + 0x30), 1);
        assert_eq!(e.mem.u8(this + 0x31), 0);
        assert_eq!(e.mem.u8(this + 0x32), 0);
    }

    #[test]
    fn destructor_sets_the_vtable_and_runs_the_base_destructor() {
        let mut e = Engine::new();
        e.register(BACK_TO_FRONT_DESTRUCTOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), ALPHA_ACCUMULATOR_VTABLE);
            0x55u32.into_ret()
        });
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b4a0, &[this]).u32(), 0x55);
        assert_eq!(calls_to(&e, BACK_TO_FRONT_DESTRUCTOR), vec![vec![this]]);
    }
}

#[cfg(test)]
mod create_tests {
    use super::*;

    #[test]
    fn creates_an_alpha_accumulator_or_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BACK_TO_FRONT_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0] + 0x30, &[0xaa; 4]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_b4b0, &args![]).u32();
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_ALLOCATE, vec![0x34])));
        assert!(log.contains(&(BACK_TO_FRONT_CONSTRUCTOR, vec![object])));
        assert_eq!(e.mem.u32(object), ALPHA_ACCUMULATOR_VTABLE);
        assert_eq!(e.mem.bytes(object + 0x30, 4), vec![1, 0, 0, 0xaa]);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_b4b0, &args![]).u32(), 0);
    }
}

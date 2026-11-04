//! `fallout shared/tempeffects/bstecreatetask.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00682490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTask` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006824c0` and frees 0xc bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn ni_task_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0068_24c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00682480 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f5afc without reading the receiver. The map names this body `NiTask::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682480(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_5afc
}

/// `NiTask`'s vtable (RTTI `.?AVNiTask@@`).
const NI_TASK_VTABLE: u32 = 0x0106_9674;
/// `00a5d3d0` (unnamed in the engine map; no stack word): the base
/// destructor body run after the vtable store.
const BASE_DESTRUCTOR_00A5D3D0: u32 = 0x00a5_d3d0;

// Translated from 006824c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTask` destructor body (unnamed in the engine map): stores `NiTask`'s
/// vtable, then runs the base destructor body `00a5d3d0`.
pub fn fn_006824c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TASK_VTABLE);
    e.call(BASE_DESTRUCTOR_00A5D3D0, &args![this]);
}

// Translated from 006824e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed by its body (the engine map's `NiTask::Clear` does not fit a
/// test): true when `m_eStatus` (+8, Xbox PDB) is 4 or 3.
pub fn fn_006824e0(e: &mut Engine, this: Ptr) -> bool {
    let status = e.mem.u32(this.addr() + 8);
    status == 4 || status == 3
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_24c0, fn_006824c0(Ptr)),
        entry!(0x0068_24e0, fn_006824e0(Ptr) -> bool),
        entry!(0x0068_2480, fn_00682480(Ptr) -> u32),
        entry!(0x0068_2490, ni_task_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn ni_task_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_2490, 0x0068_24c0, 0xc);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682480_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2480, &args![0x1000u32]).u32(), 0x011f_5afc);
        assert_eq!(e.call(0x0068_2480, &args![0u32]).u32(), 0x011f_5afc);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    #[test]
    fn fn_006824c0_stores_the_vtable_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR_00A5D3D0, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0068_24c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), NI_TASK_VTABLE);
        assert_eq!(log[1..], [(BASE_DESTRUCTOR_00A5D3D0, vec![this])]);
    }

    #[test]
    fn fn_006824e0_is_true_for_status_3_and_4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        for (status, expected) in [(0u32, false), (2, false), (3, true), (4, true), (5, false)] {
            e.mem.set_u32(this + 8, status);
            assert_eq!(e.call(0x0068_24e0, &args![this]).bool(), expected);
        }
    }
}

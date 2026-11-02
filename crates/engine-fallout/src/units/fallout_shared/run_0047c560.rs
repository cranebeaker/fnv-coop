//! `fallout shared/run_0047c560` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Slots in the 0x98-byte component array (`FormComponentCollection::pFCArray`,
/// Xbox PDB: `BaseFormComponent*[38]`).
const SLOTS: u32 = 0x26;

/// Calls vtable slot `slot` of every non-null component pointer of the
/// array at `array`, slot 0 first.
fn each_component(e: &mut Engine, array: Ptr, slot: u32) {
    for i in 0..SLOTS {
        let component = e.mem.u32(array.addr() + 4 * i);
        if component != 0 {
            e.vcall(component, slot, &args![]);
        }
    }
}

// Translated from 0047c560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs vtable slot +0x00 (`BaseFormComponent::InitializeDataComponent`,
/// Xbox PDB) on each non-null pointer of the 38-slot component array at
/// `this` (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_0047c560(e: &mut Engine, this: Ptr) {
    each_component(e, this, 0x0);
}

// Translated from 0047c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs vtable slot +0x04 (`BaseFormComponent::ClearDataComponent`, Xbox
/// PDB) on each non-null pointer of the 38-slot component array at `this`
/// (`ECX`, no stack words). The engine map gives it no name.
pub fn fn_0047c5b0(e: &mut Engine, this: Ptr) {
    each_component(e, this, 0x4);
}

// Translated from 0047c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the components of another 38-slot array (`thiscall`, `RET 4`;
/// the engine map gives it no name): nothing for a null `other`; otherwise
/// for each slot where both arrays hold a component, vtable slot +0x08
/// (`BaseFormComponent::CopyComponent`, Xbox PDB) of this one with the
/// other's.
pub fn fn_0047c600(e: &mut Engine, this: Ptr, other: u32) {
    if other == 0 {
        return;
    }
    for i in 0..SLOTS {
        let mine = e.mem.u32(this.addr() + 4 * i);
        if mine != 0 {
            let theirs = e.mem.u32(other.wrapping_add(4 * i));
            if theirs != 0 {
                e.vcall(mine, 0x8, &args![theirs]);
            }
        }
    }
}

// Translated from 0047c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the components with another 38-slot array (`thiscall`, `RET
/// 4`; the engine map gives it no name): true (they differ) for a null
/// `other`; otherwise, slot by slot, true at the first slot where this
/// array's component answers true to its vtable slot +0x0c
/// (`BaseFormComponent::CompareComponent`, Xbox PDB) with the other's
/// (null included), or where only the other array holds one. False when no
/// slot differs.
pub fn fn_0047c670(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if other == 0 {
        return true;
    }
    for i in 0..SLOTS {
        let mine = e.mem.u32(this.addr() + 4 * i);
        let theirs = e.mem.u32(other.wrapping_add(4 * i));
        if mine != 0 {
            if e.vcall(mine, 0xc, &args![theirs]).bool() {
                return true;
            }
        } else if theirs != 0 {
            return true;
        }
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_c600, fn_0047c600(Ptr, u32)),
        entry!(0x0047_c670, fn_0047c670(Ptr, u32) -> bool),
        entry!(0x0047_c560, fn_0047c560(Ptr)),
        entry!(0x0047_c5b0, fn_0047c5b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const INIT: u32 = 0x0300_0000;
    const CLEAR: u32 = 0x0300_0010;

    /// An array with components in slots 0, 5 and 37 (the last).
    fn setup() -> (Engine, Ptr, [u32; 3]) {
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[INIT, CLEAR]);
        e.register(INIT, |_, _| Ret::default());
        e.register(CLEAR, |_, _| Ret::default());
        let array: Ptr = Ptr::new(e.mem.alloc(0x98));
        let mut components = [0u32; 3];
        for (n, slot) in [0u32, 5, 37].into_iter().enumerate() {
            let c = e.mem.alloc(4);
            e.mem.set_u32(c, VTABLE);
            e.mem.set_u32(array.addr() + 4 * slot, c);
            components[n] = c;
        }
        e.call_log = Some(vec![]);
        (e, array, components)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn initializes_each_present_component() {
        let (mut e, array, c) = setup();
        e.call(0x0047_c560, &args![array]);
        assert_eq!(calls_to(&e, INIT), vec![vec![c[0]], vec![c[1]], vec![c[2]]]);
        assert!(calls_to(&e, CLEAR).is_empty());
    }

    #[test]
    fn clears_each_present_component() {
        let (mut e, array, c) = setup();
        e.call(0x0047_c5b0, &args![array]);
        assert_eq!(
            calls_to(&e, CLEAR),
            vec![vec![c[0]], vec![c[1]], vec![c[2]]]
        );
        assert!(calls_to(&e, INIT).is_empty());
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0100;
    const COPY: u32 = 0x0300_0100;
    const COMPARE: u32 = 0x0300_0110;

    /// Two arrays; the compare slot answers the byte at +4 of the component.
    fn setup() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &[0, 0, COPY, COMPARE]);
        e.register(COPY, |_, _| Ret::default());
        e.register(COMPARE, |e, a| Ret {
            eax: u32::from(e.mem.u8(a[0] + 4)),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x98);
        let other = e.mem.alloc(0x98);
        (e, this, other)
    }

    fn component(e: &mut Engine) -> u32 {
        let c = e.mem.alloc(8);
        e.mem.set_u32(c, VTABLE);
        c
    }

    #[test]
    fn at_0047c600_copies_where_both_have_one() {
        let (mut e, this, other) = setup();
        let a = component(&mut e);
        let b = component(&mut e);
        let c = component(&mut e);
        e.mem.set_u32(this, a);
        e.mem.set_u32(other, b);
        e.mem.set_u32(this + 4 * 5, c);
        e.mem.set_u32(other + 4 * 9, b);
        let d = component(&mut e);
        e.mem.set_u32(this + 4 * 37, d);
        e.mem.set_u32(other + 4 * 37, c);
        e.call_log = Some(vec![]);
        e.call(0x0047_c600, &args![this, other]);
        let copies: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|x| x.0 == COPY)
            .collect();
        assert_eq!(copies, vec![(COPY, vec![a, b]), (COPY, vec![d, c])]);
        e.call_log = Some(vec![]);
        e.call(0x0047_c600, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn at_0047c670_finds_the_first_difference() {
        let (mut e, this, other) = setup();
        assert!(e.call(0x0047_c670, &args![this, 0u32]).bool());
        assert!(!e.call(0x0047_c670, &args![this, other]).bool());
        let a = component(&mut e);
        e.mem.set_u32(this + 4 * 3, a);
        // The component compares with a null other: answers its byte (0).
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_c670, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (COMPARE, vec![a, 0]));
        e.mem.set_u8(a + 4, 1);
        assert!(e.call(0x0047_c670, &args![this, other]).bool());
        e.mem.set_u8(a + 4, 0);
        // Only the other holds one.
        let b = component(&mut e);
        e.mem.set_u32(other + 4 * 37, b);
        assert!(e.call(0x0047_c670, &args![this, other]).bool());
    }
}

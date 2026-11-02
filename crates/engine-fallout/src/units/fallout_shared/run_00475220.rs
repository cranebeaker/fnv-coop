//! `fallout shared/run_00475220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVBGSDestructibleObjectForm@@`).
const VTABLE: u32 = 0x0101_a6a0;
/// The `InitializeDataComponent` body the constructor calls (`008c6060`: a shared empty body; the engine map names it ActorPackage::NullInstanceData because the linker folded it).
const INITIALIZE_DATA: u32 = 0x008c_6060;

// Translated from 00475350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSDestructibleObjectForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVBGSDestructibleObjectForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_00475350(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

// Translated from 00475310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the decompiler shows a library destructor's name folded onto
/// it; `ECX` only, plain `RET`): when the pointer at +0x14 is not null, calls its vtable
/// slot +0x10 with 1 (a scalar deleting destructor shape: destroy and free).
pub fn fn_00475310(e: &mut Engine, this: Ptr) {
    let owned = e.mem.u32(this.addr() + 0x14);
    if owned != 0 {
        e.vcall(owned, 0x10, &args![1u32]);
    }
}

// Translated from 004752e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): a scalar deleting
/// destructor: runs `00475310`, frees the object through `00401030` (cdecl) when bit 0 of
/// `flags` is set, and returns the object.
pub fn fn_004752e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00475310(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00475220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00475220 - the exception-unwinding frame (FS:[0] chain, handler 00eeebeb, state variable) is not translated.
/// Unnamed in the engine map (`ECX` only, plain `RET`): a destructor body. For each index
/// below the byte count at +4, the pointer at that index of the array at +8 (when not null)
/// is destroyed and freed through `004752e0` with 1; then the array is freed through
/// `00401030` (cdecl) and `0044cbf0` runs on the member at +0x10.
pub fn fn_00475220(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u8(base + 4)) {
        let array = e.mem.u32(base + 8);
        let element = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
        if element != 0 {
            fn_004752e0(e, Ptr::new(element), 1);
        }
        index += 1;
    }
    let array = e.mem.u32(base + 8);
    e.call(0x0040_1030, &args![array]);
    e.call(0x0044_cbf0, &args![base + 0x10]);
}

// Translated from 00475380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the form component `BGSDestructibleObjectForm` (the vtable `0x0101_a6a0`
/// its constructor `00475350` stores; unnamed in the engine map; `ECX` only, plain `RET`):
/// stores that vtable, then runs `00478690` on the component.
pub fn fn_00475380(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(0x0047_8690, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00475350, fn_00475350(Ptr) -> Ptr),
        entry!(0x0047_5310, fn_00475310(Ptr)),
        entry!(0x0047_52e0, fn_004752e0(Ptr, u32) -> Ptr),
        entry!(0x0047_5220, fn_00475220(Ptr)),
        entry!(0x0047_5380, fn_00475380(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00475350_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x00475350,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x008c_6060, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_a6a0)],
        });
    }

    use super::*;

    /// An object whose pointer at +0x14 is `owned` (with a vtable whose slot +0x10 is a
    /// double at `0x0300_1010`).
    fn with_owned(e: &mut Engine, owned: bool) -> (u32, u32) {
        let object = e.mem.alloc(0x18);
        let inner = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x0300_1010]);
        e.mem.set_u32(inner, 0x0300_0000);
        if owned {
            e.mem.set_u32(object + 0x14, inner);
        }
        e.register(0x0300_1010, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        (object, inner)
    }

    #[test]
    fn fn_00475310_deletes_the_owned_object_when_present() {
        for owned in [false, true] {
            let mut e = Engine::new();
            let (object, inner) = with_owned(&mut e, owned);
            e.call_log = Some(vec![]);
            e.call(0x0047_5310, &args![object]);
            let log = e.call_log.take().unwrap();
            if owned {
                assert_eq!(log[1..], [(0x0300_1010, vec![inner, 1])]);
            } else {
                assert_eq!(log.len(), 1);
            }
        }
    }

    #[test]
    fn fn_004752e0_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            let mut e = Engine::new();
            let (object, inner) = with_owned(&mut e, true);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0047_52e0, &args![object, flags]).u32(), object);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(0x0300_1010, vec![inner, 1])];
            if frees {
                expected.push((0x0040_1030, vec![object]));
            }
            assert_eq!(log[1..], expected);
        }
    }

    #[test]
    fn fn_00475220_destroys_the_listed_entries_then_the_array() {
        let mut e = Engine::new();
        let (first, inner) = with_owned(&mut e, true);
        let (second, _) = with_owned(&mut e, false);
        let data = e.mem.alloc(0x20);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 8, second);
        e.mem.set_u8(data + 4, 3);
        e.mem.set_u32(data + 8, array);
        e.register(0x0044_cbf0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_5220, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0300_1010, vec![inner, 1]),
                (0x0040_1030, vec![first]),
                (0x0040_1030, vec![second]),
                (0x0040_1030, vec![array]),
                (0x0044_cbf0, vec![data + 0x10]),
            ]
        );
    }

    #[test]
    fn fn_00475380_stores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        e.register(0x0047_8690, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_5380, &args![component]);
        assert_eq!(e.mem.u32(component), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0047_8690, vec![component])]);
    }
}

//! `fallout shared/run_0058bb50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vtable `0x0103_2748` has the RTTI name
//! `NiTArray<BGSCameraPath *>` and `0x0103_2750`
//! `NiTPrimitiveArray<BGSCameraPath *>`; `0x0103_27f4` is `BGSCameraShot`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The vtable of `NiTArray<BGSCameraPath *>` (RTTI).
const CAMERA_PATH_ARRAY_VTABLE: u32 = 0x0103_2748;
/// The vtable of `NiTPrimitiveArray<BGSCameraPath *>` (RTTI).
const CAMERA_PATH_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0103_2750;
/// Allocates the element block of a `NiTArray` (`0096afc0`, cdecl count).
const ALLOCATE_ELEMENTS: u32 = 0x0096_afc0;
/// Frees the element block (`004ede70`, cdecl pointer).
const FREE_ELEMENTS: u32 = 0x004e_de70;
/// The game's deallocator (`00401030`, cdecl pointer).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 0058bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks through the forms of the array `this` (count `0084e3a0`, element
/// `NiFormArray::GetFormAt` `004d6170`) and asks each non-null one
/// `0058b510(first, second)` until one answers non-zero. When the answer
/// is set and `0058b470` on it is true, the answer is dropped unless
/// `008256d0` on `00891170(answer)` is true. Returns the answer (0 when
/// `first` is 0).
pub fn fn_0058bb50(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let mut found = 0u32;
    if first != 0 {
        let count = e.call(0x0084_e3a0, &args![this]).u32();
        let mut index = 0u32;
        while index < count {
            let form = e.call(0x004d_6170, &args![this, index]).u32();
            if form != 0 {
                found = e.call(0x0058_b510, &args![form, first, second]).u32();
            }
            if found != 0 {
                break;
            }
            index += 1;
        }
    }
    if found != 0 && e.call(0x0058_b470, &args![found]).bool() {
        let owner = e.call(0x0089_1170, &args![found]).u32();
        if !e.call(0x0082_56d0, &args![owner]).bool() {
            found = 0;
        }
    }
    found
}

// Translated from 0058bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTArray<BGSCameraPath *>`: stores its vtable and frees
/// the element block `m_pBase` (`004ede70`).
pub fn fn_0058bc00(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), CAMERA_PATH_ARRAY_VTABLE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ELEMENTS, &args![elements]);
}

// Translated from 0058bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTPrimitiveArray<BGSCameraPath *>`: the `NiTArray`
/// constructor `0058bcc0` with the capacity and growth step, then its own
/// vtable. Returns `this`.
pub fn fn_0058bc30(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    fn_0058bcc0(e, this, max_size as u16, grow_by as u16);
    e.mem
        .set_u32(this.addr(), CAMERA_PATH_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 0058bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `NiTArray<BGSCameraPath *>`: the
/// destructor `0058bc00`, then frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_0058bc60(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    fn_0058bc00(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0058bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the destructor `0058b830`: runs it, then
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058bc90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_b830, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0058bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTArray<BGSCameraPath *>`: stores the vtable, the
/// capacity and growth step, zeroes the sizes and allocates the element
/// block (`0096afc0`) when the capacity is not zero. Returns `this`.
pub fn fn_0058bcc0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), CAMERA_PATH_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let capacity = e.get(this, NiTArray::m_usMaxSize);
    let elements = if capacity != 0 {
        e.call(ALLOCATE_ELEMENTS, &args![u32::from(capacity)]).u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, elements);
    this
}

// Translated from 0058bd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058bd30 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5b (0x78 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_27f4`) is `BGSCameraShot`, so this is the constructor of `BGSCameraShot`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058bd30(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x0048_7420, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0103_27f4);
    e.mem.set_u32(base + 0x18, 0x0103_27cc);
    e.mem.set_u32(base + 0x30, 0x0103_27b8);
    e.call(0x0063_3c90, &args![base + 0x60, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x64, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x6c, 0x0u32]);
    e.call(0x0063_3c90, &args![base + 0x70, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x5bu32]);
    e.call(0x0058_bf50, &args![this]);
    e.call(0x0066_b0d0, &args![base + 0x60, 0x0u32]);
    e.call(0x0066_b0d0, &args![base + 0x64, 0x0u32]);
    e.call(0x0066_b0d0, &args![base + 0x6c, 0x0u32]);
    e.mem.set_u32(base + 0x68, 0x0);
    e.call(0x0066_b0d0, &args![base + 0x70, 0x0u32]);
    e.mem.set_u8(base + 0x74, 0xff);
    e.mem.set_u8(base + 0x75, 0x0);
    e.mem.set_u8(base + 0x76, 0x0);
    this
}

// Translated from 0058be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BGSCameraShot`: the destructor
/// `0058be90` (`BGSCameraShot::~BGSCameraShot` in the engine map), then
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058be60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_be90, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_bb50, fn_0058bb50(Ptr, u32, u32) -> u32),
        entry!(0x0058_bc00, fn_0058bc00(Ptr<NiTArray>)),
        entry!(
            0x0058_bc30,
            fn_0058bc30(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x0058_bc60,
            fn_0058bc60(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0058_bc90, fn_0058bc90(Ptr, u32) -> Ptr),
        entry!(
            0x0058_bcc0,
            fn_0058bcc0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x0058_bd30, fn_0058bd30(Ptr) -> Ptr),
        entry!(0x0058_be60, fn_0058be60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
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

    /// Forms 0, 0x100 and 0x200 at indexes 0..3; `0058b510` answers
    /// `match_form + 1` for the form `match_form`.
    fn search_engine(match_form: u32, flagged: bool, owner_ok: bool) -> Engine {
        let mut e = Engine::new();
        e.register(0x0084_e3a0, |_, _| 3u32.into_ret());
        e.register(0x004d_6170, |_, a| (a[1] * 0x100).into_ret());
        e.register_double(0x0058_b510, move |_, a| {
            u32::from(a[0] == match_form)
                .wrapping_mul(match_form + 1)
                .into_ret()
        });
        e.register_double(0x0058_b470, move |_, _| u32::from(flagged).into_ret());
        e.register(0x0089_1170, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(0x0082_56d0, move |_, _| u32::from(owner_ok).into_ret());
        e
    }

    #[test]
    fn fn_0058bb50_returns_the_first_answer() {
        let mut e = search_engine(0x100, false, false);
        e.call_log = Some(vec![]);
        let found = e.call(0x0058_bb50, &args![0x50u32, 7u32, 8u32]).u32();
        assert_eq!(found, 0x101);
        assert_eq!(
            calls_to(&e, 0x004d_6170),
            vec![vec![0x50, 0], vec![0x50, 1]]
        );
        assert_eq!(calls_to(&e, 0x0058_b510), vec![vec![0x100, 7, 8]]);
        assert_eq!(calls_to(&e, 0x0058_b470), vec![vec![0x101]]);
        assert!(calls_to(&e, 0x0089_1170).is_empty());
    }

    #[test]
    fn fn_0058bb50_drops_a_flagged_answer_whose_owner_check_fails() {
        let mut e = search_engine(0x200, true, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_bb50, &args![0x50u32, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0089_1170), vec![vec![0x201]]);
        assert_eq!(calls_to(&e, 0x0082_56d0), vec![vec![0x211]]);

        let mut e = search_engine(0x200, true, true);
        assert_eq!(
            e.call(0x0058_bb50, &args![0x50u32, 7u32, 8u32]).u32(),
            0x201
        );
    }

    #[test]
    fn fn_0058bb50_without_first_asks_nothing() {
        let mut e = search_engine(0x100, true, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_bb50, &args![0x50u32, 0u32, 8u32]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_0058bcc0_allocates_when_the_capacity_is_set() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| (0x9000 + a[0]).into_ret());
        let this: Ptr<NiTArray> = e.new_object();
        e.mem.write(this.addr(), &[0xcd; 0x10]);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0058_bcc0, &args![this, 0xffff_0005u32, 0x1_0003u32])
            .u32();
        assert_eq!(r, this.addr());
        assert_eq!(calls_to(&e, ALLOCATE_ELEMENTS), vec![vec![5]]);
        assert_eq!(e.mem.u32(this.addr()), CAMERA_PATH_ARRAY_VTABLE);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0x9005);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 5);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 3);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 0);

        e.call_log = Some(vec![]);
        e.call(0x0058_bcc0, &args![this, 0x1_0000u32, 1u32]);
        assert!(calls_to(&e, ALLOCATE_ELEMENTS).is_empty());
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
    }

    #[test]
    fn fn_0058bc30_builds_the_array_and_sets_its_vtable() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, _| 0x9000u32.into_ret());
        let this: Ptr<NiTArray> = e.new_object();
        let r = e.call(0x0058_bc30, &args![this, 4u32, 2u32]).u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), CAMERA_PATH_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 4);
        assert_eq!(e.get(this, NiTArray::m_usGrowBy), 2);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0x9000);
    }

    #[test]
    fn fn_0058bc00_and_its_deleting_destructor() {
        let mut e = Engine::new();
        e.register(FREE_ELEMENTS, noop);
        e.register(DEALLOCATE, noop);
        let this: Ptr<NiTArray> = e.new_object();
        e.set(this, NiTArray::m_pBase, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x0058_bc00, &args![this]);
        assert_eq!(calls_to(&e, FREE_ELEMENTS), vec![vec![0x9000]]);
        assert_eq!(e.mem.u32(this.addr()), CAMERA_PATH_ARRAY_VTABLE);

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_bc60, &args![this, 0u32]).u32(), this.addr());
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x0058_bc60, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FREE_ELEMENTS).len(), 2);
    }

    #[test]
    fn fn_0058bc90_and_fn_0058be60_free_only_with_bit_0() {
        for (addr, destructor) in [(0x0058_bc90, 0x0058_b830), (0x0058_be60, 0x0058_be90)] {
            let mut e = Engine::new();
            e.register(destructor, noop);
            e.register(DEALLOCATE, noop);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![0x4000u32, 0u32]).u32(), 0x4000);
            assert!(calls_to(&e, DEALLOCATE).is_empty());
            assert_eq!(e.call(addr, &args![0x4000u32, 1u32]).u32(), 0x4000);
            assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x4000]]);
            assert_eq!(calls_to(&e, destructor), vec![vec![0x4000], vec![0x4000]]);
        }
    }

    #[test]
    fn fn_0058bd30_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_bd30,
            size: 0x78,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0048_7420, &[A::T(0x30)]),
                (0x0063_3c90, &[A::T(0x60), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x64), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x6c), A::V(0x0)]),
                (0x0063_3c90, &[A::T(0x70), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5b)]),
                (0x0058_bf50, &[A::T(0x0)]),
                (0x0066_b0d0, &[A::T(0x60), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x64), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x6c), A::V(0x0)]),
                (0x0066_b0d0, &[A::T(0x70), A::V(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_27f4),
                (0x18, 4, 0x0103_27cc),
                (0x30, 4, 0x0103_27b8),
                (0x68, 4, 0x0),
                (0x74, 1, 0xff),
                (0x75, 1, 0x0),
                (0x76, 1, 0x0),
            ],
        });
    }
}

//! `fallout shared/run_0040ea20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of small functions after `BSExtraData`'s base: a
//! `ContainerItemExtra` (Xbox PDB, 0x0C bytes: `pOwnerForm` +0x00, the
//! owner global or rank +0x04, `fHealthMult` +0x08) applied to an extra
//! data list and its health clamp, two float helpers, `ExtraCell3D`'s
//! constructor and `BSExtraData`'s scalar deleting destructor.
//!
//! x87 note: the float helpers compare in extended precision; here the
//! differences are computed in `f64`, which is exact for every pair of
//! `float`s whose exponents differ by less than 30.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ExtraDataList`'s ownership setter (type 0x21, `XOWN` in
/// `extradatalist.rs`): `(list, owner form)`.
const SET_OWNER: u32 = 0x0041_9700;
/// Its owner-global setter (type 0x22, `XGLB`): `(list, global)`.
const SET_OWNER_GLOBAL: u32 = 0x0041_97d0;
/// Its owner-rank setter (type 0x23, `XRNK`; the map names it
/// `ExtraDataList::SetRadiation`): `(list, rank)`.
const SET_OWNER_RANK: u32 = 0x0041_98a0;
/// `ExtraDataList::SetHealth` (Xbox PDB): `(list, float)`.
const SET_HEALTH: u32 = 0x0041_9970;
/// Reads the byte at +4 of a form (`TESForm`'s type byte).
const FORM_TYPE: u32 = 0x0040_1170;
/// The larger of two floats (cdecl `(a, b)`: `b` when `a <= b`, else `a`),
/// in ST0.
const FLOAT_LARGER: u32 = 0x0040_4010;
/// The epsilon `0040ea20` passes to [`fn_0040eb10`] (a `float`).
const HEALTH_EPSILON: u32 = 0x0101_41cc;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `BSExtraData`'s base constructor `(this, type)`.
const EXTRA_DATA_CONSTRUCT: u32 = 0x0040_ec80;
/// `BSExtraData`'s base destructor (restores the base vtable).
const EXTRA_DATA_DESTRUCT: u32 = 0x0040_ecb0;
/// `ExtraCell3D`'s vtable (RTTI `.?AVExtraCell3D@@`).
const EXTRA_CELL_3D_VTABLE: u32 = 0x0101_4204;
/// Stores a pointer into a smart-pointer slot and, when it is not null,
/// takes a reference (`(slot, pointer)`).
const SMART_POINTER_INIT: u32 = 0x0063_3c90;
/// Assigns a pointer to a smart-pointer slot (releases the old one, takes
/// a reference on the new one) (`(slot, pointer)`).
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `operator delete` (cdecl, one argument).
const FREE: u32 = 0x0040_1030;

// Translated from 0040ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a `ContainerItemExtra` to the extra data list `list` (nothing
/// when `list` is null). With an owner form it sets the list's owner
/// (`00419700`), then, by the form's type byte (`00401170`): type 8 with a
/// word at +4 other than -1 sets the owner rank (`004198a0`), type 0x2A
/// with a non-null word at +4 the owner global (`004197d0`). Then, given
/// `health_form`, when `fHealthMult` is not within the epsilon at
/// `010141cc` of 1.0 ([`fn_0040eb10`]) and the form's slot 0x10 returns a
/// non-zero health, the list's health becomes health * `fHealthMult`
/// (`ExtraDataList::SetHealth`).
pub fn fn_0040ea20(e: &mut Engine, this: Ptr, list: Ptr, health_form: Ptr) {
    if list.is_null() {
        return;
    }
    // ContainerItemExtra::pOwnerForm (Xbox PDB) +0x00
    let owner = e.mem.u32(this.addr());
    if owner != 0 {
        e.call(SET_OWNER, &args![list, owner]);
        let owner = e.mem.u32(this.addr());
        let kind = e.call(FORM_TYPE, &args![owner]).u32();
        // ContainerItemExtra::iUnion (Xbox PDB) +0x04: rank or global
        let rank_or_global = e.mem.u32(this.addr() + 4);
        if kind == 8 {
            if rank_or_global != u32::MAX {
                e.call(SET_OWNER_RANK, &args![list, rank_or_global]);
            }
        } else if kind == 0x2a && rank_or_global != 0 {
            e.call(SET_OWNER_GLOBAL, &args![list, rank_or_global]);
        }
    }
    if health_form.is_null() {
        return;
    }
    let epsilon: f32 = e.global(HEALTH_EPSILON);
    // ContainerItemExtra::fHealthMult (Xbox PDB) +0x08
    let multiplier: f32 = e.mem.f32(this.addr() + 8);
    if fn_0040eb10(e, multiplier, 1.0, epsilon) {
        return;
    }
    let health = e.vcall(health_form.addr(), 0x10, &args![]).u32();
    if health == 0 {
        return;
    }
    // FILD of the zero-extended word, FMUL by the float, FSTP float.
    let multiplier: f32 = e.mem.f32(this.addr() + 8);
    let value = (health as f64 * multiplier as f64) as f32;
    e.call(SET_HEALTH, &args![list, value.to_bits()]);
}

// Translated from 0040eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `a` and `b` differ by less than `epsilon` (cdecl, three floats):
/// false when `epsilon` is below 0.0, otherwise `epsilon > |a - b|`, the
/// difference taken as `a - b` when `b < a` and as `b - a` otherwise
/// (also when either is NaN). Any NaN makes the result false.
pub fn fn_0040eb10(e: &mut Engine, a: f32, b: f32, epsilon: f32) -> bool {
    let zero: f64 = e.global(ZERO);
    // FCOMP: C0 alone (less) returns false; equal, greater and unordered go on.
    if (epsilon as f64) < zero {
        return false;
    }
    let difference = if (b as f64) < a as f64 {
        a as f64 - b as f64
    } else {
        b as f64 - a as f64
    };
    epsilon as f64 > difference
}

// Translated from 0040eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `ContainerItemExtra::fHealthMult` (Xbox PDB, +0x08) to `value`
/// clamped as `00404010(0.0, `[`fn_0040ebd0`]`(1.0, value))`.
pub fn fn_0040eb90(e: &mut Engine, this: Ptr, value: f32) {
    let smaller = fn_0040ebd0(e, 1.0, value);
    let clamped = e
        .call(FLOAT_LARGER, &args![0.0f32.to_bits(), smaller.to_bits()])
        .f32();
    e.mem.set_f32(this.addr() + 8, x87_quiet(clamped));
}

// Translated from 0040ebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smaller of two floats (cdecl): `a` when `b > a`, otherwise `b`
/// (also when either is NaN). The result goes through a `float` local
/// (FLD/FSTP), which quiets a signalling NaN.
pub fn fn_0040ebd0(_e: &mut Engine, a: f32, b: f32) -> f32 {
    if (b as f64) > a as f64 {
        x87_quiet(a)
    } else {
        x87_quiet(b)
    }
}

// Translated from 0040ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCell3D` constructor (class from its vtable's RTTI): the
/// `BSExtraData` base with type 2, the `ExtraCell3D` vtable, and its
/// `spCellNode` smart pointer (Xbox PDB, +0x0C) initialized to null and
/// then assigned null. Returns `this`.
// Translation incomplete: 0040ec00 - the C++ exception-unwinding frame (FS:[0]) that destroys the base if the smart pointer's set-up throws is not translated.
pub fn fn_0040ec00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, 2u32]);
    e.mem.set_u32(this.addr(), EXTRA_CELL_3D_VTABLE);
    e.call(SMART_POINTER_INIT, &args![this.addr() + 0xc, 0u32]);
    e.call(SMART_POINTER_ASSIGN, &args![this.addr() + 0xc, 0u32]);
    this
}

// Translated from 0040ecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSExtraData`'s scalar deleting destructor (slot 0 of its vtable
/// `01014210`): the base destructor, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0040ecd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040ea20, fn_0040ea20(Ptr, Ptr, Ptr)),
        entry!(0x0040eb10, fn_0040eb10(f32, f32, f32) -> bool),
        entry!(0x0040eb90, fn_0040eb90(Ptr, f32)),
        entry!(0x0040ebd0, fn_0040ebd0(f32, f32) -> f32),
        entry!(0x0040ec00, fn_0040ec00(Ptr) -> Ptr),
        entry!(0x0040ecd0, fn_0040ecd0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine_with_constants() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.map(HEALTH_EPSILON, 4);
        e.set_global(HEALTH_EPSILON, 0.0001f32);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn item_extra(e: &mut Engine, owner: u32, rank: u32, multiplier: f32) -> u32 {
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, owner);
        e.mem.set_u32(item + 4, rank);
        e.mem.set_f32(item + 8, multiplier);
        item
    }

    fn health_form(e: &mut Engine, health: u32) -> u32 {
        let vtable = 0x0150_0000;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0x0150_1000]);
        e.register_double(0x0150_1000, move |_, _| health.into_ret());
        let form = e.mem.alloc(8);
        e.mem.set_u32(form, vtable);
        form
    }

    fn double_owner_setters(e: &mut Engine, kind: u32) {
        e.register(SET_OWNER, |_, _| Ret::default());
        e.register(SET_OWNER_GLOBAL, |_, _| Ret::default());
        e.register(SET_OWNER_RANK, |_, _| Ret::default());
        e.register(SET_HEALTH, |_, _| Ret::default());
        e.register_double(FORM_TYPE, move |_, _| kind.into_ret());
    }

    #[test]
    fn faction_owner_sets_rank_and_scaled_health() {
        let mut e = engine_with_constants();
        double_owner_setters(&mut e, 8);
        let item = item_extra(&mut e, 0x0200_0000, 3, 0.5);
        let form = health_form(&mut e, 120);
        let list = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0040_ea20, &[item, list, form]);
        assert_eq!(calls_to(&e, SET_OWNER), vec![vec![list, 0x0200_0000]]);
        assert_eq!(calls_to(&e, SET_OWNER_RANK), vec![vec![list, 3]]);
        assert!(calls_to(&e, SET_OWNER_GLOBAL).is_empty());
        assert_eq!(
            calls_to(&e, SET_HEALTH),
            vec![vec![list, 60.0f32.to_bits()]]
        );
    }

    #[test]
    fn rank_minus_one_and_full_health_set_only_the_owner() {
        let mut e = engine_with_constants();
        double_owner_setters(&mut e, 8);
        let item = item_extra(&mut e, 0x0200_0000, u32::MAX, 1.0);
        let form = health_form(&mut e, 120);
        let list = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0040_ea20, &[item, list, form]);
        assert_eq!(calls_to(&e, SET_OWNER).len(), 1);
        assert!(calls_to(&e, SET_OWNER_RANK).is_empty());
        assert!(calls_to(&e, SET_HEALTH).is_empty());
        assert!(calls_to(&e, 0x0150_1000).is_empty());
    }

    #[test]
    fn npc_owner_sets_global_and_zero_health_is_skipped() {
        let mut e = engine_with_constants();
        double_owner_setters(&mut e, 0x2a);
        let item = item_extra(&mut e, 0x0200_0000, 0x0300_0000, 0.25);
        let form = health_form(&mut e, 0);
        let list = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0040_ea20, &[item, list, form]);
        assert_eq!(
            calls_to(&e, SET_OWNER_GLOBAL),
            vec![vec![list, 0x0300_0000]]
        );
        assert_eq!(calls_to(&e, 0x0150_1000).len(), 1);
        assert!(calls_to(&e, SET_HEALTH).is_empty());
    }

    #[test]
    fn no_owner_no_list_and_no_form() {
        let mut e = engine_with_constants();
        double_owner_setters(&mut e, 8);
        let item = item_extra(&mut e, 0, 3, 0.5);
        let list = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0040_ea20, &[item, 0, 0]);
        e.call(0x0040_ea20, &[item, list, 0]);
        assert!(calls_to(&e, SET_OWNER).is_empty());
        assert!(calls_to(&e, SET_HEALTH).is_empty());
    }

    #[test]
    fn approximately_equal_follows_the_flag_tests() {
        let mut e = engine_with_constants();
        let check = |e: &mut Engine, a: f32, b: f32, eps: f32| {
            e.call(0x0040_eb10, &args![a, b, eps]).bool()
        };
        assert!(check(&mut e, 1.0, 1.00001, 0.0001));
        assert!(check(&mut e, 1.00001, 1.0, 0.0001));
        assert!(!check(&mut e, 0.5, 1.0, 0.0001));
        assert!(!check(&mut e, 1.0, 1.0, -0.0001));
        assert!(!check(&mut e, 1.0, 1.0, 0.0));
        assert!(!check(&mut e, f32::NAN, 1.0, 0.1));
        assert!(!check(&mut e, 1.0, 1.0, f32::NAN));
    }

    #[test]
    fn smaller_returns_b_on_ties_and_nan() {
        let mut e = Engine::new();
        let min = |e: &mut Engine, a: f32, b: f32| e.call(0x0040_ebd0, &args![a, b]).f32();
        assert_eq!(min(&mut e, 1.0, 2.0), 1.0);
        assert_eq!(min(&mut e, 2.0, 1.0), 1.0);
        assert_eq!(min(&mut e, -0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert!(min(&mut e, 1.0, f32::NAN).is_nan());
        assert_eq!(min(&mut e, f32::NAN, 1.0), 1.0);
    }

    #[test]
    fn health_clamp_passes_the_smaller_to_the_larger() {
        let mut e = Engine::new();
        e.register(FLOAT_LARGER, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        let item = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0040_eb90, &args![item, 1.5f32]);
        assert_eq!(e.mem.f32(item + 8), 1.0);
        assert_eq!(
            calls_to(&e, FLOAT_LARGER),
            vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        e.call(0x0040_eb90, &args![item, -2.0f32]);
        assert_eq!(e.mem.f32(item + 8), 0.0);
    }

    #[test]
    fn extra_cell_3d_constructor_builds_base_vtable_and_pointer() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(SMART_POINTER_INIT, |_, _| Ret::default());
        e.register(SMART_POINTER_ASSIGN, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040_ec00, &[object]).u32();
        assert_eq!(result, object);
        assert_eq!(e.mem.u32(object), EXTRA_CELL_3D_VTABLE);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![object, 2]]);
        assert_eq!(
            calls_to(&e, SMART_POINTER_INIT),
            vec![vec![object + 0xc, 0]]
        );
        assert_eq!(
            calls_to(&e, SMART_POINTER_ASSIGN),
            vec![vec![object + 0xc, 0]]
        );
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_ecd0, &[object, 0]).u32(), object);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x0040_ecd0, &[object, 3]);
        assert_eq!(calls_to(&e, FREE), vec![vec![object]]);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 2);
    }
}

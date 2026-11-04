//! `fallout shared/run_0058b3f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00464f40` of unit tesdatahandler, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00464f40`'s translated signature `fn_00464f40(Ptr)`
/// returns nothing.
pub fn fn_0058b450(e: &mut Engine, this: Ptr) {
    e.call(0x0046_4f40, &args![this]);
}

// Translated from 0058b420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058b450` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058b420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_b450, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `NiFormArray`'s vtable (`010326fc`; its RTTI locator names `NiFormArray`).
const NI_FORM_ARRAY_VTABLE: u32 = 0x0103_26fc;
/// The base array constructor (`004701d0`, `thiscall(a, b)`).
const ARRAY_BASE_CONSTRUCT: u32 = 0x0047_01d0;
/// Returns the word at +0xC of its `this` (`0084e3a0`).
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `NiFormArray::GetFormAt` (`004d6170`, Xbox PDB; `thiscall(index)`).
const NI_FORM_ARRAY_GET_FORM_AT: u32 = 0x004d_6170;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl(object, vfdelta, source type,
/// target type, is reference)`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of `TESForm` (`01183028`) and `BGSCameraPath`
/// (`0118c268`), read from their decorated names.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const BGS_CAMERA_PATH_TYPE: u32 = 0x0118_c268;
/// The array pointer both readers below test (+0x2C of their `this`).
const FORM_ARRAY: u32 = 0x2c;

// Translated from 0058b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: builds the array base (`004701d0(this, 0, 1)`), then sets
/// `NiFormArray`'s vtable (`010326fc`). Returns `this`.
pub fn fn_0058b3f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ARRAY_BASE_CONSTRUCT, &args![this, 0u32, 1u32]);
    e.mem.set_u32(this.addr(), NI_FORM_ARRAY_VTABLE);
    this
}

// Translated from 0058b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The count (`0084e3a0`) of the array at +0x2C, or 0 when it is null.
pub fn fn_0058b600(e: &mut Engine, this: Ptr) -> u32 {
    let array = e.mem.u32(this.addr() + FORM_ARRAY);
    if array != 0 {
        return e.call(ARRAY_COUNT, &args![array]).u32();
    }
    0
}

// Translated from 0058b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form at `index` of the array at +0x2C (`NiFormArray::GetFormAt`),
/// dynamically cast from `TESForm` to `BGSCameraPath`; null when the array
/// is null.
pub fn fn_0058b630(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let array = e.mem.u32(this.addr() + FORM_ARRAY);
    if array == 0 {
        return Ptr::NULL;
    }
    let form = e
        .call(NI_FORM_ARRAY_GET_FORM_AT, &args![array, index])
        .u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TES_FORM_TYPE, BGS_CAMERA_PATH_TYPE, 0u32],
    )
    .ptr()
}

/// The flag byte of the camera path (+0x28) the small readers test.
const PATH_FLAGS: u32 = 0x28;
/// `00440d80` (`thiscall`, a byte answer), `00500940` (`thiscall`: the
/// path's conditions), `TESCondition::IsTrue` (`00680c30`, engine map;
/// `thiscall`, two words), `00891170` (`thiscall`) and `008256d0`
/// (`thiscall` on what `00891170` answers, a byte answer).
const PATH_00440D80: u32 = 0x0044_0d80;
const PATH_CONDITIONS: u32 = 0x0050_0940;
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
const PATH_00891170: u32 = 0x0089_1170;
const CHECK_008256D0: u32 = 0x0082_56d0;
/// `NiFormArray::GetIndexForForm` (`004d6100`, engine map; `thiscall`,
/// one word), `00486b80` (`thiscall` on the array, one word: an index) and
/// `0061bdc0` (`thiscall` on the array), and `008d7dc0` (`thiscall`, one
/// word; its engine map name `LowProcess::SetItemBeingUsed` is a folded
/// body's).
const GET_INDEX_FOR_FORM: u32 = 0x004d_6100;
const ARRAY_00486B80: u32 = 0x0048_6b80;
const ARRAY_0061BDC0: u32 = 0x0061_bdc0;
const SETTER_008D7DC0: u32 = 0x008d_7dc0;

// Translated from 0058b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x80 of the flag byte at +0x28 is set.
pub fn fn_0058b470(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + PATH_FLAGS) & 0x80 != 0
}

// Translated from 0058b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether neither bit 0x1 nor bit 0x2 of the flag byte at +0x28
/// is set.
pub fn fn_0058b4a0(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(this.addr() + PATH_FLAGS) & 1 != 0 {
        return false;
    }
    e.mem.u8(this.addr() + PATH_FLAGS) & 2 == 0
}

// Translated from 0058b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x1 of the flag byte at +0x28 is set.
pub fn fn_0058b4d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + PATH_FLAGS) & 1 != 0
}

// Translated from 0058b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x2 of the flag byte at +0x28 is set.
pub fn fn_0058b4f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + PATH_FLAGS) & 2 != 0
}

// Translated from 0058b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, two words, `RET 8`): the camera path to use
/// for the two words (passed to the conditions), or null. Null when
/// `00440d80` answers true or the conditions (`00500940`) are not true for
/// the two words. Otherwise the path itself is the answer when bit 0x80 of
/// its flags is set or `008256d0` on what `00891170` answers is false; then
/// the children (`0058b630`, for each index below `0058b600`) are asked
/// the same way in order, and the first non-null answer replaces it.
pub fn fn_0058b510(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let mut result = 0;
    if e.call(PATH_00440D80, &args![this]).bool() {
        return result;
    }
    let conditions = e.call(PATH_CONDITIONS, &args![this]).u32();
    if !e
        .call(CONDITION_IS_TRUE, &args![conditions, first, second])
        .bool()
    {
        return result;
    }
    let mut take = fn_0058b470(e, this);
    if !take {
        let object = e.call(PATH_00891170, &args![this]).u32();
        take = !e.call(CHECK_008256D0, &args![object]).bool();
    }
    if take {
        result = this.addr();
    }
    let count = fn_0058b600(e, this);
    if count == 0 {
        return result;
    }
    let mut found = 0;
    let mut index = 0;
    while index < count {
        let child = fn_0058b630(e, this, index);
        if !child.is_null() {
            found = fn_0058b510(e, child, first, second);
        }
        if found != 0 {
            break;
        }
        index += 1;
    }
    if found != 0 {
        result = found;
    }
    result
}

// Translated from 0058b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of `form` in the array at +0x2C
/// (`NiFormArray::GetIndexForForm`), or -1 when the array is null.
pub fn fn_0058b680(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    let array = e.mem.u32(this.addr() + FORM_ARRAY);
    if array == 0 {
        return 0xffff_ffff;
    }
    e.call(GET_INDEX_FOR_FORM, &args![array, form]).u32()
}

// Translated from 0058b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): with the array at +0x2C
/// set, takes the paths at `index + 1` and `index - 1` (`0058b630`), gives
/// the second to `008d7dc0` on the first when there is a first, then runs
/// `00486b80(index)` and `0061bdc0` on the array.
pub fn fn_0058b6c0(e: &mut Engine, this: Ptr, index: u32) {
    if e.mem.u32(this.addr() + FORM_ARRAY) == 0 {
        return;
    }
    let next = fn_0058b630(e, this, index.wrapping_add(1));
    let previous = fn_0058b630(e, this, index.wrapping_sub(1));
    if !next.is_null() {
        e.call(SETTER_008D7DC0, &args![next, previous]);
    }
    let array = e.mem.u32(this.addr() + FORM_ARRAY);
    e.call(ARRAY_00486B80, &args![array, index]);
    let array = e.mem.u32(this.addr() + FORM_ARRAY);
    e.call(ARRAY_0061BDC0, &args![array]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_b450, fn_0058b450(Ptr)),
        entry!(0x0058_b420, fn_0058b420(Ptr, u32) -> Ptr),
        entry!(0x0058_b3f0, fn_0058b3f0(Ptr) -> Ptr),
        entry!(0x0058_b600, fn_0058b600(Ptr) -> u32),
        entry!(0x0058_b630, fn_0058b630(Ptr, u32) -> Ptr),
        entry!(0x0058_b470, fn_0058b470(Ptr) -> bool),
        entry!(0x0058_b4a0, fn_0058b4a0(Ptr) -> bool),
        entry!(0x0058_b4d0, fn_0058b4d0(Ptr) -> bool),
        entry!(0x0058_b4f0, fn_0058b4f0(Ptr) -> bool),
        entry!(0x0058_b510, fn_0058b510(Ptr, u32, u32) -> u32),
        entry!(0x0058_b680, fn_0058b680(Ptr, u32) -> u32),
        entry!(0x0058_b6c0, fn_0058b6c0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0058b450_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0046_4f40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_b450, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0046_4f40), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0058b420_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_b450, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_b420, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_b420)
                .collect();
            let mut expected = vec![(0x0058_b450, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod form_array_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn at_0058b3f0_builds_the_base_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(ARRAY_BASE_CONSTRUCT, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        let back = e.call(0x0058_b3f0, &args![this]).u32();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_FORM_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, ARRAY_BASE_CONSTRUCT),
            vec![vec![this.addr(), 0, 1]]
        );
    }

    #[test]
    fn at_0058b600_counts_the_array_or_returns_zero() {
        let mut e = Engine::new();
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        assert_eq!(e.call(0x0058_b600, &args![this]).u32(), 0);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 0xc, 7);
        e.mem.set_u32(this.addr() + FORM_ARRAY, array);
        assert_eq!(e.call(0x0058_b600, &args![this]).u32(), 7);
    }

    #[test]
    fn at_0058b630_casts_the_form_at_the_index() {
        let mut e = Engine::new();
        e.register(NI_FORM_ARRAY_GET_FORM_AT, |_, a| (0x7000 + a[1]).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_b630, &args![this, 3u32]).u32(), 0);
        assert!(calls_to(&e, NI_FORM_ARRAY_GET_FORM_AT).is_empty());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + FORM_ARRAY, array);
        assert_eq!(e.call(0x0058_b630, &args![this, 3u32]).u32(), 0x7004);
        assert_eq!(
            calls_to(&e, NI_FORM_ARRAY_GET_FORM_AT),
            vec![vec![array, 3]]
        );
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x7003, 0, TES_FORM_TYPE, BGS_CAMERA_PATH_TYPE, 0]]
        );
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    #[test]
    fn flag_readers_0058b470_to_0058b4f0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let read = |e: &mut Engine, addr: u32| e.call(addr, &[this]).bool();
        for (flags, high, neither, one, two) in [
            (0x00u8, false, true, false, false),
            (0x80, true, true, false, false),
            (0x01, false, false, true, false),
            (0x02, false, false, false, true),
            (0x03, false, false, true, true),
        ] {
            e.mem.set_u8(this + 0x28, flags);
            assert_eq!(read(&mut e, 0x0058_b470), high);
            assert_eq!(read(&mut e, 0x0058_b4a0), neither);
            assert_eq!(read(&mut e, 0x0058_b4d0), one);
            assert_eq!(read(&mut e, 0x0058_b4f0), two);
        }
    }

    /// Paths: +0x28 flags, +0x2c the array, +0x10 whether `00440d80`
    /// answers true, +0x14 whether the conditions fail, +0x18 whether
    /// `008256d0` answers true. Arrays: the count at +0, the forms from +4.
    fn setup(e: &mut Engine) {
        e.register(PATH_00440D80, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
        e.register(PATH_CONDITIONS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONDITION_IS_TRUE, |e, a| {
            assert_eq!(a[1..], [0x11, 0x22]);
            Ret {
                eax: u32::from(e.mem.u32(a[0] + 0x14) == 0),
                ..Ret::default()
            }
        });
        e.register(PATH_00891170, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CHECK_008256D0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
        e.register(ARRAY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NI_FORM_ARRAY_GET_FORM_AT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4 + 4 * a[1]),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    fn path(e: &mut Engine, children: &[u32]) -> u32 {
        let path = e.mem.alloc(0x30);
        if !children.is_empty() {
            let array = e.mem.alloc(4 + 4 * children.len() as u32);
            e.mem.set_u32(array, children.len() as u32);
            for (index, child) in children.iter().enumerate() {
                e.mem.set_u32(array + 4 + 4 * index as u32, *child);
            }
            e.mem.set_u32(path + 0x2c, array);
        }
        path
    }

    #[test]
    fn fn_0058b510_picks_the_path_or_a_child() {
        let mut e = Engine::new();
        setup(&mut e);
        let call = |e: &mut Engine, path: u32| e.call(0x0058_b510, &[path, 0x11, 0x22]).u32();
        let leaf = path(&mut e, &[]);
        assert_eq!(call(&mut e, leaf), leaf);
        e.mem.set_u32(leaf + 0x18, 1);
        assert_eq!(call(&mut e, leaf), 0);
        e.mem.set_u8(leaf + 0x28, 0x80);
        assert_eq!(call(&mut e, leaf), leaf);
        e.mem.set_u32(leaf + 0x14, 1);
        assert_eq!(call(&mut e, leaf), 0);
        e.mem.set_u32(leaf + 0x14, 0);
        e.mem.set_u32(leaf + 0x10, 1);
        assert_eq!(call(&mut e, leaf), 0);
        e.mem.set_u32(leaf + 0x10, 0);
        // A parent answers its first taking child, else itself.
        let refused = path(&mut e, &[]);
        e.mem.set_u32(refused + 0x14, 1);
        let parent = path(&mut e, &[0, refused, leaf]);
        assert_eq!(call(&mut e, parent), leaf);
        e.mem.set_u32(leaf + 0x14, 1);
        assert_eq!(call(&mut e, parent), parent);
        e.mem.set_u32(parent + 0x18, 1);
        assert_eq!(call(&mut e, parent), 0);
    }

    #[test]
    fn fn_0058b680_and_0058b6c0_use_the_array() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(GET_INDEX_FOR_FORM, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(SETTER_008D7DC0, |_, _| Ret::default());
        e.register(ARRAY_00486B80, |_, _| Ret::default());
        e.register(ARRAY_0061BDC0, |_, _| Ret::default());
        let empty = path(&mut e, &[]);
        assert_eq!(e.call(0x0058_b680, &[empty, 5]).u32(), 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0058_b6c0, &[empty, 1]);
        let targets = [SETTER_008D7DC0, ARRAY_00486B80, ARRAY_0061BDC0];
        assert!(calls_to(&mut e, &targets).is_empty());
        let parent = path(&mut e, &[0x0a00, 0x0b00, 0x0c00, 0]);
        let array = e.mem.u32(parent + 0x2c);
        assert_eq!(e.call(0x0058_b680, &[parent, 5]).u32(), 6);
        e.call_log = Some(vec![]);
        e.call(0x0058_b6c0, &[parent, 1]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (SETTER_008D7DC0, vec![0x0c00, 0x0a00]),
                (ARRAY_00486B80, vec![array, 1]),
                (ARRAY_0061BDC0, vec![array]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0058_b6c0, &[parent, 2]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (ARRAY_00486B80, vec![array, 2]),
                (ARRAY_0061BDC0, vec![array]),
            ]
        );
    }
}

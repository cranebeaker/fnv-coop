//! `fallout shared/run_005a6150` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::BSStringT;

/// `TESForm::TESForm` (`00483370`).
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `TESGlobal`'s vtable.
const TES_GLOBAL_VTABLE: u32 = 0x0103_6524;
/// The form type of a `TESGlobal` (`GLOB`).
const FORM_TYPE_GLOBAL: u8 = 6;
/// `TESGlobal::cType` of a new global: `'s'`, a short.
const GLOBAL_TYPE_SHORT: u8 = 0x73;

layout! {
    /// `TESGlobal` (Xbox PDB), 0x28 bytes; the same offsets on PC.
    pub struct TESGlobal: 0x28 {
        /// `cFormEditorID` (Xbox PDB): a [`BSStringT`] at +0x18.
        0x18 cFormEditorID: Inline<BSStringT>,
        /// `cType` (Xbox PDB): `'s'`, `'l'` or `'f'`.
        0x20 cType: u8,
        /// `fValue` (Xbox PDB).
        0x24 fValue: f32,
    }
}

// Translated from 005a6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESGlobal` (the engine map gives this address no name;
/// the body fits the Xbox PDB class: `CreateFormOfType` allocates 0x28 bytes
/// for form type 6 and calls it, `cType` is at +0x20 and `fValue` at +0x24). It
/// constructs the `TESForm` base, sets the
/// vtable, constructs the editor ID string, makes the global a short
/// (`cType` `'s'`) of value 0.0 and sets the form type to 6. Returns `this`.
// Translation incomplete: 005a6320 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_005a6320(e: &mut Engine, this: Ptr<TESGlobal>) -> Ptr<TESGlobal> {
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_GLOBAL_VTABLE);
    let editor_id = this.at(TESGlobal::cFormEditorID);
    e.call(STRING_CONSTRUCT, &args![editor_id]);
    e.set(this, TESGlobal::cType, GLOBAL_TYPE_SHORT);
    e.set(this, TESGlobal::fValue, 0.0);
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_GLOBAL as u32]);
    this
}

// Translated from 005a63c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a63f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a63c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_63f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The player singleton pointer the two condition functions read
/// (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `006815c0` (`ECX` = node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0084e3a0` (`ECX` = form): the form id.
const FORM_ID: u32 = 0x0084_e3a0;

// Translated from 005a6150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0x2 of the word at +0x70 (`word & 2`).
pub fn fn_005a6150(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x70) & 2
}

// Translated from 005a6260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x610 (the list `005a6170` walks, read from the player).
pub fn fn_005a6260(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x610)
}

// Translated from 005a6170 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition function (`cdecl`: reference, `form`, a second parameter,
/// the `double` result): false for a null `form`. Otherwise walks the list
/// `005a6260` gives for the player (`011dea3c`) while the node's item is
/// not null; for each item whose first word is `form`'s id, the result
/// becomes 0.0 and then, for `i` = 0..3, `i + 1` whenever the item's
/// second word (as an integer) is at least the word at +0x28 of
/// `00504e60(form)` times `(i + 1) * *(double *)010290b0` (x87, in the
/// exe's order). Returns true.
pub fn fn_005a6170(e: &mut Engine, _reference: Ptr, form: Ptr, _param2: u32, result: Ptr) -> bool {
    if form.is_null() {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    let mut node = e.call(0x005a_6260, &args![player]).u32();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(at);
        let id = e.call(FORM_ID, &args![form]).u32();
        if e.mem.u32(item) == id {
            let data = e.call(0x0050_4e60, &args![form]).u32();
            let threshold = e.mem.i32(data + 0x28);
            let at = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(at);
            let value = e.mem.i32(item + 4);
            e.mem.set_f64(result.addr(), 0.0);
            let step: f64 = e.global(0x0102_90b0);
            for i in 0..4i32 {
                let scaled = x87_mul(f64::from(i + 1), step);
                let limit = x87_mul(f64::from(threshold), scaled);
                // FCOMPP, then `TEST AH,1` (C0): below or unordered skips.
                if f64::from(value) >= limit {
                    e.mem.set_f64(result.addr(), f64::from(i + 1));
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    true
}

// Translated from 005a6280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition function (`cdecl`: reference, `form`, a second parameter,
/// the `double` result): the result is 0.0, or 1.0 when the list
/// `004f2770` gives for the player (`011dea3c`) holds a form with
/// `form`'s id (the item of a node is `*006815c0(node + 4)`, the next node
/// `00726070(node + 4) - 4`, or none for 0; the walk stops at a null
/// item). Returns true.
pub fn fn_005a6280(e: &mut Engine, _reference: Ptr, form: Ptr, _param2: u32, result: Ptr) -> bool {
    e.mem.set_f64(result.addr(), 0.0);
    let player: u32 = e.global(PLAYER);
    let mut node = e.call(0x004f_2770, &args![player]).u32();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node + 4]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let at = e.call(NODE_ITEM, &args![node + 4]).u32();
        let item = e.mem.u32(at);
        let item_id = e.call(FORM_ID, &args![item]).u32();
        let id = e.call(FORM_ID, &args![form]).u32();
        if item_id == id {
            e.mem.set_f64(result.addr(), 1.0);
            return true;
        }
        let next = e.call(NODE_NEXT, &args![node + 4]).u32();
        node = if next != 0 { next.wrapping_sub(4) } else { 0 };
    }
    true
}

// Translated from 005a63a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESGlobal`'s vtable slot 0x134 (`SetEditorID` in the Xbox PDB's
/// `TESForm` vtable): sets `cFormEditorID` (`004037f0` with `id`, 0).
/// Returns true.
pub fn fn_005a63a0(e: &mut Engine, this: Ptr<TESGlobal>, id: Ptr) -> bool {
    let text = this.at(TESGlobal::cFormEditorID);
    e.call(0x0040_37f0, &args![text, id, 0u32]);
    true
}

// Translated from 005a63f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a63f0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESGlobal`'s destructor body: its vtable, `00483710` on `this`, the
/// editor ID string (`004037d0`) and the `TESForm` base
/// (`TESForm::~TESForm`, `00483630`).
pub fn fn_005a63f0(e: &mut Engine, this: Ptr<TESGlobal>) {
    e.mem.set_u32(this.addr(), TES_GLOBAL_VTABLE);
    e.call(0x0048_3710, &args![this]);
    let text = this.at(TESGlobal::cFormEditorID);
    e.call(0x0040_37d0, &args![text]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005a64b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a64b0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the chunk size (its alignment, and the stack growing with each EDID chunk, are not modelled)
/// `TESGlobal`'s `Load` (vtable slot 0x20): false unless the record is
/// type 6. Otherwise `TESForm::LoadForm` (`00485110`), then every chunk:
/// `EDID` (a stack buffer of the chunk's size, at most 0x200 bytes, to
/// vtable slot 0x134), `OBND` (vtable slot 0xE0), `FNAM` (one byte into
/// `cType`) and `FLTV` (`TESFile::GetChunkData` into `fValue`). Returns
/// true.
pub fn fn_005a64b0(e: &mut Engine, this: Ptr<TESGlobal>, file: Ptr) -> bool {
    if e.call(0x0047_2660, &args![file]).u32() != 6 {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            0x4d41_4e46 => {
                e.call(0x0047_2890, &args![file, base + 0x20, 1u32]);
            }
            0x5654_4c46 => {
                e.call(0x0047_27f0, &args![file, base + 0x24]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl) with the source and the
/// `TESGlobal` target type descriptors `005a65e0` and `005a6640` pass.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const GLOBAL_TYPE: u32 = 0x0118_3ad4;
/// The global's type byte (`00529ea0`) and value (`00526ac0`, in `ST0`;
/// the engine map's name for it, `BSMultiBoundCapsule::QMultiBoundRadius`,
/// is folded code), both thiscall.
const GLOBAL_TYPE_OF: u32 = 0x0052_9ea0;
const GLOBAL_VALUE_OF: u32 = 0x0052_6ac0;

// Translated from 005a6460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESGlobal`'s record writer (no stack words; the engine map has no
/// name): `TESForm::StartForm`, the type byte at +0x20 as `FNAM`
/// (`004858f0`, cdecl), the value at +0x24 as `FLTV` (`00485910`, cdecl,
/// FLD/FSTP onto the stack), then `TESForm::CloseForm`.
pub fn fn_005a6460(e: &mut Engine, this: Ptr<TESGlobal>) {
    e.call(0x0048_55a0, &args![this]);
    let kind = e.get(this, TESGlobal::cType);
    e.call(0x0048_58f0, &args![0x4d41_4e46u32, u32::from(kind)]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0x24));
    e.call(0x0048_5910, &args![0x5654_4c46u32, value]);
    e.call(0x0048_5680, &args![this]);
}

// Translated from 005a65e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESGlobal`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01183ad4, 0)`; nothing without it. Otherwise
/// `TESForm::CopyAllComponents` given the uncast source, then the
/// source's type byte (`00529ea0`) and value (`00526ac0`, stored as a
/// `float`).
pub fn fn_005a65e0(e: &mut Engine, this: Ptr<TESGlobal>, source: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, GLOBAL_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, source]);
    let kind = e.call(GLOBAL_TYPE_OF, &args![other]).u8();
    e.set(this, TESGlobal::cType, kind);
    let value = e.call(GLOBAL_VALUE_OF, &args![other]).f64();
    e.mem.set_u32(this.addr() + 0x24, x87_store32(value));
}

// Translated from 005a6640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESGlobal`'s compare (`RET 4`: the other form; true when they differ;
/// the engine map has no name): true when the cast as in `005a65e0`
/// fails, when `TESForm::CompareAllComponents` answers true, when the type
/// bytes (`00529ea0`) differ or when the values (`00526ac0`, this one kept
/// as a `double`) are not equal (unordered counts as different).
pub fn fn_005a6640(e: &mut Engine, this: Ptr<TESGlobal>, form: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, GLOBAL_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let mine = e.call(GLOBAL_TYPE_OF, &args![this]).u8();
    let theirs = e.call(GLOBAL_TYPE_OF, &args![other]).u8();
    if mine != theirs {
        return true;
    }
    let mine = e.call(GLOBAL_VALUE_OF, &args![this]).f64();
    let theirs = e.call(GLOBAL_VALUE_OF, &args![other]).f64();
    // FUCOMPP; TEST AH,0x44; JNP: equal only when ordered.
    mine != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_6460, fn_005a6460(Ptr<TESGlobal>)),
        entry!(0x005a_65e0, fn_005a65e0(Ptr<TESGlobal>, u32)),
        entry!(0x005a_6640, fn_005a6640(Ptr<TESGlobal>, u32) -> bool),
        entry!(0x005a6320, fn_005a6320(Ptr<TESGlobal>) -> Ptr<TESGlobal>),
        entry!(0x005a_63c0, fn_005a63c0(Ptr, u32) -> Ptr),
        entry!(0x005a_6150, fn_005a6150(Ptr) -> u32),
        entry!(0x005a_6260, fn_005a6260(Ptr) -> u32),
        entry!(0x005a_6170, fn_005a6170(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a_6280, fn_005a6280(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a_63a0, fn_005a63a0(Ptr<TESGlobal>, Ptr) -> bool),
        entry!(0x005a_63f0, fn_005a63f0(Ptr<TESGlobal>)),
        entry!(0x005a_64b0, fn_005a64b0(Ptr<TESGlobal>, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TESForm;

    #[test]
    fn constructs_a_zero_valued_short_global_in_the_games_order() {
        let mut e = Engine::new();
        e.register(TES_FORM_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FORM_TYPE, |_, _| Ret::default());
        let global: Ptr<TESGlobal> = e.new_object();
        // Dirty memory the constructor must overwrite.
        e.set(global, TESGlobal::cType, 0x66);
        e.set(global, TESGlobal::fValue, 9.0);
        e.call_log = Some(vec![]);
        let back = e.call(0x005a_6320, &args![global]).ptr::<TESGlobal>();
        assert_eq!(back, global);
        assert_eq!(e.mem.u32(global.addr()), TES_GLOBAL_VTABLE);
        assert_eq!(e.get(global, TESGlobal::cType), 0x73);
        assert_eq!(e.get(global, TESGlobal::fValue), 0.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (TES_FORM_CONSTRUCT, vec![global.addr()]),
                (STRING_CONSTRUCT, vec![global.addr() + 0x18]),
                (SET_FORM_TYPE, vec![global.addr(), 6]),
            ]
        );
    }

    #[test]
    fn the_form_type_is_written_by_the_game_function() {
        // With the real 004f15a0 in place the type byte lands at +4.
        let mut e = Engine::new();
        e.register(TES_FORM_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let global: Ptr<TESGlobal> = e.new_object();
        e.call(0x005a_6320, &args![global]);
        assert_eq!(e.get(global.cast::<TESForm>(), TESForm::cFormType), 6);
    }
    #[test]
    fn at_005a63c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_63f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_63c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_63c0)
                .collect();
            let mut expected = vec![(0x005a_63f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    #[test]
    fn b020_005a6150_and_005a6260_read_words() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x620);
        e.mem.set_u32(p + 0x70, 0xffff_fff7);
        e.mem.set_u32(p + 0x610, 0x1234);
        assert_eq!(e.call(0x005a_6150, &args![p]).u32(), 2);
        e.mem.set_u32(p + 0x70, 0xffff_fffd);
        assert_eq!(e.call(0x005a_6150, &args![p]).u32(), 0);
        assert_eq!(e.call(0x005a_6260, &args![p]).u32(), 0x1234);
    }

    /// Nodes are (item, next) pairs; the player at `011dea3c` is `player`.
    fn list_engine(player: u32) -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.map(PLAYER, 4);
        e.set_global::<u32>(PLAYER, player);
        e
    }

    #[test]
    fn b020_005a6170_counts_the_steps_reached() {
        let mut e = list_engine(0x0333_0000);
        e.map(0x0102_9000, 0x1000);
        e.set_global::<f64>(0x0102_90b0, 0.25);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0xc, 0x77);
        let data = e.mem.alloc(0x40);
        e.mem.set_i32(data + 0x28, 100);
        e.register_double(0x0050_4e60, move |_, _| ret(data));
        // Items: {0x77, 60}, {0x88, 1000}, then a null item.
        let a = e.mem.alloc(8);
        e.mem.set_u32(a, 0x77);
        e.mem.set_i32(a + 4, 60);
        let b = e.mem.alloc(8);
        e.mem.set_u32(b, 0x88);
        e.mem.set_i32(b + 4, 1000);
        let n1 = e.mem.alloc(8);
        let n2 = e.mem.alloc(8);
        let n3 = e.mem.alloc(8);
        e.mem.set_u32(n1, a);
        e.mem.set_u32(n1 + 4, n2);
        e.mem.set_u32(n2, b);
        e.mem.set_u32(n2 + 4, n3);
        e.register_double(0x005a_6260, move |_, args| {
            assert_eq!(args, &[0x0333_0000]);
            ret(n1)
        });
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 9.0);
        assert!(e.call(0x005a_6170, &args![0u32, form, 0u32, result]).bool());
        // 60 >= 25 and >= 50, not >= 75.
        assert_eq!(e.mem.f64(result), 2.0);
        e.mem.set_i32(a + 4, 10);
        assert!(e.call(0x005a_6170, &args![0u32, form, 0u32, result]).bool());
        assert_eq!(e.mem.f64(result), 0.0);
        e.mem.set_i32(a + 4, 100);
        e.call(0x005a_6170, &args![0u32, form, 0u32, result]);
        assert_eq!(e.mem.f64(result), 4.0);
        e.mem.set_f64(result, 9.0);
        assert!(!e.call(0x005a_6170, &args![0u32, 0u32, 0u32, result]).bool());
        assert_eq!(e.mem.f64(result), 9.0);
    }

    #[test]
    fn b020_005a6280_finds_the_form() {
        let mut e = list_engine(0x0333_0000);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0xc, 0x77);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0xc, 0x66);
        // Nodes start 4 bytes before the (item, next) pair the callees see.
        let n1 = e.mem.alloc(0xc);
        let n2 = e.mem.alloc(0xc);
        e.mem.set_u32(n1 + 4, other);
        e.mem.set_u32(n1 + 8, n2 + 4);
        e.mem.set_u32(n2 + 4, form);
        e.register_double(0x004f_2770, move |_, _| ret(n1));
        let result = e.mem.alloc(8);
        assert!(e.call(0x005a_6280, &args![0u32, form, 0u32, result]).bool());
        assert_eq!(e.mem.f64(result), 1.0);
        e.mem.set_u32(n2 + 4, other);
        assert!(e.call(0x005a_6280, &args![0u32, form, 0u32, result]).bool());
        assert_eq!(e.mem.f64(result), 0.0);
    }

    #[test]
    fn b020_005a63a0_005a63f0_editor_id_and_destructor() {
        let mut e = Engine::new();
        for a in [0x0040_37f0, 0x0048_3710, 0x0040_37d0, 0x0048_3630] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.new_object::<TESGlobal>();
        let b = p.addr();
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_63a0, &args![p, 0x7000u32]).bool());
        assert_eq!(
            calls(&mut e, 0x005a_63a0),
            vec![(0x0040_37f0, vec![b + 0x18, 0x7000, 0])]
        );
        e.call_log = Some(vec![]);
        e.call(0x005a_63f0, &args![p]);
        assert_eq!(e.mem.u32(b), TES_GLOBAL_VTABLE);
        assert_eq!(
            calls(&mut e, 0x005a_63f0),
            vec![
                (0x0048_3710, vec![b]),
                (0x0040_37d0, vec![b + 0x18]),
                (0x0048_3630, vec![b]),
            ]
        );
    }

    #[test]
    fn b020_005a64b0_loads_the_global() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = Engine::new();
        e.register(0x0047_2660, |_, _| ret(6));
        let queue = Rc::new(RefCell::new(vec![
            0x4449_4445u32,
            0x444e_424f,
            0x4d41_4e46,
            0x5654_4c46,
            0x0101_0101,
        ]));
        let q = queue.clone();
        e.register_double(0x0047_26b0, move |_, _| ret(q.borrow()[0]));
        e.register_double(0x0047_26f0, move |_, _| {
            let mut v = queue.borrow_mut();
            v.remove(0);
            ret(u32::from(!v.is_empty()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        for a in [
            0x0048_5110,
            0x0047_2890,
            0x0047_27f0,
            0x0000_1134,
            0x0000_10e0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0134, 0x0000_1134);
        e.mem.set_u32(0x0150_00e0, 0x0000_10e0);
        let p = e.new_object::<TESGlobal>();
        let b = p.addr();
        e.mem.set_u32(b, 0x0150_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_64b0, &args![p, 0x9000u32]).bool());
        let log: Vec<_> = calls(&mut e, 0x005a_64b0)
            .into_iter()
            .filter(|c| c.0 != 0x0047_26b0 && c.0 != 0x0047_26f0)
            .collect();
        let buffer = log[3].1[1];
        assert_eq!(
            log,
            vec![
                (0x0047_2660, vec![0x9000]),
                (0x0048_5110, vec![b, 0x9000]),
                (0x0040_1660, vec![0x9000]),
                (0x0047_2890, vec![0x9000, buffer, 0x200]),
                (0x0000_1134, vec![b, buffer]),
                (0x0000_10e0, vec![b, 0x9000]),
                (0x0047_2890, vec![0x9000, b + 0x20, 1]),
                (0x0047_27f0, vec![0x9000, b + 0x24]),
            ]
        );
        e.register(0x0047_2660, |_, _| ret(7));
        assert!(!e.call(0x005a_64b0, &args![p, 0x9000u32]).bool());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Getter doubles reading the type byte (+0x20) and value (+0x24).
    fn getters(e: &mut Engine) {
        e.register(GLOBAL_TYPE_OF, |e, a| ret(u32::from(e.mem.u8(a[0] + 0x20))));
        e.register(GLOBAL_VALUE_OF, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x24)),
            ..Ret::default()
        });
    }

    #[test]
    fn fn_005a6460_writes_type_and_value() {
        let mut e = Engine::new();
        for a in [0x0048_55a0u32, 0x0048_58f0, 0x0048_5910, 0x0048_5680] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.new_object::<TESGlobal>();
        e.set(this, TESGlobal::cType, 0x66);
        e.mem.set_u32(this.addr() + 0x24, 0x7f80_0005);
        e.call_log = Some(vec![]);
        e.call(0x005a_6460, &args![this]);
        assert_eq!(calls(&e, 0x0048_55a0), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, 0x0048_58f0), vec![vec![0x4d41_4e46, 0x66]]);
        assert_eq!(calls(&e, 0x0048_5910), vec![vec![0x5654_4c46, 0x7fc0_0005]]);
        assert_eq!(calls(&e, 0x0048_5680), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_005a65e0_copies_type_and_value() {
        let mut e = Engine::new();
        getters(&mut e);
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 3 { 0 } else { a[0] - 2 })
        });
        e.register(0x0048_51b0, |_, _| Ret::default());
        let this = e.new_object::<TESGlobal>();
        let source = e.new_object::<TESGlobal>().addr();
        e.mem.set_u8(source + 0x20, 0x6c);
        e.mem.set_f32(source + 0x24, -3.5);
        e.call_log = Some(vec![]);
        e.call(0x005a_65e0, &args![this, source + 2]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![source + 2, 0, CAST_SOURCE, GLOBAL_TYPE, 0]]
        );
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this.addr(), source + 2]]);
        assert_eq!(e.get(this, TESGlobal::cType), 0x6c);
        assert_eq!(e.get(this, TESGlobal::fValue), -3.5);
        // No cast: nothing.
        e.call_log = Some(vec![]);
        e.call(0x005a_65e0, &args![this, 3u32]);
        assert!(calls(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn fn_005a6640_compares_type_and_value() {
        let mut e = Engine::new();
        getters(&mut e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0048_5270, |_, _| ret(0));
        let this = e.new_object::<TESGlobal>();
        let other = e.new_object::<TESGlobal>();
        assert!(!e.call(0x005a_6640, &args![this, other]).bool());
        assert!(e.call(0x005a_6640, &args![this, 0u32]).bool());
        e.set(other, TESGlobal::cType, 1);
        assert!(e.call(0x005a_6640, &args![this, other]).bool());
        e.set(other, TESGlobal::cType, 0);
        e.set(other, TESGlobal::fValue, 1.0);
        assert!(e.call(0x005a_6640, &args![this, other]).bool());
        e.set(other, TESGlobal::fValue, f32::NAN);
        e.set(this, TESGlobal::fValue, f32::NAN);
        assert!(e.call(0x005a_6640, &args![this, other]).bool());
        e.set(other, TESGlobal::fValue, 0.0);
        e.set(this, TESGlobal::fValue, 0.0);
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005a_6640, &args![this, other]).bool());
    }
}

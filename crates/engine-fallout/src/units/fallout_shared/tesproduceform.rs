//! `fallout shared/tesproduceform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESProduceForm` (Xbox PDB, 0xc bytes; the PC code uses the same
    /// offsets).
    pub struct TESProduceForm: 0x0c {
        /// The vtable (`BaseFormComponent`).
        0x00 vtable: u32,
        /// `pFormIngredient` (Xbox PDB): an `IngredientItem*`; before
        /// `InitItem` the ingredient's form id.
        0x04 pFormIngredient: u32,
    }
}

/// `iProduceChance` (Xbox PDB): four bytes at +0x08.
const PRODUCE_CHANCE: u32 = 0x08;
/// Fills bytes (`00403d30(dest, value, count)`, cdecl).
const FILL_BYTES: u32 = 0x0040_3d30;
/// `memcpy(dest, source, count)` (wrapper `00401460`, cdecl) and `_memcmp`
/// (`00ec4835`, cdecl).
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The form-id getter (`0084e3a0`, thiscall: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm` chunk writers: a word (`00485910`, cdecl(chunk, word)) and an
/// array (`TESForm::AddChunkArray`, `004856f0`, cdecl(chunk, data, size)).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// The chunk tags `PFIG` and `PFPC`.
const CHUNK_PFIG: u32 = u32::from_le_bytes(*b"PFIG");
const CHUNK_PFPC: u32 = u32::from_le_bytes(*b"PFPC");
/// The word at `+4` (`00726070`, thiscall) and its setter (`006ecd40`,
/// thiscall, `RET 4`).
const WORD_AT_4: u32 = 0x0072_6070;
const SET_WORD_AT_4: u32 = 0x006e_cd40;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`), `BaseFormComponent` (`01183040`),
/// `IngredientItem` (`011839dc`) and `TESProduceForm` (`01186bfc`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
const TYPE_TES_PRODUCE_FORM: u32 = 0x0118_6bfc;
/// `TESForm::GetFile` (`00484e60`, thiscall, `RET 4`),
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl(id pointer, file)) and the
/// form lookup by id (`004839c0`, cdecl).
const GET_FILE: u32 = 0x0048_4e60;
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00474cb0` (thiscall on the owner form): picks which message names the
/// owner.
const OWNER_HAS_NAME: u32 = 0x0047_4cb0;
/// The log call (`005b5e40`, cdecl with a format).
const LOG: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find ingredient (%08X) on owner object ..." with
/// the owner's name, with its form id, and on an unknown owner.
const MISSING_WITH_NAME: u32 = 0x0101_d340;
const MISSING_WITH_ID: u32 = 0x0101_d2f8;
const MISSING_UNKNOWN_OWNER: u32 = 0x0101_d2b4;

/// `other` as a `TESProduceForm` (`__RTDynamicCast` from
/// `BaseFormComponent`), or 0.
fn as_produce_form(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_PRODUCE_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 0048b590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESProduceForm::InitializeDataComponent` (engine map): no ingredient,
/// the four produce chances cleared (`00403d30`).
pub fn tes_produce_form_initialize_data_component(e: &mut Engine, this: Ptr<TESProduceForm>) {
    e.set(this, TESProduceForm::pFormIngredient, 0);
    e.call(FILL_BYTES, &args![this.addr() + PRODUCE_CHANCE, 0u32, 4u32]);
}

// Translated from 0048b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The produce form's save (engine map: no name; no stack words): with an
/// ingredient, its form id as chunk `PFIG`; then always the four produce
/// chances as chunk `PFPC` (`TESForm::AddChunkArray`).
pub fn fn_0048b5c0(e: &mut Engine, this: Ptr<TESProduceForm>) {
    let ingredient = e.get(this, TESProduceForm::pFormIngredient);
    if ingredient != 0 {
        let id = e.call(FORM_ID, &args![ingredient]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_PFIG, id]);
    }
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_PFPC, this.addr() + PRODUCE_CHANCE, 4u32],
    );
}

// Translated from 0048b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESProduceForm::CopyComponent` (engine map, `RET 4`): when `other` is a
/// `TESProduceForm`, takes its ingredient (`00726070`, `006ecd40`) and its
/// produce chances (`fn_0048b670`).
pub fn tes_produce_form_copy_component(e: &mut Engine, this: Ptr<TESProduceForm>, other: u32) {
    let source = as_produce_form(e, other);
    if source == 0 {
        return;
    }
    let ingredient = e.call(WORD_AT_4, &args![source]).u32();
    e.call(SET_WORD_AT_4, &args![this, ingredient]);
    fn_0048b670(e, this, source + PRODUCE_CHANCE);
}

// Translated from 0048b670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies four produce chances from `source` (`memcpy`), when it is set
/// (engine map: no name; `RET 4`).
pub fn fn_0048b670(e: &mut Engine, this: Ptr<TESProduceForm>, source: u32) {
    if source != 0 {
        e.call(MEMCPY, &args![this.addr() + PRODUCE_CHANCE, source, 4u32]);
    }
}

// Translated from 0048b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESProduceForm::CompareComponent` (engine map, `RET 4`): true when the
/// two differ: when `other` is no `TESProduceForm`, when the ingredients
/// (`00726070`, this one's first) differ, or when the produce chances differ
/// (`_memcmp` of four bytes).
pub fn tes_produce_form_compare_component(
    e: &mut Engine,
    this: Ptr<TESProduceForm>,
    other: u32,
) -> bool {
    let source = as_produce_form(e, other);
    if source == 0 {
        return true;
    }
    let mine = e.call(WORD_AT_4, &args![this]).u32();
    let theirs = e.call(WORD_AT_4, &args![source]).u32();
    if mine != theirs {
        return true;
    }
    e.call(
        MEMCMP,
        &args![this.addr() + PRODUCE_CHANCE, source + PRODUCE_CHANCE, 4u32],
    )
    .u32()
        != 0
}

// Translated from 0048b720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESProduceForm::InitItem` (engine map, `RET 4`): with an ingredient id
/// set, adds the owner's file's compile index to it
/// (`TESForm::AddCompileIndex(&id, owner.GetFile(-1))`, asked of the owner
/// even when it is null, as the exe does) and resolves it to the
/// `IngredientItem` (`004839c0`, `__RTDynamicCast` from `TESForm`). When
/// that fails, logs (`005b5e40`) the id with the owner's name (owner vtable
/// slot `+0x130`) when `00474cb0` answers non-zero, else with the owner's
/// form id, or the unknown-owner message without an owner. The id the game
/// keeps on its stack is a block here.
pub fn tes_produce_form_init_item(e: &mut Engine, this: Ptr<TESProduceForm>, owner: u32) {
    if e.get(this, TESProduceForm::pFormIngredient) == 0 {
        return;
    }
    e.with_stack(4, |e, id| {
        let ingredient = e.get(this, TESProduceForm::pFormIngredient);
        e.mem.set_u32(id.addr(), ingredient);
        let file = e.call(GET_FILE, &args![owner, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id, file]);
        let resolved = e.mem.u32(id.addr());
        let form = e.call(LOOKUP_FORM, &args![resolved]).u32();
        let item = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_INGREDIENT_ITEM, 0u32],
            )
            .u32();
        e.set(this, TESProduceForm::pFormIngredient, item);
        if e.get(this, TESProduceForm::pFormIngredient) != 0 {
            return;
        }
        let missing = e.mem.u32(id.addr());
        if owner != 0 {
            if e.call(OWNER_HAS_NAME, &args![owner]).u32() != 0 {
                let name = e.vcall(owner, 0x130, &[]).u32();
                e.call(LOG, &args![MISSING_WITH_NAME, missing, name]);
            } else {
                let owner_id = e.call(FORM_ID, &args![owner]).u32();
                e.call(LOG, &args![MISSING_WITH_ID, missing, owner_id]);
            }
        } else {
            e.call(LOG, &args![MISSING_UNKNOWN_OWNER, missing]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0048_b590,
            tes_produce_form_initialize_data_component(Ptr<TESProduceForm>)
        ),
        entry!(0x0048_b5c0, fn_0048b5c0(Ptr<TESProduceForm>)),
        entry!(
            0x0048_b610,
            tes_produce_form_copy_component(Ptr<TESProduceForm>, u32)
        ),
        entry!(0x0048_b670, fn_0048b670(Ptr<TESProduceForm>, u32)),
        entry!(
            0x0048_b6a0,
            tes_produce_form_compare_component(Ptr<TESProduceForm>, u32) -> bool
        ),
        entry!(
            0x0048_b720,
            tes_produce_form_init_item(Ptr<TESProduceForm>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FILL_BYTES, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SET_WORD_AT_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(LOG, |_, _| Ret::default());
        e
    }

    fn produce(e: &mut Engine, ingredient: u32, chances: [u8; 4]) -> Ptr<TESProduceForm> {
        let this = e.new_object::<TESProduceForm>();
        e.set(this, TESProduceForm::pFormIngredient, ingredient);
        for (i, c) in chances.iter().enumerate() {
            e.mem.set_u8(this.addr() + 8 + i as u32, *c);
        }
        this
    }

    #[test]
    fn initialize_clears_the_ingredient_and_chances() {
        let mut e = engine();
        let this = produce(&mut e, 0x55, [1, 2, 3, 4]);
        e.call(0x0048_b590, &args![this]);
        assert_eq!(e.get(this, TESProduceForm::pFormIngredient), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
    }

    #[test]
    fn save_writes_the_ingredient_id_when_set_and_always_the_chances() {
        let mut e = engine();
        let ingredient = e.mem.alloc(0x18);
        e.mem.set_u32(ingredient + 0xc, 0x0012_3456);
        let this = produce(&mut e, ingredient, [1, 2, 3, 4]);
        let (_, log) = run(&mut e, 0x0048_b5c0, &args![this]);
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![CHUNK_PFIG, 0x0012_3456]]
        );
        assert_eq!(
            only(&log, ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_PFPC, this.addr() + 8, 4]]
        );
        let bare = produce(&mut e, 0, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b5c0, &args![bare]);
        assert!(only(&log, ADD_CHUNK_WORD).is_empty());
        assert_eq!(only(&log, ADD_CHUNK_ARRAY).len(), 1);
    }

    #[test]
    fn copy_takes_the_ingredient_and_chances_of_a_produce_form() {
        let mut e = engine();
        let this = produce(&mut e, 0, [0; 4]);
        let other = produce(&mut e, 0x77, [9, 8, 7, 6]);
        e.call(0x0048_b610, &args![this, other]);
        assert_eq!(e.get(this, TESProduceForm::pFormIngredient), 0x77);
        assert_eq!(e.mem.u32(this.addr() + 8), u32::from_le_bytes([9, 8, 7, 6]));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let fresh = produce(&mut e, 0, [0; 4]);
        e.call(0x0048_b610, &args![fresh, other]);
        assert_eq!(e.get(fresh, TESProduceForm::pFormIngredient), 0);
    }

    #[test]
    fn copy_chances_needs_a_source() {
        let mut e = engine();
        let this = produce(&mut e, 0, [5, 5, 5, 5]);
        let (_, log) = run(&mut e, 0x0048_b670, &args![this, 0u32]);
        assert!(only(&log, MEMCPY).is_empty());
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x0102_0304);
        e.call(0x0048_b670, &args![this, source]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x0102_0304);
    }

    #[test]
    fn compare_reports_ingredient_and_chance_differences() {
        let mut e = engine();
        let this = produce(&mut e, 0x77, [1, 2, 3, 4]);
        let same = produce(&mut e, 0x77, [1, 2, 3, 4]);
        let other_ingredient = produce(&mut e, 0x78, [1, 2, 3, 4]);
        let other_chance = produce(&mut e, 0x77, [1, 2, 3, 5]);
        let differs = |e: &mut Engine, other: Ptr<TESProduceForm>| {
            e.call(0x0048_b6a0, &args![this, other]).bool()
        };
        assert!(!differs(&mut e, same));
        assert!(differs(&mut e, other_ingredient));
        assert!(differs(&mut e, other_chance));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(differs(&mut e, same));
    }

    /// An owner form whose name (vtable slot `+0x130`) is `0x6000` and form
    /// id `0x0abc`.
    fn owner(e: &mut Engine) -> u32 {
        let owner = e.mem.alloc(0x18);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(owner + 0xc, 0x0abc);
        e.mem.set_u32(vtable + 0x130, 0x0f43_0130);
        e.register(0x0f43_0130, |_, _| ret(0x6000));
        owner
    }

    fn resolving(e: &mut Engine, found: u32, named: u32) {
        e.register(GET_FILE, |_, a| ret(0xf000 + a[1] % 0x10));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register_double(LOOKUP_FORM, move |_, a| {
            ret(if a[0] == 0x0100_0042 { found } else { 0 })
        });
        e.register_double(OWNER_HAS_NAME, move |_, _| ret(named));
    }

    #[test]
    fn init_item_resolves_the_ingredient_with_the_owners_index() {
        let mut e = engine();
        resolving(&mut e, 0x9000, 0);
        let owner = owner(&mut e);
        let this = produce(&mut e, 0x42, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b720, &args![this, owner]);
        assert_eq!(e.get(this, TESProduceForm::pFormIngredient), 0x9000);
        assert_eq!(only(&log, GET_FILE), vec![vec![owner, 0xffff_ffff]]);
        assert_eq!(only(&log, ADD_COMPILE_INDEX)[0][1], 0xf00f);
        assert_eq!(
            only(&log, RT_DYNAMIC_CAST),
            vec![vec![0x9000, 0, TYPE_TES_FORM, TYPE_INGREDIENT_ITEM, 0]]
        );
        assert!(only(&log, LOG).is_empty());
        let empty = produce(&mut e, 0, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b720, &args![empty, owner]);
        assert!(only(&log, GET_FILE).is_empty());
    }

    #[test]
    fn init_item_logs_a_missing_ingredient_by_owner() {
        let mut e = engine();
        resolving(&mut e, 0, 1);
        let owner = owner(&mut e);
        let this = produce(&mut e, 0x42, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b720, &args![this, owner]);
        assert_eq!(e.get(this, TESProduceForm::pFormIngredient), 0);
        assert_eq!(
            only(&log, LOG),
            vec![vec![MISSING_WITH_NAME, 0x0100_0042, 0x6000]]
        );
        resolving(&mut e, 0, 0);
        let this = produce(&mut e, 0x42, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b720, &args![this, owner]);
        assert_eq!(
            only(&log, LOG),
            vec![vec![MISSING_WITH_ID, 0x0100_0042, 0x0abc]]
        );
        let this = produce(&mut e, 0x42, [0; 4]);
        let (_, log) = run(&mut e, 0x0048_b720, &args![this, 0u32]);
        assert_eq!(only(&log, GET_FILE), vec![vec![0, 0xffff_ffff]]);
        assert_eq!(
            only(&log, LOG),
            vec![vec![MISSING_UNKNOWN_OWNER, 0x0100_0042]]
        );
    }
}

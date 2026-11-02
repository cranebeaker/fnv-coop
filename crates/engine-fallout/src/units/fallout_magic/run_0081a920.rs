//! `fallout/magic/run_0081a920` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MagicFogProjectile` code (the `GetSaveSize`-shaped `0081ad00` sits in its vtable
//! `0107ad84` at +0x50). PC offsets used here, each the Xbox PDB's minus 0x10: `eState`
//! +0xB4, `pTargetList` +0xC8 (a list of 0xC-byte nodes: two words and the next node).

#[allow(unused_imports)]
use crate::prelude::*;

/// `eState` (Xbox PDB `MagicFogProjectile` +0xC4, PC +0xB4).
const STATE: u32 = 0xb4;
/// `pTargetList` (Xbox PDB `MagicFogProjectile` +0xD8, PC +0xC8).
const TARGET_LIST: u32 = 0xc8;

/// `00440d80` (`thiscall`): bit 0x20 of the form flags (+8).
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// `0044ddc0` (`thiscall`): the word at +8.
const WORD_08: u32 = 0x0044_ddc0;
/// `00810fd0` (`thiscall`): the word at +0x98 of the projectile.
const PROJECTILE_WORD_98: u32 = 0x0081_0fd0;
/// `0084e3a0` (`thiscall`): the word at +0xC.
const WORD_0C: u32 = 0x0084_e3a0;
/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The second allocator `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE_AA13E0: u32 = 0x00aa_13e0;
/// `00438170` (`thiscall`, one word: a C string): constructs a string at `this` from it,
/// and `004381b0` (`thiscall`) destroys it.
const STRING_CONSTRUCT: u32 = 0x0043_8170;
const STRING_DESTROY: u32 = 0x0043_81b0;
/// `"MagicAreaDisplay"` (`0107a898`).
const MAGIC_AREA_DISPLAY: u32 = 0x0107_a898;
/// `00586150` (`thiscall`): the word at +0x9C of the projectile (`pDominantEffect`, Xbox
/// PDB +0xAC).
const DOMINANT_EFFECT: u32 = 0x0058_6150;
/// `EffectItem::GetArea` (engine map, `00404420`; `thiscall`): an integer.
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
/// `00403e20` (`thiscall`): the address of a setting's value (+4).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting at `011d15e0` whose float value scales the area.
const AREA_SETTING: u32 = 0x011d_15e0;
/// `009cdae0` (`thiscall`): the float at +0x64 in `ST0`.
const FLOAT_64: u32 = 0x009c_dae0;
/// `BSTestObjects::CreateSphere` (engine map, `00c56680`; `cdecl`, four words).
const CREATE_SPHERE: u32 = 0x00c5_6680;
/// `00a5b950` (`thiscall`, one word: a string): names the object.
const SET_NAME: u32 = 0x00a5_b950;
/// `00440460` (`thiscall`, one word: a point's address).
const SET_POINT: u32 = 0x0044_0460;
/// Three words at `011f426c` (a point).
const POINT_011F426C: u32 = 0x011f_426c;
/// `00416870` (`thiscall`, three floats): constructs a point.
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `00931ed0` (`thiscall`, one word: an out word): writes a word to it and returns its
/// address.
const MOBILE_OBJECT_00931ED0: u32 = 0x0093_1ed0;
/// `00c84470` (`cdecl`, two words: a word and a point's address).
const COLLISION_00C84470: u32 = 0x00c8_4470;
/// `NiMaterialProperty::NiMaterialProperty` (engine map, `00a75650`; `thiscall`).
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
/// `004534f0` (`thiscall`, one word), run twice on the property with `011a9b7c`.
const PROPERTY_004534F0: u32 = 0x0045_34f0;
const PROPERTY_ARGUMENT_011A9B7C: u32 = 0x011a_9b7c;
/// `004bc450` (`thiscall`, one word: a point's address).
const PROPERTY_004BC450: u32 = 0x004b_c450;
/// `NiAVObject::AttachProperty` (engine map, `00439410`; `thiscall`, one word).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `NiAVObject::UpdateProperties` (engine map, `00a5a040`; `thiscall`).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `0081e830` (`magicprojectile.cpp` in the engine map; `thiscall`, one word): the base
/// save size.
const PROJECTILE_SAVE_SIZE: u32 = 0x0081_e830;
/// The save game object at `011de45c` and `008df040` (`thiscall`): its version byte
/// (+0x80).
const SAVE_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;

/// The projectile's vtable slot +0x1D0 (no stack words): its scene object.
const SLOT_1D0: u32 = 0x1d0;
/// The scene object's slots: +0x9C (one word: a name; finds a child object), +0xC (no
/// stack words; a node or null), +0xDC (two words: child, 1; attaches the child).
const SLOT_9C: u32 = 0x9c;
const SLOT_0C: u32 = 0xc;
const SLOT_DC: u32 = 0xdc;
/// The target's vtable slot +4 (no stack words).
const SLOT_04: u32 = 0x4;

// Translated from 0081a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): true when `eState` (+0xB4) is not 1 and bit
/// 0x20 of the form flags (`00440d80`) is clear.
pub fn fn_0081a920(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + STATE) != 1 && !e.call(FORM_FLAG_20, &args![this]).bool()
}

// Translated from 0081a960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081a960 - the FS:[0] exception frame (freeing the node when its constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`): false when `first` or `second` is
/// null; true when the word at +8 of `second` (`0044ddc0`) differs from the projectile's
/// word at +0x98 (`00810fd0`). Otherwise it takes `first`'s slot +4 and `second`'s word at
/// +0xC (`0084e3a0`) as a pair: false when a node of the target list (+0xC8) already holds
/// that pair; else a new 0xC-byte node (`00401000`, `0081aa80`) with the pair and the old
/// list head becomes the head (0 when the allocation fails), and true.
pub fn fn_0081a960(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let base = this.addr();
    if first == 0 || second == 0 {
        return false;
    }
    let owner = e.call(WORD_08, &args![second]).u32();
    if owner != e.call(PROJECTILE_WORD_98, &args![this]).u32() {
        return true;
    }
    let key = e.vcall(first, SLOT_04, &args![]).u32();
    let value = e.call(WORD_0C, &args![second]).u32();
    let mut node = e.mem.u32(base + TARGET_LIST);
    while node != 0 {
        if e.mem.u32(node) == key && e.mem.u32(node + 4) == value {
            return false;
        }
        node = e.mem.u32(node + 8);
    }
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let head = if block == 0 {
        0
    } else {
        let next = e.mem.u32(base + TARGET_LIST);
        fn_0081aa80(e, Ptr::new(block), key, value, next).addr()
    };
    e.mem.set_u32(base + TARGET_LIST, head);
    true
}

// Translated from 0081aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): a target list node: the three words
/// at +0, +4 and +8. Returns `this`.
pub fn fn_0081aa80(e: &mut Engine, this: Ptr, key: u32, value: u32, next: u32) -> Ptr {
    e.mem.set_u32(this.addr(), key);
    e.mem.set_u32(this.addr() + 4, value);
    e.mem.set_u32(this.addr() + 8, next);
    this
}

// Translated from 0081aab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081aab0 - the FS:[0] exception frame (destroying the name strings and freeing the property when a callee throws) is not translated
/// Unnamed in the engine map (`thiscall`, plain `RET`): adds a debug sphere for the area.
/// Nothing happens when the projectile's scene object (slot +0x1D0) is null, already has a
/// child named `"MagicAreaDisplay"` (its slot +0x9C with a string made by `00438170` and
/// destroyed by `004381b0`) or is no node (slot +0xC). Otherwise the radius is
/// `value of the setting at 011d15e0 * area of the dominant effect` (`00586150`,
/// `00404420`; x87, the product kept as a `double`) divided by the object's float at +0x64
/// (`009cdae0`), rounded to `float`; a sphere (`00c56680` with the radius, 10, 10, 0) is
/// named `"MagicAreaDisplay"`, placed at the point at `011f426c` (`00440460`), passed with
/// a zero point to `00c84470` together with the word `00931ed0` writes, given a new
/// material property (0x4C bytes, `00aa13e0`, `00a75650`; `004534f0` twice with
/// `011a9b7c`, `004bc450` with the zero point) and attached to the node (slot +0xDC with
/// 1), whose properties are then updated (`00a5a040`).
pub fn fn_0081aab0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x20, |e, frame| {
        let first_name = frame.addr();
        let second_name = frame.addr() + 4;
        let out_word = frame.addr() + 8;
        let zero_point = frame.addr() + 0xc;
        let object = e.vcall(base, SLOT_1D0, &args![]).u32();
        let has_display = if object == 0 {
            true
        } else {
            let name = e
                .call(STRING_CONSTRUCT, &args![first_name, MAGIC_AREA_DISPLAY])
                .u32();
            let found = e.vcall(object, SLOT_9C, &args![name]).u32() != 0;
            e.call(STRING_DESTROY, &args![first_name]);
            found
        };
        if has_display {
            return;
        }
        let node = e.vcall(object, SLOT_0C, &args![]).u32();
        if node == 0 {
            return;
        }
        let dominant = e.call(DOMINANT_EFFECT, &args![this]).u32();
        let area = e.call(EFFECT_ITEM_GET_AREA, &args![dominant]).i32();
        let area = f64::from(area);
        let value = e.call(SETTING_VALUE, &args![AREA_SETTING]).u32();
        let product = x87_mul(x87_load32(e.mem.u32(value)), area);
        let divisor = e.call(FLOAT_64, &args![object]).f64();
        let radius = x87_store32(x87_div(product, divisor));
        let sphere = e
            .call(CREATE_SPHERE, &args![radius, 0xau32, 0xau32, 0u32])
            .u32();
        let name = e
            .call(STRING_CONSTRUCT, &args![second_name, MAGIC_AREA_DISPLAY])
            .u32();
        e.call(SET_NAME, &args![sphere, name]);
        e.call(STRING_DESTROY, &args![second_name]);
        e.call(SET_POINT, &args![sphere, POINT_011F426C]);
        e.call(POINT_CONSTRUCT, &args![zero_point, 0.0f32, 0.0f32, 0.0f32]);
        let written = e.call(MOBILE_OBJECT_00931ED0, &args![this, out_word]).u32();
        let word = e.mem.u32(written);
        e.call(COLLISION_00C84470, &args![word, zero_point]);
        let block = e.call(ALLOCATE_AA13E0, &args![0x4cu32]).u32();
        let property = if block == 0 {
            0
        } else {
            e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![block]).u32()
        };
        e.call(
            PROPERTY_004534F0,
            &args![property, PROPERTY_ARGUMENT_011A9B7C],
        );
        e.call(
            PROPERTY_004534F0,
            &args![property, PROPERTY_ARGUMENT_011A9B7C],
        );
        e.call(PROPERTY_004BC450, &args![property, zero_point]);
        e.call(ATTACH_PROPERTY, &args![sphere, property]);
        e.vcall(node, SLOT_DC, &args![sphere, 1u32]);
        e.call(UPDATE_PROPERTIES, &args![node]);
    });
}

// Translated from 0081ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The slot +0x50 of `MagicFogProjectile`'s vtable (`0107ad84`), a save size (`thiscall`,
/// `RET 4`; 16-bit sums): the base size (`0081e830` with the same word) plus 5 * 4, plus 4
/// when `eState` is 1, and, when the save game's version byte (`008df040` on `011de45c`)
/// is at least 0x71, plus 6 and 5 per node of the target list.
pub fn fn_0081ad00(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let base = this.addr();
    let mut size = e.call(PROJECTILE_SAVE_SIZE, &args![this, argument]).u16();
    for _ in 0..5 {
        size = size.wrapping_add(4);
    }
    if e.mem.u32(base + STATE) == 1 {
        size = size.wrapping_add(4);
    }
    let save_game: u32 = e.global(SAVE_GAME);
    if i32::from(e.call(SAVE_VERSION, &args![save_game]).u8()) >= 0x71 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
        let mut node = e.mem.u32(base + TARGET_LIST);
        while node != 0 {
            size = size.wrapping_add(4);
            size = size.wrapping_add(1);
            node = e.mem.u32(node + 8);
        }
    }
    size
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_a920, fn_0081a920(Ptr) -> bool),
        entry!(0x0081_a960, fn_0081a960(Ptr, u32, u32) -> bool),
        entry!(0x0081_aa80, fn_0081aa80(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0081_aab0, fn_0081aab0(Ptr)),
        entry!(0x0081_ad00, fn_0081ad00(Ptr, u32) -> u16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_0081a920_needs_state_not_1_and_flag_clear() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        e.register(FORM_FLAG_20, |_, _| 0u32.into_ret());
        assert!(e.call(0x0081_a920, &[this]).bool());
        e.mem.set_u32(this + STATE, 1);
        assert!(!e.call(0x0081_a920, &[this]).bool());
        e.mem.set_u32(this + STATE, 2);
        e.register(FORM_FLAG_20, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0081_a920, &[this]).bool());
    }

    const SLOT_04_DOUBLE: u32 = 0x0e40_0000;

    fn list_engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0x98, 0x77);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, SLOT_04_DOUBLE);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, vtable);
        e.register(SLOT_04_DOUBLE, |_, _| 0xaaa0u32.into_ret());
        e.register(WORD_08, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(PROJECTILE_WORD_98, |e, a| e.mem.u32(a[0] + 0x98).into_ret());
        e.register(WORD_0C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        (e, this, first)
    }

    #[test]
    fn fn_0081a960_adds_new_pairs_once() {
        let (mut e, this, first) = list_engine();
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x77);
        e.mem.set_u32(second + 0xc, 0xbbb0);
        assert!(!e.call(0x0081_a960, &[this, 0, second]).bool());
        assert!(!e.call(0x0081_a960, &[this, first, 0]).bool());
        assert!(e.call(0x0081_a960, &[this, first, second]).bool());
        let head = e.mem.u32(this + TARGET_LIST);
        assert_eq!(e.mem.u32(head), 0xaaa0);
        assert_eq!(e.mem.u32(head + 4), 0xbbb0);
        assert_eq!(e.mem.u32(head + 8), 0);
        // The same pair again is refused.
        assert!(!e.call(0x0081_a960, &[this, first, second]).bool());
        // Another pair goes in front.
        e.mem.set_u32(second + 0xc, 0xccc0);
        assert!(e.call(0x0081_a960, &[this, first, second]).bool());
        let new_head = e.mem.u32(this + TARGET_LIST);
        assert_eq!(e.mem.u32(new_head + 4), 0xccc0);
        assert_eq!(e.mem.u32(new_head + 8), head);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0xc], vec![0xc]]);
    }

    #[test]
    fn fn_0081a960_other_owner_is_true_without_adding() {
        let (mut e, this, first) = list_engine();
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x78);
        assert!(e.call(0x0081_a960, &[this, first, second]).bool());
        assert_eq!(e.mem.u32(this + TARGET_LIST), 0);
        assert!(calls(&e, SLOT_04_DOUBLE).is_empty());
    }

    #[test]
    fn fn_0081a960_failed_allocation_empties_the_head() {
        let (mut e, this, first) = list_engine();
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x77);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert!(e.call(0x0081_a960, &[this, first, second]).bool());
        assert_eq!(e.mem.u32(this + TARGET_LIST), 0);
    }

    #[test]
    fn fn_0081aa80_stores_the_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0081_aa80, &[node, 1, 2, 3]).u32(), node);
        assert_eq!(
            (e.mem.u32(node), e.mem.u32(node + 4), e.mem.u32(node + 8)),
            (1, 2, 3)
        );
    }

    const SLOT_1D0_DOUBLE: u32 = 0x0e41_0000;
    const SLOT_9C_DOUBLE: u32 = 0x0e41_0100;
    const SLOT_0C_DOUBLE: u32 = 0x0e41_0200;
    const SLOT_DC_DOUBLE: u32 = 0x0e41_0300;

    struct Display {
        e: Engine,
        this: u32,
        object: u32,
    }

    fn display_engine() -> Display {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_1D0, SLOT_1D0_DOUBLE);
        e.mem.set_u32(vtable + SLOT_9C, SLOT_9C_DOUBLE);
        e.mem.set_u32(vtable + SLOT_0C, SLOT_0C_DOUBLE);
        e.mem.set_u32(vtable + SLOT_DC, SLOT_DC_DOUBLE);
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this, vtable);
        let object = e.mem.alloc(0x70);
        e.mem.set_u32(object, vtable);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 3.0);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x4242);
        e.register_double(SLOT_1D0_DOUBLE, move |_, _| object.into_ret());
        e.register(SLOT_9C_DOUBLE, |_, _| 0u32.into_ret());
        e.register(SLOT_0C_DOUBLE, |_, a| a[0].into_ret());
        e.register(SLOT_DC_DOUBLE, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STRING_DESTROY, |_, _| Ret::default());
        e.register(DOMINANT_EFFECT, |_, _| 0x900u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 10u32.into_ret());
        e.register_double(SETTING_VALUE, move |_, _| value.into_ret());
        e.register(FLOAT_64, |_, _| 4.0f64.into_ret());
        e.register(CREATE_SPHERE, |_, _| 0x5150u32.into_ret());
        e.register_double(MOBILE_OBJECT_00931ED0, move |_, _| word.into_ret());
        e.register(ALLOCATE_AA13E0, |_, _| 0x6000u32.into_ret());
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        for callee in [
            SET_NAME,
            SET_POINT,
            POINT_CONSTRUCT,
            COLLISION_00C84470,
            PROPERTY_004534F0,
            PROPERTY_004BC450,
            ATTACH_PROPERTY,
            UPDATE_PROPERTIES,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        Display { e, this, object }
    }

    #[test]
    fn fn_0081aab0_builds_the_area_sphere() {
        let mut d = display_engine();
        d.e.call(0x0081_aab0, &[d.this]);
        // radius = 3.0 * 10 / 4.0
        assert_eq!(
            calls(&d.e, CREATE_SPHERE),
            vec![vec![7.5f32.to_bits(), 10, 10, 0]]
        );
        let names = calls(&d.e, STRING_CONSTRUCT);
        assert_eq!(names.len(), 2);
        assert!(names.iter().all(|n| n[1] == MAGIC_AREA_DISPLAY));
        assert_eq!(calls(&d.e, STRING_DESTROY).len(), 2);
        assert_eq!(calls(&d.e, SET_NAME), vec![vec![0x5150, names[1][0]]]);
        assert_eq!(calls(&d.e, SET_POINT), vec![vec![0x5150, POINT_011F426C]]);
        let zero = calls(&d.e, POINT_CONSTRUCT);
        assert_eq!(zero[0][1..], [0, 0, 0]);
        assert_eq!(
            calls(&d.e, COLLISION_00C84470),
            vec![vec![0x4242, zero[0][0]]]
        );
        assert_eq!(calls(&d.e, ALLOCATE_AA13E0), vec![vec![0x4c]]);
        assert_eq!(
            calls(&d.e, PROPERTY_004534F0),
            vec![
                vec![0x6001, PROPERTY_ARGUMENT_011A9B7C],
                vec![0x6001, PROPERTY_ARGUMENT_011A9B7C]
            ]
        );
        assert_eq!(calls(&d.e, ATTACH_PROPERTY), vec![vec![0x5150, 0x6001]]);
        assert_eq!(calls(&d.e, SLOT_DC_DOUBLE), vec![vec![d.object, 0x5150, 1]]);
        assert_eq!(calls(&d.e, UPDATE_PROPERTIES), vec![vec![d.object]]);
    }

    #[test]
    fn fn_0081aab0_stops_when_shown_or_without_node() {
        let mut d = display_engine();
        d.e.register(SLOT_9C_DOUBLE, |_, _| 1u32.into_ret());
        d.e.call(0x0081_aab0, &[d.this]);
        assert_eq!(calls(&d.e, STRING_DESTROY).len(), 1);
        assert!(calls(&d.e, SLOT_0C_DOUBLE).is_empty());

        let mut d = display_engine();
        d.e.register(SLOT_0C_DOUBLE, |_, _| 0u32.into_ret());
        d.e.call(0x0081_aab0, &[d.this]);
        assert!(calls(&d.e, CREATE_SPHERE).is_empty());

        let mut d = display_engine();
        d.e.register(SLOT_1D0_DOUBLE, |_, _| 0u32.into_ret());
        d.e.call(0x0081_aab0, &[d.this]);
        assert!(calls(&d.e, STRING_CONSTRUCT).is_empty());
        assert!(calls(&d.e, CREATE_SPHERE).is_empty());
    }

    #[test]
    fn fn_0081ad00_counts_the_target_list() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(SAVE_GAME, 0x3000);
        let this = e.mem.alloc(0xd0);
        e.register(PROJECTILE_SAVE_SIZE, |_, _| 0x100u32.into_ret());
        e.register(SAVE_VERSION, |_, _| 0x70u32.into_ret());
        assert_eq!(e.call(0x0081_ad00, &[this, 0]).u32(), 0x114);
        e.mem.set_u32(this + STATE, 1);
        assert_eq!(e.call(0x0081_ad00, &[this, 0]).u32(), 0x118);
        let second = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first + 8, second);
        e.mem.set_u32(this + TARGET_LIST, first);
        e.register(SAVE_VERSION, |_, _| 0x171u32.into_ret());
        // The low byte 0x71 counts: 0x118 + 6 + 2 * 5.
        assert_eq!(e.call(0x0081_ad00, &[this, 0]).u32(), 0x118 + 6 + 10);
    }
}

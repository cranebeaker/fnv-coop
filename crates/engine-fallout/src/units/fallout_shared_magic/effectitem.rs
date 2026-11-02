//! `fallout shared/magic/effectitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

layout! {
    /// `EffectItemData` (Xbox PDB), PC offsets checked at 004043bc-004045c1.
    pub struct EffectItemData: 0x14 {
        0x00 iMagnitude: i32,
        0x04 iArea: i32,
        0x08 iDuration: i32,
        0x0C iRange: u32,
        0x10 iActorValue: u32,
    }
}

layout! {
    /// `EffectItem` (Xbox PDB), PC offsets checked at 00403f0a-0040428c.
    pub struct EffectItem: 0x24 {
        0x00 data: Inline<EffectItemData>,
        0x14 pEffectSetting: Ptr,
        0x18 fRawCost: f32,
    }
}

layout! {
    /// `SettingCollectionMap<Setting>` (Xbox PDB), PC offsets checked at
    /// 00404d13, 00404d3c and 00404e47.
    pub struct SettingCollectionMap: 0x120 {
        0x108 pHandle: Ptr,
        0x10c settings: Inline<SettingStringMap>,
    }
}

layout! {
    /// `BSTCaseInsensitiveStringMap<Setting *>` (Xbox PDB), PC offsets
    /// checked at 0040528b-004052d9.
    pub struct SettingStringMap: 0x14 {
        0x04 m_uiHashSize: u32,
        0x08 m_ppkHashTable: Ptr,
        0x10 m_bCopy: bool,
    }
}

const INVALID_COST: u32 = 0x0101_2054; // float -1.0 in the exe

fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

fn invalidate_cost(e: &mut Engine, this: Ptr<EffectItem>) {
    let invalid: f32 = e.global(INVALID_COST);
    e.set(this, EffectItem::fRawCost, quiet_f32(invalid));
}

// Translated from 00403db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::InitItem` (Xbox PDB): initialize the conditions of an unmarked form.
pub fn effect_item_init_item(e: &mut Engine, this: Ptr<EffectItem>, form: Ptr) {
    if e.call(0x0040_13e0, &args![form]).bool() {
        return;
    }
    let conditions = e.call(0x0040_9140, &args![this]).ptr::<()>();
    if !conditions.is_null() {
        let conditions = e.call(0x0040_9140, &args![this]).ptr::<()>();
        e.call(0x0068_0a20, &args![conditions, form]);
    }
}

// Translated from 00403df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return the word at +4 of a non-null object, otherwise zero.
pub fn fn_00403df0(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        0
    } else {
        e.mem.u32(object.addr() + 4)
    }
}

// Translated from 00403e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return an object's +4 field address; null selects a zeroed global float.
pub fn fn_00403e20(e: &mut Engine, object: Ptr) -> Ptr {
    if object.is_null() {
        e.set_global(0x0120_2800, 0.0f32);
        Ptr::new(0x0120_2800)
    } else {
        object.byte_add(4)
    }
}

// Translated from 00403e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward a mask to the effect setting's flag predicate, returning AL.
pub fn fn_00403e60(e: &mut Engine, setting: Ptr, mask: u32) -> bool {
    e.call(0x0058_cba0, &args![setting, mask]).bool()
}

// Translated from 00403e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Test whether the setting's actor-value index differs from -1.
pub fn fn_00403e80(e: &mut Engine, setting: Ptr) -> bool {
    e.call(0x0058_6150, &args![setting]).i32() != -1
}

// Translated from 00403ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return the stored actor value only when the setting admits it.
pub fn fn_00403ea0(e: &mut Engine, this: Ptr<EffectItem>) -> i32 {
    if fn_00403ee0(e, this) {
        e.get(this.at(EffectItem::data), EffectItemData::iActorValue) as i32
    } else {
        -1
    }
}

// Translated from 00403ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Effect setting's actor-value eligibility predicate.
pub fn fn_00403ee0(e: &mut Engine, this: Ptr<EffectItem>) -> bool {
    let setting = e.get(this, EffectItem::pEffectSetting);
    e.call(0x0040_3c60, &args![setting]).bool()
}

// Translated from 00403f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetRawCost` (Xbox PDB): calculate and cache a negative cost.
pub fn effect_item_get_raw_cost(e: &mut Engine, this: Ptr<EffectItem>) -> f32 {
    let mut cost = e.get(this, EffectItem::fRawCost);
    let zero: f64 = e.global(0x0101_2060);
    if (cost as f64) < zero {
        let range_two = e.call(0x0084_e3a0, &args![this]).u32() == 2;
        let magnitude = effect_item_get_magnitude(e, this);
        let duration = effect_item_get_duration(e, this);
        let area = effect_item_get_area(e, this);
        let setting = e.call(0x0082_5c00, &args![this]).ptr::<()>();
        let base_cost = e.call(0x0063_9b20, &args![setting]).f32();
        cost = e
            .call(
                0x0064_90b0,
                &args![base_cost, area, duration, magnitude, range_two],
            )
            .f32();
        e.set(this, EffectItem::fRawCost, cost);
    }
    quiet_f32(cost) // The final FLD from cache to ST0 quiets signaling NaNs.
}

// Translated from 00403f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetCost` (Xbox PDB): adjust, floor, and clamp the raw cost.
pub fn effect_item_get_cost(e: &mut Engine, this: Ptr<EffectItem>, actor: Ptr) -> f32 {
    let mut cost = effect_item_get_raw_cost(e, this);
    if !actor.is_null() {
        let owner = actor.byte_add(0xa4);
        // 00403faa: the first result is pushed; it is the third word of the cdecl
        // `00649180` (ADD ESP,0xc), which does not read it.
        let first = e.call(0x0066_ef20, &args![owner, 11u32]).u32();
        let skill = effect_item_get_associated_skill(e, this);
        let skill_value = e.call(0x0066_ef20, &args![owner, skill]).u32();
        cost = e.call(0x0064_9180, &args![cost, skill_value, first]).f32();
    }
    let floored = fn_00404040(e, cost);
    fn_00404010(e, floored, 1.0)
}

// Translated from 00404010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return the greater float, selecting the second on an unordered comparison.
pub fn fn_00404010(_e: &mut Engine, first: f32, second: f32) -> f32 {
    let selected = if second < first { first } else { second };
    quiet_f32(selected) // x87 FLD/FSTP quiets the selected signaling NaN.
}

// Translated from 00404040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Floor through the game's float conversion and x87 subtraction.
pub fn fn_00404040(e: &mut Engine, value: f32) -> f32 {
    let mut truncated = fn_00404090(e, value) as f32;
    let zero: f64 = e.global(0x0101_2060);
    let difference = value as f64 - truncated as f64;
    if matches!(
        difference.partial_cmp(&zero),
        None | Some(std::cmp::Ordering::Less)
    ) {
        let one: f64 = e.global(0x0101_2070);
        truncated = (truncated as f64 - one) as f32;
    }
    truncated
}

// Translated from 00404090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Float-to-integer helper's integer result converted back to x87 float.
pub fn fn_00404090(e: &mut Engine, value: f32) -> f64 {
    e.call(0x00ec_62c0, &args![value as f64]).i32() as f64
}

// Translated from 00404390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetMagnitude` (Xbox PDB): zero when disallowed by flags.
pub fn effect_item_get_magnitude(e: &mut Engine, this: Ptr<EffectItem>) -> i32 {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x100) {
        0
    } else {
        e.get(this.at(EffectItem::data), EffectItemData::iMagnitude)
    }
}

// Translated from 004043d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Set a nonnegative magnitude when flags allow it; invalidate cached cost.
pub fn fn_004043d0(e: &mut Engine, this: Ptr<EffectItem>, value: i32) -> bool {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x100) || value < 0 {
        return false;
    }
    e.set(this.at(EffectItem::data), EffectItemData::iMagnitude, value);
    invalidate_cost(e, this);
    true
}

// Translated from 00404420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetArea` (Xbox PDB): zero for flagged or self-range effects.
pub fn effect_item_get_area(e: &mut Engine, this: Ptr<EffectItem>) -> i32 {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x200) || e.call(0x0084_e3a0, &args![this]).u32() == 0 {
        0
    } else {
        e.get(this.at(EffectItem::data), EffectItemData::iArea)
    }
}

// Translated from 00404470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Set a nonnegative area for a non-self range when flags allow it.
pub fn fn_00404470(e: &mut Engine, this: Ptr<EffectItem>, value: i32) -> bool {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x200) || e.call(0x0084_e3a0, &args![this]).u32() == 0 || value < 0 {
        return false;
    }
    e.set(this.at(EffectItem::data), EffectItemData::iArea, value);
    invalidate_cost(e, this);
    true
}

// Translated from 004044d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetDuration` (Xbox PDB): zero when disallowed by flags.
pub fn effect_item_get_duration(e: &mut Engine, this: Ptr<EffectItem>) -> i32 {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x80) {
        0
    } else {
        e.get(this.at(EffectItem::data), EffectItemData::iDuration)
    }
}

// Translated from 00404510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Set a nonnegative duration when flags allow it; invalidate cached cost.
pub fn fn_00404510(e: &mut Engine, this: Ptr<EffectItem>, value: i32) -> bool {
    let setting = e.get(this, EffectItem::pEffectSetting);
    if fn_00403e60(e, setting, 0x80) || value < 0 {
        return false;
    }
    e.set(this.at(EffectItem::data), EffectItemData::iDuration, value);
    invalidate_cost(e, this);
    true
}

// Translated from 00404560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Set range 0, 1, or 2 only under its corresponding setting flag.
pub fn fn_00404560(e: &mut Engine, this: Ptr<EffectItem>, range: u32) -> bool {
    let mask = match range {
        0 => 0x10,
        1 => 0x20,
        2 => 0x40,
        _ => return false,
    };
    let setting = e.get(this, EffectItem::pEffectSetting);
    if !fn_00403e60(e, setting, mask) {
        return false;
    }
    e.set(this.at(EffectItem::data), EffectItemData::iRange, range);
    invalidate_cost(e, this);
    true
}

// Translated from 00404650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetAssociatedSkill` (Xbox PDB): forward to the setting.
pub fn effect_item_get_associated_skill(e: &mut Engine, this: Ptr<EffectItem>) -> u32 {
    let setting = e.get(this, EffectItem::pEffectSetting);
    e.call(0x0070_ec90, &args![setting]).u32()
}

// Translated from 00404710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::IsHostile` (Xbox PDB): test bit 1 on its effect setting.
pub fn effect_item_is_hostile(e: &mut Engine, this: Ptr<EffectItem>) -> bool {
    let setting = e.call(0x0082_5c00, &args![this]).ptr::<()>();
    fn_00403e60(e, setting, 1)
}

// Translated from 00404730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::IsUsageMonitorEffect` (Xbox PDB): compare setting pointers.
pub fn effect_item_is_usage_monitor_effect(e: &mut Engine, this: Ptr<EffectItem>) -> bool {
    let setting = e.call(0x0082_5c00, &args![this]).u32();
    setting == e.call(0x0040_8b50, &args![]).u32()
}

// Translated from 004040b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::Save` (Xbox PDB): write EFIT data and optional conditions.
pub fn effect_item_save(e: &mut Engine, this: Ptr<EffectItem>) {
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x0058_e3a0, &args![this]);
    }
    e.call(0x0048_5990, &args![0x5449_4645u32, this, 0x14u32]);
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x0058_e3a0, &args![this]);
    }
    let conditions = e.call(0x0040_9140, &args![this]).ptr::<()>();
    if !conditions.is_null() {
        let conditions = e.call(0x0040_9140, &args![this]).ptr::<()>();
        e.call(0x0068_0910, &args![conditions]);
    }
}

// Translated from 00404120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::Load` (Xbox PDB): consume EFIT or CTDA chunk.
pub fn effect_item_load(
    e: &mut Engine,
    this: Ptr<EffectItem>,
    file: Ptr,
    effect_name: Ptr,
) -> bool {
    let chunk = e.call(0x0047_26b0, &args![file]).u32();
    if chunk == 0x4144_5443 {
        let conditions = e.call(0x0040_9140, &args![this]).ptr::<()>();
        e.call(0x0068_0960, &args![conditions, file]);
    } else if chunk == 0x5449_4645 {
        e.call(0x0040_3d30, &args![this, 0u32, 0x14u32]);
        e.call(0x0047_2890, &args![file, this, 0x14u32]);
        if e.call(0x0040_1680, &args![file]).bool() {
            e.call(0x0058_e3a0, &args![this]);
        }
        let setting = e.get(this, EffectItem::pEffectSetting);
        if setting.is_null() {
            let name = if effect_name.is_null() {
                Ptr::new(0x0101_20d8)
            } else {
                effect_name
            };
            e.call(0x005b_5e40, &args![0x0101_2078u32, name]);
        } else if e.call(0x0040_3c60, &args![setting]).bool() {
            let setting = e.get(this, EffectItem::pEffectSetting);
            if fn_00403e80(e, setting) {
                let setting = e.get(this, EffectItem::pEffectSetting);
                let index = e.call(0x0058_6150, &args![setting]).u32();
                e.set(
                    this.at(EffectItem::data),
                    EffectItemData::iActorValue,
                    index,
                );
            }
        }
    } else {
        return false;
    }
    true
}

// Translated from 00404210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy an effect item's fields through their validated setters and conditions.
pub fn fn_00404210(e: &mut Engine, this: Ptr<EffectItem>, source: Ptr<EffectItem>) {
    let setting = e.get(source, EffectItem::pEffectSetting);
    e.set(this, EffectItem::pEffectSetting, setting);
    let range = e.call(0x0084_e3a0, &args![source]).u32();
    fn_00404560(e, this, range);
    let magnitude = effect_item_get_magnitude(e, source);
    fn_004043d0(e, this, magnitude);
    let area = effect_item_get_area(e, source);
    fn_00404470(e, this, area);
    let duration = effect_item_get_duration(e, source);
    fn_00404510(e, this, duration);
    let actor_value = fn_00403ea0(e, source);
    e.call(0x0043_7730, &args![this, actor_value]);
    let conditions = e.call(0x0040_9140, &args![source]).ptr::<()>();
    fn_00404760(e, this, conditions);
    invalidate_cost(e, this);
}

// Translated from 004042a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return true if the effect items differ in setting, values, or conditions.
pub fn fn_004042a0(e: &mut Engine, this: Ptr<EffectItem>, other: Ptr<EffectItem>) -> bool {
    let mut different =
        e.call(0x0082_5c00, &args![other]).u32() != e.call(0x0082_5c00, &args![this]).u32();
    if !different {
        different = effect_item_get_magnitude(e, other) != effect_item_get_magnitude(e, this);
    }
    if !different {
        different = effect_item_get_area(e, other) != effect_item_get_area(e, this);
    }
    if !different {
        different = effect_item_get_duration(e, other) != effect_item_get_duration(e, this);
    }
    if !different {
        different =
            e.call(0x0084_e3a0, &args![other]).u32() != e.call(0x0084_e3a0, &args![this]).u32();
    }
    if !different {
        different = fn_00403ea0(e, other) != fn_00403ea0(e, this);
    }
    let left = e.call(0x0040_9140, &args![this]).ptr::<()>();
    if !left.is_null() {
        let right = e.call(0x0040_9140, &args![other]).ptr::<()>();
        let left = e.call(0x0040_9140, &args![this]).ptr::<()>();
        if e.call(0x0068_0b70, &args![left, right]).bool() {
            return true;
        }
    } else if !e.call(0x0040_9140, &args![other]).ptr::<()>().is_null() {
        return true;
    }
    different
}

// Translated from 004045e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Find a linked form only for the setting's checked flag and form type.
pub fn fn_004045e0(e: &mut Engine, this: Ptr<EffectItem>) -> Ptr {
    let setting = e.call(0x0082_5c00, &args![this]).ptr::<()>();
    let flags = e.call(0x0081_0fd0, &args![setting]).u32();
    if !e.call(0x0040_3c80, &args![flags, 0x40u32]).bool() {
        return Ptr::NULL;
    }
    let linked = e.call(0x005f_5f80, &args![setting]).ptr::<()>();
    if linked.is_null() {
        return Ptr::NULL;
    }
    let linked = e.call(0x005f_5f80, &args![setting]).ptr::<()>();
    if e.call(0x0040_1170, &args![linked]).u32() != 0x11 {
        return Ptr::NULL;
    }
    let setting = e.call(0x0082_5c00, &args![this]).ptr::<()>();
    e.call(0x005f_5f80, &args![setting]).ptr::<()>()
}

// Translated from 00404670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectItem::GetEffectName` (Xbox PDB): construct the setting name into output.
pub fn effect_item_get_effect_name(e: &mut Engine, this: Ptr<EffectItem>, output: Ptr) -> Ptr {
    let setting = e.get(this, EffectItem::pEffectSetting);
    fn_004046b0(e, setting, output)
}

// Translated from 004046b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy the effect setting's name string to the result object.
pub fn fn_004046b0(e: &mut Engine, setting: Ptr, output: Ptr) -> Ptr {
    let name = e
        .call(0x0071_7e50, &args![setting.byte_add(0x38)])
        .ptr::<()>();
    fn_004047f0(e, output, name)
}

// Translated from 004046f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward two words to the CRT string-copy wrapper.
pub fn fn_004046f0(e: &mut Engine, destination: Ptr, source: Ptr) {
    e.call(0x00ec_6370, &args![destination, source]);
}

// Translated from 00404760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assign an existing condition list or a temporary empty list.
// Translation incomplete: 00404760 - compiler SEH unwinding is not modeled.
pub fn fn_00404760(e: &mut Engine, this: Ptr<EffectItem>, source: Ptr) {
    let destination = this.byte_add(0x1c);
    if !source.is_null() {
        e.call(0x0068_0a70, &args![destination, source, 0u32]);
    } else {
        e.with_stack(8, |e, empty| {
            e.call(0x0068_0890, &args![empty]);
            e.call(0x0068_0a70, &args![destination, empty, 0u32]);
            e.call(0x0068_08b0, &args![empty]);
        });
    }
}

// Translated from 004047f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct an output string and assign the supplied source string.
pub fn fn_004047f0(e: &mut Engine, output: Ptr, source: Ptr) -> Ptr {
    e.call(0x0040_3970, &args![output]);
    let chars = e.call(0x0055_9450, &args![source]).ptr::<()>();
    e.call(0x0040_37f0, &args![output, chars, 0u32]);
    output
}

// Translated from 00404820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStringT<char>::operator+=` (Xbox PDB): append a C string.
pub fn bs_string_append(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    if source.is_null() {
        return this;
    }
    let chars = e.call(0x0055_9450, &args![this]).ptr::<()>();
    if chars.is_null() {
        e.call(0x0040_37f0, &args![this, source, 0u32]);
    } else {
        let length = e.call(0x0044_a670, &args![source]).u32();
        let new_length = fn_004048e0(e, this).wrapping_add(length);
        let capacity = e.call(0x0040_39a0, &args![this]).u32();
        if new_length > capacity {
            let old_chars = e.call(0x0055_9450, &args![this]).ptr::<()>();
            e.call(0x0040_37f0, &args![this, old_chars, new_length]);
        }
        let old_length = fn_004048e0(e, this);
        let destination = e.mem.u32(this.addr()).wrapping_add(old_length);
        e.call(
            0x0040_1460,
            &args![destination, source, length.wrapping_add(1)],
        );
        e.call(0x0040_39c0, &args![this, new_length]);
    }
    this
}

// Translated from 004048e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return the cached string length, or calculate it for sentinel 0xffff.
pub fn fn_004048e0(e: &mut Engine, this: Ptr) -> u32 {
    let length = e.mem.u16(this.addr() + 4);
    if length == 0xffff {
        let chars = e.mem.u32(this.addr());
        e.call(0x0044_a670, &args![chars]).u32()
    } else {
        length as u32
    }
}

// Translated from 00404920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct a `Setting` (Xbox PDB) with `pKey` and `uValue`.
pub fn fn_00404920(e: &mut Engine, this: Ptr, key: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_2120);
    // Setting::pKey and Setting::uValue (Xbox PDB), PC offsets +8 and +4.
    e.mem.set_u32(this.addr() + 8, key.addr());
    e.mem.set_u32(this.addr() + 4, value);
    this
}

// Translated from 00404950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy a `Setting` (Xbox PDB), freeing keys beginning with 'S'.
pub fn fn_00404950(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2120);
    // Setting::pKey (Xbox PDB), PC offset +8.
    let key = e.mem.u32(this.addr() + 8);
    if key != 0 && e.mem.u8(key) == b'S' {
        e.call(0x0040_1030, &args![key]);
    }
}

// Translated from 004049a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Setting` scalar deleting destructor (Xbox PDB).
pub fn setting_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404950(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004049d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<GameSettingCollection>` scalar deleting destructor (Xbox PDB).
pub fn setting_t_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404a00(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00404a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remove a setting from the singleton collection before base destruction.
// Translation incomplete: 00404a00 - compiler SEH unwinding is not modeled.
pub fn fn_00404a00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2114);
    let collection = fn_00404a70(e);
    e.vcall(collection.addr(), 8, &args![this]);
    fn_00404950(e, this);
}

// Translated from 00404a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Return the lazily constructed global settings collection.
pub fn fn_00404a70(e: &mut Engine) -> Ptr {
    e.call(0x0040_4a80, &args![]);
    e.global(0x011c_8048)
}

// Translated from 00404a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lazily allocate and construct the global game settings collection.
// Translation incomplete: 00404a80 - compiler SEH unwinding is not modeled.
pub fn fn_00404a80(e: &mut Engine) {
    if e.global::<u32>(0x011c_8048) == 0 {
        let storage = e.call(0x0040_1000, &args![0x120u32]).ptr::<()>();
        let collection = if storage.is_null() {
            Ptr::NULL
        } else {
            fn_00404b10(e, storage)
        };
        e.set_global(0x011c_8048, collection);
    }
}

// Translated from 00404b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct a `GameSettingCollection` (Xbox PDB) and install its vtable.
pub fn fn_00404b10(e: &mut Engine, this: Ptr) -> Ptr {
    let map_size: u32 = e.global(0x0118_9c40);
    fn_00404b90(e, this, map_size);
    e.mem.set_u32(this.addr(), 0x0101_212c);
    this
}

// Translated from 00404b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for `GameSettingCollection` (Xbox PDB).
pub fn fn_00404b40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404b70(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00404b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy the `GameSettingCollection` (Xbox PDB) base.
pub fn fn_00404b70(e: &mut Engine, this: Ptr) {
    fn_00404c00(e, this);
}

// Translated from 00404b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct the setting collection and its embedded string map.
// Translation incomplete: 00404b90 - compiler SEH unwinding is not modeled.
pub fn fn_00404b90(e: &mut Engine, this: Ptr, map_size: u32) -> Ptr {
    fn_00404d00(e, this);
    e.mem.set_u32(this.addr(), 0x0101_215c);
    fn_00404d50(e, this.byte_add(0x10c), map_size, false);
    this
}

// Translated from 00404c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy the embedded string map and then the setting collection base.
// Translation incomplete: 00404c00 - compiler SEH unwinding is not modeled.
pub fn fn_00404c00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_215c);
    e.call(0x0043_8af0, &args![this.byte_add(0x10c)]);
    fn_00404da0(e, this.byte_add(0x10c));
    fn_00404c80(e, this);
}

// Translated from 00404c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restore the `SettingCollection<Setting>` (Xbox PDB) vtable and close it.
pub fn fn_00404c80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2188);
    e.call(0x008d_0360, &args![this]);
}

// Translated from 00404ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for `SettingCollectionMap<Setting>` (Xbox PDB).
pub fn fn_00404ca0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404c00(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00404cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for `SettingCollection<Setting>` (Xbox PDB).
pub fn setting_collection_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404c80(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00404d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialize the `SettingCollection<Setting>` (Xbox PDB) base fields.
pub fn fn_00404d00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_2188);
    e.mem.set_u32(this.addr() + 0x108, 0);
    e.mem.set_u8(this.addr() + 4, 0);
    this
}

// Translated from 00404d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the collection's file handle is non-null.
pub fn fn_00404d30(e: &mut Engine, this: Ptr<SettingCollectionMap>) -> bool {
    !e.get(this, SettingCollectionMap::pHandle).is_null()
}

// Translated from 00404d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct the embedded case-insensitive setting string map.
pub fn fn_00404d50(e: &mut Engine, this: Ptr, hash_size: u32, copy: bool) -> Ptr {
    e.call(0x0040_5570, &args![this, hash_size, copy]);
    e.mem.set_u32(this.addr(), 0x0101_21b4);
    this
}

// Translated from 00404d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward the destination, capacity and source to the CRT copy routine.
pub fn fn_00404d80(e: &mut Engine, destination: Ptr, capacity: u32, source: Ptr) -> Ptr {
    e.call(0x00ec_65a6, &args![destination, capacity, source]);
    destination
}

// Translated from 00404da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restore the case-insensitive map vtable and destroy its base.
pub fn fn_00404da0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_21b4);
    fn_00405230(e, this);
}

// Translated from 00404dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forward two strings to the CRT case-insensitive comparison.
pub fn fn_00404dc0(e: &mut Engine, left: Ptr, right: Ptr) -> i32 {
    e.call(0x00ec_68e4, &args![left, right]).i32()
}

// Translated from 00404de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionMap<Setting>::Add` (Xbox PDB): add a setting by its key,
/// warning when the map already contains that key.
// Translation incomplete: 00404de0 - compiler SEH unwinding is not modeled.
pub fn setting_collection_map_add(e: &mut Engine, this: Ptr, setting: Ptr) {
    e.with_stack(8, |e, guard| {
        fn_00404eb0(e, guard, 9, true, Ptr::new(0x0101_2220), 0x21b);
        if !setting.is_null() {
            let key = e.call(0x0044_ddc0, &args![setting]).ptr::<()>();
            if !key.is_null() {
                let key = e.call(0x0044_ddc0, &args![setting]).ptr::<()>();
                let duplicate = e
                    .call(
                        0x0085_3130,
                        &args![this.byte_add(0x10c), key, guard.byte_add(4)],
                    )
                    .bool();
                if duplicate {
                    let key = e.call(0x0044_ddc0, &args![setting]).ptr::<()>();
                    e.call(0x00c3_35a0, &args![0x0101_21d0u32, key]);
                } else {
                    let key = e.call(0x0044_ddc0, &args![setting]).ptr::<()>();
                    e.call(0x0047_0200, &args![this.byte_add(0x10c), key, setting]);
                }
            }
        }
        fn_00404ee0(e, guard);
    });
}

// Translated from 00404eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct a temporary TLS setting guard; only `new_value` is read.
pub fn fn_00404eb0(
    e: &mut Engine,
    this: Ptr,
    new_value: u32,
    mode: bool,
    source: Ptr,
    line: u32,
) -> Ptr {
    fn_00404f00(e, this, new_value, mode, source, line);
    this
}

// Translated from 00404ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restore the TLS setting guard's saved value.
pub fn fn_00404ee0(e: &mut Engine, this: Ptr) {
    fn_00404f70(e, this);
}

// Translated from 00404f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Save the prior TLS setting guard value, then set a new one.
pub fn fn_00404f00(
    e: &mut Engine,
    this: Ptr,
    new_value: u32,
    _unused_mode: bool,
    _unused_source: Ptr,
    _unused_line: u32,
) {
    let previous = fn_00404f50(e);
    e.mem.set_u32(this.addr(), previous);
    fn_00404f30(e, new_value);
}

// Translated from 00404f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Set the thread-local setting guard at offset +0x2b4.
pub fn fn_00404f30(e: &mut Engine, value: u32) {
    let tls = e.tls();
    e.mem.set_u32(tls + 0x2b4, value);
}

// Translated from 00404f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Get the thread-local setting guard at offset +0x2b4.
pub fn fn_00404f50(e: &mut Engine) -> u32 {
    let tls = e.tls();
    e.mem.u32(tls + 0x2b4)
}

// Translated from 00404f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restore a saved thread-local setting guard value.
pub fn fn_00404f70(e: &mut Engine, this: Ptr) {
    let previous = e.mem.u32(this.addr());
    fn_00404f30(e, previous);
}

// Translated from 00404f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionMap<Setting>::Remove` (Xbox PDB): remove a non-null
/// setting with a non-null key from the embedded map.
pub fn setting_collection_map_remove(e: &mut Engine, this: Ptr, setting: Ptr) {
    if !setting.is_null() && !e.call(0x0044_ddc0, &args![setting]).ptr::<()>().is_null() {
        let key = e.call(0x0044_ddc0, &args![setting]).ptr::<()>();
        e.call(0x0040_5430, &args![this.byte_add(0x10c), key]);
    }
}

// Translated from 00404fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionMap<Setting>::GetViewerStrings` (Xbox PDB): visit each
/// non-null setting and count the ones sent to its viewer-string method.
// Translation incomplete: 00404fd0 - compiler SEH unwinding is not modeled.
pub fn setting_collection_map_get_viewer_strings(e: &mut Engine, this: Ptr, output: Ptr) -> u32 {
    e.with_stack(4, |e, guard| {
        fn_00404eb0(e, guard, 9, true, Ptr::new(0x0101_2220), 0x239);
        let count = e.with_stack(12, |e, work| {
            let map = this.byte_add(0x10c);
            let first = e.call(0x004b_9ba0, &args![map]).u32();
            e.mem.set_u32(work.addr(), first);
            let mut count = 0u32;
            while e.mem.u32(work.addr()) != 0 {
                e.call(
                    0x006b_7f20,
                    &args![map, work, work.byte_add(4), work.byte_add(8)],
                );
                let setting = e.mem.u32(work.addr() + 8);
                if setting != 0 {
                    e.call(0x00c3_3280, &args![setting, output]);
                    count = count.wrapping_add(1);
                }
            }
            count
        });
        fn_00404ee0(e, guard);
        count
    })
}

// Translated from 004050a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionMap<Setting>::WriteSettings` (Xbox PDB): write every
/// non-null setting while preserving the accumulated success byte.
pub fn setting_collection_map_write_settings(
    e: &mut Engine,
    this: Ptr<SettingCollectionMap>,
) -> bool {
    let mut success = fn_00404d30(e, this) as u8;
    if success != 0 {
        e.with_stack(12, |e, work| {
            let map = this.byte_add(0x10c);
            let first = e.call(0x004b_9ba0, &args![map]).u32();
            e.mem.set_u32(work.addr(), first);
            while e.mem.u32(work.addr()) != 0 {
                e.call(
                    0x006b_7f20,
                    &args![map, work, work.byte_add(4), work.byte_add(8)],
                );
                let setting = e.mem.u32(work.addr() + 8);
                if setting != 0 {
                    let result = e.vcall(this.addr(), 0x10, &args![setting]).u8();
                    success &= result;
                }
            }
        });
    }
    success != 0
}

// Translated from 00405120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingCollectionMap<Setting>::ReadSettings` (Xbox PDB): read every
/// non-null setting while preserving the accumulated success byte.
// Translation incomplete: 00405120 - compiler SEH unwinding is not modeled.
pub fn setting_collection_map_read_settings(
    e: &mut Engine,
    this: Ptr<SettingCollectionMap>,
) -> bool {
    e.with_stack(4, |e, guard| {
        fn_00404eb0(e, guard, 9, true, Ptr::new(0x0101_2220), 0x278);
        let mut success = fn_00404d30(e, this) as u8;
        if success != 0 {
            e.with_stack(12, |e, work| {
                let map = this.byte_add(0x10c);
                let first = e.call(0x004b_9ba0, &args![map]).u32();
                e.mem.set_u32(work.addr(), first);
                while e.mem.u32(work.addr()) != 0 {
                    e.call(
                        0x006b_7f20,
                        &args![map, work, work.byte_add(4), work.byte_add(8)],
                    );
                    let setting = e.mem.u32(work.addr() + 8);
                    if setting != 0 {
                        let result = e.vcall(this.addr(), 0x14, &args![setting]).u8();
                        success &= result;
                    }
                }
            });
        }
        fn_00404ee0(e, guard);
        success != 0
    })
}

// Translated from 00405200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for `BSTCaseInsensitiveStringMap<Setting *>`
/// (Xbox PDB).
pub fn fn_00405200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00404da0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00405230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy the string-template map base.
pub fn fn_00405230(e: &mut Engine, this: Ptr) {
    fn_00405250(e, this);
}

// Translated from 00405250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Free map entry values from each bucket when the map owns copied entries.
// Translation incomplete: 00405250 - compiler SEH unwinding is not modeled.
pub fn fn_00405250(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2238);
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(this.addr() + 4) {
            let table = e.mem.u32(this.addr() + 8);
            let mut entry = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            while entry != 0 {
                let next = e.mem.u32(entry);
                let value = e.mem.u32(entry + 4);
                e.call(0x00aa_10f0, &args![value]);
                entry = next;
            }
            bucket = bucket.wrapping_add(1);
        }
    }
    fn_00405310(e, this);
}

// Translated from 00405310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy the `NiTMap<char const *,Setting *>` (Xbox PDB) base.
// Translation incomplete: 00405310 - compiler SEH unwinding is not modeled.
pub fn fn_00405310(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2258);
    e.call(0x0043_8af0, &args![this]);
    fn_00405370(e, this);
}

// Translated from 00405370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy the base map and free its hash table.
pub fn fn_00405370(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_2278);
    e.call(0x0043_8af0, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(0x00aa_10f0, &args![table]);
}

// Translated from 004053a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the `NiTMapBase` (Xbox PDB) specialization.
pub fn fn_004053a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00405370(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004053d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the `NiTMap` (Xbox PDB) specialization.
pub fn fn_004053d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00405310(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00405400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor for the `NiTStringTemplateMap` (Xbox PDB)
/// specialization.
pub fn fn_00405400(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00405250(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00403db0, effect_item_init_item(Ptr<EffectItem>, Ptr)),
        entry!(0x00403df0, fn_00403df0(Ptr) -> u32),
        entry!(0x00403e20, fn_00403e20(Ptr) -> Ptr),
        entry!(0x00403e60, fn_00403e60(Ptr, u32) -> bool),
        entry!(0x00403e80, fn_00403e80(Ptr) -> bool),
        entry!(0x00403ea0, fn_00403ea0(Ptr<EffectItem>) -> i32),
        entry!(0x00403ee0, fn_00403ee0(Ptr<EffectItem>) -> bool),
        entry!(0x00403f00, effect_item_get_raw_cost(Ptr<EffectItem>) -> f32),
        entry!(
            0x00403f80,
            effect_item_get_cost(Ptr<EffectItem>, Ptr) -> f32
        ),
        entry!(0x00404010, fn_00404010(f32, f32) -> f32),
        entry!(0x00404040, fn_00404040(f32) -> f32),
        entry!(0x00404090, fn_00404090(f32) -> f64),
        entry!(0x004040b0, effect_item_save(Ptr<EffectItem>)),
        entry!(
            0x00404120,
            effect_item_load(Ptr<EffectItem>, Ptr, Ptr) -> bool
        ),
        entry!(0x00404210, fn_00404210(Ptr<EffectItem>, Ptr<EffectItem>)),
        entry!(
            0x004042a0,
            fn_004042a0(Ptr<EffectItem>, Ptr<EffectItem>) -> bool
        ),
        entry!(
            0x00404390,
            effect_item_get_magnitude(Ptr<EffectItem>) -> i32
        ),
        entry!(0x004043d0, fn_004043d0(Ptr<EffectItem>, i32) -> bool),
        entry!(0x00404420, effect_item_get_area(Ptr<EffectItem>) -> i32),
        entry!(0x00404470, fn_00404470(Ptr<EffectItem>, i32) -> bool),
        entry!(0x004044d0, effect_item_get_duration(Ptr<EffectItem>) -> i32),
        entry!(0x00404510, fn_00404510(Ptr<EffectItem>, i32) -> bool),
        entry!(0x00404560, fn_00404560(Ptr<EffectItem>, u32) -> bool),
        entry!(0x004045e0, fn_004045e0(Ptr<EffectItem>) -> Ptr),
        entry!(
            0x00404650,
            effect_item_get_associated_skill(Ptr<EffectItem>) -> u32
        ),
        entry!(0x00404710, effect_item_is_hostile(Ptr<EffectItem>) -> bool),
        entry!(
            0x00404730,
            effect_item_is_usage_monitor_effect(Ptr<EffectItem>) -> bool
        ),
        entry!(
            0x00404670,
            effect_item_get_effect_name(Ptr<EffectItem>, Ptr) -> Ptr
        ),
        entry!(0x004046b0, fn_004046b0(Ptr, Ptr) -> Ptr),
        entry!(0x004046f0, fn_004046f0(Ptr, Ptr)),
        entry!(0x00404760, fn_00404760(Ptr<EffectItem>, Ptr)),
        entry!(0x004047f0, fn_004047f0(Ptr, Ptr) -> Ptr),
        entry!(0x00404820, bs_string_append(Ptr, Ptr) -> Ptr),
        entry!(0x004048e0, fn_004048e0(Ptr) -> u32),
        entry!(0x00404920, fn_00404920(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00404950, fn_00404950(Ptr)),
        entry!(0x004049a0, setting_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004049d0, setting_t_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00404a00, fn_00404a00(Ptr)),
        entry!(0x00404a70, fn_00404a70() -> Ptr),
        entry!(0x00404a80, fn_00404a80()),
        entry!(0x00404b10, fn_00404b10(Ptr) -> Ptr),
        entry!(0x00404b40, fn_00404b40(Ptr, u32) -> Ptr),
        entry!(0x00404b70, fn_00404b70(Ptr)),
        entry!(0x00404b90, fn_00404b90(Ptr, u32) -> Ptr),
        entry!(0x00404c00, fn_00404c00(Ptr)),
        entry!(0x00404c80, fn_00404c80(Ptr)),
        entry!(0x00404ca0, fn_00404ca0(Ptr, u32) -> Ptr),
        entry!(0x00404cd0, setting_collection_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00404d00, fn_00404d00(Ptr) -> Ptr),
        entry!(0x00404d30, fn_00404d30(Ptr<SettingCollectionMap>) -> bool),
        entry!(0x00404d50, fn_00404d50(Ptr, u32, bool) -> Ptr),
        entry!(0x00404d80, fn_00404d80(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00404da0, fn_00404da0(Ptr)),
        entry!(0x00404dc0, fn_00404dc0(Ptr, Ptr) -> i32),
        entry!(0x00404de0, setting_collection_map_add(Ptr, Ptr)),
        entry!(0x00404eb0, fn_00404eb0(Ptr, u32, bool, Ptr, u32) -> Ptr),
        entry!(0x00404ee0, fn_00404ee0(Ptr)),
        entry!(0x00404f00, fn_00404f00(Ptr, u32, bool, Ptr, u32)),
        entry!(0x00404f30, fn_00404f30(u32)),
        entry!(0x00404f50, fn_00404f50() -> u32),
        entry!(0x00404f70, fn_00404f70(Ptr)),
        entry!(0x00404f90, setting_collection_map_remove(Ptr, Ptr)),
        entry!(0x00404fd0, setting_collection_map_get_viewer_strings(Ptr, Ptr) -> u32),
        entry!(
            0x004050a0,
            setting_collection_map_write_settings(Ptr<SettingCollectionMap>) -> bool
        ),
        entry!(
            0x00405120,
            setting_collection_map_read_settings(Ptr<SettingCollectionMap>) -> bool
        ),
        entry!(0x00405200, fn_00405200(Ptr, u32) -> Ptr),
        entry!(0x00405230, fn_00405230(Ptr)),
        entry!(0x00405250, fn_00405250(Ptr)),
        entry!(0x00405310, fn_00405310(Ptr)),
        entry!(0x00405370, fn_00405370(Ptr)),
        entry!(0x004053a0, fn_004053a0(Ptr, u32) -> Ptr),
        entry!(0x004053d0, fn_004053d0(Ptr, u32) -> Ptr),
        entry!(0x00405400, fn_00405400(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }
    fn float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn setup() -> (Engine, Ptr<EffectItem>, Ptr) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x200);
        e.set_global(INVALID_COST, -1.0f32);
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_2070, 1.0f64);
        let item = e.new_object::<EffectItem>();
        let setting = Ptr::new(e.mem.alloc(0x100));
        e.set(item, EffectItem::pEffectSetting, setting);
        e.register(0x0058_cba0, |e, a| {
            word((e.mem.u32(a[0]) & a[1] != 0) as u32)
        });
        e.register(0x0084_e3a0, |e, a| word(e.mem.u32(a[0] + 0xc)));
        e.register(0x0082_5c00, |e, a| word(e.mem.u32(a[0] + 0x14)));
        e.register(0x00ec_62c0, |_, a| {
            let value = f64::from_bits(a[0] as u64 | ((a[1] as u64) << 32));
            word(value as i32 as u32)
        });
        (e, item, setting)
    }

    #[test]
    fn at_00403db0_initializes_only_unmarked_forms() {
        let (mut e, item, _) = setup();
        let form = Ptr::new(e.mem.alloc(0x10));
        e.register(0x0040_13e0, |e, a| {
            word(((e.mem.u32(a[0] + 8) & 8) != 0) as u32)
        });
        e.register(0x0040_9140, |_, a| word(a[0] + 0x1c));
        e.register(0x0068_0a20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        effect_item_init_item(&mut e, item, form);
        assert_eq!(
            e.call_log.as_ref().unwrap().last().unwrap(),
            &(0x0068_0a20, vec![item.addr() + 0x1c, form.addr()])
        );
        e.mem.set_u32(form.addr() + 8, 8);
        e.call_log = Some(vec![]);
        effect_item_init_item(&mut e, item, form);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn at_00403df0_reads_or_defaults() {
        let (mut e, _, _) = setup();
        let object = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(object.addr() + 4, 73);
        assert_eq!(fn_00403df0(&mut e, object), 73);
        assert_eq!(fn_00403df0(&mut e, Ptr::NULL), 0);
    }

    #[test]
    fn at_00403e20_null_zeros_the_fallback() {
        let (mut e, _, _) = setup();
        e.map(0x0120_2800, 4);
        e.set_global(0x0120_2800, 9.0f32);
        assert_eq!(fn_00403e20(&mut e, Ptr::NULL).addr(), 0x0120_2800);
        assert_eq!(e.global::<f32>(0x0120_2800), 0.0);
        let object = Ptr::new(e.mem.alloc(8));
        assert_eq!(fn_00403e20(&mut e, object).addr(), object.addr() + 4);
    }

    #[test]
    fn at_00403e60_forwards_mask_and_low_byte() {
        let (mut e, _, setting) = setup();
        e.mem.set_u32(setting.addr(), 0x100);
        assert!(fn_00403e60(&mut e, setting, 0x100));
        assert!(!fn_00403e60(&mut e, setting, 0x80));
    }

    #[test]
    fn at_00403e80_rejects_minus_one() {
        let (mut e, _, setting) = setup();
        e.register(0x0058_6150, |e, a| word(e.mem.u32(a[0] + 4)));
        e.mem.set_u32(setting.addr() + 4, u32::MAX);
        assert!(!fn_00403e80(&mut e, setting));
        e.mem.set_u32(setting.addr() + 4, 0);
        assert!(fn_00403e80(&mut e, setting));
    }

    #[test]
    fn at_00403ea0_returns_index_only_if_eligible() {
        let (mut e, item, setting) = setup();
        e.register(0x0040_3c60, |e, a| word(e.mem.u32(a[0])));
        e.set(item.at(EffectItem::data), EffectItemData::iActorValue, 17);
        assert_eq!(fn_00403ea0(&mut e, item), -1);
        e.mem.set_u32(setting.addr(), 1);
        assert_eq!(fn_00403ea0(&mut e, item), 17);
    }

    #[test]
    fn at_00403ee0_passes_setting_pointer() {
        let (mut e, item, setting) = setup();
        e.register_double(0x0040_3c60, move |_, a| {
            word((a == [setting.addr()]) as u32)
        });
        assert!(fn_00403ee0(&mut e, item));
    }

    #[test]
    fn at_00403f00_caches_calculated_cost() {
        let (mut e, item, setting) = setup();
        e.set(item, EffectItem::fRawCost, -1.0);
        e.mem.set_f32(setting.addr() + 0x5c, 3.0);
        e.set(item.at(EffectItem::data), EffectItemData::iRange, 2);
        e.set(item.at(EffectItem::data), EffectItemData::iMagnitude, 4);
        e.register(0x0063_9b20, |e, a| float(e.mem.f32(a[0] + 0x5c)));
        e.register(0x0064_90b0, |_, a| {
            assert_eq!(a, [3.0f32.to_bits(), 0, 0, 4, 1]);
            float(14.5)
        });
        assert_eq!(effect_item_get_raw_cost(&mut e, item), 14.5);
        e.call_log = Some(vec![]);
        assert_eq!(effect_item_get_raw_cost(&mut e, item), 14.5);
        assert!(e.call_log.unwrap().is_empty());
    }

    #[test]
    fn cached_cost_return_quiets_signaling_nan_without_changing_cache() {
        let (mut e, item, _) = setup();
        let signal = f32::from_bits(0x7f80_0123);
        e.set(item, EffectItem::fRawCost, signal);
        assert_eq!(
            effect_item_get_raw_cost(&mut e, item).to_bits(),
            0x7fc0_0123
        );
        assert_eq!(
            e.get(item, EffectItem::fRawCost).to_bits(),
            signal.to_bits()
        );
    }

    #[test]
    fn at_00403f80_adjusts_actor_cost_then_clamps() {
        let (mut e, item, _) = setup();
        e.set(item, EffectItem::fRawCost, 2.7);
        let actor = Ptr::new(e.mem.alloc(0x200));
        e.register(0x0066_ef20, |_, a| word(if a[1] == 11 { 40 } else { 30 }));
        e.register(0x0070_ec90, |_, _| word(24));
        e.register(0x0064_9180, |_, a| {
            assert_eq!(a, [2.7f32.to_bits(), 30, 40]);
            float(0.8)
        });
        assert_eq!(effect_item_get_cost(&mut e, item, actor), 1.0);
        e.set(item, EffectItem::fRawCost, 2.7);
        assert_eq!(effect_item_get_cost(&mut e, item, Ptr::NULL), 2.0);
    }

    #[test]
    fn at_00404010_quiets_selected_nan() {
        let mut e = Engine::new();
        assert_eq!(fn_00404010(&mut e, 2.0, 1.0), 2.0);
        let signal = f32::from_bits(0x7f80_0001);
        assert_eq!(fn_00404010(&mut e, 1.0, signal).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn at_00404040_negative_fraction_uses_exe_one() {
        let (mut e, _, _) = setup();
        assert_eq!(fn_00404040(&mut e, -2.25), -3.0);
        assert_eq!(fn_00404040(&mut e, 2.25), 2.0);
    }

    #[test]
    fn at_00404090_preserves_double_result() {
        let (mut e, _, _) = setup();
        assert_eq!(fn_00404090(&mut e, 16_777_218.0), 16_777_218.0f64);
    }

    /// Native FalloutNV.exe 1.4.0.525 x87 vectors, SHA-256
    /// 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f;
    /// FCW 0x027f / MXCSR 0x1f80 on Ryzen AI 9 HX 370. Private oracle:
    /// `work/phase0/codex-audit/effect-float-native-results.jsonl`.
    #[test]
    fn cost_float_helpers_match_native_edges() {
        let mut e = Engine::new(); // use the actual translated CRT `_ftol2`.
        e.map(0x0101_2000, 0x100);
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_2070, 1.0f64);
        assert_eq!(fn_00404040(&mut e, f32::INFINITY), 0.0);
        assert_eq!(fn_00404040(&mut e, f32::NEG_INFINITY), -1.0);
        assert_eq!(fn_00404040(&mut e, f32::NAN), -1.0);
        assert_eq!(fn_00404090(&mut e, -2_147_483_904.0), 2_147_483_392.0);
        assert_eq!(
            fn_00404010(&mut e, 0.0, -0.0).to_bits(),
            (-0.0f32).to_bits()
        );
        let signal = f32::from_bits(0xff80_0123);
        assert_eq!(fn_00404010(&mut e, 1.0, signal).to_bits(), 0xffc0_0123);
        // The globals are read at run time; an unordered zero comparison
        // takes the subtract branch and uses the current second constant.
        e.set_global(0x0101_2060, f64::NAN);
        e.set_global(0x0101_2070, 2.0f64);
        assert_eq!(fn_00404040(&mut e, 2.25), 0.0);
    }

    #[test]
    fn at_00404390_hides_flagged_magnitude() {
        let (mut e, item, setting) = setup();
        e.set(item.at(EffectItem::data), EffectItemData::iMagnitude, 22);
        assert_eq!(effect_item_get_magnitude(&mut e, item), 22);
        e.mem.set_u32(setting.addr(), 0x100);
        assert_eq!(effect_item_get_magnitude(&mut e, item), 0);
    }

    #[test]
    fn at_004043d0_validates_and_invalidates() {
        let (mut e, item, setting) = setup();
        assert!(!fn_004043d0(&mut e, item, -1));
        assert!(fn_004043d0(&mut e, item, 5));
        assert_eq!(e.get(item, EffectItem::fRawCost), -1.0);
        e.mem.set_u32(setting.addr(), 0x100);
        assert!(!fn_004043d0(&mut e, item, 7));
        assert_eq!(
            e.get(item.at(EffectItem::data), EffectItemData::iMagnitude),
            5
        );
    }

    #[test]
    fn invalidation_quiets_signaling_nan_like_x87_store() {
        let (mut e, item, _) = setup();
        e.set_global(INVALID_COST, f32::from_bits(0x7f80_0123));
        assert!(fn_004043d0(&mut e, item, 5));
        assert_eq!(e.get(item, EffectItem::fRawCost).to_bits(), 0x7fc0_0123);
    }

    #[test]
    fn at_00404420_area_requires_nonself_range() {
        let (mut e, item, setting) = setup();
        e.set(item.at(EffectItem::data), EffectItemData::iArea, 9);
        assert_eq!(effect_item_get_area(&mut e, item), 0);
        e.set(item.at(EffectItem::data), EffectItemData::iRange, 1);
        assert_eq!(effect_item_get_area(&mut e, item), 9);
        e.mem.set_u32(setting.addr(), 0x200);
        assert_eq!(effect_item_get_area(&mut e, item), 0);
    }

    #[test]
    fn at_00404470_area_set_requires_nonself_range() {
        let (mut e, item, setting) = setup();
        assert!(!fn_00404470(&mut e, item, 9));
        e.set(item.at(EffectItem::data), EffectItemData::iRange, 1);
        assert!(fn_00404470(&mut e, item, 9));
        e.mem.set_u32(setting.addr(), 0x200);
        assert!(!fn_00404470(&mut e, item, 10));
    }

    #[test]
    fn at_004044d0_hides_flagged_duration() {
        let (mut e, item, setting) = setup();
        e.set(item.at(EffectItem::data), EffectItemData::iDuration, 12);
        assert_eq!(effect_item_get_duration(&mut e, item), 12);
        e.mem.set_u32(setting.addr(), 0x80);
        assert_eq!(effect_item_get_duration(&mut e, item), 0);
    }

    #[test]
    fn at_00404510_duration_set_rejects_negative_and_flagged() {
        let (mut e, item, setting) = setup();
        assert!(!fn_00404510(&mut e, item, -1));
        assert!(fn_00404510(&mut e, item, 12));
        e.mem.set_u32(setting.addr(), 0x80);
        assert!(!fn_00404510(&mut e, item, 13));
    }

    #[test]
    fn at_00404560_range_uses_respective_flag() {
        let (mut e, item, setting) = setup();
        e.mem.set_u32(setting.addr(), 0x30);
        assert!(fn_00404560(&mut e, item, 0));
        assert!(fn_00404560(&mut e, item, 1));
        assert!(!fn_00404560(&mut e, item, 2));
        assert!(!fn_00404560(&mut e, item, 3));
    }

    #[test]
    fn at_00404650_forwards_setting() {
        let (mut e, item, setting) = setup();
        e.register_double(0x0070_ec90, move |_, a| {
            assert_eq!(a, [setting.addr()]);
            word(24)
        });
        assert_eq!(effect_item_get_associated_skill(&mut e, item), 24);
    }

    #[test]
    fn at_00404710_checks_flag_one() {
        let (mut e, item, setting) = setup();
        assert!(!effect_item_is_hostile(&mut e, item));
        e.mem.set_u32(setting.addr(), 1);
        assert!(effect_item_is_hostile(&mut e, item));
    }

    #[test]
    fn at_00404730_matches_monitor_pointer() {
        let (mut e, item, setting) = setup();
        e.register_double(0x0040_8b50, move |_, _| word(setting.addr()));
        assert!(effect_item_is_usage_monitor_effect(&mut e, item));
    }

    #[test]
    fn at_004040b0_writes_efit_and_conditions() {
        let (mut e, item, _) = setup();
        e.register(0x0040_1500, |_, _| word(0));
        e.register(0x0048_5990, |_, _| Ret::default());
        e.register(0x0040_9140, |_, a| word(a[0] + 0x1c));
        e.register(0x0068_0910, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        effect_item_save(&mut e, item);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0048_5990, vec![0x5449_4645, item.addr(), 0x14])));
        e.register(0x0040_1500, |_, _| word(1));
        e.register(0x0058_e3a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        effect_item_save(&mut e, item);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x0058_e3a0)
                .count(),
            2
        );
    }

    #[test]
    fn at_00404120_loads_efit_and_rejects_unknown_chunks() {
        let (mut e, item, _) = setup();
        let file = Ptr::new(e.mem.alloc(0x20));
        e.register(0x0047_26b0, |e, a| word(e.mem.u32(a[0])));
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.register(0x0047_2890, |_, _| Ret::default());
        e.register(0x0040_1680, |_, _| word(0));
        e.register(0x0040_3c60, |_, _| word(1));
        e.register(0x0058_6150, |_, _| word(17));
        e.mem.set_u32(file.addr(), 0x5449_4645);
        assert!(effect_item_load(&mut e, item, file, Ptr::NULL));
        assert_eq!(
            e.get(item.at(EffectItem::data), EffectItemData::iActorValue),
            17
        );
        e.mem.set_u32(file.addr(), 0x1234);
        assert!(!effect_item_load(&mut e, item, file, Ptr::NULL));
        e.register(0x0040_9140, |_, a| word(a[0] + 0x1c));
        e.register(0x0068_0960, |_, _| Ret::default());
        e.mem.set_u32(file.addr(), 0x4144_5443);
        e.call_log = Some(vec![]);
        assert!(effect_item_load(&mut e, item, file, Ptr::NULL));
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0068_0960, vec![item.addr() + 0x1c, file.addr()])));
        e.set(item, EffectItem::pEffectSetting, Ptr::NULL);
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.mem.set_u32(file.addr(), 0x5449_4645);
        e.call_log = Some(vec![]);
        assert!(effect_item_load(&mut e, item, file, Ptr::NULL));
        assert!(e
            .call_log
            .unwrap()
            .contains(&(0x005b_5e40, vec![0x0101_2078, 0x0101_20d8])));
    }

    #[test]
    fn load_reloads_setting_after_each_callback() {
        let (mut e, item, first) = setup();
        let second = Ptr::<()>::new(e.mem.alloc(0x100));
        let third = Ptr::<()>::new(e.mem.alloc(0x100));
        let file = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(file.addr(), 0x5449_4645);
        e.register(0x0047_26b0, |_, _| word(0x5449_4645));
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.register(0x0047_2890, |_, _| Ret::default());
        e.register(0x0040_1680, |_, _| word(0));
        e.register_double(0x0040_3c60, move |e, a| {
            assert_eq!(a, [first.addr()]);
            e.mem.set_u32(item.addr() + 0x14, second.addr());
            word(1)
        });
        e.register_double(0x0058_6150, move |e, a| {
            if a == [second.addr()] {
                e.mem.set_u32(item.addr() + 0x14, third.addr());
                word(3)
            } else {
                assert_eq!(a, [third.addr()]);
                word(19)
            }
        });
        assert!(effect_item_load(&mut e, item, file, Ptr::NULL));
        assert_eq!(
            e.get(item.at(EffectItem::data), EffectItemData::iActorValue),
            19
        );
    }

    #[test]
    fn at_00404210_copies_validated_fields() {
        let (mut e, item, setting) = setup();
        let source = e.new_object::<EffectItem>();
        e.set(source, EffectItem::pEffectSetting, setting);
        e.mem.set_u32(setting.addr(), 0x30);
        e.set(source.at(EffectItem::data), EffectItemData::iMagnitude, 8);
        e.set(source.at(EffectItem::data), EffectItemData::iDuration, 9);
        e.set(source.at(EffectItem::data), EffectItemData::iRange, 1);
        e.register(0x0040_3c60, |_, _| word(0));
        e.register(0x0040_9140, |_, a| word(a[0] + 0x1c));
        e.register(0x0043_7730, |_, _| Ret::default());
        e.register(0x0068_0a70, |_, _| Ret::default());
        fn_00404210(&mut e, item, source);
        assert_eq!(
            e.get(item.at(EffectItem::data), EffectItemData::iMagnitude),
            8
        );
        assert_eq!(
            e.get(item.at(EffectItem::data), EffectItemData::iDuration),
            9
        );
        assert_eq!(e.get(item, EffectItem::fRawCost), -1.0);
    }

    #[test]
    fn at_004042a0_compares_values_and_conditions() {
        let (mut e, item, setting) = setup();
        let other = e.new_object::<EffectItem>();
        e.set(other, EffectItem::pEffectSetting, setting);
        e.register(0x0040_3c60, |_, _| word(0));
        e.register(0x0040_9140, |_, a| word(a[0] + 0x1c));
        e.register(0x0068_0b70, |_, _| word(0));
        assert!(!fn_004042a0(&mut e, item, other));
        e.set(other.at(EffectItem::data), EffectItemData::iMagnitude, 1);
        assert!(fn_004042a0(&mut e, item, other));
        e.set(other.at(EffectItem::data), EffectItemData::iMagnitude, 0);
        e.register(0x0068_0b70, |_, _| word(1));
        assert!(fn_004042a0(&mut e, item, other));
    }

    #[test]
    fn at_004045e0_requires_flag_and_type() {
        let (mut e, item, setting) = setup();
        e.register(0x0081_0fd0, |_, _| word(3));
        e.register(0x0040_3c80, |e, _| word(e.mem.u32(0x0120_28f0)));
        e.register(0x005f_5f80, |_, _| word(0x3300));
        e.register(0x0040_1170, |_, _| word(0x11));
        e.map(0x0120_28f0, 4);
        assert_eq!(fn_004045e0(&mut e, item).addr(), 0);
        e.mem.set_u32(0x0120_28f0, 1);
        assert_eq!(fn_004045e0(&mut e, item).addr(), 0x3300);
        e.call_log = Some(vec![]);
        let _ = fn_004045e0(&mut e, item);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x0081_0fd0, vec![setting.addr()])));
        e.register(0x0040_1170, |_, _| word(0x10));
        assert_eq!(fn_004045e0(&mut e, item).addr(), 0);
    }

    fn string_doubles(e: &mut Engine) {
        e.register(0x0071_7e50, |_, a| word(a[0] + 4));
        e.register(0x0040_3970, |_, _| Ret::default());
        e.register(0x0055_9450, |_, a| word(a[0] + 8));
        e.register(0x0040_37f0, |_, _| Ret::default());
    }

    #[test]
    fn at_00404670_constructs_effect_name() {
        let (mut e, item, setting) = setup();
        string_doubles(&mut e);
        let output = Ptr::new(e.mem.alloc(8));
        assert_eq!(effect_item_get_effect_name(&mut e, item, output), output);
        e.call_log = Some(vec![]);
        effect_item_get_effect_name(&mut e, item, output);
        assert!(e
            .call_log
            .unwrap()
            .contains(&(0x0071_7e50, vec![setting.addr() + 0x38])));
    }

    #[test]
    fn at_004046b0_gets_name_from_setting_field() {
        let (mut e, _, setting) = setup();
        string_doubles(&mut e);
        let output = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert_eq!(fn_004046b0(&mut e, setting, output), output);
        assert!(e
            .call_log
            .unwrap()
            .contains(&(0x0040_37f0, vec![output.addr(), setting.addr() + 0x44, 0])));
    }

    #[test]
    fn at_004046f0_forwards_copy_words() {
        let (mut e, _, _) = setup();
        e.register(0x00ec_6370, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_004046f0(&mut e, Ptr::new(11), Ptr::new(22));
        assert_eq!(e.call_log.unwrap(), vec![(0x00ec_6370, vec![11, 22])]);
    }

    #[test]
    fn at_00404760_assigns_existing_condition_list() {
        let (mut e, item, _) = setup();
        e.register(0x0068_0a70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00404760(&mut e, item, Ptr::new(0x3300));
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x0068_0a70, vec![item.addr() + 0x1c, 0x3300, 0])]
        );
        e.register(0x0068_0890, |_, _| Ret::default());
        e.register(0x0068_08b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00404760(&mut e, item, Ptr::NULL);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .collect::<Vec<_>>(),
            vec![0x0068_0890, 0x0068_0a70, 0x0068_08b0]
        );
    }

    #[test]
    fn at_004047f0_constructs_then_assigns_string() {
        let (mut e, _, _) = setup();
        string_doubles(&mut e);
        e.call_log = Some(vec![]);
        let output = Ptr::new(0x3400);
        assert_eq!(fn_004047f0(&mut e, output, Ptr::new(0x3500)), output);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .collect::<Vec<_>>(),
            vec![0x0040_3970, 0x0055_9450, 0x0040_37f0]
        );
    }

    #[test]
    fn at_00404820_appends_and_updates_length() {
        let (mut e, _, _) = setup();
        let this = Ptr::new(e.mem.alloc(8));
        let chars = e.mem.alloc(16);
        e.mem.set_u32(this.addr(), chars);
        e.mem.set_u16(this.addr() + 4, 2);
        e.mem.set_u16(this.addr() + 6, 10);
        e.register(0x0055_9450, |e, a| word(e.mem.u32(a[0])));
        e.register(0x0044_a670, |_, _| word(3));
        e.register(0x0040_39a0, |e, a| word(e.mem.u16(a[0] + 6) as u32));
        e.register(0x0040_1460, |_, _| Ret::default());
        e.register(0x0040_39c0, |e, a| {
            e.mem.set_u16(a[0] + 4, a[1] as u16);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(bs_string_append(&mut e, this, Ptr::new(0x3500)), this);
        assert_eq!(e.mem.u16(this.addr() + 4), 5);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0040_1460, vec![chars + 2, 0x3500, 4])));
        e.mem.set_u16(this.addr() + 6, 0);
        e.register(0x0040_37f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        bs_string_append(&mut e, this, Ptr::new(0x3500));
        assert!(e
            .call_log
            .unwrap()
            .contains(&(0x0040_37f0, vec![this.addr(), chars, 8])));
    }

    #[test]
    fn at_004048e0_uses_cached_or_computed_length() {
        let (mut e, _, _) = setup();
        let this = Ptr::new(e.mem.alloc(8));
        e.mem.set_u16(this.addr() + 4, 7);
        assert_eq!(fn_004048e0(&mut e, this), 7);
        e.mem.set_u16(this.addr() + 4, 0xffff);
        e.mem.set_u32(this.addr(), 0x3600);
        e.register(0x0044_a670, |_, a| {
            word(if a[0] == 0x3600 { 13 } else { 0 })
        });
        assert_eq!(fn_004048e0(&mut e, this), 13);
    }

    #[test]
    fn at_00404920_stores_vtable_value_and_key() {
        let (mut e, _, _) = setup();
        let this = Ptr::new(e.mem.alloc(12));
        assert_eq!(fn_00404920(&mut e, this, Ptr::new(0x3700), 99), this);
        assert_eq!(
            (
                e.mem.u32(this.addr()),
                e.mem.u32(this.addr() + 4),
                e.mem.u32(this.addr() + 8)
            ),
            (0x0101_2120, 99, 0x3700)
        );
    }

    #[test]
    fn at_00404950_frees_only_s_prefixed_pointer() {
        let (mut e, _, _) = setup();
        let this = Ptr::new(e.mem.alloc(12));
        let key = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 8, key);
        e.register(0x0040_1030, |_, _| Ret::default());
        e.mem.set_u8(key, b'X');
        e.call_log = Some(vec![]);
        fn_00404950(&mut e, this);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.mem.set_u8(key, b'S');
        fn_00404950(&mut e, this);
        assert_eq!(e.call_log.unwrap(), vec![(0x0040_1030, vec![key])]);
    }

    #[test]
    fn at_004049a0_deletes_only_under_flag_one() {
        let (mut e, _, _) = setup();
        let this = Ptr::new(e.mem.alloc(12));
        e.register(0x0040_1030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        setting_scalar_deleting_destructor(&mut e, this, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        setting_scalar_deleting_destructor(&mut e, this, 1);
        assert_eq!(e.call_log.unwrap(), vec![(0x0040_1030, vec![this.addr()])]);
    }

    fn collection_doubles(e: &mut Engine) -> u32 {
        let collection = e.mem.alloc(4);
        e.put_vtable(0x0101_2300, &[0, 0, 0x00ab_cdef]);
        e.mem.set_u32(collection, 0x0101_2300);
        e.map(0x011c_8048, 4);
        e.set_global(0x011c_8048, collection);
        e.register(0x0040_4a80, |_, _| Ret::default());
        e.register(0x00ab_cdef, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        collection
    }

    #[test]
    fn at_004049d0_removes_then_optionally_deletes_setting() {
        let (mut e, _, _) = setup();
        let collection = collection_doubles(&mut e);
        let this = Ptr::new(e.mem.alloc(12));
        e.call_log = Some(vec![]);
        setting_t_scalar_deleting_destructor(&mut e, this, 1);
        assert_eq!(
            e.call_log.unwrap().last().unwrap(),
            &(0x0040_1030, vec![this.addr()])
        );
        assert_eq!(e.mem.u32(collection), 0x0101_2300);
    }

    #[test]
    fn at_00404a00_invokes_collection_remove_slot() {
        let (mut e, _, _) = setup();
        let collection = collection_doubles(&mut e);
        let this = Ptr::new(e.mem.alloc(12));
        e.call_log = Some(vec![]);
        fn_00404a00(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), 0x0101_2120);
        assert!(e
            .call_log
            .unwrap()
            .contains(&(0x00ab_cdef, vec![collection, this.addr()])));
    }

    #[test]
    fn at_00404a70_reads_singleton_after_initializer() {
        let (mut e, _, _) = setup();
        let collection = collection_doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00404a70(&mut e).addr(), collection);
        assert_eq!(e.call_log.unwrap(), vec![(0x0040_4a80, vec![])]);
    }

    fn setting_fixture() -> (Engine, Ptr<SettingCollectionMap>) {
        let mut e = Engine::new();
        let collection = e.new_object::<SettingCollectionMap>();
        (e, collection)
    }

    #[test]
    fn at_00404a80_initializes_once_and_handles_failed_allocation() {
        let (mut e, collection) = setting_fixture();
        e.map(0x011c_8048, 4);
        e.map(0x0118_9c40, 4);
        e.set_global(0x0118_9c40, 13u32);
        e.register(0x0040_5570, |_, _| Ret::default());
        e.register_double(0x0040_1000, move |_, a| {
            assert_eq!(a, [0x120]);
            word(collection.addr())
        });
        fn_00404a80(&mut e);
        assert_eq!(e.global::<u32>(0x011c_8048), collection.addr());
        assert_eq!(e.mem.u32(collection.addr()), 0x0101_212c);
        e.call_log = Some(vec![]);
        fn_00404a80(&mut e);
        assert!(e.call_log.take().unwrap().is_empty());
        e.set_global(0x011c_8048, 0u32);
        e.register(0x0040_1000, |_, _| word(0));
        fn_00404a80(&mut e);
        assert_eq!(e.global::<u32>(0x011c_8048), 0);
    }

    #[test]
    fn at_00404b10_reads_current_map_size_and_installs_vtable() {
        let (mut e, collection) = setting_fixture();
        e.map(0x0118_9c40, 4);
        e.set_global(0x0118_9c40, 29u32);
        e.register(0x0040_5570, |_, a| {
            assert_eq!(a[1..], [29, 0]);
            Ret::default()
        });
        assert_eq!(fn_00404b10(&mut e, collection.cast()), collection.cast());
        assert_eq!(e.mem.u32(collection.addr()), 0x0101_212c);
    }

    fn destruction_doubles(e: &mut Engine) {
        e.register(0x0043_8af0, |_, _| Ret::default());
        e.register(0x008d_0360, |_, _| Ret::default());
        e.register(0x00aa_10f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
    }

    #[test]
    fn at_00404b40_deletes_game_collection_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let this = collection.cast();
        assert_eq!(fn_00404b40(&mut e, this, 2), this);
        e.call_log = Some(vec![]);
        fn_00404b40(&mut e, this, 1);
        assert_eq!(
            e.call_log.take().unwrap().last(),
            Some(&(0x0040_1030, vec![this.addr()]))
        );
    }

    #[test]
    fn at_00404b70_forwards_to_collection_destructor() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        fn_00404b70(&mut e, collection.cast());
        assert_eq!(e.mem.u32(collection.addr()), 0x0101_2188);
    }

    #[test]
    fn at_00404b90_constructs_map_at_pc_offset() {
        let (mut e, collection) = setting_fixture();
        e.register(0x0040_5570, |_, a| {
            assert_eq!(a[1..], [7, 0]);
            Ret::default()
        });
        assert_eq!(
            fn_00404b90(&mut e, collection.cast(), 7).addr(),
            collection.addr()
        );
        assert_eq!(e.mem.u32(collection.addr()), 0x0101_215c);
        assert_eq!(e.mem.u32(collection.addr() + 0x10c), 0x0101_21b4);
    }

    #[test]
    fn at_00404c00_destroys_embedded_map_before_base() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        e.call_log = Some(vec![]);
        fn_00404c00(&mut e, collection.cast());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[0], (0x0043_8af0, vec![collection.addr() + 0x10c]));
        assert_eq!(log.last(), Some(&(0x008d_0360, vec![collection.addr()])));
    }

    #[test]
    fn at_00404c80_restores_base_vtable_before_close() {
        let (mut e, collection) = setting_fixture();
        e.register(0x008d_0360, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x0101_2188);
            Ret::default()
        });
        fn_00404c80(&mut e, collection.cast());
    }

    #[test]
    fn at_00404ca0_deletes_map_collection_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let this = collection.cast();
        e.call_log = Some(vec![]);
        assert_eq!(fn_00404ca0(&mut e, this, 1), this);
        assert_eq!(
            e.call_log.take().unwrap().last(),
            Some(&(0x0040_1030, vec![this.addr()]))
        );
    }

    #[test]
    fn at_00404cd0_deletes_base_collection_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let this = collection.cast();
        e.call_log = Some(vec![]);
        assert_eq!(
            setting_collection_scalar_deleting_destructor(&mut e, this, 0),
            this
        );
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x008d_0360, vec![this.addr()])]
        );
    }

    #[test]
    fn at_00404d00_initializes_only_base_fields() {
        let (mut e, collection) = setting_fixture();
        let this = collection.cast();
        e.mem.set_u32(this.addr() + 0x108, 17);
        e.mem.set_u32(this.addr() + 4, 0xdead_beef);
        assert_eq!(fn_00404d00(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr() + 0x108), 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0xdead_be00);
    }

    #[test]
    fn at_00404d30_tests_handle_word() {
        let (mut e, collection) = setting_fixture();
        assert!(!fn_00404d30(&mut e, collection));
        e.set(collection, SettingCollectionMap::pHandle, Ptr::new(1));
        assert!(fn_00404d30(&mut e, collection));
    }

    #[test]
    fn at_00404d50_forwards_both_constructor_arguments() {
        let (mut e, collection) = setting_fixture();
        e.register(0x0040_5570, |_, a| {
            assert_eq!(a[1..], [37, 1]);
            Ret::default()
        });
        let map = collection.byte_add(0x10c);
        assert_eq!(fn_00404d50(&mut e, map, 37, true), map);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_21b4);
    }

    #[test]
    fn at_00404d80_forwards_three_crt_words() {
        let (mut e, collection) = setting_fixture();
        let dst = collection.cast();
        e.register(0x00ec_65a6, |_, a| {
            assert_eq!(a[1..], [20, 0x1234]);
            Ret::default()
        });
        assert_eq!(fn_00404d80(&mut e, dst, 20, Ptr::new(0x1234)), dst);
    }

    #[test]
    fn at_00404da0_destroys_string_map_base() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        fn_00404da0(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_2278);
    }

    #[test]
    fn at_00404dc0_returns_crt_comparison_result() {
        let (mut e, _, _) = setup();
        e.register(0x00ec_68e4, |_, a| {
            assert_eq!(a, [0x1111, 0x2222]);
            word((-3i32) as u32)
        });
        assert_eq!(fn_00404dc0(&mut e, Ptr::new(0x1111), Ptr::new(0x2222)), -3);
    }

    #[test]
    fn at_00404de0_adds_new_key_and_warns_on_duplicate() {
        let (mut e, collection) = setting_fixture();
        let setting = Ptr::new(e.mem.alloc(12));
        let key = e.mem.alloc(8);
        e.mem.set_u32(setting.addr() + 8, key);
        e.register(0x0044_ddc0, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(0x0085_3130, |_, _| word(0));
        e.register(0x0047_0200, |_, _| word(1));
        e.register(0x00c3_35a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        setting_collection_map_add(&mut e, collection.cast(), setting);
        assert!(e.call_log.take().unwrap().contains(&(
            0x0047_0200,
            vec![collection.addr() + 0x10c, key, setting.addr()]
        )));
        e.register(0x0085_3130, |_, _| word(1));
        e.call_log = Some(vec![]);
        setting_collection_map_add(&mut e, collection.cast(), setting);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x00c3_35a0, vec![0x0101_21d0, key])));
    }

    #[test]
    fn at_00404eb0_constructs_guard_with_only_first_argument_used() {
        let (mut e, _, _) = setup();
        fn_00404f30(&mut e, 31);
        let guard = Ptr::new(e.mem.alloc(4));
        assert_eq!(
            fn_00404eb0(&mut e, guard, 9, true, Ptr::new(0x1234), 44),
            guard
        );
        assert_eq!(e.mem.u32(guard.addr()), 31);
        assert_eq!(fn_00404f50(&mut e), 9);
    }

    #[test]
    fn at_00404ee0_restores_saved_guard() {
        let (mut e, _, _) = setup();
        let guard = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(guard.addr(), 47);
        fn_00404ee0(&mut e, guard);
        assert_eq!(fn_00404f50(&mut e), 47);
    }

    #[test]
    fn at_00404f00_saves_tls_value_and_replaces_it() {
        let (mut e, _, _) = setup();
        let guard = Ptr::new(e.mem.alloc(4));
        fn_00404f30(&mut e, 5);
        fn_00404f00(&mut e, guard, 11, false, Ptr::NULL, 0);
        assert_eq!(e.mem.u32(guard.addr()), 5);
        assert_eq!(fn_00404f50(&mut e), 11);
    }

    #[test]
    fn at_00404f30_writes_tls_at_pc_offset() {
        let (mut e, _, _) = setup();
        fn_00404f30(&mut e, 0x8765_4321);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x2b4), 0x8765_4321);
    }

    #[test]
    fn at_00404f50_reads_tls_at_pc_offset() {
        let (mut e, _, _) = setup();
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 0xabcd_ef01);
        assert_eq!(fn_00404f50(&mut e), 0xabcd_ef01);
    }

    #[test]
    fn at_00404f70_restores_from_guard_word() {
        let (mut e, _, _) = setup();
        let guard = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(guard.addr(), 55);
        fn_00404f70(&mut e, guard);
        assert_eq!(fn_00404f50(&mut e), 55);
    }

    #[test]
    fn at_00404f90_removes_only_present_key() {
        let (mut e, collection) = setting_fixture();
        let setting = Ptr::new(e.mem.alloc(12));
        e.register(0x0044_ddc0, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(0x0040_5430, |_, _| word(1));
        e.call_log = Some(vec![]);
        setting_collection_map_remove(&mut e, collection.cast(), Ptr::NULL);
        setting_collection_map_remove(&mut e, collection.cast(), setting);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(addr, _)| *addr != 0x0040_5430));
        e.mem.set_u32(setting.addr() + 8, 0x5678);
        e.call_log = Some(vec![]);
        setting_collection_map_remove(&mut e, collection.cast(), setting);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0040_5430, vec![collection.addr() + 0x10c, 0x5678])));
    }

    fn one_map_entry(e: &mut Engine, setting: u32) {
        e.register(0x004b_9ba0, |_, _| word(1));
        e.register_double(0x006b_7f20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], setting);
            Ret::default()
        });
    }

    #[test]
    fn at_00404fd0_counts_only_non_null_settings() {
        let (mut e, collection) = setting_fixture();
        one_map_entry(&mut e, 0x3333);
        e.register(0x00c3_3280, |_, a| {
            assert_eq!(a, [0x3333, 0x4444]);
            Ret::default()
        });
        let count =
            setting_collection_map_get_viewer_strings(&mut e, collection.cast(), Ptr::new(0x4444));
        assert_eq!(count, 1);
        one_map_entry(&mut e, 0);
        assert_eq!(
            setting_collection_map_get_viewer_strings(&mut e, collection.cast(), Ptr::new(0x4444),),
            0
        );
    }

    fn write_read_fixture(slot: u32) -> (Engine, Ptr<SettingCollectionMap>) {
        let (mut e, collection) = setting_fixture();
        e.set(collection, SettingCollectionMap::pHandle, Ptr::new(1));
        one_map_entry(&mut e, 0x3333);
        let mut slots = [0u32; 6];
        slots[(slot / 4) as usize] = 0x00ab_cdef;
        e.put_vtable(0x0101_2400, &slots);
        e.mem.set_u32(collection.addr(), 0x0101_2400);
        e.register(0x00ab_cdef, |_, a| {
            assert_eq!(a[1], 0x3333);
            word(0)
        });
        (e, collection)
    }

    #[test]
    fn at_004050a0_accumulates_virtual_write_result() {
        let (mut e, collection) = write_read_fixture(0x10);
        assert!(!setting_collection_map_write_settings(&mut e, collection));
        let mut visited = 0u32;
        e.register_double(0x006b_7f20, move |e, a| {
            visited += 1;
            e.mem.set_u32(a[1], if visited == 1 { 2 } else { 0 });
            e.mem
                .set_u32(a[3], if visited == 1 { 0x3333 } else { 0x4444 });
            Ret::default()
        });
        e.register(0x00ab_cdef, |_, a| word((a[1] == 0x4444) as u32));
        e.call_log = Some(vec![]);
        assert!(!setting_collection_map_write_settings(&mut e, collection));
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .filter(|(addr, _)| *addr == 0x00ab_cdef)
                .count(),
            2
        );
        e.set(collection, SettingCollectionMap::pHandle, Ptr::NULL);
        e.call_log = Some(vec![]);
        assert!(!setting_collection_map_write_settings(&mut e, collection));
        assert!(e.call_log.take().unwrap().is_empty());
    }

    #[test]
    fn at_00405120_accumulates_virtual_read_result_and_restores_guard() {
        let (mut e, collection) = write_read_fixture(0x14);
        fn_00404f30(&mut e, 41);
        assert!(!setting_collection_map_read_settings(&mut e, collection));
        assert_eq!(fn_00404f50(&mut e), 41);
        let mut visited = 0u32;
        e.register_double(0x006b_7f20, move |e, a| {
            visited += 1;
            e.mem.set_u32(a[1], if visited == 1 { 2 } else { 0 });
            e.mem
                .set_u32(a[3], if visited == 1 { 0x3333 } else { 0x4444 });
            Ret::default()
        });
        e.register(0x00ab_cdef, |_, a| word((a[1] == 0x4444) as u32));
        e.call_log = Some(vec![]);
        assert!(!setting_collection_map_read_settings(&mut e, collection));
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .filter(|(addr, _)| *addr == 0x00ab_cdef)
                .count(),
            2
        );
        assert_eq!(fn_00404f50(&mut e), 41);
    }

    #[test]
    fn at_00405200_deletes_string_map_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00405200(&mut e, map, 1), map);
        assert_eq!(
            e.call_log.take().unwrap().last(),
            Some(&(0x0040_1030, vec![map.addr()]))
        );
    }

    #[test]
    fn at_00405230_forwards_to_template_map_destructor() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        fn_00405230(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_2278);
    }

    #[test]
    fn at_00405250_frees_copied_entry_values_in_bucket_order() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        let table = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(table, first);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x1111);
        e.mem.set_u32(second + 4, 0x2222);
        e.mem.set_u32(map.addr() + 4, 2);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u8(map.addr() + 0x10, 1);
        e.call_log = Some(vec![]);
        fn_00405250(&mut e, map);
        let values: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(addr, _)| *addr == 0x00aa_10f0)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(values, vec![0x1111, 0x2222, table]);
    }

    #[test]
    fn at_00405310_calls_base_cleanup_before_hash_table_cleanup() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        e.call_log = Some(vec![]);
        fn_00405310(&mut e, map);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0043_8af0, vec![map.addr()]),
                (0x0043_8af0, vec![map.addr()]),
                (0x00aa_10f0, vec![0]),
            ]
        );
    }

    #[test]
    fn at_00405370_reads_hash_table_after_callback() {
        let (mut e, collection) = setting_fixture();
        let map = collection.byte_add(0x10c);
        e.register(0x0043_8af0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x9999);
            Ret::default()
        });
        e.register(0x00aa_10f0, |_, a| {
            assert_eq!(a, [0x9999]);
            Ret::default()
        });
        fn_00405370(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_2278);
    }

    #[test]
    fn at_004053a0_deletes_map_base_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        e.call_log = Some(vec![]);
        assert_eq!(fn_004053a0(&mut e, map, 2), map);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|(addr, args)| *addr == 0x0040_1030 && args == &[map.addr()]));
    }

    #[test]
    fn at_004053d0_deletes_ni_map_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        e.call_log = Some(vec![]);
        assert_eq!(fn_004053d0(&mut e, map, 1), map);
        assert_eq!(
            e.call_log.take().unwrap().last(),
            Some(&(0x0040_1030, vec![map.addr()]))
        );
    }

    #[test]
    fn at_00405400_deletes_template_map_only_with_low_flag_bit() {
        let (mut e, collection) = setting_fixture();
        destruction_doubles(&mut e);
        let map = collection.byte_add(0x10c);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00405400(&mut e, map, 1), map);
        assert_eq!(
            e.call_log.take().unwrap().last(),
            Some(&(0x0040_1030, vec![map.addr()]))
        );
    }
}

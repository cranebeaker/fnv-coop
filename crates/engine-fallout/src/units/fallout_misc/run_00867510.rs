//! `fallout/misc/run_00867510` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of small functions between `appmemory.cpp` and `calendar.cpp`:
//!
//! - `00867510`, `008675e0`: the constructor and the copy of a 0x48-byte
//!   record of floats and integers (the block `MiddleHighProcess`'s
//!   constructor, `00913fe0`, allocates and keeps at `+0x2c`; its class
//!   name is unconfirmed);
//! - `008676d0`: calls sixteen virtual slots of an object in turn (the
//!   actor calls it on its `CachedValuesOwner` at `+0xa8`);
//! - `008677d0`, `00867840`, `00867890`: the constructor of the `Calendar`
//!   (declared in `calendar.rs`), the clearing of its six `TESGlobal`
//!   pointers and a method that looks the six globals up by form number and
//!   folds the hour into the days-passed global.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::calendar::Calendar;

/// `-1.0f` (a `float` in the exe's constant data): the initial value of
/// most floats of the record.
const MINUS_ONE: u32 = 0x0101_2054;
/// `24.0` (`double`): hours per day.
const HOURS_PER_DAY: u32 = 0x0103_56d8;
/// `TESGlobal` value getter (`float TESGlobal::fValue`, returned in ST0).
const TES_GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `TESGlobal` value setter (`float` argument on the stack).
const TES_GLOBAL_SET_VALUE: u32 = 0x0046_dce0;
/// `004839c0`: the form lookup by form number the calendar uses to find its
/// six globals (cdecl, one word; 0 when the form table does not hold it).
const FORM_LOOKUP: u32 = 0x0048_39c0;
/// `Calendar`'s "days passed is stale" setter (`00867a20`, in `calendar.rs`).
const MARK_DAYS_PASSED_STALE: u32 = 0x0086_7a20;
/// The form numbers of the six calendar globals, in the order of
/// `Calendar`'s fields (year, month, day, hour, days passed, time scale).
const CALENDAR_GLOBAL_FORMS: [u32; 6] = [0x35, 0x36, 0x37, 0x38, 0x39, 0x3a];
/// The virtual slots `008676d0` calls, in order (the offsets into the
/// vtable; slot `+0x18` and the ones after `+0x40` are not called).
const CACHED_VALUE_SLOTS: [u32; 16] = [
    0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
];

layout! {
    /// The 0x48-byte record `00867510` initializes. Only the offsets the
    /// code uses are named; the game's field names are unknown.
    pub struct Block48: 0x48 {
        0x00 float_00: f32,
        0x04 float_04: f32,
        0x08 float_08: f32,
        0x0C float_0c: f32,
        0x10 float_10: f32,
        0x14 float_14: f32,
        0x1C float_1c: f32,
        0x20 float_20: f32,
        0x24 float_24: f32,
        0x28 float_28: f32,
        0x2C float_2c: f32,
        0x30 word_30: u32,
        0x34 word_34: u32,
        0x38 float_38: f32,
        0x3C float_3c: f32,
        0x40 byte_40: u8,
        0x44 word_44: u32,
    }
}

// Translated from 00867510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x48-byte record: eleven floats are set to -1.0, the
/// two words at `+0x30` and `+0x34` to -1, the floats at `+0x38` and `+0x3c`
/// to 0, the byte at `+0x40` and the word at `+0x44` to 0 (the word at `+0x18`
/// and the bytes `+0x41` to `+0x43` are not written). Returns `this`.
pub fn fn_00867510(e: &mut Engine, this: Ptr<Block48>) -> Ptr<Block48> {
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    for offset in [
        0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c,
    ] {
        e.mem.set_u32(this.addr().wrapping_add(offset), minus_one);
    }
    e.set(this, Block48::word_30, 0xffff_ffff);
    e.set(this, Block48::word_34, 0xffff_ffff);
    e.set(this, Block48::byte_40, 0);
    e.set(this, Block48::float_38, 0.0);
    e.set(this, Block48::float_3c, 0.0);
    e.set(this, Block48::word_44, 0);
    this
}

// Translated from 008675e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the record `source` into `this` (nothing happens when `source` is
/// null): the floats go through the x87 stack (a signalling NaN is stored
/// quiet), the two words at `+0x30`/`+0x34`, the byte at `+0x40` and the word
/// at `+0x44` are copied as they are.
pub fn fn_008675e0(e: &mut Engine, this: Ptr<Block48>, source: Ptr<Block48>) {
    if source.is_null() {
        return;
    }
    for offset in [
        0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c,
    ] {
        let bits = x87_quiet_bits(e.mem.u32(source.addr().wrapping_add(offset)));
        e.mem.set_u32(this.addr().wrapping_add(offset), bits);
    }
    let word = e.get(source, Block48::word_30);
    e.set(this, Block48::word_30, word);
    let word = e.get(source, Block48::word_34);
    e.set(this, Block48::word_34, word);
    for offset in [0x38, 0x3c] {
        let bits = x87_quiet_bits(e.mem.u32(source.addr().wrapping_add(offset)));
        e.mem.set_u32(this.addr().wrapping_add(offset), bits);
    }
    let byte = e.get(source, Block48::byte_40);
    e.set(this, Block48::byte_40, byte);
    let word = e.get(source, Block48::word_44);
    e.set(this, Block48::word_44, word);
}

// Translated from 008676d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the sixteen virtual slots of `this` listed in `CACHED_VALUE_SLOTS`
/// in turn, ignoring what they return (most return a `float`, two an
/// integer). The actor calls it on its `CachedValuesOwner`.
pub fn fn_008676d0(e: &mut Engine, this: Ptr) {
    for slot in CACHED_VALUE_SLOTS {
        e.vcall(this.addr(), slot, &[]);
    }
}

// Translated from 008677d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `Calendar`: zeroes the six global pointers and
/// `iMidnightsPassed`, marks the days-passed global stale (`00867a20`) and
/// clears the six pointers again (`00867840`). Returns `this`.
pub fn fn_008677d0(e: &mut Engine, this: Ptr<Calendar>) -> Ptr<Calendar> {
    e.set(this, Calendar::pGameYear, Ptr::new(0));
    e.set(this, Calendar::pGameMonth, Ptr::new(0));
    e.set(this, Calendar::pGameDay, Ptr::new(0));
    e.set(this, Calendar::pGameHour, Ptr::new(0));
    e.set(this, Calendar::pGameDaysPassed, Ptr::new(0));
    e.set(this, Calendar::pTimeScale, Ptr::new(0));
    e.set(this, Calendar::iMidnightsPassed, 0);
    e.call(MARK_DAYS_PASSED_STALE, &args![this]);
    fn_00867840(e, this);
    this
}

// Translated from 00867840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the six `TESGlobal` pointers of the `Calendar` to null.
pub fn fn_00867840(e: &mut Engine, this: Ptr<Calendar>) {
    e.set(this, Calendar::pGameYear, Ptr::new(0));
    e.set(this, Calendar::pGameMonth, Ptr::new(0));
    e.set(this, Calendar::pGameDay, Ptr::new(0));
    e.set(this, Calendar::pGameHour, Ptr::new(0));
    e.set(this, Calendar::pGameDaysPassed, Ptr::new(0));
    e.set(this, Calendar::pTimeScale, Ptr::new(0));
}

// Translated from 00867890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `Calendar` method (no name in the Xbox PDB): looks the six globals
/// (form numbers 0x35 to 0x3a) up through `004839c0` into the pointer
/// fields, then adds the hour global divided by 24 to the days-passed
/// global (`hour / 24 + days`, rounded to a `float`), clears
/// `iMidnightsPassed` and marks the days-passed global stale.
pub fn fn_00867890(e: &mut Engine, this: Ptr<Calendar>) {
    let year = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[0]]).ptr();
    e.set(this, Calendar::pGameYear, year);
    let month = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[1]]).ptr();
    e.set(this, Calendar::pGameMonth, month);
    let day = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[2]]).ptr();
    e.set(this, Calendar::pGameDay, day);
    let hour = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[3]]).ptr();
    e.set(this, Calendar::pGameHour, hour);
    let days_passed = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[4]]).ptr();
    e.set(this, Calendar::pGameDaysPassed, days_passed);
    let time_scale = e.call(FORM_LOOKUP, &args![CALENDAR_GLOBAL_FORMS[5]]).ptr();
    e.set(this, Calendar::pTimeScale, time_scale);

    let hour_value = e.call(TES_GLOBAL_GET_VALUE, &args![hour]).f32();
    let days_value = e.call(TES_GLOBAL_GET_VALUE, &args![days_passed]).f32();
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    let total_days = (hour_value as f64 / hours_per_day + days_value as f64) as f32;
    e.call(TES_GLOBAL_SET_VALUE, &args![days_passed, total_days]);
    e.set(this, Calendar::iMidnightsPassed, 0);
    e.call(MARK_DAYS_PASSED_STALE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00867510, fn_00867510(Ptr<Block48>) -> Ptr<Block48>),
        entry!(0x008675e0, fn_008675e0(Ptr<Block48>, Ptr<Block48>)),
        entry!(0x008676d0, fn_008676d0(Ptr)),
        entry!(0x008677d0, fn_008677d0(Ptr<Calendar>) -> Ptr<Calendar>),
        entry!(0x00867840, fn_00867840(Ptr<Calendar>)),
        entry!(0x00867890, fn_00867890(Ptr<Calendar>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_constructor_sets_the_defaults_and_leaves_the_gap() {
        let mut e = Engine::new();
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, (-1.0f32).to_bits());
        let record = e.new_object::<Block48>();
        e.mem.write(record.addr(), &[0xaa; 0x48]);
        assert_eq!(e.call(0x0086_7510, &[record.addr()]).u32(), record.addr());
        for offset in [
            0x00, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c,
        ] {
            assert_eq!(e.mem.f32(record.addr() + offset), -1.0, "+{offset:#x}");
        }
        assert_eq!(e.get(record, Block48::word_30), 0xffff_ffff);
        assert_eq!(e.get(record, Block48::word_34), 0xffff_ffff);
        assert_eq!(e.get(record, Block48::float_38), 0.0);
        assert_eq!(e.get(record, Block48::float_3c), 0.0);
        assert_eq!(e.get(record, Block48::byte_40), 0);
        assert_eq!(e.get(record, Block48::word_44), 0);
        // The word at +0x18 and the bytes after +0x40 are not touched.
        assert_eq!(e.mem.u32(record.addr() + 0x18), 0xaaaa_aaaa);
        assert_eq!(e.mem.u8(record.addr() + 0x41), 0xaa);
    }

    #[test]
    fn record_copy_copies_the_fields_and_quiets_signalling_nans() {
        let mut e = Engine::new();
        let source = e.new_object::<Block48>();
        let target = e.new_object::<Block48>();
        e.mem.write(target.addr(), &[0x55; 0x48]);
        e.mem.set_f32(source.addr(), 2.5);
        e.mem.set_u32(source.addr() + 0x04, 0x7f80_0001);
        e.mem.set_f32(source.addr() + 0x2c, -7.0);
        e.mem.set_u32(source.addr() + 0x30, 0x1234_5678);
        e.mem.set_u32(source.addr() + 0x34, 0x7f80_0001);
        e.mem.set_u32(source.addr() + 0x38, 0xff80_0002);
        e.mem.set_f32(source.addr() + 0x3c, 9.0);
        e.mem.set_u8(source.addr() + 0x40, 1);
        e.mem.set_u32(source.addr() + 0x44, 77);
        e.call(0x0086_75e0, &[target.addr(), source.addr()]);
        assert_eq!(e.mem.f32(target.addr()), 2.5);
        assert_eq!(e.mem.u32(target.addr() + 0x04), 0x7fc0_0001);
        assert_eq!(e.mem.f32(target.addr() + 0x2c), -7.0);
        assert_eq!(e.mem.u32(target.addr() + 0x30), 0x1234_5678);
        // The integer words are copied bit for bit.
        assert_eq!(e.mem.u32(target.addr() + 0x34), 0x7f80_0001);
        assert_eq!(e.mem.u32(target.addr() + 0x38), 0xffc0_0002);
        assert_eq!(e.mem.f32(target.addr() + 0x3c), 9.0);
        assert_eq!(e.mem.u8(target.addr() + 0x40), 1);
        assert_eq!(e.mem.u32(target.addr() + 0x44), 77);
        // The word at +0x18 is not copied.
        assert_eq!(e.mem.u32(target.addr() + 0x18), 0x5555_5555);
    }

    #[test]
    fn record_copy_from_null_does_nothing() {
        let mut e = Engine::new();
        let target = e.new_object::<Block48>();
        e.mem.write(target.addr(), &[0x55; 0x48]);
        e.call(0x0086_75e0, &[target.addr(), 0]);
        assert_eq!(e.mem.u32(target.addr()), 0x5555_5555);
    }

    #[test]
    fn cached_values_owner_calls_each_slot_in_order() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        let slots: Vec<u32> = (0..17).map(|n| 0x0700_0000 + n * 0x10).collect();
        e.put_vtable(0x0710_0000, &slots);
        e.mem.set_u32(object, 0x0710_0000);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0086_76d0, &[object]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<u32> = CACHED_VALUE_SLOTS
            .iter()
            .map(|slot| slots[(*slot / 4) as usize])
            .collect();
        let called: Vec<u32> = log[1..].iter().map(|call| call.0).collect();
        assert_eq!(called, expected);
        assert!(log[1..].iter().all(|call| call.1 == vec![object]));
        assert_eq!(called.len(), 16);
    }

    #[test]
    fn calendar_constructor_clears_the_fields_and_marks_the_days_stale() {
        let mut e = Engine::new();
        e.register(MARK_DAYS_PASSED_STALE, |_, _| Ret::default());
        let calendar = e.new_object::<Calendar>();
        e.mem.write(calendar.addr(), &[0xcc; 0x1c]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0086_77d0, &[calendar.addr()]).u32(),
            calendar.addr()
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MARK_DAYS_PASSED_STALE, vec![calendar.addr()]));
        for offset in (0..0x1c).step_by(4) {
            assert_eq!(e.mem.u32(calendar.addr() + offset), 0, "+{offset:#x}");
        }
    }

    #[test]
    fn calendar_globals_are_cleared_without_touching_the_rest() {
        let mut e = Engine::new();
        let calendar = e.new_object::<Calendar>();
        e.mem.write(calendar.addr(), &[0xcc; 0x20]);
        e.call(0x0086_7840, &[calendar.addr()]);
        for offset in (0..0x18).step_by(4) {
            assert_eq!(e.mem.u32(calendar.addr() + offset), 0);
        }
        assert_eq!(e.mem.u32(calendar.addr() + 0x18), 0xcccc_cccc);
    }

    #[test]
    fn calendar_lookup_folds_the_hour_into_the_days_passed() {
        let mut e = Engine::new();
        e.map(HOURS_PER_DAY, 8);
        e.mem.set_f64(HOURS_PER_DAY, 24.0);
        let globals: Vec<u32> = (0..6).map(|_| e.mem.alloc(0x28)).collect();
        e.register_double(FORM_LOOKUP, {
            let globals = globals.clone();
            move |_, a| Ret {
                eax: globals[(a[0] - 0x35) as usize],
                ..Ret::default()
            }
        });
        e.register(TES_GLOBAL_GET_VALUE, |e, a| {
            e.mem.f32(a[0] + 0x24).into_ret()
        });
        e.register(TES_GLOBAL_SET_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 0x24, a[1]);
            Ret::default()
        });
        e.register(MARK_DAYS_PASSED_STALE, |_, _| Ret::default());
        e.mem.set_f32(globals[3] + 0x24, 18.0);
        e.mem.set_f32(globals[4] + 0x24, 100.0);
        let calendar = e.new_object::<Calendar>();
        e.set(calendar, Calendar::iMidnightsPassed, 9);
        e.call_log = Some(vec![]);
        e.call(0x0086_7890, &[calendar.addr()]);
        let log = e.call_log.take().unwrap();
        for (index, offset) in [0x00u32, 0x04, 0x08, 0x0c, 0x10, 0x14].iter().enumerate() {
            assert_eq!(e.mem.u32(calendar.addr() + offset), globals[index]);
        }
        assert_eq!(e.mem.f32(globals[4] + 0x24), 100.75);
        assert_eq!(e.get(calendar, Calendar::iMidnightsPassed), 0);
        assert_eq!(
            log.last().unwrap(),
            &(MARK_DAYS_PASSED_STALE, vec![calendar.addr()])
        );
        assert_eq!(log[1], (FORM_LOOKUP, vec![0x35]));
        assert_eq!(log[6], (FORM_LOOKUP, vec![0x3a]));
    }
}

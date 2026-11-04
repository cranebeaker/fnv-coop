//! `(unplaced)/run_004612b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The form-type table: 12-byte rows, the first word of a row is the address
/// of the type's name string (the row for type 0 begins with `"NONE"`).
const FORM_TYPE_TABLE: u32 = 0x0118_7004;
/// Bytes per row of [`FORM_TYPE_TABLE`].
const FORM_TYPE_ROW: u32 = 12;
/// Number of types in the table (0x00 to 0x78).
const FORM_TYPE_COUNT: u32 = 0x79;
/// The empty string (`""`) answered for a type outside the table.
const EMPTY_STRING: u32 = 0x0101_1584;

// Translated from 004612b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of a form type (the byte `TESForm::cFormType`): the address of
/// its string in the table of 0x79 rows, or of the empty string for a type
/// from 0x79 up. `CreateFormOfType` uses it when it meets a type it has no
/// constructor for.
pub fn fn_004612b0(e: &mut Engine, form_type: u8) -> u32 {
    if (form_type as u32) < FORM_TYPE_COUNT {
        e.mem
            .u32(FORM_TYPE_TABLE + form_type as u32 * FORM_TYPE_ROW)
    } else {
        EMPTY_STRING
    }
}

// Translated from 004612e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x108` (unnamed in the map; `ECX`, no stack words). The topic
/// code calls it on the object at `011c3f2c` for the first node of its
/// topic list.
pub fn fn_004612e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x108)
}

/// The byte `fn_00461300` sets (`01202d63`).
const FLAG_01202D63: u32 = 0x0120_2d63;

// Translated from 00461300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `01202d63` to 1 (unnamed in the map; no arguments).
/// `TESDataHandler::CleanUpBadForms` and the form code call it.
pub fn fn_00461300(e: &mut Engine) {
    e.mem.set_u8(FLAG_01202D63, 1);
}

// Translated from 00461310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, one word; `RET 4`): stores its low byte at `this + 0x24`.
pub fn fn_00461310(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x24), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0046_1310, fn_00461310(Ptr, u8)),
        entry!(0x004612b0, fn_004612b0(u8) -> u32),
        entry!(0x004612e0, fn_004612e0(Ptr) -> Ptr),
        entry!(0x00461300, fn_00461300()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_row_name_and_the_empty_string_past_the_table() {
        let mut e = Engine::new();
        e.map(FORM_TYPE_TABLE & !0xfff, 0x2000);
        e.set_global(FORM_TYPE_TABLE, 0x0101_a6bfu32);
        e.set_global(FORM_TYPE_TABLE + 5 * FORM_TYPE_ROW, 0x0102_0000u32);
        e.set_global(FORM_TYPE_TABLE + 0x78 * FORM_TYPE_ROW, 0x0102_8000u32);
        assert_eq!(e.call(0x0046_12b0, &args![0u8]).u32(), 0x0101_a6bf);
        assert_eq!(e.call(0x0046_12b0, &args![5u8]).u32(), 0x0102_0000);
        assert_eq!(e.call(0x0046_12b0, &args![0x78u8]).u32(), 0x0102_8000);
        assert_eq!(e.call(0x0046_12b0, &args![0x79u8]).u32(), EMPTY_STRING);
        assert_eq!(e.call(0x0046_12b0, &args![0xffu8]).u32(), EMPTY_STRING);
    }

    #[test]
    fn the_list_is_0x108_bytes_into_the_object() {
        let mut e = Engine::new();
        let back = e.call(0x0046_12e0, &args![Ptr::<()>::new(0x0200_0000)]);
        assert_eq!(back.u32(), 0x0200_0108);
    }

    #[test]
    fn sets_the_byte_to_one_and_leaves_its_neighbours() {
        let mut e = Engine::new();
        e.map(FLAG_01202D63 & !0xfff, 0x1000);
        e.mem.set_u32(FLAG_01202D63 - 3, 0);
        e.mem.set_u8(FLAG_01202D63 + 1, 0x77);
        e.call(0x0046_1300, &args![]);
        assert_eq!(e.mem.u8(FLAG_01202D63), 1);
        assert_eq!(e.mem.u32(FLAG_01202D63 - 3), 0x0100_0000);
        assert_eq!(e.mem.u8(FLAG_01202D63 + 1), 0x77);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00461310_stores_the_byte_at_0x24() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.call(0x0046_1310, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u8(this + 0x25), 0);
        assert_eq!(e.mem.u8(this + 0x23), 0);
    }
}

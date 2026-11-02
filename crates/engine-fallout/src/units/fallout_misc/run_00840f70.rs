//! `fallout/misc/run_00840f70` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (`thiscall` on a form): its type byte (+4), zero-extended.
const FORM_TYPE: u32 = 0x0040_1170;
/// `0044dee0` (`thiscall` on a word: stores 0), `00841100` (the map's
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer_ov2`; `thiscall` on a word, one word: ORs
/// it in) and `00559450` (`thiscall`: the word at `this`): the flag word the function
/// builds on its stack.
const WORD_CLEAR: u32 = 0x0044_dee0;
const WORD_SET_BITS: u32 = 0x0084_1100;
const WORD_AT: u32 = 0x0055_9450;
/// `004280f0` (`thiscall` on a flag word, one word: whether any of the bits is set).
const ANY_BITS: u32 = 0x0042_80f0;
/// `0084e3a0` (`thiscall`: the word at +0xC, a form's id), the data handler (`011c3f2c`)
/// and its `00469860` (`thiscall`, one word, `this` not read: whether the id is above
/// `0xFEFFFFFF`).
const FORM_ID: u32 = 0x0084_e3a0;
const DATA_HANDLER: u32 = 0x011c_3f2c;
const IS_HIGH_FORM_ID: u32 = 0x0046_9860;
/// `0057b790` (`thiscall` on the form: whether the cell at its +0x40 is loaded,
/// `TES::IsCellLoaded`).
const REFERENCE_CELL_LOADED: u32 = 0x0057_b790;

/// `004280f0` on `this`, then `00841100` with the same bits on the stack word when set.
fn copy_bits(e: &mut Engine, this: Ptr, word: Ptr, bits: u32) {
    if e.call(ANY_BITS, &args![this, bits]).bool() {
        e.call(WORD_SET_BITS, &args![word, bits]);
    }
}

// Translated from 00840f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a flag word, one word: a form; `RET 4`; the
/// answer in `AL`). Builds a flag word on its stack (`0044dee0`) from the form's type (the
/// exe's switch on `type - 0x3A` with its byte map at `008410cc`, as `match` arms):
/// * types 0x3A to 0x40 and 0x69: notes whether the form's id is above `0xFEFFFFFF`
///   (`00469860` on the data handler); when the form's cell is loaded (`0057b790`),
///   copies bits `0x10000000` and `0x8000000` of `this` (`004280f0`, `00841100`); for
///   types 0x3B and 0x3C also bit `0x40000`;
/// * type 0x47: copies bit `0x40000000`;
/// * every other type: nothing.
///
/// The word then replaces `*this` (`00559450`); answers whether it is not 0 or the id was
/// above `0xFEFFFFFF`.
pub fn fn_00840f70(e: &mut Engine, this: Ptr, form: Ptr) -> bool {
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    e.with_stack(4, |e, word| {
        e.call(WORD_CLEAR, &args![word]);
        let mut high_id = false;
        match form_type.wrapping_sub(0x3a) {
            0..=6 | 0x2f => {
                let id = e.call(FORM_ID, &args![form]).u32();
                let handler: u32 = e.global(DATA_HANDLER);
                if e.call(IS_HIGH_FORM_ID, &args![handler, id]).bool() {
                    high_id = true;
                }
                if e.call(REFERENCE_CELL_LOADED, &args![form]).bool() {
                    copy_bits(e, this, word, 0x1000_0000);
                    copy_bits(e, this, word, 0x0800_0000);
                }
                if (0x3b..=0x3c).contains(&(form_type as i32)) {
                    copy_bits(e, this, word, 0x0004_0000);
                }
            }
            0xd => copy_bits(e, this, word, 0x4000_0000),
            _ => {}
        }
        let value = e.call(WORD_AT, &args![word]).u32();
        e.mem.set_u32(this.addr(), value);
        e.mem.u32(this.addr()) != 0 || high_id
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_0f70, fn_00840f70(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// A form of `form_type` with id `id`; `loaded` is the cell-loaded answer.
    fn rig(form_type: u8, id: u32, loaded: bool) -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0c00_0000u32);
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(WORD_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            a[0].into_ret()
        });
        e.register(WORD_SET_BITS, |e, a| {
            let value = e.mem.u32(a[0]) | a[1];
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ANY_BITS, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(IS_HIGH_FORM_ID, |_, a| {
            u32::from(a[1] > 0xfeff_ffff).into_ret()
        });
        e.register_double(REFERENCE_CELL_LOADED, move |_, _| {
            u32::from(loaded).into_ret()
        });
        let form = Ptr::new(e.mem.alloc(0x44));
        e.mem.set_u8(form.addr() + 4, form_type);
        e.mem.set_u32(form.addr() + 0xc, id);
        let this = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(this.addr(), 0xffff_ffff);
        (e, this, form)
    }

    fn run(e: &mut Engine, this: Ptr, form: Ptr) -> (bool, u32) {
        let answer = e.call(0x0084_0f70, &args![this, form]).bool();
        (answer, e.mem.u32(this.addr()))
    }

    #[test]
    fn types_3a_to_40_with_a_loaded_cell_keep_their_bits() {
        let (mut e, this, form) = rig(0x3a, 0x10, true);
        assert_eq!(run(&mut e, this, form), (true, 0x1800_0000));
        // Types 0x3B and 0x3C also keep 0x40000.
        let (mut e, this, form) = rig(0x3b, 0x10, true);
        assert_eq!(run(&mut e, this, form), (true, 0x1804_0000));
        let (mut e, this, form) = rig(0x69, 0x10, true);
        assert_eq!(run(&mut e, this, form), (true, 0x1800_0000));
    }

    #[test]
    fn an_unloaded_cell_keeps_only_the_type_bit() {
        let (mut e, this, form) = rig(0x3c, 0x10, false);
        assert_eq!(run(&mut e, this, form), (true, 0x0004_0000));
        let (mut e, this, form) = rig(0x40, 0x10, false);
        assert_eq!(run(&mut e, this, form), (false, 0));
        // A created form (id above 0xFEFFFFFF) answers true without bits.
        let (mut e, this, form) = rig(0x40, 0xff00_0001, false);
        assert_eq!(run(&mut e, this, form), (true, 0));
    }

    #[test]
    fn type_47_keeps_0x40000000_and_other_types_nothing() {
        let (mut e, this, form) = rig(0x47, 0xff00_0001, true);
        assert_eq!(run(&mut e, this, form), (true, 0x4000_0000));
        for form_type in [0x39u8, 0x41, 0x46, 0x48, 0x68, 0x6a] {
            let (mut e, this, form) = rig(form_type, 0xff00_0001, true);
            assert_eq!(run(&mut e, this, form), (false, 0));
        }
    }
}

//! `fallout shared/run_0047ba80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x80 of the byte at +0x100 is clear.
pub fn fn_0047bcf0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x100) & 0x80 == 0
}

// Translated from 0047bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm`'s `InitializeDataComponent` (vtable `0101aef0`, RTTI
/// `BGSTouchSpellForm`): the word at +4 becomes 0 and the 16-bit value at
/// +8 0xFF.
pub fn fn_0047bd40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 8, 0xff);
}

// Translated from 0047bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm`'s constructor: the `BaseFormComponent` base
/// (`00402d30`), its vtable `0101aef0` and `InitializeDataComponent`
/// (`0047bd40`). Returns `this`.
pub fn fn_0047bd10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_aef0);
    fn_0047bd40(e, this);
    this
}

/// `00500940` (`thiscall`, no stack word): the form list member of a form.
const FORM_LIST: u32 = 0x0050_0940;
/// `005f65d0` (`thiscall`, `RET 4`): whether the list holds the form whose
/// word is at the given address (`AL`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `00401170` (`thiscall`, no stack word): the form's type byte, as a word.
const FORM_TYPE: u32 = 0x0040_1170;
/// `BGSEntryPoint::HandleEntryPoint` (engine map, `005e58f0`, `cdecl`,
/// three words here: entry point, actor, address of the `float` result).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `TESBipedModelForm::GetPlayable` (engine map, `00480d90`, `thiscall`).
const BIPED_PLAYABLE: u32 = 0x0048_0d90;
/// `00514410` (`thiscall`, no stack word): a byte of the form (`AL`).
const FORM_BYTE: u32 = 0x0051_4410;
/// `00726070` (`thiscall`, no stack word): the word at +4 of `this`.
const WORD_AT_4: u32 = 0x0072_6070;
/// `MiddleHighProcess::GetLastBoundWeapon` (engine map name of the folded
/// body, `008d85e0`, `thiscall`, no stack word).
const GETTER_008D85E0: u32 = 0x008d_85e0;
/// `00446390` (`thiscall`, no stack word): the word at the start of the
/// object.
const FIRST_WORD: u32 = 0x0044_6390;

// Translated from 0047ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form list at +4 holds `form` (`thiscall`, `RET 4`; the engine
/// map gives it no name): false without a list; otherwise `005f65d0` on the
/// list's member (`00500940`) with the address of the stack word holding
/// `form`.
pub fn fn_0047ba80(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let list = e.mem.u32(this.addr() + 4);
    if list == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        let member = e.call(FORM_LIST, &args![list]).u32();
        e.call(LIST_CONTAINS, &args![member, slot]).bool()
    })
}

// Translated from 0047bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A form's component by its type (`cdecl`, one word; the engine map gives
/// it no name): for type 0x18 the address +0x15c of the form, for type 0x28
/// +0xbc, otherwise (and for no form) 0.
pub fn fn_0047bac0(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    match e.call(FORM_TYPE, &args![form]).u32() {
        0x18 => form.wrapping_add(0x15c),
        0x28 => form.wrapping_add(0xbc),
        _ => 0,
    }
}

// Translated from 0047bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `form` counts as `other` (`cdecl`, two words; the engine map
/// gives it no name): false when either is null, true when they are the
/// same; otherwise, when `other`'s component (`0047bac0`) lists `form`
/// (`0047ba80`) true. Else, only when the player's entry point 0x30
/// (`005e58f0` on a `float` from 0.0) leaves a value above the `0.0` at
/// `01012060`, both forms have the same type and `form`'s virtual slot
/// +0x94 answers false: for type 0x18, true when `form`'s +0x70 is playable
/// (`00480d90`) and both answer the same byte (`00514410`) and the same
/// word of their +0x154 (`00726070`); for type 0x28, true when `form`
/// passes `0047bcf0` and both answer the same `008d85e0` and `00446390`.
/// False otherwise (also when `other` has no component).
pub fn fn_0047bb50(e: &mut Engine, form: u32, other: u32) -> bool {
    if form == 0 || other == 0 {
        return false;
    }
    if form == other {
        return true;
    }
    let component = fn_0047bac0(e, other);
    if component == 0 {
        return false;
    }
    if fn_0047ba80(e, Ptr::new(component), form) {
        return true;
    }
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0.0f32.to_bits());
        let player = e.mem.u32(PLAYER_POINTER);
        e.call(HANDLE_ENTRY_POINT, &args![0x30u32, player, slot]);
        e.mem.u32(slot.addr())
    });
    let zero: f64 = e.global(ZERO);
    // `FCOMP`, `TEST AH,0x41`: on only when greater (a NaN stops).
    if !(x87_load32(value) > zero) {
        return false;
    }
    let other_type = e.call(FORM_TYPE, &args![other]).u32();
    if other_type != e.call(FORM_TYPE, &args![form]).u32() {
        return false;
    }
    if e.vcall(form, 0x94, &args![]).bool() {
        return false;
    }
    match e.call(FORM_TYPE, &args![other]).u32() {
        0x18 => {
            if !e
                .call(BIPED_PLAYABLE, &args![form.wrapping_add(0x70)])
                .bool()
            {
                return false;
            }
            let mine = e.call(FORM_BYTE, &args![form]).u8();
            let theirs = e.call(FORM_BYTE, &args![other]).u8();
            if mine != theirs {
                return false;
            }
            let mine = e.call(WORD_AT_4, &args![form.wrapping_add(0x154)]).u32();
            let theirs = e.call(WORD_AT_4, &args![other.wrapping_add(0x154)]).u32();
            mine == theirs
        }
        0x28 => {
            if !fn_0047bcf0(e, Ptr::new(form)) {
                return false;
            }
            let mine = e.call(GETTER_008D85E0, &args![form]).u32();
            let theirs = e.call(GETTER_008D85E0, &args![other]).u32();
            if mine != theirs {
                return false;
            }
            let mine = e.call(FIRST_WORD, &args![form]).u32();
            let theirs = e.call(FIRST_WORD, &args![other]).u32();
            mine == theirs
        }
        _ => false,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_ba80, fn_0047ba80(Ptr, u32) -> bool),
        entry!(0x0047_bac0, fn_0047bac0(u32) -> u32),
        entry!(0x0047_bb50, fn_0047bb50(u32, u32) -> bool),
        entry!(0x0047_bcf0, fn_0047bcf0(Ptr) -> bool),
        entry!(0x0047_bd40, fn_0047bd40(Ptr)),
        entry!(0x0047_bd10, fn_0047bd10(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_0047bcf0_tests_bit_0x80() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x104);
        e.mem.set_u8(p + 0x100, 0x7f);
        assert!(e.call(0x0047_bcf0, &args![p]).bool());
        e.mem.set_u8(p + 0x100, 0x80);
        assert!(!e.call(0x0047_bcf0, &args![p]).bool());
    }

    #[test]
    fn b020_0047bd10_constructs_and_0047bd40_initializes() {
        let mut e = Engine::new();
        e.register(0x0040_2d30, |_, _| Ret::default());
        let p = e.mem.alloc(0xc);
        e.mem.set_u32(p + 4, 5);
        e.mem.set_u32(p + 8, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_bd10, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x0101_aef0);
        assert_eq!(e.mem.u32(p + 4), 0);
        assert_eq!(e.mem.u32(p + 8), 0xffff_00ff);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0040_2d30, vec![p]));
        e.mem.set_u16(p + 8, 0);
        e.call(0x0047_bd40, &args![p]);
        assert_eq!(e.mem.u16(p + 8), 0xff);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    const SCRATCH: u32 = 0x0f00_0000;
    const VTABLE: u32 = 0x0f00_1000;
    const SLOT_94: u32 = 0x0f00_2000;

    /// Doubles: the form type is the byte at +4 of the form; the list
    /// member is the list; the list holds the word at `SCRATCH`; the entry
    /// point stores the float at `SCRATCH + 4`; slot +0x94 answers the byte
    /// at `SCRATCH + 8`; the other getters answer the word at +8 of their
    /// `this` (`this` + 0x154 for the +4 getter).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(SCRATCH, 0x10);
        e.mem.map(0x0101_2000, 0x100);
        e.mem.map(0x011d_e000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.register(FORM_TYPE, |e, a| word(u32::from(e.mem.u8(a[0] + 4))));
        e.register(FORM_LIST, |_, a| word(a[0]));
        e.register(LIST_CONTAINS, |e, a| {
            word(u32::from(e.mem.u32(a[1]) == e.mem.u32(SCRATCH)))
        });
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.u32(SCRATCH + 4);
            e.mem.set_u32(a[2], value);
            Ret::default()
        });
        e.register(SLOT_94, |e, _| word(e.mem.u32(SCRATCH + 8)));
        e.put_vtable(VTABLE, &[0; 0x25]);
        e.mem.set_u32(VTABLE + 0x94, SLOT_94);
        for addr in [
            BIPED_PLAYABLE,
            FORM_BYTE,
            GETTER_008D85E0,
            FIRST_WORD,
            WORD_AT_4,
        ] {
            e.register(addr, |e, a| word(e.mem.u32(a[0] + 8)));
        }
        e
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn form(e: &mut Engine, kind: u8) -> u32 {
        let f = e.mem.alloc(0x200);
        e.mem.set_u32(f, VTABLE);
        e.mem.set_u8(f + 4, kind);
        f
    }

    #[test]
    fn at_0047ba80_asks_the_list() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        assert!(!e.call(0x0047_ba80, &args![this, 0x77u32]).bool());
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(SCRATCH, 0x77);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_ba80, &args![this, 0x77u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FORM_LIST, vec![0x5000]));
        assert_eq!(log[2].0, LIST_CONTAINS);
        assert!(!e.call(0x0047_ba80, &args![this, 0x78u32]).bool());
    }

    #[test]
    fn at_0047bac0_picks_the_component_by_type() {
        let mut e = engine();
        assert_eq!(e.call(0x0047_bac0, &args![0u32]).u32(), 0);
        let armor = form(&mut e, 0x18);
        assert_eq!(e.call(0x0047_bac0, &args![armor]).u32(), armor + 0x15c);
        let weapon = form(&mut e, 0x28);
        assert_eq!(e.call(0x0047_bac0, &args![weapon]).u32(), weapon + 0xbc);
        let other = form(&mut e, 0x19);
        assert_eq!(e.call(0x0047_bac0, &args![other]).u32(), 0);
    }

    #[test]
    fn at_0047bb50_matches_forms() {
        let mut e = engine();
        let a = form(&mut e, 0x18);
        let b = form(&mut e, 0x18);
        let run = |e: &mut Engine, x: u32, y: u32| e.call(0x0047_bb50, &args![x, y]).bool();
        assert!(!run(&mut e, 0, b));
        assert!(!run(&mut e, a, 0));
        assert!(run(&mut e, a, a));
        // Other has no component.
        let plain = form(&mut e, 0x19);
        assert!(!run(&mut e, a, plain));
        // Listed in other's component.
        e.mem.set_u32(b + 0x15c + 4, 0x5000);
        e.mem.set_u32(SCRATCH, a);
        assert!(run(&mut e, a, b));
        e.mem.set_u32(SCRATCH, 0);
        // Entry point leaves 0: false.
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(SCRATCH + 4, 1.0f32.to_bits());
        // Type 0x18: playable, same byte, same +0x154 word.
        e.mem.set_u32(a + 0x70 + 8, 1);
        assert!(run(&mut e, a, b));
        e.mem.set_u32(b + 0x154 + 8, 9);
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(b + 0x154 + 8, 0);
        e.mem.set_u32(b + 8, 1);
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(b + 8, 0);
        e.mem.set_u32(a + 0x70 + 8, 0);
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(a + 0x70 + 8, 1);
        // Slot +0x94 true: false.
        e.mem.set_u32(SCRATCH + 8, 1);
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(SCRATCH + 8, 0);
        // A NaN from the entry point stops.
        e.mem.set_u32(SCRATCH + 4, 0x7fc0_0000);
        assert!(!run(&mut e, a, b));
        e.mem.set_u32(SCRATCH + 4, 1.0f32.to_bits());
        // Different types.
        let w = form(&mut e, 0x28);
        let v = form(&mut e, 0x28);
        assert!(!run(&mut e, a, w));
        // Type 0x28: 0047bcf0 (bit 0x80 of +0x100 clear) and both getters.
        assert!(run(&mut e, w, v));
        e.mem.set_u32(v + 8, 3);
        assert!(!run(&mut e, w, v));
        e.mem.set_u32(v + 8, 0);
        e.mem.set_u8(w + 0x100, 0x80);
        assert!(!run(&mut e, w, v));
    }
}

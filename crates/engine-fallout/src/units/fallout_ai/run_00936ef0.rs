//! `fallout/ai/run_00936ef0` (an address run of unnamed functions between
//! the Xbox PDB's source units), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two kinds of function share the run. `00936ef0`, `00936f50` and `00936fb0`
//! belong to one actor method group that decides whether to call `004534f0`
//! on the thing the actor's virtual slot `0x1d0` returns. The others
//! (`00937000`, `00937030`, `00937090`) work on a 0x10-byte list object whose
//! layout is the Xbox PDB's `ModifierList` (a `BSSimpleList<Modifier *>` head,
//! `bZeroValuesAllowed` at +8, `pFastModifiers` at +0xC), which the actor keeps
//! at `+0xD0` and `+0xE0` on PC (`pPermanentModifiers` and
//! `pBaseValueOverrides`, 0x10 lower than the Xbox offsets).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global pointer (`0x011dea3c`) whose virtual slot `0x1f4` returns a
/// pointer to a position.
const POSITION_REFERENCE: u32 = 0x011d_ea3c;
/// The setting object read through `00403e20` (`ECX` = this address), whose
/// result points at the `float` limit.
const LIMIT_SETTING: u32 = 0x011c_d2f4;
/// `(this = object, position) -> float` in ST0: a distance to a position.
const DISTANCE_TO: u32 = 0x0057_2380;
/// `(this = setting) -> pointer to its float value`.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The singleton pointer (`0x011c3b3c`) that `004534f0` is called on.
const TARGET_SINGLETON: u32 = 0x011c_3b3c;
/// `(this = singleton, object)`.
const NOTIFY: u32 = 0x0045_34f0;
/// `(this = object, flag mask) -> bool`: tests flag bits of the object.
const TEST_FLAGS: u32 = 0x0045_6630;
/// The flag mask `00936fb0` tests.
const FLAG_MASK: u32 = 0x0100_0000;
/// `(this = source) -> float` in ST0.
const GET_FLOAT: u32 = 0x006b_9130;
/// `(this, float)`.
const SET_FLOAT: u32 = 0x004f_5d90;
/// The base list constructor (`this`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The list's "delete all" step (`this`).
const LIST_DELETE_ALL: u32 = 0x0093_70b0;
/// The base list destructor (`this`).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;

layout! {
    /// The Xbox PDB's `ModifierList` (0x10 bytes), only the fields used here.
    pub struct ModifierList: 0x10 {
        /// `bZeroValuesAllowed` (Xbox PDB).
        0x08 bZeroValuesAllowed: u8,
        /// `pFastModifiers` (Xbox PDB): `FastModifierCollection*`.
        0x0C pFastModifiers: u32,
    }
}

// Translated from 00936ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `this` is within the limit of the position reference (unnamed in
/// the Xbox PDB): the distance from `this` to the position the global
/// reference object's slot `0x1f4` returns is at most the `float` setting at
/// `0x011cd2f4`. The exe's test (`FCOMPP; TEST AH,1`) is false for a NaN on
/// either side.
pub fn fn_00936ef0(e: &mut Engine, this: u32) -> bool {
    let reference: u32 = e.global(POSITION_REFERENCE);
    let position = e.vcall(reference, 0x1f4, &args![]).u32();
    let distance = e.call(DISTANCE_TO, &args![this, position]).f32();
    let limit_address = e.call(SETTING_VALUE, &args![LIMIT_SETTING]).u32();
    let limit = e.mem.f32(limit_address);
    distance as f64 <= limit as f64
}

// Translated from 00936f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Notifies `004534f0` about the object `this`'s slot `0x1d0` returns
/// (unnamed in the Xbox PDB): nothing when that is null; otherwise, when the
/// object's flag bit test (`00936fb0`) is false or `force` is set, and `this`
/// is within the limit (`fn_00936ef0`), calls `004534f0` on the singleton at
/// `0x011c3b3c` with the object.
pub fn fn_00936f50(e: &mut Engine, this: u32, force: u8) {
    let object = e.vcall(this, 0x1d0, &args![]).u32();
    if object != 0 && (!fn_00936fb0(e, object) || force != 0) && fn_00936ef0(e, this) {
        let singleton: u32 = e.global(TARGET_SINGLETON);
        e.call(NOTIFY, &args![singleton, object]);
    }
}

// Translated from 00936fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag bit `0x01000000` of the object through `00456630` (unnamed in
/// the Xbox PDB) and returns its result.
pub fn fn_00936fb0(e: &mut Engine, this: u32) -> bool {
    e.call(TEST_FLAGS, &args![this, FLAG_MASK]).bool()
}

// Translated from 00936fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of a byte plus a float (unnamed in the Xbox PDB): copies
/// the byte at `source`, then sets the float from `source`'s float getter
/// (`006b9130`) through the setter `004f5d90`. Returns `this`.
pub fn fn_00936fd0(e: &mut Engine, this: u32, source: u32) -> u32 {
    let byte = e.mem.u8(source);
    e.mem.set_u8(this, byte);
    let value = e.call(GET_FLOAT, &args![source]).f32();
    e.call(SET_FLOAT, &args![this, value]);
    this
}

// Translated from 00937000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x10-byte list object (unnamed in the Xbox PDB): runs
/// the base list constructor, then clears `pFastModifiers` and
/// `bZeroValuesAllowed`. Returns `this`.
pub fn fn_00937000(e: &mut Engine, this: Ptr<ModifierList>) -> Ptr<ModifierList> {
    e.call(LIST_CONSTRUCT, &args![this]);
    e.set(this, ModifierList::pFastModifiers, 0);
    e.set(this, ModifierList::bZeroValuesAllowed, 0);
    this
}

// Translated from 00937030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937030 - the compiler's exception frame (FS:[0], states 0 and -1 around the two steps) is not translated; it only matters if the delete-all step throws.
/// Destructor of the 0x10-byte list object (unnamed in the Xbox PDB): the list
/// object's delete-all step (`009370b0`), then the base list destructor
/// (`0046ffb0`).
pub fn fn_00937030(e: &mut Engine, this: Ptr<ModifierList>) {
    e.call(LIST_DELETE_ALL, &args![this]);
    e.call(LIST_DESTRUCT, &args![this]);
}

// Translated from 00937090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bZeroValuesAllowed` (byte at +8) of the list object (unnamed in
/// the Xbox PDB).
pub fn fn_00937090(e: &mut Engine, this: Ptr<ModifierList>, value: u8) {
    e.set(this, ModifierList::bZeroValuesAllowed, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00936ef0, fn_00936ef0(u32) -> bool),
        entry!(0x00936f50, fn_00936f50(u32, u8)),
        entry!(0x00936fb0, fn_00936fb0(u32) -> bool),
        entry!(0x00936fd0, fn_00936fd0(u32, u32) -> u32),
        entry!(
            0x00937000,
            fn_00937000(Ptr<ModifierList>) -> Ptr<ModifierList>
        ),
        entry!(0x00937030, fn_00937030(Ptr<ModifierList>)),
        entry!(0x00937090, fn_00937090(Ptr<ModifierList>, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The argument words of every `004534f0` call.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    /// The reference object (slot `0x1f4` returns `0x7777`), the distance
    /// double (returns the word at `this + 0` as a float) and the limit
    /// setting (a float at the address the setting getter returns).
    fn within_rig(distance: f32, limit: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(POSITION_REFERENCE, 4);
        let reference = e.mem.alloc(8);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(reference, table);
        e.mem.set_u32(table + 0x1f4, 0x7000_01f4);
        e.register(0x7000_01f4, |_, _| ret(0x7777));
        e.set_global(POSITION_REFERENCE, reference);
        let subject = e.mem.alloc(8);
        e.mem.set_f32(subject + 4, distance);
        e.register(DISTANCE_TO, |e, a| {
            assert_eq!(a[1], 0x7777, "the position from slot 0x1f4");
            Ret {
                st0: e.mem.f32(a[0] + 4) as f64,
                ..Ret::default()
            }
        });
        let limit_cell = e.mem.alloc(4);
        e.mem.set_f32(limit_cell, limit);
        e.register_double(SETTING_VALUE, move |_, a| {
            assert_eq!(a[0], 0x011c_d2f4);
            ret(limit_cell)
        });
        (e, subject)
    }

    #[test]
    fn within_limit_compares_less_or_equal() {
        let (mut e, subject) = within_rig(5.0, 5.0);
        assert!(e.call(0x00936ef0, &args![subject]).bool(), "equal passes");
        let (mut e, subject) = within_rig(5.5, 5.0);
        assert!(!e.call(0x00936ef0, &args![subject]).bool());
        let (mut e, subject) = within_rig(4.0, 5.0);
        assert!(e.call(0x00936ef0, &args![subject]).bool());
    }

    #[test]
    fn within_limit_is_false_for_nan_on_either_side() {
        let (mut e, subject) = within_rig(f32::NAN, 5.0);
        assert!(!e.call(0x00936ef0, &args![subject]).bool());
        let (mut e, subject) = within_rig(1.0, f32::NAN);
        assert!(!e.call(0x00936ef0, &args![subject]).bool());
    }

    /// An actor whose slot `0x1d0` returns `object`.
    fn actor_returning(e: &mut Engine, object: u32) -> u32 {
        let actor = e.mem.alloc(8);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(actor, table);
        e.mem.set_u32(table + 0x1d0, 0x7000_01d0);
        e.register_double(0x7000_01d0, move |_, _| ret(object));
        actor
    }

    fn notify_rig(distance: f32, flag_set: bool) -> (Engine, u32, Seen) {
        let (mut e, _subject) = within_rig(distance, 5.0);
        let actor = actor_returning(&mut e, 0x4242);
        // `fn_00936ef0` measures from `this` (the actor): give the actor the distance.
        e.mem.set_f32(actor + 4, distance);
        e.register_double(TEST_FLAGS, move |_, a| {
            assert_eq!(a[1], 0x0100_0000);
            ret(flag_set as u32)
        });
        e.map(TARGET_SINGLETON, 4);
        e.set_global(TARGET_SINGLETON, 0x5150u32);
        let seen = Seen::default();
        let sink = seen.clone();
        e.register_double(NOTIFY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, actor, seen)
    }

    #[test]
    fn notify_when_flag_clear_and_within_limit() {
        let (mut e, actor, seen) = notify_rig(1.0, false);
        e.call(0x00936f50, &args![actor, 0u8]);
        assert_eq!(*seen.borrow(), vec![vec![0x5150, 0x4242]]);
    }

    #[test]
    fn flag_set_blocks_the_notification_unless_forced() {
        let (mut e, actor, seen) = notify_rig(1.0, true);
        e.call(0x00936f50, &args![actor, 0u8]);
        assert!(seen.borrow().is_empty());
        e.call(0x00936f50, &args![actor, 1u8]);
        assert_eq!(*seen.borrow(), vec![vec![0x5150, 0x4242]]);
    }

    #[test]
    fn no_notification_when_out_of_limit() {
        let (mut e, actor, seen) = notify_rig(9.0, false);
        e.call(0x00936f50, &args![actor, 1u8]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn no_notification_without_an_object() {
        let (mut e, actor, seen) = notify_rig(1.0, false);
        let table = e.mem.u32(actor);
        e.register(0x7000_01d0, |_, _| ret(0));
        e.mem.set_u32(table + 0x1d0, 0x7000_01d0);
        e.call_log = Some(vec![]);
        e.call(0x00936f50, &args![actor, 1u8]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.call_log.take().unwrap().len(), 2, "the call and the slot");
    }

    #[test]
    fn flag_test_passes_the_mask_and_returns_the_result() {
        let mut e = Engine::new();
        e.register(TEST_FLAGS, |_, a| {
            ret((a[0] == 0x99 && a[1] == 0x0100_0000) as u32)
        });
        assert!(e.call(0x00936fb0, &args![0x99u32]).bool());
        assert!(!e.call(0x00936fb0, &args![0x98u32]).bool());
    }

    #[test]
    fn copy_constructor_copies_the_byte_and_sets_the_float() {
        let mut e = Engine::new();
        let source = e.mem.alloc(8);
        let this = e.mem.alloc(8);
        e.mem.set_u8(source, 0x5a);
        e.register(GET_FLOAT, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register_double(SET_FLOAT, move |_, a| {
            assert_eq!(f32::from_bits(a[1]), 2.5);
            Ret::default()
        });
        assert_eq!(e.call(0x00936fd0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u8(this), 0x5a);
    }

    #[test]
    fn list_constructor_clears_its_fields_after_the_base_constructor() {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        let list = e.new_object::<ModifierList>();
        e.mem.set_u32(list.addr() + 0xc, 0xdead);
        e.mem.set_u8(list.addr() + 8, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00937000, &args![list]).u32(), list.addr());
        assert_eq!(e.mem.u32(list.addr() + 0xc), 0);
        assert_eq!(e.mem.u8(list.addr() + 8), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LIST_CONSTRUCT, vec![list.addr()]));
    }

    #[test]
    fn list_destructor_deletes_all_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(LIST_DELETE_ALL, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        let list = e.new_object::<ModifierList>();
        e.call_log = Some(vec![]);
        e.call(0x00937030, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LIST_DELETE_ALL, vec![list.addr()]),
                (LIST_DESTRUCT, vec![list.addr()])
            ]
        );
    }

    #[test]
    fn zero_values_allowed_setter_stores_the_byte() {
        let mut e = Engine::new();
        let list = e.new_object::<ModifierList>();
        e.call(0x00937090, &args![list, 1u8]);
        assert_eq!(e.mem.u8(list.addr() + 8), 1);
        e.call(0x00937090, &args![list, 0u8]);
        assert_eq!(e.mem.u8(list.addr() + 8), 0);
    }
}

//! `fallout shared/bgspickupputdownsounds.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, the index).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; `cdecl`: address
/// of a form id, file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vfptr offset, source type,
/// target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `00474cb0` (`ECX` = form): the length of the name from the form's
/// virtual slot `0x130`, or 0.
const FORM_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;
/// The form's name virtual slot, as `00474cb0` reads it.
const NAME_SLOT: u32 = 0x130;

/// The `TESForm` type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The `TESSound` type descriptor (`.?AVTESSound@@`).
const TYPE_TES_SOUND: u32 = 0x0118_32fc;

/// The three messages of one sound: no owner, owner by form id, owner by name.
struct Messages {
    unknown_owner: u32,
    owner_id: u32,
    owner_name: u32,
}

/// `"MASTERFILE: Unable to find pickup sound  (%08X) for unknown owner."`,
/// `"... for owner object (%08X)."` and `"... for owner object \"%s\"."`
/// (the first two have two spaces after "sound").
const PICKUP_MESSAGES: Messages = Messages {
    unknown_owner: 0x0101_acc8,
    owner_id: 0x0101_ad10,
    owner_name: 0x0101_ad60,
};
/// The same three messages for the putdown sound.
const PUTDOWN_MESSAGES: Messages = Messages {
    unknown_owner: 0x0101_abe8,
    owner_id: 0x0101_ac30,
    owner_name: 0x0101_ac80,
};

layout! {
    /// `BGSPickupPutdownSounds` (Xbox PDB), 0xc bytes: a `BaseFormComponent`
    /// and the two sounds.
    pub struct BGSPickupPutdownSounds: 0xc {
        /// `pPickupSound` (Xbox PDB): the sound; before `InitItem` it holds
        /// the form id read from the file.
        0x04 pPickupSound: u32,
        /// `pPutdownSound` (Xbox PDB): likewise.
        0x08 pPutdownSound: u32,
    }
}

/// One sound of `InitItem`: when the field holds a form id, turns it into the
/// `TESSound` (adjusted by `file`, looked up, cast) and, when there is none,
/// prints the message that names the owner.
fn init_sound(
    e: &mut Engine,
    this: Ptr<BGSPickupPutdownSounds>,
    field: Field<BGSPickupPutdownSounds, u32>,
    owner: Ptr,
    file: u32,
    messages: &Messages,
) {
    let loaded = e.get(this, field);
    if loaded == 0 {
        return;
    }
    let adjusted = e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), loaded);
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
        e.mem.u32(id.addr())
    });
    let form = e.call(FORM_BY_ID, &args![adjusted]).u32();
    let sound = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SOUND, 0u32],
        )
        .u32();
    e.set(this, field, sound);
    if e.get(this, field) == 0 {
        if owner.is_null() {
            e.call(PRINT, &args![messages.unknown_owner, adjusted]);
        } else if e.call(FORM_NAME_LENGTH, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), NAME_SLOT, &[]).u32();
            e.call(PRINT, &args![messages.owner_name, adjusted, name]);
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            e.call(PRINT, &args![messages.owner_id, adjusted, owner_id]);
        }
    }
}

// Translated from 0047b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPickupPutdownSounds::InitItem` (Xbox PDB; `RET 4`, `owner` the form
/// being loaded, possibly null): the file for the id adjustment is the
/// owner's (`TESForm::GetFile(-1)`, 0 without an owner), taken once. The
/// pickup sound, then the putdown sound, are each turned from a form id into
/// a `TESSound` (adjusted, looked up with `004839c0`, cast), and a sound that
/// could not be found prints a `MASTERFILE` message naming the owner:
/// unknown (null owner), by name (when `00474cb0` finds one) or by form id.
pub fn bgs_pickup_putdown_sounds_init_item(
    e: &mut Engine,
    this: Ptr<BGSPickupPutdownSounds>,
    owner: Ptr,
) {
    let file = if owner.is_null() {
        0
    } else {
        e.call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
            .u32()
    };
    init_sound(
        e,
        this,
        BGSPickupPutdownSounds::pPickupSound,
        owner,
        file,
        &PICKUP_MESSAGES,
    );
    init_sound(
        e,
        this,
        BGSPickupPutdownSounds::pPutdownSound,
        owner,
        file,
        &PUTDOWN_MESSAGES,
    );
}

// Translated from 0047b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPickupPutdownSounds::InitializeDataComponent` (Xbox PDB): no pickup
/// and no putdown sound.
pub fn bgs_pickup_putdown_sounds_initialize_data_component(
    e: &mut Engine,
    this: Ptr<BGSPickupPutdownSounds>,
) {
    e.set(this, BGSPickupPutdownSounds::pPickupSound, 0);
    e.set(this, BGSPickupPutdownSounds::pPutdownSound, 0);
}

/// The pickup-sound getter (`00726070`, the word at +4).
const GET_PICKUP_SOUND: u32 = 0x0072_6070;
/// The putdown-sound getter (`0044ddc0`, the word at +8).
const GET_PUTDOWN_SOUND: u32 = 0x0044_ddc0;
/// The pickup-sound setter (`006ecd40`, the word at +4; the engine map's
/// name `CombatProcedureAttackMelee::Initialize` is a folded body).
const SET_PICKUP_SOUND: u32 = 0x006e_cd40;
/// The putdown-sound setter (`00403550`, the word at +8).
const SET_PUTDOWN_SOUND: u32 = 0x0040_3550;
/// Writes a one-word chunk (`00485910`, cdecl: signature, word).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The `YNAM` and `ZNAM` chunk signatures, as the words the exe pushes.
const SIG_YNAM: u32 = 0x4d41_4e59;
const SIG_ZNAM: u32 = 0x4d41_4e5a;
/// The `BaseFormComponent` type descriptor (`.?AVBaseFormComponent@@`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// The `BGSPickupPutdownSounds` type descriptor
/// (`.?AVBGSPickupPutdownSounds@@`).
const TYPE_PICKUP_PUTDOWN_SOUNDS: u32 = 0x0118_32d4;

/// `other` cast from `BaseFormComponent` to `BGSPickupPutdownSounds`.
fn cast_component(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_PICKUP_PUTDOWN_SOUNDS,
            0u32
        ],
    )
    .u32()
}

// Translated from 0047b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPickupPutdownSounds::Save` (Xbox PDB): each sound there is (read
/// through its getter, twice as the exe does) is written as a one-word chunk
/// of its form id (`00485910`): the pickup sound as `YNAM`, the putdown sound
/// as `ZNAM`.
pub fn bgs_pickup_putdown_sounds_save(e: &mut Engine, this: Ptr<BGSPickupPutdownSounds>) {
    if e.call(GET_PICKUP_SOUND, &args![this]).u32() != 0 {
        let sound = e.call(GET_PICKUP_SOUND, &args![this]).u32();
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![SIG_YNAM, id]);
    }
    if e.call(GET_PUTDOWN_SOUND, &args![this]).u32() != 0 {
        let sound = e.call(GET_PUTDOWN_SOUND, &args![this]).u32();
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![SIG_ZNAM, id]);
    }
}

// Translated from 0047b5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPickupPutdownSounds::CopyComponent` (Xbox PDB; `RET 4`): when `other`
/// casts to a `BGSPickupPutdownSounds`, its two sounds are copied in through
/// the getters and setters.
pub fn bgs_pickup_putdown_sounds_copy_component(
    e: &mut Engine,
    this: Ptr<BGSPickupPutdownSounds>,
    other: Ptr,
) {
    let source = cast_component(e, other);
    if source == 0 {
        return;
    }
    let pickup = e.call(GET_PICKUP_SOUND, &args![source]).u32();
    e.call(SET_PICKUP_SOUND, &args![this, pickup]);
    let putdown = e.call(GET_PUTDOWN_SOUND, &args![source]).u32();
    e.call(SET_PUTDOWN_SOUND, &args![this, putdown]);
}

// Translated from 0047b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPickupPutdownSounds::CompareComponent` (Xbox PDB; `RET 4`): true
/// (different) when `other` is not a `BGSPickupPutdownSounds` or either sound
/// differs, false when both match.
pub fn bgs_pickup_putdown_sounds_compare_component(
    e: &mut Engine,
    this: Ptr<BGSPickupPutdownSounds>,
    other: Ptr,
) -> bool {
    let source = cast_component(e, other);
    if source == 0 {
        return true;
    }
    let mine = e.call(GET_PICKUP_SOUND, &args![this]).u32();
    if mine != e.call(GET_PICKUP_SOUND, &args![source]).u32() {
        return true;
    }
    let mine = e.call(GET_PUTDOWN_SOUND, &args![this]).u32();
    mine != e.call(GET_PUTDOWN_SOUND, &args![source]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047b6a0,
            bgs_pickup_putdown_sounds_init_item(Ptr<BGSPickupPutdownSounds>, Ptr)
        ),
        entry!(
            0x0047_b540,
            bgs_pickup_putdown_sounds_initialize_data_component(Ptr<BGSPickupPutdownSounds>)
        ),
        entry!(
            0x0047_b560,
            bgs_pickup_putdown_sounds_save(Ptr<BGSPickupPutdownSounds>)
        ),
        entry!(
            0x0047_b5d0,
            bgs_pickup_putdown_sounds_copy_component(Ptr<BGSPickupPutdownSounds>, Ptr)
        ),
        entry!(
            0x0047_b630,
            bgs_pickup_putdown_sounds_compare_component(Ptr<BGSPickupPutdownSounds>, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine where the cast answers `cast(form)` and the owner has a name
    /// of `name_length` characters.
    fn engine(cast: fn(u32) -> u32, name_length: u32) -> Engine {
        let mut e = Engine::new();
        e.register(TES_FORM_GET_FILE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORM_BY_ID, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register_double(RT_DYNAMIC_CAST, move |_, a| Ret {
            eax: cast(a[0]),
            ..Ret::default()
        });
        e.register_double(FORM_NAME_LENGTH, move |_, _| Ret {
            eax: name_length,
            ..Ret::default()
        });
        e.register(FORM_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(PRINT, |_, _| Ret::default());
        e.register(0x0050_0000, |_, _| Ret {
            eax: 0x0400_0000,
            ..Ret::default()
        });
        e
    }

    fn component(e: &mut Engine, pickup: u32, putdown: u32) -> u32 {
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 4, pickup);
        e.mem.set_u32(object + 8, putdown);
        object
    }

    #[test]
    fn both_sounds_are_resolved_with_the_owners_file() {
        let mut e = engine(|form| form, 0);
        let object = component(&mut e, 0x42, 0x43);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047b6a0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0x0100_0052);
        assert_eq!(e.mem.u32(object + 8), 0x0100_0053);
        assert_eq!(
            calls_to(&e, TES_FORM_GET_FILE),
            vec![vec![owner, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            vec![0x77, 0x77]
        );
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![
                vec![0x0100_0052, 0, TYPE_TES_FORM, TYPE_TES_SOUND, 0],
                vec![0x0100_0053, 0, TYPE_TES_FORM, TYPE_TES_SOUND, 0],
            ]
        );
        assert!(calls_to(&e, PRINT).is_empty());
    }

    #[test]
    fn a_sound_without_a_form_id_is_left_alone() {
        let mut e = engine(|form| form, 0);
        let object = component(&mut e, 0, 0x43);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047b6a0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0x0100_0053);
        assert_eq!(calls_to(&e, FORM_BY_ID).len(), 1);
    }

    #[test]
    fn missing_sounds_are_reported_for_an_unknown_owner_with_file_zero() {
        let mut e = engine(|_| 0, 0);
        let object = component(&mut e, 0x42, 0x43);
        e.call_log = Some(vec![]);
        e.call(0x0047b6a0, &args![object, 0u32]);
        assert_eq!((e.mem.u32(object + 4), e.mem.u32(object + 8)), (0, 0));
        assert!(calls_to(&e, TES_FORM_GET_FILE).is_empty());
        assert_eq!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)[1][1], 0);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![
                vec![PICKUP_MESSAGES.unknown_owner, 0x0100_0042],
                vec![PUTDOWN_MESSAGES.unknown_owner, 0x0100_0043],
            ]
        );
    }

    #[test]
    fn a_missing_sound_is_reported_by_the_owner_id_without_a_name() {
        let mut e = engine(|form| if form == 0x0100_0052 { form } else { 0 }, 0);
        let object = component(&mut e, 0x42, 0x43);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047b6a0, &args![object, owner]);
        assert_eq!(
            (e.mem.u32(object + 4), e.mem.u32(object + 8)),
            (0x0100_0052, 0)
        );
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![PUTDOWN_MESSAGES.owner_id, 0x0100_0043, 0x1234]]
        );
    }

    #[test]
    fn a_missing_sound_is_reported_by_the_owner_name_when_it_has_one() {
        let mut e = engine(|form| if form == 0x0100_0052 { 0 } else { form }, 5);
        let object = component(&mut e, 0x42, 0x43);
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.mem.set_u32(owner, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x0047b6a0, &args![object, owner]);
        assert_eq!(
            (e.mem.u32(object + 4), e.mem.u32(object + 8)),
            (0, 0x0100_0053)
        );
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![PICKUP_MESSAGES.owner_name, 0x0100_0042, 0x0400_0000]]
        );
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_0047b540_clears_both_sounds() {
        let mut e = Engine::new();
        let this = e.new_object::<BGSPickupPutdownSounds>();
        e.mem.set_u32(this.addr(), 0x77);
        e.mem.set_u32(this.addr() + 4, 1);
        e.mem.set_u32(this.addr() + 8, 2);
        e.call(0x0047_b540, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x77);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
    }
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Getters and setters as the exe's (+4, +8), form ids the form plus 1,
    /// and a cast that keeps even addresses.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_PICKUP_SOUND, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GET_PUTDOWN_SOUND, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SET_PICKUP_SOUND, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(SET_PUTDOWN_SOUND, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] & 1 == 0 { a[0] } else { 0 })
        });
        e.call_log = Some(vec![]);
        e
    }

    fn sounds(e: &mut Engine, pickup: u32, putdown: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, pickup);
        e.mem.set_u32(object + 8, putdown);
        object
    }

    #[test]
    fn save_writes_each_sound_there_is() {
        let mut e = engine();
        let both = sounds(&mut e, 0x500, 0x600);
        e.call(0x0047_b560, &[both]);
        let only_putdown = sounds(&mut e, 0, 0x700);
        e.call(0x0047_b560, &[only_putdown]);
        let none = sounds(&mut e, 0, 0);
        e.call(0x0047_b560, &[none]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_WORD),
            vec![
                vec![SIG_YNAM, 0x501],
                vec![SIG_ZNAM, 0x601],
                vec![SIG_ZNAM, 0x701]
            ]
        );
    }

    #[test]
    fn copy_component_takes_both_sounds_of_a_cast_component() {
        let mut e = engine();
        let this = sounds(&mut e, 1, 2);
        let other = sounds(&mut e, 0x500, 0x600);
        e.call(0x0047_b5d0, &[this, other + 1]);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![
                other + 1,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_PICKUP_PUTDOWN_SOUNDS,
                0
            ]]
        );
        e.call(0x0047_b5d0, &[this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x500);
        assert_eq!(e.mem.u32(this + 8), 0x600);
    }

    #[test]
    fn compare_component_says_whether_they_differ() {
        let mut e = engine();
        let this = sounds(&mut e, 0x500, 0x600);
        let same = sounds(&mut e, 0x500, 0x600);
        let other_pickup = sounds(&mut e, 0x501, 0x600);
        let other_putdown = sounds(&mut e, 0x500, 0x601);
        assert!(!e.call(0x0047_b630, &[this, same]).bool());
        assert!(e.call(0x0047_b630, &[this, other_pickup]).bool());
        assert!(e.call(0x0047_b630, &[this, other_putdown]).bool());
        assert!(e.call(0x0047_b630, &[this, same + 1]).bool());
    }
}

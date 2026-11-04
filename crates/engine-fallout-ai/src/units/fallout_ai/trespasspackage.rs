//! `fallout/ai/trespasspackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TrespassPackage` (a `TESPackage`; vtable `010939ec`, RTTI name
//! `.?AVTrespassPackage@@`): the package type 0x17 the game gives an actor
//! who catches someone trespassing. Its own fields follow `TESPackage`
//! (0x80 bytes on the PC, layout in `fallout_shared/package.rs`), 0x10 lower
//! than the Xbox PDB says.
//!
//! The functions at `009f9500`, `009f9570`, `009f9660` and `009f9720` are
//! the vtable's slots 0x14C, 0x150, 0x154 and 0x158 (the save size, the
//! form-style save and load, and the form id resolution after loading,
//! judged by their bodies; the Xbox PDB does not name these addresses).
//! Functions that open a C++ exception frame (`FS:[0]`) carry a
//! `Translation incomplete` marker: the unwinding states are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TrespassPackage` vtable (RTTI name confirmed).
const TRESPASS_PACKAGE_VTABLE: u32 = 0x0109_39ec;
/// `TESPackage::TESPackage` (Xbox PDB) `(this)` and the `TESPackage`
/// destructor body `006708b0` `(this)`.
const TES_PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
const TES_PACKAGE_DESTROY: u32 = 0x0067_08b0;
/// `TESPackage::SetPackType` (Xbox PDB) `(this, type)`.
const SET_PACK_TYPE: u32 = 0x0067_0fc0;
/// Two `(this, 1)` setters the constructor calls after the type (map:
/// `reanimateeffect.cpp`, folded code).
const PACKAGE_SETTER_826B40: u32 = 0x0082_6b40;
const PACKAGE_SETTER_826B90: u32 = 0x0082_6b90;
/// `operator new` (cdecl `(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `PackageLocation::PackageLocation` (Xbox PDB) `(this)`, 0xC bytes.
const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
/// `PackageLocation::SetLocReference` (Xbox PDB) `(this, reference)`.
const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
/// `PackageLocation::~PackageLocation` (Xbox PDB name of the deleting
/// destructor) `(this, 1)`.
const PACKAGE_LOCATION_DELETE: u32 = 0x0067_0b30;
/// `TESPackage::SetPackageLocation` (Xbox PDB) `(this, location)`.
const SET_PACKAGE_LOCATION: u32 = 0x0067_1d30;
/// `PackageTarget::PackageTarget` (Xbox PDB) `(this)`, 0x10 bytes.
const PACKAGE_TARGET_CONSTRUCT: u32 = 0x0067_ff70;
/// `TESPackage::SetPackageTarget` (Xbox PDB) `(this, target)`.
const SET_PACKAGE_TARGET: u32 = 0x0067_2fc0;
/// The target's deleting destructor `(this, 1)` (map: `racesexmenu.cpp`,
/// folded code).
const PACKAGE_TARGET_DELETE: u32 = 0x007b_3fa0;
/// `(this, 0x14)`: a package setter (unnamed in the map).
const PACKAGE_SETTER_984F60: u32 = 0x0098_4f60;
/// `(this)`: the package's target (`pPackTarg`).
const GET_PACKAGE_TARGET: u32 = 0x0067_1d10;
/// `PackageTarget::SetTargType` (Xbox PDB) `(this, type)`.
const TARGET_SET_TYPE: u32 = 0x0068_00b0;
/// `PackageTarget::SetTargReference` (Xbox PDB) `(this, reference)`.
const TARGET_SET_REFERENCE: u32 = 0x0068_0110;
/// `(this, value)`: stores a word in the target (folded one-word setter).
const TARGET_SET_WORD: u32 = 0x0040_3550;
/// The player (`PlayerCharacter*` global) and the destructor's
/// `005d43c0(player)` / `0041ce10(result, package)` pair.
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_PACKAGE_HOLDER: u32 = 0x005d_43c0;
const HOLDER_FORGET_PACKAGE: u32 = 0x0041_ce10;
/// `TESPackage`'s own save size `(this)` (a 16-bit size).
const TES_PACKAGE_SAVE_SIZE: u32 = 0x0067_8fd0;
/// The save/load game object and its version byte `(object)`.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
/// `TESPackage`'s form-style save `(this)`, load `(this)` and form id
/// resolution `(this)`.
const TES_PACKAGE_FORM_SAVE: u32 = 0x0067_9120;
const TES_PACKAGE_FORM_LOAD: u32 = 0x0067_9340;
const TES_PACKAGE_FORM_INIT_LOAD: u32 = 0x0067_96c0;
/// `TESForm::SaveGameDataOLD`, `LoadGameDataOLD`, `SaveNumericID`,
/// `LoadNumericID` (Xbox PDB) `(this, data, size)`.
const SAVE_DATA_OLD: u32 = 0x0048_4ce0;
const LOAD_DATA_OLD: u32 = 0x0048_4d00;
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
/// `(form)`: the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// cdecl `(form id)`: the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `TESPackage::SaveGame` (Xbox PDB) `(this, buffer)` and
/// `TESPackage::LoadGame_ov2` (Xbox PDB) `(this, buffer)`.
const TES_PACKAGE_SAVE_GAME: u32 = 0x0067_97c0;
const TES_PACKAGE_LOAD_GAME: u32 = 0x0067_98c0;
/// `BGSSaveGameBuffer` raw save `(this, data, size, 0)` and
/// `SaveFormID_ov2` (Xbox PDB) `(this, form, 0)`.
const SAVE_RAW: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer` raw load `(this, data, size)` and
/// `LoadFormID_ov2` (Xbox PDB) `(this, &word)`.
const LOAD_RAW: u32 = 0x0086_4980;
const LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;

layout! {
    /// `TrespassPackage` (Xbox PDB), 0xAC bytes on the Xbox, 0x9C on the PC
    /// (its fields 0x10 lower, after the PC `TESPackage`).
    pub struct TrespassPackage: 0x9C {
        /// Virtual function table pointer.
        0x00 vfptr: u32,
        /// `fwarningtimer` (Xbox PDB).
        0x80 fwarningtimer: f32,
        /// `iwarning` (Xbox PDB).
        0x84 iwarning: i32,
        /// `pOwner` (Xbox PDB): `TESForm*`.
        0x88 pOwner: Ptr,
        /// `pObjectOwned` (Xbox PDB): `TESForm*`.
        0x8C pObjectOwned: Ptr,
        /// `iCrimeNumber` (Xbox PDB).
        0x90 iCrimeNumber: i32,
        /// `iwarninglevel` (Xbox PDB).
        0x94 iwarninglevel: i32,
        /// `iNumberActors` (Xbox PDB).
        0x98 iNumberActors: i32,
    }
}

/// The fields the saves write as raw words, in order (the last two only
/// from save version 0x6C on in the form-style save).
const SAVED_WORDS: [u32; 5] = [0x80, 0x84, 0x90, 0x94, 0x98];

fn save_version(e: &mut Engine) -> u8 {
    let save_load_game: u32 = e.global(SAVE_LOAD_GAME);
    e.call(SAVE_VERSION, &args![save_load_game]).u8()
}

// Translated from 009f9250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f9250 - C++ exception frame (FS:[0] unwinding states) not translated
/// `TrespassPackage::TrespassPackage` (Xbox PDB): a package of type 0x17 at
/// `location` (a new `PackageLocation` on that reference, copied in and
/// deleted) whose target (a new `PackageTarget`, copied in and deleted) is
/// type 0 on `location` with the word 0x80; `owner`, `object_owned` and
/// `warning_level` are kept, the warning timer and count start at 0, the
/// crime number at -1. The location is set up through the allocation's
/// result even when it is null, as the game does.
pub fn trespass_package_construct(
    e: &mut Engine,
    this: Ptr<TrespassPackage>,
    location: Ptr,
    owner: Ptr,
    object_owned: Ptr,
    warning_level: i32,
) -> Ptr<TrespassPackage> {
    e.call(TES_PACKAGE_CONSTRUCT, &args![this]);
    e.set(this, TrespassPackage::vfptr, TRESPASS_PACKAGE_VTABLE);
    e.set(this, TrespassPackage::fwarningtimer, 0.0);
    e.set(this, TrespassPackage::iwarning, 0);
    e.set(this, TrespassPackage::pOwner, owner);
    e.set(this, TrespassPackage::pObjectOwned, object_owned);
    e.call(SET_PACK_TYPE, &args![this, 0x17u32]);
    e.call(PACKAGE_SETTER_826B40, &args![this, 1u32]);
    e.call(PACKAGE_SETTER_826B90, &args![this, 1u32]);

    let block = e.call(OPERATOR_NEW, &args![0x0cu32]).u32();
    let package_location = if block == 0 {
        0
    } else {
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![block]).u32()
    };
    e.call(
        PACKAGE_LOCATION_SET_REFERENCE,
        &args![package_location, location],
    );
    e.call(SET_PACKAGE_LOCATION, &args![this, package_location]);
    if package_location != 0 {
        e.call(PACKAGE_LOCATION_DELETE, &args![package_location, 1u32]);
    }

    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let target = if block == 0 {
        0
    } else {
        e.call(PACKAGE_TARGET_CONSTRUCT, &args![block]).u32()
    };
    e.call(SET_PACKAGE_TARGET, &args![this, target]);
    if target != 0 {
        e.call(PACKAGE_TARGET_DELETE, &args![target, 1u32]);
    }

    e.call(PACKAGE_SETTER_984F60, &args![this, 0x14u32]);
    let target = e.call(GET_PACKAGE_TARGET, &args![this]).u32();
    e.call(TARGET_SET_TYPE, &args![target, 0u32]);
    let target = e.call(GET_PACKAGE_TARGET, &args![this]).u32();
    e.call(TARGET_SET_REFERENCE, &args![target, location]);
    let target = e.call(GET_PACKAGE_TARGET, &args![this]).u32();
    e.call(TARGET_SET_WORD, &args![target, 0x80u32]);

    e.set(this, TrespassPackage::iCrimeNumber, -1);
    e.set(this, TrespassPackage::iwarninglevel, warning_level);
    e.set(this, TrespassPackage::iNumberActors, 0);
    this
}

// Translated from 009f9450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f9450 - C++ exception frame (FS:[0] unwinding states) not translated
/// The `TrespassPackage` destructor: sets its vtable; with a player, hands
/// the package to `0041ce10` on `005d43c0(player)`; then runs the
/// `TESPackage` destructor body (`006708b0`).
pub fn fn_009f9450(e: &mut Engine, this: Ptr<TrespassPackage>) {
    e.set(this, TrespassPackage::vfptr, TRESPASS_PACKAGE_VTABLE);
    let player: u32 = e.global(PLAYER);
    if player != 0 {
        let holder = e.call(PLAYER_PACKAGE_HOLDER, &args![player]).u32();
        e.call(HOLDER_FORGET_PACKAGE, &args![holder, this]);
    }
    e.call(TES_PACKAGE_DESTROY, &args![this]);
}

// Translated from 009f94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` to `iwarning`.
pub fn fn_009f94d0(e: &mut Engine, this: Ptr<TrespassPackage>, count: i32) {
    let warning = e.get(this, TrespassPackage::iwarning);
    e.set(this, TrespassPackage::iwarning, warning.wrapping_add(count));
}

// Translated from 009f9500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot 0x14C: the save size, `TESPackage`'s (`00678fd0`) plus 0xC
/// for the timer, warning count and crime number, 8 more from save version
/// 0x6C on (warning level, number of actors), plus 8 for the two form ids
/// (16-bit arithmetic).
pub fn fn_009f9500(e: &mut Engine, this: Ptr<TrespassPackage>) -> u16 {
    let base = e.call(TES_PACKAGE_SAVE_SIZE, &args![this]).u16();
    let mut size = base.wrapping_add(0x0c);
    if save_version(e) >= 0x6c {
        size = size.wrapping_add(4).wrapping_add(4);
    }
    size.wrapping_add(8)
}

// Translated from 009f9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot 0x150: `TESPackage`'s form-style save (`00679120`), then
/// the timer, warning count and crime number (and from save version 0x6C on
/// the warning level and number of actors) through
/// `TESForm::SaveGameDataOLD`, then the owner's and the owned object's form
/// ids (0 for none) through `TESForm::SaveNumericID`.
pub fn fn_009f9570(e: &mut Engine, this: Ptr<TrespassPackage>) {
    e.call(TES_PACKAGE_FORM_SAVE, &args![this]);
    for offset in &SAVED_WORDS[..3] {
        e.call(SAVE_DATA_OLD, &args![this, this.addr() + offset, 4u32]);
    }
    if save_version(e) >= 0x6c {
        for offset in &SAVED_WORDS[3..] {
            e.call(SAVE_DATA_OLD, &args![this, this.addr() + offset, 4u32]);
        }
    }
    e.with_stack(8, |e, locals| {
        // `[EBP-4]`: the owner's id, `[EBP-8]`: the owned object's id.
        let owner_id = locals.addr() + 4;
        let object_id = locals.addr();
        let owner = e.get(this, TrespassPackage::pOwner);
        let id = if owner.is_null() {
            0
        } else {
            e.call(FORM_ID, &args![owner]).u32()
        };
        e.mem.set_u32(owner_id, id);
        e.call(SAVE_NUMERIC_ID, &args![this, owner_id, 4u32]);
        let object_owned = e.get(this, TrespassPackage::pObjectOwned);
        let id = if object_owned.is_null() {
            0
        } else {
            e.call(FORM_ID, &args![object_owned]).u32()
        };
        e.mem.set_u32(object_id, id);
        e.call(SAVE_NUMERIC_ID, &args![this, object_id, 4u32]);
    });
}

// Translated from 009f9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot 0x154: the load matching `009f9570`: `TESPackage`'s
/// form-style load (`00679340`), the words through
/// `TESForm::LoadGameDataOLD` (the last two from save version 0x6C on),
/// then the two form ids through `TESForm::LoadNumericID` into `pOwner` and
/// `pObjectOwned` (resolved by slot 0x158).
pub fn fn_009f9660(e: &mut Engine, this: Ptr<TrespassPackage>) {
    e.call(TES_PACKAGE_FORM_LOAD, &args![this]);
    for offset in &SAVED_WORDS[..3] {
        e.call(LOAD_DATA_OLD, &args![this, this.addr() + offset, 4u32]);
    }
    if save_version(e) >= 0x6c {
        for offset in &SAVED_WORDS[3..] {
            e.call(LOAD_DATA_OLD, &args![this, this.addr() + offset, 4u32]);
        }
    }
    e.with_stack(8, |e, locals| {
        let owner_id = locals.addr() + 4;
        let object_id = locals.addr();
        e.call(LOAD_NUMERIC_ID, &args![this, owner_id, 4u32]);
        let id = e.mem.u32(owner_id);
        e.set(this, TrespassPackage::pOwner, Ptr::new(id));
        e.call(LOAD_NUMERIC_ID, &args![this, object_id, 4u32]);
        let id = e.mem.u32(object_id);
        e.set(this, TrespassPackage::pObjectOwned, Ptr::new(id));
    });
}

// Translated from 009f9720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot 0x158: `TESPackage`'s form id resolution (`006796c0`), then
/// the loaded owner and owned object ids (when not 0) become their forms.
pub fn fn_009f9720(e: &mut Engine, this: Ptr<TrespassPackage>) {
    e.call(TES_PACKAGE_FORM_INIT_LOAD, &args![this]);
    let owner = e.get(this, TrespassPackage::pOwner);
    if !owner.is_null() {
        let form = e.call(LOOKUP_FORM, &args![owner]).u32();
        e.set(this, TrespassPackage::pOwner, Ptr::new(form));
    }
    let object_owned = e.get(this, TrespassPackage::pObjectOwned);
    if !object_owned.is_null() {
        let form = e.call(LOOKUP_FORM, &args![object_owned]).u32();
        e.set(this, TrespassPackage::pObjectOwned, Ptr::new(form));
    }
}

// Translated from 009f9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TrespassPackage::SaveGame` (Xbox PDB): `TESPackage::SaveGame`, then the
/// timer, warning count, crime number, warning level and number of actors as
/// raw words, then the owner and the owned object as form ids.
pub fn trespass_package_save_game(e: &mut Engine, this: Ptr<TrespassPackage>, buffer: Ptr) {
    e.call(TES_PACKAGE_SAVE_GAME, &args![this, buffer]);
    for offset in SAVED_WORDS {
        e.call(SAVE_RAW, &args![buffer, this.addr() + offset, 4u32, 0u32]);
    }
    let owner = e.get(this, TrespassPackage::pOwner);
    e.call(SAVE_FORM_ID, &args![buffer, owner, 0u32]);
    let object_owned = e.get(this, TrespassPackage::pObjectOwned);
    e.call(SAVE_FORM_ID, &args![buffer, object_owned, 0u32]);
}

// Translated from 009f9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TrespassPackage::LoadGame` (Xbox PDB): `TESPackage::LoadGame_ov2`, then
/// the five raw words and the two form ids, as `SaveGame` wrote them.
pub fn trespass_package_load_game(e: &mut Engine, this: Ptr<TrespassPackage>, buffer: Ptr) {
    e.call(TES_PACKAGE_LOAD_GAME, &args![this, buffer]);
    for offset in SAVED_WORDS {
        e.call(LOAD_RAW, &args![buffer, this.addr() + offset, 4u32]);
    }
    e.call(LOAD_FORM_ID_INTO, &args![buffer, this.addr() + 0x88]);
    e.call(LOAD_FORM_ID_INTO, &args![buffer, this.addr() + 0x8c]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009f9250,
            trespass_package_construct(
                Ptr<TrespassPackage>,
                Ptr,
                Ptr,
                Ptr,
                i32,
            ) -> Ptr<TrespassPackage>
        ),
        entry!(0x009f9450, fn_009f9450(Ptr<TrespassPackage>)),
        entry!(0x009f94d0, fn_009f94d0(Ptr<TrespassPackage>, i32)),
        entry!(0x009f9500, fn_009f9500(Ptr<TrespassPackage>) -> u16),
        entry!(0x009f9570, fn_009f9570(Ptr<TrespassPackage>)),
        entry!(0x009f9660, fn_009f9660(Ptr<TrespassPackage>)),
        entry!(0x009f9720, fn_009f9720(Ptr<TrespassPackage>)),
        entry!(
            0x009f9790,
            trespass_package_save_game(Ptr<TrespassPackage>, Ptr)
        ),
        entry!(
            0x009f9840,
            trespass_package_load_game(Ptr<TrespassPackage>, Ptr)
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

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Doubles (answering 0) for every callee outside this file, and the
    /// globals' page.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            TES_PACKAGE_CONSTRUCT,
            TES_PACKAGE_DESTROY,
            SET_PACK_TYPE,
            PACKAGE_SETTER_826B40,
            PACKAGE_SETTER_826B90,
            OPERATOR_NEW,
            PACKAGE_LOCATION_CONSTRUCT,
            PACKAGE_LOCATION_SET_REFERENCE,
            PACKAGE_LOCATION_DELETE,
            SET_PACKAGE_LOCATION,
            PACKAGE_TARGET_CONSTRUCT,
            SET_PACKAGE_TARGET,
            PACKAGE_TARGET_DELETE,
            PACKAGE_SETTER_984F60,
            GET_PACKAGE_TARGET,
            TARGET_SET_TYPE,
            TARGET_SET_REFERENCE,
            TARGET_SET_WORD,
            PLAYER_PACKAGE_HOLDER,
            HOLDER_FORGET_PACKAGE,
            TES_PACKAGE_SAVE_SIZE,
            SAVE_VERSION,
            TES_PACKAGE_FORM_SAVE,
            TES_PACKAGE_FORM_LOAD,
            TES_PACKAGE_FORM_INIT_LOAD,
            SAVE_DATA_OLD,
            LOAD_DATA_OLD,
            SAVE_NUMERIC_ID,
            LOAD_NUMERIC_ID,
            FORM_ID,
            LOOKUP_FORM,
            TES_PACKAGE_SAVE_GAME,
            TES_PACKAGE_LOAD_GAME,
            SAVE_RAW,
            SAVE_FORM_ID,
            LOAD_RAW,
            LOAD_FORM_ID_INTO,
        ] {
            stub(&mut e, addr, 0);
        }
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5a5au32);
        e
    }

    #[test]
    fn constructor_sets_up_location_target_and_fields() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.register(OPERATOR_NEW, |_, a| ret(0x1_0000 + a[0]));
        e.register(PACKAGE_LOCATION_CONSTRUCT, |_, a| ret(a[0]));
        e.register(PACKAGE_TARGET_CONSTRUCT, |_, a| ret(a[0]));
        stub(&mut e, GET_PACKAGE_TARGET, 0x7a7a);
        e.mem.set_u32(this.addr() + 0x84, 9);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x009f_9250,
                &args![this, 0x1111u32, 0x2222u32, 0x3333u32, 2i32],
            )
            .u32();
        assert_eq!(back, this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_PACK_TYPE), vec![vec![this.addr(), 0x17]]);
        assert_eq!(
            calls(&log, PACKAGE_LOCATION_SET_REFERENCE),
            vec![vec![0x1_000c, 0x1111]]
        );
        assert_eq!(
            calls(&log, SET_PACKAGE_LOCATION),
            vec![vec![this.addr(), 0x1_000c]]
        );
        assert_eq!(
            calls(&log, PACKAGE_LOCATION_DELETE),
            vec![vec![0x1_000c, 1]]
        );
        assert_eq!(
            calls(&log, SET_PACKAGE_TARGET),
            vec![vec![this.addr(), 0x1_0010]]
        );
        assert_eq!(calls(&log, PACKAGE_TARGET_DELETE), vec![vec![0x1_0010, 1]]);
        assert_eq!(
            calls(&log, PACKAGE_SETTER_984F60),
            vec![vec![this.addr(), 0x14]]
        );
        assert_eq!(calls(&log, TARGET_SET_TYPE), vec![vec![0x7a7a, 0]]);
        assert_eq!(
            calls(&log, TARGET_SET_REFERENCE),
            vec![vec![0x7a7a, 0x1111]]
        );
        assert_eq!(calls(&log, TARGET_SET_WORD), vec![vec![0x7a7a, 0x80]]);
        assert_eq!(e.get(this, TrespassPackage::vfptr), TRESPASS_PACKAGE_VTABLE);
        assert_eq!(e.get(this, TrespassPackage::fwarningtimer), 0.0);
        assert_eq!(e.get(this, TrespassPackage::iwarning), 0);
        assert_eq!(e.get(this, TrespassPackage::pOwner).addr(), 0x2222);
        assert_eq!(e.get(this, TrespassPackage::pObjectOwned).addr(), 0x3333);
        assert_eq!(e.get(this, TrespassPackage::iCrimeNumber), -1);
        assert_eq!(e.get(this, TrespassPackage::iwarninglevel), 2);
        assert_eq!(e.get(this, TrespassPackage::iNumberActors), 0);

        // Failed allocations: the null location still gets its reference,
        // nothing is deleted.
        stub(&mut e, OPERATOR_NEW, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x009f_9250,
            &args![this, 0x1111u32, 0x2222u32, 0x3333u32, 2i32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PACKAGE_LOCATION_SET_REFERENCE),
            vec![vec![0, 0x1111]]
        );
        assert!(calls(&log, PACKAGE_LOCATION_DELETE).is_empty());
        assert!(calls(&log, PACKAGE_TARGET_DELETE).is_empty());
        assert!(calls(&log, PACKAGE_LOCATION_CONSTRUCT).is_empty());
    }

    #[test]
    fn destructor_lets_the_player_forget_the_package() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x009f_9450, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, PLAYER_PACKAGE_HOLDER).is_empty());
        assert_eq!(calls(&log, TES_PACKAGE_DESTROY), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, TrespassPackage::vfptr), TRESPASS_PACKAGE_VTABLE);

        e.set_global(PLAYER, 0x9090u32);
        stub(&mut e, PLAYER_PACKAGE_HOLDER, 0x4545);
        e.call_log = Some(vec![]);
        e.call(0x009f_9450, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PLAYER_PACKAGE_HOLDER), vec![vec![0x9090]]);
        assert_eq!(
            calls(&log, HOLDER_FORGET_PACKAGE),
            vec![vec![0x4545, this.addr()]]
        );
    }

    #[test]
    fn add_warnings_adds_to_the_count() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.set(this, TrespassPackage::iwarning, 3);
        e.call(0x009f_94d0, &args![this, -5i32]);
        assert_eq!(e.get(this, TrespassPackage::iwarning), -2);
    }

    #[test]
    fn save_size_depends_on_the_save_version() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        stub(&mut e, TES_PACKAGE_SAVE_SIZE, 0x100);
        stub(&mut e, SAVE_VERSION, 0x6b);
        assert_eq!(e.call(0x009f_9500, &args![this]).u16(), 0x114);
        stub(&mut e, SAVE_VERSION, 0x6c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_9500, &args![this]).u16(), 0x11c);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SAVE_VERSION), vec![vec![0x5a5a]]);
        // 16-bit wrap.
        stub(&mut e, TES_PACKAGE_SAVE_SIZE, 0xfff0);
        assert_eq!(e.call(0x009f_9500, &args![this]).u16(), 0x000c);
    }

    #[test]
    fn form_save_writes_the_words_and_the_ids() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.set(this, TrespassPackage::pOwner, Ptr::new(0x4000));
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(SAVE_NUMERIC_ID, |e, a| {
            assert_eq!(a[2], 4);
            // Keep the id value where the test can see it.
            let id = e.mem.u32(a[1]);
            ret(id)
        });
        stub(&mut e, SAVE_VERSION, 0x6b);
        e.call_log = Some(vec![]);
        e.call(0x009f_9570, &args![this]);
        let log = e.call_log.take().unwrap();
        let a = this.addr();
        assert_eq!(
            calls(&log, SAVE_DATA_OLD),
            vec![
                vec![a, a + 0x80, 4],
                vec![a, a + 0x84, 4],
                vec![a, a + 0x90, 4]
            ]
        );
        let ids = calls(&log, SAVE_NUMERIC_ID);
        assert_eq!(ids.len(), 2);
        // The owner's id word is 4 bytes above the owned object's.
        assert_eq!(ids[0][1], ids[1][1] + 4);
        assert_eq!(calls(&log, FORM_ID), vec![vec![0x4000]]);

        stub(&mut e, SAVE_VERSION, 0x6c);
        e.call_log = Some(vec![]);
        e.call(0x009f_9570, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SAVE_DATA_OLD).len(), 5);
        assert_eq!(calls(&log, SAVE_DATA_OLD)[4], vec![a, a + 0x98, 4]);
    }

    #[test]
    fn form_load_reads_the_words_and_the_ids() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        stub(&mut e, SAVE_VERSION, 0x6c);
        e.register_double(LOAD_NUMERIC_ID, {
            let mut next = 0x10u32;
            move |e, a| {
                e.mem.set_u32(a[1], next);
                next += 1;
                Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_9660, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, TES_PACKAGE_FORM_LOAD), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, LOAD_DATA_OLD).len(), 5);
        assert_eq!(e.get(this, TrespassPackage::pOwner).addr(), 0x10);
        assert_eq!(e.get(this, TrespassPackage::pObjectOwned).addr(), 0x11);

        stub(&mut e, SAVE_VERSION, 0);
        e.call_log = Some(vec![]);
        e.call(0x009f_9660, &args![this]);
        assert_eq!(calls(&e.call_log.take().unwrap(), LOAD_DATA_OLD).len(), 3);
    }

    #[test]
    fn form_init_load_resolves_non_zero_ids() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.set(this, TrespassPackage::pOwner, Ptr::new(0x0001_0001));
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x100));
        e.call_log = Some(vec![]);
        e.call(0x009f_9720, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOOKUP_FORM), vec![vec![0x0001_0001]]);
        assert_eq!(e.get(this, TrespassPackage::pOwner).addr(), 0x0001_0101);
        assert!(e.get(this, TrespassPackage::pObjectOwned).is_null());
    }

    #[test]
    fn save_game_writes_words_then_form_ids() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.set(this, TrespassPackage::pOwner, Ptr::new(0x4000));
        e.set(this, TrespassPackage::pObjectOwned, Ptr::new(0x5000));
        e.call_log = Some(vec![]);
        e.call(0x009f_9790, &args![this, 0xb0b0u32]);
        let log = e.call_log.take().unwrap();
        let a = this.addr();
        assert_eq!(calls(&log, TES_PACKAGE_SAVE_GAME), vec![vec![a, 0xb0b0]]);
        let raw = calls(&log, SAVE_RAW);
        let offsets: Vec<u32> = raw.iter().map(|w| w[1] - a).collect();
        assert_eq!(offsets, vec![0x80, 0x84, 0x90, 0x94, 0x98]);
        assert!(raw.iter().all(|w| w[0] == 0xb0b0 && w[2] == 4 && w[3] == 0));
        assert_eq!(
            calls(&log, SAVE_FORM_ID),
            vec![vec![0xb0b0, 0x4000, 0], vec![0xb0b0, 0x5000, 0]]
        );
    }

    #[test]
    fn load_game_reads_words_then_form_ids() {
        let mut e = engine();
        let this: Ptr<TrespassPackage> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x009f_9840, &args![this, 0xb0b0u32]);
        let log = e.call_log.take().unwrap();
        let a = this.addr();
        assert_eq!(calls(&log, TES_PACKAGE_LOAD_GAME), vec![vec![a, 0xb0b0]]);
        let raw = calls(&log, LOAD_RAW);
        let offsets: Vec<u32> = raw.iter().map(|w| w[1] - a).collect();
        assert_eq!(offsets, vec![0x80, 0x84, 0x90, 0x94, 0x98]);
        assert_eq!(
            calls(&log, LOAD_FORM_ID_INTO),
            vec![vec![0xb0b0, a + 0x88], vec![0xb0b0, a + 0x8c]]
        );
    }
}

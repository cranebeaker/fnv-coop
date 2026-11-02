//! `fallout/ai/guardpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `GuardActorPackageData` (0x14 bytes, an `ActorPackageData`) is the run-time
//! state of a guard package: the guard state, the actor it warned, the warning
//! count and the guard timer. Its vtable is at `0x010933c8` (the RTTI locator
//! before it names `GuardActorPackageData`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The class's vtable.
const VTABLE: u32 = 0x0109_33c8;
/// `ActorPackageData`'s destructor body.
const BASE_DESTRUCT: u32 = 0x0067_d970;
/// `operator delete` (`block`).
const FREE: u32 = 0x0040_1030;
/// `BGSSaveGameBuffer` writer of raw bytes: `(this = buffer, data, size, 0)`.
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer` writer of a reference's form id: `(this = buffer, reference, 0)`.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer` reader of raw bytes: `(this = buffer, destination, size)`.
const LOAD_BYTES: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer` reader of a form id into a reference slot: `(this = buffer, slot)`.
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// Maps a form pointer to the object a dynamic cast starts from (one cdecl
/// argument).
const CAST_SOURCE: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (cdecl: object, vfptr delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors of the cast in `InitLoadGame` (source, target).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_46d4;
/// `TESObjectREFR::SetTargeted` (`this` = reference, flag).
const SET_TARGETED: u32 = 0x0056_4db0;

layout! {
    /// `GuardActorPackageData` (Xbox PDB), 0x14 bytes (the same on PC).
    pub struct GuardActorPackageData: 0x14 {
        /// `eState` (Xbox PDB): `GuardActorPackageData::GUARD_STATE`.
        0x04 eState: u32,
        /// `pGuardWarnedActor` (Xbox PDB): `Actor*`.
        0x08 pGuardWarnedActor: Ptr,
        /// `iWarningCount` (Xbox PDB).
        0x0C iWarningCount: i32,
        /// `fGuardTimer` (Xbox PDB).
        0x10 fGuardTimer: f32,
    }
}

// Translated from 008c5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData::HandleDeletedReference` (Xbox PDB): forgets the
/// warned actor when it is the reference being deleted.
pub fn guard_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<GuardActorPackageData>,
    deleted: u32,
) {
    if deleted == e.get(this, GuardActorPackageData::pGuardWarnedActor).addr() {
        e.set(this, GuardActorPackageData::pGuardWarnedActor, Ptr::new(0));
    }
}

// Translated from 009f27f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData::GetPackageType` (Xbox PDB): the package type, 14.
pub fn guard_actor_package_data_get_package_type(
    _e: &mut Engine,
    _this: Ptr<GuardActorPackageData>,
) -> u32 {
    0xe
}

// Translated from 009f2830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (unnamed in the Xbox PDB): sets the vtable and runs
/// `ActorPackageData`'s destructor body.
pub fn fn_009f2830(e: &mut Engine, this: Ptr<GuardActorPackageData>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 009f2800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData` deleting destructor (vtable slot 0; unnamed in the
/// Xbox PDB): runs the destructor body and, with bit 0 of `flags` set, frees
/// the object.
pub fn guard_actor_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<GuardActorPackageData>,
    flags: u32,
) -> Ptr<GuardActorPackageData> {
    fn_009f2830(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009f2850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData::SaveGame` (Xbox PDB): writes the state, the timer
/// and the warning count as four bytes each, then the warned actor's form id.
pub fn guard_actor_package_data_save_game(
    e: &mut Engine,
    this: Ptr<GuardActorPackageData>,
    buffer: u32,
) {
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 4, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x10, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0xc, 4u32, 0u32]);
    let warned = e.get(this, GuardActorPackageData::pGuardWarnedActor);
    e.call(SAVE_FORM_ID, &args![buffer, warned, 0u32]);
}

// Translated from 009f28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData::LoadGame` (Xbox PDB): reads the fields in the order
/// `SaveGame` wrote them; the form id is read into the warned-actor slot.
pub fn guard_actor_package_data_load_game(
    e: &mut Engine,
    this: Ptr<GuardActorPackageData>,
    buffer: u32,
) {
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 4, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x10, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0xc, 4u32]);
    e.call(LOAD_FORM_ID, &args![buffer, this.addr() + 8]);
}

// Translated from 009f2900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GuardActorPackageData::InitLoadGame` (Xbox PDB): after loading, converts
/// the warned-actor slot to the object a dynamic cast gives (null stays null)
/// and marks a non-null actor as targeted.
/// The stack word (`RET 4`) is not read.
pub fn guard_actor_package_data_init_load_game(
    e: &mut Engine,
    this: Ptr<GuardActorPackageData>,
    _unused_1: u32,
) {
    let warned = e.get(this, GuardActorPackageData::pGuardWarnedActor);
    let converted = if warned.is_null() {
        0
    } else {
        let source = e.call(CAST_SOURCE, &args![warned]).u32();
        e.call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32()
    };
    e.set(
        this,
        GuardActorPackageData::pGuardWarnedActor,
        Ptr::new(converted),
    );
    if converted != 0 {
        e.call(SET_TARGETED, &args![converted, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c5ec0,
            guard_actor_package_data_handle_deleted_reference(Ptr<GuardActorPackageData>, u32)
        ),
        entry!(
            0x009f27f0,
            guard_actor_package_data_get_package_type(Ptr<GuardActorPackageData>) -> u32
        ),
        entry!(
            0x009f2800,
            guard_actor_package_data_scalar_deleting_destructor(
                Ptr<GuardActorPackageData>,
                u32,
            )
                -> Ptr<GuardActorPackageData>
        ),
        entry!(0x009f2830, fn_009f2830(Ptr<GuardActorPackageData>)),
        entry!(
            0x009f2850,
            guard_actor_package_data_save_game(Ptr<GuardActorPackageData>, u32)
        ),
        entry!(
            0x009f28b0,
            guard_actor_package_data_load_game(Ptr<GuardActorPackageData>, u32)
        ),
        entry!(
            0x009f2900,
            guard_actor_package_data_init_load_game(Ptr<GuardActorPackageData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn guard(e: &mut Engine) -> Ptr<GuardActorPackageData> {
        e.new_object::<GuardActorPackageData>()
    }

    #[test]
    fn deleted_reference_clears_only_the_warned_actor() {
        let mut e = Engine::new();
        let g = guard(&mut e);
        e.set(
            g,
            GuardActorPackageData::pGuardWarnedActor,
            Ptr::<()>::new(0x500),
        );
        e.call(0x008c5ec0, &args![g, 0x600u32]);
        assert_eq!(
            e.get(g, GuardActorPackageData::pGuardWarnedActor).addr(),
            0x500
        );
        e.call(0x008c5ec0, &args![g, 0x500u32]);
        assert_eq!(e.get(g, GuardActorPackageData::pGuardWarnedActor).addr(), 0);
    }

    #[test]
    fn package_type_is_fourteen() {
        let mut e = Engine::new();
        let g = guard(&mut e);
        assert_eq!(e.call(0x009f27f0, &args![g]).u32(), 14);
    }

    #[test]
    fn destructor_body_sets_vtable_then_runs_base() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        let g = guard(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009f2830, &args![g]);
        assert_eq!(e.mem.u32(g.addr()), 0x0109_33c8);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_DESTRUCT, vec![g.addr()]));
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let g = guard(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f2800, &args![g, 0u32]).u32(), g.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != FREE));
        e.call_log = Some(vec![]);
        e.call(0x009f2800, &args![g, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(FREE, vec![g.addr()]));
    }

    #[test]
    fn save_writes_state_timer_count_then_the_form_id() {
        let mut e = Engine::new();
        e.register(SAVE_BYTES, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        let g = guard(&mut e);
        e.set(
            g,
            GuardActorPackageData::pGuardWarnedActor,
            Ptr::<()>::new(0x700),
        );
        e.call_log = Some(vec![]);
        e.call(0x009f2850, &args![g, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SAVE_BYTES, vec![0x9000, g.addr() + 4, 4, 0]),
                (SAVE_BYTES, vec![0x9000, g.addr() + 0x10, 4, 0]),
                (SAVE_BYTES, vec![0x9000, g.addr() + 0xc, 4, 0]),
                (SAVE_FORM_ID, vec![0x9000, 0x700, 0]),
            ]
        );
    }

    #[test]
    fn load_reads_in_the_save_order() {
        let mut e = Engine::new();
        e.register(LOAD_BYTES, |_, _| Ret::default());
        e.register(LOAD_FORM_ID, |_, _| Ret::default());
        let g = guard(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009f28b0, &args![g, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LOAD_BYTES, vec![0x9000, g.addr() + 4, 4]),
                (LOAD_BYTES, vec![0x9000, g.addr() + 0x10, 4]),
                (LOAD_BYTES, vec![0x9000, g.addr() + 0xc, 4]),
                (LOAD_FORM_ID, vec![0x9000, g.addr() + 8]),
            ]
        );
    }

    #[test]
    fn init_load_game_with_no_actor_does_nothing_else() {
        let mut e = Engine::new();
        let g = guard(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009f2900, &args![g, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "no other call");
        assert_eq!(e.get(g, GuardActorPackageData::pGuardWarnedActor).addr(), 0);
    }

    #[test]
    fn init_load_game_casts_and_marks_the_actor_targeted() {
        let mut e = Engine::new();
        e.register(CAST_SOURCE, |_, a| ret(a[0] + 0x10));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 0x100));
        e.register(SET_TARGETED, |_, _| Ret::default());
        let g = guard(&mut e);
        e.set(
            g,
            GuardActorPackageData::pGuardWarnedActor,
            Ptr::<()>::new(0x1000),
        );
        e.call_log = Some(vec![]);
        e.call(0x009f2900, &args![g, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CAST_SOURCE, vec![0x1000]),
                (DYNAMIC_CAST, vec![0x1010, 0, 0x0118_3028, 0x0118_46d4, 0]),
                (SET_TARGETED, vec![0x1110, 1]),
            ]
        );
        assert_eq!(
            e.get(g, GuardActorPackageData::pGuardWarnedActor).addr(),
            0x1110
        );
    }

    #[test]
    fn init_load_game_failed_cast_clears_the_actor_without_targeting() {
        let mut e = Engine::new();
        e.register(CAST_SOURCE, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        let g = guard(&mut e);
        e.set(
            g,
            GuardActorPackageData::pGuardWarnedActor,
            Ptr::<()>::new(0x1000),
        );
        e.call_log = Some(vec![]);
        e.call(0x009f2900, &args![g, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != SET_TARGETED));
        assert_eq!(e.get(g, GuardActorPackageData::pGuardWarnedActor).addr(), 0);
    }
}

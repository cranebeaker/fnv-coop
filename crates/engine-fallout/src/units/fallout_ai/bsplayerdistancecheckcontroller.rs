//! `fallout/ai/bsplayerdistancecheckcontroller.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSPlayerDistanceCheckController` is an `NiTimeController` (0x3c bytes on
//! PC and Xbox) that watches the distance between the object it is attached to
//! and the global object at `0x011dea3c`, and calls a callback when it is
//! closer than `fCheckDistance`.
//!
//! x87 note: the float tests below use the exe's flag tests exactly (a NaN
//! distance passes the "not negative" test and fails the "closer" test).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global pointer (`0x011dea3c`) to the object the controller measures
/// the distance to; its virtual slot `0x1f4` returns a pointer to that
/// object's position.
const DISTANCE_REFERENCE: u32 = 0x011d_ea3c;
/// `0.0` (`double`), what `fCheckDistance` is compared with.
const ZERO: u32 = 0x0101_2060;
/// The `NiRTTI` of this class (`GetRTTI`'s result).
const RTTI: u32 = 0x011d_feac;
/// The class's vtable.
const VTABLE: u32 = 0x0108_67fc;
/// The `NiRTTI` the target is cast to (second word of the cast helper
/// `00653270`, which returns 0 for a null object).
const TARGET_TYPE: u32 = 0x011f_4280;
/// Size of the object (`operator new`/`delete` size).
const SIZE: u32 = 0x3c;

/// Returns the pointer stored at `+0x38` of the object (the callback getter).
const GET_CALLBACK: u32 = 0x009e_32d0;
/// Stores the word at `+0x38` (the callback setter).
const SET_CALLBACK: u32 = 0x0040_e500;
/// Returns `fCheckDistance` (`float` at `+0x34`, in ST0).
const GET_CHECK_DISTANCE: u32 = 0x0050_8120;
/// Stores `fCheckDistance`.
const SET_CHECK_DISTANCE: u32 = 0x0092_9240;
/// `NiTimeController`'s destructor body.
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// `NiTimeController`'s clone-copy step (`this` = source; destination, cloning process).
const TIME_CONTROLLER_COPY: u32 = 0x00a6_cfe0;
/// The cast helper `(type, object) -> object or 0`.
const CAST_TARGET: u32 = 0x0065_3270;
/// Returns `object + 0x8c` (the position the cast object keeps there).
const POSITION_OF: u32 = 0x0045_bb80;
/// `(this = left, out, right)`: `out = left - right` for 3-vectors.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// Length of the vector at `this` (ST0).
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `(type, object) -> bool` (null object gives false).
const IS_TARGET_TYPE: u32 = 0x0043_b300;
/// Allocator used by `operator new` (size argument).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `operator delete` with size (`block, size`).
const FREE_SIZED: u32 = 0x00aa_1460;

layout! {
    /// `BSPlayerDistanceCheckController` (Xbox PDB), 0x3c bytes; only the
    /// fields used so far.
    pub struct BSPlayerDistanceCheckController: 0x3c {
        /// `NiTimeController::m_pkTarget` (Xbox PDB).
        0x2c m_pkTarget: Ptr,
        /// `fCheckDistance` (Xbox PDB).
        0x34 fCheckDistance: f32,
        /// `pfnCallback` (Xbox PDB): called with the cast target.
        0x38 pfnCallback: u32,
    }
}

// Translated from 008d1610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPlayerDistanceCheckController::GetRTTI` (Xbox PDB): the address of the
/// class's `NiRTTI`.
pub fn bs_player_distance_check_controller_get_rtti(
    _e: &mut Engine,
    _this: Ptr<BSPlayerDistanceCheckController>,
) -> u32 {
    RTTI
}

// Translated from 008d1650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (unnamed in the Xbox PDB): sets the vtable and runs
/// `NiTimeController`'s destructor body.
pub fn fn_008d1650(e: &mut Engine, this: Ptr<BSPlayerDistanceCheckController>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 008d1620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPlayerDistanceCheckController::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor body and, when bit 0 of `flags` is set, frees the
/// 0x3c-byte object.
pub fn bs_player_distance_check_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSPlayerDistanceCheckController>,
    flags: u32,
) -> Ptr<BSPlayerDistanceCheckController> {
    fn_008d1650(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, SIZE]);
    }
    this
}

// Translated from 008d1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPlayerDistanceCheckController::Update` (Xbox PDB): when the check
/// distance is not negative, a callback is set, the global reference object
/// exists and the target casts to the expected type, measures the distance
/// from the reference object's position to the target's position and calls
/// the callback with the target if `fCheckDistance` is greater than it.
///
/// The time argument is never read. The first test is the exe's
/// `FCOMP; TEST AH,5; JNP`: only "less than zero" leaves, so NaN goes on; the
/// last is `TEST AH,0x41; JNZ`: "less, equal or unordered" leaves.
pub fn bs_player_distance_check_controller_update(
    e: &mut Engine,
    this: Ptr<BSPlayerDistanceCheckController>,
    _unused_1: f32,
) {
    let check_distance = e.call(GET_CHECK_DISTANCE, &args![this]).f32() as f64;
    let zero: f64 = e.global(ZERO);
    if check_distance < zero {
        return;
    }
    if e.call(GET_CALLBACK, &args![this]).u32() == 0 {
        return;
    }
    let reference: u32 = e.global(DISTANCE_REFERENCE);
    if reference == 0 {
        return;
    }
    let target = e.get(this, BSPlayerDistanceCheckController::m_pkTarget);
    let target = e.call(CAST_TARGET, &args![TARGET_TYPE, target]).u32();
    if target == 0 {
        return;
    }
    let target_position = e.call(POSITION_OF, &args![target]).u32();
    let distance = e.with_stack(12, |e, offset| {
        // Slot 0x1f4 of the reference object returns the pointer used as the
        // left operand of the subtraction; it takes no stack arguments.
        let reference_position = e.vcall(reference, 0x1f4, &args![]).u32();
        e.call(
            VECTOR_SUBTRACT,
            &args![reference_position, offset, target_position],
        );
        e.call(VECTOR_LENGTH, &args![offset]).f32() as f64
    });
    let check_distance = e.call(GET_CHECK_DISTANCE, &args![this]).f32() as f64;
    if check_distance > distance {
        let callback = e.get(this, BSPlayerDistanceCheckController::pfnCallback);
        e.call(callback, &args![target]);
    }
}

// Translated from 008d1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPlayerDistanceCheckController::TargetIsRequiredType` (Xbox PDB): whether
/// the target is an object of the expected type (`false` for no target).
pub fn bs_player_distance_check_controller_target_is_required_type(
    e: &mut Engine,
    this: Ptr<BSPlayerDistanceCheckController>,
) -> bool {
    let target = e.get(this, BSPlayerDistanceCheckController::m_pkTarget);
    e.call(IS_TARGET_TYPE, &args![TARGET_TYPE, target]).bool()
}

// Translated from 008d17d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The clone-copy step of `CreateClone` (unnamed in the Xbox PDB): copies
/// the callback and the check distance of `this` into `clone`, then runs
/// `NiTimeController`'s copy step with the cloning process.
pub fn fn_008d17d0(
    e: &mut Engine,
    this: Ptr<BSPlayerDistanceCheckController>,
    clone: Ptr<BSPlayerDistanceCheckController>,
    cloning_process: u32,
) {
    let callback = e.call(GET_CALLBACK, &args![this]).u32();
    e.call(SET_CALLBACK, &args![clone, callback]);
    let check_distance = e.call(GET_CHECK_DISTANCE, &args![this]).f32();
    e.call(SET_CHECK_DISTANCE, &args![clone, check_distance]);
    e.call(TIME_CONTROLLER_COPY, &args![this, clone, cloning_process]);
}

// Translated from 008d1740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d1740 - the compiler's exception frame (FS:[0], state 0 around the constructor, which frees the allocation if the constructor throws) is not translated.
/// `BSPlayerDistanceCheckController::CreateClone` (Xbox PDB): allocates a
/// 0x3c-byte object, constructs it (`008d15d0`) when the allocation succeeded
/// and copies this controller's fields into it (`fn_008d17d0`, which is called
/// even with a null clone, as the exe does). Returns the clone.
pub fn bs_player_distance_check_controller_create_clone(
    e: &mut Engine,
    this: Ptr<BSPlayerDistanceCheckController>,
    cloning_process: u32,
) -> Ptr<BSPlayerDistanceCheckController> {
    let block = e.call(ALLOCATE, &args![SIZE]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(0x008d_15d0, &args![block]).u32()
    };
    let clone = Ptr::new(clone);
    fn_008d17d0(e, this, clone, cloning_process);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008d1610,
            bs_player_distance_check_controller_get_rtti(
                Ptr<BSPlayerDistanceCheckController>,
            ) -> u32
        ),
        entry!(
            0x008d1620,
            bs_player_distance_check_controller_scalar_deleting_destructor(
                Ptr<BSPlayerDistanceCheckController>,
                u32,
            ) -> Ptr<
                BSPlayerDistanceCheckController,
            >
        ),
        entry!(
            0x008d1650,
            fn_008d1650(Ptr<BSPlayerDistanceCheckController>)
        ),
        entry!(
            0x008d1670,
            bs_player_distance_check_controller_update(Ptr<BSPlayerDistanceCheckController>, f32)
        ),
        entry!(
            0x008d1720,
            bs_player_distance_check_controller_target_is_required_type(
                Ptr<BSPlayerDistanceCheckController>,
            ) -> bool
        ),
        entry!(
            0x008d1740,
            bs_player_distance_check_controller_create_clone(
                Ptr<BSPlayerDistanceCheckController>,
                u32,
            ) -> Ptr<
                BSPlayerDistanceCheckController,
            >
        ),
        entry!(
            0x008d17d0,
            fn_008d17d0(
                Ptr<BSPlayerDistanceCheckController>,
                Ptr<BSPlayerDistanceCheckController>,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The targets the callback was called with.
    type Received = std::rc::Rc<std::cell::RefCell<Vec<u32>>>;

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn controller(e: &mut Engine) -> Ptr<BSPlayerDistanceCheckController> {
        e.new_object::<BSPlayerDistanceCheckController>()
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        let c = controller(&mut e);
        assert_eq!(
            e.call(0x008d1610, &args![c]).u32(),
            0x011d_feac,
            "the NiRTTI address"
        );
    }

    #[test]
    fn destructor_sets_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let c = controller(&mut e);
        e.call(0x008d1650, &args![c]);
        assert_eq!(e.mem.u32(c.addr()), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (TIME_CONTROLLER_DESTRUCT, vec![c.addr()]));
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(FREE_SIZED, |_, _| Ret::default());
        let c = controller(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x008d1620, &args![c, 2u32]).u32();
        assert_eq!(r, c.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != FREE_SIZED));
        e.call_log = Some(vec![]);
        e.call(0x008d1620, &args![c, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(FREE_SIZED, vec![c.addr(), 0x3c]));
    }

    /// Doubles for the accessors, the cast and the vector helpers, a
    /// reference object whose slot `0x1f4` returns the position at
    /// `reference + 0x40`, and a target with its position at `+0x8c`.
    /// Returns (controller, target, callbacks received).
    fn update_rig(e: &mut Engine) -> (Ptr<BSPlayerDistanceCheckController>, u32, Received) {
        e.map(DISTANCE_REFERENCE, 4);
        e.map(ZERO, 8);
        e.register(GET_CHECK_DISTANCE, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x34) as f64,
            ..Ret::default()
        });
        e.register(GET_CALLBACK, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        // The cast helper returns the object unless it is null.
        e.register(CAST_TARGET, |_, a| ret(a[1]));
        e.register(POSITION_OF, |_, a| ret(a[0] + 0x8c));
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret(a[1])
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let sq: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
            Ret {
                st0: sq.sqrt() as f64,
                ..Ret::default()
            }
        });
        let reference = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(reference, table);
        e.mem.set_u32(table + 0x1f4, 0x7000_01f4);
        e.register(0x7000_01f4, |_, a| ret(a[0] + 0x40));
        e.set_global(DISTANCE_REFERENCE, reference);
        let c = controller(e);
        let target = e.mem.alloc(0x100);
        e.set(
            c,
            BSPlayerDistanceCheckController::m_pkTarget,
            Ptr::new(target),
        );
        e.set(c, BSPlayerDistanceCheckController::fCheckDistance, 10.0f32);
        e.set(c, BSPlayerDistanceCheckController::pfnCallback, 0x7000_cb00);
        let received = Received::default();
        let sink = received.clone();
        e.register_double(0x7000_cb00, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        // Reference at the origin, target 3-4-0 away: distance 5.
        e.mem.set_f32(target + 0x8c, 3.0);
        e.mem.set_f32(target + 0x90, 4.0);
        (c, target, received)
    }

    #[test]
    fn update_calls_back_when_closer_than_the_check_distance() {
        let mut e = Engine::new();
        let (c, target, received) = update_rig(&mut e);
        e.call(0x008d1670, &args![c, 0.0f32]);
        assert_eq!(*received.borrow(), vec![target]);
    }

    #[test]
    fn update_does_nothing_when_farther_or_equal() {
        let mut e = Engine::new();
        let (c, _, received) = update_rig(&mut e);
        e.set(c, BSPlayerDistanceCheckController::fCheckDistance, 5.0f32);
        e.call(0x008d1670, &args![c, 0.0f32]);
        assert!(received.borrow().is_empty(), "equal distance does not call");
        e.set(c, BSPlayerDistanceCheckController::fCheckDistance, 4.0f32);
        e.call(0x008d1670, &args![c, 0.0f32]);
        assert!(received.borrow().is_empty());
    }

    #[test]
    fn update_negative_distance_leaves_before_anything_else() {
        let mut e = Engine::new();
        let (c, _, received) = update_rig(&mut e);
        e.set(c, BSPlayerDistanceCheckController::fCheckDistance, -1.0f32);
        e.call_log = Some(vec![]);
        e.call(0x008d1670, &args![c, 0.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2, "the call itself and the distance getter");
        assert!(received.borrow().is_empty());
    }

    #[test]
    fn update_nan_distance_passes_the_first_test_and_fails_the_last() {
        let mut e = Engine::new();
        let (c, _, received) = update_rig(&mut e);
        e.set(c, BSPlayerDistanceCheckController::fCheckDistance, f32::NAN);
        e.call_log = Some(vec![]);
        e.call(0x008d1670, &args![c, 0.0f32]);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter().any(|(a, _)| *a == VECTOR_LENGTH),
            "a NaN check distance is not negative, so the distance is measured"
        );
        assert!(received.borrow().is_empty(), "NaN is not greater");
    }

    #[test]
    fn update_needs_callback_reference_and_cast_target() {
        let mut e = Engine::new();
        let (c, _, received) = update_rig(&mut e);
        e.set(c, BSPlayerDistanceCheckController::pfnCallback, 0u32);
        e.call(0x008d1670, &args![c, 0.0f32]);
        e.set(
            c,
            BSPlayerDistanceCheckController::pfnCallback,
            0x7000_cb00u32,
        );
        let reference: u32 = e.global(DISTANCE_REFERENCE);
        e.set_global(DISTANCE_REFERENCE, 0u32);
        e.call(0x008d1670, &args![c, 0.0f32]);
        e.set_global(DISTANCE_REFERENCE, reference);
        e.set(
            c,
            BSPlayerDistanceCheckController::m_pkTarget,
            Ptr::<()>::new(0),
        );
        e.call(0x008d1670, &args![c, 0.0f32]);
        assert!(received.borrow().is_empty());
    }

    #[test]
    fn target_is_required_type_passes_the_target_through() {
        let mut e = Engine::new();
        e.register(IS_TARGET_TYPE, |_, a| {
            ret((a[0] == 0x011f_4280 && a[1] == 0x1234) as u32)
        });
        let c = controller(&mut e);
        e.set(
            c,
            BSPlayerDistanceCheckController::m_pkTarget,
            Ptr::<()>::new(0x1234),
        );
        assert!(e.call(0x008d1720, &args![c]).bool());
        e.set(
            c,
            BSPlayerDistanceCheckController::m_pkTarget,
            Ptr::<()>::new(0),
        );
        assert!(!e.call(0x008d1720, &args![c]).bool());
    }

    #[test]
    fn copy_step_moves_callback_and_distance_then_runs_the_base_copy() {
        let mut e = Engine::new();
        e.register(GET_CALLBACK, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        e.register(SET_CALLBACK, |e, a| {
            e.mem.set_u32(a[0] + 0x38, a[1]);
            Ret::default()
        });
        e.register(GET_CHECK_DISTANCE, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x34) as f64,
            ..Ret::default()
        });
        e.register(SET_CHECK_DISTANCE, |e, a| {
            e.mem.set_f32(a[0] + 0x34, f32::from_bits(a[1]));
            Ret::default()
        });
        e.register(TIME_CONTROLLER_COPY, |_, _| Ret::default());
        let source = controller(&mut e);
        let clone = controller(&mut e);
        e.set(
            source,
            BSPlayerDistanceCheckController::fCheckDistance,
            7.5f32,
        );
        e.set(
            source,
            BSPlayerDistanceCheckController::pfnCallback,
            0x1111u32,
        );
        e.call_log = Some(vec![]);
        e.call(0x008d17d0, &args![source, clone, 0x2222u32]);
        assert_eq!(
            e.get(clone, BSPlayerDistanceCheckController::fCheckDistance),
            7.5
        );
        assert_eq!(
            e.get(clone, BSPlayerDistanceCheckController::pfnCallback),
            0x1111
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(
                TIME_CONTROLLER_COPY,
                vec![source.addr(), clone.addr(), 0x2222]
            )
        );
    }

    #[test]
    fn create_clone_constructs_then_copies() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x3c);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.register(0x008d_15d0, |_, a| ret(a[0]));
        e.register(GET_CALLBACK, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        e.register(SET_CALLBACK, |e, a| {
            e.mem.set_u32(a[0] + 0x38, a[1]);
            Ret::default()
        });
        e.register(GET_CHECK_DISTANCE, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x34) as f64,
            ..Ret::default()
        });
        e.register(SET_CHECK_DISTANCE, |e, a| {
            e.mem.set_f32(a[0] + 0x34, f32::from_bits(a[1]));
            Ret::default()
        });
        e.register(TIME_CONTROLLER_COPY, |_, _| Ret::default());
        let source = controller(&mut e);
        e.set(
            source,
            BSPlayerDistanceCheckController::fCheckDistance,
            3.0f32,
        );
        e.set(
            source,
            BSPlayerDistanceCheckController::pfnCallback,
            0x4444u32,
        );
        let clone = e
            .call(0x008d1740, &args![source, 9u32])
            .ptr::<BSPlayerDistanceCheckController>();
        assert_eq!(clone.addr(), block);
        assert_eq!(
            e.get(clone, BSPlayerDistanceCheckController::fCheckDistance),
            3.0
        );
        assert_eq!(
            e.get(clone, BSPlayerDistanceCheckController::pfnCallback),
            0x4444
        );
    }
}

//! `fallout shared/run_0049fc80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `BGSDecalManager` (layout in `bgsdecalmanager.rs`) constructor,
//! destructor, instance getter and teardown, its per-frame update, and an
//! array helper the compiler emitted next to them. The exception-unwinding
//! frames (`__CxxFrameHandler` states) of the constructor, destructor and
//! getter are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::bgsdecalmanager::BGSDecalManager;

/// The manager's instance pointer (`011c57f8`; `BGSDecalManager::pInstance`
/// in the Xbox PDB, matched by use: the getter allocates a 0x28-byte
/// `BGSDecalManager` into it).
const INSTANCE: u32 = 0x011c_57f8;
/// Two counters `004a0010` clears (`011c57e4`, `011c57e8`;
/// `bgsdecalmanager.rs` matches them by use with `iDecalsThisFrame` and
/// `iSkinnedDecalsThisFrame`).
const COUNTER_011C57E4: u32 = 0x011c_57e4;
const COUNTER_011C57E8: u32 = 0x011c_57e8;
/// A global `NiPointer` the destructor sets to null (`011c58e0`).
const NI_POINTER_011C58E0: u32 = 0x011c_58e0;

/// `NiPointer` constructor from a raw pointer (`00633c90`, thiscall, one
/// word).
const NI_POINTER_INIT: u32 = 0x0063_3c90;
/// `NiPointer` assignment from a raw pointer (`0066b0d0`, thiscall, one
/// word).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` destructor (`0045cec0`, thiscall).
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// The list constructor used for both lists (`0048f200`, thiscall).
const LIST_INIT: u32 = 0x0048_f200;
/// Allocation (`00aa13e0`, cdecl, size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `BSShaderAccumulator::BSShaderAccumulator` (Xbox PDB; `00b660d0`,
/// thiscall, three words).
const SHADER_ACCUMULATOR_INIT: u32 = 0x00b6_60d0;
/// The `NiCamera` unit's constructor at `00a712f0` (thiscall).
const CAMERA_INIT: u32 = 0x00a7_12f0;
/// Allocation the instance getter uses (`00401000`, cdecl, size).
const ALLOCATE_INSTANCE: u32 = 0x0040_1000;
/// Deallocation (`00401030`, cdecl, pointer).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 0049fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the first virtual (slot 0) with argument 0 on each of `count`
/// objects laid out 0x10 bytes apart from `array` (an array's element
/// destructor loop). The method's own `this` is not used.
pub fn fn_0049fc80(e: &mut Engine, _this: Ptr, array: Ptr, count: u32) {
    let mut index = 0u32;
    while index < count {
        let element = array.addr().wrapping_add(index.wrapping_mul(0x10));
        e.vcall(element, 0, &args![0u32]);
        index += 1;
    }
}

// Translated from 0049fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0049fcd0 - the exception-unwinding frame (FS:[0], states 0-6) is not translated
/// The `BGSDecalManager` constructor: null `spQueryTexture`, both lists
/// constructed, null `spQueryAccum` and `spQueryCamera`; then a new
/// `BSShaderAccumulator` (0x280 bytes, constructed with 0x63, 1, 0x2f7; null
/// if the allocation fails) into `spQueryAccum` and a new camera (0x114
/// bytes, `00a712f0`) into `spQueryCamera`. Returns `this`.
pub fn fn_0049fcd0(e: &mut Engine, this: Ptr<BGSDecalManager>) -> Ptr<BGSDecalManager> {
    let base = this.addr();
    e.call(NI_POINTER_INIT, &args![base, 0u32]);
    e.call(LIST_INIT, &args![base + 0x08]);
    e.call(LIST_INIT, &args![base + 0x14]);
    e.call(NI_POINTER_INIT, &args![base + 0x20, 0u32]);
    e.call(NI_POINTER_INIT, &args![base + 0x24, 0u32]);

    let block = e.call(ALLOCATE, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            SHADER_ACCUMULATOR_INIT,
            &args![block, 0x63u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![base + 0x20, accumulator]);

    let block = e.call(ALLOCATE, &args![0x114u32]).u32();
    let camera = if block == 0 {
        0
    } else {
        e.call(CAMERA_INIT, &args![block]).u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![base + 0x24, camera]);
    this
}

// Translated from 0049fe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004a44a0` on `this` (the pending list's destructor in the manager's
/// destructor).
pub fn fn_0049fe00(e: &mut Engine, this: Ptr) {
    e.call(0x004a_44a0, &args![this]);
}

// Translated from 0049fe20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0049fe20 - the exception-unwinding frame (FS:[0], states 0-4) is not translated
/// The `BGSDecalManager` destructor: `00622bc0`, the global `NiPointer` at
/// `011c58e0` set to null, `004a4650` on the pending list, `004ed900` on the
/// emitter list, `spQueryAccum` and `spQueryCamera` set to null; then the
/// members destroyed in reverse order (`spQueryCamera`, `spQueryAccum`,
/// the emitter list by `004a1a30`, the pending list by `0049fe00`,
/// `spQueryTexture`).
pub fn fn_0049fe20(e: &mut Engine, this: Ptr<BGSDecalManager>) {
    let base = this.addr();
    e.call(0x0062_2bc0, &args![]);
    e.call(NI_POINTER_ASSIGN, &args![NI_POINTER_011C58E0, 0u32]);
    e.call(0x004a_4650, &args![base + 0x08]);
    e.call(0x004e_d900, &args![base + 0x14]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x20, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x24, 0u32]);
    e.call(NI_POINTER_RELEASE, &args![base + 0x24]);
    e.call(NI_POINTER_RELEASE, &args![base + 0x20]);
    e.call(0x004a_1a30, &args![base + 0x14]);
    fn_0049fe00(e, Ptr::new(base + 0x08));
    e.call(NI_POINTER_RELEASE, &args![base]);
}

// Translated from 0049fef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0049fef0 - the exception-unwinding frame (FS:[0], state 0) is not translated
/// The manager's instance: created on first use (0x28 bytes from
/// `00401000`, constructed by `0049fcd0`; null if the allocation fails).
pub fn fn_0049fef0(e: &mut Engine) -> Ptr<BGSDecalManager> {
    if e.global::<u32>(INSTANCE) == 0 {
        let block = e.call(ALLOCATE_INSTANCE, &args![0x28u32]).u32();
        let instance = if block == 0 {
            0
        } else {
            fn_0049fcd0(e, Ptr::new(block)).addr()
        };
        e.set_global(INSTANCE, instance);
    }
    Ptr::new(e.global(INSTANCE))
}

// Translated from 0049ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and frees the manager's instance if there is one
/// (`0049ffc0` with flag 1) and clears the instance pointer.
pub fn fn_0049ff80(e: &mut Engine) {
    let instance: u32 = e.global(INSTANCE);
    if instance != 0 {
        fn_0049ffc0(e, Ptr::new(instance), 1);
    }
    e.set_global(INSTANCE, 0u32);
}

// Translated from 0049ffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's deleting destructor: the destructor (`0049fe20`), then
/// `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0049ffc0(e: &mut Engine, this: Ptr<BGSDecalManager>, flags: u32) -> Ptr<BGSDecalManager> {
    fn_0049fe20(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0049fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's per-frame update: clears the two counters (`004a0010`),
/// then `BGSDecalManager::UpdateSimpleDecals` (`004a0030`) and `004a03f0`
/// on `this`.
pub fn fn_0049fff0(e: &mut Engine, this: Ptr<BGSDecalManager>) {
    fn_004a0010(e);
    e.call(0x004a_0030, &args![this]);
    e.call(0x004a_03f0, &args![this]);
}

// Translated from 004a0010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the counters at `011c57e4` and `011c57e8` to 0.
pub fn fn_004a0010(e: &mut Engine) {
    e.set_global(COUNTER_011C57E4, 0u32);
    e.set_global(COUNTER_011C57E8, 0u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049fc80, fn_0049fc80(Ptr, Ptr, u32)),
        entry!(
            0x0049fcd0,
            fn_0049fcd0(Ptr<BGSDecalManager>) -> Ptr<BGSDecalManager>
        ),
        entry!(0x0049fe00, fn_0049fe00(Ptr)),
        entry!(0x0049fe20, fn_0049fe20(Ptr<BGSDecalManager>)),
        entry!(0x0049fef0, fn_0049fef0() -> Ptr<BGSDecalManager>),
        entry!(0x0049ff80, fn_0049ff80()),
        entry!(
            0x0049ffc0,
            fn_0049ffc0(Ptr<BGSDecalManager>, u32) -> Ptr<BGSDecalManager>
        ),
        entry!(0x0049fff0, fn_0049fff0(Ptr<BGSDecalManager>)),
        entry!(0x004a0010, fn_004a0010()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn quiet_double(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    fn calls(e: &mut Engine) -> CallLog {
        e.call_log.take().unwrap()
    }

    #[test]
    fn element_loop_calls_slot_zero_with_zero() {
        let mut e = Engine::new();
        e.put_vtable(0x0102_0000, &[0x00ab_0000]);
        e.register(0x00ab_0000, |e, a| {
            assert_eq!(a[1], 0);
            let n = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, n + 1);
            Ret::default()
        });
        let array = e.mem.alloc(0x40);
        for i in 0..4 {
            e.mem.set_u32(array + i * 0x10, 0x0102_0000);
        }
        fn_0049fc80(&mut e, Ptr::new(0), Ptr::new(array), 3);
        let hits: Vec<u32> = (0..4).map(|i| e.mem.u32(array + i * 0x10 + 4)).collect();
        assert_eq!(hits, [1, 1, 1, 0]);
        fn_0049fc80(&mut e, Ptr::new(0), Ptr::new(array), 0);
        assert_eq!(e.mem.u32(array + 4), 1);
    }

    #[test]
    fn constructor_builds_members_and_query_objects() {
        let mut e = Engine::new();
        for addr in [NI_POINTER_INIT, NI_POINTER_ASSIGN, LIST_INIT] {
            quiet_double(&mut e, addr);
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(SHADER_ACCUMULATOR_INIT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CAMERA_INIT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.new_object::<BGSDecalManager>();
        let b = this.addr();
        e.call_log = Some(vec![]);
        assert_eq!(fn_0049fcd0(&mut e, this), this);
        let log = calls(&mut e);
        let accumulator = log
            .iter()
            .find(|c| c.0 == SHADER_ACCUMULATOR_INIT)
            .unwrap()
            .1
            .clone();
        assert_eq!(accumulator[1..], [0x63, 1, 0x2f7]);
        let camera = log.iter().find(|c| c.0 == CAMERA_INIT).unwrap().1[0];
        let assigns: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_POINTER_ASSIGN)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            assigns,
            [vec![b + 0x20, accumulator[0]], vec![b + 0x24, camera]]
        );
        let inits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_POINTER_INIT || c.0 == LIST_INIT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(inits, [b, b + 8, b + 0x14, b + 0x20, b + 0x24]);

        // Failed allocations: null pointers, no constructors.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_0049fcd0(&mut e, this);
        let log = calls(&mut e);
        assert!(!log
            .iter()
            .any(|c| c.0 == SHADER_ACCUMULATOR_INIT || c.0 == CAMERA_INIT));
        let assigns: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_POINTER_ASSIGN)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(assigns, [vec![b + 0x20, 0], vec![b + 0x24, 0]]);
    }

    fn destructor_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            0x0062_2bc0,
            NI_POINTER_ASSIGN,
            0x004a_4650,
            0x004e_d900,
            NI_POINTER_RELEASE,
            0x004a_1a30,
            0x004a_44a0,
            DEALLOCATE,
        ] {
            quiet_double(&mut e, addr);
        }
        e
    }

    #[test]
    fn destructor_order() {
        let mut e = destructor_engine();
        let b = 0x0200_0000;
        e.call_log = Some(vec![]);
        fn_0049fe20(&mut e, Ptr::new(b));
        let log = calls(&mut e);
        let expected: CallLog = vec![
            (0x0062_2bc0, vec![]),
            (NI_POINTER_ASSIGN, vec![NI_POINTER_011C58E0, 0]),
            (0x004a_4650, vec![b + 8]),
            (0x004e_d900, vec![b + 0x14]),
            (NI_POINTER_ASSIGN, vec![b + 0x20, 0]),
            (NI_POINTER_ASSIGN, vec![b + 0x24, 0]),
            (NI_POINTER_RELEASE, vec![b + 0x24]),
            (NI_POINTER_RELEASE, vec![b + 0x20]),
            (0x004a_1a30, vec![b + 0x14]),
            (0x004a_44a0, vec![b + 8]),
            (NI_POINTER_RELEASE, vec![b]),
        ];
        assert_eq!(log, expected);
    }

    #[test]
    fn pending_list_destructor() {
        let mut e = destructor_engine();
        e.call_log = Some(vec![]);
        fn_0049fe00(&mut e, Ptr::new(0x1234));
        assert_eq!(calls(&mut e), vec![(0x004a_44a0, vec![0x1234])]);
    }

    #[test]
    fn deleting_destructor_frees_on_bit_zero() {
        let mut e = destructor_engine();
        e.call_log = Some(vec![]);
        assert_eq!(fn_0049ffc0(&mut e, Ptr::new(0x500), 2).addr(), 0x500);
        assert!(!calls(&mut e).iter().any(|c| c.0 == DEALLOCATE));
        e.call_log = Some(vec![]);
        fn_0049ffc0(&mut e, Ptr::new(0x500), 1);
        let log = calls(&mut e);
        assert_eq!(log.last().unwrap(), &(DEALLOCATE, vec![0x500]));
    }

    #[test]
    fn instance_is_created_once() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        for addr in [NI_POINTER_INIT, NI_POINTER_ASSIGN, LIST_INIT] {
            quiet_double(&mut e, addr);
        }
        e.register(ALLOCATE, |_, _| Ret::default());
        e.register(ALLOCATE_INSTANCE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let first = fn_0049fef0(&mut e);
        assert!(!first.is_null());
        assert_eq!(e.global::<u32>(INSTANCE), first.addr());
        e.call_log = Some(vec![]);
        assert_eq!(fn_0049fef0(&mut e), first);
        assert!(calls(&mut e).is_empty());

        // Allocation failure leaves it null.
        e.set_global(INSTANCE, 0u32);
        e.register(ALLOCATE_INSTANCE, |_, _| Ret::default());
        assert!(fn_0049fef0(&mut e).is_null());
    }

    #[test]
    fn teardown_destroys_and_clears() {
        let mut e = destructor_engine();
        e.map(0x011c_5000, 0x1000);
        e.call_log = Some(vec![]);
        fn_0049ff80(&mut e);
        assert!(calls(&mut e).is_empty());

        e.set_global(INSTANCE, 0x0300_0000u32);
        e.call_log = Some(vec![]);
        fn_0049ff80(&mut e);
        let log = calls(&mut e);
        assert_eq!(log.first().unwrap().0, 0x0062_2bc0);
        assert_eq!(log.last().unwrap(), &(DEALLOCATE, vec![0x0300_0000]));
        assert_eq!(e.global::<u32>(INSTANCE), 0);
    }

    #[test]
    fn update_clears_counters_then_updates() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.set_global(COUNTER_011C57E4, 5u32);
        e.set_global(COUNTER_011C57E8, 6u32);
        quiet_double(&mut e, 0x004a_0030);
        quiet_double(&mut e, 0x004a_03f0);
        e.call_log = Some(vec![]);
        fn_0049fff0(&mut e, Ptr::new(0x700));
        assert_eq!(
            calls(&mut e),
            vec![(0x004a_0030, vec![0x700]), (0x004a_03f0, vec![0x700])]
        );
        assert_eq!(e.global::<u32>(COUNTER_011C57E4), 0);
        assert_eq!(e.global::<u32>(COUNTER_011C57E8), 0);
    }
}

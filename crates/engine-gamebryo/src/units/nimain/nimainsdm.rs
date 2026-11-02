//! `NiMain/nimainsdm` (no Xbox PDB source unit; Ghidra never created functions here, they are
//! reached only through the start-up hook): the initializer of the Gamebryo main module that
//! `fn_00aa20b0` runs first (`00a89cc0`, 1343 bytes) and the constructor that installs it
//! (`00a8a5f0`, 42 bytes). The task that assigned them calls them `NiMainSDM::Init` and its
//! constructor; no symbol in the exe confirms the class name.

#[allow(unused_imports)]
use crate::prelude::*;

/// Byte `011f5b80`: set to 1 the first time the initializer runs.
const INITIALIZED: u32 = 0x011f_5b80;
/// Word `011f5b84`: how many times the constructor ran.
const CONSTRUCTED_COUNT: u32 = 0x011f_5b84;
/// `fn_00aa2060` (`cdecl`, two words): stores the start-up hook and its second word.
const SET_START_HOOK: u32 = 0x00aa_2060;
/// The initializer, `00a89cc0`: the hook the constructor stores.
const INITIALIZER: u32 = 0x00a8_9cc0;
/// The function stored as the hook's second word (`00a8a280`; not read here).
const HOOK_SECOND_WORD: u32 = 0x00a8_a280;
/// Three words at `011f426c` (zero in the exe image) copied by the initializer.
const SHARED_POINT: u32 = 0x011f_426c;
/// Where the initializer copies them: four words at `011f4288`, the fourth set to 0.0.
const COPIED_POINT: u32 = 0x011f_4288;
/// `00a64900` (`cdecl`, two words; Ghidra name `NiStream::RegisterLoader`): registers a
/// class name (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// The last call of the initializer, which the exe reaches with a `JMP`.
const LAST_STEP: u32 = 0x00a7_4da0;

/// The calls (no arguments) the initializer makes first, in order.
const FIRST_STEPS: [u32; 13] = [
    0x00a5_b630,
    0x00a8_13c0,
    0x00a9_c2a0,
    0x00a9_27e0,
    0x00a5_cef0,
    0x00a9_a090,
    0x00a7_59e0,
    0x00a2_9680,
    0x00a8_1520,
    0x00a9_ba80,
    0x00a6_f450,
    0x00a6_b410,
    0x00a8_f5b0,
];
/// The calls (no arguments) after the copy of the point, before the registrations.
const MIDDLE_STEPS: [u32; 2] = [0x00a7_6ab0, 0x00a6_60a0];
/// The calls (no arguments) after the registrations, ending with the one made by `JMP`.
const LAST_STEPS: [u32; 7] = [
    0x00a9_1290,
    0x00a6_2110,
    0x00a7_e930,
    0x00a2_9680,
    0x00a8_fd80,
    0x00a8_fa50,
    LAST_STEP,
];

/// The loader registrations, in the order the initializer makes them:
/// (class name string, creation function), the arguments of `00a64900` in that order.
const LOADERS: [(u32, u32); 76] = [
    (0x0109_df08, 0x00a7_f740),
    (0x0109_d378, 0x00a7_3e20),
    (0x0109_f220, 0x00a9_b4b0),
    (0x0109_b4e0, 0x00a5_ceb0),
    (0x0109_f210, 0x00a9_b330),
    (0x0109_f1fc, 0x00a8_f580),
    (0x0109_f1e4, 0x00a9_b150),
    (0x0109_f1c8, 0x00a9_6600),
    (0x0109_dec4, 0x00a7_e770),
    (0x0109_df58, 0x00a7_f8e0),
    (0x0109_f1b4, 0x00a9_af90),
    (0x0109_f1a8, 0x00a9_aea0),
    (0x0109_cd04, 0x00a7_1430),
    (0x0109_f190, 0x00a9_b4b0),
    (0x0109_b0f8, 0x00a5_f030),
    (0x0109_ed6c, 0x00a8_9490),
    (0x0109_ca34, 0x00a6_f000),
    (0x0109_d890, 0x00a7_5d50),
    (0x0109_f17c, 0x00a9_a640),
    (0x0109_dab0, 0x00a7_b380),
    (0x0109_f168, 0x00a9_a480),
    (0x0109_ecb0, 0x00a8_9190),
    (0x0109_f158, 0x00a9_9fd0),
    (0x0109_f144, 0x00a9_9cd0),
    (0x0109_f130, 0x00a9_9a10),
    (0x0109_f124, 0x00a9_9710),
    (0x0109_d64c, 0x00a7_4750),
    (0x0109_f118, 0x00a8_a700),
    (0x0109_d79c, 0x00a7_56d0),
    (0x0109_b6b8, 0x00a5_f030),
    (0x0109_f10c, 0x00a8_b220),
    (0x0109_f0f8, 0x00a9_8e70),
    (0x0109_f0e0, 0x00a9_8d30),
    (0x0109_f0d4, 0x00a9_b150),
    (0x0109_f0c4, 0x00a9_6600),
    (0x0109_f0a0, 0x00a9_82d0),
    (0x0109_dc94, 0x00a7_c3f0),
    (0x0109_de0c, 0x00a7_d6e0),
    (0x0109_f090, 0x00a9_7c00),
    (0x0109_f07c, 0x00a9_b150),
    (0x0109_f064, 0x00a9_6660),
    (0x0109_f054, 0x00a9_5f20),
    (0x0109_ea78, 0x00a8_77a0),
    (0x0109_ebd4, 0x00a8_7920),
    (0x0109_f044, 0x00a9_58b0),
    (0x0109_e314, 0x00a8_14e0),
    (0x0109_e6a0, 0x00a8_4d30),
    (0x0109_e6ec, 0x00a8_66f0),
    (0x0109_c808, 0x00a6_c8a0),
    (0x0109_e560, 0x00a8_1fb0),
    (0x0109_f030, 0x00a9_52a0),
    (0x0109_baa4, 0x00a6_0070),
    (0x0109_bb6c, 0x00a6_0f30),
    (0x0109_f01c, 0x00a9_5160),
    (0x0109_f010, 0x00a9_4e20),
    (0x0109_cb84, 0x00a6_f410),
    (0x0109_d43c, 0x00a7_4230),
    (0x0109_effc, 0x00a9_4930),
    (0x0109_efec, 0x00a9_4550),
    (0x0109_efd4, 0x00a9_3870),
    (0x0109_efc4, 0x00a9_29b0),
    (0x0109_c7a0, 0x00a6_ad40),
    (0x0109_d548, 0x00a7_44d0),
    (0x0109_e96c, 0x00a8_7140),
    (0x0109_db9c, 0x00a7_b790),
    (0x0109_e408, 0x00a8_1760),
    (0x0109_ce38, 0x00a7_1ce0),
    (0x0109_d990, 0x00a7_5ec0),
    (0x0109_efac, 0x00a9_23f0),
    (0x0109_ef98, 0x00a9_1fe0),
    (0x0109_b7dc, 0x00a5_f280),
    (0x0109_ef30, 0x00a8_9b80),
    (0x0109_ef84, 0x00a9_1ee0),
    (0x0109_ef70, 0x00a8_ff80),
    (0x0109_ef58, 0x00a9_0b20),
    (0x0109_ef48, 0x00a8_ffc0),
];

// Translated from 00a89cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map and without a Ghidra function (reached through the hook
/// `fn_00aa20b0` calls; called `NiMainSDM::Init` by the task, not confirmed by a symbol;
/// `cdecl`, no arguments, 1343 bytes). Runs once (byte `011f5b80`): 13 initializer calls,
/// the copy of the three words at `011f426c` to `011f4288` with a 0.0 after them,
/// two more calls, 76 loader registrations through `00a64900` (a class name string and
/// its creation function each, e.g. `NiAdditionalGeometryData`), and 7 calls, the last
/// of them a tail jump whose result the exe leaves in `EAX`.
pub fn fn_00a89cc0(e: &mut Engine) {
    if e.mem.u8(INITIALIZED) != 0 {
        return;
    }
    e.mem.set_u8(INITIALIZED, 1);
    for address in FIRST_STEPS {
        e.call(address, &args![]);
    }
    // `FLDZ`/`FSTP` of 0.0 to the fourth word; the three before it are plain moves.
    let first = e.mem.u32(SHARED_POINT);
    let second = e.mem.u32(SHARED_POINT + 4);
    let third = e.mem.u32(SHARED_POINT + 8);
    e.mem.set_u32(COPIED_POINT + 0xc, 0.0f32.to_bits());
    e.mem.set_u32(COPIED_POINT, first);
    e.mem.set_u32(COPIED_POINT + 4, second);
    e.mem.set_u32(COPIED_POINT + 8, third);
    for address in MIDDLE_STEPS {
        e.call(address, &args![]);
    }
    for (name, creator) in LOADERS {
        e.call(REGISTER_LOADER, &args![name, creator]);
    }
    for address in LAST_STEPS {
        e.call(address, &args![]);
    }
}

// Translated from 00a8a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map and without a Ghidra function (the constructor of
/// `NiMainSDM`, 42 bytes; `thiscall`, no stack word; the class name is the task's and is
/// not confirmed by a symbol): counts its constructions in `011f5b84`, and on the first
/// one stores the initializer `00a89cc0` and `00a8a280` as the start-up hook with
/// `fn_00aa2060`. Returns `this`.
pub fn fn_00a8a5f0(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.mem.u32(CONSTRUCTED_COUNT);
    e.mem.set_u32(CONSTRUCTED_COUNT, count.wrapping_add(1));
    if count == 0 {
        e.call(SET_START_HOOK, &args![INITIALIZER, HOOK_SECOND_WORD]);
    }
    this
}

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
pub(crate) const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a5c200` (`thiscall`, unit `niobjectnet.cpp`): the constructor of the object base that
/// the default property objects start with.
pub(crate) const OBJECT_BASE_CONSTRUCTOR: u32 = 0x00a5_c200;
/// `00a5b690` (`cdecl`, one word): the fixed-string handle for a C string.
pub(crate) const GET_FIXED_STRING: u32 = 0x00a5_b690;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `0109b220`: the fixed-string handle of the empty string, which is never counted.
pub(crate) const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

/// `InterlockedIncrement` on the counted word at `address`.
pub(crate) fn interlocked_increment(e: &mut Engine, address: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![address]);
}

/// `InterlockedDecrement` on the counted word at `address`; the new count.
pub(crate) fn interlocked_decrement(e: &mut Engine, address: u32) -> u32 {
    e.call(INTERLOCKED_DECREMENT, &args![address]).u32()
}

/// The compiler's smart-pointer assignment of a reference-counted object (count at `+4`,
/// virtual slot `+4` frees it) to the pointer at `slot`: nothing when `new` is already
/// there; otherwise the old object is released (freed through its vtable slot `+4` when its
/// count reaches 0), `new` is stored and `new` is counted.
pub(crate) fn assign_ref_counted(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 && interlocked_decrement(e, old.wrapping_add(4)) == 0 {
        e.vcall(old, 4, &args![]);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

/// Releases a fixed-string handle (count at `-8`) unless it is the empty string's.
pub(crate) fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        interlocked_decrement(e, handle.wrapping_sub(8));
    }
}

/// Counts a fixed-string handle (count at `-8`) unless it is the empty string's.
pub(crate) fn acquire_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        interlocked_increment(e, handle.wrapping_sub(8));
    }
}

/// The compiler's initialization of a static fixed string from a literal: when the word at
/// `slot` is not already the literal's address, stores the handle for the literal there and
/// releases the old one.
pub(crate) fn set_literal_fixed_string(e: &mut Engine, slot: u32, literal: u32) {
    let old = e.mem.u32(slot);
    if old != literal {
        let handle = e.call(GET_FIXED_STRING, &args![literal]).u32();
        e.mem.set_u32(slot, handle);
        release_fixed_string(e, old);
    }
}

/// Test doubles shared by the start-up units' tests (default property objects, static
/// strings).
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub(crate) use super::{
        ALLOCATE, EMPTY_FIXED_STRING, GET_FIXED_STRING, OBJECT_BASE_CONSTRUCTOR,
    };

    /// The vtable address given to every previous object, and its slot `+4`.
    pub(crate) const DELETE_SLOT_DOUBLE: u32 = 0x00ee_0004;

    pub(crate) fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// An engine with the start-up globals mapped (`011f4000` to `011f7000`, the empty
    /// string handle's page) and doubles for the allocation (`00aa13e0` hands out zeroed
    /// test heap) and the interlocked imports (they count in place and answer the new
    /// count).
    pub(crate) fn engine_with_counting() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x3000);
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0109_b228);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(OBJECT_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e
    }

    /// What `run_default_instance` found.
    pub(crate) struct Instance {
        pub(crate) engine: Engine,
        /// The object now held by the global (0 when none).
        pub(crate) object: u32,
        /// The object that was held before, a counted object with count 1.
        pub(crate) previous: u32,
        /// The objects the previous object's destructor slot was run on.
        pub(crate) freed: Rc<RefCell<Vec<u32>>>,
    }

    /// Runs the function at `address` (no arguments) in an engine where `global` holds a
    /// previous counted object (count 1, vtable slot `+4` a recording double), after
    /// `setup` has registered the doubles the function needs.
    pub(crate) fn run_default_instance(
        address: u32,
        global: u32,
        setup: impl FnOnce(&mut Engine),
    ) -> Instance {
        let mut e = engine_with_counting();
        setup(&mut e);
        let freed: Rc<RefCell<Vec<u32>>> = Rc::default();
        let log = freed.clone();
        e.register_double(DELETE_SLOT_DOUBLE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.put_vtable(0x0300_0000, &[0, DELETE_SLOT_DOUBLE]);
        let previous = e.mem.alloc(0x10);
        e.mem.set_u32(previous, 0x0300_0000);
        e.mem.set_u32(previous + 4, 1);
        e.mem.set_u32(global, previous);
        e.call(address, &args![]);
        let object = e.mem.u32(global);
        Instance {
            engine: e,
            object,
            previous,
            freed,
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_9cc0, fn_00a89cc0()),
        entry!(0x00a8_a5f0, fn_00a8a5f0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// Every callee of the initializer doubled with a no-op.
    fn engine_with_doubles() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        let mut callees = vec![REGISTER_LOADER];
        callees.extend(FIRST_STEPS);
        callees.extend(MIDDLE_STEPS);
        callees.extend(LAST_STEPS);
        for address in callees {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn initializer_runs_its_steps_in_order_and_registers_the_loaders() {
        let mut e = engine_with_doubles();
        e.mem.set_u32(SHARED_POINT, 0x1111);
        e.mem.set_u32(SHARED_POINT + 4, 0x2222);
        e.mem.set_u32(SHARED_POINT + 8, 0x3333);
        e.mem.set_u32(COPIED_POINT + 0xc, 0xdead_beef);
        e.call_log = Some(vec![]);
        e.call(0x00a8_9cc0, &args![]);
        let order: Vec<u32> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .collect();
        let mut expected = vec![0x00a8_9cc0];
        expected.extend(FIRST_STEPS);
        expected.extend(MIDDLE_STEPS);
        expected.extend([REGISTER_LOADER; 76]);
        expected.extend(LAST_STEPS);
        assert_eq!(order, expected);
        let loaders = calls_to(&e, REGISTER_LOADER);
        assert_eq!(loaders[0], vec![0x0109_df08, 0x00a7_f740]);
        assert_eq!(loaders[75], vec![0x0109_ef48, 0x00a8_ffc0]);
        assert_eq!(e.mem.u8(INITIALIZED), 1);
        assert_eq!(e.mem.u32(COPIED_POINT), 0x1111);
        assert_eq!(e.mem.u32(COPIED_POINT + 4), 0x2222);
        assert_eq!(e.mem.u32(COPIED_POINT + 8), 0x3333);
        assert_eq!(e.mem.u32(COPIED_POINT + 0xc), 0);
    }

    #[test]
    fn initializer_does_nothing_the_second_time() {
        let mut e = engine_with_doubles();
        e.call(0x00a8_9cc0, &args![]);
        e.call_log = Some(vec![]);
        e.call(0x00a8_9cc0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn constructor_installs_the_hook_on_the_first_construction_only() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.register(SET_START_HOOK, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_a5f0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, SET_START_HOOK),
            vec![vec![0x00a8_9cc0, 0x00a8_a280]]
        );
        assert_eq!(e.mem.u32(CONSTRUCTED_COUNT), 1);
        assert_eq!(e.call(0x00a8_a5f0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, SET_START_HOOK).len(), 1);
        assert_eq!(e.mem.u32(CONSTRUCTED_COUNT), 2);
    }
}

#[cfg(test)]
mod helper_tests {
    use super::test_support::*;
    use super::*;

    #[test]
    fn assigning_the_held_object_does_nothing() {
        let mut e = engine_with_counting();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(0x011f_4a00, object);
        e.mem.set_u32(object + 4, 5);
        assign_ref_counted(&mut e, 0x011f_4a00, object);
        assert_eq!(e.mem.u32(object + 4), 5);
        assert_eq!(e.mem.u32(0x011f_4a00), object);
    }

    #[test]
    fn assigning_releases_the_old_object_and_counts_the_new_one() {
        let mut e = engine_with_counting();
        e.put_vtable(0x0300_0000, &[0, DELETE_SLOT_DOUBLE]);
        e.register(DELETE_SLOT_DOUBLE, |_, _| Ret::default());
        let old = e.mem.alloc(0x10);
        e.mem.set_u32(old, 0x0300_0000);
        e.mem.set_u32(old + 4, 2);
        let new = e.mem.alloc(0x10);
        e.mem.set_u32(0x011f_4a00, old);
        e.call_log = Some(vec![]);
        assign_ref_counted(&mut e, 0x011f_4a00, new);
        // Still held elsewhere: not freed.
        assert_eq!(e.mem.u32(old + 4), 1);
        assert!(calls_to(&e, DELETE_SLOT_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(0x011f_4a00), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        // The last holder gone: freed through the vtable slot.
        e.mem.set_u32(new, 0x0300_0000);
        assign_ref_counted(&mut e, 0x011f_4a00, 0);
        assert_eq!(e.mem.u32(new + 4), 0);
        assert_eq!(calls_to(&e, DELETE_SLOT_DOUBLE), vec![vec![new]]);
        assert_eq!(e.mem.u32(0x011f_4a00), 0);
    }

    #[test]
    fn literal_fixed_string_is_set_once_and_the_old_handle_released() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let old = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(old - 8, 3);
        e.mem.set_u32(0x011f_4a10, old);
        e.call_log = Some(vec![]);
        set_literal_fixed_string(&mut e, 0x011f_4a10, 0x0108_0000);
        assert_eq!(e.mem.u32(0x011f_4a10), 0x0108_1000);
        assert_eq!(e.mem.u32(old - 8), 2);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x0108_0000]]);
        // The word already equals the literal's address: untouched.
        e.mem.set_u32(0x011f_4a14, 0x0108_0000);
        set_literal_fixed_string(&mut e, 0x011f_4a14, 0x0108_0000);
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 1);
        // An old empty-string handle is not released.
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(0x011f_4a18, empty);
        set_literal_fixed_string(&mut e, 0x011f_4a18, 0x0108_0004);
        assert_eq!(e.mem.u32(0x011f_4a18), 0x0108_1004);
    }
}

//! `fallout/magic/bhkcharacterlistenerspell.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00620b80` (`this`): the character proxy of a reference (a wrapper over `0044ddc0`); the
/// map files it under `cameracaster.cpp`.
const GET_HAVOK_PROXY: u32 = 0x0062_0b80;
/// `00458b20` (no arguments): answers the address `01267e30` (a static zero vector).
const ZERO_VECTOR: u32 = 0x0045_8b20;
/// A virtual thunk (`00c89720`, `this`): jumps through vtable slot 0x94; the engine map names
/// it `bhkCharacterProxy::UnMarkRead`.
const THUNK_SLOT_94: u32 = 0x00c8_9720;
/// `hkpCharacterProxy::getLinearVelocity` (Xbox PDB, `00cac950`, `this` = the Havok proxy).
const HKP_GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;

// Translated from 00807080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::GethkLinearVelocity` (Xbox PDB): the Havok character proxy of `this`
/// (`00620b80`); with none, the static zero vector (`00458b20`). With one, the call through
/// the thunk at `00c89720` (slot 0x94 of `this`) brackets the read of the proxy's linear
/// velocity (`hkpCharacterProxy::getLinearVelocity`), whose result is returned.
pub fn bhk_character_proxy_get_hk_linear_velocity(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(GET_HAVOK_PROXY, &args![this]).u32();
    let mut velocity = e.call(ZERO_VECTOR, &args![]).u32();
    if proxy != 0 {
        e.call(THUNK_SLOT_94, &args![this]);
        velocity = e.call(HKP_GET_LINEAR_VELOCITY, &args![proxy]).u32();
        e.call(THUNK_SLOT_94, &args![this]);
    }
    velocity
}

// Translated from 00806f00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01268344 without reading the receiver. The map names this body `bhkCharacterProxy::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00806f00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_8344
}

// Translated from 00806f10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (`bhkcharacterlistenerspell.cpp`): answers 0xc0 without reading the receiver;
/// takes only `this` (plain `RET`).
pub fn fn_00806f10(_e: &mut Engine, _this: Ptr) -> u32 {
    0xc0
}

/// `00807770` (thiscall on the first argument: an index; returns an object whose +0x48
/// the loop below reads).
const ENTRY_OBJECT_00807770: u32 = 0x0080_7770;
/// `004b59f0` (cdecl, one word; unnamed, `globalfunc.cpp`).
const WORD_TEST_004B59F0: u32 = 0x004b_59f0;
/// `0043b540` (thiscall, returns a word; unnamed, `modelloader.cpp`).
const VALUE_0043B540: u32 = 0x0043_b540;
/// `008c71b0` (thiscall on a stack word, one word; the map's name
/// `BGSChangeFlags::IsForcedChange` may come from folded code).
const STORE_008C71B0: u32 = 0x008c_71b0;
/// `0043b4d0` (thiscall on that stack word, returns a word; unnamed, `modelloader.cpp`).
const KIND_0043B4D0: u32 = 0x0043_b4d0;
/// `004a3f10` (thiscall, one word; unnamed).
const SET_004A3F10: u32 = 0x004a_3f10;
/// `005532a0` (thiscall on a stack block, four `float`s; returns a pointer; unnamed,
/// `tesobjectcell.cpp`).
const VECTOR_005532A0: u32 = 0x0055_32a0;
/// `00807730` (thiscall: one word; unnamed).
const OWNER_00807730: u32 = 0x0080_7730;
/// `0089f4e0` (thiscall on the player; unnamed, `actor.cpp`).
const PLAYER_VALUE_0089F4E0: u32 = 0x0089_f4e0;
/// The player pointer (`011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;

/// The two stores `fn_00806f20` makes into an entry: the receiver's Havok linear velocity
/// (`bhk_character_proxy_get_hk_linear_velocity`) through `004a3f10` on `entry + 0x10`,
/// then the vector `005532a0` builds in `block` from (0, 0, 0, 1) on `entry`.
fn store_velocity_and_unit_w(e: &mut Engine, this: Ptr, entry: u32, block: u32) {
    let velocity = bhk_character_proxy_get_hk_linear_velocity(e, this);
    e.call(SET_004A3F10, &args![entry.wrapping_add(0x10), velocity]);
    let vector = e
        .call(
            VECTOR_005532A0,
            &args![block, 0.0f32, 0.0f32, 0.0f32, 1.0f32],
        )
        .u32();
    e.call(SET_004A3F10, &args![entry, vector]);
}

// Translated from 00806f20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00806f20 - the /GS security cookie check (00ec408c) is not translated.
/// Unnamed (`bhkcharacterlistenerspell.cpp`; thiscall with two words, `RET 8`): walks the
/// `*(table + 0x4c)` entries of 0x40 bytes at `*(table + 0x48)`. For each index the object
/// `00807770(source, index)` gives is looked at through its word +0x48: when `004b59f0`
/// is 0 and the `0043b4d0` kind (of the `0043b540` word stored by `008c71b0`) is not 0x1e,
/// the entry gets this proxy's linear velocity (+0x10) and the vector (0, 0, 0, 1); when
/// `004b59f0` is set and `00807730` of that word is the player's `0089f4e0` word, the
/// entry gets the same and the walk ends.
pub fn fn_00806f20(e: &mut Engine, this: Ptr, source: u32, table: u32) {
    e.with_stack(0x60, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x60 - offset;
        let count = e.mem.i32(table + 0x4c);
        let mut index = 0i32;
        while index < count {
            let entry = ((index as u32) << 6).wrapping_add(e.mem.u32(table + 0x48));
            let object = e.call(ENTRY_OBJECT_00807770, &args![source, index]).u32();
            let word = e.mem.u32(object + 0x48);
            let test = e.call(WORD_TEST_004B59F0, &args![word]).u32();
            let word = e.mem.u32(object + 0x48);
            let value = e.call(VALUE_0043B540, &args![word]).u32();
            e.call(STORE_008C71B0, &args![local(0x20), value]);
            let kind = e.call(KIND_0043B4D0, &args![local(0x20)]).u32();
            if test == 0 {
                if kind != 0x1e {
                    store_velocity_and_unit_w(e, this, entry, local(0x40));
                }
            } else {
                let word = e.mem.u32(object + 0x48);
                let owner = e.call(OWNER_00807730, &args![this, word]).u32();
                if owner != 0 {
                    let player = e.global::<u32>(PLAYER_POINTER);
                    if owner == e.call(PLAYER_VALUE_0089F4E0, &args![player]).u32() {
                        store_velocity_and_unit_w(e, this, entry, local(0x60));
                        break;
                    }
                }
            }
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_6f00, fn_00806f00(Ptr) -> u32),
        entry!(0x0080_7080, bhk_character_proxy_get_hk_linear_velocity(Ptr) -> u32),
        entry!(0x0080_6f10, fn_00806f10(Ptr) -> u32),
        entry!(0x0080_6f20, fn_00806f20(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn rig(proxy: u32) -> (Engine, Rc<RefCell<Vec<(u32, u32)>>>) {
        let mut e = test_engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        e.register_double(GET_HAVOK_PROXY, move |_, _| Ret {
            eax: proxy,
            ..Ret::default()
        });
        e.register_double(ZERO_VECTOR, |_, _| Ret {
            eax: 0x0126_7e30,
            ..Ret::default()
        });
        let thunk = Rc::clone(&log);
        e.register_double(THUNK_SLOT_94, move |_, a| {
            thunk.borrow_mut().push((THUNK_SLOT_94, a[0]));
            Ret::default()
        });
        let velocity = Rc::clone(&log);
        e.register_double(HKP_GET_LINEAR_VELOCITY, move |_, a| {
            velocity.borrow_mut().push((HKP_GET_LINEAR_VELOCITY, a[0]));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        (e, log)
    }

    #[test]
    fn with_a_proxy_the_velocity_is_read_between_two_thunk_calls() {
        let (mut e, log) = rig(0x4444);
        assert_eq!(e.call(0x0080_7080, &args![0x1000u32]).u32(), 0x7777);
        assert_eq!(
            *log.borrow(),
            vec![
                (THUNK_SLOT_94, 0x1000),
                (HKP_GET_LINEAR_VELOCITY, 0x4444),
                (THUNK_SLOT_94, 0x1000)
            ]
        );
    }

    #[test]
    fn without_a_proxy_the_zero_vector_is_returned() {
        let (mut e, log) = rig(0);
        assert_eq!(e.call(0x0080_7080, &args![0x1000u32]).u32(), 0x0126_7e30);
        assert!(log.borrow().is_empty());
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00806f00_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0080_6f00, &args![0x1000u32]).u32(), 0x0126_8344);
        assert_eq!(e.call(0x0080_6f00, &args![0u32]).u32(), 0x0126_8344);
    }
}

#[cfg(test)]
mod listener_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00806f10_answers_0xc0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0080_6f10, &args![0x1234u32]).u32(), 0xc0);
    }

    /// A table of `count` entries at 0x4000 and doubles: `00807770` gives an object whose
    /// +0x48 is the index, `004b59f0` answers `tests[index]`, the kind is 0x1e for index 1,
    /// `00807730` answers the word + 0x100 and the player's word is 0x102.
    fn rig(count: i32, tests: Vec<u32>) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, 0x0700_0000u32);
        let table = e.mem.alloc(0x50);
        e.mem.set_i32(table + 0x4c, count);
        e.mem.set_u32(table + 0x48, 0x4000);
        e.register(ENTRY_OBJECT_00807770, |e, a| {
            let object = e.mem.alloc(0x50);
            e.mem.set_u32(object + 0x48, a[1]);
            object.into_ret()
        });
        e.register_double(WORD_TEST_004B59F0, move |_, a| {
            tests[a[0] as usize].into_ret()
        });
        e.register(VALUE_0043B540, |_, a| a[0].into_ret());
        e.register(STORE_008C71B0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(KIND_0043B4D0, |e, a| {
            (if e.mem.u32(a[0]) == 1 { 0x1eu32 } else { 0 }).into_ret()
        });
        e.register(OWNER_00807730, |_, a| (a[1] + 0x100).into_ret());
        answer(&mut e, PLAYER_VALUE_0089F4E0, 0x102);
        answer(&mut e, GET_HAVOK_PROXY, 0);
        answer(&mut e, ZERO_VECTOR, 0x0126_7e30);
        answer(&mut e, SET_004A3F10, 0);
        e.register(VECTOR_005532A0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        (e, table)
    }

    #[test]
    fn fn_00806f20_stores_velocity_for_untested_entries_except_kind_0x1e() {
        let (mut e, table) = rig(3, vec![0, 0, 0]);
        e.call(0x0080_6f20, &args![0x1000u32, 0x2000u32, table]);
        let sets = calls(&e, SET_004A3F10);
        // Entries 0 and 2 (entry 1 has kind 0x1e): velocity at +0x10, then the vector.
        assert_eq!(sets.len(), 4);
        assert_eq!(sets[0], vec![0x4010, 0x0126_7e30]);
        assert_eq!(sets[1][0], 0x4000);
        assert_eq!(sets[2][0], 0x4090);
        assert_eq!(sets[3][0], 0x4080);
        let vectors = calls(&e, VECTOR_005532A0);
        assert_eq!(vectors[0][1..], [0, 0, 0, 0x3f80_0000]);
        assert_eq!(sets[1][1], vectors[0][0]);
        assert_eq!(calls(&e, ENTRY_OBJECT_00807770)[2], vec![0x2000, 2]);
    }

    #[test]
    fn fn_00806f20_stops_at_the_players_entry() {
        let (mut e, table) = rig(4, vec![1, 1, 1, 1]);
        e.call(0x0080_6f20, &args![0x1000u32, 0x2000u32, table]);
        // Index 2 owns 0x102: stored there, then the walk ends.
        let sets = calls(&e, SET_004A3F10);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][0], 0x4090);
        assert_eq!(sets[1][0], 0x4080);
        assert_eq!(calls(&e, ENTRY_OBJECT_00807770).len(), 3);
        assert_eq!(calls(&e, OWNER_00807730)[0], vec![0x1000, 0]);
    }
}

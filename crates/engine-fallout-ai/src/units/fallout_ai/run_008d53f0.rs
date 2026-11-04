//! `fallout/ai/run_008d53f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `GamePlayFormulas::GetRequiredExperiencePoints` (engine map, `00648b50`; `cdecl`, the
/// level).
const REQUIRED_EXPERIENCE: u32 = 0x0064_8b50;
/// The global object at `011dea3c` the level and experience are read from.
const OBJECT_011DEA3C: u32 = 0x011d_ea3c;
/// `0087f9f0` (`thiscall`, no stack words; unit `fallout/ai/actor.cpp`): answers the 16-bit
/// level of the object.
const GET_LEVEL: u32 = 0x0087_f9f0;
/// `0043d4d0` (`thiscall` on `011d0c60`, no stack words; unit `fallout shared/modelloader.cpp`):
/// answers the address of the level limit (a signed word).
const LEVEL_LIMIT_OWNER: u32 = 0x011d_0c60;
const GET_LEVEL_LIMIT: u32 = 0x0043_d4d0;
/// `00865e50` (`thiscall`, three words, unit `fallout/misc/saveload/bgssavegamebuffer.cpp`):
/// called with (address, size, 0) for each member `fn_008d56c0` writes.
const BUFFER_WRITE: u32 = 0x0086_5e50;
/// `00464e10` (`thiscall`, no stack words; unit `fallout shared/tesdatahandler.cpp`) and
/// `00559450` (`thiscall`, no stack words): together give the level-up state object
/// `fn_008d53f0` reads (a `fn_008d55b0` object, by its byte +8 and `fn_008d55e0`).
const OWNER_OBJECT: u32 = 0x0046_4e10;
const GET_MEMBER: u32 = 0x0055_9450;
/// `_ftol2` (`00ec62c0`, the float in `ST0`) and `0077c440` (`cdecl`, one word; unit
/// `fallout/interface/menus/hudmainmenu.cpp`).
const FTOL2: u32 = 0x00ec_62c0;
const HUD_GAIN: u32 = 0x0077_c440;
/// The double 0.0 the change is compared with (`.rdata`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The actor value index the callback reacts to (by `fn_008d55e0`, the experience value).
const EXPERIENCE_INDEX: u32 = 0x18;

// Translated from 008d55b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): clears the byte at `this`, stores
/// `GetRequiredExperiencePoints(2)` at +0x4 and clears the byte at +0x8.
pub fn fn_008d55b0(e: &mut Engine, this: u32) {
    e.mem.set_u8(this, 0);
    let required = e.call(REQUIRED_EXPERIENCE, &args![2u32]).u32();
    e.mem.set_u32(this + 4, required);
    e.mem.set_u8(this + 8, 0);
}

// Translated from 008d5590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a constructor shape): runs
/// `fn_008d55b0` and answers `this`.
pub fn fn_008d5590(e: &mut Engine, this: u32) -> u32 {
    fn_008d55b0(e, this);
    this
}

// Translated from 008d56c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): writes the byte at `this`
/// (size 1) and the word at +4 (size 4) through `00865e50` on `buffer`.
pub fn fn_008d56c0(e: &mut Engine, this: u32, buffer: u32) {
    e.call(BUFFER_WRITE, &args![buffer, this, 1u32, 0u32]);
    e.call(BUFFER_WRITE, &args![buffer, this + 4, 4u32, 0u32]);
}

// Translated from 008d5660 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): `fn_008d56c0(buffer)` on the
/// object held at `this` (not checked for null, as in the exe).
pub fn fn_008d5660(e: &mut Engine, this: u32, buffer: u32) {
    let held = e.mem.u32(this);
    fn_008d56c0(e, held, buffer);
}

// Translated from 008d55e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): false while the level of the
/// object at `011dea3c` (`0087f9f0`) is not below the limit (`0043d4d0`); otherwise, when
/// that object's value 0x18 (slot +0x8 of its +0xA4 interface) has reached
/// `GetRequiredExperiencePoints(level)`, sets the byte at `this` and answers true.
pub fn fn_008d55e0(e: &mut Engine, this: u32, level: u32) -> bool {
    let object = e.mem.u32(OBJECT_011DEA3C);
    let current = i32::from(e.call(GET_LEVEL, &args![object]).u16());
    let limit = e.call(GET_LEVEL_LIMIT, &args![LEVEL_LIMIT_OWNER]).u32();
    if current >= e.mem.i32(limit) {
        return false;
    }
    let required = e.call(REQUIRED_EXPERIENCE, &args![level]).i32();
    let object = e.mem.u32(OBJECT_011DEA3C);
    let experience = e.vcall(object + 0xa4, 0x8, &args![EXPERIENCE_INDEX]).i32();
    if experience < required {
        return false;
    }
    e.mem.set_u8(this, 1);
    true
}

// Translated from 008d53f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words; the modified callback `RegisterActorValues`
/// gives value 0x18, the shape of `Actor::ConditionModifiedCallback`: (owner, index,
/// previous, change, source); the third and fifth are not read): for an owner whose actor
/// (`owner - 0xA4`) answers true to vtable slot +0x360, and while the level-up state's byte
/// +8 is clear: when value 0x18 rose (`change` above 0.0, ordered), shows the gain
/// (`0077c440` with `_ftol2(change)` when that is positive) and, while the level is below
/// the limit, runs `fn_008d55e0(level + 1)` on the state.
pub fn fn_008d53f0(
    e: &mut Engine,
    owner: u32,
    index: u32,
    _unused_2: u32,
    change: u32,
    _unused_4: u32,
) {
    let actor = if owner != 0 { owner - 0xa4 } else { 0 };
    if actor == 0 || !e.vcall(actor, 0x360, &[]).bool() {
        return;
    }
    let player = e.mem.u32(OBJECT_011DEA3C);
    let holder = e.call(OWNER_OBJECT, &args![player]).u32();
    let state = e.call(GET_MEMBER, &args![holder]).u32();
    if e.mem.u8(state + 8) != 0 || index != EXPERIENCE_INDEX {
        return;
    }
    // FCOMP; TEST AH,0x41; JNZ: below, equal or unordered leaves.
    if !(x87_load32(change) > e.mem.f64(ZERO_DOUBLE)) {
        return;
    }
    if e.call(FTOL2, &args![x87_load32(change)]).i32() > 0 {
        let gain = e.call(FTOL2, &args![x87_load32(change)]).u32();
        e.call(HUD_GAIN, &args![gain]);
    }
    let player = e.mem.u32(OBJECT_011DEA3C);
    let level = i32::from(e.call(GET_LEVEL, &args![player]).u16());
    let limit = e.call(GET_LEVEL_LIMIT, &args![LEVEL_LIMIT_OWNER]).u32();
    if level >= e.mem.i32(limit) {
        return;
    }
    let player = e.mem.u32(OBJECT_011DEA3C);
    let next = u32::from(e.call(GET_LEVEL, &args![player]).u16()) + 1;
    let state = e.call(GET_MEMBER, &args![holder]).u32();
    fn_008d55e0(e, state, next);
}

/// `TESForm::SaveGameDataOLD` (`00484ce0`) and `TESForm::LoadGameDataOLD`
/// (`00484d00`), `thiscall(address, size)` on the form (Xbox PDB names), and
/// `00864980` (`thiscall(address, size)` on a load buffer).
const SAVE_DATA: u32 = 0x0048_4ce0;
const LOAD_DATA: u32 = 0x0048_4d00;
const BUFFER_READ: u32 = 0x0086_4980;

// Translated from 008d54e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size `fn_008d5510` saves: 1 + 4 (16-bit); the stack word (`RET 4`)
/// is not read.
pub fn fn_008d54e0(_e: &mut Engine, _this: u32, _unused_1: u32) -> u16 {
    1 + 4
}

// Translated from 008d5510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the level-up state (`00559450` on `this`) through the object at
/// `011dea3c`: its byte (size 1) and the word at +4 (size 4). The stack word
/// (`RET 4`) is not read.
pub fn fn_008d5510(e: &mut Engine, this: u32, _unused_1: u32) {
    let player = e.mem.u32(OBJECT_011DEA3C);
    let state = e.call(GET_MEMBER, &args![this]).u32();
    e.call(SAVE_DATA, &args![player, state, 1u32]);
    let state = e.call(GET_MEMBER, &args![this]).u32();
    let player = e.mem.u32(OBJECT_011DEA3C);
    e.call(SAVE_DATA, &args![player, state + 4, 4u32]);
}

// Translated from 008d5550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads what `fn_008d5510` saves (`TESForm::LoadGameDataOLD`); the two stack
/// words (`RET 8`) are not read.
pub fn fn_008d5550(e: &mut Engine, this: u32, _unused_1: u32, _unused_2: u32) {
    let player = e.mem.u32(OBJECT_011DEA3C);
    let state = e.call(GET_MEMBER, &args![this]).u32();
    e.call(LOAD_DATA, &args![player, state, 1u32]);
    let state = e.call(GET_MEMBER, &args![this]).u32();
    let player = e.mem.u32(OBJECT_011DEA3C);
    e.call(LOAD_DATA, &args![player, state + 4, 4u32]);
}

// Translated from 008d5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_008d56f0(buffer)` on the object held at `this` (not checked for null,
/// as in the exe).
pub fn fn_008d5680(e: &mut Engine, this: u32, buffer: u32) {
    let held = e.mem.u32(this);
    fn_008d56f0(e, held, buffer);
}

// Translated from 008d56a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_008d5720` on the object held at `this` (not checked for null, as in
/// the exe), with the stack word passed on.
pub fn fn_008d56a0(e: &mut Engine, this: u32, value: u32) {
    let held = e.mem.u32(this);
    fn_008d5720(e, held, value);
}

// Translated from 008d56f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `this` (size 1) and the word at +4 (size 4) through
/// `00864980` on `buffer` (the load side of `fn_008d56c0`).
pub fn fn_008d56f0(e: &mut Engine, this: u32, buffer: u32) {
    e.call(BUFFER_READ, &args![buffer, this, 1u32]);
    e.call(BUFFER_READ, &args![buffer, this + 4, 4u32]);
}

// Translated from 008d5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this` and stores `GetRequiredExperiencePoints(2)` at
/// +4 (the start of `fn_008d55b0` without its last store); the stack word
/// (`RET 4`) is not read.
pub fn fn_008d5720(e: &mut Engine, this: u32, _unused_1: u32) {
    e.mem.set_u8(this, 0);
    let required = e.call(REQUIRED_EXPERIENCE, &args![2u32]).u32();
    e.mem.set_u32(this + 4, required);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_54e0, fn_008d54e0(u32, u32) -> u16),
        entry!(0x008d_5510, fn_008d5510(u32, u32)),
        entry!(0x008d_5550, fn_008d5550(u32, u32, u32)),
        entry!(0x008d_5680, fn_008d5680(u32, u32)),
        entry!(0x008d_56a0, fn_008d56a0(u32, u32)),
        entry!(0x008d_56f0, fn_008d56f0(u32, u32)),
        entry!(0x008d_5720, fn_008d5720(u32, u32)),
        entry!(0x008d_55b0, fn_008d55b0(u32)),
        entry!(0x008d_5590, fn_008d5590(u32) -> u32),
        entry!(0x008d_56c0, fn_008d56c0(u32, u32)),
        entry!(0x008d_5660, fn_008d5660(u32, u32)),
        entry!(0x008d_55e0, fn_008d55e0(u32, u32) -> bool),
        entry!(0x008d_53f0, fn_008d53f0(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_008d55b0_and_fn_008d5590_reset_the_state() {
        let mut e = Engine::new();
        e.register(REQUIRED_EXPERIENCE, |_, a| (a[0] * 100).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0xffff_ffff);
        e.mem.set_u32(this + 8, 0xffff_ffff);
        assert_eq!(e.call(0x008d_5590, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 4), 200);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ff00);
    }

    #[test]
    fn fn_008d5660_writes_both_members_of_the_held_object() {
        let mut e = Engine::new();
        e.register(BUFFER_WRITE, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x008d_5660, &[this, 0x9000]);
        assert_eq!(
            calls_to(&e, BUFFER_WRITE),
            vec![vec![0x9000, 0x7000, 1, 0], vec![0x9000, 0x7004, 4, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x008d_56c0, &[0x7100, 0x9000]);
        assert_eq!(calls_to(&e, BUFFER_WRITE).len(), 2);
    }

    /// The global object (level at +0x1F0 through `GET_LEVEL`, its +0xA4 interface answering
    /// the word at +0x1F4) and the level limit word.
    fn world(e: &mut Engine, level: u32, experience: u32, limit: i32) -> u32 {
        e.map(OBJECT_011DEA3C & !0xfff, 0x1000);
        e.map(LEVEL_LIMIT_OWNER & !0xfff, 0x1000);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        let player = e.mem.alloc(0x200);
        let values_table = e.mem.alloc(0x10);
        e.mem.set_u32(values_table + 8, 0x0090_1000);
        e.mem.set_u32(player + 0xa4, values_table);
        e.mem.set_u32(player + 0x1f0, level);
        e.mem.set_u32(player + 0x1f4, experience);
        e.mem.set_u32(OBJECT_011DEA3C, player);
        let limit_word = e.mem.alloc(4);
        e.mem.set_u32(limit_word, limit as u32);
        e.register(GET_LEVEL, |e, a| e.mem.u32(a[0] + 0x1f0).into_ret());
        e.register(0x0090_1000, |e, a| {
            e.mem.u32(a[0] - 0xa4 + 0x1f4).into_ret()
        });
        e.register_double(GET_LEVEL_LIMIT, move |_, _| limit_word.into_ret());
        e.register(REQUIRED_EXPERIENCE, |_, a| (a[0] * 100).into_ret());
        player
    }

    #[test]
    fn fn_008d55e0_sets_the_byte_when_the_experience_suffices() {
        let mut e = Engine::new();
        let player = world(&mut e, 4, 500, 30);
        let this = e.mem.alloc(4);
        assert!(e.call(0x008d_55e0, &[this, 5]).bool());
        assert_eq!(e.mem.u8(this), 1);
        e.mem.set_u8(this, 0);
        assert!(!e.call(0x008d_55e0, &[this, 6]).bool());
        assert_eq!(e.mem.u8(this), 0);
        // At the limit: false without asking for the experience.
        e.mem.set_u32(player + 0x1f0, 30);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008d_55e0, &[this, 1]).bool());
        assert!(calls_to(&e, REQUIRED_EXPERIENCE).is_empty());
    }

    #[test]
    fn fn_008d53f0_reacts_to_experience_gains() {
        let mut e = Engine::new();
        world(&mut e, 4, 500, 30);
        let state = e.mem.alloc(0xc);
        e.register(OWNER_OBJECT, |_, _| 0x4400u32.into_ret());
        e.register_double(GET_MEMBER, move |_, _| state.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(HUD_GAIN, |_, _| Ret::default());
        // An actor whose slot +0x360 answers true.
        let actor = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0x360, 0x0090_2000);
        e.mem.set_u32(actor, table);
        e.register(0x0090_2000, |_, _| 1u32.into_ret());
        let owner = actor + 0xa4;

        e.call_log = Some(vec![]);
        e.call(0x008d_53f0, &[owner, 0x18, 0, 25.0f32.to_bits(), 0]);
        assert_eq!(calls_to(&e, HUD_GAIN), vec![vec![25]]);
        // Level 4 + 1 = 5 needs 500: reached.
        assert_eq!(calls_to(&e, REQUIRED_EXPERIENCE), vec![vec![5]]);
        assert_eq!(e.mem.u8(state), 1);

        // Another value, a fall, a NaN, or a null owner: nothing.
        for (owner, index, change) in [
            (owner, 0x19, 25.0f32),
            (owner, 0x18, -2.0),
            (owner, 0x18, 0.0),
            (owner, 0x18, f32::NAN),
            (0, 0x18, 25.0),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x008d_53f0, &[owner, index, 0, change.to_bits(), 0]);
            assert!(calls_to(&e, HUD_GAIN).is_empty());
            assert!(calls_to(&e, REQUIRED_EXPERIENCE).is_empty());
        }

        // A gain below one shows nothing but still checks the level.
        e.mem.set_u8(state, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_53f0, &[owner, 0x18, 0, 0.5f32.to_bits(), 0]);
        assert!(calls_to(&e, HUD_GAIN).is_empty());
        assert_eq!(calls_to(&e, REQUIRED_EXPERIENCE), vec![vec![5]]);

        // The state's byte +8 set: nothing.
        e.mem.set_u8(state + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x008d_53f0, &[owner, 0x18, 0, 25.0f32.to_bits(), 0]);
        assert!(calls_to(&e, FTOL2).is_empty());
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_008d54e0_answers_five() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x008d_54e0, &[0, 9]).u16(), 5);
    }

    #[test]
    fn save_and_load_go_through_the_player_form() {
        let mut e = Engine::new();
        e.map(OBJECT_011DEA3C, 4);
        e.mem.set_u32(OBJECT_011DEA3C, 0x9a7e);
        e.register(GET_MEMBER, |_, _| Ret {
            eax: 0x5100,
            ..Ret::default()
        });
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(LOAD_DATA, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_5510, &[0x4000, 0]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![vec![0x9a7e, 0x5100, 1], vec![0x9a7e, 0x5104, 4]]
        );
        e.call(0x008d_5550, &[0x4000, 0, 0]);
        assert_eq!(
            calls_to(&e, LOAD_DATA),
            vec![vec![0x9a7e, 0x5100, 1], vec![0x9a7e, 0x5104, 4]]
        );
        assert_eq!(calls_to(&e, GET_MEMBER), vec![vec![0x4000]; 4]);
    }

    #[test]
    fn the_holder_forwards_to_the_held_state() {
        let mut e = Engine::new();
        let state = e.mem.alloc(8);
        e.mem.set_u8(state, 1);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, state);
        e.register(BUFFER_READ, |_, _| Ret::default());
        e.register(REQUIRED_EXPERIENCE, |_, a| Ret {
            eax: a[0] * 100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008d_5680, &[holder, 0xb0f0]);
        assert_eq!(
            calls_to(&e, BUFFER_READ),
            vec![vec![0xb0f0, state, 1], vec![0xb0f0, state + 4, 4]]
        );
        e.call(0x008d_56a0, &[holder, 0]);
        assert_eq!(e.mem.u8(state), 0);
        assert_eq!(e.mem.u32(state + 4), 200);
    }
}

//! `fallout/ai/combat/combatprocedureevade.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0081b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 4 (`thiscall`, `this` unread). The engine map names it
/// `CombatProcedureEvade::GetType`; the body is shared by the linker
/// (several unrelated vtables point to it), so it is named by address.
pub fn fn_0081b6c0(_e: &mut Engine, _this: Ptr) -> u32 {
    4
}

layout! {
    /// `CombatProcedureEvade` (Xbox PDB), 0x18 bytes; the offsets are the
    /// ones the PC code uses.
    pub struct CombatProcedureEvade: 0x18 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 setting up the path, 1 evading.
        0x10 eState: u32,
        /// `iLocation` (Xbox PDB): the kind of evasion (0xa, 0xc and 0xe are
        /// the cases `Update` tells apart).
        0x14 iLocation: i32,
    }
}

/// `CombatProcedure` methods (Xbox PDB names).
const GET_ATTACKER: u32 = 0x0099_6de0;
const SET_MOVE_MODE: u32 = 0x0099_6e20;
const CLEAR_MOVE_MODE: u32 = 0x0099_6e40;
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `009caac0` (one word): fails with the reason; `009caae0`: completes.
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
const COMPLETE_PROCEDURE: u32 = 0x009c_aae0;
/// Controller getters: `00586150` the combat state, `004fb070` the combat
/// group.
const GET_COMBAT_STATE: u32 = 0x0058_6150;
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// `Actor` methods (Xbox PDB names) and unnamed helpers: `008b3b90` (a path
/// test, `AL`), `004181e0` (an object whose `+0x30` has a vtable; its slot
/// `+0x28` gives a byte).
const STOP_MOVING_ACTOR: u32 = 0x008b_3ab0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const GET_CURRENT_PATHFINDING_GOAL: u32 = 0x008b_3840;
const SET_PATHFINDING_FLEE: u32 = 0x008b_b630;
const ACTOR_PATH_TEST: u32 = 0x008b_3b90;
const ACTOR_OBJECT_4181E0: u32 = 0x0041_81e0;
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// Actor vtable slots: `+0x1f4` the position, `+0x2bc` the heading (one
/// word, `ST0`), `+0x21c` (`AL`).
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_HEADING: u32 = 0x2bc;
const SLOT_TEST_21C: u32 = 0x21c;
const SLOT_FLAG_28: u32 = 0x28;
/// The flee request (`PathingRequestFlee::PathingRequestFlee` (Xbox PDB)
/// and its destructor `006dad70`) and its setters (`__thiscall`, one word
/// each): `006e29f0` the actor, `006e2bb0` the heading, `006e2b50` a flag,
/// `00507610` the distance, `008df000` (no words), `006d1bc0` a pathing
/// location, `006d61e0` the avoid nodes, `008ad8f0` a flag.
const FLEE_REQUEST_CONSTRUCT: u32 = 0x006e_5390;
const FLEE_REQUEST_DESTRUCT: u32 = 0x006d_ad70;
const REQUEST_SET_ACTOR: u32 = 0x006e_29f0;
const REQUEST_SET_HEADING: u32 = 0x006e_2bb0;
const REQUEST_SET_FLAG_2B50: u32 = 0x006e_2b50;
const REQUEST_SET_DISTANCE: u32 = 0x0050_7610;
const REQUEST_CLEAR_GOAL: u32 = 0x008d_f000;
const REQUEST_SET_LOCATION: u32 = 0x006d_1bc0;
const REQUEST_SET_AVOID: u32 = 0x006d_61e0;
const REQUEST_SET_FLAG_D8F0: u32 = 0x008a_d8f0;
/// `PathingLocation::PathingLocation_ov4` (Xbox PDB) and its destructor
/// `004ff7e0`; `006815c0` (a point constructor returning `this`).
const PATHING_LOCATION_FROM_POINT: u32 = 0x006d_ce10;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// The avoid nodes: `009a57a0` (`__thiscall` on the combat manager's word
/// `+0x14`, read through `00825c00`; three words), `009892a0` and
/// `009895a0` (`__thiscall` on the combat group; five and two words).
const MANAGER_WORD_14: u32 = 0x0082_5c00;
const THREAT_AVOID_NODES: u32 = 0x009a_57a0;
const GROUP_AVOID_NODES: u32 = 0x0098_92a0;
const GROUP_FLEE_AVOID_NODES: u32 = 0x0098_95a0;
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `CombatState::BoostConfidence` (Xbox PDB), `009a0680` and `0099aa10`
/// (`__thiscall` on the combat state).
const BOOST_CONFIDENCE: u32 = 0x009a_1280;
const STATE_REFRESH: u32 = 0x009a_0680;
const STATE_REACHED: u32 = 0x0099_aa10;
/// `CombatDialogueManager::StartDialogue` (Xbox PDB) on the manager at
/// `011f1708`.
const START_DIALOGUE: u32 = 0x0098_3970;
const DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// `00403e20` (`__thiscall` on a setting): the address of its value;
/// `fCombatFleeNormalDistance` (named by its static initializer).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_COMBAT_FLEE_NORMAL_DISTANCE: u32 = 0x011c_ea80;
/// `float` constants of the exe: -1, 128, 384.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const HUNDRED_TWENTY_EIGHT_FLOAT: u32 = 0x0101_e704;
const THREE_HUNDRED_EIGHTY_FOUR_FLOAT: u32 = 0x0105_0b98;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// Strings.
const TEXT_CANNOT_SETUP_EVADE_PATH: u32 = 0x0109_1a08;
const TEXT_PATHING_FAILED: u32 = 0x0109_19e8;
const TEXT_PATH_BUILDING_FAILED: u32 = 0x0109_19c8;

fn controller_of(e: &Engine, this: Ptr<CombatProcedureEvade>) -> u32 {
    e.get(this, CombatProcedureEvade::pCombatController).addr()
}

// Translated from 009d5b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEvade::Shutdown` (Xbox PDB): stops the attacker and
/// clears the move mode.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureEvade>) {
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(STOP_MOVING_ACTOR, &args![attacker]);
    e.call(CLEAR_MOVE_MODE, &args![this]);
}

// Translated from 009d5b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d5b90 - the compiler's FS:[0] exception frame (unwinding of the flee request and the pathing location) is not translated
/// `CombatProcedureEvade::Update` (Xbox PDB). State 0 builds a flee
/// request from the attacker's position and heading: kinds 0xa and 0xc flee
/// `fCombatFleeNormalDistance` avoiding the group (`009895a0`); kind 0xe
/// says the combat line, flees 128 from its current goal (or without one)
/// avoiding the threats (`009a57a0`) and the group within 384
/// (`009892a0`). Without the flee path it fails ("Cannot setup evade
/// path"); otherwise it evades (state 1, move mode 3). State 1: a failed
/// path (kind 0xc without a valid path lowers the confidence) refreshes
/// the combat state and fails; a finished path reports the reached
/// location and completes.
///
/// Locals follow the disassembly's frame (offsets below `EBP`): the
/// position `-0xe0`, the request `-0xd4`, the goal `-0xec`, the pathing
/// location `-0x114`, the world location `-0x124`.
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureEvade>) {
    const FRAME: u32 = 0x130;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        match e.get(this, CombatProcedureEvade::eState) {
            0 => {
                let position = ebp - 0xe0;
                let request = ebp - 0xd4;
                let from = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(from + i * 4);
                    e.mem.set_u32(position + i * 4, word);
                }
                e.call(FLEE_REQUEST_CONSTRUCT, &args![request]);
                e.call(REQUEST_SET_ACTOR, &args![request, attacker]);
                let heading = e.vcall(attacker, SLOT_HEADING, &args![0u32]).f64();
                e.call(REQUEST_SET_HEADING, &args![request, x87_store32(heading)]);
                if e.vcall(attacker, SLOT_TEST_21C, &args![]).bool() {
                    let object = e.call(ACTOR_OBJECT_4181E0, &args![attacker]).u32() + 0x30;
                    let flag = e.vcall(object, SLOT_FLAG_28, &args![]).u8();
                    e.call(REQUEST_SET_FLAG_2B50, &args![request, u32::from(flag)]);
                }
                let mut avoid = 0u32;
                match e.get(this, CombatProcedureEvade::iLocation) {
                    0xa | 0xc => {
                        let value = e
                            .call(SETTING_VALUE, &args![SETTING_COMBAT_FLEE_NORMAL_DISTANCE])
                            .u32();
                        let distance = x87_quiet_bits(e.mem.u32(value));
                        e.call(REQUEST_SET_DISTANCE, &args![request, distance]);
                        e.call(REQUEST_CLEAR_GOAL, &args![request]);
                        let controller = controller_of(e, this);
                        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                        avoid = e
                            .call(GROUP_FLEE_AVOID_NODES, &args![group, attacker, 0u32])
                            .u32();
                    }
                    0xe => {
                        let manager = e.global::<u32>(DIALOGUE_MANAGER);
                        let controller = controller_of(e, this);
                        e.call(
                            START_DIALOGUE,
                            &args![manager, controller, 2u32, 5u32, 1u32],
                        );
                        let distance = x87_quiet_bits(e.global::<u32>(HUNDRED_TWENTY_EIGHT_FLOAT));
                        e.call(REQUEST_SET_DISTANCE, &args![request, distance]);
                        let goal = ebp - 0xec;
                        e.call(POINT_CONSTRUCT, &args![goal]);
                        if e.call(GET_CURRENT_PATHFINDING_GOAL, &args![attacker, goal])
                            .bool()
                        {
                            let location = ebp - 0x114;
                            let made = e
                                .call(
                                    PATHING_LOCATION_FROM_POINT,
                                    &args![location, goal, attacker],
                                )
                                .u32();
                            e.call(REQUEST_SET_LOCATION, &args![request, made]);
                            e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
                        } else {
                            e.call(REQUEST_CLEAR_GOAL, &args![request]);
                        }
                        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
                        let manager = e.global::<u32>(COMBAT_MANAGER);
                        let threats = e.call(MANAGER_WORD_14, &args![manager]).u32();
                        avoid = e
                            .call(
                                THREAT_AVOID_NODES,
                                &args![threats, position, attacker, minus_one],
                            )
                            .u32();
                        let radius =
                            x87_quiet_bits(e.global::<u32>(THREE_HUNDRED_EIGHTY_FOUR_FLOAT));
                        let controller = controller_of(e, this);
                        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                        avoid = e
                            .call(
                                GROUP_AVOID_NODES,
                                &args![group, attacker, position, radius, avoid, 0u32],
                            )
                            .u32();
                    }
                    _ => {}
                }
                e.call(REQUEST_SET_AVOID, &args![request, avoid]);
                e.call(REQUEST_SET_FLAG_D8F0, &args![request, 1u32]);
                if !e
                    .call(SET_PATHFINDING_FLEE, &args![attacker, request])
                    .bool()
                {
                    e.call(FAIL_PROCEDURE, &args![t, TEXT_CANNOT_SETUP_EVADE_PATH]);
                    e.call(FLEE_REQUEST_DESTRUCT, &args![request]);
                    return;
                }
                e.set(this, CombatProcedureEvade::eState, 1);
                e.call(SET_MOVE_MODE, &args![t, 3u32]);
                e.call(FLEE_REQUEST_DESTRUCT, &args![request]);
            }
            1 => {
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    if !e.call(IS_PATH_VALID, &args![attacker]).bool()
                        && e.get(this, CombatProcedureEvade::iLocation) == 0xc
                    {
                        e.call(BOOST_CONFIDENCE, &args![combat_state, 0u32]);
                    }
                    e.call(STATE_REFRESH, &args![combat_state]);
                    let reason = if e.call(IS_PATH_VALID, &args![attacker]).bool() {
                        TEXT_PATHING_FAILED
                    } else {
                        TEXT_PATH_BUILDING_FAILED
                    };
                    e.call(FAIL_PROCEDURE, &args![t, reason]);
                } else if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
                    let location = ebp - 0x124;
                    let kind = e.get(this, CombatProcedureEvade::iLocation) as u32;
                    let location = e.call(GET_WORLD_LOCATION, &args![attacker, location]).u32();
                    e.call(STATE_REACHED, &args![combat_state, location, kind]);
                    e.call(COMPLETE_PROCEDURE, &args![t]);
                }
            }
            _ => {}
        }
    });
}

// Translated from 009d5fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEvade::SaveGame` (Xbox PDB): the base class, the state
/// and the kind.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureEvade>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x14, 4u32, 0u32]);
}

// Translated from 009d5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEvade::LoadGame` (Xbox PDB): the mirror of `SaveGame`.
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureEvade>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x14, 4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_b6c0, fn_0081b6c0(Ptr) -> u32),
        entry!(0x009d_5b60, shutdown(Ptr<CombatProcedureEvade>)),
        entry!(0x009d_5b90, update(Ptr<CombatProcedureEvade>)),
        entry!(0x009d_5fb0, save_game(Ptr<CombatProcedureEvade>, u32)),
        entry!(0x009d_5ff0, load_game(Ptr<CombatProcedureEvade>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0081b6c0_returns_four() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_b6c0, &args![0x1234u32]).u32(), 4);
    }

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every fake object's vtable: slot `n` holds `SLOT_BASE + n`, a double.
    const FAKE_VTABLE: u32 = 0x7e00_0000;
    const SLOT_BASE: u32 = 0x7e10_0000;
    const SLOTS: u32 = 0x800;
    /// Where the setting double puts each setting's value.
    const SETTING_CELLS: u32 = 0x7e20_0000;
    const SETTING_BASE: u32 = 0x011c_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// What the doubles answer: a queue that is used up first, then a
    /// fixed answer, then 0.
    #[derive(Default)]
    struct Script {
        queues: HashMap<u32, VecDeque<Ret>>,
        fixed: HashMap<u32, Ret>,
    }

    struct Rig {
        e: Engine,
        script: Rc<RefCell<Script>>,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            e.map(FAKE_VTABLE, SLOTS);
            for slot in (0..SLOTS).step_by(4) {
                e.mem.set_u32(FAKE_VTABLE + slot, SLOT_BASE + slot);
            }
            e.map(SETTING_CELLS, 0x4_0000);
            exe_data(&mut e);
            let mut rig = Rig {
                e,
                script: Rc::default(),
            };
            for &addr in CALLEES {
                rig.double(addr);
            }
            for slot in (0..SLOTS).step_by(4) {
                rig.double(SLOT_BASE + slot);
            }
            rig.e.register(SETTING_VALUE, |_, a| {
                ret(SETTING_CELLS + (a[0] - SETTING_BASE))
            });
            rig
        }

        fn double(&mut self, addr: u32) {
            let script = Rc::clone(&self.script);
            self.e.register_double(addr, move |_, _| {
                let mut script = script.borrow_mut();
                if let Some(answer) = script.queues.get_mut(&addr).and_then(VecDeque::pop_front) {
                    return answer;
                }
                script.fixed.get(&addr).copied().unwrap_or_default()
            });
        }

        fn answer(&self, addr: u32, value: u32) {
            self.script.borrow_mut().fixed.insert(addr, ret(value));
        }

        fn answer_float(&self, addr: u32, value: f64) {
            self.script.borrow_mut().fixed.insert(addr, float(value));
        }

        /// The answer of the fake vtable's slot `slot`.
        fn slot(&self, slot: u32, value: u32) {
            self.answer(SLOT_BASE + slot, value);
        }

        fn setting(&mut self, setting: u32, value: f32) {
            self.e
                .mem
                .set_f32(SETTING_CELLS + (setting - SETTING_BASE), value);
        }

        /// A zeroed block whose first word is the fake vtable.
        fn object(&mut self, size: u32) -> u32 {
            let block = self.e.mem.alloc(size);
            self.e.mem.set_u32(block, FAKE_VTABLE);
            block
        }

        /// An actor: a fake object whose `ActorValueOwner` (`+0xa4`) has the
        /// fake vtable too.
        fn actor(&mut self) -> u32 {
            let actor = self.object(0x200);
            self.e.mem.set_u32(actor + 0xa4, FAKE_VTABLE);
            actor
        }

        fn run(&mut self, addr: u32, words: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let result = self.e.call(addr, words);
            (result, self.e.call_log.take().unwrap())
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The `float` and `double` constants of the exe the translations read
    /// (their bit patterns, read from the exe with the Ghidra server).
    const EXE_FLOATS: [(u32, u32); 22] = [
        (0x0101_2054, 0xbf80_0000),
        (0x0101_3ea4, 0x3c23_d70a),
        (0x0101_622c, 0x3e80_0000),
        (0x0101_6248, 0x3f00_0000),
        (0x0101_62c0, 0x4000_0000),
        (0x0101_712c, 0x40a0_0000),
        (0x0101_7718, 0x4040_0000),
        (0x0101_7b78, 0x4120_0000),
        (0x0101_e2bc, 0x3dcc_cccd),
        (0x0101_e340, 0x4200_0000),
        (0x0101_e57c, 0x4180_0000),
        (0x0101_e580, 0x4170_0000),
        (0x0101_e6ec, 0x4280_0000),
        (0x0101_e704, 0x4300_0000),
        (0x0101_ff50, 0x40c9_0fdb),
        (0x0102_2958, 0x461c_4000),
        (0x0103_2adc, 0x4380_0000),
        (0x0104_e0e8, 0x41c0_0000),
        (0x0105_0b98, 0x43c0_0000),
        (0x0105_36b8, 0x4340_0000),
        (0x0109_0e90, 0x7f7f_ffff),
        (0x0109_14d0, 0x7f7f_ffff),
    ];
    const EXE_DOUBLES: [(u32, u64); 22] = [
        (0x0101_1588, 0x3fe0_0000_0000_0000),
        (0x0101_1590, 0x4000_0000_0000_0000),
        (0x0101_2060, 0x0000_0000_0000_0000),
        (0x0101_2070, 0x3ff0_0000_0000_0000),
        (0x0101_7a10, 0x40b0_0000_0000_0000),
        (0x0101_7a40, 0x4059_0000_0000_0000),
        (0x0101_db80, 0x4010_0000_0000_0000),
        (0x0101_de30, 0x3fe8_0000_0000_0000),
        (0x0101_ecb8, 0x4090_0000_0000_0000),
        (0x0102_0758, 0x4024_0000_0000_0000),
        (0x0102_0998, 0x4014_0000_0000_0000),
        (0x0102_1798, 0x3ff4_0000_0000_0000),
        (0x0102_3128, 0x3f91_df46_a000_0000),
        (0x0102_31d8, 0x4070_0000_0000_0000),
        (0x0102_f248, 0x404c_a5dc_0000_0000),
        (0x0104_f2f0, 0x4039_0000_0000_0000),
        (0x0108_cb08, 0x4122_0000_0000_0000),
        (0x0108_d6e8, 0x4002_0000_0000_0000),
        (0x0108_e058, 0x40f0_0000_0000_0000),
        (0x0109_0d68, 0x3fe5_1eb8_6000_0000),
        (0x0109_1138, 0x4110_0000_0000_0000),
        (0x0109_1b28, 0x4083_8800_0000_0000),
    ];

    /// Maps the exe data the translations read (constants, globals) and
    /// writes the constants' values; the globals start out zeroed.
    fn exe_data(e: &mut Engine) {
        for page in [
            0x011a_5000u32,
            0x011a_8000,
            0x011d_e000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        for (addr, value) in EXE_FLOATS {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u32(addr, value);
        }
        for (addr, value) in EXE_DOUBLES {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u64(addr, value);
        }
        // The empty string at `01011584` and the strings the code copies.
        e.mem.set_u8(0x0101_1584, 0);
    }

    const CALLEES: &[u32] = &[
        GET_ATTACKER,
        SET_MOVE_MODE,
        CLEAR_MOVE_MODE,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        COMPLETE_PROCEDURE,
        GET_COMBAT_STATE,
        CONTROLLER_GROUP,
        STOP_MOVING_ACTOR,
        IS_PATH_VALID,
        IS_PATHING_COMPLETE,
        GET_CURRENT_PATHFINDING_GOAL,
        SET_PATHFINDING_FLEE,
        ACTOR_PATH_TEST,
        ACTOR_OBJECT_4181E0,
        GET_WORLD_LOCATION,
        FLEE_REQUEST_CONSTRUCT,
        FLEE_REQUEST_DESTRUCT,
        REQUEST_SET_ACTOR,
        REQUEST_SET_HEADING,
        REQUEST_SET_FLAG_2B50,
        REQUEST_SET_DISTANCE,
        REQUEST_CLEAR_GOAL,
        REQUEST_SET_LOCATION,
        REQUEST_SET_AVOID,
        REQUEST_SET_FLAG_D8F0,
        PATHING_LOCATION_FROM_POINT,
        PATHING_LOCATION_DESTRUCT,
        POINT_CONSTRUCT,
        MANAGER_WORD_14,
        THREAT_AVOID_NODES,
        GROUP_AVOID_NODES,
        GROUP_FLEE_AVOID_NODES,
        BOOST_CONFIDENCE,
        STATE_REFRESH,
        STATE_REACHED,
        START_DIALOGUE,
        SAVE_BYTES,
        LOAD_BYTES,
    ];

    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureEvade>,
        t: u32,
        controller: u32,
        attacker: u32,
    }

    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x18);
        let this = Ptr::<CombatProcedureEvade>::new(t);
        let controller = rig.object(0x100);
        rig.e.set(
            this,
            CombatProcedureEvade::pCombatController,
            Ptr::new(controller),
        );
        let attacker = rig.actor();
        let position = rig.e.mem.alloc(0xc);
        rig.e.mem.set_f32(position + 4, 7.0);
        rig.answer(GET_ATTACKER, attacker);
        rig.answer(GET_COMBAT_STATE, 0x5100);
        rig.answer(CONTROLLER_GROUP, 0x5200);
        rig.slot(SLOT_POSITION, position);
        World {
            rig,
            this,
            t,
            controller,
            attacker,
        }
    }

    #[test]
    fn shutdown_stops_the_attacker() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009d_5b60, &args![w.t]);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(calls_to(&log, CLEAR_MOVE_MODE), vec![vec![w.t]]);
    }

    #[test]
    fn update_sets_up_the_flee_path() {
        // A normal flee (kind 0xc): the flee distance and the group's avoid
        // nodes.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureEvade::iLocation, 0xc);
        w.rig.setting(SETTING_COMBAT_FLEE_NORMAL_DISTANCE, 600.0);
        w.rig.answer(GROUP_FLEE_AVOID_NODES, 0x5300);
        w.rig.answer(SET_PATHFINDING_FLEE, 1);
        w.rig.answer_float(SLOT_BASE + SLOT_HEADING, 1.5);
        let (_, log) = w.rig.run(0x009d_5b90, &args![w.t]);
        let request = calls_to(&log, FLEE_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, REQUEST_SET_ACTOR),
            vec![vec![request, w.attacker]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_HEADING),
            vec![vec![request, bits(1.5)]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_DISTANCE),
            vec![vec![request, bits(600.0)]]
        );
        assert_eq!(
            calls_to(&log, GROUP_FLEE_AVOID_NODES),
            vec![vec![0x5200, w.attacker, 0]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_AVOID),
            vec![vec![request, 0x5300]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_FLAG_D8F0),
            vec![vec![request, 1]]
        );
        assert_eq!(w.rig.e.get(w.this, CombatProcedureEvade::eState), 1);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 3]]);
        assert_eq!(calls_to(&log, FLEE_REQUEST_DESTRUCT), vec![vec![request]]);

        // Kind 0xe with a goal: the combat line, 128 away from the goal,
        // the threats and the group within 384.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureEvade::iLocation, 0xe);
        let manager = 0x6100;
        w.rig.e.mem.set_u32(DIALOGUE_MANAGER, manager);
        w.rig.e.mem.set_u32(COMBAT_MANAGER, 0x6200);
        w.rig.answer(GET_CURRENT_PATHFINDING_GOAL, 1);
        w.rig.answer(PATHING_LOCATION_FROM_POINT, 0x6300);
        w.rig.answer(MANAGER_WORD_14, 0x6400);
        w.rig.answer(THREAT_AVOID_NODES, 0x6500);
        w.rig.answer(GROUP_AVOID_NODES, 0x6600);
        w.rig.slot(SLOT_TEST_21C, 1);
        w.rig.answer(ACTOR_OBJECT_4181E0, w.attacker);
        w.rig.e.mem.set_u32(w.attacker + 0x30, FAKE_VTABLE);
        w.rig.slot(SLOT_FLAG_28, 1);
        let (_, log) = w.rig.run(0x009d_5b90, &args![w.t]);
        let request = calls_to(&log, FLEE_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, START_DIALOGUE),
            vec![vec![manager, w.controller, 2, 5, 1]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_FLAG_2B50),
            vec![vec![request, 1]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_DISTANCE),
            vec![vec![request, bits(128.0)]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_LOCATION),
            vec![vec![request, 0x6300]]
        );
        let threats = calls_to(&log, THREAT_AVOID_NODES);
        assert_eq!(calls_to(&log, MANAGER_WORD_14), vec![vec![0x6200]]);
        assert_eq!(threats[0][0], 0x6400);
        assert_eq!(threats[0][2..], [w.attacker, bits(-1.0)]);
        assert_eq!(w.rig.e.mem.f32(threats[0][1] + 4), 7.0);
        assert_eq!(
            calls_to(&log, GROUP_AVOID_NODES),
            vec![vec![
                0x5200,
                w.attacker,
                threats[0][1],
                bits(384.0),
                0x6500,
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_AVOID),
            vec![vec![request, 0x6600]]
        );
        // The flee was refused: it fails.
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_CANNOT_SETUP_EVADE_PATH]]
        );
        assert_eq!(w.rig.e.get(w.this, CombatProcedureEvade::eState), 0);
        assert_eq!(calls_to(&log, FLEE_REQUEST_DESTRUCT), vec![vec![request]]);
    }

    #[test]
    fn update_watches_the_evasion() {
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureEvade::eState, 1);
        w.rig.e.set(w.this, CombatProcedureEvade::iLocation, 0xc);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        let (_, log) = w.rig.run(0x009d_5b90, &args![w.t]);
        assert_eq!(calls_to(&log, BOOST_CONFIDENCE), vec![vec![0x5100, 0]]);
        assert_eq!(calls_to(&log, STATE_REFRESH), vec![vec![0x5100]]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATH_BUILDING_FAILED]]
        );

        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureEvade::eState, 1);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        w.rig.answer(IS_PATH_VALID, 1);
        let (_, log) = w.rig.run(0x009d_5b90, &args![w.t]);
        assert!(calls_to(&log, BOOST_CONFIDENCE).is_empty());
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATHING_FAILED]]
        );

        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureEvade::eState, 1);
        w.rig.e.set(w.this, CombatProcedureEvade::iLocation, 0xa);
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        w.rig.answer(GET_WORLD_LOCATION, 0x6700);
        let (_, log) = w.rig.run(0x009d_5b90, &args![w.t]);
        assert_eq!(
            calls_to(&log, STATE_REACHED),
            vec![vec![0x5100, 0x6700, 0xa]]
        );
        assert_eq!(calls_to(&log, COMPLETE_PROCEDURE), vec![vec![w.t]]);
    }

    #[test]
    fn save_and_load_game_write_the_state_and_the_kind() {
        let mut w = world();
        let t = w.t;
        let (_, log) = w.rig.run(0x009d_5fb0, &args![t, 0x40u32]);
        let expected: Log = vec![
            (0x009d_5fb0, vec![t, 0x40]),
            (BASE_SAVE_GAME, vec![t, 0x40]),
            (SAVE_BYTES, vec![0x40, t + 0x10, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x14, 4, 0]),
        ];
        assert_eq!(log, expected);
        let (_, log) = w.rig.run(0x009d_5ff0, &args![t, 0x40u32]);
        let expected: Log = vec![
            (0x009d_5ff0, vec![t, 0x40]),
            (BASE_LOAD_GAME, vec![t, 0x40]),
            (LOAD_BYTES, vec![0x40, t + 0x10, 4]),
            (LOAD_BYTES, vec![0x40, t + 0x14, 4]),
        ];
        assert_eq!(log, expected);
    }
}

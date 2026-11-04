//! `fallout/ai/movement/playermover.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PlayerMover` is the player's `ActorMover`: instead of following a path
//! it turns the movement keys (the move-mode bits at +0x94) into a move
//! vector and hands it to the player character. Its vtable is at
//! `0109297c` (read from the exe): slot 0 the scalar deleting destructor
//! `009e9e00`, 1 `SetMoveModePreference` `009e9dc0`, 2
//! `ClearMoveModePreference` `009e9de0`, 3 `ForceMoveMode` `009ea3e0`, 5
//! `Update` `009e9e50`, 6 `UpdateTurnTimer` `009ea2f0`, 10 `SaveGame`
//! `009ea5a0`, 11 `LoadGame` `009ea610`, 12 `InitLoadGame` `009ea680`, 14
//! `Revert` `009ea6a0`; the others are `ActorMover`'s.
//!
//! The PC `ActorMover` is 0x88 bytes (`actormover.rs`), so the derived
//! fields the Xbox PDB puts at +0xA8 sit at +0x88 on the PC (checked
//! against every function here).
//!
//! x87: comparisons follow the flag tests of the disassembly (`TEST AH,0x41`
//! after `FCOMP` is a strict `>`, a NaN fails it; `TEST AH,0x44` / `JNP` is
//! `!=`, a NaN passes it); `float` stores round once, the rest is formed in
//! `f64`. The arithmetic (`FADD`/`FSUB`/`FMUL`/`FDIV`) goes through the x87 helpers
//! (`x87_add` and the others), which round to the running thread's precision: the
//! game runs `Update` on the main thread at 24-bit precision (control word `0x007F`
//! after `CreateDevice`, measured; `units::platform::direct3d_device_created`).
//! A float copied through `FLD`/`FSTP` goes through [`x87_quiet`];
//! one copied through an integer register is copied unchanged. Floats pushed
//! as arguments are passed as their bit pattern.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPoint3;

layout! {
    /// `PlayerMover` (Xbox PDB), 0xA0 bytes on the PC (the base
    /// `ActorMover` is 0x88 there).
    pub struct PlayerMover: 0xA0 {
        /// `ActorMover::pActor` (Xbox PDB), `Actor*`, from the base class.
        0x28 pActor: Ptr,
        /// `SideVec` (Xbox PDB): the requested movement (x sideways, y
        /// forward), set by `009ea570`.
        0x88 SideVec: Inline<NiPoint3>,
        /// `MoveMode` (Xbox PDB): move-mode bits.
        0x94 MoveMode: u32,
        /// `LastTurnDir` (Xbox PDB): the turn bits (`0x30`) of the last
        /// forced move mode.
        0x98 LastTurnDir: u32,
        /// `fMinTurnTime` (Xbox PDB): time left of the minimum turn.
        0x9C fMinTurnTime: f32,
    }
}

/// `PlayerMover`'s vtable.
const VTABLE: u32 = 0x0109_297c;
/// The player character pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A global `NiPoint3` the code copies (three plain words) to clear a point.
const UNSET_POINT: u32 = 0x011f_426c;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `pi / 4` (`double`, `0.7853981...`).
const QUARTER_PI: u32 = 0x0103_b100;
/// `pi / 2` (`double`, `1.5707963...`).
const HALF_PI: u32 = 0x0103_0f38;
/// `32.0` (`double`).
const THIRTY_TWO: u32 = 0x0102_f070;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The `Setting` `fPCTurnAnimMinTime` (its name `01050620` is passed by its
/// static initializer `00f5b790`).
const PC_TURN_ANIM_MIN_TIME: u32 = 0x011c_df28;

/// `Setting`'s value pointer (`00403e20`: `this + 4`, or a zeroed global
/// when `this` is null).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `PlayerCharacter::GetAnimation` (Xbox PDB), one argument word.
const GET_ANIMATION: u32 = 0x0095_0a60;
/// `004301b0`: one argument word, returns a 16-bit value (an animation
/// group of the animation data it is called on).
const ANIMATION_GROUP_AT: u32 = 0x0043_01b0;
/// `ActorMover::~ActorMover` body (`009daef0`, `actormover.cpp`).
const ACTOR_MOVER_DESTRUCT: u32 = 0x009d_aef0;
/// `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `ActorMover::SaveGame` (Xbox PDB).
const ACTOR_MOVER_SAVE_GAME: u32 = 0x009d_f100;
/// `ActorMover::LoadGame` (Xbox PDB).
const ACTOR_MOVER_LOAD_GAME: u32 = 0x009d_f3d0;
/// `ActorMover::InitLoadGame` (Xbox PDB).
const ACTOR_MOVER_INIT_LOAD_GAME: u32 = 0x009d_f8c0;
/// `ActorMover::Revert` (Xbox PDB).
const ACTOR_MOVER_REVERT: u32 = 0x009d_fb80;
/// Save-game buffer write (`00865e50`, `bgssavegamebuffer.cpp`): data,
/// size, a third word.
const SAVE_WRITE: u32 = 0x0086_5e50;
/// Load-game buffer read (`00864980`, `bgsloadgamebuffer.cpp`): data, size.
const LOAD_READ: u32 = 0x0086_4980;

/// Copies three words (an `NiPoint3` copied through integer registers).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 009e9dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::SetMoveModePreference` (Xbox PDB): sets the bits `mode`
/// in `MoveMode` (`009ea360` with `true`).
pub fn player_mover_set_move_mode_preference(e: &mut Engine, this: Ptr<PlayerMover>, mode: u32) {
    fn_009ea360(e, this, mode, true);
}

// Translated from 009e9de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::ClearMoveMode` (Xbox PDB name; vtable slot 2,
/// `ClearMoveModePreference`): clears the bits `mode` (`009ea3b0`).
pub fn player_mover_clear_move_mode(e: &mut Engine, this: Ptr<PlayerMover>, mode: u32) {
    fn_009ea3b0(e, this, mode);
}

// Translated from 009e9e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::`scalar deleting destructor'` (Xbox PDB): destroys the
/// mover and frees it when bit 0 of `flags` is set; returns `this`.
pub fn player_mover_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PlayerMover>,
    flags: u32,
) -> Ptr<PlayerMover> {
    fn_009e9e30(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009e9e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PlayerMover` destructor body: puts back `PlayerMover`'s vtable and
/// runs `ActorMover`'s (`009daef0`).
pub fn fn_009e9e30(e: &mut Engine, this: Ptr<PlayerMover>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(ACTOR_MOVER_DESTRUCT, &args![this]);
}

// Translated from 009e9e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::Update` (Xbox PDB): builds the player's move vector for
/// this frame and hands it to the player character.
///
/// The vector starts cleared. Unless `00886b30` (on the player, with the
/// vector) fills it, it comes from the animation: the first-person
/// animation (`GetAnimation(0)`), or the third-person one (`(1)`) when
/// `004eaf60` (the player's byte +0x64A) is clear and the two differ in
/// `004301b0(1)` or there is no first-person one; `00494390` writes the
/// animation's movement into it. A character controller (`MobileObject::
/// GetCharController` + 0x410) for which `008b7320` is true clears it
/// again. With any of the move bits `0xF` set (and the player's slot 0x2E8
/// and `0050d4a0` both false, bit 0x40 clear, both lengths positive) the
/// vector is rescaled to `SideVec`'s direction. With a sideways bit (`0x3`)
/// and a forward/back bit (`0xC`) (slot 0x2E8 false, the process's slot
/// 0x3E4 not 13 or 14, y not 0) it is turned by an angle
/// `(SideVec.x * seconds * scale / y) * pi/4`. Then, when `0050d4a0` (flag
/// 0x10 of the player's dword +8) is set, the vector goes through
/// `00436aa0`/`00439e90` to the player's slot 0x2A8; otherwise it goes to
/// the player's slot 0x250 with `seconds`, and if the player stands more
/// than 32 units below the land height of its cell (`008d6f30`, the word
/// at +0x40), it is put 32 above it (slot 0x2A8).
pub fn player_mover_update(e: &mut Engine, this: Ptr<PlayerMover>, seconds: f32) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    e.with_stack(0x28, |e, frame| {
        let vector: Ptr<NiPoint3> = Ptr::new(frame.addr());
        let transformed = frame.addr() + 0x0c;
        let position = frame.addr() + 0x18;
        let land_height = frame.addr() + 0x24;
        copy_point(e, UNSET_POINT, vector.addr());

        let mut animation = e.call(GET_ANIMATION, &args![player, 0u32]).u32();
        if !e.call(0x004e_af60, &args![player]).bool() {
            let third_person = e.call(GET_ANIMATION, &args![player, 1u32]).u32();
            let keep_first = animation != 0 && {
                let third_group = e.call(ANIMATION_GROUP_AT, &args![third_person, 1u32]).u16();
                let first_group = e.call(ANIMATION_GROUP_AT, &args![animation, 1u32]).u16();
                third_group != first_group
            };
            if !keep_first {
                animation = third_person;
            }
        }

        if !e.call(0x0088_6b30, &args![player, vector]).bool() {
            if animation != 0 {
                if e.call(0x0050_d4a0, &args![player]).bool() {
                    e.call(0x0049_4390, &args![animation, vector, player, 1u32, 0u32]);
                } else {
                    e.call(0x0049_4390, &args![animation, vector, player, 0u32, 1u32]);
                }
            }
            // `MobileObject::GetCharController` (Xbox PDB).
            let controller = e.call(0x0093_06d0, &args![player]).u32();
            let listener = if controller == 0 {
                0
            } else {
                controller.wrapping_add(0x410)
            };
            if listener != 0 && e.call(0x008b_7320, &args![listener]).bool() {
                copy_point(e, UNSET_POINT, vector.addr());
            }

            let zero: f64 = e.global(ZERO);
            if e.get(this, PlayerMover::MoveMode) & 0xf != 0
                && !e.vcall(player, 0x2e8, &[]).bool()
                && !e.call(0x0050_d4a0, &args![player]).bool()
            {
                // `Actor::GetAnimAction` (Xbox PDB); its result is unused.
                e.call(0x008a_7570, &args![player]);
                let side = this.at(PlayerMover::SideVec);
                let side_length = e.call(0x0045_7990, &args![side]).f32();
                let vector_length = e.call(0x0045_7990, &args![vector]).f32();
                if side_length as f64 > zero
                    && vector_length as f64 > zero
                    && e.get(this, PlayerMover::MoveMode) & 0x40 == 0
                {
                    let side_x = e.get(side, NiPoint3::x);
                    // `FLD`, `FDIV dword`, `FSTP dword` (`009ea035`..`009ea04d`).
                    let unit_x =
                        x87_div(x87_load32(side_x.to_bits()), x87_m32(side_length.to_bits()))
                            as f32;
                    let side_y = e.get(side, NiPoint3::y);
                    let unit_y =
                        x87_div(x87_load32(side_y.to_bits()), x87_m32(side_length.to_bits()))
                            as f32;
                    e.set(
                        vector,
                        NiPoint3::x,
                        x87_mul(
                            x87_load32(unit_x.to_bits()),
                            x87_m32(vector_length.to_bits()),
                        ) as f32,
                    );
                    e.set(
                        vector,
                        NiPoint3::y,
                        x87_mul(
                            x87_load32(unit_y.to_bits()),
                            x87_m32(vector_length.to_bits()),
                        ) as f32,
                    );
                }
            }

            let mode = e.get(this, PlayerMover::MoveMode);
            if mode & 3 != 0 && mode & 0xc != 0 && !e.vcall(player, 0x2e8, &[]).bool() {
                // `008d8520`: the dword at +0x68 of the player.
                let process = e.call(0x008d_8520, &args![player]).u32();
                let kind = e.vcall(process, 0x3e4, &[]).i32();
                let forward = e.get(vector, NiPoint3::y);
                if !(13..=14).contains(&kind) && forward as f64 != zero {
                    let side_x = e.get(this.at(PlayerMover::SideVec), NiPoint3::x);
                    // `FMUL dword` (`009ea0f5`), kept as a double (`FSTP qword`).
                    let distance =
                        x87_mul(x87_load32(side_x.to_bits()), x87_m32(seconds.to_bits()));
                    // `TESObjectREFR::GetScale` (Xbox PDB).
                    let scale = e.call(0x0056_7400, &args![player]).f64();
                    // `FMUL qword` (`009ea106`) on the scale `GetScale` left in ST0.
                    let scaled = x87_mul(scale, distance) as f32;
                    let quarter_pi: f64 = e.global(QUARTER_PI);
                    // `FDIV dword`, `FMUL qword` (`009ea10f`, `009ea112`): one chain.
                    let angle = x87_mul(
                        x87_div(x87_load32(scaled.to_bits()), x87_m32(forward.to_bits())),
                        quarter_pi,
                    ) as f32;
                    let half_pi: f64 = e.global(HALF_PI);
                    let mode = e.get(this, PlayerMover::MoveMode);
                    if mode & 4 != 0 {
                        let sideways = sideways_part(e, half_pi, angle, forward);
                        let x = e.get(vector, NiPoint3::x);
                        // `FCHS`, `FADD dword` (`009ea159`).
                        e.set(
                            vector,
                            NiPoint3::x,
                            x87_add(-sideways, x87_m32(x.to_bits())) as f32,
                        );
                    } else if mode & 8 != 0 {
                        let sideways = sideways_part(e, half_pi, angle, forward);
                        let x = e.get(vector, NiPoint3::x);
                        // `FADD dword` (`009ea1a1`).
                        e.set(
                            vector,
                            NiPoint3::x,
                            x87_add(sideways, x87_m32(x.to_bits())) as f32,
                        );
                    }
                    let cosine = e.call(0x004e_4470, &args![angle.to_bits()]).f64();
                    let forward = e.get(vector, NiPoint3::y);
                    // `FMUL dword` (`009ea1b6`).
                    e.set(
                        vector,
                        NiPoint3::y,
                        x87_mul(cosine, x87_m32(forward.to_bits())) as f32,
                    );
                }
            }
        }

        if e.call(0x0050_d4a0, &args![player]).bool() {
            let source = e.call(0x0043_6aa0, &args![player]).u32();
            e.call(0x0043_9e90, &args![source, transformed, vector]);
            e.vcall(player, 0x2a8, &args![transformed]);
        } else {
            let extra = e.call(0x0088_46e0, &args![player]).u32();
            e.vcall(
                player,
                0x250,
                &args![x87_quiet_bits(seconds.to_bits()), vector, extra],
            );
            let cell = e.call(0x008d_6f30, &args![player]).u32();
            if cell != 0 && !e.call(0x0042_5fd0, &args![cell]).bool() {
                e.mem.set_f32(land_height, 0.0);
                let current = e.vcall(player, 0x1f4, &[]).u32();
                copy_point(e, current, position);
                // `TESObjectCELL::GetLand`, `TESObjectLAND::GetLandHeight`
                // (Xbox PDB).
                let land = e.call(0x0054_6fb0, &args![cell]).u32();
                if e.call(0x0053_f180, &args![land, position, land_height])
                    .bool()
                {
                    let height = e.mem.f32(land_height);
                    let z = e.mem.f32(position + 8);
                    let thirty_two: f64 = e.global(THIRTY_TWO);
                    // `FSUB dword`, `FCOMP qword` (`009ea2ac`).
                    if x87_sub(x87_load32(height.to_bits()), x87_m32(z.to_bits())) > thirty_two {
                        e.mem
                            // `FADD qword` (`009ea2bf`).
                            .set_f32(
                                position + 8,
                                x87_add(x87_load32(height.to_bits()), thirty_two) as f32,
                            );
                        e.vcall(player, 0x2a8, &args![position]);
                    }
                }
            }
        }
    });
}

/// `Update`'s sideways term: `00408840(004e4470(pi/2 - angle) * forward)`,
/// each `float` argument stored before it is pushed.
fn sideways_part(e: &mut Engine, half_pi: f64, angle: f32, forward: f32) -> f64 {
    // `FSUBR qword` (`009ea12c`/`009ea174`), `FMUL dword` (`009ea144`/`009ea18c`).
    let complement = x87_sub(half_pi, x87_load32(angle.to_bits())) as f32;
    let sine = e.call(0x004e_4470, &args![complement.to_bits()]).f64();
    let product = x87_mul(sine, x87_m32(forward.to_bits())) as f32;
    e.call(0x0040_8840, &args![product.to_bits()]).f64()
}

// Translated from 009ea2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::UpdateTurnTimer` (Xbox PDB): while `fMinTurnTime` is
/// positive, counts it down by `seconds`, and while the actor's slot 0x214
/// returns 0 keeps the last turn bits set in `MoveMode`.
pub fn player_mover_update_turn_timer(e: &mut Engine, this: Ptr<PlayerMover>, seconds: f32) {
    let zero: f64 = e.global(ZERO);
    let time = e.get(this, PlayerMover::fMinTurnTime);
    // `TEST AH,0x41`: a NaN or a time of 0 or less does nothing.
    if time as f64 <= zero || time.is_nan() {
        return;
    }
    e.set(
        this,
        PlayerMover::fMinTurnTime,
        // `FSUB dword` (`009ea316`).
        x87_sub(x87_load32(time.to_bits()), x87_m32(seconds.to_bits())) as f32,
    );
    let actor = e.get(this, PlayerMover::pActor);
    if e.vcall(actor.addr(), 0x214, &[]).u32() == 0 {
        let mode = e.get(this, PlayerMover::MoveMode);
        let turn = e.get(this, PlayerMover::LastTurnDir);
        e.set(this, PlayerMover::MoveMode, mode | turn);
    }
}

// Translated from 009ea360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears the bits `mode` in `MoveMode`.
pub fn fn_009ea360(e: &mut Engine, this: Ptr<PlayerMover>, mode: u32, set: bool) {
    let current = e.get(this, PlayerMover::MoveMode);
    let value = if set { current | mode } else { !mode & current };
    e.set(this, PlayerMover::MoveMode, value);
}

// Translated from 009ea3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits `mode` in `MoveMode`.
pub fn fn_009ea3b0(e: &mut Engine, this: Ptr<PlayerMover>, mode: u32) {
    let current = e.get(this, PlayerMover::MoveMode);
    e.set(this, PlayerMover::MoveMode, !mode & current);
}

/// `ForceMoveMode`'s blend-out of one of the player's animations: blends
/// out sequence 4 (`Animation::BlendOut(4, 0)`) unless it is already the
/// group `group`.
fn blend_out_unless_playing(e: &mut Engine, player: u32, which: u32, group: u32) {
    let animation = e.call(GET_ANIMATION, &args![player, which]).u32();
    let sequence = e.call(0x0049_1040, &args![animation, 4u32]).u32();
    // `Actor::GetAnimGroup` (Xbox PDB).
    let wanted = e
        .call(0x0089_7910, &args![player, group, 0u32, 0u32, animation])
        .u16();
    if sequence != 0 {
        // `Animation::ZeroGlobalTransform` (Xbox PDB name of `0048f7f0`).
        let data = e.call(0x0048_f7f0, &args![sequence]).u32();
        let playing = e.call(0x0047_d3b0, &args![data]).u16();
        if wanted == playing {
            return;
        }
    }
    // `Animation::BlendOut` (Xbox PDB).
    e.call(0x0049_94f0, &args![animation, 4u32, 0u32]);
}

// Translated from 009ea3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::ForceMoveMode` (Xbox PDB): replaces `MoveMode` by `mode`.
/// With a turn bit (`0x30`) it starts the minimum turn time
/// (`fPCTurnAnimMinTime`) and remembers the bits. When bit 0x400 changes
/// and the first-person animation's group (`004301b0(4)`) is an aim action
/// (`TESAnimGroup::IsAimAction`), it blends out both animations' sequence
/// 4 unless it already plays the aim group (0x11, or 0x14 with iron sights
/// `Actor::GetIronSights`).
pub fn player_mover_force_move_mode(e: &mut Engine, this: Ptr<PlayerMover>, mode: u32) {
    let changed = e.get(this, PlayerMover::MoveMode) & 0x400 != mode & 0x400;
    if mode & 0x30 != 0 {
        let value = e.call(SETTING_VALUE, &args![PC_TURN_ANIM_MIN_TIME]).u32();
        let time = x87_quiet(e.mem.f32(value));
        e.set(this, PlayerMover::fMinTurnTime, time);
        e.set(this, PlayerMover::LastTurnDir, mode & 0x30);
    }
    e.set(this, PlayerMover::MoveMode, mode);
    if !changed {
        return;
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let animation = e.call(GET_ANIMATION, &args![player, 0u32]).u32();
    let group = e.call(ANIMATION_GROUP_AT, &args![animation, 4u32]).u16();
    if !e.call(0x005f_2630, &args![group as u32]).bool() {
        return;
    }
    let iron_sights = e.call(0x008b_bc10, &args![player]).bool();
    let aim_group = if iron_sights { 0x14 } else { 0x11 };
    blend_out_unless_playing(e, player, 0, aim_group);
    blend_out_unless_playing(e, player, 1, aim_group);
}

// Translated from 009ea570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the point at `side` into `SideVec` (three plain words).
pub fn fn_009ea570(e: &mut Engine, this: Ptr<PlayerMover>, side: Ptr<NiPoint3>) {
    copy_point(e, side.addr(), this.at(PlayerMover::SideVec).addr());
}

// Translated from 009ea5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::SaveGame` (Xbox PDB): `ActorMover::SaveGame`, then
/// `SideVec`, `MoveMode`, `LastTurnDir` and `fMinTurnTime` into the buffer.
pub fn player_mover_save_game(e: &mut Engine, this: Ptr<PlayerMover>, buffer: u32) {
    e.call(ACTOR_MOVER_SAVE_GAME, &args![this, buffer]);
    for (offset, size) in SAVED_FIELDS {
        e.call(SAVE_WRITE, &args![buffer, this.addr() + offset, size, 0u32]);
    }
}

/// The fields `SaveGame` and `LoadGame` carry: offset and size.
const SAVED_FIELDS: [(u32, u32); 4] = [(0x88, 0xc), (0x94, 4), (0x98, 4), (0x9c, 4)];

// Translated from 009ea610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::LoadGame` (Xbox PDB): `ActorMover::LoadGame`, then the
/// fields `SaveGame` writes, in its order.
pub fn player_mover_load_game(e: &mut Engine, this: Ptr<PlayerMover>, buffer: u32) {
    e.call(ACTOR_MOVER_LOAD_GAME, &args![this, buffer]);
    for (offset, size) in SAVED_FIELDS {
        e.call(LOAD_READ, &args![buffer, this.addr() + offset, size]);
    }
}

// Translated from 009ea680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::InitLoadGame` (Xbox PDB): `ActorMover::InitLoadGame`.
pub fn player_mover_init_load_game(e: &mut Engine, this: Ptr<PlayerMover>, buffer: u32) {
    e.call(ACTOR_MOVER_INIT_LOAD_GAME, &args![this, buffer]);
}

// Translated from 009ea6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerMover::Revert` (Xbox PDB): `ActorMover::Revert`, then clears
/// `SideVec`, `MoveMode` and `LastTurnDir` and sets `fMinTurnTime` to -1.
pub fn player_mover_revert(e: &mut Engine, this: Ptr<PlayerMover>, buffer: u32) {
    e.call(ACTOR_MOVER_REVERT, &args![this, buffer]);
    copy_point(e, UNSET_POINT, this.at(PlayerMover::SideVec).addr());
    e.set(this, PlayerMover::MoveMode, 0);
    let minus_one = x87_quiet(e.global::<f32>(MINUS_ONE));
    e.set(this, PlayerMover::fMinTurnTime, minus_one);
    e.set(this, PlayerMover::LastTurnDir, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009e9dc0,
            player_mover_set_move_mode_preference(Ptr<PlayerMover>, u32)
        ),
        entry!(
            0x009e9de0,
            player_mover_clear_move_mode(Ptr<PlayerMover>, u32)
        ),
        entry!(
            0x009e9e00,
            player_mover_scalar_deleting_destructor(Ptr<PlayerMover>, u32) -> Ptr<PlayerMover>
        ),
        entry!(0x009e9e30, fn_009e9e30(Ptr<PlayerMover>)),
        entry!(0x009e9e50, player_mover_update(Ptr<PlayerMover>, f32)),
        entry!(
            0x009ea2f0,
            player_mover_update_turn_timer(Ptr<PlayerMover>, f32)
        ),
        entry!(0x009ea360, fn_009ea360(Ptr<PlayerMover>, u32, bool)),
        entry!(0x009ea3b0, fn_009ea3b0(Ptr<PlayerMover>, u32)),
        entry!(
            0x009ea3e0,
            player_mover_force_move_mode(Ptr<PlayerMover>, u32)
        ),
        entry!(0x009ea570, fn_009ea570(Ptr<PlayerMover>, Ptr<NiPoint3>)),
        entry!(0x009ea5a0, player_mover_save_game(Ptr<PlayerMover>, u32)),
        entry!(0x009ea610, player_mover_load_game(Ptr<PlayerMover>, u32)),
        entry!(
            0x009ea680,
            player_mover_init_load_game(Ptr<PlayerMover>, u32)
        ),
        entry!(0x009ea6a0, player_mover_revert(Ptr<PlayerMover>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Points = Rc<RefCell<Vec<[f32; 3]>>>;

    const PLAYER: u32 = 0x0500_0000;
    const PLAYER_TABLE: u32 = 0x0300_0000;
    /// Player slots are doubled at `SLOT_BASE + slot`.
    const SLOT_BASE: u32 = 0x0700_0000;
    const PROCESS: u32 = 0x0501_0000;
    const PROCESS_TABLE: u32 = 0x0301_0000;
    const SCRATCH: u32 = 0x0600_0000;
    const SETTING_FLOAT: u32 = SCRATCH + 0x100;

    const CALLEES: &[u32] = &[
        SETTING_VALUE,
        GET_ANIMATION,
        ANIMATION_GROUP_AT,
        ACTOR_MOVER_DESTRUCT,
        OPERATOR_DELETE,
        ACTOR_MOVER_SAVE_GAME,
        ACTOR_MOVER_LOAD_GAME,
        ACTOR_MOVER_INIT_LOAD_GAME,
        ACTOR_MOVER_REVERT,
        SAVE_WRITE,
        LOAD_READ,
        0x004e_af60,
        0x0088_6b30,
        0x0050_d4a0,
        0x0049_4390,
        0x0093_06d0,
        0x008b_7320,
        0x008a_7570,
        0x0045_7990,
        0x008d_8520,
        0x0056_7400,
        0x004e_4470,
        0x0040_8840,
        0x0043_6aa0,
        0x0043_9e90,
        0x0088_46e0,
        0x008d_6f30,
        0x0042_5fd0,
        0x0054_6fb0,
        0x0053_f180,
        0x005f_2630,
        0x008b_bc10,
        0x0049_1040,
        0x0089_7910,
        0x0048_f7f0,
        0x0047_d3b0,
        0x0049_94f0,
    ];

    fn engine() -> (Engine, Ptr<PlayerMover>) {
        let mut e = Engine::new();
        for &a in CALLEES {
            e.register(a, |_, _| Ret::default());
        }
        for page in [
            0x011d_e000,
            0x011f_4000,
            0x0101_2000,
            0x0103_b000,
            0x0103_0000,
            0x0102_f000,
            0x011c_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(PLAYER_CHARACTER, PLAYER);
        e.set_global(ZERO, 0.0f64);
        e.set_global(QUARTER_PI, std::f64::consts::FRAC_PI_4);
        e.set_global(HALF_PI, std::f64::consts::FRAC_PI_2);
        e.set_global(THIRTY_TWO, 32.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        e.map(PLAYER, 0x1000);
        e.map(PROCESS, 0x1000);
        e.map(SCRATCH, 0x1000);
        let slots: Vec<u32> = (0..0xc0).map(|i| SLOT_BASE + 4 * i).collect();
        e.put_vtable(PLAYER_TABLE, &slots);
        e.mem.set_u32(PLAYER, PLAYER_TABLE);
        for i in 0..0xc0 {
            e.register(SLOT_BASE + 4 * i, |_, _| Ret::default());
        }
        let mover = e.new_object::<PlayerMover>();
        e.set(mover, PlayerMover::pActor, Ptr::new(PLAYER));
        (e, mover)
    }

    fn record(e: &mut Engine, addr: u32, calls: &Calls) {
        let calls = calls.clone();
        e.register_double(addr, move |_, a| {
            calls.borrow_mut().push((addr, a.to_vec()));
            Ret::default()
        });
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn set_move_mode_preference_sets_bits() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 0x10);
        e.call(0x009e_9dc0, &args![mover, 0x3u32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x13);
    }

    #[test]
    fn clear_move_mode_clears_bits() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 0x13);
        e.call(0x009e_9de0, &args![mover, 0x3u32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x10);
    }

    #[test]
    fn set_or_clear_bits_follows_the_flag() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 0x10);
        e.call(0x009e_a360, &args![mover, 0x5u32, 0x100u32]);
        // Only the low byte of the flag word is read.
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x10);
        e.call(0x009e_a360, &args![mover, 0x5u32, 1u32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x15);
    }

    #[test]
    fn clear_bits_masks_the_mode() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 0xff);
        e.call(0x009e_a3b0, &args![mover, 0x0fu32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0xf0);
    }

    #[test]
    fn destructor_restores_the_vtable_and_runs_the_base() {
        let (mut e, mover) = engine();
        e.call_log = Some(vec![]);
        e.call(0x009e_9e30, &args![mover]);
        assert_eq!(e.mem.u32(mover.addr()), VTABLE);
        assert_eq!(logged(&e, ACTOR_MOVER_DESTRUCT), vec![vec![mover.addr()]]);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let (mut e, mover) = engine();
        e.call_log = Some(vec![]);
        let r = e.call(0x009e_9e00, &args![mover, 2u32]).u32();
        assert_eq!(r, mover.addr());
        assert!(logged(&e, OPERATOR_DELETE).is_empty());
        e.call(0x009e_9e00, &args![mover, 1u32]);
        assert_eq!(logged(&e, OPERATOR_DELETE), vec![vec![mover.addr()]]);
        assert_eq!(logged(&e, ACTOR_MOVER_DESTRUCT).len(), 2);
    }

    #[test]
    fn update_turns_the_vector_and_lifts_the_player_above_the_land() {
        let (mut e, mover) = engine();
        let side = mover.at(PlayerMover::SideVec);
        e.set(side, NiPoint3::x, 3.0);
        e.set(side, NiPoint3::y, 4.0);
        e.set(mover, PlayerMover::MoveMode, 0x5);
        e.register(GET_ANIMATION, |_, a| (0xa0 + a[1]).into_ret());
        // The groups differ, so the first-person animation is kept.
        e.register(ANIMATION_GROUP_AT, |_, a| (a[0] as u16).into_ret());
        let side_addr = side.addr();
        e.register_double(0x0045_7990, move |_, a| {
            (if a[0] == side_addr { 5.0f32 } else { 10.0f32 }).into_ret()
        });
        e.register(0x008d_8520, |_, _| PROCESS.into_ret());
        e.put_vtable(PROCESS_TABLE, &[0x0700_1000; 0x100]);
        e.mem.set_u32(PROCESS, PROCESS_TABLE);
        e.register(0x0700_1000, |_, _| 3u32.into_ret());
        e.register(0x0056_7400, |_, _| 2.0f32.into_ret());
        e.register(0x004e_4470, |_, a| {
            (f32::from_bits(a[0]) as f64 * 2.0).into_ret()
        });
        e.register(0x0040_8840, |_, a| {
            (f32::from_bits(a[0]) as f64 + 1.0).into_ret()
        });
        e.register(0x0088_46e0, |_, _| 0x77u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x0505_0000u32.into_ret());
        e.register(0x0054_6fb0, |_, _| 0x0506_0000u32.into_ret());
        e.mem.set_f32(SCRATCH, 1.0);
        e.mem.set_f32(SCRATCH + 4, 2.0);
        e.mem.set_f32(SCRATCH + 8, -50.0);
        e.register(SLOT_BASE + 0x1f4, |_, _| SCRATCH.into_ret());
        e.register(0x0053_f180, |e, a| {
            e.mem.set_f32(a[2], 0.0);
            true.into_ret()
        });
        let moved: Points = Rc::default();
        let m = moved.clone();
        e.register_double(SLOT_BASE + 0x250, move |e, a| {
            assert_eq!(a[1], 0.5f32.to_bits());
            assert_eq!(a[3], 0x77);
            m.borrow_mut().push(point(e, a[2]));
            Ret::default()
        });
        let placed: Points = Rc::default();
        let p = placed.clone();
        e.register_double(SLOT_BASE + 0x2a8, move |e, a| {
            p.borrow_mut().push(point(e, a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009e_9e50, &args![mover, 0.5f32]);

        let calls = logged(&e, 0x0049_4390);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], 0xa0);
        assert_eq!(&calls[0][2..], &[PLAYER, 0, 1]);
        // Rescaled: (3/5 * 10, 4/5 * 10).
        let x = ((3.0f64 / 5.0) as f32 as f64 * 10.0) as f32;
        let y = ((4.0f64 / 5.0) as f32 as f64 * 10.0) as f32;
        let scaled = (2.0f64 * (3.0f64 * 0.5)) as f32;
        let angle = ((scaled as f64 / y as f64) * std::f64::consts::FRAC_PI_4) as f32;
        let complement = (std::f64::consts::FRAC_PI_2 - angle as f64) as f32;
        let product = ((complement as f64 * 2.0) * y as f64) as f32;
        let x = (-(product as f64 + 1.0) + x as f64) as f32;
        let y = ((angle as f64 * 2.0) * y as f64) as f32;
        assert_eq!(*moved.borrow(), vec![[x, y, 0.0]]);
        assert_eq!(*placed.borrow(), vec![[1.0, 2.0, 32.0]]);
    }

    /// At the main thread's 24-bit precision (0x007F, as the game runs `Update`
    /// once the renderer's device exists) every `FADD`/`FSUB`/`FMUL`/`FDIV`
    /// result is rounded to 24 bits, also inside a chain: the turn angle's
    /// quotient (`009ea10f`) before its `FMUL` by pi/4 (`009ea112`), and the
    /// distance kept as a double (`009ea0fe`).
    #[test]
    fn update_rounds_each_operation_at_24_bit_precision() {
        let (mut e, mover) = engine();
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        let side = mover.at(PlayerMover::SideVec);
        e.set(side, NiPoint3::x, 0.3);
        e.set(side, NiPoint3::y, 4.0);
        e.set(mover, PlayerMover::MoveMode, 0x5);
        e.register(GET_ANIMATION, |_, a| (0xa0 + a[1]).into_ret());
        e.register(ANIMATION_GROUP_AT, |_, a| (a[0] as u16).into_ret());
        let side_addr = side.addr();
        e.register_double(0x0045_7990, move |_, a| {
            (if a[0] == side_addr { 5.0f32 } else { 7.0f32 }).into_ret()
        });
        e.register(0x008d_8520, |_, _| PROCESS.into_ret());
        e.put_vtable(PROCESS_TABLE, &[0x0700_1000; 0x100]);
        e.mem.set_u32(PROCESS, PROCESS_TABLE);
        e.register(0x0700_1000, |_, _| 3u32.into_ret());
        e.register(0x0056_7400, |_, _| 1.1f64.into_ret());
        e.register(0x004e_4470, |_, a| (f32::from_bits(a[0]) as f64).into_ret());
        e.register(0x0040_8840, |_, a| (f32::from_bits(a[0]) as f64).into_ret());
        e.register(0x0088_46e0, |_, _| 0u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        let moved: Points = Rc::default();
        let m = moved.clone();
        e.register_double(SLOT_BASE + 0x250, move |e, a| {
            m.borrow_mut().push(point(e, a[2]));
            Ret::default()
        });
        e.call(0x009e_9e50, &args![mover, 0.0166f32]);
        // Float operands: the f64 result rounded to a float is the 24-bit
        // result. A double operand (the scale, pi/2, pi/4) goes through the
        // instruction's helper, still at 24 bits.
        let single = |v: f64| v as f32 as f64;
        let x = single(single(0.3f32 as f64 / 5.0) * 7.0) as f32;
        let y = single(single(4.0 / 5.0) * 7.0) as f32;
        let distance = single(0.3f32 as f64 * 0.0166f32 as f64);
        let scaled = crate::x87::x87_mul(1.1, distance) as f32;
        let quotient = single(scaled as f64 / y as f64);
        let angle = crate::x87::x87_mul(quotient, std::f64::consts::FRAC_PI_4) as f32;
        let complement = crate::x87::x87_sub(std::f64::consts::FRAC_PI_2, angle as f64) as f32;
        let product = single(complement as f64 * y as f64) as f32;
        let x = single(-(product as f64) + x as f64) as f32;
        let y = single(angle as f64 * y as f64) as f32;
        assert_eq!(*moved.borrow(), vec![[x, y, 0.0]]);
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
    }

    #[test]
    fn update_hands_a_filled_vector_through_the_transform() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 0xf);
        e.register(GET_ANIMATION, |_, _| 0xa0u32.into_ret());
        e.register(0x0088_6b30, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            true.into_ret()
        });
        e.register(0x0050_d4a0, |_, _| true.into_ret());
        e.register(0x0043_6aa0, |_, _| 0x99u32.into_ret());
        let calls: Calls = Rc::default();
        record(&mut e, 0x0043_9e90, &calls);
        record(&mut e, SLOT_BASE + 0x2a8, &calls);
        e.call_log = Some(vec![]);
        e.call(0x009e_9e50, &args![mover, 0.5f32]);
        assert!(logged(&e, 0x0049_4390).is_empty());
        assert!(logged(&e, SLOT_BASE + 0x250).is_empty());
        let calls = calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, 0x0043_9e90);
        assert_eq!(calls[0].1[0], 0x99);
        assert_eq!(calls[1], (SLOT_BASE + 0x2a8, vec![PLAYER, calls[0].1[1]]));
    }

    #[test]
    fn update_turn_timer_counts_down_and_keeps_the_turn() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::fMinTurnTime, 0.25);
        e.set(mover, PlayerMover::MoveMode, 0x1);
        e.set(mover, PlayerMover::LastTurnDir, 0x20);
        e.call(0x009e_a2f0, &args![mover, 0.125f32]);
        assert_eq!(e.get(mover, PlayerMover::fMinTurnTime), 0.125);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x21);

        e.register(SLOT_BASE + 0x214, |_, _| 1u32.into_ret());
        e.set(mover, PlayerMover::MoveMode, 0x1);
        e.call(0x009e_a2f0, &args![mover, 0.125f32]);
        assert_eq!(e.get(mover, PlayerMover::fMinTurnTime), 0.0);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x1);

        e.set(mover, PlayerMover::fMinTurnTime, f32::NAN);
        e.call_log = Some(vec![]);
        e.call(0x009e_a2f0, &args![mover, 0.125f32]);
        assert!(logged(&e, SLOT_BASE + 0x214).is_empty());
    }

    #[test]
    fn force_move_mode_starts_the_turn_and_blends_out_the_aim() {
        let (mut e, mover) = engine();
        e.mem.set_u32(SETTING_FLOAT, 0x7f80_0001);
        e.register(SETTING_VALUE, |_, a| {
            assert_eq!(a[0], PC_TURN_ANIM_MIN_TIME);
            SETTING_FLOAT.into_ret()
        });
        e.call(0x009e_a3e0, &args![mover, 0x31u32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x31);
        assert_eq!(e.get(mover, PlayerMover::LastTurnDir), 0x30);
        assert_eq!(
            e.get(mover, PlayerMover::fMinTurnTime).to_bits(),
            0x7fc0_0001
        );

        e.register(GET_ANIMATION, |_, a| (0xa0 + a[1]).into_ret());
        e.register(0x005f_2630, |_, _| true.into_ret());
        e.register(0x008b_bc10, |_, _| true.into_ret());
        // First-person sequence plays the aim group; third-person has none.
        e.register(0x0049_1040, |_, a| {
            (if a[0] == 0xa0 { 0x55u32 } else { 0 }).into_ret()
        });
        e.register(0x0089_7910, |_, _| 0x14u16.into_ret());
        e.register(0x0048_f7f0, |_, _| 0x66u32.into_ret());
        e.register(0x0047_d3b0, |_, _| 0x14u16.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009e_a3e0, &args![mover, 0x401u32]);
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0x401);
        assert_eq!(e.get(mover, PlayerMover::LastTurnDir), 0x30);
        assert_eq!(logged(&e, ANIMATION_GROUP_AT), vec![vec![0xa0, 4]]);
        assert_eq!(
            logged(&e, 0x0089_7910),
            vec![
                vec![PLAYER, 0x14, 0, 0, 0xa0],
                vec![PLAYER, 0x14, 0, 0, 0xa1]
            ]
        );
        assert_eq!(logged(&e, 0x0049_94f0), vec![vec![0xa1, 4, 0]]);

        // Bit 0x400 unchanged: nothing else.
        e.call_log = Some(vec![]);
        e.call(0x009e_a3e0, &args![mover, 0x402u32]);
        assert!(logged(&e, GET_ANIMATION).is_empty());
    }

    #[test]
    fn set_side_vector_copies_the_words() {
        let (mut e, mover) = engine();
        e.mem.set_u32(SCRATCH, 0x7f80_0001);
        e.mem.set_f32(SCRATCH + 4, 2.0);
        e.mem.set_f32(SCRATCH + 8, 3.0);
        e.call(0x009e_a570, &args![mover, SCRATCH]);
        let side = mover.at(PlayerMover::SideVec).addr();
        assert_eq!(e.mem.u32(side), 0x7f80_0001);
        assert_eq!(point(&e, side)[1..], [2.0, 3.0]);
    }

    #[test]
    fn save_game_writes_the_fields_after_the_base() {
        let (mut e, mover) = engine();
        e.call_log = Some(vec![]);
        e.call(0x009e_a5a0, &args![mover, 0x1234u32]);
        assert_eq!(
            logged(&e, ACTOR_MOVER_SAVE_GAME),
            vec![vec![mover.addr(), 0x1234]]
        );
        let m = mover.addr();
        assert_eq!(
            logged(&e, SAVE_WRITE),
            vec![
                vec![0x1234, m + 0x88, 0xc, 0],
                vec![0x1234, m + 0x94, 4, 0],
                vec![0x1234, m + 0x98, 4, 0],
                vec![0x1234, m + 0x9c, 4, 0],
            ]
        );
    }

    #[test]
    fn load_game_reads_the_fields_after_the_base() {
        let (mut e, mover) = engine();
        e.call_log = Some(vec![]);
        e.call(0x009e_a610, &args![mover, 0x1234u32]);
        assert_eq!(
            logged(&e, ACTOR_MOVER_LOAD_GAME),
            vec![vec![mover.addr(), 0x1234]]
        );
        let m = mover.addr();
        assert_eq!(
            logged(&e, LOAD_READ),
            vec![
                vec![0x1234, m + 0x88, 0xc],
                vec![0x1234, m + 0x94, 4],
                vec![0x1234, m + 0x98, 4],
                vec![0x1234, m + 0x9c, 4],
            ]
        );
    }

    #[test]
    fn init_load_game_runs_the_base() {
        let (mut e, mover) = engine();
        e.call_log = Some(vec![]);
        e.call(0x009e_a680, &args![mover, 0x1234u32]);
        assert_eq!(
            logged(&e, ACTOR_MOVER_INIT_LOAD_GAME),
            vec![vec![mover.addr(), 0x1234]]
        );
    }

    #[test]
    fn revert_clears_the_player_fields() {
        let (mut e, mover) = engine();
        e.set(mover, PlayerMover::MoveMode, 7);
        e.set(mover, PlayerMover::LastTurnDir, 0x10);
        e.set(mover.at(PlayerMover::SideVec), NiPoint3::x, 5.0);
        e.mem.set_f32(UNSET_POINT + 4, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x009e_a6a0, &args![mover, 0x1234u32]);
        assert_eq!(
            logged(&e, ACTOR_MOVER_REVERT),
            vec![vec![mover.addr(), 0x1234]]
        );
        assert_eq!(
            point(&e, mover.at(PlayerMover::SideVec).addr()),
            [0.0, 9.0, 0.0]
        );
        assert_eq!(e.get(mover, PlayerMover::MoveMode), 0);
        assert_eq!(e.get(mover, PlayerMover::LastTurnDir), 0);
        assert_eq!(e.get(mover, PlayerMover::fMinTurnTime), -1.0);
    }
}

//! `fallout/ai/combat/continuousbeamprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `005de0a0` (thiscall(mask, on)): sets or clears a projectile flag.
const SET_FLAG: u32 = 0x005d_e0a0;
/// `00559450` (thiscall): the word at `this`.
const WORD_AT: u32 = 0x0055_9450;
/// `00874480` (thiscall): the projectile's shooter.
const GET_SHOOTER: u32 = 0x0087_4480;
/// `009bc8f0` (thiscall): ends the projectile.
const KILL: u32 = 0x009b_c8f0;
/// `00979260` (thiscall, ST0) and `00644660` (thiscall, ST0): two
/// `float`s of the projectile handed to its +0x150 object.
const PROJECTILE_VALUE_00979260: u32 = 0x0097_9260;
const PROJECTILE_VALUE_00644660: u32 = 0x0064_4660;
/// `005bfbd0` (thiscall(`float`)) and `0051b840` (thiscall(`float`)) on the
/// projectile's +0x150 object.
const SET_VALUE_005BFBD0: u32 = 0x005b_fbd0;
const SET_VALUE_0051B840: u32 = 0x0051_b840;
/// `005b5e40` (cdecl(format, ...)): the game's message print.
const PRINT: u32 = 0x005b_5e40;
/// `"Continuous Beam created without Shooter!"`.
const NO_SHOOTER_MESSAGE: u32 = 0x0108_ed94;
/// `00444d00` (thiscall(mask), bool): whether a projectile flag is set.
const HAS_FLAG: u32 = 0x0044_4d00;
/// `0.01` (`double`).
const MIN_LIFE: u32 = 0x0103_1148;
/// `pi / 180` (`double`, the `float` widened).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `2 pi` (`float`).
const TWO_PI: u32 = 0x0101_ff50;
/// The `float` y of the iron-sights offset (`-15.0`).
const IRON_SIGHTS_Y: u32 = 0x0105_3114;
/// The `float` `10000.0` handed to `009795c0`.
const BEAM_LENGTH: u32 = 0x0102_2958;
/// The setting whose `float` stands in for the shooter's spread value
/// without a shooter.
const SPREAD_SETTING: u32 = 0x011c_e23c;
/// `UpdateProjectile`'s function-local `static NiPoint3` (the iron-sights
/// offset) and the guard word whose bit 0 says it has been built.
const IRON_SIGHTS_OFFSET: u32 = 0x011f_1db0;
const IRON_SIGHTS_OFFSET_BUILT: u32 = 0x011f_1dbc;
/// `RandomFloat` (`00476b70`, cdecl(min, max), ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `00476c90` (thiscall): the projectile's weapon data.
const WEAPON_DATA: u32 = 0x0047_6c90;
/// `00461130` (thiscall): an object's transform (the matrix at its
/// start).
const TRANSFORM: u32 = 0x0046_1130;
/// `008d8520` (thiscall): the actor's process.
const GET_PROCESS: u32 = 0x008d_8520;
/// `008b0dd0` (thiscall(which), ST0): one of the shooter's spread values.
const SHOOTER_SPREAD: u32 = 0x008b_0dd0;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`, thiscall(point)).
const SET_LOCATION: u32 = 0x0057_5830;

// Translated from 009ab280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContinuousBeamProjectile::Initialize` (Xbox PDB): sets the flags
/// (`005de0a0`): 0x10 off, 1 on, 2 off, 8 on, 0x20 off, 0x100, 0x80 and
/// 0x2000 on; clears +0x154; `009bda10`; when the +0x150 word is set,
/// hands it `00979260`'s `float` (`005bfbd0`). With a shooter, a beam the
/// shooter already has (+0x1A0) is ended (`009bc8f0`) and the shooter's
/// +0x1A0 becomes this one; without one, prints
/// `"Continuous Beam created without Shooter!"` and ends this beam.
pub fn continuous_beam_projectile_initialize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (mask, on) in [
        (0x10u32, 0u32),
        (1, 1),
        (2, 0),
        (8, 1),
        (0x20, 0),
        (0x100, 1),
        (0x80, 1),
        (0x2000, 1),
    ] {
        e.call(SET_FLAG, &args![this, mask, on]);
    }
    e.mem.set_u32(base + 0x154, 0);
    e.call(0x009b_da10, &args![this]);
    if e.call(WORD_AT, &args![base + 0x150]).u32() != 0 {
        let value = x87_store32(e.call(PROJECTILE_VALUE_00979260, &args![this]).f64());
        let object = e.call(WORD_AT, &args![base + 0x150]).u32();
        e.call(SET_VALUE_005BFBD0, &args![object, value]);
    }
    let shooter = e.call(GET_SHOOTER, &args![this]).u32();
    if shooter != 0 {
        let previous = e.mem.u32(shooter.wrapping_add(0x1a0));
        if previous != 0 {
            e.call(KILL, &args![previous]);
        }
        e.mem.set_u32(shooter.wrapping_add(0x1a0), base);
    } else {
        e.call(PRINT, &args![NO_SHOOTER_MESSAGE]);
        e.call(KILL, &args![this]);
    }
}

// Translated from 009ab390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ab390 - C++ exception unwinding (FS:[0] frame state, the static's construction state) is not translated.
/// The beam's per-frame update (thiscall(`time`, a `float`), `RET 4`; no
/// Xbox PDB name). Nothing when `00440d80` answers true. Otherwise:
/// `Projectile::CheckExplosion(time)`; the shooter (`00874480`) counts
/// only when its virtual +0x100 answers true, and its node is its virtual
/// +0x20C (any shooter); a projectile with flag 0x20 whose `00644660` is
/// below 0.01 is ended (`009bc8f0`).
///
/// With a shooter, the beam is aimed: from the player's +0xD30 point
/// (`009ab970`) toward this one (+0x30: `00439ef0`, `004a0c10`, z angle
/// `004b13c0`, x angle the negated `004b5530` of the offset's z) for the
/// player, from the node's transform (`NiMatrix3::ToEulerAnglesZXY`) for
/// others, after the shooter's own look angles (`00931d70`, virtual
/// +0x2BC(0)). Unless the use-weapon package data (`009344a0`,
/// `006758f0`, `00424940`) or, without a package, flag 0x1000 says so, a
/// random spread is added: the cone (degrees, `00524b80` + `00524be0` x the
/// shooter's `008b0dd0(1)` or the setting at `011ce23c`, the mod flag from
/// the process's virtual +0x148 and `004bda70(3)`) plus the process's
/// virtual +0x42C, a random radius in it and a random direction in `2 pi`
/// (`RandomFloat`, `004e44b0`, `004e4470`). The angles go to
/// `005757d0` (z), `00575770` (x) and `005757a0` (y = 0).
///
/// With a node and virtual +0x1D0, the beam starts at the node's point
/// (`0045bb80`) or, for the player in iron sights with `00524c90`'s node,
/// at that node's transform applied to the static offset (0, -15, 0)
/// (`00524c40`). Then `Projectile::Sync3DWithReference(1, 0)`,
/// `009795c0(10000.0)`, virtual +0x328 and +0x30C, and the +0x150 object
/// gets `00644660` (`0051b840`) and `00979260` (`005bfbd0`).
pub fn fn_009ab390(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    if e.call(0x0044_0d80, &args![this]).bool() {
        return;
    }
    // The function's frame: locals at `ebp - x` as the exe keeps them.
    e.with_stack(0xb0, |e, frame| {
        let ebp = frame.addr() + 0xb0;
        let angle_x = ebp - 0x20;
        let angle_z = ebp - 0x1c;

        e.call(0x009c_3190, &args![this, x87_quiet_bits(time.to_bits())]);
        let candidate = e.call(GET_SHOOTER, &args![this]).u32();
        let shooter = if e.vcall(candidate, 0x100, &args![]).bool() {
            e.call(GET_SHOOTER, &args![this]).u32()
        } else {
            0
        };
        let node = if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
            let owner = e.call(GET_SHOOTER, &args![this]).u32();
            e.vcall(owner, 0x20c, &args![]).u32()
        } else {
            0
        };
        if e.call(HAS_FLAG, &args![this, 0x20u32]).bool() {
            let life = e.call(PROJECTILE_VALUE_00644660, &args![this]).f64();
            // `FCOMP`/`TEST AH,5`/`JP`: only an ordered "below" ends it.
            if life < e.global::<f64>(MIN_LIFE) {
                e.call(KILL, &args![this]);
            }
        }
        if shooter != 0 {
            let looking = e.call(0x0093_1d70, &args![shooter]).f64();
            e.mem.set_u32(angle_x, x87_store32(looking));
            let heading = e.vcall(shooter, 0x2bc, &args![0u32]).f64();
            e.mem.set_u32(angle_z, x87_store32(heading));
            if shooter == e.global::<u32>(PLAYER_POINTER) {
                let offset = ebp - 0x2c;
                let player = Ptr::new(e.global::<u32>(PLAYER_POINTER));
                let point = fn_009ab970(e, player, Ptr::new(ebp - 0x74));
                e.call(0x0043_9ef0, &args![point, offset, base + 0x30]);
                e.call(0x004a_0c10, &args![offset]);
                let z = e.call(0x004b_13c0, &args![offset]).f64();
                e.mem.set_u32(angle_z, x87_store32(z));
                let height = x87_quiet_bits(e.mem.u32(offset + 8));
                let pitch = e.call(0x004b_5530, &args![height]).f64();
                // `FCHS`: the sign bit flips, a NaN's included.
                e.mem.set_u32(angle_x, x87_store32(-pitch));
            } else {
                let matrix = e.call(TRANSFORM, &args![node]).u32();
                e.call(0x00a5_9400, &args![matrix, angle_z, angle_x, ebp - 0x30]);
            }
            let mut skip_spread = false;
            let package = if shooter != 0 {
                e.call(0x0093_44a0, &args![shooter]).u32()
            } else {
                0
            };
            if package != 0 {
                let data = e.call(0x0067_58f0, &args![package]).u32();
                if data != 0 {
                    skip_spread = e.call(0x0042_4940, &args![data]).bool();
                }
            } else {
                skip_spread = e.call(HAS_FLAG, &args![this, 0x1000u32]).bool();
            }
            if !skip_spread {
                spread(e, this, shooter, angle_x, angle_z, ebp);
            }
            let z = x87_quiet_bits(e.mem.u32(angle_z));
            e.call(0x0057_57d0, &args![this, z]);
            let x = x87_quiet_bits(e.mem.u32(angle_x));
            e.call(0x0057_5770, &args![this, x]);
            e.call(0x0057_57a0, &args![this, x87_store32(0.0)]);
        }
        let started = e.vcall(base, 0x1d0, &args![]).u32();
        if node != 0 && started != 0 {
            let mut iron_sights = false;
            let data = e.call(WEAPON_DATA, &args![this]).u32();
            if e.call(0x004a_d010, &args![data]).bool()
                && e.call(0x008b_bc10, &args![shooter]).bool()
                && shooter == e.global::<u32>(PLAYER_POINTER)
            {
                iron_sights = true;
            }
            let sight_node = e.call(0x0052_4c90, &args![]).u32();
            if iron_sights && sight_node != 0 {
                let guard = e.global::<u32>(IRON_SIGHTS_OFFSET_BUILT);
                if guard & 1 == 0 {
                    e.set_global(IRON_SIGHTS_OFFSET_BUILT, guard | 1);
                    let y = x87_quiet_bits(e.global::<u32>(IRON_SIGHTS_Y));
                    let zero = x87_store32(0.0);
                    e.call(0x0041_6870, &args![IRON_SIGHTS_OFFSET, zero, y, zero]);
                }
                let transform = e.call(TRANSFORM, &args![sight_node]).u32();
                let point = e
                    .call(
                        0x0052_4c40,
                        &args![transform, ebp - 0x80, IRON_SIGHTS_OFFSET],
                    )
                    .u32();
                e.call(SET_LOCATION, &args![this, point]);
            } else {
                let point = e.call(0x0045_bb80, &args![node]).u32();
                e.call(SET_LOCATION, &args![this, point]);
            }
        }
        e.call(0x009b_f470, &args![this, 1u32, 0u32]);
        let length = x87_quiet_bits(e.global::<u32>(BEAM_LENGTH));
        e.call(0x0097_95c0, &args![this, length]);
        e.vcall(base, 0x328, &args![]);
        e.vcall(base, 0x30c, &args![]);
        if e.call(WORD_AT, &args![base + 0x150]).u32() != 0 {
            let value = x87_store32(e.call(PROJECTILE_VALUE_00644660, &args![this]).f64());
            let object = e.call(WORD_AT, &args![base + 0x150]).u32();
            e.call(SET_VALUE_0051B840, &args![object, value]);
            let value = x87_store32(e.call(PROJECTILE_VALUE_00979260, &args![this]).f64());
            let object = e.call(WORD_AT, &args![base + 0x150]).u32();
            e.call(SET_VALUE_005BFBD0, &args![object, value]);
        }
    });
}

/// The random spread of [`fn_009ab390`], on its frame (`ebp`): adds a
/// random offset inside the shooter's cone to the two angles.
fn spread(e: &mut Engine, this: Ptr, shooter: u32, angle_x: u32, angle_z: u32, ebp: u32) {
    let process = if shooter != 0 {
        e.call(GET_PROCESS, &args![shooter]).u32()
    } else {
        0
    };
    let item = if process != 0 {
        e.vcall(process, 0x148, &args![]).u32()
    } else {
        0
    };
    let skill = if shooter != 0 {
        x87_store32(e.call(SHOOTER_SPREAD, &args![shooter, 1u32]).f64())
    } else {
        let setting = e.call(0x0040_3e20, &args![SPREAD_SETTING]).u32();
        x87_quiet_bits(e.mem.u32(setting))
    };
    let skill = x87_quiet_bits(skill);
    e.mem.set_u32(ebp - 0x4c, skill);
    let mut modded = 0u32;
    if shooter != 0
        && e.call(GET_PROCESS, &args![shooter]).u32() != 0
        && item != 0
        && e.call(0x004b_da70, &args![item, 3u32]).bool()
    {
        modded = 1;
    }
    let data = e.call(WEAPON_DATA, &args![this]).u32();
    let minimum = e.call(0x0052_4b80, &args![data, modded]).f64();
    let data = e.call(WEAPON_DATA, &args![this]).u32();
    let scaled = e.call(0x0052_4be0, &args![data, modded]).f64();
    let cone = x87_mul(scaled, x87_m32(skill));
    let cone = x87_add(cone, minimum);
    let cone = x87_mul(cone, e.global::<f64>(DEGREES_TO_RADIANS));
    let mut cone = x87_store32(cone);
    let unused = x87_store32(e.call(SHOOTER_SPREAD, &args![shooter, 2u32]).f64());
    e.mem.set_u32(ebp - 0x44, unused);
    let process = e.call(GET_PROCESS, &args![shooter]).u32();
    let extra = x87_store32(e.vcall(process, 0x42c, &args![]).f64());
    cone = x87_store32(x87_add(x87_load32(cone), x87_m32(extra)));
    let zero = x87_store32(0.0);
    let radius = e
        .call(RANDOM_FLOAT, &args![zero, x87_quiet_bits(cone)])
        .f64();
    let radius = x87_store32(radius);
    let two_pi = x87_quiet_bits(e.global::<u32>(TWO_PI));
    let direction = x87_store32(e.call(RANDOM_FLOAT, &args![zero, two_pi]).f64());
    let direction = x87_quiet_bits(direction);
    let cosine = e.call(0x004e_44b0, &args![direction]).f64();
    let x = x87_add(
        x87_mul(cosine, x87_m32(radius)),
        x87_m32(e.mem.u32(angle_x)),
    );
    e.mem.set_u32(angle_x, x87_store32(x));
    let sine = e.call(0x004e_4470, &args![direction]).f64();
    let z = x87_add(x87_mul(sine, x87_m32(radius)), x87_m32(e.mem.u32(angle_z)));
    e.mem.set_u32(angle_z, x87_store32(z));
}

// Translated from 009ab970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 12-byte point at +0xD30 of `this` into `out` and returns
/// `out` (thiscall(`out`), `RET 4`; the beam calls it on the player).
pub fn fn_009ab970(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(this.addr().wrapping_add(0xd30 + 4 * word));
        e.mem.set_u32(out.addr().wrapping_add(4 * word), value);
    }
    out
}

// Translated from 009ab9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContinuousBeamProjectile::SetKill` (Xbox PDB; thiscall, `RET 4`, the
/// word not read): sets flag 0x20 (`005de0a0(0x20, 1)`).
pub fn continuous_beam_projectile_set_kill(e: &mut Engine, this: Ptr, _unused_0: u32) {
    e.call(SET_FLAG, &args![this, 0x20u32, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_b280, continuous_beam_projectile_initialize(Ptr)),
        entry!(0x009a_b390, fn_009ab390(Ptr, f32)),
        entry!(0x009a_b970, fn_009ab970(Ptr, Ptr) -> Ptr),
        entry!(0x009a_b9a0, continuous_beam_projectile_set_kill(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn initialize(shooter: u32, previous: u32) -> (u32, Engine, Logged) {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[SET_FLAG, 0x009b_da10, KILL, PRINT, SET_VALUE_005BFBD0],
        );
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PROJECTILE_VALUE_00979260, |_, _| st0(0.1));
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 0x150, 0x0abc_0000);
        e.mem.set_u32(this + 0x154, 0xffff_ffff);
        let shooter = if shooter != 0 {
            let actor = e.mem.alloc(0x200);
            e.mem.set_u32(actor + 0x1a0, previous);
            actor
        } else {
            0
        };
        e.register_double(GET_SHOOTER, move |_, _| shooter.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009a_b280, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x154), 0);
        let flags: Vec<_> = calls(&log, SET_FLAG).iter().map(|w| (w[1], w[2])).collect();
        assert_eq!(
            flags,
            [
                (0x10, 0),
                (1, 1),
                (2, 0),
                (8, 1),
                (0x20, 0),
                (0x100, 1),
                (0x80, 1),
                (0x2000, 1)
            ]
        );
        assert_eq!(
            calls(&log, SET_VALUE_005BFBD0),
            vec![vec![0x0abc_0000, 0.1f32.to_bits()]]
        );
        (shooter, e, log)
    }

    #[test]
    fn initialize_registers_with_the_shooter() {
        let (shooter, e, log) = initialize(1, 0x7777);
        assert_eq!(calls(&log, KILL), vec![vec![0x7777]]);
        let this = calls(&log, 0x009b_da10)[0][0];
        assert_eq!(e.mem.u32(shooter + 0x1a0), this);
        let (_, _, log) = initialize(1, 0);
        assert!(calls(&log, KILL).is_empty());
    }

    #[test]
    fn initialize_without_a_shooter_ends_the_beam() {
        let (_, _, log) = initialize(0, 0);
        assert_eq!(calls(&log, PRINT), vec![vec![NO_SHOOTER_MESSAGE]]);
        let this = calls(&log, 0x009b_da10)[0][0];
        assert_eq!(calls(&log, KILL), vec![vec![this]]);
    }

    #[test]
    fn point_copy_and_set_kill() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd40);
        for i in 0..4 {
            e.mem.set_u32(this + 0xd30 + 4 * i, 0x100 + i);
        }
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x009a_b970, &args![this, out]).u32(), out);
        assert_eq!(
            (0..4).map(|i| e.mem.u32(out + 4 * i)).collect::<Vec<_>>(),
            vec![0x100, 0x101, 0x102, 0]
        );
        quiet(&mut e, &[SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x009a_b9a0, &args![this, 0x55u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_FLAG), vec![vec![this, 0x20, 1]]);
    }

    /// The beam's update world. `player`: the shooter is the player;
    /// `package`: 0 none, 1 with use-weapon data that skips the spread.
    struct Beam {
        dead: bool,
        player: bool,
        counts: bool,
        package: u32,
        flag_1000: bool,
        iron_sights: bool,
    }

    const SHOOTER_VTABLE: u32 = 0x0a00_0000;
    const BEAM_VTABLE: u32 = 0x0a01_0000;
    const PROCESS_VTABLE: u32 = 0x0a02_0000;

    fn vtable(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        let size = slots.iter().map(|(s, _)| *s).max().unwrap() / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, target) in slots {
            words[*slot as usize / 4] = *target;
        }
        e.put_vtable(address, &words);
    }

    fn update(world: Beam) -> (u32, Engine, Logged) {
        let mut e = Engine::new();
        for (address, value) in [
            (MIN_LIFE, 0.01f64),
            (DEGREES_TO_RADIANS, f64::from(std::f32::consts::PI / 180.0)),
        ] {
            e.map(address & !0xfff, 0x1000);
            e.set_global(address, value);
        }
        for (address, value) in [
            (TWO_PI, 2.0f32 * std::f32::consts::PI),
            (IRON_SIGHTS_Y, -15.0),
            (BEAM_LENGTH, 10000.0),
        ] {
            e.map(address & !0xfff, 0x1000);
            e.set_global(address, value.to_bits());
        }
        e.map(0x011c_e000, 0x1000);
        e.mem.set_f32(SPREAD_SETTING + 4, 3.0);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_1000, 0x1000);

        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, BEAM_VTABLE);
        vtable(
            &mut e,
            BEAM_VTABLE,
            &[
                (0x1d0, 0x0b00_01d0),
                (0x328, 0x0b00_0328),
                (0x30c, 0x0b00_030c),
            ],
        );
        e.register(0x0b00_01d0, |_, _| 1u32.into_ret());
        quiet(&mut e, &[0x0b00_0328, 0x0b00_030c]);
        for i in 0..3 {
            e.mem
                .set_f32(this + 0x30 + 4 * i, [1.0f32, 2.0, 3.0][i as usize]);
        }
        e.mem.set_u32(this + 0x150, 0x0abc_0000);

        let shooter = e.mem.alloc(0xe00);
        e.mem.set_u32(shooter, SHOOTER_VTABLE);
        for i in 0..3 {
            e.mem
                .set_f32(shooter + 0xd30 + 4 * i, [4.0f32, 6.0, 3.0][i as usize]);
        }
        vtable(
            &mut e,
            SHOOTER_VTABLE,
            &[
                (0x100, 0x0b00_0100),
                (0x20c, 0x0b00_020c),
                (0x2bc, 0x0b00_02bc),
            ],
        );
        let counts = world.counts;
        e.register_double(0x0b00_0100, move |_, _| u32::from(counts).into_ret());
        e.register(0x0b00_020c, |_, _| 0x0c00_0000u32.into_ret());
        e.register(0x0b00_02bc, |_, _| st0(0.5));
        e.set_global(PLAYER_POINTER, if world.player { shooter } else { 0x1234 });
        e.register_double(GET_SHOOTER, move |_, _| shooter.into_ret());

        let process = e.mem.alloc(0x10);
        e.mem.set_u32(process, PROCESS_VTABLE);
        vtable(
            &mut e,
            PROCESS_VTABLE,
            &[(0x148, 0x0b00_0148), (0x42c, 0x0b00_042c)],
        );
        e.register(0x0b00_0148, |_, _| 0x0d00_0000u32.into_ret());
        e.register(0x0b00_042c, |_, _| st0(0.25));
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());

        let dead = world.dead;
        e.register_double(0x0044_0d80, move |_, _| u32::from(dead).into_ret());
        let flag_1000 = world.flag_1000;
        e.register_double(HAS_FLAG, move |_, a| {
            u32::from(a[1] == 0x1000 && flag_1000).into_ret()
        });
        e.register(PROJECTILE_VALUE_00644660, |_, _| st0(2.0));
        e.register(PROJECTILE_VALUE_00979260, |_, _| st0(3.0));
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0093_1d70, |_, _| st0(0.125));
        e.register(0x0043_9ef0, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(0x004b_13c0, |_, _| st0(1.5));
        e.register(0x004b_5530, |_, a| {
            st0(f64::from(f32::from_bits(a[0])) / 4.0)
        });
        e.register(TRANSFORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x00a5_9400, |e, a| {
            e.mem.set_f32(a[1], 0.75);
            e.mem.set_f32(a[2], -0.25);
            Ret::default()
        });
        let package = world.package;
        e.register_double(0x0093_44a0, move |_, _| package.into_ret());
        e.register(0x0067_58f0, |_, _| 0x0e00_0000u32.into_ret());
        e.register(0x0042_4940, |_, _| 1u32.into_ret());
        e.register(SHOOTER_SPREAD, |_, a| st0(f64::from(a[1]) * 2.0));
        e.register(0x004b_da70, |_, a| u32::from(a[1] == 3).into_ret());
        e.register(WEAPON_DATA, |_, _| 0x0f00_0000u32.into_ret());
        e.register(0x0052_4b80, |_, a| st0(1.0 + f64::from(a[1])));
        e.register(0x0052_4be0, |_, _| st0(0.5));
        e.register(RANDOM_FLOAT, |_, a| {
            assert_eq!(a[0], 0);
            st0(f64::from(f32::from_bits(a[1])) / 2.0)
        });
        e.register(0x004e_44b0, |_, _| st0(1.0));
        e.register(0x004e_4470, |_, _| st0(-1.0));
        let iron = world.iron_sights;
        e.register_double(0x004a_d010, move |_, _| u32::from(iron).into_ret());
        e.register(0x008b_bc10, |_, _| 1u32.into_ret());
        e.register(0x0052_4c90, |_, _| 0x0c10_0000u32.into_ret());
        e.register(0x0052_4c40, |_, a| a[1].into_ret());
        e.register(0x0045_bb80, |_, a| (a[0] + 0x20).into_ret());
        quiet(
            &mut e,
            &[
                0x009c_3190,
                KILL,
                0x004a_0c10,
                0x0057_57d0,
                0x0057_5770,
                0x0057_57a0,
                0x0041_6870,
                SET_LOCATION,
                0x009b_f470,
                0x0097_95c0,
                SET_VALUE_0051B840,
                SET_VALUE_005BFBD0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x009a_b390, &args![this, 0x7f80_0001u32]);
        let log = e.call_log.take().unwrap();
        (this, e, log)
    }

    fn angle(log: &CallLog, setter: u32) -> f32 {
        let set = calls(log, setter);
        assert_eq!(set.len(), 1);
        f32::from_bits(set[0][1])
    }

    #[test]
    fn update_does_nothing_for_a_dead_beam() {
        let (_, _, log) = update(Beam {
            dead: true,
            player: true,
            counts: true,
            package: 0,
            flag_1000: false,
            iron_sights: false,
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn update_aims_the_player_beam_with_spread() {
        let (this, _, log) = update(Beam {
            dead: false,
            player: true,
            counts: true,
            package: 0,
            flag_1000: false,
            iron_sights: true,
        });
        // The time is quieted on its way to CheckExplosion.
        assert_eq!(calls(&log, 0x009c_3190), vec![vec![this, 0x7fc0_0001]]);
        // Offset (4,6,3) - (1,2,3) = (3,4,0): x = -asin-like(0)/4 = -0.
        // Spread: modded (3 answers true): (0.5 * 2 + 2) degrees in
        // radians, + 0.25; radius half of it; direction pi.
        let cone = x87_store32(x87_mul(
            x87_add(x87_mul(0.5, 2.0), 2.0),
            f64::from(std::f32::consts::PI / 180.0),
        ));
        let cone = x87_store32(x87_add(x87_load32(cone), 0.25));
        let radius = x87_store32(x87_load32(cone) / 2.0);
        let radius = f32::from_bits(radius);
        assert_eq!(angle(&log, 0x0057_5770), radius);
        assert_eq!(angle(&log, 0x0057_57d0), 1.5 - radius);
        assert_eq!(calls(&log, 0x0057_57a0)[0][1], 0);
        assert_eq!(calls(&log, 0x0052_4b80)[0][1], 1);
        // Iron sights: the static offset is built once, and the location
        // comes from the sight node's transform.
        assert_eq!(
            calls(&log, 0x0041_6870),
            vec![vec![IRON_SIGHTS_OFFSET, 0, (-15.0f32).to_bits(), 0]]
        );
        let set = calls(&log, SET_LOCATION);
        assert_eq!(set.len(), 1);
        assert_eq!(calls(&log, 0x0052_4c40)[0][0], 0x0c10_0010);
        assert_eq!(calls(&log, 0x0052_4c40)[0][2], IRON_SIGHTS_OFFSET);
        assert_eq!(
            calls(&log, 0x0097_95c0),
            vec![vec![this, 10000.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, SET_VALUE_0051B840),
            vec![vec![0x0abc_0000, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, SET_VALUE_005BFBD0),
            vec![vec![0x0abc_0000, 3.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, 0x009b_f470), vec![vec![this, 1, 0]]);
    }

    #[test]
    fn update_aims_another_shooter_from_its_node() {
        let (_, e, log) = update(Beam {
            dead: false,
            player: false,
            counts: true,
            package: 0x0e10_0000,
            flag_1000: false,
            iron_sights: true,
        });
        // The node's Euler angles stand; the package data skips the spread.
        assert_eq!(angle(&log, 0x0057_57d0), 0.75);
        assert_eq!(angle(&log, 0x0057_5770), -0.25);
        assert!(calls(&log, RANDOM_FLOAT).is_empty());
        assert_eq!(calls(&log, 0x00a5_9400)[0][0], 0x0c00_0010);
        // Not the player: the location is the node's point.
        assert_eq!(calls(&log, SET_LOCATION)[0][1], 0x0c00_0020);
        assert!(calls(&log, 0x0041_6870).is_empty());
        assert_eq!(e.global::<u32>(IRON_SIGHTS_OFFSET_BUILT), 0);
    }

    #[test]
    fn update_without_a_counting_shooter_skips_the_aim() {
        let (_, _, log) = update(Beam {
            dead: false,
            player: false,
            counts: false,
            package: 0,
            flag_1000: true,
            iron_sights: false,
        });
        assert!(calls(&log, 0x0057_57d0).is_empty());
        assert!(calls(&log, 0x0093_1d70).is_empty());
        // The node still positions the beam.
        assert_eq!(calls(&log, SET_LOCATION)[0][1], 0x0c00_0020);
    }
}

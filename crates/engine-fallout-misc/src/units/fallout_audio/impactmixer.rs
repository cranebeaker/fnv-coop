//! `fallout/audio/impactmixer.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0083b640` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_0083B640: u32 = 0x0083_b640;
/// The vtable stored at `+0` (RTTI class `NiTPointerMap<int,unsigned int>`).
const VTABLE_0083B5E0: u32 = 0x0107_d9c4;

// Translated from 0083b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `impactmixer.cpp`): runs `0083b640(this, argument)`, then
/// stores the vtable `0107d9c4` (RTTI class `NiTPointerMap<int,unsigned int>`) at `+0`.
/// Returns `this`.
pub fn fn_0083b5e0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0083B640, &args![this, argument]);
    e.mem.set_u32(this.addr(), VTABLE_0083B5E0);
    this
}

// Translated from 008397d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`impactmixer.cpp`). `__fastcall` getter: the byte at `+0x7c6` of
/// `this`, in `AL`.
pub fn fn_008397d0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x7c6))
}

/// Per-material name overrides (`011dd87c`) and the default names
/// (`011a1888`), one word per material.
const MATERIAL_NAME_OVERRIDES: u32 = 0x011d_d87c;
const MATERIAL_NAMES: u32 = 0x011a_1888;
/// Thiscall (flags), `RET 4` (`cameracaster.rs`), on the mixer's +0x410.
const FLAGS_TEST_00621270: u32 = 0x0062_1270;
/// `InitFootsteps`'s done flag (`011dd85c`).
const FOOTSTEPS_READY: u32 = 0x011d_d85c;
/// The footstep sound table: 13 materials of 10 words (0x28 bytes).
const FOOTSTEP_SOUNDS: u32 = 0x011d_d650;
/// Each material's fallback material (13 means none).
const FALLBACK_MATERIALS: u32 = 0x011a_17c4;
/// The 10 footstep sound name suffixes.
const FOOTSTEP_SUFFIXES: u32 = 0x011a_18bc;
/// `"FST%s%s"` and the default footstep sound name.
const FOOTSTEP_FORMAT: u32 = 0x0107_d200;
const DEFAULT_FOOTSTEP: u32 = 0x0107_d1ec;
/// The formatted print (`run_00406cc0.rs`), cdecl (buffer, size, format,
/// ...).
const FORMAT: u32 = 0x0040_6d00;
/// `TESDataHandler::GetSound_ov2` (engine map), thiscall on the data handler
/// (`011c3f2c`) (editor id), `RET 4`.
const GET_SOUND: u32 = 0x0046_16c0;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The mixer's flags (`011dd878`): bit 1 says the impact table is reset.
const MIXER_FLAGS: u32 = 0x011d_d878;
/// The 20 impact records of 0x24 bytes (`011dda00`).
const IMPACT_RECORDS: u32 = 0x011d_da00;
/// The largest `float` (`01016970`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// A table of 0x24 words (`011a17f8`), read for indices 1 to 0x23.
const TABLE_011A17F8: u32 = 0x011a_17f8;
/// `Setting` value access (`effectitem.rs`), thiscall on the setting: the
/// address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The audio settings `GetSoundForType` compares with (their names from
/// the static initializers).
const WOOD_MEDIUM_MASS_MIN: u32 = 0x011d_d930;
const WOOD_LARGE_MASS_MIN: u32 = 0x011d_d9d8;
const STONE_MEDIUM_MASS_MIN: u32 = 0x011d_d9c8;
const STONE_LARGE_MASS_MIN: u32 = 0x011d_d9b0;
const EARTH_MEDIUM_MASS_MIN: u32 = 0x011d_d918;
const EARTH_LARGE_MASS_MIN: u32 = 0x011d_d9bc;
const SKIN_MEDIUM_MASS_MIN: u32 = 0x011d_d9a0;
const SKIN_LARGE_MASS_MIN: u32 = 0x011d_d924;
const METAL_MEDIUM_MASS_MIN: u32 = 0x011d_d8f4;
const METAL_LARGE_MASS_MIN: u32 = 0x011d_d8cc;
const COLLISION_SOUND_HEAVY_THRESHOLD: u32 = 0x011d_d8e8;
/// `fMaxFootstepDistance:Audio`.
const MAX_FOOTSTEP_DISTANCE: u32 = 0x011d_d900;
/// `double` thresholds: about `1e-4`, `6.0`, `70.0` and `15.0`.
const SMALL_MASS_01032980: u32 = 0x0103_2980;
const MASS_6_01034208: u32 = 0x0103_4208;
const MASS_70_01073568: u32 = 0x0107_3568;
const MASS_15_01015A38: u32 = 0x0101_5a38;
/// The Gamebryo `malloc`/`free` (`run_00aa1010.rs`) and `memset`
/// (`crt.rs`), cdecl.
const NI_MALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int, unsigned int>` and
/// `NiTPointerMap<int, unsigned int>` vtables (RTTI).
const MAP_BASE_VTABLE: u32 = 0x0107_d9e4;
/// The map's `RemoveAll` (`run_00438af0.rs`), thiscall.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;

// Translated from 008374b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): material `material`'s name, the
/// override (`011dd87c`) when set, else the default (`011a1888`).
pub fn fn_008374b0(e: &mut Engine, material: u32) -> u32 {
    let at = material.wrapping_mul(4);
    let name = e.mem.u32(MATERIAL_NAME_OVERRIDES.wrapping_add(at));
    if name != 0 {
        name
    } else {
        e.mem.u32(MATERIAL_NAMES.wrapping_add(at))
    }
}

// Translated from 00838750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether flag `0x1000000` is set in the flags
/// at `this + 0x410` (`00621270`), as 1 or 0.
pub fn fn_00838750(e: &mut Engine, this: Ptr) -> u32 {
    let set = e
        .call(
            FLAGS_TEST_00621270,
            &args![this.addr() + 0x410, 0x0100_0000u32],
        )
        .u32();
    u32::from(set != 0)
}

/// The sound `"FST" + material name + suffix` (through the data handler).
fn footstep_sound(e: &mut Engine, buffer: u32, material: u32, suffix: u32) -> u32 {
    let name = fn_008374b0(e, material);
    e.call(
        FORMAT,
        &args![buffer, 0x103u32, FOOTSTEP_FORMAT, name, suffix],
    );
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(GET_SOUND, &args![handler, buffer]).u32()
}

// Translated from 00839ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00839ab0 - the /GS cookie check (00ec408c) is not translated.
/// `ImpactMixer::InitFootsteps` (engine map): once (`011dd85c`), fills the
/// footstep table: for each of the 13 materials and 10 suffixes the sound
/// `"FST<material><suffix>"`, else the same for the material's fallback,
/// else for the fallback's fallback (13 meaning none), else the default
/// footstep sound.
pub fn impact_mixer_init_footsteps(e: &mut Engine) {
    if e.global::<u8>(FOOTSTEPS_READY) != 0 {
        return;
    }
    e.set_global(FOOTSTEPS_READY, 1u8);
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        for material in 0..0xdu32 {
            for step in 0..0xau32 {
                let slot = FOOTSTEP_SOUNDS + material * 0x28 + step * 4;
                let suffix = e.mem.u32(FOOTSTEP_SUFFIXES + step * 4);
                let sound = footstep_sound(e, buffer, material, suffix);
                e.mem.set_u32(slot, sound);
                if e.mem.u32(slot) == 0 {
                    let fallback = e.mem.u32(FALLBACK_MATERIALS + material * 4);
                    if fallback != 0xd {
                        let suffix = e.mem.u32(FOOTSTEP_SUFFIXES + step * 4);
                        let sound = footstep_sound(e, buffer, fallback, suffix);
                        e.mem.set_u32(slot, sound);
                        let fallback = e.mem.u32(FALLBACK_MATERIALS + material * 4);
                        let second = e
                            .mem
                            .u32(FALLBACK_MATERIALS.wrapping_add(fallback.wrapping_mul(4)));
                        if e.mem.u32(slot) == 0 && second != 0xd {
                            let suffix = e.mem.u32(FOOTSTEP_SUFFIXES + step * 4);
                            let sound = footstep_sound(e, buffer, second, suffix);
                            e.mem.set_u32(slot, sound);
                        }
                    }
                }
                if e.mem.u32(slot) == 0 {
                    let handler = e.global::<u32>(DATA_HANDLER);
                    let sound = e.call(GET_SOUND, &args![handler, DEFAULT_FOOTSTEP]).u32();
                    e.mem.set_u32(slot, sound);
                }
            }
        }
    });
}

// Translated from 00839d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless already done (`00839da0`), marks the
/// impact table reset (`00839dc0(1)`) and clears its 20 records (two words
/// 0, the float `FLT_MAX`).
pub fn fn_00839d30(e: &mut Engine) {
    if fn_00839da0(e) {
        return;
    }
    fn_00839dc0(e, 1);
    for index in 0..0x14u32 {
        let record = IMPACT_RECORDS + index * 0x24;
        e.mem.set_u32(record, 0);
        e.mem.set_u32(record + 4, 0);
        let largest = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
        e.mem.set_u32(record + 8, largest);
    }
}

// Translated from 00839da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: bit 1 of the mixer's flags (`011dd878`).
pub fn fn_00839da0(e: &mut Engine) -> bool {
    e.global::<u32>(MIXER_FLAGS) & 2 != 0
}

// Translated from 00839dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): sets or clears bit 1 of the mixer's
/// flags (`011dd878`).
pub fn fn_00839dc0(e: &mut Engine, set: u8) {
    let flags = e.global::<u32>(MIXER_FLAGS);
    let flags = if set != 0 { flags | 2 } else { flags & !2 };
    e.set_global(MIXER_FLAGS, flags);
}

/// `FLD float; FCOMP double [constant]; TEST AH,5; JP`: true when the value
/// is below the constant (ordered).
fn below_constant(e: &Engine, value: f32, constant: u32) -> bool {
    x87_load32(value.to_bits()) < e.global::<f64>(constant)
}

/// `00403e20` on the setting, then `FLD value; FLD [setting]; FCOMPP; TEST
/// AH,0x41; JNZ`: true when the value is below the setting (ordered).
fn below_setting(e: &mut Engine, value: f32, setting: u32) -> bool {
    let at = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(at)) > x87_load32(value.to_bits())
}

/// The sound with the editor id at `name`.
fn sound(e: &mut Engine, name: u32) -> u32 {
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(GET_SOUND, &args![handler, name]).u32()
}

/// Mass below `1e-4`, below the medium and the large setting, or above:
/// four names.
fn by_mass_tiers(e: &mut Engine, mass: f32, medium: u32, large: u32, names: [u32; 4]) -> u32 {
    if below_constant(e, mass, SMALL_MASS_01032980) {
        return sound(e, names[0]);
    }
    if below_setting(e, mass, medium) {
        return sound(e, names[1]);
    }
    if below_setting(e, mass, large) {
        return sound(e, names[2]);
    }
    sound(e, names[3])
}

/// Mass below the first setting, below the second, or above: three names.
fn by_two_settings(e: &mut Engine, mass: f32, first: u32, second: u32, names: [u32; 3]) -> u32 {
    if below_setting(e, mass, first) {
        return sound(e, names[0]);
    }
    if below_setting(e, mass, second) {
        return sound(e, names[1]);
    }
    sound(e, names[2])
}

/// Mass below `6.0`, below the second constant, or above: three names.
fn by_two_constants(e: &mut Engine, mass: f32, second: u32, names: [u32; 3]) -> u32 {
    if below_constant(e, mass, MASS_6_01034208) {
        return sound(e, names[0]);
    }
    if below_constant(e, mass, second) {
        return sound(e, names[1]);
    }
    sound(e, names[2])
}

/// A light (`force` below `fCollisionSoundHeavyThreshold`) or a heavy name.
fn by_force(e: &mut Engine, force: f32, light: u32, heavy: u32) -> u32 {
    if below_setting(e, force, COLLISION_SOUND_HEAVY_THRESHOLD) {
        sound(e, light)
    } else {
        sound(e, heavy)
    }
}

// Translated from 00839e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactMixer::GetSoundForType` (engine map; cdecl): the collision sound
/// for `material` (0 to 0x23; any other gives 0), chosen by `mass` against
/// fixed limits and the `f*MassMin:Audio` settings and by `force` against
/// `fCollisionSoundHeavyThreshold:Audio`, as the data handler's sound with
/// that editor id (material 0x12 and 0x13: 0). The compiler's jump table
/// (`0083a878`) is the `match`.
pub fn impact_mixer_get_sound_for_type(
    e: &mut Engine,
    material: u32,
    mass: f32,
    force: f32,
) -> u32 {
    match material {
        0 | 15 => by_mass_tiers(
            e,
            mass,
            STONE_MEDIUM_MASS_MIN,
            STONE_LARGE_MASS_MIN,
            [0x0107_d724, 0x0107_d718, 0x0107_d708, 0x0107_d6fc],
        ),
        1 => by_force(e, force, 0x0107_d5a8, 0x0107_d594),
        2 => by_two_settings(
            e,
            mass,
            EARTH_MEDIUM_MASS_MIN,
            EARTH_LARGE_MASS_MIN,
            [0x0107_d6f0, 0x0107_d6e0, 0x0107_d6d4],
        ),
        3 => by_two_constants(
            e,
            mass,
            MASS_15_01015A38,
            [0x0107_d518, 0x0107_d508, 0x0107_d4fc],
        ),
        4 => by_two_constants(
            e,
            mass,
            MASS_70_01073568,
            [0x0107_d6c8, 0x0107_d6b8, 0x0107_d6ac],
        ),
        5 => {
            if below_setting(e, force, COLLISION_SOUND_HEAVY_THRESHOLD) {
                by_mass_tiers(
                    e,
                    mass,
                    METAL_MEDIUM_MASS_MIN,
                    METAL_LARGE_MASS_MIN,
                    [0x0107_d694, 0x0107_d67c, 0x0107_d664, 0x0107_d64c],
                )
            } else {
                by_mass_tiers(
                    e,
                    mass,
                    METAL_MEDIUM_MASS_MIN,
                    METAL_LARGE_MASS_MIN,
                    [0x0107_d634, 0x0107_d61c, 0x0107_d604, 0x0107_d5ec],
                )
            }
        }
        6 => sound(e, 0x0107_d584),
        7 => {
            if below_setting(e, force, COLLISION_SOUND_HEAVY_THRESHOLD) {
                by_two_settings(
                    e,
                    mass,
                    SKIN_MEDIUM_MASS_MIN,
                    SKIN_LARGE_MASS_MIN,
                    [0x0107_d574, 0x0107_d564, 0x0107_d554],
                )
            } else {
                by_two_settings(
                    e,
                    mass,
                    SKIN_MEDIUM_MASS_MIN,
                    SKIN_LARGE_MASS_MIN,
                    [0x0107_d544, 0x0107_d534, 0x0107_d524],
                )
            }
        }
        8 => by_two_constants(
            e,
            mass,
            MASS_15_01015A38,
            [0x0107_d5d4, 0x0107_d5c8, 0x0107_d5bc],
        ),
        9 => by_mass_tiers(
            e,
            mass,
            WOOD_MEDIUM_MASS_MIN,
            WOOD_LARGE_MASS_MIN,
            [0x0107_d5e0, 0x0107_d5d4, 0x0107_d5c8, 0x0107_d5bc],
        ),
        10 => sound(e, 0x0107_d4dc),
        11 => sound(e, 0x0107_d4c8),
        12 => sound(e, 0x0107_d4b4),
        13 => sound(e, 0x0107_d4f0),
        14 => sound(e, 0x0107_d23c),
        16 => by_mass_tiers(
            e,
            mass,
            METAL_MEDIUM_MASS_MIN,
            METAL_LARGE_MASS_MIN,
            [0x0107_d498, 0x0107_d47c, 0x0107_d460, 0x0107_d444],
        ),
        17 => by_mass_tiers(
            e,
            mass,
            METAL_MEDIUM_MASS_MIN,
            METAL_LARGE_MASS_MIN,
            [0x0107_d428, 0x0107_d40c, 0x0107_d3f0, 0x0107_d3d4],
        ),
        20 => by_force(e, force, 0x0107_d3bc, 0x0107_d3a4),
        21 => by_force(e, force, 0x0107_d38c, 0x0107_d374),
        22 => by_force(e, force, 0x0107_d35c, 0x0107_d344),
        23 => by_force(e, force, 0x0107_d338, 0x0107_d32c),
        24 => by_force(e, force, 0x0107_d320, 0x0107_d314),
        25 => by_force(e, force, 0x0107_d308, 0x0107_d2fc),
        26 => by_force(e, force, 0x0107_d2e8, 0x0107_d2d4),
        27 => by_force(e, force, 0x0107_d2c4, 0x0107_d2b4),
        28 => by_force(e, force, 0x0107_d2a0, 0x0107_d28c),
        29 => by_force(e, force, 0x0107_d27c, 0x0107_d26c),
        30 => sound(e, 0x0107_d25c),
        31 => sound(e, 0x0107_d24c),
        32 => sound(e, 0x0107_d22c),
        33 => sound(e, 0x0107_d224),
        34 => sound(e, 0x0107_d218),
        35 => sound(e, 0x0107_d208),
        // 18 jumps to the end and 19 stores 0: both answer 0, as does any
        // material above 0x23.
        _ => 0,
    }
}

// Translated from 0083a910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): word `index` of the table at
/// `011a17f8` for `index` from 1 to 0x23, else 0.
pub fn fn_0083a910(e: &mut Engine, index: i32) -> u32 {
    if index <= 0 || index >= 0x24 {
        return 0;
    }
    e.mem.u32(TABLE_011A17F8 + 4 * index as u32)
}

// Translated from 0083b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>, int, unsigned int>`
/// constructor (RTTI of the vtable it stores): `size` buckets (+4) of a
/// zeroed `00aa1070` block (+8), count 0 (+0xC). Returns `this`.
pub fn fn_0083b640(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.mem.set_u32(base + 4, size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.call(NI_MALLOC, &args![bytes]).u32();
    e.mem.set_u32(base + 8, buckets);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.mem.u32(base + 8);
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 0083b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b6b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around RemoveAll) is not translated.
/// The `NiTPointerMap<int, unsigned int>` destructor: its vtable, `RemoveAll`
/// (`00438af0`), then the base destructor (`0083b710`).
pub fn fn_0083b6b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_0083B5E0);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0083b710(e, this);
}

// Translated from 0083b710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..., int, unsigned int>` destructor: its vtable,
/// `RemoveAll` (`00438af0`), then the bucket array (+8) freed.
pub fn fn_0083b710(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(NI_FREE, &args![buckets]);
}

/// The TLS setting guard (`effectitem.rs`): its constructor, thiscall
/// (value, mode, source file, line), `RET 0x10`, and destructor.
const TLS_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const TLS_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// The source file name the guards pass (`0107d0a8`).
const SOURCE_FILE: u32 = 0x0107_d0a8;
/// `__RTDynamicCast` (engine map), cdecl (object, vftable offset, source
/// type, target type, is reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors `0083a930` casts with: the reference to an actor,
/// and the actor's base object to a form with the two impact sounds.
const SOURCE_TYPE_011841CC: u32 = 0x0118_41cc;
const TARGET_TYPE_011846D4: u32 = 0x0118_46d4;
const SOURCE_TYPE_01183108: u32 = 0x0118_3108;
const TARGET_TYPE_01183998: u32 = 0x0118_3998;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map; folded body),
/// thiscall on the actor.
const ACTOR_PROCESS_008D8520: u32 = 0x008d_8520;
/// The `BSSoundHandle` constructor and destructor, thiscall.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// Thiscall on an object (`run_0044d8c0.rs`).
const OBJECT_0044DDC0: u32 = 0x0044_ddc0;
/// Thiscall on the cast form: its sounds for types 9 and 10.
const SOUND_FOR_TYPE_9: u32 = 0x0052_2520;
const SOUND_FOR_TYPE_10: u32 = 0x0052_2540;
/// Whether a point is within hearing distance (`falloutaudio.rs`), cdecl
/// (object, x, y, z, scale).
const WITHIN_HEARING: u32 = 0x0082_eca0;
/// Thiscall on the actor (`animation.rs`): `AL`.
const ACTOR_FLAG_004997B0: u32 = 0x0049_97b0;
/// The player reference (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// Thiscall on a sound form (`run_005110e0.rs`): its file name.
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// `BSAudio::QInstance` and `BSAudio::GetSoundHandleByFilename` (engine
/// map; thiscall (result, file name, flags, form), `RET 0x10`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
/// The sound handle assignment (`extradatalist.rs`), thiscall (source),
/// `RET 4`.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle::SetPosition`, `SetObjectToFollow`, `SetVolume` and
/// `Play` (engine map).
const SOUND_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
const SOUND_SET_VOLUME: u32 = 0x00ad_89e0;
const SOUND_PLAY: u32 = 0x00ad_8830;
/// The volume used when the actor's flag is set (`010231a8`, `0.4`).
const QUIET_VOLUME_010231A8: u32 = 0x0102_31a8;

// Translated from 0083a930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083a930 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the guard and the sound handles) is not translated.
/// Unnamed in the engine map (cdecl): plays an actor's impact sound of
/// `kind` (9 or 10) at its position (vtable +0x1F4), under a TLS setting
/// guard (0xB). For an actor (`__RTDynamicCast`) with a process whose
/// object (vtable +0x148, then `0044ddc0`) casts to the sound holder, the
/// sound for `kind` (`00522520`/`00522540`) is played when within hearing
/// (`0082eca0`, scale 1.0): for the player as a 2D sound (flags
/// `0x80004101`), for others positioned (flags `0x4102`, the position and
/// the actor's vtable +0x1D0 to follow), at volume 0.4 (`010231a8`) when
/// `004997b0` answers, else 1.0. The setting at `011dd900`
/// (`fMaxFootstepDistance:Audio`) is read and not used.
pub fn fn_0083a930(e: &mut Engine, actor: u32, kind: u32) {
    e.with_stack(4, |e, guard| {
        e.call(
            TLS_GUARD_CONSTRUCT,
            &args![guard, 0xbu32, 1u32, SOURCE_FILE, 0x6a3u32],
        );
        let at = e.vcall(actor, 0x1f4, &args![]).u32();
        let position = [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)];
        let setting = e.call(SETTING_VALUE, &args![MAX_FOOTSTEP_DISTANCE]).u32();
        // `FLD [setting]; FSTP` into a local nothing reads.
        let _distance = e.mem.u32(setting);
        let reference = e
            .call(
                DYNAMIC_CAST,
                &args![
                    actor,
                    0u32,
                    SOURCE_TYPE_011841CC,
                    TARGET_TYPE_011846D4,
                    0u32
                ],
            )
            .u32();
        if reference != 0 && e.call(ACTOR_PROCESS_008D8520, &args![reference]).u32() != 0 {
            e.with_stack(0xc, |e, handle| {
                e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
                play_impact(e, actor, kind, reference, handle.addr(), position);
                e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            });
        }
        e.call(TLS_GUARD_DESTRUCT, &args![guard]);
    });
}

/// The body of `0083a930` once its sound handle exists.
fn play_impact(
    e: &mut Engine,
    actor: u32,
    kind: u32,
    reference: u32,
    handle: u32,
    position: [u32; 3],
) {
    let process = e.call(ACTOR_PROCESS_008D8520, &args![reference]).u32();
    let object = e.vcall(process, 0x148, &args![]).u32();
    let base = if object != 0 {
        e.call(OBJECT_0044DDC0, &args![object]).u32()
    } else {
        0
    };
    let holder = if base != 0 {
        e.call(
            DYNAMIC_CAST,
            &args![base, 0u32, SOURCE_TYPE_01183108, TARGET_TYPE_01183998, 0u32],
        )
        .u32()
    } else {
        0
    };
    if holder == 0 {
        return;
    }
    let sound = match kind {
        9 => e.call(SOUND_FOR_TYPE_9, &args![holder]).u32(),
        10 => e.call(SOUND_FOR_TYPE_10, &args![holder]).u32(),
        _ => 0,
    };
    if sound == 0
        || !e
            .call(
                WITHIN_HEARING,
                &args![
                    sound,
                    position[0],
                    position[1],
                    position[2],
                    1.0f32.to_bits()
                ],
            )
            .bool()
    {
        return;
    }
    let quiet = e.call(ACTOR_FLAG_004997B0, &args![reference]).u8();
    let player = reference == e.global::<u32>(PLAYER);
    let flags = if player { 0x8000_4101u32 } else { 0x4102 };
    e.with_stack(0xc, |e, temporary| {
        let name = e.call(SOUND_FILE_NAME, &args![sound]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(
                GET_SOUND_HANDLE_BY_FILENAME,
                &args![audio, temporary, name, flags, sound],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
    });
    if !player {
        let [x, y, z] = position.map(x87_quiet_bits);
        e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
        let followed = e.vcall(actor, 0x1d0, &args![]).u32();
        e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![handle, followed]);
    }
    let volume = if quiet != 0 {
        x87_quiet_bits(e.global::<u32>(QUIET_VOLUME_010231A8))
    } else {
        1.0f32.to_bits()
    };
    e.call(SOUND_SET_VOLUME, &args![handle, volume]);
    e.call(SOUND_PLAY, &args![handle, 0u32]);
}

/// `ImpactMixer::PlayCollisionSound`'s function-local timer (`011ddd24`, its
/// guard word `011ddd3c`), constructed by `0044fae0` with `00fd8c40` given
/// to `_atexit`; started with a name (`004538a0`) and stopped (`004538c0`).
const COLLISION_TIMER: u32 = 0x011d_dd24;
const COLLISION_TIMER_GUARD: u32 = 0x011d_dd3c;
const TIMER_CONSTRUCT: u32 = 0x0044_fae0;
const TIMER_DESTRUCT_AT_EXIT: u32 = 0x00fd_8c40;
const ATEXIT: u32 = 0x00ec_658f;
const TIMER_START: u32 = 0x0045_38a0;
const TIMER_STOP: u32 = 0x0045_38c0;
/// `"ImpactMixer::PlayCollisionSound"`.
const COLLISION_TIMER_NAME: u32 = 0x0107_d140;
/// `bhkRigidBody::GetDynamicMass` (engine map), thiscall: `ST0`.
const GET_DYNAMIC_MASS: u32 = 0x0062_ab20;
/// The `float` mass used for a missing body (`0101712c`).
const MISSING_BODY_MASS: u32 = 0x0101_712c;
/// Thiscall on a body (out word), `RET 4`, and the object's kind
/// (`modelloader.rs`); kind 0x19 makes the collision a casing's.
const BODY_OBJECT_0043B4F0: u32 = 0x0043_b4f0;
const OBJECT_KIND_0043B4D0: u32 = 0x0043_b4d0;
/// `"PHYCasingShotgun"`, `"PHYCasingPistol"`, `"PHYCasingRifle"`.
const CASING_SHOTGUN: u32 = 0x0107_d120;
const CASING_PISTOL: u32 = 0x0107_d110;
const CASING_RIFLE: u32 = 0x0107_d100;
/// Cdecl on three floats (`run_0040ea20.rs`): `AL`.
const FLOAT_TEST_0040EB10: u32 = 0x0040_eb10;
/// The two `float`s passed with it (`010162c0`, 2.0, and `0101622c`, 0.25).
const CASING_LIMIT_010162C0: u32 = 0x0101_62c0;
const CASING_LIMIT_0101622C: u32 = 0x0101_622c;
/// `double` constants of the collision computations.
const SCALE_0107D138: u32 = 0x0107_d138;
const DIVISOR_010179E0: u32 = 0x0101_79e0;
const FACTOR_0101DB80: u32 = 0x0101_db80;
const ONE_DOUBLE: u32 = 0x0101_2070;
const STRENGTH_DIVISOR_0106B178: u32 = 0x0106_b178;
const ATTENUATION_SCALE_0102ED48: u32 = 0x0102_ed48;
const FREQUENCY_OFFSET_0107D0F8: u32 = 0x0107_d0f8;
const FREQUENCY_DIVISOR_0107D0F0: u32 = 0x0107_d0f0;
/// The clock (`011f6394`, `00825c00`: milliseconds).
const CLOCK: u32 = 0x011f_6394;
const CLOCK_NOW: u32 = 0x0082_5c00;
/// The map of last collision times by key (`011dd948`) and its methods.
const COLLISION_TIMES: u32 = 0x011d_d948;
const MAP_GET_AT: u32 = 0x0085_3130;
const MAP_SET_AT: u32 = 0x0084_4700;
const MAP_GET_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// The integer setting with the minimum time between a key's collision
/// sounds (`011dd968`, its value through `0043d4d0`).
const COLLISION_INTERVAL: u32 = 0x011d_d968;
const SETTING_INT: u32 = 0x0043_d4d0;
/// The sound form's numeric id (`run_0084e3a0.rs`), and
/// `BSAudio::GetSoundHandleByNumericID` (engine map; thiscall (result, id,
/// flags), `RET 0xC`).
const SOUND_ID: u32 = 0x0084_e3a0;
const GET_SOUND_HANDLE_BY_ID: u32 = 0x00ad_73b0;
/// `BSSoundHandle::SetPriority`, `SetStaticAttenuation` and
/// `SetModFrequency` (engine map).
const SOUND_SET_PRIORITY: u32 = 0x00ad_9030;
const SOUND_SET_STATIC_ATTENUATION: u32 = 0x00ad_89b0;
const SOUND_SET_MOD_FREQUENCY: u32 = 0x00ad_8a90;
/// The last collision position (`011dd958`, three floats).
const LAST_COLLISION_POSITION: u32 = 0x011d_d958;

/// A body's dynamic mass, or the `float` at `0101712c` without a body.
fn body_mass(e: &mut Engine, body: u32) -> u32 {
    if body != 0 {
        x87_store32(e.call(GET_DYNAMIC_MASS, &args![body]).f64())
    } else {
        x87_quiet_bits(e.global::<u32>(MISSING_BODY_MASS))
    }
}

/// Whether the body's object is of kind 0x19.
fn casing_body(e: &mut Engine, body: u32) -> bool {
    e.with_stack(4, |e, out| {
        let object = e.call(BODY_OBJECT_0043B4F0, &args![body, out]).u32();
        e.call(OBJECT_KIND_0043B4D0, &args![object]).u32() == 0x19
    })
}

/// A sound handle for `sound` (by its numeric id, `flags`) assigned to the
/// handle at `handle`.
fn handle_by_id(e: &mut Engine, handle: u32, sound: u32, flags: u32) {
    e.with_stack(0xc, |e, temporary| {
        let id = e.call(SOUND_ID, &args![sound]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(GET_SOUND_HANDLE_BY_ID, &args![audio, temporary, id, flags])
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
    });
}

/// Plays the handle at `handle` at the collision's position with the
/// attenuation and frequency computed for it, priority 0xFF.
fn play_at(e: &mut Engine, handle: u32, collision: u32, attenuation: u32, frequency: u32) {
    e.call(SOUND_SET_PRIORITY, &args![handle, 0xffu32]);
    let [x, y, z] = [0, 4, 8].map(|offset| x87_quiet_bits(e.mem.u32(collision + offset)));
    e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
    e.call(
        SOUND_SET_STATIC_ATTENUATION,
        &args![handle, attenuation & 0xffff],
    );
    e.call(
        SOUND_SET_MOD_FREQUENCY,
        &args![handle, x87_quiet_bits(frequency)],
    );
    e.call(SOUND_PLAY, &args![handle, 0u32]);
}

// Translated from 00837550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00837550 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the guard, the timer's construction and the sound handles) is not translated.
/// `ImpactMixer::PlayCollisionSound` (engine map; cdecl). The collision
/// record holds the position (+0, three floats), the strength (+0xC), the
/// two materials (+0x14, +0x15, signed bytes), a key (+0x18) and the two
/// bodies (+0x1C, +0x20). Nothing for a body colliding with itself.
/// Otherwise, timed (the function-local timer at `011ddd24`, under a TLS
/// setting guard 0xB): each side's sound is `GetSoundForType(material, its
/// body's mass, strength)`, or, when a body is a casing (kind 0x19), a
/// casing sound by the larger mass (shotgun below 1.0, pistol when
/// `0040eb10(mass, 2.0, 0.25)` answers, else rifle; flag 0x80). Unless neither sound is within
/// hearing, or the key collided less than the interval setting ago (the
/// times map `011dd948`, which then records now), each sound (not for two
/// material-7 sides) is played at the position with the static attenuation
/// `(1 - min(strength / 400, 1)) * 3000` (truncated) and the frequency 1.0
/// below 1500, else `(attenuation - 4500) / -3000`. The position is kept
/// (`011dd958`) and map entries older than four intervals are dropped (all
/// of them after 0x100 steps).
pub fn impact_mixer_play_collision_sound(e: &mut Engine, collision: u32) {
    e.with_stack(4, |e, guard| {
        e.call(
            TLS_GUARD_CONSTRUCT,
            &args![guard, 0xbu32, 1u32, SOURCE_FILE, 0x234u32],
        );
        let first_body = e.mem.u32(collision + 0x1c);
        if first_body == e.mem.u32(collision + 0x20) && first_body != 0 {
            e.call(TLS_GUARD_DESTRUCT, &args![guard]);
            return;
        }
        if e.global::<u32>(COLLISION_TIMER_GUARD) & 1 == 0 {
            let flags = e.global::<u32>(COLLISION_TIMER_GUARD);
            e.set_global(COLLISION_TIMER_GUARD, flags | 1);
            e.call(TIMER_CONSTRUCT, &args![COLLISION_TIMER]);
            e.call(ATEXIT, &args![TIMER_DESTRUCT_AT_EXIT]);
        }
        e.call(TIMER_START, &args![COLLISION_TIMER, COLLISION_TIMER_NAME]);
        collision_sound_body(e, collision);
        e.call(TLS_GUARD_DESTRUCT, &args![guard]);
    });
}

/// `PlayCollisionSound` once the guard is set and the timer started; it
/// stops the timer on every way out.
fn collision_sound_body(e: &mut Engine, collision: u32) {
    let strength = e.mem.u32(collision + 0xc);
    // Two values the code computes and does not use.
    let _scaled = x87_store32(x87_mul(
        x87_load32(strength),
        e.global::<f64>(SCALE_0107D138),
    ));
    let divided = x87_store32(x87_div(
        x87_load32(strength),
        e.global::<f64>(DIVISOR_010179E0),
    ));
    let first_material = e.mem.i8(collision + 0x14);
    let second_material = e.mem.i8(collision + 0x15);
    let _divided = if first_material == 0x24 && second_material == 6 {
        x87_store32(x87_mul(
            x87_load32(divided),
            e.global::<f64>(FACTOR_0101DB80),
        ))
    } else {
        divided
    };
    let mut flags = 0u32;
    let first_body = e.mem.u32(collision + 0x1c);
    let first_mass = body_mass(e, first_body);
    let second_body = e.mem.u32(collision + 0x20);
    let second_mass = body_mass(e, second_body);
    // The first body is asked first; the second only when the first is not a
    // casing.
    let casing = (first_body != 0 && casing_body(e, first_body))
        || (second_body != 0 && casing_body(e, second_body));
    let first_sound;
    let second_sound;
    if !casing {
        first_sound = impact_mixer_get_sound_for_type(
            e,
            i32::from(first_material) as u32,
            f32::from_bits(first_mass),
            f32::from_bits(x87_quiet_bits(strength)),
        );
        second_sound = impact_mixer_get_sound_for_type(
            e,
            i32::from(second_material) as u32,
            f32::from_bits(second_mass),
            f32::from_bits(x87_quiet_bits(strength)),
        );
    } else {
        flags = 0x80;
        // `FLD first; FLD second; FCOMPP; TEST AH,5; JP`: the first when
        // the second is below it, else the second.
        let mass = if x87_load32(second_mass) < x87_load32(first_mass) {
            first_mass
        } else {
            second_mass
        };
        first_sound = if x87_load32(mass) < e.global::<f64>(ONE_DOUBLE) {
            sound(e, CASING_SHOTGUN)
        } else {
            let first_limit = x87_quiet_bits(e.global::<u32>(CASING_LIMIT_010162C0));
            let second_limit = x87_quiet_bits(e.global::<u32>(CASING_LIMIT_0101622C));
            if e.call(FLOAT_TEST_0040EB10, &args![mass, first_limit, second_limit])
                .bool()
            {
                sound(e, CASING_PISTOL)
            } else {
                sound(e, CASING_RIFLE)
            }
        };
        second_sound = 0;
    }
    let position = [0, 4, 8].map(|offset| e.mem.u32(collision + offset));
    let audible = |e: &mut Engine, sound: u32| {
        e.call(
            WITHIN_HEARING,
            &args![
                sound,
                position[0],
                position[1],
                position[2],
                1.0f32.to_bits()
            ],
        )
        .bool()
    };
    if !audible(e, first_sound) && !audible(e, second_sound) {
        e.call(TIMER_STOP, &args![COLLISION_TIMER]);
        return;
    }
    let stopped = e.with_stack(0x18, |e, handles| {
        let first_handle = handles.addr();
        let second_handle = handles.addr() + 0xc;
        e.call(SOUND_HANDLE_CONSTRUCT, &args![first_handle]);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![second_handle]);
        let now = e.call(CLOCK_NOW, &args![CLOCK]).u32();
        let recent = e.with_stack(4, |e, last| {
            let key = e.mem.u32(collision + 0x18);
            if !e
                .call(MAP_GET_AT, &args![COLLISION_TIMES, key, last])
                .bool()
            {
                return false;
            }
            let since = now.wrapping_sub(e.mem.u32(last.addr()));
            let interval = e.call(SETTING_INT, &args![COLLISION_INTERVAL]).u32();
            since < e.mem.u32(interval)
        });
        if recent {
            e.call(TIMER_STOP, &args![COLLISION_TIMER]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![second_handle]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![first_handle]);
            return true;
        }
        let key = e.mem.u32(collision + 0x18);
        e.call(MAP_SET_AT, &args![COLLISION_TIMES, key, now]);
        let both_soft =
            |e: &Engine| e.mem.i8(collision + 0x14) == 7 && e.mem.i8(collision + 0x15) == 7;
        if first_sound != 0 && !both_soft(e) {
            handle_by_id(e, first_handle, first_sound, flags | 2 | 0x100 | 0x4000);
        }
        if second_sound != 0 && !both_soft(e) {
            handle_by_id(e, second_handle, second_sound, flags | 2 | 0x100 | 0x4000);
        }
        let divisor = e.global::<f64>(STRENGTH_DIVISOR_0106B178);
        let one = e.global::<f64>(ONE_DOUBLE);
        let ratio = x87_div(x87_load32(e.mem.u32(collision + 0xc)), divisor);
        // `FCOMP 1.0; TEST AH,0x41; JNZ`: 1.0 only when strictly above.
        let clamped = if ratio > one {
            1.0
        } else {
            x87_div(x87_load32(e.mem.u32(collision + 0xc)), divisor)
        };
        let scaled = x87_mul(
            x87_sub(1.0, clamped),
            e.global::<f64>(ATTENUATION_SCALE_0102ED48),
        );
        let attenuation = x87_fistp64_chop(X87Ext::from(scaled)) as u32;
        let frequency = if attenuation < 0x5dc {
            1.0
        } else {
            x87_div(
                x87_sub(
                    f64::from(attenuation),
                    e.global::<f64>(FREQUENCY_OFFSET_0107D0F8),
                ),
                e.global::<f64>(FREQUENCY_DIVISOR_0107D0F0),
            )
        };
        let frequency = x87_store32(frequency);
        if first_sound != 0 {
            play_at(e, first_handle, collision, attenuation, frequency);
        }
        if second_sound != 0 {
            play_at(e, second_handle, collision, attenuation, frequency);
        }
        for (index, word) in position.iter().enumerate() {
            e.mem
                .set_u32(LAST_COLLISION_POSITION + 4 * index as u32, *word);
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![second_handle]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![first_handle]);
        false
    });
    if stopped {
        return;
    }
    let interval = e.call(SETTING_INT, &args![COLLISION_INTERVAL]).u32();
    let limit = e.mem.u32(interval) << 2;
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        let first = e
            .call(MAP_GET_FIRST_POSITION, &args![COLLISION_TIMES])
            .u32();
        e.mem.set_u32(position, first);
        let now = e.call(CLOCK_NOW, &args![CLOCK]).u32();
        let mut steps = 0u32;
        while e.mem.u32(position) != 0 {
            steps += 1;
            e.call(MAP_GET_NEXT, &args![COLLISION_TIMES, position, key, value]);
            if now.wrapping_sub(e.mem.u32(value)) as i32 > limit as i32 {
                let old = e.mem.u32(key);
                e.call(MAP_REMOVE_AT, &args![COLLISION_TIMES, old]);
                let first = e
                    .call(MAP_GET_FIRST_POSITION, &args![COLLISION_TIMES])
                    .u32();
                e.mem.set_u32(position, first);
            }
            if steps > 0x100 {
                e.call(MAP_REMOVE_ALL, &args![COLLISION_TIMES]);
                e.mem.set_u32(position, 0);
            }
        }
    });
    e.call(TIMER_STOP, &args![COLLISION_TIMER]);
}

/// The 20 sound records of 0x24 bytes (`011dd9e8`): two sound handles (+0,
/// +0xC), the actor (+0x18), the time (+0x1C) and the squared distance
/// (+0x20).
const SOUND_RECORDS: u32 = 0x011d_d9e8;
/// The listener position `0083ac30` starts from (`011f426c`, three words).
const DEFAULT_SOUND_POSITION: u32 = 0x011f_426c;
/// Thiscall on the player (`tesobjectweap.rs`): `AL`.
const PLAYER_FLAG_00524D10: u32 = 0x0052_4d10;
/// Thiscall on the player: its current cell or location (`run_008d6f10.rs`),
/// and two tests on it (`run_00425fd0.rs`, `tes.rs`).
const PLAYER_PLACE_008D6F30: u32 = 0x008d_6f30;
const PLACE_TEST_00425FD0: u32 = 0x0042_5fd0;
const PLACE_TEST_00454B10: u32 = 0x0045_4b10;
/// `fWeaponInteriorNearVolumeMod`, `fWeaponInteriorFarVolumeMod`,
/// `fWeaponInteriorNearAttenuationMod` and
/// `fWeaponInteriorFarAttenuationMod` (from the static initializers).
const INTERIOR_NEAR_VOLUME: u32 = 0x011d_d008;
const INTERIOR_FAR_VOLUME: u32 = 0x011d_d034;
const INTERIOR_NEAR_ATTENUATION: u32 = 0x011d_d0ac;
const INTERIOR_FAR_ATTENUATION: u32 = 0x011d_cff4;
/// `ItemChange::HasModEffectActive_ov2` (engine map), thiscall (effect),
/// `RET 4`.
const HAS_MOD_EFFECT: u32 = 0x004b_da70;
/// Thiscall on the weapon (word), `RET 4`: its sounds.
const WEAPON_SOUND_00522470: u32 = 0x0052_2470;
const WEAPON_SOUND_005215A0: u32 = 0x0052_15a0;
const WEAPON_SOUND_00522440: u32 = 0x0052_2440;
/// Thiscall on an object: the address of its position (`tes_p2.rs`).
const OBJECT_POSITION: u32 = 0x0045_bb80;
/// Thiscall on the audio instance (out point), `RET 4` (`bsaudio.rs`): the
/// listener position.
const LISTENER_POSITION: u32 = 0x00ad_78e0;
/// The point difference (`multiboundmarkerdata.rs`), thiscall on the first
/// (result, second), `RET 8`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `BSSoundHandle::IsValid`, `Stop` and `SetMinMax` (engine map).
const SOUND_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_STOP: u32 = 0x00ad_88f0;
const SOUND_SET_MIN_MAX: u32 = 0x00ad_8be0;
/// The game time in milliseconds (`tes_p2.rs`).
const GAME_TIME: u32 = 0x0045_7fe0;
/// Thiscall on a sound form: two bytes (`falloutaudio.rs`,
/// `tesobjectcell_p3.rs`).
const SOUND_FIRST_VALUE_0082D660: u32 = 0x0082_d660;
const SOUND_SECOND_VALUE_00553B90: u32 = 0x0055_3b90;
/// `100.0` and `5.0` (`double`).
const HUNDRED_01017A40: u32 = 0x0101_7a40;
const FIVE_01020998: u32 = 0x0102_0998;
/// `006815c0` (returns `this`), called on a local point.
const RETURN_THIS: u32 = 0x0068_15c0;

/// `00403e20` on a setting, then its value's bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let at = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(at))
}

/// The three words at `at`.
fn read_point(e: &Engine, at: u32) -> [u32; 3] {
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

/// The mutable state `0083ac30` keeps in its locals.
struct WeaponSound {
    position: [u32; 3],
    near_volume: u32,
    far_volume: u32,
    near_attenuation: u32,
    far_attenuation: u32,
    first_byte: u8,
    second_byte: u8,
}

// Translated from 0083ac30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083ac30 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the two local sound handles and the temporary handles) is not translated.
/// Unnamed in the engine map (cdecl): plays a weapon's sounds for `actor`
/// once the record table is reset (`00839da0`). Inside (the player's place
/// passes `00425fd0` and not `00454b10`) the `fWeaponInterior*` settings
/// scale the volumes and attenuations. With `first_person`, the weapon's
/// `00522470(0)` sound alone is used (flags 0x20181), else its `005215a0`
/// and `00522440` sounds (flags 0x60182 when the player's `00524d10` is
/// clear, else 0x20182), each kept when within hearing from the position of
/// `object` (or the actor's followed object). A record of the 20 at
/// `011dd9e8` is taken (the oldest free one, else the actor's own, else the
/// farthest one beyond this distance for other actors when first person)
/// and its two handles get the sounds (by file name), the followed object,
/// priority 1, the position, the volume and play; outside first person the
/// handles also get min/max distances from the sounds' bytes and the
/// position again.
pub fn fn_0083ac30(e: &mut Engine, weapon: u32, actor: u32, object: u32, first_person: u8) {
    if !fn_00839da0(e) {
        return;
    }
    e.with_stack(0x18, |e, handles| {
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handles]);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handles.addr() + 0xc]);
        weapon_sound_body(e, weapon, actor, object, first_person);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handles.addr() + 0xc]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handles]);
    });
}

/// `0083ac30` between its two local handles' construction and destruction.
fn weapon_sound_body(e: &mut Engine, weapon: u32, actor: u32, object: u32, first_person: u8) {
    let mut state = WeaponSound {
        position: read_point(e, DEFAULT_SOUND_POSITION),
        near_volume: 1.0f32.to_bits(),
        far_volume: 1.0f32.to_bits(),
        near_attenuation: 1.0f32.to_bits(),
        far_attenuation: 1.0f32.to_bits(),
        first_byte: 0,
        second_byte: 0,
    };
    let mut single = false;
    let player = e.global::<u32>(PLAYER);
    let player_clear = !e.call(PLAYER_FLAG_00524D10, &args![player]).bool();
    let mut modded = false;
    e.with_stack(0xc, |e, local| {
        e.call(RETURN_THIS, &args![local]);
    });
    let player = e.global::<u32>(PLAYER);
    if e.call(PLAYER_PLACE_008D6F30, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let place = e.call(PLAYER_PLACE_008D6F30, &args![player]).u32();
        if e.call(PLACE_TEST_00425FD0, &args![place]).bool() {
            let player = e.global::<u32>(PLAYER);
            let place = e.call(PLAYER_PLACE_008D6F30, &args![player]).u32();
            if !e.call(PLACE_TEST_00454B10, &args![place]).bool() {
                state.near_volume = setting_bits(e, INTERIOR_NEAR_VOLUME);
                state.far_volume = setting_bits(e, INTERIOR_FAR_VOLUME);
                state.near_attenuation = setting_bits(e, INTERIOR_NEAR_ATTENUATION);
                state.far_attenuation = setting_bits(e, INTERIOR_FAR_ATTENUATION);
            }
        }
    }
    if first_person != 0 {
        let player = e.global::<u32>(PLAYER);
        if e.call(ACTOR_PROCESS_008D8520, &args![player]).u32() != 0 {
            let process = e.call(ACTOR_PROCESS_008D8520, &args![player]).u32();
            if e.vcall(process, 0x148, &args![]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS_008D8520, &args![player]).u32();
                let item = e.vcall(process, 0x148, &args![]).u32();
                if item != 0
                    && (e.call(HAS_MOD_EFFECT, &args![item, 0xbu32]).bool()
                        || e.call(HAS_MOD_EFFECT, &args![item, 0x10u32]).bool())
                {
                    modded = true;
                }
            }
        }
    }
    if weapon == 0 {
        return;
    }
    if first_person != 0 && e.call(WEAPON_SOUND_00522470, &args![weapon, 0u32]).u32() != 0 {
        single = true;
    }
    let flags = if single {
        0x0002_0181u32
    } else if first_person != 0 && player_clear {
        0x0006_0182
    } else {
        0x0002_0182
    };
    if first_person == 0 && object != 0 {
        let at = e.call(OBJECT_POSITION, &args![object]).u32();
        state.position = read_point(e, at);
    } else if e.vcall(actor, 0x1d0, &args![]).u32() != 0 {
        let followed = e.vcall(actor, 0x1d0, &args![]).u32();
        let at = e.call(OBJECT_POSITION, &args![followed]).u32();
        state.position = read_point(e, at);
    }
    let modded = u32::from(modded);
    if e.call(WEAPON_SOUND_005215A0, &args![weapon, modded]).u32() == 0 {
        return;
    }
    let mut first_sound;
    let mut second_sound = 0u32;
    if single {
        first_sound = e.call(WEAPON_SOUND_00522470, &args![weapon, modded]).u32();
    } else {
        first_sound = e.call(WEAPON_SOUND_005215A0, &args![weapon, modded]).u32();
        second_sound = e.call(WEAPON_SOUND_00522440, &args![weapon, modded]).u32();
    }
    let [x, y, z] = state.position;
    let first_heard = e
        .call(
            WITHIN_HEARING,
            &args![first_sound, x, y, z, x87_quiet_bits(state.near_attenuation)],
        )
        .bool();
    let second_heard = e
        .call(
            WITHIN_HEARING,
            &args![second_sound, x, y, z, x87_quiet_bits(state.far_attenuation)],
        )
        .bool();
    if !first_heard && !second_heard {
        return;
    }
    if !first_heard {
        first_sound = 0;
    }
    if !second_heard {
        second_sound = 0;
    }
    let distance = e.with_stack(0x18, |e, locals| {
        let listener = locals.addr();
        let difference = locals.addr() + 0xc;
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let at = e.call(LISTENER_POSITION, &args![audio, listener]).u32();
        e.with_stack(0xc, |e, position| {
            for (index, word) in state.position.iter().enumerate() {
                e.mem.set_u32(position.addr() + 4 * index as u32, *word);
            }
            let result = e
                .call(POINT_SUBTRACT, &args![position, difference, at])
                .u32();
            let [dx, dy, dz] = read_point(e, result);
            let square = |word: u32| x87_mul(x87_load32(word), x87_m32(word));
            let sum = x87_add(x87_add(square(dx), square(dy)), square(dz));
            x87_store32(sum)
        })
    });
    let record = choose_record(e, actor, first_person, distance);
    if record == 0 {
        return;
    }
    if first_sound != 0 {
        e.call(SOUND_SET_VOLUME, &args![record, 0.0f32.to_bits()]);
        e.call(SOUND_STOP, &args![record]);
        handle_by_name(e, record, first_sound, flags);
        state.first_byte = e.call(SOUND_FIRST_VALUE_0082D660, &args![first_sound]).u8();
        state.second_byte = e
            .call(SOUND_SECOND_VALUE_00553B90, &args![first_sound])
            .u8();
    }
    if second_sound != 0 {
        e.call(SOUND_SET_VOLUME, &args![record + 0xc, 0.0f32.to_bits()]);
        e.call(SOUND_STOP, &args![record + 0xc]);
        handle_by_name(e, record + 0xc, second_sound, flags);
        // The two bytes of the second sound are read and not used.
        e.call(SOUND_FIRST_VALUE_0082D660, &args![second_sound]);
        e.call(SOUND_SECOND_VALUE_00553B90, &args![second_sound]);
    }
    if first_person == 0 && object != 0 {
        e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![record, object]);
        e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![record + 0xc, object]);
    } else if e.vcall(actor, 0x1d0, &args![]).u32() != 0 && (first_person == 0 || !player_clear) {
        let followed = e.vcall(actor, 0x1d0, &args![]).u32();
        e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![record, followed]);
        let followed = e.vcall(actor, 0x1d0, &args![]).u32();
        e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![record + 0xc, followed]);
    }
    let position = state.position.map(x87_quiet_bits);
    for (handle, sound, volume) in [
        (record, first_sound, state.near_volume),
        (record + 0xc, second_sound, state.far_volume),
    ] {
        if sound != 0 {
            e.call(SOUND_SET_PRIORITY, &args![handle, 1u32]);
            e.call(
                SOUND_SET_POSITION,
                &args![handle, position[0], position[1], position[2]],
            );
            e.call(SOUND_SET_VOLUME, &args![handle, x87_quiet_bits(volume)]);
            e.call(SOUND_PLAY, &args![handle, 0u32]);
        }
    }
    if single {
        return;
    }
    let hundred = e.global::<f64>(HUNDRED_01017A40);
    let five = e.global::<f64>(FIVE_01020998);
    for handle in [record, record + 0xc] {
        let maximum = x87_store32(x87_mul(
            x87_mul(
                f64::from(state.second_byte),
                x87_m32(state.near_attenuation),
            ),
            hundred,
        ));
        let minimum = x87_store32(x87_mul(f64::from(state.first_byte), five));
        e.call(
            SOUND_SET_MIN_MAX,
            &args![handle, x87_quiet_bits(minimum), x87_quiet_bits(maximum)],
        );
    }
    for handle in [record, record + 0xc] {
        e.call(
            SOUND_SET_POSITION,
            &args![handle, position[0], position[1], position[2]],
        );
    }
}

/// A sound handle for `sound` by its file name (`flags`, the sound)
/// assigned to the handle at `handle`.
fn handle_by_name(e: &mut Engine, handle: u32, sound: u32, flags: u32) {
    e.with_stack(0xc, |e, temporary| {
        let name = e.call(SOUND_FILE_NAME, &args![sound]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(
                GET_SOUND_HANDLE_BY_FILENAME,
                &args![audio, temporary, name, flags, sound],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
    });
}

/// `0083ac30`'s record choice: walks the 20 records (stopping at the
/// actor's own one outside first person), keeping the actor's own, the
/// oldest (lowest time, the last of equals), and in first person the
/// farthest of the other actors' records beyond `distance`. The oldest is
/// taken when its first handle is not valid (actor, time, distance set),
/// else the actor's own (time and distance), else the farthest (actor,
/// time, distance). 0 when none.
fn choose_record(e: &mut Engine, actor: u32, first_person: u8, distance: u32) -> u32 {
    let mut own = 0u32;
    let mut oldest = 0u32;
    let mut farthest = 0u32;
    for index in 0..0x14u32 {
        let record = SOUND_RECORDS + index * 0x24;
        if e.mem.u32(record + 0x18) == actor {
            own = record;
            if first_person == 0 {
                break;
            }
        }
        if oldest == 0 || e.mem.u32(record + 0x1c) <= e.mem.u32(oldest + 0x1c) {
            oldest = record;
        }
        if first_person == 0 || e.mem.u32(record + 0x18) == e.global::<u32>(PLAYER) {
            continue;
        }
        if farthest == 0 {
            farthest = record;
        } else {
            let here = x87_load32(e.mem.u32(record + 0x20));
            if here > x87_load32(e.mem.u32(farthest + 0x20)) && here > x87_load32(distance) {
                farthest = record;
            }
        }
    }
    let mut valid = true;
    if oldest != 0 {
        valid = e.call(SOUND_IS_VALID, &args![oldest]).bool();
    }
    if oldest != 0 && !valid {
        e.mem.set_u32(oldest + 0x18, actor);
        let now = e.call(GAME_TIME, &args![]).u32();
        e.mem.set_u32(oldest + 0x1c, now);
        e.mem.set_u32(oldest + 0x20, x87_quiet_bits(distance));
        return oldest;
    }
    if own != 0 {
        let now = e.call(GAME_TIME, &args![]).u32();
        e.mem.set_u32(own + 0x1c, now);
        e.mem.set_u32(own + 0x20, x87_quiet_bits(distance));
        return own;
    }
    if farthest != 0 {
        e.mem.set_u32(farthest + 0x18, actor);
        let now = e.call(GAME_TIME, &args![]).u32();
        e.mem.set_u32(farthest + 0x1c, now);
        e.mem.set_u32(farthest + 0x20, x87_quiet_bits(distance));
        return farthest;
    }
    0
}

/// The `TES` object (`011dea10`) and its methods: the interior cell
/// (`005f36f0`, engine map `ActorMover::GetPreferredMoveMode` on folded
/// code) and `TES::GetWorldSpace` (`004fd3e0`).
const TES: u32 = 0x011d_ea10;
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESDataHandler::GetCellFromWorldCoord` (engine map), thiscall (x, y,
/// world space, word), `RET 0x10`.
const GET_CELL_FROM_WORLD_COORD: u32 = 0x0046_1bc0;
/// Thiscall on a cell (`tes.rs`): `AL`.
const CELL_TEST_004518E0: u32 = 0x0045_18e0;
/// Thiscall on the actor (`tesconditionfunctions.rs`): `AL`.
const ACTOR_TEST_005A2030: u32 = 0x005a_2030;
/// `TESObjectREFR::GetRelevantWaterHeight` (engine map): `ST0`.
const GET_RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// `TESObjectCELL::GetLand`, `TESObjectLAND::GetCoordData` (thiscall
/// (data, point, word), `RET 0xC`), the land texture lookup (`tesland.rs`,
/// thiscall (byte, word), `RET 8`), its material (`00541570`) and
/// `TESObjectCELL::GetLandHeight` (thiscall (point, out float), `RET 8`).
const CELL_GET_LAND: u32 = 0x0054_6fb0;
const LAND_COORD_DATA_CONSTRUCT: u32 = 0x0053_a510;
const LAND_GET_COORD_DATA: u32 = 0x0053_b550;
const LAND_TEXTURE_AT: u32 = 0x0053_a630;
const LAND_TEXTURE_MATERIAL: u32 = 0x0054_1570;
const CELL_GET_LAND_HEIGHT: u32 = 0x0055_47c0;
/// `RandomFloat` (engine map), cdecl (low, high): `ST0`.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// The random volume's range top (`0101e2bc`, `0.1`) and the volume it is
/// taken from (`0107d1b8`, `0.95`).
const VOLUME_SPREAD_0101E2BC: u32 = 0x0101_e2bc;
const VOLUME_BASE_0107D1B8: u32 = 0x0107_d1b8;
/// `"FSTBabyLand"`.
const BABY_LAND: u32 = 0x0107_d1c0;
/// The landing sounds: entry 8 of each footstep material row.
const LANDING_SOUNDS: u32 = 0x011d_d670;

/// The world cell under `(x, y)` (the data handler, in the current world
/// space).
fn world_cell(e: &mut Engine, x: u32, y: u32) -> u32 {
    let tes = e.global::<u32>(TES);
    let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(
        GET_CELL_FROM_WORLD_COORD,
        &args![handler, x87_quiet_bits(x), x87_quiet_bits(y), world, 0u32],
    )
    .u32()
}

/// The material of the land under `point` (the cell's land, its coordinate
/// data, the texture there), and the land height there into `point`'s z
/// when the cell has one. `material` is kept without a texture.
fn land_material(e: &mut Engine, cell: u32, point: u32, material: u32) -> u32 {
    let mut material = material;
    // The land coordinate data is 0x50 bytes (0053b550 fills it up to +0x4d); the height
    // follows it rather than sitting on its +0x40 word.
    e.with_stack(0x54, |e, data| {
        let data = data.addr();
        e.call(LAND_COORD_DATA_CONSTRUCT, &args![data]);
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(LAND_GET_COORD_DATA, &args![land, data, point, 1u32]);
        let byte = u32::from(e.mem.u8(data + 0x18));
        let word = u32::from(e.mem.u16(data + 0x3c));
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        let texture = e.call(LAND_TEXTURE_AT, &args![land, byte, word]).u32();
        if texture == 0 {
            return;
        }
        let height = data + 0x50;
        e.mem.set_u32(height, 0.0f32.to_bits());
        material = e.call(LAND_TEXTURE_MATERIAL, &args![texture]).u32();
        if e.call(CELL_GET_LAND_HEIGHT, &args![cell, point, height])
            .bool()
        {
            let value = x87_quiet_bits(e.mem.u32(height));
            e.mem.set_u32(point + 8, value);
        }
    });
    material
}

/// The landing and jumping volume: `0.95 - RandomFloat(0.0, 0.1)`.
fn random_volume(e: &mut Engine) -> u32 {
    let spread = x87_quiet_bits(e.global::<u32>(VOLUME_SPREAD_0101E2BC));
    let random = e.call(RANDOM_FLOAT, &args![0.0f32.to_bits(), spread]).f64();
    x87_store32(x87_sub(e.global::<f64>(VOLUME_BASE_0107D1B8), random))
}

// Translated from 008391b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008391b0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the sound handles) is not translated.
/// `ImpactMixer::PlayJumpLand` (engine map; cdecl): for an actor (vtable
/// +0x100), the landing sound of `material` (row of `0083a910(material)`,
/// entry 8), with material 8 (water) when the actor's feet are below the
/// water height of its cell (the interior cell, or the world cell under
/// it, when that cell passes the `00425fd0`/`004518e0` tests and the actor
/// fails `005a2030`), and for material 0x24 outside interiors the land
/// texture's material (and height) under it; `"FSTBabyLand"` for the player
/// when `008397d0` answers. For the player outside third person (the
/// player's `00524d10` clear) the sound is flat (flags 0x4109), else
/// positioned at the actor and following it (flags 0x410A) when within
/// hearing; volume `0.95 - RandomFloat(0.0, 0.1)`. The code's second
/// sound is never set (always 0); its branches are kept.
pub fn impact_mixer_play_jump_land(e: &mut Engine, actor: u32, material: u32) {
    e.with_stack(0xc, |e, handle| {
        let handle = handle.addr();
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        jump_land_body(e, actor, material, handle);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// `PlayJumpLand` between its handle's construction and destruction.
fn jump_land_body(e: &mut Engine, actor: u32, material: u32, handle: u32) {
    let mut material = material;
    let second_sound = 0u32;
    e.with_stack(0xc, |e, point| {
        let point = point.addr();
        let at = e.vcall(actor, 0x1f4, &args![]).u32();
        for word in 0..3 {
            let value = e.mem.u32(at + 4 * word);
            e.mem.set_u32(point + 4 * word, value);
        }
        let tes = e.global::<u32>(TES);
        let mut cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
        if !e.vcall(actor, 0x100, &args![]).bool() {
            return;
        }
        if cell == 0 {
            cell = world_cell(e, e.mem.u32(point), e.mem.u32(point + 4));
        }
        if e.vcall(actor, 0x100, &args![]).bool() && cell != 0 {
            let go = if e.call(PLACE_TEST_00425FD0, &args![cell]).bool() {
                e.call(CELL_TEST_004518E0, &args![cell]).bool()
                    || !e.call(PLACE_TEST_00425FD0, &args![cell]).bool()
            } else {
                !e.call(PLACE_TEST_00425FD0, &args![cell]).bool()
            };
            if go && !e.call(ACTOR_TEST_005A2030, &args![actor]).bool() {
                let water = e.call(GET_RELEVANT_WATER_HEIGHT, &args![actor]).f64();
                // `FLD z; FCOMPP; TEST AH,5; JP`: below the water height.
                if x87_load32(e.mem.u32(point + 8)) < water {
                    material = 8;
                }
            }
        }
        if material == 0x24
            && !e.call(PLACE_TEST_00425FD0, &args![cell]).bool()
            && e.call(CELL_GET_LAND, &args![cell]).u32() != 0
        {
            material = land_material(e, cell, point, material);
        }
        let row = fn_0083a910(e, material as i32);
        let mut first_sound = e
            .mem
            .u32(LANDING_SOUNDS.wrapping_add(row.wrapping_mul(0x28)));
        let player = e.global::<u32>(PLAYER);
        if actor == player && fn_008397d0(e, Ptr::new(player)) != 0 {
            first_sound = sound(e, BABY_LAND);
        }
        let player = e.global::<u32>(PLAYER);
        let flat = actor == player && !e.call(PLAYER_FLAG_00524D10, &args![player]).bool();
        let position = point;
        if flat {
            for sound in [first_sound, second_sound] {
                if sound != 0 {
                    handle_by_id(e, handle, sound, 0x4109);
                    let volume = random_volume(e);
                    e.call(SOUND_SET_VOLUME, &args![handle, volume]);
                    e.call(SOUND_PLAY, &args![handle, 0u32]);
                }
            }
            return;
        }
        let [x, y, z] = read_point(e, position);
        let heard = |e: &mut Engine, sound: u32| {
            e.call(WITHIN_HEARING, &args![sound, x, y, z, 1.0f32.to_bits()])
                .bool()
        };
        if !heard(e, first_sound) && !heard(e, second_sound) {
            return;
        }
        for sound in [first_sound, second_sound] {
            if sound != 0 {
                handle_by_id(e, handle, sound, 0x410a);
                let [x, y, z] = read_point(e, position).map(x87_quiet_bits);
                e.call(SOUND_SET_POSITION, &args![handle, x, y, z]);
                let followed = e.vcall(actor, 0x1d0, &args![]).u32();
                e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![handle, followed]);
                let volume = random_volume(e);
                e.call(SOUND_SET_VOLUME, &args![handle, volume]);
                e.call(SOUND_PLAY, &args![handle, 0u32]);
            }
        }
    });
}

/// `Interface::IsInMenuMode` (engine map).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// Cdecl, no arguments (`tesobjectweap.rs`): `AL`; halves the footstep
/// distance when set.
const FLAG_00525420: u32 = 0x0052_5420;
/// `2.0` (`double`).
const TWO_01011590: u32 = 0x0101_1590;
/// Thiscall on the audio instance (`baseprocess.rs` on folded code): the
/// listener, whose vtable +0xC gives its position.
const AUDIO_LISTENER: u32 = 0x0044_edb0;
/// The point subtraction in place (`tes.rs`, thiscall (point), `RET 4`)
/// and the point's length (`ST0`).
const POINT_SUBTRACT_IN_PLACE: u32 = 0x0045_78c0;
const POINT_LENGTH: u32 = 0x0045_7990;
/// `BGSSaveFormBuffer::GetForm` (engine map; folded body): the reference's
/// base form.
const BASE_FORM: u32 = 0x007a_f430;
/// The type descriptor the creature cast uses (`01183a00`).
const TARGET_TYPE_01183A00: u32 = 0x0118_3a00;
/// `TESCreature::PickCreatureSound` (engine map), thiscall (type), `RET 4`.
const PICK_CREATURE_SOUND: u32 = 0x005f_92d0;
/// Thiscall on the creature (`algs.rs`): an object to follow.
const CREATURE_FOLLOWED_00ACBB70: u32 = 0x00ac_bb70;
/// The creature jump volume (`010205fc`, `0.98`).
const CREATURE_JUMP_VOLUME: u32 = 0x0102_05fc;
/// `MobileObject::GetCharController` (engine map) and the controller's
/// ground material (`0059bb30` on its +0x410, engine map name on folded
/// code).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const CONTROLLER_MATERIAL: u32 = 0x0059_bb30;
/// Two actor tests (`actor_p6.rs`; the second is
/// `Actor::IsHeavyBodyArmorWorn`), `AL`.
const ACTOR_TEST_008BA410: u32 = 0x008b_a410;
const IS_HEAVY_BODY_ARMOR_WORN: u32 = 0x008b_a440;
/// The jump power and armor sounds, looked up once (`"FSTJumpPower"`,
/// `"FSTJumpArmor"`).
const JUMP_POWER_SOUND: u32 = 0x011d_d874;
const JUMP_ARMOR_SOUND: u32 = 0x011d_d870;
const JUMP_POWER: u32 = 0x0107_d1a8;
const JUMP_ARMOR: u32 = 0x0107_d198;
/// `Actor::IsRunning` (engine map).
const IS_RUNNING: u32 = 0x0088_4730;
/// The footstep armor sounds, looked up together: power armor left and
/// right, armor left and right, and their editor ids.
const ARMOR_POWER_L_SOUND: u32 = 0x011d_d860;
const ARMOR_POWER_R_SOUND: u32 = 0x011d_d864;
const ARMOR_L_SOUND: u32 = 0x011d_d868;
const ARMOR_R_SOUND: u32 = 0x011d_d86c;
const ARMOR_POWER_R: u32 = 0x0107_d188;
const ARMOR_POWER_L: u32 = 0x0107_d178;
const ARMOR_R: u32 = 0x0107_d16c;
const ARMOR_L: u32 = 0x0107_d160;
/// `fPlayerFootVolume:Audio`.
const PLAYER_FOOT_VOLUME: u32 = 0x011d_d988;
/// `128.0` (`double`): lowers the sound's height.
const HEIGHT_OFFSET_0102E430: u32 = 0x0102_e430;
/// `0.9` (`float`): the random volume's low end.
const VOLUME_LOW_010177E8: u32 = 0x0101_77e8;

/// The ground material of a jump without a creature: water (8) when
/// `water`, else the character controller's, else the land texture's (when
/// `land_first` and the controller's is 0x24), with the land height there.
fn jump_material(e: &mut Engine, reference: u32, cell: u32, water: bool, point: u32) -> u32 {
    let controller_material = |e: &mut Engine| -> Option<u32> {
        if reference == 0 {
            return None;
        }
        if e.call(GET_CHAR_CONTROLLER, &args![reference]).u32() == 0 {
            return None;
        }
        let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
        Some(
            e.call(CONTROLLER_MATERIAL, &args![controller + 0x410])
                .u32(),
        )
    };
    let tes = e.global::<u32>(TES);
    if e.call(TES_INTERIOR_CELL, &args![tes]).u32() == 0
        && cell != 0
        && e.call(CELL_GET_LAND, &args![cell]).u32() != 0
    {
        let mut material = 0u32;
        // The land coordinate data is 0x50 bytes: 0053b550 fills it up to the bytes at +0x4c and
        // +0x4d. The height is its own local after it (the exe: data at EBP-0x8c, height at
        // EBP-0x94 in 008391b0).
        e.with_stack(0x54, |e, data| {
            let data = data.addr();
            e.call(LAND_COORD_DATA_CONSTRUCT, &args![data]);
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            e.call(LAND_GET_COORD_DATA, &args![land, data, point, 1u32]);
            let byte = u32::from(e.mem.u8(data + 0x18));
            let word = u32::from(e.mem.u16(data + 0x3c));
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            let texture = e.call(LAND_TEXTURE_AT, &args![land, byte, word]).u32();
            let height = data + 0x50;
            e.mem.set_u32(height, 0.0f32.to_bits());
            if water {
                material = 8;
                return;
            }
            if reference != 0 && e.call(GET_CHAR_CONTROLLER, &args![reference]).u32() != 0 {
                let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
                if e.call(CONTROLLER_MATERIAL, &args![controller + 0x410])
                    .u32()
                    != 0x24
                {
                    let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
                    material = e
                        .call(CONTROLLER_MATERIAL, &args![controller + 0x410])
                        .u32();
                    return;
                }
            }
            if texture != 0 {
                material = e.call(LAND_TEXTURE_MATERIAL, &args![texture]).u32();
                if e.call(CELL_GET_LAND_HEIGHT, &args![cell, point, height])
                    .bool()
                {
                    let value = x87_quiet_bits(e.mem.u32(height));
                    e.mem.set_u32(point + 8, value);
                }
            }
        });
        return material;
    }
    if water {
        return 8;
    }
    controller_material(e).unwrap_or(0)
}

/// `dist <= sound byte * 100` as `FILD; FLD dist; FCOMPP; TEST AH,0x41;
/// JP` tests it (false when unordered).
fn within_reach(e: &mut Engine, sound: u32, distance: u32) -> bool {
    let reach = f64::from(e.call(SOUND_SECOND_VALUE_00553B90, &args![sound]).u8() as i32 * 0x64);
    x87_load32(distance) <= reach
}

// Translated from 00838780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00838780 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the guard and the sound handles) is not translated.
/// `ImpactMixer::PlayJump` (engine map; cdecl), under a TLS setting guard
/// (0xB): nothing in menu mode, nor when the actor is farther from the
/// listener than `fMaxFootstepDistance` (halved when `00525420` answers),
/// nor for a non-actor or when the reference's vtable +0x22C (0) answers.
/// A creature (vtable +0x21C) plays its jump sound (`PickCreatureSound`
/// 0x14, by id, flags 0x10A; when within the sound's reach) at its
/// position, following `00acbb70`, at volume 0.98. Others play the ground
/// material's jump sound (footstep entry 9) and, with power armor
/// (`008ba410`) or heavy armor, an extra sound: flat for the player outside
/// third person (flags 0x4109, `fPlayerFootVolume`), else positioned 128
/// units lower, following the actor, when heard and within reach (flags
/// 0x410A; random volumes `RandomFloat(0.9, 1.0)` and `1 -
/// RandomFloat(0.0, 0.1)`). The extra sound read for power armor is the
/// armor one (`011dd870`), as the code reads it.
pub fn impact_mixer_play_jump(e: &mut Engine, actor: u32) {
    movement_sound(e, actor, 0x3ad, None);
}

// Translated from 00837cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00837cc0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the guard and the sound handles) is not translated.
/// `ImpactMixer::PlayFootstep` (engine map; cdecl): as `PlayJump` (guard
/// line 0x2DF) with these differences: a creature plays its sound of type
/// `step`; the footstep entry is chosen from `step` (above 1 it is lowered
/// by 2; 0 gives entry 1, else 0; with `004997b0` 5 or 4, with the
/// character controller flag (`00838750`) 7 or 6, running 3 or 2); the
/// extra sounds are the power armor (`"FSTArmorPowerL"`/`"FSTArmorPowerR"`)
/// and armor (`"FSTArmorL"`/`"FSTArmorR"`) ones, by side; and there is no
/// hearing test before the positioned sounds.
pub fn impact_mixer_play_footstep(e: &mut Engine, actor: u32, step: u32) {
    movement_sound(e, actor, 0x2df, Some(step));
}

/// The shell `PlayJump` and `PlayFootstep` share: the TLS setting guard
/// (0xB, the source line), nothing in menu mode, the actor's position and
/// the two sound handles around `movement_body`.
fn movement_sound(e: &mut Engine, actor: u32, line: u32, footstep: Option<u32>) {
    e.with_stack(4, |e, guard| {
        e.call(
            TLS_GUARD_CONSTRUCT,
            &args![guard, 0xbu32, 1u32, SOURCE_FILE, line],
        );
        if !e.call(IS_IN_MENU_MODE, &args![]).bool() {
            e.with_stack(0x24, |e, locals| {
                let point = locals.addr();
                let first_handle = locals.addr() + 0xc;
                let second_handle = locals.addr() + 0x18;
                let at = e.vcall(actor, 0x1f4, &args![]).u32();
                for word in 0..3 {
                    let value = e.mem.u32(at + 4 * word);
                    e.mem.set_u32(point + 4 * word, value);
                }
                e.call(SOUND_HANDLE_CONSTRUCT, &args![first_handle]);
                e.call(SOUND_HANDLE_CONSTRUCT, &args![second_handle]);
                movement_body(e, actor, point, [first_handle, second_handle], footstep);
                e.call(SOUND_HANDLE_DESTRUCT, &args![second_handle]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![first_handle]);
            });
        }
        e.call(TLS_GUARD_DESTRUCT, &args![guard]);
    });
}

/// `PlayJump` and `PlayFootstep` once their two handles exist (`footstep`
/// is `PlayFootstep`'s step).
fn movement_body(
    e: &mut Engine,
    actor: u32,
    point: u32,
    [first_handle, second_handle]: [u32; 2],
    footstep: Option<u32>,
) {
    let at = e.call(SETTING_VALUE, &args![MAX_FOOTSTEP_DISTANCE]).u32();
    let mut reach = x87_quiet_bits(e.mem.u32(at));
    if e.call(FLAG_00525420, &args![]).bool() {
        reach = x87_store32(x87_div(x87_load32(reach), e.global::<f64>(TWO_01011590)));
    }
    let mut distance = 0.0f32.to_bits();
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    if e.call(AUDIO_LISTENER, &args![audio]).u32() != 0 {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let listener = e.call(AUDIO_LISTENER, &args![audio]).u32();
        let far = e.with_stack(0xc, |e, offset| {
            let at = e.vcall(listener, 0xc, &args![]).u32();
            for word in 0..3 {
                let value = e.mem.u32(at + 4 * word);
                e.mem.set_u32(offset.addr() + 4 * word, value);
            }
            e.call(POINT_SUBTRACT_IN_PLACE, &args![offset, point]);
            distance = x87_store32(e.call(POINT_LENGTH, &args![offset]).f64());
            let length = e.call(POINT_LENGTH, &args![offset]).f64();
            // `FLD reach; FCOMPP; TEST AH,5; JP`: too far when the reach is
            // below the length.
            x87_load32(reach) < length
        });
        if far {
            return;
        }
    }
    if !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    let reference = e
        .call(
            DYNAMIC_CAST,
            &args![
                actor,
                0u32,
                SOURCE_TYPE_011841CC,
                TARGET_TYPE_011846D4,
                0u32
            ],
        )
        .u32();
    if e.vcall(reference, 0x22c, &args![0u32]).bool() {
        return;
    }
    if e.vcall(reference, 0x21c, &args![]).bool() {
        let base = e.call(BASE_FORM, &args![actor]).u32();
        let creature = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, SOURCE_TYPE_01183108, TARGET_TYPE_01183A00, 0u32],
            )
            .u32();
        let sound = e
            .call(
                PICK_CREATURE_SOUND,
                &args![creature, footstep.unwrap_or(0x14)],
            )
            .u32();
        if sound != 0 && within_reach(e, sound, distance) {
            handle_by_id(e, first_handle, sound, 0x10a);
        }
        let [x, y, z] = read_point(e, point).map(x87_quiet_bits);
        e.call(SOUND_SET_POSITION, &args![first_handle, x, y, z]);
        if e.call(CREATURE_FOLLOWED_00ACBB70, &args![creature]).u32() != 0 {
            let followed = e.call(CREATURE_FOLLOWED_00ACBB70, &args![creature]).u32();
            e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![first_handle, followed]);
        }
        let volume = x87_quiet_bits(e.global::<u32>(CREATURE_JUMP_VOLUME));
        e.call(SOUND_SET_VOLUME, &args![first_handle, volume]);
        e.call(SOUND_PLAY, &args![first_handle, 0u32]);
        return;
    }
    let tes = e.global::<u32>(TES);
    let mut cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
    if cell == 0 {
        cell = world_cell(e, e.mem.u32(point), e.mem.u32(point + 4));
    }
    let mut water = false;
    if !e.call(ACTOR_TEST_005A2030, &args![reference]).bool() && cell != 0 {
        let height = e.call(GET_RELEVANT_WATER_HEIGHT, &args![reference]).f64();
        if x87_load32(e.mem.u32(point + 8)) < height {
            let set = if e.call(PLACE_TEST_00425FD0, &args![cell]).bool() {
                e.call(CELL_TEST_004518E0, &args![cell]).bool()
                    || !e.call(PLACE_TEST_00425FD0, &args![cell]).bool()
            } else {
                !e.call(PLACE_TEST_00425FD0, &args![cell]).bool()
            };
            water = set;
        }
    }
    // `PlayJump` computes this flag and does not use it.
    let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
    let flagged = fn_00838750(e, Ptr::new(controller)) != 0;
    let material = jump_material(e, reference, cell, water, point);
    let (entry, side) = match footstep {
        None => (9, 0),
        Some(step) => {
            let step = if (step as i32) > 1 {
                step.wrapping_sub(2)
            } else {
                step
            };
            (footstep_entry(e, reference, step, flagged), step)
        }
    };
    let row = fn_0083a910(e, material as i32);
    let first_sound = e
        .mem
        .u32(FOOTSTEP_SOUNDS.wrapping_add(row.wrapping_mul(0x28)) + entry * 4);
    let second_sound = match footstep {
        None => jump_extra_sound(e, reference),
        Some(_) => footstep_extra_sound(e, reference, side),
    };
    let player = e.global::<u32>(PLAYER);
    let flat = actor == player && !e.call(PLAYER_FLAG_00524D10, &args![player]).bool();
    if flat {
        for (handle, sound) in [(first_handle, first_sound), (second_handle, second_sound)] {
            if sound != 0 {
                handle_by_name(e, handle, sound, 0x4109);
                let volume = setting_bits(e, PLAYER_FOOT_VOLUME);
                e.call(SOUND_SET_VOLUME, &args![handle, volume]);
                e.call(SOUND_PLAY, &args![handle, 0u32]);
            }
        }
        return;
    }
    let [x, y, z] = read_point(e, point);
    let heard = |e: &mut Engine, sound: u32| {
        e.call(WITHIN_HEARING, &args![sound, x, y, z, 1.0f32.to_bits()])
            .bool()
    };
    // `PlayFootstep` has no hearing test here.
    if footstep.is_none() && !heard(e, first_sound) && !heard(e, second_sound) {
        return;
    }
    for (index, (handle, sound)) in [(first_handle, first_sound), (second_handle, second_sound)]
        .into_iter()
        .enumerate()
    {
        if sound == 0 || !within_reach(e, sound, distance) {
            continue;
        }
        handle_by_name(e, handle, sound, 0x410a);
        let lowered = x87_store32(x87_sub(
            x87_load32(e.mem.u32(point + 8)),
            e.global::<f64>(HEIGHT_OFFSET_0102E430),
        ));
        let [x, y] = [e.mem.u32(point), e.mem.u32(point + 4)].map(x87_quiet_bits);
        e.call(SOUND_SET_POSITION, &args![handle, x, y, lowered]);
        if e.vcall(actor, 0x1d0, &args![]).u32() != 0 {
            let followed = e.vcall(actor, 0x1d0, &args![]).u32();
            e.call(SOUND_SET_OBJECT_TO_FOLLOW, &args![handle, followed]);
        }
        let volume = if index == 0 {
            let low = x87_quiet_bits(e.global::<u32>(VOLUME_LOW_010177E8));
            x87_store32(e.call(RANDOM_FLOAT, &args![low, 1.0f32.to_bits()]).f64())
        } else {
            let spread = x87_quiet_bits(e.global::<u32>(VOLUME_SPREAD_0101E2BC));
            let random = e.call(RANDOM_FLOAT, &args![0.0f32.to_bits(), spread]).f64();
            x87_store32(x87_sub(1.0, random))
        };
        e.call(SOUND_SET_VOLUME, &args![handle, volume]);
        e.call(SOUND_PLAY, &args![handle, 0u32]);
    }
}

/// `PlayFootstep`'s footstep entry for the (lowered) `step`: 1 for step 0,
/// else 0; 5 or 4 when `004997b0` answers, else 7 or 6 with the character
/// controller flag, else 3 or 2 when running.
fn footstep_entry(e: &mut Engine, reference: u32, step: u32, flagged: bool) -> u32 {
    let mut entry = u32::from(step == 0);
    let pick = |base: u32| if step != 0 { base - 1 } else { base };
    if e.call(ACTOR_FLAG_004997B0, &args![reference]).bool() {
        entry = pick(5);
    } else if flagged {
        entry = pick(7);
    } else if e.call(IS_RUNNING, &args![reference]).bool() {
        entry = pick(3);
    }
    entry
}

/// `PlayJump`'s extra sound: with power armor (`008ba410`) the jump power
/// sound is looked up once but the armor one (`011dd870`) is used; with heavy
/// armor the armor one (looked up once).
fn jump_extra_sound(e: &mut Engine, reference: u32) -> u32 {
    if e.call(ACTOR_TEST_008BA410, &args![reference]).bool() {
        if e.global::<u32>(JUMP_POWER_SOUND) == 0 {
            let found = sound(e, JUMP_POWER);
            e.set_global(JUMP_POWER_SOUND, found);
        }
        return e.global::<u32>(JUMP_ARMOR_SOUND);
    }
    if e.call(IS_HEAVY_BODY_ARMOR_WORN, &args![reference]).bool() {
        if e.global::<u32>(JUMP_ARMOR_SOUND) == 0 {
            let found = sound(e, JUMP_ARMOR);
            e.set_global(JUMP_ARMOR_SOUND, found);
        }
        return e.global::<u32>(JUMP_ARMOR_SOUND);
    }
    0
}

/// `PlayFootstep`'s extra sound: the power armor (`008ba410`) or heavy armor
/// left/right pair (both looked up when either is missing), the first of
/// the pair for a non-zero `side`.
fn footstep_extra_sound(e: &mut Engine, reference: u32, side: u32) -> u32 {
    let pair = if e.call(ACTOR_TEST_008BA410, &args![reference]).bool() {
        [
            (ARMOR_POWER_R_SOUND, ARMOR_POWER_R),
            (ARMOR_POWER_L_SOUND, ARMOR_POWER_L),
        ]
    } else if e.call(IS_HEAVY_BODY_ARMOR_WORN, &args![reference]).bool() {
        [(ARMOR_R_SOUND, ARMOR_R), (ARMOR_L_SOUND, ARMOR_L)]
    } else {
        return 0;
    };
    let [(right_slot, right), (left_slot, left)] = pair;
    if e.global::<u32>(left_slot) == 0 || e.global::<u32>(right_slot) == 0 {
        let found = sound(e, right);
        e.set_global(right_slot, found);
        let found = sound(e, left);
        e.set_global(left_slot, found);
    }
    if side != 0 {
        e.global::<u32>(left_slot)
    } else {
        e.global::<u32>(right_slot)
    }
}

/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE_00401030: u32 = 0x0040_1030;
/// `BSAudio::GetSoundHandleByName` (engine map; thiscall on the audio instance: result, name,
/// flags; `RET 0xc`).
const GET_SOUND_HANDLE_BY_NAME_00AD7550: u32 = 0x00ad_7550;
/// Thiscall on the weapon form: its block sound form (`005224e0`), and its skill index
/// (`00446390`, -1 for none), as `008397f0` uses them.
const WEAPON_BLOCK_SOUND_005224E0: u32 = 0x0052_24e0;
const WEAPON_INDEX_00446390: u32 = 0x0044_6390;
/// `"WPNBlockHand"` and `"WPNBlockMelee"` (`.rdata`).
const BLOCK_HAND_0107D1DC: u32 = 0x0107_d1dc;
const BLOCK_MELEE_0107D1CC: u32 = 0x0107_d1cc;
/// The vector constructor `006815c0` (thiscall), run on the position local.
const VECTOR_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
/// The `float` minimum (`0040ebd0`) and maximum (`00404010`), cdecl, the result in ST0.
const FLOAT_MIN_0040EBD0: u32 = 0x0040_ebd0;
const FLOAT_MAX_00404010: u32 = 0x0040_4010;

// Translated from 008397f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008397f0 - the C++ exception frame (FS:[0] chain) is not translated
/// `ImpactMixer::PlayWeaponBlock` (Xbox PDB; cdecl, nine words: the first and the last are not
/// read, then the weapon form and two positions). A sound handle local (`0041a250`) gets: with a
/// weapon that has a block sound (`005224e0`), the handle `BSAudio::GetSoundHandleByFilename`
/// gives for that sound's file name (`00511840`) with flags `0x182` and the sound; otherwise
/// `GetSoundHandleByName` with `"WPNBlockHand"` when the weapon is null or its `00446390` is -1,
/// `"WPNBlockMelee"` otherwise (each through a temporary handle, assigned by `00418900` and
/// destroyed). The sound is placed at the middle of the two positions, per component
/// `(max - min) / 2.0 + min` on the x87 (`0040ebd0`/`00404010`, results kept as `double`s, the
/// constant at `01011590`), and played (`00ad8830(0)`); the handle is destroyed.
#[allow(clippy::too_many_arguments)]
pub fn impact_mixer_play_weapon_block(
    e: &mut Engine,
    _unused_0: u32,
    weapon: u32,
    first_x: f32,
    first_y: f32,
    first_z: f32,
    second_x: f32,
    second_y: f32,
    second_z: f32,
    _unused_8: u32,
) {
    e.with_stack(0x40, |e, frame| {
        // The exe's locals from EBP - 0x4c.
        let at = |offset: u32| frame.addr() + 0x4c - offset;
        let handle = at(0x18);
        let position = at(0x24);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let block_sound = if weapon != 0 {
            e.call(WEAPON_BLOCK_SOUND_005224E0, &args![weapon]).u32()
        } else {
            0
        };
        if weapon != 0 && block_sound != 0 {
            let sound = e.call(WEAPON_BLOCK_SOUND_005224E0, &args![weapon]).u32();
            let form = e.call(WEAPON_BLOCK_SOUND_005224E0, &args![weapon]).u32();
            let name = e.call(SOUND_FILE_NAME, &args![form]).u32();
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let temporary = at(0x34);
            let made = e
                .call(
                    GET_SOUND_HANDLE_BY_FILENAME,
                    &args![audio, temporary, name, 0x182u32, sound],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
        } else {
            let index = if weapon != 0 {
                e.call(WEAPON_INDEX_00446390, &args![weapon]).u32()
            } else {
                0xffff_ffff
            };
            let (name, temporary) = if index == 0xffff_ffff {
                (BLOCK_HAND_0107D1DC, at(0x40))
            } else {
                (BLOCK_MELEE_0107D1CC, at(0x4c))
            };
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let made = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME_00AD7550,
                    &args![audio, temporary, name, 0x182u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
        }
        e.call(VECTOR_CONSTRUCT_006815C0, &args![position]);
        let two: f64 = e.global(TWO_01011590);
        for (k, (first, second)) in (0u32..).zip([
            (first_x, second_x),
            (first_y, second_y),
            (first_z, second_z),
        ]) {
            let first = x87_quiet_bits(first.to_bits());
            let second = x87_quiet_bits(second.to_bits());
            let low = e.call(FLOAT_MIN_0040EBD0, &args![first, second]).f64();
            let high = e.call(FLOAT_MAX_00404010, &args![first, second]).f64();
            let low_again = e.call(FLOAT_MIN_0040EBD0, &args![first, second]).f64();
            let middle = x87_add(x87_div(x87_sub(high, low_again), two), low);
            e.mem.set_u32(position + k * 4, x87_store32(middle));
        }
        let words = [0u32, 4, 8].map(|offset| x87_quiet_bits(e.mem.u32(position + offset)));
        e.call(
            SOUND_SET_POSITION,
            &args![handle, words[0], words[1], words[2]],
        );
        e.call(SOUND_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

// Translated from 0083b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int, unsigned int>`'s scalar deleting destructor (the map's name; thiscall, one
/// word, `RET 4`): [`fn_0083b6b0`], then with flag bit 0 `operator delete`. Returns `this`.
pub fn ni_t_pointer_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0083b6b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_00401030, &args![this]);
    }
    this
}

// Translated from 0083b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int, unsigned int>`'s scalar deleting
/// destructor (the map's name; thiscall, one word, `RET 4`): [`fn_0083b710`], then with flag bit
/// 0 `operator delete`. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0083b710(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_97d0, fn_008397d0(Ptr) -> u8),
        entry!(0x0083_b5e0, fn_0083b5e0(Ptr, u32) -> Ptr),
        entry!(0x0083_74b0, fn_008374b0(u32) -> u32),
        entry!(0x0083_7550, impact_mixer_play_collision_sound(u32)),
        entry!(0x0083_8750, fn_00838750(Ptr) -> u32),
        entry!(0x0083_9ab0, impact_mixer_init_footsteps()),
        entry!(0x0083_9d30, fn_00839d30()),
        entry!(0x0083_9da0, fn_00839da0() -> bool),
        entry!(0x0083_9dc0, fn_00839dc0(u8)),
        entry!(0x0083_9e00, impact_mixer_get_sound_for_type(u32, f32, f32) -> u32),
        entry!(0x0083_a910, fn_0083a910(i32) -> u32),
        entry!(0x0083_a930, fn_0083a930(u32, u32)),
        entry!(0x0083_b640, fn_0083b640(Ptr, u32) -> Ptr),
        entry!(0x0083_b6b0, fn_0083b6b0(Ptr)),
        entry!(0x0083_b710, fn_0083b710(Ptr)),
        entry!(0x0083_ac30, fn_0083ac30(u32, u32, u32, u8)),
        entry!(0x0083_91b0, impact_mixer_play_jump_land(u32, u32)),
        entry!(0x0083_8780, impact_mixer_play_jump(u32)),
        entry!(0x0083_7cc0, impact_mixer_play_footstep(u32, u32)),
        entry!(
            0x0083_97f0,
            impact_mixer_play_weapon_block(u32, u32, f32, f32, f32, f32, f32, f32, u32)
        ),
        entry!(
            0x0083_b610,
            ni_t_pointer_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0083_b740,
            ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0083B640, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b5e0, &args![this, 0x1fu32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0083B640),
            vec![vec![this, 0x1f]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_0083B5E0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_008397d0_answers_the_byte_at_offset_0x7c6() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x7d0);
        e.mem.set_u8(this + 0x7c5, 0x11);
        e.mem.set_u8(this + 0x7c6, 0x5a);
        e.mem.set_u8(this + 0x7c7, 0x22);
        assert_eq!(e.call(0x0083_97d0, &args![this]).u8(), 0x5a);
    }
}

#[cfg(test)]
mod b047_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// An engine with the unit's globals and constants mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_d000u32,
            0x011d_e000,
            0x011a_1000,
            0x0101_1000,
            0x0101_2000,
            0x0101_5000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_3000,
            0x0102_e000,
            0x0103_2000,
            0x0103_4000,
            0x0106_b000,
            0x0107_3000,
            0x0107_d000,
            0x011c_3000,
            0x011f_4000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e
    }

    #[test]
    fn material_names_prefer_overrides() {
        let mut e = engine();
        e.set_global(MATERIAL_NAMES + 8, 0x500u32);
        assert_eq!(e.call(0x0083_74b0, &args![2u32]).u32(), 0x500);
        e.set_global(MATERIAL_NAME_OVERRIDES + 8, 0x600u32);
        assert_eq!(e.call(0x0083_74b0, &args![2u32]).u32(), 0x600);
    }

    #[test]
    fn fn_00838750_tests_the_flag() {
        let mut e = engine();
        e.register(FLAGS_TEST_00621270, |_, a| {
            eax(u32::from(a[1] == 0x0100_0000) * 4)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_8750, &args![0x5000u32]).u32(), 1);
        assert_eq!(
            calls_to(&e, FLAGS_TEST_00621270),
            vec![vec![0x5410, 0x0100_0000]]
        );
        e.register(FLAGS_TEST_00621270, |_, _| eax(0));
        assert_eq!(e.call(0x0083_8750, &args![0x5000u32]).u32(), 0);
    }

    #[test]
    fn init_footsteps_falls_back_twice_then_to_the_default() {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x7700u32);
        // Material 0 falls back to 1, 1 to 2; only material 2 has sounds for
        // the first suffix; everything else is the default.
        for material in 0..0xdu32 {
            e.set_global(FALLBACK_MATERIALS + 4 * material, 0xdu32);
            e.set_global(MATERIAL_NAMES + 4 * material, 0x100 + material);
        }
        e.set_global(FALLBACK_MATERIALS, 1u32);
        e.set_global(FALLBACK_MATERIALS + 4, 2u32);
        for step in 0..0xau32 {
            e.set_global(FOOTSTEP_SUFFIXES + 4 * step, 0x200 + step);
        }
        e.register(FORMAT, |e, a| {
            e.mem.set_u32(a[0], a[3]);
            e.mem.set_u32(a[0] + 4, a[4]);
            Ret::default()
        });
        e.register(GET_SOUND, |e, a| {
            if a[1] == DEFAULT_FOOTSTEP {
                return eax(0xdef);
            }
            let (name, suffix) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            eax(if name == 0x102 && suffix == 0x200 {
                0x5a
            } else {
                0
            })
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_9ab0, &args![]);
        assert_eq!(e.global::<u32>(FOOTSTEP_SOUNDS), 0x5a);
        assert_eq!(e.global::<u32>(FOOTSTEP_SOUNDS + 4), 0xdef);
        assert_eq!(e.global::<u32>(FOOTSTEP_SOUNDS + 0x28), 0x5a);
        assert_eq!(e.global::<u32>(FOOTSTEP_SOUNDS + 0x50), 0x5a);
        assert_eq!(e.global::<u32>(FOOTSTEP_SOUNDS + 0x78), 0xdef);
        assert_eq!(e.global::<u8>(FOOTSTEPS_READY), 1);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats[0][1..], [0x103, FOOTSTEP_FORMAT, 0x100, 0x200]);
        // Done once.
        e.call_log = Some(vec![]);
        e.call(0x0083_9ab0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn impact_table_reset_runs_once() {
        let mut e = engine();
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(IMPACT_RECORDS + 0x24, 7u32);
        assert!(!e.call(0x0083_9da0, &args![]).bool());
        e.call(0x0083_9d30, &args![]);
        assert!(e.call(0x0083_9da0, &args![]).bool());
        assert_eq!(e.global::<u32>(IMPACT_RECORDS + 0x24), 0);
        assert_eq!(e.global::<f32>(IMPACT_RECORDS + 0x24 * 19 + 8), f32::MAX);
        e.set_global(IMPACT_RECORDS, 9u32);
        e.call(0x0083_9d30, &args![]);
        assert_eq!(e.global::<u32>(IMPACT_RECORDS), 9);
        e.call(0x0083_9dc0, &args![0u32]);
        assert!(!e.call(0x0083_9da0, &args![]).bool());
        e.set_global(MIXER_FLAGS, 0xffu32);
        e.call(0x0083_9dc0, &args![0u32]);
        assert_eq!(e.global::<u32>(MIXER_FLAGS), 0xfd);
    }

    fn sound_engine() -> Engine {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x7700u32);
        e.register(GET_SOUND, |_, a| eax(a[1]));
        e.register(SETTING_VALUE, |_, a| eax(a[0]));
        e.set_global(SMALL_MASS_01032980, 0.0001f64);
        e.set_global(MASS_6_01034208, 6.0f64);
        e.set_global(MASS_70_01073568, 70.0f64);
        e.set_global(MASS_15_01015A38, 15.0f64);
        for (setting, value) in [
            (STONE_MEDIUM_MASS_MIN, 10.0f32),
            (STONE_LARGE_MASS_MIN, 50.0),
            (EARTH_MEDIUM_MASS_MIN, 10.0),
            (EARTH_LARGE_MASS_MIN, 50.0),
            (METAL_MEDIUM_MASS_MIN, 10.0),
            (METAL_LARGE_MASS_MIN, 50.0),
            (SKIN_MEDIUM_MASS_MIN, 10.0),
            (SKIN_LARGE_MASS_MIN, 50.0),
            (WOOD_MEDIUM_MASS_MIN, 10.0),
            (WOOD_LARGE_MASS_MIN, 50.0),
            (COLLISION_SOUND_HEAVY_THRESHOLD, 100.0),
        ] {
            e.set_global(setting, value);
        }
        e
    }

    #[test]
    fn sound_for_type_picks_by_mass_and_force() {
        let mut e = sound_engine();
        let pick = |e: &mut Engine, material: u32, mass: f32, force: f32| {
            e.call(0x0083_9e00, &args![material, mass, force]).u32()
        };
        assert_eq!(pick(&mut e, 0, 0.00001, 0.0), 0x0107_d724);
        assert_eq!(pick(&mut e, 15, 5.0, 0.0), 0x0107_d718);
        assert_eq!(pick(&mut e, 0, 20.0, 0.0), 0x0107_d708);
        assert_eq!(pick(&mut e, 0, 60.0, 0.0), 0x0107_d6fc);
        assert_eq!(pick(&mut e, 0, f32::NAN, 0.0), 0x0107_d6fc);
        assert_eq!(pick(&mut e, 1, 1.0, 50.0), 0x0107_d5a8);
        assert_eq!(pick(&mut e, 1, 1.0, 150.0), 0x0107_d594);
        assert_eq!(pick(&mut e, 2, 20.0, 0.0), 0x0107_d6e0);
        assert_eq!(pick(&mut e, 3, 10.0, 0.0), 0x0107_d508);
        assert_eq!(pick(&mut e, 4, 100.0, 0.0), 0x0107_d6ac);
        assert_eq!(pick(&mut e, 5, 5.0, 150.0), 0x0107_d61c);
        assert_eq!(pick(&mut e, 5, 5.0, 50.0), 0x0107_d67c);
        assert_eq!(pick(&mut e, 7, 60.0, 150.0), 0x0107_d524);
        assert_eq!(pick(&mut e, 8, 1.0, 0.0), 0x0107_d5d4);
        assert_eq!(pick(&mut e, 9, 0.00001, 0.0), 0x0107_d5e0);
        assert_eq!(pick(&mut e, 16, 60.0, 0.0), 0x0107_d444);
        assert_eq!(pick(&mut e, 17, 20.0, 0.0), 0x0107_d3f0);
        assert_eq!(pick(&mut e, 29, 0.0, 150.0), 0x0107_d26c);
        for (material, name) in [
            (6u32, 0x0107_d584u32),
            (10, 0x0107_d4dc),
            (13, 0x0107_d4f0),
            (14, 0x0107_d23c),
            (30, 0x0107_d25c),
            (35, 0x0107_d208),
        ] {
            assert_eq!(pick(&mut e, material, 0.0, 0.0), name);
        }
        for material in [18u32, 19, 0x24, 0xffff_ffff] {
            assert_eq!(pick(&mut e, material, 0.0, 0.0), 0);
        }
    }

    #[test]
    fn fn_0083a910_reads_inside_the_table() {
        let mut e = engine();
        e.set_global(TABLE_011A17F8 + 4, 0x11u32);
        e.set_global(TABLE_011A17F8 + 0x8c, 0x23u32);
        assert_eq!(e.call(0x0083_a910, &args![1i32]).u32(), 0x11);
        assert_eq!(e.call(0x0083_a910, &args![0x23i32]).u32(), 0x23);
        assert_eq!(e.call(0x0083_a910, &args![0i32]).u32(), 0);
        assert_eq!(e.call(0x0083_a910, &args![0x24i32]).u32(), 0);
        assert_eq!(e.call(0x0083_a910, &args![-1i32]).u32(), 0);
    }

    #[test]
    fn pointer_map_construction_and_destruction() {
        let mut e = engine();
        e.register(NI_MALLOC, |_, a| eax(0x9000 + a[0]));
        stub(&mut e, &[MEMSET, MAP_REMOVE_ALL, NI_FREE]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b640, &args![this, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 8), 0x9014);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x9014, 0, 0x14]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_b6b0, &args![this]);
        assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x9014]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_b710, &args![this]);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![this]]);
    }

    /// An actor-like object whose vtable has the test functions
    /// `0x7000 + slot`.
    fn object(e: &mut Engine) -> u32 {
        let slots: Vec<u32> = (0..0x90).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x0f00_0000);
        object
    }

    fn impact_engine() -> (Engine, u32) {
        let mut e = engine();
        let actor = object(&mut e);
        let position = e.mem.alloc(0xc);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * index as u32, *value);
        }
        e.register_double(0x0000_71f4, move |_, _| eax(position));
        e.register(0x0000_71d0, |_, _| eax(0x4400));
        e.register(SETTING_VALUE, |e, _| eax(e.mem.alloc(4)));
        e.register(ACTOR_PROCESS_008D8520, |_, a| eax(a[0]));
        e.register(0x0000_7148, |_, _| eax(0x5500));
        e.register(OBJECT_0044DDC0, |_, _| eax(0x5600));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        e.register(SOUND_FOR_TYPE_9, |_, _| eax(0x6609));
        e.register(SOUND_FOR_TYPE_10, |_, _| eax(0x6610));
        e.register(WITHIN_HEARING, |_, _| eax(1));
        e.register(ACTOR_FLAG_004997B0, |_, _| eax(0));
        e.register(SOUND_FILE_NAME, |_, _| eax(0x6800));
        e.register(AUDIO_INSTANCE, |_, _| eax(0x6900));
        e.register(GET_SOUND_HANDLE_BY_FILENAME, |_, a| eax(a[1]));
        e.set_global(QUIET_VOLUME_010231A8, 0.4f32);
        stub(
            &mut e,
            &[
                TLS_GUARD_CONSTRUCT,
                TLS_GUARD_DESTRUCT,
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                SOUND_HANDLE_ASSIGN,
                SOUND_SET_POSITION,
                SOUND_SET_OBJECT_TO_FOLLOW,
                SOUND_SET_VOLUME,
                SOUND_PLAY,
            ],
        );
        (e, actor)
    }

    #[test]
    fn fn_0083a930_plays_positioned_for_others() {
        let (mut e, actor) = impact_engine();
        e.call_log = Some(vec![]);
        e.call(0x0083_a930, &args![actor, 10u32]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][2..], [0x6800, 0x4102, 0x6610]);
        assert_eq!(
            calls_to(&e, SOUND_SET_POSITION)[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(calls_to(&e, SOUND_SET_OBJECT_TO_FOLLOW)[0][1], 0x4400);
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 1.0f32.to_bits());
        assert_eq!(calls_to(&e, SOUND_PLAY)[0][1], 0);
        let guards = calls_to(&e, TLS_GUARD_CONSTRUCT);
        assert_eq!(guards[0][1..], [0xb, 1, SOURCE_FILE, 0x6a3]);
        assert_eq!(calls_to(&e, TLS_GUARD_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_0083a930_plays_flat_for_the_player_and_skips_silences() {
        let (mut e, actor) = impact_engine();
        e.set_global(PLAYER, actor);
        e.register(ACTOR_FLAG_004997B0, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0083_a930, &args![actor, 9u32]);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME)[0][3],
            0x8000_4101
        );
        assert!(calls_to(&e, SOUND_SET_POSITION).is_empty());
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 0.4f32.to_bits());
        // Another kind has no sound; out of hearing nothing plays either.
        e.call_log = Some(vec![]);
        e.call(0x0083_a930, &args![actor, 3u32]);
        assert!(calls_to(&e, SOUND_PLAY).is_empty());
        e.register(WITHIN_HEARING, |_, _| eax(0));
        e.call(0x0083_a930, &args![actor, 9u32]);
        assert!(calls_to(&e, SOUND_PLAY).is_empty());
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&e, TLS_GUARD_DESTRUCT).len(), 2);
    }
}

#[cfg(test)]
mod b047_sound_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_d000u32,
            0x011d_e000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_0000,
            0x0102_e000,
            0x0103_2000,
            0x0103_4000,
            0x0106_b000,
            0x0107_3000,
            0x0107_d000,
            0x011c_3000,
            0x011f_4000,
            0x011f_6000,
            0x0101_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(DATA_HANDLER, 0x7700u32);
        e.register(GET_SOUND, |_, a| eax(a[1]));
        e.register(SETTING_VALUE, |_, a| eax(a[0]));
        e.set_global(SMALL_MASS_01032980, 0.0001f64);
        for setting in [STONE_MEDIUM_MASS_MIN, METAL_MEDIUM_MASS_MIN] {
            e.set_global(setting, 10.0f32);
        }
        for setting in [STONE_LARGE_MASS_MIN, METAL_LARGE_MASS_MIN] {
            e.set_global(setting, 50.0f32);
        }
        e.set_global(COLLISION_SOUND_HEAVY_THRESHOLD, 100.0f32);
        e.register(AUDIO_INSTANCE, |_, _| eax(0x6900));
        stub(
            &mut e,
            &[
                TLS_GUARD_CONSTRUCT,
                TLS_GUARD_DESTRUCT,
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                SOUND_HANDLE_ASSIGN,
                SOUND_SET_POSITION,
                SOUND_SET_OBJECT_TO_FOLLOW,
                SOUND_SET_VOLUME,
                SOUND_SET_PRIORITY,
                SOUND_PLAY,
            ],
        );
        e
    }

    fn collision_engine() -> (Engine, u32) {
        let mut e = engine();
        e.set_global(STRENGTH_DIVISOR_0106B178, 400.0f64);
        e.set_global(ATTENUATION_SCALE_0102ED48, 3000.0f64);
        e.set_global(FREQUENCY_OFFSET_0107D0F8, 4500.0f64);
        e.set_global(FREQUENCY_DIVISOR_0107D0F0, -3000.0f64);
        e.set_global(SCALE_0107D138, 0.01f64);
        e.set_global(DIVISOR_010179E0, 200.0f64);
        e.register(GET_DYNAMIC_MASS, |_, _| Ret {
            st0: 20.0,
            ..Ret::default()
        });
        e.register(BODY_OBJECT_0043B4F0, |_, a| eax(a[0]));
        e.register(OBJECT_KIND_0043B4D0, |_, _| eax(0));
        e.register(WITHIN_HEARING, |_, _| eax(1));
        e.register(CLOCK_NOW, |_, _| eax(1000));
        e.register(MAP_GET_AT, |_, _| eax(0));
        let interval = e.mem.alloc(4);
        e.mem.set_u32(interval, 100);
        e.register_double(SETTING_INT, move |_, _| eax(interval));
        e.register(MAP_GET_FIRST_POSITION, |_, _| eax(0));
        e.register(SOUND_ID, |_, a| eax(a[0]));
        e.register(GET_SOUND_HANDLE_BY_ID, |_, a| eax(a[1]));
        stub(
            &mut e,
            &[
                TIMER_CONSTRUCT,
                ATEXIT,
                TIMER_START,
                TIMER_STOP,
                MAP_SET_AT,
                SOUND_SET_STATIC_ATTENUATION,
                SOUND_SET_MOD_FREQUENCY,
                MAP_GET_NEXT,
                MAP_REMOVE_AT,
                MAP_REMOVE_ALL,
            ],
        );
        let collision = e.mem.alloc(0x24);
        for (offset, value) in [(0, 1.0f32), (4, 2.0), (8, 3.0), (0xc, 100.0)] {
            e.mem.set_f32(collision + offset, value);
        }
        e.mem.set_u8(collision + 0x14, 0);
        e.mem.set_u8(collision + 0x15, 1);
        e.mem.set_u32(collision + 0x18, 5);
        e.mem.set_u32(collision + 0x1c, 0x4100);
        e.mem.set_u32(collision + 0x20, 0x4200);
        (e, collision)
    }

    #[test]
    fn collision_sound_plays_both_sides() {
        let (mut e, collision) = collision_engine();
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert_eq!(calls_to(&e, TIMER_CONSTRUCT), vec![vec![COLLISION_TIMER]]);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![TIMER_DESTRUCT_AT_EXIT]]);
        assert_eq!(
            calls_to(&e, TIMER_START),
            vec![vec![COLLISION_TIMER, COLLISION_TIMER_NAME]]
        );
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_ID);
        assert_eq!(made.len(), 2);
        // Stone of mass 20 and a heavy material-1 hit.
        assert_eq!(made[0][2..], [0x0107_d708, 0x4102]);
        assert_eq!(made[1][2..], [0x0107_d594, 0x4102]);
        assert_eq!(
            calls_to(&e, MAP_SET_AT),
            vec![vec![COLLISION_TIMES, 5, 1000]]
        );
        // 100 / 400: (1 - 0.25) * 3000 = 2250, frequency (2250 - 4500) / -3000.
        let attenuation = calls_to(&e, SOUND_SET_STATIC_ATTENUATION);
        assert_eq!(attenuation.len(), 2);
        assert_eq!(attenuation[0][1], 2250);
        assert_eq!(
            calls_to(&e, SOUND_SET_MOD_FREQUENCY)[0][1],
            0.75f32.to_bits()
        );
        assert_eq!(calls_to(&e, SOUND_SET_PRIORITY)[0][1], 0xff);
        assert_eq!(calls_to(&e, SOUND_PLAY).len(), 2);
        assert_eq!(e.global::<f32>(LAST_COLLISION_POSITION + 8), 3.0);
        assert_eq!(calls_to(&e, TIMER_STOP).len(), 1);
        assert_eq!(calls_to(&e, TLS_GUARD_DESTRUCT).len(), 1);
        // The timer is constructed once.
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert!(calls_to(&e, TIMER_CONSTRUCT).is_empty());
    }

    #[test]
    fn collision_sound_waits_for_the_interval() {
        let (mut e, collision) = collision_engine();
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 950);
            eax(1)
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert!(calls_to(&e, SOUND_PLAY).is_empty());
        assert!(calls_to(&e, MAP_SET_AT).is_empty());
        assert_eq!(calls_to(&e, TIMER_STOP).len(), 1);
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
    }

    #[test]
    fn collision_sound_of_a_casing() {
        let (mut e, collision) = collision_engine();
        e.register(OBJECT_KIND_0043B4D0, |_, _| eax(0x19));
        e.register(FLOAT_TEST_0040EB10, |_, _| eax(1));
        e.set_global(CASING_LIMIT_010162C0, 2.0f32);
        e.set_global(CASING_LIMIT_0101622C, 0.25f32);
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_ID);
        assert_eq!(made, vec![vec![0x6900, made[0][1], CASING_PISTOL, 0x4182]]);
        assert_eq!(
            calls_to(&e, FLOAT_TEST_0040EB10),
            vec![vec![20.0f32.to_bits(), 2.0f32.to_bits(), 0.25f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, SOUND_PLAY).len(), 1);
        // A light casing.
        e.register(GET_DYNAMIC_MASS, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_ID)[0][2], CASING_SHOTGUN);
    }

    #[test]
    fn collision_sound_skips_self_collisions_and_silence() {
        let (mut e, collision) = collision_engine();
        e.mem.set_u32(collision + 0x20, 0x4100);
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert!(calls_to(&e, TIMER_START).is_empty());
        assert_eq!(calls_to(&e, TLS_GUARD_DESTRUCT).len(), 1);
        e.mem.set_u32(collision + 0x20, 0x4200);
        e.register(WITHIN_HEARING, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        assert!(calls_to(&e, SOUND_HANDLE_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, TIMER_STOP).len(), 1);
    }

    #[test]
    fn collision_sound_drops_old_times() {
        let (mut e, collision) = collision_engine();
        let mut first = 1u32;
        e.register_double(MAP_GET_FIRST_POSITION, move |_, _| {
            let position = first;
            first = 0;
            eax(position)
        });
        e.register(MAP_GET_NEXT, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], 77);
            e.mem.set_u32(a[3], 100);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_7550, &args![collision]);
        // 1000 - 100 = 900 is beyond four intervals (400).
        assert_eq!(calls_to(&e, MAP_REMOVE_AT), vec![vec![COLLISION_TIMES, 77]]);
        assert!(calls_to(&e, MAP_REMOVE_ALL).is_empty());
    }

    fn weapon_engine() -> (Engine, u32) {
        let mut e = engine();
        e.set_global(MIXER_FLAGS, 2u32);
        let slots: Vec<u32> = (0..0x80).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let actor = e.mem.alloc(0x40);
        e.mem.set_u32(actor, 0x0f00_0000);
        e.register(0x0000_71d0, |_, _| eax(0x4400));
        let position = e.mem.alloc(0xc);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * index as u32, *value);
        }
        e.register_double(OBJECT_POSITION, move |_, _| eax(position));
        let listener = e.mem.alloc(0xc);
        e.register_double(LISTENER_POSITION, move |_, _| eax(listener));
        e.register(POINT_SUBTRACT, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(PLAYER_FLAG_00524D10, |_, _| eax(1));
        e.register(PLAYER_PLACE_008D6F30, |_, _| eax(0));
        e.register(ACTOR_PROCESS_008D8520, |_, _| eax(0));
        e.register(RETURN_THIS, |_, a| eax(a[0]));
        e.register(WEAPON_SOUND_005215A0, |_, _| eax(0x6601));
        e.register(WEAPON_SOUND_00522440, |_, _| eax(0x6602));
        e.register(WEAPON_SOUND_00522470, |_, _| eax(0x6603));
        e.register(WITHIN_HEARING, |_, _| eax(1));
        e.register(SOUND_IS_VALID, |_, _| eax(0));
        e.register(GAME_TIME, |_, _| eax(1234));
        e.register(SOUND_FILE_NAME, |_, a| eax(a[0] + 0x100));
        e.register(GET_SOUND_HANDLE_BY_FILENAME, |_, a| eax(a[1]));
        e.register(SOUND_FIRST_VALUE_0082D660, |_, _| eax(3));
        e.register(SOUND_SECOND_VALUE_00553B90, |_, _| eax(2));
        e.set_global(HUNDRED_01017A40, 100.0f64);
        e.set_global(FIVE_01020998, 5.0f64);
        stub(&mut e, &[SOUND_STOP, SOUND_SET_MIN_MAX]);
        (e, actor)
    }

    #[test]
    fn weapon_sound_takes_the_oldest_record() {
        let (mut e, actor) = weapon_engine();
        e.call_log = Some(vec![]);
        e.call(0x0083_ac30, &args![0x5000u32, actor, 0u32, 0u32]);
        let record = SOUND_RECORDS + 19 * 0x24;
        assert_eq!(e.mem.u32(record + 0x18), actor);
        assert_eq!(e.mem.u32(record + 0x1c), 1234);
        assert_eq!(e.mem.f32(record + 0x20), 14.0);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made[0][2..], [0x6701, 0x0002_0182, 0x6601]);
        assert_eq!(made[1][2..], [0x6702, 0x0002_0182, 0x6602]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_ASSIGN),
            vec![vec![record, made[0][1]], vec![record + 0xc, made[1][1]]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_OBJECT_TO_FOLLOW),
            vec![vec![record, 0x4400], vec![record + 0xc, 0x4400]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_MIN_MAX),
            vec![
                vec![record, 15.0f32.to_bits(), 200.0f32.to_bits()],
                vec![record + 0xc, 15.0f32.to_bits(), 200.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, SOUND_SET_POSITION).len(), 4);
        assert_eq!(
            calls_to(&e, SOUND_PLAY),
            vec![vec![record, 0], vec![record + 0xc, 0]]
        );
    }

    #[test]
    fn weapon_sound_in_first_person_plays_the_single_sound() {
        let (mut e, actor) = weapon_engine();
        e.call_log = Some(vec![]);
        e.call(0x0083_ac30, &args![0x5000u32, actor, 0u32, 1u32]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][2..], [0x6703, 0x0002_0181, 0x6603]);
        assert!(calls_to(&e, SOUND_SET_MIN_MAX).is_empty());
        assert_eq!(calls_to(&e, SOUND_PLAY).len(), 1);
        // Not reset: nothing at all.
        e.set_global(MIXER_FLAGS, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0083_ac30, &args![0x5000u32, actor, 0u32, 1u32]);
        assert!(calls_to(&e, SOUND_HANDLE_CONSTRUCT).is_empty());
        // No weapon: the handles only.
        e.set_global(MIXER_FLAGS, 2u32);
        e.call_log = Some(vec![]);
        e.call(0x0083_ac30, &args![0u32, actor, 0u32, 0u32]);
        assert!(calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME).is_empty());
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
    }
}

#[cfg(test)]
mod b047_movement_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// An engine with an actor at (1, 2, 3) inside an interior cell, its
    /// ground material 2 (row 3 of the sound table), and the sound calls
    /// doubled.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x011d_d000u32,
            0x011d_e000,
            0x011a_1000,
            0x0101_1000,
            0x0101_7000,
            0x0101_e000,
            0x0102_0000,
            0x0102_e000,
            0x0107_d000,
            0x011c_3000,
        ] {
            e.map(page, 0x1000);
        }
        let slots: Vec<u32> = (0..0x90).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let actor = e.mem.alloc(0x800);
        e.mem.set_u32(actor, 0x0f00_0000);
        let position = e.mem.alloc(0xc);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * index as u32, *value);
        }
        e.register_double(0x0000_71f4, move |_, _| eax(position));
        e.register(0x0000_7100, |_, _| eax(1));
        e.register(0x0000_71d0, |_, _| eax(0x4400));
        e.register(0x0000_722c, |_, _| eax(0));
        e.register(0x0000_721c, |_, _| eax(0));
        e.set_global(TES, 0x9000u32);
        e.set_global(DATA_HANDLER, 0x7700u32);
        e.register(TES_INTERIOR_CELL, |_, _| eax(0x9100));
        e.register(PLACE_TEST_00425FD0, |_, _| eax(1));
        e.register(CELL_TEST_004518E0, |_, _| eax(1));
        e.register(ACTOR_TEST_005A2030, |_, _| eax(0));
        e.register(GET_RELEVANT_WATER_HEIGHT, |_, _| st0(-100.0));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        e.register(GET_CHAR_CONTROLLER, |_, _| eax(0x9300));
        e.register(CONTROLLER_MATERIAL, |_, _| eax(2));
        e.register(FLAGS_TEST_00621270, |_, _| eax(0));
        e.set_global(TABLE_011A17F8 + 8, 3u32);
        e.set_global(TABLE_011A17F8 + 0x20, 5u32);
        for entry in 0..10u32 {
            e.set_global(FOOTSTEP_SOUNDS + 3 * 0x28 + 4 * entry, 0x6600 + entry);
            e.set_global(FOOTSTEP_SOUNDS + 5 * 0x28 + 4 * entry, 0x6800 + entry);
        }
        e.register(GET_SOUND, |_, a| eax(a[1]));
        e.register(SETTING_VALUE, |_, a| eax(a[0]));
        e.set_global(MAX_FOOTSTEP_DISTANCE, 1000.0f32);
        e.set_global(PLAYER_FOOT_VOLUME, 0.5f32);
        e.register(PLAYER_FLAG_00524D10, |_, _| eax(1));
        e.register(WITHIN_HEARING, |_, _| eax(1));
        e.register(SOUND_SECOND_VALUE_00553B90, |_, _| eax(1));
        e.register(AUDIO_INSTANCE, |_, _| eax(0x6900));
        e.register(SOUND_ID, |_, a| eax(a[0]));
        e.register(SOUND_FILE_NAME, |_, a| eax(a[0] + 0x1000));
        e.register(GET_SOUND_HANDLE_BY_ID, |_, a| eax(a[1]));
        e.register(GET_SOUND_HANDLE_BY_FILENAME, |_, a| eax(a[1]));
        e.register(RANDOM_FLOAT, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.set_global(VOLUME_SPREAD_0101E2BC, 0.1f32);
        e.set_global(VOLUME_BASE_0107D1B8, 1.0f64);
        e.set_global(VOLUME_LOW_010177E8, 0.9f32);
        e.set_global(HEIGHT_OFFSET_0102E430, 128.0f64);
        e.set_global(CREATURE_JUMP_VOLUME, 0.98f32);
        e.register(IS_IN_MENU_MODE, |_, _| eax(0));
        e.register(FLAG_00525420, |_, _| eax(0));
        e.register(AUDIO_LISTENER, |_, _| eax(0));
        e.register(ACTOR_TEST_008BA410, |_, _| eax(0));
        e.register(IS_HEAVY_BODY_ARMOR_WORN, |_, _| eax(0));
        e.register(ACTOR_FLAG_004997B0, |_, _| eax(0));
        e.register(IS_RUNNING, |_, _| eax(0));
        stub(
            &mut e,
            &[
                TLS_GUARD_CONSTRUCT,
                TLS_GUARD_DESTRUCT,
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_DESTRUCT,
                SOUND_HANDLE_ASSIGN,
                SOUND_SET_POSITION,
                SOUND_SET_OBJECT_TO_FOLLOW,
                SOUND_SET_VOLUME,
                SOUND_PLAY,
            ],
        );
        (e, actor)
    }

    fn position(z: f32) -> Vec<u32> {
        vec![1.0f32.to_bits(), 2.0f32.to_bits(), z.to_bits()]
    }

    #[test]
    fn jump_land_plays_the_landing_sound_positioned() {
        let (mut e, actor) = engine();
        e.call_log = Some(vec![]);
        e.call(0x0083_91b0, &args![actor, 2u32]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_ID);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][2..], [0x6608, 0x410a]);
        assert_eq!(calls_to(&e, SOUND_SET_POSITION)[0][1..], position(3.0)[..]);
        assert_eq!(calls_to(&e, SOUND_SET_OBJECT_TO_FOLLOW)[0][1], 0x4400);
        // 1.0 - RandomFloat(0.0, 0.1) with the double answering its first argument.
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 1.0f32.to_bits());
        assert_eq!(calls_to(&e, SOUND_PLAY).len(), 1);
    }

    #[test]
    fn jump_land_in_water_and_on_land() {
        let (mut e, actor) = engine();
        e.register(GET_RELEVANT_WATER_HEIGHT, |_, _| st0(10.0));
        e.call_log = Some(vec![]);
        e.call(0x0083_91b0, &args![actor, 2u32]);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_ID)[0][2], 0x6808);
        // Outside, material 0x24 takes the land texture's material and height.
        let (mut e, actor) = engine();
        e.register(PLACE_TEST_00425FD0, |_, _| eax(0));
        e.register(CELL_GET_LAND, |_, _| eax(0x9500));
        e.register(LAND_TEXTURE_AT, |_, _| eax(0x9600));
        e.register(LAND_TEXTURE_MATERIAL, |_, _| eax(2));
        e.register(CELL_GET_LAND_HEIGHT, |e, a| {
            e.mem.set_f32(a[2], 7.0);
            eax(1)
        });
        stub(&mut e, &[LAND_COORD_DATA_CONSTRUCT, LAND_GET_COORD_DATA]);
        e.call_log = Some(vec![]);
        e.call(0x0083_91b0, &args![actor, 0x24u32]);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_ID)[0][2], 0x6608);
        assert_eq!(calls_to(&e, SOUND_SET_POSITION)[0][1..], position(7.0)[..]);
        assert_eq!(calls_to(&e, LAND_GET_COORD_DATA)[0][3], 1);
    }

    #[test]
    fn jump_land_for_the_player_and_non_actors() {
        let (mut e, actor) = engine();
        e.set_global(PLAYER, actor);
        e.register(PLAYER_FLAG_00524D10, |_, _| eax(0));
        e.mem.set_u8(actor + 0x7c6, 1);
        e.call_log = Some(vec![]);
        e.call(0x0083_91b0, &args![actor, 2u32]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_ID);
        assert_eq!(made[0][2..], [BABY_LAND, 0x4109]);
        assert!(calls_to(&e, SOUND_SET_POSITION).is_empty());
        assert!(calls_to(&e, WITHIN_HEARING).is_empty());
        // Not an actor: only the handle.
        e.register(0x0000_7100, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0083_91b0, &args![actor, 2u32]);
        assert!(calls_to(&e, GET_SOUND_HANDLE_BY_ID).is_empty());
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 1);
    }

    #[test]
    fn jump_plays_the_jump_and_armor_sounds() {
        let (mut e, actor) = engine();
        e.register(IS_HEAVY_BODY_ARMOR_WORN, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0083_8780, &args![actor]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][2..], [0x7609, 0x410a, 0x6609]);
        assert_eq!(made[1][3..], [0x410a, JUMP_ARMOR]);
        assert_eq!(e.global::<u32>(JUMP_ARMOR_SOUND), JUMP_ARMOR);
        assert_eq!(
            calls_to(&e, SOUND_SET_POSITION)[0][1..],
            position(3.0 - 128.0)[..]
        );
        let volumes = calls_to(&e, SOUND_SET_VOLUME);
        assert_eq!(volumes[0][1], 0.9f32.to_bits());
        assert_eq!(volumes[1][1], 1.0f32.to_bits());
        let guard = calls_to(&e, TLS_GUARD_CONSTRUCT);
        assert_eq!(guard[0][1..], [0xb, 1, SOURCE_FILE, 0x3ad]);
        // Power armor looks the power sound up but plays the armor one.
        let (mut e, actor) = engine();
        e.register(ACTOR_TEST_008BA410, |_, _| eax(1));
        e.set_global(JUMP_ARMOR_SOUND, 0x6a00u32);
        e.call_log = Some(vec![]);
        e.call(0x0083_8780, &args![actor]);
        assert_eq!(e.global::<u32>(JUMP_POWER_SOUND), JUMP_POWER);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME)[1][4], 0x6a00);
    }

    #[test]
    fn jump_stops_far_away_in_menus_and_for_creatures() {
        let (mut e, actor) = engine();
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, 0x0f00_0000);
        let at = e.mem.alloc(0xc);
        e.register_double(AUDIO_LISTENER, move |_, _| eax(listener));
        e.register_double(0x0000_700c, move |_, _| eax(at));
        stub(&mut e, &[POINT_SUBTRACT_IN_PLACE]);
        e.register(POINT_LENGTH, |_, _| st0(5000.0));
        e.call_log = Some(vec![]);
        e.call(0x0083_8780, &args![actor]);
        assert!(calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME).is_empty());
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
        // In menu mode only the guard.
        e.register(IS_IN_MENU_MODE, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0083_8780, &args![actor]);
        assert!(calls_to(&e, SOUND_HANDLE_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, TLS_GUARD_DESTRUCT).len(), 1);
        // A creature plays its own jump sound.
        let (mut e, actor) = engine();
        e.register(0x0000_721c, |_, _| eax(1));
        e.register(BASE_FORM, |_, _| eax(0x9400));
        e.register(PICK_CREATURE_SOUND, |_, a| eax(0x6700 + a[1]));
        e.register(CREATURE_FOLLOWED_00ACBB70, |_, _| eax(0x4500));
        e.call_log = Some(vec![]);
        e.call(0x0083_8780, &args![actor]);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_ID)[0][2..],
            [0x6714, 0x10a]
        );
        assert_eq!(calls_to(&e, SOUND_SET_OBJECT_TO_FOLLOW)[0][1], 0x4500);
        assert_eq!(calls_to(&e, SOUND_SET_VOLUME)[0][1], 0.98f32.to_bits());
    }

    #[test]
    fn footstep_picks_the_entry_and_the_side() {
        let (mut e, actor) = engine();
        e.register(IS_RUNNING, |_, _| eax(1));
        e.register(ACTOR_TEST_008BA410, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0083_7cc0, &args![actor, 3u32]);
        // Step 3 is lowered to 1: running gives entry 2; the left armor sound.
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made[0][4], 0x6602);
        assert_eq!(made[1][4], ARMOR_POWER_L);
        assert_eq!(e.global::<u32>(ARMOR_POWER_R_SOUND), ARMOR_POWER_R);
        assert!(calls_to(&e, WITHIN_HEARING).is_empty());
        assert_eq!(calls_to(&e, TLS_GUARD_CONSTRUCT)[0][4], 0x2df);
        // Step 0 with the 004997b0 flag: entry 5 and the right side.
        let (mut e, actor) = engine();
        e.register(ACTOR_FLAG_004997B0, |_, _| eax(1));
        e.register(IS_HEAVY_BODY_ARMOR_WORN, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x0083_7cc0, &args![actor, 0u32]);
        let made = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(made[0][4], 0x6605);
        assert_eq!(made[1][4], ARMOR_R);
        // A creature's sound of the step's type.
        let (mut e, actor) = engine();
        e.register(0x0000_721c, |_, _| eax(1));
        e.register(BASE_FORM, |_, _| eax(0x9400));
        e.register(PICK_CREATURE_SOUND, |_, a| eax(0x6700 + a[1]));
        e.register(CREATURE_FOLLOWED_00ACBB70, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x0083_7cc0, &args![actor, 3u32]);
        assert_eq!(calls_to(&e, GET_SOUND_HANDLE_BY_ID)[0][2], 0x6703);
        assert!(calls_to(&e, SOUND_SET_OBJECT_TO_FOLLOW).is_empty());
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn block_doubles(e: &mut Engine) {
        for address in [
            SOUND_HANDLE_CONSTRUCT,
            SOUND_HANDLE_DESTRUCT,
            SOUND_HANDLE_ASSIGN,
            VECTOR_CONSTRUCT_006815C0,
            SOUND_SET_POSITION,
            SOUND_PLAY,
        ] {
            e.register(address, quiet);
        }
        e.register(FLOAT_MIN_0040EBD0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(FLOAT_MAX_00404010, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        e.register(AUDIO_INSTANCE, |_, _| 0xa0d1_0000u32.into_ret());
        e.register(GET_SOUND_HANDLE_BY_FILENAME, |_, a| a[1].into_ret());
        e.register(GET_SOUND_HANDLE_BY_NAME_00AD7550, |_, a| a[1].into_ret());
        e.register(SOUND_FILE_NAME, |_, a| (a[0] + 0x10).into_ret());
        e.map(TWO_01011590, 8);
        e.mem.set_u64(TWO_01011590, 0x4000_0000_0000_0000);
    }

    fn play(e: &mut Engine, weapon: u32) {
        e.call_log = Some(vec![]);
        e.call(
            0x0083_97f0,
            &args![0xdeadu32, weapon, 1.0f32, -4.0f32, 10.0f32, 3.0f32, 2.0f32, 10.5f32, 0xbeefu32],
        );
    }

    #[test]
    fn n089_008397f0_plays_the_block_sound_between_the_positions() {
        let mut e = Engine::new();
        block_doubles(&mut e);
        // A weapon with its own block sound.
        e.register(WEAPON_BLOCK_SOUND_005224E0, |_, _| 0x5050u32.into_ret());
        play(&mut e, 0x7700);
        let handle = calls_to(&e, SOUND_HANDLE_CONSTRUCT)[0][0];
        let by_file = calls_to(&e, GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(by_file.len(), 1);
        let temporary = by_file[0][1];
        assert_eq!(
            by_file[0],
            vec![0xa0d1_0000, temporary, 0x5060, 0x182, 0x5050]
        );
        assert_eq!(calls_to(&e, WEAPON_BLOCK_SOUND_005224E0).len(), 3);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_ASSIGN),
            vec![vec![handle, temporary]]
        );
        assert_eq!(
            calls_to(&e, SOUND_SET_POSITION),
            vec![vec![
                handle,
                2.0f32.to_bits(),
                (-1.0f32).to_bits(),
                10.25f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&e, SOUND_PLAY), vec![vec![handle, 0]]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTRUCT),
            vec![vec![temporary], vec![handle]]
        );
        assert_eq!(calls_to(&e, VECTOR_CONSTRUCT_006815C0)[0][0], handle - 0xc);
        // No block sound and no skill: the hand sound.
        e.register(WEAPON_BLOCK_SOUND_005224E0, |_, _| 0u32.into_ret());
        e.register(WEAPON_INDEX_00446390, |_, _| 0xffff_ffffu32.into_ret());
        play(&mut e, 0x7700);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_NAME_00AD7550)[0][2],
            BLOCK_HAND_0107D1DC
        );
        assert_eq!(calls_to(&e, WEAPON_BLOCK_SOUND_005224E0).len(), 1);
        // A skill: the melee sound.
        e.register(WEAPON_INDEX_00446390, |_, _| 2u32.into_ret());
        play(&mut e, 0x7700);
        let by_name = calls_to(&e, GET_SOUND_HANDLE_BY_NAME_00AD7550);
        assert_eq!(by_name[0][2..], [BLOCK_MELEE_0107D1CC, 0x182]);
        // No weapon: the hand sound, without asking the weapon anything.
        play(&mut e, 0);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_NAME_00AD7550)[0][2],
            BLOCK_HAND_0107D1DC
        );
        assert!(calls_to(&e, WEAPON_BLOCK_SOUND_005224E0).is_empty());
        assert!(calls_to(&e, WEAPON_INDEX_00446390).is_empty());
    }

    #[test]
    fn n089_0083b610_deletes_the_pointer_map() {
        let mut e = Engine::new();
        for address in [MAP_REMOVE_ALL, NI_FREE, OPERATOR_DELETE_00401030] {
            e.register(address, quiet);
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x8800);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b610, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x8800]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE_00401030), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_b610, &args![this, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE_00401030).is_empty());
    }

    #[test]
    fn n089_0083b740_deletes_the_map_base() {
        let mut e = Engine::new();
        for address in [MAP_REMOVE_ALL, NI_FREE, OPERATOR_DELETE_00401030] {
            e.register(address, quiet);
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b740, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE_00401030), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0083_b740, &args![this, 2u32]);
        assert!(calls_to(&e, OPERATOR_DELETE_00401030).is_empty());
    }
}

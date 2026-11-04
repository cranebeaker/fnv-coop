//! `(unplaced)/run_005f2b60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// The note player of an animation group (`005f2b60`; no name in the engine
// map, `animation.rs` calls it for a playing group). `this` holds the notes:
// `005f36f0` counts them and `005f3710` gives one, a record with the time
// (`float`, +0), a type or volume byte (+4), a word that is the frequency
// (`float`) for a sound note and an index (low byte) and text offset (upper
// bytes) for a text note (+8), and the sound (`TESSound*`, +0xc).

/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `255.0` (`double`): the volume byte's scale.
const VOLUME_SCALE: u32 = 0x0101_e568;
/// The volume (`float`) of the creature, weapon and armor sounds
/// (`010205fc`).
const NOTE_SOUND_VOLUME: u32 = 0x0102_05fc;

/// `005f36f0` (`thiscall`): the number of notes (the engine map's name for
/// the folded body does not apply).
const NOTE_COUNT: u32 = 0x005f_36f0;
/// `005f3710` (`thiscall`, `RET 4`): the note at an index.
const NOTE_AT: u32 = 0x005f_3710;
/// `004e75d0` (`thiscall` on the sound, `RET 4`): the sound's data, built in
/// the given local; its word at +4 holds the sound's flags.
const SOUND_DATA: u32 = 0x004e_75d0;
/// `005e39b0` (`cdecl`, the sound flags): the extra flags the flags give.
const SOUND_EXTRA_FLAGS: u32 = 0x005e_39b0;
/// `0082eca0` (`cdecl`: sound, x, y, z, 1.0): whether the sound may play at
/// the point (`AL`).
const SOUND_MAY_PLAY: u32 = 0x0082_eca0;
/// `005f2420` (`thiscall`): the group's number (compared with 0xb1..0xc7).
const GROUP_NUMBER: u32 = 0x005f_2420;
/// `00507b20` (`thiscall`): a speed factor, in `ST0`.
const SPEED_FACTOR: u32 = 0x0050_7b20;
/// `00524d10` (`thiscall` on the player): a test that drops flag 0x40000.
const PLAYER_TEST: u32 = 0x0052_4d10;
/// `00511840` (`thiscall` on the sound): the sound's file name.
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// `BSAudio::QInstance` (engine map), no parameters.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByFilename` (engine map), `RET 0x10`.
const HANDLE_BY_FILE_NAME: u32 = 0x00ad_7480;
/// `BSAudio::GetSoundHandleByNumericID` (engine map), `RET 0xc`.
const HANDLE_BY_ID: u32 = 0x00ad_73b0;
/// `BSSoundHandle` methods (engine map names).
const HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const HANDLE_SET_VOLUME: u32 = 0x00ad_89e0;
const HANDLE_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
const HANDLE_SET_MOD_FREQUENCY: u32 = 0x00ad_8a90;
const HANDLE_PLAY: u32 = 0x00ad_8830;
/// `0041a250` (`thiscall`): the `BSSoundHandle` constructor.
const HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// `00418900` (`thiscall`, `RET 4`): `BSSoundHandle` assignment.
const HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `00483710` (`thiscall`): the `BSSoundHandle` destructor.
const HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `0084e3a0` (`thiscall`): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `ImpactMixer::PlayFootstep` (engine map), `cdecl` (reference, type).
const PLAY_FOOTSTEP: u32 = 0x0083_7cc0;
/// `007af430` (`thiscall`): a reference's base form.
const BASE_FORM: u32 = 0x007a_f430;
/// `TESCreature::PickCreatureSound` (engine map), `RET 4`.
const PICK_CREATURE_SOUND: u32 = 0x005f_92d0;
/// `0083a930` (`cdecl`: reference, type).
const PLAY_EFFECT_NOTE: u32 = 0x0083_a930;
/// `00700300` (`thiscall`) and `00495f30` (`thiscall`, `RET 4`, with the
/// address of a count): the text keys of the sequence.
const SEQUENCE_KEYS: u32 = 0x0070_0300;
const KEY_ARRAY: u32 = 0x0049_5f30;
/// `00717e50` (`thiscall`): a key's part (the engine map's name for the
/// folded body does not apply).
const KEY_PART: u32 = 0x0071_7e50;
/// `0043b1b0` (`thiscall`): the part's text.
const KEY_TEXT: u32 = 0x0043_b1b0;
/// `atof` (engine map), `cdecl`, in `ST0`.
const ATOF: u32 = 0x00ec_a573;
/// `ShakeCamera` (engine map), `cdecl` (strength, position).
const SHAKE_CAMERA: u32 = 0x008d_1960;
/// `BSAudio::StopMovingSounds` (engine map), `RET 0xc`.
const STOP_MOVING_SOUNDS: u32 = 0x00ad_8570;
/// `PlayerCharacter::PlayImpactBasedHitShader` (engine map), `RET 4`.
const PLAY_IMPACT_HIT_SHADER: u32 = 0x0096_2290;
/// `Actor::GetCurrentWeapon` (engine map).
const CURRENT_WEAPON: u32 = 0x008a_1710;
/// `008bffa0` (`thiscall`): the weapon's note sound.
const WEAPON_NOTE_SOUND: u32 = 0x008b_ffa0;
/// `Actor::GetArmorBeingWorn` (engine map), `RET 4`.
const ARMOR_BEING_WORN: u32 = 0x0089_1b90;
/// `0044ddc0` (`thiscall`): the word at `this`.
const WORD_AT: u32 = 0x0044_ddc0;
/// `00512d40` (`thiscall`, `RET 4`): the armor's sound for the note type.
const ARMOR_NOTE_SOUND: u32 = 0x0051_2d40;
/// `004459e0` (`thiscall`, `RET 4`): releases the worn-armor result.
const ARMOR_RELEASE: u32 = 0x0044_59e0;

/// The sound flag bit for a reference that is the player (`0x200000`),
/// computed as the exe does (`(ref != player ? 0xffe00000 : 0) + 0x200000`).
fn player_flag(e: &Engine, reference: u32) -> u32 {
    let mask = if reference != e.global::<u32>(PLAYER_POINTER) {
        0xffe0_0000u32
    } else {
        0
    };
    mask.wrapping_add(0x0020_0000)
}

/// The note sound frequency: `mod_frequency * 1.0` stored as a float.
fn note_frequency(mod_frequency: u32) -> u32 {
    x87_quiet_bits(x87_store32(x87_mul(x87_load32(mod_frequency), 1.0)))
}

/// The reference's position (slot +0x1f4), three words.
fn position(e: &mut Engine, reference: u32) -> [u32; 3] {
    let point = e.vcall(reference, 0x1f4, &[]).u32();
    [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)]
}

/// `0082eca0` with the position passed by value.
fn sound_may_play(e: &mut Engine, sound: u32, at: [u32; 3]) -> bool {
    e.call(
        SOUND_MAY_PLAY,
        &args![sound, at[0], at[1], at[2], 1f32.to_bits()],
    )
    .bool()
}

/// For groups 0xb1 to 0xc7 of a reference with the slot +0x1e4 value, the
/// frequency is multiplied by `00507b20` of it.
fn adjust_frequency(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    has_value: bool,
    frequency: u32,
) -> u32 {
    if has_value
        && e.call(GROUP_NUMBER, &args![this]).i32() >= 0xb1
        && e.call(GROUP_NUMBER, &args![this]).i32() <= 0xc7
    {
        let value = e.vcall(reference, 0x1e4, &[]).u32();
        let factor = e.call(SPEED_FACTOR, &args![value]).f64();
        x87_store32(x87_mul(factor, x87_m32(frequency)))
    } else {
        frequency
    }
}

fn set_position(e: &mut Engine, handle: u32, at: [u32; 3]) {
    e.call(
        HANDLE_SET_POSITION,
        &args![
            handle,
            x87_quiet_bits(at[0]),
            x87_quiet_bits(at[1]),
            x87_quiet_bits(at[2])
        ],
    );
}

fn follow(e: &mut Engine, handle: u32, reference: u32) {
    let object = e.vcall(reference, 0x1d0, &[]).u32();
    e.call(HANDLE_SET_OBJECT_TO_FOLLOW, &args![handle, object]);
}

/// Volume `010205fc`, frequency, play (0) and destroy, the end of the
/// creature, weapon and armor notes.
fn finish_note_sound(e: &mut Engine, handle: u32, frequency: u32) {
    let volume = x87_quiet_bits(e.global::<u32>(NOTE_SOUND_VOLUME));
    e.call(HANDLE_SET_VOLUME, &args![handle, volume]);
    e.call(
        HANDLE_SET_MOD_FREQUENCY,
        &args![handle, note_frequency(frequency)],
    );
    e.call(HANDLE_PLAY, &args![handle, 0u32]);
    e.call(HANDLE_DESTRUCT, &args![handle]);
}

/// The text keys of `sequence` (`00700300`, `00495f30` with a count local):
/// the key array, or `None` when the count is 0.
fn text_key_value(e: &mut Engine, sequence: u32, note: u32, extra: u32) -> Option<f64> {
    let (count, keys) = e.with_stack(4, |e, count| {
        let source = e.call(SEQUENCE_KEYS, &args![sequence]).u32();
        let keys = e.call(KEY_ARRAY, &args![source, count]).u32();
        (e.mem.u32(count.addr()), keys)
    });
    if count == 0 {
        return None;
    }
    let packed = e.mem.u32(note + 8);
    let key = keys.wrapping_add((packed & 0xff).wrapping_mul(8));
    let part = e.call(KEY_PART, &args![key]).u32();
    let text = e.call(KEY_TEXT, &args![part]).u32();
    let offset = (packed & 0xffff_ff00) >> 8;
    Some(
        e.call(ATOF, &args![text.wrapping_add(offset).wrapping_add(extra)])
            .f64(),
    )
}

// Translated from 005f2b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f2b60 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Plays the notes of the animation group `this` whose time falls in
/// `(from, to]` (or, when `from > to`, after `from` or up to `to`) for
/// `reference` (`thiscall`, `RET 0x10`; no name in the engine map).
///
/// A note with a sound plays it by file name (`00ad7480`, flags: 1 for a
/// looping sound else 2, 0x100, 0x40000 for the player unless `00524d10`,
/// 0x200000 for the player, the sound's extra flags and 0x40000000) at the
/// reference's position, following it, with the note's volume byte / 255 and
/// frequency, when its extra flags are set or `0082eca0` allows it.
/// Otherwise the note's type (the code's jump table, a `match` here) picks:
/// footsteps (types 0-3, `00837cc0`), a creature sound (4-6, 23-30), the
/// effect notes 9 and 10 (`0083a930`), a camera shake from the sequence's
/// text key (11, `008d1960` when above 0), stopping moving sounds with the
/// text key's volume (12, `00ad8570`), the player's hit shader (13,
/// `00962290`), the weapon's note sound (16) or the worn armor's (17-22).
/// For groups 0xb1 to 0xc7 the frequency follows the reference's speed
/// factor (`00507b20`).
pub fn fn_005f2b60(e: &mut Engine, this: Ptr, reference: u32, from: f32, to: f32, sequence: u32) {
    let mut frequency = 1f32.to_bits();
    let has_value = e.vcall(reference, 0x1e4, &[]).u32() != 0;
    let start = x87_load32(from.to_bits());
    let end = x87_load32(to.to_bits());
    let wraps = start > end;
    let mut index = 0u32;
    while index < e.call(NOTE_COUNT, &args![this]).u32() {
        let note = e.call(NOTE_AT, &args![this, index]).u32();
        let time = x87_load32(e.mem.u32(note));
        let hit = if wraps {
            start < time || end >= time
        } else {
            start < time && end >= time
        };
        if hit {
            let sound = e.mem.u32(note + 0xc);
            if sound != 0 {
                frequency = play_sound_note(e, this, reference, note, sound, has_value, frequency);
            } else {
                frequency =
                    play_typed_note(e, this, reference, note, sequence, has_value, frequency);
            }
        }
        index = index.wrapping_add(1);
    }
}

/// The sound branch of [`fn_005f2b60`]; answers the frequency factor.
fn play_sound_note(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    note: u32,
    sound: u32,
    has_value: bool,
    frequency: u32,
) -> u32 {
    e.with_stack(0x80, |e, locals| {
        let handle = locals.addr();
        let data_a = handle + 0x10;
        let data_b = handle + 0x34;
        let data_c = handle + 0x58;
        let data = e.call(SOUND_DATA, &args![sound, data_a]).u32();
        let looping = e.mem.u32(data + 4) & 0x40 != 0;
        let data = e.call(SOUND_DATA, &args![sound, data_b]).u32();
        let flags = e.mem.u32(data + 4);
        let extra_flags = e.call(SOUND_EXTRA_FLAGS, &args![flags]).u32();
        let at = position(e, reference);
        if extra_flags == 0 && !sound_may_play(e, sound, at) {
            return frequency;
        }
        let frequency = adjust_frequency(e, this, reference, has_value, frequency);
        let player = e.global::<u32>(PLAYER_POINTER);
        let quiet_player = if reference == player && !e.call(PLAYER_TEST, &args![player]).bool() {
            0x0004_0000
        } else {
            0
        };
        let kind = if looping { 1u32 } else { 2 };
        let handle_flags =
            kind | 0x100 | quiet_player | player_flag(e, reference) | extra_flags | 0x4000_0000;
        let name = e.call(SOUND_FILE_NAME, &args![sound]).u32();
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        e.call(
            HANDLE_BY_FILE_NAME,
            &args![audio, handle, name, handle_flags, sound],
        );
        set_position(e, handle, at);
        let volume = x87_store32(x87_div(
            f64::from(e.mem.u8(note + 4)),
            e.global::<f64>(VOLUME_SCALE),
        ));
        e.call(HANDLE_SET_VOLUME, &args![handle, x87_quiet_bits(volume)]);
        follow(e, handle, reference);
        let note_frequency =
            x87_store32(x87_mul(x87_load32(e.mem.u32(note + 8)), x87_m32(frequency)));
        e.call(
            HANDLE_SET_MOD_FREQUENCY,
            &args![handle, x87_quiet_bits(note_frequency)],
        );
        let data = e.call(SOUND_DATA, &args![sound, data_c]).u32();
        let flags = e.mem.u32(data + 4);
        let wait = e.call(SOUND_EXTRA_FLAGS, &args![flags]).u32() != 0;
        e.call(HANDLE_PLAY, &args![handle, u32::from(wait)]);
        e.call(HANDLE_DESTRUCT, &args![handle]);
        frequency
    })
}

/// The typed notes of [`fn_005f2b60`]; answers the frequency factor.
fn play_typed_note(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    note: u32,
    sequence: u32,
    has_value: bool,
    frequency: u32,
) -> u32 {
    let note_type = e.mem.u8(note + 4);
    match note_type {
        0..=3 => {
            e.call(PLAY_FOOTSTEP, &args![reference, u32::from(note_type)]);
        }
        4..=6 | 23..=30 => {
            if !e.vcall(reference, 0x100, &[]).bool()
                || reference == 0
                || !e.vcall(reference, 0x21c, &[]).bool()
            {
                return frequency;
            }
            let base = e.call(BASE_FORM, &args![reference]).u32();
            let note_type = e.mem.u8(note + 4);
            let kind = if note_type < 0x16 {
                u32::from(note_type)
            } else {
                u32::from(e.mem.u8(note + 4)) - 0xb
            };
            let sound = e.call(PICK_CREATURE_SOUND, &args![base, kind]).u32();
            let at = position(e, reference);
            if sound == 0 || !sound_may_play(e, sound, at) {
                return frequency;
            }
            return e.with_stack(0x10, |e, handle| {
                let handle = handle.addr();
                let flags = player_flag(e, reference) | 0x4000_0102;
                let id = e.call(FORM_ID, &args![sound]).u32();
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                e.call(HANDLE_BY_ID, &args![audio, handle, id, flags]);
                let frequency = adjust_frequency(e, this, reference, has_value, frequency);
                set_position(e, handle, at);
                follow(e, handle, reference);
                finish_note_sound(e, handle, frequency);
                frequency
            });
        }
        9 | 10 => {
            e.call(PLAY_EFFECT_NOTE, &args![reference, u32::from(note_type)]);
        }
        11 => {
            if sequence != 0 {
                if let Some(value) = text_key_value(e, sequence, note, 0x12) {
                    let strength = x87_store32(value);
                    if x87_load32(strength) > e.global::<f64>(ZERO) {
                        let point = e.vcall(reference, 0x1f4, &[]).u32();
                        e.call(SHAKE_CAMERA, &args![x87_quiet_bits(strength), point]);
                    }
                }
            }
        }
        12 => {
            let volume = match text_key_value(e, sequence, note, 0x11) {
                Some(value) => x87_store32(value),
                None => 1f32.to_bits(),
            };
            let object = e.vcall(reference, 0x1d0, &[]).u32();
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            e.call(
                STOP_MOVING_SOUNDS,
                &args![audio, object, x87_quiet_bits(volume), 0x4000_0000u32],
            );
        }
        13 => {
            let player = e.global::<u32>(PLAYER_POINTER);
            e.call(PLAY_IMPACT_HIT_SHADER, &args![player, reference]);
        }
        16 => {
            if !e.vcall(reference, 0x100, &[]).bool() {
                return frequency;
            }
            let weapon = e.call(CURRENT_WEAPON, &args![reference]).u32();
            if weapon == 0 {
                return frequency;
            }
            let sound = e.call(WEAPON_NOTE_SOUND, &args![weapon]).u32();
            if sound == 0 {
                return frequency;
            }
            e.with_stack(0x10, |e, handle| {
                let handle = handle.addr();
                let flags = player_flag(e, reference) | 0x4000_0102;
                let id = e.call(FORM_ID, &args![sound]).u32();
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                e.call(HANDLE_BY_ID, &args![audio, handle, id, flags]);
                let at = position(e, reference);
                set_position(e, handle, at);
                follow(e, handle, reference);
                finish_note_sound(e, handle, frequency);
            });
        }
        17..=22 => {
            if !e.vcall(reference, 0x100, &[]).bool() {
                return frequency;
            }
            let armor = e.call(ARMOR_BEING_WORN, &args![reference, 2u32]).u32();
            if armor == 0 {
                return frequency;
            }
            let armor_form = e.call(WORD_AT, &args![armor]).u32();
            let note_type = u32::from(e.mem.u8(note + 4));
            let sound = e
                .call(ARMOR_NOTE_SOUND, &args![armor_form, note_type])
                .u32();
            let at = position(e, reference);
            if sound != 0 && sound_may_play(e, sound, at) {
                e.with_stack(0x20, |e, locals| {
                    let handle = locals.addr();
                    let temporary = handle + 0x10;
                    e.call(HANDLE_CONSTRUCT, &args![handle]);
                    if reference == e.global::<u32>(PLAYER_POINTER) {
                        let id = e.call(FORM_ID, &args![sound]).u32();
                        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                        let made = e
                            .call(HANDLE_BY_ID, &args![audio, temporary, id, 0x4020_0101u32])
                            .u32();
                        e.call(HANDLE_ASSIGN, &args![handle, made]);
                        e.call(HANDLE_DESTRUCT, &args![temporary]);
                    } else {
                        let id = e.call(FORM_ID, &args![sound]).u32();
                        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                        let made = e
                            .call(HANDLE_BY_ID, &args![audio, temporary, id, 0x4000_0102u32])
                            .u32();
                        e.call(HANDLE_ASSIGN, &args![handle, made]);
                        e.call(HANDLE_DESTRUCT, &args![temporary]);
                        set_position(e, handle, at);
                        follow(e, handle, reference);
                    }
                    finish_note_sound(e, handle, frequency);
                });
            }
            if armor != 0 {
                e.call(ARMOR_RELEASE, &args![armor, 1u32]);
            }
        }
        _ => {}
    }
    frequency
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_2b60, fn_005f2b60(Ptr, u32, f32, f32, u32))]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    const REFERENCE_VTABLE: u32 = 0x0300_0000;
    const FOLLOW_OBJECT: u32 = 0x7777;

    /// Notes (time, byte, word at +8, sound) behind `this`, a reference
    /// at (1, 2, 3) that is an actor, and the doubles every path uses.
    fn engine(notes: &[(f32, u8, u32, u32)]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.map(0x0101_e000, 0x1000);
        e.set_global(VOLUME_SCALE, 255.0f64);
        e.map(0x0102_0000, 0x1000);
        e.set_global(NOTE_SOUND_VOLUME, 0.75f32);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, 0x0aaa_0000u32);
        let records = e.mem.alloc(0x10 * notes.len() as u32 + 0x10);
        for (i, (time, byte, packed, sound)) in notes.iter().enumerate() {
            let note = records + 0x10 * i as u32;
            e.mem.set_f32(note, *time);
            e.mem.set_u8(note + 4, *byte);
            e.mem.set_u32(note + 8, *packed);
            e.mem.set_u32(note + 0xc, *sound);
        }
        let count = notes.len() as u32;
        e.register_double(NOTE_COUNT, move |_, _| word(count));
        e.register_double(NOTE_AT, move |_, a| word(records + 0x10 * a[1]));
        e.register(GROUP_NUMBER, |_, _| word(5));
        let reference = e.mem.alloc(0x50);
        e.put_vtable(REFERENCE_VTABLE, &[0; 0x100]);
        for (slot, target) in [
            (0x100u32, 0x0300_1000u32),
            (0x1d0, 0x0300_2000),
            (0x1e4, 0x0300_3000),
            (0x1f4, 0x0300_4000),
            (0x21c, 0x0300_5000),
        ] {
            e.mem.set_u32(REFERENCE_VTABLE + slot, target);
        }
        e.mem.set_u32(reference, REFERENCE_VTABLE);
        e.mem.set_f32(reference + 0x40, 1.0);
        e.mem.set_f32(reference + 0x44, 2.0);
        e.mem.set_f32(reference + 0x48, 3.0);
        e.register(0x0300_1000, |_, _| word(1));
        e.register(0x0300_2000, |_, _| word(FOLLOW_OBJECT));
        e.register(0x0300_3000, |_, _| word(0));
        e.register(0x0300_4000, |_, a| word(a[0] + 0x40));
        e.register(0x0300_5000, |_, _| word(1));
        e.register(AUDIO_INSTANCE, |_, _| word(0x0bbb_0000));
        for callee in [
            HANDLE_BY_FILE_NAME,
            HANDLE_BY_ID,
            HANDLE_SET_POSITION,
            HANDLE_SET_VOLUME,
            HANDLE_SET_OBJECT_TO_FOLLOW,
            HANDLE_SET_MOD_FREQUENCY,
            HANDLE_PLAY,
            HANDLE_CONSTRUCT,
            HANDLE_ASSIGN,
            HANDLE_DESTRUCT,
            PLAY_FOOTSTEP,
            PLAY_EFFECT_NOTE,
            SHAKE_CAMERA,
            STOP_MOVING_SOUNDS,
            PLAY_IMPACT_HIT_SHADER,
            ARMOR_RELEASE,
        ] {
            e.register(callee, nothing);
        }
        e.register(FORM_ID, |_, a| word(a[0] + 0x10_0000));
        e.register(SOUND_MAY_PLAY, |_, _| word(1));
        (e, reference)
    }

    fn run(e: &mut Engine, reference: u32, from: f32, to: f32, sequence: u32) {
        e.call(
            0x005f_2b60,
            &args![0x0444_0000u32, reference, from, to, sequence],
        );
    }

    #[test]
    fn notes_play_inside_the_window_and_around_a_wrap() {
        let notes = [(0.1, 0, 0, 0), (0.5, 1, 0, 0), (0.9, 2, 0, 0)];
        let (mut e, reference) = engine(&notes);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.2, 0.5, 0);
        assert_eq!(calls_to(&e, PLAY_FOOTSTEP), vec![vec![reference, 1]]);
        // From 0.8 back to 0.2: the notes after 0.8 and up to 0.2.
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.8, 0.2, 0);
        assert_eq!(
            calls_to(&e, PLAY_FOOTSTEP),
            vec![vec![reference, 0], vec![reference, 2]]
        );
        // An empty window plays nothing.
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.5, 0.5, 0);
        assert!(calls_to(&e, PLAY_FOOTSTEP).is_empty());
    }

    fn sound_data(e: &mut Engine, looping: bool) {
        e.register_double(SOUND_DATA, move |e, a| {
            e.mem.set_u32(a[1] + 4, if looping { 0x40 } else { 0 });
            word(a[1])
        });
        e.register(SOUND_FILE_NAME, |_, _| word(0x0ccc_0000));
    }

    #[test]
    fn a_sound_note_plays_by_file_name() {
        let (mut e, reference) = engine(&[(0.5, 51, 2f32.to_bits(), 0x0900_0000)]);
        sound_data(&mut e, true);
        e.register(SOUND_EXTRA_FLAGS, |_, _| word(0));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        let made = calls_to(&e, HANDLE_BY_FILE_NAME);
        assert_eq!(made.len(), 1);
        let handle = made[0][1];
        assert_eq!(
            made[0],
            vec![0x0bbb_0000, handle, 0x0ccc_0000, 0x4000_0101, 0x0900_0000]
        );
        assert_eq!(
            calls_to(&e, SOUND_MAY_PLAY),
            vec![vec![
                0x0900_0000,
                1f32.to_bits(),
                2f32.to_bits(),
                3f32.to_bits(),
                1f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&e, HANDLE_SET_POSITION),
            vec![vec![handle, 1f32.to_bits(), 2f32.to_bits(), 3f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, HANDLE_SET_VOLUME),
            vec![vec![handle, 0.2f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, HANDLE_SET_OBJECT_TO_FOLLOW),
            vec![vec![handle, FOLLOW_OBJECT]]
        );
        assert_eq!(
            calls_to(&e, HANDLE_SET_MOD_FREQUENCY),
            vec![vec![handle, 2f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(calls_to(&e, HANDLE_DESTRUCT), vec![vec![handle]]);
    }

    #[test]
    fn a_sound_note_for_the_player_with_extra_flags() {
        let (mut e, reference) = engine(&[(0.5, 255, 1f32.to_bits(), 0x0900_0000)]);
        e.set_global(PLAYER_POINTER, reference);
        sound_data(&mut e, false);
        e.register(SOUND_EXTRA_FLAGS, |_, _| word(0x8));
        e.register(PLAYER_TEST, |_, _| word(0));
        // The speed factor of groups 0xb1..0xc7.
        e.register(GROUP_NUMBER, |_, _| word(0xb1));
        e.register(0x0300_3000, |_, _| word(0x55));
        e.register(SPEED_FACTOR, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(SOUND_MAY_PLAY, |_, _| word(0));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        // Extra flags skip the may-play test.
        assert!(calls_to(&e, SOUND_MAY_PLAY).is_empty());
        let made = calls_to(&e, HANDLE_BY_FILE_NAME);
        assert_eq!(
            made[0][3],
            2 | 0x100 | 0x4_0000 | 0x20_0000 | 0x8 | 0x4000_0000
        );
        assert_eq!(calls_to(&e, SPEED_FACTOR), vec![vec![0x55]]);
        assert_eq!(
            calls_to(&e, HANDLE_SET_MOD_FREQUENCY)[0][1],
            1.5f32.to_bits()
        );
        // The extra flags make the play wait.
        assert_eq!(calls_to(&e, HANDLE_PLAY)[0][1], 1);
    }

    #[test]
    fn a_sound_note_that_may_not_play_is_skipped() {
        let (mut e, reference) = engine(&[(0.5, 255, 1f32.to_bits(), 0x0900_0000)]);
        sound_data(&mut e, false);
        e.register(SOUND_EXTRA_FLAGS, |_, _| word(0));
        e.register(SOUND_MAY_PLAY, |_, _| word(0));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert!(calls_to(&e, HANDLE_BY_FILE_NAME).is_empty());
    }

    #[test]
    fn creature_notes_pick_the_creature_sound() {
        let (mut e, reference) = engine(&[(0.5, 5, 0, 0), (0.6, 24, 0, 0)]);
        e.register(BASE_FORM, |_, _| word(0x0700_0000));
        e.register(PICK_CREATURE_SOUND, |_, a| word(0x0710_0000 + a[1]));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert_eq!(
            calls_to(&e, PICK_CREATURE_SOUND),
            vec![vec![0x0700_0000, 5], vec![0x0700_0000, 13]]
        );
        let made = calls_to(&e, HANDLE_BY_ID);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][2], 0x0710_0005 + 0x10_0000);
        assert_eq!(made[0][3], 0x4000_0102);
        assert_eq!(calls_to(&e, HANDLE_SET_VOLUME)[0][1], 0.75f32.to_bits());
        assert_eq!(calls_to(&e, HANDLE_PLAY)[0][1], 0);
    }

    #[test]
    fn effect_shader_and_footstep_types() {
        let (mut e, reference) = engine(&[
            (0.1, 9, 0, 0),
            (0.2, 10, 0, 0),
            (0.3, 13, 0, 0),
            (0.4, 7, 0, 0),
            (0.5, 31, 0, 0),
        ]);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert_eq!(
            calls_to(&e, PLAY_EFFECT_NOTE),
            vec![vec![reference, 9], vec![reference, 10]]
        );
        assert_eq!(
            calls_to(&e, PLAY_IMPACT_HIT_SHADER),
            vec![vec![0x0aaa_0000, reference]]
        );
        assert!(calls_to(&e, PLAY_FOOTSTEP).is_empty());
    }

    fn text_keys(e: &mut Engine, count: u32, value: f64) {
        let array = e.mem.alloc(0x40);
        e.register(SEQUENCE_KEYS, |_, a| word(a[0] + 1));
        e.register_double(KEY_ARRAY, move |e, a| {
            e.mem.set_u32(a[1], count);
            word(array)
        });
        e.register(KEY_PART, |_, a| word(a[0]));
        e.register(KEY_TEXT, |_, _| word(0x0d00_0000));
        e.register_double(ATOF, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    #[test]
    fn camera_shake_from_the_text_key() {
        let (mut e, reference) = engine(&[(0.5, 11, 0x0000_0302, 0)]);
        text_keys(&mut e, 1, 2.5);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0x0600_0000);
        assert_eq!(calls_to(&e, SEQUENCE_KEYS), vec![vec![0x0600_0000]]);
        // Index 2 of the key array, text offset 3 + 0x12.
        assert_eq!(calls_to(&e, ATOF), vec![vec![0x0d00_0015]]);
        assert_eq!(
            calls_to(&e, SHAKE_CAMERA),
            vec![vec![2.5f32.to_bits(), reference + 0x40]]
        );
        // No sequence: nothing.
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert!(calls_to(&e, SEQUENCE_KEYS).is_empty());
        // A value of 0 does not shake.
        text_keys(&mut e, 1, 0.0);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0x0600_0000);
        assert!(calls_to(&e, SHAKE_CAMERA).is_empty());
    }

    #[test]
    fn stop_moving_sounds_with_the_text_key_volume() {
        let (mut e, reference) = engine(&[(0.5, 12, 0x0000_0100, 0)]);
        text_keys(&mut e, 1, 0.5);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert_eq!(calls_to(&e, ATOF), vec![vec![0x0d00_0012]]);
        assert_eq!(
            calls_to(&e, STOP_MOVING_SOUNDS),
            vec![vec![
                0x0bbb_0000,
                FOLLOW_OBJECT,
                0.5f32.to_bits(),
                0x4000_0000
            ]]
        );
        // Without keys the volume is 1.
        text_keys(&mut e, 0, 0.5);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert!(calls_to(&e, ATOF).is_empty());
        assert_eq!(calls_to(&e, STOP_MOVING_SOUNDS)[0][2], 1f32.to_bits());
    }

    #[test]
    fn weapon_note_plays_the_weapons_sound() {
        let (mut e, reference) = engine(&[(0.5, 16, 0, 0)]);
        e.register(CURRENT_WEAPON, |_, _| word(0x0500_0000));
        e.register(WEAPON_NOTE_SOUND, |_, _| word(0x0510_0000));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        let made = calls_to(&e, HANDLE_BY_ID);
        assert_eq!(made.len(), 1);
        assert_eq!(&made[0][2..], &[0x0520_0000, 0x4000_0102]);
        assert_eq!(calls_to(&e, HANDLE_SET_POSITION).len(), 1);
        e.register(WEAPON_NOTE_SOUND, |_, _| word(0));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert!(calls_to(&e, HANDLE_BY_ID).is_empty());
    }

    #[test]
    fn armor_note_plays_the_armors_sound() {
        let (mut e, reference) = engine(&[(0.5, 18, 0, 0)]);
        let armor = e.mem.alloc(4);
        e.mem.set_u32(armor, 0x0520_0000);
        e.register_double(ARMOR_BEING_WORN, move |_, _| word(armor));
        e.register(WORD_AT, |e, a| word(e.mem.u32(a[0])));
        e.register(ARMOR_NOTE_SOUND, |_, a| word(a[0] + a[1]));
        e.register(HANDLE_BY_ID, |_, a| word(a[1]));
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert_eq!(calls_to(&e, ARMOR_BEING_WORN), vec![vec![reference, 2]]);
        assert_eq!(calls_to(&e, ARMOR_NOTE_SOUND), vec![vec![0x0520_0000, 18]]);
        let made = calls_to(&e, HANDLE_BY_ID);
        assert_eq!(made[0][2..], [0x0520_0012 + 0x10_0000, 0x4000_0102]);
        let handle = calls_to(&e, HANDLE_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, HANDLE_ASSIGN), vec![vec![handle, made[0][1]]]);
        assert_eq!(calls_to(&e, HANDLE_SET_POSITION).len(), 1);
        assert_eq!(calls_to(&e, ARMOR_RELEASE), vec![vec![armor, 1]]);
        // The player's armor sound uses its own flags and no position.
        e.set_global(PLAYER_POINTER, reference);
        e.call_log = Some(vec![]);
        run(&mut e, reference, 0.0, 1.0, 0);
        assert_eq!(calls_to(&e, HANDLE_BY_ID)[0][3], 0x4020_0101);
        assert!(calls_to(&e, HANDLE_SET_POSITION).is_empty());
    }
}

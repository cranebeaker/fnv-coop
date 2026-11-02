//! `BSAudio/run_00aea770` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The listener's virtual tables (RTTI `BSWin32AudioListener`, then its base
/// `BSAudioListener`).
const WIN32_LISTENER_VTABLE: u32 = 0x010a_3b7c;
const LISTENER_VTABLE: u32 = 0x010a_3bb4;
/// `timeGetTime` (import slot).
const TIME_GET_TIME: u32 = 0x00fd_f334;
/// `00439ef0` (`thiscall` on a position, two words: the result vector and another position):
/// the difference of two positions; `00457990` (`thiscall`, no stack words): a vector's
/// length in `ST0`.
const VECTOR_DIFFERENCE: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// The `double` the game scales its units into the listener's (`010a3be8`).
const UNIT_SCALE: u32 = 0x010a_3be8;
/// The heap the deleting destructor frees into (`011f6238`) and its free (`00aa4060`,
/// `thiscall`, one word).
const HEAP: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;

// Translated from 00aea770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aea770 - the FS:[0] exception frame (C++ unwinding during destruction) is not translated
/// Unnamed in the engine map (`run_00aea770`): the `BSWin32AudioListener` destructor body
/// (`thiscall`, no stack words): its virtual table, the `DirectSound` listener at +0x48
/// released (`IUnknown` slot 8) and cleared, then the `BSAudioListener` virtual table and +0x44
/// cleared.
pub fn fn_00aea770(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WIN32_LISTENER_VTABLE);
    let listener = e.mem.u32(base.wrapping_add(0x48));
    if listener != 0 {
        e.vcall(listener, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x48), 0);
    }
    e.mem.set_u32(base, LISTENER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x44), 0);
}

// Translated from 00aea810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): moves the listener to `position` (`thiscall`,
/// one word: a three-float position). The speed at +0x38 becomes the distance from the old
/// position (+4, `00439ef0` then `00457990`) over the milliseconds since the last move
/// (+0x34) times 1000, times the unit scale (`010a3be8`); then the position and the time
/// (`timeGetTime`) are kept.
pub fn fn_00aea810(e: &mut Engine, this: Ptr, position: Ptr) {
    let base = this.addr();
    let now = e.call(TIME_GET_TIME, &args![]).u32();
    let distance = e.with_stack(12, |e, difference| {
        e.call(
            VECTOR_DIFFERENCE,
            &args![position, difference, base.wrapping_add(4)],
        );
        e.call(VECTOR_LENGTH, &args![difference]).f64()
    });
    // `IMUL` by 1000, then `FILD qword` of the product with a zero high word.
    let milliseconds = now
        .wrapping_sub(e.mem.u32(base.wrapping_add(0x34)))
        .wrapping_mul(1000);
    let speed = x87_div(distance, f64::from(milliseconds));
    let speed = x87_mul(speed, e.global::<f64>(UNIT_SCALE));
    e.mem.set_u32(base.wrapping_add(0x38), x87_store32(speed));
    let source = position.addr();
    for word in 0..3u32 {
        let value = e.mem.u32(source.wrapping_add(word * 4));
        e.mem.set_u32(base.wrapping_add(4 + word * 4), value);
    }
    e.mem.set_u32(base.wrapping_add(0x34), now);
}

// Translated from 00aea890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): the address of the listener's position (+4)
/// (`thiscall`, no stack words).
pub fn fn_00aea890(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(4)
}

/// A `float` field scaled by the unit scale and stored as a `float` (`FLD`, `FMUL double`,
/// `FSTP`), as the bits pushed.
fn scaled(e: &Engine, at: u32) -> u32 {
    let value = x87_load32(e.mem.u32(at));
    x87_store32(x87_mul(value, e.global::<f64>(UNIT_SCALE)))
}

// Translated from 00aea8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): hands the listener's state to the
/// `DirectSound` listener at +0x48, when there is one (`thiscall`, no stack words): its
/// position (slot 0x38, deferred) from +4, +0xc, +8 each times the unit scale, its
/// orientation (slot 0x34, deferred) from +0x58, +0x60, +0x5c, +0x4c, +0x54, +0x50, then
/// the commit (slot 0x44).
pub fn fn_00aea8b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x48)) == 0 {
        return;
    }
    // Pushed last to first: +8, +0xc, +4.
    let y = scaled(e, base.wrapping_add(8));
    let z = scaled(e, base.wrapping_add(0xc));
    let x = scaled(e, base.wrapping_add(4));
    let listener = e.mem.u32(base.wrapping_add(0x48));
    e.vcall(listener, 0x38, &args![x, z, y, 1u32]);
    // `FLD`/`FSTP` copies: a signalling NaN arrives quieted.
    let copy = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(base.wrapping_add(offset)));
    let words = [
        copy(e, 0x58),
        copy(e, 0x60),
        copy(e, 0x5c),
        copy(e, 0x4c),
        copy(e, 0x54),
        copy(e, 0x50),
    ];
    let listener = e.mem.u32(base.wrapping_add(0x48));
    e.vcall(
        listener,
        0x34,
        &args![words[0], words[1], words[2], words[3], words[4], words[5], 1u32],
    );
    let listener = e.mem.u32(base.wrapping_add(0x48));
    e.vcall(listener, 0x44, &args![]);
}

// Translated from 00aea990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): keeps the listener's orientation
/// (`thiscall`, two words: two three-float vectors): `up` at +0x4c, then `front` at +0x58.
pub fn fn_00aea990(e: &mut Engine, this: Ptr, front: Ptr, up: Ptr) {
    let base = this.addr();
    for word in 0..3u32 {
        let value = e.mem.u32(up.addr().wrapping_add(word * 4));
        e.mem.set_u32(base.wrapping_add(0x4c + word * 4), value);
    }
    for word in 0..3u32 {
        let value = e.mem.u32(front.addr().wrapping_add(word * 4));
        e.mem.set_u32(base.wrapping_add(0x58 + word * 4), value);
    }
}

// Translated from 00aea9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): hands `factor` to the `DirectSound` listener at
/// +0x48 when there is one (slot 0x3c, deferred) (`thiscall`, one word; the float is copied
/// through `FLD`/`FSTP`).
pub fn fn_00aea9d0(e: &mut Engine, this: Ptr, factor: f32) {
    let listener = e.mem.u32(this.addr().wrapping_add(0x48));
    if listener != 0 {
        let factor = x87_quiet_bits(factor.to_bits());
        e.vcall(listener, 0x3c, &args![factor, 1u32]);
    }
}

// Translated from 00aeaa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea770`): the listener's deleting destructor
/// (`thiscall`, one word of flags): the destructor body `00aea770`, then with flag bit 0
/// the block freed into the heap at `011f6238` (`00aa4060`). Answers `this`.
pub fn fn_00aeaa80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00aea770(e, this);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP, this]);
    }
    this
}

// Translated from 00aea800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +4 of the `BSWin32AudioListener` virtual table
/// (`010a3b7c`; `Init` in the Xbox PDB's `BSAudioListener` order, not confirmed) (`thiscall`,
/// no stack words): does nothing.
pub fn fn_00aea800(_e: &mut Engine, _this: Ptr) {}

// Translated from 00aeaa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x20 of the `BSWin32AudioListener` virtual table
/// (`SetVelocity` in the Xbox PDB's order, not confirmed) (`thiscall`, one word: three
/// floats). Copies them to +0x28 (`vVelocity` in the Xbox PDB layout) as words, then, when
/// there is a `DirectSound` listener at +0x48, hands them to its slot +0x40 with 1 (the
/// object first; each float copied through `FLD`/`FSTP`, a signalling NaN quieted).
pub fn fn_00aeaa10(e: &mut Engine, this: Ptr, velocity: Ptr) {
    let base = this.addr();
    let source = velocity.addr();
    for i in 0..3u32 {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(base.wrapping_add(0x28 + 4 * i), word);
    }
    let listener = e.mem.u32(base.wrapping_add(0x48));
    if listener != 0 {
        let z = x87_quiet_bits(e.mem.u32(source.wrapping_add(8)));
        let y = x87_quiet_bits(e.mem.u32(source.wrapping_add(4)));
        let x = x87_quiet_bits(e.mem.u32(source));
        e.vcall(listener, 0x40, &args![x, y, z, 1u32]);
    }
}

// Translated from 00aeaab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it): the `BSAudioListener` destructor body
/// (`thiscall`, no stack words): its virtual table (`010a3bb4`) and +0x44 cleared, as the end
/// of `00aea770` does.
pub fn fn_00aeaab0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(0x44), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_a800, fn_00aea800(Ptr)),
        entry!(0x00ae_aa10, fn_00aeaa10(Ptr, Ptr)),
        entry!(0x00ae_aab0, fn_00aeaab0(Ptr)),
        entry!(0x00ae_a770, fn_00aea770(Ptr)),
        entry!(0x00ae_a810, fn_00aea810(Ptr, Ptr)),
        entry!(0x00ae_a890, fn_00aea890(Ptr) -> u32),
        entry!(0x00ae_a8b0, fn_00aea8b0(Ptr)),
        entry!(0x00ae_a990, fn_00aea990(Ptr, Ptr, Ptr)),
        entry!(0x00ae_a9d0, fn_00aea9d0(Ptr, f32)),
        entry!(0x00ae_aa80, fn_00aeaa80(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b049_tests {
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

    /// A listener object (0x64 bytes) with a `DirectSound` listener at +0x48 whose virtual
    /// table at 0x7000 has slot `i` at `0x8000 + 4 * i`.
    fn listener_engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x20).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        for slot in [0x08u32, 0x34, 0x38, 0x3c, 0x44] {
            e.register(0x8000 + slot, |_, _| Ret::default());
        }
        let sound = e.mem.alloc(4);
        e.mem.set_u32(sound, 0x7000);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + 0x48, sound);
        e.map(0x010a_3000, 0x1000);
        e.set_global(UNIT_SCALE, 0.5f64);
        (e, this, sound)
    }

    #[test]
    fn fn_00aea770_releases_the_sound_listener() {
        let (mut e, this, sound) = listener_engine();
        e.mem.set_u32(this + 0x44, 9);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a770, &args![this]);
        assert_eq!(calls_to(&e, 0x8008), vec![vec![sound]]);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        // Without one nothing is released.
        e.call_log = Some(vec![]);
        e.call(0x00ae_a770, &args![this]);
        assert!(calls_to(&e, 0x8008).is_empty());
    }

    #[test]
    fn fn_00aea810_measures_the_speed_and_keeps_the_position() {
        let (mut e, this, _) = listener_engine();
        e.register(TIME_GET_TIME, |_, _| 3000u32.into_ret());
        e.register(VECTOR_DIFFERENCE, |_, _| Ret::default());
        e.register(VECTOR_LENGTH, |_, _| Ret {
            st0: 400.0,
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x34, 2000);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a810, &args![this, position]);
        // 400 / (1000 ms * 1000) * 0.5.
        assert_eq!(
            e.mem.f32(this + 0x38),
            (400.0f64 / 1_000_000.0 * 0.5) as f32
        );
        assert_eq!(e.mem.f32(this + 4), 1.0);
        assert_eq!(e.mem.f32(this + 0xc), 3.0);
        assert_eq!(e.mem.u32(this + 0x34), 3000);
        let difference = &calls_to(&e, VECTOR_DIFFERENCE)[0];
        assert_eq!(difference[0], position);
        assert_eq!(difference[2], this + 4);
    }

    #[test]
    fn fn_00aea890_answers_the_position() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ae_a890, &args![0x1000u32]).u32(), 0x1004);
    }

    #[test]
    fn fn_00aea8b0_hands_position_and_orientation_over() {
        let (mut e, this, sound) = listener_engine();
        e.mem.set_f32(this + 4, 2.0);
        e.mem.set_f32(this + 8, 4.0);
        e.mem.set_f32(this + 0xc, 6.0);
        for (i, offset) in (0x4c..0x64).step_by(4).enumerate() {
            e.mem.set_f32(this + offset, i as f32);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ae_a8b0, &args![this]);
        assert_eq!(
            calls_to(&e, 0x8038),
            vec![vec![
                sound,
                1.0f32.to_bits(),
                3.0f32.to_bits(),
                2.0f32.to_bits(),
                1
            ]]
        );
        // +0x58, +0x60, +0x5c, +0x4c, +0x54, +0x50 hold 3, 5, 4, 0, 2, 1.
        let expected: Vec<u32> = [3.0f32, 5.0, 4.0, 0.0, 2.0, 1.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        let orientation = &calls_to(&e, 0x8034)[0];
        assert_eq!(&orientation[1..7], expected.as_slice());
        assert_eq!(orientation[7], 1);
        assert_eq!(calls_to(&e, 0x8044), vec![vec![sound]]);
        // Without a sound listener nothing is called.
        e.mem.set_u32(this + 0x48, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a8b0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00aea990_keeps_up_then_front() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x64);
        let front = e.mem.alloc(12);
        let up = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(front + 4 * i, 10 + i);
            e.mem.set_u32(up + 4 * i, 20 + i);
        }
        e.call(0x00ae_a990, &args![this, front, up]);
        assert_eq!(e.mem.u32(this + 0x4c), 20);
        assert_eq!(e.mem.u32(this + 0x54), 22);
        assert_eq!(e.mem.u32(this + 0x58), 10);
        assert_eq!(e.mem.u32(this + 0x60), 12);
    }

    #[test]
    fn fn_00aea9d0_hands_the_factor_over() {
        let (mut e, this, sound) = listener_engine();
        e.call_log = Some(vec![]);
        e.call(0x00ae_a9d0, &args![this, 0.25f32]);
        assert_eq!(
            calls_to(&e, 0x803c),
            vec![vec![sound, 0.25f32.to_bits(), 1]]
        );
        // A signalling NaN arrives quieted.
        e.call_log = Some(vec![]);
        e.call(0x00ae_a9d0, &args![this, 0x7f80_0001u32]);
        assert_eq!(calls_to(&e, 0x803c)[0][1], 0x7fc0_0001);
        e.mem.set_u32(this + 0x48, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a9d0, &args![this, 0.25f32]);
        assert!(calls_to(&e, 0x803c).is_empty());
    }

    #[test]
    fn fn_00aeaa80_frees_only_with_flag_bit_0() {
        let (mut e, this, _) = listener_engine();
        e.register(HEAP_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_aa80, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        e.call(0x00ae_aa80, &args![this, 1u32]);
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![HEAP, this]]);
    }
}

#[cfg(test)]
mod n082_tests {
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
    fn fn_00aea800_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00ae_a800, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00aeaa10_keeps_the_velocity_and_hands_it_over() {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x20).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        e.register(0x8040, |_, _| Ret::default());
        let sound = e.mem.alloc(4);
        e.mem.set_u32(sound, 0x7000);
        let this = e.mem.alloc(0x64);
        let velocity = e.mem.alloc(12);
        e.mem.set_f32(velocity, 1.0);
        e.mem.set_f32(velocity + 4, -2.0);
        e.mem.set_u32(velocity + 8, 0x7f80_0001);
        // Without a sound listener only the copy (as words, the NaN untouched).
        e.call_log = Some(vec![]);
        e.call(0x00ae_aa10, &args![this, velocity]);
        assert_eq!(e.mem.f32(this + 0x28), 1.0);
        assert_eq!(e.mem.f32(this + 0x2c), -2.0);
        assert_eq!(e.mem.u32(this + 0x30), 0x7f80_0001);
        assert!(calls_to(&e, 0x8040).is_empty());
        e.mem.set_u32(this + 0x48, sound);
        e.call_log = Some(vec![]);
        e.call(0x00ae_aa10, &args![this, velocity]);
        assert_eq!(
            calls_to(&e, 0x8040),
            vec![vec![
                sound,
                1.0f32.to_bits(),
                (-2.0f32).to_bits(),
                0x7fc0_0001,
                1
            ]]
        );
    }

    #[test]
    fn fn_00aeaab0_resets_to_the_base_listener() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this, WIN32_LISTENER_VTABLE);
        e.mem.set_u32(this + 0x44, 5);
        e.mem.set_u32(this + 0x48, 9);
        e.call(0x00ae_aab0, &args![this]);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 9);
    }
}

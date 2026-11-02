//! `(unplaced)/run_00e80570` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DShaderInterface::Initialize` (Xbox PDB, `00e81a90`; `thiscall`, no stack words).
const SHADER_INTERFACE_INITIALIZE: u32 = 0x00e8_1a90;
/// A global object pointer read by `00e80580` and `00e80590` (its class is unconfirmed).
const GLOBAL_OBJECT: u32 = 0x0126_f74c;
/// The vtable slot of that object `00e80590` calls with one word.
const GLOBAL_OBJECT_SLOT: u32 = 0xc;
/// The two functions `00e80630` runs (no Xbox names; neither takes a word).
const RELEASE_FIRST: u32 = 0x00e8_03f0;
const RELEASE_SECOND: u32 = 0x00e7_ed80;

// Translated from 00e80570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump thunk to `NiD3DShaderInterface::Initialize`
/// (Xbox PDB, `00e81a90`) with the same `this`; answers what it answers.
pub fn fn_00e80570(e: &mut Engine, this: u32) -> u32 {
    e.call(SHADER_INTERFACE_INITIALIZE, &args![this]).u32()
}

// Translated from 00e80580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable method; `this` is not read): 1 when the global
/// object pointer `0126f74c` is set, else 0.
pub fn fn_00e80580(e: &mut Engine, _this: u32) -> u32 {
    u32::from(e.mem.u32(GLOBAL_OBJECT) != 0)
}

// Translated from 00e80590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable method, `thiscall`): when the global object
/// `0126f74c` is set, calls its vtable slot +0xC with the word at `this` +0x38. Answers 0.
pub fn fn_00e80590(e: &mut Engine, this: u32) -> u32 {
    let object = e.mem.u32(GLOBAL_OBJECT);
    if object != 0 {
        let word = e.mem.u32(this + 0x38);
        e.vcall(object, GLOBAL_OBJECT_SLOT, &args![word]);
    }
    0
}

// Translated from 00e80630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `00e803f0`, then tail-jumps to `00e7ed80` (neither
/// takes a word).
pub fn fn_00e80630(e: &mut Engine) {
    e.call(RELEASE_FIRST, &[]);
    e.call(RELEASE_SECOND, &[]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_0570, fn_00e80570(u32) -> u32),
        entry!(0x00e8_0580, fn_00e80580(u32) -> u32),
        entry!(0x00e8_0590, fn_00e80590(u32) -> u32),
        entry!(0x00e8_0630, fn_00e80630()),
        entry!(0x00e8_05b0, fn_00e805b0(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_05f0, fn_00e805f0(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_0610, fn_00e80610(u32)),
    ]
}

/// The global object's slot `00e805b0` forwards to.
const GLOBAL_OBJECT_FORWARD_SLOT: u32 = 0x8;
/// `NiD3DRenderStateGroup::RestoreRenderStates` (engine map, `00e7ef40`; thiscall).
const RESTORE_RENDER_STATES: u32 = 0x00e7_ef40;

// Translated from 00e805b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of many shader classes; `RET 0x1c`): forwards
/// its seven words, then a 0, to slot +8 of the global object at `0126f74c`, passing on
/// its answer. `this` is not read.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e805b0(
    e: &mut Engine,
    _this: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
) -> u32 {
    let object = e.mem.u32(GLOBAL_OBJECT);
    e.vcall(
        object,
        GLOBAL_OBJECT_FORWARD_SLOT,
        &args![first, second, third, fourth, fifth, sixth, seventh, 0u32],
    )
    .u32()
}

// Translated from 00e805f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word; `RET 0x1c`, its seven words not read): when the
/// render-state group at +0x2c exists, restores it
/// (`NiD3DRenderStateGroup::RestoreRenderStates`). Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e805f0(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    let group = e.mem.u32(this + 0x2c);
    if group != 0 {
        e.call(RESTORE_RENDER_STATES, &args![group]);
    }
    0
}

// Translated from 00e80610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of many shader classes): clears the global
/// object pointer at `0126f74c` and the word at +0x38.
pub fn fn_00e80610(e: &mut Engine, this: u32) {
    e.mem.set_u32(GLOBAL_OBJECT, 0);
    e.mem.set_u32(this + 0x38, 0);
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn forwards_seven_words_and_a_zero_to_the_global_object() {
        let mut e = Engine::new();
        e.map(GLOBAL_OBJECT, 4);
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, 0x0079_0008);
        e.register(0x0079_0008, |_, a| a.iter().sum::<u32>().into_ret());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(GLOBAL_OBJECT, object);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00e8_05b0,
            &args![0x1000u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(r.u32(), object + 28);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (0x0079_0008, vec![object, 1, 2, 3, 4, 5, 6, 7, 0])
        );
    }

    #[test]
    fn restores_the_render_states_when_there_is_a_group() {
        let mut e = Engine::new();
        e.register(RESTORE_RENDER_STATES, |_, _| 5u32.into_ret());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_05f0, &words).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 0x2c, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_05f0, &words).u32(), 0);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (RESTORE_RENDER_STATES, vec![0x6000])
        );
    }

    #[test]
    fn clears_the_global_and_the_word() {
        let mut e = Engine::new();
        e.map(GLOBAL_OBJECT, 4);
        e.mem.set_u32(GLOBAL_OBJECT, 9);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00e8_0610, &args![this]);
        assert_eq!(e.mem.u32(GLOBAL_OBJECT), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
    }
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
    fn fn_00e80570_passes_this_on_and_answers_the_result() {
        let mut e = Engine::new();
        e.register(SHADER_INTERFACE_INITIALIZE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0570, &[0x1234]).u32(), 1);
        assert_eq!(
            calls_to(&e, SHADER_INTERFACE_INITIALIZE),
            vec![vec![0x1234]]
        );
    }

    #[test]
    fn fn_00e80580_tests_the_global_pointer() {
        let mut e = Engine::new();
        e.mem.map(GLOBAL_OBJECT & !0xfff, 0x1000);
        assert_eq!(e.call(0x00e8_0580, &[0]).u32(), 0);
        e.mem.set_u32(GLOBAL_OBJECT, 0x4000);
        assert_eq!(e.call(0x00e8_0580, &[0]).u32(), 1);
    }

    #[test]
    fn fn_00e80590_calls_the_global_object_with_the_field() {
        let mut e = Engine::new();
        e.mem.map(GLOBAL_OBJECT & !0xfff, 0x1000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 0x77);
        e.call_log = Some(vec![]);
        // No object: nothing called.
        assert_eq!(e.call(0x00e8_0590, &[this]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        let object = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0x0090_1000]);
        e.mem.set_u32(object, 0x0090_0000);
        e.mem.set_u32(GLOBAL_OBJECT, object);
        e.register(0x0090_1000, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_0590, &[this]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0090_1000), vec![vec![object, 0x77]]);
    }

    #[test]
    fn fn_00e80630_runs_both_in_order() {
        let mut e = Engine::new();
        e.register(RELEASE_FIRST, |_, _| Ret::default());
        e.register(RELEASE_SECOND, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_0630, &[]);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![0x00e8_0630, RELEASE_FIRST, RELEASE_SECOND]);
    }
}

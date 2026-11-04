//! `(unplaced)/run_006214d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack words, no engine-map name): the constructor run on each
/// 16-byte vector-like member and local.
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
/// `00483710` (`thiscall`, unit `fallout shared/tesform.cpp`): run by `fn_006219c0` on the
/// object vtable slot +0x94 answers.
const FORM_CALL: u32 = 0x0048_3710;
/// `0052ce20` (`thiscall`, one float; unit `fallout shared/tesimagespacemodifier.cpp`).
const SET_FLOAT: u32 = 0x0052_ce20;
/// `bhkCharacterProxy::operatorP` (engine map, `004ae750`; `thiscall`): answers the phantom.
const CHARACTER_PROXY_PHANTOM: u32 = 0x004a_e750;
/// `00a29680` (`thiscall`, no engine-map name): called before and after the phantom move.
const PROXY_AROUND: u32 = 0x00a2_9680;
/// `hkpShapePhantom::setPosition` (engine map, `00c9e990`; `thiscall`, (position, float)).
const PHANTOM_SET_POSITION: u32 = 0x00c9_e990;

/// Callees of `fn_006214d0`, all named by the engine map where it has a name.
const CAMERA_CASTER_TEST: u32 = 0x0062_1240;
const OBJECT_011DEA3C: u32 = 0x011d_ea3c;
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const TES_0045BB80: u32 = 0x0045_bb80;
const MULTIBOUND_00439EF0: u32 = 0x0043_9ef0;
const TES_00457990: u32 = 0x0045_7990;
const SETTING_OBJECT: u32 = 0x011c_c188;
const EFFECT_ITEM_00403E20: u32 = 0x0040_3e20;
const COPY_POSITION: u32 = 0x004a_3e00;
const GET_HK_POSITION: u32 = 0x0062_1a40;
const SET_FROM_HK: u32 = 0x004a_3f10;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const GET_MEMBER: u32 = 0x0055_9450;
const TES_00458700: u32 = 0x0045_8700;
const TES_004586D0: u32 = 0x0045_86d0;
const ELEMENT_ADDRESS: u32 = 0x0056_0d30;
const CALL_004A3E90: u32 = 0x004a_3e90;
const POINT_COLLECTOR_CONSTRUCT: u32 = 0x0062_0ff0;
const MODEL_LOADER_0043B560: u32 = 0x0043_b560;
const CALL_00621A90: u32 = 0x0062_1a90;
const MEMBER_AT_10: u32 = 0x0046_0140;
const CALL_00621AD0: u32 = 0x0062_1ad0;
const CELL_00541C80: u32 = 0x0054_1c80;
const EXPLOSION_004FB070: u32 = 0x004f_b070;
const UNLOADED_FORM_BUFFER_CONSTRUCT: u32 = 0x0053_7e90;
const CALL_00407840: u32 = 0x0040_7840;
const CALL_004AE6C0: u32 = 0x004a_e6c0;
const MODEL_LOADER_0043B230: u32 = 0x0043_b230;
const READ_00621B00: u32 = 0x0062_1b00;
const WRITE_00621B20: u32 = 0x0062_1b20;
const CAMERA_CASTER_00621210: u32 = 0x0062_1210;
const REFR_005609F0: u32 = 0x0056_09f0;
const POINT_COLLECTOR_DESTRUCT: u32 = 0x0062_11b0;

// Translated from 006219c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): asks `this` for an object
/// (vtable slot +0x94) and, when there is one, runs `00483710` on it.
pub fn fn_006219c0(e: &mut Engine, this: u32) {
    let object = e.vcall(this, 0x94, &[]).u32();
    if object != 0 {
        e.call(FORM_CALL, &args![object]);
    }
}

// Translated from 00621990 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a constructor shape): runs
/// `006815c0` on `this`, `this` +0x10 and `this` +0x20 and answers `this`.
pub fn fn_00621990(e: &mut Engine, this: u32) -> u32 {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    e.call(VECTOR_CONSTRUCT, &args![this + 0x10]);
    e.call(VECTOR_CONSTRUCT, &args![this + 0x20]);
    this
}

// Translated from 00621930 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a constructor shape): runs
/// `006815c0` on `this` +0x0, +0x4, +0x8, +0xC, +0x10 and +0x14, then `fn_00621990` on
/// `this` +0x20, and answers `this`.
pub fn fn_00621930(e: &mut Engine, this: u32) -> u32 {
    for offset in [0x0, 0x4, 0x8, 0xc, 0x10, 0x14] {
        e.call(VECTOR_CONSTRUCT, &args![this + offset]);
    }
    fn_00621990(e, this + 0x20);
    this
}

// Translated from 006218d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float, `RET 4`): passes the float (copied
/// through `FLD`/`FSTP`, a signalling NaN quieted) to `0052ce20` on `this` +0x8.
pub fn fn_006218d0(e: &mut Engine, this: u32, value: u32) {
    e.call(SET_FLOAT, &args![this + 0x8, x87_quiet_bits(value)]);
}

// Translated from 006219f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): when `this`'s phantom
/// (`bhkCharacterProxy::operatorP`, `004ae750`) is not null, calls `00a29680` on `this`,
/// `hkpShapePhantom::setPosition(position, 0.0)` on the phantom and `00a29680` again.
pub fn fn_006219f0(e: &mut Engine, this: u32, position: u32) {
    let phantom = e.call(CHARACTER_PROXY_PHANTOM, &args![this]).u32();
    if phantom == 0 {
        return;
    }
    e.call(PROXY_AROUND, &args![this]);
    e.call(
        PHANTOM_SET_POSITION,
        &args![phantom, position, 0.0f32.to_bits()],
    );
    e.call(PROXY_AROUND, &args![this]);
}

// Translated from 006218f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`): the byte at `this` + 0x1A0 +
/// `row` * 32 + `column` is an index; answers the word at `this` + 0x9A8 + index * 0x14
/// (a table of function pointers by its use in `fn_006214d0`).
pub fn fn_006218f0(e: &mut Engine, this: u32, row: u32, column: u32) -> u32 {
    let cell = (row << 5).wrapping_add(column);
    let index = u32::from(e.mem.u8(this.wrapping_add(cell).wrapping_add(0x1a0)));
    e.mem.u32(
        this.wrapping_add(index.wrapping_mul(0x14))
            .wrapping_add(0x9a8),
    )
}

/// The frame of `fn_006214d0` (its locals at their `EBP` offsets).
const FRAME_SIZE: u32 = 0x4b0;
/// A local at `EBP - offset` in a frame starting at `base`.
fn local(base: u32, offset: u32) -> u32 {
    base + FRAME_SIZE - offset
}

// Translated from 006214d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words, `RET 0x10`: `subject`, `position`,
/// two floats `first_float` and `second_float`; the stack is realigned to 16 bytes). Builds
/// two 16-byte vector locals (`006815c0`). When `00621240` on `subject` answers true and the
/// global object at `011dea3c` is set and `GetSavedAcquireObject` (`008d8520`) answers an
/// object whose vtable slot +0x6D4 answers non-null: answers whether the float at the
/// address `00403e20` gives for `011cc188` is above the distance computed from that object
/// (`0045bb80`, `00439ef0(position, local, ...)`, `00457990`). Otherwise: copies `position`
/// into the first vector (`004a3e00`) and `subject`'s havok position into the second
/// (`00621a40`, `004a3f10`); when `subject` is the character controller of the global
/// (`009306d0`) takes `this`'s member (`00559450`) and, when the second vector's value
/// (`00458700(.., 2)`, `004586d0`) is below the first's, lowers word 2 of each vector by
/// `004a3e90(second_float)`; otherwise takes the member of `this` +4. Builds a 0x50-byte
/// local (`fn_00621930`, `fn_006218d0(0.0)`) and a point collector (`00620ff0`), then
/// through `subject`'s `00621a90`/`00460140` and `00621ad0` objects fills the local
/// (`00537e90` twice), moves the phantom (`fn_006219f0`) and, when the table of
/// `fn_006218f0` gives a function for the two objects' `0043b230` values, calls it
/// (`cdecl`, four words) with `first_float` set through `00621b20` and restored after;
/// the answer is then `005609f0` of `00621210`'s result. The collector is destroyed
/// (`006211b0`) on that path. Answers the byte (false unless set).
// Translation incomplete: 006214d0 - the FS:[0] exception frame (destroying the point collector on unwinding) and the /GS cookie check 00ec408c are not translated
pub fn fn_006214d0(
    e: &mut Engine,
    this: u32,
    subject: u32,
    position: u32,
    first_float: u32,
    second_float: u32,
) -> bool {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let base = frame.addr();
        let first = local(base, 0x30);
        let second = local(base, 0x40);
        let distance_local = local(base, 0x54);
        let first_scratch = local(base, 0x80);
        let second_scratch = local(base, 0x90);
        let block = local(base, 0xf0);
        let collector = local(base, 0x490);
        let out_byte = local(base, 0x4ad);
        let mut answer = false;
        e.call(VECTOR_CONSTRUCT, &args![first]);
        e.call(VECTOR_CONSTRUCT, &args![second]);

        if e.call(CAMERA_CASTER_TEST, &args![subject]).bool() {
            let process = e.mem.u32(OBJECT_011DEA3C);
            if process != 0 && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![process]).u32() != 0 {
                let process = e.mem.u32(OBJECT_011DEA3C);
                let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![process]).u32();
                if e.vcall(object, 0x6d4, &[]).u32() != 0 {
                    let process = e.mem.u32(OBJECT_011DEA3C);
                    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![process]).u32();
                    let target = e.vcall(object, 0x6d4, &[]).u32();
                    let value = e.call(TES_0045BB80, &args![target]).u32();
                    e.call(MULTIBOUND_00439EF0, &args![position, distance_local, value]);
                    let distance = e.call(TES_00457990, &args![distance_local]).f64();
                    let limit = e.call(EFFECT_ITEM_00403E20, &args![SETTING_OBJECT]).u32();
                    let limit = x87_load32(e.mem.u32(limit));
                    // FCOMP; TEST AH,0x41; JNZ: set only when the float is above (ordered).
                    return limit > distance;
                }
            }
        }

        e.call(COPY_POSITION, &args![first, position]);
        let hk_position = e.call(GET_HK_POSITION, &args![subject]).u32();
        e.call(SET_FROM_HK, &args![second, hk_position]);
        let process = e.mem.u32(OBJECT_011DEA3C);
        let controller = e.call(GET_CHAR_CONTROLLER, &args![process]).u32();
        let member = if subject == controller {
            let member = e.call(GET_MEMBER, &args![this]).u32();
            let a = e
                .call(TES_00458700, &args![first, first_scratch, 2u32])
                .u32();
            let first_value = e.call(TES_004586D0, &args![a]).f64();
            let b = e
                .call(TES_00458700, &args![second, second_scratch, 2u32])
                .u32();
            let second_value = e.call(TES_004586D0, &args![b]).f64();
            // FCOMP; TEST AH,0x5; JP: the body runs only when below (ordered).
            if second_value < first_value {
                for vector in [first, second] {
                    let word = e.call(ELEMENT_ADDRESS, &args![vector, 2u32]).u32();
                    let lowered = e
                        .call(CALL_004A3E90, &args![x87_quiet_bits(second_float)])
                        .f64();
                    let value = x87_sub(x87_m32(e.mem.u32(word)), lowered);
                    e.mem.set_u32(word, x87_store32(value));
                }
            }
            member
        } else {
            e.call(GET_MEMBER, &args![this + 4]).u32()
        };

        fn_00621930(e, block);
        e.call(POINT_COLLECTOR_CONSTRUCT, &args![collector]);
        fn_006218d0(e, block, 0.0f32.to_bits());
        let model_first = e.call(MODEL_LOADER_0043B560, &args![member]).u32();
        let holder = e.call(CALL_00621A90, &args![subject]).u32();
        let model_second = e.call(MEMBER_AT_10, &args![holder]).u32();
        let proxy = e.call(CALL_00621AD0, &args![subject]).u32();
        if proxy != 0 {
            fn_006219c0(e, proxy);
            let cell = e.call(CELL_00541C80, &args![proxy]).u32();
            let source = e.call(EXPLOSION_004FB070, &args![cell]).u32();
            fn_006219c0(e, proxy);
            if source != 0 {
                e.call(UNLOADED_FORM_BUFFER_CONSTRUCT, &args![block, source]);
                fn_006219c0(e, proxy);
                let cell = e.call(CELL_00541C80, &args![proxy]).u32();
                let other = e.call(CALL_00407840, &args![cell]).u32();
                e.call(UNLOADED_FORM_BUFFER_CONSTRUCT, &args![block + 0xc, other]);
                fn_006219c0(e, proxy);
                fn_006219f0(e, member, first);
                let controller_object = e.call(CALL_004AE6C0, &args![member]).u32();
                let inner = e.call(GET_MEMBER, &args![model_second]).u32();
                let column = e.call(MODEL_LOADER_0043B230, &args![inner]).u32();
                let inner = e.call(GET_MEMBER, &args![model_first]).u32();
                let row = e.call(MODEL_LOADER_0043B230, &args![inner]).u32();
                let function = fn_006218f0(e, source, row, column);
                if function != 0 {
                    let saved = e.call(READ_00621B00, &args![controller_object]).f64();
                    let saved = x87_store32(saved);
                    e.call(
                        WRITE_00621B20,
                        &args![controller_object, x87_quiet_bits(first_float)],
                    );
                    e.call(
                        function,
                        &args![model_first, model_second, block, collector],
                    );
                    let result = e
                        .call(CAMERA_CASTER_00621210, &args![collector, out_byte])
                        .u32();
                    answer = e.call(REFR_005609F0, &args![result]).bool();
                    e.call(
                        WRITE_00621B20,
                        &args![controller_object, x87_quiet_bits(saved)],
                    );
                }
            }
        }
        e.call(POINT_COLLECTOR_DESTRUCT, &args![collector]);
        answer
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_19c0, fn_006219c0(u32)),
        entry!(0x0062_1990, fn_00621990(u32) -> u32),
        entry!(0x0062_1930, fn_00621930(u32) -> u32),
        entry!(0x0062_18d0, fn_006218d0(u32, u32)),
        entry!(0x0062_19f0, fn_006219f0(u32, u32)),
        entry!(0x0062_18f0, fn_006218f0(u32, u32, u32) -> u32),
        entry!(0x0062_14d0, fn_006214d0(u32, u32, u32, u32, u32) -> bool),
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

    /// An object whose vtable slot `slot` is the function at `function`.
    fn object_with_slot(e: &mut Engine, slot: u32, function: u32) -> u32 {
        let this = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(vtable + slot, function);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn fn_006219c0_runs_the_form_call_on_the_object() {
        let mut e = Engine::new();
        e.register(0x0090_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0090_2000, |_, _| Ret::default());
        e.register(FORM_CALL, |_, _| Ret::default());
        let with = object_with_slot(&mut e, 0x94, 0x0090_1000);
        let without = object_with_slot(&mut e, 0x94, 0x0090_2000);
        e.call_log = Some(vec![]);
        e.call(0x0062_19c0, &[with]);
        e.call(0x0062_19c0, &[without]);
        assert_eq!(calls_to(&e, FORM_CALL), vec![vec![0x7000]]);
    }

    #[test]
    fn fn_00621990_and_fn_00621930_construct_their_members() {
        let mut e = Engine::new();
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_1990, &[0x1000]).u32(), 0x1000);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![0x1000], vec![0x1010], vec![0x1020]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_1930, &[0x2000]).u32(), 0x2000);
        let expected: Vec<Vec<u32>> = [0x0, 0x4, 0x8, 0xc, 0x10, 0x14, 0x20, 0x30, 0x40]
            .iter()
            .map(|o| vec![0x2000 + o])
            .collect();
        assert_eq!(calls_to(&e, VECTOR_CONSTRUCT), expected);
    }

    #[test]
    fn fn_006218d0_passes_the_quieted_float_on() {
        let mut e = Engine::new();
        e.register(SET_FLOAT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_18d0, &[0x3000, 0x7f80_0005]);
        e.call(0x0062_18d0, &[0x3000, 2.5f32.to_bits()]);
        assert_eq!(
            calls_to(&e, SET_FLOAT),
            vec![vec![0x3008, 0x7fc0_0005], vec![0x3008, 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn fn_006219f0_moves_the_phantom_when_there_is_one() {
        let mut e = Engine::new();
        e.register(CHARACTER_PROXY_PHANTOM, |_, a| {
            if a[0] == 0x1000 { 0x9000u32 } else { 0 }.into_ret()
        });
        e.register(PROXY_AROUND, |_, _| Ret::default());
        e.register(PHANTOM_SET_POSITION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_19f0, &[0x1000, 0x5550]);
        assert_eq!(calls_to(&e, PROXY_AROUND), vec![vec![0x1000], vec![0x1000]]);
        assert_eq!(
            calls_to(&e, PHANTOM_SET_POSITION),
            vec![vec![0x9000, 0x5550, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0062_19f0, &[0x2000, 0x5550]);
        assert!(calls_to(&e, PROXY_AROUND).is_empty());
        assert!(calls_to(&e, PHANTOM_SET_POSITION).is_empty());
    }

    #[test]
    fn fn_006218f0_reads_the_function_table() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1000);
        e.mem.set_u8(this + 0x1a0 + 2 * 32 + 3, 5);
        e.mem.set_u32(this + 0x9a8 + 5 * 0x14, 0x00ab_cdef);
        assert_eq!(e.call(0x0062_18f0, &[this, 2, 3]).u32(), 0x00ab_cdef);
        assert_eq!(
            e.call(0x0062_18f0, &[this, 0, 0]).u32(),
            e.mem.u32(this + 0x9a8)
        );
    }

    fn register_common(e: &mut Engine) {
        for address in [
            VECTOR_CONSTRUCT,
            COPY_POSITION,
            SET_FROM_HK,
            POINT_COLLECTOR_CONSTRUCT,
            SET_FLOAT,
            UNLOADED_FORM_BUFFER_CONSTRUCT,
            PROXY_AROUND,
            PHANTOM_SET_POSITION,
            WRITE_00621B20,
            POINT_COLLECTOR_DESTRUCT,
            FORM_CALL,
            MULTIBOUND_00439EF0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_006214d0_compares_the_saved_acquire_distance() {
        let mut e = Engine::new();
        e.map(OBJECT_011DEA3C & !0xfff, 0x1000);
        register_common(&mut e);
        e.register(CAMERA_CASTER_TEST, |_, _| 1u32.into_ret());
        e.mem.set_u32(OBJECT_011DEA3C, 0x4000);
        e.register(0x0090_1000, |_, _| 0x6000u32.into_ret());
        let object = object_with_slot(&mut e, 0x6d4, 0x0090_1000);
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |_, _| object.into_ret());
        e.register(TES_0045BB80, |_, _| 0x77u32.into_ret());
        e.register(TES_00457990, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        let limit = e.mem.alloc(4);
        e.register_double(EFFECT_ITEM_00403E20, move |_, _| limit.into_ret());
        for (value, expected) in [(12.0f32, true), (10.0, false), (f32::NAN, false)] {
            e.mem.set_f32(limit, value);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x0062_14d0, &[0x100, 0x200, 0x300, 0, 0]).bool(),
                expected
            );
            let multibound = calls_to(&e, MULTIBOUND_00439EF0);
            assert_eq!(multibound.len(), 1);
            assert_eq!((multibound[0][0], multibound[0][2]), (0x300, 0x77));
            assert_eq!(calls_to(&e, TES_0045BB80), vec![vec![0x6000]]);
            assert!(calls_to(&e, POINT_COLLECTOR_DESTRUCT).is_empty());
        }
    }

    #[test]
    fn fn_006214d0_runs_the_table_function() {
        let mut e = Engine::new();
        e.map(OBJECT_011DEA3C & !0xfff, 0x1000);
        register_common(&mut e);
        // The camera test fails: straight to the main path.
        e.register(CAMERA_CASTER_TEST, |_, _| Ret::default());
        e.register(GET_HK_POSITION, |_, _| 0x8800u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x200u32.into_ret());
        e.register(GET_MEMBER, |_, a| (a[0] + 0x1_0000).into_ret());
        let marker = e.mem.alloc(4);
        e.register_double(COPY_POSITION, move |e, a| {
            e.mem.set_u32(marker, a[0]);
            Ret::default()
        });
        let words = e.mem.alloc(0x20);
        e.mem.set_f32(words, 5.0);
        e.mem.set_f32(words + 4, 7.0);
        e.register_double(TES_00458700, move |e, a| {
            // The first vector answers 5.0, the second 1.0 (below: both lowered).
            if a[2] == 2 && a[0] == e.mem.u32(marker) {
                words
            } else {
                words + 0x10
            }
            .into_ret()
        });
        e.mem.set_f32(words + 0x10, 1.0);
        e.register(TES_004586D0, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register_double(ELEMENT_ADDRESS, move |e, a| {
            if a[0] == e.mem.u32(marker) {
                words + 8
            } else {
                words + 0xc
            }
            .into_ret()
        });
        e.mem.set_f32(words + 8, 3.0);
        e.mem.set_f32(words + 0xc, 4.0);
        e.register(CALL_004A3E90, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        e.register(MODEL_LOADER_0043B560, |_, _| 0xa000u32.into_ret());
        e.register(CALL_00621A90, |_, _| 0xb000u32.into_ret());
        e.register(MEMBER_AT_10, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0090_3000, |_, _| Ret::default());
        let proxy = object_with_slot(&mut e, 0x94, 0x0090_3000);
        e.register_double(CALL_00621AD0, move |_, _| proxy.into_ret());
        e.register(CELL_00541C80, |_, _| 0xc000u32.into_ret());
        let source = e.mem.alloc(0x1000);
        e.register_double(EXPLOSION_004FB070, move |_, _| source.into_ret());
        e.register(CALL_00407840, |_, _| 0xd000u32.into_ret());
        e.register(CHARACTER_PROXY_PHANTOM, |_, _| Ret::default());
        e.register(CALL_004AE6C0, |_, _| 0xe000u32.into_ret());
        e.register(MODEL_LOADER_0043B230, |_, a| {
            if a[0] == 0x1_a000 { 1u32 } else { 2 }.into_ret()
        });
        // Row 1 (the first model), column 2 (the second): index 4.
        e.mem.set_u8(source + 0x1a0 + 32 + 2, 4);
        e.mem.set_u32(source + 0x9a8 + 4 * 0x14, 0x0090_4000);
        e.register(0x0090_4000, |_, _| Ret::default());
        e.register(READ_00621B00, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.register(CAMERA_CASTER_00621210, |_, _| 0xf000u32.into_ret());
        e.register(REFR_005609F0, |_, _| 1u32.into_ret());

        e.call_log = Some(vec![]);
        let this = 0x100;
        assert!(e
            .call(
                0x0062_14d0,
                &[this, 0x200, 0x300, 1.5f32.to_bits(), 0.5f32.to_bits()]
            )
            .bool());
        assert_eq!(calls_to(&e, GET_MEMBER)[0], vec![this]);
        // Both words lowered by 2 * 0.5.
        assert_eq!(e.mem.f32(words + 8), 2.0);
        assert_eq!(e.mem.f32(words + 0xc), 3.0);
        let table_call = calls_to(&e, 0x0090_4000);
        assert_eq!(table_call.len(), 1);
        assert_eq!(&table_call[0][..2], &[0xa000, 0xb010]);
        assert_eq!(
            calls_to(&e, WRITE_00621B20),
            vec![
                vec![0xe000, 1.5f32.to_bits()],
                vec![0xe000, 0.25f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, POINT_COLLECTOR_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, UNLOADED_FORM_BUFFER_CONSTRUCT).len(), 2);

        // Not the controller: the member of this + 4, no lowering; no proxy: answer false.
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x999u32.into_ret());
        e.register(CALL_00621AD0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_14d0, &[this, 0x200, 0x300, 0, 0]).bool());
        assert_eq!(calls_to(&e, GET_MEMBER), vec![vec![this + 4]]);
        assert!(calls_to(&e, ELEMENT_ADDRESS).is_empty());
        assert_eq!(calls_to(&e, POINT_COLLECTOR_DESTRUCT).len(), 1);
    }
}

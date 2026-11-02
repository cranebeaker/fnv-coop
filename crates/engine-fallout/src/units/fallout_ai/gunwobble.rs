//! `fallout/ai/gunwobble.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The twelve model-name slots (`+4 * index`) of the gun wobble table: the models themselves.
const MODELS: u32 = 0x011e_0110;
/// The twelve model-name slots: copies of each model's name.
const NAMES: u32 = 0x011e_00e0;
/// Set to 0 while the table is built and to 1 afterwards.
const READY: u32 = 0x0118_5520;

// Translated from 008d66b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d66b0 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// `InitGunWobble` (engine map name, `gunwobble.cpp`): clears the two tables of twelve words at
/// `0x011e0110` and `0x011e00e0`, then for each index builds a file name (`00406d00` with the
/// format at `0x010876e4` for index 0, the one at `0x010876c0` and the word `0x011977a4 + 4 *
/// index` for the others), loads the model (`00456a20`, then `00447080` on the object at
/// `0x011c3b3c`) into the first table and stores a heap copy of its name (`0043b1b0`) in the
/// second. Each model gets the name `00438170` makes of the file name (`00a5b950`). The flag at
/// `0x01185520` is 0 meanwhile and 1 at the end.
///
/// The 0x104 byte name buffer the game keeps on the stack is a heap block here. The exception
/// frame and the security cookie are not translated.
pub fn fn_008d66b0(e: &mut Engine) {
    e.call(0x0040_3d30, &args![MODELS, 0u32, 0xcu32]);
    e.call(0x0040_3d30, &args![NAMES, 0u32, 0xcu32]);
    e.mem.set_u8(READY, 0);
    let buffer = e.mem.alloc(0x104);
    for index in 0..12u32 {
        if index == 0 {
            e.call(
                0x0040_6d00,
                &args![buffer, 0x104u32, 0x0108_76e4u32, 0x0101_dccc_u32],
            );
        } else {
            let extra = e.global::<u32>(0x0119_77a4 + index * 4);
            e.call(
                0x0040_6d00,
                &args![buffer, 0x104u32, 0x0108_76c0u32, 0x0101_dccc_u32, extra],
            );
        }
        if e.call(0x0045_6a20, &args![buffer, 0u32, 6u32, 1u32]).u32() == 0 {
            continue;
        }
        let loader = e.global::<u32>(0x011c_3b3c);
        let model = e
            .call(
                0x0044_7080,
                &args![loader, buffer, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(MODELS + index * 4, model);
        let model = e.mem.u32(MODELS + index * 4);
        let node = e.call(0x0041_3f40, &args![model]).u32();
        let text = e.call(0x0043_b1b0, &args![node]).u32();
        let length = e.call(0x0044_a670, &args![text]).u32();
        let size = length.wrapping_add(1);
        let copy = e.call(0x0040_1000, &args![size]).u32();
        e.mem.set_u32(NAMES + index * 4, copy);
        let model = e.mem.u32(MODELS + index * 4);
        let node = e.call(0x0041_3f40, &args![model]).u32();
        let text = e.call(0x0043_b1b0, &args![node]).u32();
        e.call(0x0040_6d30, &args![copy, size, text]);
        let name = e.mem.alloc(4);
        let made = e.call(0x0043_8170, &args![name, buffer]).u32();
        let model = e.mem.u32(MODELS + index * 4);
        e.call(0x00a5_b950, &args![model, made]);
        e.call(0x0043_81b0, &args![name]);
        e.mem.free(name);
    }
    e.mem.set_u8(READY, 1);
    e.mem.free(buffer);
}

/// The identity matrix (`NiMatrix3::IDENTITY`, nine floats) the wobble functions copy.
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// The model loader object (`011c3b3c`) and its release of a model by name (`0045a5e0`,
/// `thiscall`, `(name)`); a model's node (`00413f40`) and a node's name (`0043b1b0`), both
/// `thiscall` without arguments; the heap free `00401030` (`cdecl`, `(block)`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_RELEASE: u32 = 0x0045_a5e0;
const MODEL_NODE: u32 = 0x0041_3f40;
const NODE_NAME: u32 = 0x0043_b1b0;
const HEAP_FREE: u32 = 0x0040_1030;
/// Callees of `fn_008d6970`: `0043b230` (`thiscall` on the model, no arguments),
/// `0047c860` (`thiscall`, a float in ST0; the map names it `TESActorBaseData::GetKarma`, possibly a folded name), `0043d410`
/// (`thiscall` on a 12-byte local, `(float, 1, 0)`), `fn_00a59c60` (`thiscall` on the model,
/// `(&local)`), `006a9540` (`thiscall` on the model, answers a matrix), `NiMatrix3::Inverse`
/// (`thiscall`, `(&result)`, answers it) and `NiMatrix3::operator*` (`thiscall`,
/// `(&result, &matrix)`, answers it).
const MODEL_OBJECT_0043B230: u32 = 0x0043_b230;
const MODEL_VALUE_0047C860: u32 = 0x0047_c860;
const LOCAL_FROM_VALUE_0043D410: u32 = 0x0043_d410;
const MODEL_APPLY_00A59C60: u32 = 0x00a5_9c60;
const MODEL_MATRIX_006A9540: u32 = 0x006a_9540;
const MATRIX_INVERSE: u32 = 0x004b_45b0;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// Callees of `clear_gun_wobble`: `004aae30` (`cdecl`, `(object, name)`), `006838b0`
/// (`thiscall`, no arguments), `00653270` (`cdecl`, `(012681cc, key)`) and `004f0110`
/// (`thiscall`, `(&matrix)`).
const FIND_BY_NAME_004AAE30: u32 = 0x004a_ae30;
const KEY_006838B0: u32 = 0x0068_38b0;
const LOOKUP_00653270: u32 = 0x0065_3270;
const LOOKUP_TABLE: u32 = 0x0126_81cc;
const SET_MATRIX_004F0110: u32 = 0x004f_0110;

/// Copies the nine words of a matrix.
fn copy_matrix(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..9u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

// Translated from 008d68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, no arguments; the counterpart of `fn_008d66b0`): for each of
/// the twelve slots, a loaded model is released by its node's name (`0045a5e0` on the loader
/// at `011c3b3c`) and its slot cleared, and a name copy is freed (`00401030`) and its slot
/// cleared.
pub fn fn_008d68d0(e: &mut Engine) {
    for index in 0..12u32 {
        let model = e.mem.u32(MODELS + index * 4);
        if model != 0 {
            let node = e.call(MODEL_NODE, &args![model]).u32();
            let name = e.call(NODE_NAME, &args![node]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_RELEASE, &args![loader, name]);
            e.mem.set_u32(MODELS + index * 4, 0);
        }
        let copy = e.mem.u32(NAMES + index * 4);
        if copy != 0 {
            e.call(HEAP_FREE, &args![copy]);
            e.mem.set_u32(NAMES + index * 4, 0);
        }
    }
}

// Translated from 008d6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, `(index, parameter, &matrix)`): without the slot's model or
/// name the identity (`011a9448`) is copied to `matrix` and 0 answered. Otherwise, for a
/// slot other than 0, `0043d410` makes a 12-byte local from the float `0047c860` answers for the
/// model's `0043b230` object (with 1 and 0) and `fn_00a59c60` applies it to the
/// model; the model's matrix (`006a9540`) is kept; `0043d410` makes another local from
/// `parameter` that is applied the same way; `matrix` gets the kept matrix's inverse times the
/// model's new matrix. Answers the slot's name copy.
pub fn fn_008d6970(e: &mut Engine, index: u32, parameter: f32, matrix: Ptr) -> u32 {
    let slot = index.wrapping_mul(4);
    let model = e.mem.u32(MODELS.wrapping_add(slot));
    if model == 0 || e.mem.u32(NAMES.wrapping_add(slot)) == 0 {
        copy_matrix(e, matrix.addr(), IDENTITY_MATRIX);
        return 0;
    }
    // The frame below EBP: product at +0x00, inverse at +0x24, the float at +0x48, its
    // local at +0x4C, the kept matrix at +0x58, the object at +0x7C, the parameter local at
    // +0x80.
    e.with_stack(0x8c, |e, frame| {
        let frame = frame.addr();
        let object = e.call(MODEL_OBJECT_0043B230, &args![model]).u32();
        e.mem.set_u32(frame + 0x7c, object);
        if index != 0 {
            let value = e.call(MODEL_VALUE_0047C860, &args![object]).f64();
            e.mem.set_u32(frame + 0x48, x87_store32(value));
            let value = x87_quiet_bits(e.mem.u32(frame + 0x48));
            e.call(
                LOCAL_FROM_VALUE_0043D410,
                &args![frame + 0x4c, value, 1u32, 0u32],
            );
            let model = e.mem.u32(MODELS.wrapping_add(slot));
            e.call(MODEL_APPLY_00A59C60, &args![model, frame + 0x4c]);
        }
        let model = e.mem.u32(MODELS.wrapping_add(slot));
        let kept = e.call(MODEL_MATRIX_006A9540, &args![model]).u32();
        copy_matrix(e, frame + 0x58, kept);
        e.call(
            LOCAL_FROM_VALUE_0043D410,
            &args![
                frame + 0x80,
                x87_quiet_bits(parameter.to_bits()),
                1u32,
                0u32
            ],
        );
        let model = e.mem.u32(MODELS.wrapping_add(slot));
        e.call(MODEL_APPLY_00A59C60, &args![model, frame + 0x80]);
        let model = e.mem.u32(MODELS.wrapping_add(slot));
        let current = e.call(MODEL_MATRIX_006A9540, &args![model]).u32();
        let inverse = e
            .call(MATRIX_INVERSE, &args![frame + 0x58, frame + 0x24])
            .u32();
        let product = e
            .call(MATRIX_MULTIPLY, &args![inverse, frame, current])
            .u32();
        copy_matrix(e, matrix.addr(), product);
    });
    e.mem.u32(NAMES.wrapping_add(slot))
}

// Translated from 008d6a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ClearGunWobble` (engine map name), `cdecl`, `(index, object)`: with the slot's name copy,
/// `004aae30(object, name)`; when that finds something, its key (`006838b0`) is looked up
/// (`00653270` with the table at `012681cc`) and a found entry gets the identity matrix
/// (`004f0110`).
pub fn clear_gun_wobble(e: &mut Engine, index: u32, object: u32) {
    let name = e.mem.u32(NAMES.wrapping_add(index.wrapping_mul(4)));
    if name == 0 {
        return;
    }
    let found = e.call(FIND_BY_NAME_004AAE30, &args![object, name]).u32();
    if found == 0 {
        return;
    }
    let key = e.call(KEY_006838B0, &args![found]).u32();
    let entry = e.call(LOOKUP_00653270, &args![LOOKUP_TABLE, key]).u32();
    if entry != 0 {
        e.call(SET_MATRIX_004F0110, &args![entry, IDENTITY_MATRIX]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_66b0, fn_008d66b0()),
        entry!(0x008d68d0, fn_008d68d0()),
        entry!(0x008d6970, fn_008d6970(u32, f32, Ptr) -> u32),
        entry!(0x008d6a80, clear_gun_wobble(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maps the words the function reads and writes.
    fn map_globals(e: &mut Engine) {
        for (address, size) in [
            (MODELS, 0xc),
            (NAMES, 0xc),
            (READY, 1),
            (0x0119_77a4, 0x30),
            (0x011c_3b3c, 4),
        ] {
            e.map(address, size);
        }
    }

    fn double(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn run(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(0x008d_66b0, &[]);
        let log = e.call_log.take().unwrap();
        log.into_iter().skip(1).collect()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn builds_twelve_models_and_names() {
        let mut e = Engine::new();
        map_globals(&mut e);
        for (address, eax) in [
            (0x0040_3d30, 0),
            (0x0040_6d00, 0),
            (0x0045_6a20, 1),
            (0x0044_7080, 0x5000),
            (0x0041_3f40, 0x6000),
            (0x0043_b1b0, 0x7000),
            (0x0044_a670, 4),
            (0x0040_1000, 0x8000),
            (0x0040_6d30, 0),
            (0x0043_8170, 0x9000),
            (0x00a5_b950, 0),
            (0x0043_81b0, 0),
        ] {
            double(&mut e, address, eax);
        }
        e.set_global(0x0119_77a4 + 4, 0x1234u32);
        e.set_global(0x011c_3b3c, 0x4000u32);
        e.set_global(READY, 7u8);
        let log = run(&mut e);
        let formats = calls_to(&log, 0x0040_6d00);
        assert_eq!(formats.len(), 12);
        assert_eq!(formats[0][1..], [0x104, 0x0108_76e4, 0x0101_dccc]);
        assert_eq!(formats[1][1..], [0x104, 0x0108_76c0, 0x0101_dccc, 0x1234]);
        let loads = calls_to(&log, 0x0044_7080);
        assert_eq!(loads.len(), 12);
        assert_eq!(loads[0][0], 0x4000);
        assert_eq!(loads[0][2..], [0, 1, 0, 0, 0]);
        assert_eq!(calls_to(&log, 0x00a5_b950).len(), 12);
        assert_eq!(calls_to(&log, 0x0040_1000)[0], [5]);
        assert_eq!(e.mem.u32(MODELS + 11 * 4), 0x5000);
        assert_eq!(e.mem.u32(NAMES + 11 * 4), 0x8000);
        assert_eq!(e.mem.u8(READY), 1);
    }

    #[test]
    fn skips_models_that_do_not_load() {
        let mut e = Engine::new();
        map_globals(&mut e);
        for address in [0x0040_3d30, 0x0040_6d00, 0x0045_6a20] {
            double(&mut e, address, 0);
        }
        let log = run(&mut e);
        assert!(calls_to(&log, 0x0044_7080).is_empty());
        assert_eq!(e.mem.u32(MODELS), 0);
        assert_eq!(e.mem.u8(READY), 1);
    }

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn wobble_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(MODELS, 0x30);
        e.map(NAMES, 0x30);
        e.map(IDENTITY_MATRIX, 0x24);
        for word in 0..9u32 {
            e.set_global::<u32>(IDENTITY_MATRIX + 4 * word, 0x100 + word);
        }
        e.map(MODEL_LOADER, 4);
        e.set_global::<u32>(MODEL_LOADER, 0x4444);
        e
    }

    #[test]
    fn fn_008d68d0_releases_models_and_frees_names() {
        let mut e = wobble_engine();
        e.register(MODEL_NODE, |_, a| answer(a[0] + 1));
        e.register(NODE_NAME, |_, a| answer(a[0] + 1));
        e.register(MODEL_RELEASE, |_, _| Ret::default());
        e.register(HEAP_FREE, |_, _| Ret::default());
        e.mem.set_u32(MODELS + 4, 0x5000);
        e.mem.set_u32(NAMES + 4 * 11, 0x6000);
        e.call(0x008d68d0, &args![]);
        assert_eq!(calls_at(&e, MODEL_RELEASE), vec![vec![0x4444, 0x5002]]);
        assert_eq!(calls_at(&e, HEAP_FREE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(MODELS + 4), 0);
        assert_eq!(e.mem.u32(NAMES + 44), 0);
    }

    #[test]
    fn fn_008d6970_answers_the_identity_without_a_model() {
        let mut e = wobble_engine();
        let matrix = e.mem.alloc(0x24);
        e.mem.set_u32(MODELS + 8, 0x5000);
        assert_eq!(e.call(0x008d6970, &args![2u32, 1.0f32, matrix]).u32(), 0);
        assert_eq!(e.mem.u32(matrix + 0x20), 0x108);
    }

    #[test]
    fn fn_008d6970_combines_the_inverse_with_the_new_matrix() {
        let mut e = wobble_engine();
        let kept = e.mem.alloc(0x24);
        let product = e.mem.alloc(0x24);
        for word in 0..9u32 {
            e.mem.set_u32(kept + 4 * word, 0x200 + word);
            e.mem.set_u32(product + 4 * word, 0x300 + word);
        }
        e.register(MODEL_OBJECT_0043B230, |_, _| answer(0x7000));
        e.register(MODEL_VALUE_0047C860, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(LOCAL_FROM_VALUE_0043D410, |_, _| Ret::default());
        e.register(MODEL_APPLY_00A59C60, |_, _| Ret::default());
        e.register_double(MODEL_MATRIX_006A9540, move |_, _| answer(kept));
        e.register(MATRIX_INVERSE, |_, a| answer(a[1]));
        e.register_double(MATRIX_MULTIPLY, move |_, _| answer(product));
        e.mem.set_u32(MODELS + 4, 0x5000);
        e.mem.set_u32(NAMES + 4, 0x6000);
        let matrix = e.mem.alloc(0x24);
        assert_eq!(
            e.call(0x008d6970, &args![1u32, 2.0f32, matrix]).u32(),
            0x6000
        );
        assert_eq!(e.mem.u32(matrix + 4), 0x301);
        let locals = calls_at(&e, LOCAL_FROM_VALUE_0043D410);
        assert_eq!(locals.len(), 2);
        assert_eq!(locals[0][1..], [0.5f32.to_bits(), 1, 0]);
        assert_eq!(locals[1][1..], [2.0f32.to_bits(), 1, 0]);
        let applied = calls_at(&e, MODEL_APPLY_00A59C60);
        assert_eq!(
            applied,
            vec![vec![0x5000, locals[0][0]], vec![0x5000, locals[1][0]]]
        );
        let inverse = &calls_at(&e, MATRIX_INVERSE)[0];
        // The kept copy is the matrix 006a9540 answered first.
        assert_eq!(e.mem.u32(inverse[0] + 4), 0x201);
        let multiply = &calls_at(&e, MATRIX_MULTIPLY)[0];
        assert_eq!((multiply[0], multiply[2]), (inverse[1], kept));
        // Slot 0: no karma local.
        e.mem.set_u32(MODELS, 0x5000);
        e.mem.set_u32(NAMES, 0x6100);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x008d6970, &args![0u32, 2.0f32, matrix]).u32(),
            0x6100
        );
        assert_eq!(calls_at(&e, LOCAL_FROM_VALUE_0043D410).len(), 1);
        assert!(calls_at(&e, MODEL_VALUE_0047C860).is_empty());
    }

    #[test]
    fn clear_gun_wobble_resets_the_found_entry() {
        let mut e = wobble_engine();
        e.register(FIND_BY_NAME_004AAE30, |_, a| {
            answer(u32::from(a[0] == 0x8000) * 0x9000)
        });
        e.register(KEY_006838B0, |_, _| answer(0x77));
        e.register(LOOKUP_00653270, |_, a| {
            answer(u32::from(a[1] == 0x77) * 0xa000)
        });
        e.register(SET_MATRIX_004F0110, |_, _| Ret::default());
        e.call(0x008d6a80, &args![3u32, 0x8000u32]);
        assert!(calls_at(&e, FIND_BY_NAME_004AAE30).is_empty());
        e.mem.set_u32(NAMES + 12, 0x6000);
        e.call(0x008d6a80, &args![3u32, 0x8001u32]);
        assert!(calls_at(&e, KEY_006838B0).is_empty());
        e.call(0x008d6a80, &args![3u32, 0x8000u32]);
        assert_eq!(calls_at(&e, FIND_BY_NAME_004AAE30)[1], vec![0x8000, 0x6000]);
        assert_eq!(
            calls_at(&e, LOOKUP_00653270),
            vec![vec![LOOKUP_TABLE, 0x77]]
        );
        assert_eq!(
            calls_at(&e, SET_MATRIX_004F0110),
            vec![vec![0xa000, IDENTITY_MATRIX]]
        );
    }
}

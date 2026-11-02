//! `BSHavok/run_00c741e0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c741e0` constructs the 0x650-byte `bhkCharacterController` (the RTTI
//! name of its three vtables; layout in `bhkcharactercontroller.rs`).
//!
//! Floats: the x87 stores copy `float` values (`FLD`/`FSTP` copies go through
//! [`x87_quiet_bits`]); the SSE vector stores are bit copies of the
//! `float` constants they are built from.
//!
//! The exception-unwinding frame (`FS:[0]`) and the `/GS` cookie check
//! (`00ec408c`) are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCharacterProxy` constructor (`00c6ce80`, the base at +0).
const PROXY_CONSTRUCT: u32 = 0x00c6_ce80;
/// `hkpCharacterContext` constructor (`00ca9580`, `this` = +0x3E0; two stack
/// words: the state manager and the character type taken from the cinfo).
const CONTEXT_CONSTRUCT: u32 = 0x00ca_9580;
/// `bhkCharacterListener` constructor (`00c6c7a0`, `this` = +0x410; the up
/// vector, the maximum slope angle and the step height as `float` words).
const LISTENER_CONSTRUCT: u32 = 0x00c6_c7a0;
/// `__eh_vector_constructor_iterator` (`00ec782f`: array, element size,
/// count, constructor, destructor) with the `NiPointer` constructor
/// (`00b54620`) and destructor (`00b66f20`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const NI_POINTER_CONSTRUCT: u32 = 0x00b5_4620;
const NI_POINTER_DESTRUCT: u32 = 0x00b6_6f20;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `bhkCharacterController::Init` (Xbox PDB, `00c73ed0`).
const CONTROLLER_INIT: u32 = 0x00c7_3ed0;

/// Vtables of the controller, its `hkpCharacterContext` and its
/// `bhkCharacterListener`.
const VTABLE_CONTROLLER: u32 = 0x010c_49c4;
const VTABLE_CONTEXT: u32 = 0x010c_49b4;
const VTABLE_LISTENER: u32 = 0x010c_4998;

/// The 16-byte vector at `01267e30` (zero in the exe).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// `float` constants: `0.0`, `1.0`, 2.0 (the radius), 10.0 (the height) and
/// -1.0 (the LOD distance).
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_ONE: u32 = 0x0101_2068;
const FLOAT_RADIUS: u32 = 0x0101_62c0;
const FLOAT_HEIGHT: u32 = 0x0101_7b78;
const FLOAT_LOD_DISTANCE: u32 = 0x0101_2054;
/// The maximum tilt in degrees (`float`) and the degrees-to-radians factor
/// (`double`).
const MAX_TILT_DEGREES: u32 = 0x011b_0148;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// Offsets in the controller (Xbox PDB names; PC offsets agree).
const HK_CONTEXT: u32 = 0x3e0;
const LISTENER: u32 = 0x410;
const I_FLAGS: u32 = 0x414;
const HK_SUPPORT_NORM: u32 = 0x450;
const SUPPORTED_STATE: u32 = 0x480;
const SURFACE_NORMAL: u32 = 0x490;
const SURFACE_VELOCITY: u32 = 0x4a0;
const SURFACE_DISTANCE_EXCESS: u32 = 0x4b0;
const SURFACE_IS_DYNAMIC: u32 = 0x4b4;
const UP_VEC: u32 = 0x4c0;
const FORWARD_VEC: u32 = 0x4d0;
const VELOCITY_MOD: u32 = 0x500;
const WANT_STATE: u32 = 0x520;
const F_GRAVITY: u32 = 0x54c;
const F_PITCH_MULT: u32 = 0x558;
const ROT_CENTER: u32 = 0x570;
const SP_SHAPE_PHANTOM: u32 = 0x594;
const SP_COLLISION_BOX: u32 = 0x598;
const SHAPE_TYPE: u32 = 0x59c;
const SIZED_SHAPE_TYPE: u32 = 0x5a0;
const SP_SHAPES: u32 = 0x5a4;
const F_RADIUS: u32 = 0x5f0;
const F_HEIGHT: u32 = 0x5f4;
const F_DEST_RADIUS: u32 = 0x5f8;
const F_LOD_DISTANCE: u32 = 0x5fc;
const E_SIZE: u32 = 0x600;
const I_PRIORITY: u32 = 0x604;
const I_SUPPORT_COUNT: u32 = 0x608;
const SP_SUPPORT_BODY: u32 = 0x60c;
const B_FAKE_SUPPORT: u32 = 0x610;
const FAKE_SUPPORT_START: u32 = 0x620;
const SP_BUMPED_BODY: u32 = 0x630;
const F_BUMPED_FORCE: u32 = 0x634;
const SP_BUMPED_CHAR_PROXY: u32 = 0x638;
const LAST_MANIFOLD: u32 = 0x63c;
const DAMAGE_IMPACTS: u32 = 0x648;

/// The controller fields the constructor sets to `float` zero (`FLDZ` then
/// `FST`): `fVelocityTime`, `fRotMod`, `fRotModTime`, `fCalculatePitchTimer`,
/// `fAcrobatics`, `fCenter`, `fFallStartHeight`, `fFallTime`, `fPitchAngle`
/// and `fRollAngle`.
const FLOAT_ZERO_FIELDS: [u32; 10] = [
    0x524, 0x528, 0x52c, 0x530, 0x534, 0x538, 0x544, 0x548, 0x550, 0x554,
];

/// The 16-byte zero vector of the exe at `01267e30`, copied to `target`.
fn copy_zero_vector(e: &mut Engine, target: u32) {
    let zero = e.mem.bytes(ZERO_VECTOR, 16);
    e.mem.write(target, &zero);
}

fn store_vector(e: &mut Engine, target: u32, lanes: [f32; 4]) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(target + 4 * lane as u32, value);
    }
}

// Translated from 00c741e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `bhkCharacterController` (by the RTTI name of its three
/// vtables) from the controller cinfo at `info`: the `bhkCharacterProxy`
/// base, the `hkpCharacterContext` from the cinfo's words at +0xA4 and +0x9C,
/// the `bhkCharacterListener` from the up vector (0, 0, 1, 0), the maximum
/// tilt (`float` at `011b0148` in degrees, as radians) and the step height
/// (`info + 0xAC` times `info + 0xA8`), the vtables, the surface info
/// (supported state 2), up and forward vectors (0, 0, 1, 0) and (1, 0, 0, 0),
/// zeroed motion state, gravity and pitch multiplier 1, radius 2, height 10,
/// LOD distance -1, two empty `NiPointer` shape slots, an empty manifold array
/// with room for `0x230 / 0x70` entries from the Havok allocator, then (with
/// the per-thread scope word at 0x10) `bhkCharacterController::Init(info)`.
/// Returns `this`.
// Translation incomplete: 00c741e0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
pub fn fn_00c741e0(e: &mut Engine, this: Ptr, info: Ptr) -> Ptr {
    let me = this.addr();
    let info = info.addr();
    e.call(PROXY_CONSTRUCT, &args![this]);
    let character_type = e.mem.u32(info + 0x9c);
    let state_manager = e.mem.u32(info + 0xa4);
    e.call(
        CONTEXT_CONSTRUCT,
        &args![me + HK_CONTEXT, state_manager, character_type],
    );

    let step_height = (e.mem.f32(info + 0xac) as f64 * e.mem.f32(info + 0xa8) as f64) as f32;
    let max_tilt = x87_quiet(e.global::<f32>(MAX_TILT_DEGREES));
    let max_slope = (max_tilt as f64 * e.global::<f64>(DEGREES_TO_RADIANS)) as f32;
    let zero: f32 = e.global(FLOAT_ZERO);
    let one: f32 = e.global(FLOAT_ONE);
    let up = [zero, zero, one, zero];
    e.with_stack(0x10, |e, vector| {
        store_vector(e, vector.addr(), up);
        e.call(
            LISTENER_CONSTRUCT,
            &args![
                me + LISTENER,
                vector,
                max_slope.to_bits(),
                step_height.to_bits()
            ],
        );
    });

    e.mem.set_f32(me + SURFACE_DISTANCE_EXCESS, 0.0);
    store_vector(e, me + SURFACE_NORMAL, up);
    e.mem.set_u32(me + SUPPORTED_STATE, 2);
    store_vector(e, me + SURFACE_VELOCITY, [zero; 4]);
    e.mem.set_u8(me + SURFACE_IS_DYNAMIC, 0);
    e.mem.set_u32(me, VTABLE_CONTROLLER);
    e.mem.set_u32(me + HK_CONTEXT, VTABLE_CONTEXT);
    e.mem.set_u32(me + LISTENER, VTABLE_LISTENER);
    store_vector(e, me + UP_VEC, up);
    store_vector(e, me + FORWARD_VEC, [one, zero, zero, zero]);
    copy_zero_vector(e, me + VELOCITY_MOD);
    e.mem.set_u32(me + WANT_STATE, 0);
    for offset in FLOAT_ZERO_FIELDS {
        e.mem.set_f32(me + offset, 0.0);
    }
    e.mem.set_f32(me + F_GRAVITY, 1.0);
    e.mem.set_f32(me + F_PITCH_MULT, 1.0);
    copy_zero_vector(e, me + ROT_CENTER);
    e.mem.set_u32(me + SP_SHAPE_PHANTOM, 0);
    e.mem.set_u32(me + SP_COLLISION_BOX, 0);
    e.mem.set_u32(me + SHAPE_TYPE, 2);
    e.mem.set_u32(me + SIZED_SHAPE_TYPE, 2);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            me + SP_SHAPES,
            4u32,
            2u32,
            NI_POINTER_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    let radius = x87_quiet_bits(e.global::<u32>(FLOAT_RADIUS));
    e.mem.set_u32(me + F_RADIUS, radius);
    e.mem.set_u32(me + E_SIZE, 0);
    e.mem.set_u32(me + I_PRIORITY, 0);
    let height = x87_quiet_bits(e.global::<u32>(FLOAT_HEIGHT));
    e.mem.set_u32(me + F_HEIGHT, height);
    e.mem.set_u32(me + I_SUPPORT_COUNT, 0);
    e.mem.set_u32(me + F_DEST_RADIUS, radius);
    let lod_distance = x87_quiet_bits(e.global::<u32>(FLOAT_LOD_DISTANCE));
    e.mem.set_u32(me + F_LOD_DISTANCE, lod_distance);
    e.mem.set_u32(me + SP_SUPPORT_BODY, 0);
    e.mem.set_u8(me + B_FAKE_SUPPORT, 0);
    copy_zero_vector(e, me + FAKE_SUPPORT_START);
    e.mem.set_u32(me + SP_BUMPED_BODY, 0);
    e.mem.set_f32(me + F_BUMPED_FORCE, 0.0);
    e.mem.set_u32(me + SP_BUMPED_CHAR_PROXY, 0);

    // The manifold array (`hkArray<hkpRootCdPoint>`, 0x70-byte entries): data
    // from the Havok allocator's slot +0xC (given the wanted byte count by
    // reference, which it updates to what it allocated), the entry count that
    // many bytes hold, and the capacity word (flags only when that is zero).
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let (data, bytes) = e.with_stack(4, |e, wanted| {
        e.mem.set_u32(wanted.addr(), 0x230);
        let data = e.vcall(allocator, 0xc, &args![wanted]).u32();
        (data, e.mem.u32(wanted.addr()))
    });
    let entries = (bytes as i32) / 0x70;
    let capacity = if entries == 0 {
        0x8000_0000
    } else {
        entries as u32
    };
    e.mem.set_u32(me + LAST_MANIFOLD, data);
    e.mem.set_u32(me + LAST_MANIFOLD + 4, entries as u32);
    e.mem.set_u32(me + LAST_MANIFOLD + 8, capacity);
    e.mem.set_u32(me + DAMAGE_IMPACTS, 0);
    e.mem.set_u32(me + DAMAGE_IMPACTS + 4, 0);

    let scope_word = e.tls() + SCOPE_WORD;
    let saved_scope = e.mem.u32(scope_word);
    e.mem.set_u32(scope_word, 0x10);
    e.mem.set_u32(me + I_FLAGS, 0);
    store_vector(e, me + HK_SUPPORT_NORM, [0.0; 4]);
    e.mem.set_u32(me + LAST_MANIFOLD + 4, 0);
    e.call(CONTROLLER_INIT, &args![this, info]);
    e.mem.set_u32(scope_word, saved_scope);
    this
}

/// The wind globals the listener reads (`bhkwindlistener.rs`): the strength `01267c14` and the
/// phase `01267c18` (floats), and the strength limit `011b02dc` (float) the setter scales by.
const WIND_STRENGTH: u32 = 0x0126_7c14;
const WIND_PHASE: u32 = 0x0126_7c18;
const WIND_STRENGTH_LIMIT: u32 = 0x011b_02dc;

// Translated from 00c74550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wind setter (no map name; cdecl with two float stack words, `strength` and `phase`): the wind
/// strength global is the float at `011b02dc` times `strength` (x87, stored as float) and the
/// phase global is `phase` copied through the x87 stack.
pub fn fn_00c74550(e: &mut Engine, strength: f32, phase: f32) {
    let limit = e.mem.u32(WIND_STRENGTH_LIMIT);
    let product = x87_mul(x87_load32(limit), x87_m32(strength.to_bits()));
    e.mem.set_u32(WIND_STRENGTH, x87_store32(product));
    e.mem.set_u32(WIND_PHASE, x87_quiet_bits(phase.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c741e0, fn_00c741e0(Ptr, Ptr) -> Ptr),
        entry!(0x00c74550, fn_00c74550(f32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// Function address standing in for the allocator's slot +0xC.
    const SLOT_ALLOCATE: u32 = 0x7000_000c;
    /// Where the allocator's vtable lives.
    const ALLOCATOR_TABLE: u32 = 0x0120_8000;

    /// The up vector lanes the listener double read.
    type Lanes = Rc<RefCell<Vec<f32>>>;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn constructs_a_character_controller() {
        // (bytes the allocator grants, capacity word left in the array)
        for (granted, capacity) in [(0xe0u32, 2u32), (0x30, 0x8000_0000)] {
            let mut e = Engine::new();
            e.map(0x0100_0000, 0x0030_0000);
            e.call_log = Some(vec![]);
            e.mem.set_f32(FLOAT_ZERO, 0.0);
            e.mem.set_f32(FLOAT_ONE, 1.0);
            e.mem.set_f32(FLOAT_RADIUS, 2.0);
            e.mem.set_f32(FLOAT_HEIGHT, 10.0);
            e.mem.set_f32(FLOAT_LOD_DISTANCE, -1.0);
            e.mem.set_f32(MAX_TILT_DEGREES, 47.0);
            e.mem
                .set_f64(DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);

            let this = Ptr::new(e.mem.alloc(0x650));
            let info = e.mem.alloc(0xb0);
            e.mem.set_u32(info + 0x9c, 0x1111);
            e.mem.set_u32(info + 0xa4, 0x2222);
            e.mem.set_f32(info + 0xa8, 4.0);
            e.mem.set_f32(info + 0xac, 0.375);

            // The memory router: its allocator hands out a fixed block and
            // reports `granted` bytes through the size word it was given.
            let router = e.mem.alloc(0x20);
            let allocator = e.mem.alloc(8);
            e.mem.set_u32(router + 0x10, allocator);
            e.put_vtable(ALLOCATOR_TABLE, &[0, 0, 0, SLOT_ALLOCATE]);
            e.mem.set_u32(allocator, ALLOCATOR_TABLE);
            let block = e.mem.alloc(0x230);
            e.register_double(SLOT_ALLOCATE, move |e, a| {
                assert_eq!(e.mem.u32(a[1]), 0x230);
                e.mem.set_u32(a[1], granted);
                block.into_ret()
            });
            e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
            e.register(PROXY_CONSTRUCT, |_, _| Ret::default());
            e.register(CONTEXT_CONSTRUCT, |_, _| Ret::default());
            e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
            let lanes: Lanes = Rc::default();
            let seen = lanes.clone();
            e.register_double(LISTENER_CONSTRUCT, move |e, a| {
                for lane in 0..4 {
                    seen.borrow_mut().push(e.mem.f32(a[1] + 4 * lane));
                }
                Ret::default()
            });
            let scope = e.tls() + SCOPE_WORD;
            e.mem.set_u32(scope, 0x77);
            let during = Rc::new(Cell::new(0u32));
            let seen_scope = during.clone();
            e.register_double(CONTROLLER_INIT, move |e, _| {
                seen_scope.set(e.mem.u32(scope));
                Ret::default()
            });

            let result = fn_00c741e0(&mut e, this, Ptr::new(info));
            assert_eq!(result, this);
            let me = this.addr();

            assert_eq!(calls(&e, PROXY_CONSTRUCT), [vec![me]]);
            assert_eq!(
                calls(&e, CONTEXT_CONSTRUCT),
                [vec![me + HK_CONTEXT, 0x2222, 0x1111]]
            );
            let step = (0.375f32 as f64 * 4.0f32 as f64) as f32;
            let slope = (47.0f32 as f64 * (std::f64::consts::PI / 180.0)) as f32;
            let listener = calls(&e, LISTENER_CONSTRUCT).remove(0);
            assert_eq!(listener[0], me + LISTENER);
            assert_eq!(listener[2], slope.to_bits());
            assert_eq!(listener[3], step.to_bits());
            assert_eq!(*lanes.borrow(), [0.0, 0.0, 1.0, 0.0]);
            assert_eq!(
                calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
                [vec![
                    me + SP_SHAPES,
                    4,
                    2,
                    NI_POINTER_CONSTRUCT,
                    NI_POINTER_DESTRUCT
                ]]
            );
            assert_eq!(calls(&e, CONTROLLER_INIT), [vec![me, info]]);

            assert_eq!(e.mem.u32(me), VTABLE_CONTROLLER);
            assert_eq!(e.mem.u32(me + HK_CONTEXT), VTABLE_CONTEXT);
            assert_eq!(e.mem.u32(me + LISTENER), VTABLE_LISTENER);
            assert_eq!(e.mem.u32(me + SUPPORTED_STATE), 2);
            assert_eq!(e.mem.u32(me + SHAPE_TYPE), 2);
            assert_eq!(e.mem.u32(me + SIZED_SHAPE_TYPE), 2);
            for (offset, expected) in [
                (SURFACE_NORMAL, [0.0, 0.0, 1.0, 0.0]),
                (SURFACE_VELOCITY, [0.0; 4]),
                (UP_VEC, [0.0, 0.0, 1.0, 0.0]),
                (FORWARD_VEC, [1.0, 0.0, 0.0, 0.0]),
            ] {
                for (lane, value) in expected.into_iter().enumerate() {
                    assert_eq!(e.mem.f32(me + offset + 4 * lane as u32), value);
                }
            }
            assert_eq!(e.mem.f32(me + F_GRAVITY), 1.0);
            assert_eq!(e.mem.f32(me + F_PITCH_MULT), 1.0);
            assert_eq!(e.mem.u32(me + F_RADIUS), 2.0f32.to_bits());
            assert_eq!(e.mem.u32(me + F_DEST_RADIUS), 2.0f32.to_bits());
            assert_eq!(e.mem.u32(me + F_HEIGHT), 10.0f32.to_bits());
            assert_eq!(e.mem.u32(me + F_LOD_DISTANCE), (-1.0f32).to_bits());
            assert_eq!(e.mem.f32(me + F_BUMPED_FORCE), 0.0);

            // The manifold array: the allocator's block, the entry count reset
            // to 0 for `Init`, and the capacity from the granted bytes.
            assert_eq!(e.mem.u32(me + LAST_MANIFOLD), block);
            assert_eq!(e.mem.u32(me + LAST_MANIFOLD + 4), 0);
            assert_eq!(e.mem.u32(me + LAST_MANIFOLD + 8), capacity);
            // The scope word is 0x10 during `Init` and restored after.
            assert_eq!(during.get(), 0x10);
            assert_eq!(e.mem.u32(scope), 0x77);
        }
    }
}

#[cfg(test)]
mod wind_setter_tests {
    use super::*;

    #[test]
    fn the_wind_setter_scales_the_strength_by_the_limit() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(WIND_STRENGTH_LIMIT, 4.0f32);
        e.call(0x00c7_4550, &args![0.5f32, 3.25f32]);
        assert_eq!(e.global::<f32>(WIND_STRENGTH), 2.0);
        assert_eq!(e.global::<f32>(WIND_PHASE), 3.25);
    }
}

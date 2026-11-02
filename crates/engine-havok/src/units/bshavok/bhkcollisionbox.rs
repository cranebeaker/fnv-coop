//! `BSHavok/bhkcollisionbox.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCollisionBox` (0x20 bytes, vtable `010c477c`) is a `bhkSimpleShapePhantom` that follows a
//! character controller (`+0x18`): its `Update` copies the controller's position to the
//! phantom and sizes the phantom's shape from it.
//!
//! The exception-unwinding frame (`FS:[0]`) and the `/GS` cookie check (`00ec408c`) of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `~bhkSimpleShapePhantom` (engine map name, `00c9eec0`), its `Init` (`00c9f050`) and
/// `GetViewerStrings` (`00c9f270`).
const SIMPLE_SHAPE_PHANTOM_DESTRUCT: u32 = 0x00c9_eec0;
const SIMPLE_SHAPE_PHANTOM_INIT: u32 = 0x00c9_f050;
const SIMPLE_SHAPE_PHANTOM_GET_VIEWER_STRINGS: u32 = 0x00c9_f270;
/// `bhkWorldObject::Add` (engine map name, `00c85810`, thiscall: the world; answers a bool),
/// `GetAVObject` (`00c85c80`, thiscall on the owner: a word) and `SetAVObject` (`00c85c10`,
/// thiscall: the object and a word).
const WORLD_OBJECT_ADD: u32 = 0x00c8_5810;
const WORLD_OBJECT_GET_AV_OBJECT: u32 = 0x00c8_5c80;
const WORLD_OBJECT_SET_AV_OBJECT: u32 = 0x00c8_5c10;
/// `hkColor::rgbFromChars` (engine map name, `00cabb00`, cdecl: four bytes) and
/// `bhkWorld::SetDebugObjectColor` (engine map name, `00c68e30`, thiscall on the world: the
/// object and the color).
const COLOR_FROM_CHARS: u32 = 0x00ca_bb00;
const WORLD_SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;
/// The character controller's position (`00621a40`, engine map name
/// `bhkCharacterProxy::GethkPosition`, thiscall on the controller: answers a 16-byte vector)
/// and the function that takes a position vector (`006230f0`, thiscall on the box).
const CONTROLLER_GET_POSITION: u32 = 0x0062_1a40;
const SET_POSITION: u32 = 0x0062_30f0;
/// A function of no effect called on the box (`00a29680`: one `RET`).
const NO_EFFECT: u32 = 0x00a2_9680;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable of `bhkCollisionBox`.
const COLLISION_BOX_VTABLE: u32 = 0x010c_477c;
/// `bhkCollisionBox::bNoCollision` (Xbox PDB), and `bhkCollisionBox::ms_RTTI` (its first word is
/// the class name).
const NO_COLLISION: u32 = 0x0126_8440;
const COLLISION_BOX_RTTI: u32 = 0x0126_8444;
/// The default vector (`011f4288`) used when the owner has none, and the `float` constants the
/// update multiplies by: the controller position's scale (`011b21e8`) and the factor of the
/// shape's radius (`0126844c`).
const DEFAULT_VECTOR: u32 = 0x011f_4288;
const POSITION_SCALE: u32 = 0x011b_21e8;
const RADIUS_FACTOR: u32 = 0x0126_844c;
/// The debug color of a box added to a world: (30, 130, 0, 20).
const DEBUG_COLOR: [u32; 4] = [0x1e, 0x82, 0, 0x14];
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkCollisionBox` (Xbox PDB, 0x20 bytes).
    pub struct BhkCollisionBox: 0x20 {
        /// `phkObject` (Xbox PDB): the Havok phantom.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB).
        0x0C pInfo: Ptr,
        /// `iFlags` (Xbox PDB).
        0x10 iFlags: u32,
        /// `cFlags` (Xbox PDB).
        0x14 cFlags: u8,
        /// `pCharController` (Xbox PDB).
        0x18 pCharController: Ptr,
        /// `bDisabled` (Xbox PDB).
        0x1C bDisabled: bool,
    }
}

/// An x87 `FLD dword` of the float at `address`.
fn load_quiet(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

// Translated from 00cbe880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionBox::~bhkCollisionBox` (Xbox PDB): sets the vtable and continues in
/// `~bhkSimpleShapePhantom` (`00c9eec0`).
pub fn bhk_collision_box_destructor(e: &mut Engine, this: Ptr<BhkCollisionBox>) {
    e.mem.set_u32(this.addr(), COLLISION_BOX_VTABLE);
    e.call(SIMPLE_SHAPE_PHANTOM_DESTRUCT, &args![this]);
}

// Translated from 00cbe890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionBox::Add` (Xbox PDB), `(world)`: answers false at once for a disabled box
/// (`+0x1c`). Otherwise `bhkWorldObject::Add` (`00c85810`) adds it, and when that succeeded
/// the world gets the debug color (`rgbFromChars(30, 130, 0, 20)`, `00cabb00`) for the box
/// (`00c68e30`). Answers the result of the add.
pub fn bhk_collision_box_add(e: &mut Engine, this: Ptr<BhkCollisionBox>, world: Ptr) -> bool {
    if e.get(this, BhkCollisionBox::bDisabled) {
        return false;
    }
    let added = e.call(WORLD_OBJECT_ADD, &args![this, world]).bool();
    if added {
        let color = e
            .call(
                COLOR_FROM_CHARS,
                &args![
                    DEBUG_COLOR[0],
                    DEBUG_COLOR[1],
                    DEBUG_COLOR[2],
                    DEBUG_COLOR[3]
                ],
            )
            .u32();
        e.call(WORLD_SET_DEBUG_OBJECT_COLOR, &args![world, this, color]);
    }
    added
}

// Translated from 00cbe8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbe8d0 - the /GS cookie check is not modeled; the fourth lane of the vector in the first branch is uninitialised stack in the exe (here 0).
/// `bhkCollisionBox::Update` (Xbox PDB): does nothing while `bNoCollision` (`01268440`) is set
/// or without a character controller. The position vector and a weight come from the
/// controller's owner (`+0x594`) when it has one whose `GetAVObject(0)` (`00c85c80`) answers an
/// object: the vector at that object's `+0x20` (the default `011f4288` when 0), its three
/// first lanes copied and its fourth lane the weight. Otherwise the vector is the controller's
/// position (`00621a40`) with its first three lanes multiplied by the scale `011b21e8` and the
/// weight 1. A weight that is not above 0 (NaN included) becomes 1. The phantom's shape (the
/// first word of the Havok object's `+0x10`; `00a29680` runs twice on the box first) gets the
/// radius `0126844c` times the weight at its `+0x10`, and the box takes the vector
/// (`006230f0`).
pub fn bhk_collision_box_update(e: &mut Engine, this: Ptr<BhkCollisionBox>) {
    if e.mem.u8(NO_COLLISION) != 0 {
        return;
    }
    let controller = e.get(this, BhkCollisionBox::pCharController);
    if controller.addr() == 0 {
        return;
    }
    let owner = e.mem.u32(controller.addr() + 0x594);
    let object = if owner != 0 {
        e.call(WORLD_OBJECT_GET_AV_OBJECT, &args![owner, 0u32])
            .u32()
    } else {
        0
    };
    e.with_stack(0x10, |e, vector| {
        let base = vector.addr();
        let mut weight = if object != 0 {
            let mut source = e.mem.u32(object + 0x20);
            if source == 0 {
                source = DEFAULT_VECTOR;
            }
            for lane in 0..3 {
                let word = e.mem.u32(source + 4 * lane);
                e.mem.set_u32(base + 4 * lane, word);
            }
            load_quiet(e, source + 0xc)
        } else {
            let position = e.call(CONTROLLER_GET_POSITION, &args![controller]).u32();
            for lane in 0..4 {
                let word = e.mem.u32(position + 4 * lane);
                e.mem.set_u32(base + 4 * lane, word);
            }
            let scale = load_quiet(e, POSITION_SCALE);
            for lane in 0..3 {
                let value = load_quiet(e, base + 4 * lane);
                e.mem.set_f32(
                    base + 4 * lane,
                    (f64::from(value) * f64::from(scale)) as f32,
                );
            }
            1.0
        };
        let object_pointer = e.get(this, BhkCollisionBox::phkObject).addr();
        let shape = if object_pointer != 0 {
            e.call(NO_EFFECT, &args![this]);
            e.call(NO_EFFECT, &args![this]);
            e.mem.u32(object_pointer + 0x10)
        } else {
            0
        };
        let positive = weight > 0.0;
        if !positive {
            weight = 1.0;
        }
        let factor = load_quiet(e, RADIUS_FACTOR);
        e.mem
            .set_f32(shape + 0x10, (f64::from(factor) * f64::from(weight)) as f32);
        e.call(SET_POSITION, &args![this, vector]);
    });
}

// Translated from 00cbe9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionBox::Init` (Xbox PDB), `(cinfo, controller)`: clears `bDisabled`, runs
/// `bhkSimpleShapePhantom::Init` (`00c9f050`) on the Cinfo and stores the controller at
/// `+0x18`. With a controller, `SetAVObject` (`00c85c10`) is given the object its owner
/// (`+0x594`) answers from `GetAVObject(0)` (`00c85c80`), or 0 without an owner, and 0.
pub fn bhk_collision_box_init(
    e: &mut Engine,
    this: Ptr<BhkCollisionBox>,
    cinfo: Ptr,
    controller: Ptr,
) {
    e.set(this, BhkCollisionBox::bDisabled, false);
    e.call(SIMPLE_SHAPE_PHANTOM_INIT, &args![this, cinfo]);
    e.set(this, BhkCollisionBox::pCharController, controller);
    if controller.addr() == 0 {
        return;
    }
    let owner = e.mem.u32(controller.addr() + 0x594);
    let object = if owner != 0 {
        e.call(WORLD_OBJECT_GET_AV_OBJECT, &args![owner, 0u32])
            .u32()
    } else {
        0
    };
    e.call(WORLD_OBJECT_SET_AV_OBJECT, &args![this, object, 0u32]);
}

// Translated from 00cbea40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbea40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCollisionBox::GetViewerStrings` (Xbox PDB), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the base strings (`00c9f270`) and the class name
/// (`NiGetViewerString` of the first word of the RTTI at `01268444`) stored in the string array.
pub fn bhk_collision_box_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkCollisionBox>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(
        SIMPLE_SHAPE_PHANTOM_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let name = e.mem.u32(COLLISION_BOX_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cbe880,
            bhk_collision_box_destructor(Ptr<BhkCollisionBox>)
        ),
        entry!(
            0x00cbe890,
            bhk_collision_box_add(Ptr<BhkCollisionBox>, Ptr) -> bool
        ),
        entry!(0x00cbe8d0, bhk_collision_box_update(Ptr<BhkCollisionBox>)),
        entry!(
            0x00cbe9f0,
            bhk_collision_box_init(Ptr<BhkCollisionBox>, Ptr, Ptr)
        ),
        entry!(
            0x00cbea40,
            bhk_collision_box_get_viewer_strings(Ptr<BhkCollisionBox>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    #[test]
    fn bhk_collision_box_destructor_sets_the_vtable_and_runs_the_phantom_destructor() {
        let mut e = logged_engine();
        e.register(SIMPLE_SHAPE_PHANTOM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00cb_e880, &args![this]);
        assert_eq!(e.mem.u32(this), COLLISION_BOX_VTABLE);
        assert_eq!(
            calls_to(&e, SIMPLE_SHAPE_PHANTOM_DESTRUCT),
            vec![vec![this]]
        );
    }

    #[test]
    fn bhk_collision_box_add_colors_the_box_in_the_world_when_it_was_added() {
        let mut e = logged_engine();
        e.register_double(WORLD_OBJECT_ADD, |_, _| 1u32.into_ret());
        e.register(COLOR_FROM_CHARS, |_, _| 0xaabbccddu32.into_ret());
        e.register(WORLD_SET_DEBUG_OBJECT_COLOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let world = 0x5000u32;
        assert!(e.call(0x00cb_e890, &args![this, world]).bool());
        assert_eq!(calls_to(&e, WORLD_OBJECT_ADD), vec![vec![this, world]]);
        assert_eq!(
            calls_to(&e, COLOR_FROM_CHARS),
            vec![vec![0x1e, 0x82, 0, 0x14]]
        );
        assert_eq!(
            calls_to(&e, WORLD_SET_DEBUG_OBJECT_COLOR),
            vec![vec![world, this, 0xaabb_ccdd]]
        );
        let order = vec![
            WORLD_OBJECT_ADD,
            COLOR_FROM_CHARS,
            WORLD_SET_DEBUG_OBJECT_COLOR,
        ];
        assert_eq!(order_of(&e, &order), order);
        // A failed add answers false without a color.
        e.register_double(WORLD_OBJECT_ADD, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00cb_e890, &args![this, world]).bool());
        assert_eq!(calls_to(&e, COLOR_FROM_CHARS).len(), 1);
        // A disabled box does nothing.
        e.mem.set_u8(this + 0x1c, 1);
        assert!(!e.call(0x00cb_e890, &args![this, world]).bool());
        assert_eq!(calls_to(&e, WORLD_OBJECT_ADD).len(), 2);
    }

    /// The update scene: a box whose Havok object (`+8`) has a shape at its `+0x10` word, and a
    /// controller. Float constants: the position scale 0.5, the radius factor 3.0.
    struct Scene {
        this: u32,
        shape: u32,
        controller: u32,
    }

    fn update_scene(e: &mut Engine) -> Scene {
        e.register(NO_EFFECT, |_, _| Ret::default());
        e.register(SET_POSITION, |e, a| {
            // Keeps the vector the box takes, in the box's flags words.
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(0x7200_0000, &bytes);
            Ret::default()
        });
        e.map(0x7200_0000, 0x10);
        e.set_global(POSITION_SCALE, 0.5f32);
        e.set_global(RADIUS_FACTOR, 3.0f32);
        e.mem.set_u8(NO_COLLISION, 0);
        let this = e.mem.alloc(0x20);
        let object = e.mem.alloc(0x20);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x10, shape);
        e.mem.set_u32(this + 8, object);
        let controller = e.mem.alloc(0x600);
        e.mem.set_u32(this + 0x18, controller);
        Scene {
            this,
            shape,
            controller,
        }
    }

    fn sent_vector(e: &Engine) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(0x7200_0000 + 4 * lane as u32))
    }

    #[test]
    fn bhk_collision_box_update_follows_the_controller_position() {
        let mut e = logged_engine();
        let scene = update_scene(&mut e);
        let position = e.mem.alloc(0x10);
        for (lane, value) in [2.0f32, 4.0, 6.0, 8.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        e.register(CONTROLLER_GET_POSITION, |_, a| {
            // The position is the controller's first 0x10 bytes' address; kept by the test.
            (a[0] + 0x100).into_ret()
        });
        for (lane, value) in [2.0f32, 4.0, 6.0, 8.0].into_iter().enumerate() {
            e.mem
                .set_f32(scene.controller + 0x100 + 4 * lane as u32, value);
        }
        e.call(0x00cb_e8d0, &args![scene.this]);
        // No owner: the position scaled by 0.5, the fourth lane kept, the weight 1.
        assert_eq!(sent_vector(&e), [1.0, 2.0, 3.0, 8.0]);
        assert_eq!(e.mem.f32(scene.shape + 0x10), 3.0);
        assert_eq!(calls_to(&e, NO_EFFECT), vec![vec![scene.this]; 2]);
        assert_eq!(calls_to(&e, SET_POSITION).len(), 1);
        assert_eq!(calls_to(&e, SET_POSITION)[0][0], scene.this);
        // bNoCollision, or no controller, makes it a no-op.
        e.mem.set_u8(NO_COLLISION, 1);
        e.call(0x00cb_e8d0, &args![scene.this]);
        e.mem.set_u8(NO_COLLISION, 0);
        e.mem.set_u32(scene.this + 0x18, 0);
        e.call(0x00cb_e8d0, &args![scene.this]);
        assert_eq!(calls_to(&e, SET_POSITION).len(), 1);
    }

    #[test]
    fn bhk_collision_box_update_takes_the_vector_and_weight_of_the_owner_object() {
        let mut e = logged_engine();
        let scene = update_scene(&mut e);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(scene.controller + 0x594, owner);
        let av_object = e.mem.alloc(0x30);
        let vector = e.mem.alloc(0x10);
        for (lane, value) in [7.0f32, 8.0, 9.0, 2.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * lane as u32, value);
        }
        e.mem.set_u32(av_object + 0x20, vector);
        e.register_double(WORLD_OBJECT_GET_AV_OBJECT, move |_, _| av_object.into_ret());
        e.call(0x00cb_e8d0, &args![scene.this]);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_GET_AV_OBJECT),
            vec![vec![owner, 0]]
        );
        let sent = sent_vector(&e);
        assert_eq!(sent[..3], [7.0, 8.0, 9.0]);
        // The weight 2 times the factor 3.
        assert_eq!(e.mem.f32(scene.shape + 0x10), 6.0);
        // A weight that is not above zero becomes 1; an object without a vector uses the
        // default vector.
        e.mem.set_f32(vector + 0xc, 0.0);
        e.call(0x00cb_e8d0, &args![scene.this]);
        assert_eq!(e.mem.f32(scene.shape + 0x10), 3.0);
        e.mem.set_u32(vector + 0xc, 0x7fc0_0000);
        e.call(0x00cb_e8d0, &args![scene.this]);
        assert_eq!(e.mem.f32(scene.shape + 0x10), 3.0);
        e.mem.set_u32(av_object + 0x20, 0);
        e.mem.set_f32(DEFAULT_VECTOR, 1.5);
        e.mem.set_f32(DEFAULT_VECTOR + 0xc, 4.0);
        e.call(0x00cb_e8d0, &args![scene.this]);
        assert_eq!(sent_vector(&e)[0], 1.5);
        assert_eq!(e.mem.f32(scene.shape + 0x10), 12.0);
    }

    #[test]
    fn bhk_collision_box_init_stores_the_controller_and_the_owner_object() {
        let mut e = logged_engine();
        e.register(SIMPLE_SHAPE_PHANTOM_INIT, |_, _| Ret::default());
        e.register(WORLD_OBJECT_SET_AV_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x1c, 1);
        let controller = e.mem.alloc(0x600);
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(controller + 0x594, owner);
        e.register_double(WORLD_OBJECT_GET_AV_OBJECT, |_, _| 0x4444u32.into_ret());
        e.call(0x00cb_e9f0, &args![this, 0x3000u32, controller]);
        assert_eq!(e.mem.u8(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x18), controller);
        assert_eq!(
            calls_to(&e, SIMPLE_SHAPE_PHANTOM_INIT),
            vec![vec![this, 0x3000]]
        );
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_GET_AV_OBJECT),
            vec![vec![owner, 0]]
        );
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_SET_AV_OBJECT),
            vec![vec![this, 0x4444, 0]]
        );
        // Without an owner the object is 0; without a controller nothing is set.
        e.mem.set_u32(controller + 0x594, 0);
        e.call(0x00cb_e9f0, &args![this, 0x3000u32, controller]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_GET_AV_OBJECT).len(), 1);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_SET_AV_OBJECT)[1],
            vec![this, 0, 0]
        );
        e.call(0x00cb_e9f0, &args![this, 0x3000u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls_to(&e, WORLD_OBJECT_SET_AV_OBJECT).len(), 2);
        assert_eq!(calls_to(&e, SIMPLE_SHAPE_PHANTOM_INIT).len(), 3);
    }

    #[test]
    fn bhk_collision_box_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(SIMPLE_SHAPE_PHANTOM_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(COLLISION_BOX_RTTI, 0x4444);
        let this = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cb_ea40, &args![this, array]);
        assert_eq!(
            calls_to(&e, SIMPLE_SHAPE_PHANTOM_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}

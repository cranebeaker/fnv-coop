//! `fallout shared/sky/stars.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0063f940` (named `SkyObject::Initialize` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INITIALIZE_TARGET_0063F940: u32 = 0x0063_f940;

// Translated from 0063fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Stars::Initialize` (Xbox PDB `?Initialize@Stars@@UAAXPAVNiNode@@@Z`, void, vtable word
/// 0104f1e0): forwards `this` and the `NiNode` pointer to the base body `0063f940`
/// (`SkyObject::Initialize`). Returns nothing: the decorated name is void; the callee ends
/// with an exception-frame unlink after a virtual call on its argument.
pub fn stars_initialize(e: &mut Engine, this: Ptr, root: Ptr) {
    e.call(INITIALIZE_TARGET_0063F940, &args![this, root]);
}

// `Stars` (Xbox PDB, 0x10 bytes): the `SkyObject` base (vtable, `spRoot`
// `NiPointer<NiNode>` +4), `spStars` (`NiPointer<NiNode>`, +8) and
// `fAlpha` (+0xC); the PC code uses the same offsets.

/// `Stars`'s vtable (the one the destructor sets).
const STARS_VTABLE: u32 = 0x0104_f1d8;
/// `SkyObject::spRoot` (Xbox PDB).
const ROOT: u32 = 0x4;
/// `spStars` (Xbox PDB).
const STARS_NODE: u32 = 0x8;
/// `fAlpha` (Xbox PDB).
const ALPHA: u32 = 0xc;

/// the allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// thiscall on an `NiPointer`: the object it holds.
const POINTER_GET: u32 = 0x0055_9450;
/// thiscall (pointer, object): `NiPointer` assignment.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// thiscall: `NiPointer` destructor.
const POINTER_DESTROY: u32 = 0x0045_cec0;
/// `SkyObject::~SkyObject` (engine map), thiscall.
const SKY_OBJECT_DESTROY: u32 = 0x0063_f8d0;
/// thiscall on a node (node, flag): the call `LoadGeometry` makes with 0
/// when it has no stars and 1 once they are loaded (`00635fe0`).
const NODE_SET_FLAG_635FE0: u32 = 0x0063_5fe0;
/// `BSStringT` constructor and destructor (thiscall).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// cdecl (string, format, ...): formats into a `BSStringT`.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The format and the two words `LoadGeometry` formats the file name with (read from the exe).
const PATH_FORMAT: u32 = 0x0104_f208;
const PATH_PART_A: u32 = 0x0101_dccc;
const PATH_PART_B: u32 = 0x0101_3444;
/// The stream's constructor and `BSStream::~BSStream` (engine map), thiscall.
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_DESTROY: u32 = 0x0043_d090;
/// `BSStream::Load` (engine map), thiscall (stream, path, 0).
const STREAM_LOAD: u32 = 0x00c3_a8a0;
/// thiscall on the stream: the number of top-level objects.
const STREAM_OBJECT_COUNT: u32 = 0x0043_b1e0;
/// thiscall (stream, index): a top-level object.
const STREAM_OBJECT: u32 = 0x0043_b200;
/// cdecl (type, object): whether the object is of the type.
const IS_KIND_OF: u32 = 0x0043_b300;
/// The type `LoadGeometry` checks the loaded object against.
const NODE_TYPE: u32 = 0x011f_4428;
/// cdecl (text): prints a message.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The message printed when the stars file does not load as one node.
const LOAD_FAILED_MESSAGE: u32 = 0x0104_f1e8;
/// `Model::Model` (engine map), thiscall (model, path, stream, 1, 0).
const MODEL_CONSTRUCT: u32 = 0x0043_aaf0;
/// `Model::~Model` (engine map), thiscall.
const MODEL_DESTROY: u32 = 0x0043_ab70;
/// thiscall on a `Model`: its root node.
const MODEL_ROOT: u32 = 0x0043_b230;
/// thiscall (node, flag) on the new stars node.
const NODE_SET_FLAG_4902F0: u32 = 0x0049_02f0;
/// `NiAVObject::UpdateProperties` (engine map), thiscall.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The root's vtable slots: detach a child (+0xE8, one word) and attach a
/// child (+0xDC, child and 1).
const NODE_SLOT_DETACH_CHILD: u32 = 0xe8;
const NODE_SLOT_ATTACH_CHILD: u32 = 0xdc;

/// thiscall on the sky: a value `Update` returns early on when 0.
const SKY_CHECK_44EDB0: u32 = 0x0044_edb0;
/// thiscall on the sky: the hour (float at +0xEC, in ST0).
const SKY_HOUR: u32 = 0x0096_6a20;
/// The guard word (bit 0) and the `NiPoint3` of `Update`'s function-local
/// static: the last position given to the stars.
const LAST_POSITION_GUARD: u32 = 0x011c_ce48;
const LAST_POSITION: u32 = 0x011c_ce3c;
/// `NiPoint3` constructor, thiscall (point, x, y, z).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// The base update (`008d0600`, `ProcessLists::PrintLists` in the engine
/// map; folded code), thiscall (object, sky, seconds).
const BASE_UPDATE: u32 = 0x008d_0600;
/// thiscall on the sky: its mode (+0xF8).
const SKY_MODE: u32 = 0x0047_6c90;
/// thiscall on the root node: whether the stars may show (`006366b0`).
const ROOT_SHOWABLE: u32 = 0x0063_66b0;
/// thiscall (node, culled): culls or shows the node (`00450f90`).
const NODE_SET_CULLED: u32 = 0x0045_0f90;
/// thiscall on the sky: the four hours of the stars' fade (floats in ST0),
/// in the order `Update` reads them.
const SKY_HOUR_595FC0: u32 = 0x0059_5fc0;
const SKY_HOUR_63BA30: u32 = 0x0063_ba30;
const SKY_HOUR_595F50: u32 = 0x0059_5f50;
const SKY_HOUR_63B9B0: u32 = 0x0063_b9b0;
/// The `double` fraction of the fade spans (`01011588`).
const FADE_FRACTION: u32 = 0x0101_1588;
/// The `double` the alpha must exceed for the stars to show (`01012060`).
const VISIBLE_ALPHA: u32 = 0x0101_2060;
/// thiscall on the sky: a pointer to the position (three floats).
const SKY_POSITION: u32 = 0x0057_eef0;
/// thiscall on a node: its number of children.
const NODE_CHILD_COUNT: u32 = 0x0045_3470;
/// thiscall (node, index): a child.
const NODE_CHILD: u32 = 0x0043_b4a0;
/// The child's vtable slot answering its geometry (+0x18, no words).
const CHILD_SLOT_GEOMETRY: u32 = 0x18;
/// `NiAVObject::GetProperty` (engine map), thiscall (object, type).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// thiscall on a property: its kind (+0x1C).
const PROPERTY_KIND: u32 = 0x0044_1110;
/// thiscall (point, other): a comparison of the two points (the negation
/// of `004390c0`'s answer); the position is copied when it answers true.
const POINTS_DIFFER: u32 = 0x0043_9090;
/// thiscall on the calendar (`011de7b8`): the days passed (an integer).
const CALENDAR_DAYS_PASSED: u32 = 0x0086_7de0;
const CALENDAR: u32 = 0x011d_e7b8;
/// The `double` 24.0 (hours per day, `010356d8`) and the `double` factor
/// of the rotation (`0101ff48`).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
const ROTATION_FACTOR: u32 = 0x0101_ff48;
/// thiscall on a game setting: the address of its float value.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// The settings `Update` reads: the rotation period and the axis (x, y, z).
const SETTING_PERIOD: u32 = 0x011c_ce0c;
const SETTING_AXIS_X: u32 = 0x011c_cdc4;
const SETTING_AXIS_Y: u32 = 0x011c_ce30;
const SETTING_AXIS_Z: u32 = 0x011c_cddc;
/// cdecl (value, divisor): a float remainder (through `fmod`), in ST0.
const FLOAT_REMAINDER: u32 = 0x004b_1520;
/// thiscall on a point: normalizes it (`004a0c10`).
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// thiscall (node, angle, x, y, z): rotates the stars node.
const NODE_ROTATE: u32 = 0x0064_04b0;

fn stars_node(e: &mut Engine, this: u32) -> u32 {
    e.call(POINTER_GET, &args![this + STARS_NODE]).u32()
}

fn root_node(e: &mut Engine, this: u32) -> u32 {
    e.call(POINTER_GET, &args![this + ROOT]).u32()
}

// Translated from 0063fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Stars::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// (`0063fd20`) and, when bit 0 of the flags is set, frees the object.
/// Answers `this`.
pub fn stars_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    stars_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0063fd20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0063fd20 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a member destructor throws is not translated
/// `Stars::~Stars` (Xbox PDB): sets the vtable, releases `spStars`
/// (assigns 0, then its destructor) and destroys the `SkyObject` base.
pub fn stars_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, STARS_VTABLE);
    e.call(POINTER_ASSIGN, &args![base + STARS_NODE, 0u32]);
    e.call(POINTER_DESTROY, &args![base + STARS_NODE]);
    e.call(SKY_OBJECT_DESTROY, &args![base]);
}

// Translated from 0063fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0063fdb0 - the C++ exception-unwinding frame (FS:[0]) that destroys the locals when a call throws, and the /GS stack cookie check (00ec408c), are not translated
/// `Stars::LoadGeometry` (Xbox PDB), thiscall `(name)`: with no name (or
/// an empty one) calls `00635fe0(0)` on the root. Otherwise formats the
/// file name into a string, loads it into a stream (a 0x5D4-byte local);
/// unless the stream holds exactly one object of the node type at
/// `011f4428`, prints the failure message and calls `00635fe0(0)` on the
/// root. When it does, builds a `Model` (a 0x10-byte local) from the
/// stream, calls `00635fe0(1)` on the root, detaches the old stars node
/// from the root, takes the model's node as `spStars`, calls `004902f0(1)`
/// on it, attaches it to the root and updates its properties. The locals
/// are destroyed in reverse order.
pub fn stars_load_geometry(e: &mut Engine, this: Ptr, name: u32) {
    let base = this.addr();
    if name == 0 || e.mem.u8(name) == 0 {
        let root = root_node(e, base);
        e.call(NODE_SET_FLAG_635FE0, &args![root, 0u32]);
        return;
    }
    e.with_stack(8, |e, path| {
        e.call(STRING_CONSTRUCT, &args![path]);
        e.call(
            STRING_FORMAT,
            &args![path, PATH_FORMAT, PATH_PART_A, PATH_PART_B, name],
        );
        e.with_stack(0x5d4, |e, stream| {
            e.call(STREAM_CONSTRUCT, &args![stream]);
            let text = e.call(POINTER_GET, &args![path]).u32();
            let loaded = e.call(STREAM_LOAD, &args![stream, text, 0u32]).bool()
                && e.call(STREAM_OBJECT_COUNT, &args![stream]).u32() == 1
                && {
                    let object = e.call(STREAM_OBJECT, &args![stream, 0u32]).u32();
                    e.call(IS_KIND_OF, &args![NODE_TYPE, object]).bool()
                };
            if !loaded {
                e.call(PRINT_MESSAGE, &args![LOAD_FAILED_MESSAGE]);
                let root = root_node(e, base);
                e.call(NODE_SET_FLAG_635FE0, &args![root, 0u32]);
            } else {
                e.with_stack(0x10, |e, model| {
                    let text = e.call(POINTER_GET, &args![path]).u32();
                    e.call(MODEL_CONSTRUCT, &args![model, text, stream, 1u32, 0u32]);
                    let root = root_node(e, base);
                    e.call(NODE_SET_FLAG_635FE0, &args![root, 1u32]);
                    let root = root_node(e, base);
                    let old = stars_node(e, base);
                    e.vcall(root, NODE_SLOT_DETACH_CHILD, &args![old]);
                    let node = e.call(MODEL_ROOT, &args![model]).u32();
                    e.call(POINTER_ASSIGN, &args![base + STARS_NODE, node]);
                    let node = stars_node(e, base);
                    e.call(NODE_SET_FLAG_4902F0, &args![node, 1u32]);
                    let root = root_node(e, base);
                    let node = stars_node(e, base);
                    e.vcall(root, NODE_SLOT_ATTACH_CHILD, &args![node, 1u32]);
                    let node = stars_node(e, base);
                    e.call(UPDATE_PROPERTIES, &args![node]);
                    e.call(MODEL_DESTROY, &args![model]);
                });
            }
            e.call(STREAM_DESTROY, &args![stream]);
        });
        e.call(STRING_DESTROY, &args![path]);
    });
}

/// A float the code keeps after `FSTP dword` of a callee's `ST0`.
fn float_result(e: &mut Engine, callee: u32, object: u32) -> u32 {
    x87_store32(e.call(callee, &args![object]).f64())
}

// Translated from 00640020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00640020 - the C++ exception-unwinding frame (FS:[0]) of the function-local static's construction is not translated
/// `Stars::Update` (Xbox PDB), thiscall `(sky, seconds)`: unless the sky's
/// `0044edb0` answers 0, clears `fAlpha`, constructs the function-local
/// last position (0, 0, 0) once, runs the base update, and in sky modes 3
/// and 2 only: culls the root when `006366b0` says the stars may not show;
/// otherwise computes `fAlpha` from the hour and the four fade hours `a`,
/// `b`, `c`, `d` (`b - (b - a) * k` and `c - (c - d) * k` with the `double`
/// `k` at `01011588`): the fraction of one of the two fades, 0 between
/// them and 1 otherwise (`fade_alpha` has the exact tests). With `fAlpha` above the `double` at
/// `01012060` it shows the root, gives each child geometry's property of
/// type 3 and kind 0xD the sky's position (when it moved) and the alpha,
/// keeps the position, and rotates the stars node by the angle of the day
/// (days passed plus hour / 24, a remainder of the period setting, times
/// the factor at `0101ff48`, over the period) about the normalized axis
/// setting; otherwise culls the root.
pub fn stars_update(e: &mut Engine, this: Ptr, sky: u32, seconds: u32) {
    let base = this.addr();
    if e.call(SKY_CHECK_44EDB0, &args![sky]).u32() == 0 {
        return;
    }
    let hour = float_result(e, SKY_HOUR, sky);
    e.mem.set_u32(base + ALPHA, 0.0f32.to_bits());
    let guard: u32 = e.global(LAST_POSITION_GUARD);
    if guard & 1 == 0 {
        e.set_global(LAST_POSITION_GUARD, guard | 1);
        e.call(POINT_CONSTRUCT, &args![LAST_POSITION, 0u32, 0u32, 0u32]);
    }
    e.call(BASE_UPDATE, &args![base, sky, x87_quiet_bits(seconds)]);
    if e.call(SKY_MODE, &args![sky]).u32() != 3 && e.call(SKY_MODE, &args![sky]).u32() != 2 {
        return;
    }
    let root = root_node(e, base);
    if !e.call(ROOT_SHOWABLE, &args![root]).bool() {
        let root = root_node(e, base);
        e.call(NODE_SET_CULLED, &args![root, 1u32]);
        return;
    }
    let a = float_result(e, SKY_HOUR_595FC0, sky);
    let b = float_result(e, SKY_HOUR_63BA30, sky);
    let c = float_result(e, SKY_HOUR_595F50, sky);
    let d = float_result(e, SKY_HOUR_63B9B0, sky);
    let fraction: f64 = e.global(FADE_FRACTION);
    let from_b = x87_store32(x87_sub(
        x87_m32(b),
        x87_mul(x87_sub(x87_load32(b), x87_m32(a)), fraction),
    ));
    let from_c = x87_store32(x87_sub(
        x87_m32(c),
        x87_mul(x87_sub(x87_load32(c), x87_m32(d)), fraction),
    ));
    let alpha = fade_alpha(hour, b, d, from_b, from_c);
    e.mem.set_u32(base + ALPHA, alpha);
    let visible: f64 = e.global(VISIBLE_ALPHA);
    let shows = x87_load32(e.mem.u32(base + ALPHA)) > visible;
    if !shows {
        let root = root_node(e, base);
        e.call(NODE_SET_CULLED, &args![root, 1u32]);
        return;
    }
    let root = root_node(e, base);
    e.call(NODE_SET_CULLED, &args![root, 0u32]);
    let at = e.call(SKY_POSITION, &args![sky]).u32();
    e.with_stack(0xc, |e, position| {
        let position = position.addr();
        for word in 0..3 {
            let value = e.mem.u32(at + 4 * word);
            e.mem.set_u32(position + 4 * word, value);
        }
        update_children(e, base, position);
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.set_global(LAST_POSITION + 4 * word, value);
        }
    });
    rotate_stars(e, base, sky);
}

/// `fAlpha` of [`stars_update`] (bits of the stored float) from the hour and
/// the fade hours, with the exe's comparisons (a NaN takes the last branch
/// of each test).
fn fade_alpha(hour: u32, b: u32, d: u32, from_b: u32, from_c: u32) -> u32 {
    let (hour_value, b_value, d_value) = (x87_load32(hour), x87_load32(b), x87_load32(d));
    let (from_b_value, from_c_value) = (x87_load32(from_b), x87_load32(from_c));
    if hour_value > d_value && from_c_value > hour_value {
        let rest = x87_sub(x87_load32(from_c), x87_m32(hour));
        let span = x87_sub(x87_load32(from_c), x87_m32(d));
        return x87_store32(x87_div(rest, span));
    }
    if hour_value >= from_c_value && from_b_value >= hour_value {
        return 0.0f32.to_bits();
    }
    if hour_value > from_b_value && b_value > hour_value {
        let done = x87_sub(x87_load32(hour), x87_m32(from_b));
        let span = x87_sub(x87_load32(b), x87_m32(from_b));
        return x87_store32(x87_div(done, span));
    }
    1.0f32.to_bits()
}

/// The children loop of [`stars_update`]: each child of the stars node
/// with a geometry whose property of type 3 is of kind 0xD gets the
/// position (when it differs from the last one) and the alpha.
fn update_children(e: &mut Engine, base: u32, position: u32) {
    let count = if stars_node(e, base) != 0 {
        let node = stars_node(e, base);
        e.call(NODE_CHILD_COUNT, &args![node]).u32()
    } else {
        0
    };
    let mut index = 0u32;
    while index < count {
        let node = stars_node(e, base);
        let child = e.call(NODE_CHILD, &args![node, index]).u32();
        if child != 0 {
            let geometry = e.vcall(child, CHILD_SLOT_GEOMETRY, &args![]).u32();
            if geometry != 0 {
                let property = matching_property(e, geometry);
                if property != 0 {
                    if e.call(POINTS_DIFFER, &args![LAST_POSITION, position])
                        .bool()
                    {
                        for word in 0..3 {
                            let value = x87_quiet_bits(e.mem.u32(position + 4 * word));
                            e.mem.set_u32(property + 0x60 + 4 * word, value);
                        }
                    }
                    let alpha = x87_quiet_bits(e.mem.u32(base + ALPHA));
                    e.mem.set_u32(property + 0x6c, alpha);
                }
            }
        }
        index = index.wrapping_add(1);
    }
}

/// The geometry's property of type 3 when its kind is 0xD, else 0.
fn matching_property(e: &mut Engine, geometry: u32) -> u32 {
    if e.call(GET_PROPERTY, &args![geometry, 3u32]).u32() == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    if e.call(PROPERTY_KIND, &args![property]).u32() != 0xd {
        return 0;
    }
    e.call(GET_PROPERTY, &args![geometry, 3u32]).u32()
}

/// The rotation of [`stars_update`].
fn rotate_stars(e: &mut Engine, base: u32, sky: u32) {
    let days = f64::from(e.call(CALENDAR_DAYS_PASSED, &args![CALENDAR]).u32());
    let hour = e.call(SKY_HOUR, &args![sky]).f64();
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    let day = x87_store32(x87_add(x87_div(hour, hours_per_day), days));
    let period_at = e.call(SETTING_FLOAT, &args![SETTING_PERIOD]).u32();
    let period = x87_quiet_bits(e.mem.u32(period_at));
    let remainder = e
        .call(FLOAT_REMAINDER, &args![x87_quiet_bits(day), period])
        .f64();
    let factor: f64 = e.global(ROTATION_FACTOR);
    let scaled = x87_mul(remainder, factor);
    let period_at = e.call(SETTING_FLOAT, &args![SETTING_PERIOD]).u32();
    let angle = x87_store32(x87_div(scaled, x87_load32(e.mem.u32(period_at))));
    let z_at = e.call(SETTING_FLOAT, &args![SETTING_AXIS_Z]).u32();
    let z = x87_quiet_bits(e.mem.u32(z_at));
    let y_at = e.call(SETTING_FLOAT, &args![SETTING_AXIS_Y]).u32();
    let y = x87_quiet_bits(e.mem.u32(y_at));
    let x_at = e.call(SETTING_FLOAT, &args![SETTING_AXIS_X]).u32();
    let x = x87_quiet_bits(e.mem.u32(x_at));
    e.with_stack(0xc, |e, axis| {
        e.call(POINT_CONSTRUCT, &args![axis, x, y, z]);
        e.call(POINT_NORMALIZE, &args![axis]);
        if stars_node(e, base) != 0 {
            let axis = axis.addr();
            let (x, y, z) = (
                x87_quiet_bits(e.mem.u32(axis)),
                x87_quiet_bits(e.mem.u32(axis + 4)),
                x87_quiet_bits(e.mem.u32(axis + 8)),
            );
            let node = stars_node(e, base);
            e.call(NODE_ROTATE, &args![node, x87_quiet_bits(angle), x, y, z]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_fd90, stars_initialize(Ptr, Ptr)),
        entry!(
            0x0063fcf0,
            stars_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0063fd20, stars_destructor(Ptr)),
        entry!(0x0063fdb0, stars_load_geometry(Ptr, u32)),
        entry!(0x00640020, stars_update(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_stars_initialize_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INITIALIZE_TARGET_0063F940, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0063_fd90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INITIALIZE_TARGET_0063F940)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod stars_tests {
    use super::*;

    const DETACH_DOUBLE: u32 = 0xdead_3000;
    const ATTACH_DOUBLE: u32 = 0xdead_3010;
    const GEOMETRY_DOUBLE: u32 = 0xdead_3020;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// A `Stars` whose root has the detach and attach slots, and whose
    /// `NiPointer`s are plain words (`00559450` reads the word).
    fn stars(e: &mut Engine) -> (u32, u32) {
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + NODE_SLOT_DETACH_CHILD, DETACH_DOUBLE);
        e.mem
            .set_u32(vtable + NODE_SLOT_ATTACH_CHILD, ATTACH_DOUBLE);
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(root, vtable);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + ROOT, root);
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet(e, &[DETACH_DOUBLE, ATTACH_DOUBLE]);
        (this, root)
    }

    #[test]
    fn deleting_destructor_releases_the_stars_node() {
        let mut e = Engine::new();
        let (this, _) = stars(&mut e);
        quiet(&mut e, &[POINTER_DESTROY, SKY_OBJECT_DESTROY, FREE]);
        e.mem.set_u32(this + STARS_NODE, 0x500);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_fcf0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), STARS_VTABLE);
        assert_eq!(e.mem.u32(this + STARS_NODE), 0);
        assert_eq!(calls(&e, POINTER_DESTROY), vec![vec![this + 8]]);
        assert_eq!(calls(&e, SKY_OBJECT_DESTROY), vec![vec![this]]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0063_fcf0, &args![this, 0u32]);
        assert!(calls(&e, FREE).is_empty());
    }

    fn load_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                NODE_SET_FLAG_635FE0,
                STRING_CONSTRUCT,
                STRING_DESTROY,
                STRING_FORMAT,
                STREAM_CONSTRUCT,
                STREAM_DESTROY,
                PRINT_MESSAGE,
                MODEL_CONSTRUCT,
                MODEL_DESTROY,
                NODE_SET_FLAG_4902F0,
                UPDATE_PROPERTIES,
            ],
        );
        e.register(STREAM_LOAD, |_, _| ret(1));
        e.register(STREAM_OBJECT_COUNT, |_, _| ret(1));
        e.register(STREAM_OBJECT, |_, _| ret(0x600));
        e.register(IS_KIND_OF, |_, _| ret(1));
        e.register(MODEL_ROOT, |_, _| ret(0x700));
    }

    #[test]
    fn load_geometry_swaps_in_the_loaded_node() {
        let mut e = Engine::new();
        let (this, root) = stars(&mut e);
        load_doubles(&mut e);
        e.mem.set_u32(this + STARS_NODE, 0x500);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, 0x78);
        e.call_log = Some(vec![]);
        e.call(0x0063_fdb0, &args![this, name]);
        let format = calls(&e, STRING_FORMAT);
        assert_eq!(
            format[0][1..],
            [PATH_FORMAT, PATH_PART_A, PATH_PART_B, name]
        );
        assert_eq!(calls(&e, NODE_SET_FLAG_635FE0), vec![vec![root, 1]]);
        assert_eq!(calls(&e, DETACH_DOUBLE), vec![vec![root, 0x500]]);
        assert_eq!(e.mem.u32(this + STARS_NODE), 0x700);
        assert_eq!(calls(&e, NODE_SET_FLAG_4902F0), vec![vec![0x700, 1]]);
        assert_eq!(calls(&e, ATTACH_DOUBLE), vec![vec![root, 0x700, 1]]);
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![0x700]]);
        let model = calls(&e, MODEL_CONSTRUCT);
        assert_eq!(model[0][3..], [1, 0]);
        assert_eq!(calls(&e, MODEL_DESTROY).len(), 1);
        assert_eq!(calls(&e, STREAM_DESTROY).len(), 1);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
        assert_eq!(calls(&e, IS_KIND_OF), vec![vec![NODE_TYPE, 0x600]]);
    }

    #[test]
    fn load_geometry_hides_the_root_without_a_name_or_a_node() {
        let mut e = Engine::new();
        let (this, root) = stars(&mut e);
        load_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0063_fdb0, &args![this, 0u32]);
        assert_eq!(calls(&e, NODE_SET_FLAG_635FE0), vec![vec![root, 0]]);
        assert!(calls(&e, STRING_CONSTRUCT).is_empty());
        e.register(STREAM_OBJECT_COUNT, |_, _| ret(2));
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, 0x78);
        e.call_log = Some(vec![]);
        e.call(0x0063_fdb0, &args![this, name]);
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![LOAD_FAILED_MESSAGE]]);
        assert_eq!(calls(&e, NODE_SET_FLAG_635FE0), vec![vec![root, 0]]);
        assert!(calls(&e, MODEL_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, STREAM_DESTROY).len(), 1);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn fade_alpha_takes_the_fraction_of_each_fade() {
        // a = 18, b = 20, c = 6, d = 4 with k = 0.5: from_b 19, from_c 5.
        let alpha = |hour: f32| {
            f32::from_bits(fade_alpha(
                bits(hour),
                bits(20.0),
                bits(4.0),
                bits(19.0),
                bits(5.0),
            ))
        };
        assert_eq!(alpha(4.5), 0.5);
        assert_eq!(alpha(12.0), 0.0);
        assert_eq!(alpha(19.5), 0.5);
        assert_eq!(alpha(22.0), 1.0);
        assert_eq!(alpha(3.0), 1.0);
        assert_eq!(alpha(f32::NAN), 1.0);
    }

    struct Scene {
        this: u32,
        root: u32,
        property: u32,
    }

    fn update_scene(e: &mut Engine, hour: f32) -> Scene {
        e.map(0x011c_c000, 0x1000);
        e.map(0x0101_0000, 0x3_0000);
        e.set_global(FADE_FRACTION, 0.5f64);
        e.set_global(VISIBLE_ALPHA, 0.25f64);
        e.set_global(HOURS_PER_DAY, 24.0f64);
        e.set_global(ROTATION_FACTOR, 2.0f64);
        let (this, root) = stars(e);
        e.mem.set_u32(this + STARS_NODE, 0x500);
        quiet(
            e,
            &[BASE_UPDATE, NODE_SET_CULLED, POINT_NORMALIZE, NODE_ROTATE],
        );
        e.register(SKY_CHECK_44EDB0, |_, _| ret(1));
        e.register_double(SKY_HOUR, move |_, _| float(f64::from(hour)));
        e.register(POINT_CONSTRUCT, |e, a| {
            for word in 0..3u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret::default()
        });
        e.register(SKY_MODE, |_, _| ret(2));
        e.register(ROOT_SHOWABLE, |_, _| ret(1));
        e.register(SKY_HOUR_595FC0, |_, _| float(18.0));
        e.register(SKY_HOUR_63BA30, |_, _| float(20.0));
        e.register(SKY_HOUR_595F50, |_, _| float(6.0));
        e.register(SKY_HOUR_63B9B0, |_, _| float(4.0));
        let position = e.mem.alloc(0xc);
        for (word, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * word as u32, value);
        }
        e.register_double(SKY_POSITION, move |_, _| ret(position));
        e.register(NODE_CHILD_COUNT, |_, _| ret(1));
        let child_vtable = e.mem.alloc(0x20);
        e.mem
            .set_u32(child_vtable + CHILD_SLOT_GEOMETRY, GEOMETRY_DOUBLE);
        let child = e.mem.alloc(8);
        e.mem.set_u32(child, child_vtable);
        e.register_double(NODE_CHILD, move |_, _| ret(child));
        e.register(GEOMETRY_DOUBLE, |_, _| ret(0x800));
        let property = e.mem.alloc(0x70);
        e.register_double(GET_PROPERTY, move |_, _| ret(property));
        e.register(PROPERTY_KIND, |_, _| ret(0xd));
        e.register(POINTS_DIFFER, |_, _| ret(1));
        e.register(CALENDAR_DAYS_PASSED, |_, _| ret(2));
        let settings = e.mem.alloc(0x10);
        for (word, value) in [1.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(settings + 4 * word as u32, value);
        }
        e.register_double(SETTING_FLOAT, move |_, a| {
            ret(settings
                + match a[0] {
                    SETTING_PERIOD => 0,
                    SETTING_AXIS_X => 4,
                    SETTING_AXIS_Y => 8,
                    _ => 12,
                })
        });
        e.register(FLOAT_REMAINDER, |_, a| {
            float(f64::from(f32::from_bits(a[0]) % f32::from_bits(a[1])))
        });
        Scene {
            this,
            root,
            property,
        }
    }

    #[test]
    fn update_shows_the_stars_with_the_fade_alpha_and_rotates_them() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, 4.5);
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0x7f80_0001u32]);
        assert_eq!(
            calls(&e, BASE_UPDATE),
            vec![vec![scene.this, 0x900, 0x7fc0_0001]]
        );
        assert_eq!(calls(&e, POINT_CONSTRUCT)[0], vec![LAST_POSITION, 0, 0, 0]);
        assert_eq!(e.mem.f32(scene.this + ALPHA), 0.5);
        assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![scene.root, 0]]);
        assert_eq!(e.mem.f32(scene.property + 0x60), 1.0);
        assert_eq!(e.mem.f32(scene.property + 0x68), 3.0);
        assert_eq!(e.mem.f32(scene.property + 0x6c), 0.5);
        assert_eq!(e.global::<f32>(LAST_POSITION + 4), 2.0);
        // Day 2 + 4.5 / 24 = 2.1875; remainder of 1 = 0.1875; * 2 / 1.
        assert_eq!(
            calls(&e, NODE_ROTATE),
            vec![vec![0x500, bits(0.375), bits(0.0), bits(0.0), bits(1.0)]]
        );
        // The static is constructed once.
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0u32]);
        assert!(calls(&e, POINT_CONSTRUCT)
            .iter()
            .all(|call| call[0] != LAST_POSITION));
    }

    #[test]
    fn update_culls_the_root_when_hidden_or_faded_out() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, 12.0);
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0u32]);
        assert_eq!(e.mem.f32(scene.this + ALPHA), 0.0);
        assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![scene.root, 1]]);
        assert!(calls(&e, NODE_ROTATE).is_empty());
        e.register(ROOT_SHOWABLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0u32]);
        assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![scene.root, 1]]);
        assert!(calls(&e, SKY_HOUR_595FC0).is_empty());
        e.register(SKY_MODE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0u32]);
        assert_eq!(calls(&e, SKY_MODE).len(), 2);
        assert!(calls(&e, ROOT_SHOWABLE).is_empty());
        e.register(SKY_CHECK_44EDB0, |_, _| ret(0));
        e.mem.set_f32(scene.this + ALPHA, 0.75);
        e.call_log = Some(vec![]);
        e.call(0x0064_0020, &args![scene.this, 0x900u32, 0u32]);
        assert_eq!(e.mem.f32(scene.this + ALPHA), 0.75);
        assert!(calls(&e, SKY_HOUR).is_empty());
    }
}

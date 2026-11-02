//! `Havok SDK/run_00de59f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two constraint drawing functions (the engine map has no names): thiscall on the constraint
//! drawer (`hkpConstraintDrawer`, see `run_00de4470.rs` for its constructor), four stack words
//! `(constraint, display handler, first word, second word)`. Both start like
//! `hkpBallSocketDrawer::drawConstraint`: they hand the display handler to the primitive drawer
//! at `this + 0xc0`, let `updateCommonParameters` fill two 64-byte stack blocks and set the
//! drawer's points at `+0x50` and `+0x40`. The frame is addressed as the exe addresses it,
//! relative to the stack pointer after the three saved registers.
//!
//! The vector arithmetic is SSE (`f32`, lane by lane): the cross product of two vectors as
//! `rotate_up(b) * rotate_down(a) - rotate_down(b) * rotate_up(a)` and the normalisation
//! `0.5 * r * (3 - len2 * r * r)` with `r` the host's `RSQRTSS` estimate of the squared length,
//! zero when the squared length equals the float at `01011d78`. The x87 products are `f64`
//! results rounded to `float` at each store.
//!
//! The helpers and the two limit-shape steps are `pub(crate)`: `run_00de6550.rs` holds the
//! same drawer for another constraint data layout and its code (the cones, the plane and the
//! display of the shapes) is the same instructions.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `hkpPrimitiveDrawer::setDisplayHandler` (`00e10a10`, `this` = the primitive drawer at
/// `drawer + 0xc0`, one word).
pub(crate) const SET_DISPLAY_HANDLER: u32 = 0x00e1_0a10;
/// `hkpConstraintDrawer::updateCommonParameters` (`00de47e0`, `(constraint, first block, second
/// block)`), `drawPivots` (`00de44c0`) and `drawBodyFrames` (`00de4530`) (`(first word, second
/// word)`).
pub(crate) const UPDATE_COMMON_PARAMETERS: u32 = 0x00de_47e0;
pub(crate) const DRAW_PIVOTS: u32 = 0x00de_44c0;
pub(crate) const DRAW_BODY_FRAMES: u32 = 0x00de_4530;
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result, `(transform, point)`) and
/// `hkVector4::setRotatedDir` (`00c7f700`, `this` = the result, `(rotation, vector)`).
pub(crate) const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
pub(crate) const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkpPrimitiveDrawer::displayArrow` (`00e10d50`, `this` = the primitive drawer, `(transform,
/// from, to, colour, scale, first word, second word)`).
pub(crate) const DISPLAY_ARROW: u32 = 0x00e1_0d50;
/// `hkColor::rgbFromChars` (`00cabb00`, cdecl `(red, green, blue, alpha)`) and
/// `hkColor::rgbFromFloats` (`00cabb30`, cdecl, four floats as bit patterns).
pub(crate) const RGB_FROM_CHARS: u32 = 0x00ca_bb00;
pub(crate) const RGB_FROM_FLOATS: u32 = 0x00ca_bb30;
/// The cone's setter (`00de5370`, `this` = the cone, `(angle, scale, sides, direction,
/// point)`), the semicircle's setter (`00de4950`, `(radius, start angle, end angle, segments,
/// centre, axis, start direction)`) and the plane's update (`00de4ee0`, `this` = the plane,
/// four vector pointers).
pub(crate) const CONE_SET: u32 = 0x00de_5370;
pub(crate) const SEMICIRCLE_SET: u32 = 0x00de_4950;
pub(crate) const PLANE_SET: u32 = 0x00de_4ee0;
/// The memory router (`00c85750`): `+0x10` is the allocator; `hkArrayUtil::_reserve`
/// (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
pub(crate) const MEMORY_ROUTER: u32 = 0x00c8_5750;
pub(crate) const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The zero vector at `01267e30` the two points are transformed from.
pub(crate) const ORIGIN: u32 = 0x0126_7e30;
/// Constants: `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) of the
/// normaliser, the sign mask of four lanes (`010c44d0`), the doubles `pi/2` as a float widened
/// (`01030f38`), `0.75` (`0101de30`), `-0.75` (`01097800`) and `0.5` (`01011588`).
pub(crate) const ZERO: u32 = 0x0101_1d78;
pub(crate) const THREE: u32 = 0x0101_7718;
pub(crate) const HALF: u32 = 0x0101_6248;
pub(crate) const SIGN_MASK: u32 = 0x010c_44d0;
pub(crate) const HALF_PI: u32 = 0x0103_0f38;
pub(crate) const THREE_QUARTERS: u32 = 0x0101_de30;
pub(crate) const MINUS_THREE_QUARTERS: u32 = 0x0109_7800;
pub(crate) const HALF_DOUBLE: u32 = 0x0101_1588;

/// The capacity word of an empty array that owns no memory, and the mask of its capacity.
pub(crate) const NO_MEMORY: u32 = 0x8000_0000;
pub(crate) const CAPACITY_MASK: u32 = 0x3fff_ffff;

pub(crate) type Lanes = [f32; 4];

pub(crate) fn load(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at.wrapping_add(4 * lane as u32)))
}

pub(crate) fn store(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(at.wrapping_add(4 * lane as u32), *value);
    }
}

/// `MULPS`.
pub(crate) fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

pub(crate) fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

pub(crate) fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// One lane in all four (`SHUFPS v, v, 0`).
pub(crate) fn broadcast(value: f32) -> Lanes {
    [value; 4]
}

/// `SHUFPS v, v, 0xd2`: `(z, x, y, w)`.
fn rotate_up(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

/// `SHUFPS v, v, 0xc9`: `(y, z, x, w)`.
fn rotate_down(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

/// The cross product `first x second` as the exe forms it from its two loaded vectors:
/// `rotate_up(second) * rotate_down(first) - rotate_down(second) * rotate_up(first)`.
pub(crate) fn cross(first: Lanes, second: Lanes) -> Lanes {
    sub(
        mul(rotate_up(second), rotate_down(first)),
        mul(rotate_down(second), rotate_up(first)),
    )
}

/// The squared length `v2 + (v1 + v0)` of the squares of the three lowest lanes (two `ADDSS`
/// over shuffled copies).
pub(crate) fn squared_length(v: Lanes) -> f32 {
    let squares = mul(v, v);
    squares[2] + (squares[1] + squares[0])
}

/// The constants of the normaliser, read from memory once as the exe loads them.
pub(crate) struct Normaliser {
    zero: f32,
    three: f32,
    half: f32,
}

impl Normaliser {
    pub(crate) fn read(e: &Engine) -> Self {
        Normaliser {
            zero: e.global::<f32>(ZERO),
            three: e.global::<f32>(THREE),
            half: e.global::<f32>(HALF),
        }
    }

    /// The scale that makes a vector of squared length `squared` a unit vector: `(half * r) *
    /// (three - (squared * r) * r)` for the estimate `r`, or `0.0` when `squared == zero`
    /// (`CMPEQSS` then `ANDNPS`; a NaN is never equal).
    pub(crate) fn scale(&self, squared: f32) -> f32 {
        let r = rsqrt_estimate(squared);
        let value = (self.half * r) * (self.three - (squared * r) * r);
        if squared == self.zero {
            0.0
        } else {
            value
        }
    }
}

/// `FLD` of the float at `first`, `FMUL` by the float at `second`: the x87 product.
pub(crate) fn float_product(e: &Engine, first: u32, second: u32) -> f64 {
    x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)))
}

/// A float argument pushed after `FSTP`/`FLD`/`FSTP`: the product rounded to a float and
/// carried through the x87 stack again (a NaN is already quiet), as bits.
pub(crate) fn pushed_float(value: f64) -> u32 {
    x87_quiet_bits(x87_store32(value))
}

/// `FLD dword [at]; FSTP dword`: the bits of the float with a signalling NaN made quiet.
pub(crate) fn copied_float(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

/// `pi/2 - |x|` of the float with the given bits as the exe computes it: `FLD`, `FABS`, `FSTP
/// dword`, `FLD`, `FSUBR double [01030f38]`, `FSTP dword`, `FLD`, `FSTP dword`.
pub(crate) fn limit_angle(e: &Engine, bits: u32) -> u32 {
    let magnitude = x87_load32(bits).abs();
    let stored = x87_store32(magnitude);
    let difference = x87_sub(e.mem.f64(HALF_PI), x87_load32(stored));
    pushed_float(difference)
}

/// The array of pointers `hkArray` the drawers keep on the stack: `{data, size, capacity and
/// flags}` at `array`, empty, owning no memory.
pub(crate) fn new_stack_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), NO_MEMORY);
}

/// The step of the exe that makes room for `wanted` elements of 4 bytes in the local array: if
/// the capacity is below `wanted`, `00c908c0` grows it to `max(2 * capacity, wanted)` (the
/// router's allocator).
pub(crate) fn reserve_pointers(e: &mut Engine, array: u32, wanted: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = (e.mem.u32(array.wrapping_add(8)) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let capacity = if doubled > wanted { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, capacity, 4u32]);
    }
}

/// The step of the exe that empties a local array: through the router's allocator (slot
/// `0x10`, `(data, bytes)`) when its capacity word has bit 31 clear. Sets the size to 0 first.
pub(crate) fn release_pointers(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = e.mem.u32(array.wrapping_add(8));
    e.mem.set_u32(array.wrapping_add(4), 0);
    if capacity & NO_MEMORY == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
}

/// The two arrows of the cone drawers, scaled by `[drawer + 0x1c] * [drawer + 0xc]`: from the
/// point `drawer + 0x50` between the frame's `+0x80` and `+0x70` vectors in `0xff008000`, and
/// from `drawer + 0x40` between `+0xc0` and `+0x90` in the colour of `rgbFromChars(0xff, 0xff,
/// 0, 0xff)`. `frame` is the frame base.
pub(crate) fn draw_arrows(e: &mut Engine, me: u32, frame: u32, first_word: u32, second_word: u32) {
    let f = frame;
    let arrow_scale = pushed_float(float_product(e, me + 0x1c, me + 0xc));
    e.call(
        DISPLAY_ARROW,
        &args![
            me + 0xc0,
            me + 0x50,
            f + 0x80,
            f + 0x70,
            0xff00_8000u32,
            arrow_scale,
            first_word,
            second_word
        ],
    );
    let arrow_scale = pushed_float(float_product(e, me + 0x1c, me + 0xc));
    let yellow = e
        .call(RGB_FROM_CHARS, &args![0xffu32, 0xffu32, 0u32, 0xffu32])
        .u32();
    e.call(
        DISPLAY_ARROW,
        &args![
            me + 0xc0,
            me + 0x40,
            f + 0xc0,
            f + 0x90,
            yellow,
            arrow_scale,
            first_word,
            second_word
        ],
    );
}

/// The three cones and the plane of the cone drawers: `00de5370` on `drawer + 0xd0` with the
/// angle `first_angle` (float bits), the direction at frame `+0x80` and the point
/// `drawer + 0x50`; `00de4ee0` on `drawer + 0x280` with the scale `[drawer + 0x14] * [drawer +
/// 0x1c]` in all four lanes (frame `+0xa0`); `00de5370` on `drawer + 0x160` with the angle
/// `pi/2 - |second_limit|` and direction frame `+0x60`; and on `drawer + 0x1f0` with the angle
/// `pi/2 - |saved_limit|` and the direction frame `+0x60` with all signs flipped (in place).
/// The scale of the cones is `[drawer + 0x1c] * [drawer + 0x10]`, their sides `[drawer +
/// 0x18]`. `second_limit` is the raw float; `saved_limit` the bits of a float that already
/// went through the x87 stack.
pub(crate) fn update_limit_shapes(
    e: &mut Engine,
    me: u32,
    frame: u32,
    first_angle: u32,
    second_limit: u32,
    saved_limit: u32,
) {
    let f = frame;
    let cone_scale = pushed_float(float_product(e, me + 0x1c, me + 0x10));
    let sides = e.mem.u32(me + 0x18);
    e.call(
        CONE_SET,
        &args![
            me + 0xd0,
            first_angle,
            cone_scale,
            sides,
            f + 0x80,
            me + 0x50
        ],
    );
    let plane_scale = pushed_float(x87_mul(
        x87_load32(e.mem.u32(me + 0x14)),
        x87_m32(e.mem.u32(me + 0x1c)),
    ));
    store(e, f + 0xa0, broadcast(f32::from_bits(plane_scale)));
    e.call(
        PLANE_SET,
        &args![me + 0x280, f + 0x60, f + 0x70, me + 0x50, f + 0xa0],
    );
    let cone_scale = pushed_float(float_product(e, me + 0x1c, me + 0x10));
    let sides = e.mem.u32(me + 0x18);
    let second_angle = limit_angle(e, second_limit);
    e.call(
        CONE_SET,
        &args![
            me + 0x160,
            second_angle,
            cone_scale,
            sides,
            f + 0x60,
            me + 0x50
        ],
    );
    let cone_scale = pushed_float(float_product(e, me + 0x1c, me + 0x10));
    let sides = e.mem.u32(me + 0x18);
    let direction = load(e, f + 0x60);
    let mask = load(e, SIGN_MASK);
    let flipped: Lanes = std::array::from_fn(|lane| {
        f32::from_bits(direction[lane].to_bits() ^ mask[lane].to_bits())
    });
    let third_angle = limit_angle(e, saved_limit);
    store(e, f + 0x60, flipped);
    e.call(
        CONE_SET,
        &args![
            me + 0x1f0,
            third_angle,
            cone_scale,
            sides,
            f + 0x60,
            me + 0x50
        ],
    );
}

/// The display of the cone drawers' shapes through three local arrays of pointers: the
/// first cone (`drawer + 0xd0`) in `0xffffff00`, the other two (`+0x160`, `+0x1f0`) in
/// `0xffff0000` and the plane (`+0x280`) in `rgbFromChars(0xff, 0, 0xff, 0xff)`, each as one
/// call of the display handler's slot `0x38` `(array, colour, first word, second word)`. The
/// arrays are reserved through the router's allocator (capacities 1, 1, 2 elements) and freed
/// with it unless they own no memory. `frame` is the frame base (arrays at `+0x40`, `+0x20`,
/// `+0x30`).
pub(crate) fn show_limit_shapes(
    e: &mut Engine,
    me: u32,
    frame: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let f = frame;
    new_stack_array(e, f + 0x40);
    new_stack_array(e, f + 0x20);
    new_stack_array(e, f + 0x30);
    reserve_pointers(e, f + 0x40, 1);
    e.mem.set_u32(f + 0x44, 1);
    reserve_pointers(e, f + 0x30, 1);
    e.mem.set_u32(f + 0x34, 1);
    reserve_pointers(e, f + 0x20, 2);
    let first_data = e.mem.u32(f + 0x40);
    e.mem.set_u32(f + 0x24, 2);
    e.mem.set_u32(first_data, me + 0xd0);
    let second_data = e.mem.u32(f + 0x20);
    e.mem.set_u32(second_data, me + 0x160);
    let second_data = e.mem.u32(f + 0x20);
    e.mem.set_u32(second_data.wrapping_add(4), me + 0x1f0);
    let plane_data = e.mem.u32(f + 0x30);
    e.mem.set_u32(plane_data, me + 0x280);
    e.vcall(
        display_handler,
        0x38,
        &args![f + 0x40, 0xffff_ff00u32, first_word, second_word],
    );
    e.vcall(
        display_handler,
        0x38,
        &args![f + 0x20, 0xffff_0000u32, first_word, second_word],
    );
    let vtable = e.mem.u32(display_handler);
    let magenta = e
        .call(RGB_FROM_CHARS, &args![0xffu32, 0u32, 0xffu32, 0xffu32])
        .u32();
    let slot = e.mem.u32(vtable.wrapping_add(0x38));
    e.call(
        slot,
        &args![display_handler, f + 0x30, magenta, first_word, second_word],
    );
    release_pointers(e, f + 0x30);
    e.mem.set_u32(f + 0x30, 0);
    e.mem.set_u32(f + 0x38, NO_MEMORY);
    release_pointers(e, f + 0x20);
    e.mem.set_u32(f + 0x20, 0);
    e.mem.set_u32(f + 0x28, NO_MEMORY);
    release_pointers(e, f + 0x40);
}

// Translated from 00de59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a cone and a plane limit (the engine map has no name), thiscall with
/// four words. After the common set-up (`setDisplayHandler`, `updateCommonParameters` into the
/// blocks at frame `+0xd0` / `+0x110`, the drawer's points at `+0x50` / `+0x40`, `drawPivots`)
/// it rotates three directions of the constraint data (the constraint's `+0xc` pointer) by the
/// drawer's frames (`setRotatedDir`): the vector `data + 0x50` into frame `+0x80`; the unit
/// cross product of `data + 0x50` and `data + 0x60` into `+0x70`, rotated in place, and
/// `data + 0x60` into `+0x60`; the unit cross product of `data + 0x30` and `data + 0x20`
/// (rotated by the second frame, `drawer + 0x60`) into `+0x90` and `data + 0x20` into `+0xc0`.
/// Then the two arrows ([`draw_arrows`]), the cones and the plane ([`update_limit_shapes`]: the
/// angles from the constraint data's `+0xa0`, `+0xb4` and `+0xb0`) and their display
/// ([`show_limit_shapes`]).
pub fn fn_00de59f0(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let me = this.addr();
    let data = e.mem.u32(constraint.wrapping_add(0xc));
    e.call(
        SET_DISPLAY_HANDLER,
        &args![me.wrapping_add(0xc0), display_handler],
    );
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0xd0, f + 0x110],
        );
        e.call(SET_TRANSFORMED_POS, &args![me + 0x50, f + 0xd0, ORIGIN]);
        e.call(SET_TRANSFORMED_POS, &args![me + 0x40, f + 0x110, ORIGIN]);
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x80, me + 0x90, data.wrapping_add(0x50)],
        );
        // The unit normal of data +0x50 and +0x60, rotated by the first frame.
        let normaliser = Normaliser::read(e);
        let first = load(e, data.wrapping_add(0x50));
        let second = load(e, data.wrapping_add(0x60));
        let normal = cross(first, second);
        let scale = normaliser.scale(squared_length(normal));
        store(e, f + 0x70, mul(broadcast(scale), normal));
        e.call(SET_ROTATED_DIR, &args![f + 0x70, me + 0x90, f + 0x70]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x60, me + 0x90, data.wrapping_add(0x60)],
        );
        // The unit normal of data +0x30 and +0x20, rotated by the second frame.
        let first = load(e, data.wrapping_add(0x30));
        let second = load(e, data.wrapping_add(0x20));
        let normal = cross(first, second);
        let scale = normaliser.scale(squared_length(normal));
        store(e, f + 0xb0, mul(broadcast(scale), normal));
        e.call(SET_ROTATED_DIR, &args![f + 0x90, me + 0x60, f + 0xb0]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0xc0, me + 0x60, data.wrapping_add(0x20)],
        );
        draw_arrows(e, me, f, first_word, second_word);
        let first_angle = copied_float(e, data.wrapping_add(0xa0));
        let saved_limit = copied_float(e, data.wrapping_add(0xb0));
        let second_limit = e.mem.u32(data.wrapping_add(0xb4));
        update_limit_shapes(e, me, f, first_angle, second_limit, saved_limit);
        show_limit_shapes(e, me, f, display_handler, first_word, second_word);
    });
}

// Translated from 00de5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws a constraint with a fan and a half circle limit (the engine map has no name),
/// thiscall with four words. After the common set-up as `00de59f0` (blocks at frame `+0x70` /
/// `+0xb0`) it draws the body frames and the pivots, rotates `data + 0x50` into frame `+0x50`
/// and `data + 0x70` into `+0x60` by the first frame (`drawer + 0x90`), and has the display
/// handler's slot `0x24` draw the line `drawer + 0x40` to `drawer + 0x50` in `0xffff0000`.
/// With `s = [drawer + 0x1c] * [drawer]` it draws the line between the points
/// `drawer + 0x50 + 0.75 * s * (frame +0x50)` and `drawer + 0x50 - 0.75 * s * (frame +0x50)`
/// (in the colour of `rgbFromFloats(0, 0.5, 1, 1)`, each product rounded to a float first),
/// sets the half circle at `drawer + 0xd0` (`00de4950`: radius `[+4] * [+0x1c]`, the angles
/// from the data's `+0x84` and `+0x88`, the sides `[+0x18]`, centre `drawer + 0x50`, axis
/// and start direction the two rotated frames) and draws it through slot `0x38` in white
/// (`0xffffffff`; one local array of one pointer). Then it draws the line from the point
/// along `0.75 * s * (frame +0x60)` in `0xffffff00`, rotates `data + 0x40` by the second frame
/// (`drawer + 0x60`) into `+0x40`, normalises it (the host's `RSQRTSS`) and draws the line from
/// `drawer + 0x40` along `0.5 * s * normalised` in `0xffffff00`.
pub fn fn_00de5fe0(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
    display_handler: u32,
    first_word: u32,
    second_word: u32,
) {
    let me = this.addr();
    let data = e.mem.u32(constraint.wrapping_add(0xc));
    e.call(
        SET_DISPLAY_HANDLER,
        &args![me.wrapping_add(0xc0), display_handler],
    );
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        e.call(
            UPDATE_COMMON_PARAMETERS,
            &args![me, constraint, f + 0x70, f + 0xb0],
        );
        e.call(SET_TRANSFORMED_POS, &args![me + 0x50, f + 0x70, ORIGIN]);
        e.call(SET_TRANSFORMED_POS, &args![me + 0x40, f + 0xb0, ORIGIN]);
        e.call(DRAW_BODY_FRAMES, &args![me, first_word, second_word]);
        e.call(DRAW_PIVOTS, &args![me, first_word, second_word]);
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x50, me + 0x90, data.wrapping_add(0x50)],
        );
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x60, me + 0x90, data.wrapping_add(0x70)],
        );
        e.vcall(
            display_handler,
            0x24,
            &args![
                me + 0x40,
                me + 0x50,
                0xffff_0000u32,
                first_word,
                second_word
            ],
        );
        // The line through the point along the first rotated direction.
        let product = float_product(e, me + 0x1c, me);
        let product_float = x87_load32(x87_store32(product));
        let along = x87_store32(x87_mul(e.mem.f64(THREE_QUARTERS), product_float));
        let against = x87_mul(product_float, e.mem.f64(MINUS_THREE_QUARTERS));
        let direction = load(e, f + 0x50);
        let point = load(e, me + 0x50);
        let forward = add(mul(broadcast(f32::from_bits(along)), direction), point);
        store(e, f + 0x40, forward);
        let against = x87_store32(against);
        let backward = add(mul(broadcast(f32::from_bits(against)), direction), point);
        let vtable = e.mem.u32(display_handler);
        let half = copied_float(e, HALF);
        store(e, f + 0x30, backward);
        let blue = e
            .call(
                RGB_FROM_FLOATS,
                &args![0u32, half, 0x3f80_0000u32, 0x3f80_0000u32],
            )
            .u32();
        let slot = e.mem.u32(vtable.wrapping_add(0x24));
        e.call(
            slot,
            &args![
                display_handler,
                f + 0x30,
                f + 0x40,
                blue,
                first_word,
                second_word
            ],
        );
        // The half circle.
        let sides = e.mem.u32(me + 0x18);
        let end_angle = copied_float(e, data.wrapping_add(0x88));
        let start_angle = copied_float(e, data.wrapping_add(0x84));
        let radius = pushed_float(float_product(e, me + 4, me + 0x1c));
        e.call(
            SEMICIRCLE_SET,
            &args![
                me + 0xd0,
                radius,
                start_angle,
                end_angle,
                sides,
                me + 0x50,
                f + 0x50,
                f + 0x60
            ],
        );
        new_stack_array(e, f + 0x20);
        reserve_pointers(e, f + 0x20, 1);
        let array_data = e.mem.u32(f + 0x20);
        e.mem.set_u32(f + 0x24, 1);
        e.mem.set_u32(array_data, me + 0xd0);
        e.vcall(
            display_handler,
            0x38,
            &args![f + 0x20, 0xffff_ffffu32, first_word, second_word],
        );
        release_pointers(e, f + 0x20);
        // The line through the point along the second rotated direction.
        let product = float_product(e, me + 0x1c, me);
        let vtable = e.mem.u32(display_handler);
        let point = load(e, me + 0x50);
        let product_float = x87_load32(x87_store32(product));
        store(e, f + 0x20, point);
        let along = x87_store32(x87_mul(product_float, e.mem.f64(THREE_QUARTERS)));
        store(e, f + 0x30, point);
        let direction = load(e, f + 0x60);
        let end = add(mul(broadcast(f32::from_bits(along)), direction), point);
        store(e, f + 0x20, end);
        let slot = e.mem.u32(vtable.wrapping_add(0x24));
        e.call(
            slot,
            &args![
                display_handler,
                f + 0x30,
                f + 0x20,
                0xffff_ff00u32,
                first_word,
                second_word
            ],
        );
        // The line along the normalised rotated `data + 0x40`.
        e.call(
            SET_ROTATED_DIR,
            &args![f + 0x40, me + 0x60, data.wrapping_add(0x40)],
        );
        let normaliser = Normaliser::read(e);
        let rotated = load(e, f + 0x40);
        let product = float_product(e, me + 0x1c, me);
        let product_float = x87_load32(x87_store32(product));
        let half_length = x87_store32(x87_mul(product_float, e.mem.f64(HALF_DOUBLE)));
        let scale = normaliser.scale(squared_length(rotated));
        let point = load(e, me + 0x40);
        let unit = mul(broadcast(scale), rotated);
        store(e, f + 0x40, unit);
        store(e, f + 0x30, point);
        store(e, f + 0x20, point);
        let end = add(mul(broadcast(f32::from_bits(half_length)), unit), point);
        let vtable = e.mem.u32(display_handler);
        let slot = e.mem.u32(vtable.wrapping_add(0x24));
        store(e, f + 0x20, end);
        e.call(
            slot,
            &args![
                display_handler,
                f + 0x30,
                f + 0x20,
                0xffff_ff00u32,
                first_word,
                second_word
            ],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_59f0, fn_00de59f0(Ptr, u32, u32, u32, u32)),
        entry!(0x00de_5fe0, fn_00de5fe0(Ptr, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One call of a callee: its address, its argument words and the 16-byte blocks the
    /// listed words point to, read when the call happens.
    type Call = (u32, Vec<u32>, Vec<[u32; 4]>);
    type Log = Rc<RefCell<Vec<Call>>>;

    const LINE_SLOT: u32 = 0x0900_0024;
    const GEOMETRY_SLOT: u32 = 0x0900_0038;
    const FREE_SLOT: u32 = 0x0900_0010;
    const ROUTER_CELL: u32 = 0x0126_8100;

    fn block(e: &Engine, at: u32) -> [u32; 4] {
        std::array::from_fn(|lane| e.mem.u32(at + 4 * lane as u32))
    }

    fn bits(v: [f32; 4]) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    fn set_block(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
    }

    /// A double that logs `(address, words, blocks at the listed word indexes)` and answers
    /// `eax`.
    fn watch(e: &mut Engine, log: &Log, address: u32, blocks: &'static [usize], eax: u32) {
        let log = log.clone();
        e.register_double(address, move |e, words| {
            let seen = blocks.iter().map(|i| block(e, words[*i])).collect();
            log.borrow_mut().push((address, words.to_vec(), seen));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    struct Rig {
        e: Engine,
        log: Log,
        drawer: u32,
        constraint: u32,
        data: u32,
        handler: u32,
        allocator: u32,
    }

    /// A drawer with the scalars `[+0x1c] = 2`, `[+0] = 3`, `[+0xc] = 5`, `[+0x10] = 7`,
    /// `[+0x14] = 11`, `[+4] = 13`, 24 sides and the two frames' points; a constraint whose
    /// data holds the vectors `x`, `y`, `z`; the constants of the exe; doubles for every
    /// callee outside this file; a display handler whose slots `0x24` and `0x38` log; a
    /// router whose allocator logs its frees.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_u32(SIGN_MASK, 0x8000_0000);
        e.mem.set_u32(SIGN_MASK + 4, 0x8000_0000);
        e.mem.set_u32(SIGN_MASK + 8, 0x8000_0000);
        e.mem.set_u32(SIGN_MASK + 12, 0x8000_0000);
        e.mem
            .set_f64(HALF_PI, f64::from_bits(0x3ff9_21fb_6000_0000));
        e.mem.set_f64(THREE_QUARTERS, 0.75);
        e.mem.set_f64(MINUS_THREE_QUARTERS, -0.75);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        let log: Log = Rc::default();
        let drawer = e.mem.alloc(0x300);
        for (offset, value) in [
            (0x1c, 2.0f32),
            (0, 3.0),
            (0xc, 5.0),
            (0x10, 7.0),
            (0x14, 11.0),
            (4, 13.0),
        ] {
            e.mem.set_f32(drawer + offset, value);
        }
        e.mem.set_u32(drawer + 0x18, 24);
        set_block(&mut e, drawer + 0x40, [10.0, 20.0, 30.0, 0.0]);
        set_block(&mut e, drawer + 0x50, [1.0, 2.0, 3.0, 0.0]);
        let data = e.mem.alloc(0x100);
        set_block(&mut e, data + 0x20, [0.0, 0.0, 1.0, 0.0]);
        set_block(&mut e, data + 0x30, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut e, data + 0x40, [0.0, 3.0, 4.0, 0.0]);
        set_block(&mut e, data + 0x50, [1.0, 0.0, 0.0, 0.0]);
        set_block(&mut e, data + 0x60, [0.0, 1.0, 0.0, 0.0]);
        set_block(&mut e, data + 0x70, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(data + 0x84, 0.25);
        e.mem.set_f32(data + 0x88, 0.75);
        e.mem.set_f32(data + 0xa0, 0.7);
        e.mem.set_f32(data + 0xb0, 0.4);
        e.mem.set_f32(data + 0xb4, -0.3);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, data);
        for address in [
            SET_DISPLAY_HANDLER,
            UPDATE_COMMON_PARAMETERS,
            SET_TRANSFORMED_POS,
            DRAW_PIVOTS,
            DRAW_BODY_FRAMES,
        ] {
            watch(&mut e, &log, address, &[], 0);
        }
        watch(&mut e, &log, SET_ROTATED_DIR, &[2], 0);
        watch(&mut e, &log, DISPLAY_ARROW, &[2, 3], 0);
        watch(&mut e, &log, CONE_SET, &[4, 5], 0);
        watch(&mut e, &log, SEMICIRCLE_SET, &[5, 6, 7], 0);
        watch(&mut e, &log, PLANE_SET, &[1, 2, 3, 4], 0);
        watch(&mut e, &log, RGB_FROM_CHARS, &[], 0x0c01_0000);
        watch(&mut e, &log, RGB_FROM_FLOATS, &[], 0x0f10_0000);
        // the router and its allocator
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, allocator_vtable);
        e.mem.set_u32(allocator_vtable + 0x10, FREE_SLOT);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(ROUTER_CELL),
            ..Ret::default()
        });
        watch(&mut e, &log, FREE_SLOT, &[], 0);
        // reserve: gives the array a block of the requested capacity
        {
            let log = log.clone();
            e.register_double(ARRAY_RESERVE, move |e, words| {
                log.borrow_mut()
                    .push((ARRAY_RESERVE, words.to_vec(), Vec::new()));
                let block = e.mem.alloc(4 * words[2]);
                e.mem.set_u32(words[1], block);
                e.mem.set_u32(words[1] + 8, words[2]);
                Ret::default()
            });
        }
        // the display handler
        let handler = e.mem.alloc(8);
        let handler_vtable = e.mem.alloc(0x60);
        e.mem.set_u32(handler, handler_vtable);
        e.mem.set_u32(handler_vtable + 0x24, LINE_SLOT);
        e.mem.set_u32(handler_vtable + 0x38, GEOMETRY_SLOT);
        watch(&mut e, &log, LINE_SLOT, &[1, 2], 0);
        {
            let log = log.clone();
            e.register_double(GEOMETRY_SLOT, move |e, words| {
                let array = words[1];
                let data = e.mem.u32(array);
                let size = e.mem.u32(array + 4);
                let mut seen = [0u32; 4];
                seen[0] = size;
                for i in 0..size.min(2) {
                    seen[1 + i as usize] = e.mem.u32(data + 4 * i);
                }
                log.borrow_mut()
                    .push((GEOMETRY_SLOT, words.to_vec(), vec![seen]));
                Ret::default()
            });
        }
        Rig {
            e,
            log,
            drawer,
            constraint,
            data,
            handler,
            allocator,
        }
    }

    fn calls(r: &Rig, address: u32) -> Vec<Call> {
        r.log
            .borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .cloned()
            .collect()
    }

    fn scale_of(squared: f32) -> f32 {
        let r = rsqrt_estimate(squared);
        (0.5 * r) * (3.0 - (squared * r) * r)
    }

    // ----- 00de59f0 -----

    fn run_cone_plane(r: &mut Rig) {
        r.e.call(
            0x00de_59f0,
            &args![r.drawer, r.constraint, r.handler, 0x111u32, 0x222u32],
        );
    }

    #[test]
    fn the_cone_drawer_sets_up_and_rotates_the_directions() {
        let mut r = rig();
        run_cone_plane(&mut r);
        let (d, c) = (r.drawer, r.constraint);
        let log = r.log.borrow();
        let first_five: Vec<(u32, Vec<u32>)> = log
            .iter()
            .take(5)
            .map(|(a, w, _)| (*a, w.clone()))
            .collect();
        let frame = log[1].1[2] - 0xd0;
        assert_eq!(
            first_five,
            vec![
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0xd0, frame + 0x110]
                ),
                (SET_TRANSFORMED_POS, vec![d + 0x50, frame + 0xd0, ORIGIN]),
                (SET_TRANSFORMED_POS, vec![d + 0x40, frame + 0x110, ORIGIN]),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
            ]
        );
        drop(log);
        let rotations = calls(&r, SET_ROTATED_DIR);
        assert_eq!(rotations.len(), 5);
        let words: Vec<Vec<u32>> = rotations.iter().map(|c| c.1.clone()).collect();
        let data = r.data;
        assert_eq!(words[0], vec![frame + 0x80, d + 0x90, data + 0x50]);
        assert_eq!(words[1], vec![frame + 0x70, d + 0x90, frame + 0x70]);
        assert_eq!(words[2], vec![frame + 0x60, d + 0x90, data + 0x60]);
        assert_eq!(words[3], vec![frame + 0x90, d + 0x60, frame + 0xb0]);
        assert_eq!(words[4], vec![frame + 0xc0, d + 0x60, data + 0x20]);
        // the normal of x and y is z scaled to a unit vector; the normal of z-less data +0x30
        // (x) and +0x20 (z) is -y
        let unit = scale_of(1.0);
        assert_eq!(rotations[1].2[0], bits([0.0, 0.0, unit, 0.0]));
        assert_eq!(rotations[3].2[0], bits([0.0, -unit, 0.0, 0.0]));
    }

    #[test]
    fn a_zero_normal_stays_zero() {
        let mut r = rig();
        // data +0x50 parallel to +0x60: the cross product is zero, the scale is 0.0
        let data = r.data;
        set_block(&mut r.e, data + 0x60, [2.0, 0.0, 0.0, 0.0]);
        run_cone_plane(&mut r);
        let rotations = calls(&r, SET_ROTATED_DIR);
        assert_eq!(rotations[1].2[0], bits([0.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn the_cone_drawer_draws_two_arrows_three_cones_and_the_plane() {
        let mut r = rig();
        run_cone_plane(&mut r);
        let d = r.drawer;
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0xd0;
        let arrows = calls(&r, DISPLAY_ARROW);
        assert_eq!(arrows.len(), 2);
        let arrow_scale = (2.0f32 * 5.0).to_bits();
        assert_eq!(
            arrows[0].1,
            vec![
                d + 0xc0,
                d + 0x50,
                frame + 0x80,
                frame + 0x70,
                0xff00_8000,
                arrow_scale,
                0x111,
                0x222
            ]
        );
        assert_eq!(
            arrows[1].1,
            vec![
                d + 0xc0,
                d + 0x40,
                frame + 0xc0,
                frame + 0x90,
                0x0c01_0000,
                arrow_scale,
                0x111,
                0x222
            ]
        );
        let colours: Vec<Vec<u32>> = calls(&r, RGB_FROM_CHARS)
            .iter()
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            colours,
            vec![vec![0xff, 0xff, 0, 0xff], vec![0xff, 0, 0xff, 0xff]]
        );
        let cone_scale = (2.0f32 * 7.0).to_bits();
        let cones = calls(&r, CONE_SET);
        assert_eq!(cones.len(), 3);
        let half_pi = f64::from_bits(0x3ff9_21fb_6000_0000);
        let second_angle = (half_pi - f64::from(0.3f32)) as f32;
        let third_angle = (half_pi - f64::from(0.4f32)) as f32;
        assert_eq!(
            cones[0].1,
            vec![
                d + 0xd0,
                0.7f32.to_bits(),
                cone_scale,
                24,
                frame + 0x80,
                d + 0x50
            ]
        );
        assert_eq!(
            cones[1].1,
            vec![
                d + 0x160,
                second_angle.to_bits(),
                cone_scale,
                24,
                frame + 0x60,
                d + 0x50
            ]
        );
        assert_eq!(
            cones[2].1,
            vec![
                d + 0x1f0,
                third_angle.to_bits(),
                cone_scale,
                24,
                frame + 0x60,
                d + 0x50
            ]
        );
        // the third cone's direction is the second one with all signs flipped
        let second_direction = cones[1].2[0];
        let flipped: [u32; 4] = second_direction.map(|w| w ^ 0x8000_0000);
        assert_eq!(cones[2].2[0], flipped);
        let plane = calls(&r, PLANE_SET);
        assert_eq!(plane.len(), 1);
        assert_eq!(
            plane[0].1,
            vec![
                d + 0x280,
                frame + 0x60,
                frame + 0x70,
                d + 0x50,
                frame + 0xa0
            ]
        );
        assert_eq!(plane[0].2[3], bits([22.0; 4]));
    }

    #[test]
    fn the_cone_drawer_shows_the_shapes_and_frees_the_arrays() {
        let mut r = rig();
        run_cone_plane(&mut r);
        let d = r.drawer;
        let shows = calls(&r, GEOMETRY_SLOT);
        assert_eq!(shows.len(), 3);
        // first array: the first cone; second: the other two cones; third: the plane
        assert_eq!(shows[0].1[2..], [0xffff_ff00, 0x111, 0x222]);
        assert_eq!(shows[0].2[0], [1, d + 0xd0, 0, 0]);
        assert_eq!(shows[1].1[2..], [0xffff_0000, 0x111, 0x222]);
        assert_eq!(shows[1].2[0], [2, d + 0x160, d + 0x1f0, 0]);
        assert_eq!(shows[2].1[2..], [0x0c01_0000, 0x111, 0x222]);
        assert_eq!(shows[2].2[0], [1, d + 0x280, 0, 0]);
        // each array asked for room: capacities 1, 1 and 2 elements of 4 bytes
        let reserves: Vec<Vec<u32>> = calls(&r, ARRAY_RESERVE)
            .iter()
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(reserves.len(), 3);
        assert_eq!(reserves[0][0], r.allocator);
        assert_eq!(
            reserves.iter().map(|w| (w[2], w[3])).collect::<Vec<_>>(),
            vec![(1, 4), (1, 4), (2, 4)]
        );
        // the arrays owned memory (the double cleared the flag bit), so all three are freed:
        // the plane's, the cones' and the first cone's
        let frees: Vec<Vec<u32>> = calls(&r, FREE_SLOT).iter().map(|c| c.1.clone()).collect();
        assert_eq!(frees.len(), 3);
        assert_eq!(
            frees.iter().map(|w| w[2]).collect::<Vec<_>>(),
            vec![4, 8, 4]
        );
    }

    #[test]
    fn arrays_that_own_no_memory_are_not_freed() {
        let mut r = rig();
        // a reserve that leaves the arrays as they are keeps the "no memory" flag
        r.e.register(ARRAY_RESERVE, |e, words| {
            let block = e.mem.alloc(4 * words[2]);
            e.mem.set_u32(words[1], block);
            Ret::default()
        });
        run_cone_plane(&mut r);
        assert!(calls(&r, FREE_SLOT).is_empty());
    }

    // ----- 00de5fe0 -----

    fn run_fan(r: &mut Rig) {
        r.e.call(
            0x00de_5fe0,
            &args![r.drawer, r.constraint, r.handler, 0x111u32, 0x222u32],
        );
    }

    #[test]
    fn the_fan_drawer_sets_up_draws_the_frames_and_the_first_line() {
        let mut r = rig();
        run_fan(&mut r);
        let (d, c) = (r.drawer, r.constraint);
        let log = r.log.borrow();
        let frame = log[1].1[2] - 0x70;
        let first: Vec<(u32, Vec<u32>)> = log
            .iter()
            .take(8)
            .map(|(a, w, _)| (*a, w.clone()))
            .collect();
        assert_eq!(
            first,
            vec![
                (SET_DISPLAY_HANDLER, vec![d + 0xc0, r.handler]),
                (
                    UPDATE_COMMON_PARAMETERS,
                    vec![d, c, frame + 0x70, frame + 0xb0]
                ),
                (SET_TRANSFORMED_POS, vec![d + 0x50, frame + 0x70, ORIGIN]),
                (SET_TRANSFORMED_POS, vec![d + 0x40, frame + 0xb0, ORIGIN]),
                (DRAW_BODY_FRAMES, vec![d, 0x111, 0x222]),
                (DRAW_PIVOTS, vec![d, 0x111, 0x222]),
                (SET_ROTATED_DIR, vec![frame + 0x50, d + 0x90, r.data + 0x50]),
                (SET_ROTATED_DIR, vec![frame + 0x60, d + 0x90, r.data + 0x70]),
            ]
        );
        assert_eq!(log[8].0, LINE_SLOT);
        assert_eq!(
            log[8].1,
            vec![r.handler, d + 0x40, d + 0x50, 0xffff_0000, 0x111, 0x222]
        );
    }

    #[test]
    fn the_fan_drawer_draws_the_axis_line_the_half_circle_and_the_two_limit_lines() {
        let mut r = rig();
        run_fan(&mut r);
        let d = r.drawer;
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0x70;
        let lines = calls(&r, LINE_SLOT);
        assert_eq!(lines.len(), 4);
        // s = [+0x1c] * [+0] = 6; 0.75 * s = 4.5 (the vector held in rotation calls is zero
        // here because the rotation doubles write nothing)
        let blue = 0x0f10_0000;
        assert_eq!(
            lines[1].1,
            vec![r.handler, frame + 0x30, frame + 0x40, blue, 0x111, 0x222]
        );
        // the doubles write no rotated vectors, so both ends are the drawer's point
        let point = bits([1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lines[1].2, vec![point, point]);
        assert_eq!(
            lines[2].1[1..],
            [frame + 0x30, frame + 0x20, 0xffff_ff00, 0x111, 0x222]
        );
        assert_eq!(lines[2].2, vec![point, point]);
        let circle = calls(&r, SEMICIRCLE_SET);
        assert_eq!(circle.len(), 1);
        assert_eq!(
            circle[0].1,
            vec![
                d + 0xd0,
                (13.0f32 * 2.0).to_bits(),
                0.25f32.to_bits(),
                0.75f32.to_bits(),
                24,
                d + 0x50,
                frame + 0x50,
                frame + 0x60
            ]
        );
        let shows = calls(&r, GEOMETRY_SLOT);
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].1[2..], [0xffff_ffff, 0x111, 0x222]);
        assert_eq!(shows[0].2[0], [1, d + 0xd0, 0, 0]);
        let frees = calls(&r, FREE_SLOT);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1[2], 4);
    }

    #[test]
    fn the_fan_drawer_scales_the_rotated_directions() {
        let mut r = rig();
        // the rotation doubles write the direction they were given into their result
        {
            let address = SET_ROTATED_DIR;
            r.e.register(address, |e, words| {
                for lane in 0..4 {
                    let v = e.mem.u32(words[2] + 4 * lane);
                    e.mem.set_u32(words[0] + 4 * lane, v);
                }
                Ret::default()
            });
        }
        run_fan(&mut r);
        let frame = calls(&r, UPDATE_COMMON_PARAMETERS)[0].1[2] - 0x70;
        let lines = calls(&r, LINE_SLOT);
        // frame +0x50 holds data +0x50 = x; s = 6
        let forward = 4.5f32; // 0.75 * 6
        assert_eq!(
            lines[1].2,
            vec![
                bits([-forward + 1.0, 2.0, 3.0, 0.0]),
                bits([forward + 1.0, 2.0, 3.0, 0.0])
            ]
        );
        // frame +0x60 holds data +0x70 = z: the end of the second limit line
        assert_eq!(
            lines[2].2,
            vec![bits([1.0, 2.0, 3.0, 0.0]), bits([1.0, 2.0, 3.0 + 4.5, 0.0])]
        );
        // data +0x40 = (0, 3, 4) normalised by the estimate, scaled by 0.5 * 6 = 3 from the
        // drawer's point +0x40 = (10, 20, 30)
        let squared = 25.0f32;
        let scale = scale_of(squared);
        let unit = [0.0, 3.0 * scale, 4.0 * scale, 0.0];
        let end: Vec<f32> = [10.0f32, 20.0, 30.0, 0.0]
            .iter()
            .zip(unit)
            .map(|(p, u)| 3.0 * u + p)
            .collect();
        assert_eq!(
            lines[3].1[1..],
            [frame + 0x30, frame + 0x20, 0xffff_ff00, 0x111, 0x222]
        );
        assert_eq!(lines[3].2[0], bits([10.0, 20.0, 30.0, 0.0]));
        assert_eq!(lines[3].2[1], bits([end[0], end[1], end[2], end[3]]));
    }
}

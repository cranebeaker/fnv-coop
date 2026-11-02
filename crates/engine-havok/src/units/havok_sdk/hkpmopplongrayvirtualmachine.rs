//! `Havok SDK/hkpmopplongrayvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The tree query both entry points end in (`this`, `(state, bytecode, ray, 0)`).
const QUERY_RAY_ON_TREE: u32 = 0x00dd_1f30;
/// `2^-16`, the `double` at `010d5198` that scales the extent.
const SCALE: f32 = 1.0 / 65536.0;
/// The byte the tree query checks first (`01269cbd`) and the cdecl check `00df22f0` it
/// calls with the byte's address while the byte is 0 (map name `hkpProcessFlyingColors`,
/// `hkpsolvercheck.cpp`).
const GUARD_FLAG: u32 = 0x0126_9cbd;
const GUARD_CHECK: u32 = 0x00df_22f0;
/// Read-only constants: the `double`s `0.0` (`01012060`) and `1.0` (`01012070`), the
/// `float` `0.0` (`01011d78`) and the four-lane `1.0` vector (`010ec7f0`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const FLOAT_ZERO: u32 = 0x0101_1d78;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// The unknown-command report: text stream constructor `00cb1600(stream, buffer, size)`,
/// `operator<<(char*)` `00cc01c0`, destructor `00cc00e0`; the Havok error object
/// (`01268360` holds a pointer to it), slot `0xC` `(level, id, text, file, line)`; the text
/// "Unknown command - This mopp data has been corrupt..." (`010d5100`), the source file
/// name (`010d7b2c`), id `0x1298fedd` and line 554.
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_OBJECT: u32 = 0x0126_8360;
const UNKNOWN_COMMAND_TEXT: u32 = 0x010d_5100;
const SOURCE_FILE_TEXT: u32 = 0x010d_7b2c;
const UNKNOWN_COMMAND_ID: u32 = 0x1298_fedd;
const UNKNOWN_COMMAND_LINE: u32 = 0x22a;
/// The tree query's stack frame, by the offsets its code uses from its aligned `ESP`: the
/// `hkBool` the shape calls return into (`+0x2F`), its own copy of the 0x20-byte query
/// state (`+0x40`, property word at `+0x5C`), the `hkpCdBody` it passes to
/// `castRayWithCollector` (`+0x60`, also a scratch copy of the segment end), the text
/// stream (`+0x74`), a scratch copy of the segment start (`+0x80`), the ray segment it
/// passes to its recursion (`+0x90`) and the 0x200-byte shape buffer / report text
/// (`+0xB0`).
const FRAME_SIZE: u32 = 0x2b0;
const FRAME_RESULT: u32 = 0x2f;
const FRAME_STATE: u32 = 0x40;
const FRAME_PROPERTY: u32 = 0x5c;
const FRAME_BODY: u32 = 0x60;
const FRAME_STREAM: u32 = 0x74;
const FRAME_START: u32 = 0x80;
const FRAME_RAY: u32 = 0x90;
const FRAME_BUFFER: u32 = 0xb0;
/// The axis word the loop stores before each command (999): no single-axis split.
const NO_AXIS: i32 = 999;

// Translated from 00dd1f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x20-byte record at `source` into `this`: 16 raw bytes, a word, a `float`
/// (through the x87) and two more words.
pub fn fn_00dd1f00(e: &mut Engine, this: Ptr, source: Ptr) {
    for offset in (0..0x10).step_by(4) {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
    let word = e.mem.u32(source.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, word);
    let float = x87_quiet_bits(e.mem.u32(source.addr() + 0x14));
    e.mem.set_u32(this.addr() + 0x14, float);
    for offset in [0x18, 0x1c] {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
}

/// Reads four `float` lanes.
fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(at + offset))
}

/// The part both query entry points share: stores the tree object at `this + 0x10` and
/// `1 / extent` at `this + 0x14`, the ray at `this + 0x20` / `0x30` / `0x40` / `0x44`,
/// clears the byte at `this + 0x50`, stores `-1` (first bytecode byte is 13) or `0` at
/// `this + 0x68`, then runs the tree query with a zeroed 0x20-byte state whose word at +0x14
/// is the extent scaled by `2^-16`, and with the ray moved relative to the tree object's
/// vector at +0x10 and scaled by that value.
fn run_query(e: &mut Engine, this: Ptr, tree: Ptr, ray: Ptr) {
    let this_addr = this.addr();
    let extent = e.mem.f32(tree.addr() + 0x1c);
    e.mem.set_u32(this_addr + 0x10, tree.addr());
    e.mem
        .set_f32(this_addr + 0x14, (1.0f64 / f64::from(extent)) as f32);
    let bytecode = e.mem.u32(tree.addr() + 0x20);
    let scale = extent * SCALE;
    let origin = lanes(e, tree.addr() + 0x10);
    let start = lanes(e, ray.addr());
    let end = lanes(e, ray.addr() + 0x10);
    for offset in (0..0x10).step_by(4) {
        let from = e.mem.u32(ray.addr() + offset);
        e.mem.set_u32(this_addr + 0x20 + offset, from);
        let to = e.mem.u32(ray.addr() + 0x10 + offset);
        e.mem.set_u32(this_addr + 0x30 + offset, to);
    }
    let first = e.mem.u32(ray.addr() + 0x20);
    e.mem.set_u32(this_addr + 0x40, first);
    let second = e.mem.u32(ray.addr() + 0x24);
    e.mem.set_u32(this_addr + 0x44, second);
    e.mem.set_u8(this_addr + 0x50, 0);
    let opcode = e.mem.u8(bytecode);
    let mode = if opcode != 0x0d { 0 } else { u32::MAX };
    e.mem.set_u32(this_addr + 0x68, mode);
    e.with_stack(0x60, |e, block| {
        let state = block.addr();
        let segment = state + 0x20;
        e.mem.set_f32(state + 0x14, scale);
        for lane in 0..4u32 {
            let i = lane as usize;
            e.mem
                .set_f32(segment + 4 * lane, scale * (start[i] - origin[i]));
            e.mem
                .set_f32(segment + 0x10 + 4 * lane, scale * (end[i] - origin[i]));
        }
        e.call(
            QUERY_RAY_ON_TREE,
            &args![this, state, bytecode, segment, 0u32],
        );
    });
}

// Translated from 00dd2df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `this + 0x54` to 1, `this + 0x64` to `collector`, `this + 0x58` to `extra` and runs
/// the shared query (see `run_query`) on `tree` and `ray`; then stores the byte at
/// `this + 0x50` in the byte `out_hit` points to.
/// Answers `out_hit`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00dd2df0(
    e: &mut Engine,
    this: Ptr,
    out_hit: Ptr,
    collector: u32,
    tree: Ptr,
    ray: Ptr,
    extra: u32,
) -> Ptr {
    let one = x87_quiet_bits(0x3f80_0000);
    e.mem.set_u32(this.addr() + 0x54, one);
    e.mem.set_u32(this.addr() + 0x64, collector);
    e.mem.set_u32(this.addr() + 0x58, extra);
    run_query(e, this, tree, ray);
    let hit = e.mem.u8(this.addr() + 0x50);
    e.mem.set_u8(out_hit.addr(), hit);
    out_hit
}

// Translated from 00dd2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The variant without an output byte: stores `collector` at `this + 0x64`, `second` at
/// `this + 0x5c`, `first` at `this + 0x60`, 0 at `this + 0x58`, the `float` at
/// `second + 4` at `this + 0x54`, and runs the shared query (see `run_query`).
pub fn fn_00dd2ed0(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    tree: Ptr,
    ray: Ptr,
    first: u32,
    second: Ptr,
) {
    e.mem.set_u32(this.addr() + 0x64, collector);
    e.mem.set_u32(this.addr() + 0x5c, second.addr());
    e.mem.set_u32(this.addr() + 0x60, first);
    e.mem.set_u32(this.addr() + 0x58, 0);
    let weight = x87_quiet_bits(e.mem.u32(second.addr() + 4));
    e.mem.set_u32(this.addr() + 0x54, weight);
    run_query(e, this, tree, ray);
}

/// One byte of the bytecode, widened (`MOVZX`).
fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

/// Writes four `float` lanes.
fn store_lanes(e: &mut Engine, at: u32, values: [f32; 4]) {
    for (lane, value) in (0u32..).zip(values) {
        e.mem.set_f32(at + 4 * lane, value);
    }
}

/// Copies `len` bytes word by word (`MOV` pairs).
fn copy_words(e: &mut Engine, to: u32, from: u32, len: u32) {
    for offset in (0..len).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// A `float` widened onto the x87 stack.
fn wide(value: f32) -> f64 {
    f64::from(value)
}

/// `a / (a - b)` on the x87, stored as a `float`: where the segment between two signed
/// plane distances crosses the plane.
fn crossing(a: f32, b: f32) -> f32 {
    (wide(a) / (wide(a) - wide(b))) as f32
}

/// `(1 - t) * a + t * b` lane by lane (SSE, `1` from the vector at `010ec7f0`).
fn blend(e: &Engine, t: f32, a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let ones = lanes(e, ONES_VECTOR);
    [0, 1, 2, 3].map(|i| (ones[i] - t) * a[i] + t * b[i])
}

/// The query state the commands that change it work on: the frame's own copy, made from
/// the current state with `00dd1f00` unless the current state is that copy already.
fn own_state(e: &mut Engine, state: u32, frame: u32) -> u32 {
    let local = frame + FRAME_STATE;
    if state != local {
        fn_00dd1f00(e, Ptr::new(local), Ptr::new(state));
    }
    local
}

/// A split command decoded: the axis word (`NO_AXIS` for the combined axes), the two plane
/// values (`high` bounds the first child, `low` starts the second), the ray's start and
/// end coordinates along the split direction, the offsets of the two children from the
/// command's end, and that end.
struct Split {
    axis: i32,
    high: f32,
    low: f32,
    start: f32,
    end: f32,
    first: u32,
    second: u32,
    next: u32,
}

/// Decodes split commands `0x10`..`0x25`.
fn decode_split(e: &Engine, opcode: u8, pc: u32, segment: u32) -> Split {
    let b1 = byte(e, pc + 1);
    let b2 = byte(e, pc + 2);
    let point = lanes(e, segment);
    let other = lanes(e, segment + 0x10);
    // Single axis: the coordinates are copied through the x87.
    let along = |axis: u32| {
        let i = axis as usize;
        (x87_quiet(point[i]), x87_quiet(other[i]))
    };
    let sum = |v: [f32; 4], x: usize, y: usize| (wide(v[x]) + wide(v[y])) as f32;
    let difference = |v: [f32; 4], x: usize, y: usize| (wide(v[x]) - wide(v[y])) as f32;
    let doubled = |b: u32| (2 * b as i32) as f32;
    let doubled_centered = |b: u32| (2 * b as i32 - 0xff) as f32;
    let tripled = |b: u32, bias: i32| (3 * (b as i32 - bias)) as f32;
    let combined = |high: f32, low: f32, start: f32, end: f32| Split {
        axis: NO_AXIS,
        high,
        low,
        start,
        end,
        first: 0,
        second: byte(e, pc + 3),
        next: pc + 4,
    };
    match opcode {
        0x10..=0x12 => {
            let axis = u32::from(opcode - 0x10);
            let (start, end) = along(axis);
            Split {
                axis: axis as i32,
                high: b1 as f32,
                low: b2 as f32,
                start,
                end,
                first: 0,
                second: byte(e, pc + 3),
                next: pc + 4,
            }
        }
        0x13 => combined(doubled(b1), doubled(b2), sum(point, 2, 1), sum(other, 2, 1)),
        0x14 => combined(
            doubled_centered(b1),
            doubled_centered(b2),
            difference(point, 1, 2),
            difference(other, 1, 2),
        ),
        0x15 => combined(doubled(b1), doubled(b2), sum(point, 2, 0), sum(other, 2, 0)),
        0x16 => combined(
            doubled_centered(b1),
            doubled_centered(b2),
            difference(point, 0, 2),
            difference(other, 0, 2),
        ),
        0x17 => combined(doubled(b1), doubled(b2), sum(point, 1, 0), sum(other, 1, 0)),
        0x18 => combined(
            doubled_centered(b1),
            doubled_centered(b2),
            difference(point, 0, 1),
            difference(other, 0, 1),
        ),
        0x19..=0x1c => {
            let bias = match opcode {
                0x19 => 0,
                0x1a | 0x1b => 0x55,
                _ => 0xaa,
            };
            // `(x ± y) ± z` without storing the middle result.
            let three = |v: [f32; 4]| {
                let (x, y, z) = (wide(v[0]), wide(v[1]), wide(v[2]));
                (match opcode {
                    0x19 => (y + x) + z,
                    0x1a => (y + x) - z,
                    0x1b => (x - y) + z,
                    _ => (x - y) - z,
                }) as f32
            };
            combined(
                tripled(b1, bias),
                tripled(b2, bias),
                three(point),
                three(other),
            )
        }
        0x20..=0x22 => {
            let axis = u32::from(opcode - 0x20);
            let (start, end) = along(axis);
            let low = b1 as f32;
            let one = e.mem.f64(ONE_DOUBLE);
            Split {
                axis: axis as i32,
                high: (one + wide(low)) as f32,
                low,
                start,
                end,
                first: 0,
                second: b2,
                next: pc + 3,
            }
        }
        _ => {
            let axis = u32::from(opcode - 0x23);
            let (start, end) = along(axis);
            Split {
                axis: axis as i32,
                high: b1 as f32,
                low: b2 as f32,
                start,
                end,
                first: (byte(e, pc + 3) << 8) + byte(e, pc + 4),
                second: (byte(e, pc + 5) << 8) + byte(e, pc + 6),
                next: pc + 7,
            }
        }
    }
}

/// After a hit has lowered the machine's early-out fraction (`this + 0x54`) below 1: the
/// segment end becomes the point at that fraction of the machine's ray (`this + 0x20` to
/// `this + 0x30`), relative to the tree object's vector (+0x10), scaled by the state's
/// factor (+0x14) and moved by its offset (+0x00). Returns the new end.
fn shorten_to_hit(e: &mut Engine, this: u32, state: u32, segment: u32) -> [f32; 4] {
    let fraction = x87_quiet(e.mem.f32(this + 0x54));
    let ones = lanes(e, ONES_VECTOR);
    let from = lanes(e, this + 0x20);
    let to = lanes(e, this + 0x30);
    let tree = e.mem.u32(this + 0x10);
    let origin = lanes(e, tree + 0x10);
    let scale = e.mem.f32(state + 0x14);
    let offset = lanes(e, state);
    let end = [0, 1, 2, 3].map(|i| {
        let point = (ones[i] - fraction) * from[i] + to[i] * fraction - origin[i];
        scale * point - offset[i]
    });
    store_lanes(e, segment + 0x10, end);
    end
}

/// Runs a split node; `None` ends the query (a hit before the second child's region),
/// otherwise the next command's address.
fn run_split(
    e: &mut Engine,
    this: u32,
    state: u32,
    segment: u32,
    chunk: u32,
    frame: u32,
    split: &Split,
) -> Option<u32> {
    let Split {
        axis,
        high,
        low,
        start,
        end,
        ..
    } = *split;
    let first = split.next.wrapping_add(split.first);
    let second = split.next.wrapping_add(split.second);
    // Entirely below the second child's plane: the first child only.
    if end < low && start < low {
        return Some(first);
    }
    // Entirely above the first child's plane: the second child only.
    if high < start && high < end {
        return Some(second);
    }
    let point = lanes(e, segment);
    let other = lanes(e, segment + 0x10);
    let ray = frame + FRAME_RAY;
    copy_words(e, ray, segment, 0x20);
    let start_high = (wide(start) - wide(high)) as f32;
    let start_low = (wide(start) - wide(low)) as f32;
    let end_high = (wide(end) - wide(high)) as f32;
    let end_low = (wide(end) - wide(low)) as f32;
    let zero = e.mem.f64(ZERO_DOUBLE);
    let one: f32 = 1.0;
    if end_high > start_high {
        // Rising: the first child gets the part up to the high plane.
        if wide(end_high) * wide(start_high) < zero {
            let t = crossing(start_high, end_high);
            let clipped = blend(e, t, point, other);
            store_lanes(e, ray + 0x10, clipped);
        }
        hkp_mopp_long_ray_virtual_machine_query_ray_on_tree(
            e,
            Ptr::new(this),
            Ptr::new(state),
            first,
            Ptr::new(ray),
            chunk,
        );
        // The second child gets the part from the low plane on.
        if wide(end_low) * wide(start_low) < zero {
            let t = crossing(start_low, end_low);
            let (a, b) = (lanes(e, segment), lanes(e, segment + 0x10));
            let clipped = blend(e, t, a, b);
            store_lanes(e, segment, clipped);
        }
        let shortened = one > e.mem.f32(this + 0x54);
        if !shortened {
            return Some(second);
        }
        let new_end = shorten_to_hit(e, this, state, segment);
        if axis < 3 && low > new_end[axis as usize] {
            return None;
        }
        Some(second)
    } else {
        // Falling (or level): the second child first, down to the low plane.
        if wide(end_low) * wide(start_low) < zero {
            let t = crossing(start_low, end_low);
            let clipped = blend(e, t, point, other);
            store_lanes(e, ray + 0x10, clipped);
        }
        hkp_mopp_long_ray_virtual_machine_query_ray_on_tree(
            e,
            Ptr::new(this),
            Ptr::new(state),
            second,
            Ptr::new(ray),
            chunk,
        );
        // Then the first child, from the high plane on.
        if wide(end_high) * wide(start_high) < zero {
            let t = crossing(start_high, end_high);
            let (a, b) = (lanes(e, segment), lanes(e, segment + 0x10));
            let clipped = blend(e, t, a, b);
            store_lanes(e, segment, clipped);
        }
        let shortened = one > e.mem.f32(this + 0x54);
        if !shortened {
            return Some(first);
        }
        let new_end = shorten_to_hit(e, this, state, segment);
        if axis < 3 && high < new_end[axis as usize] {
            return None;
        }
        Some(first)
    }
}

/// Runs a bounds command (`0x26`..`0x2B`): `false` ends the query (the segment misses
/// `[low, high]` on `axis`); otherwise the segment is clipped to the range in place.
fn run_bounds(e: &mut Engine, segment: u32, frame: u32, axis: u32, low: f32, high: f32) -> bool {
    let start = x87_quiet(e.mem.f32(segment + 4 * axis));
    let end = x87_quiet(e.mem.f32(segment + 0x10 + 4 * axis));
    // Which end the high plane clips: 0 the end (rising), 1 the start.
    let falling = if end > start {
        if low > end || high < start {
            return false;
        }
        0
    } else {
        if low > start || high < end {
            return false;
        }
        1
    };
    copy_words(e, frame + FRAME_START, segment, 0x10);
    copy_words(e, frame + FRAME_BODY, segment + 0x10, 0x10);
    let point = lanes(e, frame + FRAME_START);
    let other = lanes(e, frame + FRAME_BODY);
    let start_high = (wide(start) - wide(high)) as f32;
    let end_high = (wide(end) - wide(high)) as f32;
    if 0.0 > wide(end_high) * wide(start_high) {
        let t = crossing(start_high, end_high);
        let clipped = blend(e, t, point, other);
        store_lanes(e, segment + 0x10 - 0x10 * falling, clipped);
    }
    let start_low = (wide(start) - wide(low)) as f32;
    let end_low = (wide(end) - wide(low)) as f32;
    if wide(end_low) * wide(start_low) < 0.0 {
        let t = crossing(start_low, end_low);
        let clipped = blend(e, t, point, other);
        store_lanes(e, segment + 0x10 * falling, clipped);
    }
    true
}

/// Commands `0x01`..`0x04`: moves the segment by `(b1, b2, b3, 0)` and scales it by
/// `2^opcode`; the frame's state copy gets `2^opcode * (b + offset)` as its offset, the
/// shift word (+0x10) plus `opcode`, the factor (+0x14) times `2^opcode`, and the current
/// state's two last words. Returns the frame's copy.
fn rescale(e: &mut Engine, state: u32, segment: u32, frame: u32, opcode: u8, pc: u32) -> u32 {
    let zero = e.mem.f32(FLOAT_ZERO);
    let shift = [
        byte(e, pc + 1) as f32,
        byte(e, pc + 2) as f32,
        byte(e, pc + 3) as f32,
        zero,
    ];
    let factor = (1i32 << opcode) as f32;
    for half in [0, 0x10] {
        let moved = lanes(e, segment + half);
        let moved = [0, 1, 2, 3].map(|i| moved[i] - shift[i]);
        store_lanes(e, segment + half, [0, 1, 2, 3].map(|i| factor * moved[i]));
    }
    let local = frame + FRAME_STATE;
    let offset = lanes(e, state);
    let offset = [0, 1, 2, 3].map(|i| factor * (shift[i] + offset[i]));
    store_lanes(e, local, offset);
    let shift_word = e.mem.u32(state + 0x10).wrapping_add(u32::from(opcode));
    e.mem.set_u32(local + 0x10, shift_word);
    let scale = (wide(e.mem.f32(state + 0x14)) * wide(factor)) as f32;
    e.mem.set_f32(local + 0x14, scale);
    let property = e.mem.u32(state + 0x1c);
    e.mem.set_u32(local + 0x1c, property);
    let base = e.mem.u32(state + 0x18);
    e.mem.set_u32(local + 0x18, base);
    local
}

/// A terminal: the shape key is `((chunk >> 9) << 8) & mask (this + 0x68)` or'ed with the
/// state's terminal base (+0x18) plus `value`. The child shape comes from the container
/// (`this + 0x64`, `hkpShapeContainer::getChildShape` (Xbox PDB) slot `0x14`,
/// `(key, buffer)`). If the ray input's filter (`this + 0x44`) is set, its slot `0`
/// (`isCollisionEnabled` (Xbox PDB), `(result, input, child, container, key)`) must
/// answer true. With an output (`this + 0x58`), the child's slot `0x20` (`castRayImpl`
/// (Xbox PDB), `(result, input, output)`) runs with the output's key depth (+0x40) one
/// deeper; on a hit the machine's hit byte (+0x50) and early-out fraction (+0x54, the
/// output's +0x10) are set and the key is stored at the output's +0x20 slot of that
/// depth. Without one, the child's slot `0x24` (`castRayWithCollectorImpl` (Xbox PDB),
/// `(input, body, collector)`) runs with an `hkpCdBody` `{child, key, parent's +8,
/// parent}` (parent `this + 0x60`, collector `this + 0x5c`), and the collector's +4 becomes
/// the early-out fraction.
fn report_terminal(e: &mut Engine, this: u32, state: u32, chunk: u32, value: u32, frame: u32) {
    let high = (((chunk as i32) >> 9) << 8) as u32 & e.mem.u32(this + 0x68);
    let key = high | e.mem.u32(state + 0x18).wrapping_add(value);
    let container = e.mem.u32(this + 0x64);
    let buffer = frame + FRAME_BUFFER;
    let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
    let input = this + 0x20;
    let result = frame + FRAME_RESULT;
    let filter = e.mem.u32(this + 0x44);
    if filter != 0 {
        let container = e.mem.u32(this + 0x64);
        let answer = e
            .vcall(filter, 0, &args![result, input, child, container, key])
            .u32();
        if e.mem.u8(answer) == 0 {
            return;
        }
    }
    let output = e.mem.u32(this + 0x58);
    if output != 0 {
        let depth = e.mem.u32(output + 0x40).wrapping_add(1);
        e.mem.set_u32(output + 0x40, depth);
        let output = e.mem.u32(this + 0x58);
        e.vcall(child, 0x20, &args![result, input, output]);
        let output = e.mem.u32(this + 0x58);
        let depth = e.mem.u32(output + 0x40).wrapping_sub(1);
        e.mem.set_u32(output + 0x40, depth);
        let hit = e.mem.u8(result);
        if hit == 0 {
            return;
        }
        e.mem.set_u8(this + 0x50, hit);
        let output = e.mem.u32(this + 0x58);
        let fraction = x87_quiet_bits(e.mem.u32(output + 0x10));
        e.mem.set_u32(this + 0x54, fraction);
        let depth = e.mem.u32(output + 0x40);
        e.mem
            .set_u32(output.wrapping_add(depth.wrapping_mul(4)) + 0x20, key);
        return;
    }
    let parent = e.mem.u32(this + 0x60);
    let collector = e.mem.u32(this + 0x5c);
    let body = frame + FRAME_BODY;
    e.mem.set_u32(body + 0xc, parent);
    let motion = e.mem.u32(parent + 8);
    e.mem.set_u32(body + 8, motion);
    e.mem.set_u32(body, child);
    e.mem.set_u32(body + 4, key);
    e.vcall(child, 0x24, &args![input, body, collector]);
    let collector = e.mem.u32(this + 0x5c);
    let fraction = x87_quiet_bits(e.mem.u32(collector + 4));
    e.mem.set_u32(this + 0x54, fraction);
}

/// An opcode the machine does not know: reports it through the Havok error object (level
/// 3) and returns; the loop then reads the same opcode again. The exe breaks into the
/// debugger (`INT3` at `00dd2b96`) when the report call returns non-zero; here that
/// panics.
fn report_unknown_command(e: &mut Engine, frame: u32) {
    let stream = frame + FRAME_STREAM;
    let buffer = frame + FRAME_BUFFER;
    e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(STREAM_WRITE, &args![stream, UNKNOWN_COMMAND_TEXT]);
    let reporter = e.mem.u32(ERROR_OBJECT);
    let stop = e
        .vcall(
            reporter,
            0xc,
            &args![
                3u32,
                UNKNOWN_COMMAND_ID,
                buffer,
                SOURCE_FILE_TEXT,
                UNKNOWN_COMMAND_LINE
            ],
        )
        .u32();
    if stop != 0 {
        panic!("INT3 at 00dd2b96: the Havok error object stopped on an unknown MOPP command");
    }
    e.call(STREAM_DESTRUCT, &args![stream]);
}

// Translated from 00dd1f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppLongRayVirtualMachine::queryRayOnTree` (Xbox PDB): runs the MOPP bytecode at
/// `code` for the ray segment at `segment` (start and end, two four-float vectors, in the
/// state's integer space), `state` being the 0x20-byte query state (offset vector,
/// shift word +0x10, factor +0x14, terminal base +0x18, property +0x1C) and `chunk` the
/// chunk offset the terminal keys use. Nothing runs while the byte at `01269cbd` stays 0
/// after the check `00df22f0`.
///
/// Commands (`opcode` → action): `00` return; `01`..`04` rescale (see `rescale`);
/// `05`/`06`/`07` jump forward by an 8/16/24-bit offset; `09`/`0A` add an 8/16-bit value
/// to the terminal base, `0B` set it (32 bits); `0C` jump to chunk `n` (16 bits, offset
/// `n << 9`) of the tree's code (`this + 0x10`, +0x20), `70` to a 32-bit offset, both
/// becoming the chunk offset; `0D` skip 5 bytes; `10`..`25` splits (see `decode_split`;
/// a segment crossing both planes runs the nearer child through a recursive call on a
/// clipped copy of the segment and continues with the other on the clipped segment);
/// `26`..`2B` bounds (8-bit or 24-bit planes, see `run_bounds`); `30`..`4F`, `50`..`53`
/// terminals (value `opcode - 0x30`, or 8/16/24/32 bits; see `report_terminal`), which
/// end the call; `60`..`6B` set property word `opcode & 3` (8/16/32 bits) of the frame's
/// state copy, whose single property word sits at `+0x5C`: indices 1..3 write the frame
/// words after it; every other opcode (`08`, `0E`, `0F`, `1D`..`1F`, `2C`..`2F`,
/// `54`..`5F`, `6C`..`6F`, above `70`) is reported (see `report_unknown_command`) and,
/// as in the exe, read again.
pub fn hkp_mopp_long_ray_virtual_machine_query_ray_on_tree(
    e: &mut Engine,
    this: Ptr,
    state: Ptr,
    code: u32,
    segment: Ptr,
    chunk: u32,
) {
    if e.mem.u8(GUARD_FLAG) == 0 {
        e.call(GUARD_CHECK, &args![GUARD_FLAG]);
        if e.mem.u8(GUARD_FLAG) == 0 {
            return;
        }
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        run_tree(
            e,
            this.addr(),
            state.addr(),
            code,
            segment.addr(),
            chunk,
            frame.addr(),
        )
    });
}

/// The command loop of `00dd1f30` (see there).
fn run_tree(
    e: &mut Engine,
    this: u32,
    mut state: u32,
    mut pc: u32,
    segment: u32,
    mut chunk: u32,
    frame: u32,
) {
    loop {
        let opcode = e.mem.u8(pc);
        match opcode {
            0x00 => return,
            0x01..=0x04 => {
                state = rescale(e, state, segment, frame, opcode, pc);
                pc = pc.wrapping_add(4);
            }
            0x05 => pc = pc.wrapping_add(byte(e, pc + 1) + 2),
            0x06 => {
                let offset = (byte(e, pc + 1) << 8) + byte(e, pc + 2);
                pc = pc.wrapping_add(offset + 3);
            }
            0x07 => {
                let offset = (((byte(e, pc + 1) << 8) + byte(e, pc + 2)) << 8) + byte(e, pc + 3);
                pc = pc.wrapping_add(offset + 4);
            }
            0x09 | 0x0a => {
                let (value, len) = if opcode == 0x09 {
                    (byte(e, pc + 1), 2)
                } else {
                    ((byte(e, pc + 1) << 8) + byte(e, pc + 2), 3)
                };
                state = own_state(e, state, frame);
                let base = e.mem.u32(state + 0x18).wrapping_add(value);
                e.mem.set_u32(state + 0x18, base);
                pc = pc.wrapping_add(len);
            }
            0x0b => {
                let value = word32(e, pc);
                state = own_state(e, state, frame);
                e.mem.set_u32(state + 0x18, value);
                pc = pc.wrapping_add(5);
            }
            0x0c => {
                let offset = ((byte(e, pc + 1) << 8) | byte(e, pc + 2)) << 9;
                chunk = offset;
                pc = tree_code(e, this).wrapping_add(offset);
            }
            0x70 => {
                let offset = word32(e, pc);
                chunk = offset;
                pc = tree_code(e, this).wrapping_add(offset);
            }
            0x0d => pc = pc.wrapping_add(5),
            0x10..=0x1c | 0x20..=0x25 => {
                let split = decode_split(e, opcode, pc, segment);
                match run_split(e, this, state, segment, chunk, frame, &split) {
                    Some(next) => pc = next,
                    None => return,
                }
            }
            0x26..=0x2b => {
                let (axis, low, high) = if opcode <= 0x28 {
                    let axis = u32::from(opcode - 0x26);
                    let low = byte(e, pc + 1) as f32;
                    let high = byte(e, pc + 2) as f32;
                    pc = pc.wrapping_add(3);
                    (axis, low, high)
                } else {
                    let axis = u32::from(opcode - 0x29);
                    let low = bounds_plane(e, this, state, axis, word24(e, pc + 1));
                    let high = bounds_plane(e, this, state, axis, word24(e, pc + 4));
                    pc = pc.wrapping_add(7);
                    (axis, low, high)
                };
                if !run_bounds(e, segment, frame, axis, low, high) {
                    return;
                }
            }
            0x30..=0x53 => {
                let value = match opcode {
                    0x50 => byte(e, pc + 1),
                    0x51 => (byte(e, pc + 1) << 8) + byte(e, pc + 2),
                    0x52 => word24(e, pc + 1),
                    0x53 => word32(e, pc),
                    _ => u32::from(opcode - 0x30),
                };
                report_terminal(e, this, state, chunk, value, frame);
                return;
            }
            0x60..=0x6b => {
                let (value, len) = match opcode {
                    0x60..=0x63 => (byte(e, pc + 1), 2),
                    0x64..=0x67 => ((byte(e, pc + 1) << 8) + byte(e, pc + 2), 3),
                    _ => (word32(e, pc), 5),
                };
                let index = u32::from(opcode & 3);
                e.mem.set_u32(frame + FRAME_PROPERTY + 4 * index, value);
                pc = pc.wrapping_add(len);
                let kept = e.mem.u32(frame + FRAME_PROPERTY);
                state = own_state(e, state, frame);
                e.mem.set_u32(frame + FRAME_PROPERTY, kept);
            }
            _ => report_unknown_command(e, frame),
        }
    }
}

/// The big-endian 32-bit value after the opcode at `pc` (shifts and adds).
fn word32(e: &Engine, pc: u32) -> u32 {
    let high = (((byte(e, pc + 1) << 8) + byte(e, pc + 2)) << 8) + byte(e, pc + 3);
    (high << 8).wrapping_add(byte(e, pc + 4))
}

/// The big-endian 24-bit value at `at`.
fn word24(e: &Engine, at: u32) -> u32 {
    (((byte(e, at) << 8) + byte(e, at + 1)) << 8) + byte(e, at + 2)
}

/// The tree object's bytecode base (`this + 0x10`, +0x20).
fn tree_code(e: &Engine, this: u32) -> u32 {
    let tree = e.mem.u32(this + 0x10);
    e.mem.u32(tree + 0x20)
}

/// A 24-bit bounds plane in the state's space: `value * (1 / extent) (this + 0x14)`, times
/// the state's factor (+0x14), minus the state's offset on `axis`, each step stored as a
/// `float`.
fn bounds_plane(e: &Engine, this: u32, state: u32, axis: u32, value: u32) -> f32 {
    let scaled = (f64::from(value as i32) * wide(e.mem.f32(this + 0x14))) as f32;
    let scaled = (wide(scaled) * wide(e.mem.f32(state + 0x14))) as f32;
    (wide(scaled) - wide(e.mem.f32(state + 4 * axis))) as f32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_1f00, fn_00dd1f00(Ptr, Ptr)),
        entry!(
            0x00dd_1f30,
            hkp_mopp_long_ray_virtual_machine_query_ray_on_tree(Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(0x00dd_2df0, fn_00dd2df0(Ptr, Ptr, u32, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00dd_2ed0, fn_00dd2ed0(Ptr, u32, Ptr, Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn record_copy_moves_all_32_bytes() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x20);
        let this = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(source + 4 * i, 0x1111_0000 + i);
        }
        e.mem.set_u32(source + 0x14, 0x3fc0_0000);
        e.call(0x00dd_1f00, &args![this, source]);
        for i in 0..8u32 {
            let expected = if i == 5 { 0x3fc0_0000 } else { 0x1111_0000 + i };
            assert_eq!(e.mem.u32(this + 4 * i), expected);
        }
    }

    /// What the tree-query double saw: (this, bytecode, last word, state words, segment, scale).
    type Seen = Rc<RefCell<Vec<(Vec<u32>, Vec<f32>, f32)>>>;

    /// A tree object with extent 4 at +0x1c and bytecode at +0x20 whose first byte is
    /// `opcode`; a ray from (3,4,5,6) to (7,8,9,10); a double for the tree query that
    /// records its arguments and the segment, and sets the hit byte.
    fn rig(e: &mut Engine, opcode: u8) -> (u32, u32, u32, Seen) {
        let this = e.mem.alloc(0x80);
        let tree = e.mem.alloc(0x40);
        let code = e.mem.alloc(8);
        e.mem.set_u8(code, opcode);
        e.mem.set_u32(tree + 0x20, code);
        e.mem.set_f32(tree + 0x1c, 4.0);
        for i in 0..4u32 {
            e.mem.set_f32(tree + 0x10 + 4 * i, 1.0 + i as f32);
        }
        let ray = e.mem.alloc(0x30);
        for i in 0..8u32 {
            e.mem.set_f32(ray + 4 * i, 3.0 + i as f32);
        }
        e.mem.set_u32(ray + 0x20, 0xaaaa);
        e.mem.set_u32(ray + 0x24, 0xbbbb);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(QUERY_RAY_ON_TREE, move |e, a| {
            let mut words = vec![a[0], a[2], a[4]];
            words.extend((0..8).map(|i| e.mem.u32(a[1] + 4 * i)));
            let segment: Vec<f32> = (0..8).map(|i| e.mem.f32(a[3] + 4 * i)).collect();
            let scale = e.mem.f32(a[1] + 0x14);
            e.mem.set_u8(a[0] + 0x50, 7);
            log.borrow_mut().push((words, segment, scale));
            Ret::default()
        });
        (this, tree, ray, seen)
    }

    #[test]
    fn long_ray_query_fills_the_machine_and_reports_the_hit() {
        let mut e = Engine::new();
        let (this, tree, ray, seen) = rig(&mut e, 0x0d);
        let out = e.mem.alloc(4);
        let extra = 0x5555u32;
        e.call(0x00dd_2df0, &args![this, out, 0x77u32, tree, ray, extra]);
        assert_eq!(e.mem.f32(this + 0x54), 1.0);
        assert_eq!(e.mem.u32(this + 0x64), 0x77);
        assert_eq!(e.mem.u32(this + 0x58), extra);
        assert_eq!(e.mem.u32(this + 0x10), tree);
        assert_eq!(e.mem.f32(this + 0x14), 0.25);
        assert_eq!(e.mem.f32(this + 0x20), 3.0);
        assert_eq!(e.mem.f32(this + 0x3c), 10.0);
        assert_eq!(e.mem.u32(this + 0x40), 0xaaaa);
        assert_eq!(e.mem.u32(this + 0x44), 0xbbbb);
        assert_eq!(e.mem.u32(this + 0x68), u32::MAX);
        assert_eq!(e.mem.u8(out), 7);
        let seen = seen.borrow();
        let (words, segment, scale) = &seen[0];
        assert_eq!(words[0], this);
        assert_eq!(words[2], 0);
        assert!(words[3..7].iter().all(|w| *w == 0));
        let unit = 4.0 / 65536.0;
        assert_eq!(*scale, unit);
        let expected: Vec<f32> = (0..8)
            .map(|i| unit * ((3.0 + i as f32) - (1.0 + (i % 4) as f32)))
            .collect();
        assert_eq!(*segment, expected);
    }

    #[test]
    fn fn_00dd2df0_answers_its_result_pointer() {
        let mut e = Engine::new();
        let (this, tree, ray, _seen) = rig(&mut e, 0x0d);
        let out = e.mem.alloc(4);
        let extra = 0x5555u32;
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00dd_2df0, &args![this, out, 0x77u32, tree, ray, extra])
                .u32(),
            args![out][0]
        );
    }

    #[test]
    fn other_first_opcode_stores_zero_mode() {
        let mut e = Engine::new();
        let (this, tree, ray, _) = rig(&mut e, 0x30);
        let out = e.mem.alloc(4);
        e.call(0x00dd_2df0, &args![this, out, 1u32, tree, ray, 2u32]);
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }

    #[test]
    fn long_ray_query_variant_uses_the_shifted_arguments() {
        let mut e = Engine::new();
        let (this, tree, ray, seen) = rig(&mut e, 0x0d);
        let second = e.mem.alloc(0x10);
        e.mem.set_f32(second + 4, 2.5);
        e.call(
            0x00dd_2ed0,
            &args![this, 0x77u32, tree, ray, 0x66u32, second],
        );
        assert_eq!(e.mem.u32(this + 0x64), 0x77);
        assert_eq!(e.mem.u32(this + 0x5c), second);
        assert_eq!(e.mem.u32(this + 0x60), 0x66);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.f32(this + 0x54), 2.5);
        assert_eq!(e.mem.u32(this + 0x10), tree);
        assert_eq!(e.mem.u32(this + 0x68), u32::MAX);
        assert_eq!(seen.borrow().len(), 1);
    }

    // The tree query (00dd1f30) on hand-built MOPP programs.

    const CONTAINER_VTABLE: u32 = 0x0300_1000;
    const CHILD_VTABLE: u32 = 0x0300_2000;
    const FILTER_VTABLE: u32 = 0x0300_3000;
    const REPORTER_VTABLE: u32 = 0x0300_4000;
    const GET_CHILD: u32 = 0x0900_0014;
    const CAST_RAY: u32 = 0x0900_0020;
    const CAST_WITH_COLLECTOR: u32 = 0x0900_0024;
    const FILTER_ENABLED: u32 = 0x0900_0030;
    const REPORT: u32 = 0x0900_0040;
    const BASE: u32 = 0x1000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A machine (`this`), a query state (factor 1, terminal base `BASE`), a segment, a
    /// tree object whose bytecode base is `code`, a container whose `getChildShape` logs
    /// the key and returns `child`, and a child whose `castRayWithCollector` logs its
    /// arguments and the body's words and copies the collector's +8 into its +4.
    struct Machine {
        this: u32,
        state: u32,
        segment: u32,
        code: u32,
        child: u32,
        container: u32,
        collector: u32,
        parent: u32,
        keys: Log,
        casts: Log,
    }

    fn machine(e: &mut Engine) -> Machine {
        e.map(GUARD_FLAG, 1);
        e.mem.set_u8(GUARD_FLAG, 1);
        e.map(ZERO_DOUBLE, 0x18);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.map(FLOAT_ZERO, 4);
        e.mem.set_f32(FLOAT_ZERO, 0.0);
        e.map(ONES_VECTOR, 0x10);
        store_lanes(e, ONES_VECTOR, [1.0; 4]);
        let this = e.mem.alloc(0x70);
        let tree = e.mem.alloc(0x30);
        let code = e.mem.alloc(0x400);
        e.mem.set_u32(tree + 0x20, code);
        let collector = e.mem.alloc(0x10);
        e.mem.set_f32(collector + 4, 1.0);
        e.mem.set_f32(collector + 8, 1.0);
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent + 8, 0x7777);
        let container = e.mem.alloc(8);
        e.put_vtable(CONTAINER_VTABLE, &[0, 0, 0, 0, 0, GET_CHILD]);
        e.mem.set_u32(container, CONTAINER_VTABLE);
        let child = e.mem.alloc(8);
        let mut slots = vec![0; 8];
        slots.extend([CAST_RAY, CAST_WITH_COLLECTOR]);
        e.put_vtable(CHILD_VTABLE, &slots);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.mem.set_u32(this + 0x10, tree);
        e.mem.set_f32(this + 0x14, 1.0);
        e.mem.set_f32(this + 0x54, 1.0);
        e.mem.set_u32(this + 0x5c, collector);
        e.mem.set_u32(this + 0x60, parent);
        e.mem.set_u32(this + 0x64, container);
        let state = e.mem.alloc(0x20);
        e.mem.set_f32(state + 0x14, 1.0);
        e.mem.set_u32(state + 0x18, BASE);
        let segment = e.mem.alloc(0x20);
        let keys: Log = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&keys);
        e.register_double(GET_CHILD, move |_, a| {
            log.borrow_mut().push(vec![a[1]]);
            Ret {
                eax: child,
                ..Ret::default()
            }
        });
        let casts: Log = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&casts);
        e.register_double(CAST_WITH_COLLECTOR, move |e, a| {
            let mut seen = vec![a[0], a[1]];
            seen.extend((0..4).map(|i| e.mem.u32(a[2] + 4 * i)));
            seen.push(a[3]);
            log.borrow_mut().push(seen);
            let next = e.mem.u32(a[3] + 8);
            e.mem.set_u32(a[3] + 4, next);
            Ret::default()
        });
        Machine {
            this,
            state,
            segment,
            code,
            child,
            container,
            collector,
            parent,
            keys,
            casts,
        }
    }

    fn program(e: &mut Engine, at: u32, bytes: &[u8]) {
        for (offset, b) in (0u32..).zip(bytes) {
            e.mem.set_u8(at + offset, *b);
        }
    }

    fn set_segment(e: &mut Engine, m: &Machine, start: [f32; 4], end: [f32; 4]) {
        store_lanes(e, m.segment, start);
        store_lanes(e, m.segment + 0x10, end);
    }

    fn run(e: &mut Engine, m: &Machine, at: u32, chunk: u32) {
        e.call(0x00dd_1f30, &args![m.this, m.state, at, m.segment, chunk]);
    }

    fn keys(m: &Machine) -> Vec<u32> {
        m.keys.borrow().iter().map(|k| k[0]).collect()
    }

    #[test]
    fn tree_query_waits_for_the_guard_byte() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        program(&mut e, m.code, &[0x30]);
        e.mem.set_u8(GUARD_FLAG, 0);
        e.register(GUARD_CHECK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        run(&mut e, &m, m.code, 0);
        let log = e.call_log.take().unwrap();
        let checks: Vec<_> = log.iter().filter(|(a, _)| *a == GUARD_CHECK).collect();
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].1, vec![GUARD_FLAG]);
        assert!(keys(&m).is_empty());
        // The check sets the byte: the query runs.
        e.register(GUARD_CHECK, |e, a| {
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE]);
        // Opcode 0 returns at once.
        program(&mut e, m.code, &[0x00, 0x30]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE]);
    }

    #[test]
    fn terminal_casts_through_the_collector_with_the_chunk_key() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        e.mem.set_u32(m.this + 0x68, u32::MAX);
        e.mem.set_f32(m.collector + 8, 0.75);
        // Base 0x2000 (32 bits), terminal 5, chunk offset 0x600: key 0x300 | 0x2005.
        program(&mut e, m.code, &[0x0b, 0x00, 0x00, 0x20, 0x00, 0x35]);
        run(&mut e, &m, m.code, 0x600);
        assert_eq!(keys(&m), vec![0x2305]);
        assert_eq!(
            m.casts.borrow()[0],
            vec![
                m.child,
                m.this + 0x20,
                m.child,
                0x2305,
                0x7777,
                m.parent,
                m.collector
            ]
        );
        assert_eq!(e.mem.f32(m.this + 0x54), 0.75);
        // The caller's state keeps its base: the command changed the frame's copy.
        assert_eq!(e.mem.u32(m.state + 0x18), BASE);
        // Without the mask the chunk bits are dropped.
        e.mem.set_u32(m.this + 0x68, 0);
        run(&mut e, &m, m.code, 0x600);
        assert_eq!(keys(&m)[1], 0x2005);
    }

    #[test]
    fn terminal_widths() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        let cases: [(&[u8], u32); 5] = [
            (&[0x4f], 0x1f),
            (&[0x50, 0x12], 0x12),
            (&[0x51, 0x12, 0x34], 0x1234),
            (&[0x52, 0x01, 0x02, 0x03], 0x01_0203),
            (&[0x53, 0x01, 0x02, 0x03, 0x04], 0x0102_0304),
        ];
        for (bytes, value) in cases {
            program(&mut e, m.code, bytes);
            run(&mut e, &m, m.code, 0);
            assert_eq!(*keys(&m).last().unwrap(), BASE.wrapping_add(value));
        }
    }

    #[test]
    fn jumps_base_changes_and_chunk_jumps() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        e.mem.set_u32(m.this + 0x68, u32::MAX);
        program(
            &mut e,
            m.code,
            &[
                0x05, 0x02, 0xff, 0xff, // +2: to 4
                0x06, 0x00, 0x01, 0xff, // +1: to 8
                0x07, 0x00, 0x00, 0x01, 0xff, // +1: to 13
                0x0d, 0xff, 0xff, 0xff, 0xff, // skip: to 18
                0x09, 0x03, // base + 3
                0x0a, 0x00, 0x20, // base + 0x20
                0x64, 0x12, 0x34, // property
                0x0c, 0x00, 0x01, // chunk 1: offset 0x200
            ],
        );
        program(&mut e, m.code + 0x200, &[0x31]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![0x100 | (BASE + 0x24)]);
        // A 32-bit chunk jump: offset 0x100, whose chunk bits (>> 9) are 0.
        program(&mut e, m.code + 0x300, &[0x70, 0x00, 0x00, 0x01, 0x00]);
        program(&mut e, m.code + 0x100, &[0x32]);
        run(&mut e, &m, m.code + 0x300, 0);
        assert_eq!(keys(&m)[1], BASE + 2);
        assert_eq!(e.mem.u32(m.state + 0x18), BASE);
    }

    #[test]
    fn axis_split_runs_the_children_in_ray_order() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        // High plane 25, low plane 15 on x; second child one byte after the first.
        program(&mut e, m.code, &[0x10, 25, 15, 1, 0x30, 0x31]);
        // Rising: first child, then the second from the low plane on (t = 0.25).
        set_segment(&mut e, &m, [10.0, 2.0, 4.0, 0.0], [30.0, 6.0, 8.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE + 1]);
        assert_eq!(lanes(&e, m.segment), [15.0, 3.0, 5.0, 0.0]);
        assert_eq!(lanes(&e, m.segment + 0x10), [30.0, 6.0, 8.0, 0.0]);
        // Falling: second child, then the first from the high plane on (t = 0.25).
        m.keys.borrow_mut().clear();
        set_segment(&mut e, &m, [30.0, 6.0, 8.0, 0.0], [10.0, 2.0, 4.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE + 1, BASE]);
        assert_eq!(lanes(&e, m.segment), [25.0, 5.0, 7.0, 0.0]);
        // Below the low plane: the first child only; above the high plane: the second.
        m.keys.borrow_mut().clear();
        set_segment(&mut e, &m, [1.0; 4], [5.0; 4]);
        run(&mut e, &m, m.code, 0);
        set_segment(&mut e, &m, [26.0; 4], [40.0; 4]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE + 1]);
        // A NaN end is never below the low plane nor above the high plane, and never
        // rising: the second child runs first, then the first.
        m.keys.borrow_mut().clear();
        set_segment(&mut e, &m, [1.0; 4], [f32::NAN; 4]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE + 1, BASE]);
    }

    #[test]
    fn axis_split_stops_after_a_hit_before_the_other_child() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        program(&mut e, m.code, &[0x10, 25, 15, 1, 0x30, 0x31]);
        // Rising, the first child's hit at 1/8 of the ray: end 12.5, below the low plane.
        store_lanes(&mut e, m.this + 0x20, [10.0, 0.0, 0.0, 0.0]);
        store_lanes(&mut e, m.this + 0x30, [30.0, 0.0, 0.0, 0.0]);
        set_segment(&mut e, &m, [10.0, 0.0, 0.0, 0.0], [30.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(m.collector + 8, 0.125);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE]);
        assert_eq!(e.mem.f32(m.this + 0x54), 0.125);
        assert_eq!(lanes(&e, m.segment + 0x10), [12.5, 0.0, 0.0, 0.0]);
        // A hit at 1/4: end 15, not below the low plane: the second child still runs.
        m.keys.borrow_mut().clear();
        e.mem.set_f32(m.this + 0x54, 1.0);
        e.mem.set_f32(m.collector + 8, 0.25);
        set_segment(&mut e, &m, [10.0, 0.0, 0.0, 0.0], [30.0, 0.0, 0.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE + 1]);
        // Falling, the second child's hit at 1/8: end 27.5, above the high plane.
        m.keys.borrow_mut().clear();
        e.mem.set_f32(m.this + 0x54, 1.0);
        e.mem.set_f32(m.collector + 8, 0.125);
        store_lanes(&mut e, m.this + 0x20, [30.0, 0.0, 0.0, 0.0]);
        store_lanes(&mut e, m.this + 0x30, [10.0, 0.0, 0.0, 0.0]);
        set_segment(&mut e, &m, [30.0, 0.0, 0.0, 0.0], [10.0, 0.0, 0.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE + 1]);
        assert_eq!(lanes(&e, m.segment + 0x10), [27.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn combined_and_offset_splits() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        // y + z split, planes 2 * 10 and 2 * 8: y + z from 3 to 7 is below 16.
        program(&mut e, m.code, &[0x13, 10, 8, 1, 0x30, 0x31]);
        set_segment(&mut e, &m, [0.0, 1.0, 2.0, 0.0], [0.0, 3.0, 4.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        // x - y - z split, planes 3 * (0xaa - 0xaa) and 3 * (0xa0 - 0xaa): 5 and 18 are
        // above 0.
        program(&mut e, m.code, &[0x1c, 0xaa, 0xa0, 1, 0x30, 0x31]);
        set_segment(&mut e, &m, [10.0, 2.0, 3.0, 0.0], [20.0, 1.0, 1.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE + 1]);
        // One-wide split on x: low 10, high 11; rising from 0 to 20 crosses both.
        m.keys.borrow_mut().clear();
        program(&mut e, m.code, &[0x20, 10, 1, 0x30, 0x31]);
        set_segment(&mut e, &m, [0.0, 0.0, 4.0, 0.0], [20.0, 2.0, 8.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE + 1]);
        assert_eq!(lanes(&e, m.segment), [10.0, 1.0, 6.0, 0.0]);
        // 16-bit child offsets: first at +2, second at +3 after the 7-byte command.
        m.keys.borrow_mut().clear();
        program(
            &mut e,
            m.code,
            &[0x23, 25, 15, 0, 2, 0, 3, 0xff, 0xff, 0x30, 0x31],
        );
        set_segment(&mut e, &m, [1.0; 4], [5.0; 4]);
        run(&mut e, &m, m.code, 0);
        set_segment(&mut e, &m, [10.0; 4], [30.0; 4]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE, BASE, BASE + 1]);
    }

    #[test]
    fn bounds_reject_or_clip_the_segment() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        // y in [20, 30] while the segment's y runs 0..10: nothing.
        program(&mut e, m.code, &[0x27, 20, 30, 0x30]);
        set_segment(&mut e, &m, [0.0; 4], [0.0, 10.0, 0.0, 0.0]);
        run(&mut e, &m, m.code, 0);
        assert!(keys(&m).is_empty());
        // y in [4, 12], rising 0..16: clipped to 4..12.
        program(&mut e, m.code, &[0x27, 4, 12, 0x30]);
        set_segment(&mut e, &m, [0.0; 4], [16.0; 4]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(lanes(&e, m.segment), [4.0; 4]);
        assert_eq!(lanes(&e, m.segment + 0x10), [12.0; 4]);
        // Falling 16..0: clipped to 12..4.
        set_segment(&mut e, &m, [16.0; 4], [0.0; 4]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(lanes(&e, m.segment), [12.0; 4]);
        assert_eq!(lanes(&e, m.segment + 0x10), [4.0; 4]);
        assert_eq!(keys(&m), vec![BASE, BASE]);
    }

    #[test]
    fn rescale_feeds_the_wide_bounds() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        store_lanes(&mut e, m.state, [0.5, 0.0, 0.0, 0.0]);
        e.mem.set_u32(m.state + 0x10, 5);
        e.mem.set_f32(m.state + 0x14, 0.5);
        // Rescale by 4 around (1, 2, 3): the copy's offset x is 4 * 1.5 = 6, its factor 2.
        // Then x in [10 * 2 - 6, 18 * 2 - 6] = [14, 30], base + 7, terminal 0.
        program(
            &mut e,
            m.code,
            &[0x02, 1, 2, 3, 0x29, 0, 0, 10, 0, 0, 18, 0x0a, 0, 7, 0x30],
        );
        set_segment(&mut e, &m, [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(keys(&m), vec![BASE + 7]);
        assert_eq!(lanes(&e, m.segment), [16.0, 16.0, 16.0, 32.0]);
        // The end crossed x = 30 at t = 0.875.
        assert_eq!(lanes(&e, m.segment + 0x10), [30.0, 30.0, 30.0, 46.0]);
        // The caller's state is unchanged.
        assert_eq!(lanes(&e, m.state), [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(m.state + 0x10), 5);
        assert_eq!(e.mem.f32(m.state + 0x14), 0.5);
    }

    #[test]
    fn terminal_with_output_and_filter() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(m.this + 0x58, output);
        // The double answers with the byte at output + 0x44 and a fraction of 0.5.
        let seen: Log = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CAST_RAY, move |e, a| {
            log.borrow_mut()
                .push(vec![a[0], a[2], a[3], e.mem.u32(a[3] + 0x40)]);
            let answer = e.mem.u8(a[3] + 0x44);
            e.mem.set_u8(a[1], answer);
            e.mem.set_f32(a[3] + 0x10, 0.5);
            Ret::default()
        });
        program(&mut e, m.code, &[0x33]);
        run(&mut e, &m, m.code, 0);
        assert_eq!(seen.borrow()[0], vec![m.child, m.this + 0x20, output, 1]);
        assert_eq!(e.mem.u8(m.this + 0x50), 0);
        assert_eq!(e.mem.f32(m.this + 0x54), 1.0);
        assert_eq!(e.mem.u32(output + 0x40), 0);
        assert_eq!(e.mem.u32(output + 0x20), 0);
        // A hit: the hit byte, the fraction and the key at depth 0.
        e.mem.set_u8(output + 0x44, 1);
        run(&mut e, &m, m.code, 0);
        assert_eq!(e.mem.u8(m.this + 0x50), 1);
        assert_eq!(e.mem.f32(m.this + 0x54), 0.5);
        assert_eq!(e.mem.u32(output + 0x40), 0);
        assert_eq!(e.mem.u32(output + 0x20), BASE + 3);
        assert!(m.casts.borrow().is_empty());
        // A filter that refuses: no cast.
        let filter = e.mem.alloc(8);
        let answer = e.mem.alloc(4);
        e.mem.set_u8(answer, 0);
        e.put_vtable(FILTER_VTABLE, &[FILTER_ENABLED]);
        e.mem.set_u32(filter, FILTER_VTABLE);
        e.mem.set_u32(filter + 4, answer);
        e.mem.set_u32(m.this + 0x44, filter);
        let asked: Log = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        e.register_double(FILTER_ENABLED, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            }
        });
        run(&mut e, &m, m.code, 0);
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(
            asked.borrow()[0][2..],
            [m.this + 0x20, m.child, m.container, BASE + 3]
        );
        // A filter that accepts: the cast runs.
        e.mem.set_u8(answer, 1);
        run(&mut e, &m, m.code, 0);
        assert_eq!(seen.borrow().len(), 3);
    }

    #[test]
    #[should_panic(expected = "INT3")]
    fn unknown_command_is_reported() {
        let mut e = Engine::new();
        let m = machine(&mut e);
        let reporter = e.mem.alloc(4);
        let mut slots = vec![0; 3];
        slots.push(REPORT);
        e.put_vtable(REPORTER_VTABLE, &slots);
        e.mem.set_u32(reporter, REPORTER_VTABLE);
        e.map(ERROR_OBJECT, 4);
        e.mem.set_u32(ERROR_OBJECT, reporter);
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITE, |_, a| {
            assert_eq!(a[1], UNKNOWN_COMMAND_TEXT);
            Ret::default()
        });
        e.register(STREAM_DESTRUCT, |_, _| Ret::default());
        e.register(REPORT, |_, a| {
            assert_eq!(
                [a[1], a[2], a[4], a[5]],
                [
                    3,
                    UNKNOWN_COMMAND_ID,
                    SOURCE_FILE_TEXT,
                    UNKNOWN_COMMAND_LINE
                ]
            );
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        program(&mut e, m.code, &[0x08]);
        run(&mut e, &m, m.code, 0);
    }
}

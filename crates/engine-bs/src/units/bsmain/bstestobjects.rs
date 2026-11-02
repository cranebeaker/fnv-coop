//! `BSMain/bstestobjects.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton (`011f6238`) and its `Allocate` (`00aa3e40`, `thiscall`,
/// the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00439ef0` (unnamed; `thiscall` on a point: the result's address and another point;
/// answers the result): what `CreateBox` gets its first direction from (`end`, `start`).
const POINT_OPERATION_00439EF0: u32 = 0x0043_9ef0;
/// `00439e90` (unnamed; `thiscall` on a point: the result's address and another point;
/// answers the result): what makes the last corner from its sum and the lower offset.
const POINT_OPERATION_00439E90: u32 = 0x0043_9e90;
/// `004b3800` (unnamed; `thiscall` on a point: the result's address and another point):
/// what the direction is turned with the vector (0, 0, 1).
const POINT_OPERATION_004B3800: u32 = 0x004b_3800;
/// `004a0c10` (unnamed; `thiscall` on the point, no stack words): run on the turned
/// direction after its z is cleared.
const POINT_OPERATION_004A0C10: u32 = 0x004a_0c10;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element
/// size, count, constructor) and the colour constructor it is given (`004a7800`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;
/// `memcpy` (`00ec44d0`, `cdecl`) and the box's 0x48-byte triangle list it copies
/// (`010c2c38`, twelve triangles of three `u16`s).
const MEMCPY: u32 = 0x00ec_44d0;
const BOX_TRIANGLES: u32 = 0x010c_2c38;
/// `NiNew` (`00aa13e0`, `cdecl`, the size) and the `NiTriShape` constructor `00a74410`
/// (`thiscall`, nine words: vertex count, vertices, normals, colours, texture sets, texture
/// set count, data flags, triangle count, triangles).
const NI_NEW: u32 = 0x00aa_13e0;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// The new shape's vtable slot `CreateBox` calls last (+0xE8, no stack words).
const SHAPE_SLOT_E8: u32 = 0xe8;

/// A vertex as the x87 code makes it: `sum = offset + base` per component (`FLD dword`
/// offset, `FADD dword` base, `FSTP dword`), then `z + sum` (`FLD dword` z, `FADD dword`
/// sum, `FSTP dword`), stored at `out`. `base` is read from memory component by component
/// as the code reads it.
fn corner(e: &mut Engine, out: u32, offset: [u32; 3], base: u32, z: [u32; 3]) {
    let mut sum = [0u32; 3];
    for i in 0..3u32 {
        let index = i as usize;
        let base_word = e.mem.u32(base.wrapping_add(4 * i));
        sum[index] = x87_store32(x87_add(x87_load32(offset[index]), x87_m32(base_word)));
    }
    let mut vertex = [0u32; 3];
    for i in 0..3 {
        vertex[i] = x87_store32(x87_add(x87_load32(z[i]), x87_m32(sum[i])));
    }
    for (i, word) in vertex.iter().enumerate() {
        e.mem.set_u32(out + 4 * i as u32, *word);
    }
}

/// `direction * factor` per component (`FLD dword`, `FMUL dword`, `FSTP dword`).
fn scaled(direction: [u32; 3], factor: u32) -> [u32; 3] {
    direction.map(|d| x87_store32(x87_mul(x87_load32(d), x87_m32(factor))))
}

// Translated from 00c55d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55d50 - the FS:[0] exception frame (unwind states 0 and 1 for the colour array and the shape allocation) is not translated
/// `BSTestObjects::CreateBox_ov2` (engine map; `cdecl`, eight words: two points, four
/// floats, two colours): a box `NiTriShape` along the segment from `start` to `end`.
///
/// The direction is `00439ef0` on `start` with `end`, turned by `004b3800` with (0, 0, 1),
/// its z cleared and `004a0c10` run on it; the side offsets are the direction times
/// `right` and times `-left`, the vertical offsets (0, 0, `up`) and (0, 0, `-down`). The
/// eight vertices (0x60 bytes from the memory manager; null when that fails, and the code
/// then writes through null) are, each side offset plus a point plus a vertical offset:
/// `-left`/`end`/up, `right`/`end`/up, `-left`/`end`/down, `right`/`end`/down, then the same
/// four with `start`, the last one's vertical offset added by `00439e90`. With either colour
/// given, 8 colours (0x80 bytes, constructed by the `vector constructor iterator` with
/// `004a7800`; null when the allocation fails) take the four words of `second_color` (the
/// first when only that is given) for the first four vertices and of `first_color` (the
/// second when only that is given) for the others. The triangles are a copy of the 0x48
/// bytes at `010c2c38`. The shape is `NiNew(0xC4)` constructed by `00a74410` with 8
/// vertices, no normals, the colours, no texture sets, data flags 0 and 12 triangles (null
/// when the allocation fails; the code then calls through null), then its vtable slot
/// +0xE8. Answers the shape.
#[allow(clippy::too_many_arguments)]
pub fn bs_test_objects_create_box_ov2(
    e: &mut Engine,
    start: Ptr,
    end: Ptr,
    right: f32,
    left: f32,
    up: f32,
    down: f32,
    first_color: Ptr,
    second_color: Ptr,
) -> u32 {
    let vertices = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
        .u32();
    // A non-null block is walked by the (empty) constructor loop eight times; nothing is
    // written.
    let start = start.addr();
    let end = end.addr();
    // The locals passed by address: the first direction (+0), (0, 0, 1) (+0xC), the turned
    // direction (+0x18), the lower offset (+0x24), the last sum (+0x30) and the last
    // corner (+0x3C).
    e.with_stack(0x48, |e, locals| {
        let l = locals.addr();
        e.call(POINT_OPERATION_00439EF0, &args![start, l, end]);
        e.mem.set_u32(l + 0xc, x87_store32(0.0));
        e.mem.set_u32(l + 0x10, x87_store32(0.0));
        e.mem.set_u32(l + 0x14, x87_store32(1.0));
        e.call(POINT_OPERATION_004B3800, &args![l, l + 0x18, l + 0xc]);
        e.mem.set_u32(l + 0x20, x87_store32(0.0));
        e.call(POINT_OPERATION_004A0C10, &args![l + 0x18]);
        let direction = [
            e.mem.u32(l + 0x18),
            e.mem.u32(l + 0x1c),
            e.mem.u32(l + 0x20),
        ];
        let right_offset = scaled(direction, right.to_bits());
        let minus_left = x87_store32(-x87_load32(left.to_bits()));
        let left_offset = scaled(direction, minus_left);
        let upper = [0, 0, x87_quiet_bits(up.to_bits())];
        let lower = [0, 0, x87_store32(-x87_load32(down.to_bits()))];
        e.mem.set_u32(l + 0x24, lower[0]);
        e.mem.set_u32(l + 0x28, lower[1]);
        e.mem.set_u32(l + 0x2c, lower[2]);
        corner(e, vertices, left_offset, end, upper);
        corner(e, vertices + 0xc, right_offset, end, upper);
        corner(e, vertices + 0x18, left_offset, end, lower);
        corner(e, vertices + 0x24, right_offset, end, lower);
        corner(e, vertices + 0x30, left_offset, start, upper);
        corner(e, vertices + 0x3c, right_offset, start, upper);
        corner(e, vertices + 0x48, left_offset, start, lower);
        for i in 0..3u32 {
            let index = i as usize;
            let base_word = e.mem.u32(start.wrapping_add(4 * i));
            let sum = x87_add(x87_load32(right_offset[index]), x87_m32(base_word));
            e.mem.set_u32(l + 0x30 + 4 * i, x87_store32(sum));
        }
        let last = e
            .call(
                POINT_OPERATION_00439E90,
                &args![l + 0x30, l + 0x3c, l + 0x24],
            )
            .u32();
        for i in 0..3u32 {
            let word = e.mem.u32(last.wrapping_add(4 * i));
            e.mem.set_u32(vertices + 0x54 + 4 * i, word);
        }
    });
    let mut colors = 0u32;
    let mut first_color = first_color.addr();
    let mut second_color = second_color.addr();
    if first_color != 0 || second_color != 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x80u32])
            .u32();
        colors = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, 0x10u32, 8u32, COLOR_CONSTRUCT],
            );
            block
        } else {
            0
        };
        if first_color != 0 && second_color == 0 {
            second_color = first_color;
        } else if second_color != 0 && first_color == 0 {
            first_color = second_color;
        }
        for index in 0..4u32 {
            let to = (index << 4).wrapping_add(colors);
            for word in 0..4u32 {
                let value = e.mem.u32(second_color.wrapping_add(4 * word));
                e.mem.set_u32(to.wrapping_add(4 * word), value);
            }
        }
        for index in 4..8u32 {
            let to = (index << 4).wrapping_add(colors);
            for word in 0..4u32 {
                let value = e.mem.u32(first_color.wrapping_add(4 * word));
                e.mem.set_u32(to.wrapping_add(4 * word), value);
            }
        }
    }
    let triangles = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x48u32])
        .u32();
    e.call(MEMCPY, &args![triangles, BOX_TRIANGLES, 0x48u32]);
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            TRI_SHAPE_CONSTRUCT,
            &args![block, 8u32, vertices, 0u32, colors, 0u32, 0u32, 0u32, 0xcu32, triangles],
        )
        .u32()
    } else {
        0
    };
    e.vcall(shape, SHAPE_SLOT_E8, &args![]);
    shape
}

/// `CreateBox`'s unit-box corner table (`012031f0`, eight points) and its guard
/// (`01203250`, bit 0), filled from the floats 0.5 (`01016248`) and -0.5 (`0102295c`).
const BOX_CORNERS: u32 = 0x0120_31f0;
const BOX_CORNERS_GUARD: u32 = 0x0120_3250;
const HALF: u32 = 0x0101_6248;
const MINUS_HALF: u32 = 0x0102_295c;
/// Which corner coordinates are -0.5, in the table's order.
const BOX_CORNER_NEGATIVE: [bool; 24] = [
    false, false, false, false, false, true, false, true, false, false, true, true, true, false,
    false, true, false, true, true, true, false, true, true, true,
];
/// `NiLines::NiLines` (`00a746e0`, `thiscall`, seven words: vertex count, vertices,
/// colours, texture sets, texture set count, data flags, connection flags) and
/// `NiTriStrips::NiTriStrips` (`00a71c40`, `thiscall`, eleven words: vertex count,
/// vertices, normals, colours, texture sets, texture set count, data flags, triangle count,
/// strip count, strip lengths, indices).
const LINES_CONSTRUCT: u32 = 0x00a7_46e0;
const TRI_STRIPS_CONSTRUCT: u32 = 0x00a7_1c40;
/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, `(child count)`) and its vtable slot
/// that attaches a child (+0xDC, `(child, 1)`).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_ATTACH_CHILD: u32 = 0xdc;
/// The fixed-string constructor (`00a5b690`, `cdecl`, `(text)`), its empty value
/// (`0109b220`), `NiObjectNET::SetName` (`00a5b950`, `thiscall`, `(&name)`) and the
/// imported `InterlockedDecrement`.
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
const SET_NAME: u32 = 0x00a5_b950;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiPoint3::ZERO` (`011f426c`) and the identity matrix (`011a9448`, nine words) the line
/// objects take as their local translation (+0x58) and rotation (+0x34).
const POINT_ZERO: u32 = 0x011f_426c;
const IDENTITY_MATRIX: u32 = 0x011a_9448;
const LOCAL_ROTATE: u32 = 0x34;
const LOCAL_TRANSLATE: u32 = 0x58;
/// The default line colour (`012032f8`) and the three axis colours of the coordinate jack
/// (`012032f8`, `01203308`, `01203318`; four words each).
const DEFAULT_COLOR: u32 = 0x0120_32f8;
const JACK_COLORS: [u32; 3] = [0x0120_32f8, 0x0120_3308, 0x0120_3318];
/// The CRT `cos` (`00ec9f30`) and `sin` (`00eca060`), as other units name them (a
/// `double` argument, the result in `ST0`), and `fabs` (`00ec6cde`).
const COS: u32 = 0x00ec_9f30;
const SIN: u32 = 0x00ec_a060;
const FABS: u32 = 0x00ec_6cde;
/// `NiPoint3::UnitCross` (`0053d1a0`, `thiscall`, `(&out, &other)`), the length
/// (`00457990`, a float in `ST0`) and the three axes `Perpendicular` crosses with
/// (`011a946c`, `011a9478`, `011a9484`).
const UNIT_CROSS: u32 = 0x0053_d1a0;
const POINT_LENGTH: u32 = 0x0045_7990;
const AXIS_011A946C: u32 = 0x011a_946c;
const AXIS_011A9478: u32 = 0x011a_9478;
const AXIS_011A9484: u32 = 0x011a_9484;
/// `pi` (`010c2c34`, a float), the doubles 2.0 (`01011590`), 2*pi (`0101ff48`), 6.0
/// (`01034208`), 8.0 (`010240b0`) and 0.15 (`01070c80`), and the float 0.75 (`01016264`).
const PI: u32 = 0x010c_2c34;
const TWO: u32 = 0x0101_1590;
const TWO_PI: u32 = 0x0101_ff48;
const SIX: u32 = 0x0103_4208;
const EIGHT: u32 = 0x0102_40b0;
const ARROW_WIDTH: u32 = 0x0107_0c80;
const ARROW_BACK: u32 = 0x0101_6264;
/// The object names: `"BSTestObjects Coordinate Jack"`, `"BSTestObjects Arrow"`, the
/// cylinder's, the torus's and `"BSTestObjects DirArrow"` (`010c2cc4`, `010c2ce4`,
/// `010c2cac`, `010c2c98`, `010c2cf8`).
const JACK_NAME: u32 = 0x010c_2cc4;
const ARROW_NAME: u32 = 0x010c_2ce4;
const CYLINDER_NAME: u32 = 0x010c_2cac;
const TORUS_NAME: u32 = 0x010c_2c98;
const DIR_ARROW_NAME: u32 = 0x010c_2cf8;
/// The sphere's name (`010c2c80`), the float -1.0 its bottom normal takes (`01012054`), the
/// double 4.0 (`0101db80`) and the 3D arrow head's 0x30-byte triangle list (`010c2d10`).
const SPHERE_NAME: u32 = 0x010c_2c80;
const MINUS_ONE: u32 = 0x0101_2054;
const FOUR: u32 = 0x0101_db80;
const ARROW_TRIANGLES: u32 = 0x010c_2d10;
/// `NiPoint3::UnitizeGetLength` (`00457910`, `thiscall`, a float in `ST0`), the block
/// allocation `00aa1070` (`cdecl`, the size) and the double 0.0 (`01012060`).
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
const NI_MALLOC: u32 = 0x00aa_1070;
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `count * element` as the code sizes an array: all ones when the product overflows
/// (`MUL`, `SETO`, `NEG`, `OR`).
fn array_bytes(count: u32, element: u32) -> u32 {
    count.checked_mul(element).unwrap_or(u32::MAX)
}

/// A block from the memory manager.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// The `NiColorA` constructor loop over `count` colours of a non-null block (each colour
/// set to 0.0 four times through `FLDZ`/`FSTP`); a signed count, as the code's `JS`.
fn zero_colors(e: &mut Engine, block: u32, count: u32) {
    let mut left = count as i32;
    let mut at = block;
    loop {
        left = left.wrapping_sub(1);
        if left < 0 {
            break;
        }
        for k in 0..4u32 {
            e.mem.set_u32(at.wrapping_add(4 * k), 0);
        }
        at = at.wrapping_add(0x10);
    }
}

/// Copies `count` words.
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for k in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * k));
        e.mem.set_u32(to.wrapping_add(4 * k), word);
    }
}

/// A CRT math call on a float: `FLD dword` and `FSTP qword` as the argument, the result
/// stored as a float (`FSTP dword`) and copied through the x87 stack once more.
fn crt_float(e: &mut Engine, function: u32, bits: u32) -> u32 {
    let result = e.call(function, &args![x87_load32(bits)]).f64();
    x87_quiet_bits(x87_store32(result))
}

/// `FLD dword` a, `FMUL dword` b, `FSTP dword`.
fn mul32(a: u32, b: u32) -> u32 {
    x87_store32(x87_mul(x87_load32(a), x87_m32(b)))
}

/// `FLD dword` a, `FADD dword` b, `FSTP dword`.
fn add32(a: u32, b: u32) -> u32 {
    x87_store32(x87_add(x87_load32(a), x87_m32(b)))
}

/// `FLD dword` a, `FSUB dword` b, `FSTP dword`.
fn sub32(a: u32, b: u32) -> u32 {
    x87_store32(x87_sub(x87_load32(a), x87_m32(b)))
}

/// `FLD dword` a, `FCHS`, `FSTP dword`.
fn negated32(a: u32) -> u32 {
    x87_store32(-x87_load32(a))
}

/// Gives the object its name: a fixed string of `text` (the code tests the constant
/// text's address first; the branch that takes the empty string never runs), then
/// `NiObjectNET::SetName` with it, then the string's reference released unless it is the
/// empty one.
fn name_object(e: &mut Engine, object: u32, text: u32) {
    e.with_stack(4, |e, name| {
        let handle = if std::hint::black_box(text) != 0 {
            e.call(FIXED_STRING_CREATE, &args![text]).u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(name.addr(), handle);
        e.call(SET_NAME, &args![object, name]);
        let handle = e.mem.u32(name.addr());
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
}

/// A line object's local transform: `NiPoint3::ZERO` and the identity matrix.
fn reset_transform(e: &mut Engine, object: u32) {
    copy_words(e, object.wrapping_add(LOCAL_TRANSLATE), POINT_ZERO, 3);
    copy_words(e, object.wrapping_add(LOCAL_ROTATE), IDENTITY_MATRIX, 9);
}

/// `NiNew(0xC4)` and `NiLines::NiLines`; null when the allocation fails.
fn new_lines(e: &mut Engine, count: u32, vertices: u32, colors: u32, flags: u32) -> u32 {
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        LINES_CONSTRUCT,
        &args![block, count, vertices, colors, 0u32, 0u32, 0u32, flags],
    )
    .u32()
}

/// Writes three words (a point).
fn set_point(e: &mut Engine, at: u32, point: [u32; 3]) {
    for (k, word) in (0u32..).zip(point) {
        e.mem.set_u32(at.wrapping_add(4 * k), word);
    }
}

/// Reads three words (a point).
fn get_point(e: &Engine, at: u32) -> [u32; 3] {
    [
        e.mem.u32(at),
        e.mem.u32(at.wrapping_add(4)),
        e.mem.u32(at.wrapping_add(8)),
    ]
}

// Translated from 00c55920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55920 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f685, state variable) is not translated
/// `BSTestObjects::CreateBox` (Xbox PDB, engine map; `cdecl`, `(half_extents, colour)`): a
/// box `NiTriShape` around the origin. On first use the unit-box corner table (`012031f0`,
/// guard bit at `01203250`) is filled with plus and minus 0.5. The eight vertices (0x60
/// bytes from the memory manager; null when that fails, and the code then writes through
/// null) are the corners times `half_extents`, component by component. With a colour, 8
/// colours (0x80 bytes, zeroed when given) all take it. The triangles are a copy of the
/// 0x48 bytes at `010c2c38`; the shape is `NiNew(0xC4)` constructed with 8 vertices, no
/// normals, the colours, no texture sets and 12 triangles (null when the allocation fails;
/// the code then calls through null), then its vtable slot +0xE8. Answers the shape.
pub fn bs_test_objects_create_box(e: &mut Engine, half_extents: u32, color: u32) -> u32 {
    let guard = e.global::<u32>(BOX_CORNERS_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(BOX_CORNERS_GUARD, guard | 1);
        for (k, negative) in (0u32..).zip(BOX_CORNER_NEGATIVE) {
            let source = if negative { MINUS_HALF } else { HALF };
            let value = x87_quiet_bits(e.mem.u32(source));
            e.mem.set_u32(BOX_CORNERS + 4 * k, value);
        }
    }
    let vertices = allocate(e, 0x60);
    for index in 0..8u32 {
        for k in 0..3u32 {
            let corner = e.mem.u32(BOX_CORNERS + 0xc * index + 4 * k);
            let extent = e.mem.u32(half_extents.wrapping_add(4 * k));
            let value = x87_quiet_bits(mul32(corner, extent));
            e.mem
                .set_u32(vertices.wrapping_add(0xc * index + 4 * k), value);
        }
    }
    let colors = if color != 0 {
        let block = allocate(e, 0x80);
        if block != 0 {
            zero_colors(e, block, 8);
        }
        block
    } else {
        0
    };
    if colors != 0 {
        for index in 0..8u32 {
            copy_words(e, colors.wrapping_add(index << 4), color, 4);
        }
    }
    let triangles = allocate(e, 0x48);
    e.call(MEMCPY, &args![triangles, BOX_TRIANGLES, 0x48u32]);
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            TRI_SHAPE_CONSTRUCT,
            &args![block, 8u32, vertices, 0u32, colors, 0u32, 0u32, 0u32, 0xcu32, triangles],
        )
        .u32()
    } else {
        0
    };
    e.vcall(shape, SHAPE_SLOT_E8, &args![]);
    shape
}

// Translated from 00c55cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, `(point, size, colour)`): `CreateBox_ov2` from the
/// point moved by `-size` in y to the point moved by `+size` in y, with all four widths
/// `size` and `colour` for both colours. Answers its shape.
pub fn fn_00c55cd0(e: &mut Engine, point: u32, size: f32, color: u32) -> u32 {
    let size = size.to_bits();
    e.with_stack(0x18, |e, locals| {
        let start = locals.addr().wrapping_add(0xc);
        let end = locals.addr();
        copy_words(e, start, point, 3);
        let y = e.mem.u32(start.wrapping_add(4));
        e.mem.set_u32(start.wrapping_add(4), sub32(y, size));
        copy_words(e, end, point, 3);
        let y = e.mem.u32(end.wrapping_add(4));
        e.mem.set_u32(end.wrapping_add(4), add32(y, size));
        let width = f32::from_bits(x87_quiet_bits(size));
        bs_test_objects_create_box_ov2(
            e,
            Ptr::new(start),
            Ptr::new(end),
            width,
            width,
            width,
            width,
            Ptr::new(color),
            Ptr::new(color),
        )
    })
}

/// `fabs` of a float as the code calls it (`FLD dword`, `FSTP qword` argument), its result
/// stored as a float and copied once more.
fn absolute(e: &mut Engine, bits: u32) -> u32 {
    crt_float(e, FABS, bits)
}

// Translated from 00c5a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::Perpendicular` (Xbox PDB, engine map; `thiscall`, `(&out)`): `UnitCross` of
/// the point with an axis chosen by its largest component: when |y| is above |x|
/// (ordered), the axis `011a9484` when |x| is above |z| else `011a946c`; otherwise
/// `011a9484` when |y| is above |z| else `011a9478` (each absolute value through `fabs`,
/// kept as a float). Returns `out`.
pub fn ni_point3_perpendicular(e: &mut Engine, this: u32, out: u32) -> u32 {
    let x = x87_quiet_bits(e.mem.u32(this));
    let ax = absolute(e, x);
    let y = x87_quiet_bits(e.mem.u32(this.wrapping_add(4)));
    let ay = absolute(e, y);
    let axis = if x87_load32(ay) > x87_load32(ax) {
        let z = x87_quiet_bits(e.mem.u32(this.wrapping_add(8)));
        let az = absolute(e, z);
        let x = x87_quiet_bits(e.mem.u32(this));
        let ax = absolute(e, x);
        if x87_load32(ax) > x87_load32(az) {
            AXIS_011A9484
        } else {
            AXIS_011A946C
        }
    } else {
        let z = x87_quiet_bits(e.mem.u32(this.wrapping_add(8)));
        let az = absolute(e, z);
        let y = x87_quiet_bits(e.mem.u32(this.wrapping_add(4)));
        let ay = absolute(e, y);
        if x87_load32(ay) > x87_load32(az) {
            AXIS_011A9484
        } else {
            AXIS_011A9478
        }
    };
    e.call(UNIT_CROSS, &args![this, out, axis]);
    out
}

// Translated from 00c58d30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c58d30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f7d7, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, `(bottom_radius, top_radius, height, segments,
/// rings, colour)`): an `NiNode` (`NiNew(0xAC)`, constructor `00a5ecb0` with 0; null when
/// the allocation fails) holding three cylinders (`CreateCylinder`): the one asked for, a
/// flat disc of `bottom_radius` (radii 0, height 0, one ring) turned half a turn about x
/// (its local rotation `[1 0 0; 0 cos sin; 0 -sin cos]` of the float pi through `FSINCOS`),
/// and the same disc moved to `(0, 0, height)`. Each is attached with the node's vtable
/// slot +0xDC. Answers the node.
pub fn fn_00c58d30(
    e: &mut Engine,
    bottom_radius: f32,
    top_radius: f32,
    height: f32,
    segments: u16,
    rings: u16,
    color: u32,
) -> u32 {
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    let quiet = |value: f32| f32::from_bits(x87_quiet_bits(value.to_bits()));
    let body = bs_test_objects_create_cylinder(
        e,
        quiet(bottom_radius),
        quiet(top_radius),
        quiet(height),
        segments,
        rings,
        color,
    );
    let lower =
        bs_test_objects_create_cylinder(e, quiet(bottom_radius), 0.0, 0.0, segments, 1, color);
    let angle = x87_quiet_bits(x87_store32(x87_load32(e.mem.u32(PI))));
    let (sine, cosine) = x87_fsincos(X87Ext::load32(angle));
    let cosine = x87_quiet_bits(cosine.store32());
    let sine = x87_quiet_bits(sine.store32());
    let one = x87_store32(1.0);
    let rotation = [
        one,
        0,
        0,
        0,
        x87_quiet_bits(cosine),
        x87_quiet_bits(sine),
        0,
        negated32(sine),
        x87_quiet_bits(cosine),
    ];
    for (k, word) in (0u32..).zip(rotation) {
        e.mem
            .set_u32(lower.wrapping_add(LOCAL_ROTATE + 4 * k), word);
    }
    let upper =
        bs_test_objects_create_cylinder(e, quiet(bottom_radius), 0.0, 0.0, segments, 1, color);
    let offset = [0, 0, x87_quiet_bits(height.to_bits())];
    set_point(e, upper.wrapping_add(LOCAL_TRANSLATE), offset);
    for child in [body, lower, upper] {
        e.vcall(node, NODE_ATTACH_CHILD, &args![child, 1u32]);
    }
    node
}

// Translated from 00c58f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c58f00 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f804, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, `(radius, count, colour)`): a circle of `count` points
/// in the xy plane as `NiLines`. The vertices (`count * 12` bytes), the connection flags
/// (`count` bytes) and, with a colour, the colours (`count * 16` bytes, zeroed when given)
/// come from the memory manager (the sizes all ones on overflow). The step is 2*pi (the
/// double at `0101ff48`) divided by the count (stored as a float); point `i` is
/// `(radius * sin(i * step), radius * cos(i * step), 0)` (the angle and each result
/// stored as floats), takes the colour and the flag 1. The lines are `NiNew(0xC4)` with
/// `NiLines::NiLines(count, vertices, colours, 0, 1, 0, flags)`; null when the allocation
/// fails. Answers them.
pub fn fn_00c58f00(e: &mut Engine, radius: f32, count: u16, color: u32) -> u32 {
    let count = u32::from(count);
    let vertices = allocate(e, array_bytes(count, 0xc));
    let flags = allocate(e, count);
    let colors = if color != 0 {
        let block = allocate(e, array_bytes(count, 0x10));
        if block != 0 {
            zero_colors(e, block, count);
        }
        block
    } else {
        0
    };
    let step = x87_store32(x87_div(e.mem.f64(TWO_PI), f64::from(count)));
    let radius = radius.to_bits();
    let mut index = 0u32;
    while index < count {
        if colors != 0 {
            copy_words(e, colors.wrapping_add(index << 4), color, 4);
        }
        e.mem.set_u8(flags.wrapping_add(index), 1);
        let angle = x87_store32(x87_mul(f64::from(index), x87_m32(step)));
        let cosine = crt_float(e, COS, angle);
        let angle = x87_store32(x87_mul(f64::from(index), x87_m32(step)));
        let sine = crt_float(e, SIN, angle);
        let point = [mul32(radius, sine), mul32(radius, cosine), 0];
        set_point(e, vertices.wrapping_add(index.wrapping_mul(0xc)), point);
        index += 1;
    }
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        LINES_CONSTRUCT,
        &args![block, count, vertices, colors, 0u32, 1u32, 0u32, flags],
    )
    .u32()
}

// Translated from 00c59200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c59200 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f83f, state variable) is not translated
/// `BSTestObjects::CreateCoordinateJack` (Xbox PDB, engine map; `cdecl`, `(size)`): three
/// lines from the origin along x, y and z of length `size` as `NiLines`: six vertices
/// (0x48 bytes), six colours (0x60 bytes, zeroed when given; each line in its axis colour
/// from `012032f8`, `01203308`, `01203318`) and the connection flags `1 0 1 0 1 0`, all
/// from the memory manager (the code writes through a null block when one fails). The
/// object (`NiNew(0xC4)`, `NiLines::NiLines(6, ...)`; null when that fails, written through
/// anyway) gets the zero translation, the identity rotation and the name
/// `"BSTestObjects Coordinate Jack"`. Answers it.
pub fn bs_test_objects_create_coordinate_jack(e: &mut Engine, size: f32) -> u32 {
    let vertices = allocate(e, 0x48);
    let size = x87_quiet_bits(size.to_bits());
    let points = [
        [0, 0, 0],
        [size, 0, 0],
        [0, 0, 0],
        [0, size, 0],
        [0, 0, 0],
        [0, 0, size],
    ];
    for (index, point) in (0u32..).zip(points) {
        set_point(e, vertices.wrapping_add(0xc * index), point);
    }
    let colors = allocate(e, 0x60);
    if colors != 0 {
        zero_colors(e, colors, 6);
    }
    for (index, axis) in (0u32..).zip(JACK_COLORS) {
        copy_words(e, colors.wrapping_add(0x20 * index), axis, 4);
        copy_words(e, colors.wrapping_add(0x20 * index + 0x10), axis, 4);
    }
    let flags = allocate(e, 6);
    for (k, flag) in (0u32..).zip([1u8, 0, 1, 0, 1, 0]) {
        e.mem.set_u8(flags.wrapping_add(k), flag);
    }
    let lines = new_lines(e, 6, vertices, colors, flags);
    reset_transform(e, lines);
    name_object(e, lines, JACK_NAME);
    lines
}

// Translated from 00c59660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c59660 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f877, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, `(length, colour)`): an arrow along z as `NiLines`:
/// the shaft from the origin to `(0, 0, length)` and two head lines from the tip back to
/// `(+-tip, 0, length - tip)`, `tip` being `length / 6.0` (the double at `01034208`, stored
/// as a float). Five vertices (0x3C bytes), six colours (0x60 bytes, zeroed when given, the
/// first five the colour or `012032f8` without one) and the connection flags `1 1 0 1 0`,
/// all from the memory manager (written through null when one fails). The object
/// (`NiNew(0xC4)`, `NiLines::NiLines(5, ...)`) gets the zero translation, the identity
/// rotation and the name `"BSTestObjects Arrow"`. Answers it.
pub fn fn_00c59660(e: &mut Engine, length: f32, color: u32) -> u32 {
    let length = length.to_bits();
    let tip = x87_store32(x87_div(x87_load32(length), e.mem.f64(SIX)));
    let vertices = allocate(e, 0x3c);
    let back = x87_store32(x87_sub(x87_m32(length), x87_load32(tip)));
    let points = [
        [0, 0, 0],
        [0, 0, x87_quiet_bits(length)],
        [x87_quiet_bits(tip), 0, back],
        [0, 0, x87_quiet_bits(length)],
        [negated32(tip), 0, back],
    ];
    for (index, point) in (0u32..).zip(points) {
        set_point(e, vertices.wrapping_add(0xc * index), point);
    }
    let color = if color == 0 { DEFAULT_COLOR } else { color };
    let colors = allocate(e, 0x60);
    if colors != 0 {
        zero_colors(e, colors, 6);
    }
    for index in 0..5u32 {
        copy_words(e, colors.wrapping_add(0x10 * index), color, 4);
    }
    let flags = allocate(e, 6);
    for (k, flag) in (0u32..).zip([1u8, 1, 0, 1, 0]) {
        e.mem.set_u8(flags.wrapping_add(k), flag);
    }
    let lines = new_lines(e, 5, vertices, colors, flags);
    reset_transform(e, lines);
    name_object(e, lines, ARROW_NAME);
    lines
}

/// The strip arrays `CreateCylinder` and the torus allocate: vertices, normals, colours
/// (with a colour; zeroed when given), and, for the cylinder, texture coordinates.
fn strip_arrays(e: &mut Engine, total: u32, color: u32) -> (u32, u32, u32) {
    let vertices = allocate(e, array_bytes(total, 0xc));
    let normals = allocate(e, array_bytes(total, 0xc));
    let colors = if color != 0 {
        let block = allocate(e, array_bytes(total, 0x10));
        if block != 0 {
            zero_colors(e, block, total);
        }
        block
    } else {
        0
    };
    (vertices, normals, colors)
}

/// `NiNew(0xC4)` and `NiTriStrips::NiTriStrips`; null when the allocation fails.
#[allow(clippy::too_many_arguments)]
fn new_strips(
    e: &mut Engine,
    count: u32,
    vertices: u32,
    normals: u32,
    colors: u32,
    texture: u32,
    texture_sets: u32,
    triangles: u32,
    strips: u32,
    lengths: u32,
    indices: u32,
) -> u32 {
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        TRI_STRIPS_CONSTRUCT,
        &args![
            block,
            count,
            vertices,
            normals,
            colors,
            texture,
            texture_sets,
            0u32,
            triangles,
            strips,
            lengths,
            indices
        ],
    )
    .u32()
}

// Translated from 00c57d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c57d70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f769, state variable) is not translated
/// Unnamed in the engine map (`cdecl`, `(tube_radius, ring_radius, rings, sides, colour)`):
/// a torus about z as `NiTriStrips`, one strip per ring of `sides * 2 + 2` vertices. The
/// steps are 2*pi (the float pi times the double 2.0) over `rings` and over `sides`. For
/// each ring (angle `a` advanced by its step first, `c = cos a`, `s = sin a`) and each of
/// `sides + 1` side angles `b` (advanced first, `cb = cos b`, `sb = sin b`, `r = tube_radius
/// * cb + ring_radius`), two vertices: `(r * c, r * -s, sb * tube_radius)` with the normal
/// `(cb * c, cb * -s, sb)` and the same with the previous ring's cosine and sine (1 and 0
/// for the first); each takes the colour and its own index. All arrays come from the memory
/// manager (sizes all ones on overflow). The strips are `NiNew(0xC4)` with the vertex
/// count, no texture sets, `rings * 2 * sides` triangles and `rings` strips (null when that
/// fails; the code then calls through null). Named (`010c2c98`), then its vtable slot
/// +0xE8. Answers them.
pub fn fn_00c57d70(
    e: &mut Engine,
    tube_radius: f32,
    ring_radius: f32,
    rings: u16,
    sides: u16,
    color: u32,
) -> u32 {
    let tube = tube_radius.to_bits();
    let ring_r = ring_radius.to_bits();
    let full_turn = x87_mul(x87_load32(e.mem.u32(PI)), e.mem.f64(TWO));
    let ring_step = x87_store32(x87_div(full_turn, f64::from(rings)));
    let full_turn = x87_mul(x87_load32(e.mem.u32(PI)), e.mem.f64(TWO));
    let side_step = x87_store32(x87_div(full_turn, f64::from(sides)));
    let per_strip = sides.wrapping_mul(2).wrapping_add(2);
    let total = u32::from(rings).wrapping_mul(u32::from(per_strip));
    let (vertices, normals, colors) = strip_arrays(e, total, color);
    let lengths = allocate(e, array_bytes(u32::from(rings), 2));
    let indices = allocate(e, array_bytes(total, 2));
    let mut index = 0u16;
    let mut strip = 0u16;
    let mut angle = 0u32;
    let mut previous_cos = x87_store32(1.0);
    let mut previous_sin = 0u32;
    let mut ring = i32::from(rings) - 1;
    while ring >= 0 {
        let a = add32(angle, ring_step);
        let c = crt_float(e, COS, a);
        let s = crt_float(e, SIN, a);
        let mut b = 0u32;
        let mut side = i32::from(sides);
        while side >= 0 {
            b = add32(b, side_step);
            let cb = crt_float(e, COS, b);
            let sb = crt_float(e, SIN, b);
            let r = x87_store32(x87_add(
                x87_mul(x87_load32(tube), x87_m32(cb)),
                x87_m32(ring_r),
            ));
            for (cosine, sine) in [(c, s), (previous_cos, previous_sin)] {
                let at = u32::from(index);
                let point = [
                    mul32(r, cosine),
                    x87_store32(x87_mul(x87_load32(r), -x87_load32(sine))),
                    mul32(sb, tube),
                ];
                set_point(e, vertices.wrapping_add(at * 0xc), point);
                let normal = [
                    mul32(cb, cosine),
                    x87_store32(x87_mul(x87_load32(cb), -x87_load32(sine))),
                    x87_quiet_bits(sb),
                ];
                set_point(e, normals.wrapping_add(at * 0xc), normal);
                if color != 0 {
                    copy_words(e, colors.wrapping_add(at << 4), color, 4);
                }
                e.mem.set_u16(indices.wrapping_add(at * 2), index);
                index = index.wrapping_add(1);
            }
            side -= 1;
        }
        e.mem
            .set_u16(lengths.wrapping_add(u32::from(strip) * 2), per_strip);
        strip = strip.wrapping_add(1);
        angle = x87_quiet_bits(a);
        previous_cos = x87_quiet_bits(c);
        previous_sin = x87_quiet_bits(s);
        ring -= 1;
    }
    let triangles = (u32::from(rings) << 1).wrapping_mul(u32::from(sides));
    let strips = new_strips(
        e,
        u32::from(index),
        vertices,
        normals,
        colors,
        0,
        0,
        triangles,
        u32::from(rings),
        lengths,
        indices,
    );
    name_object(e, strips, TORUS_NAME);
    e.vcall(strips, SHAPE_SLOT_E8, &args![]);
    strips
}

// Translated from 00c584a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c584a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f7a7, state variable) is not translated
/// `BSTestObjects::CreateCylinder` (Xbox PDB, engine map; `cdecl`, `(bottom_radius,
/// top_radius, height, segments, rings, colour)`): a cylinder along z as `NiTriStrips`, one
/// strip per ring of `segments * 2 + 2` vertices. The angle step is the float pi times the
/// double 2.0 over `segments`; per ring the radius grows by `(top - bottom) / rings`, the
/// height by `height / rings`, and the texture v falls by `1 / rings`. Each of the
/// `segments + 1` angles (the last one's sine and cosine taken as 0 and 1) gives two
/// vertices, at the ring's radius and height and at the next ring's, with the normal `(s,
/// c, (bottom - top) / height)` (times 1.0), the colour, the texture coordinates `(u, v)`
/// and `(u, v - 1 / rings)` (`u` advancing by `1 / segments`) and its own index. All
/// arrays come from the memory manager (sizes all ones on overflow). The strips are
/// `NiNew(0xC4)` with the vertex count, one texture set, `rings * 2 * segments` triangles
/// and `rings` strips (null when that fails; the code then calls through null). Named
/// (`010c2cac`), then its vtable slot +0xE8. Answers them.
pub fn bs_test_objects_create_cylinder(
    e: &mut Engine,
    bottom_radius: f32,
    top_radius: f32,
    height: f32,
    segments: u16,
    rings: u16,
    color: u32,
) -> u32 {
    let bottom = bottom_radius.to_bits();
    let top = top_radius.to_bits();
    let height = height.to_bits();
    let per_ring = segments.wrapping_mul(2).wrapping_add(2);
    let total = u32::from(rings).wrapping_mul(u32::from(per_ring));
    let vertices = allocate(e, array_bytes(total, 0xc));
    let normals = allocate(e, array_bytes(total, 0xc));
    let colors = if color != 0 {
        let block = allocate(e, array_bytes(total, 0x10));
        if block != 0 {
            zero_colors(e, block, total);
        }
        block
    } else {
        0
    };
    let texture = allocate(e, array_bytes(total, 8));
    let lengths = allocate(e, array_bytes(u32::from(rings), 2));
    let indices = allocate(e, array_bytes(total, 2));
    let segments_f = f64::from(segments);
    let rings_f = f64::from(rings);
    let full_turn = x87_mul(x87_load32(e.mem.u32(PI)), e.mem.f64(TWO));
    let angle_step = x87_store32(x87_div(full_turn, segments_f));
    let radius_step = x87_store32(x87_div(x87_sub(x87_load32(top), x87_m32(bottom)), rings_f));
    let height_step = x87_store32(x87_div(x87_m32(height), rings_f));
    let u_step = x87_store32(x87_div(1.0, segments_f));
    let v_step = x87_store32(x87_div(1.0, rings_f));
    let one = x87_store32(1.0);
    let mut v = one;
    let mut z = 0u32;
    let mut radius = x87_quiet_bits(bottom);
    let slope = x87_store32(x87_div(
        x87_sub(x87_load32(bottom), x87_m32(top)),
        x87_m32(height),
    ));
    let normal_scale = one;
    let mut index = 0u16;
    let mut ring = 0u32;
    while ring < u32::from(rings) {
        let mut u = 0u32;
        let mut segment = 0u32;
        while segment <= u32::from(segments) {
            let (sine, cosine) = if segment == u32::from(segments) {
                (0u32, one)
            } else {
                let angle = x87_store32(x87_mul(x87_load32(angle_step), f64::from(segment)));
                let sine = crt_float(e, SIN, angle);
                let angle = x87_store32(x87_mul(x87_load32(angle_step), f64::from(segment)));
                (sine, crt_float(e, COS, angle))
            };
            let normal = [
                mul32(normal_scale, sine),
                mul32(normal_scale, cosine),
                mul32(normal_scale, slope),
            ];
            let at = u32::from(index);
            set_point(
                e,
                vertices.wrapping_add(at * 0xc),
                [
                    mul32(radius, sine),
                    mul32(radius, cosine),
                    x87_quiet_bits(z),
                ],
            );
            set_point(e, normals.wrapping_add(at * 0xc), normal);
            if color != 0 {
                copy_words(e, colors.wrapping_add(at << 4), color, 4);
            }
            e.mem
                .set_u32(texture.wrapping_add(at * 8), x87_quiet_bits(u));
            e.mem
                .set_u32(texture.wrapping_add(at * 8 + 4), x87_quiet_bits(v));
            e.mem.set_u16(indices.wrapping_add(at * 2), index);
            index = index.wrapping_add(1);
            let at = u32::from(index);
            let outer = x87_add(x87_load32(radius_step), x87_m32(radius));
            let point = [
                x87_store32(x87_mul(outer, x87_m32(sine))),
                x87_store32(x87_mul(
                    x87_add(x87_load32(radius_step), x87_m32(radius)),
                    x87_m32(cosine),
                )),
                add32(height_step, z),
            ];
            set_point(e, vertices.wrapping_add(at * 0xc), point);
            set_point(e, normals.wrapping_add(at * 0xc), normal);
            if color != 0 {
                copy_words(e, colors.wrapping_add(at << 4), color, 4);
            }
            e.mem
                .set_u32(texture.wrapping_add(at * 8), x87_quiet_bits(u));
            let lower_v = x87_store32(x87_sub(x87_m32(v), x87_load32(v_step)));
            e.mem.set_u32(texture.wrapping_add(at * 8 + 4), lower_v);
            e.mem.set_u16(indices.wrapping_add(at * 2), index);
            index = index.wrapping_add(1);
            u = add32(u, u_step);
            segment += 1;
        }
        radius = add32(radius, radius_step);
        v = sub32(v, v_step);
        z = add32(z, height_step);
        e.mem.set_u16(lengths.wrapping_add(ring * 2), per_ring);
        ring += 1;
    }
    let triangles = (u32::from(rings) << 1).wrapping_mul(u32::from(segments));
    let strips = new_strips(
        e,
        u32::from(index),
        vertices,
        normals,
        colors,
        texture,
        1,
        triangles,
        u32::from(rings),
        lengths,
        indices,
    );
    name_object(e, strips, CYLINDER_NAME);
    e.vcall(strips, SHAPE_SLOT_E8, &args![]);
    strips
}

// Translated from 00c59a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c59a50 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f8b5, state variable) is not translated
/// `BSTestObjects::CreateDirArrow` (Xbox PDB, engine map; `cdecl`, `(direction, colour)`):
/// an arrow from the origin along `direction` as `NiLines`. A direction shorter than 8.0
/// (the double at `010240b0`; ordered) is scaled to that length (`8.0 / length` per
/// component) and measured again. The head's half width is the length times the double
/// 0.15 (`01070c80`) along `Perpendicular` of the direction; its back point is the
/// direction times the float 0.75 (`01016264`); the vertices are the origin, the tip, the
/// back plus the width, the tip and the back minus the width. Six colours (0x60 bytes,
/// zeroed when given): the colour (or `012032f8` without one) for the shaft, `(0, 1, 0, 1)`
/// for the head; connection flags `1 0 1 1 0`. All blocks come from the memory manager
/// (written through null when one fails). The object (`NiNew(0xC4)`,
/// `NiLines::NiLines(5, ...)`) gets the zero translation, the identity rotation and the name
/// `"BSTestObjects DirArrow"`. Answers it.
pub fn bs_test_objects_create_dir_arrow(e: &mut Engine, direction: u32, color: u32) -> u32 {
    let lines = e.with_stack(0x18, |e, locals| {
        let tip = locals.addr();
        let out = tip.wrapping_add(0xc);
        copy_words(e, tip, direction, 3);
        let mut length = x87_store32(e.call(POINT_LENGTH, &args![tip]).f64());
        let eight = e.mem.f64(EIGHT);
        // FCOMP; TEST AH,0x5; JP: only an ordered below scales.
        if x87_load32(length) < eight {
            let factor = x87_store32(x87_div(eight, x87_load32(length)));
            let scaled = get_point(e, tip).map(|c| x87_quiet_bits(mul32(c, factor)));
            set_point(e, tip, scaled);
            length = x87_store32(e.call(POINT_LENGTH, &args![tip]).f64());
        }
        let vertices = allocate(e, 0x3c);
        set_point(e, vertices, [0, 0, 0]);
        let width = x87_store32(x87_mul(x87_load32(length), e.mem.f64(ARROW_WIDTH)));
        let across = ni_point3_perpendicular(e, tip, out);
        let side = get_point(e, across).map(|c| x87_quiet_bits(mul32(c, width)));
        let back_factor = e.mem.u32(ARROW_BACK);
        let back = get_point(e, tip).map(|c| x87_quiet_bits(mul32(c, back_factor)));
        let left = [0, 1, 2]
            .map(|k| x87_quiet_bits(x87_store32(x87_sub(x87_m32(back[k]), x87_load32(side[k])))));
        let right = [0, 1, 2].map(|k| x87_quiet_bits(add32(side[k], back[k])));
        let point = get_point(e, tip);
        set_point(e, vertices.wrapping_add(0xc), point);
        set_point(e, vertices.wrapping_add(0x18), right);
        set_point(e, vertices.wrapping_add(0x24), point);
        set_point(e, vertices.wrapping_add(0x30), left);
        let color = if color == 0 { DEFAULT_COLOR } else { color };
        let colors = allocate(e, 0x60);
        if colors != 0 {
            zero_colors(e, colors, 6);
        }
        copy_words(e, colors, color, 4);
        copy_words(e, colors.wrapping_add(0x10), color, 4);
        let green = [0, one_bits(), 0, one_bits()];
        for index in 2..5u32 {
            for (k, word) in (0u32..).zip(green) {
                e.mem
                    .set_u32(colors.wrapping_add(0x10 * index + 4 * k), word);
            }
        }
        let flags = allocate(e, 6);
        for (k, flag) in (0u32..).zip([1u8, 0, 1, 1, 0]) {
            e.mem.set_u8(flags.wrapping_add(k), flag);
        }
        new_lines(e, 5, vertices, colors, flags)
    });
    reset_transform(e, lines);
    name_object(e, lines, DIR_ARROW_NAME);
    lines
}

/// 1.0 as `FLD1`/`FSTP dword` stores it.
fn one_bits() -> u32 {
    x87_store32(1.0)
}

// Translated from 00c5a0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5a0f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f8fa, state variable) is not translated
/// `BSTestObjects::Create3DArrow` (Xbox PDB, engine map; `cdecl`, `(start, end, width,
/// height, first_colour, second_colour)`): an `NiNode` (`NiNew(0xAC)`, `00a5ecb0` with 0;
/// null when that fails) holding an arrow from `start` to `end`. The direction is `end -
/// start` (`00439ef0`) made a unit vector (`NiPoint3::UnitizeGetLength`, its length kept);
/// the head is `width * 6.0` long when that is below the length (ordered), the shaft the
/// rest; otherwise the head is the whole length and there is no shaft. The neck is `start
/// + direction * shaft` (`00439e90`). A shaft above 0.0 (ordered) is a box from `start` to
/// the neck (`CreateBox_ov2` with `width`, `width`, `height`, 0.0 and both colours)
/// attached to the node. The head is an `NiTriShape` of six vertices (0x48 bytes from the
/// memory manager; written through null when that fails): with `side` the direction
/// crossed with (0, 0, 1) (`004b3800`) times `width * 4.0` (the double at `0101db80`) and
/// `lift` (0, 0, `height`): `start + lift`, `neck + side + lift`, `end`, `neck + side`,
/// `neck - side + lift`, `neck - side`. With either colour (the second, or the first when
/// only it is given), six colours (0x60 bytes, zeroed when given) take it. The triangles
/// are a copy of the 0x30 bytes at `010c2d10` (`00aa1070`); the shape is `NiNew(0xC4)` with
/// 8 triangles (null when that fails; the code then calls through null), its vtable slot
/// +0xE8 runs and it is attached to the node. Answers the node.
pub fn bs_test_objects_create_3d_arrow(
    e: &mut Engine,
    start: u32,
    end: u32,
    width: f32,
    height: f32,
    first_color: u32,
    second_color: u32,
) -> u32 {
    let width = width.to_bits();
    let height = height.to_bits();
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    // Locals: the direction (+0), the neck (+0xC), the offset (+0x18), (0, 0, 1) (+0x24)
    // and the side (+0x30).
    e.with_stack(0x3c, |e, locals| {
        let direction = locals.addr();
        let neck = direction.wrapping_add(0xc);
        let offset = direction.wrapping_add(0x18);
        let up = direction.wrapping_add(0x24);
        let side = direction.wrapping_add(0x30);
        e.call(POINT_OPERATION_00439EF0, &args![end, direction, start]);
        let length = x87_store32(e.call(UNITIZE_GET_LENGTH, &args![direction]).f64());
        let mut shaft = 0u32;
        let head = x87_store32(x87_mul(x87_load32(width), e.mem.f64(SIX)));
        // FCOMPP (the head against the length); TEST AH,0x5; JP: only an ordered below.
        if x87_load32(head) < x87_load32(length) {
            shaft = sub32(length, head);
        }
        let along = get_point(e, direction).map(|c| x87_quiet_bits(mul32(c, shaft)));
        set_point(e, offset, along);
        e.call(POINT_OPERATION_00439E90, &args![start, neck, offset]);
        // FCOMP 0.0; TEST AH,0x41; JNZ: only an ordered above makes the box.
        if x87_load32(shaft) > e.mem.f64(ZERO_DOUBLE) {
            let quiet = |bits: u32| f32::from_bits(x87_quiet_bits(bits));
            let shaft_box = bs_test_objects_create_box_ov2(
                e,
                Ptr::new(start),
                Ptr::new(neck),
                quiet(width),
                quiet(width),
                quiet(height),
                0.0,
                Ptr::new(first_color),
                Ptr::new(second_color),
            );
            e.vcall(node, NODE_ATTACH_CHILD, &args![shaft_box, 1u32]);
        }
        let vertices = allocate(e, 0x48);
        set_point(e, up, [0, 0, one_bits()]);
        e.call(POINT_OPERATION_004B3800, &args![direction, side, up]);
        let scale = x87_store32(x87_mul(x87_load32(width), e.mem.f64(FOUR)));
        let turned = get_point(e, side).map(|c| mul32(c, scale));
        set_point(e, side, turned);
        let lift = [0, 0, x87_quiet_bits(height)];
        let start_point = get_point(e, start);
        let neck_point = get_point(e, neck);
        let plus = [0, 1, 2].map(|k| x87_quiet_bits(add32(turned[k], neck_point[k])));
        let minus = [0, 1, 2].map(|k| {
            x87_quiet_bits(x87_store32(x87_sub(
                x87_m32(neck_point[k]),
                x87_load32(turned[k]),
            )))
        });
        let lifted = |base: [u32; 3]| [0, 1, 2].map(|k| x87_quiet_bits(add32(lift[k], base[k])));
        set_point(e, vertices, lifted(start_point));
        set_point(e, vertices.wrapping_add(0xc), lifted(plus));
        copy_words(e, vertices.wrapping_add(0x18), end, 3);
        set_point(e, vertices.wrapping_add(0x24), plus);
        set_point(e, vertices.wrapping_add(0x30), lifted(minus));
        set_point(e, vertices.wrapping_add(0x3c), minus);
        let mut second_color = second_color;
        if first_color != 0 && second_color == 0 {
            second_color = first_color;
        }
        let colors = if second_color != 0 {
            let block = allocate(e, 0x60);
            if block != 0 {
                zero_colors(e, block, 6);
            }
            block
        } else {
            0
        };
        if colors != 0 {
            for index in 0..6u32 {
                copy_words(e, colors.wrapping_add(index << 4), second_color, 4);
            }
        }
        let triangles = e.call(NI_MALLOC, &args![0x30u32]).u32();
        e.call(MEMCPY, &args![triangles, ARROW_TRIANGLES, 0x30u32]);
        let block = e.call(NI_NEW, &args![0xc4u32]).u32();
        let shape = if block != 0 {
            e.call(
                TRI_SHAPE_CONSTRUCT,
                &args![block, 6u32, vertices, 0u32, colors, 0u32, 0u32, 0u32, 8u32, triangles],
            )
            .u32()
        } else {
            0
        };
        e.vcall(shape, SHAPE_SLOT_E8, &args![]);
        e.vcall(node, NODE_ATTACH_CHILD, &args![shape, 1u32]);
    });
    node
}

/// The sphere's direction for the azimuth `theta` and the polar angle `phi` (both floats),
/// as the code computes it: `(sin(phi) * -sin(theta), sin(phi) * cos(theta), cos(phi) *
/// one)`, each factor of `theta` stored as a float and copied once more, each of `phi` a
/// fresh CRT call stored as a float, the other factor kept as a double, the product stored
/// as a float.
fn sphere_direction(e: &mut Engine, theta: u32, phi: u32, one: u32) -> [u32; 3] {
    let sine_theta = crt_float(e, SIN, theta);
    let minus_sine = -x87_load32(sine_theta);
    let sine_phi = x87_store32(e.call(SIN, &args![x87_load32(phi)]).f64());
    let x = x87_store32(x87_mul(x87_load32(sine_phi), minus_sine));
    let cosine_theta = x87_load32(crt_float(e, COS, theta));
    let sine_phi = x87_store32(e.call(SIN, &args![x87_load32(phi)]).f64());
    let y = x87_store32(x87_mul(x87_load32(sine_phi), cosine_theta));
    let one = x87_load32(one);
    let cosine_phi = x87_store32(e.call(COS, &args![x87_load32(phi)]).f64());
    let z = x87_store32(x87_mul(x87_load32(cosine_phi), one));
    [x, y, z]
}

/// The arrays one part of the sphere fills.
struct SphereArrays {
    vertices: u32,
    normals: u32,
    colors: u32,
    texture: u32,
}

impl SphereArrays {
    /// Vertex `index` at `radius` along `direction` (each `FLD radius`, `FMUL`), its
    /// normal `one * direction`, the colour when there is one, and its texture coordinates.
    #[allow(clippy::too_many_arguments)]
    fn put(
        &self,
        e: &mut Engine,
        index: u16,
        direction: [u32; 3],
        radius: u32,
        one: u32,
        color: u32,
        uv: [u32; 2],
    ) {
        let at = u32::from(index);
        let point = direction.map(|d| mul32(radius, d));
        set_point(e, self.vertices.wrapping_add(at * 0xc), point);
        let normal = direction.map(|d| mul32(one, d));
        set_point(e, self.normals.wrapping_add(at * 0xc), normal);
        if color != 0 {
            copy_words(e, self.colors.wrapping_add(at << 4), color, 4);
        }
        e.mem.set_u32(self.texture.wrapping_add(at * 8), uv[0]);
        e.mem.set_u32(self.texture.wrapping_add(at * 8 + 4), uv[1]);
    }

    /// A pole: the vertex `(0, 0, z)`, the normal `(0, 0, normal_z)`, the colour when there
    /// is one, and its texture coordinates.
    fn pole(&self, e: &mut Engine, index: u16, z: u32, normal_z: u32, color: u32, uv: [u32; 2]) {
        let at = u32::from(index);
        set_point(e, self.vertices.wrapping_add(at * 0xc), [0, 0, z]);
        set_point(e, self.normals.wrapping_add(at * 0xc), [0, 0, normal_z]);
        if color != 0 {
            copy_words(e, self.colors.wrapping_add(at << 4), color, 4);
        }
        e.mem.set_u32(self.texture.wrapping_add(at * 8), uv[0]);
        e.mem.set_u32(self.texture.wrapping_add(at * 8 + 4), uv[1]);
    }
}

/// The vertices, normals, colours (with a colour; zeroed when given) and texture
/// coordinates of `count` vertices, from the memory manager.
fn sphere_arrays(e: &mut Engine, count: u32, color: u32) -> SphereArrays {
    let (vertices, normals, colors) = strip_arrays(e, count, color);
    let texture = allocate(e, array_bytes(count, 8));
    SphereArrays {
        vertices,
        normals,
        colors,
        texture,
    }
}

/// Appends a triangle to the index list (`count` is the number of indices so far).
fn push_triangle(e: &mut Engine, triangles: u32, count: &mut u16, corners: [u16; 3]) {
    for corner in corners {
        e.mem
            .set_u16(triangles.wrapping_add(u32::from(*count) * 2), corner);
        *count = count.wrapping_add(1);
    }
}

/// The azimuth for step `k`: 0 for the closing step `segments`, else `k * step` (through a
/// double, stored as a float).
fn azimuth(k: u32, segments: u16, step: u32) -> u32 {
    if k == u32::from(segments) {
        0
    } else {
        x87_store32(x87_mul(f64::from(k), x87_m32(step)))
    }
}

// Translated from 00c56680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c56680 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f72b, state variable) is not translated
/// `BSTestObjects::CreateSphere` (Xbox PDB, engine map; `cdecl`, `(radius, segments, rings,
/// colour)`): an `NiNode` (`NiNew(0xAC)`, `00a5ecb0` with 0; null when that fails) named
/// (`010c2c80`) and holding the sphere in two parts. The polar step is the float pi over
/// `rings`, the azimuth step the float pi times the double 2.0 over `segments`. The caps
/// are one `NiTriShape` of `segments * 2 + 2` vertices: the top pole `(0, 0, radius)`
/// (normal +z, texture (0.5, 0)), a ring at the first polar step (azimuth `i * step`,
/// fanned to the pole from the second point on, then closed), the bottom pole `(0, 0,
/// -radius)` (normal -z from `01012054`, texture (0.5, 1)) and a ring at pi minus the step
/// walked back from `segments` to 1 (azimuth 0 first), fanned the same way. The band
/// between is one `NiTriStrips` of `rings - 2` strips of `segments * 2 + 2` vertices: for
/// each band polar angle `(band + 1) * step` and each azimuth (the closing one 0), a vertex
/// there and one a step further down. Each vertex is `radius` along its direction with the
/// normal `1.0 * direction` (`sphere_direction`), the colour and texture coordinates
/// advancing by `1 / segments` and `1 / rings`. All arrays come from the memory manager
/// (sizes all ones on overflow; written through null when one fails). Each part runs its
/// vtable slot +0xE8 (called through null when its `NiNew` fails) and is attached to the
/// node. Answers the node.
pub fn bs_test_objects_create_sphere(
    e: &mut Engine,
    radius: f32,
    segments: u16,
    rings: u16,
    color: u32,
) -> u32 {
    let radius = radius.to_bits();
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    name_object(e, node, SPHERE_NAME);
    let one = one_bits();
    let pi = e.mem.u32(PI);
    let phi_step = x87_store32(x87_div(x87_m32(pi), f64::from(rings)));
    let full_turn = x87_mul(x87_load32(pi), e.mem.f64(TWO));
    let theta_step = x87_store32(x87_div(full_turn, f64::from(segments)));
    let u_step = x87_store32(x87_div(1.0, f64::from(segments)));
    let v_step = x87_store32(x87_div(1.0, f64::from(rings)));
    let cap = u32::from(segments).wrapping_mul(2).wrapping_add(2);
    let caps = sphere_arrays(e, cap, color);
    let triangles = allocate(e, array_bytes(u32::from(segments).wrapping_mul(6), 2));
    let half = x87_quiet_bits(e.mem.u32(HALF));
    let mut index = 0u16;
    let mut count = 0u16;
    let mut first = index;
    caps.pole(e, index, mul32(radius, one), one, color, [half, 0]);
    index = index.wrapping_add(1);
    let mut u = 0u32;
    let v = x87_quiet_bits(v_step);
    let mut i = 0u32;
    while i < u32::from(segments) {
        let theta = x87_store32(x87_mul(f64::from(i), x87_m32(theta_step)));
        let direction = sphere_direction(e, theta, phi_step, one);
        caps.put(
            e,
            index,
            direction,
            radius,
            one,
            color,
            [x87_quiet_bits(u), v],
        );
        index = index.wrapping_add(1);
        u = add32(u, u_step);
        if i > 0 {
            push_triangle(
                e,
                triangles,
                &mut count,
                [first, index.wrapping_sub(2), index.wrapping_sub(1)],
            );
        }
        i += 1;
    }
    push_triangle(
        e,
        triangles,
        &mut count,
        [first, index.wrapping_sub(1), first.wrapping_add(1)],
    );
    first = index;
    let bottom = x87_store32(x87_mul(x87_load32(one), -x87_load32(radius)));
    let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    caps.pole(e, index, bottom, minus_one, color, [half, one]);
    index = index.wrapping_add(1);
    let lower_phi = sub32(pi, phi_step);
    let mut u = one;
    let v = x87_store32(x87_sub(1.0, x87_load32(v_step)));
    let mut k = u32::from(segments);
    while k > 0 {
        let theta = azimuth(k, segments, theta_step);
        let direction = sphere_direction(e, theta, lower_phi, one);
        caps.put(
            e,
            index,
            direction,
            radius,
            one,
            color,
            [x87_quiet_bits(u), x87_quiet_bits(v)],
        );
        index = index.wrapping_add(1);
        u = sub32(u, u_step);
        if k != u32::from(segments) {
            push_triangle(
                e,
                triangles,
                &mut count,
                [first, index.wrapping_sub(2), index.wrapping_sub(1)],
            );
        }
        k -= 1;
    }
    push_triangle(
        e,
        triangles,
        &mut count,
        [first, index.wrapping_sub(1), first.wrapping_add(1)],
    );
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            TRI_SHAPE_CONSTRUCT,
            &args![
                block,
                u32::from(index),
                caps.vertices,
                caps.normals,
                caps.colors,
                caps.texture,
                1u32,
                0u32,
                u32::from(count) / 3,
                triangles
            ],
        )
        .u32()
    } else {
        0
    };
    e.vcall(shape, SHAPE_SLOT_E8, &args![]);
    e.vcall(node, NODE_ATTACH_CHILD, &args![shape, 1u32]);
    let mut v = 0u32;
    let bands = rings.wrapping_sub(2);
    let per_strip = segments.wrapping_mul(2).wrapping_add(2);
    let total = u32::from(bands).wrapping_mul(u32::from(per_strip));
    let band_arrays = sphere_arrays(e, total, color);
    let lengths = allocate(e, array_bytes(u32::from(bands), 2));
    let indices = allocate(e, array_bytes(total, 2));
    let mut index = 0u16;
    let mut band = 0u32;
    while band < u32::from(bands) {
        let phi = x87_store32(x87_mul(f64::from(band.wrapping_add(1)), x87_m32(phi_step)));
        let mut u = 0u32;
        let mut segment = 0u32;
        while segment <= u32::from(segments) {
            let theta = azimuth(segment, segments, theta_step);
            let direction = sphere_direction(e, theta, phi, one);
            band_arrays.put(
                e,
                index,
                direction,
                radius,
                one,
                color,
                [x87_quiet_bits(u), x87_quiet_bits(v)],
            );
            e.mem
                .set_u16(indices.wrapping_add(u32::from(index) * 2), index);
            index = index.wrapping_add(1);
            let next_phi = add32(phi_step, phi);
            let direction = sphere_direction(e, theta, next_phi, one);
            band_arrays.put(
                e,
                index,
                direction,
                radius,
                one,
                color,
                [x87_quiet_bits(u), add32(v_step, v)],
            );
            e.mem
                .set_u16(indices.wrapping_add(u32::from(index) * 2), index);
            index = index.wrapping_add(1);
            u = add32(u, u_step);
            segment += 1;
        }
        v = add32(v, v_step);
        e.mem.set_u16(lengths.wrapping_add(band * 2), per_strip);
        band += 1;
    }
    let strip_triangles = u32::from(bands).wrapping_mul(u32::from(segments) << 1);
    let strips = new_strips(
        e,
        u32::from(index),
        band_arrays.vertices,
        band_arrays.normals,
        band_arrays.colors,
        band_arrays.texture,
        1,
        strip_triangles,
        u32::from(bands),
        lengths,
        indices,
    );
    e.vcall(strips, SHAPE_SLOT_E8, &args![]);
    e.vcall(node, NODE_ATTACH_CHILD, &args![strips, 1u32]);
    node
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5_5d50,
            bs_test_objects_create_box_ov2(Ptr, Ptr, f32, f32, f32, f32, Ptr, Ptr) -> u32
        ),
        entry!(0x00c5_5920, bs_test_objects_create_box(u32, u32) -> u32),
        entry!(0x00c5_5cd0, fn_00c55cd0(u32, f32, u32) -> u32),
        entry!(0x00c5_a800, ni_point3_perpendicular(u32, u32) -> u32),
        entry!(
            0x00c5_8d30,
            fn_00c58d30(f32, f32, f32, u16, u16, u32) -> u32
        ),
        entry!(0x00c5_8f00, fn_00c58f00(f32, u16, u32) -> u32),
        entry!(
            0x00c5_9200,
            bs_test_objects_create_coordinate_jack(f32) -> u32
        ),
        entry!(0x00c5_9660, fn_00c59660(f32, u32) -> u32),
        entry!(0x00c5_7d70, fn_00c57d70(f32, f32, u16, u16, u32) -> u32),
        entry!(
            0x00c5_84a0,
            bs_test_objects_create_cylinder(f32, f32, f32, u16, u16, u32) -> u32
        ),
        entry!(
            0x00c5_9a50,
            bs_test_objects_create_dir_arrow(u32, u32) -> u32
        ),
        entry!(
            0x00c5_a0f0,
            bs_test_objects_create_3d_arrow(u32, u32, f32, f32, u32, u32) -> u32
        ),
        entry!(
            0x00c5_6680,
            bs_test_objects_create_sphere(f32, u16, u16, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHAPE_VTABLE: u32 = 0x0e60_0000;
    const SHAPE_FINISH: u32 = 0x0e61_0000;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0xc);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
        at
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    /// Doubles: the allocations from the heap, the first direction (1, 0, 0), the turn
    /// keeping it, the unitize doing nothing, `00439e90` adding, `memcpy`, `NiNew` and the
    /// shape constructor storing the shape vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(POINT_OPERATION_00439EF0, |e, a| {
            e.mem.set_u32(a[1], 1.0f32.to_bits());
            e.mem.set_u32(a[1] + 4, 0);
            e.mem.set_u32(a[1] + 8, 0);
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004B3800, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            // A z the code clears afterwards.
            e.mem.set_u32(a[1] + 8, 9.0f32.to_bits());
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004A0C10, |_, _| Ret::default());
        e.register(POINT_OPERATION_00439E90, |e, a| {
            for i in 0..3 {
                let sum = f32::from_bits(e.mem.u32(a[0] + 4 * i))
                    + f32::from_bits(e.mem.u32(a[2] + 4 * i));
                e.mem.set_u32(a[1] + 4 * i, sum.to_bits());
            }
            a[1].into_ret()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.map(BOX_TRIANGLES, 0x48);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRI_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], SHAPE_VTABLE);
            a[0].into_ret()
        });
        e.put_vtable(SHAPE_VTABLE, &[0; 0x3c]);
        e.mem.set_u32(SHAPE_VTABLE + SHAPE_SLOT_E8, SHAPE_FINISH);
        e.register(SHAPE_FINISH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn box_corners_and_shape() {
        let mut e = engine();
        e.mem.set_u16(BOX_TRIANGLES + 2, 7);
        let start = point(&mut e, [10.0, 20.0, 30.0]);
        let end = point(&mut e, [100.0, 200.0, 300.0]);
        let shape = e
            .call(
                0x00c5_5d50,
                &args![start, end, 2.0f32, 3.0f32, 5.0f32, 7.0f32, 0u32, 0u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, POINT_OPERATION_00439EF0)[0][2], end);
        assert_eq!(calls(&log, POINT_OPERATION_00439EF0)[0][0], start);
        let made = calls(&log, TRI_SHAPE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let words = &made[0];
        assert_eq!(words[0], shape);
        assert_eq!(words[1], 8);
        assert_eq!([words[3], words[4], words[5]], [0, 0, 0]);
        assert_eq!([words[6], words[7], words[8]], [0, 0, 0xc]);
        let vertices = words[2];
        assert_eq!(
            floats(&e, vertices, 24),
            vec![
                97.0, 200.0, 305.0, 102.0, 200.0, 305.0, 97.0, 200.0, 293.0, 102.0, 200.0, 293.0,
                7.0, 20.0, 35.0, 12.0, 20.0, 35.0, 7.0, 20.0, 23.0, 12.0, 20.0, 23.0
            ]
        );
        // The triangles are a copy of the exe's list.
        assert_eq!(e.mem.u16(words[9] + 2), 7);
        assert_eq!(calls(&log, SHAPE_FINISH), vec![vec![shape]]);
        assert!(calls(&log, VECTOR_CONSTRUCT).is_empty());
        // The turned direction's z was cleared before the unitize.
        let turned = calls(&log, POINT_OPERATION_004A0C10)[0][0];
        assert_eq!(calls(&log, POINT_OPERATION_004B3800)[0][1], turned);
    }

    #[test]
    fn box_colours_fill_in_for_each_other() {
        let mut e = engine();
        let start = point(&mut e, [0.0; 3]);
        let end = point(&mut e, [1.0, 0.0, 0.0]);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(first + 4 * i, 0x100 + i);
            e.mem.set_u32(second + 4 * i, 0x200 + i);
        }
        let colours_of = |e: &mut Engine, a: u32, b: u32| -> Vec<u32> {
            e.call_log = Some(vec![]);
            e.call(
                0x00c5_5d50,
                &args![start, end, 1.0f32, 1.0f32, 1.0f32, 1.0f32, a, b],
            );
            let log = e.call_log.take().unwrap();
            let made = calls(&log, TRI_SHAPE_CONSTRUCT);
            let constructed = calls(&log, VECTOR_CONSTRUCT);
            assert_eq!(
                constructed,
                vec![vec![made[0][4], 0x10, 8, COLOR_CONSTRUCT]]
            );
            (0..8).map(|i| e.mem.u32(made[0][4] + 0x10 * i)).collect()
        };
        assert_eq!(
            colours_of(&mut e, first, second),
            vec![0x200, 0x200, 0x200, 0x200, 0x100, 0x100, 0x100, 0x100]
        );
        assert_eq!(colours_of(&mut e, first, 0), vec![0x100; 8]);
        assert_eq!(colours_of(&mut e, 0, second), vec![0x200; 8]);
    }

    #[test]
    fn box_signs_follow_the_x87() {
        let mut e = engine();
        let start = point(&mut e, [0.0; 3]);
        let end = point(&mut e, [0.0; 3]);
        // A signalling NaN for `up` is stored quiet; `-down` of a NaN flips its sign.
        let shape = e
            .call(
                0x00c5_5d50,
                &args![
                    start,
                    end,
                    1.0f32,
                    1.0f32,
                    0x7f80_0001u32,
                    0x7fc0_0002u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        assert_ne!(shape, 0);
        let log = e.call_log.take().unwrap();
        let vertices = calls(&log, TRI_SHAPE_CONSTRUCT)[0][2];
        assert_eq!(e.mem.u32(vertices + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(vertices + 0x20), 0xffc0_0002);
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SHAPE_VTABLE: u32 = 0x0e70_0000;
    const SHAPE_FINISH: u32 = 0x0e71_0000;
    const NODE_VTABLE: u32 = 0x0e72_0000;
    const NODE_ATTACH: u32 = 0x0e73_0000;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0xc);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + 4 * i, v);
        }
        at
    }

    fn f(word: u32) -> f32 {
        f32::from_bits(word)
    }

    /// Records a constructor's words and gives the object the shape vtable (slot +0xE8
    /// logged).
    fn constructor(e: &mut Engine, log: &Log, address: u32) {
        let l = log.clone();
        e.register_double(address, move |e, a| {
            l.borrow_mut().push((address, a.to_vec()));
            e.mem.set_u32(a[0], SHAPE_VTABLE);
            a[0].into_ret()
        });
    }

    /// The doubles every test object needs: allocations from the heap, the constructors,
    /// the naming, the node and the constants mapped with their exe values.
    fn engine(log: &Log) -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_MALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        constructor(&mut e, log, TRI_SHAPE_CONSTRUCT);
        constructor(&mut e, log, TRI_STRIPS_CONSTRUCT);
        constructor(&mut e, log, LINES_CONSTRUCT);
        e.put_vtable(SHAPE_VTABLE, &[0; 0x3c]);
        e.mem.set_u32(SHAPE_VTABLE + SHAPE_SLOT_E8, SHAPE_FINISH);
        logged(&mut e, log, SHAPE_FINISH, 0);
        e.register(NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], NODE_VTABLE);
            a[0].into_ret()
        });
        e.put_vtable(NODE_VTABLE, &[0; 0x38]);
        e.mem.set_u32(NODE_VTABLE + NODE_ATTACH_CHILD, NODE_ATTACH);
        logged(&mut e, log, NODE_ATTACH, 0);
        let l = log.clone();
        e.register_double(FIXED_STRING_CREATE, move |e, a| {
            l.borrow_mut().push((FIXED_STRING_CREATE, vec![a[0]]));
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 1);
            (block + 8).into_ret()
        });
        logged(&mut e, log, SET_NAME, 0);
        logged(&mut e, log, INTERLOCKED_DECREMENT, 0);
        e.map(FIXED_STRING_EMPTY, 4);
        let floats_at = [
            (PI, std::f32::consts::PI),
            (HALF, 0.5),
            (MINUS_HALF, -0.5),
            (MINUS_ONE, -1.0),
            (ARROW_BACK, 0.75),
        ];
        for (address, value) in floats_at {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
        let doubles_at = [
            (TWO, 2.0),
            (TWO_PI, std::f64::consts::TAU),
            (SIX, 6.0),
            (EIGHT, 8.0),
            (ARROW_WIDTH, f64::from(0.15f32)),
            (FOUR, 4.0),
            (ZERO_DOUBLE, 0.0),
        ];
        for (address, value) in doubles_at {
            e.map(address, 8);
            e.mem.set_f64(address, value);
        }
        e.map(BOX_CORNERS, 0x64);
        e.map(BOX_TRIANGLES, 0x48);
        e.map(ARROW_TRIANGLES, 0x30);
        e.map(POINT_ZERO, 0xc);
        e.map(IDENTITY_MATRIX, 0x24);
        for (k, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(IDENTITY_MATRIX + 4 * k as u32, *value);
        }
        e.map(DEFAULT_COLOR, 0x30);
        for k in 0..12u32 {
            e.mem.set_u32(DEFAULT_COLOR + 4 * k, 0x100 + k);
        }
        e
    }

    /// `sin` and `cos` as fixed answers (0 and 1), recording their arguments.
    fn flat_trigonometry(e: &mut Engine, log: &Log) {
        for (address, value) in [(SIN, 0.0f64), (COS, 1.0)] {
            let l = log.clone();
            e.register_double(address, move |_, a| {
                let argument = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
                l.borrow_mut()
                    .push((address, vec![(argument as f32).to_bits()]));
                value.into_ret()
            });
        }
    }

    #[test]
    fn box_corners_scale_the_unit_box() {
        let log = Log::default();
        let mut e = engine(&log);
        let extents = point(&mut e, [1.0, 2.0, 3.0]);
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color, 0x55);
        let shape = e.call(0x00c5_5920, &args![extents, color]).u32();
        assert_eq!(e.mem.u32(BOX_CORNERS_GUARD) & 1, 1);
        let made = calls(&log, TRI_SHAPE_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let words = &made[0];
        assert_eq!(words[0], shape);
        assert_eq!(
            [words[1], words[3], words[5], words[6], words[7], words[8]],
            [8, 0, 0, 0, 0, 0xc]
        );
        assert_eq!(
            floats(&e, words[2], 24),
            vec![
                0.5, 1.0, 1.5, 0.5, 1.0, -1.5, 0.5, -1.0, 1.5, 0.5, -1.0, -1.5, -0.5, 1.0, 1.5,
                -0.5, 1.0, -1.5, -0.5, -1.0, 1.5, -0.5, -1.0, -1.5
            ]
        );
        assert!((0..8).all(|i| e.mem.u32(words[4] + 0x10 * i) == 0x55));
        assert_eq!(calls(&log, SHAPE_FINISH), vec![vec![shape]]);
        // No colour: no colour array.
        e.call(0x00c5_5920, &args![extents, 0u32]);
        assert_eq!(calls(&log, TRI_SHAPE_CONSTRUCT)[1][4], 0);
    }

    #[test]
    fn small_box_spans_the_point_along_y() {
        let log = Log::default();
        let mut e = engine(&log);
        let l = log.clone();
        e.register_double(POINT_OPERATION_00439EF0, move |e, a| {
            l.borrow_mut()
                .push((0, floats(e, a[0], 3).iter().map(|v| v.to_bits()).collect()));
            l.borrow_mut()
                .push((1, floats(e, a[2], 3).iter().map(|v| v.to_bits()).collect()));
            e.mem.set_f32(a[1], 1.0);
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004B3800, |_, a| a[1].into_ret());
        e.register(POINT_OPERATION_004A0C10, |_, _| Ret::default());
        e.register(POINT_OPERATION_00439E90, |_, a| a[1].into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let at = point(&mut e, [1.0, 2.0, 3.0]);
        let shape = e.call(0x00c5_5cd0, &args![at, 0.5f32, 0u32]).u32();
        assert_eq!(calls(&log, SHAPE_FINISH), vec![vec![shape]]);
        let start: Vec<f32> = calls(&log, 0)[0].iter().map(|w| f(*w)).collect();
        let end: Vec<f32> = calls(&log, 1)[0].iter().map(|w| f(*w)).collect();
        assert_eq!(start, vec![1.0, 1.5, 3.0]);
        assert_eq!(end, vec![1.0, 2.5, 3.0]);
    }

    #[test]
    fn perpendicular_picks_the_axis_by_the_largest_component() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(FABS, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            value.abs().into_ret()
        });
        logged(&mut e, &log, UNIT_CROSS, 0);
        let out = e.mem.alloc(0xc);
        for (values, axis) in [
            ([3.0f32, 1.0, 0.0], AXIS_011A9484),
            ([3.0, 1.0, 5.0], AXIS_011A9478),
            ([1.0, 3.0, 0.0], AXIS_011A9484),
            ([1.0, -3.0, 5.0], AXIS_011A946C),
        ] {
            let at = point(&mut e, values);
            assert_eq!(e.call(0x00c5_a800, &args![at, out]).u32(), out);
            assert_eq!(
                calls(&log, UNIT_CROSS).last().unwrap(),
                &vec![at, out, axis]
            );
        }
    }

    #[test]
    fn three_cylinders_under_a_node() {
        let log = Log::default();
        let mut e = engine(&log);
        flat_trigonometry(&mut e, &log);
        let shape = e
            .call(
                0x00c5_8d30,
                &args![1.0f32, 2.0f32, 3.0f32, 2u32, 1u32, 0u32],
            )
            .u32();
        let strips = calls(&log, TRI_STRIPS_CONSTRUCT);
        assert_eq!(strips.len(), 3);
        let attached = calls(&log, NODE_ATTACH);
        assert_eq!(
            attached,
            strips
                .iter()
                .map(|s| vec![shape, s[0], 1])
                .collect::<Vec<_>>()
        );
        let lower = strips[1][0];
        let rotation = floats(&e, lower + LOCAL_ROTATE, 9);
        assert_eq!(rotation[..4], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(rotation[4], -1.0);
        assert_eq!(rotation[8], -1.0);
        assert_eq!(rotation[5], -rotation[7]);
        let upper = strips[2][0];
        assert_eq!(floats(&e, upper + LOCAL_TRANSLATE, 3), vec![0.0, 0.0, 3.0]);
        // The discs have no height: their second ring of vertices stays at z = 0.
        assert_eq!(e.mem.f32(strips[1][2] + 0x14), 0.0);
    }

    #[test]
    fn circle_points_and_lines() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(SIN, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (if x == 0.0 { 0.0 } else { 1.0 }).into_ret()
        });
        e.register(COS, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (if x == 0.0 { 1.0 } else { 0.5 }).into_ret()
        });
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color + 8, 0x77);
        let lines = e.call(0x00c5_8f00, &args![2.0f32, 3u32, color]).u32();
        let made = calls(&log, LINES_CONSTRUCT);
        assert_eq!(made.len(), 1);
        let words = &made[0];
        assert_eq!(words[0], lines);
        assert_eq!([words[1], words[4], words[5], words[6]], [3, 0, 1, 0]);
        assert_eq!(
            floats(&e, words[2], 9),
            vec![0.0, 2.0, 0.0, 2.0, 1.0, 0.0, 2.0, 1.0, 0.0]
        );
        assert_eq!(e.mem.bytes(words[7], 3), vec![1, 1, 1]);
        assert!((0..3).all(|i| e.mem.u32(words[3] + 0x10 * i + 8) == 0x77));
        e.call(0x00c5_8f00, &args![2.0f32, 1u32, 0u32]);
        assert_eq!(calls(&log, LINES_CONSTRUCT)[1][3], 0);
    }

    #[test]
    fn coordinate_jack_and_arrow_lines() {
        let log = Log::default();
        let mut e = engine(&log);
        let jack = e.call(0x00c5_9200, &args![2.0f32]).u32();
        let words = calls(&log, LINES_CONSTRUCT)[0].clone();
        assert_eq!(
            [words[0], words[1], words[4], words[5], words[6]],
            [jack, 6, 0, 0, 0]
        );
        assert_eq!(
            floats(&e, words[2], 18),
            vec![
                0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.0
            ]
        );
        let colors: Vec<u32> = (0..6).map(|i| e.mem.u32(words[3] + 0x10 * i)).collect();
        assert_eq!(colors, vec![0x100, 0x100, 0x104, 0x104, 0x108, 0x108]);
        assert_eq!(e.mem.bytes(words[7], 6), vec![1, 0, 1, 0, 1, 0]);
        assert_eq!(floats(&e, jack + LOCAL_ROTATE, 9)[4], 1.0);
        assert_eq!(calls(&log, FIXED_STRING_CREATE), vec![vec![JACK_NAME]]);
        assert_eq!(calls(&log, SET_NAME)[0][0], jack);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT).len(), 1);
        let arrow = e.call(0x00c5_9660, &args![6.0f32, 0u32]).u32();
        let words = calls(&log, LINES_CONSTRUCT)[1].clone();
        assert_eq!([words[0], words[1]], [arrow, 5]);
        assert_eq!(
            floats(&e, words[2], 15),
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 6.0, 1.0, 0.0, 5.0, 0.0, 0.0, 6.0, -1.0, 0.0, 5.0]
        );
        assert!((0..5).all(|i| e.mem.u32(words[3] + 0x10 * i) == 0x100));
        assert_eq!(e.mem.bytes(words[7], 5), vec![1, 1, 0, 1, 0]);
        assert_eq!(calls(&log, FIXED_STRING_CREATE)[1], vec![ARROW_NAME]);
    }

    #[test]
    fn torus_strips() {
        let log = Log::default();
        let mut e = engine(&log);
        flat_trigonometry(&mut e, &log);
        let torus = e
            .call(0x00c5_7d70, &args![1.0f32, 3.0f32, 2u32, 1u32, 0u32])
            .u32();
        let words = calls(&log, TRI_STRIPS_CONSTRUCT)[0].clone();
        assert_eq!(words[0], torus);
        assert_eq!(
            [words[1], words[4], words[5], words[6], words[7], words[8], words[9]],
            [8, 0, 0, 0, 0, 4, 2]
        );
        assert_eq!(e.mem.u16(words[10]), 4);
        assert_eq!(e.mem.u16(words[10] + 2), 4);
        let indices: Vec<u16> = (0..8).map(|i| e.mem.u16(words[11] + 2 * i)).collect();
        assert_eq!(indices, (0..8).collect::<Vec<u16>>());
        assert_eq!(floats(&e, words[2], 3), vec![4.0, -0.0, 0.0]);
        assert_eq!(floats(&e, words[3], 3), vec![1.0, -0.0, 0.0]);
        // The first ring angle is one step, the float pi times 2.0 over 2 rings.
        assert_eq!(f(calls(&log, COS)[0][0]), std::f32::consts::PI);
        assert_eq!(calls(&log, FIXED_STRING_CREATE), vec![vec![TORUS_NAME]]);
        assert_eq!(calls(&log, SHAPE_FINISH), vec![vec![torus]]);
    }

    #[test]
    fn cylinder_rings_normals_and_texture() {
        let log = Log::default();
        let mut e = engine(&log);
        flat_trigonometry(&mut e, &log);
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color, 0x99);
        let strips = e
            .call(
                0x00c5_84a0,
                &args![1.0f32, 2.0f32, 4.0f32, 2u32, 1u32, color],
            )
            .u32();
        let words = calls(&log, TRI_STRIPS_CONSTRUCT)[0].clone();
        assert_eq!(words[0], strips);
        assert_eq!(
            [words[1], words[6], words[7], words[8], words[9]],
            [6, 1, 0, 4, 1]
        );
        assert_eq!(floats(&e, words[2], 6), vec![0.0, 1.0, 0.0, 0.0, 2.0, 4.0]);
        assert_eq!(floats(&e, words[3], 3), vec![0.0, 1.0, -0.25]);
        assert_eq!(
            floats(&e, words[5], 12),
            vec![0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0]
        );
        assert!((0..6).all(|i| e.mem.u32(words[4] + 0x10 * i) == 0x99));
        assert_eq!(e.mem.u16(words[10]), 6);
        // Two of the three angles are computed: the last one is taken as sine 0, cosine 1.
        assert_eq!(calls(&log, SIN).len(), 2);
        assert_eq!(f(calls(&log, SIN)[1][0]), std::f32::consts::PI);
    }

    /// Point helpers that compute: `-` and `+` (`(this, &out, &other)`), the cross product
    /// and the unitize (keeping the length).
    fn point_math(e: &mut Engine) {
        e.register(POINT_OPERATION_00439EF0, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_00439E90, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) + e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_OPERATION_004B3800, |e, a| {
            let p = floats(e, a[0], 3);
            let q = floats(e, a[2], 3);
            let cross = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            for (k, v) in (0u32..).zip(cross) {
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(UNITIZE_GET_LENGTH, |e, a| {
            let p = floats(e, a[0], 3);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            for (k, v) in (0u32..).zip(p) {
                e.mem.set_f32(a[0] + 4 * k, v / length);
            }
            f64::from(length).into_ret()
        });
        e.register(POINT_LENGTH, |e, a| {
            let p = floats(e, a[0], 3);
            f64::from((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()).into_ret()
        });
        e.register(POINT_OPERATION_004A0C10, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn dir_arrow_lengthens_short_directions() {
        let log = Log::default();
        let mut e = engine(&log);
        point_math(&mut e);
        e.register(FABS, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            value.abs().into_ret()
        });
        e.register(UNIT_CROSS, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            a[1].into_ret()
        });
        let direction = point(&mut e, [0.0, 0.0, 2.0]);
        let arrow = e.call(0x00c5_9a50, &args![direction, 0u32]).u32();
        let words = calls(&log, LINES_CONSTRUCT)[0].clone();
        assert_eq!([words[0], words[1]], [arrow, 5]);
        let width = 8.0 * 0.15f32;
        assert_eq!(
            floats(&e, words[2], 15),
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 8.0, width, 0.0, 6.0, 0.0, 0.0, 8.0, -width, 0.0, 6.0]
        );
        assert_eq!(e.mem.u32(words[3]), 0x100);
        assert_eq!(floats(&e, words[3] + 0x20, 4), vec![0.0, 1.0, 0.0, 1.0]);
        assert_eq!(e.mem.bytes(words[7], 5), vec![1, 0, 1, 1, 0]);
        assert_eq!(calls(&log, FIXED_STRING_CREATE), vec![vec![DIR_ARROW_NAME]]);
        // The caller's direction is not changed.
        assert_eq!(e.mem.f32(direction + 8), 2.0);
    }

    #[test]
    fn three_d_arrow_box_and_head() {
        let log = Log::default();
        let mut e = engine(&log);
        point_math(&mut e);
        let start = point(&mut e, [0.0, 0.0, 0.0]);
        let end = point(&mut e, [10.0, 0.0, 0.0]);
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color, 0x42);
        let node = e
            .call(0x00c5_a0f0, &args![start, end, 1.0f32, 0.5f32, color, 0u32])
            .u32();
        let shapes = calls(&log, TRI_SHAPE_CONSTRUCT);
        assert_eq!(shapes.len(), 2);
        let head = &shapes[1];
        assert_eq!(
            [head[1], head[3], head[5], head[6], head[7], head[8]],
            [6, 0, 0, 0, 0, 8]
        );
        assert_eq!(
            floats(&e, head[2], 18),
            vec![
                0.0, 0.0, 0.5, 4.0, -4.0, 0.5, 10.0, 0.0, 0.0, 4.0, -4.0, 0.0, 4.0, 4.0, 0.5, 4.0,
                4.0, 0.0
            ]
        );
        assert!((0..6).all(|i| e.mem.u32(head[4] + 0x10 * i) == 0x42));
        assert_eq!(
            calls(&log, NODE_ATTACH),
            vec![vec![node, shapes[0][0], 1], vec![node, head[0], 1]]
        );
        // The box runs from the start to the neck.
        let box_vertices = floats(&e, shapes[0][2], 3);
        assert_eq!(box_vertices[0], 4.0);
        // A head as long as the whole arrow: no box.
        log.borrow_mut().clear();
        e.call(0x00c5_a0f0, &args![start, end, 2.0f32, 0.5f32, 0u32, 0u32]);
        assert_eq!(calls(&log, TRI_SHAPE_CONSTRUCT).len(), 1);
        assert_eq!(calls(&log, TRI_SHAPE_CONSTRUCT)[0][4], 0);
    }

    #[test]
    fn sphere_caps_and_band() {
        let log = Log::default();
        let mut e = engine(&log);
        flat_trigonometry(&mut e, &log);
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color, 0x31);
        let node = e.call(0x00c5_6680, &args![1.0f32, 2u32, 3u32, color]).u32();
        assert_eq!(calls(&log, FIXED_STRING_CREATE), vec![vec![SPHERE_NAME]]);
        assert_eq!(calls(&log, SET_NAME)[0][0], node);
        let caps = calls(&log, TRI_SHAPE_CONSTRUCT)[0].clone();
        assert_eq!([caps[1], caps[6], caps[7], caps[8]], [6, 1, 0, 4]);
        let triangles: Vec<u16> = (0..12).map(|i| e.mem.u16(caps[9] + 2 * i)).collect();
        assert_eq!(triangles, vec![0, 1, 2, 0, 2, 1, 3, 4, 5, 3, 5, 4]);
        let z: Vec<f32> = (0..6).map(|i| e.mem.f32(caps[2] + 0xc * i + 8)).collect();
        assert_eq!(z, vec![1.0, 1.0, 1.0, -1.0, 1.0, 1.0]);
        let third = 1.0f32 / 3.0;
        assert_eq!(
            floats(&e, caps[5], 12),
            vec![
                0.5,
                0.0,
                0.0,
                third,
                0.5,
                third,
                0.5,
                1.0,
                1.0,
                1.0 - third,
                0.5,
                1.0 - third
            ]
        );
        assert_eq!(e.mem.f32(caps[3] + 0xc * 3 + 8), -1.0);
        assert!((0..6).all(|i| e.mem.u32(caps[4] + 0x10 * i) == 0x31));
        let band = calls(&log, TRI_STRIPS_CONSTRUCT)[0].clone();
        assert_eq!(
            [band[1], band[6], band[7], band[8], band[9]],
            [6, 1, 0, 4, 1]
        );
        assert_eq!(e.mem.u16(band[10]), 6);
        assert_eq!(
            floats(&e, band[5], 12),
            vec![0.0, 0.0, 0.0, third, 0.5, 0.0, 0.5, third, 1.0, 0.0, 1.0, third]
        );
        assert_eq!(
            calls(&log, NODE_ATTACH),
            vec![vec![node, caps[0], 1], vec![node, band[0], 1]]
        );
        // The bottom cap starts at azimuth 0 (three sine calls per direction).
        assert_eq!(calls(&log, SIN)[6][0], 0);
    }
}

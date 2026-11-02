//! `fallout shared/run_0057d090` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `0057d0a0`, `0057d4f0` and `0057de00` open a scope guard (`00404eb0`)
//! naming the source file `TESTerrainLODManager.cpp` (string at
//! `0x0103_10d8`, lines 0xb1 and 0x20a). They walk the 5x5 cells around a
//! position, the land quads of each cell and the points of each quad, and
//! hand the points they keep to `00b62de0`. The three take arguments the
//! callee reads by address, so their argument words and address-taken
//! locals live in a game-memory frame (`frame`, `EBP` at a fixed offset in
//! it) at the offsets the exe uses.
//!
//! x87 note: the code computes in extended precision and stores `float`s;
//! the translations compute in `f64` and round at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `0057d090` answers.
const WORD_SOURCE: u32 = 0x011c_6224;
/// The `TES` pointer.
const TES_POINTER: u32 = 0x011d_ea10;
/// Scope guard constructor (`00404eb0`, `thiscall`, four words) and
/// destructor (`00404ee0`).
const SCOPE_GUARD_OPEN: u32 = 0x0040_4eb0;
const SCOPE_GUARD_CLOSE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESTerrainLODManager.cpp"`.
const SOURCE_FILE: u32 = 0x0103_10d8;
/// `_ftol2` (`00ec62c0`): truncates ST0 (the leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// A bool setting's value pointer (`00408d60`, `thiscall`).
const BOOL_SETTING_VALUE: u32 = 0x0040_8d60;
/// A float setting's value pointer (`00403e20`, `thiscall`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `0057d0a0` reads (static objects; their names are set by
/// the static initializers).
const SETTING_ENABLED: u32 = 0x011c_a4f0;
const SETTING_FLAG_COPY: u32 = 0x011c_a480;
const SETTING_NEAR: u32 = 0x011c_a450;
const SETTING_EXTRA_RANGE: u32 = 0x011c_a498;
const SETTING_THRESHOLD: u32 = 0x011c_a4ac;
/// Byte flags `0057d0a0` tests (`0x0118_c000` must be set; `0x011c_a430`
/// clear runs `TESTerrainLODManager::InitGrass`, `0057cf00`).
const ENABLED_FLAG: u32 = 0x0118_c000;
const INITIALIZED_FLAG: u32 = 0x011c_a430;
/// Where `0057d0a0` copies the byte of [`SETTING_FLAG_COPY`].
const FLAG_COPY: u32 = 0x011f_9ef0;
/// The scene node `0057d0a0` finds or makes and `0057d4f0` passes on.
const ROOT_NODE: u32 = 0x011c_a438;
/// `0057d4f0` returns at once while this byte is set.
const SKIP_FLAG: u32 = 0x011c_a534;
/// The point and shade buffers `0057de00` fills.
const POINT_BUFFER: u32 = 0x011c_a43c;
const SHADE_BUFFER: u32 = 0x011c_a440;
/// The object `0043d4d0` is called on for the per-quad limit divisor.
const DENSITY_SOURCE: u32 = 0x011c_a524;
/// Divisor and table of `0057e960`: a float at `0x011c_a4bc`, `512.0`
/// (`double`) and 512 floats at `0x011f_4ae0`.
const TABLE_PERIOD: u32 = 0x011c_a4bc;
const TABLE_SIZE: u32 = 0x0102_8338;
const TABLE: u32 = 0x011f_4ae0;

/// `double` constants.
const ZERO: u32 = 0x0101_2060;
const HALF: u32 = 0x0101_1588;
const THREE_QUARTERS: u32 = 0x0101_de30;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const QUAD_HALF_SIZE: u32 = 0x0102_31d8;
const THREE: u32 = 0x0102_1928;
const FRACTION_EPSILON: u32 = 0x0103_1148;
const CHANCE_SCALE: u32 = 0x0103_1140;
const NORMAL_LIMIT: u32 = 0x0103_0ec0;
const RED_WEIGHT: u32 = 0x0103_1138;
const GREEN_WEIGHT: u32 = 0x0103_1130;
const BLUE_WEIGHT: u32 = 0x0103_1128;
/// The `float` a normal component is clamped to.
const NORMAL_CLAMP: u32 = 0x0101_45a8;

/// A float copied through the x87 stack (`FLD`/`FSTP`): a signalling NaN
/// comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `_ftol2` on a value in ST0.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// `FISTP qword` with the rounding control set to truncate: the integer
/// indefinite value for NaN and out-of-range values.
fn truncate_to_i64(value: f64) -> i64 {
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if value.is_nan() || value >= LIMIT || value < -LIMIT {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    for i in 0..words {
        let w = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, w);
    }
}

/// `a < b` is false: what an x87 compare whose flag test treats
/// unordered like "not below" answers (NaN included).
fn not_below(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Less)
}

/// `a > b` is false, NaN included.
fn not_above(a: f64, b: f64) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Greater)
}

/// `EBP + offset` of a frame.
fn at(ebp: u32, offset: i32) -> u32 {
    ebp.wrapping_add(offset as u32)
}

// Translated from 0057d090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c_6224`.
pub fn fn_0057d090(e: &mut Engine) -> u32 {
    e.global::<u32>(WORD_SOURCE)
}

// Translated from 0057d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057d0a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Updates the points around the position (`x`, `y`, `z`) (the guard names
/// `TESTerrainLODManager.cpp`). Without an interior cell (`005f36f0` on
/// `TES`), with the setting `0x011c_a4f0` and the byte `0x0118_c000` set:
/// runs `0057cf00` once, copies a setting byte to `0x011f_9ef0`, reads the
/// range (sum of two float settings) and the near value, and stops unless
/// the near value is at least the third setting plus 0.5. It finds the
/// root node (slot `0x9c` of `00450b80(0)` with `0057d090`) or makes one
/// (`00aa13e0(0xac)`, `00a5ecb0`, `00a5b950`, slot `0xdc`), calls
/// `00b60120(0, 0, near, range)`, takes the cell of the position (`/4096`
/// toward zero, one less for a negative coordinate) and, for the 5x5 cells
/// around it (`00451900` on `TES`), hands the inner 3x3 ones closer than
/// the range (`0054fb70`) to `0057d4f0` and the others to `00b61570` with
/// their grid coordinates. Then it updates the lighting
/// (`00b5d9f0`) when `00454af0` answers true and the root's properties
/// (`0043d410`, `00a59c60`, `00a5a040`). Otherwise it calls `00b62970`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0057d0a0(
    e: &mut Engine,
    x: f32,
    y: f32,
    z: f32,
    dir_x: u32,
    dir_y: u32,
    dir_z: u32,
    angle_bias: f32,
) {
    // The argument words (EBP + 8 ..) and the guard (EBP - 0x10), the
    // offset vector (EBP - 0x28) and the update data (EBP - 0x4c).
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x58;
        let words = [
            x.to_bits(),
            y.to_bits(),
            z.to_bits(),
            dir_x,
            dir_y,
            dir_z,
            angle_bias.to_bits(),
        ];
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(ebp + 8 + 4 * i as u32, *w);
        }
        let guard = at(ebp, -0x10);
        e.call(
            SCOPE_GUARD_OPEN,
            &args![guard, 0x1cu32, 1u32, SOURCE_FILE, 0xb1u32],
        );
        terrain_update(e, ebp);
        e.call(SCOPE_GUARD_CLOSE, &args![guard]);
    });
}

/// The body of `0057d0a0` between the guard's construction and destruction.
fn terrain_update(e: &mut Engine, ebp: u32) {
    let tes = e.global::<u32>(TES_POINTER);
    let interior = e.call(0x005f_36f0, &args![tes]).u32();
    let enabled = interior == 0
        && {
            let value = e.call(BOOL_SETTING_VALUE, &args![SETTING_ENABLED]).u32();
            e.mem.u8(value) != 0
        }
        && e.global::<u8>(ENABLED_FLAG) != 0;
    if !enabled {
        e.call(0x00b6_2970, &args![]);
        return;
    }
    if e.global::<u8>(INITIALIZED_FLAG) == 0 {
        e.call(0x0057_cf00, &args![]);
    }
    let flag = e.call(BOOL_SETTING_VALUE, &args![SETTING_FLAG_COPY]).u32();
    let flag = e.mem.u8(flag);
    e.set_global(FLAG_COPY, flag);

    let near_value = e.call(FLOAT_SETTING_VALUE, &args![SETTING_NEAR]).u32();
    let extra = e
        .call(FLOAT_SETTING_VALUE, &args![SETTING_EXTRA_RANGE])
        .u32();
    let range = (e.mem.f32(near_value) as f64 + e.mem.f32(extra) as f64) as f32;
    let near_value = e.call(FLOAT_SETTING_VALUE, &args![SETTING_NEAR]).u32();
    let near = quiet(e.mem.f32(near_value));
    let near_value = e.call(FLOAT_SETTING_VALUE, &args![SETTING_NEAR]).u32();
    let threshold = e.call(FLOAT_SETTING_VALUE, &args![SETTING_THRESHOLD]).u32();
    let limit = e.mem.f32(threshold) as f64 + e.global::<f64>(HALF);
    if (e.mem.f32(near_value) as f64) < limit {
        return;
    }

    if e.global::<u32>(ROOT_NODE) == 0 {
        let scene = e.call(0x0045_0b80, &args![0u32]).u32();
        let key = fn_0057d090(e);
        let found = e.vcall(scene, 0x9c, &args![key]).u32();
        e.set_global(ROOT_NODE, found);
        if e.global::<u32>(ROOT_NODE) == 0 {
            let block = e.call(0x00aa_13e0, &args![0xacu32]).u32();
            let node = if block == 0 {
                0
            } else {
                e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
            };
            e.set_global(ROOT_NODE, node);
            let key = fn_0057d090(e);
            let root = e.global::<u32>(ROOT_NODE);
            e.call(0x00a5_b950, &args![root, key]);
            let root = e.global::<u32>(ROOT_NODE);
            e.vcall(scene, 0xdc, &args![root, 1u32]);
        }
    }
    e.call(0x00b6_0120, &args![0.0f32, 0.0f32, near, quiet(range)]);

    let x = e.mem.f32(ebp + 8);
    let y = e.mem.f32(ebp + 0xc);
    let raw = float_to_int(e, quiet(x) as f64);
    let mut cell_x = raw.wrapping_add((raw >> 31) & 0xfff) >> 12;
    let raw = float_to_int(e, quiet(y) as f64);
    let mut cell_y = raw.wrapping_add((raw >> 31) & 0xfff) >> 12;
    let zero = e.global::<f64>(ZERO);
    if (x as f64) < zero {
        cell_x = cell_x.wrapping_sub(1);
    }
    if (y as f64) < zero {
        cell_y = cell_y.wrapping_sub(1);
    }
    let tes = e.global::<u32>(TES_POINTER);
    e.call(0x0045_1900, &args![tes, cell_x, cell_y]);
    let offset = at(ebp, -0x28);
    e.call(0x0045_2dc0, &args![offset, quiet(x), quiet(y)]);
    let value = (e.mem.f32(offset) as f64 - (cell_x << 12) as f64) as f32;
    e.mem.set_f32(offset, value);
    let value = (e.mem.f32(offset + 4) as f64 - (cell_y << 12) as f64) as f32;
    e.mem.set_f32(offset + 4, value);

    for i in 0..5i32 {
        for j in 0..5i32 {
            let tes = e.global::<u32>(TES_POINTER);
            let cell = e
                .call(
                    0x0045_1900,
                    &args![
                        tes,
                        cell_x.wrapping_add(i).wrapping_sub(2),
                        cell_y.wrapping_add(j).wrapping_sub(2)
                    ],
                )
                .u32();
            if cell == 0 {
                continue;
            }
            let mut far = true;
            if i != 0 && i != 4 && j != 0 && j != 4 {
                let distance = e.call(0x0054_fb70, &args![cell, ebp + 8]).f64();
                if range as f64 > distance {
                    let root = e.global::<u32>(ROOT_NODE);
                    let [px, py, pz, dx, dy, dz] =
                        [0x8, 0xc, 0x10, 0x14, 0x18, 0x1c].map(|off| e.mem.u32(ebp + off));
                    let bias = quiet(e.mem.f32(ebp + 0x20));
                    fn_0057d4f0(
                        e,
                        cell,
                        root,
                        px,
                        py,
                        pz,
                        dx,
                        dy,
                        dz,
                        near,
                        quiet(range),
                        bias,
                    );
                    far = false;
                }
            }
            if far {
                let grid_y = e.call(0x0054_4c60, &args![cell]).u32();
                let grid_x = e.call(0x0054_4c30, &args![cell]).u32();
                e.call(0x00b6_1570, &args![grid_x, grid_y]);
            }
        }
    }

    if e.call(0x0045_4af0, &args![SETTING_FLAG_COPY]).bool() {
        let root = e.global::<u32>(ROOT_NODE);
        let scene = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_d9f0, &args![scene, root, 0u32]);
    }
    if e.global::<u32>(ROOT_NODE) != 0 {
        let update = at(ebp, -0x4c);
        e.call(0x0043_d410, &args![update, 0.0f32, 0u32, 0u32]);
        let root = e.global::<u32>(ROOT_NODE);
        e.call(0x00a5_9c60, &args![root, update]);
        let root = e.global::<u32>(ROOT_NODE);
        e.call(0x00a5_a040, &args![root]);
    }
}

// Translated from 0057d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057d4f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Places the points of one cell (nothing while the byte `0x011c_a534` is
/// set; nothing without a world space `0054ddd0` or land `00546fb0`). It
/// builds a view frustum from the camera's field of view (`0099e040`,
/// widened by twice `angle_bias`, tangent `0057dd70`, aspect from
/// `004dee70`/`004dee10`) and the camera frustum's near and far planes,
/// then for each of the four quads with an object (`0045c9a0`) measures the
/// distance from the position to the object's bound minus its radius. When
/// the quad's geometry kind (`0044ddc0`) is 0 or 1 or its slot `0x9c`
/// answers true for the planes, and `range` is greater than that distance,
/// every point of the quad grid (step `0057ce60`, `00b617b0` false) goes to
/// `0057de00` with the properties of its form; otherwise each grid point's
/// key (`0057ddb0`) goes to `00b61370`. `00470470` runs after each quad.
#[allow(clippy::too_many_arguments)]
pub fn fn_0057d4f0(
    e: &mut Engine,
    cell: u32,
    root: u32,
    x: u32,
    y: u32,
    z: u32,
    dir_x: u32,
    dir_y: u32,
    dir_z: u32,
    near: f32,
    range: f32,
    angle_bias: f32,
) {
    if e.global::<u8>(SKIP_FLAG) != 0 {
        return;
    }
    e.with_stack(0x240, |e, frame| {
        let ebp = frame.addr() + 0x200;
        let words = [
            cell,
            root,
            x,
            y,
            z,
            dir_x,
            dir_y,
            dir_z,
            near.to_bits(),
            range.to_bits(),
            angle_bias.to_bits(),
        ];
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(ebp + 8 + 4 * i as u32, *w);
        }
        place_cell_points(e, ebp);
    });
}

/// The body of `0057d4f0` on its frame.
fn place_cell_points(e: &mut Engine, ebp: u32) {
    let cell = e.mem.u32(ebp + 8);
    let list = at(ebp, -0x178);
    e.call(0x0096_a2d0, &args![list]);
    e.call(0x0053_a510, &args![at(ebp, -0x94)]);
    let mut stack_count = 0u16;
    let mut upper_byte = 0u8;
    let mut lower_byte = 0u8;
    let mut kind = 8u32;
    let point = at(ebp, -0x148);
    e.call(0x0068_15c0, &args![at(ebp, -0x40)]);
    e.call(0x0068_15c0, &args![at(ebp, -0x15c)]);
    e.call(0x0068_15c0, &args![point]);
    let start = e.call(0x0057_ce60, &args![]).i32();
    let step = start << 1;
    if cell == 0
        || e.call(0x0054_ddd0, &args![cell]).u32() == 0
        || e.call(0x0054_6fb0, &args![cell]).u32() == 0
    {
        e.call(0x0046_ffb0, &args![list]);
        return;
    }
    let water = e.call(0x0054_71e0, &args![cell]).f64() as f32;
    let land = e.call(0x0054_6fb0, &args![cell]).u32();
    let grid_x = e.call(0x0054_4c30, &args![cell]).u32();
    let grid_y = e.call(0x0054_4c60, &args![cell]).u32();
    e.mem.set_f32(ebp + 0x24, 0.0);
    e.call(0x004a_0c10, &args![ebp + 0x1c]);
    let bias = e.mem.f32(ebp + 0x30) as f64;
    e.mem.set_f32(ebp + 0x30, (bias + bias) as f32);

    let frustum = at(ebp, -0x28);
    e.call(0x00a7_1b70, &args![frustum, 0u32]);
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    let width = e.call(0x004d_ee70, &args![renderer]).u32() as f64 as f32;
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    let height = e.call(0x004d_ee10, &args![renderer]).u32() as f64 as f32;
    let ratio = width as f64 / height as f64;
    let aspect = ratio as f32;
    let scale = (e.global::<f64>(THREE_QUARTERS) / ratio) as f32;
    let camera = e.call(0x0045_c670, &args![]).u32();
    let fov = e.call(0x0099_e040, &args![camera]).f64();
    let angle = (fov * e.global::<f64>(DEGREES_TO_RADIANS) * e.global::<f64>(HALF)
        + e.mem.f32(ebp + 0x30) as f64) as f32;
    let t = fn_0057dd70(e, quiet(angle)) as f64;
    e.mem.set_f32(frustum, (-t * scale as f64) as f32);
    let t = fn_0057dd70(e, quiet(angle)) as f64;
    e.mem.set_f32(frustum + 4, (t * scale as f64) as f32);
    let t = fn_0057dd70(e, quiet(angle)) as f64;
    e.mem
        .set_f32(frustum + 0xc, (-t * aspect as f64 * scale as f64) as f32);
    let t = fn_0057dd70(e, quiet(angle)) as f64;
    e.mem
        .set_f32(frustum + 8, (t * aspect as f64 * scale as f64) as f32);
    let camera = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0066_29f0, &args![camera]).u32();
    let source = e.call(0x0045_bbe0, &args![node]).u32();
    let copy = at(ebp, -0x128);
    copy_words(e, copy, source, 7);
    let far_plane = quiet(e.mem.f32(copy + 0x14));
    e.mem.set_f32(frustum + 0x14, far_plane);
    let near_plane = quiet(e.mem.f32(copy + 0x10));
    e.mem.set_f32(frustum + 0x10, near_plane);
    let planes = at(ebp, -0x10c);
    e.call(0x0045_c620, &args![planes]);
    let camera = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0066_29f0, &args![camera]).u32();
    let transform = e.call(0x0046_1130, &args![node]).u32();
    e.call(0x00a7_4e10, &args![planes, frustum, transform]);

    for quad in 0..4u32 {
        let object = e.call(0x0045_c9a0, &args![cell, quad]).u32();
        if object != 0 {
            e.call(
                0x0053_b450,
                &args![land, at(ebp, -0x198), 0u32, 0u32, 0u32, 0u32],
            );
            let bound_copy = at(ebp, -0x18c);
            let bound = e.call(0x0043_d450, &args![object]).u32();
            copy_words(e, bound_copy, bound, 4);
            copy_words(e, at(ebp, -0x1a4), 0x011f_426c, 3);
            let center = e.call(0x0068_15c0, &args![bound_copy]).u32();
            let difference = e
                .call(0x0043_9ef0, &args![ebp + 0x10, at(ebp, -0x1d8), center])
                .u32();
            let length = e.call(0x0045_7990, &args![difference]).f64();
            let radius = e.call(0x0084_d030, &args![bound_copy]).f64();
            let gap = (length - radius) as f32;
            let children = e.call(0x0045_6fc0, &args![cell, 2u32]).u32();
            let child = e.call(0x0043_b4a0, &args![children, quad]).u32();
            let data = e.call(0x0066_29f0, &args![child]).u32();
            let geometry = e.call(0x0043_b230, &args![data]).u32();
            let geometry_kind = e.call(0x0044_ddc0, &args![geometry]).u32();
            let data = e.call(0x0066_29f0, &args![child]).u32();
            let geometry = e.call(0x0043_b230, &args![data]).u32();
            let visible = e.vcall(geometry, 0x9c, &args![planes]).bool();
            let candidate = geometry_kind == 1 || geometry_kind == 0 || visible;
            let range = e.mem.f32(ebp + 0x2c);
            if candidate && range as f64 > gap as f64 {
                let mut a = start;
                while a < 0x10 {
                    let mut b = start;
                    while b < 0x10 {
                        let index = b.wrapping_mul(0x11).wrapping_add(a);
                        let vertex = e
                            .call(
                                0x0053_b450,
                                &args![
                                    land,
                                    at(ebp, -0x1e4),
                                    quad & 0xff,
                                    index as u32 & 0xffff,
                                    0u32,
                                    0u32
                                ],
                            )
                            .u32();
                        copy_words(e, point, vertex, 3);
                        let point_z = quiet(e.mem.f32(ebp + 0x18));
                        e.mem.set_f32(point + 8, point_z);
                        shift_to_quad_center(e, point);
                        let key = fn_0057ddb0(e, point);
                        let entries = e.call(0x0053_ca80, &args![land, quad, index]).u32();
                        let mut k = 0u32;
                        loop {
                            if entries == 0 {
                                break;
                            }
                            let entry = e.mem.u32(entries.wrapping_add(k * 4));
                            if entry == 0 || k >= 0x10 {
                                break;
                            }
                            let entry = e.mem.u32(entries.wrapping_add(k * 4));
                            if e.mem.u32(entry) == 0 {
                                break;
                            }
                            let entry = e.mem.u32(entries.wrapping_add(k * 4));
                            if entry != 0 {
                                let item = e.mem.u32(entry + 4);
                                if !e.call(0x00b6_17b0, &args![item, key]).bool() {
                                    let entry = e.mem.u32(entries.wrapping_add(k * 4));
                                    let item = e.mem.u32(entry + 4);
                                    let form = e.call(0x0048_39c0, &args![item]).u32();
                                    if form != 0 {
                                        stack_count = e.vcall(form, 0x1a0, &args![]).u16();
                                        kind = e.vcall(form, 0x1a8, &args![]).u32();
                                        lower_byte = e.vcall(form, 0x190, &args![]).u8();
                                        upper_byte = e.vcall(form, 0x188, &args![]).u8();
                                    }
                                    let entry = e.mem.u32(entries.wrapping_add(k * 4));
                                    let [root, px, py, pz] =
                                        [0xc, 0x10, 0x14, 0x18].map(|off| e.mem.u32(ebp + off));
                                    let near = quiet(e.mem.f32(ebp + 0x28));
                                    let range = quiet(e.mem.f32(ebp + 0x2c));
                                    fn_0057de00(
                                        e,
                                        grid_x,
                                        grid_y,
                                        quad,
                                        land,
                                        key,
                                        point,
                                        root,
                                        px,
                                        py,
                                        pz,
                                        entry,
                                        entry.wrapping_add(0x20),
                                        near,
                                        range,
                                        stack_count,
                                        kind,
                                        quiet(water),
                                        upper_byte,
                                        lower_byte,
                                    );
                                }
                            }
                            k += 1;
                        }
                        b = b.wrapping_add(step);
                    }
                    a = a.wrapping_add(step);
                }
            } else {
                let mut a = start;
                while a < 0x10 {
                    let mut b = start;
                    while b < 0x10 {
                        let index = b.wrapping_mul(0x11).wrapping_add(a);
                        let vertex = e
                            .call(
                                0x0053_b450,
                                &args![
                                    land,
                                    at(ebp, -0x1f0),
                                    quad & 0xff,
                                    index as u32 & 0xffff,
                                    0u32,
                                    0u32
                                ],
                            )
                            .u32();
                        copy_words(e, point, vertex, 3);
                        shift_to_quad_center(e, point);
                        let key = fn_0057ddb0(e, point);
                        e.call(0x00b6_1370, &args![key]);
                        b = b.wrapping_add(step);
                    }
                    a = a.wrapping_add(step);
                }
            }
        }
        e.call(0x0047_0470, &args![list]);
    }
    e.call(0x0046_ffb0, &args![list]);
}

/// Moves a grid point's x and y by -256 (the `double` at `0x0102_31d8`).
fn shift_to_quad_center(e: &mut Engine, point: u32) {
    let half = e.global::<f64>(QUAD_HALF_SIZE);
    let value = (e.mem.f32(point) as f64 - half) as f32;
    e.mem.set_f32(point, value);
    let value = (e.mem.f32(point + 4) as f64 - half) as f32;
    e.mem.set_f32(point + 4, value);
}

// Translated from 0057dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0057dd90` of `value`, its `float` result passed on in ST0.
pub fn fn_0057dd70(e: &mut Engine, value: f32) -> f32 {
    fn_0057dd90(e, quiet(value))
}

// Translated from 0057dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00eca590` of `value` widened to `double`, rounded to `float`.
pub fn fn_0057dd90(e: &mut Engine, value: f32) -> f32 {
    e.call(0x00ec_a590, &args![quiet(value) as f64]).f64() as f32
}

// Translated from 0057ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of a point: `00404040` of its x and of its y, each truncated to
/// an integer (`_ftol2`); x in the high 16 bits, y in the low 16.
pub fn fn_0057ddb0(e: &mut Engine, point: u32) -> u32 {
    let x = quiet(e.mem.f32(point));
    let value = e.call(0x0040_4040, &args![x]).f64();
    let high = float_to_int(e, value);
    let y = quiet(e.mem.f32(point + 4));
    let value = e.call(0x0040_4040, &args![y]).f64();
    let low = float_to_int(e, value);
    ((high as u32) << 16) | (low as u32 & 0xffff)
}

// Translated from 0057de00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057de00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Scatters the points of one grid entry (the guard names
/// `TESTerrainLODManager.cpp`). Nothing happens when `00b61940(entry, key)`
/// answers true or `005394a0` on the land answers false. The bounds are
/// `0057e960` of `upper_byte` and `lower_byte` degrees. It divides the
/// span (`0057ce60` << 8) into `span / entry+8` steps (at most the span
/// divided by the word `0043d4d0` returns), and for each step position:
/// interpolates a chance from the 3x3 floats at `heights`, rolls
/// `00944460(0, 0x7fff)` against it, jitters the position (`004dfea0`
/// scaled by `entry+8`, snapped by `00404040`), gets its height
/// (`004572e0`) and keeps it when the `kind` test against `water` and
/// `stack_count` passes (0..5, anything else passes) and the normal's z
/// lies between the bounds. A kept point is offset by half its normal
/// (each component clamped below `0x0101_45a8`) and stored in the buffer
/// at `0x011c_a43c` with a brightness (weighted colour sum) in the buffer
/// at `0x011c_a440`. The points kept go to `00b62de0`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0057de00(
    e: &mut Engine,
    grid_x: u32,
    grid_y: u32,
    _unused_3: u32,
    land: u32,
    key: u32,
    point: u32,
    root: u32,
    x: u32,
    y: u32,
    z: u32,
    entry: u32,
    heights: u32,
    _unused_13: f32,
    range: f32,
    stack_count: u16,
    kind: u32,
    water: f32,
    upper_byte: u8,
    lower_byte: u8,
) {
    e.with_stack(0x200, |e, frame| {
        let ebp = frame.addr() + 0x1a8;
        let words = [
            grid_x,
            grid_y,
            _unused_3,
            land,
            key,
            point,
            root,
            x,
            y,
            z,
            entry,
            heights,
            _unused_13.to_bits(),
            range.to_bits(),
            u32::from(stack_count),
            kind,
            water.to_bits(),
            u32::from(upper_byte),
            u32::from(lower_byte),
        ];
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(ebp + 8 + 4 * i as u32, *w);
        }
        let guard = at(ebp, -0x24);
        e.call(
            SCOPE_GUARD_OPEN,
            &args![guard, 0x1cu32, 1u32, SOURCE_FILE, 0x20au32],
        );
        let holder = at(ebp, -0x90);
        e.call(0x0063_3c90, &args![holder, 0u32]);
        scatter_points(e, ebp);
        e.call(0x0045_cec0, &args![holder]);
        e.call(SCOPE_GUARD_CLOSE, &args![guard]);
    });
}

/// The body of `0057de00` on its frame, between the holder's construction
/// and destruction.
fn scatter_points(e: &mut Engine, ebp: u32) {
    e.call(0x0068_15c0, &args![at(ebp, -0x20)]);
    let degrees = e.global::<f64>(DEGREES_TO_RADIANS);
    let upper_angle = (e.mem.u8(ebp + 0x4c) as f64 * degrees) as f32;
    let upper = fn_0057e960(e, quiet(upper_angle));
    let lower_angle = (e.mem.u8(ebp + 0x50) as f64 * degrees) as f32;
    let lower = fn_0057e960(e, quiet(lower_angle));
    let range = e.mem.f32(ebp + 0x3c) as f64;
    e.mem.set_f32(ebp + 0x3c, (range * range) as f32);
    let key = e.mem.u32(ebp + 0x18);
    let land = e.mem.u32(ebp + 0x14);
    let entry = e.mem.u32(ebp + 0x30);
    let known = e.call(0x00b6_1940, &args![entry, key]).bool();
    if known || !e.call(0x0053_94a0, &args![land]).bool() {
        return;
    }
    let span = (e.call(0x0057_ce60, &args![]).i32() << 8) as f32;
    let spacing = e.mem.f32(entry + 8);
    let mut steps = float_to_int(e, span as f64 / spacing as f64) as u16;
    let total = e.call(0x0057_ce60, &args![]).i32() << 8;
    let divisor = e.call(0x0043_d4d0, &args![DENSITY_SOURCE]).u32();
    let limit = (total / e.mem.i32(divisor)) as u16;
    if steps > limit {
        steps = limit;
    }
    let land_data = at(ebp, -0x74);
    e.call(0x0053_a510, &args![land_data]);
    let mut count = 0u16;
    let position = at(ebp, -0xac);
    let height = at(ebp, -0xe0);
    let normal = at(ebp, -0x104);
    let surface = at(ebp, -0xd8);
    let colour = at(ebp, -0xf8);
    for r in 0..i32::from(steps) {
        for c in 0..i32::from(steps) {
            let n = f64::from(steps);
            let along_y = (c as f64 * (span as f64 / n)) as f32;
            let along_x = (r as f64 * (span as f64 / n)) as f32;
            let offset = e
                .call(
                    0x0041_6870,
                    &args![at(ebp, -0x11c), quiet(along_x), quiet(along_y), 0.0f32],
                )
                .u32();
            let base_point = e.mem.u32(ebp + 0x1c);
            e.call(0x0043_9e90, &args![base_point, position, offset]);
            let three = e.global::<f64>(THREE);
            let mut v = (c as f64 * (three / n)) as f32;
            let mut u = (r as f64 * (three / n)) as f32;
            let col = e.call(0x0040_6d90, &args![quiet(u)]).i32();
            let row = e.call(0x0040_6d90, &args![quiet(v)]).i32();
            u = (u as f64 - col as f64) as f32;
            v = (v as f64 - row as f64) as f32;
            let chance = interpolate_chance(e, ebp, row, col, u, v);
            let chance = truncate_to_i64(chance as f64 * e.global::<f64>(CHANCE_SCALE)) as i32;
            if chance == 0 {
                continue;
            }
            let roll = e.call(0x0094_4460, &args![0u32, 0x7fffu32]).i32();
            if roll >= chance {
                continue;
            }
            e.mem.set_f32(height, 0.0);
            let jitter = e.call(0x004d_fea0, &args![]).f64() as f32;
            let px =
                (jitter as f64 * e.mem.f32(entry + 8) as f64 + e.mem.f32(position) as f64) as f32;
            e.mem.set_f32(position, px);
            let jitter = e.call(0x004d_fea0, &args![]).f64() as f32;
            let py = (jitter as f64 * e.mem.f32(entry + 8) as f64 + e.mem.f32(position + 4) as f64)
                as f32;
            e.mem.set_f32(position + 4, py);
            let px = quiet(e.mem.f32(position));
            let px = e.call(0x0040_4040, &args![px]).f64() as f32;
            e.mem.set_f32(position, px);
            let py = quiet(e.mem.f32(position + 4));
            let py = e.call(0x0040_4040, &args![py]).f64() as f32;
            e.mem.set_f32(position + 4, py);
            e.mem.set_f32(position + 8, 0.0);
            let on_land = e
                .call(0x0053_b550, &args![land, land_data, position, 0u32])
                .bool();
            let tes = e.global::<u32>(TES_POINTER);
            e.call(0x0045_72e0, &args![tes, position, height]);
            if !passes_kind_test(e, ebp, e.mem.f32(height)) {
                continue;
            }

            e.call(0x0043_9ef0, &args![position, at(ebp, -0x110), ebp + 0x24]);
            let h = quiet(e.mem.f32(height));
            let pz = e.call(0x0040_4040, &args![h]).f64() as f32;
            e.mem.set_f32(position + 8, pz);
            e.call(0x0068_15c0, &args![normal]);
            e.call(0x0068_15c0, &args![surface]);
            if on_land {
                e.call(0x0053_caf0, &args![land, land_data, normal, surface]);
            } else {
                let tes = e.global::<u32>(TES_POINTER);
                e.call(0x0045_73f0, &args![tes, position, normal, surface]);
            }
            let entry = e.mem.u32(ebp + 0x30);
            if e.mem.u8(entry + 0x1e) != 0 && e.mem.u8(entry + 0x1c) != 0 {
                copy_words(e, normal, surface, 3);
            } else if e.mem.u8(entry + 0x1e) != 0 && e.mem.u8(entry + 0x1c) == 0 {
                let sum = e
                    .call(0x0043_9e90, &args![normal, at(ebp, -0x128), surface])
                    .u32();
                copy_words(e, normal, sum, 3);
                e.call(0x004a_0c10, &args![normal]);
            }
            let normal_z = e.mem.f32(normal + 8) as f64;
            if lower as f64 > normal_z || (upper as f64) < normal_z {
                continue;
            }
            let half = e.global::<f64>(HALF);
            let ceiling = e.global::<f64>(NORMAL_LIMIT);
            for i in 0..3 {
                let slot = normal + 4 * i;
                let value = (e.mem.f32(slot) as f64 * half + half) as f32;
                e.mem.set_f32(slot, value);
                if not_below(e.mem.f32(slot) as f64, ceiling) {
                    let clamp = quiet(e.global::<f32>(NORMAL_CLAMP));
                    e.mem.set_f32(slot, clamp);
                }
            }
            for i in 0..3 {
                let value =
                    (e.mem.f32(position + 4 * i) as f64 + e.mem.f32(normal + 4 * i) as f64) as f32;
                e.mem.set_f32(position + 4 * i, value);
            }
            e.call(0x0041_4430, &args![colour, 0.0f32, 0.0f32, 0.0f32, 0.0f32]);
            if on_land {
                e.call(0x0053_fa10, &args![land, land_data, colour]);
            } else {
                let tes = e.global::<u32>(TES_POINTER);
                e.call(0x0045_7520, &args![tes, position, colour]);
            }
            let brightness = (e.mem.f32(colour) as f64 * e.global::<f64>(RED_WEIGHT)
                + e.mem.f32(colour + 4) as f64 * e.global::<f64>(GREEN_WEIGHT)
                + e.mem.f32(colour + 8) as f64 * e.global::<f64>(BLUE_WEIGHT))
                as f32;
            let moved = e
                .call(0x0043_9ef0, &args![position, at(ebp, -0x134), ebp + 0x24])
                .u32();
            copy_words(e, at(ebp, -0x110), moved, 3);
            let points = e.global::<u32>(POINT_BUFFER);
            copy_words(e, points.wrapping_add(u32::from(count) * 0xc), position, 3);
            let shades = e.global::<u32>(SHADE_BUFFER);
            e.mem
                .set_f32(shades.wrapping_add(u32::from(count) * 4), quiet(brightness));
            count = count.wrapping_add(1);
        }
    }
    if count != 0 {
        let points = e.global::<u32>(POINT_BUFFER);
        let shades = e.global::<u32>(SHADE_BUFFER);
        let [first, second, root, entry] = [0x8, 0xc, 0x20, 0x30].map(|off| e.mem.u32(ebp + off));
        e.call(
            0x00b6_2de0,
            &args![
                key,
                first,
                second,
                root,
                entry,
                points,
                shades,
                u32::from(count)
            ],
        );
    }
}

/// The bilinear chance of `0057de00` from the 3x3 floats at `heights`
/// (`EBP + 0x34`): row `row`, column `col`, fractions `u` (column) and `v`
/// (row); a neighbour is read only when its fraction exceeds 0.01, and the
/// last row and column repeat themselves.
fn interpolate_chance(e: &mut Engine, ebp: u32, row: i32, col: i32, u: f32, v: f32) -> f32 {
    let heights = e.mem.u32(ebp + 0x34);
    let read = |e: &Engine, index: i32, extra: u32| {
        quiet(
            e.mem.f32(
                heights
                    .wrapping_add((index as u32).wrapping_mul(4))
                    .wrapping_add(extra),
            ),
        )
    };
    let here = row.wrapping_mul(3).wrapping_add(col);
    let below = row.wrapping_add(1).wrapping_mul(3).wrapping_add(col);
    let epsilon = e.global::<f64>(FRACTION_EPSILON);
    let h00 = read(e, here, 0);
    let mut h10 = 0.0f32;
    let mut h11 = 0.0f32;
    let mut h01 = 0.0f32;
    if u as f64 > epsilon {
        h10 = if col < 2 {
            read(e, here, 4)
        } else {
            read(e, here, 0)
        };
        if v as f64 > epsilon {
            h11 = if row < 2 {
                if col < 2 {
                    read(e, below, 4)
                } else {
                    read(e, below, 0)
                }
            } else if col < 2 {
                read(e, here, 4)
            } else {
                read(e, here, 0)
            };
        }
    }
    if v as f64 > epsilon {
        h01 = if row < 2 {
            read(e, below, 0)
        } else {
            read(e, here, 0)
        };
    }
    let (u, v) = (u as f64, v as f64);
    let top = (1.0 - u) * h00 as f64 + h10 as f64 * u;
    let bottom = (1.0 - u) * h01 as f64 + h11 as f64 * u;
    (top * (1.0 - v) + bottom * v) as f32
}

/// The `kind` test of `0057de00` (the switch through the table at
/// `0057e93c`) on the height `h` against `water` (`EBP + 0x48`) and the
/// depth `stack_count` (`EBP + 0x40`); NaN comparisons fall as the flag
/// tests do.
fn passes_kind_test(e: &mut Engine, ebp: u32, h: f32) -> bool {
    let water = e.mem.f32(ebp + 0x48) as f64;
    let depth = f64::from(e.mem.u16(ebp + 0x40));
    let h = h as f64;
    match e.mem.u32(ebp + 0x44) {
        0 => not_below(h, water - depth),
        1 => not_above(water, h) && not_above(h, water + depth),
        2 => not_above(h, water - depth),
        3 => not_below(water, h) && not_below(h, water - depth),
        4 => not_below(h, water + depth) || not_above(h, water - depth),
        5 => not_above(h, water + depth) && not_below(h, water - depth),
        _ => true,
    }
}

// Translated from 0057e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Table lookup: `angle * (512.0 / period)` truncated (`_ftol2`), masked to
/// 0..512, indexes the 512 floats at `0x011f_4ae0` (`period` is the float
/// at `0x011c_a4bc`).
pub fn fn_0057e960(e: &mut Engine, angle: f32) -> f32 {
    let scale = e.global::<f64>(TABLE_SIZE) / e.global::<f32>(TABLE_PERIOD) as f64;
    let index = float_to_int(e, scale * angle as f64) as u32 & 0x1ff;
    quiet(e.global::<f32>(TABLE + index * 4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0057_d090, fn_0057d090() -> u32),
        entry!(0x0057_d0a0, fn_0057d0a0(f32, f32, f32, u32, u32, u32, f32)),
        entry!(
            0x0057_d4f0,
            fn_0057d4f0(u32, u32, u32, u32, u32, u32, u32, u32, f32, f32, f32)
        ),
        entry!(0x0057_dd70, fn_0057dd70(f32) -> f32),
        entry!(0x0057_dd90, fn_0057dd90(f32) -> f32),
        entry!(0x0057_ddb0, fn_0057ddb0(u32) -> u32),
        entry!(
            0x0057_de00,
            fn_0057de00(
                u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, f32, f32, u16, u32,
                f32, u8, u8
            )
        ),
        entry!(0x0057_e960, fn_0057e960(f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn zero(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(value: u32) -> Ret {
        value.into_ret()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, zero);
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn position_of(e: &Engine, addr: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .position(|(a, _)| *a == addr)
            .unwrap()
    }

    /// The pages of the constants and globals, the constants' values (the
    /// exe's bit patterns) and `_ftol2` (truncation).
    fn base_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_4000,
            0x0101_d000,
            0x0102_1000,
            0x0102_3000,
            0x0102_8000,
            0x0103_0000,
            0x0103_1000,
            0x0118_c000,
            0x011c_6000,
            0x011c_a000,
            0x011d_e000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x011f_4000, 0x2000);
        e.set_global(HALF, 0.5f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(THREE_QUARTERS, 0.75f64);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(QUAD_HALF_SIZE, 256.0f64);
        e.set_global(THREE, 3.0f64);
        e.set_global(FRACTION_EPSILON, f64::from_bits(0x3f84_7ae1_47ae_147b));
        e.set_global(CHANCE_SCALE, 32768.0f64);
        e.set_global(NORMAL_LIMIT, f64::from_bits(0x3fef_0a3d_8000_0000));
        e.set_global(RED_WEIGHT, 0.3f64);
        e.set_global(GREEN_WEIGHT, 0.6f64);
        e.set_global(BLUE_WEIGHT, 0.1f64);
        e.set_global(NORMAL_CLAMP, f32::from_bits(0x3f78_51ec));
        e.set_global(TABLE_SIZE, 512.0f64);
        e.set_global(TABLE_PERIOD, 512.0f32);
        e.set_global(TES_POINTER, 0x0050_0000u32);
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e
    }

    #[test]
    fn fn_0057d090_reads_the_word() {
        let mut e = base_engine();
        e.set_global(WORD_SOURCE, 0x1234_5678u32);
        assert_eq!(e.call(0x0057_d090, &args![]).u32(), 0x1234_5678);
    }

    #[test]
    fn fn_0057dd90_and_fn_0057dd70_widen_call_and_round() {
        let mut e = base_engine();
        e.register(0x00ec_a590, |_, a| st0(f64::take(a, &mut 0) * 2.0 + 1e-12));
        assert_eq!(e.call(0x0057_dd90, &args![1.5f32]).f32(), 3.0);
        assert_eq!(e.call(0x0057_dd70, &args![0.25f32]).f32(), 0.5);
        // A signalling NaN reaches the callee quiet.
        e.call_log = Some(vec![]);
        e.call(0x0057_dd70, &args![f32::from_bits(0x7f80_0001)]);
        assert_eq!(
            calls_to(&e, 0x00ec_a590),
            vec![vec![0x2000_0000, 0x7ff8_0000]]
        );
    }

    #[test]
    fn fn_0057ddb0_packs_the_truncated_coordinates() {
        let mut e = base_engine();
        e.register(0x0040_4040, |_, a| st0(f32::from_bits(a[0]) as f64));
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 3.7);
        e.mem.set_f32(point + 4, -2.2);
        assert_eq!(e.call(0x0057_ddb0, &args![point]).u32(), 0x0003_fffe);
        e.mem.set_f32(point, -1.0);
        e.mem.set_f32(point + 4, 70000.0);
        assert_eq!(
            e.call(0x0057_ddb0, &args![point]).u32(),
            0xffff_0000 | (70000 & 0xffff)
        );
    }

    #[test]
    fn fn_0057e960_wraps_the_table_index() {
        let mut e = base_engine();
        e.set_global(TABLE_PERIOD, 2.0f32);
        e.set_global(TABLE + 128 * 4, 0.75f32);
        e.set_global(TABLE + 256 * 4, -0.5f32);
        e.set_global(TABLE + 4, f32::from_bits(0x7f80_0001));
        assert_eq!(e.call(0x0057_e960, &args![0.5f32]).f32(), 0.75);
        // 3.0 * 256 = 768, masked to 256.
        assert_eq!(e.call(0x0057_e960, &args![3.0f32]).f32(), -0.5);
        // The table's float is loaded with FLD: a signalling NaN comes out quiet.
        let value = fn_0057e960(&mut e, 0.004);
        assert_eq!(value.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn truncate_to_i64_matches_fistp() {
        assert_eq!(truncate_to_i64(-3.7), -3);
        assert_eq!(truncate_to_i64(16384.9), 16384);
        assert_eq!(truncate_to_i64(f64::NAN), i64::MIN);
        assert_eq!(truncate_to_i64(1e19), i64::MIN);
        assert_eq!(truncate_to_i64(-1e19), i64::MIN);
    }

    #[test]
    fn kind_tests_follow_the_flag_tests() {
        let mut e = base_engine();
        let ebp = e.mem.alloc(0x60);
        e.mem.set_f32(ebp + 0x48, 5.0);
        e.mem.set_u16(ebp + 0x40, 2);
        let nan = f32::NAN;
        let cases: [(u32, f32, bool); 23] = [
            (0, 2.0, false),
            (0, 3.0, true),
            (0, nan, true),
            (1, 4.0, false),
            (1, 5.0, true),
            (1, 6.0, true),
            (1, 8.0, false),
            (1, nan, true),
            (2, 2.0, true),
            (2, 4.0, false),
            (2, nan, true),
            (3, 6.0, false),
            (3, 4.0, true),
            (3, 2.0, false),
            (3, nan, true),
            (4, 8.0, true),
            (4, 2.0, true),
            (4, 5.0, false),
            (5, 5.0, true),
            (5, 8.0, false),
            (5, 2.0, false),
            (5, nan, true),
            (9, 100.0, true),
        ];
        for (kind, h, want) in cases {
            e.mem.set_u32(ebp + 0x44, kind);
            assert_eq!(passes_kind_test(&mut e, ebp, h), want, "kind {kind} h {h}");
        }
    }

    #[test]
    fn interpolation_picks_neighbours_by_fraction() {
        let mut e = base_engine();
        let ebp = e.mem.alloc(0x60);
        let heights = e.mem.alloc(9 * 4 + 4);
        for i in 0..9 {
            e.mem.set_f32(heights + 4 * i, i as f32);
        }
        e.mem.set_u32(ebp + 0x34, heights);
        assert_eq!(interpolate_chance(&mut e, ebp, 0, 0, 0.5, 0.5), 2.0);
        // The last row and column repeat themselves.
        assert_eq!(interpolate_chance(&mut e, ebp, 2, 2, 0.5, 0.5), 8.0);
        // A fraction of at most 0.01 reads no neighbour in that direction.
        assert_eq!(interpolate_chance(&mut e, ebp, 1, 2, 0.0, 0.5), 6.5);
        assert_eq!(
            interpolate_chance(&mut e, ebp, 1, 1, 0.01, 0.0),
            (4.0 * (1.0 - 0.01f32 as f64)) as f32
        );
    }

    const SCENE_VTABLE: u32 = 0x6000_0000;
    const SCENE_FIND: u32 = 0x6100_009c;
    const SCENE_ADD: u32 = 0x6100_00dc;

    /// The doubles of `0057d0a0`; `0057d4f0` stops at its world-space check.
    fn update_engine(near: f32, threshold: f32, interior: u32) -> (Engine, u32) {
        let mut e = base_engine();
        let mut slots = vec![0u32; 0xe0 / 4];
        slots[0x9c / 4] = SCENE_FIND;
        slots[0xdc / 4] = SCENE_ADD;
        e.put_vtable(SCENE_VTABLE, &slots);
        let scene = e.mem.alloc(8);
        e.mem.set_u32(scene, SCENE_VTABLE);
        e.set_global(WORD_SOURCE, 0x55u32);
        e.set_global(SETTING_ENABLED, 1u8);
        e.set_global(SETTING_FLAG_COPY, 7u8);
        e.set_global(ENABLED_FLAG, 1u8);
        e.set_global(SETTING_NEAR, near);
        e.set_global(SETTING_EXTRA_RANGE, 50.0f32);
        e.set_global(SETTING_THRESHOLD, threshold);
        stub(
            &mut e,
            &[
                SCOPE_GUARD_OPEN,
                SCOPE_GUARD_CLOSE,
                0x0057_cf00,
                SCENE_FIND,
                SCENE_ADD,
                0x00a5_b950,
                0x00b6_0120,
                0x0045_2dc0,
                0x0096_a2d0,
                0x0053_a510,
                0x0068_15c0,
                0x0046_ffb0,
                0x0054_ddd0,
                0x00b6_1570,
                0x00b5_d9f0,
                0x0043_d410,
                0x00a5_9c60,
                0x00a5_a040,
                0x00b6_2970,
            ],
        );
        e.register(0x0057_ce60, |_, _| ret(8));
        e.register_double(0x005f_36f0, move |_, _| ret(interior));
        e.register(BOOL_SETTING_VALUE, |_, a| ret(a[0]));
        e.register(FLOAT_SETTING_VALUE, |_, a| ret(a[0]));
        e.register_double(0x0045_0b80, move |_, _| ret(scene));
        e.register(0x00aa_13e0, |_, _| ret(0x7700));
        e.register(0x00a5_ecb0, |_, a| ret(a[0] + 1));
        e.register(0x0045_1900, |_, a| {
            let (x, y) = (a[1] as i32, a[2] as i32);
            ret(match (x, y) {
                (1, -1) => 0xa000,
                (-1, -3) => 0xb000,
                _ => 0,
            })
        });
        e.register(0x0054_fb70, |_, _| st0(10.0));
        e.register(0x0054_4c30, |_, _| ret(0x31));
        e.register(0x0054_4c60, |_, _| ret(0x32));
        e.register(0x0045_4af0, |_, _| ret(1));
        (e, scene)
    }

    #[test]
    fn fn_0057d0a0_updates_the_cells_around_the_position() {
        let (mut e, scene) = update_engine(100.0, 10.0, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0057_d0a0,
            &args![5000.0f32, -100.0f32, 7.0f32, 1u32, 2u32, 3u32, 0.0f32],
        );
        let open = calls_to(&e, SCOPE_GUARD_OPEN);
        assert_eq!(&open[0][1..], &[0x1c, 1, SOURCE_FILE, 0xb1]);
        assert_eq!(calls_to(&e, SCOPE_GUARD_CLOSE), vec![vec![open[0][0]]]);
        assert_eq!(calls_to(&e, 0x0057_cf00).len(), 1);
        assert_eq!(e.global::<u8>(FLAG_COPY), 7);
        assert_eq!(calls_to(&e, SCENE_FIND), vec![vec![scene, 0x55]]);
        assert_eq!(e.global::<u32>(ROOT_NODE), 0x7701);
        assert_eq!(calls_to(&e, 0x00a5_ecb0), vec![vec![0x7700, 0]]);
        assert_eq!(calls_to(&e, 0x00a5_b950), vec![vec![0x7701, 0x55]]);
        assert_eq!(calls_to(&e, SCENE_ADD), vec![vec![scene, 0x7701, 1]]);
        assert_eq!(
            calls_to(&e, 0x00b6_0120),
            vec![vec![0, 0, 100.0f32.to_bits(), 150.0f32.to_bits()]]
        );
        // The cell of (5000, -100) is (1, -1); 25 neighbours follow.
        let cells = calls_to(&e, 0x0045_1900);
        assert_eq!(cells.len(), 26);
        assert_eq!(cells[0], vec![0x0050_0000, 1, (-1i32) as u32]);
        assert_eq!(cells[1], vec![0x0050_0000, (-1i32) as u32, (-3i32) as u32]);
        // The inner cell within range goes to 0057d4f0 (which stops at its
        // world-space check); the edge cell to 00b61570.
        assert_eq!(calls_to(&e, 0x0054_fb70).len(), 1);
        assert_eq!(calls_to(&e, 0x0054_fb70)[0][0], 0xa000);
        assert_eq!(calls_to(&e, 0x0054_ddd0), vec![vec![0xa000]]);
        assert_eq!(calls_to(&e, 0x00b6_1570), vec![vec![0x31, 0x32]]);
        assert!(position_of(&e, 0x0054_4c60) < position_of(&e, 0x0054_4c30));
        assert_eq!(calls_to(&e, 0x00b5_d9f0), vec![vec![scene, 0x7701, 0]]);
        let update = calls_to(&e, 0x0043_d410);
        assert_eq!(&update[0][1..], &[0, 0, 0]);
        assert_eq!(calls_to(&e, 0x00a5_9c60), vec![vec![0x7701, update[0][0]]]);
        assert_eq!(calls_to(&e, 0x00a5_a040), vec![vec![0x7701]]);
        assert!(calls_to(&e, 0x00b6_2970).is_empty());
    }

    #[test]
    fn fn_0057d0a0_in_an_interior_only_clears() {
        let (mut e, _) = update_engine(100.0, 10.0, 0x1234);
        e.call_log = Some(vec![]);
        e.call(
            0x0057_d0a0,
            &args![0.0f32, 0.0f32, 0.0f32, 0u32, 0u32, 0u32, 0.0f32],
        );
        assert_eq!(calls_to(&e, 0x00b6_2970).len(), 1);
        assert!(calls_to(&e, BOOL_SETTING_VALUE).is_empty());
        assert_eq!(calls_to(&e, SCOPE_GUARD_CLOSE).len(), 1);
    }

    #[test]
    fn fn_0057d0a0_stops_below_the_threshold() {
        let (mut e, _) = update_engine(10.0, 10.0, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0057_d0a0,
            &args![0.0f32, 0.0f32, 0.0f32, 0u32, 0u32, 0u32, 0.0f32],
        );
        assert!(calls_to(&e, 0x00b6_0120).is_empty());
        assert!(calls_to(&e, 0x00b6_2970).is_empty());
        assert_eq!(calls_to(&e, SCOPE_GUARD_CLOSE).len(), 1);
        assert_eq!(e.global::<u32>(ROOT_NODE), 0);
    }

    const GEOMETRY_VTABLE: u32 = 0x6200_0000;
    const GEOMETRY_VISIBLE: u32 = 0x6300_009c;
    const FORM_VTABLE: u32 = 0x6400_0000;

    type Captured = Rc<RefCell<Vec<f32>>>;

    /// The doubles of `0057d4f0` for one quad with one grid point (start 8);
    /// `0057de00` stops at `00b61940`.
    fn cell_engine(geometry_kind: u32, visible: bool) -> (Engine, Captured, u32) {
        let mut e = base_engine();
        e.put_vtable(GEOMETRY_VTABLE, &[0; 0xa0 / 4]);
        e.mem.set_u32(GEOMETRY_VTABLE + 0x9c, GEOMETRY_VISIBLE);
        let geometry = e.mem.alloc(8);
        e.mem.set_u32(geometry, GEOMETRY_VTABLE);
        e.register_double(GEOMETRY_VISIBLE, move |_, _| ret(u32::from(visible)));
        e.put_vtable(FORM_VTABLE, &[0; 0x1ac / 4]);
        for (slot, value) in [
            (0x188u32, 60u32),
            (0x190, 30),
            (0x1a0, 0x1_0005),
            (0x1a8, 2),
        ] {
            let target = 0x6500_0000 + slot;
            e.mem.set_u32(FORM_VTABLE + slot, target);
            e.register_double(target, move |_, _| ret(value));
        }
        let form = e.mem.alloc(8);
        e.mem.set_u32(form, FORM_VTABLE);
        let frustum_source = e.mem.alloc(0x1c);
        e.mem.set_f32(frustum_source + 0x10, 5.0);
        e.mem.set_f32(frustum_source + 0x14, 1000.0);
        let bound = e.mem.alloc(16);
        let entry = e.mem.alloc(0x40);
        e.mem.set_u32(entry, 1);
        e.mem.set_u32(entry + 4, 0x99);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries, entry);
        stub(
            &mut e,
            &[
                0x0096_a2d0,
                0x0053_a510,
                0x0046_ffb0,
                0x0047_0470,
                0x004a_0c10,
                0x00a7_1b70,
                0x0045_c620,
                0x00b6_1370,
                0x00b6_17b0,
                SCOPE_GUARD_OPEN,
                SCOPE_GUARD_CLOSE,
                0x0063_3c90,
                0x0045_cec0,
            ],
        );
        e.register(0x0057_ce60, |_, _| ret(8));
        e.register(0x0054_ddd0, |_, _| ret(1));
        e.register(0x0054_6fb0, |_, _| ret(0x5100));
        e.register(0x0054_71e0, |_, _| st0(3.0));
        e.register(0x0054_4c30, |_, _| ret(0x31));
        e.register(0x0054_4c60, |_, _| ret(0x32));
        e.register(0x0043_c4b0, |_, _| ret(0x5200));
        e.register(0x004d_ee70, |_, _| ret(1600));
        e.register(0x004d_ee10, |_, _| ret(900));
        e.register(0x0045_c670, |_, _| ret(0x5300));
        e.register(0x0099_e040, |_, _| st0(90.0));
        e.register(0x00ec_a590, |_, a| st0(f64::take(a, &mut 0).tan()));
        e.register(0x0066_29f0, |_, a| ret(a[0]));
        e.register_double(0x0045_bbe0, move |_, _| ret(frustum_source));
        e.register(0x0046_1130, |_, _| ret(0x5400));
        let captured: Captured = Rc::new(RefCell::new(vec![]));
        let sink = captured.clone();
        e.register_double(0x00a7_4e10, move |e, a| {
            let mut v = sink.borrow_mut();
            for i in 0..6 {
                v.push(e.mem.f32(a[1] + 4 * i));
            }
            Ret::default()
        });
        e.register(0x0045_c9a0, |_, a| ret(if a[1] == 0 { 0x7000 } else { 0 }));
        e.register(0x0053_b450, |e, a| {
            e.mem.set_f32(a[1], 300.0);
            e.mem.set_f32(a[1] + 4, 400.0);
            e.mem.set_f32(a[1] + 8, 7.0);
            ret(a[1])
        });
        e.register_double(0x0043_d450, move |_, _| ret(bound));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0043_9ef0, |_, a| ret(a[1]));
        e.register(0x0045_7990, |_, _| st0(50.0));
        e.register(0x0084_d030, |_, _| st0(10.0));
        e.register(0x0045_6fc0, |_, _| ret(0x7100));
        e.register(0x0043_b4a0, |_, _| ret(0x7200));
        e.register_double(0x0043_b230, move |_, _| ret(geometry));
        e.register_double(0x0044_ddc0, move |_, _| ret(geometry_kind));
        e.register(0x0040_4040, |_, a| st0(f32::from_bits(a[0]) as f64));
        e.register_double(0x0053_ca80, move |_, _| ret(entries));
        e.register_double(0x0048_39c0, move |_, _| ret(form));
        e.register(0x00b6_1940, |_, _| ret(1));
        (e, captured, entry)
    }

    fn place(e: &mut Engine, range: f32) {
        e.call(
            0x0057_d4f0,
            &args![0x4000u32, 0x7701u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 2.0f32, range, 0.0f32],
        );
    }

    #[test]
    fn fn_0057d4f0_hands_visible_quad_points_to_0057de00() {
        let (mut e, captured, entry) = cell_engine(0, false);
        e.call_log = Some(vec![]);
        place(&mut e, 100.0);
        // The frustum from the camera: a 90 degree field of view at 16:9.
        let ratio = 1600.0f64 / 900.0;
        let aspect = ratio as f32;
        let scale = (0.75 / ratio) as f32;
        let angle = (90.0 * f64::from_bits(0x3f91_df46_a000_0000) * 0.5) as f32;
        let t = (angle as f64).tan() as f32 as f64;
        assert_eq!(
            *captured.borrow(),
            vec![
                (-t * scale as f64) as f32,
                (t * scale as f64) as f32,
                (t * aspect as f64 * scale as f64) as f32,
                (-t * aspect as f64 * scale as f64) as f32,
                5.0,
                1000.0,
            ]
        );
        assert_eq!(calls_to(&e, 0x0045_6fc0), vec![vec![0x4000, 2]]);
        assert_eq!(calls_to(&e, 0x0043_b4a0), vec![vec![0x7100, 0]]);
        // One grid point: index 8 * 17 + 8, moved by -256 in x and y.
        let grid = calls_to(&e, 0x0053_b450);
        assert_eq!(&grid[1][2..], &[0, 144, 0, 0]);
        assert_eq!(calls_to(&e, 0x0053_ca80), vec![vec![0x5100, 0, 144]]);
        let key = (44 << 16) | 144;
        assert_eq!(calls_to(&e, 0x00b6_17b0), vec![vec![0x99, key]]);
        assert_eq!(calls_to(&e, 0x0048_39c0), vec![vec![0x99]]);
        assert_eq!(calls_to(&e, 0x6500_01a0).len(), 1);
        // 0057de00 ran with the entry and the key.
        assert_eq!(calls_to(&e, 0x00b6_1940), vec![vec![entry, key]]);
        assert_eq!(
            &calls_to(&e, SCOPE_GUARD_OPEN)[0][1..],
            &[0x1c, 1, SOURCE_FILE, 0x20a]
        );
        assert!(calls_to(&e, 0x00b6_1370).is_empty());
        assert_eq!(calls_to(&e, 0x0047_0470).len(), 4);
        assert_eq!(calls_to(&e, 0x0046_ffb0).len(), 1);
    }

    #[test]
    fn fn_0057d4f0_keys_hidden_or_far_quads_only() {
        for (kind, range) in [(5u32, 100.0f32), (0, 30.0)] {
            let (mut e, _, _) = cell_engine(kind, false);
            e.call_log = Some(vec![]);
            place(&mut e, range);
            assert_eq!(calls_to(&e, 0x00b6_1370), vec![vec![(44 << 16) | 144]]);
            assert!(calls_to(&e, 0x0053_ca80).is_empty());
            assert!(calls_to(&e, 0x00b6_1940).is_empty());
        }
    }

    #[test]
    fn fn_0057d4f0_stops_early() {
        let (mut e, _, _) = cell_engine(0, false);
        e.set_global(SKIP_FLAG, 1u8);
        e.call_log = Some(vec![]);
        place(&mut e, 100.0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        let (mut e, _, _) = cell_engine(0, false);
        e.register(0x0054_6fb0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        place(&mut e, 100.0);
        assert_eq!(calls_to(&e, 0x0046_ffb0).len(), 1);
        assert!(calls_to(&e, 0x0054_71e0).is_empty());
    }

    type ScatterRig = (Engine, u32, u32, u32);

    /// The doubles of `0057de00` for one step that keeps its point.
    fn scatter_engine() -> ScatterRig {
        let mut e = base_engine();
        e.set_global(DEGREES_TO_RADIANS, 1.0f64);
        e.set_global(TABLE + 10 * 4, 1.0f32);
        e.set_global(TABLE + 3 * 4, -1.0f32);
        let points = e.mem.alloc(0x40);
        let shades = e.mem.alloc(0x40);
        e.set_global(POINT_BUFFER, points);
        e.set_global(SHADE_BUFFER, shades);
        let divisor = e.mem.alloc(4);
        e.mem.set_i32(divisor, 64);
        stub(
            &mut e,
            &[
                SCOPE_GUARD_OPEN,
                SCOPE_GUARD_CLOSE,
                0x0063_3c90,
                0x0045_cec0,
                0x0053_a510,
                0x0041_4430,
                0x00b6_2de0,
            ],
        );
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x00b6_1940, |_, _| ret(0));
        e.register(0x0053_94a0, |_, _| ret(1));
        e.register(0x0057_ce60, |_, _| ret(1));
        e.register_double(0x0043_d4d0, move |_, _| ret(divisor));
        e.register(0x0041_6870, |_, a| ret(a[0]));
        e.register(0x0043_9e90, |e, a| {
            e.mem.set_f32(a[1], 1000.0);
            e.mem.set_f32(a[1] + 4, 2000.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            ret(a[1])
        });
        e.register(0x0040_6d90, |_, a| {
            ret(f32::from_bits(a[0]).floor() as i32 as u32)
        });
        e.register(0x0094_4460, |_, _| ret(100));
        e.register(0x004d_fea0, |_, _| st0(0.0));
        e.register(0x0040_4040, |_, a| st0(f32::from_bits(a[0]) as f64));
        e.register(0x0053_b550, |_, _| ret(0));
        e.register(0x0045_72e0, |e, a| {
            e.mem.set_f32(a[2], 10.0);
            Ret::default()
        });
        e.register(0x0043_9ef0, |_, a| ret(a[1]));
        e.register(0x0045_73f0, |e, a| {
            e.mem.set_f32(a[2], 0.2);
            e.mem.set_f32(a[2] + 4, 0.4);
            e.mem.set_f32(a[2] + 8, 0.98);
            Ret::default()
        });
        e.register(0x0045_7520, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            e.mem.set_f32(a[2] + 4, 0.5);
            e.mem.set_f32(a[2] + 8, 0.25);
            Ret::default()
        });
        let entry = e.mem.alloc(0x40);
        e.mem.set_f32(entry + 8, 256.0);
        let heights = e.mem.alloc(9 * 4);
        e.mem.set_f32(heights, 0.5);
        (e, entry, heights, points)
    }

    fn scatter(e: &mut Engine, entry: u32, heights: u32, kind: u32) {
        e.call(
            0x0057_de00,
            &args![
                0x31u32, 0x32u32, 0u32, 0x5100u32, 0xabcdu32, 0x6600u32, 0x7701u32, 1u32, 2u32,
                3u32, entry, heights, 0.0f32, 100.0f32, 2u32, kind, 5.0f32, 10u32, 3u32
            ],
        );
    }

    #[test]
    fn fn_0057de00_keeps_a_point_and_hands_it_on() {
        let (mut e, entry, heights, points) = scatter_engine();
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 0);
        assert_eq!(calls_to(&e, 0x00b6_1940), vec![vec![entry, 0xabcd]]);
        assert_eq!(calls_to(&e, 0x0053_94a0), vec![vec![0x5100]]);
        assert_eq!(calls_to(&e, 0x0094_4460), vec![vec![0, 0x7fff]]);
        let clamp = f32::from_bits(0x3f78_51ec);
        let half_up = |v: f32| (v as f64 * 0.5 + 0.5) as f32;
        assert_eq!(e.mem.f32(points), (1000.0 + half_up(0.2) as f64) as f32);
        assert_eq!(e.mem.f32(points + 4), (2000.0 + half_up(0.4) as f64) as f32);
        assert_eq!(e.mem.f32(points + 8), (10.0 + clamp as f64) as f32);
        let shades = e.global::<u32>(SHADE_BUFFER);
        assert_eq!(
            e.mem.f32(shades),
            (1.0 * 0.3 + 0.5 * 0.6 + 0.25 * 0.1) as f32
        );
        assert_eq!(
            calls_to(&e, 0x00b6_2de0),
            vec![vec![0xabcd, 0x31, 0x32, 0x7701, entry, points, shades, 1]]
        );
        assert_eq!(calls_to(&e, 0x0045_cec0).len(), 1);
        assert_eq!(calls_to(&e, SCOPE_GUARD_CLOSE).len(), 1);
    }

    #[test]
    fn fn_0057de00_drops_points_by_kind_bounds_and_chance() {
        // Kind 2: the height 10 lies above water - depth.
        let (mut e, entry, heights, _) = scatter_engine();
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 2);
        assert!(calls_to(&e, 0x0045_73f0).is_empty());
        assert!(calls_to(&e, 0x00b6_2de0).is_empty());

        // The normal's z above the upper bound.
        let (mut e, entry, heights, _) = scatter_engine();
        e.set_global(TABLE + 10 * 4, 0.5f32);
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 0);
        assert_eq!(calls_to(&e, 0x0045_73f0).len(), 1);
        assert!(calls_to(&e, 0x0041_4430).is_empty());
        assert!(calls_to(&e, 0x00b6_2de0).is_empty());

        // A roll at or above the chance.
        let (mut e, entry, heights, _) = scatter_engine();
        e.register(0x0094_4460, |_, _| ret(16384));
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 0);
        assert!(calls_to(&e, 0x004d_fea0).is_empty());

        // A zero chance does not roll.
        let (mut e, entry, heights, _) = scatter_engine();
        e.mem.set_f32(heights, 0.0);
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 0);
        assert!(calls_to(&e, 0x0094_4460).is_empty());

        // A known entry does nothing but clean up.
        let (mut e, entry, heights, _) = scatter_engine();
        e.register(0x00b6_1940, |_, _| ret(1));
        e.call_log = Some(vec![]);
        scatter(&mut e, entry, heights, 0);
        assert!(calls_to(&e, 0x0053_94a0).is_empty());
        assert_eq!(calls_to(&e, 0x0045_cec0).len(), 1);
    }
}

//! `NiXenonRenderer/run_00e6fbb0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Builds a 4x4 float matrix from a rotation, a translation and a scale
/// (`00b71a40`, `BSShader/run_00b71920`; cdecl: out, rotation, point,
/// scale).
const MAKE_MATRIX_00B71A40: u32 = 0x00b7_1a40;
/// The other matrix builder `00e6b8f0` (`nixenonrenderer.cpp`; cdecl, the
/// same four words).
const MAKE_MATRIX_00E6B8F0: u32 = 0x00e6_b8f0;
/// The renderer's Direct3D device (+0x288) and its `SetTransform` slot
/// (+0xB0, `IDirect3DDevice9` order; called with the device pushed first).
const RENDERER_DEVICE: u32 = 0x288;
const DEVICE_SET_TRANSFORM: u32 = 0xb0;
/// The renderer's render-state object (+0x8B8).
const RENDERER_STATE_OBJECT: u32 = 0x8b8;
/// The renderer's vertex shader object (+0x8BC).
const RENDERER_VERTEX_SHADER: u32 = 0x8bc;
/// The 4-byte getter `00e8bf90` (engine map: `NiD3DHLSLVertexShader::
/// GetEntryPoint`; thiscall).
const SHADER_GETTER_00E8BF90: u32 = 0x00e8_bf90;
/// Frees the matrix buffer (`00aa1110`; cdecl, one word).
const FREE_ALIGNED_00AA1110: u32 = 0x00aa_1110;
/// Allocates the matrix buffer (`00aa10b0`; cdecl: size, alignment 16).
const ALLOCATE_ALIGNED_00AA10B0: u32 = 0x00aa_10b0;
/// `NiTransform::InvertNonUniform` (engine map; thiscall, one word: out).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
/// The transform product `0062c250` (`teshavokutilities.cpp`; thiscall,
/// two words: out, right-hand transform; answers `out`).
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// The point subtracted from translations (three floats).
const ORIGIN_X: u32 = 0x011f_474c;
const ORIGIN_Y: u32 = 0x011f_4750;
const ORIGIN_Z: u32 = 0x011f_4754;
/// The lower scale bound (a double) and the upper one (a float).
const SCALE_LOW_0106D508: u32 = 0x0106_d508;
const SCALE_HIGH_0106B9E0: u32 = 0x0106_b9e0;
/// Size of an `NiTransform` (9 rotation floats, the translation, the
/// scale), as the 13-word copies move it.
const TRANSFORM_SIZE: u32 = 0x34;

// Translated from 00e6fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on the renderer, two words): builds
/// the world matrix at +0x940 from `transform` (rotation, translation
/// +0x24, scale +0x30; `00b71a40`), hands it to the device's
/// `SetTransform(0x100, ..)` when `set_world` is not 0, passes the
/// transform to the render-state object's virtual slot +0x30, and then
/// computes the six floats +0x64C..+0x660: with the rotation scaled
/// (`A = R * s`, each rounded to a float), +0x64C..+0x654 are the vector
/// +0x634 times the columns of `A` and +0x658..+0x660 the vector +0x640
/// times them, in the exe's x87 order.
pub fn fn_00e6fbb0(e: &mut Engine, this: Ptr, transform: Ptr, set_world: u8) {
    let base = this.addr();
    let t = transform.addr();
    let world = base + 0x940;
    // FLD/FSTP of the scale into the pushed word.
    let scale = x87_quiet_bits(e.mem.u32(t + 0x30));
    e.call(MAKE_MATRIX_00B71A40, &[world, t, t + 0x24, scale]);
    if set_world != 0 {
        let device = e.mem.u32(base + RENDERER_DEVICE);
        e.vcall(device, DEVICE_SET_TRANSFORM, &[0x100, world]);
    }
    let state = e.mem.u32(base + RENDERER_STATE_OBJECT);
    e.vcall(state, 0x30, &[t]);
    // The scale goes through the argument slot again (FSTP, FLD).
    let s = x87_load32(x87_quiet_bits(e.mem.u32(t + 0x30)));
    let r = |e: &Engine, i: u32| x87_load32(e.mem.u32(t + i * 4));
    let mut a = [0u32; 9];
    a[0] = x87_store32(x87_mul(r(e, 0), s));
    for i in 1..8 {
        a[i] = x87_store32(x87_mul(r(e, i as u32), s));
    }
    a[8] = x87_store32(x87_mul(s, x87_m32(e.mem.u32(t + 0x20))));
    let ld = |i: usize| x87_load32(a[i]);
    let m = |i: usize| x87_m32(a[i]);
    let v = |e: &Engine, off: u32| x87_load32(e.mem.u32(base + off));
    let vm = |e: &Engine, off: u32| x87_m32(e.mem.u32(base + off));
    let (l0, l1, l2) = (v(e, 0x634), v(e, 0x638), v(e, 0x63c));
    let sum = x87_add(x87_mul(l1, ld(3)), x87_mul(l0, ld(0)));
    let out0 = x87_store32(x87_add(x87_mul(l2, ld(6)), sum));
    let (l0, l1, l2) = (v(e, 0x634), v(e, 0x638), v(e, 0x63c));
    let sum = x87_add(x87_mul(l1, ld(4)), x87_mul(l0, ld(1)));
    let out1 = x87_store32(x87_add(sum, x87_mul(l2, m(7))));
    let (l0, l1, l2) = (v(e, 0x634), v(e, 0x638), v(e, 0x63c));
    let sum = x87_add(x87_mul(l0, m(2)), x87_mul(l1, m(5)));
    let out2 = x87_store32(x87_add(sum, x87_mul(l2, m(8))));
    e.mem.set_u32(base + 0x64c, out0);
    e.mem.set_u32(base + 0x650, out1);
    e.mem.set_u32(base + 0x654, out2);
    // The second vector uses the A values still on the x87 stack.
    let a3_l4 = x87_mul(ld(3), v(e, 0x644));
    let a0_l3 = x87_mul(ld(0), v(e, 0x640));
    let sum = x87_add(a0_l3, a3_l4);
    let a6_l5 = x87_mul(ld(6), v(e, 0x648));
    let out3 = x87_store32(x87_add(a6_l5, sum));
    let a1_l3 = x87_mul(ld(1), v(e, 0x640));
    let a4_l4 = x87_mul(ld(4), vm(e, 0x644));
    let sum = x87_add(a1_l3, a4_l4);
    let out4 = x87_store32(x87_add(sum, x87_mul(v(e, 0x648), m(7))));
    let sum = x87_add(x87_mul(v(e, 0x640), m(2)), x87_mul(v(e, 0x644), m(5)));
    let out5 = x87_store32(x87_add(sum, x87_mul(v(e, 0x648), m(8))));
    e.mem.set_u32(base + 0x658, out3);
    e.mem.set_u32(base + 0x65c, out4);
    e.mem.set_u32(base + 0x660, out5);
}

/// Copies an `NiTransform` (13 words) from `from` to `to`.
fn copy_transform(e: &mut Engine, to: u32, from: u32) {
    for i in 0..TRANSFORM_SIZE / 4 {
        let word = e.mem.u32(from.wrapping_add(i * 4));
        e.mem.set_u32(to + i * 4, word);
    }
}

/// Whether a scale counts as "not 1": below the double at `0106d508` and
/// above the float at `0106b9e0` at once (`FCOM`, `TEST AH,5` twice), as
/// the exe tests it for the bones.
fn bone_scale_outside(e: &Engine, scale_bits: u32) -> bool {
    let scale = x87_load32(scale_bits);
    let low = e.mem.f64(SCALE_LOW_0106D508);
    if !(scale < low) {
        return false;
    }
    let high = x87_load32(e.mem.u32(SCALE_HIGH_0106B9E0));
    high < x87_m32(scale_bits)
}

// Translated from 00e6fe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on the renderer, five words): fills
/// the skin's bone-matrix buffer. Nothing happens when the skin's +0x18
/// equals the vertex shader getter's value (`00e8bf90` on +0x8BC) and its
/// +0x20 equals `registers`. Otherwise it stores the getter's value in
/// +0x18, (re)allocates the buffer +0x28 when `bones * registers * 16`
/// (bones: the skin data +0x08's +0x44) is larger than +0x24 (`00aa1110`,
/// `00aa10b0(size, 16)`), sets +0x1C and +0x20; `registers` 3 sets
/// `layout`. With 4 registers each 0x40-byte matrix gets three zeros
/// (+0x30.. with `layout` set, else the fourth column) and 1 at +0x3C.
///
/// Then the root transform `T = data(+0x0C) * inverse(skin root +0x10's
/// +0x68)` and `W = world * T`; for each bone `B = bone(+0x68) * bind
/// (data +0x40, 0x4C each)`, and `B = W * B` unless `skip_world`. With 4
/// registers `00e6b8f0` (`layout` set) or `00b71a40` builds the matrix
/// from `B`; otherwise the 3x4 rows are written inline (rotation times the
/// scale, translation minus `011f474c`). Last the render-state object's
/// virtual slot +0xF8 gets whether a scale was outside both bounds (`T`'s,
/// or a bone's +0x98).
///
/// The flag goes out as a pushed word whose upper three bytes are an
/// uninitialised stack slot in the exe; the translation passes them as 0.
pub fn fn_00e6fe30(
    e: &mut Engine,
    this: Ptr,
    skin: Ptr,
    world: Ptr,
    layout: u8,
    registers: u32,
    skip_world: u8,
) {
    let renderer = this.addr();
    let skin = skin.addr();
    let shader = e.mem.u32(renderer + RENDERER_VERTEX_SHADER);
    let current = e.mem.u32(skin + 0x18);
    let entry_point = e.call(SHADER_GETTER_00E8BF90, &[shader]).u32();
    if current == entry_point && registers == e.mem.u32(skin + 0x20) {
        return;
    }
    let shader = e.mem.u32(renderer + RENDERER_VERTEX_SHADER);
    let entry_point = e.call(SHADER_GETTER_00E8BF90, &[shader]).u32();
    let capacity = e.mem.u32(skin + 0x24);
    e.mem.set_u32(skin + 0x18, entry_point);
    let data = e.mem.u32(skin + 8);
    let bones = e.mem.u32(data + 0x44);
    let mut buffer = e.mem.u32(skin + 0x28);
    let mut layout = layout;
    if registers == 3 {
        layout = 1;
    }
    let size = bones.wrapping_mul(registers) << 4;
    if size > capacity {
        e.call(FREE_ALIGNED_00AA1110, &[buffer]);
        buffer = e.call(ALLOCATE_ALIGNED_00AA10B0, &[size, 0x10]).u32();
        e.mem.set_u32(skin + 0x28, buffer);
        e.mem.set_u32(skin + 0x24, size);
    }
    e.mem.set_u32(skin + 0x1c, bones);
    e.mem.set_u32(skin + 0x20, registers);
    if registers == 4 && bones != 0 {
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);
        let mut matrix = buffer;
        for _ in 0..bones {
            let cleared = if layout != 0 {
                [0x38, 0x34, 0x30]
            } else {
                [0x2c, 0x1c, 0x0c]
            };
            for offset in cleared {
                e.mem.set_u32(matrix.wrapping_add(offset), zero);
            }
            e.mem.set_u32(matrix.wrapping_add(0x3c), one);
            matrix = matrix.wrapping_add(0x40);
        }
    }
    let world = world.addr();
    e.with_stack(0xb4, |e, frame| {
        // The frame's three transforms ([ESP+0x1C], +0x5C, +0x90).
        let product = frame.addr() + 0x0c;
        let root = frame.addr() + 0x4c;
        let scratch = frame.addr() + 0x80;
        let skin_root = e.mem.u32(skin + 0x10);
        e.call(TRANSFORM_INVERT, &[skin_root.wrapping_add(0x68), product]);
        let result = e
            .call(TRANSFORM_MULTIPLY, &[data + 0xc, root, product])
            .u32();
        copy_transform(e, product, result);
        let result = e.call(TRANSFORM_MULTIPLY, &[world, scratch, product]).u32();
        // The scale test reads `T`'s scale (+0x30).
        let root_scale = e.mem.u32(product + 0x30);
        copy_transform(e, root, result);
        let mut outside = false;
        let scale = x87_load32(root_scale);
        if scale < e.mem.f64(SCALE_LOW_0106D508) {
            let high = x87_m32(e.mem.u32(SCALE_HIGH_0106B9E0));
            // FCOMP with the float, `TEST AH,0x41`: set only when above.
            if scale > high {
                outside = true;
            }
        }
        let bone_list = e.mem.u32(skin + 0x14);
        let mut bind = e.mem.u32(data + 0x40);
        let mut matrix = buffer;
        let mut rows = buffer.wrapping_add(0x20);
        let mut bone = 0u32;
        while bone < bones {
            let node = e.mem.u32(bone_list.wrapping_add(bone * 4));
            e.call(
                TRANSFORM_MULTIPLY,
                &[node.wrapping_add(0x68), product, bind],
            );
            let node = e.mem.u32(bone_list.wrapping_add(bone * 4));
            if bone_scale_outside(e, e.mem.u32(node.wrapping_add(0x98))) {
                outside = true;
            }
            if skip_world == 0 {
                let result = e.call(TRANSFORM_MULTIPLY, &[root, scratch, product]).u32();
                copy_transform(e, product, result);
            }
            if skin_check_enabled() {
                skin_check(e, skin, bone, node, product);
            }
            if registers == 4 {
                let scale = x87_quiet_bits(e.mem.u32(product + 0x30));
                let builder = if layout != 0 {
                    MAKE_MATRIX_00E6B8F0
                } else {
                    MAKE_MATRIX_00B71A40
                };
                e.call(builder, &[matrix, product, product + 0x24, scale]);
            } else {
                write_rows(e, rows, product);
            }
            bind = bind.wrapping_add(0x4c);
            bone += 1;
            matrix = matrix.wrapping_add(0x40);
            rows = rows.wrapping_add(0x30);
        }
        if registers == 3 && head_check_enabled() {
            head_skin_check(e, skin, buffer, bones);
        }
        let state = e.mem.u32(renderer + RENDERER_STATE_OBJECT);
        e.vcall(state, 0xf8, &[u32::from(outside)]);
    });
}

/// Whether the bone-matrix check below runs (`NV_SKIN_CHECK` set; read once).
fn skin_check_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NV_SKIN_CHECK").is_some())
}

/// Lines the check has printed (it stops at [`SKIN_CHECK_LIMIT`]).
static SKIN_CHECK_LINES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
const SKIN_CHECK_LIMIT: u32 = 400;

/// What the check finds wrong with one bone's skinning transform (`bone.world * bind`, or
/// with the world as well): `None` when the rotation's columns are unit length and
/// perpendicular (within 2%) and the scale is between 0.5 and 2.
fn skin_matrix_fault(rotation: [f32; 9], scale: f32) -> Option<String> {
    let column = |c: usize| [rotation[c], rotation[3 + c], rotation[6 + c]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let (c0, c1, c2) = (column(0), column(1), column(2));
    let norms = [dot(c0, c0).sqrt(), dot(c1, c1).sqrt(), dot(c2, c2).sqrt()];
    let skews = [dot(c0, c1), dot(c1, c2), dot(c0, c2)];
    let stretched = norms.iter().any(|n| !((n - 1.0).abs() <= 0.02));
    let sheared = skews.iter().any(|s| !(s.abs() <= 0.02));
    let scaled = !(0.5..=2.0).contains(&scale);
    if stretched || sheared || scaled {
        Some(format!(
            "norms {:.3} {:.3} {:.3} skews {:.3} {:.3} {:.3} scale {scale:.3}",
            norms[0], norms[1], norms[2], skews[0], skews[1], skews[2]
        ))
    } else {
        None
    }
}

/// Diagnostic only (not exe behaviour; `NV_SKIN_CHECK`): reports a bone whose skinning
/// transform at `product` is not a rotation times a sane scale, with the skin instance, the
/// bone index, the bone node and its name, so a run can tell a bad bone palette (look IK,
/// animation) from bad vertex data. With `NV_SKIN_CHECK=head` it also prints every
/// `Bip01 Head` bone it sees (until the line limit).
fn skin_check(e: &Engine, skin: u32, bone: u32, node: u32, product: u32) {
    let rotation: [f32; 9] = std::array::from_fn(|i| e.mem.f32(product + 4 * i as u32));
    let scale = e.mem.f32(product + 0x30);
    let name_pointer = e.mem.u32(node.wrapping_add(8));
    let name = if name_pointer == 0 {
        String::new()
    } else {
        String::from_utf8_lossy(&e.mem.cstr(name_pointer)).into_owned()
    };
    let fault = skin_matrix_fault(rotation, scale);
    let head = name == "Bip01 Head"
        && std::env::var("NV_SKIN_CHECK").is_ok_and(|v| v.eq_ignore_ascii_case("head"));
    if fault.is_none() && !head {
        return;
    }
    let line = SKIN_CHECK_LINES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if line >= SKIN_CHECK_LIMIT {
        return;
    }
    let translation = [0x24, 0x28, 0x2c].map(|o| e.mem.f32(product + o));
    println!(
        "skin check: skin {skin:08x} bone {bone} node {node:08x} '{name}' {} rot {:?} t {:?}",
        fault.as_deref().unwrap_or("ok"),
        rotation,
        translation
    );
}

// ---------------------------------------------------------------------------------------------
// `NV_HEAD_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory.
//
// Splits a stretched head between its layers in one run:
// - `bone` lines (00e6fe30, 3-register skins with a `Bip01 Head` bone): every bone of the skin
//   with its world position relative to `Bip01 Head` and whether the node still hangs under the
//   skin's own root (`nif-local`: never rebound to the skeleton by `FixSkinInstances`).
// - `skinned` lines (same place): the head geometry's data positions skinned on the CPU with the
//   bone rows `00e6fe30` just wrote (partition weights, palettes, bone maps), as the vertex shader
//   would; vertices far from `Bip01 Head` mean the CPU-side data (morphs, weights, palettes,
//   bones) already stretch the head.
// - `pack` lines (00e76770, the declaration packer): the layout of the head's vertex stream and
//   whether the packed positions, weights and bone indices equal their sources.
// When all three are clean while the head shows stretched, the fault is past the packed bytes:
// the vertex declaration, the stream binding or the shader constants on the real device.
// ---------------------------------------------------------------------------------------------

/// Whether `NV_HEAD_CHECK` is set (read once).
pub(crate) fn head_check_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NV_HEAD_CHECK").is_some())
}

/// Lines the head check printed (it stops at [`HEAD_CHECK_LIMIT`]).
static HEAD_CHECK_LINES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
const HEAD_CHECK_LIMIT: u32 = 1500;

fn head_line(text: std::fmt::Arguments) {
    let line = HEAD_CHECK_LINES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if line < HEAD_CHECK_LIMIT {
        println!("head check: {text}");
    }
}

/// What the head check has seen: each skin instance's geometry data (from the draws), the
/// geometry data of skins with a `Bip01 Head` bone, and how often each key was reported.
#[derive(Default)]
struct HeadCheckState {
    skin_data: std::collections::HashMap<u32, u32>,
    head_data: std::collections::HashSet<u32>,
    reports: std::collections::HashMap<(u32, u32, u32), u32>,
}

fn head_state() -> std::sync::MutexGuard<'static, HeadCheckState> {
    static STATE: std::sync::OnceLock<std::sync::Mutex<HeadCheckState>> =
        std::sync::OnceLock::new();
    let state = STATE.get_or_init(Default::default);
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Whether to report key `key` this time (and counts it): the first three times, then every
/// 600th, and always when `urgent`.
fn head_report_due(key: (u32, u32, u32), urgent: bool) -> bool {
    let mut state = head_state();
    let count = state.reports.entry(key).or_insert(0);
    *count += 1;
    urgent || *count <= 3 || *count % 600 == 0
}

/// Notes a drawn geometry's skin instance (+0xBC) and model data (+0xB8) for the head check.
pub(crate) fn head_check_note_geometry(e: &Engine, geometry: u32) {
    if geometry == 0 {
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let skin = e.mem.u32(geometry.wrapping_add(0xbc));
    if data != 0 && skin != 0 {
        head_state().skin_data.insert(skin, data);
    }
}

/// Whether `data` is the geometry data of a skin with a `Bip01 Head` bone.
pub(crate) fn head_check_is_head_data(data: u32) -> bool {
    head_state().head_data.contains(&data)
}

/// The name of an `NiObjectNET` (+8), empty without one.
fn object_name(e: &Engine, object: u32) -> String {
    if object == 0 {
        return String::new();
    }
    let name = e.mem.u32(object.wrapping_add(8));
    if name == 0 {
        return String::new();
    }
    String::from_utf8_lossy(&e.mem.cstr(name)).into_owned()
}

/// Three floats at `at`.
fn point(e: &Engine, at: u32) -> [f32; 3] {
    [
        e.mem.f32(at),
        e.mem.f32(at.wrapping_add(4)),
        e.mem.f32(at.wrapping_add(8)),
    ]
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// `v` through one bone's three rows as `00e6fe30` writes them (row r: three rotation-times-
/// scale floats, then the translation), the way the vertex shader applies them.
fn skin_point(rows: &[f32; 12], v: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|r| {
        rows[4 * r] * v[0] + rows[4 * r + 1] * v[1] + rows[4 * r + 2] * v[2] + rows[4 * r + 3]
    })
}

/// Whether `ancestor` is `node` or one of its parents (`m_pkParent` +0x18, at most 256 up).
fn hangs_under(e: &Engine, node: u32, ancestor: u32) -> bool {
    let mut at = node;
    for _ in 0..256 {
        if at == 0 {
            return false;
        }
        if at == ancestor {
            return true;
        }
        at = e.mem.u32(at.wrapping_add(0x18));
    }
    false
}

/// A running box and the farthest point from the head.
#[derive(Default)]
struct Spread {
    low: [f32; 3],
    high: [f32; 3],
    seen: bool,
}

impl Spread {
    fn add(&mut self, p: [f32; 3]) {
        if !self.seen {
            self.low = p;
            self.high = p;
            self.seen = true;
            return;
        }
        for i in 0..3 {
            self.low[i] = self.low[i].min(p[i]);
            self.high[i] = self.high[i].max(p[i]);
        }
    }

    fn text(&self) -> String {
        format!(
            "x {:.1}..{:.1} y {:.1}..{:.1} z {:.1}..{:.1}",
            self.low[0], self.high[0], self.low[1], self.high[1], self.low[2], self.high[2]
        )
    }
}

/// Head vertices skinned farther than this from `Bip01 Head` count as stretched (a human head
/// with its neck reaches about 30 units from the head bone).
const HEAD_REACH: f32 = 45.0;

/// The `bone` and `skinned` reports for one skin (see the block comment above).
fn head_skin_check(e: &Engine, skin: u32, buffer: u32, bones: u32) {
    let bone_list = e.mem.u32(skin.wrapping_add(0x14));
    if bone_list == 0 || buffer == 0 {
        return;
    }
    let nodes: Vec<u32> = (0..bones)
        .map(|b| e.mem.u32(bone_list.wrapping_add(4 * b)))
        .collect();
    let names: Vec<String> = nodes.iter().map(|&n| object_name(e, n)).collect();
    let Some(head) = names.iter().position(|n| n == "Bip01 Head") else {
        return;
    };
    let data = head_state().skin_data.get(&skin).copied();
    if let Some(data) = data {
        head_state().head_data.insert(data);
    }
    let origin = point(e, ORIGIN_X);
    let head_world = point(e, nodes[head].wrapping_add(0x8c));
    let head_at = [
        head_world[0] - origin[0],
        head_world[1] - origin[1],
        head_world[2] - origin[2],
    ];
    let root = e.mem.u32(skin.wrapping_add(0x10));
    let rows_of = |b: u32| -> [f32; 12] {
        std::array::from_fn(|i| e.mem.f32(buffer.wrapping_add(b * 0x30 + 4 * i as u32)))
    };

    // The CPU skinning of the geometry data, partition by partition.
    let mut lines = Vec::new();
    let mut far_total = 0u32;
    if let Some(data) = data {
        let positions = e.mem.u32(data.wrapping_add(0x20));
        let vertex_count = u32::from(e.mem.u16(data.wrapping_add(8)));
        let mut source = Spread::default();
        for v in 0..vertex_count {
            source.add(point(e, positions.wrapping_add(v * 12)));
        }
        let mut partition_object = e.mem.u32(skin.wrapping_add(0xc));
        if partition_object == 0 {
            let skin_data = e.mem.u32(skin.wrapping_add(8));
            partition_object = e.mem.u32(skin_data.wrapping_add(8));
        }
        let parts = if partition_object == 0 {
            0
        } else {
            e.mem.u32(partition_object.wrapping_add(8))
        };
        let array = if partition_object == 0 {
            0
        } else {
            e.mem.u32(partition_object.wrapping_add(0xc))
        };
        lines.push(format!(
            "skinned skin {skin:08x} data {data:08x} verts {vertex_count} positions {positions:08x} \
             data box {} partitions {parts}",
            source.text()
        ));
        for p in 0..parts.min(16) {
            let part = array.wrapping_add(p * 0x2c);
            let part_bones = e.mem.u32(part.wrapping_add(4));
            let weights = e.mem.u32(part.wrapping_add(8));
            let map = e.mem.u32(part.wrapping_add(0xc));
            let palette = e.mem.u32(part.wrapping_add(0x10));
            let verts = u32::from(e.mem.u16(part.wrapping_add(0x1c)));
            let bone_count = u32::from(e.mem.u16(part.wrapping_add(0x20)));
            let per_vertex = u32::from(e.mem.u16(part.wrapping_add(0x24)));
            let mut spread = Spread::default();
            let mut far = 0u32;
            let mut bad_weights = 0u32;
            let mut bad_palette = 0u32;
            let mut worst = (0.0f32, 0u32);
            for i in 0..verts {
                let index = if map == 0 {
                    i
                } else {
                    u32::from(e.mem.u16(map.wrapping_add(2 * i)))
                };
                let v = point(e, positions.wrapping_add(index * 12));
                let mut out = [0.0f32; 3];
                let mut sum = 0.0f32;
                for j in 0..per_vertex {
                    let w = if weights == 0 {
                        0.0
                    } else {
                        e.mem.f32(weights.wrapping_add(4 * (i * per_vertex + j)))
                    };
                    let local = if palette == 0 {
                        j
                    } else {
                        u32::from(e.mem.u8(palette.wrapping_add(i * per_vertex + j)))
                    };
                    if local >= bone_count {
                        bad_palette += 1;
                        continue;
                    }
                    let b = u32::from(e.mem.u16(part_bones.wrapping_add(2 * local)));
                    if b >= bones {
                        bad_palette += 1;
                        continue;
                    }
                    let s = skin_point(&rows_of(b), v);
                    for k in 0..3 {
                        out[k] += w * s[k];
                    }
                    sum += w;
                }
                if !((sum - 1.0).abs() <= 0.01) {
                    bad_weights += 1;
                }
                let rel = [
                    out[0] - head_at[0],
                    out[1] - head_at[1],
                    out[2] - head_at[2],
                ];
                spread.add(rel);
                let d = distance(out, head_at);
                if !(d <= HEAD_REACH) {
                    far += 1;
                }
                if !(d <= worst.0) {
                    worst = (d, i);
                }
            }
            far_total += far;
            lines.push(format!(
                "skinned skin {skin:08x} partition {p}: verts {verts} bones {bone_count} per vertex \
                 {per_vertex}; {far} farther than {HEAD_REACH} from Bip01 Head, worst {:.1} at {}; \
                 weight sums off {bad_weights}, palette entries out of range {bad_palette}; \
                 box around the head {}",
                worst.0,
                worst.1,
                spread.text()
            ));
            if far > 0 {
                let i = worst.1;
                let index = if map == 0 {
                    i
                } else {
                    u32::from(e.mem.u16(map.wrapping_add(2 * i)))
                };
                let mut influences = Vec::new();
                for j in 0..per_vertex {
                    let w = if weights == 0 {
                        0.0
                    } else {
                        e.mem.f32(weights.wrapping_add(4 * (i * per_vertex + j)))
                    };
                    let local = if palette == 0 {
                        j
                    } else {
                        u32::from(e.mem.u8(palette.wrapping_add(i * per_vertex + j)))
                    };
                    let b = if local < bone_count {
                        u32::from(e.mem.u16(part_bones.wrapping_add(2 * local)))
                    } else {
                        u32::MAX
                    };
                    let name = names.get(b as usize).cloned().unwrap_or_default();
                    influences.push(format!("{w:.3} x {local}->{b} '{name}'"));
                }
                lines.push(format!(
                    "skinned skin {skin:08x} partition {p} worst vertex {i} (data {index}) at {:?}: {}",
                    point(e, positions.wrapping_add(index * 12)),
                    influences.join(", ")
                ));
            }
        }
    } else {
        lines.push(format!(
            "skinned skin {skin:08x}: no draw has shown this skin's geometry yet"
        ));
    }

    if !head_report_due((skin, 0, 0), false)
        && !(far_total > 0 && head_report_due((skin, 1, 0), false))
    {
        return;
    }
    head_line(format_args!(
        "bone skin {skin:08x} root {root:08x} '{}' bones {bones}, Bip01 Head node {:08x} at {:?}",
        object_name(e, root),
        nodes[head],
        head_world
    ));
    for (b, (&node, name)) in nodes.iter().zip(&names).enumerate() {
        let at = point(e, node.wrapping_add(0x8c));
        let offset = [
            at[0] - head_world[0],
            at[1] - head_world[1],
            at[2] - head_world[2],
        ];
        let local = root != 0 && hangs_under(e, node, root);
        head_line(format_args!(
            "bone skin {skin:08x} {b} node {node:08x} '{name}' {:.1} from the head, offset {:?}{}",
            distance(at, head_world),
            offset,
            if local {
                " nif-local (under the skin's root: not rebound)"
            } else {
                ""
            }
        ));
    }
    for line in lines {
        head_line(format_args!("{line}"));
    }
}

/// The `pack` report of `00e76770` for one stream of a head's geometry data (see the block
/// comment above): `entries` the stream's declaration entries (0x1c bytes each: offset +4,
/// usage +8, type +0xc), `count` of them, `step` the vertex stride, `address` where the vertices
/// were packed (the staging copy, before the unlock), `vertices` the vertex count and `chip` the
/// buffer record (its buffer +8, offset +0xc, size +0x14, lock flags +0x10).
#[allow(clippy::too_many_arguments)]
pub(crate) fn head_pack_check(
    e: &Engine,
    data: u32,
    partition: u32,
    stream: u32,
    entries: u32,
    count: u32,
    step: u32,
    address: u32,
    vertices: u32,
    chip: u32,
    flags: u32,
) {
    let positions = e.mem.u32(data.wrapping_add(0x20));
    let (map, weights, palette, per_vertex) = if partition == 0 {
        (0, 0, 0, 0)
    } else {
        (
            e.mem.u32(partition.wrapping_add(0xc)),
            e.mem.u32(partition.wrapping_add(8)),
            e.mem.u32(partition.wrapping_add(0x10)),
            u32::from(e.mem.u16(partition.wrapping_add(0x24))),
        )
    };
    let mut layout = Vec::new();
    let mut position_off = 0u32;
    let mut weight_off = 0u32;
    let mut index_off = 0u32;
    for k in 0..count.min(32) {
        let entry = entries.wrapping_add(k * 0x1c);
        let offset = e.mem.u32(entry.wrapping_add(4));
        let usage = e.mem.u32(entry.wrapping_add(8));
        let kind = e.mem.u32(entry.wrapping_add(0xc));
        layout.push(format!("u{usage}/t{kind}@{offset}"));
        match (usage, kind) {
            (0, 2) => position_off = offset + 1,
            (1, 0..=3) => weight_off = (offset + 1) | ((kind + 1) << 24),
            (2, 4 | 5) => index_off = offset + 1,
            _ => {}
        }
    }
    let mut position_bad = (0u32, 0.0f32, 0u32);
    let mut weight_bad = 0u32;
    let mut index_bad = 0u32;
    for i in 0..vertices {
        let index = if map == 0 {
            i
        } else {
            u32::from(e.mem.u16(map.wrapping_add(2 * i)))
        };
        let vertex = address.wrapping_add(i * step);
        if position_off != 0 && positions != 0 {
            let packed = point(e, vertex.wrapping_add(position_off - 1));
            let source = point(e, positions.wrapping_add(index * 12));
            let d = distance(packed, source);
            if !(d <= 1e-4) {
                position_bad.0 += 1;
                if !(d <= position_bad.1) {
                    position_bad = (position_bad.0, d, i);
                }
            }
        }
        if weight_off != 0 && weights != 0 {
            let floats = (weight_off >> 24).min(per_vertex.min(3));
            let at = vertex.wrapping_add((weight_off & 0xff_ffff) - 1);
            for j in 0..floats {
                let packed = e.mem.u32(at.wrapping_add(4 * j));
                let source = e.mem.u32(weights.wrapping_add(4 * (i * per_vertex + j)));
                if packed != source {
                    weight_bad += 1;
                }
            }
        }
        if index_off != 0 && palette != 0 {
            let at = vertex.wrapping_add(index_off - 1);
            for j in 0..per_vertex.min(4) {
                if e.mem.u8(at.wrapping_add(j))
                    != e.mem.u8(palette.wrapping_add(i * per_vertex + j))
                {
                    index_bad += 1;
                }
            }
        }
    }
    let bad = position_bad.0 + weight_bad + index_bad;
    if !head_report_due((data, partition, stream + 0x100), bad > 0) {
        return;
    }
    let buffer = e.mem.u32(chip.wrapping_add(8));
    head_line(format_args!(
        "pack data {data:08x} partition {partition:08x} stream {stream} verts {vertices} stride \
         {step} flags {flags:03x} chip {chip:08x} (buffer {buffer:08x} offset {} size {} lock {:x}) \
         at {address:08x}; layout {}; positions differing {} (worst {:.4} at {}), weight words \
         differing {weight_bad}, bone index bytes differing {index_bad}",
        e.mem.u32(chip.wrapping_add(0xc)),
        e.mem.u32(chip.wrapping_add(0x14)),
        e.mem.u32(chip.wrapping_add(0x10)),
        layout.join(" "),
        position_bad.0,
        position_bad.1,
        position_bad.2
    ));
}

/// The `pack` report of the fixed-layout packer `00e8c820` for a head's geometry data: whether
/// the positions at `positions_at` (`size` bytes apart) equal the data's.
pub(crate) fn head_plain_pack_check(
    e: &Engine,
    data: u32,
    partition: u32,
    positions_at: u32,
    size: u32,
    vertices: u32,
    mask: u32,
) {
    let positions = e.mem.u32(data.wrapping_add(0x20));
    let map = e.mem.u32(partition.wrapping_add(0xc));
    let mut bad = 0u32;
    for i in 0..vertices {
        let index = if map == 0 {
            i
        } else {
            u32::from(e.mem.u16(map.wrapping_add(2 * i)))
        };
        let packed = point(e, positions_at.wrapping_add(i * size));
        if !(distance(packed, point(e, positions.wrapping_add(index * 12))) <= 1e-4) {
            bad += 1;
        }
    }
    if head_report_due((data, partition, 0x200), bad > 0) {
        head_line(format_args!(
            "pack (00e8c820) data {data:08x} partition {partition:08x} verts {vertices} stride \
             {size} mask {mask:03x}; positions differing {bad}"
        ));
    }
}

/// The inline 3x4 rows of `00e6fe30` (`rows` is the row block + 0x20, as
/// the exe's `EBX`): rotation times the scale, row by row, with the
/// translation minus `011f474c` in each row's fourth float.
fn write_rows(e: &mut Engine, rows: u32, transform: u32) {
    let f = |e: &Engine, off: u32| x87_load32(e.mem.u32(transform + off));
    let s = f(e, 0x30);
    let at = |off: i32| rows.wrapping_add(off as u32);
    // FMULP ST2: the entry is the destination.
    e.mem.set_u32(at(-0x20), x87_store32(x87_mul(f(e, 0), s)));
    for (target, source) in [
        (-0x10, 0x0c),
        (0, 0x18),
        (-0x1c, 0x04),
        (-0xc, 0x10),
        (4, 0x1c),
        (-0x18, 0x08),
        (-8, 0x14),
    ] {
        e.mem
            .set_u32(at(target), x87_store32(x87_mul(f(e, source), s)));
    }
    let last = x87_m32(e.mem.u32(transform + 0x20));
    e.mem.set_u32(at(8), x87_store32(x87_mul(s, last)));
    for (target, source, origin) in [
        (-0x14, 0x24, ORIGIN_X),
        (-4, 0x28, ORIGIN_Y),
        (0xc, 0x2c, ORIGIN_Z),
    ] {
        let value = x87_sub(f(e, source), x87_m32(e.mem.u32(origin)));
        e.mem.set_u32(at(target), x87_store32(value));
    }
}

// Translated from 00e6fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, three words: an object whose +0x28
/// holds 0x40-byte bone matrices, a skin partition, a transform): gives the transform to the
/// render-state object's slot +0x30, builds the world matrix at +0x940 from it
/// (`00b71a40`: rotation, translation +0x24, scale +0x30) and, when the partition has at
/// most 4 bones (`u16` +0x20), hands the matrices of its first four bones (16-bit indices
/// at +4) to the device's `SetTransform` (+0xb0) as transforms 0x100 to 0x103.
pub fn fn_00e6fda0(e: &mut Engine, this: Ptr, bones: Ptr, partition: Ptr, transform: Ptr) {
    let base = this.addr();
    let t = transform.addr();
    let state = e.mem.u32(base + RENDERER_STATE_OBJECT);
    e.vcall(state, 0x30, &args![t]);
    // FLD/FSTP of the scale into the pushed word.
    let scale = x87_quiet_bits(e.mem.u32(t + 0x30));
    e.call(
        MAKE_MATRIX_00B71A40,
        &args![base + 0x940, t, t + 0x24, scale],
    );
    let partition = partition.addr();
    if e.mem.u16(partition + 0x20) <= 4 {
        let matrices = e.mem.u32(bones.addr() + 0x28);
        for i in 0..4u32 {
            let indices = e.mem.u32(partition + 4);
            let bone = u32::from(e.mem.u16(indices.wrapping_add(i * 2)));
            let device = e.mem.u32(base + RENDERER_DEVICE);
            e.vcall(
                device,
                DEVICE_SET_TRANSFORM,
                &args![i + 0x100, (bone << 6).wrapping_add(matrices)],
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_fbb0, fn_00e6fbb0(Ptr, Ptr, u8)),
        entry!(0x00e6_fe30, fn_00e6fe30(Ptr, Ptr, Ptr, u8, u32, u8)),
        entry!(0x00e6_fda0, fn_00e6fda0(Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    #[test]
    fn fn_00e6fda0_sets_the_world_and_the_bone_transforms() {
        let mut e = Engine::new();
        let state_slot = 0x0e50_0030;
        let set_transform = 0x0e50_00b0;
        e.register(state_slot, |_, _| Ret::default());
        e.register(set_transform, |_, _| Ret::default());
        e.register(MAKE_MATRIX_00B71A40, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x30, state_slot);
        e.mem.set_u32(vtable + 0xb0, set_transform);
        let state = e.mem.alloc(4);
        e.mem.set_u32(state, vtable);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let renderer = e.mem.alloc(0x980);
        e.mem.set_u32(renderer + RENDERER_STATE_OBJECT, state);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let bones = e.mem.alloc(0x2c);
        e.mem.set_u32(bones + 0x28, 0x0e60_0000);
        let indices = e.mem.alloc(8);
        for (i, bone) in [3u16, 0, 1, 7].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *bone);
        }
        let partition = e.mem.alloc(0x24);
        e.mem.set_u32(partition + 4, indices);
        e.mem.set_u16(partition + 0x20, 4);
        let transform = e.mem.alloc(TRANSFORM_SIZE);
        e.mem.set_f32(transform + 0x30, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fda0, &args![renderer, bones, partition, transform]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (state_slot, vec![state, transform]),
                (
                    MAKE_MATRIX_00B71A40,
                    vec![
                        renderer + 0x940,
                        transform,
                        transform + 0x24,
                        2.0f32.to_bits()
                    ]
                ),
                (set_transform, vec![device, 0x100, 0x0e60_00c0]),
                (set_transform, vec![device, 0x101, 0x0e60_0000]),
                (set_transform, vec![device, 0x102, 0x0e60_0040]),
                (set_transform, vec![device, 0x103, 0x0e60_01c0]),
            ]
        );
        // More than four bones: no bone transforms.
        e.mem.set_u16(partition + 0x20, 5);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fda0, &args![renderer, bones, partition, transform]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == set_transform));
    }
}

#[cfg(test)]
mod head_check_tests {
    use super::*;

    #[test]
    fn skin_point_applies_the_rows_as_the_shader_does() {
        // Rows of a 90 degree turn about z, scale 2, translation (10, 20, 30).
        let rows = [
            0.0, -2.0, 0.0, 10.0, //
            2.0, 0.0, 0.0, 20.0, //
            0.0, 0.0, 2.0, 30.0,
        ];
        assert_eq!(skin_point(&rows, [1.0, 0.0, 0.5]), [10.0, 22.0, 31.0]);
    }

    #[test]
    fn hangs_under_follows_the_parents() {
        let mut e = Engine::new();
        let root = e.mem.alloc(0x20);
        let middle = e.mem.alloc(0x20);
        let leaf = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(middle + 0x18, root);
        e.mem.set_u32(leaf + 0x18, middle);
        assert!(hangs_under(&e, leaf, root));
        assert!(hangs_under(&e, root, root));
        assert!(!hangs_under(&e, leaf, other));
        assert!(!hangs_under(&e, 0, root));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_TRANSFORM: u32 = 0x0066_00b0;
    const STATE_30: u32 = 0x0066_0030;
    const STATE_F8: u32 = 0x0066_00f8;

    /// A renderer with a device and a render-state object whose used slots
    /// are doubles.
    fn renderer(e: &mut Engine) -> u32 {
        let renderer = e.mem.alloc(0x9a0);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + DEVICE_SET_TRANSFORM, SET_TRANSFORM);
        e.mem.set_u32(vtable + 0x30, STATE_30);
        e.mem.set_u32(vtable + 0xf8, STATE_F8);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(renderer + RENDERER_DEVICE, object);
        e.mem.set_u32(renderer + RENDERER_STATE_OBJECT, object);
        e.mem
            .set_u32(renderer + RENDERER_VERTEX_SHADER, 0x0055_0000);
        for addr in [
            SET_TRANSFORM,
            STATE_30,
            STATE_F8,
            MAKE_MATRIX_00B71A40,
            MAKE_MATRIX_00E6B8F0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        renderer
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

    #[test]
    fn model_transform_sets_the_world_and_the_two_vectors() {
        let mut e = Engine::new();
        let this = renderer(&mut e);
        let t = e.mem.alloc(0x34);
        for i in 0..9 {
            e.mem.set_f32(t + i * 4, (i + 1) as f32);
        }
        e.mem.set_f32(t + 0x30, 2.0);
        for (i, v) in [1.0f32, 10.0, 100.0, 2.0, 20.0, 200.0].iter().enumerate() {
            e.mem.set_f32(this + 0x634 + i as u32 * 4, *v);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e6_fbb0, &[this, t, 1]);
        let world = this + 0x940;
        assert_eq!(
            calls_to(&e, MAKE_MATRIX_00B71A40),
            vec![vec![world, t, t + 0x24, 2.0f32.to_bits()]]
        );
        let device = e.mem.u32(this + RENDERER_DEVICE);
        assert_eq!(
            calls_to(&e, SET_TRANSFORM),
            vec![vec![device, 0x100, world]]
        );
        assert_eq!(calls_to(&e, STATE_30), vec![vec![device, t]]);
        // A = 2R: columns (2,8,14), (4,10,16), (6,12,18).
        let got: Vec<f32> = (0..6).map(|i| e.mem.f32(this + 0x64c + i * 4)).collect();
        assert_eq!(got, vec![1482.0, 1704.0, 1926.0, 2964.0, 3408.0, 3852.0]);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fbb0, &[this, t, 0x100]);
        assert!(calls_to(&e, SET_TRANSFORM).is_empty());
    }

    /// A skin with `bones` bones whose nodes have scale `bone_scale` at
    /// +0x98 (the transforms themselves come from the product double).
    fn skin(e: &mut Engine, bones: u32, bone_scale: f32) -> u32 {
        let skin = e.mem.alloc(0x2c);
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + 0x44, bones);
        e.mem.set_u32(data + 0x40, 0x0044_0000);
        e.mem.set_u32(skin + 8, data);
        e.mem.set_u32(skin + 0x10, 0x0033_0000);
        let list = e.mem.alloc(4 * bones.max(1));
        for i in 0..bones {
            let node = e.mem.alloc(0x9c);
            e.mem.set_f32(node + 0x98, bone_scale);
            e.mem.set_u32(list + i * 4, node);
        }
        e.mem.set_u32(skin + 0x14, list);
        skin
    }

    fn skin_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0106_b000, 0x3000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f64(SCALE_LOW_0106D508, 0.99);
        e.mem.set_f32(SCALE_HIGH_0106B9E0, 1.01);
        e.register(SHADER_GETTER_00E8BF90, |_, _| 0x77u32.into_ret());
        e.register(FREE_ALIGNED_00AA1110, |_, _| Ret::default());
        e.register(ALLOCATE_ALIGNED_00AA10B0, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        // The product writes a transform with rotation 1..9, translation
        // (5, 6, 7) and scale 2 into `out` and answers it.
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            for i in 0..9 {
                e.mem.set_f32(a[1] + i * 4, (i + 1) as f32);
            }
            e.mem.set_f32(a[1] + 0x24, 5.0);
            e.mem.set_f32(a[1] + 0x28, 6.0);
            e.mem.set_f32(a[1] + 0x2c, 7.0);
            e.mem.set_f32(a[1] + 0x30, 2.0);
            a[1].into_ret()
        });
        e
    }

    #[test]
    fn skin_matrices_are_kept_when_nothing_changed() {
        let mut e = skin_engine();
        let this = renderer(&mut e);
        let skin = skin(&mut e, 2, 1.0);
        e.mem.set_u32(skin + 0x18, 0x77);
        e.mem.set_u32(skin + 0x20, 4);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fe30, &[this, skin, 0x9000, 0, 4, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn skin_matrices_with_four_registers() {
        let mut e = skin_engine();
        let this = renderer(&mut e);
        let skin = skin(&mut e, 2, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fe30, &[this, skin, 0x9000, 0, 4, 0]);
        let buffer = e.mem.u32(skin + 0x28);
        assert_eq!(
            calls_to(&e, ALLOCATE_ALIGNED_00AA10B0),
            vec![vec![0x80, 0x10]]
        );
        assert_eq!(e.mem.u32(skin + 0x24), 0x80);
        assert_eq!(e.mem.u32(skin + 0x18), 0x77);
        assert_eq!(e.mem.u32(skin + 0x1c), 2);
        assert_eq!(e.mem.u32(skin + 0x20), 4);
        assert_eq!(e.mem.f32(buffer + 0x3c), 1.0);
        assert_eq!(e.mem.f32(buffer + 0x7c), 1.0);
        let builds = calls_to(&e, MAKE_MATRIX_00B71A40);
        assert_eq!(builds.len(), 2);
        assert_eq!(builds[1][0], buffer + 0x40);
        assert_eq!(builds[0][3], 2.0f32.to_bits());
        // Root, world and per bone: product + world product.
        assert_eq!(calls_to(&e, TRANSFORM_MULTIPLY).len(), 6);
        let state = e.mem.u32(this + RENDERER_STATE_OBJECT);
        assert_eq!(calls_to(&e, STATE_F8), vec![vec![state, 0]]);
        // Layout set and the world skipped: the other builder, two products.
        let skin2 = self::skin(&mut e, 1, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fe30, &[this, skin2, 0x9000, 1, 4, 1]);
        assert_eq!(calls_to(&e, MAKE_MATRIX_00E6B8F0).len(), 1);
        assert_eq!(calls_to(&e, TRANSFORM_MULTIPLY).len(), 3);
        let buffer = e.mem.u32(skin2 + 0x28);
        assert_eq!(e.mem.u32(buffer + 0x30), 0);
    }

    #[test]
    fn skin_matrices_with_three_registers_are_written_inline() {
        let mut e = skin_engine();
        e.mem.set_f32(ORIGIN_X, 1.0);
        e.mem.set_f32(ORIGIN_Y, 1.0);
        e.mem.set_f32(ORIGIN_Z, 1.0);
        let this = renderer(&mut e);
        let skin = skin(&mut e, 1, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fe30, &[this, skin, 0x9000, 0, 3, 0]);
        let buffer = e.mem.u32(skin + 0x28);
        let got: Vec<f32> = (0..12).map(|i| e.mem.f32(buffer + i * 4)).collect();
        assert_eq!(
            got,
            vec![2.0, 4.0, 6.0, 4.0, 8.0, 10.0, 12.0, 5.0, 14.0, 16.0, 18.0, 6.0]
        );
        assert!(calls_to(&e, MAKE_MATRIX_00B71A40).is_empty());
    }

    #[test]
    fn scale_test_needs_both_bounds() {
        let mut e = skin_engine();
        e.mem.set_f32(SCALE_HIGH_0106B9E0, 0.5);
        let this = renderer(&mut e);
        let skin = skin(&mut e, 1, 0.75);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fe30, &[this, skin, 0x9000, 0, 4, 0]);
        let state = e.mem.u32(this + RENDERER_STATE_OBJECT);
        assert_eq!(calls_to(&e, STATE_F8), vec![vec![state, 1]]);
        assert!(bone_scale_outside(&e, 0.75f32.to_bits()));
        assert!(!bone_scale_outside(&e, 0.4f32.to_bits()));
        assert!(!bone_scale_outside(&e, f32::NAN.to_bits()));
    }

    #[test]
    fn skin_matrix_fault_passes_rotations_and_reports_stretch() {
        let identity = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        assert!(skin_matrix_fault(identity, 1.0).is_none());
        // A quarter turn about Z is still a rotation.
        let turn = [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        assert!(skin_matrix_fault(turn, 1.0).is_none());
        // One column stretched by 1.5 (as a non-unit quaternion turned into a matrix gives).
        let stretched = [1.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        assert!(skin_matrix_fault(stretched, 1.0).is_some());
        // Sheared columns.
        let sheared = [1.0, 0.3, 0.0, 0.0, 0.954, 0.0, 0.0, 0.0, 1.0];
        assert!(skin_matrix_fault(sheared, 1.0).is_some());
        // A wild scale or a NaN.
        assert!(skin_matrix_fault(identity, 3.0).is_some());
        assert!(skin_matrix_fault([f32::NAN; 9], 1.0).is_some());
    }
}

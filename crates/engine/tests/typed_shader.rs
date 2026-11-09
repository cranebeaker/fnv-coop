//! The differential test of every typed function of the shader family (`engine_typed::shader`)
//! against its translation. One world serves every test: a renderer with its state cache and device,
//! a texture stage with its cache, a shader, plain and skinned geometry with their properties, passes,
//! a batch renderer with a pass map, groups, holders and an active list, the node pool, an accumulator,
//! a constant map, a light list. Each test varies the words and floats its function reads.
//!
//! What stands in: every virtual function the code calls on a device, shader, geometry, skin, group,
//! map or property (test functions past `.text` answering a word per slot from a table), and the
//! callees outside the family: the map's `GetAt` and `SetAt`, the node and memory allocators and
//! frees, the lock, the shadow-pass test, the shaders' own set-ups, the renderer's skinned test, the
//! D3DX helpers, `atexit` and the interlocked imports. The render state cache's `SetRenderState` and
//! the batch renderer's `AddPass` are the real (family) functions.

use engine as _;
use engine_core::prelude::*;
use engine_core::typed::diff::{Case, Diff, Input};
use engine_core::{entry, Engine};
use engine_typed::culling;
use engine_typed::shader::{self, accumulator as acc, batch, state};

/// The stand-in virtual functions, one per slot, and the table of what each answers.
const VT_FN: u32 = 0x00ff_2000;
const RETS: u32 = 0x0300_0000;
/// The stand-in of the pass map's hash (its vtable's slot 4).
const MAP_HASH: u32 = 0x00ff_2800;
const MAX_CALLS: usize = 5_000;
const FLOAT_SWEEP: usize = 48;

const ONE: u32 = 0x3f80_0000;
const TWO: u32 = 0x4000_0000;
const HALF: u32 = 0x3f00_0000;

fn ret(eax: u32) -> Ret {
    Ret {
        eax,
        ..Ret::default()
    }
}

fn slot_answer<const S: u32>(e: &mut Engine, _: &[u32]) -> Ret {
    ret(e.mem.u32(RETS + S))
}

macro_rules! slots {
    ($e:ident; $($s:literal),* $(,)?) => { $( $e.register(VT_FN + $s, slot_answer::<$s>); )* };
}

fn set(e: &mut Engine, at: u32, value: u32) {
    e.mem.set_u32(at, value);
}

fn bits(case: &Case, i: usize) -> u32 {
    case.f32(i).to_bits()
}

/// A vtable of 0x200 bytes of stand-ins, with `real` slots pointing at real functions.
fn vtable(e: &mut Engine, real: &[(u32, u32)]) -> u32 {
    let vt = e.mem.alloc(0x200);
    for slot in (0..0x200).step_by(4) {
        let target = real
            .iter()
            .find(|(s, _)| *s == slot)
            .map_or(VT_FN + slot, |(_, a)| *a);
        set(e, vt + slot, target);
    }
    vt
}

/// An object of `size` bytes with vtable `vt`.
fn obj(e: &mut Engine, size: u32, vt: u32) -> u32 {
    let o = e.mem.alloc(size);
    set(e, o, vt);
    o
}

/// The callees outside the family.
fn stand_ins(e: &mut Engine) {
    e.map(RETS, 0x1000);
    slots!(e; 0x04, 0x08, 0x24, 0x30, 0x34, 0x64, 0x6c, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0xa0,
        0xa4, 0xa8, 0xac, 0xb0, 0xc0, 0xcc, 0xdc, 0xe0, 0xe4, 0xf0, 0xf4, 0xf8, 0xfc, 0x100, 0x104,
        0x10c, 0x110, 0x114, 0x15c, 0x170, 0x178, 0x1ac);
    // The pass map: `GetAt(map, key, out)` finds keys below 0x200, value `key & 3`.
    e.register(0x0085_3130, |e, a| {
        e.mem.set_u32(a[2], a[1] & 3);
        ret(u32::from(a[1] < 0x200))
    });
    e.register(0x0084_4700, |e, a| {
        let n = e.mem.u32(a[0] + 0xc);
        e.mem.set_u32(a[0] + 0xc, n + 1);
        ret(0)
    });
    // The node allocator, memory manager, operator new, frees and clears.
    e.register(0x0043_a010, |e, _| ret(e.mem.alloc(0xc)));
    e.register(0x00aa_3e40, |e, a| ret(e.mem.alloc(a[1].max(4))));
    e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0].max(4))));
    e.register(0x00aa_4060, |_, _| ret(0));
    e.register(0x00b6_4ec0, |_, _| ret(0));
    // The node pool's lock: taken once more.
    e.register(0x0040_fbf0, |e, _| {
        let n = e.mem.u32(0x011c_5f84);
        e.mem.set_u32(0x011c_5f84, n + 1);
        e.mem.set_u32(0x011c_5f80, 1);
        ret(0)
    });
    // The shadow-pass test: odd types.
    e.register(0x00b6_30f0, |_, a| ret(a[0] & 1));
    for addr in [
        0x00b7_dde0,
        0x00b7_ded0,
        0x00b7_dfe0,
        0x00b7_e150,
        0x00b7_dd50,
        0x00b7_deb0,
        0x00e6_d310,
        0x00b7_ac20,
        0x00bb_96d0,
        0x00b7_a870,
        0x00b6_3320,
        0x00b9_8070,
        0x00ec_658f,
    ] {
        e.register(addr, |_, _| ret(0));
    }
    // The renderer's skinned test: the geometry has a skin.
    e.register(0x00e7_2c20, |e, a| {
        ret(u32::from(e.mem.u32(a[1] + 0xbc) != 0))
    });
    e.register(0x00b5_5560, |e, _| ret(e.mem.u32(RETS + 0x1f0)));
    // The D3DX helpers: a mark into the out word.
    for addr in [0x00ee_6dfe, 0x00ee_6df2, 0x00ee_6dec] {
        e.register(addr, |e, a| {
            e.mem.set_u32(a[0], 0x3f80_0001);
            ret(a[0])
        });
    }
    e.register(0x00fd_f1c0, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_add(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    e.register(0x00fd_f1c4, |e, a| {
        let n = e.mem.u32(a[0]).wrapping_sub(1);
        e.mem.set_u32(a[0], n);
        ret(n)
    });
    // The pass map's hash: the key modulo the bucket count.
    e.register(MAP_HASH, |e, a| {
        let size = e.mem.u32(a[0] + 4);
        ret(if size == 0 { 0 } else { a[1] % size })
    });
    // The active list's pass-type comparison.
    e.register(0x00b9_88a0, |_, a| {
        ret((a[0].wrapping_sub(a[1]) as i32).signum() as u32)
    });
    for (addr, len) in [
        (0x0101_0000, 0x10000),
        (0x0106_b000, 0x3000),
        (0x011a_9000, 0x6000),
        (0x011b_f000, 0x1000),
        (0x011c_5000, 0x1000),
        (0x011f_4000, 0xc000),
        (0x0126_f000, 0x1000),
    ] {
        e.map(addr, len);
    }
}

/// The world's objects.
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct W {
    renderer: u32,
    cache: u32,
    device: u32,
    stage_cache: u32,
    stage: u32,
    shader: u32,
    geometry: u32,
    skinned: u32,
    skin: u32,
    property: u32,
    alpha: u32,
    data: u32,
    passes: [u32; 4],
    skin_pass: u32,
    br: u32,
    holders: [u32; 13],
    list: u32,
    acc: u32,
    camera: u32,
    cmap: u32,
    light: u32,
    ssn: u32,
    transform: u32,
}

/// A pass list at `at` with the items `items` and one spare node.
fn list_at(e: &mut Engine, at: u32, items: &[u32]) {
    let mut prev = 0;
    for item in items {
        let node = e.mem.alloc(0xc);
        set(e, node + 4, prev);
        set(e, node + 8, *item);
        if prev != 0 {
            set(e, prev, node);
        } else {
            set(e, at + 4, node);
        }
        prev = node;
    }
    set(e, at + 8, prev);
    let spare = e.mem.alloc(0xc);
    set(e, at + 0xc, spare);
    set(e, at + 0x10, items.len() as u32);
}

fn world(e: &mut Engine) -> W {
    stand_ins(e);
    let vt = vtable(e, &[]);
    let cache_vt = vtable(e, &[(0x68, 0x00e8_8780)]);
    let br_vt = vtable(e, &[(0x8c, 0x00b9_9c90)]);
    let device = obj(e, 0x10, vt);
    let cache = obj(e, 0x1114, cache_vt);
    set(e, cache + 0x10f8, device);
    for i in 0..8 {
        set(e, cache + 0xd4 + 4 * i, 0x100 + i);
    }
    let renderer = obj(e, 0xa40, vt);
    set(e, renderer + 0x288, device);
    let manager = obj(e, 0x10, vt);
    let group_obj = obj(e, 0x10, vt);
    set(e, renderer + 0x8ac, manager);
    set(e, renderer + 0x8b0, group_obj);
    set(e, renderer + 0x8b8, cache);
    for (i, w) in [ONE, HALF, TWO, 0x3eaa_aaab, ONE, 0xbf80_0000]
        .iter()
        .enumerate()
    {
        set(e, renderer + 0x634 + 4 * i as u32, *w);
    }
    set(e, 0x011f_9508, renderer);
    set(e, 0x011f_4748, renderer);
    let stage_cache = e.mem.alloc(0xb8);
    let stage = e.mem.alloc(0x14);
    set(e, stage + 4, 1);
    set(e, stage + 0x10, stage_cache);
    let target = obj(e, 0x10, vt);
    set(e, 0x0126_f99c, target);
    // The slot tables.
    for s in 0..0x20u32 {
        e.mem.set_u16(0x0126_f958 + 2 * s, (s % 10) as u16);
    }
    for s in 0..0x16u32 {
        e.mem.set_u16(0x0126_f92c + 2 * s, (s % 7) as u16);
    }
    for i in 0..8u32 {
        e.mem.set_u16(0x0126_f948 + 2 * i, (0x10 + i) as u16);
        set(e, 0x0126_f0b0 + 8 * i, 0x31 + i);
        set(e, 0x0126_f0b4 + 8 * i, 0x41 + i);
        for k in 0..3 {
            set(e, 0x0126_f050 + 12 * i + 4 * k, 0x51 + 0x10 * k + i);
        }
    }
    for i in 0..5u32 {
        set(e, 0x0126_f918 + 4 * i, 0x20 + i);
    }
    set(e, 0x011b_f558, 4);
    // The setters' counter indices (by state) and the function and blend tables.
    for s in 0..0xb0u32 {
        set(e, 0x011f_fa30 + 4 * s, s % 8);
    }
    for i in 0..8u32 {
        set(e, 0x011a_d888 + 4 * i, 0x200 + i);
    }
    for i in 0..16u32 {
        set(e, 0x011a_d8a8 + 4 * i, 0x300 + i);
    }
    // A shader: kind at +0x68, two objects at +0x30 and +0x34.
    let shader = obj(e, 0x80, vt);
    let first = obj(e, 0x10, vt);
    let second = obj(e, 0x10, vt);
    set(e, shader + 0x30, first);
    set(e, shader + 0x34, second);
    set(e, RETS + 0x1f0, shader);
    // Properties: alpha (blends, bit 9), shade (both bits 10 and 11), the shader property.
    let alpha = obj(e, 0x1c, vt);
    e.mem.set_u16(alpha + 0x18, 0x0201 | (5 << 1) | (9 << 5));
    let shade = obj(e, 0x1c, vt);
    e.mem.set_u16(shade + 0x18, 0xc00);
    let property = obj(e, 0xc0, vt);
    set(e, property + 0x20, 0x40080);
    set(e, property + 0x24, 0x20);
    set(e, property + 0x28, ONE);
    set(e, property + 0x2c, ONE);
    let textures = e.mem.alloc(0x10);
    set(e, textures + 4, 0x7777);
    set(e, property + 0xb0, textures);
    set(e, 0x011f_9518, 0x5518);
    set(e, 0x011f_9510, 0x5510);
    // Geometry: bound, data, the skin.
    let bound = e.mem.alloc(0x10);
    for (i, w) in [ONE, TWO, 0x4040_0000, HALF].iter().enumerate() {
        set(e, bound + 4 * i as u32, *w);
    }
    let data = e.mem.alloc(0x38);
    e.mem.set_u16(data + 0xe, 0xabcd);
    let geometry = obj(e, 0xc4, vt);
    set(e, geometry + 0x20, bound);
    set(e, geometry + 0x9c, alpha);
    set(e, geometry + 0xa8, property);
    set(e, geometry + 0xac, shade);
    set(e, geometry + 0xb8, data);
    set(e, geometry + 0xc0, shader);
    let skin = obj(e, 0x40, vt);
    let partitions = e.mem.alloc(0x10);
    set(e, partitions + 8, 3);
    let parts = e.mem.alloc(0x84);
    set(e, partitions + 0xc, parts);
    set(e, skin + 0xc, partitions);
    let flags = e.mem.alloc(0xc);
    for (i, b) in [1u8, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0].iter().enumerate() {
        e.mem.set_u8(flags + i as u32, *b);
    }
    set(e, skin + 0x38, flags);
    e.mem.set_u8(skin + 0x3c, 1);
    let skinned = obj(e, 0xc4, vt);
    for off in [0x20, 0x9c, 0xa8, 0xac, 0xb8, 0xc0] {
        let w = e.mem.u32(geometry + off);
        set(e, skinned + off, w);
    }
    set(e, skinned + 0xbc, skin);
    let pass_of = |e: &mut Engine, geometry: u32, pass_type: u16, kind: u8, flag: u8| {
        let p = e.mem.alloc(8);
        set(e, p, geometry);
        e.mem.set_u16(p + 4, pass_type);
        e.mem.set_u8(p + 6, kind);
        e.mem.set_u8(p + 7, flag);
        p
    };
    let passes = [
        pass_of(e, geometry, 0x10, 2, 1),
        pass_of(e, geometry, 0x10, 1, 1),
        pass_of(e, geometry, 0x11, 4, 0),
        pass_of(e, geometry, 0xfc, 0, 1),
    ];
    let skin_pass = pass_of(e, skinned, 0x10, 3, 1);
    // The batch renderer: four groups, a map with three keys, an active list, holders.
    let br = obj(e, 0x98, br_vt);
    let groups = e.mem.alloc(4 * 0x68);
    set(e, br + 8, groups);
    let map_vt = vtable(e, &[(4, MAP_HASH)]);
    set(e, br + 0xc, map_vt);
    set(e, br + 0x10, 4);
    let buckets = e.mem.alloc(0x10);
    let item2 = e.mem.alloc(0xc);
    set(e, item2 + 4, 0x15);
    set(e, item2 + 8, 2);
    let item1 = e.mem.alloc(0xc);
    set(e, item1, item2);
    set(e, item1 + 4, 0x11);
    set(e, item1 + 8, 1);
    set(e, buckets + 4, item1);
    let item3 = e.mem.alloc(0xc);
    set(e, item3 + 4, 0x13);
    set(e, buckets + 0xc, item3);
    set(e, br + 0x14, buckets);
    set(e, br + 0x18, 2);
    set(e, br + 0x20, 0x10);
    set(e, br + 0x24, 0x30);
    e.mem.set_u16(br + 0x2c, 0x10);
    let entry2 = e.mem.alloc(8);
    e.mem.set_u16(entry2, 0x11);
    set(e, br + 0x30, entry2);
    e.mem.set_u8(br + 0x38, 1);
    list_at(e, groups + 0x68, &[passes[0], passes[1]]);
    list_at(e, groups + 0x68 + 2 * 0x14, &[passes[2]]);
    set(e, groups + 0x68 + 0x64, 3);
    list_at(e, groups, &[skin_pass]);
    set(e, groups + 0x64, 1);
    let mut holders = [0u32; 13];
    for (i, h) in holders.iter_mut().enumerate() {
        *h = e.mem.alloc(0x20);
        e.mem.set_u8(*h, (i & 1) as u8);
        e.mem.set_u16(*h + 2, 2);
        set(e, *h + 4, 4);
        set(e, *h + 8, br);
        list_at(e, *h + 0xc, &[passes[i % 4]]);
        set(e, br + 0x3c + 4 * i as u32, *h);
    }
    for i in 0..8u32 {
        set(e, br + 0x70 + 4 * i, holders[i as usize]);
    }
    set(e, br + 0x90, holders[3]);
    set(e, br + 0x94, holders[4]);
    // The node pool: two free nodes.
    let free = e.mem.alloc(0xc);
    let free2 = e.mem.alloc(0xc);
    set(e, free, free2);
    set(e, 0x011c_5f58, free);
    let list = e.mem.alloc(0x14);
    list_at(e, list, &[passes[0], passes[2], passes[1]]);
    // The accumulator: a camera, a sorted pair, the geometry list inline, the batch renderers.
    let acc = obj(e, 0x1ac, vt);
    set(e, acc + 8, 1);
    let (n1, n2) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
    set(e, n1, n2);
    set(e, n1 + 8, geometry);
    set(e, n2 + 4, n1);
    set(e, n2 + 8, skinned);
    set(e, acc + 0xc, n1);
    set(e, acc + 0x10, n2);
    set(e, acc + 0x14, 2);
    set(e, acc + 0x1c, 2);
    let sorted = e.mem.alloc(8);
    set(e, sorted, geometry);
    set(e, sorted + 4, geometry);
    set(e, acc + 0x24, sorted);
    e.mem.set_u16(acc + 0x1a8, 0x10);
    set(e, acc + 0x174, br);
    set(e, acc + 0x178, br);
    set(e, acc + 0x18c, 2);
    let tls = e.tls();
    set(e, tls + 0x2bc, 1);
    set(e, 0x011f_9f80, 0x00b6_4020);
    let render_passes = e.mem.alloc(0x14);
    let pass_array = e.mem.alloc(0x10);
    for (i, p) in passes.iter().enumerate() {
        set(e, pass_array + 4 * i as u32, *p);
    }
    set(e, render_passes + 4, pass_array);
    set(e, render_passes + 0x10, 4);
    set(e, RETS + 0xa0, render_passes);
    set(e, RETS + 0xa8, render_passes);
    set(e, RETS + 0x6c, 1);
    // The alpha sort's camera: a direction at +0x68, +0x74, +0x80.
    let camera = e.mem.alloc(0x90);
    set(e, camera + 0x68, 0x3f35_04f3);
    set(e, camera + 0x74, 0x3f13_cd3a);
    set(e, camera + 0x80, 0xbf5d_b3d7);
    // A constant map: two arrays of four entries, enabled bytes 0, 1, 0, 1.
    let cmap = e.mem.alloc(0x2c);
    for array in [cmap + 0xc, cmap + 0x1c] {
        let base = e.mem.alloc(0x10);
        for i in 0..4u32 {
            let entry = obj(e, 0xc, vt);
            set(e, entry + 4, 1);
            e.mem.set_u8(entry + 8, (i & 1) as u8);
            set(e, base + 4 * i, entry);
        }
        set(e, array + 4, base);
        e.mem.set_u16(array + 8, 4);
        e.mem.set_u16(array + 0xa, 4);
        e.mem.set_u16(array + 0xc, 4);
    }
    set(e, 0x0126_f914, 1);
    // A light, a node's light list of three.
    let light = e.mem.alloc(0x44);
    for (i, w) in [HALF, ONE, 0x3dcc_cccd].iter().enumerate() {
        set(e, light + 0x28 + 4 * i as u32, *w);
    }
    set(e, light + 0x40, HALF);
    let ssn = e.mem.alloc(0xc0);
    let mut next = 0;
    for i in (0..3u32).rev() {
        let node = e.mem.alloc(0xc);
        set(e, node, next);
        set(e, node + 8, 0x100 + i);
        next = node;
    }
    set(e, ssn + 0xb4, next);
    set(e, ssn + 0xbc, 3);
    // Lights and colours, the current pass's stages and vertex shader, the drawn object.
    for (i, w) in [ONE, ONE, ONE].iter().enumerate() {
        set(e, 0x011f_4998 + 4 * i as u32, *w);
    }
    for (i, w) in [HALF, 0x3e80_0000, 0x3dcc_cccd, ONE].iter().enumerate() {
        set(e, 0x011f_a0c0 + 4 * i as u32, *w);
    }
    let effect_data = e.mem.alloc(0x10);
    set(e, effect_data + 0xc, TWO);
    let effect = e.mem.alloc(0x70);
    set(e, effect + 0x6c, effect_data);
    let effects = e.mem.alloc(8);
    set(e, effects + 4, effect);
    let manager = e.mem.alloc(0x10);
    set(e, manager + 8, effects);
    set(e, 0x011f_91ac, manager);
    for (i, w) in [HALF, ONE, 0x3e80_0000, ONE].iter().enumerate() {
        set(e, 0x011a_9bd0 + 4 * i as u32, *w);
    }
    set(e, 0x0101_8f5c, 0x3e80_0000);
    e.mem.set_u64(0x0101_a6b0, (-1.0f64).to_bits());
    e.mem.set_u64(0x0106_d508, 0.25f64.to_bits());
    set(e, 0x0106_b9e0, 0x3f60_0000);
    for (i, w) in [ONE, 0xc000_0000, 0x4110_0000].iter().enumerate() {
        set(e, 0x011f_474c + 4 * i as u32, *w);
    }
    let current = e.mem.alloc(0x60);
    let vertex_shader = e.mem.alloc(0x24);
    set(e, vertex_shader + 0x20, device);
    set(e, current + 0x5c, vertex_shader);
    let stages = e.mem.alloc(0x40);
    for i in 0..16u32 {
        let s = e.mem.alloc(0xc);
        set(e, stages + 4 * i, s);
    }
    set(e, current + 0x24, stages);
    set(e, 0x0126_f74c, current);
    let vector = e.mem.alloc(0x100);
    for (i, w) in [ONE, HALF, 0xbf80_0000].iter().enumerate() {
        set(e, vector + 0xf0 + 4 * i as u32, *w);
    }
    let object = e.mem.alloc(0x100);
    set(e, object + 0xf8, vector);
    let holder = e.mem.alloc(4);
    set(e, holder, object);
    let rendering = e.mem.alloc(0x10);
    set(e, rendering + 0xc, holder);
    set(e, 0x011f_91e0, rendering);
    // A transform: a rotation of a few degrees, a translation, a scale.
    let transform = e.mem.alloc(0x34);
    for (i, w) in [
        0x3f7e_8a6e,
        0xbdcc_6c67,
        0,
        0x3dcc_6c67,
        0x3f7e_8a6e,
        0,
        0,
        0,
        ONE,
        ONE,
        TWO,
        HALF,
        TWO,
    ]
    .iter()
    .enumerate()
    {
        set(e, transform + 4 * i as u32, *w);
    }
    W {
        renderer,
        cache,
        device,
        stage_cache,
        stage,
        shader,
        geometry,
        skinned,
        skin,
        property,
        alpha,
        data,
        passes,
        skin_pass,
        br,
        holders,
        list,
        acc,
        camera,
        cmap,
        light,
        ssn,
        transform,
    }
}

trait Common {
    fn common(self) -> Diff;
}

impl Common for Diff {
    fn common(self) -> Diff {
        self.family(&shader::SHADER)
            .family(&leaf::LEAVES)
            // The plain geometry pass (pass types 0xfc, 0xfd) is the render family's (its own test).
            .goes_by_address(&[0x00b7_dde0])
            .max_calls(MAX_CALLS)
            .max_cases(160)
    }
}

const BYTES: &[u32] = &[0, 1, 0x101];
const LOCKS: &[u32] = &[0, 1, u32::MAX];
const TWO_WAYS: &[u32] = &[0, 1];

/// The counter of index `i` (`011ff9d8`).
fn counter(e: &mut Engine, i: u32, value: u32) {
    set(e, 0x011f_f9d8 + 4 * i, value);
}

/// The render state cache's lock byte and cached value of `state`.
fn cached_state(e: &mut Engine, w: &W, state: u32, locked: u32, value: u32) {
    e.mem.set_u8(w.cache + 0x920 + state, locked as u8);
    set(e, w.cache + 0x120 + 8 * state, value);
}

#[test]
fn z_enable() {
    Diff::new(entry!(0x00b9_7de0, state::set_z_enable(u8, u32)))
        .common()
        .inputs(&[
            Input::Of(BYTES),
            Input::Of(LOCKS),
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            counter(e, 7, c.word(2));
            cached_state(e, &w, 7, c.word(3), 1);
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn z_write_enable() {
    Diff::new(entry!(0x00b9_7e30, state::set_z_write_enable(u8, u32)))
        .common()
        .inputs(&[Input::Of(BYTES), Input::Of(LOCKS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 6, c.word(2));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn alpha_test_enable() {
    Diff::new(entry!(0x00b9_7ed0, state::set_alpha_test_enable(u8, u32)))
        .common()
        .inputs(&[Input::Of(BYTES), Input::Of(LOCKS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 7, c.word(2));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn alpha_blend_enable() {
    Diff::new(entry!(0x00b9_7fa0, state::set_alpha_blend_enable(u8, u32)))
        .common()
        .inputs(&[Input::Of(BYTES), Input::Of(LOCKS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 3, c.word(2));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn color_write() {
    Diff::new(entry!(0x00b9_8380, state::set_color_write(u32, u32)))
        .common()
        .inputs(&[Input::Word, Input::Of(LOCKS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 0, c.word(2));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn z_function() {
    Diff::new(entry!(0x00b9_7e80, state::set_z_function(u32, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 7]), Input::Of(LOCKS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 7, c.word(2));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn alpha_function() {
    Diff::new(entry!(
        0x00b9_7f20,
        state::set_alpha_function(u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 3, 7]),
        Input::Word,
        Input::Of(LOCKS),
        Input::Of(TWO_WAYS),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let _w = world(e);
        counter(e, 1, c.word(3));
        counter(e, 0, 1 - c.word(3));
        vec![c.word(0), c.word(1), c.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn blend() {
    Diff::new(entry!(0x00b9_7ff0, state::set_blend(u32, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 6, 15]),
            Input::Of(&[1, 7]),
            Input::Of(LOCKS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, 3, c.word(3));
            vec![c.word(0), c.word(1), c.word(2)]
        })
        .run()
        .assert_same();
}

#[test]
fn any_state() {
    Diff::new(entry!(0x00b9_7d90, state::set_state(u32, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[7, 0x16, 0x1b, 0xa8]),
            Input::Word,
            Input::Of(LOCKS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            counter(e, c.word(0) % 8, c.word(3));
            vec![c.word(0), c.word(1), c.word(2)]
        })
        .run()
        .assert_same();
}

#[test]
fn alpha_to_coverage() {
    Diff::new(entry!(0x00b9_8540, state::set_alpha_to_coverage(u8, u32)))
        .common()
        .inputs(&[
            Input::Of(BYTES),
            Input::Of(LOCKS),
            Input::Of(&[0, 1, 2, 3]),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            set(e, 0x011f_94b8, c.word(2));
            set(e, 0x011f_fa28, c.word(3));
            vec![c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn cull_mode_of_renderer() {
    Diff::new(entry!(
        0x00b9_84d0,
        state::set_cull_mode_of_renderer(u32, u32)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 3]), Input::Word, Input::Of(TWO_WAYS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.cache + 0xf4, c.word(2));
        vec![c.word(0), c.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn cull_mode() {
    Diff::new(entry!(0x00e8_79b0, state::set_cull_mode(Ptr<_>, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 3]),
            Input::Word,
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.cache + 0xf4, c.word(2));
            cached_state(e, &w, 0x16, c.word(3), 0x101);
            vec![w.cache, c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn render_state() {
    Diff::new(entry!(
        0x00e8_8780,
        state::set_render_state(Ptr<_>, u32, u32, u32, u8)
    ))
    .common()
    .inputs(&[
        Input::Of(&[7, 0x1b, 0xa8]),
        Input::Of(&[0, 1, 5]),
        Input::Word,
        Input::Of(BYTES),
        Input::Of(TWO_WAYS),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        cached_state(e, &w, c.word(0), c.word(4), 1);
        vec![w.cache, c.word(0), c.word(1), c.word(2), c.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn stage_state() {
    Diff::new(entry!(
        0x00e8_8930,
        state::set_stage_state(Ptr<_>, u32, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 3]),
        Input::Of(&[1, 8, 9, 0x13]),
        Input::Of(&[0, 1, 0x55]),
        Input::Word,
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.cache + 0xa20 + 8 * (8 + 1), 1);
        vec![w.cache, c.word(0), c.word(1), c.word(2), c.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn sampler_state() {
    Diff::new(entry!(
        0x00e9_10a0,
        state::set_sampler_state(Ptr<_>, u32, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 3]),
        Input::Of(&[1, 5, 6, 0x15]),
        Input::Of(&[0, 1, 0x55]),
        Input::Word,
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.cache + 0xe20 + 8 * (5 + 1), 1);
        vec![w.cache, c.word(0), c.word(1), c.word(2), c.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn texture() {
    Diff::new(entry!(0x00e8_8a20, state::set_texture(Ptr<_>, u32, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 15]), Input::Of(&[0, 0x1234])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.cache + 0x10a0 + 4, 0x1234);
            vec![w.cache, c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn vertex_shader() {
    Diff::new(entry!(
        0x00e9_0e00,
        state::set_vertex_shader(Ptr<_>, u32, u8)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 0x40, 0x41]), Input::Of(BYTES)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.cache + 0x10e0, 0x40);
        vec![w.cache, c.word(0), c.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn pixel_shader() {
    Diff::new(entry!(
        0x00e8_8870,
        state::set_pixel_shader(Ptr<_>, u32, u8)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 0x40, 0x41]), Input::Of(BYTES)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.cache + 0x10e8, 0x40);
        vec![w.cache, c.word(0), c.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn declaration() {
    Diff::new(entry!(0x00e9_0fc0, state::set_declaration(Ptr<_>, u32, u8)))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x40, 0x41]),
            Input::Of(BYTES),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            e.mem.set_u8(w.cache + 0x1100, c.word(2) as u8);
            set(e, w.cache + 0x110c, 0x40);
            vec![w.cache, c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn alpha_test_of() {
    Diff::new(entry!(0x00e8_79e0, state::set_alpha_test_of(Ptr<_>, u32)))
        .common()
        .inputs(&[
            Input::F32,
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
            Input::Of(&[0, 0x7ff8_0000]),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            let object = e.mem.alloc(0x34);
            set(e, object + 0x30, bits(c, 0));
            e.mem.set_u8(w.cache + 0x10f4, c.word(1) as u8);
            e.mem.set_u8(w.cache + 0x10f5, c.word(2) as u8);
            if c.word(3) != 0 {
                e.mem.set_u64(0x0106_d508, 0x7ff8_0000_0000_0000);
            }
            vec![w.cache, object]
        })
        .run()
        .assert_same();
}

#[test]
fn range() {
    Diff::new(entry!(0x00e8_8730, state::set_range(Ptr<_>, f32, f32)))
        .common()
        .inputs(&[Input::F32, Input::F32])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|_e, c| {
            let w = world(_e);
            vec![w.cache, bits(c, 0), bits(c, 1)]
        })
        .run()
        .assert_same();
}

#[test]
fn cull_side() {
    Diff::new(entry!(0x00e8_8750, state::cull_side_set(Ptr<_>) -> bool))
        .common()
        .inputs(&[Input::Word])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.cache + 0xf4, c.word(0));
            vec![w.cache]
        })
        .run()
        .assert_same();
}

/// The stage cache's set bytes from the bits of `mask` (A 0-7, B 8-15, C 16-20, D 21-25, E 26-30).
fn stage_banks(e: &mut Engine, cache: u32, mask: u32) {
    let banks = [
        (0x2c, 8, 0),
        (0x5c, 8, 8),
        (0x80, 5, 16),
        (0xa4, 5, 21),
        (0xb0, 5, 26),
    ];
    for (at, n, shift) in banks {
        for i in 0..n {
            e.mem
                .set_u8(cache + at + i, ((mask >> (shift + i)) & 1) as u8);
            set(e, cache + 0xc + 4 * (i % 8), 0x700 + i);
        }
    }
    for (i, at) in [0x3cu32, 0x6c, 0x90].iter().enumerate() {
        for k in 0..5 {
            set(e, cache + at + 4 * k, 0x800 + 0x10 * i as u32 + k);
        }
    }
    set(e, cache + 0x64, 3);
    set(e, cache + 0x88, 2);
    set(e, cache + 0xac, 1);
}

const MASKS: &[u32] = &[0, 0x0421_0842, 0x7fff_ffff, 0x0a5a_5a5a];

#[test]
fn cached_stage_state() {
    Diff::new(entry!(
        0x00e8_9060,
        state::cached_stage_state(Ptr<_>, u32, u32, u32) -> bool
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 7, 8, 9]), Input::Of(MASKS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        stage_banks(e, w.stage_cache, c.word(1));
        let out = e.mem.alloc(8);
        vec![w.stage_cache, c.word(0), out, out + 4]
    })
    .run()
    .assert_same();
}

#[test]
fn apply_stage_states() {
    Diff::new(entry!(
        0x00e8_9250,
        state::apply_stage_states(Ptr<_>, u32) -> u32
    ))
    .common()
    .inputs(&[Input::Of(&[0, 3]), Input::Of(MASKS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        stage_banks(e, w.stage_cache, c.word(1));
        vec![w.stage_cache, c.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn apply_sampler_states() {
    Diff::new(entry!(
        0x00e8_92d0,
        state::apply_sampler_states(Ptr<_>, u32) -> u32
    ))
    .common()
    .inputs(&[Input::Of(&[0, 3]), Input::Of(MASKS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        stage_banks(e, w.stage_cache, c.word(1));
        vec![w.stage_cache, c.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn apply_states() {
    Diff::new(entry!(0x00e8_9410, state::apply_states(Ptr<_>, u32) -> u32))
        .common()
        .inputs(&[Input::Of(&[0, 3, 4, 7]), Input::Of(MASKS)])
        .translation_calls_plainly(&[0x00e8_9250, 0x00e8_92d0])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            stage_banks(e, w.stage_cache, c.word(1));
            vec![w.stage_cache, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn address_modes() {
    Diff::new(entry!(0x00e7_de90, state::set_address_modes(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 3, 7]), Input::Of(MASKS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            stage_banks(e, w.stage_cache, c.word(1));
            vec![w.stage, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn filter_mode() {
    Diff::new(entry!(0x00e7_def0, state::set_filter_mode(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 3, 7]), Input::Of(MASKS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            stage_banks(e, w.stage_cache, c.word(1));
            vec![w.stage, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn has_streams() {
    Diff::new(entry!(0x00e8_eeb0, state::has_streams(Ptr<_>) -> bool))
        .common()
        .inputs(&[Input::Of(&[0, 1, 3]), Input::Of(&[0, 1, 2, 3])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            let data = e.mem.alloc(0x2c);
            let streams = e.mem.alloc(0xc);
            for i in 0..3 {
                let s = e.mem.alloc(0xc);
                set(e, s + 8, u32::from(i != c.word(1)));
                set(
                    e,
                    streams + 4 * i,
                    if i == 2 && c.word(1) == 3 { 0 } else { s },
                );
            }
            set(e, data + 0x20, c.word(0));
            set(e, data + 0x28, streams);
            vec![data]
        })
        .run()
        .assert_same();
}

#[test]
fn add_object_to_group() {
    Diff::new(entry!(
        0x00e8_8dc0,
        state::add_object_to_group(u32, u32, Ptr<_>, Ptr<_>, u8, u32, u32) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(TWO_WAYS),
        Input::Of(BYTES),
        Input::Of(&[0, 1]),
        Input::Of(&[0, 9]),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let group = e.mem.u32(w.renderer + 0x8b0);
        set(e, w.data + 0x34, c.word(2));
        let parts = e.mem.u32(e.mem.u32(w.skin + 0xc) + 0xc);
        set(e, parts + 0x28, c.word(3));
        let skin = if c.word(0) != 0 { w.skin } else { 0 };
        vec![0x55, group, w.data, skin, c.word(1), 0, 0]
    })
    .run()
    .assert_same();
}

#[test]
fn array_set_at() {
    Diff::new(entry!(0x00e8_6f40, state::array_set_at(Ptr<_>, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 3, 4, 6]),
            Input::Of(&[0, 1, 2]),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let array = w.cmap + 0xc;
            let element = e.mem.alloc(4);
            let base = e.mem.u32(array + 4);
            let value = match c.word(1) {
                0 => 0,
                1 => e.mem.u32(base),
                _ => e.mem.u32(base + 4),
            };
            set(e, element, value);
            set(e, base + 8, 0);
            let big = e.mem.alloc(0x20);
            for i in 0..4 {
                let v = e.mem.u32(base + 4 * i);
                set(e, big + 4 * i, v);
            }
            set(e, array + 4, big);
            set(e, 0x0126_f914, c.word(2));
            vec![array, c.word(0), element]
        })
        .run()
        .assert_same();
}

#[test]
fn sort_entries_by_enabled() {
    Diff::new(entry!(
        0x00e8_70c0,
        state::sort_entries_by_enabled(u32, Ptr<_>)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 0b0110, 0b1001, 0b1111]),
        Input::Of(&[1, 2, 4]),
    ])
    .translation_calls_plainly(&[0x00e8_6f40])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let array = w.cmap + 0x1c;
        let base = e.mem.u32(array + 4);
        for i in 0..4 {
            let entry = e.mem.u32(base + 4 * i);
            e.mem.set_u8(entry + 8, ((c.word(0) >> i) & 1) as u8);
        }
        e.mem.set_u16(array + 0xa, c.word(1) as u16);
        vec![w.cmap, array]
    })
    .run()
    .assert_same();
}

#[test]
fn sort_by_enabled() {
    Diff::new(entry!(0x00e8_7200, state::sort_by_enabled(Ptr<_>)))
        .common()
        .inputs(&[Input::Of(&[0, 0b0110, 0b1001])])
        .translation_calls_plainly(&[0x00e8_6f40, 0x00e8_70c0])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let base = e.mem.u32(w.cmap + 0x10);
            for i in 0..4 {
                let entry = e.mem.u32(base + 4 * i);
                e.mem.set_u8(entry + 8, ((c.word(0) >> i) & 1) as u8);
            }
            vec![w.cmap]
        })
        .run()
        .assert_same();
}

// The batch renderer.

/// Group 1's list 0 holds passes 0 and 1, its list 2 pass 2; group 0's list 0 the skinned pass.
fn group_list(e: &Engine, w: &W, group: u32, list: u32) -> u32 {
    e.mem.u32(w.br + 8) + group * 0x68 + list * 0x14
}

#[test]
fn first_list() {
    Diff::new(entry!(
        0x00b9_8ae0,
        batch::first_list(Ptr<_>, u32, u32) -> bool
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 2, 3])])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let out = e.mem.alloc(4);
        vec![w.br, c.word(0), out]
    })
    .run()
    .assert_same();
}

#[test]
fn pop_active() {
    Diff::new(entry!(0x00b9_9730, batch::pop_active(Ptr<_>)))
        .common()
        .inputs(&[Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            if c.word(0) != 0 {
                set(e, w.br + 0x30, 0);
            }
            vec![w.br + 0x2c]
        })
        .run()
        .assert_same();
}

#[test]
fn next_active() {
    Diff::new(entry!(
        0x00b9_9900,
        batch::next_active(Ptr<_>, u32, u32) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 0x10, 0x11, 0x40]),
        Input::Of(&[0x10, 0x11, 0x20]),
    ])
    .translation_calls_plainly(&[0x00b9_8ae0, 0x00b9_9730])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        e.mem.set_u16(w.br + 0x2c, c.word(0) as u16);
        set(e, w.br + 0x20, c.word(1));
        let out = e.mem.alloc(8);
        vec![w.br, out, out + 4]
    })
    .run()
    .assert_same();
}

#[test]
fn next_group() {
    Diff::new(entry!(
        0x00b9_a090,
        batch::next_group(Ptr<_>, u32, u32, u32) -> bool
    ))
    .common()
    .inputs(&[Input::Of(&[0, 0x10, 0x11, 0x13]), Input::Of(&[0, 0x10])])
    .translation_calls_plainly(&[0x00b9_8ae0, 0x00b9_9730, 0x00b9_9900])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        e.mem.set_u16(w.br + 0x2c, c.word(1) as u16);
        let out = e.mem.alloc(0xc);
        set(e, out, c.word(0));
        vec![w.br, out, out + 4, out + 8]
    })
    .run()
    .assert_same();
}

#[test]
fn render_group() {
    Diff::new(entry!(
        0x00b9_a120,
        batch::render_group(Ptr<_>, u32, u32, u32, u32) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(&[0x11, 0x10]),
        Input::Of(&[0, 1, 2, 3, 4]),
        Input::Of(BYTES),
        Input::Of(&[0, 1]),
        Input::Of(&[0, 0x10]),
    ])
    .translation_calls_plainly(&[
        0x00b9_84d0,
        0x00b9_8ae0,
        0x00b9_8e80,
        0x00b9_9110,
        0x00b9_9390,
        0x00b9_94f0,
        0x00b9_9730,
        0x00b9_9900,
        0x00b9_a090,
    ])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let out = e.mem.alloc(0xc);
        set(e, out, c.word(0));
        set(e, out + 4, c.word(1));
        // Every list of groups 0 and 1 holds passes.
        for group in 0..2 {
            for list in 0..5 {
                let at = group_list(e, &w, group, list);
                list_at(e, at, &[w.passes[0], w.passes[1]]);
            }
        }
        e.mem.set_u8(0x011f_9421, c.word(3) as u8);
        e.mem.set_u16(w.br + 0x2c, c.word(4) as u16);
        set(e, 0x011f_fa24, 1);
        vec![w.br, out, out + 4, out + 8, c.word(2)]
    })
    .run()
    .assert_same();
}

/// The current pass type and shader the renderer has begun, from the case.
fn begun(e: &mut Engine, w: &W, which: u32) {
    let (pass_type, shader) = match which {
        0 => (0x10, w.shader),
        1 => (0x10, 0),
        _ => (0x11, w.shader),
    };
    set(e, 0x011f_fe30, pass_type);
    set(e, 0x011f_fe2c, shader);
}

#[test]
fn render_pass() {
    Diff::new(entry!(
        0x00b9_94f0,
        batch::render_pass(Ptr<_>, u32, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 2, 3, 4]),
        Input::Of(&[0, 1, 2]),
        Input::Of(BYTES),
        Input::Of(&[0, 1, 3]),
        Input::Of(&[0, 1, 0x11]),
    ])
    .translation_calls_plainly(&[0x00b9_8e80, 0x00b9_9110, 0x00b9_9390])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        begun(e, &w, c.word(1));
        e.mem.set_u8(0x011f_9421, (c.word(3) & 1) as u8);
        set(e, RETS + 0x30, c.word(3) >> 1);
        set(e, w.shader + 0x68, c.word(4));
        let pass = if c.word(0) == 4 {
            w.skin_pass
        } else {
            w.passes[c.word(0) as usize]
        };
        let pass_type = u32::from(e.mem.u16(pass + 4));
        vec![pass, pass_type, c.word(2), 1, c.word(2) ^ 1]
    })
    .run()
    .assert_same();
}

#[test]
fn render_standard() {
    Diff::new(entry!(
        0x00b9_8e80,
        batch::render_standard(Ptr<_>, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0x10, 0x11, 0xca, 0xd1, 0xfc, 0x102, 4, 0x23a]),
        Input::Of(BYTES),
        Input::Of(BYTES),
        Input::Of(BYTES),
        Input::Of(TWO_WAYS),
        Input::Of(TWO_WAYS),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let pass = w.passes[0];
        e.mem.set_u16(pass + 4, c.word(0) as u16);
        e.mem.set_u8(pass + 7, c.word(4) as u8);
        e.mem.set_u8(0x011a_d8ec, c.word(5) as u8);
        vec![pass, c.word(1), c.word(2), c.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn render_skinned() {
    Diff::new(entry!(
        0x00b9_9110,
        batch::render_skinned(Ptr<_>, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0x10, 0x11, 4]),
        Input::Of(BYTES),
        Input::Of(BYTES),
        Input::Of(BYTES),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 1, 0x011f_49d8]),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let pass = w.skin_pass;
        e.mem.set_u16(pass + 4, c.word(0) as u16);
        e.mem.set_u8(0x011a_d8ec, (c.word(4) & 1) as u8);
        e.mem.set_u8(pass + 7, (c.word(4) >> 1) as u8);
        set(e, RETS + 8, c.word(5));
        set(e, RETS + 0x6c, c.word(5) & 1);
        vec![pass, c.word(1), c.word(2), c.word(3)]
    })
    .run()
    .assert_same();
}

#[test]
fn begin_pass() {
    Diff::new(entry!(0x00b9_9390, batch::begin_pass(u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0x10, 0xca, 0xd1, 0xfc, 0xfd, 0x102, 0xfe]),
            Input::Of(&[0, 1, 2]),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            begun(e, &w, c.word(1));
            e.mem.set_u8(0x011a_d8ec, c.word(2) as u8);
            vec![c.word(0), w.shader]
        })
        .run()
        .assert_same();
}

#[test]
fn end_pass() {
    Diff::new(entry!(0x00b9_88e0, batch::end_pass()))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x10, 0xfe, 0x10b, 0x230]),
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, 0x011f_fe30, c.word(0));
            set(e, 0x011f_fe2c, if c.word(1) != 0 { w.shader } else { 0 });
            e.mem.set_u8(0x011a_d8ec, c.word(2) as u8);
            vec![]
        })
        .run()
        .assert_same();
}

#[test]
fn compare_by_depth() {
    Diff::new(entry!(0x00b9_8b20, batch::compare_by_depth(u32, u32) -> i32))
        .common()
        .inputs(&[
            Input::F32,
            Input::F32,
            Input::Of(TWO_WAYS),
            Input::F32,
            Input::F32,
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(256)
        .setup(|e, c| {
            let w = world(e);
            // Two geometries' bounds: the first's x and radius, the second's z and radius from the case.
            let mut items = [0u32; 2];
            for (k, item) in items.iter_mut().enumerate() {
                let bound = e.mem.alloc(0x10);
                let (a, r) = if k == 0 {
                    (bits(c, 0), bits(c, 1))
                } else {
                    (bits(c, 3), bits(c, 4))
                };
                for (i, v) in [a, TWO, a ^ 0x0080_0000, r].iter().enumerate() {
                    set(e, bound + 4 * i as u32, *v);
                }
                let geometry = e.mem.alloc(0xc4);
                set(
                    e,
                    geometry + 0x20,
                    if c.word(5) != 0 && k == 1 { 0 } else { bound },
                );
                let pass = e.mem.alloc(8);
                set(e, pass, geometry);
                *item = e.mem.alloc(4);
                set(e, *item, pass);
            }
            e.mem.set_u8(0x011f_fe35, c.word(2) as u8);
            set(e, 0x011f_fe38, w.camera);
            vec![items[0], items[1]]
        })
        .run()
        .assert_same();
}

#[test]
fn sort_alphas() {
    Diff::new(entry!(0x00b9_9a10, batch::sort_alphas(Ptr<_>, u32, u8)))
        .common()
        .inputs(&[Input::Of(BYTES), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[0x00b9_9630])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            // Each alpha holder's list: four passes with bounds at other depths.
            for k in 0..2u32 {
                let holder = e.mem.u32(w.br + 0x70 + 4 * k);
                let mut items = vec![];
                for i in 0..4u32 {
                    let bound = e.mem.alloc(0x10);
                    set(e, bound, 0x3f80_0000 + (((i * 3 + k) % 4) << 20));
                    set(e, bound + 0xc, if c.word(1) != 0 { HALF } else { 0 });
                    let geometry = e.mem.alloc(0xc4);
                    set(e, geometry + 0x20, bound);
                    let pass = e.mem.alloc(8);
                    set(e, pass, geometry);
                    items.push(pass);
                }
                list_at(e, holder + 0xc, &items);
            }
            vec![w.br, w.camera, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn sort_list() {
    Diff::new(entry!(0x00b9_9630, batch::sort_list(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2, 3, 5])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            // A list of passes keyed by their geometry word: the stand-in comparison sorts by item value.
            let list = e.mem.alloc(0x14);
            let items: Vec<u32> = (0..c.word(0))
                .map(|i| [0x30, 0x10, 0x20, 0x10, 0x40][i as usize])
                .collect();
            list_at(e, list, &items);
            let _ = w;
            vec![list, 0x00b9_88a0]
        })
        .run()
        .assert_same();
}

#[test]
fn list_append() {
    Diff::new(entry!(0x00b9_9840, batch::list_append(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(TWO_WAYS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let list = e.mem.alloc(0x14);
            list_at(
                e,
                list,
                if c.word(0) != 0 {
                    &w.passes[..2]
                } else {
                    &w.passes[..0]
                },
            );
            if c.word(1) != 0 {
                set(e, list + 0xc, 0);
            }
            let item = e.mem.alloc(4);
            set(e, item, w.passes[3]);
            vec![list, item]
        })
        .run()
        .assert_same();
}

#[test]
fn list_prepend() {
    Diff::new(entry!(0x00b9_9890, batch::list_prepend(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(TWO_WAYS), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let list = e.mem.alloc(0x14);
            list_at(
                e,
                list,
                if c.word(0) != 0 {
                    &w.passes[..2]
                } else {
                    &w.passes[..0]
                },
            );
            if c.word(1) != 0 {
                set(e, list + 0xc, 0);
            }
            let item = e.mem.alloc(4);
            set(e, item, w.passes[3]);
            vec![list, item]
        })
        .run()
        .assert_same();
}

#[test]
fn add_pass() {
    Diff::new(entry!(0x00b9_abc0, batch::add_pass(Ptr<_>, Ptr<_>, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 1, 2]),
            Input::Of(&[0, 1, 2, 3]),
            Input::Of(BYTES),
        ])
        .translation_calls_plainly(&[
            0x00b9_9840,
            0x00b9_9a50,
            0x00b9_9770,
            0x00b9_9a90,
            0x00b9_9b00,
            0x00b9_8e20,
        ])
        .float_sweep(0)
        .setup(|_e, c| {
            let w = world(_e);
            vec![
                w.holders[c.word(0) as usize],
                w.passes[c.word(1) as usize],
                c.word(2),
            ]
        })
        .run()
        .assert_same();
}

#[test]
fn register_pass() {
    Diff::new(entry!(
        0x00b9_b3b0,
        batch::register_pass(Ptr<_>, Ptr<_>, u32, u32) -> u32
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 0xc]),
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(BYTES),
    ])
    .translation_calls_plainly(&[
        0x00b9_9840,
        0x00b9_9a50,
        0x00b9_9770,
        0x00b9_9a90,
        0x00b9_9b00,
        0x00b9_8e20,
    ])
    .float_sweep(0)
    .setup(|_e, c| {
        let w = world(_e);
        vec![w.br, w.passes[c.word(1) as usize], c.word(0), c.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn register_alpha() {
    Diff::new(entry!(
        0x00b9_b3d0,
        batch::register_alpha(Ptr<_>, Ptr<_>, u32) -> u32
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1]),
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 1, u32::MAX]),
    ])
    .translation_calls_plainly(&[
        0x00b9_9840,
        0x00b9_9a50,
        0x00b9_9770,
        0x00b9_9a90,
        0x00b9_9b00,
        0x00b9_8e20,
    ])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.br + 0x34, c.word(2));
        vec![w.br, w.passes[c.word(1) as usize], c.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn clear_holder() {
    Diff::new(entry!(0x00b9_a490, batch::clear_holder(Ptr<_>)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2]), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[0x00b9_9bc0])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let holder = w.holders[c.word(0) as usize];
            if c.word(1) != 0 {
                set(e, holder + 8, 0);
            }
            vec![holder]
        })
        .run()
        .assert_same();
}

#[test]
fn clear_group() {
    Diff::new(entry!(0x00b9_9a50, batch::clear_group(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2]), Input::Word])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            vec![e.mem.u32(w.br + 8) + 0x68 * c.word(0), c.word(1)]
        })
        .run()
        .assert_same();
}

#[test]
fn clear_renderer() {
    Diff::new(entry!(0x00b9_9bc0, batch::clear_renderer(Ptr<_>, u32)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 4]), Input::Of(&[0, 1, 3, 5])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.br + 0x10, c.word(0));
            let _ = c.word(1);
            vec![w.br, 0]
        })
        .run()
        .assert_same();
}

#[test]
fn release_spare() {
    Diff::new(entry!(0x00b6_3e90, batch::release_spare(Ptr<_>) -> u32))
        .common()
        .inputs(&[Input::Of(&[0, 1, 3]), Input::Of(TWO_WAYS)])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            let list = w.list;
            let mut spare = 0;
            for _ in 0..c.word(0) {
                let node = e.mem.alloc(0xc);
                set(e, node, spare);
                spare = node;
            }
            set(e, list + 0xc, spare);
            if c.word(1) != 0 {
                set(e, list + 8, 0);
            }
            set(e, 0x011c_5f84, 1);
            vec![list]
        })
        .run()
        .assert_same();
}

#[test]
fn add_to_group() {
    Diff::new(entry!(
        0x00b9_9c90,
        batch::add_to_group(Ptr<_>, Ptr<_>, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 2, 3, 4]),
        Input::Of(&[0, 2, 4, 0x10, 0x238, 0x300]),
        Input::F32,
        Input::Of(&[0, 0x0040_0000]),
        Input::Of(&[0, 1, 2]),
    ])
    .translation_calls_plainly(&[
        0x00b9_8e20,
        0x00b9_9770,
        0x00b9_9a50,
        0x00b9_9a90,
        0x00b9_9b00,
    ])
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, c| {
        let w = world(e);
        let pass = match c.word(0) {
            3 => 0,
            4 => {
                let p = e.mem.alloc(8);
                e.mem.set_u16(p + 4, 0x10);
                p
            }
            i => w.passes[i as usize],
        };
        set(e, w.property + 0x28, bits(c, 2));
        set(e, w.property + 0x24, c.word(3));
        // The alpha property: blends without bit 0, or none.
        match c.word(4) {
            1 => e.mem.set_u16(w.alpha + 0x18, 0x200),
            2 => set(e, w.geometry + 0x9c, 0),
            _ => {}
        }
        // A group of type 0x10 with nothing counted, so the active list takes it.
        let groups = e.mem.u32(w.br + 8);
        set(e, groups + 0x64, 0);
        vec![w.br, pass, c.word(1)]
    })
    .run()
    .assert_same();
}

#[test]
fn insert_active() {
    Diff::new(entry!(0x00b9_9770, batch::insert_active(Ptr<_>, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x8, 0x10, 0x11, 0x12, 0x10010]),
            Input::Of(TWO_WAYS),
        ])
        .translation_calls_plainly(&[0x00b9_8e20])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            if c.word(1) != 0 {
                e.mem.set_u16(w.br + 0x2c, 0);
                set(e, w.br + 0x30, 0);
            }
            vec![w.br + 0x2c, c.word(0), 0x00b9_88a0]
        })
        .run()
        .assert_same();
}

#[test]
fn group_prepend() {
    Diff::new(entry!(
        0x00b9_9a90,
        batch::group_prepend(Ptr<_>, u32, Ptr<_>)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 2, 4]), Input::Of(TWO_WAYS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let group = e.mem.u32(w.br + 8) + 0x68;
        if c.word(1) != 0 {
            set(e, group + 0xc, 0);
        }
        vec![group, c.word(0), w.passes[3]]
    })
    .run()
    .assert_same();
}

#[test]
fn group_append() {
    Diff::new(entry!(
        0x00b9_9b00,
        batch::group_append(Ptr<_>, u32, Ptr<_>)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 2, 4]), Input::Of(TWO_WAYS)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let group = e.mem.u32(w.br + 8) + 0x68;
        if c.word(1) != 0 {
            set(e, group + 0xc, 0);
        }
        vec![group, c.word(0), w.passes[3]]
    })
    .run()
    .assert_same();
}

// The accumulator and the shaders' set-ups.

#[test]
fn process_commands() {
    Diff::new(entry!(
        0x00b6_4020,
        acc::process_commands(Ptr<_>, Ptr<_>, u32, Ptr<_>, u32) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 3]),
        Input::Of(&[0, 1]),
        Input::Of(&[0, 1, 4, 14]),
        Input::F32,
    ])
    .translation_calls_plainly(&[
        0x00b9_9840,
        0x00b9_9a50,
        0x00b9_9770,
        0x00b9_9a90,
        0x00b9_9b00,
        0x00b9_8e20,
    ])
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, c| {
        let w = world(e);
        let a = w.acc;
        e.mem.set_u8(a + 0x164, (c.word(0) & 1) as u8);
        set(e, w.shader + 0x68, if c.word(0) & 2 != 0 { 0xd } else { 0 });
        e.mem.set_u8(a + 0x3a, (c.word(1) & 1) as u8);
        e.mem.set_u8(a + 0x85, (c.word(1) >> 1) as u8);
        set(
            e,
            RETS + 0xac,
            if c.word(2) & 1 != 0 { w.passes[2] } else { 0 },
        );
        e.mem.set_u8(a + 0x84, (c.word(2) >> 1) as u8);
        set(e, w.br + 0x34, c.word(3));
        e.mem.set_u8(a + 0x16c, c.word(3) as u8);
        e.mem.set_u8(w.passes[2] + 6, c.word(4) as u8);
        e.mem.set_u8(a + 0x166, 1);
        set(e, a + 0x168, 2);
        let bound = e.mem.u32(w.geometry + 0x20);
        set(e, bound + 8, bits(c, 5));
        vec![a, w.geometry, w.geometry + 0x9c, w.property, w.shader]
    })
    .run()
    .assert_same();
}

#[test]
fn register_object() {
    Diff::new(entry!(
        0x00b6_3f10,
        acc::register_object(Ptr<_>, Ptr<_>) -> bool
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 2]),
        Input::Of(&[0, 1, 2]),
        Input::Of(TWO_WAYS),
    ])
    .translation_calls_plainly(&[
        0x00b9_9840,
        0x00b9_9a50,
        0x00b9_9770,
        0x00b9_9a90,
        0x00b9_9b00,
        0x00b9_8e20,
    ])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let geometry = if c.word(0) == 1 {
            w.skinned
        } else {
            w.geometry
        };
        if c.word(0) == 2 {
            set(e, w.geometry + 0xc0, 0);
        }
        set(e, RETS + 8, if c.word(2) != 0 { 0x011f_49d8 } else { 0 });
        e.mem.set_u8(w.skin + 0x3c, (c.word(1) & 1) as u8);
        set(e, w.acc + 0x19c, c.word(1));
        set(e, 0x011f_9f84, 0x00b6_4020);
        vec![w.acc, geometry]
    })
    .run()
    .assert_same();
}

#[test]
fn finish() {
    Diff::new(entry!(0x00b6_5e80, acc::finish(Ptr<_>)))
        .common()
        .inputs(&[Input::Of(TWO_WAYS), Input::Of(&[0, 1, 2])])
        .translation_calls_plainly(&[0x00b6_4f90, 0x00b9_8e80, 0x00b9_9110, 0x00b9_9390])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.acc + 8, c.word(0));
            begun(e, &w, 0);
            set(e, w.acc + 0x1c, c.word(1));
            vec![w.acc]
        })
        .run()
        .assert_same();
}

#[test]
fn draw_sorted() {
    Diff::new(entry!(0x00b6_4f90, acc::draw_sorted(Ptr<_>)))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2]), Input::Of(&[0, 1, 2])])
        .translation_calls_plainly(&[0x00b9_8e80, 0x00b9_9110, 0x00b9_9390])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            begun(e, &w, 0);
            set(e, w.acc + 0x1c, c.word(0));
            // The geometry list: inline (+0xc), or one at +0x18 with two or no nodes.
            if c.word(1) != 0 {
                let list = e.mem.alloc(0xc);
                if c.word(1) == 2 {
                    let at = e.mem.alloc(0x14);
                    list_at(e, at, &[w.geometry, w.skinned]);
                    for k in 0..2 {
                        let v = e.mem.u32(at + 4 + 4 * k);
                        set(e, list + 4 * k, v);
                    }
                    set(e, list + 8, 2);
                }
                set(e, w.acc + 0x18, list);
            }
            vec![w.acc]
        })
        .run()
        .assert_same();
}

#[test]
fn blend_of() {
    Diff::new(entry!(0x00be_1ff0, acc::set_blend_of(u32, u32)))
        .common()
        .inputs(&[
            Input::F32,
            Input::F32,
            Input::Of(&[0, 1, 2]),
            Input::Of(&[0, 1, 2]),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.property + 0x28, bits(c, 0));
            set(e, w.property + 0x2c, bits(c, 1));
            let block = e.mem.alloc(0x14);
            let alpha = match c.word(2) {
                0 => 0,
                1 => w.alpha,
                _ => {
                    let a = e.mem.alloc(0x1c);
                    e.mem.set_u16(a + 0x18, 0x200);
                    a
                }
            };
            set(e, block, alpha);
            match c.word(3) {
                0 => set(e, block + 0xc, w.property),
                1 => {
                    set(e, block + 0xc, w.property);
                    set(e, w.property + 0x1c, u32::MAX);
                }
                _ => set(e, block + 0xc, 0),
            }
            e.map(0, 0x1000);
            vec![0x55, block]
        })
        .run()
        .assert_same();
}

#[test]
fn make_matrix() {
    Diff::new(entry!(
        0x00b7_1a40,
        acc::make_matrix(u32, Ptr<_>, Ptr<_>, f32)
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::F32,
        Input::F32Swept(ONE),
        Input::F32Swept(HALF),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, c| {
        let w = world(e);
        let out = e.mem.alloc(0x40);
        let rotation = e.mem.alloc(0x24);
        for i in 0..9u32 {
            let v = if i % 4 == 0 {
                bits(c, 0)
            } else if i == 5 {
                bits(c, 4)
            } else {
                bits(c, 1)
            };
            set(e, rotation + 4 * i, v ^ (i << 3));
        }
        let point = e.mem.alloc(0xc);
        set(e, point, bits(c, 2));
        set(e, point + 4, bits(c, 3));
        set(e, point + 8, ONE);
        let _ = w;
        vec![out, rotation, point, bits(c, 3)]
    })
    .run()
    .assert_same();
}

#[test]
fn world_transform() {
    Diff::new(entry!(
        0x00e6_fbb0,
        acc::set_world_transform(Ptr<_>, Ptr<_>, u8)
    ))
    .common()
    .inputs(&[
        Input::F32,
        Input::F32,
        Input::Of(BYTES),
        Input::F32Swept(HALF),
        Input::F32Swept(ONE),
    ])
    .translation_calls_plainly(&[])
    .float_sweep(FLOAT_SWEEP)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.transform, bits(c, 0));
        set(e, w.transform + 0x10, bits(c, 3));
        set(e, w.transform + 0x20, bits(c, 4));
        set(e, w.transform + 0x30, bits(c, 1));
        set(e, w.renderer + 0x638, bits(c, 3));
        set(e, w.renderer + 0x644, bits(c, 4));
        vec![w.renderer, w.transform, c.word(2)]
    })
    .run()
    .assert_same();
}

#[test]
fn world_view_constants() {
    Diff::new(entry!(
        0x00b7_c750,
        acc::set_world_view_constants(u32, u32, u32)
    ))
    .common()
    .inputs(&[Input::F32, Input::Word])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let this = e.mem.alloc(0x20);
        set(e, this + 0x18, w.renderer);
        set(e, w.transform + 0x30, bits(c, 0));
        vec![this, c.word(1), w.transform]
    })
    .run()
    .assert_same();
}

#[test]
fn light_direction() {
    Diff::new(entry!(0x00b7_c7b0, acc::set_light_direction(u32)))
        .common()
        .inputs(&[Input::F32, Input::F32, Input::F32Swept(HALF)])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            let vector = e
                .mem
                .u32(e.mem.u32(e.mem.u32(e.mem.u32(0x011f_91e0) + 0xc)) + 0xf8);
            set(e, vector + 0xf0, bits(c, 0));
            set(e, vector + 0xf4, bits(c, 1));
            set(e, vector + 0xf8, bits(c, 2));
            vec![w.transform]
        })
        .run()
        .assert_same();
}

#[test]
fn texture_by_index() {
    Diff::new(entry!(
        0x00c0_3230,
        acc::texture_by_index(Ptr<_>, u32) -> u32
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 2]), Input::Of(&[0, 0x80])])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(e, w.property + 0x20, c.word(1));
        vec![w.property, c.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn light_color() {
    Diff::new(entry!(0x00c0_3010, acc::accumulate_light_color(Ptr<_>)))
        .common()
        .inputs(&[
            Input::F32,
            Input::F32,
            Input::Of(TWO_WAYS),
            Input::Of(TWO_WAYS),
            Input::F32Swept(ONE),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.light + 0x28, bits(c, 0));
            set(e, w.light + 0x40, bits(c, 1));
            set(e, w.light + 0x30, bits(c, 4));
            e.mem.set_u8(0x011f_941e, c.word(2) as u8);
            if c.word(3) != 0 {
                let color = e.mem.alloc(0xc);
                for i in 0..3 {
                    set(e, color + 4 * i, ONE);
                }
                set(e, w.light + 0x34, color);
            }
            vec![w.light]
        })
        .run()
        .assert_same();
}

#[test]
fn tint() {
    Diff::new(entry!(0x00c0_2d90, acc::set_tint(u32, Ptr<_>, u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0, 0x20]),
            Input::Of(&[0, 1, 0x100]),
            Input::Of(TWO_WAYS),
            Input::F32,
        ])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.property + 0x24, c.word(0));
            let source = if c.word(2) != 0 {
                let s = e.mem.alloc(0x3c);
                set(e, s + 0x38, bits(c, 3));
                s
            } else {
                set(e, 0x0101_8f5c, bits(c, 3));
                0
            };
            vec![0x55, w.geometry, c.word(1), source]
        })
        .run()
        .assert_same();
}

#[test]
fn check_create_pass_array() {
    Diff::new(entry!(
        0x00ba_95e0,
        acc::check_create_pass_array(Ptr<_>, i32)
    ))
    .common()
    .inputs(&[Input::Of(&[0, 1, 5, u32::MAX]), Input::Of(TWO_WAYS)])
    .translation_calls_plainly(&[0x00ba_94b0, 0x00ba_8b80, 0x00ba_8d30])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        set(
            e,
            w.property + 0x3c,
            if c.word(1) != 0 { 0 } else { 0x4444 },
        );
        vec![w.property, c.word(0)]
    })
    .run()
    .assert_same();
}

#[test]
fn set_alpha() {
    Diff::new(entry!(0x00ba_8ab0, acc::set_alpha(Ptr<_>, f32)))
        .common()
        .inputs(&[Input::F32, Input::F32])
        .translation_calls_plainly(&[])
        .float_sweep(FLOAT_SWEEP)
        .setup(|e, c| {
            let w = world(e);
            set(e, w.property + 0x28, bits(c, 0));
            set(e, w.property + 0x38, 0x99);
            vec![w.property, bits(c, 1)]
        })
        .run()
        .assert_same();
}

#[test]
fn light_at() {
    Diff::new(entry!(0x00b5_ac20, acc::light_at(Ptr<_>, u32) -> u32))
        .common()
        .inputs(&[Input::Of(&[0, 1, 2, 3, 7])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|_e, c| {
            let w = world(_e);
            vec![w.ssn, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn pair_textures() {
    Diff::new(entry!(0x00b7_cfb0, acc::set_pair_textures(u32, u32)))
        .common()
        .inputs(&[Input::Of(&[
            0x94, 0x1f3, 0xa1, 0x115, 0xe5, 0x161, 0x1f8, 0x1fc, 0x1f9, 0x22c, 0x230, 0x10,
        ])])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let _w = world(e);
            set(e, 0x011f_95e4, 0xe4);
            set(e, 0x011f_95e8, 0xe8);
            vec![0x55, c.word(0)]
        })
        .run()
        .assert_same();
}

#[test]
fn restore_states() {
    Diff::new(entry!(0x00b7_c320, acc::restore_states(u32, u32)))
        .common()
        .inputs(&[
            Input::Of(&[0x250, 0x251, 6, 8, 0x10]),
            Input::Of(&[0, 1, 2]),
            Input::Of(&[0, 0x40000]),
        ])
        .translation_calls_plainly(&[])
        .float_sweep(0)
        .setup(|e, c| {
            let w = world(e);
            set(e, 0x011f_91e4, c.word(0));
            let block = e.mem.alloc(0x14);
            set(
                e,
                block + 0x10,
                match c.word(1) {
                    0 => 0,
                    1 => w.alpha,
                    _ => w.skin,
                },
            );
            set(e, w.property + 0x20, c.word(2));
            set(e, block + 0xc, w.property);
            vec![0x55, block]
        })
        .run()
        .assert_same();
}

/// The alpha sort's depth less the radius in the exe's two steps: the sum stored as a float, then the
/// radius taken off. With the camera along (1, 1, 0) and the 53-bit word, a bound at (1, 2^-30, 0) of
/// radius 1 sums to 1 + 2^-30, stored as 1.0, so its depth is 0 like (1, 0, 0)'s: the two compare equal.
/// Without the store the depth would be 2^-30. Both orders; the 24-bit word rounds the sum anyway.
#[test]
fn compare_by_depth_stores_before_the_radius() {
    Diff::new(entry!(0x00b9_8b20, batch::compare_by_depth(u32, u32) -> i32))
        .common()
        .control_words(&[0x027f])
        .inputs(&[Input::Of(TWO_WAYS)])
        .setup(|e, c| {
            let w = world(e);
            set(e, w.camera + 0x68, ONE);
            set(e, w.camera + 0x74, ONE);
            set(e, w.camera + 0x80, 0);
            e.mem.set_u8(0x011f_fe35, 1);
            set(e, 0x011f_fe38, w.camera);
            let mut items = [0u32; 2];
            for (item, y) in items.iter_mut().zip([0x3080_0000, 0]) {
                let bound = e.mem.alloc(0x10);
                set(e, bound, ONE);
                set(e, bound + 4, y);
                set(e, bound + 0xc, ONE);
                let geometry = e.mem.alloc(0xc4);
                set(e, geometry + 0x20, bound);
                let pass = e.mem.alloc(8);
                set(e, pass, geometry);
                *item = e.mem.alloc(4);
                set(e, *item, pass);
            }
            if c.word(0) != 0 {
                items.swap(0, 1);
            }
            vec![items[0], items[1]]
        })
        .run()
        .assert_same();
}

/// `set_world_transform` with the render state cache's slot 0x30 holding the typed `00e879e0`: that call
/// made direct (the plain world has a stand-in there).
#[test]
fn world_transform_typed_slot() {
    Diff::new(entry!(
        0x00e6_fbb0,
        acc::set_world_transform(Ptr<_>, Ptr<_>, u8)
    ))
    .common()
    .inputs(&[Input::F32, Input::F32, Input::Of(BYTES)])
    .translation_calls_plainly(&[])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let vt = vtable(e, &[(0x68, 0x00e8_8780), (0x30, 0x00e8_79e0)]);
        set(e, w.cache, vt);
        set(e, w.transform, bits(c, 0));
        set(e, w.transform + 0x30, bits(c, 1));
        vec![w.renderer, w.transform, c.word(2)]
    })
    .run()
    .assert_same();
}

/// `render_pass` with the geometry's slots 0x30 and 0x34 holding the leaf `00acbb70`, as every geometry
/// class has them: both calls made direct (the plain world has stand-ins there).
#[test]
fn render_pass_typed_geometry_slots() {
    Diff::new(entry!(
        0x00b9_94f0,
        batch::render_pass(Ptr<_>, u32, u32, u32, u32)
    ))
    .common()
    .inputs(&[
        Input::Of(&[0, 1, 2, 3]),
        Input::Of(&[0, 1, 2]),
        Input::Of(BYTES),
    ])
    .translation_calls_plainly(&[0x00b9_8e80, 0x00b9_9110, 0x00b9_9390])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        begun(e, &w, c.word(1));
        let pass = w.passes[c.word(0) as usize];
        let geometry = e.mem.u32(pass);
        let vt = vtable(e, &[(0x30, 0x00ac_bb70), (0x34, 0x00ac_bb70)]);
        set(e, geometry, vt);
        let pass_type = u32::from(e.mem.u16(pass + 4));
        vec![pass, pass_type, c.word(2), 1, c.word(2) ^ 1]
    })
    .run()
    .assert_same();
}

/// The culling walk's `AppendVirtual` (`00c4f1d0`) with the accumulator's slot 0x98 holding the typed
/// `RegisterObject` (`00b63f10`), or a stand-in: the call made direct, the culling family on too.
#[test]
fn append_virtual_registers_through_the_typed_slot() {
    Diff::new(entry!(
        0x00c4_f1d0,
        culling::process::append_virtual(Ptr<_>, Ptr<_>)
    ))
    .common()
    .family(&culling::CULLING)
    .inputs(&[Input::Of(TWO_WAYS), Input::Of(&[0, 1, 2])])
    .translation_calls_plainly(&[
        0x00b9_9840,
        0x00b9_9a50,
        0x00b9_9770,
        0x00b9_9a90,
        0x00b9_9b00,
        0x00b9_8e20,
    ])
    .float_sweep(0)
    .setup(|e, c| {
        let w = world(e);
        let target = if c.word(0) != 0 {
            0x00b6_3f10
        } else {
            VT_FN + 0x30
        };
        let vt = vtable(e, &[(0x98, target)]);
        set(e, w.acc, vt);
        set(e, w.acc + 0x19c, c.word(1));
        let process = e.mem.alloc(0xc8);
        set(e, process + 0xc4, w.acc);
        vec![process, w.geometry]
    })
    .run()
    .assert_same();
}

//! `(unplaced)/run_00ec0280` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer holder: its `+0x8b8` word is the object the draw helpers call (`thiscall`).
const RENDERER_HOLDER_011F4748: u32 = 0x011f_4748;
/// The Bink movie manager (`0126fac4`) the movie functions call through its vtable.
const MOVIE_MANAGER_0126FAC4: u32 = 0x0126_fac4;
/// `"Data\\Video\\"`.
const VIDEO_FOLDER_010F1BA8: u32 = 0x010f_1ba8;
/// `"Could not open %s for playback."`.
const OPEN_FAILED_010F1B88: u32 = 0x010f_1b88;
/// `"Could not allocate textures for %s playback."`.
const TEXTURES_FAILED_010F1BB4: u32 = 0x010f_1bb4;
/// The word whose address the exe tests (always non-zero, so the size limit is the word at
/// `0126fad8`; the other branch clears and uses `01202800`).
const SIZE_LIMIT_PRESENT_0126FAD4: u32 = 0x0126_fad4;
const SIZE_LIMIT_0126FAD8: u32 = 0x0126_fad8;
const SIZE_LIMIT_FALLBACK_01202800: u32 = 0x0120_2800;

/// The renderer object (`[[011f4748] + 0x8b8]`).
fn renderer(e: &Engine) -> u32 {
    let holder = e.mem.u32(RENDERER_HOLDER_011F4748);
    e.mem.u32(holder.wrapping_add(0x8b8))
}

// Translated from 00ec0280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: `device`, `x`, `y`, `width`, `height`, `columns`, `rows`):
/// draws a screen quad as a four-vertex strip of six `float`s each (position `x`, `y`, 0, 1
/// and texture coordinates), the far corners `x + columns * width` and `y + rows * height`
/// (`FILD`, `FMUL`, `FADD`, stored as `float`s) with texture coordinate 1. Around it: the
/// renderer's vtable slots `0x8c(0, 1)` and `0x7c(0, 0)` before and `0x94` after; the device's
/// (`IDirect3DDevice9`) slot `0x160` saves the current shader word, slot `0x164` sets 0x104,
/// slot `0x14c` draws (`5, 2, vertices, 0x18`), slot `0x164` sets 0 and slot `0x15c` restores
/// the saved word.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ec0280(
    e: &mut Engine,
    device: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    columns: i32,
    rows: i32,
) {
    let x_bits = x.to_bits();
    let y_bits = y.to_bits();
    let x = x87_quiet_bits(x_bits);
    let y = x87_quiet_bits(y_bits);
    let one = 1.0f32.to_bits();
    let far_y = x87_store32(x87_add(
        x87_mul(f64::from(rows), x87_m32(height.to_bits())),
        x87_m32(y_bits),
    ));
    let far_x = x87_store32(x87_add(
        x87_mul(f64::from(columns), x87_m32(width.to_bits())),
        x87_m32(x_bits),
    ));
    let vertices = [
        [x, y, 0, one, 0, 0],
        [x, far_y, 0, one, 0, one],
        [far_x, y, 0, one, one, 0],
        [far_x, far_y, 0, one, one, one],
    ];
    e.with_stack(0x64, |e, frame| {
        let saved = frame.addr();
        let block = saved + 4;
        for (index, vertex) in vertices.iter().enumerate() {
            for (word, value) in vertex.iter().enumerate() {
                e.mem
                    .set_u32(block + 0x18 * index as u32 + 4 * word as u32, *value);
            }
        }
        let target = renderer(e);
        e.vcall(target, 0x8c, &args![0u32, 1u32]);
        let target = renderer(e);
        e.vcall(target, 0x7c, &args![0u32, 0u32]);
        e.mem.set_u32(saved, 0);
        e.vcall(device, 0x160, &args![saved]);
        e.vcall(device, 0x164, &args![0x104u32]);
        e.vcall(device, 0x14c, &args![5u32, 2u32, block, 0x18u32]);
        e.vcall(device, 0x164, &args![0u32]);
        let word = e.mem.u32(saved);
        e.vcall(device, 0x15c, &args![word]);
        let target = renderer(e);
        e.vcall(target, 0x94, &args![]);
    });
}

// Translated from 00ec0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, `device`): sets the movie quad's device state through
/// the `IDirect3DDevice9` vtable: slot `0xe4` (0x1b, 0); slot `0x10c` (0, 4, 2), (0, 2, 2),
/// (0, 5, 2), (0, 1, 4); slot `0x114` (0, 6, 2), (0, 5, 2), (0, 7, 2), (0, 1, 3), (0, 2, 3).
pub fn fn_00ec0460(e: &mut Engine, device: u32) {
    e.vcall(device, 0xe4, &args![0x1bu32, 0u32]);
    for (stage, state, value) in [(0u32, 4u32, 2u32), (0, 2, 2), (0, 5, 2), (0, 1, 4)] {
        e.vcall(device, 0x10c, &args![stage, state, value]);
    }
    for (sampler, state, value) in [
        (0u32, 6u32, 2u32),
        (0, 5, 2),
        (0, 7, 2),
        (0, 1, 3),
        (0, 2, 3),
    ] {
        e.vcall(device, 0x114, &args![sampler, state, value]);
    }
}

// Translated from 00ec0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec0ac0 - the /GS cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`RET 0x10`: `name`, `quiet`, `check_size`, `force_preload`; answer
/// in `AL`): opens a Bink movie. An empty name (`strlen`) gives 0. The path is the movie
/// manager's `00ec2df0(name, path)` or else `"Data\\Video\\"` + name (`strcpy`, `00ec6380`).
/// With `check_size` or `force_preload` and a file found by `__stat64i32`, a file no larger than
/// the limit at `0126fad8` (or `force_preload`) is opened with flag 0x2000; off the manager's
/// thread (`GetCurrentThreadId` against its vtable slot `0x3c`) flag 0x8000000 is added. Between
/// the manager's slots `0x14` and `0x18` the movie is opened (`BinkOpen`, import `00fdf3b8`)
/// into `+0`; a failure prints `"Could not open %s for playback."` unless `quiet`, clears `+0x20`
/// and answers 0. Off the manager's thread `00ec1a00` runs on it.
pub fn fn_00ec0ac0(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    quiet: u32,
    check_size: u32,
    force_preload: u32,
) -> u8 {
    if e.call(0x00ec_6130, &args![name]).u32() == 0 {
        return 0;
    }
    e.with_stack(0x150, |e, frame| {
        let path = frame.addr();
        let stat = path + 0x110;
        let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
        if !e.call(0x00ec_2df0, &args![manager, name, path]).bool() {
            e.call(0x00ec_6370, &args![path, VIDEO_FOLDER_010F1BA8]);
            e.call(0x00ec_6380, &args![path, name]);
        }
        let mut flags = 0u32;
        if (check_size as u8 != 0 || force_preload as u8 != 0)
            && e.call(0x00ec_923c, &args![path, stat]).i32() != -1
        {
            let limit = if SIZE_LIMIT_PRESENT_0126FAD4 != 0 {
                SIZE_LIMIT_0126FAD8
            } else {
                // Not reached: the tested address is a constant that is never 0.
                e.mem.set_u32(SIZE_LIMIT_FALLBACK_01202800, 0);
                SIZE_LIMIT_FALLBACK_01202800
            };
            let size = e.mem.i32(stat + 0x14);
            if size <= e.mem.i32(limit) || force_preload as u8 != 0 {
                flags |= 0x2000;
            }
        }
        let mut other_thread = false;
        let thread = e.call(0x00fd_f1d0, &args![]).u32();
        let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
        if thread != e.vcall(manager, 0x3c, &args![]).u32() {
            other_thread = true;
            flags |= 0x0800_0000;
        }
        let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
        e.vcall(manager, 0x14, &args![]);
        let movie = e.call(0x00fd_f3b8, &args![path, flags]).u32();
        e.mem.set_u32(this.addr(), movie);
        let mut opened = 1u8;
        if e.mem.u32(this.addr()) == 0 && e.mem.u32(this.addr()) == 0 {
            if quiet as u8 == 0 {
                e.call(0x005b_5e40, &args![OPEN_FAILED_010F1B88, name]);
            }
            e.mem.set_u32(this.addr().wrapping_add(0x20), 0);
            opened = 0;
        }
        if other_thread {
            let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
            e.call(0x00ec_1a00, &args![manager]);
        }
        let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
        e.vcall(manager, 0x18, &args![]);
        opened
    })
}

// Translated from 00ec0cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x10`: `name`, `quiet`, `check_size`, `flag`; answer in
/// `AL`): prepares a Bink movie for playback. Unless a movie is open (`+0`), it is opened
/// (`00ec0ac0(this, name, quiet, check_size, 0)`; a failure answers 0). The device (`+4`) is the
/// movie manager's vtable slot `0x1c`; with the movie's width and height (`[+0]`, `[+0]+4`) the
/// manager computes three `float`s (`00ec2aa0(width, height, flag)` into `+0x14`, then
/// `00ec2b20(+0x14, width, flag)` into `+0x18` and `00ec2bb0(+0x14, height, flag)` into `+0x1c`).
/// The textures (`00ebf8e0(device, width, height, 4, 0x100, 0x16)`) go to `+8`; without them
/// `"Could not allocate textures for %s playback."` is printed unless `quiet`, `+0x20` is
/// cleared and the answer is 0; otherwise 1.
pub fn fn_00ec0cd0(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    quiet: u32,
    check_size: u32,
    flag: u32,
) -> u8 {
    let base = this.addr();
    if e.mem.u32(base) == 0
        && fn_00ec0ac0(
            e,
            this,
            name,
            u32::from(quiet as u8),
            u32::from(check_size as u8),
            0,
        ) == 0
    {
        return 0;
    }
    let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
    let device = e.vcall(manager, 0x1c, &args![]).u32();
    e.mem.set_u32(base + 4, device);
    let flag = u32::from(flag as u8);
    let movie = e.mem.u32(base);
    let (width, height) = (e.mem.u32(movie), e.mem.u32(movie.wrapping_add(4)));
    let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
    let scale = e
        .call(0x00ec_2aa0, &args![manager, width, height, flag])
        .f64();
    e.mem.set_u32(base + 0x14, x87_store32(scale));
    let movie = e.mem.u32(base);
    let width = e.mem.u32(movie);
    let scale_bits = x87_quiet_bits(e.mem.u32(base + 0x14));
    let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
    let across = e
        .call(0x00ec_2b20, &args![manager, scale_bits, width, flag])
        .f64();
    e.mem.set_u32(base + 0x18, x87_store32(across));
    let movie = e.mem.u32(base);
    let height = e.mem.u32(movie.wrapping_add(4));
    let scale_bits = x87_quiet_bits(e.mem.u32(base + 0x14));
    let manager = e.mem.u32(MOVIE_MANAGER_0126FAC4);
    let down = e
        .call(0x00ec_2bb0, &args![manager, scale_bits, height, flag])
        .f64();
    e.mem.set_u32(base + 0x1c, x87_store32(down));
    let movie = e.mem.u32(base);
    let width = e.mem.u32(movie);
    let height = e.mem.u32(movie.wrapping_add(4));
    let device = e.mem.u32(base + 4);
    let textures = e
        .call(
            0x00eb_f8e0,
            &args![device, width, height, 4u32, 0x100u32, 0x16u32],
        )
        .u32();
    e.mem.set_u32(base + 8, textures);
    if e.mem.u32(base + 8) == 0 {
        if quiet as u8 == 0 {
            e.call(0x005b_5e40, &args![TEXTURES_FAILED_010F1BB4, name]);
        }
        e.mem.set_u32(base + 0x20, 0);
        return 0;
    }
    1
}

// Translated from 00ec0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a tiled movie frame, `x`, `y`, `width`, `height` as
/// float bit patterns): draws the frame's tiles with `00ec0720` on its device (+0). The steps
/// are `tile * width` and `tile * height` (the tile size +0x3c as a 64-bit `FILD`, x87,
/// stored as floats). For each of the +0x28 rows, each of the +0x24 columns draws a
/// `tile x tile` quad and then, with a right remainder (+0x2c), a `remainder x tile` one,
/// `x` and then `y` advancing by the steps (x87 adds of floats); with a bottom remainder
/// (+0x30) one more row draws `tile x bottom` quads and the `right x bottom` corner.
pub fn fn_00ec0550(e: &mut Engine, frame: Ptr, x: u32, y: u32, width: u32, height: u32) {
    let frame = frame.addr();
    let tile = u64::from(e.mem.u32(frame + 0x3c));
    let step_x = x87_store32(x87_mul(tile as f64, x87_m32(width)));
    let tile = u64::from(e.mem.u32(frame + 0x3c));
    let step_y = x87_store32(x87_mul(tile as f64, x87_m32(height)));
    let advance = |value: u32, step: u32| x87_store32(x87_add(x87_load32(value), x87_m32(step)));
    let quad = |e: &mut Engine, at_x: u32, at_y: u32, columns: u32, rows: u32| {
        let device = e.mem.u32(frame);
        fn_00ec0720(
            e,
            device,
            x87_quiet_bits(at_x),
            x87_quiet_bits(at_y),
            x87_quiet_bits(width),
            x87_quiet_bits(height),
            columns as i32,
            rows as i32,
        );
    };
    let mut row_y = x87_quiet_bits(y);
    let mut row = 0u32;
    while row < e.mem.u32(frame + 0x28) {
        let mut column_x = x87_quiet_bits(x);
        let mut column = 0u32;
        while column < e.mem.u32(frame + 0x24) {
            let tile = e.mem.u32(frame + 0x3c);
            quad(e, column_x, row_y, tile, tile);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        let right = e.mem.u32(frame + 0x2c);
        if right != 0 {
            let tile = e.mem.u32(frame + 0x3c);
            quad(e, column_x, row_y, right, tile);
        }
        row_y = advance(row_y, step_y);
        row += 1;
    }
    let bottom = e.mem.u32(frame + 0x30);
    if bottom != 0 {
        let mut column_x = x87_quiet_bits(x);
        let mut column = 0u32;
        while column < e.mem.u32(frame + 0x24) {
            let bottom = e.mem.u32(frame + 0x30);
            let tile = e.mem.u32(frame + 0x3c);
            quad(e, column_x, row_y, tile, bottom);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        let right = e.mem.u32(frame + 0x2c);
        if right != 0 {
            let bottom = e.mem.u32(frame + 0x30);
            quad(e, column_x, row_y, right, bottom);
        }
    }
}

// Translated from 00ec0720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `device`, `x`, `y`, `width`, `height` as float bit
/// patterns, `columns`, `rows`): draws one quad as a two-triangle fan of six-float vertices
/// (position, 0, 1, 0, 0), built in a local frame as the exe copies them: `(x, y)`,
/// `(x2, y)` twice, `(x2, y2)` twice, `(x, y2)` twice, with `x2 = columns * width + x` and
/// `y2 = rows * height + y` (`FILD`, x87, stored as floats), and five more copies after
/// them. The device (`IDirect3DDevice9` vtable) gets slot `0x10c` (0, 2, 0) and (0, 5, 0),
/// `0x170` (0), `0x164` (0x104) and `0x14c` (6, 2, the first vertex, 0x18).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ec0720(
    e: &mut Engine,
    device: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    columns: i32,
    rows: i32,
) {
    e.with_stack(0x120, |e, base| {
        // `base` is EBP-0x120; vertex `n` of the exe's chain sits at `base + 0x18 * n`.
        let base = base.addr();
        let vertex = |n: u32| base + 0x18 * n;
        let copy = |e: &mut Engine, to: u32, from: u32| {
            let bytes = e.mem.bytes(from, 0x18);
            e.mem.write(to, &bytes);
        };
        let first = vertex(0);
        e.mem.set_u32(first, x87_quiet_bits(x));
        e.mem.set_u32(first + 4, x87_quiet_bits(y));
        e.mem.set_f32(first + 8, 0.0);
        e.mem.set_f32(first + 0xc, 1.0);
        e.mem.set_f32(first + 0x10, 0.0);
        e.mem.set_f32(first + 0x14, 0.0);
        copy(e, vertex(1), vertex(0));
        let far_x = x87_store32(x87_add(
            x87_mul(f64::from(columns), x87_m32(width)),
            x87_m32(x),
        ));
        e.mem.set_u32(vertex(1), far_x);
        e.mem.set_u32(vertex(1) + 4, x87_quiet_bits(y));
        copy(e, vertex(2), vertex(1));
        copy(e, vertex(3), vertex(1));
        let far_y = x87_store32(x87_add(
            x87_mul(f64::from(rows), x87_m32(height)),
            x87_m32(y),
        ));
        e.mem.set_u32(vertex(3) + 4, far_y);
        copy(e, vertex(4), vertex(3));
        copy(e, vertex(5), vertex(3));
        e.mem.set_u32(vertex(5), x87_quiet_bits(x));
        copy(e, vertex(6), vertex(5));
        copy(e, vertex(7), vertex(0));
        copy(e, vertex(8), vertex(0));
        copy(e, vertex(9), vertex(3));
        copy(e, vertex(10), vertex(2));
        copy(e, vertex(11), vertex(6));
        e.vcall(device, 0x10c, &args![0u32, 2u32, 0u32]);
        e.vcall(device, 0x10c, &args![0u32, 5u32, 0u32]);
        e.vcall(device, 0x170, &args![0u32]);
        e.vcall(device, 0x164, &args![0x104u32]);
        e.vcall(device, 0x14c, &args![6u32, 2u32, first, 0x18u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_0280, fn_00ec0280(u32, f32, f32, f32, f32, i32, i32)),
        entry!(0x00ec_0460, fn_00ec0460(u32)),
        entry!(0x00ec_0ac0, fn_00ec0ac0(Ptr, u32, u32, u32, u32) -> u8),
        entry!(0x00ec_0cd0, fn_00ec0cd0(Ptr, u32, u32, u32, u32) -> u8),
        entry!(0x00ec_0550, fn_00ec0550(Ptr, u32, u32, u32, u32)),
        entry!(0x00ec_0720, fn_00ec0720(u32, u32, u32, u32, u32, i32, i32)),
    ]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SLOT_BASE: u32 = 0x7100_0000;

    /// An object whose vtable slot `n` calls `SLOT_BASE + n` (all doubled as no-ops).
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for slot in (0..0x200).step_by(4) {
            e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
            if !e.is_translated(SLOT_BASE + slot) {
                e.register(SLOT_BASE + slot, |_, _| Ret::default());
            }
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    fn holders(e: &mut Engine) -> (u32, u32) {
        e.map(RENDERER_HOLDER_011F4748 & !0xfff, 0x1000);
        e.map(MOVIE_MANAGER_0126FAC4 & !0xfff, 0x1000);
        let renderer = object(e);
        let holder = e.mem.alloc(0x900);
        e.mem.set_u32(holder + 0x8b8, renderer);
        e.mem.set_u32(RENDERER_HOLDER_011F4748, holder);
        let manager = object(e);
        e.mem.set_u32(MOVIE_MANAGER_0126FAC4, manager);
        (renderer, manager)
    }

    #[test]
    fn fn_00ec0280_draws_the_quad() {
        let mut e = Engine::new();
        let (renderer, _) = holders(&mut e);
        let device = object(&mut e);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(SLOT_BASE + 0x14c, move |e, a| {
            *record.borrow_mut() = (0..24).map(|word| e.mem.u32(a[3] + 4 * word)).collect();
            Ret::default()
        });
        e.register(SLOT_BASE + 0x160, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00ec_0280,
            &args![device, 10.0f32, 20.0f32, 4.0f32, 2.0f32, 3i32, 5i32],
        );
        let log = e.call_log.take().unwrap();
        let one = 1.0f32.to_bits();
        let (x, y, far_x, far_y) = (
            10.0f32.to_bits(),
            20.0f32.to_bits(),
            22.0f32.to_bits(),
            30.0f32.to_bits(),
        );
        assert_eq!(
            *seen.borrow(),
            vec![
                x, y, 0, one, 0, 0, x, far_y, 0, one, 0, one, far_x, y, 0, one, one, 0, far_x,
                far_y, 0, one, one, one
            ]
        );
        assert_eq!(calls(&log, SLOT_BASE + 0x8c), vec![vec![renderer, 0, 1]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x7c), vec![vec![renderer, 0, 0]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x94), vec![vec![renderer]]);
        assert_eq!(
            calls(&log, SLOT_BASE + 0x164),
            vec![vec![device, 0x104], vec![device, 0]]
        );
        assert_eq!(calls(&log, SLOT_BASE + 0x15c), vec![vec![device, 0x77]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x14c)[0][1..3], [5, 2]);
    }

    #[test]
    fn fn_00ec0460_sets_the_states() {
        let mut e = Engine::new();
        let device = object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00ec_0460, &args![device]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0xe4), vec![vec![device, 0x1b, 0]]);
        assert_eq!(
            calls(&log, SLOT_BASE + 0x10c),
            vec![
                vec![device, 0, 4, 2],
                vec![device, 0, 2, 2],
                vec![device, 0, 5, 2],
                vec![device, 0, 1, 4]
            ]
        );
        assert_eq!(calls(&log, SLOT_BASE + 0x114).len(), 5);
        assert_eq!(calls(&log, SLOT_BASE + 0x114)[4], vec![device, 0, 2, 3]);
    }

    fn movie_doubles(e: &mut Engine) {
        e.register(0x00ec_6130, |e, a| (e.mem.u8(a[0]) as u32).into_ret());
        e.register(0x00ec_2df0, |_, _| 0u32.into_ret());
        e.register(0x00ec_6370, |_, _| Ret::default());
        e.register(0x00ec_6380, |_, _| Ret::default());
        e.register(0x00ec_923c, |e, a| {
            e.mem.set_u32(a[1] + 0x14, 500);
            0u32.into_ret()
        });
        e.register(0x00fd_f1d0, |_, _| 7u32.into_ret());
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.register(0x00ec_1a00, |_, _| Ret::default());
    }

    #[test]
    fn fn_00ec0ac0_opens_the_movie_with_its_flags() {
        let mut e = Engine::new();
        let (_, manager) = holders(&mut e);
        movie_doubles(&mut e);
        e.map(SIZE_LIMIT_0126FAD8 & !0xfff, 0x1000);
        e.mem.set_u32(SIZE_LIMIT_0126FAD8, 1000);
        e.register(SLOT_BASE + 0x3c, |_, _| 9u32.into_ret());
        e.register(0x00fd_f3b8, |_, a| (a[1] | 0x1000_0000).into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 5);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"a.bik\0");
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00ec_0ac0, &args![this, name, 0u32, 1u32, 0u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 1);
        // Small enough (500 <= 1000) and off the manager's thread (7 != 9).
        assert_eq!(e.mem.u32(this), 0x1800_2000);
        assert_eq!(calls(&log, 0x00ec_1a00), vec![vec![manager]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x14), vec![vec![manager]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x18), vec![vec![manager]]);
        assert_eq!(calls(&log, 0x00ec_6370)[0][1], VIDEO_FOLDER_010F1BA8);
        // A failed open: the message unless quiet, +0x20 cleared, answer 0.
        e.register(0x00fd_f3b8, |_, _| 0u32.into_ret());
        e.register(SLOT_BASE + 0x3c, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00ec_0ac0, &args![this, name, 0u32, 0u32, 0u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(
            calls(&log, 0x005b_5e40),
            vec![vec![OPEN_FAILED_010F1B88, name]]
        );
        assert!(calls(&log, 0x00ec_1a00).is_empty());
        // An empty name answers 0 at once.
        let empty = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00ec_0ac0, &args![this, empty, 0u32, 0u32, 0u32])
                .u8(),
            0
        );
    }

    #[test]
    fn fn_00ec0cd0_prepares_the_textures() {
        let mut e = Engine::new();
        let (_, manager) = holders(&mut e);
        e.register(SLOT_BASE + 0x1c, |_, _| 0x900u32.into_ret());
        e.register(0x00ec_2aa0, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(0x00ec_2b20, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(0x00ec_2bb0, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(0x00eb_f8e0, |_, _| 0x5000u32.into_ret());
        let this = e.mem.alloc(0x24);
        let movie = e.mem.alloc(8);
        e.mem.set_u32(movie, 640);
        e.mem.set_u32(movie + 4, 480);
        e.mem.set_u32(this, movie);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00ec_0cd0, &args![this, 0x10u32, 0u32, 0u32, 0x101u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 1);
        assert_eq!(e.mem.u32(this + 4), 0x900);
        assert_eq!(calls(&log, 0x00ec_2aa0), vec![vec![manager, 640, 480, 1]]);
        assert_eq!(
            calls(&log, 0x00ec_2b20),
            vec![vec![manager, 0.5f32.to_bits(), 640, 1]]
        );
        assert_eq!(
            calls(&log, 0x00ec_2bb0),
            vec![vec![manager, 0.5f32.to_bits(), 480, 1]]
        );
        assert_eq!(
            (
                e.mem.u32(this + 0x14),
                e.mem.u32(this + 0x18),
                e.mem.u32(this + 0x1c)
            ),
            (0.5f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits())
        );
        assert_eq!(
            calls(&log, 0x00eb_f8e0),
            vec![vec![0x900, 640, 480, 4, 0x100, 0x16]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x5000);
        // No textures: the message, +0x20 cleared, 0.
        e.register(0x00eb_f8e0, |_, _| 0u32.into_ret());
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00ec_0cd0, &args![this, 0x10u32, 0u32, 0u32, 0u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 0);
        assert_eq!(
            calls(&log, 0x005b_5e40),
            vec![vec![TEXTURES_FAILED_010F1BB4, 0x10]]
        );
    }
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn device(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x180);
        for slot in [0x10cu32, 0x14c, 0x164, 0x170] {
            e.mem.set_u32(vtable + slot, 0x0079_0000 + slot);
        }
        e.register(0x0079_010c, |_, _| Ret::default());
        e.register(0x0079_0164, |_, _| Ret::default());
        e.register(0x0079_0170, |_, _| Ret::default());
        e.register(0x0079_014c, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        device
    }

    #[test]
    fn quad_builds_the_fan_and_sets_the_states() {
        let mut e = Engine::new();
        let d = device(&mut e);
        e.register_double(0x0079_014c, |e, a| {
            assert_eq!(&a[1..], &[6, 2, a[3], 0x18]);
            assert_eq!(e.mem.f32(a[3]), 10.0);
            assert_eq!(e.mem.f32(a[3] + 4), 20.0);
            assert_eq!(e.mem.f32(a[3] + 0xc), 1.0);
            assert_eq!(e.mem.f32(a[3] + 0x18), 10.0 + 3.0 * 2.0);
            assert_eq!(e.mem.f32(a[3] + 0x30), 16.0);
            assert_eq!(e.mem.f32(a[3] + 0x48), 16.0);
            assert_eq!(e.mem.f32(a[3] + 0x4c), 20.0 + 4.0 * 0.5);
            assert_eq!(e.mem.f32(a[3] + 0x78), 10.0);
            assert_eq!(e.mem.f32(a[3] + 0x7c), 22.0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00ec_0720,
            &args![
                d,
                10.0f32.to_bits(),
                20.0f32.to_bits(),
                2.0f32.to_bits(),
                0.5f32.to_bits(),
                3u32,
                4u32
            ],
        );
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0 - 0x0079_0000).collect();
        assert_eq!(order, vec![0x10c, 0x10c, 0x170, 0x164, 0x14c]);
        assert_eq!(log[2].1, vec![d, 0, 5, 0]);
        assert_eq!(log[4].1, vec![d, 0x104]);
    }

    #[test]
    fn tiles_cover_rows_columns_and_remainders() {
        let mut e = Engine::new();
        let d = device(&mut e);
        let frame = e.mem.alloc(0x40);
        e.mem.set_u32(frame, d);
        e.mem.set_u32(frame + 0x24, 2);
        e.mem.set_u32(frame + 0x28, 1);
        e.mem.set_u32(frame + 0x2c, 5);
        e.mem.set_u32(frame + 0x30, 7);
        e.mem.set_u32(frame + 0x3c, 64);
        let quads = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&quads);
        e.register_double(0x0079_014c, move |e, a| {
            let (x, y) = (e.mem.f32(a[3]), e.mem.f32(a[3] + 4));
            let (x2, y2) = (e.mem.f32(a[3] + 0x18), e.mem.f32(a[3] + 0x4c));
            seen.borrow_mut().push([x, y, x2 - x, y2 - y]);
            Ret::default()
        });
        e.call(
            0x00ec_0550,
            &args![frame, 0u32, 0u32, 1.0f32.to_bits(), 1.0f32.to_bits()],
        );
        // One row of 2 tiles plus the right remainder, then the bottom row of 2 plus the
        // corner.
        assert_eq!(
            *quads.borrow(),
            vec![
                [0.0, 0.0, 64.0, 64.0],
                [64.0, 0.0, 64.0, 64.0],
                [128.0, 0.0, 5.0, 64.0],
                [0.0, 64.0, 64.0, 7.0],
                [64.0, 64.0, 64.0, 7.0],
                [128.0, 64.0, 5.0, 7.0],
            ]
        );
    }
}

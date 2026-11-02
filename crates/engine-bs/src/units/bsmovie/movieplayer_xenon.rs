//! `BSMovie/movieplayer_xenon.obj` (Xbox PDB source unit), subsystem `BSMovie`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The movie player drives Bink through its imports (`BinkWait`,
//! `BinkNextFrame`, called by their import slots) and draws each frame with
//! Direct3D 9 device calls (COM vtable slots on the device). Fields of the
//! `Movie` object (read at their offsets, unnamed in the Xbox PDB here):
//! +0 the Bink handle (its frame count at +8), +4 the device, +8 the frame
//! textures, +0xC the frames shown, +0x10 the frames skipped, +0x14 the
//! scale, +0x18 and +0x1C the position, +0x20 the play state, +0x24 a busy
//! byte.

#[allow(unused_imports)]
use crate::prelude::*;

/// The movie settings object (`0126fac4`, a pointer) whose vtable the
/// player asks (+0x40 with one byte: go on; +0x30: no scene begin/end) and
/// notifies (+4/+8 and +0xC/+0x10 around a playback).
const MOVIE_SETTINGS: u32 = 0x0126_fac4;
/// The renderer pointer (`011f4748`): vtable +0xC8 (the default render
/// target group, then with 0 its first target), +0xD4 (the back buffer
/// description: width +8, height +0xC).
const RENDERER: u32 = 0x011f_4748;
/// The `NiRTTI` record the render target's class chain is checked against
/// (`0126f59c`).
const TARGET_RTTI: u32 = 0x0126_f59c;
/// The byte that turns on the letterbox viewport (`011f9426`), and the
/// viewport's left, right, bottom and top fractions (`011ad840`, `011ad844`,
/// `011ad848`, `011ad84c`).
const LETTERBOX: u32 = 0x011f_9426;
const VIEW_LEFT: u32 = 0x011a_d840;
const VIEW_RIGHT: u32 = 0x011a_d844;
const VIEW_BOTTOM: u32 = 0x011a_d848;
const VIEW_TOP: u32 = 0x011a_d84c;
/// Imports, by slot: `BinkWait` (`00fdf3ac`), `BinkNextFrame` (`00fdf3a8`),
/// `Sleep` (`00fdf104`), the tick count import (`00fdf060`),
/// `GetExitCodeThread` (`00fdf168`) and `CloseHandle` (`00fdf07c`).
const BINK_WAIT: u32 = 0x00fd_f3ac;
const BINK_NEXT_FRAME: u32 = 0x00fd_f3a8;
const SLEEP: u32 = 0x00fd_f104;
const TICK_COUNT: u32 = 0x00fd_f060;
const GET_EXIT_CODE_THREAD: u32 = 0x00fd_f168;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `STILL_ACTIVE`.
const STILL_ACTIVE: u32 = 0x103;
/// `0.0` and `1000.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const THOUSAND: u32 = 0x0101_7b70;
/// `Error` (`0040fbe0`, `cdecl`) and `"Bink playback: Skipped frame #%i.
/// Total skipped: %i"` (`010f1be4`).
const ERROR: u32 = 0x0040_fbe0;
const SKIPPED_FRAME_MESSAGE: u32 = 0x010f_1be4;
/// The movie's open (`00ec0cd0`, `thiscall`, four words), close
/// (`00ec1020`, `thiscall`), frame copy (`00ec14d0`, `cdecl`: Bink handle,
/// textures, 0), the draw state set-up (`00ec0460`, `cdecl`: device) and the
/// quad draw (`00ec0280`, `cdecl`: device, x, y, width, height, two sizes).
const MOVIE_OPEN: u32 = 0x00ec_0cd0;
const MOVIE_CLOSE: u32 = 0x00ec_1020;
const COPY_FRAME: u32 = 0x00ec_14d0;
const DRAW_SETUP: u32 = 0x00ec_0460;
const DRAW_QUAD: u32 = 0x00ec_0280;
/// The player's stop (`00ec25e0`, `MoviePlayer::Stop` by the decompiler's
/// name; `thiscall`) and the member clean-up before the map (`00ec2e50`).
const PLAYER_STOP: u32 = 0x00ec_25e0;
const PLAYER_CLEAN_UP: u32 = 0x00ec_2e50;
/// The vtables the player's destructor stores: its own (`010f1c44`) and the
/// name map's (`010f1c20`); the map destructor's (`010f1ce4`, `010f1cc4`,
/// `010f1ca4`).
const PLAYER_VTABLE: u32 = 0x010f_1c44;
const NAME_MAP_VTABLE: u32 = 0x010f_1c20;
const STRING_MAP_VTABLE: u32 = 0x010f_1ce4;
const MAP_BASE_VTABLE: u32 = 0x010f_1cc4;
const MAP_ROOT_VTABLE: u32 = 0x010f_1ca4;
/// `00438af0` (`thiscall`): the map base destructor body, run twice.
const MAP_BASE_DESTRUCT: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`): frees a block.
const NI_FREE: u32 = 0x00aa_10f0;
/// `00ec67aa` (`cdecl`, one character): the lower-case form used by the
/// hash.
const TO_LOWER: u32 = 0x00ec_67aa;

/// Direct3D 9 device slots used here.
const SET_RENDER_STATE: u32 = 0xe4;
const SET_TEXTURE: u32 = 0x104;
const CLEAR: u32 = 0xac;
const SET_VIEWPORT: u32 = 0xbc;
const BEGIN_SCENE: u32 = 0xa4;
const END_SCENE: u32 = 0xa8;
const PRESENT: u32 = 0x44;
const SET_RENDER_TARGET: u32 = 0x94;

/// `x * step` for an unsigned word: `FILD qword` (the word zero-extended),
/// `FMUL dword`, `FSTP dword`.
fn word_times(count: u32, factor: u32) -> u32 {
    x87_store32(x87_mul(f64::from(count), x87_m32(factor)))
}

/// `value + step` through the x87 stack (`FLD`, `FADD dword`, `FSTP`).
fn advance(value: u32, step: u32) -> u32 {
    x87_store32(x87_add(x87_load32(value), x87_m32(step)))
}

// Translated from 00ebff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the frame texture set, x, y, tile
/// width and height as floats): draws a movie frame tile by tile. The set
/// holds the device (+0), the tile grid (+0x24 columns, +0x28 rows), the
/// last column's and last row's partial sizes (+0x2C, +0x30), the tile size
/// (+0x3C) and the texture array (+0x40). After the draw set-up
/// (`00ec0460`) it turns render states 7 and 0xE off and sets state 0xA8 to
/// 0xF; then, row by row from `y`, each full tile and a partial last column
/// gets its texture (`SetTexture(0, ..)`) and a quad (`00ec0280`), the x
/// position advancing by `tile size * width` and the y by
/// `tile size * height`; a partial last row is drawn the same way. Finally
/// states 7 and 0xE back on and texture stage 0 cleared.
pub fn fn_00ebff30(e: &mut Engine, set: Ptr, x: f32, y: f32, width: f32, height: f32) {
    let set = set.addr();
    let device = |e: &Engine| e.mem.u32(set);
    let call_device = |e: &mut Engine, slot: u32, words: &[u32]| {
        let device = e.mem.u32(set);
        e.vcall(device, slot, words);
    };
    let d = device(e);
    e.call(DRAW_SETUP, &args![d]);
    call_device(e, SET_RENDER_STATE, &args![7u32, 0u32]);
    call_device(e, SET_RENDER_STATE, &args![0xeu32, 0u32]);
    call_device(e, SET_RENDER_STATE, &args![0xa8u32, 0xfu32]);
    let mut texture = e.mem.u32(set + 0x40);
    // Two products the code computes into locals and never reads: the tile
    // size times +8, and +0x1C times the tile size minus +0x24 times it
    // times +8 (plain loads, nothing written).
    let tile = e.mem.u32(set + 0x3c);
    let step_x = word_times(tile, width.to_bits());
    let step_y = word_times(e.mem.u32(set + 0x3c), height.to_bits());
    let mut row_y = x87_quiet_bits(y.to_bits());
    let width = x87_quiet_bits(width.to_bits());
    let height = x87_quiet_bits(height.to_bits());
    let x = x87_quiet_bits(x.to_bits());
    let draw = |e: &mut Engine, texture: &mut u32, at_x: u32, at_y: u32, w: u32, h: u32| {
        let current = e.mem.u32(*texture);
        call_device(e, SET_TEXTURE, &args![0u32, current]);
        *texture = texture.wrapping_add(4);
        let d = device(e);
        e.call(DRAW_QUAD, &args![d, at_x, at_y, width, height, w, h]);
    };
    let mut row = 0u32;
    while row < e.mem.u32(set + 0x28) {
        let mut column_x = x;
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let size = e.mem.u32(set + 0x3c);
            draw(e, &mut texture, column_x, row_y, size, size);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let (w, h) = (e.mem.u32(set + 0x2c), e.mem.u32(set + 0x3c));
            draw(e, &mut texture, column_x, row_y, w, h);
        }
        row_y = advance(row_y, step_y);
        row += 1;
    }
    if e.mem.u32(set + 0x30) != 0 {
        let mut column_x = x;
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let (w, h) = (e.mem.u32(set + 0x3c), e.mem.u32(set + 0x30));
            draw(e, &mut texture, column_x, row_y, w, h);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let (w, h) = (e.mem.u32(set + 0x2c), e.mem.u32(set + 0x30));
            draw(e, &mut texture, column_x, row_y, w, h);
        }
    }
    call_device(e, SET_RENDER_STATE, &args![7u32, 1u32]);
    call_device(e, SET_RENDER_STATE, &args![0xeu32, 1u32]);
    call_device(e, SET_TEXTURE, &args![0u32, 0u32]);
}

/// The `0.0` test of `Play`'s duration (`FCOMP qword`, `TEST AH,0x44`,
/// `JNP`): true when it is not equal (a NaN is not equal).
fn has_duration(e: &Engine, duration: u32) -> bool {
    x87_load32(duration) != f64::from_bits(e.mem.u64(ZERO))
}

// Translated from 00ec0e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Movie::Play` (Xbox PDB; `thiscall`, nine words, `RET 0x24`): marks the
/// movie playing (+0x20 = 1) and busy (+0x24) and opens it (`00ec0cd0` with
/// `name`, `open_1`, `open_2`, `open_3`); a failed open clears busy and
/// answers false. Otherwise clears busy, notifies the settings object
/// (`notify_a`: slots +4/+8, `notify_b`: +0xC/+0x10, before and after),
/// points the device's render target 0 (slot +0x94) at the renderer's first
/// target's surface (+0x14) when that target's class chain reaches the
/// `NiRTTI` at `0126f59c`, then runs `Update(go_on, keep_going, open_3)`
/// until it answers false. With a non-zero `duration` (seconds) `go_on`
/// starts at 0, and once more than `duration * 1000` ticks (truncated, low
/// word) have passed it becomes 1 and the duration 0. Answers whether every
/// frame was shown (`+0xC >= the Bink frame count`), after closing the
/// movie (`00ec1020`).
#[allow(clippy::too_many_arguments)]
pub fn movie_play(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    go_on: u8,
    open_1: u8,
    keep_going: u8,
    open_2: u8,
    open_3: u8,
    notify_a: u8,
    notify_b: u8,
    duration: f32,
) -> bool {
    let at = this.addr();
    e.mem.set_u32(at + 0x20, 1);
    e.mem.set_u8(at + 0x24, 1);
    let opened = e
        .call(
            MOVIE_OPEN,
            &args![
                at,
                name,
                u32::from(open_1),
                u32::from(open_2),
                u32::from(open_3)
            ],
        )
        .u8();
    if opened == 0 {
        e.mem.set_u8(at + 0x24, 0);
        return false;
    }
    e.mem.set_u8(at + 0x24, 0);
    if notify_a != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x4, &[]);
    }
    if notify_b != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0xc, &[]);
    }
    let renderer = e.mem.u32(RENDERER);
    let group = e.vcall(renderer, 0xc8, &[]).u32();
    let target = e.vcall(group, 0xc8, &args![0u32]).u32();
    let matching = if target == 0 {
        0
    } else {
        let mut rtti = e.vcall(target, 0xc, &[]).u32();
        let mut found = false;
        while rtti != 0 {
            if rtti == TARGET_RTTI {
                found = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
        if found {
            target
        } else {
            0
        }
    };
    if matching != 0 {
        let surface = e.mem.u32(matching + 0x14);
        let device = e.mem.u32(at + 4);
        e.vcall(device, SET_RENDER_TARGET, &args![0u32, surface]);
    }
    let mut go_on = go_on;
    let mut duration = duration.to_bits();
    if has_duration(e, duration) {
        go_on = 0;
    }
    let start = e.call(TICK_COUNT, &[]).u32();
    while movie_update(e, this, go_on, keep_going, open_3) {
        if has_duration(e, duration) {
            let now = e.call(TICK_COUNT, &[]).u32();
            let elapsed = now.wrapping_sub(start);
            let thousand = f64::from_bits(e.mem.u64(THOUSAND));
            let limit = x87_fistp64_chop(X87Ext::from(x87_mul(x87_load32(duration), thousand)));
            if elapsed > limit as u32 {
                go_on = 1;
                duration = 0.0f32.to_bits();
            }
        }
    }
    let bink = e.mem.u32(at);
    let shown_all = e.mem.u32(at + 0xc) >= e.mem.u32(bink + 8);
    e.call(MOVIE_CLOSE, &args![at]);
    if notify_a != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x8, &[]);
    }
    if notify_b != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x10, &[]);
    }
    shown_all
}

/// The letterbox viewport `Movie::Update` sets: the back buffer size
/// (renderer slot +0xD4: width +8, height +0xC) times the fractions
/// (`FILD qword`, `FMUL`, truncating `FISTP qword`, low word).
fn letterbox_viewport(e: &mut Engine, viewport: u32) {
    let renderer = e.mem.u32(RENDERER);
    let description = e.vcall(renderer, 0xd4, &[]).u32();
    let width = e.mem.u32(description + 8);
    let renderer = e.mem.u32(RENDERER);
    let description = e.vcall(renderer, 0xd4, &[]).u32();
    let height = e.mem.u32(description + 0xc);
    let left = e.mem.u32(VIEW_LEFT);
    let right = e.mem.u32(VIEW_RIGHT);
    let bottom = e.mem.u32(VIEW_BOTTOM);
    let top = e.mem.u32(VIEW_TOP);
    let chop = |value: f64| x87_fistp64_chop(X87Ext::from(value)) as u32;
    let x = chop(x87_mul(f64::from(width), x87_m32(left)));
    let w = chop(x87_mul(
        x87_sub(x87_load32(right), x87_m32(left)),
        f64::from(width),
    ));
    let y = chop(x87_mul(f64::from(height), x87_m32(top)));
    let h = chop(x87_mul(
        x87_sub(x87_load32(bottom), x87_m32(top)),
        f64::from(height),
    ));
    for (offset, word) in [x, y, w, h, 0.0f32.to_bits(), 1.0f32.to_bits()]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(viewport + 4 * offset as u32, word);
    }
}

// Translated from 00ec1060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Movie::Update` (Xbox PDB; `thiscall`, three bytes, `RET 0xC`): false
/// when the settings object's slot +0x40 (with `go_on`) answers false.
/// Unless the settings' byte +0x19 is set, it waits on Bink: a frame not
/// ready sleeps 1 ms and answers true (unless the state +0x20 is 2, which
/// draws anyway); a ready frame is copied (`00ec14d0`; a skipped copy counts
/// +0x10 and reports it through `Error`), advanced (`BinkNextFrame`) and
/// counted (+0xC). Then it clears the target (black, z 1.0), sets the
/// letterbox viewport (unless `no_viewport`, when `011f9426` is set),
/// draws the frame (`00ebff30` with the position and the scale as both tile
/// sizes) inside a scene unless the settings' slot +0x30 says not to,
/// presents unless the state is 2, restores the full viewport the same way,
/// and answers whether frames remain or `keep_going` is set.
pub fn movie_update(e: &mut Engine, this: Ptr, go_on: u8, keep_going: u8, no_viewport: u8) -> bool {
    let at = this.addr();
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x40, &args![u32::from(go_on)]).bool() {
        return false;
    }
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if e.mem.u8(settings + 0x19) == 0 {
        let bink = e.mem.u32(at);
        if e.call(BINK_WAIT, &args![bink]).u32() == 0 {
            let bink = e.mem.u32(at);
            let textures = e.mem.u32(at + 8);
            let skipped = e.call(COPY_FRAME, &args![bink, textures, 0u32]).u8();
            if skipped != 0 {
                let count = e.mem.u32(at + 0x10).wrapping_add(1);
                e.mem.set_u32(at + 0x10, count);
                let frame = e.mem.u32(at + 0xc);
                e.call(ERROR, &args![SKIPPED_FRAME_MESSAGE, frame, count]);
            }
            let bink = e.mem.u32(at);
            e.call(BINK_NEXT_FRAME, &args![bink]);
            let shown = e.mem.u32(at + 0xc).wrapping_add(1);
            e.mem.set_u32(at + 0xc, shown);
        } else if e.mem.u32(at + 0x20) != 2 {
            e.call(SLEEP, &args![1u32]);
            return true;
        }
    }
    let device = e.mem.u32(at + 4);
    e.vcall(
        device,
        CLEAR,
        &args![0u32, 0u32, 1u32, 0xff00_0000u32, 1.0f32, 0u32],
    );
    if no_viewport == 0 && e.mem.u8(LETTERBOX) != 0 {
        e.with_stack(0x18, |e, viewport| {
            letterbox_viewport(e, viewport.addr());
            let device = e.mem.u32(at + 4);
            e.vcall(device, SET_VIEWPORT, &args![viewport]);
        });
    }
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x30, &[]).bool() {
        let device = e.mem.u32(at + 4);
        e.vcall(device, BEGIN_SCENE, &[]);
    }
    let textures = e.mem.u32(at + 8);
    let x = x87_quiet_bits(e.mem.u32(at + 0x18));
    let y = x87_quiet_bits(e.mem.u32(at + 0x1c));
    let scale = x87_quiet_bits(e.mem.u32(at + 0x14));
    fn_00ebff30(
        e,
        Ptr::new(textures),
        f32::from_bits(x),
        f32::from_bits(y),
        f32::from_bits(scale),
        f32::from_bits(scale),
    );
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x30, &[]).bool() {
        let device = e.mem.u32(at + 4);
        e.vcall(device, END_SCENE, &[]);
    }
    if e.mem.u32(at + 0x20) != 2 {
        let device = e.mem.u32(at + 4);
        e.vcall(device, PRESENT, &args![0u32, 0u32, 0u32, 0u32]);
    }
    if no_viewport == 0 && e.mem.u8(LETTERBOX) != 0 {
        let renderer = e.mem.u32(RENDERER);
        let description = e.vcall(renderer, 0xd4, &[]).u32();
        let width = e.mem.u32(description + 8);
        let renderer = e.mem.u32(RENDERER);
        let description = e.vcall(renderer, 0xd4, &[]).u32();
        let height = e.mem.u32(description + 0xc);
        e.with_stack(0x18, |e, viewport| {
            let viewport = viewport.addr();
            for (offset, word) in [0, 0, width, height, 0.0f32.to_bits(), 1.0f32.to_bits()]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(viewport + 4 * offset as u32, word);
            }
            let device = e.mem.u32(at + 4);
            e.vcall(device, SET_VIEWPORT, &args![viewport]);
        });
    }
    let bink = e.mem.u32(at);
    e.mem.u32(at + 0xc) < e.mem.u32(bink + 8) || keep_going != 0
}

// Translation incomplete: 00ec15e0 - the FS:[0] exception frame (unwinding the name map when the clean-up throws) is not translated
// Translated from 00ec15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body the `MoviePlayer` destructor
/// (`thiscall`): stores its vtable, stops movies (`00ec25e0`) while the
/// count at +0xC is non-zero, cleans up (`00ec2e50`), destroys the name map
/// at +0x40 (its vtable, then `00ec1a90`) and closes the handle at +0x34.
pub fn fn_00ec15e0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, PLAYER_VTABLE);
    while e.mem.u32(at + 0xc) != 0 {
        e.call(PLAYER_STOP, &args![at]);
    }
    e.call(PLAYER_CLEAN_UP, &args![at]);
    e.mem.set_u32(at + 0x40, NAME_MAP_VTABLE);
    fn_00ec1a90(e, Ptr::new(at + 0x40));
    let handle = e.mem.u32(at + 0x34);
    e.call(CLOSE_HANDLE, &args![handle]);
}

// Translated from 00ec17c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::GetPlayingSequence` (engine map; `thiscall`): false without
/// a thread handle (+8), else whether `GetExitCodeThread` reports the
/// thread still active (`0x103`).
pub fn movie_player_get_playing_sequence(e: &mut Engine, this: Ptr) -> bool {
    let thread = e.mem.u32(this.addr() + 8);
    if thread == 0 {
        return false;
    }
    e.with_stack(4, |e, code| {
        e.call(GET_EXIT_CODE_THREAD, &args![thread, code]);
        e.mem.u32(code.addr()) == STILL_ACTIVE
    })
}

// Translation incomplete: 00ec1a90 - the FS:[0] exception frame (unwinding the base destructors when one throws) is not translated
// Translated from 00ec1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char*, char*>, char*>::~NiTStringTemplateMap`
/// (engine map): stores its vtable; when the map owns its keys (byte +0x10)
/// frees each item's key (+4) in every bucket (+8, count +4) walking the
/// chains (next at +0); then the base vtables with the base destructor
/// body (`00438af0`) twice, and frees the bucket array.
pub fn ni_t_string_template_map_destruct(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, STRING_MAP_VTABLE);
    if e.mem.u8(at + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(at + 4) {
            let buckets = e.mem.u32(at + 8);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let next = e.mem.u32(item);
                let key = e.mem.u32(item + 4);
                e.call(NI_FREE, &args![key]);
                item = next;
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(at, MAP_BASE_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
}

/// The address `00ec15e0` calls the map destructor at.
fn fn_00ec1a90(e: &mut Engine, this: Ptr) {
    ni_t_string_template_map_destruct(e, this);
}

// Translated from 00ec1d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<char*>::KeyToHashIndex` (engine map;
/// `thiscall`, one word, `RET 4`): `hash = hash * 33 + lower(c)` over the
/// key's characters (each sign-extended, lowered by `00ec67aa`, the answer's
/// low byte sign-extended), modulo the bucket count (+4).
pub fn bst_case_insensitive_string_map_key_to_hash_index(
    e: &mut Engine,
    this: Ptr,
    key: u32,
) -> u32 {
    let mut hash = 0u32;
    let mut at = key;
    loop {
        let c = e.mem.u8(at) as i8;
        if c == 0 {
            break;
        }
        let lower = e.call(TO_LOWER, &args![i32::from(c)]).u8() as i8;
        hash = (hash << 5)
            .wrapping_add(hash)
            .wrapping_add(i32::from(lower) as u32);
        at = at.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

/// The `MoviePlayer` base constructor (`00ec20c0`, this unit; thiscall, no stack words; it
/// stores `PLAYER_VTABLE`).
const PLAYER_CONSTRUCT: u32 = 0x00ec_20c0;
/// `BGSMoviePlayer`'s vtable (RTTI `.?AVBGSMoviePlayer@@`).
const BGS_MOVIE_PLAYER_VTABLE: u32 = 0x0108_2564;
/// `BSAudio::QInstance` (engine map, `00453a70`; no arguments) and `004f1540` (thiscall:
/// the byte at +4 of the object, in `AL`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const OBJECT_BYTE_AT_4: u32 = 0x004f_1540;
/// `00ec1680` (this unit; thiscall, one word: when its low byte is set, Bink's sound system
/// is set to DirectSound).
const PLAYER_SET_SOUND: u32 = 0x00ec_1680;

// Translated from 008670e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008670e0 - the compiler's exception frame (FS:[0] chain, handler 00f13b98) is not translated
/// The engine map names it `MoviePlayer::Initialize`; the body is a constructor
/// (thiscall, no stack words): the `MoviePlayer` base (`00ec20c0`), then
/// `BGSMoviePlayer`'s vtable at `+0`, then `00ec1680(this, b)` with `b` the byte
/// `004f1540` answers for `BSAudio::QInstance()` (zero-extended). Returns `this`.
pub fn fn_008670e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PLAYER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BGS_MOVIE_PLAYER_VTABLE);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let flag = e.call(OBJECT_BYTE_AT_4, &args![audio]).u8();
    e.call(PLAYER_SET_SOUND, &args![this, u32::from(flag)]);
    this
}

/// More imports, by slot: `BinkClose` (`00fdf3a4`), `BinkDoFrame` (`00fdf3bc`),
/// `BinkCopyToBufferRect` (`00fdf3c0`), `BinkSetSoundSystem` (`00fdf3a0`), the slot holding
/// `BinkOpenDirectSound`'s address (`00fdf3b4`), `PeekMessageA` (`00fdf29c`),
/// `TranslateMessage` (`00fdf298`), `DispatchMessageA` (`00fdf294`) and `CreateSemaphoreA`
/// (`00fdf0b4`).
const BINK_CLOSE: u32 = 0x00fd_f3a4;
const BINK_DO_FRAME: u32 = 0x00fd_f3bc;
const BINK_COPY_TO_BUFFER_RECT: u32 = 0x00fd_f3c0;
const BINK_SET_SOUND_SYSTEM: u32 = 0x00fd_f3a0;
const BINK_OPEN_DIRECT_SOUND: u32 = 0x00fd_f3b4;
const PEEK_MESSAGE: u32 = 0x00fd_f29c;
const TRANSLATE_MESSAGE: u32 = 0x00fd_f298;
const DISPATCH_MESSAGE: u32 = 0x00fd_f294;
const CREATE_SEMAPHORE: u32 = 0x00fd_f0b4;
/// This unit's frame texture set helpers (unnamed, `cdecl`): its release (`00ebfd20`, the
/// set), the next tile rectangle (`00ebfda0`: the set and six word addresses; answers 0 when
/// there is none) and the step past it (`00ebff00`, the set).
const TEXTURES_RELEASE: u32 = 0x00eb_fd20;
const TEXTURES_NEXT_RECT: u32 = 0x00eb_fda0;
const TEXTURES_STEP: u32 = 0x00eb_ff00;
/// `00ec1570` (this unit, unnamed; `thiscall`, no stack words): what the close runs last.
const MOVIE_CLOSE_REST: u32 = 0x00ec_1570;
/// `00ec3020` (this unit, unnamed; `thiscall`, one word): the name map's base constructor
/// (given 0x25), and a vtable the player's constructor stores over it in turn.
const NAME_MAP_CONSTRUCT: u32 = 0x00ec_3020;
const MAP_VTABLE_010F1D04: u32 = 0x010f_1d04;
/// `006b7f20` (unnamed; `thiscall` on the map: the position, a key and a value, each by
/// address): the map's next item; and `MemoryManager::Deallocate` (`00aa4060`, `thiscall`
/// on the manager `011f6238`).
const MAP_NEXT_ITEM: u32 = 0x006b_7f20;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00ec1020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the movie's close. `BinkClose`
/// on the Bink handle (+0) when there is one, the frame texture set (+8) released
/// (`00ebfd20`) when there is one, then `00ec1570`.
pub fn fn_00ec1020(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u32(at) != 0 {
        let bink = e.mem.u32(at);
        e.call(BINK_CLOSE, &args![bink]);
    }
    if e.mem.u32(at + 8) != 0 {
        let textures = e.mem.u32(at + 8);
        e.call(TEXTURES_RELEASE, &args![textures]);
    }
    e.call(MOVIE_CLOSE_REST, &args![at]);
}

// Translated from 00ec14d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the Bink handle, the frame texture set, a flag): the
/// frame copy. `BinkDoFrame`, then for each tile rectangle `00ebfda0` hands out (six words
/// into locals) `BinkCopyToBufferRect` into it (the first two words, the Bink handle's word
/// at +4, the source position 0, 0 and the other four words, flags 3 with 0x80000000 when
/// `flag` is non-zero) and `00ebff00`. Answers whether the last copy (or none) answered
/// non-zero.
pub fn fn_00ec14d0(e: &mut Engine, bink: u32, textures: u32, flag: u32) -> bool {
    e.call(BINK_DO_FRAME, &args![bink]);
    // The locals, as the frame has them: +0 is [EBP-0x1C], +4 [EBP-0x18], +8 [EBP-0x14],
    // +0xC [EBP-0x10], +0x10 [EBP-0xC], +0x14 [EBP-8].
    let result = e.with_stack(0x18, |e, locals| {
        let b = locals.addr();
        let mut result = 1u32;
        while e
            .call(
                TEXTURES_NEXT_RECT,
                &args![textures, b, b + 0xc, b + 4, b + 0x14, b + 8, b + 0x10],
            )
            .u32()
            != 0
        {
            let flags = if flag != 0 { 0x8000_0000u32 } else { 0 } | 3;
            let height = e.mem.u32(bink.wrapping_add(4));
            let words = [
                bink,
                e.mem.u32(b),
                e.mem.u32(b + 0xc),
                height,
                0,
                0,
                e.mem.u32(b + 4),
                e.mem.u32(b + 0x14),
                e.mem.u32(b + 8),
                e.mem.u32(b + 0x10),
                flags,
            ];
            result = e.call(BINK_COPY_TO_BUFFER_RECT, &words).u32();
            e.call(TEXTURES_STEP, &args![textures]);
        }
        result
    });
    result != 0
}

// Translated from 00ec1680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): when the low byte of `use_sound` is
/// set, `BinkSetSoundSystem` with the address in the `BinkOpenDirectSound` import slot and
/// 0. `this` is not read.
pub fn fn_00ec1680(e: &mut Engine, _this: Ptr, use_sound: u8) {
    if use_sound != 0 {
        let open = e.mem.u32(BINK_OPEN_DIRECT_SOUND);
        e.call(BINK_SET_SOUND_SYSTEM, &args![open, 0u32]);
    }
}

// Translated from 00ec1840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::CheckContinuePlaying` (engine map; `thiscall`, `RET 4`: one stack word the
/// body never reads): pumps the window messages (`PeekMessageA` with remove, then
/// `TranslateMessage` and `DispatchMessageA` on each) and answers whether the byte at +0x21
/// is clear.
pub fn movie_player_check_continue_playing(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    e.with_stack(0x1c, |e, message| {
        let message = message.addr();
        while e
            .call(PEEK_MESSAGE, &args![message, 0u32, 0u32, 0u32, 1u32])
            .u32()
            != 0
        {
            e.call(TRANSLATE_MESSAGE, &args![message]);
            e.call(DISPATCH_MESSAGE, &args![message]);
        }
    });
    e.mem.u8(this.addr() + 0x21) == 0
}

// Translated from 00ec20c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec20c0 - the FS:[0] exception frame (unwind state 0 for the name map) is not translated
/// The `MoviePlayer` constructor (`thiscall`, no stack words): its vtable (`010f1c44`), the
/// words +4 to +0x14 and +0x1C and the bytes +0x18, +0x19, +0x20 to +0x22 cleared, the bytes
/// +0x23 and +0x24 set to 1, +0x2C cleared; a semaphore (+0x30 initial count 0, +0x38
/// maximum 1, `CreateSemaphoreA` with those words read back, no attributes or name) whose
/// handle goes to +0x34; then the name map at +0x40: its base (`00ec3020` with 0x25), the
/// vtables `010f1cc4` and `010f1ce4`, its byte +0x10 set to 1, the vtables `010f1d04` and
/// `010f1c20`. Returns `this`.
pub fn fn_00ec20c0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, PLAYER_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(at + offset, 0);
    }
    e.mem.set_u8(at + 0x18, 0);
    e.mem.set_u8(at + 0x19, 0);
    e.mem.set_u32(at + 0x1c, 0);
    e.mem.set_u8(at + 0x20, 0);
    e.mem.set_u8(at + 0x21, 0);
    e.mem.set_u8(at + 0x22, 0);
    e.mem.set_u8(at + 0x23, 1);
    e.mem.set_u8(at + 0x24, 1);
    e.mem.set_u32(at + 0x2c, 0);
    let semaphore = at + 0x30;
    e.mem.set_u32(semaphore, 0);
    e.mem.set_u32(semaphore + 8, 1);
    let maximum = e.mem.u32(semaphore + 8);
    let initial = e.mem.u32(semaphore);
    let handle = e
        .call(CREATE_SEMAPHORE, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.mem.set_u32(semaphore + 4, handle);
    let map = at + 0x40;
    e.call(NAME_MAP_CONSTRUCT, &args![map, 0x25u32]);
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.mem.set_u32(map, STRING_MAP_VTABLE);
    e.mem.set_u8(map + 0x10, 1);
    e.mem.set_u32(map, MAP_VTABLE_010F1D04);
    e.mem.set_u32(map, NAME_MAP_VTABLE);
    this
}

// Translated from 00ec2e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the player's clean-up of its
/// name map at +0x40 (bucket count at +4, buckets at +8). From the first non-null bucket
/// head, `006b7f20` hands out the items (position, key and value locals, the key and value
/// cleared before each step) and each value is freed (`MemoryManager::Deallocate`) until
/// the position is null; then the map's base destructor body (`00438af0`).
pub fn fn_00ec2e50(e: &mut Engine, this: Ptr) {
    let map = this.addr() + 0x40;
    let mut first = 0u32;
    let mut index = 0u32;
    while index < e.mem.u32(map + 4) {
        let buckets = e.mem.u32(map + 8);
        let head = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if head != 0 {
            first = head;
            break;
        }
        index += 1;
    }
    // The locals: the position (+0), the key (+4) and the value (+8).
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(position + 4, 0);
            e.mem.set_u32(position + 8, 0);
            e.call(
                MAP_NEXT_ITEM,
                &args![map, position, position + 4, position + 8],
            );
            let value = e.mem.u32(position + 8);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, value]);
        }
    });
    e.call(MAP_BASE_DESTRUCT, &args![map]);
}

/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on the manager, one word).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiAlloc`-style allocation (`00aa1070`, `cdecl`, one word: the size).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `_strlen` (`00ec6130`), `_strcpy_s` (`00ec65a6`: destination, size, source) and
/// `_stricmp` (`00ec68e4`), all `cdecl`.
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRICMP: u32 = 0x00ec_68e4;
/// The Direct3D texture slots the frame texture set uses: `LockRect` (`+0x4C`: level,
/// locked rectangle, rectangle, flags) and `UnlockRect` (`+0x50`: level).
const LOCK_RECT: u32 = 0x4c;
const UNLOCK_RECT: u32 = 0x50;

// Translated from 00ebfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the frame texture set's release. With a
/// texture array (+0x40), releases (COM slot +8) each non-null texture of the +0x20 count,
/// clears the array word and gives the set itself back to the memory manager.
pub fn fn_00ebfd20(e: &mut Engine, set: Ptr) {
    let at = set.addr();
    if e.mem.u32(at + 0x40) != 0 {
        let mut i = 0u32;
        while i < e.mem.u32(at + 0x20) {
            let slot = e.mem.u32(at + 0x40).wrapping_add(i.wrapping_mul(4));
            let texture = e.mem.u32(slot);
            if texture != 0 {
                e.vcall(texture, 8, &args![]);
            }
            i = i.wrapping_add(1);
        }
        e.mem.set_u32(at + 0x40, 0);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, at]);
    }
}

// Translated from 00ebfda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, seven words): the frame texture set's next tile.
/// While the cursor (+0x14) is below the texture count (+0x20) and locking the cursor's
/// texture (`LockRect(0, &rect, 0, 0)`) succeeds, hands out (each where its address is not
/// 0) the locked bits and pitch, the tile's x and y in pixels (column and row of the cursor
/// in a grid of +0x24 full columns plus one when the partial width +0x2C is not 0, times the
/// tile size +0x3C) and its width and height (the tile size, or the partial sizes +0x2C /
/// +0x30 from column +0x24 / row +0x28 on), advances the cursor and answers 1; otherwise
/// resets the cursor and answers 0.
pub fn fn_00ebfda0(
    e: &mut Engine,
    set: Ptr,
    bits: u32,
    pitch: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> u32 {
    let at = set.addr();
    if e.mem.u32(at + 0x14) < e.mem.u32(at + 0x20) {
        let locked = e.with_stack(8, |e, rect| {
            let rect = rect.addr();
            let cursor = e.mem.u32(at + 0x14);
            let texture = e
                .mem
                .u32(e.mem.u32(at + 0x40).wrapping_add(cursor.wrapping_mul(4)));
            let r = e
                .vcall(texture, LOCK_RECT, &args![0u32, rect, 0u32, 0u32])
                .i32();
            // D3DLOCKED_RECT: Pitch at +0, pBits at +4.
            (r >= 0).then(|| (e.mem.u32(rect + 4), e.mem.u32(rect)))
        });
        if let Some((locked_bits, locked_pitch)) = locked {
            if bits != 0 {
                e.mem.set_u32(bits, locked_bits);
            }
            if pitch != 0 {
                e.mem.set_u32(pitch, locked_pitch);
            }
            let columns = u32::from(e.mem.u32(at + 0x2c) != 0).wrapping_add(e.mem.u32(at + 0x24));
            let cursor = e.mem.u32(at + 0x14);
            let row = cursor / columns;
            let column = cursor.wrapping_sub(columns.wrapping_mul(row));
            if x != 0 {
                let v = column.wrapping_mul(e.mem.u32(at + 0x3c));
                e.mem.set_u32(x, v);
            }
            if y != 0 {
                let v = row.wrapping_mul(e.mem.u32(at + 0x3c));
                e.mem.set_u32(y, v);
            }
            if width != 0 {
                let v = if column < e.mem.u32(at + 0x24) {
                    e.mem.u32(at + 0x3c)
                } else {
                    e.mem.u32(at + 0x2c)
                };
                e.mem.set_u32(width, v);
            }
            if height != 0 {
                let v = if row < e.mem.u32(at + 0x28) {
                    e.mem.u32(at + 0x3c)
                } else {
                    e.mem.u32(at + 0x30)
                };
                e.mem.set_u32(height, v);
            }
            let next = e.mem.u32(at + 0x14).wrapping_add(1);
            e.mem.set_u32(at + 0x14, next);
            return 1;
        }
    }
    e.mem.set_u32(at + 0x14, 0);
    0
}

// Translated from 00ebff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the frame texture set's step past a tile:
/// unlocks (`UnlockRect(0)`) the texture before the cursor (+0x14) in the array (+0x40),
/// passing its answer on.
pub fn fn_00ebff00(e: &mut Engine, set: Ptr) -> u32 {
    let at = set.addr();
    let cursor = e.mem.u32(at + 0x14);
    let slot = e
        .mem
        .u32(at + 0x40)
        .wrapping_add(cursor.wrapping_mul(4))
        .wrapping_sub(4);
    let texture = e.mem.u32(slot);
    e.vcall(texture, UNLOCK_RECT, &args![0u32]).u32()
}

// Translated from 00ec1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the movie's reset, run last by
/// its close. Clears the words +0 to +0x10, sets the scale (+0x14) to 1.0, the position
/// (+0x18, +0x1C) to 0.0, clears the play state (+0x20) and the busy byte (+0x24).
pub fn fn_00ec1570(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        e.mem.set_u32(at + offset, 0);
    }
    e.mem.set_f32(at + 0x14, 1.0);
    e.mem.set_f32(at + 0x18, 0.0);
    e.mem.set_f32(at + 0x1c, 0.0);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u8(at + 0x24, 0);
}

// Translated from 00ec1a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<const char*, char*>::NewItem` (engine map; `thiscall`, no stack words, `this`
/// not read): a 0xC-byte item from the memory manager.
pub fn ni_t_map_new_item(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32()
}

// Translated from 00ec1a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<const char*, char*>::DeleteItem` (engine map; `thiscall`, one word, `this` not
/// read): clears the item's value (+8) and gives the item back to the memory manager, passing
/// its answer on.
pub fn ni_t_map_delete_item(e: &mut Engine, _this: Ptr, item: u32) -> u32 {
    e.mem.set_u32(item.wrapping_add(8), 0);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, item])
        .u32()
}

// Translated from 00ec1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char*, char*>, char*>::KeyToHashIndex` (engine map;
/// `thiscall`, one word): `hash = hash * 33 + c` over the key's characters (each
/// sign-extended), modulo the bucket count (+4).
pub fn ni_t_string_template_map_key_to_hash_index(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut at = key;
    loop {
        let c = e.mem.u8(at) as i8;
        if c == 0 {
            break;
        }
        hash = (hash << 5)
            .wrapping_add(hash)
            .wrapping_add(i32::from(c) as u32);
        at = at.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

// Translated from 00ec1cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char*, char*>, char*>::SetValue` (engine map;
/// `thiscall`, three words): the item's key (+4) is `key` itself, or, when the map copies
/// its keys (byte +0x10), a copy (`strlen + 1` bytes from `00aa1070`, filled by
/// `strcpy_s`); the item's value (+8) is `value`.
pub fn ni_t_string_template_map_set_value(
    e: &mut Engine,
    this: Ptr,
    item: u32,
    key: u32,
    value: u32,
) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let size = e.call(STRLEN, &args![key]).u32().wrapping_add(1);
        let copy = e.call(NI_ALLOC, &args![size]).u32();
        e.mem.set_u32(item.wrapping_add(4), copy);
        let copy = e.mem.u32(item.wrapping_add(4));
        e.call(STRCPY_S, &args![copy, size, key]);
    } else {
        e.mem.set_u32(item.wrapping_add(4), key);
    }
    e.mem.set_u32(item.wrapping_add(8), value);
}

// Translated from 00ec1dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<char*>::IsKeysEqual` (engine map; `thiscall`, two words,
/// `this` not read): whether `_stricmp` finds the keys equal (1 or 0).
pub fn bst_case_insensitive_string_map_is_keys_equal(
    e: &mut Engine,
    _this: Ptr,
    first: u32,
    second: u32,
) -> bool {
    e.call(STRICMP, &args![first, second]).u32() == 0
}

// Translated from 00ec1df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<const char*, char*>>, const char*, char*>::scalar deleting
/// destructor` (engine map; `thiscall`, one word): stores the base vtable (`010f1ca4`), runs
/// the base destructor body (`00438af0`), frees the bucket array (+8, `00aa10f0`) and, with
/// bit 0 of `flags`, gives the map back to the memory manager. Answers `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, at]);
    }
    this
}

// Translated from 00ec1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<const char*, char*>::scalar deleting destructor` (engine map; `thiscall`, one
/// word): the `NiTMap` vtable (`010f1cc4`) and the base destructor body (`00438af0`), then
/// as [`ni_t_map_base_scalar_deleting_destructor`]: the base vtable (`010f1ca4`), the body
/// again, the bucket array (+8) freed and, with bit 0 of `flags`, the map given back to the
/// memory manager. Answers `this`.
// Translation incomplete: 00ec1e40 - its FS:[0] exception frame (unwinding to the base destructor) is not translated
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, MAP_BASE_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, at]);
    }
    this
}

// Translated from 00ec1ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<char*>::scalar deleting destructor` (engine map;
/// `thiscall`, one word): stores its vtable (`010f1c20`), destroys the string map
/// ([`ni_t_string_template_map_destruct`]) and, with bit 0 of `flags`, gives it back to the
/// memory manager. Answers `this`.
pub fn bst_case_insensitive_string_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NAME_MAP_VTABLE);
    ni_t_string_template_map_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ec2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char*, char*>, char*>::scalar deleting destructor`
/// (engine map; `thiscall`, one word): destroys the map
/// ([`ni_t_string_template_map_destruct`]) and, with bit 0 of `flags`, gives it back to the
/// memory manager. Answers `this`.
pub fn ni_t_string_template_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_t_string_template_map_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// `CreateThread`, `ResumeThread`, `SetThreadPriority`, `WaitForSingleObject` and
/// `InterlockedDecrement` (KERNEL32 import slots).
const CREATE_THREAD: u32 = 0x00fd_f124;
const RESUME_THREAD: u32 = 0x00fd_f158;
const SET_THREAD_PRIORITY: u32 = 0x00fd_f11c;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiThread`'s name setter (`00aa2740`, `cdecl`: thread id, name) and the name it gets
/// (`010f1c88`).
const SET_THREAD_NAME: u32 = 0x00aa_2740;
const SEQUENCE_THREAD_NAME: u32 = 0x010f_1c88;
/// The thread procedure `StartSequence` starts (`00ec16b0`) and what it runs: `00ec2760` on the
/// movie settings object `0126fac4`.
const SEQUENCE_THREAD_PROC: u32 = 0x00ec_16b0;
const SEQUENCE_RUN: u32 = 0x00ec_2760;
/// The default priority setting's word (`0126fae4`, inside the object at `0126fae0`), and the
/// fallback word `01202800` the exe uses when that object's address is null (it never is).
const DEFAULT_PRIORITY_HOLDER: u32 = 0x0126_fae0;
const DEFAULT_PRIORITY: u32 = 0x0126_fae4;
const NULL_PRIORITY: u32 = 0x0120_2800;

// Translated from 00ec16b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The sequence thread's procedure (`stdcall`, its parameter): `00ec2760` on the movie settings
/// object (`0126fac4`) with the parameter; answers 0.
pub fn fn_00ec16b0(e: &mut Engine, parameter: u32) -> u32 {
    let settings = e.mem.u32(MOVIE_SETTINGS);
    e.call(SEQUENCE_RUN, &args![settings, parameter]);
    0
}

// Translated from 00ec16d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::StartSequence` (engine map; `thiscall`, `RET 0x18`): creates the sequence
/// thread suspended (`CreateThread(0, 0, 00ec16b0, parameter, CREATE_SUSPENDED, &id)`, the
/// handle to +8), names it (`00aa2740(id, 010f1c88)`), stores the bytes +0x22, +0x23, +0x24 and
/// the float +0x28 (`FLD`/`FSTP`: a signalling NaN is stored quiet), sets the thread's priority
/// (`00ec18a0`) to `priority` or, for -1, the word at `0126fae4` (the exe tests the constant
/// address `0126fae0` for null first; when it is, which it never is, it clears `01202800` and
/// reads that), resumes the thread, then waits without a time limit on the semaphore (+0x34)
/// and decrements the count at +0x30 (`InterlockedDecrement`), whose answer stays in `EAX`.
#[allow(clippy::too_many_arguments)]
pub fn movie_player_start_sequence(
    e: &mut Engine,
    this: Ptr,
    parameter: u32,
    byte_22: u8,
    priority: u32,
    byte_23: u8,
    byte_24: u8,
    value_28: f32,
) -> u32 {
    let at = this.addr();
    e.with_stack(4, |e, id| {
        let handle = e
            .call(
                CREATE_THREAD,
                &args![0u32, 0u32, SEQUENCE_THREAD_PROC, parameter, 4u32, id],
            )
            .u32();
        e.mem.set_u32(at + 8, handle);
        let thread_id = e.mem.u32(id.addr());
        e.call(SET_THREAD_NAME, &args![thread_id, SEQUENCE_THREAD_NAME]);
        e.mem.set_u8(at + 0x22, byte_22);
        e.mem.set_u8(at + 0x23, byte_23);
        e.mem.set_u8(at + 0x24, byte_24);
        e.mem.set_u32(at + 0x28, x87_quiet_bits(value_28.to_bits()));
        let priority = if priority == 0xffff_ffff {
            #[allow(clippy::if_same_then_else)]
            let word = if DEFAULT_PRIORITY_HOLDER != 0 {
                DEFAULT_PRIORITY
            } else {
                // Never taken: the exe tests a constant address.
                e.mem.set_u32(NULL_PRIORITY, 0);
                NULL_PRIORITY
            };
            e.mem.u32(word)
        } else {
            priority
        };
        fn_00ec18a0(e, this, priority);
        let handle = e.mem.u32(at + 8);
        e.call(RESUME_THREAD, &args![handle]);
        e.mem.set_u32(id.addr(), 0);
        let semaphore = e.mem.u32(at + 0x34);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![semaphore, 0xffff_ffffu32]);
        e.call(INTERLOCKED_DECREMENT, &args![at + 0x30]).u32()
    })
}

// Translated from 00ec18a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`movieplayer_xenon.obj`; `thiscall`, `RET 4`): the sequence
/// thread's priority. The setting (0 to 5; the exe's jump table) becomes
/// `THREAD_PRIORITY_LOWEST` (-2), `BELOW_NORMAL` (-1), `NORMAL` (0), `ABOVE_NORMAL` (1),
/// `HIGHEST` (2) or `TIME_CRITICAL` (15), anything else `NORMAL`; the setting goes to +0x2c and
/// `SetThreadPriority(+8, priority)` answers.
pub fn fn_00ec18a0(e: &mut Engine, this: Ptr, setting: u32) -> u32 {
    let priority: u32 = match setting {
        0 => 0xffff_fffe,
        1 => 0xffff_ffff,
        2 => 0,
        3 => 1,
        4 => 2,
        5 => 0xf,
        _ => 0,
    };
    let at = this.addr();
    e.mem.set_u32(at + 0x2c, setting);
    let handle = e.mem.u32(at + 8);
    e.call(SET_THREAD_PRIORITY, &args![handle, priority]).u32()
}

/// The movie thread's procedure (`00ec2ca0`) and the name it gets (`010f1c94`).
const MOVIE_THREAD_PROC: u32 = 0x00ec_2ca0;
const MOVIE_THREAD_NAME: u32 = 0x010f_1c94;

// Translated from 00ec1940 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`movieplayer_xenon.obj`; `thiscall`, two words: the thread's
/// parameter and a callback, `RET 8`): the callback goes to +0x1c; the movie thread is created
/// suspended (`CreateThread(0, 0, 00ec2ca0, parameter, CREATE_SUSPENDED, &id)`, the handle to
/// +4), named (`00aa2740(id, 010f1c94)`) and resumed (`ResumeThread`), whose answer stays in
/// `EAX`; the id local is cleared.
pub fn fn_00ec1940(e: &mut Engine, this: Ptr, parameter: u32, callback: u32) -> u32 {
    let at = this.addr();
    e.mem.set_u32(at + 0x1c, callback);
    e.with_stack(4, |e, id| {
        let handle = e
            .call(
                CREATE_THREAD,
                &args![0u32, 0u32, MOVIE_THREAD_PROC, parameter, 4u32, id],
            )
            .u32();
        e.mem.set_u32(at + 4, handle);
        let thread_id = e.mem.u32(id.addr());
        e.call(SET_THREAD_NAME, &args![thread_id, MOVIE_THREAD_NAME]);
        let handle = e.mem.u32(at + 4);
        let resumed = e.call(RESUME_THREAD, &args![handle]).u32();
        e.mem.set_u32(id.addr(), 0);
        resumed
    })
}

// Translated from 00ec19a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`movieplayer_xenon.obj`; `thiscall`, plain `RET`): with a thread
/// handle at +4, waits on it without a time limit (`WaitForSingleObject`).
pub fn fn_00ec19a0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u32(at + 4) != 0 {
        let handle = e.mem.u32(at + 4);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
    }
}

// Translated from 00ec19d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer`'s scalar deleting destructor (engine map; `thiscall`, one word: the flags,
/// `RET 4`): [`fn_00ec15e0`], then with bit 0 of the flags the block is freed (`00aa4060`).
/// Answers `this`.
pub fn movie_player_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ec15e0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ec1b80 (disassembled, FalloutNV.exe 1.4.0.525)
/// A map's destructor body (unnamed in the engine map; `thiscall`, plain `RET`): the vtable
/// `010f1ca4`, `00438af0` on `this`, then the bucket array at +8 freed (`00aa10f0`, cdecl).
pub fn fn_00ec1b80(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00ec1bb0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec1bb0 - the SEH frame (FS:[0] handler 00f34b18, state 0 around the first base destructor) is not translated.
/// The derived map's destructor body (unnamed in the engine map; `thiscall`, plain `RET`): the
/// vtable `010f1cc4` and `00438af0`, then the vtable `010f1ca4`, `00438af0` again and the
/// bucket array at +8 freed (`00aa10f0`).
pub fn fn_00ec1bb0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, MAP_BASE_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00ec1d30 (disassembled, FalloutNV.exe 1.4.0.525)
/// The name map's destructor body (unnamed in the engine map; `thiscall`, plain `RET`): the
/// vtable `010f1c20`, then `00ec1a90` ([`ni_t_string_template_map_destruct`]).
pub fn fn_00ec1d30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NAME_MAP_VTABLE);
    fn_00ec1a90(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_1940, fn_00ec1940(Ptr, u32, u32) -> u32),
        entry!(0x00ec_19a0, fn_00ec19a0(Ptr)),
        entry!(
            0x00ec_19d0,
            movie_player_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00ec_1b80, fn_00ec1b80(Ptr)),
        entry!(0x00ec_1bb0, fn_00ec1bb0(Ptr)),
        entry!(0x00ec_1d30, fn_00ec1d30(Ptr)),
        entry!(0x00ec_16b0, fn_00ec16b0(u32) -> u32),
        entry!(
            0x00ec_16d0,
            movie_player_start_sequence(Ptr, u32, u8, u32, u8, u8, f32) -> u32
        ),
        entry!(0x00ec_18a0, fn_00ec18a0(Ptr, u32) -> u32),
        entry!(0x00eb_fd20, fn_00ebfd20(Ptr)),
        entry!(
            0x00eb_fda0,
            fn_00ebfda0(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00eb_ff00, fn_00ebff00(Ptr) -> u32),
        entry!(0x00ec_1570, fn_00ec1570(Ptr)),
        entry!(0x00ec_1a40, ni_t_map_new_item(Ptr) -> u32),
        entry!(0x00ec_1a60, ni_t_map_delete_item(Ptr, u32) -> u32),
        entry!(
            0x00ec_1c30,
            ni_t_string_template_map_key_to_hash_index(Ptr, u32) -> u32
        ),
        entry!(
            0x00ec_1cc0,
            ni_t_string_template_map_set_value(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00ec_1dc0,
            bst_case_insensitive_string_map_is_keys_equal(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00ec_1df0,
            ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ec_1e40,
            ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ec_1ee0,
            bst_case_insensitive_string_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ec_2ff0,
            ni_t_string_template_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0086_70e0, fn_008670e0(Ptr) -> Ptr),
        entry!(0x00ebff30, fn_00ebff30(Ptr, f32, f32, f32, f32)),
        entry!(
            0x00ec0e00,
            movie_play(Ptr, u32, u8, u8, u8, u8, u8, u8, u8, f32) -> bool
        ),
        entry!(0x00ec1060, movie_update(Ptr, u8, u8, u8) -> bool),
        entry!(0x00ec15e0, fn_00ec15e0(Ptr)),
        entry!(0x00ec17c0, movie_player_get_playing_sequence(Ptr) -> bool),
        entry!(0x00ec1a90, ni_t_string_template_map_destruct(Ptr)),
        entry!(0x00ec_1020, fn_00ec1020(Ptr)),
        entry!(0x00ec_14d0, fn_00ec14d0(u32, u32, u32) -> bool),
        entry!(0x00ec_1680, fn_00ec1680(Ptr, u8)),
        entry!(
            0x00ec_1840,
            movie_player_check_continue_playing(Ptr, u32) -> bool
        ),
        entry!(0x00ec_20c0, fn_00ec20c0(Ptr) -> Ptr),
        entry!(0x00ec_2e50, fn_00ec2e50(Ptr)),
        entry!(
            0x00ec1d50,
            bst_case_insensitive_string_map_key_to_hash_index(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A device whose every slot up to +0x108 is a recording no-op at
    /// `0x0020_0000 + slot`.
    fn device(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x120);
        for slot in (0..0x120u32).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0020_0000 + slot);
            e.register(0x0020_0000 + slot, |_, _| Ret::default());
        }
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, vtable);
        device
    }

    #[test]
    fn hash_index_lowers_and_wraps() {
        let mut e = Engine::new();
        e.register(TO_LOWER, |_, a| {
            let c = a[0] as u8;
            u32::from(c.to_ascii_lowercase()).into_ret()
        });
        let key = e.mem.alloc(4);
        e.mem.write(key, b"Ab\0");
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 4, 37);
        let expected = (u32::from(b'a') * 33 + u32::from(b'b')) % 37;
        assert_eq!(e.call(0x00ec_1d50, &args![map, key]).u32(), expected);
    }

    #[test]
    fn playing_sequence_asks_the_thread() {
        let mut e = Engine::new();
        e.register(GET_EXIT_CODE_THREAD, |e, a| {
            e.mem
                .set_u32(a[1], if a[0] == 1 { STILL_ACTIVE } else { 0 });
            1u32.into_ret()
        });
        let player = e.mem.alloc(0x10);
        assert!(!e.call(0x00ec_17c0, &args![player]).bool());
        e.mem.set_u32(player + 8, 1);
        assert!(e.call(0x00ec_17c0, &args![player]).bool());
        e.mem.set_u32(player + 8, 2);
        assert!(!e.call(0x00ec_17c0, &args![player]).bool());
    }

    #[test]
    fn map_destructor_frees_owned_keys() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MAP_BASE_DESTRUCT, |_, _| Ret::default());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x111);
        e.mem.set_u32(second + 4, 0x222);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1a90, &args![map]);
        assert_eq!(
            calls_to(&e, NI_FREE),
            vec![vec![0x111], vec![0x222], vec![buckets]]
        );
        assert_eq!(calls_to(&e, MAP_BASE_DESTRUCT).len(), 2);
        assert_eq!(e.mem.u32(map), MAP_ROOT_VTABLE);
        // Keys not owned: only the buckets are freed.
        e.mem.set_u8(map + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1a90, &args![map]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![buckets]]);
    }

    #[test]
    fn player_destructor_stops_then_closes() {
        let mut e = Engine::new();
        e.register(PLAYER_STOP, |e, a| {
            let count = e.mem.u32(a[0] + 0xc) - 1;
            e.mem.set_u32(a[0] + 0xc, count);
            Ret::default()
        });
        for address in [PLAYER_CLEAN_UP, MAP_BASE_DESTRUCT, NI_FREE, CLOSE_HANDLE] {
            e.register(address, |_, _| Ret::default());
        }
        let player = e.mem.alloc(0x60);
        e.mem.set_u32(player + 0xc, 2);
        e.mem.set_u32(player + 0x34, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00ec_15e0, &args![player]);
        assert_eq!(calls_to(&e, PLAYER_STOP).len(), 2);
        assert_eq!(e.mem.u32(player), PLAYER_VTABLE);
        assert_eq!(e.mem.u32(player + 0x40), MAP_ROOT_VTABLE);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x99]]);
    }

    #[test]
    fn frame_draw_tiles_the_grid() {
        let mut e = Engine::new();
        e.register(DRAW_SETUP, |_, _| Ret::default());
        e.register(DRAW_QUAD, |_, _| Ret::default());
        let device = device(&mut e);
        let set = e.mem.alloc(0x44);
        let textures = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(textures + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(set, device);
        e.mem.set_u32(set + 0x24, 2);
        e.mem.set_u32(set + 0x28, 1);
        e.mem.set_u32(set + 0x2c, 5);
        e.mem.set_u32(set + 0x30, 7);
        e.mem.set_u32(set + 0x3c, 64);
        e.mem.set_u32(set + 0x40, textures);
        e.call_log = Some(vec![]);
        e.call(0x00eb_ff30, &args![set, 10.0f32, 20.0f32, 0.5f32, 2.0f32]);
        let quads = calls_to(&e, DRAW_QUAD);
        let f = |v: f32| v.to_bits();
        let size = (f(0.5), f(2.0));
        let quad = |x: f32, y: f32, w: u32, h: u32| vec![device, f(x), f(y), size.0, size.1, w, h];
        assert_eq!(
            quads,
            vec![
                quad(10.0, 20.0, 64, 64),
                quad(42.0, 20.0, 64, 64),
                quad(74.0, 20.0, 5, 64),
                quad(10.0, 148.0, 64, 7),
                quad(42.0, 148.0, 64, 7),
                quad(74.0, 148.0, 5, 7),
            ]
        );
        let textures_set: Vec<u32> = calls_to(&e, 0x0020_0000 + SET_TEXTURE)
            .into_iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(textures_set, [0x100, 0x101, 0x102, 0x103, 0x104, 0x105, 0]);
        let states = calls_to(&e, 0x0020_0000 + SET_RENDER_STATE);
        assert_eq!(states[2], vec![device, 0xa8, 0xf]);
        assert_eq!(states[4], vec![device, 0xe, 1]);
    }

    /// A movie on a device, a settings object answering `go_on` from slot
    /// +0x40 and false from +0x30, and a renderer with a 1000x500 back
    /// buffer.
    fn movie_engine(frames: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let device = device(&mut e);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x6000);
        e.map(0x011a_d000, 0x1000);
        let settings_vtable = e.mem.alloc(0x50);
        for slot in (0..0x50u32).step_by(4) {
            e.mem.set_u32(settings_vtable + slot, 0x0021_0000 + slot);
            e.register(0x0021_0000 + slot, |_, _| Ret::default());
        }
        e.register(0x0021_0040, |_, a| a[1].into_ret());
        let settings = e.mem.alloc(0x20);
        e.mem.set_u32(settings, settings_vtable);
        e.set_global(MOVIE_SETTINGS, settings);
        let description = e.mem.alloc(0x10);
        e.mem.set_u32(description + 8, 1000);
        e.mem.set_u32(description + 0xc, 500);
        let renderer_vtable = e.mem.alloc(0xd8);
        e.mem.set_u32(renderer_vtable + 0xd4, 0x0022_00d4);
        e.mem.set_u32(renderer_vtable + 0xc8, 0x0022_00c8);
        e.register_double(0x0022_00d4, move |_, _| description.into_ret());
        // The group is the renderer itself; it has no first target.
        e.register(0x0022_00c8, |_, a| {
            if a.len() == 1 {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, renderer_vtable);
        e.set_global(RENDERER, renderer);
        e.set_global(VIEW_LEFT, 0.25f32);
        e.set_global(VIEW_RIGHT, 0.75f32);
        e.set_global(VIEW_TOP, 0.25f32);
        e.set_global(VIEW_BOTTOM, 0.75f32);
        e.set_global(LETTERBOX, 1u8);
        for address in [BINK_NEXT_FRAME, SLEEP, ERROR, DRAW_SETUP, DRAW_QUAD] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(BINK_WAIT, |_, _| Ret::default());
        e.register(COPY_FRAME, |_, _| 1u32.into_ret());
        let bink = e.mem.alloc(0x10);
        e.mem.set_u32(bink + 8, frames);
        let set = e.mem.alloc(0x44);
        e.mem.set_u32(set, device);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(movie, bink);
        e.mem.set_u32(movie + 4, device);
        e.mem.set_u32(movie + 8, set);
        (e, movie, device)
    }

    #[test]
    fn update_shows_a_frame_with_the_letterbox() {
        let (mut e, movie, device) = movie_engine(2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 0u32]).bool());
        assert_eq!(e.mem.u32(movie + 0xc), 1);
        assert_eq!(e.mem.u32(movie + 0x10), 1);
        assert_eq!(calls_to(&e, ERROR)[0][1..], [0, 1]);
        let clear = calls_to(&e, 0x0020_0000 + CLEAR);
        assert_eq!(
            clear,
            vec![vec![device, 0, 0, 1, 0xff00_0000, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(calls_to(&e, 0x0020_0000 + SET_VIEWPORT).len(), 2);
        assert_eq!(calls_to(&e, 0x0020_0000 + BEGIN_SCENE).len(), 1);
        assert_eq!(calls_to(&e, 0x0020_0000 + PRESENT).len(), 1);
        // The last frame: no frames remain.
        assert!(!e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 1u32]).bool());
        // ... unless kept going.
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 1u32, 1u32]).bool());
        // The settings refuse: false straight away.
        assert!(!e.call(0x00ec_1060, &args![movie, 0u32, 1u32, 1u32]).bool());
    }

    #[test]
    fn update_sleeps_on_a_frame_not_ready() {
        let (mut e, movie, _) = movie_engine(2);
        e.register(BINK_WAIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 0u32]).bool());
        assert_eq!(calls_to(&e, SLEEP), vec![vec![1]]);
        assert!(calls_to(&e, 0x0020_0000 + CLEAR).is_empty());
    }

    #[test]
    fn letterbox_uses_the_fractions() {
        let (mut e, _, _) = movie_engine(1);
        let viewport = e.mem.alloc(0x18);
        letterbox_viewport(&mut e, viewport);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(viewport + 4 * i)).collect();
        assert_eq!(words, [250, 125, 500, 250]);
        assert_eq!(e.mem.f32(viewport + 0x14), 1.0);
    }

    #[test]
    fn play_runs_updates_until_done() {
        let (mut e, movie, _) = movie_engine(2);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u64(THOUSAND, 1000.0f64.to_bits());
        e.register(MOVIE_OPEN, |_, _| 1u32.into_ret());
        e.register(MOVIE_CLOSE, |_, _| Ret::default());
        e.register(TICK_COUNT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let played = e
            .call(
                0x00ec_0e00,
                &args![movie, 0x55u32, 1u32, 2u32, 0u32, 3u32, 4u32, 1u32, 0u32, 0.0f32],
            )
            .bool();
        assert!(played);
        assert_eq!(calls_to(&e, MOVIE_OPEN), vec![vec![movie, 0x55, 2, 3, 4]]);
        assert_eq!(e.mem.u32(movie + 0xc), 2);
        assert_eq!(e.mem.u32(movie + 0x20), 1);
        assert_eq!(calls_to(&e, 0x0021_0004).len(), 1);
        assert_eq!(calls_to(&e, 0x0021_0008).len(), 1);
        assert!(calls_to(&e, 0x0021_000c).is_empty());
        assert_eq!(calls_to(&e, MOVIE_CLOSE), vec![vec![movie]]);
        // A failed open answers false and clears busy.
        e.register(MOVIE_OPEN, |_, _| Ret::default());
        e.mem.set_u8(movie + 0x24, 0);
        let played = e
            .call(
                0x00ec_0e00,
                &args![movie, 0x55u32, 1u32, 2u32, 0u32, 3u32, 4u32, 1u32, 0u32, 0.0f32],
            )
            .bool();
        assert!(!played);
        assert_eq!(e.mem.u8(movie + 0x24), 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_008670e0_constructs_and_hands_the_audio_flag_on() {
        let mut e = Engine::new();
        e.register(PLAYER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], PLAYER_VTABLE);
            Ret::default()
        });
        e.register(AUDIO_INSTANCE, |_, _| 0x0c00_0000u32.into_ret());
        // The byte comes back with garbage in the upper bytes of EAX.
        e.register(OBJECT_BYTE_AT_4, |_, _| 0x1234_5601u32.into_ret());
        e.register(PLAYER_SET_SOUND, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_70e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BGS_MOVIE_PLAYER_VTABLE);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                PLAYER_CONSTRUCT,
                AUDIO_INSTANCE,
                OBJECT_BYTE_AT_4,
                PLAYER_SET_SOUND
            ]
        );
        let query = log.iter().find(|(a, _)| *a == OBJECT_BYTE_AT_4).unwrap();
        assert_eq!(query.1, vec![0x0c00_0000]);
        let sound = log.iter().find(|(a, _)| *a == PLAYER_SET_SOUND).unwrap();
        assert_eq!(sound.1, vec![this, 1]);
    }
}

#[cfg(test)]
mod lookahead2_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
    }

    #[test]
    fn close_releases_what_is_there() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINK_CLOSE, TEXTURES_RELEASE, MOVIE_CLOSE_REST]);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(movie, 0x10);
        e.mem.set_u32(movie + 8, 0x20);
        e.call(0x00ec_1020, &args![movie]);
        assert_eq!(calls(&e, BINK_CLOSE), vec![vec![0x10]]);
        assert_eq!(calls(&e, TEXTURES_RELEASE), vec![vec![0x20]]);
        assert_eq!(calls(&e, MOVIE_CLOSE_REST), vec![vec![movie]]);
        let empty = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1020, &args![empty]);
        assert!(calls(&e, BINK_CLOSE).is_empty());
        assert!(calls(&e, TEXTURES_RELEASE).is_empty());
        assert_eq!(calls(&e, MOVIE_CLOSE_REST), vec![vec![empty]]);
    }

    #[test]
    fn frame_copy_copies_each_rectangle() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINK_DO_FRAME, TEXTURES_STEP]);
        let mut left = 2u32;
        e.register_double(TEXTURES_NEXT_RECT, move |e, a| {
            if left == 0 {
                return 0u32.into_ret();
            }
            for (index, &out) in a[1..7].iter().enumerate() {
                e.mem.set_u32(out, 0x100 * left + index as u32);
            }
            left -= 1;
            1u32.into_ret()
        });
        // The first copy answers 0, the second 5.
        let mut answers = vec![5u32, 0];
        e.register_double(BINK_COPY_TO_BUFFER_RECT, move |_, _| {
            answers.pop().unwrap().into_ret()
        });
        let bink = e.mem.alloc(8);
        e.mem.set_u32(bink + 4, 0x77);
        assert!(e.call(0x00ec_14d0, &args![bink, 0x900u32, 1u32]).bool());
        let copies = calls(&e, BINK_COPY_TO_BUFFER_RECT);
        assert_eq!(copies.len(), 2);
        // The rectangle words in the order the helper is given their addresses.
        assert_eq!(
            copies[0],
            vec![
                bink,
                0x200,
                0x201,
                0x77,
                0,
                0,
                0x202,
                0x203,
                0x204,
                0x205,
                0x8000_0003
            ]
        );
        assert_eq!(calls(&e, TEXTURES_STEP), vec![vec![0x900]; 2]);
        assert_eq!(calls(&e, BINK_DO_FRAME), vec![vec![bink]]);
    }

    #[test]
    fn frame_copy_answers_the_last_copy() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINK_DO_FRAME, TEXTURES_STEP]);
        let mut left = 1u32;
        e.register_double(TEXTURES_NEXT_RECT, move |_, _| {
            let more = left;
            left = 0;
            more.into_ret()
        });
        e.register(BINK_COPY_TO_BUFFER_RECT, |_, _| 0u32.into_ret());
        let bink = e.mem.alloc(8);
        assert!(!e.call(0x00ec_14d0, &args![bink, 0x900u32, 0u32]).bool());
        assert_eq!(calls(&e, BINK_COPY_TO_BUFFER_RECT)[0][10], 3);
        // No rectangle at all: true.
        e.register(TEXTURES_NEXT_RECT, |_, _| 0u32.into_ret());
        assert!(e.call(0x00ec_14d0, &args![bink, 0x900u32, 0u32]).bool());
    }

    #[test]
    fn sound_system_is_set_only_with_the_flag() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINK_SET_SOUND_SYSTEM]);
        e.map(BINK_OPEN_DIRECT_SOUND, 4);
        e.mem.set_u32(BINK_OPEN_DIRECT_SOUND, 0x0123_4560);
        e.call(0x00ec_1680, &args![0u32, 0x100u32]);
        assert!(calls(&e, BINK_SET_SOUND_SYSTEM).is_empty());
        e.call(0x00ec_1680, &args![0u32, 1u32]);
        assert_eq!(calls(&e, BINK_SET_SOUND_SYSTEM), vec![vec![0x0123_4560, 0]]);
    }

    #[test]
    fn continue_playing_pumps_messages_then_reads_the_stop_byte() {
        let mut e = Engine::new();
        quiet(&mut e, &[TRANSLATE_MESSAGE, DISPATCH_MESSAGE]);
        let mut waiting = 2u32;
        e.register_double(PEEK_MESSAGE, move |_, _| {
            let more = u32::from(waiting > 0);
            waiting = waiting.saturating_sub(1);
            more.into_ret()
        });
        let this = e.mem.alloc(0x28);
        assert!(e.call(0x00ec_1840, &args![this, 9u32]).bool());
        let peeks = calls(&e, PEEK_MESSAGE);
        assert_eq!(peeks.len(), 3);
        assert_eq!(peeks[0][1..], [0, 0, 0, 1]);
        assert_eq!(calls(&e, TRANSLATE_MESSAGE), vec![vec![peeks[0][0]]; 2]);
        assert_eq!(calls(&e, DISPATCH_MESSAGE), vec![vec![peeks[0][0]]; 2]);
        e.mem.set_u8(this + 0x21, 1);
        assert!(!e.call(0x00ec_1840, &args![this, 0u32]).bool());
    }

    #[test]
    fn constructor_sets_the_fields_and_the_semaphore() {
        let mut e = Engine::new();
        e.register(CREATE_SEMAPHORE, |_, _| 0x4444u32.into_ret());
        e.register(NAME_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x58);
        for offset in (0..0x58).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        assert_eq!(e.call(0x00ec_20c0, &args![this]).u32(), this);
        assert_eq!(calls(&e, CREATE_SEMAPHORE), vec![vec![0, 0, 1, 0]]);
        assert_eq!(calls(&e, NAME_MAP_CONSTRUCT), vec![vec![this + 0x40, 0x25]]);
        let words: Vec<u32> = (0..0x40u32)
            .step_by(4)
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(
            words,
            vec![
                PLAYER_VTABLE,
                0,
                0,
                0,
                0,
                0,
                0xaaaa_0000,
                0,
                0x0100_0000,
                0xaaaa_aa01,
                0xaaaa_aaaa,
                0,
                0,
                0x4444,
                1,
                0xaaaa_aaaa
            ]
        );
        assert_eq!(e.mem.u32(this + 0x40), NAME_MAP_VTABLE);
        assert_eq!(e.mem.u8(this + 0x50), 1);
    }

    #[test]
    fn clean_up_frees_each_value_from_the_first_bucket() {
        let mut e = Engine::new();
        quiet(&mut e, &[MEMORY_DEALLOCATE, MAP_BASE_DESTRUCT]);
        let mut values = vec![0x700u32, 0x600];
        e.register_double(MAP_NEXT_ITEM, move |e, a| {
            assert_eq!((e.mem.u32(a[2]), e.mem.u32(a[3])), (0, 0));
            let value = values.pop().unwrap();
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[2], 0x55);
            let next = if values.is_empty() { 0 } else { 0x99 };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let this = e.mem.alloc(0x58);
        let buckets = e.mem.alloc(0xc);
        e.mem.set_u32(buckets + 8, 0x9000);
        e.mem.set_u32(this + 0x44, 3);
        e.mem.set_u32(this + 0x48, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00ec_2e50, &args![this]);
        let steps = calls(&e, MAP_NEXT_ITEM);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0][0], this + 0x40);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x600], vec![MEMORY_MANAGER, 0x700]]
        );
        assert_eq!(calls(&e, MAP_BASE_DESTRUCT), vec![vec![this + 0x40]]);
        // An empty map: only the destructor body.
        e.mem.set_u32(buckets + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_2e50, &args![this]);
        assert!(calls(&e, MAP_NEXT_ITEM).is_empty());
        assert_eq!(calls(&e, MAP_BASE_DESTRUCT).len(), 1);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A texture whose vtable at `0x0300_0000` has 0x20 entries, each its own address.
    fn texture(e: &mut Engine) -> u32 {
        let slots: Vec<u32> = (0..0x20).map(|i| 0x0300_1000 + 4 * i).collect();
        e.put_vtable(0x0300_0000, &slots);
        let t = e.mem.alloc(8);
        e.mem.set_u32(t, 0x0300_0000);
        t
    }

    #[test]
    fn texture_set_release_releases_each_texture_and_frees_the_set() {
        let mut e = Engine::new();
        let t = texture(&mut e);
        e.register(0x0300_1008, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let set = e.mem.alloc(0x44);
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, t);
        e.mem.set_u32(array + 8, t);
        e.mem.set_u32(set + 0x20, 3);
        e.mem.set_u32(set + 0x40, array);
        e.call_log = Some(vec![]);
        e.call(0x00eb_fd20, &args![set]);
        assert_eq!(calls_to(&e, 0x0300_1008), [vec![t], vec![t]]);
        assert_eq!(e.mem.u32(set + 0x40), 0);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [vec![MEMORY_MANAGER, set]]);
        // No array: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00eb_fd20, &args![set]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
    }

    fn next_tile(e: &mut Engine, set: u32, out: u32) -> u32 {
        e.call(
            0x00eb_fda0,
            &args![
                set,
                out,
                out + 4,
                out + 8,
                out + 0xc,
                out + 0x10,
                out + 0x14
            ],
        )
        .u32()
    }

    fn words(e: &Engine, out: u32) -> Vec<u32> {
        (0..6).map(|i| e.mem.u32(out + 4 * i)).collect()
    }

    #[test]
    fn next_tile_hands_out_the_rectangle_and_advances() {
        let mut e = Engine::new();
        let t = texture(&mut e);
        e.register(0x0300_104c, |e, w| {
            e.mem.set_u32(w[2], 0x400);
            e.mem.set_u32(w[2] + 4, 0x0700_0000);
            Ret::default()
        });
        let set = e.mem.alloc(0x44);
        let array = e.mem.alloc(0x20);
        for i in 0..6 {
            e.mem.set_u32(array + 4 * i, t);
        }
        // Two full columns and a partial one (16 wide), one full row and a partial one (8).
        e.mem.set_u32(set + 0x20, 6);
        e.mem.set_u32(set + 0x24, 2);
        e.mem.set_u32(set + 0x28, 1);
        e.mem.set_u32(set + 0x2c, 16);
        e.mem.set_u32(set + 0x30, 8);
        e.mem.set_u32(set + 0x3c, 64);
        e.mem.set_u32(set + 0x40, array);
        let out = e.mem.alloc(0x18);
        assert_eq!(next_tile(&mut e, set, out), 1);
        assert_eq!(words(&e, out), [0x0700_0000, 0x400, 0, 0, 64, 64]);
        e.mem.set_u32(set + 0x14, 5);
        assert_eq!(next_tile(&mut e, set, out), 1);
        assert_eq!(words(&e, out), [0x0700_0000, 0x400, 128, 64, 16, 8]);
        assert_eq!(e.mem.u32(set + 0x14), 6);
        // Past the end: the cursor goes back to 0.
        assert_eq!(next_tile(&mut e, set, out), 0);
        assert_eq!(e.mem.u32(set + 0x14), 0);
        // Null addresses are skipped; a failed lock answers 0 and resets too.
        e.mem.set_u32(set + 0x14, 3);
        let r = e.call(0x00eb_fda0, &args![set, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), 1);
        e.register(0x0300_104c, |_, _| Ret {
            eax: 0x8876_0000,
            ..Ret::default()
        });
        assert_eq!(next_tile(&mut e, set, out), 0);
        assert_eq!(e.mem.u32(set + 0x14), 0);
    }

    #[test]
    fn step_unlocks_the_texture_before_the_cursor() {
        let mut e = Engine::new();
        let t = texture(&mut e);
        e.register(0x0300_1050, |_, _| Ret::default());
        let set = e.mem.alloc(0x44);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, t);
        e.mem.set_u32(set + 0x14, 2);
        e.mem.set_u32(set + 0x40, array);
        e.call_log = Some(vec![]);
        e.call(0x00eb_ff00, &args![set]);
        assert_eq!(calls_to(&e, 0x0300_1050), [vec![t, 0]]);
    }

    #[test]
    fn reset_clears_the_movie() {
        let mut e = Engine::new();
        let movie = e.mem.alloc(0x28);
        for off in (0..0x28).step_by(4) {
            e.mem.set_u32(movie + off, 0xffff_ffff);
        }
        e.call(0x00ec_1570, &args![movie]);
        for off in [0u32, 4, 8, 0xc, 0x10, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(movie + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.f32(movie + 0x14), 1.0);
        assert_eq!(e.mem.u8(movie + 0x24), 0);
        assert_eq!(e.mem.u8(movie + 0x25), 0xff);
    }

    #[test]
    fn map_items_come_from_and_go_to_the_memory_manager() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |_, _| Ret {
            eax: 0x4444,
            ..Ret::default()
        });
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_1a40, &args![0x1000u32]).u32(), 0x4444);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), [vec![MEMORY_MANAGER, 0xc]]);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, 9);
        e.call(0x00ec_1a60, &args![0x1000u32, item]);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [vec![MEMORY_MANAGER, item]]
        );
    }

    #[test]
    fn key_hash_is_case_sensitive_and_signed() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 4, 37);
        let key = e.mem.alloc(8);
        e.mem.set_cstr(key, b"Ab\xe9");
        // 0xE9 is -0x17 sign-extended.
        let expected = ((u32::from(b'A') * 33 + u32::from(b'b')) * 33).wrapping_sub(0x17) % 37;
        let r = e.call(0x00ec_1c30, &args![map, key]).u32();
        assert_eq!(r, expected);
        e.mem.set_cstr(key, b"");
        assert_eq!(e.call(0x00ec_1c30, &args![map, key]).u32(), 0);
    }

    #[test]
    fn set_value_copies_the_key_when_the_map_owns_keys() {
        let mut e = Engine::new();
        e.register(STRLEN, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register(NI_ALLOC, |_, _| Ret {
            eax: 0x5550,
            ..Ret::default()
        });
        e.register(STRCPY_S, |_, _| Ret::default());
        let map = e.mem.alloc(0x14);
        let item = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1cc0, &args![map, item, 0x7000u32, 0x8000u32]);
        assert_eq!(e.mem.u32(item + 4), 0x7000);
        assert_eq!(e.mem.u32(item + 8), 0x8000);
        assert!(calls_to(&e, STRLEN).is_empty());
        e.mem.set_u8(map + 0x10, 1);
        e.call(0x00ec_1cc0, &args![map, item, 0x7000u32, 0x9000u32]);
        assert_eq!(calls_to(&e, NI_ALLOC), [vec![6]]);
        assert_eq!(calls_to(&e, STRCPY_S), [vec![0x5550, 6, 0x7000]]);
        assert_eq!(e.mem.u32(item + 4), 0x5550);
        assert_eq!(e.mem.u32(item + 8), 0x9000);
    }

    #[test]
    fn keys_equal_through_stricmp() {
        let mut e = Engine::new();
        e.register(STRICMP, |_, w| Ret {
            eax: u32::from(w[0] != w[1]).wrapping_neg(),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_1dc0, &args![0u32, 5u32, 5u32]).u32(), 1);
        assert_eq!(e.call(0x00ec_1dc0, &args![0u32, 5u32, 6u32]).u32(), 0);
        assert_eq!(calls_to(&e, STRICMP)[1], [5, 6]);
    }

    fn quiet_destructors(e: &mut Engine) {
        for a in [MAP_BASE_DESTRUCT, NI_FREE, MEMORY_DEALLOCATE] {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn map_deleting_destructors_free_on_bit_zero() {
        let mut e = Engine::new();
        quiet_destructors(&mut e);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x6000);
        for (addr, bodies) in [(0x00ec_1df0u32, 1usize), (0x00ec_1e40, 2)] {
            for flags in [0u32, 1] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![map, flags]).u32(), map);
                assert_eq!(e.mem.u32(map), MAP_ROOT_VTABLE);
                assert_eq!(calls_to(&e, MAP_BASE_DESTRUCT).len(), bodies);
                assert_eq!(calls_to(&e, NI_FREE), [vec![0x6000]]);
                assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), flags as usize);
            }
        }
    }

    #[test]
    fn string_map_deleting_destructors_destroy_then_free() {
        let mut e = Engine::new();
        quiet_destructors(&mut e);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x6000);
        for addr in [0x00ec_1ee0u32, 0x00ec_2ff0] {
            for flags in [0u32, 3] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![map, flags]).u32(), map);
                assert_eq!(e.mem.u32(map), MAP_ROOT_VTABLE);
                assert_eq!(calls_to(&e, NI_FREE), [vec![0x6000]]);
                let freed = calls_to(&e, MEMORY_DEALLOCATE);
                assert_eq!(freed.len(), (flags & 1) as usize);
            }
        }
    }
}

#[cfg(test)]
mod sequence_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn start_sequence_starts_the_thread_and_waits_for_it() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(DEFAULT_PRIORITY, 3);
        for slot in [RESUME_THREAD, WAIT_FOR_SINGLE_OBJECT, SET_THREAD_NAME] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(CREATE_THREAD, |e, a| {
            e.mem.set_u32(a[5], 0x44);
            0x77u32.into_ret()
        });
        e.register(SET_THREAD_PRIORITY, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 5u32.into_ret());
        let this = e.mem.alloc(0x58);
        e.mem.set_u32(this + 0x34, 0x99);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0x1234u32,
            1u32,
            0xffff_ffffu32,
            0u32,
            1u32,
            0x7f80_0001u32
        ];
        assert_eq!(e.call(0x00ec_16d0, &words).u32(), 5, "the decrement's EAX");
        let created = calls(&e, CREATE_THREAD);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..5], [0, 0, SEQUENCE_THREAD_PROC, 0x1234, 4]);
        assert_eq!(
            calls(&e, SET_THREAD_NAME),
            vec![vec![0x44, SEQUENCE_THREAD_NAME]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x77);
        let bytes = [0x22, 0x23, 0x24].map(|o| e.mem.u8(this + o));
        assert_eq!(bytes, [1, 0, 1]);
        assert_eq!(e.mem.u32(this + 0x28), 0x7fc0_0001, "stored quiet");
        // -1: the default setting 3, ABOVE_NORMAL.
        assert_eq!(calls(&e, SET_THREAD_PRIORITY), vec![vec![0x77, 1]]);
        assert_eq!(e.mem.u32(this + 0x2c), 3);
        assert_eq!(calls(&e, RESUME_THREAD), vec![vec![0x77]]);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x99, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![this + 0x30]]);
    }

    #[test]
    fn sequence_priorities_follow_the_jump_table() {
        let mut e = Engine::new();
        e.register(SET_THREAD_PRIORITY, |_, a| a[1].into_ret());
        let this = e.mem.alloc(0x58);
        for (setting, priority) in [
            (0u32, 0xffff_fffeu32),
            (1, 0xffff_ffff),
            (2, 0),
            (3, 1),
            (4, 2),
            (5, 0xf),
            (6, 0),
        ] {
            assert_eq!(fn_00ec18a0(&mut e, Ptr::new(this), setting), priority);
            assert_eq!(e.mem.u32(this + 0x2c), setting);
        }
    }

    #[test]
    fn the_sequence_thread_runs_the_settings_object() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(MOVIE_SETTINGS, 0x5000);
        e.register(SEQUENCE_RUN, |_, _| 9u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_16b0, &args![0x1234u32]).u32(), 0);
        assert_eq!(calls(&e, SEQUENCE_RUN), vec![vec![0x5000, 0x1234]]);
    }
}

#[cfg(test)]
mod finish_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_movie_thread_starts_named_and_is_waited_for() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CREATE_THREAD, |e, a| {
            e.mem.set_u32(a[5], 0x77);
            0x55u32.into_ret()
        });
        for slot in [SET_THREAD_NAME, WAIT_FOR_SINGLE_OBJECT] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(RESUME_THREAD, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x30);
        let p = Ptr::new(this);
        e.call(0x00ec_19a0, &args![this]);
        assert!(calls(&e, WAIT_FOR_SINGLE_OBJECT).is_empty());
        assert_eq!(e.call(0x00ec_1940, &args![this, 9u32, 0x1234u32]).u32(), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0x1234);
        assert_eq!(e.mem.u32(this + 4), 0x55);
        let created = calls(&e, CREATE_THREAD);
        assert_eq!(created[0][..5], [0, 0, MOVIE_THREAD_PROC, 9, 4]);
        assert_eq!(
            calls(&e, SET_THREAD_NAME),
            vec![vec![0x77, MOVIE_THREAD_NAME]]
        );
        assert_eq!(calls(&e, RESUME_THREAD), vec![vec![0x55]]);
        fn_00ec19a0(&mut e, p);
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x55, 0xffff_ffff]]
        );
    }

    #[test]
    fn the_map_destructors_restore_their_vtables() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let vtables = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = vtables.clone();
        e.register_double(MAP_BASE_DESTRUCT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0x99);
        e.call(0x00ec_1b80, &args![this]);
        assert_eq!(*vtables.borrow(), vec![MAP_ROOT_VTABLE]);
        e.call(0x00ec_1bb0, &args![this]);
        assert_eq!(
            *vtables.borrow(),
            vec![MAP_ROOT_VTABLE, MAP_BASE_VTABLE, MAP_ROOT_VTABLE]
        );
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x99], vec![0x99]]);
        // The name map: its vtable, then the string map's destructor (keys not owned).
        fn_00ec1d30(&mut e, Ptr::new(this));
        assert_eq!(vtables.borrow().len(), 5);
        assert_eq!(e.mem.u32(this), MAP_ROOT_VTABLE);
    }

    #[test]
    fn the_player_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // The real destructor body (00ec15e0) runs; its callees are doubled.
        for slot in [
            MEMORY_DEALLOCATE,
            PLAYER_CLEAN_UP,
            MAP_BASE_DESTRUCT,
            NI_FREE,
            CLOSE_HANDLE,
        ] {
            e.register(slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00ec_19d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), PLAYER_VTABLE);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00ec_19d0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }
}

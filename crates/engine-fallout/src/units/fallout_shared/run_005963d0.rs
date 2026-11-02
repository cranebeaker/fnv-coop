//! `fallout shared/run_005963d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00598040 (decompiled, FalloutNV.exe 1.4.0.525)
/// A plain getter: the `float` at `+0x3c` of `this`, returned in `ST0`. The
/// engine map has no name for it; the translations that call it
/// (`animation.rs`, `inventorychanges.rs`) use it on a sequence object, which
/// this body does not confirm.
pub fn fn_00598040(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c)
}

// Translated from 00597610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00597610 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6f (0xc4 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_42ec`) is `MediaSet`, so this is the constructor of `MediaSet`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It also clears the global at `011ca9fc`.
/// The exception-unwinding frame is not translated.
pub fn fn_00597610(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_42ec);
    e.mem.set_u32(base + 0x18, 0x0103_42d8);
    e.call(0x0040_2d00, &args![base + 0x38]);
    e.call(0x0059_77d0, &args![base + 0x48]);
    e.call(0x0059_77d0, &args![base + 0x58]);
    e.call(0x0059_77d0, &args![base + 0x68]);
    e.call(0x0059_77d0, &args![base + 0x78]);
    e.call(0x0059_77d0, &args![base + 0x88]);
    e.call(0x0059_77d0, &args![base + 0x98]);
    e.call(0x004f_15a0, &args![this, 0x6fu32]);
    e.mem.set_u32(base + 0x44, 0xffff_ffff);
    e.call(0x0059_7850, &args![base + 0x48]);
    e.call(0x0059_7850, &args![base + 0x58]);
    e.call(0x0059_7850, &args![base + 0x68]);
    e.call(0x0059_7850, &args![base + 0x78]);
    e.call(0x0059_7850, &args![base + 0x88]);
    e.call(0x0059_7850, &args![base + 0x98]);
    e.mem.set_f32(base + 0xac, 0.0);
    e.mem.set_f32(base + 0xb0, 0.0);
    e.mem.set_f32(base + 0xb4, 0.0);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u32(base + 0xbc, 0x0);
    e.mem.set_u32(base + 0xc0, 0x0);
    e.mem.set_u32(0x011c_a9fc, 0x0);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.mem.set_u32(base + 0x28, 0x0);
    e.mem.set_u8(base + 0x30, 0x0);
    e.mem.set_u8(base + 0x31, 0x0);
    e.mem.set_u8(base + 0x33, 0x0);
    e.mem.set_u8(base + 0x34, 0x0);
    this
}

// Translated from 00597880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005978b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00597880(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_78b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// Empties a list (`00470470`, on the list) and adds an item to one
/// (`005ae3d0`, given the item's address).
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_ADD: u32 = 0x005a_e3d0;
/// The six form lists of the location controller (+0x88 .. +0xB0).
const CONTROLLER_LISTS: [u32; 6] = [0x88, 0x90, 0x98, 0xa0, 0xa8, 0xb0];
/// `BSStringT` set (`004037f0`: text, length).
const STRING_SET: u32 = 0x0040_37f0;

/// The audio media helpers on channel 2: whether media is playing
/// (`FalloutAudioMedia::MediaIsPlaying` in the engine map, `00830750`,
/// cdecl), its length (`00830970`), its position (`00830a20`), the fade
/// out (`00830680`, cdecl: milliseconds, then the media type 2), the start (`008300c0`,
/// cdecl, seven words) and the update after it (`008304c0`).
const MEDIA_IS_PLAYING: u32 = 0x0083_0750;
const MEDIA_LENGTH: u32 = 0x0083_0970;
const MEDIA_POSITION: u32 = 0x0083_0a20;
const MEDIA_FADE_OUT: u32 = 0x0083_0680;
const MEDIA_START: u32 = 0x0083_00c0;
const MEDIA_UPDATE: u32 = 0x0083_04c0;
/// `_ftol2_sse` (`00ec62c0`): the value in ST0 truncated (an `f64`
/// argument).
const FTOL: u32 = 0x00ec_62c0;
/// `1000.0` (`double`, milliseconds per second), `100.0` (`double`) and
/// `1000.0` (`float`), as the exe holds them.
const THOUSAND: u32 = 0x0101_7b70;
const HUNDRED: u32 = 0x0101_7a40;
const THOUSAND_FLOAT: u32 = 0x0101_3974;
/// The time (milliseconds) before which the music type does not start a
/// new track.
const NEXT_TRACK_TIME: u32 = 0x011d_d37c;
/// The time `00597990`'s mode 3 compares with.
const MEDIA_SET_TIME: u32 = 0x011c_a9fc;

// Translated from 005963d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005963d0 - the /GS stack cookie check (00ec408c) is not translated.
/// The media location controller's record reader (`RET 4`: the
/// `TESFile`; the engine map has no name; the form type is not checked):
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID`, `OBND`,
/// `FULL` (+0x18), `NAM1`..`NAM7` (words at +0x6C, +0x70, +0x74, +0x7C,
/// +0x80, +0x84, +0x78), `FNAM`/`RNAM` (words at +0x64/+0x60) and
/// `HNAM`/`ZNAM`/`XNAM`/`YNAM`/`LNAM`/`GNAM` (a form id added to the list
/// at +0x88/+0x90/+0x98/+0xA0/+0xA8/+0xB0; `WNAM` is skipped). The form
/// id local starts at 0 and keeps the last value read. Returns true.
pub fn fn_005963d0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let id = e.mem.alloc(4);
    let copy = e.mem.alloc(4);
    e.mem.set_u32(id, 0);
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        // The `xNAM` chunks from `FNAM` on go through the exe's byte map
        // (005964d1: type - `FNAM`, map at 00596778, table at 00596754):
        // `ZNAM` adds to +0x90 (00596651), `XNAM` to +0x98 (0059667a),
        // `YNAM` to +0xA0 (005966a3); `WNAM` maps to the default
        // (00596728) and is skipped.
        let list = match chunk {
            0x4d41_4e48 => Some(0x88u32), // HNAM
            0x4d41_4e5a => Some(0x90),    // ZNAM
            0x4d41_4e58 => Some(0x98),    // XNAM
            0x4d41_4e59 => Some(0xa0),    // YNAM
            0x4d41_4e4c => Some(0xa8),    // LNAM
            0x4d41_4e47 => Some(0xb0),    // GNAM
            _ => None,
        };
        if let Some(list) = list {
            e.call(FILE_CHUNK_DATA, &args![file, id]);
            e.mem.set_u32(copy, e.mem.u32(id));
            e.call(LIST_ADD, &args![base + list, copy]);
        } else {
            match chunk {
                // FULL
                0x4c4c_5546 => {
                    let full = if this.is_null() { 0 } else { base + 0x18 };
                    e.call(0x0048_7050, &args![full, file]);
                }
                // NAM1 .. NAM7
                0x314d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x6c]);
                }
                0x324d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x70]);
                }
                0x334d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x74]);
                }
                0x344d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x7c]);
                }
                0x354d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x80]);
                }
                0x364d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x84]);
                }
                0x374d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x78]);
                }
                // EDID
                0x4449_4445 => {
                    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                    // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                        e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                        e.vcall(base, 0x134, &args![text]);
                    });
                }
                // OBND
                0x444e_424f => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                // RNAM
                0x4d41_4e52 => {
                    e.call(FILE_CHUNK_DATA, &args![file, id]);
                    e.mem.set_u32(base + 0x60, e.mem.u32(id));
                }
                // FNAM
                0x4d41_4e46 => {
                    e.call(FILE_CHUNK_DATA, &args![file, id]);
                    e.mem.set_u32(base + 0x64, e.mem.u32(id));
                }
                _ => {}
            }
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.mem.free(copy);
    e.mem.free(id);
    true
}

// Translated from 005974f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media location controller's defaults: three floats from the exe's
/// constants at +0x70, +0x74 and +0x7C (FLD/FSTP), 0x40 at +0x80, 0x150 at
/// +0x84, its six lists emptied, then 0, 1 and 0 at +0x60, +0x64 and
/// +0x68.
pub fn fn_005974f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, constant) in [
        (0x70u32, 0x0102_6994u32),
        (0x74, 0x0103_40a4),
        (0x7c, 0x0103_4294),
    ] {
        let value = e.global::<f32>(constant);
        e.mem.set_f32(base + offset, x87_quiet(value));
    }
    e.mem.set_u32(base + 0x80, 0x40);
    e.mem.set_u32(base + 0x84, 0x150);
    fn_005975b0(e, this);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 1);
    e.mem.set_u32(base + 0x68, 0);
}

// Translated from 005975b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the media location controller's six lists (+0x88 .. +0xB0).
pub fn fn_005975b0(e: &mut Engine, this: Ptr) {
    for list in CONTROLLER_LISTS {
        e.call(LIST_CLEAR, &args![this.addr() + list]);
    }
}

// Translated from 005977d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005977d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of a media set's 0x10-byte track (`00597610` builds six):
/// the name string (`004037b0`, then set empty), the `float` at `01034424`
/// at +8 and the one at `01016410` at +0xC. Returns `this`.
pub fn fn_005977d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_37b0, &args![this]);
    e.call(STRING_SET, &args![this, 0u32, 0u32]);
    let value = e.global::<f32>(0x0103_4424);
    e.mem.set_f32(base + 8, x87_quiet(value));
    let value = e.global::<f32>(0x0101_6410);
    e.mem.set_f32(base + 0xc, x87_quiet(value));
    this
}

// Translated from 00597850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a media set's track: the name set empty, the `float` at
/// `0103395c` at +8 and the one at `01016410` at +0xC.
pub fn fn_00597850(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(STRING_SET, &args![this, 0u32, 0u32]);
    let value = e.global::<f32>(0x0103_395c);
    e.mem.set_f32(base + 8, x87_quiet(value));
    let value = e.global::<f32>(0x0101_6410);
    e.mem.set_f32(base + 0xc, x87_quiet(value));
}

// Translated from 005978b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005978b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `MediaSet` (the vtables `00597610` stores): puts
/// back the vtables, destroys the six tracks from the last (`007d9ec0` on
/// +0x98 .. +0x48), the strings at +0x38 and +0x18 (`00591300`) and the
/// `TESForm` base (`00483630`).
pub fn fn_005978b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_42ec);
    e.mem.set_u32(base + 0x18, 0x0103_42d8);
    for track in [0x98u32, 0x88, 0x78, 0x68, 0x58, 0x48] {
        e.call(0x007d_9ec0, &args![base + track]);
    }
    e.call(0x0059_1300, &args![base + 0x38]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00597990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the media set for `time` (`RET 0xC`: time, a flag byte, the
/// location object): with an object, by the set's kind at +0x44: 0
/// `00598550(flag, time, 4)`, 1 `00597a60`, 2 `00598060` (both with the
/// three words), any other false. Without one: kind 3 answers true while
/// `011ca9fc` is past `time` and otherwise runs `00598890(time)` and
/// answers false; kind 0 `00598550(flag, time, 4)`; others false.
pub fn fn_00597990(e: &mut Engine, this: Ptr, time: u32, flag: u8, location: u32) -> bool {
    let kind = e.mem.u32(this.addr() + 0x44);
    if location != 0 {
        match kind {
            0 => e
                .call(0x0059_8550, &args![this, u32::from(flag), time, 4u32])
                .bool(),
            1 => fn_00597a60(e, this, time, flag, location),
            2 => e
                .call(0x0059_8060, &args![this, time, u32::from(flag), location])
                .bool(),
            _ => false,
        }
    } else if kind == 3 {
        if e.global::<u32>(MEDIA_SET_TIME) > time {
            return true;
        }
        e.call(0x0059_8890, &args![this, time]);
        false
    } else if kind == 0 {
        e.call(0x0059_8550, &args![this, u32::from(flag), time, 4u32])
            .bool()
    } else {
        false
    }
}

/// `FISTP qword` with the rounding control set to chop: the low word of
/// the truncated value, of the integer indefinite (0) for NaN and values
/// out of range.
fn fistp_chop_low(value: f64) -> u32 {
    if value.is_nan()
        || !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// One of `00597a60`'s threshold tests: with `bit` set in the flags byte at
/// +0xA8, the `float` at `offset` above `level` and below `limit`, the
/// track `choice` is picked and `limit` lowered to it (FLD/FSTP through a
/// local, the lower of the two by the exe's flag test).
fn pick_track(
    e: &mut Engine,
    this: u32,
    bit: u8,
    offset: u32,
    level: f64,
    limit: &mut u32,
    choice: &mut u8,
    pick: u8,
) {
    if e.mem.u8(this + 0xa8) & bit == 0 {
        return;
    }
    let threshold = x87_load32(e.mem.u32(this + offset));
    // FCOMPP; TEST AH,5; JP: on only when level < threshold.
    if !(level < threshold) {
        return;
    }
    let current = x87_load32(*limit);
    // TEST AH,0x41; JNZ: on only when current > threshold.
    if !(current > threshold) {
        return;
    }
    *choice = pick;
    // TEST AH,5; JP: the threshold unless current < threshold.
    let lower = if current < threshold {
        *limit
    } else {
        e.mem.u32(this + offset)
    };
    *limit = x87_quiet_bits(lower);
}

// Translated from 00597a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media set's music type run (`RET 0xC`: time in milliseconds, a flag
/// byte, an object (`location`; its getters `00598000`, `00598020`, `00598040` are in this unit); the engine map has no name). A set flag
/// answers false at once. Otherwise: when the current track has played its
/// length (`+0x2C` start plus channel 2's length, less the fade time at
/// +0xB4 in milliseconds, below `time`) it fades out and restarts the
/// clock, answering false unless the object's state (`00598000`) is 0.
/// Then the object's level (`00598040` times 100) picks the track: by the
/// object's byte at +0x37 either the thresholds at +0x54/+0x64/+0x74 (flag
/// bits 1/2/4) or +0x84/+0x94/+0xA4 (bits 8/0x10/0x20), each the lowest
/// threshold above the level. A changed state (not playing, the flag, the
/// object's byte or the track differing from +0x31/+0x33/+0x34) once the
/// time passed `011dd37c`, or a restart, starts the picked track
/// (+0x48/+0x58/+0x68 or +0x78/+0x88/+0x98) on channel 2 with its volume
/// (+8), the fade time and the offset into the playing media, sets the
/// next start (`time` plus +0xAC seconds) and the start time; without a
/// track it fades out what plays and answers false. The flag, the
/// object's byte and the track are kept. Answers true otherwise.
pub fn fn_00597a60(e: &mut Engine, this: Ptr, time: u32, flag: u8, location: u32) -> bool {
    let base = this.addr();
    let playing = e.call(MEDIA_IS_PLAYING, &args![2u32, 0u32]).u8();
    let mut result = true;
    let mut choice = 0u8;
    if flag != 0 {
        return false;
    }
    let thousand = e.global::<f64>(THOUSAND);
    let fade_millis = |e: &mut Engine| {
        let fade = x87_load32(e.mem.u32(base + 0xb4));
        x87_mul(fade, thousand)
    };
    let length = e.call(MEDIA_LENGTH, &args![2u32]).u32();
    let mut restarted = false;
    let start = e.mem.u32(base + 0x2c);
    if start != 0 && length != 0 {
        let end = f64::from(start.wrapping_add(length));
        let fade = fade_millis(e);
        let left = x87_sub(end, fade);
        // FCOMPP; TEST AH,5; JP: on only when left < time.
        if left < f64::from(time) {
            let fade = fade_millis(e);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE_OUT, &args![millis, 2u32]);
            e.mem.set_u32(base + 0x2c, time);
            if fn_00598000(e, Ptr::new(location)) != 0 {
                return false;
            }
            restarted = true;
        }
    }
    let level = x87_load32(fn_00598040(e, Ptr::new(location)).to_bits());
    let hundred = e.global::<f64>(HUNDRED);
    let level = x87_load32(x87_store32(x87_mul(level, hundred)));
    let mut limit = x87_quiet_bits(e.global::<u32>(THOUSAND_FLOAT));
    let first_set = fn_00598020(e, Ptr::new(location)) != 0;
    let tests: [(u8, u32); 3] = if first_set {
        [(1, 0x54), (2, 0x64), (4, 0x74)]
    } else {
        [(8, 0x84), (0x10, 0x94), (0x20, 0xa4)]
    };
    for (pick, (bit, offset)) in (1u8..).zip(tests) {
        pick_track(e, base, bit, offset, level, &mut limit, &mut choice, pick);
    }
    let unchanged = playing != 0
        && e.mem.u8(base + 0x31) == flag
        && e.mem.u8(base + 0x33) == fn_00598020(e, Ptr::new(location))
        && e.mem.i8(base + 0x34) == choice as i8;
    let due = !unchanged && e.global::<u32>(NEXT_TRACK_TIME) < time;
    if !due && !restarted {
        return result;
    }
    let tracks: [u32; 3] = if fn_00598020(e, Ptr::new(location)) != 0 {
        [0x48, 0x58, 0x68]
    } else {
        [0x78, 0x88, 0x98]
    };
    let track = match choice {
        1 => base + tracks[0],
        2 => base + tracks[1],
        3 => base + tracks[2],
        _ => 0,
    };
    if track == 0 {
        if playing != 0 {
            let fade = fade_millis(e);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE_OUT, &args![millis, 2u32]);
        }
        result = false;
    } else {
        let delay = x87_load32(e.mem.u32(base + 0xac));
        let next = x87_add(f64::from(time), x87_mul(delay, thousand));
        e.set_global(NEXT_TRACK_TIME, fistp_chop_low(next));
        let mut offset = 0u32;
        if e.call(MEDIA_POSITION, &args![2u32]).u32() != 0 && !restarted {
            let position = e.call(MEDIA_POSITION, &args![2u32]).u32();
            offset = time.wrapping_sub(position);
        }
        let volume = x87_quiet_bits(e.mem.u32(track + 8));
        let fresh = u32::from(fn_00598000(e, Ptr::new(location)) == 0);
        let fade = fistp_chop_low(fade_millis(e));
        let name = e.call(0x0055_9450, &args![track]).u32();
        e.call(
            MEDIA_START,
            &args![2u32, name, fade, fresh, 1u32, volume, offset],
        );
        e.call(MEDIA_UPDATE, &args![]);
        e.mem.set_u32(base + 0x2c, time);
    }
    e.mem.set_u8(base + 0x31, flag);
    let byte = fn_00598020(e, Ptr::new(location));
    e.mem.set_u8(base + 0x33, byte);
    e.mem.set_u8(base + 0x34, choice);
    result
}

// Translated from 00598000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bits 4 and 5 of the word at +0x6C (`SAR 4`, then `AND 3`).
pub fn fn_00598000(e: &mut Engine, this: Ptr) -> u32 {
    ((e.mem.i32(this.addr() + 0x6c) >> 4) & 3) as u32
}

// Translated from 00598020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x37.
pub fn fn_00598020(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x37)
}

/// The list helpers of the controller's form lists: the item slot of a
/// node (`006815c0`, its address), the next node (`00726070`), whether the
/// list is empty (`008256d0`) and its count (`005ae380`).
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_COUNT: u32 = 0x005a_e380;
/// The form id of a form (`0084e3a0`, thiscall).
const FORM_ID: u32 = 0x0084_e3a0;
/// Writes a 4-byte chunk (`00485910`, cdecl: the chunk type, the word).
const WRITE_CHUNK: u32 = 0x0048_5910;
/// `__RTDynamicCast` (`00ec43fb`, cdecl) and the two type descriptors
/// these functions pass to it (source and target).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const CAST_TARGET: u32 = 0x0118_418c;
/// The six form lists with the chunk each is saved as by `00596790`.
const SAVED_LISTS: [(u32, u32); 6] = [
    (0x88, 0x4d41_4e48), // HNAM
    (0x90, 0x4d41_4e5a), // ZNAM
    (0x98, 0x4d41_4e58), // XNAM
    (0xa0, 0x4d41_4e59), // YNAM
    (0xa8, 0x4d41_4e4c), // LNAM
    (0xb0, 0x4d41_4e47), // GNAM
];
/// The audio media helpers: the fade out (`00830680`, cdecl: milliseconds,
/// channel), the length of what a channel plays (`008308c0`, cdecl), pause
/// and unpause (`008305e0`, `00830610`).
const MEDIA_FADE: u32 = 0x0083_0680;
const MEDIA_PLAYED: u32 = 0x0083_08c0;
const MEDIA_PAUSE: u32 = 0x0083_05e0;
const MEDIA_UNPAUSE: u32 = 0x0083_0610;
/// The sound helpers: `BSAudio::QInstance` (`00453a70`),
/// `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`), `BSSoundHandle::Play`
/// (`00ad8830`) and the handle's destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_HANDLE_BY_ID: u32 = 0x00ad_73b0;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// The object byte `00598060` compares with +0x32 (`0071aa80`, thiscall).
const LOCATION_BYTE: u32 = 0x0071_aa80;

/// The word in the item slot of a list node.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The sound sequence these functions share: the form id of `form`
/// (`0084e3a0`), a sound handle on the stack from
/// `BSAudio::GetSoundHandleByNumericID(handle, id, 0x901)` on
/// `BSAudio::QInstance()`, `BSSoundHandle::Play(0)` on what it returns,
/// then the handle's destructor.
fn play_form_sound(e: &mut Engine, form: u32) {
    let id = e.call(FORM_ID, &args![form]).u32();
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_HANDLE_BY_ID, &args![audio, handle, id, 0x901u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// The `float` at `offset` times 1000.0 (`FLD`, `FMUL double [01017b70]`).
fn millis_of(e: &mut Engine, base: u32, offset: u32) -> f64 {
    let thousand = e.global::<f64>(THOUSAND);
    x87_mul(x87_load32(e.mem.u32(base + offset)), thousand)
}

// Translated from 00596790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media location controller's record writer (no stack words; the
/// engine map has no name): `TESForm::StartForm`, `TESFullName::Save` on
/// +0x18, then the chunks through `00485910`: `NAM1`..`NAM7` (+0x6C word,
/// +0x70, +0x74, +0x7C floats, +0x80, +0x84 words, +0x78 float), each
/// non-empty form list's form ids (+0x88 `HNAM`, +0x90 `ZNAM`, +0x98
/// `XNAM`, +0xA0 `YNAM`, +0xA8 `LNAM`, +0xB0 `GNAM`), `RNAM`
/// (the form id of +0x60 when set), `FNAM` (+0x64), then
/// `TESForm::CloseForm`.
pub fn fn_00596790(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    let word = e.mem.u32(base + 0x6c);
    e.call(WRITE_CHUNK, &args![0x314d_414eu32, word]);
    for (chunk, offset) in [
        (0x324d_414eu32, 0x70u32),
        (0x334d_414e, 0x74),
        (0x344d_414e, 0x7c),
    ] {
        // FLD/FSTP onto the stack.
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.call(WRITE_CHUNK, &args![chunk, value]);
    }
    let word = e.mem.u32(base + 0x80);
    e.call(WRITE_CHUNK, &args![0x354d_414eu32, word]);
    let word = e.mem.u32(base + 0x84);
    e.call(WRITE_CHUNK, &args![0x364d_414eu32, word]);
    let value = x87_quiet_bits(e.mem.u32(base + 0x78));
    e.call(WRITE_CHUNK, &args![0x374d_414eu32, value]);
    for (offset, chunk) in SAVED_LISTS {
        let mut node = base + offset;
        if e.call(LIST_IS_EMPTY, &args![node]).bool() {
            continue;
        }
        while node != 0 {
            let form = list_item(e, node);
            if form == 0 {
                break;
            }
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(WRITE_CHUNK, &args![chunk, id]);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
    let form = e.mem.u32(base + 0x60);
    if form != 0 {
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(WRITE_CHUNK, &args![0x4d41_4e52u32, id]);
    }
    let word = e.mem.u32(base + 0x64);
    e.call(WRITE_CHUNK, &args![0x4d41_4e46u32, word]);
    e.call(0x0048_5680, &args![this]);
}

// Translated from 00596ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media location controller's copy (`RET 4`: the source form; the
/// engine map has no name): the source through `__RTDynamicCast(source, 0,
/// 01183028, 0118418c, 0)`; nothing without it. Otherwise
/// `TESForm::CopyAllComponents`, the words +0x6C, +0x80, +0x84, +0x60,
/// +0x64, +0x68 and the floats +0x70, +0x74, +0x7C, +0x78 (FLD/FSTP), and
/// each of the six form lists emptied (`00470470`) and refilled with the
/// source list's items (`005ae3d0`, given the item slot) up to its first
/// empty slot.
pub fn fn_00596ac0(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, CAST_TARGET, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, other]);
    let word = e.mem.u32(other + 0x6c);
    e.mem.set_u32(base + 0x6c, word);
    for offset in [0x70u32, 0x74, 0x7c, 0x78] {
        let value = x87_quiet_bits(e.mem.u32(other + offset));
        e.mem.set_u32(base + offset, value);
    }
    for offset in [0x80u32, 0x84, 0x60, 0x64, 0x68] {
        let word = e.mem.u32(other + offset);
        e.mem.set_u32(base + offset, word);
    }
    for offset in CONTROLLER_LISTS {
        e.call(LIST_CLEAR, &args![base + offset]);
        let mut node = other + offset;
        while node != 0 && list_item(e, node) != 0 {
            let slot = e.call(LIST_ITEM, &args![node]).u32();
            e.call(LIST_ADD, &args![base + offset, slot]);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 00596d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media location controller's compare (`RET 4`: the other form;
/// true when they differ; the engine map has no name): the other through
/// `__RTDynamicCast` as in `00596ac0`; true without it, when a word
/// (+0x6C, +0x60, +0x64, +0x68), a float (+0x70, +0x74, +0x7C, +0x78,
/// unordered counts as different) or a word (+0x80, +0x84) differs, or
/// `TESForm::CompareAllComponents` answers true. Then each of the six
/// form lists: true when either first slot is empty (the other's checked
/// first), the counts (`005ae380`) differ, or one of this list's items
/// finds no item of the other list for which its vtable slot 0x10C
/// answers false. False after all six.
pub fn fn_00596d70(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, CAST_TARGET, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    for offset in [0x6cu32, 0x60, 0x64, 0x68] {
        if e.mem.u32(base + offset) != e.mem.u32(other + offset) {
            return true;
        }
    }
    for offset in [0x70u32, 0x74, 0x7c, 0x78] {
        let mine = x87_load32(e.mem.u32(base + offset));
        let theirs = x87_load32(e.mem.u32(other + offset));
        // FUCOMPP; TEST AH,0x44; JNP: on only when equal and ordered.
        if theirs != mine {
            return true;
        }
    }
    for offset in [0x80u32, 0x84] {
        if e.mem.u32(base + offset) != e.mem.u32(other + offset) {
            return true;
        }
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    for offset in CONTROLLER_LISTS {
        if list_item(e, other + offset) == 0 || list_item(e, base + offset) == 0 {
            return true;
        }
        let theirs = e.call(LIST_COUNT, &args![other + offset]).u32();
        let mine = e.call(LIST_COUNT, &args![base + offset]).u32();
        if theirs != mine {
            return true;
        }
        let mut node = base + offset;
        while node != 0 && list_item(e, node) != 0 {
            let mut found = false;
            let mut other_node = other + offset;
            while other_node != 0 && list_item(e, other_node) != 0 {
                let item = list_item(e, node);
                let other_item = list_item(e, other_node);
                if !e.vcall(item, 0x10c, &args![other_item]).bool() {
                    found = true;
                    break;
                }
                other_node = e.call(LIST_NEXT, &args![other_node]).u32();
            }
            if !found {
                return true;
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
    false
}

// Translated from 00598060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00598060 - the exception-unwinding frame (FS:[0]) around the sound handles is not translated.
/// The media set's kind-2 run (`RET 0xC`: time in milliseconds, a flag
/// byte, the location object; the engine map has no name). Channel 3
/// playing (`MediaIsPlaying(3, 1)`), the object's level (`00598040`)
/// below 100.0 (`low`). A set flag while +0x31 is clear zeroes
/// `011dd37c`. When the start time (+0x2C) and channel 2's played length
/// (`008308c0(2)`) are set and their sum less the fade (+0xB4 seconds) is
/// below `time`, channel 2 fades out, +0x2C becomes `time`, and the
/// answer is false when the object's state (`00598000`) is set; otherwise
/// it counts as a restart. A change (not playing, the flag differs from
/// +0x31, the object's byte (`0071aa80`) from +0x32, or `low` from a clear
/// +0x34) once `time` passed `011dd37c`, or a restart, picks the track:
/// with the flag +0x48 (the sound at +0xBC played when set and +0x30
/// clear; +0x30 set), else by the object's byte +0x68 or +0x58 (the sound
/// at +0xC0 when +0x30 and +0xC0 are set; +0x30 cleared). The track then
/// starts on channel 3 (fade 1000 ms for +0x48, else +0xB4 seconds; its
/// volume, the offset into what channel 3 plays unless restarted), after
/// `011dd37c` takes `time` (plus +0xAC seconds without the flag); the
/// flag, the object's byte and `low` are kept. Without a change: when not
/// `low`, `time` past `011ca9fc` and +0x30 clear, channel 3 fades out if
/// playing and the answer is false. True otherwise.
pub fn fn_00598060(e: &mut Engine, this: Ptr, time: u32, flag: u8, location: u32) -> bool {
    let base = this.addr();
    let playing = e.call(MEDIA_IS_PLAYING, &args![3u32, 1u32]).u8();
    let mut result = true;
    let level = f64::from(fn_00598040(e, Ptr::new(location)));
    let hundred = e.global::<f64>(HUNDRED);
    // FCOMP; TEST AH,5; JP: low only when level < 100.
    let low = u8::from(level < hundred);
    if e.mem.u8(base + 0x31) == 0 && flag != 0 {
        e.set_global(NEXT_TRACK_TIME, 0u32);
    }
    let played = e.call(MEDIA_PLAYED, &args![2u32]).u32();
    let mut restarted = false;
    let start = e.mem.u32(base + 0x2c);
    if start != 0 && played != 0 {
        let end = f64::from(start.wrapping_add(played));
        let left = x87_sub(end, millis_of(e, base, 0xb4));
        // FCOMPP; TEST AH,5; JP: on only when left < time.
        if left < f64::from(time) {
            let fade = millis_of(e, base, 0xb4);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE, &args![millis, 2u32]);
            e.mem.set_u32(base + 0x2c, time);
            if fn_00598000(e, Ptr::new(location)) != 0 {
                return false;
            }
            restarted = true;
        }
    }
    let changed = playing == 0
        || e.mem.u8(base + 0x31) != flag
        || e.mem.u8(base + 0x32) != e.call(LOCATION_BYTE, &args![location]).u8()
        || (e.mem.i8(base + 0x34) != low as i8 && e.mem.i8(base + 0x34) == 0);
    if (changed && e.global::<u32>(NEXT_TRACK_TIME) < time) || restarted {
        let track;
        if flag != 0 {
            if e.mem.u32(base + 0xbc) != 0 && e.mem.u8(base + 0x30) == 0 {
                let form = e.mem.u32(base + 0xbc);
                play_form_sound(e, form);
            }
            track = base + 0x48;
            e.mem.set_u8(base + 0x30, 1);
        } else if e.call(LOCATION_BYTE, &args![location]).u8() != 0 {
            if e.mem.u8(base + 0x30) != 0 && e.mem.u32(base + 0xc0) != 0 {
                let form = e.mem.u32(base + 0xc0);
                play_form_sound(e, form);
            }
            track = base + 0x68;
            e.mem.set_u8(base + 0x30, 0);
        } else {
            if e.mem.u8(base + 0x30) != 0 && e.mem.u32(base + 0xc0) != 0 {
                let form = e.mem.u32(base + 0xc0);
                play_form_sound(e, form);
            }
            track = base + 0x58;
            e.mem.set_u8(base + 0x30, 0);
        }
        if track == 0 {
            // Only when `this` plus the offset wraps to 0.
            let fade = millis_of(e, base, 0xb4);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE, &args![millis, 3u32]);
            result = false;
        } else {
            e.set_global(NEXT_TRACK_TIME, time);
            if flag == 0 {
                let delay = millis_of(e, base, 0xac);
                let next = x87_add(delay, f64::from(e.global::<u32>(NEXT_TRACK_TIME)));
                e.set_global(NEXT_TRACK_TIME, fistp_chop_low(next));
            }
            let mut offset = 0u32;
            if e.call(MEDIA_POSITION, &args![3u32]).u32() != 0 && !restarted {
                let position = e.call(MEDIA_POSITION, &args![3u32]).u32();
                offset = time.wrapping_sub(position);
            }
            let volume = x87_quiet_bits(e.mem.u32(track + 8));
            let fade = if track == base + 0x48 {
                1000
            } else {
                fistp_chop_low(millis_of(e, base, 0xb4))
            };
            let name = e.call(0x0055_9450, &args![track]).u32();
            e.call(
                MEDIA_START,
                &args![3u32, name, fade, 1u32, 1u32, volume, offset],
            );
            e.call(MEDIA_UPDATE, &args![]);
        }
        e.mem.set_u8(base + 0x31, flag);
        let byte = e.call(LOCATION_BYTE, &args![location]).u8();
        e.mem.set_u8(base + 0x32, byte);
        e.mem.set_u8(base + 0x34, low);
    } else if low == 0 && time > e.global::<u32>(MEDIA_SET_TIME) && e.mem.u8(base + 0x30) == 0 {
        if playing != 0 {
            let fade = millis_of(e, base, 0xb4);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE, &args![millis, 3u32]);
        }
        result = false;
    }
    result
}

/// `00598550`'s start of the track at +0x48 on `channel`: its fade (+0xB0
/// seconds, truncated), volume (+0x50) and no offset.
fn start_first_track(e: &mut Engine, base: u32, channel: u32) {
    let volume = x87_quiet_bits(e.mem.u32(base + 0x50));
    let fade = fistp_chop_low(millis_of(e, base, 0xb0));
    let name = e.call(0x0055_9450, &args![base + 0x48]).u32();
    e.call(
        MEDIA_START,
        &args![channel, name, fade, 1u32, 1u32, volume, 0u32],
    );
}

// Translated from 00598550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00598550 - the exception-unwinding frame (FS:[0]) around the sound handles is not translated.
/// The media set's kind-0 run (`RET 0xC`: a flag byte, time in
/// milliseconds, the channel; the engine map has no name). With the flag:
/// when the channel is not playing (`MediaIsPlaying(channel, 0)`) and
/// +0x30 is clear, the sound at +0xBC plays (when set), `011ca9fc` becomes
/// `time - 1`, +0x30 is set and the track at +0x48 starts, then the media
/// update (`008304c0`) and `MediaPause`. Then `MediaUnPause` and +0x30 set
/// when `011ca9fc` plus the `float` at +0xAC is below `time`; and when the
/// channel's played length (`008308c0`) is set and `time` passed
/// `011ca9fc` plus it less 1000, the track starts again, the media update
/// runs and `011ca9fc` becomes `time`. Without the flag: when the channel
/// plays and +0x30 is set it fades out (+0xB0 seconds), the sound at +0xC0
/// plays (when set), `011ca9fc` becomes `time` plus +0xB4 seconds
/// (truncated), `BGSMenuPacker::RecomputePacking` (engine map name) runs on
/// `011dd374` with it when that is set, and +0x30 is cleared. Answers the
/// flag.
pub fn fn_00598550(e: &mut Engine, this: Ptr, flag: u8, time: u32, channel: u32) -> u8 {
    let base = this.addr();
    if flag != 0 {
        let playing = e.call(MEDIA_IS_PLAYING, &args![channel, 0u32]).u8();
        if playing == 0 && e.mem.u8(base + 0x30) == 0 {
            if e.mem.u32(base + 0xbc) != 0 && e.mem.u8(base + 0x30) == 0 {
                let form = e.mem.u32(base + 0xbc);
                play_form_sound(e, form);
            }
            e.set_global(MEDIA_SET_TIME, time.wrapping_sub(1));
            e.mem.set_u8(base + 0x30, 1);
            start_first_track(e, base, channel);
            e.call(MEDIA_UPDATE, &args![]);
            e.call(MEDIA_PAUSE, &args![]);
        }
        let set_time = f64::from(e.global::<u32>(MEDIA_SET_TIME));
        let until = x87_add(set_time, x87_m32(e.mem.u32(base + 0xac)));
        // FCOMPP; TEST AH,5; JP: on only when until < time.
        if until < f64::from(time) {
            e.call(MEDIA_UNPAUSE, &args![]);
            e.mem.set_u8(base + 0x30, 1);
        }
        let played = e.call(MEDIA_PLAYED, &args![channel]).u32();
        if played != 0 {
            let end = e
                .global::<u32>(MEDIA_SET_TIME)
                .wrapping_add(played)
                .wrapping_sub(1000);
            if time > end {
                start_first_track(e, base, channel);
                e.call(MEDIA_UPDATE, &args![]);
                e.set_global(MEDIA_SET_TIME, time);
            }
        }
    } else {
        let mut faded = false;
        if e.call(MEDIA_IS_PLAYING, &args![channel, 0u32]).bool() && e.mem.u8(base + 0x30) != 0 {
            faded = true;
            let fade = millis_of(e, base, 0xb0);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE, &args![millis, channel]);
        }
        if faded {
            if e.mem.u32(base + 0xc0) != 0 {
                let form = e.mem.u32(base + 0xc0);
                play_form_sound(e, form);
            }
            let next = x87_add(f64::from(time), millis_of(e, base, 0xb4));
            e.set_global(MEDIA_SET_TIME, fistp_chop_low(next));
            let packer = e.global::<u32>(0x011d_d374);
            if packer != 0 {
                let set_time = e.global::<u32>(MEDIA_SET_TIME);
                e.call(0x0070_37c0, &args![packer, set_time]);
            }
            e.mem.set_u8(base + 0x30, 0);
        }
    }
    flag
}

// Translated from 00598890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00598890 - the exception-unwinding frame (FS:[0]) around the sound handles is not translated.
/// The media set's kind-3 run (`RET 4`: time in milliseconds; the engine
/// map has no name). True at once unless `time` passed `011ca9fc`.
/// Otherwise the object `008d8520` gives on `011dea10` (engine map:
/// `MiddleHighProcess::GetSavedAcquireObject`) decides: none, or both
/// `00966a20` not below `00595ea0` and `00596030` not below `00966a20`
/// (the doubles they return in `ST0`; unordered counts as below), plays
/// the sound at +0xBC and sets `011ca9fc` to `time` plus
/// `00944460(+0xAC seconds, +0xB4 seconds)` (each through `_ftol2`);
/// otherwise the sound at +0xC0 and `00944460(+0xB0 seconds, +0xB8
/// seconds)`. Answers true.
pub fn fn_00598890(e: &mut Engine, this: Ptr, time: u32) -> bool {
    let base = this.addr();
    if time <= e.global::<u32>(MEDIA_SET_TIME) {
        return true;
    }
    let holder = e.global::<u32>(0x011d_ea10);
    let object = e.call(0x008d_8520, &args![holder]).u32();
    let first = if object != 0 {
        let low = e.call(0x0059_5ea0, &args![object]).f64();
        let middle = e.call(0x0096_6a20, &args![object]).f64();
        // FCOMP; TEST AH,1; JNZ: below (or unordered) answers false.
        if !(middle >= low) {
            false
        } else {
            let middle = e.call(0x0096_6a20, &args![object]).f64();
            let high = e.call(0x0059_6030, &args![object]).f64();
            high >= middle
        }
    } else {
        true
    };
    let (sound, a, b) = if first {
        (0xbcu32, 0xb4u32, 0xacu32)
    } else {
        (0xc0, 0xb8, 0xb0)
    };
    let form = e.mem.u32(base + sound);
    play_form_sound(e, form);
    let pushed_first = millis_of(e, base, a);
    let pushed_first = e.call(FTOL, &args![pushed_first]).u32();
    let pushed_second = millis_of(e, base, b);
    let pushed_second = e.call(FTOL, &args![pushed_second]).u32();
    let span = e
        .call(0x0094_4460, &args![pushed_second, pushed_first])
        .u32();
    e.set_global(MEDIA_SET_TIME, span.wrapping_add(time));
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00598040, fn_00598040(Ptr) -> f32),
        entry!(0x0059_7610, fn_00597610(Ptr) -> Ptr),
        entry!(0x0059_7880, fn_00597880(Ptr, u32) -> Ptr),
        entry!(0x0059_63d0, fn_005963d0(Ptr, Ptr) -> bool),
        entry!(0x0059_74f0, fn_005974f0(Ptr)),
        entry!(0x0059_75b0, fn_005975b0(Ptr)),
        entry!(0x0059_77d0, fn_005977d0(Ptr) -> Ptr),
        entry!(0x0059_7850, fn_00597850(Ptr)),
        entry!(0x0059_78b0, fn_005978b0(Ptr)),
        entry!(0x0059_7990, fn_00597990(Ptr, u32, u8, u32) -> bool),
        entry!(0x0059_7a60, fn_00597a60(Ptr, u32, u8, u32) -> bool),
        entry!(0x0059_8000, fn_00598000(Ptr) -> u32),
        entry!(0x0059_8020, fn_00598020(Ptr) -> u8),
        entry!(0x0059_6790, fn_00596790(Ptr)),
        entry!(0x0059_6ac0, fn_00596ac0(Ptr, u32)),
        entry!(0x0059_6d70, fn_00596d70(Ptr, u32) -> bool),
        entry!(0x0059_8060, fn_00598060(Ptr, u32, u8, u32) -> bool),
        entry!(0x0059_8550, fn_00598550(Ptr, u8, u32, u32) -> u8),
        entry!(0x0059_8890, fn_00598890(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_offset_0x3c() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x3c, 1.5);
        e.mem.set_f32(object + 0x38, 9.0);
        assert_eq!(e.call(0x0059_8040, &args![object]).f32(), 1.5);
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00597610_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_7610,
            size: 0xc4,
            globals: &[(0x011c_a9fc, 0xdead_beef)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2d00, &[A::T(0x38)]),
                (0x0059_77d0, &[A::T(0x48)]),
                (0x0059_77d0, &[A::T(0x58)]),
                (0x0059_77d0, &[A::T(0x68)]),
                (0x0059_77d0, &[A::T(0x78)]),
                (0x0059_77d0, &[A::T(0x88)]),
                (0x0059_77d0, &[A::T(0x98)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6f)]),
                (0x0059_7850, &[A::T(0x48)]),
                (0x0059_7850, &[A::T(0x58)]),
                (0x0059_7850, &[A::T(0x68)]),
                (0x0059_7850, &[A::T(0x78)]),
                (0x0059_7850, &[A::T(0x88)]),
                (0x0059_7850, &[A::T(0x98)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_42ec),
                (0x18, 4, 0x0103_42d8),
                (0x44, 4, 0xffff_ffff),
                (0xac, 4, 0x0),
                (0xb0, 4, 0x0),
                (0xb4, 4, 0x0),
                (0xb8, 4, 0x0),
                (0xbc, 4, 0x0),
                (0xc0, 4, 0x0),
                (0x2c, 4, 0x0),
                (0x28, 4, 0x0),
                (0x30, 1, 0x0),
                (0x31, 1, 0x0),
                (0x33, 1, 0x0),
                (0x34, 1, 0x0),
            ],
        });
    }

    #[test]
    fn fn_00597610_clears_the_global() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.mem.map(0x011c_a9fc, 4);
        e.mem.set_u32(0x011c_a9fc, 0xdead_beef);
        for a in [
            0x0048_3370u32,
            0x0040_2d00,
            0x0059_77d0,
            0x0059_7850,
            0x004f_15a0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call(0x0059_7610, &args![this]);
        assert_eq!(e.mem.u32(0x011c_a9fc), 0);
    }
    #[test]
    fn at_00597880_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_78b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_7880, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_7880)
                .collect();
            let mut expected = vec![(0x0059_78b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod media_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005963d0_reads_the_controller_chunks() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![
                (0x4449_4445u32, 0u32), // EDID
                (0x444e_424f, 0),       // OBND
                (0x4c4c_5546, 0),       // FULL
                (0x314d_414e, 1),       // NAM1
                (0x324d_414e, 2),
                (0x334d_414e, 3),
                (0x344d_414e, 4),
                (0x354d_414e, 5),
                (0x364d_414e, 6),
                (0x374d_414e, 7),
                (0x4d41_4e46, 0x46), // FNAM
                (0x4d41_4e52, 0x52), // RNAM
                (0x4d41_4e48, 0x48), // HNAM
                (0x4d41_4e59, 0x59), // YNAM
                (0x4d41_4e57, 0x57), // WNAM: nothing
                (0x4d41_4e58, 0x58), // XNAM
                (0x4d41_4e4c, 0x4c), // LNAM
                (0x4d41_4e47, 0x47), // GNAM
                (0x4d41_4e5a, 0x5a), // ZNAM
            ],
            0usize,
        )));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_63d0, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        let words: Vec<u32> = [0x6c, 0x70, 0x74, 0x7c, 0x80, 0x84, 0x78]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(e.mem.u32(this + 0x64), 0x46);
        assert_eq!(e.mem.u32(this + 0x60), 0x52);
        assert_eq!(
            *added.borrow(),
            vec![
                (this + 0x88, 0x48),
                (this + 0xa0, 0x59),
                (this + 0x98, 0x58),
                (this + 0xa8, 0x4c),
                (this + 0xb0, 0x47),
                (this + 0x90, 0x5a)
            ]
        );
    }

    #[test]
    fn controller_defaults_and_list_clear() {
        let mut e = Engine::new();
        e.map(0x0102_6000, 0x1000);
        e.map(0x0103_4000, 0x1000);
        e.set_global(0x0102_6994, 1.5f32);
        e.set_global(0x0103_40a4, 2.5f32);
        e.set_global(0x0103_4294, 3.5f32);
        quiet(&mut e, &[LIST_CLEAR]);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x64, 9);
        e.call_log = Some(vec![]);
        e.call(0x0059_74f0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x70), 1.5);
        assert_eq!(e.mem.f32(this + 0x74), 2.5);
        assert_eq!(e.mem.f32(this + 0x7c), 3.5);
        assert_eq!(e.mem.u32(this + 0x80), 0x40);
        assert_eq!(e.mem.u32(this + 0x84), 0x150);
        let cleared: Vec<u32> = calls(&e, LIST_CLEAR).iter().map(|c| c[0] - this).collect();
        assert_eq!(cleared, CONTROLLER_LISTS.to_vec());
        assert_eq!(
            (
                e.mem.u32(this + 0x60),
                e.mem.u32(this + 0x64),
                e.mem.u32(this + 0x68)
            ),
            (0, 1, 0)
        );
        e.call_log = Some(vec![]);
        e.call(0x0059_75b0, &args![this]);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 6);
    }

    #[test]
    fn tracks_and_media_set_destructor() {
        let mut e = Engine::new();
        e.map(0x0103_4000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0103_3000, 0x1000);
        e.set_global(0x0103_4424, 0.5f32);
        e.set_global(0x0101_6410, 0.25f32);
        e.set_global(0x0103_395c, 0.75f32);
        quiet(
            &mut e,
            &[
                0x0040_37b0,
                STRING_SET,
                0x007d_9ec0,
                0x0059_1300,
                0x0048_3630,
            ],
        );
        let track = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_77d0, &args![track]).u32(), track);
        assert_eq!(calls(&e, 0x0040_37b0), vec![vec![track]]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![track, 0, 0]]);
        assert_eq!((e.mem.f32(track + 8), e.mem.f32(track + 0xc)), (0.5, 0.25));
        e.call(0x0059_7850, &args![track]);
        assert_eq!((e.mem.f32(track + 8), e.mem.f32(track + 0xc)), (0.75, 0.25));

        let set = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x0059_78b0, &args![set]);
        assert_eq!(e.mem.u32(set), 0x0103_42ec);
        assert_eq!(e.mem.u32(set + 0x18), 0x0103_42d8);
        let tracks: Vec<u32> = calls(&e, 0x007d_9ec0).iter().map(|c| c[0] - set).collect();
        assert_eq!(tracks, vec![0x98, 0x88, 0x78, 0x68, 0x58, 0x48]);
        assert_eq!(
            calls(&e, 0x0059_1300),
            vec![vec![set + 0x38], vec![set + 0x18]]
        );
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![set]]);
    }

    #[test]
    fn getters_read_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x6c, 0xffff_ff30);
        e.mem.set_u8(this + 0x37, 0xab);
        assert_eq!(e.call(0x0059_8000, &args![this]).u32(), 3);
        e.mem.set_u32(this + 0x6c, 0x10);
        assert_eq!(e.call(0x0059_8000, &args![this]).u32(), 1);
        assert_eq!(e.call(0x0059_8020, &args![this]).u8(), 0xab);
    }

    #[test]
    fn fn_00597990_dispatches_by_kind() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        let this = e.mem.alloc(0xc4);
        e.register(0x0059_8550, |_, _| ret(1));
        e.register(0x0059_8060, |_, _| ret(1));
        quiet(&mut e, &[0x0059_8890]);
        // Kind 0 with an object: 00598550(flag, time, 4).
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7990, &args![this, 500u32, 0x101u32, 0x8000u32])
            .bool());
        assert_eq!(calls(&e, 0x0059_8550), vec![vec![this, 1, 500, 4]]);
        // Kind 2: 00598060 with the three words.
        e.mem.set_u32(this + 0x44, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7990, &args![this, 500u32, 0u32, 0x8000u32])
            .bool());
        assert_eq!(calls(&e, 0x0059_8060), vec![vec![this, 500, 0, 0x8000]]);
        // Kind 1 runs 00597a60 (a set flag answers false at once).
        e.mem.set_u32(this + 0x44, 1);
        e.register(MEDIA_IS_PLAYING, |_, _| ret(0));
        assert!(!e
            .call(0x0059_7990, &args![this, 500u32, 1u32, 0x8000u32])
            .bool());
        // Kind 3 with an object: false.
        e.mem.set_u32(this + 0x44, 3);
        assert!(!e
            .call(0x0059_7990, &args![this, 500u32, 0u32, 0x8000u32])
            .bool());
        // Kind 3 without one: true before the set time, else 00598890.
        e.set_global(MEDIA_SET_TIME, 600u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_7990, &args![this, 500u32, 0u32, 0u32]).bool());
        assert!(calls(&e, 0x0059_8890).is_empty());
        assert!(!e.call(0x0059_7990, &args![this, 600u32, 0u32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0059_8890), vec![vec![this, 600]]);
        // Kind 0 without one: 00598550; kind 2 without one: false.
        e.mem.set_u32(this + 0x44, 0);
        assert!(e.call(0x0059_7990, &args![this, 7u32, 0u32, 0u32]).bool());
        e.mem.set_u32(this + 0x44, 2);
        assert!(!e.call(0x0059_7990, &args![this, 7u32, 0u32, 0u32]).bool());
    }

    /// A music type and an object for `00597a60`: thresholds 10/20/30 for
    /// the first set (+0x54/+0x64/+0x74) and 40/50/60 for the other, all
    /// flag bits set, fade 2.0 s, delay 1.5 s; the object's level at +0x3C,
    /// its byte at +0x37 and state word at +0x6C.
    fn music(
        level: f32,
        first_set: bool,
        playing: bool,
        length: u32,
        position: u32,
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x0101_3000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(THOUSAND_FLOAT, 1000.0f32);
        let this = e.mem.alloc(0xc4);
        for (offset, value) in [
            (0x54u32, 10.0f32),
            (0x64, 20.0),
            (0x74, 30.0),
            (0x84, 40.0),
            (0x94, 50.0),
            (0xa4, 60.0),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        for track in [0x48u32, 0x58, 0x68, 0x78, 0x88, 0x98] {
            e.mem.set_f32(this + track + 8, track as f32);
        }
        e.mem.set_u8(this + 0xa8, 0x3f);
        e.mem.set_f32(this + 0xb4, 2.0);
        e.mem.set_f32(this + 0xac, 1.5);
        let object = e.mem.alloc(0x70);
        e.mem.set_f32(object + 0x3c, level);
        e.mem.set_u8(object + 0x37, u8::from(first_set));
        e.register_double(MEDIA_IS_PLAYING, move |_, _| ret(u32::from(playing)));
        e.register_double(MEDIA_LENGTH, move |_, _| ret(length));
        e.register_double(MEDIA_POSITION, move |_, _| ret(position));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(0x0055_9450, |_, a| ret(a[0] + 1));
        quiet(&mut e, &[MEDIA_FADE_OUT, MEDIA_START, MEDIA_UPDATE]);
        (e, this, object)
    }

    #[test]
    fn fn_00597a60_starts_the_track_above_the_level() {
        // Level 0.15 * 100 = 15: the lowest threshold above it is 20.
        let (mut e, this, object) = music(0.15, true, false, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(e.mem.u8(this + 0x34), 2);
        assert_eq!(e.mem.u8(this + 0x33), 1);
        assert_eq!(e.mem.u32(this + 0x2c), 5000);
        assert_eq!(e.global::<u32>(NEXT_TRACK_TIME), 6500);
        assert_eq!(
            calls(&e, MEDIA_START),
            vec![vec![2, this + 0x59, 2000, 1, 1, 88.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, MEDIA_UPDATE).len(), 1);
        assert!(calls(&e, MEDIA_FADE_OUT).is_empty());

        // The other set: 15 is below 40.
        let (mut e, this, object) = music(0.15, false, false, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(e.mem.u8(this + 0x34), 1);
        assert_eq!(calls(&e, MEDIA_START)[0][5], 120.0f32.to_bits());
    }

    #[test]
    fn fn_00597a60_without_a_track_fades_out() {
        // Level 70 is above every threshold: no track.
        let (mut e, this, object) = music(0.7, true, true, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT), vec![vec![2000, 2]]);
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u8(this + 0x34), 0);
    }

    #[test]
    fn fn_00597a60_keeps_a_playing_unchanged_track() {
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u32(this + 0x2c), 0);

        // A changed track before the next start time: kept too.
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u8(this + 0x34), 0);

        // A set flag: false at once.
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 5000u32, 1u32, object])
            .bool());
        assert!(calls(&e, MEDIA_LENGTH).is_empty());
    }

    #[test]
    fn fn_00597a60_restarts_a_finished_track() {
        // Started at 1000, 3000 long, 2 s fade: over once 2000 < time.
        let (mut e, this, object) = music(0.15, true, true, 3000, 4000);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT), vec![vec![2000, 2]]);
        // Restarted: no offset into the playing media.
        let started = calls(&e, MEDIA_START);
        assert_eq!(started.len(), 1);
        assert_eq!(started[0][6], 0);
        assert_eq!(e.mem.u32(this + 0x2c), 2500);

        // Not over yet (2000 is not below 2000): nothing.
        let (mut e, this, object) = music(0.15, true, true, 3000, 0);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 2000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_FADE_OUT).is_empty());

        // Over, with an object state: false after the fade.
        let (mut e, this, object) = music(0.15, true, true, 3000, 0);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u32(object + 0x6c, 0x10);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT).len(), 1);
        assert!(calls(&e, MEDIA_START).is_empty());
    }

    #[test]
    fn fn_00597a60_starts_with_the_offset_into_the_media() {
        // Not playing, so a new start; the media's position gives the offset.
        let (mut e, this, object) = music(0.15, true, false, 0, 4200);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_START)[0][6], 800);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles over nodes `{item, next}`: the item slot is the node.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut n = 0;
            while node != 0 && e.mem.u32(node) != 0 {
                n += 1;
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
    }

    /// Fills the list head at `head` with `items` (extra nodes allocated).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    fn sound_doubles(e: &mut Engine) {
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(AUDIO_INSTANCE, |_, _| ret(0x0bad_0000));
        e.register(AUDIO_HANDLE_BY_ID, |_, a| ret(a[1]));
        e.register(SOUND_PLAY, |_, _| Ret::default());
        e.register(SOUND_HANDLE_DESTROY, |_, _| Ret::default());
    }

    #[test]
    fn fn_00596790_writes_the_chunks_and_the_lists() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        for a in [0x0048_55a0u32, 0x0048_7010, 0x0048_5680, WRITE_CHUNK] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let this = e.mem.alloc(0xc4);
        for (offset, value) in [
            (0x6cu32, 1u32),
            (0x70, 0x4000_0000),
            (0x74, 0x7f80_0001),
            (0x7c, 4),
            (0x80, 5),
            (0x84, 6),
            (0x78, 7),
            (0x64, 0x64),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        let form_a = e.mem.alloc(0x10);
        e.mem.set_u32(form_a + 0xc, 0xaa);
        let form_b = e.mem.alloc(0x10);
        e.mem.set_u32(form_b + 0xc, 0xbb);
        fill(&mut e, this + 0x88, &[form_a, form_b]);
        fill(&mut e, this + 0xa0, &[form_b]);
        e.call_log = Some(vec![]);
        e.call(0x0059_6790, &args![this]);
        assert_eq!(calls(&e, 0x0048_55a0), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0048_7010), vec![vec![this + 0x18]]);
        assert_eq!(
            calls(&e, WRITE_CHUNK),
            vec![
                vec![0x314d_414e, 1],
                vec![0x324d_414e, 0x4000_0000],
                vec![0x334d_414e, 0x7fc0_0001],
                vec![0x344d_414e, 4],
                vec![0x354d_414e, 5],
                vec![0x364d_414e, 6],
                vec![0x374d_414e, 7],
                vec![0x4d41_4e48, 0xaa],
                vec![0x4d41_4e48, 0xbb],
                vec![0x4d41_4e59, 0xbb],
                vec![0x4d41_4e46, 0x64],
            ]
        );
        assert_eq!(calls(&e, 0x0048_5680), vec![vec![this]]);

        // RNAM with a form at +0x60.
        e.mem.set_u32(this + 0x60, form_a);
        e.call_log = Some(vec![]);
        e.call(0x0059_6790, &args![this]);
        assert!(calls(&e, WRITE_CHUNK).contains(&vec![0x4d41_4e52, 0xaa]));
    }

    #[test]
    fn fn_00596ac0_copies_fields_and_lists() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] & !1));
        for a in [0x0048_51b0u32, LIST_CLEAR, LIST_ADD] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xc4);
        let other = e.mem.alloc(0xc4);
        for offset in (0x60u32..0x88).step_by(4) {
            e.mem.set_u32(other + offset, offset);
        }
        e.mem.set_u32(other + 0x70, 0xff80_0001);
        fill(&mut e, other + 0x98, &[0x111, 0x222]);
        // A cast that fails: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0059_6ac0, &args![this, 1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x0059_6ac0, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, CAST_TARGET, 0]]
        );
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
        for offset in (0x60u32..0x88).step_by(4) {
            let expected = if offset == 0x70 { 0xffc0_0001 } else { offset };
            assert_eq!(e.mem.u32(this + offset), expected, "{offset:x}");
        }
        let cleared: Vec<u32> = calls(&e, LIST_CLEAR).iter().map(|c| c[0] - this).collect();
        assert_eq!(cleared, CONTROLLER_LISTS.to_vec());
        let second = e.mem.u32(other + 0x9c);
        assert_eq!(
            calls(&e, LIST_ADD),
            vec![vec![this + 0x98, other + 0x98], vec![this + 0x98, second]]
        );
    }

    /// Two controllers alike: equal fields and one item in each list.
    fn alike(e: &mut Engine) -> (u32, u32) {
        list_doubles(e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0048_5270, |_, _| ret(0));
        // The items' slot 0x10C: differs unless the other item is the same.
        let vtable = e.mem.alloc(0x110);
        e.mem.set_u32(vtable + 0x10c, 0x7000_010c);
        e.register(0x7000_010c, |_, a| ret(u32::from(a[0] != a[1])));
        let this = e.mem.alloc(0xc4);
        let other = e.mem.alloc(0xc4);
        for offset in CONTROLLER_LISTS {
            let item = e.mem.alloc(4);
            e.mem.set_u32(item, vtable);
            e.mem.set_u32(this + offset, item);
            e.mem.set_u32(other + offset, item);
        }
        (this, other)
    }

    #[test]
    fn fn_00596d70_compares_fields_and_lists() {
        let mut e = Engine::new();
        let (this, other) = alike(&mut e);
        assert!(!e.call(0x0059_6d70, &args![this, other]).bool());
        assert!(e.call(0x0059_6d70, &args![this, 0u32]).bool());
        // A word, a float (NaN is never equal) and a component differ.
        e.mem.set_u32(other + 0x64, 1);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        e.mem.set_u32(other + 0x64, 0);
        e.mem.set_u32(this + 0x74, 0x7fc0_0000);
        e.mem.set_u32(other + 0x74, 0x7fc0_0000);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        e.mem.set_u32(this + 0x74, 0);
        e.mem.set_u32(other + 0x74, 0x8000_0000);
        assert!(!e.call(0x0059_6d70, &args![this, other]).bool());
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        e.register(0x0048_5270, |_, _| ret(0));
        // A list item no item of the other list matches.
        let other_item = e.mem.alloc(4);
        let vtable = e.mem.u32(e.mem.u32(this + 0xa8));
        e.mem.set_u32(other_item, vtable);
        let mine = e.mem.u32(this + 0xa8);
        e.mem.set_u32(other + 0xa8, other_item);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        // Matched somewhere in a longer list, with equal counts.
        fill(&mut e, other + 0xa8, &[other_item, mine]);
        fill(&mut e, this + 0xa8, &[mine, mine]);
        assert!(!e.call(0x0059_6d70, &args![this, other]).bool());
        // Different counts, and an empty list on either side.
        fill(&mut e, this + 0xa8, &[mine]);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        let (this, other) = alike(&mut e);
        e.mem.set_u32(this + 0xb0, 0);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
        let (this, other) = alike(&mut e);
        e.mem.set_u32(other + 0x88, 0);
        assert!(e.call(0x0059_6d70, &args![this, other]).bool());
    }

    /// A media set and object: fade 2.0 s, delay 1.5 s, track volumes at
    /// +0x50/+0x60/+0x70, the object's level and byte.
    fn media_set(
        level: f32,
        byte: u8,
        playing: bool,
        played: u32,
        position: u32,
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e.set_global(HUNDRED, 100.0f64);
        let this = e.mem.alloc(0xc4);
        e.mem.set_f32(this + 0xb4, 2.0);
        e.mem.set_f32(this + 0xac, 1.5);
        e.mem.set_f32(this + 0x50, 0.5);
        e.mem.set_f32(this + 0x60, 0.6);
        e.mem.set_f32(this + 0x70, 0.7);
        let object = e.mem.alloc(0x70);
        e.mem.set_f32(object + 0x3c, level);
        e.mem.set_u8(object + 0x40, byte);
        e.register(LOCATION_BYTE, |e, a| ret(u32::from(e.mem.u8(a[0] + 0x40))));
        e.register_double(MEDIA_IS_PLAYING, move |_, _| ret(u32::from(playing)));
        e.register_double(MEDIA_PLAYED, move |_, _| ret(played));
        e.register_double(MEDIA_POSITION, move |_, _| ret(position));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(0x0055_9450, |_, a| ret(a[0] + 1));
        for a in [MEDIA_FADE, MEDIA_START, MEDIA_UPDATE] {
            e.register(a, |_, _| Ret::default());
        }
        sound_doubles(&mut e);
        (e, this, object)
    }

    #[test]
    fn fn_00598060_starts_the_flag_track() {
        let (mut e, this, object) = media_set(2.0, 0, false, 0, 4000);
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form + 0xc, 0x77);
        e.mem.set_u32(this + 0xbc, form);
        e.set_global(NEXT_TRACK_TIME, 99u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_8060, &args![this, 5000u32, 1u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_IS_PLAYING), vec![vec![3, 1]]);
        let handles = calls(&e, AUDIO_HANDLE_BY_ID);
        assert_eq!(handles.len(), 1);
        assert_eq!(&handles[0][2..], &[0x77, 0x901]);
        assert_eq!(calls(&e, SOUND_PLAY), vec![vec![handles[0][1], 0]]);
        assert_eq!(e.mem.u8(this + 0x30), 1);
        // The flag: the next start is the time itself.
        assert_eq!(e.global::<u32>(NEXT_TRACK_TIME), 5000);
        assert_eq!(
            calls(&e, MEDIA_START),
            vec![vec![3, this + 0x49, 1000, 1, 1, 0.5f32.to_bits(), 1000]]
        );
        assert_eq!(
            (
                e.mem.u8(this + 0x31),
                e.mem.u8(this + 0x32),
                e.mem.u8(this + 0x34)
            ),
            (1, 0, 1)
        );
    }

    #[test]
    fn fn_00598060_picks_by_the_object_byte_without_the_flag() {
        for (byte, track, volume) in [(1u8, 0x68u32, 0.7f32), (0, 0x58, 0.6)] {
            let (mut e, this, object) = media_set(0.5, byte, false, 0, 0);
            let form = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc0, form);
            e.mem.set_u8(this + 0x30, 1);
            e.call_log = Some(vec![]);
            assert!(e
                .call(0x0059_8060, &args![this, 5000u32, 0u32, object])
                .bool());
            assert_eq!(calls(&e, SOUND_PLAY).len(), 1);
            assert_eq!(e.mem.u8(this + 0x30), 0);
            assert_eq!(e.global::<u32>(NEXT_TRACK_TIME), 6500);
            assert_eq!(
                calls(&e, MEDIA_START),
                vec![vec![3, this + track + 1, 2000, 1, 1, volume.to_bits(), 0]]
            );
            assert_eq!((e.mem.u8(this + 0x32), e.mem.u8(this + 0x34)), (byte, 1));
        }
    }

    #[test]
    fn fn_00598060_restarts_and_fades() {
        // Finished on channel 2 (1000 + 3000 - 2000 < 2500), no state: restart.
        let (mut e, this, object) = media_set(2.0, 0, true, 3000, 100);
        e.mem.set_u32(this + 0x2c, 1000);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_8060, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE), vec![vec![2000, 2]]);
        assert_eq!(calls(&e, MEDIA_START)[0][6], 0);
        assert_eq!(e.mem.u32(this + 0x2c), 2500);
        // With the object's state set: false after the fade.
        let (mut e, this, object) = media_set(2.0, 0, true, 3000, 100);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u32(object + 0x6c, 0x20);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_8060, &args![this, 2500u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_START).is_empty());
        // Unchanged and not low, past 011ca9fc: fade channel 3, false.
        let (mut e, this, object) = media_set(150.0, 0, true, 0, 0);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_8060, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE), vec![vec![2000, 3]]);
        assert!(calls(&e, MEDIA_START).is_empty());
        // A set flag clears 011dd37c while +0x31 is clear.
        let (mut e, this, object) = media_set(0.5, 0, true, 0, 0);
        e.mem.set_u8(this + 0x34, 1);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_8060, &args![this, 2500u32, 1u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_START).len(), 1);
    }

    fn kind0(playing: bool, played: u32) -> (Engine, u32) {
        let (mut e, this, _) = media_set(0.0, 0, playing, played, 0);
        e.mem.set_f32(this + 0xb0, 3.0);
        e.mem.set_f32(this + 0xac, 0.25);
        for a in [MEDIA_PAUSE, MEDIA_UNPAUSE, 0x0070_37c0] {
            e.register(a, |_, _| Ret::default());
        }
        (e, this)
    }

    #[test]
    fn fn_00598550_starts_paused_then_unpauses() {
        let (mut e, this) = kind0(false, 0);
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xbc, form);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0059_8550, &args![this, 1u32, 500u32, 4u32]).u8(),
            1
        );
        assert_eq!(calls(&e, MEDIA_IS_PLAYING), vec![vec![4, 0]]);
        assert_eq!(calls(&e, SOUND_PLAY).len(), 1);
        assert_eq!(
            calls(&e, MEDIA_START),
            vec![vec![4, this + 0x49, 3000, 1, 1, 0.5f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, MEDIA_PAUSE).len(), 1);
        // 499 + 0.25 < 500: unpaused.
        assert_eq!(calls(&e, MEDIA_UNPAUSE).len(), 1);
        assert_eq!(e.global::<u32>(MEDIA_SET_TIME), 499);
        assert_eq!(e.mem.u8(this + 0x30), 1);

        // Playing: no start; past the played length less a second: again.
        let (mut e, this) = kind0(true, 1500);
        e.set_global(MEDIA_SET_TIME, 1000u32);
        e.call_log = Some(vec![]);
        e.call(0x0059_8550, &args![this, 1u32, 1501u32, 4u32]);
        assert_eq!(calls(&e, MEDIA_PAUSE).len(), 0);
        assert_eq!(calls(&e, MEDIA_START).len(), 1);
        assert_eq!(e.global::<u32>(MEDIA_SET_TIME), 1501);
        e.call_log = Some(vec![]);
        e.call(0x0059_8550, &args![this, 1u32, 1501u32, 4u32]);
        assert!(calls(&e, MEDIA_START).is_empty());
        assert!(calls(&e, MEDIA_UNPAUSE).is_empty());
    }

    #[test]
    fn fn_00598550_without_the_flag_fades_and_sets_the_time() {
        let (mut e, this) = kind0(true, 0);
        e.mem.set_u8(this + 0x30, 1);
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc0, form);
        e.set_global(0x011d_d374, 0x4444u32);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0059_8550, &args![this, 0u32, 500u32, 4u32]).u8(),
            0
        );
        assert_eq!(calls(&e, MEDIA_FADE), vec![vec![3000, 4]]);
        assert_eq!(calls(&e, SOUND_PLAY).len(), 1);
        assert_eq!(e.global::<u32>(MEDIA_SET_TIME), 2500);
        assert_eq!(calls(&e, 0x0070_37c0), vec![vec![0x4444, 2500]]);
        assert_eq!(e.mem.u8(this + 0x30), 0);
        // +0x30 clear: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0059_8550, &args![this, 0u32, 500u32, 4u32]);
        assert!(calls(&e, MEDIA_FADE).is_empty());
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_00598890_picks_the_sound_and_span() {
        let (mut e, this, _) = media_set(0.0, 0, false, 0, 0);
        e.mem.set_f32(this + 0xb0, 3.0);
        e.mem.set_f32(this + 0xb8, 4.0);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xbc, first);
        e.mem.set_u32(this + 0xc0, second);
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x011d_ea10, 0x5555u32);
        e.set_global(MEDIA_SET_TIME, 100u32);
        e.register(0x0094_4460, |_, a| ret(a[0] * 10 + a[1]));
        e.register(0x008d_8520, |_, _| ret(0));
        // Not past 011ca9fc: true at once.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_8890, &args![this, 100u32]).bool());
        assert!(calls(&e, 0x008d_8520).is_empty());
        // No object: the first sound, span(+0xAC, +0xB4).
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_8890, &args![this, 101u32]).bool());
        assert_eq!(calls(&e, 0x008d_8520), vec![vec![0x5555]]);
        assert_eq!(calls(&e, FORM_ID), vec![vec![first]]);
        assert_eq!(calls(&e, 0x0094_4460), vec![vec![1500, 2000]]);
        assert_eq!(e.global::<u32>(MEDIA_SET_TIME), 17101);
        // An object: its three values decide.
        e.register(0x008d_8520, |_, _| ret(0x6666));
        for (low, middle, high, first_sound) in [
            (1.0, 2.0, 3.0, true),
            (1.0, 1.0, 1.0, true),
            (2.0, 1.0, 3.0, false),
            (1.0, 2.0, 1.5, false),
            (f64::NAN, 2.0, 3.0, false),
        ] {
            e.register_double(0x0059_5ea0, move |_, _| st0(low));
            e.register_double(0x0096_6a20, move |_, _| st0(middle));
            e.register_double(0x0059_6030, move |_, _| st0(high));
            e.set_global(MEDIA_SET_TIME, 0u32);
            e.call_log = Some(vec![]);
            e.call(0x0059_8890, &args![this, 1u32]);
            let sound = if first_sound { first } else { second };
            assert_eq!(calls(&e, FORM_ID), vec![vec![sound]]);
            let span = if first_sound { 17001 } else { 34001 };
            assert_eq!(e.global::<u32>(MEDIA_SET_TIME), span);
        }
    }
}

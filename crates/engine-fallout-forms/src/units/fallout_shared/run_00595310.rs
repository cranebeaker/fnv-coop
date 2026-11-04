//! `fallout shared/run_00595310` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pointer at `011dea3c` (other units name it the player character
/// pointer; `00595600` calls `00969930`, `00595d10` and slot `0x1f4` on it).
const PLAYER_POINTER_011DEA3C: u32 = 0x011d_ea3c;
/// The pointer at `011dea10` `00595d30` calls `008d8520` on.
const OWNER_POINTER_011DEA10: u32 = 0x011d_ea10;
/// The controller `00595600` last ran for (cleared when its ratio passes 1.0).
const LAST_CONTROLLER_011DD374: u32 = 0x011d_d374;
/// A word that, when non-zero, keeps `00595600` from picking the list at `+0xb0`.
const BLOCK_LAST_LIST_011DD378: u32 = 0x011d_d378;
/// The `float` 1000.0 at `01013974`: `00595600`'s ratio before it is computed.
const RATIO_START_01013974: u32 = 0x0101_3974;
/// The `double` 100.0 at `01017a40` the ratio is compared with.
const RATIO_LIMIT_01017A40: u32 = 0x0101_7a40;
/// The `double` 1000.0 at `01017b70` (`+0x7c` is scaled by it).
const THOUSAND_01017B70: u32 = 0x0101_7b70;
/// The `double` 1.0 at `01012070`.
const ONE_01012070: u32 = 0x0101_2070;
/// The `double` 2.0 at `01011590` (`00595d30` averages two pairs with it).
const TWO_01011590: u32 = 0x0101_1590;
/// The `double` 0.09375 at `01034200` that scales `+0x80` and `+0x84`.
const SCALE_01034200: u32 = 0x0103_4200;
/// The `double` 6.0 at `01034208` the four byte levels are divided by.
const DIVISOR_01034208: u32 = 0x0103_4208;
/// The four cached `float`s `00595ea0`, `00595f50`, `00595fc0` and `00596030` return.
const CACHED_LEVEL_011CA9E8: u32 = 0x011c_a9e8;

/// The high byte of the x87 status word (`FNSTSW AX`, then `AH`) after
/// `FCOM st0, operand`: C0 (`0x01`) when `st0 < operand`, C3 (`0x40`) when
/// equal, nothing when greater, C0 | C2 | C3 (`0x45`) when unordered.
fn fcom(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0x00
    }
}

/// The parity flag after `TEST AH, mask` (set when the masked byte has an
/// even number of 1 bits), which `JP` / `JNP` test.
fn parity_even(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// The low word `FISTP qword` leaves with the rounding mode set to truncate
/// (`OR 0xc00`): out of range and NaN give the integer indefinite
/// `0x8000_0000_0000_0000`, whose low word is 0.
fn fistp_truncate_low(value: f64) -> u32 {
    let converted = if value.is_finite()
        && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    converted as u32
}

// Translated from 00595310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00595310 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x70 (0xb8 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_40c4`) is `MediaLocationController`, so this is the constructor of `MediaLocationController`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the floats at `01026994` and `010340a4` into `+0x70` and `+0x74`.
/// The exception-unwinding frame is not translated.
pub fn fn_00595310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_40c4);
    e.mem.set_u32(base + 0x18, 0x0103_40ac);
    e.call(0x0040_2d00, &args![base + 0x54]);
    e.call(0x0096_a2d0, &args![base + 0x88]);
    e.call(0x0096_a2d0, &args![base + 0x90]);
    e.call(0x0096_a2d0, &args![base + 0x98]);
    e.call(0x0096_a2d0, &args![base + 0xa0]);
    e.call(0x0096_a2d0, &args![base + 0xa8]);
    e.call(0x0096_a2d0, &args![base + 0xb0]);
    e.call(0x004f_15a0, &args![this, 0x70u32]);
    e.mem.set_u32(base + 0x6c, 0x4);
    let f1 = e.global::<f32>(0x0102_6994);
    e.mem.set_f32(base + 0x70, f1);
    let f2 = e.global::<f32>(0x0103_40a4);
    e.mem.set_f32(base + 0x74, f2);
    e.mem.set_f32(base + 0x7c, 0.0);
    e.mem.set_u32(base + 0x80, 0x40);
    e.mem.set_u32(base + 0x84, 0x150);
    e.call(0x0047_0470, &args![base + 0x88]);
    e.call(0x0047_0470, &args![base + 0x90]);
    e.call(0x0047_0470, &args![base + 0x98]);
    e.call(0x0047_0470, &args![base + 0xa0]);
    e.call(0x0047_0470, &args![base + 0xa8]);
    e.call(0x0047_0470, &args![base + 0xb0]);
    e.mem.set_u32(base + 0x60, 0x0);
    e.mem.set_u32(base + 0x64, 0x1);
    e.mem.set_u32(base + 0x68, 0x0);
    e.mem.set_u32(base + 0x24, 0x0);
    e.mem.set_u32(base + 0x28, 0x0);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.mem.set_u32(base + 0x30, 0x0);
    e.mem.set_u8(base + 0x35, 0x0);
    e.mem.set_u8(base + 0x36, 0x0);
    e.mem.set_u8(base + 0x37, 0x0);
    e.mem.set_u8(base + 0x38, 0x0);
    e.mem.set_u8(base + 0x4c, 0x0);
    e.mem.set_u32(base + 0x50, 0x0);
    e.mem.set_u32(base + 0x40, 0x0);
    e.mem.set_u32(base + 0x44, 0x0);
    e.mem.set_u32(base + 0x48, 0x1);
    this
}

// Translated from 00595530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of `MediaLocationController`: runs the destructor
/// `005960a0`, then frees the object through `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00595530(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005960a0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00595560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks an entry of the list `list` (`ECX` is not read): walks the nodes
/// (`006815c0` gives a node's item slot, `00726070` the next node) while the
/// item is non-null; each item replaces the pick when nothing is picked yet
/// or `00944460(0, 4)` returns 0. A pick is then passed by address to
/// `00905330` and `00905820` on the list. Returns the pick (0 for an empty
/// list).
pub fn fn_00595560(e: &mut Engine, _this: Ptr, list: Ptr) -> u32 {
    let mut node = list.addr();
    if node == 0 {
        return 0;
    }
    let slot = e.call(0x0068_15c0, &args![node]).u32();
    if e.mem.u32(slot) == 0 {
        return 0;
    }
    e.with_stack(4, |e, picked| {
        while node != 0 {
            let slot = e.call(0x0068_15c0, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let roll = e.call(0x0094_4460, &args![0u32, 4u32]).u32();
            if roll == 0 || e.mem.u32(picked.addr()) == 0 {
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                let item = e.mem.u32(slot);
                e.mem.set_u32(picked.addr(), item);
            }
            node = e.call(0x0072_6070, &args![node]).u32();
        }
        if e.mem.u32(picked.addr()) != 0 {
            e.call(0x0090_5330, &args![list, picked]);
            e.call(0x0090_5820, &args![list, picked]);
        }
        e.mem.u32(picked.addr())
    })
}

/// `00595600`'s "drop the current entry": `00595cb0` on `+0x50` when it is
/// set, then `+0x50` = 0.
fn drop_current(e: &mut Engine, this: Ptr) {
    let current = e.mem.u32(this.addr() + 0x50);
    if current != 0 {
        fn_00595cb0(e, Ptr::new(current));
    }
    e.mem.set_u32(this.addr() + 0x50, 0);
}

/// `00595600`'s pick of a list by kind: `fn_00595560` on the list at the
/// offset the jump table gives the kind, stored at `+0x50`; a kind past the
/// table leaves `+0x50` alone.
fn pick_for_kind(e: &mut Engine, this: Ptr, kind: u32, table: &[u32]) {
    if let Some(&offset) = table.get(kind as usize) {
        let picked = fn_00595560(e, this, Ptr::new(this.addr() + offset));
        e.mem.set_u32(this.addr() + 0x50, picked);
    }
}

// Translated from 00595600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-update step of `MediaLocationController`; `flag` is a byte the
/// caller passes (stored at `+0x36` and handed to `00597990`). Returns the
/// byte at `+0x34` (or 0 on the two early exits).
///
/// - With no object at `011dea3c`, when `00969930` (called
///   on that null pointer) gives a slot holding a non-null word: it returns 0.
/// - A controller other than the last one (`011dd374`) drops its entry
///   (`+0x50`); it becomes the last one. `now` is `0063d040` on
///   `00453a70()`. With `+0x34` clear the entry is dropped too.
/// - `kind` starts as `005fd3b0(+0x60, 0058db10(6))` when `+0x60` is set,
///   else 0. The ratio is 1000.0 (`01013974`), or the planar squared length
///   (`00595c80`) of `00439ef0` on the two slot-`0x1f4` points over the
///   squared `00568cb0` radius when that is not smaller.
/// - Over the list at `00595d10(011dea3c)` (until `008256d0`), each entry
///   whose word is set counts into `+0x44` when `+0x60` is set and
///   `008b8e90(entry, +0x60)` holds (its byte `+4` then makes `kind` 1), and
///   into `+0x40` when its byte `+4` is set.
/// - The change flags (`+0x37` against `00595d30`, `+0x48` against `kind`,
///   `+0x44` becoming zero or non-zero unless `kind` equals `00595cf0`) are
///   taken, then `+0x36`, `+0x35`, `+0x37`, `+0x48` and `+0x3c` are stored.
///   Two more comparisons the code makes (`+0x36` and `+0x35` against their
///   new values) are never read and have no effect.
/// - With the ratio below 100.0, `+0x24` above `now` and `+0x34` clear it
///   returns 0; otherwise `+0x34` = `00597990(+0x50, now, flag, this)`
///   (when `+0x50` is set), a change drops the entry, and with `+0x34` clear
///   and the ratio below 100.0 a new entry is picked (`00595560`) from the
///   list the kind selects (when an entry counted into `+0x44`) or the list
///   `00595cf0` selects. With no entry, or one whose `008041a0` is neither 0
///   nor 2, `+0x3c` below 100.0 and `flag` set, the entry is replaced by a
///   pick from `+0xb0` (when `011dd378` is 0). Then `00597990` runs once more
///   when `+0x34` is still clear; with `+0x34` set, `+0x24` becomes
///   `now + +0x7c * 1000` (truncated), else a ratio above 1.0 clears
///   `011dd374`.
pub fn fn_00595600(e: &mut Engine, this: Ptr, flag: u8) -> u8 {
    let base = this.addr();
    let player = e.global::<u32>(PLAYER_POINTER_011DEA3C);
    if player == 0 {
        let slot = e.call(0x0096_9930, &args![player]).u32();
        if slot != 0 {
            let slot = e.call(0x0096_9930, &args![player]).u32();
            if e.mem.u32(slot) != 0 {
                return 0;
            }
        }
    }
    if e.global::<u32>(LAST_CONTROLLER_011DD374) != base {
        drop_current(e, this);
    }
    e.set_global(LAST_CONTROLLER_011DD374, base);
    let audio = e.call(0x0045_3a70, &args![]).u32();
    let now = e.call(0x0063_d040, &args![audio]).u32();
    if e.mem.u8(base + 0x34) == 0 {
        drop_current(e, this);
    }
    let faction = e.mem.u32(base + 0x60);
    let mut kind = if faction != 0 {
        let default_object = e.call(0x0058_db10, &args![6u32]).u32();
        e.call(0x005f_d3b0, &args![faction, default_object]).u32()
    } else {
        0
    };

    let slot = e.call(0x0096_9930, &args![player]).u32();
    let near = e.mem.u32(slot);
    let radius = e.call(0x0056_8cb0, &args![near]).f32();
    let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
    let distance_squared = e.with_stack(0x18, |e, points| {
        let near_point = points.addr();
        let difference = points.addr() + 0xc;
        let position = e.vcall(near, 0x1f4, &args![]).u32();
        for i in 0..3 {
            let w = e.mem.u32(position + 4 * i);
            e.mem.set_u32(near_point + 4 * i, w);
        }
        let player_position = e.vcall(player, 0x1f4, &args![]).u32();
        e.call(0x0043_9ef0, &args![near_point, difference, player_position]);
        fn_00595c80(e, Ptr::new(difference))
    });
    let mut ratio = x87_quiet(e.global::<f32>(RATIO_START_01013974));
    if fcom(f64::from(radius_squared), f64::from(distance_squared)) & 0x01 == 0 {
        ratio = (f64::from(distance_squared) / f64::from(radius_squared)) as f32;
    }

    let mut node = fn_00595d10(e, Ptr::new(player));
    let mut counted_members = false;
    let had_members = e.mem.u32(base + 0x44) != 0;
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x40, 0);
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && e.mem.u32(entry) != 0 {
            let faction = e.mem.u32(base + 0x60);
            let member = e.mem.u32(entry);
            if faction != 0 && e.call(0x008b_8e90, &args![member, faction]).bool() {
                let count = e.mem.u32(base + 0x44);
                e.mem.set_u32(base + 0x44, count.wrapping_add(1));
                counted_members = true;
                if e.mem.u8(entry + 4) != 0 {
                    kind = 1;
                }
            }
            if e.mem.u8(entry + 4) != 0 {
                let count = e.mem.u32(base + 0x40);
                e.mem.set_u32(base + 0x40, count.wrapping_add(1));
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    let mut members_changed = had_members != (e.mem.u32(base + 0x44) != 0);
    let stored_media = e.mem.u8(base + 0x37);
    let media_changed = stored_media != u8::from(fn_00595d30(e, this));
    let kind_changed = e.mem.u32(base + 0x48) != kind;
    if kind == fn_00595cf0(e, this) {
        members_changed = false;
    }
    e.mem.set_u8(base + 0x36, flag);
    let any_flagged = e.mem.u32(base + 0x40) != 0;
    e.mem.set_u8(base + 0x35, u8::from(any_flagged));
    let media = fn_00595d30(e, this);
    e.mem.set_u8(base + 0x37, u8::from(media));
    e.mem.set_u32(base + 0x48, kind);
    e.mem.set_f32(base + 0x3c, x87_quiet(ratio));

    let limit = e.global::<f64>(RATIO_LIMIT_01017A40);
    let below_limit = |value: f32| !parity_even(fcom(f64::from(value), limit), 0x05);
    if below_limit(e.mem.f32(base + 0x3c))
        && e.mem.u32(base + 0x24) > now
        && e.mem.u8(base + 0x34) == 0
    {
        return 0;
    }
    e.mem.set_u8(base + 0x34, 0);
    let current = e.mem.u32(base + 0x50);
    if current != 0 {
        let busy = e
            .call(0x0059_7990, &args![current, now, u32::from(flag), this])
            .u8();
        e.mem.set_u8(base + 0x34, busy);
    }
    if media_changed || kind_changed || members_changed {
        drop_current(e, this);
    }
    if e.mem.u8(base + 0x34) == 0 && below_limit(ratio) {
        if e.mem.u32(base + 0x50) != 0 {
            drop_current(e, this);
        }
        if e.mem.u32(base + 0x50) == 0 {
            if counted_members {
                pick_for_kind(e, this, kind, &[0x88, 0xa0, 0x90, 0x98]);
            } else {
                let selected = fn_00595cf0(e, this);
                pick_for_kind(e, this, selected, &[0x88, 0xa0, 0x90, 0x98, 0xa8]);
            }
        }
    }
    let current = e.mem.u32(base + 0x50);
    let replace = current == 0 || {
        let location = e.call(0x0080_41a0, &args![current]).u32();
        location != 0 && e.call(0x0080_41a0, &args![current]).u32() != 2
    };
    if replace && below_limit(e.mem.f32(base + 0x3c)) && flag != 0 {
        if e.mem.u32(base + 0x50) != 0 {
            drop_current(e, this);
        }
        if e.global::<u32>(BLOCK_LAST_LIST_011DD378) == 0 {
            let picked = fn_00595560(e, this, Ptr::new(base + 0xb0));
            e.mem.set_u32(base + 0x50, picked);
        }
    }
    let current = e.mem.u32(base + 0x50);
    if e.mem.u8(base + 0x34) == 0 && current != 0 {
        let busy = e
            .call(0x0059_7990, &args![current, now, u32::from(flag), this])
            .u8();
        e.mem.set_u8(base + 0x34, busy);
    }
    if e.mem.u8(base + 0x34) != 0 {
        let delay = f64::from(e.mem.f32(base + 0x7c)) * e.global::<f64>(THOUSAND_01017B70);
        let until = f64::from(now) + delay;
        e.mem.set_u32(base + 0x24, fistp_truncate_low(until));
    }
    if e.mem.u8(base + 0x34) == 0
        && fcom(
            f64::from(e.mem.f32(base + 0x3c)),
            e.global::<f64>(ONE_01012070),
        ) & 0x41
            == 0
    {
        e.set_global(LAST_CONTROLLER_011DD374, 0u32);
    }
    e.mem.u8(base + 0x34)
}

// Translated from 00595c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `x * x + y * y` of the point at `this` (its first two floats), rounded to
/// `float`.
pub fn fn_00595c80(e: &mut Engine, this: Ptr) -> f32 {
    let x = f64::from(e.mem.f32(this.addr()));
    let y = f64::from(e.mem.f32(this.addr() + 4));
    x87_quiet((x * x + y * y) as f32)
}

// Translated from 00595cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bytes `+0x30`..`+0x34` and the word `+0x28` of the entry
/// object `00595600` keeps at `+0x50` (its class is unconfirmed).
pub fn fn_00595cb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x30, 0);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u8(base + 0x31, 0);
    e.mem.set_u8(base + 0x32, 0);
    e.mem.set_u8(base + 0x33, 0);
    e.mem.set_u8(base + 0x34, 0);
}

// Translated from 00595cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low four bits of the word at `+0x6c`.
pub fn fn_00595cf0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6c) & 0xf
}

// Translated from 00595d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xd48` (`00595600` calls it on the object at `011dea3c`).
pub fn fn_00595d10(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xd48)
}

// Translated from 00595d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the float `00966a20` returns on `008d8520([011dea10])` lies
/// between a low and a high bound: with bit 6 of `+0x6c` set, the averages
/// (over 2.0) of `00595ea0` + `00595f50` and of `00595fc0` + `00596030` on
/// that object; otherwise `+0x80` and `+0x84` scaled by the double at
/// `01034200`. True when `008d8520` gives null; false when the float is
/// below the low bound or above the high one, or a comparison is unordered.
pub fn fn_00595d30(e: &mut Engine, this: Ptr) -> bool {
    let owner = e.global::<u32>(OWNER_POINTER_011DEA10);
    let target = e.call(0x008d_8520, &args![owner]).u32();
    if fn_00595e80(e, this) {
        if target == 0 {
            return true;
        }
        let two = e.global::<f64>(TWO_01011590);
        let first = f64::from(fn_00595ea0(e, Ptr::new(target)));
        let second = f64::from(fn_00595f50(e, Ptr::new(target)));
        let low = (second + first) / two;
        let value = e.call(0x0096_6a20, &args![target]).f64();
        if fcom(value, low) & 0x01 != 0 {
            return false;
        }
        let value = e.call(0x0096_6a20, &args![target]).f64();
        let third = f64::from(fn_00595fc0(e, Ptr::new(target)));
        let fourth = f64::from(fn_00596030(e, Ptr::new(target)));
        let high = (fourth + third) / two;
        fcom(high, value) & 0x01 == 0
    } else {
        if target == 0 {
            return true;
        }
        let scale = e.global::<f64>(SCALE_01034200);
        let low = f64::from(e.mem.u32(this.addr() + 0x80)) * scale;
        let value = e.call(0x0096_6a20, &args![target]).f64();
        if fcom(value, low) & 0x01 != 0 {
            return false;
        }
        let value = e.call(0x0096_6a20, &args![target]).f64();
        let high = f64::from(e.mem.u32(this.addr() + 0x84)) * scale;
        fcom(high, value) & 0x01 == 0
    }
}

// Translated from 00595e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 6 of the word at `+0x6c`.
pub fn fn_00595e80(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u32(this.addr() + 0x6c) >> 6) & 1 != 0
}

/// The shared body of `00595ea0`, `00595f50`, `00595fc0` and `00596030`:
/// when bit `dirty` of `+0x118` is set and `+0xc` is non-null, the cached
/// float at `cache` becomes `00595f10(+0xc, index) / 6.0` and the bit is
/// cleared. Returns the cached float.
fn cached_level(e: &mut Engine, this: Ptr, dirty: u32, index: i32, cache: u32) -> f32 {
    let base = this.addr();
    let flags = e.mem.u32(base + 0x118);
    let source = e.mem.u32(base + 0xc);
    if flags & dirty != 0 && source != 0 {
        let level = fn_00595f10(e, Ptr::new(source), index);
        let level = f64::from(level) / e.global::<f64>(DIVISOR_01034208);
        e.set_global(cache, level as f32);
        let flags = e.mem.u32(base + 0x118);
        e.mem.set_u32(base + 0x118, flags & !dirty);
    }
    x87_quiet(e.global::<f32>(cache))
}

// Translated from 00595ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Level 0 of the object at `+0xc` over 6.0, cached at `011ca9e8` and
/// refreshed while bit `0x100` of `+0x118` is set.
pub fn fn_00595ea0(e: &mut Engine, this: Ptr) -> f32 {
    cached_level(e, this, 0x100, 0, CACHED_LEVEL_011CA9E8)
}

// Translated from 00595f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x50 + index` for `index` 0..=3, else 0.
pub fn fn_00595f10(e: &mut Engine, this: Ptr, index: i32) -> u8 {
    if (0..4).contains(&index) {
        e.mem.u8(this.addr().wrapping_add(index as u32) + 0x50)
    } else {
        0
    }
}

// Translated from 00595f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Level 1 of the object at `+0xc` over 6.0, cached at `011ca9ec` and
/// refreshed while bit `0x200` of `+0x118` is set.
pub fn fn_00595f50(e: &mut Engine, this: Ptr) -> f32 {
    cached_level(e, this, 0x200, 1, CACHED_LEVEL_011CA9E8 + 4)
}

// Translated from 00595fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Level 2 of the object at `+0xc` over 6.0, cached at `011ca9f0` and
/// refreshed while bit `0x400` of `+0x118` is set.
pub fn fn_00595fc0(e: &mut Engine, this: Ptr) -> f32 {
    cached_level(e, this, 0x400, 2, CACHED_LEVEL_011CA9E8 + 8)
}

// Translated from 00596030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Level 3 of the object at `+0xc` over 6.0, cached at `011ca9f4` and
/// refreshed while bit `0x800` of `+0x118` is set.
pub fn fn_00596030(e: &mut Engine, this: Ptr) -> f32 {
    cached_level(e, this, 0x800, 3, CACHED_LEVEL_011CA9E8 + 0xc)
}

// Translated from 005960a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005960a0 - the exception-unwinding frame (FS:[0]) and its security cookie are not translated.
/// Destructor of `MediaLocationController`: stores its two vtable pointers,
/// runs `0046ffb0` on the six lists `+0xb0` down to `+0x88`, `00591300` on
/// `+0x54` and `+0x18`, then the base destructor `00483630`.
pub fn fn_005960a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_40c4);
    e.mem.set_u32(base + 0x18, 0x0103_40ac);
    for offset in [0xb0, 0xa8, 0xa0, 0x98, 0x90, 0x88] {
        e.call(0x0046_ffb0, &args![base + offset]);
    }
    e.call(0x0059_1300, &args![base + 0x54]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_5310, fn_00595310(Ptr) -> Ptr),
        entry!(0x0059_5530, fn_00595530(Ptr, u32) -> Ptr),
        entry!(0x0059_5560, fn_00595560(Ptr, Ptr) -> u32),
        entry!(0x0059_5600, fn_00595600(Ptr, u8) -> u8),
        entry!(0x0059_5c80, fn_00595c80(Ptr) -> f32),
        entry!(0x0059_5cb0, fn_00595cb0(Ptr)),
        entry!(0x0059_5cf0, fn_00595cf0(Ptr) -> u32),
        entry!(0x0059_5d10, fn_00595d10(Ptr) -> u32),
        entry!(0x0059_5d30, fn_00595d30(Ptr) -> bool),
        entry!(0x0059_5e80, fn_00595e80(Ptr) -> bool),
        entry!(0x0059_5ea0, fn_00595ea0(Ptr) -> f32),
        entry!(0x0059_5f10, fn_00595f10(Ptr, i32) -> u8),
        entry!(0x0059_5f50, fn_00595f50(Ptr) -> f32),
        entry!(0x0059_5fc0, fn_00595fc0(Ptr) -> f32),
        entry!(0x0059_6030, fn_00596030(Ptr) -> f32),
        entry!(0x0059_60a0, fn_005960a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Seen = Rc<RefCell<Vec<u32>>>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn object(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// List doubles: a node is `{item, next}`; `006815c0` gives the node,
    /// `00726070` its next word, `008256d0` is never empty.
    fn register_list(e: &mut Engine) {
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0082_56d0, |_, _| false.into_ret());
    }

    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let n = e.mem.alloc(8);
        e.mem.set_u32(n, item);
        e.mem.set_u32(n + 4, next);
        n
    }

    fn map_globals(e: &mut Engine) {
        e.map(0x011d_ea10, 0x30);
        e.map(0x011d_d374, 8);
        e.map(CACHED_LEVEL_011CA9E8, 0x10);
        for (addr, len) in [
            (RATIO_START_01013974, 4),
            (RATIO_LIMIT_01017A40, 8),
            (THOUSAND_01017B70, 8),
            (ONE_01012070, 8),
            (TWO_01011590, 8),
            (SCALE_01034200, 0x10),
        ] {
            e.map(addr, len);
        }
        e.set_global(RATIO_START_01013974, 1000.0f32);
        e.set_global(RATIO_LIMIT_01017A40, 100.0f64);
        e.set_global(THOUSAND_01017B70, 1000.0f64);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(TWO_01011590, 2.0f64);
        e.set_global(SCALE_01034200, 0.09375f64);
        e.set_global(DIVISOR_01034208, 6.0f64);
    }

    #[test]
    fn fn_00595530_destroys_and_frees_on_bit_0() {
        for (flags, frees) in [(1u32, 1usize), (0, 0)] {
            let mut e = Engine::new();
            for addr in [0x0046_ffb0, 0x0059_1300, 0x0048_3630, 0x0040_1030] {
                e.register(addr, nothing);
            }
            let this = object(&mut e, 0xb8);
            e.call_log = Some(vec![]);
            let r = e.call(0x0059_5530, &args![this, flags]).u32();
            assert_eq!(r, this.addr());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0040_1030).len(), frees);
            assert_eq!(calls_to(&log, 0x0048_3630), vec![vec![this.addr()]]);
        }
    }

    #[test]
    fn fn_005960a0_tears_down_in_order() {
        let mut e = Engine::new();
        for addr in [0x0046_ffb0, 0x0059_1300, 0x0048_3630] {
            e.register(addr, nothing);
        }
        let this = object(&mut e, 0xb8);
        let b = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x0059_60a0, &args![this]);
        assert_eq!(e.mem.u32(b), 0x0103_40c4);
        assert_eq!(e.mem.u32(b + 0x18), 0x0103_40ac);
        let log: Log = e.call_log.take().unwrap().into_iter().skip(1).collect();
        let expected: Log = vec![
            (0x0046_ffb0, vec![b + 0xb0]),
            (0x0046_ffb0, vec![b + 0xa8]),
            (0x0046_ffb0, vec![b + 0xa0]),
            (0x0046_ffb0, vec![b + 0x98]),
            (0x0046_ffb0, vec![b + 0x90]),
            (0x0046_ffb0, vec![b + 0x88]),
            (0x0059_1300, vec![b + 0x54]),
            (0x0059_1300, vec![b + 0x18]),
            (0x0048_3630, vec![b]),
        ];
        assert_eq!(log, expected);
    }

    #[test]
    fn fn_00595560_picks_an_entry() {
        let mut e = Engine::new();
        register_list(&mut e);
        let rolls = Rc::new(RefCell::new(vec![1u32, 0, 1]));
        let r = rolls.clone();
        e.register_double(0x0094_4460, move |_, a| {
            assert_eq!(a, [0, 4]);
            r.borrow_mut().remove(0).into_ret()
        });
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        for addr in [0x0090_5330u32, 0x0090_5820] {
            let s = seen.clone();
            e.register_double(addr, move |e, a| {
                s.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
        }
        // Null list and a list whose first item is null: 0, no calls.
        assert_eq!(e.call(0x0059_5560, &args![0u32, 0u32]).u32(), 0);
        let empty = node(&mut e, 0, 0);
        assert_eq!(e.call(0x0059_5560, &args![0u32, empty]).u32(), 0);
        assert!(seen.borrow().is_empty());
        // Items 0x11 (first pick), 0x22 (roll 0 replaces), 0x33 (roll 1 keeps).
        let third = node(&mut e, 0x33, 0);
        let second = node(&mut e, 0x22, third);
        let list = node(&mut e, 0x11, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5560, &args![0u32, list]).u32(), 0x22);
        assert_eq!(*seen.borrow(), vec![0x22, 0x22]);
        assert!(rolls.borrow().is_empty());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0090_5330)[0][0], list);
        assert_eq!(calls_to(&log, 0x0090_5820)[0][0], list);
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        let this = object(&mut e, 0xd50);
        let b = this.addr();
        e.mem.set_f32(b, 3.0);
        e.mem.set_f32(b + 4, 4.0);
        e.mem.set_f32(b + 8, 100.0);
        assert_eq!(e.call(0x0059_5c80, &args![this]).f32(), 25.0);
        e.mem.set_u32(b + 0x6c, 0x47);
        assert_eq!(e.call(0x0059_5cf0, &args![this]).u32(), 7);
        assert!(e.call(0x0059_5e80, &args![this]).bool());
        e.mem.set_u32(b + 0x6c, 0x3f);
        assert!(!e.call(0x0059_5e80, &args![this]).bool());
        e.mem.set_u32(b + 0xd48, 0xabcd);
        assert_eq!(e.call(0x0059_5d10, &args![this]).u32(), 0xabcd);
        for i in 0..4u32 {
            e.mem.set_u8(b + 0x50 + i, 10 + i as u8);
        }
        assert_eq!(e.call(0x0059_5f10, &args![this, 0i32]).u8(), 10);
        assert_eq!(e.call(0x0059_5f10, &args![this, 3i32]).u8(), 13);
        assert_eq!(e.call(0x0059_5f10, &args![this, 4i32]).u8(), 0);
        assert_eq!(e.call(0x0059_5f10, &args![this, -1i32]).u8(), 0);
    }

    #[test]
    fn fn_00595cb0_clears_the_entry_state() {
        let mut e = Engine::new();
        let this = object(&mut e, 0x40);
        let b = this.addr();
        for off in 0x24..0x38 {
            e.mem.set_u8(b + off, 0xff);
        }
        e.call(0x0059_5cb0, &args![this]);
        assert_eq!(e.mem.u32(b + 0x28), 0);
        for off in 0x30..0x35 {
            assert_eq!(e.mem.u8(b + off), 0);
        }
        assert_eq!(e.mem.u32(b + 0x24), 0xffff_ffff);
        assert_eq!(e.mem.u8(b + 0x35), 0xff);
        assert_eq!(e.mem.u32(b + 0x2c), 0xffff_ffff);
    }

    #[test]
    fn cached_levels_refresh_on_their_bits() {
        for (addr, bit, index) in [
            (0x0059_5ea0u32, 0x100u32, 0u32),
            (0x0059_5f50, 0x200, 1),
            (0x0059_5fc0, 0x400, 2),
            (0x0059_6030, 0x800, 3),
        ] {
            let mut e = Engine::new();
            map_globals(&mut e);
            let cache = CACHED_LEVEL_011CA9E8 + 4 * index;
            e.set_global(cache, 7.0f32);
            let this = object(&mut e, 0x120);
            let source = object(&mut e, 0x60);
            e.mem.set_u8(source.addr() + 0x50 + index, 3);
            // Bit clear: the cached value.
            e.mem.set_u32(this.addr() + 0xc, source.addr());
            assert_eq!(e.call(addr, &args![this]).f32(), 7.0);
            // Bit set but no source: still cached, the bit stays.
            e.mem.set_u32(this.addr() + 0x118, bit | 0x1);
            e.mem.set_u32(this.addr() + 0xc, 0);
            assert_eq!(e.call(addr, &args![this]).f32(), 7.0);
            assert_eq!(e.mem.u32(this.addr() + 0x118), bit | 0x1);
            // Refreshed: 3 / 6.
            e.mem.set_u32(this.addr() + 0xc, source.addr());
            assert_eq!(e.call(addr, &args![this]).f32(), 0.5);
            assert_eq!(e.global::<f32>(cache), 0.5);
            assert_eq!(e.mem.u32(this.addr() + 0x118), 0x1);
        }
    }

    #[test]
    fn fn_00595d30_compares_with_the_bounds() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let owner = object(&mut e, 0x80);
        e.set_global(OWNER_POINTER_011DEA10, owner.addr());
        let this = object(&mut e, 0xb8);
        let target = object(&mut e, 0x120);
        e.register(0x008d_8520, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        let value = Rc::new(RefCell::new(0.0f32));
        let v = value.clone();
        e.register_double(0x0096_6a20, move |_, _| (*v.borrow()).into_ret());
        // No target: true on both branches.
        assert!(e.call(0x0059_5d30, &args![this]).bool());
        e.mem.set_u32(this.addr() + 0x6c, 0x40);
        assert!(e.call(0x0059_5d30, &args![this]).bool());

        e.mem.set_u32(owner.addr() + 0x68, target.addr());
        // Bit 6: bounds (1 + 2) / 2 = 1.5 and (3 + 4) / 2 = 3.5 from the cache.
        for (i, level) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.set_global(CACHED_LEVEL_011CA9E8 + 4 * i as u32, level);
        }
        for (v, expected) in [(1.0f32, false), (1.5, true), (3.5, true), (3.6, false)] {
            *value.borrow_mut() = v;
            assert_eq!(e.call(0x0059_5d30, &args![this]).bool(), expected, "{v}");
        }
        *value.borrow_mut() = f32::NAN;
        assert!(!e.call(0x0059_5d30, &args![this]).bool());
        // Bit 6 clear: +0x80 = 16 → 1.5, +0x84 = 32 → 3.0.
        e.mem.set_u32(this.addr() + 0x6c, 0);
        e.mem.set_u32(this.addr() + 0x80, 16);
        e.mem.set_u32(this.addr() + 0x84, 32);
        for (v, expected) in [(1.0f32, false), (1.5, true), (3.0, true), (3.5, false)] {
            *value.borrow_mut() = v;
            assert_eq!(e.call(0x0059_5d30, &args![this]).bool(), expected, "{v}");
        }
    }

    /// The world `00595600` runs in: a player with a list of one flagged
    /// entry at `+0xd48`, a near object (`00969930`) with radius
    /// `radius`, points (3, 4, 0) and the origin, `now` = 5000.
    struct World {
        e: Engine,
        this: Ptr,
        first_pick: u32,
        last_pick: u32,
    }

    fn world(radius: f32) -> World {
        let mut e = Engine::new();
        map_globals(&mut e);
        register_list(&mut e);
        let player = object(&mut e, 0xd50);
        let near = object(&mut e, 0x40);
        e.set_global(PLAYER_POINTER_011DEA3C, player.addr());
        let vtable = 0x0f00_0000;
        let position_fn = 0x0f10_0000;
        let mut slots = vec![0u32; 0x1f4 / 4 + 1];
        slots[0x1f4 / 4] = position_fn;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(player.addr(), vtable);
        e.mem.set_u32(near.addr(), vtable);
        let near_point = e.mem.alloc(12);
        e.mem.set_f32(near_point, 3.0);
        e.mem.set_f32(near_point + 4, 4.0);
        let player_point = e.mem.alloc(12);
        let near_addr = near.addr();
        e.register_double(position_fn, move |_, a| {
            let point = if a[0] == near_addr {
                near_point
            } else {
                player_point
            };
            point.into_ret()
        });
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, near.addr());
        e.register_double(0x0096_9930, move |_, _| slot.into_ret());
        e.register_double(0x0056_8cb0, move |_, _| radius.into_ret());
        e.register(0x0043_9ef0, |e, a| {
            for i in 0..3 {
                let v = f64::from(e.mem.f32(a[0] + 4 * i)) - f64::from(e.mem.f32(a[2] + 4 * i));
                e.mem.set_f32(a[1] + 4 * i, v as f32);
            }
            Ret::default()
        });
        e.register(0x0045_3a70, |_, _| 0x1234u32.into_ret());
        e.register(0x0063_d040, |_, _| 5000u32.into_ret());
        e.register(0x008d_8520, |_, _| 0u32.into_ret());
        e.register(0x0094_4460, |_, _| 1u32.into_ret());
        e.register(0x0090_5330, nothing);
        e.register(0x0090_5820, nothing);
        e.register(0x0080_41a0, |_, _| 1u32.into_ret());
        e.register(0x0059_7990, |_, _| 1u32.into_ret());
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x77);
        e.mem.set_u8(entry + 4, 1);
        let list = node(&mut e, entry, 0);
        e.mem.set_u32(player.addr() + 0xd48, list);

        let this = object(&mut e, 0xb8);
        let b = this.addr();
        let first_pick = e.mem.alloc(0x40);
        let last_pick = e.mem.alloc(0x40);
        e.mem.set_u32(b + 0x90, first_pick);
        e.mem.set_u32(b + 0xb0, last_pick);
        e.mem.set_u32(b + 0x6c, 2);
        e.mem.set_f32(b + 0x7c, 0.5);
        World {
            e,
            this,
            first_pick,
            last_pick,
        }
    }

    #[test]
    fn fn_00595600_stops_without_a_player() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 1);
        e.register_double(0x0096_9930, move |_, a| {
            assert_eq!(a, [0]);
            slot.into_ret()
        });
        let this = object(&mut e, 0xb8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5600, &args![this, 1u32]).u8(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn fn_00595600_picks_and_starts_an_entry() {
        let World {
            mut e,
            this,
            first_pick,
            last_pick,
        } = world(10.0);
        let b = this.addr();
        e.mem.set_u8(first_pick + 0x30, 0xff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5600, &args![this, 1u32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        // Ratio 25 / 100; the kind-2 list (+0x90) was picked, then replaced
        // (008041a0 is 1) by the +0xb0 list, which 00597990 started.
        assert_eq!(e.mem.f32(b + 0x3c), 0.25);
        assert_eq!(e.mem.u8(first_pick + 0x30), 0);
        assert_eq!(e.mem.u32(b + 0x50), last_pick);
        assert_eq!(
            calls_to(&log, 0x0059_7990),
            vec![vec![last_pick, 5000, 1, b]]
        );
        assert_eq!(e.mem.u8(b + 0x34), 1);
        assert_eq!(e.mem.u32(b + 0x24), 5500);
        assert_eq!(e.mem.u8(b + 0x35), 1);
        assert_eq!(e.mem.u8(b + 0x36), 1);
        assert_eq!(e.mem.u8(b + 0x37), 1);
        assert_eq!(e.mem.u32(b + 0x40), 1);
        assert_eq!(e.mem.u32(b + 0x44), 0);
        assert_eq!(e.global::<u32>(LAST_CONTROLLER_011DD374), b);
        assert!(calls_to(&log, 0x0058_db10).is_empty());
    }

    #[test]
    fn fn_00595600_waits_until_its_time() {
        let World { mut e, this, .. } = world(10.0);
        let b = this.addr();
        e.mem.set_u32(b + 0x24, 9999);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5600, &args![this, 1u32]).u8(), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0059_7990).is_empty());
        assert_eq!(e.mem.u32(b + 0x50), 0);
        assert_eq!(e.mem.u32(b + 0x24), 9999);
    }

    #[test]
    fn fn_00595600_far_away_clears_the_last_controller() {
        // Radius 1: the squared length 25 is above 1, the ratio stays 1000.
        let World { mut e, this, .. } = world(1.0);
        let b = this.addr();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5600, &args![this, 1u32]).u8(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(b + 0x3c), 1000.0);
        assert_eq!(e.mem.u32(b + 0x50), 0);
        assert!(calls_to(&log, 0x0059_7990).is_empty());
        assert_eq!(e.global::<u32>(LAST_CONTROLLER_011DD374), 0);
    }

    #[test]
    fn fn_00595600_counts_faction_members() {
        let World {
            mut e,
            this,
            first_pick,
            ..
        } = world(10.0);
        let b = this.addr();
        let faction = e.mem.alloc(4);
        e.mem.set_u32(b + 0x60, faction);
        e.register(0x0058_db10, |_, _| 0x99u32.into_ret());
        e.register(0x005f_d3b0, |_, _| 3u32.into_ret());
        e.register(0x008b_8e90, |_, _| true.into_ret());
        // 008041a0 = 2 keeps the pick; the kind-1 list (+0xa0) is chosen.
        e.register(0x0080_41a0, |_, _| 2u32.into_ret());
        e.mem.set_u32(b + 0xa0, first_pick);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_5600, &args![this, 1u32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0058_db10), vec![vec![6]]);
        assert_eq!(calls_to(&log, 0x005f_d3b0), vec![vec![faction, 0x99]]);
        assert_eq!(calls_to(&log, 0x008b_8e90), vec![vec![0x77, faction]]);
        assert_eq!(e.mem.u32(b + 0x44), 1);
        assert_eq!(e.mem.u32(b + 0x48), 1);
        assert_eq!(e.mem.u32(b + 0x50), first_pick);
    }

    #[test]
    fn fn_00595310_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_5310,
            size: 0xb8,
            globals: &[(0x0102_6994, 0x0100_0001), (0x0103_40a4, 0x0100_0002)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2d00, &[A::T(0x54)]),
                (0x0096_a2d0, &[A::T(0x88)]),
                (0x0096_a2d0, &[A::T(0x90)]),
                (0x0096_a2d0, &[A::T(0x98)]),
                (0x0096_a2d0, &[A::T(0xa0)]),
                (0x0096_a2d0, &[A::T(0xa8)]),
                (0x0096_a2d0, &[A::T(0xb0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x70)]),
                (0x0047_0470, &[A::T(0x88)]),
                (0x0047_0470, &[A::T(0x90)]),
                (0x0047_0470, &[A::T(0x98)]),
                (0x0047_0470, &[A::T(0xa0)]),
                (0x0047_0470, &[A::T(0xa8)]),
                (0x0047_0470, &[A::T(0xb0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_40c4),
                (0x18, 4, 0x0103_40ac),
                (0x6c, 4, 0x4),
                (0x70, 4, 0x0100_0001),
                (0x74, 4, 0x0100_0002),
                (0x7c, 4, 0x0),
                (0x80, 4, 0x40),
                (0x84, 4, 0x150),
                (0x60, 4, 0x0),
                (0x64, 4, 0x1),
                (0x68, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0),
                (0x2c, 4, 0x0),
                (0x30, 4, 0x0),
                (0x35, 1, 0x0),
                (0x36, 1, 0x0),
                (0x37, 1, 0x0),
                (0x38, 1, 0x0),
                (0x4c, 1, 0x0),
                (0x50, 4, 0x0),
                (0x40, 4, 0x0),
                (0x44, 4, 0x0),
                (0x48, 4, 0x1),
            ],
        });
    }
}

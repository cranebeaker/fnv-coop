//! `fallout/interface/animatingtext.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `AnimatingText` (Xbox PDB), 0x2c bytes: the full text (a
    /// `BSStringT<char>` at +0), the tile values it drives, its intervals
    /// and the buffer the text is revealed into.
    pub struct AnimatingText: 0x2c {
        /// `pStringTrait` (`Tile::Value *`).
        0x08 pStringTrait: u32,
        /// `pVisTrait` (`Tile::Value *`).
        0x0C pVisTrait: u32,
        /// `dwDisplayInterval`.
        0x10 dwDisplayInterval: u32,
        /// `dwFlashOnDuration`.
        0x14 dwFlashOnDuration: u32,
        /// `dwFlashOffDuration`.
        0x18 dwFlashOffDuration: u32,
        /// `dwNextDisplayUpdate`.
        0x1C dwNextDisplayUpdate: u32,
        /// `dwNextFlashUpdate`.
        0x20 dwNextFlashUpdate: u32,
        /// `pBuffer` (`char *`).
        0x24 pBuffer: u32,
        /// `ucAnimationTypes`.
        0x28 ucAnimationTypes: u8,
        /// `bDisplayFinished`.
        0x29 bDisplayFinished: u8,
    }
}

/// Cdecl: frees a block (`00401030`).
const FREE: u32 = 0x0040_1030;
/// Thiscall on a string: its characters (`00559450`, the word at +0).
const STRING_TEXT: u32 = 0x0055_9450;
/// Thiscall on a string: its length (`004048e0`, the word at +4, 0 without
/// text).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// Cdecl (size, word): allocates (`005aaee0`, `tesscript.cpp`).
const ALLOCATE_005AAEE0: u32 = 0x005a_aee0;
/// Thiscall on a string (text), `RET 4`: sets it (`00438390`).
const STRING_SET_00438390: u32 = 0x0043_8390;
/// No arguments: the current time in milliseconds (`00457fe0`, `tes.cpp`).
const CURRENT_TIME: u32 = 0x0045_7fe0;
/// `Tile::Value::SetFloat` (value, word) and `Tile::Value::SetString`
/// (text, word) (engine map), thiscall, `RET 8`.
const TILE_VALUE_SET_FLOAT: u32 = 0x00a0_a270;
const TILE_VALUE_SET_STRING: u32 = 0x00a0_a300;
/// Thiscall (buffer), `RET 4`: `AL` (`00700270`, this unit, not in this
/// file's list).
const BUFFER_TEST_00700270: u32 = 0x0070_0270;
/// Cdecl (text): its length (`0044a670`).
const TEXT_LENGTH_0044A670: u32 = 0x0044_a670;
/// Cdecl (destination, source, count): copies characters (`00401460`).
const COPY_CHARACTERS_00401460: u32 = 0x0040_1460;
/// `1.0` (`double`), the visibility value `Update` compares with.
const ONE_DOUBLE: u32 = 0x0101_2070;

/// `Tile::Value::SetFloat(value, 1)` on `tile_value`.
fn set_float(e: &mut Engine, tile_value: u32, value: f32) {
    e.call(
        TILE_VALUE_SET_FLOAT,
        &args![tile_value, value.to_bits(), 1u32],
    );
}

// Translated from 006ffda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): frees the buffer
/// (`pBuffer`), allocates a new one of the text's length plus one
/// (`005aaee0` with 1) when there is text (else 0), and clears
/// `bDisplayFinished` and `dwNextDisplayUpdate`.
pub fn fn_006ffda0(e: &mut Engine, this: Ptr<AnimatingText>) {
    let buffer = e.get(this, AnimatingText::pBuffer);
    e.call(FREE, &args![buffer]);
    let buffer = if e.call(STRING_TEXT, &args![this]).u32() != 0 {
        let length = e.call(STRING_LENGTH, &args![this]).u32();
        e.call(ALLOCATE_005AAEE0, &args![length.wrapping_add(1), 1u32])
            .u32()
    } else {
        0
    };
    e.set(this, AnimatingText::pBuffer, buffer);
    e.set(this, AnimatingText::bDisplayFinished, 0);
    e.set(this, AnimatingText::dwNextDisplayUpdate, 0);
}

/// `Update` and `ForceFinish`'s first steps: shows the text tile
/// (`pVisTrait` set to 1.0) unless the text flashes, and hides it (0.0) when
/// there is a string tile but no text.
fn show_tiles(e: &mut Engine, this: Ptr<AnimatingText>) {
    if e.get(this, AnimatingText::pVisTrait) != 0 && !fn_007000d0(e, this) {
        let vis = e.get(this, AnimatingText::pVisTrait);
        set_float(e, vis, 1.0);
    }
    if e.get(this, AnimatingText::pVisTrait) != 0
        && e.get(this, AnimatingText::pStringTrait) != 0
        && e.call(STRING_TEXT, &args![this]).u32() == 0
    {
        let vis = e.get(this, AnimatingText::pVisTrait);
        set_float(e, vis, 0.0);
    }
}

// Translated from 006ffea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimatingText::Update` (engine map; `thiscall`, plain `RET`). For a
/// typing animation (type 1) that is due (`dwNextDisplayUpdate` reached)
/// and not finished: the tiles are shown ([`show_tiles`]); with a string
/// tile and a buffer that `00700270` accepts, the next characters are
/// revealed (one plus one per display interval elapsed since the last
/// update, at most the rest of the text, which finishes it), copied into the
/// buffer after its current length and set on the string tile; otherwise
/// it is finished. The next update is due a display interval later. For a
/// flashing animation (type 2) that is due (`dwNextFlashUpdate` reached),
/// the visibility flips: from exactly 1.0 to 0.0 for the off duration,
/// otherwise to 1.0 for the on duration.
pub fn animating_text_update(e: &mut Engine, this: Ptr<AnimatingText>) {
    if fn_007000b0(e, this)
        && e.call(CURRENT_TIME, &args![]).u32() >= e.get(this, AnimatingText::dwNextDisplayUpdate)
        && e.get(this, AnimatingText::bDisplayFinished) == 0
    {
        show_tiles(e, this);
        let buffer = e.get(this, AnimatingText::pBuffer);
        if e.get(this, AnimatingText::pStringTrait) != 0
            && buffer != 0
            && e.call(BUFFER_TEST_00700270, &args![this, buffer]).bool()
        {
            let buffer = e.get(this, AnimatingText::pBuffer);
            let shown = e.call(TEXT_LENGTH_0044A670, &args![buffer]).u32();
            let mut count = if e.get(this, AnimatingText::dwNextDisplayUpdate) == 0 {
                1
            } else {
                let now = e.call(CURRENT_TIME, &args![]).u32();
                let elapsed = now.wrapping_sub(e.get(this, AnimatingText::dwNextDisplayUpdate));
                elapsed / e.get(this, AnimatingText::dwDisplayInterval) + 1
            };
            let rest = e
                .call(STRING_LENGTH, &args![this])
                .u32()
                .wrapping_sub(shown);
            if count > rest {
                count = e
                    .call(STRING_LENGTH, &args![this])
                    .u32()
                    .wrapping_sub(shown);
                e.set(this, AnimatingText::bDisplayFinished, 1);
            }
            let text = e.call(STRING_TEXT, &args![this]).u32();
            let buffer = e.get(this, AnimatingText::pBuffer);
            e.call(
                COPY_CHARACTERS_00401460,
                &args![buffer.wrapping_add(shown), text.wrapping_add(shown), count],
            );
            let string_tile = e.get(this, AnimatingText::pStringTrait);
            let buffer = e.get(this, AnimatingText::pBuffer);
            e.call(TILE_VALUE_SET_STRING, &args![string_tile, buffer, 1u32]);
        } else {
            e.set(this, AnimatingText::bDisplayFinished, 1);
        }
        let now = e.call(CURRENT_TIME, &args![]).u32();
        let next = now.wrapping_add(e.get(this, AnimatingText::dwDisplayInterval));
        e.set(this, AnimatingText::dwNextDisplayUpdate, next);
    }
    if fn_007000d0(e, this)
        && e.call(CURRENT_TIME, &args![]).u32() >= e.get(this, AnimatingText::dwNextFlashUpdate)
    {
        let vis = e.get(this, AnimatingText::pVisTrait);
        let one: f64 = e.global(ONE_DOUBLE);
        // `FCOMP; TEST AH,0x44; JP`: the "visible" arm only when equal.
        if x87_load32(e.mem.u32(vis.wrapping_add(8))) == one {
            set_float(e, vis, 0.0);
            let now = e.call(CURRENT_TIME, &args![]).u32();
            let next = now.wrapping_add(e.get(this, AnimatingText::dwFlashOffDuration));
            e.set(this, AnimatingText::dwNextFlashUpdate, next);
        } else {
            set_float(e, vis, 1.0);
            let now = e.call(CURRENT_TIME, &args![]).u32();
            let next = now.wrapping_add(e.get(this, AnimatingText::dwFlashOnDuration));
            e.set(this, AnimatingText::dwNextFlashUpdate, next);
        }
    }
}

// Translated from 007000b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): whether animation type 1 (the
/// typing animation) is on (`007000f0`).
pub fn fn_007000b0(e: &mut Engine, this: Ptr<AnimatingText>) -> bool {
    fn_007000f0(e, this, 1)
}

// Translated from 007000d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): whether animation type 2 (the
/// flashing) is on (`007000f0`).
pub fn fn_007000d0(e: &mut Engine, this: Ptr<AnimatingText>) -> bool {
    fn_007000f0(e, this, 2)
}

// Translated from 007000f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether
/// `ucAnimationTypes` has a bit of `mask`.
pub fn fn_007000f0(e: &mut Engine, this: Ptr<AnimatingText>, mask: u32) -> bool {
    u32::from(e.get(this, AnimatingText::ucAnimationTypes)) & mask != 0
}

// Translated from 00700110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimatingText::ForceFinish` (engine map; `thiscall`, plain `RET`): for
/// an unfinished typing animation, shows the tiles ([`show_tiles`]) and,
/// with a string tile and a buffer, copies the whole text into the buffer
/// (`00401460` with its length, `0044a670`), sets it on the string tile and
/// marks the display finished.
pub fn animating_text_force_finish(e: &mut Engine, this: Ptr<AnimatingText>) {
    if !fn_007000b0(e, this) || e.get(this, AnimatingText::bDisplayFinished) != 0 {
        return;
    }
    show_tiles(e, this);
    if e.get(this, AnimatingText::pStringTrait) == 0 || e.get(this, AnimatingText::pBuffer) == 0 {
        return;
    }
    let text = e.call(STRING_TEXT, &args![this]).u32();
    let length = e.call(TEXT_LENGTH_0044A670, &args![text]).u32();
    let text = e.call(STRING_TEXT, &args![this]).u32();
    let buffer = e.get(this, AnimatingText::pBuffer);
    e.call(COPY_CHARACTERS_00401460, &args![buffer, text, length]);
    let string_tile = e.get(this, AnimatingText::pStringTrait);
    let buffer = e.get(this, AnimatingText::pBuffer);
    e.call(TILE_VALUE_SET_STRING, &args![string_tile, buffer, 1u32]);
    e.set(this, AnimatingText::bDisplayFinished, 1);
}

// Translated from 00759f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimatingText::ResetBuffer` (engine map; `thiscall`, `RET 4`): sets the
/// text (`00438390`), then makes a new buffer for it (`006ffda0`). Returns
/// `this`.
pub fn animating_text_reset_buffer(
    e: &mut Engine,
    this: Ptr<AnimatingText>,
    text: u32,
) -> Ptr<AnimatingText> {
    e.call(STRING_SET_00438390, &args![this, text]);
    fn_006ffda0(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_fda0, fn_006ffda0(Ptr<AnimatingText>)),
        entry!(0x006f_fea0, animating_text_update(Ptr<AnimatingText>)),
        entry!(0x0070_00b0, fn_007000b0(Ptr<AnimatingText>) -> bool),
        entry!(0x0070_00d0, fn_007000d0(Ptr<AnimatingText>) -> bool),
        entry!(0x0070_00f0, fn_007000f0(Ptr<AnimatingText>, u32) -> bool),
        entry!(0x0070_0110, animating_text_force_finish(Ptr<AnimatingText>)),
        entry!(
            0x0075_9f20,
            animating_text_reset_buffer(Ptr<AnimatingText>, u32) -> Ptr<AnimatingText>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(address, text.as_bytes());
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// The engine with a clock at `now`, string doubles (text at +0, length
    /// at +4) and C-string helpers.
    fn engine(now: Rc<Cell<u32>>) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.register_double(CURRENT_TIME, move |_, _| now.get().into_ret());
        e.register(STRING_TEXT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TEXT_LENGTH_0044A670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(COPY_CHARACTERS_00401460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(BUFFER_TEST_00700270, |_, _| 1u32.into_ret());
        e.register(TILE_VALUE_SET_FLOAT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(TILE_VALUE_SET_STRING, |_, _| Ret::default());
        e
    }

    /// An animating text over `full` with a buffer of zeros, string and
    /// visibility tiles, interval 100, flash on 30 / off 20.
    fn animating(e: &mut Engine, full: &str, types: u8) -> Ptr<AnimatingText> {
        let this: Ptr<AnimatingText> = e.new_object();
        let chars = put_text(e, full);
        e.mem.set_u32(this.addr(), chars);
        e.mem.set_u32(this.addr() + 4, full.len() as u32);
        let buffer = e.mem.alloc(full.len() as u32 + 1);
        e.mem.write(buffer, &vec![0; full.len() + 1]);
        e.set(this, AnimatingText::pBuffer, buffer);
        let string_tile = e.mem.alloc(0x10);
        let vis_tile = e.mem.alloc(0x10);
        e.set(this, AnimatingText::pStringTrait, string_tile);
        e.set(this, AnimatingText::pVisTrait, vis_tile);
        e.set(this, AnimatingText::dwDisplayInterval, 100);
        e.set(this, AnimatingText::dwFlashOnDuration, 30);
        e.set(this, AnimatingText::dwFlashOffDuration, 20);
        e.set(this, AnimatingText::ucAnimationTypes, types);
        this
    }

    #[test]
    fn new_buffer_for_the_text() {
        let mut e = engine(Rc::new(Cell::new(0)));
        e.register(FREE, |_, _| Ret::default());
        e.register(ALLOCATE_005AAEE0, |_, a| {
            (0x5000 + a[0] * 0x10 + a[1]).into_ret()
        });
        let this = animating(&mut e, "abc", 1);
        let old = e.get(this, AnimatingText::pBuffer);
        e.set(this, AnimatingText::bDisplayFinished, 1);
        e.set(this, AnimatingText::dwNextDisplayUpdate, 9);
        e.call_log = Some(vec![]);
        e.call(0x006f_fda0, &args![this]);
        assert_eq!(calls(&e, FREE), vec![vec![old]]);
        assert_eq!(e.get(this, AnimatingText::pBuffer), 0x5000 + 4 * 0x10 + 1);
        assert_eq!(e.get(this, AnimatingText::bDisplayFinished), 0);
        assert_eq!(e.get(this, AnimatingText::dwNextDisplayUpdate), 0);
        e.mem.set_u32(this.addr(), 0);
        fn_006ffda0(&mut e, this);
        assert_eq!(e.get(this, AnimatingText::pBuffer), 0);
    }

    #[test]
    fn typing_reveals_characters_per_interval() {
        let now = Rc::new(Cell::new(1000));
        let mut e = engine(now.clone());
        let this = animating(&mut e, "hello", 1);
        e.call(0x006f_fea0, &args![this]);
        let buffer = e.get(this, AnimatingText::pBuffer);
        assert_eq!(text(&e, buffer), "h");
        assert_eq!(e.get(this, AnimatingText::dwNextDisplayUpdate), 1100);
        // The visibility tile was set to 1.0 (no flashing).
        let vis = e.get(this, AnimatingText::pVisTrait);
        assert_eq!(e.mem.u32(vis + 8), 1.0f32.to_bits());
        // Not due yet: nothing changes.
        now.set(1099);
        animating_text_update(&mut e, this);
        assert_eq!(text(&e, buffer), "h");
        // 250 ms past the due time: 250 / 100 + 1 = 3 more.
        now.set(1350);
        animating_text_update(&mut e, this);
        assert_eq!(text(&e, buffer), "hell");
        assert_eq!(e.get(this, AnimatingText::bDisplayFinished), 0);
        // Far later: only the rest, and finished.
        now.set(9000);
        e.call_log = Some(vec![]);
        animating_text_update(&mut e, this);
        assert_eq!(text(&e, buffer), "hello");
        assert_eq!(e.get(this, AnimatingText::bDisplayFinished), 1);
        assert_eq!(calls(&e, COPY_CHARACTERS_00401460)[0][2], 1);
        // Finished: no more copies.
        now.set(20000);
        e.call_log = Some(vec![]);
        animating_text_update(&mut e, this);
        assert!(calls(&e, COPY_CHARACTERS_00401460).is_empty());
    }

    #[test]
    fn typing_without_a_buffer_finishes() {
        let now = Rc::new(Cell::new(5));
        let mut e = engine(now);
        let this = animating(&mut e, "hi", 1);
        e.set(this, AnimatingText::pBuffer, 0);
        animating_text_update(&mut e, this);
        assert_eq!(e.get(this, AnimatingText::bDisplayFinished), 1);
        assert_eq!(e.get(this, AnimatingText::dwNextDisplayUpdate), 105);
    }

    #[test]
    fn flashing_flips_the_visibility() {
        let now = Rc::new(Cell::new(50));
        let mut e = engine(now.clone());
        let this = animating(&mut e, "x", 2);
        let vis = e.get(this, AnimatingText::pVisTrait);
        e.mem.set_u32(vis + 8, 1.0f32.to_bits());
        animating_text_update(&mut e, this);
        assert_eq!(e.mem.u32(vis + 8), 0);
        assert_eq!(e.get(this, AnimatingText::dwNextFlashUpdate), 70);
        now.set(69);
        animating_text_update(&mut e, this);
        assert_eq!(e.mem.u32(vis + 8), 0);
        now.set(70);
        animating_text_update(&mut e, this);
        assert_eq!(e.mem.u32(vis + 8), 1.0f32.to_bits());
        assert_eq!(e.get(this, AnimatingText::dwNextFlashUpdate), 100);
    }

    #[test]
    fn animation_type_bits() {
        let mut e = Engine::new();
        let this: Ptr<AnimatingText> = e.new_object();
        e.set(this, AnimatingText::ucAnimationTypes, 1);
        assert!(e.call(0x0070_00b0, &args![this]).bool());
        assert!(!e.call(0x0070_00d0, &args![this]).bool());
        e.set(this, AnimatingText::ucAnimationTypes, 2);
        assert!(!fn_007000b0(&mut e, this));
        assert!(fn_007000d0(&mut e, this));
        assert!(e.call(0x0070_00f0, &args![this, 0x102u32]).bool());
        assert!(!fn_007000f0(&mut e, this, 0x101));
    }

    #[test]
    fn force_finish_copies_the_whole_text() {
        let mut e = engine(Rc::new(Cell::new(0)));
        let this = animating(&mut e, "done", 1);
        e.call_log = Some(vec![]);
        e.call(0x0070_0110, &args![this]);
        let buffer = e.get(this, AnimatingText::pBuffer);
        assert_eq!(text(&e, buffer), "done");
        assert_eq!(e.get(this, AnimatingText::bDisplayFinished), 1);
        let string_tile = e.get(this, AnimatingText::pStringTrait);
        assert_eq!(
            calls(&e, TILE_VALUE_SET_STRING),
            vec![vec![string_tile, buffer, 1]]
        );
        // Already finished: nothing.
        e.call_log = Some(vec![]);
        animating_text_force_finish(&mut e, this);
        assert!(calls(&e, COPY_CHARACTERS_00401460).is_empty());
        // Without a string tile: tiles only, not finished.
        let other = animating(&mut e, "x", 1);
        e.set(other, AnimatingText::pStringTrait, 0);
        animating_text_force_finish(&mut e, other);
        assert_eq!(e.get(other, AnimatingText::bDisplayFinished), 0);
    }

    #[test]
    fn reset_buffer_sets_text_then_buffer() {
        let mut e = engine(Rc::new(Cell::new(0)));
        e.register(STRING_SET_00438390, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(ALLOCATE_005AAEE0, |_, _| 0x7777u32.into_ret());
        let this = animating(&mut e, "abc", 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x0075_9f20, &args![this, 0x1234u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(
            calls(&e, STRING_SET_00438390),
            vec![vec![this.addr(), 0x1234]]
        );
        assert_eq!(e.get(this, AnimatingText::pBuffer), 0x7777);
    }
}

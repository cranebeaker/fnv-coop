//! `BSHavok/bhkcollisionfilter.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The collision filter info is one 32-bit word (`CFilter`): the low 7 bits are the layer,
//! bits 8 to 12 the biped part (for the layers 8 and 0x1d), bit 13 "scaled", bit 14 "no
//! collision", bit 15 "linked" and the high 16 bits the group.
//!
//! The five small `isCollisionEnabled`-shaped functions are vtable entries of the filter: they
//! are called with the filter in `ECX` (never read), answer through a result pointer (their
//! first stack word, which they also return in `EAX`) and compare two filter infos with
//! `bhkCollisionFilter::IsCollisionEnabled` (`00c84740`).
//!
//! The `/GS` cookie check (`00ec408c`) of the function marked `Translation incomplete` is not
//! modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCollisionFilter::IsCollisionEnabled` (engine map name, `00c84740`, cdecl: the two
/// filter infos; answers a bool).
const IS_COLLISION_ENABLED: u32 = 0x00c8_4740;
/// `sprintf_s` (`00406d00`, cdecl: buffer, size, format, values).
const FORMAT: u32 = 0x0040_6d00;
/// `NiGetViewerString` overloads (engine map names): `_ov11` (`00a7d5e0`, cdecl: label,
/// string), `_ov2` (`00a7d370`, cdecl: label, bool word) and `_ov8` (`00a7d510`, cdecl: label,
/// integer); each answers the text pointer.
const VIEWER_STRING_OF_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRING_OF_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_OF_INT: u32 = 0x00a7_d510;
/// The string array's resize (`0096ad30`) and store (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The format strings `"0x%08X"` and `"0x%X"` and the labels `"COLFILTER"`, `"-GROUP"`,
/// `"-LAYER"`, `"-LINK"`, `"-PART"`, `"-NOCOL"` and `"-SCALED"` of the viewer strings.
const FORMAT_FILTER: u32 = 0x010c_54b0;
const FORMAT_GROUP: u32 = 0x010c_584c;
const LABEL_FILTER: u32 = 0x010c_5854;
const LABEL_GROUP: u32 = 0x010c_5844;
const LABEL_LAYER: u32 = 0x010c_583c;
const LABEL_LINK: u32 = 0x010c_5834;
const LABEL_PART: u32 = 0x010c_582c;
const LABEL_NO_COLLISION: u32 = 0x010c_5824;
const LABEL_SCALED: u32 = 0x010c_581c;
/// The table of the layer names (`011b0810`, indexed by the layer) and of the biped part names
/// (`011b08c0`, indexed by the part).
const LAYER_NAMES: u32 = 0x011b_0810;
const PART_NAMES: u32 = 0x011b_08c0;
/// The debug color (three words) of a layer outside the table (`011a9b7c`).
const DEFAULT_DEBUG_COLOR: u32 = 0x011a_9b7c;

/// One component of a layer's debug color: the x87 constant 0 or 1, or the `float` at an
/// address (copied through `FLD`/`FSTP`).
#[derive(Clone, Copy)]
enum Component {
    Zero,
    One,
    Global(u32),
}

/// The case (an index into [`debug_color_components`], 18 for the default) of each layer from
/// 1 to 31: the compiler's switch on the layer.
fn layer_case(layer: u32) -> u8 {
    match layer {
        1 => 0,
        2 | 10 => 1,
        3 => 2,
        4 => 3,
        5 => 4,
        6 => 5,
        7 => 6,
        8 => 7,
        9 => 8,
        11 => 9,
        12 | 14 => 10,
        13 => 11,
        16 => 12,
        17 => 13,
        18 => 14,
        24 => 15,
        30 => 16,
        31 => 17,
        _ => 18,
    }
}

/// The three components of the debug color of case `case` (the jump table at `00c846d4`), or
/// `None` for the default case.
fn debug_color_components(case: u8) -> Option<[Component; 3]> {
    use Component::{Global, One, Zero};
    Some(match case {
        0 => [One, Zero, Zero],
        1 => [
            Global(0x0101_77e8),
            Global(0x010c_5818),
            Global(0x010c_5814),
        ],
        2 => [
            Global(0x010c_5810),
            Global(0x0103_0ff0),
            Global(0x0103_0ff0),
        ],
        3 => [
            Global(0x010c_580c),
            Global(0x010c_5808),
            Global(0x010c_5804),
        ],
        4 => [
            Global(0x0101_9de0),
            Global(0x010c_5800),
            Global(0x0108_aff0),
        ],
        5 => [
            Global(0x0101_77e8),
            Global(0x010c_57fc),
            Global(0x010c_57f8),
        ],
        6 => [
            Global(0x010c_57f4),
            Global(0x0101_45a8),
            Global(0x0101_77e8),
        ],
        7 => [Zero, One, Zero],
        8 => [
            Global(0x0103_0ff0),
            Global(0x0108_da38),
            Global(0x0104_e2b8),
        ],
        9 => [
            Global(0x010c_57f0),
            Global(0x0102_05fc),
            Global(0x0102_05fc),
        ],
        10 => [
            Global(0x010c_57fc),
            Global(0x010c_57ec),
            Global(0x0101_6264),
        ],
        11 | 13 => [
            Global(0x010c_5810),
            Global(0x0101_45a8),
            Global(0x010c_57e8),
        ],
        12 => [
            Global(0x010c_57fc),
            Global(0x0102_d92c),
            Global(0x0101_8238),
        ],
        14 => [Zero, Global(0x0101_8238), Zero],
        15 => [
            Global(0x0104_e2b8),
            Global(0x0101_8180),
            Global(0x0101_e2bc),
        ],
        16 => [One, One, Zero],
        17 => [One, Global(0x0103_0ff0), Zero],
        _ => return None,
    })
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

/// The root of a collision body: its parent chain (`+0xc`) followed to the end.
fn root_of(e: &Engine, body: u32) -> u32 {
    let mut node = body;
    loop {
        let parent = e.mem.u32(node + 0xc);
        if parent == 0 {
            return node;
        }
        node = parent;
    }
}

/// The collision filter info of a collidable (`+0x1c`).
fn filter_info_of(e: &Engine, collidable: u32) -> u32 {
    e.mem.u32(collidable + 0x1c)
}

/// Compares two filter infos (`00c84740`), stores the answer as a byte at `out` and answers
/// `out`.
fn answer(e: &mut Engine, out: Ptr, first: u32, second: u32) -> Ptr {
    let enabled = e.call(IS_COLLISION_ENABLED, &args![first, second]).bool();
    e.mem.set_u8(out.addr(), enabled as u8);
    out
}

// Translated from 00c84470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionFilter::GetDebugColor` (Xbox PDB), cdecl `(filter, color)`: writes the three
/// words of the debug color of the filter's layer (`filter & 0x7f`, 1 to 31) to `color`: each
/// is the constant 0 or 1 or a `float` copied through `FLD`/`FSTP` (a signalling NaN comes out
/// quiet), the cases chosen by the exe's tables (`00c84720`, `00c846d4`). Any other layer
/// copies the three words at `011a9b7c`. Answers `color`.
pub fn bhk_collision_filter_get_debug_color(e: &mut Engine, filter: u32, color: Ptr) -> Ptr {
    let layer = filter & 0x7f;
    let case = layer_case(layer);
    match debug_color_components(case) {
        Some(components) => {
            for (index, component) in components.into_iter().enumerate() {
                let bits = match component {
                    Component::Zero => 0.0f32.to_bits(),
                    Component::One => 1.0f32.to_bits(),
                    Component::Global(address) => x87_quiet_bits(e.mem.u32(address)),
                };
                e.mem.set_u32(color.addr() + 4 * index as u32, bits);
            }
        }
        None => {
            for index in 0..3 {
                let word = e.mem.u32(DEFAULT_DEBUG_COLOR + 4 * index);
                e.mem.set_u32(color.addr() + 4 * index, word);
            }
        }
    }
    color
}

// Translated from 00c84880 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload of two
/// collidables has this shape), `(out, collidable_a, collidable_b)`: stores whether
/// `IsCollisionEnabled` (`00c84740`) answers true for the two collidables' filter infos
/// (`+0x1c`) as a byte at `out`. Answers `out`.
pub fn fn_00c84880(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    collidable_a: Ptr,
    collidable_b: Ptr,
) -> Ptr {
    let first = filter_info_of(e, collidable_a.addr());
    let second = filter_info_of(e, collidable_b.addr());
    answer(e, out, first, second)
}

// Translated from 00c848b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload of
/// two collision bodies has this shape), `(out, _, body_a, body_b, _, _)`: the same as
/// [`fn_00c84880`], with the filter info taken from the root (the end of the parent chain at
/// `+0xc`) of each body.
pub fn fn_00c848b0(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_0: u32,
    body_a: Ptr,
    body_b: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> Ptr {
    let second_root = root_of(e, body_b.addr());
    let second = filter_info_of(e, second_root);
    let first_root = root_of(e, body_a.addr());
    let first = filter_info_of(e, first_root);
    answer(e, out, first, second)
}

// Translated from 00c84900 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload for a
/// ray cast against a shape has this shape), `(out, input, _, container, key)`: compares the
/// filter info at `input + 0x20` with the one the container answers from its virtual `+0x10`
/// for `key`, and stores the answer of `IsCollisionEnabled` as a byte at `out`. Answers `out`.
pub fn fn_00c84900(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    input: Ptr,
    _unused_0: u32,
    container: Ptr,
    key: u32,
) -> Ptr {
    let shape_info = e.vcall(container.addr(), 0x10, &args![key]).u32();
    let ray_info = e.mem.u32(input.addr() + 0x20);
    answer(e, out, ray_info, shape_info)
}

// Translated from 00c84930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's `isCollisionEnabled` overload for a
/// world ray cast against a collidable has this shape), `(out, input, collidable)`: compares
/// the filter info at `input + 0x24` with the collidable's, and stores the answer of
/// `IsCollisionEnabled` as a byte at `out`. Answers `out`.
pub fn fn_00c84930(e: &mut Engine, _this: Ptr, out: Ptr, input: Ptr, collidable: Ptr) -> Ptr {
    let ray_info = e.mem.u32(input.addr() + 0x24);
    let collidable_info = filter_info_of(e, collidable.addr());
    answer(e, out, ray_info, collidable_info)
}

// Translated from 00c84960 (decompiled, FalloutNV.exe 1.4.0.525)
/// A filter vtable entry (unconfirmed identity; the PDB's longest `isCollisionEnabled`
/// overload has this shape), `(out, _, body_a, body_b, _, _, _, _)`: the same as
/// [`fn_00c848b0`].
pub fn fn_00c84960(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_0: u32,
    body_a: Ptr,
    body_b: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    let first_root = root_of(e, body_a.addr());
    let first = filter_info_of(e, first_root);
    let second_root = root_of(e, body_b.addr());
    let second = filter_info_of(e, second_root);
    answer(e, out, first, second)
}

// Translated from 00c849b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c849b0 - the /GS cookie check is not modeled.
/// `CFilter::GetViewerStrings` (engine map name, from a string reference), `(strings)`: stores
/// one viewer string per field of the filter info word at `this`, in this order: `COLFILTER`
/// with the word as `0x%08X` (`sprintf_s`, `00406d00`, then `00a7d5e0`), `-GROUP` with the high
/// half as `0x%X`, `-LAYER` with the layer's name from the table at `011b0810`, `-LINK` with
/// bit 15, then `-PART`: for the layers 8 and 0x1d the name from the table at `011b08c0`,
/// otherwise, when bit 15 is set, the part number (bits 8 to 12) through `00a7d510`; none
/// else. `-NOCOL` with bit 14 and `-SCALED` with bit 13 follow. The bool strings (`00a7d370`)
/// are passed the dword of the exe's stack slot holding the previous string pointer with its
/// low byte replaced by the bit.
pub fn c_filter_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let array = strings.addr();
    let info_of = |e: &Engine| e.mem.u32(this.addr());
    e.with_stack(0x80, |e, buffer| {
        let info = info_of(e);
        e.call(FORMAT, &args![buffer, 0x80u32, FORMAT_FILTER, info]);
        let text = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_FILTER, buffer])
            .u32();
        store_text(e, array, text);
        let group = u32::from(e.mem.u16(this.addr() + 2));
        e.call(FORMAT, &args![buffer, 0x80u32, FORMAT_GROUP, group]);
        let text = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_GROUP, buffer])
            .u32();
        store_text(e, array, text);
    });
    let info = info_of(e);
    let layer_name = e.mem.u32(LAYER_NAMES + 4 * (info & 0x7f));
    let mut slot = e
        .call(VIEWER_STRING_OF_TEXT, &args![LABEL_LAYER, layer_name])
        .u32();
    store_text(e, array, slot);
    let bit = (info_of(e) >> 0xf) & 1;
    slot = e
        .call(
            VIEWER_STRING_OF_BOOL,
            &args![LABEL_LINK, (slot & 0xffff_ff00) | bit],
        )
        .u32();
    store_text(e, array, slot);
    let info = info_of(e);
    let layer = info & 0x7f;
    let part = (info >> 8) & 0x1f;
    if layer == 8 || layer == 0x1d {
        let part_name = e.mem.u32(PART_NAMES + 4 * part);
        slot = e
            .call(VIEWER_STRING_OF_TEXT, &args![LABEL_PART, part_name])
            .u32();
        store_text(e, array, slot);
    } else if (info >> 0xf) & 1 != 0 {
        slot = e.call(VIEWER_STRING_OF_INT, &args![LABEL_PART, part]).u32();
        store_text(e, array, slot);
    }
    for (label, shift) in [(LABEL_NO_COLLISION, 0xe), (LABEL_SCALED, 0xd)] {
        let bit = (info_of(e) >> shift) & 1;
        slot = e
            .call(
                VIEWER_STRING_OF_BOOL,
                &args![label, (slot & 0xffff_ff00) | bit],
            )
            .u32();
        store_text(e, array, slot);
    }
}

/// `_memset` (`00ec61c0`, cdecl: destination, byte, size).
const MEMSET: u32 = 0x00ec_61c0;
/// `00c827f0` (unnamed in the engine map; cdecl: layer, other layer, enable byte): by its body,
/// sets (enable nonzero) or clears the bit of each layer in the other layer's mask of the layer
/// table.
const LAYER_PAIR_COLLISION: u32 = 0x00c8_27f0;
/// `bhkCollisionFilter::SetMultiLayerCollision` (engine map name, `00c82870`, cdecl: layer, low
/// and high words of a layer mask).
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
/// The layer table (`01267f20`, 0x158 bytes): two words (a 64-bit mask of layers) per layer, the
/// low word at `01267f20 + 8 * layer`, the high word 4 bytes after it; `00c827f0` and
/// `SetMultiLayerCollision` index it by layer. Its size is that of the Xbox PDB's static
/// `bhkCollisionFilter::iLayerFlagA`.
const LAYER_FLAGS: u32 = 0x0126_7f20;
/// The table of 32 words at `01268078` (0x80 bytes, the size of the Xbox PDB's static
/// `bhkCollisionFilter::iPartFlagA`; that it is that table is unconfirmed).
const PART_FLAGS: u32 = 0x0126_8078;
/// The table of 43 words at `01267e70` (0xac bytes, the size of the Xbox PDB's static
/// `bhkCollisionFilter::iLayerGroupA`; that it is that table is unconfirmed).
const LAYER_GROUPS: u32 = 0x0126_7e70;

/// What `InitFlags` does to the words of the layer table and of the table at `01268078` between
/// its `memset`s and its first call, one row per word it stores: (address, the bits it keeps,
/// the bits it sets); the word becomes `(word & keep) | set`. The exe does this as hundreds of
/// bitfield operations (loads, `AND`s, `OR`s, stores, in a scheduled order, several stores to a
/// word); each word's result depends only on its own value after the `memset`s and nothing
/// else is read or called before the first call, so the rows are the net effect of those
/// instructions, word by word, in address order. Rows keeping every bit and setting none are
/// words the exe stores back unchanged (the other half of a bitfield it rewrites).
const INIT_BITS: [(u32, u32, u32); 96] = [
    (0x0126_7f20, 0xffdf_ffff, 0),
    (0x0126_7f24, 0xffff_ffff, 0),
    (0x0126_7f28, 0xffdb_7fff, 0),
    (0x0126_7f2c, 0xffff_ffff, 0),
    (0x0126_7f30, 0xffdb_7fff, 0),
    (0x0126_7f34, 0xffff_ffff, 0),
    (0x0126_7f38, 0xffd3_7fff, 0),
    (0x0126_7f3c, 0xffff_ffff, 0),
    (0x0126_7f40, 0xffdb_7eff, 0),
    (0x0126_7f44, 0xffff_ffff, 0),
    (0x0126_7f48, 0xffdb_7eff, 0),
    (0x0126_7f4c, 0xffff_ffff, 0),
    (0x0126_7f50, 0xffd3_7fff, 0),
    (0x0126_7f54, 0xffff_ffff, 0),
    (0x0126_7f58, 0xffd3_7fff, 0),
    (0x0126_7f5c, 0xffff_ffff, 0),
    (0x0126_7f60, 0x9fc3_7bcf, 0),
    (0x0126_7f64, 0xffff_ffff, 0),
    (0x0126_7f68, 0xffdb_7fff, 0),
    (0x0126_7f6c, 0xffff_ffff, 0),
    (0x0126_7f70, 0xffdb_7eff, 0),
    (0x0126_7f74, 0xffff_ffff, 0),
    (0x0126_7f78, 0xbfc3_7fff, 0x4000_0000),
    (0x0126_7f7c, 0xffff_ffff, 0),
    (0x0126_7f80, 0xffd3_7fff, 0),
    (0x0126_7f84, 0xffff_ffff, 0),
    (0x0126_7f88, 0xffdb_7fff, 0),
    (0x0126_7f8c, 0xffff_ffff, 0),
    (0x0126_7f90, 0xffdb_7fff, 0),
    (0x0126_7f94, 0xffff_ffff, 0),
    (0x0126_7f98, 0xcbc0_0001, 0),
    (0x0126_7f9c, 0xffff_ffff, 0),
    (0x0126_7fa0, 0xffc3_7fff, 0),
    (0x0126_7fa4, 0xffff_ffff, 0),
    (0x0126_7fa8, 0xffdb_7fff, 0),
    (0x0126_7fac, 0xffff_ffff, 0),
    (0x0126_7fb0, 0x0be0_0001, 0),
    (0x0126_7fb4, 0xffff_fc17, 0),
    (0x0126_7fb8, 0x2bfa_6637, 0),
    (0x0126_7fbc, 0xffff_fb85, 0),
    (0x0126_7fc0, 0xfffa_76ff, 0),
    (0x0126_7fc4, 0xffff_ffdf, 0),
    (0x0126_7fc8, 0x03dc_0000, 0x4000_0000),
    (0x0126_7fcc, 0xffff_fc15, 0x0000_0002),
    (0x0126_7ff0, 0xffd3_7fff, 0),
    (0x0126_7ff4, 0xffff_ffff, 0),
    (0x0126_7ff8, 0xffdf_ffff, 0),
    (0x0126_7ffc, 0xffff_ffff, 0),
    (0x0126_8000, 0xffd3_7fff, 0),
    (0x0126_8004, 0xffff_ffff, 0),
    (0x0126_8008, 0xbfdb_7eff, 0),
    (0x0126_800c, 0xffff_ffff, 0),
    (0x0126_8010, 0xdfd3_f6ff, 0x0020_0800),
    (0x0126_8014, 0xffff_ffff, 0),
    (0x0126_8018, 0xffd3_ffff, 0),
    (0x0126_801c, 0xffff_ffff, 0),
    (0x0126_8028, 0xffd7_ffff, 0x0020_0000),
    (0x0126_802c, 0xffff_ffff, 0),
    (0x0126_8038, 0xffd3_ffff, 0),
    (0x0126_803c, 0xffff_ffff, 0),
    (0x0126_8040, 0xfff7_ffff, 0),
    (0x0126_8044, 0xffff_ffff, 0),
    (0x0126_8048, 0xffc3_ffff, 0),
    (0x0126_804c, 0xffff_ffff, 0),
    (0x0126_8050, 0xffd3_ffff, 0),
    (0x0126_8054, 0xffff_ffff, 0),
    (0x0126_8058, 0xffdb_ffff, 0),
    (0x0126_805c, 0xffff_ffff, 0),
    (0x0126_8060, 0xffdb_ffff, 0),
    (0x0126_8064, 0xffff_ffff, 0),
    (0x0126_8068, 0xffdb_ffff, 0),
    (0x0126_806c, 0xffff_ffff, 0),
    (0x0126_8070, 0xfff7_ffff, 0),
    (0x0126_8074, 0xffff_ffff, 0),
    (0x0126_807c, 0xffbd_cf3f, 0x0042_30c0),
    (0x0126_8080, 0xfebf_cf3f, 0x0140_30c0),
    (0x0126_8084, 0xfebf_cf3f, 0x0140_30c0),
    (0x0126_8088, 0xfebd_cf3f, 0x0142_30c0),
    (0x0126_808c, 0xffbf_cfff, 0x0040_3000),
    (0x0126_8090, 0xffbd_86e1, 0x0042_791e),
    (0x0126_8094, 0xffbe_00e1, 0x0041_ff1e),
    (0x0126_8098, 0xfeba_0f3f, 0x0145_f0c0),
    (0x0126_809c, 0xfeba_1f7f, 0x0145_e080),
    (0x0126_80a0, 0xfebb_1f7f, 0x0144_e080),
    (0x0126_80a4, 0xffbf_ff3f, 0x0040_00c0),
    (0x0126_80a8, 0xffbd_be01, 0x0042_41fe),
    (0x0126_80ac, 0xffba_3801, 0x0045_c7fe),
    (0x0126_80b0, 0xfebb_c83f, 0x0144_37c0),
    (0x0126_80b4, 0xfebb_d87f, 0x0144_2780),
    (0x0126_80b8, 0xfebb_dc7f, 0x0144_2380),
    (0x0126_80bc, 0xffbf_efad, 0x0040_1052),
    (0x0126_80c0, 0xffbe_18ff, 0x0041_e700),
    (0x0126_80c8, 0xffbf_ffff, 0x0040_0000),
    (0x0126_80cc, 0xffbf_ffff, 0x0040_0000),
    (0x0126_80d0, 0xffc8_0001, 0x0037_fffe),
    (0x0126_80d8, 0xfffe_38e3, 0x0001_c71c),
];

/// The layer pairs (`00c827f0` with enable 0) `InitFlags` clears before setting the pair
/// (0x21, 0xb) by hand.
const PAIRS_BEFORE: [(u32, u32); 21] = [
    (0xf, 0x1e),
    (0xf, 0x1f),
    (0xf, 0x23),
    (0xf, 0x25),
    (0xf, 0x26),
    (0xf, 0x27),
    (0xf, 0x28),
    (0xf, 0x29),
    (0x21, 0x1),
    (0x21, 0x2),
    (0x21, 0x3),
    (0x21, 0x1a),
    (0x21, 0x1c),
    (0x21, 0x4),
    (0x21, 0x5),
    (0x21, 0x6),
    (0x21, 0x7),
    (0x21, 0x8),
    (0x21, 0x1d),
    (0x21, 0x9),
    (0x21, 0xa),
];

/// The layer pairs (`00c827f0` with enable 0) `InitFlags` clears after it.
const PAIRS_AFTER: [(u32, u32); 251] = [
    (0x21, 0xc),
    (0x21, 0xd),
    (0x21, 0xe),
    (0x21, 0x21),
    (0x21, 0x11),
    (0x21, 0xf),
    (0x21, 0x10),
    (0x21, 0x1e),
    (0x21, 0x1f),
    (0x21, 0x23),
    (0x21, 0x25),
    (0x21, 0x26),
    (0x21, 0x27),
    (0x21, 0x28),
    (0x21, 0x29),
    (0x23, 0x5),
    (0x23, 0x7),
    (0x23, 0xc),
    (0x23, 0x23),
    (0x23, 0xb),
    (0x23, 0x6),
    (0x24, 0x7),
    (0x24, 0xc),
    (0x24, 0xb),
    (0x25, 0x3),
    (0x25, 0x1a),
    (0x25, 0x1c),
    (0x25, 0x7),
    (0x25, 0xb),
    (0x25, 0xc),
    (0x25, 0x1e),
    (0x26, 0x1f),
    (0x26, 0x4),
    (0x26, 0x5),
    (0x26, 0x6),
    (0x26, 0x7),
    (0x26, 0x8),
    (0x26, 0x1d),
    (0x26, 0x1e),
    (0x26, 0xa),
    (0x26, 0x23),
    (0x26, 0xc),
    (0x26, 0xe),
    (0x26, 0x10),
    (0x26, 0x29),
    (0x26, 0x24),
    (0x26, 0x25),
    (0x1f, 0x1f),
    (0x1f, 0x1),
    (0x1f, 0x4),
    (0x1f, 0x5),
    (0x1f, 0x6),
    (0x1f, 0x7),
    (0x1f, 0x8),
    (0x1f, 0x1d),
    (0x1f, 0x9),
    (0x1f, 0xa),
    (0x1f, 0xb),
    (0x1f, 0xd),
    (0x1f, 0xf),
    (0x1f, 0x11),
    (0x1f, 0x23),
    (0x1f, 0x29),
    (0x1f, 0x24),
    (0x1f, 0x25),
    (0x29, 0x3),
    (0x29, 0x1a),
    (0x29, 0x1c),
    (0x29, 0x7),
    (0x29, 0xb),
    (0x29, 0x23),
    (0x29, 0x24),
    (0x29, 0x25),
    (0x29, 0x27),
    (0x29, 0x28),
    (0x29, 0x26),
    (0x29, 0x29),
    (0x6, 0x7),
    (0x6, 0x10),
    (0x6, 0x3),
    (0xc, 0x1),
    (0xc, 0x2),
    (0xc, 0x3),
    (0xc, 0x1a),
    (0xc, 0x1c),
    (0xc, 0x9),
    (0xc, 0xa),
    (0xc, 0xb),
    (0xc, 0xc),
    (0xc, 0xd),
    (0xc, 0xf),
    (0xc, 0x10),
    (0xc, 0x11),
    (0xc, 0x16),
    (0xc, 0x20),
    (0xc, 0x26),
    (0xc, 0x2a),
    (0xc, 0x25),
    (0xc, 0x24),
    (0xc, 0x29),
    (0x16, 0x1),
    (0x16, 0x2),
    (0x16, 0x3),
    (0x16, 0x1a),
    (0x16, 0x4),
    (0x16, 0x5),
    (0x16, 0x6),
    (0x16, 0x7),
    (0x16, 0x8),
    (0x16, 0x1d),
    (0x16, 0x9),
    (0x16, 0xa),
    (0x16, 0xb),
    (0x16, 0xc),
    (0x16, 0xd),
    (0x16, 0xe),
    (0x16, 0xf),
    (0x16, 0x10),
    (0x16, 0x11),
    (0x16, 0x12),
    (0x16, 0x13),
    (0x16, 0x14),
    (0x16, 0x16),
    (0x16, 0x17),
    (0x16, 0x19),
    (0x16, 0x1f),
    (0x16, 0x20),
    (0x16, 0x21),
    (0x16, 0x23),
    (0x16, 0x24),
    (0x16, 0x25),
    (0x16, 0x26),
    (0x16, 0x29),
    (0x16, 0x2a),
    (0x16, 0x1b),
    (0x16, 0x22),
    (0x16, 0x0),
    (0x17, 0x1),
    (0x17, 0x2),
    (0x17, 0x3),
    (0x17, 0x1a),
    (0x17, 0x1c),
    (0x17, 0x4),
    (0x17, 0x5),
    (0x17, 0x7),
    (0x17, 0x8),
    (0x17, 0x1d),
    (0x17, 0x9),
    (0x17, 0xa),
    (0x17, 0xb),
    (0x17, 0xc),
    (0x17, 0xd),
    (0x17, 0xe),
    (0x17, 0xf),
    (0x17, 0x10),
    (0x17, 0x11),
    (0x17, 0x12),
    (0x17, 0x13),
    (0x17, 0x14),
    (0x17, 0x16),
    (0x17, 0x17),
    (0x17, 0x19),
    (0x17, 0x1e),
    (0x17, 0x1f),
    (0x17, 0x20),
    (0x17, 0x21),
    (0x17, 0x23),
    (0x17, 0x24),
    (0x17, 0x25),
    (0x17, 0x26),
    (0x17, 0x29),
    (0x17, 0x2a),
    (0x18, 0x1),
    (0x18, 0x2),
    (0x18, 0x3),
    (0x18, 0x1a),
    (0x18, 0x1c),
    (0x18, 0x4),
    (0x18, 0x5),
    (0x18, 0x7),
    (0x18, 0x8),
    (0x18, 0x1d),
    (0x18, 0x9),
    (0x18, 0xa),
    (0x18, 0xb),
    (0x18, 0xc),
    (0x18, 0xd),
    (0x18, 0xe),
    (0x18, 0xf),
    (0x18, 0x10),
    (0x18, 0x11),
    (0x18, 0x12),
    (0x18, 0x13),
    (0x18, 0x14),
    (0x18, 0x15),
    (0x18, 0x16),
    (0x18, 0x17),
    (0x18, 0x18),
    (0x18, 0x19),
    (0x18, 0x1f),
    (0x18, 0x20),
    (0x18, 0x21),
    (0x18, 0x23),
    (0x18, 0x24),
    (0x18, 0x25),
    (0x18, 0x26),
    (0x18, 0x2a),
    (0x19, 0x3),
    (0x19, 0x1a),
    (0x19, 0x1c),
    (0x19, 0x6),
    (0x19, 0x7),
    (0x19, 0x8),
    (0x19, 0xc),
    (0x19, 0xf),
    (0x19, 0x10),
    (0x19, 0x12),
    (0x19, 0x1e),
    (0x19, 0x1f),
    (0x19, 0x23),
    (0x19, 0x24),
    (0x19, 0x25),
    (0x19, 0x26),
    (0x19, 0x2a),
    (0x19, 0xb),
    (0x7, 0x7),
    (0x7, 0x10),
    (0x7, 0x3),
    (0x7, 0x1a),
    (0x7, 0x1c),
    (0x2a, 0x4),
    (0x2a, 0x6),
    (0x2a, 0x5),
    (0x2a, 0xb),
    (0x2a, 0x1d),
    (0x2a, 0x1f),
    (0x2a, 0x26),
    (0x2a, 0x23),
    (0x2a, 0x29),
    (0x2a, 0x25),
    (0x1b, 0x13),
    (0x1b, 0x19),
    (0x1b, 0x1f),
    (0x1b, 0x20),
    (0x1b, 0x21),
    (0x1b, 0x23),
    (0x1b, 0x25),
    (0x1b, 0x2a),
    (0x1b, 0x5),
    (0x3, 0x0),
    (0x3, 0xd),
];

/// `word |= bits` at `address`.
fn or_word(e: &mut Engine, address: u32, bits: u32) {
    let word = e.mem.u32(address);
    e.mem.set_u32(address, word | bits);
}

// Translated from 00c828f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionFilter::InitFlags` (Xbox PDB), no arguments (called by `bhkWorld::InitSystem`
/// at `00c6835a`): fills the table at `01268078` with 0 (0x80 bytes), the layer table at
/// `01267f20` with 0xff (0x158 bytes) and the table at `01267e70` with 0 (0xac bytes) through
/// `_memset`; then sets and clears the bits of [`INIT_BITS`]; clears the collision of each
/// layer pair of [`PAIRS_BEFORE`] through `00c827f0`; sets the pair (0x21, 0xb) itself (bit
/// 0x800 of layer 0x21's low word, bit 2 of layer 0xb's high word: layer 0x21's high word and
/// layer 0xb's low word are read before and stored back after, unchanged); clears the pairs of
/// [`PAIRS_AFTER`]; gives layer 0x22 the mask `0x0000_0000_1000_0004` through
/// `SetMultiLayerCollision`; and stores 2, 3 and 4 in the words 13, 9 and 11 of the table at
/// `01267e70`. The exe has no branch: every call and store always happens.
pub fn bhk_collision_filter_init_flags(e: &mut Engine) {
    e.call(MEMSET, &args![PART_FLAGS, 0u32, 0x80u32]);
    e.call(MEMSET, &args![LAYER_FLAGS, 0xffu32, 0x158u32]);
    e.call(MEMSET, &args![LAYER_GROUPS, 0u32, 0xacu32]);
    for (address, keep, set) in INIT_BITS {
        let word = e.mem.u32(address);
        e.mem.set_u32(address, (word & keep) | set);
    }
    for (layer, other) in PAIRS_BEFORE {
        e.call(LAYER_PAIR_COLLISION, &args![layer, other, 0u32]);
    }
    let high_of_0x21 = e.mem.u32(LAYER_FLAGS + 8 * 0x21 + 4);
    let low_of_0xb = e.mem.u32(LAYER_FLAGS + 8 * 0xb);
    or_word(e, LAYER_FLAGS + 8 * 0x21, 0x800);
    or_word(e, LAYER_FLAGS + 8 * 0xb + 4, 2);
    e.mem.set_u32(LAYER_FLAGS + 8 * 0x21 + 4, high_of_0x21);
    e.mem.set_u32(LAYER_FLAGS + 8 * 0xb, low_of_0xb);
    for (layer, other) in PAIRS_AFTER {
        e.call(LAYER_PAIR_COLLISION, &args![layer, other, 0u32]);
    }
    e.call(
        SET_MULTI_LAYER_COLLISION,
        &args![0x22u32, 0x1000_0004u32, 0u32],
    );
    e.mem.set_u32(LAYER_GROUPS + 4 * 13, 2);
    e.mem.set_u32(LAYER_GROUPS + 4 * 9, 3);
    e.mem.set_u32(LAYER_GROUPS + 4 * 11, 4);
}

/// The CRT's 64-bit left shift (`00ec7810`, `(value as 64-bit, shift)`).
const SHIFT_LEFT_64: u32 = 0x00ec_7810;
/// The size of the layer table in bytes (0x158, 43 layers of 8 bytes).
const LAYER_FLAGS_SIZE: u32 = 0x158;

/// The address of the layer table's entry of `layer` (`01267f20 + 8 * layer`, 32-bit wrapping
/// like the exe's address arithmetic).
fn layer_entry(layer: u32) -> u32 {
    LAYER_FLAGS.wrapping_add(layer.wrapping_mul(8))
}

/// The 64-bit mask with the bit of `layer`, through `__allshl` (`1 << layer`, the shift count
/// being the low byte of the layer).
fn layer_bit(e: &mut Engine, layer: u32) -> u64 {
    e.call(SHIFT_LEFT_64, &args![1u64, layer as u8]).u64()
}

/// Sets (`enable`) or clears the bits `bits` in the mask of `layer` in the layer table.
fn update_layer_mask(e: &mut Engine, layer: u32, bits: u64, enable: bool) {
    let entry = layer_entry(layer);
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let (old_low, old_high) = (e.mem.u32(entry), e.mem.u32(entry.wrapping_add(4)));
    if enable {
        e.mem.set_u32(entry, old_low | low);
        e.mem.set_u32(entry.wrapping_add(4), old_high | high);
    } else {
        e.mem.set_u32(entry, old_low & !low);
        e.mem.set_u32(entry.wrapping_add(4), old_high & !high);
    }
}

// Translated from 00c827f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00c827f0` (unnamed in the engine map), cdecl `(layer, other layer, enable byte)`: for the
/// layer pair, sets (enable byte nonzero) or clears the bit of the other layer in the mask of
/// the layer and the bit of the layer in the mask of the other layer, in the layer table at
/// `01267f20`. The bit of the other layer is made first (before the test of the byte), then
/// the bit of the layer, each through `__allshl`.
pub fn fn_00c827f0(e: &mut Engine, layer: u32, other_layer: u32, enable: u8) {
    let enable = enable != 0;
    let bit_of_other = layer_bit(e, other_layer);
    update_layer_mask(e, layer, bit_of_other, enable);
    let bit_of_layer = layer_bit(e, layer);
    update_layer_mask(e, other_layer, bit_of_layer, enable);
}

// Translated from 00c82870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionFilter::SetMultiLayerCollision` (engine map name), cdecl `(layer, low word,
/// high word)`: stores the 64-bit mask (high:low) as the layer's entry of the layer table at
/// `01267f20`, then for each of the 43 table entries sets the bit of `layer` (made once at the
/// start through `__allshl`) when the entry's own bit (1 << index) is in the mask and clears
/// it when not. The entry of the layer itself is part of the loop, so it can change after the
/// store.
pub fn bhk_collision_filter_set_multi_layer_collision(
    e: &mut Engine,
    layer: u32,
    low: u32,
    high: u32,
) {
    let bit_of_layer = layer_bit(e, layer);
    let entry = layer_entry(layer);
    e.mem.set_u32(entry, low);
    e.mem.set_u32(entry.wrapping_add(4), high);
    let mask = u64::from(low) | (u64::from(high) << 32);
    let mut probe = 1u64;
    let mut offset = 0u32;
    while offset < LAYER_FLAGS_SIZE {
        let target = LAYER_FLAGS + offset;
        let (old_low, old_high) = (e.mem.u32(target), e.mem.u32(target + 4));
        let (bit_low, bit_high) = (bit_of_layer as u32, (bit_of_layer >> 32) as u32);
        if probe & mask == 0 {
            e.mem.set_u32(target, old_low & !bit_low);
            e.mem.set_u32(target + 4, old_high & !bit_high);
        } else {
            e.mem.set_u32(target, old_low | bit_low);
            e.mem.set_u32(target + 4, old_high | bit_high);
        }
        probe <<= 1;
        offset += 8;
    }
}

/// The biped-part test (`00624070`, thiscall on the address of a filter info word, answers a
/// bool in `AL`; unnamed in the engine map, unit `fallout shared/havok`).
const FILTER_HAS_BIPED_PARTS: u32 = 0x0062_4070;

/// Whether the part bit of `second` (bits 8 to 12) is set in the word of `first`'s part (bits 8
/// to 12) in the table at `01268078` (the block at `00c847a1`).
fn parts_collide(e: &Engine, first: u32, second: u32) -> bool {
    let bit = 1u32 << ((second >> 8) & 0x1f);
    let word = e.mem.u32(PART_FLAGS + 4 * ((first >> 8) & 0x1f));
    bit & word != 0
}

/// Whether the 64-bit mask of `layer` in the layer table at `01267f20` (eight bytes per layer,
/// read for any layer up to 0x7f as the exe does) holds `bit`.
fn layer_mask_has(e: &Engine, layer: u32, bit: u64) -> bool {
    let entry = layer_entry(layer);
    let low = e.mem.u32(entry) & bit as u32;
    let high = e.mem.u32(entry.wrapping_add(4)) & (bit >> 32) as u32;
    low | high != 0
}

/// Asks `00624070` about the filter info `info`: the exe passes the address of its own stack
/// argument, here a stack word holding the same value.
fn has_biped_parts(e: &mut Engine, info: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), info);
        e.call(FILTER_HAS_BIPED_PARTS, &args![slot]).u32() as u8 != 0
    })
}

// Translated from 00c84740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionFilter::IsCollisionEnabled` (engine map name), cdecl `(first, second)` (two
/// filter info words): answers whether the two may collide. When the layer of `first` (low 7
/// bits) is not 0x28 and either word has bit 14 ("no collision") set, false. When either group
/// (high 16 bits) is zero, true. With both layers 8: false for different groups, otherwise the
/// part table at `01268078` decides (the word of `first`'s part must hold the bit of `second`'s
/// part). Otherwise, for different groups, whether the mask of `first`'s layer in the layer
/// table holds the bit of `second`'s layer (`__allshl(1, layer)`); for equal groups with bit 15
/// ("linked") set in both, false when that mask lacks the bit, else whether the two part numbers
/// (bits 8 to 12) differ by anything but exactly one; for equal groups without both linked
/// bits, `00624070` must answer true for `first` and then for `second` (false otherwise), and
/// the part table decides.
pub fn bhk_collision_filter_is_collision_enabled(e: &mut Engine, first: u32, second: u32) -> bool {
    let layer = first & 0x7f;
    if layer != 0x28 && ((first >> 14) & 1 != 0 || (second >> 14) & 1 != 0) {
        return false;
    }
    if first & 0xffff_0000 == 0 || second & 0xffff_0000 == 0 {
        return true;
    }
    let group_difference = (first ^ second) & 0xffff_0000;
    let second_layer = second & 0x7f;
    if layer == 8 && second_layer == layer {
        if group_difference != 0 {
            return false;
        }
        return parts_collide(e, first, second);
    }
    if group_difference != 0 {
        let bit = layer_bit(e, second_layer);
        return layer_mask_has(e, layer, bit);
    }
    if first & second & 0x8000 != 0 {
        let bit = layer_bit(e, second_layer);
        if !layer_mask_has(e, layer, bit) {
            return false;
        }
        let difference = (((first >> 8) & 0x1f) as i32).wrapping_sub(((second >> 8) & 0x1f) as i32);
        return difference.wrapping_abs() != 1;
    }
    if !has_biped_parts(e, first) || !has_biped_parts(e, second) {
        return false;
    }
    parts_collide(e, first, second)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c8_4740,
            bhk_collision_filter_is_collision_enabled(u32, u32) -> bool
        ),
        entry!(0x00c84470, bhk_collision_filter_get_debug_color(u32, Ptr) -> Ptr),
        entry!(0x00c84880, fn_00c84880(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c848b0, fn_00c848b0(Ptr, Ptr, u32, Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00c84900, fn_00c84900(Ptr, Ptr, Ptr, u32, Ptr, u32) -> Ptr),
        entry!(0x00c84930, fn_00c84930(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c84960, fn_00c84960(Ptr, Ptr, u32, Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00c849b0, c_filter_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c827f0, fn_00c827f0(u32, u32, u8)),
        entry!(
            0x00c82870,
            bhk_collision_filter_set_multi_layer_collision(u32, u32, u32)
        ),
        entry!(0x00c828f0, bhk_collision_filter_init_flags()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The words of the color a call of `GetDebugColor` for `filter` wrote.
    fn debug_color(e: &mut Engine, filter: u32) -> [u32; 3] {
        let color = e.mem.alloc(0xc);
        let result = e.call(0x00c8_4470, &args![filter, color]).u32();
        assert_eq!(result, color);
        [0, 4, 8].map(|offset| e.mem.u32(color + offset))
    }

    #[test]
    fn bhk_collision_filter_get_debug_color_picks_the_color_of_the_layer() {
        let mut e = logged_engine();
        // Every constant the table reads holds its own address as a float.
        for address in [
            0x0101_77e8u32,
            0x010c_5818,
            0x010c_5814,
            0x010c_5810,
            0x0103_0ff0,
            0x0101_8238,
            0x0104_e2b8,
        ] {
            e.mem.set_f32(address, address as f32);
        }
        for (index, word) in [0x1111u32, 0x2222, 0x3333].into_iter().enumerate() {
            e.mem.set_u32(DEFAULT_DEBUG_COLOR + 4 * index as u32, word);
        }
        let one = 1.0f32.to_bits();
        let value = |address: u32| (address as f32).to_bits();
        // Layer 1: (1, 0, 0).
        assert_eq!(debug_color(&mut e, 1), [one, 0, 0]);
        // The bits above the layer are ignored; layers 2 and 10 share a color.
        let layer_two = [value(0x0101_77e8), value(0x010c_5818), value(0x010c_5814)];
        assert_eq!(debug_color(&mut e, 0xabcd_0082), layer_two);
        assert_eq!(debug_color(&mut e, 10), layer_two);
        // Layer 3 repeats the second constant in the third place.
        assert_eq!(
            debug_color(&mut e, 3),
            [value(0x010c_5810), value(0x0103_0ff0), value(0x0103_0ff0)]
        );
        // Layers 8, 18, 30 and 31.
        assert_eq!(debug_color(&mut e, 8), [0, one, 0]);
        assert_eq!(debug_color(&mut e, 18), [0, value(0x0101_8238), 0]);
        assert_eq!(debug_color(&mut e, 30), [one, one, 0]);
        assert_eq!(debug_color(&mut e, 31), [one, value(0x0103_0ff0), 0]);
        // Layers outside the table (0, 15, 20 and 32 and up) copy the default words.
        for filter in [0u32, 15, 20, 32, 0x7f, 0x80] {
            assert_eq!(
                debug_color(&mut e, filter),
                [0x1111, 0x2222, 0x3333],
                "{filter}"
            );
        }
        // A signalling NaN comes out quiet; the default words are copied as they are.
        e.mem.set_u32(0x0101_77e8, 0x7f80_0001);
        assert_eq!(debug_color(&mut e, 2)[0], 0x7fc0_0001);
        e.mem.set_u32(DEFAULT_DEBUG_COLOR, 0x7f80_0001);
        assert_eq!(debug_color(&mut e, 0)[0], 0x7f80_0001);
    }

    #[test]
    fn fn_00c84880_compares_the_filter_infos_of_two_collidables() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] < a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x1c, 5);
        e.mem.set_u32(second + 0x1c, 9);
        let result = e
            .call(0x00c8_4880, &args![0x1234u32, out, first, second])
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![5, 9]]);
        e.call(0x00c8_4880, &args![0x1234u32, out, second, first]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED)[1], vec![9, 5]);
    }

    /// Two bodies, each a chain `body -> parent -> root` whose root has the filter info.
    fn body_chain(e: &mut Engine, info: u32) -> u32 {
        let root = e.mem.alloc(0x20);
        e.mem.set_u32(root + 0x1c, info);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(parent + 0xc, root);
        e.mem.set_u32(parent + 0x1c, 0xdead);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0xc, parent);
        e.mem.set_u32(body + 0x1c, 0xbeef);
        body
    }

    #[test]
    fn fn_00c848b0_compares_the_filter_infos_of_the_roots_of_two_bodies() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] < a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = body_chain(&mut e, 3);
        let second = body_chain(&mut e, 8);
        let result = e
            .call(
                0x00c8_48b0,
                &args![0u32, out, 1u32, first, second, 2u32, 3u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![3, 8]]);
        // A body without parent is its own root.
        let alone = e.mem.alloc(0x20);
        e.mem.set_u32(alone + 0x1c, 1);
        e.call(
            0x00c8_48b0,
            &args![0u32, out, 1u32, alone, second, 2u32, 3u32],
        );
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED)[1], vec![1, 8]);
    }

    #[test]
    fn fn_00c84900_compares_the_ray_filter_with_the_one_the_container_answers() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] == a[1]) as u32).into_ret()
        });
        e.register_double(0x7000_0010, |_, a| (a[1] + 0x100).into_ret());
        e.put_vtable(0x7001_1000, &[0, 0, 0, 0, 0x7000_0010]);
        let container = e.mem.alloc(0x10);
        e.mem.set_u32(container, 0x7001_1000);
        let out = e.mem.alloc(4);
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input + 0x20, 0x107);
        let result = e
            .call(0x00c8_4900, &args![0u32, out, input, 0u32, container, 7u32])
            .u32();
        assert_eq!(result, out);
        assert_eq!(calls_to(&e, 0x7000_0010), vec![vec![container, 7]]);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![0x107, 0x107]]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(input + 0x20, 0x108);
        e.call(0x00c8_4900, &args![0u32, out, input, 0u32, container, 7u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00c84930_compares_the_world_ray_filter_with_the_collidable() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] > a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input + 0x24, 20);
        let collidable = e.mem.alloc(0x20);
        e.mem.set_u32(collidable + 0x1c, 10);
        let result = e
            .call(0x00c8_4930, &args![0u32, out, input, collidable])
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![20, 10]]);
    }

    #[test]
    fn fn_00c84960_compares_the_filter_infos_of_the_roots_of_two_bodies() {
        let mut e = logged_engine();
        e.register_double(IS_COLLISION_ENABLED, |_, a| {
            ((a[0] > a[1]) as u32).into_ret()
        });
        let out = e.mem.alloc(4);
        let first = body_chain(&mut e, 30);
        let second = body_chain(&mut e, 4);
        let result = e
            .call(
                0x00c8_4960,
                &args![0u32, out, 0u32, first, second, 1u32, 2u32, 3u32, 4u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, IS_COLLISION_ENABLED), vec![vec![30, 4]]);
    }

    #[test]
    fn c_filter_get_viewer_strings_stores_one_string_per_field() {
        let mut e = logged_engine();
        e.register(FORMAT, |_, _| Ret::default());
        e.register(VIEWER_STRING_OF_TEXT, |_, a| {
            (0x6000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_OF_BOOL, |_, a| {
            (0x7000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_OF_INT, |_, a| {
            (0x8000_0000 | (a[0] & 0xffff)).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        // The layer and part name tables.
        for index in 0..0x80u32 {
            e.mem.set_u32(LAYER_NAMES + 4 * index, 0x7500_0000 + index);
        }
        for index in 0..0x20u32 {
            e.mem.set_u32(PART_NAMES + 4 * index, 0x7600_0000 + index);
        }
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x80);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0x20);
        e.mem.set_u16(array + 0xe, 4);
        let filter = e.mem.alloc(8);
        // Group 0x1234, linked, scaled, part 5, layer 8.
        let info = 0x1234_0000u32 | 0x8000 | 0x2000 | 0x0500 | 0x08;
        e.mem.set_u32(filter, info);
        e.call(0x00c8_49b0, &args![filter, array]);
        let texts: Vec<u32> = (0..7).map(|index| e.mem.u32(base + 4 * index)).collect();
        assert_eq!(
            texts,
            vec![
                0x6000_0000 | (LABEL_FILTER & 0xffff),
                0x6000_0000 | (LABEL_GROUP & 0xffff),
                0x6000_0000 | (LABEL_LAYER & 0xffff),
                0x7000_0000 | (LABEL_LINK & 0xffff),
                0x6000_0000 | (LABEL_PART & 0xffff),
                0x7000_0000 | (LABEL_NO_COLLISION & 0xffff),
                0x7000_0000 | (LABEL_SCALED & 0xffff),
            ]
        );
        assert_eq!(e.mem.u16(array + 0xa), 7);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[0][1..], [0x80, FORMAT_FILTER, info]);
        assert_eq!(formats[1][1..], [0x80, FORMAT_GROUP, 0x1234]);
        // Each text is made from the formatted buffer.
        let buffer = formats[0][0];
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[0],
            vec![LABEL_FILTER, buffer]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[2],
            vec![LABEL_LAYER, 0x7500_0008]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_TEXT)[3],
            vec![LABEL_PART, 0x7600_0005]
        );
        // The bool words keep the high bytes of the previous string pointer: the layer text
        // before "-LINK", the part text before "-NOCOL", "-NOCOL"'s before "-SCALED".
        let bools = calls_to(&e, VIEWER_STRING_OF_BOOL);
        assert_eq!(bools[0], vec![LABEL_LINK, 0x6000_5800 | 1]);
        assert_eq!(bools[1], vec![LABEL_NO_COLLISION, 0x6000_5800]);
        assert_eq!(bools[2], vec![LABEL_SCALED, 0x7000_5800 | 1]);
    }

    #[test]
    fn c_filter_get_viewer_strings_uses_the_part_number_of_a_linked_filter() {
        let mut e = logged_engine();
        e.register(FORMAT, |_, _| Ret::default());
        e.register(VIEWER_STRING_OF_TEXT, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_OF_BOOL, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_OF_INT, |_, a| {
            (0x8000_0000 | (a[1] & 0xffff)).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x80);
        e.mem.set_u32(array + 4, base);
        // A full array grows by its grow-by before the first store.
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let filter = e.mem.alloc(8);
        // Linked, part 6, layer 3: the part number is the string; no "-PART" name lookup.
        e.mem.set_u32(filter, 0x8000 | 0x0600 | 0x03);
        e.call(0x00c8_49b0, &args![filter, array]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OF_INT),
            vec![vec![LABEL_PART, 6]]
        );
        assert_eq!(e.mem.u16(array + 0xa), 7);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE)[0], vec![array, 4]);
        // Without the link bit, a layer other than 8 and 0x1d stores no "-PART" string.
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u32(filter, 0x0600 | 0x03);
        e.call(0x00c8_49b0, &args![filter, array]);
        assert_eq!(calls_to(&e, VIEWER_STRING_OF_INT).len(), 1);
        assert_eq!(e.mem.u16(array + 0xa), 6);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    /// The words of each layer pair call, with the low word of layer 0x21 and the high word of
    /// layer 0xb at that call (the words of the pair `InitFlags` sets by hand).
    type PairCalls = Rc<RefCell<Vec<(Vec<u32>, u32, u32)>>>;

    /// An engine whose `memset` fills (`fill`) or does nothing, with the layer pair calls
    /// recorded and `SetMultiLayerCollision` doubled.
    fn init_flags_engine(fill: bool) -> (Engine, PairCalls) {
        let mut e = logged_engine();
        if fill {
            e.register(MEMSET, |e, a| {
                for offset in 0..a[2] {
                    e.mem.set_u8(a[0] + offset, a[1] as u8);
                }
                Ret::default()
            });
        } else {
            e.register(MEMSET, |_, _| Ret::default());
        }
        let pairs: PairCalls = Rc::default();
        let seen = Rc::clone(&pairs);
        e.register_double(LAYER_PAIR_COLLISION, move |e, a| {
            let low_of_0x21 = e.mem.u32(0x0126_8028);
            let high_of_0xb = e.mem.u32(0x0126_7f7c);
            seen.borrow_mut()
                .push((a.to_vec(), low_of_0x21, high_of_0xb));
            Ret::default()
        });
        e.register(SET_MULTI_LAYER_COLLISION, |_, _| Ret::default());
        (e, pairs)
    }

    #[test]
    fn bhk_collision_filter_init_flags_fills_the_tables_then_sets_the_layer_pairs() {
        let (mut e, pairs) = init_flags_engine(true);
        // The word between the table at 01267e70 and the layer table belongs to neither.
        e.mem.set_u32(0x0126_7f1c, 0x5a5a_5a5a);
        e.mem.set_u32(LAYER_GROUPS, 7);
        e.call(0x00c8_28f0, &args![]);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![
                vec![PART_FLAGS, 0, 0x80],
                vec![LAYER_FLAGS, 0xff, 0x158],
                vec![LAYER_GROUPS, 0, 0xac],
            ]
        );
        // The bits set on the zeroed table at 01268078; its first word is not touched.
        assert_eq!(e.mem.u32(0x0126_80d0), 0x0037_fffe);
        assert_eq!(e.mem.u32(0x0126_807c), 0x0042_30c0);
        assert_eq!(e.mem.u32(PART_FLAGS), 0);
        // The bits cleared (and set) on the layer table filled with ones.
        assert_eq!(e.mem.u32(0x0126_7fb8), 0x2bfa_6637);
        assert_eq!(e.mem.u32(0x0126_7fc8), 0x43dc_0000);
        assert_eq!(e.mem.u32(0x0126_7fcc), 0xffff_fc17);
        assert_eq!(e.mem.u32(0x0126_8010), 0xdff3_feff);
        assert_eq!(e.mem.u32(0x0126_7f24), 0xffff_ffff);
        assert_eq!(e.mem.u32(0x0126_7f1c), 0x5a5a_5a5a);
        // The table at 01267e70: zeroed, then words 13, 9 and 11.
        assert_eq!(e.mem.u32(LAYER_GROUPS), 0);
        assert_eq!(
            [13, 9, 11].map(|index| e.mem.u32(LAYER_GROUPS + 4 * index)),
            [2, 3, 4]
        );
        let pairs = pairs.borrow();
        assert_eq!(pairs.len(), 272);
        assert!(pairs.iter().all(|pair| pair.0[2] == 0));
        assert_eq!(pairs[0].0, vec![0xf, 0x1e, 0]);
        assert_eq!(pairs[20].0, vec![0x21, 0xa, 0]);
        assert_eq!(pairs[21].0, vec![0x21, 0xc, 0]);
        assert_eq!(pairs[100].0, vec![6, 3, 0]);
        assert_eq!(pairs[271].0, vec![3, 0xd, 0]);
        // SetMultiLayerCollision comes last; nothing else is called.
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.len(), 1 + 3 + 272 + 1);
        assert_eq!(
            log.last().unwrap(),
            &(SET_MULTI_LAYER_COLLISION, vec![0x22, 0x1000_0004, 0])
        );
    }

    #[test]
    fn bhk_collision_filter_init_flags_keeps_the_bits_its_masks_keep() {
        let (mut e, pairs) = init_flags_engine(false);
        e.mem.set_u32(0x0126_8010, 0x1234_5678);
        e.mem.set_u32(0x0126_80d0, 0x8000_0001);
        e.mem.set_u32(0x0126_7f24, 0x8765_4321);
        e.call(0x00c8_28f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_8010), 0x1230_5e78);
        assert_eq!(e.mem.u32(0x0126_80d0), 0x8037_ffff);
        // A word the exe stores back unchanged.
        assert_eq!(e.mem.u32(0x0126_7f24), 0x8765_4321);
        // The setup is done before the first pair call; the pair (0x21, 0xb) is set between the
        // calls for (0x21, 0xa) and (0x21, 0xc).
        let pairs = pairs.borrow();
        assert_eq!((pairs[0].1, pairs[0].2), (0x0020_0000, 0));
        assert_eq!((pairs[20].1, pairs[20].2), (0x0020_0000, 0));
        assert_eq!((pairs[21].1, pairs[21].2), (0x0020_0800, 2));
        assert_eq!((pairs[271].1, pairs[271].2), (0x0020_0800, 2));
        assert_eq!(e.mem.u32(0x0126_8028), 0x0020_0800);
        assert_eq!(e.mem.u32(0x0126_802c), 0);
    }

    /// `__allshl` (`00ec7810`) is an unplaced unit of `engine-fallout`, which this crate's tests
    /// do not link: a double (the 64-bit value shifted left by the count; 0 from 64 on).
    fn allshl_double(e: &mut Engine) {
        e.register(SHIFT_LEFT_64, |_, a| {
            let value = u64::from(a[0]) | (u64::from(a[1]) << 32);
            let count = a[2] as u8;
            (if count >= 0x40 { 0 } else { value << count }).into_ret()
        });
    }

    fn layer_entry_words(e: &Engine, layer: u32) -> [u32; 2] {
        [
            e.mem.u32(layer_entry(layer)),
            e.mem.u32(layer_entry(layer) + 4),
        ]
    }

    #[test]
    fn fn_00c827f0_sets_the_bit_of_each_layer_in_the_mask_of_the_other() {
        let mut e = logged_engine();
        allshl_double(&mut e);
        e.call(0x00c8_27f0, &args![3u32, 0x22u32, 1u32]);
        // Layer 3 gets the bit of layer 0x22 (bit 2 of the high word); layer 0x22 gets bit 3.
        assert_eq!(layer_entry_words(&e, 3), [0, 4]);
        assert_eq!(layer_entry_words(&e, 0x22), [8, 0]);
        // The bit of the other layer is made first, then the bit of the layer.
        let shifts = calls_to(&e, SHIFT_LEFT_64);
        assert_eq!(shifts, vec![vec![1, 0, 0x22], vec![1, 0, 3]]);
        // Any nonzero low byte enables; the bits already there stay.
        e.call(0x00c8_27f0, &args![3u32, 0u32, 0x100u32 + 0x80]);
        assert_eq!(layer_entry_words(&e, 3), [1, 4]);
        assert_eq!(layer_entry_words(&e, 0), [8, 0]);
        // The same layer twice sets its own bit in its mask.
        e.call(0x00c8_27f0, &args![5u32, 5u32, 1u32]);
        assert_eq!(layer_entry_words(&e, 5), [0x20, 0]);
        // Only the low byte of the third word is the enable flag.
        e.call(0x00c8_27f0, &args![6u32, 7u32, 0x100u32]);
        assert_eq!(layer_entry_words(&e, 6), [0, 0]);
        assert_eq!(layer_entry_words(&e, 7), [0, 0]);
    }

    #[test]
    fn fn_00c827f0_clears_the_bit_of_each_layer_in_the_mask_of_the_other() {
        let mut e = logged_engine();
        allshl_double(&mut e);
        for layer in 0..43 {
            e.mem.set_u32(layer_entry(layer), 0xffff_ffff);
            e.mem.set_u32(layer_entry(layer) + 4, 0xffff_ffff);
        }
        e.call(0x00c8_27f0, &args![3u32, 0x22u32, 0u32]);
        assert_eq!(layer_entry_words(&e, 3), [0xffff_ffff, 0xffff_fffb]);
        assert_eq!(layer_entry_words(&e, 0x22), [0xffff_fff7, 0xffff_ffff]);
        assert_eq!(layer_entry_words(&e, 4), [0xffff_ffff; 2]);
        e.call(0x00c8_27f0, &args![5u32, 5u32, 0u32]);
        assert_eq!(layer_entry_words(&e, 5), [0xffff_ffdf, 0xffff_ffff]);
    }

    #[test]
    fn bhk_collision_filter_set_multi_layer_collision_sets_the_layer_in_the_masks_it_names() {
        let mut e = logged_engine();
        allshl_double(&mut e);
        for layer in 0..44 {
            e.mem.set_u32(layer_entry(layer), 0x0f0f_0f0f);
            e.mem.set_u32(layer_entry(layer) + 4, 0xf0f0_f0f0);
        }
        // The mask names the layers 0, 2 (low word) and 34 (bit 2 of the high word).
        e.call(0x00c8_2870, &args![7u32, 0b101u32, 4u32]);
        // The mask is stored, then the bit of layer 7 (0x80) is cleared in the entries of the
        // layer itself and of the layers outside the mask, and set in the others.
        assert_eq!(layer_entry_words(&e, 7), [0b101, 4]);
        assert_eq!(layer_entry_words(&e, 0), [0x0f0f_0f8f, 0xf0f0_f0f0]);
        assert_eq!(layer_entry_words(&e, 2), [0x0f0f_0f8f, 0xf0f0_f0f0]);
        assert_eq!(layer_entry_words(&e, 34), [0x0f0f_0f8f, 0xf0f0_f0f0]);
        assert_eq!(layer_entry_words(&e, 1), [0x0f0f_0f0f, 0xf0f0_f0f0]);
        assert_eq!(layer_entry_words(&e, 42), [0x0f0f_0f0f, 0xf0f0_f0f0]);
        // The entry past the table (43 entries) is left alone.
        assert_eq!(layer_entry_words(&e, 43), [0x0f0f_0f0f, 0xf0f0_f0f0]);
        // The bit of the layer is made once, before the mask is stored.
        assert_eq!(calls_to(&e, SHIFT_LEFT_64), vec![vec![1, 0, 7]]);
    }

    #[test]
    fn bhk_collision_filter_set_multi_layer_collision_changes_its_own_entry_in_the_loop() {
        let mut e = logged_engine();
        allshl_double(&mut e);
        // Layer 0x22 (bit 2 of the high word) in its own mask: its entry gets its own bit.
        e.call(0x00c8_2870, &args![0x22u32, 0u32, 4u32]);
        assert_eq!(layer_entry_words(&e, 0x22), [0, 4]);
        // An empty mask clears the bit of the layer everywhere.
        for layer in 0..43 {
            e.mem.set_u32(layer_entry(layer), 0xffff_ffff);
            e.mem.set_u32(layer_entry(layer) + 4, 0xffff_ffff);
        }
        e.call(0x00c8_2870, &args![1u32, 0u32, 0u32]);
        assert_eq!(layer_entry_words(&e, 1), [0, 0]);
        assert_eq!(layer_entry_words(&e, 0), [0xffff_fffd, 0xffff_ffff]);
        assert_eq!(layer_entry_words(&e, 42), [0xffff_fffd, 0xffff_ffff]);
    }
}

#[cfg(test)]
mod is_collision_enabled_tests {
    use super::*;

    /// An engine with the exe's data pages, a `__allshl` double (the 64-bit value shifted left
    /// by the count, 0 from 64 on) and a double for `00624070` that answers whether the layer of
    /// the word its argument points at is 8 or 0x1d.
    fn filter_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e.register(SHIFT_LEFT_64, |_, a| {
            let value = u64::from(a[0]) | (u64::from(a[1]) << 32);
            let count = a[2] as u8;
            (if count >= 0x40 { 0 } else { value << count }).into_ret()
        });
        e.register(FILTER_HAS_BIPED_PARTS, |e, a| {
            let layer = e.mem.u32(a[0]) & 0x7f;
            u32::from(layer == 8 || layer == 0x1d).into_ret()
        });
        e
    }

    fn enabled(e: &mut Engine, first: u32, second: u32) -> bool {
        e.call(0x00c8_4740, &args![first, second]).u32() != 0
    }

    fn calls_to(e: &Engine, address: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .count()
    }

    #[test]
    fn bhk_collision_filter_is_collision_enabled_follows_the_groups_layers_and_parts() {
        let mut e = filter_engine();
        // No-collision bit: false, unless the first layer is 0x28.
        assert!(!enabled(&mut e, 0x0001_4001, 0x0002_0001));
        assert!(!enabled(&mut e, 0x0001_0001, 0x0002_4001));
        // A zero group: true (also for layer 0x28 with the no-collision bit).
        assert!(enabled(&mut e, 0x0000_4028, 0x0002_4001));
        assert!(enabled(&mut e, 0x0001_0001, 0x0000_0001));
        // Different groups: the first layer's mask in the layer table (layer 3 holds layer 0x22,
        // bit 2 of its high word).
        e.mem.set_u32(layer_entry(3) + 4, 4);
        assert!(enabled(&mut e, 0x0001_0003, 0x0002_0022));
        assert!(!enabled(&mut e, 0x0001_0003, 0x0002_0021));
        assert_eq!(calls_to(&e, SHIFT_LEFT_64), 2);
        // Both layers 8: different groups never, the same group by the part table (part 2's
        // word holds part 5's bit).
        e.mem.set_u32(PART_FLAGS + 4 * 2, 1 << 5);
        assert!(!enabled(&mut e, 0x0001_0208, 0x0002_0508));
        assert!(enabled(&mut e, 0x0001_0208, 0x0001_0508));
        assert!(!enabled(&mut e, 0x0001_0508, 0x0001_0208));
        assert_eq!(calls_to(&e, SHIFT_LEFT_64), 2);
    }

    #[test]
    fn bhk_collision_filter_is_collision_enabled_handles_linked_and_biped_filters_of_one_group() {
        let mut e = filter_engine();
        // Linked in both, same group: the mask must hold the bit, then the parts must not be
        // neighbours.
        e.mem.set_u32(layer_entry(4), 1 << 6);
        assert!(!enabled(&mut e, 0x0001_8304, 0x0001_8405));
        assert!(!enabled(&mut e, 0x0001_8304, 0x0001_8406));
        assert!(!enabled(&mut e, 0x0001_8404, 0x0001_8306));
        assert!(enabled(&mut e, 0x0001_8304, 0x0001_8306));
        assert!(enabled(&mut e, 0x0001_8304, 0x0001_8506));
        assert!(enabled(&mut e, 0x0001_8304, 0x0001_8706));
        // Not both linked: `00624070` for the first word, then the second, then the part table.
        e.mem.set_u32(PART_FLAGS + 4 * 3, 1 << 4);
        e.call_log = Some(vec![]);
        assert!(!enabled(&mut e, 0x0001_8301, 0x0001_041d));
        assert_eq!(calls_to(&e, FILTER_HAS_BIPED_PARTS), 1);
        assert!(!enabled(&mut e, 0x0001_031d, 0x0001_0401));
        assert_eq!(calls_to(&e, FILTER_HAS_BIPED_PARTS), 3);
        assert!(enabled(&mut e, 0x0001_031d, 0x0001_8408));
        assert!(!enabled(&mut e, 0x0001_031d, 0x0001_8508));
        assert_eq!(calls_to(&e, FILTER_HAS_BIPED_PARTS), 7);
        assert_eq!(calls_to(&e, SHIFT_LEFT_64), 0);
    }
}

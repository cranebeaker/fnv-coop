//! `fallout/misc/saveload/bgssaveloadstructs.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00853500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNumericIDIndex::GetNumericID` (Xbox PDB): reads the big-endian
/// three-byte index. Bit 23 denotes a created form; other values use the
/// save manager's reverse map.
pub fn get_numeric_id(e: &mut Engine, this: Ptr) -> u32 {
    let index = ((e.mem.u8(this.addr()) as u32) << 16)
        | ((e.mem.u8(this.addr().wrapping_add(1)) as u32) << 8)
        | e.mem.u8(this.addr().wrapping_add(2)) as u32;
    if index & 0x800000 != 0 {
        (index & 0x7fffff) | 0xff000000
    } else {
        let manager = e.mem.u32(0x011ddf38);
        let map = e.call(0x0044ddc0, &args![manager]).u32();
        e.call(0x00846d20, &args![map, index]).u32()
    }
}

// Translated from 00853570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNumericIDIndex::SetNumericID` (Xbox PDB): resolves the save index,
/// marks created IDs with bit 23 and writes exactly three bytes.
pub fn set_numeric_id(e: &mut Engine, this: Ptr, id: u32) {
    let manager = e.mem.u32(0x011ddf38);
    let map = e.call(0x0044ddc0, &args![manager]).u32();
    let mut index = e.call(0x00846c90, &args![map, id]).u32();
    if index >> 24 == 0xff {
        index |= 0x800000;
    }
    e.mem.set_u8(this.addr(), (index >> 16) as u8);
    e.mem
        .set_u8(this.addr().wrapping_add(1), (index >> 8) as u8);
    e.mem.set_u8(this.addr().wrapping_add(2), index as u8);
}

/// The byte table `008535e0` fills: `0x79` bytes at `011de360`, indexed by
/// the values of the dword table at `011a2428`.
const INDEX_BYTES: u32 = 0x011d_e360;
/// The number of entries of the dword table, kept in a global the loop
/// re-reads every pass.
const INDEX_COUNT: u32 = 0x011d_e3e0;
/// The dword table at `011a2428`.
const INDEX_OFFSETS: u32 = 0x011a_2428;

// Translated from 008535e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the initializer `BGSSaveLoadGame::Create`
/// runs last): fills the 0x79 bytes at `011de360` with 0xff (`memset`),
/// stores 0x37 in `011de3e0`, then for `i` from 0 while `i < [011de3e0]`
/// writes the byte `i` at `011de360 + table[i]` (`table` is the dword array
/// at `011a2428`).
pub fn fn_008535e0(e: &mut Engine) {
    e.call(0x0040_3d30, &args![INDEX_BYTES, 0xffu32, 0x79u32]);
    e.set_global::<u32>(INDEX_COUNT, 0x37);
    let mut i = 0u32;
    while i < e.global::<u32>(INDEX_COUNT) {
        let offset = e.mem.u32(INDEX_OFFSETS.wrapping_add(i.wrapping_mul(4)));
        e.mem.set_u8(offset.wrapping_add(INDEX_BYTES), i as u8);
        i += 1;
    }
}

/// The vtable of `BGSSaveLoadStatsMap` (RTTI `.?AVBGSSaveLoadStatsMap@@`).
const STATS_MAP_VTABLE: u32 = 0x0108_0174;
/// The vtable of `NiTMap<ENUM_FORM_ID, BSSimpleList<SavedFormData*>*>`.
const FORM_MAP_VTABLE: u32 = 0x0108_0434;
/// The vtable of the `NiTMapBase<..>` that class derives from.
const FORM_MAP_BASE_VTABLE: u32 = 0x0108_0454;
/// The size of the report buffers (`sprintf_s` size argument, 0x3e8).
const LINE_SIZE: u32 = 1000;
/// The size of the save form buffer (`008659c0` constructs it on the
/// stack): the largest offset used is `+0x20` (`0050f9c0`).
const SAVE_BUFFER_SIZE: u32 = 0x24;
/// The save manager global whose map the report walks (`[011ddf38]`).
const SAVE_MANAGER: u32 = 0x011d_df38;
/// The table of the name of each form type: three words per entry, the
/// first the name (`01187004`).
const FORM_TYPE_NAMES: u32 = 0x0118_7004;
/// The empty string the report fills its buffers from (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;

// Translated from 00853640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reads the low six bits of the byte at `this`
/// (the bits `BGSSaveLoadFormInfo::SetFormType`, `00853670`, writes) and
/// returns that entry of the dword table at `011a2428`.
pub fn fn_00853640(e: &mut Engine, this: Ptr) -> u32 {
    let index = (e.mem.u8(this.addr()) & 0x3f) as u32;
    e.mem.u32(INDEX_OFFSETS.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00853670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFormInfo::SetFormType` (Xbox PDB): looks `form_type` up in
/// the byte table at `011de360`; when the byte is not below the entry count
/// in `011de3e0` it reports `SaveGameWarning` (`00845cc0`, with the name
/// `004612b0` gives for the low byte); then stores the byte in the low six
/// bits of `*this`, keeping the top two.
pub fn bgs_save_load_form_info_set_form_type(e: &mut Engine, this: Ptr, form_type: u32) {
    let index = e.mem.u8(INDEX_BYTES.wrapping_add(form_type));
    if index as u32 >= e.global::<u32>(INDEX_COUNT) {
        let name = e.call(0x004612b0, &args![form_type & 0xff]).u32();
        e.call(0x00845cc0, &args![0x0108_0124u32, form_type, name]);
    }
    let old = e.mem.u8(this.addr());
    e.mem.set_u8(this.addr(), (old & 0xc0) | index);
}

// Translated from 008536d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SavedFormData::GetFormName` (Xbox PDB): writes the name of the form at
/// `this + 0x14` into `dst` (capacity `size`). No form: `"UNLOADED"`. A form
/// whose virtual `+0xf0` answers true gives `0055d520`, any other
/// `TESFullName::GetFullName` (`00482720`); an empty result takes the string
/// of virtual `+0x130`; still empty and form type `0x46` (`00401170`), the
/// string of virtual `+0x138` on `LowProcess::GetSecondGenericLocation`
/// (`006733e0`).
pub fn saved_form_data_get_form_name(e: &mut Engine, this: Ptr, dst: Ptr, size: u32) {
    let form = e.mem.u32(this.addr().wrapping_add(0x14));
    if form == 0 {
        e.call(0x00406d30, &args![dst, size, 0x0108_0164u32]);
        return;
    }
    let object = if e.vcall(form, 0xf0, &args![]).bool() {
        form
    } else {
        0
    };
    let name = if object != 0 {
        e.call(0x0055d520, &args![object]).u32()
    } else {
        e.call(0x00482720, &args![form]).u32()
    };
    e.call(0x00406d30, &args![dst, size, name]);
    if e.call(0x0044a670, &args![dst]).u32() == 0 {
        let name = e.vcall(form, 0x130, &args![]).u32();
        e.call(0x00406d30, &args![dst, size, name]);
    }
    if e.call(0x0044a670, &args![dst]).u32() == 0 && e.call(0x00401170, &args![form]).u32() == 0x46
    {
        let location = e.call(0x006733e0, &args![form]).u32();
        let name = e.vcall(location, 0x138, &args![]).u32();
        e.call(0x00406d30, &args![dst, size, name]);
    }
}

// Translated from 00853810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00853810 - the C++ exception-unwinding frame (FS:[0], unwind table f13208) is not translated
/// Unnamed in the engine map: the constructor of `BGSSaveLoadStatsMap` (its
/// vtable `01080174` is the one `00853880` destroys): constructs the
/// `NiTMap` base with 0x25 buckets (`00854c80`), stores the vtable, constructs
/// the list at `+0x10` (`0096a2d0`) and clears the file pointer at `+0x18`.
pub fn fn_00853810(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00854c80(e, this, 0x25);
    e.mem.set_u32(this.addr(), STATS_MAP_VTABLE);
    e.call(0x0096a2d0, &args![this.addr().wrapping_add(0x10)]);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    this
}

// Translated from 00853880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadStatsMap::_scalar_deleting_destructor_` (Xbox PDB): destroys
/// (`008538b0`) and, when bit 0 of `flags` is set, frees the object.
pub fn bgs_save_load_stats_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008538b0(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

/// Frees what a stats-map node list holds: each node's item is released
/// with `operator delete` (`00401030`), node after node (`00726070`).
fn free_list_items(e: &mut Engine, first_node: u32) {
    let mut node = first_node;
    while node != 0 {
        let slot = e.call(0x006815c0, &args![node]).u32();
        let item = e.mem.u32(slot);
        e.call(0x00401030, &args![item]);
        node = e.call(0x00726070, &args![node]).u32();
    }
}

// Translated from 008538b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008538b0 - the C++ exception-unwinding frame (FS:[0], unwind table f13243) is not translated
/// Unnamed in the engine map: the destructor of `BGSSaveLoadStatsMap`. Sets
/// the vtable, destroys the file at `+0x18` through its virtual slot 0 (with
/// argument 1), frees every item of every list the map holds, then every
/// item of the list at `+0x10`, destroys that list (`0046ffb0`) and the
/// `NiTMap` base (`00854d50`).
pub fn fn_008538b0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, STATS_MAP_VTABLE);
    let file = e.mem.u32(t.wrapping_add(0x18));
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.with_stack(12, |e, locals| {
        let iter = locals.addr();
        let key = iter + 4;
        let list = iter + 8;
        let first = e.call(0x004b9ba0, &args![this]).u32();
        e.mem.set_u32(iter, first);
        while e.mem.u32(iter) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(list, 0);
            e.call(0x006b7f20, &args![this, iter, key, list]);
            let first_node = e.mem.u32(list);
            free_list_items(e, first_node);
        }
    });
    free_list_items(e, t.wrapping_add(0x10));
    e.call(0x0046ffb0, &args![t.wrapping_add(0x10)]);
    fn_00854d50(e, this);
}

// Translated from 008539e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destroys the file at `this + 0x18` through its
/// virtual slot 0 (argument 1) when there is one, and clears the pointer.
pub fn fn_008539e0(e: &mut Engine, this: Ptr) {
    let file = e.mem.u32(this.addr().wrapping_add(0x18));
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
}

// Translated from 00853a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00853a30 - the C++ exception-unwinding frame (FS:[0], unwind table f13276) and the /GS cookie check 00ec408c are not translated
/// Unnamed in the engine map (cdecl, one argument): writes the save-game
/// statistics report to `<path>.txt` and opens it. Builds a
/// `BGSSaveLoadStatsMap` on the stack, forms the file name (`path` + the
/// string at `01039788`), deletes an existing file (`_access`, `DeleteFileA`),
/// creates a `BSFile` of 0x158 bytes for it (mode 1, buffer 0x4000), stores
/// it in the map (`00984f60`), collects the data (`00853b90`), writes the
/// report (`BGSSaveLoadStatsMap::OutputStats`), closes the file
/// (`008539e0`), opens the report with `ShellExecuteA` (verb at `01080190`,
/// `SW_SHOW` 5) and destroys the map.
pub fn fn_00853a30(e: &mut Engine, path: Ptr) {
    e.with_stack(0x1c + 0x104, |e, frame| {
        let stats = frame;
        let file_name = frame.addr() + 0x1c;
        fn_00853810(e, stats);
        e.call(0x00406d30, &args![file_name, 0x104u32, path]);
        e.call(0x00406d50, &args![file_name, 0x104u32, 0x0103_9788u32]);
        if e.call(0x00ecbf05, &args![file_name, 0u32]).u32() != 0xffff_ffff {
            e.call(0x00fd_f0d4, &args![file_name]);
        }
        let object = e.call(0x00401000, &args![0x158u32]).u32();
        let file = if object != 0 {
            e.call(0x00b00260, &args![object, file_name, 1u32, 0x4000u32, 0u32])
                .u32()
        } else {
            0
        };
        e.call(0x00984f60, &args![stats, file]);
        fn_00853b90(e, stats);
        bgs_save_load_stats_map_output_stats(e, stats);
        fn_008539e0(e, stats);
        e.call(
            0x00fd_f278,
            &args![0u32, 0x0108_0190u32, file_name, 0u32, 0u32, 5u32],
        );
        fn_008538b0(e, stats);
    });
}

/// One pass of the global-data loops of `00853b90`: resets the buffer
/// (`00847db0`), and when `SaveGlobalData` (`0084bcd0`, cdecl) fills it for
/// `index`, records it in the misc list (`00854c10`).
fn collect_global_data(e: &mut Engine, this: Ptr, buffer: u32, index: u32) {
    e.call(0x00847db0, &args![buffer]);
    if e.call(0x0084bcd0, &args![index, buffer]).bool() {
        let debug_size = e.call(0x0044edb0, &args![buffer]).u32();
        let data_size = fn_00853dc0(e, Ptr::new(buffer));
        let name = fn_00853de0(e, index);
        fn_00854c10(e, this, name, data_size, debug_size);
    }
}

// Translated from 00853b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00853b90 - the C++ exception-unwinding frame (FS:[0], unwind table f132a8) is not translated
/// Unnamed in the engine map: fills the stats map. Walks the form map of
/// the object `00559450` returns for the save manager (`[011ddf38]`): an
/// entry whose `item + 4` makes `00559450` answer non-zero (unloaded) is
/// recorded with `008549b0` from the unloaded-form helpers (`0084b450`,
/// `00866450`, `008663e0`, `00849220`); any other, when its form exists
/// (`004839c0`), is saved into a `BGSSaveFormBuffer` (`00854b10`) and
/// recorded with that buffer's sizes. Then the global data records 0 to 0xb
/// and 1000 are collected (`collect_global_data`).
pub fn fn_00853b90(e: &mut Engine, this: Ptr) {
    e.with_stack(SAVE_BUFFER_SIZE + 12, |e, frame| {
        let buffer = frame.addr();
        let iter = buffer + SAVE_BUFFER_SIZE;
        let key = iter + 4;
        let item = iter + 8;
        e.call(0x008659c0, &args![buffer]);
        let manager = e.mem.u32(SAVE_MANAGER);
        let map = e.call(0x00559450, &args![manager]).u32();
        let first = e.call(0x004b9ba0, &args![map]).u32();
        e.mem.set_u32(iter, first);
        while e.mem.u32(iter) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(item, 0);
            e.call(0x006b7f20, &args![map, iter, key, item]);
            let form_id = e.mem.u32(key);
            let entry = e.mem.u32(item);
            let flags_word = e.mem.u32(entry);
            if e.call(0x00559450, &args![entry + 4]).u32() != 0 {
                let version = e.call(0x0084b450, &args![entry + 4]).u8();
                let overhead = e.call(0x00866450, &args![entry + 4]).u32();
                let data = e.call(0x008663e0, &args![entry + 4]).u32();
                let map_key = e.call(0x00849220, &args![entry + 4]).u32();
                fn_008549b0(
                    e, this, form_id, 0, map_key, flags_word, data, overhead, 0, version,
                );
            } else {
                let form = e.call(0x004839c0, &args![form_id]).u32();
                if form != 0 {
                    fn_00854b10(e, this, Ptr::new(form), flags_word, Ptr::new(buffer));
                    let version = e.call(0x008a81c0, &args![buffer]).u8();
                    let debug = e.call(0x0044edb0, &args![buffer]).u32();
                    let overhead = e.call(0x00865b10, &args![buffer]).u32();
                    let data = fn_00853dc0(e, Ptr::new(buffer));
                    let map_key = e.call(0x00401170, &args![form]).u32();
                    fn_008549b0(
                        e, this, form_id, form, map_key, flags_word, data, overhead, debug, version,
                    );
                }
            }
        }
        for index in 0..0xc {
            collect_global_data(e, this, buffer, index);
        }
        for index in 1000..0x3e9 {
            collect_global_data(e, this, buffer, index);
        }
        e.call(0x00847dd0, &args![buffer]);
    });
}

// Translated from 00853dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `*(this + 0xc) - *(this + 0x10)`, the
/// difference of two words of a save form buffer.
pub fn fn_00853dc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(0xc))
        .wrapping_sub(e.mem.u32(this.addr().wrapping_add(0x10)))
}

// Translated from 00853de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): the name of global data record
/// `index`: the pointer in the table at `011a216c` below 1000, in the table
/// at `011a11fc` from 1000.
pub fn fn_00853de0(e: &mut Engine, index: u32) -> u32 {
    let table = if index >= 1000 {
        0x011a_11fcu32
    } else {
        0x011a_216c
    };
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

// Frame of `OutputStats`: the exe's locals, in one block.
const OUT_SAVE_BUFFER: u32 = 0;
const OUT_TOTAL_STATS: u32 = OUT_SAVE_BUFFER + SAVE_BUFFER_SIZE;
const OUT_TYPE_STATS: u32 = OUT_TOTAL_STATS + 0x18;
const OUT_MISC_STATS: u32 = OUT_TYPE_STATS + 0x18;
const OUT_ITER: u32 = OUT_MISC_STATS + 0x18;
const OUT_KEY: u32 = OUT_ITER + 4;
const OUT_LIST: u32 = OUT_KEY + 4;
const OUT_MASK_SLOT: u32 = OUT_LIST + 4;
const OUT_LINE: u32 = OUT_MASK_SLOT + 4;
const OUT_NAME: u32 = OUT_LINE + 0x400;
const OUT_FLAGS_TEXT: u32 = OUT_NAME + 0x400;
const OUT_PIECE: u32 = OUT_FLAGS_TEXT + 0x400;
const OUT_PADDED: u32 = OUT_PIECE + 0x400;
const OUT_FRAME: u32 = OUT_PADDED + 0x400;

/// The name of form type `key` in the table at `01187004`.
fn form_type_name(e: &Engine, key: u32) -> u32 {
    e.mem
        .u32(FORM_TYPE_NAMES.wrapping_add(key.wrapping_mul(12)))
}

/// `sprintf_s(line, 1000, format, ...)` (`00406d00`, cdecl).
fn format_line(e: &mut Engine, dest: u32, format: u32, rest: &[u32]) {
    let mut words = vec![dest, LINE_SIZE, format];
    words.extend_from_slice(rest);
    e.call(0x00406d00, &words);
}

/// The six words of the statistics block at `stats` (count, three sums,
/// minimum, maximum) the report lines print.
fn stats_summary_words(e: &Engine, stats: Ptr) -> [u32; 6] {
    let s = stats.addr();
    [
        e.mem.u32(s),
        e.mem.u32(s + 4),
        e.mem.u32(s + 8),
        e.mem.u32(s + 0xc),
        e.mem.u32(s + 0x10),
        e.mem.u32(s + 0x14),
    ]
}

// Translated from 00853e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00853e10 - the C++ exception-unwinding frame (FS:[0], unwind table f132d8) and the /GS cookie check 00ec408c are not translated (the stack probe 00ec8660 is left out)
/// `BGSSaveLoadStatsMap::OutputStats` (Xbox PDB): writes the report to the
/// file at `this + 0x18` (`00854b80`). A header line, then per form type of
/// the map: the type's name line and one line per recorded form (hex form
/// id, size `008547f0`, `00559450` of the changes word, version byte, the
/// name `SavedFormData::GetFormName` gives and the base size and per-flag
/// sizes `00854b10` measures on a `BGSSaveFormBuffer`) followed by the
/// type's totals; then the totals of all forms; the misc list at `+0x10`
/// with its totals; and the save game history (lines of the
/// `00441110` object's `0084e320`, indented by the `***` / `---` markers).
/// An entry of the list that is null still has its sizes read (offsets 8 to
/// 0x10 of address 0), as the exe does.
pub fn bgs_save_load_stats_map_output_stats(e: &mut Engine, this: Ptr) {
    e.with_stack(OUT_FRAME, |e, frame| {
        output_stats_in(e, this, frame.addr());
    });
}

fn output_stats_in(e: &mut Engine, this: Ptr, frame: u32) {
    let t = this.addr();
    let buffer = frame + OUT_SAVE_BUFFER;
    let total = Ptr::new(frame + OUT_TOTAL_STATS);
    let type_stats = Ptr::new(frame + OUT_TYPE_STATS);
    let misc = Ptr::new(frame + OUT_MISC_STATS);
    let (iter, key, list) = (frame + OUT_ITER, frame + OUT_KEY, frame + OUT_LIST);
    let slot = frame + OUT_MASK_SLOT;
    let line = frame + OUT_LINE;
    let name = frame + OUT_NAME;
    let flags_text = frame + OUT_FLAGS_TEXT;
    let piece = frame + OUT_PIECE;
    let padded = frame + OUT_PADDED;

    e.call(0x008659c0, &args![buffer]);
    fn_00854830(e, total);
    format_line(
        e,
        line,
        0x0108_03f0,
        &[
            0x0108_0414,
            0x0108_041c,
            0x0104_4aec,
            0x0108_0424,
            0x0105_ac4c,
            0x0108_0428,
        ],
    );
    fn_00854b80(e, this, Ptr::new(line));
    let first = e.call(0x004b9ba0, &args![this]).u32();
    e.mem.set_u32(iter, first);
    while e.mem.u32(iter) != 0 {
        fn_00854830(e, type_stats);
        e.mem.set_u32(key, 0);
        e.mem.set_u32(list, 0);
        e.call(0x006b7f20, &args![this, iter, key, list]);
        let form_type = e.mem.u32(key);
        let type_name = form_type_name(e, form_type);
        format_line(e, line, 0x0108_03e4, &[type_name]);
        fn_00854b80(e, this, Ptr::new(line));
        let mut node = e.mem.u32(list);
        while node != 0 {
            let item_slot = e.call(0x006815c0, &args![node]).u32();
            let entry = e.mem.u32(item_slot);
            if entry != 0 {
                let form = e.mem.u32(entry + 0x14);
                // The combined text starts as the byte at 01011584 followed
                // by 999 zero bytes.
                let first_byte = e.mem.u8(EMPTY_STRING);
                e.mem.set_u8(flags_text, first_byte);
                e.call(0x00ec61c0, &args![flags_text + 1, 0u32, 999u32]);
                let mut base_size = 0u32;
                if form != 0 {
                    e.call(0x008c71b0, &args![slot, 0u32]);
                    let mask = e.mem.u32(slot);
                    fn_00854b10(e, this, Ptr::new(form), mask, Ptr::new(buffer));
                    base_size = e.call(0x0084e3a0, &args![buffer]).u32();
                    if base_size != 0 {
                        e.call(0x00406d30, &args![name, LINE_SIZE, 0x0108_03dcu32]);
                        if e.vcall(form, 0xf0, &args![]).bool()
                            && e.vcall(form, 0xfc, &args![]).bool()
                        {
                            let mut kind = 0xffff_ffffu32;
                            if e.call(0x008d8520, &args![form]).u32() != 0 {
                                let process = e.call(0x008d8520, &args![form]).u32();
                                kind = e.call(0x0045cd60, &args![process]).u32();
                            }
                            // Jump table at 008547d0, indexed by kind + 1.
                            let text = match kind.wrapping_add(1) {
                                0 => Some(0x0102_909cu32),
                                1 => Some(0x0105_a010),
                                2 => Some(0x0108_03c8),
                                3 => Some(0x0108_03d4),
                                4 => Some(0x0105_a004),
                                _ => None,
                            };
                            if let Some(text) = text {
                                e.call(0x00406d30, &args![name, LINE_SIZE, text]);
                            }
                        }
                        format_line(e, piece, 0x0108_03c0, &[name, base_size]);
                        format_line(e, padded, 0x0108_03b8, &[piece]);
                        e.call(0x00406d50, &args![flags_text, LINE_SIZE, padded]);
                    }
                }
                for bit_index in 0..0x20u32 {
                    let bit = 1u32 << bit_index;
                    if e.call(0x004280f0, &args![entry + 4, bit]).bool() {
                        let flag_name = e.call(0x0083fef0, &args![bit, form_type, 1u32]).u32();
                        e.call(0x00406d30, &args![name, LINE_SIZE, flag_name]);
                        if e.mem.u32(entry + 0x14) != 0 {
                            e.call(0x008c71b0, &args![slot, bit]);
                            let mask = e.mem.u32(slot);
                            let form = e.mem.u32(entry + 0x14);
                            fn_00854b10(e, this, Ptr::new(form), mask, Ptr::new(buffer));
                            let size = e
                                .call(0x0084e3a0, &args![buffer])
                                .u32()
                                .wrapping_sub(base_size);
                            format_line(e, piece, 0x0108_03c0, &[name, size]);
                        } else {
                            format_line(e, piece, 0x0101_9f08, &[name]);
                        }
                        format_line(e, padded, 0x0108_03b8, &[piece]);
                        e.call(0x00406d50, &args![flags_text, LINE_SIZE, padded]);
                    }
                }
                e.call(0x00406d30, &args![name, LINE_SIZE, EMPTY_STRING]);
                saved_form_data_get_form_name(e, Ptr::new(entry), Ptr::new(name), LINE_SIZE);
                let flags = e.call(0x00559450, &args![entry + 4]).u32();
                let size = fn_008547f0(e, Ptr::new(entry));
                let id = e.mem.u32(entry);
                let version = e.mem.u8(entry + 0x18) as u32;
                format_line(
                    e,
                    line,
                    0x0108_0398,
                    &[id, size, flags, version, name, flags_text],
                );
                fn_00854b80(e, this, Ptr::new(line));
            }
            // Outside the null test: a null entry reads address 8 here, as
            // the exe does.
            let (a, b, c) = (
                e.mem.u32(entry + 8),
                e.mem.u32(entry + 0xc),
                e.mem.u32(entry + 0x10),
            );
            fn_00854880(e, type_stats, a, b, c);
            fn_00854880(e, total, a, b, c);
            node = e.call(0x00726070, &args![node]).u32();
        }
        let average = fn_00854920(e, type_stats);
        let words = stats_summary_words(e, type_stats);
        let total_size = fn_00854900(e, type_stats);
        let mut message = vec![
            line,
            LINE_SIZE,
            0x0108_0310,
            type_name,
            words[0],
            type_name,
            total_size,
            words[1],
            words[2],
            words[3],
            type_name,
            words[4],
            type_name,
            words[5],
            type_name,
        ];
        average_words(&mut message, average);
        e.call(0x00406d00, &message);
        fn_00854b80(e, this, Ptr::new(line));
    }

    summary_line(e, this, total, line, 0x0108_0278);
    fn_00854830(e, misc);
    fn_00854b80(e, this, Ptr::new(0x0108_0264));
    let mut node = t.wrapping_add(0x10);
    while node != 0 {
        let item_slot = e.call(0x006815c0, &args![node]).u32();
        let entry = e.mem.u32(item_slot);
        if entry != 0 {
            let size = fn_00854810(e, Ptr::new(entry));
            format_line(e, line, 0x0108_0254, &[size, entry]);
            fn_00854b80(e, this, Ptr::new(line));
            let (a, c) = (e.mem.u32(entry + 0x104), e.mem.u32(entry + 0x108));
            fn_00854880(e, misc, a, 0, c);
        }
        node = e.call(0x00726070, &args![node]).u32();
    }
    summary_line(e, this, misc, line, 0x0108_01c0);

    let manager = e.mem.u32(SAVE_MANAGER);
    let history = e.call(0x00441110, &args![manager]).u32();
    let count = e.call(0x00620b80, &args![history]).u32();
    let mut depth = 0i32;
    if count != 0 {
        fn_00854b80(e, this, Ptr::new(0x0108_01a0));
        for index in 0..count {
            let text = e.call(0x0084e320, &args![history, index]).u32();
            let mut kind = 0i32;
            if e.call(0x004564f0, &args![text, 0x0108_019cu32, 3u32]).u32() == 0 {
                kind = 1;
            } else if e.call(0x004564f0, &args![text, 0x0107_7dc0u32, 3u32]).u32() == 0 {
                kind = -1;
                depth = depth.wrapping_sub(1);
            }
            let mut indent = 0i32;
            while indent < depth {
                fn_00854b80(e, this, Ptr::new(0x0108_0198));
                indent += 1;
            }
            let shown = if kind != 0 {
                text.wrapping_add(3)
            } else {
                text
            };
            fn_00854b80(e, this, Ptr::new(shown));
            fn_00854b80(e, this, Ptr::new(0x0101_f520));
            if kind == 1 {
                depth = depth.wrapping_add(1);
            }
        }
    }
    e.call(0x00847dd0, &args![buffer]);
}

/// Pushes the average as the `double` the exe's `FSTP qword [ESP]` leaves
/// on the stack: the `float` result widened, low word first.
fn average_words(words: &mut Vec<u32>, average: f32) {
    let bits = (average as f64).to_bits();
    words.push(bits as u32);
    words.push((bits >> 32) as u32);
}

/// The "Total ... Count ..." line of a stats block with no form-type name:
/// count, total, data, overhead, debug, minimum, maximum, average.
fn summary_line(e: &mut Engine, this: Ptr, stats: Ptr, line: u32, format: u32) {
    let average = fn_00854920(e, stats);
    let words = stats_summary_words(e, stats);
    let total = fn_00854900(e, stats);
    let mut message = vec![
        line, LINE_SIZE, format, words[0], total, words[1], words[2], words[3], words[4], words[5],
    ];
    average_words(&mut message, average);
    e.call(0x00406d00, &message);
    fn_00854b80(e, this, Ptr::new(line));
}

// Translated from 008547f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sum of the three words at `+8`, `+0xc`
/// and `+0x10` of a `SavedFormData` (the Data, Overhead and Debug columns of
/// the report).
pub fn fn_008547f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(8))
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0x10)))
}

// Translated from 00854810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sum of the words at `+0x104` and `+0x108`
/// of a misc list entry (`00854c10` fills them).
pub fn fn_00854810(e: &mut Engine, this: Ptr) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(0x104))
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0x108)))
}

// Translated from 00854830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initializes a statistics block of six words
/// (count, data, overhead, debug, minimum, maximum): all zero but the
/// minimum, 0xffffffff.
pub fn fn_00854830(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    for offset in [0, 4, 8, 0xc] {
        e.mem.set_u32(t.wrapping_add(offset), 0);
    }
    e.mem.set_u32(t.wrapping_add(0x10), 0xffff_ffff);
    e.mem.set_u32(t.wrapping_add(0x14), 0);
    this
}

// Translated from 00854880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds one record to a statistics block: count
/// plus 1, the three sizes added to the three sums, and the total of the three
/// sizes taken into the minimum (`+0x10`, unsigned) and the maximum (`+0x14`).
pub fn fn_00854880(e: &mut Engine, this: Ptr, data: u32, overhead: u32, debug: u32) {
    let t = this.addr();
    for (offset, add) in [(4u32, data), (8, overhead), (0xc, debug)] {
        let sum = e.mem.u32(t.wrapping_add(offset)).wrapping_add(add);
        e.mem.set_u32(t.wrapping_add(offset), sum);
    }
    let count = e.mem.u32(t).wrapping_add(1);
    e.mem.set_u32(t, count);
    let all = data.wrapping_add(overhead).wrapping_add(debug);
    if all < e.mem.u32(t.wrapping_add(0x10)) {
        e.mem.set_u32(t.wrapping_add(0x10), all);
    }
    if all > e.mem.u32(t.wrapping_add(0x14)) {
        e.mem.set_u32(t.wrapping_add(0x14), all);
    }
}

// Translated from 00854900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sum of the three size sums of a
/// statistics block (`+4`, `+8`, `+0xc`).
pub fn fn_00854900(e: &mut Engine, this: Ptr) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(4))
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(8)))
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
}

// Translated from 00854920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the average, `total / count` of a statistics
/// block as `float` (x87 `FILD`/`FDIVP`, stored with `FSTP float`). A zero
/// count gives infinity, or the x87 "real indefinite" NaN for 0 / 0.
pub fn fn_00854920(e: &mut Engine, this: Ptr) -> f32 {
    let total = fn_00854900(e, this) as f64;
    let count = e.mem.u32(this.addr()) as f64;
    if count == 0.0 {
        if total == 0.0 {
            return f32::from_bits(0xffc0_0000);
        }
        return f32::INFINITY;
    }
    (total / count) as f32
}

// Translated from 00854960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl comparator of two `SavedFormData`):
/// orders by `008547f0` descending: -1 when `first`'s is larger, 1 when it
/// is smaller, else 0.
pub fn fn_00854960(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = fn_008547f0(e, first);
    let b = fn_008547f0(e, second);
    if a > b {
        -1
    } else {
        i32::from(a < b)
    }
}

// Translated from 008549b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008549b0 - the C++ exception-unwinding frame (FS:[0], unwind table f177b6) is not translated
/// Unnamed in the engine map (`__thiscall`, 8 stack words): records one
/// form in the stats map. Allocates a 0x19-byte `SavedFormData`
/// (`00854af0`) and fills it: `+0` the form id, `+4` `word_04`, `+8`/`+0xc`/
/// `+0x10` the sizes the report labels Data, Overhead and Debug, `+0x14` the
/// form, `+0x18` the version byte. Finds the list for `map_key` in the map
/// (`00853130`) or creates an empty one (`SetAt`, `00844700`) and inserts
/// the new record by `00854960` (`007a7eb0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_008549b0(
    e: &mut Engine,
    this: Ptr,
    form_id: u32,
    form: u32,
    map_key: u32,
    word_04: u32,
    data_size: u32,
    overhead_size: u32,
    debug_size: u32,
    version: u8,
) {
    let object = e.call(0x00401000, &args![0x19u32]).u32();
    let record = if object != 0 {
        fn_00854af0(e, Ptr::new(object)).addr()
    } else {
        0
    };
    e.mem.set_u32(record, form_id);
    e.mem.set_u32(record.wrapping_add(0x14), form);
    e.mem.set_u32(record.wrapping_add(4), word_04);
    e.mem.set_u32(record.wrapping_add(8), data_size);
    e.mem.set_u32(record.wrapping_add(0xc), overhead_size);
    e.mem.set_u32(record.wrapping_add(0x10), debug_size);
    e.mem.set_u8(record.wrapping_add(0x18), version);
    let list = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if !e.call(0x00853130, &args![this, map_key, slot]).bool() {
            let object = e.call(0x00401000, &args![8u32]).u32();
            let created = if object != 0 {
                e.call(0x0096a2d0, &args![object]).u32()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), created);
            e.call(0x00844700, &args![this, map_key, created]);
        }
        e.mem.u32(slot.addr())
    });
    e.call(0x007a7eb0, &args![list, record, 0x0085_4960u32]);
}

// Translated from 00854af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of a `SavedFormData`: clears
/// the word at `+4` (`0044dee0`) and returns `this`.
pub fn fn_00854af0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044dee0, &args![this.addr().wrapping_add(4)]);
    this
}

// Translated from 00854b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, 3 stack words, `this` unused):
/// saves `form` into the save form buffer `buffer`. Resets the buffer
/// (`00847db0`), fills its header (`00865ad0`: the form id word `0084e3a0`,
/// `mask`, the form type byte `00401170`, the value `00851110` gives for
/// `[011de134]`), stores the form at `+0x20` (`0050f9c0`), calls `0084eb80`
/// and then the form's virtual `+0x54` with the buffer.
pub fn fn_00854b10(e: &mut Engine, _this: Ptr, form: Ptr, mask: u32, buffer: Ptr) {
    e.call(0x00847db0, &args![buffer]);
    let current = e.mem.u32(0x011d_e134);
    let version = e.call(0x00851110, &args![current]).u32();
    let form_type = e.call(0x00401170, &args![form]).u32();
    let form_id = e.call(0x0084e3a0, &args![form]).u32();
    e.call(
        0x00865ad0,
        &args![buffer, form_id, mask, form_type, version],
    );
    e.call(0x0050f9c0, &args![buffer, form]);
    e.call(0x0084eb80, &args![buffer, form, mask]);
    e.vcall(form.addr(), 0x54, &args![buffer]);
}

// Translated from 00854b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, one stack word): when the file at
/// `this + 0x18` exists, writes `text` to it: `00473180` with the text and
/// its length (`0044a670`).
pub fn fn_00854b80(e: &mut Engine, this: Ptr, text: Ptr) {
    let file = e.mem.u32(this.addr().wrapping_add(0x18));
    if file != 0 {
        let length = e.call(0x0044a670, &args![text]).u32();
        e.call(0x00473180, &args![file, text, length]);
    }
}

// Translated from 00854bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl comparator of two misc list entries):
/// orders by `00854810` descending: -1 when `first`'s is larger, 1 when it is
/// smaller, else 0.
pub fn fn_00854bc0(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = fn_00854810(e, first);
    let b = fn_00854810(e, second);
    if a > b {
        -1
    } else {
        i32::from(a < b)
    }
}

// Translated from 00854c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__thiscall`, 3 stack words): adds an entry to
/// the misc list at `this + 0x10`: a 0x10c-byte record with `name` copied
/// into its first 0x104 bytes (`00406d30`), `data_size` at `+0x104` and
/// `debug_size` at `+0x108`, inserted by `00854bc0` (`007a7eb0`).
pub fn fn_00854c10(e: &mut Engine, this: Ptr, name: u32, data_size: u32, debug_size: u32) {
    let record = e.call(0x00401000, &args![0x10cu32]).u32();
    e.call(0x00406d30, &args![record, 0x104u32, name]);
    e.mem.set_u32(record.wrapping_add(0x104), data_size);
    e.mem.set_u32(record.wrapping_add(0x108), debug_size);
    e.call(
        0x007a7eb0,
        &args![this.addr().wrapping_add(0x10), record, 0x0085_4bc0u32],
    );
}

// Translated from 00854c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the `NiTMap` of form ids
/// to lists of `SavedFormData` (vtable `01080434`): constructs the base with
/// `count` buckets (`00854ce0`) and stores the vtable.
pub fn fn_00854c80(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    fn_00854ce0(e, this, count);
    e.mem.set_u32(this.addr(), FORM_MAP_VTABLE);
    this
}

// Translated from 00854cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<ENUM_FORM_ID,BSSimpleList<SavedFormData*>*>::_scalar_deleting_destructor_`
/// (Xbox PDB): destroys (`00854d50`) and, when bit 0 of `flags` is set,
/// frees the object.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00854d50(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translated from 00854ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the `NiTMapBase` of that
/// map: vtable `01080454`, `count` at `+4`, zero at `+0xc`, a bucket array
/// of `count << 2` bytes (`00aa1070`) at `+8`, cleared with `memset`
/// (`00403d30`).
pub fn fn_00854ce0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, FORM_MAP_BASE_VTABLE);
    e.mem.set_u32(t.wrapping_add(4), count);
    e.mem.set_u32(t.wrapping_add(0xc), 0);
    let buckets = e.call(0x00aa1070, &args![count << 2]).u32();
    e.mem.set_u32(t.wrapping_add(8), buckets);
    e.call(0x00403d30, &args![buckets, 0u32, count << 2]);
    this
}

// Translated from 00854d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00854d50 - the C++ exception-unwinding frame (FS:[0], unwind table f13308) is not translated
/// Unnamed in the engine map: the destructor of that `NiTMap`: vtable
/// `01080434`, `NiTMapBase` clear (`00438af0`), then the base destructor
/// (`00854db0`).
pub fn fn_00854d50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FORM_MAP_VTABLE);
    e.call(0x00438af0, &args![this]);
    fn_00854db0(e, this);
}

// Translated from 00854db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of that `NiTMapBase`: vtable
/// `01080454`, clear (`00438af0`), and frees the bucket array at `+8`
/// (`00aa10f0`).
pub fn fn_00854db0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FORM_MAP_BASE_VTABLE);
    e.call(0x00438af0, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00aa10f0, &args![buckets]);
}

// Translated from 00854de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<ENUM_FORM_ID,BSSimpleList<SavedFormData*>*>>,..>::_scalar_deleting_destructor_`
/// (Xbox PDB): destroys (`00854db0`) and, when bit 0 of `flags` is set,
/// frees the object.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00854db0(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00853500, get_numeric_id(Ptr) -> u32),
        entry!(0x00853570, set_numeric_id(Ptr, u32)),
        entry!(0x008535e0, fn_008535e0()),
        entry!(0x00853640, fn_00853640(Ptr) -> u32),
        entry!(0x00853670, bgs_save_load_form_info_set_form_type(Ptr, u32)),
        entry!(0x008536d0, saved_form_data_get_form_name(Ptr, Ptr, u32)),
        entry!(0x00853810, fn_00853810(Ptr) -> Ptr),
        entry!(0x00853880, bgs_save_load_stats_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008538b0, fn_008538b0(Ptr)),
        entry!(0x008539e0, fn_008539e0(Ptr)),
        entry!(0x00853a30, fn_00853a30(Ptr)),
        entry!(0x00853b90, fn_00853b90(Ptr)),
        entry!(0x00853dc0, fn_00853dc0(Ptr) -> u32),
        entry!(0x00853de0, fn_00853de0(u32) -> u32),
        entry!(0x00853e10, bgs_save_load_stats_map_output_stats(Ptr)),
        entry!(0x008547f0, fn_008547f0(Ptr) -> u32),
        entry!(0x00854810, fn_00854810(Ptr) -> u32),
        entry!(0x00854830, fn_00854830(Ptr) -> Ptr),
        entry!(0x00854880, fn_00854880(Ptr, u32, u32, u32)),
        entry!(0x00854900, fn_00854900(Ptr) -> u32),
        entry!(0x00854920, fn_00854920(Ptr) -> f32),
        entry!(0x00854960, fn_00854960(Ptr, Ptr) -> i32),
        entry!(
            0x008549b0,
            fn_008549b0(Ptr, u32, u32, u32, u32, u32, u32, u32, u8)
        ),
        entry!(0x00854af0, fn_00854af0(Ptr) -> Ptr),
        entry!(0x00854b10, fn_00854b10(Ptr, Ptr, u32, Ptr)),
        entry!(0x00854b80, fn_00854b80(Ptr, Ptr)),
        entry!(0x00854bc0, fn_00854bc0(Ptr, Ptr) -> i32),
        entry!(0x00854c10, fn_00854c10(Ptr, u32, u32, u32)),
        entry!(0x00854c80, fn_00854c80(Ptr, u32) -> Ptr),
        entry!(0x00854cb0, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00854ce0, fn_00854ce0(Ptr, u32) -> Ptr),
        entry!(0x00854d50, fn_00854d50(Ptr)),
        entry!(0x00854db0, fn_00854db0(Ptr)),
        entry!(0x00854de0, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_ids_decode_without_a_save_manager_and_preserve_memory() {
        // Generated native vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80: all three outputs/guards agree.
        let mut e = Engine::new();
        let index = e.mem.alloc(5);
        for (bytes, expected) in [
            ([0x80, 0, 0], 0xff000000),
            ([0x81, 0x23, 0x45], 0xff012345),
            ([0xff, 0xff, 0xff], 0xff7fffff),
        ] {
            let data = [0xaa, bytes[0], bytes[1], bytes[2], 0xbb];
            e.mem.write(index, &data);
            assert_eq!(e.call(0x00853500, &args![index + 1]).u32(), expected);
            assert_eq!(e.mem.bytes(index, 5), data);
        }
    }

    #[test]
    fn ordinary_indices_use_the_returned_map_and_preserve_full_result() {
        let mut e = Engine::new();
        e.map(0x011ddf38, 4);
        e.mem.set_u32(0x011ddf38, 0x1010);
        let index = e.mem.alloc(3);
        e.mem.write(index, &[0x12, 0x34, 0x56]);
        e.register_double(0x0044ddc0, |_, a| {
            assert_eq!(a, &[0x1010]);
            0x2020u32.into_ret()
        });
        e.register_double(0x00846d20, |_, a| {
            assert_eq!(a, &[0x2020, 0x123456]);
            0xabcdef01u32.into_ret()
        });
        assert_eq!(e.call(0x00853500, &args![index]).u32(), 0xabcdef01);
    }

    #[test]
    fn writing_marks_created_ids_after_mapping_and_touches_only_three_bytes() {
        let mut e = Engine::new();
        e.map(0x011ddf38, 4);
        e.mem.set_u32(0x011ddf38, 0x1010);
        let index = e.mem.alloc(5);
        e.register_double(0x0044ddc0, |_, a| {
            assert_eq!(a, &[0x1010]);
            0x2020u32.into_ret()
        });
        for (mapped, bytes) in [
            (0x00123456u32, [0x12, 0x34, 0x56]),
            (0xff012345, [0x81, 0x23, 0x45]),
            (0xfe012345, [0x01, 0x23, 0x45]),
            (0xffffffff, [0xff, 0xff, 0xff]),
        ] {
            e.mem.write(index, &[0xaa; 5]);
            e.register_double(0x00846c90, move |_, a| {
                assert_eq!(a, &[0x2020, 0x778899]);
                mapped.into_ret()
            });
            e.call(0x00853570, &args![index + 1, 0x778899u32]);
            assert_eq!(
                e.mem.bytes(index, 5),
                [0xaa, bytes[0], bytes[1], bytes[2], 0xaa]
            );
        }
    }
}

#[cfg(test)]
mod index_table_tests {
    use super::*;

    #[test]
    fn the_index_bytes_are_filled_through_the_offset_table() {
        let mut e = Engine::new();
        e.register(0x0040_3d30, |e, a| {
            assert_eq!((a[0], a[1], a[2]), (INDEX_BYTES, 0xff, 0x79));
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.map(INDEX_BYTES, 0x100);
        e.map(INDEX_COUNT, 4);
        e.map(INDEX_OFFSETS, 0x100);
        // Offsets 0, 2, 4, ... (the first 0x37 entries); the rest stay 0xff.
        for i in 0..0x37u32 {
            e.mem.set_u32(INDEX_OFFSETS + 4 * i, 2 * i);
        }
        e.set_global::<u32>(INDEX_COUNT, 5);
        e.call(0x008535e0, &args![]);
        assert_eq!(e.global::<u32>(INDEX_COUNT), 0x37, "the count is reset");
        assert_eq!(e.mem.u8(INDEX_BYTES), 0);
        assert_eq!(e.mem.u8(INDEX_BYTES + 2), 1);
        assert_eq!(e.mem.u8(INDEX_BYTES + 4), 2);
        assert_eq!(e.mem.u8(INDEX_BYTES + 0x6c), 0x36);
        assert_eq!(e.mem.u8(INDEX_BYTES + 1), 0xff, "unused bytes stay 0xff");
        assert_eq!(e.mem.u8(INDEX_BYTES + 0x78), 0xff);
    }
}

#[cfg(test)]
mod stats_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn calls() -> Calls {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A test double that records its arguments and answers `result`.
    fn spy(e: &mut Engine, log: &Calls, addr: u32, result: u32) {
        let log = log.clone();
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            result.into_ret()
        });
    }

    fn logged(log: &Calls) -> Vec<(u32, Vec<u32>)> {
        log.borrow().clone()
    }

    fn logged_addresses(log: &Calls) -> Vec<u32> {
        log.borrow().iter().map(|call| call.0).collect()
    }

    /// An engine with test doubles for the callees almost every function of
    /// this file uses: string copy, concatenation and length, allocation,
    /// free, and the list-node accessors.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0040_6d30, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            a[0].into_ret()
        });
        e.register(0x0040_6d50, |e, a| {
            let mut s = e.mem.cstr(a[0]);
            s.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &s);
            a[0].into_ret()
        });
        e.register(0x0044_a670, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(0x0040_1000, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s.as_bytes());
        p
    }

    fn text_at(e: &mut Engine, addr: u32, s: &str) {
        e.map(addr, s.len() as u32 + 1);
        e.mem.set_cstr(addr, s.as_bytes());
    }

    fn text_of(e: &Engine, addr: u32) -> String {
        String::from_utf8(e.mem.cstr(addr)).unwrap()
    }

    /// An object whose vtable has the given slots (byte offset, target).
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, vtable);
        o
    }

    #[test]
    fn the_form_type_getter_masks_the_top_two_bits() {
        let mut e = Engine::new();
        e.map(INDEX_OFFSETS, 0x100);
        e.mem.set_u32(INDEX_OFFSETS + 5 * 4, 0xabc);
        let byte = e.mem.alloc(1);
        e.mem.set_u8(byte, 0xc5);
        assert_eq!(e.call(0x00853640, &args![byte]).u32(), 0xabc);
    }

    #[test]
    fn set_form_type_keeps_the_top_two_bits_and_warns_on_unknown_types() {
        let mut e = Engine::new();
        let log = calls();
        e.map(INDEX_BYTES, 0x100);
        e.set_global::<u32>(INDEX_COUNT, 0x37);
        e.mem.set_u8(INDEX_BYTES + 3, 7);
        e.mem.set_u8(INDEX_BYTES + 9, 0xff);
        spy(&mut e, &log, 0x004612b0, 0x1234);
        spy(&mut e, &log, 0x00845cc0, 0);
        let this = e.mem.alloc(1);
        e.mem.set_u8(this, 0xd5);
        e.call(0x00853670, &args![this, 3u32]);
        assert_eq!(e.mem.u8(this), 0xc7);
        assert!(logged(&log).is_empty(), "a known type does not warn");
        e.call(0x00853670, &args![this, 9u32]);
        assert_eq!(e.mem.u8(this), 0xff);
        assert_eq!(
            logged(&log),
            [
                (0x004612b0, vec![9]),
                (0x00845cc0, vec![0x0108_0124, 9, 0x1234])
            ]
        );
    }

    #[test]
    fn the_form_name_falls_back_through_four_sources() {
        let mut e = engine();
        text_at(&mut e, 0x0108_0164, "UNLOADED");
        let object_name = Rc::new(Cell::new(0u32));
        let full_name = Rc::new(Cell::new(0u32));
        let virtual_name = Rc::new(Cell::new(0u32));
        let location_name = Rc::new(Cell::new(0u32));
        let is_object = Rc::new(Cell::new(1u32));
        let form_type = Rc::new(Cell::new(0x46u32));
        let c = is_object.clone();
        e.register_double(0x7100_00f0, move |_, _| c.get().into_ret());
        let c = virtual_name.clone();
        e.register_double(0x7100_0130, move |_, _| c.get().into_ret());
        let c = location_name.clone();
        e.register_double(0x7100_0138, move |_, _| c.get().into_ret());
        let c = object_name.clone();
        e.register_double(0x0055_d520, move |_, _| c.get().into_ret());
        let c = full_name.clone();
        e.register_double(0x0048_2720, move |_, _| c.get().into_ret());
        let c = form_type.clone();
        e.register_double(0x0040_1170, move |_, _| c.get().into_ret());
        let form = object(&mut e, 0x20, &[(0xf0, 0x7100_00f0), (0x130, 0x7100_0130)]);
        let location = object(&mut e, 0x20, &[(0x138, 0x7100_0138)]);
        e.register_double(0x0067_33e0, move |_, a| {
            assert_eq!(a[0], form);
            location.into_ret()
        });
        let this = e.mem.alloc(0x20);
        let dst = e.mem.alloc(0x40);
        let get = |e: &mut Engine| {
            e.call(0x008536d0, &args![this, dst, 0x40u32]);
            text_of(e, dst)
        };
        assert_eq!(get(&mut e), "UNLOADED");
        e.mem.set_u32(this + 0x14, form);
        let empty = text(&mut e, "");
        let loaded = text(&mut e, "Rifle");
        let full = text(&mut e, "Full Name");
        let fallback = text(&mut e, "Fallback");
        let place = text(&mut e, "Place");
        object_name.set(loaded);
        full_name.set(full);
        assert_eq!(get(&mut e), "Rifle", "object: 0055d520");
        is_object.set(0);
        assert_eq!(get(&mut e), "Full Name", "other forms: GetFullName");
        full_name.set(empty);
        virtual_name.set(fallback);
        assert_eq!(get(&mut e), "Fallback", "empty: virtual +0x130");
        virtual_name.set(empty);
        location_name.set(place);
        assert_eq!(get(&mut e), "Place", "type 0x46: the location's name");
        form_type.set(0x45);
        assert_eq!(get(&mut e), "", "any other type stays empty");
    }

    #[test]
    fn the_stats_map_constructor_builds_the_base_and_the_list() {
        let mut e = engine();
        let log = calls();
        e.register_double(0x00aa_1070, |e, a| e.mem.alloc(a[0]).into_ret());
        spy(&mut e, &log, 0x0040_3d30, 0);
        spy(&mut e, &log, 0x0096_a2d0, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x99);
        assert_eq!(e.call(0x00853810, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), STATS_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        let buckets = e.mem.u32(this + 8);
        assert_eq!(
            logged(&log),
            [
                (0x0040_3d30, vec![buckets, 0, 0x94]),
                (0x0096_a2d0, vec![this + 0x10])
            ]
        );
    }

    /// Doubles for the destruction chain of the stats map; every delete is
    /// logged too.
    fn destruction_engine() -> (Engine, Calls) {
        let mut e = engine();
        let log = calls();
        for addr in [0x0046_ffb0, 0x0043_8af0, 0x00aa_10f0, 0x0040_1030] {
            spy(&mut e, &log, addr, 0);
        }
        (e, log)
    }

    #[test]
    fn the_stats_map_destructor_frees_every_list_and_the_file() {
        let (mut e, log) = destruction_engine();
        let this = e.mem.alloc(0x20);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, buckets);
        let file_deleter = 0x7100_0000;
        let file = object(&mut e, 0x10, &[(0, file_deleter)]);
        e.mem.set_u32(this + 0x18, file);
        spy(&mut e, &log, file_deleter, 0);
        // The map holds one list of two nodes; the misc list is the node
        // embedded at +0x10.
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x222);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x111);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(this + 0x10, 0x333);
        e.register(0x004b_9ba0, |_, _| 1u32.into_ret());
        e.register_double(0x006b_7f20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], 7);
            e.mem.set_u32(a[3], first);
            Ret::default()
        });
        e.call(0x008538b0, &args![this]);
        assert_eq!(
            logged(&log),
            [
                (file_deleter, vec![file, 1]),
                (0x0040_1030, vec![0x111]),
                (0x0040_1030, vec![0x222]),
                (0x0040_1030, vec![0x333]),
                (0x0046_ffb0, vec![this + 0x10]),
                (0x0043_8af0, vec![this]),
                (0x0043_8af0, vec![this]),
                (0x00aa_10f0, vec![buckets]),
            ]
        );
        assert_eq!(e.mem.u32(this), FORM_MAP_BASE_VTABLE);
    }

    #[test]
    fn the_scalar_deleting_destructors_free_only_when_asked() {
        let (mut e, log) = destruction_engine();
        e.register(0x004b_9ba0, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x40);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, buckets);
        assert_eq!(e.call(0x00853880, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FORM_MAP_BASE_VTABLE);
        assert!(!logged(&log).contains(&(0x0040_1030, vec![this])));
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00853880, &args![this, 1u32]).u32(), this);
        assert_eq!(logged(&log).last(), Some(&(0x0040_1030, vec![this])));
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00854cb0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            logged(&log),
            [
                (0x0043_8af0, vec![this]),
                (0x0043_8af0, vec![this]),
                (0x00aa_10f0, vec![buckets]),
                (0x0040_1030, vec![this]),
            ]
        );
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00854de0, &args![this, 0u32]).u32(), this);
        assert_eq!(
            logged(&log),
            [(0x0043_8af0, vec![this]), (0x00aa_10f0, vec![buckets])]
        );
    }

    #[test]
    fn closing_the_file_destroys_it_and_clears_the_pointer() {
        let mut e = engine();
        let log = calls();
        let deleter = 0x7100_0000;
        spy(&mut e, &log, deleter, 0);
        let file = object(&mut e, 0x10, &[(0, deleter)]);
        let this = e.mem.alloc(0x20);
        e.call(0x008539e0, &args![this]);
        assert!(logged(&log).is_empty(), "no file, nothing to destroy");
        e.mem.set_u32(this + 0x18, file);
        e.call(0x008539e0, &args![this]);
        assert_eq!(logged(&log), [(deleter, vec![file, 1])]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }

    #[test]
    fn the_report_file_is_replaced_written_and_opened() {
        for existing in [false, true] {
            let (mut e, log) = destruction_engine();
            let deleter = 0x7100_0000;
            let file = object(&mut e, 0x158, &[(0, deleter)]);
            spy(&mut e, &log, deleter, 0);
            text_at(&mut e, 0x0103_9788, ".txt");
            text_at(&mut e, 0x0108_0264, "MISC\n");
            e.map(SAVE_MANAGER, 4);
            for addr in [
                0x0096_a2d0,
                0x0040_3d30,
                0x0086_59c0,
                0x0084_7db0,
                0x0084_7dd0,
            ] {
                e.register(addr, |_, a| a[0].into_ret());
            }
            e.register(0x00aa_1070, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(0x0040_6d00, |_, _| Ret::default());
            for addr in [
                0x0055_9450,
                0x004b_9ba0,
                0x0084_bcd0,
                0x0044_1110,
                0x0062_0b80,
            ] {
                e.register(addr, |_, _| 0u32.into_ret());
            }
            e.register_double(0x0040_1000, move |_, a| {
                assert_eq!(a, &[0x158]);
                file.into_ret()
            });
            spy(&mut e, &log, 0x0047_3180, 0);
            spy(&mut e, &log, 0x00fd_f0d4, 0);
            let names = Rc::new(RefCell::new(Vec::new()));
            let seen = names.clone();
            e.register_double(0x00ec_bf05, move |e, a| {
                seen.borrow_mut().push(text_of(e, a[0]));
                (if existing { 0u32 } else { 0xffff_ffff }).into_ret()
            });
            e.register_double(0x0098_4f60, move |e, a| {
                e.mem.set_u32(a[0] + 0x18, a[1]);
                Ret::default()
            });
            e.register_double(0x00b0_0260, move |e, a| {
                assert_eq!(text_of(e, a[1]), "Data\\stats.txt");
                assert_eq!(&a[2..], &[1, 0x4000, 0]);
                a[0].into_ret()
            });
            let opened = Rc::new(RefCell::new(Vec::new()));
            let seen = opened.clone();
            e.register_double(0x00fd_f278, move |e, a| {
                seen.borrow_mut().push((text_of(e, a[2]), a.to_vec()));
                Ret::default()
            });
            let path = text(&mut e, "Data\\stats");
            e.call(0x00853a30, &args![path]);
            assert_eq!(*names.borrow(), ["Data\\stats.txt"]);
            let deleted = logged_addresses(&log).contains(&0x00fd_f0d4);
            assert_eq!(deleted, existing, "an existing report is deleted first");
            let closed = logged(&log)
                .into_iter()
                .filter(|c| c.0 == deleter)
                .collect::<Vec<_>>();
            assert_eq!(closed, [(deleter, vec![file, 1])]);
            let opened = opened.borrow();
            assert_eq!(opened.len(), 1);
            assert_eq!(opened[0].0, "Data\\stats.txt");
            assert_eq!(opened[0].1[0..2], [0, 0x0108_0190]);
            assert_eq!(opened[0].1[3..], [0, 0, 5]);
        }
    }

    #[test]
    fn small_accessors_read_their_words() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x120);
        e.mem.set_u32(o + 0xc, 100);
        e.mem.set_u32(o + 0x10, 30);
        assert_eq!(e.call(0x00853dc0, &args![o]).u32(), 70);
        e.mem.set_u32(o + 0x10, 130);
        assert_eq!(
            e.call(0x00853dc0, &args![o]).u32(),
            100u32.wrapping_sub(130)
        );
        // 008547f0: +8 + 0xc + 0x10.
        e.mem.set_u32(o + 8, 1);
        assert_eq!(e.call(0x008547f0, &args![o]).u32(), 1 + 100 + 130);
        // 00854810: +0x104 + 0x108.
        e.mem.set_u32(o + 0x104, 0xffff_fffe);
        e.mem.set_u32(o + 0x108, 5);
        assert_eq!(e.call(0x00854810, &args![o]).u32(), 3);
        // 00854900: +4 + 8 + 0xc.
        e.mem.set_u32(o + 4, 7);
        e.mem.set_u32(o + 8, 8);
        e.mem.set_u32(o + 0xc, 9);
        assert_eq!(e.call(0x00854900, &args![o]).u32(), 24);
    }

    #[test]
    fn global_data_names_come_from_two_ranges_of_one_table() {
        let mut e = Engine::new();
        e.map(0x011a_216c, 0x1000);
        e.map(0x011a_11fc, 0x1100);
        e.mem.set_u32(0x011a_216c + 2 * 4, 0xaaaa);
        e.mem.set_u32(0x011a_216c + 999 * 4, 0xcccc);
        e.mem.set_u32(0x011a_11fc + 1000 * 4, 0xbbbb);
        e.mem.set_u32(0x011a_11fc + 1001 * 4, 0xdddd);
        assert_eq!(e.call(0x00853de0, &args![2u32]).u32(), 0xaaaa);
        assert_eq!(e.call(0x00853de0, &args![999u32]).u32(), 0xcccc);
        assert_eq!(e.call(0x00853de0, &args![1000u32]).u32(), 0xbbbb);
        assert_eq!(e.call(0x00853de0, &args![1001u32]).u32(), 0xdddd);
    }

    #[test]
    fn a_stats_block_starts_empty_and_tracks_the_extremes_unsigned() {
        let mut e = Engine::new();
        let stats = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(stats + offset, 0x1234_5678);
        }
        assert_eq!(e.call(0x00854830, &args![stats]).u32(), stats);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(stats + 4 * i)).collect();
        assert_eq!(words, [0, 0, 0, 0, 0xffff_ffff, 0]);
        e.call(0x00854880, &args![stats, 5u32, 0u32, 10u32]);
        // 15 is below 0xffffffff: only an unsigned compare lowers the minimum.
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(stats + 4 * i)).collect();
        assert_eq!(words, [1, 5, 0, 10, 15, 15]);
        e.call(0x00854880, &args![stats, 1u32, 1u32, 1u32]);
        e.call(0x00854880, &args![stats, 20u32, 0u32, 0u32]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(stats + 4 * i)).collect();
        assert_eq!(words, [3, 26, 1, 11, 3, 20]);
        assert_eq!(e.call(0x00854900, &args![stats]).u32(), 38);
        assert_eq!(
            e.call(0x00854920, &args![stats]).f32(),
            (38.0f64 / 3.0) as f32
        );
    }

    #[test]
    fn the_average_of_nothing_follows_the_x87_division() {
        let mut e = Engine::new();
        let stats = e.mem.alloc(0x18);
        let nan = e.call(0x00854920, &args![stats]).f32();
        assert!(
            nan.is_nan() && nan.is_sign_negative(),
            "0 / 0 is the real indefinite"
        );
        e.mem.set_u32(stats + 4, 9);
        assert_eq!(e.call(0x00854920, &args![stats]).f32(), f32::INFINITY);
    }

    #[test]
    fn the_comparators_order_larger_totals_first() {
        let mut e = Engine::new();
        let big = e.mem.alloc(0x120);
        let small = e.mem.alloc(0x120);
        let same = e.mem.alloc(0x120);
        for (record, sizes) in [(big, [4u32, 4, 4]), (small, [1, 1, 1]), (same, [2, 3, 7])] {
            e.mem.set_u32(record + 8, sizes[0]);
            e.mem.set_u32(record + 0xc, sizes[1]);
            e.mem.set_u32(record + 0x10, sizes[2]);
            e.mem.set_u32(record + 0x104, sizes[0] * 10);
            e.mem.set_u32(record + 0x108, sizes[1] * 10 + sizes[2]);
        }
        // The form records total 12, 3 and 12; the misc records 4 * 10 + 44
        // = 84, 21 and 20 + 37 = 57.
        assert_eq!(e.call(0x00854960, &args![big, small]).i32(), -1);
        assert_eq!(e.call(0x00854960, &args![small, big]).i32(), 1);
        assert_eq!(e.call(0x00854960, &args![big, same]).i32(), 0);
        assert_eq!(e.call(0x00854bc0, &args![big, same]).i32(), -1);
        assert_eq!(e.call(0x00854bc0, &args![small, same]).i32(), 1);
        assert_eq!(e.call(0x00854bc0, &args![same, same]).i32(), 0);
    }

    fn record_fields(e: &Engine, record: u32) -> [u32; 6] {
        [
            e.mem.u32(record),
            e.mem.u32(record + 4),
            e.mem.u32(record + 8),
            e.mem.u32(record + 0xc),
            e.mem.u32(record + 0x10),
            e.mem.u32(record + 0x14),
        ]
    }

    #[test]
    fn recording_a_form_reuses_or_creates_the_list_for_its_key() {
        for found in [true, false] {
            let mut e = engine();
            let log = calls();
            let this = e.mem.alloc(0x20);
            let existing = e.mem.alloc(8);
            let looked_up = Rc::new(Cell::new(0u32));
            let seen = looked_up.clone();
            e.register_double(0x0085_3130, move |e, a| {
                assert_eq!(a[0], this);
                seen.set(a[1]);
                if found {
                    e.mem.set_u32(a[2], existing);
                }
                u32::from(found).into_ret()
            });
            e.register(0x0096_a2d0, |_, a| a[0].into_ret());
            spy(&mut e, &log, 0x0084_4700, 0);
            spy(&mut e, &log, 0x007a_7eb0, 0);
            e.call(
                0x008549b0,
                &args![
                    this, 0x10u32, 0x20u32, 0x30u32, 0x40u32, 0x50u32, 0x60u32, 0x70u32, 0x80u32
                ],
            );
            assert_eq!(looked_up.get(), 0x30);
            let calls = logged(&log);
            let insert = calls.last().unwrap();
            assert_eq!(insert.0, 0x007a_7eb0);
            assert_eq!(insert.1[2], 0x0085_4960);
            let record = insert.1[1];
            assert_eq!(
                record_fields(&e, record),
                [0x10, 0x40, 0x50, 0x60, 0x70, 0x20]
            );
            assert_eq!(e.mem.u8(record + 0x18), 0x80);
            if found {
                assert_eq!(insert.1[0], existing);
                assert_eq!(calls.len(), 1, "the list exists, nothing is added");
            } else {
                let list = insert.1[0];
                assert_ne!(list, 0);
                assert_ne!(list, existing);
                assert_eq!(calls[0], (0x0084_4700, vec![this, 0x30, list]));
            }
        }
    }

    #[test]
    fn a_saved_form_data_starts_with_a_cleared_word_at_four() {
        let mut e = engine();
        let log = calls();
        spy(&mut e, &log, 0x0044_dee0, 0);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00854af0, &args![this]).u32(), this);
        assert_eq!(logged(&log), [(0x0044_dee0, vec![this + 4])]);
    }

    #[test]
    fn saving_a_form_resets_the_buffer_and_fills_its_header() {
        let mut e = engine();
        let log = calls();
        e.map(0x011d_e134, 4);
        e.mem.set_u32(0x011d_e134, 0x7777);
        for (addr, result) in [
            (0x0084_7db0, 0),
            (0x0085_1110, 9),
            (0x0040_1170, 0x28),
            (0x0084_e3a0, 0x500),
            (0x0086_5ad0, 0),
            (0x0050_f9c0, 0),
            (0x0084_eb80, 0),
            (0x7100_0054, 0),
        ] {
            spy(&mut e, &log, addr, result);
        }
        let form = object(&mut e, 0x20, &[(0x54, 0x7100_0054)]);
        let this = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(0x24);
        e.call(0x00854b10, &args![this, form, 0xc0u32, buffer]);
        assert_eq!(
            logged(&log),
            [
                (0x0084_7db0, vec![buffer]),
                (0x0085_1110, vec![0x7777]),
                (0x0040_1170, vec![form]),
                (0x0084_e3a0, vec![form]),
                (0x0086_5ad0, vec![buffer, 0x500, 0xc0, 0x28, 9]),
                (0x0050_f9c0, vec![buffer, form]),
                (0x0084_eb80, vec![buffer, form, 0xc0]),
                (0x7100_0054, vec![form, buffer]),
            ]
        );
    }

    #[test]
    fn text_is_written_only_when_there_is_a_file() {
        let mut e = engine();
        let log = calls();
        spy(&mut e, &log, 0x0047_3180, 0);
        let this = e.mem.alloc(0x20);
        let line = text(&mut e, "hello");
        e.call(0x00854b80, &args![this, line]);
        assert!(logged(&log).is_empty());
        e.mem.set_u32(this + 0x18, 0x5000);
        e.call(0x00854b80, &args![this, line]);
        assert_eq!(logged(&log), [(0x0047_3180, vec![0x5000, line, 5])]);
    }

    #[test]
    fn a_misc_entry_keeps_its_name_and_both_sizes() {
        let mut e = engine();
        let log = calls();
        spy(&mut e, &log, 0x007a_7eb0, 0);
        let this = e.mem.alloc(0x20);
        let name = text(&mut e, "Globals");
        e.call(0x00854c10, &args![this, name, 400u32, 25u32]);
        let calls = logged(&log);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this + 0x10);
        assert_eq!(calls[0].1[2], 0x0085_4bc0);
        let record = calls[0].1[1];
        assert_eq!(text_of(&e, record), "Globals");
        assert_eq!(e.mem.u32(record + 0x104), 400);
        assert_eq!(e.mem.u32(record + 0x108), 25);
        assert_eq!(e.mem.block_size(record), Some(0x110));
    }

    #[test]
    fn the_map_constructors_set_the_vtables_and_clear_the_buckets() {
        let mut e = engine();
        let log = calls();
        e.register_double(0x00aa_1070, |e, a| e.mem.alloc(a[0]).into_ret());
        spy(&mut e, &log, 0x0040_3d30, 0);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00854c80, &args![this, 4u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FORM_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 4);
        let buckets = e.mem.u32(this + 8);
        assert_eq!(logged(&log), [(0x0040_3d30, vec![buckets, 0, 16])]);
        assert_eq!(e.call(0x00854ce0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FORM_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 2);
    }

    #[test]
    fn collection_records_unloaded_and_loaded_forms_and_global_data() {
        let mut e = engine();
        let log = calls();
        e.map(SAVE_MANAGER, 4);
        e.mem.set_u32(SAVE_MANAGER, 0x1111);
        e.map(0x011d_e134, 4);
        e.map(0x011a_216c, 0x1000);
        e.map(0x011a_11fc, 0x1100);
        let name_two = text(&mut e, "Two");
        let name_kilo = text(&mut e, "Kilo");
        e.mem.set_u32(0x011a_216c + 2 * 4, name_two);
        e.mem.set_u32(0x011a_11fc + 1000 * 4, name_kilo);
        let this = e.mem.alloc(0x20);
        let map = e.mem.alloc(0x10);
        let unloaded = e.mem.alloc(0x10);
        e.mem.set_u32(unloaded, 0xaa);
        let loaded = e.mem.alloc(0x10);
        e.mem.set_u32(loaded, 0xbb);
        let form = object(&mut e, 0x40, &[(0x54, 0x7100_0054)]);
        // 00559450 is both "the map of the manager" and "is this unloaded".
        e.register_double(0x0055_9450, move |_, a| {
            let answer = if a[0] == 0x1111 {
                map
            } else {
                u32::from(a[0] == unloaded + 4)
            };
            answer.into_ret()
        });
        e.register_double(0x004b_9ba0, move |_, a| {
            assert_eq!(a[0], map);
            1u32.into_ret()
        });
        let step = Rc::new(Cell::new(0u32));
        let counter = step.clone();
        e.register_double(0x006b_7f20, move |e, a| {
            assert_eq!(a[0], map);
            let n = counter.get();
            counter.set(n + 1);
            let (key, item) = if n == 0 {
                (0x10, unloaded)
            } else {
                (0x20, loaded)
            };
            e.mem.set_u32(a[1], u32::from(n == 0));
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], item);
            Ret::default()
        });
        for (addr, result) in [
            (0x0086_59c0, 0),
            (0x0084_b450, 5),
            (0x0086_6450, 0x20),
            (0x0086_63e0, 0x30),
            (0x0084_9220, 0x77),
            (0x0084_7db0, 0),
            (0x0085_1110, 9),
            (0x0040_1170, 0x28),
            (0x0084_e3a0, 0x500),
            (0x0086_5ad0, 0),
            (0x0050_f9c0, 0),
            (0x0084_7dd0, 0),
            (0x008a_81c0, 3),
            (0x0044_edb0, 0x40),
            (0x0086_5b10, 0x50),
            (0x0085_3130, 0),
            (0x0084_4700, 0),
            (0x007a_7eb0, 0),
            (0x7100_0054, 0),
        ] {
            spy(&mut e, &log, addr, result);
        }
        e.register(0x0096_a2d0, |_, a| a[0].into_ret());
        e.register_double(0x0048_39c0, move |_, a| {
            assert_eq!(a, &[0x20]);
            form.into_ret()
        });
        // The buffer's two words 00853dc0 subtracts; the reset (a spy here)
        // would clear them.
        e.register_double(0x0084_eb80, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0x300);
            e.mem.set_u32(a[0] + 0x10, 0x100);
            Ret::default()
        });
        e.register_double(0x0084_bcd0, |_, a| {
            u32::from(a[0] == 2 || a[0] == 1000).into_ret()
        });
        e.call(0x00853b90, &args![this]);
        let calls = logged(&log);
        let inserts: Vec<_> = calls.iter().filter(|c| c.0 == 0x007a_7eb0).collect();
        assert_eq!(inserts.len(), 4);
        // The unloaded entry: sizes from the unloaded-form helpers.
        let record = inserts[0].1[1];
        assert_eq!(record_fields(&e, record), [0x10, 0xaa, 0x30, 0x20, 0, 0]);
        assert_eq!(e.mem.u8(record + 0x18), 5);
        assert_eq!(inserts[0].1[2], 0x0085_4960);
        // The loaded one: sizes measured on the save buffer.
        let record = inserts[1].1[1];
        assert_eq!(
            record_fields(&e, record),
            [0x20, 0xbb, 0x200, 0x50, 0x40, form]
        );
        assert_eq!(e.mem.u8(record + 0x18), 3);
        let set_at: Vec<_> = calls.iter().filter(|c| c.0 == 0x0084_4700).collect();
        assert_eq!(set_at.len(), 2);
        assert_eq!(set_at[0].1, [this, 0x77, inserts[0].1[0]]);
        assert_eq!(set_at[1].1, [this, 0x28, inserts[1].1[0]]);
        // Global data records 2 and 1000.
        for (insert, name, key) in [(inserts[2], "Two", 2), (inserts[3], "Kilo", 1000)] {
            assert_eq!(insert.1[0], this + 0x10, "global record {key}");
            assert_eq!(insert.1[2], 0x0085_4bc0);
            let record = insert.1[1];
            assert_eq!(text_of(&e, record), name);
            assert_eq!(e.mem.u32(record + 0x104), 0x200);
            assert_eq!(e.mem.u32(record + 0x108), 0x40);
        }
        let forms_saved: Vec<_> = calls.iter().filter(|c| c.0 == 0x7100_0054).collect();
        assert_eq!(forms_saved.len(), 1, "only the loaded form is saved");
    }

    /// The texts the report formats and prints (the exe's own strings,
    /// written by hand; the long formats are shortened).
    const REPORT_TEXTS: &[(u32, &str)] = &[
        (0x0108_03f0, "H %s %s %s %s %s %s\n"),
        (0x0108_0414, "FormID"),
        (0x0108_041c, "Size"),
        (0x0104_4aec, "Flags"),
        (0x0108_0424, "Ver"),
        (0x0105_ac4c, "Name"),
        (0x0108_0428, "Changes"),
        (0x0108_03e4, "%ss:\n"),
        (0x0108_03dc, "Base"),
        (0x0102_909c, "None"),
        (0x0105_a010, "High"),
        (0x0108_03c8, "MidHigh"),
        (0x0108_03d4, "MidLow"),
        (0x0105_a004, "Low"),
        (0x0108_03c0, "%s(%i)"),
        (0x0108_03b8, "%-12s"),
        (0x0101_9f08, "%s"),
        (0x0101_1584, ""),
        (0x0108_0398, "%08X %i %08X %i %-8s| %s\n"),
        (
            0x0108_0310,
            "TYPE %s n=%i %s total=%i data=%i over=%i dbg=%i %s min=%i %s max=%i %s avg=%.2f\n",
        ),
        (
            0x0108_0278,
            "ALL n=%i total=%i data=%i over=%i dbg=%i min=%i max=%i avg=%.2f\n",
        ),
        (0x0108_0264, "MISC\n"),
        (0x0108_0254, "%i %s\n"),
        (
            0x0108_01c0,
            "MISCALL n=%i total=%i data=%i over=%i dbg=%i min=%i max=%i avg=%.2f\n",
        ),
        (0x0108_01a0, "HISTORY\n"),
        (0x0108_019c, "***"),
        (0x0107_7dc0, "---"),
        (0x0108_0198, "  "),
        (0x0101_f520, "\n"),
        (0x0108_0164, "UNLOADED"),
    ];

    /// A small `sprintf` for the conversions the report formats use.
    fn format_c(e: &Engine, format: u32, words: &[u32]) -> String {
        let f = e.mem.cstr(format);
        let mut out = String::new();
        let mut i = 0;
        let mut next = 0;
        while i < f.len() {
            if f[i] != b'%' {
                out.push(f[i] as char);
                i += 1;
                continue;
            }
            i += 1;
            let (mut left, mut zero, mut width, mut precision) = (false, false, 0usize, 6usize);
            while f[i] == b'-' || f[i] == b'0' {
                if f[i] == b'-' {
                    left = true;
                } else {
                    zero = true;
                }
                i += 1;
            }
            while f[i].is_ascii_digit() {
                width = width * 10 + (f[i] - b'0') as usize;
                i += 1;
            }
            if f[i] == b'.' {
                i += 1;
                precision = 0;
                while f[i].is_ascii_digit() {
                    precision = precision * 10 + (f[i] - b'0') as usize;
                    i += 1;
                }
            }
            let word = words[next];
            next += 1;
            let body = match f[i] {
                b's' => text_of(e, word),
                b'i' => (word as i32).to_string(),
                b'X' => format!("{word:X}"),
                b'f' => {
                    let value = f64::from_bits(u64::from(word) | u64::from(words[next]) << 32);
                    next += 1;
                    format!("{value:.precision$}")
                }
                other => panic!("unsupported conversion %{}", other as char),
            };
            i += 1;
            let pad = " ".repeat(width.saturating_sub(body.len()));
            if left {
                out.push_str(&body);
                out.push_str(&pad);
            } else if zero {
                out.push_str(&pad.replace(' ', "0"));
                out.push_str(&body);
            } else {
                out.push_str(&pad);
                out.push_str(&body);
            }
        }
        out
    }

    /// An engine for `OutputStats` whose file collects the text written.
    fn report_engine() -> (Engine, Rc<RefCell<String>>) {
        let mut e = engine();
        for (addr, s) in REPORT_TEXTS {
            text_at(&mut e, *addr, s);
        }
        e.register(0x0040_6d00, |e, a| {
            assert_eq!(a[1], 1000);
            let s = format_c(e, a[2], &a[3..]);
            e.mem.set_cstr(a[0], s.as_bytes());
            (s.len() as u32).into_ret()
        });
        e.register(0x00ec_61c0, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        for addr in [
            0x0086_59c0,
            0x0084_7dd0,
            0x0084_7db0,
            0x0050_f9c0,
            0x0084_eb80,
            0x0085_1110,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(SAVE_MANAGER, 4);
        e.mem.set_u32(SAVE_MANAGER, 0x1111);
        e.map(0x011d_e134, 4);
        e.map(FORM_TYPE_NAMES, 0x40);
        let out = Rc::new(RefCell::new(String::new()));
        let sink = out.clone();
        e.register_double(0x0047_3180, move |e, a| {
            assert_eq!(a[2] as usize, e.mem.cstr(a[1]).len());
            sink.borrow_mut().push_str(&text_of(e, a[1]));
            0u32.into_ret()
        });
        (e, out)
    }

    #[test]
    fn the_report_lists_forms_totals_misc_and_history() {
        let (mut e, out) = report_engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x5000);
        let weapon = text(&mut e, "Weapon");
        e.mem.set_u32(FORM_TYPE_NAMES + 3 * 12, weapon);
        // Two forms of type 3: one loaded with a base and two changed flags,
        // one unloaded with one changed flag.
        e.register(0x7100_00f0, |_, _| 1u32.into_ret());
        e.register(0x7100_00fc, |_, _| 1u32.into_ret());
        e.register(0x7100_0054, |_, _| Ret::default());
        let rifle = object(
            &mut e,
            0x40,
            &[
                (0xf0, 0x7100_00f0),
                (0xfc, 0x7100_00fc),
                (0x54, 0x7100_0054),
            ],
        );
        let first = e.mem.alloc(0x20);
        for (offset, value) in [(0, 0xabcd), (8, 10), (0xc, 20), (0x10, 30), (0x14, rifle)] {
            e.mem.set_u32(first + offset, value);
        }
        e.mem.set_u8(first + 0x18, 9);
        let second = e.mem.alloc(0x20);
        for (offset, value) in [(0, 0xbeef), (8, 1), (0xc, 2), (0x10, 3)] {
            e.mem.set_u32(second + offset, value);
        }
        e.mem.set_u8(second + 0x18, 4);
        let last = e.mem.alloc(8);
        e.mem.set_u32(last, second);
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, first);
        e.mem.set_u32(head + 4, last);
        // The misc list: one record, the node embedded at +0x10.
        let globals = e.mem.alloc(0x110);
        e.mem.set_cstr(globals, b"Globals");
        e.mem.set_u32(globals + 0x104, 400);
        e.mem.set_u32(globals + 0x108, 25);
        e.mem.set_u32(this + 0x10, globals);
        e.register(0x004b_9ba0, |_, _| 1u32.into_ret());
        e.register_double(0x006b_7f20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], 3);
            e.mem.set_u32(a[3], head);
            Ret::default()
        });
        let mask = Rc::new(Cell::new(0u32));
        let saved = mask.clone();
        e.register_double(0x0086_5ad0, move |_, a| {
            saved.set(a[2]);
            Ret::default()
        });
        let current = mask.clone();
        e.register_double(0x0084_e3a0, move |_, a| {
            let answer: u32 = if a[0] == rifle {
                0x77
            } else {
                match current.get() {
                    0 => 100,
                    1 => 130,
                    4 => 170,
                    other => panic!("unexpected mask {other}"),
                }
            };
            answer.into_ret()
        });
        e.register(0x0040_1170, |_, _| 0x28u32.into_ret());
        e.register(0x008d_8520, |_, _| 0x1234u32.into_ret());
        e.register_double(0x0045_cd60, |_, a| {
            assert_eq!(a, &[0x1234]);
            2u32.into_ret()
        });
        e.register_double(0x0042_80f0, move |_, a| {
            let flagged = (a[0] == first + 4 && (a[1] == 1 || a[1] == 4))
                || (a[0] == second + 4 && a[1] == 2);
            u32::from(flagged).into_ret()
        });
        let flag_a = text(&mut e, "FlagA");
        let flag_b = text(&mut e, "FlagB");
        let flag_c = text(&mut e, "FlagC");
        e.register_double(0x0083_fef0, move |_, a| {
            assert_eq!((a[1], a[2]), (3, 1));
            let name = match a[0] {
                1 => flag_a,
                2 => flag_b,
                4 => flag_c,
                other => panic!("flag {other:#x}"),
            };
            name.into_ret()
        });
        e.register_double(0x0055_9450, move |_, a| {
            let answer = match a[0] {
                x if x == first + 4 => 0x55u32,
                x if x == second + 4 => 0x66,
                other => panic!("00559450 on {other:#x}"),
            };
            answer.into_ret()
        });
        let rifle_name = text(&mut e, "Rifle");
        e.register_double(0x0055_d520, move |_, a| {
            assert_eq!(a[0], rifle);
            rifle_name.into_ret()
        });
        // The save game history.
        let lines = [
            text(&mut e, "***Start"),
            text(&mut e, "inside"),
            text(&mut e, "---End"),
            text(&mut e, "after"),
        ];
        e.register_double(0x0044_1110, |_, a| {
            assert_eq!(a[0], 0x1111);
            0x4242u32.into_ret()
        });
        e.register(0x0062_0b80, |_, a| {
            assert_eq!(a[0], 0x4242);
            4u32.into_ret()
        });
        e.register_double(0x0084_e320, move |_, a| {
            assert_eq!(a[0], 0x4242);
            lines[a[1] as usize].into_ret()
        });
        e.register(0x0045_64f0, |e, a| {
            assert_eq!(a[2], 3);
            let left = e.mem.cstr(a[0]);
            let right = e.mem.cstr(a[1]);
            u32::from(left[..3] != right[..3]).into_ret()
        });
        e.call(0x00853e10, &args![this]);
        let expected = [
            "H FormID Size Flags Ver Name Changes\n",
            "Weapons:\n",
            "0000ABCD 60 00000055 9 Rifle   | MidLow(100) FlagA(30)   FlagC(70)   \n",
            "0000BEEF 6 00000066 4 UNLOADED| FlagB       \n",
            "TYPE Weapon n=2 Weapon total=66 data=11 over=22 dbg=33 Weapon min=6 Weapon max=60 Weapon avg=33.00\n",
            "ALL n=2 total=66 data=11 over=22 dbg=33 min=6 max=60 avg=33.00\n",
            "MISC\n",
            "425 Globals\n",
            "MISCALL n=1 total=425 data=400 over=0 dbg=25 min=425 max=425 avg=425.00\n",
            "HISTORY\n",
            "Start\n",
            "  inside\n",
            "End\n",
            "after\n",
        ]
        .concat();
        assert_eq!(*out.borrow(), expected);
    }

    #[test]
    fn an_empty_report_has_infinite_minimums_and_no_average() {
        let (mut e, out) = report_engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x5000);
        e.register(0x004b_9ba0, |_, _| 0u32.into_ret());
        e.register(0x0044_1110, |_, _| 0x4242u32.into_ret());
        e.register(0x0062_0b80, |_, _| 0u32.into_ret());
        e.call(0x00853e10, &args![this]);
        let expected = [
            "H FormID Size Flags Ver Name Changes\n",
            "ALL n=0 total=0 data=0 over=0 dbg=0 min=-1 max=0 avg=NaN\n",
            "MISC\n",
            "MISCALL n=0 total=0 data=0 over=0 dbg=0 min=-1 max=0 avg=NaN\n",
        ]
        .concat();
        assert_eq!(*out.borrow(), expected);
    }

    #[test]
    fn the_report_writes_nothing_without_a_file() {
        let (mut e, out) = report_engine();
        let this = e.mem.alloc(0x20);
        e.register(0x004b_9ba0, |_, _| 0u32.into_ret());
        e.register(0x0044_1110, |_, _| 0x4242u32.into_ret());
        e.register(0x0062_0b80, |_, _| 0u32.into_ret());
        e.call(0x00853e10, &args![this]);
        assert!(out.borrow().is_empty());
    }
}

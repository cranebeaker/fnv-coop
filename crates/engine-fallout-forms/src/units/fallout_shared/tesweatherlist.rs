//! `fallout shared/tesweatherlist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004f10b0` calls on the member (`005827d0`).
const CALLEE_005827D0: u32 = 0x0058_27d0;

// Translated from 004f10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESWeatherList::GetRandomWeather` (Xbox PDB name in the engine map). Thin wrapper: calls `005827d0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_004f10b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_005827D0, &args![this.byte_add(0x8)]).u32()
}

/// `006815c0` (`ECX`, answers `this`: a list node read as its item word).
const LIST_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (`ECX`, answers the word at +4: a list node's next).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00470470` (`ECX`): empties a `BSSimpleList`.
const LIST_REMOVE_ALL_00470470: u32 = 0x0047_0470;
/// `00526ac0` (`ECX` = a `TESGlobal`): its value (the float at +0x24) in `ST0`.
const GLOBAL_VALUE_00526AC0: u32 = 0x0052_6ac0;
/// `00487f50` (`cdecl`, no words): a random number (as other translations call it).
const RANDOM_00487F50: u32 = 0x0048_7f50;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The compiler's vector constructor iterator (`00401050`, `stdcall`: array, element size,
/// count, constructor).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size; `AL` whether it read).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `0084e3a0` (`ECX`): the word at +0xc (the form id).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "FORMS: Error getting TESWeatherList chunk for form ID: %d"
const MESSAGE_CHUNK_ERROR: u32 = 0x0103_18b4;
/// "DEFAULT: Error while loading weather list...cannot identify TESWeather from form ID:
/// (%08X)"
const MESSAGE_NOT_A_WEATHER: u32 = 0x0103_1858;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `005a6b10` (`ECX`): swaps the byte order of three words in place.
const SWAP_THREE_WORDS: u32 = 0x005a_6b10;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `00824340` (`ECX` = a new 0xc-byte block; engine map `BSSimpleList<WeatherEntry_P>::AddHead`,
/// in `tesclimate.cpp`): makes the weather entry, answers it.
const NEW_ENTRY_00824340: u32 = 0x0082_4340;
/// `005ae3d0` (`ECX` = list, one stack word: address of the new head value): pushes a value
/// onto the front of a `BSSimpleList`.
const LIST_ADD_HEAD_005AE3D0: u32 = 0x005a_e3d0;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@`, `.?AVTESWeather@@` and `.?AVTESGlobal@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_WEATHER: u32 = 0x0118_629c;
const TYPE_TES_GLOBAL: u32 = 0x0118_3ad4;
/// `0083fd60` (`ECX` = list, one stack word: a compare function), here with `00582be0`.
const LIST_SORT_0083FD60: u32 = 0x0083_fd60;
const COMPARE_00582BE0: u32 = 0x0058_2be0;

/// `FISTP qword` under a control word with rounding set to truncate (`OR 0xc00`): the value
/// toward zero, or the integer indefinite for NaN and values out of the 64-bit range.
fn truncate_to_qword(value: f64) -> u64 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0x8000_0000_0000_0000
    } else {
        value.trunc() as i64 as u64
    }
}

/// A weather entry's chance (`WeatherEntry`: weather +0, chance +4, global +8): with a global,
/// its value truncated (the low word of the 64-bit `FISTP`); otherwise the chance word.
fn entry_chance(e: &mut Engine, entry: u32) -> u32 {
    let global = e.mem.u32(entry + 8);
    if global != 0 {
        let value = e.call(GLOBAL_VALUE_00526AC0, &args![global]).f64();
        truncate_to_qword(value) as u32
    } else {
        e.mem.u32(entry + 4)
    }
}

// Translated from 00582570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `00582940` constructs each 0xc-byte read entry
/// with it), `ECX`: clears the words at +8, +4 and +0, answers `this`.
pub fn fn_00582570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 00582780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` = a `BSSimpleList` of heap items: from
/// the head node while the node is nonzero, deletes its item (read through `006815c0`,
/// `operator delete`, null items too) and goes to the next node (`00726070`); then empties
/// the list (`00470470`).
pub fn fn_00582780(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 {
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let item = e.mem.u32(item_at);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    e.call(LIST_REMOVE_ALL_00470470, &args![this]);
}

// Translated from 005827d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `TESWeatherList::GetRandomWeather` `004f10b0`
/// calls it on its list), `ECX` = a `BSSimpleList` of `WeatherEntry` pointers: picks a weather
/// by chance. The total adds each entry's chance (a global's truncated value, or the chance
/// word) over all nodes, skipping nodes without an entry; a total of 0 answers null. A random
/// number (`00487f50`) modulo the total is then looked up over all nodes: each nonzero chance
/// spans `[low, low + chance - 1]` (the running high bound starts at -1), and the first span
/// holding the number stops the walk. The answer is the weather of the last entry read (the
/// found one, or the last node's), null when that node had none.
pub fn fn_005827d0(e: &mut Engine, this: Ptr) -> u32 {
    let mut total = 0u32;
    let mut node = this.addr();
    while node != 0 {
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        // 005827f7: a node without an entry adds nothing and the walk goes on to the next
        // node (00582851); it ends only at a zero node (005827ea).
        if e.mem.u32(item_at) != 0 {
            let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let entry = e.mem.u32(item_at);
            let chance = if e.mem.u32(entry + 8) != 0 {
                let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
                let entry = e.mem.u32(item_at);
                entry_chance(e, entry)
            } else {
                let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
                let entry = e.mem.u32(item_at);
                e.mem.u32(entry + 4)
            };
            total = total.wrapping_add(chance);
        }
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    if total == 0 {
        return 0;
    }
    let pick = e.call(RANDOM_00487F50, &args![]).u32() % total;
    let mut low = 0u32;
    let mut high = u32::MAX;
    let mut entry = 0u32;
    let mut node = this.addr();
    while node != 0 {
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        entry = e.mem.u32(item_at);
        if entry != 0 {
            let chance = entry_chance(e, entry);
            if chance != 0 {
                high = high.wrapping_add(chance);
                if low <= pick && pick <= high {
                    break;
                }
                low = low.wrapping_add(chance);
            }
        }
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    if entry != 0 {
        e.mem.u32(entry)
    } else {
        0
    }
}

// Translated from 00582940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00582940 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef7bd6, state variable) is not translated
/// `TESWeatherList::LoadWeatherListChunk` (Xbox PDB), `cdecl` with three words: the list,
/// the file and the owning form; nothing happens unless all three are nonzero or when the
/// chunk holds no whole 12-byte entry (size / 0xc). The list's entries are freed
/// (`00582780`), an array of the entries is allocated (`count * 0xc`, all ones when that
/// overflows; each entry cleared by `00582570` through the vector constructor iterator when
/// the allocation succeeded) and the chunk read into it (`GetChunkData_ov3`, a failed read
/// reported with the form id). For each entry: byte-swapped (`005a6b10`) when the file needs
/// it, its weather id given the file's compile index; a new list entry (`00824340` on a new
/// 0xc-byte block, null when that fails) is pushed onto the list (`005ae3d0`); the head
/// entry's weather is the id's form cast to `TESWeather` (reported when it is none), its
/// chance the read chance, and, once the global id has the compile index too, its global the
/// id's form cast to `TESGlobal`. Then the list is passed to `0083fd60` with `00582be0` and
/// the array deleted.
pub fn tes_weather_list_load_weather_list_chunk(e: &mut Engine, list: Ptr, file: Ptr, form: Ptr) {
    if file.is_null() || list.is_null() || form.is_null() {
        return;
    }
    let count = e.call(CHUNK_SIZE, &args![file]).u32() / 0xc;
    if count == 0 {
        return;
    }
    fn_00582780(e, list);
    let size = u64::from(count) * 0xc;
    let size = if size > u64::from(u32::MAX) {
        u32::MAX
    } else {
        size as u32
    };
    let block = e.call(ALLOCATE, &args![size]).u32();
    let array = if block != 0 {
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![block, 0xcu32, count, 0x0058_2570u32],
        );
        block
    } else {
        0
    };
    let read = e
        .call(
            GET_CHUNK_DATA_SIZED,
            &args![file, array, count.wrapping_mul(0xc)],
        )
        .bool();
    if !read {
        let id = e.call(FORM_ID_0084E3A0, &args![form]).u32();
        e.call(LOG, &args![MESSAGE_CHUNK_ERROR, id]);
    }
    e.with_stack(4, |e, new_head| {
        for index in 0..count {
            let read_entry = array.wrapping_add(index.wrapping_mul(0xc));
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_THREE_WORDS, &args![read_entry]);
            }
            e.call(ADD_COMPILE_INDEX, &args![read_entry, file]);
            let node_block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let entry = if node_block != 0 {
                e.call(NEW_ENTRY_00824340, &args![node_block]).u32()
            } else {
                0
            };
            e.mem.set_u32(new_head.addr(), entry);
            e.call(LIST_ADD_HEAD_005AE3D0, &args![list, new_head]);
            let item_at = e.call(LIST_ITEM_006815C0, &args![list]).u32();
            let head = e.mem.u32(item_at);
            let id = e.mem.u32(read_entry);
            let found = e.call(LOOKUP_FORM, &args![id]).u32();
            let weather = e
                .call(
                    DYNAMIC_CAST,
                    &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_WEATHER, 0u32],
                )
                .u32();
            e.mem.set_u32(head, weather);
            if e.mem.u32(head) == 0 {
                let id = e.mem.u32(read_entry);
                e.call(LOG, &args![MESSAGE_NOT_A_WEATHER, id]);
            }
            let item_at = e.call(LIST_ITEM_006815C0, &args![list]).u32();
            let head = e.mem.u32(item_at);
            let chance = e.mem.u32(read_entry + 4);
            e.mem.set_u32(head + 4, chance);
            e.call(ADD_COMPILE_INDEX, &args![read_entry + 8, file]);
            let id = e.mem.u32(read_entry + 8);
            let found = e.call(LOOKUP_FORM, &args![id]).u32();
            let global = e
                .call(
                    DYNAMIC_CAST,
                    &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0u32],
                )
                .u32();
            let item_at = e.call(LIST_ITEM_006815C0, &args![list]).u32();
            let head = e.mem.u32(item_at);
            e.mem.set_u32(head + 8, global);
        }
    });
    e.call(LIST_SORT_0083FD60, &args![list, COMPARE_00582BE0]);
    e.call(OPERATOR_DELETE, &args![array]);
}

/// `VATS::GetCount` in the engine map (`005ae380`, `ECX` = list; a folded body): the list's
/// item count.
const LIST_COUNT: u32 = 0x005a_e380;
/// `00401500` (`cdecl`, no words): the byte at `011c54ba`, whether saved data has its byte
/// order swapped.
const SWAPS_BYTE_ORDER: u32 = 0x0040_1500;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: chunk, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// The game's `memcpy` wrapper (`00401460`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x0040_1460;
/// `_memcmp` (engine map, `00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;
/// `008256d0` (`ECX` = list): whether its head node is empty (both words 0).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `00905820` (`ECX` = list, one stack word: the address of the item): adds an item to a
/// `BSSimpleList`.
const LIST_ADD_00905820: u32 = 0x0090_5820;

/// The byte size the exe allocates for `count` entries of 0xc bytes: `count * 0xc`, or all
/// ones when the product overflows (`MUL`, `SETO`, `NEG`, `OR`).
fn entry_array_size(count: u32) -> u32 {
    let size = u64::from(count) * 0xc;
    if size > u64::from(u32::MAX) {
        u32::MAX
    } else {
        size as u32
    }
}

/// When saved data is byte-swapped (`00401500`): `005a6b10` on each of the first `count`
/// entries of the array.
fn swap_entries(e: &mut Engine, array: u32, count: u32) {
    if !e.call(SWAPS_BYTE_ORDER, &[]).bool() {
        return;
    }
    let mut index = 0u32;
    while index < count {
        e.call(
            SWAP_THREE_WORDS,
            &args![index.wrapping_mul(0xc).wrapping_add(array)],
        );
        index += 1;
    }
}

// Translated from 00582380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00582380 - the FS:[0] exception frame (unwinding the array allocation) is not translated.
/// `TESWeatherList::Save` (Xbox PDB, `ECX` = the list, `RET 4`: the chunk): for a list with
/// items (`005ae380`), builds an array of 0xc-byte entries (`00401000` with the overflow-safe
/// size, each cleared by `00582570` through the vector constructor `00401050`): for every node
/// (empty items are not skipped) the weather's form id (`0084e3a0`), the chance word and the
/// global's form id (0 without one). The entries are byte-swapped (`005a6b10`) around the
/// chunk write (`TESForm::__AddChunkData`) when `00401500` says so, and the array deleted.
/// The exception frame around the vector construction is not translated.
pub fn tes_weather_list_save(e: &mut Engine, this: Ptr, chunk: u32) {
    let count = e.call(LIST_COUNT, &args![this]).u32();
    if count == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![entry_array_size(count)]).u32();
    let array = if block != 0 {
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![block, 0xcu32, count, 0x0058_2570u32],
        );
        block
    } else {
        0
    };
    let mut index = 0u32;
    let mut node = this.addr();
    while node != 0 {
        let entry_at = array.wrapping_add(index.wrapping_mul(0xc));
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let weather = e.mem.u32(e.mem.u32(item_at));
        let id = e.call(FORM_ID_0084E3A0, &args![weather]).u32();
        e.mem.set_u32(entry_at, id);
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let chance = e.mem.u32(e.mem.u32(item_at).wrapping_add(4));
        e.mem.set_u32(entry_at.wrapping_add(4), chance);
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let global_id = if e.mem.u32(e.mem.u32(item_at).wrapping_add(8)) != 0 {
            let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let global = e.mem.u32(e.mem.u32(item_at).wrapping_add(8));
            e.call(FORM_ID_0084E3A0, &args![global]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry_at.wrapping_add(8), global_id);
        index = index.wrapping_add(1);
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    swap_entries(e, array, index);
    e.call(
        ADD_CHUNK_DATA,
        &args![chunk, array, index.wrapping_mul(0xc)],
    );
    swap_entries(e, array, index);
    e.call(OPERATOR_DELETE, &args![array]);
}

// Translated from 005825a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005825a0 - the FS:[0] exception frame (unwinding the entry allocation) is not translated.
/// Unnamed in the map (`ECX` = the list, `RET 8`: another list and a byte flag; a copy by its
/// body): with another list, first frees this list's entries (`00582780`) unless the flag is
/// set; then, unless the other list is empty (`008256d0`), adds a copy of each of its entries
/// (every node, empty items included): a new 0xc-byte entry (`00401000`, `00824340`; null when
/// the allocation fails) gets the entry's 12 bytes (`00401460`) and is added (`00905820`).
/// The exception frame around the entry's construction is not translated.
pub fn fn_005825a0(e: &mut Engine, this: Ptr, other: u32, keep: u8) {
    if other == 0 {
        return;
    }
    if keep == 0 {
        fn_00582780(e, this);
    }
    if e.call(LIST_IS_EMPTY, &args![other]).bool() {
        return;
    }
    let mut node = other;
    while node != 0 {
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let entry = if block != 0 {
            e.call(NEW_ENTRY_00824340, &args![block]).u32()
        } else {
            0
        };
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let source = e.mem.u32(item_at);
        e.call(MEMCPY, &args![entry, source, 0xcu32]);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), entry);
            e.call(LIST_ADD_00905820, &args![this, slot]);
        });
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
}

// Translated from 00582690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = the list, `RET 4`: another list; a comparison by its body):
/// true ("different") without another list; false when both are empty (`008256d0`); true
/// when their counts differ (`005ae380`); otherwise true when some entry of this list (every
/// node) has no 12-byte match (`_memcmp`) among the other's nodes, else false. The inner walk
/// steps once more after a match.
pub fn fn_00582690(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if other == 0 {
        return true;
    }
    if e.call(LIST_IS_EMPTY, &args![this]).bool() && e.call(LIST_IS_EMPTY, &args![other]).bool() {
        return false;
    }
    let mine = e.call(LIST_COUNT, &args![this]).u32();
    if mine != e.call(LIST_COUNT, &args![other]).u32() {
        return true;
    }
    let mut node = this.addr();
    while node != 0 {
        let mut found = false;
        let mut theirs = other;
        while !found && theirs != 0 {
            let item_at = e.call(LIST_ITEM_006815C0, &args![theirs]).u32();
            let their_entry = e.mem.u32(item_at);
            let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let my_entry = e.mem.u32(item_at);
            found = e.call(MEMCMP, &args![my_entry, their_entry, 0xcu32]).u32() == 0;
            theirs = e.call(NODE_FOLLOWING_00726070, &args![theirs]).u32();
        }
        if !found {
            return true;
        }
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    false
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_10b0, fn_004f10b0(Ptr) -> u32),
        entry!(0x0058_2570, fn_00582570(Ptr) -> Ptr),
        entry!(0x0058_2780, fn_00582780(Ptr)),
        entry!(0x0058_27d0, fn_005827d0(Ptr) -> u32),
        entry!(
            0x0058_2940,
            tes_weather_list_load_weather_list_chunk(Ptr, Ptr, Ptr)
        ),
        entry!(0x0058_2380, tes_weather_list_save(Ptr, u32)),
        entry!(0x0058_25a0, fn_005825a0(Ptr, u32, u8)),
        entry!(0x0058_2690, fn_00582690(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004f10b0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_005827D0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_10b0, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_005827D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn list_doubles(e: &mut Engine) {
        e.register(LIST_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL_00470470, |_, _| Ret::default());
    }

    /// A list (head node at the returned address) of the given items.
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    fn weather_entry(e: &mut Engine, weather: u32, chance: u32, global: u32) -> u32 {
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, weather);
        e.mem.set_u32(entry + 4, chance);
        e.mem.set_u32(entry + 8, global);
        entry
    }

    #[test]
    fn fn_00582570_clears_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        assert_eq!(e.call(0x0058_2570, &args![this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0xc), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00582780_deletes_every_item_then_empties() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let list = make_list(&mut e, &[0x111, 0, 0x333]);
        e.call_log = Some(vec![]);
        e.call(0x0058_2780, &args![list]);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x111], vec![0], vec![0x333]]
        );
        assert_eq!(calls(&e, LIST_REMOVE_ALL_00470470), vec![vec![list]]);
    }

    fn pick_rig(random: u32) -> Engine {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register_double(RANDOM_00487F50, move |_, _| random.into_ret());
        // A global's value: the float at +0x24.
        e.register(GLOBAL_VALUE_00526AC0, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x24)),
            ..Ret::default()
        });
        e
    }

    #[test]
    fn fn_005827d0_picks_by_chance() {
        // Chances 10, 0, 25.9 (a global, truncated to 25), 5: total 40.
        for (random, want) in [
            (0u32, 0xa1u32),
            (9, 0xa1),
            (10, 0xa3),
            (34, 0xa3),
            (35, 0xa4),
            (79, 0xa4),
        ] {
            let mut e = pick_rig(random);
            let global = e.mem.alloc(0x28);
            e.mem.set_f32(global + 0x24, 25.9);
            let entries = [
                weather_entry(&mut e, 0xa1, 10, 0),
                weather_entry(&mut e, 0xa2, 0, 0),
                weather_entry(&mut e, 0xa3, 99, global),
                weather_entry(&mut e, 0xa4, 5, 0),
            ];
            let list = make_list(&mut e, &entries);
            assert_eq!(
                e.call(0x0058_27d0, &args![list]).u32(),
                want,
                "random {random}"
            );
        }
    }

    #[test]
    fn fn_005827d0_answers_null_for_no_chance_and_skips_an_empty_node_in_the_sum() {
        let mut e = pick_rig(0);
        let entries = [weather_entry(&mut e, 0xa1, 0, 0)];
        let list = make_list(&mut e, &entries);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_27d0, &args![list]).u32(), 0);
        assert!(calls(&e, RANDOM_00487F50).is_empty());
        // The sum skips the empty second node and goes on to the third (005827f7 jumps to
        // 00582851): total 3 + 5 = 8, so the spans are [0, 2] and [3, 7]. Pick 2 is in the
        // first span, pick 5 in the third entry's.
        for (random, want) in [(2u32, 0xa1u32), (5, 0xa5), (13, 0xa5)] {
            let mut e = pick_rig(random);
            let first = weather_entry(&mut e, 0xa1, 3, 0);
            let third = weather_entry(&mut e, 0xa5, 5, 0);
            let list = make_list(&mut e, &[first, 0, third]);
            assert_eq!(
                e.call(0x0058_27d0, &args![list]).u32(),
                want,
                "random {random}"
            );
        }
        // The calls: in the sum, an entry's node is read three times and the empty node once,
        // each followed by its next (00726070); the lookup reads each node once and stops at
        // the third node's span.
        let mut e = pick_rig(5);
        let first = weather_entry(&mut e, 0xa1, 3, 0);
        let third = weather_entry(&mut e, 0xa5, 5, 0);
        let list = make_list(&mut e, &[first, 0, third]);
        let second_node = e.mem.u32(list + 4);
        let third_node = e.mem.u32(second_node + 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_27d0, &args![list]).u32(), 0xa5);
        assert_eq!(
            calls(&e, LIST_ITEM_006815C0),
            vec![
                vec![list],
                vec![list],
                vec![list],
                vec![second_node],
                vec![third_node],
                vec![third_node],
                vec![third_node],
                vec![list],
                vec![second_node],
                vec![third_node],
            ]
        );
        assert_eq!(
            calls(&e, NODE_FOLLOWING_00726070),
            vec![
                vec![list],
                vec![second_node],
                vec![third_node],
                vec![list],
                vec![second_node],
            ]
        );
        // A global whose value changes between the two walks leaves the number outside
        // every span: the last entry read comes back.
        let mut e = pick_rig(5);
        let global = e.mem.alloc(0x28);
        e.mem.set_f32(global + 0x24, 6.0);
        let first = weather_entry(&mut e, 0xa1, 0, global);
        let last = weather_entry(&mut e, 0xa8, 0, 0);
        let list = make_list(&mut e, &[first, last]);
        let mut seen = 0;
        e.register_double(GLOBAL_VALUE_00526AC0, move |_, _| {
            seen += 1;
            Ret {
                st0: if seen == 1 { 6.0 } else { 2.0 },
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0058_27d0, &args![list]).u32(), 0xa8);
    }

    #[test]
    fn fn_005827d0_truncation_and_the_indefinite() {
        assert_eq!(truncate_to_qword(25.9), 25);
        assert_eq!(truncate_to_qword(-2.5), (-2i64) as u64);
        assert_eq!(truncate_to_qword(f64::NAN), 0x8000_0000_0000_0000);
        assert_eq!(truncate_to_qword(1e19), 0x8000_0000_0000_0000);
    }

    const FILE: u32 = 0x0e00_0000;
    const FORM: u32 = 0x0e10_0000;

    /// Two read entries: (weather id 0x10, chance 7, global id 0x20), (0x30, 9, 0).
    fn load_rig(read_ok: bool, swap: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let list = e.mem.alloc(8);
        let array = e.mem.alloc(0x18);
        let mut blocks = vec![array];
        e.register_double(ALLOCATE, move |e, a| {
            if a[0] == 0xc {
                e.mem.alloc(0xc).into_ret()
            } else {
                blocks.pop().unwrap_or(0).into_ret()
            }
        });
        e.register(CHUNK_SIZE, |_, _| 0x1fu32.into_ret());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            for (i, word) in [0x10u32, 7, 0x20, 0x30, 9, 0].iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            read_ok.into_ret()
        });
        e.register(FORM_ID_0084E3A0, |_, _| 0x4321u32.into_ret());
        e.register(LOG, |_, _| Ret::default());
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.register(SWAP_THREE_WORDS, |_, _| Ret::default());
        e.register(ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(NEW_ENTRY_00824340, |_, a| a[0].into_ret());
        // The new head value becomes the list item.
        e.register(LIST_ADD_HEAD_005AE3D0, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        // Id 0x10 and 0x20 have forms; 0x30 and 0 do not.
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] == 0x10 || a[0] == 0x20 {
                (0x0d00_0000 + a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(LIST_SORT_0083FD60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, list, array)
    }

    #[test]
    fn fn_00582940_builds_the_entries() {
        let (mut e, list, array) = load_rig(true, true);
        e.call(0x0058_2940, &args![list, FILE, FORM]);
        assert_eq!(calls(&e, ALLOCATE)[0], vec![0x18]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR),
            vec![vec![array, 0xc, 2, 0x0058_2570]]
        );
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, array, 0x18]]
        );
        assert_eq!(
            calls(&e, SWAP_THREE_WORDS),
            vec![vec![array], vec![array + 0xc]]
        );
        assert_eq!(
            calls(&e, ADD_COMPILE_INDEX),
            vec![
                vec![array, FILE],
                vec![array + 8, FILE],
                vec![array + 0xc, FILE],
                vec![array + 0x14, FILE]
            ]
        );
        let casts = calls(&e, DYNAMIC_CAST);
        assert_eq!(
            casts[0],
            vec![0x0d00_0010, 0, TYPE_TES_FORM, TYPE_TES_WEATHER, 0]
        );
        assert_eq!(
            casts[1],
            vec![0x0d00_0020, 0, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0]
        );
        // The last entry is the head: weather none (reported), chance 9, global none.
        let head = e.mem.u32(list);
        assert_eq!(e.mem.u32(head), 0);
        assert_eq!(e.mem.u32(head + 4), 9);
        assert_eq!(e.mem.u32(head + 8), 0);
        assert_eq!(calls(&e, LOG), vec![vec![MESSAGE_NOT_A_WEATHER, 0x30]]);
        let first = calls(&e, NEW_ENTRY_00824340)[0][0];
        assert_eq!(e.mem.u32(first), 0x0d00_0010);
        assert_eq!(e.mem.u32(first + 4), 7);
        assert_eq!(e.mem.u32(first + 8), 0x0d00_0020);
        assert_eq!(
            calls(&e, LIST_SORT_0083FD60),
            vec![vec![list, COMPARE_00582BE0]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE).last().unwrap(), &vec![array]);
    }

    #[test]
    fn fn_00582940_reports_a_failed_read_and_skips_bad_arguments() {
        let (mut e, list, _) = load_rig(false, false);
        e.call(0x0058_2940, &args![list, FILE, FORM]);
        assert_eq!(calls(&e, LOG)[0], vec![MESSAGE_CHUNK_ERROR, 0x4321]);
        assert!(calls(&e, SWAP_THREE_WORDS).is_empty());
        let (mut e, list, _) = load_rig(true, false);
        e.call(0x0058_2940, &args![list, FILE, 0u32]);
        e.call(0x0058_2940, &args![list, 0u32, FORM]);
        e.call(0x0058_2940, &args![0u32, FILE, FORM]);
        assert!(calls(&e, CHUNK_SIZE).is_empty());
        e.register(CHUNK_SIZE, |_, _| 0xbu32.into_ret());
        e.call(0x0058_2940, &args![list, FILE, FORM]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_00582940_with_failed_allocations_reads_into_null() {
        let (mut e, list, _) = load_rig(true, false);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(GET_CHUNK_DATA_SIZED, |_, _| true.into_ret());
        // The entries at null: map the first page so the reads see zeros.
        e.map(0, 0x100);
        e.register(LIST_ADD_HEAD_005AE3D0, |_, _| Ret::default());
        let head = e.mem.alloc(0xc);
        e.mem.set_u32(list, head);
        e.call(0x0058_2940, &args![list, FILE, FORM]);
        assert!(calls(&e, VECTOR_CONSTRUCTOR).is_empty());
        assert!(calls(&e, NEW_ENTRY_00824340).is_empty());
        assert_eq!(calls(&e, GET_CHUNK_DATA_SIZED), vec![vec![FILE, 0, 0x18]]);
        assert_eq!(calls(&e, ADD_COMPILE_INDEX)[2], vec![0xc, FILE]);
        assert_eq!(calls(&e, OPERATOR_DELETE).last().unwrap(), &vec![0]);
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut n = 0u32;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    n += 1;
                }
                node = e.mem.u32(node + 4);
            }
            n.into_ret()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FORM_ID_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        e.register(SWAP_THREE_WORDS, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL_00470470, |_, _| Ret::default());
        e.register(NEW_ENTRY_00824340, |_, a| a[0].into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            u32::from(e.mem.bytes(a[0], a[2]) != e.mem.bytes(a[1], a[2])).into_ret()
        });
        e
    }

    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    fn entry(e: &mut Engine, words: [u32; 3]) -> u32 {
        let entry = e.mem.alloc(0xc);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(entry + 4 * i as u32, *w);
        }
        entry
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let f = e.mem.alloc(0x10);
        e.mem.set_u32(f + 0xc, id);
        f
    }

    #[test]
    fn save_writes_ids_chances_and_globals() {
        let mut e = engine();
        let swap = std::rc::Rc::new(std::cell::Cell::new(false));
        let s = swap.clone();
        e.register_double(SWAPS_BYTE_ORDER, move |_, _| u32::from(s.get()).into_ret());
        let w1 = form(&mut e, 0x101);
        let w2 = form(&mut e, 0x102);
        let g = form(&mut e, 0x201);
        let a = entry(&mut e, [w1, 30, g]);
        let b = entry(&mut e, [w2, 70, 0]);
        let empty = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0058_2380, &[empty, 0x4d414e57]);
        assert!(e.call_log.take().unwrap().iter().all(|c| c.0 != ALLOCATE));
        let this = list(&mut e, &[a, b]);
        let saved = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let s2 = saved.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *s2.borrow_mut() = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_2380, &[this, 0x4d414e57]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, ALLOCATE), vec![vec![0x18]]);
        let array = only(&log, VECTOR_CONSTRUCTOR)[0][0];
        assert_eq!(
            only(&log, VECTOR_CONSTRUCTOR),
            vec![vec![array, 0xc, 2, 0x0058_2570]]
        );
        assert_eq!(*saved.borrow(), vec![0x101, 30, 0x201, 0x102, 70, 0]);
        assert_eq!(only(&log, ADD_CHUNK_DATA)[0][0], 0x4d414e57);
        assert!(only(&log, SWAP_THREE_WORDS).is_empty());
        assert_eq!(only(&log, OPERATOR_DELETE), vec![vec![array]]);
        swap.set(true);
        e.call_log = Some(vec![]);
        e.call(0x0058_2380, &[this, 0x4d414e57]);
        let log = e.call_log.take().unwrap();
        let array = only(&log, VECTOR_CONSTRUCTOR)[0][0];
        assert_eq!(
            only(&log, SWAP_THREE_WORDS),
            vec![
                vec![array],
                vec![array + 0xc],
                vec![array],
                vec![array + 0xc]
            ]
        );
        assert_eq!(entry_array_size(0x1555_5556), u32::MAX);
    }

    #[test]
    fn copy_frees_unless_kept_and_copies_each_entry() {
        let mut e = engine();
        let a = entry(&mut e, [1, 2, 3]);
        let source = list(&mut e, &[a]);
        let this = list(&mut e, &[]);
        let added = std::rc::Rc::new(std::cell::Cell::new(0));
        let s = added.clone();
        e.register_double(LIST_ADD_00905820, move |e, a| {
            s.set(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_25a0, &[this, 0, 0]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0058_25a0, &[this, source, 1]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, LIST_REMOVE_ALL_00470470).is_empty());
        let copy = added.get();
        assert_eq!(e.mem.bytes(copy, 0xc), e.mem.bytes(a, 0xc));
        let empty = list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0058_25a0, &[this, empty, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, LIST_REMOVE_ALL_00470470), vec![vec![this]]);
        assert!(only(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn compare_matches_entries_in_any_order() {
        let mut e = engine();
        let a = entry(&mut e, [1, 2, 3]);
        let b = entry(&mut e, [4, 5, 6]);
        let a2 = entry(&mut e, [1, 2, 3]);
        let b2 = entry(&mut e, [4, 5, 6]);
        let c = entry(&mut e, [4, 5, 7]);
        let this = list(&mut e, &[a, b]);
        let same = list(&mut e, &[b2, a2]);
        let other = list(&mut e, &[a2, c]);
        let short = list(&mut e, &[a2]);
        let empty1 = list(&mut e, &[]);
        let empty2 = list(&mut e, &[]);
        assert!(e.call(0x0058_2690, &[this, 0]).bool());
        assert!(!e.call(0x0058_2690, &[empty1, empty2]).bool());
        assert!(e.call(0x0058_2690, &[this, short]).bool());
        assert!(!e.call(0x0058_2690, &[this, same]).bool());
        assert!(e.call(0x0058_2690, &[this, other]).bool());
    }
}

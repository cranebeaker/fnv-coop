//! `NiMain/bstexturepalette.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The texture palette keeps every loaded texture in seven hash tables
//! (`BSTexMap`, one per size class, 0xb28 bytes each, 0x2c9 buckets) and a
//! second table (`BSEntryMap`, 0xfa8 bytes, 0x3e9 buckets) from a file entry
//! to the palette item of its texture. Both live behind two globals and are
//! guarded by one spin lock (`AddCrit`).
//!
//! The roles of the globals below come from the Xbox PDB's list of
//! `BSTexturePalette` statics (`pTexMapA`, `pEntryMap`, `AddCrit`,
//! `iMaxTexturesToPurgeS`, `bPartiallyPurgedS`); which address is which is
//! read from how the code uses it, not from a symbol, so it is not confirmed
//! beyond that.
//!
//! The spin lock is entered through `0040fbf0` and left by the inlined
//! `count -= 1; if count == 0 { owner = 0 }` the compiler put after each
//! use. The compiler's exception-unwinding frames are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// Global holding the array of seven `BSTexMap` (`pTexMapA`, by use).
const TEX_MAP_ARRAY: u32 = 0x011f_4464;
/// Global holding the `BSEntryMap` (`pEntryMap`, by use); null until the
/// palette is initialized, and every public function does nothing then.
const ENTRY_MAP: u32 = 0x011f_4468;
/// The palette's spin lock (`AddCrit`, by use): owner thread id at +0,
/// recursion count at +4.
const ADD_CRIT: u32 = 0x011f_4480;
/// Byte cleared by the initialization and by the purge (`bPartiallyPurgedS`,
/// by use).
const PARTIALLY_PURGED: u32 = 0x011f_4461;
/// Signed limit of textures one purge may remove (`iMaxTexturesToPurgeS`,
/// by use).
const MAX_TEXTURES_TO_PURGE: u32 = 0x011a_95fc;
/// The `MemoryManager` singleton the pooled allocations go through.
const HEAP: u32 = 0x011f_6238;

/// `MemoryManager::Allocate`.
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate`.
const DEALLOCATE: u32 = 0x00aa_4060;
/// `tolower` (LIBCMT), cdecl.
const TOLOWER: u32 = 0x00ec_67aa;
/// `memset` (LIBCMT), cdecl: destination, value, byte count.
const MEMSET: u32 = 0x00ec_61c0;
/// `FindTexturePathOffset(path, 1)`: offset of the part of a texture path
/// that is the key.
const FIND_TEXTURE_PATH_OFFSET: u32 = 0x00af_4170;
/// `BSSpinLock::Enter`-style entry (`0040fbf0`): lock object, one word that
/// is 0 here.
const SPIN_LOCK_ENTER: u32 = 0x0040_fbf0;
/// Vector constructor helper (`00401050`): block, element size, count,
/// constructor.
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// Per-element constructor of a `BSTexMap` (`00a61590`), passed to the
/// vector constructor; not part of this unit's open functions.
const TEX_MAP_CONSTRUCTOR: u32 = 0x00a6_1590;
/// `00c5d560(texture)`: the file entry of a texture, or 0.
const FILE_ENTRY_OF_TEXTURE: u32 = 0x00c5_d560;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Number of size classes (`BSTexMap`s) in the array.
const MAP_COUNT: u32 = 7;
/// Bytes of one `BSTexMap`.
const MAP_SIZE: u32 = 0xb28;
/// Buckets of a `BSTexMap` (`iTableSize`).
const MAP_BUCKETS: u32 = 0x2c9;
/// Buckets of the `BSEntryMap`.
const ENTRY_BUCKETS: u32 = 0x3e9;

layout! {
    /// `BSTexMapItem` (Xbox PDB), 0x10 bytes: one texture in a size class.
    pub struct BSTexMapItem: 0x10 {
        /// `spTexture` (Xbox PDB): the texture, one reference held.
        0x00 spTexture: u32,
        /// `pFileEntry` (Xbox PDB): key in the entry map, 0 when absent.
        0x04 pFileEntry: u32,
        /// `pNext` (Xbox PDB): next item of the bucket.
        0x08 pNext: u32,
        /// `sPriority` (Xbox PDB): the size class the item is stored in.
        0x0C sPriority: u16,
        /// `sNameBaseOffset` (Xbox PDB): offset into the texture's name at
        /// which the key starts.
        0x0E sNameBaseOffset: i16,
    }

    /// `BSTexMap` (Xbox PDB): bucket heads at +0 (`pItemA`), item count at
    /// +0xb24.
    pub struct BSTexMap: 0xb28 {
        /// `iCount` (Xbox PDB).
        0xB24 iCount: u32,
    }

    /// `BSEntryMap` (Xbox PDB): bucket heads at +0 (`pItemA`), entry count
    /// at +0xfa4.
    pub struct BSEntryMap: 0xfa8 {
        /// `iCount` (Xbox PDB).
        0xFA4 iCount: u32,
    }

    /// `BSTexMapIterator` (Xbox PDB), 0xc bytes.
    pub struct BSTexMapIterator: 0xC {
        /// `pItem` (Xbox PDB).
        0x00 pItem: u32,
        /// `iIndex` (Xbox PDB): bucket of `pItem`.
        0x04 iIndex: u32,
        /// `iPriority` (Xbox PDB): size class being walked.
        0x08 iPriority: u32,
    }
}

/// `InterlockedDecrement` on a reference count at `object + 4`; the object's
/// virtual slot 1 (`DeleteThis`) runs when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

fn add_reference(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// `NiPointer<NiTexture>::operator=` on the holder at `holder`.
fn assign_texture(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old != value {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(holder, value);
        if value != 0 {
            add_reference(e, value);
        }
    }
}

fn heap_free(e: &mut Engine, block: u32) {
    e.call(DEALLOCATE, &args![HEAP, block]);
}

fn heap_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![HEAP, size]).u32()
}

fn tolower(e: &mut Engine, byte: u8) -> i32 {
    e.call(TOLOWER, &args![byte as i8 as i32]).i32()
}

/// Enters the palette's spin lock.
fn lock(e: &mut Engine) {
    e.call(SPIN_LOCK_ENTER, &args![ADD_CRIT, 0u32]);
}

/// The inlined leave: one recursion level less, owner cleared at zero.
fn unlock(e: &mut Engine) {
    let count = e.mem.u32(ADD_CRIT + 4).wrapping_sub(1);
    e.mem.set_u32(ADD_CRIT + 4, count);
    if count == 0 {
        e.mem.set_u32(ADD_CRIT, 0);
    }
}

/// The texture's name through its virtual slot `0x9c` (it returns the
/// address of the name pointer).
fn texture_name(e: &mut Engine, texture: u32) -> u32 {
    let holder = e.vcall(texture, 0x9c, &[]).u32();
    e.mem.u32(holder)
}

/// The key of a texture: its name past `FindTexturePathOffset`; also
/// returns that offset.
fn texture_key(e: &mut Engine, texture: u32) -> (u32, u32) {
    let name = texture_name(e, texture);
    let offset = e.call(FIND_TEXTURE_PATH_OFFSET, &args![name, 1u32]).u32();
    (offset, name.wrapping_add(offset))
}

/// The key stored for an item: the texture's name at the item's base offset.
fn item_key(e: &mut Engine, item: u32) -> u32 {
    let texture = e.get(Ptr::<BSTexMapItem>::new(item), BSTexMapItem::spTexture);
    let offset = e.get(
        Ptr::<BSTexMapItem>::new(item),
        BSTexMapItem::sNameBaseOffset,
    );
    texture_name(e, texture).wrapping_add(offset as i32 as u32)
}

fn tex_map(e: &Engine, priority: u32) -> u32 {
    e.mem
        .u32(TEX_MAP_ARRAY)
        .wrapping_add(priority.wrapping_mul(MAP_SIZE))
}

// Translated from 00a614d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Case-insensitive prefix comparison of a palette key (`wanted`) against a
/// stored key (`stored`): true when `wanted` ends first. A `.` in `wanted`
/// that is followed by exactly three more characters before the end of the
/// string ends the comparison with a match (the extension is ignored), but
/// `stored` must have a `.` at the same place. Unnamed in the engine map.
pub fn fn_00a614d0(e: &mut Engine, wanted: u32, stored: u32) -> bool {
    let mut wanted = wanted;
    let mut stored = stored;
    if e.mem.u8(wanted) == 0 {
        return true;
    }
    loop {
        let wanted_char = e.mem.u8(wanted);
        let wanted_lower = tolower(e, wanted_char);
        let stored_char = e.mem.u8(stored);
        let stored_lower = tolower(e, stored_char);
        wanted += 1;
        stored += 1;
        if wanted_lower == 0x2e {
            if stored_lower != 0x2e {
                return false;
            }
            if e.mem.u8(wanted + 3) == 0 {
                return true;
            }
        } else if wanted_lower != stored_lower {
            return false;
        }
        if e.mem.u8(wanted) == 0 {
            return true;
        }
    }
}

// Translated from 00a61530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexMap::KeyToHashIndex` (Xbox PDB): bucket of a key. `hash * 33 +
/// lowercase character` over the string, stopping at a `.` that has exactly
/// three characters after it, modulo the 0x2c9 buckets.
pub fn bs_tex_map_key_to_hash_index(e: &mut Engine, key: u32) -> u32 {
    let mut hash: u32 = 0;
    let mut at = key;
    while e.mem.u8(at) != 0 {
        let character = e.mem.u8(at);
        let lower = tolower(e, character);
        at += 1;
        if lower == 0x2e && e.mem.u8(at + 3) == 0 {
            break;
        }
        hash = hash.wrapping_mul(0x21).wrapping_add(lower as u32);
    }
    hash % MAP_BUCKETS
}

// Translated from 00a615c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the `BSEntryMap`: clears each item's file entry, frees the
/// bucket nodes and zeroes the count. Unnamed in the engine map.
pub fn fn_00a615c0(e: &mut Engine, this: Ptr<BSEntryMap>) {
    for index in 0..ENTRY_BUCKETS {
        let bucket = this.addr() + index * 4;
        let mut node = e.mem.u32(bucket);
        while node != 0 {
            let item = e.mem.u32(node);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(item + 4, 0);
            heap_free(e, node);
            node = next;
        }
        e.mem.set_u32(bucket, 0);
    }
    e.set(this, BSEntryMap::iCount, 0);
}

// Translated from 00a61610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` to the `BSEntryMap` under `file_entry`: stores the key in the
/// item and pushes a two-word node on the bucket `(file_entry >> 4) % 0x3e9`.
/// When the node cannot be allocated the bucket is cleared (as the game
/// does) and the count still grows. Unnamed in the engine map.
pub fn fn_00a61610(e: &mut Engine, this: Ptr<BSEntryMap>, file_entry: u32, item: u32) {
    let bucket = this.addr() + (file_entry >> 4) % ENTRY_BUCKETS * 4;
    e.mem.set_u32(item + 4, file_entry);
    let node = heap_allocate(e, 8);
    if node != 0 {
        let head = e.mem.u32(bucket);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, head);
        e.mem.set_u32(bucket, node);
    } else {
        e.mem.set_u32(bucket, 0);
    }
    let count = e.get(this, BSEntryMap::iCount);
    e.set(this, BSEntryMap::iCount, count.wrapping_add(1));
}

// Translated from 00a61670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSEntryMap::Remove` (Xbox PDB): unlinks the node whose item has
/// `file_entry`, frees it, clears the item's file entry and returns the
/// item; 0 when there is none.
pub fn bs_entry_map_remove(e: &mut Engine, this: Ptr<BSEntryMap>, file_entry: u32) -> u32 {
    let bucket = this.addr() + (file_entry >> 4) % ENTRY_BUCKETS * 4;
    let mut previous = 0;
    let mut node = e.mem.u32(bucket);
    while node != 0 {
        let item = e.mem.u32(node);
        if e.mem.u32(item + 4) == file_entry {
            let next = e.mem.u32(node + 4);
            if previous == 0 {
                e.mem.set_u32(bucket, next);
            } else {
                e.mem.set_u32(previous + 4, next);
            }
            let count = e.get(this, BSEntryMap::iCount);
            e.set(this, BSEntryMap::iCount, count.wrapping_sub(1));
            heap_free(e, node);
            e.mem.set_u32(item + 4, 0);
            return item;
        }
        previous = node;
        node = e.mem.u32(node + 4);
    }
    0
}

// Translated from 00a616e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts an iteration over all textures of the seven size classes, from
/// the largest class down: the iterator gets the first item of the first
/// non-empty class (item, bucket, class); returns whether there is one.
/// When a class has items its bucket and class are written, otherwise the
/// iterator's item and bucket are zero. Unnamed in the engine map.
pub fn fn_00a616e0(e: &mut Engine, iterator: Ptr<BSTexMapIterator>) -> bool {
    let mut offset = MAP_COUNT * MAP_SIZE;
    loop {
        offset -= MAP_SIZE;
        let map = e.mem.u32(TEX_MAP_ARRAY) + offset;
        e.set(iterator, BSTexMapIterator::iIndex, 0);
        e.set(iterator, BSTexMapIterator::pItem, 0);
        if e.get(Ptr::<BSTexMap>::new(map), BSTexMap::iCount) != 0 {
            for index in 0..MAP_BUCKETS {
                let item = e.mem.u32(map + index * 4);
                if item != 0 {
                    e.set(iterator, BSTexMapIterator::iIndex, index);
                    e.set(iterator, BSTexMapIterator::pItem, item);
                    let priority = e.get(Ptr::<BSTexMapItem>::new(item), BSTexMapItem::sPriority);
                    e.set(iterator, BSTexMapIterator::iPriority, priority as u32);
                    break;
                }
            }
        }
        let found = e.get(iterator, BSTexMapIterator::pItem) != 0;
        if found || offset == 0 {
            return found;
        }
    }
}

// Translated from 00a61750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size class of a texture from the larger of its two dimensions (virtual
/// slots `0x94` and `0x98`): 4 for 1024 and up, 3 for 512, 2 for 256, 1 for
/// 128, otherwise 0. Unnamed in the engine map.
pub fn fn_00a61750(e: &mut Engine, texture: u32) -> u32 {
    let second = e.vcall(texture, 0x98, &[]).u32();
    let first = e.vcall(texture, 0x94, &[]).u32();
    let largest = first.max(second);
    if largest >= 0x400 {
        4
    } else if largest >= 0x200 {
        3
    } else if largest >= 0x100 {
        2
    } else {
        (largest >= 0x80) as u32
    }
}

// Translated from 00a617c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the item of `texture` in a `BSTexMap` or adds a new one at the
/// head of its bucket (taking a reference on the texture) and returns it.
/// A new item gets the path offset as its name base offset. If the item
/// cannot be allocated the game writes through a null item; that write is
/// kept. Unnamed in the engine map.
pub fn fn_00a617c0(e: &mut Engine, this: Ptr<BSTexMap>, texture: u32) -> u32 {
    let (offset, key) = texture_key(e, texture);
    let bucket = bs_tex_map_key_to_hash_index(e, key);
    let mut item = e.mem.u32(this.addr() + bucket * 4);
    while item != 0 {
        let stored = item_key(e, item);
        if fn_00a614d0(e, key, stored) {
            return item;
        }
        item = e.mem.u32(item + 8);
    }
    item = heap_allocate(e, 0x10);
    if item != 0 {
        let head = e.mem.u32(this.addr() + bucket * 4);
        e.mem.set_u32(item, texture);
        add_reference(e, texture);
        e.mem.set_u32(item + 4, 0);
        e.mem.set_u32(item + 8, head);
        e.mem.set_u16(item + 0xc, 0);
        e.mem.set_u16(item + 0xe, 0);
    }
    e.mem.set_u16(item + 0xe, offset as u16);
    e.mem.set_u32(this.addr() + bucket * 4, item);
    let count = e.get(this, BSTexMap::iCount);
    e.set(this, BSTexMap::iCount, count.wrapping_add(1));
    item
}

// Translated from 00a61890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexMap::Remove` (Xbox PDB): unlinks `item` from its bucket (found by
/// hashing its texture's key), releases the texture, frees the item and
/// decrements the count. Nothing happens when the item is not in the
/// bucket.
pub fn bs_tex_map_remove(e: &mut Engine, this: Ptr<BSTexMap>, item: u32) {
    let texture = e.mem.u32(item);
    let (_, key) = texture_key(e, texture);
    let bucket = this.addr() + bs_tex_map_key_to_hash_index(e, key) * 4;
    let mut current = e.mem.u32(bucket);
    if current == 0 {
        return;
    }
    let mut previous = 0;
    while current != item {
        previous = current;
        current = e.mem.u32(current + 8);
        if current == 0 {
            return;
        }
    }
    let next = e.mem.u32(current + 8);
    if previous == 0 {
        e.mem.set_u32(bucket, next);
    } else {
        e.mem.set_u32(previous + 8, next);
    }
    let held = e.mem.u32(current);
    if held != 0 {
        release(e, held);
    }
    heap_free(e, current);
    let count = e.get(this, BSTexMap::iCount);
    e.set(this, BSTexMap::iCount, count.wrapping_sub(1));
}

// Translated from 00a61920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `key` up in one bucket of a `BSTexMap` (items compared with
/// [`fn_00a614d0`]); on a match stores the item's texture in the
/// `NiPointer` at `out` (reference counts adjusted) and returns true.
/// Unnamed in the engine map.
pub fn fn_00a61920(e: &mut Engine, this: Ptr<BSTexMap>, bucket: u32, key: u32, out: u32) -> bool {
    let mut item = e.mem.u32(this.addr() + bucket * 4);
    while item != 0 {
        let stored = item_key(e, item);
        if fn_00a614d0(e, key, stored) {
            let texture = e.mem.u32(item);
            assign_texture(e, out, texture);
            return true;
        }
        item = e.mem.u32(item + 8);
    }
    false
}

// Translated from 00a619b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Steps a `BSTexMap` iterator: stores the current item's texture in the
/// `NiPointer` at `out`, moves to the next item (and the next non-empty
/// bucket of this map) and returns whether the iterator still points at an
/// item. Does nothing but report when it is already at the end. Unnamed in
/// the engine map.
pub fn fn_00a619b0(
    e: &mut Engine,
    this: Ptr<BSTexMap>,
    iterator: Ptr<BSTexMapIterator>,
    out: u32,
) -> bool {
    let current = e.get(iterator, BSTexMapIterator::pItem);
    if current != 0 {
        // 00a619cb: the inlined `NiPointer` = `NiPointer` takes the item's texture from
        // the item again after the old texture's release (00a619ed), which can run its
        // destructor, and counts up the value it stored.
        let old = e.mem.u32(out);
        if old != e.mem.u32(current) {
            if old != 0 {
                release(e, old);
            }
            let texture = e.mem.u32(current);
            e.mem.set_u32(out, texture);
            if texture != 0 {
                add_reference(e, texture);
            }
        }
        // 00a61a00: the iterator's item is read again for its next link.
        let current = e.get(iterator, BSTexMapIterator::pItem);
        let next = e.mem.u32(current.wrapping_add(8));
        e.set(iterator, BSTexMapIterator::pItem, next);
        if next == 0 {
            let mut index = e.get(iterator, BSTexMapIterator::iIndex).wrapping_add(1);
            e.set(iterator, BSTexMapIterator::iIndex, index);
            if index < MAP_BUCKETS {
                loop {
                    let head = e.mem.u32(this.addr() + index * 4);
                    if head != 0 {
                        e.set(iterator, BSTexMapIterator::pItem, head);
                        break;
                    }
                    index += 1;
                    e.set(iterator, BSTexMapIterator::iIndex, index);
                    if index >= MAP_BUCKETS {
                        break;
                    }
                }
            }
        }
    }
    e.get(iterator, BSTexMapIterator::pItem) != 0
}

// Translated from 00a61a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `file_entry` up in one bucket of the `BSEntryMap`; on a match
/// stores the texture of the found item in the `NiPointer` at `out`
/// (reference counts adjusted) and returns true. Unnamed in the engine map.
pub fn fn_00a61a60(
    e: &mut Engine,
    this: Ptr<BSEntryMap>,
    bucket: u32,
    file_entry: u32,
    out: u32,
) -> bool {
    let mut node = e.mem.u32(this.addr() + bucket * 4);
    while node != 0 {
        let item = e.mem.u32(node);
        if e.mem.u32(item + 4) == file_entry {
            let texture = e.mem.u32(item);
            assign_texture(e, out, texture);
            return true;
        }
        node = e.mem.u32(node + 4);
    }
    false
}

// Translated from 00a61ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the palette: a zeroed `BSEntryMap` and seven `BSTexMap`s (an
/// array block with its element count 7 in front, built by the vector
/// constructor at `00401050`), with the TLS allocator-context word (+0x2b4)
/// set to 0xf during the allocations and restored. A failed allocation
/// leaves the matching global null. Unnamed in the engine map.
pub fn fn_00a61ad0(e: &mut Engine) {
    let tls = e.tls();
    let previous = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0xf);
    let entry_map = heap_allocate(e, 0xfa8);
    if entry_map != 0 {
        e.call(MEMSET, &args![entry_map, 0u32, 0xfa4u32]);
        e.mem.set_u32(entry_map + 0xfa4, 0);
        e.mem.set_u32(ENTRY_MAP, entry_map);
    } else {
        e.mem.set_u32(ENTRY_MAP, 0);
    }
    let block = heap_allocate(e, 0x4e1c);
    if block != 0 {
        e.mem.set_u32(block, MAP_COUNT);
        let maps = block + 4;
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![maps, MAP_SIZE, MAP_COUNT, TEX_MAP_CONSTRUCTOR],
        );
        e.mem.set_u32(TEX_MAP_ARRAY, maps);
        e.mem.set_u32(tls + 0x2b4, previous);
        e.mem.set_u8(PARTIALLY_PURGED, 0);
        return;
    }
    e.mem.set_u32(tls + 0x2b4, previous);
    e.mem.set_u32(TEX_MAP_ARRAY, 0);
    e.mem.set_u8(PARTIALLY_PURGED, 0);
}

// Translated from 00a61b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::GetTexture_ov2` (Xbox PDB): looks the texture path up
/// in the size classes from the largest down and stores the texture in the
/// `NiPointer` at `out` (the previous one is released first and `out` is
/// cleared); returns whether it was found. False and untouched when the
/// palette does not exist.
pub fn bs_texture_palette_get_texture_ov2(e: &mut Engine, path: u32, out: u32) -> bool {
    if e.mem.u32(ENTRY_MAP) == 0 {
        return false;
    }
    lock(e);
    let offset = e.call(FIND_TEXTURE_PATH_OFFSET, &args![path, 1u32]).u32();
    let key = path.wrapping_add(offset);
    let bucket = bs_tex_map_key_to_hash_index(e, key);
    let held = e.mem.u32(out);
    if held != 0 {
        release(e, held);
        e.mem.set_u32(out, 0);
    }
    let mut found = false;
    for priority in (0..MAP_COUNT).rev() {
        let map = tex_map(e, priority);
        found = fn_00a61920(e, Ptr::new(map), bucket, key, out);
        if found {
            break;
        }
    }
    unlock(e);
    found
}

// Translated from 00a61c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::SetTexture` (Xbox PDB): finds or adds the texture's
/// item in its size class ([`fn_00a61750`], [`fn_00a617c0`]), records the
/// class, and, if the item has no file entry yet, registers it in the entry
/// map under `file_entry` (or, when that is 0, under the file entry
/// `00c5d560` finds for the texture; nothing when that is 0 too).
pub fn bs_texture_palette_set_texture(e: &mut Engine, texture: u32, file_entry: u32) {
    if e.mem.u32(ENTRY_MAP) == 0 {
        return;
    }
    lock(e);
    let priority = fn_00a61750(e, texture);
    let map = tex_map(e, priority);
    let item = fn_00a617c0(e, Ptr::new(map), texture);
    let item_ptr = Ptr::<BSTexMapItem>::new(item);
    let had_entry = e.get(item_ptr, BSTexMapItem::pFileEntry) != 0;
    e.set(item_ptr, BSTexMapItem::sPriority, priority as u16);
    if !had_entry {
        let mut entry = file_entry;
        if entry == 0 {
            entry = e.call(FILE_ENTRY_OF_TEXTURE, &args![texture]).u32();
        }
        if entry != 0 {
            let entry_map = e.mem.u32(ENTRY_MAP);
            fn_00a61610(e, Ptr::new(entry_map), entry, item);
        }
    }
    unlock(e);
}

// Translated from 00a61cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::PurgeUnusedTextures` (Xbox PDB): walks every texture
/// of every size class (largest class first) with an iterator and removes,
/// from the entry map and from its size class, each texture whose only
/// references are the palette's own and the walker's (reference count 2),
/// until `iMaxTexturesToPurgeS` have been removed. Clears the
/// partially-purged flag first.
pub fn bs_texture_palette_purge_unused_textures(e: &mut Engine) {
    if e.mem.u32(ENTRY_MAP) == 0 {
        return;
    }
    lock(e);
    let mut removed: i32 = 0;
    e.mem.set_u8(PARTIALLY_PURGED, 0);
    // The exe's frame (`SUB ESP,0x1c`, one push, three more inside): the base after the first
    // push holds the iterator at +0x14 (00a61ced) and the answered texture at +0x8 (00a61d2c,
    // three pushes later); the whole frame is 0x2c bytes, the base 0xc above its bottom.
    e.with_stack(0x2c, |e, frame| {
        let base = frame.addr() + 0xc;
        let iterator_block = base + 0x14;
        let out = base + 0x8;
        let iterator = Ptr::<BSTexMapIterator>::new(iterator_block);
        if fn_00a616e0(e, iterator) {
            loop {
                let class = e.get(iterator, BSTexMapIterator::iPriority);
                let item = e.get(iterator, BSTexMapIterator::pItem);
                let map = e
                    .mem
                    .u32(TEX_MAP_ARRAY)
                    .wrapping_add(class.wrapping_mul(MAP_SIZE));
                let mut more = fn_00a619b0(e, Ptr::new(map), iterator, out);
                if !more {
                    // Move to the next lower class that has items.
                    loop {
                        let class_now = e.get(iterator, BSTexMapIterator::iPriority);
                        if class_now == 0 {
                            e.set(iterator, BSTexMapIterator::iPriority, u32::MAX);
                            break;
                        }
                        let lower = class_now - 1;
                        e.set(iterator, BSTexMapIterator::iPriority, lower);
                        let lower_map = e.mem.u32(TEX_MAP_ARRAY) + lower * MAP_SIZE;
                        e.set(iterator, BSTexMapIterator::iIndex, 0);
                        e.set(iterator, BSTexMapIterator::pItem, 0);
                        if e.get(Ptr::<BSTexMap>::new(lower_map), BSTexMap::iCount) != 0 {
                            for index in 0..MAP_BUCKETS {
                                let head = e.mem.u32(lower_map + index * 4);
                                if head != 0 {
                                    let head_class = e.get(
                                        Ptr::<BSTexMapItem>::new(head),
                                        BSTexMapItem::sPriority,
                                    );
                                    e.set(iterator, BSTexMapIterator::pItem, head);
                                    e.set(iterator, BSTexMapIterator::iPriority, head_class as u32);
                                    e.set(iterator, BSTexMapIterator::iIndex, index);
                                    break;
                                }
                            }
                        }
                        more = e.get(iterator, BSTexMapIterator::pItem) != 0;
                        if more {
                            break;
                        }
                    }
                }
                let texture = e.mem.u32(out);
                if texture != 0 && e.mem.u32(texture + 4) == 2 {
                    let file_entry = e.mem.u32(item + 4);
                    if file_entry != 0 {
                        let entry_map = e.mem.u32(ENTRY_MAP);
                        bs_entry_map_remove(e, Ptr::new(entry_map), file_entry);
                    }
                    bs_tex_map_remove(e, Ptr::new(map), item);
                    removed += 1;
                }
                if !(more && removed < e.global::<i32>(MAX_TEXTURES_TO_PURGE)) {
                    break;
                }
            }
            let texture = e.mem.u32(out);
            if texture != 0 {
                release(e, texture);
            }
        }
    });
    unlock(e);
}

// Translated from 00a61e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::GetItemFor` (Xbox PDB): the palette item whose texture
/// is `texture`, searched in the bucket of its key in each of the seven
/// size classes (smallest class first); 0 when there is none.
pub fn bs_texture_palette_get_item_for(e: &mut Engine, texture: u32) -> u32 {
    let (_, key) = texture_key(e, texture);
    let bucket = bs_tex_map_key_to_hash_index(e, key);
    for priority in 0..MAP_COUNT {
        let map = tex_map(e, priority);
        let mut item = e.mem.u32(map + bucket * 4);
        while item != 0 {
            if e.mem.u32(item) == texture {
                return item;
            }
            item = e.mem.u32(item + 8);
        }
    }
    0
}

// Translated from 00a61ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a `BSTexMap`: releases every item's texture, frees the items,
/// clears the buckets and the count. Unnamed in the engine map.
pub fn fn_00a61ec0(e: &mut Engine, this: Ptr<BSTexMap>) {
    for index in 0..MAP_BUCKETS {
        let bucket = this.addr() + index * 4;
        let mut item = e.mem.u32(bucket);
        while item != 0 {
            let texture = e.mem.u32(item);
            let next = e.mem.u32(item + 8);
            if texture != 0 {
                release(e, texture);
            }
            heap_free(e, item);
            item = next;
        }
        e.mem.set_u32(bucket, 0);
    }
    e.set(this, BSTexMap::iCount, 0);
}

// Translated from 00a61f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::RemoveTexture` (Xbox PDB): removes `texture`'s item
/// from the entry map (if it has a file entry) and from its size class;
/// returns whether the texture was in the palette.
pub fn bs_texture_palette_remove_texture(e: &mut Engine, texture: u32) -> bool {
    if e.mem.u32(ENTRY_MAP) == 0 {
        return false;
    }
    lock(e);
    let item = bs_texture_palette_get_item_for(e, texture);
    let mut removed = false;
    if item != 0 {
        let file_entry = e.mem.u32(item + 4);
        if file_entry != 0 {
            let entry_map = e.mem.u32(ENTRY_MAP);
            bs_entry_map_remove(e, Ptr::new(entry_map), file_entry);
        }
        let priority = e.get(Ptr::<BSTexMapItem>::new(item), BSTexMapItem::sPriority);
        let map = tex_map(e, priority as u32);
        bs_tex_map_remove(e, Ptr::new(map), item);
        removed = true;
    }
    unlock(e);
    removed
}

// Translated from 00a61fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTexturePalette::RemoveTexture_ov2` (Xbox PDB): removes the texture
/// with this path: looks it up with [`bs_texture_palette_get_texture_ov2`]
/// and removes it with [`bs_texture_palette_remove_texture`]; the local
/// reference taken by the lookup is released at the end.
pub fn bs_texture_palette_remove_texture_ov2(e: &mut Engine, path: u32) -> bool {
    if e.mem.u32(ENTRY_MAP) == 0 {
        return false;
    }
    // The exe's frame (`PUSH ECX` and two pushes): 0xc bytes, the answered texture in the `ECX`
    // slot at +0x8 (00a61fc5, 00a61fd6).
    e.with_stack(0xc, |e, frame| {
        let out = frame.addr() + 8;
        lock(e);
        let mut removed = false;
        if bs_texture_palette_get_texture_ov2(e, path, out) {
            let texture = e.mem.u32(out);
            removed = bs_texture_palette_remove_texture(e, texture);
        }
        unlock(e);
        let texture = e.mem.u32(out);
        if texture != 0 {
            release(e, texture);
        }
        removed
    })
}

// Translated from 00a61590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `BSTexMap` constructor the vector constructor runs per size class
/// (Ghidra made no function of it; disassembled from its bytes): zeroes the
/// first 0xB24 bytes (`memset`) and the word at `+0xB24`, returns `this`.
pub fn fn_00a61590(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMSET, &args![this, 0u32, 0xb24u32]);
    e.mem.set_u32(this.addr().wrapping_add(0xb24), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_1590, fn_00a61590(Ptr) -> Ptr),
        entry!(0x00a614d0, fn_00a614d0(u32, u32) -> bool),
        entry!(0x00a61530, bs_tex_map_key_to_hash_index(u32) -> u32),
        entry!(0x00a615c0, fn_00a615c0(Ptr<BSEntryMap>)),
        entry!(0x00a61610, fn_00a61610(Ptr<BSEntryMap>, u32, u32)),
        entry!(0x00a61670, bs_entry_map_remove(Ptr<BSEntryMap>, u32) -> u32),
        entry!(0x00a616e0, fn_00a616e0(Ptr<BSTexMapIterator>) -> bool),
        entry!(0x00a61750, fn_00a61750(u32) -> u32),
        entry!(0x00a617c0, fn_00a617c0(Ptr<BSTexMap>, u32) -> u32),
        entry!(0x00a61890, bs_tex_map_remove(Ptr<BSTexMap>, u32)),
        entry!(
            0x00a61920,
            fn_00a61920(Ptr<BSTexMap>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00a619b0,
            fn_00a619b0(Ptr<BSTexMap>, Ptr<BSTexMapIterator>, u32) -> bool
        ),
        entry!(
            0x00a61a60,
            fn_00a61a60(Ptr<BSEntryMap>, u32, u32, u32) -> bool
        ),
        entry!(0x00a61ad0, fn_00a61ad0()),
        entry!(0x00a61b90, bs_texture_palette_get_texture_ov2(u32, u32) -> bool),
        entry!(0x00a61c50, bs_texture_palette_set_texture(u32, u32)),
        entry!(0x00a61cd0, bs_texture_palette_purge_unused_textures()),
        entry!(0x00a61e60, bs_texture_palette_get_item_for(u32) -> u32),
        entry!(0x00a61ec0, fn_00a61ec0(Ptr<BSTexMap>)),
        entry!(0x00a61f30, bs_texture_palette_remove_texture(u32) -> bool),
        entry!(0x00a61fb0, bs_texture_palette_remove_texture_ov2(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const TEXTURE_VTABLE: u32 = 0x0300_0000;
    const DOUBLE_DELETE: u32 = 0x0700_0001;
    const DOUBLE_WIDTH: u32 = 0x0700_0002;
    const DOUBLE_HEIGHT: u32 = 0x0700_0003;
    const DOUBLE_NAME: u32 = 0x0700_0004;

    /// An engine with the platform imports, test doubles for every callee
    /// outside this file, and fake textures (name pointer at +0x10, width
    /// at +0x14, height at +0x18).
    struct Fixture {
        e: Engine,
        deleted: Rc<RefCell<Vec<u32>>>,
        file_entry: Rc<Cell<u32>>,
    }

    impl Fixture {
        fn new() -> Fixture {
            let mut e = Engine::new();
            let imports = [
                (INTERLOCKED_INCREMENT, "InterlockedIncrement"),
                (INTERLOCKED_DECREMENT, "InterlockedDecrement"),
            ]
            .map(|(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            });
            assert_eq!(e.register_imports(&imports), 2);
            e.map(0x011f_4000, 0x2000);
            e.map(0x011a_9000, 0x1000);
            let mut slots = vec![0u32; 0x28];
            slots[1] = DOUBLE_DELETE;
            slots[0x94 / 4] = DOUBLE_WIDTH;
            slots[0x98 / 4] = DOUBLE_HEIGHT;
            slots[0x9c / 4] = DOUBLE_NAME;
            e.put_vtable(TEXTURE_VTABLE, &slots);
            let deleted = Rc::new(RefCell::new(Vec::new()));
            let log = deleted.clone();
            e.register_double(DOUBLE_DELETE, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            e.register(DOUBLE_WIDTH, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
            e.register(DOUBLE_HEIGHT, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
            e.register(DOUBLE_NAME, |_, a| (a[0] + 0x10).into_ret());
            e.register(TOLOWER, |_, a| (a[0] as u8).to_ascii_lowercase().into_ret());
            // The key of a path starts after its last backslash.
            e.register(FIND_TEXTURE_PATH_OFFSET, |e, a| {
                let text = e.mem.cstr(a[0]);
                let start = text.iter().rposition(|c| *c == b'\\').map_or(0, |i| i + 1);
                (start as u32).into_ret()
            });
            e.register(SPIN_LOCK_ENTER, |e, a| {
                assert_eq!(a, &[ADD_CRIT, 0]);
                e.mem.set_u32(ADD_CRIT, 1);
                let count = e.mem.u32(ADD_CRIT + 4);
                e.mem.set_u32(ADD_CRIT + 4, count + 1);
                Ret::default()
            });
            e.register(MEMSET, |e, a| {
                e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
                a[0].into_ret()
            });
            e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
            let file_entry = Rc::new(Cell::new(0));
            let entry = file_entry.clone();
            e.register_double(FILE_ENTRY_OF_TEXTURE, move |_, _| entry.get().into_ret());
            Fixture {
                e,
                deleted,
                file_entry,
            }
        }

        fn string(&mut self, text: &str) -> u32 {
            let at = self.e.mem.alloc(text.len() as u32 + 8);
            self.e.mem.set_cstr(at, text.as_bytes());
            at
        }

        fn texture(&mut self, name: &str, width: u32, height: u32) -> u32 {
            let texture = self.e.mem.alloc(0x20);
            let name = self.string(name);
            self.e.mem.set_u32(texture, TEXTURE_VTABLE);
            self.e.mem.set_u32(texture + 4, 1);
            self.e.mem.set_u32(texture + 0x10, name);
            self.e.mem.set_u32(texture + 0x14, width);
            self.e.mem.set_u32(texture + 0x18, height);
            texture
        }

        fn init(&mut self) {
            fn_00a61ad0(&mut self.e);
        }

        fn map(&self, class: u32) -> u32 {
            tex_map(&self.e, class)
        }

        fn entry_map(&self) -> u32 {
            self.e.mem.u32(ENTRY_MAP)
        }

        fn refs(&self, object: u32) -> u32 {
            self.e.mem.u32(object + 4)
        }

        /// Adds a texture to the initialized palette under `file_entry`.
        fn add(&mut self, name: &str, size: u32, file_entry: u32) -> u32 {
            let texture = self.texture(name, size, size);
            bs_texture_palette_set_texture(&mut self.e, texture, file_entry);
            texture
        }

        fn count(&self, class: u32) -> u32 {
            self.e.mem.u32(self.map(class) + 0xb24)
        }
    }

    fn reference_hash(name: &str) -> u32 {
        name.bytes()
            .fold(0u32, |h, c| h.wrapping_mul(33).wrapping_add(c as u32))
            % 0x2c9
    }

    #[test]
    fn compare_ignores_case_and_a_three_letter_extension() {
        let mut f = Fixture::new();
        let mut same = |a: &str, b: &str| {
            let (a, b) = (f.string(a), f.string(b));
            fn_00a614d0(&mut f.e, a, b)
        };
        assert!(same("Wall.DDS", "wall.dds"));
        assert!(
            same("wall.dds", "wall.ddx"),
            "the extension is not compared"
        );
        assert!(same("wa", "wall.dds"), "a prefix matches");
        assert!(same("", "anything"));
        assert!(!same("wall", "wald"));
        assert!(
            !same("wall.dds", "wall_dds"),
            "the stored key needs the dot too"
        );
        assert!(
            !same("wall.ddsx", "wall.ddsy"),
            "a longer extension is compared"
        );
    }

    #[test]
    fn hash_index_stops_before_a_three_letter_extension() {
        let mut f = Fixture::new();
        let mut hash = |text: &str| {
            let at = f.string(text);
            bs_tex_map_key_to_hash_index(&mut f.e, at)
        };
        assert_eq!(hash("ab"), 447);
        assert_eq!(hash("AB"), 447, "case is folded");
        assert_eq!(hash("ab.dds"), 447);
        assert_eq!(hash("ab.ddsx"), reference_hash("ab.ddsx"));
        assert_eq!(hash(""), 0);
    }

    #[test]
    fn entry_map_add_stores_the_key_and_a_node_in_the_bucket() {
        let mut f = Fixture::new();
        f.init();
        let item = f.e.mem.alloc(0x10);
        let entry_map = f.entry_map();
        fn_00a61610(&mut f.e, Ptr::new(entry_map), 0x1230, item);
        let bucket = entry_map + 0x123 * 4;
        let node = f.e.mem.u32(bucket);
        assert_ne!(node, 0);
        assert_eq!(f.e.mem.u32(node), item);
        assert_eq!(f.e.mem.u32(node + 4), 0);
        assert_eq!(f.e.mem.u32(item + 4), 0x1230);
        assert_eq!(f.e.mem.u32(entry_map + 0xfa4), 1);
        // A second key in the same bucket goes in front.
        let other = f.e.mem.alloc(0x10);
        fn_00a61610(&mut f.e, Ptr::new(entry_map), 0x1231, other);
        let head = f.e.mem.u32(bucket);
        assert_eq!(f.e.mem.u32(head), other);
        assert_eq!(f.e.mem.u32(head + 4), node);
    }

    #[test]
    fn entry_map_remove_unlinks_clears_the_key_and_returns_the_item() {
        let mut f = Fixture::new();
        f.init();
        let entry_map = f.entry_map();
        let (first, second) = (f.e.mem.alloc(0x10), f.e.mem.alloc(0x10));
        fn_00a61610(&mut f.e, Ptr::new(entry_map), 0x1230, first);
        fn_00a61610(&mut f.e, Ptr::new(entry_map), 0x1231, second);
        // The list is second -> first; remove the tail, then the head.
        let removed = bs_entry_map_remove(&mut f.e, Ptr::new(entry_map), 0x1230);
        assert_eq!(removed, first);
        assert_eq!(f.e.mem.u32(first + 4), 0);
        assert_eq!(f.e.mem.u32(entry_map + 0xfa4), 1);
        assert_eq!(
            bs_entry_map_remove(&mut f.e, Ptr::new(entry_map), 0x1230),
            0
        );
        let removed = bs_entry_map_remove(&mut f.e, Ptr::new(entry_map), 0x1231);
        assert_eq!(removed, second);
        assert_eq!(f.e.mem.u32(entry_map + 0x123 * 4), 0);
        assert_eq!(f.e.mem.u32(entry_map + 0xfa4), 0);
    }

    #[test]
    fn entry_map_clear_frees_nodes_and_zeroes_keys_and_count() {
        let mut f = Fixture::new();
        f.init();
        let entry_map = f.entry_map();
        let items: Vec<u32> = (0..3).map(|_| f.e.mem.alloc(0x10)).collect();
        for (i, item) in items.iter().enumerate() {
            let key = 0x1230 + 0x10 * i as u32;
            fn_00a61610(&mut f.e, Ptr::new(entry_map), key, *item);
        }
        fn_00a615c0(&mut f.e, Ptr::new(entry_map));
        assert_eq!(f.e.mem.u32(entry_map + 0xfa4), 0);
        for (i, item) in items.iter().enumerate() {
            assert_eq!(f.e.mem.u32(*item + 4), 0);
            assert_eq!(f.e.mem.u32(entry_map + (0x123 + i as u32) % 0x3e9 * 4), 0);
        }
    }

    #[test]
    fn size_class_follows_the_larger_dimension() {
        let mut f = Fixture::new();
        for (width, height, class) in [
            (64, 64, 0),
            (128, 8, 1),
            (8, 255, 1),
            (256, 1, 2),
            (1, 511, 2),
            (512, 512, 3),
            (1024, 4, 4),
            (4, 4096, 4),
        ] {
            let texture = f.texture("t", width, height);
            assert_eq!(fn_00a61750(&mut f.e, texture), class, "{width}x{height}");
        }
    }

    #[test]
    fn find_or_create_adds_one_item_per_key_and_takes_a_reference() {
        let mut f = Fixture::new();
        f.init();
        let map = f.map(0);
        let texture = f.texture("textures\\wall.dds", 64, 64);
        let item = fn_00a617c0(&mut f.e, Ptr::new(map), texture);
        assert_eq!(f.e.mem.u32(item), texture);
        assert_eq!(f.refs(texture), 2);
        assert_eq!(f.e.mem.u32(item + 4), 0);
        assert_eq!(f.e.mem.u16(item + 0xc), 0);
        assert_eq!(f.e.mem.i16(item + 0xe), 9, "after `textures\\`");
        assert_eq!(f.count(0), 1);
        let bucket = map + reference_hash("wall") * 4;
        assert_eq!(f.e.mem.u32(bucket), item);
        // The same key from another texture finds the same item.
        let twin = f.texture("textures\\WALL.dds", 64, 64);
        assert_eq!(fn_00a617c0(&mut f.e, Ptr::new(map), twin), item);
        assert_eq!(f.count(0), 1);
        // A different key makes a second item.
        let other = f.texture("floor.dds", 64, 64);
        let second = fn_00a617c0(&mut f.e, Ptr::new(map), other);
        assert_ne!(second, item);
        assert_eq!(f.count(0), 2);
        assert_eq!(f.e.mem.i16(second + 0xe), 0);
    }

    #[test]
    fn tex_map_remove_unlinks_releases_the_texture_and_frees_the_item() {
        let mut f = Fixture::new();
        f.init();
        let map = f.map(0);
        let texture = f.texture("wall.dds", 64, 64);
        let item = fn_00a617c0(&mut f.e, Ptr::new(map), texture);
        bs_tex_map_remove(&mut f.e, Ptr::new(map), item);
        assert_eq!(f.count(0), 0);
        assert_eq!(f.e.mem.u32(map + reference_hash("wall") * 4), 0);
        assert_eq!(f.refs(texture), 1, "the item's reference is gone");
        assert!(f.deleted.borrow().is_empty());
        // An item that is not in the table changes nothing.
        let stranger = f.e.mem.alloc(0x10);
        f.e.mem.set_u32(stranger, texture);
        bs_tex_map_remove(&mut f.e, Ptr::new(map), stranger);
        assert_eq!(f.refs(texture), 1);
        // The last reference deletes the texture.
        let again = fn_00a617c0(&mut f.e, Ptr::new(map), texture);
        f.e.mem.set_u32(texture + 4, 1);
        bs_tex_map_remove(&mut f.e, Ptr::new(map), again);
        assert_eq!(*f.deleted.borrow(), vec![texture]);
    }

    #[test]
    fn bucket_lookup_stores_the_texture_in_the_pointer() {
        let mut f = Fixture::new();
        f.init();
        let map = f.map(0);
        let texture = f.texture("wall.dds", 64, 64);
        fn_00a617c0(&mut f.e, Ptr::new(map), texture);
        let out = f.e.mem.alloc(4);
        let key = f.string("WALL.dds");
        let bucket = reference_hash("wall");
        assert!(fn_00a61920(&mut f.e, Ptr::new(map), bucket, key, out));
        assert_eq!(f.e.mem.u32(out), texture);
        assert_eq!(f.refs(texture), 3);
        // Looking up again with the same texture in place changes no count.
        assert!(fn_00a61920(&mut f.e, Ptr::new(map), bucket, key, out));
        assert_eq!(f.refs(texture), 3);
        let missing = f.string("nothing.dds");
        assert!(!fn_00a61920(&mut f.e, Ptr::new(map), bucket, missing, out));
        assert!(!fn_00a61920(&mut f.e, Ptr::new(map), bucket + 1, key, out));
    }

    #[test]
    fn iterator_begin_starts_in_the_largest_non_empty_class() {
        let mut f = Fixture::new();
        f.init();
        let iterator = f.e.mem.alloc(0xc);
        assert!(!fn_00a616e0(&mut f.e, Ptr::new(iterator)));
        assert_eq!(f.e.mem.u32(iterator), 0);
        f.add("small.dds", 64, 0x100);
        let large = f.add("large.dds", 600, 0x200);
        assert!(fn_00a616e0(&mut f.e, Ptr::new(iterator)));
        let item = f.e.mem.u32(iterator);
        assert_eq!(f.e.mem.u32(item), large);
        assert_eq!(f.e.mem.u32(iterator + 4), reference_hash("large"));
        assert_eq!(f.e.mem.u32(iterator + 8), 3);
    }

    #[test]
    fn iterator_next_copies_the_texture_and_walks_buckets() {
        let mut f = Fixture::new();
        f.init();
        let map = f.map(0);
        let (a, b) = (f.texture("aa.dds", 64, 64), f.texture("bb.dds", 64, 64));
        let first = fn_00a617c0(&mut f.e, Ptr::new(map), a);
        let second = fn_00a617c0(&mut f.e, Ptr::new(map), b);
        let iterator = f.e.mem.alloc(0xc);
        let (low, high) = (reference_hash("aa"), reference_hash("bb"));
        assert_ne!(low, high);
        let (first_bucket, second_bucket) = (low.min(high), low.max(high));
        let (head, tail) = if low < high {
            (first, second)
        } else {
            (second, first)
        };
        f.e.mem.set_u32(iterator, head);
        f.e.mem.set_u32(iterator + 4, first_bucket);
        let out = f.e.mem.alloc(4);
        assert!(fn_00a619b0(
            &mut f.e,
            Ptr::new(map),
            Ptr::new(iterator),
            out
        ));
        assert_eq!(f.e.mem.u32(out), f.e.mem.u32(head));
        assert_eq!(f.e.mem.u32(iterator), tail);
        assert_eq!(f.e.mem.u32(iterator + 4), second_bucket);
        let more = fn_00a619b0(&mut f.e, Ptr::new(map), Ptr::new(iterator), out);
        assert!(!more);
        assert_eq!(f.e.mem.u32(out), f.e.mem.u32(tail));
        assert_eq!(f.e.mem.u32(iterator), 0);
        assert_eq!(f.e.mem.u32(iterator + 4), 0x2c9);
        // At the end it only reports.
        let more = fn_00a619b0(&mut f.e, Ptr::new(map), Ptr::new(iterator), out);
        assert!(!more);
    }

    #[test]
    fn iterator_next_reads_the_item_and_the_iterator_again_after_the_release() {
        // 00a619ed and 00a61a00: the item's texture and the iterator's item are loaded
        // again after the old texture's release, whose destructor here changes both.
        let mut f = Fixture::new();
        f.init();
        let map = f.map(0);
        let (first, second) = (f.texture("aa.dds", 64, 64), f.texture("bb.dds", 64, 64));
        let old = f.texture("old.dds", 64, 64);
        let item = f.e.mem.alloc(0x10);
        f.e.mem.set_u32(item, first);
        let moved = f.e.mem.alloc(0x10);
        f.e.mem.set_u32(moved + 8, 0x0600_0010);
        let iterator = f.e.mem.alloc(0xc);
        f.e.mem.set_u32(iterator, item);
        let out = f.e.mem.alloc(4);
        f.e.mem.set_u32(out, old);
        f.e.register_double(DOUBLE_DELETE, move |e, a| {
            assert_eq!(a[0], old);
            e.mem.set_u32(item, second);
            e.mem.set_u32(iterator, moved);
            Ret::default()
        });
        assert!(fn_00a619b0(
            &mut f.e,
            Ptr::new(map),
            Ptr::new(iterator),
            out
        ));
        assert_eq!(f.e.mem.u32(out), second);
        assert_eq!((f.refs(old), f.refs(first), f.refs(second)), (0, 1, 2));
        assert_eq!(f.e.mem.u32(iterator), 0x0600_0010);
    }

    #[test]
    fn entry_lookup_finds_the_texture_of_a_file_entry() {
        let mut f = Fixture::new();
        f.init();
        let texture = f.add("wall.dds", 64, 0x5550);
        let out = f.e.mem.alloc(4);
        let entry_map = Ptr::new(f.entry_map());
        let bucket = 0x555 % 0x3e9;
        assert!(fn_00a61a60(&mut f.e, entry_map, bucket, 0x5550, out));
        assert_eq!(f.e.mem.u32(out), texture);
        assert_eq!(f.refs(texture), 3);
        assert!(!fn_00a61a60(&mut f.e, entry_map, bucket, 0x5551, out));
        assert!(!fn_00a61a60(&mut f.e, entry_map, bucket + 1, 0x5550, out));
    }

    #[test]
    fn init_builds_the_entry_map_and_seven_tex_maps_and_restores_the_tls_word() {
        let mut f = Fixture::new();
        let tls = f.e.tls();
        f.e.mem.set_u32(tls + 0x2b4, 0x77);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        f.e.register_double(VECTOR_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        f.e.mem.set_u8(PARTIALLY_PURGED, 1);
        f.init();
        let entry_map = f.entry_map();
        assert_ne!(entry_map, 0);
        assert_eq!(f.e.mem.u32(entry_map + 0xfa4), 0);
        let maps = f.e.mem.u32(TEX_MAP_ARRAY);
        assert_eq!(f.e.mem.u32(maps - 4), 7);
        assert_eq!(*calls.borrow(), vec![vec![maps, 0xb28, 7, 0x00a6_1590]]);
        assert_eq!(f.e.mem.u32(tls + 0x2b4), 0x77);
        assert_eq!(f.e.mem.u8(PARTIALLY_PURGED), 0);
    }

    #[test]
    fn get_texture_by_path_searches_the_classes_and_replaces_the_pointer() {
        let mut f = Fixture::new();
        let out = f.e.mem.alloc(4);
        let path = f.string("textures\\wall.dds");
        let found = bs_texture_palette_get_texture_ov2(&mut f.e, path, out);
        assert!(!found, "no palette");
        f.init();
        let wall = f.add("wall.dds", 600, 0);
        assert!(bs_texture_palette_get_texture_ov2(&mut f.e, path, out));
        assert_eq!(f.e.mem.u32(out), wall);
        assert_eq!(f.refs(wall), 3);
        assert_eq!(f.e.mem.u32(ADD_CRIT), 0, "the lock was left");
        // An unknown path releases the old texture and clears the pointer.
        let unknown = f.string("nothing.dds");
        assert!(!bs_texture_palette_get_texture_ov2(&mut f.e, unknown, out));
        assert_eq!(f.e.mem.u32(out), 0);
        assert_eq!(f.refs(wall), 2);
    }

    #[test]
    fn set_texture_files_it_by_size_and_registers_the_file_entry_once() {
        let mut f = Fixture::new();
        let texture = f.texture("wall.dds", 300, 20);
        bs_texture_palette_set_texture(&mut f.e, texture, 0x40);
        assert_eq!(f.refs(texture), 1, "no palette yet");
        f.init();
        bs_texture_palette_set_texture(&mut f.e, texture, 0x40);
        assert_eq!(f.count(2), 1);
        let item = f.e.mem.u32(f.map(2) + reference_hash("wall") * 4);
        assert_eq!(f.e.mem.u16(item + 0xc), 2);
        assert_eq!(f.e.mem.u32(item + 4), 0x40);
        assert_eq!(f.e.mem.u32(f.entry_map() + 0xfa4), 1);
        // Again: same item, key kept, nothing registered twice.
        bs_texture_palette_set_texture(&mut f.e, texture, 0x50);
        assert_eq!(f.count(2), 1);
        assert_eq!(f.e.mem.u32(item + 4), 0x40);
        assert_eq!(f.e.mem.u32(f.entry_map() + 0xfa4), 1);
        // Without a key the texture's own file entry is used; with none at
        // all nothing is registered.
        f.file_entry.set(0x60);
        let second = f.texture("floor.dds", 64, 64);
        bs_texture_palette_set_texture(&mut f.e, second, 0);
        let item = f.e.mem.u32(f.map(0) + reference_hash("floor") * 4);
        assert_eq!(f.e.mem.u32(item + 4), 0x60);
        f.file_entry.set(0);
        let third = f.texture("sky.dds", 64, 64);
        bs_texture_palette_set_texture(&mut f.e, third, 0);
        let item = f.e.mem.u32(f.map(0) + reference_hash("sky") * 4);
        assert_eq!(f.e.mem.u32(item + 4), 0);
        assert_eq!(f.e.mem.u32(f.entry_map() + 0xfa4), 2);
    }

    #[test]
    fn purge_removes_textures_only_the_palette_holds() {
        let mut f = Fixture::new();
        f.init();
        let unused_small = f.add("small.dds", 64, 0x100);
        let unused_large = f.add("large.dds", 600, 0x200);
        let used = f.add("used.dds", 64, 0x300);
        // Only the palette's reference remains on the first two; `used` has
        // another holder.
        f.e.mem.set_u32(unused_small + 4, 1);
        f.e.mem.set_u32(unused_large + 4, 1);
        f.e.mem.set_u32(used + 4, 3);
        f.e.mem.set_u32(MAX_TEXTURES_TO_PURGE, 10);
        f.e.mem.set_u8(PARTIALLY_PURGED, 1);
        bs_texture_palette_purge_unused_textures(&mut f.e);
        assert_eq!(f.count(3), 0);
        assert_eq!(f.count(0), 1, "the shared texture stays");
        assert_eq!(f.e.mem.u32(f.entry_map() + 0xfa4), 1);
        let mut deleted = f.deleted.borrow().clone();
        deleted.sort();
        let mut expected = vec![unused_small, unused_large];
        expected.sort();
        assert_eq!(deleted, expected);
        assert_eq!(f.refs(used), 3);
        assert_eq!(f.e.mem.u8(PARTIALLY_PURGED), 0);
        assert_eq!(f.e.mem.u32(ADD_CRIT), 0);
    }

    #[test]
    fn purge_stops_at_the_limit() {
        let mut f = Fixture::new();
        f.init();
        let textures: Vec<u32> = (0..3)
            .map(|i| f.add(&format!("t{i}.dds"), 64, 0x100 * (i + 1)))
            .collect();
        for t in &textures {
            f.e.mem.set_u32(*t + 4, 1);
        }
        f.e.mem.set_u32(MAX_TEXTURES_TO_PURGE, 1);
        bs_texture_palette_purge_unused_textures(&mut f.e);
        assert_eq!(f.count(0), 2);
        assert_eq!(f.deleted.borrow().len(), 1);
    }

    #[test]
    fn purge_without_a_palette_does_nothing() {
        let mut f = Fixture::new();
        f.e.mem.set_u8(PARTIALLY_PURGED, 1);
        bs_texture_palette_purge_unused_textures(&mut f.e);
        assert_eq!(f.e.mem.u8(PARTIALLY_PURGED), 1);
    }

    #[test]
    fn item_for_finds_the_exact_texture_in_any_class() {
        let mut f = Fixture::new();
        f.init();
        let wall = f.add("wall.dds", 600, 0);
        let twin = f.texture("wall.dds", 600, 600);
        let item = bs_texture_palette_get_item_for(&mut f.e, wall);
        assert_eq!(f.e.mem.u32(item), wall);
        assert_eq!(f.e.mem.u16(item + 0xc), 3);
        let other = bs_texture_palette_get_item_for(&mut f.e, twin);
        assert_eq!(other, 0, "same key, other texture");
    }

    #[test]
    fn clearing_a_tex_map_releases_every_texture() {
        let mut f = Fixture::new();
        f.init();
        let a = f.add("a1.dds", 64, 0);
        let b = f.add("b2.dds", 64, 0);
        f.e.mem.set_u32(a + 4, 2);
        f.e.mem.set_u32(b + 4, 1);
        let map = f.map(0);
        fn_00a61ec0(&mut f.e, Ptr::new(map));
        assert_eq!(f.count(0), 0);
        assert_eq!(f.refs(a), 1);
        assert_eq!(*f.deleted.borrow(), vec![b]);
        for bucket in 0..0x2c9 {
            assert_eq!(f.e.mem.u32(map + bucket * 4), 0);
        }
    }

    #[test]
    fn remove_texture_drops_it_from_the_entry_map_and_its_class() {
        let mut f = Fixture::new();
        let wall = f.texture("wall.dds", 64, 64);
        assert!(
            !bs_texture_palette_remove_texture(&mut f.e, wall),
            "no palette"
        );
        f.init();
        bs_texture_palette_set_texture(&mut f.e, wall, 0x70);
        let stranger = f.texture("other.dds", 64, 64);
        assert!(!bs_texture_palette_remove_texture(&mut f.e, stranger));
        assert!(bs_texture_palette_remove_texture(&mut f.e, wall));
        assert_eq!(f.count(0), 0);
        assert_eq!(f.e.mem.u32(f.entry_map() + 0xfa4), 0);
        assert_eq!(f.refs(wall), 1);
        assert_eq!(f.e.mem.u32(ADD_CRIT), 0);
        assert!(!bs_texture_palette_remove_texture(&mut f.e, wall));
    }

    #[test]
    fn purging_and_removing_by_path_keep_their_locals_in_the_exe_frames() {
        let mut f = Fixture::new();
        f.init();
        let only = f.add("small.dds", 64, 0x100);
        f.e.mem.set_u32(only + 4, 3);
        f.e.mem.set_u32(MAX_TEXTURES_TO_PURGE, 10);
        // The frame is the next local on the game thread's stack; its bytes stay there after
        // the call returns (nothing takes a local before the reads below).
        let frame = f.e.next_stack_local(0x2c);
        bs_texture_palette_purge_unused_textures(&mut f.e);
        // The answered texture sits at +0x8 of the base 0xc above the frame's bottom.
        assert_eq!(f.e.mem.u32(frame + 0xc + 8), only);
        let path = f.string("textures\\small.dds");
        let frame = f.e.next_stack_local(0xc);
        assert!(bs_texture_palette_remove_texture_ov2(&mut f.e, path));
        assert_eq!(f.e.mem.u32(frame + 8), only);
    }

    #[test]
    fn remove_texture_by_path_removes_and_releases_the_lookup_reference() {
        let mut f = Fixture::new();
        let path = f.string("textures\\wall.dds");
        assert!(
            !bs_texture_palette_remove_texture_ov2(&mut f.e, path),
            "no palette"
        );
        f.init();
        let wall = f.add("wall.dds", 64, 0x80);
        assert!(bs_texture_palette_remove_texture_ov2(&mut f.e, path));
        assert_eq!(f.count(0), 0);
        assert_eq!(f.refs(wall), 1, "palette and lookup references are gone");
        assert!(!bs_texture_palette_remove_texture_ov2(&mut f.e, path));
        assert_eq!(f.e.mem.u32(ADD_CRIT), 0);
        assert_eq!(f.e.mem.u32(ADD_CRIT + 4), 0);
    }
}

#[cfg(test)]
mod tex_map_constructor_tests {
    use super::*;

    #[test]
    fn tex_map_constructor_clears_the_map() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, _| Ret::default());
        let map = e.mem.alloc(MAP_SIZE);
        e.mem.set_u32(map + 0xb24, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_1590, &args![map]).u32(), map);
        assert_eq!(e.mem.u32(map + 0xb24), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MEMSET, vec![map, 0, 0xb24]));
    }
}

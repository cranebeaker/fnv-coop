//! `NiXenonRenderer/run_00e8c570` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRefObject` virtual table and the global object count it counts up (`011f4400`,
/// `InterlockedIncrement`), the class's virtual table (RTTI `NiDX9VertexBufferManager`), the
/// maps' virtual tables (RTTI `NiTMapBase<..., unsigned int,
/// NiDX9VertexBufferManager::NiDX9VBInfo *>`, then `NiTPointerMap<...>`), the bucket allocation
/// `00aa1070`, `memset`, `InitializeCriticalSection` (import slot) and the map set-up
/// `00b601a0` (unnamed; `thiscall`, no stack words).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE: u32 = 0x010e_fe94;
const VB_MAP_BASE_VTABLE: u32 = 0x010e_fe54;
const VB_MAP_VTABLE: u32 = 0x010e_fe74;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const MAP_00B601A0: u32 = 0x00b6_01a0;

// Translated from 00e8d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the `NiDX9VertexBufferManager` constructor (the
/// RTTI of the virtual table it stores), `thiscall` with the device. The `NiRefObject` start
/// (its table, count 0, the global object count up), the class's table, three maps at +0xc,
/// +0x1c, +0x2c (0x25 buckets each, cleared), +0x40 to +0x4c cleared, the critical section at
/// +0x80, the device at +8 (counted: its slot 4), `00b601a0` on each map and +0x3c cleared.
/// Answers `this`.
pub fn fn_00e8d0e0(e: &mut Engine, this: Ptr, device: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
    for offset in [0xcu32, 0x1c, 0x2c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, VB_MAP_BASE_VTABLE);
        e.mem.set_u32(map.wrapping_add(4), 0x25);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, VB_MAP_VTABLE);
    }
    for offset in [0x40u32, 0x44, 0x48, 0x4c] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    for offset in [0xcu32, 0x1c, 0x2c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    e.mem.set_u32(base.wrapping_add(0x3c), 0);
    this
}

/// The first node of a pointer map (`NiTMapBase`: vtable +0, bucket count +4, buckets +8): the
/// first non-null bucket, or 0.
fn map_first(e: &mut Engine, map: u32) -> u32 {
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    for index in 0..count {
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
    }
    0
}

/// The node after `node` (next +0, key +4): its next, or the first non-null bucket after the
/// key's bucket (the map's virtual slot 4, `thiscall` with the key), or 0.
fn map_next(e: &mut Engine, map: u32, node: u32) -> u32 {
    let next = e.mem.u32(node);
    if next != 0 {
        return next;
    }
    let key = e.mem.u32(node.wrapping_add(4));
    let mut index = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
    while index < e.mem.u32(map.wrapping_add(4)) {
        let buckets = e.mem.u32(map.wrapping_add(8));
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
        index += 1;
    }
    0
}

/// What the destructor `00e8ceb0` uses besides the constructor's: the interlocked decrement
/// import, `DeleteCriticalSection` (import slot), the free `00aa10f0` (`cdecl`, one word), the
/// Ni free `00aa1460` (`cdecl`: block, size) and the buffer entry clean-up `00e95b80` (unnamed;
/// `thiscall`, no stack words).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const ENTRY_CLEAN_UP_00E95B80: u32 = 0x00e9_5b80;

// Translated from 00e8ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the `NiDX9VertexBufferManager` destructor,
/// `thiscall`, no stack words. Stores the class's table; then for each of the three maps
/// (+0xc, +0x1c, +0x2c) walks its nodes (the next node found before the current one's value is
/// freed): a value (a 0x1c-byte record) has five entries at +8 to +0x18, each one not null
/// releasing its +0x20 (slot 8, `stdcall`) when set, cleaned up (`00e95b80`) and freed (0x3c
/// bytes); then the record is freed. Then `00b601a0` on each map, the device (+8) released
/// (slot 8, no null check) and cleared, +0x40 freed (`00aa10f0`), the critical section (+0x80)
/// deleted, and each map from the last undone: its two tables with `00b601a0` after each and
/// its buckets freed; finally the `NiRefObject` table and the global object count counted down.
pub fn fn_00e8ceb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
    for offset in [0xcu32, 0x1c, 0x2c] {
        let map = base.wrapping_add(offset);
        let mut node = map_first(e, map);
        while node != 0 {
            let value = e.mem.u32(node.wrapping_add(8));
            node = map_next(e, map, node);
            if value == 0 {
                continue;
            }
            for slot in 0..5u32 {
                let entry = e.mem.u32(value.wrapping_add(8 + 4 * slot));
                if entry == 0 {
                    continue;
                }
                let buffer = e.mem.u32(entry.wrapping_add(0x20));
                if buffer != 0 {
                    let buffer = e.mem.u32(entry.wrapping_add(0x20));
                    e.vcall(buffer, 8, &args![]);
                }
                e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                e.call(NI_FREE_00AA1460, &args![entry, 0x3cu32]);
            }
            e.call(NI_FREE_00AA1460, &args![value, 0x1cu32]);
        }
    }
    for offset in [0xcu32, 0x1c, 0x2c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    let device = e.mem.u32(base.wrapping_add(8));
    e.vcall(device, 8, &args![]);
    let block = e.mem.u32(base.wrapping_add(0x40));
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.call(FREE_00AA10F0, &args![block]);
    e.call(
        IMPORT_DELETE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    for offset in [0x2cu32, 0x1c, 0xc] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, VB_MAP_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        e.mem.set_u32(map, VB_MAP_BASE_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        let buckets = e.mem.u32(map.wrapping_add(8));
        e.call(FREE_00AA10F0, &args![buckets]);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e8d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the scalar deleting destructor, `thiscall` with
/// one word (the flags): the destructor `00e8ceb0`, then with bit 0 set the 0x100 bytes freed
/// (`00aa1460`). Answers `this`.
pub fn fn_00e8d200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e8ceb0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x100u32]);
    }
    this
}

/// What the vertex packers use: the geometry data's stream set-up `00e8f010` (`thiscall`, the
/// vertex format), the manager's buffer reservation `00e8bfa0` (`thiscall`: the data's stream
/// object and 0; answers in `AL`), the data's new buffer record `00e69b40` (`thiscall`, 0), the
/// staging lock `00e8bff0` (`thiscall`, four words of the record: +8, +0xc, +0x14, +0x10),
/// `memcpy` (`00ec44d0`), `LeaveCriticalSection` (import slot) and the `double` 255.0
/// (`0101e568`) the colours are scaled by.
const DATA_SET_FORMAT_00E8F010: u32 = 0x00e8_f010;
const RESERVE_00E8BFA0: u32 = 0x00e8_bfa0;
const NEW_RECORD_00E69B40: u32 = 0x00e6_9b40;
const LOCK_00E8BFF0: u32 = 0x00e8_bff0;
const MEMCPY: u32 = 0x00ec_44d0;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const SCALE_0101E568: u32 = 0x0101_e568;

/// A colour channel as the packers compute it: the `float` times the `double` 255.0, rounded to
/// a `float`, then `FISTP dword` with the default rounding.
fn colour_channel(e: &Engine, bits: u32) -> u32 {
    let scale = f64::from_bits(e.mem.u64(SCALE_0101E568));
    let scaled = x87_store32(x87_mul(x87_load32(bits), scale));
    crate::x87::fistp(x87_load32(scaled)) as u32
}

/// The packed colour of four `float`s at `at` (red, green, blue, alpha): each channel from
/// `colour_channel`, combined as `((alpha << 8 | red) << 8 | green) << 8 | blue` without
/// masking.
fn packed_colour(e: &Engine, at: u32) -> u32 {
    let alpha = colour_channel(e, e.mem.u32(at.wrapping_add(0xc)));
    let red = colour_channel(e, e.mem.u32(at));
    let green = colour_channel(e, e.mem.u32(at.wrapping_add(4)));
    let blue = colour_channel(e, e.mem.u32(at.wrapping_add(8)));
    ((alpha.wrapping_shl(8) | red).wrapping_shl(8) | green).wrapping_shl(8) | blue
}

/// The manager's unlock of its staging copy (as `00e8c0d0` does it): copies the staging block
/// (+0x40) back to the locked address (+0x48, size +0x4c), clears both, leaves the critical
/// section (+0x80) and answers the buffer's `Unlock` (slot 0x30) result.
fn unlock_staging(e: &mut Engine, this: u32, buffer: u32) -> i32 {
    let size = e.mem.u32(this.wrapping_add(0x4c));
    let block = e.mem.u32(this.wrapping_add(0x40));
    let locked = e.mem.u32(this.wrapping_add(0x48));
    e.call(MEMCPY, &args![locked, block, size]);
    e.mem.set_u32(this.wrapping_add(0x48), 0);
    e.mem.set_u32(this.wrapping_add(0x4c), 0);
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &args![this.wrapping_add(0x80)],
    );
    e.vcall(buffer, 0x30, &args![]).i32()
}

/// Whether the record's vertex buffer (+8) can take `count` vertices of `size` bytes in
/// `format`: its description (slot 0x34, a six-word buffer) has format 100, the vertex format
/// (+0x14) and at least `count * size` bytes (+0x10).
fn record_fits(e: &mut Engine, record: u32, format: u32, count: u32, size: u32) -> bool {
    if record == 0 {
        return false;
    }
    let buffer = e.mem.u32(record.wrapping_add(8));
    if buffer == 0 {
        return false;
    }
    e.with_stack(0x18, |e, desc| {
        let desc = desc.addr();
        e.vcall(buffer, 0x34, &args![desc]);
        e.mem.u32(desc) == 0x64
            && e.mem.u32(desc + 0x14) == format
            && e.mem.u32(desc + 0x10) >= count.wrapping_mul(size)
    })
}

// Translated from 00e8c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): packs screen-space vertices into a vertex buffer
/// (`thiscall` on the manager, eight words: the geometry data, a 16-bit count, the positions
/// (two `float`s each), the colours (four `float`s each) or 0, the texture coordinates (two
/// `float`s each) or 0, the flags of what to write, a buffer record and a locked address or 0).
/// Without a count or positions it answers the record. The format is 4 (16-byte vertices),
/// 0x44 with colours (at +0x10, 20 bytes), plus 0x100 and 8 bytes with coordinates; it goes to
/// the data (`00e8f010`) and the size to the data's first stream size when it has streams
/// (+0x20, +0x24). With a locked address the record is used as it is and stays locked;
/// otherwise a record whose buffer fits (`record_fits`) is kept, else a new one is reserved
/// (`00e8bfa0`, false answers 0) and made (`00e69b40`), and it is locked (`00e8bff0`). Flag 1
/// writes each position as x, y, 0.0, 1.0; flag 4 the packed colours (each channel times
/// 255.0, rounded); flag 2 the coordinates. A record locked here is unlocked again (answering
/// 0 when the buffer's `Unlock` fails). Answers the record used.
///
/// The exe keeps its unlock flag in the low byte of the colours' stack word and the colour
/// channels in other argument words; the translation keeps them in locals.
pub fn fn_00e8c570(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    count: u16,
    positions: u32,
    colours: u32,
    coordinates: u32,
    flags: u8,
    record: u32,
    locked: u32,
) -> u32 {
    let base = this.addr();
    if count == 0 || positions == 0 {
        return record;
    }
    let mut format = 4u32;
    let mut size = 0x10u32;
    let mut colour_offset = 0u32;
    let mut coordinate_offset = 0u32;
    if colours != 0 {
        format = 0x44;
        colour_offset = 0x10;
        size = 0x14;
    }
    if coordinates != 0 {
        format |= 0x100;
        coordinate_offset = size;
        size += 8;
    }
    e.call(DATA_SET_FORMAT_00E8F010, &args![data, format]);
    if e.mem.u32(data.wrapping_add(0x20)) != 0 {
        let first = e.mem.u32(data.wrapping_add(0x24));
        e.mem.set_u32(first, size);
    }
    let mut unlock = true;
    let (result, vertices) = if locked != 0 {
        unlock = false;
        (record, locked)
    } else {
        let used = if record_fits(e, record, format, u32::from(count), size) {
            record
        } else {
            if !e.call(RESERVE_00E8BFA0, &args![base, data, 0u32]).bool() {
                return 0;
            }
            e.call(NEW_RECORD_00E69B40, &args![data, 0u32]).u32()
        };
        let words = [0x10u32, 0x14, 0xc, 8].map(|offset| e.mem.u32(used.wrapping_add(offset)));
        let address = e
            .call(
                LOCK_00E8BFF0,
                &args![base, words[3], words[2], words[1], words[0]],
            )
            .u32();
        (used, address)
    };
    let count = u32::from(count);
    if flags & 1 != 0 {
        let mut at = vertices;
        let mut source = positions;
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);
        for _ in 0..count {
            let x = e.mem.u32(source);
            e.mem.set_u32(at, x);
            let y = e.mem.u32(source.wrapping_add(4));
            e.mem.set_u32(at.wrapping_add(4), y);
            e.mem.set_u32(at.wrapping_add(8), zero);
            source = source.wrapping_add(8);
            e.mem.set_u32(at.wrapping_add(0xc), one);
            at = at.wrapping_add(size);
        }
    }
    if colours != 0 && flags & 4 != 0 {
        let mut at = vertices.wrapping_add(colour_offset);
        let mut source = colours;
        for _ in 0..count {
            let packed = packed_colour(e, source);
            source = source.wrapping_add(0x10);
            e.mem.set_u32(at, packed);
            at = at.wrapping_add(size);
        }
    }
    if coordinates != 0 && flags & 2 != 0 {
        let mut at = vertices.wrapping_add(coordinate_offset);
        let mut source = coordinates;
        for _ in 0..count {
            let u = e.mem.u32(source);
            e.mem.set_u32(at, u);
            let v = e.mem.u32(source.wrapping_add(4));
            e.mem.set_u32(at.wrapping_add(4), v);
            at = at.wrapping_add(size);
            source = source.wrapping_add(8);
        }
    }
    if unlock {
        let buffer = e.mem.u32(result.wrapping_add(8));
        if unlock_staging(e, base, buffer) < 0 {
            return 0;
        }
    }
    result
}

/// The vertex layout `00e8c120` (`thiscall` on the manager, nine words), `NiSprintf`
/// (`00a305b0`, cdecl), `NiGeometryData::GetTextureSet` (`00a66960`, `thiscall`, the set),
/// the weight-sum tolerance (`float` 1e-5, `010718c0`) and the message
/// `"ILLEGAL WEIGHTS! Vertex %4d - Weight Sum %8.5f  (%8.5f)\n"` (`010efe14`).
const LAYOUT_00E8C120: u32 = 0x00e8_c120;
const NI_SPRINTF: u32 = 0x00a3_05b0;
const GET_TEXTURE_SET: u32 = 0x00a6_6960;
const WEIGHT_TOLERANCE_010718C0: u32 = 0x0107_18c0;
const ILLEGAL_WEIGHTS_MESSAGE: u32 = 0x010e_fe14;
/// The cap on the texture-set count (`011bf558`).
const TEXTURE_SET_LIMIT_011BF558: u32 = 0x011b_f558;

/// Copies three words per vertex from `source` (12 bytes per entry, by the 16-bit index of
/// each vertex at `indices`) to `at`, `size` bytes apart.
fn copy_indexed_triples(e: &mut Engine, indices: u32, count: u32, source: u32, at: u32, size: u32) {
    let mut at = at;
    for vertex in 0..count {
        let index = u32::from(e.mem.u16(indices.wrapping_add(vertex * 2)));
        let from = source.wrapping_add(index * 12);
        for word in 0..3u32 {
            let value = e.mem.u32(from + 4 * word);
            e.mem.set_u32(at.wrapping_add(4 * word), value);
        }
        at = at.wrapping_add(size);
    }
}

// Translated from 00e8c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): packs a skinned partition's vertices into a
/// vertex buffer (`thiscall` on the manager, six words: the geometry data, a word it does not
/// read, the partition, the mask of what to write, a buffer record and a locked address or 0).
/// The layout comes from `00e8c120` (flags, size and the offsets); the format goes to the
/// partition's stream object (+0x28, `00e8f010`) and the size to its first stream size. With
/// a locked address the record is used as it is; otherwise a fitting record is kept, else a
/// buffer is reserved (`00e8bfa0`; false answers 0), the stream object's first record (or 0
/// without streams) taken and the mask set to 0xfff; it is locked (`00e8bff0`). Then by the
/// mask, for each of the partition's vertices (count +0x1c, data indices +0xc): bit 1 the
/// data's position (+0x20) and the blend weights (the partition's +8, +0x20 per vertex): all
/// but the last, and the last too for 1 or 3 weights; a weight sum off 1.0 by more than 1e-5
/// prints the message (`NiSprintf` into a 0x100-byte buffer); bit 2 the normals (+0x24);
/// bit 4 the packed colours (+0x28); bit 8 each texture set's coordinates (the count is bit
/// 0 of the data's +0xc, capped by `011bf558`). A record locked here is unlocked again
/// (answering 0 when the buffer's `Unlock` fails). Answers the record used.
// Translation incomplete: 00e8c820 - the /GS security cookie check (00ec408c) on the 0x168-byte frame is not translated.
pub fn fn_00e8c820(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    _unused_1: u32,
    partition: u32,
    mask: u32,
    record: u32,
    locked: u32,
) -> u32 {
    let base = this.addr();
    let positions = e.mem.u32(data.wrapping_add(0x20));
    let normals = e.mem.u32(data.wrapping_add(0x24));
    let colours = e.mem.u32(data.wrapping_add(0x28));
    let mut sets = u32::from(e.mem.u16(data.wrapping_add(0xc))) & 1;
    let limit = e.mem.u32(TEXTURE_SET_LIMIT_011BF558);
    if sets > limit {
        sets = limit;
    }
    let weight_count = u32::from(e.mem.u16(partition.wrapping_add(0x20)));
    let vertex_count = u32::from(e.mem.u16(partition.wrapping_add(0x1c)));
    let indices = e.mem.u32(partition.wrapping_add(0xc));
    let mut mask = mask;
    // The frame's out words for the layout (flags, size, five offsets), the buffer
    // description and the message buffer.
    e.with_stack(0x140, |e, frame| {
        let frame = frame.addr();
        let (flags_at, size_at) = (frame, frame + 4);
        let offsets = frame + 8;
        let desc = frame + 0x20;
        let text = frame + 0x40;
        e.mem.set_u32(flags_at, 0);
        e.mem.set_u32(size_at, 0);
        e.call(
            LAYOUT_00E8C120,
            &args![
                base,
                data,
                partition,
                flags_at,
                size_at,
                offsets,
                offsets + 4,
                offsets + 8,
                offsets + 12,
                offsets + 16
            ],
        );
        let stream = e.mem.u32(partition.wrapping_add(0x28));
        let format = e.mem.u32(flags_at);
        e.call(DATA_SET_FORMAT_00E8F010, &args![stream, format]);
        let size = e.mem.u32(size_at);
        if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
            let first = e.mem.u32(stream.wrapping_add(0x24));
            e.mem.set_u32(first, size);
        }
        let offset = |e: &Engine, index: u32| e.mem.u32(offsets + 4 * index);
        let mut unlock = true;
        let (used, vertices) = if locked != 0 {
            unlock = false;
            (record, locked)
        } else {
            let fits = record != 0 && {
                let buffer = e.mem.u32(record.wrapping_add(8));
                buffer != 0 && {
                    e.vcall(buffer, 0x34, &args![desc]);
                    e.mem.u32(desc) == 0x64
                        && e.mem.u32(desc + 0x14) == format
                        && e.mem.u32(desc + 0x10) >= (vertex_count & 0xffff).wrapping_mul(size)
                }
            };
            let used = if fits {
                record
            } else {
                if !e.call(RESERVE_00E8BFA0, &args![base, stream, 0u32]).bool() {
                    return 0;
                }
                let first = if e.mem.u32(stream.wrapping_add(0x20)) != 0 {
                    e.mem.u32(e.mem.u32(stream.wrapping_add(0x28)))
                } else {
                    0
                };
                mask = 0xfff;
                first
            };
            let words = [0x10u32, 0x14, 0xc, 8].map(|at| e.mem.u32(used.wrapping_add(at)));
            let address = e
                .call(
                    LOCK_00E8BFF0,
                    &args![base, words[3], words[2], words[1], words[0]],
                )
                .u32();
            (used, address)
        };
        if positions != 0 && mask & 1 != 0 {
            let at = vertices.wrapping_add(offset(e, 0));
            copy_indexed_triples(e, indices, vertex_count, positions, at, size);
            let mut weights = e.mem.u32(partition.wrapping_add(8));
            let mut at = vertices.wrapping_add(offset(e, 1));
            let tolerance = e.mem.u32(WEIGHT_TOLERANCE_010718C0);
            for vertex in 0..vertex_count {
                let mut sum = x87_store32(1.0);
                let mut done = 0u32;
                // `CMP EDI,4; JL`: groups of four first (signed compare).
                if (weight_count as i32) >= 4 {
                    let groups = ((weight_count - 4) >> 2) + 1;
                    done = groups * 4;
                    for group in 0..groups {
                        for lane in 0..4u32 {
                            let weight = e.mem.u32(weights + 4 * (group * 4 + lane));
                            sum = x87_store32(x87_sub(x87_load32(sum), x87_m32(weight)));
                        }
                    }
                }
                while done < weight_count {
                    done += 1;
                    let weight = e.mem.u32(weights + 4 * (done - 1));
                    sum = x87_store32(x87_sub(x87_load32(sum), x87_m32(weight)));
                }
                let total = x87_load32(sum);
                let magnitude = x87_load32(x87_store32(total.abs()));
                // FCOMP; TEST AH,0x41; JNZ: the message only when above the tolerance.
                if magnitude > x87_m32(tolerance) {
                    let mut words = vec![text, 0x100, ILLEGAL_WEIGHTS_MESSAGE, vertex];
                    x87_sub(1.0, total).put(&mut words);
                    total.put(&mut words);
                    e.call(NI_SPRINTF, &words);
                }
                let last = weight_count.wrapping_sub(1) as i32;
                let mut copied = 0u32;
                if last > 0 {
                    while (copied as i32) < last {
                        let weight = x87_quiet_bits(e.mem.u32(weights + 4 * copied));
                        e.mem.set_u32(at + 4 * copied, weight);
                        copied = (copied + 1) & 0xffff;
                    }
                }
                if weight_count & 0xffff == 3 || weight_count & 0xffff == 1 {
                    let weight = x87_quiet_bits(e.mem.u32(weights + 4 * copied));
                    e.mem.set_u32(at + 4 * copied, weight);
                }
                at = at.wrapping_add(size);
                weights = weights.wrapping_add(weight_count * 4);
            }
        }
        if normals != 0 && mask & 2 != 0 {
            let at = vertices.wrapping_add(offset(e, 2));
            copy_indexed_triples(e, indices, vertex_count, normals, at, size);
        }
        if colours != 0 && mask & 4 != 0 {
            let mut at = vertices.wrapping_add(offset(e, 3));
            for vertex in 0..vertex_count {
                let index = u32::from(e.mem.u16(indices.wrapping_add(vertex * 2)));
                let packed = packed_colour(e, colours.wrapping_add(index * 0x10));
                e.mem.set_u32(at, packed);
                at = at.wrapping_add(size);
            }
        }
        if mask & 8 != 0 {
            let mut column = vertices.wrapping_add(offset(e, 4));
            for set in 0..sets {
                let coordinates = e.call(GET_TEXTURE_SET, &args![data, set]).u32();
                let mut at = column;
                for vertex in 0..vertex_count {
                    let index = u32::from(e.mem.u16(indices.wrapping_add(vertex * 2)));
                    let from = coordinates.wrapping_add(index * 8);
                    let u = e.mem.u32(from);
                    e.mem.set_u32(at, u);
                    let v = e.mem.u32(from + 4);
                    e.mem.set_u32(at.wrapping_add(4), v);
                    at = at.wrapping_add(size);
                }
                column = column.wrapping_add(8);
            }
        }
        // Diagnostic only (`NV_HEAD_CHECK`, not exe behaviour): reads what was packed.
        if positions != 0
            && mask & 1 != 0
            && super::run_00e6fbb0::head_check_enabled()
            && super::run_00e6fbb0::head_check_is_head_data(data)
        {
            super::run_00e6fbb0::head_plain_pack_check(
                e,
                data,
                partition,
                vertices.wrapping_add(offset(e, 0)),
                size,
                vertex_count,
                mask,
            );
        }
        if unlock {
            let buffer = e.mem.u32(used.wrapping_add(8));
            if unlock_staging(e, base, buffer) < 0 {
                return 0;
            }
        }
        used
    })
}

/// `00b57f20` (`thiscall` on a pointer map: the iterator, the key's and the value's out
/// words): the map's `GetNext`.
const MAP_ITERATE_00B57F20: u32 = 0x00b5_7f20;

// Translated from 00e8cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): frees the buffer records of the first map
/// (+0xc; `thiscall`). From the first non-null bucket, each item goes through `GetNext`
/// (`00b57f20`); a record has five entries at +8 to +0x18, each one not null releasing its
/// +0x20 (slot 8) when set, cleaned up (`00e95b80`) and freed (0x3c bytes); then the record
/// is freed (0x1c bytes). Ends with the map's `00b601a0` (a tail jump).
pub fn fn_00e8cd60(e: &mut Engine, this: Ptr) {
    let map = this.addr().wrapping_add(0xc);
    let first = map_first(e, map);
    e.with_stack(0xc, |e, words| {
        let iterator = words.addr();
        let key = iterator + 4;
        let value = iterator + 8;
        e.mem.set_u32(iterator, first);
        while e.mem.u32(iterator) != 0 {
            e.call(MAP_ITERATE_00B57F20, &args![map, iterator, key, value]);
            let record = e.mem.u32(value);
            if record == 0 {
                continue;
            }
            for slot in 0..5u32 {
                let entry = e.mem.u32(record.wrapping_add(8 + 4 * slot));
                if entry == 0 {
                    continue;
                }
                let held = e.mem.u32(entry.wrapping_add(0x20));
                if held != 0 {
                    e.vcall(held, 8, &args![]);
                }
                e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                e.call(NI_FREE_00AA1460, &args![entry, 0x3cu32]);
            }
            e.call(NI_FREE_00AA1460, &args![record, 0x1cu32]);
        }
    });
    e.call(MAP_00B601A0, &args![map]);
}

/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00e8ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the scalar deleting destructor of a buffer map's
/// base (`thiscall`, the flags): its table (`010efe54`), `00b601a0`, the bucket array (+8)
/// freed (`00aa10f0`), and with bit 0 the block goes back to the memory manager (`00aa4060`).
/// Answers `this`.
pub fn fn_00e8ce30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VB_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

// Translated from 00e8ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the scalar deleting destructor of a buffer map
/// (`thiscall`, the flags): its table (`010efe74`) and `00b601a0`, the base's table
/// (`010efe54`) and `00b601a0` again, the bucket array (+8) freed (`00aa10f0`), and with bit 0
/// the block goes back to the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00e8ce70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VB_MAP_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    e.mem.set_u32(base, VB_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e8_c570,
            fn_00e8c570(Ptr, u32, u16, u32, u32, u32, u8, u32, u32) -> u32
        ),
        entry!(
            0x00e8_c820,
            fn_00e8c820(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_cd60, fn_00e8cd60(Ptr)),
        entry!(0x00e8_ce30, fn_00e8ce30(Ptr, u32) -> Ptr),
        entry!(0x00e8_ce70, fn_00e8ce70(Ptr, u32) -> Ptr),
        entry!(0x00e8_d0e0, fn_00e8d0e0(Ptr, u32) -> Ptr),
        entry!(0x00e8_ceb0, fn_00e8ceb0(Ptr)),
        entry!(0x00e8_d200, fn_00e8d200(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e8d0e0_builds_the_vertex_buffer_manager() {
        let mut e = Engine::new();
        for no_op in [
            IMPORT_INTERLOCKED_INCREMENT,
            MEMSET,
            IMPORT_INITIALIZE_CRITICAL_SECTION,
            MAP_00B601A0,
            0x6100,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.put_vtable(0x6000, &[0, 0x6100]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x3c, 7);
        assert_eq!(e.call(0x00e8_d0e0, &args![this, device]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
        for offset in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), VB_MAP_VTABLE);
            assert_eq!(e.mem.u32(this + offset + 4), 0x25);
        }
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        let log = e.call_log.clone().unwrap();
        let set_ups: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MAP_00B601A0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(set_ups, vec![this + 0xc, this + 0x1c, this + 0x2c]);
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        assert!(log.contains(&(0x6100, vec![device])));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const KEY_TO_INDEX: u32 = 0x0e30_0004;
    const RELEASE: u32 = 0x0e30_0008;
    const MAP_VTABLE: u32 = 0x0e31_0000;
    const DEVICE_VTABLE: u32 = 0x0e32_0000;

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A manager whose first map holds two nodes in buckets 1 and 3 (keys 1 and 3), the first
    /// value with one entry holding a buffer at +0x20.
    fn rig() -> (Engine, u32, [u32; 4]) {
        let mut e = Engine::new();
        for no_op in [
            IMPORT_INTERLOCKED_DECREMENT,
            IMPORT_DELETE_CRITICAL_SECTION,
            FREE_00AA10F0,
            NI_FREE_00AA1460,
            ENTRY_CLEAN_UP_00E95B80,
            MAP_00B601A0,
            RELEASE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(KEY_TO_INDEX, |_, a| a[1].into_ret());
        e.put_vtable(MAP_VTABLE, &[0, KEY_TO_INDEX]);
        e.put_vtable(DEVICE_VTABLE, &[0, 0, RELEASE]);
        let this = e.mem.alloc(0x100);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(this + 8, device);
        e.mem.set_u32(this + 0x40, 0x4040);
        for offset in [0xc, 0x1c, 0x2c] {
            let buckets = e.mem.alloc(0x10);
            e.mem.set_u32(this + offset, MAP_VTABLE);
            e.mem.set_u32(this + offset + 4, 4);
            e.mem.set_u32(this + offset + 8, buckets);
        }
        let buckets = e.mem.u32(this + 0xc + 8);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first + 4, 1);
        e.mem.set_u32(second + 4, 3);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(buckets + 0xc, second);
        let value = e.mem.alloc(0x1c);
        e.mem.set_u32(first + 8, value);
        let entry = e.mem.alloc(0x3c);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, DEVICE_VTABLE);
        e.mem.set_u32(entry + 0x20, buffer);
        e.mem.set_u32(value + 0x10, entry);
        (e, this, [device, value, entry, buffer])
    }

    #[test]
    fn fn_00e8ceb0_frees_the_records_and_undoes_the_maps() {
        let (mut e, this, [device, value, entry, buffer]) = rig();
        e.call_log = Some(vec![]);
        e.call(0x00e8_ceb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, NI_FREE_00AA1460),
            vec![vec![entry, 0x3c], vec![value, 0x1c]]
        );
        assert_eq!(calls(&log, ENTRY_CLEAN_UP_00E95B80), vec![vec![entry]]);
        assert_eq!(calls(&log, RELEASE), vec![vec![buffer], vec![device]]);
        // The walk asked for the bucket of each node without a next.
        assert_eq!(
            calls(&log, KEY_TO_INDEX),
            vec![vec![this + 0xc, 1], vec![this + 0xc, 3]]
        );
        let maps: Vec<u32> = calls(&log, MAP_00B601A0).iter().map(|w| w[0]).collect();
        assert_eq!(
            maps,
            vec![
                this + 0xc,
                this + 0x1c,
                this + 0x2c,
                this + 0x2c,
                this + 0x2c,
                this + 0x1c,
                this + 0x1c,
                this + 0xc,
                this + 0xc
            ]
        );
        assert_eq!(calls(&log, FREE_00AA10F0)[0], vec![0x4040]);
        assert_eq!(calls(&log, FREE_00AA10F0).len(), 4);
        assert!(log.contains(&(IMPORT_DELETE_CRITICAL_SECTION, vec![this + 0x80])));
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), VB_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert!(log.contains(&(IMPORT_INTERLOCKED_DECREMENT, vec![OBJECT_COUNT_011F4400])));
    }

    #[test]
    fn fn_00e8d200_frees_with_bit_0() {
        let (mut e, this, _) = rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_d200, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, NI_FREE_00AA1460).last().unwrap(),
            &vec![this, 0x100]
        );
        let (mut e, this, _) = rig();
        e.call_log = Some(vec![]);
        e.call(0x00e8_d200, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_FREE_00AA1460).len(), 2);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    /// An object whose virtual slots call quiet doubles at `doubles + slot`.
    fn object(e: &mut Engine, doubles: u32, size: u32) -> u32 {
        let table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(table + slot, doubles + slot);
            quiet(e, doubles + slot);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    const BUFFER: u32 = 0x00ed_0000;

    /// A manager with staging fields, a vertex buffer record (+8 the buffer, +0xc..+0x14 the
    /// lock words) whose buffer describes itself as `format` with `bytes` bytes, and a
    /// lock that answers a fresh vertex block.
    fn packer(e: &mut Engine, format: u32, bytes: u32) -> (u32, u32, u32) {
        e.map(SCALE_0101E568, 8);
        e.set_global(SCALE_0101E568, 255.0f64);
        let manager = e.mem.alloc(0x100);
        let staging = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x40, staging);
        let buffer = object(e, BUFFER, 8);
        e.register_double(BUFFER + 0x34, move |e, a| {
            e.mem.set_u32(a[1], 0x64);
            e.mem.set_u32(a[1] + 0x10, bytes);
            e.mem.set_u32(a[1] + 0x14, format);
            word(0)
        });
        let record = e.mem.alloc(0x1c);
        e.mem.set_u32(record + 8, buffer);
        for (i, at) in [0xcu32, 0x10, 0x14].iter().enumerate() {
            e.mem.set_u32(record + at, 0x100 + i as u32);
        }
        let vertices = e.mem.alloc(0x200);
        e.register_double(LOCK_00E8BFF0, move |_, _| word(vertices));
        quiet(e, DATA_SET_FORMAT_00E8F010);
        quiet(e, MEMCPY);
        quiet(e, IMPORT_LEAVE_CRITICAL_SECTION);
        e.call_log = Some(vec![]);
        (manager, record, vertices)
    }

    fn data_with_stream(e: &mut Engine) -> (u32, u32) {
        let data = e.mem.alloc(0x30);
        let first = e.mem.alloc(4);
        e.mem.set_u32(data + 0x20, 1);
        e.mem.set_u32(data + 0x24, first);
        (data, first)
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
        at
    }

    #[test]
    fn fn_00e8c570_packs_positions_colours_and_coordinates() {
        let mut e = Engine::new();
        let (manager, record, vertices) = packer(&mut e, 0x144, 0x38);
        let (data, first) = data_with_stream(&mut e);
        let positions = floats(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        let colours = floats(&mut e, &[1.0, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.25]);
        let coordinates = floats(&mut e, &[0.5, 0.75, 0.25, 0.125]);
        let got = e.call(
            0x00e8_c570,
            &args![
                manager,
                data,
                2u32,
                positions,
                colours,
                coordinates,
                7u32,
                record,
                0u32
            ],
        );
        assert_eq!(got.u32(), record);
        assert_eq!(calls(&e, DATA_SET_FORMAT_00E8F010), vec![vec![data, 0x144]]);
        assert_eq!(e.mem.u32(first), 0x1c);
        // The record fits: no reservation; locked with (+8, +0xc, +0x14, +0x10).
        let buffer = e.mem.u32(record + 8);
        assert_eq!(
            calls(&e, LOCK_00E8BFF0),
            vec![vec![manager, buffer, 0x100, 0x102, 0x101]]
        );
        let v = |i: u32| e.mem.u32(vertices + i);
        assert_eq!(
            [v(0), v(4), v(8), v(0xc)],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 0, 1.0f32.to_bits()]
        );
        // Alpha 255, red 255, green 128 (127.5 rounded to even), blue 0.
        assert_eq!(v(0x10), 0xffff_8000);
        assert_eq!(v(0x14), 0.5f32.to_bits());
        assert_eq!(v(0x1c + 8), 0);
        // Second vertex: alpha 64 (63.75), blue 255.
        assert_eq!(v(0x1c + 0x10), 0x4000_00ff);
        assert_eq!(v(0x1c + 0x18), 0.125f32.to_bits());
        // Unlocked through the staging copy and the buffer's slot 0x30.
        assert_eq!(calls(&e, MEMCPY).len(), 1);
        assert_eq!(calls(&e, BUFFER + 0x30), vec![vec![buffer]]);
    }

    #[test]
    fn fn_00e8c570_reserves_a_new_record_or_uses_the_locked_address() {
        // Too small a buffer: reserve, make a record, lock it.
        let mut e = Engine::new();
        let (manager, record, _) = packer(&mut e, 4, 0x10);
        let (data, _) = data_with_stream(&mut e);
        e.register(RESERVE_00E8BFA0, |_, _| word(1));
        e.register_double(NEW_RECORD_00E69B40, move |_, _| word(record));
        let positions = floats(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        let got = e.call(
            0x00e8_c570,
            &args![manager, data, 2u32, positions, 0u32, 0u32, 1u32, 0u32, 0u32],
        );
        assert_eq!(got.u32(), record);
        assert_eq!(calls(&e, RESERVE_00E8BFA0), vec![vec![manager, data, 0]]);
        // A refused reservation answers 0.
        e.register(RESERVE_00E8BFA0, |_, _| word(0x100));
        let got = e.call(
            0x00e8_c570,
            &args![manager, data, 2u32, positions, 0u32, 0u32, 1u32, 0u32, 0u32],
        );
        assert_eq!(got.u32(), 0);
        // A locked address: written there, no lock and no unlock.
        e.call_log = Some(vec![]);
        let block = e.mem.alloc(0x40);
        let got = e.call(
            0x00e8_c570,
            &args![manager, data, 1u32, positions, 0u32, 0u32, 1u32, record, block],
        );
        assert_eq!(got.u32(), record);
        assert_eq!(e.mem.u32(block + 4), 2.0f32.to_bits());
        assert!(calls(&e, LOCK_00E8BFF0).is_empty());
        assert!(calls(&e, MEMCPY).is_empty());
        // No count: the record comes back untouched.
        e.call_log = Some(vec![]);
        let got = e.call(
            0x00e8_c570,
            &args![
                manager,
                data,
                0x1_0000u32,
                positions,
                0u32,
                0u32,
                1u32,
                0x55u32,
                0u32
            ],
        );
        assert_eq!(got.u32(), 0x55);
        assert!(calls(&e, DATA_SET_FORMAT_00E8F010).is_empty());
    }

    #[test]
    fn fn_00e8c570_answers_zero_when_the_unlock_fails() {
        let mut e = Engine::new();
        let (manager, record, _) = packer(&mut e, 4, 0x40);
        let (data, _) = data_with_stream(&mut e);
        e.register(BUFFER + 0x30, |_, _| word(0x8876_0001));
        let positions = floats(&mut e, &[1.0, 2.0]);
        let got = e.call(
            0x00e8_c570,
            &args![manager, data, 1u32, positions, 0u32, 0u32, 0u32, record, 0u32],
        );
        assert_eq!(got.u32(), 0);
    }

    /// A skinned partition of `vertices` vertices with `weights` weights each.
    fn partition(e: &mut Engine, vertices: u16, weights: u16, values: &[f32]) -> (u32, u32) {
        let partition = e.mem.alloc(0x30);
        e.mem.set_u16(partition + 0x1c, vertices);
        e.mem.set_u16(partition + 0x20, weights);
        let indices = e.mem.alloc(2 * u32::from(vertices) + 2);
        for i in 0..u32::from(vertices) {
            e.mem
                .set_u16(indices + 2 * i, (u32::from(vertices) - 1 - i) as u16);
        }
        e.mem.set_u32(partition + 0xc, indices);
        let weight_values = floats(e, values);
        e.mem.set_u32(partition + 8, weight_values);
        let (stream, first) = data_with_stream(e);
        e.mem.set_u32(partition + 0x28, stream);
        (partition, first)
    }

    /// A layout double: flags 0x15a, size 0x30, offsets 0, 0xc, 0x18, 0x24, 0x28.
    fn layout(e: &mut Engine) {
        e.register(LAYOUT_00E8C120, |e, a| {
            for (at, value) in a[3..]
                .iter()
                .zip([0x15au32, 0x30, 0, 0xc, 0x18, 0x24, 0x28])
            {
                e.mem.set_u32(*at, value);
            }
            Ret::default()
        });
    }

    #[test]
    fn fn_00e8c820_packs_a_skinned_partition() {
        let mut e = Engine::new();
        e.map(TEXTURE_SET_LIMIT_011BF558, 4);
        e.set_global(TEXTURE_SET_LIMIT_011BF558, 4u32);
        e.map(WEIGHT_TOLERANCE_010718C0, 4);
        e.set_global(WEIGHT_TOLERANCE_010718C0, 1e-5f32);
        let (manager, record, vertices) = packer(&mut e, 0x15a, 0x60);
        layout(&mut e);
        let data = e.mem.alloc(0x30);
        let positions = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let normals = floats(&mut e, &[0.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
        let colours = floats(&mut e, &[1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x24, normals);
        e.mem.set_u32(data + 0x28, colours);
        e.mem.set_u16(data + 0xc, 1);
        let set = floats(&mut e, &[0.1, 0.2, 0.3, 0.4]);
        e.register_double(GET_TEXTURE_SET, move |_, _| word(set));
        // Two vertices of three weights: the second sums to 0.9 (illegal).
        let (part, first) = partition(&mut e, 2, 3, &[0.5, 0.25, 0.25, 0.5, 0.25, 0.15]);
        quiet(&mut e, NI_SPRINTF);
        let got = e.call(
            0x00e8_c820,
            &args![manager, data, 0u32, part, 0xfu32, record, 0u32],
        );
        assert_eq!(got.u32(), record);
        let stream = e.mem.u32(part + 0x28);
        assert_eq!(
            calls(&e, DATA_SET_FORMAT_00E8F010),
            vec![vec![stream, 0x15a]]
        );
        assert_eq!(e.mem.u32(first), 0x30);
        let v = |i: u32| f32::from_bits(e.mem.u32(vertices + i));
        // Vertex 0 is data vertex 1 (indices run backwards).
        assert_eq!([v(0), v(4), v(8)], [4.0, 5.0, 6.0]);
        assert_eq!([v(0x30), v(0x34), v(0x38)], [1.0, 2.0, 3.0]);
        // All three weights (count 3).
        assert_eq!([v(0xc), v(0x10), v(0x14)], [0.5, 0.25, 0.25]);
        assert_eq!(v(0x3c + 8), 0.15);
        assert_eq!([v(0x18), v(0x1c), v(0x20)], [0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u32(vertices + 0x24), 0xff00_ff00);
        assert_eq!(e.mem.u32(vertices + 0x30 + 0x24), 0xffff_0000);
        assert_eq!([v(0x28), v(0x2c)], [0.3, 0.4]);
        let prints = calls(&e, NI_SPRINTF);
        assert_eq!(prints.len(), 1);
        assert_eq!(&prints[0][1..4], &[0x100, ILLEGAL_WEIGHTS_MESSAGE, 1]);
        assert_eq!(calls(&e, GET_TEXTURE_SET), vec![vec![data, 0]]);
        assert_eq!(calls(&e, MEMCPY).len(), 1);
    }

    #[test]
    fn fn_00e8c820_leaves_out_the_implicit_last_weight_and_reserves() {
        let mut e = Engine::new();
        e.map(TEXTURE_SET_LIMIT_011BF558, 4);
        e.map(WEIGHT_TOLERANCE_010718C0, 4);
        e.set_global(WEIGHT_TOLERANCE_010718C0, 1e-5f32);
        let (manager, record, vertices) = packer(&mut e, 0x15a, 0);
        layout(&mut e);
        let data = e.mem.alloc(0x30);
        let positions = floats(&mut e, &[1.0, 2.0, 3.0]);
        e.mem.set_u32(data + 0x20, positions);
        // Four weights summing to 1: the fourth is not written.
        let (part, _) = partition(&mut e, 1, 4, &[0.25, 0.25, 0.25, 0.25]);
        let stream = e.mem.u32(part + 0x28);
        // Reserved: the stream object's first record (+0x28) is used and the mask becomes
        // 0xfff.
        let records = e.mem.alloc(4);
        e.mem.set_u32(records, record);
        e.mem.set_u32(stream + 0x28, records);
        e.register(RESERVE_00E8BFA0, |_, _| word(1));
        let got = e.call(
            0x00e8_c820,
            &args![manager, data, 0u32, part, 0u32, record, 0u32],
        );
        assert_eq!(got.u32(), record);
        assert_eq!(calls(&e, RESERVE_00E8BFA0), vec![vec![manager, stream, 0]]);
        let v = |i: u32| f32::from_bits(e.mem.u32(vertices + i));
        assert_eq!([v(0), v(0xc), v(0x10), v(0x14)], [1.0, 0.25, 0.25, 0.25]);
        assert_eq!(e.mem.u32(vertices + 0x18), 0);
        assert!(calls(&e, NI_SPRINTF).is_empty());
        // A refused reservation answers 0; a locked address skips the lock.
        e.register(RESERVE_00E8BFA0, |_, _| word(0));
        let got = e.call(
            0x00e8_c820,
            &args![manager, data, 0u32, part, 0u32, record, 0u32],
        );
        assert_eq!(got.u32(), 0);
        e.call_log = Some(vec![]);
        let block = e.mem.alloc(0x40);
        let got = e.call(
            0x00e8_c820,
            &args![manager, data, 0u32, part, 1u32, 0x77u32, block],
        );
        assert_eq!(got.u32(), 0x77);
        assert_eq!(f32::from_bits(e.mem.u32(block + 8)), 3.0);
        assert!(calls(&e, LOCK_00E8BFF0).is_empty());
        assert!(calls(&e, MEMCPY).is_empty());
    }

    #[test]
    fn fn_00e8cd60_frees_every_record_of_the_first_map() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc + 4, 2);
        e.mem.set_u32(this + 0xc + 8, buckets);
        let held = object(&mut e, 0x00ec_0000, 8);
        let entry = e.mem.alloc(0x3c);
        e.mem.set_u32(entry + 0x20, held);
        let bare = e.mem.alloc(0x3c);
        let record = e.mem.alloc(0x1c);
        e.mem.set_u32(record + 8, entry);
        e.mem.set_u32(record + 0x18, bare);
        e.mem.set_u32(buckets + 4, 0x0e0a_0000);
        // `GetNext`: the first item gives the record and ends the walk; nothing else.
        e.register_double(MAP_ITERATE_00B57F20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], record);
            Ret::default()
        });
        quiet(&mut e, ENTRY_CLEAN_UP_00E95B80);
        quiet(&mut e, NI_FREE_00AA1460);
        quiet(&mut e, MAP_00B601A0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_cd60, &args![this]);
        assert_eq!(calls(&e, MAP_ITERATE_00B57F20).len(), 1);
        assert_eq!(calls(&e, 0x00ec_0008), vec![vec![held]]);
        assert_eq!(
            calls(&e, ENTRY_CLEAN_UP_00E95B80),
            vec![vec![entry], vec![bare]]
        );
        assert_eq!(
            calls(&e, NI_FREE_00AA1460),
            vec![vec![entry, 0x3c], vec![bare, 0x3c], vec![record, 0x1c]]
        );
        assert_eq!(calls(&e, MAP_00B601A0), vec![vec![this + 0xc]]);
        // An empty map only runs the clean-up.
        let empty = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00e8_cd60, &args![empty]);
        assert!(calls(&e, MAP_ITERATE_00B57F20).is_empty());
        assert_eq!(calls(&e, MAP_00B601A0), vec![vec![empty + 0xc]]);
    }

    #[test]
    fn buffer_map_deleting_destructors_free_on_bit_zero() {
        for (address, tables) in [
            (0x00e8_ce30u32, vec![VB_MAP_BASE_VTABLE]),
            (0x00e8_ce70, vec![VB_MAP_VTABLE, VB_MAP_BASE_VTABLE]),
        ] {
            for (flags, frees) in [(0u32, false), (1, true)] {
                let mut e = Engine::new();
                let seen = Rc::new(RefCell::new(vec![]));
                let tables_seen = seen.clone();
                e.register_double(MAP_00B601A0, move |e, a| {
                    tables_seen.borrow_mut().push(e.mem.u32(a[0]));
                    Ret::default()
                });
                quiet(&mut e, FREE_00AA10F0);
                quiet(&mut e, MEMORY_DEALLOCATE);
                let this = e.mem.alloc(0x10);
                e.mem.set_u32(this + 8, 0x0e0b_0000);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(*seen.borrow(), tables);
                assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e0b_0000]]);
                let freed = calls(&e, MEMORY_DEALLOCATE);
                if frees {
                    assert_eq!(freed, vec![vec![MEMORY_MANAGER, this]]);
                } else {
                    assert!(freed.is_empty());
                }
            }
        }
    }
}

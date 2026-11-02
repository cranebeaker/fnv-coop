//! `Havok SDK/hkxvertexbuffer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e3e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_e760`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e3e7b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_e760, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// The layout used below is the PC code's own (the Xbox PDB has no type for these classes; the
// meaning of each field is named by its role and is unconfirmed). A vertex buffer keeps five
// `hkArray`s of vertex data: bytes at `+0x38`, 16-bit words at `+0x2c`, 32-bit words at
// `+0x20`, floats at `+0x14` and 16-byte vectors at `+0x8` (each: data, size, capacity word),
// its vertex count at `+0x44`, one stride in bytes per data array at `+0x58` (bytes), `+0x54`
// (words), `+0x50`, `+0x4c` (floats) and `+0x48` (vectors), and its vertex description (an
// array of 12-byte elements: data `+0x5c`, size `+0x60`, capacity word `+0x64`). A description
// element holds an offset word at `+0`, a type at `+4`, a usage word at `+6` and a stride at
// `+8`.

/// `hkxVertexDescription::operator_` (`00e3dee0`): `this` equal to the argument.
const DESCRIPTION_EQUALS: u32 = 0x00e3_dee0;
/// `hkArrayUtil` growth helper (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkString::memSet` (`00cb1280`, cdecl: destination, value, size).
const MEM_SET: u32 = 0x00cb_1280;
/// `hkString::memCpy` (`00cb1260`, cdecl: destination, source, size).
const MEM_COPY: u32 = 0x00cb_1260;
/// The destructor of the description array's owner part (`00dfd680`, `this` = `buffer + 8`).
const DATA_ARRAYS_DESTRUCT: u32 = 0x00df_d680;
/// The vtable word of `hkBaseObject` (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;
/// The vtable word `00e3e380` writes (`010eba94`).
const VERTEX_BUFFER_VTABLE: u32 = 0x010e_ba94;
/// The mask of the capacity bits of an `hkArray`'s capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator (`00c85750`, field `+0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The data pointer of the vertex array a description element of `kind` addresses, as an
/// offset into the buffer: bytes `+0x38`, words `+0x2c`, 32-bit `+0x20`, floats (kinds 4 and
/// 5) `+0x14`, vectors (6 and 7) `+0x8`; `None` for any other kind.
fn data_array_offset(kind: u16) -> Option<u32> {
    match kind {
        1 => Some(0x38),
        2 => Some(0x2c),
        3 => Some(0x20),
        4 | 5 => Some(0x14),
        6 | 7 => Some(0x8),
        _ => None,
    }
}

/// The `(array offset, stride offset, shift, element size)` of the five vertex data arrays in
/// the order `00e3e3a0` resizes them: the array's size is `stride * count >> shift`.
const DATA_ARRAYS: [(u32, u32, u32, u32); 5] = [
    (0x38, 0x58, 0, 1),
    (0x2c, 0x54, 1, 2),
    (0x20, 0x50, 2, 4),
    (0x14, 0x4c, 2, 4),
    (0x08, 0x48, 4, 0x10),
];

// Translated from 00e3df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `this` and one word it never reads and returns `this` (the `hkxVertexDescription`
/// constructor `00e3e380` calls on `buffer + 0x5c`; the class is unconfirmed).
pub fn fn_00e3df50(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 00e3df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexDescription::getElementDecl` (Xbox PDB): the address of the `index`th element
/// (counting only the elements whose 16-bit word at `+6` equals `usage`) of the description's
/// array (data `+0`, size `+4`, elements `0xc` bytes), or 0 when there are fewer.
pub fn hkx_vertex_description_get_element_decl(
    e: &mut Engine,
    this: Ptr,
    usage: u16,
    index: i32,
) -> u32 {
    let size = e.mem.i32(this.addr() + 4);
    let mut found = 0;
    let mut position = 0;
    if size > 0 {
        let data = e.mem.u32(this.addr());
        let mut word = data.wrapping_add(6);
        loop {
            if e.mem.u16(word) == usage {
                if found == index {
                    return data.wrapping_add(position * 0xc);
                }
                found += 1;
            }
            position += 1;
            word = word.wrapping_add(0xc);
            if position as i32 >= size {
                break;
            }
        }
    }
    0
}

// Translated from 00e3dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexBuffer::getVertexDataPtr` (Xbox PDB): the start of the vertex data a description
/// element addresses: the element's offset word (`+0`) added to the data pointer of the array
/// its 16-bit type (`+4`) selects (1: bytes, 2: words, 3: 32-bit words, 4 and 5: floats, 6 and
/// 7: vectors); 0 for any other type.
pub fn hkx_vertex_buffer_get_vertex_data_ptr(e: &mut Engine, this: Ptr, element: Ptr) -> u32 {
    match data_array_offset(e.mem.u16(element.addr() + 4)) {
        Some(array) => {
            let data = e.mem.u32(this.addr() + array);
            e.mem.u32(element.addr()).wrapping_add(data)
        }
        None => 0,
    }
}

// Translated from 00e3e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `JMP` to `00e3dfb0` (the linker's identical-code copy of `getVertexDataPtr`, here the
/// version for a const buffer): the same function.
pub fn fn_00e3e020(e: &mut Engine, this: Ptr, element: Ptr) -> u32 {
    hkx_vertex_buffer_get_vertex_data_ptr(e, this, element)
}

// Translated from 00e3e030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexBuffer::copyVertex` (Xbox PDB): copies one vertex of `other` (index `source`)
/// over one vertex of `this` (index `destination`). Nothing happens unless `destination <
/// this.count` and `source < other.count` (signed). With equal descriptions the five data
/// arrays are copied from the start of `other`'s into the start of `this`'s, each `stride`
/// bytes long (the exe does not offset by the vertex index there). Otherwise, for every
/// element of this buffer's description in turn, the matching element of `other`'s (the
/// `n`th element with the same usage word, `n` counted per usage in a local table of 257
/// words, indexed by the 16-bit usage word as the exe does) is found with `getElementDecl`
/// and, when the types agree, `4`, `8` or `16` bytes (by type: 1-4, 5, 6-7; 0 for others) are
/// copied from the other buffer's vertex data to this one's, each at its stride times the
/// vertex index.
pub fn hkx_vertex_buffer_copy_vertex(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    source: i32,
    destination: i32,
) {
    let (base, other_base) = (this.addr(), other.addr());
    let same = e
        .call(DESCRIPTION_EQUALS, &args![base + 0x5c, other_base + 0x5c])
        .bool();
    e.with_stack(0x410, |e, counts| {
        let counts = counts.addr();
        e.call(MEM_SET, &args![counts, 0u32, 0x404u32]);
        if destination < e.mem.i32(base + 0x44) && source < e.mem.i32(other_base + 0x44) {
            if same {
                for (array, stride, _, _) in DATA_ARRAYS {
                    let size = e.mem.u32(base + stride);
                    if size > 0 {
                        let to = e.mem.u32(base + array);
                        let from = e.mem.u32(other_base + array);
                        e.call(MEM_COPY, &args![to, from, size]);
                    }
                }
            } else if e.mem.i32(base + 0x60) > 0 {
                let mut offset = 0u32;
                let mut index = 0;
                loop {
                    let element = e.mem.u32(base + 0x5c).wrapping_add(offset);
                    let usage = e.mem.u16(element + 6);
                    let slot = counts.wrapping_add(u32::from(usage) * 4);
                    let seen = e.mem.u32(slot);
                    let found = hkx_vertex_description_get_element_decl(
                        e,
                        Ptr::new(other_base + 0x5c),
                        usage,
                        seen as i32,
                    );
                    if found != 0 && e.mem.u16(element + 4) == e.mem.u16(found + 4) {
                        let seen = e.mem.u32(slot);
                        e.mem.set_u32(slot, seen.wrapping_add(1));
                        let mine =
                            hkx_vertex_buffer_get_vertex_data_ptr(e, this, Ptr::new(element));
                        let theirs = fn_00e3e020(e, other, Ptr::new(found));
                        let size = match e.mem.u16(element + 4) {
                            1..=4 => 4,
                            5 => 8,
                            6 | 7 => 0x10,
                            _ => 0,
                        };
                        let to = e
                            .mem
                            .u32(element + 8)
                            .wrapping_mul(destination as u32)
                            .wrapping_add(mine);
                        let from = e
                            .mem
                            .u32(found + 8)
                            .wrapping_mul(source as u32)
                            .wrapping_add(theirs);
                        e.call(MEM_COPY, &args![to, from, size]);
                    }
                    offset = offset.wrapping_add(0xc);
                    index += 1;
                    if index >= e.mem.i32(base + 0x60) {
                        break;
                    }
                }
            }
        }
    });
}

// Translated from 00e3e220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the sizes and strides of a vertex buffer's data (`this` is `buffer + 8`, in `ECX`):
/// the five array sizes (`buffer + 0xc`, `+0x18`, `+0x24`, `+0x30`, `+0x3c`), the vertex count
/// (`buffer + 0x44`) and the five strides (`buffer + 0x48` to `+0x58`) all become 0.
pub fn fn_00e3e220(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [
        0x4, 0x10, 0x1c, 0x28, 0x34, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
}

// Translated from 00e3e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable word `010eba94` at `this`, runs `00e3df50` on the description at `this +
/// 0x5c` with `argument`, and returns `this` (a vertex buffer constructor; the class is
/// unconfirmed).
pub fn fn_00e3e380(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VERTEX_BUFFER_VTABLE);
    fn_00e3df50(e, Ptr::new(this.addr() + 0x5c), argument);
    this
}

// Translated from 00e3e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexBuffer::setNumVertices` (Xbox PDB): does nothing when `count` equals the current
/// vertex count and `description` equals this buffer's own. Otherwise this buffer's
/// description is emptied and the vertex data cleared (`00e3e220`), then it is rebuilt from
/// the elements of `description` (read again at every step, so an argument that is this
/// buffer's own description is already empty): each element is appended (growing through
/// `00c90940`, zeroed first) with its type and usage copied and its offset set to the running
/// size of the data array its type selects, which then grows by 4 bytes (types 1-4), 8
/// (type 5, shares the float array) or 16 (types 6-7). Every element's stride word is then set
/// to the final stride of its array. Finally the count is stored and the five data arrays are
/// resized to `stride * count` elements of their size (reserving `max(2 * capacity, needed)`
/// through `00c908c0` when the capacity is short).
pub fn hkx_vertex_buffer_set_num_vertices(e: &mut Engine, this: Ptr, count: i32, description: Ptr) {
    let base = this.addr();
    let wanted = description.addr();
    if count == e.mem.i32(base + 0x44)
        && e.call(DESCRIPTION_EQUALS, &args![wanted, base + 0x5c])
            .bool()
    {
        return;
    }
    let own = base + 0x5c;
    e.mem.set_u32(own + 4, 0);
    fn_00e3e220(e, Ptr::new(base + 8));
    if e.mem.i32(wanted + 4) > 0 {
        let mut offset = 0u32;
        let mut index = 0;
        loop {
            let source = e.mem.u32(wanted).wrapping_add(offset);
            let allocator = memory_allocator(e);
            let size = e.mem.u32(own + 4);
            if size == e.mem.u32(own + 8) & CAPACITY_MASK {
                e.call(ARRAY_GROW, &args![allocator, own, 0xcu32]);
            }
            let size = e.mem.u32(own + 4);
            let slot = e.mem.u32(own).wrapping_add(size.wrapping_mul(0xc));
            if slot != 0 {
                e.mem.set_u32(slot, 0);
                e.mem.set_u16(slot + 4, 0);
                e.mem.set_u16(slot + 6, 0);
                e.mem.set_u32(slot + 8, 0);
            }
            let size = e.mem.u32(own + 4);
            let slot = e.mem.u32(own).wrapping_add(size.wrapping_mul(0xc));
            e.mem.set_u32(own + 4, size.wrapping_add(1));
            let kind = e.mem.u16(source + 4);
            e.mem.set_u16(slot + 4, kind);
            let usage = e.mem.u16(source + 6);
            e.mem.set_u16(slot + 6, usage);
            if let Some((array, step)) = data_stride_step(kind) {
                let running = e.mem.u32(base + array);
                e.mem.set_u32(slot, running);
                e.mem.set_u32(base + array, running.wrapping_add(step));
            }
            offset = offset.wrapping_add(0xc);
            index += 1;
            if index >= e.mem.i32(wanted + 4) {
                break;
            }
        }
    }
    if e.mem.i32(base + 0x60) > 0 {
        let mut offset = 0u32;
        let mut index = 0;
        loop {
            let element = e.mem.u32(own).wrapping_add(offset);
            if let Some((array, _)) = data_stride_step(e.mem.u16(element + 4)) {
                let stride = e.mem.u32(base + array);
                e.mem.set_u32(element + 8, stride);
            }
            index += 1;
            offset = offset.wrapping_add(0xc);
            if index >= e.mem.i32(base + 0x60) {
                break;
            }
        }
    }
    e.mem.set_i32(base + 0x44, count);
    for (array, stride, shift, element_size) in DATA_ARRAYS {
        let needed = ((e.mem.u32(base + stride).wrapping_mul(count as u32)) >> shift) as i32;
        let allocator = memory_allocator(e);
        let capacity = (e.mem.u32(base + array + 8) & CAPACITY_MASK) as i32;
        if capacity < needed {
            let mut request = capacity.wrapping_add(capacity);
            if needed >= request {
                request = needed;
            }
            e.call(
                ARRAY_RESERVE,
                &args![allocator, base + array, request as u32, element_size],
            );
        }
        e.mem.set_i32(base + array + 4, needed);
    }
}

/// The stride word a description element of `kind` advances (`buffer + 0x58` for bytes,
/// `0x54` words, `0x50` 32-bit words, `0x4c` floats and 8-byte pairs, `0x48` vectors) and by
/// how many bytes (4; 8 for kind 5; 16 for kinds 6 and 7); `None` for any other kind.
fn data_stride_step(kind: u16) -> Option<(u32, u32)> {
    match kind {
        1 => Some((0x58, 4)),
        2 => Some((0x54, 4)),
        3 => Some((0x50, 4)),
        4 => Some((0x4c, 4)),
        5 => Some((0x4c, 8)),
        6 | 7 => Some((0x48, 0x10)),
        _ => None,
    }
}

// Translated from 00e3e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxVertexBuffer::copy` (Xbox PDB): copies the vertices of `other` into this buffer. With
/// `resize` set, first calls `setNumVertices(other.count, this.description)` (the exe passes
/// this buffer's own description). With equal descriptions the five data arrays are copied in
/// one piece, `stride * min(this.count, other.count)` bytes each (a stride of 0 copies
/// nothing); otherwise every vertex `0..other.count` is copied with `copyVertex`.
pub fn hkx_vertex_buffer_copy(e: &mut Engine, this: Ptr, other: Ptr, resize: u8) {
    let (base, other_base) = (this.addr(), other.addr());
    let same = e
        .call(DESCRIPTION_EQUALS, &args![base + 0x5c, other_base + 0x5c])
        .bool();
    if resize != 0 {
        let count = e.mem.i32(other_base + 0x44);
        hkx_vertex_buffer_set_num_vertices(e, this, count, Ptr::new(base + 0x5c));
    }
    if same {
        let mut count = e.mem.u32(other_base + 0x44);
        let mine = e.mem.u32(base + 0x44);
        if count >= mine {
            count = mine;
        }
        for (array, stride, _, _) in DATA_ARRAYS {
            let size = e.mem.u32(base + stride);
            if size > 0 {
                let to = e.mem.u32(base + array);
                let from = e.mem.u32(other_base + array);
                e.call(MEM_COPY, &args![to, from, size.wrapping_mul(count)]);
            }
        }
    } else {
        let mut index = 0u32;
        if e.mem.u32(other_base + 0x44) > 0 {
            loop {
                hkx_vertex_buffer_copy_vertex(e, this, other, index as i32, index as i32);
                index = index.wrapping_add(1);
                if index >= e.mem.u32(other_base + 0x44) {
                    break;
                }
            }
        }
    }
}

// Translated from 00e3e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a vertex buffer (the class is unconfirmed; `deleting_destructor_00e3e7b0`
/// calls it): empties the description array (data `+0x5c`, size `+0x60`, capacity word
/// `+0x64`, `0xc`-byte elements): size 0 and, unless the capacity word's top bit says the
/// memory is not owned, the data returned to the memory router's allocator (slot `0x10`,
/// `(capacity & 0x3fffffff) * 12` bytes); data pointer cleared, capacity word `0x80000000`.
/// Then `00dfd680` runs on `this + 8` and the vtable word `0102e388` is written.
pub fn fn_00e3e760(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(base + 0x64);
    e.mem.set_u32(base + 0x60, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(base + 0x5c);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(12);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x64, 0x8000_0000);
    e.call(DATA_ARRAYS_DESTRUCT, &args![base + 8]);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_df50, fn_00e3df50(Ptr, u32) -> Ptr),
        entry!(
            0x00e3_df60,
            hkx_vertex_description_get_element_decl(Ptr, u16, i32) -> u32
        ),
        entry!(
            0x00e3_dfb0,
            hkx_vertex_buffer_get_vertex_data_ptr(Ptr, Ptr) -> u32
        ),
        entry!(0x00e3_e020, fn_00e3e020(Ptr, Ptr) -> u32),
        entry!(
            0x00e3_e030,
            hkx_vertex_buffer_copy_vertex(Ptr, Ptr, i32, i32)
        ),
        entry!(0x00e3_e220, fn_00e3e220(Ptr)),
        entry!(0x00e3_e380, fn_00e3e380(Ptr, u32) -> Ptr),
        entry!(
            0x00e3_e3a0,
            hkx_vertex_buffer_set_num_vertices(Ptr, i32, Ptr)
        ),
        entry!(0x00e3_e670, hkx_vertex_buffer_copy(Ptr, Ptr, u8)),
        entry!(0x00e3_e760, fn_00e3e760(Ptr)),
        entry!(0x00e3_e7b0, deleting_destructor_00e3e7b0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e3e7b0_frees_only_when_bit_zero_is_set() {
        check(0x00e3_e7b0, 0x00e3_e760);
    }

    const ARRAY_FREE_DOUBLE: u32 = 0x0900_0021;

    /// Moves an array's elements to a new block of `capacity` elements (the part of the array
    /// allocator the exe's callees play) and stores the capacity word.
    fn relocate(e: &mut Engine, array: u32, capacity: u32, element_size: u32) {
        let new = e.mem.alloc(capacity * element_size);
        let size = e.mem.u32(array + 4);
        if size <= capacity {
            let old = e.mem.u32(array);
            let bytes = e.mem.bytes(old, size * element_size);
            e.mem.write(new, &bytes);
        }
        e.mem.set_u32(array, new);
        e.mem.set_u32(array + 8, capacity);
    }

    /// An engine whose memory router frees through a double, with real-copying doubles of
    /// `hkString::memCpy`/`memSet` and array doubles that relocate: growth by one step of 4
    /// or double the capacity, and a reserve of exactly the requested capacity.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ARRAY_FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register_double(MEM_SET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        e.register_double(ARRAY_GROW, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & CAPACITY_MASK;
            relocate(e, a[1], (capacity * 2).max(4), a[2]);
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE, |e, a| {
            relocate(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e
    }

    /// A free-standing description with the given `(type, usage)` elements (offset and
    /// stride 0).
    fn wanted_description(e: &mut Engine, elements: &[(u16, u16)]) -> u32 {
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0xc * elements.len().max(1) as u32);
        for (i, &(kind, usage)) in elements.iter().enumerate() {
            let at = data + 0xc * i as u32;
            e.mem.set_u16(at + 4, kind);
            e.mem.set_u16(at + 6, usage);
        }
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, elements.len() as u32);
        this
    }

    /// A vertex buffer of `count` vertices of the given `(type, usage)` elements, made by
    /// `setNumVertices`.
    fn buffer(e: &mut Engine, elements: &[(u16, u16)], count: i32) -> u32 {
        let wanted = wanted_description(e, elements);
        let buffer = e.mem.alloc(0x70);
        e.call(0x00e3_e3a0, &args![buffer, count, wanted]);
        buffer
    }

    /// The `(offset, type, usage, stride)` of description element `index` of a buffer.
    fn element(e: &Engine, buffer: u32, index: u32) -> (u32, u16, u16, u32) {
        let at = e.mem.u32(buffer + 0x5c) + 0xc * index;
        (
            e.mem.u32(at),
            e.mem.u16(at + 4),
            e.mem.u16(at + 6),
            e.mem.u32(at + 8),
        )
    }

    /// Fills `len` bytes of the data array at `array` with `seed, seed + 1, ...`.
    fn fill(e: &mut Engine, buffer: u32, array: u32, len: u32, seed: u8) {
        let data = e.mem.u32(buffer + array);
        for i in 0..len {
            e.mem.set_u8(data + i, seed.wrapping_add(i as u8));
        }
    }

    fn bytes_of(e: &Engine, buffer: u32, array: u32, len: u32) -> Vec<u8> {
        e.mem.bytes(e.mem.u32(buffer + array), len)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// The `index`th element with a usage is found; a missing one gives 0.
    #[test]
    fn get_element_decl_counts_elements_of_one_usage() {
        let mut e = rig();
        let description = wanted_description(&mut e, &[(6, 1), (4, 2), (6, 1)]);
        let data = e.mem.u32(description);
        let find = |e: &mut Engine, usage: u32, index: u32| {
            e.call(0x00e3_df60, &args![description, usage, index]).u32()
        };
        assert_eq!(find(&mut e, 1, 0), data);
        assert_eq!(find(&mut e, 1, 1), data + 0x18);
        assert_eq!(find(&mut e, 1, 2), 0);
        assert_eq!(find(&mut e, 2, 0), data + 0xc);
        assert_eq!(find(&mut e, 3, 0), 0);
        // Only the low 16 bits of the usage word are compared.
        assert_eq!(find(&mut e, 0x1_0002, 0), data + 0xc);
        let empty = wanted_description(&mut e, &[]);
        assert_eq!(e.call(0x00e3_df60, &args![empty, 1u32, 0u32]).u32(), 0);
    }

    /// Each element type selects its array; the element's offset word is added; both entry
    /// points agree.
    #[test]
    fn get_vertex_data_ptr_selects_the_array_by_type() {
        let mut e = rig();
        let this = e.mem.alloc(0x70);
        for (offset, value) in [
            (0x38, 0x1000),
            (0x2c, 0x2000),
            (0x20, 0x3000),
            (0x14, 0x4000),
            (0x08, 0x5000),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        let element = e.mem.alloc(0x10);
        e.mem.set_u32(element, 0x24);
        for (kind, expected) in [
            (1, 0x1024),
            (2, 0x2024),
            (3, 0x3024),
            (4, 0x4024),
            (5, 0x4024),
            (6, 0x5024),
            (7, 0x5024),
            (0, 0),
            (8, 0),
        ] {
            e.mem.set_u16(element + 4, kind);
            assert_eq!(e.call(0x00e3_dfb0, &args![this, element]).u32(), expected);
            assert_eq!(e.call(0x00e3_e020, &args![this, element]).u32(), expected);
        }
    }

    /// The clearing function zeroes exactly the eleven words it names.
    #[test]
    fn clear_zeroes_the_sizes_counts_and_strides() {
        let mut e = rig();
        let this = e.mem.alloc(0x70);
        for offset in (0..0x70).step_by(4) {
            e.mem.set_u32(this + offset, 0x1111_1111);
        }
        e.call(0x00e3_e220, &args![this + 8]);
        for offset in (8..0x70).step_by(4) {
            let cleared = [
                0xc, 0x18, 0x24, 0x30, 0x3c, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58,
            ];
            let expected = if cleared.contains(&offset) {
                0
            } else {
                0x1111_1111
            };
            assert_eq!(e.mem.u32(this + offset), expected, "offset {offset:#x}");
        }
    }

    /// The constructor writes its vtable word and returns `this`; the no-op returns `this`.
    #[test]
    fn constructor_writes_the_vtable_word() {
        let mut e = rig();
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00e3_e380, &args![this, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VERTEX_BUFFER_VTABLE);
        assert_eq!(
            e.call(0x00e3_df50, &args![this + 0x5c, 9u32]).u32(),
            this + 0x5c
        );
    }

    /// The description is rebuilt with running offsets and final strides, the arrays are
    /// sized and reserved, a buffer already in the wanted state is left alone, and a larger
    /// count reserves the larger of double the capacity and what is needed.
    #[test]
    fn set_num_vertices_rebuilds_description_strides_and_arrays() {
        let mut e = rig();
        let wanted = wanted_description(
            &mut e,
            &[(6, 1), (4, 2), (5, 3), (1, 4), (3, 5), (2, 6), (9, 7)],
        );
        let this = e.mem.alloc(0x70);
        e.call_log = Some(vec![]);
        e.call(0x00e3_e3a0, &args![this, 3i32, wanted]);
        assert_eq!(e.mem.u32(this + 0x60), 7);
        let expected = [
            (0, 6, 1, 16),
            (0, 4, 2, 12),
            (4, 5, 3, 12),
            (0, 1, 4, 4),
            (0, 3, 5, 4),
            (0, 2, 6, 4),
            (0, 9, 7, 0),
        ];
        for (i, want) in expected.iter().enumerate() {
            assert_eq!(element(&e, this, i as u32), *want, "element {i}");
        }
        assert_eq!(e.mem.u32(this + 0x44), 3);
        for (stride, value) in [(0x48, 16), (0x4c, 12), (0x50, 4), (0x54, 4), (0x58, 4)] {
            assert_eq!(e.mem.u32(this + stride), value);
        }
        for (size, value) in [(0x3c, 12), (0x30, 6), (0x24, 3), (0x18, 9), (0xc, 3)] {
            assert_eq!(e.mem.u32(this + size), value);
        }
        // The description grew twice (at sizes 0 and 4); the data arrays were reserved with
        // exactly what was needed, the empty one not at all.
        let grows = calls_to(&e, ARRAY_GROW);
        assert_eq!(grows.len(), 2);
        assert_eq!(grows[0][1..], [this + 0x5c, 0xc]);
        let reserves: Vec<Vec<u32>> = calls_to(&e, ARRAY_RESERVE)
            .into_iter()
            .map(|a| a[1..].to_vec())
            .collect();
        assert_eq!(
            reserves,
            vec![
                vec![this + 0x38, 12, 1],
                vec![this + 0x2c, 6, 2],
                vec![this + 0x20, 3, 4],
                vec![this + 0x14, 9, 4],
                vec![this + 8, 3, 0x10],
            ]
        );
        // Already in the wanted state: its own description and the same count change nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e3_e3a0, &args![this, 3i32, this + 0x5c]);
        let log = e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(at, _)| *at).collect();
        assert_eq!(addresses, vec![0x00e3_e3a0, 0x00e3_dee0]);
        // A larger count: capacity short everywhere, so max(2 * capacity, needed) is asked for.
        e.call_log = Some(vec![]);
        e.call(0x00e3_e3a0, &args![this, 4i32, wanted]);
        let reserves: Vec<u32> = calls_to(&e, ARRAY_RESERVE).iter().map(|a| a[2]).collect();
        assert_eq!(reserves, vec![24, 12, 6, 18, 6]);
        assert_eq!(e.mem.u32(this + 0x44), 4);
        assert_eq!(e.mem.u32(this + 0x3c), 16);
    }

    /// Different descriptions: every element of this buffer takes the vertex of the matching
    /// usage and type from the other buffer, at each buffer's stride times its vertex index;
    /// out-of-range vertices copy nothing.
    #[test]
    fn copy_vertex_copies_matching_elements_at_their_strides() {
        let mut e = rig();
        let source = buffer(&mut e, &[(4, 2), (6, 1), (1, 3)], 2);
        let this = buffer(&mut e, &[(6, 1), (4, 2), (2, 9), (1, 3)], 3);
        fill(&mut e, source, 0x14, 8, 0x10);
        fill(&mut e, source, 0x08, 32, 0x40);
        fill(&mut e, source, 0x38, 8, 0x70);
        e.call_log = Some(vec![]);
        e.call(0x00e3_e030, &args![this, source, 1i32, 2i32]);
        assert_eq!(
            bytes_of(&e, this, 0x08, 48)[32..],
            bytes_of(&e, source, 0x08, 32)[16..]
        );
        assert_eq!(
            bytes_of(&e, this, 0x14, 12)[8..],
            bytes_of(&e, source, 0x14, 8)[4..]
        );
        assert_eq!(bytes_of(&e, this, 0x38, 12)[8..], [0x74, 0x75, 0x76, 0x77]);
        assert_eq!(bytes_of(&e, this, 0x08, 48)[..32], [0; 32]);
        assert_eq!(bytes_of(&e, this, 0x2c, 12), [0; 12]);
        // Three matching elements copied 16, 4 and 4 bytes; the unmatched one nothing.
        let sizes: Vec<u32> = calls_to(&e, MEM_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, vec![16, 4, 4]);
        // The destination vertex must be below this buffer's count (3), the source below the
        // other's (2).
        e.call_log = Some(vec![]);
        e.call(0x00e3_e030, &args![this, source, 1i32, 3i32]);
        e.call(0x00e3_e030, &args![this, source, 2i32, 0i32]);
        assert!(calls_to(&e, MEM_COPY).is_empty());
    }

    /// Equal descriptions: each array's first `stride` bytes are copied from the start of the
    /// other buffer's array, whatever the vertex indices are.
    #[test]
    fn copy_vertex_with_equal_descriptions_copies_one_stride_from_the_start() {
        let mut e = rig();
        let this = buffer(&mut e, &[(6, 1), (4, 2)], 2);
        let source = buffer(&mut e, &[(6, 1), (4, 2)], 2);
        fill(&mut e, source, 0x14, 8, 0x10);
        fill(&mut e, source, 0x08, 32, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x00e3_e030, &args![this, source, 1i32, 1i32]);
        let sizes: Vec<u32> = calls_to(&e, MEM_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, vec![4, 16]);
        assert_eq!(
            bytes_of(&e, this, 0x08, 32)[..16],
            bytes_of(&e, source, 0x08, 32)[..16]
        );
        assert_eq!(bytes_of(&e, this, 0x08, 32)[16..], [0; 16]);
        assert_eq!(
            bytes_of(&e, this, 0x14, 8)[..4],
            bytes_of(&e, source, 0x14, 8)[..4]
        );
        assert_eq!(bytes_of(&e, this, 0x14, 8)[4..], [0; 4]);
    }

    /// Equal descriptions copy `stride * min(counts)` bytes of each array in one piece (a
    /// zero stride copies nothing); different ones copy vertex by vertex.
    #[test]
    fn copy_moves_whole_arrays_or_single_vertices() {
        let mut e = rig();
        let this = buffer(&mut e, &[(6, 1), (4, 2)], 2);
        let source = buffer(&mut e, &[(6, 1), (4, 2)], 3);
        fill(&mut e, source, 0x14, 12, 0x10);
        fill(&mut e, source, 0x08, 48, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x00e3_e670, &args![this, source, 0u32]);
        let sizes: Vec<u32> = calls_to(&e, MEM_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, vec![8, 32]);
        assert_eq!(bytes_of(&e, this, 0x08, 32), bytes_of(&e, source, 0x08, 32));
        assert_eq!(bytes_of(&e, this, 0x14, 8), bytes_of(&e, source, 0x14, 8));

        // Different descriptions: `copyVertex` per vertex, so one copy per element of the two vertices that exist (the third is out of range).
        let other = buffer(&mut e, &[(4, 2), (6, 1)], 2);
        e.call_log = Some(vec![]);
        e.call(0x00e3_e670, &args![other, source, 0u32]);
        let copies = calls_to(&e, MEM_COPY).len();
        assert_eq!(copies, 4);
        assert_eq!(
            bytes_of(&e, other, 0x14, 8),
            bytes_of(&e, source, 0x14, 12)[..8]
        );
    }

    /// With `resize` the exe passes this buffer's own description to `setNumVertices`, which
    /// empties it when the counts differ: the buffer ends with the other's count and no
    /// description, and nothing is copied.
    #[test]
    fn copy_with_resize_and_different_counts_empties_the_description() {
        let mut e = rig();
        let this = buffer(&mut e, &[(6, 1)], 2);
        let source = buffer(&mut e, &[(6, 1)], 3);
        fill(&mut e, source, 0x08, 48, 0x40);
        e.call(0x00e3_e670, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0x44), 3);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(bytes_of(&e, this, 0x08, 16), [0; 16]);
        // Same count and description: `setNumVertices` returns at once and the arrays copy.
        let same = buffer(&mut e, &[(6, 1)], 3);
        e.call(0x00e3_e670, &args![same, source, 1u32]);
        assert_eq!(bytes_of(&e, same, 0x08, 48), bytes_of(&e, source, 0x08, 48));
    }

    /// The description array is freed with its capacity times 12 only when owned; the data
    /// arrays' destructor and the vtable word follow.
    #[test]
    fn destructor_00e3e760_frees_the_description_when_owned() {
        let mut e = rig();
        let freed = Rc::new(RefCell::new(Vec::new()));
        let free_log = Rc::clone(&freed);
        e.register_double(ARRAY_FREE_DOUBLE, move |_, a| {
            free_log.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let arrays = Rc::new(RefCell::new(Vec::new()));
        let arrays_log = Rc::clone(&arrays);
        e.register_double(DATA_ARRAYS_DESTRUCT, move |_, a| {
            arrays_log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x5c, 0x0222_0000);
        e.mem.set_u32(this + 0x60, 2);
        e.mem.set_u32(this + 0x64, 6);
        e.call(0x00e3_e760, &args![this]);
        assert_eq!(*freed.borrow(), vec![(0x0222_0000, 72)]);
        assert_eq!(*arrays.borrow(), vec![this + 8]);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        e.call(0x00e3_e760, &args![this]);
        assert_eq!(freed.borrow().len(), 1);
    }
}

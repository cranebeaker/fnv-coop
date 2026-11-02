//! `Havok SDK/hkxmeshsection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve(allocator, array, capacity, element size)` (`00c908c0`, cdecl) and
/// the growth routine `(allocator, array, element size)` (`00c90940`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkxVertexDescription::getElementDecl(usage, index)` (`00e3df60`, `thiscall` on the vertex
/// description at `+0x5c` of a vertex buffer, `RET 8`) and `hkxVertexBuffer::getVertexDataPtr(
/// decl)` (`00e3dfb0`, `thiscall`, `RET 4`): the names are the decompiler's, unconfirmed.
const GET_ELEMENT_DECL: u32 = 0x00e3_df60;
const GET_VERTEX_DATA_PTR: u32 = 0x00e3_dfb0;
/// The error text pieces and the error handler (see `hkxattributegroup`): the `hkOstream`
/// constructor (`00cb1600`, `RET 8`), its `operator<<(const char *)` (`00cc01c0`, `RET 4`) and
/// destructor (`00cc00e0`), the handler object pointer at `01268360` (slot `+0xc` takes
/// `(severity 1, id, text, file, line)`).
const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
const OSTREAM_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The text `"Unsupported index buffer type - Ignoring"`, the file `".\Mesh\hkxMeshSection.cpp"`
/// and the id of the message.
const TEXT_UNSUPPORTED_INDEX_BUFFER: u32 = 0x010e_cea0;
const SOURCE_FILE_MESH_SECTION: u32 = 0x010e_ce84;
const ERROR_ID_INDEX_BUFFER: u32 = 0xabba_a883;
/// The size of the message buffer.
const MESSAGE_SIZE: u32 = 0x200;

// Translated from 00e61480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxMeshSection::collectVertexPositions` (Xbox PDB), `thiscall`, `RET 4` (`out`, an array
/// of 16-byte vectors: data `+0`, size `+4`, capacity word `+8`): with a vertex buffer at
/// `+8` (otherwise nothing happens), asks its vertex description (at the buffer's `+0x5c`) for
/// the element declaration of usage 1, index 0 (`00e3df60`); with one found and the buffer's
/// vertex count (`+0x44`) above zero it takes the vertex data pointer (`00e3dfb0`) and the
/// declaration's stride (`+8`), makes room for that many more vectors in `out` (reserving
/// twice the masked capacity or the new size, whichever is larger, element size 0x10, when
/// the capacity is below the new size), sets the new size and copies the vertices' first 16
/// bytes into the new entries, stepping through the vertex data by the stride.
pub fn hkx_mesh_section_collect_vertex_positions(e: &mut Engine, this: Ptr, out: Ptr) {
    let buffer = e.mem.u32(this.addr() + 8);
    if buffer == 0 {
        return;
    }
    let declaration = e
        .call(GET_ELEMENT_DECL, &args![buffer + 0x5c, 1u32, 0u32])
        .u32();
    let count = e.mem.i32(buffer + 0x44);
    if declaration == 0 || count <= 0 {
        return;
    }
    let mut vertex = e
        .call(GET_VERTEX_DATA_PTR, &args![buffer, declaration])
        .u32();
    let stride = e.mem.u32(declaration + 8);
    let array = out.addr();
    let old_size = e.mem.i32(array + 4);
    let new_size = count.wrapping_add(old_size);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    e.mem.set_u32(array + 4, new_size as u32);
    let mut at = (old_size as u32).wrapping_shl(4);
    let mut left = count;
    while left > 0 {
        let data = e.mem.u32(array);
        for lane in 0..4 {
            let word = e.mem.u32(vertex + 4 * lane);
            e.mem
                .set_u32(data.wrapping_add(at).wrapping_add(4 * lane), word);
        }
        vertex = vertex.wrapping_add(stride);
        at = at.wrapping_add(0x10);
        left -= 1;
    }
}

/// Reports `"Unsupported index buffer type - Ignoring"` (error id `0xabbaa883`, the file
/// `hkxMeshSection.cpp`, line 0x8d) through the error handler, over a 0x200-byte `hkOstream`.
fn report_unsupported_index_buffer(e: &mut Engine) {
    e.with_stack(0xc + MESSAGE_SIZE, |e, stream| {
        let buffer = stream.addr() + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, MESSAGE_SIZE]);
        e.call(OSTREAM_TEXT, &args![stream, TEXT_UNSUPPORTED_INDEX_BUFFER]);
        let handler: u32 = e.global(ERROR_HANDLER);
        e.vcall(
            handler,
            0xc,
            &args![
                1u32,
                ERROR_ID_INDEX_BUFFER,
                buffer,
                SOURCE_FILE_MESH_SECTION,
                0x8du32
            ],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
}

/// Appends the triangle `(first, second, third, extra)` to the array of 16-byte triangles at
/// `array` (data `+0`, size `+4`, capacity word `+8`), after a growth call (`00c90940`, element
/// size 0x10, through the memory router's allocator) when the size equals the masked capacity.
fn push_triangle(e: &mut Engine, array: u32, triangle: [u32; 4]) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let at = size.wrapping_shl(4).wrapping_add(e.mem.u32(array));
    for (lane, word) in triangle.iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, *word);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00e61530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxMeshSection::appendGeometry` (Xbox PDB), `thiscall`, `RET 8` (`geometry`, `extra`):
/// first `collectVertexPositions` into `geometry`'s vertex array (at `+0`), then every index
/// buffer of the section (pointers at `+0xc`, count `+0x10`) is read: an index buffer has a
/// type byte at `+8` (1: triangle list, 2: triangle strip), 16-bit indices (`+0xc`, count
/// `+0x10`) and 32-bit indices (`+0x18`, count `+0x1c`); the 16-bit ones are used when their
/// count is above zero, else the 32-bit ones, and the number of indices is the two counts
/// or-ed. For a list every three indices append a triangle `(i0, i1, i2, extra)` to
/// `geometry`'s triangle array (at `+0xc`). For a strip the first two indices only count, the
/// third gives the first triangle `(i0, i1, i2)`, and each later index `n` gives `(previous
/// triangle's third, previous triangle's second, n)`. Any other type reports an error
/// (`"Unsupported index buffer type - Ignoring"`) and leaves that buffer. The same index
/// buffer's remaining indices are skipped after the error and the next buffer follows.
pub fn hkx_mesh_section_append_geometry(e: &mut Engine, this: Ptr, geometry: Ptr, extra: u32) {
    hkx_mesh_section_collect_vertex_positions(e, this, geometry);
    let triangles = geometry.addr() + 0xc;
    let mut buffer_index = 0u32;
    while (buffer_index as i32) < e.mem.i32(this.addr() + 0x10) {
        let buffers = e.mem.u32(this.addr() + 0xc);
        let buffer = e.mem.u32(buffers.wrapping_add(4 * buffer_index));
        let count16 = e.mem.i32(buffer + 0x10);
        let total = e.mem.i32(buffer + 0x1c) | count16;
        let use16 = count16 > 0;
        let index_at = |e: &Engine, position: i32| -> u32 {
            if use16 {
                let data = e.mem.u32(buffer + 0xc);
                u32::from(
                    e.mem
                        .u16(data.wrapping_add((position as u32).wrapping_mul(2))),
                )
            } else {
                let data = e.mem.u32(buffer + 0x18);
                e.mem
                    .u32(data.wrapping_add((position as u32).wrapping_mul(4)))
            }
        };
        let mut position = 0i32;
        while position < total {
            let kind = e.mem.i8(buffer + 8);
            let triangle = match kind {
                1 => {
                    let triangle = [
                        index_at(e, position),
                        index_at(e, position + 1),
                        index_at(e, position + 2),
                    ];
                    position += 3;
                    triangle
                }
                2 => {
                    if position < 2 {
                        position += 1;
                        continue;
                    }
                    if position == 2 {
                        let triangle = [index_at(e, 0), index_at(e, 1), index_at(e, 2)];
                        position = 3;
                        triangle
                    } else {
                        let size = e.mem.u32(triangles + 4);
                        let last = size
                            .wrapping_shl(4)
                            .wrapping_add(e.mem.u32(triangles))
                            .wrapping_sub(0x10);
                        let third = e.mem.u32(last + 8);
                        let second = e.mem.u32(last + 4);
                        let new = index_at(e, position);
                        position += 1;
                        [third, second, new]
                    }
                }
                _ => {
                    report_unsupported_index_buffer(e);
                    break;
                }
            };
            push_triangle(e, triangles, [triangle[0], triangle[1], triangle[2], extra]);
        }
        buffer_index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e61480,
            hkx_mesh_section_collect_vertex_positions(Ptr, Ptr)
        ),
        entry!(0x00e61530, hkx_mesh_section_append_geometry(Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that logs its argument words and answers `answer`.
    fn log_calls(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    const ALLOCATOR_OBJECT: u32 = 0x0060_0000;
    const ALLOCATOR_TABLE: u32 = 0x0060_1000;
    const DELETE_DOUBLE: u32 = 0x0060_2008;
    const FREE_DOUBLE: u32 = 0x0060_2010;

    /// A memory router (`00c85750`) whose allocator logs `[block, size]` for slot `+0x10`
    /// (`bufFree`, the first log) and for slot `+8` (the second log).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        e.map(ALLOCATOR_OBJECT, 0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[2] = DELETE_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let deletes: Log = Rc::new(RefCell::new(Vec::new()));
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let log = deletes.clone();
        e.register_double(DELETE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (frees, deletes)
    }

    /// A vertex buffer (`+8` of the section) with `count` vertices of `stride` bytes whose first
    /// four words are `0x1000 * (vertex + 1) + lane`; `GET_ELEMENT_DECL` answers a declaration
    /// whose stride is `stride`, `GET_VERTEX_DATA_PTR` the vertex data.
    fn install_vertex_buffer(e: &mut Engine, count: u32, stride: u32) -> (u32, u32) {
        let section = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(0x80);
        e.mem.set_u32(section + 8, buffer);
        e.mem.set_u32(buffer + 0x44, count);
        let declaration = e.mem.alloc(0x10);
        e.mem.set_u32(declaration + 8, stride);
        let data = e.mem.alloc(count.max(1) * stride);
        for vertex in 0..count {
            for lane in 0..4 {
                e.mem.set_u32(
                    data + vertex * stride + 4 * lane,
                    0x1000 * (vertex + 1) + lane,
                );
            }
        }
        e.register_double(GET_ELEMENT_DECL, move |_, a| {
            assert_eq!(a[1..], [1, 0]);
            assert_eq!(a[0], buffer + 0x5c);
            Ret {
                eax: declaration,
                ..Ret::default()
            }
        });
        e.register_double(GET_VERTEX_DATA_PTR, move |_, a| {
            assert_eq!(a[..], [buffer, declaration]);
            Ret {
                eax: data,
                ..Ret::default()
            }
        });
        (section, buffer)
    }

    /// `_reserve` and the growth routine both give the array room: the data is moved to a new
    /// block with the capacity word set to what was asked (`_reserve`) or doubled (growth,
    /// at least 4). Both log their arguments.
    fn install_growth(e: &mut Engine) -> (Log, Log) {
        let reserves = log_calls_with(e, ARRAY_RESERVE, |e, a| {
            let new_capacity = a[2];
            move_array(e, a[1], new_capacity, a[3]);
        });
        let grows = log_calls_with(e, ARRAY_RESERVE_MORE, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & 0x3fff_ffff;
            move_array(e, a[1], (capacity * 2).max(4), a[2]);
        });
        (reserves, grows)
    }

    fn move_array(e: &mut Engine, array: u32, new_capacity: u32, element: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let new = e.mem.alloc(new_capacity * element);
        let bytes = e.mem.bytes(old, size * element);
        e.mem.write(new, &bytes);
        e.mem.set_u32(array, new);
        e.mem.set_u32(array + 8, new_capacity);
    }

    /// A double that logs its argument words and runs `action` on them.
    fn log_calls_with(
        e: &mut Engine,
        addr: u32,
        action: impl Fn(&mut Engine, &[u32]) + 'static,
    ) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |e, a| {
            kept.borrow_mut().push(a.to_vec());
            action(e, a);
            Ret::default()
        });
        log
    }

    fn vectors(e: &Engine, array: u32) -> Vec<[u32; 4]> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|n| std::array::from_fn(|lane| e.mem.u32(data + 16 * n + 4 * lane as u32)))
            .collect()
    }

    #[test]
    fn positions_are_appended_after_a_reserve() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let (section, _) = install_vertex_buffer(&mut e, 3, 32);
        let (reserves, _) = install_growth(&mut e);
        let out = e.mem.alloc(0x20);
        let first = e.mem.alloc(16);
        e.mem.set_u32(first, 0xaa);
        e.mem.set_u32(out, first);
        e.mem.set_u32(out + 4, 1);
        e.mem.set_u32(out + 8, 2);
        e.call(0x00e6_1480, &args![section, out]);
        // New size 4 is above the capacity 2: reserve for max(4, 4) entries of 0x10 bytes.
        assert_eq!(
            *reserves.borrow(),
            vec![vec![ALLOCATOR_OBJECT, out, 4, 0x10]]
        );
        let all = vectors(&e, out);
        assert_eq!(all.len(), 4);
        assert_eq!(all[0][0], 0xaa);
        assert_eq!(all[1], [0x1000, 0x1001, 0x1002, 0x1003]);
        assert_eq!(all[3], [0x3000, 0x3001, 0x3002, 0x3003]);
        // With enough room nothing is reserved.
        let out2 = e.mem.alloc(0x20);
        let data2 = e.mem.alloc(0x40);
        e.mem.set_u32(out2, data2);
        e.mem.set_u32(out2 + 8, 0x4000_0004);
        reserves.borrow_mut().clear();
        e.call(0x00e6_1480, &args![section, out2]);
        assert!(reserves.borrow().is_empty());
        assert_eq!(vectors(&e, out2).len(), 3);
    }

    #[test]
    fn nothing_is_collected_without_a_buffer_a_declaration_or_vertices() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let out = e.mem.alloc(0x20);
        let section = e.mem.alloc(0x20);
        let decl = log_calls(&mut e, GET_ELEMENT_DECL, 0);
        e.call(0x00e6_1480, &args![section, out]);
        assert!(decl.borrow().is_empty());
        // A buffer whose declaration is null.
        let buffer = e.mem.alloc(0x80);
        e.mem.set_u32(section + 8, buffer);
        e.mem.set_u32(buffer + 0x44, 3);
        e.call(0x00e6_1480, &args![section, out]);
        assert_eq!(decl.borrow().len(), 1);
        assert_eq!(e.mem.u32(out + 4), 0);
        // A declaration but no vertices: the data pointer is not asked for.
        let data = log_calls(&mut e, GET_VERTEX_DATA_PTR, 0);
        let decl = log_calls(&mut e, GET_ELEMENT_DECL, 0x1234);
        e.mem.set_u32(buffer + 0x44, 0);
        e.call(0x00e6_1480, &args![section, out]);
        assert_eq!(decl.borrow().len(), 1);
        assert!(data.borrow().is_empty());
    }

    /// An index buffer object: type byte, 16-bit indices and 32-bit indices.
    fn index_buffer(e: &mut Engine, kind: u8, words16: &[u16], words32: &[u32]) -> u32 {
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u8(buffer + 8, kind);
        let data16 = e.mem.alloc(2 * words16.len() as u32 + 2);
        for (i, word) in words16.iter().enumerate() {
            e.mem.set_u16(data16 + 2 * i as u32, *word);
        }
        e.mem.set_u32(buffer + 0xc, data16);
        e.mem.set_u32(buffer + 0x10, words16.len() as u32);
        let data32 = e.mem.alloc(4 * words32.len() as u32 + 4);
        for (i, word) in words32.iter().enumerate() {
            e.mem.set_u32(data32 + 4 * i as u32, *word);
        }
        e.mem.set_u32(buffer + 0x18, data32);
        e.mem.set_u32(buffer + 0x1c, words32.len() as u32);
        buffer
    }

    fn section_with(e: &mut Engine, buffers: &[u32]) -> u32 {
        let section = e.mem.alloc(0x20);
        let list = e.mem.alloc(4 * buffers.len() as u32 + 4);
        for (i, buffer) in buffers.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *buffer);
        }
        e.mem.set_u32(section + 0xc, list);
        e.mem.set_u32(section + 0x10, buffers.len() as u32);
        section
    }

    #[test]
    fn lists_strips_and_32_bit_indices_become_triangles() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let (_, grows) = install_growth(&mut e);
        let list16 = index_buffer(&mut e, 1, &[0, 1, 2, 2, 1, 3], &[]);
        let list32 = index_buffer(&mut e, 1, &[], &[5, 6, 7]);
        let strip = index_buffer(&mut e, 2, &[0, 1, 2, 3, 4], &[]);
        let strip32 = index_buffer(&mut e, 2, &[], &[9, 8, 7, 6]);
        let section = section_with(&mut e, &[list16, list32, strip, strip32]);
        let geometry = e.mem.alloc(0x20);
        e.call(0x00e6_1530, &args![section, geometry, 0x99u32]);
        // The vertex buffer is absent, so only triangles appear (array at +0xc).
        let triangles = vectors(&e, geometry + 0xc);
        assert_eq!(
            triangles,
            vec![
                [0, 1, 2, 0x99],
                [2, 1, 3, 0x99],
                [5, 6, 7, 0x99],
                [0, 1, 2, 0x99],
                [2, 1, 3, 0x99],
                [3, 1, 4, 0x99],
                [9, 8, 7, 0x99],
                [7, 8, 6, 0x99],
            ]
        );
        // The array grew from empty (capacity 0): 4, then 8 entries.
        assert_eq!(grows.borrow().len(), 2);
        assert_eq!(
            grows.borrow()[0],
            vec![ALLOCATOR_OBJECT, geometry + 0xc, 0x10]
        );
    }

    #[test]
    fn an_unsupported_index_buffer_reports_and_is_left_for_the_next() {
        let mut e = Engine::new();
        install_memory(&mut e);
        install_growth(&mut e);
        let events = Rc::new(RefCell::new(Vec::new()));
        for (address, tag) in [
            (OSTREAM_CONSTRUCT, "construct"),
            (OSTREAM_TEXT, "text"),
            (OSTREAM_DESTRUCT, "destruct"),
            (0x0061_1000, "report"),
        ] {
            let kept = events.clone();
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((tag, a.to_vec()));
                Ret::default()
            });
        }
        e.map(ERROR_HANDLER, 4);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(ERROR_HANDLER, handler);
        e.put_vtable(0x0061_0000, &[0, 0, 0, 0x0061_1000]);
        e.mem.set_u32(handler, 0x0061_0000);
        let strange = index_buffer(&mut e, 7, &[0, 1, 2], &[]);
        let list = index_buffer(&mut e, 1, &[4, 5, 6], &[]);
        let section = section_with(&mut e, &[strange, list]);
        let geometry = e.mem.alloc(0x20);
        e.call(0x00e6_1530, &args![section, geometry, 3u32]);
        assert_eq!(vectors(&e, geometry + 0xc), vec![[4, 5, 6, 3]]);
        let events = events.borrow();
        let tags: Vec<&str> = events.iter().map(|(tag, _)| *tag).collect();
        assert_eq!(tags, vec!["construct", "text", "report", "destruct"]);
        let stream = events[0].1[0];
        assert_eq!(events[0].1, vec![stream, stream + 0xc, 0x200]);
        assert_eq!(events[1].1, vec![stream, TEXT_UNSUPPORTED_INDEX_BUFFER]);
        assert_eq!(
            events[2].1,
            vec![
                handler,
                1,
                ERROR_ID_INDEX_BUFFER,
                stream + 0xc,
                SOURCE_FILE_MESH_SECTION,
                0x8d
            ]
        );
    }

    #[test]
    fn append_geometry_collects_the_vertices_first() {
        let mut e = Engine::new();
        install_memory(&mut e);
        install_growth(&mut e);
        let (section, _) = install_vertex_buffer(&mut e, 2, 16);
        let list = index_buffer(&mut e, 1, &[0, 1, 1], &[]);
        let indices = e.mem.alloc(4);
        e.mem.set_u32(indices, list);
        e.mem.set_u32(section + 0xc, indices);
        e.mem.set_u32(section + 0x10, 1);
        let geometry = e.mem.alloc(0x20);
        e.call(0x00e6_1530, &args![section, geometry, 0u32]);
        assert_eq!(vectors(&e, geometry).len(), 2);
        assert_eq!(vectors(&e, geometry + 0xc), vec![[0, 1, 1, 0]]);
    }
}

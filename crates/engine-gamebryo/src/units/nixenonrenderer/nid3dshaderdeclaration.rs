//! `NiXenonRenderer/nid3dshaderdeclaration.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni array allocation `00aa1420` (no unit in the engine map; `cdecl`, the byte count),
/// the compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element size,
/// count, constructor) with the stream constructor at `00e78bd0` (code Ghidra made no function
/// of), and the entry constructor `00e76320` (unit `nid3dshaderdeclaration.cpp`, unnamed;
/// `thiscall`, no stack words).
const NI_ARRAY_ALLOCATE_00AA1420: u32 = 0x00aa_1420;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const STREAM_CONSTRUCT_00E78BD0: u32 = 0x00e7_8bd0;
const ENTRY_CONSTRUCT_00E76320: u32 = 0x00e7_6320;

/// The byte count of `count` elements of `size` bytes as the code computes it (`MUL`, `SETO`,
/// `NEG`, `OR`): all ones when the product overflows.
fn array_bytes(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    }
}

// Translated from 00e78c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the declaration's
/// initializer that `NiXenonShaderDeclaration::Create` (`00e76700`) runs, `thiscall` with
/// three stack words: the renderer, the entry count and the stream count. Keeps the renderer
/// (`m_pkRenderer`, +8), the renderer's word at +0x8bc (`m_pkVBManager`, +0xc) and its device
/// at +0x288 (`m_pkD3DDevice`, +0x10; the device's vtable slot +4 is called on it, `stdcall`),
/// the entry count (`m_uiMaxStreamEntryCount`, +0x1c) and the stream count
/// (`m_uiStreamCount`, +0x20) (Xbox PDB names). Allocates the streams (16 bytes each,
/// `00aa1420`; built by the `vector constructor iterator` with `00e78bd0` when the allocation
/// succeeds) into `m_pkStreamEntries` (+0x24); then for each stream (the count read again from
/// +0x20 each time) allocates the entries (0x1c bytes each, the count read from +0x1c), runs
/// `00e76320` on each of them when the allocation succeeds (none when that count is 0 or above
/// `i32::MAX`: the loop counts down a signed register) and stores the array (null when the
/// allocation failed) at the stream's +8. Answers 1.
pub fn fn_00e78c60(
    e: &mut Engine,
    this: Ptr,
    renderer: Ptr,
    entry_count: u32,
    stream_count: u32,
) -> bool {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 8, renderer);
    let manager = e.mem.u32(renderer.wrapping_add(0x8bc));
    e.mem.set_u32(base + 0xc, manager);
    let device = e.mem.u32(renderer.wrapping_add(0x288));
    e.mem.set_u32(base + 0x10, device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(base + 0x1c, entry_count);
    e.mem.set_u32(base + 0x20, stream_count);
    let block = e
        .call(
            NI_ARRAY_ALLOCATE_00AA1420,
            &args![array_bytes(stream_count, 0x10)],
        )
        .u32();
    let streams = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT_00401050,
            &args![block, 0x10u32, stream_count, STREAM_CONSTRUCT_00E78BD0],
        );
        block
    } else {
        0
    };
    e.mem.set_u32(base + 0x24, streams);
    let mut index = 0u32;
    if e.mem.u32(base + 0x20) != 0 {
        loop {
            let count = e.mem.u32(base + 0x1c);
            let entries = e
                .call(NI_ARRAY_ALLOCATE_00AA1420, &args![array_bytes(count, 0x1c)])
                .u32();
            if entries != 0 {
                // `LEA EBX,[EBP-1]`, `JL`: a signed count down to 0.
                let mut left = count.wrapping_sub(1) as i32;
                let mut entry = entries;
                while left >= 0 {
                    e.call(ENTRY_CONSTRUCT_00E76320, &args![entry]);
                    entry = entry.wrapping_add(0x1c);
                    left = left.wrapping_sub(1);
                }
            }
            let streams = e.mem.u32(base + 0x24);
            let stream = streams.wrapping_add(index.wrapping_mul(0x10));
            e.mem.set_u32(stream.wrapping_add(8), entries);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0x20) {
                break;
            }
        }
    }
    true
}

// Translated from 00e76320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the constructor of a 0x1c-byte
/// declaration entry, `thiscall`, no stack words: clears +0 and +4, sets +8 to -1 and +0xc to
/// 0x11, clears +0x10 to +0x18 and answers `this`.
pub fn fn_00e76320(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0xffff_ffff);
    e.mem.set_u32(base + 0xc, 0x11);
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_8c60, fn_00e78c60(Ptr, Ptr, u32, u32) -> bool),
        entry!(0x00e7_6320, fn_00e76320(Ptr) -> Ptr),
        entry!(0x00e7_8bd0, fn_00e78bd0(Ptr) -> Ptr),
        entry!(0x00e7_8be0, fn_00e78be0(Ptr)),
        entry!(0x00e7_8f80, fn_00e78f80(u32) -> u32),
        entry!(0x00e7_9d00, fn_00e79d00(u32) -> u32),
        entry!(
            0x00e7_a200,
            fn_00e7a200(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e7_a5e0,
            ni_t_primitive_array_packer_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e7_66f0, ni_shader_declaration_get_rtti(Ptr) -> u32),
        entry!(
            0x00e7_8870,
            ni_d3d_shader_declaration_set_constant_data(Ptr, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e7_a1e0,
            ni_d3d_shader_declaration_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e7_a610,
            ni_t_array_packer_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e7_8880, fn_00e78880(u32) -> u32),
        entry!(0x00e7_8910, fn_00e78910(u32) -> u32),
        entry!(0x00e7_8a70, fn_00e78a70(u32) -> u32),
        entry!(0x00e7_8d50, fn_00e78d50(u32) -> u32),
        entry!(0x00e7_8e30, fn_00e78e30(u32) -> u32),
        entry!(0x00e7_9160, fn_00e79160(u32) -> u32),
        entry!(0x00e7_9370, fn_00e79370(u32) -> u32),
        entry!(0x00e7_9540, fn_00e79540(u32) -> u32),
        entry!(0x00e7_9760, fn_00e79760(u32) -> u32),
        entry!(0x00e7_9920, fn_00e79920(u32) -> u32),
        entry!(0x00e7_9b40, fn_00e79b40(u32) -> u32),
        entry!(0x00e7_9f40, fn_00e79f40(u32) -> u32),
        entry!(0x00e7_a0a0, fn_00e7a0a0(u32) -> u32),
    ]
}

// Translated from 00e78bd0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The stream constructor the `vector constructor iterator` runs on each 16-byte stream of
/// `00e78c60` (code Ghidra made no function of), `thiscall`, no stack words: clears the byte +0
/// and the words +4, +8 and +0xc. Answers `this`.
pub fn fn_00e78bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    for offset in [4u32, 8, 0xc] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    this
}

/// The declaration's table (`010ee7cc`, RTTI `.?AVNiD3DShaderDeclaration@@`), the Ni array
/// free `00aa1490` (`cdecl`: block, element size) and the base destructor `00ee88d0` (no unit
/// in the engine map; `thiscall`, no stack words).
const D3D_DECLARATION_VTABLE_010EE7CC: u32 = 0x010e_e7cc;
const NI_ARRAY_FREE_00AA1490: u32 = 0x00aa_1490;
const BASE_DESTRUCT_00EE88D0: u32 = 0x00ee_88d0;

// Translated from 00e78be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the `NiD3DShaderDeclaration`
/// destructor, `thiscall`, no stack words. Stores the class's table; with streams (+0x24) each
/// stream (16 bytes; the count +0x20 read again each time) whose address is not null has its
/// entries (+8) freed (`00aa1490` with 0x1c), then the streams (with 0x10). Clears +0x24 and
/// the renderer +8, releases the device (+0x10, slot 8, `stdcall`; no null check), clears it and
/// runs the base destructor `00ee88d0` (a tail jump).
pub fn fn_00e78be0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, D3D_DECLARATION_VTABLE_010EE7CC);
    if e.mem.u32(base.wrapping_add(0x24)) != 0 {
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < e.mem.u32(base.wrapping_add(0x20)) {
            let stream = e.mem.u32(base.wrapping_add(0x24)).wrapping_add(offset);
            if stream != 0 {
                let entries = e.mem.u32(stream.wrapping_add(8));
                e.call(NI_ARRAY_FREE_00AA1490, &args![entries, 0x1cu32]);
            }
            index += 1;
            offset = offset.wrapping_add(0x10);
        }
        let streams = e.mem.u32(base.wrapping_add(0x24));
        e.call(NI_ARRAY_FREE_00AA1490, &args![streams, 0x10u32]);
    }
    let device = e.mem.u32(base.wrapping_add(0x10));
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.vcall(device, 8, &args![]);
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    e.call(BASE_DESTRUCT_00EE88D0, &args![base]);
}

/// `memset` (`00ec61c0`) and `memcpy` (`00ec44d0`), both `cdecl`.
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;

/// The packer entry the pack handlers take (`NiD3DShaderDeclaration::NiPackerEntry`, built on
/// the stack by `00e7a200`): +8 the 16-bit vertex count, +0xc the 16-bit index array (or 0),
/// +0x10 the source, +0x14 the element size, +0x18 the source stride, +0x1c the size written
/// per vertex, +0x20 the destination stride, +0x2c the destination.
fn vertex_count(e: &Engine, entry: u32) -> u16 {
    e.mem.u16(entry.wrapping_add(8))
}

// Translated from 00e78f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the plain-copy pack handler,
/// `cdecl` with the packer entry. Without a source each vertex's slot is zeroed (`memset` with
/// the size per vertex). With an index array each vertex copies the indexed source element
/// (`memcpy` of the element size from the source plus index times the source stride). Without
/// one, elements of 8, 12 and 16 bytes are copied word by word (strides and sizes read once;
/// the answer is the count times the size per vertex) and any other size through `memcpy`.
/// The destination steps by its stride; answers the bytes written (the size per vertex for
/// each vertex).
pub fn fn_00e78f80(e: &mut Engine, entry: u32) -> u32 {
    let mut source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    let mut total = 0u32;
    if source == 0 {
        let mut vertex = 0u16;
        while vertex < vertex_count(e, entry) {
            let size = e.mem.u32(entry.wrapping_add(0x1c));
            e.call(MEMSET, &args![destination, 0u32, size]);
            let stride = e.mem.u32(entry.wrapping_add(0x20));
            destination = destination.wrapping_add(stride);
            let size = e.mem.u32(entry.wrapping_add(0x1c));
            total = total.wrapping_add(size);
            vertex = vertex.wrapping_add(1);
        }
        return total;
    }
    if index != 0 {
        let mut vertex = 0u16;
        while vertex < vertex_count(e, entry) {
            let size = e.mem.u32(entry.wrapping_add(0x14));
            let element = u32::from(e.mem.u16(index.wrapping_add(u32::from(vertex) * 2)));
            let stride = e.mem.u32(entry.wrapping_add(0x18));
            let from = element.wrapping_mul(stride).wrapping_add(source);
            vertex = vertex.wrapping_add(1);
            e.call(MEMCPY, &args![destination, from, size]);
            let step = e.mem.u32(entry.wrapping_add(0x20));
            destination = destination.wrapping_add(step);
            let written = e.mem.u32(entry.wrapping_add(0x1c));
            total = total.wrapping_add(written);
        }
        return total;
    }
    let per_vertex = e.mem.u32(entry.wrapping_add(0x1c));
    let size = e.mem.u32(entry.wrapping_add(0x14));
    let step = e.mem.u32(entry.wrapping_add(0x20));
    let source_stride = e.mem.u32(entry.wrapping_add(0x18));
    let count = u32::from(vertex_count(e, entry));
    let words = match size {
        8 => 2u32,
        0xc => 3,
        0x10 => 4,
        _ => {
            let mut vertex = 0u32;
            while vertex < u32::from(vertex_count(e, entry)) {
                let size = e.mem.u32(entry.wrapping_add(0x14));
                e.call(MEMCPY, &args![destination, source, size]);
                destination = destination.wrapping_add(step);
                total = total.wrapping_add(per_vertex);
                vertex += 1;
                source = source.wrapping_add(source_stride);
            }
            return total;
        }
    };
    if count == 0 {
        return 0;
    }
    for _ in 0..count {
        for word in 0..words {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(destination.wrapping_add(4 * word), value);
        }
        source = source.wrapping_add(source_stride);
        destination = destination.wrapping_add(step);
    }
    count.wrapping_mul(per_vertex)
}

/// The colour pack handler's constants: `255.0` as a `float` (`01023cd8`, the colour written
/// without a source) and as a `double` (`0101e568`, the scale of each channel).
const WHITE_CHANNEL_01023CD8: u32 = 0x0102_3cd8;
const CHANNEL_SCALE_0101E568: u32 = 0x0101_e568;

/// One channel of the colour pack: the float at `bits` (`FLD dword`) times the scale
/// (`FMUL qword`), stored as a float and loaded again, then `FISTP`.
fn channel(bits: u32, scale: f64) -> u32 {
    let stored = x87_store32(x87_mul(x87_load32(bits), scale));
    crate::x87::fistp(x87_load32(stored)) as u32
}

/// Four channels packed as the code does (`SHL 8` and `OR` of the whole words).
fn pack_channels(alpha: u32, red: u32, green: u32, blue: u32) -> u32 {
    (((((alpha << 8) | red) << 8) | green) << 8) | blue
}

// Translated from 00e79d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): the colour pack handler,
/// `cdecl` with the packer entry: each vertex's colour as one word, alpha, red, green and blue
/// (the high byte down) from the four floats of the source element times 255.0 (`0101e568`,
/// each product stored as a float and rounded by `FISTP`; with the 16-bit +4 equal to 3 the
/// alpha is 1.0), the element indexed through the 16-bit index array when there is one. Without
/// a source every vertex gets the four channels of the float 255.0 at `01023cd8`. The
/// destination steps by its stride; answers the bytes written (the size per vertex for each
/// vertex).
pub fn fn_00e79d00(e: &mut Engine, entry: u32) -> u32 {
    let mut source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    let mut total = 0u32;
    if source == 0 {
        if vertex_count(e, entry) == 0 {
            return 0;
        }
        // `FLD` then four `FST`/`FSTP` of the same value into the four channel locals.
        let white = x87_store32(x87_load32(e.mem.u32(WHITE_CHANNEL_01023CD8)));
        let mut vertex = 0u16;
        loop {
            let value = |bits: u32| crate::x87::fistp(x87_load32(bits)) as u32;
            let packed = pack_channels(value(white), value(white), value(white), value(white));
            vertex = vertex.wrapping_add(1);
            e.mem.set_u32(destination, packed);
            let step = e.mem.u32(entry.wrapping_add(0x20));
            destination = destination.wrapping_add(step);
            let written = e.mem.u32(entry.wrapping_add(0x1c));
            total = total.wrapping_add(written);
            if vertex >= vertex_count(e, entry) {
                return total;
            }
        }
    }
    let components = e.mem.u16(entry.wrapping_add(4));
    let scale = f64::from_bits(e.mem.u64(CHANNEL_SCALE_0101E568));
    let mut vertex = 0u16;
    while vertex < vertex_count(e, entry) {
        let element = if index != 0 {
            let at = u32::from(e.mem.u16(index.wrapping_add(u32::from(vertex) * 2)));
            let stride = e.mem.u32(entry.wrapping_add(0x18));
            at.wrapping_mul(stride).wrapping_add(source)
        } else {
            source
        };
        // `FLD1` or `FLD dword [+0xc]`, `FSTP dword` to a local, then the channel.
        let alpha_bits = if components == 3 {
            x87_store32(1.0)
        } else {
            x87_store32(x87_load32(e.mem.u32(element.wrapping_add(0xc))))
        };
        let alpha = channel(alpha_bits, scale);
        let red = channel(e.mem.u32(element), scale);
        let green = channel(e.mem.u32(element.wrapping_add(4)), scale);
        let blue = channel(e.mem.u32(element.wrapping_add(8)), scale);
        e.mem
            .set_u32(destination, pack_channels(alpha, red, green, blue));
        if index == 0 {
            let stride = e.mem.u32(entry.wrapping_add(0x18));
            source = source.wrapping_add(stride);
        }
        let step = e.mem.u32(entry.wrapping_add(0x20));
        destination = destination.wrapping_add(step);
        let written = e.mem.u32(entry.wrapping_add(0x1c));
        total = total.wrapping_add(written);
        vertex = vertex.wrapping_add(1);
    }
    total
}

/// What `00e7a200` reads: the size each type writes per vertex (words at `0126f508`), the
/// handler tables (`0126f550`, its +4: per type a table whose +8 lists the handlers by format),
/// `NiGeometryData::GetTextureSet` (`00a66960`, engine map; `thiscall`, the set) and the
/// additional-data lookup `00a72720` (unit `niadditionalgeometrydata.cpp`; `stdcall`, seven
/// words: the stream and the addresses of the source, format, a byte, the data, the element
/// size and the stride; answers a byte).
const WRITE_SIZES_0126F508: u32 = 0x0126_f508;
const HANDLER_TABLES_0126F550: u32 = 0x0126_f550;
const GET_TEXTURE_SET_00A66960: u32 = 0x00a6_6960;
const ADDITIONAL_DATA_00A72720: u32 = 0x00a7_2720;

/// The source, element size, stride and format a declaration entry's usage picks.
struct Picked {
    source: u32,
    size: u32,
    stride: u32,
    format: u32,
}

// Translated from 00e7a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`): packs one declaration entry
/// of a vertex stream, `thiscall` with nine words (`this` and the third word not read): the
/// vertex count (low 16 bits), the geometry data, a word never read, the skin partition, the
/// declaration entry, the destination stride, the destination, a byte and a word for the
/// handler. 0 for an entry of type 0x11 (unused). The type (4 with usage 2 counts as 5) picks
/// the size per vertex (`0126f508`) and the handler table (`0126f550` +4; none: 0). The usage
/// (+8) picks the source: -1 none (0); 0 the vertices (+0x20, 12 bytes, format 3); 1 the
/// partition's +8 by its 16-bit +0x24 (1 to 3 floats: formats 1 to 3, 4 floats in 12 of a
/// 16-byte stride: format 3; none for a null partition); 2 the partition's +0x10 (null: 0) by the
/// same (bytes, formats 0x19 to 0x1c); 3 the normals (+0x24, 12 bytes, 3); 4 the colours (+0x28,
/// 16 bytes, 4); 5 to 12 texture set `usage - 5` (`00a66960`, 8 bytes, 2); 13 and 14 the
/// binormals and tangents after the normals (the normals + count * 0x18 or * 0xc, when there are
/// normals and the data's 16-bit +0xc has any of the bits 0xf000; else none), 12 bytes, 3; a
/// negative usage asks `00a72720` (failing: no source, format 0). The handler for the format
/// then runs (`cdecl`) on a packer entry built on the stack (type, format, count, index array
/// from the partition's +0xc unless usage 1 or 2 or no partition, source, size, stride, size per
/// vertex, destination stride, the last word, the byte, destination) and its result is
/// answered; 0 without a handler or a source.
// Translation incomplete: 00e7a200 - usages 15 and up, and partitions whose 16-bit +0x24 is not 1 to 4 for usages 1 and 2, leave the source, element size or stride words uninitialized on the game's stack; the translation has 0 there (and the byte's upper three bytes in the packer entry are stack contents too).
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7a200(
    e: &mut Engine,
    _this: Ptr,
    count: u32,
    geometry: u32,
    _unused_3: u32,
    partition: u32,
    declaration_entry: u32,
    destination_stride: u32,
    destination: u32,
    flag: u32,
    last: u32,
) -> u32 {
    let entry_type = e.mem.u32(declaration_entry.wrapping_add(0xc));
    if entry_type == 0x11 {
        return 0;
    }
    let mut kind = entry_type;
    if kind == 4 && e.mem.u32(declaration_entry.wrapping_add(8)) == 2 {
        kind = 5;
    }
    let write_size = e
        .mem
        .u32(WRITE_SIZES_0126F508.wrapping_add(kind.wrapping_mul(4)));
    let tables = e.mem.u32(HANDLER_TABLES_0126F550);
    let tables = e.mem.u32(tables.wrapping_add(4));
    let handlers = e.mem.u32(tables.wrapping_add(kind.wrapping_mul(4)));
    if handlers == 0 {
        return 0;
    }
    let usage = e.mem.u32(declaration_entry.wrapping_add(8));
    let fixed = |source: u32, size: u32, format: u32| Picked {
        source,
        size,
        stride: size,
        format,
    };
    // The words the game leaves as they were on its stack keep 0 here, the format its first
    // argument (the slot it reuses).
    let unset = |source: u32| Picked {
        source,
        size: 0,
        stride: 0,
        format: count,
    };
    let after_normals = |e: &mut Engine, element: u32| {
        let normals = e.mem.u32(geometry.wrapping_add(0x24));
        let source = if normals != 0 && e.mem.u16(geometry.wrapping_add(0xc)) & 0xf000 != 0 {
            let vertices = u32::from(e.mem.u16(geometry.wrapping_add(8)));
            normals.wrapping_add(vertices.wrapping_mul(element))
        } else {
            0
        };
        fixed(source, 0xc, 3)
    };
    let picked = match usage.wrapping_add(1) {
        0 => return 0,
        1 => {
            let source = e.mem.u32(geometry.wrapping_add(0x20));
            fixed(source, 0xc, 3)
        }
        2 => {
            if partition == 0 {
                return 0;
            }
            let source = e.mem.u32(partition.wrapping_add(8));
            match e.mem.u16(partition.wrapping_add(0x24)) {
                1 => fixed(source, 4, 1),
                2 => fixed(source, 8, 2),
                3 => fixed(source, 0xc, 3),
                4 => Picked {
                    source,
                    size: 0xc,
                    stride: 0x10,
                    format: 3,
                },
                _ => unset(source),
            }
        }
        3 => {
            if partition == 0 {
                return 0;
            }
            let source = e.mem.u32(partition.wrapping_add(0x10));
            if source == 0 {
                return 0;
            }
            match e.mem.u16(partition.wrapping_add(0x24)) {
                1 => fixed(source, 1, 0x19),
                2 => fixed(source, 2, 0x1a),
                3 => fixed(source, 3, 0x1b),
                4 => fixed(source, 4, 0x1c),
                _ => unset(source),
            }
        }
        4 => {
            let source = e.mem.u32(geometry.wrapping_add(0x24));
            fixed(source, 0xc, 3)
        }
        5 => {
            let source = e.mem.u32(geometry.wrapping_add(0x28));
            fixed(source, 0x10, 4)
        }
        6..=13 => {
            let set = usage.wrapping_sub(5) & 0xffff;
            let source = e
                .call(GET_TEXTURE_SET_00A66960, &args![geometry, set])
                .u32();
            fixed(source, 8, 2)
        }
        14 => after_normals(e, 0x18),
        15 => after_normals(e, 0xc),
        _ if (usage as i32) < 0 => e.with_stack(0x18, |e, slots| {
            // The argument slots the lookup writes through: the destination word (the
            // source it answers), the first argument (the format), the byte and the geometry
            // data, then the element size and the stride.
            let slots = slots.addr();
            e.mem.set_u32(slots, destination);
            e.mem.set_u32(slots + 4, count);
            e.mem.set_u32(slots + 8, flag);
            e.mem.set_u32(slots + 0xc, geometry);
            // 00e7a4e5: `00a72720` is a thiscall (RET 0x1c) on the geometry data's +0x30
            // (ECX loaded at 00e7a284), then the seven pushed words.
            let additional = e.mem.u32(geometry.wrapping_add(0x30));
            let found = e
                .call(
                    ADDITIONAL_DATA_00A72720,
                    &args![
                        additional,
                        usage & 0x7fff_ffff,
                        slots,
                        slots + 4,
                        slots + 8,
                        slots + 0xc,
                        slots + 0x10,
                        slots + 0x14
                    ],
                )
                .bool();
            if found {
                Picked {
                    source: e.mem.u32(slots),
                    size: e.mem.u32(slots + 0x10),
                    stride: e.mem.u32(slots + 0x14),
                    format: e.mem.u32(slots + 4),
                }
            } else {
                fixed(0, 0, 0)
            }
        }),
        _ => Picked {
            source: 0,
            size: 0,
            stride: 0,
            format: count,
        },
    };
    let list = e.mem.u32(handlers.wrapping_add(8));
    let handler = e.mem.u32(list.wrapping_add(picked.format.wrapping_mul(4)));
    if handler == 0 {
        return 0;
    }
    let usage = e.mem.u32(declaration_entry.wrapping_add(8));
    let indices = if partition == 0 || usage == 1 || usage == 2 {
        0
    } else {
        e.mem.u32(partition.wrapping_add(0xc))
    };
    if picked.source == 0 {
        return 0;
    }
    e.with_stack(0x30, |e, packer| {
        let packer = packer.addr();
        e.mem.set_u32(packer, kind);
        e.mem.set_u32(packer + 4, picked.format);
        e.mem.set_u16(packer + 8, count as u16);
        e.mem.set_u32(packer + 0xc, indices);
        e.mem.set_u32(packer + 0x10, picked.source);
        e.mem.set_u32(packer + 0x14, picked.size);
        e.mem.set_u32(packer + 0x18, picked.stride);
        e.mem.set_u32(packer + 0x1c, write_size);
        e.mem.set_u32(packer + 0x20, destination_stride);
        e.mem.set_u32(packer + 0x24, last);
        e.mem.set_u8(packer + 0x28, flag as u8);
        e.mem.set_u32(packer + 0x2c, destination);
        e.call(handler, &args![packer]).u32()
    })
}

/// The array's table (`010ee888`), the free `00aa10f0` (`cdecl`, one word) and the memory
/// manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
const PACKER_ARRAY_VTABLE_010EE888: u32 = 0x010e_e888;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

// Translated from 00e7a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiD3DShaderDeclaration::NiPackerEntry *>`'s scalar deleting destructor
/// (engine map), `thiscall` with the flags: the array's table stored, its data (+4) freed
/// (`00aa10f0`), then with bit 0 set the array given back to the memory manager. Answers
/// `this`.
pub fn ni_t_primitive_array_packer_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    let data = e.mem.u32(base.wrapping_add(4));
    e.mem.set_u32(base, PACKER_ARRAY_VTABLE_010EE888);
    e.call(FREE_00AA10F0, &args![data]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, base]);
    }
    this
}

/// The RTTI record `00e766f0` answers (`01270900`).
const DECLARATION_RTTI_01270900: u32 = 0x0127_0900;

// Translated from 00e766f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderDeclaration::GetRTTI` (engine map; the slot +8 of the `NiD3DShaderDeclaration`
/// vtable `010ee7cc` among others), `thiscall`, `this` not read: the RTTI record at `01270900`.
pub fn ni_shader_declaration_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    DECLARATION_RTTI_01270900
}

// Translated from 00e78870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderDeclaration::SetConstantData` (engine map), `thiscall` with three words it does
/// not read: answers true.
pub fn ni_d3d_shader_declaration_set_constant_data(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> bool {
    true
}

/// The sized free `00aa1460` (`cdecl`: block, size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e7a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderDeclaration::scalar deleting destructor` (engine map), `thiscall` with the
/// flags: the destructor ([`fn_00e78be0`]), then with bit 0 the 0x2c bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_d3d_shader_declaration_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e78be0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x2cu32]);
    }
    this
}

/// The `NiTArray` base table `00e7a610` stores (`010ee890`).
const PACKER_ARRAY_BASE_VTABLE_010EE890: u32 = 0x010e_e890;

// Translated from 00e7a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiD3DShaderDeclaration::NiPackerEntry *,NiTMallocInterface<..> >::scalar deleting
/// destructor` (engine map), `thiscall` with the flags: the table `010ee890` stored, the data
/// (+4, read first) freed (`00aa10f0`), then with bit 0 the array given back to the memory
/// manager (`00aa4060`). Answers `this`.
pub fn ni_t_array_packer_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    let data = e.mem.u32(base.wrapping_add(4));
    e.mem.set_u32(base, PACKER_ARRAY_BASE_VTABLE_010EE890);
    e.call(FREE_00AA10F0, &args![data]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, base]);
    }
    this
}

/// The source element of vertex `vertex` through the 16-bit index array: the index times the
/// source stride (+0x18) on from the source.
fn indexed_element(e: &Engine, entry: u32, index: u32, source: u32, vertex: u16) -> u32 {
    let at = u32::from(e.mem.u16(index.wrapping_add(u32::from(vertex) * 2)));
    at.wrapping_mul(e.mem.u32(entry.wrapping_add(0x18)))
        .wrapping_add(source)
}

/// The pack handlers' loop without a source: each vertex's slot filled with `byte`
/// (`memset` of the size per vertex, the size read before the first call and after each), the
/// destination stepping by its stride. Answers the bytes written.
fn fill_vertices(e: &mut Engine, entry: u32, byte: u32) -> u32 {
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    if vertex_count(e, entry) == 0 {
        return 0;
    }
    let mut total = 0u32;
    let mut size = e.mem.u32(entry.wrapping_add(0x1c));
    let mut vertex = 0u16;
    loop {
        e.call(MEMSET, &args![destination, byte, size]);
        size = e.mem.u32(entry.wrapping_add(0x1c));
        let step = e.mem.u32(entry.wrapping_add(0x20));
        destination = destination.wrapping_add(step);
        total = total.wrapping_add(size);
        vertex = vertex.wrapping_add(1);
        if vertex >= vertex_count(e, entry) {
            return total;
        }
    }
}

/// How a word pack handler fills the vertices when there is no source.
#[derive(Clone, Copy)]
enum WordFill {
    /// The word 0xffffffff stored.
    AllOnes,
    /// `memset` with 0xff of the size per vertex ([`fill_vertices`]).
    Bytes,
}

/// The loop of the pack handlers that write one word per vertex: without a source the
/// vertices are filled (`fill`); else each vertex's source element (indexed through the 16-bit
/// index array when there is one, else stepping by the source stride) gives the word `word`
/// computes. The destination steps by its stride; answers the bytes written (the size per
/// vertex for each vertex).
fn pack_words(
    e: &mut Engine,
    entry: u32,
    fill: WordFill,
    word: impl Fn(&Engine, u32) -> u32,
) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    if source == 0 {
        if let WordFill::Bytes = fill {
            return fill_vertices(e, entry, 0xff);
        }
    }
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    let mut total = 0u32;
    let mut current = source;
    let mut vertex = 0u16;
    while vertex < vertex_count(e, entry) {
        let value = if source == 0 {
            0xffff_ffff
        } else if index != 0 {
            word(e, indexed_element(e, entry, index, source, vertex))
        } else {
            word(e, current)
        };
        e.mem.set_u32(destination, value);
        if source != 0 && index == 0 {
            current = current.wrapping_add(e.mem.u32(entry.wrapping_add(0x18)));
        }
        destination = destination.wrapping_add(e.mem.u32(entry.wrapping_add(0x20)));
        total = total.wrapping_add(e.mem.u32(entry.wrapping_add(0x1c)));
        vertex = vertex.wrapping_add(1);
    }
    total
}

/// The format of an entry (+4, 16 bits) less `base`, as the handlers compare it: 3 means a
/// source without alpha (the alpha is then opaque).
fn opaque_format(e: &Engine, entry: u32, base: u16) -> bool {
    e.mem.u16(entry.wrapping_add(4)).wrapping_sub(base) == 3
}

// Translated from 00e78880 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The word copy pack handler (code Ghidra made no function of; the handler `00e78560` stores
/// for the first number 4 and the numbers 5 and 9), `cdecl` with the packer entry: each vertex
/// gets the source element's first word; without a source the word 0xffffffff
/// ([`pack_words`]).
pub fn fn_00e78880(e: &mut Engine, entry: u32) -> u32 {
    pack_words(e, entry, WordFill::AllOnes, |e, element| e.mem.u32(element))
}

/// The colour word of four 16-bit channels as `00e78910` packs it: the high bytes of the
/// sign-extended alpha (+6; 0xffff when opaque), red (+0) and green (+2), and the byte +5.
fn signed_short_colour(e: &Engine, element: u32, opaque: bool) -> u32 {
    let alpha_word = if opaque {
        0xffff
    } else {
        e.mem.u16(element.wrapping_add(6))
    };
    let widen = |word: u16| word as i16 as i32 as u32;
    let alpha = widen(alpha_word) & 0xffff_ff00;
    let red = widen(e.mem.u16(element)) & 0xff00;
    let green = widen(e.mem.u16(element.wrapping_add(2))) & 0xff00;
    let blue = u32::from(e.mem.u8(element.wrapping_add(5)));
    ((((alpha << 8) | red) << 8) | green) | blue
}

// Translated from 00e78910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first number 4 and the number 16): the colour pack handler of signed 16-bit
/// channels, `cdecl` with the packer entry. The format (+4) less 0xc being 3 makes the alpha
/// opaque. Each vertex gets [`signed_short_colour`] of its element; without a source the word
/// 0xffffffff ([`pack_words`]).
pub fn fn_00e78910(e: &mut Engine, entry: u32) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let opaque = source != 0 && opaque_format(e, entry, 0xc);
    pack_words(e, entry, WordFill::AllOnes, |e, element| {
        signed_short_colour(e, element, opaque)
    })
}

/// The colour word of four unsigned 16-bit channels as `00e78a70` packs it: the high bytes of
/// alpha (+6; 0xffff when opaque), red (+0) and green (+2), and the byte +5.
fn unsigned_short_colour(e: &Engine, element: u32, opaque: bool) -> u32 {
    let alpha_word = if opaque {
        0xffff
    } else {
        e.mem.u16(element.wrapping_add(6))
    };
    let alpha = u32::from(alpha_word) & 0xffff_ff00;
    let red = u32::from(e.mem.u16(element)) & 0xff00;
    let green = u32::from(e.mem.u16(element.wrapping_add(2))) & 0xff00;
    let blue = u32::from(e.mem.u8(element.wrapping_add(5)));
    ((((alpha << 8) | red) << 8) | green) | blue
}

// Translated from 00e78a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first number 4 and the number 20): as [`fn_00e78910`] for unsigned 16-bit
/// channels ([`unsigned_short_colour`]), opaque when the format less 0x10 is 3.
pub fn fn_00e78a70(e: &mut Engine, entry: u32) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let opaque = source != 0 && opaque_format(e, entry, 0x10);
    pack_words(e, entry, WordFill::AllOnes, |e, element| {
        unsigned_short_colour(e, element, opaque)
    })
}

// Translated from 00e78d50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The narrowing copy pack handler (code Ghidra made no function of; the handler `00e78560`
/// stores for the first numbers 0 to 3 when the number is above the first), `cdecl` with the
/// packer entry: each vertex copies the size per vertex (+0x1c, read before the first copy and
/// after each) from its source element (`memcpy`; indexed through the 16-bit index array when
/// there is one, else stepping by the source stride); without a source each slot is zeroed
/// ([`fill_vertices`]). The destination steps by its stride; answers the bytes written.
pub fn fn_00e78d50(e: &mut Engine, entry: u32) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    if source == 0 {
        return fill_vertices(e, entry, 0);
    }
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    if vertex_count(e, entry) == 0 {
        return 0;
    }
    let mut total = 0u32;
    let mut current = source;
    let mut size = e.mem.u32(entry.wrapping_add(0x1c));
    let mut vertex = 0u16;
    loop {
        let element = if index != 0 {
            indexed_element(e, entry, index, source, vertex)
        } else {
            current
        };
        e.call(MEMCPY, &args![destination, element, size]);
        size = e.mem.u32(entry.wrapping_add(0x1c));
        if index == 0 {
            current = current.wrapping_add(e.mem.u32(entry.wrapping_add(0x18)));
        }
        destination = destination.wrapping_add(e.mem.u32(entry.wrapping_add(0x20)));
        total = total.wrapping_add(size);
        vertex = vertex.wrapping_add(1);
        if vertex >= vertex_count(e, entry) {
            return total;
        }
    }
}

/// Clears `count` words from `at` on (`REP STOSD`).
fn zero_words(e: &mut Engine, at: u32, count: u16) {
    for word in 0..u32::from(count) {
        e.mem.set_u32(at.wrapping_add(4 * word), 0);
    }
}

// Translated from 00e78e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first numbers 0 to 3 when the number is below the first, and for the numbers
/// 0x1d to 0x20): the widening copy pack handler, `cdecl` with the packer entry. Each vertex
/// copies its source element (`memcpy` of the element size +0x14; indexed through the 16-bit
/// index array when there is one, else stepping by the source stride) and clears the
/// following words: the type (+0, 16 bits) less the format (+4) plus one, as 16 bits. Without
/// a source each slot is zeroed ([`fill_vertices`]). The destination steps by its stride;
/// answers the bytes written.
pub fn fn_00e78e30(e: &mut Engine, entry: u32) -> u32 {
    let pad = e
        .mem
        .u16(entry)
        .wrapping_sub(e.mem.u16(entry.wrapping_add(4)))
        .wrapping_add(1);
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    if source == 0 {
        return fill_vertices(e, entry, 0);
    }
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    let mut total = 0u32;
    let mut current = source;
    let mut vertex = 0u16;
    while vertex < vertex_count(e, entry) {
        let element = if index != 0 {
            indexed_element(e, entry, index, source, vertex)
        } else {
            current
        };
        let size = e.mem.u32(entry.wrapping_add(0x14));
        e.call(MEMCPY, &args![destination, element, size]);
        let end = e
            .mem
            .u32(entry.wrapping_add(0x14))
            .wrapping_add(destination);
        zero_words(e, end, pad);
        total = total.wrapping_add(e.mem.u32(entry.wrapping_add(0x1c)));
        if index == 0 {
            current = current.wrapping_add(e.mem.u32(entry.wrapping_add(0x18)));
        }
        destination = destination.wrapping_add(e.mem.u32(entry.wrapping_add(0x20)));
        vertex = vertex.wrapping_add(1);
    }
    total
}

/// The loop of the pack handlers that turn integer components into floats: without a source
/// each slot is zeroed ([`fill_vertices`]). Else the source has the format (+4, 16 bits) less
/// `base` components of `width` bytes and the destination the type (+0) plus one floats; each
/// vertex's element (indexed through the 16-bit index array when there is one, else stepping by
/// the source stride) gives the lesser number of floats through `convert` (the address of the
/// component; answers the float's bits), and the destination's remaining floats are cleared.
/// The destination steps by its stride; answers the bytes written.
fn pack_floats(
    e: &mut Engine,
    entry: u32,
    base: u16,
    width: u32,
    convert: impl Fn(&Engine, u32) -> u32,
) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let index = e.mem.u32(entry.wrapping_add(0xc));
    if source == 0 {
        return fill_vertices(e, entry, 0);
    }
    let components = e.mem.u16(entry.wrapping_add(4)).wrapping_sub(base);
    let floats = e.mem.u16(entry).wrapping_add(1);
    let converted = components.min(floats);
    let mut destination = e.mem.u32(entry.wrapping_add(0x2c));
    let mut total = 0u32;
    let mut current = source;
    let mut vertex = 0u16;
    while vertex < vertex_count(e, entry) {
        let element = if index != 0 {
            indexed_element(e, entry, index, source, vertex)
        } else {
            current
        };
        for component in 0..u32::from(converted) {
            let value = convert(e, element.wrapping_add(width * component));
            e.mem
                .set_u32(destination.wrapping_add(4 * component), value);
        }
        if converted < floats {
            let end = destination.wrapping_add(4 * u32::from(converted));
            zero_words(e, end, floats - converted);
        }
        total = total.wrapping_add(e.mem.u32(entry.wrapping_add(0x1c)));
        if index == 0 {
            current = current.wrapping_add(e.mem.u32(entry.wrapping_add(0x18)));
        }
        destination = destination.wrapping_add(e.mem.u32(entry.wrapping_add(0x20)));
        vertex = vertex.wrapping_add(1);
    }
    total
}

/// The constants of the integer-to-float handlers: the lower bound `-1.0` (`float`,
/// `01012054`), 2^32 (`float`, `01099378`) and the scales (`double`): 2^-31 (`010229b8`), 2^-32
/// (`010229c0`), 32767 (`01029790`), 65535 (`010217d0`), 127 (`0102dfe0`) and 255
/// (`0101e568`); the values are read from the exe.
const LOWER_BOUND_01012054: u32 = 0x0101_2054;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const SCALE_I32_010229B8: u32 = 0x0102_29b8;
const SCALE_U32_010229C0: u32 = 0x0102_29c0;
const SCALE_I16_01029790: u32 = 0x0102_9790;
const SCALE_U16_010217D0: u32 = 0x0102_17d0;
const SCALE_I8_0102DFE0: u32 = 0x0102_dfe0;
const SCALE_U8_0101E568: u32 = CHANNEL_SCALE_0101E568;

/// A `double` constant as `FLD qword` loads it.
fn scale(e: &Engine, at: u32) -> f64 {
    x87_load64(e.mem.u64(at))
}

/// The clamp of the signed handlers: the value stored as a float (`FSTP`) and loaded again,
/// compared with the lower bound (`FCOM`; `TEST AH,5`, `JP`): below it (ordered) the bound,
/// else the value; the result goes through a float local (`FST`/`FSTP`, `FLD`, `FSTP`).
fn clamped(e: &Engine, value: f64) -> u32 {
    let stored = x87_store32(value);
    let bound_bits = e.mem.u32(LOWER_BOUND_01012054);
    let bound = x87_load32(bound_bits);
    if x87_load32(stored) < bound {
        x87_quiet_bits(x87_store32(bound))
    } else {
        x87_quiet_bits(stored)
    }
}

// Translated from 00e79160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first numbers 0 to 3 and the numbers 5 to 8): signed 32-bit components
/// (format less 4) to floats ([`pack_floats`]): each `FILD` times the scale at `010229b8`
/// (`FMUL`) and clamped below at the bound ([`clamped`]).
pub fn fn_00e79160(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 4, 4, |e, at| {
        let value = f64::from(e.mem.u32(at) as i32);
        clamped(e, x87_mul(value, scale(e, SCALE_I32_010229B8)))
    })
}

// Translated from 00e79370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first numbers 0 to 3 and the numbers 9 to 12): unsigned 32-bit components
/// (format less 8) to floats ([`pack_floats`]): each `FILD` (2^32 added, `FADD` of the float at
/// `01099378`, when the word is negative as signed) times the scale at `010229c0` (`FMUL`),
/// stored.
pub fn fn_00e79370(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 8, 4, |e, at| {
        let word = e.mem.u32(at) as i32;
        let mut value = f64::from(word);
        if word < 0 {
            value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
        }
        x87_store32(x87_mul(value, scale(e, SCALE_U32_010229C0)))
    })
}

// Translated from 00e79540 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Code Ghidra made no function of (the handler `00e78560` stores for the first numbers 0 to 3
/// and the numbers 13 to 16): signed 16-bit components (format less 0xc) to floats
/// ([`pack_floats`]): each divided by the scale at `01029790` (`FDIV`) and clamped below at the
/// bound ([`clamped`]).
pub fn fn_00e79540(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 0xc, 2, |e, at| {
        let value = f64::from(e.mem.u16(at) as i16);
        clamped(e, x87_div(value, scale(e, SCALE_I16_01029790)))
    })
}

// Translated from 00e79760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first numbers 0 to 3 and the numbers 17 to 20): unsigned 16-bit components
/// (format less 0x10) to floats ([`pack_floats`]): each divided by the scale at `010217d0`
/// (`FDIV`), stored.
pub fn fn_00e79760(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 0x10, 2, |e, at| {
        let value = f64::from(e.mem.u16(at));
        x87_store32(x87_div(value, scale(e, SCALE_U16_010217D0)))
    })
}

// Translated from 00e79920 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Code Ghidra made no function of (the handler `00e78560` stores for the first numbers 0 to 3
/// and the numbers 21 to 24): signed bytes (format less 0x14) to floats ([`pack_floats`]): each
/// divided by the scale at `0102dfe0` (`FDIV`) and clamped below at the bound ([`clamped`]).
pub fn fn_00e79920(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 0x14, 1, |e, at| {
        let value = f64::from(e.mem.u8(at) as i8);
        clamped(e, x87_div(value, scale(e, SCALE_I8_0102DFE0)))
    })
}

// Translated from 00e79b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderdeclaration.cpp`; the handler `00e78560`
/// stores for the first numbers 0 to 3 and the numbers 25 to 28): unsigned bytes (format less
/// 0x18) to floats ([`pack_floats`]): each divided by the scale at `0101e568` (`FDIV`), stored.
pub fn fn_00e79b40(e: &mut Engine, entry: u32) -> u32 {
    pack_floats(e, entry, 0x18, 1, |e, at| {
        let value = f64::from(e.mem.u8(at));
        x87_store32(x87_div(value, scale(e, SCALE_U8_0101E568)))
    })
}

// Translated from 00e79f40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Code Ghidra made no function of (the handler `00e78560` stores for the first number 4 and
/// the numbers 21 to 24): the colour pack handler of signed bytes, `cdecl` with the packer
/// entry: each vertex gets the bytes alpha (+3; 0xff when the format less 0x14 is 3), +0, +1
/// and +2 from the high byte down (the alpha sign-extended before its shifts); without a source
/// each slot gets 0xff bytes (`memset`, [`fill_vertices`]).
pub fn fn_00e79f40(e: &mut Engine, entry: u32) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let opaque = source != 0 && opaque_format(e, entry, 0x14);
    pack_words(e, entry, WordFill::Bytes, |e, element| {
        let alpha = if opaque {
            0xff
        } else {
            e.mem.u8(element.wrapping_add(3))
        };
        let widened_alpha = alpha as i8 as i32 as u32;
        let first = u32::from(e.mem.u8(element));
        let second = u32::from(e.mem.u8(element.wrapping_add(1)));
        let third = u32::from(e.mem.u8(element.wrapping_add(2)));
        ((((first | (widened_alpha << 8)) << 8) | second) << 8) | third
    })
}

// Translated from 00e7a0a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Code Ghidra made no function of (the handler `00e78560` stores for the first number 4 and
/// the numbers 25 to 28): as [`fn_00e79f40`] for unsigned bytes (the alpha zero-extended),
/// opaque when the format less 0x18 is 3.
pub fn fn_00e7a0a0(e: &mut Engine, entry: u32) -> u32 {
    let source = e.mem.u32(entry.wrapping_add(0x10));
    let opaque = source != 0 && opaque_format(e, entry, 0x18);
    pack_words(e, entry, WordFill::Bytes, |e, element| {
        let alpha = if opaque {
            0xff
        } else {
            u32::from(e.mem.u8(element.wrapping_add(3)))
        };
        let first = u32::from(e.mem.u8(element));
        let second = u32::from(e.mem.u8(element.wrapping_add(1)));
        let third = u32::from(e.mem.u8(element.wrapping_add(2)));
        ((((alpha << 8) | first) << 8) | second) << 8 | third
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_REFERENCE: u32 = 0x0e60_0000;
    const DEVICE_VTABLE: u32 = 0x0e61_0000;

    /// An engine whose allocation double answers memory for sizes up to 0x1000 and 0 above.
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE]);
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |e, a| {
            if a[0] <= 0x1000 {
                e.mem.alloc(a[0].max(1)).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(VECTOR_CONSTRUCT_00401050, |_, a| a[0].into_ret());
        e.register(ENTRY_CONSTRUCT_00E76320, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let renderer = e.mem.alloc(0x900);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8bc, 0x0e62_0000);
        (e, renderer, device)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e78c60_allocates_the_streams_and_their_entries() {
        let (mut e, renderer, device) = engine();
        let this = e.mem.alloc(0x2c);
        let answer = e
            .call(0x00e7_8c60, &args![this, renderer, 3u32, 2u32])
            .u32();
        assert_eq!(answer & 0xff, 1);
        assert_eq!(e.mem.u32(this + 8), renderer);
        assert_eq!(e.mem.u32(this + 0xc), 0x0e62_0000);
        assert_eq!(e.mem.u32(this + 0x10), device);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        assert_eq!(e.mem.u32(this + 0x20), 2);
        let streams = e.mem.u32(this + 0x24);
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420),
            vec![vec![0x20], vec![0x54], vec![0x54]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![streams, 0x10, 2, STREAM_CONSTRUCT_00E78BD0]]
        );
        let first = e.mem.u32(streams + 8);
        let second = e.mem.u32(streams + 0x18);
        assert_eq!(
            calls(&e, ENTRY_CONSTRUCT_00E76320),
            vec![
                vec![first],
                vec![first + 0x1c],
                vec![first + 0x38],
                vec![second],
                vec![second + 0x1c],
                vec![second + 0x38],
            ]
        );
    }

    #[test]
    fn fn_00e78c60_keeps_null_arrays_and_skips_an_empty_count() {
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 0u32, 1u32]);
        assert!(calls(&e, ENTRY_CONSTRUCT_00E76320).is_empty());
        // An entry count whose size overflows asks for all ones; the null array is kept.
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 0x1000_0000u32, 1u32]);
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420),
            vec![vec![0x10], vec![u32::MAX]]
        );
        let streams = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.u32(streams + 8), 0);
        assert!(calls(&e, ENTRY_CONSTRUCT_00E76320).is_empty());
        // No streams: nothing but the empty stream array.
        let (mut e, renderer, _) = engine();
        let this = e.mem.alloc(0x2c);
        e.call(0x00e7_8c60, &args![this, renderer, 3u32, 0u32]);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420), vec![vec![0]]);
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn fn_00e76320_sets_up_the_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcc; 0x20]);
        assert_eq!(e.call(0x00e7_6320, &args![this]).u32(), this);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0xffff_ffff, 0x11, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0x1c), 0xcccc_cccc);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A packer entry: `count` vertices, source, index array, element size, source stride,
    /// size per vertex, destination stride and destination.
    #[allow(clippy::too_many_arguments)]
    fn packer(
        e: &mut Engine,
        count: u16,
        source: u32,
        index: u32,
        size: u32,
        stride: u32,
        per_vertex: u32,
        step: u32,
        destination: u32,
    ) -> u32 {
        let entry = e.mem.alloc(0x30);
        e.mem.set_u16(entry + 8, count);
        e.mem.set_u32(entry + 0xc, index);
        e.mem.set_u32(entry + 0x10, source);
        e.mem.set_u32(entry + 0x14, size);
        e.mem.set_u32(entry + 0x18, stride);
        e.mem.set_u32(entry + 0x1c, per_vertex);
        e.mem.set_u32(entry + 0x20, step);
        e.mem.set_u32(entry + 0x2c, destination);
        entry
    }

    #[test]
    fn fn_00e78bd0_clears_the_stream() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xcc; 0x14]);
        assert_eq!(e.call(0x00e7_8bd0, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(e.mem.bytes(this + 1, 3), vec![0xcc; 3]);
        assert_eq!(e.mem.bytes(this + 4, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0x10), 0xcccc_cccc);
    }

    #[test]
    fn fn_00e78be0_frees_the_streams_and_releases_the_device() {
        let mut e = Engine::new();
        for no_op in [NI_ARRAY_FREE_00AA1490, BASE_DESTRUCT_00EE88D0, 0x0e90_0008] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.put_vtable(0x0e91_0000, &[0, 0, 0x0e90_0008]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x0e91_0000);
        let streams = e.mem.alloc(0x20);
        e.mem.set_u32(streams + 8, 0x5100);
        e.mem.set_u32(streams + 0x18, 0x5200);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0x10, device);
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x24, streams);
        e.call_log = Some(vec![]);
        e.call(0x00e7_8be0, &args![this]);
        assert_eq!(
            calls(&e, NI_ARRAY_FREE_00AA1490),
            vec![vec![0x5100, 0x1c], vec![0x5200, 0x1c], vec![streams, 0x10]]
        );
        assert_eq!(calls(&e, 0x0e90_0008), vec![vec![device]]);
        assert_eq!(calls(&e, BASE_DESTRUCT_00EE88D0), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), D3D_DECLARATION_VTABLE_010EE7CC);
        for offset in [8, 0x10, 0x24] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn fn_00e78f80_copies_by_size() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        let source = e.mem.alloc(0x40);
        for word in 0..0x10u32 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        // 12-byte elements, source stride 0x10, destination stride 0x14: word copies.
        let destination = e.mem.alloc(0x40);
        let entry = packer(&mut e, 2, source, 0, 0xc, 0x10, 0x18, 0x14, destination);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_8f80, &args![entry]).u32(), 0x30);
        assert_eq!(e.mem.u32(destination), 0x100);
        assert_eq!(e.mem.u32(destination + 8), 0x102);
        assert_eq!(e.mem.u32(destination + 0xc), 0);
        assert_eq!(e.mem.u32(destination + 0x14), 0x104);
        assert_eq!(e.mem.u32(destination + 0x1c), 0x106);
        assert!(calls(&e, MEMCPY).is_empty());
        // Another size goes through memcpy.
        let entry = packer(&mut e, 2, source, 0, 6, 0x10, 0x18, 0x14, destination);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_8f80, &args![entry]).u32(), 0x30);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![
                vec![destination, source, 6],
                vec![destination + 0x14, source + 0x10, 6]
            ]
        );
        // An index array: the indexed elements.
        let index = e.mem.alloc(4);
        e.mem.set_u16(index, 3);
        e.mem.set_u16(index + 2, 1);
        let entry = packer(&mut e, 2, source, index, 8, 0x10, 0x18, 0x14, destination);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_8f80, &args![entry]).u32(), 0x30);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![
                vec![destination, source + 0x30, 8],
                vec![destination + 0x14, source + 0x10, 8]
            ]
        );
        // No source: zeroed slots.
        let entry = packer(&mut e, 3, 0, 0, 8, 0x10, 0x18, 0x14, destination);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_8f80, &args![entry]).u32(), 0x48);
        assert_eq!(calls(&e, MEMSET)[2], vec![destination + 0x28, 0, 0x18]);
        // No vertices.
        let entry = packer(&mut e, 0, source, 0, 0x10, 0x10, 0x18, 0x14, destination);
        assert_eq!(e.call(0x00e7_8f80, &args![entry]).u32(), 0);
    }

    fn colour_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0102_3000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_u32(WHITE_CHANNEL_01023CD8, 255f32.to_bits());
        e.mem.set_u64(CHANNEL_SCALE_0101E568, 255f64.to_bits());
        e
    }

    #[test]
    fn fn_00e79d00_packs_colours() {
        let mut e = colour_engine();
        let source = e.mem.alloc(0x20);
        for (index, value) in [1.0f32, 0.5, 0.0, 0.25, 0.2, 0.4, 0.6, 0.8]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(source + 4 * index as u32, value.to_bits());
        }
        let destination = e.mem.alloc(0x10);
        let entry = packer(&mut e, 2, source, 0, 0x10, 0x10, 4, 8, destination);
        e.mem.set_u16(entry + 4, 4);
        assert_eq!(e.call(0x00e7_9d00, &args![entry]).u32(), 8);
        // 0.25 * 255 = 63.75 -> 64; 0.5 * 255 = 127.5 -> 128 (to even).
        assert_eq!(e.mem.u32(destination), 0x40ff_8000);
        // 0.8 * 255 = 204; 0.2 -> 51, 0.4 -> 102, 0.6 -> 153.
        assert_eq!(e.mem.u32(destination + 8), 0xcc33_6699);
        // Three components: alpha 255; through an index array.
        let index = e.mem.alloc(2);
        e.mem.set_u16(index, 1);
        let entry = packer(&mut e, 1, source, index, 0x10, 0x10, 4, 8, destination);
        e.mem.set_u16(entry + 4, 3);
        assert_eq!(e.call(0x00e7_9d00, &args![entry]).u32(), 4);
        assert_eq!(e.mem.u32(destination), 0xff33_6699);
        // No source: white.
        let entry = packer(&mut e, 2, 0, 0, 0x10, 0x10, 4, 8, destination);
        assert_eq!(e.call(0x00e7_9d00, &args![entry]).u32(), 8);
        assert_eq!(e.mem.u32(destination), 0xffff_ffff);
        assert_eq!(e.mem.u32(destination + 8), 0xffff_ffff);
        let entry = packer(&mut e, 0, 0, 0, 0x10, 0x10, 4, 8, destination);
        assert_eq!(e.call(0x00e7_9d00, &args![entry]).u32(), 0);
    }

    const HANDLER: u32 = 0x0e92_0000;

    /// Type 2 has a handler table whose format 3 entry is `HANDLER` (which answers the
    /// packer entry's address and records nothing else) and writes 0xc bytes per vertex.
    fn pack_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(HANDLER, |e, a| {
            let packer = a[0];
            let fields: Vec<u32> = (0..12).map(|i| e.mem.u32(packer + 4 * i)).collect();
            let copy = e.mem.alloc(0x30);
            for (index, field) in fields.into_iter().enumerate() {
                e.mem.set_u32(copy + 4 * index as u32, field);
            }
            copy.into_ret()
        });
        let formats = e.mem.alloc(0x80);
        for format in [1u32, 2, 3, 4, 0x19] {
            e.mem.set_u32(formats + 4 * format, HANDLER);
        }
        let handlers = e.mem.alloc(0xc);
        e.mem.set_u32(handlers + 8, formats);
        let per_type = e.mem.alloc(0x50);
        for kind in [2u32, 5] {
            e.mem.set_u32(per_type + 4 * kind, handlers);
        }
        let tables = e.mem.alloc(8);
        e.mem.set_u32(tables + 4, per_type);
        e.mem.set_u32(HANDLER_TABLES_0126F550, tables);
        e.mem.set_u32(WRITE_SIZES_0126F508 + 8, 0xc);
        e.mem.set_u32(WRITE_SIZES_0126F508 + 0x14, 0x10);
        let geometry = e.mem.alloc(0x30);
        e.mem.set_u16(geometry + 8, 10);
        e.mem.set_u32(geometry + 0x20, 0x2000);
        e.mem.set_u32(geometry + 0x24, 0x2400);
        e.mem.set_u32(geometry + 0x28, 0x2800);
        (e, geometry)
    }

    fn declaration(e: &mut Engine, usage: u32, kind: u32) -> u32 {
        let entry = e.mem.alloc(0x1c);
        e.mem.set_u32(entry + 8, usage);
        e.mem.set_u32(entry + 0xc, kind);
        entry
    }

    /// The packer entry the handler saw: (type, format, count word, index, source, size,
    /// stride, per vertex, step, last, byte word, destination).
    fn pack(e: &mut Engine, geometry: u32, partition: u32, entry: u32) -> Option<Vec<u32>> {
        let copy = e
            .call(
                0x00e7_a200,
                &args![
                    0u32, 7u32, geometry, 0u32, partition, entry, 0x20u32, 0x9000u32, 0x15u32,
                    0x99u32
                ],
            )
            .u32();
        if copy == 0 {
            return None;
        }
        Some((0..12).map(|i| e.mem.u32(copy + 4 * i)).collect())
    }

    #[test]
    fn fn_00e7a200_packs_the_vertices_and_normals() {
        let (mut e, geometry) = pack_engine();
        let entry = declaration(&mut e, 0, 2);
        assert_eq!(
            pack(&mut e, geometry, 0, entry).unwrap(),
            vec![2, 3, 7, 0, 0x2000, 0xc, 0xc, 0xc, 0x20, 0x99, 0x15, 0x9000]
        );
        // Normals with a partition: its index array.
        let partition = e.mem.alloc(0x28);
        e.mem.set_u32(partition + 0xc, 0x4c00);
        let entry = declaration(&mut e, 3, 2);
        let seen = pack(&mut e, geometry, partition, entry).unwrap();
        assert_eq!((seen[3], seen[4]), (0x4c00, 0x2400));
        // Binormals: after the normals, only with the 0xf000 bits.
        let entry = declaration(&mut e, 13, 2);
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
        e.mem.set_u16(geometry + 0xc, 0x1000);
        assert_eq!(
            pack(&mut e, geometry, 0, entry).unwrap()[4],
            0x2400 + 10 * 0x18
        );
        let entry = declaration(&mut e, 14, 2);
        assert_eq!(
            pack(&mut e, geometry, 0, entry).unwrap()[4],
            0x2400 + 10 * 0xc
        );
    }

    #[test]
    fn fn_00e7a200_picks_by_usage() {
        let (mut e, geometry) = pack_engine();
        // Unused type, no handler table, usage -1: nothing.
        let entry = declaration(&mut e, 0, 0x11);
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
        let entry = declaration(&mut e, 0, 3);
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
        let entry = declaration(&mut e, 0xffff_ffff, 2);
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
        // Colours: format 4 and 16 bytes; type 4 with usage 2 counts as type 5.
        let entry = declaration(&mut e, 4, 2);
        let seen = pack(&mut e, geometry, 0, entry).unwrap();
        assert_eq!((seen[1], seen[4], seen[5]), (4, 0x2800, 0x10));
        // Usage 1: the partition's weights by its count; no index array.
        let partition = e.mem.alloc(0x28);
        e.mem.set_u32(partition + 8, 0x4800);
        e.mem.set_u32(partition + 0xc, 0x4c00);
        e.mem.set_u16(partition + 0x24, 2);
        let entry = declaration(&mut e, 1, 2);
        let seen = pack(&mut e, geometry, partition, entry).unwrap();
        assert_eq!(seen[..7].to_vec(), vec![2, 2, 7, 0, 0x4800, 8, 8]);
        e.mem.set_u16(partition + 0x24, 4);
        let seen = pack(&mut e, geometry, partition, entry).unwrap();
        assert_eq!((seen[1], seen[5], seen[6]), (3, 0xc, 0x10));
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
        // Usage 2 (type 4, so type 5): the partition's bytes, format 0x19.
        e.mem.set_u16(partition + 0x24, 1);
        e.mem.set_u32(partition + 0x10, 0x4100);
        let entry = declaration(&mut e, 2, 4);
        let seen = pack(&mut e, geometry, partition, entry).unwrap();
        assert_eq!(seen[..8].to_vec(), vec![5, 0x19, 7, 0, 0x4100, 1, 1, 0x10]);
        e.mem.set_u32(partition + 0x10, 0);
        assert_eq!(pack(&mut e, geometry, partition, entry), None);
    }

    #[test]
    fn fn_00e7a200_texture_sets_and_additional_data() {
        let (mut e, geometry) = pack_engine();
        e.register(GET_TEXTURE_SET_00A66960, |_, a| (0x3000 + a[1]).into_ret());
        let entry = declaration(&mut e, 7, 2);
        e.call_log = Some(vec![]);
        let seen = pack(&mut e, geometry, 0, entry).unwrap();
        assert_eq!((seen[1], seen[4], seen[5]), (2, 0x3002, 8));
        assert_eq!(calls(&e, GET_TEXTURE_SET_00A66960), vec![vec![geometry, 2]]);
        // A negative usage asks the additional data, which answers the source, format, size
        // and stride through its slots.
        e.register(ADDITIONAL_DATA_00A72720, |e, a| {
            assert_eq!(a.len(), 8);
            assert_eq!(a[1], 5);
            assert_eq!(e.mem.u32(a[2]), 0x9000);
            assert_eq!(e.mem.u32(a[3]), 7);
            e.mem.set_u32(a[2], 0x6000);
            e.mem.set_u32(a[3], 1);
            e.mem.set_u32(a[6], 4);
            e.mem.set_u32(a[7], 0x14);
            1u32.into_ret()
        });
        let entry = declaration(&mut e, 0x8000_0005, 2);
        let seen = pack(&mut e, geometry, 0, entry).unwrap();
        assert_eq!(seen[..7].to_vec(), vec![2, 1, 7, 0, 0x6000, 4, 0x14]);
        assert_eq!(seen[11], 0x9000);
        e.register(ADDITIONAL_DATA_00A72720, |_, _| 0u32.into_ret());
        assert_eq!(pack(&mut e, geometry, 0, entry), None);
    }

    #[test]
    fn packer_array_destructor() {
        let mut e = Engine::new();
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_a5e0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), PACKER_ARRAY_VTABLE_010EE888);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x7400]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, this]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00e7_a5e0, &args![this, 0u32]);
        assert!(calls(&e, MEMORY_MANAGER_FREE).is_empty());
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the handlers' constants in place and `memset`/`memcpy` doubles that do
    /// the work.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x0109_9000, 0x1000);
        e.mem.set_u32(LOWER_BOUND_01012054, (-1.0f32).to_bits());
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        for (at, value) in [
            (SCALE_I32_010229B8, 2f64.powi(-31)),
            (SCALE_U32_010229C0, 2f64.powi(-32)),
            (SCALE_I16_01029790, 32767.0),
            (SCALE_U16_010217D0, 65535.0),
            (SCALE_I8_0102DFE0, 127.0),
            (SCALE_U8_0101E568, 255.0),
        ] {
            e.mem.set_u64(at, value.to_bits());
        }
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A packer entry: type, format, vertex count, index array, source, element size, source
    /// stride, size per vertex, destination stride and destination.
    #[allow(clippy::too_many_arguments)]
    fn packer(
        e: &mut Engine,
        kind: u16,
        format: u16,
        count: u16,
        index: u32,
        source: u32,
        size: u32,
        stride: u32,
        per_vertex: u32,
        destination_stride: u32,
        destination: u32,
    ) -> u32 {
        let entry = e.mem.alloc(0x30);
        e.mem.set_u16(entry, kind);
        e.mem.set_u16(entry + 4, format);
        e.mem.set_u16(entry + 8, count);
        e.mem.set_u32(entry + 0xc, index);
        e.mem.set_u32(entry + 0x10, source);
        e.mem.set_u32(entry + 0x14, size);
        e.mem.set_u32(entry + 0x18, stride);
        e.mem.set_u32(entry + 0x1c, per_vertex);
        e.mem.set_u32(entry + 0x20, destination_stride);
        e.mem.set_u32(entry + 0x2c, destination);
        entry
    }

    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    /// Two indices: 1 then 0.
    fn indices(e: &mut Engine) -> u32 {
        let index = e.mem.alloc(4);
        e.mem.set_u16(index, 1);
        e.mem.set_u16(index + 2, 0);
        index
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00e7_66f0, &args![0u32]).u32(), 0x0127_0900);
    }

    #[test]
    fn set_constant_data_answers_true() {
        let mut e = engine();
        assert!(e.call(0x00e7_8870, &args![0u32, 1u32, 2u32, 3u32]).bool());
    }

    #[test]
    fn declaration_scalar_deleting_destructor_frees_with_bit_zero() {
        for (flags, frees) in [(1u32, true), (0, false)] {
            let mut e = engine();
            e.put_vtable(0x0e00_0000, &[0, 0, 0x0e01_0008]);
            for callee in [0x0e01_0008, BASE_DESTRUCT_00EE88D0, FREE_SIZED_00AA1460] {
                e.register(callee, |_, _| Ret::default());
            }
            let device = e.mem.alloc(4);
            e.mem.set_u32(device, 0x0e00_0000);
            let this = e.mem.alloc(0x2c);
            e.mem.set_u32(this + 0x10, device);
            assert_eq!(e.call(0x00e7_a1e0, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), D3D_DECLARATION_VTABLE_010EE7CC);
            assert_eq!(calls(&e, BASE_DESTRUCT_00EE88D0), vec![vec![this]]);
            let freed = if frees {
                vec![vec![this, 0x2c]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, FREE_SIZED_00AA1460), freed);
        }
    }

    #[test]
    fn packer_array_scalar_deleting_destructor_frees_the_data() {
        for (flags, frees) in [(1u32, true), (0, false)] {
            let mut e = engine();
            e.register(FREE_00AA10F0, |_, _| Ret::default());
            e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 4, 0x0e40_0000);
            assert_eq!(e.call(0x00e7_a610, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), PACKER_ARRAY_BASE_VTABLE_010EE890);
            assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e40_0000]]);
            let freed = if frees {
                vec![vec![MEMORY_MANAGER_OBJECT, this]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, MEMORY_MANAGER_FREE), freed);
        }
    }

    #[test]
    fn word_copy_handler_copies_first_words() {
        let mut e = engine();
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x1111_1111);
        e.mem.set_u32(source + 8, 0x2222_2222);
        let out = e.mem.alloc(0x10);
        let entry = packer(&mut e, 0, 5, 2, 0, source, 4, 8, 4, 8, out);
        assert_eq!(e.call(0x00e7_8880, &args![entry]).u32(), 8);
        assert_eq!(e.mem.u32(out), 0x1111_1111);
        assert_eq!(e.mem.u32(out + 8), 0x2222_2222);
        let index = indices(&mut e);
        e.mem.set_u32(entry + 0xc, index);
        e.call(0x00e7_8880, &args![entry]);
        assert_eq!(e.mem.u32(out), 0x2222_2222);
        assert_eq!(e.mem.u32(out + 8), 0x1111_1111);
        e.mem.set_u32(entry + 0x10, 0);
        assert_eq!(e.call(0x00e7_8880, &args![entry]).u32(), 8);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        assert_eq!(e.mem.u32(out + 8), 0xffff_ffff);
    }

    #[test]
    fn signed_short_colour_handler_takes_the_high_bytes() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        for (i, word) in [0x8123u16, 0x7f45, 0x0006, 0x40ff].iter().enumerate() {
            e.mem.set_u16(source + 2 * i as u32, *word);
        }
        let out = e.mem.alloc(4);
        let entry = packer(&mut e, 3, 16, 1, 0, source, 8, 8, 4, 4, out);
        assert_eq!(e.call(0x00e7_8910, &args![entry]).u32(), 4);
        // Alpha 0x40, red 0x81, green 0x7f; the byte +5 (the third word's high byte) 0x00.
        assert_eq!(e.mem.u32(out), 0x4081_7f00);
        // Format 15: opaque. The sign extension of 0xffff fills the top byte.
        e.mem.set_u16(entry + 4, 15);
        e.call(0x00e7_8910, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xff81_7f00);
        let index = indices(&mut e);
        e.mem.set_u32(entry + 0xc, index);
        e.mem.set_u16(entry + 8, 1);
        e.mem.set_u32(entry + 0x18, 0);
        e.call(0x00e7_8910, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xff81_7f00);
        e.mem.set_u32(entry + 0x10, 0);
        e.call(0x00e7_8910, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
    }

    #[test]
    fn unsigned_short_colour_handler_takes_the_high_bytes() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        for (i, word) in [0x8123u16, 0x7f45, 0x9906, 0x40ff].iter().enumerate() {
            e.mem.set_u16(source + 2 * i as u32, *word);
        }
        let out = e.mem.alloc(4);
        let entry = packer(&mut e, 3, 20, 1, 0, source, 8, 8, 4, 4, out);
        e.call(0x00e7_8a70, &args![entry]);
        assert_eq!(e.mem.u32(out), 0x4081_7f99);
        e.mem.set_u16(entry + 4, 19);
        e.call(0x00e7_8a70, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xff81_7f99);
    }

    #[test]
    fn narrowing_copy_handler_copies_the_size_per_vertex() {
        let mut e = engine();
        let source = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(source + 4 * i, i + 1);
        }
        let out = e.mem.alloc(0x20);
        let entry = packer(&mut e, 1, 3, 2, 0, source, 12, 16, 8, 8, out);
        assert_eq!(e.call(0x00e7_8d50, &args![entry]).u32(), 16);
        assert_eq!(words(&e, out, 4), vec![1, 2, 5, 6]);
        let index = indices(&mut e);
        e.mem.set_u32(entry + 0xc, index);
        e.call(0x00e7_8d50, &args![entry]);
        assert_eq!(words(&e, out, 4), vec![5, 6, 1, 2]);
        e.mem.set_u32(entry + 0x10, 0);
        e.call(0x00e7_8d50, &args![entry]);
        assert_eq!(words(&e, out, 4), vec![0; 4]);
    }

    #[test]
    fn widening_copy_handler_pads_with_zeros() {
        let mut e = engine();
        let source = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_u32(source + 4 * i, i + 1);
        }
        let out = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(out + 4 * i, 0xaaaa_aaaa);
        }
        // Type 3 (four floats) from format 2: 8 bytes copied, 3 - 2 + 1 = 2 words cleared.
        let entry = packer(&mut e, 3, 2, 2, 0, source, 8, 8, 16, 16, out);
        assert_eq!(e.call(0x00e7_8e30, &args![entry]).u32(), 32);
        assert_eq!(words(&e, out, 8), vec![1, 2, 0, 0, 3, 4, 0, 0]);
        let index = indices(&mut e);
        e.mem.set_u32(entry + 0xc, index);
        e.call(0x00e7_8e30, &args![entry]);
        assert_eq!(words(&e, out, 8), vec![3, 4, 0, 0, 1, 2, 0, 0]);
    }

    #[test]
    fn signed_int_handler_scales_and_clamps() {
        let mut e = engine();
        let source = e.mem.alloc(0x10);
        for (i, word) in [0x4000_0000u32, 0x8000_0000, 0x8000_0001, 7]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(source + 4 * i as u32, *word);
        }
        let out = e.mem.alloc(0x10);
        // Format 7: three components; type 3: four floats, the last cleared.
        let entry = packer(&mut e, 3, 7, 1, 0, source, 12, 12, 16, 16, out);
        assert_eq!(e.call(0x00e7_9160, &args![entry]).u32(), 16);
        let below = -(0x7fff_ffff as f64) * 2f64.powi(-31);
        assert_eq!(
            floats(&e, out, 4),
            vec![0.5, -1.0, (below as f32).max(-1.0), 0.0]
        );
        // The source wider than the destination: only two floats.
        e.mem.set_u16(entry, 1);
        e.mem.set_u32(out + 8, 0x1234);
        e.call(0x00e7_9160, &args![entry]);
        assert_eq!(e.mem.u32(out + 8), 0x1234);
        e.mem.set_u32(entry + 0x10, 0);
        e.call(0x00e7_9160, &args![entry]);
        assert_eq!(words(&e, out, 4), vec![0, 0, 0, 0]);
    }

    #[test]
    fn unsigned_int_handler_adds_two_to_the_32_for_high_words() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x8000_0000);
        e.mem.set_u32(source + 4, 0x4000_0000);
        let out = e.mem.alloc(8);
        let entry = packer(&mut e, 1, 10, 1, 0, source, 8, 8, 8, 8, out);
        e.call(0x00e7_9370, &args![entry]);
        assert_eq!(floats(&e, out, 2), vec![0.5, 0.25]);
    }

    #[test]
    fn short_handlers_divide_by_their_scales() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 0x8000);
        e.mem.set_u16(source + 2, 32767);
        let out = e.mem.alloc(8);
        let entry = packer(&mut e, 1, 14, 1, 0, source, 4, 4, 8, 8, out);
        e.call(0x00e7_9540, &args![entry]);
        // -32768 / 32767 is below -1: clamped.
        assert_eq!(floats(&e, out, 2), vec![-1.0, 1.0]);
        e.mem.set_u16(entry + 4, 18);
        e.mem.set_u16(source, 65535);
        e.call(0x00e7_9760, &args![entry]);
        assert_eq!(floats(&e, out, 2), vec![1.0, (32767.0f64 / 65535.0) as f32]);
    }

    #[test]
    fn byte_handlers_divide_by_their_scales() {
        let mut e = engine();
        let source = e.mem.alloc(4);
        e.mem.write(source, &[0x80, 0x7f, 0, 0]);
        let out = e.mem.alloc(8);
        let index = indices(&mut e);
        let entry = packer(&mut e, 1, 22, 2, index, source, 2, 2, 8, 0, out);
        e.call(0x00e7_9920, &args![entry]);
        // The last vertex is element 0 (the index list is 1, 0): -128 / 127 clamped.
        assert_eq!(floats(&e, out, 2), vec![-1.0, 1.0]);
        e.mem.set_u16(entry + 4, 26);
        e.call(0x00e7_9b40, &args![entry]);
        assert_eq!(floats(&e, out, 2), vec![128.0 / 255.0, 127.0 / 255.0]);
    }

    #[test]
    fn byte_colour_handlers_pack_alpha_first() {
        let mut e = engine();
        let source = e.mem.alloc(4);
        e.mem.write(source, &[0x81, 0x02, 0x83, 0x44]);
        let out = e.mem.alloc(8);
        let entry = packer(&mut e, 3, 24, 1, 0, source, 4, 4, 4, 4, out);
        e.call(0x00e7_9f40, &args![entry]);
        assert_eq!(e.mem.u32(out), 0x4481_0283);
        e.mem.set_u16(entry + 4, 23);
        e.call(0x00e7_9f40, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xff81_0283);
        e.mem.set_u16(entry + 4, 28);
        e.call(0x00e7_a0a0, &args![entry]);
        assert_eq!(e.mem.u32(out), 0x4481_0283);
        e.mem.set_u16(entry + 4, 27);
        e.call(0x00e7_a0a0, &args![entry]);
        assert_eq!(e.mem.u32(out), 0xff81_0283);
        // Without a source: 0xff bytes through memset.
        e.mem.set_u32(entry + 0x10, 0);
        e.mem.set_u32(out, 0);
        assert_eq!(e.call(0x00e7_9f40, &args![entry]).u32(), 4);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        assert_eq!(calls(&e, MEMSET), vec![vec![out, 0xff, 4]]);
    }
}
